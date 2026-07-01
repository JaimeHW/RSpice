#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_56(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign54570_e69666, assign54570_e69666_d_n4, assign54570_e69666_d_n6, assign54570_e69666_d_n7, assign54570_e69666_d_n8, assign54570_e69666_d_n9,) = {
    if (((locals.var_guard1534 != 0.0) && (locals.var_guard1535 != 0.0)) && (locals.var_guard1537 == 0.0)) {
        let assign54570_e69660: f64 = (2.0 * locals.var_xgbeff_ov_s);
        let assign54570_e69663: f64 = (2.0 + locals.var_xgbeff_ov_s);
        let assign54570_e69664: f64 = (assign54570_e69660 / assign54570_e69663);
        (assign54570_e69664, ((((2.0 * locals.var_xgbeff_ov_s_dn4) * assign54570_e69663) - (assign54570_e69660 * locals.var_xgbeff_ov_s_dn4)) / (assign54570_e69663 * assign54570_e69663)), ((((2.0 * locals.var_xgbeff_ov_s_dn6) * assign54570_e69663) - (assign54570_e69660 * locals.var_xgbeff_ov_s_dn6)) / (assign54570_e69663 * assign54570_e69663)), ((((2.0 * locals.var_xgbeff_ov_s_dn7) * assign54570_e69663) - (assign54570_e69660 * locals.var_xgbeff_ov_s_dn7)) / (assign54570_e69663 * assign54570_e69663)), ((((2.0 * locals.var_xgbeff_ov_s_dn8) * assign54570_e69663) - (assign54570_e69660 * locals.var_xgbeff_ov_s_dn8)) / (assign54570_e69663 * assign54570_e69663)), ((((2.0 * locals.var_xgbeff_ov_s_dn9) * assign54570_e69663) - (assign54570_e69660 * locals.var_xgbeff_ov_s_dn9)) / (assign54570_e69663 * assign54570_e69663)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign54570_e69666;
        locals.var_temp1_dn4 = assign54570_e69666_d_n4;
        locals.var_temp1_dn6 = assign54570_e69666_d_n6;
        locals.var_temp1_dn7 = assign54570_e69666_d_n7;
        locals.var_temp1_dn8 = assign54570_e69666_d_n8;
        locals.var_temp1_dn9 = assign54570_e69666_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign54580_e69673, assign54580_e69673_d_n4, assign54580_e69673_d_n6, assign54580_e69673_d_n7, assign54580_e69673_d_n8, assign54580_e69673_d_n9,) = {
    if ((locals.var_guard1534 != 0.0) && (locals.var_guard1535 == 0.0)) {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    } else {
        (locals.var_xgbeff_ov_s, locals.var_xgbeff_ov_s_dn4, locals.var_xgbeff_ov_s_dn6, locals.var_xgbeff_ov_s_dn7, locals.var_xgbeff_ov_s_dn8, locals.var_xgbeff_ov_s_dn9,)
    }
};
        locals.var_xgbeff_ov_s = assign54580_e69673;
        locals.var_xgbeff_ov_s_dn4 = assign54580_e69673_d_n4;
        locals.var_xgbeff_ov_s_dn6 = assign54580_e69673_d_n6;
        locals.var_xgbeff_ov_s_dn7 = assign54580_e69673_d_n7;
        locals.var_xgbeff_ov_s_dn8 = assign54580_e69673_d_n8;
        locals.var_xgbeff_ov_s_dn9 = assign54580_e69673_d_n9;
        locals.var_xgbeff_ov_s_rv = 0.0;

        let (assign54590_e69691, assign54590_e69691_d_n4, assign54590_e69691_d_n6, assign54590_e69691_d_n7, assign54590_e69691_d_n8, assign54590_e69691_d_n9,) = {
    if ((locals.var_guard1534 != 0.0) && (locals.var_guard1535 == 0.0)) {
        let assign54590_e69682: f64 = (1.0 + locals.var_xgbeff_ov_s);
        let assign54590_e69683: f64 = (assign54590_e69682).ln();
        let assign54590_e69686: f64 = (2.0 + locals.var_xgbeff_ov_s);
        let assign54590_e69687: f64 = (assign54590_e69683 / assign54590_e69686);
        let assign54590_e69688: f64 = (1.0 - assign54590_e69687);
        let assign54590_e69689: f64 = (locals.var_xgbeff_ov_s * assign54590_e69688);
        (assign54590_e69689, ((locals.var_xgbeff_ov_s_dn4 * assign54590_e69688) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn4 / assign54590_e69682) * assign54590_e69686) - (assign54590_e69683 * locals.var_xgbeff_ov_s_dn4)) / (assign54590_e69686 * assign54590_e69686))))), ((locals.var_xgbeff_ov_s_dn6 * assign54590_e69688) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn6 / assign54590_e69682) * assign54590_e69686) - (assign54590_e69683 * locals.var_xgbeff_ov_s_dn6)) / (assign54590_e69686 * assign54590_e69686))))), ((locals.var_xgbeff_ov_s_dn7 * assign54590_e69688) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn7 / assign54590_e69682) * assign54590_e69686) - (assign54590_e69683 * locals.var_xgbeff_ov_s_dn7)) / (assign54590_e69686 * assign54590_e69686))))), ((locals.var_xgbeff_ov_s_dn8 * assign54590_e69688) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn8 / assign54590_e69682) * assign54590_e69686) - (assign54590_e69683 * locals.var_xgbeff_ov_s_dn8)) / (assign54590_e69686 * assign54590_e69686))))), ((locals.var_xgbeff_ov_s_dn9 * assign54590_e69688) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn9 / assign54590_e69682) * assign54590_e69686) - (assign54590_e69683 * locals.var_xgbeff_ov_s_dn9)) / (assign54590_e69686 * assign54590_e69686))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign54590_e69691;
        locals.var_temp1_dn4 = assign54590_e69691_d_n4;
        locals.var_temp1_dn6 = assign54590_e69691_d_n6;
        locals.var_temp1_dn7 = assign54590_e69691_d_n7;
        locals.var_temp1_dn8 = assign54590_e69691_d_n8;
        locals.var_temp1_dn9 = assign54590_e69691_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign54600_e69706, assign54600_e69706_d_n4, assign54600_e69706_d_n6, assign54600_e69706_d_n7, assign54600_e69706_d_n8, assign54600_e69706_d_n9,) = {
    if (locals.var_guard1534 != 0.0) {
        let assign54600_e69694: f64 = (-2.0);
        let assign54600_e69696: f64 = (assign54600_e69694 * locals.var_fcgovacc_i);
        let assign54600_e69698: f64 = (assign54600_e69696 / locals.var_cgovaccg_i);
        let assign54600_e69700: f64 = (assign54600_e69698 * locals.var_cgov_i);
        let assign54600_e69702: f64 = (assign54600_e69700 * locals.var_phita);
        let assign54600_e69704: f64 = (assign54600_e69702 * locals.var_temp1);
        (assign54600_e69704, (assign54600_e69702 * locals.var_temp1_dn4), (assign54600_e69702 * locals.var_temp1_dn6), (assign54600_e69702 * locals.var_temp1_dn7), (assign54600_e69702 * locals.var_temp1_dn8), (assign54600_e69702 * locals.var_temp1_dn9),)
    } else {
        (locals.var_qg_ov_s, locals.var_qg_ov_s_dn4, locals.var_qg_ov_s_dn6, locals.var_qg_ov_s_dn7, locals.var_qg_ov_s_dn8, locals.var_qg_ov_s_dn9,)
    }
};
        locals.var_qg_ov_s = assign54600_e69706;
        locals.var_qg_ov_s_dn4 = assign54600_e69706_d_n4;
        locals.var_qg_ov_s_dn6 = assign54600_e69706_d_n6;
        locals.var_qg_ov_s_dn7 = assign54600_e69706_d_n7;
        locals.var_qg_ov_s_dn8 = assign54600_e69706_d_n8;
        locals.var_qg_ov_s_dn9 = assign54600_e69706_d_n9;
        locals.var_qg_ov_s_rv = 0.0;

        locals.var_qg_ov_d = 0.0;
        locals.var_qg_ov_d_dn4 = 0.0;
        locals.var_qg_ov_d_dn6 = 0.0;
        locals.var_qg_ov_d_dn7 = 0.0;
        locals.var_qg_ov_d_dn8 = 0.0;
        locals.var_qg_ov_d_dn9 = 0.0;
        locals.var_qg_ov_d_rv = 0.0;

        locals.var_yb_ov_d = 0.0;
        locals.var_yb_ov_d_dn4 = 0.0;
        locals.var_yb_ov_d_dn6 = 0.0;
        locals.var_yb_ov_d_dn7 = 0.0;
        locals.var_yb_ov_d_dn8 = 0.0;
        locals.var_yb_ov_d_dn9 = 0.0;
        locals.var_yb_ov_d_rv = 0.0;

        let assign54630_e69715: f64 = if ((locals.var_cgovd_i > 0.0) && (locals.var_fcgovaccd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1538 = assign54630_e69715;
        locals.var_guard1538_rv = 0.0;

        let (assign54640_e69725, assign54640_e69725_d_n4, assign54640_e69725_d_n6, assign54640_e69725_d_n7, assign54640_e69725_d_n8, assign54640_e69725_d_n9,) = {
    if (locals.var_guard1538 != 0.0) {
        let assign54640_e69720: f64 = (0.5 * locals.var_xgb_ov);
        let assign54640_e69722: f64 = (assign54640_e69720 + locals.var_dxgb_ov_d);
        let assign54640_e69723: f64 = (locals.var_cgovaccg_i * assign54640_e69722);
        (assign54640_e69723, (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn4)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn6)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn7)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn8)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign54640_e69725;
        locals.var_temp__blk949_dn4 = assign54640_e69725_d_n4;
        locals.var_temp__blk949_dn6 = assign54640_e69725_d_n6;
        locals.var_temp__blk949_dn7 = assign54640_e69725_d_n7;
        locals.var_temp__blk949_dn8 = assign54640_e69725_d_n8;
        locals.var_temp__blk949_dn9 = assign54640_e69725_d_n9;
        locals.var_temp__blk949_rv = 0.0;

        let assign54650_e69728: f64 = if locals.var_temp__blk949 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1539 = assign54650_e69728;
        locals.var_guard1539_rv = 0.0;

        let assign54660_e69731: f64 = (-230.25850929940458);
        let assign54660_e69732: f64 = if locals.var_temp__blk949 > assign54660_e69731 { 1.0 } else { 0.0 };
        locals.var_guard1540 = assign54660_e69732;
        locals.var_guard1540_rv = 0.0;

        let (assign54670_e69741, assign54670_e69741_d_n4, assign54670_e69741_d_n6, assign54670_e69741_d_n7, assign54670_e69741_d_n8, assign54670_e69741_d_n9,) = {
    if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1540 != 0.0)) {
        let assign54670_e69739: f64 = (locals.var_temp__blk949).exp();
        (assign54670_e69739, (assign54670_e69739 * locals.var_temp__blk949_dn4), (assign54670_e69739 * locals.var_temp__blk949_dn6), (assign54670_e69739 * locals.var_temp__blk949_dn7), (assign54670_e69739 * locals.var_temp__blk949_dn8), (assign54670_e69739 * locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_yb_ov_d, locals.var_yb_ov_d_dn4, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8, locals.var_yb_ov_d_dn9,)
    }
};
        locals.var_yb_ov_d = assign54670_e69741;
        locals.var_yb_ov_d_dn4 = assign54670_e69741_d_n4;
        locals.var_yb_ov_d_dn6 = assign54670_e69741_d_n6;
        locals.var_yb_ov_d_dn7 = assign54670_e69741_d_n7;
        locals.var_yb_ov_d_dn8 = assign54670_e69741_d_n8;
        locals.var_yb_ov_d_dn9 = assign54670_e69741_d_n9;
        locals.var_yb_ov_d_rv = 0.0;

        let (assign54680_e69775, assign54680_e69775_d_n4, assign54680_e69775_d_n6, assign54680_e69775_d_n7, assign54680_e69775_d_n8, assign54680_e69775_d_n9,) = {
    if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1540 == 0.0)) {
        let assign54680_e69751: f64 = (-230.25850929940458);
        let assign54680_e69753: f64 = (assign54680_e69751 - locals.var_temp__blk949);
        let assign54680_e69757: f64 = (-230.25850929940458);
        let assign54680_e69759: f64 = (assign54680_e69757 - locals.var_temp__blk949);
        let assign54680_e69762: f64 = (-230.25850929940458);
        let assign54680_e69764: f64 = (assign54680_e69762 - locals.var_temp__blk949);
        let assign54680_e69766: f64 = (assign54680_e69764 * 0.3333333333333333);
        let assign54680_e69767: f64 = (1.0 + assign54680_e69766);
        let assign54680_e69768: f64 = (assign54680_e69759 * assign54680_e69767);
        let assign54680_e69769: f64 = (0.5 * assign54680_e69768);
        let assign54680_e69770: f64 = (1.0 + assign54680_e69769);
        let assign54680_e69771: f64 = (assign54680_e69753 * assign54680_e69770);
        let assign54680_e69772: f64 = (1.0 + assign54680_e69771);
        let assign54680_e69773: f64 = (1e-100 / assign54680_e69772);
        (assign54680_e69773, (-((1e-100 * (((-locals.var_temp__blk949_dn4) * assign54680_e69770) + (assign54680_e69753 * (0.5 * (((-locals.var_temp__blk949_dn4) * assign54680_e69767) + (assign54680_e69759 * ((-locals.var_temp__blk949_dn4) * 0.3333333333333333))))))) / (assign54680_e69772 * assign54680_e69772))), (-((1e-100 * (((-locals.var_temp__blk949_dn6) * assign54680_e69770) + (assign54680_e69753 * (0.5 * (((-locals.var_temp__blk949_dn6) * assign54680_e69767) + (assign54680_e69759 * ((-locals.var_temp__blk949_dn6) * 0.3333333333333333))))))) / (assign54680_e69772 * assign54680_e69772))), (-((1e-100 * (((-locals.var_temp__blk949_dn7) * assign54680_e69770) + (assign54680_e69753 * (0.5 * (((-locals.var_temp__blk949_dn7) * assign54680_e69767) + (assign54680_e69759 * ((-locals.var_temp__blk949_dn7) * 0.3333333333333333))))))) / (assign54680_e69772 * assign54680_e69772))), (-((1e-100 * (((-locals.var_temp__blk949_dn8) * assign54680_e69770) + (assign54680_e69753 * (0.5 * (((-locals.var_temp__blk949_dn8) * assign54680_e69767) + (assign54680_e69759 * ((-locals.var_temp__blk949_dn8) * 0.3333333333333333))))))) / (assign54680_e69772 * assign54680_e69772))), (-((1e-100 * (((-locals.var_temp__blk949_dn9) * assign54680_e69770) + (assign54680_e69753 * (0.5 * (((-locals.var_temp__blk949_dn9) * assign54680_e69767) + (assign54680_e69759 * ((-locals.var_temp__blk949_dn9) * 0.3333333333333333))))))) / (assign54680_e69772 * assign54680_e69772))),)
    } else {
        (locals.var_yb_ov_d, locals.var_yb_ov_d_dn4, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8, locals.var_yb_ov_d_dn9,)
    }
};
        locals.var_yb_ov_d = assign54680_e69775;
        locals.var_yb_ov_d_dn4 = assign54680_e69775_d_n4;
        locals.var_yb_ov_d_dn6 = assign54680_e69775_d_n6;
        locals.var_yb_ov_d_dn7 = assign54680_e69775_d_n7;
        locals.var_yb_ov_d_dn8 = assign54680_e69775_d_n8;
        locals.var_yb_ov_d_dn9 = assign54680_e69775_d_n9;
        locals.var_yb_ov_d_rv = 0.0;

        let assign54690_e69778: f64 = if locals.var_yb_ov_d > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1541 = assign54690_e69778;
        locals.var_guard1541_rv = 0.0;

        let (assign54700_e69789, assign54700_e69789_d_n4, assign54700_e69789_d_n6, assign54700_e69789_d_n7, assign54700_e69789_d_n8, assign54700_e69789_d_n9,) = {
    if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign54700_e69786: f64 = (1.0 + locals.var_yb_ov_d);
        let assign54700_e69787: f64 = (assign54700_e69786).ln();
        (assign54700_e69787, (locals.var_yb_ov_d_dn4 / assign54700_e69786), (locals.var_yb_ov_d_dn6 / assign54700_e69786), (locals.var_yb_ov_d_dn7 / assign54700_e69786), (locals.var_yb_ov_d_dn8 / assign54700_e69786), (locals.var_yb_ov_d_dn9 / assign54700_e69786),)
    } else {
        (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn4, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8, locals.var_xgbeff_ov_d_dn9,)
    }
};
        locals.var_xgbeff_ov_d = assign54700_e69789;
        locals.var_xgbeff_ov_d_dn4 = assign54700_e69789_d_n4;
        locals.var_xgbeff_ov_d_dn6 = assign54700_e69789_d_n6;
        locals.var_xgbeff_ov_d_dn7 = assign54700_e69789_d_n7;
        locals.var_xgbeff_ov_d_dn8 = assign54700_e69789_d_n8;
        locals.var_xgbeff_ov_d_dn9 = assign54700_e69789_d_n9;
        locals.var_xgbeff_ov_d_rv = 0.0;

        let (assign54710_e69808, assign54710_e69808_d_n4, assign54710_e69808_d_n6, assign54710_e69808_d_n7, assign54710_e69808_d_n8, assign54710_e69808_d_n9,) = {
    if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1541 != 0.0)) {
        let assign54710_e69799: f64 = (1.0 + locals.var_xgbeff_ov_d);
        let assign54710_e69800: f64 = (assign54710_e69799).ln();
        let assign54710_e69803: f64 = (2.0 + locals.var_xgbeff_ov_d);
        let assign54710_e69804: f64 = (assign54710_e69800 / assign54710_e69803);
        let assign54710_e69805: f64 = (1.0 - assign54710_e69804);
        let assign54710_e69806: f64 = (locals.var_xgbeff_ov_d * assign54710_e69805);
        (assign54710_e69806, ((locals.var_xgbeff_ov_d_dn4 * assign54710_e69805) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn4 / assign54710_e69799) * assign54710_e69803) - (assign54710_e69800 * locals.var_xgbeff_ov_d_dn4)) / (assign54710_e69803 * assign54710_e69803))))), ((locals.var_xgbeff_ov_d_dn6 * assign54710_e69805) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn6 / assign54710_e69799) * assign54710_e69803) - (assign54710_e69800 * locals.var_xgbeff_ov_d_dn6)) / (assign54710_e69803 * assign54710_e69803))))), ((locals.var_xgbeff_ov_d_dn7 * assign54710_e69805) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn7 / assign54710_e69799) * assign54710_e69803) - (assign54710_e69800 * locals.var_xgbeff_ov_d_dn7)) / (assign54710_e69803 * assign54710_e69803))))), ((locals.var_xgbeff_ov_d_dn8 * assign54710_e69805) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn8 / assign54710_e69799) * assign54710_e69803) - (assign54710_e69800 * locals.var_xgbeff_ov_d_dn8)) / (assign54710_e69803 * assign54710_e69803))))), ((locals.var_xgbeff_ov_d_dn9 * assign54710_e69805) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn9 / assign54710_e69799) * assign54710_e69803) - (assign54710_e69800 * locals.var_xgbeff_ov_d_dn9)) / (assign54710_e69803 * assign54710_e69803))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign54710_e69808;
        locals.var_temp1_dn4 = assign54710_e69808_d_n4;
        locals.var_temp1_dn6 = assign54710_e69808_d_n6;
        locals.var_temp1_dn7 = assign54710_e69808_d_n7;
        locals.var_temp1_dn8 = assign54710_e69808_d_n8;
        locals.var_temp1_dn9 = assign54710_e69808_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign54720_e69817, assign54720_e69817_d_n4, assign54720_e69817_d_n6, assign54720_e69817_d_n7, assign54720_e69817_d_n8, assign54720_e69817_d_n9,) = {
    if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1541 == 0.0)) {
        (locals.var_yb_ov_d, locals.var_yb_ov_d_dn4, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8, locals.var_yb_ov_d_dn9,)
    } else {
        (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn4, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8, locals.var_xgbeff_ov_d_dn9,)
    }
};
        locals.var_xgbeff_ov_d = assign54720_e69817;
        locals.var_xgbeff_ov_d_dn4 = assign54720_e69817_d_n4;
        locals.var_xgbeff_ov_d_dn6 = assign54720_e69817_d_n6;
        locals.var_xgbeff_ov_d_dn7 = assign54720_e69817_d_n7;
        locals.var_xgbeff_ov_d_dn8 = assign54720_e69817_d_n8;
        locals.var_xgbeff_ov_d_dn9 = assign54720_e69817_d_n9;
        locals.var_xgbeff_ov_d_rv = 0.0;

        let (assign54730_e69832, assign54730_e69832_d_n4, assign54730_e69832_d_n6, assign54730_e69832_d_n7, assign54730_e69832_d_n8, assign54730_e69832_d_n9,) = {
    if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1541 == 0.0)) {
        let assign54730_e69826: f64 = (2.0 * locals.var_xgbeff_ov_d);
        let assign54730_e69829: f64 = (2.0 + locals.var_xgbeff_ov_d);
        let assign54730_e69830: f64 = (assign54730_e69826 / assign54730_e69829);
        (assign54730_e69830, ((((2.0 * locals.var_xgbeff_ov_d_dn4) * assign54730_e69829) - (assign54730_e69826 * locals.var_xgbeff_ov_d_dn4)) / (assign54730_e69829 * assign54730_e69829)), ((((2.0 * locals.var_xgbeff_ov_d_dn6) * assign54730_e69829) - (assign54730_e69826 * locals.var_xgbeff_ov_d_dn6)) / (assign54730_e69829 * assign54730_e69829)), ((((2.0 * locals.var_xgbeff_ov_d_dn7) * assign54730_e69829) - (assign54730_e69826 * locals.var_xgbeff_ov_d_dn7)) / (assign54730_e69829 * assign54730_e69829)), ((((2.0 * locals.var_xgbeff_ov_d_dn8) * assign54730_e69829) - (assign54730_e69826 * locals.var_xgbeff_ov_d_dn8)) / (assign54730_e69829 * assign54730_e69829)), ((((2.0 * locals.var_xgbeff_ov_d_dn9) * assign54730_e69829) - (assign54730_e69826 * locals.var_xgbeff_ov_d_dn9)) / (assign54730_e69829 * assign54730_e69829)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign54730_e69832;
        locals.var_temp1_dn4 = assign54730_e69832_d_n4;
        locals.var_temp1_dn6 = assign54730_e69832_d_n6;
        locals.var_temp1_dn7 = assign54730_e69832_d_n7;
        locals.var_temp1_dn8 = assign54730_e69832_d_n8;
        locals.var_temp1_dn9 = assign54730_e69832_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign54740_e69839, assign54740_e69839_d_n4, assign54740_e69839_d_n6, assign54740_e69839_d_n7, assign54740_e69839_d_n8, assign54740_e69839_d_n9,) = {
    if ((locals.var_guard1538 != 0.0) && (locals.var_guard1539 == 0.0)) {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    } else {
        (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn4, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8, locals.var_xgbeff_ov_d_dn9,)
    }
};
        locals.var_xgbeff_ov_d = assign54740_e69839;
        locals.var_xgbeff_ov_d_dn4 = assign54740_e69839_d_n4;
        locals.var_xgbeff_ov_d_dn6 = assign54740_e69839_d_n6;
        locals.var_xgbeff_ov_d_dn7 = assign54740_e69839_d_n7;
        locals.var_xgbeff_ov_d_dn8 = assign54740_e69839_d_n8;
        locals.var_xgbeff_ov_d_dn9 = assign54740_e69839_d_n9;
        locals.var_xgbeff_ov_d_rv = 0.0;

        let (assign54750_e69857, assign54750_e69857_d_n4, assign54750_e69857_d_n6, assign54750_e69857_d_n7, assign54750_e69857_d_n8, assign54750_e69857_d_n9,) = {
    if ((locals.var_guard1538 != 0.0) && (locals.var_guard1539 == 0.0)) {
        let assign54750_e69848: f64 = (1.0 + locals.var_xgbeff_ov_d);
        let assign54750_e69849: f64 = (assign54750_e69848).ln();
        let assign54750_e69852: f64 = (2.0 + locals.var_xgbeff_ov_d);
        let assign54750_e69853: f64 = (assign54750_e69849 / assign54750_e69852);
        let assign54750_e69854: f64 = (1.0 - assign54750_e69853);
        let assign54750_e69855: f64 = (locals.var_xgbeff_ov_d * assign54750_e69854);
        (assign54750_e69855, ((locals.var_xgbeff_ov_d_dn4 * assign54750_e69854) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn4 / assign54750_e69848) * assign54750_e69852) - (assign54750_e69849 * locals.var_xgbeff_ov_d_dn4)) / (assign54750_e69852 * assign54750_e69852))))), ((locals.var_xgbeff_ov_d_dn6 * assign54750_e69854) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn6 / assign54750_e69848) * assign54750_e69852) - (assign54750_e69849 * locals.var_xgbeff_ov_d_dn6)) / (assign54750_e69852 * assign54750_e69852))))), ((locals.var_xgbeff_ov_d_dn7 * assign54750_e69854) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn7 / assign54750_e69848) * assign54750_e69852) - (assign54750_e69849 * locals.var_xgbeff_ov_d_dn7)) / (assign54750_e69852 * assign54750_e69852))))), ((locals.var_xgbeff_ov_d_dn8 * assign54750_e69854) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn8 / assign54750_e69848) * assign54750_e69852) - (assign54750_e69849 * locals.var_xgbeff_ov_d_dn8)) / (assign54750_e69852 * assign54750_e69852))))), ((locals.var_xgbeff_ov_d_dn9 * assign54750_e69854) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn9 / assign54750_e69848) * assign54750_e69852) - (assign54750_e69849 * locals.var_xgbeff_ov_d_dn9)) / (assign54750_e69852 * assign54750_e69852))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign54750_e69857;
        locals.var_temp1_dn4 = assign54750_e69857_d_n4;
        locals.var_temp1_dn6 = assign54750_e69857_d_n6;
        locals.var_temp1_dn7 = assign54750_e69857_d_n7;
        locals.var_temp1_dn8 = assign54750_e69857_d_n8;
        locals.var_temp1_dn9 = assign54750_e69857_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign54760_e69872, assign54760_e69872_d_n4, assign54760_e69872_d_n6, assign54760_e69872_d_n7, assign54760_e69872_d_n8, assign54760_e69872_d_n9,) = {
    if (locals.var_guard1538 != 0.0) {
        let assign54760_e69860: f64 = (-2.0);
        let assign54760_e69862: f64 = (assign54760_e69860 * locals.var_fcgovaccd_i);
        let assign54760_e69864: f64 = (assign54760_e69862 / locals.var_cgovaccg_i);
        let assign54760_e69866: f64 = (assign54760_e69864 * locals.var_cgovd_i);
        let assign54760_e69868: f64 = (assign54760_e69866 * locals.var_phita);
        let assign54760_e69870: f64 = (assign54760_e69868 * locals.var_temp1);
        (assign54760_e69870, (assign54760_e69868 * locals.var_temp1_dn4), (assign54760_e69868 * locals.var_temp1_dn6), (assign54760_e69868 * locals.var_temp1_dn7), (assign54760_e69868 * locals.var_temp1_dn8), (assign54760_e69868 * locals.var_temp1_dn9),)
    } else {
        (locals.var_qg_ov_d, locals.var_qg_ov_d_dn4, locals.var_qg_ov_d_dn6, locals.var_qg_ov_d_dn7, locals.var_qg_ov_d_dn8, locals.var_qg_ov_d_dn9,)
    }
};
        locals.var_qg_ov_d = assign54760_e69872;
        locals.var_qg_ov_d_dn4 = assign54760_e69872_d_n4;
        locals.var_qg_ov_d_dn6 = assign54760_e69872_d_n6;
        locals.var_qg_ov_d_dn7 = assign54760_e69872_d_n7;
        locals.var_qg_ov_d_dn8 = assign54760_e69872_d_n8;
        locals.var_qg_ov_d_dn9 = assign54760_e69872_d_n9;
        locals.var_qg_ov_d_rv = 0.0;

        let assign54770_e69875: f64 = (locals.var_qg_ov_s + locals.var_qg_ov_d);
        locals.var_qg_ov = assign54770_e69875;
        locals.var_qg_ov_dn4 = (locals.var_qg_ov_s_dn4 + locals.var_qg_ov_d_dn4);
        locals.var_qg_ov_dn6 = (locals.var_qg_ov_s_dn6 + locals.var_qg_ov_d_dn6);
        locals.var_qg_ov_dn7 = (locals.var_qg_ov_s_dn7 + locals.var_qg_ov_d_dn7);
        locals.var_qg_ov_dn8 = (locals.var_qg_ov_s_dn8 + locals.var_qg_ov_d_dn8);
        locals.var_qg_ov_dn9 = (locals.var_qg_ov_s_dn9 + locals.var_qg_ov_d_dn9);
        locals.var_qg_ov_rv = 0.0;

        let assign54780_e69878: f64 = (locals.var_cgbov_i * locals.var_vgb);
        let assign54780_e69880: f64 = (assign54780_e69878 + locals.var_qg_ov);
        locals.var_qgb_ov = assign54780_e69880;
        locals.var_qgb_ov_dn4 = locals.var_qg_ov_dn4;
        locals.var_qgb_ov_dn6 = ((locals.var_cgbov_i * locals.var_vgb_dn6) + locals.var_qg_ov_dn6);
        locals.var_qgb_ov_dn7 = ((locals.var_cgbov_i * locals.var_vgb_dn7) + locals.var_qg_ov_dn7);
        locals.var_qgb_ov_dn8 = ((locals.var_cgbov_i * locals.var_vgb_dn8) + locals.var_qg_ov_dn8);
        locals.var_qgb_ov_dn9 = ((locals.var_cgbov_i * locals.var_vgb_dn9) + locals.var_qg_ov_dn9);
        locals.var_qgb_ov_rv = 0.0;

        let assign62240_e80805: f64 = (locals.var_qg + locals.var_qb);
        let assign62240_e80807: f64 = (assign62240_e80805 + locals.var_qd);
        let assign62240_e80808: f64 = (-assign62240_e80807);
        locals.var_qs = assign62240_e80808;
        locals.var_qs_dn4 = (-((locals.var_qg_dn4 + locals.var_qb_dn4) + locals.var_qd_dn4));
        locals.var_qs_dn6 = (-((locals.var_qg_dn6 + locals.var_qb_dn6) + locals.var_qd_dn6));
        locals.var_qs_dn7 = (-((locals.var_qg_dn7 + locals.var_qb_dn7) + locals.var_qd_dn7));
        locals.var_qs_dn8 = (-((locals.var_qg_dn8 + locals.var_qb_dn8) + locals.var_qd_dn8));
        locals.var_qs_dn9 = (-((locals.var_qg_dn9 + locals.var_qb_dn9) + locals.var_qd_dn9));
        locals.var_qs_rv = 0.0;

        let assign62290_e80839: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1749 = assign62290_e80839;
        locals.var_guard1749_rv = 0.0;

        let (assign62300_e80843, assign62300_e80843_d_n4, assign62300_e80843_d_n6, assign62300_e80843_d_n7, assign62300_e80843_d_n8, assign62300_e80843_d_n9,) = {
    if (locals.var_guard1749 != 0.0) {
        (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9,)
    } else {
        (locals.var_temp__blk1748, locals.var_temp__blk1748_dn4, locals.var_temp__blk1748_dn6, locals.var_temp__blk1748_dn7, locals.var_temp__blk1748_dn8, locals.var_temp__blk1748_dn9,)
    }
};
        locals.var_temp__blk1748 = assign62300_e80843;
        locals.var_temp__blk1748_dn4 = assign62300_e80843_d_n4;
        locals.var_temp__blk1748_dn6 = assign62300_e80843_d_n6;
        locals.var_temp__blk1748_dn7 = assign62300_e80843_d_n7;
        locals.var_temp__blk1748_dn8 = assign62300_e80843_d_n8;
        locals.var_temp__blk1748_dn9 = assign62300_e80843_d_n9;
        locals.var_temp__blk1748_rv = 0.0;

        let (assign62310_e80847, assign62310_e80847_d_n4, assign62310_e80847_d_n6, assign62310_e80847_d_n7, assign62310_e80847_d_n8, assign62310_e80847_d_n9,) = {
    if (locals.var_guard1749 != 0.0) {
        (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9,)
    } else {
        (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9,)
    }
};
        locals.var_qd = assign62310_e80847;
        locals.var_qd_dn4 = assign62310_e80847_d_n4;
        locals.var_qd_dn6 = assign62310_e80847_d_n6;
        locals.var_qd_dn7 = assign62310_e80847_d_n7;
        locals.var_qd_dn8 = assign62310_e80847_d_n8;
        locals.var_qd_dn9 = assign62310_e80847_d_n9;
        locals.var_qd_rv = 0.0;

        let (assign62320_e80851, assign62320_e80851_d_n4, assign62320_e80851_d_n6, assign62320_e80851_d_n7, assign62320_e80851_d_n8, assign62320_e80851_d_n9,) = {
    if (locals.var_guard1749 != 0.0) {
        (locals.var_temp__blk1748, locals.var_temp__blk1748_dn4, locals.var_temp__blk1748_dn6, locals.var_temp__blk1748_dn7, locals.var_temp__blk1748_dn8, locals.var_temp__blk1748_dn9,)
    } else {
        (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9,)
    }
};
        locals.var_qs = assign62320_e80851;
        locals.var_qs_dn4 = assign62320_e80851_d_n4;
        locals.var_qs_dn6 = assign62320_e80851_d_n6;
        locals.var_qs_dn7 = assign62320_e80851_d_n7;
        locals.var_qs_dn8 = assign62320_e80851_d_n8;
        locals.var_qs_dn9 = assign62320_e80851_d_n9;
        locals.var_qs_rv = 0.0;

        let assign62390_e80860: f64 = (locals.var_cox_qm * locals.var_eta_p_ac);
        locals.var_cgeff = assign62390_e80860;
        locals.var_cgeff_dn4 = ((locals.var_cox_qm_dn4 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn4));
        locals.var_cgeff_dn6 = ((locals.var_cox_qm_dn6 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn6));
        locals.var_cgeff_dn7 = ((locals.var_cox_qm_dn7 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn7));
        locals.var_cgeff_dn8 = ((locals.var_cox_qm_dn8 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn8));
        locals.var_cgeff_dn9 = ((locals.var_cox_qm_dn9 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn9));
        locals.var_cgeff_rv = 0.0;

        let assign62450_e80872: f64 = if ((locals.var_xg_dc > 0.0) && (locals.var_bet_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1782 = assign62450_e80872;
        locals.var_guard1782_rv = 0.0;

        let assign62760_e81238: f64 = if ((((p.p50 == 1.0) && (locals.var_nt > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1787 = assign62760_e81238;
        locals.var_guard1787_rv = 0.0;

        let (assign62810_e81335, assign62810_e81335_d_n4, assign62810_e81335_d_n6, assign62810_e81335_d_n7, assign62810_e81335_d_n8, assign62810_e81335_d_n9,) = {
    if ((locals.var_guard1782 != 0.0) && (locals.var_guard1787 != 0.0)) {
        let assign62810_e81325: f64 = (locals.var_gvsat_ac * locals.var_gvsat_ac);
        let assign62810_e81327: f64 = (assign62810_e81325 * locals.var_cox_qm);
        let assign62810_e81329: f64 = (assign62810_e81327 * locals.var_eta_p_ac);
        let assign62810_e81332: f64 = (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac);
        let assign62810_e81333: f64 = (assign62810_e81329 / assign62810_e81332);
        (assign62810_e81333, (((((((((locals.var_gvsat_ac_dn4 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn4)) * locals.var_cox_qm) + (assign62810_e81325 * locals.var_cox_qm_dn4)) * locals.var_eta_p_ac) + (assign62810_e81327 * locals.var_eta_p_ac_dn4)) * assign62810_e81332) - (assign62810_e81329 * ((locals.var_gmob_dl_ac_dn4 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn4)))) / (assign62810_e81332 * assign62810_e81332)), (((((((((locals.var_gvsat_ac_dn6 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn6)) * locals.var_cox_qm) + (assign62810_e81325 * locals.var_cox_qm_dn6)) * locals.var_eta_p_ac) + (assign62810_e81327 * locals.var_eta_p_ac_dn6)) * assign62810_e81332) - (assign62810_e81329 * ((locals.var_gmob_dl_ac_dn6 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn6)))) / (assign62810_e81332 * assign62810_e81332)), (((((((((locals.var_gvsat_ac_dn7 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn7)) * locals.var_cox_qm) + (assign62810_e81325 * locals.var_cox_qm_dn7)) * locals.var_eta_p_ac) + (assign62810_e81327 * locals.var_eta_p_ac_dn7)) * assign62810_e81332) - (assign62810_e81329 * ((locals.var_gmob_dl_ac_dn7 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn7)))) / (assign62810_e81332 * assign62810_e81332)), (((((((((locals.var_gvsat_ac_dn8 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn8)) * locals.var_cox_qm) + (assign62810_e81325 * locals.var_cox_qm_dn8)) * locals.var_eta_p_ac) + (assign62810_e81327 * locals.var_eta_p_ac_dn8)) * assign62810_e81332) - (assign62810_e81329 * ((locals.var_gmob_dl_ac_dn8 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn8)))) / (assign62810_e81332 * assign62810_e81332)), (((((((((locals.var_gvsat_ac_dn9 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn9)) * locals.var_cox_qm) + (assign62810_e81325 * locals.var_cox_qm_dn9)) * locals.var_eta_p_ac) + (assign62810_e81327 * locals.var_eta_p_ac_dn9)) * assign62810_e81332) - (assign62810_e81329 * ((locals.var_gmob_dl_ac_dn9 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn9)))) / (assign62810_e81332 * assign62810_e81332)),)
    } else {
        (locals.var_cgeff, locals.var_cgeff_dn4, locals.var_cgeff_dn6, locals.var_cgeff_dn7, locals.var_cgeff_dn8, locals.var_cgeff_dn9,)
    }
};
        locals.var_cgeff = assign62810_e81335;
        locals.var_cgeff_dn4 = assign62810_e81335_d_n4;
        locals.var_cgeff_dn6 = assign62810_e81335_d_n6;
        locals.var_cgeff_dn7 = assign62810_e81335_d_n7;
        locals.var_cgeff_dn8 = assign62810_e81335_d_n8;
        locals.var_cgeff_dn9 = assign62810_e81335_d_n9;
        locals.var_cgeff_rv = 0.0;

        let assign63070_e81549: f64 = if (((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) && (locals.var_xgedge > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1791 = assign63070_e81549;
        locals.var_guard1791_rv = 0.0;

        let (assign63080_e81557, assign63080_e81557_d_n4, assign63080_e81557_d_n6, assign63080_e81557_d_n7, assign63080_e81557_d_n8, assign63080_e81557_d_n9,) = {
    if (locals.var_guard1791 != 0.0) {
        let assign63080_e81553: f64 = (4.0 * locals.var_dsqredge);
        let assign63080_e81555: f64 = (assign63080_e81553 / locals.var_gfedge2);
        (assign63080_e81555, ((((4.0 * locals.var_dsqredge_dn4) * locals.var_gfedge2) - (assign63080_e81553 * locals.var_gfedge2_dn4)) / (locals.var_gfedge2 * locals.var_gfedge2)), ((4.0 * locals.var_dsqredge_dn6) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn7) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn8) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn9) / locals.var_gfedge2),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign63080_e81557;
        locals.var_temp1_dn4 = assign63080_e81557_d_n4;
        locals.var_temp1_dn6 = assign63080_e81557_d_n6;
        locals.var_temp1_dn7 = assign63080_e81557_d_n7;
        locals.var_temp1_dn8 = assign63080_e81557_d_n8;
        locals.var_temp1_dn9 = assign63080_e81557_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign63100_e81577, assign63100_e81577_d_n4, assign63100_e81577_d_n6, assign63100_e81577_d_n7, assign63100_e81577_d_n8, assign63100_e81577_d_n9,) = {
    if (locals.var_guard1791 != 0.0) {
        let assign63100_e81575: f64 = (locals.var_cox_over_q * locals.var_phit);
        (assign63100_e81575, (locals.var_cox_over_q * locals.var_phit_dn4), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign63100_e81577;
        locals.var_temp1_dn4 = assign63100_e81577_d_n4;
        locals.var_temp1_dn6 = assign63100_e81577_d_n6;
        locals.var_temp1_dn7 = assign63100_e81577_d_n7;
        locals.var_temp1_dn8 = assign63100_e81577_d_n8;
        locals.var_temp1_dn9 = assign63100_e81577_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign63230_e81717, assign63230_e81717_d_n4, assign63230_e81717_d_n6, assign63230_e81717_d_n7, assign63230_e81717_d_n8, assign63230_e81717_d_n9,) = {
    if (locals.var_guard1791 != 0.0) {
        let assign63230_e81715: f64 = (locals.var_alpha_dc * locals.var_h_dc);
        (assign63230_e81715, ((locals.var_alpha_dc_dn4 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn4)), ((locals.var_alpha_dc_dn6 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn6)), ((locals.var_alpha_dc_dn7 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn7)), ((locals.var_alpha_dc_dn8 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn8)), ((locals.var_alpha_dc_dn9 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign63230_e81717;
        locals.var_temp1_dn4 = assign63230_e81717_d_n4;
        locals.var_temp1_dn6 = assign63230_e81717_d_n6;
        locals.var_temp1_dn7 = assign63230_e81717_d_n7;
        locals.var_temp1_dn8 = assign63230_e81717_d_n8;
        locals.var_temp1_dn9 = assign63230_e81717_d_n9;
        locals.var_temp1_rv = 0.0;

    }

    pub(super) fn stamp_transient_equations_block_0(
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq0_e972, eq0_e972_d_n4, eq0_e972_d_n6, eq0_e972_d_n7, eq0_e972_d_n8, eq0_e972_d_n9,) = {
    if (locals.var_guard1735 != 0.0) {
        let eq0_e966: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq0_e968: f64 = (eq0_e966 * p.p32);
        let eq0_e970: f64 = (eq0_e968 * locals.var_iimpact);
        let eq0_e970_d_n4: f64 = (eq0_e968 * locals.var_iimpact_dn4);
        let eq0_e970_d_n6: f64 = (eq0_e968 * locals.var_iimpact_dn6);
        let eq0_e970_d_n7: f64 = (eq0_e968 * locals.var_iimpact_dn7);
        let eq0_e970_d_n8: f64 = (eq0_e968 * locals.var_iimpact_dn8);
        let eq0_e970_d_n9: f64 = (eq0_e968 * locals.var_iimpact_dn9);
        (eq0_e970, eq0_e970_d_n4, eq0_e970_d_n6, eq0_e970_d_n7, eq0_e970_d_n8, eq0_e970_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e972;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(9),
            multiplicity * (eq0_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq0_e972_d_n4), multiplicity * (eq0_e972_d_n6), multiplicity * (eq0_e972_d_n7), multiplicity * (eq0_e972_d_n8), multiplicity * (eq0_e972_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq1_e984, eq1_e984_d_n4, eq1_e984_d_n6, eq1_e984_d_n7, eq1_e984_d_n8, eq1_e984_d_n9,) = {
    if (locals.var_guard1735 != 0.0) {
        let eq1_e976: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq1_e978: f64 = (eq1_e976 * p.p32);
        let eq1_e981: f64 = (locals.var_i_ds + locals.var_i_dsedge);
        let eq1_e981_d_n4: f64 = (locals.var_i_ds_dn4 + locals.var_i_dsedge_dn4);
        let eq1_e981_d_n6: f64 = (locals.var_i_ds_dn6 + locals.var_i_dsedge_dn6);
        let eq1_e981_d_n7: f64 = (locals.var_i_ds_dn7 + locals.var_i_dsedge_dn7);
        let eq1_e981_d_n8: f64 = (locals.var_i_ds_dn8 + locals.var_i_dsedge_dn8);
        let eq1_e981_d_n9: f64 = (locals.var_i_ds_dn9 + locals.var_i_dsedge_dn9);
        let eq1_e982: f64 = (eq1_e978 * eq1_e981);
        let eq1_e982_d_n4: f64 = (eq1_e978 * eq1_e981_d_n4);
        let eq1_e982_d_n6: f64 = (eq1_e978 * eq1_e981_d_n6);
        let eq1_e982_d_n7: f64 = (eq1_e978 * eq1_e981_d_n7);
        let eq1_e982_d_n8: f64 = (eq1_e978 * eq1_e981_d_n8);
        let eq1_e982_d_n9: f64 = (eq1_e978 * eq1_e981_d_n9);
        (eq1_e982, eq1_e982_d_n4, eq1_e982_d_n6, eq1_e982_d_n7, eq1_e982_d_n8, eq1_e982_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e984;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq1_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq1_e984_d_n4), multiplicity * (eq1_e984_d_n6), multiplicity * (eq1_e984_d_n7), multiplicity * (eq1_e984_d_n8), multiplicity * (eq1_e984_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq2_e994, eq2_e994_d_n4, eq2_e994_d_n6, eq2_e994_d_n7, eq2_e994_d_n8, eq2_e994_d_n9,) = {
    if (locals.var_guard1735 != 0.0) {
        let eq2_e988: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq2_e990: f64 = (eq2_e988 * p.p32);
        let eq2_e992: f64 = (eq2_e990 * locals.var_i_gcs);
        let eq2_e992_d_n4: f64 = (eq2_e990 * locals.var_i_gcs_dn4);
        let eq2_e992_d_n6: f64 = (eq2_e990 * locals.var_i_gcs_dn6);
        let eq2_e992_d_n7: f64 = (eq2_e990 * locals.var_i_gcs_dn7);
        let eq2_e992_d_n8: f64 = (eq2_e990 * locals.var_i_gcs_dn8);
        let eq2_e992_d_n9: f64 = (eq2_e990 * locals.var_i_gcs_dn9);
        (eq2_e992, eq2_e992_d_n4, eq2_e992_d_n6, eq2_e992_d_n7, eq2_e992_d_n8, eq2_e992_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e994;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq2_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq2_e994_d_n4), multiplicity * (eq2_e994_d_n6), multiplicity * (eq2_e994_d_n7), multiplicity * (eq2_e994_d_n8), multiplicity * (eq2_e994_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq3_e1004, eq3_e1004_d_n4, eq3_e1004_d_n6, eq3_e1004_d_n7, eq3_e1004_d_n8, eq3_e1004_d_n9,) = {
    if (locals.var_guard1735 != 0.0) {
        let eq3_e998: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq3_e1000: f64 = (eq3_e998 * p.p32);
        let eq3_e1002: f64 = (eq3_e1000 * locals.var_i_gcd);
        let eq3_e1002_d_n4: f64 = (eq3_e1000 * locals.var_i_gcd_dn4);
        let eq3_e1002_d_n6: f64 = (eq3_e1000 * locals.var_i_gcd_dn6);
        let eq3_e1002_d_n7: f64 = (eq3_e1000 * locals.var_i_gcd_dn7);
        let eq3_e1002_d_n8: f64 = (eq3_e1000 * locals.var_i_gcd_dn8);
        let eq3_e1002_d_n9: f64 = (eq3_e1000 * locals.var_i_gcd_dn9);
        (eq3_e1002, eq3_e1002_d_n4, eq3_e1002_d_n6, eq3_e1002_d_n7, eq3_e1002_d_n8, eq3_e1002_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e1004;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq3_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq3_e1004_d_n4), multiplicity * (eq3_e1004_d_n6), multiplicity * (eq3_e1004_d_n7), multiplicity * (eq3_e1004_d_n8), multiplicity * (eq3_e1004_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq4_e1015, eq4_e1015_d_n4, eq4_e1015_d_n6, eq4_e1015_d_n7, eq4_e1015_d_n8, eq4_e1015_d_n9,) = {
    if (locals.var_guard1735 == 0.0) {
        let eq4_e1009: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq4_e1011: f64 = (eq4_e1009 * p.p32);
        let eq4_e1013: f64 = (eq4_e1011 * locals.var_iimpact);
        let eq4_e1013_d_n4: f64 = (eq4_e1011 * locals.var_iimpact_dn4);
        let eq4_e1013_d_n6: f64 = (eq4_e1011 * locals.var_iimpact_dn6);
        let eq4_e1013_d_n7: f64 = (eq4_e1011 * locals.var_iimpact_dn7);
        let eq4_e1013_d_n8: f64 = (eq4_e1011 * locals.var_iimpact_dn8);
        let eq4_e1013_d_n9: f64 = (eq4_e1011 * locals.var_iimpact_dn9);
        (eq4_e1013, eq4_e1013_d_n4, eq4_e1013_d_n6, eq4_e1013_d_n7, eq4_e1013_d_n8, eq4_e1013_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1015;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq4_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq4_e1015_d_n4), multiplicity * (eq4_e1015_d_n6), multiplicity * (eq4_e1015_d_n7), multiplicity * (eq4_e1015_d_n8), multiplicity * (eq4_e1015_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq5_e1028, eq5_e1028_d_n4, eq5_e1028_d_n6, eq5_e1028_d_n7, eq5_e1028_d_n8, eq5_e1028_d_n9,) = {
    if (locals.var_guard1735 == 0.0) {
        let eq5_e1020: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq5_e1022: f64 = (eq5_e1020 * p.p32);
        let eq5_e1025: f64 = (locals.var_i_ds + locals.var_i_dsedge);
        let eq5_e1025_d_n4: f64 = (locals.var_i_ds_dn4 + locals.var_i_dsedge_dn4);
        let eq5_e1025_d_n6: f64 = (locals.var_i_ds_dn6 + locals.var_i_dsedge_dn6);
        let eq5_e1025_d_n7: f64 = (locals.var_i_ds_dn7 + locals.var_i_dsedge_dn7);
        let eq5_e1025_d_n8: f64 = (locals.var_i_ds_dn8 + locals.var_i_dsedge_dn8);
        let eq5_e1025_d_n9: f64 = (locals.var_i_ds_dn9 + locals.var_i_dsedge_dn9);
        let eq5_e1026: f64 = (eq5_e1022 * eq5_e1025);
        let eq5_e1026_d_n4: f64 = (eq5_e1022 * eq5_e1025_d_n4);
        let eq5_e1026_d_n6: f64 = (eq5_e1022 * eq5_e1025_d_n6);
        let eq5_e1026_d_n7: f64 = (eq5_e1022 * eq5_e1025_d_n7);
        let eq5_e1026_d_n8: f64 = (eq5_e1022 * eq5_e1025_d_n8);
        let eq5_e1026_d_n9: f64 = (eq5_e1022 * eq5_e1025_d_n9);
        (eq5_e1026, eq5_e1026_d_n4, eq5_e1026_d_n6, eq5_e1026_d_n7, eq5_e1026_d_n8, eq5_e1026_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1028;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq5_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq5_e1028_d_n4), multiplicity * (eq5_e1028_d_n6), multiplicity * (eq5_e1028_d_n7), multiplicity * (eq5_e1028_d_n8), multiplicity * (eq5_e1028_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq6_e1039, eq6_e1039_d_n4, eq6_e1039_d_n6, eq6_e1039_d_n7, eq6_e1039_d_n8, eq6_e1039_d_n9,) = {
    if (locals.var_guard1735 == 0.0) {
        let eq6_e1033: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq6_e1035: f64 = (eq6_e1033 * p.p32);
        let eq6_e1037: f64 = (eq6_e1035 * locals.var_i_gcs);
        let eq6_e1037_d_n4: f64 = (eq6_e1035 * locals.var_i_gcs_dn4);
        let eq6_e1037_d_n6: f64 = (eq6_e1035 * locals.var_i_gcs_dn6);
        let eq6_e1037_d_n7: f64 = (eq6_e1035 * locals.var_i_gcs_dn7);
        let eq6_e1037_d_n8: f64 = (eq6_e1035 * locals.var_i_gcs_dn8);
        let eq6_e1037_d_n9: f64 = (eq6_e1035 * locals.var_i_gcs_dn9);
        (eq6_e1037, eq6_e1037_d_n4, eq6_e1037_d_n6, eq6_e1037_d_n7, eq6_e1037_d_n8, eq6_e1037_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1039;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq6_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq6_e1039_d_n4), multiplicity * (eq6_e1039_d_n6), multiplicity * (eq6_e1039_d_n7), multiplicity * (eq6_e1039_d_n8), multiplicity * (eq6_e1039_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq7_e1050, eq7_e1050_d_n4, eq7_e1050_d_n6, eq7_e1050_d_n7, eq7_e1050_d_n8, eq7_e1050_d_n9,) = {
    if (locals.var_guard1735 == 0.0) {
        let eq7_e1044: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq7_e1046: f64 = (eq7_e1044 * p.p32);
        let eq7_e1048: f64 = (eq7_e1046 * locals.var_i_gcd);
        let eq7_e1048_d_n4: f64 = (eq7_e1046 * locals.var_i_gcd_dn4);
        let eq7_e1048_d_n6: f64 = (eq7_e1046 * locals.var_i_gcd_dn6);
        let eq7_e1048_d_n7: f64 = (eq7_e1046 * locals.var_i_gcd_dn7);
        let eq7_e1048_d_n8: f64 = (eq7_e1046 * locals.var_i_gcd_dn8);
        let eq7_e1048_d_n9: f64 = (eq7_e1046 * locals.var_i_gcd_dn9);
        (eq7_e1048, eq7_e1048_d_n4, eq7_e1048_d_n6, eq7_e1048_d_n7, eq7_e1048_d_n8, eq7_e1048_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1050;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq7_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq7_e1050_d_n4), multiplicity * (eq7_e1050_d_n6), multiplicity * (eq7_e1050_d_n7), multiplicity * (eq7_e1050_d_n8), multiplicity * (eq7_e1050_d_n9)],
            [],
            [],
            1.0,
        );
        let eq8_e1053: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq8_e1055: f64 = (eq8_e1053 * p.p32);
        let eq8_e1057: f64 = (eq8_e1055 * locals.var_i_gb);
        let eq8_e1057_d_n4: f64 = (eq8_e1055 * locals.var_i_gb_dn4);
        let eq8_e1057_d_n6: f64 = (eq8_e1055 * locals.var_i_gb_dn6);
        let eq8_e1057_d_n7: f64 = (eq8_e1055 * locals.var_i_gb_dn7);
        let eq8_e1057_d_n8: f64 = (eq8_e1055 * locals.var_i_gb_dn8);
        let eq8_e1057_d_n9: f64 = (eq8_e1055 * locals.var_i_gb_dn9);
        let eq8_value: f64 = eq8_e1057;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (eq8_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq8_e1057_d_n4), multiplicity * (eq8_e1057_d_n6), multiplicity * (eq8_e1057_d_n7), multiplicity * (eq8_e1057_d_n8), multiplicity * (eq8_e1057_d_n9)],
            [],
            [],
            1.0,
        );
        let eq9_e1060: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq9_e1062: f64 = (eq9_e1060 * p.p32);
        let eq9_e1064: f64 = (eq9_e1062 * locals.var_igsov);
        let eq9_e1064_d_n4: f64 = (eq9_e1062 * locals.var_igsov_dn4);
        let eq9_e1064_d_n6: f64 = (eq9_e1062 * locals.var_igsov_dn6);
        let eq9_e1064_d_n7: f64 = (eq9_e1062 * locals.var_igsov_dn7);
        let eq9_e1064_d_n8: f64 = (eq9_e1062 * locals.var_igsov_dn8);
        let eq9_e1064_d_n9: f64 = (eq9_e1062 * locals.var_igsov_dn9);
        let eq9_value: f64 = eq9_e1064;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq9_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq9_e1064_d_n4), multiplicity * (eq9_e1064_d_n6), multiplicity * (eq9_e1064_d_n7), multiplicity * (eq9_e1064_d_n8), multiplicity * (eq9_e1064_d_n9)],
            [],
            [],
            1.0,
        );
        let eq10_e1067: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq10_e1069: f64 = (eq10_e1067 * p.p32);
        let eq10_e1071: f64 = (eq10_e1069 * locals.var_igdov);
        let eq10_e1071_d_n4: f64 = (eq10_e1069 * locals.var_igdov_dn4);
        let eq10_e1071_d_n6: f64 = (eq10_e1069 * locals.var_igdov_dn6);
        let eq10_e1071_d_n7: f64 = (eq10_e1069 * locals.var_igdov_dn7);
        let eq10_e1071_d_n8: f64 = (eq10_e1069 * locals.var_igdov_dn8);
        let eq10_e1071_d_n9: f64 = (eq10_e1069 * locals.var_igdov_dn9);
        let eq10_value: f64 = eq10_e1071;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq10_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq10_e1071_d_n4), multiplicity * (eq10_e1071_d_n6), multiplicity * (eq10_e1071_d_n7), multiplicity * (eq10_e1071_d_n8), multiplicity * (eq10_e1071_d_n9)],
            [],
            [],
            1.0,
        );
        let eq11_e1074: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq11_e1076: f64 = (eq11_e1074 * p.p32);
        let eq11_e1078: f64 = (eq11_e1076 * locals.var_i_gisl);
        let eq11_e1078_d_n4: f64 = (eq11_e1076 * locals.var_i_gisl_dn4);
        let eq11_e1078_d_n6: f64 = (eq11_e1076 * locals.var_i_gisl_dn6);
        let eq11_e1078_d_n7: f64 = (eq11_e1076 * locals.var_i_gisl_dn7);
        let eq11_e1078_d_n8: f64 = (eq11_e1076 * locals.var_i_gisl_dn8);
        let eq11_e1078_d_n9: f64 = (eq11_e1076 * locals.var_i_gisl_dn9);
        let eq11_value: f64 = eq11_e1078;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq11_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq11_e1078_d_n4), multiplicity * (eq11_e1078_d_n6), multiplicity * (eq11_e1078_d_n7), multiplicity * (eq11_e1078_d_n8), multiplicity * (eq11_e1078_d_n9)],
            [],
            [],
            1.0,
        );
        let eq12_e1081: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq12_e1083: f64 = (eq12_e1081 * p.p32);
        let eq12_e1085: f64 = (eq12_e1083 * locals.var_i_gidl);
        let eq12_e1085_d_n4: f64 = (eq12_e1083 * locals.var_i_gidl_dn4);
        let eq12_e1085_d_n6: f64 = (eq12_e1083 * locals.var_i_gidl_dn6);
        let eq12_e1085_d_n7: f64 = (eq12_e1083 * locals.var_i_gidl_dn7);
        let eq12_e1085_d_n8: f64 = (eq12_e1083 * locals.var_i_gidl_dn8);
        let eq12_e1085_d_n9: f64 = (eq12_e1083 * locals.var_i_gidl_dn9);
        let eq12_value: f64 = eq12_e1085;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(9),
            multiplicity * (eq12_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq12_e1085_d_n4), multiplicity * (eq12_e1085_d_n6), multiplicity * (eq12_e1085_d_n7), multiplicity * (eq12_e1085_d_n8), multiplicity * (eq12_e1085_d_n9)],
            [],
            [],
            1.0,
        );
        let eq38_e1286: f64 = (-locals.var_mult_inst);
        let eq38_e1288: f64 = (eq38_e1286 * locals.var_pdiss_1);
        let eq38_e1288_d_n0: f64 = (eq38_e1286 * locals.var_pdiss_1_dn0);
        let eq38_e1288_d_n2: f64 = (eq38_e1286 * locals.var_pdiss_1_dn2);
        let eq38_e1288_d_n4: f64 = (eq38_e1286 * locals.var_pdiss_1_dn4);
        let eq38_e1288_d_n6: f64 = (eq38_e1286 * locals.var_pdiss_1_dn6);
        let eq38_e1288_d_n7: f64 = (eq38_e1286 * locals.var_pdiss_1_dn7);
        let eq38_e1288_d_n8: f64 = (eq38_e1286 * locals.var_pdiss_1_dn8);
        let eq38_e1288_d_n9: f64 = (eq38_e1286 * locals.var_pdiss_1_dn9);
        let eq38_value: f64 = eq38_e1288;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(4),
            None,
            multiplicity * (eq38_value),
            [0, 2, 4, 6, 7, 8, 9],
            [multiplicity * (eq38_e1288_d_n0), multiplicity * (eq38_e1288_d_n2), multiplicity * (eq38_e1288_d_n4), multiplicity * (eq38_e1288_d_n6), multiplicity * (eq38_e1288_d_n7), multiplicity * (eq38_e1288_d_n8), multiplicity * (eq38_e1288_d_n9)],
            [],
            [],
            1.0,
        );
        let eq39_e1291: f64 = (locals.var_mult_inst * locals.var_cth_i);
        let eq39_e1293: f64 = (eq39_e1291 * (nv4 - 0.0));
        let eq39_e1294: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq39_e1293);
        let eq39_value: f64 = eq39_e1294;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq39_value),
            4,
            multiplicity * ((eq39_e1291 * ddt_scale)),
        );
        let eq40_e1297: f64 = (locals.var_mult_inst * (nv4 - 0.0));
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_rth_t;
        let eq40_e1299: f64 = (eq40_e1297 * __rspice_inv_cse_0);
        let eq40_e1299_d_n4: f64 = (locals.var_mult_inst * __rspice_inv_cse_0);
        let eq40_value: f64 = eq40_e1299;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq40_value),
            4,
            multiplicity * (eq40_e1299_d_n4),
        );
        let eq41_e1302: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq41_e1304: f64 = (eq41_e1302 * p.p33);
        let eq41_e1306: f64 = (eq41_e1304 * locals.var_qg);
        let eq41_e1306_d_n4: f64 = (eq41_e1304 * locals.var_qg_dn4);
        let eq41_e1306_d_n6: f64 = (eq41_e1304 * locals.var_qg_dn6);
        let eq41_e1306_d_n7: f64 = (eq41_e1304 * locals.var_qg_dn7);
        let eq41_e1306_d_n8: f64 = (eq41_e1304 * locals.var_qg_dn8);
        let eq41_e1306_d_n9: f64 = (eq41_e1304 * locals.var_qg_dn9);
        let eq41_e1307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq41_e1306);
        let eq41_value: f64 = eq41_e1307;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq41_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq41_e1306_d_n4 * ddt_scale)), multiplicity * ((eq41_e1306_d_n6 * ddt_scale)), multiplicity * ((eq41_e1306_d_n7 * ddt_scale)), multiplicity * ((eq41_e1306_d_n8 * ddt_scale)), multiplicity * ((eq41_e1306_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq42_e1310: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq42_e1312: f64 = (eq42_e1310 * p.p33);
        let eq42_e1314: f64 = (eq42_e1312 * locals.var_qb);
        let eq42_e1314_d_n4: f64 = (eq42_e1312 * locals.var_qb_dn4);
        let eq42_e1314_d_n6: f64 = (eq42_e1312 * locals.var_qb_dn6);
        let eq42_e1314_d_n7: f64 = (eq42_e1312 * locals.var_qb_dn7);
        let eq42_e1314_d_n8: f64 = (eq42_e1312 * locals.var_qb_dn8);
        let eq42_e1314_d_n9: f64 = (eq42_e1312 * locals.var_qb_dn9);
        let eq42_e1315: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq42_e1314);
        let eq42_value: f64 = eq42_e1315;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq42_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq42_e1314_d_n4 * ddt_scale)), multiplicity * ((eq42_e1314_d_n6 * ddt_scale)), multiplicity * ((eq42_e1314_d_n7 * ddt_scale)), multiplicity * ((eq42_e1314_d_n8 * ddt_scale)), multiplicity * ((eq42_e1314_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq43_e1318: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq43_e1320: f64 = (eq43_e1318 * p.p33);
        let eq43_e1322: f64 = (eq43_e1320 * locals.var_qd);
        let eq43_e1322_d_n4: f64 = (eq43_e1320 * locals.var_qd_dn4);
        let eq43_e1322_d_n6: f64 = (eq43_e1320 * locals.var_qd_dn6);
        let eq43_e1322_d_n7: f64 = (eq43_e1320 * locals.var_qd_dn7);
        let eq43_e1322_d_n8: f64 = (eq43_e1320 * locals.var_qd_dn8);
        let eq43_e1322_d_n9: f64 = (eq43_e1320 * locals.var_qd_dn9);
        let eq43_e1323: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq43_e1322);
        let eq43_value: f64 = eq43_e1323;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq43_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq43_e1322_d_n4 * ddt_scale)), multiplicity * ((eq43_e1322_d_n6 * ddt_scale)), multiplicity * ((eq43_e1322_d_n7 * ddt_scale)), multiplicity * ((eq43_e1322_d_n8 * ddt_scale)), multiplicity * ((eq43_e1322_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq46_e1342: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq46_e1344: f64 = (eq46_e1342 * p.p33);
        let eq46_e1346: f64 = (eq46_e1344 * locals.var_qgb_ov);
        let eq46_e1346_d_n4: f64 = (eq46_e1344 * locals.var_qgb_ov_dn4);
        let eq46_e1346_d_n6: f64 = (eq46_e1344 * locals.var_qgb_ov_dn6);
        let eq46_e1346_d_n7: f64 = (eq46_e1344 * locals.var_qgb_ov_dn7);
        let eq46_e1346_d_n8: f64 = (eq46_e1344 * locals.var_qgb_ov_dn8);
        let eq46_e1346_d_n9: f64 = (eq46_e1344 * locals.var_qgb_ov_dn9);
        let eq46_e1347: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq46_e1346);
        let eq46_value: f64 = eq46_e1347;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (eq46_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq46_e1346_d_n4 * ddt_scale)), multiplicity * ((eq46_e1346_d_n6 * ddt_scale)), multiplicity * ((eq46_e1346_d_n7 * ddt_scale)), multiplicity * ((eq46_e1346_d_n8 * ddt_scale)), multiplicity * ((eq46_e1346_d_n9 * ddt_scale))],
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq50_e1371: f64 = ((nv5 - 0.0) / locals.var_mig);
        let eq50_e1371_d_n4: f64 = (-(((nv5 - 0.0) * locals.var_mig_dn4) / (locals.var_mig * locals.var_mig)));
        let eq50_e1371_d_n5: f64 = (1.0 / locals.var_mig);
        let eq50_e1371_d_n6: f64 = (-(((nv5 - 0.0) * locals.var_mig_dn6) / (locals.var_mig * locals.var_mig)));
        let eq50_e1371_d_n7: f64 = (-(((nv5 - 0.0) * locals.var_mig_dn7) / (locals.var_mig * locals.var_mig)));
        let eq50_e1371_d_n8: f64 = (-(((nv5 - 0.0) * locals.var_mig_dn8) / (locals.var_mig * locals.var_mig)));
        let eq50_e1371_d_n9: f64 = (-(((nv5 - 0.0) * locals.var_mig_dn9) / (locals.var_mig * locals.var_mig)));
        let eq50_value: f64 = eq50_e1371;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq50_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq50_e1371_d_n4), multiplicity * (eq50_e1371_d_n5), multiplicity * (eq50_e1371_d_n6), multiplicity * (eq50_e1371_d_n7), multiplicity * (eq50_e1371_d_n8), multiplicity * (eq50_e1371_d_n9)],
            [],
            [],
            1.0,
        );
        let eq51_e1374: f64 = (locals.var_cgeff * (nv5 - 0.0));
        let eq51_e1374_d_n4: f64 = (locals.var_cgeff_dn4 * (nv5 - 0.0));
        let eq51_e1374_d_n6: f64 = (locals.var_cgeff_dn6 * (nv5 - 0.0));
        let eq51_e1374_d_n7: f64 = (locals.var_cgeff_dn7 * (nv5 - 0.0));
        let eq51_e1374_d_n8: f64 = (locals.var_cgeff_dn8 * (nv5 - 0.0));
        let eq51_e1374_d_n9: f64 = (locals.var_cgeff_dn9 * (nv5 - 0.0));
        let eq51_e1375: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq51_e1374);
        let eq51_value: f64 = eq51_e1375;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq51_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq51_e1374_d_n4 * ddt_scale)), multiplicity * ((locals.var_cgeff * ddt_scale)), multiplicity * ((eq51_e1374_d_n6 * ddt_scale)), multiplicity * ((eq51_e1374_d_n7 * ddt_scale)), multiplicity * ((eq51_e1374_d_n8 * ddt_scale)), multiplicity * ((eq51_e1374_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq52_e1378: f64 = (locals.var_mult_inst * p.p32);
        let eq52_e1379: f64 = (eq52_e1378).sqrt();
        let eq52_e1381: f64 = (eq52_e1379 * 0.5);
        let eq52_e1383: f64 = (eq52_e1381 * locals.var_cgeff);
        let eq52_e1383_d_n4: f64 = (eq52_e1381 * locals.var_cgeff_dn4);
        let eq52_e1383_d_n6: f64 = (eq52_e1381 * locals.var_cgeff_dn6);
        let eq52_e1383_d_n7: f64 = (eq52_e1381 * locals.var_cgeff_dn7);
        let eq52_e1383_d_n8: f64 = (eq52_e1381 * locals.var_cgeff_dn8);
        let eq52_e1383_d_n9: f64 = (eq52_e1381 * locals.var_cgeff_dn9);
        let eq52_e1385: f64 = (eq52_e1383 * (nv5 - 0.0));
        let eq52_e1385_d_n4: f64 = (eq52_e1383_d_n4 * (nv5 - 0.0));
        let eq52_e1385_d_n6: f64 = (eq52_e1383_d_n6 * (nv5 - 0.0));
        let eq52_e1385_d_n7: f64 = (eq52_e1383_d_n7 * (nv5 - 0.0));
        let eq52_e1385_d_n8: f64 = (eq52_e1383_d_n8 * (nv5 - 0.0));
        let eq52_e1385_d_n9: f64 = (eq52_e1383_d_n9 * (nv5 - 0.0));
        let eq52_e1386: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq52_e1385);
        let eq52_e1387: f64 = (-eq52_e1386);
        let eq52_e1387_d_n4: f64 = (-(eq52_e1385_d_n4 * ddt_scale));
        let eq52_e1387_d_n5: f64 = (-(eq52_e1383 * ddt_scale));
        let eq52_e1387_d_n6: f64 = (-(eq52_e1385_d_n6 * ddt_scale));
        let eq52_e1387_d_n7: f64 = (-(eq52_e1385_d_n7 * ddt_scale));
        let eq52_e1387_d_n8: f64 = (-(eq52_e1385_d_n8 * ddt_scale));
        let eq52_e1387_d_n9: f64 = (-(eq52_e1385_d_n9 * ddt_scale));
        let eq52_value: f64 = eq52_e1387;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq52_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq52_e1387_d_n4), multiplicity * (eq52_e1387_d_n5), multiplicity * (eq52_e1387_d_n6), multiplicity * (eq52_e1387_d_n7), multiplicity * (eq52_e1387_d_n8), multiplicity * (eq52_e1387_d_n9)],
            [],
            [],
            1.0,
        );
        let eq53_e1390: f64 = (locals.var_mult_inst * p.p32);
        let eq53_e1391: f64 = (eq53_e1390).sqrt();
        let eq53_e1393: f64 = (eq53_e1391 * 0.5);
        let eq53_e1395: f64 = (eq53_e1393 * locals.var_cgeff);
        let eq53_e1395_d_n4: f64 = (eq53_e1393 * locals.var_cgeff_dn4);
        let eq53_e1395_d_n6: f64 = (eq53_e1393 * locals.var_cgeff_dn6);
        let eq53_e1395_d_n7: f64 = (eq53_e1393 * locals.var_cgeff_dn7);
        let eq53_e1395_d_n8: f64 = (eq53_e1393 * locals.var_cgeff_dn8);
        let eq53_e1395_d_n9: f64 = (eq53_e1393 * locals.var_cgeff_dn9);
        let eq53_e1397: f64 = (eq53_e1395 * (nv5 - 0.0));
        let eq53_e1397_d_n4: f64 = (eq53_e1395_d_n4 * (nv5 - 0.0));
        let eq53_e1397_d_n6: f64 = (eq53_e1395_d_n6 * (nv5 - 0.0));
        let eq53_e1397_d_n7: f64 = (eq53_e1395_d_n7 * (nv5 - 0.0));
        let eq53_e1397_d_n8: f64 = (eq53_e1395_d_n8 * (nv5 - 0.0));
        let eq53_e1397_d_n9: f64 = (eq53_e1395_d_n9 * (nv5 - 0.0));
        let eq53_e1398: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq53_e1397);
        let eq53_e1399: f64 = (-eq53_e1398);
        let eq53_e1399_d_n4: f64 = (-(eq53_e1397_d_n4 * ddt_scale));
        let eq53_e1399_d_n5: f64 = (-(eq53_e1395 * ddt_scale));
        let eq53_e1399_d_n6: f64 = (-(eq53_e1397_d_n6 * ddt_scale));
        let eq53_e1399_d_n7: f64 = (-(eq53_e1397_d_n7 * ddt_scale));
        let eq53_e1399_d_n8: f64 = (-(eq53_e1397_d_n8 * ddt_scale));
        let eq53_e1399_d_n9: f64 = (-(eq53_e1397_d_n9 * ddt_scale));
        let eq53_value: f64 = eq53_e1399;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq53_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq53_e1399_d_n4), multiplicity * (eq53_e1399_d_n5), multiplicity * (eq53_e1399_d_n6), multiplicity * (eq53_e1399_d_n7), multiplicity * (eq53_e1399_d_n8), multiplicity * (eq53_e1399_d_n9)],
            [],
            [],
            1.0,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq39_e1291: f64 = (locals.var_mult_inst * locals.var_cth_i);
        let eq39_e1293: f64 = (eq39_e1291 * (nv4 - 0.0));
        let eq39_e1294_q: f64 = eq39_e1293;
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq39_e1291),
        );
        let eq41_e1302: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq41_e1304: f64 = (eq41_e1302 * p.p33);
        let eq41_e1306: f64 = (eq41_e1304 * locals.var_qg);
        let eq41_e1306_d_n4: f64 = (eq41_e1304 * locals.var_qg_dn4);
        let eq41_e1306_d_n6: f64 = (eq41_e1304 * locals.var_qg_dn6);
        let eq41_e1306_d_n7: f64 = (eq41_e1304 * locals.var_qg_dn7);
        let eq41_e1306_d_n8: f64 = (eq41_e1304 * locals.var_qg_dn8);
        let eq41_e1306_d_n9: f64 = (eq41_e1304 * locals.var_qg_dn9);
        let eq41_e1307_q: f64 = eq41_e1306;
        let eq41_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq41_e1306_d_n4, 0.0, eq41_e1306_d_n6, eq41_e1306_d_n7, eq41_e1306_d_n8, eq41_e1306_d_n9, 0.0, 0.0, 0.0];
        let eq41_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1310: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq42_e1312: f64 = (eq42_e1310 * p.p33);
        let eq42_e1314: f64 = (eq42_e1312 * locals.var_qb);
        let eq42_e1314_d_n4: f64 = (eq42_e1312 * locals.var_qb_dn4);
        let eq42_e1314_d_n6: f64 = (eq42_e1312 * locals.var_qb_dn6);
        let eq42_e1314_d_n7: f64 = (eq42_e1312 * locals.var_qb_dn7);
        let eq42_e1314_d_n8: f64 = (eq42_e1312 * locals.var_qb_dn8);
        let eq42_e1314_d_n9: f64 = (eq42_e1312 * locals.var_qb_dn9);
        let eq42_e1315_q: f64 = eq42_e1314;
        let eq42_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq42_e1314_d_n4, 0.0, eq42_e1314_d_n6, eq42_e1314_d_n7, eq42_e1314_d_n8, eq42_e1314_d_n9, 0.0, 0.0, 0.0];
        let eq42_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e1318: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq43_e1320: f64 = (eq43_e1318 * p.p33);
        let eq43_e1322: f64 = (eq43_e1320 * locals.var_qd);
        let eq43_e1322_d_n4: f64 = (eq43_e1320 * locals.var_qd_dn4);
        let eq43_e1322_d_n6: f64 = (eq43_e1320 * locals.var_qd_dn6);
        let eq43_e1322_d_n7: f64 = (eq43_e1320 * locals.var_qd_dn7);
        let eq43_e1322_d_n8: f64 = (eq43_e1320 * locals.var_qd_dn8);
        let eq43_e1322_d_n9: f64 = (eq43_e1320 * locals.var_qd_dn9);
        let eq43_e1323_q: f64 = eq43_e1322;
        let eq43_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq43_e1322_d_n4, 0.0, eq43_e1322_d_n6, eq43_e1322_d_n7, eq43_e1322_d_n8, eq43_e1322_d_n9, 0.0, 0.0, 0.0];
        let eq43_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let eq46_e1342: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq46_e1344: f64 = (eq46_e1342 * p.p33);
        let eq46_e1346: f64 = (eq46_e1344 * locals.var_qgb_ov);
        let eq46_e1346_d_n4: f64 = (eq46_e1344 * locals.var_qgb_ov_dn4);
        let eq46_e1346_d_n6: f64 = (eq46_e1344 * locals.var_qgb_ov_dn6);
        let eq46_e1346_d_n7: f64 = (eq46_e1344 * locals.var_qgb_ov_dn7);
        let eq46_e1346_d_n8: f64 = (eq46_e1344 * locals.var_qgb_ov_dn8);
        let eq46_e1346_d_n9: f64 = (eq46_e1344 * locals.var_qgb_ov_dn9);
        let eq46_e1347_q: f64 = eq46_e1346;
        let eq46_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq46_e1346_d_n4, 0.0, eq46_e1346_d_n6, eq46_e1346_d_n7, eq46_e1346_d_n8, eq46_e1346_d_n9, 0.0, 0.0, 0.0];
        let eq46_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[9]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let eq51_e1374: f64 = (locals.var_cgeff * (nv5 - 0.0));
        let eq51_e1374_d_n4: f64 = (locals.var_cgeff_dn4 * (nv5 - 0.0));
        let eq51_e1374_d_n6: f64 = (locals.var_cgeff_dn6 * (nv5 - 0.0));
        let eq51_e1374_d_n7: f64 = (locals.var_cgeff_dn7 * (nv5 - 0.0));
        let eq51_e1374_d_n8: f64 = (locals.var_cgeff_dn8 * (nv5 - 0.0));
        let eq51_e1374_d_n9: f64 = (locals.var_cgeff_dn9 * (nv5 - 0.0));
        let eq51_e1375_q: f64 = eq51_e1374;
        let eq51_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq51_e1374_d_n4, locals.var_cgeff, eq51_e1374_d_n6, eq51_e1374_d_n7, eq51_e1374_d_n8, eq51_e1374_d_n9, 0.0, 0.0, 0.0];
        let eq51_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let eq52_e1378: f64 = (locals.var_mult_inst * p.p32);
        let eq52_e1379: f64 = (eq52_e1378).sqrt();
        let eq52_e1381: f64 = (eq52_e1379 * 0.5);
        let eq52_e1383: f64 = (eq52_e1381 * locals.var_cgeff);
        let eq52_e1383_d_n4: f64 = (eq52_e1381 * locals.var_cgeff_dn4);
        let eq52_e1383_d_n6: f64 = (eq52_e1381 * locals.var_cgeff_dn6);
        let eq52_e1383_d_n7: f64 = (eq52_e1381 * locals.var_cgeff_dn7);
        let eq52_e1383_d_n8: f64 = (eq52_e1381 * locals.var_cgeff_dn8);
        let eq52_e1383_d_n9: f64 = (eq52_e1381 * locals.var_cgeff_dn9);
        let eq52_e1385: f64 = (eq52_e1383 * (nv5 - 0.0));
        let eq52_e1385_d_n4: f64 = (eq52_e1383_d_n4 * (nv5 - 0.0));
        let eq52_e1385_d_n6: f64 = (eq52_e1383_d_n6 * (nv5 - 0.0));
        let eq52_e1385_d_n7: f64 = (eq52_e1383_d_n7 * (nv5 - 0.0));
        let eq52_e1385_d_n8: f64 = (eq52_e1383_d_n8 * (nv5 - 0.0));
        let eq52_e1385_d_n9: f64 = (eq52_e1383_d_n9 * (nv5 - 0.0));
        let eq52_e1386_q: f64 = eq52_e1385;
        let eq52_e1387: f64 = (-eq52_e1385);
        let eq52_e1387_q: f64 = (-eq52_e1386_q);
        let eq52_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, (-eq52_e1385_d_n4), (-eq52_e1383), (-eq52_e1385_d_n6), (-eq52_e1385_d_n7), (-eq52_e1385_d_n8), (-eq52_e1385_d_n9), 0.0, 0.0, 0.0];
        let eq52_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq52_reactive_node_derivatives,
            branches,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );
        let eq53_e1390: f64 = (locals.var_mult_inst * p.p32);
        let eq53_e1391: f64 = (eq53_e1390).sqrt();
        let eq53_e1393: f64 = (eq53_e1391 * 0.5);
        let eq53_e1395: f64 = (eq53_e1393 * locals.var_cgeff);
        let eq53_e1395_d_n4: f64 = (eq53_e1393 * locals.var_cgeff_dn4);
        let eq53_e1395_d_n6: f64 = (eq53_e1393 * locals.var_cgeff_dn6);
        let eq53_e1395_d_n7: f64 = (eq53_e1393 * locals.var_cgeff_dn7);
        let eq53_e1395_d_n8: f64 = (eq53_e1393 * locals.var_cgeff_dn8);
        let eq53_e1395_d_n9: f64 = (eq53_e1393 * locals.var_cgeff_dn9);
        let eq53_e1397: f64 = (eq53_e1395 * (nv5 - 0.0));
        let eq53_e1397_d_n4: f64 = (eq53_e1395_d_n4 * (nv5 - 0.0));
        let eq53_e1397_d_n6: f64 = (eq53_e1395_d_n6 * (nv5 - 0.0));
        let eq53_e1397_d_n7: f64 = (eq53_e1395_d_n7 * (nv5 - 0.0));
        let eq53_e1397_d_n8: f64 = (eq53_e1395_d_n8 * (nv5 - 0.0));
        let eq53_e1397_d_n9: f64 = (eq53_e1395_d_n9 * (nv5 - 0.0));
        let eq53_e1398_q: f64 = eq53_e1397;
        let eq53_e1399: f64 = (-eq53_e1397);
        let eq53_e1399_q: f64 = (-eq53_e1398_q);
        let eq53_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, (-eq53_e1397_d_n4), (-eq53_e1395), (-eq53_e1397_d_n6), (-eq53_e1397_d_n7), (-eq53_e1397_d_n8), (-eq53_e1397_d_n9), 0.0, 0.0, 0.0];
        let eq53_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
