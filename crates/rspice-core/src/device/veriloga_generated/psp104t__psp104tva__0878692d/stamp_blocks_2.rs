#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47260_e60716, assign47260_e60716_d_n4, assign47260_e60716_d_n6, assign47260_e60716_d_n7, assign47260_e60716_d_n8, assign47260_e60716_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign47260_e60710: f64 = (locals.var_igc0 * locals.var_igc);
        let assign47260_e60713: f64 = (1.0 - locals.var_sg);
        let assign47260_e60714: f64 = (assign47260_e60710 * assign47260_e60713);
        (assign47260_e60714, ((((locals.var_igc0_dn4 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn4)) * assign47260_e60713) + (assign47260_e60710 * (-locals.var_sg_dn4))), ((((locals.var_igc0_dn6 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn6)) * assign47260_e60713) + (assign47260_e60710 * (-locals.var_sg_dn6))), ((((locals.var_igc0_dn7 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn7)) * assign47260_e60713) + (assign47260_e60710 * (-locals.var_sg_dn7))), ((((locals.var_igc0_dn8 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn8)) * assign47260_e60713) + (assign47260_e60710 * (-locals.var_sg_dn8))), ((((locals.var_igc0_dn9 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn9)) * assign47260_e60713) + (assign47260_e60710 * (-locals.var_sg_dn9))),)
    } else {
        (locals.var_i_gb, locals.var_i_gb_dn4, locals.var_i_gb_dn6, locals.var_i_gb_dn7, locals.var_i_gb_dn8, locals.var_i_gb_dn9,)
    }
};
        locals.var_i_gb = assign47260_e60716;
        locals.var_i_gb_dn4 = assign47260_e60716_d_n4;
        locals.var_i_gb_dn6 = assign47260_e60716_d_n6;
        locals.var_i_gb_dn7 = assign47260_e60716_d_n7;
        locals.var_i_gb_dn8 = assign47260_e60716_d_n8;
        locals.var_i_gb_dn9 = assign47260_e60716_d_n9;

        locals.var_i_gidl = 0.0;
        locals.var_i_gidl_dn4 = 0.0;
        locals.var_i_gidl_dn6 = 0.0;
        locals.var_i_gidl_dn7 = 0.0;
        locals.var_i_gidl_dn8 = 0.0;
        locals.var_i_gidl_dn9 = 0.0;

        locals.var_i_gisl = 0.0;
        locals.var_i_gisl_dn4 = 0.0;
        locals.var_i_gisl_dn6 = 0.0;
        locals.var_i_gisl_dn7 = 0.0;
        locals.var_i_gisl_dn8 = 0.0;
        locals.var_i_gisl_dn9 = 0.0;

        let assign47290_e60721: f64 = if p.p42 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1261 = assign47290_e60721;

        let assign47300_e60728: f64 = if ((locals.var_agidld_i > 0.0) && (locals.var_vovd < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1262 = assign47300_e60728;

        let (assign47310_e60747, assign47310_e60747_d_n6, assign47310_e60747_d_n7, assign47310_e60747_d_n8, assign47310_e60747_d_n9,) = {
    if ((locals.var_guard1261 != 0.0) && (locals.var_guard1262 != 0.0)) {
        let assign47310_e60734: f64 = (locals.var_vovd * locals.var_vovd);
        let assign47310_e60737: f64 = (locals.var_cgidld_i * locals.var_cgidld_i);
        let assign47310_e60740: f64 = (locals.var_vdbprime * locals.var_vdbprime);
        let assign47310_e60741: f64 = (assign47310_e60737 * assign47310_e60740);
        let assign47310_e60742: f64 = (assign47310_e60734 + assign47310_e60741);
        let assign47310_e60744: f64 = (assign47310_e60742 + 1e-6);
        let assign47310_e60745: f64 = (assign47310_e60744).sqrt();
        (assign47310_e60745, (((locals.var_vovd_dn6 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn6)) / (2.0 * assign47310_e60745)), ((((locals.var_vovd_dn7 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn7)) + (assign47310_e60737 * ((locals.var_vdbprime_dn7 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn7)))) / (2.0 * assign47310_e60745)), ((((locals.var_vovd_dn8 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn8)) + (assign47310_e60737 * ((locals.var_vdbprime_dn8 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn8)))) / (2.0 * assign47310_e60745)), ((assign47310_e60737 * ((locals.var_vdbprime_dn9 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn9))) / (2.0 * assign47310_e60745)),)
    } else {
        (locals.var_vtovd, locals.var_vtovd_dn6, locals.var_vtovd_dn7, locals.var_vtovd_dn8, locals.var_vtovd_dn9,)
    }
};
        locals.var_vtovd = assign47310_e60747;
        locals.var_vtovd_dn6 = assign47310_e60747_d_n6;
        locals.var_vtovd_dn7 = assign47310_e60747_d_n7;
        locals.var_vtovd_dn8 = assign47310_e60747_d_n8;
        locals.var_vtovd_dn9 = assign47310_e60747_d_n9;

        let (assign47320_e60756, assign47320_e60756_d_n4, assign47320_e60756_d_n6, assign47320_e60756_d_n7, assign47320_e60756_d_n8, assign47320_e60756_d_n9,) = {
    if ((locals.var_guard1261 != 0.0) && (locals.var_guard1262 != 0.0)) {
        let assign47320_e60752: f64 = (-locals.var_bgidlds);
        let assign47320_e60754: f64 = (assign47320_e60752 / locals.var_vtovd);
        (assign47320_e60754, 0.0, (-((assign47320_e60752 * locals.var_vtovd_dn6) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47320_e60752 * locals.var_vtovd_dn7) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47320_e60752 * locals.var_vtovd_dn8) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47320_e60752 * locals.var_vtovd_dn9) / (locals.var_vtovd * locals.var_vtovd))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign47320_e60756;
        locals.var_temp__blk949_dn4 = assign47320_e60756_d_n4;
        locals.var_temp__blk949_dn6 = assign47320_e60756_d_n6;
        locals.var_temp__blk949_dn7 = assign47320_e60756_d_n7;
        locals.var_temp__blk949_dn8 = assign47320_e60756_d_n8;
        locals.var_temp__blk949_dn9 = assign47320_e60756_d_n9;

        let assign47330_e60759: f64 = (-230.25850929940458);
        let assign47330_e60760: f64 = if locals.var_temp__blk949 > assign47330_e60759 { 1.0 } else { 0.0 };
        locals.var_guard1263 = assign47330_e60760;

        let (assign47340_e60769, assign47340_e60769_d_n4, assign47340_e60769_d_n6, assign47340_e60769_d_n7, assign47340_e60769_d_n8, assign47340_e60769_d_n9,) = {
    if (((locals.var_guard1261 != 0.0) && (locals.var_guard1262 != 0.0)) && (locals.var_guard1263 != 0.0)) {
        let assign47340_e60767: f64 = (locals.var_temp__blk949).exp();
        (assign47340_e60767, (assign47340_e60767 * locals.var_temp__blk949_dn4), (assign47340_e60767 * locals.var_temp__blk949_dn6), (assign47340_e60767 * locals.var_temp__blk949_dn7), (assign47340_e60767 * locals.var_temp__blk949_dn8), (assign47340_e60767 * locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign47340_e60769;
        locals.var_temp2_dn4 = assign47340_e60769_d_n4;
        locals.var_temp2_dn6 = assign47340_e60769_d_n6;
        locals.var_temp2_dn7 = assign47340_e60769_d_n7;
        locals.var_temp2_dn8 = assign47340_e60769_d_n8;
        locals.var_temp2_dn9 = assign47340_e60769_d_n9;

        let (assign47350_e60803, assign47350_e60803_d_n4, assign47350_e60803_d_n6, assign47350_e60803_d_n7, assign47350_e60803_d_n8, assign47350_e60803_d_n9,) = {
    if (((locals.var_guard1261 != 0.0) && (locals.var_guard1262 != 0.0)) && (locals.var_guard1263 == 0.0)) {
        let assign47350_e60779: f64 = (-230.25850929940458);
        let assign47350_e60781: f64 = (assign47350_e60779 - locals.var_temp__blk949);
        let assign47350_e60785: f64 = (-230.25850929940458);
        let assign47350_e60787: f64 = (assign47350_e60785 - locals.var_temp__blk949);
        let assign47350_e60790: f64 = (-230.25850929940458);
        let assign47350_e60792: f64 = (assign47350_e60790 - locals.var_temp__blk949);
        let assign47350_e60794: f64 = (assign47350_e60792 * 0.3333333333333333);
        let assign47350_e60795: f64 = (1.0 + assign47350_e60794);
        let assign47350_e60796: f64 = (assign47350_e60787 * assign47350_e60795);
        let assign47350_e60797: f64 = (0.5 * assign47350_e60796);
        let assign47350_e60798: f64 = (1.0 + assign47350_e60797);
        let assign47350_e60799: f64 = (assign47350_e60781 * assign47350_e60798);
        let assign47350_e60800: f64 = (1.0 + assign47350_e60799);
        let assign47350_e60801: f64 = (1e-100 / assign47350_e60800);
        (assign47350_e60801, (-((1e-100 * (((-locals.var_temp__blk949_dn4) * assign47350_e60798) + (assign47350_e60781 * (0.5 * (((-locals.var_temp__blk949_dn4) * assign47350_e60795) + (assign47350_e60787 * ((-locals.var_temp__blk949_dn4) * 0.3333333333333333))))))) / (assign47350_e60800 * assign47350_e60800))), (-((1e-100 * (((-locals.var_temp__blk949_dn6) * assign47350_e60798) + (assign47350_e60781 * (0.5 * (((-locals.var_temp__blk949_dn6) * assign47350_e60795) + (assign47350_e60787 * ((-locals.var_temp__blk949_dn6) * 0.3333333333333333))))))) / (assign47350_e60800 * assign47350_e60800))), (-((1e-100 * (((-locals.var_temp__blk949_dn7) * assign47350_e60798) + (assign47350_e60781 * (0.5 * (((-locals.var_temp__blk949_dn7) * assign47350_e60795) + (assign47350_e60787 * ((-locals.var_temp__blk949_dn7) * 0.3333333333333333))))))) / (assign47350_e60800 * assign47350_e60800))), (-((1e-100 * (((-locals.var_temp__blk949_dn8) * assign47350_e60798) + (assign47350_e60781 * (0.5 * (((-locals.var_temp__blk949_dn8) * assign47350_e60795) + (assign47350_e60787 * ((-locals.var_temp__blk949_dn8) * 0.3333333333333333))))))) / (assign47350_e60800 * assign47350_e60800))), (-((1e-100 * (((-locals.var_temp__blk949_dn9) * assign47350_e60798) + (assign47350_e60781 * (0.5 * (((-locals.var_temp__blk949_dn9) * assign47350_e60795) + (assign47350_e60787 * ((-locals.var_temp__blk949_dn9) * 0.3333333333333333))))))) / (assign47350_e60800 * assign47350_e60800))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign47350_e60803;
        locals.var_temp2_dn4 = assign47350_e60803_d_n4;
        locals.var_temp2_dn6 = assign47350_e60803_d_n6;
        locals.var_temp2_dn7 = assign47350_e60803_d_n7;
        locals.var_temp2_dn8 = assign47350_e60803_d_n8;
        locals.var_temp2_dn9 = assign47350_e60803_d_n9;

        let (assign47360_e60818, assign47360_e60818_d_n4, assign47360_e60818_d_n6, assign47360_e60818_d_n7, assign47360_e60818_d_n8, assign47360_e60818_d_n9,) = {
    if ((locals.var_guard1261 != 0.0) && (locals.var_guard1262 != 0.0)) {
        let assign47360_e60808: f64 = (-locals.var_agidlds);
        let assign47360_e60811: f64 = (locals.var_vdbprime * locals.var_vovd);
        let assign47360_e60813: f64 = (assign47360_e60811 * locals.var_vtovd);
        let assign47360_e60815: f64 = (assign47360_e60813 * locals.var_temp2);
        let assign47360_e60816: f64 = (assign47360_e60808 * assign47360_e60815);
        (assign47360_e60816, (assign47360_e60808 * (assign47360_e60813 * locals.var_temp2_dn4)), (assign47360_e60808 * (((((locals.var_vdbprime * locals.var_vovd_dn6) * locals.var_vtovd) + (assign47360_e60811 * locals.var_vtovd_dn6)) * locals.var_temp2) + (assign47360_e60813 * locals.var_temp2_dn6))), (assign47360_e60808 * ((((((locals.var_vdbprime_dn7 * locals.var_vovd) + (locals.var_vdbprime * locals.var_vovd_dn7)) * locals.var_vtovd) + (assign47360_e60811 * locals.var_vtovd_dn7)) * locals.var_temp2) + (assign47360_e60813 * locals.var_temp2_dn7))), (assign47360_e60808 * ((((((locals.var_vdbprime_dn8 * locals.var_vovd) + (locals.var_vdbprime * locals.var_vovd_dn8)) * locals.var_vtovd) + (assign47360_e60811 * locals.var_vtovd_dn8)) * locals.var_temp2) + (assign47360_e60813 * locals.var_temp2_dn8))), (assign47360_e60808 * (((((locals.var_vdbprime_dn9 * locals.var_vovd) * locals.var_vtovd) + (assign47360_e60811 * locals.var_vtovd_dn9)) * locals.var_temp2) + (assign47360_e60813 * locals.var_temp2_dn9))),)
    } else {
        (locals.var_i_gidl, locals.var_i_gidl_dn4, locals.var_i_gidl_dn6, locals.var_i_gidl_dn7, locals.var_i_gidl_dn8, locals.var_i_gidl_dn9,)
    }
};
        locals.var_i_gidl = assign47360_e60818;
        locals.var_i_gidl_dn4 = assign47360_e60818_d_n4;
        locals.var_i_gidl_dn6 = assign47360_e60818_d_n6;
        locals.var_i_gidl_dn7 = assign47360_e60818_d_n7;
        locals.var_i_gidl_dn8 = assign47360_e60818_d_n8;
        locals.var_i_gidl_dn9 = assign47360_e60818_d_n9;

        let assign47370_e60825: f64 = if ((locals.var_agidl_i > 0.0) && (locals.var_vovs < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1264 = assign47370_e60825;

        let (assign47380_e60844, assign47380_e60844_d_n6, assign47380_e60844_d_n7, assign47380_e60844_d_n8, assign47380_e60844_d_n9,) = {
    if ((locals.var_guard1261 != 0.0) && (locals.var_guard1264 != 0.0)) {
        let assign47380_e60831: f64 = (locals.var_vovs * locals.var_vovs);
        let assign47380_e60834: f64 = (locals.var_cgidl_i * locals.var_cgidl_i);
        let assign47380_e60837: f64 = (locals.var_vsbprime * locals.var_vsbprime);
        let assign47380_e60838: f64 = (assign47380_e60834 * assign47380_e60837);
        let assign47380_e60839: f64 = (assign47380_e60831 + assign47380_e60838);
        let assign47380_e60841: f64 = (assign47380_e60839 + 1e-6);
        let assign47380_e60842: f64 = (assign47380_e60841).sqrt();
        (assign47380_e60842, (((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) / (2.0 * assign47380_e60842)), ((((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) + (assign47380_e60834 * ((locals.var_vsbprime_dn7 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn7)))) / (2.0 * assign47380_e60842)), ((((locals.var_vovs_dn8 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn8)) + (assign47380_e60834 * ((locals.var_vsbprime_dn8 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn8)))) / (2.0 * assign47380_e60842)), ((assign47380_e60834 * ((locals.var_vsbprime_dn9 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn9))) / (2.0 * assign47380_e60842)),)
    } else {
        (locals.var_vtovs, locals.var_vtovs_dn6, locals.var_vtovs_dn7, locals.var_vtovs_dn8, locals.var_vtovs_dn9,)
    }
};
        locals.var_vtovs = assign47380_e60844;
        locals.var_vtovs_dn6 = assign47380_e60844_d_n6;
        locals.var_vtovs_dn7 = assign47380_e60844_d_n7;
        locals.var_vtovs_dn8 = assign47380_e60844_d_n8;
        locals.var_vtovs_dn9 = assign47380_e60844_d_n9;

        let (assign47390_e60853, assign47390_e60853_d_n4, assign47390_e60853_d_n6, assign47390_e60853_d_n7, assign47390_e60853_d_n8, assign47390_e60853_d_n9,) = {
    if ((locals.var_guard1261 != 0.0) && (locals.var_guard1264 != 0.0)) {
        let assign47390_e60849: f64 = (-locals.var_bgidls);
        let assign47390_e60851: f64 = (assign47390_e60849 / locals.var_vtovs);
        (assign47390_e60851, 0.0, (-((assign47390_e60849 * locals.var_vtovs_dn6) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47390_e60849 * locals.var_vtovs_dn7) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47390_e60849 * locals.var_vtovs_dn8) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47390_e60849 * locals.var_vtovs_dn9) / (locals.var_vtovs * locals.var_vtovs))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign47390_e60853;
        locals.var_temp__blk949_dn4 = assign47390_e60853_d_n4;
        locals.var_temp__blk949_dn6 = assign47390_e60853_d_n6;
        locals.var_temp__blk949_dn7 = assign47390_e60853_d_n7;
        locals.var_temp__blk949_dn8 = assign47390_e60853_d_n8;
        locals.var_temp__blk949_dn9 = assign47390_e60853_d_n9;

        let assign47400_e60856: f64 = (-230.25850929940458);
        let assign47400_e60857: f64 = if locals.var_temp__blk949 > assign47400_e60856 { 1.0 } else { 0.0 };
        locals.var_guard1265 = assign47400_e60857;

        let (assign47410_e60866, assign47410_e60866_d_n4, assign47410_e60866_d_n6, assign47410_e60866_d_n7, assign47410_e60866_d_n8, assign47410_e60866_d_n9,) = {
    if (((locals.var_guard1261 != 0.0) && (locals.var_guard1264 != 0.0)) && (locals.var_guard1265 != 0.0)) {
        let assign47410_e60864: f64 = (locals.var_temp__blk949).exp();
        (assign47410_e60864, (assign47410_e60864 * locals.var_temp__blk949_dn4), (assign47410_e60864 * locals.var_temp__blk949_dn6), (assign47410_e60864 * locals.var_temp__blk949_dn7), (assign47410_e60864 * locals.var_temp__blk949_dn8), (assign47410_e60864 * locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign47410_e60866;
        locals.var_temp2_dn4 = assign47410_e60866_d_n4;
        locals.var_temp2_dn6 = assign47410_e60866_d_n6;
        locals.var_temp2_dn7 = assign47410_e60866_d_n7;
        locals.var_temp2_dn8 = assign47410_e60866_d_n8;
        locals.var_temp2_dn9 = assign47410_e60866_d_n9;

        let (assign47420_e60900, assign47420_e60900_d_n4, assign47420_e60900_d_n6, assign47420_e60900_d_n7, assign47420_e60900_d_n8, assign47420_e60900_d_n9,) = {
    if (((locals.var_guard1261 != 0.0) && (locals.var_guard1264 != 0.0)) && (locals.var_guard1265 == 0.0)) {
        let assign47420_e60876: f64 = (-230.25850929940458);
        let assign47420_e60878: f64 = (assign47420_e60876 - locals.var_temp__blk949);
        let assign47420_e60882: f64 = (-230.25850929940458);
        let assign47420_e60884: f64 = (assign47420_e60882 - locals.var_temp__blk949);
        let assign47420_e60887: f64 = (-230.25850929940458);
        let assign47420_e60889: f64 = (assign47420_e60887 - locals.var_temp__blk949);
        let assign47420_e60891: f64 = (assign47420_e60889 * 0.3333333333333333);
        let assign47420_e60892: f64 = (1.0 + assign47420_e60891);
        let assign47420_e60893: f64 = (assign47420_e60884 * assign47420_e60892);
        let assign47420_e60894: f64 = (0.5 * assign47420_e60893);
        let assign47420_e60895: f64 = (1.0 + assign47420_e60894);
        let assign47420_e60896: f64 = (assign47420_e60878 * assign47420_e60895);
        let assign47420_e60897: f64 = (1.0 + assign47420_e60896);
        let assign47420_e60898: f64 = (1e-100 / assign47420_e60897);
        (assign47420_e60898, (-((1e-100 * (((-locals.var_temp__blk949_dn4) * assign47420_e60895) + (assign47420_e60878 * (0.5 * (((-locals.var_temp__blk949_dn4) * assign47420_e60892) + (assign47420_e60884 * ((-locals.var_temp__blk949_dn4) * 0.3333333333333333))))))) / (assign47420_e60897 * assign47420_e60897))), (-((1e-100 * (((-locals.var_temp__blk949_dn6) * assign47420_e60895) + (assign47420_e60878 * (0.5 * (((-locals.var_temp__blk949_dn6) * assign47420_e60892) + (assign47420_e60884 * ((-locals.var_temp__blk949_dn6) * 0.3333333333333333))))))) / (assign47420_e60897 * assign47420_e60897))), (-((1e-100 * (((-locals.var_temp__blk949_dn7) * assign47420_e60895) + (assign47420_e60878 * (0.5 * (((-locals.var_temp__blk949_dn7) * assign47420_e60892) + (assign47420_e60884 * ((-locals.var_temp__blk949_dn7) * 0.3333333333333333))))))) / (assign47420_e60897 * assign47420_e60897))), (-((1e-100 * (((-locals.var_temp__blk949_dn8) * assign47420_e60895) + (assign47420_e60878 * (0.5 * (((-locals.var_temp__blk949_dn8) * assign47420_e60892) + (assign47420_e60884 * ((-locals.var_temp__blk949_dn8) * 0.3333333333333333))))))) / (assign47420_e60897 * assign47420_e60897))), (-((1e-100 * (((-locals.var_temp__blk949_dn9) * assign47420_e60895) + (assign47420_e60878 * (0.5 * (((-locals.var_temp__blk949_dn9) * assign47420_e60892) + (assign47420_e60884 * ((-locals.var_temp__blk949_dn9) * 0.3333333333333333))))))) / (assign47420_e60897 * assign47420_e60897))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign47420_e60900;
        locals.var_temp2_dn4 = assign47420_e60900_d_n4;
        locals.var_temp2_dn6 = assign47420_e60900_d_n6;
        locals.var_temp2_dn7 = assign47420_e60900_d_n7;
        locals.var_temp2_dn8 = assign47420_e60900_d_n8;
        locals.var_temp2_dn9 = assign47420_e60900_d_n9;

        let (assign47430_e60915, assign47430_e60915_d_n4, assign47430_e60915_d_n6, assign47430_e60915_d_n7, assign47430_e60915_d_n8, assign47430_e60915_d_n9,) = {
    if ((locals.var_guard1261 != 0.0) && (locals.var_guard1264 != 0.0)) {
        let assign47430_e60905: f64 = (-locals.var_agidls);
        let assign47430_e60908: f64 = (locals.var_vsbprime * locals.var_vovs);
        let assign47430_e60910: f64 = (assign47430_e60908 * locals.var_vtovs);
        let assign47430_e60912: f64 = (assign47430_e60910 * locals.var_temp2);
        let assign47430_e60913: f64 = (assign47430_e60905 * assign47430_e60912);
        (assign47430_e60913, (assign47430_e60905 * (assign47430_e60910 * locals.var_temp2_dn4)), (assign47430_e60905 * (((((locals.var_vsbprime * locals.var_vovs_dn6) * locals.var_vtovs) + (assign47430_e60908 * locals.var_vtovs_dn6)) * locals.var_temp2) + (assign47430_e60910 * locals.var_temp2_dn6))), (assign47430_e60905 * ((((((locals.var_vsbprime_dn7 * locals.var_vovs) + (locals.var_vsbprime * locals.var_vovs_dn7)) * locals.var_vtovs) + (assign47430_e60908 * locals.var_vtovs_dn7)) * locals.var_temp2) + (assign47430_e60910 * locals.var_temp2_dn7))), (assign47430_e60905 * ((((((locals.var_vsbprime_dn8 * locals.var_vovs) + (locals.var_vsbprime * locals.var_vovs_dn8)) * locals.var_vtovs) + (assign47430_e60908 * locals.var_vtovs_dn8)) * locals.var_temp2) + (assign47430_e60910 * locals.var_temp2_dn8))), (assign47430_e60905 * (((((locals.var_vsbprime_dn9 * locals.var_vovs) * locals.var_vtovs) + (assign47430_e60908 * locals.var_vtovs_dn9)) * locals.var_temp2) + (assign47430_e60910 * locals.var_temp2_dn9))),)
    } else {
        (locals.var_i_gisl, locals.var_i_gisl_dn4, locals.var_i_gisl_dn6, locals.var_i_gisl_dn7, locals.var_i_gisl_dn8, locals.var_i_gisl_dn9,)
    }
};
        locals.var_i_gisl = assign47430_e60915;
        locals.var_i_gisl_dn4 = assign47430_e60915_d_n4;
        locals.var_i_gisl_dn6 = assign47430_e60915_d_n6;
        locals.var_i_gisl_dn7 = assign47430_e60915_d_n7;
        locals.var_i_gisl_dn8 = assign47430_e60915_d_n8;
        locals.var_i_gisl_dn9 = assign47430_e60915_d_n9;

        locals.var_phit1edge = locals.var_phit;
        locals.var_phit1edge_dn4 = locals.var_phit_dn4;
        locals.var_phit1edge_dn6 = 0.0;
        locals.var_phit1edge_dn7 = 0.0;
        locals.var_phit1edge_dn8 = 0.0;
        locals.var_phit1edge_dn9 = 0.0;

        locals.var_xgedge = 0.0;
        locals.var_xgedge_dn4 = 0.0;
        locals.var_xgedge_dn6 = 0.0;
        locals.var_xgedge_dn7 = 0.0;
        locals.var_xgedge_dn8 = 0.0;
        locals.var_xgedge_dn9 = 0.0;

        locals.var_qdseffedge = 0.0;
        locals.var_qdseffedge_dn4 = 0.0;
        locals.var_qdseffedge_dn6 = 0.0;
        locals.var_qdseffedge_dn7 = 0.0;
        locals.var_qdseffedge_dn8 = 0.0;
        locals.var_qdseffedge_dn9 = 0.0;

        locals.var_qmeffedge = 0.0;
        locals.var_qmeffedge_dn4 = 0.0;
        locals.var_qmeffedge_dn6 = 0.0;
        locals.var_qmeffedge_dn7 = 0.0;
        locals.var_qmeffedge_dn8 = 0.0;
        locals.var_qmeffedge_dn9 = 0.0;

        locals.var_dsqredge = 1e-40;
        locals.var_dsqredge_dn4 = 0.0;
        locals.var_dsqredge_dn6 = 0.0;
        locals.var_dsqredge_dn7 = 0.0;
        locals.var_dsqredge_dn8 = 0.0;
        locals.var_dsqredge_dn9 = 0.0;

        locals.var_alphabmedge = 1.0;
        locals.var_alphabmedge_dn4 = 0.0;
        locals.var_alphabmedge_dn6 = 0.0;
        locals.var_alphabmedge_dn7 = 0.0;
        locals.var_alphabmedge_dn8 = 0.0;
        locals.var_alphabmedge_dn9 = 0.0;

        locals.var_i_dsedge = 0.0;
        locals.var_i_dsedge_dn4 = 0.0;
        locals.var_i_dsedge_dn6 = 0.0;
        locals.var_i_dsedge_dn7 = 0.0;
        locals.var_i_dsedge_dn8 = 0.0;
        locals.var_i_dsedge_dn9 = 0.0;

        let assign47510_e60929: f64 = if ((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1266 = assign47510_e60929;

        let (assign47520_e60950, assign47520_e60950_d_n4, assign47520_e60950_d_n6, assign47520_e60950_d_n7, assign47520_e60950_d_n8, assign47520_e60950_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47520_e60934: f64 = (locals.var_v_db + locals.var_v_sb);
        let assign47520_e60937: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign47520_e60940: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign47520_e60941: f64 = (assign47520_e60937 * assign47520_e60940);
        let assign47520_e60943: f64 = (assign47520_e60941 + locals.var_bphiedge);
        let assign47520_e60944: f64 = (assign47520_e60943).sqrt();
        let assign47520_e60945: f64 = (assign47520_e60934 - assign47520_e60944);
        let assign47520_e60946: f64 = (0.5 * assign47520_e60945);
        let assign47520_e60948: f64 = (assign47520_e60946 + locals.var_phixedge);
        (assign47520_e60948, ((0.5 * (-(locals.var_bphiedge_dn4 / (2.0 * assign47520_e60944)))) + locals.var_phixedge_dn4), 0.0, (0.5 * ((locals.var_v_db_dn7 + locals.var_v_sb_dn7) - ((((locals.var_v_db_dn7 - locals.var_v_sb_dn7) * assign47520_e60940) + (assign47520_e60937 * (locals.var_v_db_dn7 - locals.var_v_sb_dn7))) / (2.0 * assign47520_e60944)))), (0.5 * ((locals.var_v_db_dn8 + locals.var_v_sb_dn8) - ((((locals.var_v_db_dn8 - locals.var_v_sb_dn8) * assign47520_e60940) + (assign47520_e60937 * (locals.var_v_db_dn8 - locals.var_v_sb_dn8))) / (2.0 * assign47520_e60944)))), (0.5 * ((locals.var_v_db_dn9 + locals.var_v_sb_dn9) - ((((locals.var_v_db_dn9 - locals.var_v_sb_dn9) * assign47520_e60940) + (assign47520_e60937 * (locals.var_v_db_dn9 - locals.var_v_sb_dn9))) / (2.0 * assign47520_e60944)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign47520_e60950;
        locals.var_temp__blk949_dn4 = assign47520_e60950_d_n4;
        locals.var_temp__blk949_dn6 = assign47520_e60950_d_n6;
        locals.var_temp__blk949_dn7 = assign47520_e60950_d_n7;
        locals.var_temp__blk949_dn8 = assign47520_e60950_d_n8;
        locals.var_temp__blk949_dn9 = assign47520_e60950_d_n9;

        let (assign47530_e60973, assign47530_e60973_d_n4, assign47530_e60973_d_n6, assign47530_e60973_d_n7, assign47530_e60973_d_n8, assign47530_e60973_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47530_e60956: f64 = locals.var_temp__blk949;
        let assign47530_e60959: f64 = locals.var_temp__blk949;
        let assign47530_e60962: f64 = locals.var_temp__blk949;
        let assign47530_e60963: f64 = (assign47530_e60959 * assign47530_e60962);
        let assign47530_e60965: f64 = (assign47530_e60963 + locals.var_aphiedge);
        let assign47530_e60966: f64 = (assign47530_e60965).sqrt();
        let assign47530_e60967: f64 = (assign47530_e60956 - assign47530_e60966);
        let assign47530_e60968: f64 = (0.5 * assign47530_e60967);
        let assign47530_e60969: f64 = (locals.var_v_sb - assign47530_e60968);
        let assign47530_e60971: f64 = (assign47530_e60969 + locals.var_phix1edge);
        (assign47530_e60971, ((-(0.5 * (locals.var_temp__blk949_dn4 - ((((locals.var_temp__blk949_dn4 * assign47530_e60962) + (assign47530_e60959 * locals.var_temp__blk949_dn4)) + locals.var_aphiedge_dn4) / (2.0 * assign47530_e60966))))) + locals.var_phix1edge_dn4), (-(0.5 * (locals.var_temp__blk949_dn6 - (((locals.var_temp__blk949_dn6 * assign47530_e60962) + (assign47530_e60959 * locals.var_temp__blk949_dn6)) / (2.0 * assign47530_e60966))))), (locals.var_v_sb_dn7 - (0.5 * (locals.var_temp__blk949_dn7 - (((locals.var_temp__blk949_dn7 * assign47530_e60962) + (assign47530_e60959 * locals.var_temp__blk949_dn7)) / (2.0 * assign47530_e60966))))), (locals.var_v_sb_dn8 - (0.5 * (locals.var_temp__blk949_dn8 - (((locals.var_temp__blk949_dn8 * assign47530_e60962) + (assign47530_e60959 * locals.var_temp__blk949_dn8)) / (2.0 * assign47530_e60966))))), (locals.var_v_sb_dn9 - (0.5 * (locals.var_temp__blk949_dn9 - (((locals.var_temp__blk949_dn9 * assign47530_e60962) + (assign47530_e60959 * locals.var_temp__blk949_dn9)) / (2.0 * assign47530_e60966))))),)
    } else {
        (locals.var_vsbstaredge, locals.var_vsbstaredge_dn4, locals.var_vsbstaredge_dn6, locals.var_vsbstaredge_dn7, locals.var_vsbstaredge_dn8, locals.var_vsbstaredge_dn9,)
    }
};
        locals.var_vsbstaredge = assign47530_e60973;
        locals.var_vsbstaredge_dn4 = assign47530_e60973_d_n4;
        locals.var_vsbstaredge_dn6 = assign47530_e60973_d_n6;
        locals.var_vsbstaredge_dn7 = assign47530_e60973_d_n7;
        locals.var_vsbstaredge_dn8 = assign47530_e60973_d_n8;
        locals.var_vsbstaredge_dn9 = assign47530_e60973_d_n9;

        let (assign47540_e60983, assign47540_e60983_d_n4, assign47540_e60983_d_n6, assign47540_e60983_d_n7, assign47540_e60983_d_n8, assign47540_e60983_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47540_e60979: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign47540_e60980: f64 = (0.5 * assign47540_e60979);
        let assign47540_e60981: f64 = (locals.var_vsbstaredge + assign47540_e60980);
        (assign47540_e60981, locals.var_vsbstaredge_dn4, locals.var_vsbstaredge_dn6, (locals.var_vsbstaredge_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), (locals.var_vsbstaredge_dn8 + (0.5 * (locals.var_v_ds_dn8 - locals.var_vdsx_dn8))), locals.var_vsbstaredge_dn9,)
    } else {
        (locals.var_vsbxedge, locals.var_vsbxedge_dn4, locals.var_vsbxedge_dn6, locals.var_vsbxedge_dn7, locals.var_vsbxedge_dn8, locals.var_vsbxedge_dn9,)
    }
};
        locals.var_vsbxedge = assign47540_e60983;
        locals.var_vsbxedge_dn4 = assign47540_e60983_d_n4;
        locals.var_vsbxedge_dn6 = assign47540_e60983_d_n6;
        locals.var_vsbxedge_dn7 = assign47540_e60983_d_n7;
        locals.var_vsbxedge_dn8 = assign47540_e60983_d_n8;
        locals.var_vsbxedge_dn9 = assign47540_e60983_d_n9;

        let (assign47550_e60999, assign47550_e60999_d_n4, assign47550_e60999_d_n6, assign47550_e60999_d_n7, assign47550_e60999_d_n8, assign47550_e60999_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47550_e60989: f64 = (locals.var_pscededge_i * locals.var_vdsx);
        let assign47550_e60990: f64 = (1.0 + assign47550_e60989);
        let assign47550_e60991: f64 = (locals.var_psceedge_i * assign47550_e60990);
        let assign47550_e60995: f64 = (locals.var_pscebedge_i * locals.var_vsbxedge);
        let assign47550_e60996: f64 = (1.0 + assign47550_e60995);
        let assign47550_e60997: f64 = (assign47550_e60991 * assign47550_e60996);
        (assign47550_e60997, (assign47550_e60991 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn4)), (assign47550_e60991 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn6)), (((locals.var_psceedge_i * (locals.var_pscededge_i * locals.var_vdsx_dn7)) * assign47550_e60996) + (assign47550_e60991 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn7))), (((locals.var_psceedge_i * (locals.var_pscededge_i * locals.var_vdsx_dn8)) * assign47550_e60996) + (assign47550_e60991 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn8))), (assign47550_e60991 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn9)),)
    } else {
        (locals.var_dphit1edge, locals.var_dphit1edge_dn4, locals.var_dphit1edge_dn6, locals.var_dphit1edge_dn7, locals.var_dphit1edge_dn8, locals.var_dphit1edge_dn9,)
    }
};
        locals.var_dphit1edge = assign47550_e60999;
        locals.var_dphit1edge_dn4 = assign47550_e60999_d_n4;
        locals.var_dphit1edge_dn6 = assign47550_e60999_d_n6;
        locals.var_dphit1edge_dn7 = assign47550_e60999_d_n7;
        locals.var_dphit1edge_dn8 = assign47550_e60999_d_n8;
        locals.var_dphit1edge_dn9 = assign47550_e60999_d_n9;

        let (assign47560_e61007, assign47560_e61007_d_n4, assign47560_e61007_d_n6, assign47560_e61007_d_n7, assign47560_e61007_d_n8, assign47560_e61007_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47560_e61004: f64 = (1.0 + locals.var_dphit1edge);
        let assign47560_e61005: f64 = (locals.var_phit0edge * assign47560_e61004);
        (assign47560_e61005, ((locals.var_phit0edge_dn4 * assign47560_e61004) + (locals.var_phit0edge * locals.var_dphit1edge_dn4)), (locals.var_phit0edge * locals.var_dphit1edge_dn6), (locals.var_phit0edge * locals.var_dphit1edge_dn7), (locals.var_phit0edge * locals.var_dphit1edge_dn8), (locals.var_phit0edge * locals.var_dphit1edge_dn9),)
    } else {
        (locals.var_phit1edge, locals.var_phit1edge_dn4, locals.var_phit1edge_dn6, locals.var_phit1edge_dn7, locals.var_phit1edge_dn8, locals.var_phit1edge_dn9,)
    }
};
        locals.var_phit1edge = assign47560_e61007;
        locals.var_phit1edge_dn4 = assign47560_e61007_d_n4;
        locals.var_phit1edge_dn6 = assign47560_e61007_d_n6;
        locals.var_phit1edge_dn7 = assign47560_e61007_d_n7;
        locals.var_phit1edge_dn8 = assign47560_e61007_d_n8;
        locals.var_phit1edge_dn9 = assign47560_e61007_d_n9;

        let (assign47570_e61013, assign47570_e61013_d_n4, assign47570_e61013_d_n6, assign47570_e61013_d_n7, assign47570_e61013_d_n8, assign47570_e61013_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47570_e61011: f64 = (1.0 / locals.var_phit1edge);
        (assign47570_e61011, (-(locals.var_phit1edge_dn4 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn6 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn7 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn8 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn9 / (locals.var_phit1edge * locals.var_phit1edge))),)
    } else {
        (locals.var_inv_phit1edge, locals.var_inv_phit1edge_dn4, locals.var_inv_phit1edge_dn6, locals.var_inv_phit1edge_dn7, locals.var_inv_phit1edge_dn8, locals.var_inv_phit1edge_dn9,)
    }
};
        locals.var_inv_phit1edge = assign47570_e61013;
        locals.var_inv_phit1edge_dn4 = assign47570_e61013_d_n4;
        locals.var_inv_phit1edge_dn6 = assign47570_e61013_d_n6;
        locals.var_inv_phit1edge_dn7 = assign47570_e61013_d_n7;
        locals.var_inv_phit1edge_dn8 = assign47570_e61013_d_n8;
        locals.var_inv_phit1edge_dn9 = assign47570_e61013_d_n9;

        let (assign47580_e61028, assign47580_e61028_d_n7, assign47580_e61028_d_n8,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47580_e61017: f64 = (2.0 * locals.var_vdsx);
        let assign47580_e61022: f64 = (locals.var_cfdedge_i * locals.var_vdsx);
        let assign47580_e61023: f64 = (1.0 + assign47580_e61022);
        let assign47580_e61024: f64 = (assign47580_e61023).sqrt();
        let assign47580_e61025: f64 = (1.0 + assign47580_e61024);
        let assign47580_e61026: f64 = (assign47580_e61017 / assign47580_e61025);
        (assign47580_e61026, ((((2.0 * locals.var_vdsx_dn7) * assign47580_e61025) - (assign47580_e61017 * ((locals.var_cfdedge_i * locals.var_vdsx_dn7) / (2.0 * assign47580_e61024)))) / (assign47580_e61025 * assign47580_e61025)), ((((2.0 * locals.var_vdsx_dn8) * assign47580_e61025) - (assign47580_e61017 * ((locals.var_cfdedge_i * locals.var_vdsx_dn8) / (2.0 * assign47580_e61024)))) / (assign47580_e61025 * assign47580_e61025)),)
    } else {
        (locals.var_vdspedge, locals.var_vdspedge_dn7, locals.var_vdspedge_dn8,)
    }
};
        locals.var_vdspedge = assign47580_e61028;
        locals.var_vdspedge_dn7 = assign47580_e61028_d_n7;
        locals.var_vdspedge_dn8 = assign47580_e61028_d_n8;

        let (assign47590_e61040, assign47590_e61040_d_n4, assign47590_e61040_d_n6, assign47590_e61040_d_n7, assign47590_e61040_d_n8, assign47590_e61040_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47590_e61032: f64 = (locals.var_cfedge_i * locals.var_vdspedge);
        let assign47590_e61036: f64 = (locals.var_cfbedge_i * locals.var_vsbxedge);
        let assign47590_e61037: f64 = (1.0 + assign47590_e61036);
        let assign47590_e61038: f64 = (assign47590_e61032 * assign47590_e61037);
        (assign47590_e61038, (assign47590_e61032 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn4)), (assign47590_e61032 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn6)), (((locals.var_cfedge_i * locals.var_vdspedge_dn7) * assign47590_e61037) + (assign47590_e61032 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn7))), (((locals.var_cfedge_i * locals.var_vdspedge_dn8) * assign47590_e61037) + (assign47590_e61032 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn8))), (assign47590_e61032 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn9)),)
    } else {
        (locals.var_delvgedge, locals.var_delvgedge_dn4, locals.var_delvgedge_dn6, locals.var_delvgedge_dn7, locals.var_delvgedge_dn8, locals.var_delvgedge_dn9,)
    }
};
        locals.var_delvgedge = assign47590_e61040;
        locals.var_delvgedge_dn4 = assign47590_e61040_d_n4;
        locals.var_delvgedge_dn6 = assign47590_e61040_d_n6;
        locals.var_delvgedge_dn7 = assign47590_e61040_d_n7;
        locals.var_delvgedge_dn8 = assign47590_e61040_d_n8;
        locals.var_delvgedge_dn9 = assign47590_e61040_d_n9;

        let (assign47600_e61050, assign47600_e61050_d_n4, assign47600_e61050_d_n6, assign47600_e61050_d_n7, assign47600_e61050_d_n8, assign47600_e61050_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47600_e61045: f64 = (locals.var_vgb + locals.var_delvgedge);
        let assign47600_e61047: f64 = (assign47600_e61045 - locals.var_vfbedge_t);
        let assign47600_e61048: f64 = (locals.var_inv_phit1edge * assign47600_e61047);
        (assign47600_e61048, ((locals.var_inv_phit1edge_dn4 * assign47600_e61047) + (locals.var_inv_phit1edge * (locals.var_delvgedge_dn4 - locals.var_vfbedge_t_dn4))), ((locals.var_inv_phit1edge_dn6 * assign47600_e61047) + (locals.var_inv_phit1edge * (locals.var_vgb_dn6 + locals.var_delvgedge_dn6))), ((locals.var_inv_phit1edge_dn7 * assign47600_e61047) + (locals.var_inv_phit1edge * (locals.var_vgb_dn7 + locals.var_delvgedge_dn7))), ((locals.var_inv_phit1edge_dn8 * assign47600_e61047) + (locals.var_inv_phit1edge * (locals.var_vgb_dn8 + locals.var_delvgedge_dn8))), ((locals.var_inv_phit1edge_dn9 * assign47600_e61047) + (locals.var_inv_phit1edge * (locals.var_vgb_dn9 + locals.var_delvgedge_dn9))),)
    } else {
        (locals.var_xgedge, locals.var_xgedge_dn4, locals.var_xgedge_dn6, locals.var_xgedge_dn7, locals.var_xgedge_dn8, locals.var_xgedge_dn9,)
    }
};
        locals.var_xgedge = assign47600_e61050;
        locals.var_xgedge_dn4 = assign47600_e61050_d_n4;
        locals.var_xgedge_dn6 = assign47600_e61050_d_n6;
        locals.var_xgedge_dn7 = assign47600_e61050_d_n7;
        locals.var_xgedge_dn8 = assign47600_e61050_d_n8;
        locals.var_xgedge_dn9 = assign47600_e61050_d_n9;

        let (assign47610_e61056, assign47610_e61056_d_n4, assign47610_e61056_d_n6, assign47610_e61056_d_n7, assign47610_e61056_d_n8, assign47610_e61056_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47610_e61054: f64 = (locals.var_inv_phit1edge * locals.var_phibedge);
        (assign47610_e61054, ((locals.var_inv_phit1edge_dn4 * locals.var_phibedge) + (locals.var_inv_phit1edge * locals.var_phibedge_dn4)), (locals.var_inv_phit1edge_dn6 * locals.var_phibedge), (locals.var_inv_phit1edge_dn7 * locals.var_phibedge), (locals.var_inv_phit1edge_dn8 * locals.var_phibedge), (locals.var_inv_phit1edge_dn9 * locals.var_phibedge),)
    } else {
        (locals.var_xbedge, locals.var_xbedge_dn4, locals.var_xbedge_dn6, locals.var_xbedge_dn7, locals.var_xbedge_dn8, locals.var_xbedge_dn9,)
    }
};
        locals.var_xbedge = assign47610_e61056;
        locals.var_xbedge_dn4 = assign47610_e61056_d_n4;
        locals.var_xbedge_dn6 = assign47610_e61056_d_n6;
        locals.var_xbedge_dn7 = assign47610_e61056_d_n7;
        locals.var_xbedge_dn8 = assign47610_e61056_d_n8;
        locals.var_xbedge_dn9 = assign47610_e61056_d_n9;

        let (assign47620_e61068, assign47620_e61068_d_n4, assign47620_e61068_d_n6, assign47620_e61068_d_n7, assign47620_e61068_d_n8, assign47620_e61068_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47620_e61061: f64 = (locals.var_xbedge / locals.var_gfedge);
        let assign47620_e61063: f64 = (locals.var_xbedge).sqrt();
        let assign47620_e61064: f64 = (assign47620_e61061 + assign47620_e61063);
        let assign47620_e61065: f64 = (assign47620_e61064).ln();
        let assign47620_e61066: f64 = (2.0 * assign47620_e61065);
        (assign47620_e61066, (2.0 * (((((locals.var_xbedge_dn4 * locals.var_gfedge) - (locals.var_xbedge * locals.var_gfedge_dn4)) / (locals.var_gfedge * locals.var_gfedge)) + (locals.var_xbedge_dn4 / (2.0 * assign47620_e61063))) / assign47620_e61064)), (2.0 * (((locals.var_xbedge_dn6 / locals.var_gfedge) + (locals.var_xbedge_dn6 / (2.0 * assign47620_e61063))) / assign47620_e61064)), (2.0 * (((locals.var_xbedge_dn7 / locals.var_gfedge) + (locals.var_xbedge_dn7 / (2.0 * assign47620_e61063))) / assign47620_e61064)), (2.0 * (((locals.var_xbedge_dn8 / locals.var_gfedge) + (locals.var_xbedge_dn8 / (2.0 * assign47620_e61063))) / assign47620_e61064)), (2.0 * (((locals.var_xbedge_dn9 / locals.var_gfedge) + (locals.var_xbedge_dn9 / (2.0 * assign47620_e61063))) / assign47620_e61064)),)
    } else {
        (locals.var_dxthedge, locals.var_dxthedge_dn4, locals.var_dxthedge_dn6, locals.var_dxthedge_dn7, locals.var_dxthedge_dn8, locals.var_dxthedge_dn9,)
    }
};
        locals.var_dxthedge = assign47620_e61068;
        locals.var_dxthedge_dn4 = assign47620_e61068_d_n4;
        locals.var_dxthedge_dn6 = assign47620_e61068_d_n6;
        locals.var_dxthedge_dn7 = assign47620_e61068_d_n7;
        locals.var_dxthedge_dn8 = assign47620_e61068_d_n8;
        locals.var_dxthedge_dn9 = assign47620_e61068_d_n9;

        let (assign47630_e61074, assign47630_e61074_d_n4, assign47630_e61074_d_n6, assign47630_e61074_d_n7, assign47630_e61074_d_n8, assign47630_e61074_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47630_e61072: f64 = (locals.var_inv_phit1edge * locals.var_vsbstaredge);
        (assign47630_e61072, ((locals.var_inv_phit1edge_dn4 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn4)), ((locals.var_inv_phit1edge_dn6 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn6)), ((locals.var_inv_phit1edge_dn7 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn7)), ((locals.var_inv_phit1edge_dn8 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn8)), ((locals.var_inv_phit1edge_dn9 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn9)),)
    } else {
        (locals.var_xnedge_s, locals.var_xnedge_s_dn4, locals.var_xnedge_s_dn6, locals.var_xnedge_s_dn7, locals.var_xnedge_s_dn8, locals.var_xnedge_s_dn9,)
    }
};
        locals.var_xnedge_s = assign47630_e61074;
        locals.var_xnedge_s_dn4 = assign47630_e61074_d_n4;
        locals.var_xnedge_s_dn6 = assign47630_e61074_d_n6;
        locals.var_xnedge_s_dn7 = assign47630_e61074_d_n7;
        locals.var_xnedge_s_dn8 = assign47630_e61074_d_n8;
        locals.var_xnedge_s_dn9 = assign47630_e61074_d_n9;

    }

    pub(super) fn stamp_transient_block_33(
        locals: &mut StampLocals,
    ) {
        let (assign47640_e61080, assign47640_e61080_d_n4, assign47640_e61080_d_n6, assign47640_e61080_d_n7, assign47640_e61080_d_n8, assign47640_e61080_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47640_e61078: f64 = (locals.var_xbedge + locals.var_xnedge_s);
        (assign47640_e61078, (locals.var_xbedge_dn4 + locals.var_xnedge_s_dn4), (locals.var_xbedge_dn6 + locals.var_xnedge_s_dn6), (locals.var_xbedge_dn7 + locals.var_xnedge_s_dn7), (locals.var_xbedge_dn8 + locals.var_xnedge_s_dn8), (locals.var_xbedge_dn9 + locals.var_xnedge_s_dn9),)
    } else {
        (locals.var_q_edge_xsth, locals.var_q_edge_xsth_dn4, locals.var_q_edge_xsth_dn6, locals.var_q_edge_xsth_dn7, locals.var_q_edge_xsth_dn8, locals.var_q_edge_xsth_dn9,)
    }
};
        locals.var_q_edge_xsth = assign47640_e61080;
        locals.var_q_edge_xsth_dn4 = assign47640_e61080_d_n4;
        locals.var_q_edge_xsth_dn6 = assign47640_e61080_d_n6;
        locals.var_q_edge_xsth_dn7 = assign47640_e61080_d_n7;
        locals.var_q_edge_xsth_dn8 = assign47640_e61080_d_n8;
        locals.var_q_edge_xsth_dn9 = assign47640_e61080_d_n9;

        let (assign47650_e61089, assign47650_e61089_d_n4, assign47650_e61089_d_n6, assign47650_e61089_d_n7, assign47650_e61089_d_n8, assign47650_e61089_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47650_e61085: f64 = (locals.var_q_edge_xsth).sqrt();
        let assign47650_e61086: f64 = (locals.var_gfedge * assign47650_e61085);
        let assign47650_e61087: f64 = (locals.var_q_edge_xsth + assign47650_e61086);
        (assign47650_e61087, (locals.var_q_edge_xsth_dn4 + ((locals.var_gfedge_dn4 * assign47650_e61085) + (locals.var_gfedge * (locals.var_q_edge_xsth_dn4 / (2.0 * assign47650_e61085))))), (locals.var_q_edge_xsth_dn6 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47650_e61085)))), (locals.var_q_edge_xsth_dn7 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47650_e61085)))), (locals.var_q_edge_xsth_dn8 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47650_e61085)))), (locals.var_q_edge_xsth_dn9 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn9 / (2.0 * assign47650_e61085)))),)
    } else {
        (locals.var_q_edge_xth0, locals.var_q_edge_xth0_dn4, locals.var_q_edge_xth0_dn6, locals.var_q_edge_xth0_dn7, locals.var_q_edge_xth0_dn8, locals.var_q_edge_xth0_dn9,)
    }
};
        locals.var_q_edge_xth0 = assign47650_e61089;
        locals.var_q_edge_xth0_dn4 = assign47650_e61089_d_n4;
        locals.var_q_edge_xth0_dn6 = assign47650_e61089_d_n6;
        locals.var_q_edge_xth0_dn7 = assign47650_e61089_d_n7;
        locals.var_q_edge_xth0_dn8 = assign47650_e61089_d_n8;
        locals.var_q_edge_xth0_dn9 = assign47650_e61089_d_n9;

        let (assign47660_e61095, assign47660_e61095_d_n4, assign47660_e61095_d_n6, assign47660_e61095_d_n7, assign47660_e61095_d_n8, assign47660_e61095_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47660_e61093: f64 = (locals.var_q_edge_xth0 + locals.var_dxthedge);
        (assign47660_e61093, (locals.var_q_edge_xth0_dn4 + locals.var_dxthedge_dn4), (locals.var_q_edge_xth0_dn6 + locals.var_dxthedge_dn6), (locals.var_q_edge_xth0_dn7 + locals.var_dxthedge_dn7), (locals.var_q_edge_xth0_dn8 + locals.var_dxthedge_dn8), (locals.var_q_edge_xth0_dn9 + locals.var_dxthedge_dn9),)
    } else {
        (locals.var_q_edge_xth, locals.var_q_edge_xth_dn4, locals.var_q_edge_xth_dn6, locals.var_q_edge_xth_dn7, locals.var_q_edge_xth_dn8, locals.var_q_edge_xth_dn9,)
    }
};
        locals.var_q_edge_xth = assign47660_e61095;
        locals.var_q_edge_xth_dn4 = assign47660_e61095_d_n4;
        locals.var_q_edge_xth_dn6 = assign47660_e61095_d_n6;
        locals.var_q_edge_xth_dn7 = assign47660_e61095_d_n7;
        locals.var_q_edge_xth_dn8 = assign47660_e61095_d_n8;
        locals.var_q_edge_xth_dn9 = assign47660_e61095_d_n9;

        let (assign47670_e61106, assign47670_e61106_d_n4, assign47670_e61106_d_n6, assign47670_e61106_d_n7, assign47670_e61106_d_n8, assign47670_e61106_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47670_e61101: f64 = (locals.var_q_edge_xsth).sqrt();
        let assign47670_e61102: f64 = (2.0 * assign47670_e61101);
        let assign47670_e61103: f64 = (locals.var_gfedge / assign47670_e61102);
        let assign47670_e61104: f64 = (1.0 + assign47670_e61103);
        (assign47670_e61104, (((locals.var_gfedge_dn4 * assign47670_e61102) - (locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn4 / (2.0 * assign47670_e61101))))) / (assign47670_e61102 * assign47670_e61102)), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47670_e61101)))) / (assign47670_e61102 * assign47670_e61102))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47670_e61101)))) / (assign47670_e61102 * assign47670_e61102))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47670_e61101)))) / (assign47670_e61102 * assign47670_e61102))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn9 / (2.0 * assign47670_e61101)))) / (assign47670_e61102 * assign47670_e61102))),)
    } else {
        (locals.var_q_edge_n, locals.var_q_edge_n_dn4, locals.var_q_edge_n_dn6, locals.var_q_edge_n_dn7, locals.var_q_edge_n_dn8, locals.var_q_edge_n_dn9,)
    }
};
        locals.var_q_edge_n = assign47670_e61106;
        locals.var_q_edge_n_dn4 = assign47670_e61106_d_n4;
        locals.var_q_edge_n_dn6 = assign47670_e61106_d_n6;
        locals.var_q_edge_n_dn7 = assign47670_e61106_d_n7;
        locals.var_q_edge_n_dn8 = assign47670_e61106_d_n8;
        locals.var_q_edge_n_dn9 = assign47670_e61106_d_n9;

        let (assign47680_e61112, assign47680_e61112_d_n4, assign47680_e61112_d_n6, assign47680_e61112_d_n7, assign47680_e61112_d_n8, assign47680_e61112_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47680_e61110: f64 = (1.0 / locals.var_q_edge_n);
        (assign47680_e61110, (-(locals.var_q_edge_n_dn4 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn6 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn7 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn8 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn9 / (locals.var_q_edge_n * locals.var_q_edge_n))),)
    } else {
        (locals.var_q_edge_n_inv, locals.var_q_edge_n_inv_dn4, locals.var_q_edge_n_inv_dn6, locals.var_q_edge_n_inv_dn7, locals.var_q_edge_n_inv_dn8, locals.var_q_edge_n_inv_dn9,)
    }
};
        locals.var_q_edge_n_inv = assign47680_e61112;
        locals.var_q_edge_n_inv_dn4 = assign47680_e61112_d_n4;
        locals.var_q_edge_n_inv_dn6 = assign47680_e61112_d_n6;
        locals.var_q_edge_n_inv_dn7 = assign47680_e61112_d_n7;
        locals.var_q_edge_n_inv_dn8 = assign47680_e61112_d_n8;
        locals.var_q_edge_n_inv_dn9 = assign47680_e61112_d_n9;

        let (assign47690_e61118, assign47690_e61118_d_n4, assign47690_e61118_d_n6, assign47690_e61118_d_n7, assign47690_e61118_d_n8, assign47690_e61118_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47690_e61116: f64 = (locals.var_xgedge - locals.var_q_edge_xth);
        (assign47690_e61116, (locals.var_xgedge_dn4 - locals.var_q_edge_xth_dn4), (locals.var_xgedge_dn6 - locals.var_q_edge_xth_dn6), (locals.var_xgedge_dn7 - locals.var_q_edge_xth_dn7), (locals.var_xgedge_dn8 - locals.var_q_edge_xth_dn8), (locals.var_xgedge_dn9 - locals.var_q_edge_xth_dn9),)
    } else {
        (locals.var_q_edge_xgt, locals.var_q_edge_xgt_dn4, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8, locals.var_q_edge_xgt_dn9,)
    }
};
        locals.var_q_edge_xgt = assign47690_e61118;
        locals.var_q_edge_xgt_dn4 = assign47690_e61118_d_n4;
        locals.var_q_edge_xgt_dn6 = assign47690_e61118_d_n6;
        locals.var_q_edge_xgt_dn7 = assign47690_e61118_d_n7;
        locals.var_q_edge_xgt_dn8 = assign47690_e61118_d_n8;
        locals.var_q_edge_xgt_dn9 = assign47690_e61118_d_n9;

        let assign47700_e61121: f64 = (-12.0);
        let assign47700_e61122: f64 = if locals.var_q_edge_xgt > assign47700_e61121 { 1.0 } else { 0.0 };
        locals.var_guard1267 = assign47700_e61122;

        let (assign47710_e61132, assign47710_e61132_d_n4, assign47710_e61132_d_n6, assign47710_e61132_d_n7, assign47710_e61132_d_n8, assign47710_e61132_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47710_e61128: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47710_e61130: f64 = (assign47710_e61128 - 1.0);
        (assign47710_e61130, (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4), locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8, locals.var_q_edge_xgt_dn9,)
    } else {
        (locals.var_q_edge_xgt0, locals.var_q_edge_xgt0_dn4, locals.var_q_edge_xgt0_dn6, locals.var_q_edge_xgt0_dn7, locals.var_q_edge_xgt0_dn8, locals.var_q_edge_xgt0_dn9,)
    }
};
        locals.var_q_edge_xgt0 = assign47710_e61132;
        locals.var_q_edge_xgt0_dn4 = assign47710_e61132_d_n4;
        locals.var_q_edge_xgt0_dn6 = assign47710_e61132_d_n6;
        locals.var_q_edge_xgt0_dn7 = assign47710_e61132_d_n7;
        locals.var_q_edge_xgt0_dn8 = assign47710_e61132_d_n8;
        locals.var_q_edge_xgt0_dn9 = assign47710_e61132_d_n9;

        let (assign47720_e61147, assign47720_e61147_d_n4, assign47720_e61147_d_n6, assign47720_e61147_d_n7, assign47720_e61147_d_n8, assign47720_e61147_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47720_e61140: f64 = (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0);
        let assign47720_e61142: f64 = (assign47720_e61140 + 10.0);
        let assign47720_e61143: f64 = (assign47720_e61142).sqrt();
        let assign47720_e61144: f64 = (locals.var_q_edge_xgt0 + assign47720_e61143);
        let assign47720_e61145: f64 = (0.5 * assign47720_e61144);
        (assign47720_e61145, (0.5 * (locals.var_q_edge_xgt0_dn4 + (((locals.var_q_edge_xgt0_dn4 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn4)) / (2.0 * assign47720_e61143)))), (0.5 * (locals.var_q_edge_xgt0_dn6 + (((locals.var_q_edge_xgt0_dn6 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn6)) / (2.0 * assign47720_e61143)))), (0.5 * (locals.var_q_edge_xgt0_dn7 + (((locals.var_q_edge_xgt0_dn7 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn7)) / (2.0 * assign47720_e61143)))), (0.5 * (locals.var_q_edge_xgt0_dn8 + (((locals.var_q_edge_xgt0_dn8 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn8)) / (2.0 * assign47720_e61143)))), (0.5 * (locals.var_q_edge_xgt0_dn9 + (((locals.var_q_edge_xgt0_dn9 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn9)) / (2.0 * assign47720_e61143)))),)
    } else {
        (locals.var_q_edge_xgt0e, locals.var_q_edge_xgt0e_dn4, locals.var_q_edge_xgt0e_dn6, locals.var_q_edge_xgt0e_dn7, locals.var_q_edge_xgt0e_dn8, locals.var_q_edge_xgt0e_dn9,)
    }
};
        locals.var_q_edge_xgt0e = assign47720_e61147;
        locals.var_q_edge_xgt0e_dn4 = assign47720_e61147_d_n4;
        locals.var_q_edge_xgt0e_dn6 = assign47720_e61147_d_n6;
        locals.var_q_edge_xgt0e_dn7 = assign47720_e61147_d_n7;
        locals.var_q_edge_xgt0e_dn8 = assign47720_e61147_d_n8;
        locals.var_q_edge_xgt0e_dn9 = assign47720_e61147_d_n9;

        let (assign47730_e61160, assign47730_e61160_d_n4, assign47730_e61160_d_n6, assign47730_e61160_d_n7, assign47730_e61160_d_n8, assign47730_e61160_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47730_e61154: f64 = (locals.var_q_edge_xgt0e).ln();
        let assign47730_e61155: f64 = (locals.var_q_edge_n * assign47730_e61154);
        let assign47730_e61156: f64 = (locals.var_q_edge_xgt - assign47730_e61155);
        let assign47730_e61158: f64 = (assign47730_e61156 + locals.var_lngfedge2);
        (assign47730_e61158, ((locals.var_q_edge_xgt_dn4 - ((locals.var_q_edge_n_dn4 * assign47730_e61154) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn4 / locals.var_q_edge_xgt0e)))) + locals.var_lngfedge2_dn4), (locals.var_q_edge_xgt_dn6 - ((locals.var_q_edge_n_dn6 * assign47730_e61154) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn6 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn7 - ((locals.var_q_edge_n_dn7 * assign47730_e61154) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn7 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn8 - ((locals.var_q_edge_n_dn8 * assign47730_e61154) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn8 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn9 - ((locals.var_q_edge_n_dn9 * assign47730_e61154) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn9 / locals.var_q_edge_xgt0e)))),)
    } else {
        (locals.var_q_edge_qi0si, locals.var_q_edge_qi0si_dn4, locals.var_q_edge_qi0si_dn6, locals.var_q_edge_qi0si_dn7, locals.var_q_edge_qi0si_dn8, locals.var_q_edge_qi0si_dn9,)
    }
};
        locals.var_q_edge_qi0si = assign47730_e61160;
        locals.var_q_edge_qi0si_dn4 = assign47730_e61160_d_n4;
        locals.var_q_edge_qi0si_dn6 = assign47730_e61160_d_n6;
        locals.var_q_edge_qi0si_dn7 = assign47730_e61160_d_n7;
        locals.var_q_edge_qi0si_dn8 = assign47730_e61160_d_n8;
        locals.var_q_edge_qi0si_dn9 = assign47730_e61160_d_n9;

        let (assign47740_e61175, assign47740_e61175_d_n4, assign47740_e61175_d_n6, assign47740_e61175_d_n7, assign47740_e61175_d_n8, assign47740_e61175_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47740_e61168: f64 = (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si);
        let assign47740_e61170: f64 = (assign47740_e61168 + 2.0);
        let assign47740_e61171: f64 = (assign47740_e61170).sqrt();
        let assign47740_e61172: f64 = (locals.var_q_edge_qi0si + assign47740_e61171);
        let assign47740_e61173: f64 = (0.5 * assign47740_e61172);
        (assign47740_e61173, (0.5 * (locals.var_q_edge_qi0si_dn4 + (((locals.var_q_edge_qi0si_dn4 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn4)) / (2.0 * assign47740_e61171)))), (0.5 * (locals.var_q_edge_qi0si_dn6 + (((locals.var_q_edge_qi0si_dn6 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn6)) / (2.0 * assign47740_e61171)))), (0.5 * (locals.var_q_edge_qi0si_dn7 + (((locals.var_q_edge_qi0si_dn7 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn7)) / (2.0 * assign47740_e61171)))), (0.5 * (locals.var_q_edge_qi0si_dn8 + (((locals.var_q_edge_qi0si_dn8 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn8)) / (2.0 * assign47740_e61171)))), (0.5 * (locals.var_q_edge_qi0si_dn9 + (((locals.var_q_edge_qi0si_dn9 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn9)) / (2.0 * assign47740_e61171)))),)
    } else {
        (locals.var_q_edge_qi0, locals.var_q_edge_qi0_dn4, locals.var_q_edge_qi0_dn6, locals.var_q_edge_qi0_dn7, locals.var_q_edge_qi0_dn8, locals.var_q_edge_qi0_dn9,)
    }
};
        locals.var_q_edge_qi0 = assign47740_e61175;
        locals.var_q_edge_qi0_dn4 = assign47740_e61175_d_n4;
        locals.var_q_edge_qi0_dn6 = assign47740_e61175_d_n6;
        locals.var_q_edge_qi0_dn7 = assign47740_e61175_d_n7;
        locals.var_q_edge_qi0_dn8 = assign47740_e61175_d_n8;
        locals.var_q_edge_qi0_dn9 = assign47740_e61175_d_n9;

        let assign47750_e61178: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47750_e61180: f64 = if assign47750_e61178 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1268 = assign47750_e61180;

        let (assign47760_e61191, assign47760_e61191_d_n4, assign47760_e61191_d_n6, assign47760_e61191_d_n7, assign47760_e61191_d_n8, assign47760_e61191_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) && (locals.var_guard1268 != 0.0)) {
        let assign47760_e61188: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47760_e61189: f64 = (assign47760_e61188).exp();
        (assign47760_e61189, (assign47760_e61189 * (locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4)), (assign47760_e61189 * (locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6)), (assign47760_e61189 * (locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7)), (assign47760_e61189 * (locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8)), (assign47760_e61189 * (locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9)),)
    } else {
        (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn4, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8, locals.var_q_edge_exp_x_dn9,)
    }
};
        locals.var_q_edge_exp_x = assign47760_e61191;
        locals.var_q_edge_exp_x_dn4 = assign47760_e61191_d_n4;
        locals.var_q_edge_exp_x_dn6 = assign47760_e61191_d_n6;
        locals.var_q_edge_exp_x_dn7 = assign47760_e61191_d_n7;
        locals.var_q_edge_exp_x_dn8 = assign47760_e61191_d_n8;
        locals.var_q_edge_exp_x_dn9 = assign47760_e61191_d_n9;

        let (assign47770_e61228, assign47770_e61228_d_n4, assign47770_e61228_d_n6, assign47770_e61228_d_n7, assign47770_e61228_d_n8, assign47770_e61228_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) && (locals.var_guard1268 == 0.0)) {
        let assign47770_e61202: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47770_e61204: f64 = (assign47770_e61202 - 230.25850929940458);
        let assign47770_e61209: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47770_e61211: f64 = (assign47770_e61209 - 230.25850929940458);
        let assign47770_e61215: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47770_e61217: f64 = (assign47770_e61215 - 230.25850929940458);
        let assign47770_e61219: f64 = (assign47770_e61217 * 0.3333333333333333);
        let assign47770_e61220: f64 = (1.0 + assign47770_e61219);
        let assign47770_e61221: f64 = (assign47770_e61211 * assign47770_e61220);
        let assign47770_e61222: f64 = (0.5 * assign47770_e61221);
        let assign47770_e61223: f64 = (1.0 + assign47770_e61222);
        let assign47770_e61224: f64 = (assign47770_e61204 * assign47770_e61223);
        let assign47770_e61225: f64 = (1.0 + assign47770_e61224);
        let assign47770_e61226: f64 = (1e100 * assign47770_e61225);
        (assign47770_e61226, (1e100 * (((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * assign47770_e61223) + (assign47770_e61204 * (0.5 * (((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * assign47770_e61220) + (assign47770_e61211 * ((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47770_e61223) + (assign47770_e61204 * (0.5 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47770_e61220) + (assign47770_e61211 * ((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47770_e61223) + (assign47770_e61204 * (0.5 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47770_e61220) + (assign47770_e61211 * ((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47770_e61223) + (assign47770_e61204 * (0.5 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47770_e61220) + (assign47770_e61211 * ((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * assign47770_e61223) + (assign47770_e61204 * (0.5 * (((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * assign47770_e61220) + (assign47770_e61211 * ((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * 0.3333333333333333))))))),)
    } else {
        (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn4, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8, locals.var_q_edge_exp_x_dn9,)
    }
};
        locals.var_q_edge_exp_x = assign47770_e61228;
        locals.var_q_edge_exp_x_dn4 = assign47770_e61228_d_n4;
        locals.var_q_edge_exp_x_dn6 = assign47770_e61228_d_n6;
        locals.var_q_edge_exp_x_dn7 = assign47770_e61228_d_n7;
        locals.var_q_edge_exp_x_dn8 = assign47770_e61228_d_n8;
        locals.var_q_edge_exp_x_dn9 = assign47770_e61228_d_n9;

        let (assign47780_e61236, assign47780_e61236_d_n4, assign47780_e61236_d_n6, assign47780_e61236_d_n7, assign47780_e61236_d_n8, assign47780_e61236_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47780_e61234: f64 = (locals.var_gfedge2 * locals.var_q_edge_exp_x);
        (assign47780_e61234, ((locals.var_gfedge2_dn4 * locals.var_q_edge_exp_x) + (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn4)), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn6), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn7), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn8), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn9),)
    } else {
        (locals.var_q_edge_d0, locals.var_q_edge_d0_dn4, locals.var_q_edge_d0_dn6, locals.var_q_edge_d0_dn7, locals.var_q_edge_d0_dn8, locals.var_q_edge_d0_dn9,)
    }
};
        locals.var_q_edge_d0 = assign47780_e61236;
        locals.var_q_edge_d0_dn4 = assign47780_e61236_d_n4;
        locals.var_q_edge_d0_dn6 = assign47780_e61236_d_n6;
        locals.var_q_edge_d0_dn7 = assign47780_e61236_d_n7;
        locals.var_q_edge_d0_dn8 = assign47780_e61236_d_n8;
        locals.var_q_edge_d0_dn9 = assign47780_e61236_d_n9;

        let (assign47790_e61244, assign47790_e61244_d_n4, assign47790_e61244_d_n6, assign47790_e61244_d_n7, assign47790_e61244_d_n8, assign47790_e61244_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47790_e61242: f64 = (locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv);
        (assign47790_e61242, if locals.var_q_edge_n_inv_dn4 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn4)) } } else { (assign47790_e61242 * ((locals.var_q_edge_n_inv_dn4 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn4 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn6 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn6)) } } else { (assign47790_e61242 * ((locals.var_q_edge_n_inv_dn6 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn6 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn7 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn7)) } } else { (assign47790_e61242 * ((locals.var_q_edge_n_inv_dn7 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn7 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn8 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn8)) } } else { (assign47790_e61242 * ((locals.var_q_edge_n_inv_dn8 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn8 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn9 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn9)) } } else { (assign47790_e61242 * ((locals.var_q_edge_n_inv_dn9 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn9 / locals.var_q_edge_d0)))) },)
    } else {
        (locals.var_q_edge_d0p, locals.var_q_edge_d0p_dn4, locals.var_q_edge_d0p_dn6, locals.var_q_edge_d0p_dn7, locals.var_q_edge_d0p_dn8, locals.var_q_edge_d0p_dn9,)
    }
};
        locals.var_q_edge_d0p = assign47790_e61244;
        locals.var_q_edge_d0p_dn4 = assign47790_e61244_d_n4;
        locals.var_q_edge_d0p_dn6 = assign47790_e61244_d_n6;
        locals.var_q_edge_d0p_dn7 = assign47790_e61244_d_n7;
        locals.var_q_edge_d0p_dn8 = assign47790_e61244_d_n8;
        locals.var_q_edge_d0p_dn9 = assign47790_e61244_d_n9;

        let (assign47800_e61262, assign47800_e61262_d_n4, assign47800_e61262_d_n6, assign47800_e61262_d_n7, assign47800_e61262_d_n8, assign47800_e61262_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47800_e61250: f64 = (locals.var_q_edge_n * locals.var_q_edge_n);
        let assign47800_e61254: f64 = (locals.var_q_edge_qi0 + locals.var_q_edge_n);
        let assign47800_e61255: f64 = (2.0 * assign47800_e61254);
        let assign47800_e61257: f64 = (assign47800_e61255 - locals.var_q_edge_d0p);
        let assign47800_e61259: f64 = (assign47800_e61257 * locals.var_q_edge_d0p);
        let assign47800_e61260: f64 = (assign47800_e61250 + assign47800_e61259);
        (assign47800_e61260, (((locals.var_q_edge_n_dn4 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn4)) + ((((2.0 * (locals.var_q_edge_qi0_dn4 + locals.var_q_edge_n_dn4)) - locals.var_q_edge_d0p_dn4) * locals.var_q_edge_d0p) + (assign47800_e61257 * locals.var_q_edge_d0p_dn4))), (((locals.var_q_edge_n_dn6 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn6)) + ((((2.0 * (locals.var_q_edge_qi0_dn6 + locals.var_q_edge_n_dn6)) - locals.var_q_edge_d0p_dn6) * locals.var_q_edge_d0p) + (assign47800_e61257 * locals.var_q_edge_d0p_dn6))), (((locals.var_q_edge_n_dn7 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn7)) + ((((2.0 * (locals.var_q_edge_qi0_dn7 + locals.var_q_edge_n_dn7)) - locals.var_q_edge_d0p_dn7) * locals.var_q_edge_d0p) + (assign47800_e61257 * locals.var_q_edge_d0p_dn7))), (((locals.var_q_edge_n_dn8 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn8)) + ((((2.0 * (locals.var_q_edge_qi0_dn8 + locals.var_q_edge_n_dn8)) - locals.var_q_edge_d0p_dn8) * locals.var_q_edge_d0p) + (assign47800_e61257 * locals.var_q_edge_d0p_dn8))), (((locals.var_q_edge_n_dn9 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn9)) + ((((2.0 * (locals.var_q_edge_qi0_dn9 + locals.var_q_edge_n_dn9)) - locals.var_q_edge_d0p_dn9) * locals.var_q_edge_d0p) + (assign47800_e61257 * locals.var_q_edge_d0p_dn9))),)
    } else {
        (locals.var_q_edge_sqerr, locals.var_q_edge_sqerr_dn4, locals.var_q_edge_sqerr_dn6, locals.var_q_edge_sqerr_dn7, locals.var_q_edge_sqerr_dn8, locals.var_q_edge_sqerr_dn9,)
    }
};
        locals.var_q_edge_sqerr = assign47800_e61262;
        locals.var_q_edge_sqerr_dn4 = assign47800_e61262_d_n4;
        locals.var_q_edge_sqerr_dn6 = assign47800_e61262_d_n6;
        locals.var_q_edge_sqerr_dn7 = assign47800_e61262_d_n7;
        locals.var_q_edge_sqerr_dn8 = assign47800_e61262_d_n8;
        locals.var_q_edge_sqerr_dn9 = assign47800_e61262_d_n9;

        let (assign47810_e61277, assign47810_e61277_d_n4, assign47810_e61277_d_n6, assign47810_e61277_d_n7, assign47810_e61277_d_n8, assign47810_e61277_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47810_e61268: f64 = (locals.var_q_edge_sqerr).sqrt();
        let assign47810_e61270: f64 = (assign47810_e61268 - locals.var_q_edge_n);
        let assign47810_e61272: f64 = (assign47810_e61270 / locals.var_q_edge_d0p);
        let assign47810_e61274: f64 = (assign47810_e61272 - 1.0);
        let assign47810_e61275: f64 = (locals.var_q_edge_n * assign47810_e61274);
        (assign47810_e61275, ((locals.var_q_edge_n_dn4 * assign47810_e61274) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn4 / (2.0 * assign47810_e61268)) - locals.var_q_edge_n_dn4) * locals.var_q_edge_d0p) - (assign47810_e61270 * locals.var_q_edge_d0p_dn4)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn6 * assign47810_e61274) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn6 / (2.0 * assign47810_e61268)) - locals.var_q_edge_n_dn6) * locals.var_q_edge_d0p) - (assign47810_e61270 * locals.var_q_edge_d0p_dn6)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn7 * assign47810_e61274) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn7 / (2.0 * assign47810_e61268)) - locals.var_q_edge_n_dn7) * locals.var_q_edge_d0p) - (assign47810_e61270 * locals.var_q_edge_d0p_dn7)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn8 * assign47810_e61274) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn8 / (2.0 * assign47810_e61268)) - locals.var_q_edge_n_dn8) * locals.var_q_edge_d0p) - (assign47810_e61270 * locals.var_q_edge_d0p_dn8)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn9 * assign47810_e61274) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn9 / (2.0 * assign47810_e61268)) - locals.var_q_edge_n_dn9) * locals.var_q_edge_d0p) - (assign47810_e61270 * locals.var_q_edge_d0p_dn9)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))),)
    } else {
        (locals.var_q_edge_errq, locals.var_q_edge_errq_dn4, locals.var_q_edge_errq_dn6, locals.var_q_edge_errq_dn7, locals.var_q_edge_errq_dn8, locals.var_q_edge_errq_dn9,)
    }
};
        locals.var_q_edge_errq = assign47810_e61277;
        locals.var_q_edge_errq_dn4 = assign47810_e61277_d_n4;
        locals.var_q_edge_errq_dn6 = assign47810_e61277_d_n6;
        locals.var_q_edge_errq_dn7 = assign47810_e61277_d_n7;
        locals.var_q_edge_errq_dn8 = assign47810_e61277_d_n8;
        locals.var_q_edge_errq_dn9 = assign47810_e61277_d_n9;

        let (assign47820_e61285, assign47820_e61285_d_n4, assign47820_e61285_d_n6, assign47820_e61285_d_n7, assign47820_e61285_d_n8, assign47820_e61285_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
        let assign47820_e61283: f64 = (locals.var_q_edge_qi0 - locals.var_q_edge_errq);
        (assign47820_e61283, (locals.var_q_edge_qi0_dn4 - locals.var_q_edge_errq_dn4), (locals.var_q_edge_qi0_dn6 - locals.var_q_edge_errq_dn6), (locals.var_q_edge_qi0_dn7 - locals.var_q_edge_errq_dn7), (locals.var_q_edge_qi0_dn8 - locals.var_q_edge_errq_dn8), (locals.var_q_edge_qi0_dn9 - locals.var_q_edge_errq_dn9),)
    } else {
        (locals.var_qseffedge, locals.var_qseffedge_dn4, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8, locals.var_qseffedge_dn9,)
    }
};
        locals.var_qseffedge = assign47820_e61285;
        locals.var_qseffedge_dn4 = assign47820_e61285_d_n4;
        locals.var_qseffedge_dn6 = assign47820_e61285_d_n6;
        locals.var_qseffedge_dn7 = assign47820_e61285_d_n7;
        locals.var_qseffedge_dn8 = assign47820_e61285_d_n8;
        locals.var_qseffedge_dn9 = assign47820_e61285_d_n9;

        let assign47830_e61289: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47830_e61290: f64 = (locals.var_q_edge_n_inv * assign47830_e61289);
        let assign47830_e61292: f64 = (-230.25850929940458);
        let assign47830_e61293: f64 = if assign47830_e61290 > assign47830_e61292 { 1.0 } else { 0.0 };
        locals.var_guard1269 = assign47830_e61293;

        let (assign47840_e61307, assign47840_e61307_d_n4, assign47840_e61307_d_n6, assign47840_e61307_d_n7, assign47840_e61307_d_n8, assign47840_e61307_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1267 == 0.0)) && (locals.var_guard1269 != 0.0)) {
        let assign47840_e61303: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47840_e61304: f64 = (locals.var_q_edge_n_inv * assign47840_e61303);
        let assign47840_e61305: f64 = (assign47840_e61304).exp();
        (assign47840_e61305, (assign47840_e61305 * ((locals.var_q_edge_n_inv_dn4 * assign47840_e61303) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))), (assign47840_e61305 * ((locals.var_q_edge_n_inv_dn6 * assign47840_e61303) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))), (assign47840_e61305 * ((locals.var_q_edge_n_inv_dn7 * assign47840_e61303) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))), (assign47840_e61305 * ((locals.var_q_edge_n_inv_dn8 * assign47840_e61303) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))), (assign47840_e61305 * ((locals.var_q_edge_n_inv_dn9 * assign47840_e61303) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))),)
    } else {
        (locals.var_qseffedge, locals.var_qseffedge_dn4, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8, locals.var_qseffedge_dn9,)
    }
};
        locals.var_qseffedge = assign47840_e61307;
        locals.var_qseffedge_dn4 = assign47840_e61307_d_n4;
        locals.var_qseffedge_dn6 = assign47840_e61307_d_n6;
        locals.var_qseffedge_dn7 = assign47840_e61307_d_n7;
        locals.var_qseffedge_dn8 = assign47840_e61307_d_n8;
        locals.var_qseffedge_dn9 = assign47840_e61307_d_n9;

        let (assign47850_e61354, assign47850_e61354_d_n4, assign47850_e61354_d_n6, assign47850_e61354_d_n7, assign47850_e61354_d_n8, assign47850_e61354_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1267 == 0.0)) && (locals.var_guard1269 == 0.0)) {
        let assign47850_e61318: f64 = (-230.25850929940458);
        let assign47850_e61322: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47850_e61323: f64 = (locals.var_q_edge_n_inv * assign47850_e61322);
        let assign47850_e61324: f64 = (assign47850_e61318 - assign47850_e61323);
        let assign47850_e61328: f64 = (-230.25850929940458);
        let assign47850_e61332: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47850_e61333: f64 = (locals.var_q_edge_n_inv * assign47850_e61332);
        let assign47850_e61334: f64 = (assign47850_e61328 - assign47850_e61333);
        let assign47850_e61337: f64 = (-230.25850929940458);
        let assign47850_e61341: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47850_e61342: f64 = (locals.var_q_edge_n_inv * assign47850_e61341);
        let assign47850_e61343: f64 = (assign47850_e61337 - assign47850_e61342);
        let assign47850_e61345: f64 = (assign47850_e61343 * 0.3333333333333333);
        let assign47850_e61346: f64 = (1.0 + assign47850_e61345);
        let assign47850_e61347: f64 = (assign47850_e61334 * assign47850_e61346);
        let assign47850_e61348: f64 = (0.5 * assign47850_e61347);
        let assign47850_e61349: f64 = (1.0 + assign47850_e61348);
        let assign47850_e61350: f64 = (assign47850_e61324 * assign47850_e61349);
        let assign47850_e61351: f64 = (1.0 + assign47850_e61350);
        let assign47850_e61352: f64 = (1e-100 / assign47850_e61351);
        (assign47850_e61352, (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn4 * assign47850_e61322) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * assign47850_e61349) + (assign47850_e61324 * (0.5 * (((-((locals.var_q_edge_n_inv_dn4 * assign47850_e61332) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * assign47850_e61346) + (assign47850_e61334 * ((-((locals.var_q_edge_n_inv_dn4 * assign47850_e61341) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * 0.3333333333333333))))))) / (assign47850_e61351 * assign47850_e61351))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn6 * assign47850_e61322) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47850_e61349) + (assign47850_e61324 * (0.5 * (((-((locals.var_q_edge_n_inv_dn6 * assign47850_e61332) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47850_e61346) + (assign47850_e61334 * ((-((locals.var_q_edge_n_inv_dn6 * assign47850_e61341) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * 0.3333333333333333))))))) / (assign47850_e61351 * assign47850_e61351))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn7 * assign47850_e61322) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47850_e61349) + (assign47850_e61324 * (0.5 * (((-((locals.var_q_edge_n_inv_dn7 * assign47850_e61332) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47850_e61346) + (assign47850_e61334 * ((-((locals.var_q_edge_n_inv_dn7 * assign47850_e61341) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * 0.3333333333333333))))))) / (assign47850_e61351 * assign47850_e61351))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn8 * assign47850_e61322) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47850_e61349) + (assign47850_e61324 * (0.5 * (((-((locals.var_q_edge_n_inv_dn8 * assign47850_e61332) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47850_e61346) + (assign47850_e61334 * ((-((locals.var_q_edge_n_inv_dn8 * assign47850_e61341) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * 0.3333333333333333))))))) / (assign47850_e61351 * assign47850_e61351))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn9 * assign47850_e61322) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * assign47850_e61349) + (assign47850_e61324 * (0.5 * (((-((locals.var_q_edge_n_inv_dn9 * assign47850_e61332) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * assign47850_e61346) + (assign47850_e61334 * ((-((locals.var_q_edge_n_inv_dn9 * assign47850_e61341) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * 0.3333333333333333))))))) / (assign47850_e61351 * assign47850_e61351))),)
    } else {
        (locals.var_qseffedge, locals.var_qseffedge_dn4, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8, locals.var_qseffedge_dn9,)
    }
};
        locals.var_qseffedge = assign47850_e61354;
        locals.var_qseffedge_dn4 = assign47850_e61354_d_n4;
        locals.var_qseffedge_dn6 = assign47850_e61354_d_n6;
        locals.var_qseffedge_dn7 = assign47850_e61354_d_n7;
        locals.var_qseffedge_dn8 = assign47850_e61354_d_n8;
        locals.var_qseffedge_dn9 = assign47850_e61354_d_n9;

        let (assign47860_e61362, assign47860_e61362_d_n4, assign47860_e61362_d_n6, assign47860_e61362_d_n7, assign47860_e61362_d_n8, assign47860_e61362_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign47860_e61359: f64 = (locals.var_vdse_dc + locals.var_vsbstaredge);
        let assign47860_e61360: f64 = (locals.var_inv_phit1edge * assign47860_e61359);
        (assign47860_e61360, ((locals.var_inv_phit1edge_dn4 * assign47860_e61359) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn4 + locals.var_vsbstaredge_dn4))), ((locals.var_inv_phit1edge_dn6 * assign47860_e61359) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn6 + locals.var_vsbstaredge_dn6))), ((locals.var_inv_phit1edge_dn7 * assign47860_e61359) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn7 + locals.var_vsbstaredge_dn7))), ((locals.var_inv_phit1edge_dn8 * assign47860_e61359) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn8 + locals.var_vsbstaredge_dn8))), ((locals.var_inv_phit1edge_dn9 * assign47860_e61359) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn9 + locals.var_vsbstaredge_dn9))),)
    } else {
        (locals.var_xnedge_d, locals.var_xnedge_d_dn4, locals.var_xnedge_d_dn6, locals.var_xnedge_d_dn7, locals.var_xnedge_d_dn8, locals.var_xnedge_d_dn9,)
    }
};
        locals.var_xnedge_d = assign47860_e61362;
        locals.var_xnedge_d_dn4 = assign47860_e61362_d_n4;
        locals.var_xnedge_d_dn6 = assign47860_e61362_d_n6;
        locals.var_xnedge_d_dn7 = assign47860_e61362_d_n7;
        locals.var_xnedge_d_dn8 = assign47860_e61362_d_n8;
        locals.var_xnedge_d_dn9 = assign47860_e61362_d_n9;

        let assign47870_e61369: f64 = if ((locals.var_qseffedge < 0.001) && (locals.var_vdse_dc < 1e-6)) { 1.0 } else { 0.0 };
        locals.var_guard1270 = assign47870_e61369;

        let assign47880_e61371: f64 = (-locals.var_xnedge_d);
        let assign47880_e61373: f64 = (assign47880_e61371 + locals.var_xnedge_s);
        let assign47880_e61375: f64 = (-230.25850929940458);
        let assign47880_e61376: f64 = if assign47880_e61373 > assign47880_e61375 { 1.0 } else { 0.0 };
        locals.var_guard1271 = assign47880_e61376;

        let (assign47890_e61388, assign47890_e61388_d_n4, assign47890_e61388_d_n6, assign47890_e61388_d_n7, assign47890_e61388_d_n8, assign47890_e61388_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 != 0.0)) && (locals.var_guard1271 != 0.0)) {
        let assign47890_e61383: f64 = (-locals.var_xnedge_d);
        let assign47890_e61385: f64 = (assign47890_e61383 + locals.var_xnedge_s);
        let assign47890_e61386: f64 = (assign47890_e61385).exp();
        (assign47890_e61386, (assign47890_e61386 * ((-locals.var_xnedge_d_dn4) + locals.var_xnedge_s_dn4)), (assign47890_e61386 * ((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)), (assign47890_e61386 * ((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)), (assign47890_e61386 * ((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)), (assign47890_e61386 * ((-locals.var_xnedge_d_dn9) + locals.var_xnedge_s_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign47890_e61388;
        locals.var_temp__blk949_dn4 = assign47890_e61388_d_n4;
        locals.var_temp__blk949_dn6 = assign47890_e61388_d_n6;
        locals.var_temp__blk949_dn7 = assign47890_e61388_d_n7;
        locals.var_temp__blk949_dn8 = assign47890_e61388_d_n8;
        locals.var_temp__blk949_dn9 = assign47890_e61388_d_n9;

        let (assign47900_e61431, assign47900_e61431_d_n4, assign47900_e61431_d_n6, assign47900_e61431_d_n7, assign47900_e61431_d_n8, assign47900_e61431_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 != 0.0)) && (locals.var_guard1271 == 0.0)) {
        let assign47900_e61398: f64 = (-230.25850929940458);
        let assign47900_e61400: f64 = (-locals.var_xnedge_d);
        let assign47900_e61402: f64 = (assign47900_e61400 + locals.var_xnedge_s);
        let assign47900_e61403: f64 = (assign47900_e61398 - assign47900_e61402);
        let assign47900_e61407: f64 = (-230.25850929940458);
        let assign47900_e61409: f64 = (-locals.var_xnedge_d);
        let assign47900_e61411: f64 = (assign47900_e61409 + locals.var_xnedge_s);
        let assign47900_e61412: f64 = (assign47900_e61407 - assign47900_e61411);
        let assign47900_e61415: f64 = (-230.25850929940458);
        let assign47900_e61417: f64 = (-locals.var_xnedge_d);
        let assign47900_e61419: f64 = (assign47900_e61417 + locals.var_xnedge_s);
        let assign47900_e61420: f64 = (assign47900_e61415 - assign47900_e61419);
        let assign47900_e61422: f64 = (assign47900_e61420 * 0.3333333333333333);
        let assign47900_e61423: f64 = (1.0 + assign47900_e61422);
        let assign47900_e61424: f64 = (assign47900_e61412 * assign47900_e61423);
        let assign47900_e61425: f64 = (0.5 * assign47900_e61424);
        let assign47900_e61426: f64 = (1.0 + assign47900_e61425);
        let assign47900_e61427: f64 = (assign47900_e61403 * assign47900_e61426);
        let assign47900_e61428: f64 = (1.0 + assign47900_e61427);
        let assign47900_e61429: f64 = (1e-100 / assign47900_e61428);
        (assign47900_e61429, (-((1e-100 * (((-((-locals.var_xnedge_d_dn4) + locals.var_xnedge_s_dn4)) * assign47900_e61426) + (assign47900_e61403 * (0.5 * (((-((-locals.var_xnedge_d_dn4) + locals.var_xnedge_s_dn4)) * assign47900_e61423) + (assign47900_e61412 * ((-((-locals.var_xnedge_d_dn4) + locals.var_xnedge_s_dn4)) * 0.3333333333333333))))))) / (assign47900_e61428 * assign47900_e61428))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * assign47900_e61426) + (assign47900_e61403 * (0.5 * (((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * assign47900_e61423) + (assign47900_e61412 * ((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * 0.3333333333333333))))))) / (assign47900_e61428 * assign47900_e61428))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * assign47900_e61426) + (assign47900_e61403 * (0.5 * (((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * assign47900_e61423) + (assign47900_e61412 * ((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * 0.3333333333333333))))))) / (assign47900_e61428 * assign47900_e61428))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * assign47900_e61426) + (assign47900_e61403 * (0.5 * (((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * assign47900_e61423) + (assign47900_e61412 * ((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * 0.3333333333333333))))))) / (assign47900_e61428 * assign47900_e61428))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn9) + locals.var_xnedge_s_dn9)) * assign47900_e61426) + (assign47900_e61403 * (0.5 * (((-((-locals.var_xnedge_d_dn9) + locals.var_xnedge_s_dn9)) * assign47900_e61423) + (assign47900_e61412 * ((-((-locals.var_xnedge_d_dn9) + locals.var_xnedge_s_dn9)) * 0.3333333333333333))))))) / (assign47900_e61428 * assign47900_e61428))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign47900_e61431;
        locals.var_temp__blk949_dn4 = assign47900_e61431_d_n4;
        locals.var_temp__blk949_dn6 = assign47900_e61431_d_n6;
        locals.var_temp__blk949_dn7 = assign47900_e61431_d_n7;
        locals.var_temp__blk949_dn8 = assign47900_e61431_d_n8;
        locals.var_temp__blk949_dn9 = assign47900_e61431_d_n9;

        let (assign47910_e61441, assign47910_e61441_d_n4, assign47910_e61441_d_n6, assign47910_e61441_d_n7, assign47910_e61441_d_n8, assign47910_e61441_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 != 0.0)) {
        let assign47910_e61438: f64 = (locals.var_temp__blk949 - 1.0);
        let assign47910_e61439: f64 = (locals.var_qseffedge * assign47910_e61438);
        (assign47910_e61439, ((locals.var_qseffedge_dn4 * assign47910_e61438) + (locals.var_qseffedge * locals.var_temp__blk949_dn4)), ((locals.var_qseffedge_dn6 * assign47910_e61438) + (locals.var_qseffedge * locals.var_temp__blk949_dn6)), ((locals.var_qseffedge_dn7 * assign47910_e61438) + (locals.var_qseffedge * locals.var_temp__blk949_dn7)), ((locals.var_qseffedge_dn8 * assign47910_e61438) + (locals.var_qseffedge * locals.var_temp__blk949_dn8)), ((locals.var_qseffedge_dn9 * assign47910_e61438) + (locals.var_qseffedge * locals.var_temp__blk949_dn9)),)
    } else {
        (locals.var_qdseffedge, locals.var_qdseffedge_dn4, locals.var_qdseffedge_dn6, locals.var_qdseffedge_dn7, locals.var_qdseffedge_dn8, locals.var_qdseffedge_dn9,)
    }
};
        locals.var_qdseffedge = assign47910_e61441;
        locals.var_qdseffedge_dn4 = assign47910_e61441_d_n4;
        locals.var_qdseffedge_dn6 = assign47910_e61441_d_n6;
        locals.var_qdseffedge_dn7 = assign47910_e61441_d_n7;
        locals.var_qdseffedge_dn8 = assign47910_e61441_d_n8;
        locals.var_qdseffedge_dn9 = assign47910_e61441_d_n9;

        let (assign47920_e61449, assign47920_e61449_d_n4, assign47920_e61449_d_n6, assign47920_e61449_d_n7, assign47920_e61449_d_n8, assign47920_e61449_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 != 0.0)) {
        let assign47920_e61447: f64 = (locals.var_qdseffedge + locals.var_qseffedge);
        (assign47920_e61447, (locals.var_qdseffedge_dn4 + locals.var_qseffedge_dn4), (locals.var_qdseffedge_dn6 + locals.var_qseffedge_dn6), (locals.var_qdseffedge_dn7 + locals.var_qseffedge_dn7), (locals.var_qdseffedge_dn8 + locals.var_qseffedge_dn8), (locals.var_qdseffedge_dn9 + locals.var_qseffedge_dn9),)
    } else {
        (locals.var_qdeffedge, locals.var_qdeffedge_dn4, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8, locals.var_qdeffedge_dn9,)
    }
};
        locals.var_qdeffedge = assign47920_e61449;
        locals.var_qdeffedge_dn4 = assign47920_e61449_d_n4;
        locals.var_qdeffedge_dn6 = assign47920_e61449_d_n6;
        locals.var_qdeffedge_dn7 = assign47920_e61449_d_n7;
        locals.var_qdeffedge_dn8 = assign47920_e61449_d_n8;
        locals.var_qdeffedge_dn9 = assign47920_e61449_d_n9;

        let (assign47930_e61458, assign47930_e61458_d_n4, assign47930_e61458_d_n6, assign47930_e61458_d_n7, assign47930_e61458_d_n8, assign47930_e61458_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
        let assign47930_e61456: f64 = (locals.var_xbedge + locals.var_xnedge_d);
        (assign47930_e61456, (locals.var_xbedge_dn4 + locals.var_xnedge_d_dn4), (locals.var_xbedge_dn6 + locals.var_xnedge_d_dn6), (locals.var_xbedge_dn7 + locals.var_xnedge_d_dn7), (locals.var_xbedge_dn8 + locals.var_xnedge_d_dn8), (locals.var_xbedge_dn9 + locals.var_xnedge_d_dn9),)
    } else {
        (locals.var_q_edge_xsth, locals.var_q_edge_xsth_dn4, locals.var_q_edge_xsth_dn6, locals.var_q_edge_xsth_dn7, locals.var_q_edge_xsth_dn8, locals.var_q_edge_xsth_dn9,)
    }
};
        locals.var_q_edge_xsth = assign47930_e61458;
        locals.var_q_edge_xsth_dn4 = assign47930_e61458_d_n4;
        locals.var_q_edge_xsth_dn6 = assign47930_e61458_d_n6;
        locals.var_q_edge_xsth_dn7 = assign47930_e61458_d_n7;
        locals.var_q_edge_xsth_dn8 = assign47930_e61458_d_n8;
        locals.var_q_edge_xsth_dn9 = assign47930_e61458_d_n9;

        let (assign47940_e61470, assign47940_e61470_d_n4, assign47940_e61470_d_n6, assign47940_e61470_d_n7, assign47940_e61470_d_n8, assign47940_e61470_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
        let assign47940_e61466: f64 = (locals.var_q_edge_xsth).sqrt();
        let assign47940_e61467: f64 = (locals.var_gfedge * assign47940_e61466);
        let assign47940_e61468: f64 = (locals.var_q_edge_xsth + assign47940_e61467);
        (assign47940_e61468, (locals.var_q_edge_xsth_dn4 + ((locals.var_gfedge_dn4 * assign47940_e61466) + (locals.var_gfedge * (locals.var_q_edge_xsth_dn4 / (2.0 * assign47940_e61466))))), (locals.var_q_edge_xsth_dn6 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47940_e61466)))), (locals.var_q_edge_xsth_dn7 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47940_e61466)))), (locals.var_q_edge_xsth_dn8 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47940_e61466)))), (locals.var_q_edge_xsth_dn9 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn9 / (2.0 * assign47940_e61466)))),)
    } else {
        (locals.var_q_edge_xth0, locals.var_q_edge_xth0_dn4, locals.var_q_edge_xth0_dn6, locals.var_q_edge_xth0_dn7, locals.var_q_edge_xth0_dn8, locals.var_q_edge_xth0_dn9,)
    }
};
        locals.var_q_edge_xth0 = assign47940_e61470;
        locals.var_q_edge_xth0_dn4 = assign47940_e61470_d_n4;
        locals.var_q_edge_xth0_dn6 = assign47940_e61470_d_n6;
        locals.var_q_edge_xth0_dn7 = assign47940_e61470_d_n7;
        locals.var_q_edge_xth0_dn8 = assign47940_e61470_d_n8;
        locals.var_q_edge_xth0_dn9 = assign47940_e61470_d_n9;

    }

    pub(super) fn stamp_transient_block_34(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47950_e61479, assign47950_e61479_d_n4, assign47950_e61479_d_n6, assign47950_e61479_d_n7, assign47950_e61479_d_n8, assign47950_e61479_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
        let assign47950_e61477: f64 = (locals.var_q_edge_xth0 + locals.var_dxthedge);
        (assign47950_e61477, (locals.var_q_edge_xth0_dn4 + locals.var_dxthedge_dn4), (locals.var_q_edge_xth0_dn6 + locals.var_dxthedge_dn6), (locals.var_q_edge_xth0_dn7 + locals.var_dxthedge_dn7), (locals.var_q_edge_xth0_dn8 + locals.var_dxthedge_dn8), (locals.var_q_edge_xth0_dn9 + locals.var_dxthedge_dn9),)
    } else {
        (locals.var_q_edge_xth, locals.var_q_edge_xth_dn4, locals.var_q_edge_xth_dn6, locals.var_q_edge_xth_dn7, locals.var_q_edge_xth_dn8, locals.var_q_edge_xth_dn9,)
    }
};
        locals.var_q_edge_xth = assign47950_e61479;
        locals.var_q_edge_xth_dn4 = assign47950_e61479_d_n4;
        locals.var_q_edge_xth_dn6 = assign47950_e61479_d_n6;
        locals.var_q_edge_xth_dn7 = assign47950_e61479_d_n7;
        locals.var_q_edge_xth_dn8 = assign47950_e61479_d_n8;
        locals.var_q_edge_xth_dn9 = assign47950_e61479_d_n9;

        let (assign47960_e61493, assign47960_e61493_d_n4, assign47960_e61493_d_n6, assign47960_e61493_d_n7, assign47960_e61493_d_n8, assign47960_e61493_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
        let assign47960_e61488: f64 = (locals.var_q_edge_xsth).sqrt();
        let assign47960_e61489: f64 = (2.0 * assign47960_e61488);
        let assign47960_e61490: f64 = (locals.var_gfedge / assign47960_e61489);
        let assign47960_e61491: f64 = (1.0 + assign47960_e61490);
        (assign47960_e61491, (((locals.var_gfedge_dn4 * assign47960_e61489) - (locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn4 / (2.0 * assign47960_e61488))))) / (assign47960_e61489 * assign47960_e61489)), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47960_e61488)))) / (assign47960_e61489 * assign47960_e61489))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47960_e61488)))) / (assign47960_e61489 * assign47960_e61489))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47960_e61488)))) / (assign47960_e61489 * assign47960_e61489))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn9 / (2.0 * assign47960_e61488)))) / (assign47960_e61489 * assign47960_e61489))),)
    } else {
        (locals.var_q_edge_n, locals.var_q_edge_n_dn4, locals.var_q_edge_n_dn6, locals.var_q_edge_n_dn7, locals.var_q_edge_n_dn8, locals.var_q_edge_n_dn9,)
    }
};
        locals.var_q_edge_n = assign47960_e61493;
        locals.var_q_edge_n_dn4 = assign47960_e61493_d_n4;
        locals.var_q_edge_n_dn6 = assign47960_e61493_d_n6;
        locals.var_q_edge_n_dn7 = assign47960_e61493_d_n7;
        locals.var_q_edge_n_dn8 = assign47960_e61493_d_n8;
        locals.var_q_edge_n_dn9 = assign47960_e61493_d_n9;

        let (assign47970_e61502, assign47970_e61502_d_n4, assign47970_e61502_d_n6, assign47970_e61502_d_n7, assign47970_e61502_d_n8, assign47970_e61502_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
        let assign47970_e61500: f64 = (1.0 / locals.var_q_edge_n);
        (assign47970_e61500, (-(locals.var_q_edge_n_dn4 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn6 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn7 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn8 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn9 / (locals.var_q_edge_n * locals.var_q_edge_n))),)
    } else {
        (locals.var_q_edge_n_inv, locals.var_q_edge_n_inv_dn4, locals.var_q_edge_n_inv_dn6, locals.var_q_edge_n_inv_dn7, locals.var_q_edge_n_inv_dn8, locals.var_q_edge_n_inv_dn9,)
    }
};
        locals.var_q_edge_n_inv = assign47970_e61502;
        locals.var_q_edge_n_inv_dn4 = assign47970_e61502_d_n4;
        locals.var_q_edge_n_inv_dn6 = assign47970_e61502_d_n6;
        locals.var_q_edge_n_inv_dn7 = assign47970_e61502_d_n7;
        locals.var_q_edge_n_inv_dn8 = assign47970_e61502_d_n8;
        locals.var_q_edge_n_inv_dn9 = assign47970_e61502_d_n9;

        let (assign47980_e61511, assign47980_e61511_d_n4, assign47980_e61511_d_n6, assign47980_e61511_d_n7, assign47980_e61511_d_n8, assign47980_e61511_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
        let assign47980_e61509: f64 = (locals.var_xgedge - locals.var_q_edge_xth);
        (assign47980_e61509, (locals.var_xgedge_dn4 - locals.var_q_edge_xth_dn4), (locals.var_xgedge_dn6 - locals.var_q_edge_xth_dn6), (locals.var_xgedge_dn7 - locals.var_q_edge_xth_dn7), (locals.var_xgedge_dn8 - locals.var_q_edge_xth_dn8), (locals.var_xgedge_dn9 - locals.var_q_edge_xth_dn9),)
    } else {
        (locals.var_q_edge_xgt, locals.var_q_edge_xgt_dn4, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8, locals.var_q_edge_xgt_dn9,)
    }
};
        locals.var_q_edge_xgt = assign47980_e61511;
        locals.var_q_edge_xgt_dn4 = assign47980_e61511_d_n4;
        locals.var_q_edge_xgt_dn6 = assign47980_e61511_d_n6;
        locals.var_q_edge_xgt_dn7 = assign47980_e61511_d_n7;
        locals.var_q_edge_xgt_dn8 = assign47980_e61511_d_n8;
        locals.var_q_edge_xgt_dn9 = assign47980_e61511_d_n9;

        let assign47990_e61514: f64 = (-12.0);
        let assign47990_e61515: f64 = if locals.var_q_edge_xgt > assign47990_e61514 { 1.0 } else { 0.0 };
        locals.var_guard1272 = assign47990_e61515;

        let (assign48000_e61528, assign48000_e61528_d_n4, assign48000_e61528_d_n6, assign48000_e61528_d_n7, assign48000_e61528_d_n8, assign48000_e61528_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48000_e61524: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign48000_e61526: f64 = (assign48000_e61524 - 1.0);
        (assign48000_e61526, (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4), locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8, locals.var_q_edge_xgt_dn9,)
    } else {
        (locals.var_q_edge_xgt0, locals.var_q_edge_xgt0_dn4, locals.var_q_edge_xgt0_dn6, locals.var_q_edge_xgt0_dn7, locals.var_q_edge_xgt0_dn8, locals.var_q_edge_xgt0_dn9,)
    }
};
        locals.var_q_edge_xgt0 = assign48000_e61528;
        locals.var_q_edge_xgt0_dn4 = assign48000_e61528_d_n4;
        locals.var_q_edge_xgt0_dn6 = assign48000_e61528_d_n6;
        locals.var_q_edge_xgt0_dn7 = assign48000_e61528_d_n7;
        locals.var_q_edge_xgt0_dn8 = assign48000_e61528_d_n8;
        locals.var_q_edge_xgt0_dn9 = assign48000_e61528_d_n9;

        let (assign48010_e61546, assign48010_e61546_d_n4, assign48010_e61546_d_n6, assign48010_e61546_d_n7, assign48010_e61546_d_n8, assign48010_e61546_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48010_e61539: f64 = (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0);
        let assign48010_e61541: f64 = (assign48010_e61539 + 10.0);
        let assign48010_e61542: f64 = (assign48010_e61541).sqrt();
        let assign48010_e61543: f64 = (locals.var_q_edge_xgt0 + assign48010_e61542);
        let assign48010_e61544: f64 = (0.5 * assign48010_e61543);
        (assign48010_e61544, (0.5 * (locals.var_q_edge_xgt0_dn4 + (((locals.var_q_edge_xgt0_dn4 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn4)) / (2.0 * assign48010_e61542)))), (0.5 * (locals.var_q_edge_xgt0_dn6 + (((locals.var_q_edge_xgt0_dn6 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn6)) / (2.0 * assign48010_e61542)))), (0.5 * (locals.var_q_edge_xgt0_dn7 + (((locals.var_q_edge_xgt0_dn7 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn7)) / (2.0 * assign48010_e61542)))), (0.5 * (locals.var_q_edge_xgt0_dn8 + (((locals.var_q_edge_xgt0_dn8 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn8)) / (2.0 * assign48010_e61542)))), (0.5 * (locals.var_q_edge_xgt0_dn9 + (((locals.var_q_edge_xgt0_dn9 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn9)) / (2.0 * assign48010_e61542)))),)
    } else {
        (locals.var_q_edge_xgt0e, locals.var_q_edge_xgt0e_dn4, locals.var_q_edge_xgt0e_dn6, locals.var_q_edge_xgt0e_dn7, locals.var_q_edge_xgt0e_dn8, locals.var_q_edge_xgt0e_dn9,)
    }
};
        locals.var_q_edge_xgt0e = assign48010_e61546;
        locals.var_q_edge_xgt0e_dn4 = assign48010_e61546_d_n4;
        locals.var_q_edge_xgt0e_dn6 = assign48010_e61546_d_n6;
        locals.var_q_edge_xgt0e_dn7 = assign48010_e61546_d_n7;
        locals.var_q_edge_xgt0e_dn8 = assign48010_e61546_d_n8;
        locals.var_q_edge_xgt0e_dn9 = assign48010_e61546_d_n9;

        let (assign48020_e61562, assign48020_e61562_d_n4, assign48020_e61562_d_n6, assign48020_e61562_d_n7, assign48020_e61562_d_n8, assign48020_e61562_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48020_e61556: f64 = (locals.var_q_edge_xgt0e).ln();
        let assign48020_e61557: f64 = (locals.var_q_edge_n * assign48020_e61556);
        let assign48020_e61558: f64 = (locals.var_q_edge_xgt - assign48020_e61557);
        let assign48020_e61560: f64 = (assign48020_e61558 + locals.var_lngfedge2);
        (assign48020_e61560, ((locals.var_q_edge_xgt_dn4 - ((locals.var_q_edge_n_dn4 * assign48020_e61556) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn4 / locals.var_q_edge_xgt0e)))) + locals.var_lngfedge2_dn4), (locals.var_q_edge_xgt_dn6 - ((locals.var_q_edge_n_dn6 * assign48020_e61556) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn6 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn7 - ((locals.var_q_edge_n_dn7 * assign48020_e61556) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn7 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn8 - ((locals.var_q_edge_n_dn8 * assign48020_e61556) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn8 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn9 - ((locals.var_q_edge_n_dn9 * assign48020_e61556) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn9 / locals.var_q_edge_xgt0e)))),)
    } else {
        (locals.var_q_edge_qi0si, locals.var_q_edge_qi0si_dn4, locals.var_q_edge_qi0si_dn6, locals.var_q_edge_qi0si_dn7, locals.var_q_edge_qi0si_dn8, locals.var_q_edge_qi0si_dn9,)
    }
};
        locals.var_q_edge_qi0si = assign48020_e61562;
        locals.var_q_edge_qi0si_dn4 = assign48020_e61562_d_n4;
        locals.var_q_edge_qi0si_dn6 = assign48020_e61562_d_n6;
        locals.var_q_edge_qi0si_dn7 = assign48020_e61562_d_n7;
        locals.var_q_edge_qi0si_dn8 = assign48020_e61562_d_n8;
        locals.var_q_edge_qi0si_dn9 = assign48020_e61562_d_n9;

        let (assign48030_e61580, assign48030_e61580_d_n4, assign48030_e61580_d_n6, assign48030_e61580_d_n7, assign48030_e61580_d_n8, assign48030_e61580_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48030_e61573: f64 = (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si);
        let assign48030_e61575: f64 = (assign48030_e61573 + 2.0);
        let assign48030_e61576: f64 = (assign48030_e61575).sqrt();
        let assign48030_e61577: f64 = (locals.var_q_edge_qi0si + assign48030_e61576);
        let assign48030_e61578: f64 = (0.5 * assign48030_e61577);
        (assign48030_e61578, (0.5 * (locals.var_q_edge_qi0si_dn4 + (((locals.var_q_edge_qi0si_dn4 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn4)) / (2.0 * assign48030_e61576)))), (0.5 * (locals.var_q_edge_qi0si_dn6 + (((locals.var_q_edge_qi0si_dn6 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn6)) / (2.0 * assign48030_e61576)))), (0.5 * (locals.var_q_edge_qi0si_dn7 + (((locals.var_q_edge_qi0si_dn7 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn7)) / (2.0 * assign48030_e61576)))), (0.5 * (locals.var_q_edge_qi0si_dn8 + (((locals.var_q_edge_qi0si_dn8 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn8)) / (2.0 * assign48030_e61576)))), (0.5 * (locals.var_q_edge_qi0si_dn9 + (((locals.var_q_edge_qi0si_dn9 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn9)) / (2.0 * assign48030_e61576)))),)
    } else {
        (locals.var_q_edge_qi0, locals.var_q_edge_qi0_dn4, locals.var_q_edge_qi0_dn6, locals.var_q_edge_qi0_dn7, locals.var_q_edge_qi0_dn8, locals.var_q_edge_qi0_dn9,)
    }
};
        locals.var_q_edge_qi0 = assign48030_e61580;
        locals.var_q_edge_qi0_dn4 = assign48030_e61580_d_n4;
        locals.var_q_edge_qi0_dn6 = assign48030_e61580_d_n6;
        locals.var_q_edge_qi0_dn7 = assign48030_e61580_d_n7;
        locals.var_q_edge_qi0_dn8 = assign48030_e61580_d_n8;
        locals.var_q_edge_qi0_dn9 = assign48030_e61580_d_n9;

        let assign48040_e61583: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign48040_e61585: f64 = if assign48040_e61583 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1273 = assign48040_e61585;

        let (assign48050_e61599, assign48050_e61599_d_n4, assign48050_e61599_d_n6, assign48050_e61599_d_n7, assign48050_e61599_d_n8, assign48050_e61599_d_n9,) = {
    if ((((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) && (locals.var_guard1273 != 0.0)) {
        let assign48050_e61596: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign48050_e61597: f64 = (assign48050_e61596).exp();
        (assign48050_e61597, (assign48050_e61597 * (locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4)), (assign48050_e61597 * (locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6)), (assign48050_e61597 * (locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7)), (assign48050_e61597 * (locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8)), (assign48050_e61597 * (locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9)),)
    } else {
        (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn4, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8, locals.var_q_edge_exp_x_dn9,)
    }
};
        locals.var_q_edge_exp_x = assign48050_e61599;
        locals.var_q_edge_exp_x_dn4 = assign48050_e61599_d_n4;
        locals.var_q_edge_exp_x_dn6 = assign48050_e61599_d_n6;
        locals.var_q_edge_exp_x_dn7 = assign48050_e61599_d_n7;
        locals.var_q_edge_exp_x_dn8 = assign48050_e61599_d_n8;
        locals.var_q_edge_exp_x_dn9 = assign48050_e61599_d_n9;

        let (assign48060_e61639, assign48060_e61639_d_n4, assign48060_e61639_d_n6, assign48060_e61639_d_n7, assign48060_e61639_d_n8, assign48060_e61639_d_n9,) = {
    if ((((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) && (locals.var_guard1273 == 0.0)) {
        let assign48060_e61613: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign48060_e61615: f64 = (assign48060_e61613 - 230.25850929940458);
        let assign48060_e61620: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign48060_e61622: f64 = (assign48060_e61620 - 230.25850929940458);
        let assign48060_e61626: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign48060_e61628: f64 = (assign48060_e61626 - 230.25850929940458);
        let assign48060_e61630: f64 = (assign48060_e61628 * 0.3333333333333333);
        let assign48060_e61631: f64 = (1.0 + assign48060_e61630);
        let assign48060_e61632: f64 = (assign48060_e61622 * assign48060_e61631);
        let assign48060_e61633: f64 = (0.5 * assign48060_e61632);
        let assign48060_e61634: f64 = (1.0 + assign48060_e61633);
        let assign48060_e61635: f64 = (assign48060_e61615 * assign48060_e61634);
        let assign48060_e61636: f64 = (1.0 + assign48060_e61635);
        let assign48060_e61637: f64 = (1e100 * assign48060_e61636);
        (assign48060_e61637, (1e100 * (((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * assign48060_e61634) + (assign48060_e61615 * (0.5 * (((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * assign48060_e61631) + (assign48060_e61622 * ((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign48060_e61634) + (assign48060_e61615 * (0.5 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign48060_e61631) + (assign48060_e61622 * ((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign48060_e61634) + (assign48060_e61615 * (0.5 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign48060_e61631) + (assign48060_e61622 * ((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign48060_e61634) + (assign48060_e61615 * (0.5 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign48060_e61631) + (assign48060_e61622 * ((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * assign48060_e61634) + (assign48060_e61615 * (0.5 * (((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * assign48060_e61631) + (assign48060_e61622 * ((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * 0.3333333333333333))))))),)
    } else {
        (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn4, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8, locals.var_q_edge_exp_x_dn9,)
    }
};
        locals.var_q_edge_exp_x = assign48060_e61639;
        locals.var_q_edge_exp_x_dn4 = assign48060_e61639_d_n4;
        locals.var_q_edge_exp_x_dn6 = assign48060_e61639_d_n6;
        locals.var_q_edge_exp_x_dn7 = assign48060_e61639_d_n7;
        locals.var_q_edge_exp_x_dn8 = assign48060_e61639_d_n8;
        locals.var_q_edge_exp_x_dn9 = assign48060_e61639_d_n9;

        let (assign48070_e61650, assign48070_e61650_d_n4, assign48070_e61650_d_n6, assign48070_e61650_d_n7, assign48070_e61650_d_n8, assign48070_e61650_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48070_e61648: f64 = (locals.var_gfedge2 * locals.var_q_edge_exp_x);
        (assign48070_e61648, ((locals.var_gfedge2_dn4 * locals.var_q_edge_exp_x) + (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn4)), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn6), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn7), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn8), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn9),)
    } else {
        (locals.var_q_edge_d0, locals.var_q_edge_d0_dn4, locals.var_q_edge_d0_dn6, locals.var_q_edge_d0_dn7, locals.var_q_edge_d0_dn8, locals.var_q_edge_d0_dn9,)
    }
};
        locals.var_q_edge_d0 = assign48070_e61650;
        locals.var_q_edge_d0_dn4 = assign48070_e61650_d_n4;
        locals.var_q_edge_d0_dn6 = assign48070_e61650_d_n6;
        locals.var_q_edge_d0_dn7 = assign48070_e61650_d_n7;
        locals.var_q_edge_d0_dn8 = assign48070_e61650_d_n8;
        locals.var_q_edge_d0_dn9 = assign48070_e61650_d_n9;

        let (assign48080_e61661, assign48080_e61661_d_n4, assign48080_e61661_d_n6, assign48080_e61661_d_n7, assign48080_e61661_d_n8, assign48080_e61661_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48080_e61659: f64 = (locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv);
        (assign48080_e61659, if locals.var_q_edge_n_inv_dn4 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn4)) } } else { (assign48080_e61659 * ((locals.var_q_edge_n_inv_dn4 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn4 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn6 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn6)) } } else { (assign48080_e61659 * ((locals.var_q_edge_n_inv_dn6 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn6 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn7 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn7)) } } else { (assign48080_e61659 * ((locals.var_q_edge_n_inv_dn7 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn7 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn8 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn8)) } } else { (assign48080_e61659 * ((locals.var_q_edge_n_inv_dn8 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn8 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn9 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn9)) } } else { (assign48080_e61659 * ((locals.var_q_edge_n_inv_dn9 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn9 / locals.var_q_edge_d0)))) },)
    } else {
        (locals.var_q_edge_d0p, locals.var_q_edge_d0p_dn4, locals.var_q_edge_d0p_dn6, locals.var_q_edge_d0p_dn7, locals.var_q_edge_d0p_dn8, locals.var_q_edge_d0p_dn9,)
    }
};
        locals.var_q_edge_d0p = assign48080_e61661;
        locals.var_q_edge_d0p_dn4 = assign48080_e61661_d_n4;
        locals.var_q_edge_d0p_dn6 = assign48080_e61661_d_n6;
        locals.var_q_edge_d0p_dn7 = assign48080_e61661_d_n7;
        locals.var_q_edge_d0p_dn8 = assign48080_e61661_d_n8;
        locals.var_q_edge_d0p_dn9 = assign48080_e61661_d_n9;

        let (assign48090_e61682, assign48090_e61682_d_n4, assign48090_e61682_d_n6, assign48090_e61682_d_n7, assign48090_e61682_d_n8, assign48090_e61682_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48090_e61670: f64 = (locals.var_q_edge_n * locals.var_q_edge_n);
        let assign48090_e61674: f64 = (locals.var_q_edge_qi0 + locals.var_q_edge_n);
        let assign48090_e61675: f64 = (2.0 * assign48090_e61674);
        let assign48090_e61677: f64 = (assign48090_e61675 - locals.var_q_edge_d0p);
        let assign48090_e61679: f64 = (assign48090_e61677 * locals.var_q_edge_d0p);
        let assign48090_e61680: f64 = (assign48090_e61670 + assign48090_e61679);
        (assign48090_e61680, (((locals.var_q_edge_n_dn4 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn4)) + ((((2.0 * (locals.var_q_edge_qi0_dn4 + locals.var_q_edge_n_dn4)) - locals.var_q_edge_d0p_dn4) * locals.var_q_edge_d0p) + (assign48090_e61677 * locals.var_q_edge_d0p_dn4))), (((locals.var_q_edge_n_dn6 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn6)) + ((((2.0 * (locals.var_q_edge_qi0_dn6 + locals.var_q_edge_n_dn6)) - locals.var_q_edge_d0p_dn6) * locals.var_q_edge_d0p) + (assign48090_e61677 * locals.var_q_edge_d0p_dn6))), (((locals.var_q_edge_n_dn7 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn7)) + ((((2.0 * (locals.var_q_edge_qi0_dn7 + locals.var_q_edge_n_dn7)) - locals.var_q_edge_d0p_dn7) * locals.var_q_edge_d0p) + (assign48090_e61677 * locals.var_q_edge_d0p_dn7))), (((locals.var_q_edge_n_dn8 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn8)) + ((((2.0 * (locals.var_q_edge_qi0_dn8 + locals.var_q_edge_n_dn8)) - locals.var_q_edge_d0p_dn8) * locals.var_q_edge_d0p) + (assign48090_e61677 * locals.var_q_edge_d0p_dn8))), (((locals.var_q_edge_n_dn9 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn9)) + ((((2.0 * (locals.var_q_edge_qi0_dn9 + locals.var_q_edge_n_dn9)) - locals.var_q_edge_d0p_dn9) * locals.var_q_edge_d0p) + (assign48090_e61677 * locals.var_q_edge_d0p_dn9))),)
    } else {
        (locals.var_q_edge_sqerr, locals.var_q_edge_sqerr_dn4, locals.var_q_edge_sqerr_dn6, locals.var_q_edge_sqerr_dn7, locals.var_q_edge_sqerr_dn8, locals.var_q_edge_sqerr_dn9,)
    }
};
        locals.var_q_edge_sqerr = assign48090_e61682;
        locals.var_q_edge_sqerr_dn4 = assign48090_e61682_d_n4;
        locals.var_q_edge_sqerr_dn6 = assign48090_e61682_d_n6;
        locals.var_q_edge_sqerr_dn7 = assign48090_e61682_d_n7;
        locals.var_q_edge_sqerr_dn8 = assign48090_e61682_d_n8;
        locals.var_q_edge_sqerr_dn9 = assign48090_e61682_d_n9;

        let (assign48100_e61700, assign48100_e61700_d_n4, assign48100_e61700_d_n6, assign48100_e61700_d_n7, assign48100_e61700_d_n8, assign48100_e61700_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48100_e61691: f64 = (locals.var_q_edge_sqerr).sqrt();
        let assign48100_e61693: f64 = (assign48100_e61691 - locals.var_q_edge_n);
        let assign48100_e61695: f64 = (assign48100_e61693 / locals.var_q_edge_d0p);
        let assign48100_e61697: f64 = (assign48100_e61695 - 1.0);
        let assign48100_e61698: f64 = (locals.var_q_edge_n * assign48100_e61697);
        (assign48100_e61698, ((locals.var_q_edge_n_dn4 * assign48100_e61697) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn4 / (2.0 * assign48100_e61691)) - locals.var_q_edge_n_dn4) * locals.var_q_edge_d0p) - (assign48100_e61693 * locals.var_q_edge_d0p_dn4)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn6 * assign48100_e61697) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn6 / (2.0 * assign48100_e61691)) - locals.var_q_edge_n_dn6) * locals.var_q_edge_d0p) - (assign48100_e61693 * locals.var_q_edge_d0p_dn6)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn7 * assign48100_e61697) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn7 / (2.0 * assign48100_e61691)) - locals.var_q_edge_n_dn7) * locals.var_q_edge_d0p) - (assign48100_e61693 * locals.var_q_edge_d0p_dn7)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn8 * assign48100_e61697) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn8 / (2.0 * assign48100_e61691)) - locals.var_q_edge_n_dn8) * locals.var_q_edge_d0p) - (assign48100_e61693 * locals.var_q_edge_d0p_dn8)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn9 * assign48100_e61697) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn9 / (2.0 * assign48100_e61691)) - locals.var_q_edge_n_dn9) * locals.var_q_edge_d0p) - (assign48100_e61693 * locals.var_q_edge_d0p_dn9)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))),)
    } else {
        (locals.var_q_edge_errq, locals.var_q_edge_errq_dn4, locals.var_q_edge_errq_dn6, locals.var_q_edge_errq_dn7, locals.var_q_edge_errq_dn8, locals.var_q_edge_errq_dn9,)
    }
};
        locals.var_q_edge_errq = assign48100_e61700;
        locals.var_q_edge_errq_dn4 = assign48100_e61700_d_n4;
        locals.var_q_edge_errq_dn6 = assign48100_e61700_d_n6;
        locals.var_q_edge_errq_dn7 = assign48100_e61700_d_n7;
        locals.var_q_edge_errq_dn8 = assign48100_e61700_d_n8;
        locals.var_q_edge_errq_dn9 = assign48100_e61700_d_n9;

        let (assign48110_e61711, assign48110_e61711_d_n4, assign48110_e61711_d_n6, assign48110_e61711_d_n7, assign48110_e61711_d_n8, assign48110_e61711_d_n9,) = {
    if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
        let assign48110_e61709: f64 = (locals.var_q_edge_qi0 - locals.var_q_edge_errq);
        (assign48110_e61709, (locals.var_q_edge_qi0_dn4 - locals.var_q_edge_errq_dn4), (locals.var_q_edge_qi0_dn6 - locals.var_q_edge_errq_dn6), (locals.var_q_edge_qi0_dn7 - locals.var_q_edge_errq_dn7), (locals.var_q_edge_qi0_dn8 - locals.var_q_edge_errq_dn8), (locals.var_q_edge_qi0_dn9 - locals.var_q_edge_errq_dn9),)
    } else {
        (locals.var_qdeffedge, locals.var_qdeffedge_dn4, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8, locals.var_qdeffedge_dn9,)
    }
};
        locals.var_qdeffedge = assign48110_e61711;
        locals.var_qdeffedge_dn4 = assign48110_e61711_d_n4;
        locals.var_qdeffedge_dn6 = assign48110_e61711_d_n6;
        locals.var_qdeffedge_dn7 = assign48110_e61711_d_n7;
        locals.var_qdeffedge_dn8 = assign48110_e61711_d_n8;
        locals.var_qdeffedge_dn9 = assign48110_e61711_d_n9;

        let assign48120_e61715: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign48120_e61716: f64 = (locals.var_q_edge_n_inv * assign48120_e61715);
        let assign48120_e61718: f64 = (-230.25850929940458);
        let assign48120_e61719: f64 = if assign48120_e61716 > assign48120_e61718 { 1.0 } else { 0.0 };
        locals.var_guard1274 = assign48120_e61719;

        let (assign48130_e61736, assign48130_e61736_d_n4, assign48130_e61736_d_n6, assign48130_e61736_d_n7, assign48130_e61736_d_n8, assign48130_e61736_d_n9,) = {
    if ((((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 == 0.0)) && (locals.var_guard1274 != 0.0)) {
        let assign48130_e61732: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign48130_e61733: f64 = (locals.var_q_edge_n_inv * assign48130_e61732);
        let assign48130_e61734: f64 = (assign48130_e61733).exp();
        (assign48130_e61734, (assign48130_e61734 * ((locals.var_q_edge_n_inv_dn4 * assign48130_e61732) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))), (assign48130_e61734 * ((locals.var_q_edge_n_inv_dn6 * assign48130_e61732) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))), (assign48130_e61734 * ((locals.var_q_edge_n_inv_dn7 * assign48130_e61732) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))), (assign48130_e61734 * ((locals.var_q_edge_n_inv_dn8 * assign48130_e61732) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))), (assign48130_e61734 * ((locals.var_q_edge_n_inv_dn9 * assign48130_e61732) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))),)
    } else {
        (locals.var_qdeffedge, locals.var_qdeffedge_dn4, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8, locals.var_qdeffedge_dn9,)
    }
};
        locals.var_qdeffedge = assign48130_e61736;
        locals.var_qdeffedge_dn4 = assign48130_e61736_d_n4;
        locals.var_qdeffedge_dn6 = assign48130_e61736_d_n6;
        locals.var_qdeffedge_dn7 = assign48130_e61736_d_n7;
        locals.var_qdeffedge_dn8 = assign48130_e61736_d_n8;
        locals.var_qdeffedge_dn9 = assign48130_e61736_d_n9;

        let (assign48140_e61786, assign48140_e61786_d_n4, assign48140_e61786_d_n6, assign48140_e61786_d_n7, assign48140_e61786_d_n8, assign48140_e61786_d_n9,) = {
    if ((((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 == 0.0)) && (locals.var_guard1274 == 0.0)) {
        let assign48140_e61750: f64 = (-230.25850929940458);
        let assign48140_e61754: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign48140_e61755: f64 = (locals.var_q_edge_n_inv * assign48140_e61754);
        let assign48140_e61756: f64 = (assign48140_e61750 - assign48140_e61755);
        let assign48140_e61760: f64 = (-230.25850929940458);
        let assign48140_e61764: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign48140_e61765: f64 = (locals.var_q_edge_n_inv * assign48140_e61764);
        let assign48140_e61766: f64 = (assign48140_e61760 - assign48140_e61765);
        let assign48140_e61769: f64 = (-230.25850929940458);
        let assign48140_e61773: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign48140_e61774: f64 = (locals.var_q_edge_n_inv * assign48140_e61773);
        let assign48140_e61775: f64 = (assign48140_e61769 - assign48140_e61774);
        let assign48140_e61777: f64 = (assign48140_e61775 * 0.3333333333333333);
        let assign48140_e61778: f64 = (1.0 + assign48140_e61777);
        let assign48140_e61779: f64 = (assign48140_e61766 * assign48140_e61778);
        let assign48140_e61780: f64 = (0.5 * assign48140_e61779);
        let assign48140_e61781: f64 = (1.0 + assign48140_e61780);
        let assign48140_e61782: f64 = (assign48140_e61756 * assign48140_e61781);
        let assign48140_e61783: f64 = (1.0 + assign48140_e61782);
        let assign48140_e61784: f64 = (1e-100 / assign48140_e61783);
        (assign48140_e61784, (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn4 * assign48140_e61754) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * assign48140_e61781) + (assign48140_e61756 * (0.5 * (((-((locals.var_q_edge_n_inv_dn4 * assign48140_e61764) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * assign48140_e61778) + (assign48140_e61766 * ((-((locals.var_q_edge_n_inv_dn4 * assign48140_e61773) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * 0.3333333333333333))))))) / (assign48140_e61783 * assign48140_e61783))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn6 * assign48140_e61754) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign48140_e61781) + (assign48140_e61756 * (0.5 * (((-((locals.var_q_edge_n_inv_dn6 * assign48140_e61764) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign48140_e61778) + (assign48140_e61766 * ((-((locals.var_q_edge_n_inv_dn6 * assign48140_e61773) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * 0.3333333333333333))))))) / (assign48140_e61783 * assign48140_e61783))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn7 * assign48140_e61754) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign48140_e61781) + (assign48140_e61756 * (0.5 * (((-((locals.var_q_edge_n_inv_dn7 * assign48140_e61764) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign48140_e61778) + (assign48140_e61766 * ((-((locals.var_q_edge_n_inv_dn7 * assign48140_e61773) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * 0.3333333333333333))))))) / (assign48140_e61783 * assign48140_e61783))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn8 * assign48140_e61754) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign48140_e61781) + (assign48140_e61756 * (0.5 * (((-((locals.var_q_edge_n_inv_dn8 * assign48140_e61764) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign48140_e61778) + (assign48140_e61766 * ((-((locals.var_q_edge_n_inv_dn8 * assign48140_e61773) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * 0.3333333333333333))))))) / (assign48140_e61783 * assign48140_e61783))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn9 * assign48140_e61754) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * assign48140_e61781) + (assign48140_e61756 * (0.5 * (((-((locals.var_q_edge_n_inv_dn9 * assign48140_e61764) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * assign48140_e61778) + (assign48140_e61766 * ((-((locals.var_q_edge_n_inv_dn9 * assign48140_e61773) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * 0.3333333333333333))))))) / (assign48140_e61783 * assign48140_e61783))),)
    } else {
        (locals.var_qdeffedge, locals.var_qdeffedge_dn4, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8, locals.var_qdeffedge_dn9,)
    }
};
        locals.var_qdeffedge = assign48140_e61786;
        locals.var_qdeffedge_dn4 = assign48140_e61786_d_n4;
        locals.var_qdeffedge_dn6 = assign48140_e61786_d_n6;
        locals.var_qdeffedge_dn7 = assign48140_e61786_d_n7;
        locals.var_qdeffedge_dn8 = assign48140_e61786_d_n8;
        locals.var_qdeffedge_dn9 = assign48140_e61786_d_n9;

        let (assign48150_e61795, assign48150_e61795_d_n4, assign48150_e61795_d_n6, assign48150_e61795_d_n7, assign48150_e61795_d_n8, assign48150_e61795_d_n9,) = {
    if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
        let assign48150_e61793: f64 = (locals.var_qdeffedge - locals.var_qseffedge);
        (assign48150_e61793, (locals.var_qdeffedge_dn4 - locals.var_qseffedge_dn4), (locals.var_qdeffedge_dn6 - locals.var_qseffedge_dn6), (locals.var_qdeffedge_dn7 - locals.var_qseffedge_dn7), (locals.var_qdeffedge_dn8 - locals.var_qseffedge_dn8), (locals.var_qdeffedge_dn9 - locals.var_qseffedge_dn9),)
    } else {
        (locals.var_qdseffedge, locals.var_qdseffedge_dn4, locals.var_qdseffedge_dn6, locals.var_qdseffedge_dn7, locals.var_qdseffedge_dn8, locals.var_qdseffedge_dn9,)
    }
};
        locals.var_qdseffedge = assign48150_e61795;
        locals.var_qdseffedge_dn4 = assign48150_e61795_d_n4;
        locals.var_qdseffedge_dn6 = assign48150_e61795_d_n6;
        locals.var_qdseffedge_dn7 = assign48150_e61795_d_n7;
        locals.var_qdseffedge_dn8 = assign48150_e61795_d_n8;
        locals.var_qdseffedge_dn9 = assign48150_e61795_d_n9;

        let (assign48160_e61803, assign48160_e61803_d_n4, assign48160_e61803_d_n6, assign48160_e61803_d_n7, assign48160_e61803_d_n8, assign48160_e61803_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign48160_e61800: f64 = (locals.var_qdeffedge + locals.var_qseffedge);
        let assign48160_e61801: f64 = (0.5 * assign48160_e61800);
        (assign48160_e61801, (0.5 * (locals.var_qdeffedge_dn4 + locals.var_qseffedge_dn4)), (0.5 * (locals.var_qdeffedge_dn6 + locals.var_qseffedge_dn6)), (0.5 * (locals.var_qdeffedge_dn7 + locals.var_qseffedge_dn7)), (0.5 * (locals.var_qdeffedge_dn8 + locals.var_qseffedge_dn8)), (0.5 * (locals.var_qdeffedge_dn9 + locals.var_qseffedge_dn9)),)
    } else {
        (locals.var_qmeffedge, locals.var_qmeffedge_dn4, locals.var_qmeffedge_dn6, locals.var_qmeffedge_dn7, locals.var_qmeffedge_dn8, locals.var_qmeffedge_dn9,)
    }
};
        locals.var_qmeffedge = assign48160_e61803;
        locals.var_qmeffedge_dn4 = assign48160_e61803_d_n4;
        locals.var_qmeffedge_dn6 = assign48160_e61803_d_n6;
        locals.var_qmeffedge_dn7 = assign48160_e61803_d_n7;
        locals.var_qmeffedge_dn8 = assign48160_e61803_d_n8;
        locals.var_qmeffedge_dn9 = assign48160_e61803_d_n9;

        let (assign48170_e61816, assign48170_e61816_d_n4, assign48170_e61816_d_n6, assign48170_e61816_d_n7, assign48170_e61816_d_n8, assign48170_e61816_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign48170_e61807: f64 = (locals.var_xgedge - locals.var_qmeffedge);
        let (assign48170_e61814, assign48170_e61814_d_n4, assign48170_e61814_d_n6, assign48170_e61814_d_n7, assign48170_e61814_d_n8, assign48170_e61814_d_n9,) = {
            if (assign48170_e61807 > 1e-40) {
                let assign48170_e61812: f64 = (locals.var_xgedge - locals.var_qmeffedge);
                (assign48170_e61812, (locals.var_xgedge_dn4 - locals.var_qmeffedge_dn4), (locals.var_xgedge_dn6 - locals.var_qmeffedge_dn6), (locals.var_xgedge_dn7 - locals.var_qmeffedge_dn7), (locals.var_xgedge_dn8 - locals.var_qmeffedge_dn8), (locals.var_xgedge_dn9 - locals.var_qmeffedge_dn9),)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign48170_e61814, assign48170_e61814_d_n4, assign48170_e61814_d_n6, assign48170_e61814_d_n7, assign48170_e61814_d_n8, assign48170_e61814_d_n9,)
    } else {
        (locals.var_dsqredge, locals.var_dsqredge_dn4, locals.var_dsqredge_dn6, locals.var_dsqredge_dn7, locals.var_dsqredge_dn8, locals.var_dsqredge_dn9,)
    }
};
        locals.var_dsqredge = assign48170_e61816;
        locals.var_dsqredge_dn4 = assign48170_e61816_d_n4;
        locals.var_dsqredge_dn6 = assign48170_e61816_d_n6;
        locals.var_dsqredge_dn7 = assign48170_e61816_d_n7;
        locals.var_dsqredge_dn8 = assign48170_e61816_d_n8;
        locals.var_dsqredge_dn9 = assign48170_e61816_d_n9;

        let (assign48180_e61831, assign48180_e61831_d_n4, assign48180_e61831_d_n6, assign48180_e61831_d_n7, assign48180_e61831_d_n8, assign48180_e61831_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign48180_e61821: f64 = (0.5 * locals.var_gfedge);
        let assign48180_e61825: f64 = (0.25 * locals.var_gfedge2);
        let assign48180_e61826: f64 = (locals.var_dsqredge + assign48180_e61825);
        let assign48180_e61827: f64 = (assign48180_e61826).sqrt();
        let assign48180_e61828: f64 = (assign48180_e61821 / assign48180_e61827);
        let assign48180_e61829: f64 = (1.0 - assign48180_e61828);
        (assign48180_e61829, (-((((0.5 * locals.var_gfedge_dn4) * assign48180_e61827) - (assign48180_e61821 * ((locals.var_dsqredge_dn4 + (0.25 * locals.var_gfedge2_dn4)) / (2.0 * assign48180_e61827)))) / (assign48180_e61827 * assign48180_e61827))), (-(-((assign48180_e61821 * (locals.var_dsqredge_dn6 / (2.0 * assign48180_e61827))) / (assign48180_e61827 * assign48180_e61827)))), (-(-((assign48180_e61821 * (locals.var_dsqredge_dn7 / (2.0 * assign48180_e61827))) / (assign48180_e61827 * assign48180_e61827)))), (-(-((assign48180_e61821 * (locals.var_dsqredge_dn8 / (2.0 * assign48180_e61827))) / (assign48180_e61827 * assign48180_e61827)))), (-(-((assign48180_e61821 * (locals.var_dsqredge_dn9 / (2.0 * assign48180_e61827))) / (assign48180_e61827 * assign48180_e61827)))),)
    } else {
        (locals.var_alphabmedge, locals.var_alphabmedge_dn4, locals.var_alphabmedge_dn6, locals.var_alphabmedge_dn7, locals.var_alphabmedge_dn8, locals.var_alphabmedge_dn9,)
    }
};
        locals.var_alphabmedge = assign48180_e61831;
        locals.var_alphabmedge_dn4 = assign48180_e61831_d_n4;
        locals.var_alphabmedge_dn6 = assign48180_e61831_d_n6;
        locals.var_alphabmedge_dn7 = assign48180_e61831_d_n7;
        locals.var_alphabmedge_dn8 = assign48180_e61831_d_n8;
        locals.var_alphabmedge_dn9 = assign48180_e61831_d_n9;

        let (assign48190_e61850, assign48190_e61850_d_n4, assign48190_e61850_d_n6, assign48190_e61850_d_n7, assign48190_e61850_d_n8, assign48190_e61850_d_n9,) = {
    if (locals.var_guard1266 != 0.0) {
        let assign48190_e61834: f64 = (-locals.var_betedge_i);
        let assign48190_e61836: f64 = (assign48190_e61834 * locals.var_phit1edge);
        let assign48190_e61838: f64 = (assign48190_e61836 * locals.var_phit1edge);
        let assign48190_e61841: f64 = (locals.var_alphabmedge * locals.var_qmeffedge);
        let assign48190_e61843: f64 = (assign48190_e61841 + 1.0);
        let assign48190_e61844: f64 = (assign48190_e61838 * assign48190_e61843);
        let assign48190_e61846: f64 = (assign48190_e61844 * locals.var_qdseffedge);
        let assign48190_e61848: f64 = (assign48190_e61846 / locals.var_gmob_dc);
        (assign48190_e61848, ((((((((((((-locals.var_betedge_i_dn4) * locals.var_phit1edge) + (assign48190_e61834 * locals.var_phit1edge_dn4)) * locals.var_phit1edge) + (assign48190_e61836 * locals.var_phit1edge_dn4)) * assign48190_e61843) + (assign48190_e61838 * ((locals.var_alphabmedge_dn4 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn4)))) * locals.var_qdseffedge) + (assign48190_e61844 * locals.var_qdseffedge_dn4)) * locals.var_gmob_dc) - (assign48190_e61846 * locals.var_gmob_dc_dn4)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48190_e61834 * locals.var_phit1edge_dn6) * locals.var_phit1edge) + (assign48190_e61836 * locals.var_phit1edge_dn6)) * assign48190_e61843) + (assign48190_e61838 * ((locals.var_alphabmedge_dn6 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn6)))) * locals.var_qdseffedge) + (assign48190_e61844 * locals.var_qdseffedge_dn6)) * locals.var_gmob_dc) - (assign48190_e61846 * locals.var_gmob_dc_dn6)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48190_e61834 * locals.var_phit1edge_dn7) * locals.var_phit1edge) + (assign48190_e61836 * locals.var_phit1edge_dn7)) * assign48190_e61843) + (assign48190_e61838 * ((locals.var_alphabmedge_dn7 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn7)))) * locals.var_qdseffedge) + (assign48190_e61844 * locals.var_qdseffedge_dn7)) * locals.var_gmob_dc) - (assign48190_e61846 * locals.var_gmob_dc_dn7)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48190_e61834 * locals.var_phit1edge_dn8) * locals.var_phit1edge) + (assign48190_e61836 * locals.var_phit1edge_dn8)) * assign48190_e61843) + (assign48190_e61838 * ((locals.var_alphabmedge_dn8 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn8)))) * locals.var_qdseffedge) + (assign48190_e61844 * locals.var_qdseffedge_dn8)) * locals.var_gmob_dc) - (assign48190_e61846 * locals.var_gmob_dc_dn8)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48190_e61834 * locals.var_phit1edge_dn9) * locals.var_phit1edge) + (assign48190_e61836 * locals.var_phit1edge_dn9)) * assign48190_e61843) + (assign48190_e61838 * ((locals.var_alphabmedge_dn9 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn9)))) * locals.var_qdseffedge) + (assign48190_e61844 * locals.var_qdseffedge_dn9)) * locals.var_gmob_dc) - (assign48190_e61846 * locals.var_gmob_dc_dn9)) / (locals.var_gmob_dc * locals.var_gmob_dc)),)
    } else {
        (locals.var_i_dsedge, locals.var_i_dsedge_dn4, locals.var_i_dsedge_dn6, locals.var_i_dsedge_dn7, locals.var_i_dsedge_dn8, locals.var_i_dsedge_dn9,)
    }
};
        locals.var_i_dsedge = assign48190_e61850;
        locals.var_i_dsedge_dn4 = assign48190_e61850_d_n4;
        locals.var_i_dsedge_dn6 = assign48190_e61850_d_n6;
        locals.var_i_dsedge_dn7 = assign48190_e61850_d_n7;
        locals.var_i_dsedge_dn8 = assign48190_e61850_d_n8;
        locals.var_i_dsedge_dn9 = assign48190_e61850_d_n9;

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

        let assign48220_e61859: f64 = if ((locals.var_xg_dc > 0.0) && (p.p41 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1275 = assign48220_e61859;

        let (assign48230_e61867, assign48230_e61867_d_n4, assign48230_e61867_d_n6, assign48230_e61867_d_n7, assign48230_e61867_d_n8, assign48230_e61867_d_n9,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign48230_e61864: f64 = (locals.var_a3_i * locals.var_dps_dc);
        let assign48230_e61865: f64 = (locals.var_v_ds - assign48230_e61864);
        (assign48230_e61865, (-(locals.var_a3_i * locals.var_dps_dc_dn4)), (-(locals.var_a3_i * locals.var_dps_dc_dn6)), (locals.var_v_ds_dn7 - (locals.var_a3_i * locals.var_dps_dc_dn7)), (locals.var_v_ds_dn8 - (locals.var_a3_i * locals.var_dps_dc_dn8)), (-(locals.var_a3_i * locals.var_dps_dc_dn9)),)
    } else {
        (locals.var_delvsat, locals.var_delvsat_dn4, locals.var_delvsat_dn6, locals.var_delvsat_dn7, locals.var_delvsat_dn8, locals.var_delvsat_dn9,)
    }
};
        locals.var_delvsat = assign48230_e61867;
        locals.var_delvsat_dn4 = assign48230_e61867_d_n4;
        locals.var_delvsat_dn6 = assign48230_e61867_d_n6;
        locals.var_delvsat_dn7 = assign48230_e61867_d_n7;
        locals.var_delvsat_dn8 = assign48230_e61867_d_n8;
        locals.var_delvsat_dn9 = assign48230_e61867_d_n9;

        let assign48240_e61870: f64 = if locals.var_delvsat > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1276 = assign48240_e61870;

        let (assign48250_e61891, assign48250_e61891_d_n4, assign48250_e61891_d_n6, assign48250_e61891_d_n7, assign48250_e61891_d_n8, assign48250_e61891_d_n9,) = {
    if ((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) {
        let assign48250_e61879: f64 = (locals.var_phib_dc + locals.var_vsbstar_dc);
        let assign48250_e61880: f64 = (assign48250_e61879).sqrt();
        let assign48250_e61882: f64 = (assign48250_e61880 - locals.var_sqrt_phib_dc);
        let assign48250_e61883: f64 = (locals.var_a4_i * assign48250_e61882);
        let assign48250_e61884: f64 = (1.0 + assign48250_e61883);
        let assign48250_e61887: f64 = (locals.var_delvsat + 1e-30);
        let assign48250_e61888: f64 = (assign48250_e61884 / assign48250_e61887);
        let assign48250_e61889: f64 = (locals.var_a2_t * assign48250_e61888);
        (assign48250_e61889, ((locals.var_a2_t_dn4 * assign48250_e61888) + (locals.var_a2_t * ((((locals.var_a4_i * (((locals.var_phib_dc_dn4 + locals.var_vsbstar_dc_dn4) / (2.0 * assign48250_e61880)) - locals.var_sqrt_phib_dc_dn4)) * assign48250_e61887) - (assign48250_e61884 * locals.var_delvsat_dn4)) / (assign48250_e61887 * assign48250_e61887)))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn6 / (2.0 * assign48250_e61880))) * assign48250_e61887) - (assign48250_e61884 * locals.var_delvsat_dn6)) / (assign48250_e61887 * assign48250_e61887))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn7 / (2.0 * assign48250_e61880))) * assign48250_e61887) - (assign48250_e61884 * locals.var_delvsat_dn7)) / (assign48250_e61887 * assign48250_e61887))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn8 / (2.0 * assign48250_e61880))) * assign48250_e61887) - (assign48250_e61884 * locals.var_delvsat_dn8)) / (assign48250_e61887 * assign48250_e61887))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn9 / (2.0 * assign48250_e61880))) * assign48250_e61887) - (assign48250_e61884 * locals.var_delvsat_dn9)) / (assign48250_e61887 * assign48250_e61887))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign48250_e61891;
        locals.var_temp2_dn4 = assign48250_e61891_d_n4;
        locals.var_temp2_dn6 = assign48250_e61891_d_n6;
        locals.var_temp2_dn7 = assign48250_e61891_d_n7;
        locals.var_temp2_dn8 = assign48250_e61891_d_n8;
        locals.var_temp2_dn9 = assign48250_e61891_d_n9;

        let assign48260_e61893: f64 = (-locals.var_temp2);
        let assign48260_e61894: f64 = (assign48260_e61893).abs();
        let assign48260_e61896: f64 = if assign48260_e61894 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1277 = assign48260_e61896;

        let (assign48270_e61906, assign48270_e61906_d_n4, assign48270_e61906_d_n6, assign48270_e61906_d_n7, assign48270_e61906_d_n8, assign48270_e61906_d_n9,) = {
    if (((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) && (locals.var_guard1277 != 0.0)) {
        let assign48270_e61903: f64 = (-locals.var_temp2);
        let assign48270_e61904: f64 = (assign48270_e61903).exp();
        (assign48270_e61904, (assign48270_e61904 * (-locals.var_temp2_dn4)), (assign48270_e61904 * (-locals.var_temp2_dn6)), (assign48270_e61904 * (-locals.var_temp2_dn7)), (assign48270_e61904 * (-locals.var_temp2_dn8)), (assign48270_e61904 * (-locals.var_temp2_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign48270_e61906;
        locals.var_temp__blk949_dn4 = assign48270_e61906_d_n4;
        locals.var_temp__blk949_dn6 = assign48270_e61906_d_n6;
        locals.var_temp__blk949_dn7 = assign48270_e61906_d_n7;
        locals.var_temp__blk949_dn8 = assign48270_e61906_d_n8;
        locals.var_temp__blk949_dn9 = assign48270_e61906_d_n9;

        let assign48280_e61908: f64 = (-locals.var_temp2);
        let assign48280_e61910: f64 = if assign48280_e61908 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1278 = assign48280_e61910;

    }

    pub(super) fn stamp_transient_block_35(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48290_e61949, assign48290_e61949_d_n4, assign48290_e61949_d_n6, assign48290_e61949_d_n7, assign48290_e61949_d_n8, assign48290_e61949_d_n9,) = {
    if ((((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) && (locals.var_guard1277 == 0.0)) && (locals.var_guard1278 != 0.0)) {
        let assign48290_e61922: f64 = (-230.25850929940458);
        let assign48290_e61924: f64 = (-locals.var_temp2);
        let assign48290_e61925: f64 = (assign48290_e61922 - assign48290_e61924);
        let assign48290_e61929: f64 = (-230.25850929940458);
        let assign48290_e61931: f64 = (-locals.var_temp2);
        let assign48290_e61932: f64 = (assign48290_e61929 - assign48290_e61931);
        let assign48290_e61935: f64 = (-230.25850929940458);
        let assign48290_e61937: f64 = (-locals.var_temp2);
        let assign48290_e61938: f64 = (assign48290_e61935 - assign48290_e61937);
        let assign48290_e61940: f64 = (assign48290_e61938 * 0.3333333333333333);
        let assign48290_e61941: f64 = (1.0 + assign48290_e61940);
        let assign48290_e61942: f64 = (assign48290_e61932 * assign48290_e61941);
        let assign48290_e61943: f64 = (0.5 * assign48290_e61942);
        let assign48290_e61944: f64 = (1.0 + assign48290_e61943);
        let assign48290_e61945: f64 = (assign48290_e61925 * assign48290_e61944);
        let assign48290_e61946: f64 = (1.0 + assign48290_e61945);
        let assign48290_e61947: f64 = (1e-100 / assign48290_e61946);
        (assign48290_e61947, (-((1e-100 * (((-(-locals.var_temp2_dn4)) * assign48290_e61944) + (assign48290_e61925 * (0.5 * (((-(-locals.var_temp2_dn4)) * assign48290_e61941) + (assign48290_e61932 * ((-(-locals.var_temp2_dn4)) * 0.3333333333333333))))))) / (assign48290_e61946 * assign48290_e61946))), (-((1e-100 * (((-(-locals.var_temp2_dn6)) * assign48290_e61944) + (assign48290_e61925 * (0.5 * (((-(-locals.var_temp2_dn6)) * assign48290_e61941) + (assign48290_e61932 * ((-(-locals.var_temp2_dn6)) * 0.3333333333333333))))))) / (assign48290_e61946 * assign48290_e61946))), (-((1e-100 * (((-(-locals.var_temp2_dn7)) * assign48290_e61944) + (assign48290_e61925 * (0.5 * (((-(-locals.var_temp2_dn7)) * assign48290_e61941) + (assign48290_e61932 * ((-(-locals.var_temp2_dn7)) * 0.3333333333333333))))))) / (assign48290_e61946 * assign48290_e61946))), (-((1e-100 * (((-(-locals.var_temp2_dn8)) * assign48290_e61944) + (assign48290_e61925 * (0.5 * (((-(-locals.var_temp2_dn8)) * assign48290_e61941) + (assign48290_e61932 * ((-(-locals.var_temp2_dn8)) * 0.3333333333333333))))))) / (assign48290_e61946 * assign48290_e61946))), (-((1e-100 * (((-(-locals.var_temp2_dn9)) * assign48290_e61944) + (assign48290_e61925 * (0.5 * (((-(-locals.var_temp2_dn9)) * assign48290_e61941) + (assign48290_e61932 * ((-(-locals.var_temp2_dn9)) * 0.3333333333333333))))))) / (assign48290_e61946 * assign48290_e61946))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign48290_e61949;
        locals.var_temp__blk949_dn4 = assign48290_e61949_d_n4;
        locals.var_temp__blk949_dn6 = assign48290_e61949_d_n6;
        locals.var_temp__blk949_dn7 = assign48290_e61949_d_n7;
        locals.var_temp__blk949_dn8 = assign48290_e61949_d_n8;
        locals.var_temp__blk949_dn9 = assign48290_e61949_d_n9;

        let (assign48300_e61986, assign48300_e61986_d_n4, assign48300_e61986_d_n6, assign48300_e61986_d_n7, assign48300_e61986_d_n8, assign48300_e61986_d_n9,) = {
    if ((((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) && (locals.var_guard1277 == 0.0)) && (locals.var_guard1278 == 0.0)) {
        let assign48300_e61962: f64 = (-locals.var_temp2);
        let assign48300_e61964: f64 = (assign48300_e61962 - 230.25850929940458);
        let assign48300_e61968: f64 = (-locals.var_temp2);
        let assign48300_e61970: f64 = (assign48300_e61968 - 230.25850929940458);
        let assign48300_e61973: f64 = (-locals.var_temp2);
        let assign48300_e61975: f64 = (assign48300_e61973 - 230.25850929940458);
        let assign48300_e61977: f64 = (assign48300_e61975 * 0.3333333333333333);
        let assign48300_e61978: f64 = (1.0 + assign48300_e61977);
        let assign48300_e61979: f64 = (assign48300_e61970 * assign48300_e61978);
        let assign48300_e61980: f64 = (0.5 * assign48300_e61979);
        let assign48300_e61981: f64 = (1.0 + assign48300_e61980);
        let assign48300_e61982: f64 = (assign48300_e61964 * assign48300_e61981);
        let assign48300_e61983: f64 = (1.0 + assign48300_e61982);
        let assign48300_e61984: f64 = (1e100 * assign48300_e61983);
        (assign48300_e61984, (1e100 * (((-locals.var_temp2_dn4) * assign48300_e61981) + (assign48300_e61964 * (0.5 * (((-locals.var_temp2_dn4) * assign48300_e61978) + (assign48300_e61970 * ((-locals.var_temp2_dn4) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn6) * assign48300_e61981) + (assign48300_e61964 * (0.5 * (((-locals.var_temp2_dn6) * assign48300_e61978) + (assign48300_e61970 * ((-locals.var_temp2_dn6) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn7) * assign48300_e61981) + (assign48300_e61964 * (0.5 * (((-locals.var_temp2_dn7) * assign48300_e61978) + (assign48300_e61970 * ((-locals.var_temp2_dn7) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn8) * assign48300_e61981) + (assign48300_e61964 * (0.5 * (((-locals.var_temp2_dn8) * assign48300_e61978) + (assign48300_e61970 * ((-locals.var_temp2_dn8) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn9) * assign48300_e61981) + (assign48300_e61964 * (0.5 * (((-locals.var_temp2_dn9) * assign48300_e61978) + (assign48300_e61970 * ((-locals.var_temp2_dn9) * 0.3333333333333333))))))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign48300_e61986;
        locals.var_temp__blk949_dn4 = assign48300_e61986_d_n4;
        locals.var_temp__blk949_dn6 = assign48300_e61986_d_n6;
        locals.var_temp__blk949_dn7 = assign48300_e61986_d_n7;
        locals.var_temp__blk949_dn8 = assign48300_e61986_d_n8;
        locals.var_temp__blk949_dn9 = assign48300_e61986_d_n9;

        let (assign48310_e61996, assign48310_e61996_d_n4, assign48310_e61996_d_n6, assign48310_e61996_d_n7, assign48310_e61996_d_n8, assign48310_e61996_d_n9,) = {
    if ((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) {
        let assign48310_e61993: f64 = (locals.var_delvsat * locals.var_temp__blk949);
        let assign48310_e61994: f64 = (locals.var_a1_i * assign48310_e61993);
        (assign48310_e61994, (locals.var_a1_i * ((locals.var_delvsat_dn4 * locals.var_temp__blk949) + (locals.var_delvsat * locals.var_temp__blk949_dn4))), (locals.var_a1_i * ((locals.var_delvsat_dn6 * locals.var_temp__blk949) + (locals.var_delvsat * locals.var_temp__blk949_dn6))), (locals.var_a1_i * ((locals.var_delvsat_dn7 * locals.var_temp__blk949) + (locals.var_delvsat * locals.var_temp__blk949_dn7))), (locals.var_a1_i * ((locals.var_delvsat_dn8 * locals.var_temp__blk949) + (locals.var_delvsat * locals.var_temp__blk949_dn8))), (locals.var_a1_i * ((locals.var_delvsat_dn9 * locals.var_temp__blk949) + (locals.var_delvsat * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_mavl, locals.var_mavl_dn4, locals.var_mavl_dn6, locals.var_mavl_dn7, locals.var_mavl_dn8, locals.var_mavl_dn9,)
    }
};
        locals.var_mavl = assign48310_e61996;
        locals.var_mavl_dn4 = assign48310_e61996_d_n4;
        locals.var_mavl_dn6 = assign48310_e61996_d_n6;
        locals.var_mavl_dn7 = assign48310_e61996_d_n7;
        locals.var_mavl_dn8 = assign48310_e61996_d_n8;
        locals.var_mavl_dn9 = assign48310_e61996_d_n9;

        let (assign48320_e62006, assign48320_e62006_d_n4, assign48320_e62006_d_n6, assign48320_e62006_d_n7, assign48320_e62006_d_n8, assign48320_e62006_d_n9,) = {
    if ((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) {
        let assign48320_e62003: f64 = (locals.var_i_ds + locals.var_i_dsedge);
        let assign48320_e62004: f64 = (locals.var_mavl * assign48320_e62003);
        (assign48320_e62004, ((locals.var_mavl_dn4 * assign48320_e62003) + (locals.var_mavl * (locals.var_i_ds_dn4 + locals.var_i_dsedge_dn4))), ((locals.var_mavl_dn6 * assign48320_e62003) + (locals.var_mavl * (locals.var_i_ds_dn6 + locals.var_i_dsedge_dn6))), ((locals.var_mavl_dn7 * assign48320_e62003) + (locals.var_mavl * (locals.var_i_ds_dn7 + locals.var_i_dsedge_dn7))), ((locals.var_mavl_dn8 * assign48320_e62003) + (locals.var_mavl * (locals.var_i_ds_dn8 + locals.var_i_dsedge_dn8))), ((locals.var_mavl_dn9 * assign48320_e62003) + (locals.var_mavl * (locals.var_i_ds_dn9 + locals.var_i_dsedge_dn9))),)
    } else {
        (locals.var_iimpact, locals.var_iimpact_dn4, locals.var_iimpact_dn6, locals.var_iimpact_dn7, locals.var_iimpact_dn8, locals.var_iimpact_dn9,)
    }
};
        locals.var_iimpact = assign48320_e62006;
        locals.var_iimpact_dn4 = assign48320_e62006_d_n4;
        locals.var_iimpact_dn6 = assign48320_e62006_d_n6;
        locals.var_iimpact_dn7 = assign48320_e62006_d_n7;
        locals.var_iimpact_dn8 = assign48320_e62006_d_n8;
        locals.var_iimpact_dn9 = assign48320_e62006_d_n9;

        let assign48330_e62010: f64 = (0.5 * locals.var_imaxii_i);
        let assign48330_e62011: f64 = if locals.var_iimpact > assign48330_e62010 { 1.0 } else { 0.0 };
        locals.var_guard1279 = assign48330_e62011;

        let (assign48340_e62025, assign48340_e62025_d_n4, assign48340_e62025_d_n6, assign48340_e62025_d_n7, assign48340_e62025_d_n8, assign48340_e62025_d_n9,) = {
    if (((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) && (locals.var_guard1279 != 0.0)) {
        let assign48340_e62019: f64 = (2.0 * locals.var_iimpact);
        let assign48340_e62021: f64 = (assign48340_e62019 / locals.var_imaxii_i);
        let assign48340_e62023: f64 = (assign48340_e62021 - 1.0);
        (assign48340_e62023, ((2.0 * locals.var_iimpact_dn4) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn6) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn7) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn8) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn9) / locals.var_imaxii_i),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign48340_e62025;
        locals.var_temp__blk949_dn4 = assign48340_e62025_d_n4;
        locals.var_temp__blk949_dn6 = assign48340_e62025_d_n6;
        locals.var_temp__blk949_dn7 = assign48340_e62025_d_n7;
        locals.var_temp__blk949_dn8 = assign48340_e62025_d_n8;
        locals.var_temp__blk949_dn9 = assign48340_e62025_d_n9;

        let (assign48350_e62046, assign48350_e62046_d_n4, assign48350_e62046_d_n6, assign48350_e62046_d_n7, assign48350_e62046_d_n8, assign48350_e62046_d_n9,) = {
    if (((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) && (locals.var_guard1279 != 0.0)) {
        let assign48350_e62033: f64 = (0.5 * locals.var_imaxii_i);
        let assign48350_e62039: f64 = (locals.var_temp__blk949 * locals.var_temp__blk949);
        let assign48350_e62040: f64 = (1.0 + assign48350_e62039);
        let assign48350_e62041: f64 = (assign48350_e62040).sqrt();
        let assign48350_e62042: f64 = (locals.var_temp__blk949 / assign48350_e62041);
        let assign48350_e62043: f64 = (1.0 + assign48350_e62042);
        let assign48350_e62044: f64 = (assign48350_e62033 * assign48350_e62043);
        (assign48350_e62044, (assign48350_e62033 * (((locals.var_temp__blk949_dn4 * assign48350_e62041) - (locals.var_temp__blk949 * (((locals.var_temp__blk949_dn4 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn4)) / (2.0 * assign48350_e62041)))) / (assign48350_e62041 * assign48350_e62041))), (assign48350_e62033 * (((locals.var_temp__blk949_dn6 * assign48350_e62041) - (locals.var_temp__blk949 * (((locals.var_temp__blk949_dn6 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn6)) / (2.0 * assign48350_e62041)))) / (assign48350_e62041 * assign48350_e62041))), (assign48350_e62033 * (((locals.var_temp__blk949_dn7 * assign48350_e62041) - (locals.var_temp__blk949 * (((locals.var_temp__blk949_dn7 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn7)) / (2.0 * assign48350_e62041)))) / (assign48350_e62041 * assign48350_e62041))), (assign48350_e62033 * (((locals.var_temp__blk949_dn8 * assign48350_e62041) - (locals.var_temp__blk949 * (((locals.var_temp__blk949_dn8 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn8)) / (2.0 * assign48350_e62041)))) / (assign48350_e62041 * assign48350_e62041))), (assign48350_e62033 * (((locals.var_temp__blk949_dn9 * assign48350_e62041) - (locals.var_temp__blk949 * (((locals.var_temp__blk949_dn9 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn9)) / (2.0 * assign48350_e62041)))) / (assign48350_e62041 * assign48350_e62041))),)
    } else {
        (locals.var_iimpact, locals.var_iimpact_dn4, locals.var_iimpact_dn6, locals.var_iimpact_dn7, locals.var_iimpact_dn8, locals.var_iimpact_dn9,)
    }
};
        locals.var_iimpact = assign48350_e62046;
        locals.var_iimpact_dn4 = assign48350_e62046_d_n4;
        locals.var_iimpact_dn6 = assign48350_e62046_d_n6;
        locals.var_iimpact_dn7 = assign48350_e62046_d_n7;
        locals.var_iimpact_dn8 = assign48350_e62046_d_n8;
        locals.var_iimpact_dn9 = assign48350_e62046_d_n9;

        let assign48360_e62057: f64 = if (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1473 = assign48360_e62057;

        let assign48370_e62064: f64 = if ((p.p45 > 0.0) || (p.p47 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1474 = assign48370_e62064;

        let (assign48380_e62070, assign48380_e62070_d_n4,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (locals.var_phib_dc, locals.var_phib_dc_dn4,)
    } else {
        (locals.var_phib__blk1314, locals.var_phib__blk1314_dn4,)
    }
};
        locals.var_phib__blk1314 = assign48380_e62070;
        locals.var_phib__blk1314_dn4 = assign48380_e62070_d_n4;

        let (assign48390_e62076, assign48390_e62076_d_n4,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (locals.var_aphi_dc, locals.var_aphi_dc_dn4,)
    } else {
        (locals.var_aphi__blk1315, locals.var_aphi__blk1315_dn4,)
    }
};
        locals.var_aphi__blk1315 = assign48390_e62076;
        locals.var_aphi__blk1315_dn4 = assign48390_e62076_d_n4;

        let (assign48400_e62082, assign48400_e62082_d_n4,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (locals.var_g_0_dc, locals.var_g_0_dc_dn4,)
    } else {
        (locals.var_g_0__blk1316, locals.var_g_0__blk1316_dn4,)
    }
};
        locals.var_g_0__blk1316 = assign48400_e62082;
        locals.var_g_0__blk1316_dn4 = assign48400_e62082_d_n4;

        let (assign48410_e62088, assign48410_e62088_d_n4, assign48410_e62088_d_n7, assign48410_e62088_d_n8, assign48410_e62088_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (locals.var_v_xb_dc_tmp, locals.var_v_xb_dc_tmp_dn4, locals.var_v_xb_dc_tmp_dn7, locals.var_v_xb_dc_tmp_dn8, locals.var_v_xb_dc_tmp_dn9,)
    } else {
        (locals.var_v_xb__blk1317, locals.var_v_xb__blk1317_dn4, locals.var_v_xb__blk1317_dn7, locals.var_v_xb__blk1317_dn8, locals.var_v_xb__blk1317_dn9,)
    }
};
        locals.var_v_xb__blk1317 = assign48410_e62088;
        locals.var_v_xb__blk1317_dn4 = assign48410_e62088_d_n4;
        locals.var_v_xb__blk1317_dn7 = assign48410_e62088_d_n7;
        locals.var_v_xb__blk1317_dn8 = assign48410_e62088_d_n8;
        locals.var_v_xb__blk1317_dn9 = assign48410_e62088_d_n9;

        let (assign48420_e62094, assign48420_e62094_d_n4, assign48420_e62094_d_n6, assign48420_e62094_d_n7, assign48420_e62094_d_n8, assign48420_e62094_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (locals.var_vsbstar_dc_tmp, locals.var_vsbstar_dc_tmp_dn4, locals.var_vsbstar_dc_tmp_dn6, locals.var_vsbstar_dc_tmp_dn7, locals.var_vsbstar_dc_tmp_dn8, locals.var_vsbstar_dc_tmp_dn9,)
    } else {
        (locals.var_vsbstar__blk1318, locals.var_vsbstar__blk1318_dn4, locals.var_vsbstar__blk1318_dn6, locals.var_vsbstar__blk1318_dn7, locals.var_vsbstar__blk1318_dn8, locals.var_vsbstar__blk1318_dn9,)
    }
};
        locals.var_vsbstar__blk1318 = assign48420_e62094;
        locals.var_vsbstar__blk1318_dn4 = assign48420_e62094_d_n4;
        locals.var_vsbstar__blk1318_dn6 = assign48420_e62094_d_n6;
        locals.var_vsbstar__blk1318_dn7 = assign48420_e62094_d_n7;
        locals.var_vsbstar__blk1318_dn8 = assign48420_e62094_d_n8;
        locals.var_vsbstar__blk1318_dn9 = assign48420_e62094_d_n9;

        let (assign48430_e62100,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_dvbstar__blk1322,)
    }
};
        locals.var_dvbstar__blk1322 = assign48430_e62100;

        let assign48440_e62103: f64 = if p.p47 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1475 = assign48440_e62103;

        let (assign48450_e62128, assign48450_e62128_d_n4, assign48450_e62128_d_n7, assign48450_e62128_d_n8, assign48450_e62128_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        let assign48450_e62112: f64 = (locals.var_v_db + locals.var_v_sb);
        let assign48450_e62115: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign48450_e62118: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign48450_e62119: f64 = (assign48450_e62115 * assign48450_e62118);
        let assign48450_e62121: f64 = (assign48450_e62119 + locals.var_bphi_ac);
        let assign48450_e62122: f64 = (assign48450_e62121).sqrt();
        let assign48450_e62123: f64 = (assign48450_e62112 - assign48450_e62122);
        let assign48450_e62124: f64 = (0.5 * assign48450_e62123);
        let assign48450_e62126: f64 = (assign48450_e62124 + locals.var_phix_ac);
        (assign48450_e62126, ((0.5 * (-(locals.var_bphi_ac_dn4 / (2.0 * assign48450_e62122)))) + locals.var_phix_ac_dn4), (0.5 * ((locals.var_v_db_dn7 + locals.var_v_sb_dn7) - ((((locals.var_v_db_dn7 - locals.var_v_sb_dn7) * assign48450_e62118) + (assign48450_e62115 * (locals.var_v_db_dn7 - locals.var_v_sb_dn7))) / (2.0 * assign48450_e62122)))), (0.5 * ((locals.var_v_db_dn8 + locals.var_v_sb_dn8) - ((((locals.var_v_db_dn8 - locals.var_v_sb_dn8) * assign48450_e62118) + (assign48450_e62115 * (locals.var_v_db_dn8 - locals.var_v_sb_dn8))) / (2.0 * assign48450_e62122)))), (0.5 * ((locals.var_v_db_dn9 + locals.var_v_sb_dn9) - ((((locals.var_v_db_dn9 - locals.var_v_sb_dn9) * assign48450_e62118) + (assign48450_e62115 * (locals.var_v_db_dn9 - locals.var_v_sb_dn9))) / (2.0 * assign48450_e62122)))),)
    } else {
        (locals.var_v_xb__blk1317, locals.var_v_xb__blk1317_dn4, locals.var_v_xb__blk1317_dn7, locals.var_v_xb__blk1317_dn8, locals.var_v_xb__blk1317_dn9,)
    }
};
        locals.var_v_xb__blk1317 = assign48450_e62128;
        locals.var_v_xb__blk1317_dn4 = assign48450_e62128_d_n4;
        locals.var_v_xb__blk1317_dn7 = assign48450_e62128_d_n7;
        locals.var_v_xb__blk1317_dn8 = assign48450_e62128_d_n8;
        locals.var_v_xb__blk1317_dn9 = assign48450_e62128_d_n9;

        let (assign48460_e62155, assign48460_e62155_d_n4, assign48460_e62155_d_n7, assign48460_e62155_d_n8, assign48460_e62155_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        let assign48460_e62138: f64 = locals.var_v_xb__blk1317;
        let assign48460_e62141: f64 = locals.var_v_xb__blk1317;
        let assign48460_e62144: f64 = locals.var_v_xb__blk1317;
        let assign48460_e62145: f64 = (assign48460_e62141 * assign48460_e62144);
        let assign48460_e62147: f64 = (assign48460_e62145 + locals.var_aphi_ac);
        let assign48460_e62148: f64 = (assign48460_e62147).sqrt();
        let assign48460_e62149: f64 = (assign48460_e62138 - assign48460_e62148);
        let assign48460_e62150: f64 = (0.5 * assign48460_e62149);
        let assign48460_e62151: f64 = (locals.var_v_sb - assign48460_e62150);
        let assign48460_e62153: f64 = (assign48460_e62151 + locals.var_phix1_ac);
        (assign48460_e62153, ((-(0.5 * (locals.var_v_xb__blk1317_dn4 - ((((locals.var_v_xb__blk1317_dn4 * assign48460_e62144) + (assign48460_e62141 * locals.var_v_xb__blk1317_dn4)) + locals.var_aphi_ac_dn4) / (2.0 * assign48460_e62148))))) + locals.var_phix1_ac_dn4), (locals.var_v_sb_dn7 - (0.5 * (locals.var_v_xb__blk1317_dn7 - (((locals.var_v_xb__blk1317_dn7 * assign48460_e62144) + (assign48460_e62141 * locals.var_v_xb__blk1317_dn7)) / (2.0 * assign48460_e62148))))), (locals.var_v_sb_dn8 - (0.5 * (locals.var_v_xb__blk1317_dn8 - (((locals.var_v_xb__blk1317_dn8 * assign48460_e62144) + (assign48460_e62141 * locals.var_v_xb__blk1317_dn8)) / (2.0 * assign48460_e62148))))), (locals.var_v_sb_dn9 - (0.5 * (locals.var_v_xb__blk1317_dn9 - (((locals.var_v_xb__blk1317_dn9 * assign48460_e62144) + (assign48460_e62141 * locals.var_v_xb__blk1317_dn9)) / (2.0 * assign48460_e62148))))),)
    } else {
        (locals.var_vsbstar_ac, locals.var_vsbstar_ac_dn4, locals.var_vsbstar_ac_dn7, locals.var_vsbstar_ac_dn8, locals.var_vsbstar_ac_dn9,)
    }
};
        locals.var_vsbstar_ac = assign48460_e62155;
        locals.var_vsbstar_ac_dn4 = assign48460_e62155_d_n4;
        locals.var_vsbstar_ac_dn7 = assign48460_e62155_d_n7;
        locals.var_vsbstar_ac_dn8 = assign48460_e62155_d_n8;
        locals.var_vsbstar_ac_dn9 = assign48460_e62155_d_n9;

        let (assign48470_e62163, assign48470_e62163_d_n4, assign48470_e62163_d_n6, assign48470_e62163_d_n7, assign48470_e62163_d_n8, assign48470_e62163_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        (locals.var_vsbstar_ac, locals.var_vsbstar_ac_dn4, 0.0, locals.var_vsbstar_ac_dn7, locals.var_vsbstar_ac_dn8, locals.var_vsbstar_ac_dn9,)
    } else {
        (locals.var_vsbstar__blk1318, locals.var_vsbstar__blk1318_dn4, locals.var_vsbstar__blk1318_dn6, locals.var_vsbstar__blk1318_dn7, locals.var_vsbstar__blk1318_dn8, locals.var_vsbstar__blk1318_dn9,)
    }
};
        locals.var_vsbstar__blk1318 = assign48470_e62163;
        locals.var_vsbstar__blk1318_dn4 = assign48470_e62163_d_n4;
        locals.var_vsbstar__blk1318_dn6 = assign48470_e62163_d_n6;
        locals.var_vsbstar__blk1318_dn7 = assign48470_e62163_d_n7;
        locals.var_vsbstar__blk1318_dn8 = assign48470_e62163_d_n8;
        locals.var_vsbstar__blk1318_dn9 = assign48470_e62163_d_n9;

        let (assign48480_e62171, assign48480_e62171_d_n4,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        (locals.var_phib_ac, locals.var_phib_ac_dn4,)
    } else {
        (locals.var_phib__blk1314, locals.var_phib__blk1314_dn4,)
    }
};
        locals.var_phib__blk1314 = assign48480_e62171;
        locals.var_phib__blk1314_dn4 = assign48480_e62171_d_n4;

        let (assign48490_e62179, assign48490_e62179_d_n4,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        (locals.var_aphi_ac, locals.var_aphi_ac_dn4,)
    } else {
        (locals.var_aphi__blk1315, locals.var_aphi__blk1315_dn4,)
    }
};
        locals.var_aphi__blk1315 = assign48490_e62179;
        locals.var_aphi__blk1315_dn4 = assign48490_e62179_d_n4;

        let (assign48500_e62187, assign48500_e62187_d_n4,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        (locals.var_g_0_ac, locals.var_g_0_ac_dn4,)
    } else {
        (locals.var_g_0__blk1316, locals.var_g_0__blk1316_dn4,)
    }
};
        locals.var_g_0__blk1316 = assign48500_e62187;
        locals.var_g_0__blk1316_dn4 = assign48500_e62187_d_n4;

        let (assign48510_e62197, assign48510_e62197_d_n4, assign48510_e62197_d_n6, assign48510_e62197_d_n7, assign48510_e62197_d_n8, assign48510_e62197_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48510_e62193: f64 = (locals.var_vgb - locals.var_dvbstar__blk1322);
        let assign48510_e62195: f64 = (assign48510_e62193 - locals.var_vfb_t);
        (assign48510_e62195, (-locals.var_vfb_t_dn4), locals.var_vgb_dn6, locals.var_vgb_dn7, locals.var_vgb_dn8, locals.var_vgb_dn9,)
    } else {
        (locals.var_vgb1__blk1321, locals.var_vgb1__blk1321_dn4, locals.var_vgb1__blk1321_dn6, locals.var_vgb1__blk1321_dn7, locals.var_vgb1__blk1321_dn8, locals.var_vgb1__blk1321_dn9,)
    }
};
        locals.var_vgb1__blk1321 = assign48510_e62197;
        locals.var_vgb1__blk1321_dn4 = assign48510_e62197_d_n4;
        locals.var_vgb1__blk1321_dn6 = assign48510_e62197_d_n6;
        locals.var_vgb1__blk1321_dn7 = assign48510_e62197_d_n7;
        locals.var_vgb1__blk1321_dn8 = assign48510_e62197_d_n8;
        locals.var_vgb1__blk1321_dn9 = assign48510_e62197_d_n9;

        let (assign48520_e62209, assign48520_e62209_d_n4, assign48520_e62209_d_n6, assign48520_e62209_d_n7, assign48520_e62209_d_n8, assign48520_e62209_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48520_e62205: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign48520_e62206: f64 = (0.5 * assign48520_e62205);
        let assign48520_e62207: f64 = (locals.var_vsbstar__blk1318 + assign48520_e62206);
        (assign48520_e62207, locals.var_vsbstar__blk1318_dn4, locals.var_vsbstar__blk1318_dn6, (locals.var_vsbstar__blk1318_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), (locals.var_vsbstar__blk1318_dn8 + (0.5 * (locals.var_v_ds_dn8 - locals.var_vdsx_dn8))), locals.var_vsbstar__blk1318_dn9,)
    } else {
        (locals.var_vsbx__blk1323, locals.var_vsbx__blk1323_dn4, locals.var_vsbx__blk1323_dn6, locals.var_vsbx__blk1323_dn7, locals.var_vsbx__blk1323_dn8, locals.var_vsbx__blk1323_dn9,)
    }
};
        locals.var_vsbx__blk1323 = assign48520_e62209;
        locals.var_vsbx__blk1323_dn4 = assign48520_e62209_d_n4;
        locals.var_vsbx__blk1323_dn6 = assign48520_e62209_d_n6;
        locals.var_vsbx__blk1323_dn7 = assign48520_e62209_d_n7;
        locals.var_vsbx__blk1323_dn8 = assign48520_e62209_d_n8;
        locals.var_vsbx__blk1323_dn9 = assign48520_e62209_d_n9;

        let (assign48530_e62215, assign48530_e62215_d_n4, assign48530_e62215_d_n6, assign48530_e62215_d_n7, assign48530_e62215_d_n8, assign48530_e62215_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dctg__blk1335, locals.var_dctg__blk1335_dn4, locals.var_dctg__blk1335_dn6, locals.var_dctg__blk1335_dn7, locals.var_dctg__blk1335_dn8, locals.var_dctg__blk1335_dn9,)
    }
};
        locals.var_dctg__blk1335 = assign48530_e62215;
        locals.var_dctg__blk1335_dn4 = assign48530_e62215_d_n4;
        locals.var_dctg__blk1335_dn6 = assign48530_e62215_d_n6;
        locals.var_dctg__blk1335_dn7 = assign48530_e62215_d_n7;
        locals.var_dctg__blk1335_dn8 = assign48530_e62215_d_n8;
        locals.var_dctg__blk1335_dn9 = assign48530_e62215_d_n9;

        let assign48540_e62218: f64 = if locals.var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1476 = assign48540_e62218;

        let (assign48550_e62228, assign48550_e62228_d_n4,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48550_e62226: f64 = (locals.var_phib__blk1314 * locals.var_inv_phit);
        (assign48550_e62226, ((locals.var_phib__blk1314_dn4 * locals.var_inv_phit) + (locals.var_phib__blk1314 * locals.var_inv_phit_dn4)),)
    } else {
        (locals.var_xbct__blk1326, locals.var_xbct__blk1326_dn4,)
    }
};
        locals.var_xbct__blk1326 = assign48550_e62228;
        locals.var_xbct__blk1326_dn4 = assign48550_e62228_d_n4;

        let (assign48560_e62238, assign48560_e62238_d_n4, assign48560_e62238_d_n6, assign48560_e62238_d_n7, assign48560_e62238_d_n8, assign48560_e62238_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48560_e62236: f64 = (locals.var_vsbx__blk1323 * locals.var_inv_phit);
        (assign48560_e62236, ((locals.var_vsbx__blk1323_dn4 * locals.var_inv_phit) + (locals.var_vsbx__blk1323 * locals.var_inv_phit_dn4)), (locals.var_vsbx__blk1323_dn6 * locals.var_inv_phit), (locals.var_vsbx__blk1323_dn7 * locals.var_inv_phit), (locals.var_vsbx__blk1323_dn8 * locals.var_inv_phit), (locals.var_vsbx__blk1323_dn9 * locals.var_inv_phit),)
    } else {
        (locals.var_xsbstar__blk1327, locals.var_xsbstar__blk1327_dn4, locals.var_xsbstar__blk1327_dn6, locals.var_xsbstar__blk1327_dn7, locals.var_xsbstar__blk1327_dn8, locals.var_xsbstar__blk1327_dn9,)
    }
};
        locals.var_xsbstar__blk1327 = assign48560_e62238;
        locals.var_xsbstar__blk1327_dn4 = assign48560_e62238_d_n4;
        locals.var_xsbstar__blk1327_dn6 = assign48560_e62238_d_n6;
        locals.var_xsbstar__blk1327_dn7 = assign48560_e62238_d_n7;
        locals.var_xsbstar__blk1327_dn8 = assign48560_e62238_d_n8;
        locals.var_xsbstar__blk1327_dn9 = assign48560_e62238_d_n9;

        let (assign48570_e62248, assign48570_e62248_d_n4, assign48570_e62248_d_n6, assign48570_e62248_d_n7, assign48570_e62248_d_n8, assign48570_e62248_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48570_e62246: f64 = (locals.var_vgb1__blk1321 * locals.var_inv_phit);
        (assign48570_e62246, ((locals.var_vgb1__blk1321_dn4 * locals.var_inv_phit) + (locals.var_vgb1__blk1321 * locals.var_inv_phit_dn4)), (locals.var_vgb1__blk1321_dn6 * locals.var_inv_phit), (locals.var_vgb1__blk1321_dn7 * locals.var_inv_phit), (locals.var_vgb1__blk1321_dn8 * locals.var_inv_phit), (locals.var_vgb1__blk1321_dn9 * locals.var_inv_phit),)
    } else {
        (locals.var_xgct__blk1328, locals.var_xgct__blk1328_dn4, locals.var_xgct__blk1328_dn6, locals.var_xgct__blk1328_dn7, locals.var_xgct__blk1328_dn8, locals.var_xgct__blk1328_dn9,)
    }
};
        locals.var_xgct__blk1328 = assign48570_e62248;
        locals.var_xgct__blk1328_dn4 = assign48570_e62248_d_n4;
        locals.var_xgct__blk1328_dn6 = assign48570_e62248_d_n6;
        locals.var_xgct__blk1328_dn7 = assign48570_e62248_d_n7;
        locals.var_xgct__blk1328_dn8 = assign48570_e62248_d_n8;
        locals.var_xgct__blk1328_dn9 = assign48570_e62248_d_n9;

        let (assign48580_e62263, assign48580_e62263_d_n4, assign48580_e62263_d_n6, assign48580_e62263_d_n7, assign48580_e62263_d_n8, assign48580_e62263_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48580_e62257: f64 = (0.5 * locals.var_g_0__blk1316);
        let assign48580_e62259: f64 = (locals.var_xbct__blk1326).sqrt();
        let assign48580_e62260: f64 = (assign48580_e62257 / assign48580_e62259);
        let assign48580_e62261: f64 = (1.0 + assign48580_e62260);
        (assign48580_e62261, ((((0.5 * locals.var_g_0__blk1316_dn4) * assign48580_e62259) - (assign48580_e62257 * (locals.var_xbct__blk1326_dn4 / (2.0 * assign48580_e62259)))) / (assign48580_e62259 * assign48580_e62259)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign48580_e62263;
        locals.var_temp1_dn4 = assign48580_e62263_d_n4;
        locals.var_temp1_dn6 = assign48580_e62263_d_n6;
        locals.var_temp1_dn7 = assign48580_e62263_d_n7;
        locals.var_temp1_dn8 = assign48580_e62263_d_n8;
        locals.var_temp1_dn9 = assign48580_e62263_d_n9;

        let (assign48590_e62276, assign48590_e62276_d_n4, assign48590_e62276_d_n6, assign48590_e62276_d_n7, assign48590_e62276_d_n8, assign48590_e62276_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48590_e62272: f64 = (locals.var_xbct__blk1326).sqrt();
        let assign48590_e62273: f64 = (locals.var_g_0__blk1316 * assign48590_e62272);
        let assign48590_e62274: f64 = (locals.var_xbct__blk1326 + assign48590_e62273);
        (assign48590_e62274, (locals.var_xbct__blk1326_dn4 + ((locals.var_g_0__blk1316_dn4 * assign48590_e62272) + (locals.var_g_0__blk1316 * (locals.var_xbct__blk1326_dn4 / (2.0 * assign48590_e62272))))), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign48590_e62276;
        locals.var_temp2_dn4 = assign48590_e62276_d_n4;
        locals.var_temp2_dn6 = assign48590_e62276_d_n6;
        locals.var_temp2_dn7 = assign48590_e62276_d_n7;
        locals.var_temp2_dn8 = assign48590_e62276_d_n8;
        locals.var_temp2_dn9 = assign48590_e62276_d_n9;

        let (assign48600_e62298, assign48600_e62298_d_n4, assign48600_e62298_d_n6, assign48600_e62298_d_n7, assign48600_e62298_d_n8, assign48600_e62298_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48600_e62284: f64 = (locals.var_xgct__blk1328 - locals.var_temp2);
        let assign48600_e62286: f64 = (assign48600_e62284 / locals.var_temp1);
        let assign48600_e62289: f64 = (0.5 * locals.var_xbct__blk1326);
        let assign48600_e62290: f64 = (assign48600_e62286 + assign48600_e62289);
        let assign48600_e62293: f64 = (1.0 + locals.var_ctb_i);
        let assign48600_e62295: f64 = (assign48600_e62293 * locals.var_xsbstar__blk1327);
        let assign48600_e62296: f64 = (assign48600_e62290 - assign48600_e62295);
        (assign48600_e62296, ((((((locals.var_xgct__blk1328_dn4 - locals.var_temp2_dn4) * locals.var_temp1) - (assign48600_e62284 * locals.var_temp1_dn4)) / (locals.var_temp1 * locals.var_temp1)) + (0.5 * locals.var_xbct__blk1326_dn4)) - (assign48600_e62293 * locals.var_xsbstar__blk1327_dn4)), (((((locals.var_xgct__blk1328_dn6 - locals.var_temp2_dn6) * locals.var_temp1) - (assign48600_e62284 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)) - (assign48600_e62293 * locals.var_xsbstar__blk1327_dn6)), (((((locals.var_xgct__blk1328_dn7 - locals.var_temp2_dn7) * locals.var_temp1) - (assign48600_e62284 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)) - (assign48600_e62293 * locals.var_xsbstar__blk1327_dn7)), (((((locals.var_xgct__blk1328_dn8 - locals.var_temp2_dn8) * locals.var_temp1) - (assign48600_e62284 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)) - (assign48600_e62293 * locals.var_xsbstar__blk1327_dn8)), (((((locals.var_xgct__blk1328_dn9 - locals.var_temp2_dn9) * locals.var_temp1) - (assign48600_e62284 * locals.var_temp1_dn9)) / (locals.var_temp1 * locals.var_temp1)) - (assign48600_e62293 * locals.var_xsbstar__blk1327_dn9)),)
    } else {
        (locals.var_xwict__blk1329, locals.var_xwict__blk1329_dn4, locals.var_xwict__blk1329_dn6, locals.var_xwict__blk1329_dn7, locals.var_xwict__blk1329_dn8, locals.var_xwict__blk1329_dn9,)
    }
};
        locals.var_xwict__blk1329 = assign48600_e62298;
        locals.var_xwict__blk1329_dn4 = assign48600_e62298_d_n4;
        locals.var_xwict__blk1329_dn6 = assign48600_e62298_d_n6;
        locals.var_xwict__blk1329_dn7 = assign48600_e62298_d_n7;
        locals.var_xwict__blk1329_dn8 = assign48600_e62298_d_n8;
        locals.var_xwict__blk1329_dn9 = assign48600_e62298_d_n9;

        let (assign48610_e62310, assign48610_e62310_d_n4,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48610_e62306: f64 = (0.5 * locals.var_xbct__blk1326);
        let assign48610_e62308: f64 = (assign48610_e62306 + 2.0);
        (assign48610_e62308, (0.5 * locals.var_xbct__blk1326_dn4),)
    } else {
        (locals.var_xctmax__blk1330, locals.var_xctmax__blk1330_dn4,)
    }
};
        locals.var_xctmax__blk1330 = assign48610_e62310;
        locals.var_xctmax__blk1330_dn4 = assign48610_e62310_d_n4;

        let (assign48620_e62320, assign48620_e62320_d_n4, assign48620_e62320_d_n6, assign48620_e62320_d_n7, assign48620_e62320_d_n8, assign48620_e62320_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48620_e62318: f64 = (locals.var_xbct__blk1326 + locals.var_xsbstar__blk1327);
        (assign48620_e62318, (locals.var_xbct__blk1326_dn4 + locals.var_xsbstar__blk1327_dn4), locals.var_xsbstar__blk1327_dn6, locals.var_xsbstar__blk1327_dn7, locals.var_xsbstar__blk1327_dn8, locals.var_xsbstar__blk1327_dn9,)
    } else {
        (locals.var_xnct__blk1331, locals.var_xnct__blk1331_dn4, locals.var_xnct__blk1331_dn6, locals.var_xnct__blk1331_dn7, locals.var_xnct__blk1331_dn8, locals.var_xnct__blk1331_dn9,)
    }
};
        locals.var_xnct__blk1331 = assign48620_e62320;
        locals.var_xnct__blk1331_dn4 = assign48620_e62320_d_n4;
        locals.var_xnct__blk1331_dn6 = assign48620_e62320_d_n6;
        locals.var_xnct__blk1331_dn7 = assign48620_e62320_d_n7;
        locals.var_xnct__blk1331_dn8 = assign48620_e62320_d_n8;
        locals.var_xnct__blk1331_dn9 = assign48620_e62320_d_n9;

        let (assign48630_e62345, assign48630_e62345_d_n4, assign48630_e62345_d_n6, assign48630_e62345_d_n7, assign48630_e62345_d_n8, assign48630_e62345_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48630_e62328: f64 = (locals.var_xgct__blk1328 - locals.var_xnct__blk1331);
        let assign48630_e62331: f64 = (locals.var_xnct__blk1331).sqrt();
        let assign48630_e62332: f64 = (locals.var_g_0__blk1316 * assign48630_e62331);
        let assign48630_e62333: f64 = (assign48630_e62328 - assign48630_e62332);
        let assign48630_e62337: f64 = (locals.var_xbct__blk1326 / locals.var_g_0__blk1316);
        let assign48630_e62339: f64 = (locals.var_xbct__blk1326).sqrt();
        let assign48630_e62340: f64 = (assign48630_e62337 + assign48630_e62339);
        let assign48630_e62341: f64 = (assign48630_e62340).ln();
        let assign48630_e62342: f64 = (2.0 * assign48630_e62341);
        let assign48630_e62343: f64 = (assign48630_e62333 - assign48630_e62342);
        (assign48630_e62343, (((locals.var_xgct__blk1328_dn4 - locals.var_xnct__blk1331_dn4) - ((locals.var_g_0__blk1316_dn4 * assign48630_e62331) + (locals.var_g_0__blk1316 * (locals.var_xnct__blk1331_dn4 / (2.0 * assign48630_e62331))))) - (2.0 * (((((locals.var_xbct__blk1326_dn4 * locals.var_g_0__blk1316) - (locals.var_xbct__blk1326 * locals.var_g_0__blk1316_dn4)) / (locals.var_g_0__blk1316 * locals.var_g_0__blk1316)) + (locals.var_xbct__blk1326_dn4 / (2.0 * assign48630_e62339))) / assign48630_e62340))), ((locals.var_xgct__blk1328_dn6 - locals.var_xnct__blk1331_dn6) - (locals.var_g_0__blk1316 * (locals.var_xnct__blk1331_dn6 / (2.0 * assign48630_e62331)))), ((locals.var_xgct__blk1328_dn7 - locals.var_xnct__blk1331_dn7) - (locals.var_g_0__blk1316 * (locals.var_xnct__blk1331_dn7 / (2.0 * assign48630_e62331)))), ((locals.var_xgct__blk1328_dn8 - locals.var_xnct__blk1331_dn8) - (locals.var_g_0__blk1316 * (locals.var_xnct__blk1331_dn8 / (2.0 * assign48630_e62331)))), ((locals.var_xgct__blk1328_dn9 - locals.var_xnct__blk1331_dn9) - (locals.var_g_0__blk1316 * (locals.var_xnct__blk1331_dn9 / (2.0 * assign48630_e62331)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign48630_e62345;
        locals.var_temp1_dn4 = assign48630_e62345_d_n4;
        locals.var_temp1_dn6 = assign48630_e62345_d_n6;
        locals.var_temp1_dn7 = assign48630_e62345_d_n7;
        locals.var_temp1_dn8 = assign48630_e62345_d_n8;
        locals.var_temp1_dn9 = assign48630_e62345_d_n9;

        let (assign48640_e62357, assign48640_e62357_d_n4, assign48640_e62357_d_n6, assign48640_e62357_d_n7, assign48640_e62357_d_n8, assign48640_e62357_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48640_e62353: f64 = (2.0 * locals.var_temp1);
        let assign48640_e62355: f64 = (assign48640_e62353 + locals.var_xctmax__blk1330);
        (assign48640_e62355, ((2.0 * locals.var_temp1_dn4) + locals.var_xctmax__blk1330_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9),)
    } else {
        (locals.var_xmict__blk1332, locals.var_xmict__blk1332_dn4, locals.var_xmict__blk1332_dn6, locals.var_xmict__blk1332_dn7, locals.var_xmict__blk1332_dn8, locals.var_xmict__blk1332_dn9,)
    }
};
        locals.var_xmict__blk1332 = assign48640_e62357;
        locals.var_xmict__blk1332_dn4 = assign48640_e62357_d_n4;
        locals.var_xmict__blk1332_dn6 = assign48640_e62357_d_n6;
        locals.var_xmict__blk1332_dn7 = assign48640_e62357_d_n7;
        locals.var_xmict__blk1332_dn8 = assign48640_e62357_d_n8;
        locals.var_xmict__blk1332_dn9 = assign48640_e62357_d_n9;

    }

    pub(super) fn stamp_transient_block_36(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48650_e62380, assign48650_e62380_d_n4, assign48650_e62380_d_n6, assign48650_e62380_d_n7, assign48650_e62380_d_n8, assign48650_e62380_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48650_e62366: f64 = (locals.var_xwict__blk1329 + locals.var_xmict__blk1332);
        let assign48650_e62369: f64 = (locals.var_xwict__blk1329 - locals.var_xmict__blk1332);
        let assign48650_e62372: f64 = (locals.var_xwict__blk1329 - locals.var_xmict__blk1332);
        let assign48650_e62373: f64 = (assign48650_e62369 * assign48650_e62372);
        let assign48650_e62375: f64 = (assign48650_e62373 + 20.0);
        let assign48650_e62376: f64 = (assign48650_e62375).sqrt();
        let assign48650_e62377: f64 = (assign48650_e62366 + assign48650_e62376);
        let assign48650_e62378: f64 = (0.5 * assign48650_e62377);
        (assign48650_e62378, (0.5 * ((locals.var_xwict__blk1329_dn4 + locals.var_xmict__blk1332_dn4) + ((((locals.var_xwict__blk1329_dn4 - locals.var_xmict__blk1332_dn4) * assign48650_e62372) + (assign48650_e62369 * (locals.var_xwict__blk1329_dn4 - locals.var_xmict__blk1332_dn4))) / (2.0 * assign48650_e62376)))), (0.5 * ((locals.var_xwict__blk1329_dn6 + locals.var_xmict__blk1332_dn6) + ((((locals.var_xwict__blk1329_dn6 - locals.var_xmict__blk1332_dn6) * assign48650_e62372) + (assign48650_e62369 * (locals.var_xwict__blk1329_dn6 - locals.var_xmict__blk1332_dn6))) / (2.0 * assign48650_e62376)))), (0.5 * ((locals.var_xwict__blk1329_dn7 + locals.var_xmict__blk1332_dn7) + ((((locals.var_xwict__blk1329_dn7 - locals.var_xmict__blk1332_dn7) * assign48650_e62372) + (assign48650_e62369 * (locals.var_xwict__blk1329_dn7 - locals.var_xmict__blk1332_dn7))) / (2.0 * assign48650_e62376)))), (0.5 * ((locals.var_xwict__blk1329_dn8 + locals.var_xmict__blk1332_dn8) + ((((locals.var_xwict__blk1329_dn8 - locals.var_xmict__blk1332_dn8) * assign48650_e62372) + (assign48650_e62369 * (locals.var_xwict__blk1329_dn8 - locals.var_xmict__blk1332_dn8))) / (2.0 * assign48650_e62376)))), (0.5 * ((locals.var_xwict__blk1329_dn9 + locals.var_xmict__blk1332_dn9) + ((((locals.var_xwict__blk1329_dn9 - locals.var_xmict__blk1332_dn9) * assign48650_e62372) + (assign48650_e62369 * (locals.var_xwict__blk1329_dn9 - locals.var_xmict__blk1332_dn9))) / (2.0 * assign48650_e62376)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign48650_e62380;
        locals.var_temp1_dn4 = assign48650_e62380_d_n4;
        locals.var_temp1_dn6 = assign48650_e62380_d_n6;
        locals.var_temp1_dn7 = assign48650_e62380_d_n7;
        locals.var_temp1_dn8 = assign48650_e62380_d_n8;
        locals.var_temp1_dn9 = assign48650_e62380_d_n9;

        let (assign48660_e62394, assign48660_e62394_d_n4, assign48660_e62394_d_n6, assign48660_e62394_d_n7, assign48660_e62394_d_n8, assign48660_e62394_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48660_e62389: f64 = (locals.var_xgct__blk1328 - locals.var_xsbstar__blk1327);
        let assign48660_e62390: f64 = (2.0 * assign48660_e62389);
        let assign48660_e62392: f64 = (assign48660_e62390 - locals.var_xctmax__blk1330);
        (assign48660_e62392, ((2.0 * (locals.var_xgct__blk1328_dn4 - locals.var_xsbstar__blk1327_dn4)) - locals.var_xctmax__blk1330_dn4), (2.0 * (locals.var_xgct__blk1328_dn6 - locals.var_xsbstar__blk1327_dn6)), (2.0 * (locals.var_xgct__blk1328_dn7 - locals.var_xsbstar__blk1327_dn7)), (2.0 * (locals.var_xgct__blk1328_dn8 - locals.var_xsbstar__blk1327_dn8)), (2.0 * (locals.var_xgct__blk1328_dn9 - locals.var_xsbstar__blk1327_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign48660_e62394;
        locals.var_temp2_dn4 = assign48660_e62394_d_n4;
        locals.var_temp2_dn6 = assign48660_e62394_d_n6;
        locals.var_temp2_dn7 = assign48660_e62394_d_n7;
        locals.var_temp2_dn8 = assign48660_e62394_d_n8;
        locals.var_temp2_dn9 = assign48660_e62394_d_n9;

        let (assign48670_e62417, assign48670_e62417_d_n4, assign48670_e62417_d_n6, assign48670_e62417_d_n7, assign48670_e62417_d_n8, assign48670_e62417_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48670_e62403: f64 = (locals.var_temp1 + locals.var_temp2);
        let assign48670_e62406: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign48670_e62409: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign48670_e62410: f64 = (assign48670_e62406 * assign48670_e62409);
        let assign48670_e62412: f64 = (assign48670_e62410 + 20.0);
        let assign48670_e62413: f64 = (assign48670_e62412).sqrt();
        let assign48670_e62414: f64 = (assign48670_e62403 - assign48670_e62413);
        let assign48670_e62415: f64 = (0.5 * assign48670_e62414);
        (assign48670_e62415, (0.5 * ((locals.var_temp1_dn4 + locals.var_temp2_dn4) - ((((locals.var_temp1_dn4 - locals.var_temp2_dn4) * assign48670_e62409) + (assign48670_e62406 * (locals.var_temp1_dn4 - locals.var_temp2_dn4))) / (2.0 * assign48670_e62413)))), (0.5 * ((locals.var_temp1_dn6 + locals.var_temp2_dn6) - ((((locals.var_temp1_dn6 - locals.var_temp2_dn6) * assign48670_e62409) + (assign48670_e62406 * (locals.var_temp1_dn6 - locals.var_temp2_dn6))) / (2.0 * assign48670_e62413)))), (0.5 * ((locals.var_temp1_dn7 + locals.var_temp2_dn7) - ((((locals.var_temp1_dn7 - locals.var_temp2_dn7) * assign48670_e62409) + (assign48670_e62406 * (locals.var_temp1_dn7 - locals.var_temp2_dn7))) / (2.0 * assign48670_e62413)))), (0.5 * ((locals.var_temp1_dn8 + locals.var_temp2_dn8) - ((((locals.var_temp1_dn8 - locals.var_temp2_dn8) * assign48670_e62409) + (assign48670_e62406 * (locals.var_temp1_dn8 - locals.var_temp2_dn8))) / (2.0 * assign48670_e62413)))), (0.5 * ((locals.var_temp1_dn9 + locals.var_temp2_dn9) - ((((locals.var_temp1_dn9 - locals.var_temp2_dn9) * assign48670_e62409) + (assign48670_e62406 * (locals.var_temp1_dn9 - locals.var_temp2_dn9))) / (2.0 * assign48670_e62413)))),)
    } else {
        (locals.var_xsubct__blk1333, locals.var_xsubct__blk1333_dn4, locals.var_xsubct__blk1333_dn6, locals.var_xsubct__blk1333_dn7, locals.var_xsubct__blk1333_dn8, locals.var_xsubct__blk1333_dn9,)
    }
};
        locals.var_xsubct__blk1333 = assign48670_e62417;
        locals.var_xsubct__blk1333_dn4 = assign48670_e62417_d_n4;
        locals.var_xsubct__blk1333_dn6 = assign48670_e62417_d_n6;
        locals.var_xsubct__blk1333_dn7 = assign48670_e62417_d_n7;
        locals.var_xsubct__blk1333_dn8 = assign48670_e62417_d_n8;
        locals.var_xsubct__blk1333_dn9 = assign48670_e62417_d_n9;

        let (assign48680_e62440, assign48680_e62440_d_n4, assign48680_e62440_d_n6, assign48680_e62440_d_n7, assign48680_e62440_d_n8, assign48680_e62440_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48680_e62426: f64 = (locals.var_xsubct__blk1333 + locals.var_xctmax__blk1330);
        let assign48680_e62429: f64 = (locals.var_xsubct__blk1333 - locals.var_xctmax__blk1330);
        let assign48680_e62432: f64 = (locals.var_xsubct__blk1333 - locals.var_xctmax__blk1330);
        let assign48680_e62433: f64 = (assign48680_e62429 * assign48680_e62432);
        let assign48680_e62435: f64 = (assign48680_e62433 + 5.0);
        let assign48680_e62436: f64 = (assign48680_e62435).sqrt();
        let assign48680_e62437: f64 = (assign48680_e62426 - assign48680_e62436);
        let assign48680_e62438: f64 = (0.5 * assign48680_e62437);
        (assign48680_e62438, (0.5 * ((locals.var_xsubct__blk1333_dn4 + locals.var_xctmax__blk1330_dn4) - ((((locals.var_xsubct__blk1333_dn4 - locals.var_xctmax__blk1330_dn4) * assign48680_e62432) + (assign48680_e62429 * (locals.var_xsubct__blk1333_dn4 - locals.var_xctmax__blk1330_dn4))) / (2.0 * assign48680_e62436)))), (0.5 * (locals.var_xsubct__blk1333_dn6 - (((locals.var_xsubct__blk1333_dn6 * assign48680_e62432) + (assign48680_e62429 * locals.var_xsubct__blk1333_dn6)) / (2.0 * assign48680_e62436)))), (0.5 * (locals.var_xsubct__blk1333_dn7 - (((locals.var_xsubct__blk1333_dn7 * assign48680_e62432) + (assign48680_e62429 * locals.var_xsubct__blk1333_dn7)) / (2.0 * assign48680_e62436)))), (0.5 * (locals.var_xsubct__blk1333_dn8 - (((locals.var_xsubct__blk1333_dn8 * assign48680_e62432) + (assign48680_e62429 * locals.var_xsubct__blk1333_dn8)) / (2.0 * assign48680_e62436)))), (0.5 * (locals.var_xsubct__blk1333_dn9 - (((locals.var_xsubct__blk1333_dn9 * assign48680_e62432) + (assign48680_e62429 * locals.var_xsubct__blk1333_dn9)) / (2.0 * assign48680_e62436)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign48680_e62440;
        locals.var_temp1_dn4 = assign48680_e62440_d_n4;
        locals.var_temp1_dn6 = assign48680_e62440_d_n6;
        locals.var_temp1_dn7 = assign48680_e62440_d_n7;
        locals.var_temp1_dn8 = assign48680_e62440_d_n8;
        locals.var_temp1_dn9 = assign48680_e62440_d_n9;

        let (assign48690_e62466, assign48690_e62466_d_n4, assign48690_e62466_d_n6, assign48690_e62466_d_n7, assign48690_e62466_d_n8, assign48690_e62466_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48690_e62449: f64 = (-locals.var_xctmax__blk1330);
        let assign48690_e62450: f64 = (locals.var_temp1 + assign48690_e62449);
        let assign48690_e62453: f64 = (-locals.var_xctmax__blk1330);
        let assign48690_e62454: f64 = (locals.var_temp1 - assign48690_e62453);
        let assign48690_e62457: f64 = (-locals.var_xctmax__blk1330);
        let assign48690_e62458: f64 = (locals.var_temp1 - assign48690_e62457);
        let assign48690_e62459: f64 = (assign48690_e62454 * assign48690_e62458);
        let assign48690_e62461: f64 = (assign48690_e62459 + 20.0);
        let assign48690_e62462: f64 = (assign48690_e62461).sqrt();
        let assign48690_e62463: f64 = (assign48690_e62450 + assign48690_e62462);
        let assign48690_e62464: f64 = (0.5 * assign48690_e62463);
        (assign48690_e62464, (0.5 * ((locals.var_temp1_dn4 + (-locals.var_xctmax__blk1330_dn4)) + ((((locals.var_temp1_dn4 - (-locals.var_xctmax__blk1330_dn4)) * assign48690_e62458) + (assign48690_e62454 * (locals.var_temp1_dn4 - (-locals.var_xctmax__blk1330_dn4)))) / (2.0 * assign48690_e62462)))), (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign48690_e62458) + (assign48690_e62454 * locals.var_temp1_dn6)) / (2.0 * assign48690_e62462)))), (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign48690_e62458) + (assign48690_e62454 * locals.var_temp1_dn7)) / (2.0 * assign48690_e62462)))), (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign48690_e62458) + (assign48690_e62454 * locals.var_temp1_dn8)) / (2.0 * assign48690_e62462)))), (0.5 * (locals.var_temp1_dn9 + (((locals.var_temp1_dn9 * assign48690_e62458) + (assign48690_e62454 * locals.var_temp1_dn9)) / (2.0 * assign48690_e62462)))),)
    } else {
        (locals.var_xct__blk1334, locals.var_xct__blk1334_dn4, locals.var_xct__blk1334_dn6, locals.var_xct__blk1334_dn7, locals.var_xct__blk1334_dn8, locals.var_xct__blk1334_dn9,)
    }
};
        locals.var_xct__blk1334 = assign48690_e62466;
        locals.var_xct__blk1334_dn4 = assign48690_e62466_d_n4;
        locals.var_xct__blk1334_dn6 = assign48690_e62466_d_n6;
        locals.var_xct__blk1334_dn7 = assign48690_e62466_d_n7;
        locals.var_xct__blk1334_dn8 = assign48690_e62466_d_n8;
        locals.var_xct__blk1334_dn9 = assign48690_e62466_d_n9;

        let (assign48700_e62480, assign48700_e62480_d_n4, assign48700_e62480_d_n6, assign48700_e62480_d_n7, assign48700_e62480_d_n8, assign48700_e62480_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
        let assign48700_e62475: f64 = (locals.var_xct__blk1334 / locals.var_xctmax__blk1330);
        let assign48700_e62477: f64 = (assign48700_e62475 + 1.0);
        let assign48700_e62478: f64 = (locals.var_ctg_t * assign48700_e62477);
        (assign48700_e62478, ((locals.var_ctg_t_dn4 * assign48700_e62477) + (locals.var_ctg_t * (((locals.var_xct__blk1334_dn4 * locals.var_xctmax__blk1330) - (locals.var_xct__blk1334 * locals.var_xctmax__blk1330_dn4)) / (locals.var_xctmax__blk1330 * locals.var_xctmax__blk1330)))), (locals.var_ctg_t * (locals.var_xct__blk1334_dn6 / locals.var_xctmax__blk1330)), (locals.var_ctg_t * (locals.var_xct__blk1334_dn7 / locals.var_xctmax__blk1330)), (locals.var_ctg_t * (locals.var_xct__blk1334_dn8 / locals.var_xctmax__blk1330)), (locals.var_ctg_t * (locals.var_xct__blk1334_dn9 / locals.var_xctmax__blk1330)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign48700_e62480;
        locals.var_temp2_dn4 = assign48700_e62480_d_n4;
        locals.var_temp2_dn6 = assign48700_e62480_d_n6;
        locals.var_temp2_dn7 = assign48700_e62480_d_n7;
        locals.var_temp2_dn8 = assign48700_e62480_d_n8;
        locals.var_temp2_dn9 = assign48700_e62480_d_n9;

        let assign48710_e62483: f64 = (-230.25850929940458);
        let assign48710_e62484: f64 = if locals.var_temp2 > assign48710_e62483 { 1.0 } else { 0.0 };
        locals.var_guard1477 = assign48710_e62484;

        let (assign48720_e62495, assign48720_e62495_d_n4, assign48720_e62495_d_n6, assign48720_e62495_d_n7, assign48720_e62495_d_n8, assign48720_e62495_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) && (locals.var_guard1477 != 0.0)) {
        let assign48720_e62493: f64 = (locals.var_temp2).exp();
        (assign48720_e62493, (assign48720_e62493 * locals.var_temp2_dn4), (assign48720_e62493 * locals.var_temp2_dn6), (assign48720_e62493 * locals.var_temp2_dn7), (assign48720_e62493 * locals.var_temp2_dn8), (assign48720_e62493 * locals.var_temp2_dn9),)
    } else {
        (locals.var_dctg__blk1335, locals.var_dctg__blk1335_dn4, locals.var_dctg__blk1335_dn6, locals.var_dctg__blk1335_dn7, locals.var_dctg__blk1335_dn8, locals.var_dctg__blk1335_dn9,)
    }
};
        locals.var_dctg__blk1335 = assign48720_e62495;
        locals.var_dctg__blk1335_dn4 = assign48720_e62495_d_n4;
        locals.var_dctg__blk1335_dn6 = assign48720_e62495_d_n6;
        locals.var_dctg__blk1335_dn7 = assign48720_e62495_d_n7;
        locals.var_dctg__blk1335_dn8 = assign48720_e62495_d_n8;
        locals.var_dctg__blk1335_dn9 = assign48720_e62495_d_n9;

        let (assign48730_e62531, assign48730_e62531_d_n4, assign48730_e62531_d_n6, assign48730_e62531_d_n7, assign48730_e62531_d_n8, assign48730_e62531_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) && (locals.var_guard1477 == 0.0)) {
        let assign48730_e62507: f64 = (-230.25850929940458);
        let assign48730_e62509: f64 = (assign48730_e62507 - locals.var_temp2);
        let assign48730_e62513: f64 = (-230.25850929940458);
        let assign48730_e62515: f64 = (assign48730_e62513 - locals.var_temp2);
        let assign48730_e62518: f64 = (-230.25850929940458);
        let assign48730_e62520: f64 = (assign48730_e62518 - locals.var_temp2);
        let assign48730_e62522: f64 = (assign48730_e62520 * 0.3333333333333333);
        let assign48730_e62523: f64 = (1.0 + assign48730_e62522);
        let assign48730_e62524: f64 = (assign48730_e62515 * assign48730_e62523);
        let assign48730_e62525: f64 = (0.5 * assign48730_e62524);
        let assign48730_e62526: f64 = (1.0 + assign48730_e62525);
        let assign48730_e62527: f64 = (assign48730_e62509 * assign48730_e62526);
        let assign48730_e62528: f64 = (1.0 + assign48730_e62527);
        let assign48730_e62529: f64 = (1e-100 / assign48730_e62528);
        (assign48730_e62529, (-((1e-100 * (((-locals.var_temp2_dn4) * assign48730_e62526) + (assign48730_e62509 * (0.5 * (((-locals.var_temp2_dn4) * assign48730_e62523) + (assign48730_e62515 * ((-locals.var_temp2_dn4) * 0.3333333333333333))))))) / (assign48730_e62528 * assign48730_e62528))), (-((1e-100 * (((-locals.var_temp2_dn6) * assign48730_e62526) + (assign48730_e62509 * (0.5 * (((-locals.var_temp2_dn6) * assign48730_e62523) + (assign48730_e62515 * ((-locals.var_temp2_dn6) * 0.3333333333333333))))))) / (assign48730_e62528 * assign48730_e62528))), (-((1e-100 * (((-locals.var_temp2_dn7) * assign48730_e62526) + (assign48730_e62509 * (0.5 * (((-locals.var_temp2_dn7) * assign48730_e62523) + (assign48730_e62515 * ((-locals.var_temp2_dn7) * 0.3333333333333333))))))) / (assign48730_e62528 * assign48730_e62528))), (-((1e-100 * (((-locals.var_temp2_dn8) * assign48730_e62526) + (assign48730_e62509 * (0.5 * (((-locals.var_temp2_dn8) * assign48730_e62523) + (assign48730_e62515 * ((-locals.var_temp2_dn8) * 0.3333333333333333))))))) / (assign48730_e62528 * assign48730_e62528))), (-((1e-100 * (((-locals.var_temp2_dn9) * assign48730_e62526) + (assign48730_e62509 * (0.5 * (((-locals.var_temp2_dn9) * assign48730_e62523) + (assign48730_e62515 * ((-locals.var_temp2_dn9) * 0.3333333333333333))))))) / (assign48730_e62528 * assign48730_e62528))),)
    } else {
        (locals.var_dctg__blk1335, locals.var_dctg__blk1335_dn4, locals.var_dctg__blk1335_dn6, locals.var_dctg__blk1335_dn7, locals.var_dctg__blk1335_dn8, locals.var_dctg__blk1335_dn9,)
    }
};
        locals.var_dctg__blk1335 = assign48730_e62531;
        locals.var_dctg__blk1335_dn4 = assign48730_e62531_d_n4;
        locals.var_dctg__blk1335_dn6 = assign48730_e62531_d_n6;
        locals.var_dctg__blk1335_dn7 = assign48730_e62531_d_n7;
        locals.var_dctg__blk1335_dn8 = assign48730_e62531_d_n8;
        locals.var_dctg__blk1335_dn9 = assign48730_e62531_d_n9;

        let (assign48740_e62541, assign48740_e62541_d_n4, assign48740_e62541_d_n6, assign48740_e62541_d_n7, assign48740_e62541_d_n8, assign48740_e62541_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48740_e62538: f64 = (locals.var_ct_t * locals.var_dctg__blk1335);
        let assign48740_e62539: f64 = (1.0 + assign48740_e62538);
        (assign48740_e62539, ((locals.var_ct_t_dn4 * locals.var_dctg__blk1335) + (locals.var_ct_t * locals.var_dctg__blk1335_dn4)), (locals.var_ct_t * locals.var_dctg__blk1335_dn6), (locals.var_ct_t * locals.var_dctg__blk1335_dn7), (locals.var_ct_t * locals.var_dctg__blk1335_dn8), (locals.var_ct_t * locals.var_dctg__blk1335_dn9),)
    } else {
        (locals.var_ct_fact__blk1336, locals.var_ct_fact__blk1336_dn4, locals.var_ct_fact__blk1336_dn6, locals.var_ct_fact__blk1336_dn7, locals.var_ct_fact__blk1336_dn8, locals.var_ct_fact__blk1336_dn9,)
    }
};
        locals.var_ct_fact__blk1336 = assign48740_e62541;
        locals.var_ct_fact__blk1336_dn4 = assign48740_e62541_d_n4;
        locals.var_ct_fact__blk1336_dn6 = assign48740_e62541_d_n6;
        locals.var_ct_fact__blk1336_dn7 = assign48740_e62541_d_n7;
        locals.var_ct_fact__blk1336_dn8 = assign48740_e62541_d_n8;
        locals.var_ct_fact__blk1336_dn9 = assign48740_e62541_d_n9;

        let (assign48750_e62549, assign48750_e62549_d_n4, assign48750_e62549_d_n6, assign48750_e62549_d_n7, assign48750_e62549_d_n8, assign48750_e62549_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48750_e62547: f64 = (locals.var_phit * locals.var_ct_fact__blk1336);
        (assign48750_e62547, ((locals.var_phit_dn4 * locals.var_ct_fact__blk1336) + (locals.var_phit * locals.var_ct_fact__blk1336_dn4)), (locals.var_phit * locals.var_ct_fact__blk1336_dn6), (locals.var_phit * locals.var_ct_fact__blk1336_dn7), (locals.var_phit * locals.var_ct_fact__blk1336_dn8), (locals.var_phit * locals.var_ct_fact__blk1336_dn9),)
    } else {
        (locals.var_phitct__blk1337, locals.var_phitct__blk1337_dn4, locals.var_phitct__blk1337_dn6, locals.var_phitct__blk1337_dn7, locals.var_phitct__blk1337_dn8, locals.var_phitct__blk1337_dn9,)
    }
};
        locals.var_phitct__blk1337 = assign48750_e62549;
        locals.var_phitct__blk1337_dn4 = assign48750_e62549_d_n4;
        locals.var_phitct__blk1337_dn6 = assign48750_e62549_d_n6;
        locals.var_phitct__blk1337_dn7 = assign48750_e62549_d_n7;
        locals.var_phitct__blk1337_dn8 = assign48750_e62549_d_n8;
        locals.var_phitct__blk1337_dn9 = assign48750_e62549_d_n9;

        let (assign48760_e62567, assign48760_e62567_d_n4, assign48760_e62567_d_n6, assign48760_e62567_d_n7, assign48760_e62567_d_n8, assign48760_e62567_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48760_e62557: f64 = (locals.var_psced_i * locals.var_vdsx);
        let assign48760_e62558: f64 = (1.0 + assign48760_e62557);
        let assign48760_e62559: f64 = (locals.var_psce_i * assign48760_e62558);
        let assign48760_e62563: f64 = (locals.var_psceb_i * locals.var_vsbx__blk1323);
        let assign48760_e62564: f64 = (1.0 + assign48760_e62563);
        let assign48760_e62565: f64 = (assign48760_e62559 * assign48760_e62564);
        (assign48760_e62565, (assign48760_e62559 * (locals.var_psceb_i * locals.var_vsbx__blk1323_dn4)), (assign48760_e62559 * (locals.var_psceb_i * locals.var_vsbx__blk1323_dn6)), (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn7)) * assign48760_e62564) + (assign48760_e62559 * (locals.var_psceb_i * locals.var_vsbx__blk1323_dn7))), (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn8)) * assign48760_e62564) + (assign48760_e62559 * (locals.var_psceb_i * locals.var_vsbx__blk1323_dn8))), (assign48760_e62559 * (locals.var_psceb_i * locals.var_vsbx__blk1323_dn9)),)
    } else {
        (locals.var_dphit1__blk1338, locals.var_dphit1__blk1338_dn4, locals.var_dphit1__blk1338_dn6, locals.var_dphit1__blk1338_dn7, locals.var_dphit1__blk1338_dn8, locals.var_dphit1__blk1338_dn9,)
    }
};
        locals.var_dphit1__blk1338 = assign48760_e62567;
        locals.var_dphit1__blk1338_dn4 = assign48760_e62567_d_n4;
        locals.var_dphit1__blk1338_dn6 = assign48760_e62567_d_n6;
        locals.var_dphit1__blk1338_dn7 = assign48760_e62567_d_n7;
        locals.var_dphit1__blk1338_dn8 = assign48760_e62567_d_n8;
        locals.var_dphit1__blk1338_dn9 = assign48760_e62567_d_n9;

        let (assign48770_e62577, assign48770_e62577_d_n4, assign48770_e62577_d_n6, assign48770_e62577_d_n7, assign48770_e62577_d_n8, assign48770_e62577_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48770_e62574: f64 = (1.0 + locals.var_dphit1__blk1338);
        let assign48770_e62575: f64 = (locals.var_phitct__blk1337 * assign48770_e62574);
        (assign48770_e62575, ((locals.var_phitct__blk1337_dn4 * assign48770_e62574) + (locals.var_phitct__blk1337 * locals.var_dphit1__blk1338_dn4)), ((locals.var_phitct__blk1337_dn6 * assign48770_e62574) + (locals.var_phitct__blk1337 * locals.var_dphit1__blk1338_dn6)), ((locals.var_phitct__blk1337_dn7 * assign48770_e62574) + (locals.var_phitct__blk1337 * locals.var_dphit1__blk1338_dn7)), ((locals.var_phitct__blk1337_dn8 * assign48770_e62574) + (locals.var_phitct__blk1337 * locals.var_dphit1__blk1338_dn8)), ((locals.var_phitct__blk1337_dn9 * assign48770_e62574) + (locals.var_phitct__blk1337 * locals.var_dphit1__blk1338_dn9)),)
    } else {
        (locals.var_phit1__blk1339, locals.var_phit1__blk1339_dn4, locals.var_phit1__blk1339_dn6, locals.var_phit1__blk1339_dn7, locals.var_phit1__blk1339_dn8, locals.var_phit1__blk1339_dn9,)
    }
};
        locals.var_phit1__blk1339 = assign48770_e62577;
        locals.var_phit1__blk1339_dn4 = assign48770_e62577_d_n4;
        locals.var_phit1__blk1339_dn6 = assign48770_e62577_d_n6;
        locals.var_phit1__blk1339_dn7 = assign48770_e62577_d_n7;
        locals.var_phit1__blk1339_dn8 = assign48770_e62577_d_n8;
        locals.var_phit1__blk1339_dn9 = assign48770_e62577_d_n9;

        let (assign48780_e62585, assign48780_e62585_d_n4, assign48780_e62585_d_n6, assign48780_e62585_d_n7, assign48780_e62585_d_n8, assign48780_e62585_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48780_e62583: f64 = (1.0 / locals.var_phit1__blk1339);
        (assign48780_e62583, (-(locals.var_phit1__blk1339_dn4 / (locals.var_phit1__blk1339 * locals.var_phit1__blk1339))), (-(locals.var_phit1__blk1339_dn6 / (locals.var_phit1__blk1339 * locals.var_phit1__blk1339))), (-(locals.var_phit1__blk1339_dn7 / (locals.var_phit1__blk1339 * locals.var_phit1__blk1339))), (-(locals.var_phit1__blk1339_dn8 / (locals.var_phit1__blk1339 * locals.var_phit1__blk1339))), (-(locals.var_phit1__blk1339_dn9 / (locals.var_phit1__blk1339 * locals.var_phit1__blk1339))),)
    } else {
        (locals.var_inv_phit1__blk1340, locals.var_inv_phit1__blk1340_dn4, locals.var_inv_phit1__blk1340_dn6, locals.var_inv_phit1__blk1340_dn7, locals.var_inv_phit1__blk1340_dn8, locals.var_inv_phit1__blk1340_dn9,)
    }
};
        locals.var_inv_phit1__blk1340 = assign48780_e62585;
        locals.var_inv_phit1__blk1340_dn4 = assign48780_e62585_d_n4;
        locals.var_inv_phit1__blk1340_dn6 = assign48780_e62585_d_n6;
        locals.var_inv_phit1__blk1340_dn7 = assign48780_e62585_d_n7;
        locals.var_inv_phit1__blk1340_dn8 = assign48780_e62585_d_n8;
        locals.var_inv_phit1__blk1340_dn9 = assign48780_e62585_d_n9;

        let (assign48790_e62596, assign48790_e62596_d_n4, assign48790_e62596_d_n6, assign48790_e62596_d_n7, assign48790_e62596_d_n8, assign48790_e62596_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48790_e62592: f64 = (locals.var_phit * locals.var_inv_phit1__blk1340);
        let assign48790_e62593: f64 = (assign48790_e62592).sqrt();
        let assign48790_e62594: f64 = (locals.var_g_0__blk1316 * assign48790_e62593);
        (assign48790_e62594, ((locals.var_g_0__blk1316_dn4 * assign48790_e62593) + (locals.var_g_0__blk1316 * (((locals.var_phit_dn4 * locals.var_inv_phit1__blk1340) + (locals.var_phit * locals.var_inv_phit1__blk1340_dn4)) / (2.0 * assign48790_e62593)))), (locals.var_g_0__blk1316 * ((locals.var_phit * locals.var_inv_phit1__blk1340_dn6) / (2.0 * assign48790_e62593))), (locals.var_g_0__blk1316 * ((locals.var_phit * locals.var_inv_phit1__blk1340_dn7) / (2.0 * assign48790_e62593))), (locals.var_g_0__blk1316 * ((locals.var_phit * locals.var_inv_phit1__blk1340_dn8) / (2.0 * assign48790_e62593))), (locals.var_g_0__blk1316 * ((locals.var_phit * locals.var_inv_phit1__blk1340_dn9) / (2.0 * assign48790_e62593))),)
    } else {
        (locals.var_gf__blk1324, locals.var_gf__blk1324_dn4, locals.var_gf__blk1324_dn6, locals.var_gf__blk1324_dn7, locals.var_gf__blk1324_dn8, locals.var_gf__blk1324_dn9,)
    }
};
        locals.var_gf__blk1324 = assign48790_e62596;
        locals.var_gf__blk1324_dn4 = assign48790_e62596_d_n4;
        locals.var_gf__blk1324_dn6 = assign48790_e62596_d_n6;
        locals.var_gf__blk1324_dn7 = assign48790_e62596_d_n7;
        locals.var_gf__blk1324_dn8 = assign48790_e62596_d_n8;
        locals.var_gf__blk1324_dn9 = assign48790_e62596_d_n9;

        let (assign48800_e62604, assign48800_e62604_d_n4, assign48800_e62604_d_n6, assign48800_e62604_d_n7, assign48800_e62604_d_n8, assign48800_e62604_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48800_e62602: f64 = (locals.var_gf__blk1324 * locals.var_gf__blk1324);
        (assign48800_e62602, ((locals.var_gf__blk1324_dn4 * locals.var_gf__blk1324) + (locals.var_gf__blk1324 * locals.var_gf__blk1324_dn4)), ((locals.var_gf__blk1324_dn6 * locals.var_gf__blk1324) + (locals.var_gf__blk1324 * locals.var_gf__blk1324_dn6)), ((locals.var_gf__blk1324_dn7 * locals.var_gf__blk1324) + (locals.var_gf__blk1324 * locals.var_gf__blk1324_dn7)), ((locals.var_gf__blk1324_dn8 * locals.var_gf__blk1324) + (locals.var_gf__blk1324 * locals.var_gf__blk1324_dn8)), ((locals.var_gf__blk1324_dn9 * locals.var_gf__blk1324) + (locals.var_gf__blk1324 * locals.var_gf__blk1324_dn9)),)
    } else {
        (locals.var_gf2__blk1325, locals.var_gf2__blk1325_dn4, locals.var_gf2__blk1325_dn6, locals.var_gf2__blk1325_dn7, locals.var_gf2__blk1325_dn8, locals.var_gf2__blk1325_dn9,)
    }
};
        locals.var_gf2__blk1325 = assign48800_e62604;
        locals.var_gf2__blk1325_dn4 = assign48800_e62604_d_n4;
        locals.var_gf2__blk1325_dn6 = assign48800_e62604_d_n6;
        locals.var_gf2__blk1325_dn7 = assign48800_e62604_d_n7;
        locals.var_gf2__blk1325_dn8 = assign48800_e62604_d_n8;
        locals.var_gf2__blk1325_dn9 = assign48800_e62604_d_n9;

        let (assign48810_e62612, assign48810_e62612_d_n4, assign48810_e62612_d_n6, assign48810_e62612_d_n7, assign48810_e62612_d_n8, assign48810_e62612_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48810_e62610: f64 = (1.0 / locals.var_gf2__blk1325);
        (assign48810_e62610, (-(locals.var_gf2__blk1325_dn4 / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))), (-(locals.var_gf2__blk1325_dn6 / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))), (-(locals.var_gf2__blk1325_dn7 / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))), (-(locals.var_gf2__blk1325_dn8 / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))), (-(locals.var_gf2__blk1325_dn9 / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))),)
    } else {
        (locals.var_inv_gf2__blk1341, locals.var_inv_gf2__blk1341_dn4, locals.var_inv_gf2__blk1341_dn6, locals.var_inv_gf2__blk1341_dn7, locals.var_inv_gf2__blk1341_dn8, locals.var_inv_gf2__blk1341_dn9,)
    }
};
        locals.var_inv_gf2__blk1341 = assign48810_e62612;
        locals.var_inv_gf2__blk1341_dn4 = assign48810_e62612_d_n4;
        locals.var_inv_gf2__blk1341_dn6 = assign48810_e62612_d_n6;
        locals.var_inv_gf2__blk1341_dn7 = assign48810_e62612_d_n7;
        locals.var_inv_gf2__blk1341_dn8 = assign48810_e62612_d_n8;
        locals.var_inv_gf2__blk1341_dn9 = assign48810_e62612_d_n9;

        let (assign48820_e62620, assign48820_e62620_d_n4, assign48820_e62620_d_n6, assign48820_e62620_d_n7, assign48820_e62620_d_n8, assign48820_e62620_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48820_e62618: f64 = (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340);
        (assign48820_e62618, ((locals.var_vsbstar__blk1318_dn4 * locals.var_inv_phit1__blk1340) + (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340_dn4)), ((locals.var_vsbstar__blk1318_dn6 * locals.var_inv_phit1__blk1340) + (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340_dn6)), ((locals.var_vsbstar__blk1318_dn7 * locals.var_inv_phit1__blk1340) + (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340_dn7)), ((locals.var_vsbstar__blk1318_dn8 * locals.var_inv_phit1__blk1340) + (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340_dn8)), ((locals.var_vsbstar__blk1318_dn9 * locals.var_inv_phit1__blk1340) + (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340_dn9)),)
    } else {
        (locals.var_ux__blk1342, locals.var_ux__blk1342_dn4, locals.var_ux__blk1342_dn6, locals.var_ux__blk1342_dn7, locals.var_ux__blk1342_dn8, locals.var_ux__blk1342_dn9,)
    }
};
        locals.var_ux__blk1342 = assign48820_e62620;
        locals.var_ux__blk1342_dn4 = assign48820_e62620_d_n4;
        locals.var_ux__blk1342_dn6 = assign48820_e62620_d_n6;
        locals.var_ux__blk1342_dn7 = assign48820_e62620_d_n7;
        locals.var_ux__blk1342_dn8 = assign48820_e62620_d_n8;
        locals.var_ux__blk1342_dn9 = assign48820_e62620_d_n9;

        let (assign48830_e62628, assign48830_e62628_d_n4, assign48830_e62628_d_n6, assign48830_e62628_d_n7, assign48830_e62628_d_n8, assign48830_e62628_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48830_e62626: f64 = (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340);
        (assign48830_e62626, ((locals.var_vgb1__blk1321_dn4 * locals.var_inv_phit1__blk1340) + (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340_dn4)), ((locals.var_vgb1__blk1321_dn6 * locals.var_inv_phit1__blk1340) + (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340_dn6)), ((locals.var_vgb1__blk1321_dn7 * locals.var_inv_phit1__blk1340) + (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340_dn7)), ((locals.var_vgb1__blk1321_dn8 * locals.var_inv_phit1__blk1340) + (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340_dn8)), ((locals.var_vgb1__blk1321_dn9 * locals.var_inv_phit1__blk1340) + (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340_dn9)),)
    } else {
        (locals.var_xg__blk1343, locals.var_xg__blk1343_dn4, locals.var_xg__blk1343_dn6, locals.var_xg__blk1343_dn7, locals.var_xg__blk1343_dn8, locals.var_xg__blk1343_dn9,)
    }
};
        locals.var_xg__blk1343 = assign48830_e62628;
        locals.var_xg__blk1343_dn4 = assign48830_e62628_d_n4;
        locals.var_xg__blk1343_dn6 = assign48830_e62628_d_n6;
        locals.var_xg__blk1343_dn7 = assign48830_e62628_d_n7;
        locals.var_xg__blk1343_dn8 = assign48830_e62628_d_n8;
        locals.var_xg__blk1343_dn9 = assign48830_e62628_d_n9;

        let (assign48840_e62645, assign48840_e62645_d_n7, assign48840_e62645_d_n8,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48840_e62634: f64 = (2.0 * locals.var_vdsx);
        let assign48840_e62639: f64 = (locals.var_cfd_i * locals.var_vdsx);
        let assign48840_e62640: f64 = (1.0 + assign48840_e62639);
        let assign48840_e62641: f64 = (assign48840_e62640).sqrt();
        let assign48840_e62642: f64 = (1.0 + assign48840_e62641);
        let assign48840_e62643: f64 = (assign48840_e62634 / assign48840_e62642);
        (assign48840_e62643, ((((2.0 * locals.var_vdsx_dn7) * assign48840_e62642) - (assign48840_e62634 * ((locals.var_cfd_i * locals.var_vdsx_dn7) / (2.0 * assign48840_e62641)))) / (assign48840_e62642 * assign48840_e62642)), ((((2.0 * locals.var_vdsx_dn8) * assign48840_e62642) - (assign48840_e62634 * ((locals.var_cfd_i * locals.var_vdsx_dn8) / (2.0 * assign48840_e62641)))) / (assign48840_e62642 * assign48840_e62642)),)
    } else {
        (locals.var_vdsp__blk1344, locals.var_vdsp__blk1344_dn7, locals.var_vdsp__blk1344_dn8,)
    }
};
        locals.var_vdsp__blk1344 = assign48840_e62645;
        locals.var_vdsp__blk1344_dn7 = assign48840_e62645_d_n7;
        locals.var_vdsp__blk1344_dn8 = assign48840_e62645_d_n8;

        let (assign48850_e62659, assign48850_e62659_d_n4, assign48850_e62659_d_n6, assign48850_e62659_d_n7, assign48850_e62659_d_n8, assign48850_e62659_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48850_e62651: f64 = (locals.var_cf_i * locals.var_vdsp__blk1344);
        let assign48850_e62655: f64 = (locals.var_cfb_i * locals.var_vsbx__blk1323);
        let assign48850_e62656: f64 = (1.0 + assign48850_e62655);
        let assign48850_e62657: f64 = (assign48850_e62651 * assign48850_e62656);
        (assign48850_e62657, (assign48850_e62651 * (locals.var_cfb_i * locals.var_vsbx__blk1323_dn4)), (assign48850_e62651 * (locals.var_cfb_i * locals.var_vsbx__blk1323_dn6)), (((locals.var_cf_i * locals.var_vdsp__blk1344_dn7) * assign48850_e62656) + (assign48850_e62651 * (locals.var_cfb_i * locals.var_vsbx__blk1323_dn7))), (((locals.var_cf_i * locals.var_vdsp__blk1344_dn8) * assign48850_e62656) + (assign48850_e62651 * (locals.var_cfb_i * locals.var_vsbx__blk1323_dn8))), (assign48850_e62651 * (locals.var_cfb_i * locals.var_vsbx__blk1323_dn9)),)
    } else {
        (locals.var_delphib__blk1345, locals.var_delphib__blk1345_dn4, locals.var_delphib__blk1345_dn6, locals.var_delphib__blk1345_dn7, locals.var_delphib__blk1345_dn8, locals.var_delphib__blk1345_dn9,)
    }
};
        locals.var_delphib__blk1345 = assign48850_e62659;
        locals.var_delphib__blk1345_dn4 = assign48850_e62659_d_n4;
        locals.var_delphib__blk1345_dn6 = assign48850_e62659_d_n6;
        locals.var_delphib__blk1345_dn7 = assign48850_e62659_d_n7;
        locals.var_delphib__blk1345_dn8 = assign48850_e62659_d_n8;
        locals.var_delphib__blk1345_dn9 = assign48850_e62659_d_n9;

        let (assign48860_e62667, assign48860_e62667_d_n4, assign48860_e62667_d_n6, assign48860_e62667_d_n7, assign48860_e62667_d_n8, assign48860_e62667_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48860_e62665: f64 = (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340);
        (assign48860_e62665, ((locals.var_phib__blk1314_dn4 * locals.var_inv_phit1__blk1340) + (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340_dn4)), (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340_dn6), (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340_dn7), (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340_dn8), (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340_dn9),)
    } else {
        (locals.var_xb__blk1346, locals.var_xb__blk1346_dn4, locals.var_xb__blk1346_dn6, locals.var_xb__blk1346_dn7, locals.var_xb__blk1346_dn8, locals.var_xb__blk1346_dn9,)
    }
};
        locals.var_xb__blk1346 = assign48860_e62667;
        locals.var_xb__blk1346_dn4 = assign48860_e62667_d_n4;
        locals.var_xb__blk1346_dn6 = assign48860_e62667_d_n6;
        locals.var_xb__blk1346_dn7 = assign48860_e62667_d_n7;
        locals.var_xb__blk1346_dn8 = assign48860_e62667_d_n8;
        locals.var_xb__blk1346_dn9 = assign48860_e62667_d_n9;

        let (assign48870_e62678, assign48870_e62678_d_n4, assign48870_e62678_d_n6, assign48870_e62678_d_n7, assign48870_e62678_d_n8, assign48870_e62678_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48870_e62673: f64 = (locals.var_v_xb__blk1317 * locals.var_v_xb__blk1317);
        let assign48870_e62675: f64 = (assign48870_e62673 + locals.var_aphi__blk1315);
        let assign48870_e62676: f64 = (assign48870_e62675).sqrt();
        (assign48870_e62676, ((((locals.var_v_xb__blk1317_dn4 * locals.var_v_xb__blk1317) + (locals.var_v_xb__blk1317 * locals.var_v_xb__blk1317_dn4)) + locals.var_aphi__blk1315_dn4) / (2.0 * assign48870_e62676)), 0.0, (((locals.var_v_xb__blk1317_dn7 * locals.var_v_xb__blk1317) + (locals.var_v_xb__blk1317 * locals.var_v_xb__blk1317_dn7)) / (2.0 * assign48870_e62676)), (((locals.var_v_xb__blk1317_dn8 * locals.var_v_xb__blk1317) + (locals.var_v_xb__blk1317 * locals.var_v_xb__blk1317_dn8)) / (2.0 * assign48870_e62676)), (((locals.var_v_xb__blk1317_dn9 * locals.var_v_xb__blk1317) + (locals.var_v_xb__blk1317 * locals.var_v_xb__blk1317_dn9)) / (2.0 * assign48870_e62676)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign48870_e62678;
        locals.var_temp1_dn4 = assign48870_e62678_d_n4;
        locals.var_temp1_dn6 = assign48870_e62678_d_n6;
        locals.var_temp1_dn7 = assign48870_e62678_d_n7;
        locals.var_temp1_dn8 = assign48870_e62678_d_n8;
        locals.var_temp1_dn9 = assign48870_e62678_d_n9;

        let (assign48880_e62693, assign48880_e62693_d_n4, assign48880_e62693_d_n6, assign48880_e62693_d_n7, assign48880_e62693_d_n8, assign48880_e62693_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48880_e62684: f64 = (locals.var_v_xb__blk1317 - locals.var_delphib__blk1345);
        let assign48880_e62687: f64 = (locals.var_v_xb__blk1317 - locals.var_delphib__blk1345);
        let assign48880_e62688: f64 = (assign48880_e62684 * assign48880_e62687);
        let assign48880_e62690: f64 = (assign48880_e62688 + locals.var_aphi__blk1315);
        let assign48880_e62691: f64 = (assign48880_e62690).sqrt();
        (assign48880_e62691, (((((locals.var_v_xb__blk1317_dn4 - locals.var_delphib__blk1345_dn4) * assign48880_e62687) + (assign48880_e62684 * (locals.var_v_xb__blk1317_dn4 - locals.var_delphib__blk1345_dn4))) + locals.var_aphi__blk1315_dn4) / (2.0 * assign48880_e62691)), ((((-locals.var_delphib__blk1345_dn6) * assign48880_e62687) + (assign48880_e62684 * (-locals.var_delphib__blk1345_dn6))) / (2.0 * assign48880_e62691)), ((((locals.var_v_xb__blk1317_dn7 - locals.var_delphib__blk1345_dn7) * assign48880_e62687) + (assign48880_e62684 * (locals.var_v_xb__blk1317_dn7 - locals.var_delphib__blk1345_dn7))) / (2.0 * assign48880_e62691)), ((((locals.var_v_xb__blk1317_dn8 - locals.var_delphib__blk1345_dn8) * assign48880_e62687) + (assign48880_e62684 * (locals.var_v_xb__blk1317_dn8 - locals.var_delphib__blk1345_dn8))) / (2.0 * assign48880_e62691)), ((((locals.var_v_xb__blk1317_dn9 - locals.var_delphib__blk1345_dn9) * assign48880_e62687) + (assign48880_e62684 * (locals.var_v_xb__blk1317_dn9 - locals.var_delphib__blk1345_dn9))) / (2.0 * assign48880_e62691)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign48880_e62693;
        locals.var_temp2_dn4 = assign48880_e62693_d_n4;
        locals.var_temp2_dn6 = assign48880_e62693_d_n6;
        locals.var_temp2_dn7 = assign48880_e62693_d_n7;
        locals.var_temp2_dn8 = assign48880_e62693_d_n8;
        locals.var_temp2_dn9 = assign48880_e62693_d_n9;

        let (assign48890_e62707, assign48890_e62707_d_n4, assign48890_e62707_d_n6, assign48890_e62707_d_n7, assign48890_e62707_d_n8, assign48890_e62707_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48890_e62699: f64 = (0.5 * locals.var_inv_phit1__blk1340);
        let assign48890_e62702: f64 = (locals.var_delphib__blk1345 + locals.var_temp1);
        let assign48890_e62704: f64 = (assign48890_e62702 - locals.var_temp2);
        let assign48890_e62705: f64 = (assign48890_e62699 * assign48890_e62704);
        (assign48890_e62705, (((0.5 * locals.var_inv_phit1__blk1340_dn4) * assign48890_e62704) + (assign48890_e62699 * ((locals.var_delphib__blk1345_dn4 + locals.var_temp1_dn4) - locals.var_temp2_dn4))), (((0.5 * locals.var_inv_phit1__blk1340_dn6) * assign48890_e62704) + (assign48890_e62699 * ((locals.var_delphib__blk1345_dn6 + locals.var_temp1_dn6) - locals.var_temp2_dn6))), (((0.5 * locals.var_inv_phit1__blk1340_dn7) * assign48890_e62704) + (assign48890_e62699 * ((locals.var_delphib__blk1345_dn7 + locals.var_temp1_dn7) - locals.var_temp2_dn7))), (((0.5 * locals.var_inv_phit1__blk1340_dn8) * assign48890_e62704) + (assign48890_e62699 * ((locals.var_delphib__blk1345_dn8 + locals.var_temp1_dn8) - locals.var_temp2_dn8))), (((0.5 * locals.var_inv_phit1__blk1340_dn9) * assign48890_e62704) + (assign48890_e62699 * ((locals.var_delphib__blk1345_dn9 + locals.var_temp1_dn9) - locals.var_temp2_dn9))),)
    } else {
        (locals.var_delxb__blk1347, locals.var_delxb__blk1347_dn4, locals.var_delxb__blk1347_dn6, locals.var_delxb__blk1347_dn7, locals.var_delxb__blk1347_dn8, locals.var_delxb__blk1347_dn9,)
    }
};
        locals.var_delxb__blk1347 = assign48890_e62707;
        locals.var_delxb__blk1347_dn4 = assign48890_e62707_d_n4;
        locals.var_delxb__blk1347_dn6 = assign48890_e62707_d_n6;
        locals.var_delxb__blk1347_dn7 = assign48890_e62707_d_n7;
        locals.var_delxb__blk1347_dn8 = assign48890_e62707_d_n8;
        locals.var_delxb__blk1347_dn9 = assign48890_e62707_d_n9;

        let (assign48900_e62715, assign48900_e62715_d_n4, assign48900_e62715_d_n6, assign48900_e62715_d_n7, assign48900_e62715_d_n8, assign48900_e62715_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48900_e62713: f64 = (locals.var_xb__blk1346 + locals.var_ux__blk1342);
        (assign48900_e62713, (locals.var_xb__blk1346_dn4 + locals.var_ux__blk1342_dn4), (locals.var_xb__blk1346_dn6 + locals.var_ux__blk1342_dn6), (locals.var_xb__blk1346_dn7 + locals.var_ux__blk1342_dn7), (locals.var_xb__blk1346_dn8 + locals.var_ux__blk1342_dn8), (locals.var_xb__blk1346_dn9 + locals.var_ux__blk1342_dn9),)
    } else {
        (locals.var_xno_s__blk1348, locals.var_xno_s__blk1348_dn4, locals.var_xno_s__blk1348_dn6, locals.var_xno_s__blk1348_dn7, locals.var_xno_s__blk1348_dn8, locals.var_xno_s__blk1348_dn9,)
    }
};
        locals.var_xno_s__blk1348 = assign48900_e62715;
        locals.var_xno_s__blk1348_dn4 = assign48900_e62715_d_n4;
        locals.var_xno_s__blk1348_dn6 = assign48900_e62715_d_n6;
        locals.var_xno_s__blk1348_dn7 = assign48900_e62715_d_n7;
        locals.var_xno_s__blk1348_dn8 = assign48900_e62715_d_n8;
        locals.var_xno_s__blk1348_dn9 = assign48900_e62715_d_n9;

        let (assign48910_e62723, assign48910_e62723_d_n4, assign48910_e62723_d_n6, assign48910_e62723_d_n7, assign48910_e62723_d_n8, assign48910_e62723_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign48910_e62721: f64 = (locals.var_xno_s__blk1348 - locals.var_delxb__blk1347);
        (assign48910_e62721, (locals.var_xno_s__blk1348_dn4 - locals.var_delxb__blk1347_dn4), (locals.var_xno_s__blk1348_dn6 - locals.var_delxb__blk1347_dn6), (locals.var_xno_s__blk1348_dn7 - locals.var_delxb__blk1347_dn7), (locals.var_xno_s__blk1348_dn8 - locals.var_delxb__blk1347_dn8), (locals.var_xno_s__blk1348_dn9 - locals.var_delxb__blk1347_dn9),)
    } else {
        (locals.var_xn_s__blk1349, locals.var_xn_s__blk1349_dn4, locals.var_xn_s__blk1349_dn6, locals.var_xn_s__blk1349_dn7, locals.var_xn_s__blk1349_dn8, locals.var_xn_s__blk1349_dn9,)
    }
};
        locals.var_xn_s__blk1349 = assign48910_e62723;
        locals.var_xn_s__blk1349_dn4 = assign48910_e62723_d_n4;
        locals.var_xn_s__blk1349_dn6 = assign48910_e62723_d_n6;
        locals.var_xn_s__blk1349_dn7 = assign48910_e62723_d_n7;
        locals.var_xn_s__blk1349_dn8 = assign48910_e62723_d_n8;
        locals.var_xn_s__blk1349_dn9 = assign48910_e62723_d_n9;

        let assign48920_e62726: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1478 = assign48920_e62726;

        let assign48930_e62728: f64 = (locals.var_xn_s__blk1349).abs();
        let assign48930_e62730: f64 = if assign48930_e62728 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1479 = assign48930_e62730;

        let (assign48940_e62754, assign48940_e62754_d_n4, assign48940_e62754_d_n6, assign48940_e62754_d_n7, assign48940_e62754_d_n8, assign48940_e62754_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 != 0.0)) {
        let assign48940_e62743: f64 = (0.5 * locals.var_xn_s__blk1349);
        let assign48940_e62747: f64 = (0.3125 * locals.var_xn_s__blk1349);
        let assign48940_e62748: f64 = (1.0 - assign48940_e62747);
        let assign48940_e62749: f64 = (assign48940_e62743 * assign48940_e62748);
        let assign48940_e62750: f64 = (1.0 - assign48940_e62749);
        let assign48940_e62751: f64 = (locals.var_gf__blk1324 * assign48940_e62750);
        let assign48940_e62752: f64 = (1.0 + assign48940_e62751);
        (assign48940_e62752, ((locals.var_gf__blk1324_dn4 * assign48940_e62750) + (locals.var_gf__blk1324 * (-(((0.5 * locals.var_xn_s__blk1349_dn4) * assign48940_e62748) + (assign48940_e62743 * (-(0.3125 * locals.var_xn_s__blk1349_dn4))))))), ((locals.var_gf__blk1324_dn6 * assign48940_e62750) + (locals.var_gf__blk1324 * (-(((0.5 * locals.var_xn_s__blk1349_dn6) * assign48940_e62748) + (assign48940_e62743 * (-(0.3125 * locals.var_xn_s__blk1349_dn6))))))), ((locals.var_gf__blk1324_dn7 * assign48940_e62750) + (locals.var_gf__blk1324 * (-(((0.5 * locals.var_xn_s__blk1349_dn7) * assign48940_e62748) + (assign48940_e62743 * (-(0.3125 * locals.var_xn_s__blk1349_dn7))))))), ((locals.var_gf__blk1324_dn8 * assign48940_e62750) + (locals.var_gf__blk1324 * (-(((0.5 * locals.var_xn_s__blk1349_dn8) * assign48940_e62748) + (assign48940_e62743 * (-(0.3125 * locals.var_xn_s__blk1349_dn8))))))), ((locals.var_gf__blk1324_dn9 * assign48940_e62750) + (locals.var_gf__blk1324 * (-(((0.5 * locals.var_xn_s__blk1349_dn9) * assign48940_e62748) + (assign48940_e62743 * (-(0.3125 * locals.var_xn_s__blk1349_dn9))))))),)
    } else {
        (locals.var_nscr__blk1350, locals.var_nscr__blk1350_dn4, locals.var_nscr__blk1350_dn6, locals.var_nscr__blk1350_dn7, locals.var_nscr__blk1350_dn8, locals.var_nscr__blk1350_dn9,)
    }
};
        locals.var_nscr__blk1350 = assign48940_e62754;
        locals.var_nscr__blk1350_dn4 = assign48940_e62754_d_n4;
        locals.var_nscr__blk1350_dn6 = assign48940_e62754_d_n6;
        locals.var_nscr__blk1350_dn7 = assign48940_e62754_d_n7;
        locals.var_nscr__blk1350_dn8 = assign48940_e62754_d_n8;
        locals.var_nscr__blk1350_dn9 = assign48940_e62754_d_n9;

        let assign48950_e62757: f64 = if locals.var_xn_s__blk1349 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1480 = assign48950_e62757;

        let (assign48960_e62772, assign48960_e62772_d_n4, assign48960_e62772_d_n6, assign48960_e62772_d_n7, assign48960_e62772_d_n8, assign48960_e62772_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 == 0.0)) && (locals.var_guard1480 != 0.0)) {
        let assign48960_e62769: f64 = (-locals.var_xn_s__blk1349);
        let assign48960_e62770: f64 = (assign48960_e62769).exp();
        (assign48960_e62770, (assign48960_e62770 * (-locals.var_xn_s__blk1349_dn4)), (assign48960_e62770 * (-locals.var_xn_s__blk1349_dn6)), (assign48960_e62770 * (-locals.var_xn_s__blk1349_dn7)), (assign48960_e62770 * (-locals.var_xn_s__blk1349_dn8)), (assign48960_e62770 * (-locals.var_xn_s__blk1349_dn9)),)
    } else {
        (locals.var_delta_ns__blk1364, locals.var_delta_ns__blk1364_dn4, locals.var_delta_ns__blk1364_dn6, locals.var_delta_ns__blk1364_dn7, locals.var_delta_ns__blk1364_dn8, locals.var_delta_ns__blk1364_dn9,)
    }
};
        locals.var_delta_ns__blk1364 = assign48960_e62772;
        locals.var_delta_ns__blk1364_dn4 = assign48960_e62772_d_n4;
        locals.var_delta_ns__blk1364_dn6 = assign48960_e62772_d_n6;
        locals.var_delta_ns__blk1364_dn7 = assign48960_e62772_d_n7;
        locals.var_delta_ns__blk1364_dn8 = assign48960_e62772_d_n8;
        locals.var_delta_ns__blk1364_dn9 = assign48960_e62772_d_n9;

    }

    pub(super) fn stamp_transient_block_37(
        locals: &mut StampLocals,
    ) {
        let (assign48970_e62808, assign48970_e62808_d_n4, assign48970_e62808_d_n6, assign48970_e62808_d_n7, assign48970_e62808_d_n8, assign48970_e62808_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 == 0.0)) && (locals.var_guard1480 == 0.0)) {
        let assign48970_e62788: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
        let assign48970_e62793: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
        let assign48970_e62797: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
        let assign48970_e62799: f64 = (assign48970_e62797 * 0.3333333333333333);
        let assign48970_e62800: f64 = (1.0 + assign48970_e62799);
        let assign48970_e62801: f64 = (assign48970_e62793 * assign48970_e62800);
        let assign48970_e62802: f64 = (0.5 * assign48970_e62801);
        let assign48970_e62803: f64 = (1.0 + assign48970_e62802);
        let assign48970_e62804: f64 = (assign48970_e62788 * assign48970_e62803);
        let assign48970_e62805: f64 = (1.0 + assign48970_e62804);
        let assign48970_e62806: f64 = (1e-200 / assign48970_e62805);
        (assign48970_e62806, (-((1e-200 * ((locals.var_xn_s__blk1349_dn4 * assign48970_e62803) + (assign48970_e62788 * (0.5 * ((locals.var_xn_s__blk1349_dn4 * assign48970_e62800) + (assign48970_e62793 * (locals.var_xn_s__blk1349_dn4 * 0.3333333333333333))))))) / (assign48970_e62805 * assign48970_e62805))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn6 * assign48970_e62803) + (assign48970_e62788 * (0.5 * ((locals.var_xn_s__blk1349_dn6 * assign48970_e62800) + (assign48970_e62793 * (locals.var_xn_s__blk1349_dn6 * 0.3333333333333333))))))) / (assign48970_e62805 * assign48970_e62805))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn7 * assign48970_e62803) + (assign48970_e62788 * (0.5 * ((locals.var_xn_s__blk1349_dn7 * assign48970_e62800) + (assign48970_e62793 * (locals.var_xn_s__blk1349_dn7 * 0.3333333333333333))))))) / (assign48970_e62805 * assign48970_e62805))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn8 * assign48970_e62803) + (assign48970_e62788 * (0.5 * ((locals.var_xn_s__blk1349_dn8 * assign48970_e62800) + (assign48970_e62793 * (locals.var_xn_s__blk1349_dn8 * 0.3333333333333333))))))) / (assign48970_e62805 * assign48970_e62805))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn9 * assign48970_e62803) + (assign48970_e62788 * (0.5 * ((locals.var_xn_s__blk1349_dn9 * assign48970_e62800) + (assign48970_e62793 * (locals.var_xn_s__blk1349_dn9 * 0.3333333333333333))))))) / (assign48970_e62805 * assign48970_e62805))),)
    } else {
        (locals.var_delta_ns__blk1364, locals.var_delta_ns__blk1364_dn4, locals.var_delta_ns__blk1364_dn6, locals.var_delta_ns__blk1364_dn7, locals.var_delta_ns__blk1364_dn8, locals.var_delta_ns__blk1364_dn9,)
    }
};
        locals.var_delta_ns__blk1364 = assign48970_e62808;
        locals.var_delta_ns__blk1364_dn4 = assign48970_e62808_d_n4;
        locals.var_delta_ns__blk1364_dn6 = assign48970_e62808_d_n6;
        locals.var_delta_ns__blk1364_dn7 = assign48970_e62808_d_n7;
        locals.var_delta_ns__blk1364_dn8 = assign48970_e62808_d_n8;
        locals.var_delta_ns__blk1364_dn9 = assign48970_e62808_d_n9;

        let (assign48980_e62825, assign48980_e62825_d_n4, assign48980_e62825_d_n6, assign48980_e62825_d_n7, assign48980_e62825_d_n8, assign48980_e62825_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 == 0.0)) {
        let (assign48980_e62823,) = {
            if (locals.var_xn_s__blk1349 > 0.0) {
                (1.0,)
            } else {
                let assign48980_e62822: f64 = (-1.0);
                (assign48980_e62822,)
            }
        };
        (assign48980_e62823, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign48980_e62825;
        locals.var_temp__blk949_dn4 = assign48980_e62825_d_n4;
        locals.var_temp__blk949_dn6 = assign48980_e62825_d_n6;
        locals.var_temp__blk949_dn7 = assign48980_e62825_d_n7;
        locals.var_temp__blk949_dn8 = assign48980_e62825_d_n8;
        locals.var_temp__blk949_dn9 = assign48980_e62825_d_n9;

        let (assign48990_e62857, assign48990_e62857_d_n4, assign48990_e62857_d_n6, assign48990_e62857_d_n7, assign48990_e62857_d_n8, assign48990_e62857_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 == 0.0)) {
        let assign48990_e62837: f64 = (locals.var_temp__blk949 * locals.var_gf__blk1324);
        let assign48990_e62842: f64 = (1.0 - locals.var_xn_s__blk1349);
        let assign48990_e62843: f64 = (locals.var_delta_ns__blk1364 * assign48990_e62842);
        let assign48990_e62844: f64 = (1.0 - assign48990_e62843);
        let assign48990_e62845: f64 = (assign48990_e62837 * assign48990_e62844);
        let assign48990_e62850: f64 = (1.0 - locals.var_delta_ns__blk1364);
        let assign48990_e62851: f64 = (locals.var_xn_s__blk1349 * assign48990_e62850);
        let assign48990_e62852: f64 = (assign48990_e62851).sqrt();
        let assign48990_e62853: f64 = (2.0 * assign48990_e62852);
        let assign48990_e62854: f64 = (assign48990_e62845 / assign48990_e62853);
        let assign48990_e62855: f64 = (1.0 + assign48990_e62854);
        (assign48990_e62855, (((((((locals.var_temp__blk949_dn4 * locals.var_gf__blk1324) + (locals.var_temp__blk949 * locals.var_gf__blk1324_dn4)) * assign48990_e62844) + (assign48990_e62837 * (-((locals.var_delta_ns__blk1364_dn4 * assign48990_e62842) + (locals.var_delta_ns__blk1364 * (-locals.var_xn_s__blk1349_dn4)))))) * assign48990_e62853) - (assign48990_e62845 * (2.0 * (((locals.var_xn_s__blk1349_dn4 * assign48990_e62850) + (locals.var_xn_s__blk1349 * (-locals.var_delta_ns__blk1364_dn4))) / (2.0 * assign48990_e62852))))) / (assign48990_e62853 * assign48990_e62853)), (((((((locals.var_temp__blk949_dn6 * locals.var_gf__blk1324) + (locals.var_temp__blk949 * locals.var_gf__blk1324_dn6)) * assign48990_e62844) + (assign48990_e62837 * (-((locals.var_delta_ns__blk1364_dn6 * assign48990_e62842) + (locals.var_delta_ns__blk1364 * (-locals.var_xn_s__blk1349_dn6)))))) * assign48990_e62853) - (assign48990_e62845 * (2.0 * (((locals.var_xn_s__blk1349_dn6 * assign48990_e62850) + (locals.var_xn_s__blk1349 * (-locals.var_delta_ns__blk1364_dn6))) / (2.0 * assign48990_e62852))))) / (assign48990_e62853 * assign48990_e62853)), (((((((locals.var_temp__blk949_dn7 * locals.var_gf__blk1324) + (locals.var_temp__blk949 * locals.var_gf__blk1324_dn7)) * assign48990_e62844) + (assign48990_e62837 * (-((locals.var_delta_ns__blk1364_dn7 * assign48990_e62842) + (locals.var_delta_ns__blk1364 * (-locals.var_xn_s__blk1349_dn7)))))) * assign48990_e62853) - (assign48990_e62845 * (2.0 * (((locals.var_xn_s__blk1349_dn7 * assign48990_e62850) + (locals.var_xn_s__blk1349 * (-locals.var_delta_ns__blk1364_dn7))) / (2.0 * assign48990_e62852))))) / (assign48990_e62853 * assign48990_e62853)), (((((((locals.var_temp__blk949_dn8 * locals.var_gf__blk1324) + (locals.var_temp__blk949 * locals.var_gf__blk1324_dn8)) * assign48990_e62844) + (assign48990_e62837 * (-((locals.var_delta_ns__blk1364_dn8 * assign48990_e62842) + (locals.var_delta_ns__blk1364 * (-locals.var_xn_s__blk1349_dn8)))))) * assign48990_e62853) - (assign48990_e62845 * (2.0 * (((locals.var_xn_s__blk1349_dn8 * assign48990_e62850) + (locals.var_xn_s__blk1349 * (-locals.var_delta_ns__blk1364_dn8))) / (2.0 * assign48990_e62852))))) / (assign48990_e62853 * assign48990_e62853)), (((((((locals.var_temp__blk949_dn9 * locals.var_gf__blk1324) + (locals.var_temp__blk949 * locals.var_gf__blk1324_dn9)) * assign48990_e62844) + (assign48990_e62837 * (-((locals.var_delta_ns__blk1364_dn9 * assign48990_e62842) + (locals.var_delta_ns__blk1364 * (-locals.var_xn_s__blk1349_dn9)))))) * assign48990_e62853) - (assign48990_e62845 * (2.0 * (((locals.var_xn_s__blk1349_dn9 * assign48990_e62850) + (locals.var_xn_s__blk1349 * (-locals.var_delta_ns__blk1364_dn9))) / (2.0 * assign48990_e62852))))) / (assign48990_e62853 * assign48990_e62853)),)
    } else {
        (locals.var_nscr__blk1350, locals.var_nscr__blk1350_dn4, locals.var_nscr__blk1350_dn6, locals.var_nscr__blk1350_dn7, locals.var_nscr__blk1350_dn8, locals.var_nscr__blk1350_dn9,)
    }
};
        locals.var_nscr__blk1350 = assign48990_e62857;
        locals.var_nscr__blk1350_dn4 = assign48990_e62857_d_n4;
        locals.var_nscr__blk1350_dn6 = assign48990_e62857_d_n6;
        locals.var_nscr__blk1350_dn7 = assign48990_e62857_d_n7;
        locals.var_nscr__blk1350_dn8 = assign48990_e62857_d_n8;
        locals.var_nscr__blk1350_dn9 = assign48990_e62857_d_n9;

        let (assign49000_e62873, assign49000_e62873_d_n4, assign49000_e62873_d_n6, assign49000_e62873_d_n7, assign49000_e62873_d_n8, assign49000_e62873_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 == 0.0)) {
        let assign49000_e62867: f64 = (0.5 * locals.var_gf__blk1324);
        let assign49000_e62869: f64 = (locals.var_xn_s__blk1349).sqrt();
        let assign49000_e62870: f64 = (assign49000_e62867 / assign49000_e62869);
        let assign49000_e62871: f64 = (1.0 + assign49000_e62870);
        (assign49000_e62871, ((((0.5 * locals.var_gf__blk1324_dn4) * assign49000_e62869) - (assign49000_e62867 * (locals.var_xn_s__blk1349_dn4 / (2.0 * assign49000_e62869)))) / (assign49000_e62869 * assign49000_e62869)), ((((0.5 * locals.var_gf__blk1324_dn6) * assign49000_e62869) - (assign49000_e62867 * (locals.var_xn_s__blk1349_dn6 / (2.0 * assign49000_e62869)))) / (assign49000_e62869 * assign49000_e62869)), ((((0.5 * locals.var_gf__blk1324_dn7) * assign49000_e62869) - (assign49000_e62867 * (locals.var_xn_s__blk1349_dn7 / (2.0 * assign49000_e62869)))) / (assign49000_e62869 * assign49000_e62869)), ((((0.5 * locals.var_gf__blk1324_dn8) * assign49000_e62869) - (assign49000_e62867 * (locals.var_xn_s__blk1349_dn8 / (2.0 * assign49000_e62869)))) / (assign49000_e62869 * assign49000_e62869)), ((((0.5 * locals.var_gf__blk1324_dn9) * assign49000_e62869) - (assign49000_e62867 * (locals.var_xn_s__blk1349_dn9 / (2.0 * assign49000_e62869)))) / (assign49000_e62869 * assign49000_e62869)),)
    } else {
        (locals.var_nscr__blk1350, locals.var_nscr__blk1350_dn4, locals.var_nscr__blk1350_dn6, locals.var_nscr__blk1350_dn7, locals.var_nscr__blk1350_dn8, locals.var_nscr__blk1350_dn9,)
    }
};
        locals.var_nscr__blk1350 = assign49000_e62873;
        locals.var_nscr__blk1350_dn4 = assign49000_e62873_d_n4;
        locals.var_nscr__blk1350_dn6 = assign49000_e62873_d_n6;
        locals.var_nscr__blk1350_dn7 = assign49000_e62873_d_n7;
        locals.var_nscr__blk1350_dn8 = assign49000_e62873_d_n8;
        locals.var_nscr__blk1350_dn9 = assign49000_e62873_d_n9;

        let (assign49010_e62891, assign49010_e62891_d_n4, assign49010_e62891_d_n6, assign49010_e62891_d_n7, assign49010_e62891_d_n8, assign49010_e62891_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign49010_e62880: f64 = (locals.var_xn_s__blk1349).sqrt();
        let assign49010_e62881: f64 = (locals.var_gf__blk1324 * assign49010_e62880);
        let assign49010_e62882: f64 = (locals.var_xn_s__blk1349 + assign49010_e62881);
        let assign49010_e62886: f64 = (locals.var_nscr__blk1350 - 1.0);
        let assign49010_e62887: f64 = (assign49010_e62886).ln();
        let assign49010_e62888: f64 = (locals.var_nscr__blk1350 * assign49010_e62887);
        let assign49010_e62889: f64 = (assign49010_e62882 - assign49010_e62888);
        (assign49010_e62889, ((locals.var_xn_s__blk1349_dn4 + ((locals.var_gf__blk1324_dn4 * assign49010_e62880) + (locals.var_gf__blk1324 * (locals.var_xn_s__blk1349_dn4 / (2.0 * assign49010_e62880))))) - ((locals.var_nscr__blk1350_dn4 * assign49010_e62887) + (locals.var_nscr__blk1350 * (locals.var_nscr__blk1350_dn4 / assign49010_e62886)))), ((locals.var_xn_s__blk1349_dn6 + ((locals.var_gf__blk1324_dn6 * assign49010_e62880) + (locals.var_gf__blk1324 * (locals.var_xn_s__blk1349_dn6 / (2.0 * assign49010_e62880))))) - ((locals.var_nscr__blk1350_dn6 * assign49010_e62887) + (locals.var_nscr__blk1350 * (locals.var_nscr__blk1350_dn6 / assign49010_e62886)))), ((locals.var_xn_s__blk1349_dn7 + ((locals.var_gf__blk1324_dn7 * assign49010_e62880) + (locals.var_gf__blk1324 * (locals.var_xn_s__blk1349_dn7 / (2.0 * assign49010_e62880))))) - ((locals.var_nscr__blk1350_dn7 * assign49010_e62887) + (locals.var_nscr__blk1350 * (locals.var_nscr__blk1350_dn7 / assign49010_e62886)))), ((locals.var_xn_s__blk1349_dn8 + ((locals.var_gf__blk1324_dn8 * assign49010_e62880) + (locals.var_gf__blk1324 * (locals.var_xn_s__blk1349_dn8 / (2.0 * assign49010_e62880))))) - ((locals.var_nscr__blk1350_dn8 * assign49010_e62887) + (locals.var_nscr__blk1350 * (locals.var_nscr__blk1350_dn8 / assign49010_e62886)))), ((locals.var_xn_s__blk1349_dn9 + ((locals.var_gf__blk1324_dn9 * assign49010_e62880) + (locals.var_gf__blk1324 * (locals.var_xn_s__blk1349_dn9 / (2.0 * assign49010_e62880))))) - ((locals.var_nscr__blk1350_dn9 * assign49010_e62887) + (locals.var_nscr__blk1350 * (locals.var_nscr__blk1350_dn9 / assign49010_e62886)))),)
    } else {
        (locals.var_xthscr__blk1351, locals.var_xthscr__blk1351_dn4, locals.var_xthscr__blk1351_dn6, locals.var_xthscr__blk1351_dn7, locals.var_xthscr__blk1351_dn8, locals.var_xthscr__blk1351_dn9,)
    }
};
        locals.var_xthscr__blk1351 = assign49010_e62891;
        locals.var_xthscr__blk1351_dn4 = assign49010_e62891_d_n4;
        locals.var_xthscr__blk1351_dn6 = assign49010_e62891_d_n6;
        locals.var_xthscr__blk1351_dn7 = assign49010_e62891_d_n7;
        locals.var_xthscr__blk1351_dn8 = assign49010_e62891_d_n8;
        locals.var_xthscr__blk1351_dn9 = assign49010_e62891_d_n9;

        let (assign49020_e62901, assign49020_e62901_d_n4, assign49020_e62901_d_n6, assign49020_e62901_d_n7, assign49020_e62901_d_n8, assign49020_e62901_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign49020_e62897: f64 = (locals.var_xg__blk1343 - locals.var_xthscr__blk1351);
        let assign49020_e62899: f64 = (assign49020_e62897 / locals.var_nscr__blk1350);
        (assign49020_e62899, ((((locals.var_xg__blk1343_dn4 - locals.var_xthscr__blk1351_dn4) * locals.var_nscr__blk1350) - (assign49020_e62897 * locals.var_nscr__blk1350_dn4)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), ((((locals.var_xg__blk1343_dn6 - locals.var_xthscr__blk1351_dn6) * locals.var_nscr__blk1350) - (assign49020_e62897 * locals.var_nscr__blk1350_dn6)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), ((((locals.var_xg__blk1343_dn7 - locals.var_xthscr__blk1351_dn7) * locals.var_nscr__blk1350) - (assign49020_e62897 * locals.var_nscr__blk1350_dn7)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), ((((locals.var_xg__blk1343_dn8 - locals.var_xthscr__blk1351_dn8) * locals.var_nscr__blk1350) - (assign49020_e62897 * locals.var_nscr__blk1350_dn8)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), ((((locals.var_xg__blk1343_dn9 - locals.var_xthscr__blk1351_dn9) * locals.var_nscr__blk1350) - (assign49020_e62897 * locals.var_nscr__blk1350_dn9)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)),)
    } else {
        (locals.var_xgtscr__blk1352, locals.var_xgtscr__blk1352_dn4, locals.var_xgtscr__blk1352_dn6, locals.var_xgtscr__blk1352_dn7, locals.var_xgtscr__blk1352_dn8, locals.var_xgtscr__blk1352_dn9,)
    }
};
        locals.var_xgtscr__blk1352 = assign49020_e62901;
        locals.var_xgtscr__blk1352_dn4 = assign49020_e62901_d_n4;
        locals.var_xgtscr__blk1352_dn6 = assign49020_e62901_d_n6;
        locals.var_xgtscr__blk1352_dn7 = assign49020_e62901_d_n7;
        locals.var_xgtscr__blk1352_dn8 = assign49020_e62901_d_n8;
        locals.var_xgtscr__blk1352_dn9 = assign49020_e62901_d_n9;

        let (assign49030_e62918, assign49030_e62918_d_n4, assign49030_e62918_d_n6, assign49030_e62918_d_n7, assign49030_e62918_d_n8, assign49030_e62918_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign49030_e62907: f64 = (0.5 * locals.var_gf2__blk1325);
        let assign49030_e62911: f64 = (8.0 / locals.var_gf2__blk1325);
        let assign49030_e62912: f64 = (1.0 + assign49030_e62911);
        let assign49030_e62913: f64 = (assign49030_e62912).sqrt();
        let assign49030_e62915: f64 = (assign49030_e62913 - 1.0);
        let assign49030_e62916: f64 = (assign49030_e62907 * assign49030_e62915);
        (assign49030_e62916, (((0.5 * locals.var_gf2__blk1325_dn4) * assign49030_e62915) + (assign49030_e62907 * ((-((8.0 * locals.var_gf2__blk1325_dn4) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) / (2.0 * assign49030_e62913)))), (((0.5 * locals.var_gf2__blk1325_dn6) * assign49030_e62915) + (assign49030_e62907 * ((-((8.0 * locals.var_gf2__blk1325_dn6) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) / (2.0 * assign49030_e62913)))), (((0.5 * locals.var_gf2__blk1325_dn7) * assign49030_e62915) + (assign49030_e62907 * ((-((8.0 * locals.var_gf2__blk1325_dn7) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) / (2.0 * assign49030_e62913)))), (((0.5 * locals.var_gf2__blk1325_dn8) * assign49030_e62915) + (assign49030_e62907 * ((-((8.0 * locals.var_gf2__blk1325_dn8) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) / (2.0 * assign49030_e62913)))), (((0.5 * locals.var_gf2__blk1325_dn9) * assign49030_e62915) + (assign49030_e62907 * ((-((8.0 * locals.var_gf2__blk1325_dn9) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) / (2.0 * assign49030_e62913)))),)
    } else {
        (locals.var_qbscr__blk1358, locals.var_qbscr__blk1358_dn4, locals.var_qbscr__blk1358_dn6, locals.var_qbscr__blk1358_dn7, locals.var_qbscr__blk1358_dn8, locals.var_qbscr__blk1358_dn9,)
    }
};
        locals.var_qbscr__blk1358 = assign49030_e62918;
        locals.var_qbscr__blk1358_dn4 = assign49030_e62918_d_n4;
        locals.var_qbscr__blk1358_dn6 = assign49030_e62918_d_n6;
        locals.var_qbscr__blk1358_dn7 = assign49030_e62918_d_n7;
        locals.var_qbscr__blk1358_dn8 = assign49030_e62918_d_n8;
        locals.var_qbscr__blk1358_dn9 = assign49030_e62918_d_n9;

        let (assign49040_e62924, assign49040_e62924_d_n4, assign49040_e62924_d_n6, assign49040_e62924_d_n7, assign49040_e62924_d_n8, assign49040_e62924_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qiscr__blk1357, locals.var_qiscr__blk1357_dn4, locals.var_qiscr__blk1357_dn6, locals.var_qiscr__blk1357_dn7, locals.var_qiscr__blk1357_dn8, locals.var_qiscr__blk1357_dn9,)
    }
};
        locals.var_qiscr__blk1357 = assign49040_e62924;
        locals.var_qiscr__blk1357_dn4 = assign49040_e62924_d_n4;
        locals.var_qiscr__blk1357_dn6 = assign49040_e62924_d_n6;
        locals.var_qiscr__blk1357_dn7 = assign49040_e62924_d_n7;
        locals.var_qiscr__blk1357_dn8 = assign49040_e62924_d_n8;
        locals.var_qiscr__blk1357_dn9 = assign49040_e62924_d_n9;

        let (assign49050_e62930, assign49050_e62930_d_n4, assign49050_e62930_d_n6, assign49050_e62930_d_n7, assign49050_e62930_d_n8, assign49050_e62930_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fscr__blk1359, locals.var_fscr__blk1359_dn4, locals.var_fscr__blk1359_dn6, locals.var_fscr__blk1359_dn7, locals.var_fscr__blk1359_dn8, locals.var_fscr__blk1359_dn9,)
    }
};
        locals.var_fscr__blk1359 = assign49050_e62930;
        locals.var_fscr__blk1359_dn4 = assign49050_e62930_d_n4;
        locals.var_fscr__blk1359_dn6 = assign49050_e62930_d_n6;
        locals.var_fscr__blk1359_dn7 = assign49050_e62930_d_n7;
        locals.var_fscr__blk1359_dn8 = assign49050_e62930_d_n8;
        locals.var_fscr__blk1359_dn9 = assign49050_e62930_d_n9;

        let assign49060_e62933: f64 = (-30.0);
        let assign49060_e62934: f64 = if locals.var_xgtscr__blk1352 > assign49060_e62933 { 1.0 } else { 0.0 };
        locals.var_guard1481 = assign49060_e62934;

        let (assign49070_e62946, assign49070_e62946_d_n4, assign49070_e62946_d_n6, assign49070_e62946_d_n7, assign49070_e62946_d_n8, assign49070_e62946_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49070_e62942: f64 = (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352);
        let assign49070_e62944: f64 = (assign49070_e62942 - 1.0);
        (assign49070_e62944, ((locals.var_nscr__blk1350_dn4 * locals.var_xgtscr__blk1352) + (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352_dn4)), ((locals.var_nscr__blk1350_dn6 * locals.var_xgtscr__blk1352) + (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352_dn6)), ((locals.var_nscr__blk1350_dn7 * locals.var_xgtscr__blk1352) + (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352_dn7)), ((locals.var_nscr__blk1350_dn8 * locals.var_xgtscr__blk1352) + (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352_dn8)), ((locals.var_nscr__blk1350_dn9 * locals.var_xgtscr__blk1352) + (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352_dn9)),)
    } else {
        (locals.var_xgtscr0__blk1353, locals.var_xgtscr0__blk1353_dn4, locals.var_xgtscr0__blk1353_dn6, locals.var_xgtscr0__blk1353_dn7, locals.var_xgtscr0__blk1353_dn8, locals.var_xgtscr0__blk1353_dn9,)
    }
};
        locals.var_xgtscr0__blk1353 = assign49070_e62946;
        locals.var_xgtscr0__blk1353_dn4 = assign49070_e62946_d_n4;
        locals.var_xgtscr0__blk1353_dn6 = assign49070_e62946_d_n6;
        locals.var_xgtscr0__blk1353_dn7 = assign49070_e62946_d_n7;
        locals.var_xgtscr0__blk1353_dn8 = assign49070_e62946_d_n8;
        locals.var_xgtscr0__blk1353_dn9 = assign49070_e62946_d_n9;

        let (assign49080_e62963, assign49080_e62963_d_n4, assign49080_e62963_d_n6, assign49080_e62963_d_n7, assign49080_e62963_d_n8, assign49080_e62963_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49080_e62956: f64 = (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353);
        let assign49080_e62958: f64 = (assign49080_e62956 + 10.0);
        let assign49080_e62959: f64 = (assign49080_e62958).sqrt();
        let assign49080_e62960: f64 = (locals.var_xgtscr0__blk1353 + assign49080_e62959);
        let assign49080_e62961: f64 = (0.5 * assign49080_e62960);
        (assign49080_e62961, (0.5 * (locals.var_xgtscr0__blk1353_dn4 + (((locals.var_xgtscr0__blk1353_dn4 * locals.var_xgtscr0__blk1353) + (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353_dn4)) / (2.0 * assign49080_e62959)))), (0.5 * (locals.var_xgtscr0__blk1353_dn6 + (((locals.var_xgtscr0__blk1353_dn6 * locals.var_xgtscr0__blk1353) + (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353_dn6)) / (2.0 * assign49080_e62959)))), (0.5 * (locals.var_xgtscr0__blk1353_dn7 + (((locals.var_xgtscr0__blk1353_dn7 * locals.var_xgtscr0__blk1353) + (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353_dn7)) / (2.0 * assign49080_e62959)))), (0.5 * (locals.var_xgtscr0__blk1353_dn8 + (((locals.var_xgtscr0__blk1353_dn8 * locals.var_xgtscr0__blk1353) + (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353_dn8)) / (2.0 * assign49080_e62959)))), (0.5 * (locals.var_xgtscr0__blk1353_dn9 + (((locals.var_xgtscr0__blk1353_dn9 * locals.var_xgtscr0__blk1353) + (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353_dn9)) / (2.0 * assign49080_e62959)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign49080_e62963;
        locals.var_temp__blk949_dn4 = assign49080_e62963_d_n4;
        locals.var_temp__blk949_dn6 = assign49080_e62963_d_n6;
        locals.var_temp__blk949_dn7 = assign49080_e62963_d_n7;
        locals.var_temp__blk949_dn8 = assign49080_e62963_d_n8;
        locals.var_temp__blk949_dn9 = assign49080_e62963_d_n9;

        let (assign49090_e62974, assign49090_e62974_d_n4, assign49090_e62974_d_n6, assign49090_e62974_d_n7, assign49090_e62974_d_n8, assign49090_e62974_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49090_e62971: f64 = (locals.var_temp__blk949).ln();
        let assign49090_e62972: f64 = (locals.var_xgtscr__blk1352 - assign49090_e62971);
        (assign49090_e62972, (locals.var_xgtscr__blk1352_dn4 - (locals.var_temp__blk949_dn4 / locals.var_temp__blk949)), (locals.var_xgtscr__blk1352_dn6 - (locals.var_temp__blk949_dn6 / locals.var_temp__blk949)), (locals.var_xgtscr__blk1352_dn7 - (locals.var_temp__blk949_dn7 / locals.var_temp__blk949)), (locals.var_xgtscr__blk1352_dn8 - (locals.var_temp__blk949_dn8 / locals.var_temp__blk949)), (locals.var_xgtscr__blk1352_dn9 - (locals.var_temp__blk949_dn9 / locals.var_temp__blk949)),)
    } else {
        (locals.var_qiscr0si__blk1354, locals.var_qiscr0si__blk1354_dn4, locals.var_qiscr0si__blk1354_dn6, locals.var_qiscr0si__blk1354_dn7, locals.var_qiscr0si__blk1354_dn8, locals.var_qiscr0si__blk1354_dn9,)
    }
};
        locals.var_qiscr0si__blk1354 = assign49090_e62974;
        locals.var_qiscr0si__blk1354_dn4 = assign49090_e62974_d_n4;
        locals.var_qiscr0si__blk1354_dn6 = assign49090_e62974_d_n6;
        locals.var_qiscr0si__blk1354_dn7 = assign49090_e62974_d_n7;
        locals.var_qiscr0si__blk1354_dn8 = assign49090_e62974_d_n8;
        locals.var_qiscr0si__blk1354_dn9 = assign49090_e62974_d_n9;

        let (assign49100_e62991, assign49100_e62991_d_n4, assign49100_e62991_d_n6, assign49100_e62991_d_n7, assign49100_e62991_d_n8, assign49100_e62991_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49100_e62984: f64 = (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354);
        let assign49100_e62986: f64 = (assign49100_e62984 + 2.0);
        let assign49100_e62987: f64 = (assign49100_e62986).sqrt();
        let assign49100_e62988: f64 = (locals.var_qiscr0si__blk1354 + assign49100_e62987);
        let assign49100_e62989: f64 = (0.5 * assign49100_e62988);
        (assign49100_e62989, (0.5 * (locals.var_qiscr0si__blk1354_dn4 + (((locals.var_qiscr0si__blk1354_dn4 * locals.var_qiscr0si__blk1354) + (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354_dn4)) / (2.0 * assign49100_e62987)))), (0.5 * (locals.var_qiscr0si__blk1354_dn6 + (((locals.var_qiscr0si__blk1354_dn6 * locals.var_qiscr0si__blk1354) + (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354_dn6)) / (2.0 * assign49100_e62987)))), (0.5 * (locals.var_qiscr0si__blk1354_dn7 + (((locals.var_qiscr0si__blk1354_dn7 * locals.var_qiscr0si__blk1354) + (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354_dn7)) / (2.0 * assign49100_e62987)))), (0.5 * (locals.var_qiscr0si__blk1354_dn8 + (((locals.var_qiscr0si__blk1354_dn8 * locals.var_qiscr0si__blk1354) + (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354_dn8)) / (2.0 * assign49100_e62987)))), (0.5 * (locals.var_qiscr0si__blk1354_dn9 + (((locals.var_qiscr0si__blk1354_dn9 * locals.var_qiscr0si__blk1354) + (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354_dn9)) / (2.0 * assign49100_e62987)))),)
    } else {
        (locals.var_qiscr0__blk1355, locals.var_qiscr0__blk1355_dn4, locals.var_qiscr0__blk1355_dn6, locals.var_qiscr0__blk1355_dn7, locals.var_qiscr0__blk1355_dn8, locals.var_qiscr0__blk1355_dn9,)
    }
};
        locals.var_qiscr0__blk1355 = assign49100_e62991;
        locals.var_qiscr0__blk1355_dn4 = assign49100_e62991_d_n4;
        locals.var_qiscr0__blk1355_dn6 = assign49100_e62991_d_n6;
        locals.var_qiscr0__blk1355_dn7 = assign49100_e62991_d_n7;
        locals.var_qiscr0__blk1355_dn8 = assign49100_e62991_d_n8;
        locals.var_qiscr0__blk1355_dn9 = assign49100_e62991_d_n9;

        let assign49110_e62994: f64 = (locals.var_xgtscr__blk1352 - locals.var_qiscr0__blk1355);
        let assign49110_e62996: f64 = if assign49110_e62994 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1482 = assign49110_e62996;

        let (assign49120_e63009, assign49120_e63009_d_n4, assign49120_e63009_d_n6, assign49120_e63009_d_n7, assign49120_e63009_d_n8, assign49120_e63009_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) && (locals.var_guard1482 != 0.0)) {
        let assign49120_e63006: f64 = (locals.var_xgtscr__blk1352 - locals.var_qiscr0__blk1355);
        let assign49120_e63007: f64 = (assign49120_e63006).exp();
        (assign49120_e63007, (assign49120_e63007 * (locals.var_xgtscr__blk1352_dn4 - locals.var_qiscr0__blk1355_dn4)), (assign49120_e63007 * (locals.var_xgtscr__blk1352_dn6 - locals.var_qiscr0__blk1355_dn6)), (assign49120_e63007 * (locals.var_xgtscr__blk1352_dn7 - locals.var_qiscr0__blk1355_dn7)), (assign49120_e63007 * (locals.var_xgtscr__blk1352_dn8 - locals.var_qiscr0__blk1355_dn8)), (assign49120_e63007 * (locals.var_xgtscr__blk1352_dn9 - locals.var_qiscr0__blk1355_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign49120_e63009;
        locals.var_temp__blk949_dn4 = assign49120_e63009_d_n4;
        locals.var_temp__blk949_dn6 = assign49120_e63009_d_n6;
        locals.var_temp__blk949_dn7 = assign49120_e63009_d_n7;
        locals.var_temp__blk949_dn8 = assign49120_e63009_d_n8;
        locals.var_temp__blk949_dn9 = assign49120_e63009_d_n9;

        let (assign49130_e63048, assign49130_e63048_d_n4, assign49130_e63048_d_n6, assign49130_e63048_d_n7, assign49130_e63048_d_n8, assign49130_e63048_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) && (locals.var_guard1482 == 0.0)) {
        let assign49130_e63022: f64 = (locals.var_xgtscr__blk1352 - locals.var_qiscr0__blk1355);
        let assign49130_e63024: f64 = (assign49130_e63022 - 230.25850929940458);
        let assign49130_e63029: f64 = (locals.var_xgtscr__blk1352 - locals.var_qiscr0__blk1355);
        let assign49130_e63031: f64 = (assign49130_e63029 - 230.25850929940458);
        let assign49130_e63035: f64 = (locals.var_xgtscr__blk1352 - locals.var_qiscr0__blk1355);
        let assign49130_e63037: f64 = (assign49130_e63035 - 230.25850929940458);
        let assign49130_e63039: f64 = (assign49130_e63037 * 0.3333333333333333);
        let assign49130_e63040: f64 = (1.0 + assign49130_e63039);
        let assign49130_e63041: f64 = (assign49130_e63031 * assign49130_e63040);
        let assign49130_e63042: f64 = (0.5 * assign49130_e63041);
        let assign49130_e63043: f64 = (1.0 + assign49130_e63042);
        let assign49130_e63044: f64 = (assign49130_e63024 * assign49130_e63043);
        let assign49130_e63045: f64 = (1.0 + assign49130_e63044);
        let assign49130_e63046: f64 = (1e100 * assign49130_e63045);
        (assign49130_e63046, (1e100 * (((locals.var_xgtscr__blk1352_dn4 - locals.var_qiscr0__blk1355_dn4) * assign49130_e63043) + (assign49130_e63024 * (0.5 * (((locals.var_xgtscr__blk1352_dn4 - locals.var_qiscr0__blk1355_dn4) * assign49130_e63040) + (assign49130_e63031 * ((locals.var_xgtscr__blk1352_dn4 - locals.var_qiscr0__blk1355_dn4) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1352_dn6 - locals.var_qiscr0__blk1355_dn6) * assign49130_e63043) + (assign49130_e63024 * (0.5 * (((locals.var_xgtscr__blk1352_dn6 - locals.var_qiscr0__blk1355_dn6) * assign49130_e63040) + (assign49130_e63031 * ((locals.var_xgtscr__blk1352_dn6 - locals.var_qiscr0__blk1355_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1352_dn7 - locals.var_qiscr0__blk1355_dn7) * assign49130_e63043) + (assign49130_e63024 * (0.5 * (((locals.var_xgtscr__blk1352_dn7 - locals.var_qiscr0__blk1355_dn7) * assign49130_e63040) + (assign49130_e63031 * ((locals.var_xgtscr__blk1352_dn7 - locals.var_qiscr0__blk1355_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1352_dn8 - locals.var_qiscr0__blk1355_dn8) * assign49130_e63043) + (assign49130_e63024 * (0.5 * (((locals.var_xgtscr__blk1352_dn8 - locals.var_qiscr0__blk1355_dn8) * assign49130_e63040) + (assign49130_e63031 * ((locals.var_xgtscr__blk1352_dn8 - locals.var_qiscr0__blk1355_dn8) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1352_dn9 - locals.var_qiscr0__blk1355_dn9) * assign49130_e63043) + (assign49130_e63024 * (0.5 * (((locals.var_xgtscr__blk1352_dn9 - locals.var_qiscr0__blk1355_dn9) * assign49130_e63040) + (assign49130_e63031 * ((locals.var_xgtscr__blk1352_dn9 - locals.var_qiscr0__blk1355_dn9) * 0.3333333333333333))))))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign49130_e63048;
        locals.var_temp__blk949_dn4 = assign49130_e63048_d_n4;
        locals.var_temp__blk949_dn6 = assign49130_e63048_d_n6;
        locals.var_temp__blk949_dn7 = assign49130_e63048_d_n7;
        locals.var_temp__blk949_dn8 = assign49130_e63048_d_n8;
        locals.var_temp__blk949_dn9 = assign49130_e63048_d_n9;

        let (assign49140_e63058, assign49140_e63058_d_n4, assign49140_e63058_d_n6, assign49140_e63058_d_n7, assign49140_e63058_d_n8, assign49140_e63058_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49140_e63056: f64 = (locals.var_temp__blk949 / locals.var_nscr__blk1350);
        (assign49140_e63056, (((locals.var_temp__blk949_dn4 * locals.var_nscr__blk1350) - (locals.var_temp__blk949 * locals.var_nscr__blk1350_dn4)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), (((locals.var_temp__blk949_dn6 * locals.var_nscr__blk1350) - (locals.var_temp__blk949 * locals.var_nscr__blk1350_dn6)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), (((locals.var_temp__blk949_dn7 * locals.var_nscr__blk1350) - (locals.var_temp__blk949 * locals.var_nscr__blk1350_dn7)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), (((locals.var_temp__blk949_dn8 * locals.var_nscr__blk1350) - (locals.var_temp__blk949 * locals.var_nscr__blk1350_dn8)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), (((locals.var_temp__blk949_dn9 * locals.var_nscr__blk1350) - (locals.var_temp__blk949 * locals.var_nscr__blk1350_dn9)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)),)
    } else {
        (locals.var_dscr0__blk1356, locals.var_dscr0__blk1356_dn4, locals.var_dscr0__blk1356_dn6, locals.var_dscr0__blk1356_dn7, locals.var_dscr0__blk1356_dn8, locals.var_dscr0__blk1356_dn9,)
    }
};
        locals.var_dscr0__blk1356 = assign49140_e63058;
        locals.var_dscr0__blk1356_dn4 = assign49140_e63058_d_n4;
        locals.var_dscr0__blk1356_dn6 = assign49140_e63058_d_n6;
        locals.var_dscr0__blk1356_dn7 = assign49140_e63058_d_n7;
        locals.var_dscr0__blk1356_dn8 = assign49140_e63058_d_n8;
        locals.var_dscr0__blk1356_dn9 = assign49140_e63058_d_n9;

        let (assign49150_e63072, assign49150_e63072_d_n4, assign49150_e63072_d_n6, assign49150_e63072_d_n7, assign49150_e63072_d_n8, assign49150_e63072_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49150_e63067: f64 = (locals.var_qiscr0__blk1355 + 1.0);
        let assign49150_e63068: f64 = (2.0 * assign49150_e63067);
        let assign49150_e63070: f64 = (assign49150_e63068 - locals.var_dscr0__blk1356);
        (assign49150_e63070, ((2.0 * locals.var_qiscr0__blk1355_dn4) - locals.var_dscr0__blk1356_dn4), ((2.0 * locals.var_qiscr0__blk1355_dn6) - locals.var_dscr0__blk1356_dn6), ((2.0 * locals.var_qiscr0__blk1355_dn7) - locals.var_dscr0__blk1356_dn7), ((2.0 * locals.var_qiscr0__blk1355_dn8) - locals.var_dscr0__blk1356_dn8), ((2.0 * locals.var_qiscr0__blk1355_dn9) - locals.var_dscr0__blk1356_dn9),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign49150_e63072;
        locals.var_temp__blk949_dn4 = assign49150_e63072_d_n4;
        locals.var_temp__blk949_dn6 = assign49150_e63072_d_n6;
        locals.var_temp__blk949_dn7 = assign49150_e63072_d_n7;
        locals.var_temp__blk949_dn8 = assign49150_e63072_d_n8;
        locals.var_temp__blk949_dn9 = assign49150_e63072_d_n9;

        let assign49160_e63075: f64 = if locals.var_dscr0__blk1356 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1483 = assign49160_e63075;

        let (assign49170_e63100, assign49170_e63100_d_n4, assign49170_e63100_d_n6, assign49170_e63100_d_n7, assign49170_e63100_d_n8, assign49170_e63100_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) && (locals.var_guard1483 != 0.0)) {
        let assign49170_e63088: f64 = (locals.var_dscr0__blk1356 * locals.var_temp__blk949);
        let assign49170_e63089: f64 = (1.0 + assign49170_e63088);
        let assign49170_e63090: f64 = (assign49170_e63089).sqrt();
        let assign49170_e63092: f64 = (assign49170_e63090 - 1.0);
        let assign49170_e63094: f64 = (assign49170_e63092 / locals.var_dscr0__blk1356);
        let assign49170_e63095: f64 = (locals.var_qiscr0__blk1355 - assign49170_e63094);
        let assign49170_e63097: f64 = (assign49170_e63095 + 1.0);
        let assign49170_e63098: f64 = (locals.var_nscr__blk1350 * assign49170_e63097);
        (assign49170_e63098, ((locals.var_nscr__blk1350_dn4 * assign49170_e63097) + (locals.var_nscr__blk1350 * (locals.var_qiscr0__blk1355_dn4 - ((((((locals.var_dscr0__blk1356_dn4 * locals.var_temp__blk949) + (locals.var_dscr0__blk1356 * locals.var_temp__blk949_dn4)) / (2.0 * assign49170_e63090)) * locals.var_dscr0__blk1356) - (assign49170_e63092 * locals.var_dscr0__blk1356_dn4)) / (locals.var_dscr0__blk1356 * locals.var_dscr0__blk1356))))), ((locals.var_nscr__blk1350_dn6 * assign49170_e63097) + (locals.var_nscr__blk1350 * (locals.var_qiscr0__blk1355_dn6 - ((((((locals.var_dscr0__blk1356_dn6 * locals.var_temp__blk949) + (locals.var_dscr0__blk1356 * locals.var_temp__blk949_dn6)) / (2.0 * assign49170_e63090)) * locals.var_dscr0__blk1356) - (assign49170_e63092 * locals.var_dscr0__blk1356_dn6)) / (locals.var_dscr0__blk1356 * locals.var_dscr0__blk1356))))), ((locals.var_nscr__blk1350_dn7 * assign49170_e63097) + (locals.var_nscr__blk1350 * (locals.var_qiscr0__blk1355_dn7 - ((((((locals.var_dscr0__blk1356_dn7 * locals.var_temp__blk949) + (locals.var_dscr0__blk1356 * locals.var_temp__blk949_dn7)) / (2.0 * assign49170_e63090)) * locals.var_dscr0__blk1356) - (assign49170_e63092 * locals.var_dscr0__blk1356_dn7)) / (locals.var_dscr0__blk1356 * locals.var_dscr0__blk1356))))), ((locals.var_nscr__blk1350_dn8 * assign49170_e63097) + (locals.var_nscr__blk1350 * (locals.var_qiscr0__blk1355_dn8 - ((((((locals.var_dscr0__blk1356_dn8 * locals.var_temp__blk949) + (locals.var_dscr0__blk1356 * locals.var_temp__blk949_dn8)) / (2.0 * assign49170_e63090)) * locals.var_dscr0__blk1356) - (assign49170_e63092 * locals.var_dscr0__blk1356_dn8)) / (locals.var_dscr0__blk1356 * locals.var_dscr0__blk1356))))), ((locals.var_nscr__blk1350_dn9 * assign49170_e63097) + (locals.var_nscr__blk1350 * (locals.var_qiscr0__blk1355_dn9 - ((((((locals.var_dscr0__blk1356_dn9 * locals.var_temp__blk949) + (locals.var_dscr0__blk1356 * locals.var_temp__blk949_dn9)) / (2.0 * assign49170_e63090)) * locals.var_dscr0__blk1356) - (assign49170_e63092 * locals.var_dscr0__blk1356_dn9)) / (locals.var_dscr0__blk1356 * locals.var_dscr0__blk1356))))),)
    } else {
        (locals.var_qiscr__blk1357, locals.var_qiscr__blk1357_dn4, locals.var_qiscr__blk1357_dn6, locals.var_qiscr__blk1357_dn7, locals.var_qiscr__blk1357_dn8, locals.var_qiscr__blk1357_dn9,)
    }
};
        locals.var_qiscr__blk1357 = assign49170_e63100;
        locals.var_qiscr__blk1357_dn4 = assign49170_e63100_d_n4;
        locals.var_qiscr__blk1357_dn6 = assign49170_e63100_d_n6;
        locals.var_qiscr__blk1357_dn7 = assign49170_e63100_d_n7;
        locals.var_qiscr__blk1357_dn8 = assign49170_e63100_d_n8;
        locals.var_qiscr__blk1357_dn9 = assign49170_e63100_d_n9;

        let (assign49180_e63123, assign49180_e63123_d_n4, assign49180_e63123_d_n6, assign49180_e63123_d_n7, assign49180_e63123_d_n8, assign49180_e63123_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) && (locals.var_guard1483 == 0.0)) {
        let assign49180_e63111: f64 = (locals.var_nscr__blk1350 * 0.5);
        let assign49180_e63113: f64 = (assign49180_e63111 * locals.var_dscr0__blk1356);
        let assign49180_e63117: f64 = (0.25 * locals.var_temp__blk949);
        let assign49180_e63119: f64 = (assign49180_e63117 * locals.var_temp__blk949);
        let assign49180_e63120: f64 = (1.0 + assign49180_e63119);
        let assign49180_e63121: f64 = (assign49180_e63113 * assign49180_e63120);
        (assign49180_e63121, (((((locals.var_nscr__blk1350_dn4 * 0.5) * locals.var_dscr0__blk1356) + (assign49180_e63111 * locals.var_dscr0__blk1356_dn4)) * assign49180_e63120) + (assign49180_e63113 * (((0.25 * locals.var_temp__blk949_dn4) * locals.var_temp__blk949) + (assign49180_e63117 * locals.var_temp__blk949_dn4)))), (((((locals.var_nscr__blk1350_dn6 * 0.5) * locals.var_dscr0__blk1356) + (assign49180_e63111 * locals.var_dscr0__blk1356_dn6)) * assign49180_e63120) + (assign49180_e63113 * (((0.25 * locals.var_temp__blk949_dn6) * locals.var_temp__blk949) + (assign49180_e63117 * locals.var_temp__blk949_dn6)))), (((((locals.var_nscr__blk1350_dn7 * 0.5) * locals.var_dscr0__blk1356) + (assign49180_e63111 * locals.var_dscr0__blk1356_dn7)) * assign49180_e63120) + (assign49180_e63113 * (((0.25 * locals.var_temp__blk949_dn7) * locals.var_temp__blk949) + (assign49180_e63117 * locals.var_temp__blk949_dn7)))), (((((locals.var_nscr__blk1350_dn8 * 0.5) * locals.var_dscr0__blk1356) + (assign49180_e63111 * locals.var_dscr0__blk1356_dn8)) * assign49180_e63120) + (assign49180_e63113 * (((0.25 * locals.var_temp__blk949_dn8) * locals.var_temp__blk949) + (assign49180_e63117 * locals.var_temp__blk949_dn8)))), (((((locals.var_nscr__blk1350_dn9 * 0.5) * locals.var_dscr0__blk1356) + (assign49180_e63111 * locals.var_dscr0__blk1356_dn9)) * assign49180_e63120) + (assign49180_e63113 * (((0.25 * locals.var_temp__blk949_dn9) * locals.var_temp__blk949) + (assign49180_e63117 * locals.var_temp__blk949_dn9)))),)
    } else {
        (locals.var_qiscr__blk1357, locals.var_qiscr__blk1357_dn4, locals.var_qiscr__blk1357_dn6, locals.var_qiscr__blk1357_dn7, locals.var_qiscr__blk1357_dn8, locals.var_qiscr__blk1357_dn9,)
    }
};
        locals.var_qiscr__blk1357 = assign49180_e63123;
        locals.var_qiscr__blk1357_dn4 = assign49180_e63123_d_n4;
        locals.var_qiscr__blk1357_dn6 = assign49180_e63123_d_n6;
        locals.var_qiscr__blk1357_dn7 = assign49180_e63123_d_n7;
        locals.var_qiscr__blk1357_dn8 = assign49180_e63123_d_n8;
        locals.var_qiscr__blk1357_dn9 = assign49180_e63123_d_n9;

        let (assign49190_e63152, assign49190_e63152_d_n4, assign49190_e63152_d_n6, assign49190_e63152_d_n7, assign49190_e63152_d_n8, assign49190_e63152_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49190_e63132: f64 = (locals.var_xg__blk1343 - locals.var_qiscr__blk1357);
        let assign49190_e63134: f64 = (assign49190_e63132 + 2.0);
        let assign49190_e63137: f64 = (locals.var_xg__blk1343 - locals.var_qiscr__blk1357);
        let assign49190_e63139: f64 = (assign49190_e63137 - 2.0);
        let assign49190_e63142: f64 = (locals.var_xg__blk1343 - locals.var_qiscr__blk1357);
        let assign49190_e63144: f64 = (assign49190_e63142 - 2.0);
        let assign49190_e63145: f64 = (assign49190_e63139 * assign49190_e63144);
        let assign49190_e63147: f64 = (assign49190_e63145 + 1.0);
        let assign49190_e63148: f64 = (assign49190_e63147).sqrt();
        let assign49190_e63149: f64 = (assign49190_e63134 + assign49190_e63148);
        let assign49190_e63150: f64 = (0.5 * assign49190_e63149);
        (assign49190_e63150, (0.5 * ((locals.var_xg__blk1343_dn4 - locals.var_qiscr__blk1357_dn4) + ((((locals.var_xg__blk1343_dn4 - locals.var_qiscr__blk1357_dn4) * assign49190_e63144) + (assign49190_e63139 * (locals.var_xg__blk1343_dn4 - locals.var_qiscr__blk1357_dn4))) / (2.0 * assign49190_e63148)))), (0.5 * ((locals.var_xg__blk1343_dn6 - locals.var_qiscr__blk1357_dn6) + ((((locals.var_xg__blk1343_dn6 - locals.var_qiscr__blk1357_dn6) * assign49190_e63144) + (assign49190_e63139 * (locals.var_xg__blk1343_dn6 - locals.var_qiscr__blk1357_dn6))) / (2.0 * assign49190_e63148)))), (0.5 * ((locals.var_xg__blk1343_dn7 - locals.var_qiscr__blk1357_dn7) + ((((locals.var_xg__blk1343_dn7 - locals.var_qiscr__blk1357_dn7) * assign49190_e63144) + (assign49190_e63139 * (locals.var_xg__blk1343_dn7 - locals.var_qiscr__blk1357_dn7))) / (2.0 * assign49190_e63148)))), (0.5 * ((locals.var_xg__blk1343_dn8 - locals.var_qiscr__blk1357_dn8) + ((((locals.var_xg__blk1343_dn8 - locals.var_qiscr__blk1357_dn8) * assign49190_e63144) + (assign49190_e63139 * (locals.var_xg__blk1343_dn8 - locals.var_qiscr__blk1357_dn8))) / (2.0 * assign49190_e63148)))), (0.5 * ((locals.var_xg__blk1343_dn9 - locals.var_qiscr__blk1357_dn9) + ((((locals.var_xg__blk1343_dn9 - locals.var_qiscr__blk1357_dn9) * assign49190_e63144) + (assign49190_e63139 * (locals.var_xg__blk1343_dn9 - locals.var_qiscr__blk1357_dn9))) / (2.0 * assign49190_e63148)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign49190_e63152;
        locals.var_temp__blk949_dn4 = assign49190_e63152_d_n4;
        locals.var_temp__blk949_dn6 = assign49190_e63152_d_n6;
        locals.var_temp__blk949_dn7 = assign49190_e63152_d_n7;
        locals.var_temp__blk949_dn8 = assign49190_e63152_d_n8;
        locals.var_temp__blk949_dn9 = assign49190_e63152_d_n9;

        let (assign49200_e63173, assign49200_e63173_d_n4, assign49200_e63173_d_n6, assign49200_e63173_d_n7, assign49200_e63173_d_n8, assign49200_e63173_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49200_e63160: f64 = (0.5 * locals.var_gf2__blk1325);
        let assign49200_e63164: f64 = (4.0 / locals.var_gf2__blk1325);
        let assign49200_e63166: f64 = (assign49200_e63164 * locals.var_temp__blk949);
        let assign49200_e63167: f64 = (1.0 + assign49200_e63166);
        let assign49200_e63168: f64 = (assign49200_e63167).sqrt();
        let assign49200_e63170: f64 = (assign49200_e63168 - 1.0);
        let assign49200_e63171: f64 = (assign49200_e63160 * assign49200_e63170);
        (assign49200_e63171, (((0.5 * locals.var_gf2__blk1325_dn4) * assign49200_e63170) + (assign49200_e63160 * ((((-((4.0 * locals.var_gf2__blk1325_dn4) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) * locals.var_temp__blk949) + (assign49200_e63164 * locals.var_temp__blk949_dn4)) / (2.0 * assign49200_e63168)))), (((0.5 * locals.var_gf2__blk1325_dn6) * assign49200_e63170) + (assign49200_e63160 * ((((-((4.0 * locals.var_gf2__blk1325_dn6) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) * locals.var_temp__blk949) + (assign49200_e63164 * locals.var_temp__blk949_dn6)) / (2.0 * assign49200_e63168)))), (((0.5 * locals.var_gf2__blk1325_dn7) * assign49200_e63170) + (assign49200_e63160 * ((((-((4.0 * locals.var_gf2__blk1325_dn7) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) * locals.var_temp__blk949) + (assign49200_e63164 * locals.var_temp__blk949_dn7)) / (2.0 * assign49200_e63168)))), (((0.5 * locals.var_gf2__blk1325_dn8) * assign49200_e63170) + (assign49200_e63160 * ((((-((4.0 * locals.var_gf2__blk1325_dn8) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) * locals.var_temp__blk949) + (assign49200_e63164 * locals.var_temp__blk949_dn8)) / (2.0 * assign49200_e63168)))), (((0.5 * locals.var_gf2__blk1325_dn9) * assign49200_e63170) + (assign49200_e63160 * ((((-((4.0 * locals.var_gf2__blk1325_dn9) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) * locals.var_temp__blk949) + (assign49200_e63164 * locals.var_temp__blk949_dn9)) / (2.0 * assign49200_e63168)))),)
    } else {
        (locals.var_qbscr__blk1358, locals.var_qbscr__blk1358_dn4, locals.var_qbscr__blk1358_dn6, locals.var_qbscr__blk1358_dn7, locals.var_qbscr__blk1358_dn8, locals.var_qbscr__blk1358_dn9,)
    }
};
        locals.var_qbscr__blk1358 = assign49200_e63173;
        locals.var_qbscr__blk1358_dn4 = assign49200_e63173_d_n4;
        locals.var_qbscr__blk1358_dn6 = assign49200_e63173_d_n6;
        locals.var_qbscr__blk1358_dn7 = assign49200_e63173_d_n7;
        locals.var_qbscr__blk1358_dn8 = assign49200_e63173_d_n8;
        locals.var_qbscr__blk1358_dn9 = assign49200_e63173_d_n9;

        let (assign49210_e63185, assign49210_e63185_d_n4, assign49210_e63185_d_n6, assign49210_e63185_d_n7, assign49210_e63185_d_n8, assign49210_e63185_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49210_e63182: f64 = (locals.var_qbscr__blk1358 + locals.var_qiscr__blk1357);
        let assign49210_e63183: f64 = (locals.var_qbscr__blk1358 / assign49210_e63182);
        (assign49210_e63183, (((locals.var_qbscr__blk1358_dn4 * assign49210_e63182) - (locals.var_qbscr__blk1358 * (locals.var_qbscr__blk1358_dn4 + locals.var_qiscr__blk1357_dn4))) / (assign49210_e63182 * assign49210_e63182)), (((locals.var_qbscr__blk1358_dn6 * assign49210_e63182) - (locals.var_qbscr__blk1358 * (locals.var_qbscr__blk1358_dn6 + locals.var_qiscr__blk1357_dn6))) / (assign49210_e63182 * assign49210_e63182)), (((locals.var_qbscr__blk1358_dn7 * assign49210_e63182) - (locals.var_qbscr__blk1358 * (locals.var_qbscr__blk1358_dn7 + locals.var_qiscr__blk1357_dn7))) / (assign49210_e63182 * assign49210_e63182)), (((locals.var_qbscr__blk1358_dn8 * assign49210_e63182) - (locals.var_qbscr__blk1358 * (locals.var_qbscr__blk1358_dn8 + locals.var_qiscr__blk1357_dn8))) / (assign49210_e63182 * assign49210_e63182)), (((locals.var_qbscr__blk1358_dn9 * assign49210_e63182) - (locals.var_qbscr__blk1358 * (locals.var_qbscr__blk1358_dn9 + locals.var_qiscr__blk1357_dn9))) / (assign49210_e63182 * assign49210_e63182)),)
    } else {
        (locals.var_fscr__blk1359, locals.var_fscr__blk1359_dn4, locals.var_fscr__blk1359_dn6, locals.var_fscr__blk1359_dn7, locals.var_fscr__blk1359_dn8, locals.var_fscr__blk1359_dn9,)
    }
};
        locals.var_fscr__blk1359 = assign49210_e63185;
        locals.var_fscr__blk1359_dn4 = assign49210_e63185_d_n4;
        locals.var_fscr__blk1359_dn6 = assign49210_e63185_d_n6;
        locals.var_fscr__blk1359_dn7 = assign49210_e63185_d_n7;
        locals.var_fscr__blk1359_dn8 = assign49210_e63185_d_n8;
        locals.var_fscr__blk1359_dn9 = assign49210_e63185_d_n9;

        let (assign49220_e63197, assign49220_e63197_d_n4, assign49220_e63197_d_n6, assign49220_e63197_d_n7, assign49220_e63197_d_n8, assign49220_e63197_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
        let assign49220_e63194: f64 = (locals.var_fscr__blk1359 * locals.var_delxb__blk1347);
        let assign49220_e63195: f64 = (locals.var_xno_s__blk1348 - assign49220_e63194);
        (assign49220_e63195, (locals.var_xno_s__blk1348_dn4 - ((locals.var_fscr__blk1359_dn4 * locals.var_delxb__blk1347) + (locals.var_fscr__blk1359 * locals.var_delxb__blk1347_dn4))), (locals.var_xno_s__blk1348_dn6 - ((locals.var_fscr__blk1359_dn6 * locals.var_delxb__blk1347) + (locals.var_fscr__blk1359 * locals.var_delxb__blk1347_dn6))), (locals.var_xno_s__blk1348_dn7 - ((locals.var_fscr__blk1359_dn7 * locals.var_delxb__blk1347) + (locals.var_fscr__blk1359 * locals.var_delxb__blk1347_dn7))), (locals.var_xno_s__blk1348_dn8 - ((locals.var_fscr__blk1359_dn8 * locals.var_delxb__blk1347) + (locals.var_fscr__blk1359 * locals.var_delxb__blk1347_dn8))), (locals.var_xno_s__blk1348_dn9 - ((locals.var_fscr__blk1359_dn9 * locals.var_delxb__blk1347) + (locals.var_fscr__blk1359 * locals.var_delxb__blk1347_dn9))),)
    } else {
        (locals.var_xn_s__blk1349, locals.var_xn_s__blk1349_dn4, locals.var_xn_s__blk1349_dn6, locals.var_xn_s__blk1349_dn7, locals.var_xn_s__blk1349_dn8, locals.var_xn_s__blk1349_dn9,)
    }
};
        locals.var_xn_s__blk1349 = assign49220_e63197;
        locals.var_xn_s__blk1349_dn4 = assign49220_e63197_d_n4;
        locals.var_xn_s__blk1349_dn6 = assign49220_e63197_d_n6;
        locals.var_xn_s__blk1349_dn7 = assign49220_e63197_d_n7;
        locals.var_xn_s__blk1349_dn8 = assign49220_e63197_d_n8;
        locals.var_xn_s__blk1349_dn9 = assign49220_e63197_d_n9;

        let (assign49230_e63207, assign49230_e63207_d_n4, assign49230_e63207_d_n6, assign49230_e63207_d_n7, assign49230_e63207_d_n8, assign49230_e63207_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign49230_e63204: f64 = (locals.var_gf__blk1324 * 0.7071067811865475);
        let assign49230_e63205: f64 = (1.0 + assign49230_e63204);
        (assign49230_e63205, (locals.var_gf__blk1324_dn4 * 0.7071067811865475), (locals.var_gf__blk1324_dn6 * 0.7071067811865475), (locals.var_gf__blk1324_dn7 * 0.7071067811865475), (locals.var_gf__blk1324_dn8 * 0.7071067811865475), (locals.var_gf__blk1324_dn9 * 0.7071067811865475),)
    } else {
        (locals.var_xi__blk1360, locals.var_xi__blk1360_dn4, locals.var_xi__blk1360_dn6, locals.var_xi__blk1360_dn7, locals.var_xi__blk1360_dn8, locals.var_xi__blk1360_dn9,)
    }
};
        locals.var_xi__blk1360 = assign49230_e63207;
        locals.var_xi__blk1360_dn4 = assign49230_e63207_d_n4;
        locals.var_xi__blk1360_dn6 = assign49230_e63207_d_n6;
        locals.var_xi__blk1360_dn7 = assign49230_e63207_d_n7;
        locals.var_xi__blk1360_dn8 = assign49230_e63207_d_n8;
        locals.var_xi__blk1360_dn9 = assign49230_e63207_d_n9;

        let (assign49240_e63215,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign49240_e63213: f64 = (1e-5 * locals.var_xi__blk1360);
        (assign49240_e63213,)
    } else {
        (locals.var_margin__blk1361,)
    }
};
        locals.var_margin__blk1361 = assign49240_e63215;

        let (assign49250_e63223, assign49250_e63223_d_n4, assign49250_e63223_d_n6, assign49250_e63223_d_n7, assign49250_e63223_d_n8, assign49250_e63223_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign49250_e63221: f64 = (1.0 / locals.var_xi__blk1360);
        (assign49250_e63221, (-(locals.var_xi__blk1360_dn4 / (locals.var_xi__blk1360 * locals.var_xi__blk1360))), (-(locals.var_xi__blk1360_dn6 / (locals.var_xi__blk1360 * locals.var_xi__blk1360))), (-(locals.var_xi__blk1360_dn7 / (locals.var_xi__blk1360 * locals.var_xi__blk1360))), (-(locals.var_xi__blk1360_dn8 / (locals.var_xi__blk1360 * locals.var_xi__blk1360))), (-(locals.var_xi__blk1360_dn9 / (locals.var_xi__blk1360 * locals.var_xi__blk1360))),)
    } else {
        (locals.var_inv_xi__blk1362, locals.var_inv_xi__blk1362_dn4, locals.var_inv_xi__blk1362_dn6, locals.var_inv_xi__blk1362_dn7, locals.var_inv_xi__blk1362_dn8, locals.var_inv_xi__blk1362_dn9,)
    }
};
        locals.var_inv_xi__blk1362 = assign49250_e63223;
        locals.var_inv_xi__blk1362_dn4 = assign49250_e63223_d_n4;
        locals.var_inv_xi__blk1362_dn6 = assign49250_e63223_d_n6;
        locals.var_inv_xi__blk1362_dn7 = assign49250_e63223_d_n7;
        locals.var_inv_xi__blk1362_dn8 = assign49250_e63223_d_n8;
        locals.var_inv_xi__blk1362_dn9 = assign49250_e63223_d_n9;

        let (assign49260_e63229, assign49260_e63229_d_n4, assign49260_e63229_d_n6, assign49260_e63229_d_n7, assign49260_e63229_d_n8, assign49260_e63229_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sp_s_x1__blk1469, locals.var_sp_s_x1__blk1469_dn4, locals.var_sp_s_x1__blk1469_dn6, locals.var_sp_s_x1__blk1469_dn7, locals.var_sp_s_x1__blk1469_dn8, locals.var_sp_s_x1__blk1469_dn9,)
    }
};
        locals.var_sp_s_x1__blk1469 = assign49260_e63229;
        locals.var_sp_s_x1__blk1469_dn4 = assign49260_e63229_d_n4;
        locals.var_sp_s_x1__blk1469_dn6 = assign49260_e63229_d_n6;
        locals.var_sp_s_x1__blk1469_dn7 = assign49260_e63229_d_n7;
        locals.var_sp_s_x1__blk1469_dn8 = assign49260_e63229_d_n8;
        locals.var_sp_s_x1__blk1469_dn9 = assign49260_e63229_d_n9;

    }

    pub(super) fn stamp_transient_block_38(
        locals: &mut StampLocals,
    ) {
        let (assign49270_e63235, assign49270_e63235_d_n4, assign49270_e63235_d_n6, assign49270_e63235_d_n7, assign49270_e63235_d_n8, assign49270_e63235_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9,)
    }
};
        locals.var_x_s__blk1363 = assign49270_e63235;
        locals.var_x_s__blk1363_dn4 = assign49270_e63235_d_n4;
        locals.var_x_s__blk1363_dn6 = assign49270_e63235_d_n6;
        locals.var_x_s__blk1363_dn7 = assign49270_e63235_d_n7;
        locals.var_x_s__blk1363_dn8 = assign49270_e63235_d_n8;
        locals.var_x_s__blk1363_dn9 = assign49270_e63235_d_n9;

        let assign49280_e63238: f64 = if locals.var_xn_s__blk1349 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1484 = assign49280_e63238;

        let (assign49290_e63248, assign49290_e63248_d_n4, assign49290_e63248_d_n6, assign49290_e63248_d_n7, assign49290_e63248_d_n8, assign49290_e63248_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        let assign49290_e63245: f64 = (-locals.var_xn_s__blk1349);
        let assign49290_e63246: f64 = (assign49290_e63245).exp();
        (assign49290_e63246, (assign49290_e63246 * (-locals.var_xn_s__blk1349_dn4)), (assign49290_e63246 * (-locals.var_xn_s__blk1349_dn6)), (assign49290_e63246 * (-locals.var_xn_s__blk1349_dn7)), (assign49290_e63246 * (-locals.var_xn_s__blk1349_dn8)), (assign49290_e63246 * (-locals.var_xn_s__blk1349_dn9)),)
    } else {
        (locals.var_delta_ns__blk1364, locals.var_delta_ns__blk1364_dn4, locals.var_delta_ns__blk1364_dn6, locals.var_delta_ns__blk1364_dn7, locals.var_delta_ns__blk1364_dn8, locals.var_delta_ns__blk1364_dn9,)
    }
};
        locals.var_delta_ns__blk1364 = assign49290_e63248;
        locals.var_delta_ns__blk1364_dn4 = assign49290_e63248_d_n4;
        locals.var_delta_ns__blk1364_dn6 = assign49290_e63248_d_n6;
        locals.var_delta_ns__blk1364_dn7 = assign49290_e63248_d_n7;
        locals.var_delta_ns__blk1364_dn8 = assign49290_e63248_d_n8;
        locals.var_delta_ns__blk1364_dn9 = assign49290_e63248_d_n9;

        let (assign49300_e63279, assign49300_e63279_d_n4, assign49300_e63279_d_n6, assign49300_e63279_d_n7, assign49300_e63279_d_n8, assign49300_e63279_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1484 == 0.0)) {
        let assign49300_e63259: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
        let assign49300_e63264: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
        let assign49300_e63268: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
        let assign49300_e63270: f64 = (assign49300_e63268 * 0.3333333333333333);
        let assign49300_e63271: f64 = (1.0 + assign49300_e63270);
        let assign49300_e63272: f64 = (assign49300_e63264 * assign49300_e63271);
        let assign49300_e63273: f64 = (0.5 * assign49300_e63272);
        let assign49300_e63274: f64 = (1.0 + assign49300_e63273);
        let assign49300_e63275: f64 = (assign49300_e63259 * assign49300_e63274);
        let assign49300_e63276: f64 = (1.0 + assign49300_e63275);
        let assign49300_e63277: f64 = (1e-200 / assign49300_e63276);
        (assign49300_e63277, (-((1e-200 * ((locals.var_xn_s__blk1349_dn4 * assign49300_e63274) + (assign49300_e63259 * (0.5 * ((locals.var_xn_s__blk1349_dn4 * assign49300_e63271) + (assign49300_e63264 * (locals.var_xn_s__blk1349_dn4 * 0.3333333333333333))))))) / (assign49300_e63276 * assign49300_e63276))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn6 * assign49300_e63274) + (assign49300_e63259 * (0.5 * ((locals.var_xn_s__blk1349_dn6 * assign49300_e63271) + (assign49300_e63264 * (locals.var_xn_s__blk1349_dn6 * 0.3333333333333333))))))) / (assign49300_e63276 * assign49300_e63276))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn7 * assign49300_e63274) + (assign49300_e63259 * (0.5 * ((locals.var_xn_s__blk1349_dn7 * assign49300_e63271) + (assign49300_e63264 * (locals.var_xn_s__blk1349_dn7 * 0.3333333333333333))))))) / (assign49300_e63276 * assign49300_e63276))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn8 * assign49300_e63274) + (assign49300_e63259 * (0.5 * ((locals.var_xn_s__blk1349_dn8 * assign49300_e63271) + (assign49300_e63264 * (locals.var_xn_s__blk1349_dn8 * 0.3333333333333333))))))) / (assign49300_e63276 * assign49300_e63276))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn9 * assign49300_e63274) + (assign49300_e63259 * (0.5 * ((locals.var_xn_s__blk1349_dn9 * assign49300_e63271) + (assign49300_e63264 * (locals.var_xn_s__blk1349_dn9 * 0.3333333333333333))))))) / (assign49300_e63276 * assign49300_e63276))),)
    } else {
        (locals.var_delta_ns__blk1364, locals.var_delta_ns__blk1364_dn4, locals.var_delta_ns__blk1364_dn6, locals.var_delta_ns__blk1364_dn7, locals.var_delta_ns__blk1364_dn8, locals.var_delta_ns__blk1364_dn9,)
    }
};
        locals.var_delta_ns__blk1364 = assign49300_e63279;
        locals.var_delta_ns__blk1364_dn4 = assign49300_e63279_d_n4;
        locals.var_delta_ns__blk1364_dn6 = assign49300_e63279_d_n6;
        locals.var_delta_ns__blk1364_dn7 = assign49300_e63279_d_n7;
        locals.var_delta_ns__blk1364_dn8 = assign49300_e63279_d_n8;
        locals.var_delta_ns__blk1364_dn9 = assign49300_e63279_d_n9;

        let assign49310_e63281: f64 = (locals.var_xg__blk1343).abs();
        let assign49310_e63283: f64 = if assign49310_e63281 <= locals.var_margin__blk1361 { 1.0 } else { 0.0 };
        locals.var_guard1485 = assign49310_e63283;

        let (assign49320_e63297, assign49320_e63297_d_n4, assign49320_e63297_d_n6, assign49320_e63297_d_n7, assign49320_e63297_d_n8, assign49320_e63297_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign49320_e63291: f64 = (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362);
        let assign49320_e63293: f64 = (assign49320_e63291 * 0.16666666666666666);
        let assign49320_e63295: f64 = (assign49320_e63293 * 0.7071067811865475);
        (assign49320_e63295, ((((locals.var_inv_xi__blk1362_dn4 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn4)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn6 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn7 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn8 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn8)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn9 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn9)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_sp_s_temp1__blk1449, locals.var_sp_s_temp1__blk1449_dn4, locals.var_sp_s_temp1__blk1449_dn6, locals.var_sp_s_temp1__blk1449_dn7, locals.var_sp_s_temp1__blk1449_dn8, locals.var_sp_s_temp1__blk1449_dn9,)
    }
};
        locals.var_sp_s_temp1__blk1449 = assign49320_e63297;
        locals.var_sp_s_temp1__blk1449_dn4 = assign49320_e63297_d_n4;
        locals.var_sp_s_temp1__blk1449_dn6 = assign49320_e63297_d_n6;
        locals.var_sp_s_temp1__blk1449_dn7 = assign49320_e63297_d_n7;
        locals.var_sp_s_temp1__blk1449_dn8 = assign49320_e63297_d_n8;
        locals.var_sp_s_temp1__blk1449_dn9 = assign49320_e63297_d_n9;

        let (assign49330_e63319, assign49330_e63319_d_n4, assign49330_e63319_d_n6, assign49330_e63319_d_n7, assign49330_e63319_d_n8, assign49330_e63319_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign49330_e63305: f64 = (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362);
        let assign49330_e63310: f64 = (1.0 - locals.var_delta_ns__blk1364);
        let assign49330_e63311: f64 = (locals.var_xg__blk1343 * assign49330_e63310);
        let assign49330_e63313: f64 = (assign49330_e63311 * locals.var_gf__blk1324);
        let assign49330_e63315: f64 = (assign49330_e63313 * locals.var_sp_s_temp1__blk1449);
        let assign49330_e63316: f64 = (1.0 + assign49330_e63315);
        let assign49330_e63317: f64 = (assign49330_e63305 * assign49330_e63316);
        (assign49330_e63317, ((((locals.var_xg__blk1343_dn4 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn4)) * assign49330_e63316) + (assign49330_e63305 * ((((((locals.var_xg__blk1343_dn4 * assign49330_e63310) + (locals.var_xg__blk1343 * (-locals.var_delta_ns__blk1364_dn4))) * locals.var_gf__blk1324) + (assign49330_e63311 * locals.var_gf__blk1324_dn4)) * locals.var_sp_s_temp1__blk1449) + (assign49330_e63313 * locals.var_sp_s_temp1__blk1449_dn4)))), ((((locals.var_xg__blk1343_dn6 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn6)) * assign49330_e63316) + (assign49330_e63305 * ((((((locals.var_xg__blk1343_dn6 * assign49330_e63310) + (locals.var_xg__blk1343 * (-locals.var_delta_ns__blk1364_dn6))) * locals.var_gf__blk1324) + (assign49330_e63311 * locals.var_gf__blk1324_dn6)) * locals.var_sp_s_temp1__blk1449) + (assign49330_e63313 * locals.var_sp_s_temp1__blk1449_dn6)))), ((((locals.var_xg__blk1343_dn7 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn7)) * assign49330_e63316) + (assign49330_e63305 * ((((((locals.var_xg__blk1343_dn7 * assign49330_e63310) + (locals.var_xg__blk1343 * (-locals.var_delta_ns__blk1364_dn7))) * locals.var_gf__blk1324) + (assign49330_e63311 * locals.var_gf__blk1324_dn7)) * locals.var_sp_s_temp1__blk1449) + (assign49330_e63313 * locals.var_sp_s_temp1__blk1449_dn7)))), ((((locals.var_xg__blk1343_dn8 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn8)) * assign49330_e63316) + (assign49330_e63305 * ((((((locals.var_xg__blk1343_dn8 * assign49330_e63310) + (locals.var_xg__blk1343 * (-locals.var_delta_ns__blk1364_dn8))) * locals.var_gf__blk1324) + (assign49330_e63311 * locals.var_gf__blk1324_dn8)) * locals.var_sp_s_temp1__blk1449) + (assign49330_e63313 * locals.var_sp_s_temp1__blk1449_dn8)))), ((((locals.var_xg__blk1343_dn9 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn9)) * assign49330_e63316) + (assign49330_e63305 * ((((((locals.var_xg__blk1343_dn9 * assign49330_e63310) + (locals.var_xg__blk1343 * (-locals.var_delta_ns__blk1364_dn9))) * locals.var_gf__blk1324) + (assign49330_e63311 * locals.var_gf__blk1324_dn9)) * locals.var_sp_s_temp1__blk1449) + (assign49330_e63313 * locals.var_sp_s_temp1__blk1449_dn9)))),)
    } else {
        (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9,)
    }
};
        locals.var_x_s__blk1363 = assign49330_e63319;
        locals.var_x_s__blk1363_dn4 = assign49330_e63319_d_n4;
        locals.var_x_s__blk1363_dn6 = assign49330_e63319_d_n6;
        locals.var_x_s__blk1363_dn7 = assign49330_e63319_d_n7;
        locals.var_x_s__blk1363_dn8 = assign49330_e63319_d_n8;
        locals.var_x_s__blk1363_dn9 = assign49330_e63319_d_n9;

        let assign49340_e63322: f64 = (-locals.var_margin__blk1361);
        let assign49340_e63323: f64 = if locals.var_xg__blk1343 < assign49340_e63322 { 1.0 } else { 0.0 };
        locals.var_guard1486 = assign49340_e63323;

        let (assign49350_e63335, assign49350_e63335_d_n4, assign49350_e63335_d_n6, assign49350_e63335_d_n7, assign49350_e63335_d_n8, assign49350_e63335_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49350_e63333: f64 = (-locals.var_xg__blk1343);
        (assign49350_e63333, (-locals.var_xg__blk1343_dn4), (-locals.var_xg__blk1343_dn6), (-locals.var_xg__blk1343_dn7), (-locals.var_xg__blk1343_dn8), (-locals.var_xg__blk1343_dn9),)
    } else {
        (locals.var_sp_s_yg__blk1451, locals.var_sp_s_yg__blk1451_dn4, locals.var_sp_s_yg__blk1451_dn6, locals.var_sp_s_yg__blk1451_dn7, locals.var_sp_s_yg__blk1451_dn8, locals.var_sp_s_yg__blk1451_dn9,)
    }
};
        locals.var_sp_s_yg__blk1451 = assign49350_e63335;
        locals.var_sp_s_yg__blk1451_dn4 = assign49350_e63335_d_n4;
        locals.var_sp_s_yg__blk1451_dn6 = assign49350_e63335_d_n6;
        locals.var_sp_s_yg__blk1451_dn7 = assign49350_e63335_d_n7;
        locals.var_sp_s_yg__blk1451_dn8 = assign49350_e63335_d_n8;
        locals.var_sp_s_yg__blk1451_dn9 = assign49350_e63335_d_n9;

        let (assign49360_e63350, assign49360_e63350_d_n4, assign49360_e63350_d_n6, assign49360_e63350_d_n7, assign49360_e63350_d_n8, assign49360_e63350_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49360_e63347: f64 = (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362);
        let assign49360_e63348: f64 = (1.25 * assign49360_e63347);
        (assign49360_e63348, (1.25 * ((locals.var_sp_s_yg__blk1451_dn4 * locals.var_inv_xi__blk1362) + (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362_dn4))), (1.25 * ((locals.var_sp_s_yg__blk1451_dn6 * locals.var_inv_xi__blk1362) + (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362_dn6))), (1.25 * ((locals.var_sp_s_yg__blk1451_dn7 * locals.var_inv_xi__blk1362) + (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362_dn7))), (1.25 * ((locals.var_sp_s_yg__blk1451_dn8 * locals.var_inv_xi__blk1362) + (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362_dn8))), (1.25 * ((locals.var_sp_s_yg__blk1451_dn9 * locals.var_inv_xi__blk1362) + (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362_dn9))),)
    } else {
        (locals.var_sp_s_ysub__blk1452, locals.var_sp_s_ysub__blk1452_dn4, locals.var_sp_s_ysub__blk1452_dn6, locals.var_sp_s_ysub__blk1452_dn7, locals.var_sp_s_ysub__blk1452_dn8, locals.var_sp_s_ysub__blk1452_dn9,)
    }
};
        locals.var_sp_s_ysub__blk1452 = assign49360_e63350;
        locals.var_sp_s_ysub__blk1452_dn4 = assign49360_e63350_d_n4;
        locals.var_sp_s_ysub__blk1452_dn6 = assign49360_e63350_d_n6;
        locals.var_sp_s_ysub__blk1452_dn7 = assign49360_e63350_d_n7;
        locals.var_sp_s_ysub__blk1452_dn8 = assign49360_e63350_d_n8;
        locals.var_sp_s_ysub__blk1452_dn9 = assign49360_e63350_d_n9;

        let (assign49370_e63376, assign49370_e63376_d_n4, assign49370_e63376_d_n6, assign49370_e63376_d_n7, assign49370_e63376_d_n8, assign49370_e63376_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49370_e63362: f64 = (locals.var_sp_s_ysub__blk1452 + 10.0);
        let assign49370_e63365: f64 = (locals.var_sp_s_ysub__blk1452 - 6.0);
        let assign49370_e63368: f64 = (locals.var_sp_s_ysub__blk1452 - 6.0);
        let assign49370_e63369: f64 = (assign49370_e63365 * assign49370_e63368);
        let assign49370_e63371: f64 = (assign49370_e63369 + 64.0);
        let assign49370_e63372: f64 = (assign49370_e63371).sqrt();
        let assign49370_e63373: f64 = (assign49370_e63362 - assign49370_e63372);
        let assign49370_e63374: f64 = (0.5 * assign49370_e63373);
        (assign49370_e63374, (0.5 * (locals.var_sp_s_ysub__blk1452_dn4 - (((locals.var_sp_s_ysub__blk1452_dn4 * assign49370_e63368) + (assign49370_e63365 * locals.var_sp_s_ysub__blk1452_dn4)) / (2.0 * assign49370_e63372)))), (0.5 * (locals.var_sp_s_ysub__blk1452_dn6 - (((locals.var_sp_s_ysub__blk1452_dn6 * assign49370_e63368) + (assign49370_e63365 * locals.var_sp_s_ysub__blk1452_dn6)) / (2.0 * assign49370_e63372)))), (0.5 * (locals.var_sp_s_ysub__blk1452_dn7 - (((locals.var_sp_s_ysub__blk1452_dn7 * assign49370_e63368) + (assign49370_e63365 * locals.var_sp_s_ysub__blk1452_dn7)) / (2.0 * assign49370_e63372)))), (0.5 * (locals.var_sp_s_ysub__blk1452_dn8 - (((locals.var_sp_s_ysub__blk1452_dn8 * assign49370_e63368) + (assign49370_e63365 * locals.var_sp_s_ysub__blk1452_dn8)) / (2.0 * assign49370_e63372)))), (0.5 * (locals.var_sp_s_ysub__blk1452_dn9 - (((locals.var_sp_s_ysub__blk1452_dn9 * assign49370_e63368) + (assign49370_e63365 * locals.var_sp_s_ysub__blk1452_dn9)) / (2.0 * assign49370_e63372)))),)
    } else {
        (locals.var_sp_s_eta__blk1453, locals.var_sp_s_eta__blk1453_dn4, locals.var_sp_s_eta__blk1453_dn6, locals.var_sp_s_eta__blk1453_dn7, locals.var_sp_s_eta__blk1453_dn8, locals.var_sp_s_eta__blk1453_dn9,)
    }
};
        locals.var_sp_s_eta__blk1453 = assign49370_e63376;
        locals.var_sp_s_eta__blk1453_dn4 = assign49370_e63376_d_n4;
        locals.var_sp_s_eta__blk1453_dn6 = assign49370_e63376_d_n6;
        locals.var_sp_s_eta__blk1453_dn7 = assign49370_e63376_d_n7;
        locals.var_sp_s_eta__blk1453_dn8 = assign49370_e63376_d_n8;
        locals.var_sp_s_eta__blk1453_dn9 = assign49370_e63376_d_n9;

        let (assign49380_e63389, assign49380_e63389_d_n4, assign49380_e63389_d_n6, assign49380_e63389_d_n7, assign49380_e63389_d_n8, assign49380_e63389_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49380_e63387: f64 = (locals.var_sp_s_yg__blk1451 - locals.var_sp_s_eta__blk1453);
        (assign49380_e63387, (locals.var_sp_s_yg__blk1451_dn4 - locals.var_sp_s_eta__blk1453_dn4), (locals.var_sp_s_yg__blk1451_dn6 - locals.var_sp_s_eta__blk1453_dn6), (locals.var_sp_s_yg__blk1451_dn7 - locals.var_sp_s_eta__blk1453_dn7), (locals.var_sp_s_yg__blk1451_dn8 - locals.var_sp_s_eta__blk1453_dn8), (locals.var_sp_s_yg__blk1451_dn9 - locals.var_sp_s_eta__blk1453_dn9),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49380_e63389;
        locals.var_sp_s_temp__blk1448_dn4 = assign49380_e63389_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49380_e63389_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49380_e63389_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49380_e63389_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49380_e63389_d_n9;

        let (assign49390_e63408, assign49390_e63408_d_n4, assign49390_e63408_d_n6, assign49390_e63408_d_n7, assign49390_e63408_d_n8, assign49390_e63408_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49390_e63400: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
        let assign49390_e63404: f64 = (locals.var_sp_s_eta__blk1453 + 1.0);
        let assign49390_e63405: f64 = (locals.var_gf2__blk1325 * assign49390_e63404);
        let assign49390_e63406: f64 = (assign49390_e63400 + assign49390_e63405);
        (assign49390_e63406, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) + ((locals.var_gf2__blk1325_dn4 * assign49390_e63404) + (locals.var_gf2__blk1325 * locals.var_sp_s_eta__blk1453_dn4))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) + ((locals.var_gf2__blk1325_dn6 * assign49390_e63404) + (locals.var_gf2__blk1325 * locals.var_sp_s_eta__blk1453_dn6))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) + ((locals.var_gf2__blk1325_dn7 * assign49390_e63404) + (locals.var_gf2__blk1325 * locals.var_sp_s_eta__blk1453_dn7))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) + ((locals.var_gf2__blk1325_dn8 * assign49390_e63404) + (locals.var_gf2__blk1325 * locals.var_sp_s_eta__blk1453_dn8))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) + ((locals.var_gf2__blk1325_dn9 * assign49390_e63404) + (locals.var_gf2__blk1325 * locals.var_sp_s_eta__blk1453_dn9))),)
    } else {
        (locals.var_sp_s_a__blk1454, locals.var_sp_s_a__blk1454_dn4, locals.var_sp_s_a__blk1454_dn6, locals.var_sp_s_a__blk1454_dn7, locals.var_sp_s_a__blk1454_dn8, locals.var_sp_s_a__blk1454_dn9,)
    }
};
        locals.var_sp_s_a__blk1454 = assign49390_e63408;
        locals.var_sp_s_a__blk1454_dn4 = assign49390_e63408_d_n4;
        locals.var_sp_s_a__blk1454_dn6 = assign49390_e63408_d_n6;
        locals.var_sp_s_a__blk1454_dn7 = assign49390_e63408_d_n7;
        locals.var_sp_s_a__blk1454_dn8 = assign49390_e63408_d_n8;
        locals.var_sp_s_a__blk1454_dn9 = assign49390_e63408_d_n9;

        let (assign49400_e63423, assign49400_e63423_d_n4, assign49400_e63423_d_n6, assign49400_e63423_d_n7, assign49400_e63423_d_n8, assign49400_e63423_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49400_e63419: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
        let assign49400_e63421: f64 = (assign49400_e63419 - locals.var_gf2__blk1325);
        (assign49400_e63421, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) - locals.var_gf2__blk1325_dn4), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) - locals.var_gf2__blk1325_dn6), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) - locals.var_gf2__blk1325_dn7), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) - locals.var_gf2__blk1325_dn8), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) - locals.var_gf2__blk1325_dn9),)
    } else {
        (locals.var_sp_s_c__blk1455, locals.var_sp_s_c__blk1455_dn4, locals.var_sp_s_c__blk1455_dn6, locals.var_sp_s_c__blk1455_dn7, locals.var_sp_s_c__blk1455_dn8, locals.var_sp_s_c__blk1455_dn9,)
    }
};
        locals.var_sp_s_c__blk1455 = assign49400_e63423;
        locals.var_sp_s_c__blk1455_dn4 = assign49400_e63423_d_n4;
        locals.var_sp_s_c__blk1455_dn6 = assign49400_e63423_d_n6;
        locals.var_sp_s_c__blk1455_dn7 = assign49400_e63423_d_n7;
        locals.var_sp_s_c__blk1455_dn8 = assign49400_e63423_d_n8;
        locals.var_sp_s_c__blk1455_dn9 = assign49400_e63423_d_n9;

        let (assign49410_e63440, assign49410_e63440_d_n4, assign49410_e63440_d_n6, assign49410_e63440_d_n7, assign49410_e63440_d_n8, assign49410_e63440_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49410_e63433: f64 = (-locals.var_sp_s_eta__blk1453);
        let assign49410_e63436: f64 = (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341);
        let assign49410_e63437: f64 = (assign49410_e63436).ln();
        let assign49410_e63438: f64 = (assign49410_e63433 + assign49410_e63437);
        (assign49410_e63438, ((-locals.var_sp_s_eta__blk1453_dn4) + (((locals.var_sp_s_a__blk1454_dn4 * locals.var_inv_gf2__blk1341) + (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341_dn4)) / assign49410_e63436)), ((-locals.var_sp_s_eta__blk1453_dn6) + (((locals.var_sp_s_a__blk1454_dn6 * locals.var_inv_gf2__blk1341) + (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341_dn6)) / assign49410_e63436)), ((-locals.var_sp_s_eta__blk1453_dn7) + (((locals.var_sp_s_a__blk1454_dn7 * locals.var_inv_gf2__blk1341) + (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341_dn7)) / assign49410_e63436)), ((-locals.var_sp_s_eta__blk1453_dn8) + (((locals.var_sp_s_a__blk1454_dn8 * locals.var_inv_gf2__blk1341) + (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341_dn8)) / assign49410_e63436)), ((-locals.var_sp_s_eta__blk1453_dn9) + (((locals.var_sp_s_a__blk1454_dn9 * locals.var_inv_gf2__blk1341) + (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341_dn9)) / assign49410_e63436)),)
    } else {
        (locals.var_sp_s_tau__blk1456, locals.var_sp_s_tau__blk1456_dn4, locals.var_sp_s_tau__blk1456_dn6, locals.var_sp_s_tau__blk1456_dn7, locals.var_sp_s_tau__blk1456_dn8, locals.var_sp_s_tau__blk1456_dn9,)
    }
};
        locals.var_sp_s_tau__blk1456 = assign49410_e63440;
        locals.var_sp_s_tau__blk1456_dn4 = assign49410_e63440_d_n4;
        locals.var_sp_s_tau__blk1456_dn6 = assign49410_e63440_d_n6;
        locals.var_sp_s_tau__blk1456_dn7 = assign49410_e63440_d_n7;
        locals.var_sp_s_tau__blk1456_dn8 = assign49410_e63440_d_n8;
        locals.var_sp_s_tau__blk1456_dn9 = assign49410_e63440_d_n9;

        let (assign49420_e63453, assign49420_e63453_d_n4, assign49420_e63453_d_n6, assign49420_e63453_d_n7, assign49420_e63453_d_n8, assign49420_e63453_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49420_e63451: f64 = (locals.var_sp_s_a__blk1454 + locals.var_sp_s_c__blk1455);
        (assign49420_e63451, (locals.var_sp_s_a__blk1454_dn4 + locals.var_sp_s_c__blk1455_dn4), (locals.var_sp_s_a__blk1454_dn6 + locals.var_sp_s_c__blk1455_dn6), (locals.var_sp_s_a__blk1454_dn7 + locals.var_sp_s_c__blk1455_dn7), (locals.var_sp_s_a__blk1454_dn8 + locals.var_sp_s_c__blk1455_dn8), (locals.var_sp_s_a__blk1454_dn9 + locals.var_sp_s_c__blk1455_dn9),)
    } else {
        (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9,)
    }
};
        locals.var_nu = assign49420_e63453;
        locals.var_nu_dn4 = assign49420_e63453_d_n4;
        locals.var_nu_dn6 = assign49420_e63453_d_n6;
        locals.var_nu_dn7 = assign49420_e63453_d_n7;
        locals.var_nu_dn8 = assign49420_e63453_d_n8;
        locals.var_nu_dn9 = assign49420_e63453_d_n9;

        let (assign49430_e63476, assign49430_e63476_d_n4, assign49430_e63476_d_n6, assign49430_e63476_d_n7, assign49430_e63476_d_n8, assign49430_e63476_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49430_e63464: f64 = (locals.var_nu * locals.var_nu);
        let assign49430_e63469: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
        let assign49430_e63470: f64 = (0.5 * assign49430_e63469);
        let assign49430_e63472: f64 = (assign49430_e63470 - locals.var_sp_s_a__blk1454);
        let assign49430_e63473: f64 = (locals.var_sp_s_tau__blk1456 * assign49430_e63472);
        let assign49430_e63474: f64 = (assign49430_e63464 + assign49430_e63473);
        (assign49430_e63474, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau__blk1456_dn4 * assign49430_e63472) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4))) - locals.var_sp_s_a__blk1454_dn4)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1456_dn6 * assign49430_e63472) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6))) - locals.var_sp_s_a__blk1454_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1456_dn7 * assign49430_e63472) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7))) - locals.var_sp_s_a__blk1454_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1456_dn8 * assign49430_e63472) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8))) - locals.var_sp_s_a__blk1454_dn8)))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau__blk1456_dn9 * assign49430_e63472) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9))) - locals.var_sp_s_a__blk1454_dn9)))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9,)
    }
};
        locals.var_mutau = assign49430_e63476;
        locals.var_mutau_dn4 = assign49430_e63476_d_n4;
        locals.var_mutau_dn6 = assign49430_e63476_d_n6;
        locals.var_mutau_dn7 = assign49430_e63476_d_n7;
        locals.var_mutau_dn8 = assign49430_e63476_d_n8;
        locals.var_mutau_dn9 = assign49430_e63476_d_n9;

        let (assign49440_e63513, assign49440_e63513_d_n4, assign49440_e63513_d_n6, assign49440_e63513_d_n7, assign49440_e63513_d_n8, assign49440_e63513_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49440_e63488: f64 = (locals.var_sp_s_a__blk1454 * locals.var_nu);
        let assign49440_e63490: f64 = (assign49440_e63488 * locals.var_sp_s_tau__blk1456);
        let assign49440_e63494: f64 = (locals.var_nu / locals.var_mutau);
        let assign49440_e63496: f64 = (assign49440_e63494 * locals.var_sp_s_tau__blk1456);
        let assign49440_e63498: f64 = (assign49440_e63496 * locals.var_sp_s_tau__blk1456);
        let assign49440_e63500: f64 = (assign49440_e63498 * locals.var_sp_s_c__blk1455);
        let assign49440_e63503: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
        let assign49440_e63505: f64 = (assign49440_e63503 * 0.3333333333333333);
        let assign49440_e63507: f64 = (assign49440_e63505 - locals.var_sp_s_a__blk1454);
        let assign49440_e63508: f64 = (assign49440_e63500 * assign49440_e63507);
        let assign49440_e63509: f64 = (locals.var_mutau + assign49440_e63508);
        let assign49440_e63510: f64 = (assign49440_e63490 / assign49440_e63509);
        let assign49440_e63511: f64 = (locals.var_sp_s_eta__blk1453 + assign49440_e63510);
        (assign49440_e63511, (locals.var_sp_s_eta__blk1453_dn4 + (((((((locals.var_sp_s_a__blk1454_dn4 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn4)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63488 * locals.var_sp_s_tau__blk1456_dn4)) * assign49440_e63509) - (assign49440_e63490 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63494 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63496 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_c__blk1455) + (assign49440_e63498 * locals.var_sp_s_c__blk1455_dn4)) * assign49440_e63507) + (assign49440_e63500 * ((((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4)) * 0.3333333333333333) - locals.var_sp_s_a__blk1454_dn4)))))) / (assign49440_e63509 * assign49440_e63509))), (locals.var_sp_s_eta__blk1453_dn6 + (((((((locals.var_sp_s_a__blk1454_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63488 * locals.var_sp_s_tau__blk1456_dn6)) * assign49440_e63509) - (assign49440_e63490 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63494 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63496 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_c__blk1455) + (assign49440_e63498 * locals.var_sp_s_c__blk1455_dn6)) * assign49440_e63507) + (assign49440_e63500 * ((((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6)) * 0.3333333333333333) - locals.var_sp_s_a__blk1454_dn6)))))) / (assign49440_e63509 * assign49440_e63509))), (locals.var_sp_s_eta__blk1453_dn7 + (((((((locals.var_sp_s_a__blk1454_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63488 * locals.var_sp_s_tau__blk1456_dn7)) * assign49440_e63509) - (assign49440_e63490 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63494 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63496 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_c__blk1455) + (assign49440_e63498 * locals.var_sp_s_c__blk1455_dn7)) * assign49440_e63507) + (assign49440_e63500 * ((((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7)) * 0.3333333333333333) - locals.var_sp_s_a__blk1454_dn7)))))) / (assign49440_e63509 * assign49440_e63509))), (locals.var_sp_s_eta__blk1453_dn8 + (((((((locals.var_sp_s_a__blk1454_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63488 * locals.var_sp_s_tau__blk1456_dn8)) * assign49440_e63509) - (assign49440_e63490 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63494 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63496 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_c__blk1455) + (assign49440_e63498 * locals.var_sp_s_c__blk1455_dn8)) * assign49440_e63507) + (assign49440_e63500 * ((((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8)) * 0.3333333333333333) - locals.var_sp_s_a__blk1454_dn8)))))) / (assign49440_e63509 * assign49440_e63509))), (locals.var_sp_s_eta__blk1453_dn9 + (((((((locals.var_sp_s_a__blk1454_dn9 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn9)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63488 * locals.var_sp_s_tau__blk1456_dn9)) * assign49440_e63509) - (assign49440_e63490 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63494 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63496 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_c__blk1455) + (assign49440_e63498 * locals.var_sp_s_c__blk1455_dn9)) * assign49440_e63507) + (assign49440_e63500 * ((((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9)) * 0.3333333333333333) - locals.var_sp_s_a__blk1454_dn9)))))) / (assign49440_e63509 * assign49440_e63509))),)
    } else {
        (locals.var_sp_s_y0__blk1457, locals.var_sp_s_y0__blk1457_dn4, locals.var_sp_s_y0__blk1457_dn6, locals.var_sp_s_y0__blk1457_dn7, locals.var_sp_s_y0__blk1457_dn8, locals.var_sp_s_y0__blk1457_dn9,)
    }
};
        locals.var_sp_s_y0__blk1457 = assign49440_e63513;
        locals.var_sp_s_y0__blk1457_dn4 = assign49440_e63513_d_n4;
        locals.var_sp_s_y0__blk1457_dn6 = assign49440_e63513_d_n6;
        locals.var_sp_s_y0__blk1457_dn7 = assign49440_e63513_d_n7;
        locals.var_sp_s_y0__blk1457_dn8 = assign49440_e63513_d_n8;
        locals.var_sp_s_y0__blk1457_dn9 = assign49440_e63513_d_n9;

        let assign49450_e63516: f64 = if locals.var_sp_s_y0__blk1457 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1487 = assign49450_e63516;

        let (assign49460_e63530, assign49460_e63530_d_n4, assign49460_e63530_d_n6, assign49460_e63530_d_n7, assign49460_e63530_d_n8, assign49460_e63530_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) && (locals.var_guard1487 != 0.0)) {
        let assign49460_e63528: f64 = (locals.var_sp_s_y0__blk1457).exp();
        (assign49460_e63528, (assign49460_e63528 * locals.var_sp_s_y0__blk1457_dn4), (assign49460_e63528 * locals.var_sp_s_y0__blk1457_dn6), (assign49460_e63528 * locals.var_sp_s_y0__blk1457_dn7), (assign49460_e63528 * locals.var_sp_s_y0__blk1457_dn8), (assign49460_e63528 * locals.var_sp_s_y0__blk1457_dn9),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign49460_e63530;
        locals.var_sp_s_delta0__blk1458_dn4 = assign49460_e63530_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign49460_e63530_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign49460_e63530_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign49460_e63530_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign49460_e63530_d_n9;

        let (assign49470_e63566, assign49470_e63566_d_n4, assign49470_e63566_d_n6, assign49470_e63566_d_n7, assign49470_e63566_d_n8, assign49470_e63566_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) && (locals.var_guard1487 == 0.0)) {
        let assign49470_e63546: f64 = (locals.var_sp_s_y0__blk1457 - 230.25850929940458);
        let assign49470_e63551: f64 = (locals.var_sp_s_y0__blk1457 - 230.25850929940458);
        let assign49470_e63555: f64 = (locals.var_sp_s_y0__blk1457 - 230.25850929940458);
        let assign49470_e63557: f64 = (assign49470_e63555 * 0.3333333333333333);
        let assign49470_e63558: f64 = (1.0 + assign49470_e63557);
        let assign49470_e63559: f64 = (assign49470_e63551 * assign49470_e63558);
        let assign49470_e63560: f64 = (0.5 * assign49470_e63559);
        let assign49470_e63561: f64 = (1.0 + assign49470_e63560);
        let assign49470_e63562: f64 = (assign49470_e63546 * assign49470_e63561);
        let assign49470_e63563: f64 = (1.0 + assign49470_e63562);
        let assign49470_e63564: f64 = (1e100 * assign49470_e63563);
        (assign49470_e63564, (1e100 * ((locals.var_sp_s_y0__blk1457_dn4 * assign49470_e63561) + (assign49470_e63546 * (0.5 * ((locals.var_sp_s_y0__blk1457_dn4 * assign49470_e63558) + (assign49470_e63551 * (locals.var_sp_s_y0__blk1457_dn4 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1457_dn6 * assign49470_e63561) + (assign49470_e63546 * (0.5 * ((locals.var_sp_s_y0__blk1457_dn6 * assign49470_e63558) + (assign49470_e63551 * (locals.var_sp_s_y0__blk1457_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1457_dn7 * assign49470_e63561) + (assign49470_e63546 * (0.5 * ((locals.var_sp_s_y0__blk1457_dn7 * assign49470_e63558) + (assign49470_e63551 * (locals.var_sp_s_y0__blk1457_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1457_dn8 * assign49470_e63561) + (assign49470_e63546 * (0.5 * ((locals.var_sp_s_y0__blk1457_dn8 * assign49470_e63558) + (assign49470_e63551 * (locals.var_sp_s_y0__blk1457_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1457_dn9 * assign49470_e63561) + (assign49470_e63546 * (0.5 * ((locals.var_sp_s_y0__blk1457_dn9 * assign49470_e63558) + (assign49470_e63551 * (locals.var_sp_s_y0__blk1457_dn9 * 0.3333333333333333))))))),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign49470_e63566;
        locals.var_sp_s_delta0__blk1458_dn4 = assign49470_e63566_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign49470_e63566_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign49470_e63566_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign49470_e63566_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign49470_e63566_d_n9;

        let (assign49480_e63579, assign49480_e63579_d_n4, assign49480_e63579_d_n6, assign49480_e63579_d_n7, assign49480_e63579_d_n8, assign49480_e63579_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49480_e63577: f64 = (1.0 / locals.var_sp_s_delta0__blk1458);
        (assign49480_e63577, (-(locals.var_sp_s_delta0__blk1458_dn4 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn6 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn7 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn8 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn9 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))),)
    } else {
        (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9,)
    }
};
        locals.var_sp_s_delta1__blk1459 = assign49480_e63579;
        locals.var_sp_s_delta1__blk1459_dn4 = assign49480_e63579_d_n4;
        locals.var_sp_s_delta1__blk1459_dn6 = assign49480_e63579_d_n6;
        locals.var_sp_s_delta1__blk1459_dn7 = assign49480_e63579_d_n7;
        locals.var_sp_s_delta1__blk1459_dn8 = assign49480_e63579_d_n8;
        locals.var_sp_s_delta1__blk1459_dn9 = assign49480_e63579_d_n9;

        let (assign49490_e63596, assign49490_e63596_d_n4, assign49490_e63596_d_n6, assign49490_e63596_d_n7, assign49490_e63596_d_n8, assign49490_e63596_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49490_e63592: f64 = (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457);
        let assign49490_e63593: f64 = (2.0 + assign49490_e63592);
        let assign49490_e63594: f64 = (1.0 / assign49490_e63593);
        (assign49490_e63594, (-(((locals.var_sp_s_y0__blk1457_dn4 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn4)) / (assign49490_e63593 * assign49490_e63593))), (-(((locals.var_sp_s_y0__blk1457_dn6 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn6)) / (assign49490_e63593 * assign49490_e63593))), (-(((locals.var_sp_s_y0__blk1457_dn7 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn7)) / (assign49490_e63593 * assign49490_e63593))), (-(((locals.var_sp_s_y0__blk1457_dn8 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn8)) / (assign49490_e63593 * assign49490_e63593))), (-(((locals.var_sp_s_y0__blk1457_dn9 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn9)) / (assign49490_e63593 * assign49490_e63593))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49490_e63596;
        locals.var_sp_s_temp__blk1448_dn4 = assign49490_e63596_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49490_e63596_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49490_e63596_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49490_e63596_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49490_e63596_d_n9;

        let (assign49500_e63611, assign49500_e63611_d_n4, assign49500_e63611_d_n6, assign49500_e63611_d_n7, assign49500_e63611_d_n8, assign49500_e63611_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49500_e63607: f64 = (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457);
        let assign49500_e63609: f64 = (assign49500_e63607 * locals.var_sp_s_temp__blk1448);
        (assign49500_e63609, ((((locals.var_sp_s_y0__blk1457_dn4 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49500_e63607 * locals.var_sp_s_temp__blk1448_dn4)), ((((locals.var_sp_s_y0__blk1457_dn6 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49500_e63607 * locals.var_sp_s_temp__blk1448_dn6)), ((((locals.var_sp_s_y0__blk1457_dn7 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49500_e63607 * locals.var_sp_s_temp__blk1448_dn7)), ((((locals.var_sp_s_y0__blk1457_dn8 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49500_e63607 * locals.var_sp_s_temp__blk1448_dn8)), ((((locals.var_sp_s_y0__blk1457_dn9 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49500_e63607 * locals.var_sp_s_temp__blk1448_dn9)),)
    } else {
        (locals.var_sp_s_xi0__blk1460, locals.var_sp_s_xi0__blk1460_dn4, locals.var_sp_s_xi0__blk1460_dn6, locals.var_sp_s_xi0__blk1460_dn7, locals.var_sp_s_xi0__blk1460_dn8, locals.var_sp_s_xi0__blk1460_dn9,)
    }
};
        locals.var_sp_s_xi0__blk1460 = assign49500_e63611;
        locals.var_sp_s_xi0__blk1460_dn4 = assign49500_e63611_d_n4;
        locals.var_sp_s_xi0__blk1460_dn6 = assign49500_e63611_d_n6;
        locals.var_sp_s_xi0__blk1460_dn7 = assign49500_e63611_d_n7;
        locals.var_sp_s_xi0__blk1460_dn8 = assign49500_e63611_d_n8;
        locals.var_sp_s_xi0__blk1460_dn9 = assign49500_e63611_d_n9;

        let (assign49510_e63628, assign49510_e63628_d_n4, assign49510_e63628_d_n6, assign49510_e63628_d_n7, assign49510_e63628_d_n8, assign49510_e63628_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49510_e63623: f64 = (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448);
        let assign49510_e63625: f64 = (assign49510_e63623 * locals.var_sp_s_temp__blk1448);
        let assign49510_e63626: f64 = (4.0 * assign49510_e63625);
        (assign49510_e63626, (4.0 * ((((locals.var_sp_s_y0__blk1457_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49510_e63623 * locals.var_sp_s_temp__blk1448_dn4))), (4.0 * ((((locals.var_sp_s_y0__blk1457_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49510_e63623 * locals.var_sp_s_temp__blk1448_dn6))), (4.0 * ((((locals.var_sp_s_y0__blk1457_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49510_e63623 * locals.var_sp_s_temp__blk1448_dn7))), (4.0 * ((((locals.var_sp_s_y0__blk1457_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49510_e63623 * locals.var_sp_s_temp__blk1448_dn8))), (4.0 * ((((locals.var_sp_s_y0__blk1457_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49510_e63623 * locals.var_sp_s_temp__blk1448_dn9))),)
    } else {
        (locals.var_sp_s_xi1__blk1461, locals.var_sp_s_xi1__blk1461_dn4, locals.var_sp_s_xi1__blk1461_dn6, locals.var_sp_s_xi1__blk1461_dn7, locals.var_sp_s_xi1__blk1461_dn8, locals.var_sp_s_xi1__blk1461_dn9,)
    }
};
        locals.var_sp_s_xi1__blk1461 = assign49510_e63628;
        locals.var_sp_s_xi1__blk1461_dn4 = assign49510_e63628_d_n4;
        locals.var_sp_s_xi1__blk1461_dn6 = assign49510_e63628_d_n6;
        locals.var_sp_s_xi1__blk1461_dn7 = assign49510_e63628_d_n7;
        locals.var_sp_s_xi1__blk1461_dn8 = assign49510_e63628_d_n8;
        locals.var_sp_s_xi1__blk1461_dn9 = assign49510_e63628_d_n9;

        let (assign49520_e63649, assign49520_e63649_d_n4, assign49520_e63649_d_n6, assign49520_e63649_d_n7, assign49520_e63649_d_n8, assign49520_e63649_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49520_e63639: f64 = (8.0 * locals.var_sp_s_temp__blk1448);
        let assign49520_e63642: f64 = (12.0 * locals.var_sp_s_xi0__blk1460);
        let assign49520_e63643: f64 = (assign49520_e63639 - assign49520_e63642);
        let assign49520_e63645: f64 = (assign49520_e63643 * locals.var_sp_s_temp__blk1448);
        let assign49520_e63647: f64 = (assign49520_e63645 * locals.var_sp_s_temp__blk1448);
        (assign49520_e63647, ((((((8.0 * locals.var_sp_s_temp__blk1448_dn4) - (12.0 * locals.var_sp_s_xi0__blk1460_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63643 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63645 * locals.var_sp_s_temp__blk1448_dn4)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn6) - (12.0 * locals.var_sp_s_xi0__blk1460_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63643 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63645 * locals.var_sp_s_temp__blk1448_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn7) - (12.0 * locals.var_sp_s_xi0__blk1460_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63643 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63645 * locals.var_sp_s_temp__blk1448_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn8) - (12.0 * locals.var_sp_s_xi0__blk1460_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63643 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63645 * locals.var_sp_s_temp__blk1448_dn8)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn9) - (12.0 * locals.var_sp_s_xi0__blk1460_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63643 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63645 * locals.var_sp_s_temp__blk1448_dn9)),)
    } else {
        (locals.var_sp_s_xi2__blk1462, locals.var_sp_s_xi2__blk1462_dn4, locals.var_sp_s_xi2__blk1462_dn6, locals.var_sp_s_xi2__blk1462_dn7, locals.var_sp_s_xi2__blk1462_dn8, locals.var_sp_s_xi2__blk1462_dn9,)
    }
};
        locals.var_sp_s_xi2__blk1462 = assign49520_e63649;
        locals.var_sp_s_xi2__blk1462_dn4 = assign49520_e63649_d_n4;
        locals.var_sp_s_xi2__blk1462_dn6 = assign49520_e63649_d_n6;
        locals.var_sp_s_xi2__blk1462_dn7 = assign49520_e63649_d_n7;
        locals.var_sp_s_xi2__blk1462_dn8 = assign49520_e63649_d_n8;
        locals.var_sp_s_xi2__blk1462_dn9 = assign49520_e63649_d_n9;

        let (assign49530_e63662, assign49530_e63662_d_n4, assign49530_e63662_d_n6, assign49530_e63662_d_n7, assign49530_e63662_d_n8, assign49530_e63662_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49530_e63660: f64 = (locals.var_sp_s_yg__blk1451 - locals.var_sp_s_y0__blk1457);
        (assign49530_e63660, (locals.var_sp_s_yg__blk1451_dn4 - locals.var_sp_s_y0__blk1457_dn4), (locals.var_sp_s_yg__blk1451_dn6 - locals.var_sp_s_y0__blk1457_dn6), (locals.var_sp_s_yg__blk1451_dn7 - locals.var_sp_s_y0__blk1457_dn7), (locals.var_sp_s_yg__blk1451_dn8 - locals.var_sp_s_y0__blk1457_dn8), (locals.var_sp_s_yg__blk1451_dn9 - locals.var_sp_s_y0__blk1457_dn9),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49530_e63662;
        locals.var_sp_s_temp__blk1448_dn4 = assign49530_e63662_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49530_e63662_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49530_e63662_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49530_e63662_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49530_e63662_d_n9;

        let (assign49540_e63675, assign49540_e63675_d_n4, assign49540_e63675_d_n6, assign49540_e63675_d_n7, assign49540_e63675_d_n8, assign49540_e63675_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49540_e63673: f64 = (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459);
        (assign49540_e63673, ((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_delta1__blk1459) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459_dn4)), ((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_delta1__blk1459) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459_dn6)), ((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_delta1__blk1459) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459_dn7)), ((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_delta1__blk1459) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459_dn8)), ((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_delta1__blk1459) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459_dn9)),)
    } else {
        (locals.var_sp_s_temp1__blk1449, locals.var_sp_s_temp1__blk1449_dn4, locals.var_sp_s_temp1__blk1449_dn6, locals.var_sp_s_temp1__blk1449_dn7, locals.var_sp_s_temp1__blk1449_dn8, locals.var_sp_s_temp1__blk1449_dn9,)
    }
};
        locals.var_sp_s_temp1__blk1449 = assign49540_e63675;
        locals.var_sp_s_temp1__blk1449_dn4 = assign49540_e63675_d_n4;
        locals.var_sp_s_temp1__blk1449_dn6 = assign49540_e63675_d_n6;
        locals.var_sp_s_temp1__blk1449_dn7 = assign49540_e63675_d_n7;
        locals.var_sp_s_temp1__blk1449_dn8 = assign49540_e63675_d_n8;
        locals.var_sp_s_temp1__blk1449_dn9 = assign49540_e63675_d_n9;

        let (assign49550_e63702, assign49550_e63702_d_n4, assign49550_e63702_d_n6, assign49550_e63702_d_n7, assign49550_e63702_d_n8, assign49550_e63702_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49550_e63686: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
        let assign49550_e63690: f64 = (locals.var_sp_s_delta0__blk1458 - 1.0);
        let assign49550_e63692: f64 = (assign49550_e63690 - locals.var_sp_s_temp1__blk1449);
        let assign49550_e63696: f64 = (1.0 - locals.var_sp_s_xi1__blk1461);
        let assign49550_e63697: f64 = (locals.var_delta_ns__blk1364 * assign49550_e63696);
        let assign49550_e63698: f64 = (assign49550_e63692 + assign49550_e63697);
        let assign49550_e63699: f64 = (locals.var_gf2__blk1325 * assign49550_e63698);
        let assign49550_e63700: f64 = (assign49550_e63686 + assign49550_e63699);
        (assign49550_e63700, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) + ((locals.var_gf2__blk1325_dn4 * assign49550_e63698) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn4 - locals.var_sp_s_temp1__blk1449_dn4) + ((locals.var_delta_ns__blk1364_dn4 * assign49550_e63696) + (locals.var_delta_ns__blk1364 * (-locals.var_sp_s_xi1__blk1461_dn4))))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) + ((locals.var_gf2__blk1325_dn6 * assign49550_e63698) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn6 - locals.var_sp_s_temp1__blk1449_dn6) + ((locals.var_delta_ns__blk1364_dn6 * assign49550_e63696) + (locals.var_delta_ns__blk1364 * (-locals.var_sp_s_xi1__blk1461_dn6))))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) + ((locals.var_gf2__blk1325_dn7 * assign49550_e63698) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn7 - locals.var_sp_s_temp1__blk1449_dn7) + ((locals.var_delta_ns__blk1364_dn7 * assign49550_e63696) + (locals.var_delta_ns__blk1364 * (-locals.var_sp_s_xi1__blk1461_dn7))))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) + ((locals.var_gf2__blk1325_dn8 * assign49550_e63698) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn8 - locals.var_sp_s_temp1__blk1449_dn8) + ((locals.var_delta_ns__blk1364_dn8 * assign49550_e63696) + (locals.var_delta_ns__blk1364 * (-locals.var_sp_s_xi1__blk1461_dn8))))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) + ((locals.var_gf2__blk1325_dn9 * assign49550_e63698) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn9 - locals.var_sp_s_temp1__blk1449_dn9) + ((locals.var_delta_ns__blk1364_dn9 * assign49550_e63696) + (locals.var_delta_ns__blk1364 * (-locals.var_sp_s_xi1__blk1461_dn9))))))),)
    } else {
        (locals.var_sp_s_pc__blk1463, locals.var_sp_s_pc__blk1463_dn4, locals.var_sp_s_pc__blk1463_dn6, locals.var_sp_s_pc__blk1463_dn7, locals.var_sp_s_pc__blk1463_dn8, locals.var_sp_s_pc__blk1463_dn9,)
    }
};
        locals.var_sp_s_pc__blk1463 = assign49550_e63702;
        locals.var_sp_s_pc__blk1463_dn4 = assign49550_e63702_d_n4;
        locals.var_sp_s_pc__blk1463_dn6 = assign49550_e63702_d_n6;
        locals.var_sp_s_pc__blk1463_dn7 = assign49550_e63702_d_n7;
        locals.var_sp_s_pc__blk1463_dn8 = assign49550_e63702_d_n8;
        locals.var_sp_s_pc__blk1463_dn9 = assign49550_e63702_d_n9;

        let (assign49560_e63733, assign49560_e63733_d_n4, assign49560_e63733_d_n6, assign49560_e63733_d_n7, assign49560_e63733_d_n8, assign49560_e63733_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49560_e63713: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
        let assign49560_e63717: f64 = (locals.var_sp_s_delta0__blk1458 - locals.var_sp_s_y0__blk1457);
        let assign49560_e63719: f64 = (assign49560_e63717 - 1.0);
        let assign49560_e63721: f64 = (assign49560_e63719 + locals.var_sp_s_temp1__blk1449);
        let assign49560_e63725: f64 = (locals.var_sp_s_y0__blk1457 - 1.0);
        let assign49560_e63727: f64 = (assign49560_e63725 - locals.var_sp_s_xi0__blk1460);
        let assign49560_e63728: f64 = (locals.var_delta_ns__blk1364 * assign49560_e63727);
        let assign49560_e63729: f64 = (assign49560_e63721 + assign49560_e63728);
        let assign49560_e63730: f64 = (locals.var_gf2__blk1325 * assign49560_e63729);
        let assign49560_e63731: f64 = (assign49560_e63713 - assign49560_e63730);
        (assign49560_e63731, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) - ((locals.var_gf2__blk1325_dn4 * assign49560_e63729) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta0__blk1458_dn4 - locals.var_sp_s_y0__blk1457_dn4) + locals.var_sp_s_temp1__blk1449_dn4) + ((locals.var_delta_ns__blk1364_dn4 * assign49560_e63727) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_y0__blk1457_dn4 - locals.var_sp_s_xi0__blk1460_dn4))))))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) - ((locals.var_gf2__blk1325_dn6 * assign49560_e63729) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta0__blk1458_dn6 - locals.var_sp_s_y0__blk1457_dn6) + locals.var_sp_s_temp1__blk1449_dn6) + ((locals.var_delta_ns__blk1364_dn6 * assign49560_e63727) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_y0__blk1457_dn6 - locals.var_sp_s_xi0__blk1460_dn6))))))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) - ((locals.var_gf2__blk1325_dn7 * assign49560_e63729) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta0__blk1458_dn7 - locals.var_sp_s_y0__blk1457_dn7) + locals.var_sp_s_temp1__blk1449_dn7) + ((locals.var_delta_ns__blk1364_dn7 * assign49560_e63727) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_y0__blk1457_dn7 - locals.var_sp_s_xi0__blk1460_dn7))))))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) - ((locals.var_gf2__blk1325_dn8 * assign49560_e63729) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta0__blk1458_dn8 - locals.var_sp_s_y0__blk1457_dn8) + locals.var_sp_s_temp1__blk1449_dn8) + ((locals.var_delta_ns__blk1364_dn8 * assign49560_e63727) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_y0__blk1457_dn8 - locals.var_sp_s_xi0__blk1460_dn8))))))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) - ((locals.var_gf2__blk1325_dn9 * assign49560_e63729) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta0__blk1458_dn9 - locals.var_sp_s_y0__blk1457_dn9) + locals.var_sp_s_temp1__blk1449_dn9) + ((locals.var_delta_ns__blk1364_dn9 * assign49560_e63727) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_y0__blk1457_dn9 - locals.var_sp_s_xi0__blk1460_dn9))))))),)
    } else {
        (locals.var_sp_s_qc__blk1464, locals.var_sp_s_qc__blk1464_dn4, locals.var_sp_s_qc__blk1464_dn6, locals.var_sp_s_qc__blk1464_dn7, locals.var_sp_s_qc__blk1464_dn8, locals.var_sp_s_qc__blk1464_dn9,)
    }
};
        locals.var_sp_s_qc__blk1464 = assign49560_e63733;
        locals.var_sp_s_qc__blk1464_dn4 = assign49560_e63733_d_n4;
        locals.var_sp_s_qc__blk1464_dn6 = assign49560_e63733_d_n6;
        locals.var_sp_s_qc__blk1464_dn7 = assign49560_e63733_d_n7;
        locals.var_sp_s_qc__blk1464_dn8 = assign49560_e63733_d_n8;
        locals.var_sp_s_qc__blk1464_dn9 = assign49560_e63733_d_n9;

        let (assign49570_e63754, assign49570_e63754_d_n4, assign49570_e63754_d_n6, assign49570_e63754_d_n7, assign49570_e63754_d_n8, assign49570_e63754_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49570_e63746: f64 = (locals.var_sp_s_delta0__blk1458 + locals.var_sp_s_temp1__blk1449);
        let assign49570_e63749: f64 = (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462);
        let assign49570_e63750: f64 = (assign49570_e63746 - assign49570_e63749);
        let assign49570_e63751: f64 = (locals.var_gf2__blk1325 * assign49570_e63750);
        let assign49570_e63752: f64 = (2.0 - assign49570_e63751);
        (assign49570_e63752, (-((locals.var_gf2__blk1325_dn4 * assign49570_e63750) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn4 + locals.var_sp_s_temp1__blk1449_dn4) - ((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn4)))))), (-((locals.var_gf2__blk1325_dn6 * assign49570_e63750) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn6 + locals.var_sp_s_temp1__blk1449_dn6) - ((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn6)))))), (-((locals.var_gf2__blk1325_dn7 * assign49570_e63750) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn7 + locals.var_sp_s_temp1__blk1449_dn7) - ((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn7)))))), (-((locals.var_gf2__blk1325_dn8 * assign49570_e63750) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn8 + locals.var_sp_s_temp1__blk1449_dn8) - ((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn8)))))), (-((locals.var_gf2__blk1325_dn9 * assign49570_e63750) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn9 + locals.var_sp_s_temp1__blk1449_dn9) - ((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn9)))))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49570_e63754;
        locals.var_sp_s_temp__blk1448_dn4 = assign49570_e63754_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49570_e63754_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49570_e63754_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49570_e63754_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49570_e63754_d_n9;

    }

    pub(super) fn stamp_transient_block_39(
        locals: &mut StampLocals,
    ) {
        let (assign49580_e63773, assign49580_e63773_d_n4, assign49580_e63773_d_n6, assign49580_e63773_d_n7, assign49580_e63773_d_n8, assign49580_e63773_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49580_e63765: f64 = (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463);
        let assign49580_e63769: f64 = (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448);
        let assign49580_e63770: f64 = (2.0 * assign49580_e63769);
        let assign49580_e63771: f64 = (assign49580_e63765 - assign49580_e63770);
        (assign49580_e63771, (((locals.var_sp_s_pc__blk1463_dn4 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn4)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn4)))), (((locals.var_sp_s_pc__blk1463_dn6 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn6)))), (((locals.var_sp_s_pc__blk1463_dn7 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn7)))), (((locals.var_sp_s_pc__blk1463_dn8 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn8)))), (((locals.var_sp_s_pc__blk1463_dn9 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn9)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn9)))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49580_e63773;
        locals.var_sp_s_temp__blk1448_dn4 = assign49580_e63773_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49580_e63773_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49580_e63773_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49580_e63773_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49580_e63773_d_n9;

        let (assign49590_e63794, assign49590_e63794_d_n4, assign49590_e63794_d_n6, assign49590_e63794_d_n7, assign49590_e63794_d_n8, assign49590_e63794_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
        let assign49590_e63783: f64 = (-locals.var_sp_s_y0__blk1457);
        let assign49590_e63788: f64 = (locals.var_sp_s_temp__blk1448).sqrt();
        let assign49590_e63789: f64 = (locals.var_sp_s_pc__blk1463 + assign49590_e63788);
        let assign49590_e63790: f64 = (locals.var_sp_s_qc__blk1464 / assign49590_e63789);
        let assign49590_e63791: f64 = (2.0 * assign49590_e63790);
        let assign49590_e63792: f64 = (assign49590_e63783 - assign49590_e63791);
        (assign49590_e63792, ((-locals.var_sp_s_y0__blk1457_dn4) - (2.0 * (((locals.var_sp_s_qc__blk1464_dn4 * assign49590_e63789) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn4 + (locals.var_sp_s_temp__blk1448_dn4 / (2.0 * assign49590_e63788))))) / (assign49590_e63789 * assign49590_e63789)))), ((-locals.var_sp_s_y0__blk1457_dn6) - (2.0 * (((locals.var_sp_s_qc__blk1464_dn6 * assign49590_e63789) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn6 + (locals.var_sp_s_temp__blk1448_dn6 / (2.0 * assign49590_e63788))))) / (assign49590_e63789 * assign49590_e63789)))), ((-locals.var_sp_s_y0__blk1457_dn7) - (2.0 * (((locals.var_sp_s_qc__blk1464_dn7 * assign49590_e63789) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn7 + (locals.var_sp_s_temp__blk1448_dn7 / (2.0 * assign49590_e63788))))) / (assign49590_e63789 * assign49590_e63789)))), ((-locals.var_sp_s_y0__blk1457_dn8) - (2.0 * (((locals.var_sp_s_qc__blk1464_dn8 * assign49590_e63789) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn8 + (locals.var_sp_s_temp__blk1448_dn8 / (2.0 * assign49590_e63788))))) / (assign49590_e63789 * assign49590_e63789)))), ((-locals.var_sp_s_y0__blk1457_dn9) - (2.0 * (((locals.var_sp_s_qc__blk1464_dn9 * assign49590_e63789) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn9 + (locals.var_sp_s_temp__blk1448_dn9 / (2.0 * assign49590_e63788))))) / (assign49590_e63789 * assign49590_e63789)))),)
    } else {
        (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9,)
    }
};
        locals.var_x_s__blk1363 = assign49590_e63794;
        locals.var_x_s__blk1363_dn4 = assign49590_e63794_d_n4;
        locals.var_x_s__blk1363_dn6 = assign49590_e63794_d_n6;
        locals.var_x_s__blk1363_dn7 = assign49590_e63794_d_n7;
        locals.var_x_s__blk1363_dn8 = assign49590_e63794_d_n8;
        locals.var_x_s__blk1363_dn9 = assign49590_e63794_d_n9;

        let (assign49600_e63812, assign49600_e63812_d_n4, assign49600_e63812_d_n6, assign49600_e63812_d_n7, assign49600_e63812_d_n8, assign49600_e63812_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49600_e63808: f64 = (locals.var_gf__blk1324 * 0.7324648775608221);
        let assign49600_e63809: f64 = (1.25 + assign49600_e63808);
        let assign49600_e63810: f64 = (1.0 / assign49600_e63809);
        (assign49600_e63810, (-((locals.var_gf__blk1324_dn4 * 0.7324648775608221) / (assign49600_e63809 * assign49600_e63809))), (-((locals.var_gf__blk1324_dn6 * 0.7324648775608221) / (assign49600_e63809 * assign49600_e63809))), (-((locals.var_gf__blk1324_dn7 * 0.7324648775608221) / (assign49600_e63809 * assign49600_e63809))), (-((locals.var_gf__blk1324_dn8 * 0.7324648775608221) / (assign49600_e63809 * assign49600_e63809))), (-((locals.var_gf__blk1324_dn9 * 0.7324648775608221) / (assign49600_e63809 * assign49600_e63809))),)
    } else {
        (locals.var_sp_xg1__blk1465, locals.var_sp_xg1__blk1465_dn4, locals.var_sp_xg1__blk1465_dn6, locals.var_sp_xg1__blk1465_dn7, locals.var_sp_xg1__blk1465_dn8, locals.var_sp_xg1__blk1465_dn9,)
    }
};
        locals.var_sp_xg1__blk1465 = assign49600_e63812;
        locals.var_sp_xg1__blk1465_dn4 = assign49600_e63812_d_n4;
        locals.var_sp_xg1__blk1465_dn6 = assign49600_e63812_d_n6;
        locals.var_sp_xg1__blk1465_dn7 = assign49600_e63812_d_n7;
        locals.var_sp_xg1__blk1465_dn8 = assign49600_e63812_d_n8;
        locals.var_sp_xg1__blk1465_dn9 = assign49600_e63812_d_n9;

        let (assign49610_e63832, assign49610_e63832_d_n4, assign49610_e63832_d_n6, assign49610_e63832_d_n7, assign49610_e63832_d_n8, assign49610_e63832_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49610_e63824: f64 = (locals.var_xi__blk1360 * 1.25);
        let assign49610_e63826: f64 = (assign49610_e63824 * locals.var_sp_xg1__blk1465);
        let assign49610_e63828: f64 = (assign49610_e63826 - 1.0);
        let assign49610_e63830: f64 = (assign49610_e63828 * locals.var_sp_xg1__blk1465);
        (assign49610_e63830, (((((locals.var_xi__blk1360_dn4 * 1.25) * locals.var_sp_xg1__blk1465) + (assign49610_e63824 * locals.var_sp_xg1__blk1465_dn4)) * locals.var_sp_xg1__blk1465) + (assign49610_e63828 * locals.var_sp_xg1__blk1465_dn4)), (((((locals.var_xi__blk1360_dn6 * 1.25) * locals.var_sp_xg1__blk1465) + (assign49610_e63824 * locals.var_sp_xg1__blk1465_dn6)) * locals.var_sp_xg1__blk1465) + (assign49610_e63828 * locals.var_sp_xg1__blk1465_dn6)), (((((locals.var_xi__blk1360_dn7 * 1.25) * locals.var_sp_xg1__blk1465) + (assign49610_e63824 * locals.var_sp_xg1__blk1465_dn7)) * locals.var_sp_xg1__blk1465) + (assign49610_e63828 * locals.var_sp_xg1__blk1465_dn7)), (((((locals.var_xi__blk1360_dn8 * 1.25) * locals.var_sp_xg1__blk1465) + (assign49610_e63824 * locals.var_sp_xg1__blk1465_dn8)) * locals.var_sp_xg1__blk1465) + (assign49610_e63828 * locals.var_sp_xg1__blk1465_dn8)), (((((locals.var_xi__blk1360_dn9 * 1.25) * locals.var_sp_xg1__blk1465) + (assign49610_e63824 * locals.var_sp_xg1__blk1465_dn9)) * locals.var_sp_xg1__blk1465) + (assign49610_e63828 * locals.var_sp_xg1__blk1465_dn9)),)
    } else {
        (locals.var_sp_s_a_fac__blk1466, locals.var_sp_s_a_fac__blk1466_dn4, locals.var_sp_s_a_fac__blk1466_dn6, locals.var_sp_s_a_fac__blk1466_dn7, locals.var_sp_s_a_fac__blk1466_dn8, locals.var_sp_s_a_fac__blk1466_dn9,)
    }
};
        locals.var_sp_s_a_fac__blk1466 = assign49610_e63832;
        locals.var_sp_s_a_fac__blk1466_dn4 = assign49610_e63832_d_n4;
        locals.var_sp_s_a_fac__blk1466_dn6 = assign49610_e63832_d_n6;
        locals.var_sp_s_a_fac__blk1466_dn7 = assign49610_e63832_d_n7;
        locals.var_sp_s_a_fac__blk1466_dn8 = assign49610_e63832_d_n8;
        locals.var_sp_s_a_fac__blk1466_dn9 = assign49610_e63832_d_n9;

        let (assign49620_e63852, assign49620_e63852_d_n4, assign49620_e63852_d_n6, assign49620_e63852_d_n7, assign49620_e63852_d_n8, assign49620_e63852_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49620_e63844: f64 = (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362);
        let assign49620_e63848: f64 = (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343);
        let assign49620_e63849: f64 = (1.0 + assign49620_e63848);
        let assign49620_e63850: f64 = (assign49620_e63844 * assign49620_e63849);
        (assign49620_e63850, ((((locals.var_xg__blk1343_dn4 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn4)) * assign49620_e63849) + (assign49620_e63844 * ((locals.var_sp_s_a_fac__blk1466_dn4 * locals.var_xg__blk1343) + (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343_dn4)))), ((((locals.var_xg__blk1343_dn6 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn6)) * assign49620_e63849) + (assign49620_e63844 * ((locals.var_sp_s_a_fac__blk1466_dn6 * locals.var_xg__blk1343) + (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343_dn6)))), ((((locals.var_xg__blk1343_dn7 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn7)) * assign49620_e63849) + (assign49620_e63844 * ((locals.var_sp_s_a_fac__blk1466_dn7 * locals.var_xg__blk1343) + (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343_dn7)))), ((((locals.var_xg__blk1343_dn8 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn8)) * assign49620_e63849) + (assign49620_e63844 * ((locals.var_sp_s_a_fac__blk1466_dn8 * locals.var_xg__blk1343) + (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343_dn8)))), ((((locals.var_xg__blk1343_dn9 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn9)) * assign49620_e63849) + (assign49620_e63844 * ((locals.var_sp_s_a_fac__blk1466_dn9 * locals.var_xg__blk1343) + (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343_dn9)))),)
    } else {
        (locals.var_sp_s_xbar__blk1467, locals.var_sp_s_xbar__blk1467_dn4, locals.var_sp_s_xbar__blk1467_dn6, locals.var_sp_s_xbar__blk1467_dn7, locals.var_sp_s_xbar__blk1467_dn8, locals.var_sp_s_xbar__blk1467_dn9,)
    }
};
        locals.var_sp_s_xbar__blk1467 = assign49620_e63852;
        locals.var_sp_s_xbar__blk1467_dn4 = assign49620_e63852_d_n4;
        locals.var_sp_s_xbar__blk1467_dn6 = assign49620_e63852_d_n6;
        locals.var_sp_s_xbar__blk1467_dn7 = assign49620_e63852_d_n7;
        locals.var_sp_s_xbar__blk1467_dn8 = assign49620_e63852_d_n8;
        locals.var_sp_s_xbar__blk1467_dn9 = assign49620_e63852_d_n9;

        let assign49630_e63854: f64 = (-locals.var_sp_s_xbar__blk1467);
        let assign49630_e63856: f64 = (-230.25850929940458);
        let assign49630_e63857: f64 = if assign49630_e63854 > assign49630_e63856 { 1.0 } else { 0.0 };
        locals.var_guard1488 = assign49630_e63857;

        let (assign49640_e63873, assign49640_e63873_d_n4, assign49640_e63873_d_n6, assign49640_e63873_d_n7, assign49640_e63873_d_n8, assign49640_e63873_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1488 != 0.0)) {
        let assign49640_e63870: f64 = (-locals.var_sp_s_xbar__blk1467);
        let assign49640_e63871: f64 = (assign49640_e63870).exp();
        (assign49640_e63871, (assign49640_e63871 * (-locals.var_sp_s_xbar__blk1467_dn4)), (assign49640_e63871 * (-locals.var_sp_s_xbar__blk1467_dn6)), (assign49640_e63871 * (-locals.var_sp_s_xbar__blk1467_dn7)), (assign49640_e63871 * (-locals.var_sp_s_xbar__blk1467_dn8)), (assign49640_e63871 * (-locals.var_sp_s_xbar__blk1467_dn9)),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49640_e63873;
        locals.var_sp_s_temp__blk1448_dn4 = assign49640_e63873_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49640_e63873_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49640_e63873_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49640_e63873_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49640_e63873_d_n9;

        let (assign49650_e63916, assign49650_e63916_d_n4, assign49650_e63916_d_n6, assign49650_e63916_d_n7, assign49650_e63916_d_n8, assign49650_e63916_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1488 == 0.0)) {
        let assign49650_e63889: f64 = (-230.25850929940458);
        let assign49650_e63891: f64 = (-locals.var_sp_s_xbar__blk1467);
        let assign49650_e63892: f64 = (assign49650_e63889 - assign49650_e63891);
        let assign49650_e63896: f64 = (-230.25850929940458);
        let assign49650_e63898: f64 = (-locals.var_sp_s_xbar__blk1467);
        let assign49650_e63899: f64 = (assign49650_e63896 - assign49650_e63898);
        let assign49650_e63902: f64 = (-230.25850929940458);
        let assign49650_e63904: f64 = (-locals.var_sp_s_xbar__blk1467);
        let assign49650_e63905: f64 = (assign49650_e63902 - assign49650_e63904);
        let assign49650_e63907: f64 = (assign49650_e63905 * 0.3333333333333333);
        let assign49650_e63908: f64 = (1.0 + assign49650_e63907);
        let assign49650_e63909: f64 = (assign49650_e63899 * assign49650_e63908);
        let assign49650_e63910: f64 = (0.5 * assign49650_e63909);
        let assign49650_e63911: f64 = (1.0 + assign49650_e63910);
        let assign49650_e63912: f64 = (assign49650_e63892 * assign49650_e63911);
        let assign49650_e63913: f64 = (1.0 + assign49650_e63912);
        let assign49650_e63914: f64 = (1e-100 / assign49650_e63913);
        (assign49650_e63914, (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1467_dn4)) * assign49650_e63911) + (assign49650_e63892 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1467_dn4)) * assign49650_e63908) + (assign49650_e63899 * ((-(-locals.var_sp_s_xbar__blk1467_dn4)) * 0.3333333333333333))))))) / (assign49650_e63913 * assign49650_e63913))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1467_dn6)) * assign49650_e63911) + (assign49650_e63892 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1467_dn6)) * assign49650_e63908) + (assign49650_e63899 * ((-(-locals.var_sp_s_xbar__blk1467_dn6)) * 0.3333333333333333))))))) / (assign49650_e63913 * assign49650_e63913))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1467_dn7)) * assign49650_e63911) + (assign49650_e63892 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1467_dn7)) * assign49650_e63908) + (assign49650_e63899 * ((-(-locals.var_sp_s_xbar__blk1467_dn7)) * 0.3333333333333333))))))) / (assign49650_e63913 * assign49650_e63913))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1467_dn8)) * assign49650_e63911) + (assign49650_e63892 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1467_dn8)) * assign49650_e63908) + (assign49650_e63899 * ((-(-locals.var_sp_s_xbar__blk1467_dn8)) * 0.3333333333333333))))))) / (assign49650_e63913 * assign49650_e63913))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1467_dn9)) * assign49650_e63911) + (assign49650_e63892 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1467_dn9)) * assign49650_e63908) + (assign49650_e63899 * ((-(-locals.var_sp_s_xbar__blk1467_dn9)) * 0.3333333333333333))))))) / (assign49650_e63913 * assign49650_e63913))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49650_e63916;
        locals.var_sp_s_temp__blk1448_dn4 = assign49650_e63916_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49650_e63916_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49650_e63916_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49650_e63916_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49650_e63916_d_n9;

        let (assign49660_e63930, assign49660_e63930_d_n4, assign49660_e63930_d_n6, assign49660_e63930_d_n7, assign49660_e63930_d_n8, assign49660_e63930_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49660_e63928: f64 = (1.0 - locals.var_sp_s_temp__blk1448);
        (assign49660_e63928, (-locals.var_sp_s_temp__blk1448_dn4), (-locals.var_sp_s_temp__blk1448_dn6), (-locals.var_sp_s_temp__blk1448_dn7), (-locals.var_sp_s_temp__blk1448_dn8), (-locals.var_sp_s_temp__blk1448_dn9),)
    } else {
        (locals.var_sp_s_w__blk1468, locals.var_sp_s_w__blk1468_dn4, locals.var_sp_s_w__blk1468_dn6, locals.var_sp_s_w__blk1468_dn7, locals.var_sp_s_w__blk1468_dn8, locals.var_sp_s_w__blk1468_dn9,)
    }
};
        locals.var_sp_s_w__blk1468 = assign49660_e63930;
        locals.var_sp_s_w__blk1468_dn4 = assign49660_e63930_d_n4;
        locals.var_sp_s_w__blk1468_dn6 = assign49660_e63930_d_n6;
        locals.var_sp_s_w__blk1468_dn7 = assign49660_e63930_d_n7;
        locals.var_sp_s_w__blk1468_dn8 = assign49660_e63930_d_n8;
        locals.var_sp_s_w__blk1468_dn9 = assign49660_e63930_d_n9;

        let (assign49670_e63957, assign49670_e63957_d_n4, assign49670_e63957_d_n6, assign49670_e63957_d_n7, assign49670_e63957_d_n8, assign49670_e63957_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49670_e63943: f64 = (locals.var_gf2__blk1325 * 0.5);
        let assign49670_e63944: f64 = (locals.var_xg__blk1343 + assign49670_e63943);
        let assign49670_e63949: f64 = (locals.var_gf2__blk1325 * 0.25);
        let assign49670_e63950: f64 = (locals.var_xg__blk1343 + assign49670_e63949);
        let assign49670_e63952: f64 = (assign49670_e63950 - locals.var_sp_s_w__blk1468);
        let assign49670_e63953: f64 = (assign49670_e63952).sqrt();
        let assign49670_e63954: f64 = (locals.var_gf__blk1324 * assign49670_e63953);
        let assign49670_e63955: f64 = (assign49670_e63944 - assign49670_e63954);
        (assign49670_e63955, ((locals.var_xg__blk1343_dn4 + (locals.var_gf2__blk1325_dn4 * 0.5)) - ((locals.var_gf__blk1324_dn4 * assign49670_e63953) + (locals.var_gf__blk1324 * (((locals.var_xg__blk1343_dn4 + (locals.var_gf2__blk1325_dn4 * 0.25)) - locals.var_sp_s_w__blk1468_dn4) / (2.0 * assign49670_e63953))))), ((locals.var_xg__blk1343_dn6 + (locals.var_gf2__blk1325_dn6 * 0.5)) - ((locals.var_gf__blk1324_dn6 * assign49670_e63953) + (locals.var_gf__blk1324 * (((locals.var_xg__blk1343_dn6 + (locals.var_gf2__blk1325_dn6 * 0.25)) - locals.var_sp_s_w__blk1468_dn6) / (2.0 * assign49670_e63953))))), ((locals.var_xg__blk1343_dn7 + (locals.var_gf2__blk1325_dn7 * 0.5)) - ((locals.var_gf__blk1324_dn7 * assign49670_e63953) + (locals.var_gf__blk1324 * (((locals.var_xg__blk1343_dn7 + (locals.var_gf2__blk1325_dn7 * 0.25)) - locals.var_sp_s_w__blk1468_dn7) / (2.0 * assign49670_e63953))))), ((locals.var_xg__blk1343_dn8 + (locals.var_gf2__blk1325_dn8 * 0.5)) - ((locals.var_gf__blk1324_dn8 * assign49670_e63953) + (locals.var_gf__blk1324 * (((locals.var_xg__blk1343_dn8 + (locals.var_gf2__blk1325_dn8 * 0.25)) - locals.var_sp_s_w__blk1468_dn8) / (2.0 * assign49670_e63953))))), ((locals.var_xg__blk1343_dn9 + (locals.var_gf2__blk1325_dn9 * 0.5)) - ((locals.var_gf__blk1324_dn9 * assign49670_e63953) + (locals.var_gf__blk1324 * (((locals.var_xg__blk1343_dn9 + (locals.var_gf2__blk1325_dn9 * 0.25)) - locals.var_sp_s_w__blk1468_dn9) / (2.0 * assign49670_e63953))))),)
    } else {
        (locals.var_sp_s_x1__blk1469, locals.var_sp_s_x1__blk1469_dn4, locals.var_sp_s_x1__blk1469_dn6, locals.var_sp_s_x1__blk1469_dn7, locals.var_sp_s_x1__blk1469_dn8, locals.var_sp_s_x1__blk1469_dn9,)
    }
};
        locals.var_sp_s_x1__blk1469 = assign49670_e63957;
        locals.var_sp_s_x1__blk1469_dn4 = assign49670_e63957_d_n4;
        locals.var_sp_s_x1__blk1469_dn6 = assign49670_e63957_d_n6;
        locals.var_sp_s_x1__blk1469_dn7 = assign49670_e63957_d_n7;
        locals.var_sp_s_x1__blk1469_dn8 = assign49670_e63957_d_n8;
        locals.var_sp_s_x1__blk1469_dn9 = assign49670_e63957_d_n9;

        let (assign49680_e63971, assign49680_e63971_d_n4, assign49680_e63971_d_n6, assign49680_e63971_d_n7, assign49680_e63971_d_n8, assign49680_e63971_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49680_e63969: f64 = (locals.var_xn_s__blk1349 + 3.0);
        (assign49680_e63969, locals.var_xn_s__blk1349_dn4, locals.var_xn_s__blk1349_dn6, locals.var_xn_s__blk1349_dn7, locals.var_xn_s__blk1349_dn8, locals.var_xn_s__blk1349_dn9,)
    } else {
        (locals.var_sp_s_bx__blk1470, locals.var_sp_s_bx__blk1470_dn4, locals.var_sp_s_bx__blk1470_dn6, locals.var_sp_s_bx__blk1470_dn7, locals.var_sp_s_bx__blk1470_dn8, locals.var_sp_s_bx__blk1470_dn9,)
    }
};
        locals.var_sp_s_bx__blk1470 = assign49680_e63971;
        locals.var_sp_s_bx__blk1470_dn4 = assign49680_e63971_d_n4;
        locals.var_sp_s_bx__blk1470_dn6 = assign49680_e63971_d_n6;
        locals.var_sp_s_bx__blk1470_dn7 = assign49680_e63971_d_n7;
        locals.var_sp_s_bx__blk1470_dn8 = assign49680_e63971_d_n8;
        locals.var_sp_s_bx__blk1470_dn9 = assign49680_e63971_d_n9;

        let (assign49690_e64009, assign49690_e64009_d_n4, assign49690_e64009_d_n6, assign49690_e64009_d_n7, assign49690_e64009_d_n8, assign49690_e64009_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49690_e63984: f64 = (locals.var_sp_s_x1__blk1469 + locals.var_sp_s_bx__blk1470);
        let assign49690_e63987: f64 = (locals.var_sp_s_x1__blk1469 - locals.var_sp_s_bx__blk1470);
        let assign49690_e63990: f64 = (locals.var_sp_s_x1__blk1469 - locals.var_sp_s_bx__blk1470);
        let assign49690_e63991: f64 = (assign49690_e63987 * assign49690_e63990);
        let assign49690_e63993: f64 = (assign49690_e63991 + 5.0);
        let assign49690_e63994: f64 = (assign49690_e63993).sqrt();
        let assign49690_e63995: f64 = (assign49690_e63984 - assign49690_e63994);
        let assign49690_e63996: f64 = (0.5 * assign49690_e63995);
        let assign49690_e64001: f64 = (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470);
        let assign49690_e64003: f64 = (assign49690_e64001 + 5.0);
        let assign49690_e64004: f64 = (assign49690_e64003).sqrt();
        let assign49690_e64005: f64 = (locals.var_sp_s_bx__blk1470 - assign49690_e64004);
        let assign49690_e64006: f64 = (0.5 * assign49690_e64005);
        let assign49690_e64007: f64 = (assign49690_e63996 - assign49690_e64006);
        (assign49690_e64007, ((0.5 * ((locals.var_sp_s_x1__blk1469_dn4 + locals.var_sp_s_bx__blk1470_dn4) - ((((locals.var_sp_s_x1__blk1469_dn4 - locals.var_sp_s_bx__blk1470_dn4) * assign49690_e63990) + (assign49690_e63987 * (locals.var_sp_s_x1__blk1469_dn4 - locals.var_sp_s_bx__blk1470_dn4))) / (2.0 * assign49690_e63994)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn4 - (((locals.var_sp_s_bx__blk1470_dn4 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn4)) / (2.0 * assign49690_e64004))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn6 + locals.var_sp_s_bx__blk1470_dn6) - ((((locals.var_sp_s_x1__blk1469_dn6 - locals.var_sp_s_bx__blk1470_dn6) * assign49690_e63990) + (assign49690_e63987 * (locals.var_sp_s_x1__blk1469_dn6 - locals.var_sp_s_bx__blk1470_dn6))) / (2.0 * assign49690_e63994)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn6 - (((locals.var_sp_s_bx__blk1470_dn6 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn6)) / (2.0 * assign49690_e64004))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn7 + locals.var_sp_s_bx__blk1470_dn7) - ((((locals.var_sp_s_x1__blk1469_dn7 - locals.var_sp_s_bx__blk1470_dn7) * assign49690_e63990) + (assign49690_e63987 * (locals.var_sp_s_x1__blk1469_dn7 - locals.var_sp_s_bx__blk1470_dn7))) / (2.0 * assign49690_e63994)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn7 - (((locals.var_sp_s_bx__blk1470_dn7 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn7)) / (2.0 * assign49690_e64004))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn8 + locals.var_sp_s_bx__blk1470_dn8) - ((((locals.var_sp_s_x1__blk1469_dn8 - locals.var_sp_s_bx__blk1470_dn8) * assign49690_e63990) + (assign49690_e63987 * (locals.var_sp_s_x1__blk1469_dn8 - locals.var_sp_s_bx__blk1470_dn8))) / (2.0 * assign49690_e63994)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn8 - (((locals.var_sp_s_bx__blk1470_dn8 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn8)) / (2.0 * assign49690_e64004))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn9 + locals.var_sp_s_bx__blk1470_dn9) - ((((locals.var_sp_s_x1__blk1469_dn9 - locals.var_sp_s_bx__blk1470_dn9) * assign49690_e63990) + (assign49690_e63987 * (locals.var_sp_s_x1__blk1469_dn9 - locals.var_sp_s_bx__blk1470_dn9))) / (2.0 * assign49690_e63994)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn9 - (((locals.var_sp_s_bx__blk1470_dn9 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn9)) / (2.0 * assign49690_e64004))))),)
    } else {
        (locals.var_sp_s_eta__blk1453, locals.var_sp_s_eta__blk1453_dn4, locals.var_sp_s_eta__blk1453_dn6, locals.var_sp_s_eta__blk1453_dn7, locals.var_sp_s_eta__blk1453_dn8, locals.var_sp_s_eta__blk1453_dn9,)
    }
};
        locals.var_sp_s_eta__blk1453 = assign49690_e64009;
        locals.var_sp_s_eta__blk1453_dn4 = assign49690_e64009_d_n4;
        locals.var_sp_s_eta__blk1453_dn6 = assign49690_e64009_d_n6;
        locals.var_sp_s_eta__blk1453_dn7 = assign49690_e64009_d_n7;
        locals.var_sp_s_eta__blk1453_dn8 = assign49690_e64009_d_n8;
        locals.var_sp_s_eta__blk1453_dn9 = assign49690_e64009_d_n9;

        let (assign49700_e64023, assign49700_e64023_d_n4, assign49700_e64023_d_n6, assign49700_e64023_d_n7, assign49700_e64023_d_n8, assign49700_e64023_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49700_e64021: f64 = (locals.var_xg__blk1343 - locals.var_sp_s_eta__blk1453);
        (assign49700_e64021, (locals.var_xg__blk1343_dn4 - locals.var_sp_s_eta__blk1453_dn4), (locals.var_xg__blk1343_dn6 - locals.var_sp_s_eta__blk1453_dn6), (locals.var_xg__blk1343_dn7 - locals.var_sp_s_eta__blk1453_dn7), (locals.var_xg__blk1343_dn8 - locals.var_sp_s_eta__blk1453_dn8), (locals.var_xg__blk1343_dn9 - locals.var_sp_s_eta__blk1453_dn9),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49700_e64023;
        locals.var_sp_s_temp__blk1448_dn4 = assign49700_e64023_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49700_e64023_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49700_e64023_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49700_e64023_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49700_e64023_d_n9;

        let (assign49710_e64037, assign49710_e64037_d_n4, assign49710_e64037_d_n6, assign49710_e64037_d_n7, assign49710_e64037_d_n8, assign49710_e64037_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49710_e64034: f64 = (-locals.var_sp_s_eta__blk1453);
        let assign49710_e64035: f64 = (assign49710_e64034).exp();
        (assign49710_e64035, (assign49710_e64035 * (-locals.var_sp_s_eta__blk1453_dn4)), (assign49710_e64035 * (-locals.var_sp_s_eta__blk1453_dn6)), (assign49710_e64035 * (-locals.var_sp_s_eta__blk1453_dn7)), (assign49710_e64035 * (-locals.var_sp_s_eta__blk1453_dn8)), (assign49710_e64035 * (-locals.var_sp_s_eta__blk1453_dn9)),)
    } else {
        (locals.var_sp_s_temp1__blk1449, locals.var_sp_s_temp1__blk1449_dn4, locals.var_sp_s_temp1__blk1449_dn6, locals.var_sp_s_temp1__blk1449_dn7, locals.var_sp_s_temp1__blk1449_dn8, locals.var_sp_s_temp1__blk1449_dn9,)
    }
};
        locals.var_sp_s_temp1__blk1449 = assign49710_e64037;
        locals.var_sp_s_temp1__blk1449_dn4 = assign49710_e64037_d_n4;
        locals.var_sp_s_temp1__blk1449_dn6 = assign49710_e64037_d_n6;
        locals.var_sp_s_temp1__blk1449_dn7 = assign49710_e64037_d_n7;
        locals.var_sp_s_temp1__blk1449_dn8 = assign49710_e64037_d_n8;
        locals.var_sp_s_temp1__blk1449_dn9 = assign49710_e64037_d_n9;

        let (assign49720_e64055, assign49720_e64055_d_n4, assign49720_e64055_d_n6, assign49720_e64055_d_n7, assign49720_e64055_d_n8, assign49720_e64055_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49720_e64051: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453);
        let assign49720_e64052: f64 = (2.0 + assign49720_e64051);
        let assign49720_e64053: f64 = (1.0 / assign49720_e64052);
        (assign49720_e64053, (-(((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn4)) / (assign49720_e64052 * assign49720_e64052))), (-(((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn6)) / (assign49720_e64052 * assign49720_e64052))), (-(((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn7)) / (assign49720_e64052 * assign49720_e64052))), (-(((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn8)) / (assign49720_e64052 * assign49720_e64052))), (-(((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn9)) / (assign49720_e64052 * assign49720_e64052))),)
    } else {
        (locals.var_sp_s_temp2__blk1450, locals.var_sp_s_temp2__blk1450_dn4, locals.var_sp_s_temp2__blk1450_dn6, locals.var_sp_s_temp2__blk1450_dn7, locals.var_sp_s_temp2__blk1450_dn8, locals.var_sp_s_temp2__blk1450_dn9,)
    }
};
        locals.var_sp_s_temp2__blk1450 = assign49720_e64055;
        locals.var_sp_s_temp2__blk1450_dn4 = assign49720_e64055_d_n4;
        locals.var_sp_s_temp2__blk1450_dn6 = assign49720_e64055_d_n6;
        locals.var_sp_s_temp2__blk1450_dn7 = assign49720_e64055_d_n7;
        locals.var_sp_s_temp2__blk1450_dn8 = assign49720_e64055_d_n8;
        locals.var_sp_s_temp2__blk1450_dn9 = assign49720_e64055_d_n9;

        let (assign49730_e64071, assign49730_e64071_d_n4, assign49730_e64071_d_n6, assign49730_e64071_d_n7, assign49730_e64071_d_n8, assign49730_e64071_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49730_e64067: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453);
        let assign49730_e64069: f64 = (assign49730_e64067 * locals.var_sp_s_temp2__blk1450);
        (assign49730_e64069, ((((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign49730_e64067 * locals.var_sp_s_temp2__blk1450_dn4)), ((((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign49730_e64067 * locals.var_sp_s_temp2__blk1450_dn6)), ((((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign49730_e64067 * locals.var_sp_s_temp2__blk1450_dn7)), ((((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign49730_e64067 * locals.var_sp_s_temp2__blk1450_dn8)), ((((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign49730_e64067 * locals.var_sp_s_temp2__blk1450_dn9)),)
    } else {
        (locals.var_sp_s_xi0__blk1460, locals.var_sp_s_xi0__blk1460_dn4, locals.var_sp_s_xi0__blk1460_dn6, locals.var_sp_s_xi0__blk1460_dn7, locals.var_sp_s_xi0__blk1460_dn8, locals.var_sp_s_xi0__blk1460_dn9,)
    }
};
        locals.var_sp_s_xi0__blk1460 = assign49730_e64071;
        locals.var_sp_s_xi0__blk1460_dn4 = assign49730_e64071_d_n4;
        locals.var_sp_s_xi0__blk1460_dn6 = assign49730_e64071_d_n6;
        locals.var_sp_s_xi0__blk1460_dn7 = assign49730_e64071_d_n7;
        locals.var_sp_s_xi0__blk1460_dn8 = assign49730_e64071_d_n8;
        locals.var_sp_s_xi0__blk1460_dn9 = assign49730_e64071_d_n9;

        let (assign49740_e64089, assign49740_e64089_d_n4, assign49740_e64089_d_n6, assign49740_e64089_d_n7, assign49740_e64089_d_n8, assign49740_e64089_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49740_e64084: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450);
        let assign49740_e64086: f64 = (assign49740_e64084 * locals.var_sp_s_temp2__blk1450);
        let assign49740_e64087: f64 = (4.0 * assign49740_e64086);
        (assign49740_e64087, (4.0 * ((((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign49740_e64084 * locals.var_sp_s_temp2__blk1450_dn4))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign49740_e64084 * locals.var_sp_s_temp2__blk1450_dn6))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign49740_e64084 * locals.var_sp_s_temp2__blk1450_dn7))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign49740_e64084 * locals.var_sp_s_temp2__blk1450_dn8))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign49740_e64084 * locals.var_sp_s_temp2__blk1450_dn9))),)
    } else {
        (locals.var_sp_s_xi1__blk1461, locals.var_sp_s_xi1__blk1461_dn4, locals.var_sp_s_xi1__blk1461_dn6, locals.var_sp_s_xi1__blk1461_dn7, locals.var_sp_s_xi1__blk1461_dn8, locals.var_sp_s_xi1__blk1461_dn9,)
    }
};
        locals.var_sp_s_xi1__blk1461 = assign49740_e64089;
        locals.var_sp_s_xi1__blk1461_dn4 = assign49740_e64089_d_n4;
        locals.var_sp_s_xi1__blk1461_dn6 = assign49740_e64089_d_n6;
        locals.var_sp_s_xi1__blk1461_dn7 = assign49740_e64089_d_n7;
        locals.var_sp_s_xi1__blk1461_dn8 = assign49740_e64089_d_n8;
        locals.var_sp_s_xi1__blk1461_dn9 = assign49740_e64089_d_n9;

        let (assign49750_e64111, assign49750_e64111_d_n4, assign49750_e64111_d_n6, assign49750_e64111_d_n7, assign49750_e64111_d_n8, assign49750_e64111_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49750_e64101: f64 = (8.0 * locals.var_sp_s_temp2__blk1450);
        let assign49750_e64104: f64 = (12.0 * locals.var_sp_s_xi0__blk1460);
        let assign49750_e64105: f64 = (assign49750_e64101 - assign49750_e64104);
        let assign49750_e64107: f64 = (assign49750_e64105 * locals.var_sp_s_temp2__blk1450);
        let assign49750_e64109: f64 = (assign49750_e64107 * locals.var_sp_s_temp2__blk1450);
        (assign49750_e64109, ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn4) - (12.0 * locals.var_sp_s_xi0__blk1460_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64105 * locals.var_sp_s_temp2__blk1450_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64107 * locals.var_sp_s_temp2__blk1450_dn4)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn6) - (12.0 * locals.var_sp_s_xi0__blk1460_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64105 * locals.var_sp_s_temp2__blk1450_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64107 * locals.var_sp_s_temp2__blk1450_dn6)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn7) - (12.0 * locals.var_sp_s_xi0__blk1460_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64105 * locals.var_sp_s_temp2__blk1450_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64107 * locals.var_sp_s_temp2__blk1450_dn7)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn8) - (12.0 * locals.var_sp_s_xi0__blk1460_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64105 * locals.var_sp_s_temp2__blk1450_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64107 * locals.var_sp_s_temp2__blk1450_dn8)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn9) - (12.0 * locals.var_sp_s_xi0__blk1460_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64105 * locals.var_sp_s_temp2__blk1450_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64107 * locals.var_sp_s_temp2__blk1450_dn9)),)
    } else {
        (locals.var_sp_s_xi2__blk1462, locals.var_sp_s_xi2__blk1462_dn4, locals.var_sp_s_xi2__blk1462_dn6, locals.var_sp_s_xi2__blk1462_dn7, locals.var_sp_s_xi2__blk1462_dn8, locals.var_sp_s_xi2__blk1462_dn9,)
    }
};
        locals.var_sp_s_xi2__blk1462 = assign49750_e64111;
        locals.var_sp_s_xi2__blk1462_dn4 = assign49750_e64111_d_n4;
        locals.var_sp_s_xi2__blk1462_dn6 = assign49750_e64111_d_n6;
        locals.var_sp_s_xi2__blk1462_dn7 = assign49750_e64111_d_n7;
        locals.var_sp_s_xi2__blk1462_dn8 = assign49750_e64111_d_n8;
        locals.var_sp_s_xi2__blk1462_dn9 = assign49750_e64111_d_n9;

        let (assign49760_e64164, assign49760_e64164_d_n4, assign49760_e64164_d_n6, assign49760_e64164_d_n7, assign49760_e64164_d_n8, assign49760_e64164_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49760_e64124: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
        let assign49760_e64128: f64 = (locals.var_sp_s_temp1__blk1449 + locals.var_sp_s_eta__blk1453);
        let assign49760_e64130: f64 = (assign49760_e64128 - 1.0);
        let assign49760_e64134: f64 = (locals.var_sp_s_eta__blk1453 + 1.0);
        let assign49760_e64136: f64 = (assign49760_e64134 + locals.var_sp_s_xi0__blk1460);
        let assign49760_e64137: f64 = (locals.var_delta_ns__blk1364 * assign49760_e64136);
        let assign49760_e64138: f64 = (assign49760_e64130 - assign49760_e64137);
        let assign49760_e64139: f64 = (locals.var_gf2__blk1325 * assign49760_e64138);
        let assign49760_e64140: f64 = (assign49760_e64124 - assign49760_e64139);
        let (assign49760_e64162, assign49760_e64162_d_n4, assign49760_e64162_d_n6, assign49760_e64162_d_n7, assign49760_e64162_d_n8, assign49760_e64162_d_n9,) = {
            if (1e-40 > assign49760_e64140) {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign49760_e64145: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
                let assign49760_e64149: f64 = (locals.var_sp_s_temp1__blk1449 + locals.var_sp_s_eta__blk1453);
                let assign49760_e64151: f64 = (assign49760_e64149 - 1.0);
                let assign49760_e64155: f64 = (locals.var_sp_s_eta__blk1453 + 1.0);
                let assign49760_e64157: f64 = (assign49760_e64155 + locals.var_sp_s_xi0__blk1460);
                let assign49760_e64158: f64 = (locals.var_delta_ns__blk1364 * assign49760_e64157);
                let assign49760_e64159: f64 = (assign49760_e64151 - assign49760_e64158);
                let assign49760_e64160: f64 = (locals.var_gf2__blk1325 * assign49760_e64159);
                let assign49760_e64161: f64 = (assign49760_e64145 - assign49760_e64160);
                (assign49760_e64161, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) - ((locals.var_gf2__blk1325_dn4 * assign49760_e64159) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn4 + locals.var_sp_s_eta__blk1453_dn4) - ((locals.var_delta_ns__blk1364_dn4 * assign49760_e64157) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_eta__blk1453_dn4 + locals.var_sp_s_xi0__blk1460_dn4))))))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) - ((locals.var_gf2__blk1325_dn6 * assign49760_e64159) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn6 + locals.var_sp_s_eta__blk1453_dn6) - ((locals.var_delta_ns__blk1364_dn6 * assign49760_e64157) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_eta__blk1453_dn6 + locals.var_sp_s_xi0__blk1460_dn6))))))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) - ((locals.var_gf2__blk1325_dn7 * assign49760_e64159) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn7 + locals.var_sp_s_eta__blk1453_dn7) - ((locals.var_delta_ns__blk1364_dn7 * assign49760_e64157) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_eta__blk1453_dn7 + locals.var_sp_s_xi0__blk1460_dn7))))))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) - ((locals.var_gf2__blk1325_dn8 * assign49760_e64159) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn8 + locals.var_sp_s_eta__blk1453_dn8) - ((locals.var_delta_ns__blk1364_dn8 * assign49760_e64157) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_eta__blk1453_dn8 + locals.var_sp_s_xi0__blk1460_dn8))))))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) - ((locals.var_gf2__blk1325_dn9 * assign49760_e64159) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn9 + locals.var_sp_s_eta__blk1453_dn9) - ((locals.var_delta_ns__blk1364_dn9 * assign49760_e64157) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_eta__blk1453_dn9 + locals.var_sp_s_xi0__blk1460_dn9))))))),)
            }
        };
        (assign49760_e64162, assign49760_e64162_d_n4, assign49760_e64162_d_n6, assign49760_e64162_d_n7, assign49760_e64162_d_n8, assign49760_e64162_d_n9,)
    } else {
        (locals.var_sp_s_a__blk1454, locals.var_sp_s_a__blk1454_dn4, locals.var_sp_s_a__blk1454_dn6, locals.var_sp_s_a__blk1454_dn7, locals.var_sp_s_a__blk1454_dn8, locals.var_sp_s_a__blk1454_dn9,)
    }
};
        locals.var_sp_s_a__blk1454 = assign49760_e64164;
        locals.var_sp_s_a__blk1454_dn4 = assign49760_e64164_d_n4;
        locals.var_sp_s_a__blk1454_dn6 = assign49760_e64164_d_n6;
        locals.var_sp_s_a__blk1454_dn7 = assign49760_e64164_d_n7;
        locals.var_sp_s_a__blk1454_dn8 = assign49760_e64164_d_n8;
        locals.var_sp_s_a__blk1454_dn9 = assign49760_e64164_d_n9;

        let (assign49770_e64186, assign49770_e64186_d_n4, assign49770_e64186_d_n6, assign49770_e64186_d_n7, assign49770_e64186_d_n8, assign49770_e64186_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49770_e64180: f64 = (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462);
        let assign49770_e64181: f64 = (locals.var_sp_s_temp1__blk1449 - assign49770_e64180);
        let assign49770_e64182: f64 = (locals.var_gf2__blk1325 * assign49770_e64181);
        let assign49770_e64183: f64 = (0.5 * assign49770_e64182);
        let assign49770_e64184: f64 = (1.0 - assign49770_e64183);
        (assign49770_e64184, (-(0.5 * ((locals.var_gf2__blk1325_dn4 * assign49770_e64181) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn4 - ((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn4))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn6 * assign49770_e64181) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn6 - ((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn6))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn7 * assign49770_e64181) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn7 - ((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn7))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn8 * assign49770_e64181) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn8 - ((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn8))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn9 * assign49770_e64181) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn9 - ((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn9))))))),)
    } else {
        (locals.var_sp_s_b__blk1471, locals.var_sp_s_b__blk1471_dn4, locals.var_sp_s_b__blk1471_dn6, locals.var_sp_s_b__blk1471_dn7, locals.var_sp_s_b__blk1471_dn8, locals.var_sp_s_b__blk1471_dn9,)
    }
};
        locals.var_sp_s_b__blk1471 = assign49770_e64186;
        locals.var_sp_s_b__blk1471_dn4 = assign49770_e64186_d_n4;
        locals.var_sp_s_b__blk1471_dn6 = assign49770_e64186_d_n6;
        locals.var_sp_s_b__blk1471_dn7 = assign49770_e64186_d_n7;
        locals.var_sp_s_b__blk1471_dn8 = assign49770_e64186_d_n8;
        locals.var_sp_s_b__blk1471_dn9 = assign49770_e64186_d_n9;

        let (assign49780_e64212, assign49780_e64212_d_n4, assign49780_e64212_d_n6, assign49780_e64212_d_n7, assign49780_e64212_d_n8, assign49780_e64212_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49780_e64198: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
        let assign49780_e64202: f64 = (1.0 - locals.var_sp_s_temp1__blk1449);
        let assign49780_e64206: f64 = (1.0 + locals.var_sp_s_xi1__blk1461);
        let assign49780_e64207: f64 = (locals.var_delta_ns__blk1364 * assign49780_e64206);
        let assign49780_e64208: f64 = (assign49780_e64202 - assign49780_e64207);
        let assign49780_e64209: f64 = (locals.var_gf2__blk1325 * assign49780_e64208);
        let assign49780_e64210: f64 = (assign49780_e64198 + assign49780_e64209);
        (assign49780_e64210, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) + ((locals.var_gf2__blk1325_dn4 * assign49780_e64208) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn4) - ((locals.var_delta_ns__blk1364_dn4 * assign49780_e64206) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn4)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) + ((locals.var_gf2__blk1325_dn6 * assign49780_e64208) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn6) - ((locals.var_delta_ns__blk1364_dn6 * assign49780_e64206) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) + ((locals.var_gf2__blk1325_dn7 * assign49780_e64208) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn7) - ((locals.var_delta_ns__blk1364_dn7 * assign49780_e64206) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) + ((locals.var_gf2__blk1325_dn8 * assign49780_e64208) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn8) - ((locals.var_delta_ns__blk1364_dn8 * assign49780_e64206) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn8)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) + ((locals.var_gf2__blk1325_dn9 * assign49780_e64208) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn9) - ((locals.var_delta_ns__blk1364_dn9 * assign49780_e64206) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn9)))))),)
    } else {
        (locals.var_sp_s_c__blk1455, locals.var_sp_s_c__blk1455_dn4, locals.var_sp_s_c__blk1455_dn6, locals.var_sp_s_c__blk1455_dn7, locals.var_sp_s_c__blk1455_dn8, locals.var_sp_s_c__blk1455_dn9,)
    }
};
        locals.var_sp_s_c__blk1455 = assign49780_e64212;
        locals.var_sp_s_c__blk1455_dn4 = assign49780_e64212_d_n4;
        locals.var_sp_s_c__blk1455_dn6 = assign49780_e64212_d_n6;
        locals.var_sp_s_c__blk1455_dn7 = assign49780_e64212_d_n7;
        locals.var_sp_s_c__blk1455_dn8 = assign49780_e64212_d_n8;
        locals.var_sp_s_c__blk1455_dn9 = assign49780_e64212_d_n9;

        let (assign49790_e64231, assign49790_e64231_d_n4, assign49790_e64231_d_n6, assign49790_e64231_d_n7, assign49790_e64231_d_n8, assign49790_e64231_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49790_e64224: f64 = (locals.var_xn_s__blk1349 - locals.var_sp_s_eta__blk1453);
        let assign49790_e64227: f64 = (locals.var_sp_s_a__blk1454 / locals.var_gf2__blk1325);
        let assign49790_e64228: f64 = (assign49790_e64227).ln();
        let assign49790_e64229: f64 = (assign49790_e64224 + assign49790_e64228);
        (assign49790_e64229, ((locals.var_xn_s__blk1349_dn4 - locals.var_sp_s_eta__blk1453_dn4) + ((((locals.var_sp_s_a__blk1454_dn4 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn4)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign49790_e64227)), ((locals.var_xn_s__blk1349_dn6 - locals.var_sp_s_eta__blk1453_dn6) + ((((locals.var_sp_s_a__blk1454_dn6 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn6)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign49790_e64227)), ((locals.var_xn_s__blk1349_dn7 - locals.var_sp_s_eta__blk1453_dn7) + ((((locals.var_sp_s_a__blk1454_dn7 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn7)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign49790_e64227)), ((locals.var_xn_s__blk1349_dn8 - locals.var_sp_s_eta__blk1453_dn8) + ((((locals.var_sp_s_a__blk1454_dn8 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn8)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign49790_e64227)), ((locals.var_xn_s__blk1349_dn9 - locals.var_sp_s_eta__blk1453_dn9) + ((((locals.var_sp_s_a__blk1454_dn9 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn9)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign49790_e64227)),)
    } else {
        (locals.var_sp_s_tau__blk1456, locals.var_sp_s_tau__blk1456_dn4, locals.var_sp_s_tau__blk1456_dn6, locals.var_sp_s_tau__blk1456_dn7, locals.var_sp_s_tau__blk1456_dn8, locals.var_sp_s_tau__blk1456_dn9,)
    }
};
        locals.var_sp_s_tau__blk1456 = assign49790_e64231;
        locals.var_sp_s_tau__blk1456_dn4 = assign49790_e64231_d_n4;
        locals.var_sp_s_tau__blk1456_dn6 = assign49790_e64231_d_n6;
        locals.var_sp_s_tau__blk1456_dn7 = assign49790_e64231_d_n7;
        locals.var_sp_s_tau__blk1456_dn8 = assign49790_e64231_d_n8;
        locals.var_sp_s_tau__blk1456_dn9 = assign49790_e64231_d_n9;

        let (assign49800_e64245, assign49800_e64245_d_n4, assign49800_e64245_d_n6, assign49800_e64245_d_n7, assign49800_e64245_d_n8, assign49800_e64245_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49800_e64243: f64 = (locals.var_sp_s_a__blk1454 + locals.var_sp_s_c__blk1455);
        (assign49800_e64243, (locals.var_sp_s_a__blk1454_dn4 + locals.var_sp_s_c__blk1455_dn4), (locals.var_sp_s_a__blk1454_dn6 + locals.var_sp_s_c__blk1455_dn6), (locals.var_sp_s_a__blk1454_dn7 + locals.var_sp_s_c__blk1455_dn7), (locals.var_sp_s_a__blk1454_dn8 + locals.var_sp_s_c__blk1455_dn8), (locals.var_sp_s_a__blk1454_dn9 + locals.var_sp_s_c__blk1455_dn9),)
    } else {
        (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9,)
    }
};
        locals.var_nu = assign49800_e64245;
        locals.var_nu_dn4 = assign49800_e64245_d_n4;
        locals.var_nu_dn6 = assign49800_e64245_d_n6;
        locals.var_nu_dn7 = assign49800_e64245_d_n7;
        locals.var_nu_dn8 = assign49800_e64245_d_n8;
        locals.var_nu_dn9 = assign49800_e64245_d_n9;

        let (assign49810_e64271, assign49810_e64271_d_n4, assign49810_e64271_d_n6, assign49810_e64271_d_n7, assign49810_e64271_d_n8, assign49810_e64271_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49810_e64257: f64 = (locals.var_nu * locals.var_nu);
        let assign49810_e64262: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
        let assign49810_e64263: f64 = (0.5 * assign49810_e64262);
        let assign49810_e64266: f64 = (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471);
        let assign49810_e64267: f64 = (assign49810_e64263 - assign49810_e64266);
        let assign49810_e64268: f64 = (locals.var_sp_s_tau__blk1456 * assign49810_e64267);
        let assign49810_e64269: f64 = (assign49810_e64257 + assign49810_e64268);
        (assign49810_e64269, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau__blk1456_dn4 * assign49810_e64267) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4))) - ((locals.var_sp_s_a__blk1454_dn4 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn4)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1456_dn6 * assign49810_e64267) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6))) - ((locals.var_sp_s_a__blk1454_dn6 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1456_dn7 * assign49810_e64267) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7))) - ((locals.var_sp_s_a__blk1454_dn7 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1456_dn8 * assign49810_e64267) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8))) - ((locals.var_sp_s_a__blk1454_dn8 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn8)))))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau__blk1456_dn9 * assign49810_e64267) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9))) - ((locals.var_sp_s_a__blk1454_dn9 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn9)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9,)
    }
};
        locals.var_mutau = assign49810_e64271;
        locals.var_mutau_dn4 = assign49810_e64271_d_n4;
        locals.var_mutau_dn6 = assign49810_e64271_d_n6;
        locals.var_mutau_dn7 = assign49810_e64271_d_n7;
        locals.var_mutau_dn8 = assign49810_e64271_d_n8;
        locals.var_mutau_dn9 = assign49810_e64271_d_n9;

        let (assign49820_e64311, assign49820_e64311_d_n4, assign49820_e64311_d_n6, assign49820_e64311_d_n7, assign49820_e64311_d_n8, assign49820_e64311_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49820_e64284: f64 = (locals.var_sp_s_a__blk1454 * locals.var_nu);
        let assign49820_e64286: f64 = (assign49820_e64284 * locals.var_sp_s_tau__blk1456);
        let assign49820_e64290: f64 = (locals.var_nu / locals.var_mutau);
        let assign49820_e64292: f64 = (assign49820_e64290 * locals.var_sp_s_tau__blk1456);
        let assign49820_e64294: f64 = (assign49820_e64292 * locals.var_sp_s_tau__blk1456);
        let assign49820_e64296: f64 = (assign49820_e64294 * locals.var_sp_s_c__blk1455);
        let assign49820_e64299: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
        let assign49820_e64301: f64 = (assign49820_e64299 * 0.3333333333333333);
        let assign49820_e64304: f64 = (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471);
        let assign49820_e64305: f64 = (assign49820_e64301 - assign49820_e64304);
        let assign49820_e64306: f64 = (assign49820_e64296 * assign49820_e64305);
        let assign49820_e64307: f64 = (locals.var_mutau + assign49820_e64306);
        let assign49820_e64308: f64 = (assign49820_e64286 / assign49820_e64307);
        let assign49820_e64309: f64 = (locals.var_sp_s_eta__blk1453 + assign49820_e64308);
        (assign49820_e64309, (locals.var_sp_s_eta__blk1453_dn4 + (((((((locals.var_sp_s_a__blk1454_dn4 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn4)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64284 * locals.var_sp_s_tau__blk1456_dn4)) * assign49820_e64307) - (assign49820_e64286 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64290 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64292 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_c__blk1455) + (assign49820_e64294 * locals.var_sp_s_c__blk1455_dn4)) * assign49820_e64305) + (assign49820_e64296 * ((((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn4 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn4)))))))) / (assign49820_e64307 * assign49820_e64307))), (locals.var_sp_s_eta__blk1453_dn6 + (((((((locals.var_sp_s_a__blk1454_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64284 * locals.var_sp_s_tau__blk1456_dn6)) * assign49820_e64307) - (assign49820_e64286 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64290 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64292 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_c__blk1455) + (assign49820_e64294 * locals.var_sp_s_c__blk1455_dn6)) * assign49820_e64305) + (assign49820_e64296 * ((((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn6 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn6)))))))) / (assign49820_e64307 * assign49820_e64307))), (locals.var_sp_s_eta__blk1453_dn7 + (((((((locals.var_sp_s_a__blk1454_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64284 * locals.var_sp_s_tau__blk1456_dn7)) * assign49820_e64307) - (assign49820_e64286 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64290 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64292 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_c__blk1455) + (assign49820_e64294 * locals.var_sp_s_c__blk1455_dn7)) * assign49820_e64305) + (assign49820_e64296 * ((((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn7 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn7)))))))) / (assign49820_e64307 * assign49820_e64307))), (locals.var_sp_s_eta__blk1453_dn8 + (((((((locals.var_sp_s_a__blk1454_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64284 * locals.var_sp_s_tau__blk1456_dn8)) * assign49820_e64307) - (assign49820_e64286 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64290 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64292 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_c__blk1455) + (assign49820_e64294 * locals.var_sp_s_c__blk1455_dn8)) * assign49820_e64305) + (assign49820_e64296 * ((((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn8 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn8)))))))) / (assign49820_e64307 * assign49820_e64307))), (locals.var_sp_s_eta__blk1453_dn9 + (((((((locals.var_sp_s_a__blk1454_dn9 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn9)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64284 * locals.var_sp_s_tau__blk1456_dn9)) * assign49820_e64307) - (assign49820_e64286 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64290 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64292 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_c__blk1455) + (assign49820_e64294 * locals.var_sp_s_c__blk1455_dn9)) * assign49820_e64305) + (assign49820_e64296 * ((((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn9 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn9)))))))) / (assign49820_e64307 * assign49820_e64307))),)
    } else {
        (locals.var_sp_s_x0__blk1472, locals.var_sp_s_x0__blk1472_dn4, locals.var_sp_s_x0__blk1472_dn6, locals.var_sp_s_x0__blk1472_dn7, locals.var_sp_s_x0__blk1472_dn8, locals.var_sp_s_x0__blk1472_dn9,)
    }
};
        locals.var_sp_s_x0__blk1472 = assign49820_e64311;
        locals.var_sp_s_x0__blk1472_dn4 = assign49820_e64311_d_n4;
        locals.var_sp_s_x0__blk1472_dn6 = assign49820_e64311_d_n6;
        locals.var_sp_s_x0__blk1472_dn7 = assign49820_e64311_d_n7;
        locals.var_sp_s_x0__blk1472_dn8 = assign49820_e64311_d_n8;
        locals.var_sp_s_x0__blk1472_dn9 = assign49820_e64311_d_n9;

        let assign49830_e64314: f64 = if locals.var_sp_s_x0__blk1472 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1489 = assign49830_e64314;

        let (assign49840_e64329, assign49840_e64329_d_n4, assign49840_e64329_d_n6, assign49840_e64329_d_n7, assign49840_e64329_d_n8, assign49840_e64329_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 != 0.0)) {
        let assign49840_e64327: f64 = (locals.var_sp_s_x0__blk1472).exp();
        (assign49840_e64327, (assign49840_e64327 * locals.var_sp_s_x0__blk1472_dn4), (assign49840_e64327 * locals.var_sp_s_x0__blk1472_dn6), (assign49840_e64327 * locals.var_sp_s_x0__blk1472_dn7), (assign49840_e64327 * locals.var_sp_s_x0__blk1472_dn8), (assign49840_e64327 * locals.var_sp_s_x0__blk1472_dn9),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign49840_e64329;
        locals.var_sp_s_delta0__blk1458_dn4 = assign49840_e64329_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign49840_e64329_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign49840_e64329_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign49840_e64329_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign49840_e64329_d_n9;

    }

    pub(super) fn stamp_transient_block_40(
        locals: &mut StampLocals,
    ) {
        let (assign49850_e64345, assign49850_e64345_d_n4, assign49850_e64345_d_n6, assign49850_e64345_d_n7, assign49850_e64345_d_n8, assign49850_e64345_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 != 0.0)) {
        let assign49850_e64343: f64 = (1.0 / locals.var_sp_s_delta0__blk1458);
        (assign49850_e64343, (-(locals.var_sp_s_delta0__blk1458_dn4 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn6 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn7 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn8 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn9 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))),)
    } else {
        (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9,)
    }
};
        locals.var_sp_s_delta1__blk1459 = assign49850_e64345;
        locals.var_sp_s_delta1__blk1459_dn4 = assign49850_e64345_d_n4;
        locals.var_sp_s_delta1__blk1459_dn6 = assign49850_e64345_d_n6;
        locals.var_sp_s_delta1__blk1459_dn7 = assign49850_e64345_d_n7;
        locals.var_sp_s_delta1__blk1459_dn8 = assign49850_e64345_d_n8;
        locals.var_sp_s_delta1__blk1459_dn9 = assign49850_e64345_d_n9;

        let (assign49860_e64361, assign49860_e64361_d_n4, assign49860_e64361_d_n6, assign49860_e64361_d_n7, assign49860_e64361_d_n8, assign49860_e64361_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 != 0.0)) {
        let assign49860_e64359: f64 = (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458);
        (assign49860_e64359, ((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn4)), ((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn6)), ((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn7)), ((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn8)), ((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn9)),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign49860_e64361;
        locals.var_sp_s_delta0__blk1458_dn4 = assign49860_e64361_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign49860_e64361_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign49860_e64361_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign49860_e64361_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign49860_e64361_d_n9;

        let assign49870_e64365: f64 = (locals.var_xn_s__blk1349 - 230.25850929940458);
        let assign49870_e64366: f64 = if locals.var_sp_s_x0__blk1472 > assign49870_e64365 { 1.0 } else { 0.0 };
        locals.var_guard1490 = assign49870_e64366;

        let (assign49880_e64386, assign49880_e64386_d_n4, assign49880_e64386_d_n6, assign49880_e64386_d_n7, assign49880_e64386_d_n8, assign49880_e64386_d_n9,) = {
    if ((((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 == 0.0)) && (locals.var_guard1490 != 0.0)) {
        let assign49880_e64383: f64 = (locals.var_sp_s_x0__blk1472 - locals.var_xn_s__blk1349);
        let assign49880_e64384: f64 = (assign49880_e64383).exp();
        (assign49880_e64384, (assign49880_e64384 * (locals.var_sp_s_x0__blk1472_dn4 - locals.var_xn_s__blk1349_dn4)), (assign49880_e64384 * (locals.var_sp_s_x0__blk1472_dn6 - locals.var_xn_s__blk1349_dn6)), (assign49880_e64384 * (locals.var_sp_s_x0__blk1472_dn7 - locals.var_xn_s__blk1349_dn7)), (assign49880_e64384 * (locals.var_sp_s_x0__blk1472_dn8 - locals.var_xn_s__blk1349_dn8)), (assign49880_e64384 * (locals.var_sp_s_x0__blk1472_dn9 - locals.var_xn_s__blk1349_dn9)),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign49880_e64386;
        locals.var_sp_s_delta0__blk1458_dn4 = assign49880_e64386_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign49880_e64386_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign49880_e64386_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign49880_e64386_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign49880_e64386_d_n9;

        let (assign49890_e64405, assign49890_e64405_d_n4, assign49890_e64405_d_n6, assign49890_e64405_d_n7, assign49890_e64405_d_n8, assign49890_e64405_d_n9,) = {
    if ((((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 == 0.0)) && (locals.var_guard1490 != 0.0)) {
        let assign49890_e64403: f64 = (locals.var_delta_ns__blk1364 / locals.var_sp_s_delta0__blk1458);
        (assign49890_e64403, (((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn4)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn6)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn7)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn8)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn9)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)),)
    } else {
        (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9,)
    }
};
        locals.var_sp_s_delta1__blk1459 = assign49890_e64405;
        locals.var_sp_s_delta1__blk1459_dn4 = assign49890_e64405_d_n4;
        locals.var_sp_s_delta1__blk1459_dn6 = assign49890_e64405_d_n6;
        locals.var_sp_s_delta1__blk1459_dn7 = assign49890_e64405_d_n7;
        locals.var_sp_s_delta1__blk1459_dn8 = assign49890_e64405_d_n8;
        locals.var_sp_s_delta1__blk1459_dn9 = assign49890_e64405_d_n9;

        let (assign49900_e64451, assign49900_e64451_d_n4, assign49900_e64451_d_n6, assign49900_e64451_d_n7, assign49900_e64451_d_n8, assign49900_e64451_d_n9,) = {
    if ((((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 == 0.0)) && (locals.var_guard1490 == 0.0)) {
        let assign49900_e64425: f64 = (locals.var_xn_s__blk1349 - locals.var_sp_s_x0__blk1472);
        let assign49900_e64427: f64 = (assign49900_e64425 - 230.25850929940458);
        let assign49900_e64432: f64 = (locals.var_xn_s__blk1349 - locals.var_sp_s_x0__blk1472);
        let assign49900_e64434: f64 = (assign49900_e64432 - 230.25850929940458);
        let assign49900_e64438: f64 = (locals.var_xn_s__blk1349 - locals.var_sp_s_x0__blk1472);
        let assign49900_e64440: f64 = (assign49900_e64438 - 230.25850929940458);
        let assign49900_e64442: f64 = (assign49900_e64440 * 0.3333333333333333);
        let assign49900_e64443: f64 = (1.0 + assign49900_e64442);
        let assign49900_e64444: f64 = (assign49900_e64434 * assign49900_e64443);
        let assign49900_e64445: f64 = (0.5 * assign49900_e64444);
        let assign49900_e64446: f64 = (1.0 + assign49900_e64445);
        let assign49900_e64447: f64 = (assign49900_e64427 * assign49900_e64446);
        let assign49900_e64448: f64 = (1.0 + assign49900_e64447);
        let assign49900_e64449: f64 = (1e-100 / assign49900_e64448);
        (assign49900_e64449, (-((1e-100 * (((locals.var_xn_s__blk1349_dn4 - locals.var_sp_s_x0__blk1472_dn4) * assign49900_e64446) + (assign49900_e64427 * (0.5 * (((locals.var_xn_s__blk1349_dn4 - locals.var_sp_s_x0__blk1472_dn4) * assign49900_e64443) + (assign49900_e64434 * ((locals.var_xn_s__blk1349_dn4 - locals.var_sp_s_x0__blk1472_dn4) * 0.3333333333333333))))))) / (assign49900_e64448 * assign49900_e64448))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn6 - locals.var_sp_s_x0__blk1472_dn6) * assign49900_e64446) + (assign49900_e64427 * (0.5 * (((locals.var_xn_s__blk1349_dn6 - locals.var_sp_s_x0__blk1472_dn6) * assign49900_e64443) + (assign49900_e64434 * ((locals.var_xn_s__blk1349_dn6 - locals.var_sp_s_x0__blk1472_dn6) * 0.3333333333333333))))))) / (assign49900_e64448 * assign49900_e64448))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn7 - locals.var_sp_s_x0__blk1472_dn7) * assign49900_e64446) + (assign49900_e64427 * (0.5 * (((locals.var_xn_s__blk1349_dn7 - locals.var_sp_s_x0__blk1472_dn7) * assign49900_e64443) + (assign49900_e64434 * ((locals.var_xn_s__blk1349_dn7 - locals.var_sp_s_x0__blk1472_dn7) * 0.3333333333333333))))))) / (assign49900_e64448 * assign49900_e64448))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn8 - locals.var_sp_s_x0__blk1472_dn8) * assign49900_e64446) + (assign49900_e64427 * (0.5 * (((locals.var_xn_s__blk1349_dn8 - locals.var_sp_s_x0__blk1472_dn8) * assign49900_e64443) + (assign49900_e64434 * ((locals.var_xn_s__blk1349_dn8 - locals.var_sp_s_x0__blk1472_dn8) * 0.3333333333333333))))))) / (assign49900_e64448 * assign49900_e64448))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn9 - locals.var_sp_s_x0__blk1472_dn9) * assign49900_e64446) + (assign49900_e64427 * (0.5 * (((locals.var_xn_s__blk1349_dn9 - locals.var_sp_s_x0__blk1472_dn9) * assign49900_e64443) + (assign49900_e64434 * ((locals.var_xn_s__blk1349_dn9 - locals.var_sp_s_x0__blk1472_dn9) * 0.3333333333333333))))))) / (assign49900_e64448 * assign49900_e64448))),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign49900_e64451;
        locals.var_sp_s_delta0__blk1458_dn4 = assign49900_e64451_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign49900_e64451_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign49900_e64451_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign49900_e64451_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign49900_e64451_d_n9;

        let (assign49910_e64491, assign49910_e64491_d_n4, assign49910_e64491_d_n6, assign49910_e64491_d_n7, assign49910_e64491_d_n8, assign49910_e64491_d_n9,) = {
    if ((((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 == 0.0)) && (locals.var_guard1490 == 0.0)) {
        let assign49910_e64471: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
        let assign49910_e64476: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
        let assign49910_e64480: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
        let assign49910_e64482: f64 = (assign49910_e64480 * 0.3333333333333333);
        let assign49910_e64483: f64 = (1.0 + assign49910_e64482);
        let assign49910_e64484: f64 = (assign49910_e64476 * assign49910_e64483);
        let assign49910_e64485: f64 = (0.5 * assign49910_e64484);
        let assign49910_e64486: f64 = (1.0 + assign49910_e64485);
        let assign49910_e64487: f64 = (assign49910_e64471 * assign49910_e64486);
        let assign49910_e64488: f64 = (1.0 + assign49910_e64487);
        let assign49910_e64489: f64 = (1e-100 / assign49910_e64488);
        (assign49910_e64489, (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn4 * assign49910_e64486) + (assign49910_e64471 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn4 * assign49910_e64483) + (assign49910_e64476 * (locals.var_sp_s_x0__blk1472_dn4 * 0.3333333333333333))))))) / (assign49910_e64488 * assign49910_e64488))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn6 * assign49910_e64486) + (assign49910_e64471 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn6 * assign49910_e64483) + (assign49910_e64476 * (locals.var_sp_s_x0__blk1472_dn6 * 0.3333333333333333))))))) / (assign49910_e64488 * assign49910_e64488))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn7 * assign49910_e64486) + (assign49910_e64471 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn7 * assign49910_e64483) + (assign49910_e64476 * (locals.var_sp_s_x0__blk1472_dn7 * 0.3333333333333333))))))) / (assign49910_e64488 * assign49910_e64488))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn8 * assign49910_e64486) + (assign49910_e64471 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn8 * assign49910_e64483) + (assign49910_e64476 * (locals.var_sp_s_x0__blk1472_dn8 * 0.3333333333333333))))))) / (assign49910_e64488 * assign49910_e64488))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn9 * assign49910_e64486) + (assign49910_e64471 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn9 * assign49910_e64483) + (assign49910_e64476 * (locals.var_sp_s_x0__blk1472_dn9 * 0.3333333333333333))))))) / (assign49910_e64488 * assign49910_e64488))),)
    } else {
        (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9,)
    }
};
        locals.var_sp_s_delta1__blk1459 = assign49910_e64491;
        locals.var_sp_s_delta1__blk1459_dn4 = assign49910_e64491_d_n4;
        locals.var_sp_s_delta1__blk1459_dn6 = assign49910_e64491_d_n6;
        locals.var_sp_s_delta1__blk1459_dn7 = assign49910_e64491_d_n7;
        locals.var_sp_s_delta1__blk1459_dn8 = assign49910_e64491_d_n8;
        locals.var_sp_s_delta1__blk1459_dn9 = assign49910_e64491_d_n9;

        let (assign49920_e64509, assign49920_e64509_d_n4, assign49920_e64509_d_n6, assign49920_e64509_d_n7, assign49920_e64509_d_n8, assign49920_e64509_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49920_e64505: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472);
        let assign49920_e64506: f64 = (2.0 + assign49920_e64505);
        let assign49920_e64507: f64 = (1.0 / assign49920_e64506);
        (assign49920_e64507, (-(((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn4)) / (assign49920_e64506 * assign49920_e64506))), (-(((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn6)) / (assign49920_e64506 * assign49920_e64506))), (-(((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn7)) / (assign49920_e64506 * assign49920_e64506))), (-(((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn8)) / (assign49920_e64506 * assign49920_e64506))), (-(((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn9)) / (assign49920_e64506 * assign49920_e64506))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49920_e64509;
        locals.var_sp_s_temp__blk1448_dn4 = assign49920_e64509_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49920_e64509_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49920_e64509_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49920_e64509_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49920_e64509_d_n9;

        let (assign49930_e64525, assign49930_e64525_d_n4, assign49930_e64525_d_n6, assign49930_e64525_d_n7, assign49930_e64525_d_n8, assign49930_e64525_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49930_e64521: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472);
        let assign49930_e64523: f64 = (assign49930_e64521 * locals.var_sp_s_temp__blk1448);
        (assign49930_e64523, ((((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49930_e64521 * locals.var_sp_s_temp__blk1448_dn4)), ((((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49930_e64521 * locals.var_sp_s_temp__blk1448_dn6)), ((((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49930_e64521 * locals.var_sp_s_temp__blk1448_dn7)), ((((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49930_e64521 * locals.var_sp_s_temp__blk1448_dn8)), ((((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49930_e64521 * locals.var_sp_s_temp__blk1448_dn9)),)
    } else {
        (locals.var_sp_s_xi0__blk1460, locals.var_sp_s_xi0__blk1460_dn4, locals.var_sp_s_xi0__blk1460_dn6, locals.var_sp_s_xi0__blk1460_dn7, locals.var_sp_s_xi0__blk1460_dn8, locals.var_sp_s_xi0__blk1460_dn9,)
    }
};
        locals.var_sp_s_xi0__blk1460 = assign49930_e64525;
        locals.var_sp_s_xi0__blk1460_dn4 = assign49930_e64525_d_n4;
        locals.var_sp_s_xi0__blk1460_dn6 = assign49930_e64525_d_n6;
        locals.var_sp_s_xi0__blk1460_dn7 = assign49930_e64525_d_n7;
        locals.var_sp_s_xi0__blk1460_dn8 = assign49930_e64525_d_n8;
        locals.var_sp_s_xi0__blk1460_dn9 = assign49930_e64525_d_n9;

        let (assign49940_e64543, assign49940_e64543_d_n4, assign49940_e64543_d_n6, assign49940_e64543_d_n7, assign49940_e64543_d_n8, assign49940_e64543_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49940_e64538: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448);
        let assign49940_e64540: f64 = (assign49940_e64538 * locals.var_sp_s_temp__blk1448);
        let assign49940_e64541: f64 = (4.0 * assign49940_e64540);
        (assign49940_e64541, (4.0 * ((((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49940_e64538 * locals.var_sp_s_temp__blk1448_dn4))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49940_e64538 * locals.var_sp_s_temp__blk1448_dn6))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49940_e64538 * locals.var_sp_s_temp__blk1448_dn7))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49940_e64538 * locals.var_sp_s_temp__blk1448_dn8))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49940_e64538 * locals.var_sp_s_temp__blk1448_dn9))),)
    } else {
        (locals.var_sp_s_xi1__blk1461, locals.var_sp_s_xi1__blk1461_dn4, locals.var_sp_s_xi1__blk1461_dn6, locals.var_sp_s_xi1__blk1461_dn7, locals.var_sp_s_xi1__blk1461_dn8, locals.var_sp_s_xi1__blk1461_dn9,)
    }
};
        locals.var_sp_s_xi1__blk1461 = assign49940_e64543;
        locals.var_sp_s_xi1__blk1461_dn4 = assign49940_e64543_d_n4;
        locals.var_sp_s_xi1__blk1461_dn6 = assign49940_e64543_d_n6;
        locals.var_sp_s_xi1__blk1461_dn7 = assign49940_e64543_d_n7;
        locals.var_sp_s_xi1__blk1461_dn8 = assign49940_e64543_d_n8;
        locals.var_sp_s_xi1__blk1461_dn9 = assign49940_e64543_d_n9;

        let (assign49950_e64565, assign49950_e64565_d_n4, assign49950_e64565_d_n6, assign49950_e64565_d_n7, assign49950_e64565_d_n8, assign49950_e64565_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49950_e64555: f64 = (8.0 * locals.var_sp_s_temp__blk1448);
        let assign49950_e64558: f64 = (12.0 * locals.var_sp_s_xi0__blk1460);
        let assign49950_e64559: f64 = (assign49950_e64555 - assign49950_e64558);
        let assign49950_e64561: f64 = (assign49950_e64559 * locals.var_sp_s_temp__blk1448);
        let assign49950_e64563: f64 = (assign49950_e64561 * locals.var_sp_s_temp__blk1448);
        (assign49950_e64563, ((((((8.0 * locals.var_sp_s_temp__blk1448_dn4) - (12.0 * locals.var_sp_s_xi0__blk1460_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64559 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64561 * locals.var_sp_s_temp__blk1448_dn4)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn6) - (12.0 * locals.var_sp_s_xi0__blk1460_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64559 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64561 * locals.var_sp_s_temp__blk1448_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn7) - (12.0 * locals.var_sp_s_xi0__blk1460_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64559 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64561 * locals.var_sp_s_temp__blk1448_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn8) - (12.0 * locals.var_sp_s_xi0__blk1460_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64559 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64561 * locals.var_sp_s_temp__blk1448_dn8)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn9) - (12.0 * locals.var_sp_s_xi0__blk1460_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64559 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64561 * locals.var_sp_s_temp__blk1448_dn9)),)
    } else {
        (locals.var_sp_s_xi2__blk1462, locals.var_sp_s_xi2__blk1462_dn4, locals.var_sp_s_xi2__blk1462_dn6, locals.var_sp_s_xi2__blk1462_dn7, locals.var_sp_s_xi2__blk1462_dn8, locals.var_sp_s_xi2__blk1462_dn9,)
    }
};
        locals.var_sp_s_xi2__blk1462 = assign49950_e64565;
        locals.var_sp_s_xi2__blk1462_dn4 = assign49950_e64565_d_n4;
        locals.var_sp_s_xi2__blk1462_dn6 = assign49950_e64565_d_n6;
        locals.var_sp_s_xi2__blk1462_dn7 = assign49950_e64565_d_n7;
        locals.var_sp_s_xi2__blk1462_dn8 = assign49950_e64565_d_n8;
        locals.var_sp_s_xi2__blk1462_dn9 = assign49950_e64565_d_n9;

        let (assign49960_e64579, assign49960_e64579_d_n4, assign49960_e64579_d_n6, assign49960_e64579_d_n7, assign49960_e64579_d_n8, assign49960_e64579_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49960_e64577: f64 = (locals.var_xg__blk1343 - locals.var_sp_s_x0__blk1472);
        (assign49960_e64577, (locals.var_xg__blk1343_dn4 - locals.var_sp_s_x0__blk1472_dn4), (locals.var_xg__blk1343_dn6 - locals.var_sp_s_x0__blk1472_dn6), (locals.var_xg__blk1343_dn7 - locals.var_sp_s_x0__blk1472_dn7), (locals.var_xg__blk1343_dn8 - locals.var_sp_s_x0__blk1472_dn8), (locals.var_xg__blk1343_dn9 - locals.var_sp_s_x0__blk1472_dn9),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49960_e64579;
        locals.var_sp_s_temp__blk1448_dn4 = assign49960_e64579_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49960_e64579_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49960_e64579_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49960_e64579_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49960_e64579_d_n9;

        let (assign49970_e64607, assign49970_e64607_d_n4, assign49970_e64607_d_n6, assign49970_e64607_d_n7, assign49970_e64607_d_n8, assign49970_e64607_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49970_e64591: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
        let assign49970_e64595: f64 = (1.0 - locals.var_sp_s_delta1__blk1459);
        let assign49970_e64597: f64 = (assign49970_e64595 + locals.var_sp_s_delta0__blk1458);
        let assign49970_e64601: f64 = (1.0 + locals.var_sp_s_xi1__blk1461);
        let assign49970_e64602: f64 = (locals.var_delta_ns__blk1364 * assign49970_e64601);
        let assign49970_e64603: f64 = (assign49970_e64597 - assign49970_e64602);
        let assign49970_e64604: f64 = (locals.var_gf2__blk1325 * assign49970_e64603);
        let assign49970_e64605: f64 = (assign49970_e64591 + assign49970_e64604);
        (assign49970_e64605, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) + ((locals.var_gf2__blk1325_dn4 * assign49970_e64603) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn4) + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_ns__blk1364_dn4 * assign49970_e64601) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn4)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) + ((locals.var_gf2__blk1325_dn6 * assign49970_e64603) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn6) + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_ns__blk1364_dn6 * assign49970_e64601) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) + ((locals.var_gf2__blk1325_dn7 * assign49970_e64603) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn7) + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_ns__blk1364_dn7 * assign49970_e64601) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) + ((locals.var_gf2__blk1325_dn8 * assign49970_e64603) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn8) + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_ns__blk1364_dn8 * assign49970_e64601) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn8)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) + ((locals.var_gf2__blk1325_dn9 * assign49970_e64603) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn9) + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_ns__blk1364_dn9 * assign49970_e64601) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn9)))))),)
    } else {
        (locals.var_sp_s_pc__blk1463, locals.var_sp_s_pc__blk1463_dn4, locals.var_sp_s_pc__blk1463_dn6, locals.var_sp_s_pc__blk1463_dn7, locals.var_sp_s_pc__blk1463_dn8, locals.var_sp_s_pc__blk1463_dn9,)
    }
};
        locals.var_sp_s_pc__blk1463 = assign49970_e64607;
        locals.var_sp_s_pc__blk1463_dn4 = assign49970_e64607_d_n4;
        locals.var_sp_s_pc__blk1463_dn6 = assign49970_e64607_d_n6;
        locals.var_sp_s_pc__blk1463_dn7 = assign49970_e64607_d_n7;
        locals.var_sp_s_pc__blk1463_dn8 = assign49970_e64607_d_n8;
        locals.var_sp_s_pc__blk1463_dn9 = assign49970_e64607_d_n9;

        let (assign49980_e64639, assign49980_e64639_d_n4, assign49980_e64639_d_n6, assign49980_e64639_d_n7, assign49980_e64639_d_n8, assign49980_e64639_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49980_e64619: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
        let assign49980_e64623: f64 = (locals.var_sp_s_delta1__blk1459 + locals.var_sp_s_x0__blk1472);
        let assign49980_e64625: f64 = (assign49980_e64623 - 1.0);
        let assign49980_e64627: f64 = (assign49980_e64625 + locals.var_sp_s_delta0__blk1458);
        let assign49980_e64631: f64 = (locals.var_sp_s_x0__blk1472 + 1.0);
        let assign49980_e64633: f64 = (assign49980_e64631 + locals.var_sp_s_xi0__blk1460);
        let assign49980_e64634: f64 = (locals.var_delta_ns__blk1364 * assign49980_e64633);
        let assign49980_e64635: f64 = (assign49980_e64627 - assign49980_e64634);
        let assign49980_e64636: f64 = (locals.var_gf2__blk1325 * assign49980_e64635);
        let assign49980_e64637: f64 = (assign49980_e64619 - assign49980_e64636);
        (assign49980_e64637, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) - ((locals.var_gf2__blk1325_dn4 * assign49980_e64635) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn4 + locals.var_sp_s_x0__blk1472_dn4) + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_ns__blk1364_dn4 * assign49980_e64633) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_x0__blk1472_dn4 + locals.var_sp_s_xi0__blk1460_dn4))))))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) - ((locals.var_gf2__blk1325_dn6 * assign49980_e64635) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn6 + locals.var_sp_s_x0__blk1472_dn6) + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_ns__blk1364_dn6 * assign49980_e64633) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_x0__blk1472_dn6 + locals.var_sp_s_xi0__blk1460_dn6))))))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) - ((locals.var_gf2__blk1325_dn7 * assign49980_e64635) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn7 + locals.var_sp_s_x0__blk1472_dn7) + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_ns__blk1364_dn7 * assign49980_e64633) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_x0__blk1472_dn7 + locals.var_sp_s_xi0__blk1460_dn7))))))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) - ((locals.var_gf2__blk1325_dn8 * assign49980_e64635) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn8 + locals.var_sp_s_x0__blk1472_dn8) + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_ns__blk1364_dn8 * assign49980_e64633) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_x0__blk1472_dn8 + locals.var_sp_s_xi0__blk1460_dn8))))))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) - ((locals.var_gf2__blk1325_dn9 * assign49980_e64635) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn9 + locals.var_sp_s_x0__blk1472_dn9) + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_ns__blk1364_dn9 * assign49980_e64633) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_x0__blk1472_dn9 + locals.var_sp_s_xi0__blk1460_dn9))))))),)
    } else {
        (locals.var_sp_s_qc__blk1464, locals.var_sp_s_qc__blk1464_dn4, locals.var_sp_s_qc__blk1464_dn6, locals.var_sp_s_qc__blk1464_dn7, locals.var_sp_s_qc__blk1464_dn8, locals.var_sp_s_qc__blk1464_dn9,)
    }
};
        locals.var_sp_s_qc__blk1464 = assign49980_e64639;
        locals.var_sp_s_qc__blk1464_dn4 = assign49980_e64639_d_n4;
        locals.var_sp_s_qc__blk1464_dn6 = assign49980_e64639_d_n6;
        locals.var_sp_s_qc__blk1464_dn7 = assign49980_e64639_d_n7;
        locals.var_sp_s_qc__blk1464_dn8 = assign49980_e64639_d_n8;
        locals.var_sp_s_qc__blk1464_dn9 = assign49980_e64639_d_n9;

        let (assign49990_e64661, assign49990_e64661_d_n4, assign49990_e64661_d_n6, assign49990_e64661_d_n7, assign49990_e64661_d_n8, assign49990_e64661_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign49990_e64653: f64 = (locals.var_sp_s_delta1__blk1459 + locals.var_sp_s_delta0__blk1458);
        let assign49990_e64656: f64 = (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462);
        let assign49990_e64657: f64 = (assign49990_e64653 - assign49990_e64656);
        let assign49990_e64658: f64 = (locals.var_gf2__blk1325 * assign49990_e64657);
        let assign49990_e64659: f64 = (2.0 - assign49990_e64658);
        (assign49990_e64659, (-((locals.var_gf2__blk1325_dn4 * assign49990_e64657) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn4 + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn4)))))), (-((locals.var_gf2__blk1325_dn6 * assign49990_e64657) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn6 + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn6)))))), (-((locals.var_gf2__blk1325_dn7 * assign49990_e64657) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn7 + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn7)))))), (-((locals.var_gf2__blk1325_dn8 * assign49990_e64657) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn8 + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn8)))))), (-((locals.var_gf2__blk1325_dn9 * assign49990_e64657) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn9 + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn9)))))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign49990_e64661;
        locals.var_sp_s_temp__blk1448_dn4 = assign49990_e64661_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign49990_e64661_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign49990_e64661_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign49990_e64661_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign49990_e64661_d_n9;

        let (assign50000_e64681, assign50000_e64681_d_n4, assign50000_e64681_d_n6, assign50000_e64681_d_n7, assign50000_e64681_d_n8, assign50000_e64681_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign50000_e64673: f64 = (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463);
        let assign50000_e64677: f64 = (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448);
        let assign50000_e64678: f64 = (2.0 * assign50000_e64677);
        let assign50000_e64679: f64 = (assign50000_e64673 - assign50000_e64678);
        (assign50000_e64679, (((locals.var_sp_s_pc__blk1463_dn4 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn4)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn4)))), (((locals.var_sp_s_pc__blk1463_dn6 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn6)))), (((locals.var_sp_s_pc__blk1463_dn7 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn7)))), (((locals.var_sp_s_pc__blk1463_dn8 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn8)))), (((locals.var_sp_s_pc__blk1463_dn9 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn9)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn9)))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign50000_e64681;
        locals.var_sp_s_temp__blk1448_dn4 = assign50000_e64681_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign50000_e64681_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign50000_e64681_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign50000_e64681_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign50000_e64681_d_n9;

        let (assign50010_e64702, assign50010_e64702_d_n4, assign50010_e64702_d_n6, assign50010_e64702_d_n7, assign50010_e64702_d_n8, assign50010_e64702_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
        let assign50010_e64696: f64 = (locals.var_sp_s_temp__blk1448).sqrt();
        let assign50010_e64697: f64 = (locals.var_sp_s_pc__blk1463 + assign50010_e64696);
        let assign50010_e64698: f64 = (locals.var_sp_s_qc__blk1464 / assign50010_e64697);
        let assign50010_e64699: f64 = (2.0 * assign50010_e64698);
        let assign50010_e64700: f64 = (locals.var_sp_s_x0__blk1472 + assign50010_e64699);
        (assign50010_e64700, (locals.var_sp_s_x0__blk1472_dn4 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn4 * assign50010_e64697) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn4 + (locals.var_sp_s_temp__blk1448_dn4 / (2.0 * assign50010_e64696))))) / (assign50010_e64697 * assign50010_e64697)))), (locals.var_sp_s_x0__blk1472_dn6 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn6 * assign50010_e64697) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn6 + (locals.var_sp_s_temp__blk1448_dn6 / (2.0 * assign50010_e64696))))) / (assign50010_e64697 * assign50010_e64697)))), (locals.var_sp_s_x0__blk1472_dn7 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn7 * assign50010_e64697) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn7 + (locals.var_sp_s_temp__blk1448_dn7 / (2.0 * assign50010_e64696))))) / (assign50010_e64697 * assign50010_e64697)))), (locals.var_sp_s_x0__blk1472_dn8 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn8 * assign50010_e64697) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn8 + (locals.var_sp_s_temp__blk1448_dn8 / (2.0 * assign50010_e64696))))) / (assign50010_e64697 * assign50010_e64697)))), (locals.var_sp_s_x0__blk1472_dn9 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn9 * assign50010_e64697) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn9 + (locals.var_sp_s_temp__blk1448_dn9 / (2.0 * assign50010_e64696))))) / (assign50010_e64697 * assign50010_e64697)))),)
    } else {
        (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9,)
    }
};
        locals.var_x_s__blk1363 = assign50010_e64702;
        locals.var_x_s__blk1363_dn4 = assign50010_e64702_d_n4;
        locals.var_x_s__blk1363_dn6 = assign50010_e64702_d_n6;
        locals.var_x_s__blk1363_dn7 = assign50010_e64702_d_n7;
        locals.var_x_s__blk1363_dn8 = assign50010_e64702_d_n8;
        locals.var_x_s__blk1363_dn9 = assign50010_e64702_d_n9;

        let (assign50020_e64708, assign50020_e64708_d_n4, assign50020_e64708_d_n6, assign50020_e64708_d_n7, assign50020_e64708_d_n8, assign50020_e64708_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xi1s__blk1366, locals.var_xi1s__blk1366_dn4, locals.var_xi1s__blk1366_dn6, locals.var_xi1s__blk1366_dn7, locals.var_xi1s__blk1366_dn8, locals.var_xi1s__blk1366_dn9,)
    }
};
        locals.var_xi1s__blk1366 = assign50020_e64708;
        locals.var_xi1s__blk1366_dn4 = assign50020_e64708_d_n4;
        locals.var_xi1s__blk1366_dn6 = assign50020_e64708_d_n6;
        locals.var_xi1s__blk1366_dn7 = assign50020_e64708_d_n7;
        locals.var_xi1s__blk1366_dn8 = assign50020_e64708_d_n8;
        locals.var_xi1s__blk1366_dn9 = assign50020_e64708_d_n9;

        let (assign50030_e64714, assign50030_e64714_d_n4, assign50030_e64714_d_n6, assign50030_e64714_d_n7, assign50030_e64714_d_n8, assign50030_e64714_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xi2s__blk1367, locals.var_xi2s__blk1367_dn4, locals.var_xi2s__blk1367_dn6, locals.var_xi2s__blk1367_dn7, locals.var_xi2s__blk1367_dn8, locals.var_xi2s__blk1367_dn9,)
    }
};
        locals.var_xi2s__blk1367 = assign50030_e64714;
        locals.var_xi2s__blk1367_dn4 = assign50030_e64714_d_n4;
        locals.var_xi2s__blk1367_dn6 = assign50030_e64714_d_n6;
        locals.var_xi2s__blk1367_dn7 = assign50030_e64714_d_n7;
        locals.var_xi2s__blk1367_dn8 = assign50030_e64714_d_n8;
        locals.var_xi2s__blk1367_dn9 = assign50030_e64714_d_n9;

        let (assign50040_e64720, assign50040_e64720_d_n4, assign50040_e64720_d_n6, assign50040_e64720_d_n7, assign50040_e64720_d_n8, assign50040_e64720_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9,)
    }
};
        locals.var_delta_1s__blk1368 = assign50040_e64720;
        locals.var_delta_1s__blk1368_dn4 = assign50040_e64720_d_n4;
        locals.var_delta_1s__blk1368_dn6 = assign50040_e64720_d_n6;
        locals.var_delta_1s__blk1368_dn7 = assign50040_e64720_d_n7;
        locals.var_delta_1s__blk1368_dn8 = assign50040_e64720_d_n8;
        locals.var_delta_1s__blk1368_dn9 = assign50040_e64720_d_n9;

        let (assign50050_e64726, assign50050_e64726_d_n4, assign50050_e64726_d_n6, assign50050_e64726_d_n7, assign50050_e64726_d_n8, assign50050_e64726_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9,)
    }
};
        locals.var_es__blk1369 = assign50050_e64726;
        locals.var_es__blk1369_dn4 = assign50050_e64726_d_n4;
        locals.var_es__blk1369_dn6 = assign50050_e64726_d_n6;
        locals.var_es__blk1369_dn7 = assign50050_e64726_d_n7;
        locals.var_es__blk1369_dn8 = assign50050_e64726_d_n8;
        locals.var_es__blk1369_dn9 = assign50050_e64726_d_n9;

        let (assign50060_e64732, assign50060_e64732_d_n4, assign50060_e64732_d_n6, assign50060_e64732_d_n7, assign50060_e64732_d_n8, assign50060_e64732_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9,)
    }
};
        locals.var_ds__blk1370 = assign50060_e64732;
        locals.var_ds__blk1370_dn4 = assign50060_e64732_d_n4;
        locals.var_ds__blk1370_dn6 = assign50060_e64732_d_n6;
        locals.var_ds__blk1370_dn7 = assign50060_e64732_d_n7;
        locals.var_ds__blk1370_dn8 = assign50060_e64732_d_n8;
        locals.var_ds__blk1370_dn9 = assign50060_e64732_d_n9;

        let (assign50070_e64738, assign50070_e64738_d_n4, assign50070_e64738_d_n6, assign50070_e64738_d_n7, assign50070_e64738_d_n8, assign50070_e64738_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9,)
    }
};
        locals.var_ps__blk1371 = assign50070_e64738;
        locals.var_ps__blk1371_dn4 = assign50070_e64738_d_n4;
        locals.var_ps__blk1371_dn6 = assign50070_e64738_d_n6;
        locals.var_ps__blk1371_dn7 = assign50070_e64738_d_n7;
        locals.var_ps__blk1371_dn8 = assign50070_e64738_d_n8;
        locals.var_ps__blk1371_dn9 = assign50070_e64738_d_n9;

        let (assign50080_e64744, assign50080_e64744_d_n4, assign50080_e64744_d_n6, assign50080_e64744_d_n7, assign50080_e64744_d_n8, assign50080_e64744_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sqs__blk1372, locals.var_sqs__blk1372_dn4, locals.var_sqs__blk1372_dn6, locals.var_sqs__blk1372_dn7, locals.var_sqs__blk1372_dn8, locals.var_sqs__blk1372_dn9,)
    }
};
        locals.var_sqs__blk1372 = assign50080_e64744;
        locals.var_sqs__blk1372_dn4 = assign50080_e64744_d_n4;
        locals.var_sqs__blk1372_dn6 = assign50080_e64744_d_n6;
        locals.var_sqs__blk1372_dn7 = assign50080_e64744_d_n7;
        locals.var_sqs__blk1372_dn8 = assign50080_e64744_d_n8;
        locals.var_sqs__blk1372_dn9 = assign50080_e64744_d_n9;

        let (assign50090_e64750, assign50090_e64750_d_n4, assign50090_e64750_d_n6, assign50090_e64750_d_n7, assign50090_e64750_d_n8, assign50090_e64750_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_alphas__blk1373, locals.var_alphas__blk1373_dn4, locals.var_alphas__blk1373_dn6, locals.var_alphas__blk1373_dn7, locals.var_alphas__blk1373_dn8, locals.var_alphas__blk1373_dn9,)
    }
};
        locals.var_alphas__blk1373 = assign50090_e64750;
        locals.var_alphas__blk1373_dn4 = assign50090_e64750_d_n4;
        locals.var_alphas__blk1373_dn6 = assign50090_e64750_d_n6;
        locals.var_alphas__blk1373_dn7 = assign50090_e64750_d_n7;
        locals.var_alphas__blk1373_dn8 = assign50090_e64750_d_n8;
        locals.var_alphas__blk1373_dn9 = assign50090_e64750_d_n9;

        let (assign50100_e64756, assign50100_e64756_d_n4, assign50100_e64756_d_n6, assign50100_e64756_d_n7, assign50100_e64756_d_n8, assign50100_e64756_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rxcor__blk1374, locals.var_rxcor__blk1374_dn4, locals.var_rxcor__blk1374_dn6, locals.var_rxcor__blk1374_dn7, locals.var_rxcor__blk1374_dn8, locals.var_rxcor__blk1374_dn9,)
    }
};
        locals.var_rxcor__blk1374 = assign50100_e64756;
        locals.var_rxcor__blk1374_dn4 = assign50100_e64756_d_n4;
        locals.var_rxcor__blk1374_dn6 = assign50100_e64756_d_n6;
        locals.var_rxcor__blk1374_dn7 = assign50100_e64756_d_n7;
        locals.var_rxcor__blk1374_dn8 = assign50100_e64756_d_n8;
        locals.var_rxcor__blk1374_dn9 = assign50100_e64756_d_n9;

        let (assign50110_e64764, assign50110_e64764_d_n4, assign50110_e64764_d_n6, assign50110_e64764_d_n7, assign50110_e64764_d_n8, assign50110_e64764_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign50110_e64762: f64 = (locals.var_xg__blk1343 - locals.var_x_s__blk1363);
        (assign50110_e64762, (locals.var_xg__blk1343_dn4 - locals.var_x_s__blk1363_dn4), (locals.var_xg__blk1343_dn6 - locals.var_x_s__blk1363_dn6), (locals.var_xg__blk1343_dn7 - locals.var_x_s__blk1363_dn7), (locals.var_xg__blk1343_dn8 - locals.var_x_s__blk1363_dn8), (locals.var_xg__blk1343_dn9 - locals.var_x_s__blk1363_dn9),)
    } else {
        (locals.var_xgs__blk1375, locals.var_xgs__blk1375_dn4, locals.var_xgs__blk1375_dn6, locals.var_xgs__blk1375_dn7, locals.var_xgs__blk1375_dn8, locals.var_xgs__blk1375_dn9,)
    }
};
        locals.var_xgs__blk1375 = assign50110_e64764;
        locals.var_xgs__blk1375_dn4 = assign50110_e64764_d_n4;
        locals.var_xgs__blk1375_dn6 = assign50110_e64764_d_n6;
        locals.var_xgs__blk1375_dn7 = assign50110_e64764_d_n7;
        locals.var_xgs__blk1375_dn8 = assign50110_e64764_d_n8;
        locals.var_xgs__blk1375_dn9 = assign50110_e64764_d_n9;

        let (assign50120_e64770, assign50120_e64770_d_n4, assign50120_e64770_d_n6, assign50120_e64770_d_n7, assign50120_e64770_d_n8, assign50120_e64770_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qis__blk1376, locals.var_qis__blk1376_dn4, locals.var_qis__blk1376_dn6, locals.var_qis__blk1376_dn7, locals.var_qis__blk1376_dn8, locals.var_qis__blk1376_dn9,)
    }
};
        locals.var_qis__blk1376 = assign50120_e64770;
        locals.var_qis__blk1376_dn4 = assign50120_e64770_d_n4;
        locals.var_qis__blk1376_dn6 = assign50120_e64770_d_n6;
        locals.var_qis__blk1376_dn7 = assign50120_e64770_d_n7;
        locals.var_qis__blk1376_dn8 = assign50120_e64770_d_n8;
        locals.var_qis__blk1376_dn9 = assign50120_e64770_d_n9;

        let (assign50130_e64778, assign50130_e64778_d_n4, assign50130_e64778_d_n6, assign50130_e64778_d_n7, assign50130_e64778_d_n8, assign50130_e64778_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        let assign50130_e64776: f64 = (locals.var_phit1__blk1339 * locals.var_xgs__blk1375);
        (assign50130_e64776, ((locals.var_phit1__blk1339_dn4 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn4)), ((locals.var_phit1__blk1339_dn6 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn6)), ((locals.var_phit1__blk1339_dn7 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn7)), ((locals.var_phit1__blk1339_dn8 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn8)), ((locals.var_phit1__blk1339_dn9 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn9)),)
    } else {
        (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9,)
    }
};
        locals.var_qbs__blk1377 = assign50130_e64778;
        locals.var_qbs__blk1377_dn4 = assign50130_e64778_d_n4;
        locals.var_qbs__blk1377_dn6 = assign50130_e64778_d_n6;
        locals.var_qbs__blk1377_dn7 = assign50130_e64778_d_n7;
        locals.var_qbs__blk1377_dn8 = assign50130_e64778_d_n8;
        locals.var_qbs__blk1377_dn9 = assign50130_e64778_d_n9;

        let (assign50140_e64784, assign50140_e64784_d_n4, assign50140_e64784_d_n6, assign50140_e64784_d_n7, assign50140_e64784_d_n8, assign50140_e64784_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rhob__blk1378, locals.var_rhob__blk1378_dn4, locals.var_rhob__blk1378_dn6, locals.var_rhob__blk1378_dn7, locals.var_rhob__blk1378_dn8, locals.var_rhob__blk1378_dn9,)
    }
};
        locals.var_rhob__blk1378 = assign50140_e64784;
        locals.var_rhob__blk1378_dn4 = assign50140_e64784_d_n4;
        locals.var_rhob__blk1378_dn6 = assign50140_e64784_d_n6;
        locals.var_rhob__blk1378_dn7 = assign50140_e64784_d_n7;
        locals.var_rhob__blk1378_dn8 = assign50140_e64784_d_n8;
        locals.var_rhob__blk1378_dn9 = assign50140_e64784_d_n9;

        let (assign50150_e64790, assign50150_e64790_d_n4, assign50150_e64790_d_n6, assign50150_e64790_d_n7, assign50150_e64790_d_n8, assign50150_e64790_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rhog__blk1379, locals.var_rhog__blk1379_dn4, locals.var_rhog__blk1379_dn6, locals.var_rhog__blk1379_dn7, locals.var_rhog__blk1379_dn8, locals.var_rhog__blk1379_dn9,)
    }
};
        locals.var_rhog__blk1379 = assign50150_e64790;
        locals.var_rhog__blk1379_dn4 = assign50150_e64790_d_n4;
        locals.var_rhog__blk1379_dn6 = assign50150_e64790_d_n6;
        locals.var_rhog__blk1379_dn7 = assign50150_e64790_d_n7;
        locals.var_rhog__blk1379_dn8 = assign50150_e64790_d_n8;
        locals.var_rhog__blk1379_dn9 = assign50150_e64790_d_n9;

    }

    pub(super) fn stamp_transient_block_41(
        locals: &mut StampLocals,
    ) {
        let (assign50160_e64796, assign50160_e64796_d_n4, assign50160_e64796_d_n6, assign50160_e64796_d_n7, assign50160_e64796_d_n8, assign50160_e64796_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gmobs__blk1383, locals.var_gmobs__blk1383_dn4, locals.var_gmobs__blk1383_dn6, locals.var_gmobs__blk1383_dn7, locals.var_gmobs__blk1383_dn8, locals.var_gmobs__blk1383_dn9,)
    }
};
        locals.var_gmobs__blk1383 = assign50160_e64796;
        locals.var_gmobs__blk1383_dn4 = assign50160_e64796_d_n4;
        locals.var_gmobs__blk1383_dn6 = assign50160_e64796_d_n6;
        locals.var_gmobs__blk1383_dn7 = assign50160_e64796_d_n7;
        locals.var_gmobs__blk1383_dn8 = assign50160_e64796_d_n8;
        locals.var_gmobs__blk1383_dn9 = assign50160_e64796_d_n9;

        let (assign50170_e64802, assign50170_e64802_d_n4, assign50170_e64802_d_n6, assign50170_e64802_d_n7, assign50170_e64802_d_n8, assign50170_e64802_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xitsb__blk1384, locals.var_xitsb__blk1384_dn4, locals.var_xitsb__blk1384_dn6, locals.var_xitsb__blk1384_dn7, locals.var_xitsb__blk1384_dn8, locals.var_xitsb__blk1384_dn9,)
    }
};
        locals.var_xitsb__blk1384 = assign50170_e64802;
        locals.var_xitsb__blk1384_dn4 = assign50170_e64802_d_n4;
        locals.var_xitsb__blk1384_dn6 = assign50170_e64802_d_n6;
        locals.var_xitsb__blk1384_dn7 = assign50170_e64802_d_n7;
        locals.var_xitsb__blk1384_dn8 = assign50170_e64802_d_n8;
        locals.var_xitsb__blk1384_dn9 = assign50170_e64802_d_n9;

        let (assign50180_e64808, assign50180_e64808_d_n4, assign50180_e64808_d_n6, assign50180_e64808_d_n7, assign50180_e64808_d_n8, assign50180_e64808_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_factheta__blk1386, locals.var_factheta__blk1386_dn4, locals.var_factheta__blk1386_dn6, locals.var_factheta__blk1386_dn7, locals.var_factheta__blk1386_dn8, locals.var_factheta__blk1386_dn9,)
    }
};
        locals.var_factheta__blk1386 = assign50180_e64808;
        locals.var_factheta__blk1386_dn4 = assign50180_e64808_d_n4;
        locals.var_factheta__blk1386_dn6 = assign50180_e64808_d_n6;
        locals.var_factheta__blk1386_dn7 = assign50180_e64808_d_n7;
        locals.var_factheta__blk1386_dn8 = assign50180_e64808_d_n8;
        locals.var_factheta__blk1386_dn9 = assign50180_e64808_d_n9;

        let assign50190_e64811: f64 = if locals.var_xg__blk1343 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1491 = assign50190_e64811;

        let (assign50200_e64825, assign50200_e64825_d_n4, assign50200_e64825_d_n6, assign50200_e64825_d_n7, assign50200_e64825_d_n8, assign50200_e64825_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign50200_e64821: f64 = (locals.var_x_s__blk1363 * locals.var_x_s__blk1363);
        let assign50200_e64822: f64 = (2.0 + assign50200_e64821);
        let assign50200_e64823: f64 = (1.0 / assign50200_e64822);
        (assign50200_e64823, (-(((locals.var_x_s__blk1363_dn4 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn4)) / (assign50200_e64822 * assign50200_e64822))), (-(((locals.var_x_s__blk1363_dn6 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn6)) / (assign50200_e64822 * assign50200_e64822))), (-(((locals.var_x_s__blk1363_dn7 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn7)) / (assign50200_e64822 * assign50200_e64822))), (-(((locals.var_x_s__blk1363_dn8 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn8)) / (assign50200_e64822 * assign50200_e64822))), (-(((locals.var_x_s__blk1363_dn9 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn9)) / (assign50200_e64822 * assign50200_e64822))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign50200_e64825;
        locals.var_temp__blk949_dn4 = assign50200_e64825_d_n4;
        locals.var_temp__blk949_dn6 = assign50200_e64825_d_n6;
        locals.var_temp__blk949_dn7 = assign50200_e64825_d_n7;
        locals.var_temp__blk949_dn8 = assign50200_e64825_d_n8;
        locals.var_temp__blk949_dn9 = assign50200_e64825_d_n9;

        let (assign50210_e64837, assign50210_e64837_d_n4, assign50210_e64837_d_n6, assign50210_e64837_d_n7, assign50210_e64837_d_n8, assign50210_e64837_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign50210_e64833: f64 = (locals.var_x_s__blk1363 * locals.var_x_s__blk1363);
        let assign50210_e64835: f64 = (assign50210_e64833 * locals.var_temp__blk949);
        (assign50210_e64835, ((((locals.var_x_s__blk1363_dn4 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn4)) * locals.var_temp__blk949) + (assign50210_e64833 * locals.var_temp__blk949_dn4)), ((((locals.var_x_s__blk1363_dn6 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn6)) * locals.var_temp__blk949) + (assign50210_e64833 * locals.var_temp__blk949_dn6)), ((((locals.var_x_s__blk1363_dn7 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn7)) * locals.var_temp__blk949) + (assign50210_e64833 * locals.var_temp__blk949_dn7)), ((((locals.var_x_s__blk1363_dn8 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn8)) * locals.var_temp__blk949) + (assign50210_e64833 * locals.var_temp__blk949_dn8)), ((((locals.var_x_s__blk1363_dn9 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn9)) * locals.var_temp__blk949) + (assign50210_e64833 * locals.var_temp__blk949_dn9)),)
    } else {
        (locals.var_xi0s__blk1365, locals.var_xi0s__blk1365_dn4, locals.var_xi0s__blk1365_dn6, locals.var_xi0s__blk1365_dn7, locals.var_xi0s__blk1365_dn8, locals.var_xi0s__blk1365_dn9,)
    }
};
        locals.var_xi0s__blk1365 = assign50210_e64837;
        locals.var_xi0s__blk1365_dn4 = assign50210_e64837_d_n4;
        locals.var_xi0s__blk1365_dn6 = assign50210_e64837_d_n6;
        locals.var_xi0s__blk1365_dn7 = assign50210_e64837_d_n7;
        locals.var_xi0s__blk1365_dn8 = assign50210_e64837_d_n8;
        locals.var_xi0s__blk1365_dn9 = assign50210_e64837_d_n9;

        let (assign50220_e64851, assign50220_e64851_d_n4, assign50220_e64851_d_n6, assign50220_e64851_d_n7, assign50220_e64851_d_n8, assign50220_e64851_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign50220_e64846: f64 = (locals.var_x_s__blk1363 * locals.var_temp__blk949);
        let assign50220_e64848: f64 = (assign50220_e64846 * locals.var_temp__blk949);
        let assign50220_e64849: f64 = (4.0 * assign50220_e64848);
        (assign50220_e64849, (4.0 * ((((locals.var_x_s__blk1363_dn4 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn4)) * locals.var_temp__blk949) + (assign50220_e64846 * locals.var_temp__blk949_dn4))), (4.0 * ((((locals.var_x_s__blk1363_dn6 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn6)) * locals.var_temp__blk949) + (assign50220_e64846 * locals.var_temp__blk949_dn6))), (4.0 * ((((locals.var_x_s__blk1363_dn7 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn7)) * locals.var_temp__blk949) + (assign50220_e64846 * locals.var_temp__blk949_dn7))), (4.0 * ((((locals.var_x_s__blk1363_dn8 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn8)) * locals.var_temp__blk949) + (assign50220_e64846 * locals.var_temp__blk949_dn8))), (4.0 * ((((locals.var_x_s__blk1363_dn9 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn9)) * locals.var_temp__blk949) + (assign50220_e64846 * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_xi1s__blk1366, locals.var_xi1s__blk1366_dn4, locals.var_xi1s__blk1366_dn6, locals.var_xi1s__blk1366_dn7, locals.var_xi1s__blk1366_dn8, locals.var_xi1s__blk1366_dn9,)
    }
};
        locals.var_xi1s__blk1366 = assign50220_e64851;
        locals.var_xi1s__blk1366_dn4 = assign50220_e64851_d_n4;
        locals.var_xi1s__blk1366_dn6 = assign50220_e64851_d_n6;
        locals.var_xi1s__blk1366_dn7 = assign50220_e64851_d_n7;
        locals.var_xi1s__blk1366_dn8 = assign50220_e64851_d_n8;
        locals.var_xi1s__blk1366_dn9 = assign50220_e64851_d_n9;

        let (assign50230_e64869, assign50230_e64869_d_n4, assign50230_e64869_d_n6, assign50230_e64869_d_n7, assign50230_e64869_d_n8, assign50230_e64869_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign50230_e64859: f64 = (8.0 * locals.var_temp__blk949);
        let assign50230_e64862: f64 = (12.0 * locals.var_xi0s__blk1365);
        let assign50230_e64863: f64 = (assign50230_e64859 - assign50230_e64862);
        let assign50230_e64865: f64 = (assign50230_e64863 * locals.var_temp__blk949);
        let assign50230_e64867: f64 = (assign50230_e64865 * locals.var_temp__blk949);
        (assign50230_e64867, ((((((8.0 * locals.var_temp__blk949_dn4) - (12.0 * locals.var_xi0s__blk1365_dn4)) * locals.var_temp__blk949) + (assign50230_e64863 * locals.var_temp__blk949_dn4)) * locals.var_temp__blk949) + (assign50230_e64865 * locals.var_temp__blk949_dn4)), ((((((8.0 * locals.var_temp__blk949_dn6) - (12.0 * locals.var_xi0s__blk1365_dn6)) * locals.var_temp__blk949) + (assign50230_e64863 * locals.var_temp__blk949_dn6)) * locals.var_temp__blk949) + (assign50230_e64865 * locals.var_temp__blk949_dn6)), ((((((8.0 * locals.var_temp__blk949_dn7) - (12.0 * locals.var_xi0s__blk1365_dn7)) * locals.var_temp__blk949) + (assign50230_e64863 * locals.var_temp__blk949_dn7)) * locals.var_temp__blk949) + (assign50230_e64865 * locals.var_temp__blk949_dn7)), ((((((8.0 * locals.var_temp__blk949_dn8) - (12.0 * locals.var_xi0s__blk1365_dn8)) * locals.var_temp__blk949) + (assign50230_e64863 * locals.var_temp__blk949_dn8)) * locals.var_temp__blk949) + (assign50230_e64865 * locals.var_temp__blk949_dn8)), ((((((8.0 * locals.var_temp__blk949_dn9) - (12.0 * locals.var_xi0s__blk1365_dn9)) * locals.var_temp__blk949) + (assign50230_e64863 * locals.var_temp__blk949_dn9)) * locals.var_temp__blk949) + (assign50230_e64865 * locals.var_temp__blk949_dn9)),)
    } else {
        (locals.var_xi2s__blk1367, locals.var_xi2s__blk1367_dn4, locals.var_xi2s__blk1367_dn6, locals.var_xi2s__blk1367_dn7, locals.var_xi2s__blk1367_dn8, locals.var_xi2s__blk1367_dn9,)
    }
};
        locals.var_xi2s__blk1367 = assign50230_e64869;
        locals.var_xi2s__blk1367_dn4 = assign50230_e64869_d_n4;
        locals.var_xi2s__blk1367_dn6 = assign50230_e64869_d_n6;
        locals.var_xi2s__blk1367_dn7 = assign50230_e64869_d_n7;
        locals.var_xi2s__blk1367_dn8 = assign50230_e64869_d_n8;
        locals.var_xi2s__blk1367_dn9 = assign50230_e64869_d_n9;

        let (assign50240_e64877, assign50240_e64877_d_n4, assign50240_e64877_d_n6, assign50240_e64877_d_n7, assign50240_e64877_d_n8, assign50240_e64877_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9,)
    }
};
        locals.var_delta_1s__blk1368 = assign50240_e64877;
        locals.var_delta_1s__blk1368_dn4 = assign50240_e64877_d_n4;
        locals.var_delta_1s__blk1368_dn6 = assign50240_e64877_d_n6;
        locals.var_delta_1s__blk1368_dn7 = assign50240_e64877_d_n7;
        locals.var_delta_1s__blk1368_dn8 = assign50240_e64877_d_n8;
        locals.var_delta_1s__blk1368_dn9 = assign50240_e64877_d_n9;

        let assign50250_e64880: f64 = if locals.var_x_s__blk1363 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1492 = assign50250_e64880;

        let (assign50260_e64891, assign50260_e64891_d_n4, assign50260_e64891_d_n6, assign50260_e64891_d_n7, assign50260_e64891_d_n8, assign50260_e64891_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 != 0.0)) {
        let assign50260_e64889: f64 = (locals.var_x_s__blk1363).exp();
        (assign50260_e64889, (assign50260_e64889 * locals.var_x_s__blk1363_dn4), (assign50260_e64889 * locals.var_x_s__blk1363_dn6), (assign50260_e64889 * locals.var_x_s__blk1363_dn7), (assign50260_e64889 * locals.var_x_s__blk1363_dn8), (assign50260_e64889 * locals.var_x_s__blk1363_dn9),)
    } else {
        (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9,)
    }
};
        locals.var_delta_1s__blk1368 = assign50260_e64891;
        locals.var_delta_1s__blk1368_dn4 = assign50260_e64891_d_n4;
        locals.var_delta_1s__blk1368_dn6 = assign50260_e64891_d_n6;
        locals.var_delta_1s__blk1368_dn7 = assign50260_e64891_d_n7;
        locals.var_delta_1s__blk1368_dn8 = assign50260_e64891_d_n8;
        locals.var_delta_1s__blk1368_dn9 = assign50260_e64891_d_n9;

        let (assign50270_e64903, assign50270_e64903_d_n4, assign50270_e64903_d_n6, assign50270_e64903_d_n7, assign50270_e64903_d_n8, assign50270_e64903_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 != 0.0)) {
        let assign50270_e64901: f64 = (1.0 / locals.var_delta_1s__blk1368);
        (assign50270_e64901, (-(locals.var_delta_1s__blk1368_dn4 / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368))), (-(locals.var_delta_1s__blk1368_dn6 / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368))), (-(locals.var_delta_1s__blk1368_dn7 / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368))), (-(locals.var_delta_1s__blk1368_dn8 / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368))), (-(locals.var_delta_1s__blk1368_dn9 / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368))),)
    } else {
        (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9,)
    }
};
        locals.var_es__blk1369 = assign50270_e64903;
        locals.var_es__blk1369_dn4 = assign50270_e64903_d_n4;
        locals.var_es__blk1369_dn6 = assign50270_e64903_d_n6;
        locals.var_es__blk1369_dn7 = assign50270_e64903_d_n7;
        locals.var_es__blk1369_dn8 = assign50270_e64903_d_n8;
        locals.var_es__blk1369_dn9 = assign50270_e64903_d_n9;

        let (assign50280_e64915, assign50280_e64915_d_n4, assign50280_e64915_d_n6, assign50280_e64915_d_n7, assign50280_e64915_d_n8, assign50280_e64915_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 != 0.0)) {
        let assign50280_e64913: f64 = (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368);
        (assign50280_e64913, ((locals.var_delta_ns__blk1364_dn4 * locals.var_delta_1s__blk1368) + (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn4)), ((locals.var_delta_ns__blk1364_dn6 * locals.var_delta_1s__blk1368) + (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn6)), ((locals.var_delta_ns__blk1364_dn7 * locals.var_delta_1s__blk1368) + (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn7)), ((locals.var_delta_ns__blk1364_dn8 * locals.var_delta_1s__blk1368) + (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn8)), ((locals.var_delta_ns__blk1364_dn9 * locals.var_delta_1s__blk1368) + (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn9)),)
    } else {
        (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9,)
    }
};
        locals.var_delta_1s__blk1368 = assign50280_e64915;
        locals.var_delta_1s__blk1368_dn4 = assign50280_e64915_d_n4;
        locals.var_delta_1s__blk1368_dn6 = assign50280_e64915_d_n6;
        locals.var_delta_1s__blk1368_dn7 = assign50280_e64915_d_n7;
        locals.var_delta_1s__blk1368_dn8 = assign50280_e64915_d_n8;
        locals.var_delta_1s__blk1368_dn9 = assign50280_e64915_d_n9;

        let assign50290_e64919: f64 = (locals.var_xn_s__blk1349 - 230.25850929940458);
        let assign50290_e64920: f64 = if locals.var_x_s__blk1363 > assign50290_e64919 { 1.0 } else { 0.0 };
        locals.var_guard1493 = assign50290_e64920;

        let (assign50300_e64936, assign50300_e64936_d_n4, assign50300_e64936_d_n6, assign50300_e64936_d_n7, assign50300_e64936_d_n8, assign50300_e64936_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 != 0.0)) {
        let assign50300_e64933: f64 = (locals.var_x_s__blk1363 - locals.var_xn_s__blk1349);
        let assign50300_e64934: f64 = (assign50300_e64933).exp();
        (assign50300_e64934, (assign50300_e64934 * (locals.var_x_s__blk1363_dn4 - locals.var_xn_s__blk1349_dn4)), (assign50300_e64934 * (locals.var_x_s__blk1363_dn6 - locals.var_xn_s__blk1349_dn6)), (assign50300_e64934 * (locals.var_x_s__blk1363_dn7 - locals.var_xn_s__blk1349_dn7)), (assign50300_e64934 * (locals.var_x_s__blk1363_dn8 - locals.var_xn_s__blk1349_dn8)), (assign50300_e64934 * (locals.var_x_s__blk1363_dn9 - locals.var_xn_s__blk1349_dn9)),)
    } else {
        (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9,)
    }
};
        locals.var_delta_1s__blk1368 = assign50300_e64936;
        locals.var_delta_1s__blk1368_dn4 = assign50300_e64936_d_n4;
        locals.var_delta_1s__blk1368_dn6 = assign50300_e64936_d_n6;
        locals.var_delta_1s__blk1368_dn7 = assign50300_e64936_d_n7;
        locals.var_delta_1s__blk1368_dn8 = assign50300_e64936_d_n8;
        locals.var_delta_1s__blk1368_dn9 = assign50300_e64936_d_n9;

        let (assign50310_e64951, assign50310_e64951_d_n4, assign50310_e64951_d_n6, assign50310_e64951_d_n7, assign50310_e64951_d_n8, assign50310_e64951_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 != 0.0)) {
        let assign50310_e64949: f64 = (locals.var_delta_ns__blk1364 / locals.var_delta_1s__blk1368);
        (assign50310_e64949, (((locals.var_delta_ns__blk1364_dn4 * locals.var_delta_1s__blk1368) - (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn4)) / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368)), (((locals.var_delta_ns__blk1364_dn6 * locals.var_delta_1s__blk1368) - (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn6)) / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368)), (((locals.var_delta_ns__blk1364_dn7 * locals.var_delta_1s__blk1368) - (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn7)) / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368)), (((locals.var_delta_ns__blk1364_dn8 * locals.var_delta_1s__blk1368) - (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn8)) / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368)), (((locals.var_delta_ns__blk1364_dn9 * locals.var_delta_1s__blk1368) - (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn9)) / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368)),)
    } else {
        (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9,)
    }
};
        locals.var_es__blk1369 = assign50310_e64951;
        locals.var_es__blk1369_dn4 = assign50310_e64951_d_n4;
        locals.var_es__blk1369_dn6 = assign50310_e64951_d_n6;
        locals.var_es__blk1369_dn7 = assign50310_e64951_d_n7;
        locals.var_es__blk1369_dn8 = assign50310_e64951_d_n8;
        locals.var_es__blk1369_dn9 = assign50310_e64951_d_n9;

        let (assign50320_e64993, assign50320_e64993_d_n4, assign50320_e64993_d_n6, assign50320_e64993_d_n7, assign50320_e64993_d_n8, assign50320_e64993_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) {
        let assign50320_e64967: f64 = (locals.var_xn_s__blk1349 - locals.var_x_s__blk1363);
        let assign50320_e64969: f64 = (assign50320_e64967 - 230.25850929940458);
        let assign50320_e64974: f64 = (locals.var_xn_s__blk1349 - locals.var_x_s__blk1363);
        let assign50320_e64976: f64 = (assign50320_e64974 - 230.25850929940458);
        let assign50320_e64980: f64 = (locals.var_xn_s__blk1349 - locals.var_x_s__blk1363);
        let assign50320_e64982: f64 = (assign50320_e64980 - 230.25850929940458);
        let assign50320_e64984: f64 = (assign50320_e64982 * 0.3333333333333333);
        let assign50320_e64985: f64 = (1.0 + assign50320_e64984);
        let assign50320_e64986: f64 = (assign50320_e64976 * assign50320_e64985);
        let assign50320_e64987: f64 = (0.5 * assign50320_e64986);
        let assign50320_e64988: f64 = (1.0 + assign50320_e64987);
        let assign50320_e64989: f64 = (assign50320_e64969 * assign50320_e64988);
        let assign50320_e64990: f64 = (1.0 + assign50320_e64989);
        let assign50320_e64991: f64 = (1e-100 / assign50320_e64990);
        (assign50320_e64991, (-((1e-100 * (((locals.var_xn_s__blk1349_dn4 - locals.var_x_s__blk1363_dn4) * assign50320_e64988) + (assign50320_e64969 * (0.5 * (((locals.var_xn_s__blk1349_dn4 - locals.var_x_s__blk1363_dn4) * assign50320_e64985) + (assign50320_e64976 * ((locals.var_xn_s__blk1349_dn4 - locals.var_x_s__blk1363_dn4) * 0.3333333333333333))))))) / (assign50320_e64990 * assign50320_e64990))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn6 - locals.var_x_s__blk1363_dn6) * assign50320_e64988) + (assign50320_e64969 * (0.5 * (((locals.var_xn_s__blk1349_dn6 - locals.var_x_s__blk1363_dn6) * assign50320_e64985) + (assign50320_e64976 * ((locals.var_xn_s__blk1349_dn6 - locals.var_x_s__blk1363_dn6) * 0.3333333333333333))))))) / (assign50320_e64990 * assign50320_e64990))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn7 - locals.var_x_s__blk1363_dn7) * assign50320_e64988) + (assign50320_e64969 * (0.5 * (((locals.var_xn_s__blk1349_dn7 - locals.var_x_s__blk1363_dn7) * assign50320_e64985) + (assign50320_e64976 * ((locals.var_xn_s__blk1349_dn7 - locals.var_x_s__blk1363_dn7) * 0.3333333333333333))))))) / (assign50320_e64990 * assign50320_e64990))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn8 - locals.var_x_s__blk1363_dn8) * assign50320_e64988) + (assign50320_e64969 * (0.5 * (((locals.var_xn_s__blk1349_dn8 - locals.var_x_s__blk1363_dn8) * assign50320_e64985) + (assign50320_e64976 * ((locals.var_xn_s__blk1349_dn8 - locals.var_x_s__blk1363_dn8) * 0.3333333333333333))))))) / (assign50320_e64990 * assign50320_e64990))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn9 - locals.var_x_s__blk1363_dn9) * assign50320_e64988) + (assign50320_e64969 * (0.5 * (((locals.var_xn_s__blk1349_dn9 - locals.var_x_s__blk1363_dn9) * assign50320_e64985) + (assign50320_e64976 * ((locals.var_xn_s__blk1349_dn9 - locals.var_x_s__blk1363_dn9) * 0.3333333333333333))))))) / (assign50320_e64990 * assign50320_e64990))),)
    } else {
        (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9,)
    }
};
        locals.var_delta_1s__blk1368 = assign50320_e64993;
        locals.var_delta_1s__blk1368_dn4 = assign50320_e64993_d_n4;
        locals.var_delta_1s__blk1368_dn6 = assign50320_e64993_d_n6;
        locals.var_delta_1s__blk1368_dn7 = assign50320_e64993_d_n7;
        locals.var_delta_1s__blk1368_dn8 = assign50320_e64993_d_n8;
        locals.var_delta_1s__blk1368_dn9 = assign50320_e64993_d_n9;

        let (assign50330_e65029, assign50330_e65029_d_n4, assign50330_e65029_d_n6, assign50330_e65029_d_n7, assign50330_e65029_d_n8, assign50330_e65029_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) {
        let assign50330_e65009: f64 = (locals.var_x_s__blk1363 - 230.25850929940458);
        let assign50330_e65014: f64 = (locals.var_x_s__blk1363 - 230.25850929940458);
        let assign50330_e65018: f64 = (locals.var_x_s__blk1363 - 230.25850929940458);
        let assign50330_e65020: f64 = (assign50330_e65018 * 0.3333333333333333);
        let assign50330_e65021: f64 = (1.0 + assign50330_e65020);
        let assign50330_e65022: f64 = (assign50330_e65014 * assign50330_e65021);
        let assign50330_e65023: f64 = (0.5 * assign50330_e65022);
        let assign50330_e65024: f64 = (1.0 + assign50330_e65023);
        let assign50330_e65025: f64 = (assign50330_e65009 * assign50330_e65024);
        let assign50330_e65026: f64 = (1.0 + assign50330_e65025);
        let assign50330_e65027: f64 = (1e-100 / assign50330_e65026);
        (assign50330_e65027, (-((1e-100 * ((locals.var_x_s__blk1363_dn4 * assign50330_e65024) + (assign50330_e65009 * (0.5 * ((locals.var_x_s__blk1363_dn4 * assign50330_e65021) + (assign50330_e65014 * (locals.var_x_s__blk1363_dn4 * 0.3333333333333333))))))) / (assign50330_e65026 * assign50330_e65026))), (-((1e-100 * ((locals.var_x_s__blk1363_dn6 * assign50330_e65024) + (assign50330_e65009 * (0.5 * ((locals.var_x_s__blk1363_dn6 * assign50330_e65021) + (assign50330_e65014 * (locals.var_x_s__blk1363_dn6 * 0.3333333333333333))))))) / (assign50330_e65026 * assign50330_e65026))), (-((1e-100 * ((locals.var_x_s__blk1363_dn7 * assign50330_e65024) + (assign50330_e65009 * (0.5 * ((locals.var_x_s__blk1363_dn7 * assign50330_e65021) + (assign50330_e65014 * (locals.var_x_s__blk1363_dn7 * 0.3333333333333333))))))) / (assign50330_e65026 * assign50330_e65026))), (-((1e-100 * ((locals.var_x_s__blk1363_dn8 * assign50330_e65024) + (assign50330_e65009 * (0.5 * ((locals.var_x_s__blk1363_dn8 * assign50330_e65021) + (assign50330_e65014 * (locals.var_x_s__blk1363_dn8 * 0.3333333333333333))))))) / (assign50330_e65026 * assign50330_e65026))), (-((1e-100 * ((locals.var_x_s__blk1363_dn9 * assign50330_e65024) + (assign50330_e65009 * (0.5 * ((locals.var_x_s__blk1363_dn9 * assign50330_e65021) + (assign50330_e65014 * (locals.var_x_s__blk1363_dn9 * 0.3333333333333333))))))) / (assign50330_e65026 * assign50330_e65026))),)
    } else {
        (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9,)
    }
};
        locals.var_es__blk1369 = assign50330_e65029;
        locals.var_es__blk1369_dn4 = assign50330_e65029_d_n4;
        locals.var_es__blk1369_dn6 = assign50330_e65029_d_n6;
        locals.var_es__blk1369_dn7 = assign50330_e65029_d_n7;
        locals.var_es__blk1369_dn8 = assign50330_e65029_d_n8;
        locals.var_es__blk1369_dn9 = assign50330_e65029_d_n9;

        let (assign50340_e65045, assign50340_e65045_d_n4, assign50340_e65045_d_n6, assign50340_e65045_d_n7, assign50340_e65045_d_n8, assign50340_e65045_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign50340_e65039: f64 = (locals.var_x_s__blk1363 + 1.0);
        let assign50340_e65041: f64 = (assign50340_e65039 + locals.var_xi0s__blk1365);
        let assign50340_e65042: f64 = (locals.var_delta_ns__blk1364 * assign50340_e65041);
        let assign50340_e65043: f64 = (locals.var_delta_1s__blk1368 - assign50340_e65042);
        (assign50340_e65043, (locals.var_delta_1s__blk1368_dn4 - ((locals.var_delta_ns__blk1364_dn4 * assign50340_e65041) + (locals.var_delta_ns__blk1364 * (locals.var_x_s__blk1363_dn4 + locals.var_xi0s__blk1365_dn4)))), (locals.var_delta_1s__blk1368_dn6 - ((locals.var_delta_ns__blk1364_dn6 * assign50340_e65041) + (locals.var_delta_ns__blk1364 * (locals.var_x_s__blk1363_dn6 + locals.var_xi0s__blk1365_dn6)))), (locals.var_delta_1s__blk1368_dn7 - ((locals.var_delta_ns__blk1364_dn7 * assign50340_e65041) + (locals.var_delta_ns__blk1364 * (locals.var_x_s__blk1363_dn7 + locals.var_xi0s__blk1365_dn7)))), (locals.var_delta_1s__blk1368_dn8 - ((locals.var_delta_ns__blk1364_dn8 * assign50340_e65041) + (locals.var_delta_ns__blk1364 * (locals.var_x_s__blk1363_dn8 + locals.var_xi0s__blk1365_dn8)))), (locals.var_delta_1s__blk1368_dn9 - ((locals.var_delta_ns__blk1364_dn9 * assign50340_e65041) + (locals.var_delta_ns__blk1364 * (locals.var_x_s__blk1363_dn9 + locals.var_xi0s__blk1365_dn9)))),)
    } else {
        (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9,)
    }
};
        locals.var_ds__blk1370 = assign50340_e65045;
        locals.var_ds__blk1370_dn4 = assign50340_e65045_d_n4;
        locals.var_ds__blk1370_dn6 = assign50340_e65045_d_n6;
        locals.var_ds__blk1370_dn7 = assign50340_e65045_d_n7;
        locals.var_ds__blk1370_dn8 = assign50340_e65045_d_n8;
        locals.var_ds__blk1370_dn9 = assign50340_e65045_d_n9;

        let assign50350_e65048: f64 = if locals.var_x_s__blk1363 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1494 = assign50350_e65048;

        let (assign50360_e65074, assign50360_e65074_d_n4, assign50360_e65074_d_n6, assign50360_e65074_d_n7, assign50360_e65074_d_n8, assign50360_e65074_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 != 0.0)) {
        let assign50360_e65059: f64 = (locals.var_x_s__blk1363 * locals.var_x_s__blk1363);
        let assign50360_e65066: f64 = (0.25 * locals.var_x_s__blk1363);
        let assign50360_e65067: f64 = (1.0 - assign50360_e65066);
        let assign50360_e65068: f64 = (locals.var_x_s__blk1363 * assign50360_e65067);
        let assign50360_e65069: f64 = (0.3333333333333333 * assign50360_e65068);
        let assign50360_e65070: f64 = (1.0 - assign50360_e65069);
        let assign50360_e65071: f64 = (assign50360_e65059 * assign50360_e65070);
        let assign50360_e65072: f64 = (0.5 * assign50360_e65071);
        (assign50360_e65072, (0.5 * ((((locals.var_x_s__blk1363_dn4 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn4)) * assign50360_e65070) + (assign50360_e65059 * (-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn4 * assign50360_e65067) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn4))))))))), (0.5 * ((((locals.var_x_s__blk1363_dn6 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn6)) * assign50360_e65070) + (assign50360_e65059 * (-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn6 * assign50360_e65067) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn6))))))))), (0.5 * ((((locals.var_x_s__blk1363_dn7 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn7)) * assign50360_e65070) + (assign50360_e65059 * (-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn7 * assign50360_e65067) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn7))))))))), (0.5 * ((((locals.var_x_s__blk1363_dn8 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn8)) * assign50360_e65070) + (assign50360_e65059 * (-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn8 * assign50360_e65067) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn8))))))))), (0.5 * ((((locals.var_x_s__blk1363_dn9 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn9)) * assign50360_e65070) + (assign50360_e65059 * (-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn9 * assign50360_e65067) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn9))))))))),)
    } else {
        (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9,)
    }
};
        locals.var_ps__blk1371 = assign50360_e65074;
        locals.var_ps__blk1371_dn4 = assign50360_e65074_d_n4;
        locals.var_ps__blk1371_dn6 = assign50360_e65074_d_n6;
        locals.var_ps__blk1371_dn7 = assign50360_e65074_d_n7;
        locals.var_ps__blk1371_dn8 = assign50360_e65074_d_n8;
        locals.var_ps__blk1371_dn9 = assign50360_e65074_d_n9;

        let (assign50370_e65098, assign50370_e65098_d_n4, assign50370_e65098_d_n6, assign50370_e65098_d_n7, assign50370_e65098_d_n8, assign50370_e65098_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 != 0.0)) {
        let assign50370_e65085: f64 = (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363);
        let assign50370_e65087: f64 = (assign50370_e65085 * locals.var_x_s__blk1363);
        let assign50370_e65089: f64 = (assign50370_e65087 * locals.var_x_s__blk1363);
        let assign50370_e65093: f64 = (1.75 * locals.var_x_s__blk1363);
        let assign50370_e65094: f64 = (1.0 + assign50370_e65093);
        let assign50370_e65095: f64 = (assign50370_e65089 * assign50370_e65094);
        let assign50370_e65096: f64 = (0.16666666666666666 * assign50370_e65095);
        (assign50370_e65096, (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1364_dn4 * locals.var_x_s__blk1363) + (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363_dn4)) * locals.var_x_s__blk1363) + (assign50370_e65085 * locals.var_x_s__blk1363_dn4)) * locals.var_x_s__blk1363) + (assign50370_e65087 * locals.var_x_s__blk1363_dn4)) * assign50370_e65094) + (assign50370_e65089 * (1.75 * locals.var_x_s__blk1363_dn4)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1364_dn6 * locals.var_x_s__blk1363) + (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363_dn6)) * locals.var_x_s__blk1363) + (assign50370_e65085 * locals.var_x_s__blk1363_dn6)) * locals.var_x_s__blk1363) + (assign50370_e65087 * locals.var_x_s__blk1363_dn6)) * assign50370_e65094) + (assign50370_e65089 * (1.75 * locals.var_x_s__blk1363_dn6)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1364_dn7 * locals.var_x_s__blk1363) + (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363_dn7)) * locals.var_x_s__blk1363) + (assign50370_e65085 * locals.var_x_s__blk1363_dn7)) * locals.var_x_s__blk1363) + (assign50370_e65087 * locals.var_x_s__blk1363_dn7)) * assign50370_e65094) + (assign50370_e65089 * (1.75 * locals.var_x_s__blk1363_dn7)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1364_dn8 * locals.var_x_s__blk1363) + (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363_dn8)) * locals.var_x_s__blk1363) + (assign50370_e65085 * locals.var_x_s__blk1363_dn8)) * locals.var_x_s__blk1363) + (assign50370_e65087 * locals.var_x_s__blk1363_dn8)) * assign50370_e65094) + (assign50370_e65089 * (1.75 * locals.var_x_s__blk1363_dn8)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1364_dn9 * locals.var_x_s__blk1363) + (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363_dn9)) * locals.var_x_s__blk1363) + (assign50370_e65085 * locals.var_x_s__blk1363_dn9)) * locals.var_x_s__blk1363) + (assign50370_e65087 * locals.var_x_s__blk1363_dn9)) * assign50370_e65094) + (assign50370_e65089 * (1.75 * locals.var_x_s__blk1363_dn9)))),)
    } else {
        (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9,)
    }
};
        locals.var_ds__blk1370 = assign50370_e65098;
        locals.var_ds__blk1370_dn4 = assign50370_e65098_d_n4;
        locals.var_ds__blk1370_dn6 = assign50370_e65098_d_n6;
        locals.var_ds__blk1370_dn7 = assign50370_e65098_d_n7;
        locals.var_ds__blk1370_dn8 = assign50370_e65098_d_n8;
        locals.var_ds__blk1370_dn9 = assign50370_e65098_d_n9;

        let (assign50380_e65119, assign50380_e65119_d_n4, assign50380_e65119_d_n6, assign50380_e65119_d_n7, assign50380_e65119_d_n8, assign50380_e65119_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 != 0.0)) {
        let assign50380_e65112: f64 = (0.25 * locals.var_x_s__blk1363);
        let assign50380_e65113: f64 = (1.0 - assign50380_e65112);
        let assign50380_e65114: f64 = (locals.var_x_s__blk1363 * assign50380_e65113);
        let assign50380_e65115: f64 = (0.3333333333333333 * assign50380_e65114);
        let assign50380_e65116: f64 = (1.0 - assign50380_e65115);
        let assign50380_e65117: f64 = (assign50380_e65116).sqrt();
        (assign50380_e65117, ((-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn4 * assign50380_e65113) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn4)))))) / (2.0 * assign50380_e65117)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn6 * assign50380_e65113) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn6)))))) / (2.0 * assign50380_e65117)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn7 * assign50380_e65113) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn7)))))) / (2.0 * assign50380_e65117)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn8 * assign50380_e65113) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn8)))))) / (2.0 * assign50380_e65117)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn9 * assign50380_e65113) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn9)))))) / (2.0 * assign50380_e65117)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign50380_e65119;
        locals.var_temp__blk949_dn4 = assign50380_e65119_d_n4;
        locals.var_temp__blk949_dn6 = assign50380_e65119_d_n6;
        locals.var_temp__blk949_dn7 = assign50380_e65119_d_n7;
        locals.var_temp__blk949_dn8 = assign50380_e65119_d_n8;
        locals.var_temp__blk949_dn9 = assign50380_e65119_d_n9;

        let (assign50390_e65133, assign50390_e65133_d_n4, assign50390_e65133_d_n6, assign50390_e65133_d_n7, assign50390_e65133_d_n8, assign50390_e65133_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 != 0.0)) {
        let assign50390_e65130: f64 = (locals.var_x_s__blk1363 * locals.var_temp__blk949);
        let assign50390_e65131: f64 = (0.7071067811865475 * assign50390_e65130);
        (assign50390_e65131, (0.7071067811865475 * ((locals.var_x_s__blk1363_dn4 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn4))), (0.7071067811865475 * ((locals.var_x_s__blk1363_dn6 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn6))), (0.7071067811865475 * ((locals.var_x_s__blk1363_dn7 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn7))), (0.7071067811865475 * ((locals.var_x_s__blk1363_dn8 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn8))), (0.7071067811865475 * ((locals.var_x_s__blk1363_dn9 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_sqs__blk1372, locals.var_sqs__blk1372_dn4, locals.var_sqs__blk1372_dn6, locals.var_sqs__blk1372_dn7, locals.var_sqs__blk1372_dn8, locals.var_sqs__blk1372_dn9,)
    }
};
        locals.var_sqs__blk1372 = assign50390_e65133;
        locals.var_sqs__blk1372_dn4 = assign50390_e65133_d_n4;
        locals.var_sqs__blk1372_dn6 = assign50390_e65133_d_n6;
        locals.var_sqs__blk1372_dn7 = assign50390_e65133_d_n7;
        locals.var_sqs__blk1372_dn8 = assign50390_e65133_d_n8;
        locals.var_sqs__blk1372_dn9 = assign50390_e65133_d_n9;

        let (assign50400_e65161, assign50400_e65161_d_n4, assign50400_e65161_d_n6, assign50400_e65161_d_n7, assign50400_e65161_d_n8, assign50400_e65161_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 != 0.0)) {
        let assign50400_e65147: f64 = (0.5 * locals.var_x_s__blk1363);
        let assign50400_e65148: f64 = (1.0 - assign50400_e65147);
        let assign50400_e65152: f64 = (locals.var_x_s__blk1363 * locals.var_x_s__blk1363);
        let assign50400_e65153: f64 = (0.16666666666666666 * assign50400_e65152);
        let assign50400_e65154: f64 = (assign50400_e65148 + assign50400_e65153);
        let assign50400_e65155: f64 = (locals.var_gf__blk1324 * assign50400_e65154);
        let assign50400_e65157: f64 = (assign50400_e65155 / locals.var_temp__blk949);
        let assign50400_e65158: f64 = (0.7071067811865475 * assign50400_e65157);
        let assign50400_e65159: f64 = (1.0 + assign50400_e65158);
        (assign50400_e65159, (0.7071067811865475 * (((((locals.var_gf__blk1324_dn4 * assign50400_e65154) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_s__blk1363_dn4)) + (0.16666666666666666 * ((locals.var_x_s__blk1363_dn4 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn4)))))) * locals.var_temp__blk949) - (assign50400_e65155 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf__blk1324_dn6 * assign50400_e65154) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_s__blk1363_dn6)) + (0.16666666666666666 * ((locals.var_x_s__blk1363_dn6 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn6)))))) * locals.var_temp__blk949) - (assign50400_e65155 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf__blk1324_dn7 * assign50400_e65154) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_s__blk1363_dn7)) + (0.16666666666666666 * ((locals.var_x_s__blk1363_dn7 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn7)))))) * locals.var_temp__blk949) - (assign50400_e65155 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf__blk1324_dn8 * assign50400_e65154) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_s__blk1363_dn8)) + (0.16666666666666666 * ((locals.var_x_s__blk1363_dn8 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn8)))))) * locals.var_temp__blk949) - (assign50400_e65155 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf__blk1324_dn9 * assign50400_e65154) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_s__blk1363_dn9)) + (0.16666666666666666 * ((locals.var_x_s__blk1363_dn9 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn9)))))) * locals.var_temp__blk949) - (assign50400_e65155 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949))),)
    } else {
        (locals.var_alphas__blk1373, locals.var_alphas__blk1373_dn4, locals.var_alphas__blk1373_dn6, locals.var_alphas__blk1373_dn7, locals.var_alphas__blk1373_dn8, locals.var_alphas__blk1373_dn9,)
    }
};
        locals.var_alphas__blk1373 = assign50400_e65161;
        locals.var_alphas__blk1373_dn4 = assign50400_e65161_d_n4;
        locals.var_alphas__blk1373_dn6 = assign50400_e65161_d_n6;
        locals.var_alphas__blk1373_dn7 = assign50400_e65161_d_n7;
        locals.var_alphas__blk1373_dn8 = assign50400_e65161_d_n8;
        locals.var_alphas__blk1373_dn9 = assign50400_e65161_d_n9;

        let (assign50410_e65176, assign50410_e65176_d_n4, assign50410_e65176_d_n6, assign50410_e65176_d_n7, assign50410_e65176_d_n8, assign50410_e65176_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 == 0.0)) {
        let assign50410_e65172: f64 = (locals.var_x_s__blk1363 - 1.0);
        let assign50410_e65174: f64 = (assign50410_e65172 + locals.var_es__blk1369);
        (assign50410_e65174, (locals.var_x_s__blk1363_dn4 + locals.var_es__blk1369_dn4), (locals.var_x_s__blk1363_dn6 + locals.var_es__blk1369_dn6), (locals.var_x_s__blk1363_dn7 + locals.var_es__blk1369_dn7), (locals.var_x_s__blk1363_dn8 + locals.var_es__blk1369_dn8), (locals.var_x_s__blk1363_dn9 + locals.var_es__blk1369_dn9),)
    } else {
        (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9,)
    }
};
        locals.var_ps__blk1371 = assign50410_e65176;
        locals.var_ps__blk1371_dn4 = assign50410_e65176_d_n4;
        locals.var_ps__blk1371_dn6 = assign50410_e65176_d_n6;
        locals.var_ps__blk1371_dn7 = assign50410_e65176_d_n7;
        locals.var_ps__blk1371_dn8 = assign50410_e65176_d_n8;
        locals.var_ps__blk1371_dn9 = assign50410_e65176_d_n9;

        let (assign50420_e65188, assign50420_e65188_d_n4, assign50420_e65188_d_n6, assign50420_e65188_d_n7, assign50420_e65188_d_n8, assign50420_e65188_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 == 0.0)) {
        let assign50420_e65186: f64 = (locals.var_ps__blk1371).sqrt();
        (assign50420_e65186, (locals.var_ps__blk1371_dn4 / (2.0 * assign50420_e65186)), (locals.var_ps__blk1371_dn6 / (2.0 * assign50420_e65186)), (locals.var_ps__blk1371_dn7 / (2.0 * assign50420_e65186)), (locals.var_ps__blk1371_dn8 / (2.0 * assign50420_e65186)), (locals.var_ps__blk1371_dn9 / (2.0 * assign50420_e65186)),)
    } else {
        (locals.var_sqs__blk1372, locals.var_sqs__blk1372_dn4, locals.var_sqs__blk1372_dn6, locals.var_sqs__blk1372_dn7, locals.var_sqs__blk1372_dn8, locals.var_sqs__blk1372_dn9,)
    }
};
        locals.var_sqs__blk1372 = assign50420_e65188;
        locals.var_sqs__blk1372_dn4 = assign50420_e65188_d_n4;
        locals.var_sqs__blk1372_dn6 = assign50420_e65188_d_n6;
        locals.var_sqs__blk1372_dn7 = assign50420_e65188_d_n7;
        locals.var_sqs__blk1372_dn8 = assign50420_e65188_d_n8;
        locals.var_sqs__blk1372_dn9 = assign50420_e65188_d_n9;

        let (assign50430_e65209, assign50430_e65209_d_n4, assign50430_e65209_d_n6, assign50430_e65209_d_n7, assign50430_e65209_d_n8, assign50430_e65209_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 == 0.0)) {
        let assign50430_e65202: f64 = (1.0 - locals.var_es__blk1369);
        let assign50430_e65203: f64 = (locals.var_gf__blk1324 * assign50430_e65202);
        let assign50430_e65205: f64 = (assign50430_e65203 / locals.var_sqs__blk1372);
        let assign50430_e65206: f64 = (0.5 * assign50430_e65205);
        let assign50430_e65207: f64 = (1.0 + assign50430_e65206);
        (assign50430_e65207, (0.5 * (((((locals.var_gf__blk1324_dn4 * assign50430_e65202) + (locals.var_gf__blk1324 * (-locals.var_es__blk1369_dn4))) * locals.var_sqs__blk1372) - (assign50430_e65203 * locals.var_sqs__blk1372_dn4)) / (locals.var_sqs__blk1372 * locals.var_sqs__blk1372))), (0.5 * (((((locals.var_gf__blk1324_dn6 * assign50430_e65202) + (locals.var_gf__blk1324 * (-locals.var_es__blk1369_dn6))) * locals.var_sqs__blk1372) - (assign50430_e65203 * locals.var_sqs__blk1372_dn6)) / (locals.var_sqs__blk1372 * locals.var_sqs__blk1372))), (0.5 * (((((locals.var_gf__blk1324_dn7 * assign50430_e65202) + (locals.var_gf__blk1324 * (-locals.var_es__blk1369_dn7))) * locals.var_sqs__blk1372) - (assign50430_e65203 * locals.var_sqs__blk1372_dn7)) / (locals.var_sqs__blk1372 * locals.var_sqs__blk1372))), (0.5 * (((((locals.var_gf__blk1324_dn8 * assign50430_e65202) + (locals.var_gf__blk1324 * (-locals.var_es__blk1369_dn8))) * locals.var_sqs__blk1372) - (assign50430_e65203 * locals.var_sqs__blk1372_dn8)) / (locals.var_sqs__blk1372 * locals.var_sqs__blk1372))), (0.5 * (((((locals.var_gf__blk1324_dn9 * assign50430_e65202) + (locals.var_gf__blk1324 * (-locals.var_es__blk1369_dn9))) * locals.var_sqs__blk1372) - (assign50430_e65203 * locals.var_sqs__blk1372_dn9)) / (locals.var_sqs__blk1372 * locals.var_sqs__blk1372))),)
    } else {
        (locals.var_alphas__blk1373, locals.var_alphas__blk1373_dn4, locals.var_alphas__blk1373_dn6, locals.var_alphas__blk1373_dn7, locals.var_alphas__blk1373_dn8, locals.var_alphas__blk1373_dn9,)
    }
};
        locals.var_alphas__blk1373 = assign50430_e65209;
        locals.var_alphas__blk1373_dn4 = assign50430_e65209_d_n4;
        locals.var_alphas__blk1373_dn6 = assign50430_e65209_d_n6;
        locals.var_alphas__blk1373_dn7 = assign50430_e65209_d_n7;
        locals.var_alphas__blk1373_dn8 = assign50430_e65209_d_n8;
        locals.var_alphas__blk1373_dn9 = assign50430_e65209_d_n9;

        let (assign50440_e65229, assign50440_e65229_d_n4, assign50440_e65229_d_n6, assign50440_e65229_d_n7, assign50440_e65229_d_n8, assign50440_e65229_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign50440_e65218: f64 = (0.2 * locals.var_xcor_t);
        let assign50440_e65220: f64 = (assign50440_e65218 * locals.var_vsbx__blk1323);
        let assign50440_e65221: f64 = (1.0 + assign50440_e65220);
        let assign50440_e65225: f64 = (locals.var_xcor_t * locals.var_vsbx__blk1323);
        let assign50440_e65226: f64 = (1.0 + assign50440_e65225);
        let assign50440_e65227: f64 = (assign50440_e65221 / assign50440_e65226);
        (assign50440_e65227, ((((((0.2 * locals.var_xcor_t_dn4) * locals.var_vsbx__blk1323) + (assign50440_e65218 * locals.var_vsbx__blk1323_dn4)) * assign50440_e65226) - (assign50440_e65221 * ((locals.var_xcor_t_dn4 * locals.var_vsbx__blk1323) + (locals.var_xcor_t * locals.var_vsbx__blk1323_dn4)))) / (assign50440_e65226 * assign50440_e65226)), ((((assign50440_e65218 * locals.var_vsbx__blk1323_dn6) * assign50440_e65226) - (assign50440_e65221 * (locals.var_xcor_t * locals.var_vsbx__blk1323_dn6))) / (assign50440_e65226 * assign50440_e65226)), ((((assign50440_e65218 * locals.var_vsbx__blk1323_dn7) * assign50440_e65226) - (assign50440_e65221 * (locals.var_xcor_t * locals.var_vsbx__blk1323_dn7))) / (assign50440_e65226 * assign50440_e65226)), ((((assign50440_e65218 * locals.var_vsbx__blk1323_dn8) * assign50440_e65226) - (assign50440_e65221 * (locals.var_xcor_t * locals.var_vsbx__blk1323_dn8))) / (assign50440_e65226 * assign50440_e65226)), ((((assign50440_e65218 * locals.var_vsbx__blk1323_dn9) * assign50440_e65226) - (assign50440_e65221 * (locals.var_xcor_t * locals.var_vsbx__blk1323_dn9))) / (assign50440_e65226 * assign50440_e65226)),)
    } else {
        (locals.var_rxcor__blk1374, locals.var_rxcor__blk1374_dn4, locals.var_rxcor__blk1374_dn6, locals.var_rxcor__blk1374_dn7, locals.var_rxcor__blk1374_dn8, locals.var_rxcor__blk1374_dn9,)
    }
};
        locals.var_rxcor__blk1374 = assign50440_e65229;
        locals.var_rxcor__blk1374_dn4 = assign50440_e65229_d_n4;
        locals.var_rxcor__blk1374_dn6 = assign50440_e65229_d_n6;
        locals.var_rxcor__blk1374_dn7 = assign50440_e65229_d_n7;
        locals.var_rxcor__blk1374_dn8 = assign50440_e65229_d_n8;
        locals.var_rxcor__blk1374_dn9 = assign50440_e65229_d_n9;

        let assign50450_e65232: f64 = if locals.var_ds__blk1370 > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1495 = assign50450_e65232;

        let (assign50460_e65247, assign50460_e65247_d_n4, assign50460_e65247_d_n6, assign50460_e65247_d_n7, assign50460_e65247_d_n8, assign50460_e65247_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50460_e65243: f64 = (locals.var_ps__blk1371 + locals.var_ds__blk1370);
        let assign50460_e65244: f64 = (assign50460_e65243).sqrt();
        let assign50460_e65245: f64 = (locals.var_gf__blk1324 * assign50460_e65244);
        (assign50460_e65245, ((locals.var_gf__blk1324_dn4 * assign50460_e65244) + (locals.var_gf__blk1324 * ((locals.var_ps__blk1371_dn4 + locals.var_ds__blk1370_dn4) / (2.0 * assign50460_e65244)))), ((locals.var_gf__blk1324_dn6 * assign50460_e65244) + (locals.var_gf__blk1324 * ((locals.var_ps__blk1371_dn6 + locals.var_ds__blk1370_dn6) / (2.0 * assign50460_e65244)))), ((locals.var_gf__blk1324_dn7 * assign50460_e65244) + (locals.var_gf__blk1324 * ((locals.var_ps__blk1371_dn7 + locals.var_ds__blk1370_dn7) / (2.0 * assign50460_e65244)))), ((locals.var_gf__blk1324_dn8 * assign50460_e65244) + (locals.var_gf__blk1324 * ((locals.var_ps__blk1371_dn8 + locals.var_ds__blk1370_dn8) / (2.0 * assign50460_e65244)))), ((locals.var_gf__blk1324_dn9 * assign50460_e65244) + (locals.var_gf__blk1324 * ((locals.var_ps__blk1371_dn9 + locals.var_ds__blk1370_dn9) / (2.0 * assign50460_e65244)))),)
    } else {
        (locals.var_xgs__blk1375, locals.var_xgs__blk1375_dn4, locals.var_xgs__blk1375_dn6, locals.var_xgs__blk1375_dn7, locals.var_xgs__blk1375_dn8, locals.var_xgs__blk1375_dn9,)
    }
};
        locals.var_xgs__blk1375 = assign50460_e65247;
        locals.var_xgs__blk1375_dn4 = assign50460_e65247_d_n4;
        locals.var_xgs__blk1375_dn6 = assign50460_e65247_d_n6;
        locals.var_xgs__blk1375_dn7 = assign50460_e65247_d_n7;
        locals.var_xgs__blk1375_dn8 = assign50460_e65247_d_n8;
        locals.var_xgs__blk1375_dn9 = assign50460_e65247_d_n9;

        let (assign50470_e65267, assign50470_e65267_d_n4, assign50470_e65267_d_n6, assign50470_e65267_d_n7, assign50470_e65267_d_n8, assign50470_e65267_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50470_e65257: f64 = (locals.var_gf2__blk1325 * locals.var_ds__blk1370);
        let assign50470_e65259: f64 = (assign50470_e65257 * locals.var_phit1__blk1339);
        let assign50470_e65263: f64 = (locals.var_gf__blk1324 * locals.var_sqs__blk1372);
        let assign50470_e65264: f64 = (locals.var_xgs__blk1375 + assign50470_e65263);
        let assign50470_e65265: f64 = (assign50470_e65259 / assign50470_e65264);
        (assign50470_e65265, (((((((locals.var_gf2__blk1325_dn4 * locals.var_ds__blk1370) + (locals.var_gf2__blk1325 * locals.var_ds__blk1370_dn4)) * locals.var_phit1__blk1339) + (assign50470_e65257 * locals.var_phit1__blk1339_dn4)) * assign50470_e65264) - (assign50470_e65259 * (locals.var_xgs__blk1375_dn4 + ((locals.var_gf__blk1324_dn4 * locals.var_sqs__blk1372) + (locals.var_gf__blk1324 * locals.var_sqs__blk1372_dn4))))) / (assign50470_e65264 * assign50470_e65264)), (((((((locals.var_gf2__blk1325_dn6 * locals.var_ds__blk1370) + (locals.var_gf2__blk1325 * locals.var_ds__blk1370_dn6)) * locals.var_phit1__blk1339) + (assign50470_e65257 * locals.var_phit1__blk1339_dn6)) * assign50470_e65264) - (assign50470_e65259 * (locals.var_xgs__blk1375_dn6 + ((locals.var_gf__blk1324_dn6 * locals.var_sqs__blk1372) + (locals.var_gf__blk1324 * locals.var_sqs__blk1372_dn6))))) / (assign50470_e65264 * assign50470_e65264)), (((((((locals.var_gf2__blk1325_dn7 * locals.var_ds__blk1370) + (locals.var_gf2__blk1325 * locals.var_ds__blk1370_dn7)) * locals.var_phit1__blk1339) + (assign50470_e65257 * locals.var_phit1__blk1339_dn7)) * assign50470_e65264) - (assign50470_e65259 * (locals.var_xgs__blk1375_dn7 + ((locals.var_gf__blk1324_dn7 * locals.var_sqs__blk1372) + (locals.var_gf__blk1324 * locals.var_sqs__blk1372_dn7))))) / (assign50470_e65264 * assign50470_e65264)), (((((((locals.var_gf2__blk1325_dn8 * locals.var_ds__blk1370) + (locals.var_gf2__blk1325 * locals.var_ds__blk1370_dn8)) * locals.var_phit1__blk1339) + (assign50470_e65257 * locals.var_phit1__blk1339_dn8)) * assign50470_e65264) - (assign50470_e65259 * (locals.var_xgs__blk1375_dn8 + ((locals.var_gf__blk1324_dn8 * locals.var_sqs__blk1372) + (locals.var_gf__blk1324 * locals.var_sqs__blk1372_dn8))))) / (assign50470_e65264 * assign50470_e65264)), (((((((locals.var_gf2__blk1325_dn9 * locals.var_ds__blk1370) + (locals.var_gf2__blk1325 * locals.var_ds__blk1370_dn9)) * locals.var_phit1__blk1339) + (assign50470_e65257 * locals.var_phit1__blk1339_dn9)) * assign50470_e65264) - (assign50470_e65259 * (locals.var_xgs__blk1375_dn9 + ((locals.var_gf__blk1324_dn9 * locals.var_sqs__blk1372) + (locals.var_gf__blk1324 * locals.var_sqs__blk1372_dn9))))) / (assign50470_e65264 * assign50470_e65264)),)
    } else {
        (locals.var_qis__blk1376, locals.var_qis__blk1376_dn4, locals.var_qis__blk1376_dn6, locals.var_qis__blk1376_dn7, locals.var_qis__blk1376_dn8, locals.var_qis__blk1376_dn9,)
    }
};
        locals.var_qis__blk1376 = assign50470_e65267;
        locals.var_qis__blk1376_dn4 = assign50470_e65267_d_n4;
        locals.var_qis__blk1376_dn6 = assign50470_e65267_d_n6;
        locals.var_qis__blk1376_dn7 = assign50470_e65267_d_n7;
        locals.var_qis__blk1376_dn8 = assign50470_e65267_d_n8;
        locals.var_qis__blk1376_dn9 = assign50470_e65267_d_n9;

        let (assign50480_e65281, assign50480_e65281_d_n4, assign50480_e65281_d_n6, assign50480_e65281_d_n7, assign50480_e65281_d_n8, assign50480_e65281_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50480_e65277: f64 = (locals.var_sqs__blk1372 * locals.var_gf__blk1324);
        let assign50480_e65279: f64 = (assign50480_e65277 * locals.var_phit1__blk1339);
        (assign50480_e65279, ((((locals.var_sqs__blk1372_dn4 * locals.var_gf__blk1324) + (locals.var_sqs__blk1372 * locals.var_gf__blk1324_dn4)) * locals.var_phit1__blk1339) + (assign50480_e65277 * locals.var_phit1__blk1339_dn4)), ((((locals.var_sqs__blk1372_dn6 * locals.var_gf__blk1324) + (locals.var_sqs__blk1372 * locals.var_gf__blk1324_dn6)) * locals.var_phit1__blk1339) + (assign50480_e65277 * locals.var_phit1__blk1339_dn6)), ((((locals.var_sqs__blk1372_dn7 * locals.var_gf__blk1324) + (locals.var_sqs__blk1372 * locals.var_gf__blk1324_dn7)) * locals.var_phit1__blk1339) + (assign50480_e65277 * locals.var_phit1__blk1339_dn7)), ((((locals.var_sqs__blk1372_dn8 * locals.var_gf__blk1324) + (locals.var_sqs__blk1372 * locals.var_gf__blk1324_dn8)) * locals.var_phit1__blk1339) + (assign50480_e65277 * locals.var_phit1__blk1339_dn8)), ((((locals.var_sqs__blk1372_dn9 * locals.var_gf__blk1324) + (locals.var_sqs__blk1372 * locals.var_gf__blk1324_dn9)) * locals.var_phit1__blk1339) + (assign50480_e65277 * locals.var_phit1__blk1339_dn9)),)
    } else {
        (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9,)
    }
};
        locals.var_qbs__blk1377 = assign50480_e65281;
        locals.var_qbs__blk1377_dn4 = assign50480_e65281_d_n4;
        locals.var_qbs__blk1377_dn6 = assign50480_e65281_d_n6;
        locals.var_qbs__blk1377_dn7 = assign50480_e65281_d_n7;
        locals.var_qbs__blk1377_dn8 = assign50480_e65281_d_n8;
        locals.var_qbs__blk1377_dn9 = assign50480_e65281_d_n9;

    }

    pub(super) fn stamp_transient_block_42(
        locals: &mut StampLocals,
    ) {
        let assign50490_e65284: f64 = if locals.var_rsb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1496 = assign50490_e65284;

        let (assign50500_e65302, assign50500_e65302_d_n4, assign50500_e65302_d_n6, assign50500_e65302_d_n7, assign50500_e65302_d_n8, assign50500_e65302_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1496 != 0.0)) {
        let assign50500_e65298: f64 = (locals.var_rsb_i * locals.var_vsbx__blk1323);
        let assign50500_e65299: f64 = (1.0 - assign50500_e65298);
        let assign50500_e65300: f64 = (1.0 / assign50500_e65299);
        (assign50500_e65300, (-((-(locals.var_rsb_i * locals.var_vsbx__blk1323_dn4)) / (assign50500_e65299 * assign50500_e65299))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1323_dn6)) / (assign50500_e65299 * assign50500_e65299))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1323_dn7)) / (assign50500_e65299 * assign50500_e65299))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1323_dn8)) / (assign50500_e65299 * assign50500_e65299))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1323_dn9)) / (assign50500_e65299 * assign50500_e65299))),)
    } else {
        (locals.var_rhob__blk1378, locals.var_rhob__blk1378_dn4, locals.var_rhob__blk1378_dn6, locals.var_rhob__blk1378_dn7, locals.var_rhob__blk1378_dn8, locals.var_rhob__blk1378_dn9,)
    }
};
        locals.var_rhob__blk1378 = assign50500_e65302;
        locals.var_rhob__blk1378_dn4 = assign50500_e65302_d_n4;
        locals.var_rhob__blk1378_dn6 = assign50500_e65302_d_n6;
        locals.var_rhob__blk1378_dn7 = assign50500_e65302_d_n7;
        locals.var_rhob__blk1378_dn8 = assign50500_e65302_d_n8;
        locals.var_rhob__blk1378_dn9 = assign50500_e65302_d_n9;

        let (assign50510_e65319, assign50510_e65319_d_n4, assign50510_e65319_d_n6, assign50510_e65319_d_n7, assign50510_e65319_d_n8, assign50510_e65319_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1496 == 0.0)) {
        let assign50510_e65316: f64 = (locals.var_rsb_i * locals.var_vsbx__blk1323);
        let assign50510_e65317: f64 = (1.0 + assign50510_e65316);
        (assign50510_e65317, (locals.var_rsb_i * locals.var_vsbx__blk1323_dn4), (locals.var_rsb_i * locals.var_vsbx__blk1323_dn6), (locals.var_rsb_i * locals.var_vsbx__blk1323_dn7), (locals.var_rsb_i * locals.var_vsbx__blk1323_dn8), (locals.var_rsb_i * locals.var_vsbx__blk1323_dn9),)
    } else {
        (locals.var_rhob__blk1378, locals.var_rhob__blk1378_dn4, locals.var_rhob__blk1378_dn6, locals.var_rhob__blk1378_dn7, locals.var_rhob__blk1378_dn8, locals.var_rhob__blk1378_dn9,)
    }
};
        locals.var_rhob__blk1378 = assign50510_e65319;
        locals.var_rhob__blk1378_dn4 = assign50510_e65319_d_n4;
        locals.var_rhob__blk1378_dn6 = assign50510_e65319_d_n6;
        locals.var_rhob__blk1378_dn7 = assign50510_e65319_d_n7;
        locals.var_rhob__blk1378_dn8 = assign50510_e65319_d_n8;
        locals.var_rhob__blk1378_dn9 = assign50510_e65319_d_n9;

        let assign50520_e65322: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1497 = assign50520_e65322;

        let (assign50530_e65338, assign50530_e65338_d_n4, assign50530_e65338_d_n6, assign50530_e65338_d_n7, assign50530_e65338_d_n8, assign50530_e65338_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign50530_e65335: f64 = (locals.var_rsg_i * locals.var_qis__blk1376);
        let assign50530_e65336: f64 = (1.0 - assign50530_e65335);
        (assign50530_e65336, (-(locals.var_rsg_i * locals.var_qis__blk1376_dn4)), (-(locals.var_rsg_i * locals.var_qis__blk1376_dn6)), (-(locals.var_rsg_i * locals.var_qis__blk1376_dn7)), (-(locals.var_rsg_i * locals.var_qis__blk1376_dn8)), (-(locals.var_rsg_i * locals.var_qis__blk1376_dn9)),)
    } else {
        (locals.var_rhog__blk1379, locals.var_rhog__blk1379_dn4, locals.var_rhog__blk1379_dn6, locals.var_rhog__blk1379_dn7, locals.var_rhog__blk1379_dn8, locals.var_rhog__blk1379_dn9,)
    }
};
        locals.var_rhog__blk1379 = assign50530_e65338;
        locals.var_rhog__blk1379_dn4 = assign50530_e65338_d_n4;
        locals.var_rhog__blk1379_dn6 = assign50530_e65338_d_n6;
        locals.var_rhog__blk1379_dn7 = assign50530_e65338_d_n7;
        locals.var_rhog__blk1379_dn8 = assign50530_e65338_d_n8;
        locals.var_rhog__blk1379_dn9 = assign50530_e65338_d_n9;

        let (assign50540_e65357, assign50540_e65357_d_n4, assign50540_e65357_d_n6, assign50540_e65357_d_n7, assign50540_e65357_d_n8, assign50540_e65357_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1497 == 0.0)) {
        let assign50540_e65353: f64 = (locals.var_rsg_i * locals.var_qis__blk1376);
        let assign50540_e65354: f64 = (1.0 + assign50540_e65353);
        let assign50540_e65355: f64 = (1.0 / assign50540_e65354);
        (assign50540_e65355, (-((locals.var_rsg_i * locals.var_qis__blk1376_dn4) / (assign50540_e65354 * assign50540_e65354))), (-((locals.var_rsg_i * locals.var_qis__blk1376_dn6) / (assign50540_e65354 * assign50540_e65354))), (-((locals.var_rsg_i * locals.var_qis__blk1376_dn7) / (assign50540_e65354 * assign50540_e65354))), (-((locals.var_rsg_i * locals.var_qis__blk1376_dn8) / (assign50540_e65354 * assign50540_e65354))), (-((locals.var_rsg_i * locals.var_qis__blk1376_dn9) / (assign50540_e65354 * assign50540_e65354))),)
    } else {
        (locals.var_rhog__blk1379, locals.var_rhog__blk1379_dn4, locals.var_rhog__blk1379_dn6, locals.var_rhog__blk1379_dn7, locals.var_rhog__blk1379_dn8, locals.var_rhog__blk1379_dn9,)
    }
};
        locals.var_rhog__blk1379 = assign50540_e65357;
        locals.var_rhog__blk1379_dn4 = assign50540_e65357_d_n4;
        locals.var_rhog__blk1379_dn6 = assign50540_e65357_d_n6;
        locals.var_rhog__blk1379_dn7 = assign50540_e65357_d_n7;
        locals.var_rhog__blk1379_dn8 = assign50540_e65357_d_n8;
        locals.var_rhog__blk1379_dn9 = assign50540_e65357_d_n9;

        let (assign50550_e65373, assign50550_e65373_d_n4, assign50550_e65373_d_n6, assign50550_e65373_d_n7, assign50550_e65373_d_n8, assign50550_e65373_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50550_e65367: f64 = (locals.var_ther_i * locals.var_rhob__blk1378);
        let assign50550_e65369: f64 = (assign50550_e65367 * locals.var_rhog__blk1379);
        let assign50550_e65371: f64 = (assign50550_e65369 * locals.var_qis__blk1376);
        (assign50550_e65371, ((((((locals.var_ther_i_dn4 * locals.var_rhob__blk1378) + (locals.var_ther_i * locals.var_rhob__blk1378_dn4)) * locals.var_rhog__blk1379) + (assign50550_e65367 * locals.var_rhog__blk1379_dn4)) * locals.var_qis__blk1376) + (assign50550_e65369 * locals.var_qis__blk1376_dn4)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn6) * locals.var_rhog__blk1379) + (assign50550_e65367 * locals.var_rhog__blk1379_dn6)) * locals.var_qis__blk1376) + (assign50550_e65369 * locals.var_qis__blk1376_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn7) * locals.var_rhog__blk1379) + (assign50550_e65367 * locals.var_rhog__blk1379_dn7)) * locals.var_qis__blk1376) + (assign50550_e65369 * locals.var_qis__blk1376_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn8) * locals.var_rhog__blk1379) + (assign50550_e65367 * locals.var_rhog__blk1379_dn8)) * locals.var_qis__blk1376) + (assign50550_e65369 * locals.var_qis__blk1376_dn8)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn9) * locals.var_rhog__blk1379) + (assign50550_e65367 * locals.var_rhog__blk1379_dn9)) * locals.var_qis__blk1376) + (assign50550_e65369 * locals.var_qis__blk1376_dn9)),)
    } else {
        (locals.var_gr__blk1380, locals.var_gr__blk1380_dn4, locals.var_gr__blk1380_dn6, locals.var_gr__blk1380_dn7, locals.var_gr__blk1380_dn8, locals.var_gr__blk1380_dn9,)
    }
};
        locals.var_gr__blk1380 = assign50550_e65373;
        locals.var_gr__blk1380_dn4 = assign50550_e65373_d_n4;
        locals.var_gr__blk1380_dn6 = assign50550_e65373_d_n6;
        locals.var_gr__blk1380_dn7 = assign50550_e65373_d_n7;
        locals.var_gr__blk1380_dn8 = assign50550_e65373_d_n8;
        locals.var_gr__blk1380_dn9 = assign50550_e65373_d_n9;

        let (assign50560_e65389, assign50560_e65389_d_n4, assign50560_e65389_d_n6, assign50560_e65389_d_n7, assign50560_e65389_d_n8, assign50560_e65389_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50560_e65385: f64 = (locals.var_eta_mu * locals.var_qis__blk1376);
        let assign50560_e65386: f64 = (locals.var_qbs__blk1377 + assign50560_e65385);
        let assign50560_e65387: f64 = (locals.var_e_eff0 * assign50560_e65386);
        (assign50560_e65387, (locals.var_e_eff0 * (locals.var_qbs__blk1377_dn4 + (locals.var_eta_mu * locals.var_qis__blk1376_dn4))), (locals.var_e_eff0 * (locals.var_qbs__blk1377_dn6 + (locals.var_eta_mu * locals.var_qis__blk1376_dn6))), (locals.var_e_eff0 * (locals.var_qbs__blk1377_dn7 + (locals.var_eta_mu * locals.var_qis__blk1376_dn7))), (locals.var_e_eff0 * (locals.var_qbs__blk1377_dn8 + (locals.var_eta_mu * locals.var_qis__blk1376_dn8))), (locals.var_e_eff0 * (locals.var_qbs__blk1377_dn9 + (locals.var_eta_mu * locals.var_qis__blk1376_dn9))),)
    } else {
        (locals.var_eeffs__blk1381, locals.var_eeffs__blk1381_dn4, locals.var_eeffs__blk1381_dn6, locals.var_eeffs__blk1381_dn7, locals.var_eeffs__blk1381_dn8, locals.var_eeffs__blk1381_dn9,)
    }
};
        locals.var_eeffs__blk1381 = assign50560_e65389;
        locals.var_eeffs__blk1381_dn4 = assign50560_e65389_d_n4;
        locals.var_eeffs__blk1381_dn6 = assign50560_e65389_d_n6;
        locals.var_eeffs__blk1381_dn7 = assign50560_e65389_d_n7;
        locals.var_eeffs__blk1381_dn8 = assign50560_e65389_d_n8;
        locals.var_eeffs__blk1381_dn9 = assign50560_e65389_d_n9;

        let (assign50570_e65406, assign50570_e65406_d_n4, assign50570_e65406_d_n6, assign50570_e65406_d_n7, assign50570_e65406_d_n8, assign50570_e65406_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50570_e65400: f64 = (locals.var_ps__blk1371 + locals.var_ds__blk1370);
        let assign50570_e65402: f64 = (assign50570_e65400 + 1e-14);
        let assign50570_e65403: f64 = (locals.var_ps__blk1371 / assign50570_e65402);
        let assign50570_e65404: f64 = (assign50570_e65403).ln();
        (assign50570_e65404, ((((locals.var_ps__blk1371_dn4 * assign50570_e65402) - (locals.var_ps__blk1371 * (locals.var_ps__blk1371_dn4 + locals.var_ds__blk1370_dn4))) / (assign50570_e65402 * assign50570_e65402)) / assign50570_e65403), ((((locals.var_ps__blk1371_dn6 * assign50570_e65402) - (locals.var_ps__blk1371 * (locals.var_ps__blk1371_dn6 + locals.var_ds__blk1370_dn6))) / (assign50570_e65402 * assign50570_e65402)) / assign50570_e65403), ((((locals.var_ps__blk1371_dn7 * assign50570_e65402) - (locals.var_ps__blk1371 * (locals.var_ps__blk1371_dn7 + locals.var_ds__blk1370_dn7))) / (assign50570_e65402 * assign50570_e65402)) / assign50570_e65403), ((((locals.var_ps__blk1371_dn8 * assign50570_e65402) - (locals.var_ps__blk1371 * (locals.var_ps__blk1371_dn8 + locals.var_ds__blk1370_dn8))) / (assign50570_e65402 * assign50570_e65402)) / assign50570_e65403), ((((locals.var_ps__blk1371_dn9 * assign50570_e65402) - (locals.var_ps__blk1371 * (locals.var_ps__blk1371_dn9 + locals.var_ds__blk1370_dn9))) / (assign50570_e65402 * assign50570_e65402)) / assign50570_e65403),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign50570_e65406;
        locals.var_temp1_dn4 = assign50570_e65406_d_n4;
        locals.var_temp1_dn6 = assign50570_e65406_d_n6;
        locals.var_temp1_dn7 = assign50570_e65406_d_n7;
        locals.var_temp1_dn8 = assign50570_e65406_d_n8;
        locals.var_temp1_dn9 = assign50570_e65406_d_n9;

        let (assign50580_e65429, assign50580_e65429_d_n4, assign50580_e65429_d_n6, assign50580_e65429_d_n7, assign50580_e65429_d_n8, assign50580_e65429_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50580_e65416: f64 = (locals.var_eeffs__blk1381 * locals.var_mue_t);
        let assign50580_e65418: f64 = (assign50580_e65416).powf(locals.var_themu_t);
        let assign50580_e65422: f64 = (0.5 * locals.var_thecs_t);
        let assign50580_e65424: f64 = (assign50580_e65422 * locals.var_temp1);
        let assign50580_e65425: f64 = (assign50580_e65424).exp();
        let assign50580_e65426: f64 = (locals.var_cs_t * assign50580_e65425);
        let assign50580_e65427: f64 = (assign50580_e65418 + assign50580_e65426);
        (assign50580_e65427, (if locals.var_themu_t_dn4 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50580_e65416).powf(locals.var_themu_t - 1.0) * ((locals.var_eeffs__blk1381_dn4 * locals.var_mue_t) + (locals.var_eeffs__blk1381 * locals.var_mue_t_dn4)))) } } else { (assign50580_e65418 * ((locals.var_themu_t_dn4 * (assign50580_e65416).ln()) + (locals.var_themu_t * (((locals.var_eeffs__blk1381_dn4 * locals.var_mue_t) + (locals.var_eeffs__blk1381 * locals.var_mue_t_dn4)) / assign50580_e65416)))) } + ((locals.var_cs_t_dn4 * assign50580_e65425) + (locals.var_cs_t * (assign50580_e65425 * (((0.5 * locals.var_thecs_t_dn4) * locals.var_temp1) + (assign50580_e65422 * locals.var_temp1_dn4)))))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50580_e65416).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1381_dn6 * locals.var_mue_t))) } } else { (assign50580_e65418 * (locals.var_themu_t * ((locals.var_eeffs__blk1381_dn6 * locals.var_mue_t) / assign50580_e65416))) } + (locals.var_cs_t * (assign50580_e65425 * (assign50580_e65422 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50580_e65416).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1381_dn7 * locals.var_mue_t))) } } else { (assign50580_e65418 * (locals.var_themu_t * ((locals.var_eeffs__blk1381_dn7 * locals.var_mue_t) / assign50580_e65416))) } + (locals.var_cs_t * (assign50580_e65425 * (assign50580_e65422 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50580_e65416).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1381_dn8 * locals.var_mue_t))) } } else { (assign50580_e65418 * (locals.var_themu_t * ((locals.var_eeffs__blk1381_dn8 * locals.var_mue_t) / assign50580_e65416))) } + (locals.var_cs_t * (assign50580_e65425 * (assign50580_e65422 * locals.var_temp1_dn8)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50580_e65416).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1381_dn9 * locals.var_mue_t))) } } else { (assign50580_e65418 * (locals.var_themu_t * ((locals.var_eeffs__blk1381_dn9 * locals.var_mue_t) / assign50580_e65416))) } + (locals.var_cs_t * (assign50580_e65425 * (assign50580_e65422 * locals.var_temp1_dn9)))),)
    } else {
        (locals.var_mutmp__blk1382, locals.var_mutmp__blk1382_dn4, locals.var_mutmp__blk1382_dn6, locals.var_mutmp__blk1382_dn7, locals.var_mutmp__blk1382_dn8, locals.var_mutmp__blk1382_dn9,)
    }
};
        locals.var_mutmp__blk1382 = assign50580_e65429;
        locals.var_mutmp__blk1382_dn4 = assign50580_e65429_d_n4;
        locals.var_mutmp__blk1382_dn6 = assign50580_e65429_d_n6;
        locals.var_mutmp__blk1382_dn7 = assign50580_e65429_d_n7;
        locals.var_mutmp__blk1382_dn8 = assign50580_e65429_d_n8;
        locals.var_mutmp__blk1382_dn9 = assign50580_e65429_d_n9;

        let (assign50590_e65445, assign50590_e65445_d_n4, assign50590_e65445_d_n6, assign50590_e65445_d_n7, assign50590_e65445_d_n8, assign50590_e65445_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50590_e65439: f64 = (1.0 + locals.var_mutmp__blk1382);
        let assign50590_e65441: f64 = (assign50590_e65439 + locals.var_gr__blk1380);
        let assign50590_e65443: f64 = (assign50590_e65441 * locals.var_rxcor__blk1374);
        (assign50590_e65443, (((locals.var_mutmp__blk1382_dn4 + locals.var_gr__blk1380_dn4) * locals.var_rxcor__blk1374) + (assign50590_e65441 * locals.var_rxcor__blk1374_dn4)), (((locals.var_mutmp__blk1382_dn6 + locals.var_gr__blk1380_dn6) * locals.var_rxcor__blk1374) + (assign50590_e65441 * locals.var_rxcor__blk1374_dn6)), (((locals.var_mutmp__blk1382_dn7 + locals.var_gr__blk1380_dn7) * locals.var_rxcor__blk1374) + (assign50590_e65441 * locals.var_rxcor__blk1374_dn7)), (((locals.var_mutmp__blk1382_dn8 + locals.var_gr__blk1380_dn8) * locals.var_rxcor__blk1374) + (assign50590_e65441 * locals.var_rxcor__blk1374_dn8)), (((locals.var_mutmp__blk1382_dn9 + locals.var_gr__blk1380_dn9) * locals.var_rxcor__blk1374) + (assign50590_e65441 * locals.var_rxcor__blk1374_dn9)),)
    } else {
        (locals.var_gmobs__blk1383, locals.var_gmobs__blk1383_dn4, locals.var_gmobs__blk1383_dn6, locals.var_gmobs__blk1383_dn7, locals.var_gmobs__blk1383_dn8, locals.var_gmobs__blk1383_dn9,)
    }
};
        locals.var_gmobs__blk1383 = assign50590_e65445;
        locals.var_gmobs__blk1383_dn4 = assign50590_e65445_d_n4;
        locals.var_gmobs__blk1383_dn6 = assign50590_e65445_d_n6;
        locals.var_gmobs__blk1383_dn7 = assign50590_e65445_d_n7;
        locals.var_gmobs__blk1383_dn8 = assign50590_e65445_d_n8;
        locals.var_gmobs__blk1383_dn9 = assign50590_e65445_d_n9;

        let assign50600_e65448: f64 = if locals.var_thesatb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1498 = assign50600_e65448;

        let (assign50610_e65466, assign50610_e65466_d_n4, assign50610_e65466_d_n6, assign50610_e65466_d_n7, assign50610_e65466_d_n8, assign50610_e65466_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1498 != 0.0)) {
        let assign50610_e65462: f64 = (locals.var_thesatb_i * locals.var_vsbx__blk1323);
        let assign50610_e65463: f64 = (1.0 - assign50610_e65462);
        let assign50610_e65464: f64 = (1.0 / assign50610_e65463);
        (assign50610_e65464, (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1323_dn4)) / (assign50610_e65463 * assign50610_e65463))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1323_dn6)) / (assign50610_e65463 * assign50610_e65463))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1323_dn7)) / (assign50610_e65463 * assign50610_e65463))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1323_dn8)) / (assign50610_e65463 * assign50610_e65463))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1323_dn9)) / (assign50610_e65463 * assign50610_e65463))),)
    } else {
        (locals.var_xitsb__blk1384, locals.var_xitsb__blk1384_dn4, locals.var_xitsb__blk1384_dn6, locals.var_xitsb__blk1384_dn7, locals.var_xitsb__blk1384_dn8, locals.var_xitsb__blk1384_dn9,)
    }
};
        locals.var_xitsb__blk1384 = assign50610_e65466;
        locals.var_xitsb__blk1384_dn4 = assign50610_e65466_d_n4;
        locals.var_xitsb__blk1384_dn6 = assign50610_e65466_d_n6;
        locals.var_xitsb__blk1384_dn7 = assign50610_e65466_d_n7;
        locals.var_xitsb__blk1384_dn8 = assign50610_e65466_d_n8;
        locals.var_xitsb__blk1384_dn9 = assign50610_e65466_d_n9;

        let (assign50620_e65483, assign50620_e65483_d_n4, assign50620_e65483_d_n6, assign50620_e65483_d_n7, assign50620_e65483_d_n8, assign50620_e65483_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1498 == 0.0)) {
        let assign50620_e65480: f64 = (locals.var_thesatb_i * locals.var_vsbx__blk1323);
        let assign50620_e65481: f64 = (1.0 + assign50620_e65480);
        (assign50620_e65481, (locals.var_thesatb_i * locals.var_vsbx__blk1323_dn4), (locals.var_thesatb_i * locals.var_vsbx__blk1323_dn6), (locals.var_thesatb_i * locals.var_vsbx__blk1323_dn7), (locals.var_thesatb_i * locals.var_vsbx__blk1323_dn8), (locals.var_thesatb_i * locals.var_vsbx__blk1323_dn9),)
    } else {
        (locals.var_xitsb__blk1384, locals.var_xitsb__blk1384_dn4, locals.var_xitsb__blk1384_dn6, locals.var_xitsb__blk1384_dn7, locals.var_xitsb__blk1384_dn8, locals.var_xitsb__blk1384_dn9,)
    }
};
        locals.var_xitsb__blk1384 = assign50620_e65483;
        locals.var_xitsb__blk1384_dn4 = assign50620_e65483_d_n4;
        locals.var_xitsb__blk1384_dn6 = assign50620_e65483_d_n6;
        locals.var_xitsb__blk1384_dn7 = assign50620_e65483_d_n7;
        locals.var_xitsb__blk1384_dn8 = assign50620_e65483_d_n8;
        locals.var_xitsb__blk1384_dn9 = assign50620_e65483_d_n9;

        let (assign50630_e65495, assign50630_e65495_d_n4, assign50630_e65495_d_n6, assign50630_e65495_d_n7, assign50630_e65495_d_n8, assign50630_e65495_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50630_e65493: f64 = (locals.var_qis__blk1376 * locals.var_xitsb__blk1384);
        (assign50630_e65493, ((locals.var_qis__blk1376_dn4 * locals.var_xitsb__blk1384) + (locals.var_qis__blk1376 * locals.var_xitsb__blk1384_dn4)), ((locals.var_qis__blk1376_dn6 * locals.var_xitsb__blk1384) + (locals.var_qis__blk1376 * locals.var_xitsb__blk1384_dn6)), ((locals.var_qis__blk1376_dn7 * locals.var_xitsb__blk1384) + (locals.var_qis__blk1376 * locals.var_xitsb__blk1384_dn7)), ((locals.var_qis__blk1376_dn8 * locals.var_xitsb__blk1384) + (locals.var_qis__blk1376 * locals.var_xitsb__blk1384_dn8)), ((locals.var_qis__blk1376_dn9 * locals.var_xitsb__blk1384) + (locals.var_qis__blk1376 * locals.var_xitsb__blk1384_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign50630_e65495;
        locals.var_temp2_dn4 = assign50630_e65495_d_n4;
        locals.var_temp2_dn6 = assign50630_e65495_d_n6;
        locals.var_temp2_dn7 = assign50630_e65495_d_n7;
        locals.var_temp2_dn8 = assign50630_e65495_d_n8;
        locals.var_temp2_dn9 = assign50630_e65495_d_n9;

        let (assign50640_e65509, assign50640_e65509_d_n4, assign50640_e65509_d_n6, assign50640_e65509_d_n7, assign50640_e65509_d_n8, assign50640_e65509_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        let assign50640_e65506: f64 = (locals.var_thesatt_i + locals.var_temp2);
        let assign50640_e65507: f64 = (locals.var_temp2 / assign50640_e65506);
        (assign50640_e65507, (((locals.var_temp2_dn4 * assign50640_e65506) - (locals.var_temp2 * locals.var_temp2_dn4)) / (assign50640_e65506 * assign50640_e65506)), (((locals.var_temp2_dn6 * assign50640_e65506) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign50640_e65506 * assign50640_e65506)), (((locals.var_temp2_dn7 * assign50640_e65506) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign50640_e65506 * assign50640_e65506)), (((locals.var_temp2_dn8 * assign50640_e65506) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign50640_e65506 * assign50640_e65506)), (((locals.var_temp2_dn9 * assign50640_e65506) - (locals.var_temp2 * locals.var_temp2_dn9)) / (assign50640_e65506 * assign50640_e65506)),)
    } else {
        (locals.var_wsat__blk1385, locals.var_wsat__blk1385_dn4, locals.var_wsat__blk1385_dn6, locals.var_wsat__blk1385_dn7, locals.var_wsat__blk1385_dn8, locals.var_wsat__blk1385_dn9,)
    }
};
        locals.var_wsat__blk1385 = assign50640_e65509;
        locals.var_wsat__blk1385_dn4 = assign50640_e65509_d_n4;
        locals.var_wsat__blk1385_dn6 = assign50640_e65509_d_n6;
        locals.var_wsat__blk1385_dn7 = assign50640_e65509_d_n7;
        locals.var_wsat__blk1385_dn8 = assign50640_e65509_d_n8;
        locals.var_wsat__blk1385_dn9 = assign50640_e65509_d_n9;

        let assign50650_e65512: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1499 = assign50650_e65512;

        let (assign50660_e65530, assign50660_e65530_d_n4, assign50660_e65530_d_n6, assign50660_e65530_d_n7, assign50660_e65530_d_n8, assign50660_e65530_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign50660_e65526: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1385);
        let assign50660_e65527: f64 = (1.0 - assign50660_e65526);
        let assign50660_e65528: f64 = (1.0 / assign50660_e65527);
        (assign50660_e65528, (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn4)) / (assign50660_e65527 * assign50660_e65527))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn6)) / (assign50660_e65527 * assign50660_e65527))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn7)) / (assign50660_e65527 * assign50660_e65527))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn8)) / (assign50660_e65527 * assign50660_e65527))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn9)) / (assign50660_e65527 * assign50660_e65527))),)
    } else {
        (locals.var_factheta__blk1386, locals.var_factheta__blk1386_dn4, locals.var_factheta__blk1386_dn6, locals.var_factheta__blk1386_dn7, locals.var_factheta__blk1386_dn8, locals.var_factheta__blk1386_dn9,)
    }
};
        locals.var_factheta__blk1386 = assign50660_e65530;
        locals.var_factheta__blk1386_dn4 = assign50660_e65530_d_n4;
        locals.var_factheta__blk1386_dn6 = assign50660_e65530_d_n6;
        locals.var_factheta__blk1386_dn7 = assign50660_e65530_d_n7;
        locals.var_factheta__blk1386_dn8 = assign50660_e65530_d_n8;
        locals.var_factheta__blk1386_dn9 = assign50660_e65530_d_n9;

        let (assign50670_e65547, assign50670_e65547_d_n4, assign50670_e65547_d_n6, assign50670_e65547_d_n7, assign50670_e65547_d_n8, assign50670_e65547_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1499 == 0.0)) {
        let assign50670_e65544: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1385);
        let assign50670_e65545: f64 = (1.0 + assign50670_e65544);
        (assign50670_e65545, (locals.var_thesatg_i * locals.var_wsat__blk1385_dn4), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn6), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn7), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn8), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn9),)
    } else {
        (locals.var_factheta__blk1386, locals.var_factheta__blk1386_dn4, locals.var_factheta__blk1386_dn6, locals.var_factheta__blk1386_dn7, locals.var_factheta__blk1386_dn8, locals.var_factheta__blk1386_dn9,)
    }
};
        locals.var_factheta__blk1386 = assign50670_e65547;
        locals.var_factheta__blk1386_dn4 = assign50670_e65547_d_n4;
        locals.var_factheta__blk1386_dn6 = assign50670_e65547_d_n6;
        locals.var_factheta__blk1386_dn7 = assign50670_e65547_d_n7;
        locals.var_factheta__blk1386_dn8 = assign50670_e65547_d_n8;
        locals.var_factheta__blk1386_dn9 = assign50670_e65547_d_n9;

        let (assign50770_e65646, assign50770_e65646_d_n4, assign50770_e65646_d_n6, assign50770_e65646_d_n7, assign50770_e65646_d_n8, assign50770_e65646_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_vgb1_dc, locals.var_vgb1_dc_dn4, locals.var_vgb1_dc_dn6, locals.var_vgb1_dc_dn7, locals.var_vgb1_dc_dn8, locals.var_vgb1_dc_dn9,)
    } else {
        (locals.var_vgb1__blk1321, locals.var_vgb1__blk1321_dn4, locals.var_vgb1__blk1321_dn6, locals.var_vgb1__blk1321_dn7, locals.var_vgb1__blk1321_dn8, locals.var_vgb1__blk1321_dn9,)
    }
};
        locals.var_vgb1__blk1321 = assign50770_e65646;
        locals.var_vgb1__blk1321_dn4 = assign50770_e65646_d_n4;
        locals.var_vgb1__blk1321_dn6 = assign50770_e65646_d_n6;
        locals.var_vgb1__blk1321_dn7 = assign50770_e65646_d_n7;
        locals.var_vgb1__blk1321_dn8 = assign50770_e65646_d_n8;
        locals.var_vgb1__blk1321_dn9 = assign50770_e65646_d_n9;

        let (assign50780_e65653, assign50780_e65653_d_n4, assign50780_e65653_d_n6, assign50780_e65653_d_n7, assign50780_e65653_d_n8, assign50780_e65653_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_vsbx_dc, locals.var_vsbx_dc_dn4, locals.var_vsbx_dc_dn6, locals.var_vsbx_dc_dn7, locals.var_vsbx_dc_dn8, locals.var_vsbx_dc_dn9,)
    } else {
        (locals.var_vsbx__blk1323, locals.var_vsbx__blk1323_dn4, locals.var_vsbx__blk1323_dn6, locals.var_vsbx__blk1323_dn7, locals.var_vsbx__blk1323_dn8, locals.var_vsbx__blk1323_dn9,)
    }
};
        locals.var_vsbx__blk1323 = assign50780_e65653;
        locals.var_vsbx__blk1323_dn4 = assign50780_e65653_d_n4;
        locals.var_vsbx__blk1323_dn6 = assign50780_e65653_d_n6;
        locals.var_vsbx__blk1323_dn7 = assign50780_e65653_d_n7;
        locals.var_vsbx__blk1323_dn8 = assign50780_e65653_d_n8;
        locals.var_vsbx__blk1323_dn9 = assign50780_e65653_d_n9;

        let (assign50790_e65660, assign50790_e65660_d_n4, assign50790_e65660_d_n6, assign50790_e65660_d_n7, assign50790_e65660_d_n8, assign50790_e65660_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_phit1_dc, locals.var_phit1_dc_dn4, locals.var_phit1_dc_dn6, locals.var_phit1_dc_dn7, locals.var_phit1_dc_dn8, locals.var_phit1_dc_dn9,)
    } else {
        (locals.var_phit1__blk1339, locals.var_phit1__blk1339_dn4, locals.var_phit1__blk1339_dn6, locals.var_phit1__blk1339_dn7, locals.var_phit1__blk1339_dn8, locals.var_phit1__blk1339_dn9,)
    }
};
        locals.var_phit1__blk1339 = assign50790_e65660;
        locals.var_phit1__blk1339_dn4 = assign50790_e65660_d_n4;
        locals.var_phit1__blk1339_dn6 = assign50790_e65660_d_n6;
        locals.var_phit1__blk1339_dn7 = assign50790_e65660_d_n7;
        locals.var_phit1__blk1339_dn8 = assign50790_e65660_d_n8;
        locals.var_phit1__blk1339_dn9 = assign50790_e65660_d_n9;

        let (assign50800_e65667, assign50800_e65667_d_n4, assign50800_e65667_d_n6, assign50800_e65667_d_n7, assign50800_e65667_d_n8, assign50800_e65667_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_inv_phit1_dc, locals.var_inv_phit1_dc_dn4, locals.var_inv_phit1_dc_dn6, locals.var_inv_phit1_dc_dn7, locals.var_inv_phit1_dc_dn8, locals.var_inv_phit1_dc_dn9,)
    } else {
        (locals.var_inv_phit1__blk1340, locals.var_inv_phit1__blk1340_dn4, locals.var_inv_phit1__blk1340_dn6, locals.var_inv_phit1__blk1340_dn7, locals.var_inv_phit1__blk1340_dn8, locals.var_inv_phit1__blk1340_dn9,)
    }
};
        locals.var_inv_phit1__blk1340 = assign50800_e65667;
        locals.var_inv_phit1__blk1340_dn4 = assign50800_e65667_d_n4;
        locals.var_inv_phit1__blk1340_dn6 = assign50800_e65667_d_n6;
        locals.var_inv_phit1__blk1340_dn7 = assign50800_e65667_d_n7;
        locals.var_inv_phit1__blk1340_dn8 = assign50800_e65667_d_n8;
        locals.var_inv_phit1__blk1340_dn9 = assign50800_e65667_d_n9;

        let (assign50810_e65674, assign50810_e65674_d_n4, assign50810_e65674_d_n6, assign50810_e65674_d_n7, assign50810_e65674_d_n8, assign50810_e65674_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_gf_dc, locals.var_gf_dc_dn4, locals.var_gf_dc_dn6, locals.var_gf_dc_dn7, locals.var_gf_dc_dn8, locals.var_gf_dc_dn9,)
    } else {
        (locals.var_gf__blk1324, locals.var_gf__blk1324_dn4, locals.var_gf__blk1324_dn6, locals.var_gf__blk1324_dn7, locals.var_gf__blk1324_dn8, locals.var_gf__blk1324_dn9,)
    }
};
        locals.var_gf__blk1324 = assign50810_e65674;
        locals.var_gf__blk1324_dn4 = assign50810_e65674_d_n4;
        locals.var_gf__blk1324_dn6 = assign50810_e65674_d_n6;
        locals.var_gf__blk1324_dn7 = assign50810_e65674_d_n7;
        locals.var_gf__blk1324_dn8 = assign50810_e65674_d_n8;
        locals.var_gf__blk1324_dn9 = assign50810_e65674_d_n9;

        let (assign50820_e65681, assign50820_e65681_d_n4, assign50820_e65681_d_n6, assign50820_e65681_d_n7, assign50820_e65681_d_n8, assign50820_e65681_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_gf2_dc, locals.var_gf2_dc_dn4, locals.var_gf2_dc_dn6, locals.var_gf2_dc_dn7, locals.var_gf2_dc_dn8, locals.var_gf2_dc_dn9,)
    } else {
        (locals.var_gf2__blk1325, locals.var_gf2__blk1325_dn4, locals.var_gf2__blk1325_dn6, locals.var_gf2__blk1325_dn7, locals.var_gf2__blk1325_dn8, locals.var_gf2__blk1325_dn9,)
    }
};
        locals.var_gf2__blk1325 = assign50820_e65681;
        locals.var_gf2__blk1325_dn4 = assign50820_e65681_d_n4;
        locals.var_gf2__blk1325_dn6 = assign50820_e65681_d_n6;
        locals.var_gf2__blk1325_dn7 = assign50820_e65681_d_n7;
        locals.var_gf2__blk1325_dn8 = assign50820_e65681_d_n8;
        locals.var_gf2__blk1325_dn9 = assign50820_e65681_d_n9;

        let (assign50830_e65688, assign50830_e65688_d_n4, assign50830_e65688_d_n6, assign50830_e65688_d_n7, assign50830_e65688_d_n8, assign50830_e65688_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_inv_gf2_dc, locals.var_inv_gf2_dc_dn4, locals.var_inv_gf2_dc_dn6, locals.var_inv_gf2_dc_dn7, locals.var_inv_gf2_dc_dn8, locals.var_inv_gf2_dc_dn9,)
    } else {
        (locals.var_inv_gf2__blk1341, locals.var_inv_gf2__blk1341_dn4, locals.var_inv_gf2__blk1341_dn6, locals.var_inv_gf2__blk1341_dn7, locals.var_inv_gf2__blk1341_dn8, locals.var_inv_gf2__blk1341_dn9,)
    }
};
        locals.var_inv_gf2__blk1341 = assign50830_e65688;
        locals.var_inv_gf2__blk1341_dn4 = assign50830_e65688_d_n4;
        locals.var_inv_gf2__blk1341_dn6 = assign50830_e65688_d_n6;
        locals.var_inv_gf2__blk1341_dn7 = assign50830_e65688_d_n7;
        locals.var_inv_gf2__blk1341_dn8 = assign50830_e65688_d_n8;
        locals.var_inv_gf2__blk1341_dn9 = assign50830_e65688_d_n9;

        let (assign50840_e65695, assign50840_e65695_d_n4, assign50840_e65695_d_n6, assign50840_e65695_d_n7, assign50840_e65695_d_n8, assign50840_e65695_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_xg_dc, locals.var_xg_dc_dn4, locals.var_xg_dc_dn6, locals.var_xg_dc_dn7, locals.var_xg_dc_dn8, locals.var_xg_dc_dn9,)
    } else {
        (locals.var_xg__blk1343, locals.var_xg__blk1343_dn4, locals.var_xg__blk1343_dn6, locals.var_xg__blk1343_dn7, locals.var_xg__blk1343_dn8, locals.var_xg__blk1343_dn9,)
    }
};
        locals.var_xg__blk1343 = assign50840_e65695;
        locals.var_xg__blk1343_dn4 = assign50840_e65695_d_n4;
        locals.var_xg__blk1343_dn6 = assign50840_e65695_d_n6;
        locals.var_xg__blk1343_dn7 = assign50840_e65695_d_n7;
        locals.var_xg__blk1343_dn8 = assign50840_e65695_d_n8;
        locals.var_xg__blk1343_dn9 = assign50840_e65695_d_n9;

        let (assign50850_e65702, assign50850_e65702_d_n4, assign50850_e65702_d_n6, assign50850_e65702_d_n7, assign50850_e65702_d_n8, assign50850_e65702_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_xno_s_dc, locals.var_xno_s_dc_dn4, locals.var_xno_s_dc_dn6, locals.var_xno_s_dc_dn7, locals.var_xno_s_dc_dn8, locals.var_xno_s_dc_dn9,)
    } else {
        (locals.var_xno_s__blk1348, locals.var_xno_s__blk1348_dn4, locals.var_xno_s__blk1348_dn6, locals.var_xno_s__blk1348_dn7, locals.var_xno_s__blk1348_dn8, locals.var_xno_s__blk1348_dn9,)
    }
};
        locals.var_xno_s__blk1348 = assign50850_e65702;
        locals.var_xno_s__blk1348_dn4 = assign50850_e65702_d_n4;
        locals.var_xno_s__blk1348_dn6 = assign50850_e65702_d_n6;
        locals.var_xno_s__blk1348_dn7 = assign50850_e65702_d_n7;
        locals.var_xno_s__blk1348_dn8 = assign50850_e65702_d_n8;
        locals.var_xno_s__blk1348_dn9 = assign50850_e65702_d_n9;

        let (assign50860_e65709, assign50860_e65709_d_n4, assign50860_e65709_d_n6, assign50860_e65709_d_n7, assign50860_e65709_d_n8, assign50860_e65709_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_xn_s_dc, locals.var_xn_s_dc_dn4, locals.var_xn_s_dc_dn6, locals.var_xn_s_dc_dn7, locals.var_xn_s_dc_dn8, locals.var_xn_s_dc_dn9,)
    } else {
        (locals.var_xn_s__blk1349, locals.var_xn_s__blk1349_dn4, locals.var_xn_s__blk1349_dn6, locals.var_xn_s__blk1349_dn7, locals.var_xn_s__blk1349_dn8, locals.var_xn_s__blk1349_dn9,)
    }
};
        locals.var_xn_s__blk1349 = assign50860_e65709;
        locals.var_xn_s__blk1349_dn4 = assign50860_e65709_d_n4;
        locals.var_xn_s__blk1349_dn6 = assign50860_e65709_d_n6;
        locals.var_xn_s__blk1349_dn7 = assign50860_e65709_d_n7;
        locals.var_xn_s__blk1349_dn8 = assign50860_e65709_d_n8;
        locals.var_xn_s__blk1349_dn9 = assign50860_e65709_d_n9;

        let (assign50870_e65716, assign50870_e65716_d_n4, assign50870_e65716_d_n6, assign50870_e65716_d_n7, assign50870_e65716_d_n8, assign50870_e65716_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_xi_dc, locals.var_xi_dc_dn4, locals.var_xi_dc_dn6, locals.var_xi_dc_dn7, locals.var_xi_dc_dn8, locals.var_xi_dc_dn9,)
    } else {
        (locals.var_xi__blk1360, locals.var_xi__blk1360_dn4, locals.var_xi__blk1360_dn6, locals.var_xi__blk1360_dn7, locals.var_xi__blk1360_dn8, locals.var_xi__blk1360_dn9,)
    }
};
        locals.var_xi__blk1360 = assign50870_e65716;
        locals.var_xi__blk1360_dn4 = assign50870_e65716_d_n4;
        locals.var_xi__blk1360_dn6 = assign50870_e65716_d_n6;
        locals.var_xi__blk1360_dn7 = assign50870_e65716_d_n7;
        locals.var_xi__blk1360_dn8 = assign50870_e65716_d_n8;
        locals.var_xi__blk1360_dn9 = assign50870_e65716_d_n9;

        let (assign50880_e65723,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_margin_dc,)
    } else {
        (locals.var_margin__blk1361,)
    }
};
        locals.var_margin__blk1361 = assign50880_e65723;

        let (assign50890_e65730, assign50890_e65730_d_n4, assign50890_e65730_d_n6, assign50890_e65730_d_n7, assign50890_e65730_d_n8, assign50890_e65730_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_inv_xi_dc, locals.var_inv_xi_dc_dn4, locals.var_inv_xi_dc_dn6, locals.var_inv_xi_dc_dn7, locals.var_inv_xi_dc_dn8, locals.var_inv_xi_dc_dn9,)
    } else {
        (locals.var_inv_xi__blk1362, locals.var_inv_xi__blk1362_dn4, locals.var_inv_xi__blk1362_dn6, locals.var_inv_xi__blk1362_dn7, locals.var_inv_xi__blk1362_dn8, locals.var_inv_xi__blk1362_dn9,)
    }
};
        locals.var_inv_xi__blk1362 = assign50890_e65730;
        locals.var_inv_xi__blk1362_dn4 = assign50890_e65730_d_n4;
        locals.var_inv_xi__blk1362_dn6 = assign50890_e65730_d_n6;
        locals.var_inv_xi__blk1362_dn7 = assign50890_e65730_d_n7;
        locals.var_inv_xi__blk1362_dn8 = assign50890_e65730_d_n8;
        locals.var_inv_xi__blk1362_dn9 = assign50890_e65730_d_n9;

        let (assign50900_e65737, assign50900_e65737_d_n4, assign50900_e65737_d_n6, assign50900_e65737_d_n7, assign50900_e65737_d_n8, assign50900_e65737_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_sp_s_x1_dc, locals.var_sp_s_x1_dc_dn4, locals.var_sp_s_x1_dc_dn6, locals.var_sp_s_x1_dc_dn7, locals.var_sp_s_x1_dc_dn8, locals.var_sp_s_x1_dc_dn9,)
    } else {
        (locals.var_sp_s_x1__blk1469, locals.var_sp_s_x1__blk1469_dn4, locals.var_sp_s_x1__blk1469_dn6, locals.var_sp_s_x1__blk1469_dn7, locals.var_sp_s_x1__blk1469_dn8, locals.var_sp_s_x1__blk1469_dn9,)
    }
};
        locals.var_sp_s_x1__blk1469 = assign50900_e65737;
        locals.var_sp_s_x1__blk1469_dn4 = assign50900_e65737_d_n4;
        locals.var_sp_s_x1__blk1469_dn6 = assign50900_e65737_d_n6;
        locals.var_sp_s_x1__blk1469_dn7 = assign50900_e65737_d_n7;
        locals.var_sp_s_x1__blk1469_dn8 = assign50900_e65737_d_n8;
        locals.var_sp_s_x1__blk1469_dn9 = assign50900_e65737_d_n9;

        let (assign50910_e65744, assign50910_e65744_d_n4, assign50910_e65744_d_n6, assign50910_e65744_d_n7, assign50910_e65744_d_n8, assign50910_e65744_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_delta_ns_dc, locals.var_delta_ns_dc_dn4, locals.var_delta_ns_dc_dn6, locals.var_delta_ns_dc_dn7, locals.var_delta_ns_dc_dn8, locals.var_delta_ns_dc_dn9,)
    } else {
        (locals.var_delta_ns__blk1364, locals.var_delta_ns__blk1364_dn4, locals.var_delta_ns__blk1364_dn6, locals.var_delta_ns__blk1364_dn7, locals.var_delta_ns__blk1364_dn8, locals.var_delta_ns__blk1364_dn9,)
    }
};
        locals.var_delta_ns__blk1364 = assign50910_e65744;
        locals.var_delta_ns__blk1364_dn4 = assign50910_e65744_d_n4;
        locals.var_delta_ns__blk1364_dn6 = assign50910_e65744_d_n6;
        locals.var_delta_ns__blk1364_dn7 = assign50910_e65744_d_n7;
        locals.var_delta_ns__blk1364_dn8 = assign50910_e65744_d_n8;
        locals.var_delta_ns__blk1364_dn9 = assign50910_e65744_d_n9;

        let (assign50920_e65751, assign50920_e65751_d_n4, assign50920_e65751_d_n6, assign50920_e65751_d_n7, assign50920_e65751_d_n8, assign50920_e65751_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_x_s_dc, locals.var_x_s_dc_dn4, locals.var_x_s_dc_dn6, locals.var_x_s_dc_dn7, locals.var_x_s_dc_dn8, locals.var_x_s_dc_dn9,)
    } else {
        (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9,)
    }
};
        locals.var_x_s__blk1363 = assign50920_e65751;
        locals.var_x_s__blk1363_dn4 = assign50920_e65751_d_n4;
        locals.var_x_s__blk1363_dn6 = assign50920_e65751_d_n6;
        locals.var_x_s__blk1363_dn7 = assign50920_e65751_d_n7;
        locals.var_x_s__blk1363_dn8 = assign50920_e65751_d_n8;
        locals.var_x_s__blk1363_dn9 = assign50920_e65751_d_n9;

        let (assign50930_e65758, assign50930_e65758_d_n4, assign50930_e65758_d_n6, assign50930_e65758_d_n7, assign50930_e65758_d_n8, assign50930_e65758_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_xi1s_dc, locals.var_xi1s_dc_dn4, locals.var_xi1s_dc_dn6, locals.var_xi1s_dc_dn7, locals.var_xi1s_dc_dn8, locals.var_xi1s_dc_dn9,)
    } else {
        (locals.var_xi1s__blk1366, locals.var_xi1s__blk1366_dn4, locals.var_xi1s__blk1366_dn6, locals.var_xi1s__blk1366_dn7, locals.var_xi1s__blk1366_dn8, locals.var_xi1s__blk1366_dn9,)
    }
};
        locals.var_xi1s__blk1366 = assign50930_e65758;
        locals.var_xi1s__blk1366_dn4 = assign50930_e65758_d_n4;
        locals.var_xi1s__blk1366_dn6 = assign50930_e65758_d_n6;
        locals.var_xi1s__blk1366_dn7 = assign50930_e65758_d_n7;
        locals.var_xi1s__blk1366_dn8 = assign50930_e65758_d_n8;
        locals.var_xi1s__blk1366_dn9 = assign50930_e65758_d_n9;

        let (assign50940_e65765, assign50940_e65765_d_n4, assign50940_e65765_d_n6, assign50940_e65765_d_n7, assign50940_e65765_d_n8, assign50940_e65765_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_xi2s_dc, locals.var_xi2s_dc_dn4, locals.var_xi2s_dc_dn6, locals.var_xi2s_dc_dn7, locals.var_xi2s_dc_dn8, locals.var_xi2s_dc_dn9,)
    } else {
        (locals.var_xi2s__blk1367, locals.var_xi2s__blk1367_dn4, locals.var_xi2s__blk1367_dn6, locals.var_xi2s__blk1367_dn7, locals.var_xi2s__blk1367_dn8, locals.var_xi2s__blk1367_dn9,)
    }
};
        locals.var_xi2s__blk1367 = assign50940_e65765;
        locals.var_xi2s__blk1367_dn4 = assign50940_e65765_d_n4;
        locals.var_xi2s__blk1367_dn6 = assign50940_e65765_d_n6;
        locals.var_xi2s__blk1367_dn7 = assign50940_e65765_d_n7;
        locals.var_xi2s__blk1367_dn8 = assign50940_e65765_d_n8;
        locals.var_xi2s__blk1367_dn9 = assign50940_e65765_d_n9;

    }

    pub(super) fn stamp_transient_block_43(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign50950_e65772, assign50950_e65772_d_n4, assign50950_e65772_d_n6, assign50950_e65772_d_n7, assign50950_e65772_d_n8, assign50950_e65772_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_delta_1s_dc, locals.var_delta_1s_dc_dn4, locals.var_delta_1s_dc_dn6, locals.var_delta_1s_dc_dn7, locals.var_delta_1s_dc_dn8, locals.var_delta_1s_dc_dn9,)
    } else {
        (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9,)
    }
};
        locals.var_delta_1s__blk1368 = assign50950_e65772;
        locals.var_delta_1s__blk1368_dn4 = assign50950_e65772_d_n4;
        locals.var_delta_1s__blk1368_dn6 = assign50950_e65772_d_n6;
        locals.var_delta_1s__blk1368_dn7 = assign50950_e65772_d_n7;
        locals.var_delta_1s__blk1368_dn8 = assign50950_e65772_d_n8;
        locals.var_delta_1s__blk1368_dn9 = assign50950_e65772_d_n9;

        let (assign50960_e65779, assign50960_e65779_d_n4, assign50960_e65779_d_n6, assign50960_e65779_d_n7, assign50960_e65779_d_n8, assign50960_e65779_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_es_dc, locals.var_es_dc_dn4, locals.var_es_dc_dn6, locals.var_es_dc_dn7, locals.var_es_dc_dn8, locals.var_es_dc_dn9,)
    } else {
        (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9,)
    }
};
        locals.var_es__blk1369 = assign50960_e65779;
        locals.var_es__blk1369_dn4 = assign50960_e65779_d_n4;
        locals.var_es__blk1369_dn6 = assign50960_e65779_d_n6;
        locals.var_es__blk1369_dn7 = assign50960_e65779_d_n7;
        locals.var_es__blk1369_dn8 = assign50960_e65779_d_n8;
        locals.var_es__blk1369_dn9 = assign50960_e65779_d_n9;

        let (assign50970_e65786, assign50970_e65786_d_n4, assign50970_e65786_d_n6, assign50970_e65786_d_n7, assign50970_e65786_d_n8, assign50970_e65786_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_ps_dc, locals.var_ps_dc_dn4, locals.var_ps_dc_dn6, locals.var_ps_dc_dn7, locals.var_ps_dc_dn8, locals.var_ps_dc_dn9,)
    } else {
        (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9,)
    }
};
        locals.var_ps__blk1371 = assign50970_e65786;
        locals.var_ps__blk1371_dn4 = assign50970_e65786_d_n4;
        locals.var_ps__blk1371_dn6 = assign50970_e65786_d_n6;
        locals.var_ps__blk1371_dn7 = assign50970_e65786_d_n7;
        locals.var_ps__blk1371_dn8 = assign50970_e65786_d_n8;
        locals.var_ps__blk1371_dn9 = assign50970_e65786_d_n9;

        let (assign50980_e65793, assign50980_e65793_d_n4, assign50980_e65793_d_n6, assign50980_e65793_d_n7, assign50980_e65793_d_n8, assign50980_e65793_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_ds_dc, locals.var_ds_dc_dn4, locals.var_ds_dc_dn6, locals.var_ds_dc_dn7, locals.var_ds_dc_dn8, locals.var_ds_dc_dn9,)
    } else {
        (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9,)
    }
};
        locals.var_ds__blk1370 = assign50980_e65793;
        locals.var_ds__blk1370_dn4 = assign50980_e65793_d_n4;
        locals.var_ds__blk1370_dn6 = assign50980_e65793_d_n6;
        locals.var_ds__blk1370_dn7 = assign50980_e65793_d_n7;
        locals.var_ds__blk1370_dn8 = assign50980_e65793_d_n8;
        locals.var_ds__blk1370_dn9 = assign50980_e65793_d_n9;

        let (assign50990_e65800, assign50990_e65800_d_n4, assign50990_e65800_d_n6, assign50990_e65800_d_n7, assign50990_e65800_d_n8, assign50990_e65800_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_sqs_dc, locals.var_sqs_dc_dn4, locals.var_sqs_dc_dn6, locals.var_sqs_dc_dn7, locals.var_sqs_dc_dn8, locals.var_sqs_dc_dn9,)
    } else {
        (locals.var_sqs__blk1372, locals.var_sqs__blk1372_dn4, locals.var_sqs__blk1372_dn6, locals.var_sqs__blk1372_dn7, locals.var_sqs__blk1372_dn8, locals.var_sqs__blk1372_dn9,)
    }
};
        locals.var_sqs__blk1372 = assign50990_e65800;
        locals.var_sqs__blk1372_dn4 = assign50990_e65800_d_n4;
        locals.var_sqs__blk1372_dn6 = assign50990_e65800_d_n6;
        locals.var_sqs__blk1372_dn7 = assign50990_e65800_d_n7;
        locals.var_sqs__blk1372_dn8 = assign50990_e65800_d_n8;
        locals.var_sqs__blk1372_dn9 = assign50990_e65800_d_n9;

        let (assign51000_e65807, assign51000_e65807_d_n4, assign51000_e65807_d_n6, assign51000_e65807_d_n7, assign51000_e65807_d_n8, assign51000_e65807_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_alphas_dc, locals.var_alphas_dc_dn4, locals.var_alphas_dc_dn6, locals.var_alphas_dc_dn7, locals.var_alphas_dc_dn8, locals.var_alphas_dc_dn9,)
    } else {
        (locals.var_alphas__blk1373, locals.var_alphas__blk1373_dn4, locals.var_alphas__blk1373_dn6, locals.var_alphas__blk1373_dn7, locals.var_alphas__blk1373_dn8, locals.var_alphas__blk1373_dn9,)
    }
};
        locals.var_alphas__blk1373 = assign51000_e65807;
        locals.var_alphas__blk1373_dn4 = assign51000_e65807_d_n4;
        locals.var_alphas__blk1373_dn6 = assign51000_e65807_d_n6;
        locals.var_alphas__blk1373_dn7 = assign51000_e65807_d_n7;
        locals.var_alphas__blk1373_dn8 = assign51000_e65807_d_n8;
        locals.var_alphas__blk1373_dn9 = assign51000_e65807_d_n9;

        let (assign51010_e65814, assign51010_e65814_d_n4, assign51010_e65814_d_n6, assign51010_e65814_d_n7, assign51010_e65814_d_n8, assign51010_e65814_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_rxcor_dc, locals.var_rxcor_dc_dn4, locals.var_rxcor_dc_dn6, locals.var_rxcor_dc_dn7, locals.var_rxcor_dc_dn8, locals.var_rxcor_dc_dn9,)
    } else {
        (locals.var_rxcor__blk1374, locals.var_rxcor__blk1374_dn4, locals.var_rxcor__blk1374_dn6, locals.var_rxcor__blk1374_dn7, locals.var_rxcor__blk1374_dn8, locals.var_rxcor__blk1374_dn9,)
    }
};
        locals.var_rxcor__blk1374 = assign51010_e65814;
        locals.var_rxcor__blk1374_dn4 = assign51010_e65814_d_n4;
        locals.var_rxcor__blk1374_dn6 = assign51010_e65814_d_n6;
        locals.var_rxcor__blk1374_dn7 = assign51010_e65814_d_n7;
        locals.var_rxcor__blk1374_dn8 = assign51010_e65814_d_n8;
        locals.var_rxcor__blk1374_dn9 = assign51010_e65814_d_n9;

        let (assign51020_e65821, assign51020_e65821_d_n4, assign51020_e65821_d_n6, assign51020_e65821_d_n7, assign51020_e65821_d_n8, assign51020_e65821_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_xgs_dc, locals.var_xgs_dc_dn4, locals.var_xgs_dc_dn6, locals.var_xgs_dc_dn7, locals.var_xgs_dc_dn8, locals.var_xgs_dc_dn9,)
    } else {
        (locals.var_xgs__blk1375, locals.var_xgs__blk1375_dn4, locals.var_xgs__blk1375_dn6, locals.var_xgs__blk1375_dn7, locals.var_xgs__blk1375_dn8, locals.var_xgs__blk1375_dn9,)
    }
};
        locals.var_xgs__blk1375 = assign51020_e65821;
        locals.var_xgs__blk1375_dn4 = assign51020_e65821_d_n4;
        locals.var_xgs__blk1375_dn6 = assign51020_e65821_d_n6;
        locals.var_xgs__blk1375_dn7 = assign51020_e65821_d_n7;
        locals.var_xgs__blk1375_dn8 = assign51020_e65821_d_n8;
        locals.var_xgs__blk1375_dn9 = assign51020_e65821_d_n9;

        let (assign51030_e65828, assign51030_e65828_d_n4, assign51030_e65828_d_n6, assign51030_e65828_d_n7, assign51030_e65828_d_n8, assign51030_e65828_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_qis_dc, locals.var_qis_dc_dn4, locals.var_qis_dc_dn6, locals.var_qis_dc_dn7, locals.var_qis_dc_dn8, locals.var_qis_dc_dn9,)
    } else {
        (locals.var_qis__blk1376, locals.var_qis__blk1376_dn4, locals.var_qis__blk1376_dn6, locals.var_qis__blk1376_dn7, locals.var_qis__blk1376_dn8, locals.var_qis__blk1376_dn9,)
    }
};
        locals.var_qis__blk1376 = assign51030_e65828;
        locals.var_qis__blk1376_dn4 = assign51030_e65828_d_n4;
        locals.var_qis__blk1376_dn6 = assign51030_e65828_d_n6;
        locals.var_qis__blk1376_dn7 = assign51030_e65828_d_n7;
        locals.var_qis__blk1376_dn8 = assign51030_e65828_d_n8;
        locals.var_qis__blk1376_dn9 = assign51030_e65828_d_n9;

        let (assign51040_e65835, assign51040_e65835_d_n4, assign51040_e65835_d_n6, assign51040_e65835_d_n7, assign51040_e65835_d_n8, assign51040_e65835_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_qbs_dc, locals.var_qbs_dc_dn4, locals.var_qbs_dc_dn6, locals.var_qbs_dc_dn7, locals.var_qbs_dc_dn8, locals.var_qbs_dc_dn9,)
    } else {
        (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9,)
    }
};
        locals.var_qbs__blk1377 = assign51040_e65835;
        locals.var_qbs__blk1377_dn4 = assign51040_e65835_d_n4;
        locals.var_qbs__blk1377_dn6 = assign51040_e65835_d_n6;
        locals.var_qbs__blk1377_dn7 = assign51040_e65835_d_n7;
        locals.var_qbs__blk1377_dn8 = assign51040_e65835_d_n8;
        locals.var_qbs__blk1377_dn9 = assign51040_e65835_d_n9;

        let (assign51050_e65842, assign51050_e65842_d_n4, assign51050_e65842_d_n6, assign51050_e65842_d_n7, assign51050_e65842_d_n8, assign51050_e65842_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_rhob_dc, locals.var_rhob_dc_dn4, locals.var_rhob_dc_dn6, locals.var_rhob_dc_dn7, locals.var_rhob_dc_dn8, locals.var_rhob_dc_dn9,)
    } else {
        (locals.var_rhob__blk1378, locals.var_rhob__blk1378_dn4, locals.var_rhob__blk1378_dn6, locals.var_rhob__blk1378_dn7, locals.var_rhob__blk1378_dn8, locals.var_rhob__blk1378_dn9,)
    }
};
        locals.var_rhob__blk1378 = assign51050_e65842;
        locals.var_rhob__blk1378_dn4 = assign51050_e65842_d_n4;
        locals.var_rhob__blk1378_dn6 = assign51050_e65842_d_n6;
        locals.var_rhob__blk1378_dn7 = assign51050_e65842_d_n7;
        locals.var_rhob__blk1378_dn8 = assign51050_e65842_d_n8;
        locals.var_rhob__blk1378_dn9 = assign51050_e65842_d_n9;

        let (assign51060_e65849, assign51060_e65849_d_n4, assign51060_e65849_d_n6, assign51060_e65849_d_n7, assign51060_e65849_d_n8, assign51060_e65849_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_rhog_dc, locals.var_rhog_dc_dn4, locals.var_rhog_dc_dn6, locals.var_rhog_dc_dn7, locals.var_rhog_dc_dn8, locals.var_rhog_dc_dn9,)
    } else {
        (locals.var_rhog__blk1379, locals.var_rhog__blk1379_dn4, locals.var_rhog__blk1379_dn6, locals.var_rhog__blk1379_dn7, locals.var_rhog__blk1379_dn8, locals.var_rhog__blk1379_dn9,)
    }
};
        locals.var_rhog__blk1379 = assign51060_e65849;
        locals.var_rhog__blk1379_dn4 = assign51060_e65849_d_n4;
        locals.var_rhog__blk1379_dn6 = assign51060_e65849_d_n6;
        locals.var_rhog__blk1379_dn7 = assign51060_e65849_d_n7;
        locals.var_rhog__blk1379_dn8 = assign51060_e65849_d_n8;
        locals.var_rhog__blk1379_dn9 = assign51060_e65849_d_n9;

        let (assign51070_e65856, assign51070_e65856_d_n4, assign51070_e65856_d_n6, assign51070_e65856_d_n7, assign51070_e65856_d_n8, assign51070_e65856_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_gmobs_dc, locals.var_gmobs_dc_dn4, locals.var_gmobs_dc_dn6, locals.var_gmobs_dc_dn7, locals.var_gmobs_dc_dn8, locals.var_gmobs_dc_dn9,)
    } else {
        (locals.var_gmobs__blk1383, locals.var_gmobs__blk1383_dn4, locals.var_gmobs__blk1383_dn6, locals.var_gmobs__blk1383_dn7, locals.var_gmobs__blk1383_dn8, locals.var_gmobs__blk1383_dn9,)
    }
};
        locals.var_gmobs__blk1383 = assign51070_e65856;
        locals.var_gmobs__blk1383_dn4 = assign51070_e65856_d_n4;
        locals.var_gmobs__blk1383_dn6 = assign51070_e65856_d_n6;
        locals.var_gmobs__blk1383_dn7 = assign51070_e65856_d_n7;
        locals.var_gmobs__blk1383_dn8 = assign51070_e65856_d_n8;
        locals.var_gmobs__blk1383_dn9 = assign51070_e65856_d_n9;

        let (assign51080_e65863, assign51080_e65863_d_n4, assign51080_e65863_d_n6, assign51080_e65863_d_n7, assign51080_e65863_d_n8, assign51080_e65863_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_xitsb_dc, locals.var_xitsb_dc_dn4, locals.var_xitsb_dc_dn6, locals.var_xitsb_dc_dn7, locals.var_xitsb_dc_dn8, locals.var_xitsb_dc_dn9,)
    } else {
        (locals.var_xitsb__blk1384, locals.var_xitsb__blk1384_dn4, locals.var_xitsb__blk1384_dn6, locals.var_xitsb__blk1384_dn7, locals.var_xitsb__blk1384_dn8, locals.var_xitsb__blk1384_dn9,)
    }
};
        locals.var_xitsb__blk1384 = assign51080_e65863;
        locals.var_xitsb__blk1384_dn4 = assign51080_e65863_d_n4;
        locals.var_xitsb__blk1384_dn6 = assign51080_e65863_d_n6;
        locals.var_xitsb__blk1384_dn7 = assign51080_e65863_d_n7;
        locals.var_xitsb__blk1384_dn8 = assign51080_e65863_d_n8;
        locals.var_xitsb__blk1384_dn9 = assign51080_e65863_d_n9;

        let (assign51090_e65870, assign51090_e65870_d_n4, assign51090_e65870_d_n6, assign51090_e65870_d_n7, assign51090_e65870_d_n8, assign51090_e65870_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
        (locals.var_factheta_dc, locals.var_factheta_dc_dn4, locals.var_factheta_dc_dn6, locals.var_factheta_dc_dn7, locals.var_factheta_dc_dn8, locals.var_factheta_dc_dn9,)
    } else {
        (locals.var_factheta__blk1386, locals.var_factheta__blk1386_dn4, locals.var_factheta__blk1386_dn6, locals.var_factheta__blk1386_dn7, locals.var_factheta__blk1386_dn8, locals.var_factheta__blk1386_dn9,)
    }
};
        locals.var_factheta__blk1386 = assign51090_e65870;
        locals.var_factheta__blk1386_dn4 = assign51090_e65870_d_n4;
        locals.var_factheta__blk1386_dn6 = assign51090_e65870_d_n6;
        locals.var_factheta__blk1386_dn7 = assign51090_e65870_d_n7;
        locals.var_factheta__blk1386_dn8 = assign51090_e65870_d_n8;
        locals.var_factheta__blk1386_dn9 = assign51090_e65870_d_n9;

        let (assign51110_e65881, assign51110_e65881_d_n4,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_thesat_t, locals.var_thesat_t_dn4,)
    } else {
        (locals.var_thesatloc__blk1319, locals.var_thesatloc__blk1319_dn4,)
    }
};
        locals.var_thesatloc__blk1319 = assign51110_e65881;
        locals.var_thesatloc__blk1319_dn4 = assign51110_e65881_d_n4;

        let (assign51120_e65885,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_ar,)
    } else {
        (locals.var_arloc__blk1320,)
    }
};
        locals.var_arloc__blk1320 = assign51120_e65885;

        let assign51130_e65888: f64 = if p.p48 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1500 = assign51130_e65888;

        let (assign51140_e65894, assign51140_e65894_d_n4,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1500 != 0.0)) {
        (locals.var_thesatac_t, locals.var_thesatac_t_dn4,)
    } else {
        (locals.var_thesatloc__blk1319, locals.var_thesatloc__blk1319_dn4,)
    }
};
        locals.var_thesatloc__blk1319 = assign51140_e65894;
        locals.var_thesatloc__blk1319_dn4 = assign51140_e65894_d_n4;

        let (assign51150_e65900,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1500 != 0.0)) {
        (locals.var_arac,)
    } else {
        (locals.var_arloc__blk1320,)
    }
};
        locals.var_arloc__blk1320 = assign51150_e65900;

        let (assign51160_e65904, assign51160_e65904_d_n4, assign51160_e65904_d_n6, assign51160_e65904_d_n7, assign51160_e65904_d_n8, assign51160_e65904_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_thesat1__blk1388, locals.var_thesat1__blk1388_dn4, locals.var_thesat1__blk1388_dn6, locals.var_thesat1__blk1388_dn7, locals.var_thesat1__blk1388_dn8, locals.var_thesat1__blk1388_dn9,)
    }
};
        locals.var_thesat1__blk1388 = assign51160_e65904;
        locals.var_thesat1__blk1388_dn4 = assign51160_e65904_d_n4;
        locals.var_thesat1__blk1388_dn6 = assign51160_e65904_d_n6;
        locals.var_thesat1__blk1388_dn7 = assign51160_e65904_d_n7;
        locals.var_thesat1__blk1388_dn8 = assign51160_e65904_d_n8;
        locals.var_thesat1__blk1388_dn9 = assign51160_e65904_d_n9;

        let (assign51170_e65910, assign51170_e65910_d_n4, assign51170_e65910_d_n6, assign51170_e65910_d_n7, assign51170_e65910_d_n8, assign51170_e65910_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        let assign51170_e65908: f64 = (locals.var_phit1__blk1339 * 4.60517018598809);
        (assign51170_e65908, (locals.var_phit1__blk1339_dn4 * 4.60517018598809), (locals.var_phit1__blk1339_dn6 * 4.60517018598809), (locals.var_phit1__blk1339_dn7 * 4.60517018598809), (locals.var_phit1__blk1339_dn8 * 4.60517018598809), (locals.var_phit1__blk1339_dn9 * 4.60517018598809),)
    } else {
        (locals.var_vdsat_lim__blk1387, locals.var_vdsat_lim__blk1387_dn4, locals.var_vdsat_lim__blk1387_dn6, locals.var_vdsat_lim__blk1387_dn7, locals.var_vdsat_lim__blk1387_dn8, locals.var_vdsat_lim__blk1387_dn9,)
    }
};
        locals.var_vdsat_lim__blk1387 = assign51170_e65910;
        locals.var_vdsat_lim__blk1387_dn4 = assign51170_e65910_d_n4;
        locals.var_vdsat_lim__blk1387_dn6 = assign51170_e65910_d_n6;
        locals.var_vdsat_lim__blk1387_dn7 = assign51170_e65910_d_n7;
        locals.var_vdsat_lim__blk1387_dn8 = assign51170_e65910_d_n8;
        locals.var_vdsat_lim__blk1387_dn9 = assign51170_e65910_d_n9;

        let (assign51180_e65914, assign51180_e65914_d_n4, assign51180_e65914_d_n6, assign51180_e65914_d_n7, assign51180_e65914_d_n8, assign51180_e65914_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_vdsat_lim__blk1387, locals.var_vdsat_lim__blk1387_dn4, locals.var_vdsat_lim__blk1387_dn6, locals.var_vdsat_lim__blk1387_dn7, locals.var_vdsat_lim__blk1387_dn8, locals.var_vdsat_lim__blk1387_dn9,)
    } else {
        (locals.var_v_dsat__blk1404, locals.var_v_dsat__blk1404_dn4, locals.var_v_dsat__blk1404_dn6, locals.var_v_dsat__blk1404_dn7, locals.var_v_dsat__blk1404_dn8, locals.var_v_dsat__blk1404_dn9,)
    }
};
        locals.var_v_dsat__blk1404 = assign51180_e65914;
        locals.var_v_dsat__blk1404_dn4 = assign51180_e65914_d_n4;
        locals.var_v_dsat__blk1404_dn6 = assign51180_e65914_d_n6;
        locals.var_v_dsat__blk1404_dn7 = assign51180_e65914_d_n7;
        locals.var_v_dsat__blk1404_dn8 = assign51180_e65914_d_n8;
        locals.var_v_dsat__blk1404_dn9 = assign51180_e65914_d_n9;

        let (assign51190_e65918, assign51190_e65918_d_n4, assign51190_e65918_d_n6, assign51190_e65918_d_n7, assign51190_e65918_d_n8, assign51190_e65918_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_v_ds, 0.0, 0.0, locals.var_v_ds_dn7, locals.var_v_ds_dn8, 0.0,)
    } else {
        (locals.var_vdse__blk1405, locals.var_vdse__blk1405_dn4, locals.var_vdse__blk1405_dn6, locals.var_vdse__blk1405_dn7, locals.var_vdse__blk1405_dn8, locals.var_vdse__blk1405_dn9,)
    }
};
        locals.var_vdse__blk1405 = assign51190_e65918;
        locals.var_vdse__blk1405_dn4 = assign51190_e65918_d_n4;
        locals.var_vdse__blk1405_dn6 = assign51190_e65918_d_n6;
        locals.var_vdse__blk1405_dn7 = assign51190_e65918_d_n7;
        locals.var_vdse__blk1405_dn8 = assign51190_e65918_d_n8;
        locals.var_vdse__blk1405_dn9 = assign51190_e65918_d_n9;

        let (assign51200_e65924, assign51200_e65924_d_n4, assign51200_e65924_d_n6, assign51200_e65924_d_n7, assign51200_e65924_d_n8, assign51200_e65924_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        let assign51200_e65922: f64 = (locals.var_v_ds * locals.var_inv_phit1__blk1340);
        (assign51200_e65922, (locals.var_v_ds * locals.var_inv_phit1__blk1340_dn4), (locals.var_v_ds * locals.var_inv_phit1__blk1340_dn6), ((locals.var_v_ds_dn7 * locals.var_inv_phit1__blk1340) + (locals.var_v_ds * locals.var_inv_phit1__blk1340_dn7)), ((locals.var_v_ds_dn8 * locals.var_inv_phit1__blk1340) + (locals.var_v_ds * locals.var_inv_phit1__blk1340_dn8)), (locals.var_v_ds * locals.var_inv_phit1__blk1340_dn9),)
    } else {
        (locals.var_udse__blk1406, locals.var_udse__blk1406_dn4, locals.var_udse__blk1406_dn6, locals.var_udse__blk1406_dn7, locals.var_udse__blk1406_dn8, locals.var_udse__blk1406_dn9,)
    }
};
        locals.var_udse__blk1406 = assign51200_e65924;
        locals.var_udse__blk1406_dn4 = assign51200_e65924_d_n4;
        locals.var_udse__blk1406_dn6 = assign51200_e65924_d_n6;
        locals.var_udse__blk1406_dn7 = assign51200_e65924_d_n7;
        locals.var_udse__blk1406_dn8 = assign51200_e65924_d_n8;
        locals.var_udse__blk1406_dn9 = assign51200_e65924_d_n9;

        let (assign51210_e65928, assign51210_e65928_d_n4, assign51210_e65928_d_n6, assign51210_e65928_d_n7, assign51210_e65928_d_n8, assign51210_e65928_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9,)
    } else {
        (locals.var_x_d__blk1410, locals.var_x_d__blk1410_dn4, locals.var_x_d__blk1410_dn6, locals.var_x_d__blk1410_dn7, locals.var_x_d__blk1410_dn8, locals.var_x_d__blk1410_dn9,)
    }
};
        locals.var_x_d__blk1410 = assign51210_e65928;
        locals.var_x_d__blk1410_dn4 = assign51210_e65928_d_n4;
        locals.var_x_d__blk1410_dn6 = assign51210_e65928_d_n6;
        locals.var_x_d__blk1410_dn7 = assign51210_e65928_d_n7;
        locals.var_x_d__blk1410_dn8 = assign51210_e65928_d_n8;
        locals.var_x_d__blk1410_dn9 = assign51210_e65928_d_n9;

        let (assign51220_e65932, assign51220_e65932_d_n4, assign51220_e65932_d_n6, assign51220_e65932_d_n7, assign51220_e65932_d_n8, assign51220_e65932_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_x_ds__blk1411, locals.var_x_ds__blk1411_dn4, locals.var_x_ds__blk1411_dn6, locals.var_x_ds__blk1411_dn7, locals.var_x_ds__blk1411_dn8, locals.var_x_ds__blk1411_dn9,)
    }
};
        locals.var_x_ds__blk1411 = assign51220_e65932;
        locals.var_x_ds__blk1411_dn4 = assign51220_e65932_d_n4;
        locals.var_x_ds__blk1411_dn6 = assign51220_e65932_d_n6;
        locals.var_x_ds__blk1411_dn7 = assign51220_e65932_d_n7;
        locals.var_x_ds__blk1411_dn8 = assign51220_e65932_d_n8;
        locals.var_x_ds__blk1411_dn9 = assign51220_e65932_d_n9;

        let (assign51230_e65936, assign51230_e65936_d_n4, assign51230_e65936_d_n6, assign51230_e65936_d_n7, assign51230_e65936_d_n8, assign51230_e65936_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1414, locals.var_dps__blk1414_dn4, locals.var_dps__blk1414_dn6, locals.var_dps__blk1414_dn7, locals.var_dps__blk1414_dn8, locals.var_dps__blk1414_dn9,)
    }
};
        locals.var_dps__blk1414 = assign51230_e65936;
        locals.var_dps__blk1414_dn4 = assign51230_e65936_d_n4;
        locals.var_dps__blk1414_dn6 = assign51230_e65936_d_n6;
        locals.var_dps__blk1414_dn7 = assign51230_e65936_d_n7;
        locals.var_dps__blk1414_dn8 = assign51230_e65936_d_n8;
        locals.var_dps__blk1414_dn9 = assign51230_e65936_d_n9;

        let (assign51240_e65940, assign51240_e65940_d_n4, assign51240_e65940_d_n6, assign51240_e65940_d_n7, assign51240_e65940_d_n8, assign51240_e65940_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9,)
    } else {
        (locals.var_ed__blk1416, locals.var_ed__blk1416_dn4, locals.var_ed__blk1416_dn6, locals.var_ed__blk1416_dn7, locals.var_ed__blk1416_dn8, locals.var_ed__blk1416_dn9,)
    }
};
        locals.var_ed__blk1416 = assign51240_e65940;
        locals.var_ed__blk1416_dn4 = assign51240_e65940_d_n4;
        locals.var_ed__blk1416_dn6 = assign51240_e65940_d_n6;
        locals.var_ed__blk1416_dn7 = assign51240_e65940_d_n7;
        locals.var_ed__blk1416_dn8 = assign51240_e65940_d_n8;
        locals.var_ed__blk1416_dn9 = assign51240_e65940_d_n9;

        let (assign51250_e65944, assign51250_e65944_d_n4, assign51250_e65944_d_n6, assign51250_e65944_d_n7, assign51250_e65944_d_n8, assign51250_e65944_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9,)
    } else {
        (locals.var_pd__blk1417, locals.var_pd__blk1417_dn4, locals.var_pd__blk1417_dn6, locals.var_pd__blk1417_dn7, locals.var_pd__blk1417_dn8, locals.var_pd__blk1417_dn9,)
    }
};
        locals.var_pd__blk1417 = assign51250_e65944;
        locals.var_pd__blk1417_dn4 = assign51250_e65944_d_n4;
        locals.var_pd__blk1417_dn6 = assign51250_e65944_d_n6;
        locals.var_pd__blk1417_dn7 = assign51250_e65944_d_n7;
        locals.var_pd__blk1417_dn8 = assign51250_e65944_d_n8;
        locals.var_pd__blk1417_dn9 = assign51250_e65944_d_n9;

        let (assign51260_e65948, assign51260_e65948_d_n4, assign51260_e65948_d_n6, assign51260_e65948_d_n7, assign51260_e65948_d_n8, assign51260_e65948_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9,)
    } else {
        (locals.var_dd__blk1419, locals.var_dd__blk1419_dn4, locals.var_dd__blk1419_dn6, locals.var_dd__blk1419_dn7, locals.var_dd__blk1419_dn8, locals.var_dd__blk1419_dn9,)
    }
};
        locals.var_dd__blk1419 = assign51260_e65948;
        locals.var_dd__blk1419_dn4 = assign51260_e65948_d_n4;
        locals.var_dd__blk1419_dn6 = assign51260_e65948_d_n6;
        locals.var_dd__blk1419_dn7 = assign51260_e65948_d_n7;
        locals.var_dd__blk1419_dn8 = assign51260_e65948_d_n8;
        locals.var_dd__blk1419_dn9 = assign51260_e65948_d_n9;

        let (assign51270_e65952, assign51270_e65952_d_n4, assign51270_e65952_d_n6, assign51270_e65952_d_n7, assign51270_e65952_d_n8, assign51270_e65952_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9,)
    } else {
        (locals.var_qbd__blk1420, locals.var_qbd__blk1420_dn4, locals.var_qbd__blk1420_dn6, locals.var_qbd__blk1420_dn7, locals.var_qbd__blk1420_dn8, locals.var_qbd__blk1420_dn9,)
    }
};
        locals.var_qbd__blk1420 = assign51270_e65952;
        locals.var_qbd__blk1420_dn4 = assign51270_e65952_d_n4;
        locals.var_qbd__blk1420_dn6 = assign51270_e65952_d_n6;
        locals.var_qbd__blk1420_dn7 = assign51270_e65952_d_n7;
        locals.var_qbd__blk1420_dn8 = assign51270_e65952_d_n8;
        locals.var_qbd__blk1420_dn9 = assign51270_e65952_d_n9;

        let (assign51280_e65956, assign51280_e65956_d_n4, assign51280_e65956_d_n6, assign51280_e65956_d_n7, assign51280_e65956_d_n8, assign51280_e65956_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9,)
    } else {
        (locals.var_x_m__blk1421, locals.var_x_m__blk1421_dn4, locals.var_x_m__blk1421_dn6, locals.var_x_m__blk1421_dn7, locals.var_x_m__blk1421_dn8, locals.var_x_m__blk1421_dn9,)
    }
};
        locals.var_x_m__blk1421 = assign51280_e65956;
        locals.var_x_m__blk1421_dn4 = assign51280_e65956_d_n4;
        locals.var_x_m__blk1421_dn6 = assign51280_e65956_d_n6;
        locals.var_x_m__blk1421_dn7 = assign51280_e65956_d_n7;
        locals.var_x_m__blk1421_dn8 = assign51280_e65956_d_n8;
        locals.var_x_m__blk1421_dn9 = assign51280_e65956_d_n9;

        let (assign51290_e65960, assign51290_e65960_d_n4, assign51290_e65960_d_n6, assign51290_e65960_d_n7, assign51290_e65960_d_n8, assign51290_e65960_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9,)
    } else {
        (locals.var_em__blk1422, locals.var_em__blk1422_dn4, locals.var_em__blk1422_dn6, locals.var_em__blk1422_dn7, locals.var_em__blk1422_dn8, locals.var_em__blk1422_dn9,)
    }
};
        locals.var_em__blk1422 = assign51290_e65960;
        locals.var_em__blk1422_dn4 = assign51290_e65960_d_n4;
        locals.var_em__blk1422_dn6 = assign51290_e65960_d_n6;
        locals.var_em__blk1422_dn7 = assign51290_e65960_d_n7;
        locals.var_em__blk1422_dn8 = assign51290_e65960_d_n8;
        locals.var_em__blk1422_dn9 = assign51290_e65960_d_n9;

        let (assign51300_e65964, assign51300_e65964_d_n4, assign51300_e65964_d_n6, assign51300_e65964_d_n7, assign51300_e65964_d_n8, assign51300_e65964_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9,)
    } else {
        (locals.var_dm__blk1424, locals.var_dm__blk1424_dn4, locals.var_dm__blk1424_dn6, locals.var_dm__blk1424_dn7, locals.var_dm__blk1424_dn8, locals.var_dm__blk1424_dn9,)
    }
};
        locals.var_dm__blk1424 = assign51300_e65964;
        locals.var_dm__blk1424_dn4 = assign51300_e65964_d_n4;
        locals.var_dm__blk1424_dn6 = assign51300_e65964_d_n6;
        locals.var_dm__blk1424_dn7 = assign51300_e65964_d_n7;
        locals.var_dm__blk1424_dn8 = assign51300_e65964_d_n8;
        locals.var_dm__blk1424_dn9 = assign51300_e65964_d_n9;

        let (assign51310_e65968, assign51310_e65968_d_n4, assign51310_e65968_d_n6, assign51310_e65968_d_n7, assign51310_e65968_d_n8, assign51310_e65968_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9,)
    } else {
        (locals.var_pm__blk1425, locals.var_pm__blk1425_dn4, locals.var_pm__blk1425_dn6, locals.var_pm__blk1425_dn7, locals.var_pm__blk1425_dn8, locals.var_pm__blk1425_dn9,)
    }
};
        locals.var_pm__blk1425 = assign51310_e65968;
        locals.var_pm__blk1425_dn4 = assign51310_e65968_d_n4;
        locals.var_pm__blk1425_dn6 = assign51310_e65968_d_n6;
        locals.var_pm__blk1425_dn7 = assign51310_e65968_d_n7;
        locals.var_pm__blk1425_dn8 = assign51310_e65968_d_n8;
        locals.var_pm__blk1425_dn9 = assign51310_e65968_d_n9;

        let (assign51320_e65974, assign51320_e65974_d_n4, assign51320_e65974_d_n6, assign51320_e65974_d_n7, assign51320_e65974_d_n8, assign51320_e65974_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        let assign51320_e65972: f64 = (locals.var_xg__blk1343 - locals.var_x_s__blk1363);
        (assign51320_e65972, (locals.var_xg__blk1343_dn4 - locals.var_x_s__blk1363_dn4), (locals.var_xg__blk1343_dn6 - locals.var_x_s__blk1363_dn6), (locals.var_xg__blk1343_dn7 - locals.var_x_s__blk1363_dn7), (locals.var_xg__blk1343_dn8 - locals.var_x_s__blk1363_dn8), (locals.var_xg__blk1343_dn9 - locals.var_x_s__blk1363_dn9),)
    } else {
        (locals.var_xgm__blk1426, locals.var_xgm__blk1426_dn4, locals.var_xgm__blk1426_dn6, locals.var_xgm__blk1426_dn7, locals.var_xgm__blk1426_dn8, locals.var_xgm__blk1426_dn9,)
    }
};
        locals.var_xgm__blk1426 = assign51320_e65974;
        locals.var_xgm__blk1426_dn4 = assign51320_e65974_d_n4;
        locals.var_xgm__blk1426_dn6 = assign51320_e65974_d_n6;
        locals.var_xgm__blk1426_dn7 = assign51320_e65974_d_n7;
        locals.var_xgm__blk1426_dn8 = assign51320_e65974_d_n8;
        locals.var_xgm__blk1426_dn9 = assign51320_e65974_d_n9;

        let (assign51330_e65978, assign51330_e65978_d_n4, assign51330_e65978_d_n6, assign51330_e65978_d_n7, assign51330_e65978_d_n8, assign51330_e65978_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eta_p__blk1427, locals.var_eta_p__blk1427_dn4, locals.var_eta_p__blk1427_dn6, locals.var_eta_p__blk1427_dn7, locals.var_eta_p__blk1427_dn8, locals.var_eta_p__blk1427_dn9,)
    }
};
        locals.var_eta_p__blk1427 = assign51330_e65978;
        locals.var_eta_p__blk1427_dn4 = assign51330_e65978_d_n4;
        locals.var_eta_p__blk1427_dn6 = assign51330_e65978_d_n6;
        locals.var_eta_p__blk1427_dn7 = assign51330_e65978_d_n7;
        locals.var_eta_p__blk1427_dn8 = assign51330_e65978_d_n8;
        locals.var_eta_p__blk1427_dn9 = assign51330_e65978_d_n9;

    }

    pub(super) fn stamp_transient_block_44(
        locals: &mut StampLocals,
    ) {
        let (assign51340_e65982, assign51340_e65982_d_n4, assign51340_e65982_d_n6, assign51340_e65982_d_n7, assign51340_e65982_d_n8, assign51340_e65982_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_alpha__blk1429, locals.var_alpha__blk1429_dn4, locals.var_alpha__blk1429_dn6, locals.var_alpha__blk1429_dn7, locals.var_alpha__blk1429_dn8, locals.var_alpha__blk1429_dn9,)
    }
};
        locals.var_alpha__blk1429 = assign51340_e65982;
        locals.var_alpha__blk1429_dn4 = assign51340_e65982_d_n4;
        locals.var_alpha__blk1429_dn6 = assign51340_e65982_d_n6;
        locals.var_alpha__blk1429_dn7 = assign51340_e65982_d_n7;
        locals.var_alpha__blk1429_dn8 = assign51340_e65982_d_n8;
        locals.var_alpha__blk1429_dn9 = assign51340_e65982_d_n9;

        let (assign51350_e65986, assign51350_e65986_d_n4, assign51350_e65986_d_n6, assign51350_e65986_d_n7, assign51350_e65986_d_n8, assign51350_e65986_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sqm__blk1428, locals.var_sqm__blk1428_dn4, locals.var_sqm__blk1428_dn6, locals.var_sqm__blk1428_dn7, locals.var_sqm__blk1428_dn8, locals.var_sqm__blk1428_dn9,)
    }
};
        locals.var_sqm__blk1428 = assign51350_e65986;
        locals.var_sqm__blk1428_dn4 = assign51350_e65986_d_n4;
        locals.var_sqm__blk1428_dn6 = assign51350_e65986_d_n6;
        locals.var_sqm__blk1428_dn7 = assign51350_e65986_d_n7;
        locals.var_sqm__blk1428_dn8 = assign51350_e65986_d_n8;
        locals.var_sqm__blk1428_dn9 = assign51350_e65986_d_n9;

        let (assign51360_e65990, assign51360_e65990_d_n4, assign51360_e65990_d_n6, assign51360_e65990_d_n7, assign51360_e65990_d_n8, assign51360_e65990_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_qis__blk1376, locals.var_qis__blk1376_dn4, locals.var_qis__blk1376_dn6, locals.var_qis__blk1376_dn7, locals.var_qis__blk1376_dn8, locals.var_qis__blk1376_dn9,)
    } else {
        (locals.var_qim__blk1438, locals.var_qim__blk1438_dn4, locals.var_qim__blk1438_dn6, locals.var_qim__blk1438_dn7, locals.var_qim__blk1438_dn8, locals.var_qim__blk1438_dn9,)
    }
};
        locals.var_qim__blk1438 = assign51360_e65990;
        locals.var_qim__blk1438_dn4 = assign51360_e65990_d_n4;
        locals.var_qim__blk1438_dn6 = assign51360_e65990_d_n6;
        locals.var_qim__blk1438_dn7 = assign51360_e65990_d_n7;
        locals.var_qim__blk1438_dn8 = assign51360_e65990_d_n8;
        locals.var_qim__blk1438_dn9 = assign51360_e65990_d_n9;

        let (assign51370_e65996, assign51370_e65996_d_n4, assign51370_e65996_d_n6, assign51370_e65996_d_n7, assign51370_e65996_d_n8, assign51370_e65996_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        let assign51370_e65994: f64 = (locals.var_xgm__blk1426 * locals.var_phit1__blk1339);
        (assign51370_e65994, ((locals.var_xgm__blk1426_dn4 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn4)), ((locals.var_xgm__blk1426_dn6 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn6)), ((locals.var_xgm__blk1426_dn7 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn7)), ((locals.var_xgm__blk1426_dn8 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn8)), ((locals.var_xgm__blk1426_dn9 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn9)),)
    } else {
        (locals.var_qeff1__blk1442, locals.var_qeff1__blk1442_dn4, locals.var_qeff1__blk1442_dn6, locals.var_qeff1__blk1442_dn7, locals.var_qeff1__blk1442_dn8, locals.var_qeff1__blk1442_dn9,)
    }
};
        locals.var_qeff1__blk1442 = assign51370_e65996;
        locals.var_qeff1__blk1442_dn4 = assign51370_e65996_d_n4;
        locals.var_qeff1__blk1442_dn6 = assign51370_e65996_d_n6;
        locals.var_qeff1__blk1442_dn7 = assign51370_e65996_d_n7;
        locals.var_qeff1__blk1442_dn8 = assign51370_e65996_d_n8;
        locals.var_qeff1__blk1442_dn9 = assign51370_e65996_d_n9;

        let (assign51380_e66000, assign51380_e66000_d_n4, assign51380_e66000_d_n6, assign51380_e66000_d_n7, assign51380_e66000_d_n8, assign51380_e66000_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qim1__blk1439, locals.var_qim1__blk1439_dn4, locals.var_qim1__blk1439_dn6, locals.var_qim1__blk1439_dn7, locals.var_qim1__blk1439_dn8, locals.var_qim1__blk1439_dn9,)
    }
};
        locals.var_qim1__blk1439 = assign51380_e66000;
        locals.var_qim1__blk1439_dn4 = assign51380_e66000_d_n4;
        locals.var_qim1__blk1439_dn6 = assign51380_e66000_d_n6;
        locals.var_qim1__blk1439_dn7 = assign51380_e66000_d_n7;
        locals.var_qim1__blk1439_dn8 = assign51380_e66000_d_n8;
        locals.var_qim1__blk1439_dn9 = assign51380_e66000_d_n9;

        let (assign51390_e66004, assign51390_e66004_d_n4, assign51390_e66004_d_n6, assign51390_e66004_d_n7, assign51390_e66004_d_n8, assign51390_e66004_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9,)
    } else {
        (locals.var_qbm__blk1440, locals.var_qbm__blk1440_dn4, locals.var_qbm__blk1440_dn6, locals.var_qbm__blk1440_dn7, locals.var_qbm__blk1440_dn8, locals.var_qbm__blk1440_dn9,)
    }
};
        locals.var_qbm__blk1440 = assign51390_e66004;
        locals.var_qbm__blk1440_dn4 = assign51390_e66004_d_n4;
        locals.var_qbm__blk1440_dn6 = assign51390_e66004_d_n6;
        locals.var_qbm__blk1440_dn7 = assign51390_e66004_d_n7;
        locals.var_qbm__blk1440_dn8 = assign51390_e66004_d_n8;
        locals.var_qbm__blk1440_dn9 = assign51390_e66004_d_n9;

        let (assign51400_e66008, assign51400_e66008_d_n4, assign51400_e66008_d_n6, assign51400_e66008_d_n7, assign51400_e66008_d_n8, assign51400_e66008_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_s1__blk1445, locals.var_s1__blk1445_dn4, locals.var_s1__blk1445_dn6, locals.var_s1__blk1445_dn7, locals.var_s1__blk1445_dn8, locals.var_s1__blk1445_dn9,)
    }
};
        locals.var_s1__blk1445 = assign51400_e66008;
        locals.var_s1__blk1445_dn4 = assign51400_e66008_d_n4;
        locals.var_s1__blk1445_dn6 = assign51400_e66008_d_n6;
        locals.var_s1__blk1445_dn7 = assign51400_e66008_d_n7;
        locals.var_s1__blk1445_dn8 = assign51400_e66008_d_n8;
        locals.var_s1__blk1445_dn9 = assign51400_e66008_d_n9;

        let (assign51410_e66012, assign51410_e66012_d_n4, assign51410_e66012_d_n6, assign51410_e66012_d_n7, assign51410_e66012_d_n8, assign51410_e66012_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gmob__blk1444, locals.var_gmob__blk1444_dn4, locals.var_gmob__blk1444_dn6, locals.var_gmob__blk1444_dn7, locals.var_gmob__blk1444_dn8, locals.var_gmob__blk1444_dn9,)
    }
};
        locals.var_gmob__blk1444 = assign51410_e66012;
        locals.var_gmob__blk1444_dn4 = assign51410_e66012_d_n4;
        locals.var_gmob__blk1444_dn6 = assign51410_e66012_d_n6;
        locals.var_gmob__blk1444_dn7 = assign51410_e66012_d_n7;
        locals.var_gmob__blk1444_dn8 = assign51410_e66012_d_n8;
        locals.var_gmob__blk1444_dn9 = assign51410_e66012_d_n9;

        let (assign51420_e66016, assign51420_e66016_d_n4, assign51420_e66016_d_n6, assign51420_e66016_d_n7, assign51420_e66016_d_n8, assign51420_e66016_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_thesatloc__blk1319, locals.var_thesatloc__blk1319_dn4, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_thesateff__blk1447, locals.var_thesateff__blk1447_dn4, locals.var_thesateff__blk1447_dn6, locals.var_thesateff__blk1447_dn7, locals.var_thesateff__blk1447_dn8, locals.var_thesateff__blk1447_dn9,)
    }
};
        locals.var_thesateff__blk1447 = assign51420_e66016;
        locals.var_thesateff__blk1447_dn4 = assign51420_e66016_d_n4;
        locals.var_thesateff__blk1447_dn6 = assign51420_e66016_d_n6;
        locals.var_thesateff__blk1447_dn7 = assign51420_e66016_d_n7;
        locals.var_thesateff__blk1447_dn8 = assign51420_e66016_d_n8;
        locals.var_thesateff__blk1447_dn9 = assign51420_e66016_d_n9;

        let (assign51430_e66020, assign51430_e66020_d_n4, assign51430_e66020_d_n6, assign51430_e66020_d_n7, assign51430_e66020_d_n8, assign51430_e66020_d_n9,) = {
    if (locals.var_guard1473 != 0.0) {
        (locals.var_qeff1__blk1442, locals.var_qeff1__blk1442_dn4, locals.var_qeff1__blk1442_dn6, locals.var_qeff1__blk1442_dn7, locals.var_qeff1__blk1442_dn8, locals.var_qeff1__blk1442_dn9,)
    } else {
        (locals.var_voxm__blk1446, locals.var_voxm__blk1446_dn4, locals.var_voxm__blk1446_dn6, locals.var_voxm__blk1446_dn7, locals.var_voxm__blk1446_dn8, locals.var_voxm__blk1446_dn9,)
    }
};
        locals.var_voxm__blk1446 = assign51430_e66020;
        locals.var_voxm__blk1446_dn4 = assign51430_e66020_d_n4;
        locals.var_voxm__blk1446_dn6 = assign51430_e66020_d_n6;
        locals.var_voxm__blk1446_dn7 = assign51430_e66020_d_n7;
        locals.var_voxm__blk1446_dn8 = assign51430_e66020_d_n8;
        locals.var_voxm__blk1446_dn9 = assign51430_e66020_d_n9;

        let assign51440_e66023: f64 = if locals.var_xg__blk1343 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1501 = assign51440_e66023;

        let assign51450_e66026: f64 = if locals.var_ds__blk1370 > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1502 = assign51450_e66026;

        let (assign51460_e66036, assign51460_e66036_d_n4, assign51460_e66036_d_n6, assign51460_e66036_d_n7, assign51460_e66036_d_n8, assign51460_e66036_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51460_e66034: f64 = (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386);
        (assign51460_e66034, ((locals.var_thesatloc__blk1319_dn4 * locals.var_factheta__blk1386) + (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn4)), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn6), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn7), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn8), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn9),)
    } else {
        (locals.var_thesateff__blk1447, locals.var_thesateff__blk1447_dn4, locals.var_thesateff__blk1447_dn6, locals.var_thesateff__blk1447_dn7, locals.var_thesateff__blk1447_dn8, locals.var_thesateff__blk1447_dn9,)
    }
};
        locals.var_thesateff__blk1447 = assign51460_e66036;
        locals.var_thesateff__blk1447_dn4 = assign51460_e66036_d_n4;
        locals.var_thesateff__blk1447_dn6 = assign51460_e66036_d_n6;
        locals.var_thesateff__blk1447_dn7 = assign51460_e66036_d_n7;
        locals.var_thesateff__blk1447_dn8 = assign51460_e66036_d_n8;
        locals.var_thesateff__blk1447_dn9 = assign51460_e66036_d_n9;

        let (assign51470_e66046, assign51470_e66046_d_n4, assign51470_e66046_d_n6, assign51470_e66046_d_n7, assign51470_e66046_d_n8, assign51470_e66046_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51470_e66044: f64 = (locals.var_thesateff__blk1447 / locals.var_gmobs__blk1383);
        (assign51470_e66044, (((locals.var_thesateff__blk1447_dn4 * locals.var_gmobs__blk1383) - (locals.var_thesateff__blk1447 * locals.var_gmobs__blk1383_dn4)) / (locals.var_gmobs__blk1383 * locals.var_gmobs__blk1383)), (((locals.var_thesateff__blk1447_dn6 * locals.var_gmobs__blk1383) - (locals.var_thesateff__blk1447 * locals.var_gmobs__blk1383_dn6)) / (locals.var_gmobs__blk1383 * locals.var_gmobs__blk1383)), (((locals.var_thesateff__blk1447_dn7 * locals.var_gmobs__blk1383) - (locals.var_thesateff__blk1447 * locals.var_gmobs__blk1383_dn7)) / (locals.var_gmobs__blk1383 * locals.var_gmobs__blk1383)), (((locals.var_thesateff__blk1447_dn8 * locals.var_gmobs__blk1383) - (locals.var_thesateff__blk1447 * locals.var_gmobs__blk1383_dn8)) / (locals.var_gmobs__blk1383 * locals.var_gmobs__blk1383)), (((locals.var_thesateff__blk1447_dn9 * locals.var_gmobs__blk1383) - (locals.var_thesateff__blk1447 * locals.var_gmobs__blk1383_dn9)) / (locals.var_gmobs__blk1383 * locals.var_gmobs__blk1383)),)
    } else {
        (locals.var_thesat1__blk1388, locals.var_thesat1__blk1388_dn4, locals.var_thesat1__blk1388_dn6, locals.var_thesat1__blk1388_dn7, locals.var_thesat1__blk1388_dn8, locals.var_thesat1__blk1388_dn9,)
    }
};
        locals.var_thesat1__blk1388 = assign51470_e66046;
        locals.var_thesat1__blk1388_dn4 = assign51470_e66046_d_n4;
        locals.var_thesat1__blk1388_dn6 = assign51470_e66046_d_n6;
        locals.var_thesat1__blk1388_dn7 = assign51470_e66046_d_n7;
        locals.var_thesat1__blk1388_dn8 = assign51470_e66046_d_n8;
        locals.var_thesat1__blk1388_dn9 = assign51470_e66046_d_n9;

        let (assign51480_e66058, assign51480_e66058_d_n4, assign51480_e66058_d_n6, assign51480_e66058_d_n7, assign51480_e66058_d_n8, assign51480_e66058_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51480_e66055: f64 = (0.5 * locals.var_gf2__blk1325);
        let assign51480_e66056: f64 = (locals.var_xgs__blk1375 + assign51480_e66055);
        (assign51480_e66056, (locals.var_xgs__blk1375_dn4 + (0.5 * locals.var_gf2__blk1325_dn4)), (locals.var_xgs__blk1375_dn6 + (0.5 * locals.var_gf2__blk1325_dn6)), (locals.var_xgs__blk1375_dn7 + (0.5 * locals.var_gf2__blk1325_dn7)), (locals.var_xgs__blk1375_dn8 + (0.5 * locals.var_gf2__blk1325_dn8)), (locals.var_xgs__blk1375_dn9 + (0.5 * locals.var_gf2__blk1325_dn9)),)
    } else {
        (locals.var_asat__blk1389, locals.var_asat__blk1389_dn4, locals.var_asat__blk1389_dn6, locals.var_asat__blk1389_dn7, locals.var_asat__blk1389_dn8, locals.var_asat__blk1389_dn9,)
    }
};
        locals.var_asat__blk1389 = assign51480_e66058;
        locals.var_asat__blk1389_dn4 = assign51480_e66058_d_n4;
        locals.var_asat__blk1389_dn6 = assign51480_e66058_d_n6;
        locals.var_asat__blk1389_dn7 = assign51480_e66058_d_n7;
        locals.var_asat__blk1389_dn8 = assign51480_e66058_d_n8;
        locals.var_asat__blk1389_dn9 = assign51480_e66058_d_n9;

        let (assign51490_e66072, assign51490_e66072_d_n4, assign51490_e66072_d_n6, assign51490_e66072_d_n7, assign51490_e66072_d_n8, assign51490_e66072_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51490_e66066: f64 = (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_asat__blk1389;
        let assign51490_e66068: f64 = (assign51490_e66066 * __rspice_inv_cse_0);
        let assign51490_e66070: f64 = (assign51490_e66068 * __rspice_inv_cse_0);
        (assign51490_e66070, ((((((((locals.var_gf2__blk1325_dn4 * locals.var_delta_1s__blk1368) + (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368_dn4)) * locals.var_asat__blk1389) - (assign51490_e66066 * locals.var_asat__blk1389_dn4)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)) * locals.var_asat__blk1389) - (assign51490_e66068 * locals.var_asat__blk1389_dn4)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)), ((((((((locals.var_gf2__blk1325_dn6 * locals.var_delta_1s__blk1368) + (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368_dn6)) * locals.var_asat__blk1389) - (assign51490_e66066 * locals.var_asat__blk1389_dn6)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)) * locals.var_asat__blk1389) - (assign51490_e66068 * locals.var_asat__blk1389_dn6)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)), ((((((((locals.var_gf2__blk1325_dn7 * locals.var_delta_1s__blk1368) + (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368_dn7)) * locals.var_asat__blk1389) - (assign51490_e66066 * locals.var_asat__blk1389_dn7)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)) * locals.var_asat__blk1389) - (assign51490_e66068 * locals.var_asat__blk1389_dn7)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)), ((((((((locals.var_gf2__blk1325_dn8 * locals.var_delta_1s__blk1368) + (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368_dn8)) * locals.var_asat__blk1389) - (assign51490_e66066 * locals.var_asat__blk1389_dn8)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)) * locals.var_asat__blk1389) - (assign51490_e66068 * locals.var_asat__blk1389_dn8)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)), ((((((((locals.var_gf2__blk1325_dn9 * locals.var_delta_1s__blk1368) + (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368_dn9)) * locals.var_asat__blk1389) - (assign51490_e66066 * locals.var_asat__blk1389_dn9)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)) * locals.var_asat__blk1389) - (assign51490_e66068 * locals.var_asat__blk1389_dn9)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51490_e66072;
        locals.var_temp__blk949_dn4 = assign51490_e66072_d_n4;
        locals.var_temp__blk949_dn6 = assign51490_e66072_d_n6;
        locals.var_temp__blk949_dn7 = assign51490_e66072_d_n7;
        locals.var_temp__blk949_dn8 = assign51490_e66072_d_n8;
        locals.var_temp__blk949_dn9 = assign51490_e66072_d_n9;

        let assign51500_e66075: f64 = if locals.var_temp__blk949 > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1503 = assign51500_e66075;

        let (assign51510_e66087, assign51510_e66087_d_n4, assign51510_e66087_d_n6, assign51510_e66087_d_n7, assign51510_e66087_d_n8, assign51510_e66087_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign51510_e66085: f64 = (1.0 - locals.var_temp__blk949);
        (assign51510_e66085, (-locals.var_temp__blk949_dn4), (-locals.var_temp__blk949_dn6), (-locals.var_temp__blk949_dn7), (-locals.var_temp__blk949_dn8), (-locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign51510_e66087;
        locals.var_temp1_dn4 = assign51510_e66087_d_n4;
        locals.var_temp1_dn6 = assign51510_e66087_d_n6;
        locals.var_temp1_dn7 = assign51510_e66087_d_n7;
        locals.var_temp1_dn8 = assign51510_e66087_d_n8;
        locals.var_temp1_dn9 = assign51510_e66087_d_n9;

        let assign51520_e66090: f64 = if locals.var_temp1 < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1504 = assign51520_e66090;

        let (assign51530_e66102, assign51530_e66102_d_n4, assign51530_e66102_d_n6, assign51530_e66102_d_n7, assign51530_e66102_d_n8, assign51530_e66102_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign51530_e66102;
        locals.var_temp2_dn4 = assign51530_e66102_d_n4;
        locals.var_temp2_dn6 = assign51530_e66102_d_n6;
        locals.var_temp2_dn7 = assign51530_e66102_d_n7;
        locals.var_temp2_dn8 = assign51530_e66102_d_n8;
        locals.var_temp2_dn9 = assign51530_e66102_d_n9;

        let (assign51540_e66118, assign51540_e66118_d_n4, assign51540_e66118_d_n6, assign51540_e66118_d_n7, assign51540_e66118_d_n8, assign51540_e66118_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 == 0.0)) {
        let assign51540_e66115: f64 = (locals.var_temp1).sqrt();
        let assign51540_e66116: f64 = (1.0 - assign51540_e66115);
        (assign51540_e66116, (-(locals.var_temp1_dn4 / (2.0 * assign51540_e66115))), (-(locals.var_temp1_dn6 / (2.0 * assign51540_e66115))), (-(locals.var_temp1_dn7 / (2.0 * assign51540_e66115))), (-(locals.var_temp1_dn8 / (2.0 * assign51540_e66115))), (-(locals.var_temp1_dn9 / (2.0 * assign51540_e66115))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign51540_e66118;
        locals.var_temp2_dn4 = assign51540_e66118_d_n4;
        locals.var_temp2_dn6 = assign51540_e66118_d_n6;
        locals.var_temp2_dn7 = assign51540_e66118_d_n7;
        locals.var_temp2_dn8 = assign51540_e66118_d_n8;
        locals.var_temp2_dn9 = assign51540_e66118_d_n9;

        let (assign51550_e66131, assign51550_e66131_d_n4, assign51550_e66131_d_n6, assign51550_e66131_d_n7, assign51550_e66131_d_n8, assign51550_e66131_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1503 == 0.0)) {
        let assign51550_e66129: f64 = (0.5 * locals.var_temp__blk949);
        (assign51550_e66129, (0.5 * locals.var_temp__blk949_dn4), (0.5 * locals.var_temp__blk949_dn6), (0.5 * locals.var_temp__blk949_dn7), (0.5 * locals.var_temp__blk949_dn8), (0.5 * locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign51550_e66131;
        locals.var_temp2_dn4 = assign51550_e66131_d_n4;
        locals.var_temp2_dn6 = assign51550_e66131_d_n6;
        locals.var_temp2_dn7 = assign51550_e66131_d_n7;
        locals.var_temp2_dn8 = assign51550_e66131_d_n8;
        locals.var_temp2_dn9 = assign51550_e66131_d_n9;

        let (assign51560_e66141, assign51560_e66141_d_n4, assign51560_e66141_d_n6, assign51560_e66141_d_n7, assign51560_e66141_d_n8, assign51560_e66141_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51560_e66139: f64 = (locals.var_temp2 * locals.var_asat__blk1389);
        (assign51560_e66139, ((locals.var_temp2_dn4 * locals.var_asat__blk1389) + (locals.var_temp2 * locals.var_asat__blk1389_dn4)), ((locals.var_temp2_dn6 * locals.var_asat__blk1389) + (locals.var_temp2 * locals.var_asat__blk1389_dn6)), ((locals.var_temp2_dn7 * locals.var_asat__blk1389) + (locals.var_temp2 * locals.var_asat__blk1389_dn7)), ((locals.var_temp2_dn8 * locals.var_asat__blk1389) + (locals.var_temp2 * locals.var_asat__blk1389_dn8)), ((locals.var_temp2_dn9 * locals.var_asat__blk1389) + (locals.var_temp2 * locals.var_asat__blk1389_dn9)),)
    } else {
        (locals.var_x_inf0__blk1390, locals.var_x_inf0__blk1390_dn4, locals.var_x_inf0__blk1390_dn6, locals.var_x_inf0__blk1390_dn7, locals.var_x_inf0__blk1390_dn8, locals.var_x_inf0__blk1390_dn9,)
    }
};
        locals.var_x_inf0__blk1390 = assign51560_e66141;
        locals.var_x_inf0__blk1390_dn4 = assign51560_e66141_d_n4;
        locals.var_x_inf0__blk1390_dn6 = assign51560_e66141_d_n6;
        locals.var_x_inf0__blk1390_dn7 = assign51560_e66141_d_n7;
        locals.var_x_inf0__blk1390_dn8 = assign51560_e66141_d_n8;
        locals.var_x_inf0__blk1390_dn9 = assign51560_e66141_d_n9;

        let assign51570_e66148: f64 = if ((locals.var_cs_t > 0.0) && (locals.var_thecs_t > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1505 = assign51570_e66148;

        let (assign51580_e66162, assign51580_e66162_d_n4, assign51580_e66162_d_n6, assign51580_e66162_d_n7, assign51580_e66162_d_n8, assign51580_e66162_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51580_e66158: f64 = (0.475 * locals.var_phit1__blk1339);
        let assign51580_e66160: f64 = (assign51580_e66158 * locals.var_x_inf0__blk1390);
        (assign51580_e66160, (((0.475 * locals.var_phit1__blk1339_dn4) * locals.var_x_inf0__blk1390) + (assign51580_e66158 * locals.var_x_inf0__blk1390_dn4)), (((0.475 * locals.var_phit1__blk1339_dn6) * locals.var_x_inf0__blk1390) + (assign51580_e66158 * locals.var_x_inf0__blk1390_dn6)), (((0.475 * locals.var_phit1__blk1339_dn7) * locals.var_x_inf0__blk1390) + (assign51580_e66158 * locals.var_x_inf0__blk1390_dn7)), (((0.475 * locals.var_phit1__blk1339_dn8) * locals.var_x_inf0__blk1390) + (assign51580_e66158 * locals.var_x_inf0__blk1390_dn8)), (((0.475 * locals.var_phit1__blk1339_dn9) * locals.var_x_inf0__blk1390) + (assign51580_e66158 * locals.var_x_inf0__blk1390_dn9)),)
    } else {
        (locals.var_midphi0__blk1391, locals.var_midphi0__blk1391_dn4, locals.var_midphi0__blk1391_dn6, locals.var_midphi0__blk1391_dn7, locals.var_midphi0__blk1391_dn8, locals.var_midphi0__blk1391_dn9,)
    }
};
        locals.var_midphi0__blk1391 = assign51580_e66162;
        locals.var_midphi0__blk1391_dn4 = assign51580_e66162_d_n4;
        locals.var_midphi0__blk1391_dn6 = assign51580_e66162_d_n6;
        locals.var_midphi0__blk1391_dn7 = assign51580_e66162_d_n7;
        locals.var_midphi0__blk1391_dn8 = assign51580_e66162_d_n8;
        locals.var_midphi0__blk1391_dn9 = assign51580_e66162_d_n9;

        let (assign51590_e66176, assign51590_e66176_d_n4, assign51590_e66176_d_n6, assign51590_e66176_d_n7, assign51590_e66176_d_n8, assign51590_e66176_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51590_e66173: f64 = (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391);
        let assign51590_e66174: f64 = (locals.var_qis__blk1376 - assign51590_e66173);
        (assign51590_e66174, (locals.var_qis__blk1376_dn4 - ((locals.var_alphas__blk1373_dn4 * locals.var_midphi0__blk1391) + (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391_dn4))), (locals.var_qis__blk1376_dn6 - ((locals.var_alphas__blk1373_dn6 * locals.var_midphi0__blk1391) + (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391_dn6))), (locals.var_qis__blk1376_dn7 - ((locals.var_alphas__blk1373_dn7 * locals.var_midphi0__blk1391) + (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391_dn7))), (locals.var_qis__blk1376_dn8 - ((locals.var_alphas__blk1373_dn8 * locals.var_midphi0__blk1391) + (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391_dn8))), (locals.var_qis__blk1376_dn9 - ((locals.var_alphas__blk1373_dn9 * locals.var_midphi0__blk1391) + (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391_dn9))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51590_e66176;
        locals.var_temp__blk949_dn4 = assign51590_e66176_d_n4;
        locals.var_temp__blk949_dn6 = assign51590_e66176_d_n6;
        locals.var_temp__blk949_dn7 = assign51590_e66176_d_n7;
        locals.var_temp__blk949_dn8 = assign51590_e66176_d_n8;
        locals.var_temp__blk949_dn9 = assign51590_e66176_d_n9;

        let (assign51600_e66195, assign51600_e66195_d_n4, assign51600_e66195_d_n6, assign51600_e66195_d_n7, assign51600_e66195_d_n8, assign51600_e66195_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51600_e66188: f64 = (locals.var_temp__blk949 * locals.var_temp__blk949);
        let assign51600_e66190: f64 = (assign51600_e66188 + 1e-12);
        let assign51600_e66191: f64 = (assign51600_e66190).sqrt();
        let assign51600_e66192: f64 = (locals.var_temp__blk949 + assign51600_e66191);
        let assign51600_e66193: f64 = (0.5 * assign51600_e66192);
        (assign51600_e66193, (0.5 * (locals.var_temp__blk949_dn4 + (((locals.var_temp__blk949_dn4 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn4)) / (2.0 * assign51600_e66191)))), (0.5 * (locals.var_temp__blk949_dn6 + (((locals.var_temp__blk949_dn6 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn6)) / (2.0 * assign51600_e66191)))), (0.5 * (locals.var_temp__blk949_dn7 + (((locals.var_temp__blk949_dn7 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn7)) / (2.0 * assign51600_e66191)))), (0.5 * (locals.var_temp__blk949_dn8 + (((locals.var_temp__blk949_dn8 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn8)) / (2.0 * assign51600_e66191)))), (0.5 * (locals.var_temp__blk949_dn9 + (((locals.var_temp__blk949_dn9 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn9)) / (2.0 * assign51600_e66191)))),)
    } else {
        (locals.var_qisat__blk1392, locals.var_qisat__blk1392_dn4, locals.var_qisat__blk1392_dn6, locals.var_qisat__blk1392_dn7, locals.var_qisat__blk1392_dn8, locals.var_qisat__blk1392_dn9,)
    }
};
        locals.var_qisat__blk1392 = assign51600_e66195;
        locals.var_qisat__blk1392_dn4 = assign51600_e66195_d_n4;
        locals.var_qisat__blk1392_dn6 = assign51600_e66195_d_n6;
        locals.var_qisat__blk1392_dn7 = assign51600_e66195_d_n7;
        locals.var_qisat__blk1392_dn8 = assign51600_e66195_d_n8;
        locals.var_qisat__blk1392_dn9 = assign51600_e66195_d_n9;

        let (assign51610_e66215, assign51610_e66215_d_n4, assign51610_e66215_d_n6, assign51610_e66215_d_n7, assign51610_e66215_d_n8, assign51610_e66215_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51610_e66205: f64 = (locals.var_phit1__blk1339 * locals.var_xgs__blk1375);
        let assign51610_e66207: f64 = (assign51610_e66205 - locals.var_qis__blk1376);
        let assign51610_e66210: f64 = (locals.var_alphas__blk1373 - 1.0);
        let assign51610_e66212: f64 = (assign51610_e66210 * locals.var_midphi0__blk1391);
        let assign51610_e66213: f64 = (assign51610_e66207 + assign51610_e66212);
        (assign51610_e66213, ((((locals.var_phit1__blk1339_dn4 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn4)) - locals.var_qis__blk1376_dn4) + ((locals.var_alphas__blk1373_dn4 * locals.var_midphi0__blk1391) + (assign51610_e66210 * locals.var_midphi0__blk1391_dn4))), ((((locals.var_phit1__blk1339_dn6 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn6)) - locals.var_qis__blk1376_dn6) + ((locals.var_alphas__blk1373_dn6 * locals.var_midphi0__blk1391) + (assign51610_e66210 * locals.var_midphi0__blk1391_dn6))), ((((locals.var_phit1__blk1339_dn7 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn7)) - locals.var_qis__blk1376_dn7) + ((locals.var_alphas__blk1373_dn7 * locals.var_midphi0__blk1391) + (assign51610_e66210 * locals.var_midphi0__blk1391_dn7))), ((((locals.var_phit1__blk1339_dn8 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn8)) - locals.var_qis__blk1376_dn8) + ((locals.var_alphas__blk1373_dn8 * locals.var_midphi0__blk1391) + (assign51610_e66210 * locals.var_midphi0__blk1391_dn8))), ((((locals.var_phit1__blk1339_dn9 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn9)) - locals.var_qis__blk1376_dn9) + ((locals.var_alphas__blk1373_dn9 * locals.var_midphi0__blk1391) + (assign51610_e66210 * locals.var_midphi0__blk1391_dn9))),)
    } else {
        (locals.var_qbsat__blk1393, locals.var_qbsat__blk1393_dn4, locals.var_qbsat__blk1393_dn6, locals.var_qbsat__blk1393_dn7, locals.var_qbsat__blk1393_dn8, locals.var_qbsat__blk1393_dn9,)
    }
};
        locals.var_qbsat__blk1393 = assign51610_e66215;
        locals.var_qbsat__blk1393_dn4 = assign51610_e66215_d_n4;
        locals.var_qbsat__blk1393_dn6 = assign51610_e66215_d_n6;
        locals.var_qbsat__blk1393_dn7 = assign51610_e66215_d_n7;
        locals.var_qbsat__blk1393_dn8 = assign51610_e66215_d_n8;
        locals.var_qbsat__blk1393_dn9 = assign51610_e66215_d_n9;

        let (assign51620_e66233, assign51620_e66233_d_n4, assign51620_e66233_d_n6, assign51620_e66233_d_n7, assign51620_e66233_d_n8, assign51620_e66233_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51620_e66226: f64 = (0.5 * locals.var_gf2__blk1325);
        let assign51620_e66228: f64 = (assign51620_e66226 * locals.var_phit1__blk1339);
        let assign51620_e66230: f64 = (assign51620_e66228 / locals.var_qbsat__blk1393);
        let assign51620_e66231: f64 = (1.0 + assign51620_e66230);
        (assign51620_e66231, ((((((0.5 * locals.var_gf2__blk1325_dn4) * locals.var_phit1__blk1339) + (assign51620_e66226 * locals.var_phit1__blk1339_dn4)) * locals.var_qbsat__blk1393) - (assign51620_e66228 * locals.var_qbsat__blk1393_dn4)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), ((((((0.5 * locals.var_gf2__blk1325_dn6) * locals.var_phit1__blk1339) + (assign51620_e66226 * locals.var_phit1__blk1339_dn6)) * locals.var_qbsat__blk1393) - (assign51620_e66228 * locals.var_qbsat__blk1393_dn6)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), ((((((0.5 * locals.var_gf2__blk1325_dn7) * locals.var_phit1__blk1339) + (assign51620_e66226 * locals.var_phit1__blk1339_dn7)) * locals.var_qbsat__blk1393) - (assign51620_e66228 * locals.var_qbsat__blk1393_dn7)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), ((((((0.5 * locals.var_gf2__blk1325_dn8) * locals.var_phit1__blk1339) + (assign51620_e66226 * locals.var_phit1__blk1339_dn8)) * locals.var_qbsat__blk1393) - (assign51620_e66228 * locals.var_qbsat__blk1393_dn8)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), ((((((0.5 * locals.var_gf2__blk1325_dn9) * locals.var_phit1__blk1339) + (assign51620_e66226 * locals.var_phit1__blk1339_dn9)) * locals.var_qbsat__blk1393) - (assign51620_e66228 * locals.var_qbsat__blk1393_dn9)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)),)
    } else {
        (locals.var_alphasat__blk1394, locals.var_alphasat__blk1394_dn4, locals.var_alphasat__blk1394_dn6, locals.var_alphasat__blk1394_dn7, locals.var_alphasat__blk1394_dn8, locals.var_alphasat__blk1394_dn9,)
    }
};
        locals.var_alphasat__blk1394 = assign51620_e66233;
        locals.var_alphasat__blk1394_dn4 = assign51620_e66233_d_n4;
        locals.var_alphasat__blk1394_dn6 = assign51620_e66233_d_n6;
        locals.var_alphasat__blk1394_dn7 = assign51620_e66233_d_n7;
        locals.var_alphasat__blk1394_dn8 = assign51620_e66233_d_n8;
        locals.var_alphasat__blk1394_dn9 = assign51620_e66233_d_n9;

        let (assign51630_e66247, assign51630_e66247_d_n4, assign51630_e66247_d_n6, assign51630_e66247_d_n7, assign51630_e66247_d_n8, assign51630_e66247_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51630_e66244: f64 = (locals.var_eta_mu * locals.var_qisat__blk1392);
        let assign51630_e66245: f64 = (locals.var_qbsat__blk1393 + assign51630_e66244);
        (assign51630_e66245, (locals.var_qbsat__blk1393_dn4 + (locals.var_eta_mu * locals.var_qisat__blk1392_dn4)), (locals.var_qbsat__blk1393_dn6 + (locals.var_eta_mu * locals.var_qisat__blk1392_dn6)), (locals.var_qbsat__blk1393_dn7 + (locals.var_eta_mu * locals.var_qisat__blk1392_dn7)), (locals.var_qbsat__blk1393_dn8 + (locals.var_eta_mu * locals.var_qisat__blk1392_dn8)), (locals.var_qbsat__blk1393_dn9 + (locals.var_eta_mu * locals.var_qisat__blk1392_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51630_e66247;
        locals.var_temp__blk949_dn4 = assign51630_e66247_d_n4;
        locals.var_temp__blk949_dn6 = assign51630_e66247_d_n6;
        locals.var_temp__blk949_dn7 = assign51630_e66247_d_n7;
        locals.var_temp__blk949_dn8 = assign51630_e66247_d_n8;
        locals.var_temp__blk949_dn9 = assign51630_e66247_d_n9;

        let (assign51640_e66263, assign51640_e66263_d_n4, assign51640_e66263_d_n6, assign51640_e66263_d_n7, assign51640_e66263_d_n8, assign51640_e66263_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51640_e66257: f64 = (locals.var_e_eff0 * locals.var_temp__blk949);
        let assign51640_e66259: f64 = (assign51640_e66257 * locals.var_mue_t);
        let assign51640_e66261: f64 = (assign51640_e66259).powf(locals.var_themu_t);
        (assign51640_e66261, if locals.var_themu_t_dn4 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51640_e66259).powf(locals.var_themu_t - 1.0) * (((locals.var_e_eff0 * locals.var_temp__blk949_dn4) * locals.var_mue_t) + (assign51640_e66257 * locals.var_mue_t_dn4)))) } } else { (assign51640_e66261 * ((locals.var_themu_t_dn4 * (assign51640_e66259).ln()) + (locals.var_themu_t * ((((locals.var_e_eff0 * locals.var_temp__blk949_dn4) * locals.var_mue_t) + (assign51640_e66257 * locals.var_mue_t_dn4)) / assign51640_e66259)))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51640_e66259).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn6) * locals.var_mue_t))) } } else { (assign51640_e66261 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn6) * locals.var_mue_t) / assign51640_e66259))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51640_e66259).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn7) * locals.var_mue_t))) } } else { (assign51640_e66261 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn7) * locals.var_mue_t) / assign51640_e66259))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51640_e66259).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn8) * locals.var_mue_t))) } } else { (assign51640_e66261 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn8) * locals.var_mue_t) / assign51640_e66259))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51640_e66259).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn9) * locals.var_mue_t))) } } else { (assign51640_e66261 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn9) * locals.var_mue_t) / assign51640_e66259))) },)
    } else {
        (locals.var_gmobmusat__blk1395, locals.var_gmobmusat__blk1395_dn4, locals.var_gmobmusat__blk1395_dn6, locals.var_gmobmusat__blk1395_dn7, locals.var_gmobmusat__blk1395_dn8, locals.var_gmobmusat__blk1395_dn9,)
    }
};
        locals.var_gmobmusat__blk1395 = assign51640_e66263;
        locals.var_gmobmusat__blk1395_dn4 = assign51640_e66263_d_n4;
        locals.var_gmobmusat__blk1395_dn6 = assign51640_e66263_d_n6;
        locals.var_gmobmusat__blk1395_dn7 = assign51640_e66263_d_n7;
        locals.var_gmobmusat__blk1395_dn8 = assign51640_e66263_d_n8;
        locals.var_gmobmusat__blk1395_dn9 = assign51640_e66263_d_n9;

        let (assign51650_e66285, assign51650_e66285_d_n4, assign51650_e66285_d_n6, assign51650_e66285_d_n7, assign51650_e66285_d_n8, assign51650_e66285_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51650_e66275: f64 = (1.0 - locals.var_eta_mu);
        let assign51650_e66276: f64 = (locals.var_alphasat__blk1394 * assign51650_e66275);
        let assign51650_e66278: f64 = (assign51650_e66276 - 1.0);
        let assign51650_e66279: f64 = (locals.var_themu_t * assign51650_e66278);
        let assign51650_e66281: f64 = (assign51650_e66279 / locals.var_temp__blk949);
        let assign51650_e66283: f64 = (assign51650_e66281 * locals.var_gmobmusat__blk1395);
        (assign51650_e66283, (((((((locals.var_themu_t_dn4 * assign51650_e66278) + (locals.var_themu_t * (locals.var_alphasat__blk1394_dn4 * assign51650_e66275))) * locals.var_temp__blk949) - (assign51650_e66279 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat__blk1395) + (assign51650_e66281 * locals.var_gmobmusat__blk1395_dn4)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1394_dn6 * assign51650_e66275)) * locals.var_temp__blk949) - (assign51650_e66279 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat__blk1395) + (assign51650_e66281 * locals.var_gmobmusat__blk1395_dn6)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1394_dn7 * assign51650_e66275)) * locals.var_temp__blk949) - (assign51650_e66279 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat__blk1395) + (assign51650_e66281 * locals.var_gmobmusat__blk1395_dn7)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1394_dn8 * assign51650_e66275)) * locals.var_temp__blk949) - (assign51650_e66279 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat__blk1395) + (assign51650_e66281 * locals.var_gmobmusat__blk1395_dn8)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1394_dn9 * assign51650_e66275)) * locals.var_temp__blk949) - (assign51650_e66279 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat__blk1395) + (assign51650_e66281 * locals.var_gmobmusat__blk1395_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign51650_e66285;
        locals.var_temp1_dn4 = assign51650_e66285_d_n4;
        locals.var_temp1_dn6 = assign51650_e66285_d_n6;
        locals.var_temp1_dn7 = assign51650_e66285_d_n7;
        locals.var_temp1_dn8 = assign51650_e66285_d_n8;
        locals.var_temp1_dn9 = assign51650_e66285_d_n9;

        let (assign51660_e66297, assign51660_e66297_d_n4, assign51660_e66297_d_n6, assign51660_e66297_d_n7, assign51660_e66297_d_n8, assign51660_e66297_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51660_e66295: f64 = (locals.var_qisat__blk1392 / locals.var_qbsat__blk1393);
        (assign51660_e66295, (((locals.var_qisat__blk1392_dn4 * locals.var_qbsat__blk1393) - (locals.var_qisat__blk1392 * locals.var_qbsat__blk1393_dn4)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), (((locals.var_qisat__blk1392_dn6 * locals.var_qbsat__blk1393) - (locals.var_qisat__blk1392 * locals.var_qbsat__blk1393_dn6)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), (((locals.var_qisat__blk1392_dn7 * locals.var_qbsat__blk1393) - (locals.var_qisat__blk1392 * locals.var_qbsat__blk1393_dn7)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), (((locals.var_qisat__blk1392_dn8 * locals.var_qbsat__blk1393) - (locals.var_qisat__blk1392 * locals.var_qbsat__blk1393_dn8)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), (((locals.var_qisat__blk1392_dn9 * locals.var_qbsat__blk1393) - (locals.var_qisat__blk1392 * locals.var_qbsat__blk1393_dn9)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51660_e66297;
        locals.var_temp__blk949_dn4 = assign51660_e66297_d_n4;
        locals.var_temp__blk949_dn6 = assign51660_e66297_d_n6;
        locals.var_temp__blk949_dn7 = assign51660_e66297_d_n7;
        locals.var_temp__blk949_dn8 = assign51660_e66297_d_n8;
        locals.var_temp__blk949_dn9 = assign51660_e66297_d_n9;

        let (assign51670_e66314, assign51670_e66314_d_n4, assign51670_e66314_d_n6, assign51670_e66314_d_n7, assign51670_e66314_d_n8, assign51670_e66314_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51670_e66308: f64 = (1.0 + locals.var_temp__blk949);
        let assign51670_e66310: f64 = (-locals.var_thecs_t);
        let assign51670_e66311: f64 = (assign51670_e66308).powf(assign51670_e66310);
        let assign51670_e66312: f64 = (locals.var_cs_t * assign51670_e66311);
        (assign51670_e66312, ((locals.var_cs_t_dn4 * assign51670_e66311) + (locals.var_cs_t * if (-locals.var_thecs_t_dn4) == 0.0 && ((assign51670_e66310) as f64).is_finite() && ((assign51670_e66310) as f64).fract() == 0.0 { if assign51670_e66310 == 0.0 { 0.0 } else { (assign51670_e66310 * ((assign51670_e66308).powf(assign51670_e66310 - 1.0) * locals.var_temp__blk949_dn4)) } } else { (assign51670_e66311 * (((-locals.var_thecs_t_dn4) * (assign51670_e66308).ln()) + (assign51670_e66310 * (locals.var_temp__blk949_dn4 / assign51670_e66308)))) })), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51670_e66310) as f64).is_finite() && ((assign51670_e66310) as f64).fract() == 0.0 { if assign51670_e66310 == 0.0 { 0.0 } else { (assign51670_e66310 * ((assign51670_e66308).powf(assign51670_e66310 - 1.0) * locals.var_temp__blk949_dn6)) } } else { (assign51670_e66311 * (assign51670_e66310 * (locals.var_temp__blk949_dn6 / assign51670_e66308))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51670_e66310) as f64).is_finite() && ((assign51670_e66310) as f64).fract() == 0.0 { if assign51670_e66310 == 0.0 { 0.0 } else { (assign51670_e66310 * ((assign51670_e66308).powf(assign51670_e66310 - 1.0) * locals.var_temp__blk949_dn7)) } } else { (assign51670_e66311 * (assign51670_e66310 * (locals.var_temp__blk949_dn7 / assign51670_e66308))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51670_e66310) as f64).is_finite() && ((assign51670_e66310) as f64).fract() == 0.0 { if assign51670_e66310 == 0.0 { 0.0 } else { (assign51670_e66310 * ((assign51670_e66308).powf(assign51670_e66310 - 1.0) * locals.var_temp__blk949_dn8)) } } else { (assign51670_e66311 * (assign51670_e66310 * (locals.var_temp__blk949_dn8 / assign51670_e66308))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51670_e66310) as f64).is_finite() && ((assign51670_e66310) as f64).fract() == 0.0 { if assign51670_e66310 == 0.0 { 0.0 } else { (assign51670_e66310 * ((assign51670_e66308).powf(assign51670_e66310 - 1.0) * locals.var_temp__blk949_dn9)) } } else { (assign51670_e66311 * (assign51670_e66310 * (locals.var_temp__blk949_dn9 / assign51670_e66308))) }),)
    } else {
        (locals.var_gmobcssat__blk1396, locals.var_gmobcssat__blk1396_dn4, locals.var_gmobcssat__blk1396_dn6, locals.var_gmobcssat__blk1396_dn7, locals.var_gmobcssat__blk1396_dn8, locals.var_gmobcssat__blk1396_dn9,)
    }
};
        locals.var_gmobcssat__blk1396 = assign51670_e66314;
        locals.var_gmobcssat__blk1396_dn4 = assign51670_e66314_d_n4;
        locals.var_gmobcssat__blk1396_dn6 = assign51670_e66314_d_n6;
        locals.var_gmobcssat__blk1396_dn7 = assign51670_e66314_d_n7;
        locals.var_gmobcssat__blk1396_dn8 = assign51670_e66314_d_n8;
        locals.var_gmobcssat__blk1396_dn9 = assign51670_e66314_d_n9;

        let (assign51680_e66338, assign51680_e66338_d_n4, assign51680_e66338_d_n6, assign51680_e66338_d_n7, assign51680_e66338_d_n8, assign51680_e66338_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51680_e66325: f64 = (locals.var_alphasat__blk1394 - 1.0);
        let assign51680_e66329: f64 = (locals.var_temp__blk949 + 1.0);
        let assign51680_e66330: f64 = (1.0 / assign51680_e66329);
        let assign51680_e66331: f64 = (assign51680_e66325 + assign51680_e66330);
        let assign51680_e66332: f64 = (locals.var_thecs_t * assign51680_e66331);
        let assign51680_e66334: f64 = (assign51680_e66332 / locals.var_qbsat__blk1393);
        let assign51680_e66336: f64 = (assign51680_e66334 * locals.var_gmobcssat__blk1396);
        (assign51680_e66336, (((((((locals.var_thecs_t_dn4 * assign51680_e66331) + (locals.var_thecs_t * (locals.var_alphasat__blk1394_dn4 + (-(locals.var_temp__blk949_dn4 / (assign51680_e66329 * assign51680_e66329)))))) * locals.var_qbsat__blk1393) - (assign51680_e66332 * locals.var_qbsat__blk1393_dn4)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)) * locals.var_gmobcssat__blk1396) + (assign51680_e66334 * locals.var_gmobcssat__blk1396_dn4)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1394_dn6 + (-(locals.var_temp__blk949_dn6 / (assign51680_e66329 * assign51680_e66329))))) * locals.var_qbsat__blk1393) - (assign51680_e66332 * locals.var_qbsat__blk1393_dn6)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)) * locals.var_gmobcssat__blk1396) + (assign51680_e66334 * locals.var_gmobcssat__blk1396_dn6)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1394_dn7 + (-(locals.var_temp__blk949_dn7 / (assign51680_e66329 * assign51680_e66329))))) * locals.var_qbsat__blk1393) - (assign51680_e66332 * locals.var_qbsat__blk1393_dn7)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)) * locals.var_gmobcssat__blk1396) + (assign51680_e66334 * locals.var_gmobcssat__blk1396_dn7)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1394_dn8 + (-(locals.var_temp__blk949_dn8 / (assign51680_e66329 * assign51680_e66329))))) * locals.var_qbsat__blk1393) - (assign51680_e66332 * locals.var_qbsat__blk1393_dn8)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)) * locals.var_gmobcssat__blk1396) + (assign51680_e66334 * locals.var_gmobcssat__blk1396_dn8)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1394_dn9 + (-(locals.var_temp__blk949_dn9 / (assign51680_e66329 * assign51680_e66329))))) * locals.var_qbsat__blk1393) - (assign51680_e66332 * locals.var_qbsat__blk1393_dn9)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)) * locals.var_gmobcssat__blk1396) + (assign51680_e66334 * locals.var_gmobcssat__blk1396_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign51680_e66338;
        locals.var_temp2_dn4 = assign51680_e66338_d_n4;
        locals.var_temp2_dn6 = assign51680_e66338_d_n6;
        locals.var_temp2_dn7 = assign51680_e66338_d_n7;
        locals.var_temp2_dn8 = assign51680_e66338_d_n8;
        locals.var_temp2_dn9 = assign51680_e66338_d_n9;

        let (assign51690_e66354, assign51690_e66354_d_n4, assign51690_e66354_d_n6, assign51690_e66354_d_n7, assign51690_e66354_d_n8, assign51690_e66354_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51690_e66348: f64 = (locals.var_ther_i * locals.var_rhob__blk1378);
        let assign51690_e66350: f64 = (assign51690_e66348 * locals.var_rhog__blk1379);
        let assign51690_e66352: f64 = (assign51690_e66350 * locals.var_qisat__blk1392);
        (assign51690_e66352, ((((((locals.var_ther_i_dn4 * locals.var_rhob__blk1378) + (locals.var_ther_i * locals.var_rhob__blk1378_dn4)) * locals.var_rhog__blk1379) + (assign51690_e66348 * locals.var_rhog__blk1379_dn4)) * locals.var_qisat__blk1392) + (assign51690_e66350 * locals.var_qisat__blk1392_dn4)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn6) * locals.var_rhog__blk1379) + (assign51690_e66348 * locals.var_rhog__blk1379_dn6)) * locals.var_qisat__blk1392) + (assign51690_e66350 * locals.var_qisat__blk1392_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn7) * locals.var_rhog__blk1379) + (assign51690_e66348 * locals.var_rhog__blk1379_dn7)) * locals.var_qisat__blk1392) + (assign51690_e66350 * locals.var_qisat__blk1392_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn8) * locals.var_rhog__blk1379) + (assign51690_e66348 * locals.var_rhog__blk1379_dn8)) * locals.var_qisat__blk1392) + (assign51690_e66350 * locals.var_qisat__blk1392_dn8)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn9) * locals.var_rhog__blk1379) + (assign51690_e66348 * locals.var_rhog__blk1379_dn9)) * locals.var_qisat__blk1392) + (assign51690_e66350 * locals.var_qisat__blk1392_dn9)),)
    } else {
        (locals.var_grsat__blk1397, locals.var_grsat__blk1397_dn4, locals.var_grsat__blk1397_dn6, locals.var_grsat__blk1397_dn7, locals.var_grsat__blk1397_dn8, locals.var_grsat__blk1397_dn9,)
    }
};
        locals.var_grsat__blk1397 = assign51690_e66354;
        locals.var_grsat__blk1397_dn4 = assign51690_e66354_d_n4;
        locals.var_grsat__blk1397_dn6 = assign51690_e66354_d_n6;
        locals.var_grsat__blk1397_dn7 = assign51690_e66354_d_n7;
        locals.var_grsat__blk1397_dn8 = assign51690_e66354_d_n8;
        locals.var_grsat__blk1397_dn9 = assign51690_e66354_d_n9;

    }

    pub(super) fn stamp_transient_block_45(
        locals: &mut StampLocals,
    ) {
        let (assign51700_e66376, assign51700_e66376_d_n4, assign51700_e66376_d_n6, assign51700_e66376_d_n7, assign51700_e66376_d_n8, assign51700_e66376_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51700_e66366: f64 = (locals.var_ther_i * locals.var_rhob__blk1378);
        let assign51700_e66368: f64 = (assign51700_e66366 * locals.var_rhog__blk1379);
        let assign51700_e66370: f64 = (assign51700_e66368 * locals.var_alphasat__blk1394);
        let assign51700_e66371: f64 = (locals.var_temp1 - assign51700_e66370);
        let assign51700_e66373: f64 = (assign51700_e66371 / locals.var_temp2);
        let assign51700_e66374: f64 = (1.0 + assign51700_e66373);
        (assign51700_e66374, ((((locals.var_temp1_dn4 - ((((((locals.var_ther_i_dn4 * locals.var_rhob__blk1378) + (locals.var_ther_i * locals.var_rhob__blk1378_dn4)) * locals.var_rhog__blk1379) + (assign51700_e66366 * locals.var_rhog__blk1379_dn4)) * locals.var_alphasat__blk1394) + (assign51700_e66368 * locals.var_alphasat__blk1394_dn4))) * locals.var_temp2) - (assign51700_e66371 * locals.var_temp2_dn4)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn6 - (((((locals.var_ther_i * locals.var_rhob__blk1378_dn6) * locals.var_rhog__blk1379) + (assign51700_e66366 * locals.var_rhog__blk1379_dn6)) * locals.var_alphasat__blk1394) + (assign51700_e66368 * locals.var_alphasat__blk1394_dn6))) * locals.var_temp2) - (assign51700_e66371 * locals.var_temp2_dn6)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn7 - (((((locals.var_ther_i * locals.var_rhob__blk1378_dn7) * locals.var_rhog__blk1379) + (assign51700_e66366 * locals.var_rhog__blk1379_dn7)) * locals.var_alphasat__blk1394) + (assign51700_e66368 * locals.var_alphasat__blk1394_dn7))) * locals.var_temp2) - (assign51700_e66371 * locals.var_temp2_dn7)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn8 - (((((locals.var_ther_i * locals.var_rhob__blk1378_dn8) * locals.var_rhog__blk1379) + (assign51700_e66366 * locals.var_rhog__blk1379_dn8)) * locals.var_alphasat__blk1394) + (assign51700_e66368 * locals.var_alphasat__blk1394_dn8))) * locals.var_temp2) - (assign51700_e66371 * locals.var_temp2_dn8)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn9 - (((((locals.var_ther_i * locals.var_rhob__blk1378_dn9) * locals.var_rhog__blk1379) + (assign51700_e66366 * locals.var_rhog__blk1379_dn9)) * locals.var_alphasat__blk1394) + (assign51700_e66368 * locals.var_alphasat__blk1394_dn9))) * locals.var_temp2) - (assign51700_e66371 * locals.var_temp2_dn9)) / (locals.var_temp2 * locals.var_temp2)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51700_e66376;
        locals.var_temp__blk949_dn4 = assign51700_e66376_d_n4;
        locals.var_temp__blk949_dn6 = assign51700_e66376_d_n6;
        locals.var_temp__blk949_dn7 = assign51700_e66376_d_n7;
        locals.var_temp__blk949_dn8 = assign51700_e66376_d_n8;
        locals.var_temp__blk949_dn9 = assign51700_e66376_d_n9;

        let assign51710_e66379: f64 = if locals.var_temp__blk949 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1506 = assign51710_e66379;

        let (assign51720_e66399, assign51720_e66399_d_n4, assign51720_e66399_d_n6, assign51720_e66399_d_n7, assign51720_e66399_d_n8, assign51720_e66399_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1506 != 0.0)) {
        let assign51720_e66393: f64 = (2.0 * locals.var_temp__blk949);
        let assign51720_e66394: f64 = (assign51720_e66393).exp();
        let assign51720_e66395: f64 = (1.0 + assign51720_e66394);
        let assign51720_e66396: f64 = (assign51720_e66395).ln();
        let assign51720_e66397: f64 = (0.5 * assign51720_e66396);
        (assign51720_e66397, (0.5 * ((assign51720_e66394 * (2.0 * locals.var_temp__blk949_dn4)) / assign51720_e66395)), (0.5 * ((assign51720_e66394 * (2.0 * locals.var_temp__blk949_dn6)) / assign51720_e66395)), (0.5 * ((assign51720_e66394 * (2.0 * locals.var_temp__blk949_dn7)) / assign51720_e66395)), (0.5 * ((assign51720_e66394 * (2.0 * locals.var_temp__blk949_dn8)) / assign51720_e66395)), (0.5 * ((assign51720_e66394 * (2.0 * locals.var_temp__blk949_dn9)) / assign51720_e66395)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign51720_e66399;
        locals.var_temp1_dn4 = assign51720_e66399_d_n4;
        locals.var_temp1_dn6 = assign51720_e66399_d_n6;
        locals.var_temp1_dn7 = assign51720_e66399_d_n7;
        locals.var_temp1_dn8 = assign51720_e66399_d_n8;
        locals.var_temp1_dn9 = assign51720_e66399_d_n9;

        let (assign51730_e66412, assign51730_e66412_d_n4, assign51730_e66412_d_n6, assign51730_e66412_d_n7, assign51730_e66412_d_n8, assign51730_e66412_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1506 == 0.0)) {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign51730_e66412;
        locals.var_temp1_dn4 = assign51730_e66412_d_n4;
        locals.var_temp1_dn6 = assign51730_e66412_d_n6;
        locals.var_temp1_dn7 = assign51730_e66412_d_n7;
        locals.var_temp1_dn8 = assign51730_e66412_d_n8;
        locals.var_temp1_dn9 = assign51730_e66412_d_n9;

        let (assign51740_e66435, assign51740_e66435_d_n4, assign51740_e66435_d_n6, assign51740_e66435_d_n7, assign51740_e66435_d_n8, assign51740_e66435_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51740_e66421: f64 = (-locals.var_midphi0__blk1391);
        let assign51740_e66423: f64 = (assign51740_e66421 * locals.var_temp2);
        let assign51740_e66425: f64 = (assign51740_e66423 * locals.var_temp1);
        let assign51740_e66428: f64 = (1.0 + locals.var_gmobmusat__blk1395);
        let assign51740_e66430: f64 = (assign51740_e66428 + locals.var_gmobcssat__blk1396);
        let assign51740_e66432: f64 = (assign51740_e66430 + locals.var_grsat__blk1397);
        let assign51740_e66433: f64 = (assign51740_e66425 / assign51740_e66432);
        (assign51740_e66433, ((((((((-locals.var_midphi0__blk1391_dn4) * locals.var_temp2) + (assign51740_e66421 * locals.var_temp2_dn4)) * locals.var_temp1) + (assign51740_e66423 * locals.var_temp1_dn4)) * assign51740_e66432) - (assign51740_e66425 * ((locals.var_gmobmusat__blk1395_dn4 + locals.var_gmobcssat__blk1396_dn4) + locals.var_grsat__blk1397_dn4))) / (assign51740_e66432 * assign51740_e66432)), ((((((((-locals.var_midphi0__blk1391_dn6) * locals.var_temp2) + (assign51740_e66421 * locals.var_temp2_dn6)) * locals.var_temp1) + (assign51740_e66423 * locals.var_temp1_dn6)) * assign51740_e66432) - (assign51740_e66425 * ((locals.var_gmobmusat__blk1395_dn6 + locals.var_gmobcssat__blk1396_dn6) + locals.var_grsat__blk1397_dn6))) / (assign51740_e66432 * assign51740_e66432)), ((((((((-locals.var_midphi0__blk1391_dn7) * locals.var_temp2) + (assign51740_e66421 * locals.var_temp2_dn7)) * locals.var_temp1) + (assign51740_e66423 * locals.var_temp1_dn7)) * assign51740_e66432) - (assign51740_e66425 * ((locals.var_gmobmusat__blk1395_dn7 + locals.var_gmobcssat__blk1396_dn7) + locals.var_grsat__blk1397_dn7))) / (assign51740_e66432 * assign51740_e66432)), ((((((((-locals.var_midphi0__blk1391_dn8) * locals.var_temp2) + (assign51740_e66421 * locals.var_temp2_dn8)) * locals.var_temp1) + (assign51740_e66423 * locals.var_temp1_dn8)) * assign51740_e66432) - (assign51740_e66425 * ((locals.var_gmobmusat__blk1395_dn8 + locals.var_gmobcssat__blk1396_dn8) + locals.var_grsat__blk1397_dn8))) / (assign51740_e66432 * assign51740_e66432)), ((((((((-locals.var_midphi0__blk1391_dn9) * locals.var_temp2) + (assign51740_e66421 * locals.var_temp2_dn9)) * locals.var_temp1) + (assign51740_e66423 * locals.var_temp1_dn9)) * assign51740_e66432) - (assign51740_e66425 * ((locals.var_gmobmusat__blk1395_dn9 + locals.var_gmobcssat__blk1396_dn9) + locals.var_grsat__blk1397_dn9))) / (assign51740_e66432 * assign51740_e66432)),)
    } else {
        (locals.var_delta_gmob__blk1398, locals.var_delta_gmob__blk1398_dn4, locals.var_delta_gmob__blk1398_dn6, locals.var_delta_gmob__blk1398_dn7, locals.var_delta_gmob__blk1398_dn8, locals.var_delta_gmob__blk1398_dn9,)
    }
};
        locals.var_delta_gmob__blk1398 = assign51740_e66435;
        locals.var_delta_gmob__blk1398_dn4 = assign51740_e66435_d_n4;
        locals.var_delta_gmob__blk1398_dn6 = assign51740_e66435_d_n6;
        locals.var_delta_gmob__blk1398_dn7 = assign51740_e66435_d_n7;
        locals.var_delta_gmob__blk1398_dn8 = assign51740_e66435_d_n8;
        locals.var_delta_gmob__blk1398_dn9 = assign51740_e66435_d_n9;

        let (assign51750_e66458, assign51750_e66458_d_n4, assign51750_e66458_d_n6, assign51750_e66458_d_n7, assign51750_e66458_d_n8, assign51750_e66458_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign51750_e66450: f64 = (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398);
        let assign51750_e66451: f64 = (1.0 + assign51750_e66450);
        let assign51750_e66452: f64 = (assign51750_e66451).sqrt();
        let assign51750_e66453: f64 = (1.0 + assign51750_e66452);
        let assign51750_e66454: f64 = (locals.var_delta_gmob__blk1398 / assign51750_e66453);
        let assign51750_e66455: f64 = (1.0 + assign51750_e66454);
        let assign51750_e66456: f64 = (locals.var_x_inf0__blk1390 * assign51750_e66455);
        (assign51750_e66456, ((locals.var_x_inf0__blk1390_dn4 * assign51750_e66455) + (locals.var_x_inf0__blk1390 * (((locals.var_delta_gmob__blk1398_dn4 * assign51750_e66453) - (locals.var_delta_gmob__blk1398 * (((locals.var_delta_gmob__blk1398_dn4 * locals.var_delta_gmob__blk1398) + (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398_dn4)) / (2.0 * assign51750_e66452)))) / (assign51750_e66453 * assign51750_e66453)))), ((locals.var_x_inf0__blk1390_dn6 * assign51750_e66455) + (locals.var_x_inf0__blk1390 * (((locals.var_delta_gmob__blk1398_dn6 * assign51750_e66453) - (locals.var_delta_gmob__blk1398 * (((locals.var_delta_gmob__blk1398_dn6 * locals.var_delta_gmob__blk1398) + (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398_dn6)) / (2.0 * assign51750_e66452)))) / (assign51750_e66453 * assign51750_e66453)))), ((locals.var_x_inf0__blk1390_dn7 * assign51750_e66455) + (locals.var_x_inf0__blk1390 * (((locals.var_delta_gmob__blk1398_dn7 * assign51750_e66453) - (locals.var_delta_gmob__blk1398 * (((locals.var_delta_gmob__blk1398_dn7 * locals.var_delta_gmob__blk1398) + (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398_dn7)) / (2.0 * assign51750_e66452)))) / (assign51750_e66453 * assign51750_e66453)))), ((locals.var_x_inf0__blk1390_dn8 * assign51750_e66455) + (locals.var_x_inf0__blk1390 * (((locals.var_delta_gmob__blk1398_dn8 * assign51750_e66453) - (locals.var_delta_gmob__blk1398 * (((locals.var_delta_gmob__blk1398_dn8 * locals.var_delta_gmob__blk1398) + (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398_dn8)) / (2.0 * assign51750_e66452)))) / (assign51750_e66453 * assign51750_e66453)))), ((locals.var_x_inf0__blk1390_dn9 * assign51750_e66455) + (locals.var_x_inf0__blk1390 * (((locals.var_delta_gmob__blk1398_dn9 * assign51750_e66453) - (locals.var_delta_gmob__blk1398 * (((locals.var_delta_gmob__blk1398_dn9 * locals.var_delta_gmob__blk1398) + (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398_dn9)) / (2.0 * assign51750_e66452)))) / (assign51750_e66453 * assign51750_e66453)))),)
    } else {
        (locals.var_x_inf__blk1399, locals.var_x_inf__blk1399_dn4, locals.var_x_inf__blk1399_dn6, locals.var_x_inf__blk1399_dn7, locals.var_x_inf__blk1399_dn8, locals.var_x_inf__blk1399_dn9,)
    }
};
        locals.var_x_inf__blk1399 = assign51750_e66458;
        locals.var_x_inf__blk1399_dn4 = assign51750_e66458_d_n4;
        locals.var_x_inf__blk1399_dn6 = assign51750_e66458_d_n6;
        locals.var_x_inf__blk1399_dn7 = assign51750_e66458_d_n7;
        locals.var_x_inf__blk1399_dn8 = assign51750_e66458_d_n8;
        locals.var_x_inf__blk1399_dn9 = assign51750_e66458_d_n9;

        let (assign51760_e66469, assign51760_e66469_d_n4, assign51760_e66469_d_n6, assign51760_e66469_d_n7, assign51760_e66469_d_n8, assign51760_e66469_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 == 0.0)) {
        (locals.var_x_inf0__blk1390, locals.var_x_inf0__blk1390_dn4, locals.var_x_inf0__blk1390_dn6, locals.var_x_inf0__blk1390_dn7, locals.var_x_inf0__blk1390_dn8, locals.var_x_inf0__blk1390_dn9,)
    } else {
        (locals.var_x_inf__blk1399, locals.var_x_inf__blk1399_dn4, locals.var_x_inf__blk1399_dn6, locals.var_x_inf__blk1399_dn7, locals.var_x_inf__blk1399_dn8, locals.var_x_inf__blk1399_dn9,)
    }
};
        locals.var_x_inf__blk1399 = assign51760_e66469;
        locals.var_x_inf__blk1399_dn4 = assign51760_e66469_d_n4;
        locals.var_x_inf__blk1399_dn6 = assign51760_e66469_d_n6;
        locals.var_x_inf__blk1399_dn7 = assign51760_e66469_d_n7;
        locals.var_x_inf__blk1399_dn8 = assign51760_e66469_d_n8;
        locals.var_x_inf__blk1399_dn9 = assign51760_e66469_d_n9;

        let (assign51770_e66483, assign51770_e66483_d_n4, assign51770_e66483_d_n6, assign51770_e66483_d_n7, assign51770_e66483_d_n8, assign51770_e66483_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51770_e66477: f64 = (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388);
        let assign51770_e66479: f64 = (assign51770_e66477 * locals.var_x_inf__blk1399);
        let assign51770_e66481: f64 = (assign51770_e66479 * 0.7071067811865475);
        (assign51770_e66481, (((((locals.var_phit1__blk1339_dn4 * locals.var_thesat1__blk1388) + (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388_dn4)) * locals.var_x_inf__blk1399) + (assign51770_e66477 * locals.var_x_inf__blk1399_dn4)) * 0.7071067811865475), (((((locals.var_phit1__blk1339_dn6 * locals.var_thesat1__blk1388) + (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388_dn6)) * locals.var_x_inf__blk1399) + (assign51770_e66477 * locals.var_x_inf__blk1399_dn6)) * 0.7071067811865475), (((((locals.var_phit1__blk1339_dn7 * locals.var_thesat1__blk1388) + (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388_dn7)) * locals.var_x_inf__blk1399) + (assign51770_e66477 * locals.var_x_inf__blk1399_dn7)) * 0.7071067811865475), (((((locals.var_phit1__blk1339_dn8 * locals.var_thesat1__blk1388) + (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388_dn8)) * locals.var_x_inf__blk1399) + (assign51770_e66477 * locals.var_x_inf__blk1399_dn8)) * 0.7071067811865475), (((((locals.var_phit1__blk1339_dn9 * locals.var_thesat1__blk1388) + (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388_dn9)) * locals.var_x_inf__blk1399) + (assign51770_e66477 * locals.var_x_inf__blk1399_dn9)) * 0.7071067811865475),)
    } else {
        (locals.var_ysat__blk1400, locals.var_ysat__blk1400_dn4, locals.var_ysat__blk1400_dn6, locals.var_ysat__blk1400_dn7, locals.var_ysat__blk1400_dn8, locals.var_ysat__blk1400_dn9,)
    }
};
        locals.var_ysat__blk1400 = assign51770_e66483;
        locals.var_ysat__blk1400_dn4 = assign51770_e66483_d_n4;
        locals.var_ysat__blk1400_dn6 = assign51770_e66483_d_n6;
        locals.var_ysat__blk1400_dn7 = assign51770_e66483_d_n7;
        locals.var_ysat__blk1400_dn8 = assign51770_e66483_d_n8;
        locals.var_ysat__blk1400_dn9 = assign51770_e66483_d_n9;

        let assign51780_e66486: f64 = (-1.0);
        let assign51780_e66487: f64 = if locals.var_chnl_type == assign51780_e66486 { 1.0 } else { 0.0 };
        locals.var_guard1507 = assign51780_e66487;

        let (assign51790_e66502, assign51790_e66502_d_n4, assign51790_e66502_d_n6, assign51790_e66502_d_n7, assign51790_e66502_d_n8, assign51790_e66502_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign51790_e66498: f64 = (1.0 + locals.var_ysat__blk1400);
        let assign51790_e66499: f64 = (assign51790_e66498).sqrt();
        let assign51790_e66500: f64 = (locals.var_ysat__blk1400 / assign51790_e66499);
        (assign51790_e66500, (((locals.var_ysat__blk1400_dn4 * assign51790_e66499) - (locals.var_ysat__blk1400 * (locals.var_ysat__blk1400_dn4 / (2.0 * assign51790_e66499)))) / (assign51790_e66499 * assign51790_e66499)), (((locals.var_ysat__blk1400_dn6 * assign51790_e66499) - (locals.var_ysat__blk1400 * (locals.var_ysat__blk1400_dn6 / (2.0 * assign51790_e66499)))) / (assign51790_e66499 * assign51790_e66499)), (((locals.var_ysat__blk1400_dn7 * assign51790_e66499) - (locals.var_ysat__blk1400 * (locals.var_ysat__blk1400_dn7 / (2.0 * assign51790_e66499)))) / (assign51790_e66499 * assign51790_e66499)), (((locals.var_ysat__blk1400_dn8 * assign51790_e66499) - (locals.var_ysat__blk1400 * (locals.var_ysat__blk1400_dn8 / (2.0 * assign51790_e66499)))) / (assign51790_e66499 * assign51790_e66499)), (((locals.var_ysat__blk1400_dn9 * assign51790_e66499) - (locals.var_ysat__blk1400 * (locals.var_ysat__blk1400_dn9 / (2.0 * assign51790_e66499)))) / (assign51790_e66499 * assign51790_e66499)),)
    } else {
        (locals.var_ysat__blk1400, locals.var_ysat__blk1400_dn4, locals.var_ysat__blk1400_dn6, locals.var_ysat__blk1400_dn7, locals.var_ysat__blk1400_dn8, locals.var_ysat__blk1400_dn9,)
    }
};
        locals.var_ysat__blk1400 = assign51790_e66502;
        locals.var_ysat__blk1400_dn4 = assign51790_e66502_d_n4;
        locals.var_ysat__blk1400_dn6 = assign51790_e66502_d_n6;
        locals.var_ysat__blk1400_dn7 = assign51790_e66502_d_n7;
        locals.var_ysat__blk1400_dn8 = assign51790_e66502_d_n8;
        locals.var_ysat__blk1400_dn9 = assign51790_e66502_d_n9;

        let (assign51800_e66519, assign51800_e66519_d_n4, assign51800_e66519_d_n6, assign51800_e66519_d_n7, assign51800_e66519_d_n8, assign51800_e66519_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51800_e66513: f64 = (4.0 * locals.var_ysat__blk1400);
        let assign51800_e66514: f64 = (1.0 + assign51800_e66513);
        let assign51800_e66515: f64 = (assign51800_e66514).sqrt();
        let assign51800_e66516: f64 = (1.0 + assign51800_e66515);
        let assign51800_e66517: f64 = (2.0 / assign51800_e66516);
        (assign51800_e66517, (-((2.0 * ((4.0 * locals.var_ysat__blk1400_dn4) / (2.0 * assign51800_e66515))) / (assign51800_e66516 * assign51800_e66516))), (-((2.0 * ((4.0 * locals.var_ysat__blk1400_dn6) / (2.0 * assign51800_e66515))) / (assign51800_e66516 * assign51800_e66516))), (-((2.0 * ((4.0 * locals.var_ysat__blk1400_dn7) / (2.0 * assign51800_e66515))) / (assign51800_e66516 * assign51800_e66516))), (-((2.0 * ((4.0 * locals.var_ysat__blk1400_dn8) / (2.0 * assign51800_e66515))) / (assign51800_e66516 * assign51800_e66516))), (-((2.0 * ((4.0 * locals.var_ysat__blk1400_dn9) / (2.0 * assign51800_e66515))) / (assign51800_e66516 * assign51800_e66516))),)
    } else {
        (locals.var_za__blk1401, locals.var_za__blk1401_dn4, locals.var_za__blk1401_dn6, locals.var_za__blk1401_dn7, locals.var_za__blk1401_dn8, locals.var_za__blk1401_dn9,)
    }
};
        locals.var_za__blk1401 = assign51800_e66519;
        locals.var_za__blk1401_dn4 = assign51800_e66519_d_n4;
        locals.var_za__blk1401_dn6 = assign51800_e66519_d_n6;
        locals.var_za__blk1401_dn7 = assign51800_e66519_d_n7;
        locals.var_za__blk1401_dn8 = assign51800_e66519_d_n8;
        locals.var_za__blk1401_dn9 = assign51800_e66519_d_n9;

        let (assign51810_e66529, assign51810_e66529_d_n4, assign51810_e66529_d_n6, assign51810_e66529_d_n7, assign51810_e66529_d_n8, assign51810_e66529_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51810_e66527: f64 = (locals.var_za__blk1401 * locals.var_ysat__blk1400);
        (assign51810_e66527, ((locals.var_za__blk1401_dn4 * locals.var_ysat__blk1400) + (locals.var_za__blk1401 * locals.var_ysat__blk1400_dn4)), ((locals.var_za__blk1401_dn6 * locals.var_ysat__blk1400) + (locals.var_za__blk1401 * locals.var_ysat__blk1400_dn6)), ((locals.var_za__blk1401_dn7 * locals.var_ysat__blk1400) + (locals.var_za__blk1401 * locals.var_ysat__blk1400_dn7)), ((locals.var_za__blk1401_dn8 * locals.var_ysat__blk1400) + (locals.var_za__blk1401 * locals.var_ysat__blk1400_dn8)), ((locals.var_za__blk1401_dn9 * locals.var_ysat__blk1400) + (locals.var_za__blk1401 * locals.var_ysat__blk1400_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51810_e66529;
        locals.var_temp__blk949_dn4 = assign51810_e66529_d_n4;
        locals.var_temp__blk949_dn6 = assign51810_e66529_d_n6;
        locals.var_temp__blk949_dn7 = assign51810_e66529_d_n7;
        locals.var_temp__blk949_dn8 = assign51810_e66529_d_n8;
        locals.var_temp__blk949_dn9 = assign51810_e66529_d_n9;

        let (assign51820_e66561, assign51820_e66561_d_n4, assign51820_e66561_d_n6, assign51820_e66561_d_n7, assign51820_e66561_d_n8, assign51820_e66561_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51820_e66537: f64 = (locals.var_x_inf__blk1399 * locals.var_za__blk1401);
        let assign51820_e66541: f64 = (0.86 * locals.var_temp__blk949);
        let assign51820_e66545: f64 = (locals.var_temp__blk949 * locals.var_za__blk1401);
        let assign51820_e66546: f64 = (1.0 - assign51820_e66545);
        let assign51820_e66547: f64 = (assign51820_e66541 * assign51820_e66546);
        let assign51820_e66551: f64 = (4.0 * locals.var_temp__blk949);
        let assign51820_e66553: f64 = (assign51820_e66551 * locals.var_temp__blk949);
        let assign51820_e66555: f64 = (assign51820_e66553 * locals.var_za__blk1401);
        let assign51820_e66556: f64 = (1.0 + assign51820_e66555);
        let assign51820_e66557: f64 = (assign51820_e66547 / assign51820_e66556);
        let assign51820_e66558: f64 = (1.0 + assign51820_e66557);
        let assign51820_e66559: f64 = (assign51820_e66537 * assign51820_e66558);
        (assign51820_e66559, ((((locals.var_x_inf__blk1399_dn4 * locals.var_za__blk1401) + (locals.var_x_inf__blk1399 * locals.var_za__blk1401_dn4)) * assign51820_e66558) + (assign51820_e66537 * ((((((0.86 * locals.var_temp__blk949_dn4) * assign51820_e66546) + (assign51820_e66541 * (-((locals.var_temp__blk949_dn4 * locals.var_za__blk1401) + (locals.var_temp__blk949 * locals.var_za__blk1401_dn4))))) * assign51820_e66556) - (assign51820_e66547 * (((((4.0 * locals.var_temp__blk949_dn4) * locals.var_temp__blk949) + (assign51820_e66551 * locals.var_temp__blk949_dn4)) * locals.var_za__blk1401) + (assign51820_e66553 * locals.var_za__blk1401_dn4)))) / (assign51820_e66556 * assign51820_e66556)))), ((((locals.var_x_inf__blk1399_dn6 * locals.var_za__blk1401) + (locals.var_x_inf__blk1399 * locals.var_za__blk1401_dn6)) * assign51820_e66558) + (assign51820_e66537 * ((((((0.86 * locals.var_temp__blk949_dn6) * assign51820_e66546) + (assign51820_e66541 * (-((locals.var_temp__blk949_dn6 * locals.var_za__blk1401) + (locals.var_temp__blk949 * locals.var_za__blk1401_dn6))))) * assign51820_e66556) - (assign51820_e66547 * (((((4.0 * locals.var_temp__blk949_dn6) * locals.var_temp__blk949) + (assign51820_e66551 * locals.var_temp__blk949_dn6)) * locals.var_za__blk1401) + (assign51820_e66553 * locals.var_za__blk1401_dn6)))) / (assign51820_e66556 * assign51820_e66556)))), ((((locals.var_x_inf__blk1399_dn7 * locals.var_za__blk1401) + (locals.var_x_inf__blk1399 * locals.var_za__blk1401_dn7)) * assign51820_e66558) + (assign51820_e66537 * ((((((0.86 * locals.var_temp__blk949_dn7) * assign51820_e66546) + (assign51820_e66541 * (-((locals.var_temp__blk949_dn7 * locals.var_za__blk1401) + (locals.var_temp__blk949 * locals.var_za__blk1401_dn7))))) * assign51820_e66556) - (assign51820_e66547 * (((((4.0 * locals.var_temp__blk949_dn7) * locals.var_temp__blk949) + (assign51820_e66551 * locals.var_temp__blk949_dn7)) * locals.var_za__blk1401) + (assign51820_e66553 * locals.var_za__blk1401_dn7)))) / (assign51820_e66556 * assign51820_e66556)))), ((((locals.var_x_inf__blk1399_dn8 * locals.var_za__blk1401) + (locals.var_x_inf__blk1399 * locals.var_za__blk1401_dn8)) * assign51820_e66558) + (assign51820_e66537 * ((((((0.86 * locals.var_temp__blk949_dn8) * assign51820_e66546) + (assign51820_e66541 * (-((locals.var_temp__blk949_dn8 * locals.var_za__blk1401) + (locals.var_temp__blk949 * locals.var_za__blk1401_dn8))))) * assign51820_e66556) - (assign51820_e66547 * (((((4.0 * locals.var_temp__blk949_dn8) * locals.var_temp__blk949) + (assign51820_e66551 * locals.var_temp__blk949_dn8)) * locals.var_za__blk1401) + (assign51820_e66553 * locals.var_za__blk1401_dn8)))) / (assign51820_e66556 * assign51820_e66556)))), ((((locals.var_x_inf__blk1399_dn9 * locals.var_za__blk1401) + (locals.var_x_inf__blk1399 * locals.var_za__blk1401_dn9)) * assign51820_e66558) + (assign51820_e66537 * ((((((0.86 * locals.var_temp__blk949_dn9) * assign51820_e66546) + (assign51820_e66541 * (-((locals.var_temp__blk949_dn9 * locals.var_za__blk1401) + (locals.var_temp__blk949 * locals.var_za__blk1401_dn9))))) * assign51820_e66556) - (assign51820_e66547 * (((((4.0 * locals.var_temp__blk949_dn9) * locals.var_temp__blk949) + (assign51820_e66551 * locals.var_temp__blk949_dn9)) * locals.var_za__blk1401) + (assign51820_e66553 * locals.var_za__blk1401_dn9)))) / (assign51820_e66556 * assign51820_e66556)))),)
    } else {
        (locals.var_x_0__blk1402, locals.var_x_0__blk1402_dn4, locals.var_x_0__blk1402_dn6, locals.var_x_0__blk1402_dn7, locals.var_x_0__blk1402_dn8, locals.var_x_0__blk1402_dn9,)
    }
};
        locals.var_x_0__blk1402 = assign51820_e66561;
        locals.var_x_0__blk1402_dn4 = assign51820_e66561_d_n4;
        locals.var_x_0__blk1402_dn6 = assign51820_e66561_d_n6;
        locals.var_x_0__blk1402_dn7 = assign51820_e66561_d_n7;
        locals.var_x_0__blk1402_dn8 = assign51820_e66561_d_n8;
        locals.var_x_0__blk1402_dn9 = assign51820_e66561_d_n9;

        let (assign51830_e66571, assign51830_e66571_d_n4, assign51830_e66571_d_n6, assign51830_e66571_d_n7, assign51830_e66571_d_n8, assign51830_e66571_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51830_e66569: f64 = (0.99 * locals.var_x_0__blk1402);
        (assign51830_e66569, (0.99 * locals.var_x_0__blk1402_dn4), (0.99 * locals.var_x_0__blk1402_dn6), (0.99 * locals.var_x_0__blk1402_dn7), (0.99 * locals.var_x_0__blk1402_dn8), (0.99 * locals.var_x_0__blk1402_dn9),)
    } else {
        (locals.var_x_sat__blk1403, locals.var_x_sat__blk1403_dn4, locals.var_x_sat__blk1403_dn6, locals.var_x_sat__blk1403_dn7, locals.var_x_sat__blk1403_dn8, locals.var_x_sat__blk1403_dn9,)
    }
};
        locals.var_x_sat__blk1403 = assign51830_e66571;
        locals.var_x_sat__blk1403_dn4 = assign51830_e66571_d_n4;
        locals.var_x_sat__blk1403_dn6 = assign51830_e66571_d_n6;
        locals.var_x_sat__blk1403_dn7 = assign51830_e66571_d_n7;
        locals.var_x_sat__blk1403_dn8 = assign51830_e66571_d_n8;
        locals.var_x_sat__blk1403_dn9 = assign51830_e66571_d_n9;

        let (assign51840_e66589, assign51840_e66589_d_n4, assign51840_e66589_d_n6, assign51840_e66589_d_n7, assign51840_e66589_d_n8, assign51840_e66589_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51840_e66581: f64 = (2.0 * locals.var_asat__blk1389);
        let assign51840_e66582: f64 = (locals.var_x_sat__blk1403 - assign51840_e66581);
        let assign51840_e66583: f64 = (locals.var_x_sat__blk1403 * assign51840_e66582);
        let assign51840_e66585: f64 = (assign51840_e66583 * locals.var_inv_gf2__blk1341);
        let assign51840_e66587: f64 = (assign51840_e66585 / locals.var_ds__blk1370);
        (assign51840_e66587, (((((((locals.var_x_sat__blk1403_dn4 * assign51840_e66582) + (locals.var_x_sat__blk1403 * (locals.var_x_sat__blk1403_dn4 - (2.0 * locals.var_asat__blk1389_dn4)))) * locals.var_inv_gf2__blk1341) + (assign51840_e66583 * locals.var_inv_gf2__blk1341_dn4)) * locals.var_ds__blk1370) - (assign51840_e66585 * locals.var_ds__blk1370_dn4)) / (locals.var_ds__blk1370 * locals.var_ds__blk1370)), (((((((locals.var_x_sat__blk1403_dn6 * assign51840_e66582) + (locals.var_x_sat__blk1403 * (locals.var_x_sat__blk1403_dn6 - (2.0 * locals.var_asat__blk1389_dn6)))) * locals.var_inv_gf2__blk1341) + (assign51840_e66583 * locals.var_inv_gf2__blk1341_dn6)) * locals.var_ds__blk1370) - (assign51840_e66585 * locals.var_ds__blk1370_dn6)) / (locals.var_ds__blk1370 * locals.var_ds__blk1370)), (((((((locals.var_x_sat__blk1403_dn7 * assign51840_e66582) + (locals.var_x_sat__blk1403 * (locals.var_x_sat__blk1403_dn7 - (2.0 * locals.var_asat__blk1389_dn7)))) * locals.var_inv_gf2__blk1341) + (assign51840_e66583 * locals.var_inv_gf2__blk1341_dn7)) * locals.var_ds__blk1370) - (assign51840_e66585 * locals.var_ds__blk1370_dn7)) / (locals.var_ds__blk1370 * locals.var_ds__blk1370)), (((((((locals.var_x_sat__blk1403_dn8 * assign51840_e66582) + (locals.var_x_sat__blk1403 * (locals.var_x_sat__blk1403_dn8 - (2.0 * locals.var_asat__blk1389_dn8)))) * locals.var_inv_gf2__blk1341) + (assign51840_e66583 * locals.var_inv_gf2__blk1341_dn8)) * locals.var_ds__blk1370) - (assign51840_e66585 * locals.var_ds__blk1370_dn8)) / (locals.var_ds__blk1370 * locals.var_ds__blk1370)), (((((((locals.var_x_sat__blk1403_dn9 * assign51840_e66582) + (locals.var_x_sat__blk1403 * (locals.var_x_sat__blk1403_dn9 - (2.0 * locals.var_asat__blk1389_dn9)))) * locals.var_inv_gf2__blk1341) + (assign51840_e66583 * locals.var_inv_gf2__blk1341_dn9)) * locals.var_ds__blk1370) - (assign51840_e66585 * locals.var_ds__blk1370_dn9)) / (locals.var_ds__blk1370 * locals.var_ds__blk1370)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51840_e66589;
        locals.var_temp__blk949_dn4 = assign51840_e66589_d_n4;
        locals.var_temp__blk949_dn6 = assign51840_e66589_d_n6;
        locals.var_temp__blk949_dn7 = assign51840_e66589_d_n7;
        locals.var_temp__blk949_dn8 = assign51840_e66589_d_n8;
        locals.var_temp__blk949_dn9 = assign51840_e66589_d_n9;

        let (assign51850_e66611, assign51850_e66611_d_n4, assign51850_e66611_d_n6, assign51850_e66611_d_n7, assign51850_e66611_d_n8, assign51850_e66611_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign51850_e66600: f64 = (-0.99);
        let (assign51850_e66605, assign51850_e66605_d_n4, assign51850_e66605_d_n6, assign51850_e66605_d_n7, assign51850_e66605_d_n8, assign51850_e66605_d_n9,) = {
            if (locals.var_temp__blk949 > assign51850_e66600) {
                (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
            } else {
                let assign51850_e66604: f64 = (-0.99);
                (assign51850_e66604, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign51850_e66606: f64 = (1.0 + assign51850_e66605);
        let assign51850_e66607: f64 = (assign51850_e66606).ln();
        let assign51850_e66608: f64 = (locals.var_x_sat__blk1403 - assign51850_e66607);
        let assign51850_e66609: f64 = (locals.var_phit1__blk1339 * assign51850_e66608);
        (assign51850_e66609, ((locals.var_phit1__blk1339_dn4 * assign51850_e66608) + (locals.var_phit1__blk1339 * (locals.var_x_sat__blk1403_dn4 - (assign51850_e66605_d_n4 / assign51850_e66606)))), ((locals.var_phit1__blk1339_dn6 * assign51850_e66608) + (locals.var_phit1__blk1339 * (locals.var_x_sat__blk1403_dn6 - (assign51850_e66605_d_n6 / assign51850_e66606)))), ((locals.var_phit1__blk1339_dn7 * assign51850_e66608) + (locals.var_phit1__blk1339 * (locals.var_x_sat__blk1403_dn7 - (assign51850_e66605_d_n7 / assign51850_e66606)))), ((locals.var_phit1__blk1339_dn8 * assign51850_e66608) + (locals.var_phit1__blk1339 * (locals.var_x_sat__blk1403_dn8 - (assign51850_e66605_d_n8 / assign51850_e66606)))), ((locals.var_phit1__blk1339_dn9 * assign51850_e66608) + (locals.var_phit1__blk1339 * (locals.var_x_sat__blk1403_dn9 - (assign51850_e66605_d_n9 / assign51850_e66606)))),)
    } else {
        (locals.var_v_dsat__blk1404, locals.var_v_dsat__blk1404_dn4, locals.var_v_dsat__blk1404_dn6, locals.var_v_dsat__blk1404_dn7, locals.var_v_dsat__blk1404_dn8, locals.var_v_dsat__blk1404_dn9,)
    }
};
        locals.var_v_dsat__blk1404 = assign51850_e66611;
        locals.var_v_dsat__blk1404_dn4 = assign51850_e66611_d_n4;
        locals.var_v_dsat__blk1404_dn6 = assign51850_e66611_d_n6;
        locals.var_v_dsat__blk1404_dn7 = assign51850_e66611_d_n7;
        locals.var_v_dsat__blk1404_dn8 = assign51850_e66611_d_n8;
        locals.var_v_dsat__blk1404_dn9 = assign51850_e66611_d_n9;

        let (assign51860_e66620, assign51860_e66620_d_n4, assign51860_e66620_d_n6, assign51860_e66620_d_n7, assign51860_e66620_d_n8, assign51860_e66620_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 == 0.0)) {
        (locals.var_vdsat_lim__blk1387, locals.var_vdsat_lim__blk1387_dn4, locals.var_vdsat_lim__blk1387_dn6, locals.var_vdsat_lim__blk1387_dn7, locals.var_vdsat_lim__blk1387_dn8, locals.var_vdsat_lim__blk1387_dn9,)
    } else {
        (locals.var_v_dsat__blk1404, locals.var_v_dsat__blk1404_dn4, locals.var_v_dsat__blk1404_dn6, locals.var_v_dsat__blk1404_dn7, locals.var_v_dsat__blk1404_dn8, locals.var_v_dsat__blk1404_dn9,)
    }
};
        locals.var_v_dsat__blk1404 = assign51860_e66620;
        locals.var_v_dsat__blk1404_dn4 = assign51860_e66620_d_n4;
        locals.var_v_dsat__blk1404_dn6 = assign51860_e66620_d_n6;
        locals.var_v_dsat__blk1404_dn7 = assign51860_e66620_d_n7;
        locals.var_v_dsat__blk1404_dn8 = assign51860_e66620_d_n8;
        locals.var_v_dsat__blk1404_dn9 = assign51860_e66620_d_n9;

        let (assign51870_e66628, assign51870_e66628_d_n4, assign51870_e66628_d_n6, assign51870_e66628_d_n7, assign51870_e66628_d_n8, assign51870_e66628_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign51870_e66626: f64 = (1.0 + locals.var_arloc__blk1320);
        (assign51870_e66626, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51870_e66628;
        locals.var_temp__blk949_dn4 = assign51870_e66628_d_n4;
        locals.var_temp__blk949_dn6 = assign51870_e66628_d_n6;
        locals.var_temp__blk949_dn7 = assign51870_e66628_d_n7;
        locals.var_temp__blk949_dn8 = assign51870_e66628_d_n8;
        locals.var_temp__blk949_dn9 = assign51870_e66628_d_n9;

        let (assign51880_e66639, assign51880_e66639_d_n4, assign51880_e66639_d_n6, assign51880_e66639_d_n7, assign51880_e66639_d_n8, assign51880_e66639_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign51880_e66633: f64 = (locals.var_temp__blk949).sqrt();
        let assign51880_e66635: f64 = (assign51880_e66633 * locals.var_v_ds);
        let assign51880_e66637: f64 = (assign51880_e66635 / locals.var_v_dsat__blk1404);
        (assign51880_e66637, (((((locals.var_temp__blk949_dn4 / (2.0 * assign51880_e66633)) * locals.var_v_ds) * locals.var_v_dsat__blk1404) - (assign51880_e66635 * locals.var_v_dsat__blk1404_dn4)) / (locals.var_v_dsat__blk1404 * locals.var_v_dsat__blk1404)), (((((locals.var_temp__blk949_dn6 / (2.0 * assign51880_e66633)) * locals.var_v_ds) * locals.var_v_dsat__blk1404) - (assign51880_e66635 * locals.var_v_dsat__blk1404_dn6)) / (locals.var_v_dsat__blk1404 * locals.var_v_dsat__blk1404)), ((((((locals.var_temp__blk949_dn7 / (2.0 * assign51880_e66633)) * locals.var_v_ds) + (assign51880_e66633 * locals.var_v_ds_dn7)) * locals.var_v_dsat__blk1404) - (assign51880_e66635 * locals.var_v_dsat__blk1404_dn7)) / (locals.var_v_dsat__blk1404 * locals.var_v_dsat__blk1404)), ((((((locals.var_temp__blk949_dn8 / (2.0 * assign51880_e66633)) * locals.var_v_ds) + (assign51880_e66633 * locals.var_v_ds_dn8)) * locals.var_v_dsat__blk1404) - (assign51880_e66635 * locals.var_v_dsat__blk1404_dn8)) / (locals.var_v_dsat__blk1404 * locals.var_v_dsat__blk1404)), (((((locals.var_temp__blk949_dn9 / (2.0 * assign51880_e66633)) * locals.var_v_ds) * locals.var_v_dsat__blk1404) - (assign51880_e66635 * locals.var_v_dsat__blk1404_dn9)) / (locals.var_v_dsat__blk1404 * locals.var_v_dsat__blk1404)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign51880_e66639;
        locals.var_temp1_dn4 = assign51880_e66639_d_n4;
        locals.var_temp1_dn6 = assign51880_e66639_d_n6;
        locals.var_temp1_dn7 = assign51880_e66639_d_n7;
        locals.var_temp1_dn8 = assign51880_e66639_d_n8;
        locals.var_temp1_dn9 = assign51880_e66639_d_n9;

        let (assign51890_e66649, assign51890_e66649_d_n4, assign51890_e66649_d_n6, assign51890_e66649_d_n7, assign51890_e66649_d_n8, assign51890_e66649_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign51890_e66645: f64 = (locals.var_temp1 * locals.var_temp1);
        let assign51890_e66647: f64 = (assign51890_e66645 + locals.var_temp__blk949);
        (assign51890_e66647, (((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4)) + locals.var_temp__blk949_dn4), (((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)) + locals.var_temp__blk949_dn6), (((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)) + locals.var_temp__blk949_dn7), (((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)) + locals.var_temp__blk949_dn8), (((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9)) + locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign51890_e66649;
        locals.var_temp2_dn4 = assign51890_e66649_d_n4;
        locals.var_temp2_dn6 = assign51890_e66649_d_n6;
        locals.var_temp2_dn7 = assign51890_e66649_d_n7;
        locals.var_temp2_dn8 = assign51890_e66649_d_n8;
        locals.var_temp2_dn9 = assign51890_e66649_d_n9;

        let (assign51900_e66657, assign51900_e66657_d_n4, assign51900_e66657_d_n6, assign51900_e66657_d_n7, assign51900_e66657_d_n8, assign51900_e66657_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign51900_e66655: f64 = (2.0 * locals.var_temp1);
        (assign51900_e66655, (2.0 * locals.var_temp1_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign51900_e66657;
        locals.var_temp__blk949_dn4 = assign51900_e66657_d_n4;
        locals.var_temp__blk949_dn6 = assign51900_e66657_d_n6;
        locals.var_temp__blk949_dn7 = assign51900_e66657_d_n7;
        locals.var_temp__blk949_dn8 = assign51900_e66657_d_n8;
        locals.var_temp__blk949_dn9 = assign51900_e66657_d_n9;

        let (assign51910_e66675, assign51910_e66675_d_n4, assign51910_e66675_d_n6, assign51910_e66675_d_n7, assign51910_e66675_d_n8, assign51910_e66675_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign51910_e66663: f64 = (locals.var_v_dsat__blk1404 * locals.var_temp__blk949);
        let assign51910_e66666: f64 = (locals.var_temp2 - locals.var_temp__blk949);
        let assign51910_e66667: f64 = (assign51910_e66666).sqrt();
        let assign51910_e66670: f64 = (locals.var_temp2 + locals.var_temp__blk949);
        let assign51910_e66671: f64 = (assign51910_e66670).sqrt();
        let assign51910_e66672: f64 = (assign51910_e66667 + assign51910_e66671);
        let assign51910_e66673: f64 = (assign51910_e66663 / assign51910_e66672);
        (assign51910_e66673, (((((locals.var_v_dsat__blk1404_dn4 * locals.var_temp__blk949) + (locals.var_v_dsat__blk1404 * locals.var_temp__blk949_dn4)) * assign51910_e66672) - (assign51910_e66663 * (((locals.var_temp2_dn4 - locals.var_temp__blk949_dn4) / (2.0 * assign51910_e66667)) + ((locals.var_temp2_dn4 + locals.var_temp__blk949_dn4) / (2.0 * assign51910_e66671))))) / (assign51910_e66672 * assign51910_e66672)), (((((locals.var_v_dsat__blk1404_dn6 * locals.var_temp__blk949) + (locals.var_v_dsat__blk1404 * locals.var_temp__blk949_dn6)) * assign51910_e66672) - (assign51910_e66663 * (((locals.var_temp2_dn6 - locals.var_temp__blk949_dn6) / (2.0 * assign51910_e66667)) + ((locals.var_temp2_dn6 + locals.var_temp__blk949_dn6) / (2.0 * assign51910_e66671))))) / (assign51910_e66672 * assign51910_e66672)), (((((locals.var_v_dsat__blk1404_dn7 * locals.var_temp__blk949) + (locals.var_v_dsat__blk1404 * locals.var_temp__blk949_dn7)) * assign51910_e66672) - (assign51910_e66663 * (((locals.var_temp2_dn7 - locals.var_temp__blk949_dn7) / (2.0 * assign51910_e66667)) + ((locals.var_temp2_dn7 + locals.var_temp__blk949_dn7) / (2.0 * assign51910_e66671))))) / (assign51910_e66672 * assign51910_e66672)), (((((locals.var_v_dsat__blk1404_dn8 * locals.var_temp__blk949) + (locals.var_v_dsat__blk1404 * locals.var_temp__blk949_dn8)) * assign51910_e66672) - (assign51910_e66663 * (((locals.var_temp2_dn8 - locals.var_temp__blk949_dn8) / (2.0 * assign51910_e66667)) + ((locals.var_temp2_dn8 + locals.var_temp__blk949_dn8) / (2.0 * assign51910_e66671))))) / (assign51910_e66672 * assign51910_e66672)), (((((locals.var_v_dsat__blk1404_dn9 * locals.var_temp__blk949) + (locals.var_v_dsat__blk1404 * locals.var_temp__blk949_dn9)) * assign51910_e66672) - (assign51910_e66663 * (((locals.var_temp2_dn9 - locals.var_temp__blk949_dn9) / (2.0 * assign51910_e66667)) + ((locals.var_temp2_dn9 + locals.var_temp__blk949_dn9) / (2.0 * assign51910_e66671))))) / (assign51910_e66672 * assign51910_e66672)),)
    } else {
        (locals.var_vdse__blk1405, locals.var_vdse__blk1405_dn4, locals.var_vdse__blk1405_dn6, locals.var_vdse__blk1405_dn7, locals.var_vdse__blk1405_dn8, locals.var_vdse__blk1405_dn9,)
    }
};
        locals.var_vdse__blk1405 = assign51910_e66675;
        locals.var_vdse__blk1405_dn4 = assign51910_e66675_d_n4;
        locals.var_vdse__blk1405_dn6 = assign51910_e66675_d_n6;
        locals.var_vdse__blk1405_dn7 = assign51910_e66675_d_n7;
        locals.var_vdse__blk1405_dn8 = assign51910_e66675_d_n8;
        locals.var_vdse__blk1405_dn9 = assign51910_e66675_d_n9;

        let (assign51920_e66683, assign51920_e66683_d_n4, assign51920_e66683_d_n6, assign51920_e66683_d_n7, assign51920_e66683_d_n8, assign51920_e66683_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign51920_e66681: f64 = (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340);
        (assign51920_e66681, ((locals.var_vdse__blk1405_dn4 * locals.var_inv_phit1__blk1340) + (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340_dn4)), ((locals.var_vdse__blk1405_dn6 * locals.var_inv_phit1__blk1340) + (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340_dn6)), ((locals.var_vdse__blk1405_dn7 * locals.var_inv_phit1__blk1340) + (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340_dn7)), ((locals.var_vdse__blk1405_dn8 * locals.var_inv_phit1__blk1340) + (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340_dn8)), ((locals.var_vdse__blk1405_dn9 * locals.var_inv_phit1__blk1340) + (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340_dn9)),)
    } else {
        (locals.var_udse__blk1406, locals.var_udse__blk1406_dn4, locals.var_udse__blk1406_dn6, locals.var_udse__blk1406_dn7, locals.var_udse__blk1406_dn8, locals.var_udse__blk1406_dn9,)
    }
};
        locals.var_udse__blk1406 = assign51920_e66683;
        locals.var_udse__blk1406_dn4 = assign51920_e66683_d_n4;
        locals.var_udse__blk1406_dn6 = assign51920_e66683_d_n6;
        locals.var_udse__blk1406_dn7 = assign51920_e66683_d_n7;
        locals.var_udse__blk1406_dn8 = assign51920_e66683_d_n8;
        locals.var_udse__blk1406_dn9 = assign51920_e66683_d_n9;

        let (assign51930_e66691, assign51930_e66691_d_n4, assign51930_e66691_d_n6, assign51930_e66691_d_n7, assign51930_e66691_d_n8, assign51930_e66691_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign51930_e66689: f64 = (locals.var_xn_s__blk1349 + locals.var_udse__blk1406);
        (assign51930_e66689, (locals.var_xn_s__blk1349_dn4 + locals.var_udse__blk1406_dn4), (locals.var_xn_s__blk1349_dn6 + locals.var_udse__blk1406_dn6), (locals.var_xn_s__blk1349_dn7 + locals.var_udse__blk1406_dn7), (locals.var_xn_s__blk1349_dn8 + locals.var_udse__blk1406_dn8), (locals.var_xn_s__blk1349_dn9 + locals.var_udse__blk1406_dn9),)
    } else {
        (locals.var_xn_d__blk1407, locals.var_xn_d__blk1407_dn4, locals.var_xn_d__blk1407_dn6, locals.var_xn_d__blk1407_dn7, locals.var_xn_d__blk1407_dn8, locals.var_xn_d__blk1407_dn9,)
    }
};
        locals.var_xn_d__blk1407 = assign51930_e66691;
        locals.var_xn_d__blk1407_dn4 = assign51930_e66691_d_n4;
        locals.var_xn_d__blk1407_dn6 = assign51930_e66691_d_n6;
        locals.var_xn_d__blk1407_dn7 = assign51930_e66691_d_n7;
        locals.var_xn_d__blk1407_dn8 = assign51930_e66691_d_n8;
        locals.var_xn_d__blk1407_dn9 = assign51930_e66691_d_n9;

        let assign51940_e66694: f64 = if locals.var_udse__blk1406 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1508 = assign51940_e66694;

        let (assign51950_e66704, assign51950_e66704_d_n4, assign51950_e66704_d_n6, assign51950_e66704_d_n7, assign51950_e66704_d_n8, assign51950_e66704_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1508 != 0.0)) {
        let assign51950_e66701: f64 = (-locals.var_udse__blk1406);
        let assign51950_e66702: f64 = (assign51950_e66701).exp();
        (assign51950_e66702, (assign51950_e66702 * (-locals.var_udse__blk1406_dn4)), (assign51950_e66702 * (-locals.var_udse__blk1406_dn6)), (assign51950_e66702 * (-locals.var_udse__blk1406_dn7)), (assign51950_e66702 * (-locals.var_udse__blk1406_dn8)), (assign51950_e66702 * (-locals.var_udse__blk1406_dn9)),)
    } else {
        (locals.var_k_ds__blk1408, locals.var_k_ds__blk1408_dn4, locals.var_k_ds__blk1408_dn6, locals.var_k_ds__blk1408_dn7, locals.var_k_ds__blk1408_dn8, locals.var_k_ds__blk1408_dn9,)
    }
};
        locals.var_k_ds__blk1408 = assign51950_e66704;
        locals.var_k_ds__blk1408_dn4 = assign51950_e66704_d_n4;
        locals.var_k_ds__blk1408_dn6 = assign51950_e66704_d_n6;
        locals.var_k_ds__blk1408_dn7 = assign51950_e66704_d_n7;
        locals.var_k_ds__blk1408_dn8 = assign51950_e66704_d_n8;
        locals.var_k_ds__blk1408_dn9 = assign51950_e66704_d_n9;

        let (assign51960_e66735, assign51960_e66735_d_n4, assign51960_e66735_d_n6, assign51960_e66735_d_n7, assign51960_e66735_d_n8, assign51960_e66735_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1508 == 0.0)) {
        let assign51960_e66715: f64 = (locals.var_udse__blk1406 - 460.51701859880916);
        let assign51960_e66720: f64 = (locals.var_udse__blk1406 - 460.51701859880916);
        let assign51960_e66724: f64 = (locals.var_udse__blk1406 - 460.51701859880916);
        let assign51960_e66726: f64 = (assign51960_e66724 * 0.3333333333333333);
        let assign51960_e66727: f64 = (1.0 + assign51960_e66726);
        let assign51960_e66728: f64 = (assign51960_e66720 * assign51960_e66727);
        let assign51960_e66729: f64 = (0.5 * assign51960_e66728);
        let assign51960_e66730: f64 = (1.0 + assign51960_e66729);
        let assign51960_e66731: f64 = (assign51960_e66715 * assign51960_e66730);
        let assign51960_e66732: f64 = (1.0 + assign51960_e66731);
        let assign51960_e66733: f64 = (1e-200 / assign51960_e66732);
        (assign51960_e66733, (-((1e-200 * ((locals.var_udse__blk1406_dn4 * assign51960_e66730) + (assign51960_e66715 * (0.5 * ((locals.var_udse__blk1406_dn4 * assign51960_e66727) + (assign51960_e66720 * (locals.var_udse__blk1406_dn4 * 0.3333333333333333))))))) / (assign51960_e66732 * assign51960_e66732))), (-((1e-200 * ((locals.var_udse__blk1406_dn6 * assign51960_e66730) + (assign51960_e66715 * (0.5 * ((locals.var_udse__blk1406_dn6 * assign51960_e66727) + (assign51960_e66720 * (locals.var_udse__blk1406_dn6 * 0.3333333333333333))))))) / (assign51960_e66732 * assign51960_e66732))), (-((1e-200 * ((locals.var_udse__blk1406_dn7 * assign51960_e66730) + (assign51960_e66715 * (0.5 * ((locals.var_udse__blk1406_dn7 * assign51960_e66727) + (assign51960_e66720 * (locals.var_udse__blk1406_dn7 * 0.3333333333333333))))))) / (assign51960_e66732 * assign51960_e66732))), (-((1e-200 * ((locals.var_udse__blk1406_dn8 * assign51960_e66730) + (assign51960_e66715 * (0.5 * ((locals.var_udse__blk1406_dn8 * assign51960_e66727) + (assign51960_e66720 * (locals.var_udse__blk1406_dn8 * 0.3333333333333333))))))) / (assign51960_e66732 * assign51960_e66732))), (-((1e-200 * ((locals.var_udse__blk1406_dn9 * assign51960_e66730) + (assign51960_e66715 * (0.5 * ((locals.var_udse__blk1406_dn9 * assign51960_e66727) + (assign51960_e66720 * (locals.var_udse__blk1406_dn9 * 0.3333333333333333))))))) / (assign51960_e66732 * assign51960_e66732))),)
    } else {
        (locals.var_k_ds__blk1408, locals.var_k_ds__blk1408_dn4, locals.var_k_ds__blk1408_dn6, locals.var_k_ds__blk1408_dn7, locals.var_k_ds__blk1408_dn8, locals.var_k_ds__blk1408_dn9,)
    }
};
        locals.var_k_ds__blk1408 = assign51960_e66735;
        locals.var_k_ds__blk1408_dn4 = assign51960_e66735_d_n4;
        locals.var_k_ds__blk1408_dn6 = assign51960_e66735_d_n6;
        locals.var_k_ds__blk1408_dn7 = assign51960_e66735_d_n7;
        locals.var_k_ds__blk1408_dn8 = assign51960_e66735_d_n8;
        locals.var_k_ds__blk1408_dn9 = assign51960_e66735_d_n9;

        let (assign51970_e66743, assign51970_e66743_d_n4, assign51970_e66743_d_n6, assign51970_e66743_d_n7, assign51970_e66743_d_n8, assign51970_e66743_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign51970_e66741: f64 = (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408);
        (assign51970_e66741, ((locals.var_delta_ns__blk1364_dn4 * locals.var_k_ds__blk1408) + (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408_dn4)), ((locals.var_delta_ns__blk1364_dn6 * locals.var_k_ds__blk1408) + (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408_dn6)), ((locals.var_delta_ns__blk1364_dn7 * locals.var_k_ds__blk1408) + (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408_dn7)), ((locals.var_delta_ns__blk1364_dn8 * locals.var_k_ds__blk1408) + (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408_dn8)), ((locals.var_delta_ns__blk1364_dn9 * locals.var_k_ds__blk1408) + (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408_dn9)),)
    } else {
        (locals.var_delta_nd__blk1409, locals.var_delta_nd__blk1409_dn4, locals.var_delta_nd__blk1409_dn6, locals.var_delta_nd__blk1409_dn7, locals.var_delta_nd__blk1409_dn8, locals.var_delta_nd__blk1409_dn9,)
    }
};
        locals.var_delta_nd__blk1409 = assign51970_e66743;
        locals.var_delta_nd__blk1409_dn4 = assign51970_e66743_d_n4;
        locals.var_delta_nd__blk1409_dn6 = assign51970_e66743_d_n6;
        locals.var_delta_nd__blk1409_dn7 = assign51970_e66743_d_n7;
        locals.var_delta_nd__blk1409_dn8 = assign51970_e66743_d_n8;
        locals.var_delta_nd__blk1409_dn9 = assign51970_e66743_d_n9;

        let assign51980_e66745: f64 = (locals.var_xg__blk1343).abs();
        let assign51980_e66747: f64 = if assign51980_e66745 <= locals.var_margin__blk1361 { 1.0 } else { 0.0 };
        locals.var_guard1509 = assign51980_e66747;

        let (assign51990_e66761, assign51990_e66761_d_n4, assign51990_e66761_d_n6, assign51990_e66761_d_n7, assign51990_e66761_d_n8, assign51990_e66761_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 != 0.0)) {
        let assign51990_e66755: f64 = (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362);
        let assign51990_e66757: f64 = (assign51990_e66755 * 0.16666666666666666);
        let assign51990_e66759: f64 = (assign51990_e66757 * 0.7071067811865475);
        (assign51990_e66759, ((((locals.var_inv_xi__blk1362_dn4 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn4)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn6 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn7 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn8 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn8)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn9 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn9)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_sp_s_temp1__blk1449, locals.var_sp_s_temp1__blk1449_dn4, locals.var_sp_s_temp1__blk1449_dn6, locals.var_sp_s_temp1__blk1449_dn7, locals.var_sp_s_temp1__blk1449_dn8, locals.var_sp_s_temp1__blk1449_dn9,)
    }
};
        locals.var_sp_s_temp1__blk1449 = assign51990_e66761;
        locals.var_sp_s_temp1__blk1449_dn4 = assign51990_e66761_d_n4;
        locals.var_sp_s_temp1__blk1449_dn6 = assign51990_e66761_d_n6;
        locals.var_sp_s_temp1__blk1449_dn7 = assign51990_e66761_d_n7;
        locals.var_sp_s_temp1__blk1449_dn8 = assign51990_e66761_d_n8;
        locals.var_sp_s_temp1__blk1449_dn9 = assign51990_e66761_d_n9;

        let (assign52000_e66783, assign52000_e66783_d_n4, assign52000_e66783_d_n6, assign52000_e66783_d_n7, assign52000_e66783_d_n8, assign52000_e66783_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 != 0.0)) {
        let assign52000_e66769: f64 = (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362);
        let assign52000_e66774: f64 = (1.0 - locals.var_delta_nd__blk1409);
        let assign52000_e66775: f64 = (locals.var_xg__blk1343 * assign52000_e66774);
        let assign52000_e66777: f64 = (assign52000_e66775 * locals.var_gf__blk1324);
        let assign52000_e66779: f64 = (assign52000_e66777 * locals.var_sp_s_temp1__blk1449);
        let assign52000_e66780: f64 = (1.0 + assign52000_e66779);
        let assign52000_e66781: f64 = (assign52000_e66769 * assign52000_e66780);
        (assign52000_e66781, ((((locals.var_xg__blk1343_dn4 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn4)) * assign52000_e66780) + (assign52000_e66769 * ((((((locals.var_xg__blk1343_dn4 * assign52000_e66774) + (locals.var_xg__blk1343 * (-locals.var_delta_nd__blk1409_dn4))) * locals.var_gf__blk1324) + (assign52000_e66775 * locals.var_gf__blk1324_dn4)) * locals.var_sp_s_temp1__blk1449) + (assign52000_e66777 * locals.var_sp_s_temp1__blk1449_dn4)))), ((((locals.var_xg__blk1343_dn6 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn6)) * assign52000_e66780) + (assign52000_e66769 * ((((((locals.var_xg__blk1343_dn6 * assign52000_e66774) + (locals.var_xg__blk1343 * (-locals.var_delta_nd__blk1409_dn6))) * locals.var_gf__blk1324) + (assign52000_e66775 * locals.var_gf__blk1324_dn6)) * locals.var_sp_s_temp1__blk1449) + (assign52000_e66777 * locals.var_sp_s_temp1__blk1449_dn6)))), ((((locals.var_xg__blk1343_dn7 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn7)) * assign52000_e66780) + (assign52000_e66769 * ((((((locals.var_xg__blk1343_dn7 * assign52000_e66774) + (locals.var_xg__blk1343 * (-locals.var_delta_nd__blk1409_dn7))) * locals.var_gf__blk1324) + (assign52000_e66775 * locals.var_gf__blk1324_dn7)) * locals.var_sp_s_temp1__blk1449) + (assign52000_e66777 * locals.var_sp_s_temp1__blk1449_dn7)))), ((((locals.var_xg__blk1343_dn8 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn8)) * assign52000_e66780) + (assign52000_e66769 * ((((((locals.var_xg__blk1343_dn8 * assign52000_e66774) + (locals.var_xg__blk1343 * (-locals.var_delta_nd__blk1409_dn8))) * locals.var_gf__blk1324) + (assign52000_e66775 * locals.var_gf__blk1324_dn8)) * locals.var_sp_s_temp1__blk1449) + (assign52000_e66777 * locals.var_sp_s_temp1__blk1449_dn8)))), ((((locals.var_xg__blk1343_dn9 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn9)) * assign52000_e66780) + (assign52000_e66769 * ((((((locals.var_xg__blk1343_dn9 * assign52000_e66774) + (locals.var_xg__blk1343 * (-locals.var_delta_nd__blk1409_dn9))) * locals.var_gf__blk1324) + (assign52000_e66775 * locals.var_gf__blk1324_dn9)) * locals.var_sp_s_temp1__blk1449) + (assign52000_e66777 * locals.var_sp_s_temp1__blk1449_dn9)))),)
    } else {
        (locals.var_x_d__blk1410, locals.var_x_d__blk1410_dn4, locals.var_x_d__blk1410_dn6, locals.var_x_d__blk1410_dn7, locals.var_x_d__blk1410_dn8, locals.var_x_d__blk1410_dn9,)
    }
};
        locals.var_x_d__blk1410 = assign52000_e66783;
        locals.var_x_d__blk1410_dn4 = assign52000_e66783_d_n4;
        locals.var_x_d__blk1410_dn6 = assign52000_e66783_d_n6;
        locals.var_x_d__blk1410_dn7 = assign52000_e66783_d_n7;
        locals.var_x_d__blk1410_dn8 = assign52000_e66783_d_n8;
        locals.var_x_d__blk1410_dn9 = assign52000_e66783_d_n9;

    }

    pub(super) fn stamp_transient_block_46(
        locals: &mut StampLocals,
    ) {
        let (assign52010_e66794, assign52010_e66794_d_n4, assign52010_e66794_d_n6, assign52010_e66794_d_n7, assign52010_e66794_d_n8, assign52010_e66794_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52010_e66792: f64 = (locals.var_xn_d__blk1407 + 3.0);
        (assign52010_e66792, locals.var_xn_d__blk1407_dn4, locals.var_xn_d__blk1407_dn6, locals.var_xn_d__blk1407_dn7, locals.var_xn_d__blk1407_dn8, locals.var_xn_d__blk1407_dn9,)
    } else {
        (locals.var_sp_s_bx__blk1470, locals.var_sp_s_bx__blk1470_dn4, locals.var_sp_s_bx__blk1470_dn6, locals.var_sp_s_bx__blk1470_dn7, locals.var_sp_s_bx__blk1470_dn8, locals.var_sp_s_bx__blk1470_dn9,)
    }
};
        locals.var_sp_s_bx__blk1470 = assign52010_e66794;
        locals.var_sp_s_bx__blk1470_dn4 = assign52010_e66794_d_n4;
        locals.var_sp_s_bx__blk1470_dn6 = assign52010_e66794_d_n6;
        locals.var_sp_s_bx__blk1470_dn7 = assign52010_e66794_d_n7;
        locals.var_sp_s_bx__blk1470_dn8 = assign52010_e66794_d_n8;
        locals.var_sp_s_bx__blk1470_dn9 = assign52010_e66794_d_n9;

        let (assign52020_e66829, assign52020_e66829_d_n4, assign52020_e66829_d_n6, assign52020_e66829_d_n7, assign52020_e66829_d_n8, assign52020_e66829_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52020_e66804: f64 = (locals.var_sp_s_x1__blk1469 + locals.var_sp_s_bx__blk1470);
        let assign52020_e66807: f64 = (locals.var_sp_s_x1__blk1469 - locals.var_sp_s_bx__blk1470);
        let assign52020_e66810: f64 = (locals.var_sp_s_x1__blk1469 - locals.var_sp_s_bx__blk1470);
        let assign52020_e66811: f64 = (assign52020_e66807 * assign52020_e66810);
        let assign52020_e66813: f64 = (assign52020_e66811 + 5.0);
        let assign52020_e66814: f64 = (assign52020_e66813).sqrt();
        let assign52020_e66815: f64 = (assign52020_e66804 - assign52020_e66814);
        let assign52020_e66816: f64 = (0.5 * assign52020_e66815);
        let assign52020_e66821: f64 = (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470);
        let assign52020_e66823: f64 = (assign52020_e66821 + 5.0);
        let assign52020_e66824: f64 = (assign52020_e66823).sqrt();
        let assign52020_e66825: f64 = (locals.var_sp_s_bx__blk1470 - assign52020_e66824);
        let assign52020_e66826: f64 = (0.5 * assign52020_e66825);
        let assign52020_e66827: f64 = (assign52020_e66816 - assign52020_e66826);
        (assign52020_e66827, ((0.5 * ((locals.var_sp_s_x1__blk1469_dn4 + locals.var_sp_s_bx__blk1470_dn4) - ((((locals.var_sp_s_x1__blk1469_dn4 - locals.var_sp_s_bx__blk1470_dn4) * assign52020_e66810) + (assign52020_e66807 * (locals.var_sp_s_x1__blk1469_dn4 - locals.var_sp_s_bx__blk1470_dn4))) / (2.0 * assign52020_e66814)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn4 - (((locals.var_sp_s_bx__blk1470_dn4 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn4)) / (2.0 * assign52020_e66824))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn6 + locals.var_sp_s_bx__blk1470_dn6) - ((((locals.var_sp_s_x1__blk1469_dn6 - locals.var_sp_s_bx__blk1470_dn6) * assign52020_e66810) + (assign52020_e66807 * (locals.var_sp_s_x1__blk1469_dn6 - locals.var_sp_s_bx__blk1470_dn6))) / (2.0 * assign52020_e66814)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn6 - (((locals.var_sp_s_bx__blk1470_dn6 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn6)) / (2.0 * assign52020_e66824))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn7 + locals.var_sp_s_bx__blk1470_dn7) - ((((locals.var_sp_s_x1__blk1469_dn7 - locals.var_sp_s_bx__blk1470_dn7) * assign52020_e66810) + (assign52020_e66807 * (locals.var_sp_s_x1__blk1469_dn7 - locals.var_sp_s_bx__blk1470_dn7))) / (2.0 * assign52020_e66814)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn7 - (((locals.var_sp_s_bx__blk1470_dn7 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn7)) / (2.0 * assign52020_e66824))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn8 + locals.var_sp_s_bx__blk1470_dn8) - ((((locals.var_sp_s_x1__blk1469_dn8 - locals.var_sp_s_bx__blk1470_dn8) * assign52020_e66810) + (assign52020_e66807 * (locals.var_sp_s_x1__blk1469_dn8 - locals.var_sp_s_bx__blk1470_dn8))) / (2.0 * assign52020_e66814)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn8 - (((locals.var_sp_s_bx__blk1470_dn8 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn8)) / (2.0 * assign52020_e66824))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn9 + locals.var_sp_s_bx__blk1470_dn9) - ((((locals.var_sp_s_x1__blk1469_dn9 - locals.var_sp_s_bx__blk1470_dn9) * assign52020_e66810) + (assign52020_e66807 * (locals.var_sp_s_x1__blk1469_dn9 - locals.var_sp_s_bx__blk1470_dn9))) / (2.0 * assign52020_e66814)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn9 - (((locals.var_sp_s_bx__blk1470_dn9 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn9)) / (2.0 * assign52020_e66824))))),)
    } else {
        (locals.var_sp_s_eta__blk1453, locals.var_sp_s_eta__blk1453_dn4, locals.var_sp_s_eta__blk1453_dn6, locals.var_sp_s_eta__blk1453_dn7, locals.var_sp_s_eta__blk1453_dn8, locals.var_sp_s_eta__blk1453_dn9,)
    }
};
        locals.var_sp_s_eta__blk1453 = assign52020_e66829;
        locals.var_sp_s_eta__blk1453_dn4 = assign52020_e66829_d_n4;
        locals.var_sp_s_eta__blk1453_dn6 = assign52020_e66829_d_n6;
        locals.var_sp_s_eta__blk1453_dn7 = assign52020_e66829_d_n7;
        locals.var_sp_s_eta__blk1453_dn8 = assign52020_e66829_d_n8;
        locals.var_sp_s_eta__blk1453_dn9 = assign52020_e66829_d_n9;

        let (assign52030_e66840, assign52030_e66840_d_n4, assign52030_e66840_d_n6, assign52030_e66840_d_n7, assign52030_e66840_d_n8, assign52030_e66840_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52030_e66838: f64 = (locals.var_xg__blk1343 - locals.var_sp_s_eta__blk1453);
        (assign52030_e66838, (locals.var_xg__blk1343_dn4 - locals.var_sp_s_eta__blk1453_dn4), (locals.var_xg__blk1343_dn6 - locals.var_sp_s_eta__blk1453_dn6), (locals.var_xg__blk1343_dn7 - locals.var_sp_s_eta__blk1453_dn7), (locals.var_xg__blk1343_dn8 - locals.var_sp_s_eta__blk1453_dn8), (locals.var_xg__blk1343_dn9 - locals.var_sp_s_eta__blk1453_dn9),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign52030_e66840;
        locals.var_sp_s_temp__blk1448_dn4 = assign52030_e66840_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign52030_e66840_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign52030_e66840_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign52030_e66840_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign52030_e66840_d_n9;

        let (assign52040_e66851, assign52040_e66851_d_n4, assign52040_e66851_d_n6, assign52040_e66851_d_n7, assign52040_e66851_d_n8, assign52040_e66851_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52040_e66848: f64 = (-locals.var_sp_s_eta__blk1453);
        let assign52040_e66849: f64 = (assign52040_e66848).exp();
        (assign52040_e66849, (assign52040_e66849 * (-locals.var_sp_s_eta__blk1453_dn4)), (assign52040_e66849 * (-locals.var_sp_s_eta__blk1453_dn6)), (assign52040_e66849 * (-locals.var_sp_s_eta__blk1453_dn7)), (assign52040_e66849 * (-locals.var_sp_s_eta__blk1453_dn8)), (assign52040_e66849 * (-locals.var_sp_s_eta__blk1453_dn9)),)
    } else {
        (locals.var_sp_s_temp1__blk1449, locals.var_sp_s_temp1__blk1449_dn4, locals.var_sp_s_temp1__blk1449_dn6, locals.var_sp_s_temp1__blk1449_dn7, locals.var_sp_s_temp1__blk1449_dn8, locals.var_sp_s_temp1__blk1449_dn9,)
    }
};
        locals.var_sp_s_temp1__blk1449 = assign52040_e66851;
        locals.var_sp_s_temp1__blk1449_dn4 = assign52040_e66851_d_n4;
        locals.var_sp_s_temp1__blk1449_dn6 = assign52040_e66851_d_n6;
        locals.var_sp_s_temp1__blk1449_dn7 = assign52040_e66851_d_n7;
        locals.var_sp_s_temp1__blk1449_dn8 = assign52040_e66851_d_n8;
        locals.var_sp_s_temp1__blk1449_dn9 = assign52040_e66851_d_n9;

        let (assign52050_e66866, assign52050_e66866_d_n4, assign52050_e66866_d_n6, assign52050_e66866_d_n7, assign52050_e66866_d_n8, assign52050_e66866_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52050_e66862: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453);
        let assign52050_e66863: f64 = (2.0 + assign52050_e66862);
        let assign52050_e66864: f64 = (1.0 / assign52050_e66863);
        (assign52050_e66864, (-(((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn4)) / (assign52050_e66863 * assign52050_e66863))), (-(((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn6)) / (assign52050_e66863 * assign52050_e66863))), (-(((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn7)) / (assign52050_e66863 * assign52050_e66863))), (-(((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn8)) / (assign52050_e66863 * assign52050_e66863))), (-(((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn9)) / (assign52050_e66863 * assign52050_e66863))),)
    } else {
        (locals.var_sp_s_temp2__blk1450, locals.var_sp_s_temp2__blk1450_dn4, locals.var_sp_s_temp2__blk1450_dn6, locals.var_sp_s_temp2__blk1450_dn7, locals.var_sp_s_temp2__blk1450_dn8, locals.var_sp_s_temp2__blk1450_dn9,)
    }
};
        locals.var_sp_s_temp2__blk1450 = assign52050_e66866;
        locals.var_sp_s_temp2__blk1450_dn4 = assign52050_e66866_d_n4;
        locals.var_sp_s_temp2__blk1450_dn6 = assign52050_e66866_d_n6;
        locals.var_sp_s_temp2__blk1450_dn7 = assign52050_e66866_d_n7;
        locals.var_sp_s_temp2__blk1450_dn8 = assign52050_e66866_d_n8;
        locals.var_sp_s_temp2__blk1450_dn9 = assign52050_e66866_d_n9;

        let (assign52060_e66879, assign52060_e66879_d_n4, assign52060_e66879_d_n6, assign52060_e66879_d_n7, assign52060_e66879_d_n8, assign52060_e66879_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52060_e66875: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453);
        let assign52060_e66877: f64 = (assign52060_e66875 * locals.var_sp_s_temp2__blk1450);
        (assign52060_e66877, ((((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign52060_e66875 * locals.var_sp_s_temp2__blk1450_dn4)), ((((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign52060_e66875 * locals.var_sp_s_temp2__blk1450_dn6)), ((((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign52060_e66875 * locals.var_sp_s_temp2__blk1450_dn7)), ((((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign52060_e66875 * locals.var_sp_s_temp2__blk1450_dn8)), ((((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign52060_e66875 * locals.var_sp_s_temp2__blk1450_dn9)),)
    } else {
        (locals.var_sp_s_xi0__blk1460, locals.var_sp_s_xi0__blk1460_dn4, locals.var_sp_s_xi0__blk1460_dn6, locals.var_sp_s_xi0__blk1460_dn7, locals.var_sp_s_xi0__blk1460_dn8, locals.var_sp_s_xi0__blk1460_dn9,)
    }
};
        locals.var_sp_s_xi0__blk1460 = assign52060_e66879;
        locals.var_sp_s_xi0__blk1460_dn4 = assign52060_e66879_d_n4;
        locals.var_sp_s_xi0__blk1460_dn6 = assign52060_e66879_d_n6;
        locals.var_sp_s_xi0__blk1460_dn7 = assign52060_e66879_d_n7;
        locals.var_sp_s_xi0__blk1460_dn8 = assign52060_e66879_d_n8;
        locals.var_sp_s_xi0__blk1460_dn9 = assign52060_e66879_d_n9;

        let (assign52070_e66894, assign52070_e66894_d_n4, assign52070_e66894_d_n6, assign52070_e66894_d_n7, assign52070_e66894_d_n8, assign52070_e66894_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52070_e66889: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450);
        let assign52070_e66891: f64 = (assign52070_e66889 * locals.var_sp_s_temp2__blk1450);
        let assign52070_e66892: f64 = (4.0 * assign52070_e66891);
        (assign52070_e66892, (4.0 * ((((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign52070_e66889 * locals.var_sp_s_temp2__blk1450_dn4))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign52070_e66889 * locals.var_sp_s_temp2__blk1450_dn6))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign52070_e66889 * locals.var_sp_s_temp2__blk1450_dn7))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign52070_e66889 * locals.var_sp_s_temp2__blk1450_dn8))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign52070_e66889 * locals.var_sp_s_temp2__blk1450_dn9))),)
    } else {
        (locals.var_sp_s_xi1__blk1461, locals.var_sp_s_xi1__blk1461_dn4, locals.var_sp_s_xi1__blk1461_dn6, locals.var_sp_s_xi1__blk1461_dn7, locals.var_sp_s_xi1__blk1461_dn8, locals.var_sp_s_xi1__blk1461_dn9,)
    }
};
        locals.var_sp_s_xi1__blk1461 = assign52070_e66894;
        locals.var_sp_s_xi1__blk1461_dn4 = assign52070_e66894_d_n4;
        locals.var_sp_s_xi1__blk1461_dn6 = assign52070_e66894_d_n6;
        locals.var_sp_s_xi1__blk1461_dn7 = assign52070_e66894_d_n7;
        locals.var_sp_s_xi1__blk1461_dn8 = assign52070_e66894_d_n8;
        locals.var_sp_s_xi1__blk1461_dn9 = assign52070_e66894_d_n9;

        let (assign52080_e66913, assign52080_e66913_d_n4, assign52080_e66913_d_n6, assign52080_e66913_d_n7, assign52080_e66913_d_n8, assign52080_e66913_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52080_e66903: f64 = (8.0 * locals.var_sp_s_temp2__blk1450);
        let assign52080_e66906: f64 = (12.0 * locals.var_sp_s_xi0__blk1460);
        let assign52080_e66907: f64 = (assign52080_e66903 - assign52080_e66906);
        let assign52080_e66909: f64 = (assign52080_e66907 * locals.var_sp_s_temp2__blk1450);
        let assign52080_e66911: f64 = (assign52080_e66909 * locals.var_sp_s_temp2__blk1450);
        (assign52080_e66911, ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn4) - (12.0 * locals.var_sp_s_xi0__blk1460_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66907 * locals.var_sp_s_temp2__blk1450_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66909 * locals.var_sp_s_temp2__blk1450_dn4)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn6) - (12.0 * locals.var_sp_s_xi0__blk1460_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66907 * locals.var_sp_s_temp2__blk1450_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66909 * locals.var_sp_s_temp2__blk1450_dn6)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn7) - (12.0 * locals.var_sp_s_xi0__blk1460_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66907 * locals.var_sp_s_temp2__blk1450_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66909 * locals.var_sp_s_temp2__blk1450_dn7)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn8) - (12.0 * locals.var_sp_s_xi0__blk1460_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66907 * locals.var_sp_s_temp2__blk1450_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66909 * locals.var_sp_s_temp2__blk1450_dn8)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn9) - (12.0 * locals.var_sp_s_xi0__blk1460_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66907 * locals.var_sp_s_temp2__blk1450_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66909 * locals.var_sp_s_temp2__blk1450_dn9)),)
    } else {
        (locals.var_sp_s_xi2__blk1462, locals.var_sp_s_xi2__blk1462_dn4, locals.var_sp_s_xi2__blk1462_dn6, locals.var_sp_s_xi2__blk1462_dn7, locals.var_sp_s_xi2__blk1462_dn8, locals.var_sp_s_xi2__blk1462_dn9,)
    }
};
        locals.var_sp_s_xi2__blk1462 = assign52080_e66913;
        locals.var_sp_s_xi2__blk1462_dn4 = assign52080_e66913_d_n4;
        locals.var_sp_s_xi2__blk1462_dn6 = assign52080_e66913_d_n6;
        locals.var_sp_s_xi2__blk1462_dn7 = assign52080_e66913_d_n7;
        locals.var_sp_s_xi2__blk1462_dn8 = assign52080_e66913_d_n8;
        locals.var_sp_s_xi2__blk1462_dn9 = assign52080_e66913_d_n9;

        let (assign52090_e66963, assign52090_e66963_d_n4, assign52090_e66963_d_n6, assign52090_e66963_d_n7, assign52090_e66963_d_n8, assign52090_e66963_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52090_e66923: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
        let assign52090_e66927: f64 = (locals.var_sp_s_temp1__blk1449 + locals.var_sp_s_eta__blk1453);
        let assign52090_e66929: f64 = (assign52090_e66927 - 1.0);
        let assign52090_e66933: f64 = (locals.var_sp_s_eta__blk1453 + 1.0);
        let assign52090_e66935: f64 = (assign52090_e66933 + locals.var_sp_s_xi0__blk1460);
        let assign52090_e66936: f64 = (locals.var_delta_nd__blk1409 * assign52090_e66935);
        let assign52090_e66937: f64 = (assign52090_e66929 - assign52090_e66936);
        let assign52090_e66938: f64 = (locals.var_gf2__blk1325 * assign52090_e66937);
        let assign52090_e66939: f64 = (assign52090_e66923 - assign52090_e66938);
        let (assign52090_e66961, assign52090_e66961_d_n4, assign52090_e66961_d_n6, assign52090_e66961_d_n7, assign52090_e66961_d_n8, assign52090_e66961_d_n9,) = {
            if (1e-40 > assign52090_e66939) {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign52090_e66944: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
                let assign52090_e66948: f64 = (locals.var_sp_s_temp1__blk1449 + locals.var_sp_s_eta__blk1453);
                let assign52090_e66950: f64 = (assign52090_e66948 - 1.0);
                let assign52090_e66954: f64 = (locals.var_sp_s_eta__blk1453 + 1.0);
                let assign52090_e66956: f64 = (assign52090_e66954 + locals.var_sp_s_xi0__blk1460);
                let assign52090_e66957: f64 = (locals.var_delta_nd__blk1409 * assign52090_e66956);
                let assign52090_e66958: f64 = (assign52090_e66950 - assign52090_e66957);
                let assign52090_e66959: f64 = (locals.var_gf2__blk1325 * assign52090_e66958);
                let assign52090_e66960: f64 = (assign52090_e66944 - assign52090_e66959);
                (assign52090_e66960, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) - ((locals.var_gf2__blk1325_dn4 * assign52090_e66958) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn4 + locals.var_sp_s_eta__blk1453_dn4) - ((locals.var_delta_nd__blk1409_dn4 * assign52090_e66956) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_eta__blk1453_dn4 + locals.var_sp_s_xi0__blk1460_dn4))))))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) - ((locals.var_gf2__blk1325_dn6 * assign52090_e66958) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn6 + locals.var_sp_s_eta__blk1453_dn6) - ((locals.var_delta_nd__blk1409_dn6 * assign52090_e66956) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_eta__blk1453_dn6 + locals.var_sp_s_xi0__blk1460_dn6))))))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) - ((locals.var_gf2__blk1325_dn7 * assign52090_e66958) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn7 + locals.var_sp_s_eta__blk1453_dn7) - ((locals.var_delta_nd__blk1409_dn7 * assign52090_e66956) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_eta__blk1453_dn7 + locals.var_sp_s_xi0__blk1460_dn7))))))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) - ((locals.var_gf2__blk1325_dn8 * assign52090_e66958) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn8 + locals.var_sp_s_eta__blk1453_dn8) - ((locals.var_delta_nd__blk1409_dn8 * assign52090_e66956) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_eta__blk1453_dn8 + locals.var_sp_s_xi0__blk1460_dn8))))))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) - ((locals.var_gf2__blk1325_dn9 * assign52090_e66958) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn9 + locals.var_sp_s_eta__blk1453_dn9) - ((locals.var_delta_nd__blk1409_dn9 * assign52090_e66956) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_eta__blk1453_dn9 + locals.var_sp_s_xi0__blk1460_dn9))))))),)
            }
        };
        (assign52090_e66961, assign52090_e66961_d_n4, assign52090_e66961_d_n6, assign52090_e66961_d_n7, assign52090_e66961_d_n8, assign52090_e66961_d_n9,)
    } else {
        (locals.var_sp_s_a__blk1454, locals.var_sp_s_a__blk1454_dn4, locals.var_sp_s_a__blk1454_dn6, locals.var_sp_s_a__blk1454_dn7, locals.var_sp_s_a__blk1454_dn8, locals.var_sp_s_a__blk1454_dn9,)
    }
};
        locals.var_sp_s_a__blk1454 = assign52090_e66963;
        locals.var_sp_s_a__blk1454_dn4 = assign52090_e66963_d_n4;
        locals.var_sp_s_a__blk1454_dn6 = assign52090_e66963_d_n6;
        locals.var_sp_s_a__blk1454_dn7 = assign52090_e66963_d_n7;
        locals.var_sp_s_a__blk1454_dn8 = assign52090_e66963_d_n8;
        locals.var_sp_s_a__blk1454_dn9 = assign52090_e66963_d_n9;

        let (assign52100_e66982, assign52100_e66982_d_n4, assign52100_e66982_d_n6, assign52100_e66982_d_n7, assign52100_e66982_d_n8, assign52100_e66982_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52100_e66976: f64 = (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462);
        let assign52100_e66977: f64 = (locals.var_sp_s_temp1__blk1449 - assign52100_e66976);
        let assign52100_e66978: f64 = (locals.var_gf2__blk1325 * assign52100_e66977);
        let assign52100_e66979: f64 = (0.5 * assign52100_e66978);
        let assign52100_e66980: f64 = (1.0 - assign52100_e66979);
        (assign52100_e66980, (-(0.5 * ((locals.var_gf2__blk1325_dn4 * assign52100_e66977) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn4 - ((locals.var_delta_nd__blk1409_dn4 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn4))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn6 * assign52100_e66977) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn6 - ((locals.var_delta_nd__blk1409_dn6 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn6))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn7 * assign52100_e66977) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn7 - ((locals.var_delta_nd__blk1409_dn7 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn7))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn8 * assign52100_e66977) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn8 - ((locals.var_delta_nd__blk1409_dn8 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn8))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn9 * assign52100_e66977) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn9 - ((locals.var_delta_nd__blk1409_dn9 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn9))))))),)
    } else {
        (locals.var_sp_s_b__blk1471, locals.var_sp_s_b__blk1471_dn4, locals.var_sp_s_b__blk1471_dn6, locals.var_sp_s_b__blk1471_dn7, locals.var_sp_s_b__blk1471_dn8, locals.var_sp_s_b__blk1471_dn9,)
    }
};
        locals.var_sp_s_b__blk1471 = assign52100_e66982;
        locals.var_sp_s_b__blk1471_dn4 = assign52100_e66982_d_n4;
        locals.var_sp_s_b__blk1471_dn6 = assign52100_e66982_d_n6;
        locals.var_sp_s_b__blk1471_dn7 = assign52100_e66982_d_n7;
        locals.var_sp_s_b__blk1471_dn8 = assign52100_e66982_d_n8;
        locals.var_sp_s_b__blk1471_dn9 = assign52100_e66982_d_n9;

        let (assign52110_e67005, assign52110_e67005_d_n4, assign52110_e67005_d_n6, assign52110_e67005_d_n7, assign52110_e67005_d_n8, assign52110_e67005_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52110_e66991: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
        let assign52110_e66995: f64 = (1.0 - locals.var_sp_s_temp1__blk1449);
        let assign52110_e66999: f64 = (1.0 + locals.var_sp_s_xi1__blk1461);
        let assign52110_e67000: f64 = (locals.var_delta_nd__blk1409 * assign52110_e66999);
        let assign52110_e67001: f64 = (assign52110_e66995 - assign52110_e67000);
        let assign52110_e67002: f64 = (locals.var_gf2__blk1325 * assign52110_e67001);
        let assign52110_e67003: f64 = (assign52110_e66991 + assign52110_e67002);
        (assign52110_e67003, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) + ((locals.var_gf2__blk1325_dn4 * assign52110_e67001) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn4) - ((locals.var_delta_nd__blk1409_dn4 * assign52110_e66999) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn4)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) + ((locals.var_gf2__blk1325_dn6 * assign52110_e67001) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn6) - ((locals.var_delta_nd__blk1409_dn6 * assign52110_e66999) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) + ((locals.var_gf2__blk1325_dn7 * assign52110_e67001) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn7) - ((locals.var_delta_nd__blk1409_dn7 * assign52110_e66999) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) + ((locals.var_gf2__blk1325_dn8 * assign52110_e67001) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn8) - ((locals.var_delta_nd__blk1409_dn8 * assign52110_e66999) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn8)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) + ((locals.var_gf2__blk1325_dn9 * assign52110_e67001) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn9) - ((locals.var_delta_nd__blk1409_dn9 * assign52110_e66999) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn9)))))),)
    } else {
        (locals.var_sp_s_c__blk1455, locals.var_sp_s_c__blk1455_dn4, locals.var_sp_s_c__blk1455_dn6, locals.var_sp_s_c__blk1455_dn7, locals.var_sp_s_c__blk1455_dn8, locals.var_sp_s_c__blk1455_dn9,)
    }
};
        locals.var_sp_s_c__blk1455 = assign52110_e67005;
        locals.var_sp_s_c__blk1455_dn4 = assign52110_e67005_d_n4;
        locals.var_sp_s_c__blk1455_dn6 = assign52110_e67005_d_n6;
        locals.var_sp_s_c__blk1455_dn7 = assign52110_e67005_d_n7;
        locals.var_sp_s_c__blk1455_dn8 = assign52110_e67005_d_n8;
        locals.var_sp_s_c__blk1455_dn9 = assign52110_e67005_d_n9;

        let (assign52120_e67021, assign52120_e67021_d_n4, assign52120_e67021_d_n6, assign52120_e67021_d_n7, assign52120_e67021_d_n8, assign52120_e67021_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52120_e67014: f64 = (locals.var_xn_d__blk1407 - locals.var_sp_s_eta__blk1453);
        let assign52120_e67017: f64 = (locals.var_sp_s_a__blk1454 / locals.var_gf2__blk1325);
        let assign52120_e67018: f64 = (assign52120_e67017).ln();
        let assign52120_e67019: f64 = (assign52120_e67014 + assign52120_e67018);
        (assign52120_e67019, ((locals.var_xn_d__blk1407_dn4 - locals.var_sp_s_eta__blk1453_dn4) + ((((locals.var_sp_s_a__blk1454_dn4 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn4)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign52120_e67017)), ((locals.var_xn_d__blk1407_dn6 - locals.var_sp_s_eta__blk1453_dn6) + ((((locals.var_sp_s_a__blk1454_dn6 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn6)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign52120_e67017)), ((locals.var_xn_d__blk1407_dn7 - locals.var_sp_s_eta__blk1453_dn7) + ((((locals.var_sp_s_a__blk1454_dn7 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn7)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign52120_e67017)), ((locals.var_xn_d__blk1407_dn8 - locals.var_sp_s_eta__blk1453_dn8) + ((((locals.var_sp_s_a__blk1454_dn8 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn8)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign52120_e67017)), ((locals.var_xn_d__blk1407_dn9 - locals.var_sp_s_eta__blk1453_dn9) + ((((locals.var_sp_s_a__blk1454_dn9 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn9)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign52120_e67017)),)
    } else {
        (locals.var_sp_s_tau__blk1456, locals.var_sp_s_tau__blk1456_dn4, locals.var_sp_s_tau__blk1456_dn6, locals.var_sp_s_tau__blk1456_dn7, locals.var_sp_s_tau__blk1456_dn8, locals.var_sp_s_tau__blk1456_dn9,)
    }
};
        locals.var_sp_s_tau__blk1456 = assign52120_e67021;
        locals.var_sp_s_tau__blk1456_dn4 = assign52120_e67021_d_n4;
        locals.var_sp_s_tau__blk1456_dn6 = assign52120_e67021_d_n6;
        locals.var_sp_s_tau__blk1456_dn7 = assign52120_e67021_d_n7;
        locals.var_sp_s_tau__blk1456_dn8 = assign52120_e67021_d_n8;
        locals.var_sp_s_tau__blk1456_dn9 = assign52120_e67021_d_n9;

        let (assign52130_e67032, assign52130_e67032_d_n4, assign52130_e67032_d_n6, assign52130_e67032_d_n7, assign52130_e67032_d_n8, assign52130_e67032_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52130_e67030: f64 = (locals.var_sp_s_a__blk1454 + locals.var_sp_s_c__blk1455);
        (assign52130_e67030, (locals.var_sp_s_a__blk1454_dn4 + locals.var_sp_s_c__blk1455_dn4), (locals.var_sp_s_a__blk1454_dn6 + locals.var_sp_s_c__blk1455_dn6), (locals.var_sp_s_a__blk1454_dn7 + locals.var_sp_s_c__blk1455_dn7), (locals.var_sp_s_a__blk1454_dn8 + locals.var_sp_s_c__blk1455_dn8), (locals.var_sp_s_a__blk1454_dn9 + locals.var_sp_s_c__blk1455_dn9),)
    } else {
        (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9,)
    }
};
        locals.var_nu = assign52130_e67032;
        locals.var_nu_dn4 = assign52130_e67032_d_n4;
        locals.var_nu_dn6 = assign52130_e67032_d_n6;
        locals.var_nu_dn7 = assign52130_e67032_d_n7;
        locals.var_nu_dn8 = assign52130_e67032_d_n8;
        locals.var_nu_dn9 = assign52130_e67032_d_n9;

        let (assign52140_e67055, assign52140_e67055_d_n4, assign52140_e67055_d_n6, assign52140_e67055_d_n7, assign52140_e67055_d_n8, assign52140_e67055_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52140_e67041: f64 = (locals.var_nu * locals.var_nu);
        let assign52140_e67046: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
        let assign52140_e67047: f64 = (0.5 * assign52140_e67046);
        let assign52140_e67050: f64 = (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471);
        let assign52140_e67051: f64 = (assign52140_e67047 - assign52140_e67050);
        let assign52140_e67052: f64 = (locals.var_sp_s_tau__blk1456 * assign52140_e67051);
        let assign52140_e67053: f64 = (assign52140_e67041 + assign52140_e67052);
        (assign52140_e67053, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau__blk1456_dn4 * assign52140_e67051) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4))) - ((locals.var_sp_s_a__blk1454_dn4 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn4)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1456_dn6 * assign52140_e67051) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6))) - ((locals.var_sp_s_a__blk1454_dn6 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1456_dn7 * assign52140_e67051) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7))) - ((locals.var_sp_s_a__blk1454_dn7 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1456_dn8 * assign52140_e67051) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8))) - ((locals.var_sp_s_a__blk1454_dn8 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn8)))))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau__blk1456_dn9 * assign52140_e67051) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9))) - ((locals.var_sp_s_a__blk1454_dn9 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn9)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9,)
    }
};
        locals.var_mutau = assign52140_e67055;
        locals.var_mutau_dn4 = assign52140_e67055_d_n4;
        locals.var_mutau_dn6 = assign52140_e67055_d_n6;
        locals.var_mutau_dn7 = assign52140_e67055_d_n7;
        locals.var_mutau_dn8 = assign52140_e67055_d_n8;
        locals.var_mutau_dn9 = assign52140_e67055_d_n9;

        let (assign52150_e67092, assign52150_e67092_d_n4, assign52150_e67092_d_n6, assign52150_e67092_d_n7, assign52150_e67092_d_n8, assign52150_e67092_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52150_e67065: f64 = (locals.var_sp_s_a__blk1454 * locals.var_nu);
        let assign52150_e67067: f64 = (assign52150_e67065 * locals.var_sp_s_tau__blk1456);
        let assign52150_e67071: f64 = (locals.var_nu / locals.var_mutau);
        let assign52150_e67073: f64 = (assign52150_e67071 * locals.var_sp_s_tau__blk1456);
        let assign52150_e67075: f64 = (assign52150_e67073 * locals.var_sp_s_tau__blk1456);
        let assign52150_e67077: f64 = (assign52150_e67075 * locals.var_sp_s_c__blk1455);
        let assign52150_e67080: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
        let assign52150_e67082: f64 = (assign52150_e67080 * 0.3333333333333333);
        let assign52150_e67085: f64 = (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471);
        let assign52150_e67086: f64 = (assign52150_e67082 - assign52150_e67085);
        let assign52150_e67087: f64 = (assign52150_e67077 * assign52150_e67086);
        let assign52150_e67088: f64 = (locals.var_mutau + assign52150_e67087);
        let assign52150_e67089: f64 = (assign52150_e67067 / assign52150_e67088);
        let assign52150_e67090: f64 = (locals.var_sp_s_eta__blk1453 + assign52150_e67089);
        (assign52150_e67090, (locals.var_sp_s_eta__blk1453_dn4 + (((((((locals.var_sp_s_a__blk1454_dn4 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn4)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67065 * locals.var_sp_s_tau__blk1456_dn4)) * assign52150_e67088) - (assign52150_e67067 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67071 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67073 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_c__blk1455) + (assign52150_e67075 * locals.var_sp_s_c__blk1455_dn4)) * assign52150_e67086) + (assign52150_e67077 * ((((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn4 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn4)))))))) / (assign52150_e67088 * assign52150_e67088))), (locals.var_sp_s_eta__blk1453_dn6 + (((((((locals.var_sp_s_a__blk1454_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67065 * locals.var_sp_s_tau__blk1456_dn6)) * assign52150_e67088) - (assign52150_e67067 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67071 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67073 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_c__blk1455) + (assign52150_e67075 * locals.var_sp_s_c__blk1455_dn6)) * assign52150_e67086) + (assign52150_e67077 * ((((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn6 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn6)))))))) / (assign52150_e67088 * assign52150_e67088))), (locals.var_sp_s_eta__blk1453_dn7 + (((((((locals.var_sp_s_a__blk1454_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67065 * locals.var_sp_s_tau__blk1456_dn7)) * assign52150_e67088) - (assign52150_e67067 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67071 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67073 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_c__blk1455) + (assign52150_e67075 * locals.var_sp_s_c__blk1455_dn7)) * assign52150_e67086) + (assign52150_e67077 * ((((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn7 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn7)))))))) / (assign52150_e67088 * assign52150_e67088))), (locals.var_sp_s_eta__blk1453_dn8 + (((((((locals.var_sp_s_a__blk1454_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67065 * locals.var_sp_s_tau__blk1456_dn8)) * assign52150_e67088) - (assign52150_e67067 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67071 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67073 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_c__blk1455) + (assign52150_e67075 * locals.var_sp_s_c__blk1455_dn8)) * assign52150_e67086) + (assign52150_e67077 * ((((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn8 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn8)))))))) / (assign52150_e67088 * assign52150_e67088))), (locals.var_sp_s_eta__blk1453_dn9 + (((((((locals.var_sp_s_a__blk1454_dn9 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn9)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67065 * locals.var_sp_s_tau__blk1456_dn9)) * assign52150_e67088) - (assign52150_e67067 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67071 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67073 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_c__blk1455) + (assign52150_e67075 * locals.var_sp_s_c__blk1455_dn9)) * assign52150_e67086) + (assign52150_e67077 * ((((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn9 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn9)))))))) / (assign52150_e67088 * assign52150_e67088))),)
    } else {
        (locals.var_sp_s_x0__blk1472, locals.var_sp_s_x0__blk1472_dn4, locals.var_sp_s_x0__blk1472_dn6, locals.var_sp_s_x0__blk1472_dn7, locals.var_sp_s_x0__blk1472_dn8, locals.var_sp_s_x0__blk1472_dn9,)
    }
};
        locals.var_sp_s_x0__blk1472 = assign52150_e67092;
        locals.var_sp_s_x0__blk1472_dn4 = assign52150_e67092_d_n4;
        locals.var_sp_s_x0__blk1472_dn6 = assign52150_e67092_d_n6;
        locals.var_sp_s_x0__blk1472_dn7 = assign52150_e67092_d_n7;
        locals.var_sp_s_x0__blk1472_dn8 = assign52150_e67092_d_n8;
        locals.var_sp_s_x0__blk1472_dn9 = assign52150_e67092_d_n9;

        let assign52160_e67095: f64 = if locals.var_sp_s_x0__blk1472 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1510 = assign52160_e67095;

        let (assign52170_e67107, assign52170_e67107_d_n4, assign52170_e67107_d_n6, assign52170_e67107_d_n7, assign52170_e67107_d_n8, assign52170_e67107_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 != 0.0)) {
        let assign52170_e67105: f64 = (locals.var_sp_s_x0__blk1472).exp();
        (assign52170_e67105, (assign52170_e67105 * locals.var_sp_s_x0__blk1472_dn4), (assign52170_e67105 * locals.var_sp_s_x0__blk1472_dn6), (assign52170_e67105 * locals.var_sp_s_x0__blk1472_dn7), (assign52170_e67105 * locals.var_sp_s_x0__blk1472_dn8), (assign52170_e67105 * locals.var_sp_s_x0__blk1472_dn9),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign52170_e67107;
        locals.var_sp_s_delta0__blk1458_dn4 = assign52170_e67107_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign52170_e67107_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign52170_e67107_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign52170_e67107_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign52170_e67107_d_n9;

        let (assign52180_e67120, assign52180_e67120_d_n4, assign52180_e67120_d_n6, assign52180_e67120_d_n7, assign52180_e67120_d_n8, assign52180_e67120_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 != 0.0)) {
        let assign52180_e67118: f64 = (1.0 / locals.var_sp_s_delta0__blk1458);
        (assign52180_e67118, (-(locals.var_sp_s_delta0__blk1458_dn4 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn6 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn7 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn8 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn9 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))),)
    } else {
        (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9,)
    }
};
        locals.var_sp_s_delta1__blk1459 = assign52180_e67120;
        locals.var_sp_s_delta1__blk1459_dn4 = assign52180_e67120_d_n4;
        locals.var_sp_s_delta1__blk1459_dn6 = assign52180_e67120_d_n6;
        locals.var_sp_s_delta1__blk1459_dn7 = assign52180_e67120_d_n7;
        locals.var_sp_s_delta1__blk1459_dn8 = assign52180_e67120_d_n8;
        locals.var_sp_s_delta1__blk1459_dn9 = assign52180_e67120_d_n9;

        let (assign52190_e67133, assign52190_e67133_d_n4, assign52190_e67133_d_n6, assign52190_e67133_d_n7, assign52190_e67133_d_n8, assign52190_e67133_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 != 0.0)) {
        let assign52190_e67131: f64 = (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458);
        (assign52190_e67131, ((locals.var_delta_nd__blk1409_dn4 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn4)), ((locals.var_delta_nd__blk1409_dn6 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn6)), ((locals.var_delta_nd__blk1409_dn7 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn7)), ((locals.var_delta_nd__blk1409_dn8 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn8)), ((locals.var_delta_nd__blk1409_dn9 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn9)),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign52190_e67133;
        locals.var_sp_s_delta0__blk1458_dn4 = assign52190_e67133_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign52190_e67133_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign52190_e67133_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign52190_e67133_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign52190_e67133_d_n9;

        let assign52200_e67137: f64 = (locals.var_xn_d__blk1407 - 230.25850929940458);
        let assign52200_e67138: f64 = if locals.var_sp_s_x0__blk1472 > assign52200_e67137 { 1.0 } else { 0.0 };
        locals.var_guard1511 = assign52200_e67138;

        let (assign52210_e67155, assign52210_e67155_d_n4, assign52210_e67155_d_n6, assign52210_e67155_d_n7, assign52210_e67155_d_n8, assign52210_e67155_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 == 0.0)) && (locals.var_guard1511 != 0.0)) {
        let assign52210_e67152: f64 = (locals.var_sp_s_x0__blk1472 - locals.var_xn_d__blk1407);
        let assign52210_e67153: f64 = (assign52210_e67152).exp();
        (assign52210_e67153, (assign52210_e67153 * (locals.var_sp_s_x0__blk1472_dn4 - locals.var_xn_d__blk1407_dn4)), (assign52210_e67153 * (locals.var_sp_s_x0__blk1472_dn6 - locals.var_xn_d__blk1407_dn6)), (assign52210_e67153 * (locals.var_sp_s_x0__blk1472_dn7 - locals.var_xn_d__blk1407_dn7)), (assign52210_e67153 * (locals.var_sp_s_x0__blk1472_dn8 - locals.var_xn_d__blk1407_dn8)), (assign52210_e67153 * (locals.var_sp_s_x0__blk1472_dn9 - locals.var_xn_d__blk1407_dn9)),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign52210_e67155;
        locals.var_sp_s_delta0__blk1458_dn4 = assign52210_e67155_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign52210_e67155_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign52210_e67155_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign52210_e67155_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign52210_e67155_d_n9;

        let (assign52220_e67171, assign52220_e67171_d_n4, assign52220_e67171_d_n6, assign52220_e67171_d_n7, assign52220_e67171_d_n8, assign52220_e67171_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 == 0.0)) && (locals.var_guard1511 != 0.0)) {
        let assign52220_e67169: f64 = (locals.var_delta_nd__blk1409 / locals.var_sp_s_delta0__blk1458);
        (assign52220_e67169, (((locals.var_delta_nd__blk1409_dn4 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn4)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_nd__blk1409_dn6 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn6)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_nd__blk1409_dn7 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn7)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_nd__blk1409_dn8 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn8)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_nd__blk1409_dn9 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn9)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)),)
    } else {
        (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9,)
    }
};
        locals.var_sp_s_delta1__blk1459 = assign52220_e67171;
        locals.var_sp_s_delta1__blk1459_dn4 = assign52220_e67171_d_n4;
        locals.var_sp_s_delta1__blk1459_dn6 = assign52220_e67171_d_n6;
        locals.var_sp_s_delta1__blk1459_dn7 = assign52220_e67171_d_n7;
        locals.var_sp_s_delta1__blk1459_dn8 = assign52220_e67171_d_n8;
        locals.var_sp_s_delta1__blk1459_dn9 = assign52220_e67171_d_n9;

        let (assign52230_e67214, assign52230_e67214_d_n4, assign52230_e67214_d_n6, assign52230_e67214_d_n7, assign52230_e67214_d_n8, assign52230_e67214_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 == 0.0)) && (locals.var_guard1511 == 0.0)) {
        let assign52230_e67188: f64 = (locals.var_xn_d__blk1407 - locals.var_sp_s_x0__blk1472);
        let assign52230_e67190: f64 = (assign52230_e67188 - 230.25850929940458);
        let assign52230_e67195: f64 = (locals.var_xn_d__blk1407 - locals.var_sp_s_x0__blk1472);
        let assign52230_e67197: f64 = (assign52230_e67195 - 230.25850929940458);
        let assign52230_e67201: f64 = (locals.var_xn_d__blk1407 - locals.var_sp_s_x0__blk1472);
        let assign52230_e67203: f64 = (assign52230_e67201 - 230.25850929940458);
        let assign52230_e67205: f64 = (assign52230_e67203 * 0.3333333333333333);
        let assign52230_e67206: f64 = (1.0 + assign52230_e67205);
        let assign52230_e67207: f64 = (assign52230_e67197 * assign52230_e67206);
        let assign52230_e67208: f64 = (0.5 * assign52230_e67207);
        let assign52230_e67209: f64 = (1.0 + assign52230_e67208);
        let assign52230_e67210: f64 = (assign52230_e67190 * assign52230_e67209);
        let assign52230_e67211: f64 = (1.0 + assign52230_e67210);
        let assign52230_e67212: f64 = (1e-100 / assign52230_e67211);
        (assign52230_e67212, (-((1e-100 * (((locals.var_xn_d__blk1407_dn4 - locals.var_sp_s_x0__blk1472_dn4) * assign52230_e67209) + (assign52230_e67190 * (0.5 * (((locals.var_xn_d__blk1407_dn4 - locals.var_sp_s_x0__blk1472_dn4) * assign52230_e67206) + (assign52230_e67197 * ((locals.var_xn_d__blk1407_dn4 - locals.var_sp_s_x0__blk1472_dn4) * 0.3333333333333333))))))) / (assign52230_e67211 * assign52230_e67211))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn6 - locals.var_sp_s_x0__blk1472_dn6) * assign52230_e67209) + (assign52230_e67190 * (0.5 * (((locals.var_xn_d__blk1407_dn6 - locals.var_sp_s_x0__blk1472_dn6) * assign52230_e67206) + (assign52230_e67197 * ((locals.var_xn_d__blk1407_dn6 - locals.var_sp_s_x0__blk1472_dn6) * 0.3333333333333333))))))) / (assign52230_e67211 * assign52230_e67211))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn7 - locals.var_sp_s_x0__blk1472_dn7) * assign52230_e67209) + (assign52230_e67190 * (0.5 * (((locals.var_xn_d__blk1407_dn7 - locals.var_sp_s_x0__blk1472_dn7) * assign52230_e67206) + (assign52230_e67197 * ((locals.var_xn_d__blk1407_dn7 - locals.var_sp_s_x0__blk1472_dn7) * 0.3333333333333333))))))) / (assign52230_e67211 * assign52230_e67211))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn8 - locals.var_sp_s_x0__blk1472_dn8) * assign52230_e67209) + (assign52230_e67190 * (0.5 * (((locals.var_xn_d__blk1407_dn8 - locals.var_sp_s_x0__blk1472_dn8) * assign52230_e67206) + (assign52230_e67197 * ((locals.var_xn_d__blk1407_dn8 - locals.var_sp_s_x0__blk1472_dn8) * 0.3333333333333333))))))) / (assign52230_e67211 * assign52230_e67211))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn9 - locals.var_sp_s_x0__blk1472_dn9) * assign52230_e67209) + (assign52230_e67190 * (0.5 * (((locals.var_xn_d__blk1407_dn9 - locals.var_sp_s_x0__blk1472_dn9) * assign52230_e67206) + (assign52230_e67197 * ((locals.var_xn_d__blk1407_dn9 - locals.var_sp_s_x0__blk1472_dn9) * 0.3333333333333333))))))) / (assign52230_e67211 * assign52230_e67211))),)
    } else {
        (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9,)
    }
};
        locals.var_sp_s_delta0__blk1458 = assign52230_e67214;
        locals.var_sp_s_delta0__blk1458_dn4 = assign52230_e67214_d_n4;
        locals.var_sp_s_delta0__blk1458_dn6 = assign52230_e67214_d_n6;
        locals.var_sp_s_delta0__blk1458_dn7 = assign52230_e67214_d_n7;
        locals.var_sp_s_delta0__blk1458_dn8 = assign52230_e67214_d_n8;
        locals.var_sp_s_delta0__blk1458_dn9 = assign52230_e67214_d_n9;

        let (assign52240_e67251, assign52240_e67251_d_n4, assign52240_e67251_d_n6, assign52240_e67251_d_n7, assign52240_e67251_d_n8, assign52240_e67251_d_n9,) = {
    if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 == 0.0)) && (locals.var_guard1511 == 0.0)) {
        let assign52240_e67231: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
        let assign52240_e67236: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
        let assign52240_e67240: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
        let assign52240_e67242: f64 = (assign52240_e67240 * 0.3333333333333333);
        let assign52240_e67243: f64 = (1.0 + assign52240_e67242);
        let assign52240_e67244: f64 = (assign52240_e67236 * assign52240_e67243);
        let assign52240_e67245: f64 = (0.5 * assign52240_e67244);
        let assign52240_e67246: f64 = (1.0 + assign52240_e67245);
        let assign52240_e67247: f64 = (assign52240_e67231 * assign52240_e67246);
        let assign52240_e67248: f64 = (1.0 + assign52240_e67247);
        let assign52240_e67249: f64 = (1e-100 / assign52240_e67248);
        (assign52240_e67249, (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn4 * assign52240_e67246) + (assign52240_e67231 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn4 * assign52240_e67243) + (assign52240_e67236 * (locals.var_sp_s_x0__blk1472_dn4 * 0.3333333333333333))))))) / (assign52240_e67248 * assign52240_e67248))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn6 * assign52240_e67246) + (assign52240_e67231 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn6 * assign52240_e67243) + (assign52240_e67236 * (locals.var_sp_s_x0__blk1472_dn6 * 0.3333333333333333))))))) / (assign52240_e67248 * assign52240_e67248))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn7 * assign52240_e67246) + (assign52240_e67231 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn7 * assign52240_e67243) + (assign52240_e67236 * (locals.var_sp_s_x0__blk1472_dn7 * 0.3333333333333333))))))) / (assign52240_e67248 * assign52240_e67248))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn8 * assign52240_e67246) + (assign52240_e67231 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn8 * assign52240_e67243) + (assign52240_e67236 * (locals.var_sp_s_x0__blk1472_dn8 * 0.3333333333333333))))))) / (assign52240_e67248 * assign52240_e67248))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn9 * assign52240_e67246) + (assign52240_e67231 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn9 * assign52240_e67243) + (assign52240_e67236 * (locals.var_sp_s_x0__blk1472_dn9 * 0.3333333333333333))))))) / (assign52240_e67248 * assign52240_e67248))),)
    } else {
        (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9,)
    }
};
        locals.var_sp_s_delta1__blk1459 = assign52240_e67251;
        locals.var_sp_s_delta1__blk1459_dn4 = assign52240_e67251_d_n4;
        locals.var_sp_s_delta1__blk1459_dn6 = assign52240_e67251_d_n6;
        locals.var_sp_s_delta1__blk1459_dn7 = assign52240_e67251_d_n7;
        locals.var_sp_s_delta1__blk1459_dn8 = assign52240_e67251_d_n8;
        locals.var_sp_s_delta1__blk1459_dn9 = assign52240_e67251_d_n9;

        let (assign52250_e67266, assign52250_e67266_d_n4, assign52250_e67266_d_n6, assign52250_e67266_d_n7, assign52250_e67266_d_n8, assign52250_e67266_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52250_e67262: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472);
        let assign52250_e67263: f64 = (2.0 + assign52250_e67262);
        let assign52250_e67264: f64 = (1.0 / assign52250_e67263);
        (assign52250_e67264, (-(((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn4)) / (assign52250_e67263 * assign52250_e67263))), (-(((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn6)) / (assign52250_e67263 * assign52250_e67263))), (-(((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn7)) / (assign52250_e67263 * assign52250_e67263))), (-(((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn8)) / (assign52250_e67263 * assign52250_e67263))), (-(((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn9)) / (assign52250_e67263 * assign52250_e67263))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign52250_e67266;
        locals.var_sp_s_temp__blk1448_dn4 = assign52250_e67266_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign52250_e67266_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign52250_e67266_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign52250_e67266_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign52250_e67266_d_n9;

        let (assign52260_e67279, assign52260_e67279_d_n4, assign52260_e67279_d_n6, assign52260_e67279_d_n7, assign52260_e67279_d_n8, assign52260_e67279_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52260_e67275: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472);
        let assign52260_e67277: f64 = (assign52260_e67275 * locals.var_sp_s_temp__blk1448);
        (assign52260_e67277, ((((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn4)) * locals.var_sp_s_temp__blk1448) + (assign52260_e67275 * locals.var_sp_s_temp__blk1448_dn4)), ((((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn6)) * locals.var_sp_s_temp__blk1448) + (assign52260_e67275 * locals.var_sp_s_temp__blk1448_dn6)), ((((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn7)) * locals.var_sp_s_temp__blk1448) + (assign52260_e67275 * locals.var_sp_s_temp__blk1448_dn7)), ((((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn8)) * locals.var_sp_s_temp__blk1448) + (assign52260_e67275 * locals.var_sp_s_temp__blk1448_dn8)), ((((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn9)) * locals.var_sp_s_temp__blk1448) + (assign52260_e67275 * locals.var_sp_s_temp__blk1448_dn9)),)
    } else {
        (locals.var_sp_s_xi0__blk1460, locals.var_sp_s_xi0__blk1460_dn4, locals.var_sp_s_xi0__blk1460_dn6, locals.var_sp_s_xi0__blk1460_dn7, locals.var_sp_s_xi0__blk1460_dn8, locals.var_sp_s_xi0__blk1460_dn9,)
    }
};
        locals.var_sp_s_xi0__blk1460 = assign52260_e67279;
        locals.var_sp_s_xi0__blk1460_dn4 = assign52260_e67279_d_n4;
        locals.var_sp_s_xi0__blk1460_dn6 = assign52260_e67279_d_n6;
        locals.var_sp_s_xi0__blk1460_dn7 = assign52260_e67279_d_n7;
        locals.var_sp_s_xi0__blk1460_dn8 = assign52260_e67279_d_n8;
        locals.var_sp_s_xi0__blk1460_dn9 = assign52260_e67279_d_n9;

        let (assign52270_e67294, assign52270_e67294_d_n4, assign52270_e67294_d_n6, assign52270_e67294_d_n7, assign52270_e67294_d_n8, assign52270_e67294_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52270_e67289: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448);
        let assign52270_e67291: f64 = (assign52270_e67289 * locals.var_sp_s_temp__blk1448);
        let assign52270_e67292: f64 = (4.0 * assign52270_e67291);
        (assign52270_e67292, (4.0 * ((((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign52270_e67289 * locals.var_sp_s_temp__blk1448_dn4))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign52270_e67289 * locals.var_sp_s_temp__blk1448_dn6))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign52270_e67289 * locals.var_sp_s_temp__blk1448_dn7))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign52270_e67289 * locals.var_sp_s_temp__blk1448_dn8))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign52270_e67289 * locals.var_sp_s_temp__blk1448_dn9))),)
    } else {
        (locals.var_sp_s_xi1__blk1461, locals.var_sp_s_xi1__blk1461_dn4, locals.var_sp_s_xi1__blk1461_dn6, locals.var_sp_s_xi1__blk1461_dn7, locals.var_sp_s_xi1__blk1461_dn8, locals.var_sp_s_xi1__blk1461_dn9,)
    }
};
        locals.var_sp_s_xi1__blk1461 = assign52270_e67294;
        locals.var_sp_s_xi1__blk1461_dn4 = assign52270_e67294_d_n4;
        locals.var_sp_s_xi1__blk1461_dn6 = assign52270_e67294_d_n6;
        locals.var_sp_s_xi1__blk1461_dn7 = assign52270_e67294_d_n7;
        locals.var_sp_s_xi1__blk1461_dn8 = assign52270_e67294_d_n8;
        locals.var_sp_s_xi1__blk1461_dn9 = assign52270_e67294_d_n9;

        let (assign52280_e67313, assign52280_e67313_d_n4, assign52280_e67313_d_n6, assign52280_e67313_d_n7, assign52280_e67313_d_n8, assign52280_e67313_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52280_e67303: f64 = (8.0 * locals.var_sp_s_temp__blk1448);
        let assign52280_e67306: f64 = (12.0 * locals.var_sp_s_xi0__blk1460);
        let assign52280_e67307: f64 = (assign52280_e67303 - assign52280_e67306);
        let assign52280_e67309: f64 = (assign52280_e67307 * locals.var_sp_s_temp__blk1448);
        let assign52280_e67311: f64 = (assign52280_e67309 * locals.var_sp_s_temp__blk1448);
        (assign52280_e67311, ((((((8.0 * locals.var_sp_s_temp__blk1448_dn4) - (12.0 * locals.var_sp_s_xi0__blk1460_dn4)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67307 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67309 * locals.var_sp_s_temp__blk1448_dn4)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn6) - (12.0 * locals.var_sp_s_xi0__blk1460_dn6)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67307 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67309 * locals.var_sp_s_temp__blk1448_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn7) - (12.0 * locals.var_sp_s_xi0__blk1460_dn7)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67307 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67309 * locals.var_sp_s_temp__blk1448_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn8) - (12.0 * locals.var_sp_s_xi0__blk1460_dn8)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67307 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67309 * locals.var_sp_s_temp__blk1448_dn8)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn9) - (12.0 * locals.var_sp_s_xi0__blk1460_dn9)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67307 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67309 * locals.var_sp_s_temp__blk1448_dn9)),)
    } else {
        (locals.var_sp_s_xi2__blk1462, locals.var_sp_s_xi2__blk1462_dn4, locals.var_sp_s_xi2__blk1462_dn6, locals.var_sp_s_xi2__blk1462_dn7, locals.var_sp_s_xi2__blk1462_dn8, locals.var_sp_s_xi2__blk1462_dn9,)
    }
};
        locals.var_sp_s_xi2__blk1462 = assign52280_e67313;
        locals.var_sp_s_xi2__blk1462_dn4 = assign52280_e67313_d_n4;
        locals.var_sp_s_xi2__blk1462_dn6 = assign52280_e67313_d_n6;
        locals.var_sp_s_xi2__blk1462_dn7 = assign52280_e67313_d_n7;
        locals.var_sp_s_xi2__blk1462_dn8 = assign52280_e67313_d_n8;
        locals.var_sp_s_xi2__blk1462_dn9 = assign52280_e67313_d_n9;

    }

    pub(super) fn stamp_transient_block_47(
        locals: &mut StampLocals,
    ) {
        let (assign52290_e67324, assign52290_e67324_d_n4, assign52290_e67324_d_n6, assign52290_e67324_d_n7, assign52290_e67324_d_n8, assign52290_e67324_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52290_e67322: f64 = (locals.var_xg__blk1343 - locals.var_sp_s_x0__blk1472);
        (assign52290_e67322, (locals.var_xg__blk1343_dn4 - locals.var_sp_s_x0__blk1472_dn4), (locals.var_xg__blk1343_dn6 - locals.var_sp_s_x0__blk1472_dn6), (locals.var_xg__blk1343_dn7 - locals.var_sp_s_x0__blk1472_dn7), (locals.var_xg__blk1343_dn8 - locals.var_sp_s_x0__blk1472_dn8), (locals.var_xg__blk1343_dn9 - locals.var_sp_s_x0__blk1472_dn9),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign52290_e67324;
        locals.var_sp_s_temp__blk1448_dn4 = assign52290_e67324_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign52290_e67324_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign52290_e67324_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign52290_e67324_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign52290_e67324_d_n9;

        let (assign52300_e67349, assign52300_e67349_d_n4, assign52300_e67349_d_n6, assign52300_e67349_d_n7, assign52300_e67349_d_n8, assign52300_e67349_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52300_e67333: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
        let assign52300_e67337: f64 = (1.0 - locals.var_sp_s_delta1__blk1459);
        let assign52300_e67339: f64 = (assign52300_e67337 + locals.var_sp_s_delta0__blk1458);
        let assign52300_e67343: f64 = (1.0 + locals.var_sp_s_xi1__blk1461);
        let assign52300_e67344: f64 = (locals.var_delta_nd__blk1409 * assign52300_e67343);
        let assign52300_e67345: f64 = (assign52300_e67339 - assign52300_e67344);
        let assign52300_e67346: f64 = (locals.var_gf2__blk1325 * assign52300_e67345);
        let assign52300_e67347: f64 = (assign52300_e67333 + assign52300_e67346);
        (assign52300_e67347, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) + ((locals.var_gf2__blk1325_dn4 * assign52300_e67345) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn4) + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_nd__blk1409_dn4 * assign52300_e67343) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn4)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) + ((locals.var_gf2__blk1325_dn6 * assign52300_e67345) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn6) + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_nd__blk1409_dn6 * assign52300_e67343) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) + ((locals.var_gf2__blk1325_dn7 * assign52300_e67345) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn7) + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_nd__blk1409_dn7 * assign52300_e67343) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) + ((locals.var_gf2__blk1325_dn8 * assign52300_e67345) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn8) + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_nd__blk1409_dn8 * assign52300_e67343) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn8)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) + ((locals.var_gf2__blk1325_dn9 * assign52300_e67345) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn9) + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_nd__blk1409_dn9 * assign52300_e67343) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn9)))))),)
    } else {
        (locals.var_sp_s_pc__blk1463, locals.var_sp_s_pc__blk1463_dn4, locals.var_sp_s_pc__blk1463_dn6, locals.var_sp_s_pc__blk1463_dn7, locals.var_sp_s_pc__blk1463_dn8, locals.var_sp_s_pc__blk1463_dn9,)
    }
};
        locals.var_sp_s_pc__blk1463 = assign52300_e67349;
        locals.var_sp_s_pc__blk1463_dn4 = assign52300_e67349_d_n4;
        locals.var_sp_s_pc__blk1463_dn6 = assign52300_e67349_d_n6;
        locals.var_sp_s_pc__blk1463_dn7 = assign52300_e67349_d_n7;
        locals.var_sp_s_pc__blk1463_dn8 = assign52300_e67349_d_n8;
        locals.var_sp_s_pc__blk1463_dn9 = assign52300_e67349_d_n9;

        let (assign52310_e67378, assign52310_e67378_d_n4, assign52310_e67378_d_n6, assign52310_e67378_d_n7, assign52310_e67378_d_n8, assign52310_e67378_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52310_e67358: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
        let assign52310_e67362: f64 = (locals.var_sp_s_delta1__blk1459 + locals.var_sp_s_x0__blk1472);
        let assign52310_e67364: f64 = (assign52310_e67362 - 1.0);
        let assign52310_e67366: f64 = (assign52310_e67364 + locals.var_sp_s_delta0__blk1458);
        let assign52310_e67370: f64 = (locals.var_sp_s_x0__blk1472 + 1.0);
        let assign52310_e67372: f64 = (assign52310_e67370 + locals.var_sp_s_xi0__blk1460);
        let assign52310_e67373: f64 = (locals.var_delta_nd__blk1409 * assign52310_e67372);
        let assign52310_e67374: f64 = (assign52310_e67366 - assign52310_e67373);
        let assign52310_e67375: f64 = (locals.var_gf2__blk1325 * assign52310_e67374);
        let assign52310_e67376: f64 = (assign52310_e67358 - assign52310_e67375);
        (assign52310_e67376, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) - ((locals.var_gf2__blk1325_dn4 * assign52310_e67374) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn4 + locals.var_sp_s_x0__blk1472_dn4) + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_nd__blk1409_dn4 * assign52310_e67372) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_x0__blk1472_dn4 + locals.var_sp_s_xi0__blk1460_dn4))))))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) - ((locals.var_gf2__blk1325_dn6 * assign52310_e67374) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn6 + locals.var_sp_s_x0__blk1472_dn6) + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_nd__blk1409_dn6 * assign52310_e67372) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_x0__blk1472_dn6 + locals.var_sp_s_xi0__blk1460_dn6))))))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) - ((locals.var_gf2__blk1325_dn7 * assign52310_e67374) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn7 + locals.var_sp_s_x0__blk1472_dn7) + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_nd__blk1409_dn7 * assign52310_e67372) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_x0__blk1472_dn7 + locals.var_sp_s_xi0__blk1460_dn7))))))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) - ((locals.var_gf2__blk1325_dn8 * assign52310_e67374) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn8 + locals.var_sp_s_x0__blk1472_dn8) + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_nd__blk1409_dn8 * assign52310_e67372) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_x0__blk1472_dn8 + locals.var_sp_s_xi0__blk1460_dn8))))))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) - ((locals.var_gf2__blk1325_dn9 * assign52310_e67374) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn9 + locals.var_sp_s_x0__blk1472_dn9) + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_nd__blk1409_dn9 * assign52310_e67372) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_x0__blk1472_dn9 + locals.var_sp_s_xi0__blk1460_dn9))))))),)
    } else {
        (locals.var_sp_s_qc__blk1464, locals.var_sp_s_qc__blk1464_dn4, locals.var_sp_s_qc__blk1464_dn6, locals.var_sp_s_qc__blk1464_dn7, locals.var_sp_s_qc__blk1464_dn8, locals.var_sp_s_qc__blk1464_dn9,)
    }
};
        locals.var_sp_s_qc__blk1464 = assign52310_e67378;
        locals.var_sp_s_qc__blk1464_dn4 = assign52310_e67378_d_n4;
        locals.var_sp_s_qc__blk1464_dn6 = assign52310_e67378_d_n6;
        locals.var_sp_s_qc__blk1464_dn7 = assign52310_e67378_d_n7;
        locals.var_sp_s_qc__blk1464_dn8 = assign52310_e67378_d_n8;
        locals.var_sp_s_qc__blk1464_dn9 = assign52310_e67378_d_n9;

        let (assign52320_e67397, assign52320_e67397_d_n4, assign52320_e67397_d_n6, assign52320_e67397_d_n7, assign52320_e67397_d_n8, assign52320_e67397_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52320_e67389: f64 = (locals.var_sp_s_delta1__blk1459 + locals.var_sp_s_delta0__blk1458);
        let assign52320_e67392: f64 = (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462);
        let assign52320_e67393: f64 = (assign52320_e67389 - assign52320_e67392);
        let assign52320_e67394: f64 = (locals.var_gf2__blk1325 * assign52320_e67393);
        let assign52320_e67395: f64 = (2.0 - assign52320_e67394);
        (assign52320_e67395, (-((locals.var_gf2__blk1325_dn4 * assign52320_e67393) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn4 + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_nd__blk1409_dn4 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn4)))))), (-((locals.var_gf2__blk1325_dn6 * assign52320_e67393) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn6 + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_nd__blk1409_dn6 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn6)))))), (-((locals.var_gf2__blk1325_dn7 * assign52320_e67393) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn7 + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_nd__blk1409_dn7 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn7)))))), (-((locals.var_gf2__blk1325_dn8 * assign52320_e67393) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn8 + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_nd__blk1409_dn8 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn8)))))), (-((locals.var_gf2__blk1325_dn9 * assign52320_e67393) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn9 + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_nd__blk1409_dn9 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn9)))))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign52320_e67397;
        locals.var_sp_s_temp__blk1448_dn4 = assign52320_e67397_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign52320_e67397_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign52320_e67397_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign52320_e67397_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign52320_e67397_d_n9;

        let (assign52330_e67414, assign52330_e67414_d_n4, assign52330_e67414_d_n6, assign52330_e67414_d_n7, assign52330_e67414_d_n8, assign52330_e67414_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52330_e67406: f64 = (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463);
        let assign52330_e67410: f64 = (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448);
        let assign52330_e67411: f64 = (2.0 * assign52330_e67410);
        let assign52330_e67412: f64 = (assign52330_e67406 - assign52330_e67411);
        (assign52330_e67412, (((locals.var_sp_s_pc__blk1463_dn4 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn4)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn4)))), (((locals.var_sp_s_pc__blk1463_dn6 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn6)))), (((locals.var_sp_s_pc__blk1463_dn7 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn7)))), (((locals.var_sp_s_pc__blk1463_dn8 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn8)))), (((locals.var_sp_s_pc__blk1463_dn9 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn9)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn9)))),)
    } else {
        (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9,)
    }
};
        locals.var_sp_s_temp__blk1448 = assign52330_e67414;
        locals.var_sp_s_temp__blk1448_dn4 = assign52330_e67414_d_n4;
        locals.var_sp_s_temp__blk1448_dn6 = assign52330_e67414_d_n6;
        locals.var_sp_s_temp__blk1448_dn7 = assign52330_e67414_d_n7;
        locals.var_sp_s_temp__blk1448_dn8 = assign52330_e67414_d_n8;
        locals.var_sp_s_temp__blk1448_dn9 = assign52330_e67414_d_n9;

        let (assign52340_e67432, assign52340_e67432_d_n4, assign52340_e67432_d_n6, assign52340_e67432_d_n7, assign52340_e67432_d_n8, assign52340_e67432_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
        let assign52340_e67426: f64 = (locals.var_sp_s_temp__blk1448).sqrt();
        let assign52340_e67427: f64 = (locals.var_sp_s_pc__blk1463 + assign52340_e67426);
        let assign52340_e67428: f64 = (locals.var_sp_s_qc__blk1464 / assign52340_e67427);
        let assign52340_e67429: f64 = (2.0 * assign52340_e67428);
        let assign52340_e67430: f64 = (locals.var_sp_s_x0__blk1472 + assign52340_e67429);
        (assign52340_e67430, (locals.var_sp_s_x0__blk1472_dn4 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn4 * assign52340_e67427) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn4 + (locals.var_sp_s_temp__blk1448_dn4 / (2.0 * assign52340_e67426))))) / (assign52340_e67427 * assign52340_e67427)))), (locals.var_sp_s_x0__blk1472_dn6 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn6 * assign52340_e67427) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn6 + (locals.var_sp_s_temp__blk1448_dn6 / (2.0 * assign52340_e67426))))) / (assign52340_e67427 * assign52340_e67427)))), (locals.var_sp_s_x0__blk1472_dn7 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn7 * assign52340_e67427) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn7 + (locals.var_sp_s_temp__blk1448_dn7 / (2.0 * assign52340_e67426))))) / (assign52340_e67427 * assign52340_e67427)))), (locals.var_sp_s_x0__blk1472_dn8 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn8 * assign52340_e67427) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn8 + (locals.var_sp_s_temp__blk1448_dn8 / (2.0 * assign52340_e67426))))) / (assign52340_e67427 * assign52340_e67427)))), (locals.var_sp_s_x0__blk1472_dn9 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn9 * assign52340_e67427) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn9 + (locals.var_sp_s_temp__blk1448_dn9 / (2.0 * assign52340_e67426))))) / (assign52340_e67427 * assign52340_e67427)))),)
    } else {
        (locals.var_x_d__blk1410, locals.var_x_d__blk1410_dn4, locals.var_x_d__blk1410_dn6, locals.var_x_d__blk1410_dn7, locals.var_x_d__blk1410_dn8, locals.var_x_d__blk1410_dn9,)
    }
};
        locals.var_x_d__blk1410 = assign52340_e67432;
        locals.var_x_d__blk1410_dn4 = assign52340_e67432_d_n4;
        locals.var_x_d__blk1410_dn6 = assign52340_e67432_d_n6;
        locals.var_x_d__blk1410_dn7 = assign52340_e67432_d_n7;
        locals.var_x_d__blk1410_dn8 = assign52340_e67432_d_n8;
        locals.var_x_d__blk1410_dn9 = assign52340_e67432_d_n9;

        let (assign52350_e67440, assign52350_e67440_d_n4, assign52350_e67440_d_n6, assign52350_e67440_d_n7, assign52350_e67440_d_n8, assign52350_e67440_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign52350_e67438: f64 = (locals.var_x_d__blk1410 - locals.var_x_s__blk1363);
        (assign52350_e67438, (locals.var_x_d__blk1410_dn4 - locals.var_x_s__blk1363_dn4), (locals.var_x_d__blk1410_dn6 - locals.var_x_s__blk1363_dn6), (locals.var_x_d__blk1410_dn7 - locals.var_x_s__blk1363_dn7), (locals.var_x_d__blk1410_dn8 - locals.var_x_s__blk1363_dn8), (locals.var_x_d__blk1410_dn9 - locals.var_x_s__blk1363_dn9),)
    } else {
        (locals.var_x_ds__blk1411, locals.var_x_ds__blk1411_dn4, locals.var_x_ds__blk1411_dn6, locals.var_x_ds__blk1411_dn7, locals.var_x_ds__blk1411_dn8, locals.var_x_ds__blk1411_dn9,)
    }
};
        locals.var_x_ds__blk1411 = assign52350_e67440;
        locals.var_x_ds__blk1411_dn4 = assign52350_e67440_d_n4;
        locals.var_x_ds__blk1411_dn6 = assign52350_e67440_d_n6;
        locals.var_x_ds__blk1411_dn7 = assign52350_e67440_d_n7;
        locals.var_x_ds__blk1411_dn8 = assign52350_e67440_d_n8;
        locals.var_x_ds__blk1411_dn9 = assign52350_e67440_d_n9;

        let assign52360_e67443: f64 = if locals.var_x_ds__blk1411 < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1512 = assign52360_e67443;

        let (assign52370_e67471, assign52370_e67471_d_n4, assign52370_e67471_d_n6, assign52370_e67471_d_n7, assign52370_e67471_d_n8, assign52370_e67471_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
        let assign52370_e67452: f64 = (locals.var_xg__blk1343 - locals.var_x_s__blk1363);
        let assign52370_e67453: f64 = (2.0 * assign52370_e67452);
        let assign52370_e67457: f64 = (1.0 - locals.var_es__blk1369);
        let assign52370_e67460: f64 = (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408);
        let assign52370_e67461: f64 = (assign52370_e67457 + assign52370_e67460);
        let assign52370_e67465: f64 = (1.0 + locals.var_xi1s__blk1366);
        let assign52370_e67466: f64 = (locals.var_delta_nd__blk1409 * assign52370_e67465);
        let assign52370_e67467: f64 = (assign52370_e67461 - assign52370_e67466);
        let assign52370_e67468: f64 = (locals.var_gf2__blk1325 * assign52370_e67467);
        let assign52370_e67469: f64 = (assign52370_e67453 + assign52370_e67468);
        (assign52370_e67469, ((2.0 * (locals.var_xg__blk1343_dn4 - locals.var_x_s__blk1363_dn4)) + ((locals.var_gf2__blk1325_dn4 * assign52370_e67467) + (locals.var_gf2__blk1325 * (((-locals.var_es__blk1369_dn4) + ((locals.var_delta_1s__blk1368_dn4 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn4))) - ((locals.var_delta_nd__blk1409_dn4 * assign52370_e67465) + (locals.var_delta_nd__blk1409 * locals.var_xi1s__blk1366_dn4)))))), ((2.0 * (locals.var_xg__blk1343_dn6 - locals.var_x_s__blk1363_dn6)) + ((locals.var_gf2__blk1325_dn6 * assign52370_e67467) + (locals.var_gf2__blk1325 * (((-locals.var_es__blk1369_dn6) + ((locals.var_delta_1s__blk1368_dn6 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn6))) - ((locals.var_delta_nd__blk1409_dn6 * assign52370_e67465) + (locals.var_delta_nd__blk1409 * locals.var_xi1s__blk1366_dn6)))))), ((2.0 * (locals.var_xg__blk1343_dn7 - locals.var_x_s__blk1363_dn7)) + ((locals.var_gf2__blk1325_dn7 * assign52370_e67467) + (locals.var_gf2__blk1325 * (((-locals.var_es__blk1369_dn7) + ((locals.var_delta_1s__blk1368_dn7 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn7))) - ((locals.var_delta_nd__blk1409_dn7 * assign52370_e67465) + (locals.var_delta_nd__blk1409 * locals.var_xi1s__blk1366_dn7)))))), ((2.0 * (locals.var_xg__blk1343_dn8 - locals.var_x_s__blk1363_dn8)) + ((locals.var_gf2__blk1325_dn8 * assign52370_e67467) + (locals.var_gf2__blk1325 * (((-locals.var_es__blk1369_dn8) + ((locals.var_delta_1s__blk1368_dn8 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn8))) - ((locals.var_delta_nd__blk1409_dn8 * assign52370_e67465) + (locals.var_delta_nd__blk1409 * locals.var_xi1s__blk1366_dn8)))))), ((2.0 * (locals.var_xg__blk1343_dn9 - locals.var_x_s__blk1363_dn9)) + ((locals.var_gf2__blk1325_dn9 * assign52370_e67467) + (locals.var_gf2__blk1325 * (((-locals.var_es__blk1369_dn9) + ((locals.var_delta_1s__blk1368_dn9 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn9))) - ((locals.var_delta_nd__blk1409_dn9 * assign52370_e67465) + (locals.var_delta_nd__blk1409 * locals.var_xi1s__blk1366_dn9)))))),)
    } else {
        (locals.var_pc__blk1412, locals.var_pc__blk1412_dn4, locals.var_pc__blk1412_dn6, locals.var_pc__blk1412_dn7, locals.var_pc__blk1412_dn8, locals.var_pc__blk1412_dn9,)
    }
};
        locals.var_pc__blk1412 = assign52370_e67471;
        locals.var_pc__blk1412_dn4 = assign52370_e67471_d_n4;
        locals.var_pc__blk1412_dn6 = assign52370_e67471_d_n6;
        locals.var_pc__blk1412_dn7 = assign52370_e67471_d_n7;
        locals.var_pc__blk1412_dn8 = assign52370_e67471_d_n8;
        locals.var_pc__blk1412_dn9 = assign52370_e67471_d_n9;

        let (assign52380_e67485, assign52380_e67485_d_n4, assign52380_e67485_d_n6, assign52380_e67485_d_n7, assign52380_e67485_d_n8, assign52380_e67485_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
        let assign52380_e67480: f64 = (1.0 - locals.var_k_ds__blk1408);
        let assign52380_e67481: f64 = (locals.var_gf2__blk1325 * assign52380_e67480);
        let assign52380_e67483: f64 = (assign52380_e67481 * locals.var_ds__blk1370);
        (assign52380_e67483, ((((locals.var_gf2__blk1325_dn4 * assign52380_e67480) + (locals.var_gf2__blk1325 * (-locals.var_k_ds__blk1408_dn4))) * locals.var_ds__blk1370) + (assign52380_e67481 * locals.var_ds__blk1370_dn4)), ((((locals.var_gf2__blk1325_dn6 * assign52380_e67480) + (locals.var_gf2__blk1325 * (-locals.var_k_ds__blk1408_dn6))) * locals.var_ds__blk1370) + (assign52380_e67481 * locals.var_ds__blk1370_dn6)), ((((locals.var_gf2__blk1325_dn7 * assign52380_e67480) + (locals.var_gf2__blk1325 * (-locals.var_k_ds__blk1408_dn7))) * locals.var_ds__blk1370) + (assign52380_e67481 * locals.var_ds__blk1370_dn7)), ((((locals.var_gf2__blk1325_dn8 * assign52380_e67480) + (locals.var_gf2__blk1325 * (-locals.var_k_ds__blk1408_dn8))) * locals.var_ds__blk1370) + (assign52380_e67481 * locals.var_ds__blk1370_dn8)), ((((locals.var_gf2__blk1325_dn9 * assign52380_e67480) + (locals.var_gf2__blk1325 * (-locals.var_k_ds__blk1408_dn9))) * locals.var_ds__blk1370) + (assign52380_e67481 * locals.var_ds__blk1370_dn9)),)
    } else {
        (locals.var_qc__blk1413, locals.var_qc__blk1413_dn4, locals.var_qc__blk1413_dn6, locals.var_qc__blk1413_dn7, locals.var_qc__blk1413_dn8, locals.var_qc__blk1413_dn9,)
    }
};
        locals.var_qc__blk1413 = assign52380_e67485;
        locals.var_qc__blk1413_dn4 = assign52380_e67485_d_n4;
        locals.var_qc__blk1413_dn6 = assign52380_e67485_d_n6;
        locals.var_qc__blk1413_dn7 = assign52380_e67485_d_n7;
        locals.var_qc__blk1413_dn8 = assign52380_e67485_d_n8;
        locals.var_qc__blk1413_dn9 = assign52380_e67485_d_n9;

        let (assign52390_e67505, assign52390_e67505_d_n4, assign52390_e67505_d_n6, assign52390_e67505_d_n7, assign52390_e67505_d_n8, assign52390_e67505_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
        let assign52390_e67496: f64 = (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408);
        let assign52390_e67497: f64 = (locals.var_es__blk1369 + assign52390_e67496);
        let assign52390_e67500: f64 = (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367);
        let assign52390_e67501: f64 = (assign52390_e67497 - assign52390_e67500);
        let assign52390_e67502: f64 = (locals.var_gf2__blk1325 * assign52390_e67501);
        let assign52390_e67503: f64 = (2.0 - assign52390_e67502);
        (assign52390_e67503, (-((locals.var_gf2__blk1325_dn4 * assign52390_e67501) + (locals.var_gf2__blk1325 * ((locals.var_es__blk1369_dn4 + ((locals.var_delta_1s__blk1368_dn4 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn4))) - ((locals.var_delta_nd__blk1409_dn4 * locals.var_xi2s__blk1367) + (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367_dn4)))))), (-((locals.var_gf2__blk1325_dn6 * assign52390_e67501) + (locals.var_gf2__blk1325 * ((locals.var_es__blk1369_dn6 + ((locals.var_delta_1s__blk1368_dn6 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn6))) - ((locals.var_delta_nd__blk1409_dn6 * locals.var_xi2s__blk1367) + (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367_dn6)))))), (-((locals.var_gf2__blk1325_dn7 * assign52390_e67501) + (locals.var_gf2__blk1325 * ((locals.var_es__blk1369_dn7 + ((locals.var_delta_1s__blk1368_dn7 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn7))) - ((locals.var_delta_nd__blk1409_dn7 * locals.var_xi2s__blk1367) + (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367_dn7)))))), (-((locals.var_gf2__blk1325_dn8 * assign52390_e67501) + (locals.var_gf2__blk1325 * ((locals.var_es__blk1369_dn8 + ((locals.var_delta_1s__blk1368_dn8 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn8))) - ((locals.var_delta_nd__blk1409_dn8 * locals.var_xi2s__blk1367) + (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367_dn8)))))), (-((locals.var_gf2__blk1325_dn9 * assign52390_e67501) + (locals.var_gf2__blk1325 * ((locals.var_es__blk1369_dn9 + ((locals.var_delta_1s__blk1368_dn9 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn9))) - ((locals.var_delta_nd__blk1409_dn9 * locals.var_xi2s__blk1367) + (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367_dn9)))))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign52390_e67505;
        locals.var_temp__blk949_dn4 = assign52390_e67505_d_n4;
        locals.var_temp__blk949_dn6 = assign52390_e67505_d_n6;
        locals.var_temp__blk949_dn7 = assign52390_e67505_d_n7;
        locals.var_temp__blk949_dn8 = assign52390_e67505_d_n8;
        locals.var_temp__blk949_dn9 = assign52390_e67505_d_n9;

        let (assign52400_e67521, assign52400_e67521_d_n4, assign52400_e67521_d_n6, assign52400_e67521_d_n7, assign52400_e67521_d_n8, assign52400_e67521_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
        let assign52400_e67513: f64 = (locals.var_pc__blk1412 * locals.var_pc__blk1412);
        let assign52400_e67517: f64 = (locals.var_temp__blk949 * locals.var_qc__blk1413);
        let assign52400_e67518: f64 = (2.0 * assign52400_e67517);
        let assign52400_e67519: f64 = (assign52400_e67513 - assign52400_e67518);
        (assign52400_e67519, (((locals.var_pc__blk1412_dn4 * locals.var_pc__blk1412) + (locals.var_pc__blk1412 * locals.var_pc__blk1412_dn4)) - (2.0 * ((locals.var_temp__blk949_dn4 * locals.var_qc__blk1413) + (locals.var_temp__blk949 * locals.var_qc__blk1413_dn4)))), (((locals.var_pc__blk1412_dn6 * locals.var_pc__blk1412) + (locals.var_pc__blk1412 * locals.var_pc__blk1412_dn6)) - (2.0 * ((locals.var_temp__blk949_dn6 * locals.var_qc__blk1413) + (locals.var_temp__blk949 * locals.var_qc__blk1413_dn6)))), (((locals.var_pc__blk1412_dn7 * locals.var_pc__blk1412) + (locals.var_pc__blk1412 * locals.var_pc__blk1412_dn7)) - (2.0 * ((locals.var_temp__blk949_dn7 * locals.var_qc__blk1413) + (locals.var_temp__blk949 * locals.var_qc__blk1413_dn7)))), (((locals.var_pc__blk1412_dn8 * locals.var_pc__blk1412) + (locals.var_pc__blk1412 * locals.var_pc__blk1412_dn8)) - (2.0 * ((locals.var_temp__blk949_dn8 * locals.var_qc__blk1413) + (locals.var_temp__blk949 * locals.var_qc__blk1413_dn8)))), (((locals.var_pc__blk1412_dn9 * locals.var_pc__blk1412) + (locals.var_pc__blk1412 * locals.var_pc__blk1412_dn9)) - (2.0 * ((locals.var_temp__blk949_dn9 * locals.var_qc__blk1413) + (locals.var_temp__blk949 * locals.var_qc__blk1413_dn9)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign52400_e67521;
        locals.var_temp__blk949_dn4 = assign52400_e67521_d_n4;
        locals.var_temp__blk949_dn6 = assign52400_e67521_d_n6;
        locals.var_temp__blk949_dn7 = assign52400_e67521_d_n7;
        locals.var_temp__blk949_dn8 = assign52400_e67521_d_n8;
        locals.var_temp__blk949_dn9 = assign52400_e67521_d_n9;

        let (assign52410_e67536, assign52410_e67536_d_n4, assign52410_e67536_d_n6, assign52410_e67536_d_n7, assign52410_e67536_d_n8, assign52410_e67536_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
        let assign52410_e67531: f64 = (locals.var_temp__blk949).sqrt();
        let assign52410_e67532: f64 = (locals.var_pc__blk1412 + assign52410_e67531);
        let assign52410_e67533: f64 = (locals.var_qc__blk1413 / assign52410_e67532);
        let assign52410_e67534: f64 = (2.0 * assign52410_e67533);
        (assign52410_e67534, (2.0 * (((locals.var_qc__blk1413_dn4 * assign52410_e67532) - (locals.var_qc__blk1413 * (locals.var_pc__blk1412_dn4 + (locals.var_temp__blk949_dn4 / (2.0 * assign52410_e67531))))) / (assign52410_e67532 * assign52410_e67532))), (2.0 * (((locals.var_qc__blk1413_dn6 * assign52410_e67532) - (locals.var_qc__blk1413 * (locals.var_pc__blk1412_dn6 + (locals.var_temp__blk949_dn6 / (2.0 * assign52410_e67531))))) / (assign52410_e67532 * assign52410_e67532))), (2.0 * (((locals.var_qc__blk1413_dn7 * assign52410_e67532) - (locals.var_qc__blk1413 * (locals.var_pc__blk1412_dn7 + (locals.var_temp__blk949_dn7 / (2.0 * assign52410_e67531))))) / (assign52410_e67532 * assign52410_e67532))), (2.0 * (((locals.var_qc__blk1413_dn8 * assign52410_e67532) - (locals.var_qc__blk1413 * (locals.var_pc__blk1412_dn8 + (locals.var_temp__blk949_dn8 / (2.0 * assign52410_e67531))))) / (assign52410_e67532 * assign52410_e67532))), (2.0 * (((locals.var_qc__blk1413_dn9 * assign52410_e67532) - (locals.var_qc__blk1413 * (locals.var_pc__blk1412_dn9 + (locals.var_temp__blk949_dn9 / (2.0 * assign52410_e67531))))) / (assign52410_e67532 * assign52410_e67532))),)
    } else {
        (locals.var_x_ds__blk1411, locals.var_x_ds__blk1411_dn4, locals.var_x_ds__blk1411_dn6, locals.var_x_ds__blk1411_dn7, locals.var_x_ds__blk1411_dn8, locals.var_x_ds__blk1411_dn9,)
    }
};
        locals.var_x_ds__blk1411 = assign52410_e67536;
        locals.var_x_ds__blk1411_dn4 = assign52410_e67536_d_n4;
        locals.var_x_ds__blk1411_dn6 = assign52410_e67536_d_n6;
        locals.var_x_ds__blk1411_dn7 = assign52410_e67536_d_n7;
        locals.var_x_ds__blk1411_dn8 = assign52410_e67536_d_n8;
        locals.var_x_ds__blk1411_dn9 = assign52410_e67536_d_n9;

        let (assign52420_e67546, assign52420_e67546_d_n4, assign52420_e67546_d_n6, assign52420_e67546_d_n7, assign52420_e67546_d_n8, assign52420_e67546_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
        let assign52420_e67544: f64 = (locals.var_x_s__blk1363 + locals.var_x_ds__blk1411);
        (assign52420_e67544, (locals.var_x_s__blk1363_dn4 + locals.var_x_ds__blk1411_dn4), (locals.var_x_s__blk1363_dn6 + locals.var_x_ds__blk1411_dn6), (locals.var_x_s__blk1363_dn7 + locals.var_x_ds__blk1411_dn7), (locals.var_x_s__blk1363_dn8 + locals.var_x_ds__blk1411_dn8), (locals.var_x_s__blk1363_dn9 + locals.var_x_ds__blk1411_dn9),)
    } else {
        (locals.var_x_d__blk1410, locals.var_x_d__blk1410_dn4, locals.var_x_d__blk1410_dn6, locals.var_x_d__blk1410_dn7, locals.var_x_d__blk1410_dn8, locals.var_x_d__blk1410_dn9,)
    }
};
        locals.var_x_d__blk1410 = assign52420_e67546;
        locals.var_x_d__blk1410_dn4 = assign52420_e67546_d_n4;
        locals.var_x_d__blk1410_dn6 = assign52420_e67546_d_n6;
        locals.var_x_d__blk1410_dn7 = assign52420_e67546_d_n7;
        locals.var_x_d__blk1410_dn8 = assign52420_e67546_d_n8;
        locals.var_x_d__blk1410_dn9 = assign52420_e67546_d_n9;

        let (assign52430_e67554, assign52430_e67554_d_n4, assign52430_e67554_d_n6, assign52430_e67554_d_n7, assign52430_e67554_d_n8, assign52430_e67554_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign52430_e67552: f64 = (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339);
        (assign52430_e67552, ((locals.var_x_ds__blk1411_dn4 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn4)), ((locals.var_x_ds__blk1411_dn6 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn6)), ((locals.var_x_ds__blk1411_dn7 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn7)), ((locals.var_x_ds__blk1411_dn8 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn8)), ((locals.var_x_ds__blk1411_dn9 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn9)),)
    } else {
        (locals.var_dps__blk1414, locals.var_dps__blk1414_dn4, locals.var_dps__blk1414_dn6, locals.var_dps__blk1414_dn7, locals.var_dps__blk1414_dn8, locals.var_dps__blk1414_dn9,)
    }
};
        locals.var_dps__blk1414 = assign52430_e67554;
        locals.var_dps__blk1414_dn4 = assign52430_e67554_d_n4;
        locals.var_dps__blk1414_dn6 = assign52430_e67554_d_n6;
        locals.var_dps__blk1414_dn7 = assign52430_e67554_d_n7;
        locals.var_dps__blk1414_dn8 = assign52430_e67554_d_n8;
        locals.var_dps__blk1414_dn9 = assign52430_e67554_d_n9;

        let (assign52440_e67568, assign52440_e67568_d_n4, assign52440_e67568_d_n6, assign52440_e67568_d_n7, assign52440_e67568_d_n8, assign52440_e67568_d_n9,) = {
    if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
        let assign52440_e67560: f64 = (locals.var_x_d__blk1410 * locals.var_x_d__blk1410);
        let assign52440_e67564: f64 = (locals.var_x_d__blk1410 * locals.var_x_d__blk1410);
        let assign52440_e67565: f64 = (2.0 + assign52440_e67564);
        let assign52440_e67566: f64 = (assign52440_e67560 / assign52440_e67565);
        (assign52440_e67566, (((((locals.var_x_d__blk1410_dn4 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn4)) * assign52440_e67565) - (assign52440_e67560 * ((locals.var_x_d__blk1410_dn4 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn4)))) / (assign52440_e67565 * assign52440_e67565)), (((((locals.var_x_d__blk1410_dn6 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn6)) * assign52440_e67565) - (assign52440_e67560 * ((locals.var_x_d__blk1410_dn6 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn6)))) / (assign52440_e67565 * assign52440_e67565)), (((((locals.var_x_d__blk1410_dn7 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn7)) * assign52440_e67565) - (assign52440_e67560 * ((locals.var_x_d__blk1410_dn7 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn7)))) / (assign52440_e67565 * assign52440_e67565)), (((((locals.var_x_d__blk1410_dn8 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn8)) * assign52440_e67565) - (assign52440_e67560 * ((locals.var_x_d__blk1410_dn8 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn8)))) / (assign52440_e67565 * assign52440_e67565)), (((((locals.var_x_d__blk1410_dn9 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn9)) * assign52440_e67565) - (assign52440_e67560 * ((locals.var_x_d__blk1410_dn9 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn9)))) / (assign52440_e67565 * assign52440_e67565)),)
    } else {
        (locals.var_xi0d__blk1415, locals.var_xi0d__blk1415_dn4, locals.var_xi0d__blk1415_dn6, locals.var_xi0d__blk1415_dn7, locals.var_xi0d__blk1415_dn8, locals.var_xi0d__blk1415_dn9,)
    }
};
        locals.var_xi0d__blk1415 = assign52440_e67568;
        locals.var_xi0d__blk1415_dn4 = assign52440_e67568_d_n4;
        locals.var_xi0d__blk1415_dn6 = assign52440_e67568_d_n6;
        locals.var_xi0d__blk1415_dn7 = assign52440_e67568_d_n7;
        locals.var_xi0d__blk1415_dn8 = assign52440_e67568_d_n8;
        locals.var_xi0d__blk1415_dn9 = assign52440_e67568_d_n9;

        let assign52450_e67571: f64 = if locals.var_x_d__blk1410 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1513 = assign52450_e67571;

        let (assign52460_e67581, assign52460_e67581_d_n4, assign52460_e67581_d_n6, assign52460_e67581_d_n7, assign52460_e67581_d_n8, assign52460_e67581_d_n9,) = {
    if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        let assign52460_e67578: f64 = (-locals.var_x_d__blk1410);
        let assign52460_e67579: f64 = (assign52460_e67578).exp();
        (assign52460_e67579, (assign52460_e67579 * (-locals.var_x_d__blk1410_dn4)), (assign52460_e67579 * (-locals.var_x_d__blk1410_dn6)), (assign52460_e67579 * (-locals.var_x_d__blk1410_dn7)), (assign52460_e67579 * (-locals.var_x_d__blk1410_dn8)), (assign52460_e67579 * (-locals.var_x_d__blk1410_dn9)),)
    } else {
        (locals.var_ed__blk1416, locals.var_ed__blk1416_dn4, locals.var_ed__blk1416_dn6, locals.var_ed__blk1416_dn7, locals.var_ed__blk1416_dn8, locals.var_ed__blk1416_dn9,)
    }
};
        locals.var_ed__blk1416 = assign52460_e67581;
        locals.var_ed__blk1416_dn4 = assign52460_e67581_d_n4;
        locals.var_ed__blk1416_dn6 = assign52460_e67581_d_n6;
        locals.var_ed__blk1416_dn7 = assign52460_e67581_d_n7;
        locals.var_ed__blk1416_dn8 = assign52460_e67581_d_n8;
        locals.var_ed__blk1416_dn9 = assign52460_e67581_d_n9;

        let assign52470_e67584: f64 = if locals.var_x_d__blk1410 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1514 = assign52470_e67584;

        let (assign52480_e67610, assign52480_e67610_d_n4, assign52480_e67610_d_n6, assign52480_e67610_d_n7, assign52480_e67610_d_n8, assign52480_e67610_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 != 0.0)) {
        let assign52480_e67595: f64 = (locals.var_x_d__blk1410 * locals.var_x_d__blk1410);
        let assign52480_e67602: f64 = (0.25 * locals.var_x_d__blk1410);
        let assign52480_e67603: f64 = (1.0 - assign52480_e67602);
        let assign52480_e67604: f64 = (locals.var_x_d__blk1410 * assign52480_e67603);
        let assign52480_e67605: f64 = (0.3333333333333333 * assign52480_e67604);
        let assign52480_e67606: f64 = (1.0 - assign52480_e67605);
        let assign52480_e67607: f64 = (assign52480_e67595 * assign52480_e67606);
        let assign52480_e67608: f64 = (0.5 * assign52480_e67607);
        (assign52480_e67608, (0.5 * ((((locals.var_x_d__blk1410_dn4 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn4)) * assign52480_e67606) + (assign52480_e67595 * (-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn4 * assign52480_e67603) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn4))))))))), (0.5 * ((((locals.var_x_d__blk1410_dn6 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn6)) * assign52480_e67606) + (assign52480_e67595 * (-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn6 * assign52480_e67603) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn6))))))))), (0.5 * ((((locals.var_x_d__blk1410_dn7 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn7)) * assign52480_e67606) + (assign52480_e67595 * (-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn7 * assign52480_e67603) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn7))))))))), (0.5 * ((((locals.var_x_d__blk1410_dn8 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn8)) * assign52480_e67606) + (assign52480_e67595 * (-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn8 * assign52480_e67603) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn8))))))))), (0.5 * ((((locals.var_x_d__blk1410_dn9 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn9)) * assign52480_e67606) + (assign52480_e67595 * (-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn9 * assign52480_e67603) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn9))))))))),)
    } else {
        (locals.var_pd__blk1417, locals.var_pd__blk1417_dn4, locals.var_pd__blk1417_dn6, locals.var_pd__blk1417_dn7, locals.var_pd__blk1417_dn8, locals.var_pd__blk1417_dn9,)
    }
};
        locals.var_pd__blk1417 = assign52480_e67610;
        locals.var_pd__blk1417_dn4 = assign52480_e67610_d_n4;
        locals.var_pd__blk1417_dn6 = assign52480_e67610_d_n6;
        locals.var_pd__blk1417_dn7 = assign52480_e67610_d_n7;
        locals.var_pd__blk1417_dn8 = assign52480_e67610_d_n8;
        locals.var_pd__blk1417_dn9 = assign52480_e67610_d_n9;

        let (assign52490_e67631, assign52490_e67631_d_n4, assign52490_e67631_d_n6, assign52490_e67631_d_n7, assign52490_e67631_d_n8, assign52490_e67631_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 != 0.0)) {
        let assign52490_e67624: f64 = (0.25 * locals.var_x_d__blk1410);
        let assign52490_e67625: f64 = (1.0 - assign52490_e67624);
        let assign52490_e67626: f64 = (locals.var_x_d__blk1410 * assign52490_e67625);
        let assign52490_e67627: f64 = (0.3333333333333333 * assign52490_e67626);
        let assign52490_e67628: f64 = (1.0 - assign52490_e67627);
        let assign52490_e67629: f64 = (assign52490_e67628).sqrt();
        (assign52490_e67629, ((-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn4 * assign52490_e67625) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn4)))))) / (2.0 * assign52490_e67629)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn6 * assign52490_e67625) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn6)))))) / (2.0 * assign52490_e67629)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn7 * assign52490_e67625) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn7)))))) / (2.0 * assign52490_e67629)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn8 * assign52490_e67625) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn8)))))) / (2.0 * assign52490_e67629)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn9 * assign52490_e67625) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn9)))))) / (2.0 * assign52490_e67629)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign52490_e67631;
        locals.var_temp__blk949_dn4 = assign52490_e67631_d_n4;
        locals.var_temp__blk949_dn6 = assign52490_e67631_d_n6;
        locals.var_temp__blk949_dn7 = assign52490_e67631_d_n7;
        locals.var_temp__blk949_dn8 = assign52490_e67631_d_n8;
        locals.var_temp__blk949_dn9 = assign52490_e67631_d_n9;

        let (assign52500_e67645, assign52500_e67645_d_n4, assign52500_e67645_d_n6, assign52500_e67645_d_n7, assign52500_e67645_d_n8, assign52500_e67645_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 != 0.0)) {
        let assign52500_e67642: f64 = (locals.var_x_d__blk1410 * locals.var_temp__blk949);
        let assign52500_e67643: f64 = (0.7071067811865475 * assign52500_e67642);
        (assign52500_e67643, (0.7071067811865475 * ((locals.var_x_d__blk1410_dn4 * locals.var_temp__blk949) + (locals.var_x_d__blk1410 * locals.var_temp__blk949_dn4))), (0.7071067811865475 * ((locals.var_x_d__blk1410_dn6 * locals.var_temp__blk949) + (locals.var_x_d__blk1410 * locals.var_temp__blk949_dn6))), (0.7071067811865475 * ((locals.var_x_d__blk1410_dn7 * locals.var_temp__blk949) + (locals.var_x_d__blk1410 * locals.var_temp__blk949_dn7))), (0.7071067811865475 * ((locals.var_x_d__blk1410_dn8 * locals.var_temp__blk949) + (locals.var_x_d__blk1410 * locals.var_temp__blk949_dn8))), (0.7071067811865475 * ((locals.var_x_d__blk1410_dn9 * locals.var_temp__blk949) + (locals.var_x_d__blk1410 * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_sqd__blk1418, locals.var_sqd__blk1418_dn4, locals.var_sqd__blk1418_dn6, locals.var_sqd__blk1418_dn7, locals.var_sqd__blk1418_dn8, locals.var_sqd__blk1418_dn9,)
    }
};
        locals.var_sqd__blk1418 = assign52500_e67645;
        locals.var_sqd__blk1418_dn4 = assign52500_e67645_d_n4;
        locals.var_sqd__blk1418_dn6 = assign52500_e67645_d_n6;
        locals.var_sqd__blk1418_dn7 = assign52500_e67645_d_n7;
        locals.var_sqd__blk1418_dn8 = assign52500_e67645_d_n8;
        locals.var_sqd__blk1418_dn9 = assign52500_e67645_d_n9;

        let (assign52510_e67669, assign52510_e67669_d_n4, assign52510_e67669_d_n6, assign52510_e67669_d_n7, assign52510_e67669_d_n8, assign52510_e67669_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 != 0.0)) {
        let assign52510_e67655: f64 = (0.16666666666666666 * locals.var_delta_nd__blk1409);
        let assign52510_e67657: f64 = (assign52510_e67655 * locals.var_x_d__blk1410);
        let assign52510_e67659: f64 = (assign52510_e67657 * locals.var_x_d__blk1410);
        let assign52510_e67661: f64 = (assign52510_e67659 * locals.var_x_d__blk1410);
        let assign52510_e67665: f64 = (1.75 * locals.var_x_d__blk1410);
        let assign52510_e67666: f64 = (1.0 + assign52510_e67665);
        let assign52510_e67667: f64 = (assign52510_e67661 * assign52510_e67666);
        (assign52510_e67667, (((((((((0.16666666666666666 * locals.var_delta_nd__blk1409_dn4) * locals.var_x_d__blk1410) + (assign52510_e67655 * locals.var_x_d__blk1410_dn4)) * locals.var_x_d__blk1410) + (assign52510_e67657 * locals.var_x_d__blk1410_dn4)) * locals.var_x_d__blk1410) + (assign52510_e67659 * locals.var_x_d__blk1410_dn4)) * assign52510_e67666) + (assign52510_e67661 * (1.75 * locals.var_x_d__blk1410_dn4))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1409_dn6) * locals.var_x_d__blk1410) + (assign52510_e67655 * locals.var_x_d__blk1410_dn6)) * locals.var_x_d__blk1410) + (assign52510_e67657 * locals.var_x_d__blk1410_dn6)) * locals.var_x_d__blk1410) + (assign52510_e67659 * locals.var_x_d__blk1410_dn6)) * assign52510_e67666) + (assign52510_e67661 * (1.75 * locals.var_x_d__blk1410_dn6))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1409_dn7) * locals.var_x_d__blk1410) + (assign52510_e67655 * locals.var_x_d__blk1410_dn7)) * locals.var_x_d__blk1410) + (assign52510_e67657 * locals.var_x_d__blk1410_dn7)) * locals.var_x_d__blk1410) + (assign52510_e67659 * locals.var_x_d__blk1410_dn7)) * assign52510_e67666) + (assign52510_e67661 * (1.75 * locals.var_x_d__blk1410_dn7))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1409_dn8) * locals.var_x_d__blk1410) + (assign52510_e67655 * locals.var_x_d__blk1410_dn8)) * locals.var_x_d__blk1410) + (assign52510_e67657 * locals.var_x_d__blk1410_dn8)) * locals.var_x_d__blk1410) + (assign52510_e67659 * locals.var_x_d__blk1410_dn8)) * assign52510_e67666) + (assign52510_e67661 * (1.75 * locals.var_x_d__blk1410_dn8))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1409_dn9) * locals.var_x_d__blk1410) + (assign52510_e67655 * locals.var_x_d__blk1410_dn9)) * locals.var_x_d__blk1410) + (assign52510_e67657 * locals.var_x_d__blk1410_dn9)) * locals.var_x_d__blk1410) + (assign52510_e67659 * locals.var_x_d__blk1410_dn9)) * assign52510_e67666) + (assign52510_e67661 * (1.75 * locals.var_x_d__blk1410_dn9))),)
    } else {
        (locals.var_dd__blk1419, locals.var_dd__blk1419_dn4, locals.var_dd__blk1419_dn6, locals.var_dd__blk1419_dn7, locals.var_dd__blk1419_dn8, locals.var_dd__blk1419_dn9,)
    }
};
        locals.var_dd__blk1419 = assign52510_e67669;
        locals.var_dd__blk1419_dn4 = assign52510_e67669_d_n4;
        locals.var_dd__blk1419_dn6 = assign52510_e67669_d_n6;
        locals.var_dd__blk1419_dn7 = assign52510_e67669_d_n7;
        locals.var_dd__blk1419_dn8 = assign52510_e67669_d_n8;
        locals.var_dd__blk1419_dn9 = assign52510_e67669_d_n9;

        let (assign52520_e67684, assign52520_e67684_d_n4, assign52520_e67684_d_n6, assign52520_e67684_d_n7, assign52520_e67684_d_n8, assign52520_e67684_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 == 0.0)) {
        let assign52520_e67680: f64 = (locals.var_x_d__blk1410 - 1.0);
        let assign52520_e67682: f64 = (assign52520_e67680 + locals.var_ed__blk1416);
        (assign52520_e67682, (locals.var_x_d__blk1410_dn4 + locals.var_ed__blk1416_dn4), (locals.var_x_d__blk1410_dn6 + locals.var_ed__blk1416_dn6), (locals.var_x_d__blk1410_dn7 + locals.var_ed__blk1416_dn7), (locals.var_x_d__blk1410_dn8 + locals.var_ed__blk1416_dn8), (locals.var_x_d__blk1410_dn9 + locals.var_ed__blk1416_dn9),)
    } else {
        (locals.var_pd__blk1417, locals.var_pd__blk1417_dn4, locals.var_pd__blk1417_dn6, locals.var_pd__blk1417_dn7, locals.var_pd__blk1417_dn8, locals.var_pd__blk1417_dn9,)
    }
};
        locals.var_pd__blk1417 = assign52520_e67684;
        locals.var_pd__blk1417_dn4 = assign52520_e67684_d_n4;
        locals.var_pd__blk1417_dn6 = assign52520_e67684_d_n6;
        locals.var_pd__blk1417_dn7 = assign52520_e67684_d_n7;
        locals.var_pd__blk1417_dn8 = assign52520_e67684_d_n8;
        locals.var_pd__blk1417_dn9 = assign52520_e67684_d_n9;

        let (assign52530_e67696, assign52530_e67696_d_n4, assign52530_e67696_d_n6, assign52530_e67696_d_n7, assign52530_e67696_d_n8, assign52530_e67696_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 == 0.0)) {
        let assign52530_e67694: f64 = (locals.var_pd__blk1417).sqrt();
        (assign52530_e67694, (locals.var_pd__blk1417_dn4 / (2.0 * assign52530_e67694)), (locals.var_pd__blk1417_dn6 / (2.0 * assign52530_e67694)), (locals.var_pd__blk1417_dn7 / (2.0 * assign52530_e67694)), (locals.var_pd__blk1417_dn8 / (2.0 * assign52530_e67694)), (locals.var_pd__blk1417_dn9 / (2.0 * assign52530_e67694)),)
    } else {
        (locals.var_sqd__blk1418, locals.var_sqd__blk1418_dn4, locals.var_sqd__blk1418_dn6, locals.var_sqd__blk1418_dn7, locals.var_sqd__blk1418_dn8, locals.var_sqd__blk1418_dn9,)
    }
};
        locals.var_sqd__blk1418 = assign52530_e67696;
        locals.var_sqd__blk1418_dn4 = assign52530_e67696_d_n4;
        locals.var_sqd__blk1418_dn6 = assign52530_e67696_d_n6;
        locals.var_sqd__blk1418_dn7 = assign52530_e67696_d_n7;
        locals.var_sqd__blk1418_dn8 = assign52530_e67696_d_n8;
        locals.var_sqd__blk1418_dn9 = assign52530_e67696_d_n9;

        let (assign52540_e67717, assign52540_e67717_d_n4, assign52540_e67717_d_n6, assign52540_e67717_d_n7, assign52540_e67717_d_n8, assign52540_e67717_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 == 0.0)) {
        let assign52540_e67708: f64 = (1.0 / locals.var_ed__blk1416);
        let assign52540_e67710: f64 = (assign52540_e67708 - locals.var_x_d__blk1410);
        let assign52540_e67712: f64 = (assign52540_e67710 - 1.0);
        let assign52540_e67714: f64 = (assign52540_e67712 - locals.var_xi0d__blk1415);
        let assign52540_e67715: f64 = (locals.var_delta_nd__blk1409 * assign52540_e67714);
        (assign52540_e67715, ((locals.var_delta_nd__blk1409_dn4 * assign52540_e67714) + (locals.var_delta_nd__blk1409 * (((-(locals.var_ed__blk1416_dn4 / (locals.var_ed__blk1416 * locals.var_ed__blk1416))) - locals.var_x_d__blk1410_dn4) - locals.var_xi0d__blk1415_dn4))), ((locals.var_delta_nd__blk1409_dn6 * assign52540_e67714) + (locals.var_delta_nd__blk1409 * (((-(locals.var_ed__blk1416_dn6 / (locals.var_ed__blk1416 * locals.var_ed__blk1416))) - locals.var_x_d__blk1410_dn6) - locals.var_xi0d__blk1415_dn6))), ((locals.var_delta_nd__blk1409_dn7 * assign52540_e67714) + (locals.var_delta_nd__blk1409 * (((-(locals.var_ed__blk1416_dn7 / (locals.var_ed__blk1416 * locals.var_ed__blk1416))) - locals.var_x_d__blk1410_dn7) - locals.var_xi0d__blk1415_dn7))), ((locals.var_delta_nd__blk1409_dn8 * assign52540_e67714) + (locals.var_delta_nd__blk1409 * (((-(locals.var_ed__blk1416_dn8 / (locals.var_ed__blk1416 * locals.var_ed__blk1416))) - locals.var_x_d__blk1410_dn8) - locals.var_xi0d__blk1415_dn8))), ((locals.var_delta_nd__blk1409_dn9 * assign52540_e67714) + (locals.var_delta_nd__blk1409 * (((-(locals.var_ed__blk1416_dn9 / (locals.var_ed__blk1416 * locals.var_ed__blk1416))) - locals.var_x_d__blk1410_dn9) - locals.var_xi0d__blk1415_dn9))),)
    } else {
        (locals.var_dd__blk1419, locals.var_dd__blk1419_dn4, locals.var_dd__blk1419_dn6, locals.var_dd__blk1419_dn7, locals.var_dd__blk1419_dn8, locals.var_dd__blk1419_dn9,)
    }
};
        locals.var_dd__blk1419 = assign52540_e67717;
        locals.var_dd__blk1419_dn4 = assign52540_e67717_d_n4;
        locals.var_dd__blk1419_dn6 = assign52540_e67717_d_n6;
        locals.var_dd__blk1419_dn7 = assign52540_e67717_d_n7;
        locals.var_dd__blk1419_dn8 = assign52540_e67717_d_n8;
        locals.var_dd__blk1419_dn9 = assign52540_e67717_d_n9;

        let assign52550_e67721: f64 = (locals.var_xn_d__blk1407 - 230.25850929940458);
        let assign52550_e67722: f64 = if locals.var_x_d__blk1410 > assign52550_e67721 { 1.0 } else { 0.0 };
        locals.var_guard1515 = assign52550_e67722;

        let (assign52560_e67736, assign52560_e67736_d_n4, assign52560_e67736_d_n6, assign52560_e67736_d_n7, assign52560_e67736_d_n8, assign52560_e67736_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign52560_e67733: f64 = (locals.var_x_d__blk1410 - locals.var_xn_d__blk1407);
        let assign52560_e67734: f64 = (assign52560_e67733).exp();
        (assign52560_e67734, (assign52560_e67734 * (locals.var_x_d__blk1410_dn4 - locals.var_xn_d__blk1407_dn4)), (assign52560_e67734 * (locals.var_x_d__blk1410_dn6 - locals.var_xn_d__blk1407_dn6)), (assign52560_e67734 * (locals.var_x_d__blk1410_dn7 - locals.var_xn_d__blk1407_dn7)), (assign52560_e67734 * (locals.var_x_d__blk1410_dn8 - locals.var_xn_d__blk1407_dn8)), (assign52560_e67734 * (locals.var_x_d__blk1410_dn9 - locals.var_xn_d__blk1407_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign52560_e67736;
        locals.var_temp__blk949_dn4 = assign52560_e67736_d_n4;
        locals.var_temp__blk949_dn6 = assign52560_e67736_d_n6;
        locals.var_temp__blk949_dn7 = assign52560_e67736_d_n7;
        locals.var_temp__blk949_dn8 = assign52560_e67736_d_n8;
        locals.var_temp__blk949_dn9 = assign52560_e67736_d_n9;

        let (assign52570_e67749, assign52570_e67749_d_n4, assign52570_e67749_d_n6, assign52570_e67749_d_n7, assign52570_e67749_d_n8, assign52570_e67749_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign52570_e67747: f64 = (locals.var_delta_nd__blk1409 / locals.var_temp__blk949);
        (assign52570_e67747, (((locals.var_delta_nd__blk1409_dn4 * locals.var_temp__blk949) - (locals.var_delta_nd__blk1409 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd__blk1409_dn6 * locals.var_temp__blk949) - (locals.var_delta_nd__blk1409 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd__blk1409_dn7 * locals.var_temp__blk949) - (locals.var_delta_nd__blk1409 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd__blk1409_dn8 * locals.var_temp__blk949) - (locals.var_delta_nd__blk1409 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd__blk1409_dn9 * locals.var_temp__blk949) - (locals.var_delta_nd__blk1409 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949)),)
    } else {
        (locals.var_ed__blk1416, locals.var_ed__blk1416_dn4, locals.var_ed__blk1416_dn6, locals.var_ed__blk1416_dn7, locals.var_ed__blk1416_dn8, locals.var_ed__blk1416_dn9,)
    }
};
        locals.var_ed__blk1416 = assign52570_e67749;
        locals.var_ed__blk1416_dn4 = assign52570_e67749_d_n4;
        locals.var_ed__blk1416_dn6 = assign52570_e67749_d_n6;
        locals.var_ed__blk1416_dn7 = assign52570_e67749_d_n7;
        locals.var_ed__blk1416_dn8 = assign52570_e67749_d_n8;
        locals.var_ed__blk1416_dn9 = assign52570_e67749_d_n9;

        let (assign52580_e67768, assign52580_e67768_d_n4, assign52580_e67768_d_n6, assign52580_e67768_d_n7, assign52580_e67768_d_n8, assign52580_e67768_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign52580_e67762: f64 = (locals.var_x_d__blk1410 + 1.0);
        let assign52580_e67764: f64 = (assign52580_e67762 + locals.var_xi0d__blk1415);
        let assign52580_e67765: f64 = (locals.var_delta_nd__blk1409 * assign52580_e67764);
        let assign52580_e67766: f64 = (locals.var_temp__blk949 - assign52580_e67765);
        (assign52580_e67766, (locals.var_temp__blk949_dn4 - ((locals.var_delta_nd__blk1409_dn4 * assign52580_e67764) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn4 + locals.var_xi0d__blk1415_dn4)))), (locals.var_temp__blk949_dn6 - ((locals.var_delta_nd__blk1409_dn6 * assign52580_e67764) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn6 + locals.var_xi0d__blk1415_dn6)))), (locals.var_temp__blk949_dn7 - ((locals.var_delta_nd__blk1409_dn7 * assign52580_e67764) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn7 + locals.var_xi0d__blk1415_dn7)))), (locals.var_temp__blk949_dn8 - ((locals.var_delta_nd__blk1409_dn8 * assign52580_e67764) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn8 + locals.var_xi0d__blk1415_dn8)))), (locals.var_temp__blk949_dn9 - ((locals.var_delta_nd__blk1409_dn9 * assign52580_e67764) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn9 + locals.var_xi0d__blk1415_dn9)))),)
    } else {
        (locals.var_dd__blk1419, locals.var_dd__blk1419_dn4, locals.var_dd__blk1419_dn6, locals.var_dd__blk1419_dn7, locals.var_dd__blk1419_dn8, locals.var_dd__blk1419_dn9,)
    }
};
        locals.var_dd__blk1419 = assign52580_e67768;
        locals.var_dd__blk1419_dn4 = assign52580_e67768_d_n4;
        locals.var_dd__blk1419_dn6 = assign52580_e67768_d_n6;
        locals.var_dd__blk1419_dn7 = assign52580_e67768_d_n7;
        locals.var_dd__blk1419_dn8 = assign52580_e67768_d_n8;
        locals.var_dd__blk1419_dn9 = assign52580_e67768_d_n9;

        let (assign52590_e67802, assign52590_e67802_d_n4, assign52590_e67802_d_n6, assign52590_e67802_d_n7, assign52590_e67802_d_n8, assign52590_e67802_d_n9,) = {
    if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1515 == 0.0)) {
        let assign52590_e67782: f64 = (locals.var_x_d__blk1410 - 230.25850929940458);
        let assign52590_e67787: f64 = (locals.var_x_d__blk1410 - 230.25850929940458);
        let assign52590_e67791: f64 = (locals.var_x_d__blk1410 - 230.25850929940458);
        let assign52590_e67793: f64 = (assign52590_e67791 * 0.3333333333333333);
        let assign52590_e67794: f64 = (1.0 + assign52590_e67793);
        let assign52590_e67795: f64 = (assign52590_e67787 * assign52590_e67794);
        let assign52590_e67796: f64 = (0.5 * assign52590_e67795);
        let assign52590_e67797: f64 = (1.0 + assign52590_e67796);
        let assign52590_e67798: f64 = (assign52590_e67782 * assign52590_e67797);
        let assign52590_e67799: f64 = (1.0 + assign52590_e67798);
        let assign52590_e67800: f64 = (1e-100 / assign52590_e67799);
        (assign52590_e67800, (-((1e-100 * ((locals.var_x_d__blk1410_dn4 * assign52590_e67797) + (assign52590_e67782 * (0.5 * ((locals.var_x_d__blk1410_dn4 * assign52590_e67794) + (assign52590_e67787 * (locals.var_x_d__blk1410_dn4 * 0.3333333333333333))))))) / (assign52590_e67799 * assign52590_e67799))), (-((1e-100 * ((locals.var_x_d__blk1410_dn6 * assign52590_e67797) + (assign52590_e67782 * (0.5 * ((locals.var_x_d__blk1410_dn6 * assign52590_e67794) + (assign52590_e67787 * (locals.var_x_d__blk1410_dn6 * 0.3333333333333333))))))) / (assign52590_e67799 * assign52590_e67799))), (-((1e-100 * ((locals.var_x_d__blk1410_dn7 * assign52590_e67797) + (assign52590_e67782 * (0.5 * ((locals.var_x_d__blk1410_dn7 * assign52590_e67794) + (assign52590_e67787 * (locals.var_x_d__blk1410_dn7 * 0.3333333333333333))))))) / (assign52590_e67799 * assign52590_e67799))), (-((1e-100 * ((locals.var_x_d__blk1410_dn8 * assign52590_e67797) + (assign52590_e67782 * (0.5 * ((locals.var_x_d__blk1410_dn8 * assign52590_e67794) + (assign52590_e67787 * (locals.var_x_d__blk1410_dn8 * 0.3333333333333333))))))) / (assign52590_e67799 * assign52590_e67799))), (-((1e-100 * ((locals.var_x_d__blk1410_dn9 * assign52590_e67797) + (assign52590_e67782 * (0.5 * ((locals.var_x_d__blk1410_dn9 * assign52590_e67794) + (assign52590_e67787 * (locals.var_x_d__blk1410_dn9 * 0.3333333333333333))))))) / (assign52590_e67799 * assign52590_e67799))),)
    } else {
        (locals.var_ed__blk1416, locals.var_ed__blk1416_dn4, locals.var_ed__blk1416_dn6, locals.var_ed__blk1416_dn7, locals.var_ed__blk1416_dn8, locals.var_ed__blk1416_dn9,)
    }
};
        locals.var_ed__blk1416 = assign52590_e67802;
        locals.var_ed__blk1416_dn4 = assign52590_e67802_d_n4;
        locals.var_ed__blk1416_dn6 = assign52590_e67802_d_n6;
        locals.var_ed__blk1416_dn7 = assign52590_e67802_d_n7;
        locals.var_ed__blk1416_dn8 = assign52590_e67802_d_n8;
        locals.var_ed__blk1416_dn9 = assign52590_e67802_d_n9;

    }
}
