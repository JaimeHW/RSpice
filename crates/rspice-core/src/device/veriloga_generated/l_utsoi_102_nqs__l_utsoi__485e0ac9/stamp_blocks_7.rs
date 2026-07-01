#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_112(
        locals: &mut StampLocals,
    ) {
        let (assign40390_e46160, assign40390_e46160_d_n4, assign40390_e46160_d_n6, assign40390_e46160_d_n7, assign40390_e46160_d_n8, assign40390_e46160_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40390_e46158: f64 = (locals.var_esurf2d__blk1022 - locals.var_k2q2d__blk1005);
        (assign40390_e46158, (locals.var_esurf2d__blk1022_dn4 - locals.var_k2q2d__blk1005_dn4), (locals.var_esurf2d__blk1022_dn6 - locals.var_k2q2d__blk1005_dn6), (locals.var_esurf2d__blk1022_dn7 - locals.var_k2q2d__blk1005_dn7), (locals.var_esurf2d__blk1022_dn8 - locals.var_k2q2d__blk1005_dn8), (locals.var_esurf2d__blk1022_dn9 - locals.var_k2q2d__blk1005_dn9),)
    } else {
        (locals.var_ecpl1d__blk1023, locals.var_ecpl1d__blk1023_dn4, locals.var_ecpl1d__blk1023_dn6, locals.var_ecpl1d__blk1023_dn7, locals.var_ecpl1d__blk1023_dn8, locals.var_ecpl1d__blk1023_dn9,)
    }
};
        locals.var_ecpl1d__blk1023 = assign40390_e46160;
        locals.var_ecpl1d__blk1023_dn4 = assign40390_e46160_d_n4;
        locals.var_ecpl1d__blk1023_dn6 = assign40390_e46160_d_n6;
        locals.var_ecpl1d__blk1023_dn7 = assign40390_e46160_d_n7;
        locals.var_ecpl1d__blk1023_dn8 = assign40390_e46160_d_n8;
        locals.var_ecpl1d__blk1023_dn9 = assign40390_e46160_d_n9;

        let (assign40400_e46166, assign40400_e46166_d_n4, assign40400_e46166_d_n6, assign40400_e46166_d_n7, assign40400_e46166_d_n8, assign40400_e46166_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40400_e46164: f64 = (locals.var_esurf1d__blk1021 - locals.var_k1q1d__blk1004);
        (assign40400_e46164, (locals.var_esurf1d__blk1021_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_esurf1d__blk1021_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_esurf1d__blk1021_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_esurf1d__blk1021_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_esurf1d__blk1021_dn9 - locals.var_k1q1d__blk1004_dn9),)
    } else {
        (locals.var_ecpl2d__blk1024, locals.var_ecpl2d__blk1024_dn4, locals.var_ecpl2d__blk1024_dn6, locals.var_ecpl2d__blk1024_dn7, locals.var_ecpl2d__blk1024_dn8, locals.var_ecpl2d__blk1024_dn9,)
    }
};
        locals.var_ecpl2d__blk1024 = assign40400_e46166;
        locals.var_ecpl2d__blk1024_dn4 = assign40400_e46166_d_n4;
        locals.var_ecpl2d__blk1024_dn6 = assign40400_e46166_d_n6;
        locals.var_ecpl2d__blk1024_dn7 = assign40400_e46166_d_n7;
        locals.var_ecpl2d__blk1024_dn8 = assign40400_e46166_d_n8;
        locals.var_ecpl2d__blk1024_dn9 = assign40400_e46166_d_n9;

        let (assign40410_e46176, assign40410_e46176_d_n4, assign40410_e46176_d_n6, assign40410_e46176_d_n7, assign40410_e46176_d_n8, assign40410_e46176_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40410_e46170: f64 = (locals.var_eta_mu * locals.var_esurf1d__blk1021);
        let assign40410_e46173: f64 = (locals.var_one_m_eta * locals.var_ecpl1d__blk1023);
        let assign40410_e46174: f64 = (assign40410_e46170 + assign40410_e46173);
        (assign40410_e46174, ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn4) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn4)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn6) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn6)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn7) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn7)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn8) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn8)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn9) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn9)),)
    } else {
        (locals.var_eeff1d__blk1025, locals.var_eeff1d__blk1025_dn4, locals.var_eeff1d__blk1025_dn6, locals.var_eeff1d__blk1025_dn7, locals.var_eeff1d__blk1025_dn8, locals.var_eeff1d__blk1025_dn9,)
    }
};
        locals.var_eeff1d__blk1025 = assign40410_e46176;
        locals.var_eeff1d__blk1025_dn4 = assign40410_e46176_d_n4;
        locals.var_eeff1d__blk1025_dn6 = assign40410_e46176_d_n6;
        locals.var_eeff1d__blk1025_dn7 = assign40410_e46176_d_n7;
        locals.var_eeff1d__blk1025_dn8 = assign40410_e46176_d_n8;
        locals.var_eeff1d__blk1025_dn9 = assign40410_e46176_d_n9;

        let (assign40420_e46186, assign40420_e46186_d_n4, assign40420_e46186_d_n6, assign40420_e46186_d_n7, assign40420_e46186_d_n8, assign40420_e46186_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40420_e46180: f64 = (locals.var_eta_mu * locals.var_esurf2d__blk1022);
        let assign40420_e46183: f64 = (locals.var_one_m_eta * locals.var_ecpl2d__blk1024);
        let assign40420_e46184: f64 = (assign40420_e46180 + assign40420_e46183);
        (assign40420_e46184, ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn4) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn4)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn6) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn6)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn7) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn7)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn8) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn8)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn9) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn9)),)
    } else {
        (locals.var_eeff2d__blk1026, locals.var_eeff2d__blk1026_dn4, locals.var_eeff2d__blk1026_dn6, locals.var_eeff2d__blk1026_dn7, locals.var_eeff2d__blk1026_dn8, locals.var_eeff2d__blk1026_dn9,)
    }
};
        locals.var_eeff2d__blk1026 = assign40420_e46186;
        locals.var_eeff2d__blk1026_dn4 = assign40420_e46186_d_n4;
        locals.var_eeff2d__blk1026_dn6 = assign40420_e46186_d_n6;
        locals.var_eeff2d__blk1026_dn7 = assign40420_e46186_d_n7;
        locals.var_eeff2d__blk1026_dn8 = assign40420_e46186_d_n8;
        locals.var_eeff2d__blk1026_dn9 = assign40420_e46186_d_n9;

        let (assign40430_e46194, assign40430_e46194_d_n4, assign40430_e46194_d_n6, assign40430_e46194_d_n7, assign40430_e46194_d_n8, assign40430_e46194_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40430_e46191: f64 = (locals.var_esurf1s__blk952 + locals.var_esurf1d__blk1021);
        let assign40430_e46192: f64 = (0.5 * assign40430_e46191);
        (assign40430_e46192, (0.5 * (locals.var_esurf1s__blk952_dn4 + locals.var_esurf1d__blk1021_dn4)), (0.5 * (locals.var_esurf1s__blk952_dn6 + locals.var_esurf1d__blk1021_dn6)), (0.5 * (locals.var_esurf1s__blk952_dn7 + locals.var_esurf1d__blk1021_dn7)), (0.5 * (locals.var_esurf1s__blk952_dn8 + locals.var_esurf1d__blk1021_dn8)), (0.5 * (locals.var_esurf1s__blk952_dn9 + locals.var_esurf1d__blk1021_dn9)),)
    } else {
        (locals.var_esurf1__blk1027, locals.var_esurf1__blk1027_dn4, locals.var_esurf1__blk1027_dn6, locals.var_esurf1__blk1027_dn7, locals.var_esurf1__blk1027_dn8, locals.var_esurf1__blk1027_dn9,)
    }
};
        locals.var_esurf1__blk1027 = assign40430_e46194;
        locals.var_esurf1__blk1027_dn4 = assign40430_e46194_d_n4;
        locals.var_esurf1__blk1027_dn6 = assign40430_e46194_d_n6;
        locals.var_esurf1__blk1027_dn7 = assign40430_e46194_d_n7;
        locals.var_esurf1__blk1027_dn8 = assign40430_e46194_d_n8;
        locals.var_esurf1__blk1027_dn9 = assign40430_e46194_d_n9;

        let (assign40440_e46202, assign40440_e46202_d_n4, assign40440_e46202_d_n6, assign40440_e46202_d_n7, assign40440_e46202_d_n8, assign40440_e46202_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40440_e46199: f64 = (locals.var_esurf2s__blk953 + locals.var_esurf2d__blk1022);
        let assign40440_e46200: f64 = (0.5 * assign40440_e46199);
        (assign40440_e46200, (0.5 * (locals.var_esurf2s__blk953_dn4 + locals.var_esurf2d__blk1022_dn4)), (0.5 * (locals.var_esurf2s__blk953_dn6 + locals.var_esurf2d__blk1022_dn6)), (0.5 * (locals.var_esurf2s__blk953_dn7 + locals.var_esurf2d__blk1022_dn7)), (0.5 * (locals.var_esurf2s__blk953_dn8 + locals.var_esurf2d__blk1022_dn8)), (0.5 * (locals.var_esurf2s__blk953_dn9 + locals.var_esurf2d__blk1022_dn9)),)
    } else {
        (locals.var_esurf2__blk1028, locals.var_esurf2__blk1028_dn4, locals.var_esurf2__blk1028_dn6, locals.var_esurf2__blk1028_dn7, locals.var_esurf2__blk1028_dn8, locals.var_esurf2__blk1028_dn9,)
    }
};
        locals.var_esurf2__blk1028 = assign40440_e46202;
        locals.var_esurf2__blk1028_dn4 = assign40440_e46202_d_n4;
        locals.var_esurf2__blk1028_dn6 = assign40440_e46202_d_n6;
        locals.var_esurf2__blk1028_dn7 = assign40440_e46202_d_n7;
        locals.var_esurf2__blk1028_dn8 = assign40440_e46202_d_n8;
        locals.var_esurf2__blk1028_dn9 = assign40440_e46202_d_n9;

        let (assign40450_e46210, assign40450_e46210_d_n4, assign40450_e46210_d_n6, assign40450_e46210_d_n7, assign40450_e46210_d_n8, assign40450_e46210_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40450_e46207: f64 = (locals.var_esurf1__blk1027 + locals.var_esurf2__blk1028);
        let assign40450_e46208: f64 = (1.0 / assign40450_e46207);
        (assign40450_e46208, (-((locals.var_esurf1__blk1027_dn4 + locals.var_esurf2__blk1028_dn4) / (assign40450_e46207 * assign40450_e46207))), (-((locals.var_esurf1__blk1027_dn6 + locals.var_esurf2__blk1028_dn6) / (assign40450_e46207 * assign40450_e46207))), (-((locals.var_esurf1__blk1027_dn7 + locals.var_esurf2__blk1028_dn7) / (assign40450_e46207 * assign40450_e46207))), (-((locals.var_esurf1__blk1027_dn8 + locals.var_esurf2__blk1028_dn8) / (assign40450_e46207 * assign40450_e46207))), (-((locals.var_esurf1__blk1027_dn9 + locals.var_esurf2__blk1028_dn9) / (assign40450_e46207 * assign40450_e46207))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign40450_e46210;
        locals.var_temp_dn4 = assign40450_e46210_d_n4;
        locals.var_temp_dn6 = assign40450_e46210_d_n6;
        locals.var_temp_dn7 = assign40450_e46210_d_n7;
        locals.var_temp_dn8 = assign40450_e46210_d_n8;
        locals.var_temp_dn9 = assign40450_e46210_d_n9;

        let (assign40460_e46218, assign40460_e46218_d_n4, assign40460_e46218_d_n6, assign40460_e46218_d_n7, assign40460_e46218_d_n8, assign40460_e46218_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40460_e46214: f64 = (locals.var_qim__blk1016 * locals.var_esurf1__blk1027);
        let assign40460_e46216: f64 = (assign40460_e46214 * locals.var_temp);
        (assign40460_e46216, ((((locals.var_qim__blk1016_dn4 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn4)) * locals.var_temp) + (assign40460_e46214 * locals.var_temp_dn4)), ((((locals.var_qim__blk1016_dn6 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn6)) * locals.var_temp) + (assign40460_e46214 * locals.var_temp_dn6)), ((((locals.var_qim__blk1016_dn7 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn7)) * locals.var_temp) + (assign40460_e46214 * locals.var_temp_dn7)), ((((locals.var_qim__blk1016_dn8 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn8)) * locals.var_temp) + (assign40460_e46214 * locals.var_temp_dn8)), ((((locals.var_qim__blk1016_dn9 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn9)) * locals.var_temp) + (assign40460_e46214 * locals.var_temp_dn9)),)
    } else {
        (locals.var_qi1m__blk1029, locals.var_qi1m__blk1029_dn4, locals.var_qi1m__blk1029_dn6, locals.var_qi1m__blk1029_dn7, locals.var_qi1m__blk1029_dn8, locals.var_qi1m__blk1029_dn9,)
    }
};
        locals.var_qi1m__blk1029 = assign40460_e46218;
        locals.var_qi1m__blk1029_dn4 = assign40460_e46218_d_n4;
        locals.var_qi1m__blk1029_dn6 = assign40460_e46218_d_n6;
        locals.var_qi1m__blk1029_dn7 = assign40460_e46218_d_n7;
        locals.var_qi1m__blk1029_dn8 = assign40460_e46218_d_n8;
        locals.var_qi1m__blk1029_dn9 = assign40460_e46218_d_n9;

        let (assign40470_e46226, assign40470_e46226_d_n4, assign40470_e46226_d_n6, assign40470_e46226_d_n7, assign40470_e46226_d_n8, assign40470_e46226_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40470_e46222: f64 = (locals.var_qim__blk1016 * locals.var_esurf2__blk1028);
        let assign40470_e46224: f64 = (assign40470_e46222 * locals.var_temp);
        (assign40470_e46224, ((((locals.var_qim__blk1016_dn4 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn4)) * locals.var_temp) + (assign40470_e46222 * locals.var_temp_dn4)), ((((locals.var_qim__blk1016_dn6 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn6)) * locals.var_temp) + (assign40470_e46222 * locals.var_temp_dn6)), ((((locals.var_qim__blk1016_dn7 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn7)) * locals.var_temp) + (assign40470_e46222 * locals.var_temp_dn7)), ((((locals.var_qim__blk1016_dn8 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn8)) * locals.var_temp) + (assign40470_e46222 * locals.var_temp_dn8)), ((((locals.var_qim__blk1016_dn9 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn9)) * locals.var_temp) + (assign40470_e46222 * locals.var_temp_dn9)),)
    } else {
        (locals.var_qi2m__blk1030, locals.var_qi2m__blk1030_dn4, locals.var_qi2m__blk1030_dn6, locals.var_qi2m__blk1030_dn7, locals.var_qi2m__blk1030_dn8, locals.var_qi2m__blk1030_dn9,)
    }
};
        locals.var_qi2m__blk1030 = assign40470_e46226;
        locals.var_qi2m__blk1030_dn4 = assign40470_e46226_d_n4;
        locals.var_qi2m__blk1030_dn6 = assign40470_e46226_d_n6;
        locals.var_qi2m__blk1030_dn7 = assign40470_e46226_d_n7;
        locals.var_qi2m__blk1030_dn8 = assign40470_e46226_d_n8;
        locals.var_qi2m__blk1030_dn9 = assign40470_e46226_d_n9;

        let (assign40480_e46234, assign40480_e46234_d_n4, assign40480_e46234_d_n6, assign40480_e46234_d_n7, assign40480_e46234_d_n8, assign40480_e46234_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40480_e46231: f64 = (locals.var_ecpl1s__blk954 + locals.var_ecpl1d__blk1023);
        let assign40480_e46232: f64 = (0.5 * assign40480_e46231);
        (assign40480_e46232, (0.5 * (locals.var_ecpl1s__blk954_dn4 + locals.var_ecpl1d__blk1023_dn4)), (0.5 * (locals.var_ecpl1s__blk954_dn6 + locals.var_ecpl1d__blk1023_dn6)), (0.5 * (locals.var_ecpl1s__blk954_dn7 + locals.var_ecpl1d__blk1023_dn7)), (0.5 * (locals.var_ecpl1s__blk954_dn8 + locals.var_ecpl1d__blk1023_dn8)), (0.5 * (locals.var_ecpl1s__blk954_dn9 + locals.var_ecpl1d__blk1023_dn9)),)
    } else {
        (locals.var_ecpl1__blk1031, locals.var_ecpl1__blk1031_dn4, locals.var_ecpl1__blk1031_dn6, locals.var_ecpl1__blk1031_dn7, locals.var_ecpl1__blk1031_dn8, locals.var_ecpl1__blk1031_dn9,)
    }
};
        locals.var_ecpl1__blk1031 = assign40480_e46234;
        locals.var_ecpl1__blk1031_dn4 = assign40480_e46234_d_n4;
        locals.var_ecpl1__blk1031_dn6 = assign40480_e46234_d_n6;
        locals.var_ecpl1__blk1031_dn7 = assign40480_e46234_d_n7;
        locals.var_ecpl1__blk1031_dn8 = assign40480_e46234_d_n8;
        locals.var_ecpl1__blk1031_dn9 = assign40480_e46234_d_n9;

        let (assign40490_e46242, assign40490_e46242_d_n4, assign40490_e46242_d_n6, assign40490_e46242_d_n7, assign40490_e46242_d_n8, assign40490_e46242_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40490_e46239: f64 = (locals.var_ecpl2s__blk955 + locals.var_ecpl2d__blk1024);
        let assign40490_e46240: f64 = (0.5 * assign40490_e46239);
        (assign40490_e46240, (0.5 * (locals.var_ecpl2s__blk955_dn4 + locals.var_ecpl2d__blk1024_dn4)), (0.5 * (locals.var_ecpl2s__blk955_dn6 + locals.var_ecpl2d__blk1024_dn6)), (0.5 * (locals.var_ecpl2s__blk955_dn7 + locals.var_ecpl2d__blk1024_dn7)), (0.5 * (locals.var_ecpl2s__blk955_dn8 + locals.var_ecpl2d__blk1024_dn8)), (0.5 * (locals.var_ecpl2s__blk955_dn9 + locals.var_ecpl2d__blk1024_dn9)),)
    } else {
        (locals.var_ecpl2__blk1032, locals.var_ecpl2__blk1032_dn4, locals.var_ecpl2__blk1032_dn6, locals.var_ecpl2__blk1032_dn7, locals.var_ecpl2__blk1032_dn8, locals.var_ecpl2__blk1032_dn9,)
    }
};
        locals.var_ecpl2__blk1032 = assign40490_e46242;
        locals.var_ecpl2__blk1032_dn4 = assign40490_e46242_d_n4;
        locals.var_ecpl2__blk1032_dn6 = assign40490_e46242_d_n6;
        locals.var_ecpl2__blk1032_dn7 = assign40490_e46242_d_n7;
        locals.var_ecpl2__blk1032_dn8 = assign40490_e46242_d_n8;
        locals.var_ecpl2__blk1032_dn9 = assign40490_e46242_d_n9;

        let (assign40500_e46250, assign40500_e46250_d_n4, assign40500_e46250_d_n6, assign40500_e46250_d_n7, assign40500_e46250_d_n8, assign40500_e46250_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40500_e46247: f64 = (locals.var_eeff1s__blk956 + locals.var_eeff1d__blk1025);
        let assign40500_e46248: f64 = (0.5 * assign40500_e46247);
        (assign40500_e46248, (0.5 * (locals.var_eeff1s__blk956_dn4 + locals.var_eeff1d__blk1025_dn4)), (0.5 * (locals.var_eeff1s__blk956_dn6 + locals.var_eeff1d__blk1025_dn6)), (0.5 * (locals.var_eeff1s__blk956_dn7 + locals.var_eeff1d__blk1025_dn7)), (0.5 * (locals.var_eeff1s__blk956_dn8 + locals.var_eeff1d__blk1025_dn8)), (0.5 * (locals.var_eeff1s__blk956_dn9 + locals.var_eeff1d__blk1025_dn9)),)
    } else {
        (locals.var_eeff1__blk1033, locals.var_eeff1__blk1033_dn4, locals.var_eeff1__blk1033_dn6, locals.var_eeff1__blk1033_dn7, locals.var_eeff1__blk1033_dn8, locals.var_eeff1__blk1033_dn9,)
    }
};
        locals.var_eeff1__blk1033 = assign40500_e46250;
        locals.var_eeff1__blk1033_dn4 = assign40500_e46250_d_n4;
        locals.var_eeff1__blk1033_dn6 = assign40500_e46250_d_n6;
        locals.var_eeff1__blk1033_dn7 = assign40500_e46250_d_n7;
        locals.var_eeff1__blk1033_dn8 = assign40500_e46250_d_n8;
        locals.var_eeff1__blk1033_dn9 = assign40500_e46250_d_n9;

        let (assign40510_e46258, assign40510_e46258_d_n4, assign40510_e46258_d_n6, assign40510_e46258_d_n7, assign40510_e46258_d_n8, assign40510_e46258_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40510_e46255: f64 = (locals.var_eeff2s__blk957 + locals.var_eeff2d__blk1026);
        let assign40510_e46256: f64 = (0.5 * assign40510_e46255);
        (assign40510_e46256, (0.5 * (locals.var_eeff2s__blk957_dn4 + locals.var_eeff2d__blk1026_dn4)), (0.5 * (locals.var_eeff2s__blk957_dn6 + locals.var_eeff2d__blk1026_dn6)), (0.5 * (locals.var_eeff2s__blk957_dn7 + locals.var_eeff2d__blk1026_dn7)), (0.5 * (locals.var_eeff2s__blk957_dn8 + locals.var_eeff2d__blk1026_dn8)), (0.5 * (locals.var_eeff2s__blk957_dn9 + locals.var_eeff2d__blk1026_dn9)),)
    } else {
        (locals.var_eeff2__blk1034, locals.var_eeff2__blk1034_dn4, locals.var_eeff2__blk1034_dn6, locals.var_eeff2__blk1034_dn7, locals.var_eeff2__blk1034_dn8, locals.var_eeff2__blk1034_dn9,)
    }
};
        locals.var_eeff2__blk1034 = assign40510_e46258;
        locals.var_eeff2__blk1034_dn4 = assign40510_e46258_d_n4;
        locals.var_eeff2__blk1034_dn6 = assign40510_e46258_d_n6;
        locals.var_eeff2__blk1034_dn7 = assign40510_e46258_d_n7;
        locals.var_eeff2__blk1034_dn8 = assign40510_e46258_d_n8;
        locals.var_eeff2__blk1034_dn9 = assign40510_e46258_d_n9;

        let (assign40520_e46271, assign40520_e46271_d_n4, assign40520_e46271_d_n6, assign40520_e46271_d_n7, assign40520_e46271_d_n8, assign40520_e46271_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40520_e46262: f64 = (locals.var_esurf1__blk1027 * locals.var_betn1_t);
        let assign40520_e46265: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign40520_e46266: f64 = (assign40520_e46265).exp();
        let assign40520_e46267: f64 = (assign40520_e46262 * assign40520_e46266);
        let assign40520_e46269: f64 = (assign40520_e46267 * locals.var_ratio_pd__blk1020);
        (assign40520_e46269, ((((((locals.var_esurf1__blk1027_dn4 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn4)) * assign40520_e46266) + (assign40520_e46262 * (assign40520_e46266 * (locals.var_stbet_i * locals.var_lnrtn_dn4)))) * locals.var_ratio_pd__blk1020) + (assign40520_e46267 * locals.var_ratio_pd__blk1020_dn4)), ((((((locals.var_esurf1__blk1027_dn6 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn6)) * assign40520_e46266) + (assign40520_e46262 * (assign40520_e46266 * (locals.var_stbet_i * locals.var_lnrtn_dn6)))) * locals.var_ratio_pd__blk1020) + (assign40520_e46267 * locals.var_ratio_pd__blk1020_dn6)), ((((((locals.var_esurf1__blk1027_dn7 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn7)) * assign40520_e46266) + (assign40520_e46262 * (assign40520_e46266 * (locals.var_stbet_i * locals.var_lnrtn_dn7)))) * locals.var_ratio_pd__blk1020) + (assign40520_e46267 * locals.var_ratio_pd__blk1020_dn7)), ((((((locals.var_esurf1__blk1027_dn8 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn8)) * assign40520_e46266) + (assign40520_e46262 * (assign40520_e46266 * (locals.var_stbet_i * locals.var_lnrtn_dn8)))) * locals.var_ratio_pd__blk1020) + (assign40520_e46267 * locals.var_ratio_pd__blk1020_dn8)), ((((((locals.var_esurf1__blk1027_dn9 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn9)) * assign40520_e46266) + (assign40520_e46262 * (assign40520_e46266 * (locals.var_stbet_i * locals.var_lnrtn_dn9)))) * locals.var_ratio_pd__blk1020) + (assign40520_e46267 * locals.var_ratio_pd__blk1020_dn9)),)
    } else {
        (locals.var_c1__blk1035, locals.var_c1__blk1035_dn4, locals.var_c1__blk1035_dn6, locals.var_c1__blk1035_dn7, locals.var_c1__blk1035_dn8, locals.var_c1__blk1035_dn9,)
    }
};
        locals.var_c1__blk1035 = assign40520_e46271;
        locals.var_c1__blk1035_dn4 = assign40520_e46271_d_n4;
        locals.var_c1__blk1035_dn6 = assign40520_e46271_d_n6;
        locals.var_c1__blk1035_dn7 = assign40520_e46271_d_n7;
        locals.var_c1__blk1035_dn8 = assign40520_e46271_d_n8;
        locals.var_c1__blk1035_dn9 = assign40520_e46271_d_n9;

        let (assign40530_e46282, assign40530_e46282_d_n4, assign40530_e46282_d_n6, assign40530_e46282_d_n7, assign40530_e46282_d_n8, assign40530_e46282_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40530_e46275: f64 = (locals.var_esurf2__blk1028 * locals.var_betn2_t);
        let assign40530_e46278: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign40530_e46279: f64 = (assign40530_e46278).exp();
        let assign40530_e46280: f64 = (assign40530_e46275 * assign40530_e46279);
        (assign40530_e46280, ((((locals.var_esurf2__blk1028_dn4 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn4)) * assign40530_e46279) + (assign40530_e46275 * (assign40530_e46279 * (locals.var_stbet_i * locals.var_lnrtn_dn4)))), ((((locals.var_esurf2__blk1028_dn6 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn6)) * assign40530_e46279) + (assign40530_e46275 * (assign40530_e46279 * (locals.var_stbet_i * locals.var_lnrtn_dn6)))), ((((locals.var_esurf2__blk1028_dn7 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn7)) * assign40530_e46279) + (assign40530_e46275 * (assign40530_e46279 * (locals.var_stbet_i * locals.var_lnrtn_dn7)))), ((((locals.var_esurf2__blk1028_dn8 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn8)) * assign40530_e46279) + (assign40530_e46275 * (assign40530_e46279 * (locals.var_stbet_i * locals.var_lnrtn_dn8)))), ((((locals.var_esurf2__blk1028_dn9 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn9)) * assign40530_e46279) + (assign40530_e46275 * (assign40530_e46279 * (locals.var_stbet_i * locals.var_lnrtn_dn9)))),)
    } else {
        (locals.var_c2__blk1036, locals.var_c2__blk1036_dn4, locals.var_c2__blk1036_dn6, locals.var_c2__blk1036_dn7, locals.var_c2__blk1036_dn8, locals.var_c2__blk1036_dn9,)
    }
};
        locals.var_c2__blk1036 = assign40530_e46282;
        locals.var_c2__blk1036_dn4 = assign40530_e46282_d_n4;
        locals.var_c2__blk1036_dn6 = assign40530_e46282_d_n6;
        locals.var_c2__blk1036_dn7 = assign40530_e46282_d_n7;
        locals.var_c2__blk1036_dn8 = assign40530_e46282_d_n8;
        locals.var_c2__blk1036_dn9 = assign40530_e46282_d_n9;

        let (assign40540_e46288, assign40540_e46288_d_n4, assign40540_e46288_d_n6, assign40540_e46288_d_n7, assign40540_e46288_d_n8, assign40540_e46288_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40540_e46286: f64 = (locals.var_c1__blk1035 + locals.var_c2__blk1036);
        (assign40540_e46286, (locals.var_c1__blk1035_dn4 + locals.var_c2__blk1036_dn4), (locals.var_c1__blk1035_dn6 + locals.var_c2__blk1036_dn6), (locals.var_c1__blk1035_dn7 + locals.var_c2__blk1036_dn7), (locals.var_c1__blk1035_dn8 + locals.var_c2__blk1036_dn8), (locals.var_c1__blk1035_dn9 + locals.var_c2__blk1036_dn9),)
    } else {
        (locals.var_csum__blk1037, locals.var_csum__blk1037_dn4, locals.var_csum__blk1037_dn6, locals.var_csum__blk1037_dn7, locals.var_csum__blk1037_dn8, locals.var_csum__blk1037_dn9,)
    }
};
        locals.var_csum__blk1037 = assign40540_e46288;
        locals.var_csum__blk1037_dn4 = assign40540_e46288_d_n4;
        locals.var_csum__blk1037_dn6 = assign40540_e46288_d_n6;
        locals.var_csum__blk1037_dn7 = assign40540_e46288_d_n7;
        locals.var_csum__blk1037_dn8 = assign40540_e46288_d_n8;
        locals.var_csum__blk1037_dn9 = assign40540_e46288_d_n9;

        let (assign40550_e46298, assign40550_e46298_d_n4, assign40550_e46298_d_n6, assign40550_e46298_d_n7, assign40550_e46298_d_n8, assign40550_e46298_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40550_e46294: f64 = (locals.var_xcorb_i * locals.var_ecpl2__blk1032);
        let assign40550_e46295: f64 = (locals.var_ecpl1__blk1031 + assign40550_e46294);
        let assign40550_e46296: f64 = (locals.var_xcor_i * assign40550_e46295);
        (assign40550_e46296, ((locals.var_xcor_i_dn4 * assign40550_e46295) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn4 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn4)))), ((locals.var_xcor_i_dn6 * assign40550_e46295) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn6 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn6)))), ((locals.var_xcor_i_dn7 * assign40550_e46295) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn7 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn7)))), ((locals.var_xcor_i_dn8 * assign40550_e46295) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn8 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn8)))), ((locals.var_xcor_i_dn9 * assign40550_e46295) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn9 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn9)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40550_e46298;
        locals.var_temp1_dn4 = assign40550_e46298_d_n4;
        locals.var_temp1_dn6 = assign40550_e46298_d_n6;
        locals.var_temp1_dn7 = assign40550_e46298_d_n7;
        locals.var_temp1_dn8 = assign40550_e46298_d_n8;
        locals.var_temp1_dn9 = assign40550_e46298_d_n9;

        let (assign40560_e46323, assign40560_e46323_d_n4, assign40560_e46323_d_n6, assign40560_e46323_d_n7, assign40560_e46323_d_n8, assign40560_e46323_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40560_e46303: f64 = (1.0 + locals.var_temp1);
        let assign40560_e46305: f64 = assign40560_e46303;
        let assign40560_e46308: f64 = (1.0 + locals.var_temp1);
        let assign40560_e46310: f64 = assign40560_e46308;
        let assign40560_e46313: f64 = (1.0 + locals.var_temp1);
        let assign40560_e46315: f64 = assign40560_e46313;
        let assign40560_e46316: f64 = (assign40560_e46310 * assign40560_e46315);
        let assign40560_e46318: f64 = (assign40560_e46316 + 0.01);
        let assign40560_e46319: f64 = (assign40560_e46318).sqrt();
        let assign40560_e46320: f64 = (assign40560_e46305 + assign40560_e46319);
        let assign40560_e46321: f64 = (0.5 * assign40560_e46320);
        (assign40560_e46321, (0.5 * (locals.var_temp1_dn4 + (((locals.var_temp1_dn4 * assign40560_e46315) + (assign40560_e46310 * locals.var_temp1_dn4)) / (2.0 * assign40560_e46319)))), (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign40560_e46315) + (assign40560_e46310 * locals.var_temp1_dn6)) / (2.0 * assign40560_e46319)))), (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign40560_e46315) + (assign40560_e46310 * locals.var_temp1_dn7)) / (2.0 * assign40560_e46319)))), (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign40560_e46315) + (assign40560_e46310 * locals.var_temp1_dn8)) / (2.0 * assign40560_e46319)))), (0.5 * (locals.var_temp1_dn9 + (((locals.var_temp1_dn9 * assign40560_e46315) + (assign40560_e46310 * locals.var_temp1_dn9)) / (2.0 * assign40560_e46319)))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign40560_e46323;
        locals.var_temp2_dn4 = assign40560_e46323_d_n4;
        locals.var_temp2_dn6 = assign40560_e46323_d_n6;
        locals.var_temp2_dn7 = assign40560_e46323_d_n7;
        locals.var_temp2_dn8 = assign40560_e46323_d_n8;
        locals.var_temp2_dn9 = assign40560_e46323_d_n9;

        let (assign40570_e46354, assign40570_e46354_d_n4, assign40570_e46354_d_n6, assign40570_e46354_d_n7, assign40570_e46354_d_n8, assign40570_e46354_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40570_e46329: f64 = (0.2 * locals.var_temp1);
        let assign40570_e46330: f64 = (1.0 + assign40570_e46329);
        let assign40570_e46332: f64 = assign40570_e46330;
        let assign40570_e46336: f64 = (0.2 * locals.var_temp1);
        let assign40570_e46337: f64 = (1.0 + assign40570_e46336);
        let assign40570_e46339: f64 = assign40570_e46337;
        let assign40570_e46343: f64 = (0.2 * locals.var_temp1);
        let assign40570_e46344: f64 = (1.0 + assign40570_e46343);
        let assign40570_e46346: f64 = assign40570_e46344;
        let assign40570_e46347: f64 = (assign40570_e46339 * assign40570_e46346);
        let assign40570_e46349: f64 = (assign40570_e46347 + 0.01);
        let assign40570_e46350: f64 = (assign40570_e46349).sqrt();
        let assign40570_e46351: f64 = (assign40570_e46332 + assign40570_e46350);
        let assign40570_e46352: f64 = (0.5 * assign40570_e46351);
        (assign40570_e46352, (0.5 * ((0.2 * locals.var_temp1_dn4) + ((((0.2 * locals.var_temp1_dn4) * assign40570_e46346) + (assign40570_e46339 * (0.2 * locals.var_temp1_dn4))) / (2.0 * assign40570_e46350)))), (0.5 * ((0.2 * locals.var_temp1_dn6) + ((((0.2 * locals.var_temp1_dn6) * assign40570_e46346) + (assign40570_e46339 * (0.2 * locals.var_temp1_dn6))) / (2.0 * assign40570_e46350)))), (0.5 * ((0.2 * locals.var_temp1_dn7) + ((((0.2 * locals.var_temp1_dn7) * assign40570_e46346) + (assign40570_e46339 * (0.2 * locals.var_temp1_dn7))) / (2.0 * assign40570_e46350)))), (0.5 * ((0.2 * locals.var_temp1_dn8) + ((((0.2 * locals.var_temp1_dn8) * assign40570_e46346) + (assign40570_e46339 * (0.2 * locals.var_temp1_dn8))) / (2.0 * assign40570_e46350)))), (0.5 * ((0.2 * locals.var_temp1_dn9) + ((((0.2 * locals.var_temp1_dn9) * assign40570_e46346) + (assign40570_e46339 * (0.2 * locals.var_temp1_dn9))) / (2.0 * assign40570_e46350)))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign40570_e46354;
        locals.var_temp3_dn4 = assign40570_e46354_d_n4;
        locals.var_temp3_dn6 = assign40570_e46354_d_n6;
        locals.var_temp3_dn7 = assign40570_e46354_d_n7;
        locals.var_temp3_dn8 = assign40570_e46354_d_n8;
        locals.var_temp3_dn9 = assign40570_e46354_d_n9;

        let (assign40580_e46360, assign40580_e46360_d_n4, assign40580_e46360_d_n6, assign40580_e46360_d_n7, assign40580_e46360_d_n8, assign40580_e46360_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40580_e46358: f64 = (locals.var_temp2 / locals.var_temp3);
        (assign40580_e46358, (((locals.var_temp2_dn4 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn4)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn6 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn6)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn7 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn7)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn8 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn8)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn9 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn9)) / (locals.var_temp3 * locals.var_temp3)),)
    } else {
        (locals.var_fcor__blk1038, locals.var_fcor__blk1038_dn4, locals.var_fcor__blk1038_dn6, locals.var_fcor__blk1038_dn7, locals.var_fcor__blk1038_dn8, locals.var_fcor__blk1038_dn9,)
    }
};
        locals.var_fcor__blk1038 = assign40580_e46360;
        locals.var_fcor__blk1038_dn4 = assign40580_e46360_d_n4;
        locals.var_fcor__blk1038_dn6 = assign40580_e46360_d_n6;
        locals.var_fcor__blk1038_dn7 = assign40580_e46360_d_n7;
        locals.var_fcor__blk1038_dn8 = assign40580_e46360_d_n8;
        locals.var_fcor__blk1038_dn9 = assign40580_e46360_d_n9;

        let (assign40590_e46389, assign40590_e46389_d_n4, assign40590_e46389_d_n6, assign40590_e46389_d_n7, assign40590_e46389_d_n8, assign40590_e46389_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40590_e46366: f64 = (locals.var_csfi_i * locals.var_ecpl1__blk1031);
        let assign40590_e46367: f64 = (1.0 + assign40590_e46366);
        let assign40590_e46370: f64 = (locals.var_csbi_i * locals.var_ecpl2__blk1032);
        let assign40590_e46371: f64 = (assign40590_e46367 + assign40590_e46370);
        let assign40590_e46372: f64 = (locals.var_cs_i * assign40590_e46371);
        let assign40590_e46374: f64 = (-locals.var_thecs_i);
        let assign40590_e46378: f64 = (locals.var_qi1m__blk1029 * locals.var_inv_qi1cs);
        let assign40590_e46379: f64 = (1.0 + assign40590_e46378);
        let assign40590_e46382: f64 = (locals.var_qi2m__blk1030 * locals.var_inv_qi2cs);
        let assign40590_e46383: f64 = (assign40590_e46379 + assign40590_e46382);
        let assign40590_e46384: f64 = (assign40590_e46383).ln();
        let assign40590_e46385: f64 = (assign40590_e46374 * assign40590_e46384);
        let assign40590_e46386: f64 = (assign40590_e46385).exp();
        let assign40590_e46387: f64 = (assign40590_e46372 * assign40590_e46386);
        (assign40590_e46387, ((((locals.var_cs_i_dn4 * assign40590_e46371) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn4) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn4)))) * assign40590_e46386) + (assign40590_e46372 * (assign40590_e46386 * (((-locals.var_thecs_i_dn4) * assign40590_e46384) + (assign40590_e46374 * (((locals.var_qi1m__blk1029_dn4 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn4 * locals.var_inv_qi2cs)) / assign40590_e46383)))))), ((((locals.var_cs_i_dn6 * assign40590_e46371) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn6) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn6)))) * assign40590_e46386) + (assign40590_e46372 * (assign40590_e46386 * (((-locals.var_thecs_i_dn6) * assign40590_e46384) + (assign40590_e46374 * (((locals.var_qi1m__blk1029_dn6 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn6 * locals.var_inv_qi2cs)) / assign40590_e46383)))))), ((((locals.var_cs_i_dn7 * assign40590_e46371) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn7) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn7)))) * assign40590_e46386) + (assign40590_e46372 * (assign40590_e46386 * (((-locals.var_thecs_i_dn7) * assign40590_e46384) + (assign40590_e46374 * (((locals.var_qi1m__blk1029_dn7 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn7 * locals.var_inv_qi2cs)) / assign40590_e46383)))))), ((((locals.var_cs_i_dn8 * assign40590_e46371) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn8) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn8)))) * assign40590_e46386) + (assign40590_e46372 * (assign40590_e46386 * (((-locals.var_thecs_i_dn8) * assign40590_e46384) + (assign40590_e46374 * (((locals.var_qi1m__blk1029_dn8 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn8 * locals.var_inv_qi2cs)) / assign40590_e46383)))))), ((((locals.var_cs_i_dn9 * assign40590_e46371) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn9) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn9)))) * assign40590_e46386) + (assign40590_e46372 * (assign40590_e46386 * (((-locals.var_thecs_i_dn9) * assign40590_e46384) + (assign40590_e46374 * (((locals.var_qi1m__blk1029_dn9 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn9 * locals.var_inv_qi2cs)) / assign40590_e46383)))))),)
    } else {
        (locals.var_gcs__blk1039, locals.var_gcs__blk1039_dn4, locals.var_gcs__blk1039_dn6, locals.var_gcs__blk1039_dn7, locals.var_gcs__blk1039_dn8, locals.var_gcs__blk1039_dn9,)
    }
};
        locals.var_gcs__blk1039 = assign40590_e46389;
        locals.var_gcs__blk1039_dn4 = assign40590_e46389_d_n4;
        locals.var_gcs__blk1039_dn6 = assign40590_e46389_d_n6;
        locals.var_gcs__blk1039_dn7 = assign40590_e46389_d_n7;
        locals.var_gcs__blk1039_dn8 = assign40590_e46389_d_n8;
        locals.var_gcs__blk1039_dn9 = assign40590_e46389_d_n9;

        let assign40600_e46392: f64 = if locals.var_rsg_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign40600_e46392;

        let (assign40610_e46398, assign40610_e46398_d_n4, assign40610_e46398_d_n6, assign40610_e46398_d_n7, assign40610_e46398_d_n8, assign40610_e46398_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1222 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign40610_e46398;
        locals.var_temp3_dn4 = assign40610_e46398_d_n4;
        locals.var_temp3_dn6 = assign40610_e46398_d_n6;
        locals.var_temp3_dn7 = assign40610_e46398_d_n7;
        locals.var_temp3_dn8 = assign40610_e46398_d_n8;
        locals.var_temp3_dn9 = assign40610_e46398_d_n9;

        let assign40620_e46401: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1223 = assign40620_e46401;

        let (assign40630_e46418, assign40630_e46418_d_n4, assign40630_e46418_d_n6, assign40630_e46418_d_n7, assign40630_e46418_d_n8, assign40630_e46418_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 != 0.0)) {
        let assign40630_e46412: f64 = (locals.var_qim__blk1016 + 1e-12);
        let assign40630_e46413: f64 = (assign40630_e46412).ln();
        let assign40630_e46414: f64 = (locals.var_thersg_i * assign40630_e46413);
        let assign40630_e46415: f64 = (assign40630_e46414).exp();
        let assign40630_e46416: f64 = (locals.var_rsg_i * assign40630_e46415);
        (assign40630_e46416, (locals.var_rsg_i * (assign40630_e46415 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn4 / assign40630_e46412)))), (locals.var_rsg_i * (assign40630_e46415 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn6 / assign40630_e46412)))), (locals.var_rsg_i * (assign40630_e46415 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn7 / assign40630_e46412)))), (locals.var_rsg_i * (assign40630_e46415 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn8 / assign40630_e46412)))), (locals.var_rsg_i * (assign40630_e46415 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn9 / assign40630_e46412)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40630_e46418;
        locals.var_temp1_dn4 = assign40630_e46418_d_n4;
        locals.var_temp1_dn6 = assign40630_e46418_d_n6;
        locals.var_temp1_dn7 = assign40630_e46418_d_n7;
        locals.var_temp1_dn8 = assign40630_e46418_d_n8;
        locals.var_temp1_dn9 = assign40630_e46418_d_n9;

        let (assign40640_e46429, assign40640_e46429_d_n4, assign40640_e46429_d_n6, assign40640_e46429_d_n7, assign40640_e46429_d_n8, assign40640_e46429_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 != 0.0)) {
        let assign40640_e46427: f64 = (1.0 - locals.var_temp1);
        (assign40640_e46427, (-locals.var_temp1_dn4), (-locals.var_temp1_dn6), (-locals.var_temp1_dn7), (-locals.var_temp1_dn8), (-locals.var_temp1_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign40640_e46429;
        locals.var_temp3_dn4 = assign40640_e46429_d_n4;
        locals.var_temp3_dn6 = assign40640_e46429_d_n6;
        locals.var_temp3_dn7 = assign40640_e46429_d_n7;
        locals.var_temp3_dn8 = assign40640_e46429_d_n8;
        locals.var_temp3_dn9 = assign40640_e46429_d_n9;

        let (assign40650_e46447, assign40650_e46447_d_n4, assign40650_e46447_d_n6, assign40650_e46447_d_n7, assign40650_e46447_d_n8, assign40650_e46447_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 == 0.0)) {
        let assign40650_e46441: f64 = (locals.var_qim__blk1016 + 1e-12);
        let assign40650_e46442: f64 = (assign40650_e46441).ln();
        let assign40650_e46443: f64 = (locals.var_thersg_i * assign40650_e46442);
        let assign40650_e46444: f64 = (assign40650_e46443).exp();
        let assign40650_e46445: f64 = (locals.var_rsg_i * assign40650_e46444);
        (assign40650_e46445, (locals.var_rsg_i * (assign40650_e46444 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn4 / assign40650_e46441)))), (locals.var_rsg_i * (assign40650_e46444 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn6 / assign40650_e46441)))), (locals.var_rsg_i * (assign40650_e46444 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn7 / assign40650_e46441)))), (locals.var_rsg_i * (assign40650_e46444 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn8 / assign40650_e46441)))), (locals.var_rsg_i * (assign40650_e46444 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn9 / assign40650_e46441)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40650_e46447;
        locals.var_temp1_dn4 = assign40650_e46447_d_n4;
        locals.var_temp1_dn6 = assign40650_e46447_d_n6;
        locals.var_temp1_dn7 = assign40650_e46447_d_n7;
        locals.var_temp1_dn8 = assign40650_e46447_d_n8;
        locals.var_temp1_dn9 = assign40650_e46447_d_n9;

        let (assign40660_e46461, assign40660_e46461_d_n4, assign40660_e46461_d_n6, assign40660_e46461_d_n7, assign40660_e46461_d_n8, assign40660_e46461_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 == 0.0)) {
        let assign40660_e46458: f64 = (1.0 + locals.var_temp1);
        let assign40660_e46459: f64 = (1.0 / assign40660_e46458);
        (assign40660_e46459, (-(locals.var_temp1_dn4 / (assign40660_e46458 * assign40660_e46458))), (-(locals.var_temp1_dn6 / (assign40660_e46458 * assign40660_e46458))), (-(locals.var_temp1_dn7 / (assign40660_e46458 * assign40660_e46458))), (-(locals.var_temp1_dn8 / (assign40660_e46458 * assign40660_e46458))), (-(locals.var_temp1_dn9 / (assign40660_e46458 * assign40660_e46458))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign40660_e46461;
        locals.var_temp3_dn4 = assign40660_e46461_d_n4;
        locals.var_temp3_dn6 = assign40660_e46461_d_n6;
        locals.var_temp3_dn7 = assign40660_e46461_d_n7;
        locals.var_temp3_dn8 = assign40660_e46461_d_n8;
        locals.var_temp3_dn9 = assign40660_e46461_d_n9;

        let (assign40670_e46471, assign40670_e46471_d_n4, assign40670_e46471_d_n6, assign40670_e46471_d_n7, assign40670_e46471_d_n8, assign40670_e46471_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40670_e46466: f64 = (locals.var_qim__blk1016 * locals.var_temp3);
        let assign40670_e46468: f64 = (assign40670_e46466 + locals.var_rsig_i);
        let assign40670_e46469: f64 = (locals.var_frscsi__blk964 * assign40670_e46468);
        (assign40670_e46469, ((locals.var_frscsi__blk964_dn4 * assign40670_e46468) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn4 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn4)))), ((locals.var_frscsi__blk964_dn6 * assign40670_e46468) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn6 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn6)))), ((locals.var_frscsi__blk964_dn7 * assign40670_e46468) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn7 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn7)))), ((locals.var_frscsi__blk964_dn8 * assign40670_e46468) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn8 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn8)))), ((locals.var_frscsi__blk964_dn9 * assign40670_e46468) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn9 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn9)))),)
    } else {
        (locals.var_grs__blk1040, locals.var_grs__blk1040_dn4, locals.var_grs__blk1040_dn6, locals.var_grs__blk1040_dn7, locals.var_grs__blk1040_dn8, locals.var_grs__blk1040_dn9,)
    }
};
        locals.var_grs__blk1040 = assign40670_e46471;
        locals.var_grs__blk1040_dn4 = assign40670_e46471_d_n4;
        locals.var_grs__blk1040_dn6 = assign40670_e46471_d_n6;
        locals.var_grs__blk1040_dn7 = assign40670_e46471_d_n7;
        locals.var_grs__blk1040_dn8 = assign40670_e46471_d_n8;
        locals.var_grs__blk1040_dn9 = assign40670_e46471_d_n9;

        let (assign40680_e46491, assign40680_e46491_d_n4, assign40680_e46491_d_n6, assign40680_e46491_d_n7, assign40680_e46491_d_n8, assign40680_e46491_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40680_e46477: f64 = (locals.var_fmue * locals.var_eeff1__blk1033);
        let assign40680_e46479: f64 = (assign40680_e46477 + 1e-6);
        let assign40680_e46480: f64 = (assign40680_e46479).ln();
        let assign40680_e46481: f64 = (locals.var_themu_i * assign40680_e46480);
        let assign40680_e46482: f64 = (assign40680_e46481).exp();
        let assign40680_e46483: f64 = (1.0 + assign40680_e46482);
        let assign40680_e46485: f64 = (assign40680_e46483 + locals.var_gcs__blk1039);
        let assign40680_e46488: f64 = (locals.var_betn1_i * locals.var_grs__blk1040);
        let assign40680_e46489: f64 = (assign40680_e46485 + assign40680_e46488);
        (assign40680_e46489, (((assign40680_e46482 * ((locals.var_themu_i_dn4 * assign40680_e46480) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn4)) / assign40680_e46479)))) + locals.var_gcs__blk1039_dn4) + ((locals.var_betn1_i_dn4 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn4))), (((assign40680_e46482 * ((locals.var_themu_i_dn6 * assign40680_e46480) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn6)) / assign40680_e46479)))) + locals.var_gcs__blk1039_dn6) + ((locals.var_betn1_i_dn6 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn6))), (((assign40680_e46482 * ((locals.var_themu_i_dn7 * assign40680_e46480) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn7)) / assign40680_e46479)))) + locals.var_gcs__blk1039_dn7) + ((locals.var_betn1_i_dn7 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn7))), (((assign40680_e46482 * ((locals.var_themu_i_dn8 * assign40680_e46480) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn8)) / assign40680_e46479)))) + locals.var_gcs__blk1039_dn8) + ((locals.var_betn1_i_dn8 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn8))), (((assign40680_e46482 * ((locals.var_themu_i_dn9 * assign40680_e46480) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn9)) / assign40680_e46479)))) + locals.var_gcs__blk1039_dn9) + ((locals.var_betn1_i_dn9 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn9))),)
    } else {
        (locals.var_gmob1__blk1041, locals.var_gmob1__blk1041_dn4, locals.var_gmob1__blk1041_dn6, locals.var_gmob1__blk1041_dn7, locals.var_gmob1__blk1041_dn8, locals.var_gmob1__blk1041_dn9,)
    }
};
        locals.var_gmob1__blk1041 = assign40680_e46491;
        locals.var_gmob1__blk1041_dn4 = assign40680_e46491_d_n4;
        locals.var_gmob1__blk1041_dn6 = assign40680_e46491_d_n6;
        locals.var_gmob1__blk1041_dn7 = assign40680_e46491_d_n7;
        locals.var_gmob1__blk1041_dn8 = assign40680_e46491_d_n8;
        locals.var_gmob1__blk1041_dn9 = assign40680_e46491_d_n9;

    }

    pub(super) fn stamp_transient_block_113(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign40690_e46511, assign40690_e46511_d_n4, assign40690_e46511_d_n6, assign40690_e46511_d_n7, assign40690_e46511_d_n8, assign40690_e46511_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40690_e46497: f64 = (locals.var_fmue * locals.var_eeff2__blk1034);
        let assign40690_e46499: f64 = (assign40690_e46497 + 1e-6);
        let assign40690_e46500: f64 = (assign40690_e46499).ln();
        let assign40690_e46501: f64 = (locals.var_themu_i * assign40690_e46500);
        let assign40690_e46502: f64 = (assign40690_e46501).exp();
        let assign40690_e46503: f64 = (1.0 + assign40690_e46502);
        let assign40690_e46505: f64 = (assign40690_e46503 + locals.var_gcs__blk1039);
        let assign40690_e46508: f64 = (locals.var_betn2_i * locals.var_grs__blk1040);
        let assign40690_e46509: f64 = (assign40690_e46505 + assign40690_e46508);
        (assign40690_e46509, (((assign40690_e46502 * ((locals.var_themu_i_dn4 * assign40690_e46500) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn4)) / assign40690_e46499)))) + locals.var_gcs__blk1039_dn4) + ((locals.var_betn2_i_dn4 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn4))), (((assign40690_e46502 * ((locals.var_themu_i_dn6 * assign40690_e46500) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn6)) / assign40690_e46499)))) + locals.var_gcs__blk1039_dn6) + ((locals.var_betn2_i_dn6 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn6))), (((assign40690_e46502 * ((locals.var_themu_i_dn7 * assign40690_e46500) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn7)) / assign40690_e46499)))) + locals.var_gcs__blk1039_dn7) + ((locals.var_betn2_i_dn7 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn7))), (((assign40690_e46502 * ((locals.var_themu_i_dn8 * assign40690_e46500) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn8)) / assign40690_e46499)))) + locals.var_gcs__blk1039_dn8) + ((locals.var_betn2_i_dn8 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn8))), (((assign40690_e46502 * ((locals.var_themu_i_dn9 * assign40690_e46500) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn9)) / assign40690_e46499)))) + locals.var_gcs__blk1039_dn9) + ((locals.var_betn2_i_dn9 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn9))),)
    } else {
        (locals.var_gmob2__blk1042, locals.var_gmob2__blk1042_dn4, locals.var_gmob2__blk1042_dn6, locals.var_gmob2__blk1042_dn7, locals.var_gmob2__blk1042_dn8, locals.var_gmob2__blk1042_dn9,)
    }
};
        locals.var_gmob2__blk1042 = assign40690_e46511;
        locals.var_gmob2__blk1042_dn4 = assign40690_e46511_d_n4;
        locals.var_gmob2__blk1042_dn6 = assign40690_e46511_d_n6;
        locals.var_gmob2__blk1042_dn7 = assign40690_e46511_d_n7;
        locals.var_gmob2__blk1042_dn8 = assign40690_e46511_d_n8;
        locals.var_gmob2__blk1042_dn9 = assign40690_e46511_d_n9;

        let (assign40700_e46525, assign40700_e46525_d_n4, assign40700_e46525_d_n6, assign40700_e46525_d_n7, assign40700_e46525_d_n8, assign40700_e46525_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40700_e46515: f64 = (locals.var_fcor__blk1038 * locals.var_csum__blk1037);
        let assign40700_e46518: f64 = (locals.var_c1__blk1035 / locals.var_gmob1__blk1041);
        let assign40700_e46521: f64 = (locals.var_c2__blk1036 / locals.var_gmob2__blk1042);
        let assign40700_e46522: f64 = (assign40700_e46518 + assign40700_e46521);
        let assign40700_e46523: f64 = (assign40700_e46515 / assign40700_e46522);
        (assign40700_e46523, (((((locals.var_fcor__blk1038_dn4 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn4)) * assign40700_e46522) - (assign40700_e46515 * ((((locals.var_c1__blk1035_dn4 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn4)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn4 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn4)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40700_e46522 * assign40700_e46522)), (((((locals.var_fcor__blk1038_dn6 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn6)) * assign40700_e46522) - (assign40700_e46515 * ((((locals.var_c1__blk1035_dn6 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn6)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn6 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn6)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40700_e46522 * assign40700_e46522)), (((((locals.var_fcor__blk1038_dn7 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn7)) * assign40700_e46522) - (assign40700_e46515 * ((((locals.var_c1__blk1035_dn7 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn7)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn7 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn7)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40700_e46522 * assign40700_e46522)), (((((locals.var_fcor__blk1038_dn8 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn8)) * assign40700_e46522) - (assign40700_e46515 * ((((locals.var_c1__blk1035_dn8 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn8)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn8 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn8)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40700_e46522 * assign40700_e46522)), (((((locals.var_fcor__blk1038_dn9 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn9)) * assign40700_e46522) - (assign40700_e46515 * ((((locals.var_c1__blk1035_dn9 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn9)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn9 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn9)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40700_e46522 * assign40700_e46522)),)
    } else {
        (locals.var_gmob__blk1043, locals.var_gmob__blk1043_dn4, locals.var_gmob__blk1043_dn6, locals.var_gmob__blk1043_dn7, locals.var_gmob__blk1043_dn8, locals.var_gmob__blk1043_dn9,)
    }
};
        locals.var_gmob__blk1043 = assign40700_e46525;
        locals.var_gmob__blk1043_dn4 = assign40700_e46525_d_n4;
        locals.var_gmob__blk1043_dn6 = assign40700_e46525_d_n6;
        locals.var_gmob__blk1043_dn7 = assign40700_e46525_d_n7;
        locals.var_gmob__blk1043_dn8 = assign40700_e46525_d_n8;
        locals.var_gmob__blk1043_dn9 = assign40700_e46525_d_n9;

        let (assign40710_e46533, assign40710_e46533_d_n4, assign40710_e46533_d_n6, assign40710_e46533_d_n7, assign40710_e46533_d_n8, assign40710_e46533_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40710_e46530: f64 = (4.0 + locals.var_qim__blk1016);
        let assign40710_e46531: f64 = (1.0 / assign40710_e46530);
        (assign40710_e46531, (-(locals.var_qim__blk1016_dn4 / (assign40710_e46530 * assign40710_e46530))), (-(locals.var_qim__blk1016_dn6 / (assign40710_e46530 * assign40710_e46530))), (-(locals.var_qim__blk1016_dn7 / (assign40710_e46530 * assign40710_e46530))), (-(locals.var_qim__blk1016_dn8 / (assign40710_e46530 * assign40710_e46530))), (-(locals.var_qim__blk1016_dn9 / (assign40710_e46530 * assign40710_e46530))),)
    } else {
        (locals.var_inv_qimstar1__blk1044, locals.var_inv_qimstar1__blk1044_dn4, locals.var_inv_qimstar1__blk1044_dn6, locals.var_inv_qimstar1__blk1044_dn7, locals.var_inv_qimstar1__blk1044_dn8, locals.var_inv_qimstar1__blk1044_dn9,)
    }
};
        locals.var_inv_qimstar1__blk1044 = assign40710_e46533;
        locals.var_inv_qimstar1__blk1044_dn4 = assign40710_e46533_d_n4;
        locals.var_inv_qimstar1__blk1044_dn6 = assign40710_e46533_d_n6;
        locals.var_inv_qimstar1__blk1044_dn7 = assign40710_e46533_d_n7;
        locals.var_inv_qimstar1__blk1044_dn8 = assign40710_e46533_d_n8;
        locals.var_inv_qimstar1__blk1044_dn9 = assign40710_e46533_d_n9;

        let assign40720_e46536: f64 = if locals.var_alpb_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1224 = assign40720_e46536;

        let (assign40730_e46548, assign40730_e46548_d_n4, assign40730_e46548_d_n6, assign40730_e46548_d_n7, assign40730_e46548_d_n8, assign40730_e46548_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1224 != 0.0)) {
        let assign40730_e46544: f64 = (locals.var_alpb_i * locals.var_qi2m__blk1030);
        let assign40730_e46545: f64 = (1.0 + assign40730_e46544);
        let assign40730_e46546: f64 = (1.0 / assign40730_e46545);
        (assign40730_e46546, (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn4) / (assign40730_e46545 * assign40730_e46545))), (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn6) / (assign40730_e46545 * assign40730_e46545))), (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn7) / (assign40730_e46545 * assign40730_e46545))), (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn8) / (assign40730_e46545 * assign40730_e46545))), (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn9) / (assign40730_e46545 * assign40730_e46545))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign40730_e46548;
        locals.var_temp_dn4 = assign40730_e46548_d_n4;
        locals.var_temp_dn6 = assign40730_e46548_d_n6;
        locals.var_temp_dn7 = assign40730_e46548_d_n7;
        locals.var_temp_dn8 = assign40730_e46548_d_n8;
        locals.var_temp_dn9 = assign40730_e46548_d_n9;

        let (assign40740_e46559, assign40740_e46559_d_n4, assign40740_e46559_d_n6, assign40740_e46559_d_n7, assign40740_e46559_d_n8, assign40740_e46559_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1224 == 0.0)) {
        let assign40740_e46556: f64 = (locals.var_alpb_i * locals.var_qi2m__blk1030);
        let assign40740_e46557: f64 = (1.0 - assign40740_e46556);
        (assign40740_e46557, (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn4)), (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn6)), (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn7)), (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn8)), (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign40740_e46559;
        locals.var_temp_dn4 = assign40740_e46559_d_n4;
        locals.var_temp_dn6 = assign40740_e46559_d_n6;
        locals.var_temp_dn7 = assign40740_e46559_d_n7;
        locals.var_temp_dn8 = assign40740_e46559_d_n8;
        locals.var_temp_dn9 = assign40740_e46559_d_n9;

        let (assign40750_e46567, assign40750_e46567_d_n4, assign40750_e46567_d_n6, assign40750_e46567_d_n7, assign40750_e46567_d_n8, assign40750_e46567_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40750_e46563: f64 = (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044);
        let assign40750_e46565: f64 = (assign40750_e46563 * locals.var_temp);
        (assign40750_e46565, ((((locals.var_qim__blk1016_dn4 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn4)) * locals.var_temp) + (assign40750_e46563 * locals.var_temp_dn4)), ((((locals.var_qim__blk1016_dn6 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn6)) * locals.var_temp) + (assign40750_e46563 * locals.var_temp_dn6)), ((((locals.var_qim__blk1016_dn7 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn7)) * locals.var_temp) + (assign40750_e46563 * locals.var_temp_dn7)), ((((locals.var_qim__blk1016_dn8 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn8)) * locals.var_temp) + (assign40750_e46563 * locals.var_temp_dn8)), ((((locals.var_qim__blk1016_dn9 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn9)) * locals.var_temp) + (assign40750_e46563 * locals.var_temp_dn9)),)
    } else {
        (locals.var_r1__blk1045, locals.var_r1__blk1045_dn4, locals.var_r1__blk1045_dn6, locals.var_r1__blk1045_dn7, locals.var_r1__blk1045_dn8, locals.var_r1__blk1045_dn9,)
    }
};
        locals.var_r1__blk1045 = assign40750_e46567;
        locals.var_r1__blk1045_dn4 = assign40750_e46567_d_n4;
        locals.var_r1__blk1045_dn6 = assign40750_e46567_d_n6;
        locals.var_r1__blk1045_dn7 = assign40750_e46567_d_n7;
        locals.var_r1__blk1045_dn8 = assign40750_e46567_d_n8;
        locals.var_r1__blk1045_dn9 = assign40750_e46567_d_n9;

        let (assign40760_e46588, assign40760_e46588_d_n4, assign40760_e46588_d_n6, assign40760_e46588_d_n7, assign40760_e46588_d_n8, assign40760_e46588_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40760_e46572: f64 = (locals.var_xd - locals.var_xdeff__blk1000);
        let assign40760_e46575: f64 = (locals.var_vp_i * locals.var_inv_phit);
        let assign40760_e46578: f64 = (locals.var_vpg_i * locals.var_qim__blk1016);
        let assign40760_e46580: f64 = (assign40760_e46578 * locals.var_qim__blk1016);
        let assign40760_e46581: f64 = (assign40760_e46575 + assign40760_e46580);
        let assign40760_e46582: f64 = (assign40760_e46572 / assign40760_e46581);
        let assign40760_e46583: f64 = (1.0 + assign40760_e46582);
        let assign40760_e46584: f64 = (assign40760_e46583).ln();
        let assign40760_e46586: f64 = (assign40760_e46584 * locals.var_r1__blk1045);
        (assign40760_e46586, (((((((locals.var_xd_dn4 - locals.var_xdeff__blk1000_dn4) * assign40760_e46581) - (assign40760_e46572 * ((locals.var_vp_i * locals.var_inv_phit_dn4) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn4) * locals.var_qim__blk1016) + (assign40760_e46578 * locals.var_qim__blk1016_dn4))))) / (assign40760_e46581 * assign40760_e46581)) / assign40760_e46583) * locals.var_r1__blk1045) + (assign40760_e46584 * locals.var_r1__blk1045_dn4)), (((((((locals.var_xd_dn6 - locals.var_xdeff__blk1000_dn6) * assign40760_e46581) - (assign40760_e46572 * ((locals.var_vp_i * locals.var_inv_phit_dn6) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn6) * locals.var_qim__blk1016) + (assign40760_e46578 * locals.var_qim__blk1016_dn6))))) / (assign40760_e46581 * assign40760_e46581)) / assign40760_e46583) * locals.var_r1__blk1045) + (assign40760_e46584 * locals.var_r1__blk1045_dn6)), (((((((locals.var_xd_dn7 - locals.var_xdeff__blk1000_dn7) * assign40760_e46581) - (assign40760_e46572 * ((locals.var_vp_i * locals.var_inv_phit_dn7) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn7) * locals.var_qim__blk1016) + (assign40760_e46578 * locals.var_qim__blk1016_dn7))))) / (assign40760_e46581 * assign40760_e46581)) / assign40760_e46583) * locals.var_r1__blk1045) + (assign40760_e46584 * locals.var_r1__blk1045_dn7)), (((((((locals.var_xd_dn8 - locals.var_xdeff__blk1000_dn8) * assign40760_e46581) - (assign40760_e46572 * ((locals.var_vp_i * locals.var_inv_phit_dn8) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn8) * locals.var_qim__blk1016) + (assign40760_e46578 * locals.var_qim__blk1016_dn8))))) / (assign40760_e46581 * assign40760_e46581)) / assign40760_e46583) * locals.var_r1__blk1045) + (assign40760_e46584 * locals.var_r1__blk1045_dn8)), (((((((locals.var_xd_dn9 - locals.var_xdeff__blk1000_dn9) * assign40760_e46581) - (assign40760_e46572 * ((locals.var_vp_i * locals.var_inv_phit_dn9) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn9) * locals.var_qim__blk1016) + (assign40760_e46578 * locals.var_qim__blk1016_dn9))))) / (assign40760_e46581 * assign40760_e46581)) / assign40760_e46583) * locals.var_r1__blk1045) + (assign40760_e46584 * locals.var_r1__blk1045_dn9)),)
    } else {
        (locals.var_dl_l_fact__blk1046, locals.var_dl_l_fact__blk1046_dn4, locals.var_dl_l_fact__blk1046_dn6, locals.var_dl_l_fact__blk1046_dn7, locals.var_dl_l_fact__blk1046_dn8, locals.var_dl_l_fact__blk1046_dn9,)
    }
};
        locals.var_dl_l_fact__blk1046 = assign40760_e46588;
        locals.var_dl_l_fact__blk1046_dn4 = assign40760_e46588_d_n4;
        locals.var_dl_l_fact__blk1046_dn6 = assign40760_e46588_d_n6;
        locals.var_dl_l_fact__blk1046_dn7 = assign40760_e46588_d_n7;
        locals.var_dl_l_fact__blk1046_dn8 = assign40760_e46588_d_n8;
        locals.var_dl_l_fact__blk1046_dn9 = assign40760_e46588_d_n9;

        let (assign40770_e46594, assign40770_e46594_d_n4, assign40770_e46594_d_n6, assign40770_e46594_d_n7, assign40770_e46594_d_n8, assign40770_e46594_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40770_e46592: f64 = (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046);
        (assign40770_e46592, (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn4), (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn6), (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn7), (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn8), (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn9),)
    } else {
        (locals.var_dl_l__blk1047, locals.var_dl_l__blk1047_dn4, locals.var_dl_l__blk1047_dn6, locals.var_dl_l__blk1047_dn7, locals.var_dl_l__blk1047_dn8, locals.var_dl_l__blk1047_dn9,)
    }
};
        locals.var_dl_l__blk1047 = assign40770_e46594;
        locals.var_dl_l__blk1047_dn4 = assign40770_e46594_d_n4;
        locals.var_dl_l__blk1047_dn6 = assign40770_e46594_d_n6;
        locals.var_dl_l__blk1047_dn7 = assign40770_e46594_d_n7;
        locals.var_dl_l__blk1047_dn8 = assign40770_e46594_d_n8;
        locals.var_dl_l__blk1047_dn9 = assign40770_e46594_d_n9;

        let (assign40780_e46606, assign40780_e46606_d_n4, assign40780_e46606_d_n6, assign40780_e46606_d_n7, assign40780_e46606_d_n8, assign40780_e46606_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40780_e46601: f64 = (1.0 + locals.var_dl_l__blk1047);
        let assign40780_e46602: f64 = (locals.var_dl_l__blk1047 * assign40780_e46601);
        let assign40780_e46603: f64 = (1.0 + assign40780_e46602);
        let assign40780_e46604: f64 = (1.0 / assign40780_e46603);
        (assign40780_e46604, (-(((locals.var_dl_l__blk1047_dn4 * assign40780_e46601) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn4)) / (assign40780_e46603 * assign40780_e46603))), (-(((locals.var_dl_l__blk1047_dn6 * assign40780_e46601) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn6)) / (assign40780_e46603 * assign40780_e46603))), (-(((locals.var_dl_l__blk1047_dn7 * assign40780_e46601) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn7)) / (assign40780_e46603 * assign40780_e46603))), (-(((locals.var_dl_l__blk1047_dn8 * assign40780_e46601) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn8)) / (assign40780_e46603 * assign40780_e46603))), (-(((locals.var_dl_l__blk1047_dn9 * assign40780_e46601) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn9)) / (assign40780_e46603 * assign40780_e46603))),)
    } else {
        (locals.var_gdl__blk1048, locals.var_gdl__blk1048_dn4, locals.var_gdl__blk1048_dn6, locals.var_gdl__blk1048_dn7, locals.var_gdl__blk1048_dn8, locals.var_gdl__blk1048_dn9,)
    }
};
        locals.var_gdl__blk1048 = assign40780_e46606;
        locals.var_gdl__blk1048_dn4 = assign40780_e46606_d_n4;
        locals.var_gdl__blk1048_dn6 = assign40780_e46606_d_n6;
        locals.var_gdl__blk1048_dn7 = assign40780_e46606_d_n7;
        locals.var_gdl__blk1048_dn8 = assign40780_e46606_d_n8;
        locals.var_gdl__blk1048_dn9 = assign40780_e46606_d_n9;

        let (assign40790_e46616, assign40790_e46616_d_n4, assign40790_e46616_d_n6, assign40790_e46616_d_n7, assign40790_e46616_d_n8, assign40790_e46616_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40790_e46610: f64 = (100.0 * locals.var_esurf1__blk1027);
        let assign40790_e46613: f64 = (100.0 + locals.var_esurf1__blk1027);
        let assign40790_e46614: f64 = (assign40790_e46610 / assign40790_e46613);
        (assign40790_e46614, ((((100.0 * locals.var_esurf1__blk1027_dn4) * assign40790_e46613) - (assign40790_e46610 * locals.var_esurf1__blk1027_dn4)) / (assign40790_e46613 * assign40790_e46613)), ((((100.0 * locals.var_esurf1__blk1027_dn6) * assign40790_e46613) - (assign40790_e46610 * locals.var_esurf1__blk1027_dn6)) / (assign40790_e46613 * assign40790_e46613)), ((((100.0 * locals.var_esurf1__blk1027_dn7) * assign40790_e46613) - (assign40790_e46610 * locals.var_esurf1__blk1027_dn7)) / (assign40790_e46613 * assign40790_e46613)), ((((100.0 * locals.var_esurf1__blk1027_dn8) * assign40790_e46613) - (assign40790_e46610 * locals.var_esurf1__blk1027_dn8)) / (assign40790_e46613 * assign40790_e46613)), ((((100.0 * locals.var_esurf1__blk1027_dn9) * assign40790_e46613) - (assign40790_e46610 * locals.var_esurf1__blk1027_dn9)) / (assign40790_e46613 * assign40790_e46613)),)
    } else {
        (locals.var_wsat1__blk976, locals.var_wsat1__blk976_dn4, locals.var_wsat1__blk976_dn6, locals.var_wsat1__blk976_dn7, locals.var_wsat1__blk976_dn8, locals.var_wsat1__blk976_dn9,)
    }
};
        locals.var_wsat1__blk976 = assign40790_e46616;
        locals.var_wsat1__blk976_dn4 = assign40790_e46616_d_n4;
        locals.var_wsat1__blk976_dn6 = assign40790_e46616_d_n6;
        locals.var_wsat1__blk976_dn7 = assign40790_e46616_d_n7;
        locals.var_wsat1__blk976_dn8 = assign40790_e46616_d_n8;
        locals.var_wsat1__blk976_dn9 = assign40790_e46616_d_n9;

        let assign40800_e46619: f64 = if locals.var_thesat1_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1225 = assign40800_e46619;

        let (assign40810_e46631, assign40810_e46631_d_n4, assign40810_e46631_d_n6, assign40810_e46631_d_n7, assign40810_e46631_d_n8, assign40810_e46631_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1225 != 0.0)) {
        let assign40810_e46627: f64 = (locals.var_thesat1_i * locals.var_wsat1__blk976);
        let assign40810_e46628: f64 = (1.0 - assign40810_e46627);
        let assign40810_e46629: f64 = (1.0 / assign40810_e46628);
        (assign40810_e46629, (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn4)) / (assign40810_e46628 * assign40810_e46628))), (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn6)) / (assign40810_e46628 * assign40810_e46628))), (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn7)) / (assign40810_e46628 * assign40810_e46628))), (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn8)) / (assign40810_e46628 * assign40810_e46628))), (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn9)) / (assign40810_e46628 * assign40810_e46628))),)
    } else {
        (locals.var_sat_fact1__blk977, locals.var_sat_fact1__blk977_dn4, locals.var_sat_fact1__blk977_dn6, locals.var_sat_fact1__blk977_dn7, locals.var_sat_fact1__blk977_dn8, locals.var_sat_fact1__blk977_dn9,)
    }
};
        locals.var_sat_fact1__blk977 = assign40810_e46631;
        locals.var_sat_fact1__blk977_dn4 = assign40810_e46631_d_n4;
        locals.var_sat_fact1__blk977_dn6 = assign40810_e46631_d_n6;
        locals.var_sat_fact1__blk977_dn7 = assign40810_e46631_d_n7;
        locals.var_sat_fact1__blk977_dn8 = assign40810_e46631_d_n8;
        locals.var_sat_fact1__blk977_dn9 = assign40810_e46631_d_n9;

        let (assign40820_e46642, assign40820_e46642_d_n4, assign40820_e46642_d_n6, assign40820_e46642_d_n7, assign40820_e46642_d_n8, assign40820_e46642_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1225 == 0.0)) {
        let assign40820_e46639: f64 = (locals.var_thesat1_i * locals.var_wsat1__blk976);
        let assign40820_e46640: f64 = (1.0 + assign40820_e46639);
        (assign40820_e46640, (locals.var_thesat1_i * locals.var_wsat1__blk976_dn4), (locals.var_thesat1_i * locals.var_wsat1__blk976_dn6), (locals.var_thesat1_i * locals.var_wsat1__blk976_dn7), (locals.var_thesat1_i * locals.var_wsat1__blk976_dn8), (locals.var_thesat1_i * locals.var_wsat1__blk976_dn9),)
    } else {
        (locals.var_sat_fact1__blk977, locals.var_sat_fact1__blk977_dn4, locals.var_sat_fact1__blk977_dn6, locals.var_sat_fact1__blk977_dn7, locals.var_sat_fact1__blk977_dn8, locals.var_sat_fact1__blk977_dn9,)
    }
};
        locals.var_sat_fact1__blk977 = assign40820_e46642;
        locals.var_sat_fact1__blk977_dn4 = assign40820_e46642_d_n4;
        locals.var_sat_fact1__blk977_dn6 = assign40820_e46642_d_n6;
        locals.var_sat_fact1__blk977_dn7 = assign40820_e46642_d_n7;
        locals.var_sat_fact1__blk977_dn8 = assign40820_e46642_d_n8;
        locals.var_sat_fact1__blk977_dn9 = assign40820_e46642_d_n9;

        let (assign40830_e46652, assign40830_e46652_d_n4, assign40830_e46652_d_n6, assign40830_e46652_d_n7, assign40830_e46652_d_n8, assign40830_e46652_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40830_e46646: f64 = (100.0 * locals.var_esurf2__blk1028);
        let assign40830_e46649: f64 = (100.0 + locals.var_esurf2__blk1028);
        let assign40830_e46650: f64 = (assign40830_e46646 / assign40830_e46649);
        (assign40830_e46650, ((((100.0 * locals.var_esurf2__blk1028_dn4) * assign40830_e46649) - (assign40830_e46646 * locals.var_esurf2__blk1028_dn4)) / (assign40830_e46649 * assign40830_e46649)), ((((100.0 * locals.var_esurf2__blk1028_dn6) * assign40830_e46649) - (assign40830_e46646 * locals.var_esurf2__blk1028_dn6)) / (assign40830_e46649 * assign40830_e46649)), ((((100.0 * locals.var_esurf2__blk1028_dn7) * assign40830_e46649) - (assign40830_e46646 * locals.var_esurf2__blk1028_dn7)) / (assign40830_e46649 * assign40830_e46649)), ((((100.0 * locals.var_esurf2__blk1028_dn8) * assign40830_e46649) - (assign40830_e46646 * locals.var_esurf2__blk1028_dn8)) / (assign40830_e46649 * assign40830_e46649)), ((((100.0 * locals.var_esurf2__blk1028_dn9) * assign40830_e46649) - (assign40830_e46646 * locals.var_esurf2__blk1028_dn9)) / (assign40830_e46649 * assign40830_e46649)),)
    } else {
        (locals.var_wsat2__blk978, locals.var_wsat2__blk978_dn4, locals.var_wsat2__blk978_dn6, locals.var_wsat2__blk978_dn7, locals.var_wsat2__blk978_dn8, locals.var_wsat2__blk978_dn9,)
    }
};
        locals.var_wsat2__blk978 = assign40830_e46652;
        locals.var_wsat2__blk978_dn4 = assign40830_e46652_d_n4;
        locals.var_wsat2__blk978_dn6 = assign40830_e46652_d_n6;
        locals.var_wsat2__blk978_dn7 = assign40830_e46652_d_n7;
        locals.var_wsat2__blk978_dn8 = assign40830_e46652_d_n8;
        locals.var_wsat2__blk978_dn9 = assign40830_e46652_d_n9;

        let assign40840_e46655: f64 = if locals.var_thesat2_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1226 = assign40840_e46655;

        let (assign40850_e46667, assign40850_e46667_d_n4, assign40850_e46667_d_n6, assign40850_e46667_d_n7, assign40850_e46667_d_n8, assign40850_e46667_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign40850_e46663: f64 = (locals.var_thesat2_i * locals.var_wsat2__blk978);
        let assign40850_e46664: f64 = (1.0 - assign40850_e46663);
        let assign40850_e46665: f64 = (1.0 / assign40850_e46664);
        (assign40850_e46665, (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn4)) / (assign40850_e46664 * assign40850_e46664))), (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn6)) / (assign40850_e46664 * assign40850_e46664))), (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn7)) / (assign40850_e46664 * assign40850_e46664))), (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn8)) / (assign40850_e46664 * assign40850_e46664))), (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn9)) / (assign40850_e46664 * assign40850_e46664))),)
    } else {
        (locals.var_sat_fact2__blk979, locals.var_sat_fact2__blk979_dn4, locals.var_sat_fact2__blk979_dn6, locals.var_sat_fact2__blk979_dn7, locals.var_sat_fact2__blk979_dn8, locals.var_sat_fact2__blk979_dn9,)
    }
};
        locals.var_sat_fact2__blk979 = assign40850_e46667;
        locals.var_sat_fact2__blk979_dn4 = assign40850_e46667_d_n4;
        locals.var_sat_fact2__blk979_dn6 = assign40850_e46667_d_n6;
        locals.var_sat_fact2__blk979_dn7 = assign40850_e46667_d_n7;
        locals.var_sat_fact2__blk979_dn8 = assign40850_e46667_d_n8;
        locals.var_sat_fact2__blk979_dn9 = assign40850_e46667_d_n9;

        let (assign40860_e46678, assign40860_e46678_d_n4, assign40860_e46678_d_n6, assign40860_e46678_d_n7, assign40860_e46678_d_n8, assign40860_e46678_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1226 == 0.0)) {
        let assign40860_e46675: f64 = (locals.var_thesat2_i * locals.var_wsat2__blk978);
        let assign40860_e46676: f64 = (1.0 + assign40860_e46675);
        (assign40860_e46676, (locals.var_thesat2_i * locals.var_wsat2__blk978_dn4), (locals.var_thesat2_i * locals.var_wsat2__blk978_dn6), (locals.var_thesat2_i * locals.var_wsat2__blk978_dn7), (locals.var_thesat2_i * locals.var_wsat2__blk978_dn8), (locals.var_thesat2_i * locals.var_wsat2__blk978_dn9),)
    } else {
        (locals.var_sat_fact2__blk979, locals.var_sat_fact2__blk979_dn4, locals.var_sat_fact2__blk979_dn6, locals.var_sat_fact2__blk979_dn7, locals.var_sat_fact2__blk979_dn8, locals.var_sat_fact2__blk979_dn9,)
    }
};
        locals.var_sat_fact2__blk979 = assign40860_e46678;
        locals.var_sat_fact2__blk979_dn4 = assign40860_e46678_d_n4;
        locals.var_sat_fact2__blk979_dn6 = assign40860_e46678_d_n6;
        locals.var_sat_fact2__blk979_dn7 = assign40860_e46678_d_n7;
        locals.var_sat_fact2__blk979_dn8 = assign40860_e46678_d_n8;
        locals.var_sat_fact2__blk979_dn9 = assign40860_e46678_d_n9;

        let (assign40870_e46690, assign40870_e46690_d_n4, assign40870_e46690_d_n6, assign40870_e46690_d_n7, assign40870_e46690_d_n8, assign40870_e46690_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40870_e46682: f64 = (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017);
        let assign40870_e46684: f64 = (assign40870_e46682 * 0.5);
        let assign40870_e46687: f64 = (locals.var_sat_fact1__blk977 + locals.var_sat_fact2__blk979);
        let assign40870_e46688: f64 = (assign40870_e46684 * assign40870_e46687);
        (assign40870_e46688, (((((locals.var_sat_phit_loc__blk896_dn4 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn4)) * 0.5) * assign40870_e46687) + (assign40870_e46684 * (locals.var_sat_fact1__blk977_dn4 + locals.var_sat_fact2__blk979_dn4))), (((((locals.var_sat_phit_loc__blk896_dn6 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn6)) * 0.5) * assign40870_e46687) + (assign40870_e46684 * (locals.var_sat_fact1__blk977_dn6 + locals.var_sat_fact2__blk979_dn6))), (((((locals.var_sat_phit_loc__blk896_dn7 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn7)) * 0.5) * assign40870_e46687) + (assign40870_e46684 * (locals.var_sat_fact1__blk977_dn7 + locals.var_sat_fact2__blk979_dn7))), (((((locals.var_sat_phit_loc__blk896_dn8 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn8)) * 0.5) * assign40870_e46687) + (assign40870_e46684 * (locals.var_sat_fact1__blk977_dn8 + locals.var_sat_fact2__blk979_dn8))), (((((locals.var_sat_phit_loc__blk896_dn9 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn9)) * 0.5) * assign40870_e46687) + (assign40870_e46684 * (locals.var_sat_fact1__blk977_dn9 + locals.var_sat_fact2__blk979_dn9))),)
    } else {
        (locals.var_ggamma__blk1049, locals.var_ggamma__blk1049_dn4, locals.var_ggamma__blk1049_dn6, locals.var_ggamma__blk1049_dn7, locals.var_ggamma__blk1049_dn8, locals.var_ggamma__blk1049_dn9,)
    }
};
        locals.var_ggamma__blk1049 = assign40870_e46690;
        locals.var_ggamma__blk1049_dn4 = assign40870_e46690_d_n4;
        locals.var_ggamma__blk1049_dn6 = assign40870_e46690_d_n6;
        locals.var_ggamma__blk1049_dn7 = assign40870_e46690_d_n7;
        locals.var_ggamma__blk1049_dn8 = assign40870_e46690_d_n8;
        locals.var_ggamma__blk1049_dn9 = assign40870_e46690_d_n9;

        let (assign40880_e46698, assign40880_e46698_d_n4, assign40880_e46698_d_n6, assign40880_e46698_d_n7, assign40880_e46698_d_n8, assign40880_e46698_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40880_e46695: f64 = (locals.var_gmob__blk1043 * locals.var_gdl__blk1048);
        let assign40880_e46696: f64 = (locals.var_ggamma__blk1049 / assign40880_e46695);
        (assign40880_e46696, (((locals.var_ggamma__blk1049_dn4 * assign40880_e46695) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn4 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn4)))) / (assign40880_e46695 * assign40880_e46695)), (((locals.var_ggamma__blk1049_dn6 * assign40880_e46695) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn6 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn6)))) / (assign40880_e46695 * assign40880_e46695)), (((locals.var_ggamma__blk1049_dn7 * assign40880_e46695) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn7 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn7)))) / (assign40880_e46695 * assign40880_e46695)), (((locals.var_ggamma__blk1049_dn8 * assign40880_e46695) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn8 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn8)))) / (assign40880_e46695 * assign40880_e46695)), (((locals.var_ggamma__blk1049_dn9 * assign40880_e46695) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn9 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn9)))) / (assign40880_e46695 * assign40880_e46695)),)
    } else {
        (locals.var_sqrt_zsat__blk1050, locals.var_sqrt_zsat__blk1050_dn4, locals.var_sqrt_zsat__blk1050_dn6, locals.var_sqrt_zsat__blk1050_dn7, locals.var_sqrt_zsat__blk1050_dn8, locals.var_sqrt_zsat__blk1050_dn9,)
    }
};
        locals.var_sqrt_zsat__blk1050 = assign40880_e46698;
        locals.var_sqrt_zsat__blk1050_dn4 = assign40880_e46698_d_n4;
        locals.var_sqrt_zsat__blk1050_dn6 = assign40880_e46698_d_n6;
        locals.var_sqrt_zsat__blk1050_dn7 = assign40880_e46698_d_n7;
        locals.var_sqrt_zsat__blk1050_dn8 = assign40880_e46698_d_n8;
        locals.var_sqrt_zsat__blk1050_dn9 = assign40880_e46698_d_n9;

        let (assign40890_e46704, assign40890_e46704_d_n4, assign40890_e46704_d_n6, assign40890_e46704_d_n7, assign40890_e46704_d_n8, assign40890_e46704_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40890_e46702: f64 = (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050);
        (assign40890_e46702, ((locals.var_sqrt_zsat__blk1050_dn4 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn4)), ((locals.var_sqrt_zsat__blk1050_dn6 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn6)), ((locals.var_sqrt_zsat__blk1050_dn7 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn7)), ((locals.var_sqrt_zsat__blk1050_dn8 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn8)), ((locals.var_sqrt_zsat__blk1050_dn9 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn9)),)
    } else {
        (locals.var_zsat__blk1051, locals.var_zsat__blk1051_dn4, locals.var_zsat__blk1051_dn6, locals.var_zsat__blk1051_dn7, locals.var_zsat__blk1051_dn8, locals.var_zsat__blk1051_dn9,)
    }
};
        locals.var_zsat__blk1051 = assign40890_e46704;
        locals.var_zsat__blk1051_dn4 = assign40890_e46704_d_n4;
        locals.var_zsat__blk1051_dn6 = assign40890_e46704_d_n6;
        locals.var_zsat__blk1051_dn7 = assign40890_e46704_d_n7;
        locals.var_zsat__blk1051_dn8 = assign40890_e46704_d_n8;
        locals.var_zsat__blk1051_dn9 = assign40890_e46704_d_n9;

        let (assign40900_e46711, assign40900_e46711_d_n4, assign40900_e46711_d_n6, assign40900_e46711_d_n7, assign40900_e46711_d_n8, assign40900_e46711_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40900_e46708: f64 = (1.0 + locals.var_zsat__blk1051);
        let assign40900_e46709: f64 = (assign40900_e46708).sqrt();
        (assign40900_e46709, (locals.var_zsat__blk1051_dn4 / (2.0 * assign40900_e46709)), (locals.var_zsat__blk1051_dn6 / (2.0 * assign40900_e46709)), (locals.var_zsat__blk1051_dn7 / (2.0 * assign40900_e46709)), (locals.var_zsat__blk1051_dn8 / (2.0 * assign40900_e46709)), (locals.var_zsat__blk1051_dn9 / (2.0 * assign40900_e46709)),)
    } else {
        (locals.var_vsat_fact__blk1052, locals.var_vsat_fact__blk1052_dn4, locals.var_vsat_fact__blk1052_dn6, locals.var_vsat_fact__blk1052_dn7, locals.var_vsat_fact__blk1052_dn8, locals.var_vsat_fact__blk1052_dn9,)
    }
};
        locals.var_vsat_fact__blk1052 = assign40900_e46711;
        locals.var_vsat_fact__blk1052_dn4 = assign40900_e46711_d_n4;
        locals.var_vsat_fact__blk1052_dn6 = assign40900_e46711_d_n6;
        locals.var_vsat_fact__blk1052_dn7 = assign40900_e46711_d_n7;
        locals.var_vsat_fact__blk1052_dn8 = assign40900_e46711_d_n8;
        locals.var_vsat_fact__blk1052_dn9 = assign40900_e46711_d_n9;

        let (assign40910_e46721, assign40910_e46721_d_n4, assign40910_e46721_d_n6, assign40910_e46721_d_n7, assign40910_e46721_d_n8, assign40910_e46721_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40910_e46716: f64 = (1.5 * locals.var_zsat__blk1051);
        let assign40910_e46717: f64 = (1.0 + assign40910_e46716);
        let assign40910_e46719: f64 = (assign40910_e46717 / locals.var_vsat_fact__blk1052);
        (assign40910_e46719, ((((1.5 * locals.var_zsat__blk1051_dn4) * locals.var_vsat_fact__blk1052) - (assign40910_e46717 * locals.var_vsat_fact__blk1052_dn4)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), ((((1.5 * locals.var_zsat__blk1051_dn6) * locals.var_vsat_fact__blk1052) - (assign40910_e46717 * locals.var_vsat_fact__blk1052_dn6)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), ((((1.5 * locals.var_zsat__blk1051_dn7) * locals.var_vsat_fact__blk1052) - (assign40910_e46717 * locals.var_vsat_fact__blk1052_dn7)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), ((((1.5 * locals.var_zsat__blk1051_dn8) * locals.var_vsat_fact__blk1052) - (assign40910_e46717 * locals.var_vsat_fact__blk1052_dn8)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), ((((1.5 * locals.var_zsat__blk1051_dn9) * locals.var_vsat_fact__blk1052) - (assign40910_e46717 * locals.var_vsat_fact__blk1052_dn9)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)),)
    } else {
        (locals.var_hsat__blk1053, locals.var_hsat__blk1053_dn4, locals.var_hsat__blk1053_dn6, locals.var_hsat__blk1053_dn7, locals.var_hsat__blk1053_dn8, locals.var_hsat__blk1053_dn9,)
    }
};
        locals.var_hsat__blk1053 = assign40910_e46721;
        locals.var_hsat__blk1053_dn4 = assign40910_e46721_d_n4;
        locals.var_hsat__blk1053_dn6 = assign40910_e46721_d_n6;
        locals.var_hsat__blk1053_dn7 = assign40910_e46721_d_n7;
        locals.var_hsat__blk1053_dn8 = assign40910_e46721_d_n8;
        locals.var_hsat__blk1053_dn9 = assign40910_e46721_d_n9;

        let assign40920_e46724: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1227 = assign40920_e46724;

        let (assign40930_e46743, assign40930_e46743_d_n4, assign40930_e46743_d_n6, assign40930_e46743_d_n7, assign40930_e46743_d_n8, assign40930_e46743_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 != 0.0)) {
        let assign40930_e46730: f64 = (0.6 * locals.var_qq);
        let assign40930_e46732: f64 = (-0.1666666666667);
        let assign40930_e46735: f64 = (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027);
        let assign40930_e46737: f64 = (assign40930_e46735 + 60.0);
        let assign40930_e46738: f64 = (assign40930_e46737).ln();
        let assign40930_e46739: f64 = (assign40930_e46732 * assign40930_e46738);
        let assign40930_e46740: f64 = (assign40930_e46739).exp();
        let assign40930_e46741: f64 = (assign40930_e46730 * assign40930_e46740);
        (assign40930_e46741, (((0.6 * locals.var_qq_dn4) * assign40930_e46740) + (assign40930_e46730 * (assign40930_e46740 * (assign40930_e46732 * (((locals.var_esurf1__blk1027_dn4 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn4)) / assign40930_e46737))))), (((0.6 * locals.var_qq_dn6) * assign40930_e46740) + (assign40930_e46730 * (assign40930_e46740 * (assign40930_e46732 * (((locals.var_esurf1__blk1027_dn6 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn6)) / assign40930_e46737))))), (((0.6 * locals.var_qq_dn7) * assign40930_e46740) + (assign40930_e46730 * (assign40930_e46740 * (assign40930_e46732 * (((locals.var_esurf1__blk1027_dn7 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn7)) / assign40930_e46737))))), (((0.6 * locals.var_qq_dn8) * assign40930_e46740) + (assign40930_e46730 * (assign40930_e46740 * (assign40930_e46732 * (((locals.var_esurf1__blk1027_dn8 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn8)) / assign40930_e46737))))), (((0.6 * locals.var_qq_dn9) * assign40930_e46740) + (assign40930_e46730 * (assign40930_e46740 * (assign40930_e46732 * (((locals.var_esurf1__blk1027_dn9 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn9)) / assign40930_e46737))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40930_e46743;
        locals.var_temp1_dn4 = assign40930_e46743_d_n4;
        locals.var_temp1_dn6 = assign40930_e46743_d_n6;
        locals.var_temp1_dn7 = assign40930_e46743_d_n7;
        locals.var_temp1_dn8 = assign40930_e46743_d_n8;
        locals.var_temp1_dn9 = assign40930_e46743_d_n9;

        let (assign40940_e46762, assign40940_e46762_d_n4, assign40940_e46762_d_n6, assign40940_e46762_d_n7, assign40940_e46762_d_n8, assign40940_e46762_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 != 0.0)) {
        let assign40940_e46749: f64 = (0.6 * locals.var_qq);
        let assign40940_e46751: f64 = (-0.1666666666667);
        let assign40940_e46754: f64 = (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028);
        let assign40940_e46756: f64 = (assign40940_e46754 + 60.0);
        let assign40940_e46757: f64 = (assign40940_e46756).ln();
        let assign40940_e46758: f64 = (assign40940_e46751 * assign40940_e46757);
        let assign40940_e46759: f64 = (assign40940_e46758).exp();
        let assign40940_e46760: f64 = (assign40940_e46749 * assign40940_e46759);
        (assign40940_e46760, (((0.6 * locals.var_qq_dn4) * assign40940_e46759) + (assign40940_e46749 * (assign40940_e46759 * (assign40940_e46751 * (((locals.var_esurf2__blk1028_dn4 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn4)) / assign40940_e46756))))), (((0.6 * locals.var_qq_dn6) * assign40940_e46759) + (assign40940_e46749 * (assign40940_e46759 * (assign40940_e46751 * (((locals.var_esurf2__blk1028_dn6 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn6)) / assign40940_e46756))))), (((0.6 * locals.var_qq_dn7) * assign40940_e46759) + (assign40940_e46749 * (assign40940_e46759 * (assign40940_e46751 * (((locals.var_esurf2__blk1028_dn7 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn7)) / assign40940_e46756))))), (((0.6 * locals.var_qq_dn8) * assign40940_e46759) + (assign40940_e46749 * (assign40940_e46759 * (assign40940_e46751 * (((locals.var_esurf2__blk1028_dn8 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn8)) / assign40940_e46756))))), (((0.6 * locals.var_qq_dn9) * assign40940_e46759) + (assign40940_e46749 * (assign40940_e46759 * (assign40940_e46751 * (((locals.var_esurf2__blk1028_dn9 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn9)) / assign40940_e46756))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign40940_e46762;
        locals.var_temp2_dn4 = assign40940_e46762_d_n4;
        locals.var_temp2_dn6 = assign40940_e46762_d_n6;
        locals.var_temp2_dn7 = assign40940_e46762_d_n7;
        locals.var_temp2_dn8 = assign40940_e46762_d_n8;
        locals.var_temp2_dn9 = assign40940_e46762_d_n9;

        let (assign40950_e46774, assign40950_e46774_d_n4, assign40950_e46774_d_n6, assign40950_e46774_d_n7, assign40950_e46774_d_n8, assign40950_e46774_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 != 0.0)) {
        let assign40950_e46769: f64 = (locals.var_k1__blk932 * locals.var_temp1);
        let assign40950_e46770: f64 = (1.0 + assign40950_e46769);
        let assign40950_e46772: f64 = (assign40950_e46770 / locals.var_tox1fact__blk913);
        (assign40950_e46772, (((((locals.var_k1__blk932_dn4 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn4)) * locals.var_tox1fact__blk913) - (assign40950_e46770 * locals.var_tox1fact__blk913_dn4)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), (((((locals.var_k1__blk932_dn6 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn6)) * locals.var_tox1fact__blk913) - (assign40950_e46770 * locals.var_tox1fact__blk913_dn6)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), (((((locals.var_k1__blk932_dn7 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn7)) * locals.var_tox1fact__blk913) - (assign40950_e46770 * locals.var_tox1fact__blk913_dn7)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), (((((locals.var_k1__blk932_dn8 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn8)) * locals.var_tox1fact__blk913) - (assign40950_e46770 * locals.var_tox1fact__blk913_dn8)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), (((((locals.var_k1__blk932_dn9 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn9)) * locals.var_tox1fact__blk913) - (assign40950_e46770 * locals.var_tox1fact__blk913_dn9)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)),)
    } else {
        (locals.var_qmfact1__blk1054, locals.var_qmfact1__blk1054_dn4, locals.var_qmfact1__blk1054_dn6, locals.var_qmfact1__blk1054_dn7, locals.var_qmfact1__blk1054_dn8, locals.var_qmfact1__blk1054_dn9,)
    }
};
        locals.var_qmfact1__blk1054 = assign40950_e46774;
        locals.var_qmfact1__blk1054_dn4 = assign40950_e46774_d_n4;
        locals.var_qmfact1__blk1054_dn6 = assign40950_e46774_d_n6;
        locals.var_qmfact1__blk1054_dn7 = assign40950_e46774_d_n7;
        locals.var_qmfact1__blk1054_dn8 = assign40950_e46774_d_n8;
        locals.var_qmfact1__blk1054_dn9 = assign40950_e46774_d_n9;

        let (assign40960_e46786, assign40960_e46786_d_n4, assign40960_e46786_d_n6, assign40960_e46786_d_n7, assign40960_e46786_d_n8, assign40960_e46786_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 != 0.0)) {
        let assign40960_e46781: f64 = (locals.var_k2__blk933 * locals.var_temp2);
        let assign40960_e46782: f64 = (1.0 + assign40960_e46781);
        let assign40960_e46784: f64 = (assign40960_e46782 / locals.var_tox2fact__blk914);
        (assign40960_e46784, (((((locals.var_k2__blk933_dn4 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn4)) * locals.var_tox2fact__blk914) - (assign40960_e46782 * locals.var_tox2fact__blk914_dn4)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), (((((locals.var_k2__blk933_dn6 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn6)) * locals.var_tox2fact__blk914) - (assign40960_e46782 * locals.var_tox2fact__blk914_dn6)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), (((((locals.var_k2__blk933_dn7 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn7)) * locals.var_tox2fact__blk914) - (assign40960_e46782 * locals.var_tox2fact__blk914_dn7)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), (((((locals.var_k2__blk933_dn8 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn8)) * locals.var_tox2fact__blk914) - (assign40960_e46782 * locals.var_tox2fact__blk914_dn8)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), (((((locals.var_k2__blk933_dn9 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn9)) * locals.var_tox2fact__blk914) - (assign40960_e46782 * locals.var_tox2fact__blk914_dn9)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)),)
    } else {
        (locals.var_qmfact2__blk1055, locals.var_qmfact2__blk1055_dn4, locals.var_qmfact2__blk1055_dn6, locals.var_qmfact2__blk1055_dn7, locals.var_qmfact2__blk1055_dn8, locals.var_qmfact2__blk1055_dn9,)
    }
};
        locals.var_qmfact2__blk1055 = assign40960_e46786;
        locals.var_qmfact2__blk1055_dn4 = assign40960_e46786_d_n4;
        locals.var_qmfact2__blk1055_dn6 = assign40960_e46786_d_n6;
        locals.var_qmfact2__blk1055_dn7 = assign40960_e46786_d_n7;
        locals.var_qmfact2__blk1055_dn8 = assign40960_e46786_d_n8;
        locals.var_qmfact2__blk1055_dn9 = assign40960_e46786_d_n9;

        let (assign40970_e46793, assign40970_e46793_d_n4, assign40970_e46793_d_n6, assign40970_e46793_d_n7, assign40970_e46793_d_n8, assign40970_e46793_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qmfact1__blk1054, locals.var_qmfact1__blk1054_dn4, locals.var_qmfact1__blk1054_dn6, locals.var_qmfact1__blk1054_dn7, locals.var_qmfact1__blk1054_dn8, locals.var_qmfact1__blk1054_dn9,)
    }
};
        locals.var_qmfact1__blk1054 = assign40970_e46793;
        locals.var_qmfact1__blk1054_dn4 = assign40970_e46793_d_n4;
        locals.var_qmfact1__blk1054_dn6 = assign40970_e46793_d_n6;
        locals.var_qmfact1__blk1054_dn7 = assign40970_e46793_d_n7;
        locals.var_qmfact1__blk1054_dn8 = assign40970_e46793_d_n8;
        locals.var_qmfact1__blk1054_dn9 = assign40970_e46793_d_n9;

        let (assign40980_e46800, assign40980_e46800_d_n4, assign40980_e46800_d_n6, assign40980_e46800_d_n7, assign40980_e46800_d_n8, assign40980_e46800_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qmfact2__blk1055, locals.var_qmfact2__blk1055_dn4, locals.var_qmfact2__blk1055_dn6, locals.var_qmfact2__blk1055_dn7, locals.var_qmfact2__blk1055_dn8, locals.var_qmfact2__blk1055_dn9,)
    }
};
        locals.var_qmfact2__blk1055 = assign40980_e46800;
        locals.var_qmfact2__blk1055_dn4 = assign40980_e46800_d_n4;
        locals.var_qmfact2__blk1055_dn6 = assign40980_e46800_d_n6;
        locals.var_qmfact2__blk1055_dn7 = assign40980_e46800_d_n7;
        locals.var_qmfact2__blk1055_dn8 = assign40980_e46800_d_n8;
        locals.var_qmfact2__blk1055_dn9 = assign40980_e46800_d_n9;

        let assign40990_e46803: f64 = if locals.var_qis__blk938 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1228 = assign40990_e46803;

        let assign41000_e46806: f64 = if locals.var_qid__blk1003 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1229 = assign41000_e46806;

        let assign41010_e46808: f64 = (locals.var_a2d__blk1012).abs();
        let assign41010_e46810: f64 = if assign41010_e46808 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1230 = assign41010_e46810;

        let (assign41020_e46832, assign41020_e46832_d_n4, assign41020_e46832_d_n6, assign41020_e46832_d_n7, assign41020_e46832_d_n8, assign41020_e46832_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
        let assign41020_e46820: f64 = (2.0 + locals.var_q1d__blk1001);
        let assign41020_e46823: f64 = (0.5 * locals.var_a1d__blk1011);
        let assign41020_e46824: f64 = (assign41020_e46820 + assign41020_e46823);
        let assign41020_e46827: f64 = (2.0 + locals.var_q2d__blk1002);
        let assign41020_e46829: f64 = (assign41020_e46827 * locals.var_a1d__blk1011);
        let assign41020_e46830: f64 = (assign41020_e46824 / assign41020_e46829);
        (assign41020_e46830, ((((locals.var_q1d__blk1001_dn4 + (0.5 * locals.var_a1d__blk1011_dn4)) * assign41020_e46829) - (assign41020_e46824 * ((locals.var_q2d__blk1002_dn4 * locals.var_a1d__blk1011) + (assign41020_e46827 * locals.var_a1d__blk1011_dn4)))) / (assign41020_e46829 * assign41020_e46829)), ((((locals.var_q1d__blk1001_dn6 + (0.5 * locals.var_a1d__blk1011_dn6)) * assign41020_e46829) - (assign41020_e46824 * ((locals.var_q2d__blk1002_dn6 * locals.var_a1d__blk1011) + (assign41020_e46827 * locals.var_a1d__blk1011_dn6)))) / (assign41020_e46829 * assign41020_e46829)), ((((locals.var_q1d__blk1001_dn7 + (0.5 * locals.var_a1d__blk1011_dn7)) * assign41020_e46829) - (assign41020_e46824 * ((locals.var_q2d__blk1002_dn7 * locals.var_a1d__blk1011) + (assign41020_e46827 * locals.var_a1d__blk1011_dn7)))) / (assign41020_e46829 * assign41020_e46829)), ((((locals.var_q1d__blk1001_dn8 + (0.5 * locals.var_a1d__blk1011_dn8)) * assign41020_e46829) - (assign41020_e46824 * ((locals.var_q2d__blk1002_dn8 * locals.var_a1d__blk1011) + (assign41020_e46827 * locals.var_a1d__blk1011_dn8)))) / (assign41020_e46829 * assign41020_e46829)), ((((locals.var_q1d__blk1001_dn9 + (0.5 * locals.var_a1d__blk1011_dn9)) * assign41020_e46829) - (assign41020_e46824 * ((locals.var_q2d__blk1002_dn9 * locals.var_a1d__blk1011) + (assign41020_e46827 * locals.var_a1d__blk1011_dn9)))) / (assign41020_e46829 * assign41020_e46829)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign41020_e46832;
        locals.var_temp_dn4 = assign41020_e46832_d_n4;
        locals.var_temp_dn6 = assign41020_e46832_d_n6;
        locals.var_temp_dn7 = assign41020_e46832_d_n7;
        locals.var_temp_dn8 = assign41020_e46832_d_n8;
        locals.var_temp_dn9 = assign41020_e46832_d_n9;

        let (assign41030_e46844, assign41030_e46844_d_n4, assign41030_e46844_d_n6, assign41030_e46844_d_n7, assign41030_e46844_d_n8, assign41030_e46844_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
        let assign41030_e46842: f64 = (locals.var_temp * locals.var_a2d__blk1012);
        (assign41030_e46842, ((locals.var_temp_dn4 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn4)), ((locals.var_temp_dn6 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn6)), ((locals.var_temp_dn7 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn7)), ((locals.var_temp_dn8 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn8)), ((locals.var_temp_dn9 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign41030_e46844;
        locals.var_temp1_dn4 = assign41030_e46844_d_n4;
        locals.var_temp1_dn6 = assign41030_e46844_d_n6;
        locals.var_temp1_dn7 = assign41030_e46844_d_n7;
        locals.var_temp1_dn8 = assign41030_e46844_d_n8;
        locals.var_temp1_dn9 = assign41030_e46844_d_n9;

    }

    pub(super) fn stamp_transient_block_114(
        locals: &mut StampLocals,
    ) {
        let (assign41040_e46856, assign41040_e46856_d_n4, assign41040_e46856_d_n6, assign41040_e46856_d_n7, assign41040_e46856_d_n8, assign41040_e46856_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
        let assign41040_e46854: f64 = (locals.var_temp1 * locals.var_temp1);
        (assign41040_e46854, ((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4)), ((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)), ((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)), ((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)), ((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign41040_e46856;
        locals.var_temp2_dn4 = assign41040_e46856_d_n4;
        locals.var_temp2_dn6 = assign41040_e46856_d_n6;
        locals.var_temp2_dn7 = assign41040_e46856_d_n7;
        locals.var_temp2_dn8 = assign41040_e46856_d_n8;
        locals.var_temp2_dn9 = assign41040_e46856_d_n9;

        let (assign41050_e46870, assign41050_e46870_d_n4, assign41050_e46870_d_n6, assign41050_e46870_d_n7, assign41050_e46870_d_n8, assign41050_e46870_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
        let assign41050_e46866: f64 = (1.0 - locals.var_temp1);
        let assign41050_e46868: f64 = (assign41050_e46866 + locals.var_temp2);
        (assign41050_e46868, ((-locals.var_temp1_dn4) + locals.var_temp2_dn4), ((-locals.var_temp1_dn6) + locals.var_temp2_dn6), ((-locals.var_temp1_dn7) + locals.var_temp2_dn7), ((-locals.var_temp1_dn8) + locals.var_temp2_dn8), ((-locals.var_temp1_dn9) + locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign41050_e46870;
        locals.var_temp3_dn4 = assign41050_e46870_d_n4;
        locals.var_temp3_dn6 = assign41050_e46870_d_n6;
        locals.var_temp3_dn7 = assign41050_e46870_d_n7;
        locals.var_temp3_dn8 = assign41050_e46870_d_n8;
        locals.var_temp3_dn9 = assign41050_e46870_d_n9;

        let (assign41060_e46884, assign41060_e46884_d_n4, assign41060_e46884_d_n6, assign41060_e46884_d_n7, assign41060_e46884_d_n8, assign41060_e46884_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
        let assign41060_e46881: f64 = (locals.var_temp1 * locals.var_temp2);
        let assign41060_e46882: f64 = (locals.var_temp3 - assign41060_e46881);
        (assign41060_e46882, (locals.var_temp3_dn4 - ((locals.var_temp1_dn4 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn4))), (locals.var_temp3_dn6 - ((locals.var_temp1_dn6 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn6))), (locals.var_temp3_dn7 - ((locals.var_temp1_dn7 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn7))), (locals.var_temp3_dn8 - ((locals.var_temp1_dn8 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn8))), (locals.var_temp3_dn9 - ((locals.var_temp1_dn9 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn9))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign41060_e46884;
        locals.var_temp4_dn4 = assign41060_e46884_d_n4;
        locals.var_temp4_dn6 = assign41060_e46884_d_n6;
        locals.var_temp4_dn7 = assign41060_e46884_d_n7;
        locals.var_temp4_dn8 = assign41060_e46884_d_n8;
        locals.var_temp4_dn9 = assign41060_e46884_d_n9;

        let (assign41070_e46910, assign41070_e46910_d_n4, assign41070_e46910_d_n6, assign41070_e46910_d_n7, assign41070_e46910_d_n8, assign41070_e46910_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
        let assign41070_e46895: f64 = (2.0 * locals.var_qsqd__blk1006);
        let assign41070_e46899: f64 = (1.0 / locals.var_a1d__blk1011);
        let assign41070_e46900: f64 = (locals.var_temp - assign41070_e46899);
        let assign41070_e46901: f64 = (assign41070_e46895 * assign41070_e46900);
        let assign41070_e46903: f64 = (assign41070_e46901 * locals.var_temp4);
        let assign41070_e46904: f64 = (locals.var_k2q2d__blk1005 - assign41070_e46903);
        let assign41070_e46907: f64 = (2.0 + locals.var_q2d__blk1002);
        let assign41070_e46908: f64 = (assign41070_e46904 / assign41070_e46907);
        (assign41070_e46908, ((((locals.var_k2q2d__blk1005_dn4 - (((((2.0 * locals.var_qsqd__blk1006_dn4) * assign41070_e46900) + (assign41070_e46895 * (locals.var_temp_dn4 - (-(locals.var_a1d__blk1011_dn4 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign41070_e46901 * locals.var_temp4_dn4))) * assign41070_e46907) - (assign41070_e46904 * locals.var_q2d__blk1002_dn4)) / (assign41070_e46907 * assign41070_e46907)), ((((locals.var_k2q2d__blk1005_dn6 - (((((2.0 * locals.var_qsqd__blk1006_dn6) * assign41070_e46900) + (assign41070_e46895 * (locals.var_temp_dn6 - (-(locals.var_a1d__blk1011_dn6 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign41070_e46901 * locals.var_temp4_dn6))) * assign41070_e46907) - (assign41070_e46904 * locals.var_q2d__blk1002_dn6)) / (assign41070_e46907 * assign41070_e46907)), ((((locals.var_k2q2d__blk1005_dn7 - (((((2.0 * locals.var_qsqd__blk1006_dn7) * assign41070_e46900) + (assign41070_e46895 * (locals.var_temp_dn7 - (-(locals.var_a1d__blk1011_dn7 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign41070_e46901 * locals.var_temp4_dn7))) * assign41070_e46907) - (assign41070_e46904 * locals.var_q2d__blk1002_dn7)) / (assign41070_e46907 * assign41070_e46907)), ((((locals.var_k2q2d__blk1005_dn8 - (((((2.0 * locals.var_qsqd__blk1006_dn8) * assign41070_e46900) + (assign41070_e46895 * (locals.var_temp_dn8 - (-(locals.var_a1d__blk1011_dn8 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign41070_e46901 * locals.var_temp4_dn8))) * assign41070_e46907) - (assign41070_e46904 * locals.var_q2d__blk1002_dn8)) / (assign41070_e46907 * assign41070_e46907)), ((((locals.var_k2q2d__blk1005_dn9 - (((((2.0 * locals.var_qsqd__blk1006_dn9) * assign41070_e46900) + (assign41070_e46895 * (locals.var_temp_dn9 - (-(locals.var_a1d__blk1011_dn9 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign41070_e46901 * locals.var_temp4_dn9))) * assign41070_e46907) - (assign41070_e46904 * locals.var_q2d__blk1002_dn9)) / (assign41070_e46907 * assign41070_e46907)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign41070_e46910;
        locals.var_temp1_dn4 = assign41070_e46910_d_n4;
        locals.var_temp1_dn6 = assign41070_e46910_d_n6;
        locals.var_temp1_dn7 = assign41070_e46910_d_n7;
        locals.var_temp1_dn8 = assign41070_e46910_d_n8;
        locals.var_temp1_dn9 = assign41070_e46910_d_n9;

        let (assign41080_e46930, assign41080_e46930_d_n4, assign41080_e46930_d_n6, assign41080_e46930_d_n7, assign41080_e46930_d_n8, assign41080_e46930_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
        let assign41080_e46920: f64 = (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003);
        let assign41080_e46922: f64 = (assign41080_e46920 - locals.var_aexp1d__blk1007);
        let assign41080_e46924: f64 = (assign41080_e46922 / locals.var_a1d__blk1011);
        let assign41080_e46926: f64 = (assign41080_e46924 - locals.var_temp1);
        let assign41080_e46928: f64 = (assign41080_e46926 / locals.var_qid__blk1003);
        (assign41080_e46928, ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn4 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn4)) - locals.var_aexp1d__blk1007_dn4) * locals.var_a1d__blk1011) - (assign41080_e46922 * locals.var_a1d__blk1011_dn4)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn4) * locals.var_qid__blk1003) - (assign41080_e46926 * locals.var_qid__blk1003_dn4)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn6 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn6)) - locals.var_aexp1d__blk1007_dn6) * locals.var_a1d__blk1011) - (assign41080_e46922 * locals.var_a1d__blk1011_dn6)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn6) * locals.var_qid__blk1003) - (assign41080_e46926 * locals.var_qid__blk1003_dn6)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn7 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn7)) - locals.var_aexp1d__blk1007_dn7) * locals.var_a1d__blk1011) - (assign41080_e46922 * locals.var_a1d__blk1011_dn7)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn7) * locals.var_qid__blk1003) - (assign41080_e46926 * locals.var_qid__blk1003_dn7)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn8 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn8)) - locals.var_aexp1d__blk1007_dn8) * locals.var_a1d__blk1011) - (assign41080_e46922 * locals.var_a1d__blk1011_dn8)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn8) * locals.var_qid__blk1003) - (assign41080_e46926 * locals.var_qid__blk1003_dn8)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn9 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn9)) - locals.var_aexp1d__blk1007_dn9) * locals.var_a1d__blk1011) - (assign41080_e46922 * locals.var_a1d__blk1011_dn9)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn9) * locals.var_qid__blk1003) - (assign41080_e46926 * locals.var_qid__blk1003_dn9)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)),)
    } else {
        (locals.var_dqid_dxn_qi__blk1056, locals.var_dqid_dxn_qi__blk1056_dn4, locals.var_dqid_dxn_qi__blk1056_dn6, locals.var_dqid_dxn_qi__blk1056_dn7, locals.var_dqid_dxn_qi__blk1056_dn8, locals.var_dqid_dxn_qi__blk1056_dn9,)
    }
};
        locals.var_dqid_dxn_qi__blk1056 = assign41080_e46930;
        locals.var_dqid_dxn_qi__blk1056_dn4 = assign41080_e46930_d_n4;
        locals.var_dqid_dxn_qi__blk1056_dn6 = assign41080_e46930_d_n6;
        locals.var_dqid_dxn_qi__blk1056_dn7 = assign41080_e46930_d_n7;
        locals.var_dqid_dxn_qi__blk1056_dn8 = assign41080_e46930_d_n8;
        locals.var_dqid_dxn_qi__blk1056_dn9 = assign41080_e46930_d_n9;

        let (assign41090_e46946, assign41090_e46946_d_n4, assign41090_e46946_d_n6, assign41090_e46946_d_n7, assign41090_e46946_d_n8, assign41090_e46946_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
        let assign41090_e46940: f64 = (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003);
        let assign41090_e46943: f64 = (locals.var_dqid_dxn_qi__blk1056 + 1.0);
        let assign41090_e46944: f64 = (assign41090_e46940 / assign41090_e46943);
        (assign41090_e46944, (((((locals.var_dqid_dxn_qi__blk1056_dn4 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn4)) * assign41090_e46943) - (assign41090_e46940 * locals.var_dqid_dxn_qi__blk1056_dn4)) / (assign41090_e46943 * assign41090_e46943)), (((((locals.var_dqid_dxn_qi__blk1056_dn6 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn6)) * assign41090_e46943) - (assign41090_e46940 * locals.var_dqid_dxn_qi__blk1056_dn6)) / (assign41090_e46943 * assign41090_e46943)), (((((locals.var_dqid_dxn_qi__blk1056_dn7 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn7)) * assign41090_e46943) - (assign41090_e46940 * locals.var_dqid_dxn_qi__blk1056_dn7)) / (assign41090_e46943 * assign41090_e46943)), (((((locals.var_dqid_dxn_qi__blk1056_dn8 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn8)) * assign41090_e46943) - (assign41090_e46940 * locals.var_dqid_dxn_qi__blk1056_dn8)) / (assign41090_e46943 * assign41090_e46943)), (((((locals.var_dqid_dxn_qi__blk1056_dn9 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn9)) * assign41090_e46943) - (assign41090_e46940 * locals.var_dqid_dxn_qi__blk1056_dn9)) / (assign41090_e46943 * assign41090_e46943)),)
    } else {
        (locals.var_dd__blk1057, locals.var_dd__blk1057_dn4, locals.var_dd__blk1057_dn6, locals.var_dd__blk1057_dn7, locals.var_dd__blk1057_dn8, locals.var_dd__blk1057_dn9,)
    }
};
        locals.var_dd__blk1057 = assign41090_e46946;
        locals.var_dd__blk1057_dn4 = assign41090_e46946_d_n4;
        locals.var_dd__blk1057_dn6 = assign41090_e46946_d_n6;
        locals.var_dd__blk1057_dn7 = assign41090_e46946_d_n7;
        locals.var_dd__blk1057_dn8 = assign41090_e46946_d_n8;
        locals.var_dd__blk1057_dn9 = assign41090_e46946_d_n9;

        let (assign41100_e46973, assign41100_e46973_d_n4, assign41100_e46973_d_n6, assign41100_e46973_d_n7, assign41100_e46973_d_n8, assign41100_e46973_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 == 0.0)) {
        let assign41100_e46957: f64 = (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013);
        let assign41100_e46960: f64 = (locals.var_a1d__blk1011 * locals.var_a2d__blk1012);
        let assign41100_e46961: f64 = (assign41100_e46957 / assign41100_e46960);
        let assign41100_e46964: f64 = (locals.var_aexp1d__blk1007 / locals.var_a1d__blk1011);
        let assign41100_e46967: f64 = (locals.var_aexp2d__blk1008 / locals.var_a2d__blk1012);
        let assign41100_e46968: f64 = (assign41100_e46964 + assign41100_e46967);
        let assign41100_e46970: f64 = (assign41100_e46968 / locals.var_qid__blk1003);
        let assign41100_e46971: f64 = (assign41100_e46961 - assign41100_e46970);
        (assign41100_e46971, ((((((locals.var_dqsqd_dxn_qi__blk1014_dn4 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn4)) * assign41100_e46960) - (assign41100_e46957 * ((locals.var_a1d__blk1011_dn4 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn4)))) / (assign41100_e46960 * assign41100_e46960)) - (((((((locals.var_aexp1d__blk1007_dn4 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn4)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn4 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn4)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41100_e46968 * locals.var_qid__blk1003_dn4)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), ((((((locals.var_dqsqd_dxn_qi__blk1014_dn6 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn6)) * assign41100_e46960) - (assign41100_e46957 * ((locals.var_a1d__blk1011_dn6 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn6)))) / (assign41100_e46960 * assign41100_e46960)) - (((((((locals.var_aexp1d__blk1007_dn6 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn6)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn6 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn6)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41100_e46968 * locals.var_qid__blk1003_dn6)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), ((((((locals.var_dqsqd_dxn_qi__blk1014_dn7 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn7)) * assign41100_e46960) - (assign41100_e46957 * ((locals.var_a1d__blk1011_dn7 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn7)))) / (assign41100_e46960 * assign41100_e46960)) - (((((((locals.var_aexp1d__blk1007_dn7 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn7)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn7 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn7)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41100_e46968 * locals.var_qid__blk1003_dn7)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), ((((((locals.var_dqsqd_dxn_qi__blk1014_dn8 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn8)) * assign41100_e46960) - (assign41100_e46957 * ((locals.var_a1d__blk1011_dn8 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn8)))) / (assign41100_e46960 * assign41100_e46960)) - (((((((locals.var_aexp1d__blk1007_dn8 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn8)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn8 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn8)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41100_e46968 * locals.var_qid__blk1003_dn8)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), ((((((locals.var_dqsqd_dxn_qi__blk1014_dn9 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn9)) * assign41100_e46960) - (assign41100_e46957 * ((locals.var_a1d__blk1011_dn9 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn9)))) / (assign41100_e46960 * assign41100_e46960)) - (((((((locals.var_aexp1d__blk1007_dn9 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn9)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn9 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn9)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41100_e46968 * locals.var_qid__blk1003_dn9)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))),)
    } else {
        (locals.var_dqid_dxn_qi__blk1056, locals.var_dqid_dxn_qi__blk1056_dn4, locals.var_dqid_dxn_qi__blk1056_dn6, locals.var_dqid_dxn_qi__blk1056_dn7, locals.var_dqid_dxn_qi__blk1056_dn8, locals.var_dqid_dxn_qi__blk1056_dn9,)
    }
};
        locals.var_dqid_dxn_qi__blk1056 = assign41100_e46973;
        locals.var_dqid_dxn_qi__blk1056_dn4 = assign41100_e46973_d_n4;
        locals.var_dqid_dxn_qi__blk1056_dn6 = assign41100_e46973_d_n6;
        locals.var_dqid_dxn_qi__blk1056_dn7 = assign41100_e46973_d_n7;
        locals.var_dqid_dxn_qi__blk1056_dn8 = assign41100_e46973_d_n8;
        locals.var_dqid_dxn_qi__blk1056_dn9 = assign41100_e46973_d_n9;

        let (assign41110_e46990, assign41110_e46990_d_n4, assign41110_e46990_d_n6, assign41110_e46990_d_n7, assign41110_e46990_d_n8, assign41110_e46990_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 == 0.0)) {
        let assign41110_e46984: f64 = (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003);
        let assign41110_e46987: f64 = (locals.var_dqid_dxn_qi__blk1056 + 1.0);
        let assign41110_e46988: f64 = (assign41110_e46984 / assign41110_e46987);
        (assign41110_e46988, (((((locals.var_dqid_dxn_qi__blk1056_dn4 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn4)) * assign41110_e46987) - (assign41110_e46984 * locals.var_dqid_dxn_qi__blk1056_dn4)) / (assign41110_e46987 * assign41110_e46987)), (((((locals.var_dqid_dxn_qi__blk1056_dn6 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn6)) * assign41110_e46987) - (assign41110_e46984 * locals.var_dqid_dxn_qi__blk1056_dn6)) / (assign41110_e46987 * assign41110_e46987)), (((((locals.var_dqid_dxn_qi__blk1056_dn7 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn7)) * assign41110_e46987) - (assign41110_e46984 * locals.var_dqid_dxn_qi__blk1056_dn7)) / (assign41110_e46987 * assign41110_e46987)), (((((locals.var_dqid_dxn_qi__blk1056_dn8 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn8)) * assign41110_e46987) - (assign41110_e46984 * locals.var_dqid_dxn_qi__blk1056_dn8)) / (assign41110_e46987 * assign41110_e46987)), (((((locals.var_dqid_dxn_qi__blk1056_dn9 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn9)) * assign41110_e46987) - (assign41110_e46984 * locals.var_dqid_dxn_qi__blk1056_dn9)) / (assign41110_e46987 * assign41110_e46987)),)
    } else {
        (locals.var_dd__blk1057, locals.var_dd__blk1057_dn4, locals.var_dd__blk1057_dn6, locals.var_dd__blk1057_dn7, locals.var_dd__blk1057_dn8, locals.var_dd__blk1057_dn9,)
    }
};
        locals.var_dd__blk1057 = assign41110_e46990;
        locals.var_dd__blk1057_dn4 = assign41110_e46990_d_n4;
        locals.var_dd__blk1057_dn6 = assign41110_e46990_d_n6;
        locals.var_dd__blk1057_dn7 = assign41110_e46990_d_n7;
        locals.var_dd__blk1057_dn8 = assign41110_e46990_d_n8;
        locals.var_dd__blk1057_dn9 = assign41110_e46990_d_n9;

        let (assign41120_e46999, assign41120_e46999_d_n4, assign41120_e46999_d_n6, assign41120_e46999_d_n7, assign41120_e46999_d_n8, assign41120_e46999_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 == 0.0)) {
        (locals.var_dinf__blk974, locals.var_dinf__blk974_dn4, locals.var_dinf__blk974_dn6, locals.var_dinf__blk974_dn7, locals.var_dinf__blk974_dn8, locals.var_dinf__blk974_dn9,)
    } else {
        (locals.var_dd__blk1057, locals.var_dd__blk1057_dn4, locals.var_dd__blk1057_dn6, locals.var_dd__blk1057_dn7, locals.var_dd__blk1057_dn8, locals.var_dd__blk1057_dn9,)
    }
};
        locals.var_dd__blk1057 = assign41120_e46999;
        locals.var_dd__blk1057_dn4 = assign41120_e46999_d_n4;
        locals.var_dd__blk1057_dn6 = assign41120_e46999_d_n6;
        locals.var_dd__blk1057_dn7 = assign41120_e46999_d_n7;
        locals.var_dd__blk1057_dn8 = assign41120_e46999_d_n8;
        locals.var_dd__blk1057_dn9 = assign41120_e46999_d_n9;

        let (assign41130_e47007, assign41130_e47007_d_n4, assign41130_e47007_d_n6, assign41130_e47007_d_n7, assign41130_e47007_d_n8, assign41130_e47007_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) {
        let assign41130_e47005: f64 = (locals.var_dd__blk1057 - locals.var_ds__blk981);
        (assign41130_e47005, (locals.var_dd__blk1057_dn4 - locals.var_ds__blk981_dn4), (locals.var_dd__blk1057_dn6 - locals.var_ds__blk981_dn6), (locals.var_dd__blk1057_dn7 - locals.var_ds__blk981_dn7), (locals.var_dd__blk1057_dn8 - locals.var_ds__blk981_dn8), (locals.var_dd__blk1057_dn9 - locals.var_ds__blk981_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign41130_e47007;
        locals.var_temp1_dn4 = assign41130_e47007_d_n4;
        locals.var_temp1_dn6 = assign41130_e47007_d_n6;
        locals.var_temp1_dn7 = assign41130_e47007_d_n7;
        locals.var_temp1_dn8 = assign41130_e47007_d_n8;
        locals.var_temp1_dn9 = assign41130_e47007_d_n9;

        let (assign41140_e47019, assign41140_e47019_d_n4, assign41140_e47019_d_n6, assign41140_e47019_d_n7, assign41140_e47019_d_n8, assign41140_e47019_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) {
        let assign41140_e47014: f64 = (36.0 * locals.var_temp1);
        let assign41140_e47016: f64 = (assign41140_e47014 * locals.var_temp1);
        let assign41140_e47017: f64 = (1.0 + assign41140_e47016);
        (assign41140_e47017, (((36.0 * locals.var_temp1_dn4) * locals.var_temp1) + (assign41140_e47014 * locals.var_temp1_dn4)), (((36.0 * locals.var_temp1_dn6) * locals.var_temp1) + (assign41140_e47014 * locals.var_temp1_dn6)), (((36.0 * locals.var_temp1_dn7) * locals.var_temp1) + (assign41140_e47014 * locals.var_temp1_dn7)), (((36.0 * locals.var_temp1_dn8) * locals.var_temp1) + (assign41140_e47014 * locals.var_temp1_dn8)), (((36.0 * locals.var_temp1_dn9) * locals.var_temp1) + (assign41140_e47014 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign41140_e47019;
        locals.var_temp2_dn4 = assign41140_e47019_d_n4;
        locals.var_temp2_dn6 = assign41140_e47019_d_n6;
        locals.var_temp2_dn7 = assign41140_e47019_d_n7;
        locals.var_temp2_dn8 = assign41140_e47019_d_n8;
        locals.var_temp2_dn9 = assign41140_e47019_d_n9;

        let assign41150_e47021: f64 = (locals.var_temp1).abs();
        let assign41150_e47023: f64 = if assign41150_e47021 > 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1231 = assign41150_e47023;

        let (assign41160_e47033, assign41160_e47033_d_n4, assign41160_e47033_d_n6, assign41160_e47033_d_n7, assign41160_e47033_d_n8, assign41160_e47033_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign41160_e47031: f64 = (locals.var_qid__blk1003 - locals.var_qis__blk938);
        (assign41160_e47031, (locals.var_qid__blk1003_dn4 - locals.var_qis__blk938_dn4), (locals.var_qid__blk1003_dn6 - locals.var_qis__blk938_dn6), (locals.var_qid__blk1003_dn7 - locals.var_qis__blk938_dn7), (locals.var_qid__blk1003_dn8 - locals.var_qis__blk938_dn8), (locals.var_qid__blk1003_dn9 - locals.var_qis__blk938_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign41160_e47033;
        locals.var_temp3_dn4 = assign41160_e47033_d_n4;
        locals.var_temp3_dn6 = assign41160_e47033_d_n6;
        locals.var_temp3_dn7 = assign41160_e47033_d_n7;
        locals.var_temp3_dn8 = assign41160_e47033_d_n8;
        locals.var_temp3_dn9 = assign41160_e47033_d_n9;

        let (assign41170_e47045, assign41170_e47045_d_n4, assign41170_e47045_d_n6, assign41170_e47045_d_n7, assign41170_e47045_d_n8, assign41170_e47045_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign41170_e47042: f64 = (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017);
        let assign41170_e47043: f64 = (locals.var_temp3 - assign41170_e47042);
        (assign41170_e47043, (locals.var_temp3_dn4 - ((locals.var_dd__blk1057_dn4 * locals.var_dxdrift__blk1017) + (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017_dn4))), (locals.var_temp3_dn6 - ((locals.var_dd__blk1057_dn6 * locals.var_dxdrift__blk1017) + (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017_dn6))), (locals.var_temp3_dn7 - ((locals.var_dd__blk1057_dn7 * locals.var_dxdrift__blk1017) + (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017_dn7))), (locals.var_temp3_dn8 - ((locals.var_dd__blk1057_dn8 * locals.var_dxdrift__blk1017) + (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017_dn8))), (locals.var_temp3_dn9 - ((locals.var_dd__blk1057_dn9 * locals.var_dxdrift__blk1017) + (locals.var_dd__blk1057 * locals.var_dxdrift__blk1017_dn9))),)
    } else {
        (locals.var_ls__blk1058, locals.var_ls__blk1058_dn4, locals.var_ls__blk1058_dn6, locals.var_ls__blk1058_dn7, locals.var_ls__blk1058_dn8, locals.var_ls__blk1058_dn9,)
    }
};
        locals.var_ls__blk1058 = assign41170_e47045;
        locals.var_ls__blk1058_dn4 = assign41170_e47045_d_n4;
        locals.var_ls__blk1058_dn6 = assign41170_e47045_d_n6;
        locals.var_ls__blk1058_dn7 = assign41170_e47045_d_n7;
        locals.var_ls__blk1058_dn8 = assign41170_e47045_d_n8;
        locals.var_ls__blk1058_dn9 = assign41170_e47045_d_n9;

        let (assign41180_e47057, assign41180_e47057_d_n4, assign41180_e47057_d_n6, assign41180_e47057_d_n7, assign41180_e47057_d_n8, assign41180_e47057_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign41180_e47054: f64 = (locals.var_ds__blk981 * locals.var_dxdrift__blk1017);
        let assign41180_e47055: f64 = (locals.var_temp3 - assign41180_e47054);
        (assign41180_e47055, (locals.var_temp3_dn4 - ((locals.var_ds__blk981_dn4 * locals.var_dxdrift__blk1017) + (locals.var_ds__blk981 * locals.var_dxdrift__blk1017_dn4))), (locals.var_temp3_dn6 - ((locals.var_ds__blk981_dn6 * locals.var_dxdrift__blk1017) + (locals.var_ds__blk981 * locals.var_dxdrift__blk1017_dn6))), (locals.var_temp3_dn7 - ((locals.var_ds__blk981_dn7 * locals.var_dxdrift__blk1017) + (locals.var_ds__blk981 * locals.var_dxdrift__blk1017_dn7))), (locals.var_temp3_dn8 - ((locals.var_ds__blk981_dn8 * locals.var_dxdrift__blk1017) + (locals.var_ds__blk981 * locals.var_dxdrift__blk1017_dn8))), (locals.var_temp3_dn9 - ((locals.var_ds__blk981_dn9 * locals.var_dxdrift__blk1017) + (locals.var_ds__blk981 * locals.var_dxdrift__blk1017_dn9))),)
    } else {
        (locals.var_ld__blk1059, locals.var_ld__blk1059_dn4, locals.var_ld__blk1059_dn6, locals.var_ld__blk1059_dn7, locals.var_ld__blk1059_dn8, locals.var_ld__blk1059_dn9,)
    }
};
        locals.var_ld__blk1059 = assign41180_e47057;
        locals.var_ld__blk1059_dn4 = assign41180_e47057_d_n4;
        locals.var_ld__blk1059_dn6 = assign41180_e47057_d_n6;
        locals.var_ld__blk1059_dn7 = assign41180_e47057_d_n7;
        locals.var_ld__blk1059_dn8 = assign41180_e47057_d_n8;
        locals.var_ld__blk1059_dn9 = assign41180_e47057_d_n9;

        let (assign41190_e47070, assign41190_e47070_d_n4, assign41190_e47070_d_n6, assign41190_e47070_d_n7, assign41190_e47070_d_n8, assign41190_e47070_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign41190_e47065: f64 = (locals.var_ls__blk1058 * locals.var_ls__blk1058);
        let assign41190_e47067: f64 = (assign41190_e47065 + locals.var_temp2);
        let assign41190_e47068: f64 = (assign41190_e47067).sqrt();
        (assign41190_e47068, ((((locals.var_ls__blk1058_dn4 * locals.var_ls__blk1058) + (locals.var_ls__blk1058 * locals.var_ls__blk1058_dn4)) + locals.var_temp2_dn4) / (2.0 * assign41190_e47068)), ((((locals.var_ls__blk1058_dn6 * locals.var_ls__blk1058) + (locals.var_ls__blk1058 * locals.var_ls__blk1058_dn6)) + locals.var_temp2_dn6) / (2.0 * assign41190_e47068)), ((((locals.var_ls__blk1058_dn7 * locals.var_ls__blk1058) + (locals.var_ls__blk1058 * locals.var_ls__blk1058_dn7)) + locals.var_temp2_dn7) / (2.0 * assign41190_e47068)), ((((locals.var_ls__blk1058_dn8 * locals.var_ls__blk1058) + (locals.var_ls__blk1058 * locals.var_ls__blk1058_dn8)) + locals.var_temp2_dn8) / (2.0 * assign41190_e47068)), ((((locals.var_ls__blk1058_dn9 * locals.var_ls__blk1058) + (locals.var_ls__blk1058 * locals.var_ls__blk1058_dn9)) + locals.var_temp2_dn9) / (2.0 * assign41190_e47068)),)
    } else {
        (locals.var_us__blk1060, locals.var_us__blk1060_dn4, locals.var_us__blk1060_dn6, locals.var_us__blk1060_dn7, locals.var_us__blk1060_dn8, locals.var_us__blk1060_dn9,)
    }
};
        locals.var_us__blk1060 = assign41190_e47070;
        locals.var_us__blk1060_dn4 = assign41190_e47070_d_n4;
        locals.var_us__blk1060_dn6 = assign41190_e47070_d_n6;
        locals.var_us__blk1060_dn7 = assign41190_e47070_d_n7;
        locals.var_us__blk1060_dn8 = assign41190_e47070_d_n8;
        locals.var_us__blk1060_dn9 = assign41190_e47070_d_n9;

        let (assign41200_e47083, assign41200_e47083_d_n4, assign41200_e47083_d_n6, assign41200_e47083_d_n7, assign41200_e47083_d_n8, assign41200_e47083_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign41200_e47078: f64 = (locals.var_ld__blk1059 * locals.var_ld__blk1059);
        let assign41200_e47080: f64 = (assign41200_e47078 + locals.var_temp2);
        let assign41200_e47081: f64 = (assign41200_e47080).sqrt();
        (assign41200_e47081, ((((locals.var_ld__blk1059_dn4 * locals.var_ld__blk1059) + (locals.var_ld__blk1059 * locals.var_ld__blk1059_dn4)) + locals.var_temp2_dn4) / (2.0 * assign41200_e47081)), ((((locals.var_ld__blk1059_dn6 * locals.var_ld__blk1059) + (locals.var_ld__blk1059 * locals.var_ld__blk1059_dn6)) + locals.var_temp2_dn6) / (2.0 * assign41200_e47081)), ((((locals.var_ld__blk1059_dn7 * locals.var_ld__blk1059) + (locals.var_ld__blk1059 * locals.var_ld__blk1059_dn7)) + locals.var_temp2_dn7) / (2.0 * assign41200_e47081)), ((((locals.var_ld__blk1059_dn8 * locals.var_ld__blk1059) + (locals.var_ld__blk1059 * locals.var_ld__blk1059_dn8)) + locals.var_temp2_dn8) / (2.0 * assign41200_e47081)), ((((locals.var_ld__blk1059_dn9 * locals.var_ld__blk1059) + (locals.var_ld__blk1059 * locals.var_ld__blk1059_dn9)) + locals.var_temp2_dn9) / (2.0 * assign41200_e47081)),)
    } else {
        (locals.var_ud__blk1061, locals.var_ud__blk1061_dn4, locals.var_ud__blk1061_dn6, locals.var_ud__blk1061_dn7, locals.var_ud__blk1061_dn8, locals.var_ud__blk1061_dn9,)
    }
};
        locals.var_ud__blk1061 = assign41200_e47083;
        locals.var_ud__blk1061_dn4 = assign41200_e47083_d_n4;
        locals.var_ud__blk1061_dn6 = assign41200_e47083_d_n6;
        locals.var_ud__blk1061_dn7 = assign41200_e47083_d_n7;
        locals.var_ud__blk1061_dn8 = assign41200_e47083_d_n8;
        locals.var_ud__blk1061_dn9 = assign41200_e47083_d_n9;

        let (assign41210_e47112, assign41210_e47112_d_n4, assign41210_e47112_d_n6, assign41210_e47112_d_n7, assign41210_e47112_d_n8, assign41210_e47112_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign41210_e47091: f64 = (0.25 / locals.var_temp1);
        let assign41210_e47094: f64 = (locals.var_ud__blk1061 * locals.var_ls__blk1058);
        let assign41210_e47097: f64 = (locals.var_us__blk1060 * locals.var_ld__blk1059);
        let assign41210_e47098: f64 = (assign41210_e47094 - assign41210_e47097);
        let assign41210_e47102: f64 = (locals.var_ld__blk1059 + locals.var_ud__blk1061);
        let assign41210_e47105: f64 = (locals.var_ls__blk1058 + locals.var_us__blk1060);
        let assign41210_e47106: f64 = (assign41210_e47102 / assign41210_e47105);
        let assign41210_e47107: f64 = (assign41210_e47106).ln();
        let assign41210_e47108: f64 = (locals.var_temp2 * assign41210_e47107);
        let assign41210_e47109: f64 = (assign41210_e47098 + assign41210_e47108);
        let assign41210_e47110: f64 = (assign41210_e47091 * assign41210_e47109);
        (assign41210_e47110, (((-((0.25 * locals.var_temp1_dn4) / (locals.var_temp1 * locals.var_temp1))) * assign41210_e47109) + (assign41210_e47091 * ((((locals.var_ud__blk1061_dn4 * locals.var_ls__blk1058) + (locals.var_ud__blk1061 * locals.var_ls__blk1058_dn4)) - ((locals.var_us__blk1060_dn4 * locals.var_ld__blk1059) + (locals.var_us__blk1060 * locals.var_ld__blk1059_dn4))) + ((locals.var_temp2_dn4 * assign41210_e47107) + (locals.var_temp2 * (((((locals.var_ld__blk1059_dn4 + locals.var_ud__blk1061_dn4) * assign41210_e47105) - (assign41210_e47102 * (locals.var_ls__blk1058_dn4 + locals.var_us__blk1060_dn4))) / (assign41210_e47105 * assign41210_e47105)) / assign41210_e47106)))))), (((-((0.25 * locals.var_temp1_dn6) / (locals.var_temp1 * locals.var_temp1))) * assign41210_e47109) + (assign41210_e47091 * ((((locals.var_ud__blk1061_dn6 * locals.var_ls__blk1058) + (locals.var_ud__blk1061 * locals.var_ls__blk1058_dn6)) - ((locals.var_us__blk1060_dn6 * locals.var_ld__blk1059) + (locals.var_us__blk1060 * locals.var_ld__blk1059_dn6))) + ((locals.var_temp2_dn6 * assign41210_e47107) + (locals.var_temp2 * (((((locals.var_ld__blk1059_dn6 + locals.var_ud__blk1061_dn6) * assign41210_e47105) - (assign41210_e47102 * (locals.var_ls__blk1058_dn6 + locals.var_us__blk1060_dn6))) / (assign41210_e47105 * assign41210_e47105)) / assign41210_e47106)))))), (((-((0.25 * locals.var_temp1_dn7) / (locals.var_temp1 * locals.var_temp1))) * assign41210_e47109) + (assign41210_e47091 * ((((locals.var_ud__blk1061_dn7 * locals.var_ls__blk1058) + (locals.var_ud__blk1061 * locals.var_ls__blk1058_dn7)) - ((locals.var_us__blk1060_dn7 * locals.var_ld__blk1059) + (locals.var_us__blk1060 * locals.var_ld__blk1059_dn7))) + ((locals.var_temp2_dn7 * assign41210_e47107) + (locals.var_temp2 * (((((locals.var_ld__blk1059_dn7 + locals.var_ud__blk1061_dn7) * assign41210_e47105) - (assign41210_e47102 * (locals.var_ls__blk1058_dn7 + locals.var_us__blk1060_dn7))) / (assign41210_e47105 * assign41210_e47105)) / assign41210_e47106)))))), (((-((0.25 * locals.var_temp1_dn8) / (locals.var_temp1 * locals.var_temp1))) * assign41210_e47109) + (assign41210_e47091 * ((((locals.var_ud__blk1061_dn8 * locals.var_ls__blk1058) + (locals.var_ud__blk1061 * locals.var_ls__blk1058_dn8)) - ((locals.var_us__blk1060_dn8 * locals.var_ld__blk1059) + (locals.var_us__blk1060 * locals.var_ld__blk1059_dn8))) + ((locals.var_temp2_dn8 * assign41210_e47107) + (locals.var_temp2 * (((((locals.var_ld__blk1059_dn8 + locals.var_ud__blk1061_dn8) * assign41210_e47105) - (assign41210_e47102 * (locals.var_ls__blk1058_dn8 + locals.var_us__blk1060_dn8))) / (assign41210_e47105 * assign41210_e47105)) / assign41210_e47106)))))), (((-((0.25 * locals.var_temp1_dn9) / (locals.var_temp1 * locals.var_temp1))) * assign41210_e47109) + (assign41210_e47091 * ((((locals.var_ud__blk1061_dn9 * locals.var_ls__blk1058) + (locals.var_ud__blk1061 * locals.var_ls__blk1058_dn9)) - ((locals.var_us__blk1060_dn9 * locals.var_ld__blk1059) + (locals.var_us__blk1060 * locals.var_ld__blk1059_dn9))) + ((locals.var_temp2_dn9 * assign41210_e47107) + (locals.var_temp2 * (((((locals.var_ld__blk1059_dn9 + locals.var_ud__blk1061_dn9) * assign41210_e47105) - (assign41210_e47102 * (locals.var_ls__blk1058_dn9 + locals.var_us__blk1060_dn9))) / (assign41210_e47105 * assign41210_e47105)) / assign41210_e47106)))))),)
    } else {
        (locals.var_idrift2__blk1062, locals.var_idrift2__blk1062_dn4, locals.var_idrift2__blk1062_dn6, locals.var_idrift2__blk1062_dn7, locals.var_idrift2__blk1062_dn8, locals.var_idrift2__blk1062_dn9,)
    }
};
        locals.var_idrift2__blk1062 = assign41210_e47112;
        locals.var_idrift2__blk1062_dn4 = assign41210_e47112_d_n4;
        locals.var_idrift2__blk1062_dn6 = assign41210_e47112_d_n6;
        locals.var_idrift2__blk1062_dn7 = assign41210_e47112_d_n7;
        locals.var_idrift2__blk1062_dn8 = assign41210_e47112_d_n8;
        locals.var_idrift2__blk1062_dn9 = assign41210_e47112_d_n9;

        let (assign41220_e47123, assign41220_e47123_d_n4, assign41220_e47123_d_n6, assign41220_e47123_d_n7, assign41220_e47123_d_n8, assign41220_e47123_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 == 0.0)) {
        let assign41220_e47121: f64 = (locals.var_dxdrift__blk1017 * locals.var_temp1);
        (assign41220_e47121, ((locals.var_dxdrift__blk1017_dn4 * locals.var_temp1) + (locals.var_dxdrift__blk1017 * locals.var_temp1_dn4)), ((locals.var_dxdrift__blk1017_dn6 * locals.var_temp1) + (locals.var_dxdrift__blk1017 * locals.var_temp1_dn6)), ((locals.var_dxdrift__blk1017_dn7 * locals.var_temp1) + (locals.var_dxdrift__blk1017 * locals.var_temp1_dn7)), ((locals.var_dxdrift__blk1017_dn8 * locals.var_temp1) + (locals.var_dxdrift__blk1017 * locals.var_temp1_dn8)), ((locals.var_dxdrift__blk1017_dn9 * locals.var_temp1) + (locals.var_dxdrift__blk1017 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign41220_e47123;
        locals.var_temp3_dn4 = assign41220_e47123_d_n4;
        locals.var_temp3_dn6 = assign41220_e47123_d_n6;
        locals.var_temp3_dn7 = assign41220_e47123_d_n7;
        locals.var_temp3_dn8 = assign41220_e47123_d_n8;
        locals.var_temp3_dn9 = assign41220_e47123_d_n9;

        let (assign41230_e47144, assign41230_e47144_d_n4, assign41230_e47144_d_n6, assign41230_e47144_d_n7, assign41230_e47144_d_n8, assign41230_e47144_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1231 == 0.0)) {
        let assign41230_e47131: f64 = (-0.25);
        let assign41230_e47133: f64 = (assign41230_e47131 * 0.1666666666667);
        let assign41230_e47135: f64 = (assign41230_e47133 * locals.var_dxdrift__blk1017);
        let assign41230_e47137: f64 = (assign41230_e47135 * locals.var_temp3);
        let assign41230_e47139: f64 = (assign41230_e47137 * locals.var_temp3);
        let assign41230_e47141: f64 = (locals.var_temp2).sqrt();
        let assign41230_e47142: f64 = (assign41230_e47139 / assign41230_e47141);
        (assign41230_e47142, ((((((((assign41230_e47133 * locals.var_dxdrift__blk1017_dn4) * locals.var_temp3) + (assign41230_e47135 * locals.var_temp3_dn4)) * locals.var_temp3) + (assign41230_e47137 * locals.var_temp3_dn4)) * assign41230_e47141) - (assign41230_e47139 * (locals.var_temp2_dn4 / (2.0 * assign41230_e47141)))) / (assign41230_e47141 * assign41230_e47141)), ((((((((assign41230_e47133 * locals.var_dxdrift__blk1017_dn6) * locals.var_temp3) + (assign41230_e47135 * locals.var_temp3_dn6)) * locals.var_temp3) + (assign41230_e47137 * locals.var_temp3_dn6)) * assign41230_e47141) - (assign41230_e47139 * (locals.var_temp2_dn6 / (2.0 * assign41230_e47141)))) / (assign41230_e47141 * assign41230_e47141)), ((((((((assign41230_e47133 * locals.var_dxdrift__blk1017_dn7) * locals.var_temp3) + (assign41230_e47135 * locals.var_temp3_dn7)) * locals.var_temp3) + (assign41230_e47137 * locals.var_temp3_dn7)) * assign41230_e47141) - (assign41230_e47139 * (locals.var_temp2_dn7 / (2.0 * assign41230_e47141)))) / (assign41230_e47141 * assign41230_e47141)), ((((((((assign41230_e47133 * locals.var_dxdrift__blk1017_dn8) * locals.var_temp3) + (assign41230_e47135 * locals.var_temp3_dn8)) * locals.var_temp3) + (assign41230_e47137 * locals.var_temp3_dn8)) * assign41230_e47141) - (assign41230_e47139 * (locals.var_temp2_dn8 / (2.0 * assign41230_e47141)))) / (assign41230_e47141 * assign41230_e47141)), ((((((((assign41230_e47133 * locals.var_dxdrift__blk1017_dn9) * locals.var_temp3) + (assign41230_e47135 * locals.var_temp3_dn9)) * locals.var_temp3) + (assign41230_e47137 * locals.var_temp3_dn9)) * assign41230_e47141) - (assign41230_e47139 * (locals.var_temp2_dn9 / (2.0 * assign41230_e47141)))) / (assign41230_e47141 * assign41230_e47141)),)
    } else {
        (locals.var_idrift2__blk1062, locals.var_idrift2__blk1062_dn4, locals.var_idrift2__blk1062_dn6, locals.var_idrift2__blk1062_dn7, locals.var_idrift2__blk1062_dn8, locals.var_idrift2__blk1062_dn9,)
    }
};
        locals.var_idrift2__blk1062 = assign41230_e47144;
        locals.var_idrift2__blk1062_dn4 = assign41230_e47144_d_n4;
        locals.var_idrift2__blk1062_dn6 = assign41230_e47144_d_n6;
        locals.var_idrift2__blk1062_dn7 = assign41230_e47144_d_n7;
        locals.var_idrift2__blk1062_dn8 = assign41230_e47144_d_n8;
        locals.var_idrift2__blk1062_dn9 = assign41230_e47144_d_n9;

        let (assign41240_e47151, assign41240_e47151_d_n4, assign41240_e47151_d_n6, assign41240_e47151_d_n7, assign41240_e47151_d_n8, assign41240_e47151_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1228 == 0.0)) {
        (locals.var_dinf__blk974, locals.var_dinf__blk974_dn4, locals.var_dinf__blk974_dn6, locals.var_dinf__blk974_dn7, locals.var_dinf__blk974_dn8, locals.var_dinf__blk974_dn9,)
    } else {
        (locals.var_dd__blk1057, locals.var_dd__blk1057_dn4, locals.var_dd__blk1057_dn6, locals.var_dd__blk1057_dn7, locals.var_dd__blk1057_dn8, locals.var_dd__blk1057_dn9,)
    }
};
        locals.var_dd__blk1057 = assign41240_e47151;
        locals.var_dd__blk1057_dn4 = assign41240_e47151_d_n4;
        locals.var_dd__blk1057_dn6 = assign41240_e47151_d_n6;
        locals.var_dd__blk1057_dn7 = assign41240_e47151_d_n7;
        locals.var_dd__blk1057_dn8 = assign41240_e47151_d_n8;
        locals.var_dd__blk1057_dn9 = assign41240_e47151_d_n9;

        let (assign41250_e47158, assign41250_e47158_d_n4, assign41250_e47158_d_n6, assign41250_e47158_d_n7, assign41250_e47158_d_n8, assign41250_e47158_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1228 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idrift2__blk1062, locals.var_idrift2__blk1062_dn4, locals.var_idrift2__blk1062_dn6, locals.var_idrift2__blk1062_dn7, locals.var_idrift2__blk1062_dn8, locals.var_idrift2__blk1062_dn9,)
    }
};
        locals.var_idrift2__blk1062 = assign41250_e47158;
        locals.var_idrift2__blk1062_dn4 = assign41250_e47158_d_n4;
        locals.var_idrift2__blk1062_dn6 = assign41250_e47158_d_n6;
        locals.var_idrift2__blk1062_dn7 = assign41250_e47158_d_n7;
        locals.var_idrift2__blk1062_dn8 = assign41250_e47158_d_n8;
        locals.var_idrift2__blk1062_dn9 = assign41250_e47158_d_n9;

        let (assign41260_e47170, assign41260_e47170_d_n4, assign41260_e47170_d_n6, assign41260_e47170_d_n7, assign41260_e47170_d_n8, assign41260_e47170_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign41260_e47162: f64 = (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017);
        let assign41260_e47164: f64 = (assign41260_e47162 + locals.var_idrift2__blk1062);
        let assign41260_e47166: f64 = (assign41260_e47164 + locals.var_qis__blk938);
        let assign41260_e47168: f64 = (assign41260_e47166 - locals.var_qid__blk1003);
        (assign41260_e47168, (((((locals.var_qim__blk1016_dn4 * locals.var_dxdrift__blk1017) + (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017_dn4)) + locals.var_idrift2__blk1062_dn4) + locals.var_qis__blk938_dn4) - locals.var_qid__blk1003_dn4), (((((locals.var_qim__blk1016_dn6 * locals.var_dxdrift__blk1017) + (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017_dn6)) + locals.var_idrift2__blk1062_dn6) + locals.var_qis__blk938_dn6) - locals.var_qid__blk1003_dn6), (((((locals.var_qim__blk1016_dn7 * locals.var_dxdrift__blk1017) + (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017_dn7)) + locals.var_idrift2__blk1062_dn7) + locals.var_qis__blk938_dn7) - locals.var_qid__blk1003_dn7), (((((locals.var_qim__blk1016_dn8 * locals.var_dxdrift__blk1017) + (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017_dn8)) + locals.var_idrift2__blk1062_dn8) + locals.var_qis__blk938_dn8) - locals.var_qid__blk1003_dn8), (((((locals.var_qim__blk1016_dn9 * locals.var_dxdrift__blk1017) + (locals.var_qim__blk1016 * locals.var_dxdrift__blk1017_dn9)) + locals.var_idrift2__blk1062_dn9) + locals.var_qis__blk938_dn9) - locals.var_qid__blk1003_dn9),)
    } else {
        (locals.var_norm_ids__blk1063, locals.var_norm_ids__blk1063_dn4, locals.var_norm_ids__blk1063_dn6, locals.var_norm_ids__blk1063_dn7, locals.var_norm_ids__blk1063_dn8, locals.var_norm_ids__blk1063_dn9,)
    }
};
        locals.var_norm_ids__blk1063 = assign41260_e47170;
        locals.var_norm_ids__blk1063_dn4 = assign41260_e47170_d_n4;
        locals.var_norm_ids__blk1063_dn6 = assign41260_e47170_d_n6;
        locals.var_norm_ids__blk1063_dn7 = assign41260_e47170_d_n7;
        locals.var_norm_ids__blk1063_dn8 = assign41260_e47170_d_n8;
        locals.var_norm_ids__blk1063_dn9 = assign41260_e47170_d_n9;

        let assign41270_e47173: f64 = if locals.var_qis__blk938 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1232 = assign41270_e47173;

        let assign41280_e47176: f64 = if locals.var_norm_ids__blk1063 > 1e-30 { 1.0 } else { 0.0 };
        locals.var_guard1233 = assign41280_e47176;

        let (assign41290_e47190, assign41290_e47190_d_n4, assign41290_e47190_d_n6, assign41290_e47190_d_n7, assign41290_e47190_d_n8, assign41290_e47190_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
        let assign41290_e47185: f64 = (locals.var_aexp1s__blk943 / locals.var_qis__blk938);
        let assign41290_e47187: f64 = (assign41290_e47185 - locals.var_dqsqs_dxn_qi__blk950);
        let assign41290_e47188: f64 = (locals.var_a1s__blk947 / assign41290_e47187);
        (assign41290_e47188, (((locals.var_a1s__blk947_dn4 * assign41290_e47187) - (locals.var_a1s__blk947 * ((((locals.var_aexp1s__blk943_dn4 * locals.var_qis__blk938) - (locals.var_aexp1s__blk943 * locals.var_qis__blk938_dn4)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn4))) / (assign41290_e47187 * assign41290_e47187)), (((locals.var_a1s__blk947_dn6 * assign41290_e47187) - (locals.var_a1s__blk947 * ((((locals.var_aexp1s__blk943_dn6 * locals.var_qis__blk938) - (locals.var_aexp1s__blk943 * locals.var_qis__blk938_dn6)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn6))) / (assign41290_e47187 * assign41290_e47187)), (((locals.var_a1s__blk947_dn7 * assign41290_e47187) - (locals.var_a1s__blk947 * ((((locals.var_aexp1s__blk943_dn7 * locals.var_qis__blk938) - (locals.var_aexp1s__blk943 * locals.var_qis__blk938_dn7)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn7))) / (assign41290_e47187 * assign41290_e47187)), (((locals.var_a1s__blk947_dn8 * assign41290_e47187) - (locals.var_a1s__blk947 * ((((locals.var_aexp1s__blk943_dn8 * locals.var_qis__blk938) - (locals.var_aexp1s__blk943 * locals.var_qis__blk938_dn8)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn8))) / (assign41290_e47187 * assign41290_e47187)), (((locals.var_a1s__blk947_dn9 * assign41290_e47187) - (locals.var_a1s__blk947 * ((((locals.var_aexp1s__blk943_dn9 * locals.var_qis__blk938) - (locals.var_aexp1s__blk943 * locals.var_qis__blk938_dn9)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn9))) / (assign41290_e47187 * assign41290_e47187)),)
    } else {
        (locals.var_q1s_chap__blk1064, locals.var_q1s_chap__blk1064_dn4, locals.var_q1s_chap__blk1064_dn6, locals.var_q1s_chap__blk1064_dn7, locals.var_q1s_chap__blk1064_dn8, locals.var_q1s_chap__blk1064_dn9,)
    }
};
        locals.var_q1s_chap__blk1064 = assign41290_e47190;
        locals.var_q1s_chap__blk1064_dn4 = assign41290_e47190_d_n4;
        locals.var_q1s_chap__blk1064_dn6 = assign41290_e47190_d_n6;
        locals.var_q1s_chap__blk1064_dn7 = assign41290_e47190_d_n7;
        locals.var_q1s_chap__blk1064_dn8 = assign41290_e47190_d_n8;
        locals.var_q1s_chap__blk1064_dn9 = assign41290_e47190_d_n9;

        let (assign41300_e47204, assign41300_e47204_d_n4, assign41300_e47204_d_n6, assign41300_e47204_d_n7, assign41300_e47204_d_n8, assign41300_e47204_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
        let assign41300_e47199: f64 = (locals.var_aexp1d__blk1007 / locals.var_qid__blk1003);
        let assign41300_e47201: f64 = (assign41300_e47199 - locals.var_dqsqd_dxn_qi__blk1014);
        let assign41300_e47202: f64 = (locals.var_a1d__blk1011 / assign41300_e47201);
        (assign41300_e47202, (((locals.var_a1d__blk1011_dn4 * assign41300_e47201) - (locals.var_a1d__blk1011 * ((((locals.var_aexp1d__blk1007_dn4 * locals.var_qid__blk1003) - (locals.var_aexp1d__blk1007 * locals.var_qid__blk1003_dn4)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn4))) / (assign41300_e47201 * assign41300_e47201)), (((locals.var_a1d__blk1011_dn6 * assign41300_e47201) - (locals.var_a1d__blk1011 * ((((locals.var_aexp1d__blk1007_dn6 * locals.var_qid__blk1003) - (locals.var_aexp1d__blk1007 * locals.var_qid__blk1003_dn6)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn6))) / (assign41300_e47201 * assign41300_e47201)), (((locals.var_a1d__blk1011_dn7 * assign41300_e47201) - (locals.var_a1d__blk1011 * ((((locals.var_aexp1d__blk1007_dn7 * locals.var_qid__blk1003) - (locals.var_aexp1d__blk1007 * locals.var_qid__blk1003_dn7)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn7))) / (assign41300_e47201 * assign41300_e47201)), (((locals.var_a1d__blk1011_dn8 * assign41300_e47201) - (locals.var_a1d__blk1011 * ((((locals.var_aexp1d__blk1007_dn8 * locals.var_qid__blk1003) - (locals.var_aexp1d__blk1007 * locals.var_qid__blk1003_dn8)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn8))) / (assign41300_e47201 * assign41300_e47201)), (((locals.var_a1d__blk1011_dn9 * assign41300_e47201) - (locals.var_a1d__blk1011 * ((((locals.var_aexp1d__blk1007_dn9 * locals.var_qid__blk1003) - (locals.var_aexp1d__blk1007 * locals.var_qid__blk1003_dn9)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn9))) / (assign41300_e47201 * assign41300_e47201)),)
    } else {
        (locals.var_q1d_chap__blk1065, locals.var_q1d_chap__blk1065_dn4, locals.var_q1d_chap__blk1065_dn6, locals.var_q1d_chap__blk1065_dn7, locals.var_q1d_chap__blk1065_dn8, locals.var_q1d_chap__blk1065_dn9,)
    }
};
        locals.var_q1d_chap__blk1065 = assign41300_e47204;
        locals.var_q1d_chap__blk1065_dn4 = assign41300_e47204_d_n4;
        locals.var_q1d_chap__blk1065_dn6 = assign41300_e47204_d_n6;
        locals.var_q1d_chap__blk1065_dn7 = assign41300_e47204_d_n7;
        locals.var_q1d_chap__blk1065_dn8 = assign41300_e47204_d_n8;
        locals.var_q1d_chap__blk1065_dn9 = assign41300_e47204_d_n9;

        let (assign41310_e47216, assign41310_e47216_d_n4, assign41310_e47216_d_n6, assign41310_e47216_d_n7, assign41310_e47216_d_n8, assign41310_e47216_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
        let assign41310_e47212: f64 = (locals.var_q1s_chap__blk1064 - locals.var_q1d_chap__blk1065);
        let assign41310_e47214: f64 = (assign41310_e47212 / locals.var_norm_ids__blk1063);
        (assign41310_e47214, ((((locals.var_q1s_chap__blk1064_dn4 - locals.var_q1d_chap__blk1065_dn4) * locals.var_norm_ids__blk1063) - (assign41310_e47212 * locals.var_norm_ids__blk1063_dn4)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q1s_chap__blk1064_dn6 - locals.var_q1d_chap__blk1065_dn6) * locals.var_norm_ids__blk1063) - (assign41310_e47212 * locals.var_norm_ids__blk1063_dn6)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q1s_chap__blk1064_dn7 - locals.var_q1d_chap__blk1065_dn7) * locals.var_norm_ids__blk1063) - (assign41310_e47212 * locals.var_norm_ids__blk1063_dn7)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q1s_chap__blk1064_dn8 - locals.var_q1d_chap__blk1065_dn8) * locals.var_norm_ids__blk1063) - (assign41310_e47212 * locals.var_norm_ids__blk1063_dn8)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q1s_chap__blk1064_dn9 - locals.var_q1d_chap__blk1065_dn9) * locals.var_norm_ids__blk1063) - (assign41310_e47212 * locals.var_norm_ids__blk1063_dn9)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)),)
    } else {
        (locals.var_inv_k1h1_0__blk1066, locals.var_inv_k1h1_0__blk1066_dn4, locals.var_inv_k1h1_0__blk1066_dn6, locals.var_inv_k1h1_0__blk1066_dn7, locals.var_inv_k1h1_0__blk1066_dn8, locals.var_inv_k1h1_0__blk1066_dn9,)
    }
};
        locals.var_inv_k1h1_0__blk1066 = assign41310_e47216;
        locals.var_inv_k1h1_0__blk1066_dn4 = assign41310_e47216_d_n4;
        locals.var_inv_k1h1_0__blk1066_dn6 = assign41310_e47216_d_n6;
        locals.var_inv_k1h1_0__blk1066_dn7 = assign41310_e47216_d_n7;
        locals.var_inv_k1h1_0__blk1066_dn8 = assign41310_e47216_d_n8;
        locals.var_inv_k1h1_0__blk1066_dn9 = assign41310_e47216_d_n9;

        let (assign41320_e47230, assign41320_e47230_d_n4, assign41320_e47230_d_n6, assign41320_e47230_d_n7, assign41320_e47230_d_n8, assign41320_e47230_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
        let assign41320_e47225: f64 = (locals.var_aexp2s__blk944 / locals.var_qis__blk938);
        let assign41320_e47227: f64 = (assign41320_e47225 - locals.var_dqsqs_dxn_qi__blk950);
        let assign41320_e47228: f64 = (locals.var_a2s__blk948 / assign41320_e47227);
        (assign41320_e47228, (((locals.var_a2s__blk948_dn4 * assign41320_e47227) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn4 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn4)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn4))) / (assign41320_e47227 * assign41320_e47227)), (((locals.var_a2s__blk948_dn6 * assign41320_e47227) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn6 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn6)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn6))) / (assign41320_e47227 * assign41320_e47227)), (((locals.var_a2s__blk948_dn7 * assign41320_e47227) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn7 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn7)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn7))) / (assign41320_e47227 * assign41320_e47227)), (((locals.var_a2s__blk948_dn8 * assign41320_e47227) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn8 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn8)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn8))) / (assign41320_e47227 * assign41320_e47227)), (((locals.var_a2s__blk948_dn9 * assign41320_e47227) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn9 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn9)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn9))) / (assign41320_e47227 * assign41320_e47227)),)
    } else {
        (locals.var_q2s_chap__blk1067, locals.var_q2s_chap__blk1067_dn4, locals.var_q2s_chap__blk1067_dn6, locals.var_q2s_chap__blk1067_dn7, locals.var_q2s_chap__blk1067_dn8, locals.var_q2s_chap__blk1067_dn9,)
    }
};
        locals.var_q2s_chap__blk1067 = assign41320_e47230;
        locals.var_q2s_chap__blk1067_dn4 = assign41320_e47230_d_n4;
        locals.var_q2s_chap__blk1067_dn6 = assign41320_e47230_d_n6;
        locals.var_q2s_chap__blk1067_dn7 = assign41320_e47230_d_n7;
        locals.var_q2s_chap__blk1067_dn8 = assign41320_e47230_d_n8;
        locals.var_q2s_chap__blk1067_dn9 = assign41320_e47230_d_n9;

        let (assign41330_e47244, assign41330_e47244_d_n4, assign41330_e47244_d_n6, assign41330_e47244_d_n7, assign41330_e47244_d_n8, assign41330_e47244_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
        let assign41330_e47239: f64 = (locals.var_aexp2d__blk1008 / locals.var_qid__blk1003);
        let assign41330_e47241: f64 = (assign41330_e47239 - locals.var_dqsqd_dxn_qi__blk1014);
        let assign41330_e47242: f64 = (locals.var_a2d__blk1012 / assign41330_e47241);
        (assign41330_e47242, (((locals.var_a2d__blk1012_dn4 * assign41330_e47241) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn4 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn4)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn4))) / (assign41330_e47241 * assign41330_e47241)), (((locals.var_a2d__blk1012_dn6 * assign41330_e47241) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn6 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn6)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn6))) / (assign41330_e47241 * assign41330_e47241)), (((locals.var_a2d__blk1012_dn7 * assign41330_e47241) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn7 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn7)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn7))) / (assign41330_e47241 * assign41330_e47241)), (((locals.var_a2d__blk1012_dn8 * assign41330_e47241) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn8 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn8)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn8))) / (assign41330_e47241 * assign41330_e47241)), (((locals.var_a2d__blk1012_dn9 * assign41330_e47241) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn9 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn9)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn9))) / (assign41330_e47241 * assign41330_e47241)),)
    } else {
        (locals.var_q2d_chap__blk1068, locals.var_q2d_chap__blk1068_dn4, locals.var_q2d_chap__blk1068_dn6, locals.var_q2d_chap__blk1068_dn7, locals.var_q2d_chap__blk1068_dn8, locals.var_q2d_chap__blk1068_dn9,)
    }
};
        locals.var_q2d_chap__blk1068 = assign41330_e47244;
        locals.var_q2d_chap__blk1068_dn4 = assign41330_e47244_d_n4;
        locals.var_q2d_chap__blk1068_dn6 = assign41330_e47244_d_n6;
        locals.var_q2d_chap__blk1068_dn7 = assign41330_e47244_d_n7;
        locals.var_q2d_chap__blk1068_dn8 = assign41330_e47244_d_n8;
        locals.var_q2d_chap__blk1068_dn9 = assign41330_e47244_d_n9;

        let (assign41340_e47256, assign41340_e47256_d_n4, assign41340_e47256_d_n6, assign41340_e47256_d_n7, assign41340_e47256_d_n8, assign41340_e47256_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
        let assign41340_e47252: f64 = (locals.var_q2s_chap__blk1067 - locals.var_q2d_chap__blk1068);
        let assign41340_e47254: f64 = (assign41340_e47252 / locals.var_norm_ids__blk1063);
        (assign41340_e47254, ((((locals.var_q2s_chap__blk1067_dn4 - locals.var_q2d_chap__blk1068_dn4) * locals.var_norm_ids__blk1063) - (assign41340_e47252 * locals.var_norm_ids__blk1063_dn4)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q2s_chap__blk1067_dn6 - locals.var_q2d_chap__blk1068_dn6) * locals.var_norm_ids__blk1063) - (assign41340_e47252 * locals.var_norm_ids__blk1063_dn6)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q2s_chap__blk1067_dn7 - locals.var_q2d_chap__blk1068_dn7) * locals.var_norm_ids__blk1063) - (assign41340_e47252 * locals.var_norm_ids__blk1063_dn7)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q2s_chap__blk1067_dn8 - locals.var_q2d_chap__blk1068_dn8) * locals.var_norm_ids__blk1063) - (assign41340_e47252 * locals.var_norm_ids__blk1063_dn8)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q2s_chap__blk1067_dn9 - locals.var_q2d_chap__blk1068_dn9) * locals.var_norm_ids__blk1063) - (assign41340_e47252 * locals.var_norm_ids__blk1063_dn9)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)),)
    } else {
        (locals.var_inv_k2h2_0__blk1069, locals.var_inv_k2h2_0__blk1069_dn4, locals.var_inv_k2h2_0__blk1069_dn6, locals.var_inv_k2h2_0__blk1069_dn7, locals.var_inv_k2h2_0__blk1069_dn8, locals.var_inv_k2h2_0__blk1069_dn9,)
    }
};
        locals.var_inv_k2h2_0__blk1069 = assign41340_e47256;
        locals.var_inv_k2h2_0__blk1069_dn4 = assign41340_e47256_d_n4;
        locals.var_inv_k2h2_0__blk1069_dn6 = assign41340_e47256_d_n6;
        locals.var_inv_k2h2_0__blk1069_dn7 = assign41340_e47256_d_n7;
        locals.var_inv_k2h2_0__blk1069_dn8 = assign41340_e47256_d_n8;
        locals.var_inv_k2h2_0__blk1069_dn9 = assign41340_e47256_d_n9;

        let (assign41350_e47265, assign41350_e47265_d_n4, assign41350_e47265_d_n6, assign41350_e47265_d_n7, assign41350_e47265_d_n8, assign41350_e47265_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_inv_k1h1_0__blk1066, locals.var_inv_k1h1_0__blk1066_dn4, locals.var_inv_k1h1_0__blk1066_dn6, locals.var_inv_k1h1_0__blk1066_dn7, locals.var_inv_k1h1_0__blk1066_dn8, locals.var_inv_k1h1_0__blk1066_dn9,)
    }
};
        locals.var_inv_k1h1_0__blk1066 = assign41350_e47265;
        locals.var_inv_k1h1_0__blk1066_dn4 = assign41350_e47265_d_n4;
        locals.var_inv_k1h1_0__blk1066_dn6 = assign41350_e47265_d_n6;
        locals.var_inv_k1h1_0__blk1066_dn7 = assign41350_e47265_d_n7;
        locals.var_inv_k1h1_0__blk1066_dn8 = assign41350_e47265_d_n8;
        locals.var_inv_k1h1_0__blk1066_dn9 = assign41350_e47265_d_n9;

    }

    pub(super) fn stamp_transient_block_115(
        locals: &mut StampLocals,
    ) {
        let (assign41360_e47274, assign41360_e47274_d_n4, assign41360_e47274_d_n6, assign41360_e47274_d_n7, assign41360_e47274_d_n8, assign41360_e47274_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_inv_k2h2_0__blk1069, locals.var_inv_k2h2_0__blk1069_dn4, locals.var_inv_k2h2_0__blk1069_dn6, locals.var_inv_k2h2_0__blk1069_dn7, locals.var_inv_k2h2_0__blk1069_dn8, locals.var_inv_k2h2_0__blk1069_dn9,)
    }
};
        locals.var_inv_k2h2_0__blk1069 = assign41360_e47274;
        locals.var_inv_k2h2_0__blk1069_dn4 = assign41360_e47274_d_n4;
        locals.var_inv_k2h2_0__blk1069_dn6 = assign41360_e47274_d_n6;
        locals.var_inv_k2h2_0__blk1069_dn7 = assign41360_e47274_d_n7;
        locals.var_inv_k2h2_0__blk1069_dn8 = assign41360_e47274_d_n8;
        locals.var_inv_k2h2_0__blk1069_dn9 = assign41360_e47274_d_n9;

        let (assign41370_e47290, assign41370_e47290_d_n4, assign41370_e47290_d_n6, assign41370_e47290_d_n7, assign41370_e47290_d_n8, assign41370_e47290_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41370_e47280: f64 = (-2.0);
        let assign41370_e47282: f64 = (assign41370_e47280 * locals.var_s1__blk969);
        let assign41370_e47285: f64 = (locals.var_inv_k1__blk906 / locals.var_q1chapinf__blk972);
        let assign41370_e47287: f64 = (assign41370_e47285 + locals.var_inv_dinf__blk975);
        let assign41370_e47288: f64 = (assign41370_e47282 * assign41370_e47287);
        (assign41370_e47288, (((assign41370_e47280 * locals.var_s1__blk969_dn4) * assign41370_e47287) + (assign41370_e47282 * ((((locals.var_inv_k1__blk906_dn4 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn4)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn4))), (((assign41370_e47280 * locals.var_s1__blk969_dn6) * assign41370_e47287) + (assign41370_e47282 * ((((locals.var_inv_k1__blk906_dn6 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn6)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn6))), (((assign41370_e47280 * locals.var_s1__blk969_dn7) * assign41370_e47287) + (assign41370_e47282 * ((((locals.var_inv_k1__blk906_dn7 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn7)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn7))), (((assign41370_e47280 * locals.var_s1__blk969_dn8) * assign41370_e47287) + (assign41370_e47282 * ((((locals.var_inv_k1__blk906_dn8 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn8)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn8))), (((assign41370_e47280 * locals.var_s1__blk969_dn9) * assign41370_e47287) + (assign41370_e47282 * ((((locals.var_inv_k1__blk906_dn9 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn9)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn9))),)
    } else {
        (locals.var_zeta1__blk1070, locals.var_zeta1__blk1070_dn4, locals.var_zeta1__blk1070_dn6, locals.var_zeta1__blk1070_dn7, locals.var_zeta1__blk1070_dn8, locals.var_zeta1__blk1070_dn9,)
    }
};
        locals.var_zeta1__blk1070 = assign41370_e47290;
        locals.var_zeta1__blk1070_dn4 = assign41370_e47290_d_n4;
        locals.var_zeta1__blk1070_dn6 = assign41370_e47290_d_n6;
        locals.var_zeta1__blk1070_dn7 = assign41370_e47290_d_n7;
        locals.var_zeta1__blk1070_dn8 = assign41370_e47290_d_n8;
        locals.var_zeta1__blk1070_dn9 = assign41370_e47290_d_n9;

        let (assign41380_e47306, assign41380_e47306_d_n4, assign41380_e47306_d_n6, assign41380_e47306_d_n7, assign41380_e47306_d_n8, assign41380_e47306_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41380_e47296: f64 = (-2.0);
        let assign41380_e47298: f64 = (assign41380_e47296 * locals.var_s2__blk970);
        let assign41380_e47301: f64 = (locals.var_inv_k2__blk907 / locals.var_q2chapinf__blk973);
        let assign41380_e47303: f64 = (assign41380_e47301 + locals.var_inv_dinf__blk975);
        let assign41380_e47304: f64 = (assign41380_e47298 * assign41380_e47303);
        (assign41380_e47304, (((assign41380_e47296 * locals.var_s2__blk970_dn4) * assign41380_e47303) + (assign41380_e47298 * ((((locals.var_inv_k2__blk907_dn4 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn4)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn4))), (((assign41380_e47296 * locals.var_s2__blk970_dn6) * assign41380_e47303) + (assign41380_e47298 * ((((locals.var_inv_k2__blk907_dn6 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn6)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn6))), (((assign41380_e47296 * locals.var_s2__blk970_dn7) * assign41380_e47303) + (assign41380_e47298 * ((((locals.var_inv_k2__blk907_dn7 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn7)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn7))), (((assign41380_e47296 * locals.var_s2__blk970_dn8) * assign41380_e47303) + (assign41380_e47298 * ((((locals.var_inv_k2__blk907_dn8 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn8)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn8))), (((assign41380_e47296 * locals.var_s2__blk970_dn9) * assign41380_e47303) + (assign41380_e47298 * ((((locals.var_inv_k2__blk907_dn9 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn9)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn9))),)
    } else {
        (locals.var_zeta2__blk1071, locals.var_zeta2__blk1071_dn4, locals.var_zeta2__blk1071_dn6, locals.var_zeta2__blk1071_dn7, locals.var_zeta2__blk1071_dn8, locals.var_zeta2__blk1071_dn9,)
    }
};
        locals.var_zeta2__blk1071 = assign41380_e47306;
        locals.var_zeta2__blk1071_dn4 = assign41380_e47306_d_n4;
        locals.var_zeta2__blk1071_dn6 = assign41380_e47306_d_n6;
        locals.var_zeta2__blk1071_dn7 = assign41380_e47306_d_n7;
        locals.var_zeta2__blk1071_dn8 = assign41380_e47306_d_n8;
        locals.var_zeta2__blk1071_dn9 = assign41380_e47306_d_n9;

        let (assign41390_e47317, assign41390_e47317_d_n4, assign41390_e47317_d_n6, assign41390_e47317_d_n7, assign41390_e47317_d_n8, assign41390_e47317_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41390_e47313: f64 = (locals.var_zeta2__blk1071 - locals.var_zeta1__blk1070);
        let assign41390_e47315: f64 = (assign41390_e47313 * locals.var_inv_dinf__blk975);
        (assign41390_e47315, (((locals.var_zeta2__blk1071_dn4 - locals.var_zeta1__blk1070_dn4) * locals.var_inv_dinf__blk975) + (assign41390_e47313 * locals.var_inv_dinf__blk975_dn4)), (((locals.var_zeta2__blk1071_dn6 - locals.var_zeta1__blk1070_dn6) * locals.var_inv_dinf__blk975) + (assign41390_e47313 * locals.var_inv_dinf__blk975_dn6)), (((locals.var_zeta2__blk1071_dn7 - locals.var_zeta1__blk1070_dn7) * locals.var_inv_dinf__blk975) + (assign41390_e47313 * locals.var_inv_dinf__blk975_dn7)), (((locals.var_zeta2__blk1071_dn8 - locals.var_zeta1__blk1070_dn8) * locals.var_inv_dinf__blk975) + (assign41390_e47313 * locals.var_inv_dinf__blk975_dn8)), (((locals.var_zeta2__blk1071_dn9 - locals.var_zeta1__blk1070_dn9) * locals.var_inv_dinf__blk975) + (assign41390_e47313 * locals.var_inv_dinf__blk975_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign41390_e47317;
        locals.var_temp_dn4 = assign41390_e47317_d_n4;
        locals.var_temp_dn6 = assign41390_e47317_d_n6;
        locals.var_temp_dn7 = assign41390_e47317_d_n7;
        locals.var_temp_dn8 = assign41390_e47317_d_n8;
        locals.var_temp_dn9 = assign41390_e47317_d_n9;

        let (assign41400_e47326, assign41400_e47326_d_n4, assign41400_e47326_d_n6, assign41400_e47326_d_n7, assign41400_e47326_d_n8, assign41400_e47326_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41400_e47324: f64 = (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906);
        (assign41400_e47324, ((locals.var_zeta1__blk1070_dn4 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn4)), ((locals.var_zeta1__blk1070_dn6 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn6)), ((locals.var_zeta1__blk1070_dn7 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn7)), ((locals.var_zeta1__blk1070_dn8 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn8)), ((locals.var_zeta1__blk1070_dn9 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign41400_e47326;
        locals.var_temp1_dn4 = assign41400_e47326_d_n4;
        locals.var_temp1_dn6 = assign41400_e47326_d_n6;
        locals.var_temp1_dn7 = assign41400_e47326_d_n7;
        locals.var_temp1_dn8 = assign41400_e47326_d_n8;
        locals.var_temp1_dn9 = assign41400_e47326_d_n9;

        let (assign41410_e47335, assign41410_e47335_d_n4, assign41410_e47335_d_n6, assign41410_e47335_d_n7, assign41410_e47335_d_n8, assign41410_e47335_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41410_e47333: f64 = (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907);
        (assign41410_e47333, ((locals.var_zeta2__blk1071_dn4 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn4)), ((locals.var_zeta2__blk1071_dn6 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn6)), ((locals.var_zeta2__blk1071_dn7 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn7)), ((locals.var_zeta2__blk1071_dn8 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn8)), ((locals.var_zeta2__blk1071_dn9 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign41410_e47335;
        locals.var_temp2_dn4 = assign41410_e47335_d_n4;
        locals.var_temp2_dn6 = assign41410_e47335_d_n6;
        locals.var_temp2_dn7 = assign41410_e47335_d_n7;
        locals.var_temp2_dn8 = assign41410_e47335_d_n8;
        locals.var_temp2_dn9 = assign41410_e47335_d_n9;

        let (assign41420_e47344, assign41420_e47344_d_n4, assign41420_e47344_d_n6, assign41420_e47344_d_n7, assign41420_e47344_d_n8, assign41420_e47344_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41420_e47342: f64 = (locals.var_temp1 + locals.var_temp2);
        (assign41420_e47342, (locals.var_temp1_dn4 + locals.var_temp2_dn4), (locals.var_temp1_dn6 + locals.var_temp2_dn6), (locals.var_temp1_dn7 + locals.var_temp2_dn7), (locals.var_temp1_dn8 + locals.var_temp2_dn8), (locals.var_temp1_dn9 + locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign41420_e47344;
        locals.var_temp3_dn4 = assign41420_e47344_d_n4;
        locals.var_temp3_dn6 = assign41420_e47344_d_n6;
        locals.var_temp3_dn7 = assign41420_e47344_d_n7;
        locals.var_temp3_dn8 = assign41420_e47344_d_n8;
        locals.var_temp3_dn9 = assign41420_e47344_d_n9;

        let (assign41430_e47361, assign41430_e47361_d_n4, assign41430_e47361_d_n6, assign41430_e47361_d_n7, assign41430_e47361_d_n8, assign41430_e47361_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41430_e47353: f64 = (locals.var_s1__blk969 * locals.var_inv_k1__blk906);
        let assign41430_e47356: f64 = (locals.var_s2__blk970 * locals.var_inv_k2__blk907);
        let assign41430_e47357: f64 = (assign41430_e47353 + assign41430_e47356);
        let assign41430_e47358: f64 = (2.0 * assign41430_e47357);
        let assign41430_e47359: f64 = (3.0 + assign41430_e47358);
        (assign41430_e47359, (2.0 * (((locals.var_s1__blk969_dn4 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn4)) + ((locals.var_s2__blk970_dn4 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn4)))), (2.0 * (((locals.var_s1__blk969_dn6 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn6)) + ((locals.var_s2__blk970_dn6 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn6)))), (2.0 * (((locals.var_s1__blk969_dn7 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn7)) + ((locals.var_s2__blk970_dn7 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn7)))), (2.0 * (((locals.var_s1__blk969_dn8 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn8)) + ((locals.var_s2__blk970_dn8 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn8)))), (2.0 * (((locals.var_s1__blk969_dn9 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn9)) + ((locals.var_s2__blk970_dn9 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn9)))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign41430_e47361;
        locals.var_temp4_dn4 = assign41430_e47361_d_n4;
        locals.var_temp4_dn6 = assign41430_e47361_d_n6;
        locals.var_temp4_dn7 = assign41430_e47361_d_n7;
        locals.var_temp4_dn8 = assign41430_e47361_d_n8;
        locals.var_temp4_dn9 = assign41430_e47361_d_n9;

        let (assign41440_e47376, assign41440_e47376_d_n4, assign41440_e47376_d_n6, assign41440_e47376_d_n7, assign41440_e47376_d_n8, assign41440_e47376_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41440_e47368: f64 = (locals.var_temp2 + locals.var_temp);
        let assign41440_e47371: f64 = (locals.var_temp3 / locals.var_q1chapinf__blk972);
        let assign41440_e47372: f64 = (assign41440_e47368 - assign41440_e47371);
        let assign41440_e47374: f64 = (assign41440_e47372 / locals.var_temp4);
        (assign41440_e47374, (((((locals.var_temp2_dn4 + locals.var_temp_dn4) - (((locals.var_temp3_dn4 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn4)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41440_e47372 * locals.var_temp4_dn4)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn6 + locals.var_temp_dn6) - (((locals.var_temp3_dn6 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn6)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41440_e47372 * locals.var_temp4_dn6)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn7 + locals.var_temp_dn7) - (((locals.var_temp3_dn7 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn7)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41440_e47372 * locals.var_temp4_dn7)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn8 + locals.var_temp_dn8) - (((locals.var_temp3_dn8 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn8)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41440_e47372 * locals.var_temp4_dn8)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn9 + locals.var_temp_dn9) - (((locals.var_temp3_dn9 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn9)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41440_e47372 * locals.var_temp4_dn9)) / (locals.var_temp4 * locals.var_temp4)),)
    } else {
        (locals.var_ksi1__blk1072, locals.var_ksi1__blk1072_dn4, locals.var_ksi1__blk1072_dn6, locals.var_ksi1__blk1072_dn7, locals.var_ksi1__blk1072_dn8, locals.var_ksi1__blk1072_dn9,)
    }
};
        locals.var_ksi1__blk1072 = assign41440_e47376;
        locals.var_ksi1__blk1072_dn4 = assign41440_e47376_d_n4;
        locals.var_ksi1__blk1072_dn6 = assign41440_e47376_d_n6;
        locals.var_ksi1__blk1072_dn7 = assign41440_e47376_d_n7;
        locals.var_ksi1__blk1072_dn8 = assign41440_e47376_d_n8;
        locals.var_ksi1__blk1072_dn9 = assign41440_e47376_d_n9;

        let (assign41450_e47391, assign41450_e47391_d_n4, assign41450_e47391_d_n6, assign41450_e47391_d_n7, assign41450_e47391_d_n8, assign41450_e47391_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41450_e47383: f64 = (locals.var_temp1 - locals.var_temp);
        let assign41450_e47386: f64 = (locals.var_temp3 / locals.var_q2chapinf__blk973);
        let assign41450_e47387: f64 = (assign41450_e47383 - assign41450_e47386);
        let assign41450_e47389: f64 = (assign41450_e47387 / locals.var_temp4);
        (assign41450_e47389, (((((locals.var_temp1_dn4 - locals.var_temp_dn4) - (((locals.var_temp3_dn4 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn4)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41450_e47387 * locals.var_temp4_dn4)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn6 - locals.var_temp_dn6) - (((locals.var_temp3_dn6 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn6)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41450_e47387 * locals.var_temp4_dn6)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn7 - locals.var_temp_dn7) - (((locals.var_temp3_dn7 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn7)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41450_e47387 * locals.var_temp4_dn7)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn8 - locals.var_temp_dn8) - (((locals.var_temp3_dn8 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn8)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41450_e47387 * locals.var_temp4_dn8)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn9 - locals.var_temp_dn9) - (((locals.var_temp3_dn9 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn9)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41450_e47387 * locals.var_temp4_dn9)) / (locals.var_temp4 * locals.var_temp4)),)
    } else {
        (locals.var_ksi2__blk1073, locals.var_ksi2__blk1073_dn4, locals.var_ksi2__blk1073_dn6, locals.var_ksi2__blk1073_dn7, locals.var_ksi2__blk1073_dn8, locals.var_ksi2__blk1073_dn9,)
    }
};
        locals.var_ksi2__blk1073 = assign41450_e47391;
        locals.var_ksi2__blk1073_dn4 = assign41450_e47391_d_n4;
        locals.var_ksi2__blk1073_dn6 = assign41450_e47391_d_n6;
        locals.var_ksi2__blk1073_dn7 = assign41450_e47391_d_n7;
        locals.var_ksi2__blk1073_dn8 = assign41450_e47391_d_n8;
        locals.var_ksi2__blk1073_dn9 = assign41450_e47391_d_n9;

        let (assign41460_e47405, assign41460_e47405_d_n4, assign41460_e47405_d_n6, assign41460_e47405_d_n7, assign41460_e47405_d_n8, assign41460_e47405_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41460_e47397: f64 = (-locals.var_q1chapinf__blk972);
        let assign41460_e47400: f64 = (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972);
        let assign41460_e47402: f64 = (assign41460_e47400 + locals.var_inv_dinf__blk975);
        let assign41460_e47403: f64 = (assign41460_e47397 * assign41460_e47402);
        (assign41460_e47403, (((-locals.var_q1chapinf__blk972_dn4) * assign41460_e47402) + (assign41460_e47397 * (((locals.var_ksi1__blk1072_dn4 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn4)) + locals.var_inv_dinf__blk975_dn4))), (((-locals.var_q1chapinf__blk972_dn6) * assign41460_e47402) + (assign41460_e47397 * (((locals.var_ksi1__blk1072_dn6 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn6)) + locals.var_inv_dinf__blk975_dn6))), (((-locals.var_q1chapinf__blk972_dn7) * assign41460_e47402) + (assign41460_e47397 * (((locals.var_ksi1__blk1072_dn7 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn7)) + locals.var_inv_dinf__blk975_dn7))), (((-locals.var_q1chapinf__blk972_dn8) * assign41460_e47402) + (assign41460_e47397 * (((locals.var_ksi1__blk1072_dn8 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn8)) + locals.var_inv_dinf__blk975_dn8))), (((-locals.var_q1chapinf__blk972_dn9) * assign41460_e47402) + (assign41460_e47397 * (((locals.var_ksi1__blk1072_dn9 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn9)) + locals.var_inv_dinf__blk975_dn9))),)
    } else {
        (locals.var_inv_k1h1_0__blk1066, locals.var_inv_k1h1_0__blk1066_dn4, locals.var_inv_k1h1_0__blk1066_dn6, locals.var_inv_k1h1_0__blk1066_dn7, locals.var_inv_k1h1_0__blk1066_dn8, locals.var_inv_k1h1_0__blk1066_dn9,)
    }
};
        locals.var_inv_k1h1_0__blk1066 = assign41460_e47405;
        locals.var_inv_k1h1_0__blk1066_dn4 = assign41460_e47405_d_n4;
        locals.var_inv_k1h1_0__blk1066_dn6 = assign41460_e47405_d_n6;
        locals.var_inv_k1h1_0__blk1066_dn7 = assign41460_e47405_d_n7;
        locals.var_inv_k1h1_0__blk1066_dn8 = assign41460_e47405_d_n8;
        locals.var_inv_k1h1_0__blk1066_dn9 = assign41460_e47405_d_n9;

        let (assign41470_e47419, assign41470_e47419_d_n4, assign41470_e47419_d_n6, assign41470_e47419_d_n7, assign41470_e47419_d_n8, assign41470_e47419_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41470_e47411: f64 = (-locals.var_q2chapinf__blk973);
        let assign41470_e47414: f64 = (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973);
        let assign41470_e47416: f64 = (assign41470_e47414 + locals.var_inv_dinf__blk975);
        let assign41470_e47417: f64 = (assign41470_e47411 * assign41470_e47416);
        (assign41470_e47417, (((-locals.var_q2chapinf__blk973_dn4) * assign41470_e47416) + (assign41470_e47411 * (((locals.var_ksi2__blk1073_dn4 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn4)) + locals.var_inv_dinf__blk975_dn4))), (((-locals.var_q2chapinf__blk973_dn6) * assign41470_e47416) + (assign41470_e47411 * (((locals.var_ksi2__blk1073_dn6 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn6)) + locals.var_inv_dinf__blk975_dn6))), (((-locals.var_q2chapinf__blk973_dn7) * assign41470_e47416) + (assign41470_e47411 * (((locals.var_ksi2__blk1073_dn7 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn7)) + locals.var_inv_dinf__blk975_dn7))), (((-locals.var_q2chapinf__blk973_dn8) * assign41470_e47416) + (assign41470_e47411 * (((locals.var_ksi2__blk1073_dn8 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn8)) + locals.var_inv_dinf__blk975_dn8))), (((-locals.var_q2chapinf__blk973_dn9) * assign41470_e47416) + (assign41470_e47411 * (((locals.var_ksi2__blk1073_dn9 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn9)) + locals.var_inv_dinf__blk975_dn9))),)
    } else {
        (locals.var_inv_k2h2_0__blk1069, locals.var_inv_k2h2_0__blk1069_dn4, locals.var_inv_k2h2_0__blk1069_dn6, locals.var_inv_k2h2_0__blk1069_dn7, locals.var_inv_k2h2_0__blk1069_dn8, locals.var_inv_k2h2_0__blk1069_dn9,)
    }
};
        locals.var_inv_k2h2_0__blk1069 = assign41470_e47419;
        locals.var_inv_k2h2_0__blk1069_dn4 = assign41470_e47419_d_n4;
        locals.var_inv_k2h2_0__blk1069_dn6 = assign41470_e47419_d_n6;
        locals.var_inv_k2h2_0__blk1069_dn7 = assign41470_e47419_d_n7;
        locals.var_inv_k2h2_0__blk1069_dn8 = assign41470_e47419_d_n8;
        locals.var_inv_k2h2_0__blk1069_dn9 = assign41470_e47419_d_n9;

        let (assign41480_e47425, assign41480_e47425_d_n4, assign41480_e47425_d_n6, assign41480_e47425_d_n7, assign41480_e47425_d_n8, assign41480_e47425_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign41480_e47423: f64 = (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053);
        (assign41480_e47423, ((locals.var_inv_k1h1_0__blk1066_dn4 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn4)), ((locals.var_inv_k1h1_0__blk1066_dn6 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn6)), ((locals.var_inv_k1h1_0__blk1066_dn7 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn7)), ((locals.var_inv_k1h1_0__blk1066_dn8 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn8)), ((locals.var_inv_k1h1_0__blk1066_dn9 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn9)),)
    } else {
        (locals.var_inv_k1h1__blk1074, locals.var_inv_k1h1__blk1074_dn4, locals.var_inv_k1h1__blk1074_dn6, locals.var_inv_k1h1__blk1074_dn7, locals.var_inv_k1h1__blk1074_dn8, locals.var_inv_k1h1__blk1074_dn9,)
    }
};
        locals.var_inv_k1h1__blk1074 = assign41480_e47425;
        locals.var_inv_k1h1__blk1074_dn4 = assign41480_e47425_d_n4;
        locals.var_inv_k1h1__blk1074_dn6 = assign41480_e47425_d_n6;
        locals.var_inv_k1h1__blk1074_dn7 = assign41480_e47425_d_n7;
        locals.var_inv_k1h1__blk1074_dn8 = assign41480_e47425_d_n8;
        locals.var_inv_k1h1__blk1074_dn9 = assign41480_e47425_d_n9;

        let (assign41490_e47431, assign41490_e47431_d_n4, assign41490_e47431_d_n6, assign41490_e47431_d_n7, assign41490_e47431_d_n8, assign41490_e47431_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign41490_e47429: f64 = (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053);
        (assign41490_e47429, ((locals.var_inv_k2h2_0__blk1069_dn4 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn4)), ((locals.var_inv_k2h2_0__blk1069_dn6 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn6)), ((locals.var_inv_k2h2_0__blk1069_dn7 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn7)), ((locals.var_inv_k2h2_0__blk1069_dn8 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn8)), ((locals.var_inv_k2h2_0__blk1069_dn9 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn9)),)
    } else {
        (locals.var_inv_k2h2__blk1075, locals.var_inv_k2h2__blk1075_dn4, locals.var_inv_k2h2__blk1075_dn6, locals.var_inv_k2h2__blk1075_dn7, locals.var_inv_k2h2__blk1075_dn8, locals.var_inv_k2h2__blk1075_dn9,)
    }
};
        locals.var_inv_k2h2__blk1075 = assign41490_e47431;
        locals.var_inv_k2h2__blk1075_dn4 = assign41490_e47431_d_n4;
        locals.var_inv_k2h2__blk1075_dn6 = assign41490_e47431_d_n6;
        locals.var_inv_k2h2__blk1075_dn7 = assign41490_e47431_d_n7;
        locals.var_inv_k2h2__blk1075_dn8 = assign41490_e47431_d_n8;
        locals.var_inv_k2h2__blk1075_dn9 = assign41490_e47431_d_n9;

        let (assign41500_e47439, assign41500_e47439_d_n4, assign41500_e47439_d_n6, assign41500_e47439_d_n7, assign41500_e47439_d_n8, assign41500_e47439_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign41500_e47436: f64 = (locals.var_k1q1d__blk1004 - locals.var_k1q1s__blk939);
        let assign41500_e47437: f64 = (0.5 * assign41500_e47436);
        (assign41500_e47437, (0.5 * (locals.var_k1q1d__blk1004_dn4 - locals.var_k1q1s__blk939_dn4)), (0.5 * (locals.var_k1q1d__blk1004_dn6 - locals.var_k1q1s__blk939_dn6)), (0.5 * (locals.var_k1q1d__blk1004_dn7 - locals.var_k1q1s__blk939_dn7)), (0.5 * (locals.var_k1q1d__blk1004_dn8 - locals.var_k1q1s__blk939_dn8)), (0.5 * (locals.var_k1q1d__blk1004_dn9 - locals.var_k1q1s__blk939_dn9)),)
    } else {
        (locals.var_delta_k1q1__blk1076, locals.var_delta_k1q1__blk1076_dn4, locals.var_delta_k1q1__blk1076_dn6, locals.var_delta_k1q1__blk1076_dn7, locals.var_delta_k1q1__blk1076_dn8, locals.var_delta_k1q1__blk1076_dn9,)
    }
};
        locals.var_delta_k1q1__blk1076 = assign41500_e47439;
        locals.var_delta_k1q1__blk1076_dn4 = assign41500_e47439_d_n4;
        locals.var_delta_k1q1__blk1076_dn6 = assign41500_e47439_d_n6;
        locals.var_delta_k1q1__blk1076_dn7 = assign41500_e47439_d_n7;
        locals.var_delta_k1q1__blk1076_dn8 = assign41500_e47439_d_n8;
        locals.var_delta_k1q1__blk1076_dn9 = assign41500_e47439_d_n9;

        let (assign41510_e47447, assign41510_e47447_d_n4, assign41510_e47447_d_n6, assign41510_e47447_d_n7, assign41510_e47447_d_n8, assign41510_e47447_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign41510_e47444: f64 = (locals.var_k2q2d__blk1005 - locals.var_k2q2s__blk940);
        let assign41510_e47445: f64 = (0.5 * assign41510_e47444);
        (assign41510_e47445, (0.5 * (locals.var_k2q2d__blk1005_dn4 - locals.var_k2q2s__blk940_dn4)), (0.5 * (locals.var_k2q2d__blk1005_dn6 - locals.var_k2q2s__blk940_dn6)), (0.5 * (locals.var_k2q2d__blk1005_dn7 - locals.var_k2q2s__blk940_dn7)), (0.5 * (locals.var_k2q2d__blk1005_dn8 - locals.var_k2q2s__blk940_dn8)), (0.5 * (locals.var_k2q2d__blk1005_dn9 - locals.var_k2q2s__blk940_dn9)),)
    } else {
        (locals.var_delta_k2q2__blk1077, locals.var_delta_k2q2__blk1077_dn4, locals.var_delta_k2q2__blk1077_dn6, locals.var_delta_k2q2__blk1077_dn7, locals.var_delta_k2q2__blk1077_dn8, locals.var_delta_k2q2__blk1077_dn9,)
    }
};
        locals.var_delta_k2q2__blk1077 = assign41510_e47447;
        locals.var_delta_k2q2__blk1077_dn4 = assign41510_e47447_d_n4;
        locals.var_delta_k2q2__blk1077_dn6 = assign41510_e47447_d_n6;
        locals.var_delta_k2q2__blk1077_dn7 = assign41510_e47447_d_n7;
        locals.var_delta_k2q2__blk1077_dn8 = assign41510_e47447_d_n8;
        locals.var_delta_k2q2__blk1077_dn9 = assign41510_e47447_d_n9;

        let (assign41520_e47453, assign41520_e47453_d_n4, assign41520_e47453_d_n6, assign41520_e47453_d_n7, assign41520_e47453_d_n8, assign41520_e47453_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign41520_e47451: f64 = (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074);
        (assign41520_e47451, ((locals.var_delta_k1q1__blk1076_dn4 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn4)), ((locals.var_delta_k1q1__blk1076_dn6 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn6)), ((locals.var_delta_k1q1__blk1076_dn7 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn7)), ((locals.var_delta_k1q1__blk1076_dn8 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn8)), ((locals.var_delta_k1q1__blk1076_dn9 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn9)),)
    } else {
        (locals.var_prod1__blk1078, locals.var_prod1__blk1078_dn4, locals.var_prod1__blk1078_dn6, locals.var_prod1__blk1078_dn7, locals.var_prod1__blk1078_dn8, locals.var_prod1__blk1078_dn9,)
    }
};
        locals.var_prod1__blk1078 = assign41520_e47453;
        locals.var_prod1__blk1078_dn4 = assign41520_e47453_d_n4;
        locals.var_prod1__blk1078_dn6 = assign41520_e47453_d_n6;
        locals.var_prod1__blk1078_dn7 = assign41520_e47453_d_n7;
        locals.var_prod1__blk1078_dn8 = assign41520_e47453_d_n8;
        locals.var_prod1__blk1078_dn9 = assign41520_e47453_d_n9;

        let (assign41530_e47459, assign41530_e47459_d_n4, assign41530_e47459_d_n6, assign41530_e47459_d_n7, assign41530_e47459_d_n8, assign41530_e47459_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign41530_e47457: f64 = (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075);
        (assign41530_e47457, ((locals.var_delta_k2q2__blk1077_dn4 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn4)), ((locals.var_delta_k2q2__blk1077_dn6 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn6)), ((locals.var_delta_k2q2__blk1077_dn7 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn7)), ((locals.var_delta_k2q2__blk1077_dn8 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn8)), ((locals.var_delta_k2q2__blk1077_dn9 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn9)),)
    } else {
        (locals.var_prod2__blk1079, locals.var_prod2__blk1079_dn4, locals.var_prod2__blk1079_dn6, locals.var_prod2__blk1079_dn7, locals.var_prod2__blk1079_dn8, locals.var_prod2__blk1079_dn9,)
    }
};
        locals.var_prod2__blk1079 = assign41530_e47459;
        locals.var_prod2__blk1079_dn4 = assign41530_e47459_d_n4;
        locals.var_prod2__blk1079_dn6 = assign41530_e47459_d_n6;
        locals.var_prod2__blk1079_dn7 = assign41530_e47459_d_n7;
        locals.var_prod2__blk1079_dn8 = assign41530_e47459_d_n8;
        locals.var_prod2__blk1079_dn9 = assign41530_e47459_d_n9;

        let (assign41540_e47463, assign41540_e47463_d_n4, assign41540_e47463_d_n6, assign41540_e47463_d_n7, assign41540_e47463_d_n8, assign41540_e47463_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_xg20shift__blk900, locals.var_xg20shift__blk900_dn4, locals.var_xg20shift__blk900_dn6, locals.var_xg20shift__blk900_dn7, locals.var_xg20shift__blk900_dn8, locals.var_xg20shift__blk900_dn9,)
    } else {
        (locals.var_xg20shift_ac, locals.var_xg20shift_ac_dn4, locals.var_xg20shift_ac_dn6, locals.var_xg20shift_ac_dn7, locals.var_xg20shift_ac_dn8, locals.var_xg20shift_ac_dn9,)
    }
};
        locals.var_xg20shift_ac = assign41540_e47463;
        locals.var_xg20shift_ac_dn4 = assign41540_e47463_d_n4;
        locals.var_xg20shift_ac_dn6 = assign41540_e47463_d_n6;
        locals.var_xg20shift_ac_dn7 = assign41540_e47463_d_n7;
        locals.var_xg20shift_ac_dn8 = assign41540_e47463_d_n8;
        locals.var_xg20shift_ac_dn9 = assign41540_e47463_d_n9;

        let (assign41550_e47467, assign41550_e47467_d_n4, assign41550_e47467_d_n6, assign41550_e47467_d_n7, assign41550_e47467_d_n8, assign41550_e47467_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_diff_min__blk904, locals.var_diff_min__blk904_dn4, locals.var_diff_min__blk904_dn6, locals.var_diff_min__blk904_dn7, locals.var_diff_min__blk904_dn8, locals.var_diff_min__blk904_dn9,)
    } else {
        (locals.var_diff_min_ac, locals.var_diff_min_ac_dn4, locals.var_diff_min_ac_dn6, locals.var_diff_min_ac_dn7, locals.var_diff_min_ac_dn8, locals.var_diff_min_ac_dn9,)
    }
};
        locals.var_diff_min_ac = assign41550_e47467;
        locals.var_diff_min_ac_dn4 = assign41550_e47467_d_n4;
        locals.var_diff_min_ac_dn6 = assign41550_e47467_d_n6;
        locals.var_diff_min_ac_dn7 = assign41550_e47467_d_n7;
        locals.var_diff_min_ac_dn8 = assign41550_e47467_d_n8;
        locals.var_diff_min_ac_dn9 = assign41550_e47467_d_n9;

        let (assign41560_e47471, assign41560_e47471_d_n4, assign41560_e47471_d_n6, assign41560_e47471_d_n7, assign41560_e47471_d_n8, assign41560_e47471_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_a0__blk905, locals.var_a0__blk905_dn4, locals.var_a0__blk905_dn6, locals.var_a0__blk905_dn7, locals.var_a0__blk905_dn8, locals.var_a0__blk905_dn9,)
    } else {
        (locals.var_a0_ac, locals.var_a0_ac_dn4, locals.var_a0_ac_dn6, locals.var_a0_ac_dn7, locals.var_a0_ac_dn8, locals.var_a0_ac_dn9,)
    }
};
        locals.var_a0_ac = assign41560_e47471;
        locals.var_a0_ac_dn4 = assign41560_e47471_d_n4;
        locals.var_a0_ac_dn6 = assign41560_e47471_d_n6;
        locals.var_a0_ac_dn7 = assign41560_e47471_d_n7;
        locals.var_a0_ac_dn8 = assign41560_e47471_d_n8;
        locals.var_a0_ac_dn9 = assign41560_e47471_d_n9;

        let (assign41570_e47475, assign41570_e47475_d_n4, assign41570_e47475_d_n6, assign41570_e47475_d_n7, assign41570_e47475_d_n8, assign41570_e47475_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_inv_k1__blk906, locals.var_inv_k1__blk906_dn4, locals.var_inv_k1__blk906_dn6, locals.var_inv_k1__blk906_dn7, locals.var_inv_k1__blk906_dn8, locals.var_inv_k1__blk906_dn9,)
    } else {
        (locals.var_inv_k1_ac, locals.var_inv_k1_ac_dn4, locals.var_inv_k1_ac_dn6, locals.var_inv_k1_ac_dn7, locals.var_inv_k1_ac_dn8, locals.var_inv_k1_ac_dn9,)
    }
};
        locals.var_inv_k1_ac = assign41570_e47475;
        locals.var_inv_k1_ac_dn4 = assign41570_e47475_d_n4;
        locals.var_inv_k1_ac_dn6 = assign41570_e47475_d_n6;
        locals.var_inv_k1_ac_dn7 = assign41570_e47475_d_n7;
        locals.var_inv_k1_ac_dn8 = assign41570_e47475_d_n8;
        locals.var_inv_k1_ac_dn9 = assign41570_e47475_d_n9;

        let (assign41580_e47479, assign41580_e47479_d_n4, assign41580_e47479_d_n6, assign41580_e47479_d_n7, assign41580_e47479_d_n8, assign41580_e47479_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_inv_k2__blk907, locals.var_inv_k2__blk907_dn4, locals.var_inv_k2__blk907_dn6, locals.var_inv_k2__blk907_dn7, locals.var_inv_k2__blk907_dn8, locals.var_inv_k2__blk907_dn9,)
    } else {
        (locals.var_inv_k2_ac, locals.var_inv_k2_ac_dn4, locals.var_inv_k2_ac_dn6, locals.var_inv_k2_ac_dn7, locals.var_inv_k2_ac_dn8, locals.var_inv_k2_ac_dn9,)
    }
};
        locals.var_inv_k2_ac = assign41580_e47479;
        locals.var_inv_k2_ac_dn4 = assign41580_e47479_d_n4;
        locals.var_inv_k2_ac_dn6 = assign41580_e47479_d_n6;
        locals.var_inv_k2_ac_dn7 = assign41580_e47479_d_n7;
        locals.var_inv_k2_ac_dn8 = assign41580_e47479_d_n8;
        locals.var_inv_k2_ac_dn9 = assign41580_e47479_d_n9;

        let (assign41590_e47483, assign41590_e47483_d_n4, assign41590_e47483_d_n6, assign41590_e47483_d_n7, assign41590_e47483_d_n8, assign41590_e47483_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_keq__blk934, locals.var_keq__blk934_dn4, locals.var_keq__blk934_dn6, locals.var_keq__blk934_dn7, locals.var_keq__blk934_dn8, locals.var_keq__blk934_dn9,)
    } else {
        (locals.var_keq_ac, locals.var_keq_ac_dn4, locals.var_keq_ac_dn6, locals.var_keq_ac_dn7, locals.var_keq_ac_dn8, locals.var_keq_ac_dn9,)
    }
};
        locals.var_keq_ac = assign41590_e47483;
        locals.var_keq_ac_dn4 = assign41590_e47483_d_n4;
        locals.var_keq_ac_dn6 = assign41590_e47483_d_n6;
        locals.var_keq_ac_dn7 = assign41590_e47483_d_n7;
        locals.var_keq_ac_dn8 = assign41590_e47483_d_n8;
        locals.var_keq_ac_dn9 = assign41590_e47483_d_n9;

        let (assign41600_e47487, assign41600_e47487_d_n4, assign41600_e47487_d_n6, assign41600_e47487_d_n7, assign41600_e47487_d_n8, assign41600_e47487_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_dx_wi__blk935, locals.var_dx_wi__blk935_dn4, locals.var_dx_wi__blk935_dn6, locals.var_dx_wi__blk935_dn7, locals.var_dx_wi__blk935_dn8, locals.var_dx_wi__blk935_dn9,)
    } else {
        (locals.var_dx_wi_ac, locals.var_dx_wi_ac_dn4, locals.var_dx_wi_ac_dn6, locals.var_dx_wi_ac_dn7, locals.var_dx_wi_ac_dn8, locals.var_dx_wi_ac_dn9,)
    }
};
        locals.var_dx_wi_ac = assign41600_e47487;
        locals.var_dx_wi_ac_dn4 = assign41600_e47487_d_n4;
        locals.var_dx_wi_ac_dn6 = assign41600_e47487_d_n6;
        locals.var_dx_wi_ac_dn7 = assign41600_e47487_d_n7;
        locals.var_dx_wi_ac_dn8 = assign41600_e47487_d_n8;
        locals.var_dx_wi_ac_dn9 = assign41600_e47487_d_n9;

        let (assign41610_e47491, assign41610_e47491_d_n4, assign41610_e47491_d_n6, assign41610_e47491_d_n7, assign41610_e47491_d_n8, assign41610_e47491_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_csiprime__blk919, locals.var_csiprime__blk919_dn4, locals.var_csiprime__blk919_dn6, locals.var_csiprime__blk919_dn7, locals.var_csiprime__blk919_dn8, locals.var_csiprime__blk919_dn9,)
    } else {
        (locals.var_csiprime_ac, locals.var_csiprime_ac_dn4, locals.var_csiprime_ac_dn6, locals.var_csiprime_ac_dn7, locals.var_csiprime_ac_dn8, locals.var_csiprime_ac_dn9,)
    }
};
        locals.var_csiprime_ac = assign41610_e47491;
        locals.var_csiprime_ac_dn4 = assign41610_e47491_d_n4;
        locals.var_csiprime_ac_dn6 = assign41610_e47491_d_n6;
        locals.var_csiprime_ac_dn7 = assign41610_e47491_d_n7;
        locals.var_csiprime_ac_dn8 = assign41610_e47491_d_n8;
        locals.var_csiprime_ac_dn9 = assign41610_e47491_d_n9;

        let (assign41620_e47495, assign41620_e47495_d_n4, assign41620_e47495_d_n6, assign41620_e47495_d_n7, assign41620_e47495_d_n8, assign41620_e47495_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_dx_wi_1d__blk918, locals.var_dx_wi_1d__blk918_dn4, locals.var_dx_wi_1d__blk918_dn6, locals.var_dx_wi_1d__blk918_dn7, locals.var_dx_wi_1d__blk918_dn8, locals.var_dx_wi_1d__blk918_dn9,)
    } else {
        (locals.var_dx_wi_1d_ac, locals.var_dx_wi_1d_ac_dn4, locals.var_dx_wi_1d_ac_dn6, locals.var_dx_wi_1d_ac_dn7, locals.var_dx_wi_1d_ac_dn8, locals.var_dx_wi_1d_ac_dn9,)
    }
};
        locals.var_dx_wi_1d_ac = assign41620_e47495;
        locals.var_dx_wi_1d_ac_dn4 = assign41620_e47495_d_n4;
        locals.var_dx_wi_1d_ac_dn6 = assign41620_e47495_d_n6;
        locals.var_dx_wi_1d_ac_dn7 = assign41620_e47495_d_n7;
        locals.var_dx_wi_1d_ac_dn8 = assign41620_e47495_d_n8;
        locals.var_dx_wi_1d_ac_dn9 = assign41620_e47495_d_n9;

        let (assign41630_e47499, assign41630_e47499_d_n4, assign41630_e47499_d_n6, assign41630_e47499_d_n7, assign41630_e47499_d_n8, assign41630_e47499_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_dleff__blk922, locals.var_dleff__blk922_dn4, locals.var_dleff__blk922_dn6, locals.var_dleff__blk922_dn7, locals.var_dleff__blk922_dn8, locals.var_dleff__blk922_dn9,)
    } else {
        (locals.var_dleff_ac, locals.var_dleff_ac_dn4, locals.var_dleff_ac_dn6, locals.var_dleff_ac_dn7, locals.var_dleff_ac_dn8, locals.var_dleff_ac_dn9,)
    }
};
        locals.var_dleff_ac = assign41630_e47499;
        locals.var_dleff_ac_dn4 = assign41630_e47499_d_n4;
        locals.var_dleff_ac_dn6 = assign41630_e47499_d_n6;
        locals.var_dleff_ac_dn7 = assign41630_e47499_d_n7;
        locals.var_dleff_ac_dn8 = assign41630_e47499_d_n8;
        locals.var_dleff_ac_dn9 = assign41630_e47499_d_n9;

        let (assign41640_e47503, assign41640_e47503_d_n4, assign41640_e47503_d_n6, assign41640_e47503_d_n7, assign41640_e47503_d_n8, assign41640_e47503_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_xedge__blk923, locals.var_xedge__blk923_dn4, locals.var_xedge__blk923_dn6, locals.var_xedge__blk923_dn7, locals.var_xedge__blk923_dn8, locals.var_xedge__blk923_dn9,)
    } else {
        (locals.var_xedge_ac, locals.var_xedge_ac_dn4, locals.var_xedge_ac_dn6, locals.var_xedge_ac_dn7, locals.var_xedge_ac_dn8, locals.var_xedge_ac_dn9,)
    }
};
        locals.var_xedge_ac = assign41640_e47503;
        locals.var_xedge_ac_dn4 = assign41640_e47503_d_n4;
        locals.var_xedge_ac_dn6 = assign41640_e47503_d_n6;
        locals.var_xedge_ac_dn7 = assign41640_e47503_d_n7;
        locals.var_xedge_ac_dn8 = assign41640_e47503_d_n8;
        locals.var_xedge_ac_dn9 = assign41640_e47503_d_n9;

        let (assign41650_e47507, assign41650_e47507_d_n4, assign41650_e47507_d_n6, assign41650_e47507_d_n7, assign41650_e47507_d_n8, assign41650_e47507_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_sce1__blk924, locals.var_sce1__blk924_dn4, locals.var_sce1__blk924_dn6, locals.var_sce1__blk924_dn7, locals.var_sce1__blk924_dn8, locals.var_sce1__blk924_dn9,)
    } else {
        (locals.var_sce1_ac, locals.var_sce1_ac_dn4, locals.var_sce1_ac_dn6, locals.var_sce1_ac_dn7, locals.var_sce1_ac_dn8, locals.var_sce1_ac_dn9,)
    }
};
        locals.var_sce1_ac = assign41650_e47507;
        locals.var_sce1_ac_dn4 = assign41650_e47507_d_n4;
        locals.var_sce1_ac_dn6 = assign41650_e47507_d_n6;
        locals.var_sce1_ac_dn7 = assign41650_e47507_d_n7;
        locals.var_sce1_ac_dn8 = assign41650_e47507_d_n8;
        locals.var_sce1_ac_dn9 = assign41650_e47507_d_n9;

        let (assign41660_e47511, assign41660_e47511_d_n4, assign41660_e47511_d_n6, assign41660_e47511_d_n7, assign41660_e47511_d_n8, assign41660_e47511_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_sce2__blk925, locals.var_sce2__blk925_dn4, locals.var_sce2__blk925_dn6, locals.var_sce2__blk925_dn7, locals.var_sce2__blk925_dn8, locals.var_sce2__blk925_dn9,)
    } else {
        (locals.var_sce2_ac, locals.var_sce2_ac_dn4, locals.var_sce2_ac_dn6, locals.var_sce2_ac_dn7, locals.var_sce2_ac_dn8, locals.var_sce2_ac_dn9,)
    }
};
        locals.var_sce2_ac = assign41660_e47511;
        locals.var_sce2_ac_dn4 = assign41660_e47511_d_n4;
        locals.var_sce2_ac_dn6 = assign41660_e47511_d_n6;
        locals.var_sce2_ac_dn7 = assign41660_e47511_d_n7;
        locals.var_sce2_ac_dn8 = assign41660_e47511_d_n8;
        locals.var_sce2_ac_dn9 = assign41660_e47511_d_n9;

        let (assign41670_e47515, assign41670_e47515_d_n4, assign41670_e47515_d_n6, assign41670_e47515_d_n7, assign41670_e47515_d_n8, assign41670_e47515_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_dxg1_dibl__blk926, locals.var_dxg1_dibl__blk926_dn4, locals.var_dxg1_dibl__blk926_dn6, locals.var_dxg1_dibl__blk926_dn7, locals.var_dxg1_dibl__blk926_dn8, locals.var_dxg1_dibl__blk926_dn9,)
    } else {
        (locals.var_dxg1_dibl_ac, locals.var_dxg1_dibl_ac_dn4, locals.var_dxg1_dibl_ac_dn6, locals.var_dxg1_dibl_ac_dn7, locals.var_dxg1_dibl_ac_dn8, locals.var_dxg1_dibl_ac_dn9,)
    }
};
        locals.var_dxg1_dibl_ac = assign41670_e47515;
        locals.var_dxg1_dibl_ac_dn4 = assign41670_e47515_d_n4;
        locals.var_dxg1_dibl_ac_dn6 = assign41670_e47515_d_n6;
        locals.var_dxg1_dibl_ac_dn7 = assign41670_e47515_d_n7;
        locals.var_dxg1_dibl_ac_dn8 = assign41670_e47515_d_n8;
        locals.var_dxg1_dibl_ac_dn9 = assign41670_e47515_d_n9;

        let (assign41680_e47519, assign41680_e47519_d_n4, assign41680_e47519_d_n6, assign41680_e47519_d_n7, assign41680_e47519_d_n8, assign41680_e47519_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_xg2__blk929, locals.var_xg2__blk929_dn4, locals.var_xg2__blk929_dn6, locals.var_xg2__blk929_dn7, locals.var_xg2__blk929_dn8, locals.var_xg2__blk929_dn9,)
    } else {
        (locals.var_xg2_ac, locals.var_xg2_ac_dn4, locals.var_xg2_ac_dn6, locals.var_xg2_ac_dn7, locals.var_xg2_ac_dn8, locals.var_xg2_ac_dn9,)
    }
};
        locals.var_xg2_ac = assign41680_e47519;
        locals.var_xg2_ac_dn4 = assign41680_e47519_d_n4;
        locals.var_xg2_ac_dn6 = assign41680_e47519_d_n6;
        locals.var_xg2_ac_dn7 = assign41680_e47519_d_n7;
        locals.var_xg2_ac_dn8 = assign41680_e47519_d_n8;
        locals.var_xg2_ac_dn9 = assign41680_e47519_d_n9;

    }

    pub(super) fn stamp_transient_block_116(
        locals: &mut StampLocals,
    ) {
        let (assign41690_e47523, assign41690_e47523_d_n4, assign41690_e47523_d_n6, assign41690_e47523_d_n7, assign41690_e47523_d_n8, assign41690_e47523_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_xg2x__blk931, locals.var_xg2x__blk931_dn4, locals.var_xg2x__blk931_dn6, locals.var_xg2x__blk931_dn7, locals.var_xg2x__blk931_dn8, locals.var_xg2x__blk931_dn9,)
    } else {
        (locals.var_xg2x_ac, locals.var_xg2x_ac_dn4, locals.var_xg2x_ac_dn6, locals.var_xg2x_ac_dn7, locals.var_xg2x_ac_dn8, locals.var_xg2x_ac_dn9,)
    }
};
        locals.var_xg2x_ac = assign41690_e47523;
        locals.var_xg2x_ac_dn4 = assign41690_e47523_d_n4;
        locals.var_xg2x_ac_dn6 = assign41690_e47523_d_n6;
        locals.var_xg2x_ac_dn7 = assign41690_e47523_d_n7;
        locals.var_xg2x_ac_dn8 = assign41690_e47523_d_n8;
        locals.var_xg2x_ac_dn9 = assign41690_e47523_d_n9;

        let (assign41700_e47527, assign41700_e47527_d_n4, assign41700_e47527_d_n6, assign41700_e47527_d_n7, assign41700_e47527_d_n8, assign41700_e47527_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_k1__blk932, locals.var_k1__blk932_dn4, locals.var_k1__blk932_dn6, locals.var_k1__blk932_dn7, locals.var_k1__blk932_dn8, locals.var_k1__blk932_dn9,)
    } else {
        (locals.var_k1_ac, locals.var_k1_ac_dn4, locals.var_k1_ac_dn6, locals.var_k1_ac_dn7, locals.var_k1_ac_dn8, locals.var_k1_ac_dn9,)
    }
};
        locals.var_k1_ac = assign41700_e47527;
        locals.var_k1_ac_dn4 = assign41700_e47527_d_n4;
        locals.var_k1_ac_dn6 = assign41700_e47527_d_n6;
        locals.var_k1_ac_dn7 = assign41700_e47527_d_n7;
        locals.var_k1_ac_dn8 = assign41700_e47527_d_n8;
        locals.var_k1_ac_dn9 = assign41700_e47527_d_n9;

        let (assign41710_e47531, assign41710_e47531_d_n4, assign41710_e47531_d_n6, assign41710_e47531_d_n7, assign41710_e47531_d_n8, assign41710_e47531_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_k2__blk933, locals.var_k2__blk933_dn4, locals.var_k2__blk933_dn6, locals.var_k2__blk933_dn7, locals.var_k2__blk933_dn8, locals.var_k2__blk933_dn9,)
    } else {
        (locals.var_k2_ac, locals.var_k2_ac_dn4, locals.var_k2_ac_dn6, locals.var_k2_ac_dn7, locals.var_k2_ac_dn8, locals.var_k2_ac_dn9,)
    }
};
        locals.var_k2_ac = assign41710_e47531;
        locals.var_k2_ac_dn4 = assign41710_e47531_d_n4;
        locals.var_k2_ac_dn6 = assign41710_e47531_d_n6;
        locals.var_k2_ac_dn7 = assign41710_e47531_d_n7;
        locals.var_k2_ac_dn8 = assign41710_e47531_d_n8;
        locals.var_k2_ac_dn9 = assign41710_e47531_d_n9;

        let (assign41720_e47535, assign41720_e47535_d_n4, assign41720_e47535_d_n6, assign41720_e47535_d_n7, assign41720_e47535_d_n8, assign41720_e47535_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_k1q1s__blk939, locals.var_k1q1s__blk939_dn4, locals.var_k1q1s__blk939_dn6, locals.var_k1q1s__blk939_dn7, locals.var_k1q1s__blk939_dn8, locals.var_k1q1s__blk939_dn9,)
    } else {
        (locals.var_k1q1s_ac, locals.var_k1q1s_ac_dn4, locals.var_k1q1s_ac_dn6, locals.var_k1q1s_ac_dn7, locals.var_k1q1s_ac_dn8, locals.var_k1q1s_ac_dn9,)
    }
};
        locals.var_k1q1s_ac = assign41720_e47535;
        locals.var_k1q1s_ac_dn4 = assign41720_e47535_d_n4;
        locals.var_k1q1s_ac_dn6 = assign41720_e47535_d_n6;
        locals.var_k1q1s_ac_dn7 = assign41720_e47535_d_n7;
        locals.var_k1q1s_ac_dn8 = assign41720_e47535_d_n8;
        locals.var_k1q1s_ac_dn9 = assign41720_e47535_d_n9;

        let (assign41730_e47539, assign41730_e47539_d_n4, assign41730_e47539_d_n6, assign41730_e47539_d_n7, assign41730_e47539_d_n8, assign41730_e47539_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_k2q2s__blk940, locals.var_k2q2s__blk940_dn4, locals.var_k2q2s__blk940_dn6, locals.var_k2q2s__blk940_dn7, locals.var_k2q2s__blk940_dn8, locals.var_k2q2s__blk940_dn9,)
    } else {
        (locals.var_k2q2s_ac, locals.var_k2q2s_ac_dn4, locals.var_k2q2s_ac_dn6, locals.var_k2q2s_ac_dn7, locals.var_k2q2s_ac_dn8, locals.var_k2q2s_ac_dn9,)
    }
};
        locals.var_k2q2s_ac = assign41730_e47539;
        locals.var_k2q2s_ac_dn4 = assign41730_e47539_d_n4;
        locals.var_k2q2s_ac_dn6 = assign41730_e47539_d_n6;
        locals.var_k2q2s_ac_dn7 = assign41730_e47539_d_n7;
        locals.var_k2q2s_ac_dn8 = assign41730_e47539_d_n8;
        locals.var_k2q2s_ac_dn9 = assign41730_e47539_d_n9;

        let (assign41740_e47543, assign41740_e47543_d_n4, assign41740_e47543_d_n6, assign41740_e47543_d_n7, assign41740_e47543_d_n8, assign41740_e47543_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_xdrifts__blk951, locals.var_xdrifts__blk951_dn4, locals.var_xdrifts__blk951_dn6, locals.var_xdrifts__blk951_dn7, locals.var_xdrifts__blk951_dn8, locals.var_xdrifts__blk951_dn9,)
    } else {
        (locals.var_xdrifts_ac, locals.var_xdrifts_ac_dn4, locals.var_xdrifts_ac_dn6, locals.var_xdrifts_ac_dn7, locals.var_xdrifts_ac_dn8, locals.var_xdrifts_ac_dn9,)
    }
};
        locals.var_xdrifts_ac = assign41740_e47543;
        locals.var_xdrifts_ac_dn4 = assign41740_e47543_d_n4;
        locals.var_xdrifts_ac_dn6 = assign41740_e47543_d_n6;
        locals.var_xdrifts_ac_dn7 = assign41740_e47543_d_n7;
        locals.var_xdrifts_ac_dn8 = assign41740_e47543_d_n8;
        locals.var_xdrifts_ac_dn9 = assign41740_e47543_d_n9;

        let (assign41750_e47547, assign41750_e47547_d_n4, assign41750_e47547_d_n6, assign41750_e47547_d_n7, assign41750_e47547_d_n8, assign41750_e47547_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_k1q1d__blk1004, locals.var_k1q1d__blk1004_dn4, locals.var_k1q1d__blk1004_dn6, locals.var_k1q1d__blk1004_dn7, locals.var_k1q1d__blk1004_dn8, locals.var_k1q1d__blk1004_dn9,)
    } else {
        (locals.var_k1q1d_ac, locals.var_k1q1d_ac_dn4, locals.var_k1q1d_ac_dn6, locals.var_k1q1d_ac_dn7, locals.var_k1q1d_ac_dn8, locals.var_k1q1d_ac_dn9,)
    }
};
        locals.var_k1q1d_ac = assign41750_e47547;
        locals.var_k1q1d_ac_dn4 = assign41750_e47547_d_n4;
        locals.var_k1q1d_ac_dn6 = assign41750_e47547_d_n6;
        locals.var_k1q1d_ac_dn7 = assign41750_e47547_d_n7;
        locals.var_k1q1d_ac_dn8 = assign41750_e47547_d_n8;
        locals.var_k1q1d_ac_dn9 = assign41750_e47547_d_n9;

        let (assign41760_e47551, assign41760_e47551_d_n4, assign41760_e47551_d_n6, assign41760_e47551_d_n7, assign41760_e47551_d_n8, assign41760_e47551_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9,)
    } else {
        (locals.var_k2q2d_ac, locals.var_k2q2d_ac_dn4, locals.var_k2q2d_ac_dn6, locals.var_k2q2d_ac_dn7, locals.var_k2q2d_ac_dn8, locals.var_k2q2d_ac_dn9,)
    }
};
        locals.var_k2q2d_ac = assign41760_e47551;
        locals.var_k2q2d_ac_dn4 = assign41760_e47551_d_n4;
        locals.var_k2q2d_ac_dn6 = assign41760_e47551_d_n6;
        locals.var_k2q2d_ac_dn7 = assign41760_e47551_d_n7;
        locals.var_k2q2d_ac_dn8 = assign41760_e47551_d_n8;
        locals.var_k2q2d_ac_dn9 = assign41760_e47551_d_n9;

        let (assign41770_e47555, assign41770_e47555_d_n4, assign41770_e47555_d_n6, assign41770_e47555_d_n7, assign41770_e47555_d_n8, assign41770_e47555_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_xdriftd__blk1015, locals.var_xdriftd__blk1015_dn4, locals.var_xdriftd__blk1015_dn6, locals.var_xdriftd__blk1015_dn7, locals.var_xdriftd__blk1015_dn8, locals.var_xdriftd__blk1015_dn9,)
    } else {
        (locals.var_xdriftd_ac, locals.var_xdriftd_ac_dn4, locals.var_xdriftd_ac_dn6, locals.var_xdriftd_ac_dn7, locals.var_xdriftd_ac_dn8, locals.var_xdriftd_ac_dn9,)
    }
};
        locals.var_xdriftd_ac = assign41770_e47555;
        locals.var_xdriftd_ac_dn4 = assign41770_e47555_d_n4;
        locals.var_xdriftd_ac_dn6 = assign41770_e47555_d_n6;
        locals.var_xdriftd_ac_dn7 = assign41770_e47555_d_n7;
        locals.var_xdriftd_ac_dn8 = assign41770_e47555_d_n8;
        locals.var_xdriftd_ac_dn9 = assign41770_e47555_d_n9;

        let (assign41780_e47559, assign41780_e47559_d_n4, assign41780_e47559_d_n6, assign41780_e47559_d_n7, assign41780_e47559_d_n8, assign41780_e47559_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_qim__blk1016, locals.var_qim__blk1016_dn4, locals.var_qim__blk1016_dn6, locals.var_qim__blk1016_dn7, locals.var_qim__blk1016_dn8, locals.var_qim__blk1016_dn9,)
    } else {
        (locals.var_qim_ac, locals.var_qim_ac_dn4, locals.var_qim_ac_dn6, locals.var_qim_ac_dn7, locals.var_qim_ac_dn8, locals.var_qim_ac_dn9,)
    }
};
        locals.var_qim_ac = assign41780_e47559;
        locals.var_qim_ac_dn4 = assign41780_e47559_d_n4;
        locals.var_qim_ac_dn6 = assign41780_e47559_d_n6;
        locals.var_qim_ac_dn7 = assign41780_e47559_d_n7;
        locals.var_qim_ac_dn8 = assign41780_e47559_d_n8;
        locals.var_qim_ac_dn9 = assign41780_e47559_d_n9;

        let (assign41790_e47563, assign41790_e47563_d_n4, assign41790_e47563_d_n6, assign41790_e47563_d_n7, assign41790_e47563_d_n8, assign41790_e47563_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_ratio_pd__blk1020, locals.var_ratio_pd__blk1020_dn4, locals.var_ratio_pd__blk1020_dn6, locals.var_ratio_pd__blk1020_dn7, locals.var_ratio_pd__blk1020_dn8, locals.var_ratio_pd__blk1020_dn9,)
    } else {
        (locals.var_ratio_pd_ac, locals.var_ratio_pd_ac_dn4, locals.var_ratio_pd_ac_dn6, locals.var_ratio_pd_ac_dn7, locals.var_ratio_pd_ac_dn8, locals.var_ratio_pd_ac_dn9,)
    }
};
        locals.var_ratio_pd_ac = assign41790_e47563;
        locals.var_ratio_pd_ac_dn4 = assign41790_e47563_d_n4;
        locals.var_ratio_pd_ac_dn6 = assign41790_e47563_d_n6;
        locals.var_ratio_pd_ac_dn7 = assign41790_e47563_d_n7;
        locals.var_ratio_pd_ac_dn8 = assign41790_e47563_d_n8;
        locals.var_ratio_pd_ac_dn9 = assign41790_e47563_d_n9;

        let (assign41800_e47567, assign41800_e47567_d_n4, assign41800_e47567_d_n6, assign41800_e47567_d_n7, assign41800_e47567_d_n8, assign41800_e47567_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_qi1m__blk1029, locals.var_qi1m__blk1029_dn4, locals.var_qi1m__blk1029_dn6, locals.var_qi1m__blk1029_dn7, locals.var_qi1m__blk1029_dn8, locals.var_qi1m__blk1029_dn9,)
    } else {
        (locals.var_qi1m_ac, locals.var_qi1m_ac_dn4, locals.var_qi1m_ac_dn6, locals.var_qi1m_ac_dn7, locals.var_qi1m_ac_dn8, locals.var_qi1m_ac_dn9,)
    }
};
        locals.var_qi1m_ac = assign41800_e47567;
        locals.var_qi1m_ac_dn4 = assign41800_e47567_d_n4;
        locals.var_qi1m_ac_dn6 = assign41800_e47567_d_n6;
        locals.var_qi1m_ac_dn7 = assign41800_e47567_d_n7;
        locals.var_qi1m_ac_dn8 = assign41800_e47567_d_n8;
        locals.var_qi1m_ac_dn9 = assign41800_e47567_d_n9;

        let (assign41810_e47571, assign41810_e47571_d_n4, assign41810_e47571_d_n6, assign41810_e47571_d_n7, assign41810_e47571_d_n8, assign41810_e47571_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_qi2m__blk1030, locals.var_qi2m__blk1030_dn4, locals.var_qi2m__blk1030_dn6, locals.var_qi2m__blk1030_dn7, locals.var_qi2m__blk1030_dn8, locals.var_qi2m__blk1030_dn9,)
    } else {
        (locals.var_qi2m_ac, locals.var_qi2m_ac_dn4, locals.var_qi2m_ac_dn6, locals.var_qi2m_ac_dn7, locals.var_qi2m_ac_dn8, locals.var_qi2m_ac_dn9,)
    }
};
        locals.var_qi2m_ac = assign41810_e47571;
        locals.var_qi2m_ac_dn4 = assign41810_e47571_d_n4;
        locals.var_qi2m_ac_dn6 = assign41810_e47571_d_n6;
        locals.var_qi2m_ac_dn7 = assign41810_e47571_d_n7;
        locals.var_qi2m_ac_dn8 = assign41810_e47571_d_n8;
        locals.var_qi2m_ac_dn9 = assign41810_e47571_d_n9;

        let (assign41820_e47575, assign41820_e47575_d_n4, assign41820_e47575_d_n6, assign41820_e47575_d_n7, assign41820_e47575_d_n8, assign41820_e47575_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_zsat__blk1051, locals.var_zsat__blk1051_dn4, locals.var_zsat__blk1051_dn6, locals.var_zsat__blk1051_dn7, locals.var_zsat__blk1051_dn8, locals.var_zsat__blk1051_dn9,)
    } else {
        (locals.var_zsat_ac, locals.var_zsat_ac_dn4, locals.var_zsat_ac_dn6, locals.var_zsat_ac_dn7, locals.var_zsat_ac_dn8, locals.var_zsat_ac_dn9,)
    }
};
        locals.var_zsat_ac = assign41820_e47575;
        locals.var_zsat_ac_dn4 = assign41820_e47575_d_n4;
        locals.var_zsat_ac_dn6 = assign41820_e47575_d_n6;
        locals.var_zsat_ac_dn7 = assign41820_e47575_d_n7;
        locals.var_zsat_ac_dn8 = assign41820_e47575_d_n8;
        locals.var_zsat_ac_dn9 = assign41820_e47575_d_n9;

        let (assign41830_e47579, assign41830_e47579_d_n4, assign41830_e47579_d_n6, assign41830_e47579_d_n7, assign41830_e47579_d_n8, assign41830_e47579_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_qmfact1__blk1054, locals.var_qmfact1__blk1054_dn4, locals.var_qmfact1__blk1054_dn6, locals.var_qmfact1__blk1054_dn7, locals.var_qmfact1__blk1054_dn8, locals.var_qmfact1__blk1054_dn9,)
    } else {
        (locals.var_qmfact1_ac, locals.var_qmfact1_ac_dn4, locals.var_qmfact1_ac_dn6, locals.var_qmfact1_ac_dn7, locals.var_qmfact1_ac_dn8, locals.var_qmfact1_ac_dn9,)
    }
};
        locals.var_qmfact1_ac = assign41830_e47579;
        locals.var_qmfact1_ac_dn4 = assign41830_e47579_d_n4;
        locals.var_qmfact1_ac_dn6 = assign41830_e47579_d_n6;
        locals.var_qmfact1_ac_dn7 = assign41830_e47579_d_n7;
        locals.var_qmfact1_ac_dn8 = assign41830_e47579_d_n8;
        locals.var_qmfact1_ac_dn9 = assign41830_e47579_d_n9;

        let (assign41840_e47583, assign41840_e47583_d_n4, assign41840_e47583_d_n6, assign41840_e47583_d_n7, assign41840_e47583_d_n8, assign41840_e47583_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_qmfact2__blk1055, locals.var_qmfact2__blk1055_dn4, locals.var_qmfact2__blk1055_dn6, locals.var_qmfact2__blk1055_dn7, locals.var_qmfact2__blk1055_dn8, locals.var_qmfact2__blk1055_dn9,)
    } else {
        (locals.var_qmfact2_ac, locals.var_qmfact2_ac_dn4, locals.var_qmfact2_ac_dn6, locals.var_qmfact2_ac_dn7, locals.var_qmfact2_ac_dn8, locals.var_qmfact2_ac_dn9,)
    }
};
        locals.var_qmfact2_ac = assign41840_e47583;
        locals.var_qmfact2_ac_dn4 = assign41840_e47583_d_n4;
        locals.var_qmfact2_ac_dn6 = assign41840_e47583_d_n6;
        locals.var_qmfact2_ac_dn7 = assign41840_e47583_d_n7;
        locals.var_qmfact2_ac_dn8 = assign41840_e47583_d_n8;
        locals.var_qmfact2_ac_dn9 = assign41840_e47583_d_n9;

        let (assign41850_e47587, assign41850_e47587_d_n4, assign41850_e47587_d_n6, assign41850_e47587_d_n7, assign41850_e47587_d_n8, assign41850_e47587_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_delta_k1q1__blk1076, locals.var_delta_k1q1__blk1076_dn4, locals.var_delta_k1q1__blk1076_dn6, locals.var_delta_k1q1__blk1076_dn7, locals.var_delta_k1q1__blk1076_dn8, locals.var_delta_k1q1__blk1076_dn9,)
    } else {
        (locals.var_delta_k1q1_ac, locals.var_delta_k1q1_ac_dn4, locals.var_delta_k1q1_ac_dn6, locals.var_delta_k1q1_ac_dn7, locals.var_delta_k1q1_ac_dn8, locals.var_delta_k1q1_ac_dn9,)
    }
};
        locals.var_delta_k1q1_ac = assign41850_e47587;
        locals.var_delta_k1q1_ac_dn4 = assign41850_e47587_d_n4;
        locals.var_delta_k1q1_ac_dn6 = assign41850_e47587_d_n6;
        locals.var_delta_k1q1_ac_dn7 = assign41850_e47587_d_n7;
        locals.var_delta_k1q1_ac_dn8 = assign41850_e47587_d_n8;
        locals.var_delta_k1q1_ac_dn9 = assign41850_e47587_d_n9;

        let (assign41860_e47591, assign41860_e47591_d_n4, assign41860_e47591_d_n6, assign41860_e47591_d_n7, assign41860_e47591_d_n8, assign41860_e47591_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_delta_k2q2__blk1077, locals.var_delta_k2q2__blk1077_dn4, locals.var_delta_k2q2__blk1077_dn6, locals.var_delta_k2q2__blk1077_dn7, locals.var_delta_k2q2__blk1077_dn8, locals.var_delta_k2q2__blk1077_dn9,)
    } else {
        (locals.var_delta_k2q2_ac, locals.var_delta_k2q2_ac_dn4, locals.var_delta_k2q2_ac_dn6, locals.var_delta_k2q2_ac_dn7, locals.var_delta_k2q2_ac_dn8, locals.var_delta_k2q2_ac_dn9,)
    }
};
        locals.var_delta_k2q2_ac = assign41860_e47591;
        locals.var_delta_k2q2_ac_dn4 = assign41860_e47591_d_n4;
        locals.var_delta_k2q2_ac_dn6 = assign41860_e47591_d_n6;
        locals.var_delta_k2q2_ac_dn7 = assign41860_e47591_d_n7;
        locals.var_delta_k2q2_ac_dn8 = assign41860_e47591_d_n8;
        locals.var_delta_k2q2_ac_dn9 = assign41860_e47591_d_n9;

        let (assign41870_e47595, assign41870_e47595_d_n4, assign41870_e47595_d_n6, assign41870_e47595_d_n7, assign41870_e47595_d_n8, assign41870_e47595_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_prod1__blk1078, locals.var_prod1__blk1078_dn4, locals.var_prod1__blk1078_dn6, locals.var_prod1__blk1078_dn7, locals.var_prod1__blk1078_dn8, locals.var_prod1__blk1078_dn9,)
    } else {
        (locals.var_prod1_ac, locals.var_prod1_ac_dn4, locals.var_prod1_ac_dn6, locals.var_prod1_ac_dn7, locals.var_prod1_ac_dn8, locals.var_prod1_ac_dn9,)
    }
};
        locals.var_prod1_ac = assign41870_e47595;
        locals.var_prod1_ac_dn4 = assign41870_e47595_d_n4;
        locals.var_prod1_ac_dn6 = assign41870_e47595_d_n6;
        locals.var_prod1_ac_dn7 = assign41870_e47595_d_n7;
        locals.var_prod1_ac_dn8 = assign41870_e47595_d_n8;
        locals.var_prod1_ac_dn9 = assign41870_e47595_d_n9;

        let (assign41880_e47599, assign41880_e47599_d_n4, assign41880_e47599_d_n6, assign41880_e47599_d_n7, assign41880_e47599_d_n8, assign41880_e47599_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_prod2__blk1079, locals.var_prod2__blk1079_dn4, locals.var_prod2__blk1079_dn6, locals.var_prod2__blk1079_dn7, locals.var_prod2__blk1079_dn8, locals.var_prod2__blk1079_dn9,)
    } else {
        (locals.var_prod2_ac, locals.var_prod2_ac_dn4, locals.var_prod2_ac_dn6, locals.var_prod2_ac_dn7, locals.var_prod2_ac_dn8, locals.var_prod2_ac_dn9,)
    }
};
        locals.var_prod2_ac = assign41880_e47599;
        locals.var_prod2_ac_dn4 = assign41880_e47599_d_n4;
        locals.var_prod2_ac_dn6 = assign41880_e47599_d_n6;
        locals.var_prod2_ac_dn7 = assign41880_e47599_d_n7;
        locals.var_prod2_ac_dn8 = assign41880_e47599_d_n8;
        locals.var_prod2_ac_dn9 = assign41880_e47599_d_n9;

        let (assign41890_e47604, assign41890_e47604_d_n4, assign41890_e47604_d_n6, assign41890_e47604_d_n7, assign41890_e47604_d_n8, assign41890_e47604_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_xg20shift_dc, locals.var_xg20shift_dc_dn4, locals.var_xg20shift_dc_dn6, locals.var_xg20shift_dc_dn7, locals.var_xg20shift_dc_dn8, locals.var_xg20shift_dc_dn9,)
    } else {
        (locals.var_xg20shift_ac, locals.var_xg20shift_ac_dn4, locals.var_xg20shift_ac_dn6, locals.var_xg20shift_ac_dn7, locals.var_xg20shift_ac_dn8, locals.var_xg20shift_ac_dn9,)
    }
};
        locals.var_xg20shift_ac = assign41890_e47604;
        locals.var_xg20shift_ac_dn4 = assign41890_e47604_d_n4;
        locals.var_xg20shift_ac_dn6 = assign41890_e47604_d_n6;
        locals.var_xg20shift_ac_dn7 = assign41890_e47604_d_n7;
        locals.var_xg20shift_ac_dn8 = assign41890_e47604_d_n8;
        locals.var_xg20shift_ac_dn9 = assign41890_e47604_d_n9;

        let (assign41900_e47609, assign41900_e47609_d_n4, assign41900_e47609_d_n6, assign41900_e47609_d_n7, assign41900_e47609_d_n8, assign41900_e47609_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_diff_min_dc, locals.var_diff_min_dc_dn4, locals.var_diff_min_dc_dn6, locals.var_diff_min_dc_dn7, locals.var_diff_min_dc_dn8, locals.var_diff_min_dc_dn9,)
    } else {
        (locals.var_diff_min_ac, locals.var_diff_min_ac_dn4, locals.var_diff_min_ac_dn6, locals.var_diff_min_ac_dn7, locals.var_diff_min_ac_dn8, locals.var_diff_min_ac_dn9,)
    }
};
        locals.var_diff_min_ac = assign41900_e47609;
        locals.var_diff_min_ac_dn4 = assign41900_e47609_d_n4;
        locals.var_diff_min_ac_dn6 = assign41900_e47609_d_n6;
        locals.var_diff_min_ac_dn7 = assign41900_e47609_d_n7;
        locals.var_diff_min_ac_dn8 = assign41900_e47609_d_n8;
        locals.var_diff_min_ac_dn9 = assign41900_e47609_d_n9;

        let (assign41910_e47614, assign41910_e47614_d_n4, assign41910_e47614_d_n6, assign41910_e47614_d_n7, assign41910_e47614_d_n8, assign41910_e47614_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_a0_dc, locals.var_a0_dc_dn4, locals.var_a0_dc_dn6, locals.var_a0_dc_dn7, locals.var_a0_dc_dn8, locals.var_a0_dc_dn9,)
    } else {
        (locals.var_a0_ac, locals.var_a0_ac_dn4, locals.var_a0_ac_dn6, locals.var_a0_ac_dn7, locals.var_a0_ac_dn8, locals.var_a0_ac_dn9,)
    }
};
        locals.var_a0_ac = assign41910_e47614;
        locals.var_a0_ac_dn4 = assign41910_e47614_d_n4;
        locals.var_a0_ac_dn6 = assign41910_e47614_d_n6;
        locals.var_a0_ac_dn7 = assign41910_e47614_d_n7;
        locals.var_a0_ac_dn8 = assign41910_e47614_d_n8;
        locals.var_a0_ac_dn9 = assign41910_e47614_d_n9;

        let (assign41920_e47619, assign41920_e47619_d_n4, assign41920_e47619_d_n6, assign41920_e47619_d_n7, assign41920_e47619_d_n8, assign41920_e47619_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_inv_k1_dc, locals.var_inv_k1_dc_dn4, locals.var_inv_k1_dc_dn6, locals.var_inv_k1_dc_dn7, locals.var_inv_k1_dc_dn8, locals.var_inv_k1_dc_dn9,)
    } else {
        (locals.var_inv_k1_ac, locals.var_inv_k1_ac_dn4, locals.var_inv_k1_ac_dn6, locals.var_inv_k1_ac_dn7, locals.var_inv_k1_ac_dn8, locals.var_inv_k1_ac_dn9,)
    }
};
        locals.var_inv_k1_ac = assign41920_e47619;
        locals.var_inv_k1_ac_dn4 = assign41920_e47619_d_n4;
        locals.var_inv_k1_ac_dn6 = assign41920_e47619_d_n6;
        locals.var_inv_k1_ac_dn7 = assign41920_e47619_d_n7;
        locals.var_inv_k1_ac_dn8 = assign41920_e47619_d_n8;
        locals.var_inv_k1_ac_dn9 = assign41920_e47619_d_n9;

        let (assign41930_e47624, assign41930_e47624_d_n4, assign41930_e47624_d_n6, assign41930_e47624_d_n7, assign41930_e47624_d_n8, assign41930_e47624_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_inv_k2_dc, locals.var_inv_k2_dc_dn4, locals.var_inv_k2_dc_dn6, locals.var_inv_k2_dc_dn7, locals.var_inv_k2_dc_dn8, locals.var_inv_k2_dc_dn9,)
    } else {
        (locals.var_inv_k2_ac, locals.var_inv_k2_ac_dn4, locals.var_inv_k2_ac_dn6, locals.var_inv_k2_ac_dn7, locals.var_inv_k2_ac_dn8, locals.var_inv_k2_ac_dn9,)
    }
};
        locals.var_inv_k2_ac = assign41930_e47624;
        locals.var_inv_k2_ac_dn4 = assign41930_e47624_d_n4;
        locals.var_inv_k2_ac_dn6 = assign41930_e47624_d_n6;
        locals.var_inv_k2_ac_dn7 = assign41930_e47624_d_n7;
        locals.var_inv_k2_ac_dn8 = assign41930_e47624_d_n8;
        locals.var_inv_k2_ac_dn9 = assign41930_e47624_d_n9;

        let (assign41940_e47629, assign41940_e47629_d_n4, assign41940_e47629_d_n6, assign41940_e47629_d_n7, assign41940_e47629_d_n8, assign41940_e47629_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_keq_dc, locals.var_keq_dc_dn4, locals.var_keq_dc_dn6, locals.var_keq_dc_dn7, locals.var_keq_dc_dn8, locals.var_keq_dc_dn9,)
    } else {
        (locals.var_keq_ac, locals.var_keq_ac_dn4, locals.var_keq_ac_dn6, locals.var_keq_ac_dn7, locals.var_keq_ac_dn8, locals.var_keq_ac_dn9,)
    }
};
        locals.var_keq_ac = assign41940_e47629;
        locals.var_keq_ac_dn4 = assign41940_e47629_d_n4;
        locals.var_keq_ac_dn6 = assign41940_e47629_d_n6;
        locals.var_keq_ac_dn7 = assign41940_e47629_d_n7;
        locals.var_keq_ac_dn8 = assign41940_e47629_d_n8;
        locals.var_keq_ac_dn9 = assign41940_e47629_d_n9;

        let (assign41950_e47634, assign41950_e47634_d_n4, assign41950_e47634_d_n6, assign41950_e47634_d_n7, assign41950_e47634_d_n8, assign41950_e47634_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_dx_wi_dc, locals.var_dx_wi_dc_dn4, locals.var_dx_wi_dc_dn6, locals.var_dx_wi_dc_dn7, locals.var_dx_wi_dc_dn8, locals.var_dx_wi_dc_dn9,)
    } else {
        (locals.var_dx_wi_ac, locals.var_dx_wi_ac_dn4, locals.var_dx_wi_ac_dn6, locals.var_dx_wi_ac_dn7, locals.var_dx_wi_ac_dn8, locals.var_dx_wi_ac_dn9,)
    }
};
        locals.var_dx_wi_ac = assign41950_e47634;
        locals.var_dx_wi_ac_dn4 = assign41950_e47634_d_n4;
        locals.var_dx_wi_ac_dn6 = assign41950_e47634_d_n6;
        locals.var_dx_wi_ac_dn7 = assign41950_e47634_d_n7;
        locals.var_dx_wi_ac_dn8 = assign41950_e47634_d_n8;
        locals.var_dx_wi_ac_dn9 = assign41950_e47634_d_n9;

        let (assign41960_e47639, assign41960_e47639_d_n4, assign41960_e47639_d_n6, assign41960_e47639_d_n7, assign41960_e47639_d_n8, assign41960_e47639_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_csiprime_dc, locals.var_csiprime_dc_dn4, locals.var_csiprime_dc_dn6, locals.var_csiprime_dc_dn7, locals.var_csiprime_dc_dn8, locals.var_csiprime_dc_dn9,)
    } else {
        (locals.var_csiprime_ac, locals.var_csiprime_ac_dn4, locals.var_csiprime_ac_dn6, locals.var_csiprime_ac_dn7, locals.var_csiprime_ac_dn8, locals.var_csiprime_ac_dn9,)
    }
};
        locals.var_csiprime_ac = assign41960_e47639;
        locals.var_csiprime_ac_dn4 = assign41960_e47639_d_n4;
        locals.var_csiprime_ac_dn6 = assign41960_e47639_d_n6;
        locals.var_csiprime_ac_dn7 = assign41960_e47639_d_n7;
        locals.var_csiprime_ac_dn8 = assign41960_e47639_d_n8;
        locals.var_csiprime_ac_dn9 = assign41960_e47639_d_n9;

        let (assign41970_e47644, assign41970_e47644_d_n4, assign41970_e47644_d_n6, assign41970_e47644_d_n7, assign41970_e47644_d_n8, assign41970_e47644_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_dx_wi_1d_dc, locals.var_dx_wi_1d_dc_dn4, locals.var_dx_wi_1d_dc_dn6, locals.var_dx_wi_1d_dc_dn7, locals.var_dx_wi_1d_dc_dn8, locals.var_dx_wi_1d_dc_dn9,)
    } else {
        (locals.var_dx_wi_1d_ac, locals.var_dx_wi_1d_ac_dn4, locals.var_dx_wi_1d_ac_dn6, locals.var_dx_wi_1d_ac_dn7, locals.var_dx_wi_1d_ac_dn8, locals.var_dx_wi_1d_ac_dn9,)
    }
};
        locals.var_dx_wi_1d_ac = assign41970_e47644;
        locals.var_dx_wi_1d_ac_dn4 = assign41970_e47644_d_n4;
        locals.var_dx_wi_1d_ac_dn6 = assign41970_e47644_d_n6;
        locals.var_dx_wi_1d_ac_dn7 = assign41970_e47644_d_n7;
        locals.var_dx_wi_1d_ac_dn8 = assign41970_e47644_d_n8;
        locals.var_dx_wi_1d_ac_dn9 = assign41970_e47644_d_n9;

        let (assign41980_e47649, assign41980_e47649_d_n4, assign41980_e47649_d_n6, assign41980_e47649_d_n7, assign41980_e47649_d_n8, assign41980_e47649_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_dleff_dc, locals.var_dleff_dc_dn4, locals.var_dleff_dc_dn6, locals.var_dleff_dc_dn7, locals.var_dleff_dc_dn8, locals.var_dleff_dc_dn9,)
    } else {
        (locals.var_dleff_ac, locals.var_dleff_ac_dn4, locals.var_dleff_ac_dn6, locals.var_dleff_ac_dn7, locals.var_dleff_ac_dn8, locals.var_dleff_ac_dn9,)
    }
};
        locals.var_dleff_ac = assign41980_e47649;
        locals.var_dleff_ac_dn4 = assign41980_e47649_d_n4;
        locals.var_dleff_ac_dn6 = assign41980_e47649_d_n6;
        locals.var_dleff_ac_dn7 = assign41980_e47649_d_n7;
        locals.var_dleff_ac_dn8 = assign41980_e47649_d_n8;
        locals.var_dleff_ac_dn9 = assign41980_e47649_d_n9;

        let (assign41990_e47654, assign41990_e47654_d_n4, assign41990_e47654_d_n6, assign41990_e47654_d_n7, assign41990_e47654_d_n8, assign41990_e47654_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_xedge_dc, locals.var_xedge_dc_dn4, locals.var_xedge_dc_dn6, locals.var_xedge_dc_dn7, locals.var_xedge_dc_dn8, locals.var_xedge_dc_dn9,)
    } else {
        (locals.var_xedge_ac, locals.var_xedge_ac_dn4, locals.var_xedge_ac_dn6, locals.var_xedge_ac_dn7, locals.var_xedge_ac_dn8, locals.var_xedge_ac_dn9,)
    }
};
        locals.var_xedge_ac = assign41990_e47654;
        locals.var_xedge_ac_dn4 = assign41990_e47654_d_n4;
        locals.var_xedge_ac_dn6 = assign41990_e47654_d_n6;
        locals.var_xedge_ac_dn7 = assign41990_e47654_d_n7;
        locals.var_xedge_ac_dn8 = assign41990_e47654_d_n8;
        locals.var_xedge_ac_dn9 = assign41990_e47654_d_n9;

        let (assign42000_e47659, assign42000_e47659_d_n4, assign42000_e47659_d_n6, assign42000_e47659_d_n7, assign42000_e47659_d_n8, assign42000_e47659_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_sce1_dc, locals.var_sce1_dc_dn4, locals.var_sce1_dc_dn6, locals.var_sce1_dc_dn7, locals.var_sce1_dc_dn8, locals.var_sce1_dc_dn9,)
    } else {
        (locals.var_sce1_ac, locals.var_sce1_ac_dn4, locals.var_sce1_ac_dn6, locals.var_sce1_ac_dn7, locals.var_sce1_ac_dn8, locals.var_sce1_ac_dn9,)
    }
};
        locals.var_sce1_ac = assign42000_e47659;
        locals.var_sce1_ac_dn4 = assign42000_e47659_d_n4;
        locals.var_sce1_ac_dn6 = assign42000_e47659_d_n6;
        locals.var_sce1_ac_dn7 = assign42000_e47659_d_n7;
        locals.var_sce1_ac_dn8 = assign42000_e47659_d_n8;
        locals.var_sce1_ac_dn9 = assign42000_e47659_d_n9;

        let (assign42010_e47664, assign42010_e47664_d_n4, assign42010_e47664_d_n6, assign42010_e47664_d_n7, assign42010_e47664_d_n8, assign42010_e47664_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_sce2_dc, locals.var_sce2_dc_dn4, locals.var_sce2_dc_dn6, locals.var_sce2_dc_dn7, locals.var_sce2_dc_dn8, locals.var_sce2_dc_dn9,)
    } else {
        (locals.var_sce2_ac, locals.var_sce2_ac_dn4, locals.var_sce2_ac_dn6, locals.var_sce2_ac_dn7, locals.var_sce2_ac_dn8, locals.var_sce2_ac_dn9,)
    }
};
        locals.var_sce2_ac = assign42010_e47664;
        locals.var_sce2_ac_dn4 = assign42010_e47664_d_n4;
        locals.var_sce2_ac_dn6 = assign42010_e47664_d_n6;
        locals.var_sce2_ac_dn7 = assign42010_e47664_d_n7;
        locals.var_sce2_ac_dn8 = assign42010_e47664_d_n8;
        locals.var_sce2_ac_dn9 = assign42010_e47664_d_n9;

        let (assign42020_e47669, assign42020_e47669_d_n4, assign42020_e47669_d_n6, assign42020_e47669_d_n7, assign42020_e47669_d_n8, assign42020_e47669_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_dxg1_dibl_dc, locals.var_dxg1_dibl_dc_dn4, locals.var_dxg1_dibl_dc_dn6, locals.var_dxg1_dibl_dc_dn7, locals.var_dxg1_dibl_dc_dn8, locals.var_dxg1_dibl_dc_dn9,)
    } else {
        (locals.var_dxg1_dibl_ac, locals.var_dxg1_dibl_ac_dn4, locals.var_dxg1_dibl_ac_dn6, locals.var_dxg1_dibl_ac_dn7, locals.var_dxg1_dibl_ac_dn8, locals.var_dxg1_dibl_ac_dn9,)
    }
};
        locals.var_dxg1_dibl_ac = assign42020_e47669;
        locals.var_dxg1_dibl_ac_dn4 = assign42020_e47669_d_n4;
        locals.var_dxg1_dibl_ac_dn6 = assign42020_e47669_d_n6;
        locals.var_dxg1_dibl_ac_dn7 = assign42020_e47669_d_n7;
        locals.var_dxg1_dibl_ac_dn8 = assign42020_e47669_d_n8;
        locals.var_dxg1_dibl_ac_dn9 = assign42020_e47669_d_n9;

        let (assign42030_e47674, assign42030_e47674_d_n4, assign42030_e47674_d_n6, assign42030_e47674_d_n7, assign42030_e47674_d_n8, assign42030_e47674_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_xg2_dc, locals.var_xg2_dc_dn4, locals.var_xg2_dc_dn6, locals.var_xg2_dc_dn7, locals.var_xg2_dc_dn8, locals.var_xg2_dc_dn9,)
    } else {
        (locals.var_xg2_ac, locals.var_xg2_ac_dn4, locals.var_xg2_ac_dn6, locals.var_xg2_ac_dn7, locals.var_xg2_ac_dn8, locals.var_xg2_ac_dn9,)
    }
};
        locals.var_xg2_ac = assign42030_e47674;
        locals.var_xg2_ac_dn4 = assign42030_e47674_d_n4;
        locals.var_xg2_ac_dn6 = assign42030_e47674_d_n6;
        locals.var_xg2_ac_dn7 = assign42030_e47674_d_n7;
        locals.var_xg2_ac_dn8 = assign42030_e47674_d_n8;
        locals.var_xg2_ac_dn9 = assign42030_e47674_d_n9;

        let (assign42040_e47679, assign42040_e47679_d_n4, assign42040_e47679_d_n6, assign42040_e47679_d_n7, assign42040_e47679_d_n8, assign42040_e47679_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_xg2x_dc, locals.var_xg2x_dc_dn4, locals.var_xg2x_dc_dn6, locals.var_xg2x_dc_dn7, locals.var_xg2x_dc_dn8, locals.var_xg2x_dc_dn9,)
    } else {
        (locals.var_xg2x_ac, locals.var_xg2x_ac_dn4, locals.var_xg2x_ac_dn6, locals.var_xg2x_ac_dn7, locals.var_xg2x_ac_dn8, locals.var_xg2x_ac_dn9,)
    }
};
        locals.var_xg2x_ac = assign42040_e47679;
        locals.var_xg2x_ac_dn4 = assign42040_e47679_d_n4;
        locals.var_xg2x_ac_dn6 = assign42040_e47679_d_n6;
        locals.var_xg2x_ac_dn7 = assign42040_e47679_d_n7;
        locals.var_xg2x_ac_dn8 = assign42040_e47679_d_n8;
        locals.var_xg2x_ac_dn9 = assign42040_e47679_d_n9;

    }

    pub(super) fn stamp_transient_block_117(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign42050_e47684, assign42050_e47684_d_n4, assign42050_e47684_d_n6, assign42050_e47684_d_n7, assign42050_e47684_d_n8, assign42050_e47684_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_k1_dc, locals.var_k1_dc_dn4, locals.var_k1_dc_dn6, locals.var_k1_dc_dn7, locals.var_k1_dc_dn8, locals.var_k1_dc_dn9,)
    } else {
        (locals.var_k1_ac, locals.var_k1_ac_dn4, locals.var_k1_ac_dn6, locals.var_k1_ac_dn7, locals.var_k1_ac_dn8, locals.var_k1_ac_dn9,)
    }
};
        locals.var_k1_ac = assign42050_e47684;
        locals.var_k1_ac_dn4 = assign42050_e47684_d_n4;
        locals.var_k1_ac_dn6 = assign42050_e47684_d_n6;
        locals.var_k1_ac_dn7 = assign42050_e47684_d_n7;
        locals.var_k1_ac_dn8 = assign42050_e47684_d_n8;
        locals.var_k1_ac_dn9 = assign42050_e47684_d_n9;

        let (assign42060_e47689, assign42060_e47689_d_n4, assign42060_e47689_d_n6, assign42060_e47689_d_n7, assign42060_e47689_d_n8, assign42060_e47689_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_k2_dc, locals.var_k2_dc_dn4, locals.var_k2_dc_dn6, locals.var_k2_dc_dn7, locals.var_k2_dc_dn8, locals.var_k2_dc_dn9,)
    } else {
        (locals.var_k2_ac, locals.var_k2_ac_dn4, locals.var_k2_ac_dn6, locals.var_k2_ac_dn7, locals.var_k2_ac_dn8, locals.var_k2_ac_dn9,)
    }
};
        locals.var_k2_ac = assign42060_e47689;
        locals.var_k2_ac_dn4 = assign42060_e47689_d_n4;
        locals.var_k2_ac_dn6 = assign42060_e47689_d_n6;
        locals.var_k2_ac_dn7 = assign42060_e47689_d_n7;
        locals.var_k2_ac_dn8 = assign42060_e47689_d_n8;
        locals.var_k2_ac_dn9 = assign42060_e47689_d_n9;

        let (assign42070_e47694, assign42070_e47694_d_n4, assign42070_e47694_d_n6, assign42070_e47694_d_n7, assign42070_e47694_d_n8, assign42070_e47694_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_k1q1s_dc, locals.var_k1q1s_dc_dn4, locals.var_k1q1s_dc_dn6, locals.var_k1q1s_dc_dn7, locals.var_k1q1s_dc_dn8, locals.var_k1q1s_dc_dn9,)
    } else {
        (locals.var_k1q1s_ac, locals.var_k1q1s_ac_dn4, locals.var_k1q1s_ac_dn6, locals.var_k1q1s_ac_dn7, locals.var_k1q1s_ac_dn8, locals.var_k1q1s_ac_dn9,)
    }
};
        locals.var_k1q1s_ac = assign42070_e47694;
        locals.var_k1q1s_ac_dn4 = assign42070_e47694_d_n4;
        locals.var_k1q1s_ac_dn6 = assign42070_e47694_d_n6;
        locals.var_k1q1s_ac_dn7 = assign42070_e47694_d_n7;
        locals.var_k1q1s_ac_dn8 = assign42070_e47694_d_n8;
        locals.var_k1q1s_ac_dn9 = assign42070_e47694_d_n9;

        let (assign42080_e47699, assign42080_e47699_d_n4, assign42080_e47699_d_n6, assign42080_e47699_d_n7, assign42080_e47699_d_n8, assign42080_e47699_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_k2q2s_dc, locals.var_k2q2s_dc_dn4, locals.var_k2q2s_dc_dn6, locals.var_k2q2s_dc_dn7, locals.var_k2q2s_dc_dn8, locals.var_k2q2s_dc_dn9,)
    } else {
        (locals.var_k2q2s_ac, locals.var_k2q2s_ac_dn4, locals.var_k2q2s_ac_dn6, locals.var_k2q2s_ac_dn7, locals.var_k2q2s_ac_dn8, locals.var_k2q2s_ac_dn9,)
    }
};
        locals.var_k2q2s_ac = assign42080_e47699;
        locals.var_k2q2s_ac_dn4 = assign42080_e47699_d_n4;
        locals.var_k2q2s_ac_dn6 = assign42080_e47699_d_n6;
        locals.var_k2q2s_ac_dn7 = assign42080_e47699_d_n7;
        locals.var_k2q2s_ac_dn8 = assign42080_e47699_d_n8;
        locals.var_k2q2s_ac_dn9 = assign42080_e47699_d_n9;

        let (assign42090_e47704, assign42090_e47704_d_n4, assign42090_e47704_d_n6, assign42090_e47704_d_n7, assign42090_e47704_d_n8, assign42090_e47704_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_xdrifts_dc, locals.var_xdrifts_dc_dn4, locals.var_xdrifts_dc_dn6, locals.var_xdrifts_dc_dn7, locals.var_xdrifts_dc_dn8, locals.var_xdrifts_dc_dn9,)
    } else {
        (locals.var_xdrifts_ac, locals.var_xdrifts_ac_dn4, locals.var_xdrifts_ac_dn6, locals.var_xdrifts_ac_dn7, locals.var_xdrifts_ac_dn8, locals.var_xdrifts_ac_dn9,)
    }
};
        locals.var_xdrifts_ac = assign42090_e47704;
        locals.var_xdrifts_ac_dn4 = assign42090_e47704_d_n4;
        locals.var_xdrifts_ac_dn6 = assign42090_e47704_d_n6;
        locals.var_xdrifts_ac_dn7 = assign42090_e47704_d_n7;
        locals.var_xdrifts_ac_dn8 = assign42090_e47704_d_n8;
        locals.var_xdrifts_ac_dn9 = assign42090_e47704_d_n9;

        let (assign42100_e47709, assign42100_e47709_d_n4, assign42100_e47709_d_n6, assign42100_e47709_d_n7, assign42100_e47709_d_n8, assign42100_e47709_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_k1q1d_dc, locals.var_k1q1d_dc_dn4, locals.var_k1q1d_dc_dn6, locals.var_k1q1d_dc_dn7, locals.var_k1q1d_dc_dn8, locals.var_k1q1d_dc_dn9,)
    } else {
        (locals.var_k1q1d_ac, locals.var_k1q1d_ac_dn4, locals.var_k1q1d_ac_dn6, locals.var_k1q1d_ac_dn7, locals.var_k1q1d_ac_dn8, locals.var_k1q1d_ac_dn9,)
    }
};
        locals.var_k1q1d_ac = assign42100_e47709;
        locals.var_k1q1d_ac_dn4 = assign42100_e47709_d_n4;
        locals.var_k1q1d_ac_dn6 = assign42100_e47709_d_n6;
        locals.var_k1q1d_ac_dn7 = assign42100_e47709_d_n7;
        locals.var_k1q1d_ac_dn8 = assign42100_e47709_d_n8;
        locals.var_k1q1d_ac_dn9 = assign42100_e47709_d_n9;

        let (assign42110_e47714, assign42110_e47714_d_n4, assign42110_e47714_d_n6, assign42110_e47714_d_n7, assign42110_e47714_d_n8, assign42110_e47714_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_k2q2d_dc, locals.var_k2q2d_dc_dn4, locals.var_k2q2d_dc_dn6, locals.var_k2q2d_dc_dn7, locals.var_k2q2d_dc_dn8, locals.var_k2q2d_dc_dn9,)
    } else {
        (locals.var_k2q2d_ac, locals.var_k2q2d_ac_dn4, locals.var_k2q2d_ac_dn6, locals.var_k2q2d_ac_dn7, locals.var_k2q2d_ac_dn8, locals.var_k2q2d_ac_dn9,)
    }
};
        locals.var_k2q2d_ac = assign42110_e47714;
        locals.var_k2q2d_ac_dn4 = assign42110_e47714_d_n4;
        locals.var_k2q2d_ac_dn6 = assign42110_e47714_d_n6;
        locals.var_k2q2d_ac_dn7 = assign42110_e47714_d_n7;
        locals.var_k2q2d_ac_dn8 = assign42110_e47714_d_n8;
        locals.var_k2q2d_ac_dn9 = assign42110_e47714_d_n9;

        let (assign42120_e47719, assign42120_e47719_d_n4, assign42120_e47719_d_n6, assign42120_e47719_d_n7, assign42120_e47719_d_n8, assign42120_e47719_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_xdriftd_dc, locals.var_xdriftd_dc_dn4, locals.var_xdriftd_dc_dn6, locals.var_xdriftd_dc_dn7, locals.var_xdriftd_dc_dn8, locals.var_xdriftd_dc_dn9,)
    } else {
        (locals.var_xdriftd_ac, locals.var_xdriftd_ac_dn4, locals.var_xdriftd_ac_dn6, locals.var_xdriftd_ac_dn7, locals.var_xdriftd_ac_dn8, locals.var_xdriftd_ac_dn9,)
    }
};
        locals.var_xdriftd_ac = assign42120_e47719;
        locals.var_xdriftd_ac_dn4 = assign42120_e47719_d_n4;
        locals.var_xdriftd_ac_dn6 = assign42120_e47719_d_n6;
        locals.var_xdriftd_ac_dn7 = assign42120_e47719_d_n7;
        locals.var_xdriftd_ac_dn8 = assign42120_e47719_d_n8;
        locals.var_xdriftd_ac_dn9 = assign42120_e47719_d_n9;

        let (assign42130_e47724, assign42130_e47724_d_n4, assign42130_e47724_d_n6, assign42130_e47724_d_n7, assign42130_e47724_d_n8, assign42130_e47724_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_qim_dc, locals.var_qim_dc_dn4, locals.var_qim_dc_dn6, locals.var_qim_dc_dn7, locals.var_qim_dc_dn8, locals.var_qim_dc_dn9,)
    } else {
        (locals.var_qim_ac, locals.var_qim_ac_dn4, locals.var_qim_ac_dn6, locals.var_qim_ac_dn7, locals.var_qim_ac_dn8, locals.var_qim_ac_dn9,)
    }
};
        locals.var_qim_ac = assign42130_e47724;
        locals.var_qim_ac_dn4 = assign42130_e47724_d_n4;
        locals.var_qim_ac_dn6 = assign42130_e47724_d_n6;
        locals.var_qim_ac_dn7 = assign42130_e47724_d_n7;
        locals.var_qim_ac_dn8 = assign42130_e47724_d_n8;
        locals.var_qim_ac_dn9 = assign42130_e47724_d_n9;

        let (assign42140_e47729, assign42140_e47729_d_n4, assign42140_e47729_d_n6, assign42140_e47729_d_n7, assign42140_e47729_d_n8, assign42140_e47729_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_ratio_pd_dc, locals.var_ratio_pd_dc_dn4, locals.var_ratio_pd_dc_dn6, locals.var_ratio_pd_dc_dn7, locals.var_ratio_pd_dc_dn8, locals.var_ratio_pd_dc_dn9,)
    } else {
        (locals.var_ratio_pd_ac, locals.var_ratio_pd_ac_dn4, locals.var_ratio_pd_ac_dn6, locals.var_ratio_pd_ac_dn7, locals.var_ratio_pd_ac_dn8, locals.var_ratio_pd_ac_dn9,)
    }
};
        locals.var_ratio_pd_ac = assign42140_e47729;
        locals.var_ratio_pd_ac_dn4 = assign42140_e47729_d_n4;
        locals.var_ratio_pd_ac_dn6 = assign42140_e47729_d_n6;
        locals.var_ratio_pd_ac_dn7 = assign42140_e47729_d_n7;
        locals.var_ratio_pd_ac_dn8 = assign42140_e47729_d_n8;
        locals.var_ratio_pd_ac_dn9 = assign42140_e47729_d_n9;

        let (assign42150_e47734, assign42150_e47734_d_n4, assign42150_e47734_d_n6, assign42150_e47734_d_n7, assign42150_e47734_d_n8, assign42150_e47734_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_qi1m_dc, locals.var_qi1m_dc_dn4, locals.var_qi1m_dc_dn6, locals.var_qi1m_dc_dn7, locals.var_qi1m_dc_dn8, locals.var_qi1m_dc_dn9,)
    } else {
        (locals.var_qi1m_ac, locals.var_qi1m_ac_dn4, locals.var_qi1m_ac_dn6, locals.var_qi1m_ac_dn7, locals.var_qi1m_ac_dn8, locals.var_qi1m_ac_dn9,)
    }
};
        locals.var_qi1m_ac = assign42150_e47734;
        locals.var_qi1m_ac_dn4 = assign42150_e47734_d_n4;
        locals.var_qi1m_ac_dn6 = assign42150_e47734_d_n6;
        locals.var_qi1m_ac_dn7 = assign42150_e47734_d_n7;
        locals.var_qi1m_ac_dn8 = assign42150_e47734_d_n8;
        locals.var_qi1m_ac_dn9 = assign42150_e47734_d_n9;

        let (assign42160_e47739, assign42160_e47739_d_n4, assign42160_e47739_d_n6, assign42160_e47739_d_n7, assign42160_e47739_d_n8, assign42160_e47739_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_qi2m_dc, locals.var_qi2m_dc_dn4, locals.var_qi2m_dc_dn6, locals.var_qi2m_dc_dn7, locals.var_qi2m_dc_dn8, locals.var_qi2m_dc_dn9,)
    } else {
        (locals.var_qi2m_ac, locals.var_qi2m_ac_dn4, locals.var_qi2m_ac_dn6, locals.var_qi2m_ac_dn7, locals.var_qi2m_ac_dn8, locals.var_qi2m_ac_dn9,)
    }
};
        locals.var_qi2m_ac = assign42160_e47739;
        locals.var_qi2m_ac_dn4 = assign42160_e47739_d_n4;
        locals.var_qi2m_ac_dn6 = assign42160_e47739_d_n6;
        locals.var_qi2m_ac_dn7 = assign42160_e47739_d_n7;
        locals.var_qi2m_ac_dn8 = assign42160_e47739_d_n8;
        locals.var_qi2m_ac_dn9 = assign42160_e47739_d_n9;

        let (assign42170_e47744, assign42170_e47744_d_n4, assign42170_e47744_d_n6, assign42170_e47744_d_n7, assign42170_e47744_d_n8, assign42170_e47744_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_zsat_dc, locals.var_zsat_dc_dn4, locals.var_zsat_dc_dn6, locals.var_zsat_dc_dn7, locals.var_zsat_dc_dn8, locals.var_zsat_dc_dn9,)
    } else {
        (locals.var_zsat_ac, locals.var_zsat_ac_dn4, locals.var_zsat_ac_dn6, locals.var_zsat_ac_dn7, locals.var_zsat_ac_dn8, locals.var_zsat_ac_dn9,)
    }
};
        locals.var_zsat_ac = assign42170_e47744;
        locals.var_zsat_ac_dn4 = assign42170_e47744_d_n4;
        locals.var_zsat_ac_dn6 = assign42170_e47744_d_n6;
        locals.var_zsat_ac_dn7 = assign42170_e47744_d_n7;
        locals.var_zsat_ac_dn8 = assign42170_e47744_d_n8;
        locals.var_zsat_ac_dn9 = assign42170_e47744_d_n9;

        let (assign42180_e47749, assign42180_e47749_d_n4, assign42180_e47749_d_n6, assign42180_e47749_d_n7, assign42180_e47749_d_n8, assign42180_e47749_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_qmfact1_dc, locals.var_qmfact1_dc_dn4, locals.var_qmfact1_dc_dn6, locals.var_qmfact1_dc_dn7, locals.var_qmfact1_dc_dn8, locals.var_qmfact1_dc_dn9,)
    } else {
        (locals.var_qmfact1_ac, locals.var_qmfact1_ac_dn4, locals.var_qmfact1_ac_dn6, locals.var_qmfact1_ac_dn7, locals.var_qmfact1_ac_dn8, locals.var_qmfact1_ac_dn9,)
    }
};
        locals.var_qmfact1_ac = assign42180_e47749;
        locals.var_qmfact1_ac_dn4 = assign42180_e47749_d_n4;
        locals.var_qmfact1_ac_dn6 = assign42180_e47749_d_n6;
        locals.var_qmfact1_ac_dn7 = assign42180_e47749_d_n7;
        locals.var_qmfact1_ac_dn8 = assign42180_e47749_d_n8;
        locals.var_qmfact1_ac_dn9 = assign42180_e47749_d_n9;

        let (assign42190_e47754, assign42190_e47754_d_n4, assign42190_e47754_d_n6, assign42190_e47754_d_n7, assign42190_e47754_d_n8, assign42190_e47754_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_qmfact2_dc, locals.var_qmfact2_dc_dn4, locals.var_qmfact2_dc_dn6, locals.var_qmfact2_dc_dn7, locals.var_qmfact2_dc_dn8, locals.var_qmfact2_dc_dn9,)
    } else {
        (locals.var_qmfact2_ac, locals.var_qmfact2_ac_dn4, locals.var_qmfact2_ac_dn6, locals.var_qmfact2_ac_dn7, locals.var_qmfact2_ac_dn8, locals.var_qmfact2_ac_dn9,)
    }
};
        locals.var_qmfact2_ac = assign42190_e47754;
        locals.var_qmfact2_ac_dn4 = assign42190_e47754_d_n4;
        locals.var_qmfact2_ac_dn6 = assign42190_e47754_d_n6;
        locals.var_qmfact2_ac_dn7 = assign42190_e47754_d_n7;
        locals.var_qmfact2_ac_dn8 = assign42190_e47754_d_n8;
        locals.var_qmfact2_ac_dn9 = assign42190_e47754_d_n9;

        let (assign42200_e47759, assign42200_e47759_d_n4, assign42200_e47759_d_n6, assign42200_e47759_d_n7, assign42200_e47759_d_n8, assign42200_e47759_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_delta_k1q1_dc, locals.var_delta_k1q1_dc_dn4, locals.var_delta_k1q1_dc_dn6, locals.var_delta_k1q1_dc_dn7, locals.var_delta_k1q1_dc_dn8, locals.var_delta_k1q1_dc_dn9,)
    } else {
        (locals.var_delta_k1q1_ac, locals.var_delta_k1q1_ac_dn4, locals.var_delta_k1q1_ac_dn6, locals.var_delta_k1q1_ac_dn7, locals.var_delta_k1q1_ac_dn8, locals.var_delta_k1q1_ac_dn9,)
    }
};
        locals.var_delta_k1q1_ac = assign42200_e47759;
        locals.var_delta_k1q1_ac_dn4 = assign42200_e47759_d_n4;
        locals.var_delta_k1q1_ac_dn6 = assign42200_e47759_d_n6;
        locals.var_delta_k1q1_ac_dn7 = assign42200_e47759_d_n7;
        locals.var_delta_k1q1_ac_dn8 = assign42200_e47759_d_n8;
        locals.var_delta_k1q1_ac_dn9 = assign42200_e47759_d_n9;

        let (assign42210_e47764, assign42210_e47764_d_n4, assign42210_e47764_d_n6, assign42210_e47764_d_n7, assign42210_e47764_d_n8, assign42210_e47764_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_delta_k2q2_dc, locals.var_delta_k2q2_dc_dn4, locals.var_delta_k2q2_dc_dn6, locals.var_delta_k2q2_dc_dn7, locals.var_delta_k2q2_dc_dn8, locals.var_delta_k2q2_dc_dn9,)
    } else {
        (locals.var_delta_k2q2_ac, locals.var_delta_k2q2_ac_dn4, locals.var_delta_k2q2_ac_dn6, locals.var_delta_k2q2_ac_dn7, locals.var_delta_k2q2_ac_dn8, locals.var_delta_k2q2_ac_dn9,)
    }
};
        locals.var_delta_k2q2_ac = assign42210_e47764;
        locals.var_delta_k2q2_ac_dn4 = assign42210_e47764_d_n4;
        locals.var_delta_k2q2_ac_dn6 = assign42210_e47764_d_n6;
        locals.var_delta_k2q2_ac_dn7 = assign42210_e47764_d_n7;
        locals.var_delta_k2q2_ac_dn8 = assign42210_e47764_d_n8;
        locals.var_delta_k2q2_ac_dn9 = assign42210_e47764_d_n9;

        let (assign42220_e47769, assign42220_e47769_d_n4, assign42220_e47769_d_n6, assign42220_e47769_d_n7, assign42220_e47769_d_n8, assign42220_e47769_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_prod1_dc, locals.var_prod1_dc_dn4, locals.var_prod1_dc_dn6, locals.var_prod1_dc_dn7, locals.var_prod1_dc_dn8, locals.var_prod1_dc_dn9,)
    } else {
        (locals.var_prod1_ac, locals.var_prod1_ac_dn4, locals.var_prod1_ac_dn6, locals.var_prod1_ac_dn7, locals.var_prod1_ac_dn8, locals.var_prod1_ac_dn9,)
    }
};
        locals.var_prod1_ac = assign42220_e47769;
        locals.var_prod1_ac_dn4 = assign42220_e47769_d_n4;
        locals.var_prod1_ac_dn6 = assign42220_e47769_d_n6;
        locals.var_prod1_ac_dn7 = assign42220_e47769_d_n7;
        locals.var_prod1_ac_dn8 = assign42220_e47769_d_n8;
        locals.var_prod1_ac_dn9 = assign42220_e47769_d_n9;

        let (assign42230_e47774, assign42230_e47774_d_n4, assign42230_e47774_d_n6, assign42230_e47774_d_n7, assign42230_e47774_d_n8, assign42230_e47774_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_prod2_dc, locals.var_prod2_dc_dn4, locals.var_prod2_dc_dn6, locals.var_prod2_dc_dn7, locals.var_prod2_dc_dn8, locals.var_prod2_dc_dn9,)
    } else {
        (locals.var_prod2_ac, locals.var_prod2_ac_dn4, locals.var_prod2_ac_dn6, locals.var_prod2_ac_dn7, locals.var_prod2_ac_dn8, locals.var_prod2_ac_dn9,)
    }
};
        locals.var_prod2_ac = assign42230_e47774;
        locals.var_prod2_ac_dn4 = assign42230_e47774_d_n4;
        locals.var_prod2_ac_dn6 = assign42230_e47774_d_n6;
        locals.var_prod2_ac_dn7 = assign42230_e47774_d_n7;
        locals.var_prod2_ac_dn8 = assign42230_e47774_d_n8;
        locals.var_prod2_ac_dn9 = assign42230_e47774_d_n9;

        let assign42240_e47778: f64 = (locals.var_dx_wi_1d_ac - locals.var_dx_wi_ac);
        let assign42240_e47779: f64 = (locals.var_fsceac_i * assign42240_e47778);
        let assign42240_e47783: f64 = (0.25 * locals.var_qim_ac);
        let assign42240_e47784: f64 = (1.0 + assign42240_e47783);
        let assign42240_e47785: f64 = (assign42240_e47779 / assign42240_e47784);
        locals.var_temp = assign42240_e47785;
        locals.var_temp_dn4 = ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn4 - locals.var_dx_wi_ac_dn4)) * assign42240_e47784) - (assign42240_e47779 * (0.25 * locals.var_qim_ac_dn4))) / (assign42240_e47784 * assign42240_e47784));
        locals.var_temp_dn6 = ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn6 - locals.var_dx_wi_ac_dn6)) * assign42240_e47784) - (assign42240_e47779 * (0.25 * locals.var_qim_ac_dn6))) / (assign42240_e47784 * assign42240_e47784));
        locals.var_temp_dn7 = ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn7 - locals.var_dx_wi_ac_dn7)) * assign42240_e47784) - (assign42240_e47779 * (0.25 * locals.var_qim_ac_dn7))) / (assign42240_e47784 * assign42240_e47784));
        locals.var_temp_dn8 = ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn8 - locals.var_dx_wi_ac_dn8)) * assign42240_e47784) - (assign42240_e47779 * (0.25 * locals.var_qim_ac_dn8))) / (assign42240_e47784 * assign42240_e47784));
        locals.var_temp_dn9 = ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn9 - locals.var_dx_wi_ac_dn9)) * assign42240_e47784) - (assign42240_e47779 * (0.25 * locals.var_qim_ac_dn9))) / (assign42240_e47784 * assign42240_e47784));

        let assign42250_e47789: f64 = (locals.var_k1q1s_ac + locals.var_k1q1d_ac);
        let assign42250_e47790: f64 = (0.5 * assign42250_e47789);
        let assign42250_e47792: f64 = (assign42250_e47790 + locals.var_temp);
        locals.var_k1q1m = assign42250_e47792;
        locals.var_k1q1m_dn4 = ((0.5 * (locals.var_k1q1s_ac_dn4 + locals.var_k1q1d_ac_dn4)) + locals.var_temp_dn4);
        locals.var_k1q1m_dn6 = ((0.5 * (locals.var_k1q1s_ac_dn6 + locals.var_k1q1d_ac_dn6)) + locals.var_temp_dn6);
        locals.var_k1q1m_dn7 = ((0.5 * (locals.var_k1q1s_ac_dn7 + locals.var_k1q1d_ac_dn7)) + locals.var_temp_dn7);
        locals.var_k1q1m_dn8 = ((0.5 * (locals.var_k1q1s_ac_dn8 + locals.var_k1q1d_ac_dn8)) + locals.var_temp_dn8);
        locals.var_k1q1m_dn9 = ((0.5 * (locals.var_k1q1s_ac_dn9 + locals.var_k1q1d_ac_dn9)) + locals.var_temp_dn9);

        let assign42260_e47796: f64 = (locals.var_k2q2s_ac + locals.var_k2q2d_ac);
        let assign42260_e47797: f64 = (0.5 * assign42260_e47796);
        let assign42260_e47799: f64 = (assign42260_e47797 - locals.var_temp);
        locals.var_k2q2m = assign42260_e47799;
        locals.var_k2q2m_dn4 = ((0.5 * (locals.var_k2q2s_ac_dn4 + locals.var_k2q2d_ac_dn4)) - locals.var_temp_dn4);
        locals.var_k2q2m_dn6 = ((0.5 * (locals.var_k2q2s_ac_dn6 + locals.var_k2q2d_ac_dn6)) - locals.var_temp_dn6);
        locals.var_k2q2m_dn7 = ((0.5 * (locals.var_k2q2s_ac_dn7 + locals.var_k2q2d_ac_dn7)) - locals.var_temp_dn7);
        locals.var_k2q2m_dn8 = ((0.5 * (locals.var_k2q2s_ac_dn8 + locals.var_k2q2d_ac_dn8)) - locals.var_temp_dn8);
        locals.var_k2q2m_dn9 = ((0.5 * (locals.var_k2q2s_ac_dn9 + locals.var_k2q2d_ac_dn9)) - locals.var_temp_dn9);

        let assign42270_e47802: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1234 = assign42270_e47802;

        let (assign42280_e47812, assign42280_e47812_d_n4, assign42280_e47812_d_n6, assign42280_e47812_d_n7, assign42280_e47812_d_n8, assign42280_e47812_d_n9,) = {
    if (locals.var_guard1234 != 0.0) {
        let assign42280_e47807: f64 = (locals.var_qi1m_ac / locals.var_qmfact1_ac);
        let assign42280_e47808: f64 = (locals.var_k1q1m + assign42280_e47807);
        let assign42280_e47810: f64 = (assign42280_e47808 - locals.var_qi1m_ac);
        (assign42280_e47810, ((locals.var_k1q1m_dn4 + (((locals.var_qi1m_ac_dn4 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn4)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn4), ((locals.var_k1q1m_dn6 + (((locals.var_qi1m_ac_dn6 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn6)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn6), ((locals.var_k1q1m_dn7 + (((locals.var_qi1m_ac_dn7 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn7)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn7), ((locals.var_k1q1m_dn8 + (((locals.var_qi1m_ac_dn8 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn8)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn8), ((locals.var_k1q1m_dn9 + (((locals.var_qi1m_ac_dn9 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn9)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn9),)
    } else {
        (locals.var_k1q1eff, locals.var_k1q1eff_dn4, locals.var_k1q1eff_dn6, locals.var_k1q1eff_dn7, locals.var_k1q1eff_dn8, locals.var_k1q1eff_dn9,)
    }
};
        locals.var_k1q1eff = assign42280_e47812;
        locals.var_k1q1eff_dn4 = assign42280_e47812_d_n4;
        locals.var_k1q1eff_dn6 = assign42280_e47812_d_n6;
        locals.var_k1q1eff_dn7 = assign42280_e47812_d_n7;
        locals.var_k1q1eff_dn8 = assign42280_e47812_d_n8;
        locals.var_k1q1eff_dn9 = assign42280_e47812_d_n9;

        let (assign42290_e47822, assign42290_e47822_d_n4, assign42290_e47822_d_n6, assign42290_e47822_d_n7, assign42290_e47822_d_n8, assign42290_e47822_d_n9,) = {
    if (locals.var_guard1234 != 0.0) {
        let assign42290_e47817: f64 = (locals.var_qi2m_ac / locals.var_qmfact2_ac);
        let assign42290_e47818: f64 = (locals.var_k2q2m + assign42290_e47817);
        let assign42290_e47820: f64 = (assign42290_e47818 - locals.var_qi2m_ac);
        (assign42290_e47820, ((locals.var_k2q2m_dn4 + (((locals.var_qi2m_ac_dn4 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn4)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn4), ((locals.var_k2q2m_dn6 + (((locals.var_qi2m_ac_dn6 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn6)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn6), ((locals.var_k2q2m_dn7 + (((locals.var_qi2m_ac_dn7 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn7)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn7), ((locals.var_k2q2m_dn8 + (((locals.var_qi2m_ac_dn8 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn8)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn8), ((locals.var_k2q2m_dn9 + (((locals.var_qi2m_ac_dn9 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn9)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn9),)
    } else {
        (locals.var_k2q2eff, locals.var_k2q2eff_dn4, locals.var_k2q2eff_dn6, locals.var_k2q2eff_dn7, locals.var_k2q2eff_dn8, locals.var_k2q2eff_dn9,)
    }
};
        locals.var_k2q2eff = assign42290_e47822;
        locals.var_k2q2eff_dn4 = assign42290_e47822_d_n4;
        locals.var_k2q2eff_dn6 = assign42290_e47822_d_n6;
        locals.var_k2q2eff_dn7 = assign42290_e47822_d_n7;
        locals.var_k2q2eff_dn8 = assign42290_e47822_d_n8;
        locals.var_k2q2eff_dn9 = assign42290_e47822_d_n9;

        let (assign42300_e47827, assign42300_e47827_d_n4, assign42300_e47827_d_n6, assign42300_e47827_d_n7, assign42300_e47827_d_n8, assign42300_e47827_d_n9,) = {
    if (locals.var_guard1234 == 0.0) {
        (locals.var_k1q1m, locals.var_k1q1m_dn4, locals.var_k1q1m_dn6, locals.var_k1q1m_dn7, locals.var_k1q1m_dn8, locals.var_k1q1m_dn9,)
    } else {
        (locals.var_k1q1eff, locals.var_k1q1eff_dn4, locals.var_k1q1eff_dn6, locals.var_k1q1eff_dn7, locals.var_k1q1eff_dn8, locals.var_k1q1eff_dn9,)
    }
};
        locals.var_k1q1eff = assign42300_e47827;
        locals.var_k1q1eff_dn4 = assign42300_e47827_d_n4;
        locals.var_k1q1eff_dn6 = assign42300_e47827_d_n6;
        locals.var_k1q1eff_dn7 = assign42300_e47827_d_n7;
        locals.var_k1q1eff_dn8 = assign42300_e47827_d_n8;
        locals.var_k1q1eff_dn9 = assign42300_e47827_d_n9;

        let (assign42310_e47832, assign42310_e47832_d_n4, assign42310_e47832_d_n6, assign42310_e47832_d_n7, assign42310_e47832_d_n8, assign42310_e47832_d_n9,) = {
    if (locals.var_guard1234 == 0.0) {
        (locals.var_k2q2m, locals.var_k2q2m_dn4, locals.var_k2q2m_dn6, locals.var_k2q2m_dn7, locals.var_k2q2m_dn8, locals.var_k2q2m_dn9,)
    } else {
        (locals.var_k2q2eff, locals.var_k2q2eff_dn4, locals.var_k2q2eff_dn6, locals.var_k2q2eff_dn7, locals.var_k2q2eff_dn8, locals.var_k2q2eff_dn9,)
    }
};
        locals.var_k2q2eff = assign42310_e47832;
        locals.var_k2q2eff_dn4 = assign42310_e47832_d_n4;
        locals.var_k2q2eff_dn6 = assign42310_e47832_d_n6;
        locals.var_k2q2eff_dn7 = assign42310_e47832_d_n7;
        locals.var_k2q2eff_dn8 = assign42310_e47832_d_n8;
        locals.var_k2q2eff_dn9 = assign42310_e47832_d_n9;

        let assign42320_e47835: f64 = (locals.var_delta_k1q1_ac * locals.var_prod1_ac);
        let assign42320_e47837: f64 = (assign42320_e47835 * 0.3333333333333);
        locals.var_temp1 = assign42320_e47837;
        locals.var_temp1_dn4 = (((locals.var_delta_k1q1_ac_dn4 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn4)) * 0.3333333333333);
        locals.var_temp1_dn6 = (((locals.var_delta_k1q1_ac_dn6 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn6)) * 0.3333333333333);
        locals.var_temp1_dn7 = (((locals.var_delta_k1q1_ac_dn7 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn7)) * 0.3333333333333);
        locals.var_temp1_dn8 = (((locals.var_delta_k1q1_ac_dn8 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn8)) * 0.3333333333333);
        locals.var_temp1_dn9 = (((locals.var_delta_k1q1_ac_dn9 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn9)) * 0.3333333333333);

        let assign42330_e47840: f64 = (locals.var_delta_k1q1_ac * 0.1666666666667);
        let assign42330_e47846: f64 = (0.2 * locals.var_prod1_ac);
        let assign42330_e47847: f64 = (1.0 - assign42330_e47846);
        let assign42330_e47848: f64 = (locals.var_prod1_ac * assign42330_e47847);
        let assign42330_e47849: f64 = (1.0 + assign42330_e47848);
        let assign42330_e47850: f64 = (assign42330_e47840 * assign42330_e47849);
        locals.var_temp2 = assign42330_e47850;
        locals.var_temp2_dn4 = (((locals.var_delta_k1q1_ac_dn4 * 0.1666666666667) * assign42330_e47849) + (assign42330_e47840 * ((locals.var_prod1_ac_dn4 * assign42330_e47847) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn4))))));
        locals.var_temp2_dn6 = (((locals.var_delta_k1q1_ac_dn6 * 0.1666666666667) * assign42330_e47849) + (assign42330_e47840 * ((locals.var_prod1_ac_dn6 * assign42330_e47847) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn6))))));
        locals.var_temp2_dn7 = (((locals.var_delta_k1q1_ac_dn7 * 0.1666666666667) * assign42330_e47849) + (assign42330_e47840 * ((locals.var_prod1_ac_dn7 * assign42330_e47847) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn7))))));
        locals.var_temp2_dn8 = (((locals.var_delta_k1q1_ac_dn8 * 0.1666666666667) * assign42330_e47849) + (assign42330_e47840 * ((locals.var_prod1_ac_dn8 * assign42330_e47847) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn8))))));
        locals.var_temp2_dn9 = (((locals.var_delta_k1q1_ac_dn9 * 0.1666666666667) * assign42330_e47849) + (assign42330_e47840 * ((locals.var_prod1_ac_dn9 * assign42330_e47847) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn9))))));

        let assign42340_e47853: f64 = (0.5 * locals.var_k1q1eff);
        let assign42340_e47855: f64 = (assign42340_e47853 * locals.var_ratio_pd_ac);
        let assign42340_e47857: f64 = (assign42340_e47855 + locals.var_temp2);
        locals.var_k1q1deff = assign42340_e47857;
        locals.var_k1q1deff_dn4 = ((((0.5 * locals.var_k1q1eff_dn4) * locals.var_ratio_pd_ac) + (assign42340_e47853 * locals.var_ratio_pd_ac_dn4)) + locals.var_temp2_dn4);
        locals.var_k1q1deff_dn6 = ((((0.5 * locals.var_k1q1eff_dn6) * locals.var_ratio_pd_ac) + (assign42340_e47853 * locals.var_ratio_pd_ac_dn6)) + locals.var_temp2_dn6);
        locals.var_k1q1deff_dn7 = ((((0.5 * locals.var_k1q1eff_dn7) * locals.var_ratio_pd_ac) + (assign42340_e47853 * locals.var_ratio_pd_ac_dn7)) + locals.var_temp2_dn7);
        locals.var_k1q1deff_dn8 = ((((0.5 * locals.var_k1q1eff_dn8) * locals.var_ratio_pd_ac) + (assign42340_e47853 * locals.var_ratio_pd_ac_dn8)) + locals.var_temp2_dn8);
        locals.var_k1q1deff_dn9 = ((((0.5 * locals.var_k1q1eff_dn9) * locals.var_ratio_pd_ac) + (assign42340_e47853 * locals.var_ratio_pd_ac_dn9)) + locals.var_temp2_dn9);

        let assign42350_e47860: f64 = (locals.var_k1q1eff * locals.var_ratio_pd_ac);
        let assign42350_e47862: f64 = (assign42350_e47860 + locals.var_temp1);
        locals.var_k1q1eff = assign42350_e47862;
        locals.var_k1q1eff_dn4 = (((locals.var_k1q1eff_dn4 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn4)) + locals.var_temp1_dn4);
        locals.var_k1q1eff_dn6 = (((locals.var_k1q1eff_dn6 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn6)) + locals.var_temp1_dn6);
        locals.var_k1q1eff_dn7 = (((locals.var_k1q1eff_dn7 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn7)) + locals.var_temp1_dn7);
        locals.var_k1q1eff_dn8 = (((locals.var_k1q1eff_dn8 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn8)) + locals.var_temp1_dn8);
        locals.var_k1q1eff_dn9 = (((locals.var_k1q1eff_dn9 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn9)) + locals.var_temp1_dn9);

        let assign42360_e47865: f64 = (locals.var_delta_k2q2_ac * locals.var_prod2_ac);
        let assign42360_e47867: f64 = (assign42360_e47865 * 0.3333333333333);
        locals.var_temp1 = assign42360_e47867;
        locals.var_temp1_dn4 = (((locals.var_delta_k2q2_ac_dn4 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn4)) * 0.3333333333333);
        locals.var_temp1_dn6 = (((locals.var_delta_k2q2_ac_dn6 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn6)) * 0.3333333333333);
        locals.var_temp1_dn7 = (((locals.var_delta_k2q2_ac_dn7 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn7)) * 0.3333333333333);
        locals.var_temp1_dn8 = (((locals.var_delta_k2q2_ac_dn8 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn8)) * 0.3333333333333);
        locals.var_temp1_dn9 = (((locals.var_delta_k2q2_ac_dn9 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn9)) * 0.3333333333333);

        let assign42370_e47870: f64 = (locals.var_delta_k2q2_ac * 0.1666666666667);
        let assign42370_e47876: f64 = (0.2 * locals.var_prod2_ac);
        let assign42370_e47877: f64 = (1.0 - assign42370_e47876);
        let assign42370_e47878: f64 = (locals.var_prod2_ac * assign42370_e47877);
        let assign42370_e47879: f64 = (1.0 + assign42370_e47878);
        let assign42370_e47880: f64 = (assign42370_e47870 * assign42370_e47879);
        locals.var_temp2 = assign42370_e47880;
        locals.var_temp2_dn4 = (((locals.var_delta_k2q2_ac_dn4 * 0.1666666666667) * assign42370_e47879) + (assign42370_e47870 * ((locals.var_prod2_ac_dn4 * assign42370_e47877) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn4))))));
        locals.var_temp2_dn6 = (((locals.var_delta_k2q2_ac_dn6 * 0.1666666666667) * assign42370_e47879) + (assign42370_e47870 * ((locals.var_prod2_ac_dn6 * assign42370_e47877) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn6))))));
        locals.var_temp2_dn7 = (((locals.var_delta_k2q2_ac_dn7 * 0.1666666666667) * assign42370_e47879) + (assign42370_e47870 * ((locals.var_prod2_ac_dn7 * assign42370_e47877) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn7))))));
        locals.var_temp2_dn8 = (((locals.var_delta_k2q2_ac_dn8 * 0.1666666666667) * assign42370_e47879) + (assign42370_e47870 * ((locals.var_prod2_ac_dn8 * assign42370_e47877) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn8))))));
        locals.var_temp2_dn9 = (((locals.var_delta_k2q2_ac_dn9 * 0.1666666666667) * assign42370_e47879) + (assign42370_e47870 * ((locals.var_prod2_ac_dn9 * assign42370_e47877) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn9))))));

        let assign42380_e47883: f64 = (0.5 * locals.var_k2q2eff);
        let assign42380_e47885: f64 = (assign42380_e47883 + locals.var_temp2);
        locals.var_k2q2deff = assign42380_e47885;
        locals.var_k2q2deff_dn4 = ((0.5 * locals.var_k2q2eff_dn4) + locals.var_temp2_dn4);
        locals.var_k2q2deff_dn6 = ((0.5 * locals.var_k2q2eff_dn6) + locals.var_temp2_dn6);
        locals.var_k2q2deff_dn7 = ((0.5 * locals.var_k2q2eff_dn7) + locals.var_temp2_dn7);
        locals.var_k2q2deff_dn8 = ((0.5 * locals.var_k2q2eff_dn8) + locals.var_temp2_dn8);
        locals.var_k2q2deff_dn9 = ((0.5 * locals.var_k2q2eff_dn9) + locals.var_temp2_dn9);

        let assign42390_e47888: f64 = (locals.var_k2q2eff + locals.var_temp1);
        locals.var_k2q2eff = assign42390_e47888;
        locals.var_k2q2eff_dn4 = (locals.var_k2q2eff_dn4 + locals.var_temp1_dn4);
        locals.var_k2q2eff_dn6 = (locals.var_k2q2eff_dn6 + locals.var_temp1_dn6);
        locals.var_k2q2eff_dn7 = (locals.var_k2q2eff_dn7 + locals.var_temp1_dn7);
        locals.var_k2q2eff_dn8 = (locals.var_k2q2eff_dn8 + locals.var_temp1_dn8);
        locals.var_k2q2eff_dn9 = (locals.var_k2q2eff_dn9 + locals.var_temp1_dn9);

        let assign42400_e47891: f64 = (locals.var_csiprime_ac * locals.var_area_phit);
        locals.var_temp = assign42400_e47891;
        locals.var_temp_dn4 = ((locals.var_csiprime_ac_dn4 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn4));
        locals.var_temp_dn6 = ((locals.var_csiprime_ac_dn6 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn6));
        locals.var_temp_dn7 = ((locals.var_csiprime_ac_dn7 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn7));
        locals.var_temp_dn8 = ((locals.var_csiprime_ac_dn8 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn8));
        locals.var_temp_dn9 = ((locals.var_csiprime_ac_dn9 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn9));

        let assign42410_e47894: f64 = (locals.var_temp * locals.var_k1q1eff);
        locals.var_qg = assign42410_e47894;
        locals.var_qg_dn4 = ((locals.var_temp_dn4 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn4));
        locals.var_qg_dn6 = ((locals.var_temp_dn6 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn6));
        locals.var_qg_dn7 = ((locals.var_temp_dn7 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn7));
        locals.var_qg_dn8 = ((locals.var_temp_dn8 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn8));
        locals.var_qg_dn9 = ((locals.var_temp_dn9 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn9));

        let assign42420_e47897: f64 = (locals.var_temp * locals.var_k2q2eff);
        locals.var_qb = assign42420_e47897;
        locals.var_qb_dn4 = ((locals.var_temp_dn4 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn4));
        locals.var_qb_dn6 = ((locals.var_temp_dn6 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn6));
        locals.var_qb_dn7 = ((locals.var_temp_dn7 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn7));
        locals.var_qb_dn8 = ((locals.var_temp_dn8 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn8));
        locals.var_qb_dn9 = ((locals.var_temp_dn9 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn9));

        let assign42430_e47899: f64 = (-locals.var_temp);
        let assign42430_e47902: f64 = (locals.var_k1q1deff + locals.var_k2q2deff);
        let assign42430_e47903: f64 = (assign42430_e47899 * assign42430_e47902);
        locals.var_qd = assign42430_e47903;
        locals.var_qd_dn4 = (((-locals.var_temp_dn4) * assign42430_e47902) + (assign42430_e47899 * (locals.var_k1q1deff_dn4 + locals.var_k2q2deff_dn4)));
        locals.var_qd_dn6 = (((-locals.var_temp_dn6) * assign42430_e47902) + (assign42430_e47899 * (locals.var_k1q1deff_dn6 + locals.var_k2q2deff_dn6)));
        locals.var_qd_dn7 = (((-locals.var_temp_dn7) * assign42430_e47902) + (assign42430_e47899 * (locals.var_k1q1deff_dn7 + locals.var_k2q2deff_dn7)));
        locals.var_qd_dn8 = (((-locals.var_temp_dn8) * assign42430_e47902) + (assign42430_e47899 * (locals.var_k1q1deff_dn8 + locals.var_k2q2deff_dn8)));
        locals.var_qd_dn9 = (((-locals.var_temp_dn9) * assign42430_e47902) + (assign42430_e47899 * (locals.var_k1q1deff_dn9 + locals.var_k2q2deff_dn9)));

        let assign42440_e47906: f64 = if locals.var_fif_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1235 = assign42440_e47906;

        let (assign42450_e47914, assign42450_e47914_d_n4, assign42450_e47914_d_n6, assign42450_e47914_d_n7, assign42450_e47914_d_n8, assign42450_e47914_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42450_e47911: f64 = (2.0 * 0.6931471805599);
        let assign42450_e47912: f64 = (locals.var_xth_1d + assign42450_e47911);
        (assign42450_e47912, locals.var_xth_1d_dn4, locals.var_xth_1d_dn6, locals.var_xth_1d_dn7, locals.var_xth_1d_dn8, locals.var_xth_1d_dn9,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign42450_e47914;
        locals.var_temp_dn4 = assign42450_e47914_d_n4;
        locals.var_temp_dn6 = assign42450_e47914_d_n6;
        locals.var_temp_dn7 = assign42450_e47914_d_n7;
        locals.var_temp_dn8 = assign42450_e47914_d_n8;
        locals.var_temp_dn9 = assign42450_e47914_d_n9;

        let (assign42460_e47920, assign42460_e47920_d_n4, assign42460_e47920_d_n6, assign42460_e47920_d_n7, assign42460_e47920_d_n8, assign42460_e47920_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42460_e47918: f64 = (locals.var_xdrifts_ac + locals.var_temp);
        (assign42460_e47918, (locals.var_xdrifts_ac_dn4 + locals.var_temp_dn4), (locals.var_xdrifts_ac_dn6 + locals.var_temp_dn6), (locals.var_xdrifts_ac_dn7 + locals.var_temp_dn7), (locals.var_xdrifts_ac_dn8 + locals.var_temp_dn8), (locals.var_xdrifts_ac_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_xeffs, locals.var_xeffs_dn4, locals.var_xeffs_dn6, locals.var_xeffs_dn7, locals.var_xeffs_dn8, locals.var_xeffs_dn9,)
    }
};
        locals.var_xeffs = assign42460_e47920;
        locals.var_xeffs_dn4 = assign42460_e47920_d_n4;
        locals.var_xeffs_dn6 = assign42460_e47920_d_n6;
        locals.var_xeffs_dn7 = assign42460_e47920_d_n7;
        locals.var_xeffs_dn8 = assign42460_e47920_d_n8;
        locals.var_xeffs_dn9 = assign42460_e47920_d_n9;

    }

    pub(super) fn stamp_transient_block_118(
        locals: &mut StampLocals,
    ) {
        let (assign42470_e47926, assign42470_e47926_d_n4, assign42470_e47926_d_n6, assign42470_e47926_d_n7, assign42470_e47926_d_n8, assign42470_e47926_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42470_e47924: f64 = (locals.var_xdriftd_ac + locals.var_temp);
        (assign42470_e47924, (locals.var_xdriftd_ac_dn4 + locals.var_temp_dn4), (locals.var_xdriftd_ac_dn6 + locals.var_temp_dn6), (locals.var_xdriftd_ac_dn7 + locals.var_temp_dn7), (locals.var_xdriftd_ac_dn8 + locals.var_temp_dn8), (locals.var_xdriftd_ac_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_xeffd, locals.var_xeffd_dn4, locals.var_xeffd_dn6, locals.var_xeffd_dn7, locals.var_xeffd_dn8, locals.var_xeffd_dn9,)
    }
};
        locals.var_xeffd = assign42470_e47926;
        locals.var_xeffd_dn4 = assign42470_e47926_d_n4;
        locals.var_xeffd_dn6 = assign42470_e47926_d_n6;
        locals.var_xeffd_dn7 = assign42470_e47926_d_n7;
        locals.var_xeffd_dn8 = assign42470_e47926_d_n8;
        locals.var_xeffd_dn9 = assign42470_e47926_d_n9;

        let (assign42480_e47945, assign42480_e47945_d_n4, assign42480_e47945_d_n6, assign42480_e47945_d_n7, assign42480_e47945_d_n8, assign42480_e47945_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42480_e47931: f64 = (locals.var_xeffs + locals.var_xth_1d);
        let assign42480_e47934: f64 = (locals.var_xeffs - locals.var_xth_1d);
        let assign42480_e47937: f64 = (locals.var_xeffs - locals.var_xth_1d);
        let assign42480_e47938: f64 = (assign42480_e47934 * assign42480_e47937);
        let assign42480_e47940: f64 = (assign42480_e47938 + 9.0);
        let assign42480_e47941: f64 = (assign42480_e47940).sqrt();
        let assign42480_e47942: f64 = (assign42480_e47931 - assign42480_e47941);
        let assign42480_e47943: f64 = (0.5 * assign42480_e47942);
        (assign42480_e47943, (0.5 * ((locals.var_xeffs_dn4 + locals.var_xth_1d_dn4) - ((((locals.var_xeffs_dn4 - locals.var_xth_1d_dn4) * assign42480_e47937) + (assign42480_e47934 * (locals.var_xeffs_dn4 - locals.var_xth_1d_dn4))) / (2.0 * assign42480_e47941)))), (0.5 * ((locals.var_xeffs_dn6 + locals.var_xth_1d_dn6) - ((((locals.var_xeffs_dn6 - locals.var_xth_1d_dn6) * assign42480_e47937) + (assign42480_e47934 * (locals.var_xeffs_dn6 - locals.var_xth_1d_dn6))) / (2.0 * assign42480_e47941)))), (0.5 * ((locals.var_xeffs_dn7 + locals.var_xth_1d_dn7) - ((((locals.var_xeffs_dn7 - locals.var_xth_1d_dn7) * assign42480_e47937) + (assign42480_e47934 * (locals.var_xeffs_dn7 - locals.var_xth_1d_dn7))) / (2.0 * assign42480_e47941)))), (0.5 * ((locals.var_xeffs_dn8 + locals.var_xth_1d_dn8) - ((((locals.var_xeffs_dn8 - locals.var_xth_1d_dn8) * assign42480_e47937) + (assign42480_e47934 * (locals.var_xeffs_dn8 - locals.var_xth_1d_dn8))) / (2.0 * assign42480_e47941)))), (0.5 * ((locals.var_xeffs_dn9 + locals.var_xth_1d_dn9) - ((((locals.var_xeffs_dn9 - locals.var_xth_1d_dn9) * assign42480_e47937) + (assign42480_e47934 * (locals.var_xeffs_dn9 - locals.var_xth_1d_dn9))) / (2.0 * assign42480_e47941)))),)
    } else {
        (locals.var_xstars, locals.var_xstars_dn4, locals.var_xstars_dn6, locals.var_xstars_dn7, locals.var_xstars_dn8, locals.var_xstars_dn9,)
    }
};
        locals.var_xstars = assign42480_e47945;
        locals.var_xstars_dn4 = assign42480_e47945_d_n4;
        locals.var_xstars_dn6 = assign42480_e47945_d_n6;
        locals.var_xstars_dn7 = assign42480_e47945_d_n7;
        locals.var_xstars_dn8 = assign42480_e47945_d_n8;
        locals.var_xstars_dn9 = assign42480_e47945_d_n9;

        let (assign42490_e47970, assign42490_e47970_d_n4, assign42490_e47970_d_n6, assign42490_e47970_d_n7, assign42490_e47970_d_n8, assign42490_e47970_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42490_e47951: f64 = (locals.var_xth_1d + locals.var_xd);
        let assign42490_e47952: f64 = (locals.var_xeffd + assign42490_e47951);
        let assign42490_e47956: f64 = (locals.var_xth_1d + locals.var_xd);
        let assign42490_e47957: f64 = (locals.var_xeffd - assign42490_e47956);
        let assign42490_e47961: f64 = (locals.var_xth_1d + locals.var_xd);
        let assign42490_e47962: f64 = (locals.var_xeffd - assign42490_e47961);
        let assign42490_e47963: f64 = (assign42490_e47957 * assign42490_e47962);
        let assign42490_e47965: f64 = (assign42490_e47963 + 9.0);
        let assign42490_e47966: f64 = (assign42490_e47965).sqrt();
        let assign42490_e47967: f64 = (assign42490_e47952 - assign42490_e47966);
        let assign42490_e47968: f64 = (0.5 * assign42490_e47967);
        (assign42490_e47968, (0.5 * ((locals.var_xeffd_dn4 + (locals.var_xth_1d_dn4 + locals.var_xd_dn4)) - ((((locals.var_xeffd_dn4 - (locals.var_xth_1d_dn4 + locals.var_xd_dn4)) * assign42490_e47962) + (assign42490_e47957 * (locals.var_xeffd_dn4 - (locals.var_xth_1d_dn4 + locals.var_xd_dn4)))) / (2.0 * assign42490_e47966)))), (0.5 * ((locals.var_xeffd_dn6 + (locals.var_xth_1d_dn6 + locals.var_xd_dn6)) - ((((locals.var_xeffd_dn6 - (locals.var_xth_1d_dn6 + locals.var_xd_dn6)) * assign42490_e47962) + (assign42490_e47957 * (locals.var_xeffd_dn6 - (locals.var_xth_1d_dn6 + locals.var_xd_dn6)))) / (2.0 * assign42490_e47966)))), (0.5 * ((locals.var_xeffd_dn7 + (locals.var_xth_1d_dn7 + locals.var_xd_dn7)) - ((((locals.var_xeffd_dn7 - (locals.var_xth_1d_dn7 + locals.var_xd_dn7)) * assign42490_e47962) + (assign42490_e47957 * (locals.var_xeffd_dn7 - (locals.var_xth_1d_dn7 + locals.var_xd_dn7)))) / (2.0 * assign42490_e47966)))), (0.5 * ((locals.var_xeffd_dn8 + (locals.var_xth_1d_dn8 + locals.var_xd_dn8)) - ((((locals.var_xeffd_dn8 - (locals.var_xth_1d_dn8 + locals.var_xd_dn8)) * assign42490_e47962) + (assign42490_e47957 * (locals.var_xeffd_dn8 - (locals.var_xth_1d_dn8 + locals.var_xd_dn8)))) / (2.0 * assign42490_e47966)))), (0.5 * ((locals.var_xeffd_dn9 + (locals.var_xth_1d_dn9 + locals.var_xd_dn9)) - ((((locals.var_xeffd_dn9 - (locals.var_xth_1d_dn9 + locals.var_xd_dn9)) * assign42490_e47962) + (assign42490_e47957 * (locals.var_xeffd_dn9 - (locals.var_xth_1d_dn9 + locals.var_xd_dn9)))) / (2.0 * assign42490_e47966)))),)
    } else {
        (locals.var_xstard, locals.var_xstard_dn4, locals.var_xstard_dn6, locals.var_xstard_dn7, locals.var_xstard_dn8, locals.var_xstard_dn9,)
    }
};
        locals.var_xstard = assign42490_e47970;
        locals.var_xstard_dn4 = assign42490_e47970_d_n4;
        locals.var_xstard_dn6 = assign42490_e47970_d_n6;
        locals.var_xstard_dn7 = assign42490_e47970_d_n7;
        locals.var_xstard_dn8 = assign42490_e47970_d_n8;
        locals.var_xstard_dn9 = assign42490_e47970_d_n9;

        let (assign42500_e47981, assign42500_e47981_d_n4, assign42500_e47981_d_n6, assign42500_e47981_d_n7, assign42500_e47981_d_n8, assign42500_e47981_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42500_e47976: f64 = (0.5 + locals.var_inv_k2_ac);
        let assign42500_e47977: f64 = (locals.var_keq_ac * assign42500_e47976);
        let assign42500_e47978: f64 = (assign42500_e47977).sqrt();
        let assign42500_e47979: f64 = (locals.var_lambda2d * assign42500_e47978);
        (assign42500_e47979, (locals.var_lambda2d * (((locals.var_keq_ac_dn4 * assign42500_e47976) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn4)) / (2.0 * assign42500_e47978))), (locals.var_lambda2d * (((locals.var_keq_ac_dn6 * assign42500_e47976) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn6)) / (2.0 * assign42500_e47978))), (locals.var_lambda2d * (((locals.var_keq_ac_dn7 * assign42500_e47976) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn7)) / (2.0 * assign42500_e47978))), (locals.var_lambda2d * (((locals.var_keq_ac_dn8 * assign42500_e47976) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn8)) / (2.0 * assign42500_e47978))), (locals.var_lambda2d * (((locals.var_keq_ac_dn9 * assign42500_e47976) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn9)) / (2.0 * assign42500_e47978))),)
    } else {
        (locals.var_lambdaf, locals.var_lambdaf_dn4, locals.var_lambdaf_dn6, locals.var_lambdaf_dn7, locals.var_lambdaf_dn8, locals.var_lambdaf_dn9,)
    }
};
        locals.var_lambdaf = assign42500_e47981;
        locals.var_lambdaf_dn4 = assign42500_e47981_d_n4;
        locals.var_lambdaf_dn6 = assign42500_e47981_d_n6;
        locals.var_lambdaf_dn7 = assign42500_e47981_d_n7;
        locals.var_lambdaf_dn8 = assign42500_e47981_d_n8;
        locals.var_lambdaf_dn9 = assign42500_e47981_d_n9;

        let (assign42510_e47996, assign42510_e47996_d_n4, assign42510_e47996_d_n6, assign42510_e47996_d_n7, assign42510_e47996_d_n8, assign42510_e47996_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42510_e47986: f64 = (locals.var_keq_ac * locals.var_k1_ac);
        let assign42510_e47988: f64 = (assign42510_e47986 * locals.var_inv_k2_ac);
        let assign42510_e47991: f64 = (0.5 + locals.var_inv_k1_ac);
        let assign42510_e47992: f64 = (assign42510_e47988 * assign42510_e47991);
        let assign42510_e47993: f64 = (assign42510_e47992).sqrt();
        let assign42510_e47994: f64 = (locals.var_lambda2d * assign42510_e47993);
        (assign42510_e47994, (locals.var_lambda2d * (((((((locals.var_keq_ac_dn4 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn4)) * locals.var_inv_k2_ac) + (assign42510_e47986 * locals.var_inv_k2_ac_dn4)) * assign42510_e47991) + (assign42510_e47988 * locals.var_inv_k1_ac_dn4)) / (2.0 * assign42510_e47993))), (locals.var_lambda2d * (((((((locals.var_keq_ac_dn6 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn6)) * locals.var_inv_k2_ac) + (assign42510_e47986 * locals.var_inv_k2_ac_dn6)) * assign42510_e47991) + (assign42510_e47988 * locals.var_inv_k1_ac_dn6)) / (2.0 * assign42510_e47993))), (locals.var_lambda2d * (((((((locals.var_keq_ac_dn7 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn7)) * locals.var_inv_k2_ac) + (assign42510_e47986 * locals.var_inv_k2_ac_dn7)) * assign42510_e47991) + (assign42510_e47988 * locals.var_inv_k1_ac_dn7)) / (2.0 * assign42510_e47993))), (locals.var_lambda2d * (((((((locals.var_keq_ac_dn8 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn8)) * locals.var_inv_k2_ac) + (assign42510_e47986 * locals.var_inv_k2_ac_dn8)) * assign42510_e47991) + (assign42510_e47988 * locals.var_inv_k1_ac_dn8)) / (2.0 * assign42510_e47993))), (locals.var_lambda2d * (((((((locals.var_keq_ac_dn9 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn9)) * locals.var_inv_k2_ac) + (assign42510_e47986 * locals.var_inv_k2_ac_dn9)) * assign42510_e47991) + (assign42510_e47988 * locals.var_inv_k1_ac_dn9)) / (2.0 * assign42510_e47993))),)
    } else {
        (locals.var_lambdab, locals.var_lambdab_dn4, locals.var_lambdab_dn6, locals.var_lambdab_dn7, locals.var_lambdab_dn8, locals.var_lambdab_dn9,)
    }
};
        locals.var_lambdab = assign42510_e47996;
        locals.var_lambdab_dn4 = assign42510_e47996_d_n4;
        locals.var_lambdab_dn6 = assign42510_e47996_d_n6;
        locals.var_lambdab_dn7 = assign42510_e47996_d_n7;
        locals.var_lambdab_dn8 = assign42510_e47996_d_n8;
        locals.var_lambdab_dn9 = assign42510_e47996_d_n9;

        let (assign42520_e48004, assign42520_e48004_d_n4, assign42520_e48004_d_n6, assign42520_e48004_d_n7, assign42520_e48004_d_n8, assign42520_e48004_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42520_e48000: f64 = (locals.var_lambdaf * locals.var_lambdaf);
        let assign42520_e48002: f64 = (assign42520_e48000 * locals.var_inner_sd);
        (assign42520_e48002, ((((locals.var_lambdaf_dn4 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn4)) * locals.var_inner_sd) + (assign42520_e48000 * locals.var_inner_sd_dn4)), ((((locals.var_lambdaf_dn6 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn6)) * locals.var_inner_sd) + (assign42520_e48000 * locals.var_inner_sd_dn6)), ((((locals.var_lambdaf_dn7 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn7)) * locals.var_inner_sd) + (assign42520_e48000 * locals.var_inner_sd_dn7)), ((((locals.var_lambdaf_dn8 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn8)) * locals.var_inner_sd) + (assign42520_e48000 * locals.var_inner_sd_dn8)), ((((locals.var_lambdaf_dn9 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn9)) * locals.var_inner_sd) + (assign42520_e48000 * locals.var_inner_sd_dn9)),)
    } else {
        (locals.var_xalphaf, locals.var_xalphaf_dn4, locals.var_xalphaf_dn6, locals.var_xalphaf_dn7, locals.var_xalphaf_dn8, locals.var_xalphaf_dn9,)
    }
};
        locals.var_xalphaf = assign42520_e48004;
        locals.var_xalphaf_dn4 = assign42520_e48004_d_n4;
        locals.var_xalphaf_dn6 = assign42520_e48004_d_n6;
        locals.var_xalphaf_dn7 = assign42520_e48004_d_n7;
        locals.var_xalphaf_dn8 = assign42520_e48004_d_n8;
        locals.var_xalphaf_dn9 = assign42520_e48004_d_n9;

        let (assign42530_e48012, assign42530_e48012_d_n4, assign42530_e48012_d_n6, assign42530_e48012_d_n7, assign42530_e48012_d_n8, assign42530_e48012_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42530_e48008: f64 = (locals.var_lambdab * locals.var_lambdab);
        let assign42530_e48010: f64 = (assign42530_e48008 * locals.var_inner_sd);
        (assign42530_e48010, ((((locals.var_lambdab_dn4 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn4)) * locals.var_inner_sd) + (assign42530_e48008 * locals.var_inner_sd_dn4)), ((((locals.var_lambdab_dn6 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn6)) * locals.var_inner_sd) + (assign42530_e48008 * locals.var_inner_sd_dn6)), ((((locals.var_lambdab_dn7 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn7)) * locals.var_inner_sd) + (assign42530_e48008 * locals.var_inner_sd_dn7)), ((((locals.var_lambdab_dn8 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn8)) * locals.var_inner_sd) + (assign42530_e48008 * locals.var_inner_sd_dn8)), ((((locals.var_lambdab_dn9 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn9)) * locals.var_inner_sd) + (assign42530_e48008 * locals.var_inner_sd_dn9)),)
    } else {
        (locals.var_xalphab, locals.var_xalphab_dn4, locals.var_xalphab_dn6, locals.var_xalphab_dn7, locals.var_xalphab_dn8, locals.var_xalphab_dn9,)
    }
};
        locals.var_xalphab = assign42530_e48012;
        locals.var_xalphab_dn4 = assign42530_e48012_d_n4;
        locals.var_xalphab_dn6 = assign42530_e48012_d_n6;
        locals.var_xalphab_dn7 = assign42530_e48012_d_n7;
        locals.var_xalphab_dn8 = assign42530_e48012_d_n8;
        locals.var_xalphab_dn9 = assign42530_e48012_d_n9;

        let (assign42540_e48018, assign42540_e48018_d_n4, assign42540_e48018_d_n6, assign42540_e48018_d_n7, assign42540_e48018_d_n8, assign42540_e48018_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42540_e48016: f64 = (locals.var_xsd - locals.var_xstars);
        (assign42540_e48016, (locals.var_xsd_dn4 - locals.var_xstars_dn4), (locals.var_xsd_dn6 - locals.var_xstars_dn6), (locals.var_xsd_dn7 - locals.var_xstars_dn7), (locals.var_xsd_dn8 - locals.var_xstars_dn8), (locals.var_xsd_dn9 - locals.var_xstars_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign42540_e48018;
        locals.var_temp1_dn4 = assign42540_e48018_d_n4;
        locals.var_temp1_dn6 = assign42540_e48018_d_n6;
        locals.var_temp1_dn7 = assign42540_e48018_d_n7;
        locals.var_temp1_dn8 = assign42540_e48018_d_n8;
        locals.var_temp1_dn9 = assign42540_e48018_d_n9;

        let (assign42550_e48026, assign42550_e48026_d_n4, assign42550_e48026_d_n6, assign42550_e48026_d_n7, assign42550_e48026_d_n8, assign42550_e48026_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42550_e48022: f64 = (locals.var_xsd + locals.var_xd);
        let assign42550_e48024: f64 = (assign42550_e48022 - locals.var_xstard);
        (assign42550_e48024, ((locals.var_xsd_dn4 + locals.var_xd_dn4) - locals.var_xstard_dn4), ((locals.var_xsd_dn6 + locals.var_xd_dn6) - locals.var_xstard_dn6), ((locals.var_xsd_dn7 + locals.var_xd_dn7) - locals.var_xstard_dn7), ((locals.var_xsd_dn8 + locals.var_xd_dn8) - locals.var_xstard_dn8), ((locals.var_xsd_dn9 + locals.var_xd_dn9) - locals.var_xstard_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign42550_e48026;
        locals.var_temp2_dn4 = assign42550_e48026_d_n4;
        locals.var_temp2_dn6 = assign42550_e48026_d_n6;
        locals.var_temp2_dn7 = assign42550_e48026_d_n7;
        locals.var_temp2_dn8 = assign42550_e48026_d_n8;
        locals.var_temp2_dn9 = assign42550_e48026_d_n9;

        let (assign42560_e48032, assign42560_e48032_d_n4, assign42560_e48032_d_n6, assign42560_e48032_d_n7, assign42560_e48032_d_n8, assign42560_e48032_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42560_e48030: f64 = (2.0 * locals.var_xalphaf);
        (assign42560_e48030, (2.0 * locals.var_xalphaf_dn4), (2.0 * locals.var_xalphaf_dn6), (2.0 * locals.var_xalphaf_dn7), (2.0 * locals.var_xalphaf_dn8), (2.0 * locals.var_xalphaf_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign42560_e48032;
        locals.var_temp_dn4 = assign42560_e48032_d_n4;
        locals.var_temp_dn6 = assign42560_e48032_d_n6;
        locals.var_temp_dn7 = assign42560_e48032_d_n7;
        locals.var_temp_dn8 = assign42560_e48032_d_n8;
        locals.var_temp_dn9 = assign42560_e48032_d_n9;

        let (assign42570_e48047, assign42570_e48047_d_n4, assign42570_e48047_d_n6, assign42570_e48047_d_n7, assign42570_e48047_d_n8, assign42570_e48047_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42570_e48039: f64 = (locals.var_temp1 / locals.var_xalphaf);
        let assign42570_e48040: f64 = (1.0 + assign42570_e48039);
        let assign42570_e48041: f64 = (assign42570_e48040).sqrt();
        let assign42570_e48043: f64 = (assign42570_e48041 - 1.0);
        let assign42570_e48044: f64 = (locals.var_temp * assign42570_e48043);
        let assign42570_e48045: f64 = (locals.var_xstars + assign42570_e48044);
        (assign42570_e48045, (locals.var_xstars_dn4 + ((locals.var_temp_dn4 * assign42570_e48043) + (locals.var_temp * ((((locals.var_temp1_dn4 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn4)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42570_e48041))))), (locals.var_xstars_dn6 + ((locals.var_temp_dn6 * assign42570_e48043) + (locals.var_temp * ((((locals.var_temp1_dn6 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn6)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42570_e48041))))), (locals.var_xstars_dn7 + ((locals.var_temp_dn7 * assign42570_e48043) + (locals.var_temp * ((((locals.var_temp1_dn7 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn7)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42570_e48041))))), (locals.var_xstars_dn8 + ((locals.var_temp_dn8 * assign42570_e48043) + (locals.var_temp * ((((locals.var_temp1_dn8 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn8)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42570_e48041))))), (locals.var_xstars_dn9 + ((locals.var_temp_dn9 * assign42570_e48043) + (locals.var_temp * ((((locals.var_temp1_dn9 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn9)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42570_e48041))))),)
    } else {
        (locals.var_xedgefs, locals.var_xedgefs_dn4, locals.var_xedgefs_dn6, locals.var_xedgefs_dn7, locals.var_xedgefs_dn8, locals.var_xedgefs_dn9,)
    }
};
        locals.var_xedgefs = assign42570_e48047;
        locals.var_xedgefs_dn4 = assign42570_e48047_d_n4;
        locals.var_xedgefs_dn6 = assign42570_e48047_d_n6;
        locals.var_xedgefs_dn7 = assign42570_e48047_d_n7;
        locals.var_xedgefs_dn8 = assign42570_e48047_d_n8;
        locals.var_xedgefs_dn9 = assign42570_e48047_d_n9;

        let (assign42580_e48062, assign42580_e48062_d_n4, assign42580_e48062_d_n6, assign42580_e48062_d_n7, assign42580_e48062_d_n8, assign42580_e48062_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42580_e48054: f64 = (locals.var_temp2 / locals.var_xalphaf);
        let assign42580_e48055: f64 = (1.0 + assign42580_e48054);
        let assign42580_e48056: f64 = (assign42580_e48055).sqrt();
        let assign42580_e48058: f64 = (assign42580_e48056 - 1.0);
        let assign42580_e48059: f64 = (locals.var_temp * assign42580_e48058);
        let assign42580_e48060: f64 = (locals.var_xstard + assign42580_e48059);
        (assign42580_e48060, (locals.var_xstard_dn4 + ((locals.var_temp_dn4 * assign42580_e48058) + (locals.var_temp * ((((locals.var_temp2_dn4 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn4)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42580_e48056))))), (locals.var_xstard_dn6 + ((locals.var_temp_dn6 * assign42580_e48058) + (locals.var_temp * ((((locals.var_temp2_dn6 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn6)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42580_e48056))))), (locals.var_xstard_dn7 + ((locals.var_temp_dn7 * assign42580_e48058) + (locals.var_temp * ((((locals.var_temp2_dn7 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn7)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42580_e48056))))), (locals.var_xstard_dn8 + ((locals.var_temp_dn8 * assign42580_e48058) + (locals.var_temp * ((((locals.var_temp2_dn8 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn8)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42580_e48056))))), (locals.var_xstard_dn9 + ((locals.var_temp_dn9 * assign42580_e48058) + (locals.var_temp * ((((locals.var_temp2_dn9 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn9)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42580_e48056))))),)
    } else {
        (locals.var_xedgefd, locals.var_xedgefd_dn4, locals.var_xedgefd_dn6, locals.var_xedgefd_dn7, locals.var_xedgefd_dn8, locals.var_xedgefd_dn9,)
    }
};
        locals.var_xedgefd = assign42580_e48062;
        locals.var_xedgefd_dn4 = assign42580_e48062_d_n4;
        locals.var_xedgefd_dn6 = assign42580_e48062_d_n6;
        locals.var_xedgefd_dn7 = assign42580_e48062_d_n7;
        locals.var_xedgefd_dn8 = assign42580_e48062_d_n8;
        locals.var_xedgefd_dn9 = assign42580_e48062_d_n9;

        let (assign42590_e48068, assign42590_e48068_d_n4, assign42590_e48068_d_n6, assign42590_e48068_d_n7, assign42590_e48068_d_n8, assign42590_e48068_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42590_e48066: f64 = (2.0 * locals.var_xalphab);
        (assign42590_e48066, (2.0 * locals.var_xalphab_dn4), (2.0 * locals.var_xalphab_dn6), (2.0 * locals.var_xalphab_dn7), (2.0 * locals.var_xalphab_dn8), (2.0 * locals.var_xalphab_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign42590_e48068;
        locals.var_temp_dn4 = assign42590_e48068_d_n4;
        locals.var_temp_dn6 = assign42590_e48068_d_n6;
        locals.var_temp_dn7 = assign42590_e48068_d_n7;
        locals.var_temp_dn8 = assign42590_e48068_d_n8;
        locals.var_temp_dn9 = assign42590_e48068_d_n9;

        let (assign42600_e48083, assign42600_e48083_d_n4, assign42600_e48083_d_n6, assign42600_e48083_d_n7, assign42600_e48083_d_n8, assign42600_e48083_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42600_e48075: f64 = (locals.var_temp1 / locals.var_xalphab);
        let assign42600_e48076: f64 = (1.0 + assign42600_e48075);
        let assign42600_e48077: f64 = (assign42600_e48076).sqrt();
        let assign42600_e48079: f64 = (assign42600_e48077 - 1.0);
        let assign42600_e48080: f64 = (locals.var_temp * assign42600_e48079);
        let assign42600_e48081: f64 = (locals.var_xstars + assign42600_e48080);
        (assign42600_e48081, (locals.var_xstars_dn4 + ((locals.var_temp_dn4 * assign42600_e48079) + (locals.var_temp * ((((locals.var_temp1_dn4 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn4)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42600_e48077))))), (locals.var_xstars_dn6 + ((locals.var_temp_dn6 * assign42600_e48079) + (locals.var_temp * ((((locals.var_temp1_dn6 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn6)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42600_e48077))))), (locals.var_xstars_dn7 + ((locals.var_temp_dn7 * assign42600_e48079) + (locals.var_temp * ((((locals.var_temp1_dn7 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn7)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42600_e48077))))), (locals.var_xstars_dn8 + ((locals.var_temp_dn8 * assign42600_e48079) + (locals.var_temp * ((((locals.var_temp1_dn8 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn8)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42600_e48077))))), (locals.var_xstars_dn9 + ((locals.var_temp_dn9 * assign42600_e48079) + (locals.var_temp * ((((locals.var_temp1_dn9 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn9)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42600_e48077))))),)
    } else {
        (locals.var_xedgebs, locals.var_xedgebs_dn4, locals.var_xedgebs_dn6, locals.var_xedgebs_dn7, locals.var_xedgebs_dn8, locals.var_xedgebs_dn9,)
    }
};
        locals.var_xedgebs = assign42600_e48083;
        locals.var_xedgebs_dn4 = assign42600_e48083_d_n4;
        locals.var_xedgebs_dn6 = assign42600_e48083_d_n6;
        locals.var_xedgebs_dn7 = assign42600_e48083_d_n7;
        locals.var_xedgebs_dn8 = assign42600_e48083_d_n8;
        locals.var_xedgebs_dn9 = assign42600_e48083_d_n9;

        let (assign42610_e48098, assign42610_e48098_d_n4, assign42610_e48098_d_n6, assign42610_e48098_d_n7, assign42610_e48098_d_n8, assign42610_e48098_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42610_e48090: f64 = (locals.var_temp2 / locals.var_xalphab);
        let assign42610_e48091: f64 = (1.0 + assign42610_e48090);
        let assign42610_e48092: f64 = (assign42610_e48091).sqrt();
        let assign42610_e48094: f64 = (assign42610_e48092 - 1.0);
        let assign42610_e48095: f64 = (locals.var_temp * assign42610_e48094);
        let assign42610_e48096: f64 = (locals.var_xstard + assign42610_e48095);
        (assign42610_e48096, (locals.var_xstard_dn4 + ((locals.var_temp_dn4 * assign42610_e48094) + (locals.var_temp * ((((locals.var_temp2_dn4 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn4)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42610_e48092))))), (locals.var_xstard_dn6 + ((locals.var_temp_dn6 * assign42610_e48094) + (locals.var_temp * ((((locals.var_temp2_dn6 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn6)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42610_e48092))))), (locals.var_xstard_dn7 + ((locals.var_temp_dn7 * assign42610_e48094) + (locals.var_temp * ((((locals.var_temp2_dn7 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn7)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42610_e48092))))), (locals.var_xstard_dn8 + ((locals.var_temp_dn8 * assign42610_e48094) + (locals.var_temp * ((((locals.var_temp2_dn8 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn8)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42610_e48092))))), (locals.var_xstard_dn9 + ((locals.var_temp_dn9 * assign42610_e48094) + (locals.var_temp * ((((locals.var_temp2_dn9 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn9)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42610_e48092))))),)
    } else {
        (locals.var_xedgebd, locals.var_xedgebd_dn4, locals.var_xedgebd_dn6, locals.var_xedgebd_dn7, locals.var_xedgebd_dn8, locals.var_xedgebd_dn9,)
    }
};
        locals.var_xedgebd = assign42610_e48098;
        locals.var_xedgebd_dn4 = assign42610_e48098_d_n4;
        locals.var_xedgebd_dn6 = assign42610_e48098_d_n6;
        locals.var_xedgebd_dn7 = assign42610_e48098_d_n7;
        locals.var_xedgebd_dn8 = assign42610_e48098_d_n8;
        locals.var_xedgebd_dn9 = assign42610_e48098_d_n9;

        let (assign42620_e48104, assign42620_e48104_d_n4, assign42620_e48104_d_n6, assign42620_e48104_d_n7, assign42620_e48104_d_n8, assign42620_e48104_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42620_e48102: f64 = (locals.var_fif_phit * locals.var_csiprime_ac);
        (assign42620_e48102, ((locals.var_fif_phit_dn4 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn4)), ((locals.var_fif_phit_dn6 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn6)), ((locals.var_fif_phit_dn7 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn7)), ((locals.var_fif_phit_dn8 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn8)), ((locals.var_fif_phit_dn9 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign42620_e48104;
        locals.var_temp_dn4 = assign42620_e48104_d_n4;
        locals.var_temp_dn6 = assign42620_e48104_d_n6;
        locals.var_temp_dn7 = assign42620_e48104_d_n7;
        locals.var_temp_dn8 = assign42620_e48104_d_n8;
        locals.var_temp_dn9 = assign42620_e48104_d_n9;

        let (assign42630_e48115, assign42630_e48115_d_n4, assign42630_e48115_d_n6, assign42630_e48115_d_n7, assign42630_e48115_d_n8, assign42630_e48115_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42630_e48107: f64 = (-locals.var_temp);
        let assign42630_e48109: f64 = (assign42630_e48107 * locals.var_lambdaf);
        let assign42630_e48111: f64 = (assign42630_e48109 * locals.var_k1_ac);
        let assign42630_e48113: f64 = (assign42630_e48111 * locals.var_sce1_ac);
        (assign42630_e48113, (((((((-locals.var_temp_dn4) * locals.var_lambdaf) + (assign42630_e48107 * locals.var_lambdaf_dn4)) * locals.var_k1_ac) + (assign42630_e48109 * locals.var_k1_ac_dn4)) * locals.var_sce1_ac) + (assign42630_e48111 * locals.var_sce1_ac_dn4)), (((((((-locals.var_temp_dn6) * locals.var_lambdaf) + (assign42630_e48107 * locals.var_lambdaf_dn6)) * locals.var_k1_ac) + (assign42630_e48109 * locals.var_k1_ac_dn6)) * locals.var_sce1_ac) + (assign42630_e48111 * locals.var_sce1_ac_dn6)), (((((((-locals.var_temp_dn7) * locals.var_lambdaf) + (assign42630_e48107 * locals.var_lambdaf_dn7)) * locals.var_k1_ac) + (assign42630_e48109 * locals.var_k1_ac_dn7)) * locals.var_sce1_ac) + (assign42630_e48111 * locals.var_sce1_ac_dn7)), (((((((-locals.var_temp_dn8) * locals.var_lambdaf) + (assign42630_e48107 * locals.var_lambdaf_dn8)) * locals.var_k1_ac) + (assign42630_e48109 * locals.var_k1_ac_dn8)) * locals.var_sce1_ac) + (assign42630_e48111 * locals.var_sce1_ac_dn8)), (((((((-locals.var_temp_dn9) * locals.var_lambdaf) + (assign42630_e48107 * locals.var_lambdaf_dn9)) * locals.var_k1_ac) + (assign42630_e48109 * locals.var_k1_ac_dn9)) * locals.var_sce1_ac) + (assign42630_e48111 * locals.var_sce1_ac_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign42630_e48115;
        locals.var_temp1_dn4 = assign42630_e48115_d_n4;
        locals.var_temp1_dn6 = assign42630_e48115_d_n6;
        locals.var_temp1_dn7 = assign42630_e48115_d_n7;
        locals.var_temp1_dn8 = assign42630_e48115_d_n8;
        locals.var_temp1_dn9 = assign42630_e48115_d_n9;

        let (assign42640_e48126, assign42640_e48126_d_n4, assign42640_e48126_d_n6, assign42640_e48126_d_n7, assign42640_e48126_d_n8, assign42640_e48126_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42640_e48118: f64 = (-locals.var_temp);
        let assign42640_e48120: f64 = (assign42640_e48118 * locals.var_lambdab);
        let assign42640_e48122: f64 = (assign42640_e48120 * locals.var_k2_ac);
        let assign42640_e48124: f64 = (assign42640_e48122 * locals.var_sce2_ac);
        (assign42640_e48124, (((((((-locals.var_temp_dn4) * locals.var_lambdab) + (assign42640_e48118 * locals.var_lambdab_dn4)) * locals.var_k2_ac) + (assign42640_e48120 * locals.var_k2_ac_dn4)) * locals.var_sce2_ac) + (assign42640_e48122 * locals.var_sce2_ac_dn4)), (((((((-locals.var_temp_dn6) * locals.var_lambdab) + (assign42640_e48118 * locals.var_lambdab_dn6)) * locals.var_k2_ac) + (assign42640_e48120 * locals.var_k2_ac_dn6)) * locals.var_sce2_ac) + (assign42640_e48122 * locals.var_sce2_ac_dn6)), (((((((-locals.var_temp_dn7) * locals.var_lambdab) + (assign42640_e48118 * locals.var_lambdab_dn7)) * locals.var_k2_ac) + (assign42640_e48120 * locals.var_k2_ac_dn7)) * locals.var_sce2_ac) + (assign42640_e48122 * locals.var_sce2_ac_dn7)), (((((((-locals.var_temp_dn8) * locals.var_lambdab) + (assign42640_e48118 * locals.var_lambdab_dn8)) * locals.var_k2_ac) + (assign42640_e48120 * locals.var_k2_ac_dn8)) * locals.var_sce2_ac) + (assign42640_e48122 * locals.var_sce2_ac_dn8)), (((((((-locals.var_temp_dn9) * locals.var_lambdab) + (assign42640_e48118 * locals.var_lambdab_dn9)) * locals.var_k2_ac) + (assign42640_e48120 * locals.var_k2_ac_dn9)) * locals.var_sce2_ac) + (assign42640_e48122 * locals.var_sce2_ac_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign42640_e48126;
        locals.var_temp2_dn4 = assign42640_e48126_d_n4;
        locals.var_temp2_dn6 = assign42640_e48126_d_n6;
        locals.var_temp2_dn7 = assign42640_e48126_d_n7;
        locals.var_temp2_dn8 = assign42640_e48126_d_n8;
        locals.var_temp2_dn9 = assign42640_e48126_d_n9;

        let (assign42650_e48151, assign42650_e48151_d_n4, assign42650_e48151_d_n6, assign42650_e48151_d_n7, assign42650_e48151_d_n8, assign42650_e48151_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42650_e48131: f64 = (locals.var_xedgefs - locals.var_xeffs);
        let assign42650_e48133: f64 = assign42650_e48131;
        let assign42650_e48136: f64 = (locals.var_xedgefs - locals.var_xeffs);
        let assign42650_e48138: f64 = assign42650_e48136;
        let assign42650_e48141: f64 = (locals.var_xedgefs - locals.var_xeffs);
        let assign42650_e48143: f64 = assign42650_e48141;
        let assign42650_e48144: f64 = (assign42650_e48138 * assign42650_e48143);
        let assign42650_e48146: f64 = (assign42650_e48144 + 1.0);
        let assign42650_e48147: f64 = (assign42650_e48146).sqrt();
        let assign42650_e48148: f64 = (assign42650_e48133 + assign42650_e48147);
        let assign42650_e48149: f64 = (0.5 * assign42650_e48148);
        (assign42650_e48149, (0.5 * ((locals.var_xedgefs_dn4 - locals.var_xeffs_dn4) + ((((locals.var_xedgefs_dn4 - locals.var_xeffs_dn4) * assign42650_e48143) + (assign42650_e48138 * (locals.var_xedgefs_dn4 - locals.var_xeffs_dn4))) / (2.0 * assign42650_e48147)))), (0.5 * ((locals.var_xedgefs_dn6 - locals.var_xeffs_dn6) + ((((locals.var_xedgefs_dn6 - locals.var_xeffs_dn6) * assign42650_e48143) + (assign42650_e48138 * (locals.var_xedgefs_dn6 - locals.var_xeffs_dn6))) / (2.0 * assign42650_e48147)))), (0.5 * ((locals.var_xedgefs_dn7 - locals.var_xeffs_dn7) + ((((locals.var_xedgefs_dn7 - locals.var_xeffs_dn7) * assign42650_e48143) + (assign42650_e48138 * (locals.var_xedgefs_dn7 - locals.var_xeffs_dn7))) / (2.0 * assign42650_e48147)))), (0.5 * ((locals.var_xedgefs_dn8 - locals.var_xeffs_dn8) + ((((locals.var_xedgefs_dn8 - locals.var_xeffs_dn8) * assign42650_e48143) + (assign42650_e48138 * (locals.var_xedgefs_dn8 - locals.var_xeffs_dn8))) / (2.0 * assign42650_e48147)))), (0.5 * ((locals.var_xedgefs_dn9 - locals.var_xeffs_dn9) + ((((locals.var_xedgefs_dn9 - locals.var_xeffs_dn9) * assign42650_e48143) + (assign42650_e48138 * (locals.var_xedgefs_dn9 - locals.var_xeffs_dn9))) / (2.0 * assign42650_e48147)))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign42650_e48151;
        locals.var_temp_dn4 = assign42650_e48151_d_n4;
        locals.var_temp_dn6 = assign42650_e48151_d_n6;
        locals.var_temp_dn7 = assign42650_e48151_d_n7;
        locals.var_temp_dn8 = assign42650_e48151_d_n8;
        locals.var_temp_dn9 = assign42650_e48151_d_n9;

        let (assign42660_e48163, assign42660_e48163_d_n4, assign42660_e48163_d_n6, assign42660_e48163_d_n7, assign42660_e48163_d_n8, assign42660_e48163_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42660_e48155: f64 = (locals.var_temp1 * locals.var_temp);
        let assign42660_e48157: f64 = (assign42660_e48155 * locals.var_temp);
        let assign42660_e48160: f64 = (locals.var_xedgefs - locals.var_xstars);
        let assign42660_e48161: f64 = (assign42660_e48157 / assign42660_e48160);
        (assign42660_e48161, (((((((locals.var_temp1_dn4 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn4)) * locals.var_temp) + (assign42660_e48155 * locals.var_temp_dn4)) * assign42660_e48160) - (assign42660_e48157 * (locals.var_xedgefs_dn4 - locals.var_xstars_dn4))) / (assign42660_e48160 * assign42660_e48160)), (((((((locals.var_temp1_dn6 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn6)) * locals.var_temp) + (assign42660_e48155 * locals.var_temp_dn6)) * assign42660_e48160) - (assign42660_e48157 * (locals.var_xedgefs_dn6 - locals.var_xstars_dn6))) / (assign42660_e48160 * assign42660_e48160)), (((((((locals.var_temp1_dn7 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn7)) * locals.var_temp) + (assign42660_e48155 * locals.var_temp_dn7)) * assign42660_e48160) - (assign42660_e48157 * (locals.var_xedgefs_dn7 - locals.var_xstars_dn7))) / (assign42660_e48160 * assign42660_e48160)), (((((((locals.var_temp1_dn8 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn8)) * locals.var_temp) + (assign42660_e48155 * locals.var_temp_dn8)) * assign42660_e48160) - (assign42660_e48157 * (locals.var_xedgefs_dn8 - locals.var_xstars_dn8))) / (assign42660_e48160 * assign42660_e48160)), (((((((locals.var_temp1_dn9 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn9)) * locals.var_temp) + (assign42660_e48155 * locals.var_temp_dn9)) * assign42660_e48160) - (assign42660_e48157 * (locals.var_xedgefs_dn9 - locals.var_xstars_dn9))) / (assign42660_e48160 * assign42660_e48160)),)
    } else {
        (locals.var_qgsif, locals.var_qgsif_dn4, locals.var_qgsif_dn6, locals.var_qgsif_dn7, locals.var_qgsif_dn8, locals.var_qgsif_dn9,)
    }
};
        locals.var_qgsif = assign42660_e48163;
        locals.var_qgsif_dn4 = assign42660_e48163_d_n4;
        locals.var_qgsif_dn6 = assign42660_e48163_d_n6;
        locals.var_qgsif_dn7 = assign42660_e48163_d_n7;
        locals.var_qgsif_dn8 = assign42660_e48163_d_n8;
        locals.var_qgsif_dn9 = assign42660_e48163_d_n9;

        let (assign42670_e48188, assign42670_e48188_d_n4, assign42670_e48188_d_n6, assign42670_e48188_d_n7, assign42670_e48188_d_n8, assign42670_e48188_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42670_e48168: f64 = (locals.var_xedgefd - locals.var_xeffd);
        let assign42670_e48170: f64 = assign42670_e48168;
        let assign42670_e48173: f64 = (locals.var_xedgefd - locals.var_xeffd);
        let assign42670_e48175: f64 = assign42670_e48173;
        let assign42670_e48178: f64 = (locals.var_xedgefd - locals.var_xeffd);
        let assign42670_e48180: f64 = assign42670_e48178;
        let assign42670_e48181: f64 = (assign42670_e48175 * assign42670_e48180);
        let assign42670_e48183: f64 = (assign42670_e48181 + 1.0);
        let assign42670_e48184: f64 = (assign42670_e48183).sqrt();
        let assign42670_e48185: f64 = (assign42670_e48170 + assign42670_e48184);
        let assign42670_e48186: f64 = (0.5 * assign42670_e48185);
        (assign42670_e48186, (0.5 * ((locals.var_xedgefd_dn4 - locals.var_xeffd_dn4) + ((((locals.var_xedgefd_dn4 - locals.var_xeffd_dn4) * assign42670_e48180) + (assign42670_e48175 * (locals.var_xedgefd_dn4 - locals.var_xeffd_dn4))) / (2.0 * assign42670_e48184)))), (0.5 * ((locals.var_xedgefd_dn6 - locals.var_xeffd_dn6) + ((((locals.var_xedgefd_dn6 - locals.var_xeffd_dn6) * assign42670_e48180) + (assign42670_e48175 * (locals.var_xedgefd_dn6 - locals.var_xeffd_dn6))) / (2.0 * assign42670_e48184)))), (0.5 * ((locals.var_xedgefd_dn7 - locals.var_xeffd_dn7) + ((((locals.var_xedgefd_dn7 - locals.var_xeffd_dn7) * assign42670_e48180) + (assign42670_e48175 * (locals.var_xedgefd_dn7 - locals.var_xeffd_dn7))) / (2.0 * assign42670_e48184)))), (0.5 * ((locals.var_xedgefd_dn8 - locals.var_xeffd_dn8) + ((((locals.var_xedgefd_dn8 - locals.var_xeffd_dn8) * assign42670_e48180) + (assign42670_e48175 * (locals.var_xedgefd_dn8 - locals.var_xeffd_dn8))) / (2.0 * assign42670_e48184)))), (0.5 * ((locals.var_xedgefd_dn9 - locals.var_xeffd_dn9) + ((((locals.var_xedgefd_dn9 - locals.var_xeffd_dn9) * assign42670_e48180) + (assign42670_e48175 * (locals.var_xedgefd_dn9 - locals.var_xeffd_dn9))) / (2.0 * assign42670_e48184)))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign42670_e48188;
        locals.var_temp_dn4 = assign42670_e48188_d_n4;
        locals.var_temp_dn6 = assign42670_e48188_d_n6;
        locals.var_temp_dn7 = assign42670_e48188_d_n7;
        locals.var_temp_dn8 = assign42670_e48188_d_n8;
        locals.var_temp_dn9 = assign42670_e48188_d_n9;

        let (assign42680_e48200, assign42680_e48200_d_n4, assign42680_e48200_d_n6, assign42680_e48200_d_n7, assign42680_e48200_d_n8, assign42680_e48200_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42680_e48192: f64 = (locals.var_temp1 * locals.var_temp);
        let assign42680_e48194: f64 = (assign42680_e48192 * locals.var_temp);
        let assign42680_e48197: f64 = (locals.var_xedgefd - locals.var_xstard);
        let assign42680_e48198: f64 = (assign42680_e48194 / assign42680_e48197);
        (assign42680_e48198, (((((((locals.var_temp1_dn4 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn4)) * locals.var_temp) + (assign42680_e48192 * locals.var_temp_dn4)) * assign42680_e48197) - (assign42680_e48194 * (locals.var_xedgefd_dn4 - locals.var_xstard_dn4))) / (assign42680_e48197 * assign42680_e48197)), (((((((locals.var_temp1_dn6 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn6)) * locals.var_temp) + (assign42680_e48192 * locals.var_temp_dn6)) * assign42680_e48197) - (assign42680_e48194 * (locals.var_xedgefd_dn6 - locals.var_xstard_dn6))) / (assign42680_e48197 * assign42680_e48197)), (((((((locals.var_temp1_dn7 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn7)) * locals.var_temp) + (assign42680_e48192 * locals.var_temp_dn7)) * assign42680_e48197) - (assign42680_e48194 * (locals.var_xedgefd_dn7 - locals.var_xstard_dn7))) / (assign42680_e48197 * assign42680_e48197)), (((((((locals.var_temp1_dn8 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn8)) * locals.var_temp) + (assign42680_e48192 * locals.var_temp_dn8)) * assign42680_e48197) - (assign42680_e48194 * (locals.var_xedgefd_dn8 - locals.var_xstard_dn8))) / (assign42680_e48197 * assign42680_e48197)), (((((((locals.var_temp1_dn9 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn9)) * locals.var_temp) + (assign42680_e48192 * locals.var_temp_dn9)) * assign42680_e48197) - (assign42680_e48194 * (locals.var_xedgefd_dn9 - locals.var_xstard_dn9))) / (assign42680_e48197 * assign42680_e48197)),)
    } else {
        (locals.var_qgdif, locals.var_qgdif_dn4, locals.var_qgdif_dn6, locals.var_qgdif_dn7, locals.var_qgdif_dn8, locals.var_qgdif_dn9,)
    }
};
        locals.var_qgdif = assign42680_e48200;
        locals.var_qgdif_dn4 = assign42680_e48200_d_n4;
        locals.var_qgdif_dn6 = assign42680_e48200_d_n6;
        locals.var_qgdif_dn7 = assign42680_e48200_d_n7;
        locals.var_qgdif_dn8 = assign42680_e48200_d_n8;
        locals.var_qgdif_dn9 = assign42680_e48200_d_n9;

        let (assign42690_e48225, assign42690_e48225_d_n4, assign42690_e48225_d_n6, assign42690_e48225_d_n7, assign42690_e48225_d_n8, assign42690_e48225_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42690_e48205: f64 = (locals.var_xedgebs - locals.var_xeffs);
        let assign42690_e48207: f64 = assign42690_e48205;
        let assign42690_e48210: f64 = (locals.var_xedgebs - locals.var_xeffs);
        let assign42690_e48212: f64 = assign42690_e48210;
        let assign42690_e48215: f64 = (locals.var_xedgebs - locals.var_xeffs);
        let assign42690_e48217: f64 = assign42690_e48215;
        let assign42690_e48218: f64 = (assign42690_e48212 * assign42690_e48217);
        let assign42690_e48220: f64 = (assign42690_e48218 + 1.0);
        let assign42690_e48221: f64 = (assign42690_e48220).sqrt();
        let assign42690_e48222: f64 = (assign42690_e48207 + assign42690_e48221);
        let assign42690_e48223: f64 = (0.5 * assign42690_e48222);
        (assign42690_e48223, (0.5 * ((locals.var_xedgebs_dn4 - locals.var_xeffs_dn4) + ((((locals.var_xedgebs_dn4 - locals.var_xeffs_dn4) * assign42690_e48217) + (assign42690_e48212 * (locals.var_xedgebs_dn4 - locals.var_xeffs_dn4))) / (2.0 * assign42690_e48221)))), (0.5 * ((locals.var_xedgebs_dn6 - locals.var_xeffs_dn6) + ((((locals.var_xedgebs_dn6 - locals.var_xeffs_dn6) * assign42690_e48217) + (assign42690_e48212 * (locals.var_xedgebs_dn6 - locals.var_xeffs_dn6))) / (2.0 * assign42690_e48221)))), (0.5 * ((locals.var_xedgebs_dn7 - locals.var_xeffs_dn7) + ((((locals.var_xedgebs_dn7 - locals.var_xeffs_dn7) * assign42690_e48217) + (assign42690_e48212 * (locals.var_xedgebs_dn7 - locals.var_xeffs_dn7))) / (2.0 * assign42690_e48221)))), (0.5 * ((locals.var_xedgebs_dn8 - locals.var_xeffs_dn8) + ((((locals.var_xedgebs_dn8 - locals.var_xeffs_dn8) * assign42690_e48217) + (assign42690_e48212 * (locals.var_xedgebs_dn8 - locals.var_xeffs_dn8))) / (2.0 * assign42690_e48221)))), (0.5 * ((locals.var_xedgebs_dn9 - locals.var_xeffs_dn9) + ((((locals.var_xedgebs_dn9 - locals.var_xeffs_dn9) * assign42690_e48217) + (assign42690_e48212 * (locals.var_xedgebs_dn9 - locals.var_xeffs_dn9))) / (2.0 * assign42690_e48221)))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign42690_e48225;
        locals.var_temp_dn4 = assign42690_e48225_d_n4;
        locals.var_temp_dn6 = assign42690_e48225_d_n6;
        locals.var_temp_dn7 = assign42690_e48225_d_n7;
        locals.var_temp_dn8 = assign42690_e48225_d_n8;
        locals.var_temp_dn9 = assign42690_e48225_d_n9;

        let (assign42700_e48237, assign42700_e48237_d_n4, assign42700_e48237_d_n6, assign42700_e48237_d_n7, assign42700_e48237_d_n8, assign42700_e48237_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42700_e48229: f64 = (locals.var_temp2 * locals.var_temp);
        let assign42700_e48231: f64 = (assign42700_e48229 * locals.var_temp);
        let assign42700_e48234: f64 = (locals.var_xedgebs - locals.var_xstars);
        let assign42700_e48235: f64 = (assign42700_e48231 / assign42700_e48234);
        (assign42700_e48235, (((((((locals.var_temp2_dn4 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn4)) * locals.var_temp) + (assign42700_e48229 * locals.var_temp_dn4)) * assign42700_e48234) - (assign42700_e48231 * (locals.var_xedgebs_dn4 - locals.var_xstars_dn4))) / (assign42700_e48234 * assign42700_e48234)), (((((((locals.var_temp2_dn6 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn6)) * locals.var_temp) + (assign42700_e48229 * locals.var_temp_dn6)) * assign42700_e48234) - (assign42700_e48231 * (locals.var_xedgebs_dn6 - locals.var_xstars_dn6))) / (assign42700_e48234 * assign42700_e48234)), (((((((locals.var_temp2_dn7 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn7)) * locals.var_temp) + (assign42700_e48229 * locals.var_temp_dn7)) * assign42700_e48234) - (assign42700_e48231 * (locals.var_xedgebs_dn7 - locals.var_xstars_dn7))) / (assign42700_e48234 * assign42700_e48234)), (((((((locals.var_temp2_dn8 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn8)) * locals.var_temp) + (assign42700_e48229 * locals.var_temp_dn8)) * assign42700_e48234) - (assign42700_e48231 * (locals.var_xedgebs_dn8 - locals.var_xstars_dn8))) / (assign42700_e48234 * assign42700_e48234)), (((((((locals.var_temp2_dn9 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn9)) * locals.var_temp) + (assign42700_e48229 * locals.var_temp_dn9)) * assign42700_e48234) - (assign42700_e48231 * (locals.var_xedgebs_dn9 - locals.var_xstars_dn9))) / (assign42700_e48234 * assign42700_e48234)),)
    } else {
        (locals.var_qbsif, locals.var_qbsif_dn4, locals.var_qbsif_dn6, locals.var_qbsif_dn7, locals.var_qbsif_dn8, locals.var_qbsif_dn9,)
    }
};
        locals.var_qbsif = assign42700_e48237;
        locals.var_qbsif_dn4 = assign42700_e48237_d_n4;
        locals.var_qbsif_dn6 = assign42700_e48237_d_n6;
        locals.var_qbsif_dn7 = assign42700_e48237_d_n7;
        locals.var_qbsif_dn8 = assign42700_e48237_d_n8;
        locals.var_qbsif_dn9 = assign42700_e48237_d_n9;

        let (assign42710_e48262, assign42710_e48262_d_n4, assign42710_e48262_d_n6, assign42710_e48262_d_n7, assign42710_e48262_d_n8, assign42710_e48262_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42710_e48242: f64 = (locals.var_xedgebd - locals.var_xeffd);
        let assign42710_e48244: f64 = assign42710_e48242;
        let assign42710_e48247: f64 = (locals.var_xedgebd - locals.var_xeffd);
        let assign42710_e48249: f64 = assign42710_e48247;
        let assign42710_e48252: f64 = (locals.var_xedgebd - locals.var_xeffd);
        let assign42710_e48254: f64 = assign42710_e48252;
        let assign42710_e48255: f64 = (assign42710_e48249 * assign42710_e48254);
        let assign42710_e48257: f64 = (assign42710_e48255 + 1.0);
        let assign42710_e48258: f64 = (assign42710_e48257).sqrt();
        let assign42710_e48259: f64 = (assign42710_e48244 + assign42710_e48258);
        let assign42710_e48260: f64 = (0.5 * assign42710_e48259);
        (assign42710_e48260, (0.5 * ((locals.var_xedgebd_dn4 - locals.var_xeffd_dn4) + ((((locals.var_xedgebd_dn4 - locals.var_xeffd_dn4) * assign42710_e48254) + (assign42710_e48249 * (locals.var_xedgebd_dn4 - locals.var_xeffd_dn4))) / (2.0 * assign42710_e48258)))), (0.5 * ((locals.var_xedgebd_dn6 - locals.var_xeffd_dn6) + ((((locals.var_xedgebd_dn6 - locals.var_xeffd_dn6) * assign42710_e48254) + (assign42710_e48249 * (locals.var_xedgebd_dn6 - locals.var_xeffd_dn6))) / (2.0 * assign42710_e48258)))), (0.5 * ((locals.var_xedgebd_dn7 - locals.var_xeffd_dn7) + ((((locals.var_xedgebd_dn7 - locals.var_xeffd_dn7) * assign42710_e48254) + (assign42710_e48249 * (locals.var_xedgebd_dn7 - locals.var_xeffd_dn7))) / (2.0 * assign42710_e48258)))), (0.5 * ((locals.var_xedgebd_dn8 - locals.var_xeffd_dn8) + ((((locals.var_xedgebd_dn8 - locals.var_xeffd_dn8) * assign42710_e48254) + (assign42710_e48249 * (locals.var_xedgebd_dn8 - locals.var_xeffd_dn8))) / (2.0 * assign42710_e48258)))), (0.5 * ((locals.var_xedgebd_dn9 - locals.var_xeffd_dn9) + ((((locals.var_xedgebd_dn9 - locals.var_xeffd_dn9) * assign42710_e48254) + (assign42710_e48249 * (locals.var_xedgebd_dn9 - locals.var_xeffd_dn9))) / (2.0 * assign42710_e48258)))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign42710_e48262;
        locals.var_temp_dn4 = assign42710_e48262_d_n4;
        locals.var_temp_dn6 = assign42710_e48262_d_n6;
        locals.var_temp_dn7 = assign42710_e48262_d_n7;
        locals.var_temp_dn8 = assign42710_e48262_d_n8;
        locals.var_temp_dn9 = assign42710_e48262_d_n9;

        let (assign42720_e48274, assign42720_e48274_d_n4, assign42720_e48274_d_n6, assign42720_e48274_d_n7, assign42720_e48274_d_n8, assign42720_e48274_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42720_e48266: f64 = (locals.var_temp2 * locals.var_temp);
        let assign42720_e48268: f64 = (assign42720_e48266 * locals.var_temp);
        let assign42720_e48271: f64 = (locals.var_xedgebd - locals.var_xstard);
        let assign42720_e48272: f64 = (assign42720_e48268 / assign42720_e48271);
        (assign42720_e48272, (((((((locals.var_temp2_dn4 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn4)) * locals.var_temp) + (assign42720_e48266 * locals.var_temp_dn4)) * assign42720_e48271) - (assign42720_e48268 * (locals.var_xedgebd_dn4 - locals.var_xstard_dn4))) / (assign42720_e48271 * assign42720_e48271)), (((((((locals.var_temp2_dn6 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn6)) * locals.var_temp) + (assign42720_e48266 * locals.var_temp_dn6)) * assign42720_e48271) - (assign42720_e48268 * (locals.var_xedgebd_dn6 - locals.var_xstard_dn6))) / (assign42720_e48271 * assign42720_e48271)), (((((((locals.var_temp2_dn7 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn7)) * locals.var_temp) + (assign42720_e48266 * locals.var_temp_dn7)) * assign42720_e48271) - (assign42720_e48268 * (locals.var_xedgebd_dn7 - locals.var_xstard_dn7))) / (assign42720_e48271 * assign42720_e48271)), (((((((locals.var_temp2_dn8 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn8)) * locals.var_temp) + (assign42720_e48266 * locals.var_temp_dn8)) * assign42720_e48271) - (assign42720_e48268 * (locals.var_xedgebd_dn8 - locals.var_xstard_dn8))) / (assign42720_e48271 * assign42720_e48271)), (((((((locals.var_temp2_dn9 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn9)) * locals.var_temp) + (assign42720_e48266 * locals.var_temp_dn9)) * assign42720_e48271) - (assign42720_e48268 * (locals.var_xedgebd_dn9 - locals.var_xstard_dn9))) / (assign42720_e48271 * assign42720_e48271)),)
    } else {
        (locals.var_qbdif, locals.var_qbdif_dn4, locals.var_qbdif_dn6, locals.var_qbdif_dn7, locals.var_qbdif_dn8, locals.var_qbdif_dn9,)
    }
};
        locals.var_qbdif = assign42720_e48274;
        locals.var_qbdif_dn4 = assign42720_e48274_d_n4;
        locals.var_qbdif_dn6 = assign42720_e48274_d_n6;
        locals.var_qbdif_dn7 = assign42720_e48274_d_n7;
        locals.var_qbdif_dn8 = assign42720_e48274_d_n8;
        locals.var_qbdif_dn9 = assign42720_e48274_d_n9;

        let (assign42730_e48279, assign42730_e48279_d_n4, assign42730_e48279_d_n6, assign42730_e48279_d_n7, assign42730_e48279_d_n8, assign42730_e48279_d_n9,) = {
    if (locals.var_guard1235 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qgsif, locals.var_qgsif_dn4, locals.var_qgsif_dn6, locals.var_qgsif_dn7, locals.var_qgsif_dn8, locals.var_qgsif_dn9,)
    }
};
        locals.var_qgsif = assign42730_e48279;
        locals.var_qgsif_dn4 = assign42730_e48279_d_n4;
        locals.var_qgsif_dn6 = assign42730_e48279_d_n6;
        locals.var_qgsif_dn7 = assign42730_e48279_d_n7;
        locals.var_qgsif_dn8 = assign42730_e48279_d_n8;
        locals.var_qgsif_dn9 = assign42730_e48279_d_n9;

    }

    pub(super) fn stamp_transient_block_119(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign42740_e48284, assign42740_e48284_d_n4, assign42740_e48284_d_n6, assign42740_e48284_d_n7, assign42740_e48284_d_n8, assign42740_e48284_d_n9,) = {
    if (locals.var_guard1235 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qgdif, locals.var_qgdif_dn4, locals.var_qgdif_dn6, locals.var_qgdif_dn7, locals.var_qgdif_dn8, locals.var_qgdif_dn9,)
    }
};
        locals.var_qgdif = assign42740_e48284;
        locals.var_qgdif_dn4 = assign42740_e48284_d_n4;
        locals.var_qgdif_dn6 = assign42740_e48284_d_n6;
        locals.var_qgdif_dn7 = assign42740_e48284_d_n7;
        locals.var_qgdif_dn8 = assign42740_e48284_d_n8;
        locals.var_qgdif_dn9 = assign42740_e48284_d_n9;

        let (assign42750_e48289, assign42750_e48289_d_n4, assign42750_e48289_d_n6, assign42750_e48289_d_n7, assign42750_e48289_d_n8, assign42750_e48289_d_n9,) = {
    if (locals.var_guard1235 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsif, locals.var_qbsif_dn4, locals.var_qbsif_dn6, locals.var_qbsif_dn7, locals.var_qbsif_dn8, locals.var_qbsif_dn9,)
    }
};
        locals.var_qbsif = assign42750_e48289;
        locals.var_qbsif_dn4 = assign42750_e48289_d_n4;
        locals.var_qbsif_dn6 = assign42750_e48289_d_n6;
        locals.var_qbsif_dn7 = assign42750_e48289_d_n7;
        locals.var_qbsif_dn8 = assign42750_e48289_d_n8;
        locals.var_qbsif_dn9 = assign42750_e48289_d_n9;

        let (assign42760_e48294, assign42760_e48294_d_n4, assign42760_e48294_d_n6, assign42760_e48294_d_n7, assign42760_e48294_d_n8, assign42760_e48294_d_n9,) = {
    if (locals.var_guard1235 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdif, locals.var_qbdif_dn4, locals.var_qbdif_dn6, locals.var_qbdif_dn7, locals.var_qbdif_dn8, locals.var_qbdif_dn9,)
    }
};
        locals.var_qbdif = assign42760_e48294;
        locals.var_qbdif_dn4 = assign42760_e48294_d_n4;
        locals.var_qbdif_dn6 = assign42760_e48294_d_n6;
        locals.var_qbdif_dn7 = assign42760_e48294_d_n7;
        locals.var_qbdif_dn8 = assign42760_e48294_d_n8;
        locals.var_qbdif_dn9 = assign42760_e48294_d_n9;

        let assign42770_e48297: f64 = (locals.var_cfr_i * locals.var_vgsu);
        locals.var_qgse = assign42770_e48297;
        locals.var_qgse_dn4 = (locals.var_cfr_i_dn4 * locals.var_vgsu);
        locals.var_qgse_dn6 = ((locals.var_cfr_i_dn6 * locals.var_vgsu) + (locals.var_cfr_i * locals.var_vgsu_dn6));
        locals.var_qgse_dn7 = (locals.var_cfr_i_dn7 * locals.var_vgsu);
        locals.var_qgse_dn8 = (locals.var_cfr_i_dn8 * locals.var_vgsu);
        locals.var_qgse_dn9 = ((locals.var_cfr_i_dn9 * locals.var_vgsu) + (locals.var_cfr_i * locals.var_vgsu_dn9));

        let assign42780_e48300: f64 = (locals.var_cfrd_i * locals.var_vgdu);
        locals.var_qgde = assign42780_e48300;
        locals.var_qgde_dn4 = (locals.var_cfrd_i_dn4 * locals.var_vgdu);
        locals.var_qgde_dn6 = ((locals.var_cfrd_i_dn6 * locals.var_vgdu) + (locals.var_cfrd_i * locals.var_vgdu_dn6));
        locals.var_qgde_dn7 = ((locals.var_cfrd_i_dn7 * locals.var_vgdu) + (locals.var_cfrd_i * locals.var_vgdu_dn7));
        locals.var_qgde_dn8 = (locals.var_cfrd_i_dn8 * locals.var_vgdu);
        locals.var_qgde_dn9 = ((locals.var_cfrd_i_dn9 * locals.var_vgdu) + (locals.var_cfrd_i * locals.var_vgdu_dn9));

        let assign42790_e48305: f64 = (locals.var_covdl_i * locals.var_dleff_ac);
        let assign42790_e48309: f64 = (locals.var_covdlb_i * locals.var_xg20shift_ac);
        let assign42790_e48310: f64 = (1.0 - assign42790_e48309);
        let assign42790_e48311: f64 = (assign42790_e48305 * assign42790_e48310);
        let assign42790_e48312: f64 = (1.0 - assign42790_e48311);
        let assign42790_e48314: f64 = assign42790_e48312;
        let assign42790_e48318: f64 = (locals.var_covdl_i * locals.var_dleff_ac);
        let assign42790_e48322: f64 = (locals.var_covdlb_i * locals.var_xg20shift_ac);
        let assign42790_e48323: f64 = (1.0 - assign42790_e48322);
        let assign42790_e48324: f64 = (assign42790_e48318 * assign42790_e48323);
        let assign42790_e48325: f64 = (1.0 - assign42790_e48324);
        let assign42790_e48327: f64 = assign42790_e48325;
        let assign42790_e48331: f64 = (locals.var_covdl_i * locals.var_dleff_ac);
        let assign42790_e48335: f64 = (locals.var_covdlb_i * locals.var_xg20shift_ac);
        let assign42790_e48336: f64 = (1.0 - assign42790_e48335);
        let assign42790_e48337: f64 = (assign42790_e48331 * assign42790_e48336);
        let assign42790_e48338: f64 = (1.0 - assign42790_e48337);
        let assign42790_e48340: f64 = assign42790_e48338;
        let assign42790_e48341: f64 = (assign42790_e48327 * assign42790_e48340);
        let assign42790_e48343: f64 = (assign42790_e48341 + 0.2);
        let assign42790_e48344: f64 = (assign42790_e48343).sqrt();
        let assign42790_e48345: f64 = (assign42790_e48314 + assign42790_e48344);
        let assign42790_e48346: f64 = (0.5 * assign42790_e48345);
        locals.var_temp = assign42790_e48346;
        locals.var_temp_dn4 = (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn4) * assign42790_e48310) + (assign42790_e48305 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn4))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn4) * assign42790_e48323) + (assign42790_e48318 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn4))))) * assign42790_e48340) + (assign42790_e48327 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn4) * assign42790_e48336) + (assign42790_e48331 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn4))))))) / (2.0 * assign42790_e48344))));
        locals.var_temp_dn6 = (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn6) * assign42790_e48310) + (assign42790_e48305 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn6))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn6) * assign42790_e48323) + (assign42790_e48318 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn6))))) * assign42790_e48340) + (assign42790_e48327 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn6) * assign42790_e48336) + (assign42790_e48331 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn6))))))) / (2.0 * assign42790_e48344))));
        locals.var_temp_dn7 = (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn7) * assign42790_e48310) + (assign42790_e48305 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn7))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn7) * assign42790_e48323) + (assign42790_e48318 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn7))))) * assign42790_e48340) + (assign42790_e48327 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn7) * assign42790_e48336) + (assign42790_e48331 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn7))))))) / (2.0 * assign42790_e48344))));
        locals.var_temp_dn8 = (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn8) * assign42790_e48310) + (assign42790_e48305 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn8))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn8) * assign42790_e48323) + (assign42790_e48318 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn8))))) * assign42790_e48340) + (assign42790_e48327 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn8) * assign42790_e48336) + (assign42790_e48331 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn8))))))) / (2.0 * assign42790_e48344))));
        locals.var_temp_dn9 = (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn9) * assign42790_e48310) + (assign42790_e48305 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn9))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn9) * assign42790_e48323) + (assign42790_e48318 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn9))))) * assign42790_e48340) + (assign42790_e48327 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn9) * assign42790_e48336) + (assign42790_e48331 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn9))))))) / (2.0 * assign42790_e48344))));

        let assign42800_e48349: f64 = (locals.var_cov_i * locals.var_vovscv);
        let assign42800_e48351: f64 = (assign42800_e48349 * locals.var_temp);
        locals.var_qovs = assign42800_e48351;
        locals.var_qovs_dn4 = ((((locals.var_cov_i_dn4 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn4)) * locals.var_temp) + (assign42800_e48349 * locals.var_temp_dn4));
        locals.var_qovs_dn6 = ((((locals.var_cov_i_dn6 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn6)) * locals.var_temp) + (assign42800_e48349 * locals.var_temp_dn6));
        locals.var_qovs_dn7 = ((((locals.var_cov_i_dn7 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn7)) * locals.var_temp) + (assign42800_e48349 * locals.var_temp_dn7));
        locals.var_qovs_dn8 = ((((locals.var_cov_i_dn8 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn8)) * locals.var_temp) + (assign42800_e48349 * locals.var_temp_dn8));
        locals.var_qovs_dn9 = ((((locals.var_cov_i_dn9 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn9)) * locals.var_temp) + (assign42800_e48349 * locals.var_temp_dn9));

        let assign42810_e48354: f64 = (locals.var_covd_i * locals.var_vovdcv);
        let assign42810_e48356: f64 = (assign42810_e48354 * locals.var_temp);
        locals.var_qovd = assign42810_e48356;
        locals.var_qovd_dn4 = ((((locals.var_covd_i_dn4 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn4)) * locals.var_temp) + (assign42810_e48354 * locals.var_temp_dn4));
        locals.var_qovd_dn6 = ((((locals.var_covd_i_dn6 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn6)) * locals.var_temp) + (assign42810_e48354 * locals.var_temp_dn6));
        locals.var_qovd_dn7 = ((((locals.var_covd_i_dn7 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn7)) * locals.var_temp) + (assign42810_e48354 * locals.var_temp_dn7));
        locals.var_qovd_dn8 = ((((locals.var_covd_i_dn8 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn8)) * locals.var_temp) + (assign42810_e48354 * locals.var_temp_dn8));
        locals.var_qovd_dn9 = ((((locals.var_covd_i_dn9 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn9)) * locals.var_temp) + (assign42810_e48354 * locals.var_temp_dn9));

        let assign42820_e48359: f64 = (locals.var_cgbov_i * locals.var_vgb);
        locals.var_qgbe = assign42820_e48359;
        locals.var_qgbe_dn4 = (locals.var_cgbov_i_dn4 * locals.var_vgb);
        locals.var_qgbe_dn6 = ((locals.var_cgbov_i_dn6 * locals.var_vgb) + (locals.var_cgbov_i * locals.var_vgb_dn6));
        locals.var_qgbe_dn7 = ((locals.var_cgbov_i_dn7 * locals.var_vgb) + (locals.var_cgbov_i * locals.var_vgb_dn7));
        locals.var_qgbe_dn8 = ((locals.var_cgbov_i_dn8 * locals.var_vgb) + (locals.var_cgbov_i * locals.var_vgb_dn8));
        locals.var_qgbe_dn9 = ((locals.var_cgbov_i_dn9 * locals.var_vgb) + (locals.var_cgbov_i * locals.var_vgb_dn9));

        let assign42830_e48362: f64 = (locals.var_csd_i * locals.var_vds);
        locals.var_qdse = assign42830_e48362;
        locals.var_qdse_dn6 = (locals.var_csd_i * locals.var_vds_dn6);
        locals.var_qdse_dn7 = (locals.var_csd_i * locals.var_vds_dn7);

        let assign42840_e48365: f64 = (locals.var_cox2init * locals.var_asource_i);
        let assign42840_e48368: f64 = (locals.var_csdbp_i * locals.var_psource_i);
        let assign42840_e48369: f64 = (assign42840_e48365 + assign42840_e48368);
        let assign42840_e48370: f64 = (-assign42840_e48369);
        let assign42840_e48372: f64 = (assign42840_e48370 * locals.var_vsbu);
        locals.var_qssub = assign42840_e48372;
        locals.var_qssub_dn6 = (assign42840_e48370 * locals.var_vsbu_dn6);
        locals.var_qssub_dn8 = (assign42840_e48370 * locals.var_vsbu_dn8);

        let assign42850_e48375: f64 = (locals.var_cox2init * locals.var_adrain_i);
        let assign42850_e48378: f64 = (locals.var_csdbp_i * locals.var_pdrain_i);
        let assign42850_e48379: f64 = (assign42850_e48375 + assign42850_e48378);
        let assign42850_e48380: f64 = (-assign42850_e48379);
        let assign42850_e48382: f64 = (assign42850_e48380 * locals.var_vdbu);
        locals.var_qdsub = assign42850_e48382;
        locals.var_qdsub_dn6 = (assign42850_e48380 * locals.var_vdbu_dn6);
        locals.var_qdsub_dn7 = (assign42850_e48380 * locals.var_vdbu_dn7);
        locals.var_qdsub_dn8 = (assign42850_e48380 * locals.var_vdbu_dn8);

        let assign42860_e48385: f64 = if locals.var_swshe_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1236 = assign42860_e48385;

        let (assign42870_e48391, assign42870_e48391_d_n4, assign42870_e48391_d_n6, assign42870_e48391_d_n7, assign42870_e48391_d_n8, assign42870_e48391_d_n9,) = {
    if (locals.var_guard1236 != 0.0) {
        let assign42870_e48389: f64 = (locals.var_cth_i * locals.var_dtc);
        (assign42870_e48389, ((locals.var_cth_i_dn4 * locals.var_dtc) + (locals.var_cth_i * locals.var_dtc_dn4)), (locals.var_cth_i_dn6 * locals.var_dtc), (locals.var_cth_i_dn7 * locals.var_dtc), (locals.var_cth_i_dn8 * locals.var_dtc), (locals.var_cth_i_dn9 * locals.var_dtc),)
    } else {
        (locals.var_qth, locals.var_qth_dn4, locals.var_qth_dn6, locals.var_qth_dn7, locals.var_qth_dn8, locals.var_qth_dn9,)
    }
};
        locals.var_qth = assign42870_e48391;
        locals.var_qth_dn4 = assign42870_e48391_d_n4;
        locals.var_qth_dn6 = assign42870_e48391_d_n6;
        locals.var_qth_dn7 = assign42870_e48391_d_n7;
        locals.var_qth_dn8 = assign42870_e48391_d_n8;
        locals.var_qth_dn9 = assign42870_e48391_d_n9;

        let (assign42880_e48396, assign42880_e48396_d_n4, assign42880_e48396_d_n6, assign42880_e48396_d_n7, assign42880_e48396_d_n8, assign42880_e48396_d_n9,) = {
    if (locals.var_guard1236 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qth, locals.var_qth_dn4, locals.var_qth_dn6, locals.var_qth_dn7, locals.var_qth_dn8, locals.var_qth_dn9,)
    }
};
        locals.var_qth = assign42880_e48396;
        locals.var_qth_dn4 = assign42880_e48396_d_n4;
        locals.var_qth_dn6 = assign42880_e48396_d_n6;
        locals.var_qth_dn7 = assign42880_e48396_d_n7;
        locals.var_qth_dn8 = assign42880_e48396_d_n8;
        locals.var_qth_dn9 = assign42880_e48396_d_n9;

        let assign42890_e48399: f64 = (p.p31 * locals.var_mult_i_int);
        let assign42890_e48402: f64 = (locals.var_ids + locals.var_ids_edge);
        let assign42890_e48404: f64 = (assign42890_e48402 + locals.var_iimpact);
        let assign42890_e48405: f64 = (assign42890_e48399 * assign42890_e48404);
        locals.var_idse = assign42890_e48405;
        locals.var_idse_dn4 = (assign42890_e48399 * ((locals.var_ids_dn4 + locals.var_ids_edge_dn4) + locals.var_iimpact_dn4));
        locals.var_idse_dn6 = (assign42890_e48399 * ((locals.var_ids_dn6 + locals.var_ids_edge_dn6) + locals.var_iimpact_dn6));
        locals.var_idse_dn7 = (assign42890_e48399 * ((locals.var_ids_dn7 + locals.var_ids_edge_dn7) + locals.var_iimpact_dn7));
        locals.var_idse_dn8 = (assign42890_e48399 * ((locals.var_ids_dn8 + locals.var_ids_edge_dn8) + locals.var_iimpact_dn8));
        locals.var_idse_dn9 = (assign42890_e48399 * ((locals.var_ids_dn9 + locals.var_ids_edge_dn9) + locals.var_iimpact_dn9));

        let assign42900_e48408: f64 = (p.p31 * locals.var_mult_i_int);
        let assign42900_e48410: f64 = (assign42900_e48408 * locals.var_igs);
        locals.var_igse = assign42900_e48410;
        locals.var_igse_dn4 = (assign42900_e48408 * locals.var_igs_dn4);
        locals.var_igse_dn6 = (assign42900_e48408 * locals.var_igs_dn6);
        locals.var_igse_dn7 = (assign42900_e48408 * locals.var_igs_dn7);
        locals.var_igse_dn8 = (assign42900_e48408 * locals.var_igs_dn8);
        locals.var_igse_dn9 = (assign42900_e48408 * locals.var_igs_dn9);

        let assign42910_e48413: f64 = (p.p31 * locals.var_mult_i_int);
        let assign42910_e48415: f64 = (assign42910_e48413 * locals.var_igd);
        locals.var_igde = assign42910_e48415;
        locals.var_igde_dn4 = (assign42910_e48413 * locals.var_igd_dn4);
        locals.var_igde_dn6 = (assign42910_e48413 * locals.var_igd_dn6);
        locals.var_igde_dn7 = (assign42910_e48413 * locals.var_igd_dn7);
        locals.var_igde_dn8 = (assign42910_e48413 * locals.var_igd_dn8);
        locals.var_igde_dn9 = (assign42910_e48413 * locals.var_igd_dn9);

        let assign42920_e48418: f64 = (p.p31 * locals.var_mult_i_int);
        let assign42920_e48420: f64 = (assign42920_e48418 * locals.var_igidl);
        locals.var_igidle = assign42920_e48420;
        locals.var_igidle_dn4 = (assign42920_e48418 * locals.var_igidl_dn4);
        locals.var_igidle_dn6 = (assign42920_e48418 * locals.var_igidl_dn6);
        locals.var_igidle_dn7 = (assign42920_e48418 * locals.var_igidl_dn7);
        locals.var_igidle_dn8 = (assign42920_e48418 * locals.var_igidl_dn8);
        locals.var_igidle_dn9 = (assign42920_e48418 * locals.var_igidl_dn9);

        let assign42930_e48423: f64 = (p.p31 * locals.var_mult_i_int);
        let assign42930_e48425: f64 = (assign42930_e48423 * locals.var_igisl);
        locals.var_igisle = assign42930_e48425;
        locals.var_igisle_dn4 = (assign42930_e48423 * locals.var_igisl_dn4);
        locals.var_igisle_dn6 = (assign42930_e48423 * locals.var_igisl_dn6);
        locals.var_igisle_dn7 = (assign42930_e48423 * locals.var_igisl_dn7);
        locals.var_igisle_dn8 = (assign42930_e48423 * locals.var_igisl_dn8);
        locals.var_igisle_dn9 = (assign42930_e48423 * locals.var_igisl_dn9);

        let assign42940_e48428: f64 = (locals.var_mult_i_int * locals.var_ithpwr);
        locals.var_ithpwre = assign42940_e48428;
        locals.var_ithpwre_dn4 = (locals.var_mult_i_int * locals.var_ithpwr_dn4);
        locals.var_ithpwre_dn6 = (locals.var_mult_i_int * locals.var_ithpwr_dn6);
        locals.var_ithpwre_dn7 = (locals.var_mult_i_int * locals.var_ithpwr_dn7);
        locals.var_ithpwre_dn8 = (locals.var_mult_i_int * locals.var_ithpwr_dn8);
        locals.var_ithpwre_dn9 = (locals.var_mult_i_int * locals.var_ithpwr_dn9);

        let assign42950_e48431: f64 = (locals.var_mult_i_int * locals.var_ithrc);
        locals.var_ithrce = assign42950_e48431;
        locals.var_ithrce_dn4 = (locals.var_mult_i_int * locals.var_ithrc_dn4);
        locals.var_ithrce_dn6 = (locals.var_mult_i_int * locals.var_ithrc_dn6);
        locals.var_ithrce_dn7 = (locals.var_mult_i_int * locals.var_ithrc_dn7);
        locals.var_ithrce_dn8 = (locals.var_mult_i_int * locals.var_ithrc_dn8);
        locals.var_ithrce_dn9 = (locals.var_mult_i_int * locals.var_ithrc_dn9);

        let assign42960_e48434: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1239 = assign42960_e48434;

        locals.var_qg_wo_mult = locals.var_qg;
        locals.var_qg_wo_mult_dn4 = locals.var_qg_dn4;
        locals.var_qg_wo_mult_dn6 = locals.var_qg_dn6;
        locals.var_qg_wo_mult_dn7 = locals.var_qg_dn7;
        locals.var_qg_wo_mult_dn8 = locals.var_qg_dn8;
        locals.var_qg_wo_mult_dn9 = locals.var_qg_dn9;

        locals.var_qb_wo_mult = locals.var_qb;
        locals.var_qb_wo_mult_dn4 = locals.var_qb_dn4;
        locals.var_qb_wo_mult_dn6 = locals.var_qb_dn6;
        locals.var_qb_wo_mult_dn7 = locals.var_qb_dn7;
        locals.var_qb_wo_mult_dn8 = locals.var_qb_dn8;
        locals.var_qb_wo_mult_dn9 = locals.var_qb_dn9;

        locals.var_qd_wo_mult = locals.var_qd;
        locals.var_qd_wo_mult_dn4 = locals.var_qd_dn4;
        locals.var_qd_wo_mult_dn6 = locals.var_qd_dn6;
        locals.var_qd_wo_mult_dn7 = locals.var_qd_dn7;
        locals.var_qd_wo_mult_dn8 = locals.var_qd_dn8;
        locals.var_qd_wo_mult_dn9 = locals.var_qd_dn9;

        let assign43040_e48452: f64 = (locals.var_qg + locals.var_qb);
        let assign43040_e48454: f64 = (assign43040_e48452 + locals.var_qd);
        let assign43040_e48455: f64 = (-assign43040_e48454);
        locals.var_qs = assign43040_e48455;
        locals.var_qs_dn4 = (-((locals.var_qg_dn4 + locals.var_qb_dn4) + locals.var_qd_dn4));
        locals.var_qs_dn6 = (-((locals.var_qg_dn6 + locals.var_qb_dn6) + locals.var_qd_dn6));
        locals.var_qs_dn7 = (-((locals.var_qg_dn7 + locals.var_qb_dn7) + locals.var_qd_dn7));
        locals.var_qs_dn8 = (-((locals.var_qg_dn8 + locals.var_qb_dn8) + locals.var_qd_dn8));
        locals.var_qs_dn9 = (-((locals.var_qg_dn9 + locals.var_qb_dn9) + locals.var_qd_dn9));

        let assign43050_e48458: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1249 = assign43050_e48458;

        let (assign43060_e48462, assign43060_e48462_d_n4, assign43060_e48462_d_n6, assign43060_e48462_d_n7, assign43060_e48462_d_n8, assign43060_e48462_d_n9,) = {
    if (locals.var_guard1249 != 0.0) {
        (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9,)
    } else {
        (locals.var_qd_wo_mult, locals.var_qd_wo_mult_dn4, locals.var_qd_wo_mult_dn6, locals.var_qd_wo_mult_dn7, locals.var_qd_wo_mult_dn8, locals.var_qd_wo_mult_dn9,)
    }
};
        locals.var_qd_wo_mult = assign43060_e48462;
        locals.var_qd_wo_mult_dn4 = assign43060_e48462_d_n4;
        locals.var_qd_wo_mult_dn6 = assign43060_e48462_d_n6;
        locals.var_qd_wo_mult_dn7 = assign43060_e48462_d_n7;
        locals.var_qd_wo_mult_dn8 = assign43060_e48462_d_n8;
        locals.var_qd_wo_mult_dn9 = assign43060_e48462_d_n9;

        let assign43070_e48465: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43070_e48467: f64 = (assign43070_e48465 * locals.var_qg);
        locals.var_qg = assign43070_e48467;
        locals.var_qg_dn4 = (assign43070_e48465 * locals.var_qg_dn4);
        locals.var_qg_dn6 = (assign43070_e48465 * locals.var_qg_dn6);
        locals.var_qg_dn7 = (assign43070_e48465 * locals.var_qg_dn7);
        locals.var_qg_dn8 = (assign43070_e48465 * locals.var_qg_dn8);
        locals.var_qg_dn9 = (assign43070_e48465 * locals.var_qg_dn9);

        let assign43080_e48470: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43080_e48472: f64 = (assign43080_e48470 * locals.var_qb);
        locals.var_qb = assign43080_e48472;
        locals.var_qb_dn4 = (assign43080_e48470 * locals.var_qb_dn4);
        locals.var_qb_dn6 = (assign43080_e48470 * locals.var_qb_dn6);
        locals.var_qb_dn7 = (assign43080_e48470 * locals.var_qb_dn7);
        locals.var_qb_dn8 = (assign43080_e48470 * locals.var_qb_dn8);
        locals.var_qb_dn9 = (assign43080_e48470 * locals.var_qb_dn9);

        let assign43090_e48475: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43090_e48477: f64 = (assign43090_e48475 * locals.var_qd);
        locals.var_qd = assign43090_e48477;
        locals.var_qd_dn4 = (assign43090_e48475 * locals.var_qd_dn4);
        locals.var_qd_dn6 = (assign43090_e48475 * locals.var_qd_dn6);
        locals.var_qd_dn7 = (assign43090_e48475 * locals.var_qd_dn7);
        locals.var_qd_dn8 = (assign43090_e48475 * locals.var_qd_dn8);
        locals.var_qd_dn9 = (assign43090_e48475 * locals.var_qd_dn9);

        let assign43100_e48480: f64 = (locals.var_qg + locals.var_qb);
        let assign43100_e48482: f64 = (assign43100_e48480 + locals.var_qd);
        let assign43100_e48483: f64 = (-assign43100_e48482);
        locals.var_qs = assign43100_e48483;
        locals.var_qs_dn4 = (-((locals.var_qg_dn4 + locals.var_qb_dn4) + locals.var_qd_dn4));
        locals.var_qs_dn6 = (-((locals.var_qg_dn6 + locals.var_qb_dn6) + locals.var_qd_dn6));
        locals.var_qs_dn7 = (-((locals.var_qg_dn7 + locals.var_qb_dn7) + locals.var_qd_dn7));
        locals.var_qs_dn8 = (-((locals.var_qg_dn8 + locals.var_qb_dn8) + locals.var_qd_dn8));
        locals.var_qs_dn9 = (-((locals.var_qg_dn9 + locals.var_qb_dn9) + locals.var_qd_dn9));

        let assign43110_e48486: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43110_e48488: f64 = (assign43110_e48486 * locals.var_qgsif);
        locals.var_qgsif = assign43110_e48488;
        locals.var_qgsif_dn4 = (assign43110_e48486 * locals.var_qgsif_dn4);
        locals.var_qgsif_dn6 = (assign43110_e48486 * locals.var_qgsif_dn6);
        locals.var_qgsif_dn7 = (assign43110_e48486 * locals.var_qgsif_dn7);
        locals.var_qgsif_dn8 = (assign43110_e48486 * locals.var_qgsif_dn8);
        locals.var_qgsif_dn9 = (assign43110_e48486 * locals.var_qgsif_dn9);

        let assign43120_e48491: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43120_e48493: f64 = (assign43120_e48491 * locals.var_qgdif);
        locals.var_qgdif = assign43120_e48493;
        locals.var_qgdif_dn4 = (assign43120_e48491 * locals.var_qgdif_dn4);
        locals.var_qgdif_dn6 = (assign43120_e48491 * locals.var_qgdif_dn6);
        locals.var_qgdif_dn7 = (assign43120_e48491 * locals.var_qgdif_dn7);
        locals.var_qgdif_dn8 = (assign43120_e48491 * locals.var_qgdif_dn8);
        locals.var_qgdif_dn9 = (assign43120_e48491 * locals.var_qgdif_dn9);

        let assign43130_e48496: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43130_e48498: f64 = (assign43130_e48496 * locals.var_qbsif);
        locals.var_qbsif = assign43130_e48498;
        locals.var_qbsif_dn4 = (assign43130_e48496 * locals.var_qbsif_dn4);
        locals.var_qbsif_dn6 = (assign43130_e48496 * locals.var_qbsif_dn6);
        locals.var_qbsif_dn7 = (assign43130_e48496 * locals.var_qbsif_dn7);
        locals.var_qbsif_dn8 = (assign43130_e48496 * locals.var_qbsif_dn8);
        locals.var_qbsif_dn9 = (assign43130_e48496 * locals.var_qbsif_dn9);

        let assign43140_e48501: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43140_e48503: f64 = (assign43140_e48501 * locals.var_qbdif);
        locals.var_qbdif = assign43140_e48503;
        locals.var_qbdif_dn4 = (assign43140_e48501 * locals.var_qbdif_dn4);
        locals.var_qbdif_dn6 = (assign43140_e48501 * locals.var_qbdif_dn6);
        locals.var_qbdif_dn7 = (assign43140_e48501 * locals.var_qbdif_dn7);
        locals.var_qbdif_dn8 = (assign43140_e48501 * locals.var_qbdif_dn8);
        locals.var_qbdif_dn9 = (assign43140_e48501 * locals.var_qbdif_dn9);

        let assign43150_e48506: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43150_e48508: f64 = (assign43150_e48506 * locals.var_qgse);
        locals.var_qgse = assign43150_e48508;
        locals.var_qgse_dn4 = (assign43150_e48506 * locals.var_qgse_dn4);
        locals.var_qgse_dn6 = (assign43150_e48506 * locals.var_qgse_dn6);
        locals.var_qgse_dn7 = (assign43150_e48506 * locals.var_qgse_dn7);
        locals.var_qgse_dn8 = (assign43150_e48506 * locals.var_qgse_dn8);
        locals.var_qgse_dn9 = (assign43150_e48506 * locals.var_qgse_dn9);

        let assign43160_e48511: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43160_e48513: f64 = (assign43160_e48511 * locals.var_qgde);
        locals.var_qgde = assign43160_e48513;
        locals.var_qgde_dn4 = (assign43160_e48511 * locals.var_qgde_dn4);
        locals.var_qgde_dn6 = (assign43160_e48511 * locals.var_qgde_dn6);
        locals.var_qgde_dn7 = (assign43160_e48511 * locals.var_qgde_dn7);
        locals.var_qgde_dn8 = (assign43160_e48511 * locals.var_qgde_dn8);
        locals.var_qgde_dn9 = (assign43160_e48511 * locals.var_qgde_dn9);

        let assign43170_e48516: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43170_e48518: f64 = (assign43170_e48516 * locals.var_qovs);
        locals.var_qovs = assign43170_e48518;
        locals.var_qovs_dn4 = (assign43170_e48516 * locals.var_qovs_dn4);
        locals.var_qovs_dn6 = (assign43170_e48516 * locals.var_qovs_dn6);
        locals.var_qovs_dn7 = (assign43170_e48516 * locals.var_qovs_dn7);
        locals.var_qovs_dn8 = (assign43170_e48516 * locals.var_qovs_dn8);
        locals.var_qovs_dn9 = (assign43170_e48516 * locals.var_qovs_dn9);

        let assign43180_e48521: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43180_e48523: f64 = (assign43180_e48521 * locals.var_qovd);
        locals.var_qovd = assign43180_e48523;
        locals.var_qovd_dn4 = (assign43180_e48521 * locals.var_qovd_dn4);
        locals.var_qovd_dn6 = (assign43180_e48521 * locals.var_qovd_dn6);
        locals.var_qovd_dn7 = (assign43180_e48521 * locals.var_qovd_dn7);
        locals.var_qovd_dn8 = (assign43180_e48521 * locals.var_qovd_dn8);
        locals.var_qovd_dn9 = (assign43180_e48521 * locals.var_qovd_dn9);

        let assign43190_e48526: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43190_e48528: f64 = (assign43190_e48526 * locals.var_qgbe);
        locals.var_qgbe = assign43190_e48528;
        locals.var_qgbe_dn4 = (assign43190_e48526 * locals.var_qgbe_dn4);
        locals.var_qgbe_dn6 = (assign43190_e48526 * locals.var_qgbe_dn6);
        locals.var_qgbe_dn7 = (assign43190_e48526 * locals.var_qgbe_dn7);
        locals.var_qgbe_dn8 = (assign43190_e48526 * locals.var_qgbe_dn8);
        locals.var_qgbe_dn9 = (assign43190_e48526 * locals.var_qgbe_dn9);

        let assign43200_e48531: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43200_e48533: f64 = (assign43200_e48531 * locals.var_qssub);
        locals.var_qssub = assign43200_e48533;
        locals.var_qssub_dn6 = (assign43200_e48531 * locals.var_qssub_dn6);
        locals.var_qssub_dn8 = (assign43200_e48531 * locals.var_qssub_dn8);

        let assign43210_e48536: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43210_e48538: f64 = (assign43210_e48536 * locals.var_qdsub);
        locals.var_qdsub = assign43210_e48538;
        locals.var_qdsub_dn6 = (assign43210_e48536 * locals.var_qdsub_dn6);
        locals.var_qdsub_dn7 = (assign43210_e48536 * locals.var_qdsub_dn7);
        locals.var_qdsub_dn8 = (assign43210_e48536 * locals.var_qdsub_dn8);

        let assign43220_e48541: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43220_e48543: f64 = (assign43220_e48541 * locals.var_qdse);
        locals.var_qdse = assign43220_e48543;
        locals.var_qdse_dn6 = (assign43220_e48541 * locals.var_qdse_dn6);
        locals.var_qdse_dn7 = (assign43220_e48541 * locals.var_qdse_dn7);

        let assign43230_e48546: f64 = (locals.var_mult_i_int * locals.var_qth);
        locals.var_qth = assign43230_e48546;
        locals.var_qth_dn4 = (locals.var_mult_i_int * locals.var_qth_dn4);
        locals.var_qth_dn6 = (locals.var_mult_i_int * locals.var_qth_dn6);
        locals.var_qth_dn7 = (locals.var_mult_i_int * locals.var_qth_dn7);
        locals.var_qth_dn8 = (locals.var_mult_i_int * locals.var_qth_dn8);
        locals.var_qth_dn9 = (locals.var_mult_i_int * locals.var_qth_dn9);

        let assign43240_e48549: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1250 = assign43240_e48549;

        let (assign43250_e48553, assign43250_e48553_d_n4, assign43250_e48553_d_n6, assign43250_e48553_d_n7, assign43250_e48553_d_n8, assign43250_e48553_d_n9,) = {
    if (locals.var_guard1250 != 0.0) {
        (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9,)
    } else {
        (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9,)
    }
};
        locals.var_temp_q = assign43250_e48553;
        locals.var_temp_q_dn4 = assign43250_e48553_d_n4;
        locals.var_temp_q_dn6 = assign43250_e48553_d_n6;
        locals.var_temp_q_dn7 = assign43250_e48553_d_n7;
        locals.var_temp_q_dn8 = assign43250_e48553_d_n8;
        locals.var_temp_q_dn9 = assign43250_e48553_d_n9;

        let (assign43260_e48557, assign43260_e48557_d_n4, assign43260_e48557_d_n6, assign43260_e48557_d_n7, assign43260_e48557_d_n8, assign43260_e48557_d_n9,) = {
    if (locals.var_guard1250 != 0.0) {
        (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9,)
    } else {
        (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9,)
    }
};
        locals.var_qd = assign43260_e48557;
        locals.var_qd_dn4 = assign43260_e48557_d_n4;
        locals.var_qd_dn6 = assign43260_e48557_d_n6;
        locals.var_qd_dn7 = assign43260_e48557_d_n7;
        locals.var_qd_dn8 = assign43260_e48557_d_n8;
        locals.var_qd_dn9 = assign43260_e48557_d_n9;

        let (assign43270_e48561, assign43270_e48561_d_n4, assign43270_e48561_d_n6, assign43270_e48561_d_n7, assign43270_e48561_d_n8, assign43270_e48561_d_n9,) = {
    if (locals.var_guard1250 != 0.0) {
        (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9,)
    } else {
        (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9,)
    }
};
        locals.var_qs = assign43270_e48561;
        locals.var_qs_dn4 = assign43270_e48561_d_n4;
        locals.var_qs_dn6 = assign43270_e48561_d_n6;
        locals.var_qs_dn7 = assign43270_e48561_d_n7;
        locals.var_qs_dn8 = assign43270_e48561_d_n8;
        locals.var_qs_dn9 = assign43270_e48561_d_n9;

        let (assign43280_e48566, assign43280_e48566_d_n6, assign43280_e48566_d_n7,) = {
    if (locals.var_guard1250 != 0.0) {
        let assign43280_e48564: f64 = (-locals.var_qdse);
        (assign43280_e48564, (-locals.var_qdse_dn6), (-locals.var_qdse_dn7),)
    } else {
        (locals.var_qdse, locals.var_qdse_dn6, locals.var_qdse_dn7,)
    }
};
        locals.var_qdse = assign43280_e48566;
        locals.var_qdse_dn6 = assign43280_e48566_d_n6;
        locals.var_qdse_dn7 = assign43280_e48566_d_n7;

        let (assign43290_e48570, assign43290_e48570_d_n4, assign43290_e48570_d_n6, assign43290_e48570_d_n7, assign43290_e48570_d_n8, assign43290_e48570_d_n9,) = {
    if (locals.var_guard1250 != 0.0) {
        (locals.var_qgdif, locals.var_qgdif_dn4, locals.var_qgdif_dn6, locals.var_qgdif_dn7, locals.var_qgdif_dn8, locals.var_qgdif_dn9,)
    } else {
        (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9,)
    }
};
        locals.var_temp_q = assign43290_e48570;
        locals.var_temp_q_dn4 = assign43290_e48570_d_n4;
        locals.var_temp_q_dn6 = assign43290_e48570_d_n6;
        locals.var_temp_q_dn7 = assign43290_e48570_d_n7;
        locals.var_temp_q_dn8 = assign43290_e48570_d_n8;
        locals.var_temp_q_dn9 = assign43290_e48570_d_n9;

    }

    pub(super) fn stamp_transient_block_120(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign43300_e48574, assign43300_e48574_d_n4, assign43300_e48574_d_n6, assign43300_e48574_d_n7, assign43300_e48574_d_n8, assign43300_e48574_d_n9,) = {
    if (locals.var_guard1250 != 0.0) {
        (locals.var_qgsif, locals.var_qgsif_dn4, locals.var_qgsif_dn6, locals.var_qgsif_dn7, locals.var_qgsif_dn8, locals.var_qgsif_dn9,)
    } else {
        (locals.var_qgdif, locals.var_qgdif_dn4, locals.var_qgdif_dn6, locals.var_qgdif_dn7, locals.var_qgdif_dn8, locals.var_qgdif_dn9,)
    }
};
        locals.var_qgdif = assign43300_e48574;
        locals.var_qgdif_dn4 = assign43300_e48574_d_n4;
        locals.var_qgdif_dn6 = assign43300_e48574_d_n6;
        locals.var_qgdif_dn7 = assign43300_e48574_d_n7;
        locals.var_qgdif_dn8 = assign43300_e48574_d_n8;
        locals.var_qgdif_dn9 = assign43300_e48574_d_n9;

        let (assign43310_e48578, assign43310_e48578_d_n4, assign43310_e48578_d_n6, assign43310_e48578_d_n7, assign43310_e48578_d_n8, assign43310_e48578_d_n9,) = {
    if (locals.var_guard1250 != 0.0) {
        (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9,)
    } else {
        (locals.var_qgsif, locals.var_qgsif_dn4, locals.var_qgsif_dn6, locals.var_qgsif_dn7, locals.var_qgsif_dn8, locals.var_qgsif_dn9,)
    }
};
        locals.var_qgsif = assign43310_e48578;
        locals.var_qgsif_dn4 = assign43310_e48578_d_n4;
        locals.var_qgsif_dn6 = assign43310_e48578_d_n6;
        locals.var_qgsif_dn7 = assign43310_e48578_d_n7;
        locals.var_qgsif_dn8 = assign43310_e48578_d_n8;
        locals.var_qgsif_dn9 = assign43310_e48578_d_n9;

        let (assign43320_e48582, assign43320_e48582_d_n4, assign43320_e48582_d_n6, assign43320_e48582_d_n7, assign43320_e48582_d_n8, assign43320_e48582_d_n9,) = {
    if (locals.var_guard1250 != 0.0) {
        (locals.var_qbdif, locals.var_qbdif_dn4, locals.var_qbdif_dn6, locals.var_qbdif_dn7, locals.var_qbdif_dn8, locals.var_qbdif_dn9,)
    } else {
        (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9,)
    }
};
        locals.var_temp_q = assign43320_e48582;
        locals.var_temp_q_dn4 = assign43320_e48582_d_n4;
        locals.var_temp_q_dn6 = assign43320_e48582_d_n6;
        locals.var_temp_q_dn7 = assign43320_e48582_d_n7;
        locals.var_temp_q_dn8 = assign43320_e48582_d_n8;
        locals.var_temp_q_dn9 = assign43320_e48582_d_n9;

        let (assign43330_e48586, assign43330_e48586_d_n4, assign43330_e48586_d_n6, assign43330_e48586_d_n7, assign43330_e48586_d_n8, assign43330_e48586_d_n9,) = {
    if (locals.var_guard1250 != 0.0) {
        (locals.var_qbsif, locals.var_qbsif_dn4, locals.var_qbsif_dn6, locals.var_qbsif_dn7, locals.var_qbsif_dn8, locals.var_qbsif_dn9,)
    } else {
        (locals.var_qbdif, locals.var_qbdif_dn4, locals.var_qbdif_dn6, locals.var_qbdif_dn7, locals.var_qbdif_dn8, locals.var_qbdif_dn9,)
    }
};
        locals.var_qbdif = assign43330_e48586;
        locals.var_qbdif_dn4 = assign43330_e48586_d_n4;
        locals.var_qbdif_dn6 = assign43330_e48586_d_n6;
        locals.var_qbdif_dn7 = assign43330_e48586_d_n7;
        locals.var_qbdif_dn8 = assign43330_e48586_d_n8;
        locals.var_qbdif_dn9 = assign43330_e48586_d_n9;

        let (assign43340_e48590, assign43340_e48590_d_n4, assign43340_e48590_d_n6, assign43340_e48590_d_n7, assign43340_e48590_d_n8, assign43340_e48590_d_n9,) = {
    if (locals.var_guard1250 != 0.0) {
        (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9,)
    } else {
        (locals.var_qbsif, locals.var_qbsif_dn4, locals.var_qbsif_dn6, locals.var_qbsif_dn7, locals.var_qbsif_dn8, locals.var_qbsif_dn9,)
    }
};
        locals.var_qbsif = assign43340_e48590;
        locals.var_qbsif_dn4 = assign43340_e48590_d_n4;
        locals.var_qbsif_dn6 = assign43340_e48590_d_n6;
        locals.var_qbsif_dn7 = assign43340_e48590_d_n7;
        locals.var_qbsif_dn8 = assign43340_e48590_d_n8;
        locals.var_qbsif_dn9 = assign43340_e48590_d_n9;

        let assign43350_e48593: f64 = if locals.var_mult_i_int > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1251 = assign43350_e48593;

        let (assign43360_e48617, assign43360_e48617_d_n4, assign43360_e48617_d_n6, assign43360_e48617_d_n7, assign43360_e48617_d_n8, assign43360_e48617_d_n9,) = {
    if (locals.var_guard1251 != 0.0) {
        let assign43360_e48597: f64 = (1e-9 * locals.var_betneff);
        let assign43360_e48600: f64 = (locals.var_gvsat * locals.var_areaq_i);
        let assign43360_e48601: f64 = (assign43360_e48597 / assign43360_e48600);
        let assign43360_e48605: f64 = (locals.var_qg_wo_mult + locals.var_qb_wo_mult);
        let assign43360_e48606: f64 = (locals.var_kdrift_i * assign43360_e48605);
        let assign43360_e48609: f64 = (locals.var_areaq_i * locals.var_cox1init);
        let assign43360_e48610: f64 = (assign43360_e48606 / assign43360_e48609);
        let assign43360_e48613: f64 = (locals.var_kdiff_i * locals.var_phit);
        let assign43360_e48614: f64 = (assign43360_e48610 + assign43360_e48613);
        let assign43360_e48615: f64 = (assign43360_e48601 * assign43360_e48614);
        (assign43360_e48615, ((((((1e-9 * locals.var_betneff_dn4) * assign43360_e48600) - (assign43360_e48597 * (locals.var_gvsat_dn4 * locals.var_areaq_i))) / (assign43360_e48600 * assign43360_e48600)) * assign43360_e48614) + (assign43360_e48601 * ((((locals.var_kdrift_i_dn4 * assign43360_e48605) + (locals.var_kdrift_i * (locals.var_qg_wo_mult_dn4 + locals.var_qb_wo_mult_dn4))) / assign43360_e48609) + ((locals.var_kdiff_i_dn4 * locals.var_phit) + (locals.var_kdiff_i * locals.var_phit_dn4))))), ((((((1e-9 * locals.var_betneff_dn6) * assign43360_e48600) - (assign43360_e48597 * (locals.var_gvsat_dn6 * locals.var_areaq_i))) / (assign43360_e48600 * assign43360_e48600)) * assign43360_e48614) + (assign43360_e48601 * ((((locals.var_kdrift_i_dn6 * assign43360_e48605) + (locals.var_kdrift_i * (locals.var_qg_wo_mult_dn6 + locals.var_qb_wo_mult_dn6))) / assign43360_e48609) + ((locals.var_kdiff_i_dn6 * locals.var_phit) + (locals.var_kdiff_i * locals.var_phit_dn6))))), ((((((1e-9 * locals.var_betneff_dn7) * assign43360_e48600) - (assign43360_e48597 * (locals.var_gvsat_dn7 * locals.var_areaq_i))) / (assign43360_e48600 * assign43360_e48600)) * assign43360_e48614) + (assign43360_e48601 * ((((locals.var_kdrift_i_dn7 * assign43360_e48605) + (locals.var_kdrift_i * (locals.var_qg_wo_mult_dn7 + locals.var_qb_wo_mult_dn7))) / assign43360_e48609) + ((locals.var_kdiff_i_dn7 * locals.var_phit) + (locals.var_kdiff_i * locals.var_phit_dn7))))), ((((((1e-9 * locals.var_betneff_dn8) * assign43360_e48600) - (assign43360_e48597 * (locals.var_gvsat_dn8 * locals.var_areaq_i))) / (assign43360_e48600 * assign43360_e48600)) * assign43360_e48614) + (assign43360_e48601 * ((((locals.var_kdrift_i_dn8 * assign43360_e48605) + (locals.var_kdrift_i * (locals.var_qg_wo_mult_dn8 + locals.var_qb_wo_mult_dn8))) / assign43360_e48609) + ((locals.var_kdiff_i_dn8 * locals.var_phit) + (locals.var_kdiff_i * locals.var_phit_dn8))))), ((((((1e-9 * locals.var_betneff_dn9) * assign43360_e48600) - (assign43360_e48597 * (locals.var_gvsat_dn9 * locals.var_areaq_i))) / (assign43360_e48600 * assign43360_e48600)) * assign43360_e48614) + (assign43360_e48601 * ((((locals.var_kdrift_i_dn9 * assign43360_e48605) + (locals.var_kdrift_i * (locals.var_qg_wo_mult_dn9 + locals.var_qb_wo_mult_dn9))) / assign43360_e48609) + ((locals.var_kdiff_i_dn9 * locals.var_phit) + (locals.var_kdiff_i * locals.var_phit_dn9))))),)
    } else {
        (locals.var_itaueff, locals.var_itaueff_dn4, locals.var_itaueff_dn6, locals.var_itaueff_dn7, locals.var_itaueff_dn8, locals.var_itaueff_dn9,)
    }
};
        locals.var_itaueff = assign43360_e48617;
        locals.var_itaueff_dn4 = assign43360_e48617_d_n4;
        locals.var_itaueff_dn6 = assign43360_e48617_d_n6;
        locals.var_itaueff_dn7 = assign43360_e48617_d_n7;
        locals.var_itaueff_dn8 = assign43360_e48617_d_n8;
        locals.var_itaueff_dn9 = assign43360_e48617_d_n9;

        let (assign43370_e48622, assign43370_e48622_d_n4, assign43370_e48622_d_n6, assign43370_e48622_d_n7, assign43370_e48622_d_n8, assign43370_e48622_d_n9,) = {
    if (locals.var_guard1251 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_itaueff, locals.var_itaueff_dn4, locals.var_itaueff_dn6, locals.var_itaueff_dn7, locals.var_itaueff_dn8, locals.var_itaueff_dn9,)
    }
};
        locals.var_itaueff = assign43370_e48622;
        locals.var_itaueff_dn4 = assign43370_e48622_d_n4;
        locals.var_itaueff_dn6 = assign43370_e48622_d_n6;
        locals.var_itaueff_dn7 = assign43370_e48622_d_n7;
        locals.var_itaueff_dn8 = assign43370_e48622_d_n8;
        locals.var_itaueff_dn9 = assign43370_e48622_d_n9;

        let assign43380_e48625: f64 = (locals.var_csiprime_dc / 1.602176565e-19);
        let assign43380_e48627: f64 = (assign43380_e48625 * locals.var_phit);
        locals.var_nunit = assign43380_e48627;
        locals.var_nunit_dn4 = (((locals.var_csiprime_dc_dn4 / 1.602176565e-19) * locals.var_phit) + (assign43380_e48625 * locals.var_phit_dn4));
        locals.var_nunit_dn6 = (((locals.var_csiprime_dc_dn6 / 1.602176565e-19) * locals.var_phit) + (assign43380_e48625 * locals.var_phit_dn6));
        locals.var_nunit_dn7 = (((locals.var_csiprime_dc_dn7 / 1.602176565e-19) * locals.var_phit) + (assign43380_e48625 * locals.var_phit_dn7));
        locals.var_nunit_dn8 = (((locals.var_csiprime_dc_dn8 / 1.602176565e-19) * locals.var_phit) + (assign43380_e48625 * locals.var_phit_dn8));
        locals.var_nunit_dn9 = (((locals.var_csiprime_dc_dn9 / 1.602176565e-19) * locals.var_phit) + (assign43380_e48625 * locals.var_phit_dn9));

        let assign43390_e48629: f64 = (-0.5);
        let assign43390_e48632: f64 = (locals.var_ds_dc + locals.var_dd_dc);
        let assign43390_e48633: f64 = (assign43390_e48629 * assign43390_e48632);
        locals.var_dm = assign43390_e48633;
        locals.var_dm_dn4 = (assign43390_e48629 * (locals.var_ds_dc_dn4 + locals.var_dd_dc_dn4));
        locals.var_dm_dn6 = (assign43390_e48629 * (locals.var_ds_dc_dn6 + locals.var_dd_dc_dn6));
        locals.var_dm_dn7 = (assign43390_e48629 * (locals.var_ds_dc_dn7 + locals.var_dd_dc_dn7));
        locals.var_dm_dn8 = (assign43390_e48629 * (locals.var_ds_dc_dn8 + locals.var_dd_dc_dn8));
        locals.var_dm_dn9 = (assign43390_e48629 * (locals.var_ds_dc_dn9 + locals.var_dd_dc_dn9));

        let assign43400_e48636: f64 = (locals.var_qim_dc + locals.var_dm);
        locals.var_qimstar = assign43400_e48636;
        locals.var_qimstar_dn4 = (locals.var_qim_dc_dn4 + locals.var_dm_dn4);
        locals.var_qimstar_dn6 = (locals.var_qim_dc_dn6 + locals.var_dm_dn6);
        locals.var_qimstar_dn7 = (locals.var_qim_dc_dn7 + locals.var_dm_dn7);
        locals.var_qimstar_dn8 = (locals.var_qim_dc_dn8 + locals.var_dm_dn8);
        locals.var_qimstar_dn9 = (locals.var_qim_dc_dn9 + locals.var_dm_dn9);

        let assign43410_e48639: f64 = (locals.var_qim_dc / locals.var_qimstar);
        locals.var_temp = assign43410_e48639;
        locals.var_temp_dn4 = (((locals.var_qim_dc_dn4 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn4)) / (locals.var_qimstar * locals.var_qimstar));
        locals.var_temp_dn6 = (((locals.var_qim_dc_dn6 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn6)) / (locals.var_qimstar * locals.var_qimstar));
        locals.var_temp_dn7 = (((locals.var_qim_dc_dn7 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn7)) / (locals.var_qimstar * locals.var_qimstar));
        locals.var_temp_dn8 = (((locals.var_qim_dc_dn8 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn8)) / (locals.var_qimstar * locals.var_qimstar));
        locals.var_temp_dn9 = (((locals.var_qim_dc_dn9 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn9)) / (locals.var_qimstar * locals.var_qimstar));

        let assign43420_e48643: f64 = locals.var_temp;
        let assign43420_e48646: f64 = locals.var_temp;
        let assign43420_e48649: f64 = locals.var_temp;
        let assign43420_e48650: f64 = (assign43420_e48646 * assign43420_e48649);
        let assign43420_e48652: f64 = (assign43420_e48650 + 1e-20);
        let assign43420_e48653: f64 = (assign43420_e48652).sqrt();
        let assign43420_e48654: f64 = (assign43420_e48643 + assign43420_e48653);
        let assign43420_e48655: f64 = (0.5 * assign43420_e48654);
        locals.var_t1 = assign43420_e48655;
        locals.var_t1_dn4 = (0.5 * (locals.var_temp_dn4 + (((locals.var_temp_dn4 * assign43420_e48649) + (assign43420_e48646 * locals.var_temp_dn4)) / (2.0 * assign43420_e48653))));
        locals.var_t1_dn6 = (0.5 * (locals.var_temp_dn6 + (((locals.var_temp_dn6 * assign43420_e48649) + (assign43420_e48646 * locals.var_temp_dn6)) / (2.0 * assign43420_e48653))));
        locals.var_t1_dn7 = (0.5 * (locals.var_temp_dn7 + (((locals.var_temp_dn7 * assign43420_e48649) + (assign43420_e48646 * locals.var_temp_dn7)) / (2.0 * assign43420_e48653))));
        locals.var_t1_dn8 = (0.5 * (locals.var_temp_dn8 + (((locals.var_temp_dn8 * assign43420_e48649) + (assign43420_e48646 * locals.var_temp_dn8)) / (2.0 * assign43420_e48653))));
        locals.var_t1_dn9 = (0.5 * (locals.var_temp_dn9 + (((locals.var_temp_dn9 * assign43420_e48649) + (assign43420_e48646 * locals.var_temp_dn9)) / (2.0 * assign43420_e48653))));

        let assign43430_e48657: f64 = (-0.1666666666667);
        let assign43430_e48659: f64 = (assign43430_e48657 * locals.var_delta_k1q1_dc);
        let assign43430_e48661: f64 = (assign43430_e48659 * locals.var_inv_k1h1_0_dc);
        locals.var_sqrt_t2 = assign43430_e48661;
        locals.var_sqrt_t2_dn4 = (((assign43430_e48657 * locals.var_delta_k1q1_dc_dn4) * locals.var_inv_k1h1_0_dc) + (assign43430_e48659 * locals.var_inv_k1h1_0_dc_dn4));
        locals.var_sqrt_t2_dn6 = (((assign43430_e48657 * locals.var_delta_k1q1_dc_dn6) * locals.var_inv_k1h1_0_dc) + (assign43430_e48659 * locals.var_inv_k1h1_0_dc_dn6));
        locals.var_sqrt_t2_dn7 = (((assign43430_e48657 * locals.var_delta_k1q1_dc_dn7) * locals.var_inv_k1h1_0_dc) + (assign43430_e48659 * locals.var_inv_k1h1_0_dc_dn7));
        locals.var_sqrt_t2_dn8 = (((assign43430_e48657 * locals.var_delta_k1q1_dc_dn8) * locals.var_inv_k1h1_0_dc) + (assign43430_e48659 * locals.var_inv_k1h1_0_dc_dn8));
        locals.var_sqrt_t2_dn9 = (((assign43430_e48657 * locals.var_delta_k1q1_dc_dn9) * locals.var_inv_k1h1_0_dc) + (assign43430_e48659 * locals.var_inv_k1h1_0_dc_dn9));

        let assign43440_e48664: f64 = (locals.var_sqrt_t2 * locals.var_sqrt_t2);
        locals.var_t2 = assign43440_e48664;
        locals.var_t2_dn4 = ((locals.var_sqrt_t2_dn4 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn4));
        locals.var_t2_dn6 = ((locals.var_sqrt_t2_dn6 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn6));
        locals.var_t2_dn7 = ((locals.var_sqrt_t2_dn7 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn7));
        locals.var_t2_dn8 = ((locals.var_sqrt_t2_dn8 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn8));
        locals.var_t2_dn9 = ((locals.var_sqrt_t2_dn9 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn9));

        let assign43450_e48667: f64 = (locals.var_hsat_dc - 1.0);
        locals.var_r = assign43450_e48667;
        locals.var_r_dn4 = locals.var_hsat_dc_dn4;
        locals.var_r_dn6 = locals.var_hsat_dc_dn6;
        locals.var_r_dn7 = locals.var_hsat_dc_dn7;
        locals.var_r_dn8 = locals.var_hsat_dc_dn8;
        locals.var_r_dn9 = locals.var_hsat_dc_dn9;

        let assign43460_e48671: f64 = (12.0 * locals.var_r);
        let assign43460_e48673: f64 = (assign43460_e48671 * locals.var_t2);
        let assign43460_e48674: f64 = (1.0 - assign43460_e48673);
        let assign43460_e48676: f64 = (assign43460_e48674).max(1e-20);
        locals.var_lc = assign43460_e48676;
        locals.var_lc_dn4 = if assign43460_e48674 >= 1e-20 { (-(((12.0 * locals.var_r_dn4) * locals.var_t2) + (assign43460_e48671 * locals.var_t2_dn4))) } else { 0.0 };
        locals.var_lc_dn6 = if assign43460_e48674 >= 1e-20 { (-(((12.0 * locals.var_r_dn6) * locals.var_t2) + (assign43460_e48671 * locals.var_t2_dn6))) } else { 0.0 };
        locals.var_lc_dn7 = if assign43460_e48674 >= 1e-20 { (-(((12.0 * locals.var_r_dn7) * locals.var_t2) + (assign43460_e48671 * locals.var_t2_dn7))) } else { 0.0 };
        locals.var_lc_dn8 = if assign43460_e48674 >= 1e-20 { (-(((12.0 * locals.var_r_dn8) * locals.var_t2) + (assign43460_e48671 * locals.var_t2_dn8))) } else { 0.0 };
        locals.var_lc_dn9 = if assign43460_e48674 >= 1e-20 { (-(((12.0 * locals.var_r_dn9) * locals.var_t2) + (assign43460_e48671 * locals.var_t2_dn9))) } else { 0.0 };

        let assign43470_e48680: f64 = (locals.var_lc * locals.var_lc);
        let assign43470_e48681: f64 = (1.0 / assign43470_e48680);
        locals.var_lcinv2 = assign43470_e48681;
        locals.var_lcinv2_dn4 = (-(((locals.var_lc_dn4 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn4)) / (assign43470_e48680 * assign43470_e48680)));
        locals.var_lcinv2_dn6 = (-(((locals.var_lc_dn6 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn6)) / (assign43470_e48680 * assign43470_e48680)));
        locals.var_lcinv2_dn7 = (-(((locals.var_lc_dn7 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn7)) / (assign43470_e48680 * assign43470_e48680)));
        locals.var_lcinv2_dn8 = (-(((locals.var_lc_dn8 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn8)) / (assign43470_e48680 * assign43470_e48680)));
        locals.var_lcinv2_dn9 = (-(((locals.var_lc_dn9 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn9)) / (assign43470_e48680 * assign43470_e48680)));

        let assign43480_e48684: f64 = (locals.var_betneff * locals.var_csiprime_dc);
        let assign43480_e48686: f64 = (assign43480_e48684 * locals.var_phit);
        let assign43480_e48688: f64 = (assign43480_e48686 * locals.var_qimstar);
        let assign43480_e48690: f64 = (assign43480_e48688 * locals.var_fdl);
        let assign43480_e48692: f64 = (assign43480_e48690 / locals.var_gvsat);
        let assign43480_e48694: f64 = (assign43480_e48692 / locals.var_qmfact);
        locals.var_g_ideal = assign43480_e48694;
        locals.var_g_ideal_dn4 = ((((((((((((((locals.var_betneff_dn4 * locals.var_csiprime_dc) + (locals.var_betneff * locals.var_csiprime_dc_dn4)) * locals.var_phit) + (assign43480_e48684 * locals.var_phit_dn4)) * locals.var_qimstar) + (assign43480_e48686 * locals.var_qimstar_dn4)) * locals.var_fdl) + (assign43480_e48688 * locals.var_fdl_dn4)) * locals.var_gvsat) - (assign43480_e48690 * locals.var_gvsat_dn4)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign43480_e48692 * locals.var_qmfact_dn4)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_g_ideal_dn6 = ((((((((((((((locals.var_betneff_dn6 * locals.var_csiprime_dc) + (locals.var_betneff * locals.var_csiprime_dc_dn6)) * locals.var_phit) + (assign43480_e48684 * locals.var_phit_dn6)) * locals.var_qimstar) + (assign43480_e48686 * locals.var_qimstar_dn6)) * locals.var_fdl) + (assign43480_e48688 * locals.var_fdl_dn6)) * locals.var_gvsat) - (assign43480_e48690 * locals.var_gvsat_dn6)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign43480_e48692 * locals.var_qmfact_dn6)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_g_ideal_dn7 = ((((((((((((((locals.var_betneff_dn7 * locals.var_csiprime_dc) + (locals.var_betneff * locals.var_csiprime_dc_dn7)) * locals.var_phit) + (assign43480_e48684 * locals.var_phit_dn7)) * locals.var_qimstar) + (assign43480_e48686 * locals.var_qimstar_dn7)) * locals.var_fdl) + (assign43480_e48688 * locals.var_fdl_dn7)) * locals.var_gvsat) - (assign43480_e48690 * locals.var_gvsat_dn7)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign43480_e48692 * locals.var_qmfact_dn7)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_g_ideal_dn8 = ((((((((((((((locals.var_betneff_dn8 * locals.var_csiprime_dc) + (locals.var_betneff * locals.var_csiprime_dc_dn8)) * locals.var_phit) + (assign43480_e48684 * locals.var_phit_dn8)) * locals.var_qimstar) + (assign43480_e48686 * locals.var_qimstar_dn8)) * locals.var_fdl) + (assign43480_e48688 * locals.var_fdl_dn8)) * locals.var_gvsat) - (assign43480_e48690 * locals.var_gvsat_dn8)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign43480_e48692 * locals.var_qmfact_dn8)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_g_ideal_dn9 = ((((((((((((((locals.var_betneff_dn9 * locals.var_csiprime_dc) + (locals.var_betneff * locals.var_csiprime_dc_dn9)) * locals.var_phit) + (assign43480_e48684 * locals.var_phit_dn9)) * locals.var_qimstar) + (assign43480_e48686 * locals.var_qimstar_dn9)) * locals.var_fdl) + (assign43480_e48688 * locals.var_fdl_dn9)) * locals.var_gvsat) - (assign43480_e48690 * locals.var_gvsat_dn9)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign43480_e48692 * locals.var_qmfact_dn9)) / (locals.var_qmfact * locals.var_qmfact));

        let assign43490_e48697: f64 = (12.0 * locals.var_t2);
        locals.var_t2x12 = assign43490_e48697;
        locals.var_t2x12_dn4 = (12.0 * locals.var_t2_dn4);
        locals.var_t2x12_dn6 = (12.0 * locals.var_t2_dn6);
        locals.var_t2x12_dn7 = (12.0 * locals.var_t2_dn7);
        locals.var_t2x12_dn8 = (12.0 * locals.var_t2_dn8);
        locals.var_t2x12_dn9 = (12.0 * locals.var_t2_dn9);

        let assign43500_e48700: f64 = (locals.var_t1 + locals.var_t2x12);
        let assign43500_e48704: f64 = (1.0 + locals.var_t1);
        let assign43500_e48705: f64 = (2.0 * assign43500_e48704);
        let assign43500_e48707: f64 = (assign43500_e48705 * locals.var_t2x12);
        let assign43500_e48709: f64 = (assign43500_e48707 * locals.var_r);
        let assign43500_e48710: f64 = (assign43500_e48700 - assign43500_e48709);
        locals.var_temp1 = assign43500_e48710;
        locals.var_temp1_dn4 = ((locals.var_t1_dn4 + locals.var_t2x12_dn4) - (((((2.0 * locals.var_t1_dn4) * locals.var_t2x12) + (assign43500_e48705 * locals.var_t2x12_dn4)) * locals.var_r) + (assign43500_e48707 * locals.var_r_dn4)));
        locals.var_temp1_dn6 = ((locals.var_t1_dn6 + locals.var_t2x12_dn6) - (((((2.0 * locals.var_t1_dn6) * locals.var_t2x12) + (assign43500_e48705 * locals.var_t2x12_dn6)) * locals.var_r) + (assign43500_e48707 * locals.var_r_dn6)));
        locals.var_temp1_dn7 = ((locals.var_t1_dn7 + locals.var_t2x12_dn7) - (((((2.0 * locals.var_t1_dn7) * locals.var_t2x12) + (assign43500_e48705 * locals.var_t2x12_dn7)) * locals.var_r) + (assign43500_e48707 * locals.var_r_dn7)));
        locals.var_temp1_dn8 = ((locals.var_t1_dn8 + locals.var_t2x12_dn8) - (((((2.0 * locals.var_t1_dn8) * locals.var_t2x12) + (assign43500_e48705 * locals.var_t2x12_dn8)) * locals.var_r) + (assign43500_e48707 * locals.var_r_dn8)));
        locals.var_temp1_dn9 = ((locals.var_t1_dn9 + locals.var_t2x12_dn9) - (((((2.0 * locals.var_t1_dn9) * locals.var_t2x12) + (assign43500_e48705 * locals.var_t2x12_dn9)) * locals.var_r) + (assign43500_e48707 * locals.var_r_dn9)));

        let assign43510_e48713: f64 = (locals.var_temp1).max(1e-40);
        locals.var_temp2 = assign43510_e48713;
        locals.var_temp2_dn4 = if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn4 } else { 0.0 };
        locals.var_temp2_dn6 = if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn6 } else { 0.0 };
        locals.var_temp2_dn7 = if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn7 } else { 0.0 };
        locals.var_temp2_dn8 = if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn8 } else { 0.0 };
        locals.var_temp2_dn9 = if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn9 } else { 0.0 };

        let assign43520_e48716: f64 = (locals.var_g_ideal * locals.var_lcinv2);
        let assign43520_e48718: f64 = (assign43520_e48716 * locals.var_temp2);
        locals.var_gsid = assign43520_e48718;

        let assign43530_e48721: f64 = if locals.var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1284 = assign43530_e48721;

        let (assign43540_e48727,) = {
    if (locals.var_guard1284 != 0.0) {
        let assign43540_e48725: f64 = (locals.var_ggamma_dc / locals.var_gmob_dc);
        (assign43540_e48725,)
    } else {
        (locals.var_sqrt_zsatexc,)
    }
};
        locals.var_sqrt_zsatexc = assign43540_e48727;

        let (assign43550_e48747,) = {
    if (locals.var_guard1284 != 0.0) {
        let assign43550_e48731: f64 = (locals.var_fac_exc * locals.var_ids);
        let assign43550_e48733: f64 = (assign43550_e48731 * locals.var_xdeff_dc);
        let assign43550_e48735: f64 = (assign43550_e48733 * locals.var_phit0);
        let assign43550_e48739: f64 = (locals.var_sqrt_zsatexc * locals.var_sqrt_zsatexc);
        let assign43550_e48740: f64 = (1.0 + assign43550_e48739);
        let assign43550_e48742: f64 = (assign43550_e48740 * locals.var_lc);
        let assign43550_e48744: f64 = (assign43550_e48742 * locals.var_lc);
        let assign43550_e48745: f64 = (assign43550_e48735 / assign43550_e48744);
        (assign43550_e48745,)
    } else {
        (locals.var_sidexc,)
    }
};
        locals.var_sidexc = assign43550_e48747;

        let (assign43560_e48755,) = {
    if (locals.var_guard1284 != 0.0) {
        let assign43560_e48752: f64 = (locals.var_sidexc / locals.var_nt0);
        let assign43560_e48753: f64 = (locals.var_gsid + assign43560_e48752);
        (assign43560_e48753,)
    } else {
        (locals.var_gsid,)
    }
};
        locals.var_gsid = assign43560_e48755;

        let assign43580_e48765: f64 = (locals.var_k1_ac * locals.var_csiprime_ac);
        let assign43580_e48767: f64 = (assign43580_e48765 * locals.var_areaq_i);
        let assign43580_e48769: f64 = (assign43580_e48767 / locals.var_qmfact1_ac);
        locals.var_cox_qm = assign43580_e48769;
        locals.var_cox_qm_dn4 = ((((((locals.var_k1_ac_dn4 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn4)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43580_e48767 * locals.var_qmfact1_ac_dn4)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac));
        locals.var_cox_qm_dn6 = ((((((locals.var_k1_ac_dn6 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn6)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43580_e48767 * locals.var_qmfact1_ac_dn6)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac));
        locals.var_cox_qm_dn7 = ((((((locals.var_k1_ac_dn7 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn7)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43580_e48767 * locals.var_qmfact1_ac_dn7)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac));
        locals.var_cox_qm_dn8 = ((((((locals.var_k1_ac_dn8 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn8)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43580_e48767 * locals.var_qmfact1_ac_dn8)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac));
        locals.var_cox_qm_dn9 = ((((((locals.var_k1_ac_dn9 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn9)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43580_e48767 * locals.var_qmfact1_ac_dn9)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac));

        let assign43590_e48772: f64 = (1.0 + locals.var_zsat_ac);
        let assign43590_e48774: f64 = (assign43590_e48772 * locals.var_cox_qm);
        locals.var_cgeff = assign43590_e48774;
        locals.var_cgeff_dn4 = ((locals.var_zsat_ac_dn4 * locals.var_cox_qm) + (assign43590_e48772 * locals.var_cox_qm_dn4));
        locals.var_cgeff_dn6 = ((locals.var_zsat_ac_dn6 * locals.var_cox_qm) + (assign43590_e48772 * locals.var_cox_qm_dn6));
        locals.var_cgeff_dn7 = ((locals.var_zsat_ac_dn7 * locals.var_cox_qm) + (assign43590_e48772 * locals.var_cox_qm_dn7));
        locals.var_cgeff_dn8 = ((locals.var_zsat_ac_dn8 * locals.var_cox_qm) + (assign43590_e48772 * locals.var_cox_qm_dn8));
        locals.var_cgeff_dn9 = ((locals.var_zsat_ac_dn9 * locals.var_cox_qm) + (assign43590_e48772 * locals.var_cox_qm_dn9));

        let assign43600_e48779: f64 = (0.25 * locals.var_sigvds);
        let assign43600_e48781: f64 = (assign43600_e48779 * locals.var_sqrt_t2);
        let assign43600_e48782: f64 = (0.5 - assign43600_e48781);
        let assign43600_e48783: f64 = (locals.var_cgeff * assign43600_e48782);
        locals.var_cdgeff = assign43600_e48783;
        locals.var_cdgeff_dn4 = ((locals.var_cgeff_dn4 * assign43600_e48782) + (locals.var_cgeff * (-(assign43600_e48779 * locals.var_sqrt_t2_dn4))));
        locals.var_cdgeff_dn6 = ((locals.var_cgeff_dn6 * assign43600_e48782) + (locals.var_cgeff * (-(assign43600_e48779 * locals.var_sqrt_t2_dn6))));
        locals.var_cdgeff_dn7 = ((locals.var_cgeff_dn7 * assign43600_e48782) + (locals.var_cgeff * (-(assign43600_e48779 * locals.var_sqrt_t2_dn7))));
        locals.var_cdgeff_dn8 = ((locals.var_cgeff_dn8 * assign43600_e48782) + (locals.var_cgeff * (-(assign43600_e48779 * locals.var_sqrt_t2_dn8))));
        locals.var_cdgeff_dn9 = ((locals.var_cgeff_dn9 * assign43600_e48782) + (locals.var_cgeff * (-(assign43600_e48779 * locals.var_sqrt_t2_dn9))));

        let assign43610_e48786: f64 = (locals.var_cgeff - locals.var_cdgeff);
        locals.var_csgeff = assign43610_e48786;
        locals.var_csgeff_dn4 = (locals.var_cgeff_dn4 - locals.var_cdgeff_dn4);
        locals.var_csgeff_dn6 = (locals.var_cgeff_dn6 - locals.var_cdgeff_dn6);
        locals.var_csgeff_dn7 = (locals.var_cgeff_dn7 - locals.var_cdgeff_dn7);
        locals.var_csgeff_dn8 = (locals.var_cgeff_dn8 - locals.var_cdgeff_dn8);
        locals.var_csgeff_dn9 = (locals.var_cgeff_dn9 - locals.var_cdgeff_dn9);

        locals.var_migid = 0.0;
        locals.var_migid_dn4 = 0.0;
        locals.var_migid_dn6 = 0.0;
        locals.var_migid_dn7 = 0.0;
        locals.var_migid_dn8 = 0.0;
        locals.var_migid_dn9 = 0.0;

        let assign43640_e48791: f64 = if p.p6 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1285 = assign43640_e48791;

        let (assign43650_e48817, assign43650_e48817_d_n4, assign43650_e48817_d_n6, assign43650_e48817_d_n7, assign43650_e48817_d_n8, assign43650_e48817_d_n9,) = {
    if (locals.var_guard1285 != 0.0) {
        let assign43650_e48795: f64 = (locals.var_t1 / 12.0);
        let assign43650_e48799: f64 = (locals.var_t1 + 0.2);
        let assign43650_e48801: f64 = (assign43650_e48799 - locals.var_t2x12);
        let assign43650_e48802: f64 = (locals.var_t2 * assign43650_e48801);
        let assign43650_e48803: f64 = (assign43650_e48795 - assign43650_e48802);
        let assign43650_e48806: f64 = (1.6 * locals.var_t2);
        let assign43650_e48809: f64 = (locals.var_t1 + 1.0);
        let assign43650_e48811: f64 = (assign43650_e48809 - locals.var_t2x12);
        let assign43650_e48812: f64 = (assign43650_e48806 * assign43650_e48811);
        let assign43650_e48814: f64 = (assign43650_e48812 * locals.var_r);
        let assign43650_e48815: f64 = (assign43650_e48803 - assign43650_e48814);
        (assign43650_e48815, (((locals.var_t1_dn4 / 12.0) - ((locals.var_t2_dn4 * assign43650_e48801) + (locals.var_t2 * (locals.var_t1_dn4 - locals.var_t2x12_dn4)))) - (((((1.6 * locals.var_t2_dn4) * assign43650_e48811) + (assign43650_e48806 * (locals.var_t1_dn4 - locals.var_t2x12_dn4))) * locals.var_r) + (assign43650_e48812 * locals.var_r_dn4))), (((locals.var_t1_dn6 / 12.0) - ((locals.var_t2_dn6 * assign43650_e48801) + (locals.var_t2 * (locals.var_t1_dn6 - locals.var_t2x12_dn6)))) - (((((1.6 * locals.var_t2_dn6) * assign43650_e48811) + (assign43650_e48806 * (locals.var_t1_dn6 - locals.var_t2x12_dn6))) * locals.var_r) + (assign43650_e48812 * locals.var_r_dn6))), (((locals.var_t1_dn7 / 12.0) - ((locals.var_t2_dn7 * assign43650_e48801) + (locals.var_t2 * (locals.var_t1_dn7 - locals.var_t2x12_dn7)))) - (((((1.6 * locals.var_t2_dn7) * assign43650_e48811) + (assign43650_e48806 * (locals.var_t1_dn7 - locals.var_t2x12_dn7))) * locals.var_r) + (assign43650_e48812 * locals.var_r_dn7))), (((locals.var_t1_dn8 / 12.0) - ((locals.var_t2_dn8 * assign43650_e48801) + (locals.var_t2 * (locals.var_t1_dn8 - locals.var_t2x12_dn8)))) - (((((1.6 * locals.var_t2_dn8) * assign43650_e48811) + (assign43650_e48806 * (locals.var_t1_dn8 - locals.var_t2x12_dn8))) * locals.var_r) + (assign43650_e48812 * locals.var_r_dn8))), (((locals.var_t1_dn9 / 12.0) - ((locals.var_t2_dn9 * assign43650_e48801) + (locals.var_t2 * (locals.var_t1_dn9 - locals.var_t2x12_dn9)))) - (((((1.6 * locals.var_t2_dn9) * assign43650_e48811) + (assign43650_e48806 * (locals.var_t1_dn9 - locals.var_t2x12_dn9))) * locals.var_r) + (assign43650_e48812 * locals.var_r_dn9))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign43650_e48817;
        locals.var_temp1_dn4 = assign43650_e48817_d_n4;
        locals.var_temp1_dn6 = assign43650_e48817_d_n6;
        locals.var_temp1_dn7 = assign43650_e48817_d_n7;
        locals.var_temp1_dn8 = assign43650_e48817_d_n8;
        locals.var_temp1_dn9 = assign43650_e48817_d_n9;

        let (assign43660_e48823, assign43660_e48823_d_n4, assign43660_e48823_d_n6, assign43660_e48823_d_n7, assign43660_e48823_d_n8, assign43660_e48823_d_n9,) = {
    if (locals.var_guard1285 != 0.0) {
        let assign43660_e48821: f64 = (locals.var_temp1).max(1e-40);
        (assign43660_e48821, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn4 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn6 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn7 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn8 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn9 } else { 0.0 },)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign43660_e48823;
        locals.var_temp2_dn4 = assign43660_e48823_d_n4;
        locals.var_temp2_dn6 = assign43660_e48823_d_n6;
        locals.var_temp2_dn7 = assign43660_e48823_d_n7;
        locals.var_temp2_dn8 = assign43660_e48823_d_n8;
        locals.var_temp2_dn9 = assign43660_e48823_d_n9;

        let (assign43670_e48833, assign43670_e48833_d_n4, assign43670_e48833_d_n6, assign43670_e48833_d_n7, assign43670_e48833_d_n8, assign43670_e48833_d_n9,) = {
    if (locals.var_guard1285 != 0.0) {
        let assign43670_e48827: f64 = (locals.var_g_ideal * locals.var_lc);
        let assign43670_e48829: f64 = (assign43670_e48827 * locals.var_lc);
        let assign43670_e48831: f64 = (assign43670_e48829 / locals.var_temp2);
        (assign43670_e48831, (((((((locals.var_g_ideal_dn4 * locals.var_lc) + (locals.var_g_ideal * locals.var_lc_dn4)) * locals.var_lc) + (assign43670_e48827 * locals.var_lc_dn4)) * locals.var_temp2) - (assign43670_e48829 * locals.var_temp2_dn4)) / (locals.var_temp2 * locals.var_temp2)), (((((((locals.var_g_ideal_dn6 * locals.var_lc) + (locals.var_g_ideal * locals.var_lc_dn6)) * locals.var_lc) + (assign43670_e48827 * locals.var_lc_dn6)) * locals.var_temp2) - (assign43670_e48829 * locals.var_temp2_dn6)) / (locals.var_temp2 * locals.var_temp2)), (((((((locals.var_g_ideal_dn7 * locals.var_lc) + (locals.var_g_ideal * locals.var_lc_dn7)) * locals.var_lc) + (assign43670_e48827 * locals.var_lc_dn7)) * locals.var_temp2) - (assign43670_e48829 * locals.var_temp2_dn7)) / (locals.var_temp2 * locals.var_temp2)), (((((((locals.var_g_ideal_dn8 * locals.var_lc) + (locals.var_g_ideal * locals.var_lc_dn8)) * locals.var_lc) + (assign43670_e48827 * locals.var_lc_dn8)) * locals.var_temp2) - (assign43670_e48829 * locals.var_temp2_dn8)) / (locals.var_temp2 * locals.var_temp2)), (((((((locals.var_g_ideal_dn9 * locals.var_lc) + (locals.var_g_ideal * locals.var_lc_dn9)) * locals.var_lc) + (assign43670_e48827 * locals.var_lc_dn9)) * locals.var_temp2) - (assign43670_e48829 * locals.var_temp2_dn9)) / (locals.var_temp2 * locals.var_temp2)),)
    } else {
        (locals.var_gsig, locals.var_gsig_dn4, locals.var_gsig_dn6, locals.var_gsig_dn7, locals.var_gsig_dn8, locals.var_gsig_dn9,)
    }
};
        locals.var_gsig = assign43670_e48833;
        locals.var_gsig_dn4 = assign43670_e48833_d_n4;
        locals.var_gsig_dn6 = assign43670_e48833_d_n6;
        locals.var_gsig_dn7 = assign43670_e48833_d_n7;
        locals.var_gsig_dn8 = assign43670_e48833_d_n8;
        locals.var_gsig_dn9 = assign43670_e48833_d_n9;

        let assign43690_e48846: f64 = if locals.var_gsid > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1286 = assign43690_e48846;

        let (assign43700_e48870, assign43700_e48870_d_n4, assign43700_e48870_d_n6, assign43700_e48870_d_n7, assign43700_e48870_d_n8, assign43700_e48870_d_n9,) = {
    if ((locals.var_guard1285 != 0.0) && (locals.var_guard1286 != 0.0)) {
        let assign43700_e48852: f64 = (locals.var_lcinv2 * locals.var_sqrt_t2);
        let assign43700_e48855: f64 = (1.0 - locals.var_t2x12);
        let assign43700_e48859: f64 = (19.2 * locals.var_t2);
        let assign43700_e48860: f64 = (locals.var_t1 + assign43700_e48859);
        let assign43700_e48863: f64 = (locals.var_t1 * locals.var_t2x12);
        let assign43700_e48864: f64 = (assign43700_e48860 - assign43700_e48863);
        let assign43700_e48866: f64 = (assign43700_e48864 * locals.var_r);
        let assign43700_e48867: f64 = (assign43700_e48855 - assign43700_e48866);
        let assign43700_e48868: f64 = (assign43700_e48852 * assign43700_e48867);
        (assign43700_e48868, ((((locals.var_lcinv2_dn4 * locals.var_sqrt_t2) + (locals.var_lcinv2 * locals.var_sqrt_t2_dn4)) * assign43700_e48867) + (assign43700_e48852 * ((-locals.var_t2x12_dn4) - ((((locals.var_t1_dn4 + (19.2 * locals.var_t2_dn4)) - ((locals.var_t1_dn4 * locals.var_t2x12) + (locals.var_t1 * locals.var_t2x12_dn4))) * locals.var_r) + (assign43700_e48864 * locals.var_r_dn4))))), ((((locals.var_lcinv2_dn6 * locals.var_sqrt_t2) + (locals.var_lcinv2 * locals.var_sqrt_t2_dn6)) * assign43700_e48867) + (assign43700_e48852 * ((-locals.var_t2x12_dn6) - ((((locals.var_t1_dn6 + (19.2 * locals.var_t2_dn6)) - ((locals.var_t1_dn6 * locals.var_t2x12) + (locals.var_t1 * locals.var_t2x12_dn6))) * locals.var_r) + (assign43700_e48864 * locals.var_r_dn6))))), ((((locals.var_lcinv2_dn7 * locals.var_sqrt_t2) + (locals.var_lcinv2 * locals.var_sqrt_t2_dn7)) * assign43700_e48867) + (assign43700_e48852 * ((-locals.var_t2x12_dn7) - ((((locals.var_t1_dn7 + (19.2 * locals.var_t2_dn7)) - ((locals.var_t1_dn7 * locals.var_t2x12) + (locals.var_t1 * locals.var_t2x12_dn7))) * locals.var_r) + (assign43700_e48864 * locals.var_r_dn7))))), ((((locals.var_lcinv2_dn8 * locals.var_sqrt_t2) + (locals.var_lcinv2 * locals.var_sqrt_t2_dn8)) * assign43700_e48867) + (assign43700_e48852 * ((-locals.var_t2x12_dn8) - ((((locals.var_t1_dn8 + (19.2 * locals.var_t2_dn8)) - ((locals.var_t1_dn8 * locals.var_t2x12) + (locals.var_t1 * locals.var_t2x12_dn8))) * locals.var_r) + (assign43700_e48864 * locals.var_r_dn8))))), ((((locals.var_lcinv2_dn9 * locals.var_sqrt_t2) + (locals.var_lcinv2 * locals.var_sqrt_t2_dn9)) * assign43700_e48867) + (assign43700_e48852 * ((-locals.var_t2x12_dn9) - ((((locals.var_t1_dn9 + (19.2 * locals.var_t2_dn9)) - ((locals.var_t1_dn9 * locals.var_t2x12) + (locals.var_t1 * locals.var_t2x12_dn9))) * locals.var_r) + (assign43700_e48864 * locals.var_r_dn9))))),)
    } else {
        (locals.var_migid, locals.var_migid_dn4, locals.var_migid_dn6, locals.var_migid_dn7, locals.var_migid_dn8, locals.var_migid_dn9,)
    }
};
        locals.var_migid = assign43700_e48870;
        locals.var_migid_dn4 = assign43700_e48870_d_n4;
        locals.var_migid_dn6 = assign43700_e48870_d_n6;
        locals.var_migid_dn7 = assign43700_e48870_d_n7;
        locals.var_migid_dn8 = assign43700_e48870_d_n8;
        locals.var_migid_dn9 = assign43700_e48870_d_n9;

        let (assign43730_e48953, assign43730_e48953_d_n4, assign43730_e48953_d_n6, assign43730_e48953_d_n7, assign43730_e48953_d_n8, assign43730_e48953_d_n9,) = {
    if (locals.var_guard1285 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gsig, locals.var_gsig_dn4, locals.var_gsig_dn6, locals.var_gsig_dn7, locals.var_gsig_dn8, locals.var_gsig_dn9,)
    }
};
        locals.var_gsig = assign43730_e48953;
        locals.var_gsig_dn4 = assign43730_e48953_d_n4;
        locals.var_gsig_dn6 = assign43730_e48953_d_n6;
        locals.var_gsig_dn7 = assign43730_e48953_d_n7;
        locals.var_gsig_dn8 = assign43730_e48953_d_n8;
        locals.var_gsig_dn9 = assign43730_e48953_d_n9;

        locals.var_nstar = locals.var_nunit;
        locals.var_nstar_dn4 = locals.var_nunit_dn4;
        locals.var_nstar_dn6 = locals.var_nunit_dn6;
        locals.var_nstar_dn7 = locals.var_nunit_dn7;
        locals.var_nstar_dn8 = locals.var_nunit_dn8;
        locals.var_nstar_dn9 = locals.var_nunit_dn9;

        let assign43770_e48968: f64 = (locals.var_qim_dc + 1.0);
        let assign43770_e48969: f64 = (locals.var_nunit * assign43770_e48968);
        locals.var_nmstar = assign43770_e48969;
        locals.var_nmstar_dn4 = ((locals.var_nunit_dn4 * assign43770_e48968) + (locals.var_nunit * locals.var_qim_dc_dn4));
        locals.var_nmstar_dn6 = ((locals.var_nunit_dn6 * assign43770_e48968) + (locals.var_nunit * locals.var_qim_dc_dn6));
        locals.var_nmstar_dn7 = ((locals.var_nunit_dn7 * assign43770_e48968) + (locals.var_nunit * locals.var_qim_dc_dn7));
        locals.var_nmstar_dn8 = ((locals.var_nunit_dn8 * assign43770_e48968) + (locals.var_nunit * locals.var_qim_dc_dn8));
        locals.var_nmstar_dn9 = ((locals.var_nunit_dn9 * assign43770_e48968) + (locals.var_nunit * locals.var_qim_dc_dn9));

        let assign43780_e48973: f64 = (locals.var_qis_dc - locals.var_qid_dc);
        let assign43780_e48974: f64 = (locals.var_nunit * assign43780_e48973);
        locals.var_deltan = assign43780_e48974;
        locals.var_deltan_dn4 = ((locals.var_nunit_dn4 * assign43780_e48973) + (locals.var_nunit * (locals.var_qis_dc_dn4 - locals.var_qid_dc_dn4)));
        locals.var_deltan_dn6 = ((locals.var_nunit_dn6 * assign43780_e48973) + (locals.var_nunit * (locals.var_qis_dc_dn6 - locals.var_qid_dc_dn6)));
        locals.var_deltan_dn7 = ((locals.var_nunit_dn7 * assign43780_e48973) + (locals.var_nunit * (locals.var_qis_dc_dn7 - locals.var_qid_dc_dn7)));
        locals.var_deltan_dn8 = ((locals.var_nunit_dn8 * assign43780_e48973) + (locals.var_nunit * (locals.var_qis_dc_dn8 - locals.var_qid_dc_dn8)));
        locals.var_deltan_dn9 = ((locals.var_nunit_dn9 * assign43780_e48973) + (locals.var_nunit * (locals.var_qis_dc_dn9 - locals.var_qid_dc_dn9)));

        let assign43790_e48978: f64 = (locals.var_nfb_i * locals.var_nstar);
        let assign43790_e48979: f64 = (locals.var_nfa_i - assign43790_e48978);
        let assign43790_e48982: f64 = (locals.var_nfc_i * locals.var_nstar);
        let assign43790_e48984: f64 = (assign43790_e48982 * locals.var_nstar);
        let assign43790_e48985: f64 = (assign43790_e48979 + assign43790_e48984);
        let assign43790_e48989: f64 = (0.5 * locals.var_deltan);
        let assign43790_e48990: f64 = (locals.var_nmstar + assign43790_e48989);
        let assign43790_e48994: f64 = (0.5 * locals.var_deltan);
        let assign43790_e48995: f64 = (locals.var_nmstar - assign43790_e48994);
        let assign43790_e48996: f64 = (assign43790_e48990 / assign43790_e48995);
        let assign43790_e48997: f64 = (assign43790_e48996).ln();
        let assign43790_e48998: f64 = (assign43790_e48985 * assign43790_e48997);
        locals.var_temp1 = assign43790_e48998;
        locals.var_temp1_dn4 = ((((-(locals.var_nfb_i * locals.var_nstar_dn4)) + (((locals.var_nfc_i * locals.var_nstar_dn4) * locals.var_nstar) + (assign43790_e48982 * locals.var_nstar_dn4))) * assign43790_e48997) + (assign43790_e48985 * (((((locals.var_nmstar_dn4 + (0.5 * locals.var_deltan_dn4)) * assign43790_e48995) - (assign43790_e48990 * (locals.var_nmstar_dn4 - (0.5 * locals.var_deltan_dn4)))) / (assign43790_e48995 * assign43790_e48995)) / assign43790_e48996)));
        locals.var_temp1_dn6 = ((((-(locals.var_nfb_i * locals.var_nstar_dn6)) + (((locals.var_nfc_i * locals.var_nstar_dn6) * locals.var_nstar) + (assign43790_e48982 * locals.var_nstar_dn6))) * assign43790_e48997) + (assign43790_e48985 * (((((locals.var_nmstar_dn6 + (0.5 * locals.var_deltan_dn6)) * assign43790_e48995) - (assign43790_e48990 * (locals.var_nmstar_dn6 - (0.5 * locals.var_deltan_dn6)))) / (assign43790_e48995 * assign43790_e48995)) / assign43790_e48996)));
        locals.var_temp1_dn7 = ((((-(locals.var_nfb_i * locals.var_nstar_dn7)) + (((locals.var_nfc_i * locals.var_nstar_dn7) * locals.var_nstar) + (assign43790_e48982 * locals.var_nstar_dn7))) * assign43790_e48997) + (assign43790_e48985 * (((((locals.var_nmstar_dn7 + (0.5 * locals.var_deltan_dn7)) * assign43790_e48995) - (assign43790_e48990 * (locals.var_nmstar_dn7 - (0.5 * locals.var_deltan_dn7)))) / (assign43790_e48995 * assign43790_e48995)) / assign43790_e48996)));
        locals.var_temp1_dn8 = ((((-(locals.var_nfb_i * locals.var_nstar_dn8)) + (((locals.var_nfc_i * locals.var_nstar_dn8) * locals.var_nstar) + (assign43790_e48982 * locals.var_nstar_dn8))) * assign43790_e48997) + (assign43790_e48985 * (((((locals.var_nmstar_dn8 + (0.5 * locals.var_deltan_dn8)) * assign43790_e48995) - (assign43790_e48990 * (locals.var_nmstar_dn8 - (0.5 * locals.var_deltan_dn8)))) / (assign43790_e48995 * assign43790_e48995)) / assign43790_e48996)));
        locals.var_temp1_dn9 = ((((-(locals.var_nfb_i * locals.var_nstar_dn9)) + (((locals.var_nfc_i * locals.var_nstar_dn9) * locals.var_nstar) + (assign43790_e48982 * locals.var_nstar_dn9))) * assign43790_e48997) + (assign43790_e48985 * (((((locals.var_nmstar_dn9 + (0.5 * locals.var_deltan_dn9)) * assign43790_e48995) - (assign43790_e48990 * (locals.var_nmstar_dn9 - (0.5 * locals.var_deltan_dn9)))) / (assign43790_e48995 * assign43790_e48995)) / assign43790_e48996)));

        let assign43800_e49005: f64 = (2.0 * locals.var_nstar);
        let assign43800_e49006: f64 = (locals.var_nmstar - assign43800_e49005);
        let assign43800_e49007: f64 = (locals.var_nfc_i * assign43800_e49006);
        let assign43800_e49008: f64 = (locals.var_nfb_i + assign43800_e49007);
        let assign43800_e49010: f64 = (assign43800_e49008 * locals.var_deltan);
        let assign43800_e49011: f64 = (locals.var_temp1 + assign43800_e49010);
        locals.var_temp2 = assign43800_e49011;
        locals.var_temp2_dn4 = (locals.var_temp1_dn4 + (((locals.var_nfc_i * (locals.var_nmstar_dn4 - (2.0 * locals.var_nstar_dn4))) * locals.var_deltan) + (assign43800_e49008 * locals.var_deltan_dn4)));
        locals.var_temp2_dn6 = (locals.var_temp1_dn6 + (((locals.var_nfc_i * (locals.var_nmstar_dn6 - (2.0 * locals.var_nstar_dn6))) * locals.var_deltan) + (assign43800_e49008 * locals.var_deltan_dn6)));
        locals.var_temp2_dn7 = (locals.var_temp1_dn7 + (((locals.var_nfc_i * (locals.var_nmstar_dn7 - (2.0 * locals.var_nstar_dn7))) * locals.var_deltan) + (assign43800_e49008 * locals.var_deltan_dn7)));
        locals.var_temp2_dn8 = (locals.var_temp1_dn8 + (((locals.var_nfc_i * (locals.var_nmstar_dn8 - (2.0 * locals.var_nstar_dn8))) * locals.var_deltan) + (assign43800_e49008 * locals.var_deltan_dn8)));
        locals.var_temp2_dn9 = (locals.var_temp1_dn9 + (((locals.var_nfc_i * (locals.var_nmstar_dn9 - (2.0 * locals.var_nstar_dn9))) * locals.var_deltan) + (assign43800_e49008 * locals.var_deltan_dn9)));

        let assign43810_e49015: f64 = (locals.var_nfe_i * locals.var_esurf1_dc);
        let assign43810_e49018: f64 = (locals.var_nfeb_i * locals.var_esurf2_dc);
        let assign43810_e49019: f64 = (assign43810_e49015 + assign43810_e49018);
        let assign43810_e49022: f64 = (locals.var_qim_dc + 1.0);
        let assign43810_e49023: f64 = (assign43810_e49019 / assign43810_e49022);
        let assign43810_e49024: f64 = (1.0 + assign43810_e49023);
        locals.var_temp = assign43810_e49024;
        locals.var_temp_dn4 = (((((locals.var_nfe_i * locals.var_esurf1_dc_dn4) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn4)) * assign43810_e49022) - (assign43810_e49019 * locals.var_qim_dc_dn4)) / (assign43810_e49022 * assign43810_e49022));
        locals.var_temp_dn6 = (((((locals.var_nfe_i * locals.var_esurf1_dc_dn6) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn6)) * assign43810_e49022) - (assign43810_e49019 * locals.var_qim_dc_dn6)) / (assign43810_e49022 * assign43810_e49022));
        locals.var_temp_dn7 = (((((locals.var_nfe_i * locals.var_esurf1_dc_dn7) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn7)) * assign43810_e49022) - (assign43810_e49019 * locals.var_qim_dc_dn7)) / (assign43810_e49022 * assign43810_e49022));
        locals.var_temp_dn8 = (((((locals.var_nfe_i * locals.var_esurf1_dc_dn8) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn8)) * assign43810_e49022) - (assign43810_e49019 * locals.var_qim_dc_dn8)) / (assign43810_e49022 * assign43810_e49022));
        locals.var_temp_dn9 = (((((locals.var_nfe_i * locals.var_esurf1_dc_dn9) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn9)) * assign43810_e49022) - (assign43810_e49019 * locals.var_qim_dc_dn9)) / (assign43810_e49022 * assign43810_e49022));

    }

    pub(super) fn stamp_transient_block_121(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign43820_e49028: f64 = (locals.var_temp + 0.01);
        let assign43820_e49031: f64 = (locals.var_temp - 0.01);
        let assign43820_e49034: f64 = (locals.var_temp - 0.01);
        let assign43820_e49035: f64 = (assign43820_e49031 * assign43820_e49034);
        let assign43820_e49037: f64 = (assign43820_e49035 + 0.0001);
        let assign43820_e49038: f64 = (assign43820_e49037).sqrt();
        let assign43820_e49039: f64 = (assign43820_e49028 + assign43820_e49038);
        let assign43820_e49040: f64 = (0.5 * assign43820_e49039);
        locals.var_temp3 = assign43820_e49040;
        locals.var_temp3_dn4 = (0.5 * (locals.var_temp_dn4 + (((locals.var_temp_dn4 * assign43820_e49034) + (assign43820_e49031 * locals.var_temp_dn4)) / (2.0 * assign43820_e49038))));
        locals.var_temp3_dn6 = (0.5 * (locals.var_temp_dn6 + (((locals.var_temp_dn6 * assign43820_e49034) + (assign43820_e49031 * locals.var_temp_dn6)) / (2.0 * assign43820_e49038))));
        locals.var_temp3_dn7 = (0.5 * (locals.var_temp_dn7 + (((locals.var_temp_dn7 * assign43820_e49034) + (assign43820_e49031 * locals.var_temp_dn7)) / (2.0 * assign43820_e49038))));
        locals.var_temp3_dn8 = (0.5 * (locals.var_temp_dn8 + (((locals.var_temp_dn8 * assign43820_e49034) + (assign43820_e49031 * locals.var_temp_dn8)) / (2.0 * assign43820_e49038))));
        locals.var_temp3_dn9 = (0.5 * (locals.var_temp_dn9 + (((locals.var_temp_dn9 * assign43820_e49034) + (assign43820_e49031 * locals.var_temp_dn9)) / (2.0 * assign43820_e49038))));

        let assign43830_e49043: f64 = (1.602176565e-19 * locals.var_fact_ids);
        let assign43830_e49045: f64 = (assign43830_e49043 * locals.var_ids);
        let assign43830_e49047: f64 = (assign43830_e49045 / locals.var_gvsat);
        let assign43830_e49049: f64 = (assign43830_e49047 * locals.var_temp2);
        let assign43830_e49051: f64 = (assign43830_e49049 / locals.var_nstar);
        let assign43830_e49053: f64 = (assign43830_e49051 * locals.var_temp3);
        locals.var_temp = assign43830_e49053;
        locals.var_temp_dn4 = (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn4) * locals.var_ids) + (assign43830_e49043 * locals.var_ids_dn4)) * locals.var_gvsat) - (assign43830_e49045 * locals.var_gvsat_dn4)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43830_e49047 * locals.var_temp2_dn4)) * locals.var_nstar) - (assign43830_e49049 * locals.var_nstar_dn4)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43830_e49051 * locals.var_temp3_dn4));
        locals.var_temp_dn6 = (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn6) * locals.var_ids) + (assign43830_e49043 * locals.var_ids_dn6)) * locals.var_gvsat) - (assign43830_e49045 * locals.var_gvsat_dn6)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43830_e49047 * locals.var_temp2_dn6)) * locals.var_nstar) - (assign43830_e49049 * locals.var_nstar_dn6)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43830_e49051 * locals.var_temp3_dn6));
        locals.var_temp_dn7 = (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn7) * locals.var_ids) + (assign43830_e49043 * locals.var_ids_dn7)) * locals.var_gvsat) - (assign43830_e49045 * locals.var_gvsat_dn7)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43830_e49047 * locals.var_temp2_dn7)) * locals.var_nstar) - (assign43830_e49049 * locals.var_nstar_dn7)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43830_e49051 * locals.var_temp3_dn7));
        locals.var_temp_dn8 = (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn8) * locals.var_ids) + (assign43830_e49043 * locals.var_ids_dn8)) * locals.var_gvsat) - (assign43830_e49045 * locals.var_gvsat_dn8)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43830_e49047 * locals.var_temp2_dn8)) * locals.var_nstar) - (assign43830_e49049 * locals.var_nstar_dn8)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43830_e49051 * locals.var_temp3_dn8));
        locals.var_temp_dn9 = (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn9) * locals.var_ids) + (assign43830_e49043 * locals.var_ids_dn9)) * locals.var_gvsat) - (assign43830_e49045 * locals.var_gvsat_dn9)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43830_e49047 * locals.var_temp2_dn9)) * locals.var_nstar) - (assign43830_e49049 * locals.var_nstar_dn9)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43830_e49051 * locals.var_temp3_dn9));

        let assign43900_e49113: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign43900_e49114: f64 = (1.0 / assign43900_e49113);
        locals.var_inv_phit0_op = assign43900_e49114;
        locals.var_inv_phit0_op_dn4 = (-((locals.var_tkd_dn4 * 8.617332384961e-5) / (assign43900_e49113 * assign43900_e49113)));
        locals.var_inv_phit0_op_dn6 = (-((locals.var_tkd_dn6 * 8.617332384961e-5) / (assign43900_e49113 * assign43900_e49113)));
        locals.var_inv_phit0_op_dn7 = (-((locals.var_tkd_dn7 * 8.617332384961e-5) / (assign43900_e49113 * assign43900_e49113)));
        locals.var_inv_phit0_op_dn8 = (-((locals.var_tkd_dn8 * 8.617332384961e-5) / (assign43900_e49113 * assign43900_e49113)));
        locals.var_inv_phit0_op_dn9 = (-((locals.var_tkd_dn9 * 8.617332384961e-5) / (assign43900_e49113 * assign43900_e49113)));

        let assign43910_e49118: f64 = (0.000473 * locals.var_tkd);
        let assign43910_e49120: f64 = (assign43910_e49118 * locals.var_tkd);
        let assign43910_e49123: f64 = (636.0 + locals.var_tkd);
        let assign43910_e49124: f64 = (assign43910_e49120 / assign43910_e49123);
        let assign43910_e49125: f64 = (1.17 - assign43910_e49124);
        locals.var_egsi_op = assign43910_e49125;
        locals.var_egsi_op_dn4 = (-((((((0.000473 * locals.var_tkd_dn4) * locals.var_tkd) + (assign43910_e49118 * locals.var_tkd_dn4)) * assign43910_e49123) - (assign43910_e49120 * locals.var_tkd_dn4)) / (assign43910_e49123 * assign43910_e49123)));
        locals.var_egsi_op_dn6 = (-((((((0.000473 * locals.var_tkd_dn6) * locals.var_tkd) + (assign43910_e49118 * locals.var_tkd_dn6)) * assign43910_e49123) - (assign43910_e49120 * locals.var_tkd_dn6)) / (assign43910_e49123 * assign43910_e49123)));
        locals.var_egsi_op_dn7 = (-((((((0.000473 * locals.var_tkd_dn7) * locals.var_tkd) + (assign43910_e49118 * locals.var_tkd_dn7)) * assign43910_e49123) - (assign43910_e49120 * locals.var_tkd_dn7)) / (assign43910_e49123 * assign43910_e49123)));
        locals.var_egsi_op_dn8 = (-((((((0.000473 * locals.var_tkd_dn8) * locals.var_tkd) + (assign43910_e49118 * locals.var_tkd_dn8)) * assign43910_e49123) - (assign43910_e49120 * locals.var_tkd_dn8)) / (assign43910_e49123 * assign43910_e49123)));
        locals.var_egsi_op_dn9 = (-((((((0.000473 * locals.var_tkd_dn9) * locals.var_tkd) + (assign43910_e49118 * locals.var_tkd_dn9)) * assign43910_e49123) - (assign43910_e49120 * locals.var_tkd_dn9)) / (assign43910_e49123 * assign43910_e49123)));

        let assign43920_e49129: f64 = (0.0004774 * locals.var_tkd);
        let assign43920_e49131: f64 = (assign43920_e49129 * locals.var_tkd);
        let assign43920_e49134: f64 = (235.0 + locals.var_tkd);
        let assign43920_e49135: f64 = (assign43920_e49131 / assign43920_e49134);
        let assign43920_e49136: f64 = (0.744 - assign43920_e49135);
        locals.var_egge_op = assign43920_e49136;
        locals.var_egge_op_dn4 = (-((((((0.0004774 * locals.var_tkd_dn4) * locals.var_tkd) + (assign43920_e49129 * locals.var_tkd_dn4)) * assign43920_e49134) - (assign43920_e49131 * locals.var_tkd_dn4)) / (assign43920_e49134 * assign43920_e49134)));
        locals.var_egge_op_dn6 = (-((((((0.0004774 * locals.var_tkd_dn6) * locals.var_tkd) + (assign43920_e49129 * locals.var_tkd_dn6)) * assign43920_e49134) - (assign43920_e49131 * locals.var_tkd_dn6)) / (assign43920_e49134 * assign43920_e49134)));
        locals.var_egge_op_dn7 = (-((((((0.0004774 * locals.var_tkd_dn7) * locals.var_tkd) + (assign43920_e49129 * locals.var_tkd_dn7)) * assign43920_e49134) - (assign43920_e49131 * locals.var_tkd_dn7)) / (assign43920_e49134 * assign43920_e49134)));
        locals.var_egge_op_dn8 = (-((((((0.0004774 * locals.var_tkd_dn8) * locals.var_tkd) + (assign43920_e49129 * locals.var_tkd_dn8)) * assign43920_e49134) - (assign43920_e49131 * locals.var_tkd_dn8)) / (assign43920_e49134 * assign43920_e49134)));
        locals.var_egge_op_dn9 = (-((((((0.0004774 * locals.var_tkd_dn9) * locals.var_tkd) + (assign43920_e49129 * locals.var_tkd_dn9)) * assign43920_e49134) - (assign43920_e49131 * locals.var_tkd_dn9)) / (assign43920_e49134 * assign43920_e49134)));

        let assign43930_e49139: f64 = (locals.var_egge_op - locals.var_egsi_op);
        let assign43930_e49141: f64 = (-0.4);
        let assign43930_e49143: f64 = (assign43930_e49141 * locals.var_one_m_xge);
        let assign43930_e49144: f64 = (assign43930_e49139 + assign43930_e49143);
        let assign43930_e49146: f64 = (assign43930_e49144 * locals.var_xge_i);
        locals.var_deg_op = assign43930_e49146;
        locals.var_deg_op_dn4 = ((locals.var_egge_op_dn4 - locals.var_egsi_op_dn4) * locals.var_xge_i);
        locals.var_deg_op_dn6 = ((locals.var_egge_op_dn6 - locals.var_egsi_op_dn6) * locals.var_xge_i);
        locals.var_deg_op_dn7 = ((locals.var_egge_op_dn7 - locals.var_egsi_op_dn7) * locals.var_xge_i);
        locals.var_deg_op_dn8 = ((locals.var_egge_op_dn8 - locals.var_egsi_op_dn8) * locals.var_xge_i);
        locals.var_deg_op_dn9 = ((locals.var_egge_op_dn9 - locals.var_egsi_op_dn9) * locals.var_xge_i);

        let assign43940_e49149: f64 = (locals.var_egsi_op + locals.var_deg_op);
        locals.var_eg_op = assign43940_e49149;
        locals.var_eg_op_dn4 = (locals.var_egsi_op_dn4 + locals.var_deg_op_dn4);
        locals.var_eg_op_dn6 = (locals.var_egsi_op_dn6 + locals.var_deg_op_dn6);
        locals.var_eg_op_dn7 = (locals.var_egsi_op_dn7 + locals.var_deg_op_dn7);
        locals.var_eg_op_dn8 = (locals.var_egsi_op_dn8 + locals.var_deg_op_dn8);
        locals.var_eg_op_dn9 = (locals.var_egsi_op_dn9 + locals.var_deg_op_dn9);

        let assign43950_e49152: f64 = (0.5 * locals.var_eg_op);
        let assign43950_e49154: f64 = (assign43950_e49152 * locals.var_inv_phit0_op);
        locals.var_eg_2phit0_op = assign43950_e49154;
        locals.var_eg_2phit0_op_dn4 = (((0.5 * locals.var_eg_op_dn4) * locals.var_inv_phit0_op) + (assign43950_e49152 * locals.var_inv_phit0_op_dn4));
        locals.var_eg_2phit0_op_dn6 = (((0.5 * locals.var_eg_op_dn6) * locals.var_inv_phit0_op) + (assign43950_e49152 * locals.var_inv_phit0_op_dn6));
        locals.var_eg_2phit0_op_dn7 = (((0.5 * locals.var_eg_op_dn7) * locals.var_inv_phit0_op) + (assign43950_e49152 * locals.var_inv_phit0_op_dn7));
        locals.var_eg_2phit0_op_dn8 = (((0.5 * locals.var_eg_op_dn8) * locals.var_inv_phit0_op) + (assign43950_e49152 * locals.var_inv_phit0_op_dn8));
        locals.var_eg_2phit0_op_dn9 = (((0.5 * locals.var_eg_op_dn9) * locals.var_inv_phit0_op) + (assign43950_e49152 * locals.var_inv_phit0_op_dn9));

        let assign43960_e49157: f64 = (0.05 * locals.var_xge_i);
        let assign43960_e49160: f64 = (0.5 * locals.var_deg_op);
        let assign43960_e49161: f64 = (assign43960_e49157 - assign43960_e49160);
        locals.var_dvfbch_op = assign43960_e49161;
        locals.var_dvfbch_op_dn4 = (-(0.5 * locals.var_deg_op_dn4));
        locals.var_dvfbch_op_dn6 = (-(0.5 * locals.var_deg_op_dn6));
        locals.var_dvfbch_op_dn7 = (-(0.5 * locals.var_deg_op_dn7));
        locals.var_dvfbch_op_dn8 = (-(0.5 * locals.var_deg_op_dn8));
        locals.var_dvfbch_op_dn9 = (-(0.5 * locals.var_deg_op_dn9));

        let assign43970_e49164: f64 = (locals.var_tkd * 0.0033333333333);
        let assign43970_e49165: f64 = (assign43970_e49164).sqrt();
        locals.var_temp = assign43970_e49165;
        locals.var_temp_dn4 = ((locals.var_tkd_dn4 * 0.0033333333333) / (2.0 * assign43970_e49165));
        locals.var_temp_dn6 = ((locals.var_tkd_dn6 * 0.0033333333333) / (2.0 * assign43970_e49165));
        locals.var_temp_dn7 = ((locals.var_tkd_dn7 * 0.0033333333333) / (2.0 * assign43970_e49165));
        locals.var_temp_dn8 = ((locals.var_tkd_dn8 * 0.0033333333333) / (2.0 * assign43970_e49165));
        locals.var_temp_dn9 = ((locals.var_tkd_dn9 * 0.0033333333333) / (2.0 * assign43970_e49165));

        let assign43980_e49168: f64 = (4.05e25 * locals.var_temp);
        let assign43980_e49170: f64 = (assign43980_e49168 * locals.var_temp);
        let assign43980_e49172: f64 = (assign43980_e49170 * locals.var_temp);
        locals.var_temp1 = assign43980_e49172;
        locals.var_temp1_dn4 = (((((4.05e25 * locals.var_temp_dn4) * locals.var_temp) + (assign43980_e49168 * locals.var_temp_dn4)) * locals.var_temp) + (assign43980_e49170 * locals.var_temp_dn4));
        locals.var_temp1_dn6 = (((((4.05e25 * locals.var_temp_dn6) * locals.var_temp) + (assign43980_e49168 * locals.var_temp_dn6)) * locals.var_temp) + (assign43980_e49170 * locals.var_temp_dn6));
        locals.var_temp1_dn7 = (((((4.05e25 * locals.var_temp_dn7) * locals.var_temp) + (assign43980_e49168 * locals.var_temp_dn7)) * locals.var_temp) + (assign43980_e49170 * locals.var_temp_dn7));
        locals.var_temp1_dn8 = (((((4.05e25 * locals.var_temp_dn8) * locals.var_temp) + (assign43980_e49168 * locals.var_temp_dn8)) * locals.var_temp) + (assign43980_e49170 * locals.var_temp_dn8));
        locals.var_temp1_dn9 = (((((4.05e25 * locals.var_temp_dn9) * locals.var_temp) + (assign43980_e49168 * locals.var_temp_dn9)) * locals.var_temp) + (assign43980_e49170 * locals.var_temp_dn9));

        let assign43990_e49175: f64 = (locals.var_temp1 * locals.var_niratio);
        locals.var_neff_op = assign43990_e49175;
        locals.var_neff_op_dn4 = (locals.var_temp1_dn4 * locals.var_niratio);
        locals.var_neff_op_dn6 = (locals.var_temp1_dn6 * locals.var_niratio);
        locals.var_neff_op_dn7 = (locals.var_temp1_dn7 * locals.var_niratio);
        locals.var_neff_op_dn8 = (locals.var_temp1_dn8 * locals.var_niratio);
        locals.var_neff_op_dn9 = (locals.var_temp1_dn9 * locals.var_niratio);

        let assign44000_e49180: f64 = (locals.var_ct_i * locals.var_tkr);
        let assign44000_e49182: f64 = (assign44000_e49180 / locals.var_tkd);
        let assign44000_e49183: f64 = (1.0 + assign44000_e49182);
        let assign44000_e49184: f64 = (locals.var_inv_phit0_op / assign44000_e49183);
        locals.var_inv_phit_op = assign44000_e49184;
        locals.var_inv_phit_op_dn4 = (((locals.var_inv_phit0_op_dn4 * assign44000_e49183) - (locals.var_inv_phit0_op * (-((assign44000_e49180 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd))))) / (assign44000_e49183 * assign44000_e49183));
        locals.var_inv_phit_op_dn6 = (((locals.var_inv_phit0_op_dn6 * assign44000_e49183) - (locals.var_inv_phit0_op * (-((assign44000_e49180 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd))))) / (assign44000_e49183 * assign44000_e49183));
        locals.var_inv_phit_op_dn7 = (((locals.var_inv_phit0_op_dn7 * assign44000_e49183) - (locals.var_inv_phit0_op * (-((assign44000_e49180 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd))))) / (assign44000_e49183 * assign44000_e49183));
        locals.var_inv_phit_op_dn8 = (((locals.var_inv_phit0_op_dn8 * assign44000_e49183) - (locals.var_inv_phit0_op * (-((assign44000_e49180 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd))))) / (assign44000_e49183 * assign44000_e49183));
        locals.var_inv_phit_op_dn9 = (((locals.var_inv_phit0_op_dn9 * assign44000_e49183) - (locals.var_inv_phit0_op * (-((assign44000_e49180 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd))))) / (assign44000_e49183 * assign44000_e49183));

        let assign44010_e49187: f64 = (2.0 * 1.602176565e-19);
        let assign44010_e49189: f64 = (assign44010_e49187 * locals.var_neff_op);
        let assign44010_e49191: f64 = (assign44010_e49189 * locals.var_epsch);
        let assign44010_e49193: f64 = (assign44010_e49191 * locals.var_inv_phit_op);
        locals.var_a0_csisq_op = assign44010_e49193;
        locals.var_a0_csisq_op_dn4 = ((((assign44010_e49187 * locals.var_neff_op_dn4) * locals.var_epsch) * locals.var_inv_phit_op) + (assign44010_e49191 * locals.var_inv_phit_op_dn4));
        locals.var_a0_csisq_op_dn6 = ((((assign44010_e49187 * locals.var_neff_op_dn6) * locals.var_epsch) * locals.var_inv_phit_op) + (assign44010_e49191 * locals.var_inv_phit_op_dn6));
        locals.var_a0_csisq_op_dn7 = ((((assign44010_e49187 * locals.var_neff_op_dn7) * locals.var_epsch) * locals.var_inv_phit_op) + (assign44010_e49191 * locals.var_inv_phit_op_dn7));
        locals.var_a0_csisq_op_dn8 = ((((assign44010_e49187 * locals.var_neff_op_dn8) * locals.var_epsch) * locals.var_inv_phit_op) + (assign44010_e49191 * locals.var_inv_phit_op_dn8));
        locals.var_a0_csisq_op_dn9 = ((((assign44010_e49187 * locals.var_neff_op_dn9) * locals.var_epsch) * locals.var_inv_phit_op) + (assign44010_e49191 * locals.var_inv_phit_op_dn9));

        let assign44020_e49196: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
        let assign44020_e49198: f64 = (assign44020_e49196 / locals.var_a0_csisq_op);
        let assign44020_e49199: f64 = (assign44020_e49198).ln();
        let assign44020_e49201: f64 = (assign44020_e49199 - 0.6931471805599);
        let assign44020_e49203: f64 = (assign44020_e49201 + locals.var_eg_2phit0_op);
        locals.var_xth_1d_op = assign44020_e49203;
        locals.var_xth_1d_op_dn4 = (((-((assign44020_e49196 * locals.var_a0_csisq_op_dn4) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign44020_e49198) + locals.var_eg_2phit0_op_dn4);
        locals.var_xth_1d_op_dn6 = (((-((assign44020_e49196 * locals.var_a0_csisq_op_dn6) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign44020_e49198) + locals.var_eg_2phit0_op_dn6);
        locals.var_xth_1d_op_dn7 = (((-((assign44020_e49196 * locals.var_a0_csisq_op_dn7) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign44020_e49198) + locals.var_eg_2phit0_op_dn7);
        locals.var_xth_1d_op_dn8 = (((-((assign44020_e49196 * locals.var_a0_csisq_op_dn8) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign44020_e49198) + locals.var_eg_2phit0_op_dn8);
        locals.var_xth_1d_op_dn9 = (((-((assign44020_e49196 * locals.var_a0_csisq_op_dn9) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign44020_e49198) + locals.var_eg_2phit0_op_dn9);

        let assign44030_e49206: f64 = (0.5 * 1.602176565e-19);
        let assign44030_e49208: f64 = (assign44030_e49206 * locals.var_nsddc_i);
        let assign44030_e49210: f64 = (assign44030_e49208 * locals.var_tsi_i);
        let assign44030_e49213: f64 = (locals.var_cox1prime + locals.var_cox2prime);
        let assign44030_e49214: f64 = (assign44030_e49210 / assign44030_e49213);
        let assign44030_e49216: f64 = (assign44030_e49214 * locals.var_inv_phit_op);
        locals.var_xsddep_op = assign44030_e49216;
        locals.var_xsddep_op_dn4 = (assign44030_e49214 * locals.var_inv_phit_op_dn4);
        locals.var_xsddep_op_dn6 = (assign44030_e49214 * locals.var_inv_phit_op_dn6);
        locals.var_xsddep_op_dn7 = (assign44030_e49214 * locals.var_inv_phit_op_dn7);
        locals.var_xsddep_op_dn8 = (assign44030_e49214 * locals.var_inv_phit_op_dn8);
        locals.var_xsddep_op_dn9 = (assign44030_e49214 * locals.var_inv_phit_op_dn9);

        let assign44040_e49219: f64 = (locals.var_cfd_i * locals.var_inv_phit_op);
        locals.var_xd0_op = assign44040_e49219;
        locals.var_xd0_op_dn4 = (locals.var_cfd_i * locals.var_inv_phit_op_dn4);
        locals.var_xd0_op_dn6 = (locals.var_cfd_i * locals.var_inv_phit_op_dn6);
        locals.var_xd0_op_dn7 = (locals.var_cfd_i * locals.var_inv_phit_op_dn7);
        locals.var_xd0_op_dn8 = (locals.var_cfd_i * locals.var_inv_phit_op_dn8);
        locals.var_xd0_op_dn9 = (locals.var_cfd_i * locals.var_inv_phit_op_dn9);

        locals.var_qq_op = 0.0;
        locals.var_qq_op_dn4 = 0.0;
        locals.var_qq_op_dn6 = 0.0;
        locals.var_qq_op_dn7 = 0.0;
        locals.var_qq_op_dn8 = 0.0;
        locals.var_qq_op_dn9 = 0.0;

        locals.var_dvfbpdep_op = 0.0;
        locals.var_dvfbpdep_op_dn4 = 0.0;
        locals.var_dvfbpdep_op_dn6 = 0.0;
        locals.var_dvfbpdep_op_dn7 = 0.0;
        locals.var_dvfbpdep_op_dn8 = 0.0;
        locals.var_dvfbpdep_op_dn9 = 0.0;

        let assign44070_e49224: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1356 = assign44070_e49224;

        let (assign44080_e49235, assign44080_e49235_d_n4, assign44080_e49235_d_n6, assign44080_e49235_d_n7, assign44080_e49235_d_n8, assign44080_e49235_d_n9,) = {
    if (locals.var_guard1356 != 0.0) {
        let assign44080_e49228: f64 = (1.0 / locals.var_inv_phit0_op);
        let assign44080_e49231: f64 = (locals.var_np_i / locals.var_neff_poly);
        let assign44080_e49232: f64 = (assign44080_e49231).ln();
        let assign44080_e49233: f64 = (assign44080_e49228 * assign44080_e49232);
        (assign44080_e49233, (((-(locals.var_inv_phit0_op_dn4 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign44080_e49232) + (assign44080_e49228 * ((((locals.var_np_i_dn4 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn4)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign44080_e49231))), (((-(locals.var_inv_phit0_op_dn6 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign44080_e49232) + (assign44080_e49228 * ((((locals.var_np_i_dn6 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn6)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign44080_e49231))), (((-(locals.var_inv_phit0_op_dn7 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign44080_e49232) + (assign44080_e49228 * ((((locals.var_np_i_dn7 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn7)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign44080_e49231))), (((-(locals.var_inv_phit0_op_dn8 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign44080_e49232) + (assign44080_e49228 * ((((locals.var_np_i_dn8 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn8)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign44080_e49231))), (((-(locals.var_inv_phit0_op_dn9 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign44080_e49232) + (assign44080_e49228 * ((((locals.var_np_i_dn9 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn9)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign44080_e49231))),)
    } else {
        (locals.var_dvfbpdep_op, locals.var_dvfbpdep_op_dn4, locals.var_dvfbpdep_op_dn6, locals.var_dvfbpdep_op_dn7, locals.var_dvfbpdep_op_dn8, locals.var_dvfbpdep_op_dn9,)
    }
};
        locals.var_dvfbpdep_op = assign44080_e49235;
        locals.var_dvfbpdep_op_dn4 = assign44080_e49235_d_n4;
        locals.var_dvfbpdep_op_dn6 = assign44080_e49235_d_n6;
        locals.var_dvfbpdep_op_dn7 = assign44080_e49235_d_n7;
        locals.var_dvfbpdep_op_dn8 = assign44080_e49235_d_n8;
        locals.var_dvfbpdep_op_dn9 = assign44080_e49235_d_n9;

        let assign44090_e49238: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1357 = assign44090_e49238;

        let assign44100_e49241: f64 = 1.0;
        let assign44100_e49242: f64 = if p.p14 == assign44100_e49241 { 1.0 } else { 0.0 };
        locals.var_guard1358 = assign44100_e49242;

        let (assign44110_e49261, assign44110_e49261_d_n4, assign44110_e49261_d_n6, assign44110_e49261_d_n7, assign44110_e49261_d_n8, assign44110_e49261_d_n9,) = {
    if ((locals.var_guard1357 != 0.0) && (locals.var_guard1358 != 0.0)) {
        let assign44110_e49248: f64 = (0.4 * p.p13);
        let assign44110_e49250: f64 = (assign44110_e49248 * 1.27520989);
        let assign44110_e49252: f64 = (-0.3333333333333);
        let assign44110_e49255: f64 = (locals.var_tsisq / locals.var_inv_phit_op);
        let assign44110_e49256: f64 = (assign44110_e49255).ln();
        let assign44110_e49257: f64 = (assign44110_e49252 * assign44110_e49256);
        let assign44110_e49258: f64 = (assign44110_e49257).exp();
        let assign44110_e49259: f64 = (assign44110_e49250 * assign44110_e49258);
        (assign44110_e49259, (assign44110_e49250 * (assign44110_e49258 * (assign44110_e49252 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn4) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44110_e49255)))), (assign44110_e49250 * (assign44110_e49258 * (assign44110_e49252 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn6) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44110_e49255)))), (assign44110_e49250 * (assign44110_e49258 * (assign44110_e49252 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn7) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44110_e49255)))), (assign44110_e49250 * (assign44110_e49258 * (assign44110_e49252 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn8) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44110_e49255)))), (assign44110_e49250 * (assign44110_e49258 * (assign44110_e49252 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn9) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44110_e49255)))),)
    } else {
        (locals.var_qq_op, locals.var_qq_op_dn4, locals.var_qq_op_dn6, locals.var_qq_op_dn7, locals.var_qq_op_dn8, locals.var_qq_op_dn9,)
    }
};
        locals.var_qq_op = assign44110_e49261;
        locals.var_qq_op_dn4 = assign44110_e49261_d_n4;
        locals.var_qq_op_dn6 = assign44110_e49261_d_n6;
        locals.var_qq_op_dn7 = assign44110_e49261_d_n7;
        locals.var_qq_op_dn8 = assign44110_e49261_d_n8;
        locals.var_qq_op_dn9 = assign44110_e49261_d_n9;

        let (assign44120_e49281, assign44120_e49281_d_n4, assign44120_e49281_d_n6, assign44120_e49281_d_n7, assign44120_e49281_d_n8, assign44120_e49281_d_n9,) = {
    if ((locals.var_guard1357 != 0.0) && (locals.var_guard1358 == 0.0)) {
        let assign44120_e49268: f64 = (0.4 * p.p13);
        let assign44120_e49270: f64 = (assign44120_e49268 * 1.5412087);
        let assign44120_e49272: f64 = (-0.3333333333333);
        let assign44120_e49275: f64 = (locals.var_tsisq / locals.var_inv_phit_op);
        let assign44120_e49276: f64 = (assign44120_e49275).ln();
        let assign44120_e49277: f64 = (assign44120_e49272 * assign44120_e49276);
        let assign44120_e49278: f64 = (assign44120_e49277).exp();
        let assign44120_e49279: f64 = (assign44120_e49270 * assign44120_e49278);
        (assign44120_e49279, (assign44120_e49270 * (assign44120_e49278 * (assign44120_e49272 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn4) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44120_e49275)))), (assign44120_e49270 * (assign44120_e49278 * (assign44120_e49272 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn6) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44120_e49275)))), (assign44120_e49270 * (assign44120_e49278 * (assign44120_e49272 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn7) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44120_e49275)))), (assign44120_e49270 * (assign44120_e49278 * (assign44120_e49272 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn8) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44120_e49275)))), (assign44120_e49270 * (assign44120_e49278 * (assign44120_e49272 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn9) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign44120_e49275)))),)
    } else {
        (locals.var_qq_op, locals.var_qq_op_dn4, locals.var_qq_op_dn6, locals.var_qq_op_dn7, locals.var_qq_op_dn8, locals.var_qq_op_dn9,)
    }
};
        locals.var_qq_op = assign44120_e49281;
        locals.var_qq_op_dn4 = assign44120_e49281_d_n4;
        locals.var_qq_op_dn6 = assign44120_e49281_d_n6;
        locals.var_qq_op_dn7 = assign44120_e49281_d_n7;
        locals.var_qq_op_dn8 = assign44120_e49281_d_n8;
        locals.var_qq_op_dn9 = assign44120_e49281_d_n9;

        let assign44130_e49284: f64 = (locals.var_vds * locals.var_inv_phit_op);
        locals.var_xd_op = assign44130_e49284;
        locals.var_xd_op_dn4 = (locals.var_vds * locals.var_inv_phit_op_dn4);
        locals.var_xd_op_dn6 = ((locals.var_vds_dn6 * locals.var_inv_phit_op) + (locals.var_vds * locals.var_inv_phit_op_dn6));
        locals.var_xd_op_dn7 = ((locals.var_vds_dn7 * locals.var_inv_phit_op) + (locals.var_vds * locals.var_inv_phit_op_dn7));
        locals.var_xd_op_dn8 = (locals.var_vds * locals.var_inv_phit_op_dn8);
        locals.var_xd_op_dn9 = (locals.var_vds * locals.var_inv_phit_op_dn9);

        let assign44140_e49287: f64 = (locals.var_vds * locals.var_vds);
        let assign44140_e49289: f64 = (assign44140_e49287 + 0.01);
        let assign44140_e49290: f64 = (assign44140_e49289).sqrt();
        let assign44140_e49292: f64 = (assign44140_e49290 - 0.1);
        let assign44140_e49294: f64 = (assign44140_e49292 * locals.var_inv_phit_op);
        locals.var_xdsx_op = assign44140_e49294;
        locals.var_xdsx_op_dn4 = (assign44140_e49292 * locals.var_inv_phit_op_dn4);
        locals.var_xdsx_op_dn6 = (((((locals.var_vds_dn6 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn6)) / (2.0 * assign44140_e49290)) * locals.var_inv_phit_op) + (assign44140_e49292 * locals.var_inv_phit_op_dn6));
        locals.var_xdsx_op_dn7 = (((((locals.var_vds_dn7 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn7)) / (2.0 * assign44140_e49290)) * locals.var_inv_phit_op) + (assign44140_e49292 * locals.var_inv_phit_op_dn7));
        locals.var_xdsx_op_dn8 = (assign44140_e49292 * locals.var_inv_phit_op_dn8);
        locals.var_xdsx_op_dn9 = (assign44140_e49292 * locals.var_inv_phit_op_dn9);

        let assign44150_e49298: f64 = (locals.var_xd_op - locals.var_xdsx_op);
        let assign44150_e49299: f64 = (0.5 * assign44150_e49298);
        locals.var_dxdsx_op = assign44150_e49299;
        locals.var_dxdsx_op_dn4 = (0.5 * (locals.var_xd_op_dn4 - locals.var_xdsx_op_dn4));
        locals.var_dxdsx_op_dn6 = (0.5 * (locals.var_xd_op_dn6 - locals.var_xdsx_op_dn6));
        locals.var_dxdsx_op_dn7 = (0.5 * (locals.var_xd_op_dn7 - locals.var_xdsx_op_dn7));
        locals.var_dxdsx_op_dn8 = (0.5 * (locals.var_xd_op_dn8 - locals.var_xdsx_op_dn8));
        locals.var_dxdsx_op_dn9 = (0.5 * (locals.var_xd_op_dn9 - locals.var_xdsx_op_dn9));

        let assign44160_e49302: f64 = (locals.var_k2_dc / locals.var_k1_dc);
        let assign44160_e49305: f64 = (1.0 + locals.var_k2_dc);
        let assign44160_e49306: f64 = (assign44160_e49302 / assign44160_e49305);
        locals.var_r1init_op = assign44160_e49306;
        locals.var_r1init_op_dn4 = ((((((locals.var_k2_dc_dn4 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn4)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign44160_e49305) - (assign44160_e49302 * locals.var_k2_dc_dn4)) / (assign44160_e49305 * assign44160_e49305));
        locals.var_r1init_op_dn6 = ((((((locals.var_k2_dc_dn6 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn6)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign44160_e49305) - (assign44160_e49302 * locals.var_k2_dc_dn6)) / (assign44160_e49305 * assign44160_e49305));
        locals.var_r1init_op_dn7 = ((((((locals.var_k2_dc_dn7 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn7)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign44160_e49305) - (assign44160_e49302 * locals.var_k2_dc_dn7)) / (assign44160_e49305 * assign44160_e49305));
        locals.var_r1init_op_dn8 = ((((((locals.var_k2_dc_dn8 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn8)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign44160_e49305) - (assign44160_e49302 * locals.var_k2_dc_dn8)) / (assign44160_e49305 * assign44160_e49305));
        locals.var_r1init_op_dn9 = ((((((locals.var_k2_dc_dn9 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn9)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign44160_e49305) - (assign44160_e49302 * locals.var_k2_dc_dn9)) / (assign44160_e49305 * assign44160_e49305));

        let assign44170_e49309: f64 = (locals.var_k1_dc / locals.var_k2_dc);
        let assign44170_e49312: f64 = (1.0 + locals.var_k1_dc);
        let assign44170_e49313: f64 = (assign44170_e49309 / assign44170_e49312);
        locals.var_r2init_op = assign44170_e49313;
        locals.var_r2init_op_dn4 = ((((((locals.var_k1_dc_dn4 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn4)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign44170_e49312) - (assign44170_e49309 * locals.var_k1_dc_dn4)) / (assign44170_e49312 * assign44170_e49312));
        locals.var_r2init_op_dn6 = ((((((locals.var_k1_dc_dn6 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn6)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign44170_e49312) - (assign44170_e49309 * locals.var_k1_dc_dn6)) / (assign44170_e49312 * assign44170_e49312));
        locals.var_r2init_op_dn7 = ((((((locals.var_k1_dc_dn7 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn7)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign44170_e49312) - (assign44170_e49309 * locals.var_k1_dc_dn7)) / (assign44170_e49312 * assign44170_e49312));
        locals.var_r2init_op_dn8 = ((((((locals.var_k1_dc_dn8 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn8)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign44170_e49312) - (assign44170_e49309 * locals.var_k1_dc_dn8)) / (assign44170_e49312 * assign44170_e49312));
        locals.var_r2init_op_dn9 = ((((((locals.var_k1_dc_dn9 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn9)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign44170_e49312) - (assign44170_e49309 * locals.var_k1_dc_dn9)) / (assign44170_e49312 * assign44170_e49312));

        let assign44180_e49317: f64 = (1.0 + locals.var_r1init_op);
        let assign44180_e49318: f64 = (locals.var_k1_dc * assign44180_e49317);
        let assign44180_e49320: f64 = (assign44180_e49318 * locals.var_diff_min_dc);
        let assign44180_e49322: f64 = (assign44180_e49320 / locals.var_a0_dc);
        let assign44180_e49323: f64 = (assign44180_e49322).ln();
        let assign44180_e49325: f64 = (assign44180_e49323 + 2.0);
        locals.var_x1init_op = assign44180_e49325;
        locals.var_x1init_op_dn4 = ((((((((locals.var_k1_dc_dn4 * assign44180_e49317) + (locals.var_k1_dc * locals.var_r1init_op_dn4)) * locals.var_diff_min_dc) + (assign44180_e49318 * locals.var_diff_min_dc_dn4)) * locals.var_a0_dc) - (assign44180_e49320 * locals.var_a0_dc_dn4)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44180_e49322);
        locals.var_x1init_op_dn6 = ((((((((locals.var_k1_dc_dn6 * assign44180_e49317) + (locals.var_k1_dc * locals.var_r1init_op_dn6)) * locals.var_diff_min_dc) + (assign44180_e49318 * locals.var_diff_min_dc_dn6)) * locals.var_a0_dc) - (assign44180_e49320 * locals.var_a0_dc_dn6)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44180_e49322);
        locals.var_x1init_op_dn7 = ((((((((locals.var_k1_dc_dn7 * assign44180_e49317) + (locals.var_k1_dc * locals.var_r1init_op_dn7)) * locals.var_diff_min_dc) + (assign44180_e49318 * locals.var_diff_min_dc_dn7)) * locals.var_a0_dc) - (assign44180_e49320 * locals.var_a0_dc_dn7)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44180_e49322);
        locals.var_x1init_op_dn8 = ((((((((locals.var_k1_dc_dn8 * assign44180_e49317) + (locals.var_k1_dc * locals.var_r1init_op_dn8)) * locals.var_diff_min_dc) + (assign44180_e49318 * locals.var_diff_min_dc_dn8)) * locals.var_a0_dc) - (assign44180_e49320 * locals.var_a0_dc_dn8)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44180_e49322);
        locals.var_x1init_op_dn9 = ((((((((locals.var_k1_dc_dn9 * assign44180_e49317) + (locals.var_k1_dc * locals.var_r1init_op_dn9)) * locals.var_diff_min_dc) + (assign44180_e49318 * locals.var_diff_min_dc_dn9)) * locals.var_a0_dc) - (assign44180_e49320 * locals.var_a0_dc_dn9)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44180_e49322);

        let assign44190_e49329: f64 = (1.0 + locals.var_r2init_op);
        let assign44190_e49330: f64 = (locals.var_k2_dc * assign44190_e49329);
        let assign44190_e49332: f64 = (assign44190_e49330 * locals.var_diff_min_dc);
        let assign44190_e49334: f64 = (assign44190_e49332 / locals.var_a0_dc);
        let assign44190_e49335: f64 = (assign44190_e49334).ln();
        let assign44190_e49337: f64 = (assign44190_e49335 + 2.0);
        locals.var_x2init_op = assign44190_e49337;
        locals.var_x2init_op_dn4 = ((((((((locals.var_k2_dc_dn4 * assign44190_e49329) + (locals.var_k2_dc * locals.var_r2init_op_dn4)) * locals.var_diff_min_dc) + (assign44190_e49330 * locals.var_diff_min_dc_dn4)) * locals.var_a0_dc) - (assign44190_e49332 * locals.var_a0_dc_dn4)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44190_e49334);
        locals.var_x2init_op_dn6 = ((((((((locals.var_k2_dc_dn6 * assign44190_e49329) + (locals.var_k2_dc * locals.var_r2init_op_dn6)) * locals.var_diff_min_dc) + (assign44190_e49330 * locals.var_diff_min_dc_dn6)) * locals.var_a0_dc) - (assign44190_e49332 * locals.var_a0_dc_dn6)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44190_e49334);
        locals.var_x2init_op_dn7 = ((((((((locals.var_k2_dc_dn7 * assign44190_e49329) + (locals.var_k2_dc * locals.var_r2init_op_dn7)) * locals.var_diff_min_dc) + (assign44190_e49330 * locals.var_diff_min_dc_dn7)) * locals.var_a0_dc) - (assign44190_e49332 * locals.var_a0_dc_dn7)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44190_e49334);
        locals.var_x2init_op_dn8 = ((((((((locals.var_k2_dc_dn8 * assign44190_e49329) + (locals.var_k2_dc * locals.var_r2init_op_dn8)) * locals.var_diff_min_dc) + (assign44190_e49330 * locals.var_diff_min_dc_dn8)) * locals.var_a0_dc) - (assign44190_e49332 * locals.var_a0_dc_dn8)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44190_e49334);
        locals.var_x2init_op_dn9 = ((((((((locals.var_k2_dc_dn9 * assign44190_e49329) + (locals.var_k2_dc * locals.var_r2init_op_dn9)) * locals.var_diff_min_dc) + (assign44190_e49330 * locals.var_diff_min_dc_dn9)) * locals.var_a0_dc) - (assign44190_e49332 * locals.var_a0_dc_dn9)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44190_e49334);

        let assign44200_e49340: f64 = (1.0 + locals.var_r1init_op);
        let assign44200_e49342: f64 = (assign44200_e49340 * locals.var_x1init_op);
        let assign44200_e49345: f64 = (locals.var_xg2x_dc * locals.var_r1init_op);
        let assign44200_e49346: f64 = (assign44200_e49342 - assign44200_e49345);
        locals.var_xth1init_op = assign44200_e49346;
        locals.var_xth1init_op_dn4 = (((locals.var_r1init_op_dn4 * locals.var_x1init_op) + (assign44200_e49340 * locals.var_x1init_op_dn4)) - ((locals.var_xg2x_dc_dn4 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn4)));
        locals.var_xth1init_op_dn6 = (((locals.var_r1init_op_dn6 * locals.var_x1init_op) + (assign44200_e49340 * locals.var_x1init_op_dn6)) - ((locals.var_xg2x_dc_dn6 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn6)));
        locals.var_xth1init_op_dn7 = (((locals.var_r1init_op_dn7 * locals.var_x1init_op) + (assign44200_e49340 * locals.var_x1init_op_dn7)) - ((locals.var_xg2x_dc_dn7 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn7)));
        locals.var_xth1init_op_dn8 = (((locals.var_r1init_op_dn8 * locals.var_x1init_op) + (assign44200_e49340 * locals.var_x1init_op_dn8)) - ((locals.var_xg2x_dc_dn8 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn8)));
        locals.var_xth1init_op_dn9 = (((locals.var_r1init_op_dn9 * locals.var_x1init_op) + (assign44200_e49340 * locals.var_x1init_op_dn9)) - ((locals.var_xg2x_dc_dn9 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn9)));

        let assign44210_e49350: f64 = (1.0 / locals.var_r2init_op);
        let assign44210_e49351: f64 = (1.0 + assign44210_e49350);
        let assign44210_e49353: f64 = (assign44210_e49351 * locals.var_x2init_op);
        let assign44210_e49356: f64 = (locals.var_xg2x_dc / locals.var_r2init_op);
        let assign44210_e49357: f64 = (assign44210_e49353 - assign44210_e49356);
        locals.var_xth2init_op = assign44210_e49357;
        locals.var_xth2init_op_dn4 = ((((-(locals.var_r2init_op_dn4 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44210_e49351 * locals.var_x2init_op_dn4)) - (((locals.var_xg2x_dc_dn4 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn4)) / (locals.var_r2init_op * locals.var_r2init_op)));
        locals.var_xth2init_op_dn6 = ((((-(locals.var_r2init_op_dn6 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44210_e49351 * locals.var_x2init_op_dn6)) - (((locals.var_xg2x_dc_dn6 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn6)) / (locals.var_r2init_op * locals.var_r2init_op)));
        locals.var_xth2init_op_dn7 = ((((-(locals.var_r2init_op_dn7 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44210_e49351 * locals.var_x2init_op_dn7)) - (((locals.var_xg2x_dc_dn7 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn7)) / (locals.var_r2init_op * locals.var_r2init_op)));
        locals.var_xth2init_op_dn8 = ((((-(locals.var_r2init_op_dn8 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44210_e49351 * locals.var_x2init_op_dn8)) - (((locals.var_xg2x_dc_dn8 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn8)) / (locals.var_r2init_op * locals.var_r2init_op)));
        locals.var_xth2init_op_dn9 = ((((-(locals.var_r2init_op_dn9 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44210_e49351 * locals.var_x2init_op_dn9)) - (((locals.var_xg2x_dc_dn9 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn9)) / (locals.var_r2init_op * locals.var_r2init_op)));

        let assign44220_e49361: f64 = (locals.var_xth1init_op + locals.var_xth2init_op);
        let assign44220_e49364: f64 = (locals.var_xth1init_op - locals.var_xth2init_op);
        let assign44220_e49367: f64 = (locals.var_xth1init_op - locals.var_xth2init_op);
        let assign44220_e49368: f64 = (assign44220_e49364 * assign44220_e49367);
        let assign44220_e49370: f64 = (assign44220_e49368 + 38.0);
        let assign44220_e49371: f64 = (assign44220_e49370).sqrt();
        let assign44220_e49372: f64 = (assign44220_e49361 - assign44220_e49371);
        let assign44220_e49373: f64 = (0.5 * assign44220_e49372);
        let assign44220_e49375: f64 = (assign44220_e49373 - locals.var_xg2_dc);
        let assign44220_e49377: f64 = (assign44220_e49375 / locals.var_cic1_i);
        let assign44220_e49379: f64 = (assign44220_e49377 + locals.var_xg2_dc);
        locals.var_xg1thinit_op = assign44220_e49379;
        locals.var_xg1thinit_op_dn4 = ((((0.5 * ((locals.var_xth1init_op_dn4 + locals.var_xth2init_op_dn4) - ((((locals.var_xth1init_op_dn4 - locals.var_xth2init_op_dn4) * assign44220_e49367) + (assign44220_e49364 * (locals.var_xth1init_op_dn4 - locals.var_xth2init_op_dn4))) / (2.0 * assign44220_e49371)))) - locals.var_xg2_dc_dn4) / locals.var_cic1_i) + locals.var_xg2_dc_dn4);
        locals.var_xg1thinit_op_dn6 = ((((0.5 * ((locals.var_xth1init_op_dn6 + locals.var_xth2init_op_dn6) - ((((locals.var_xth1init_op_dn6 - locals.var_xth2init_op_dn6) * assign44220_e49367) + (assign44220_e49364 * (locals.var_xth1init_op_dn6 - locals.var_xth2init_op_dn6))) / (2.0 * assign44220_e49371)))) - locals.var_xg2_dc_dn6) / locals.var_cic1_i) + locals.var_xg2_dc_dn6);
        locals.var_xg1thinit_op_dn7 = ((((0.5 * ((locals.var_xth1init_op_dn7 + locals.var_xth2init_op_dn7) - ((((locals.var_xth1init_op_dn7 - locals.var_xth2init_op_dn7) * assign44220_e49367) + (assign44220_e49364 * (locals.var_xth1init_op_dn7 - locals.var_xth2init_op_dn7))) / (2.0 * assign44220_e49371)))) - locals.var_xg2_dc_dn7) / locals.var_cic1_i) + locals.var_xg2_dc_dn7);
        locals.var_xg1thinit_op_dn8 = ((((0.5 * ((locals.var_xth1init_op_dn8 + locals.var_xth2init_op_dn8) - ((((locals.var_xth1init_op_dn8 - locals.var_xth2init_op_dn8) * assign44220_e49367) + (assign44220_e49364 * (locals.var_xth1init_op_dn8 - locals.var_xth2init_op_dn8))) / (2.0 * assign44220_e49371)))) - locals.var_xg2_dc_dn8) / locals.var_cic1_i) + locals.var_xg2_dc_dn8);
        locals.var_xg1thinit_op_dn9 = ((((0.5 * ((locals.var_xth1init_op_dn9 + locals.var_xth2init_op_dn9) - ((((locals.var_xth1init_op_dn9 - locals.var_xth2init_op_dn9) * assign44220_e49367) + (assign44220_e49364 * (locals.var_xth1init_op_dn9 - locals.var_xth2init_op_dn9))) / (2.0 * assign44220_e49371)))) - locals.var_xg2_dc_dn9) / locals.var_cic1_i) + locals.var_xg2_dc_dn9);

        let assign44230_e49383: f64 = (locals.var_xg1thinit_op - locals.var_xedge_dc);
        let assign44230_e49385: f64 = (assign44230_e49383 / locals.var_sce1_dc);
        let assign44230_e49387: f64 = (assign44230_e49385 - locals.var_dxg1_dibl_dc);
        let assign44230_e49389: f64 = (assign44230_e49387 + locals.var_xedge_dc);
        let assign44230_e49390: f64 = (locals.var_phit * assign44230_e49389);
        let assign44230_e49392: f64 = (assign44230_e49390 + locals.var_vfb1_i);
        locals.var_vthinit_op = assign44230_e49392;
        locals.var_vthinit_op_dn4 = (((locals.var_phit_dn4 * assign44230_e49389) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn4 - locals.var_xedge_dc_dn4) * locals.var_sce1_dc) - (assign44230_e49383 * locals.var_sce1_dc_dn4)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn4) + locals.var_xedge_dc_dn4))) + locals.var_vfb1_i_dn4);
        locals.var_vthinit_op_dn6 = (((locals.var_phit_dn6 * assign44230_e49389) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn6 - locals.var_xedge_dc_dn6) * locals.var_sce1_dc) - (assign44230_e49383 * locals.var_sce1_dc_dn6)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn6) + locals.var_xedge_dc_dn6))) + locals.var_vfb1_i_dn6);
        locals.var_vthinit_op_dn7 = (((locals.var_phit_dn7 * assign44230_e49389) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn7 - locals.var_xedge_dc_dn7) * locals.var_sce1_dc) - (assign44230_e49383 * locals.var_sce1_dc_dn7)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn7) + locals.var_xedge_dc_dn7))) + locals.var_vfb1_i_dn7);
        locals.var_vthinit_op_dn8 = (((locals.var_phit_dn8 * assign44230_e49389) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn8 - locals.var_xedge_dc_dn8) * locals.var_sce1_dc) - (assign44230_e49383 * locals.var_sce1_dc_dn8)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn8) + locals.var_xedge_dc_dn8))) + locals.var_vfb1_i_dn8);
        locals.var_vthinit_op_dn9 = (((locals.var_phit_dn9 * assign44230_e49389) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn9 - locals.var_xedge_dc_dn9) * locals.var_sce1_dc) - (assign44230_e49383 * locals.var_sce1_dc_dn9)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn9) + locals.var_xedge_dc_dn9))) + locals.var_vfb1_i_dn9);

        let assign44240_e49396: f64 = (locals.var_tkd - locals.var_tkr);
        let assign44240_e49397: f64 = (locals.var_stcf_i * assign44240_e49396);
        locals.var_temp = assign44240_e49397;
        locals.var_temp_dn4 = ((locals.var_stcf_i_dn4 * assign44240_e49396) + (locals.var_stcf_i * locals.var_tkd_dn4));
        locals.var_temp_dn6 = ((locals.var_stcf_i_dn6 * assign44240_e49396) + (locals.var_stcf_i * locals.var_tkd_dn6));
        locals.var_temp_dn7 = ((locals.var_stcf_i_dn7 * assign44240_e49396) + (locals.var_stcf_i * locals.var_tkd_dn7));
        locals.var_temp_dn8 = ((locals.var_stcf_i_dn8 * assign44240_e49396) + (locals.var_stcf_i * locals.var_tkd_dn8));
        locals.var_temp_dn9 = ((locals.var_stcf_i_dn9 * assign44240_e49396) + (locals.var_stcf_i * locals.var_tkd_dn9));

        let assign44270_e49406: f64 = (p.p14 * locals.var_stvfb_i);
        let assign44270_e49409: f64 = (locals.var_tkd - locals.var_tkr);
        let assign44270_e49410: f64 = (assign44270_e49406 * assign44270_e49409);
        let assign44270_e49412: f64 = (assign44270_e49410 + locals.var_dvfbqm);
        locals.var_temp = assign44270_e49412;
        locals.var_temp_dn4 = (assign44270_e49406 * locals.var_tkd_dn4);
        locals.var_temp_dn6 = (assign44270_e49406 * locals.var_tkd_dn6);
        locals.var_temp_dn7 = (assign44270_e49406 * locals.var_tkd_dn7);
        locals.var_temp_dn8 = (assign44270_e49406 * locals.var_tkd_dn8);
        locals.var_temp_dn9 = (assign44270_e49406 * locals.var_tkd_dn9);

        let assign44280_e49416: f64 = (locals.var_vfb1_t + locals.var_dvfbch_op);
        let assign44280_e49418: f64 = (assign44280_e49416 + locals.var_dvfb1nch);
        let assign44280_e49419: f64 = (p.p14 * assign44280_e49418);
        let assign44280_e49421: f64 = (assign44280_e49419 + locals.var_temp);
        let assign44280_e49423: f64 = (assign44280_e49421 + p.p34);
        let assign44280_e49425: f64 = (assign44280_e49423 - locals.var_dvfbpdep_op);
        locals.var_vfb1_op = assign44280_e49425;
        locals.var_vfb1_op_dn4 = (((p.p14 * ((locals.var_vfb1_t_dn4 + locals.var_dvfbch_op_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp_dn4) - locals.var_dvfbpdep_op_dn4);
        locals.var_vfb1_op_dn6 = (((p.p14 * ((locals.var_vfb1_t_dn6 + locals.var_dvfbch_op_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp_dn6) - locals.var_dvfbpdep_op_dn6);
        locals.var_vfb1_op_dn7 = (((p.p14 * ((locals.var_vfb1_t_dn7 + locals.var_dvfbch_op_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp_dn7) - locals.var_dvfbpdep_op_dn7);
        locals.var_vfb1_op_dn8 = (((p.p14 * ((locals.var_vfb1_t_dn8 + locals.var_dvfbch_op_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp_dn8) - locals.var_dvfbpdep_op_dn8);
        locals.var_vfb1_op_dn9 = (((p.p14 * ((locals.var_vfb1_t_dn9 + locals.var_dvfbch_op_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp_dn9) - locals.var_dvfbpdep_op_dn9);

        let assign44290_e49429: f64 = (locals.var_vfb2_t + locals.var_dvfbch_op);
        let assign44290_e49431: f64 = (assign44290_e49429 + locals.var_dvfb2nch);
        let assign44290_e49432: f64 = (p.p14 * assign44290_e49431);
        let assign44290_e49434: f64 = (assign44290_e49432 + locals.var_temp);
        locals.var_vfb2_op = assign44290_e49434;
        locals.var_vfb2_op_dn4 = ((p.p14 * ((locals.var_vfb2_t_dn4 + locals.var_dvfbch_op_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4);
        locals.var_vfb2_op_dn6 = ((p.p14 * ((locals.var_vfb2_t_dn6 + locals.var_dvfbch_op_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6);
        locals.var_vfb2_op_dn7 = ((p.p14 * ((locals.var_vfb2_t_dn7 + locals.var_dvfbch_op_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7);
        locals.var_vfb2_op_dn8 = ((p.p14 * ((locals.var_vfb2_t_dn8 + locals.var_dvfbch_op_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8);
        locals.var_vfb2_op_dn9 = ((p.p14 * ((locals.var_vfb2_t_dn9 + locals.var_dvfbch_op_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9);

        let assign44300_e49437: f64 = (locals.var_vthinit_op - locals.var_vfb1_op);
        let assign44300_e49439: f64 = (assign44300_e49437 * locals.var_inv_phit_op);
        let assign44300_e49441: f64 = (assign44300_e49439 - locals.var_dxdsx_op);
        locals.var_xg10_op = assign44300_e49441;
        locals.var_xg10_op_dn4 = ((((locals.var_vthinit_op_dn4 - locals.var_vfb1_op_dn4) * locals.var_inv_phit_op) + (assign44300_e49437 * locals.var_inv_phit_op_dn4)) - locals.var_dxdsx_op_dn4);
        locals.var_xg10_op_dn6 = ((((locals.var_vthinit_op_dn6 - locals.var_vfb1_op_dn6) * locals.var_inv_phit_op) + (assign44300_e49437 * locals.var_inv_phit_op_dn6)) - locals.var_dxdsx_op_dn6);
        locals.var_xg10_op_dn7 = ((((locals.var_vthinit_op_dn7 - locals.var_vfb1_op_dn7) * locals.var_inv_phit_op) + (assign44300_e49437 * locals.var_inv_phit_op_dn7)) - locals.var_dxdsx_op_dn7);
        locals.var_xg10_op_dn8 = ((((locals.var_vthinit_op_dn8 - locals.var_vfb1_op_dn8) * locals.var_inv_phit_op) + (assign44300_e49437 * locals.var_inv_phit_op_dn8)) - locals.var_dxdsx_op_dn8);
        locals.var_xg10_op_dn9 = ((((locals.var_vthinit_op_dn9 - locals.var_vfb1_op_dn9) * locals.var_inv_phit_op) + (assign44300_e49437 * locals.var_inv_phit_op_dn9)) - locals.var_dxdsx_op_dn9);

        let assign44310_e49443: f64 = (-locals.var_vsb);
        let assign44310_e49445: f64 = (assign44310_e49443 - locals.var_vfb2_op);
        let assign44310_e49447: f64 = (assign44310_e49445 * locals.var_inv_phit_op);
        let assign44310_e49449: f64 = (assign44310_e49447 - locals.var_dxdsx_op);
        locals.var_xg20_op = assign44310_e49449;
        locals.var_xg20_op_dn4 = ((((-locals.var_vfb2_op_dn4) * locals.var_inv_phit_op) + (assign44310_e49445 * locals.var_inv_phit_op_dn4)) - locals.var_dxdsx_op_dn4);
        locals.var_xg20_op_dn6 = (((((-locals.var_vsb_dn6) - locals.var_vfb2_op_dn6) * locals.var_inv_phit_op) + (assign44310_e49445 * locals.var_inv_phit_op_dn6)) - locals.var_dxdsx_op_dn6);
        locals.var_xg20_op_dn7 = (((((-locals.var_vsb_dn7) - locals.var_vfb2_op_dn7) * locals.var_inv_phit_op) + (assign44310_e49445 * locals.var_inv_phit_op_dn7)) - locals.var_dxdsx_op_dn7);
        locals.var_xg20_op_dn8 = (((((-locals.var_vsb_dn8) - locals.var_vfb2_op_dn8) * locals.var_inv_phit_op) + (assign44310_e49445 * locals.var_inv_phit_op_dn8)) - locals.var_dxdsx_op_dn8);
        locals.var_xg20_op_dn9 = ((((-locals.var_vfb2_op_dn9) * locals.var_inv_phit_op) + (assign44310_e49445 * locals.var_inv_phit_op_dn9)) - locals.var_dxdsx_op_dn9);

        let assign44320_e49452: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1359 = assign44320_e49452;

        let (assign44330_e49464, assign44330_e49464_d_n4, assign44330_e49464_d_n6, assign44330_e49464_d_n7, assign44330_e49464_d_n8, assign44330_e49464_d_n9,) = {
    if (locals.var_guard1359 != 0.0) {
        let assign44330_e49456: f64 = (p.p14 * locals.var_typesub_i);
        let assign44330_e49459: f64 = (locals.var_xg10_op - locals.var_xg20_op);
        let assign44330_e49460: f64 = (assign44330_e49456 * assign44330_e49459);
        let assign44330_e49462: f64 = (assign44330_e49460 / locals.var_gfsub);
        (assign44330_e49462, ((((assign44330_e49456 * (locals.var_xg10_op_dn4 - locals.var_xg20_op_dn4)) * locals.var_gfsub) - (assign44330_e49460 * locals.var_gfsub_dn4)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44330_e49456 * (locals.var_xg10_op_dn6 - locals.var_xg20_op_dn6)) * locals.var_gfsub) - (assign44330_e49460 * locals.var_gfsub_dn6)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44330_e49456 * (locals.var_xg10_op_dn7 - locals.var_xg20_op_dn7)) * locals.var_gfsub) - (assign44330_e49460 * locals.var_gfsub_dn7)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44330_e49456 * (locals.var_xg10_op_dn8 - locals.var_xg20_op_dn8)) * locals.var_gfsub) - (assign44330_e49460 * locals.var_gfsub_dn8)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44330_e49456 * (locals.var_xg10_op_dn9 - locals.var_xg20_op_dn9)) * locals.var_gfsub) - (assign44330_e49460 * locals.var_gfsub_dn9)) / (locals.var_gfsub * locals.var_gfsub)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign44330_e49464;
        locals.var_temp_dn4 = assign44330_e49464_d_n4;
        locals.var_temp_dn6 = assign44330_e49464_d_n6;
        locals.var_temp_dn7 = assign44330_e49464_d_n7;
        locals.var_temp_dn8 = assign44330_e49464_d_n8;
        locals.var_temp_dn9 = assign44330_e49464_d_n9;

        let assign44340_e49467: f64 = if locals.var_temp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1360 = assign44340_e49467;

        let (assign44350_e49479, assign44350_e49479_d_n4, assign44350_e49479_d_n6, assign44350_e49479_d_n7, assign44350_e49479_d_n8, assign44350_e49479_d_n9,) = {
    if ((locals.var_guard1359 != 0.0) && (locals.var_guard1360 != 0.0)) {
        let assign44350_e49472: f64 = (-2.0);
        let assign44350_e49475: f64 = (1.0 - locals.var_temp);
        let assign44350_e49476: f64 = (assign44350_e49475).ln();
        let assign44350_e49477: f64 = (assign44350_e49472 * assign44350_e49476);
        (assign44350_e49477, (assign44350_e49472 * ((-locals.var_temp_dn4) / assign44350_e49475)), (assign44350_e49472 * ((-locals.var_temp_dn6) / assign44350_e49475)), (assign44350_e49472 * ((-locals.var_temp_dn7) / assign44350_e49475)), (assign44350_e49472 * ((-locals.var_temp_dn8) / assign44350_e49475)), (assign44350_e49472 * ((-locals.var_temp_dn9) / assign44350_e49475)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign44350_e49479;
        locals.var_temp1_dn4 = assign44350_e49479_d_n4;
        locals.var_temp1_dn6 = assign44350_e49479_d_n6;
        locals.var_temp1_dn7 = assign44350_e49479_d_n7;
        locals.var_temp1_dn8 = assign44350_e49479_d_n8;
        locals.var_temp1_dn9 = assign44350_e49479_d_n9;

    }

    pub(super) fn stamp_transient_block_122(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign44360_e49496, assign44360_e49496_d_n4, assign44360_e49496_d_n6, assign44360_e49496_d_n7, assign44360_e49496_d_n8, assign44360_e49496_d_n9,) = {
    if ((locals.var_guard1359 != 0.0) && (locals.var_guard1360 == 0.0)) {
        let assign44360_e49486: f64 = (locals.var_temp * locals.var_temp);
        let assign44360_e49490: f64 = (2.0 * locals.var_temp);
        let assign44360_e49492: f64 = (assign44360_e49490 / locals.var_gfsub);
        let assign44360_e49493: f64 = (1.0 + assign44360_e49492);
        let assign44360_e49494: f64 = (assign44360_e49486 / assign44360_e49493);
        (assign44360_e49494, (((((locals.var_temp_dn4 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn4)) * assign44360_e49493) - (assign44360_e49486 * ((((2.0 * locals.var_temp_dn4) * locals.var_gfsub) - (assign44360_e49490 * locals.var_gfsub_dn4)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44360_e49493 * assign44360_e49493)), (((((locals.var_temp_dn6 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn6)) * assign44360_e49493) - (assign44360_e49486 * ((((2.0 * locals.var_temp_dn6) * locals.var_gfsub) - (assign44360_e49490 * locals.var_gfsub_dn6)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44360_e49493 * assign44360_e49493)), (((((locals.var_temp_dn7 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn7)) * assign44360_e49493) - (assign44360_e49486 * ((((2.0 * locals.var_temp_dn7) * locals.var_gfsub) - (assign44360_e49490 * locals.var_gfsub_dn7)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44360_e49493 * assign44360_e49493)), (((((locals.var_temp_dn8 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn8)) * assign44360_e49493) - (assign44360_e49486 * ((((2.0 * locals.var_temp_dn8) * locals.var_gfsub) - (assign44360_e49490 * locals.var_gfsub_dn8)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44360_e49493 * assign44360_e49493)), (((((locals.var_temp_dn9 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn9)) * assign44360_e49493) - (assign44360_e49486 * ((((2.0 * locals.var_temp_dn9) * locals.var_gfsub) - (assign44360_e49490 * locals.var_gfsub_dn9)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44360_e49493 * assign44360_e49493)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign44360_e49496;
        locals.var_temp1_dn4 = assign44360_e49496_d_n4;
        locals.var_temp1_dn6 = assign44360_e49496_d_n6;
        locals.var_temp1_dn7 = assign44360_e49496_d_n7;
        locals.var_temp1_dn8 = assign44360_e49496_d_n8;
        locals.var_temp1_dn9 = assign44360_e49496_d_n9;

        let (assign44370_e49506, assign44370_e49506_d_n4, assign44370_e49506_d_n6, assign44370_e49506_d_n7, assign44370_e49506_d_n8, assign44370_e49506_d_n9,) = {
    if (locals.var_guard1359 != 0.0) {
        let assign44370_e49501: f64 = (p.p14 * locals.var_typesub_i);
        let assign44370_e49503: f64 = (assign44370_e49501 * locals.var_temp1);
        let assign44370_e49504: f64 = (locals.var_xg20_op + assign44370_e49503);
        (assign44370_e49504, (locals.var_xg20_op_dn4 + (assign44370_e49501 * locals.var_temp1_dn4)), (locals.var_xg20_op_dn6 + (assign44370_e49501 * locals.var_temp1_dn6)), (locals.var_xg20_op_dn7 + (assign44370_e49501 * locals.var_temp1_dn7)), (locals.var_xg20_op_dn8 + (assign44370_e49501 * locals.var_temp1_dn8)), (locals.var_xg20_op_dn9 + (assign44370_e49501 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_xg2eff_op, locals.var_xg2eff_op_dn4, locals.var_xg2eff_op_dn6, locals.var_xg2eff_op_dn7, locals.var_xg2eff_op_dn8, locals.var_xg2eff_op_dn9,)
    }
};
        locals.var_xg2eff_op = assign44370_e49506;
        locals.var_xg2eff_op_dn4 = assign44370_e49506_d_n4;
        locals.var_xg2eff_op_dn6 = assign44370_e49506_d_n6;
        locals.var_xg2eff_op_dn7 = assign44370_e49506_d_n7;
        locals.var_xg2eff_op_dn8 = assign44370_e49506_d_n8;
        locals.var_xg2eff_op_dn9 = assign44370_e49506_d_n9;

        let (assign44380_e49511, assign44380_e49511_d_n4, assign44380_e49511_d_n6, assign44380_e49511_d_n7, assign44380_e49511_d_n8, assign44380_e49511_d_n9,) = {
    if (locals.var_guard1359 == 0.0) {
        (locals.var_xg20_op, locals.var_xg20_op_dn4, locals.var_xg20_op_dn6, locals.var_xg20_op_dn7, locals.var_xg20_op_dn8, locals.var_xg20_op_dn9,)
    } else {
        (locals.var_xg2eff_op, locals.var_xg2eff_op_dn4, locals.var_xg2eff_op_dn6, locals.var_xg2eff_op_dn7, locals.var_xg2eff_op_dn8, locals.var_xg2eff_op_dn9,)
    }
};
        locals.var_xg2eff_op = assign44380_e49511;
        locals.var_xg2eff_op_dn4 = assign44380_e49511_d_n4;
        locals.var_xg2eff_op_dn6 = assign44380_e49511_d_n6;
        locals.var_xg2eff_op_dn7 = assign44380_e49511_d_n7;
        locals.var_xg2eff_op_dn8 = assign44380_e49511_d_n8;
        locals.var_xg2eff_op_dn9 = assign44380_e49511_d_n9;

        let assign44390_e49515: f64 = (locals.var_xg10_op - locals.var_xg2eff_op);
        let assign44390_e49516: f64 = (locals.var_keq_1d * assign44390_e49515);
        locals.var_temp = assign44390_e49516;
        locals.var_temp_dn4 = (locals.var_keq_1d * (locals.var_xg10_op_dn4 - locals.var_xg2eff_op_dn4));
        locals.var_temp_dn6 = (locals.var_keq_1d * (locals.var_xg10_op_dn6 - locals.var_xg2eff_op_dn6));
        locals.var_temp_dn7 = (locals.var_keq_1d * (locals.var_xg10_op_dn7 - locals.var_xg2eff_op_dn7));
        locals.var_temp_dn8 = (locals.var_keq_1d * (locals.var_xg10_op_dn8 - locals.var_xg2eff_op_dn8));
        locals.var_temp_dn9 = (locals.var_keq_1d * (locals.var_xg10_op_dn9 - locals.var_xg2eff_op_dn9));

        let assign44400_e49519: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1361 = assign44400_e49519;

        let (assign44410_e49540, assign44410_e49540_d_n4, assign44410_e49540_d_n6, assign44410_e49540_d_n7, assign44410_e49540_d_n8, assign44410_e49540_d_n9,) = {
    if (locals.var_guard1361 != 0.0) {
        let assign44410_e49524: f64 = (locals.var_temp + locals.var_emin);
        let assign44410_e49527: f64 = (locals.var_temp - locals.var_emin);
        let assign44410_e49530: f64 = (locals.var_temp - locals.var_emin);
        let assign44410_e49531: f64 = (assign44410_e49527 * assign44410_e49530);
        let assign44410_e49534: f64 = (locals.var_emin * locals.var_emin);
        let assign44410_e49535: f64 = (assign44410_e49531 + assign44410_e49534);
        let assign44410_e49536: f64 = (assign44410_e49535).sqrt();
        let assign44410_e49537: f64 = (assign44410_e49524 + assign44410_e49536);
        let assign44410_e49538: f64 = (0.5 * assign44410_e49537);
        (assign44410_e49538, (0.5 * ((locals.var_temp_dn4 + locals.var_emin_dn4) + (((((locals.var_temp_dn4 - locals.var_emin_dn4) * assign44410_e49530) + (assign44410_e49527 * (locals.var_temp_dn4 - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign44410_e49536)))), (0.5 * ((locals.var_temp_dn6 + locals.var_emin_dn6) + (((((locals.var_temp_dn6 - locals.var_emin_dn6) * assign44410_e49530) + (assign44410_e49527 * (locals.var_temp_dn6 - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign44410_e49536)))), (0.5 * ((locals.var_temp_dn7 + locals.var_emin_dn7) + (((((locals.var_temp_dn7 - locals.var_emin_dn7) * assign44410_e49530) + (assign44410_e49527 * (locals.var_temp_dn7 - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign44410_e49536)))), (0.5 * ((locals.var_temp_dn8 + locals.var_emin_dn8) + (((((locals.var_temp_dn8 - locals.var_emin_dn8) * assign44410_e49530) + (assign44410_e49527 * (locals.var_temp_dn8 - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign44410_e49536)))), (0.5 * ((locals.var_temp_dn9 + locals.var_emin_dn9) + (((((locals.var_temp_dn9 - locals.var_emin_dn9) * assign44410_e49530) + (assign44410_e49527 * (locals.var_temp_dn9 - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign44410_e49536)))),)
    } else {
        (locals.var_e1_op, locals.var_e1_op_dn4, locals.var_e1_op_dn6, locals.var_e1_op_dn7, locals.var_e1_op_dn8, locals.var_e1_op_dn9,)
    }
};
        locals.var_e1_op = assign44410_e49540;
        locals.var_e1_op_dn4 = assign44410_e49540_d_n4;
        locals.var_e1_op_dn6 = assign44410_e49540_d_n6;
        locals.var_e1_op_dn7 = assign44410_e49540_d_n7;
        locals.var_e1_op_dn8 = assign44410_e49540_d_n8;
        locals.var_e1_op_dn9 = assign44410_e49540_d_n9;

        let (assign44420_e49564, assign44420_e49564_d_n4, assign44420_e49564_d_n6, assign44420_e49564_d_n7, assign44420_e49564_d_n8, assign44420_e49564_d_n9,) = {
    if (locals.var_guard1361 != 0.0) {
        let assign44420_e49544: f64 = (-locals.var_temp);
        let assign44420_e49546: f64 = (assign44420_e49544 + locals.var_emin);
        let assign44420_e49548: f64 = (-locals.var_temp);
        let assign44420_e49550: f64 = (assign44420_e49548 - locals.var_emin);
        let assign44420_e49552: f64 = (-locals.var_temp);
        let assign44420_e49554: f64 = (assign44420_e49552 - locals.var_emin);
        let assign44420_e49555: f64 = (assign44420_e49550 * assign44420_e49554);
        let assign44420_e49558: f64 = (locals.var_emin * locals.var_emin);
        let assign44420_e49559: f64 = (assign44420_e49555 + assign44420_e49558);
        let assign44420_e49560: f64 = (assign44420_e49559).sqrt();
        let assign44420_e49561: f64 = (assign44420_e49546 + assign44420_e49560);
        let assign44420_e49562: f64 = (0.5 * assign44420_e49561);
        (assign44420_e49562, (0.5 * (((-locals.var_temp_dn4) + locals.var_emin_dn4) + ((((((-locals.var_temp_dn4) - locals.var_emin_dn4) * assign44420_e49554) + (assign44420_e49550 * ((-locals.var_temp_dn4) - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign44420_e49560)))), (0.5 * (((-locals.var_temp_dn6) + locals.var_emin_dn6) + ((((((-locals.var_temp_dn6) - locals.var_emin_dn6) * assign44420_e49554) + (assign44420_e49550 * ((-locals.var_temp_dn6) - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign44420_e49560)))), (0.5 * (((-locals.var_temp_dn7) + locals.var_emin_dn7) + ((((((-locals.var_temp_dn7) - locals.var_emin_dn7) * assign44420_e49554) + (assign44420_e49550 * ((-locals.var_temp_dn7) - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign44420_e49560)))), (0.5 * (((-locals.var_temp_dn8) + locals.var_emin_dn8) + ((((((-locals.var_temp_dn8) - locals.var_emin_dn8) * assign44420_e49554) + (assign44420_e49550 * ((-locals.var_temp_dn8) - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign44420_e49560)))), (0.5 * (((-locals.var_temp_dn9) + locals.var_emin_dn9) + ((((((-locals.var_temp_dn9) - locals.var_emin_dn9) * assign44420_e49554) + (assign44420_e49550 * ((-locals.var_temp_dn9) - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign44420_e49560)))),)
    } else {
        (locals.var_e2_op, locals.var_e2_op_dn4, locals.var_e2_op_dn6, locals.var_e2_op_dn7, locals.var_e2_op_dn8, locals.var_e2_op_dn9,)
    }
};
        locals.var_e2_op = assign44420_e49564;
        locals.var_e2_op_dn4 = assign44420_e49564_d_n4;
        locals.var_e2_op_dn6 = assign44420_e49564_d_n6;
        locals.var_e2_op_dn7 = assign44420_e49564_d_n7;
        locals.var_e2_op_dn8 = assign44420_e49564_d_n8;
        locals.var_e2_op_dn9 = assign44420_e49564_d_n9;

        let (assign44430_e49575, assign44430_e49575_d_n4, assign44430_e49575_d_n6, assign44430_e49575_d_n7, assign44430_e49575_d_n8, assign44430_e49575_d_n9,) = {
    if (locals.var_guard1361 != 0.0) {
        let assign44430_e49568: f64 = (-0.3333333333333);
        let assign44430_e49570: f64 = (locals.var_e1_op).ln();
        let assign44430_e49571: f64 = (assign44430_e49568 * assign44430_e49570);
        let assign44430_e49572: f64 = (assign44430_e49571).exp();
        let assign44430_e49573: f64 = (locals.var_qq_op * assign44430_e49572);
        (assign44430_e49573, ((locals.var_qq_op_dn4 * assign44430_e49572) + (locals.var_qq_op * (assign44430_e49572 * (assign44430_e49568 * (locals.var_e1_op_dn4 / locals.var_e1_op))))), ((locals.var_qq_op_dn6 * assign44430_e49572) + (locals.var_qq_op * (assign44430_e49572 * (assign44430_e49568 * (locals.var_e1_op_dn6 / locals.var_e1_op))))), ((locals.var_qq_op_dn7 * assign44430_e49572) + (locals.var_qq_op * (assign44430_e49572 * (assign44430_e49568 * (locals.var_e1_op_dn7 / locals.var_e1_op))))), ((locals.var_qq_op_dn8 * assign44430_e49572) + (locals.var_qq_op * (assign44430_e49572 * (assign44430_e49568 * (locals.var_e1_op_dn8 / locals.var_e1_op))))), ((locals.var_qq_op_dn9 * assign44430_e49572) + (locals.var_qq_op * (assign44430_e49572 * (assign44430_e49568 * (locals.var_e1_op_dn9 / locals.var_e1_op))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign44430_e49575;
        locals.var_temp1_dn4 = assign44430_e49575_d_n4;
        locals.var_temp1_dn6 = assign44430_e49575_d_n6;
        locals.var_temp1_dn7 = assign44430_e49575_d_n7;
        locals.var_temp1_dn8 = assign44430_e49575_d_n8;
        locals.var_temp1_dn9 = assign44430_e49575_d_n9;

        let (assign44440_e49586, assign44440_e49586_d_n4, assign44440_e49586_d_n6, assign44440_e49586_d_n7, assign44440_e49586_d_n8, assign44440_e49586_d_n9,) = {
    if (locals.var_guard1361 != 0.0) {
        let assign44440_e49579: f64 = (-0.3333333333333);
        let assign44440_e49581: f64 = (locals.var_e2_op).ln();
        let assign44440_e49582: f64 = (assign44440_e49579 * assign44440_e49581);
        let assign44440_e49583: f64 = (assign44440_e49582).exp();
        let assign44440_e49584: f64 = (locals.var_qq_op * assign44440_e49583);
        (assign44440_e49584, ((locals.var_qq_op_dn4 * assign44440_e49583) + (locals.var_qq_op * (assign44440_e49583 * (assign44440_e49579 * (locals.var_e2_op_dn4 / locals.var_e2_op))))), ((locals.var_qq_op_dn6 * assign44440_e49583) + (locals.var_qq_op * (assign44440_e49583 * (assign44440_e49579 * (locals.var_e2_op_dn6 / locals.var_e2_op))))), ((locals.var_qq_op_dn7 * assign44440_e49583) + (locals.var_qq_op * (assign44440_e49583 * (assign44440_e49579 * (locals.var_e2_op_dn7 / locals.var_e2_op))))), ((locals.var_qq_op_dn8 * assign44440_e49583) + (locals.var_qq_op * (assign44440_e49583 * (assign44440_e49579 * (locals.var_e2_op_dn8 / locals.var_e2_op))))), ((locals.var_qq_op_dn9 * assign44440_e49583) + (locals.var_qq_op * (assign44440_e49583 * (assign44440_e49579 * (locals.var_e2_op_dn9 / locals.var_e2_op))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign44440_e49586;
        locals.var_temp2_dn4 = assign44440_e49586_d_n4;
        locals.var_temp2_dn6 = assign44440_e49586_d_n6;
        locals.var_temp2_dn7 = assign44440_e49586_d_n7;
        locals.var_temp2_dn8 = assign44440_e49586_d_n8;
        locals.var_temp2_dn9 = assign44440_e49586_d_n9;

        let (assign44450_e49594, assign44450_e49594_d_n4, assign44450_e49594_d_n6, assign44450_e49594_d_n7, assign44450_e49594_d_n8, assign44450_e49594_d_n9,) = {
    if (locals.var_guard1361 != 0.0) {
        let assign44450_e49590: f64 = (1.0 - locals.var_temp1);
        let assign44450_e49592: f64 = (assign44450_e49590 - locals.var_temp2);
        (assign44450_e49592, ((-locals.var_temp1_dn4) - locals.var_temp2_dn4), ((-locals.var_temp1_dn6) - locals.var_temp2_dn6), ((-locals.var_temp1_dn7) - locals.var_temp2_dn7), ((-locals.var_temp1_dn8) - locals.var_temp2_dn8), ((-locals.var_temp1_dn9) - locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign44450_e49594;
        locals.var_temp3_dn4 = assign44450_e49594_d_n4;
        locals.var_temp3_dn6 = assign44450_e49594_d_n6;
        locals.var_temp3_dn7 = assign44450_e49594_d_n7;
        locals.var_temp3_dn8 = assign44450_e49594_d_n8;
        locals.var_temp3_dn9 = assign44450_e49594_d_n9;

        let (assign44470_e49612, assign44470_e49612_d_n4, assign44470_e49612_d_n6, assign44470_e49612_d_n7, assign44470_e49612_d_n8, assign44470_e49612_d_n9,) = {
    if (locals.var_guard1361 != 0.0) {
        let assign44470_e49604: f64 = (locals.var_k1_1d * locals.var_temp3);
        let assign44470_e49608: f64 = (locals.var_k1_1d * locals.var_temp1);
        let assign44470_e49609: f64 = (1.0 + assign44470_e49608);
        let assign44470_e49610: f64 = (assign44470_e49604 / assign44470_e49609);
        (assign44470_e49610, ((((locals.var_k1_1d * locals.var_temp3_dn4) * assign44470_e49609) - (assign44470_e49604 * (locals.var_k1_1d * locals.var_temp1_dn4))) / (assign44470_e49609 * assign44470_e49609)), ((((locals.var_k1_1d * locals.var_temp3_dn6) * assign44470_e49609) - (assign44470_e49604 * (locals.var_k1_1d * locals.var_temp1_dn6))) / (assign44470_e49609 * assign44470_e49609)), ((((locals.var_k1_1d * locals.var_temp3_dn7) * assign44470_e49609) - (assign44470_e49604 * (locals.var_k1_1d * locals.var_temp1_dn7))) / (assign44470_e49609 * assign44470_e49609)), ((((locals.var_k1_1d * locals.var_temp3_dn8) * assign44470_e49609) - (assign44470_e49604 * (locals.var_k1_1d * locals.var_temp1_dn8))) / (assign44470_e49609 * assign44470_e49609)), ((((locals.var_k1_1d * locals.var_temp3_dn9) * assign44470_e49609) - (assign44470_e49604 * (locals.var_k1_1d * locals.var_temp1_dn9))) / (assign44470_e49609 * assign44470_e49609)),)
    } else {
        (locals.var_k1_1d_qm_op, locals.var_k1_1d_qm_op_dn4, locals.var_k1_1d_qm_op_dn6, locals.var_k1_1d_qm_op_dn7, locals.var_k1_1d_qm_op_dn8, locals.var_k1_1d_qm_op_dn9,)
    }
};
        locals.var_k1_1d_qm_op = assign44470_e49612;
        locals.var_k1_1d_qm_op_dn4 = assign44470_e49612_d_n4;
        locals.var_k1_1d_qm_op_dn6 = assign44470_e49612_d_n6;
        locals.var_k1_1d_qm_op_dn7 = assign44470_e49612_d_n7;
        locals.var_k1_1d_qm_op_dn8 = assign44470_e49612_d_n8;
        locals.var_k1_1d_qm_op_dn9 = assign44470_e49612_d_n9;

        let (assign44480_e49624, assign44480_e49624_d_n4, assign44480_e49624_d_n6, assign44480_e49624_d_n7, assign44480_e49624_d_n8, assign44480_e49624_d_n9,) = {
    if (locals.var_guard1361 != 0.0) {
        let assign44480_e49616: f64 = (locals.var_k2_1d * locals.var_temp3);
        let assign44480_e49620: f64 = (locals.var_k2_1d * locals.var_temp2);
        let assign44480_e49621: f64 = (1.0 + assign44480_e49620);
        let assign44480_e49622: f64 = (assign44480_e49616 / assign44480_e49621);
        (assign44480_e49622, ((((locals.var_k2_1d * locals.var_temp3_dn4) * assign44480_e49621) - (assign44480_e49616 * (locals.var_k2_1d * locals.var_temp2_dn4))) / (assign44480_e49621 * assign44480_e49621)), ((((locals.var_k2_1d * locals.var_temp3_dn6) * assign44480_e49621) - (assign44480_e49616 * (locals.var_k2_1d * locals.var_temp2_dn6))) / (assign44480_e49621 * assign44480_e49621)), ((((locals.var_k2_1d * locals.var_temp3_dn7) * assign44480_e49621) - (assign44480_e49616 * (locals.var_k2_1d * locals.var_temp2_dn7))) / (assign44480_e49621 * assign44480_e49621)), ((((locals.var_k2_1d * locals.var_temp3_dn8) * assign44480_e49621) - (assign44480_e49616 * (locals.var_k2_1d * locals.var_temp2_dn8))) / (assign44480_e49621 * assign44480_e49621)), ((((locals.var_k2_1d * locals.var_temp3_dn9) * assign44480_e49621) - (assign44480_e49616 * (locals.var_k2_1d * locals.var_temp2_dn9))) / (assign44480_e49621 * assign44480_e49621)),)
    } else {
        (locals.var_k2_1d_qm_op, locals.var_k2_1d_qm_op_dn4, locals.var_k2_1d_qm_op_dn6, locals.var_k2_1d_qm_op_dn7, locals.var_k2_1d_qm_op_dn8, locals.var_k2_1d_qm_op_dn9,)
    }
};
        locals.var_k2_1d_qm_op = assign44480_e49624;
        locals.var_k2_1d_qm_op_dn4 = assign44480_e49624_d_n4;
        locals.var_k2_1d_qm_op_dn6 = assign44480_e49624_d_n6;
        locals.var_k2_1d_qm_op_dn7 = assign44480_e49624_d_n7;
        locals.var_k2_1d_qm_op_dn8 = assign44480_e49624_d_n8;
        locals.var_k2_1d_qm_op_dn9 = assign44480_e49624_d_n9;

        let (assign44490_e49638, assign44490_e49638_d_n4, assign44490_e49638_d_n6, assign44490_e49638_d_n7, assign44490_e49638_d_n8, assign44490_e49638_d_n9,) = {
    if (locals.var_guard1361 != 0.0) {
        let assign44490_e49630: f64 = (1.0 / locals.var_k1_1d_qm_op);
        let assign44490_e49631: f64 = (1.0 + assign44490_e49630);
        let assign44490_e49634: f64 = (1.0 / locals.var_k2_1d_qm_op);
        let assign44490_e49635: f64 = (assign44490_e49631 + assign44490_e49634);
        let assign44490_e49636: f64 = (1.0 / assign44490_e49635);
        (assign44490_e49636, (-(((-(locals.var_k1_1d_qm_op_dn4 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn4 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44490_e49635 * assign44490_e49635))), (-(((-(locals.var_k1_1d_qm_op_dn6 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn6 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44490_e49635 * assign44490_e49635))), (-(((-(locals.var_k1_1d_qm_op_dn7 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn7 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44490_e49635 * assign44490_e49635))), (-(((-(locals.var_k1_1d_qm_op_dn8 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn8 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44490_e49635 * assign44490_e49635))), (-(((-(locals.var_k1_1d_qm_op_dn9 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn9 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44490_e49635 * assign44490_e49635))),)
    } else {
        (locals.var_keq_1d_qm_op, locals.var_keq_1d_qm_op_dn4, locals.var_keq_1d_qm_op_dn6, locals.var_keq_1d_qm_op_dn7, locals.var_keq_1d_qm_op_dn8, locals.var_keq_1d_qm_op_dn9,)
    }
};
        locals.var_keq_1d_qm_op = assign44490_e49638;
        locals.var_keq_1d_qm_op_dn4 = assign44490_e49638_d_n4;
        locals.var_keq_1d_qm_op_dn6 = assign44490_e49638_d_n6;
        locals.var_keq_1d_qm_op_dn7 = assign44490_e49638_d_n7;
        locals.var_keq_1d_qm_op_dn8 = assign44490_e49638_d_n8;
        locals.var_keq_1d_qm_op_dn9 = assign44490_e49638_d_n9;

        let (assign44510_e49648, assign44510_e49648_d_n4, assign44510_e49648_d_n6, assign44510_e49648_d_n7, assign44510_e49648_d_n8, assign44510_e49648_d_n9,) = {
    if (locals.var_guard1361 == 0.0) {
        (locals.var_k1_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_k1_1d_qm_op, locals.var_k1_1d_qm_op_dn4, locals.var_k1_1d_qm_op_dn6, locals.var_k1_1d_qm_op_dn7, locals.var_k1_1d_qm_op_dn8, locals.var_k1_1d_qm_op_dn9,)
    }
};
        locals.var_k1_1d_qm_op = assign44510_e49648;
        locals.var_k1_1d_qm_op_dn4 = assign44510_e49648_d_n4;
        locals.var_k1_1d_qm_op_dn6 = assign44510_e49648_d_n6;
        locals.var_k1_1d_qm_op_dn7 = assign44510_e49648_d_n7;
        locals.var_k1_1d_qm_op_dn8 = assign44510_e49648_d_n8;
        locals.var_k1_1d_qm_op_dn9 = assign44510_e49648_d_n9;

        let (assign44520_e49653, assign44520_e49653_d_n4, assign44520_e49653_d_n6, assign44520_e49653_d_n7, assign44520_e49653_d_n8, assign44520_e49653_d_n9,) = {
    if (locals.var_guard1361 == 0.0) {
        (locals.var_k2_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_k2_1d_qm_op, locals.var_k2_1d_qm_op_dn4, locals.var_k2_1d_qm_op_dn6, locals.var_k2_1d_qm_op_dn7, locals.var_k2_1d_qm_op_dn8, locals.var_k2_1d_qm_op_dn9,)
    }
};
        locals.var_k2_1d_qm_op = assign44520_e49653;
        locals.var_k2_1d_qm_op_dn4 = assign44520_e49653_d_n4;
        locals.var_k2_1d_qm_op_dn6 = assign44520_e49653_d_n6;
        locals.var_k2_1d_qm_op_dn7 = assign44520_e49653_d_n7;
        locals.var_k2_1d_qm_op_dn8 = assign44520_e49653_d_n8;
        locals.var_k2_1d_qm_op_dn9 = assign44520_e49653_d_n9;

        let (assign44530_e49658, assign44530_e49658_d_n4, assign44530_e49658_d_n6, assign44530_e49658_d_n7, assign44530_e49658_d_n8, assign44530_e49658_d_n9,) = {
    if (locals.var_guard1361 == 0.0) {
        (locals.var_keq_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_keq_1d_qm_op, locals.var_keq_1d_qm_op_dn4, locals.var_keq_1d_qm_op_dn6, locals.var_keq_1d_qm_op_dn7, locals.var_keq_1d_qm_op_dn8, locals.var_keq_1d_qm_op_dn9,)
    }
};
        locals.var_keq_1d_qm_op = assign44530_e49658;
        locals.var_keq_1d_qm_op_dn4 = assign44530_e49658_d_n4;
        locals.var_keq_1d_qm_op_dn6 = assign44530_e49658_d_n6;
        locals.var_keq_1d_qm_op_dn7 = assign44530_e49658_d_n7;
        locals.var_keq_1d_qm_op_dn8 = assign44530_e49658_d_n8;
        locals.var_keq_1d_qm_op_dn9 = assign44530_e49658_d_n9;

        let assign44540_e49662: f64 = (locals.var_xg10_op - locals.var_xg2eff_op);
        let assign44540_e49663: f64 = (locals.var_keq_1d_qm_op * assign44540_e49662);
        locals.var_dx_wi_1d_op = assign44540_e49663;
        locals.var_dx_wi_1d_op_dn4 = ((locals.var_keq_1d_qm_op_dn4 * assign44540_e49662) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn4 - locals.var_xg2eff_op_dn4)));
        locals.var_dx_wi_1d_op_dn6 = ((locals.var_keq_1d_qm_op_dn6 * assign44540_e49662) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn6 - locals.var_xg2eff_op_dn6)));
        locals.var_dx_wi_1d_op_dn7 = ((locals.var_keq_1d_qm_op_dn7 * assign44540_e49662) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn7 - locals.var_xg2eff_op_dn7)));
        locals.var_dx_wi_1d_op_dn8 = ((locals.var_keq_1d_qm_op_dn8 * assign44540_e49662) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn8 - locals.var_xg2eff_op_dn8)));
        locals.var_dx_wi_1d_op_dn9 = ((locals.var_keq_1d_qm_op_dn9 * assign44540_e49662) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn9 - locals.var_xg2eff_op_dn9)));

        let assign44550_e49666: f64 = if locals.var_dx_wi_1d_op > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1362 = assign44550_e49666;

        let assign44560_e49668: f64 = (-locals.var_dx_wi_1d_op);
        let assign44560_e49670: f64 = if assign44560_e49668 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1363 = assign44560_e49670;

        let (assign44570_e49681, assign44570_e49681_d_n4, assign44570_e49681_d_n6, assign44570_e49681_d_n7, assign44570_e49681_d_n8, assign44570_e49681_d_n9,) = {
    if ((locals.var_guard1362 != 0.0) && (locals.var_guard1363 != 0.0)) {
        let assign44570_e49676: f64 = (-locals.var_dx_wi_1d_op);
        let assign44570_e49677: f64 = (assign44570_e49676).exp();
        let assign44570_e49678: f64 = (1.0 + assign44570_e49677);
        let assign44570_e49679: f64 = (assign44570_e49678).ln();
        (assign44570_e49679, ((assign44570_e49677 * (-locals.var_dx_wi_1d_op_dn4)) / assign44570_e49678), ((assign44570_e49677 * (-locals.var_dx_wi_1d_op_dn6)) / assign44570_e49678), ((assign44570_e49677 * (-locals.var_dx_wi_1d_op_dn7)) / assign44570_e49678), ((assign44570_e49677 * (-locals.var_dx_wi_1d_op_dn8)) / assign44570_e49678), ((assign44570_e49677 * (-locals.var_dx_wi_1d_op_dn9)) / assign44570_e49678),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign44570_e49681;
        locals.var_temp_dn4 = assign44570_e49681_d_n4;
        locals.var_temp_dn6 = assign44570_e49681_d_n6;
        locals.var_temp_dn7 = assign44570_e49681_d_n7;
        locals.var_temp_dn8 = assign44570_e49681_d_n8;
        locals.var_temp_dn9 = assign44570_e49681_d_n9;

        let (assign44580_e49689, assign44580_e49689_d_n4, assign44580_e49689_d_n6, assign44580_e49689_d_n7, assign44580_e49689_d_n8, assign44580_e49689_d_n9,) = {
    if ((locals.var_guard1362 != 0.0) && (locals.var_guard1363 == 0.0)) {
        let assign44580_e49687: f64 = (-locals.var_dx_wi_1d_op);
        (assign44580_e49687, (-locals.var_dx_wi_1d_op_dn4), (-locals.var_dx_wi_1d_op_dn6), (-locals.var_dx_wi_1d_op_dn7), (-locals.var_dx_wi_1d_op_dn8), (-locals.var_dx_wi_1d_op_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign44580_e49689;
        locals.var_temp_dn4 = assign44580_e49689_d_n4;
        locals.var_temp_dn6 = assign44580_e49689_d_n6;
        locals.var_temp_dn7 = assign44580_e49689_d_n7;
        locals.var_temp_dn8 = assign44580_e49689_d_n8;
        locals.var_temp_dn9 = assign44580_e49689_d_n9;

        let (assign44590_e49701, assign44590_e49701_d_n4, assign44590_e49701_d_n6, assign44590_e49701_d_n7, assign44590_e49701_d_n8, assign44590_e49701_d_n9,) = {
    if (locals.var_guard1362 != 0.0) {
        let assign44590_e49694: f64 = (locals.var_dx_wi_1d_op / locals.var_k1_1d_qm_op);
        let assign44590_e49695: f64 = (locals.var_xg10_op - assign44590_e49694);
        let assign44590_e49697: f64 = (assign44590_e49695 + locals.var_temp);
        let assign44590_e49699: f64 = (assign44590_e49697 - 0.6931471805599);
        (assign44590_e49699, ((locals.var_xg10_op_dn4 - (((locals.var_dx_wi_1d_op_dn4 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn4)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn4), ((locals.var_xg10_op_dn6 - (((locals.var_dx_wi_1d_op_dn6 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn6)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn6), ((locals.var_xg10_op_dn7 - (((locals.var_dx_wi_1d_op_dn7 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn7)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn7), ((locals.var_xg10_op_dn8 - (((locals.var_dx_wi_1d_op_dn8 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn8)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn8), ((locals.var_xg10_op_dn9 - (((locals.var_dx_wi_1d_op_dn9 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn9)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn9),)
    } else {
        (locals.var_x_wi_1d_op, locals.var_x_wi_1d_op_dn4, locals.var_x_wi_1d_op_dn6, locals.var_x_wi_1d_op_dn7, locals.var_x_wi_1d_op_dn8, locals.var_x_wi_1d_op_dn9,)
    }
};
        locals.var_x_wi_1d_op = assign44590_e49701;
        locals.var_x_wi_1d_op_dn4 = assign44590_e49701_d_n4;
        locals.var_x_wi_1d_op_dn6 = assign44590_e49701_d_n6;
        locals.var_x_wi_1d_op_dn7 = assign44590_e49701_d_n7;
        locals.var_x_wi_1d_op_dn8 = assign44590_e49701_d_n8;
        locals.var_x_wi_1d_op_dn9 = assign44590_e49701_d_n9;

        let assign44600_e49704: f64 = if locals.var_dx_wi_1d_op < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1364 = assign44600_e49704;

        let (assign44610_e49715, assign44610_e49715_d_n4, assign44610_e49715_d_n6, assign44610_e49715_d_n7, assign44610_e49715_d_n8, assign44610_e49715_d_n9,) = {
    if ((locals.var_guard1362 == 0.0) && (locals.var_guard1364 != 0.0)) {
        let assign44610_e49711: f64 = (locals.var_dx_wi_1d_op).exp();
        let assign44610_e49712: f64 = (1.0 + assign44610_e49711);
        let assign44610_e49713: f64 = (assign44610_e49712).ln();
        (assign44610_e49713, ((assign44610_e49711 * locals.var_dx_wi_1d_op_dn4) / assign44610_e49712), ((assign44610_e49711 * locals.var_dx_wi_1d_op_dn6) / assign44610_e49712), ((assign44610_e49711 * locals.var_dx_wi_1d_op_dn7) / assign44610_e49712), ((assign44610_e49711 * locals.var_dx_wi_1d_op_dn8) / assign44610_e49712), ((assign44610_e49711 * locals.var_dx_wi_1d_op_dn9) / assign44610_e49712),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign44610_e49715;
        locals.var_temp_dn4 = assign44610_e49715_d_n4;
        locals.var_temp_dn6 = assign44610_e49715_d_n6;
        locals.var_temp_dn7 = assign44610_e49715_d_n7;
        locals.var_temp_dn8 = assign44610_e49715_d_n8;
        locals.var_temp_dn9 = assign44610_e49715_d_n9;

        let (assign44620_e49723, assign44620_e49723_d_n4, assign44620_e49723_d_n6, assign44620_e49723_d_n7, assign44620_e49723_d_n8, assign44620_e49723_d_n9,) = {
    if ((locals.var_guard1362 == 0.0) && (locals.var_guard1364 == 0.0)) {
        (locals.var_dx_wi_1d_op, locals.var_dx_wi_1d_op_dn4, locals.var_dx_wi_1d_op_dn6, locals.var_dx_wi_1d_op_dn7, locals.var_dx_wi_1d_op_dn8, locals.var_dx_wi_1d_op_dn9,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign44620_e49723;
        locals.var_temp_dn4 = assign44620_e49723_d_n4;
        locals.var_temp_dn6 = assign44620_e49723_d_n6;
        locals.var_temp_dn7 = assign44620_e49723_d_n7;
        locals.var_temp_dn8 = assign44620_e49723_d_n8;
        locals.var_temp_dn9 = assign44620_e49723_d_n9;

        let (assign44630_e49736, assign44630_e49736_d_n4, assign44630_e49736_d_n6, assign44630_e49736_d_n7, assign44630_e49736_d_n8, assign44630_e49736_d_n9,) = {
    if (locals.var_guard1362 == 0.0) {
        let assign44630_e49729: f64 = (locals.var_dx_wi_1d_op / locals.var_k2_1d_qm_op);
        let assign44630_e49730: f64 = (locals.var_xg2eff_op + assign44630_e49729);
        let assign44630_e49732: f64 = (assign44630_e49730 + locals.var_temp);
        let assign44630_e49734: f64 = (assign44630_e49732 - 0.6931471805599);
        (assign44630_e49734, ((locals.var_xg2eff_op_dn4 + (((locals.var_dx_wi_1d_op_dn4 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn4)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn4), ((locals.var_xg2eff_op_dn6 + (((locals.var_dx_wi_1d_op_dn6 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn6)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn6), ((locals.var_xg2eff_op_dn7 + (((locals.var_dx_wi_1d_op_dn7 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn7)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn7), ((locals.var_xg2eff_op_dn8 + (((locals.var_dx_wi_1d_op_dn8 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn8)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn8), ((locals.var_xg2eff_op_dn9 + (((locals.var_dx_wi_1d_op_dn9 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn9)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn9),)
    } else {
        (locals.var_x_wi_1d_op, locals.var_x_wi_1d_op_dn4, locals.var_x_wi_1d_op_dn6, locals.var_x_wi_1d_op_dn7, locals.var_x_wi_1d_op_dn8, locals.var_x_wi_1d_op_dn9,)
    }
};
        locals.var_x_wi_1d_op = assign44630_e49736;
        locals.var_x_wi_1d_op_dn4 = assign44630_e49736_d_n4;
        locals.var_x_wi_1d_op_dn6 = assign44630_e49736_d_n6;
        locals.var_x_wi_1d_op_dn7 = assign44630_e49736_d_n7;
        locals.var_x_wi_1d_op_dn8 = assign44630_e49736_d_n8;
        locals.var_x_wi_1d_op_dn9 = assign44630_e49736_d_n9;

        let assign44640_e49740: f64 = (locals.var_x_wi_1d_op + locals.var_xth_1d_op);
        let assign44640_e49743: f64 = (locals.var_x_wi_1d_op - locals.var_xth_1d_op);
        let assign44640_e49746: f64 = (locals.var_x_wi_1d_op - locals.var_xth_1d_op);
        let assign44640_e49747: f64 = (assign44640_e49743 * assign44640_e49746);
        let assign44640_e49749: f64 = (assign44640_e49747 + 4.0);
        let assign44640_e49750: f64 = (assign44640_e49749).sqrt();
        let assign44640_e49751: f64 = (assign44640_e49740 - assign44640_e49750);
        let assign44640_e49752: f64 = (0.5 * assign44640_e49751);
        locals.var_x_1d_op = assign44640_e49752;
        locals.var_x_1d_op_dn4 = (0.5 * ((locals.var_x_wi_1d_op_dn4 + locals.var_xth_1d_op_dn4) - ((((locals.var_x_wi_1d_op_dn4 - locals.var_xth_1d_op_dn4) * assign44640_e49746) + (assign44640_e49743 * (locals.var_x_wi_1d_op_dn4 - locals.var_xth_1d_op_dn4))) / (2.0 * assign44640_e49750))));
        locals.var_x_1d_op_dn6 = (0.5 * ((locals.var_x_wi_1d_op_dn6 + locals.var_xth_1d_op_dn6) - ((((locals.var_x_wi_1d_op_dn6 - locals.var_xth_1d_op_dn6) * assign44640_e49746) + (assign44640_e49743 * (locals.var_x_wi_1d_op_dn6 - locals.var_xth_1d_op_dn6))) / (2.0 * assign44640_e49750))));
        locals.var_x_1d_op_dn7 = (0.5 * ((locals.var_x_wi_1d_op_dn7 + locals.var_xth_1d_op_dn7) - ((((locals.var_x_wi_1d_op_dn7 - locals.var_xth_1d_op_dn7) * assign44640_e49746) + (assign44640_e49743 * (locals.var_x_wi_1d_op_dn7 - locals.var_xth_1d_op_dn7))) / (2.0 * assign44640_e49750))));
        locals.var_x_1d_op_dn8 = (0.5 * ((locals.var_x_wi_1d_op_dn8 + locals.var_xth_1d_op_dn8) - ((((locals.var_x_wi_1d_op_dn8 - locals.var_xth_1d_op_dn8) * assign44640_e49746) + (assign44640_e49743 * (locals.var_x_wi_1d_op_dn8 - locals.var_xth_1d_op_dn8))) / (2.0 * assign44640_e49750))));
        locals.var_x_1d_op_dn9 = (0.5 * ((locals.var_x_wi_1d_op_dn9 + locals.var_xth_1d_op_dn9) - ((((locals.var_x_wi_1d_op_dn9 - locals.var_xth_1d_op_dn9) * assign44640_e49746) + (assign44640_e49743 * (locals.var_x_wi_1d_op_dn9 - locals.var_xth_1d_op_dn9))) / (2.0 * assign44640_e49750))));

        let assign44650_e49757: f64 = (locals.var_xth_1d_op - locals.var_x_1d_op);
        let assign44650_e49758: f64 = (2.0 * assign44650_e49757);
        let assign44650_e49760: f64 = (assign44650_e49758 / locals.var_xsddep_op);
        let assign44650_e49761: f64 = (1.0 + assign44650_e49760);
        let assign44650_e49762: f64 = (assign44650_e49761).sqrt();
        let assign44650_e49764: f64 = (assign44650_e49762 - 1.0);
        locals.var_dleff_op = assign44650_e49764;
        locals.var_dleff_op_dn4 = (((((2.0 * (locals.var_xth_1d_op_dn4 - locals.var_x_1d_op_dn4)) * locals.var_xsddep_op) - (assign44650_e49758 * locals.var_xsddep_op_dn4)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44650_e49762));
        locals.var_dleff_op_dn6 = (((((2.0 * (locals.var_xth_1d_op_dn6 - locals.var_x_1d_op_dn6)) * locals.var_xsddep_op) - (assign44650_e49758 * locals.var_xsddep_op_dn6)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44650_e49762));
        locals.var_dleff_op_dn7 = (((((2.0 * (locals.var_xth_1d_op_dn7 - locals.var_x_1d_op_dn7)) * locals.var_xsddep_op) - (assign44650_e49758 * locals.var_xsddep_op_dn7)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44650_e49762));
        locals.var_dleff_op_dn8 = (((((2.0 * (locals.var_xth_1d_op_dn8 - locals.var_x_1d_op_dn8)) * locals.var_xsddep_op) - (assign44650_e49758 * locals.var_xsddep_op_dn8)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44650_e49762));
        locals.var_dleff_op_dn9 = (((((2.0 * (locals.var_xth_1d_op_dn9 - locals.var_x_1d_op_dn9)) * locals.var_xsddep_op) - (assign44650_e49758 * locals.var_xsddep_op_dn9)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44650_e49762));

        let assign44670_e49774: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign44670_e49775: f64 = (1.0 + assign44670_e49774);
        let assign44670_e49777: f64 = (assign44670_e49775 + 0.5);
        let assign44670_e49781: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign44670_e49782: f64 = (1.0 + assign44670_e49781);
        let assign44670_e49784: f64 = (assign44670_e49782 - 0.5);
        let assign44670_e49788: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign44670_e49789: f64 = (1.0 + assign44670_e49788);
        let assign44670_e49791: f64 = (assign44670_e49789 - 0.5);
        let assign44670_e49792: f64 = (assign44670_e49784 * assign44670_e49791);
        let assign44670_e49794: f64 = (assign44670_e49792 + 0.01);
        let assign44670_e49795: f64 = (assign44670_e49794).sqrt();
        let assign44670_e49796: f64 = (assign44670_e49777 + assign44670_e49795);
        let assign44670_e49797: f64 = (0.5 * assign44670_e49796);
        locals.var_temp = assign44670_e49797;
        locals.var_temp_dn4 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn4) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn4) * assign44670_e49791) + (assign44670_e49784 * (locals.var_pscedlb_i * locals.var_xg20_op_dn4))) / (2.0 * assign44670_e49795))));
        locals.var_temp_dn6 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn6) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn6) * assign44670_e49791) + (assign44670_e49784 * (locals.var_pscedlb_i * locals.var_xg20_op_dn6))) / (2.0 * assign44670_e49795))));
        locals.var_temp_dn7 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn7) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn7) * assign44670_e49791) + (assign44670_e49784 * (locals.var_pscedlb_i * locals.var_xg20_op_dn7))) / (2.0 * assign44670_e49795))));
        locals.var_temp_dn8 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn8) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn8) * assign44670_e49791) + (assign44670_e49784 * (locals.var_pscedlb_i * locals.var_xg20_op_dn8))) / (2.0 * assign44670_e49795))));
        locals.var_temp_dn9 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn9) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn9) * assign44670_e49791) + (assign44670_e49784 * (locals.var_pscedlb_i * locals.var_xg20_op_dn9))) / (2.0 * assign44670_e49795))));

        let assign44700_e49814: f64 = (2.0 * locals.var_xd0_op);
        let assign44700_e49818: f64 = (locals.var_xdsx_op / locals.var_xd0_op);
        let assign44700_e49819: f64 = (1.0 + assign44700_e49818);
        let assign44700_e49820: f64 = (assign44700_e49819).sqrt();
        let assign44700_e49822: f64 = (assign44700_e49820 - 1.0);
        let assign44700_e49823: f64 = (assign44700_e49814 * assign44700_e49822);
        let assign44700_e49827: f64 = (locals.var_cfdl_i * locals.var_dleff_op);
        let assign44700_e49828: f64 = (1.0 + assign44700_e49827);
        let assign44700_e49829: f64 = (assign44700_e49823 * assign44700_e49828);
        let assign44700_e49833: f64 = (locals.var_cfdlb_i * locals.var_xg20_op);
        let assign44700_e49834: f64 = (1.0 + assign44700_e49833);
        let assign44700_e49835: f64 = (assign44700_e49829 * assign44700_e49834);
        locals.var_temp = assign44700_e49835;
        locals.var_temp_dn4 = (((((((2.0 * locals.var_xd0_op_dn4) * assign44700_e49822) + (assign44700_e49814 * ((((locals.var_xdsx_op_dn4 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn4)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44700_e49820)))) * assign44700_e49828) + (assign44700_e49823 * (locals.var_cfdl_i * locals.var_dleff_op_dn4))) * assign44700_e49834) + (assign44700_e49829 * (locals.var_cfdlb_i * locals.var_xg20_op_dn4)));
        locals.var_temp_dn6 = (((((((2.0 * locals.var_xd0_op_dn6) * assign44700_e49822) + (assign44700_e49814 * ((((locals.var_xdsx_op_dn6 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn6)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44700_e49820)))) * assign44700_e49828) + (assign44700_e49823 * (locals.var_cfdl_i * locals.var_dleff_op_dn6))) * assign44700_e49834) + (assign44700_e49829 * (locals.var_cfdlb_i * locals.var_xg20_op_dn6)));
        locals.var_temp_dn7 = (((((((2.0 * locals.var_xd0_op_dn7) * assign44700_e49822) + (assign44700_e49814 * ((((locals.var_xdsx_op_dn7 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn7)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44700_e49820)))) * assign44700_e49828) + (assign44700_e49823 * (locals.var_cfdl_i * locals.var_dleff_op_dn7))) * assign44700_e49834) + (assign44700_e49829 * (locals.var_cfdlb_i * locals.var_xg20_op_dn7)));
        locals.var_temp_dn8 = (((((((2.0 * locals.var_xd0_op_dn8) * assign44700_e49822) + (assign44700_e49814 * ((((locals.var_xdsx_op_dn8 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn8)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44700_e49820)))) * assign44700_e49828) + (assign44700_e49823 * (locals.var_cfdl_i * locals.var_dleff_op_dn8))) * assign44700_e49834) + (assign44700_e49829 * (locals.var_cfdlb_i * locals.var_xg20_op_dn8)));
        locals.var_temp_dn9 = (((((((2.0 * locals.var_xd0_op_dn9) * assign44700_e49822) + (assign44700_e49814 * ((((locals.var_xdsx_op_dn9 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn9)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44700_e49820)))) * assign44700_e49828) + (assign44700_e49823 * (locals.var_cfdl_i * locals.var_dleff_op_dn9))) * assign44700_e49834) + (assign44700_e49829 * (locals.var_cfdlb_i * locals.var_xg20_op_dn9)));

        let assign44940_e50018: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1366 = assign44940_e50018;

        let (assign44950_e50028, assign44950_e50028_d_n4, assign44950_e50028_d_n6, assign44950_e50028_d_n7, assign44950_e50028_d_n8, assign44950_e50028_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign44950_e50022: f64 = (locals.var_k2_ac / locals.var_k1_ac);
        let assign44950_e50025: f64 = (1.0 + locals.var_k2_ac);
        let assign44950_e50026: f64 = (assign44950_e50022 / assign44950_e50025);
        (assign44950_e50026, ((((((locals.var_k2_ac_dn4 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn4)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44950_e50025) - (assign44950_e50022 * locals.var_k2_ac_dn4)) / (assign44950_e50025 * assign44950_e50025)), ((((((locals.var_k2_ac_dn6 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn6)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44950_e50025) - (assign44950_e50022 * locals.var_k2_ac_dn6)) / (assign44950_e50025 * assign44950_e50025)), ((((((locals.var_k2_ac_dn7 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn7)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44950_e50025) - (assign44950_e50022 * locals.var_k2_ac_dn7)) / (assign44950_e50025 * assign44950_e50025)), ((((((locals.var_k2_ac_dn8 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn8)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44950_e50025) - (assign44950_e50022 * locals.var_k2_ac_dn8)) / (assign44950_e50025 * assign44950_e50025)), ((((((locals.var_k2_ac_dn9 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn9)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44950_e50025) - (assign44950_e50022 * locals.var_k2_ac_dn9)) / (assign44950_e50025 * assign44950_e50025)),)
    } else {
        (locals.var_r1init_op, locals.var_r1init_op_dn4, locals.var_r1init_op_dn6, locals.var_r1init_op_dn7, locals.var_r1init_op_dn8, locals.var_r1init_op_dn9,)
    }
};
        locals.var_r1init_op = assign44950_e50028;
        locals.var_r1init_op_dn4 = assign44950_e50028_d_n4;
        locals.var_r1init_op_dn6 = assign44950_e50028_d_n6;
        locals.var_r1init_op_dn7 = assign44950_e50028_d_n7;
        locals.var_r1init_op_dn8 = assign44950_e50028_d_n8;
        locals.var_r1init_op_dn9 = assign44950_e50028_d_n9;

        let (assign44960_e50038, assign44960_e50038_d_n4, assign44960_e50038_d_n6, assign44960_e50038_d_n7, assign44960_e50038_d_n8, assign44960_e50038_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign44960_e50032: f64 = (locals.var_k1_ac / locals.var_k2_ac);
        let assign44960_e50035: f64 = (1.0 + locals.var_k1_ac);
        let assign44960_e50036: f64 = (assign44960_e50032 / assign44960_e50035);
        (assign44960_e50036, ((((((locals.var_k1_ac_dn4 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn4)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44960_e50035) - (assign44960_e50032 * locals.var_k1_ac_dn4)) / (assign44960_e50035 * assign44960_e50035)), ((((((locals.var_k1_ac_dn6 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn6)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44960_e50035) - (assign44960_e50032 * locals.var_k1_ac_dn6)) / (assign44960_e50035 * assign44960_e50035)), ((((((locals.var_k1_ac_dn7 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn7)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44960_e50035) - (assign44960_e50032 * locals.var_k1_ac_dn7)) / (assign44960_e50035 * assign44960_e50035)), ((((((locals.var_k1_ac_dn8 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn8)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44960_e50035) - (assign44960_e50032 * locals.var_k1_ac_dn8)) / (assign44960_e50035 * assign44960_e50035)), ((((((locals.var_k1_ac_dn9 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn9)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44960_e50035) - (assign44960_e50032 * locals.var_k1_ac_dn9)) / (assign44960_e50035 * assign44960_e50035)),)
    } else {
        (locals.var_r2init_op, locals.var_r2init_op_dn4, locals.var_r2init_op_dn6, locals.var_r2init_op_dn7, locals.var_r2init_op_dn8, locals.var_r2init_op_dn9,)
    }
};
        locals.var_r2init_op = assign44960_e50038;
        locals.var_r2init_op_dn4 = assign44960_e50038_d_n4;
        locals.var_r2init_op_dn6 = assign44960_e50038_d_n6;
        locals.var_r2init_op_dn7 = assign44960_e50038_d_n7;
        locals.var_r2init_op_dn8 = assign44960_e50038_d_n8;
        locals.var_r2init_op_dn9 = assign44960_e50038_d_n9;

        let (assign44970_e50053, assign44970_e50053_d_n4, assign44970_e50053_d_n6, assign44970_e50053_d_n7, assign44970_e50053_d_n8, assign44970_e50053_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign44970_e50043: f64 = (1.0 + locals.var_r1init_op);
        let assign44970_e50044: f64 = (locals.var_k1_ac * assign44970_e50043);
        let assign44970_e50046: f64 = (assign44970_e50044 * locals.var_diff_min_ac);
        let assign44970_e50048: f64 = (assign44970_e50046 / locals.var_a0_ac);
        let assign44970_e50049: f64 = (assign44970_e50048).ln();
        let assign44970_e50051: f64 = (assign44970_e50049 + 2.0);
        (assign44970_e50051, ((((((((locals.var_k1_ac_dn4 * assign44970_e50043) + (locals.var_k1_ac * locals.var_r1init_op_dn4)) * locals.var_diff_min_ac) + (assign44970_e50044 * locals.var_diff_min_ac_dn4)) * locals.var_a0_ac) - (assign44970_e50046 * locals.var_a0_ac_dn4)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44970_e50048), ((((((((locals.var_k1_ac_dn6 * assign44970_e50043) + (locals.var_k1_ac * locals.var_r1init_op_dn6)) * locals.var_diff_min_ac) + (assign44970_e50044 * locals.var_diff_min_ac_dn6)) * locals.var_a0_ac) - (assign44970_e50046 * locals.var_a0_ac_dn6)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44970_e50048), ((((((((locals.var_k1_ac_dn7 * assign44970_e50043) + (locals.var_k1_ac * locals.var_r1init_op_dn7)) * locals.var_diff_min_ac) + (assign44970_e50044 * locals.var_diff_min_ac_dn7)) * locals.var_a0_ac) - (assign44970_e50046 * locals.var_a0_ac_dn7)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44970_e50048), ((((((((locals.var_k1_ac_dn8 * assign44970_e50043) + (locals.var_k1_ac * locals.var_r1init_op_dn8)) * locals.var_diff_min_ac) + (assign44970_e50044 * locals.var_diff_min_ac_dn8)) * locals.var_a0_ac) - (assign44970_e50046 * locals.var_a0_ac_dn8)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44970_e50048), ((((((((locals.var_k1_ac_dn9 * assign44970_e50043) + (locals.var_k1_ac * locals.var_r1init_op_dn9)) * locals.var_diff_min_ac) + (assign44970_e50044 * locals.var_diff_min_ac_dn9)) * locals.var_a0_ac) - (assign44970_e50046 * locals.var_a0_ac_dn9)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44970_e50048),)
    } else {
        (locals.var_x1init_op, locals.var_x1init_op_dn4, locals.var_x1init_op_dn6, locals.var_x1init_op_dn7, locals.var_x1init_op_dn8, locals.var_x1init_op_dn9,)
    }
};
        locals.var_x1init_op = assign44970_e50053;
        locals.var_x1init_op_dn4 = assign44970_e50053_d_n4;
        locals.var_x1init_op_dn6 = assign44970_e50053_d_n6;
        locals.var_x1init_op_dn7 = assign44970_e50053_d_n7;
        locals.var_x1init_op_dn8 = assign44970_e50053_d_n8;
        locals.var_x1init_op_dn9 = assign44970_e50053_d_n9;

    }

    pub(super) fn stamp_transient_block_123(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign44980_e50068, assign44980_e50068_d_n4, assign44980_e50068_d_n6, assign44980_e50068_d_n7, assign44980_e50068_d_n8, assign44980_e50068_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign44980_e50058: f64 = (1.0 + locals.var_r2init_op);
        let assign44980_e50059: f64 = (locals.var_k2_ac * assign44980_e50058);
        let assign44980_e50061: f64 = (assign44980_e50059 * locals.var_diff_min_ac);
        let assign44980_e50063: f64 = (assign44980_e50061 / locals.var_a0_ac);
        let assign44980_e50064: f64 = (assign44980_e50063).ln();
        let assign44980_e50066: f64 = (assign44980_e50064 + 2.0);
        (assign44980_e50066, ((((((((locals.var_k2_ac_dn4 * assign44980_e50058) + (locals.var_k2_ac * locals.var_r2init_op_dn4)) * locals.var_diff_min_ac) + (assign44980_e50059 * locals.var_diff_min_ac_dn4)) * locals.var_a0_ac) - (assign44980_e50061 * locals.var_a0_ac_dn4)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44980_e50063), ((((((((locals.var_k2_ac_dn6 * assign44980_e50058) + (locals.var_k2_ac * locals.var_r2init_op_dn6)) * locals.var_diff_min_ac) + (assign44980_e50059 * locals.var_diff_min_ac_dn6)) * locals.var_a0_ac) - (assign44980_e50061 * locals.var_a0_ac_dn6)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44980_e50063), ((((((((locals.var_k2_ac_dn7 * assign44980_e50058) + (locals.var_k2_ac * locals.var_r2init_op_dn7)) * locals.var_diff_min_ac) + (assign44980_e50059 * locals.var_diff_min_ac_dn7)) * locals.var_a0_ac) - (assign44980_e50061 * locals.var_a0_ac_dn7)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44980_e50063), ((((((((locals.var_k2_ac_dn8 * assign44980_e50058) + (locals.var_k2_ac * locals.var_r2init_op_dn8)) * locals.var_diff_min_ac) + (assign44980_e50059 * locals.var_diff_min_ac_dn8)) * locals.var_a0_ac) - (assign44980_e50061 * locals.var_a0_ac_dn8)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44980_e50063), ((((((((locals.var_k2_ac_dn9 * assign44980_e50058) + (locals.var_k2_ac * locals.var_r2init_op_dn9)) * locals.var_diff_min_ac) + (assign44980_e50059 * locals.var_diff_min_ac_dn9)) * locals.var_a0_ac) - (assign44980_e50061 * locals.var_a0_ac_dn9)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44980_e50063),)
    } else {
        (locals.var_x2init_op, locals.var_x2init_op_dn4, locals.var_x2init_op_dn6, locals.var_x2init_op_dn7, locals.var_x2init_op_dn8, locals.var_x2init_op_dn9,)
    }
};
        locals.var_x2init_op = assign44980_e50068;
        locals.var_x2init_op_dn4 = assign44980_e50068_d_n4;
        locals.var_x2init_op_dn6 = assign44980_e50068_d_n6;
        locals.var_x2init_op_dn7 = assign44980_e50068_d_n7;
        locals.var_x2init_op_dn8 = assign44980_e50068_d_n8;
        locals.var_x2init_op_dn9 = assign44980_e50068_d_n9;

        let (assign44990_e50080, assign44990_e50080_d_n4, assign44990_e50080_d_n6, assign44990_e50080_d_n7, assign44990_e50080_d_n8, assign44990_e50080_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign44990_e50072: f64 = (1.0 + locals.var_r1init_op);
        let assign44990_e50074: f64 = (assign44990_e50072 * locals.var_x1init_op);
        let assign44990_e50077: f64 = (locals.var_xg2x_ac * locals.var_r1init_op);
        let assign44990_e50078: f64 = (assign44990_e50074 - assign44990_e50077);
        (assign44990_e50078, (((locals.var_r1init_op_dn4 * locals.var_x1init_op) + (assign44990_e50072 * locals.var_x1init_op_dn4)) - ((locals.var_xg2x_ac_dn4 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn4))), (((locals.var_r1init_op_dn6 * locals.var_x1init_op) + (assign44990_e50072 * locals.var_x1init_op_dn6)) - ((locals.var_xg2x_ac_dn6 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn6))), (((locals.var_r1init_op_dn7 * locals.var_x1init_op) + (assign44990_e50072 * locals.var_x1init_op_dn7)) - ((locals.var_xg2x_ac_dn7 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn7))), (((locals.var_r1init_op_dn8 * locals.var_x1init_op) + (assign44990_e50072 * locals.var_x1init_op_dn8)) - ((locals.var_xg2x_ac_dn8 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn8))), (((locals.var_r1init_op_dn9 * locals.var_x1init_op) + (assign44990_e50072 * locals.var_x1init_op_dn9)) - ((locals.var_xg2x_ac_dn9 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn9))),)
    } else {
        (locals.var_xth1init_op, locals.var_xth1init_op_dn4, locals.var_xth1init_op_dn6, locals.var_xth1init_op_dn7, locals.var_xth1init_op_dn8, locals.var_xth1init_op_dn9,)
    }
};
        locals.var_xth1init_op = assign44990_e50080;
        locals.var_xth1init_op_dn4 = assign44990_e50080_d_n4;
        locals.var_xth1init_op_dn6 = assign44990_e50080_d_n6;
        locals.var_xth1init_op_dn7 = assign44990_e50080_d_n7;
        locals.var_xth1init_op_dn8 = assign44990_e50080_d_n8;
        locals.var_xth1init_op_dn9 = assign44990_e50080_d_n9;

        let (assign45000_e50094, assign45000_e50094_d_n4, assign45000_e50094_d_n6, assign45000_e50094_d_n7, assign45000_e50094_d_n8, assign45000_e50094_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign45000_e50085: f64 = (1.0 / locals.var_r2init_op);
        let assign45000_e50086: f64 = (1.0 + assign45000_e50085);
        let assign45000_e50088: f64 = (assign45000_e50086 * locals.var_x2init_op);
        let assign45000_e50091: f64 = (locals.var_xg2x_ac / locals.var_r2init_op);
        let assign45000_e50092: f64 = (assign45000_e50088 - assign45000_e50091);
        (assign45000_e50092, ((((-(locals.var_r2init_op_dn4 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign45000_e50086 * locals.var_x2init_op_dn4)) - (((locals.var_xg2x_ac_dn4 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn4)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn6 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign45000_e50086 * locals.var_x2init_op_dn6)) - (((locals.var_xg2x_ac_dn6 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn6)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn7 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign45000_e50086 * locals.var_x2init_op_dn7)) - (((locals.var_xg2x_ac_dn7 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn7)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn8 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign45000_e50086 * locals.var_x2init_op_dn8)) - (((locals.var_xg2x_ac_dn8 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn8)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn9 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign45000_e50086 * locals.var_x2init_op_dn9)) - (((locals.var_xg2x_ac_dn9 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn9)) / (locals.var_r2init_op * locals.var_r2init_op))),)
    } else {
        (locals.var_xth2init_op, locals.var_xth2init_op_dn4, locals.var_xth2init_op_dn6, locals.var_xth2init_op_dn7, locals.var_xth2init_op_dn8, locals.var_xth2init_op_dn9,)
    }
};
        locals.var_xth2init_op = assign45000_e50094;
        locals.var_xth2init_op_dn4 = assign45000_e50094_d_n4;
        locals.var_xth2init_op_dn6 = assign45000_e50094_d_n6;
        locals.var_xth2init_op_dn7 = assign45000_e50094_d_n7;
        locals.var_xth2init_op_dn8 = assign45000_e50094_d_n8;
        locals.var_xth2init_op_dn9 = assign45000_e50094_d_n9;

        let (assign45010_e50119, assign45010_e50119_d_n4, assign45010_e50119_d_n6, assign45010_e50119_d_n7, assign45010_e50119_d_n8, assign45010_e50119_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign45010_e50099: f64 = (locals.var_xth1init_op + locals.var_xth2init_op);
        let assign45010_e50102: f64 = (locals.var_xth1init_op - locals.var_xth2init_op);
        let assign45010_e50105: f64 = (locals.var_xth1init_op - locals.var_xth2init_op);
        let assign45010_e50106: f64 = (assign45010_e50102 * assign45010_e50105);
        let assign45010_e50108: f64 = (assign45010_e50106 + 38.0);
        let assign45010_e50109: f64 = (assign45010_e50108).sqrt();
        let assign45010_e50110: f64 = (assign45010_e50099 - assign45010_e50109);
        let assign45010_e50111: f64 = (0.5 * assign45010_e50110);
        let assign45010_e50113: f64 = (assign45010_e50111 - locals.var_xg2_ac);
        let assign45010_e50115: f64 = (assign45010_e50113 / locals.var_cic1_i);
        let assign45010_e50117: f64 = (assign45010_e50115 + locals.var_xg2_ac);
        (assign45010_e50117, ((((0.5 * ((locals.var_xth1init_op_dn4 + locals.var_xth2init_op_dn4) - ((((locals.var_xth1init_op_dn4 - locals.var_xth2init_op_dn4) * assign45010_e50105) + (assign45010_e50102 * (locals.var_xth1init_op_dn4 - locals.var_xth2init_op_dn4))) / (2.0 * assign45010_e50109)))) - locals.var_xg2_ac_dn4) / locals.var_cic1_i) + locals.var_xg2_ac_dn4), ((((0.5 * ((locals.var_xth1init_op_dn6 + locals.var_xth2init_op_dn6) - ((((locals.var_xth1init_op_dn6 - locals.var_xth2init_op_dn6) * assign45010_e50105) + (assign45010_e50102 * (locals.var_xth1init_op_dn6 - locals.var_xth2init_op_dn6))) / (2.0 * assign45010_e50109)))) - locals.var_xg2_ac_dn6) / locals.var_cic1_i) + locals.var_xg2_ac_dn6), ((((0.5 * ((locals.var_xth1init_op_dn7 + locals.var_xth2init_op_dn7) - ((((locals.var_xth1init_op_dn7 - locals.var_xth2init_op_dn7) * assign45010_e50105) + (assign45010_e50102 * (locals.var_xth1init_op_dn7 - locals.var_xth2init_op_dn7))) / (2.0 * assign45010_e50109)))) - locals.var_xg2_ac_dn7) / locals.var_cic1_i) + locals.var_xg2_ac_dn7), ((((0.5 * ((locals.var_xth1init_op_dn8 + locals.var_xth2init_op_dn8) - ((((locals.var_xth1init_op_dn8 - locals.var_xth2init_op_dn8) * assign45010_e50105) + (assign45010_e50102 * (locals.var_xth1init_op_dn8 - locals.var_xth2init_op_dn8))) / (2.0 * assign45010_e50109)))) - locals.var_xg2_ac_dn8) / locals.var_cic1_i) + locals.var_xg2_ac_dn8), ((((0.5 * ((locals.var_xth1init_op_dn9 + locals.var_xth2init_op_dn9) - ((((locals.var_xth1init_op_dn9 - locals.var_xth2init_op_dn9) * assign45010_e50105) + (assign45010_e50102 * (locals.var_xth1init_op_dn9 - locals.var_xth2init_op_dn9))) / (2.0 * assign45010_e50109)))) - locals.var_xg2_ac_dn9) / locals.var_cic1_i) + locals.var_xg2_ac_dn9),)
    } else {
        (locals.var_xg1thinit_op, locals.var_xg1thinit_op_dn4, locals.var_xg1thinit_op_dn6, locals.var_xg1thinit_op_dn7, locals.var_xg1thinit_op_dn8, locals.var_xg1thinit_op_dn9,)
    }
};
        locals.var_xg1thinit_op = assign45010_e50119;
        locals.var_xg1thinit_op_dn4 = assign45010_e50119_d_n4;
        locals.var_xg1thinit_op_dn6 = assign45010_e50119_d_n6;
        locals.var_xg1thinit_op_dn7 = assign45010_e50119_d_n7;
        locals.var_xg1thinit_op_dn8 = assign45010_e50119_d_n8;
        locals.var_xg1thinit_op_dn9 = assign45010_e50119_d_n9;

        let (assign45020_e50135, assign45020_e50135_d_n4, assign45020_e50135_d_n6, assign45020_e50135_d_n7, assign45020_e50135_d_n8, assign45020_e50135_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign45020_e50124: f64 = (locals.var_xg1thinit_op - locals.var_xedge_ac);
        let assign45020_e50126: f64 = (assign45020_e50124 / locals.var_sce1_ac);
        let assign45020_e50128: f64 = (assign45020_e50126 - locals.var_dxg1_dibl_ac);
        let assign45020_e50130: f64 = (assign45020_e50128 + locals.var_xedge_ac);
        let assign45020_e50131: f64 = (locals.var_phit * assign45020_e50130);
        let assign45020_e50133: f64 = (assign45020_e50131 + locals.var_vfbac1_i);
        (assign45020_e50133, (((locals.var_phit_dn4 * assign45020_e50130) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn4 - locals.var_xedge_ac_dn4) * locals.var_sce1_ac) - (assign45020_e50124 * locals.var_sce1_ac_dn4)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn4) + locals.var_xedge_ac_dn4))) + locals.var_vfbac1_i_dn4), (((locals.var_phit_dn6 * assign45020_e50130) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn6 - locals.var_xedge_ac_dn6) * locals.var_sce1_ac) - (assign45020_e50124 * locals.var_sce1_ac_dn6)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn6) + locals.var_xedge_ac_dn6))) + locals.var_vfbac1_i_dn6), (((locals.var_phit_dn7 * assign45020_e50130) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn7 - locals.var_xedge_ac_dn7) * locals.var_sce1_ac) - (assign45020_e50124 * locals.var_sce1_ac_dn7)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn7) + locals.var_xedge_ac_dn7))) + locals.var_vfbac1_i_dn7), (((locals.var_phit_dn8 * assign45020_e50130) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn8 - locals.var_xedge_ac_dn8) * locals.var_sce1_ac) - (assign45020_e50124 * locals.var_sce1_ac_dn8)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn8) + locals.var_xedge_ac_dn8))) + locals.var_vfbac1_i_dn8), (((locals.var_phit_dn9 * assign45020_e50130) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn9 - locals.var_xedge_ac_dn9) * locals.var_sce1_ac) - (assign45020_e50124 * locals.var_sce1_ac_dn9)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn9) + locals.var_xedge_ac_dn9))) + locals.var_vfbac1_i_dn9),)
    } else {
        (locals.var_vthinit_op, locals.var_vthinit_op_dn4, locals.var_vthinit_op_dn6, locals.var_vthinit_op_dn7, locals.var_vthinit_op_dn8, locals.var_vthinit_op_dn9,)
    }
};
        locals.var_vthinit_op = assign45020_e50135;
        locals.var_vthinit_op_dn4 = assign45020_e50135_d_n4;
        locals.var_vthinit_op_dn6 = assign45020_e50135_d_n6;
        locals.var_vthinit_op_dn7 = assign45020_e50135_d_n7;
        locals.var_vthinit_op_dn8 = assign45020_e50135_d_n8;
        locals.var_vthinit_op_dn9 = assign45020_e50135_d_n9;

        let (assign45030_e50143, assign45030_e50143_d_n4, assign45030_e50143_d_n6, assign45030_e50143_d_n7, assign45030_e50143_d_n8, assign45030_e50143_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign45030_e50140: f64 = (locals.var_tkd - locals.var_tkr);
        let assign45030_e50141: f64 = (locals.var_stcf_i * assign45030_e50140);
        (assign45030_e50141, ((locals.var_stcf_i_dn4 * assign45030_e50140) + (locals.var_stcf_i * locals.var_tkd_dn4)), ((locals.var_stcf_i_dn6 * assign45030_e50140) + (locals.var_stcf_i * locals.var_tkd_dn6)), ((locals.var_stcf_i_dn7 * assign45030_e50140) + (locals.var_stcf_i * locals.var_tkd_dn7)), ((locals.var_stcf_i_dn8 * assign45030_e50140) + (locals.var_stcf_i * locals.var_tkd_dn8)), ((locals.var_stcf_i_dn9 * assign45030_e50140) + (locals.var_stcf_i * locals.var_tkd_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign45030_e50143;
        locals.var_temp_dn4 = assign45030_e50143_d_n4;
        locals.var_temp_dn6 = assign45030_e50143_d_n6;
        locals.var_temp_dn7 = assign45030_e50143_d_n7;
        locals.var_temp_dn8 = assign45030_e50143_d_n8;
        locals.var_temp_dn9 = assign45030_e50143_d_n9;

        let (assign45060_e50167, assign45060_e50167_d_n4, assign45060_e50167_d_n6, assign45060_e50167_d_n7, assign45060_e50167_d_n8, assign45060_e50167_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign45060_e50159: f64 = (p.p14 * locals.var_stvfb_i);
        let assign45060_e50162: f64 = (locals.var_tkd - locals.var_tkr);
        let assign45060_e50163: f64 = (assign45060_e50159 * assign45060_e50162);
        let assign45060_e50165: f64 = (assign45060_e50163 + locals.var_dvfbqm);
        (assign45060_e50165, (assign45060_e50159 * locals.var_tkd_dn4), (assign45060_e50159 * locals.var_tkd_dn6), (assign45060_e50159 * locals.var_tkd_dn7), (assign45060_e50159 * locals.var_tkd_dn8), (assign45060_e50159 * locals.var_tkd_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign45060_e50167;
        locals.var_temp_dn4 = assign45060_e50167_d_n4;
        locals.var_temp_dn6 = assign45060_e50167_d_n6;
        locals.var_temp_dn7 = assign45060_e50167_d_n7;
        locals.var_temp_dn8 = assign45060_e50167_d_n8;
        locals.var_temp_dn9 = assign45060_e50167_d_n9;

        let (assign45070_e50183, assign45070_e50183_d_n4, assign45070_e50183_d_n6, assign45070_e50183_d_n7, assign45070_e50183_d_n8, assign45070_e50183_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign45070_e50172: f64 = (locals.var_vfbac1_t + locals.var_dvfbch_op);
        let assign45070_e50174: f64 = (assign45070_e50172 + locals.var_dvfb1nch);
        let assign45070_e50175: f64 = (p.p14 * assign45070_e50174);
        let assign45070_e50177: f64 = (assign45070_e50175 + locals.var_temp);
        let assign45070_e50179: f64 = (assign45070_e50177 + p.p34);
        let assign45070_e50181: f64 = (assign45070_e50179 - locals.var_dvfbpdep_op);
        (assign45070_e50181, (((p.p14 * ((locals.var_vfbac1_t_dn4 + locals.var_dvfbch_op_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp_dn4) - locals.var_dvfbpdep_op_dn4), (((p.p14 * ((locals.var_vfbac1_t_dn6 + locals.var_dvfbch_op_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp_dn6) - locals.var_dvfbpdep_op_dn6), (((p.p14 * ((locals.var_vfbac1_t_dn7 + locals.var_dvfbch_op_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp_dn7) - locals.var_dvfbpdep_op_dn7), (((p.p14 * ((locals.var_vfbac1_t_dn8 + locals.var_dvfbch_op_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp_dn8) - locals.var_dvfbpdep_op_dn8), (((p.p14 * ((locals.var_vfbac1_t_dn9 + locals.var_dvfbch_op_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp_dn9) - locals.var_dvfbpdep_op_dn9),)
    } else {
        (locals.var_vfb1_op, locals.var_vfb1_op_dn4, locals.var_vfb1_op_dn6, locals.var_vfb1_op_dn7, locals.var_vfb1_op_dn8, locals.var_vfb1_op_dn9,)
    }
};
        locals.var_vfb1_op = assign45070_e50183;
        locals.var_vfb1_op_dn4 = assign45070_e50183_d_n4;
        locals.var_vfb1_op_dn6 = assign45070_e50183_d_n6;
        locals.var_vfb1_op_dn7 = assign45070_e50183_d_n7;
        locals.var_vfb1_op_dn8 = assign45070_e50183_d_n8;
        locals.var_vfb1_op_dn9 = assign45070_e50183_d_n9;

        let (assign45080_e50195, assign45080_e50195_d_n4, assign45080_e50195_d_n6, assign45080_e50195_d_n7, assign45080_e50195_d_n8, assign45080_e50195_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign45080_e50188: f64 = (locals.var_vfbac2_t + locals.var_dvfbch_op);
        let assign45080_e50190: f64 = (assign45080_e50188 + locals.var_dvfb2nch);
        let assign45080_e50191: f64 = (p.p14 * assign45080_e50190);
        let assign45080_e50193: f64 = (assign45080_e50191 + locals.var_temp);
        (assign45080_e50193, ((p.p14 * ((locals.var_vfbac2_t_dn4 + locals.var_dvfbch_op_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfbac2_t_dn6 + locals.var_dvfbch_op_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfbac2_t_dn7 + locals.var_dvfbch_op_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfbac2_t_dn8 + locals.var_dvfbch_op_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfbac2_t_dn9 + locals.var_dvfbch_op_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9),)
    } else {
        (locals.var_vfb2_op, locals.var_vfb2_op_dn4, locals.var_vfb2_op_dn6, locals.var_vfb2_op_dn7, locals.var_vfb2_op_dn8, locals.var_vfb2_op_dn9,)
    }
};
        locals.var_vfb2_op = assign45080_e50195;
        locals.var_vfb2_op_dn4 = assign45080_e50195_d_n4;
        locals.var_vfb2_op_dn6 = assign45080_e50195_d_n6;
        locals.var_vfb2_op_dn7 = assign45080_e50195_d_n7;
        locals.var_vfb2_op_dn8 = assign45080_e50195_d_n8;
        locals.var_vfb2_op_dn9 = assign45080_e50195_d_n9;

        let (assign45090_e50205, assign45090_e50205_d_n4, assign45090_e50205_d_n6, assign45090_e50205_d_n7, assign45090_e50205_d_n8, assign45090_e50205_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign45090_e50199: f64 = (locals.var_vthinit_op - locals.var_vfb1_op);
        let assign45090_e50201: f64 = (assign45090_e50199 * locals.var_inv_phit_op);
        let assign45090_e50203: f64 = (assign45090_e50201 - locals.var_dxdsx_op);
        (assign45090_e50203, ((((locals.var_vthinit_op_dn4 - locals.var_vfb1_op_dn4) * locals.var_inv_phit_op) + (assign45090_e50199 * locals.var_inv_phit_op_dn4)) - locals.var_dxdsx_op_dn4), ((((locals.var_vthinit_op_dn6 - locals.var_vfb1_op_dn6) * locals.var_inv_phit_op) + (assign45090_e50199 * locals.var_inv_phit_op_dn6)) - locals.var_dxdsx_op_dn6), ((((locals.var_vthinit_op_dn7 - locals.var_vfb1_op_dn7) * locals.var_inv_phit_op) + (assign45090_e50199 * locals.var_inv_phit_op_dn7)) - locals.var_dxdsx_op_dn7), ((((locals.var_vthinit_op_dn8 - locals.var_vfb1_op_dn8) * locals.var_inv_phit_op) + (assign45090_e50199 * locals.var_inv_phit_op_dn8)) - locals.var_dxdsx_op_dn8), ((((locals.var_vthinit_op_dn9 - locals.var_vfb1_op_dn9) * locals.var_inv_phit_op) + (assign45090_e50199 * locals.var_inv_phit_op_dn9)) - locals.var_dxdsx_op_dn9),)
    } else {
        (locals.var_xg10_op, locals.var_xg10_op_dn4, locals.var_xg10_op_dn6, locals.var_xg10_op_dn7, locals.var_xg10_op_dn8, locals.var_xg10_op_dn9,)
    }
};
        locals.var_xg10_op = assign45090_e50205;
        locals.var_xg10_op_dn4 = assign45090_e50205_d_n4;
        locals.var_xg10_op_dn6 = assign45090_e50205_d_n6;
        locals.var_xg10_op_dn7 = assign45090_e50205_d_n7;
        locals.var_xg10_op_dn8 = assign45090_e50205_d_n8;
        locals.var_xg10_op_dn9 = assign45090_e50205_d_n9;

        let (assign45100_e50216, assign45100_e50216_d_n4, assign45100_e50216_d_n6, assign45100_e50216_d_n7, assign45100_e50216_d_n8, assign45100_e50216_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign45100_e50208: f64 = (-locals.var_vsb);
        let assign45100_e50210: f64 = (assign45100_e50208 - locals.var_vfb2_op);
        let assign45100_e50212: f64 = (assign45100_e50210 * locals.var_inv_phit_op);
        let assign45100_e50214: f64 = (assign45100_e50212 - locals.var_dxdsx_op);
        (assign45100_e50214, ((((-locals.var_vfb2_op_dn4) * locals.var_inv_phit_op) + (assign45100_e50210 * locals.var_inv_phit_op_dn4)) - locals.var_dxdsx_op_dn4), (((((-locals.var_vsb_dn6) - locals.var_vfb2_op_dn6) * locals.var_inv_phit_op) + (assign45100_e50210 * locals.var_inv_phit_op_dn6)) - locals.var_dxdsx_op_dn6), (((((-locals.var_vsb_dn7) - locals.var_vfb2_op_dn7) * locals.var_inv_phit_op) + (assign45100_e50210 * locals.var_inv_phit_op_dn7)) - locals.var_dxdsx_op_dn7), (((((-locals.var_vsb_dn8) - locals.var_vfb2_op_dn8) * locals.var_inv_phit_op) + (assign45100_e50210 * locals.var_inv_phit_op_dn8)) - locals.var_dxdsx_op_dn8), ((((-locals.var_vfb2_op_dn9) * locals.var_inv_phit_op) + (assign45100_e50210 * locals.var_inv_phit_op_dn9)) - locals.var_dxdsx_op_dn9),)
    } else {
        (locals.var_xg20_op, locals.var_xg20_op_dn4, locals.var_xg20_op_dn6, locals.var_xg20_op_dn7, locals.var_xg20_op_dn8, locals.var_xg20_op_dn9,)
    }
};
        locals.var_xg20_op = assign45100_e50216;
        locals.var_xg20_op_dn4 = assign45100_e50216_d_n4;
        locals.var_xg20_op_dn6 = assign45100_e50216_d_n6;
        locals.var_xg20_op_dn7 = assign45100_e50216_d_n7;
        locals.var_xg20_op_dn8 = assign45100_e50216_d_n8;
        locals.var_xg20_op_dn9 = assign45100_e50216_d_n9;

        let assign45110_e50219: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1367 = assign45110_e50219;

        let (assign45120_e50233, assign45120_e50233_d_n4, assign45120_e50233_d_n6, assign45120_e50233_d_n7, assign45120_e50233_d_n8, assign45120_e50233_d_n9,) = {
    if ((locals.var_guard1366 != 0.0) && (locals.var_guard1367 != 0.0)) {
        let assign45120_e50225: f64 = (p.p14 * locals.var_typesub_i);
        let assign45120_e50228: f64 = (locals.var_xg10_op - locals.var_xg20_op);
        let assign45120_e50229: f64 = (assign45120_e50225 * assign45120_e50228);
        let assign45120_e50231: f64 = (assign45120_e50229 / locals.var_gfsub);
        (assign45120_e50231, ((((assign45120_e50225 * (locals.var_xg10_op_dn4 - locals.var_xg20_op_dn4)) * locals.var_gfsub) - (assign45120_e50229 * locals.var_gfsub_dn4)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign45120_e50225 * (locals.var_xg10_op_dn6 - locals.var_xg20_op_dn6)) * locals.var_gfsub) - (assign45120_e50229 * locals.var_gfsub_dn6)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign45120_e50225 * (locals.var_xg10_op_dn7 - locals.var_xg20_op_dn7)) * locals.var_gfsub) - (assign45120_e50229 * locals.var_gfsub_dn7)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign45120_e50225 * (locals.var_xg10_op_dn8 - locals.var_xg20_op_dn8)) * locals.var_gfsub) - (assign45120_e50229 * locals.var_gfsub_dn8)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign45120_e50225 * (locals.var_xg10_op_dn9 - locals.var_xg20_op_dn9)) * locals.var_gfsub) - (assign45120_e50229 * locals.var_gfsub_dn9)) / (locals.var_gfsub * locals.var_gfsub)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign45120_e50233;
        locals.var_temp_dn4 = assign45120_e50233_d_n4;
        locals.var_temp_dn6 = assign45120_e50233_d_n6;
        locals.var_temp_dn7 = assign45120_e50233_d_n7;
        locals.var_temp_dn8 = assign45120_e50233_d_n8;
        locals.var_temp_dn9 = assign45120_e50233_d_n9;

        let assign45130_e50236: f64 = if locals.var_temp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1368 = assign45130_e50236;

        let (assign45140_e50250, assign45140_e50250_d_n4, assign45140_e50250_d_n6, assign45140_e50250_d_n7, assign45140_e50250_d_n8, assign45140_e50250_d_n9,) = {
    if (((locals.var_guard1366 != 0.0) && (locals.var_guard1367 != 0.0)) && (locals.var_guard1368 != 0.0)) {
        let assign45140_e50243: f64 = (-2.0);
        let assign45140_e50246: f64 = (1.0 - locals.var_temp);
        let assign45140_e50247: f64 = (assign45140_e50246).ln();
        let assign45140_e50248: f64 = (assign45140_e50243 * assign45140_e50247);
        (assign45140_e50248, (assign45140_e50243 * ((-locals.var_temp_dn4) / assign45140_e50246)), (assign45140_e50243 * ((-locals.var_temp_dn6) / assign45140_e50246)), (assign45140_e50243 * ((-locals.var_temp_dn7) / assign45140_e50246)), (assign45140_e50243 * ((-locals.var_temp_dn8) / assign45140_e50246)), (assign45140_e50243 * ((-locals.var_temp_dn9) / assign45140_e50246)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign45140_e50250;
        locals.var_temp1_dn4 = assign45140_e50250_d_n4;
        locals.var_temp1_dn6 = assign45140_e50250_d_n6;
        locals.var_temp1_dn7 = assign45140_e50250_d_n7;
        locals.var_temp1_dn8 = assign45140_e50250_d_n8;
        locals.var_temp1_dn9 = assign45140_e50250_d_n9;

        let (assign45150_e50269, assign45150_e50269_d_n4, assign45150_e50269_d_n6, assign45150_e50269_d_n7, assign45150_e50269_d_n8, assign45150_e50269_d_n9,) = {
    if (((locals.var_guard1366 != 0.0) && (locals.var_guard1367 != 0.0)) && (locals.var_guard1368 == 0.0)) {
        let assign45150_e50259: f64 = (locals.var_temp * locals.var_temp);
        let assign45150_e50263: f64 = (2.0 * locals.var_temp);
        let assign45150_e50265: f64 = (assign45150_e50263 / locals.var_gfsub);
        let assign45150_e50266: f64 = (1.0 + assign45150_e50265);
        let assign45150_e50267: f64 = (assign45150_e50259 / assign45150_e50266);
        (assign45150_e50267, (((((locals.var_temp_dn4 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn4)) * assign45150_e50266) - (assign45150_e50259 * ((((2.0 * locals.var_temp_dn4) * locals.var_gfsub) - (assign45150_e50263 * locals.var_gfsub_dn4)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign45150_e50266 * assign45150_e50266)), (((((locals.var_temp_dn6 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn6)) * assign45150_e50266) - (assign45150_e50259 * ((((2.0 * locals.var_temp_dn6) * locals.var_gfsub) - (assign45150_e50263 * locals.var_gfsub_dn6)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign45150_e50266 * assign45150_e50266)), (((((locals.var_temp_dn7 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn7)) * assign45150_e50266) - (assign45150_e50259 * ((((2.0 * locals.var_temp_dn7) * locals.var_gfsub) - (assign45150_e50263 * locals.var_gfsub_dn7)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign45150_e50266 * assign45150_e50266)), (((((locals.var_temp_dn8 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn8)) * assign45150_e50266) - (assign45150_e50259 * ((((2.0 * locals.var_temp_dn8) * locals.var_gfsub) - (assign45150_e50263 * locals.var_gfsub_dn8)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign45150_e50266 * assign45150_e50266)), (((((locals.var_temp_dn9 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn9)) * assign45150_e50266) - (assign45150_e50259 * ((((2.0 * locals.var_temp_dn9) * locals.var_gfsub) - (assign45150_e50263 * locals.var_gfsub_dn9)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign45150_e50266 * assign45150_e50266)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign45150_e50269;
        locals.var_temp1_dn4 = assign45150_e50269_d_n4;
        locals.var_temp1_dn6 = assign45150_e50269_d_n6;
        locals.var_temp1_dn7 = assign45150_e50269_d_n7;
        locals.var_temp1_dn8 = assign45150_e50269_d_n8;
        locals.var_temp1_dn9 = assign45150_e50269_d_n9;

        let (assign45160_e50281, assign45160_e50281_d_n4, assign45160_e50281_d_n6, assign45160_e50281_d_n7, assign45160_e50281_d_n8, assign45160_e50281_d_n9,) = {
    if ((locals.var_guard1366 != 0.0) && (locals.var_guard1367 != 0.0)) {
        let assign45160_e50276: f64 = (p.p14 * locals.var_typesub_i);
        let assign45160_e50278: f64 = (assign45160_e50276 * locals.var_temp1);
        let assign45160_e50279: f64 = (locals.var_xg20_op + assign45160_e50278);
        (assign45160_e50279, (locals.var_xg20_op_dn4 + (assign45160_e50276 * locals.var_temp1_dn4)), (locals.var_xg20_op_dn6 + (assign45160_e50276 * locals.var_temp1_dn6)), (locals.var_xg20_op_dn7 + (assign45160_e50276 * locals.var_temp1_dn7)), (locals.var_xg20_op_dn8 + (assign45160_e50276 * locals.var_temp1_dn8)), (locals.var_xg20_op_dn9 + (assign45160_e50276 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_xg2eff_op, locals.var_xg2eff_op_dn4, locals.var_xg2eff_op_dn6, locals.var_xg2eff_op_dn7, locals.var_xg2eff_op_dn8, locals.var_xg2eff_op_dn9,)
    }
};
        locals.var_xg2eff_op = assign45160_e50281;
        locals.var_xg2eff_op_dn4 = assign45160_e50281_d_n4;
        locals.var_xg2eff_op_dn6 = assign45160_e50281_d_n6;
        locals.var_xg2eff_op_dn7 = assign45160_e50281_d_n7;
        locals.var_xg2eff_op_dn8 = assign45160_e50281_d_n8;
        locals.var_xg2eff_op_dn9 = assign45160_e50281_d_n9;

        let (assign45170_e50288, assign45170_e50288_d_n4, assign45170_e50288_d_n6, assign45170_e50288_d_n7, assign45170_e50288_d_n8, assign45170_e50288_d_n9,) = {
    if ((locals.var_guard1366 != 0.0) && (locals.var_guard1367 == 0.0)) {
        (locals.var_xg20_op, locals.var_xg20_op_dn4, locals.var_xg20_op_dn6, locals.var_xg20_op_dn7, locals.var_xg20_op_dn8, locals.var_xg20_op_dn9,)
    } else {
        (locals.var_xg2eff_op, locals.var_xg2eff_op_dn4, locals.var_xg2eff_op_dn6, locals.var_xg2eff_op_dn7, locals.var_xg2eff_op_dn8, locals.var_xg2eff_op_dn9,)
    }
};
        locals.var_xg2eff_op = assign45170_e50288;
        locals.var_xg2eff_op_dn4 = assign45170_e50288_d_n4;
        locals.var_xg2eff_op_dn6 = assign45170_e50288_d_n6;
        locals.var_xg2eff_op_dn7 = assign45170_e50288_d_n7;
        locals.var_xg2eff_op_dn8 = assign45170_e50288_d_n8;
        locals.var_xg2eff_op_dn9 = assign45170_e50288_d_n9;

        let (assign45180_e50296, assign45180_e50296_d_n4, assign45180_e50296_d_n6, assign45180_e50296_d_n7, assign45180_e50296_d_n8, assign45180_e50296_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign45180_e50293: f64 = (locals.var_xg10_op - locals.var_xg2eff_op);
        let assign45180_e50294: f64 = (locals.var_keq_1d * assign45180_e50293);
        (assign45180_e50294, (locals.var_keq_1d * (locals.var_xg10_op_dn4 - locals.var_xg2eff_op_dn4)), (locals.var_keq_1d * (locals.var_xg10_op_dn6 - locals.var_xg2eff_op_dn6)), (locals.var_keq_1d * (locals.var_xg10_op_dn7 - locals.var_xg2eff_op_dn7)), (locals.var_keq_1d * (locals.var_xg10_op_dn8 - locals.var_xg2eff_op_dn8)), (locals.var_keq_1d * (locals.var_xg10_op_dn9 - locals.var_xg2eff_op_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign45180_e50296;
        locals.var_temp_dn4 = assign45180_e50296_d_n4;
        locals.var_temp_dn6 = assign45180_e50296_d_n6;
        locals.var_temp_dn7 = assign45180_e50296_d_n7;
        locals.var_temp_dn8 = assign45180_e50296_d_n8;
        locals.var_temp_dn9 = assign45180_e50296_d_n9;

        let assign45190_e50299: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1369 = assign45190_e50299;

        let (assign45200_e50322, assign45200_e50322_d_n4, assign45200_e50322_d_n6, assign45200_e50322_d_n7, assign45200_e50322_d_n8, assign45200_e50322_d_n9,) = {
    if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 != 0.0)) {
        let assign45200_e50306: f64 = (locals.var_temp + locals.var_emin);
        let assign45200_e50309: f64 = (locals.var_temp - locals.var_emin);
        let assign45200_e50312: f64 = (locals.var_temp - locals.var_emin);
        let assign45200_e50313: f64 = (assign45200_e50309 * assign45200_e50312);
        let assign45200_e50316: f64 = (locals.var_emin * locals.var_emin);
        let assign45200_e50317: f64 = (assign45200_e50313 + assign45200_e50316);
        let assign45200_e50318: f64 = (assign45200_e50317).sqrt();
        let assign45200_e50319: f64 = (assign45200_e50306 + assign45200_e50318);
        let assign45200_e50320: f64 = (0.5 * assign45200_e50319);
        (assign45200_e50320, (0.5 * ((locals.var_temp_dn4 + locals.var_emin_dn4) + (((((locals.var_temp_dn4 - locals.var_emin_dn4) * assign45200_e50312) + (assign45200_e50309 * (locals.var_temp_dn4 - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign45200_e50318)))), (0.5 * ((locals.var_temp_dn6 + locals.var_emin_dn6) + (((((locals.var_temp_dn6 - locals.var_emin_dn6) * assign45200_e50312) + (assign45200_e50309 * (locals.var_temp_dn6 - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign45200_e50318)))), (0.5 * ((locals.var_temp_dn7 + locals.var_emin_dn7) + (((((locals.var_temp_dn7 - locals.var_emin_dn7) * assign45200_e50312) + (assign45200_e50309 * (locals.var_temp_dn7 - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign45200_e50318)))), (0.5 * ((locals.var_temp_dn8 + locals.var_emin_dn8) + (((((locals.var_temp_dn8 - locals.var_emin_dn8) * assign45200_e50312) + (assign45200_e50309 * (locals.var_temp_dn8 - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign45200_e50318)))), (0.5 * ((locals.var_temp_dn9 + locals.var_emin_dn9) + (((((locals.var_temp_dn9 - locals.var_emin_dn9) * assign45200_e50312) + (assign45200_e50309 * (locals.var_temp_dn9 - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign45200_e50318)))),)
    } else {
        (locals.var_e1_op, locals.var_e1_op_dn4, locals.var_e1_op_dn6, locals.var_e1_op_dn7, locals.var_e1_op_dn8, locals.var_e1_op_dn9,)
    }
};
        locals.var_e1_op = assign45200_e50322;
        locals.var_e1_op_dn4 = assign45200_e50322_d_n4;
        locals.var_e1_op_dn6 = assign45200_e50322_d_n6;
        locals.var_e1_op_dn7 = assign45200_e50322_d_n7;
        locals.var_e1_op_dn8 = assign45200_e50322_d_n8;
        locals.var_e1_op_dn9 = assign45200_e50322_d_n9;

        let (assign45210_e50348, assign45210_e50348_d_n4, assign45210_e50348_d_n6, assign45210_e50348_d_n7, assign45210_e50348_d_n8, assign45210_e50348_d_n9,) = {
    if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 != 0.0)) {
        let assign45210_e50328: f64 = (-locals.var_temp);
        let assign45210_e50330: f64 = (assign45210_e50328 + locals.var_emin);
        let assign45210_e50332: f64 = (-locals.var_temp);
        let assign45210_e50334: f64 = (assign45210_e50332 - locals.var_emin);
        let assign45210_e50336: f64 = (-locals.var_temp);
        let assign45210_e50338: f64 = (assign45210_e50336 - locals.var_emin);
        let assign45210_e50339: f64 = (assign45210_e50334 * assign45210_e50338);
        let assign45210_e50342: f64 = (locals.var_emin * locals.var_emin);
        let assign45210_e50343: f64 = (assign45210_e50339 + assign45210_e50342);
        let assign45210_e50344: f64 = (assign45210_e50343).sqrt();
        let assign45210_e50345: f64 = (assign45210_e50330 + assign45210_e50344);
        let assign45210_e50346: f64 = (0.5 * assign45210_e50345);
        (assign45210_e50346, (0.5 * (((-locals.var_temp_dn4) + locals.var_emin_dn4) + ((((((-locals.var_temp_dn4) - locals.var_emin_dn4) * assign45210_e50338) + (assign45210_e50334 * ((-locals.var_temp_dn4) - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign45210_e50344)))), (0.5 * (((-locals.var_temp_dn6) + locals.var_emin_dn6) + ((((((-locals.var_temp_dn6) - locals.var_emin_dn6) * assign45210_e50338) + (assign45210_e50334 * ((-locals.var_temp_dn6) - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign45210_e50344)))), (0.5 * (((-locals.var_temp_dn7) + locals.var_emin_dn7) + ((((((-locals.var_temp_dn7) - locals.var_emin_dn7) * assign45210_e50338) + (assign45210_e50334 * ((-locals.var_temp_dn7) - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign45210_e50344)))), (0.5 * (((-locals.var_temp_dn8) + locals.var_emin_dn8) + ((((((-locals.var_temp_dn8) - locals.var_emin_dn8) * assign45210_e50338) + (assign45210_e50334 * ((-locals.var_temp_dn8) - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign45210_e50344)))), (0.5 * (((-locals.var_temp_dn9) + locals.var_emin_dn9) + ((((((-locals.var_temp_dn9) - locals.var_emin_dn9) * assign45210_e50338) + (assign45210_e50334 * ((-locals.var_temp_dn9) - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign45210_e50344)))),)
    } else {
        (locals.var_e2_op, locals.var_e2_op_dn4, locals.var_e2_op_dn6, locals.var_e2_op_dn7, locals.var_e2_op_dn8, locals.var_e2_op_dn9,)
    }
};
        locals.var_e2_op = assign45210_e50348;
        locals.var_e2_op_dn4 = assign45210_e50348_d_n4;
        locals.var_e2_op_dn6 = assign45210_e50348_d_n6;
        locals.var_e2_op_dn7 = assign45210_e50348_d_n7;
        locals.var_e2_op_dn8 = assign45210_e50348_d_n8;
        locals.var_e2_op_dn9 = assign45210_e50348_d_n9;

        let (assign45220_e50361, assign45220_e50361_d_n4, assign45220_e50361_d_n6, assign45220_e50361_d_n7, assign45220_e50361_d_n8, assign45220_e50361_d_n9,) = {
    if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 != 0.0)) {
        let assign45220_e50354: f64 = (-0.3333333333333);
        let assign45220_e50356: f64 = (locals.var_e1_op).ln();
        let assign45220_e50357: f64 = (assign45220_e50354 * assign45220_e50356);
        let assign45220_e50358: f64 = (assign45220_e50357).exp();
        let assign45220_e50359: f64 = (locals.var_qq_op * assign45220_e50358);
        (assign45220_e50359, ((locals.var_qq_op_dn4 * assign45220_e50358) + (locals.var_qq_op * (assign45220_e50358 * (assign45220_e50354 * (locals.var_e1_op_dn4 / locals.var_e1_op))))), ((locals.var_qq_op_dn6 * assign45220_e50358) + (locals.var_qq_op * (assign45220_e50358 * (assign45220_e50354 * (locals.var_e1_op_dn6 / locals.var_e1_op))))), ((locals.var_qq_op_dn7 * assign45220_e50358) + (locals.var_qq_op * (assign45220_e50358 * (assign45220_e50354 * (locals.var_e1_op_dn7 / locals.var_e1_op))))), ((locals.var_qq_op_dn8 * assign45220_e50358) + (locals.var_qq_op * (assign45220_e50358 * (assign45220_e50354 * (locals.var_e1_op_dn8 / locals.var_e1_op))))), ((locals.var_qq_op_dn9 * assign45220_e50358) + (locals.var_qq_op * (assign45220_e50358 * (assign45220_e50354 * (locals.var_e1_op_dn9 / locals.var_e1_op))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign45220_e50361;
        locals.var_temp1_dn4 = assign45220_e50361_d_n4;
        locals.var_temp1_dn6 = assign45220_e50361_d_n6;
        locals.var_temp1_dn7 = assign45220_e50361_d_n7;
        locals.var_temp1_dn8 = assign45220_e50361_d_n8;
        locals.var_temp1_dn9 = assign45220_e50361_d_n9;

        let (assign45230_e50374, assign45230_e50374_d_n4, assign45230_e50374_d_n6, assign45230_e50374_d_n7, assign45230_e50374_d_n8, assign45230_e50374_d_n9,) = {
    if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 != 0.0)) {
        let assign45230_e50367: f64 = (-0.3333333333333);
        let assign45230_e50369: f64 = (locals.var_e2_op).ln();
        let assign45230_e50370: f64 = (assign45230_e50367 * assign45230_e50369);
        let assign45230_e50371: f64 = (assign45230_e50370).exp();
        let assign45230_e50372: f64 = (locals.var_qq_op * assign45230_e50371);
        (assign45230_e50372, ((locals.var_qq_op_dn4 * assign45230_e50371) + (locals.var_qq_op * (assign45230_e50371 * (assign45230_e50367 * (locals.var_e2_op_dn4 / locals.var_e2_op))))), ((locals.var_qq_op_dn6 * assign45230_e50371) + (locals.var_qq_op * (assign45230_e50371 * (assign45230_e50367 * (locals.var_e2_op_dn6 / locals.var_e2_op))))), ((locals.var_qq_op_dn7 * assign45230_e50371) + (locals.var_qq_op * (assign45230_e50371 * (assign45230_e50367 * (locals.var_e2_op_dn7 / locals.var_e2_op))))), ((locals.var_qq_op_dn8 * assign45230_e50371) + (locals.var_qq_op * (assign45230_e50371 * (assign45230_e50367 * (locals.var_e2_op_dn8 / locals.var_e2_op))))), ((locals.var_qq_op_dn9 * assign45230_e50371) + (locals.var_qq_op * (assign45230_e50371 * (assign45230_e50367 * (locals.var_e2_op_dn9 / locals.var_e2_op))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign45230_e50374;
        locals.var_temp2_dn4 = assign45230_e50374_d_n4;
        locals.var_temp2_dn6 = assign45230_e50374_d_n6;
        locals.var_temp2_dn7 = assign45230_e50374_d_n7;
        locals.var_temp2_dn8 = assign45230_e50374_d_n8;
        locals.var_temp2_dn9 = assign45230_e50374_d_n9;

        let (assign45240_e50384, assign45240_e50384_d_n4, assign45240_e50384_d_n6, assign45240_e50384_d_n7, assign45240_e50384_d_n8, assign45240_e50384_d_n9,) = {
    if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 != 0.0)) {
        let assign45240_e50380: f64 = (1.0 - locals.var_temp1);
        let assign45240_e50382: f64 = (assign45240_e50380 - locals.var_temp2);
        (assign45240_e50382, ((-locals.var_temp1_dn4) - locals.var_temp2_dn4), ((-locals.var_temp1_dn6) - locals.var_temp2_dn6), ((-locals.var_temp1_dn7) - locals.var_temp2_dn7), ((-locals.var_temp1_dn8) - locals.var_temp2_dn8), ((-locals.var_temp1_dn9) - locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign45240_e50384;
        locals.var_temp3_dn4 = assign45240_e50384_d_n4;
        locals.var_temp3_dn6 = assign45240_e50384_d_n6;
        locals.var_temp3_dn7 = assign45240_e50384_d_n7;
        locals.var_temp3_dn8 = assign45240_e50384_d_n8;
        locals.var_temp3_dn9 = assign45240_e50384_d_n9;

        let (assign45260_e50406, assign45260_e50406_d_n4, assign45260_e50406_d_n6, assign45260_e50406_d_n7, assign45260_e50406_d_n8, assign45260_e50406_d_n9,) = {
    if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 != 0.0)) {
        let assign45260_e50398: f64 = (locals.var_k1_1d * locals.var_temp3);
        let assign45260_e50402: f64 = (locals.var_k1_1d * locals.var_temp1);
        let assign45260_e50403: f64 = (1.0 + assign45260_e50402);
        let assign45260_e50404: f64 = (assign45260_e50398 / assign45260_e50403);
        (assign45260_e50404, ((((locals.var_k1_1d * locals.var_temp3_dn4) * assign45260_e50403) - (assign45260_e50398 * (locals.var_k1_1d * locals.var_temp1_dn4))) / (assign45260_e50403 * assign45260_e50403)), ((((locals.var_k1_1d * locals.var_temp3_dn6) * assign45260_e50403) - (assign45260_e50398 * (locals.var_k1_1d * locals.var_temp1_dn6))) / (assign45260_e50403 * assign45260_e50403)), ((((locals.var_k1_1d * locals.var_temp3_dn7) * assign45260_e50403) - (assign45260_e50398 * (locals.var_k1_1d * locals.var_temp1_dn7))) / (assign45260_e50403 * assign45260_e50403)), ((((locals.var_k1_1d * locals.var_temp3_dn8) * assign45260_e50403) - (assign45260_e50398 * (locals.var_k1_1d * locals.var_temp1_dn8))) / (assign45260_e50403 * assign45260_e50403)), ((((locals.var_k1_1d * locals.var_temp3_dn9) * assign45260_e50403) - (assign45260_e50398 * (locals.var_k1_1d * locals.var_temp1_dn9))) / (assign45260_e50403 * assign45260_e50403)),)
    } else {
        (locals.var_k1_1d_qm_op, locals.var_k1_1d_qm_op_dn4, locals.var_k1_1d_qm_op_dn6, locals.var_k1_1d_qm_op_dn7, locals.var_k1_1d_qm_op_dn8, locals.var_k1_1d_qm_op_dn9,)
    }
};
        locals.var_k1_1d_qm_op = assign45260_e50406;
        locals.var_k1_1d_qm_op_dn4 = assign45260_e50406_d_n4;
        locals.var_k1_1d_qm_op_dn6 = assign45260_e50406_d_n6;
        locals.var_k1_1d_qm_op_dn7 = assign45260_e50406_d_n7;
        locals.var_k1_1d_qm_op_dn8 = assign45260_e50406_d_n8;
        locals.var_k1_1d_qm_op_dn9 = assign45260_e50406_d_n9;

        let (assign45270_e50420, assign45270_e50420_d_n4, assign45270_e50420_d_n6, assign45270_e50420_d_n7, assign45270_e50420_d_n8, assign45270_e50420_d_n9,) = {
    if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 != 0.0)) {
        let assign45270_e50412: f64 = (locals.var_k2_1d * locals.var_temp3);
        let assign45270_e50416: f64 = (locals.var_k2_1d * locals.var_temp2);
        let assign45270_e50417: f64 = (1.0 + assign45270_e50416);
        let assign45270_e50418: f64 = (assign45270_e50412 / assign45270_e50417);
        (assign45270_e50418, ((((locals.var_k2_1d * locals.var_temp3_dn4) * assign45270_e50417) - (assign45270_e50412 * (locals.var_k2_1d * locals.var_temp2_dn4))) / (assign45270_e50417 * assign45270_e50417)), ((((locals.var_k2_1d * locals.var_temp3_dn6) * assign45270_e50417) - (assign45270_e50412 * (locals.var_k2_1d * locals.var_temp2_dn6))) / (assign45270_e50417 * assign45270_e50417)), ((((locals.var_k2_1d * locals.var_temp3_dn7) * assign45270_e50417) - (assign45270_e50412 * (locals.var_k2_1d * locals.var_temp2_dn7))) / (assign45270_e50417 * assign45270_e50417)), ((((locals.var_k2_1d * locals.var_temp3_dn8) * assign45270_e50417) - (assign45270_e50412 * (locals.var_k2_1d * locals.var_temp2_dn8))) / (assign45270_e50417 * assign45270_e50417)), ((((locals.var_k2_1d * locals.var_temp3_dn9) * assign45270_e50417) - (assign45270_e50412 * (locals.var_k2_1d * locals.var_temp2_dn9))) / (assign45270_e50417 * assign45270_e50417)),)
    } else {
        (locals.var_k2_1d_qm_op, locals.var_k2_1d_qm_op_dn4, locals.var_k2_1d_qm_op_dn6, locals.var_k2_1d_qm_op_dn7, locals.var_k2_1d_qm_op_dn8, locals.var_k2_1d_qm_op_dn9,)
    }
};
        locals.var_k2_1d_qm_op = assign45270_e50420;
        locals.var_k2_1d_qm_op_dn4 = assign45270_e50420_d_n4;
        locals.var_k2_1d_qm_op_dn6 = assign45270_e50420_d_n6;
        locals.var_k2_1d_qm_op_dn7 = assign45270_e50420_d_n7;
        locals.var_k2_1d_qm_op_dn8 = assign45270_e50420_d_n8;
        locals.var_k2_1d_qm_op_dn9 = assign45270_e50420_d_n9;

        let (assign45280_e50436, assign45280_e50436_d_n4, assign45280_e50436_d_n6, assign45280_e50436_d_n7, assign45280_e50436_d_n8, assign45280_e50436_d_n9,) = {
    if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 != 0.0)) {
        let assign45280_e50428: f64 = (1.0 / locals.var_k1_1d_qm_op);
        let assign45280_e50429: f64 = (1.0 + assign45280_e50428);
        let assign45280_e50432: f64 = (1.0 / locals.var_k2_1d_qm_op);
        let assign45280_e50433: f64 = (assign45280_e50429 + assign45280_e50432);
        let assign45280_e50434: f64 = (1.0 / assign45280_e50433);
        (assign45280_e50434, (-(((-(locals.var_k1_1d_qm_op_dn4 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn4 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45280_e50433 * assign45280_e50433))), (-(((-(locals.var_k1_1d_qm_op_dn6 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn6 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45280_e50433 * assign45280_e50433))), (-(((-(locals.var_k1_1d_qm_op_dn7 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn7 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45280_e50433 * assign45280_e50433))), (-(((-(locals.var_k1_1d_qm_op_dn8 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn8 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45280_e50433 * assign45280_e50433))), (-(((-(locals.var_k1_1d_qm_op_dn9 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn9 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45280_e50433 * assign45280_e50433))),)
    } else {
        (locals.var_keq_1d_qm_op, locals.var_keq_1d_qm_op_dn4, locals.var_keq_1d_qm_op_dn6, locals.var_keq_1d_qm_op_dn7, locals.var_keq_1d_qm_op_dn8, locals.var_keq_1d_qm_op_dn9,)
    }
};
        locals.var_keq_1d_qm_op = assign45280_e50436;
        locals.var_keq_1d_qm_op_dn4 = assign45280_e50436_d_n4;
        locals.var_keq_1d_qm_op_dn6 = assign45280_e50436_d_n6;
        locals.var_keq_1d_qm_op_dn7 = assign45280_e50436_d_n7;
        locals.var_keq_1d_qm_op_dn8 = assign45280_e50436_d_n8;
        locals.var_keq_1d_qm_op_dn9 = assign45280_e50436_d_n9;

        let (assign45300_e50450, assign45300_e50450_d_n4, assign45300_e50450_d_n6, assign45300_e50450_d_n7, assign45300_e50450_d_n8, assign45300_e50450_d_n9,) = {
    if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 == 0.0)) {
        (locals.var_k1_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_k1_1d_qm_op, locals.var_k1_1d_qm_op_dn4, locals.var_k1_1d_qm_op_dn6, locals.var_k1_1d_qm_op_dn7, locals.var_k1_1d_qm_op_dn8, locals.var_k1_1d_qm_op_dn9,)
    }
};
        locals.var_k1_1d_qm_op = assign45300_e50450;
        locals.var_k1_1d_qm_op_dn4 = assign45300_e50450_d_n4;
        locals.var_k1_1d_qm_op_dn6 = assign45300_e50450_d_n6;
        locals.var_k1_1d_qm_op_dn7 = assign45300_e50450_d_n7;
        locals.var_k1_1d_qm_op_dn8 = assign45300_e50450_d_n8;
        locals.var_k1_1d_qm_op_dn9 = assign45300_e50450_d_n9;

        let (assign45310_e50457, assign45310_e50457_d_n4, assign45310_e50457_d_n6, assign45310_e50457_d_n7, assign45310_e50457_d_n8, assign45310_e50457_d_n9,) = {
    if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 == 0.0)) {
        (locals.var_k2_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_k2_1d_qm_op, locals.var_k2_1d_qm_op_dn4, locals.var_k2_1d_qm_op_dn6, locals.var_k2_1d_qm_op_dn7, locals.var_k2_1d_qm_op_dn8, locals.var_k2_1d_qm_op_dn9,)
    }
};
        locals.var_k2_1d_qm_op = assign45310_e50457;
        locals.var_k2_1d_qm_op_dn4 = assign45310_e50457_d_n4;
        locals.var_k2_1d_qm_op_dn6 = assign45310_e50457_d_n6;
        locals.var_k2_1d_qm_op_dn7 = assign45310_e50457_d_n7;
        locals.var_k2_1d_qm_op_dn8 = assign45310_e50457_d_n8;
        locals.var_k2_1d_qm_op_dn9 = assign45310_e50457_d_n9;

    }

    pub(super) fn stamp_transient_block_124(
        locals: &mut StampLocals,
    ) {
        let (assign45320_e50464, assign45320_e50464_d_n4, assign45320_e50464_d_n6, assign45320_e50464_d_n7, assign45320_e50464_d_n8, assign45320_e50464_d_n9,) = {
    if ((locals.var_guard1366 != 0.0) && (locals.var_guard1369 == 0.0)) {
        (locals.var_keq_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_keq_1d_qm_op, locals.var_keq_1d_qm_op_dn4, locals.var_keq_1d_qm_op_dn6, locals.var_keq_1d_qm_op_dn7, locals.var_keq_1d_qm_op_dn8, locals.var_keq_1d_qm_op_dn9,)
    }
};
        locals.var_keq_1d_qm_op = assign45320_e50464;
        locals.var_keq_1d_qm_op_dn4 = assign45320_e50464_d_n4;
        locals.var_keq_1d_qm_op_dn6 = assign45320_e50464_d_n6;
        locals.var_keq_1d_qm_op_dn7 = assign45320_e50464_d_n7;
        locals.var_keq_1d_qm_op_dn8 = assign45320_e50464_d_n8;
        locals.var_keq_1d_qm_op_dn9 = assign45320_e50464_d_n9;

        let (assign45330_e50472, assign45330_e50472_d_n4, assign45330_e50472_d_n6, assign45330_e50472_d_n7, assign45330_e50472_d_n8, assign45330_e50472_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign45330_e50469: f64 = (locals.var_xg10_op - locals.var_xg2eff_op);
        let assign45330_e50470: f64 = (locals.var_keq_1d_qm_op * assign45330_e50469);
        (assign45330_e50470, ((locals.var_keq_1d_qm_op_dn4 * assign45330_e50469) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn4 - locals.var_xg2eff_op_dn4))), ((locals.var_keq_1d_qm_op_dn6 * assign45330_e50469) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn6 - locals.var_xg2eff_op_dn6))), ((locals.var_keq_1d_qm_op_dn7 * assign45330_e50469) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn7 - locals.var_xg2eff_op_dn7))), ((locals.var_keq_1d_qm_op_dn8 * assign45330_e50469) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn8 - locals.var_xg2eff_op_dn8))), ((locals.var_keq_1d_qm_op_dn9 * assign45330_e50469) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn9 - locals.var_xg2eff_op_dn9))),)
    } else {
        (locals.var_dx_wi_1d_op, locals.var_dx_wi_1d_op_dn4, locals.var_dx_wi_1d_op_dn6, locals.var_dx_wi_1d_op_dn7, locals.var_dx_wi_1d_op_dn8, locals.var_dx_wi_1d_op_dn9,)
    }
};
        locals.var_dx_wi_1d_op = assign45330_e50472;
        locals.var_dx_wi_1d_op_dn4 = assign45330_e50472_d_n4;
        locals.var_dx_wi_1d_op_dn6 = assign45330_e50472_d_n6;
        locals.var_dx_wi_1d_op_dn7 = assign45330_e50472_d_n7;
        locals.var_dx_wi_1d_op_dn8 = assign45330_e50472_d_n8;
        locals.var_dx_wi_1d_op_dn9 = assign45330_e50472_d_n9;

        let assign45340_e50475: f64 = if locals.var_dx_wi_1d_op > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1370 = assign45340_e50475;

        let assign45350_e50477: f64 = (-locals.var_dx_wi_1d_op);
        let assign45350_e50479: f64 = if assign45350_e50477 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1371 = assign45350_e50479;

        let (assign45360_e50492, assign45360_e50492_d_n4, assign45360_e50492_d_n6, assign45360_e50492_d_n7, assign45360_e50492_d_n8, assign45360_e50492_d_n9,) = {
    if (((locals.var_guard1366 != 0.0) && (locals.var_guard1370 != 0.0)) && (locals.var_guard1371 != 0.0)) {
        let assign45360_e50487: f64 = (-locals.var_dx_wi_1d_op);
        let assign45360_e50488: f64 = (assign45360_e50487).exp();
        let assign45360_e50489: f64 = (1.0 + assign45360_e50488);
        let assign45360_e50490: f64 = (assign45360_e50489).ln();
        (assign45360_e50490, ((assign45360_e50488 * (-locals.var_dx_wi_1d_op_dn4)) / assign45360_e50489), ((assign45360_e50488 * (-locals.var_dx_wi_1d_op_dn6)) / assign45360_e50489), ((assign45360_e50488 * (-locals.var_dx_wi_1d_op_dn7)) / assign45360_e50489), ((assign45360_e50488 * (-locals.var_dx_wi_1d_op_dn8)) / assign45360_e50489), ((assign45360_e50488 * (-locals.var_dx_wi_1d_op_dn9)) / assign45360_e50489),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign45360_e50492;
        locals.var_temp_dn4 = assign45360_e50492_d_n4;
        locals.var_temp_dn6 = assign45360_e50492_d_n6;
        locals.var_temp_dn7 = assign45360_e50492_d_n7;
        locals.var_temp_dn8 = assign45360_e50492_d_n8;
        locals.var_temp_dn9 = assign45360_e50492_d_n9;

        let (assign45370_e50502, assign45370_e50502_d_n4, assign45370_e50502_d_n6, assign45370_e50502_d_n7, assign45370_e50502_d_n8, assign45370_e50502_d_n9,) = {
    if (((locals.var_guard1366 != 0.0) && (locals.var_guard1370 != 0.0)) && (locals.var_guard1371 == 0.0)) {
        let assign45370_e50500: f64 = (-locals.var_dx_wi_1d_op);
        (assign45370_e50500, (-locals.var_dx_wi_1d_op_dn4), (-locals.var_dx_wi_1d_op_dn6), (-locals.var_dx_wi_1d_op_dn7), (-locals.var_dx_wi_1d_op_dn8), (-locals.var_dx_wi_1d_op_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign45370_e50502;
        locals.var_temp_dn4 = assign45370_e50502_d_n4;
        locals.var_temp_dn6 = assign45370_e50502_d_n6;
        locals.var_temp_dn7 = assign45370_e50502_d_n7;
        locals.var_temp_dn8 = assign45370_e50502_d_n8;
        locals.var_temp_dn9 = assign45370_e50502_d_n9;

        let (assign45380_e50516, assign45380_e50516_d_n4, assign45380_e50516_d_n6, assign45380_e50516_d_n7, assign45380_e50516_d_n8, assign45380_e50516_d_n9,) = {
    if ((locals.var_guard1366 != 0.0) && (locals.var_guard1370 != 0.0)) {
        let assign45380_e50509: f64 = (locals.var_dx_wi_1d_op / locals.var_k1_1d_qm_op);
        let assign45380_e50510: f64 = (locals.var_xg10_op - assign45380_e50509);
        let assign45380_e50512: f64 = (assign45380_e50510 + locals.var_temp);
        let assign45380_e50514: f64 = (assign45380_e50512 - 0.6931471805599);
        (assign45380_e50514, ((locals.var_xg10_op_dn4 - (((locals.var_dx_wi_1d_op_dn4 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn4)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn4), ((locals.var_xg10_op_dn6 - (((locals.var_dx_wi_1d_op_dn6 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn6)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn6), ((locals.var_xg10_op_dn7 - (((locals.var_dx_wi_1d_op_dn7 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn7)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn7), ((locals.var_xg10_op_dn8 - (((locals.var_dx_wi_1d_op_dn8 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn8)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn8), ((locals.var_xg10_op_dn9 - (((locals.var_dx_wi_1d_op_dn9 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn9)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn9),)
    } else {
        (locals.var_x_wi_1d_op, locals.var_x_wi_1d_op_dn4, locals.var_x_wi_1d_op_dn6, locals.var_x_wi_1d_op_dn7, locals.var_x_wi_1d_op_dn8, locals.var_x_wi_1d_op_dn9,)
    }
};
        locals.var_x_wi_1d_op = assign45380_e50516;
        locals.var_x_wi_1d_op_dn4 = assign45380_e50516_d_n4;
        locals.var_x_wi_1d_op_dn6 = assign45380_e50516_d_n6;
        locals.var_x_wi_1d_op_dn7 = assign45380_e50516_d_n7;
        locals.var_x_wi_1d_op_dn8 = assign45380_e50516_d_n8;
        locals.var_x_wi_1d_op_dn9 = assign45380_e50516_d_n9;

        let assign45390_e50519: f64 = if locals.var_dx_wi_1d_op < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1372 = assign45390_e50519;

        let (assign45400_e50532, assign45400_e50532_d_n4, assign45400_e50532_d_n6, assign45400_e50532_d_n7, assign45400_e50532_d_n8, assign45400_e50532_d_n9,) = {
    if (((locals.var_guard1366 != 0.0) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1372 != 0.0)) {
        let assign45400_e50528: f64 = (locals.var_dx_wi_1d_op).exp();
        let assign45400_e50529: f64 = (1.0 + assign45400_e50528);
        let assign45400_e50530: f64 = (assign45400_e50529).ln();
        (assign45400_e50530, ((assign45400_e50528 * locals.var_dx_wi_1d_op_dn4) / assign45400_e50529), ((assign45400_e50528 * locals.var_dx_wi_1d_op_dn6) / assign45400_e50529), ((assign45400_e50528 * locals.var_dx_wi_1d_op_dn7) / assign45400_e50529), ((assign45400_e50528 * locals.var_dx_wi_1d_op_dn8) / assign45400_e50529), ((assign45400_e50528 * locals.var_dx_wi_1d_op_dn9) / assign45400_e50529),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign45400_e50532;
        locals.var_temp_dn4 = assign45400_e50532_d_n4;
        locals.var_temp_dn6 = assign45400_e50532_d_n6;
        locals.var_temp_dn7 = assign45400_e50532_d_n7;
        locals.var_temp_dn8 = assign45400_e50532_d_n8;
        locals.var_temp_dn9 = assign45400_e50532_d_n9;

        let (assign45410_e50542, assign45410_e50542_d_n4, assign45410_e50542_d_n6, assign45410_e50542_d_n7, assign45410_e50542_d_n8, assign45410_e50542_d_n9,) = {
    if (((locals.var_guard1366 != 0.0) && (locals.var_guard1370 == 0.0)) && (locals.var_guard1372 == 0.0)) {
        (locals.var_dx_wi_1d_op, locals.var_dx_wi_1d_op_dn4, locals.var_dx_wi_1d_op_dn6, locals.var_dx_wi_1d_op_dn7, locals.var_dx_wi_1d_op_dn8, locals.var_dx_wi_1d_op_dn9,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign45410_e50542;
        locals.var_temp_dn4 = assign45410_e50542_d_n4;
        locals.var_temp_dn6 = assign45410_e50542_d_n6;
        locals.var_temp_dn7 = assign45410_e50542_d_n7;
        locals.var_temp_dn8 = assign45410_e50542_d_n8;
        locals.var_temp_dn9 = assign45410_e50542_d_n9;

        let (assign45420_e50557, assign45420_e50557_d_n4, assign45420_e50557_d_n6, assign45420_e50557_d_n7, assign45420_e50557_d_n8, assign45420_e50557_d_n9,) = {
    if ((locals.var_guard1366 != 0.0) && (locals.var_guard1370 == 0.0)) {
        let assign45420_e50550: f64 = (locals.var_dx_wi_1d_op / locals.var_k2_1d_qm_op);
        let assign45420_e50551: f64 = (locals.var_xg2eff_op + assign45420_e50550);
        let assign45420_e50553: f64 = (assign45420_e50551 + locals.var_temp);
        let assign45420_e50555: f64 = (assign45420_e50553 - 0.6931471805599);
        (assign45420_e50555, ((locals.var_xg2eff_op_dn4 + (((locals.var_dx_wi_1d_op_dn4 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn4)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn4), ((locals.var_xg2eff_op_dn6 + (((locals.var_dx_wi_1d_op_dn6 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn6)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn6), ((locals.var_xg2eff_op_dn7 + (((locals.var_dx_wi_1d_op_dn7 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn7)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn7), ((locals.var_xg2eff_op_dn8 + (((locals.var_dx_wi_1d_op_dn8 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn8)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn8), ((locals.var_xg2eff_op_dn9 + (((locals.var_dx_wi_1d_op_dn9 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn9)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn9),)
    } else {
        (locals.var_x_wi_1d_op, locals.var_x_wi_1d_op_dn4, locals.var_x_wi_1d_op_dn6, locals.var_x_wi_1d_op_dn7, locals.var_x_wi_1d_op_dn8, locals.var_x_wi_1d_op_dn9,)
    }
};
        locals.var_x_wi_1d_op = assign45420_e50557;
        locals.var_x_wi_1d_op_dn4 = assign45420_e50557_d_n4;
        locals.var_x_wi_1d_op_dn6 = assign45420_e50557_d_n6;
        locals.var_x_wi_1d_op_dn7 = assign45420_e50557_d_n7;
        locals.var_x_wi_1d_op_dn8 = assign45420_e50557_d_n8;
        locals.var_x_wi_1d_op_dn9 = assign45420_e50557_d_n9;

        let (assign45430_e50576, assign45430_e50576_d_n4, assign45430_e50576_d_n6, assign45430_e50576_d_n7, assign45430_e50576_d_n8, assign45430_e50576_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign45430_e50562: f64 = (locals.var_x_wi_1d_op + locals.var_xth_1d_op);
        let assign45430_e50565: f64 = (locals.var_x_wi_1d_op - locals.var_xth_1d_op);
        let assign45430_e50568: f64 = (locals.var_x_wi_1d_op - locals.var_xth_1d_op);
        let assign45430_e50569: f64 = (assign45430_e50565 * assign45430_e50568);
        let assign45430_e50571: f64 = (assign45430_e50569 + 4.0);
        let assign45430_e50572: f64 = (assign45430_e50571).sqrt();
        let assign45430_e50573: f64 = (assign45430_e50562 - assign45430_e50572);
        let assign45430_e50574: f64 = (0.5 * assign45430_e50573);
        (assign45430_e50574, (0.5 * ((locals.var_x_wi_1d_op_dn4 + locals.var_xth_1d_op_dn4) - ((((locals.var_x_wi_1d_op_dn4 - locals.var_xth_1d_op_dn4) * assign45430_e50568) + (assign45430_e50565 * (locals.var_x_wi_1d_op_dn4 - locals.var_xth_1d_op_dn4))) / (2.0 * assign45430_e50572)))), (0.5 * ((locals.var_x_wi_1d_op_dn6 + locals.var_xth_1d_op_dn6) - ((((locals.var_x_wi_1d_op_dn6 - locals.var_xth_1d_op_dn6) * assign45430_e50568) + (assign45430_e50565 * (locals.var_x_wi_1d_op_dn6 - locals.var_xth_1d_op_dn6))) / (2.0 * assign45430_e50572)))), (0.5 * ((locals.var_x_wi_1d_op_dn7 + locals.var_xth_1d_op_dn7) - ((((locals.var_x_wi_1d_op_dn7 - locals.var_xth_1d_op_dn7) * assign45430_e50568) + (assign45430_e50565 * (locals.var_x_wi_1d_op_dn7 - locals.var_xth_1d_op_dn7))) / (2.0 * assign45430_e50572)))), (0.5 * ((locals.var_x_wi_1d_op_dn8 + locals.var_xth_1d_op_dn8) - ((((locals.var_x_wi_1d_op_dn8 - locals.var_xth_1d_op_dn8) * assign45430_e50568) + (assign45430_e50565 * (locals.var_x_wi_1d_op_dn8 - locals.var_xth_1d_op_dn8))) / (2.0 * assign45430_e50572)))), (0.5 * ((locals.var_x_wi_1d_op_dn9 + locals.var_xth_1d_op_dn9) - ((((locals.var_x_wi_1d_op_dn9 - locals.var_xth_1d_op_dn9) * assign45430_e50568) + (assign45430_e50565 * (locals.var_x_wi_1d_op_dn9 - locals.var_xth_1d_op_dn9))) / (2.0 * assign45430_e50572)))),)
    } else {
        (locals.var_x_1d_op, locals.var_x_1d_op_dn4, locals.var_x_1d_op_dn6, locals.var_x_1d_op_dn7, locals.var_x_1d_op_dn8, locals.var_x_1d_op_dn9,)
    }
};
        locals.var_x_1d_op = assign45430_e50576;
        locals.var_x_1d_op_dn4 = assign45430_e50576_d_n4;
        locals.var_x_1d_op_dn6 = assign45430_e50576_d_n6;
        locals.var_x_1d_op_dn7 = assign45430_e50576_d_n7;
        locals.var_x_1d_op_dn8 = assign45430_e50576_d_n8;
        locals.var_x_1d_op_dn9 = assign45430_e50576_d_n9;

        let (assign45440_e50591, assign45440_e50591_d_n4, assign45440_e50591_d_n6, assign45440_e50591_d_n7, assign45440_e50591_d_n8, assign45440_e50591_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign45440_e50582: f64 = (locals.var_xth_1d_op - locals.var_x_1d_op);
        let assign45440_e50583: f64 = (2.0 * assign45440_e50582);
        let assign45440_e50585: f64 = (assign45440_e50583 / locals.var_xsddep_op);
        let assign45440_e50586: f64 = (1.0 + assign45440_e50585);
        let assign45440_e50587: f64 = (assign45440_e50586).sqrt();
        let assign45440_e50589: f64 = (assign45440_e50587 - 1.0);
        (assign45440_e50589, (((((2.0 * (locals.var_xth_1d_op_dn4 - locals.var_x_1d_op_dn4)) * locals.var_xsddep_op) - (assign45440_e50583 * locals.var_xsddep_op_dn4)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45440_e50587)), (((((2.0 * (locals.var_xth_1d_op_dn6 - locals.var_x_1d_op_dn6)) * locals.var_xsddep_op) - (assign45440_e50583 * locals.var_xsddep_op_dn6)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45440_e50587)), (((((2.0 * (locals.var_xth_1d_op_dn7 - locals.var_x_1d_op_dn7)) * locals.var_xsddep_op) - (assign45440_e50583 * locals.var_xsddep_op_dn7)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45440_e50587)), (((((2.0 * (locals.var_xth_1d_op_dn8 - locals.var_x_1d_op_dn8)) * locals.var_xsddep_op) - (assign45440_e50583 * locals.var_xsddep_op_dn8)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45440_e50587)), (((((2.0 * (locals.var_xth_1d_op_dn9 - locals.var_x_1d_op_dn9)) * locals.var_xsddep_op) - (assign45440_e50583 * locals.var_xsddep_op_dn9)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45440_e50587)),)
    } else {
        (locals.var_dleff_op, locals.var_dleff_op_dn4, locals.var_dleff_op_dn6, locals.var_dleff_op_dn7, locals.var_dleff_op_dn8, locals.var_dleff_op_dn9,)
    }
};
        locals.var_dleff_op = assign45440_e50591;
        locals.var_dleff_op_dn4 = assign45440_e50591_d_n4;
        locals.var_dleff_op_dn6 = assign45440_e50591_d_n6;
        locals.var_dleff_op_dn7 = assign45440_e50591_d_n7;
        locals.var_dleff_op_dn8 = assign45440_e50591_d_n8;
        locals.var_dleff_op_dn9 = assign45440_e50591_d_n9;

        let (assign45460_e50630, assign45460_e50630_d_n4, assign45460_e50630_d_n6, assign45460_e50630_d_n7, assign45460_e50630_d_n8, assign45460_e50630_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign45460_e50605: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign45460_e50606: f64 = (1.0 + assign45460_e50605);
        let assign45460_e50608: f64 = (assign45460_e50606 + 0.5);
        let assign45460_e50612: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign45460_e50613: f64 = (1.0 + assign45460_e50612);
        let assign45460_e50615: f64 = (assign45460_e50613 - 0.5);
        let assign45460_e50619: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign45460_e50620: f64 = (1.0 + assign45460_e50619);
        let assign45460_e50622: f64 = (assign45460_e50620 - 0.5);
        let assign45460_e50623: f64 = (assign45460_e50615 * assign45460_e50622);
        let assign45460_e50625: f64 = (assign45460_e50623 + 0.01);
        let assign45460_e50626: f64 = (assign45460_e50625).sqrt();
        let assign45460_e50627: f64 = (assign45460_e50608 + assign45460_e50626);
        let assign45460_e50628: f64 = (0.5 * assign45460_e50627);
        (assign45460_e50628, (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn4) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn4) * assign45460_e50622) + (assign45460_e50615 * (locals.var_pscedlb_i * locals.var_xg20_op_dn4))) / (2.0 * assign45460_e50626)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn6) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn6) * assign45460_e50622) + (assign45460_e50615 * (locals.var_pscedlb_i * locals.var_xg20_op_dn6))) / (2.0 * assign45460_e50626)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn7) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn7) * assign45460_e50622) + (assign45460_e50615 * (locals.var_pscedlb_i * locals.var_xg20_op_dn7))) / (2.0 * assign45460_e50626)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn8) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn8) * assign45460_e50622) + (assign45460_e50615 * (locals.var_pscedlb_i * locals.var_xg20_op_dn8))) / (2.0 * assign45460_e50626)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn9) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn9) * assign45460_e50622) + (assign45460_e50615 * (locals.var_pscedlb_i * locals.var_xg20_op_dn9))) / (2.0 * assign45460_e50626)))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign45460_e50630;
        locals.var_temp_dn4 = assign45460_e50630_d_n4;
        locals.var_temp_dn6 = assign45460_e50630_d_n6;
        locals.var_temp_dn7 = assign45460_e50630_d_n7;
        locals.var_temp_dn8 = assign45460_e50630_d_n8;
        locals.var_temp_dn9 = assign45460_e50630_d_n9;

        let (assign45490_e50677, assign45490_e50677_d_n4, assign45490_e50677_d_n6, assign45490_e50677_d_n7, assign45490_e50677_d_n8, assign45490_e50677_d_n9,) = {
    if (locals.var_guard1366 != 0.0) {
        let assign45490_e50654: f64 = (2.0 * locals.var_xd0_op);
        let assign45490_e50658: f64 = (locals.var_xdsx_op / locals.var_xd0_op);
        let assign45490_e50659: f64 = (1.0 + assign45490_e50658);
        let assign45490_e50660: f64 = (assign45490_e50659).sqrt();
        let assign45490_e50662: f64 = (assign45490_e50660 - 1.0);
        let assign45490_e50663: f64 = (assign45490_e50654 * assign45490_e50662);
        let assign45490_e50667: f64 = (locals.var_cfdl_i * locals.var_dleff_op);
        let assign45490_e50668: f64 = (1.0 + assign45490_e50667);
        let assign45490_e50669: f64 = (assign45490_e50663 * assign45490_e50668);
        let assign45490_e50673: f64 = (locals.var_cfdlb_i * locals.var_xg20_op);
        let assign45490_e50674: f64 = (1.0 + assign45490_e50673);
        let assign45490_e50675: f64 = (assign45490_e50669 * assign45490_e50674);
        (assign45490_e50675, (((((((2.0 * locals.var_xd0_op_dn4) * assign45490_e50662) + (assign45490_e50654 * ((((locals.var_xdsx_op_dn4 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn4)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45490_e50660)))) * assign45490_e50668) + (assign45490_e50663 * (locals.var_cfdl_i * locals.var_dleff_op_dn4))) * assign45490_e50674) + (assign45490_e50669 * (locals.var_cfdlb_i * locals.var_xg20_op_dn4))), (((((((2.0 * locals.var_xd0_op_dn6) * assign45490_e50662) + (assign45490_e50654 * ((((locals.var_xdsx_op_dn6 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn6)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45490_e50660)))) * assign45490_e50668) + (assign45490_e50663 * (locals.var_cfdl_i * locals.var_dleff_op_dn6))) * assign45490_e50674) + (assign45490_e50669 * (locals.var_cfdlb_i * locals.var_xg20_op_dn6))), (((((((2.0 * locals.var_xd0_op_dn7) * assign45490_e50662) + (assign45490_e50654 * ((((locals.var_xdsx_op_dn7 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn7)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45490_e50660)))) * assign45490_e50668) + (assign45490_e50663 * (locals.var_cfdl_i * locals.var_dleff_op_dn7))) * assign45490_e50674) + (assign45490_e50669 * (locals.var_cfdlb_i * locals.var_xg20_op_dn7))), (((((((2.0 * locals.var_xd0_op_dn8) * assign45490_e50662) + (assign45490_e50654 * ((((locals.var_xdsx_op_dn8 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn8)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45490_e50660)))) * assign45490_e50668) + (assign45490_e50663 * (locals.var_cfdl_i * locals.var_dleff_op_dn8))) * assign45490_e50674) + (assign45490_e50669 * (locals.var_cfdlb_i * locals.var_xg20_op_dn8))), (((((((2.0 * locals.var_xd0_op_dn9) * assign45490_e50662) + (assign45490_e50654 * ((((locals.var_xdsx_op_dn9 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn9)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45490_e50660)))) * assign45490_e50668) + (assign45490_e50663 * (locals.var_cfdl_i * locals.var_dleff_op_dn9))) * assign45490_e50674) + (assign45490_e50669 * (locals.var_cfdlb_i * locals.var_xg20_op_dn9))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign45490_e50677;
        locals.var_temp_dn4 = assign45490_e50677_d_n4;
        locals.var_temp_dn6 = assign45490_e50677_d_n6;
        locals.var_temp_dn7 = assign45490_e50677_d_n7;
        locals.var_temp_dn8 = assign45490_e50677_d_n8;
        locals.var_temp_dn9 = assign45490_e50677_d_n9;

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign00_e799: f64 = (273.15 + p.p15);
        locals.var_tkr = assign00_e799;
        locals.var_tkr_rv = 0.0;

        let assign10_e800: f64 = ctx_temp;
        let assign10_e802: f64 = (assign10_e800 + p.p36);
        let assign10_e804: f64 = (assign10_e802).min(1000.0);
        locals.var_temp = assign10_e804;
        locals.var_temp_dn4 = 0.0;
        locals.var_temp_dn6 = 0.0;
        locals.var_temp_dn7 = 0.0;
        locals.var_temp_dn8 = 0.0;
        locals.var_temp_dn9 = 0.0;
        locals.var_temp_rv = 0.0;

        let assign20_e807: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign20_e807;
        locals.var_guard1_rv = 0.0;

        let (assign30_e838, assign30_e838_d_n4, assign30_e838_d_n6, assign30_e838_d_n7, assign30_e838_d_n8, assign30_e838_d_n9,) = {
    if (locals.var_guard1 != 0.0) {
        let assign30_e814: f64 = (p.p18 * locals.var_temp);
        let assign30_e815: f64 = (p.p17 + assign30_e814);
        let assign30_e816: f64 = (locals.var_temp + assign30_e815);
        let assign30_e821: f64 = (p.p18 * locals.var_temp);
        let assign30_e822: f64 = (p.p17 + assign30_e821);
        let assign30_e823: f64 = (locals.var_temp - assign30_e822);
        let assign30_e828: f64 = (p.p18 * locals.var_temp);
        let assign30_e829: f64 = (p.p17 + assign30_e828);
        let assign30_e830: f64 = (locals.var_temp - assign30_e829);
        let assign30_e831: f64 = (assign30_e823 * assign30_e830);
        let assign30_e833: f64 = (assign30_e831 + p.p19);
        let assign30_e834: f64 = (assign30_e833).sqrt();
        let assign30_e835: f64 = (assign30_e816 + assign30_e834);
        let assign30_e836: f64 = (0.5 * assign30_e835);
        (assign30_e836, (0.5 * ((locals.var_temp_dn4 + (p.p18 * locals.var_temp_dn4)) + ((((locals.var_temp_dn4 - (p.p18 * locals.var_temp_dn4)) * assign30_e830) + (assign30_e823 * (locals.var_temp_dn4 - (p.p18 * locals.var_temp_dn4)))) / (2.0 * assign30_e834)))), (0.5 * ((locals.var_temp_dn6 + (p.p18 * locals.var_temp_dn6)) + ((((locals.var_temp_dn6 - (p.p18 * locals.var_temp_dn6)) * assign30_e830) + (assign30_e823 * (locals.var_temp_dn6 - (p.p18 * locals.var_temp_dn6)))) / (2.0 * assign30_e834)))), (0.5 * ((locals.var_temp_dn7 + (p.p18 * locals.var_temp_dn7)) + ((((locals.var_temp_dn7 - (p.p18 * locals.var_temp_dn7)) * assign30_e830) + (assign30_e823 * (locals.var_temp_dn7 - (p.p18 * locals.var_temp_dn7)))) / (2.0 * assign30_e834)))), (0.5 * ((locals.var_temp_dn8 + (p.p18 * locals.var_temp_dn8)) + ((((locals.var_temp_dn8 - (p.p18 * locals.var_temp_dn8)) * assign30_e830) + (assign30_e823 * (locals.var_temp_dn8 - (p.p18 * locals.var_temp_dn8)))) / (2.0 * assign30_e834)))), (0.5 * ((locals.var_temp_dn9 + (p.p18 * locals.var_temp_dn9)) + ((((locals.var_temp_dn9 - (p.p18 * locals.var_temp_dn9)) * assign30_e830) + (assign30_e823 * (locals.var_temp_dn9 - (p.p18 * locals.var_temp_dn9)))) / (2.0 * assign30_e834)))),)
    } else {
        (locals.var_tkd, locals.var_tkd_dn4, locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9,)
    }
};
        locals.var_tkd = assign30_e838;
        locals.var_tkd_dn4 = assign30_e838_d_n4;
        locals.var_tkd_dn6 = assign30_e838_d_n6;
        locals.var_tkd_dn7 = assign30_e838_d_n7;
        locals.var_tkd_dn8 = assign30_e838_d_n8;
        locals.var_tkd_dn9 = assign30_e838_d_n9;
        locals.var_tkd_rv = 0.0;

        let (assign40_e869, assign40_e869_d_n4, assign40_e869_d_n6, assign40_e869_d_n7, assign40_e869_d_n8, assign40_e869_d_n9,) = {
    if (locals.var_guard1 != 0.0) {
        let assign40_e844: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign40_e845: f64 = (10.0 / assign40_e844);
        let assign40_e847: f64 = (assign40_e845 + 600.0);
        let assign40_e851: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign40_e852: f64 = (10.0 / assign40_e851);
        let assign40_e854: f64 = (assign40_e852 - 600.0);
        let assign40_e858: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign40_e859: f64 = (10.0 / assign40_e858);
        let assign40_e861: f64 = (assign40_e859 - 600.0);
        let assign40_e862: f64 = (assign40_e854 * assign40_e861);
        let assign40_e864: f64 = (assign40_e862 + 0.01);
        let assign40_e865: f64 = (assign40_e864).sqrt();
        let assign40_e866: f64 = (assign40_e847 + assign40_e865);
        let assign40_e867: f64 = (0.5 * assign40_e866);
        (assign40_e867, (0.5 * ((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e844 * assign40_e844))) + ((((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e851 * assign40_e851))) * assign40_e861) + (assign40_e854 * (-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e858 * assign40_e858))))) / (2.0 * assign40_e865)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e844 * assign40_e844))) + ((((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e851 * assign40_e851))) * assign40_e861) + (assign40_e854 * (-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e858 * assign40_e858))))) / (2.0 * assign40_e865)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e844 * assign40_e844))) + ((((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e851 * assign40_e851))) * assign40_e861) + (assign40_e854 * (-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e858 * assign40_e858))))) / (2.0 * assign40_e865)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e844 * assign40_e844))) + ((((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e851 * assign40_e851))) * assign40_e861) + (assign40_e854 * (-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e858 * assign40_e858))))) / (2.0 * assign40_e865)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e844 * assign40_e844))) + ((((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e851 * assign40_e851))) * assign40_e861) + (assign40_e854 * (-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e858 * assign40_e858))))) / (2.0 * assign40_e865)))),)
    } else {
        (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9,)
    }
};
        locals.var_xsatmax = assign40_e869;
        locals.var_xsatmax_dn4 = assign40_e869_d_n4;
        locals.var_xsatmax_dn6 = assign40_e869_d_n6;
        locals.var_xsatmax_dn7 = assign40_e869_d_n7;
        locals.var_xsatmax_dn8 = assign40_e869_d_n8;
        locals.var_xsatmax_dn9 = assign40_e869_d_n9;
        locals.var_xsatmax_rv = 0.0;

        let (assign50_e889, assign50_e889_d_n4, assign50_e889_d_n6, assign50_e889_d_n7, assign50_e889_d_n8, assign50_e889_d_n9,) = {
    if (locals.var_guard1 == 0.0) {
        let assign50_e875: f64 = (locals.var_temp + 1.0);
        let assign50_e878: f64 = (locals.var_temp - 1.0);
        let assign50_e881: f64 = (locals.var_temp - 1.0);
        let assign50_e882: f64 = (assign50_e878 * assign50_e881);
        let assign50_e884: f64 = (assign50_e882 + 0.001);
        let assign50_e885: f64 = (assign50_e884).sqrt();
        let assign50_e886: f64 = (assign50_e875 + assign50_e885);
        let assign50_e887: f64 = (0.5 * assign50_e886);
        (assign50_e887, (0.5 * (locals.var_temp_dn4 + (((locals.var_temp_dn4 * assign50_e881) + (assign50_e878 * locals.var_temp_dn4)) / (2.0 * assign50_e885)))), (0.5 * (locals.var_temp_dn6 + (((locals.var_temp_dn6 * assign50_e881) + (assign50_e878 * locals.var_temp_dn6)) / (2.0 * assign50_e885)))), (0.5 * (locals.var_temp_dn7 + (((locals.var_temp_dn7 * assign50_e881) + (assign50_e878 * locals.var_temp_dn7)) / (2.0 * assign50_e885)))), (0.5 * (locals.var_temp_dn8 + (((locals.var_temp_dn8 * assign50_e881) + (assign50_e878 * locals.var_temp_dn8)) / (2.0 * assign50_e885)))), (0.5 * (locals.var_temp_dn9 + (((locals.var_temp_dn9 * assign50_e881) + (assign50_e878 * locals.var_temp_dn9)) / (2.0 * assign50_e885)))),)
    } else {
        (locals.var_tkd, locals.var_tkd_dn4, locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9,)
    }
};
        locals.var_tkd = assign50_e889;
        locals.var_tkd_dn4 = assign50_e889_d_n4;
        locals.var_tkd_dn6 = assign50_e889_d_n6;
        locals.var_tkd_dn7 = assign50_e889_d_n7;
        locals.var_tkd_dn8 = assign50_e889_d_n8;
        locals.var_tkd_dn9 = assign50_e889_d_n9;
        locals.var_tkd_rv = 0.0;

        let (assign60_e894, assign60_e894_d_n4, assign60_e894_d_n6, assign60_e894_d_n7, assign60_e894_d_n8, assign60_e894_d_n9,) = {
    if (locals.var_guard1 == 0.0) {
        (600.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9,)
    }
};
        locals.var_xsatmax = assign60_e894;
        locals.var_xsatmax_dn4 = assign60_e894_d_n4;
        locals.var_xsatmax_dn6 = assign60_e894_d_n6;
        locals.var_xsatmax_dn7 = assign60_e894_d_n7;
        locals.var_xsatmax_dn8 = assign60_e894_d_n8;
        locals.var_xsatmax_dn9 = assign60_e894_d_n9;
        locals.var_xsatmax_rv = 0.0;

        let assign70_e909: f64 = if (((p.p0 == 0.0) && (p.p172 > 0.0)) || ((p.p0 > 0.0) && (p.p443 > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard2 = assign70_e909;
        locals.var_guard2_rv = 0.0;

        let (assign80_e913,) = {
    if (locals.var_guard2 != 0.0) {
        (p.p5,)
    } else {
        (locals.var_swshe_i,)
    }
};
        locals.var_swshe_i = assign80_e913;
        locals.var_swshe_i_rv = 0.0;

        let (assign90_e918,) = {
    if (locals.var_guard2 == 0.0) {
        (0.0,)
    } else {
        (locals.var_swshe_i,)
    }
};
        locals.var_swshe_i = assign90_e918;
        locals.var_swshe_i_rv = 0.0;

        locals.var_dtc = 0.0;
        locals.var_dtc_dn4 = 0.0;
        locals.var_dtc_rv = 0.0;

        locals.var_tkc = locals.var_tkd;
        locals.var_tkc_dn4 = locals.var_tkd_dn4;
        locals.var_tkc_dn6 = locals.var_tkd_dn6;
        locals.var_tkc_dn7 = locals.var_tkd_dn7;
        locals.var_tkc_dn8 = locals.var_tkd_dn8;
        locals.var_tkc_dn9 = locals.var_tkd_dn9;
        locals.var_tkc_rv = 0.0;

        let assign140_e928: f64 = (locals.var_tkc * locals.var_tkc);
        locals.var_tkc_sq = assign140_e928;
        locals.var_tkc_sq_dn4 = ((locals.var_tkc_dn4 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn4));
        locals.var_tkc_sq_dn6 = ((locals.var_tkc_dn6 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn6));
        locals.var_tkc_sq_dn7 = ((locals.var_tkc_dn7 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn7));
        locals.var_tkc_sq_dn8 = ((locals.var_tkc_dn8 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn8));
        locals.var_tkc_sq_dn9 = ((locals.var_tkc_dn9 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn9));
        locals.var_tkc_sq_rv = 0.0;

        let assign150_e931: f64 = (locals.var_tkc - locals.var_tkr);
        locals.var_dt = assign150_e931;
        locals.var_dt_dn4 = locals.var_tkc_dn4;
        locals.var_dt_dn6 = locals.var_tkc_dn6;
        locals.var_dt_dn7 = locals.var_tkc_dn7;
        locals.var_dt_dn8 = locals.var_tkc_dn8;
        locals.var_dt_dn9 = locals.var_tkc_dn9;
        locals.var_dt_rv = 0.0;

        let assign160_e934: f64 = (locals.var_tkc / locals.var_tkr);
        locals.var_rt = assign160_e934;
        locals.var_rt_dn4 = (locals.var_tkc_dn4 / locals.var_tkr);
        locals.var_rt_dn6 = (locals.var_tkc_dn6 / locals.var_tkr);
        locals.var_rt_dn7 = (locals.var_tkc_dn7 / locals.var_tkr);
        locals.var_rt_dn8 = (locals.var_tkc_dn8 / locals.var_tkr);
        locals.var_rt_dn9 = (locals.var_tkc_dn9 / locals.var_tkr);
        locals.var_rt_rv = 0.0;

        let assign170_e937: f64 = (locals.var_tkr / locals.var_tkc);
        locals.var_rtn = assign170_e937;
        locals.var_rtn_dn4 = (-((locals.var_tkr * locals.var_tkc_dn4) / (locals.var_tkc * locals.var_tkc)));
        locals.var_rtn_dn6 = (-((locals.var_tkr * locals.var_tkc_dn6) / (locals.var_tkc * locals.var_tkc)));
        locals.var_rtn_dn7 = (-((locals.var_tkr * locals.var_tkc_dn7) / (locals.var_tkc * locals.var_tkc)));
        locals.var_rtn_dn8 = (-((locals.var_tkr * locals.var_tkc_dn8) / (locals.var_tkc * locals.var_tkc)));
        locals.var_rtn_dn9 = (-((locals.var_tkr * locals.var_tkc_dn9) / (locals.var_tkc * locals.var_tkc)));
        locals.var_rtn_rv = 0.0;

        let assign180_e940: f64 = (locals.var_tkc * 8.617332384961e-5);
        locals.var_phit0 = assign180_e940;
        locals.var_phit0_dn4 = (locals.var_tkc_dn4 * 8.617332384961e-5);
        locals.var_phit0_dn6 = (locals.var_tkc_dn6 * 8.617332384961e-5);
        locals.var_phit0_dn7 = (locals.var_tkc_dn7 * 8.617332384961e-5);
        locals.var_phit0_dn8 = (locals.var_tkc_dn8 * 8.617332384961e-5);
        locals.var_phit0_dn9 = (locals.var_tkc_dn9 * 8.617332384961e-5);
        locals.var_phit0_rv = 0.0;

        let assign190_e943: f64 = (1.0 / locals.var_phit0);
        locals.var_inv_phit0 = assign190_e943;
        locals.var_inv_phit0_dn4 = (-(locals.var_phit0_dn4 / (locals.var_phit0 * locals.var_phit0)));
        locals.var_inv_phit0_dn6 = (-(locals.var_phit0_dn6 / (locals.var_phit0 * locals.var_phit0)));
        locals.var_inv_phit0_dn7 = (-(locals.var_phit0_dn7 / (locals.var_phit0 * locals.var_phit0)));
        locals.var_inv_phit0_dn8 = (-(locals.var_phit0_dn8 / (locals.var_phit0 * locals.var_phit0)));
        locals.var_inv_phit0_dn9 = (-(locals.var_phit0_dn9 / (locals.var_phit0 * locals.var_phit0)));
        locals.var_inv_phit0_rv = 0.0;

        let assign200_e946: f64 = if p.p0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign200_e946;
        locals.var_guard83_rv = 0.0;

        let (assign210_e950,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p23,)
    } else {
        (locals.var_adrain_i,)
    }
};
        locals.var_adrain_i = assign210_e950;
        locals.var_adrain_i_rv = 0.0;

        let (assign220_e954,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p22,)
    } else {
        (locals.var_asource_i,)
    }
};
        locals.var_asource_i = assign220_e954;
        locals.var_asource_i_rv = 0.0;

        let (assign230_e958,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p25,)
    } else {
        (locals.var_pdrain_i,)
    }
};
        locals.var_pdrain_i = assign230_e958;
        locals.var_pdrain_i_rv = 0.0;

        let (assign240_e962,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p24,)
    } else {
        (locals.var_psource_i,)
    }
};
        locals.var_psource_i = assign240_e962;
        locals.var_psource_i_rv = 0.0;

        let (assign250_e966,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p30,)
    } else {
        (locals.var_mult_i_int,)
    }
};
        locals.var_mult_i_int = assign250_e966;
        locals.var_mult_i_int_rv = 0.0;

        let (assign260_e970,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p41,)
    } else {
        (locals.var_tox1_i,)
    }
};
        locals.var_tox1_i = assign260_e970;
        locals.var_tox1_i_rv = 0.0;

        let (assign270_e974,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_tsi_i,)
    }
};
        locals.var_tsi_i = assign270_e974;
        locals.var_tsi_i_rv = 0.0;

        let (assign280_e978,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p43,)
    } else {
        (locals.var_xge_i,)
    }
};
        locals.var_xge_i = assign280_e978;
        locals.var_xge_i_rv = 0.0;

        let (assign290_e982,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p44,)
    } else {
        (locals.var_tox2_i,)
    }
};
        locals.var_tox2_i = assign290_e982;
        locals.var_tox2_i_rv = 0.0;

        let (assign300_e986,) = {
    if (locals.var_guard83 != 0.0) {
        (1.0,)
    } else {
        (locals.var_typech_i,)
    }
};
        locals.var_typech_i = assign300_e986;
        locals.var_typech_i_rv = 0.0;

        let assign310_e989: f64 = if p.p45 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign310_e989;
        locals.var_guard84_rv = 0.0;

        let (assign320_e996,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard84 != 0.0)) {
        let assign320_e994: f64 = (-1.0);
        (assign320_e994,)
    } else {
        (locals.var_typech_i,)
    }
};
        locals.var_typech_i = assign320_e996;
        locals.var_typech_i_rv = 0.0;

        let (assign330_e1005,) = {
    if (locals.var_guard83 != 0.0) {
        let assign330_e999: f64 = (p.p45).abs();
        let assign330_e1001: f64 = (assign330_e999).min(1e19);
        let assign330_e1003: f64 = (assign330_e1001 * 1000000.0);
        (assign330_e1003,)
    } else {
        (locals.var_nch_i,)
    }
};
        locals.var_nch_i = assign330_e1005;
        locals.var_nch_i_rv = 0.0;

        let (assign340_e1009,) = {
    if (locals.var_guard83 != 0.0) {
        (1.0,)
    } else {
        (locals.var_typesub_i,)
    }
};
        locals.var_typesub_i = assign340_e1009;
        locals.var_typesub_i_rv = 0.0;

        let assign350_e1012: f64 = if p.p46 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard85 = assign350_e1012;
        locals.var_guard85_rv = 0.0;

        let (assign360_e1019,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign360_e1017: f64 = (-1.0);
        (assign360_e1017,)
    } else {
        (locals.var_typesub_i,)
    }
};
        locals.var_typesub_i = assign360_e1019;
        locals.var_typesub_i_rv = 0.0;

        let (assign370_e1030,) = {
    if (locals.var_guard83 != 0.0) {
        let assign370_e1022: f64 = (p.p46).abs();
        let assign370_e1024: f64 = (assign370_e1022).max(1e16);
        let assign370_e1026: f64 = (assign370_e1024).min(1e21);
        let assign370_e1028: f64 = (assign370_e1026 * 1000000.0);
        (assign370_e1028,)
    } else {
        (locals.var_nsub_i,)
    }
};
        locals.var_nsub_i = assign370_e1030;
        locals.var_nsub_i_rv = 0.0;

        let (assign380_e1034,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_ct_i,)
    }
};
        locals.var_ct_i = assign380_e1034;
        locals.var_ct_i_rv = 0.0;

        let (assign390_e1038,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p48,)
    } else {
        (locals.var_toxp_i,)
    }
};
        locals.var_toxp_i = assign390_e1038;
        locals.var_toxp_i_rv = 0.0;

        let (assign400_e1044,) = {
    if (locals.var_guard83 != 0.0) {
        let assign400_e1042: f64 = (p.p49 * 1000000.0);
        (assign400_e1042,)
    } else {
        (locals.var_nov_i,)
    }
};
        locals.var_nov_i = assign400_e1044;
        locals.var_nov_i_rv = 0.0;

        let (assign410_e1050,) = {
    if (locals.var_guard83 != 0.0) {
        let assign410_e1048: f64 = (p.p50 * 1000000.0);
        (assign410_e1048,)
    } else {
        (locals.var_novd_i,)
    }
};
        locals.var_novd_i = assign410_e1050;
        locals.var_novd_i_rv = 0.0;

        let (assign420_e1054, assign420_e1054_d_n4, assign420_e1054_d_n6, assign420_e1054_d_n7, assign420_e1054_d_n8, assign420_e1054_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p51, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9,)
    }
};
        locals.var_vfb1_t = assign420_e1054;
        locals.var_vfb1_t_dn4 = assign420_e1054_d_n4;
        locals.var_vfb1_t_dn6 = assign420_e1054_d_n6;
        locals.var_vfb1_t_dn7 = assign420_e1054_d_n7;
        locals.var_vfb1_t_dn8 = assign420_e1054_d_n8;
        locals.var_vfb1_t_dn9 = assign420_e1054_d_n9;
        locals.var_vfb1_t_rv = 0.0;

        let (assign430_e1058, assign430_e1058_d_n4, assign430_e1058_d_n6, assign430_e1058_d_n7, assign430_e1058_d_n8, assign430_e1058_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p52, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9,)
    }
};
        locals.var_vfb2_t = assign430_e1058;
        locals.var_vfb2_t_dn4 = assign430_e1058_d_n4;
        locals.var_vfb2_t_dn6 = assign430_e1058_d_n6;
        locals.var_vfb2_t_dn7 = assign430_e1058_d_n7;
        locals.var_vfb2_t_dn8 = assign430_e1058_d_n8;
        locals.var_vfb2_t_dn9 = assign430_e1058_d_n9;
        locals.var_vfb2_t_rv = 0.0;

        let (assign440_e1062,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p53,)
    } else {
        (locals.var_stvfb_i,)
    }
};
        locals.var_stvfb_i = assign440_e1062;
        locals.var_stvfb_i_rv = 0.0;

        let (assign450_e1068, assign450_e1068_d_n4, assign450_e1068_d_n6, assign450_e1068_d_n7, assign450_e1068_d_n8, assign450_e1068_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        let assign450_e1066: f64 = (p.p54 * 1000000.0);
        (assign450_e1066, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_np_i, locals.var_np_i_dn4, locals.var_np_i_dn6, locals.var_np_i_dn7, locals.var_np_i_dn8, locals.var_np_i_dn9,)
    }
};
        locals.var_np_i = assign450_e1068;
        locals.var_np_i_dn4 = assign450_e1068_d_n4;
        locals.var_np_i_dn6 = assign450_e1068_d_n6;
        locals.var_np_i_dn7 = assign450_e1068_d_n7;
        locals.var_np_i_dn8 = assign450_e1068_d_n8;
        locals.var_np_i_dn9 = assign450_e1068_d_n9;
        locals.var_np_i_rv = 0.0;

        let (assign460_e1072,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p55,)
    } else {
        (locals.var_cic1_i,)
    }
};
        locals.var_cic1_i = assign460_e1072;
        locals.var_cic1_i_rv = 0.0;

        let (assign470_e1076,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p56,)
    } else {
        (locals.var_cic2_i,)
    }
};
        locals.var_cic2_i = assign470_e1076;
        locals.var_cic2_i_rv = 0.0;

        let (assign480_e1080,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p57,)
    } else {
        (locals.var_psce1_i,)
    }
};
        locals.var_psce1_i = assign480_e1080;
        locals.var_psce1_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign490_e1090,) = {
    if (locals.var_guard83 != 0.0) {
        let assign490_e1084: f64 = (p.p58 * locals.var_psce1_i);
        let assign490_e1086: f64 = (assign490_e1084 * locals.var_tox2_i);
        let assign490_e1088: f64 = (assign490_e1086 / locals.var_tox1_i);
        (assign490_e1088,)
    } else {
        (locals.var_psce2_i,)
    }
};
        locals.var_psce2_i = assign490_e1090;
        locals.var_psce2_i_rv = 0.0;

        let (assign500_e1096,) = {
    if (locals.var_guard83 != 0.0) {
        let assign500_e1094: f64 = (p.p59 * 1000000.0);
        (assign500_e1094,)
    } else {
        (locals.var_nsddc_i,)
    }
};
        locals.var_nsddc_i = assign500_e1096;
        locals.var_nsddc_i_rv = 0.0;

        let (assign510_e1100,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p60,)
    } else {
        (locals.var_pscedlb_i,)
    }
};
        locals.var_pscedlb_i = assign510_e1100;
        locals.var_pscedlb_i_rv = 0.0;

        let (assign520_e1104,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p61,)
    } else {
        (locals.var_pnce_i,)
    }
};
        locals.var_pnce_i = assign520_e1104;
        locals.var_pnce_i_rv = 0.0;

        let (assign530_e1108, assign530_e1108_d_n4, assign530_e1108_d_n6, assign530_e1108_d_n7, assign530_e1108_d_n8, assign530_e1108_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p62, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9,)
    }
};
        locals.var_cf1_t = assign530_e1108;
        locals.var_cf1_t_dn4 = assign530_e1108_d_n4;
        locals.var_cf1_t_dn6 = assign530_e1108_d_n6;
        locals.var_cf1_t_dn7 = assign530_e1108_d_n7;
        locals.var_cf1_t_dn8 = assign530_e1108_d_n8;
        locals.var_cf1_t_dn9 = assign530_e1108_d_n9;
        locals.var_cf1_t_rv = 0.0;

        let (assign540_e1118, assign540_e1118_d_n4, assign540_e1118_d_n6, assign540_e1118_d_n7, assign540_e1118_d_n8, assign540_e1118_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        let assign540_e1112: f64 = (p.p63 * locals.var_cf1_t);
        let assign540_e1114: f64 = (assign540_e1112 * locals.var_tox2_i);
        let assign540_e1116: f64 = (assign540_e1114 / locals.var_tox1_i);
        (assign540_e1116, (((p.p63 * locals.var_cf1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9,)
    }
};
        locals.var_cf2_t = assign540_e1118;
        locals.var_cf2_t_dn4 = assign540_e1118_d_n4;
        locals.var_cf2_t_dn6 = assign540_e1118_d_n6;
        locals.var_cf2_t_dn7 = assign540_e1118_d_n7;
        locals.var_cf2_t_dn8 = assign540_e1118_d_n8;
        locals.var_cf2_t_dn9 = assign540_e1118_d_n9;
        locals.var_cf2_t_rv = 0.0;

        let (assign550_e1122, assign550_e1122_d_n4, assign550_e1122_d_n6, assign550_e1122_d_n7, assign550_e1122_d_n8, assign550_e1122_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p64, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_stcf_i, locals.var_stcf_i_dn4, locals.var_stcf_i_dn6, locals.var_stcf_i_dn7, locals.var_stcf_i_dn8, locals.var_stcf_i_dn9,)
    }
};
        locals.var_stcf_i = assign550_e1122;
        locals.var_stcf_i_dn4 = assign550_e1122_d_n4;
        locals.var_stcf_i_dn6 = assign550_e1122_d_n6;
        locals.var_stcf_i_dn7 = assign550_e1122_d_n7;
        locals.var_stcf_i_dn8 = assign550_e1122_d_n8;
        locals.var_stcf_i_dn9 = assign550_e1122_d_n9;
        locals.var_stcf_i_rv = 0.0;

        let (assign560_e1126,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p65,)
    } else {
        (locals.var_cfd_i,)
    }
};
        locals.var_cfd_i = assign560_e1126;
        locals.var_cfd_i_rv = 0.0;

        let (assign570_e1130,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p66,)
    } else {
        (locals.var_cfdl_i,)
    }
};
        locals.var_cfdl_i = assign570_e1130;
        locals.var_cfdl_i_rv = 0.0;

        let (assign580_e1134,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p67,)
    } else {
        (locals.var_cfdlb_i,)
    }
};
        locals.var_cfdlb_i = assign580_e1134;
        locals.var_cfdlb_i_rv = 0.0;

        let (assign590_e1138, assign590_e1138_d_n4, assign590_e1138_d_n6, assign590_e1138_d_n7, assign590_e1138_d_n8, assign590_e1138_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p68, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9,)
    }
};
        locals.var_betn1_t = assign590_e1138;
        locals.var_betn1_t_dn4 = assign590_e1138_d_n4;
        locals.var_betn1_t_dn6 = assign590_e1138_d_n6;
        locals.var_betn1_t_dn7 = assign590_e1138_d_n7;
        locals.var_betn1_t_dn8 = assign590_e1138_d_n8;
        locals.var_betn1_t_dn9 = assign590_e1138_d_n9;
        locals.var_betn1_t_rv = 0.0;

        let (assign600_e1144, assign600_e1144_d_n4, assign600_e1144_d_n6, assign600_e1144_d_n7, assign600_e1144_d_n8, assign600_e1144_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        let assign600_e1142: f64 = (p.p69 * locals.var_betn1_t);
        (assign600_e1142, (p.p69 * locals.var_betn1_t_dn4), (p.p69 * locals.var_betn1_t_dn6), (p.p69 * locals.var_betn1_t_dn7), (p.p69 * locals.var_betn1_t_dn8), (p.p69 * locals.var_betn1_t_dn9),)
    } else {
        (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9,)
    }
};
        locals.var_betn2_t = assign600_e1144;
        locals.var_betn2_t_dn4 = assign600_e1144_d_n4;
        locals.var_betn2_t_dn6 = assign600_e1144_d_n6;
        locals.var_betn2_t_dn7 = assign600_e1144_d_n7;
        locals.var_betn2_t_dn8 = assign600_e1144_d_n8;
        locals.var_betn2_t_dn9 = assign600_e1144_d_n9;
        locals.var_betn2_t_rv = 0.0;

        let (assign610_e1148,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p70,)
    } else {
        (locals.var_stbet_i,)
    }
};
        locals.var_stbet_i = assign610_e1148;
        locals.var_stbet_i_rv = 0.0;

        let (assign620_e1152,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p71,)
    } else {
        (locals.var_cs_t,)
    }
};
        locals.var_cs_t = assign620_e1152;
        locals.var_cs_t_rv = 0.0;

        let (assign630_e1156,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p72,)
    } else {
        (locals.var_csfi_i,)
    }
};
        locals.var_csfi_i = assign630_e1156;
        locals.var_csfi_i_rv = 0.0;

        let (assign640_e1160,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p73,)
    } else {
        (locals.var_csbi_i,)
    }
};
        locals.var_csbi_i = assign640_e1160;
        locals.var_csbi_i_rv = 0.0;

        let (assign650_e1164,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p74,)
    } else {
        (locals.var_stcs_i,)
    }
};
        locals.var_stcs_i = assign650_e1164;
        locals.var_stcs_i_rv = 0.0;

        let (assign660_e1168,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p75,)
    } else {
        (locals.var_thecs_t,)
    }
};
        locals.var_thecs_t = assign660_e1168;
        locals.var_thecs_t_rv = 0.0;

        let (assign670_e1172,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p76,)
    } else {
        (locals.var_stthecs_i,)
    }
};
        locals.var_stthecs_i = assign670_e1172;
        locals.var_stthecs_i_rv = 0.0;

        let (assign680_e1176,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p77,)
    } else {
        (locals.var_csthr_i,)
    }
};
        locals.var_csthr_i = assign680_e1176;
        locals.var_csthr_i_rv = 0.0;

        let (assign690_e1180,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p78,)
    } else {
        (locals.var_csthrb_i,)
    }
};
        locals.var_csthrb_i = assign690_e1180;
        locals.var_csthrb_i_rv = 0.0;

        let (assign700_e1184,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p79,)
    } else {
        (locals.var_mue_t,)
    }
};
        locals.var_mue_t = assign700_e1184;
        locals.var_mue_t_rv = 0.0;

        let (assign710_e1188,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p80,)
    } else {
        (locals.var_stmue_i,)
    }
};
        locals.var_stmue_i = assign710_e1188;
        locals.var_stmue_i_rv = 0.0;

        let (assign720_e1192,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p81,)
    } else {
        (locals.var_themu_t,)
    }
};
        locals.var_themu_t = assign720_e1192;
        locals.var_themu_t_rv = 0.0;

        let (assign730_e1196,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p82,)
    } else {
        (locals.var_stthemu_i,)
    }
};
        locals.var_stthemu_i = assign730_e1196;
        locals.var_stthemu_i_rv = 0.0;

        let (assign740_e1200,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p83,)
    } else {
        (locals.var_xcor_t,)
    }
};
        locals.var_xcor_t = assign740_e1200;
        locals.var_xcor_t_rv = 0.0;

        let (assign750_e1204,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p84,)
    } else {
        (locals.var_xcorb_i,)
    }
};
        locals.var_xcorb_i = assign750_e1204;
        locals.var_xcorb_i_rv = 0.0;

        let (assign760_e1208,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p85,)
    } else {
        (locals.var_stxcor_i,)
    }
};
        locals.var_stxcor_i = assign760_e1208;
        locals.var_stxcor_i_rv = 0.0;

        let (assign770_e1212,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p86,)
    } else {
        (locals.var_feta_i,)
    }
};
        locals.var_feta_i = assign770_e1212;
        locals.var_feta_i_rv = 0.0;

        let (assign780_e1216,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p87,)
    } else {
        (locals.var_rs_t,)
    }
};
        locals.var_rs_t = assign780_e1216;
        locals.var_rs_t_rv = 0.0;

        let (assign790_e1220,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p88,)
    } else {
        (locals.var_rsig_i,)
    }
};
        locals.var_rsig_i = assign790_e1220;
        locals.var_rsig_i_rv = 0.0;

        let (assign800_e1224,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p89,)
    } else {
        (locals.var_strs_i,)
    }
};
        locals.var_strs_i = assign800_e1224;
        locals.var_strs_i_rv = 0.0;

        let (assign810_e1228,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p90,)
    } else {
        (locals.var_rsg_i,)
    }
};
        locals.var_rsg_i = assign810_e1228;
        locals.var_rsg_i_rv = 0.0;

        let (assign820_e1232,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p91,)
    } else {
        (locals.var_thersg_i,)
    }
};
        locals.var_thersg_i = assign820_e1232;
        locals.var_thersg_i_rv = 0.0;

        let (assign830_e1236,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p92,)
    } else {
        (locals.var_rsb_i,)
    }
};
        locals.var_rsb_i = assign830_e1236;
        locals.var_rsb_i_rv = 0.0;

        let (assign840_e1240, assign840_e1240_d_n4, assign840_e1240_d_n6, assign840_e1240_d_n7, assign840_e1240_d_n8, assign840_e1240_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p93, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9,)
    }
};
        locals.var_thesat_t = assign840_e1240;
        locals.var_thesat_t_dn4 = assign840_e1240_d_n4;
        locals.var_thesat_t_dn6 = assign840_e1240_d_n6;
        locals.var_thesat_t_dn7 = assign840_e1240_d_n7;
        locals.var_thesat_t_dn8 = assign840_e1240_d_n8;
        locals.var_thesat_t_dn9 = assign840_e1240_d_n9;
        locals.var_thesat_t_rv = 0.0;

        let (assign850_e1244,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p94,)
    } else {
        (locals.var_stthesat_i,)
    }
};
        locals.var_stthesat_i = assign850_e1244;
        locals.var_stthesat_i_rv = 0.0;

        let (assign860_e1248,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p95,)
    } else {
        (locals.var_thesat1_i,)
    }
};
        locals.var_thesat1_i = assign860_e1248;
        locals.var_thesat1_i_rv = 0.0;

        let (assign870_e1252,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p96,)
    } else {
        (locals.var_thesat2_i,)
    }
};
        locals.var_thesat2_i = assign870_e1252;
        locals.var_thesat2_i_rv = 0.0;

        let (assign880_e1256,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p97,)
    } else {
        (locals.var_ax_i,)
    }
};
        locals.var_ax_i = assign880_e1256;
        locals.var_ax_i_rv = 0.0;

        let (assign890_e1260,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p98,)
    } else {
        (locals.var_alp_i,)
    }
};
        locals.var_alp_i = assign890_e1260;
        locals.var_alp_i_rv = 0.0;

        let (assign900_e1264,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p99,)
    } else {
        (locals.var_alp1_i,)
    }
};
        locals.var_alp1_i = assign900_e1264;
        locals.var_alp1_i_rv = 0.0;

        let (assign910_e1268,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p100,)
    } else {
        (locals.var_alpb_i,)
    }
};
        locals.var_alpb_i = assign910_e1268;
        locals.var_alpb_i_rv = 0.0;

        let (assign920_e1272,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p101,)
    } else {
        (locals.var_vp_i,)
    }
};
        locals.var_vp_i = assign920_e1272;
        locals.var_vp_i_rv = 0.0;

        let (assign930_e1276,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p102,)
    } else {
        (locals.var_vpg_i,)
    }
};
        locals.var_vpg_i = assign930_e1276;
        locals.var_vpg_i_rv = 0.0;

        let (assign940_e1280,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p103,)
    } else {
        (locals.var_gco_i,)
    }
};
        locals.var_gco_i = assign940_e1280;
        locals.var_gco_i_rv = 0.0;

        let (assign950_e1284,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p104,)
    } else {
        (locals.var_iginv_t,)
    }
};
        locals.var_iginv_t = assign950_e1284;
        locals.var_iginv_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign960_e1288,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p105,)
    } else {
        (locals.var_igovinv_t,)
    }
};
        locals.var_igovinv_t = assign960_e1288;
        locals.var_igovinv_t_rv = 0.0;

        let (assign970_e1292,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p106,)
    } else {
        (locals.var_igovinvd_t,)
    }
};
        locals.var_igovinvd_t = assign970_e1292;
        locals.var_igovinvd_t_rv = 0.0;

        let (assign1000_e1304,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p107,)
    } else {
        (locals.var_igovacc_t,)
    }
};
        locals.var_igovacc_t = assign1000_e1304;
        locals.var_igovacc_t_rv = 0.0;

        let (assign1010_e1308,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p108,)
    } else {
        (locals.var_igovaccd_t,)
    }
};
        locals.var_igovaccd_t = assign1010_e1308;
        locals.var_igovaccd_t_rv = 0.0;

        let (assign1020_e1312,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p109,)
    } else {
        (locals.var_stig_i,)
    }
};
        locals.var_stig_i = assign1020_e1312;
        locals.var_stig_i_rv = 0.0;

        let (assign1030_e1316,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p123,)
    } else {
        (locals.var_stigfn_i,)
    }
};
        locals.var_stigfn_i = assign1030_e1316;
        locals.var_stigfn_i_rv = 0.0;

        let (assign1040_e1320,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p110,)
    } else {
        (locals.var_gc2ch_i,)
    }
};
        locals.var_gc2ch_i = assign1040_e1320;
        locals.var_gc2ch_i_rv = 0.0;

        let (assign1050_e1324,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p111,)
    } else {
        (locals.var_gc3ch_i,)
    }
};
        locals.var_gc3ch_i = assign1050_e1324;
        locals.var_gc3ch_i_rv = 0.0;

        let (assign1060_e1328,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p112,)
    } else {
        (locals.var_gc2ovinv_i,)
    }
};
        locals.var_gc2ovinv_i = assign1060_e1328;
        locals.var_gc2ovinv_i_rv = 0.0;

        let (assign1070_e1332,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p122,)
    } else {
        (locals.var_gcovinvfn_i,)
    }
};
        locals.var_gcovinvfn_i = assign1070_e1332;
        locals.var_gcovinvfn_i_rv = 0.0;

        let (assign1080_e1336,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p113,)
    } else {
        (locals.var_gc3ovinv_i,)
    }
};
        locals.var_gc3ovinv_i = assign1080_e1336;
        locals.var_gc3ovinv_i_rv = 0.0;

        let (assign1090_e1340,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p114,)
    } else {
        (locals.var_gc2ovacc_i,)
    }
};
        locals.var_gc2ovacc_i = assign1090_e1340;
        locals.var_gc2ovacc_i_rv = 0.0;

        let (assign1100_e1344,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p115,)
    } else {
        (locals.var_gc3ovacc_i,)
    }
};
        locals.var_gc3ovacc_i = assign1100_e1344;
        locals.var_gc3ovacc_i_rv = 0.0;

        let (assign1110_e1348,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p116,)
    } else {
        (locals.var_gcdov_i,)
    }
};
        locals.var_gcdov_i = assign1110_e1348;
        locals.var_gcdov_i_rv = 0.0;

        let (assign1120_e1352,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p117,)
    } else {
        (locals.var_gcvdov_i,)
    }
};
        locals.var_gcvdov_i = assign1120_e1352;
        locals.var_gcvdov_i_rv = 0.0;

        let (assign1130_e1356,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p118,)
    } else {
        (locals.var_chib_i,)
    }
};
        locals.var_chib_i = assign1130_e1356;
        locals.var_chib_i_rv = 0.0;

        let (assign1140_e1360,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p119,)
    } else {
        (locals.var_niginv_i,)
    }
};
        locals.var_niginv_i = assign1140_e1360;
        locals.var_niginv_i_rv = 0.0;

        let (assign1150_e1364, assign1150_e1364_d_n4, assign1150_e1364_d_n6, assign1150_e1364_d_n7, assign1150_e1364_d_n8, assign1150_e1364_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p124, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_agidl_i, locals.var_agidl_i_dn4, locals.var_agidl_i_dn6, locals.var_agidl_i_dn7, locals.var_agidl_i_dn8, locals.var_agidl_i_dn9,)
    }
};
        locals.var_agidl_i = assign1150_e1364;
        locals.var_agidl_i_dn4 = assign1150_e1364_d_n4;
        locals.var_agidl_i_dn6 = assign1150_e1364_d_n6;
        locals.var_agidl_i_dn7 = assign1150_e1364_d_n7;
        locals.var_agidl_i_dn8 = assign1150_e1364_d_n8;
        locals.var_agidl_i_dn9 = assign1150_e1364_d_n9;
        locals.var_agidl_i_rv = 0.0;

        let (assign1160_e1368, assign1160_e1368_d_n4, assign1160_e1368_d_n6, assign1160_e1368_d_n7, assign1160_e1368_d_n8, assign1160_e1368_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p125, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_agidld_i, locals.var_agidld_i_dn4, locals.var_agidld_i_dn6, locals.var_agidld_i_dn7, locals.var_agidld_i_dn8, locals.var_agidld_i_dn9,)
    }
};
        locals.var_agidld_i = assign1160_e1368;
        locals.var_agidld_i_dn4 = assign1160_e1368_d_n4;
        locals.var_agidld_i_dn6 = assign1160_e1368_d_n6;
        locals.var_agidld_i_dn7 = assign1160_e1368_d_n7;
        locals.var_agidld_i_dn8 = assign1160_e1368_d_n8;
        locals.var_agidld_i_dn9 = assign1160_e1368_d_n9;
        locals.var_agidld_i_rv = 0.0;

        let (assign1170_e1372,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p126,)
    } else {
        (locals.var_bgidl_t,)
    }
};
        locals.var_bgidl_t = assign1170_e1372;
        locals.var_bgidl_t_rv = 0.0;

        let (assign1180_e1376,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p127,)
    } else {
        (locals.var_bgidld_t,)
    }
};
        locals.var_bgidld_t = assign1180_e1376;
        locals.var_bgidld_t_rv = 0.0;

        let (assign1190_e1380,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p128,)
    } else {
        (locals.var_stbgidl_i,)
    }
};
        locals.var_stbgidl_i = assign1190_e1380;
        locals.var_stbgidl_i_rv = 0.0;

        let (assign1200_e1384,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p129,)
    } else {
        (locals.var_stbgidld_i,)
    }
};
        locals.var_stbgidld_i = assign1200_e1384;
        locals.var_stbgidld_i_rv = 0.0;

        let (assign1210_e1388,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p130,)
    } else {
        (locals.var_cgidl_i,)
    }
};
        locals.var_cgidl_i = assign1210_e1388;
        locals.var_cgidl_i_rv = 0.0;

        let (assign1220_e1392,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p131,)
    } else {
        (locals.var_cgidld_i,)
    }
};
        locals.var_cgidld_i = assign1220_e1392;
        locals.var_cgidld_i_rv = 0.0;

        let (assign1230_e1396,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p132,)
    } else {
        (locals.var_dgidl_i,)
    }
};
        locals.var_dgidl_i = assign1230_e1396;
        locals.var_dgidl_i_rv = 0.0;

        let (assign1240_e1400,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p133,)
    } else {
        (locals.var_dgidld_i,)
    }
};
        locals.var_dgidld_i = assign1240_e1400;
        locals.var_dgidld_i_rv = 0.0;

        let (assign1260_e1408,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p148,)
    } else {
        (locals.var_a2_t,)
    }
};
        locals.var_a2_t = assign1260_e1408;
        locals.var_a2_t_rv = 0.0;

        let (assign1270_e1412,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p149,)
    } else {
        (locals.var_sta2_i,)
    }
};
        locals.var_sta2_i = assign1270_e1412;
        locals.var_sta2_i_rv = 0.0;

        let (assign1280_e1416,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p150,)
    } else {
        (locals.var_a3_i,)
    }
};
        locals.var_a3_i = assign1280_e1416;
        locals.var_a3_i_rv = 0.0;

        let (assign1290_e1420,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p134,)
    } else {
        (locals.var_ctedge_i,)
    }
};
        locals.var_ctedge_i = assign1290_e1420;
        locals.var_ctedge_i_rv = 0.0;

        let (assign1300_e1424, assign1300_e1424_d_n4, assign1300_e1424_d_n6, assign1300_e1424_d_n7, assign1300_e1424_d_n8, assign1300_e1424_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p135, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfb1edge_t, locals.var_vfb1edge_t_dn4, locals.var_vfb1edge_t_dn6, locals.var_vfb1edge_t_dn7, locals.var_vfb1edge_t_dn8, locals.var_vfb1edge_t_dn9,)
    }
};
        locals.var_vfb1edge_t = assign1300_e1424;
        locals.var_vfb1edge_t_dn4 = assign1300_e1424_d_n4;
        locals.var_vfb1edge_t_dn6 = assign1300_e1424_d_n6;
        locals.var_vfb1edge_t_dn7 = assign1300_e1424_d_n7;
        locals.var_vfb1edge_t_dn8 = assign1300_e1424_d_n8;
        locals.var_vfb1edge_t_dn9 = assign1300_e1424_d_n9;
        locals.var_vfb1edge_t_rv = 0.0;

        let (assign1310_e1428,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p136,)
    } else {
        (locals.var_vfb2edge_t,)
    }
};
        locals.var_vfb2edge_t = assign1310_e1428;
        locals.var_vfb2edge_t_rv = 0.0;

        let (assign1320_e1432,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p137,)
    } else {
        (locals.var_stvfbedge_i,)
    }
};
        locals.var_stvfbedge_i = assign1320_e1432;
        locals.var_stvfbedge_i_rv = 0.0;

        let (assign1330_e1436,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p138,)
    } else {
        (locals.var_cic1edge_i,)
    }
};
        locals.var_cic1edge_i = assign1330_e1436;
        locals.var_cic1edge_i_rv = 0.0;

        let (assign1340_e1440,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p139,)
    } else {
        (locals.var_cic2edge_i,)
    }
};
        locals.var_cic2edge_i = assign1340_e1440;
        locals.var_cic2edge_i_rv = 0.0;

        let (assign1350_e1444, assign1350_e1444_d_n4, assign1350_e1444_d_n6, assign1350_e1444_d_n7, assign1350_e1444_d_n8, assign1350_e1444_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p140, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psce1edge_i, locals.var_psce1edge_i_dn4, locals.var_psce1edge_i_dn6, locals.var_psce1edge_i_dn7, locals.var_psce1edge_i_dn8, locals.var_psce1edge_i_dn9,)
    }
};
        locals.var_psce1edge_i = assign1350_e1444;
        locals.var_psce1edge_i_dn4 = assign1350_e1444_d_n4;
        locals.var_psce1edge_i_dn6 = assign1350_e1444_d_n6;
        locals.var_psce1edge_i_dn7 = assign1350_e1444_d_n7;
        locals.var_psce1edge_i_dn8 = assign1350_e1444_d_n8;
        locals.var_psce1edge_i_dn9 = assign1350_e1444_d_n9;
        locals.var_psce1edge_i_rv = 0.0;

        let (assign1360_e1454, assign1360_e1454_d_n4, assign1360_e1454_d_n6, assign1360_e1454_d_n7, assign1360_e1454_d_n8, assign1360_e1454_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        let assign1360_e1448: f64 = (p.p141 * locals.var_psce1edge_i);
        let assign1360_e1450: f64 = (assign1360_e1448 * locals.var_tox2_i);
        let assign1360_e1452: f64 = (assign1360_e1450 / locals.var_tox1_i);
        (assign1360_e1452, (((p.p141 * locals.var_psce1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_psce2edge_i, locals.var_psce2edge_i_dn4, locals.var_psce2edge_i_dn6, locals.var_psce2edge_i_dn7, locals.var_psce2edge_i_dn8, locals.var_psce2edge_i_dn9,)
    }
};
        locals.var_psce2edge_i = assign1360_e1454;
        locals.var_psce2edge_i_dn4 = assign1360_e1454_d_n4;
        locals.var_psce2edge_i_dn6 = assign1360_e1454_d_n6;
        locals.var_psce2edge_i_dn7 = assign1360_e1454_d_n7;
        locals.var_psce2edge_i_dn8 = assign1360_e1454_d_n8;
        locals.var_psce2edge_i_dn9 = assign1360_e1454_d_n9;
        locals.var_psce2edge_i_rv = 0.0;

        let (assign1370_e1458, assign1370_e1458_d_n4, assign1370_e1458_d_n6, assign1370_e1458_d_n7, assign1370_e1458_d_n8, assign1370_e1458_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p142, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cf1edge_i, locals.var_cf1edge_i_dn4, locals.var_cf1edge_i_dn6, locals.var_cf1edge_i_dn7, locals.var_cf1edge_i_dn8, locals.var_cf1edge_i_dn9,)
    }
};
        locals.var_cf1edge_i = assign1370_e1458;
        locals.var_cf1edge_i_dn4 = assign1370_e1458_d_n4;
        locals.var_cf1edge_i_dn6 = assign1370_e1458_d_n6;
        locals.var_cf1edge_i_dn7 = assign1370_e1458_d_n7;
        locals.var_cf1edge_i_dn8 = assign1370_e1458_d_n8;
        locals.var_cf1edge_i_dn9 = assign1370_e1458_d_n9;
        locals.var_cf1edge_i_rv = 0.0;

        let (assign1380_e1468, assign1380_e1468_d_n4, assign1380_e1468_d_n6, assign1380_e1468_d_n7, assign1380_e1468_d_n8, assign1380_e1468_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        let assign1380_e1462: f64 = (p.p143 * locals.var_cf1edge_i);
        let assign1380_e1464: f64 = (assign1380_e1462 * locals.var_tox2_i);
        let assign1380_e1466: f64 = (assign1380_e1464 / locals.var_tox1_i);
        (assign1380_e1466, (((p.p143 * locals.var_cf1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_cf2edge_i, locals.var_cf2edge_i_dn4, locals.var_cf2edge_i_dn6, locals.var_cf2edge_i_dn7, locals.var_cf2edge_i_dn8, locals.var_cf2edge_i_dn9,)
    }
};
        locals.var_cf2edge_i = assign1380_e1468;
        locals.var_cf2edge_i_dn4 = assign1380_e1468_d_n4;
        locals.var_cf2edge_i_dn6 = assign1380_e1468_d_n6;
        locals.var_cf2edge_i_dn7 = assign1380_e1468_d_n7;
        locals.var_cf2edge_i_dn8 = assign1380_e1468_d_n8;
        locals.var_cf2edge_i_dn9 = assign1380_e1468_d_n9;
        locals.var_cf2edge_i_rv = 0.0;

        let (assign1390_e1472,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p144,)
    } else {
        (locals.var_cfdedge_i,)
    }
};
        locals.var_cfdedge_i = assign1390_e1472;
        locals.var_cfdedge_i_rv = 0.0;

        let (assign1400_e1476, assign1400_e1476_d_n4, assign1400_e1476_d_n6, assign1400_e1476_d_n7, assign1400_e1476_d_n8, assign1400_e1476_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p145, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_betnedge_t, locals.var_betnedge_t_dn4, locals.var_betnedge_t_dn6, locals.var_betnedge_t_dn7, locals.var_betnedge_t_dn8, locals.var_betnedge_t_dn9,)
    }
};
        locals.var_betnedge_t = assign1400_e1476;
        locals.var_betnedge_t_dn4 = assign1400_e1476_d_n4;
        locals.var_betnedge_t_dn6 = assign1400_e1476_d_n6;
        locals.var_betnedge_t_dn7 = assign1400_e1476_d_n7;
        locals.var_betnedge_t_dn8 = assign1400_e1476_d_n8;
        locals.var_betnedge_t_dn9 = assign1400_e1476_d_n9;
        locals.var_betnedge_t_rv = 0.0;

        let (assign1410_e1480,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p146,)
    } else {
        (locals.var_stbetedge_i,)
    }
};
        locals.var_stbetedge_i = assign1410_e1480;
        locals.var_stbetedge_i_rv = 0.0;

        let (assign1420_e1484,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p151,)
    } else {
        (locals.var_areaq_i,)
    }
};
        locals.var_areaq_i = assign1420_e1484;
        locals.var_areaq_i_rv = 0.0;

        let (assign1430_e1488, assign1430_e1488_d_n4, assign1430_e1488_d_n6, assign1430_e1488_d_n7, assign1430_e1488_d_n8, assign1430_e1488_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p152, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgbov_i, locals.var_cgbov_i_dn4, locals.var_cgbov_i_dn6, locals.var_cgbov_i_dn7, locals.var_cgbov_i_dn8, locals.var_cgbov_i_dn9,)
    }
};
        locals.var_cgbov_i = assign1430_e1488;
        locals.var_cgbov_i_dn4 = assign1430_e1488_d_n4;
        locals.var_cgbov_i_dn6 = assign1430_e1488_d_n6;
        locals.var_cgbov_i_dn7 = assign1430_e1488_d_n7;
        locals.var_cgbov_i_dn8 = assign1430_e1488_d_n8;
        locals.var_cgbov_i_dn9 = assign1430_e1488_d_n9;
        locals.var_cgbov_i_rv = 0.0;

        let (assign1440_e1494,) = {
    if (locals.var_guard83 != 0.0) {
        let assign1440_e1492: f64 = (p.p153 * 1000000.0);
        (assign1440_e1492,)
    } else {
        (locals.var_nsdac_i,)
    }
};
        locals.var_nsdac_i = assign1440_e1494;
        locals.var_nsdac_i_rv = 0.0;

    }
}
