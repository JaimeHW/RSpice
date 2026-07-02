#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_64(
        locals: &mut StampLocals,
    ) {
        let (assign25020_e25640, assign25020_e25640_d_n4, assign25020_e25640_d_n6, assign25020_e25640_d_n7, assign25020_e25640_d_n8, assign25020_e25640_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25020_e25628: f64 = (locals.var_sp_ov_nu * locals.var_sp_ov_nu);
        let assign25020_e25632: f64 = (0.5 * locals.var_sp_ov_c);
        let assign25020_e25634: f64 = (assign25020_e25632 * locals.var_sp_ov_c);
        let assign25020_e25636: f64 = (assign25020_e25634 - locals.var_sp_ov_a);
        let assign25020_e25637: f64 = (locals.var_sp_ov_tau * assign25020_e25636);
        let assign25020_e25638: f64 = (assign25020_e25628 + assign25020_e25637);
        (assign25020_e25638, (((locals.var_sp_ov_nu_dn4 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn4)) + ((locals.var_sp_ov_tau_dn4 * assign25020_e25636) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn4) * locals.var_sp_ov_c) + (assign25020_e25632 * locals.var_sp_ov_c_dn4)) - locals.var_sp_ov_a_dn4)))), (((locals.var_sp_ov_nu_dn6 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn6)) + ((locals.var_sp_ov_tau_dn6 * assign25020_e25636) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn6) * locals.var_sp_ov_c) + (assign25020_e25632 * locals.var_sp_ov_c_dn6)) - locals.var_sp_ov_a_dn6)))), (((locals.var_sp_ov_nu_dn7 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn7)) + ((locals.var_sp_ov_tau_dn7 * assign25020_e25636) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn7) * locals.var_sp_ov_c) + (assign25020_e25632 * locals.var_sp_ov_c_dn7)) - locals.var_sp_ov_a_dn7)))), (((locals.var_sp_ov_nu_dn8 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn8)) + ((locals.var_sp_ov_tau_dn8 * assign25020_e25636) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn8) * locals.var_sp_ov_c) + (assign25020_e25632 * locals.var_sp_ov_c_dn8)) - locals.var_sp_ov_a_dn8)))), (((locals.var_sp_ov_nu_dn9 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn9)) + ((locals.var_sp_ov_tau_dn9 * assign25020_e25636) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn9) * locals.var_sp_ov_c) + (assign25020_e25632 * locals.var_sp_ov_c_dn9)) - locals.var_sp_ov_a_dn9)))),)
    } else {
        (locals.var_sp_ov_mutau, locals.var_sp_ov_mutau_dn4, locals.var_sp_ov_mutau_dn6, locals.var_sp_ov_mutau_dn7, locals.var_sp_ov_mutau_dn8, locals.var_sp_ov_mutau_dn9,)
    }
};
        locals.var_sp_ov_mutau = assign25020_e25640;
        locals.var_sp_ov_mutau_dn4 = assign25020_e25640_d_n4;
        locals.var_sp_ov_mutau_dn6 = assign25020_e25640_d_n6;
        locals.var_sp_ov_mutau_dn7 = assign25020_e25640_d_n7;
        locals.var_sp_ov_mutau_dn8 = assign25020_e25640_d_n8;
        locals.var_sp_ov_mutau_dn9 = assign25020_e25640_d_n9;

        let (assign25030_e25667, assign25030_e25667_d_n4, assign25030_e25667_d_n6, assign25030_e25667_d_n7, assign25030_e25667_d_n8, assign25030_e25667_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25030_e25650: f64 = (locals.var_sp_ov_nu / locals.var_sp_ov_mutau);
        let assign25030_e25652: f64 = (assign25030_e25650 * locals.var_sp_ov_tau);
        let assign25030_e25654: f64 = (assign25030_e25652 * locals.var_sp_ov_tau);
        let assign25030_e25656: f64 = (assign25030_e25654 * locals.var_sp_ov_c);
        let assign25030_e25659: f64 = (locals.var_sp_ov_c * locals.var_sp_ov_c);
        let assign25030_e25661: f64 = (assign25030_e25659 * 0.3333333333333);
        let assign25030_e25663: f64 = (assign25030_e25661 - locals.var_sp_ov_a);
        let assign25030_e25664: f64 = (assign25030_e25656 * assign25030_e25663);
        let assign25030_e25665: f64 = (locals.var_sp_ov_mutau + assign25030_e25664);
        (assign25030_e25665, (locals.var_sp_ov_mutau_dn4 + (((((((((((locals.var_sp_ov_nu_dn4 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn4)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign25030_e25650 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_tau) + (assign25030_e25652 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_c) + (assign25030_e25654 * locals.var_sp_ov_c_dn4)) * assign25030_e25663) + (assign25030_e25656 * ((((locals.var_sp_ov_c_dn4 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn4)) * 0.3333333333333) - locals.var_sp_ov_a_dn4)))), (locals.var_sp_ov_mutau_dn6 + (((((((((((locals.var_sp_ov_nu_dn6 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn6)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign25030_e25650 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_tau) + (assign25030_e25652 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_c) + (assign25030_e25654 * locals.var_sp_ov_c_dn6)) * assign25030_e25663) + (assign25030_e25656 * ((((locals.var_sp_ov_c_dn6 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn6)) * 0.3333333333333) - locals.var_sp_ov_a_dn6)))), (locals.var_sp_ov_mutau_dn7 + (((((((((((locals.var_sp_ov_nu_dn7 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn7)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign25030_e25650 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_tau) + (assign25030_e25652 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_c) + (assign25030_e25654 * locals.var_sp_ov_c_dn7)) * assign25030_e25663) + (assign25030_e25656 * ((((locals.var_sp_ov_c_dn7 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn7)) * 0.3333333333333) - locals.var_sp_ov_a_dn7)))), (locals.var_sp_ov_mutau_dn8 + (((((((((((locals.var_sp_ov_nu_dn8 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn8)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign25030_e25650 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_tau) + (assign25030_e25652 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_c) + (assign25030_e25654 * locals.var_sp_ov_c_dn8)) * assign25030_e25663) + (assign25030_e25656 * ((((locals.var_sp_ov_c_dn8 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn8)) * 0.3333333333333) - locals.var_sp_ov_a_dn8)))), (locals.var_sp_ov_mutau_dn9 + (((((((((((locals.var_sp_ov_nu_dn9 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn9)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign25030_e25650 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_tau) + (assign25030_e25652 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_c) + (assign25030_e25654 * locals.var_sp_ov_c_dn9)) * assign25030_e25663) + (assign25030_e25656 * ((((locals.var_sp_ov_c_dn9 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn9)) * 0.3333333333333) - locals.var_sp_ov_a_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign25030_e25667;
        locals.var_sp_ov_temp_dn4 = assign25030_e25667_d_n4;
        locals.var_sp_ov_temp_dn6 = assign25030_e25667_d_n6;
        locals.var_sp_ov_temp_dn7 = assign25030_e25667_d_n7;
        locals.var_sp_ov_temp_dn8 = assign25030_e25667_d_n8;
        locals.var_sp_ov_temp_dn9 = assign25030_e25667_d_n9;

        let (assign25040_e25684, assign25040_e25684_d_n4, assign25040_e25684_d_n6, assign25040_e25684_d_n7, assign25040_e25684_d_n8, assign25040_e25684_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25040_e25677: f64 = (locals.var_sp_ov_a * locals.var_sp_ov_nu);
        let assign25040_e25679: f64 = (assign25040_e25677 * locals.var_sp_ov_tau);
        let assign25040_e25681: f64 = (assign25040_e25679 / locals.var_sp_ov_temp);
        let assign25040_e25682: f64 = (locals.var_sp_ov_eta + assign25040_e25681);
        (assign25040_e25682, (locals.var_sp_ov_eta_dn4 + (((((((locals.var_sp_ov_a_dn4 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn4)) * locals.var_sp_ov_tau) + (assign25040_e25677 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_temp) - (assign25040_e25679 * locals.var_sp_ov_temp_dn4)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn6 + (((((((locals.var_sp_ov_a_dn6 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn6)) * locals.var_sp_ov_tau) + (assign25040_e25677 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_temp) - (assign25040_e25679 * locals.var_sp_ov_temp_dn6)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn7 + (((((((locals.var_sp_ov_a_dn7 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn7)) * locals.var_sp_ov_tau) + (assign25040_e25677 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_temp) - (assign25040_e25679 * locals.var_sp_ov_temp_dn7)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn8 + (((((((locals.var_sp_ov_a_dn8 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn8)) * locals.var_sp_ov_tau) + (assign25040_e25677 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_temp) - (assign25040_e25679 * locals.var_sp_ov_temp_dn8)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn9 + (((((((locals.var_sp_ov_a_dn9 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn9)) * locals.var_sp_ov_tau) + (assign25040_e25677 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_temp) - (assign25040_e25679 * locals.var_sp_ov_temp_dn9)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))),)
    } else {
        (locals.var_sp_ov_y0, locals.var_sp_ov_y0_dn4, locals.var_sp_ov_y0_dn6, locals.var_sp_ov_y0_dn7, locals.var_sp_ov_y0_dn8, locals.var_sp_ov_y0_dn9,)
    }
};
        locals.var_sp_ov_y0 = assign25040_e25684;
        locals.var_sp_ov_y0_dn4 = assign25040_e25684_d_n4;
        locals.var_sp_ov_y0_dn6 = assign25040_e25684_d_n6;
        locals.var_sp_ov_y0_dn7 = assign25040_e25684_d_n7;
        locals.var_sp_ov_y0_dn8 = assign25040_e25684_d_n8;
        locals.var_sp_ov_y0_dn9 = assign25040_e25684_d_n9;

        let assign25050_e25686: f64 = (locals.var_sp_ov_y0).abs();
        let assign25050_e25688: f64 = if assign25050_e25686 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard715 = assign25050_e25688;

        let (assign25060_e25700, assign25060_e25700_d_n4, assign25060_e25700_d_n6, assign25060_e25700_d_n7, assign25060_e25700_d_n8, assign25060_e25700_d_n9,) = {
    if ((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        let assign25060_e25698: f64 = (locals.var_sp_ov_y0).exp();
        (assign25060_e25698, (assign25060_e25698 * locals.var_sp_ov_y0_dn4), (assign25060_e25698 * locals.var_sp_ov_y0_dn6), (assign25060_e25698 * locals.var_sp_ov_y0_dn7), (assign25060_e25698 * locals.var_sp_ov_y0_dn8), (assign25060_e25698 * locals.var_sp_ov_y0_dn9),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign25060_e25700;
        locals.var_sp_ov_d0_dn4 = assign25060_e25700_d_n4;
        locals.var_sp_ov_d0_dn6 = assign25060_e25700_d_n6;
        locals.var_sp_ov_d0_dn7 = assign25060_e25700_d_n7;
        locals.var_sp_ov_d0_dn8 = assign25060_e25700_d_n8;
        locals.var_sp_ov_d0_dn9 = assign25060_e25700_d_n9;

        let assign25070_e25703: f64 = (-80.0);
        let assign25070_e25704: f64 = if locals.var_sp_ov_y0 < assign25070_e25703 { 1.0 } else { 0.0 };
        locals.var_guard716 = assign25070_e25704;

        let (assign25080_e25743, assign25080_e25743_d_n4, assign25080_e25743_d_n6, assign25080_e25743_d_n7, assign25080_e25743_d_n8, assign25080_e25743_d_n9,) = {
    if (((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign25080_e25719: f64 = (-locals.var_sp_ov_y0);
        let assign25080_e25721: f64 = (assign25080_e25719 - 80.0);
        let assign25080_e25725: f64 = (-locals.var_sp_ov_y0);
        let assign25080_e25727: f64 = (assign25080_e25725 - 80.0);
        let assign25080_e25728: f64 = (0.5 * assign25080_e25727);
        let assign25080_e25731: f64 = (-locals.var_sp_ov_y0);
        let assign25080_e25733: f64 = (assign25080_e25731 - 80.0);
        let assign25080_e25735: f64 = (assign25080_e25733 * 0.3333333333333);
        let assign25080_e25736: f64 = (1.0 + assign25080_e25735);
        let assign25080_e25737: f64 = (assign25080_e25728 * assign25080_e25736);
        let assign25080_e25738: f64 = (1.0 + assign25080_e25737);
        let assign25080_e25739: f64 = (assign25080_e25721 * assign25080_e25738);
        let assign25080_e25740: f64 = (1.0 + assign25080_e25739);
        let assign25080_e25741: f64 = (1.80485e-35 / assign25080_e25740);
        (assign25080_e25741, (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn4) * assign25080_e25738) + (assign25080_e25721 * (((0.5 * (-locals.var_sp_ov_y0_dn4)) * assign25080_e25736) + (assign25080_e25728 * ((-locals.var_sp_ov_y0_dn4) * 0.3333333333333)))))) / (assign25080_e25740 * assign25080_e25740))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn6) * assign25080_e25738) + (assign25080_e25721 * (((0.5 * (-locals.var_sp_ov_y0_dn6)) * assign25080_e25736) + (assign25080_e25728 * ((-locals.var_sp_ov_y0_dn6) * 0.3333333333333)))))) / (assign25080_e25740 * assign25080_e25740))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn7) * assign25080_e25738) + (assign25080_e25721 * (((0.5 * (-locals.var_sp_ov_y0_dn7)) * assign25080_e25736) + (assign25080_e25728 * ((-locals.var_sp_ov_y0_dn7) * 0.3333333333333)))))) / (assign25080_e25740 * assign25080_e25740))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn8) * assign25080_e25738) + (assign25080_e25721 * (((0.5 * (-locals.var_sp_ov_y0_dn8)) * assign25080_e25736) + (assign25080_e25728 * ((-locals.var_sp_ov_y0_dn8) * 0.3333333333333)))))) / (assign25080_e25740 * assign25080_e25740))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn9) * assign25080_e25738) + (assign25080_e25721 * (((0.5 * (-locals.var_sp_ov_y0_dn9)) * assign25080_e25736) + (assign25080_e25728 * ((-locals.var_sp_ov_y0_dn9) * 0.3333333333333)))))) / (assign25080_e25740 * assign25080_e25740))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign25080_e25743;
        locals.var_sp_ov_d0_dn4 = assign25080_e25743_d_n4;
        locals.var_sp_ov_d0_dn6 = assign25080_e25743_d_n6;
        locals.var_sp_ov_d0_dn7 = assign25080_e25743_d_n7;
        locals.var_sp_ov_d0_dn8 = assign25080_e25743_d_n8;
        locals.var_sp_ov_d0_dn9 = assign25080_e25743_d_n9;

        let (assign25090_e25780, assign25090_e25780_d_n4, assign25090_e25780_d_n6, assign25090_e25780_d_n7, assign25090_e25780_d_n8, assign25090_e25780_d_n9,) = {
    if (((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 == 0.0)) {
        let assign25090_e25760: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign25090_e25765: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign25090_e25766: f64 = (0.5 * assign25090_e25765);
        let assign25090_e25770: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign25090_e25772: f64 = (assign25090_e25770 * 0.3333333333333);
        let assign25090_e25773: f64 = (1.0 + assign25090_e25772);
        let assign25090_e25774: f64 = (assign25090_e25766 * assign25090_e25773);
        let assign25090_e25775: f64 = (1.0 + assign25090_e25774);
        let assign25090_e25776: f64 = (assign25090_e25760 * assign25090_e25775);
        let assign25090_e25777: f64 = (1.0 + assign25090_e25776);
        let assign25090_e25778: f64 = (5.54062e34 * assign25090_e25777);
        (assign25090_e25778, (5.54062e34 * ((locals.var_sp_ov_y0_dn4 * assign25090_e25775) + (assign25090_e25760 * (((0.5 * locals.var_sp_ov_y0_dn4) * assign25090_e25773) + (assign25090_e25766 * (locals.var_sp_ov_y0_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn6 * assign25090_e25775) + (assign25090_e25760 * (((0.5 * locals.var_sp_ov_y0_dn6) * assign25090_e25773) + (assign25090_e25766 * (locals.var_sp_ov_y0_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn7 * assign25090_e25775) + (assign25090_e25760 * (((0.5 * locals.var_sp_ov_y0_dn7) * assign25090_e25773) + (assign25090_e25766 * (locals.var_sp_ov_y0_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn8 * assign25090_e25775) + (assign25090_e25760 * (((0.5 * locals.var_sp_ov_y0_dn8) * assign25090_e25773) + (assign25090_e25766 * (locals.var_sp_ov_y0_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn9 * assign25090_e25775) + (assign25090_e25760 * (((0.5 * locals.var_sp_ov_y0_dn9) * assign25090_e25773) + (assign25090_e25766 * (locals.var_sp_ov_y0_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign25090_e25780;
        locals.var_sp_ov_d0_dn4 = assign25090_e25780_d_n4;
        locals.var_sp_ov_d0_dn6 = assign25090_e25780_d_n6;
        locals.var_sp_ov_d0_dn7 = assign25090_e25780_d_n7;
        locals.var_sp_ov_d0_dn8 = assign25090_e25780_d_n8;
        locals.var_sp_ov_d0_dn9 = assign25090_e25780_d_n9;

        let (assign25100_e25791, assign25100_e25791_d_n4, assign25100_e25791_d_n6, assign25100_e25791_d_n7, assign25100_e25791_d_n8, assign25100_e25791_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25100_e25789: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_y0);
        (assign25100_e25789, (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_y0_dn4), (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_y0_dn6), (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_y0_dn7), (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_y0_dn8), (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_y0_dn9),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign25100_e25791;
        locals.var_sp_ov_temp_dn4 = assign25100_e25791_d_n4;
        locals.var_sp_ov_temp_dn6 = assign25100_e25791_d_n6;
        locals.var_sp_ov_temp_dn7 = assign25100_e25791_d_n7;
        locals.var_sp_ov_temp_dn8 = assign25100_e25791_d_n8;
        locals.var_sp_ov_temp_dn9 = assign25100_e25791_d_n9;

        let (assign25110_e25808, assign25110_e25808_d_n4, assign25110_e25808_d_n6, assign25110_e25808_d_n7, assign25110_e25808_d_n8, assign25110_e25808_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25110_e25800: f64 = (2.0 * locals.var_sp_ov_temp);
        let assign25110_e25804: f64 = (locals.var_sp_ov_d0 - 1.0);
        let assign25110_e25805: f64 = (locals.var_gov2 * assign25110_e25804);
        let assign25110_e25806: f64 = (assign25110_e25800 + assign25110_e25805);
        (assign25110_e25806, ((2.0 * locals.var_sp_ov_temp_dn4) + ((locals.var_gov2_dn4 * assign25110_e25804) + (locals.var_gov2 * locals.var_sp_ov_d0_dn4))), ((2.0 * locals.var_sp_ov_temp_dn6) + ((locals.var_gov2_dn6 * assign25110_e25804) + (locals.var_gov2 * locals.var_sp_ov_d0_dn6))), ((2.0 * locals.var_sp_ov_temp_dn7) + ((locals.var_gov2_dn7 * assign25110_e25804) + (locals.var_gov2 * locals.var_sp_ov_d0_dn7))), ((2.0 * locals.var_sp_ov_temp_dn8) + ((locals.var_gov2_dn8 * assign25110_e25804) + (locals.var_gov2 * locals.var_sp_ov_d0_dn8))), ((2.0 * locals.var_sp_ov_temp_dn9) + ((locals.var_gov2_dn9 * assign25110_e25804) + (locals.var_gov2 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_p, locals.var_sp_ov_p_dn4, locals.var_sp_ov_p_dn6, locals.var_sp_ov_p_dn7, locals.var_sp_ov_p_dn8, locals.var_sp_ov_p_dn9,)
    }
};
        locals.var_sp_ov_p = assign25110_e25808;
        locals.var_sp_ov_p_dn4 = assign25110_e25808_d_n4;
        locals.var_sp_ov_p_dn6 = assign25110_e25808_d_n6;
        locals.var_sp_ov_p_dn7 = assign25110_e25808_d_n7;
        locals.var_sp_ov_p_dn8 = assign25110_e25808_d_n8;
        locals.var_sp_ov_p_dn9 = assign25110_e25808_d_n9;

        let (assign25120_e25827, assign25120_e25827_d_n4, assign25120_e25827_d_n6, assign25120_e25827_d_n7, assign25120_e25827_d_n8, assign25120_e25827_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25120_e25817: f64 = (locals.var_sp_ov_temp * locals.var_sp_ov_temp);
        let assign25120_e25821: f64 = (locals.var_sp_ov_y0 + 1.0);
        let assign25120_e25823: f64 = (assign25120_e25821 - locals.var_sp_ov_d0);
        let assign25120_e25824: f64 = (locals.var_gov2 * assign25120_e25823);
        let assign25120_e25825: f64 = (assign25120_e25817 + assign25120_e25824);
        (assign25120_e25825, (((locals.var_sp_ov_temp_dn4 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn4)) + ((locals.var_gov2_dn4 * assign25120_e25823) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn4 - locals.var_sp_ov_d0_dn4)))), (((locals.var_sp_ov_temp_dn6 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn6)) + ((locals.var_gov2_dn6 * assign25120_e25823) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn6 - locals.var_sp_ov_d0_dn6)))), (((locals.var_sp_ov_temp_dn7 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn7)) + ((locals.var_gov2_dn7 * assign25120_e25823) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn7 - locals.var_sp_ov_d0_dn7)))), (((locals.var_sp_ov_temp_dn8 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn8)) + ((locals.var_gov2_dn8 * assign25120_e25823) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn8 - locals.var_sp_ov_d0_dn8)))), (((locals.var_sp_ov_temp_dn9 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn9)) + ((locals.var_gov2_dn9 * assign25120_e25823) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn9 - locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_q, locals.var_sp_ov_q_dn4, locals.var_sp_ov_q_dn6, locals.var_sp_ov_q_dn7, locals.var_sp_ov_q_dn8, locals.var_sp_ov_q_dn9,)
    }
};
        locals.var_sp_ov_q = assign25120_e25827;
        locals.var_sp_ov_q_dn4 = assign25120_e25827_d_n4;
        locals.var_sp_ov_q_dn6 = assign25120_e25827_d_n6;
        locals.var_sp_ov_q_dn7 = assign25120_e25827_d_n7;
        locals.var_sp_ov_q_dn8 = assign25120_e25827_d_n8;
        locals.var_sp_ov_q_dn9 = assign25120_e25827_d_n9;

        let (assign25130_e25842, assign25130_e25842_d_n4, assign25130_e25842_d_n6, assign25130_e25842_d_n7, assign25130_e25842_d_n8, assign25130_e25842_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25130_e25837: f64 = (locals.var_gov2 * 0.5);
        let assign25130_e25839: f64 = (assign25130_e25837 * locals.var_sp_ov_d0);
        let assign25130_e25840: f64 = (1.0 - assign25130_e25839);
        (assign25130_e25840, (-(((locals.var_gov2_dn4 * 0.5) * locals.var_sp_ov_d0) + (assign25130_e25837 * locals.var_sp_ov_d0_dn4))), (-(((locals.var_gov2_dn6 * 0.5) * locals.var_sp_ov_d0) + (assign25130_e25837 * locals.var_sp_ov_d0_dn6))), (-(((locals.var_gov2_dn7 * 0.5) * locals.var_sp_ov_d0) + (assign25130_e25837 * locals.var_sp_ov_d0_dn7))), (-(((locals.var_gov2_dn8 * 0.5) * locals.var_sp_ov_d0) + (assign25130_e25837 * locals.var_sp_ov_d0_dn8))), (-(((locals.var_gov2_dn9 * 0.5) * locals.var_sp_ov_d0) + (assign25130_e25837 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_xi, locals.var_sp_ov_xi_dn4, locals.var_sp_ov_xi_dn6, locals.var_sp_ov_xi_dn7, locals.var_sp_ov_xi_dn8, locals.var_sp_ov_xi_dn9,)
    }
};
        locals.var_sp_ov_xi = assign25130_e25842;
        locals.var_sp_ov_xi_dn4 = assign25130_e25842_d_n4;
        locals.var_sp_ov_xi_dn6 = assign25130_e25842_d_n6;
        locals.var_sp_ov_xi_dn7 = assign25130_e25842_d_n7;
        locals.var_sp_ov_xi_dn8 = assign25130_e25842_d_n8;
        locals.var_sp_ov_xi_dn9 = assign25130_e25842_d_n9;

        let (assign25140_e25859, assign25140_e25859_d_n4, assign25140_e25859_d_n6, assign25140_e25859_d_n7, assign25140_e25859_d_n8, assign25140_e25859_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25140_e25851: f64 = (locals.var_sp_ov_p * locals.var_sp_ov_p);
        let assign25140_e25855: f64 = (locals.var_sp_ov_xi * locals.var_sp_ov_q);
        let assign25140_e25856: f64 = (4.0 * assign25140_e25855);
        let assign25140_e25857: f64 = (assign25140_e25851 - assign25140_e25856);
        (assign25140_e25857, (((locals.var_sp_ov_p_dn4 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn4)) - (4.0 * ((locals.var_sp_ov_xi_dn4 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn4)))), (((locals.var_sp_ov_p_dn6 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn6)) - (4.0 * ((locals.var_sp_ov_xi_dn6 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn6)))), (((locals.var_sp_ov_p_dn7 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn7)) - (4.0 * ((locals.var_sp_ov_xi_dn7 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn7)))), (((locals.var_sp_ov_p_dn8 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn8)) - (4.0 * ((locals.var_sp_ov_xi_dn8 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn8)))), (((locals.var_sp_ov_p_dn9 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn9)) - (4.0 * ((locals.var_sp_ov_xi_dn9 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign25140_e25859;
        locals.var_sp_ov_temp_dn4 = assign25140_e25859_d_n4;
        locals.var_sp_ov_temp_dn6 = assign25140_e25859_d_n6;
        locals.var_sp_ov_temp_dn7 = assign25140_e25859_d_n7;
        locals.var_sp_ov_temp_dn8 = assign25140_e25859_d_n8;
        locals.var_sp_ov_temp_dn9 = assign25140_e25859_d_n9;

        let (assign25150_e25875, assign25150_e25875_d_n4, assign25150_e25875_d_n6, assign25150_e25875_d_n7, assign25150_e25875_d_n8, assign25150_e25875_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25150_e25868: f64 = (2.0 * locals.var_sp_ov_q);
        let assign25150_e25871: f64 = (locals.var_sp_ov_temp).sqrt();
        let assign25150_e25872: f64 = (locals.var_sp_ov_p + assign25150_e25871);
        let assign25150_e25873: f64 = (assign25150_e25868 / assign25150_e25872);
        (assign25150_e25873, ((((2.0 * locals.var_sp_ov_q_dn4) * assign25150_e25872) - (assign25150_e25868 * (locals.var_sp_ov_p_dn4 + (locals.var_sp_ov_temp_dn4 / (2.0 * assign25150_e25871))))) / (assign25150_e25872 * assign25150_e25872)), ((((2.0 * locals.var_sp_ov_q_dn6) * assign25150_e25872) - (assign25150_e25868 * (locals.var_sp_ov_p_dn6 + (locals.var_sp_ov_temp_dn6 / (2.0 * assign25150_e25871))))) / (assign25150_e25872 * assign25150_e25872)), ((((2.0 * locals.var_sp_ov_q_dn7) * assign25150_e25872) - (assign25150_e25868 * (locals.var_sp_ov_p_dn7 + (locals.var_sp_ov_temp_dn7 / (2.0 * assign25150_e25871))))) / (assign25150_e25872 * assign25150_e25872)), ((((2.0 * locals.var_sp_ov_q_dn8) * assign25150_e25872) - (assign25150_e25868 * (locals.var_sp_ov_p_dn8 + (locals.var_sp_ov_temp_dn8 / (2.0 * assign25150_e25871))))) / (assign25150_e25872 * assign25150_e25872)), ((((2.0 * locals.var_sp_ov_q_dn9) * assign25150_e25872) - (assign25150_e25868 * (locals.var_sp_ov_p_dn9 + (locals.var_sp_ov_temp_dn9 / (2.0 * assign25150_e25871))))) / (assign25150_e25872 * assign25150_e25872)),)
    } else {
        (locals.var_sp_ov_w, locals.var_sp_ov_w_dn4, locals.var_sp_ov_w_dn6, locals.var_sp_ov_w_dn7, locals.var_sp_ov_w_dn8, locals.var_sp_ov_w_dn9,)
    }
};
        locals.var_sp_ov_w = assign25150_e25875;
        locals.var_sp_ov_w_dn4 = assign25150_e25875_d_n4;
        locals.var_sp_ov_w_dn6 = assign25150_e25875_d_n6;
        locals.var_sp_ov_w_dn7 = assign25150_e25875_d_n7;
        locals.var_sp_ov_w_dn8 = assign25150_e25875_d_n8;
        locals.var_sp_ov_w_dn9 = assign25150_e25875_d_n9;

        let (assign25160_e25887, assign25160_e25887_d_n4, assign25160_e25887_d_n6, assign25160_e25887_d_n7, assign25160_e25887_d_n8, assign25160_e25887_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25160_e25884: f64 = (locals.var_sp_ov_y0 + locals.var_sp_ov_w);
        let assign25160_e25885: f64 = (-assign25160_e25884);
        (assign25160_e25885, (-(locals.var_sp_ov_y0_dn4 + locals.var_sp_ov_w_dn4)), (-(locals.var_sp_ov_y0_dn6 + locals.var_sp_ov_w_dn6)), (-(locals.var_sp_ov_y0_dn7 + locals.var_sp_ov_w_dn7)), (-(locals.var_sp_ov_y0_dn8 + locals.var_sp_ov_w_dn8)), (-(locals.var_sp_ov_y0_dn9 + locals.var_sp_ov_w_dn9)),)
    } else {
        (locals.var_xd_ovcv, locals.var_xd_ovcv_dn4, locals.var_xd_ovcv_dn6, locals.var_xd_ovcv_dn7, locals.var_xd_ovcv_dn8, locals.var_xd_ovcv_dn9,)
    }
};
        locals.var_xd_ovcv = assign25160_e25887;
        locals.var_xd_ovcv_dn4 = assign25160_e25887_d_n4;
        locals.var_xd_ovcv_dn6 = assign25160_e25887_d_n6;
        locals.var_xd_ovcv_dn7 = assign25160_e25887_d_n7;
        locals.var_xd_ovcv_dn8 = assign25160_e25887_d_n8;
        locals.var_xd_ovcv_dn9 = assign25160_e25887_d_n9;

        let (assign25170_e25905, assign25170_e25905_d_n4, assign25170_e25905_d_n6, assign25170_e25905_d_n7, assign25170_e25905_d_n8, assign25170_e25905_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25170_e25897: f64 = (locals.var_xi_ov * 1.25);
        let assign25170_e25899: f64 = (assign25170_e25897 * locals.var_inv_xg1);
        let assign25170_e25901: f64 = (assign25170_e25899 - 1.0);
        let assign25170_e25903: f64 = (assign25170_e25901 * locals.var_inv_xg1);
        (assign25170_e25903, (((((locals.var_xi_ov_dn4 * 1.25) * locals.var_inv_xg1) + (assign25170_e25897 * locals.var_inv_xg1_dn4)) * locals.var_inv_xg1) + (assign25170_e25901 * locals.var_inv_xg1_dn4)), (((((locals.var_xi_ov_dn6 * 1.25) * locals.var_inv_xg1) + (assign25170_e25897 * locals.var_inv_xg1_dn6)) * locals.var_inv_xg1) + (assign25170_e25901 * locals.var_inv_xg1_dn6)), (((((locals.var_xi_ov_dn7 * 1.25) * locals.var_inv_xg1) + (assign25170_e25897 * locals.var_inv_xg1_dn7)) * locals.var_inv_xg1) + (assign25170_e25901 * locals.var_inv_xg1_dn7)), (((((locals.var_xi_ov_dn8 * 1.25) * locals.var_inv_xg1) + (assign25170_e25897 * locals.var_inv_xg1_dn8)) * locals.var_inv_xg1) + (assign25170_e25901 * locals.var_inv_xg1_dn8)), (((((locals.var_xi_ov_dn9 * 1.25) * locals.var_inv_xg1) + (assign25170_e25897 * locals.var_inv_xg1_dn9)) * locals.var_inv_xg1) + (assign25170_e25901 * locals.var_inv_xg1_dn9)),)
    } else {
        (locals.var_sp_ov_afac, locals.var_sp_ov_afac_dn4, locals.var_sp_ov_afac_dn6, locals.var_sp_ov_afac_dn7, locals.var_sp_ov_afac_dn8, locals.var_sp_ov_afac_dn9,)
    }
};
        locals.var_sp_ov_afac = assign25170_e25905;
        locals.var_sp_ov_afac_dn4 = assign25170_e25905_d_n4;
        locals.var_sp_ov_afac_dn6 = assign25170_e25905_d_n6;
        locals.var_sp_ov_afac_dn7 = assign25170_e25905_d_n7;
        locals.var_sp_ov_afac_dn8 = assign25170_e25905_d_n8;
        locals.var_sp_ov_afac_dn9 = assign25170_e25905_d_n9;

        let (assign25180_e25923, assign25180_e25923_d_n4, assign25180_e25923_d_n6, assign25180_e25923_d_n7, assign25180_e25923_d_n8, assign25180_e25923_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25180_e25915: f64 = (locals.var_xgd_ovcv * locals.var_inv_xi_ov);
        let assign25180_e25919: f64 = (locals.var_sp_ov_afac * locals.var_xgd_ovcv);
        let assign25180_e25920: f64 = (1.0 + assign25180_e25919);
        let assign25180_e25921: f64 = (assign25180_e25915 * assign25180_e25920);
        (assign25180_e25921, ((((locals.var_xgd_ovcv_dn4 * locals.var_inv_xi_ov) + (locals.var_xgd_ovcv * locals.var_inv_xi_ov_dn4)) * assign25180_e25920) + (assign25180_e25915 * ((locals.var_sp_ov_afac_dn4 * locals.var_xgd_ovcv) + (locals.var_sp_ov_afac * locals.var_xgd_ovcv_dn4)))), ((((locals.var_xgd_ovcv_dn6 * locals.var_inv_xi_ov) + (locals.var_xgd_ovcv * locals.var_inv_xi_ov_dn6)) * assign25180_e25920) + (assign25180_e25915 * ((locals.var_sp_ov_afac_dn6 * locals.var_xgd_ovcv) + (locals.var_sp_ov_afac * locals.var_xgd_ovcv_dn6)))), ((((locals.var_xgd_ovcv_dn7 * locals.var_inv_xi_ov) + (locals.var_xgd_ovcv * locals.var_inv_xi_ov_dn7)) * assign25180_e25920) + (assign25180_e25915 * ((locals.var_sp_ov_afac_dn7 * locals.var_xgd_ovcv) + (locals.var_sp_ov_afac * locals.var_xgd_ovcv_dn7)))), ((((locals.var_xgd_ovcv_dn8 * locals.var_inv_xi_ov) + (locals.var_xgd_ovcv * locals.var_inv_xi_ov_dn8)) * assign25180_e25920) + (assign25180_e25915 * ((locals.var_sp_ov_afac_dn8 * locals.var_xgd_ovcv) + (locals.var_sp_ov_afac * locals.var_xgd_ovcv_dn8)))), ((((locals.var_xgd_ovcv_dn9 * locals.var_inv_xi_ov) + (locals.var_xgd_ovcv * locals.var_inv_xi_ov_dn9)) * assign25180_e25920) + (assign25180_e25915 * ((locals.var_sp_ov_afac_dn9 * locals.var_xgd_ovcv) + (locals.var_sp_ov_afac * locals.var_xgd_ovcv_dn9)))),)
    } else {
        (locals.var_sp_ov_xbar, locals.var_sp_ov_xbar_dn4, locals.var_sp_ov_xbar_dn6, locals.var_sp_ov_xbar_dn7, locals.var_sp_ov_xbar_dn8, locals.var_sp_ov_xbar_dn9,)
    }
};
        locals.var_sp_ov_xbar = assign25180_e25923;
        locals.var_sp_ov_xbar_dn4 = assign25180_e25923_d_n4;
        locals.var_sp_ov_xbar_dn6 = assign25180_e25923_d_n6;
        locals.var_sp_ov_xbar_dn7 = assign25180_e25923_d_n7;
        locals.var_sp_ov_xbar_dn8 = assign25180_e25923_d_n8;
        locals.var_sp_ov_xbar_dn9 = assign25180_e25923_d_n9;

        let assign25190_e25925: f64 = (-locals.var_sp_ov_xbar);
        let assign25190_e25926: f64 = (assign25190_e25925).abs();
        let assign25190_e25928: f64 = if assign25190_e25926 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard717 = assign25190_e25928;

        let (assign25200_e25942, assign25200_e25942_d_n4, assign25200_e25942_d_n6, assign25200_e25942_d_n7, assign25200_e25942_d_n8, assign25200_e25942_d_n9,) = {
    if ((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard717 != 0.0)) {
        let assign25200_e25939: f64 = (-locals.var_sp_ov_xbar);
        let assign25200_e25940: f64 = (assign25200_e25939).exp();
        (assign25200_e25940, (assign25200_e25940 * (-locals.var_sp_ov_xbar_dn4)), (assign25200_e25940 * (-locals.var_sp_ov_xbar_dn6)), (assign25200_e25940 * (-locals.var_sp_ov_xbar_dn7)), (assign25200_e25940 * (-locals.var_sp_ov_xbar_dn8)), (assign25200_e25940 * (-locals.var_sp_ov_xbar_dn9)),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign25200_e25942;
        locals.var_sp_ov_temp_dn4 = assign25200_e25942_d_n4;
        locals.var_sp_ov_temp_dn6 = assign25200_e25942_d_n6;
        locals.var_sp_ov_temp_dn7 = assign25200_e25942_d_n7;
        locals.var_sp_ov_temp_dn8 = assign25200_e25942_d_n8;
        locals.var_sp_ov_temp_dn9 = assign25200_e25942_d_n9;

        let assign25210_e25944: f64 = (-locals.var_sp_ov_xbar);
        let assign25210_e25946: f64 = (-80.0);
        let assign25210_e25947: f64 = if assign25210_e25944 < assign25210_e25946 { 1.0 } else { 0.0 };
        locals.var_guard718 = assign25210_e25947;

        let (assign25220_e25990, assign25220_e25990_d_n4, assign25220_e25990_d_n6, assign25220_e25990_d_n7, assign25220_e25990_d_n8, assign25220_e25990_d_n9,) = {
    if (((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard717 == 0.0)) && (locals.var_guard718 != 0.0)) {
        let assign25220_e25963: f64 = (-locals.var_sp_ov_xbar);
        let assign25220_e25964: f64 = (-assign25220_e25963);
        let assign25220_e25966: f64 = (assign25220_e25964 - 80.0);
        let assign25220_e25970: f64 = (-locals.var_sp_ov_xbar);
        let assign25220_e25971: f64 = (-assign25220_e25970);
        let assign25220_e25973: f64 = (assign25220_e25971 - 80.0);
        let assign25220_e25974: f64 = (0.5 * assign25220_e25973);
        let assign25220_e25977: f64 = (-locals.var_sp_ov_xbar);
        let assign25220_e25978: f64 = (-assign25220_e25977);
        let assign25220_e25980: f64 = (assign25220_e25978 - 80.0);
        let assign25220_e25982: f64 = (assign25220_e25980 * 0.3333333333333);
        let assign25220_e25983: f64 = (1.0 + assign25220_e25982);
        let assign25220_e25984: f64 = (assign25220_e25974 * assign25220_e25983);
        let assign25220_e25985: f64 = (1.0 + assign25220_e25984);
        let assign25220_e25986: f64 = (assign25220_e25966 * assign25220_e25985);
        let assign25220_e25987: f64 = (1.0 + assign25220_e25986);
        let assign25220_e25988: f64 = (1.80485e-35 / assign25220_e25987);
        (assign25220_e25988, (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn4)) * assign25220_e25985) + (assign25220_e25966 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn4))) * assign25220_e25983) + (assign25220_e25974 * ((-(-locals.var_sp_ov_xbar_dn4)) * 0.3333333333333)))))) / (assign25220_e25987 * assign25220_e25987))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn6)) * assign25220_e25985) + (assign25220_e25966 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn6))) * assign25220_e25983) + (assign25220_e25974 * ((-(-locals.var_sp_ov_xbar_dn6)) * 0.3333333333333)))))) / (assign25220_e25987 * assign25220_e25987))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn7)) * assign25220_e25985) + (assign25220_e25966 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn7))) * assign25220_e25983) + (assign25220_e25974 * ((-(-locals.var_sp_ov_xbar_dn7)) * 0.3333333333333)))))) / (assign25220_e25987 * assign25220_e25987))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn8)) * assign25220_e25985) + (assign25220_e25966 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn8))) * assign25220_e25983) + (assign25220_e25974 * ((-(-locals.var_sp_ov_xbar_dn8)) * 0.3333333333333)))))) / (assign25220_e25987 * assign25220_e25987))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn9)) * assign25220_e25985) + (assign25220_e25966 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn9))) * assign25220_e25983) + (assign25220_e25974 * ((-(-locals.var_sp_ov_xbar_dn9)) * 0.3333333333333)))))) / (assign25220_e25987 * assign25220_e25987))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign25220_e25990;
        locals.var_sp_ov_temp_dn4 = assign25220_e25990_d_n4;
        locals.var_sp_ov_temp_dn6 = assign25220_e25990_d_n6;
        locals.var_sp_ov_temp_dn7 = assign25220_e25990_d_n7;
        locals.var_sp_ov_temp_dn8 = assign25220_e25990_d_n8;
        locals.var_sp_ov_temp_dn9 = assign25220_e25990_d_n9;

        let (assign25230_e26031, assign25230_e26031_d_n4, assign25230_e26031_d_n6, assign25230_e26031_d_n7, assign25230_e26031_d_n8, assign25230_e26031_d_n9,) = {
    if (((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard717 == 0.0)) && (locals.var_guard718 == 0.0)) {
        let assign25230_e26007: f64 = (-locals.var_sp_ov_xbar);
        let assign25230_e26009: f64 = (assign25230_e26007 - 80.0);
        let assign25230_e26013: f64 = (-locals.var_sp_ov_xbar);
        let assign25230_e26015: f64 = (assign25230_e26013 - 80.0);
        let assign25230_e26016: f64 = (0.5 * assign25230_e26015);
        let assign25230_e26019: f64 = (-locals.var_sp_ov_xbar);
        let assign25230_e26021: f64 = (assign25230_e26019 - 80.0);
        let assign25230_e26023: f64 = (assign25230_e26021 * 0.3333333333333);
        let assign25230_e26024: f64 = (1.0 + assign25230_e26023);
        let assign25230_e26025: f64 = (assign25230_e26016 * assign25230_e26024);
        let assign25230_e26026: f64 = (1.0 + assign25230_e26025);
        let assign25230_e26027: f64 = (assign25230_e26009 * assign25230_e26026);
        let assign25230_e26028: f64 = (1.0 + assign25230_e26027);
        let assign25230_e26029: f64 = (5.54062e34 * assign25230_e26028);
        (assign25230_e26029, (5.54062e34 * (((-locals.var_sp_ov_xbar_dn4) * assign25230_e26026) + (assign25230_e26009 * (((0.5 * (-locals.var_sp_ov_xbar_dn4)) * assign25230_e26024) + (assign25230_e26016 * ((-locals.var_sp_ov_xbar_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn6) * assign25230_e26026) + (assign25230_e26009 * (((0.5 * (-locals.var_sp_ov_xbar_dn6)) * assign25230_e26024) + (assign25230_e26016 * ((-locals.var_sp_ov_xbar_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn7) * assign25230_e26026) + (assign25230_e26009 * (((0.5 * (-locals.var_sp_ov_xbar_dn7)) * assign25230_e26024) + (assign25230_e26016 * ((-locals.var_sp_ov_xbar_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn8) * assign25230_e26026) + (assign25230_e26009 * (((0.5 * (-locals.var_sp_ov_xbar_dn8)) * assign25230_e26024) + (assign25230_e26016 * ((-locals.var_sp_ov_xbar_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn9) * assign25230_e26026) + (assign25230_e26009 * (((0.5 * (-locals.var_sp_ov_xbar_dn9)) * assign25230_e26024) + (assign25230_e26016 * ((-locals.var_sp_ov_xbar_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign25230_e26031;
        locals.var_sp_ov_temp_dn4 = assign25230_e26031_d_n4;
        locals.var_sp_ov_temp_dn6 = assign25230_e26031_d_n6;
        locals.var_sp_ov_temp_dn7 = assign25230_e26031_d_n7;
        locals.var_sp_ov_temp_dn8 = assign25230_e26031_d_n8;
        locals.var_sp_ov_temp_dn9 = assign25230_e26031_d_n9;

        let (assign25240_e26043, assign25240_e26043_d_n4, assign25240_e26043_d_n6, assign25240_e26043_d_n7, assign25240_e26043_d_n8, assign25240_e26043_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25240_e26041: f64 = (1.0 - locals.var_sp_ov_temp);
        (assign25240_e26041, (-locals.var_sp_ov_temp_dn4), (-locals.var_sp_ov_temp_dn6), (-locals.var_sp_ov_temp_dn7), (-locals.var_sp_ov_temp_dn8), (-locals.var_sp_ov_temp_dn9),)
    } else {
        (locals.var_sp_ov_w, locals.var_sp_ov_w_dn4, locals.var_sp_ov_w_dn6, locals.var_sp_ov_w_dn7, locals.var_sp_ov_w_dn8, locals.var_sp_ov_w_dn9,)
    }
};
        locals.var_sp_ov_w = assign25240_e26043;
        locals.var_sp_ov_w_dn4 = assign25240_e26043_d_n4;
        locals.var_sp_ov_w_dn6 = assign25240_e26043_d_n6;
        locals.var_sp_ov_w_dn7 = assign25240_e26043_d_n7;
        locals.var_sp_ov_w_dn8 = assign25240_e26043_d_n8;
        locals.var_sp_ov_w_dn9 = assign25240_e26043_d_n9;

        let (assign25250_e26068, assign25250_e26068_d_n4, assign25250_e26068_d_n6, assign25250_e26068_d_n7, assign25250_e26068_d_n8, assign25250_e26068_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25250_e26054: f64 = (locals.var_gov2 * 0.5);
        let assign25250_e26055: f64 = (locals.var_xgd_ovcv + assign25250_e26054);
        let assign25250_e26060: f64 = (locals.var_gov2 * 0.25);
        let assign25250_e26061: f64 = (locals.var_xgd_ovcv + assign25250_e26060);
        let assign25250_e26063: f64 = (assign25250_e26061 - locals.var_sp_ov_w);
        let assign25250_e26064: f64 = (assign25250_e26063).sqrt();
        let assign25250_e26065: f64 = (locals.var_gov * assign25250_e26064);
        let assign25250_e26066: f64 = (assign25250_e26055 - assign25250_e26065);
        (assign25250_e26066, ((locals.var_xgd_ovcv_dn4 + (locals.var_gov2_dn4 * 0.5)) - ((locals.var_gov_dn4 * assign25250_e26064) + (locals.var_gov * (((locals.var_xgd_ovcv_dn4 + (locals.var_gov2_dn4 * 0.25)) - locals.var_sp_ov_w_dn4) / (2.0 * assign25250_e26064))))), ((locals.var_xgd_ovcv_dn6 + (locals.var_gov2_dn6 * 0.5)) - ((locals.var_gov_dn6 * assign25250_e26064) + (locals.var_gov * (((locals.var_xgd_ovcv_dn6 + (locals.var_gov2_dn6 * 0.25)) - locals.var_sp_ov_w_dn6) / (2.0 * assign25250_e26064))))), ((locals.var_xgd_ovcv_dn7 + (locals.var_gov2_dn7 * 0.5)) - ((locals.var_gov_dn7 * assign25250_e26064) + (locals.var_gov * (((locals.var_xgd_ovcv_dn7 + (locals.var_gov2_dn7 * 0.25)) - locals.var_sp_ov_w_dn7) / (2.0 * assign25250_e26064))))), ((locals.var_xgd_ovcv_dn8 + (locals.var_gov2_dn8 * 0.5)) - ((locals.var_gov_dn8 * assign25250_e26064) + (locals.var_gov * (((locals.var_xgd_ovcv_dn8 + (locals.var_gov2_dn8 * 0.25)) - locals.var_sp_ov_w_dn8) / (2.0 * assign25250_e26064))))), ((locals.var_xgd_ovcv_dn9 + (locals.var_gov2_dn9 * 0.5)) - ((locals.var_gov_dn9 * assign25250_e26064) + (locals.var_gov * (((locals.var_xgd_ovcv_dn9 + (locals.var_gov2_dn9 * 0.25)) - locals.var_sp_ov_w_dn9) / (2.0 * assign25250_e26064))))),)
    } else {
        (locals.var_sp_ov_x0, locals.var_sp_ov_x0_dn4, locals.var_sp_ov_x0_dn6, locals.var_sp_ov_x0_dn7, locals.var_sp_ov_x0_dn8, locals.var_sp_ov_x0_dn9,)
    }
};
        locals.var_sp_ov_x0 = assign25250_e26068;
        locals.var_sp_ov_x0_dn4 = assign25250_e26068_d_n4;
        locals.var_sp_ov_x0_dn6 = assign25250_e26068_d_n6;
        locals.var_sp_ov_x0_dn7 = assign25250_e26068_d_n7;
        locals.var_sp_ov_x0_dn8 = assign25250_e26068_d_n8;
        locals.var_sp_ov_x0_dn9 = assign25250_e26068_d_n9;

        let assign25260_e26070: f64 = (-locals.var_sp_ov_x0);
        let assign25260_e26071: f64 = (assign25260_e26070).abs();
        let assign25260_e26073: f64 = if assign25260_e26071 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard719 = assign25260_e26073;

        let (assign25270_e26087, assign25270_e26087_d_n4, assign25270_e26087_d_n6, assign25270_e26087_d_n7, assign25270_e26087_d_n8, assign25270_e26087_d_n9,) = {
    if ((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard719 != 0.0)) {
        let assign25270_e26084: f64 = (-locals.var_sp_ov_x0);
        let assign25270_e26085: f64 = (assign25270_e26084).exp();
        (assign25270_e26085, (assign25270_e26085 * (-locals.var_sp_ov_x0_dn4)), (assign25270_e26085 * (-locals.var_sp_ov_x0_dn6)), (assign25270_e26085 * (-locals.var_sp_ov_x0_dn7)), (assign25270_e26085 * (-locals.var_sp_ov_x0_dn8)), (assign25270_e26085 * (-locals.var_sp_ov_x0_dn9)),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign25270_e26087;
        locals.var_sp_ov_d0_dn4 = assign25270_e26087_d_n4;
        locals.var_sp_ov_d0_dn6 = assign25270_e26087_d_n6;
        locals.var_sp_ov_d0_dn7 = assign25270_e26087_d_n7;
        locals.var_sp_ov_d0_dn8 = assign25270_e26087_d_n8;
        locals.var_sp_ov_d0_dn9 = assign25270_e26087_d_n9;

        let assign25280_e26089: f64 = (-locals.var_sp_ov_x0);
        let assign25280_e26091: f64 = (-80.0);
        let assign25280_e26092: f64 = if assign25280_e26089 < assign25280_e26091 { 1.0 } else { 0.0 };
        locals.var_guard720 = assign25280_e26092;

        let (assign25290_e26135, assign25290_e26135_d_n4, assign25290_e26135_d_n6, assign25290_e26135_d_n7, assign25290_e26135_d_n8, assign25290_e26135_d_n9,) = {
    if (((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard719 == 0.0)) && (locals.var_guard720 != 0.0)) {
        let assign25290_e26108: f64 = (-locals.var_sp_ov_x0);
        let assign25290_e26109: f64 = (-assign25290_e26108);
        let assign25290_e26111: f64 = (assign25290_e26109 - 80.0);
        let assign25290_e26115: f64 = (-locals.var_sp_ov_x0);
        let assign25290_e26116: f64 = (-assign25290_e26115);
        let assign25290_e26118: f64 = (assign25290_e26116 - 80.0);
        let assign25290_e26119: f64 = (0.5 * assign25290_e26118);
        let assign25290_e26122: f64 = (-locals.var_sp_ov_x0);
        let assign25290_e26123: f64 = (-assign25290_e26122);
        let assign25290_e26125: f64 = (assign25290_e26123 - 80.0);
        let assign25290_e26127: f64 = (assign25290_e26125 * 0.3333333333333);
        let assign25290_e26128: f64 = (1.0 + assign25290_e26127);
        let assign25290_e26129: f64 = (assign25290_e26119 * assign25290_e26128);
        let assign25290_e26130: f64 = (1.0 + assign25290_e26129);
        let assign25290_e26131: f64 = (assign25290_e26111 * assign25290_e26130);
        let assign25290_e26132: f64 = (1.0 + assign25290_e26131);
        let assign25290_e26133: f64 = (1.80485e-35 / assign25290_e26132);
        (assign25290_e26133, (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn4)) * assign25290_e26130) + (assign25290_e26111 * (((0.5 * (-(-locals.var_sp_ov_x0_dn4))) * assign25290_e26128) + (assign25290_e26119 * ((-(-locals.var_sp_ov_x0_dn4)) * 0.3333333333333)))))) / (assign25290_e26132 * assign25290_e26132))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn6)) * assign25290_e26130) + (assign25290_e26111 * (((0.5 * (-(-locals.var_sp_ov_x0_dn6))) * assign25290_e26128) + (assign25290_e26119 * ((-(-locals.var_sp_ov_x0_dn6)) * 0.3333333333333)))))) / (assign25290_e26132 * assign25290_e26132))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn7)) * assign25290_e26130) + (assign25290_e26111 * (((0.5 * (-(-locals.var_sp_ov_x0_dn7))) * assign25290_e26128) + (assign25290_e26119 * ((-(-locals.var_sp_ov_x0_dn7)) * 0.3333333333333)))))) / (assign25290_e26132 * assign25290_e26132))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn8)) * assign25290_e26130) + (assign25290_e26111 * (((0.5 * (-(-locals.var_sp_ov_x0_dn8))) * assign25290_e26128) + (assign25290_e26119 * ((-(-locals.var_sp_ov_x0_dn8)) * 0.3333333333333)))))) / (assign25290_e26132 * assign25290_e26132))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn9)) * assign25290_e26130) + (assign25290_e26111 * (((0.5 * (-(-locals.var_sp_ov_x0_dn9))) * assign25290_e26128) + (assign25290_e26119 * ((-(-locals.var_sp_ov_x0_dn9)) * 0.3333333333333)))))) / (assign25290_e26132 * assign25290_e26132))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign25290_e26135;
        locals.var_sp_ov_d0_dn4 = assign25290_e26135_d_n4;
        locals.var_sp_ov_d0_dn6 = assign25290_e26135_d_n6;
        locals.var_sp_ov_d0_dn7 = assign25290_e26135_d_n7;
        locals.var_sp_ov_d0_dn8 = assign25290_e26135_d_n8;
        locals.var_sp_ov_d0_dn9 = assign25290_e26135_d_n9;

        let (assign25300_e26176, assign25300_e26176_d_n4, assign25300_e26176_d_n6, assign25300_e26176_d_n7, assign25300_e26176_d_n8, assign25300_e26176_d_n9,) = {
    if (((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard719 == 0.0)) && (locals.var_guard720 == 0.0)) {
        let assign25300_e26152: f64 = (-locals.var_sp_ov_x0);
        let assign25300_e26154: f64 = (assign25300_e26152 - 80.0);
        let assign25300_e26158: f64 = (-locals.var_sp_ov_x0);
        let assign25300_e26160: f64 = (assign25300_e26158 - 80.0);
        let assign25300_e26161: f64 = (0.5 * assign25300_e26160);
        let assign25300_e26164: f64 = (-locals.var_sp_ov_x0);
        let assign25300_e26166: f64 = (assign25300_e26164 - 80.0);
        let assign25300_e26168: f64 = (assign25300_e26166 * 0.3333333333333);
        let assign25300_e26169: f64 = (1.0 + assign25300_e26168);
        let assign25300_e26170: f64 = (assign25300_e26161 * assign25300_e26169);
        let assign25300_e26171: f64 = (1.0 + assign25300_e26170);
        let assign25300_e26172: f64 = (assign25300_e26154 * assign25300_e26171);
        let assign25300_e26173: f64 = (1.0 + assign25300_e26172);
        let assign25300_e26174: f64 = (5.54062e34 * assign25300_e26173);
        (assign25300_e26174, (5.54062e34 * (((-locals.var_sp_ov_x0_dn4) * assign25300_e26171) + (assign25300_e26154 * (((0.5 * (-locals.var_sp_ov_x0_dn4)) * assign25300_e26169) + (assign25300_e26161 * ((-locals.var_sp_ov_x0_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn6) * assign25300_e26171) + (assign25300_e26154 * (((0.5 * (-locals.var_sp_ov_x0_dn6)) * assign25300_e26169) + (assign25300_e26161 * ((-locals.var_sp_ov_x0_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn7) * assign25300_e26171) + (assign25300_e26154 * (((0.5 * (-locals.var_sp_ov_x0_dn7)) * assign25300_e26169) + (assign25300_e26161 * ((-locals.var_sp_ov_x0_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn8) * assign25300_e26171) + (assign25300_e26154 * (((0.5 * (-locals.var_sp_ov_x0_dn8)) * assign25300_e26169) + (assign25300_e26161 * ((-locals.var_sp_ov_x0_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn9) * assign25300_e26171) + (assign25300_e26154 * (((0.5 * (-locals.var_sp_ov_x0_dn9)) * assign25300_e26169) + (assign25300_e26161 * ((-locals.var_sp_ov_x0_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign25300_e26176;
        locals.var_sp_ov_d0_dn4 = assign25300_e26176_d_n4;
        locals.var_sp_ov_d0_dn6 = assign25300_e26176_d_n6;
        locals.var_sp_ov_d0_dn7 = assign25300_e26176_d_n7;
        locals.var_sp_ov_d0_dn8 = assign25300_e26176_d_n8;
        locals.var_sp_ov_d0_dn9 = assign25300_e26176_d_n9;

    }

    pub(super) fn stamp_transient_block_65(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25310_e26196, assign25310_e26196_d_n4, assign25310_e26196_d_n6, assign25310_e26196_d_n7, assign25310_e26196_d_n8, assign25310_e26196_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25310_e26187: f64 = (locals.var_xgd_ovcv - locals.var_sp_ov_x0);
        let assign25310_e26188: f64 = (2.0 * assign25310_e26187);
        let assign25310_e26192: f64 = (1.0 - locals.var_sp_ov_d0);
        let assign25310_e26193: f64 = (locals.var_gov2 * assign25310_e26192);
        let assign25310_e26194: f64 = (assign25310_e26188 + assign25310_e26193);
        (assign25310_e26194, ((2.0 * (locals.var_xgd_ovcv_dn4 - locals.var_sp_ov_x0_dn4)) + ((locals.var_gov2_dn4 * assign25310_e26192) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn4)))), ((2.0 * (locals.var_xgd_ovcv_dn6 - locals.var_sp_ov_x0_dn6)) + ((locals.var_gov2_dn6 * assign25310_e26192) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn6)))), ((2.0 * (locals.var_xgd_ovcv_dn7 - locals.var_sp_ov_x0_dn7)) + ((locals.var_gov2_dn7 * assign25310_e26192) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn7)))), ((2.0 * (locals.var_xgd_ovcv_dn8 - locals.var_sp_ov_x0_dn8)) + ((locals.var_gov2_dn8 * assign25310_e26192) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn8)))), ((2.0 * (locals.var_xgd_ovcv_dn9 - locals.var_sp_ov_x0_dn9)) + ((locals.var_gov2_dn9 * assign25310_e26192) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_p, locals.var_sp_ov_p_dn4, locals.var_sp_ov_p_dn6, locals.var_sp_ov_p_dn7, locals.var_sp_ov_p_dn8, locals.var_sp_ov_p_dn9,)
    }
};
        locals.var_sp_ov_p = assign25310_e26196;
        locals.var_sp_ov_p_dn4 = assign25310_e26196_d_n4;
        locals.var_sp_ov_p_dn6 = assign25310_e26196_d_n6;
        locals.var_sp_ov_p_dn7 = assign25310_e26196_d_n7;
        locals.var_sp_ov_p_dn8 = assign25310_e26196_d_n8;
        locals.var_sp_ov_p_dn9 = assign25310_e26196_d_n9;

        let (assign25320_e26220, assign25320_e26220_d_n4, assign25320_e26220_d_n6, assign25320_e26220_d_n7, assign25320_e26220_d_n8, assign25320_e26220_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25320_e26206: f64 = (locals.var_xgd_ovcv - locals.var_sp_ov_x0);
        let assign25320_e26209: f64 = (locals.var_xgd_ovcv - locals.var_sp_ov_x0);
        let assign25320_e26210: f64 = (assign25320_e26206 * assign25320_e26209);
        let assign25320_e26214: f64 = (locals.var_sp_ov_x0 - 1.0);
        let assign25320_e26216: f64 = (assign25320_e26214 + locals.var_sp_ov_d0);
        let assign25320_e26217: f64 = (locals.var_gov2 * assign25320_e26216);
        let assign25320_e26218: f64 = (assign25320_e26210 - assign25320_e26217);
        (assign25320_e26218, ((((locals.var_xgd_ovcv_dn4 - locals.var_sp_ov_x0_dn4) * assign25320_e26209) + (assign25320_e26206 * (locals.var_xgd_ovcv_dn4 - locals.var_sp_ov_x0_dn4))) - ((locals.var_gov2_dn4 * assign25320_e26216) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn4 + locals.var_sp_ov_d0_dn4)))), ((((locals.var_xgd_ovcv_dn6 - locals.var_sp_ov_x0_dn6) * assign25320_e26209) + (assign25320_e26206 * (locals.var_xgd_ovcv_dn6 - locals.var_sp_ov_x0_dn6))) - ((locals.var_gov2_dn6 * assign25320_e26216) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn6 + locals.var_sp_ov_d0_dn6)))), ((((locals.var_xgd_ovcv_dn7 - locals.var_sp_ov_x0_dn7) * assign25320_e26209) + (assign25320_e26206 * (locals.var_xgd_ovcv_dn7 - locals.var_sp_ov_x0_dn7))) - ((locals.var_gov2_dn7 * assign25320_e26216) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn7 + locals.var_sp_ov_d0_dn7)))), ((((locals.var_xgd_ovcv_dn8 - locals.var_sp_ov_x0_dn8) * assign25320_e26209) + (assign25320_e26206 * (locals.var_xgd_ovcv_dn8 - locals.var_sp_ov_x0_dn8))) - ((locals.var_gov2_dn8 * assign25320_e26216) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn8 + locals.var_sp_ov_d0_dn8)))), ((((locals.var_xgd_ovcv_dn9 - locals.var_sp_ov_x0_dn9) * assign25320_e26209) + (assign25320_e26206 * (locals.var_xgd_ovcv_dn9 - locals.var_sp_ov_x0_dn9))) - ((locals.var_gov2_dn9 * assign25320_e26216) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn9 + locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_q, locals.var_sp_ov_q_dn4, locals.var_sp_ov_q_dn6, locals.var_sp_ov_q_dn7, locals.var_sp_ov_q_dn8, locals.var_sp_ov_q_dn9,)
    }
};
        locals.var_sp_ov_q = assign25320_e26220;
        locals.var_sp_ov_q_dn4 = assign25320_e26220_d_n4;
        locals.var_sp_ov_q_dn6 = assign25320_e26220_d_n6;
        locals.var_sp_ov_q_dn7 = assign25320_e26220_d_n7;
        locals.var_sp_ov_q_dn8 = assign25320_e26220_d_n8;
        locals.var_sp_ov_q_dn9 = assign25320_e26220_d_n9;

        let (assign25330_e26236, assign25330_e26236_d_n4, assign25330_e26236_d_n6, assign25330_e26236_d_n7, assign25330_e26236_d_n8, assign25330_e26236_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25330_e26231: f64 = (locals.var_gov2 * 0.5);
        let assign25330_e26233: f64 = (assign25330_e26231 * locals.var_sp_ov_d0);
        let assign25330_e26234: f64 = (1.0 - assign25330_e26233);
        (assign25330_e26234, (-(((locals.var_gov2_dn4 * 0.5) * locals.var_sp_ov_d0) + (assign25330_e26231 * locals.var_sp_ov_d0_dn4))), (-(((locals.var_gov2_dn6 * 0.5) * locals.var_sp_ov_d0) + (assign25330_e26231 * locals.var_sp_ov_d0_dn6))), (-(((locals.var_gov2_dn7 * 0.5) * locals.var_sp_ov_d0) + (assign25330_e26231 * locals.var_sp_ov_d0_dn7))), (-(((locals.var_gov2_dn8 * 0.5) * locals.var_sp_ov_d0) + (assign25330_e26231 * locals.var_sp_ov_d0_dn8))), (-(((locals.var_gov2_dn9 * 0.5) * locals.var_sp_ov_d0) + (assign25330_e26231 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_xi, locals.var_sp_ov_xi_dn4, locals.var_sp_ov_xi_dn6, locals.var_sp_ov_xi_dn7, locals.var_sp_ov_xi_dn8, locals.var_sp_ov_xi_dn9,)
    }
};
        locals.var_sp_ov_xi = assign25330_e26236;
        locals.var_sp_ov_xi_dn4 = assign25330_e26236_d_n4;
        locals.var_sp_ov_xi_dn6 = assign25330_e26236_d_n6;
        locals.var_sp_ov_xi_dn7 = assign25330_e26236_d_n7;
        locals.var_sp_ov_xi_dn8 = assign25330_e26236_d_n8;
        locals.var_sp_ov_xi_dn9 = assign25330_e26236_d_n9;

        let (assign25340_e26254, assign25340_e26254_d_n4, assign25340_e26254_d_n6, assign25340_e26254_d_n7, assign25340_e26254_d_n8, assign25340_e26254_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25340_e26246: f64 = (locals.var_sp_ov_p * locals.var_sp_ov_p);
        let assign25340_e26250: f64 = (locals.var_sp_ov_xi * locals.var_sp_ov_q);
        let assign25340_e26251: f64 = (4.0 * assign25340_e26250);
        let assign25340_e26252: f64 = (assign25340_e26246 - assign25340_e26251);
        (assign25340_e26252, (((locals.var_sp_ov_p_dn4 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn4)) - (4.0 * ((locals.var_sp_ov_xi_dn4 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn4)))), (((locals.var_sp_ov_p_dn6 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn6)) - (4.0 * ((locals.var_sp_ov_xi_dn6 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn6)))), (((locals.var_sp_ov_p_dn7 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn7)) - (4.0 * ((locals.var_sp_ov_xi_dn7 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn7)))), (((locals.var_sp_ov_p_dn8 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn8)) - (4.0 * ((locals.var_sp_ov_xi_dn8 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn8)))), (((locals.var_sp_ov_p_dn9 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn9)) - (4.0 * ((locals.var_sp_ov_xi_dn9 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign25340_e26254;
        locals.var_sp_ov_temp_dn4 = assign25340_e26254_d_n4;
        locals.var_sp_ov_temp_dn6 = assign25340_e26254_d_n6;
        locals.var_sp_ov_temp_dn7 = assign25340_e26254_d_n7;
        locals.var_sp_ov_temp_dn8 = assign25340_e26254_d_n8;
        locals.var_sp_ov_temp_dn9 = assign25340_e26254_d_n9;

        let (assign25350_e26271, assign25350_e26271_d_n4, assign25350_e26271_d_n6, assign25350_e26271_d_n7, assign25350_e26271_d_n8, assign25350_e26271_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25350_e26264: f64 = (2.0 * locals.var_sp_ov_q);
        let assign25350_e26267: f64 = (locals.var_sp_ov_temp).sqrt();
        let assign25350_e26268: f64 = (locals.var_sp_ov_p + assign25350_e26267);
        let assign25350_e26269: f64 = (assign25350_e26264 / assign25350_e26268);
        (assign25350_e26269, ((((2.0 * locals.var_sp_ov_q_dn4) * assign25350_e26268) - (assign25350_e26264 * (locals.var_sp_ov_p_dn4 + (locals.var_sp_ov_temp_dn4 / (2.0 * assign25350_e26267))))) / (assign25350_e26268 * assign25350_e26268)), ((((2.0 * locals.var_sp_ov_q_dn6) * assign25350_e26268) - (assign25350_e26264 * (locals.var_sp_ov_p_dn6 + (locals.var_sp_ov_temp_dn6 / (2.0 * assign25350_e26267))))) / (assign25350_e26268 * assign25350_e26268)), ((((2.0 * locals.var_sp_ov_q_dn7) * assign25350_e26268) - (assign25350_e26264 * (locals.var_sp_ov_p_dn7 + (locals.var_sp_ov_temp_dn7 / (2.0 * assign25350_e26267))))) / (assign25350_e26268 * assign25350_e26268)), ((((2.0 * locals.var_sp_ov_q_dn8) * assign25350_e26268) - (assign25350_e26264 * (locals.var_sp_ov_p_dn8 + (locals.var_sp_ov_temp_dn8 / (2.0 * assign25350_e26267))))) / (assign25350_e26268 * assign25350_e26268)), ((((2.0 * locals.var_sp_ov_q_dn9) * assign25350_e26268) - (assign25350_e26264 * (locals.var_sp_ov_p_dn9 + (locals.var_sp_ov_temp_dn9 / (2.0 * assign25350_e26267))))) / (assign25350_e26268 * assign25350_e26268)),)
    } else {
        (locals.var_sp_ov_u, locals.var_sp_ov_u_dn4, locals.var_sp_ov_u_dn6, locals.var_sp_ov_u_dn7, locals.var_sp_ov_u_dn8, locals.var_sp_ov_u_dn9,)
    }
};
        locals.var_sp_ov_u = assign25350_e26271;
        locals.var_sp_ov_u_dn4 = assign25350_e26271_d_n4;
        locals.var_sp_ov_u_dn6 = assign25350_e26271_d_n6;
        locals.var_sp_ov_u_dn7 = assign25350_e26271_d_n7;
        locals.var_sp_ov_u_dn8 = assign25350_e26271_d_n8;
        locals.var_sp_ov_u_dn9 = assign25350_e26271_d_n9;

        let (assign25360_e26283, assign25360_e26283_d_n4, assign25360_e26283_d_n6, assign25360_e26283_d_n7, assign25360_e26283_d_n8, assign25360_e26283_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25360_e26281: f64 = (locals.var_sp_ov_x0 + locals.var_sp_ov_u);
        (assign25360_e26281, (locals.var_sp_ov_x0_dn4 + locals.var_sp_ov_u_dn4), (locals.var_sp_ov_x0_dn6 + locals.var_sp_ov_u_dn6), (locals.var_sp_ov_x0_dn7 + locals.var_sp_ov_u_dn7), (locals.var_sp_ov_x0_dn8 + locals.var_sp_ov_u_dn8), (locals.var_sp_ov_x0_dn9 + locals.var_sp_ov_u_dn9),)
    } else {
        (locals.var_xd_ovcv, locals.var_xd_ovcv_dn4, locals.var_xd_ovcv_dn6, locals.var_xd_ovcv_dn7, locals.var_xd_ovcv_dn8, locals.var_xd_ovcv_dn9,)
    }
};
        locals.var_xd_ovcv = assign25360_e26283;
        locals.var_xd_ovcv_dn4 = assign25360_e26283_d_n4;
        locals.var_xd_ovcv_dn6 = assign25360_e26283_d_n6;
        locals.var_xd_ovcv_dn7 = assign25360_e26283_d_n7;
        locals.var_xd_ovcv_dn8 = assign25360_e26283_d_n8;
        locals.var_xd_ovcv_dn9 = assign25360_e26283_d_n9;

        let (assign25370_e26291, assign25370_e26291_d_n4, assign25370_e26291_d_n6, assign25370_e26291_d_n7, assign25370_e26291_d_n8, assign25370_e26291_d_n9,) = {
    if ((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) {
        let assign25370_e26289: f64 = (-locals.var_xd_ovcv);
        (assign25370_e26289, (-locals.var_xd_ovcv_dn4), (-locals.var_xd_ovcv_dn6), (-locals.var_xd_ovcv_dn7), (-locals.var_xd_ovcv_dn8), (-locals.var_xd_ovcv_dn9),)
    } else {
        (locals.var_xd_ovcv, locals.var_xd_ovcv_dn4, locals.var_xd_ovcv_dn6, locals.var_xd_ovcv_dn7, locals.var_xd_ovcv_dn8, locals.var_xd_ovcv_dn9,)
    }
};
        locals.var_xd_ovcv = assign25370_e26291;
        locals.var_xd_ovcv_dn4 = assign25370_e26291_d_n4;
        locals.var_xd_ovcv_dn6 = assign25370_e26291_d_n6;
        locals.var_xd_ovcv_dn7 = assign25370_e26291_d_n7;
        locals.var_xd_ovcv_dn8 = assign25370_e26291_d_n8;
        locals.var_xd_ovcv_dn9 = assign25370_e26291_d_n9;

        let assign25380_e26293: f64 = (-locals.var_phit0);
        let assign25380_e26296: f64 = (locals.var_xgs_ov + locals.var_xs_ov);
        let assign25380_e26297: f64 = (assign25380_e26293 * assign25380_e26296);
        locals.var_vovs = assign25380_e26297;
        locals.var_vovs_dn4 = (((-locals.var_phit0_dn4) * assign25380_e26296) + (assign25380_e26293 * (locals.var_xgs_ov_dn4 + locals.var_xs_ov_dn4)));
        locals.var_vovs_dn6 = (((-locals.var_phit0_dn6) * assign25380_e26296) + (assign25380_e26293 * (locals.var_xgs_ov_dn6 + locals.var_xs_ov_dn6)));
        locals.var_vovs_dn7 = (((-locals.var_phit0_dn7) * assign25380_e26296) + (assign25380_e26293 * (locals.var_xgs_ov_dn7 + locals.var_xs_ov_dn7)));
        locals.var_vovs_dn8 = (((-locals.var_phit0_dn8) * assign25380_e26296) + (assign25380_e26293 * (locals.var_xgs_ov_dn8 + locals.var_xs_ov_dn8)));
        locals.var_vovs_dn9 = (((-locals.var_phit0_dn9) * assign25380_e26296) + (assign25380_e26293 * (locals.var_xgs_ov_dn9 + locals.var_xs_ov_dn9)));

        let assign25390_e26299: f64 = (-locals.var_phit0);
        let assign25390_e26302: f64 = (locals.var_xgd_ov + locals.var_xd_ov);
        let assign25390_e26303: f64 = (assign25390_e26299 * assign25390_e26302);
        locals.var_vovd = assign25390_e26303;
        locals.var_vovd_dn4 = (((-locals.var_phit0_dn4) * assign25390_e26302) + (assign25390_e26299 * (locals.var_xgd_ov_dn4 + locals.var_xd_ov_dn4)));
        locals.var_vovd_dn6 = (((-locals.var_phit0_dn6) * assign25390_e26302) + (assign25390_e26299 * (locals.var_xgd_ov_dn6 + locals.var_xd_ov_dn6)));
        locals.var_vovd_dn7 = (((-locals.var_phit0_dn7) * assign25390_e26302) + (assign25390_e26299 * (locals.var_xgd_ov_dn7 + locals.var_xd_ov_dn7)));
        locals.var_vovd_dn8 = (((-locals.var_phit0_dn8) * assign25390_e26302) + (assign25390_e26299 * (locals.var_xgd_ov_dn8 + locals.var_xd_ov_dn8)));
        locals.var_vovd_dn9 = (((-locals.var_phit0_dn9) * assign25390_e26302) + (assign25390_e26299 * (locals.var_xgd_ov_dn9 + locals.var_xd_ov_dn9)));

        let assign25400_e26305: f64 = (-locals.var_phit0);
        let assign25400_e26308: f64 = (locals.var_xgs_ovcv + locals.var_xs_ovcv);
        let assign25400_e26309: f64 = (assign25400_e26305 * assign25400_e26308);
        locals.var_vovscv = assign25400_e26309;
        locals.var_vovscv_dn4 = (((-locals.var_phit0_dn4) * assign25400_e26308) + (assign25400_e26305 * (locals.var_xgs_ovcv_dn4 + locals.var_xs_ovcv_dn4)));
        locals.var_vovscv_dn6 = (((-locals.var_phit0_dn6) * assign25400_e26308) + (assign25400_e26305 * (locals.var_xgs_ovcv_dn6 + locals.var_xs_ovcv_dn6)));
        locals.var_vovscv_dn7 = (((-locals.var_phit0_dn7) * assign25400_e26308) + (assign25400_e26305 * (locals.var_xgs_ovcv_dn7 + locals.var_xs_ovcv_dn7)));
        locals.var_vovscv_dn8 = (((-locals.var_phit0_dn8) * assign25400_e26308) + (assign25400_e26305 * (locals.var_xgs_ovcv_dn8 + locals.var_xs_ovcv_dn8)));
        locals.var_vovscv_dn9 = (((-locals.var_phit0_dn9) * assign25400_e26308) + (assign25400_e26305 * (locals.var_xgs_ovcv_dn9 + locals.var_xs_ovcv_dn9)));

        let assign25410_e26311: f64 = (-locals.var_phit0);
        let assign25410_e26314: f64 = (locals.var_xgd_ovcv + locals.var_xd_ovcv);
        let assign25410_e26315: f64 = (assign25410_e26311 * assign25410_e26314);
        locals.var_vovdcv = assign25410_e26315;
        locals.var_vovdcv_dn4 = (((-locals.var_phit0_dn4) * assign25410_e26314) + (assign25410_e26311 * (locals.var_xgd_ovcv_dn4 + locals.var_xd_ovcv_dn4)));
        locals.var_vovdcv_dn6 = (((-locals.var_phit0_dn6) * assign25410_e26314) + (assign25410_e26311 * (locals.var_xgd_ovcv_dn6 + locals.var_xd_ovcv_dn6)));
        locals.var_vovdcv_dn7 = (((-locals.var_phit0_dn7) * assign25410_e26314) + (assign25410_e26311 * (locals.var_xgd_ovcv_dn7 + locals.var_xd_ovcv_dn7)));
        locals.var_vovdcv_dn8 = (((-locals.var_phit0_dn8) * assign25410_e26314) + (assign25410_e26311 * (locals.var_xgd_ovcv_dn8 + locals.var_xd_ovcv_dn8)));
        locals.var_vovdcv_dn9 = (((-locals.var_phit0_dn9) * assign25410_e26314) + (assign25410_e26311 * (locals.var_xgd_ovcv_dn9 + locals.var_xd_ovcv_dn9)));

        locals.var_igsov = 0.0;
        locals.var_igsov_dn4 = 0.0;
        locals.var_igsov_dn6 = 0.0;
        locals.var_igsov_dn7 = 0.0;
        locals.var_igsov_dn8 = 0.0;
        locals.var_igsov_dn9 = 0.0;

        locals.var_igdov = 0.0;
        locals.var_igdov_dn4 = 0.0;
        locals.var_igdov_dn6 = 0.0;
        locals.var_igdov_dn7 = 0.0;
        locals.var_igdov_dn8 = 0.0;
        locals.var_igdov_dn9 = 0.0;

        locals.var_igc = 0.0;
        locals.var_igc_dn4 = 0.0;
        locals.var_igc_dn6 = 0.0;
        locals.var_igc_dn7 = 0.0;
        locals.var_igc_dn8 = 0.0;
        locals.var_igc_dn9 = 0.0;

        locals.var_igs = 0.0;
        locals.var_igs_dn4 = 0.0;
        locals.var_igs_dn6 = 0.0;
        locals.var_igs_dn7 = 0.0;
        locals.var_igs_dn8 = 0.0;
        locals.var_igs_dn9 = 0.0;

        locals.var_igd = 0.0;
        locals.var_igd_dn4 = 0.0;
        locals.var_igd_dn6 = 0.0;
        locals.var_igd_dn7 = 0.0;
        locals.var_igd_dn8 = 0.0;
        locals.var_igd_dn9 = 0.0;

        locals.var_igcs = 0.0;
        locals.var_igcs_dn4 = 0.0;
        locals.var_igcs_dn6 = 0.0;
        locals.var_igcs_dn7 = 0.0;
        locals.var_igcs_dn8 = 0.0;
        locals.var_igcs_dn9 = 0.0;

        locals.var_igcd = 0.0;
        locals.var_igcd_dn4 = 0.0;
        locals.var_igcd_dn6 = 0.0;
        locals.var_igcd_dn7 = 0.0;
        locals.var_igcd_dn8 = 0.0;
        locals.var_igcd_dn9 = 0.0;

        let assign25490_e26325: f64 = if p.p3 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard721 = assign25490_e26325;

        let assign25500_e26332: f64 = if ((locals.var_igovinv_i > 0.0) || (locals.var_igovacc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard722 = assign25500_e26332;

        let (assign25510_e26340, assign25510_e26340_d_n4, assign25510_e26340_d_n6, assign25510_e26340_d_n7, assign25510_e26340_d_n8, assign25510_e26340_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25510_e26338: f64 = (locals.var_vovs + locals.var_dov);
        (assign25510_e26338, (locals.var_vovs_dn4 + locals.var_dov_dn4), (locals.var_vovs_dn6 + locals.var_dov_dn6), (locals.var_vovs_dn7 + locals.var_dov_dn7), (locals.var_vovs_dn8 + locals.var_dov_dn8), (locals.var_vovs_dn9 + locals.var_dov_dn9),)
    } else {
        (locals.var_arg2mina, locals.var_arg2mina_dn4, locals.var_arg2mina_dn6, locals.var_arg2mina_dn7, locals.var_arg2mina_dn8, locals.var_arg2mina_dn9,)
    }
};
        locals.var_arg2mina = assign25510_e26340;
        locals.var_arg2mina_dn4 = assign25510_e26340_d_n4;
        locals.var_arg2mina_dn6 = assign25510_e26340_d_n6;
        locals.var_arg2mina_dn7 = assign25510_e26340_d_n7;
        locals.var_arg2mina_dn8 = assign25510_e26340_d_n8;
        locals.var_arg2mina_dn9 = assign25510_e26340_d_n9;

        let (assign25520_e26361, assign25520_e26361_d_n4, assign25520_e26361_d_n6, assign25520_e26361_d_n7, assign25520_e26361_d_n8, assign25520_e26361_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25520_e26347: f64 = locals.var_arg2mina;
        let assign25520_e26350: f64 = (-locals.var_arg2mina);
        let assign25520_e26353: f64 = (-locals.var_arg2mina);
        let assign25520_e26354: f64 = (assign25520_e26350 * assign25520_e26353);
        let assign25520_e26356: f64 = (assign25520_e26354 + 0.01);
        let assign25520_e26357: f64 = (assign25520_e26356).sqrt();
        let assign25520_e26358: f64 = (assign25520_e26347 - assign25520_e26357);
        let assign25520_e26359: f64 = (0.5 * assign25520_e26358);
        (assign25520_e26359, (0.5 * (locals.var_arg2mina_dn4 - ((((-locals.var_arg2mina_dn4) * assign25520_e26353) + (assign25520_e26350 * (-locals.var_arg2mina_dn4))) / (2.0 * assign25520_e26357)))), (0.5 * (locals.var_arg2mina_dn6 - ((((-locals.var_arg2mina_dn6) * assign25520_e26353) + (assign25520_e26350 * (-locals.var_arg2mina_dn6))) / (2.0 * assign25520_e26357)))), (0.5 * (locals.var_arg2mina_dn7 - ((((-locals.var_arg2mina_dn7) * assign25520_e26353) + (assign25520_e26350 * (-locals.var_arg2mina_dn7))) / (2.0 * assign25520_e26357)))), (0.5 * (locals.var_arg2mina_dn8 - ((((-locals.var_arg2mina_dn8) * assign25520_e26353) + (assign25520_e26350 * (-locals.var_arg2mina_dn8))) / (2.0 * assign25520_e26357)))), (0.5 * (locals.var_arg2mina_dn9 - ((((-locals.var_arg2mina_dn9) * assign25520_e26353) + (assign25520_e26350 * (-locals.var_arg2mina_dn9))) / (2.0 * assign25520_e26357)))),)
    } else {
        (locals.var_psi_t, locals.var_psi_t_dn4, locals.var_psi_t_dn6, locals.var_psi_t_dn7, locals.var_psi_t_dn8, locals.var_psi_t_dn9,)
    }
};
        locals.var_psi_t = assign25520_e26361;
        locals.var_psi_t_dn4 = assign25520_e26361_d_n4;
        locals.var_psi_t_dn6 = assign25520_e26361_d_n6;
        locals.var_psi_t_dn7 = assign25520_e26361_d_n7;
        locals.var_psi_t_dn8 = assign25520_e26361_d_n8;
        locals.var_psi_t_dn9 = assign25520_e26361_d_n9;

        let (assign25530_e26374, assign25530_e26374_d_n4, assign25530_e26374_d_n6, assign25530_e26374_d_n7, assign25530_e26374_d_n8, assign25530_e26374_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25530_e26367: f64 = (locals.var_vovs * locals.var_vovs);
        let assign25530_e26369: f64 = (assign25530_e26367 + 0.0001);
        let assign25530_e26370: f64 = (assign25530_e26369).sqrt();
        let assign25530_e26372: f64 = (assign25530_e26370 * locals.var_inv_chib);
        (assign25530_e26372, ((((locals.var_vovs_dn4 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn4)) / (2.0 * assign25530_e26370)) * locals.var_inv_chib), ((((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) / (2.0 * assign25530_e26370)) * locals.var_inv_chib), ((((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) / (2.0 * assign25530_e26370)) * locals.var_inv_chib), ((((locals.var_vovs_dn8 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn8)) / (2.0 * assign25530_e26370)) * locals.var_inv_chib), ((((locals.var_vovs_dn9 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn9)) / (2.0 * assign25530_e26370)) * locals.var_inv_chib),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign25530_e26374;
        locals.var_zg_dn4 = assign25530_e26374_d_n4;
        locals.var_zg_dn6 = assign25530_e26374_d_n6;
        locals.var_zg_dn7 = assign25530_e26374_d_n7;
        locals.var_zg_dn8 = assign25530_e26374_d_n8;
        locals.var_zg_dn9 = assign25530_e26374_d_n9;

        let assign25540_e26377: f64 = (0.5 * locals.var_xgs_ov);
        let assign25540_e26378: f64 = (assign25540_e26377).abs();
        let assign25540_e26380: f64 = if assign25540_e26378 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard723 = assign25540_e26380;

        let (assign25550_e26391, assign25550_e26391_d_n4, assign25550_e26391_d_n6, assign25550_e26391_d_n7, assign25550_e26391_d_n8, assign25550_e26391_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard723 != 0.0)) {
        let assign25550_e26388: f64 = (0.5 * locals.var_xgs_ov);
        let assign25550_e26389: f64 = (assign25550_e26388).exp();
        (assign25550_e26389, (assign25550_e26389 * (0.5 * locals.var_xgs_ov_dn4)), (assign25550_e26389 * (0.5 * locals.var_xgs_ov_dn6)), (assign25550_e26389 * (0.5 * locals.var_xgs_ov_dn7)), (assign25550_e26389 * (0.5 * locals.var_xgs_ov_dn8)), (assign25550_e26389 * (0.5 * locals.var_xgs_ov_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign25550_e26391;
        locals.var_temp_dn4 = assign25550_e26391_d_n4;
        locals.var_temp_dn6 = assign25550_e26391_d_n6;
        locals.var_temp_dn7 = assign25550_e26391_d_n7;
        locals.var_temp_dn8 = assign25550_e26391_d_n8;
        locals.var_temp_dn9 = assign25550_e26391_d_n9;

        let assign25560_e26394: f64 = (0.5 * locals.var_xgs_ov);
        let assign25560_e26396: f64 = (-80.0);
        let assign25560_e26397: f64 = if assign25560_e26394 < assign25560_e26396 { 1.0 } else { 0.0 };
        locals.var_guard724 = assign25560_e26397;

        let (assign25570_e26439, assign25570_e26439_d_n4, assign25570_e26439_d_n6, assign25570_e26439_d_n7, assign25570_e26439_d_n8, assign25570_e26439_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard723 == 0.0)) && (locals.var_guard724 != 0.0)) {
        let assign25570_e26410: f64 = (0.5 * locals.var_xgs_ov);
        let assign25570_e26411: f64 = (-assign25570_e26410);
        let assign25570_e26413: f64 = (assign25570_e26411 - 80.0);
        let assign25570_e26418: f64 = (0.5 * locals.var_xgs_ov);
        let assign25570_e26419: f64 = (-assign25570_e26418);
        let assign25570_e26421: f64 = (assign25570_e26419 - 80.0);
        let assign25570_e26422: f64 = (0.5 * assign25570_e26421);
        let assign25570_e26426: f64 = (0.5 * locals.var_xgs_ov);
        let assign25570_e26427: f64 = (-assign25570_e26426);
        let assign25570_e26429: f64 = (assign25570_e26427 - 80.0);
        let assign25570_e26431: f64 = (assign25570_e26429 * 0.3333333333333);
        let assign25570_e26432: f64 = (1.0 + assign25570_e26431);
        let assign25570_e26433: f64 = (assign25570_e26422 * assign25570_e26432);
        let assign25570_e26434: f64 = (1.0 + assign25570_e26433);
        let assign25570_e26435: f64 = (assign25570_e26413 * assign25570_e26434);
        let assign25570_e26436: f64 = (1.0 + assign25570_e26435);
        let assign25570_e26437: f64 = (1.80485e-35 / assign25570_e26436);
        (assign25570_e26437, (-((1.80485e-35 * (((-(0.5 * locals.var_xgs_ov_dn4)) * assign25570_e26434) + (assign25570_e26413 * (((0.5 * (-(0.5 * locals.var_xgs_ov_dn4))) * assign25570_e26432) + (assign25570_e26422 * ((-(0.5 * locals.var_xgs_ov_dn4)) * 0.3333333333333)))))) / (assign25570_e26436 * assign25570_e26436))), (-((1.80485e-35 * (((-(0.5 * locals.var_xgs_ov_dn6)) * assign25570_e26434) + (assign25570_e26413 * (((0.5 * (-(0.5 * locals.var_xgs_ov_dn6))) * assign25570_e26432) + (assign25570_e26422 * ((-(0.5 * locals.var_xgs_ov_dn6)) * 0.3333333333333)))))) / (assign25570_e26436 * assign25570_e26436))), (-((1.80485e-35 * (((-(0.5 * locals.var_xgs_ov_dn7)) * assign25570_e26434) + (assign25570_e26413 * (((0.5 * (-(0.5 * locals.var_xgs_ov_dn7))) * assign25570_e26432) + (assign25570_e26422 * ((-(0.5 * locals.var_xgs_ov_dn7)) * 0.3333333333333)))))) / (assign25570_e26436 * assign25570_e26436))), (-((1.80485e-35 * (((-(0.5 * locals.var_xgs_ov_dn8)) * assign25570_e26434) + (assign25570_e26413 * (((0.5 * (-(0.5 * locals.var_xgs_ov_dn8))) * assign25570_e26432) + (assign25570_e26422 * ((-(0.5 * locals.var_xgs_ov_dn8)) * 0.3333333333333)))))) / (assign25570_e26436 * assign25570_e26436))), (-((1.80485e-35 * (((-(0.5 * locals.var_xgs_ov_dn9)) * assign25570_e26434) + (assign25570_e26413 * (((0.5 * (-(0.5 * locals.var_xgs_ov_dn9))) * assign25570_e26432) + (assign25570_e26422 * ((-(0.5 * locals.var_xgs_ov_dn9)) * 0.3333333333333)))))) / (assign25570_e26436 * assign25570_e26436))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign25570_e26439;
        locals.var_temp_dn4 = assign25570_e26439_d_n4;
        locals.var_temp_dn6 = assign25570_e26439_d_n6;
        locals.var_temp_dn7 = assign25570_e26439_d_n7;
        locals.var_temp_dn8 = assign25570_e26439_d_n8;
        locals.var_temp_dn9 = assign25570_e26439_d_n9;

        let (assign25580_e26479, assign25580_e26479_d_n4, assign25580_e26479_d_n6, assign25580_e26479_d_n7, assign25580_e26479_d_n8, assign25580_e26479_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard723 == 0.0)) && (locals.var_guard724 == 0.0)) {
        let assign25580_e26453: f64 = (0.5 * locals.var_xgs_ov);
        let assign25580_e26455: f64 = (assign25580_e26453 - 80.0);
        let assign25580_e26460: f64 = (0.5 * locals.var_xgs_ov);
        let assign25580_e26462: f64 = (assign25580_e26460 - 80.0);
        let assign25580_e26463: f64 = (0.5 * assign25580_e26462);
        let assign25580_e26467: f64 = (0.5 * locals.var_xgs_ov);
        let assign25580_e26469: f64 = (assign25580_e26467 - 80.0);
        let assign25580_e26471: f64 = (assign25580_e26469 * 0.3333333333333);
        let assign25580_e26472: f64 = (1.0 + assign25580_e26471);
        let assign25580_e26473: f64 = (assign25580_e26463 * assign25580_e26472);
        let assign25580_e26474: f64 = (1.0 + assign25580_e26473);
        let assign25580_e26475: f64 = (assign25580_e26455 * assign25580_e26474);
        let assign25580_e26476: f64 = (1.0 + assign25580_e26475);
        let assign25580_e26477: f64 = (5.54062e34 * assign25580_e26476);
        (assign25580_e26477, (5.54062e34 * (((0.5 * locals.var_xgs_ov_dn4) * assign25580_e26474) + (assign25580_e26455 * (((0.5 * (0.5 * locals.var_xgs_ov_dn4)) * assign25580_e26472) + (assign25580_e26463 * ((0.5 * locals.var_xgs_ov_dn4) * 0.3333333333333)))))), (5.54062e34 * (((0.5 * locals.var_xgs_ov_dn6) * assign25580_e26474) + (assign25580_e26455 * (((0.5 * (0.5 * locals.var_xgs_ov_dn6)) * assign25580_e26472) + (assign25580_e26463 * ((0.5 * locals.var_xgs_ov_dn6) * 0.3333333333333)))))), (5.54062e34 * (((0.5 * locals.var_xgs_ov_dn7) * assign25580_e26474) + (assign25580_e26455 * (((0.5 * (0.5 * locals.var_xgs_ov_dn7)) * assign25580_e26472) + (assign25580_e26463 * ((0.5 * locals.var_xgs_ov_dn7) * 0.3333333333333)))))), (5.54062e34 * (((0.5 * locals.var_xgs_ov_dn8) * assign25580_e26474) + (assign25580_e26455 * (((0.5 * (0.5 * locals.var_xgs_ov_dn8)) * assign25580_e26472) + (assign25580_e26463 * ((0.5 * locals.var_xgs_ov_dn8) * 0.3333333333333)))))), (5.54062e34 * (((0.5 * locals.var_xgs_ov_dn9) * assign25580_e26474) + (assign25580_e26455 * (((0.5 * (0.5 * locals.var_xgs_ov_dn9)) * assign25580_e26472) + (assign25580_e26463 * ((0.5 * locals.var_xgs_ov_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign25580_e26479;
        locals.var_temp_dn4 = assign25580_e26479_d_n4;
        locals.var_temp_dn6 = assign25580_e26479_d_n6;
        locals.var_temp_dn7 = assign25580_e26479_d_n7;
        locals.var_temp_dn8 = assign25580_e26479_d_n8;
        locals.var_temp_dn9 = assign25580_e26479_d_n9;

        let (assign25590_e26489, assign25590_e26489_d_n4, assign25590_e26489_d_n6, assign25590_e26489_d_n7, assign25590_e26489_d_n8, assign25590_e26489_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25590_e26486: f64 = (1.0 + locals.var_temp);
        let assign25590_e26487: f64 = (1.0 / assign25590_e26486);
        (assign25590_e26487, (-(locals.var_temp_dn4 / (assign25590_e26486 * assign25590_e26486))), (-(locals.var_temp_dn6 / (assign25590_e26486 * assign25590_e26486))), (-(locals.var_temp_dn7 / (assign25590_e26486 * assign25590_e26486))), (-(locals.var_temp_dn8 / (assign25590_e26486 * assign25590_e26486))), (-(locals.var_temp_dn9 / (assign25590_e26486 * assign25590_e26486))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign25590_e26489;
        locals.var_temp1_dn4 = assign25590_e26489_d_n4;
        locals.var_temp1_dn6 = assign25590_e26489_d_n6;
        locals.var_temp1_dn7 = assign25590_e26489_d_n7;
        locals.var_temp1_dn8 = assign25590_e26489_d_n8;
        locals.var_temp1_dn9 = assign25590_e26489_d_n9;

        let (assign25600_e26497, assign25600_e26497_d_n4, assign25600_e26497_d_n6, assign25600_e26497_d_n7, assign25600_e26497_d_n8, assign25600_e26497_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25600_e26495: f64 = (1.0 - locals.var_temp1);
        (assign25600_e26495, (-locals.var_temp1_dn4), (-locals.var_temp1_dn6), (-locals.var_temp1_dn7), (-locals.var_temp1_dn8), (-locals.var_temp1_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign25600_e26497;
        locals.var_temp2_dn4 = assign25600_e26497_d_n4;
        locals.var_temp2_dn6 = assign25600_e26497_d_n6;
        locals.var_temp2_dn7 = assign25600_e26497_d_n7;
        locals.var_temp2_dn8 = assign25600_e26497_d_n8;
        locals.var_temp2_dn9 = assign25600_e26497_d_n9;

        let (assign25610_e26509, assign25610_e26509_d_n4, assign25610_e26509_d_n6, assign25610_e26509_d_n7, assign25610_e26509_d_n8, assign25610_e26509_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25610_e26503: f64 = (locals.var_gc2ovacc_i * locals.var_temp1);
        let assign25610_e26506: f64 = (locals.var_gc2ovinv_i * locals.var_temp2);
        let assign25610_e26507: f64 = (assign25610_e26503 + assign25610_e26506);
        (assign25610_e26507, ((locals.var_gc2ovacc_i * locals.var_temp1_dn4) + (locals.var_gc2ovinv_i * locals.var_temp2_dn4)), ((locals.var_gc2ovacc_i * locals.var_temp1_dn6) + (locals.var_gc2ovinv_i * locals.var_temp2_dn6)), ((locals.var_gc2ovacc_i * locals.var_temp1_dn7) + (locals.var_gc2ovinv_i * locals.var_temp2_dn7)), ((locals.var_gc2ovacc_i * locals.var_temp1_dn8) + (locals.var_gc2ovinv_i * locals.var_temp2_dn8)), ((locals.var_gc2ovacc_i * locals.var_temp1_dn9) + (locals.var_gc2ovinv_i * locals.var_temp2_dn9)),)
    } else {
        (locals.var_gc2oveff, locals.var_gc2oveff_dn4, locals.var_gc2oveff_dn6, locals.var_gc2oveff_dn7, locals.var_gc2oveff_dn8, locals.var_gc2oveff_dn9,)
    }
};
        locals.var_gc2oveff = assign25610_e26509;
        locals.var_gc2oveff_dn4 = assign25610_e26509_d_n4;
        locals.var_gc2oveff_dn6 = assign25610_e26509_d_n6;
        locals.var_gc2oveff_dn7 = assign25610_e26509_d_n7;
        locals.var_gc2oveff_dn8 = assign25610_e26509_d_n8;
        locals.var_gc2oveff_dn9 = assign25610_e26509_d_n9;

        let (assign25620_e26521, assign25620_e26521_d_n4, assign25620_e26521_d_n6, assign25620_e26521_d_n7, assign25620_e26521_d_n8, assign25620_e26521_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25620_e26515: f64 = (locals.var_gc3ovacc_i * locals.var_temp1);
        let assign25620_e26518: f64 = (locals.var_gc3ovinv_i * locals.var_temp2);
        let assign25620_e26519: f64 = (assign25620_e26515 + assign25620_e26518);
        (assign25620_e26519, ((locals.var_gc3ovacc_i * locals.var_temp1_dn4) + (locals.var_gc3ovinv_i * locals.var_temp2_dn4)), ((locals.var_gc3ovacc_i * locals.var_temp1_dn6) + (locals.var_gc3ovinv_i * locals.var_temp2_dn6)), ((locals.var_gc3ovacc_i * locals.var_temp1_dn7) + (locals.var_gc3ovinv_i * locals.var_temp2_dn7)), ((locals.var_gc3ovacc_i * locals.var_temp1_dn8) + (locals.var_gc3ovinv_i * locals.var_temp2_dn8)), ((locals.var_gc3ovacc_i * locals.var_temp1_dn9) + (locals.var_gc3ovinv_i * locals.var_temp2_dn9)),)
    } else {
        (locals.var_gc3oveff, locals.var_gc3oveff_dn4, locals.var_gc3oveff_dn6, locals.var_gc3oveff_dn7, locals.var_gc3oveff_dn8, locals.var_gc3oveff_dn9,)
    }
};
        locals.var_gc3oveff = assign25620_e26521;
        locals.var_gc3oveff_dn4 = assign25620_e26521_d_n4;
        locals.var_gc3oveff_dn6 = assign25620_e26521_d_n6;
        locals.var_gc3oveff_dn7 = assign25620_e26521_d_n7;
        locals.var_gc3oveff_dn8 = assign25620_e26521_d_n8;
        locals.var_gc3oveff_dn9 = assign25620_e26521_d_n9;

        let (assign25630_e26533, assign25630_e26533_d_n4, assign25630_e26533_d_n6, assign25630_e26533_d_n7, assign25630_e26533_d_n8, assign25630_e26533_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25630_e26527: f64 = (locals.var_gcqovacc * locals.var_temp1);
        let assign25630_e26530: f64 = (locals.var_gcqovinv * locals.var_temp2);
        let assign25630_e26531: f64 = (assign25630_e26527 + assign25630_e26530);
        (assign25630_e26531, ((locals.var_gcqovacc * locals.var_temp1_dn4) + (locals.var_gcqovinv * locals.var_temp2_dn4)), ((locals.var_gcqovacc * locals.var_temp1_dn6) + (locals.var_gcqovinv * locals.var_temp2_dn6)), ((locals.var_gcqovacc * locals.var_temp1_dn7) + (locals.var_gcqovinv * locals.var_temp2_dn7)), ((locals.var_gcqovacc * locals.var_temp1_dn8) + (locals.var_gcqovinv * locals.var_temp2_dn8)), ((locals.var_gcqovacc * locals.var_temp1_dn9) + (locals.var_gcqovinv * locals.var_temp2_dn9)),)
    } else {
        (locals.var_gcqoveff, locals.var_gcqoveff_dn4, locals.var_gcqoveff_dn6, locals.var_gcqoveff_dn7, locals.var_gcqoveff_dn8, locals.var_gcqoveff_dn9,)
    }
};
        locals.var_gcqoveff = assign25630_e26533;
        locals.var_gcqoveff_dn4 = assign25630_e26533_d_n4;
        locals.var_gcqoveff_dn6 = assign25630_e26533_d_n6;
        locals.var_gcqoveff_dn7 = assign25630_e26533_d_n7;
        locals.var_gcqoveff_dn8 = assign25630_e26533_d_n8;
        locals.var_gcqoveff_dn9 = assign25630_e26533_d_n9;

        let (assign25640_e26545, assign25640_e26545_d_n4, assign25640_e26545_d_n6, assign25640_e26545_d_n7, assign25640_e26545_d_n8, assign25640_e26545_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25640_e26539: f64 = (locals.var_igovacc_i * locals.var_temp1);
        let assign25640_e26542: f64 = (locals.var_igovinv_i * locals.var_temp2);
        let assign25640_e26543: f64 = (assign25640_e26539 + assign25640_e26542);
        (assign25640_e26543, (((locals.var_igovacc_i_dn4 * locals.var_temp1) + (locals.var_igovacc_i * locals.var_temp1_dn4)) + ((locals.var_igovinv_i_dn4 * locals.var_temp2) + (locals.var_igovinv_i * locals.var_temp2_dn4))), (((locals.var_igovacc_i_dn6 * locals.var_temp1) + (locals.var_igovacc_i * locals.var_temp1_dn6)) + ((locals.var_igovinv_i_dn6 * locals.var_temp2) + (locals.var_igovinv_i * locals.var_temp2_dn6))), (((locals.var_igovacc_i_dn7 * locals.var_temp1) + (locals.var_igovacc_i * locals.var_temp1_dn7)) + ((locals.var_igovinv_i_dn7 * locals.var_temp2) + (locals.var_igovinv_i * locals.var_temp2_dn7))), (((locals.var_igovacc_i_dn8 * locals.var_temp1) + (locals.var_igovacc_i * locals.var_temp1_dn8)) + ((locals.var_igovinv_i_dn8 * locals.var_temp2) + (locals.var_igovinv_i * locals.var_temp2_dn8))), (((locals.var_igovacc_i_dn9 * locals.var_temp1) + (locals.var_igovacc_i * locals.var_temp1_dn9)) + ((locals.var_igovinv_i_dn9 * locals.var_temp2) + (locals.var_igovinv_i * locals.var_temp2_dn9))),)
    } else {
        (locals.var_igoveff, locals.var_igoveff_dn4, locals.var_igoveff_dn6, locals.var_igoveff_dn7, locals.var_igoveff_dn8, locals.var_igoveff_dn9,)
    }
};
        locals.var_igoveff = assign25640_e26545;
        locals.var_igoveff_dn4 = assign25640_e26545_d_n4;
        locals.var_igoveff_dn6 = assign25640_e26545_d_n6;
        locals.var_igoveff_dn7 = assign25640_e26545_d_n7;
        locals.var_igoveff_dn8 = assign25640_e26545_d_n8;
        locals.var_igoveff_dn9 = assign25640_e26545_d_n9;

        let (assign25650_e26555, assign25650_e26555_d_n4, assign25650_e26555_d_n6, assign25650_e26555_d_n7, assign25650_e26555_d_n8, assign25650_e26555_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25650_e26551: f64 = (locals.var_fnovinv_i * locals.var_temp2);
        let assign25650_e26553: f64 = (assign25650_e26551 * 1e-6);
        (assign25650_e26553, (((locals.var_fnovinv_i_dn4 * locals.var_temp2) + (locals.var_fnovinv_i * locals.var_temp2_dn4)) * 1e-6), (((locals.var_fnovinv_i_dn6 * locals.var_temp2) + (locals.var_fnovinv_i * locals.var_temp2_dn6)) * 1e-6), (((locals.var_fnovinv_i_dn7 * locals.var_temp2) + (locals.var_fnovinv_i * locals.var_temp2_dn7)) * 1e-6), (((locals.var_fnovinv_i_dn8 * locals.var_temp2) + (locals.var_fnovinv_i * locals.var_temp2_dn8)) * 1e-6), (((locals.var_fnovinv_i_dn9 * locals.var_temp2) + (locals.var_fnovinv_i * locals.var_temp2_dn9)) * 1e-6),)
    } else {
        (locals.var_igovefffowler, locals.var_igovefffowler_dn4, locals.var_igovefffowler_dn6, locals.var_igovefffowler_dn7, locals.var_igovefffowler_dn8, locals.var_igovefffowler_dn9,)
    }
};
        locals.var_igovefffowler = assign25650_e26555;
        locals.var_igovefffowler_dn4 = assign25650_e26555_d_n4;
        locals.var_igovefffowler_dn6 = assign25650_e26555_d_n6;
        locals.var_igovefffowler_dn7 = assign25650_e26555_d_n7;
        locals.var_igovefffowler_dn8 = assign25650_e26555_d_n8;
        locals.var_igovefffowler_dn9 = assign25650_e26555_d_n9;

        let (assign25660_e26568, assign25660_e26568_d_n4, assign25660_e26568_d_n6, assign25660_e26568_d_n7, assign25660_e26568_d_n8, assign25660_e26568_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25660_e26561: f64 = (-1.0);
        let assign25660_e26563: f64 = (assign25660_e26561 * locals.var_gcovinvfn_i);
        let assign25660_e26565: f64 = (assign25660_e26563 / locals.var_zg);
        let assign25660_e26566: f64 = (locals.var_bov * assign25660_e26565);
        (assign25660_e26566, ((locals.var_bov_dn4 * assign25660_e26565) + (locals.var_bov * (-((assign25660_e26563 * locals.var_zg_dn4) / (locals.var_zg * locals.var_zg))))), ((locals.var_bov_dn6 * assign25660_e26565) + (locals.var_bov * (-((assign25660_e26563 * locals.var_zg_dn6) / (locals.var_zg * locals.var_zg))))), ((locals.var_bov_dn7 * assign25660_e26565) + (locals.var_bov * (-((assign25660_e26563 * locals.var_zg_dn7) / (locals.var_zg * locals.var_zg))))), ((locals.var_bov_dn8 * assign25660_e26565) + (locals.var_bov * (-((assign25660_e26563 * locals.var_zg_dn8) / (locals.var_zg * locals.var_zg))))), ((locals.var_bov_dn9 * assign25660_e26565) + (locals.var_bov * (-((assign25660_e26563 * locals.var_zg_dn9) / (locals.var_zg * locals.var_zg))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign25660_e26568;
        locals.var_temp1_dn4 = assign25660_e26568_d_n4;
        locals.var_temp1_dn6 = assign25660_e26568_d_n6;
        locals.var_temp1_dn7 = assign25660_e26568_d_n7;
        locals.var_temp1_dn8 = assign25660_e26568_d_n8;
        locals.var_temp1_dn9 = assign25660_e26568_d_n9;

        let assign25670_e26571: f64 = if locals.var_gc3oveff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard725 = assign25670_e26571;

    }

    pub(super) fn stamp_transient_block_66(
        locals: &mut StampLocals,
    ) {
        let (assign25680_e26594, assign25680_e26594_d_n4, assign25680_e26594_d_n6, assign25680_e26594_d_n7, assign25680_e26594_d_n8, assign25680_e26594_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard725 != 0.0)) {
        let assign25680_e26580: f64 = (locals.var_zg + locals.var_gcqoveff);
        let assign25680_e26583: f64 = (locals.var_zg - locals.var_gcqoveff);
        let assign25680_e26586: f64 = (locals.var_zg - locals.var_gcqoveff);
        let assign25680_e26587: f64 = (assign25680_e26583 * assign25680_e26586);
        let assign25680_e26589: f64 = (assign25680_e26587 + 1e-6);
        let assign25680_e26590: f64 = (assign25680_e26589).sqrt();
        let assign25680_e26591: f64 = (assign25680_e26580 - assign25680_e26590);
        let assign25680_e26592: f64 = (0.5 * assign25680_e26591);
        (assign25680_e26592, (0.5 * ((locals.var_zg_dn4 + locals.var_gcqoveff_dn4) - ((((locals.var_zg_dn4 - locals.var_gcqoveff_dn4) * assign25680_e26586) + (assign25680_e26583 * (locals.var_zg_dn4 - locals.var_gcqoveff_dn4))) / (2.0 * assign25680_e26590)))), (0.5 * ((locals.var_zg_dn6 + locals.var_gcqoveff_dn6) - ((((locals.var_zg_dn6 - locals.var_gcqoveff_dn6) * assign25680_e26586) + (assign25680_e26583 * (locals.var_zg_dn6 - locals.var_gcqoveff_dn6))) / (2.0 * assign25680_e26590)))), (0.5 * ((locals.var_zg_dn7 + locals.var_gcqoveff_dn7) - ((((locals.var_zg_dn7 - locals.var_gcqoveff_dn7) * assign25680_e26586) + (assign25680_e26583 * (locals.var_zg_dn7 - locals.var_gcqoveff_dn7))) / (2.0 * assign25680_e26590)))), (0.5 * ((locals.var_zg_dn8 + locals.var_gcqoveff_dn8) - ((((locals.var_zg_dn8 - locals.var_gcqoveff_dn8) * assign25680_e26586) + (assign25680_e26583 * (locals.var_zg_dn8 - locals.var_gcqoveff_dn8))) / (2.0 * assign25680_e26590)))), (0.5 * ((locals.var_zg_dn9 + locals.var_gcqoveff_dn9) - ((((locals.var_zg_dn9 - locals.var_gcqoveff_dn9) * assign25680_e26586) + (assign25680_e26583 * (locals.var_zg_dn9 - locals.var_gcqoveff_dn9))) / (2.0 * assign25680_e26590)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign25680_e26594;
        locals.var_zg_dn4 = assign25680_e26594_d_n4;
        locals.var_zg_dn6 = assign25680_e26594_d_n6;
        locals.var_zg_dn7 = assign25680_e26594_d_n7;
        locals.var_zg_dn8 = assign25680_e26594_d_n8;
        locals.var_zg_dn9 = assign25680_e26594_d_n9;

        let (assign25690_e26606, assign25690_e26606_d_n4, assign25690_e26606_d_n6, assign25690_e26606_d_n7, assign25690_e26606_d_n8, assign25690_e26606_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25690_e26600: f64 = (3.0 + locals.var_xs_ov);
        let assign25690_e26603: f64 = (locals.var_psi_t * locals.var_inv_phit0);
        let assign25690_e26604: f64 = (assign25690_e26600 + assign25690_e26603);
        (assign25690_e26604, (locals.var_xs_ov_dn4 + ((locals.var_psi_t_dn4 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn4))), (locals.var_xs_ov_dn6 + ((locals.var_psi_t_dn6 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn6))), (locals.var_xs_ov_dn7 + ((locals.var_psi_t_dn7 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn7))), (locals.var_xs_ov_dn8 + ((locals.var_psi_t_dn8 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn8))), (locals.var_xs_ov_dn9 + ((locals.var_psi_t_dn9 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn9))),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9,)
    }
};
        locals.var_arg1 = assign25690_e26606;
        locals.var_arg1_dn4 = assign25690_e26606_d_n4;
        locals.var_arg1_dn6 = assign25690_e26606_d_n6;
        locals.var_arg1_dn7 = assign25690_e26606_d_n7;
        locals.var_arg1_dn8 = assign25690_e26606_d_n8;
        locals.var_arg1_dn9 = assign25690_e26606_d_n9;

        let assign25700_e26608: f64 = (locals.var_arg1).abs();
        let assign25700_e26610: f64 = if assign25700_e26608 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard726 = assign25700_e26610;

        let (assign25710_e26619, assign25710_e26619_d_n4, assign25710_e26619_d_n6, assign25710_e26619_d_n7, assign25710_e26619_d_n8, assign25710_e26619_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard726 != 0.0)) {
        let assign25710_e26617: f64 = (locals.var_arg1).exp();
        (assign25710_e26617, (assign25710_e26617 * locals.var_arg1_dn4), (assign25710_e26617 * locals.var_arg1_dn6), (assign25710_e26617 * locals.var_arg1_dn7), (assign25710_e26617 * locals.var_arg1_dn8), (assign25710_e26617 * locals.var_arg1_dn9),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign25710_e26619;
        locals.var_dsi_dn4 = assign25710_e26619_d_n4;
        locals.var_dsi_dn6 = assign25710_e26619_d_n6;
        locals.var_dsi_dn7 = assign25710_e26619_d_n7;
        locals.var_dsi_dn8 = assign25710_e26619_d_n8;
        locals.var_dsi_dn9 = assign25710_e26619_d_n9;

        let assign25720_e26622: f64 = (-80.0);
        let assign25720_e26623: f64 = if locals.var_arg1 < assign25720_e26622 { 1.0 } else { 0.0 };
        locals.var_guard727 = assign25720_e26623;

        let (assign25730_e26659, assign25730_e26659_d_n4, assign25730_e26659_d_n6, assign25730_e26659_d_n7, assign25730_e26659_d_n8, assign25730_e26659_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard726 == 0.0)) && (locals.var_guard727 != 0.0)) {
        let assign25730_e26635: f64 = (-locals.var_arg1);
        let assign25730_e26637: f64 = (assign25730_e26635 - 80.0);
        let assign25730_e26641: f64 = (-locals.var_arg1);
        let assign25730_e26643: f64 = (assign25730_e26641 - 80.0);
        let assign25730_e26644: f64 = (0.5 * assign25730_e26643);
        let assign25730_e26647: f64 = (-locals.var_arg1);
        let assign25730_e26649: f64 = (assign25730_e26647 - 80.0);
        let assign25730_e26651: f64 = (assign25730_e26649 * 0.3333333333333);
        let assign25730_e26652: f64 = (1.0 + assign25730_e26651);
        let assign25730_e26653: f64 = (assign25730_e26644 * assign25730_e26652);
        let assign25730_e26654: f64 = (1.0 + assign25730_e26653);
        let assign25730_e26655: f64 = (assign25730_e26637 * assign25730_e26654);
        let assign25730_e26656: f64 = (1.0 + assign25730_e26655);
        let assign25730_e26657: f64 = (1.80485e-35 / assign25730_e26656);
        (assign25730_e26657, (-((1.80485e-35 * (((-locals.var_arg1_dn4) * assign25730_e26654) + (assign25730_e26637 * (((0.5 * (-locals.var_arg1_dn4)) * assign25730_e26652) + (assign25730_e26644 * ((-locals.var_arg1_dn4) * 0.3333333333333)))))) / (assign25730_e26656 * assign25730_e26656))), (-((1.80485e-35 * (((-locals.var_arg1_dn6) * assign25730_e26654) + (assign25730_e26637 * (((0.5 * (-locals.var_arg1_dn6)) * assign25730_e26652) + (assign25730_e26644 * ((-locals.var_arg1_dn6) * 0.3333333333333)))))) / (assign25730_e26656 * assign25730_e26656))), (-((1.80485e-35 * (((-locals.var_arg1_dn7) * assign25730_e26654) + (assign25730_e26637 * (((0.5 * (-locals.var_arg1_dn7)) * assign25730_e26652) + (assign25730_e26644 * ((-locals.var_arg1_dn7) * 0.3333333333333)))))) / (assign25730_e26656 * assign25730_e26656))), (-((1.80485e-35 * (((-locals.var_arg1_dn8) * assign25730_e26654) + (assign25730_e26637 * (((0.5 * (-locals.var_arg1_dn8)) * assign25730_e26652) + (assign25730_e26644 * ((-locals.var_arg1_dn8) * 0.3333333333333)))))) / (assign25730_e26656 * assign25730_e26656))), (-((1.80485e-35 * (((-locals.var_arg1_dn9) * assign25730_e26654) + (assign25730_e26637 * (((0.5 * (-locals.var_arg1_dn9)) * assign25730_e26652) + (assign25730_e26644 * ((-locals.var_arg1_dn9) * 0.3333333333333)))))) / (assign25730_e26656 * assign25730_e26656))),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign25730_e26659;
        locals.var_dsi_dn4 = assign25730_e26659_d_n4;
        locals.var_dsi_dn6 = assign25730_e26659_d_n6;
        locals.var_dsi_dn7 = assign25730_e26659_d_n7;
        locals.var_dsi_dn8 = assign25730_e26659_d_n8;
        locals.var_dsi_dn9 = assign25730_e26659_d_n9;

        let (assign25740_e26693, assign25740_e26693_d_n4, assign25740_e26693_d_n6, assign25740_e26693_d_n7, assign25740_e26693_d_n8, assign25740_e26693_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard726 == 0.0)) && (locals.var_guard727 == 0.0)) {
        let assign25740_e26673: f64 = (locals.var_arg1 - 80.0);
        let assign25740_e26678: f64 = (locals.var_arg1 - 80.0);
        let assign25740_e26679: f64 = (0.5 * assign25740_e26678);
        let assign25740_e26683: f64 = (locals.var_arg1 - 80.0);
        let assign25740_e26685: f64 = (assign25740_e26683 * 0.3333333333333);
        let assign25740_e26686: f64 = (1.0 + assign25740_e26685);
        let assign25740_e26687: f64 = (assign25740_e26679 * assign25740_e26686);
        let assign25740_e26688: f64 = (1.0 + assign25740_e26687);
        let assign25740_e26689: f64 = (assign25740_e26673 * assign25740_e26688);
        let assign25740_e26690: f64 = (1.0 + assign25740_e26689);
        let assign25740_e26691: f64 = (5.54062e34 * assign25740_e26690);
        (assign25740_e26691, (5.54062e34 * ((locals.var_arg1_dn4 * assign25740_e26688) + (assign25740_e26673 * (((0.5 * locals.var_arg1_dn4) * assign25740_e26686) + (assign25740_e26679 * (locals.var_arg1_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn6 * assign25740_e26688) + (assign25740_e26673 * (((0.5 * locals.var_arg1_dn6) * assign25740_e26686) + (assign25740_e26679 * (locals.var_arg1_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn7 * assign25740_e26688) + (assign25740_e26673 * (((0.5 * locals.var_arg1_dn7) * assign25740_e26686) + (assign25740_e26679 * (locals.var_arg1_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn8 * assign25740_e26688) + (assign25740_e26673 * (((0.5 * locals.var_arg1_dn8) * assign25740_e26686) + (assign25740_e26679 * (locals.var_arg1_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn9 * assign25740_e26688) + (assign25740_e26673 * (((0.5 * locals.var_arg1_dn9) * assign25740_e26686) + (assign25740_e26679 * (locals.var_arg1_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign25740_e26693;
        locals.var_dsi_dn4 = assign25740_e26693_d_n4;
        locals.var_dsi_dn6 = assign25740_e26693_d_n6;
        locals.var_dsi_dn7 = assign25740_e26693_d_n7;
        locals.var_dsi_dn8 = assign25740_e26693_d_n8;
        locals.var_dsi_dn9 = assign25740_e26693_d_n9;

        let (assign25750_e26707, assign25750_e26707_d_n4, assign25750_e26707_d_n6, assign25750_e26707_d_n7, assign25750_e26707_d_n8, assign25750_e26707_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25750_e26699: f64 = (3.0 + locals.var_xs_ov);
        let assign25750_e26702: f64 = (locals.var_psi_t * locals.var_inv_phit0);
        let assign25750_e26703: f64 = (assign25750_e26699 + assign25750_e26702);
        let assign25750_e26705: f64 = (assign25750_e26703 + locals.var_xgs_ov);
        (assign25750_e26705, ((locals.var_xs_ov_dn4 + ((locals.var_psi_t_dn4 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn4))) + locals.var_xgs_ov_dn4), ((locals.var_xs_ov_dn6 + ((locals.var_psi_t_dn6 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn6))) + locals.var_xgs_ov_dn6), ((locals.var_xs_ov_dn7 + ((locals.var_psi_t_dn7 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn7))) + locals.var_xgs_ov_dn7), ((locals.var_xs_ov_dn8 + ((locals.var_psi_t_dn8 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn8))) + locals.var_xgs_ov_dn8), ((locals.var_xs_ov_dn9 + ((locals.var_psi_t_dn9 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn9))) + locals.var_xgs_ov_dn9),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9,)
    }
};
        locals.var_arg1 = assign25750_e26707;
        locals.var_arg1_dn4 = assign25750_e26707_d_n4;
        locals.var_arg1_dn6 = assign25750_e26707_d_n6;
        locals.var_arg1_dn7 = assign25750_e26707_d_n7;
        locals.var_arg1_dn8 = assign25750_e26707_d_n8;
        locals.var_arg1_dn9 = assign25750_e26707_d_n9;

        let assign25760_e26709: f64 = (locals.var_arg1).abs();
        let assign25760_e26711: f64 = if assign25760_e26709 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard728 = assign25760_e26711;

        let (assign25770_e26720, assign25770_e26720_d_n4, assign25770_e26720_d_n6, assign25770_e26720_d_n7, assign25770_e26720_d_n8, assign25770_e26720_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard728 != 0.0)) {
        let assign25770_e26718: f64 = (locals.var_arg1).exp();
        (assign25770_e26718, (assign25770_e26718 * locals.var_arg1_dn4), (assign25770_e26718 * locals.var_arg1_dn6), (assign25770_e26718 * locals.var_arg1_dn7), (assign25770_e26718 * locals.var_arg1_dn8), (assign25770_e26718 * locals.var_arg1_dn9),)
    } else {
        (locals.var_dgate, locals.var_dgate_dn4, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8, locals.var_dgate_dn9,)
    }
};
        locals.var_dgate = assign25770_e26720;
        locals.var_dgate_dn4 = assign25770_e26720_d_n4;
        locals.var_dgate_dn6 = assign25770_e26720_d_n6;
        locals.var_dgate_dn7 = assign25770_e26720_d_n7;
        locals.var_dgate_dn8 = assign25770_e26720_d_n8;
        locals.var_dgate_dn9 = assign25770_e26720_d_n9;

        let assign25780_e26723: f64 = (-80.0);
        let assign25780_e26724: f64 = if locals.var_arg1 < assign25780_e26723 { 1.0 } else { 0.0 };
        locals.var_guard729 = assign25780_e26724;

        let (assign25790_e26760, assign25790_e26760_d_n4, assign25790_e26760_d_n6, assign25790_e26760_d_n7, assign25790_e26760_d_n8, assign25790_e26760_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard729 != 0.0)) {
        let assign25790_e26736: f64 = (-locals.var_arg1);
        let assign25790_e26738: f64 = (assign25790_e26736 - 80.0);
        let assign25790_e26742: f64 = (-locals.var_arg1);
        let assign25790_e26744: f64 = (assign25790_e26742 - 80.0);
        let assign25790_e26745: f64 = (0.5 * assign25790_e26744);
        let assign25790_e26748: f64 = (-locals.var_arg1);
        let assign25790_e26750: f64 = (assign25790_e26748 - 80.0);
        let assign25790_e26752: f64 = (assign25790_e26750 * 0.3333333333333);
        let assign25790_e26753: f64 = (1.0 + assign25790_e26752);
        let assign25790_e26754: f64 = (assign25790_e26745 * assign25790_e26753);
        let assign25790_e26755: f64 = (1.0 + assign25790_e26754);
        let assign25790_e26756: f64 = (assign25790_e26738 * assign25790_e26755);
        let assign25790_e26757: f64 = (1.0 + assign25790_e26756);
        let assign25790_e26758: f64 = (1.80485e-35 / assign25790_e26757);
        (assign25790_e26758, (-((1.80485e-35 * (((-locals.var_arg1_dn4) * assign25790_e26755) + (assign25790_e26738 * (((0.5 * (-locals.var_arg1_dn4)) * assign25790_e26753) + (assign25790_e26745 * ((-locals.var_arg1_dn4) * 0.3333333333333)))))) / (assign25790_e26757 * assign25790_e26757))), (-((1.80485e-35 * (((-locals.var_arg1_dn6) * assign25790_e26755) + (assign25790_e26738 * (((0.5 * (-locals.var_arg1_dn6)) * assign25790_e26753) + (assign25790_e26745 * ((-locals.var_arg1_dn6) * 0.3333333333333)))))) / (assign25790_e26757 * assign25790_e26757))), (-((1.80485e-35 * (((-locals.var_arg1_dn7) * assign25790_e26755) + (assign25790_e26738 * (((0.5 * (-locals.var_arg1_dn7)) * assign25790_e26753) + (assign25790_e26745 * ((-locals.var_arg1_dn7) * 0.3333333333333)))))) / (assign25790_e26757 * assign25790_e26757))), (-((1.80485e-35 * (((-locals.var_arg1_dn8) * assign25790_e26755) + (assign25790_e26738 * (((0.5 * (-locals.var_arg1_dn8)) * assign25790_e26753) + (assign25790_e26745 * ((-locals.var_arg1_dn8) * 0.3333333333333)))))) / (assign25790_e26757 * assign25790_e26757))), (-((1.80485e-35 * (((-locals.var_arg1_dn9) * assign25790_e26755) + (assign25790_e26738 * (((0.5 * (-locals.var_arg1_dn9)) * assign25790_e26753) + (assign25790_e26745 * ((-locals.var_arg1_dn9) * 0.3333333333333)))))) / (assign25790_e26757 * assign25790_e26757))),)
    } else {
        (locals.var_dgate, locals.var_dgate_dn4, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8, locals.var_dgate_dn9,)
    }
};
        locals.var_dgate = assign25790_e26760;
        locals.var_dgate_dn4 = assign25790_e26760_d_n4;
        locals.var_dgate_dn6 = assign25790_e26760_d_n6;
        locals.var_dgate_dn7 = assign25790_e26760_d_n7;
        locals.var_dgate_dn8 = assign25790_e26760_d_n8;
        locals.var_dgate_dn9 = assign25790_e26760_d_n9;

        let (assign25800_e26794, assign25800_e26794_d_n4, assign25800_e26794_d_n6, assign25800_e26794_d_n7, assign25800_e26794_d_n8, assign25800_e26794_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard728 == 0.0)) && (locals.var_guard729 == 0.0)) {
        let assign25800_e26774: f64 = (locals.var_arg1 - 80.0);
        let assign25800_e26779: f64 = (locals.var_arg1 - 80.0);
        let assign25800_e26780: f64 = (0.5 * assign25800_e26779);
        let assign25800_e26784: f64 = (locals.var_arg1 - 80.0);
        let assign25800_e26786: f64 = (assign25800_e26784 * 0.3333333333333);
        let assign25800_e26787: f64 = (1.0 + assign25800_e26786);
        let assign25800_e26788: f64 = (assign25800_e26780 * assign25800_e26787);
        let assign25800_e26789: f64 = (1.0 + assign25800_e26788);
        let assign25800_e26790: f64 = (assign25800_e26774 * assign25800_e26789);
        let assign25800_e26791: f64 = (1.0 + assign25800_e26790);
        let assign25800_e26792: f64 = (5.54062e34 * assign25800_e26791);
        (assign25800_e26792, (5.54062e34 * ((locals.var_arg1_dn4 * assign25800_e26789) + (assign25800_e26774 * (((0.5 * locals.var_arg1_dn4) * assign25800_e26787) + (assign25800_e26780 * (locals.var_arg1_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn6 * assign25800_e26789) + (assign25800_e26774 * (((0.5 * locals.var_arg1_dn6) * assign25800_e26787) + (assign25800_e26780 * (locals.var_arg1_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn7 * assign25800_e26789) + (assign25800_e26774 * (((0.5 * locals.var_arg1_dn7) * assign25800_e26787) + (assign25800_e26780 * (locals.var_arg1_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn8 * assign25800_e26789) + (assign25800_e26774 * (((0.5 * locals.var_arg1_dn8) * assign25800_e26787) + (assign25800_e26780 * (locals.var_arg1_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn9 * assign25800_e26789) + (assign25800_e26774 * (((0.5 * locals.var_arg1_dn9) * assign25800_e26787) + (assign25800_e26780 * (locals.var_arg1_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_dgate, locals.var_dgate_dn4, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8, locals.var_dgate_dn9,)
    }
};
        locals.var_dgate = assign25800_e26794;
        locals.var_dgate_dn4 = assign25800_e26794_d_n4;
        locals.var_dgate_dn6 = assign25800_e26794_d_n6;
        locals.var_dgate_dn7 = assign25800_e26794_d_n7;
        locals.var_dgate_dn8 = assign25800_e26794_d_n8;
        locals.var_dgate_dn9 = assign25800_e26794_d_n9;

        let (assign25810_e26811, assign25810_e26811_d_n4, assign25810_e26811_d_n6, assign25810_e26811_d_n7, assign25810_e26811_d_n8, assign25810_e26811_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25810_e26800: f64 = (-1.5);
        let assign25810_e26805: f64 = (locals.var_gc3oveff * locals.var_zg);
        let assign25810_e26806: f64 = (locals.var_gc2oveff + assign25810_e26805);
        let assign25810_e26807: f64 = (locals.var_zg * assign25810_e26806);
        let assign25810_e26808: f64 = (assign25810_e26800 + assign25810_e26807);
        let assign25810_e26809: f64 = (locals.var_bov * assign25810_e26808);
        (assign25810_e26809, ((locals.var_bov_dn4 * assign25810_e26808) + (locals.var_bov * ((locals.var_zg_dn4 * assign25810_e26806) + (locals.var_zg * (locals.var_gc2oveff_dn4 + ((locals.var_gc3oveff_dn4 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn4))))))), ((locals.var_bov_dn6 * assign25810_e26808) + (locals.var_bov * ((locals.var_zg_dn6 * assign25810_e26806) + (locals.var_zg * (locals.var_gc2oveff_dn6 + ((locals.var_gc3oveff_dn6 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn6))))))), ((locals.var_bov_dn7 * assign25810_e26808) + (locals.var_bov * ((locals.var_zg_dn7 * assign25810_e26806) + (locals.var_zg * (locals.var_gc2oveff_dn7 + ((locals.var_gc3oveff_dn7 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn7))))))), ((locals.var_bov_dn8 * assign25810_e26808) + (locals.var_bov * ((locals.var_zg_dn8 * assign25810_e26806) + (locals.var_zg * (locals.var_gc2oveff_dn8 + ((locals.var_gc3oveff_dn8 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn8))))))), ((locals.var_bov_dn9 * assign25810_e26808) + (locals.var_bov * ((locals.var_zg_dn9 * assign25810_e26806) + (locals.var_zg * (locals.var_gc2oveff_dn9 + ((locals.var_gc3oveff_dn9 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn9))))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign25810_e26811;
        locals.var_temp_dn4 = assign25810_e26811_d_n4;
        locals.var_temp_dn6 = assign25810_e26811_d_n6;
        locals.var_temp_dn7 = assign25810_e26811_d_n7;
        locals.var_temp_dn8 = assign25810_e26811_d_n8;
        locals.var_temp_dn9 = assign25810_e26811_d_n9;

        let assign25820_e26814: f64 = if locals.var_temp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard730 = assign25820_e26814;

        let (assign25830_e26836, assign25830_e26836_d_n4, assign25830_e26836_d_n6, assign25830_e26836_d_n7, assign25830_e26836_d_n8, assign25830_e26836_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard730 != 0.0)) {
        let assign25830_e26825: f64 = (0.5 * locals.var_temp);
        let assign25830_e26829: f64 = (locals.var_temp * 0.3333333333333);
        let assign25830_e26830: f64 = (1.0 + assign25830_e26829);
        let assign25830_e26831: f64 = (assign25830_e26825 * assign25830_e26830);
        let assign25830_e26832: f64 = (1.0 + assign25830_e26831);
        let assign25830_e26833: f64 = (locals.var_temp * assign25830_e26832);
        let assign25830_e26834: f64 = (1.0 + assign25830_e26833);
        (assign25830_e26834, ((locals.var_temp_dn4 * assign25830_e26832) + (locals.var_temp * (((0.5 * locals.var_temp_dn4) * assign25830_e26830) + (assign25830_e26825 * (locals.var_temp_dn4 * 0.3333333333333))))), ((locals.var_temp_dn6 * assign25830_e26832) + (locals.var_temp * (((0.5 * locals.var_temp_dn6) * assign25830_e26830) + (assign25830_e26825 * (locals.var_temp_dn6 * 0.3333333333333))))), ((locals.var_temp_dn7 * assign25830_e26832) + (locals.var_temp * (((0.5 * locals.var_temp_dn7) * assign25830_e26830) + (assign25830_e26825 * (locals.var_temp_dn7 * 0.3333333333333))))), ((locals.var_temp_dn8 * assign25830_e26832) + (locals.var_temp * (((0.5 * locals.var_temp_dn8) * assign25830_e26830) + (assign25830_e26825 * (locals.var_temp_dn8 * 0.3333333333333))))), ((locals.var_temp_dn9 * assign25830_e26832) + (locals.var_temp * (((0.5 * locals.var_temp_dn9) * assign25830_e26830) + (assign25830_e26825 * (locals.var_temp_dn9 * 0.3333333333333))))),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign25830_e26836;
        locals.var_tp_dn4 = assign25830_e26836_d_n4;
        locals.var_tp_dn6 = assign25830_e26836_d_n6;
        locals.var_tp_dn7 = assign25830_e26836_d_n7;
        locals.var_tp_dn8 = assign25830_e26836_d_n8;
        locals.var_tp_dn9 = assign25830_e26836_d_n9;

        let assign25840_e26839: f64 = (-80.0);
        let assign25840_e26840: f64 = if locals.var_temp > assign25840_e26839 { 1.0 } else { 0.0 };
        locals.var_guard731 = assign25840_e26840;

        let (assign25850_e26852, assign25850_e26852_d_n4, assign25850_e26852_d_n6, assign25850_e26852_d_n7, assign25850_e26852_d_n8, assign25850_e26852_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard730 == 0.0)) && (locals.var_guard731 != 0.0)) {
        let assign25850_e26850: f64 = (locals.var_temp).exp();
        (assign25850_e26850, (assign25850_e26850 * locals.var_temp_dn4), (assign25850_e26850 * locals.var_temp_dn6), (assign25850_e26850 * locals.var_temp_dn7), (assign25850_e26850 * locals.var_temp_dn8), (assign25850_e26850 * locals.var_temp_dn9),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign25850_e26852;
        locals.var_tp_dn4 = assign25850_e26852_d_n4;
        locals.var_tp_dn6 = assign25850_e26852_d_n6;
        locals.var_tp_dn7 = assign25850_e26852_d_n7;
        locals.var_tp_dn8 = assign25850_e26852_d_n8;
        locals.var_tp_dn9 = assign25850_e26852_d_n9;

        let (assign25860_e26889, assign25860_e26889_d_n4, assign25860_e26889_d_n6, assign25860_e26889_d_n7, assign25860_e26889_d_n8, assign25860_e26889_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard730 == 0.0)) && (locals.var_guard731 == 0.0)) {
        let assign25860_e26865: f64 = (-locals.var_temp);
        let assign25860_e26867: f64 = (assign25860_e26865 - 80.0);
        let assign25860_e26871: f64 = (-locals.var_temp);
        let assign25860_e26873: f64 = (assign25860_e26871 - 80.0);
        let assign25860_e26874: f64 = (0.5 * assign25860_e26873);
        let assign25860_e26877: f64 = (-locals.var_temp);
        let assign25860_e26879: f64 = (assign25860_e26877 - 80.0);
        let assign25860_e26881: f64 = (assign25860_e26879 * 0.3333333333333);
        let assign25860_e26882: f64 = (1.0 + assign25860_e26881);
        let assign25860_e26883: f64 = (assign25860_e26874 * assign25860_e26882);
        let assign25860_e26884: f64 = (1.0 + assign25860_e26883);
        let assign25860_e26885: f64 = (assign25860_e26867 * assign25860_e26884);
        let assign25860_e26886: f64 = (1.0 + assign25860_e26885);
        let assign25860_e26887: f64 = (1.80485e-35 / assign25860_e26886);
        (assign25860_e26887, (-((1.80485e-35 * (((-locals.var_temp_dn4) * assign25860_e26884) + (assign25860_e26867 * (((0.5 * (-locals.var_temp_dn4)) * assign25860_e26882) + (assign25860_e26874 * ((-locals.var_temp_dn4) * 0.3333333333333)))))) / (assign25860_e26886 * assign25860_e26886))), (-((1.80485e-35 * (((-locals.var_temp_dn6) * assign25860_e26884) + (assign25860_e26867 * (((0.5 * (-locals.var_temp_dn6)) * assign25860_e26882) + (assign25860_e26874 * ((-locals.var_temp_dn6) * 0.3333333333333)))))) / (assign25860_e26886 * assign25860_e26886))), (-((1.80485e-35 * (((-locals.var_temp_dn7) * assign25860_e26884) + (assign25860_e26867 * (((0.5 * (-locals.var_temp_dn7)) * assign25860_e26882) + (assign25860_e26874 * ((-locals.var_temp_dn7) * 0.3333333333333)))))) / (assign25860_e26886 * assign25860_e26886))), (-((1.80485e-35 * (((-locals.var_temp_dn8) * assign25860_e26884) + (assign25860_e26867 * (((0.5 * (-locals.var_temp_dn8)) * assign25860_e26882) + (assign25860_e26874 * ((-locals.var_temp_dn8) * 0.3333333333333)))))) / (assign25860_e26886 * assign25860_e26886))), (-((1.80485e-35 * (((-locals.var_temp_dn9) * assign25860_e26884) + (assign25860_e26867 * (((0.5 * (-locals.var_temp_dn9)) * assign25860_e26882) + (assign25860_e26874 * ((-locals.var_temp_dn9) * 0.3333333333333)))))) / (assign25860_e26886 * assign25860_e26886))),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign25860_e26889;
        locals.var_tp_dn4 = assign25860_e26889_d_n4;
        locals.var_tp_dn6 = assign25860_e26889_d_n6;
        locals.var_tp_dn7 = assign25860_e26889_d_n7;
        locals.var_tp_dn8 = assign25860_e26889_d_n8;
        locals.var_tp_dn9 = assign25860_e26889_d_n9;

        let assign25870_e26892: f64 = if locals.var_temp1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard732 = assign25870_e26892;

        let (assign25880_e26914, assign25880_e26914_d_n4, assign25880_e26914_d_n6, assign25880_e26914_d_n7, assign25880_e26914_d_n8, assign25880_e26914_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard732 != 0.0)) {
        let assign25880_e26903: f64 = (0.5 * locals.var_temp1);
        let assign25880_e26907: f64 = (locals.var_temp1 * 0.3333333333333);
        let assign25880_e26908: f64 = (1.0 + assign25880_e26907);
        let assign25880_e26909: f64 = (assign25880_e26903 * assign25880_e26908);
        let assign25880_e26910: f64 = (1.0 + assign25880_e26909);
        let assign25880_e26911: f64 = (locals.var_temp1 * assign25880_e26910);
        let assign25880_e26912: f64 = (1.0 + assign25880_e26911);
        (assign25880_e26912, ((locals.var_temp1_dn4 * assign25880_e26910) + (locals.var_temp1 * (((0.5 * locals.var_temp1_dn4) * assign25880_e26908) + (assign25880_e26903 * (locals.var_temp1_dn4 * 0.3333333333333))))), ((locals.var_temp1_dn6 * assign25880_e26910) + (locals.var_temp1 * (((0.5 * locals.var_temp1_dn6) * assign25880_e26908) + (assign25880_e26903 * (locals.var_temp1_dn6 * 0.3333333333333))))), ((locals.var_temp1_dn7 * assign25880_e26910) + (locals.var_temp1 * (((0.5 * locals.var_temp1_dn7) * assign25880_e26908) + (assign25880_e26903 * (locals.var_temp1_dn7 * 0.3333333333333))))), ((locals.var_temp1_dn8 * assign25880_e26910) + (locals.var_temp1 * (((0.5 * locals.var_temp1_dn8) * assign25880_e26908) + (assign25880_e26903 * (locals.var_temp1_dn8 * 0.3333333333333))))), ((locals.var_temp1_dn9 * assign25880_e26910) + (locals.var_temp1 * (((0.5 * locals.var_temp1_dn9) * assign25880_e26908) + (assign25880_e26903 * (locals.var_temp1_dn9 * 0.3333333333333))))),)
    } else {
        (locals.var_tp2, locals.var_tp2_dn4, locals.var_tp2_dn6, locals.var_tp2_dn7, locals.var_tp2_dn8, locals.var_tp2_dn9,)
    }
};
        locals.var_tp2 = assign25880_e26914;
        locals.var_tp2_dn4 = assign25880_e26914_d_n4;
        locals.var_tp2_dn6 = assign25880_e26914_d_n6;
        locals.var_tp2_dn7 = assign25880_e26914_d_n7;
        locals.var_tp2_dn8 = assign25880_e26914_d_n8;
        locals.var_tp2_dn9 = assign25880_e26914_d_n9;

        let assign25890_e26917: f64 = (-80.0);
        let assign25890_e26918: f64 = if locals.var_temp1 > assign25890_e26917 { 1.0 } else { 0.0 };
        locals.var_guard733 = assign25890_e26918;

        let (assign25900_e26930, assign25900_e26930_d_n4, assign25900_e26930_d_n6, assign25900_e26930_d_n7, assign25900_e26930_d_n8, assign25900_e26930_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard732 == 0.0)) && (locals.var_guard733 != 0.0)) {
        let assign25900_e26928: f64 = (locals.var_temp1).exp();
        (assign25900_e26928, (assign25900_e26928 * locals.var_temp1_dn4), (assign25900_e26928 * locals.var_temp1_dn6), (assign25900_e26928 * locals.var_temp1_dn7), (assign25900_e26928 * locals.var_temp1_dn8), (assign25900_e26928 * locals.var_temp1_dn9),)
    } else {
        (locals.var_tp2, locals.var_tp2_dn4, locals.var_tp2_dn6, locals.var_tp2_dn7, locals.var_tp2_dn8, locals.var_tp2_dn9,)
    }
};
        locals.var_tp2 = assign25900_e26930;
        locals.var_tp2_dn4 = assign25900_e26930_d_n4;
        locals.var_tp2_dn6 = assign25900_e26930_d_n6;
        locals.var_tp2_dn7 = assign25900_e26930_d_n7;
        locals.var_tp2_dn8 = assign25900_e26930_d_n8;
        locals.var_tp2_dn9 = assign25900_e26930_d_n9;

        let (assign25910_e26967, assign25910_e26967_d_n4, assign25910_e26967_d_n6, assign25910_e26967_d_n7, assign25910_e26967_d_n8, assign25910_e26967_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard732 == 0.0)) && (locals.var_guard733 == 0.0)) {
        let assign25910_e26943: f64 = (-locals.var_temp1);
        let assign25910_e26945: f64 = (assign25910_e26943 - 80.0);
        let assign25910_e26949: f64 = (-locals.var_temp1);
        let assign25910_e26951: f64 = (assign25910_e26949 - 80.0);
        let assign25910_e26952: f64 = (0.5 * assign25910_e26951);
        let assign25910_e26955: f64 = (-locals.var_temp1);
        let assign25910_e26957: f64 = (assign25910_e26955 - 80.0);
        let assign25910_e26959: f64 = (assign25910_e26957 * 0.3333333333333);
        let assign25910_e26960: f64 = (1.0 + assign25910_e26959);
        let assign25910_e26961: f64 = (assign25910_e26952 * assign25910_e26960);
        let assign25910_e26962: f64 = (1.0 + assign25910_e26961);
        let assign25910_e26963: f64 = (assign25910_e26945 * assign25910_e26962);
        let assign25910_e26964: f64 = (1.0 + assign25910_e26963);
        let assign25910_e26965: f64 = (1.80485e-35 / assign25910_e26964);
        (assign25910_e26965, (-((1.80485e-35 * (((-locals.var_temp1_dn4) * assign25910_e26962) + (assign25910_e26945 * (((0.5 * (-locals.var_temp1_dn4)) * assign25910_e26960) + (assign25910_e26952 * ((-locals.var_temp1_dn4) * 0.3333333333333)))))) / (assign25910_e26964 * assign25910_e26964))), (-((1.80485e-35 * (((-locals.var_temp1_dn6) * assign25910_e26962) + (assign25910_e26945 * (((0.5 * (-locals.var_temp1_dn6)) * assign25910_e26960) + (assign25910_e26952 * ((-locals.var_temp1_dn6) * 0.3333333333333)))))) / (assign25910_e26964 * assign25910_e26964))), (-((1.80485e-35 * (((-locals.var_temp1_dn7) * assign25910_e26962) + (assign25910_e26945 * (((0.5 * (-locals.var_temp1_dn7)) * assign25910_e26960) + (assign25910_e26952 * ((-locals.var_temp1_dn7) * 0.3333333333333)))))) / (assign25910_e26964 * assign25910_e26964))), (-((1.80485e-35 * (((-locals.var_temp1_dn8) * assign25910_e26962) + (assign25910_e26945 * (((0.5 * (-locals.var_temp1_dn8)) * assign25910_e26960) + (assign25910_e26952 * ((-locals.var_temp1_dn8) * 0.3333333333333)))))) / (assign25910_e26964 * assign25910_e26964))), (-((1.80485e-35 * (((-locals.var_temp1_dn9) * assign25910_e26962) + (assign25910_e26945 * (((0.5 * (-locals.var_temp1_dn9)) * assign25910_e26960) + (assign25910_e26952 * ((-locals.var_temp1_dn9) * 0.3333333333333)))))) / (assign25910_e26964 * assign25910_e26964))),)
    } else {
        (locals.var_tp2, locals.var_tp2_dn4, locals.var_tp2_dn6, locals.var_tp2_dn7, locals.var_tp2_dn8, locals.var_tp2_dn9,)
    }
};
        locals.var_tp2 = assign25910_e26967;
        locals.var_tp2_dn4 = assign25910_e26967_d_n4;
        locals.var_tp2_dn6 = assign25910_e26967_d_n6;
        locals.var_tp2_dn7 = assign25910_e26967_d_n7;
        locals.var_tp2_dn8 = assign25910_e26967_d_n8;
        locals.var_tp2_dn9 = assign25910_e26967_d_n9;

        let (assign25920_e26979, assign25920_e26979_d_n4, assign25920_e26979_d_n6, assign25920_e26979_d_n7, assign25920_e26979_d_n8, assign25920_e26979_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25920_e26973: f64 = (1.0 + locals.var_dsi);
        let assign25920_e26976: f64 = (1.0 + locals.var_dgate);
        let assign25920_e26977: f64 = (assign25920_e26973 / assign25920_e26976);
        (assign25920_e26977, (((locals.var_dsi_dn4 * assign25920_e26976) - (assign25920_e26973 * locals.var_dgate_dn4)) / (assign25920_e26976 * assign25920_e26976)), (((locals.var_dsi_dn6 * assign25920_e26976) - (assign25920_e26973 * locals.var_dgate_dn6)) / (assign25920_e26976 * assign25920_e26976)), (((locals.var_dsi_dn7 * assign25920_e26976) - (assign25920_e26973 * locals.var_dgate_dn7)) / (assign25920_e26976 * assign25920_e26976)), (((locals.var_dsi_dn8 * assign25920_e26976) - (assign25920_e26973 * locals.var_dgate_dn8)) / (assign25920_e26976 * assign25920_e26976)), (((locals.var_dsi_dn9 * assign25920_e26976) - (assign25920_e26973 * locals.var_dgate_dn9)) / (assign25920_e26976 * assign25920_e26976)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign25920_e26979;
        locals.var_temp_dn4 = assign25920_e26979_d_n4;
        locals.var_temp_dn6 = assign25920_e26979_d_n6;
        locals.var_temp_dn7 = assign25920_e26979_d_n7;
        locals.var_temp_dn8 = assign25920_e26979_d_n8;
        locals.var_temp_dn9 = assign25920_e26979_d_n9;

        let assign25930_e26982: f64 = if locals.var_temp < 1e-80 { 1.0 } else { 0.0 };
        locals.var_guard734 = assign25930_e26982;

        let (assign25940_e26990, assign25940_e26990_d_n4, assign25940_e26990_d_n6, assign25940_e26990_d_n7, assign25940_e26990_d_n8, assign25940_e26990_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard734 != 0.0)) {
        (1e-80, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign25940_e26990;
        locals.var_temp_dn4 = assign25940_e26990_d_n4;
        locals.var_temp_dn6 = assign25940_e26990_d_n6;
        locals.var_temp_dn7 = assign25940_e26990_d_n7;
        locals.var_temp_dn8 = assign25940_e26990_d_n8;
        locals.var_temp_dn9 = assign25940_e26990_d_n9;

        let (assign25950_e27000, assign25950_e27000_d_n4, assign25950_e27000_d_n6, assign25950_e27000_d_n7, assign25950_e27000_d_n8, assign25950_e27000_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25950_e26997: f64 = (locals.var_vgdu - locals.var_gcvdov_i);
        let assign25950_e26998: f64 = (locals.var_gcdov_i * assign25950_e26997);
        (assign25950_e26998, 0.0, (locals.var_gcdov_i * locals.var_vgdu_dn6), (locals.var_gcdov_i * locals.var_vgdu_dn7), 0.0, (locals.var_gcdov_i * locals.var_vgdu_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign25950_e27000;
        locals.var_temp1_dn4 = assign25950_e27000_d_n4;
        locals.var_temp1_dn6 = assign25950_e27000_d_n6;
        locals.var_temp1_dn7 = assign25950_e27000_d_n7;
        locals.var_temp1_dn8 = assign25950_e27000_d_n8;
        locals.var_temp1_dn9 = assign25950_e27000_d_n9;

        let assign25960_e27002: f64 = (locals.var_temp1).abs();
        let assign25960_e27004: f64 = if assign25960_e27002 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard735 = assign25960_e27004;

        let (assign25970_e27013, assign25970_e27013_d_n4, assign25970_e27013_d_n6, assign25970_e27013_d_n7, assign25970_e27013_d_n8, assign25970_e27013_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard735 != 0.0)) {
        let assign25970_e27011: f64 = (locals.var_temp1).exp();
        (assign25970_e27011, (assign25970_e27011 * locals.var_temp1_dn4), (assign25970_e27011 * locals.var_temp1_dn6), (assign25970_e27011 * locals.var_temp1_dn7), (assign25970_e27011 * locals.var_temp1_dn8), (assign25970_e27011 * locals.var_temp1_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign25970_e27013;
        locals.var_temp2_dn4 = assign25970_e27013_d_n4;
        locals.var_temp2_dn6 = assign25970_e27013_d_n6;
        locals.var_temp2_dn7 = assign25970_e27013_d_n7;
        locals.var_temp2_dn8 = assign25970_e27013_d_n8;
        locals.var_temp2_dn9 = assign25970_e27013_d_n9;

        let assign25980_e27016: f64 = (-80.0);
        let assign25980_e27017: f64 = if locals.var_temp1 < assign25980_e27016 { 1.0 } else { 0.0 };
        locals.var_guard736 = assign25980_e27017;

        let (assign25990_e27053, assign25990_e27053_d_n4, assign25990_e27053_d_n6, assign25990_e27053_d_n7, assign25990_e27053_d_n8, assign25990_e27053_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 != 0.0)) {
        let assign25990_e27029: f64 = (-locals.var_temp1);
        let assign25990_e27031: f64 = (assign25990_e27029 - 80.0);
        let assign25990_e27035: f64 = (-locals.var_temp1);
        let assign25990_e27037: f64 = (assign25990_e27035 - 80.0);
        let assign25990_e27038: f64 = (0.5 * assign25990_e27037);
        let assign25990_e27041: f64 = (-locals.var_temp1);
        let assign25990_e27043: f64 = (assign25990_e27041 - 80.0);
        let assign25990_e27045: f64 = (assign25990_e27043 * 0.3333333333333);
        let assign25990_e27046: f64 = (1.0 + assign25990_e27045);
        let assign25990_e27047: f64 = (assign25990_e27038 * assign25990_e27046);
        let assign25990_e27048: f64 = (1.0 + assign25990_e27047);
        let assign25990_e27049: f64 = (assign25990_e27031 * assign25990_e27048);
        let assign25990_e27050: f64 = (1.0 + assign25990_e27049);
        let assign25990_e27051: f64 = (1.80485e-35 / assign25990_e27050);
        (assign25990_e27051, (-((1.80485e-35 * (((-locals.var_temp1_dn4) * assign25990_e27048) + (assign25990_e27031 * (((0.5 * (-locals.var_temp1_dn4)) * assign25990_e27046) + (assign25990_e27038 * ((-locals.var_temp1_dn4) * 0.3333333333333)))))) / (assign25990_e27050 * assign25990_e27050))), (-((1.80485e-35 * (((-locals.var_temp1_dn6) * assign25990_e27048) + (assign25990_e27031 * (((0.5 * (-locals.var_temp1_dn6)) * assign25990_e27046) + (assign25990_e27038 * ((-locals.var_temp1_dn6) * 0.3333333333333)))))) / (assign25990_e27050 * assign25990_e27050))), (-((1.80485e-35 * (((-locals.var_temp1_dn7) * assign25990_e27048) + (assign25990_e27031 * (((0.5 * (-locals.var_temp1_dn7)) * assign25990_e27046) + (assign25990_e27038 * ((-locals.var_temp1_dn7) * 0.3333333333333)))))) / (assign25990_e27050 * assign25990_e27050))), (-((1.80485e-35 * (((-locals.var_temp1_dn8) * assign25990_e27048) + (assign25990_e27031 * (((0.5 * (-locals.var_temp1_dn8)) * assign25990_e27046) + (assign25990_e27038 * ((-locals.var_temp1_dn8) * 0.3333333333333)))))) / (assign25990_e27050 * assign25990_e27050))), (-((1.80485e-35 * (((-locals.var_temp1_dn9) * assign25990_e27048) + (assign25990_e27031 * (((0.5 * (-locals.var_temp1_dn9)) * assign25990_e27046) + (assign25990_e27038 * ((-locals.var_temp1_dn9) * 0.3333333333333)))))) / (assign25990_e27050 * assign25990_e27050))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign25990_e27053;
        locals.var_temp2_dn4 = assign25990_e27053_d_n4;
        locals.var_temp2_dn6 = assign25990_e27053_d_n6;
        locals.var_temp2_dn7 = assign25990_e27053_d_n7;
        locals.var_temp2_dn8 = assign25990_e27053_d_n8;
        locals.var_temp2_dn9 = assign25990_e27053_d_n9;

        let (assign26000_e27087, assign26000_e27087_d_n4, assign26000_e27087_d_n6, assign26000_e27087_d_n7, assign26000_e27087_d_n8, assign26000_e27087_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard735 == 0.0)) && (locals.var_guard736 == 0.0)) {
        let assign26000_e27067: f64 = (locals.var_temp1 - 80.0);
        let assign26000_e27072: f64 = (locals.var_temp1 - 80.0);
        let assign26000_e27073: f64 = (0.5 * assign26000_e27072);
        let assign26000_e27077: f64 = (locals.var_temp1 - 80.0);
        let assign26000_e27079: f64 = (assign26000_e27077 * 0.3333333333333);
        let assign26000_e27080: f64 = (1.0 + assign26000_e27079);
        let assign26000_e27081: f64 = (assign26000_e27073 * assign26000_e27080);
        let assign26000_e27082: f64 = (1.0 + assign26000_e27081);
        let assign26000_e27083: f64 = (assign26000_e27067 * assign26000_e27082);
        let assign26000_e27084: f64 = (1.0 + assign26000_e27083);
        let assign26000_e27085: f64 = (5.54062e34 * assign26000_e27084);
        (assign26000_e27085, (5.54062e34 * ((locals.var_temp1_dn4 * assign26000_e27082) + (assign26000_e27067 * (((0.5 * locals.var_temp1_dn4) * assign26000_e27080) + (assign26000_e27073 * (locals.var_temp1_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp1_dn6 * assign26000_e27082) + (assign26000_e27067 * (((0.5 * locals.var_temp1_dn6) * assign26000_e27080) + (assign26000_e27073 * (locals.var_temp1_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp1_dn7 * assign26000_e27082) + (assign26000_e27067 * (((0.5 * locals.var_temp1_dn7) * assign26000_e27080) + (assign26000_e27073 * (locals.var_temp1_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp1_dn8 * assign26000_e27082) + (assign26000_e27067 * (((0.5 * locals.var_temp1_dn8) * assign26000_e27080) + (assign26000_e27073 * (locals.var_temp1_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp1_dn9 * assign26000_e27082) + (assign26000_e27067 * (((0.5 * locals.var_temp1_dn9) * assign26000_e27080) + (assign26000_e27073 * (locals.var_temp1_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign26000_e27087;
        locals.var_temp2_dn4 = assign26000_e27087_d_n4;
        locals.var_temp2_dn6 = assign26000_e27087_d_n6;
        locals.var_temp2_dn7 = assign26000_e27087_d_n7;
        locals.var_temp2_dn8 = assign26000_e27087_d_n8;
        locals.var_temp2_dn9 = assign26000_e27087_d_n9;

    }

    pub(super) fn stamp_transient_block_67(
        locals: &mut StampLocals,
    ) {
        let (assign26010_e27097, assign26010_e27097_d_n4, assign26010_e27097_d_n6, assign26010_e27097_d_n7, assign26010_e27097_d_n8, assign26010_e27097_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign26010_e27093: f64 = (locals.var_gcdov_i * locals.var_vsdu);
        let assign26010_e27095: f64 = (assign26010_e27093 + locals.var_temp1);
        (assign26010_e27095, locals.var_temp1_dn4, ((locals.var_gcdov_i * locals.var_vsdu_dn6) + locals.var_temp1_dn6), ((locals.var_gcdov_i * locals.var_vsdu_dn7) + locals.var_temp1_dn7), locals.var_temp1_dn8, locals.var_temp1_dn9,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign26010_e27097;
        locals.var_temp3_dn4 = assign26010_e27097_d_n4;
        locals.var_temp3_dn6 = assign26010_e27097_d_n6;
        locals.var_temp3_dn7 = assign26010_e27097_d_n7;
        locals.var_temp3_dn8 = assign26010_e27097_d_n8;
        locals.var_temp3_dn9 = assign26010_e27097_d_n9;

        let assign26020_e27099: f64 = (locals.var_temp3).abs();
        let assign26020_e27101: f64 = if assign26020_e27099 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard737 = assign26020_e27101;

        let (assign26030_e27110, assign26030_e27110_d_n4, assign26030_e27110_d_n6, assign26030_e27110_d_n7, assign26030_e27110_d_n8, assign26030_e27110_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard737 != 0.0)) {
        let assign26030_e27108: f64 = (locals.var_temp3).exp();
        (assign26030_e27108, (assign26030_e27108 * locals.var_temp3_dn4), (assign26030_e27108 * locals.var_temp3_dn6), (assign26030_e27108 * locals.var_temp3_dn7), (assign26030_e27108 * locals.var_temp3_dn8), (assign26030_e27108 * locals.var_temp3_dn9),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign26030_e27110;
        locals.var_temp4_dn4 = assign26030_e27110_d_n4;
        locals.var_temp4_dn6 = assign26030_e27110_d_n6;
        locals.var_temp4_dn7 = assign26030_e27110_d_n7;
        locals.var_temp4_dn8 = assign26030_e27110_d_n8;
        locals.var_temp4_dn9 = assign26030_e27110_d_n9;

        let assign26040_e27113: f64 = (-80.0);
        let assign26040_e27114: f64 = if locals.var_temp3 < assign26040_e27113 { 1.0 } else { 0.0 };
        locals.var_guard738 = assign26040_e27114;

        let (assign26050_e27150, assign26050_e27150_d_n4, assign26050_e27150_d_n6, assign26050_e27150_d_n7, assign26050_e27150_d_n8, assign26050_e27150_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign26050_e27126: f64 = (-locals.var_temp3);
        let assign26050_e27128: f64 = (assign26050_e27126 - 80.0);
        let assign26050_e27132: f64 = (-locals.var_temp3);
        let assign26050_e27134: f64 = (assign26050_e27132 - 80.0);
        let assign26050_e27135: f64 = (0.5 * assign26050_e27134);
        let assign26050_e27138: f64 = (-locals.var_temp3);
        let assign26050_e27140: f64 = (assign26050_e27138 - 80.0);
        let assign26050_e27142: f64 = (assign26050_e27140 * 0.3333333333333);
        let assign26050_e27143: f64 = (1.0 + assign26050_e27142);
        let assign26050_e27144: f64 = (assign26050_e27135 * assign26050_e27143);
        let assign26050_e27145: f64 = (1.0 + assign26050_e27144);
        let assign26050_e27146: f64 = (assign26050_e27128 * assign26050_e27145);
        let assign26050_e27147: f64 = (1.0 + assign26050_e27146);
        let assign26050_e27148: f64 = (1.80485e-35 / assign26050_e27147);
        (assign26050_e27148, (-((1.80485e-35 * (((-locals.var_temp3_dn4) * assign26050_e27145) + (assign26050_e27128 * (((0.5 * (-locals.var_temp3_dn4)) * assign26050_e27143) + (assign26050_e27135 * ((-locals.var_temp3_dn4) * 0.3333333333333)))))) / (assign26050_e27147 * assign26050_e27147))), (-((1.80485e-35 * (((-locals.var_temp3_dn6) * assign26050_e27145) + (assign26050_e27128 * (((0.5 * (-locals.var_temp3_dn6)) * assign26050_e27143) + (assign26050_e27135 * ((-locals.var_temp3_dn6) * 0.3333333333333)))))) / (assign26050_e27147 * assign26050_e27147))), (-((1.80485e-35 * (((-locals.var_temp3_dn7) * assign26050_e27145) + (assign26050_e27128 * (((0.5 * (-locals.var_temp3_dn7)) * assign26050_e27143) + (assign26050_e27135 * ((-locals.var_temp3_dn7) * 0.3333333333333)))))) / (assign26050_e27147 * assign26050_e27147))), (-((1.80485e-35 * (((-locals.var_temp3_dn8) * assign26050_e27145) + (assign26050_e27128 * (((0.5 * (-locals.var_temp3_dn8)) * assign26050_e27143) + (assign26050_e27135 * ((-locals.var_temp3_dn8) * 0.3333333333333)))))) / (assign26050_e27147 * assign26050_e27147))), (-((1.80485e-35 * (((-locals.var_temp3_dn9) * assign26050_e27145) + (assign26050_e27128 * (((0.5 * (-locals.var_temp3_dn9)) * assign26050_e27143) + (assign26050_e27135 * ((-locals.var_temp3_dn9) * 0.3333333333333)))))) / (assign26050_e27147 * assign26050_e27147))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign26050_e27150;
        locals.var_temp4_dn4 = assign26050_e27150_d_n4;
        locals.var_temp4_dn6 = assign26050_e27150_d_n6;
        locals.var_temp4_dn7 = assign26050_e27150_d_n7;
        locals.var_temp4_dn8 = assign26050_e27150_d_n8;
        locals.var_temp4_dn9 = assign26050_e27150_d_n9;

        let (assign26060_e27184, assign26060_e27184_d_n4, assign26060_e27184_d_n6, assign26060_e27184_d_n7, assign26060_e27184_d_n8, assign26060_e27184_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 == 0.0)) {
        let assign26060_e27164: f64 = (locals.var_temp3 - 80.0);
        let assign26060_e27169: f64 = (locals.var_temp3 - 80.0);
        let assign26060_e27170: f64 = (0.5 * assign26060_e27169);
        let assign26060_e27174: f64 = (locals.var_temp3 - 80.0);
        let assign26060_e27176: f64 = (assign26060_e27174 * 0.3333333333333);
        let assign26060_e27177: f64 = (1.0 + assign26060_e27176);
        let assign26060_e27178: f64 = (assign26060_e27170 * assign26060_e27177);
        let assign26060_e27179: f64 = (1.0 + assign26060_e27178);
        let assign26060_e27180: f64 = (assign26060_e27164 * assign26060_e27179);
        let assign26060_e27181: f64 = (1.0 + assign26060_e27180);
        let assign26060_e27182: f64 = (5.54062e34 * assign26060_e27181);
        (assign26060_e27182, (5.54062e34 * ((locals.var_temp3_dn4 * assign26060_e27179) + (assign26060_e27164 * (((0.5 * locals.var_temp3_dn4) * assign26060_e27177) + (assign26060_e27170 * (locals.var_temp3_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn6 * assign26060_e27179) + (assign26060_e27164 * (((0.5 * locals.var_temp3_dn6) * assign26060_e27177) + (assign26060_e27170 * (locals.var_temp3_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn7 * assign26060_e27179) + (assign26060_e27164 * (((0.5 * locals.var_temp3_dn7) * assign26060_e27177) + (assign26060_e27170 * (locals.var_temp3_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn8 * assign26060_e27179) + (assign26060_e27164 * (((0.5 * locals.var_temp3_dn8) * assign26060_e27177) + (assign26060_e27170 * (locals.var_temp3_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn9 * assign26060_e27179) + (assign26060_e27164 * (((0.5 * locals.var_temp3_dn9) * assign26060_e27177) + (assign26060_e27170 * (locals.var_temp3_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign26060_e27184;
        locals.var_temp4_dn4 = assign26060_e27184_d_n4;
        locals.var_temp4_dn6 = assign26060_e27184_d_n6;
        locals.var_temp4_dn7 = assign26060_e27184_d_n7;
        locals.var_temp4_dn8 = assign26060_e27184_d_n8;
        locals.var_temp4_dn9 = assign26060_e27184_d_n9;

        let (assign26070_e27215, assign26070_e27215_d_n4, assign26070_e27215_d_n6, assign26070_e27215_d_n7, assign26070_e27215_d_n8, assign26070_e27215_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign26070_e27190: f64 = (locals.var_igoveff * locals.var_tp);
        let assign26070_e27192: f64 = (locals.var_temp).ln();
        let assign26070_e27193: f64 = (assign26070_e27190 * assign26070_e27192);
        let assign26070_e27196: f64 = (1.0 + locals.var_temp2);
        let assign26070_e27197: f64 = (assign26070_e27193 * assign26070_e27196);
        let assign26070_e27200: f64 = (1.0 + locals.var_temp4);
        let assign26070_e27201: f64 = (assign26070_e27197 / assign26070_e27200);
        let assign26070_e27204: f64 = (locals.var_igovefffowler * locals.var_tp2);
        let assign26070_e27207: f64 = (1.0 + locals.var_temp2);
        let assign26070_e27208: f64 = (assign26070_e27204 * assign26070_e27207);
        let assign26070_e27211: f64 = (1.0 + locals.var_temp4);
        let assign26070_e27212: f64 = (assign26070_e27208 / assign26070_e27211);
        let assign26070_e27213: f64 = (assign26070_e27201 - assign26070_e27212);
        (assign26070_e27213, ((((((((((locals.var_igoveff_dn4 * locals.var_tp) + (locals.var_igoveff * locals.var_tp_dn4)) * assign26070_e27192) + (assign26070_e27190 * (locals.var_temp_dn4 / locals.var_temp))) * assign26070_e27196) + (assign26070_e27193 * locals.var_temp2_dn4)) * assign26070_e27200) - (assign26070_e27197 * locals.var_temp4_dn4)) / (assign26070_e27200 * assign26070_e27200)) - (((((((locals.var_igovefffowler_dn4 * locals.var_tp2) + (locals.var_igovefffowler * locals.var_tp2_dn4)) * assign26070_e27207) + (assign26070_e27204 * locals.var_temp2_dn4)) * assign26070_e27211) - (assign26070_e27208 * locals.var_temp4_dn4)) / (assign26070_e27211 * assign26070_e27211))), ((((((((((locals.var_igoveff_dn6 * locals.var_tp) + (locals.var_igoveff * locals.var_tp_dn6)) * assign26070_e27192) + (assign26070_e27190 * (locals.var_temp_dn6 / locals.var_temp))) * assign26070_e27196) + (assign26070_e27193 * locals.var_temp2_dn6)) * assign26070_e27200) - (assign26070_e27197 * locals.var_temp4_dn6)) / (assign26070_e27200 * assign26070_e27200)) - (((((((locals.var_igovefffowler_dn6 * locals.var_tp2) + (locals.var_igovefffowler * locals.var_tp2_dn6)) * assign26070_e27207) + (assign26070_e27204 * locals.var_temp2_dn6)) * assign26070_e27211) - (assign26070_e27208 * locals.var_temp4_dn6)) / (assign26070_e27211 * assign26070_e27211))), ((((((((((locals.var_igoveff_dn7 * locals.var_tp) + (locals.var_igoveff * locals.var_tp_dn7)) * assign26070_e27192) + (assign26070_e27190 * (locals.var_temp_dn7 / locals.var_temp))) * assign26070_e27196) + (assign26070_e27193 * locals.var_temp2_dn7)) * assign26070_e27200) - (assign26070_e27197 * locals.var_temp4_dn7)) / (assign26070_e27200 * assign26070_e27200)) - (((((((locals.var_igovefffowler_dn7 * locals.var_tp2) + (locals.var_igovefffowler * locals.var_tp2_dn7)) * assign26070_e27207) + (assign26070_e27204 * locals.var_temp2_dn7)) * assign26070_e27211) - (assign26070_e27208 * locals.var_temp4_dn7)) / (assign26070_e27211 * assign26070_e27211))), ((((((((((locals.var_igoveff_dn8 * locals.var_tp) + (locals.var_igoveff * locals.var_tp_dn8)) * assign26070_e27192) + (assign26070_e27190 * (locals.var_temp_dn8 / locals.var_temp))) * assign26070_e27196) + (assign26070_e27193 * locals.var_temp2_dn8)) * assign26070_e27200) - (assign26070_e27197 * locals.var_temp4_dn8)) / (assign26070_e27200 * assign26070_e27200)) - (((((((locals.var_igovefffowler_dn8 * locals.var_tp2) + (locals.var_igovefffowler * locals.var_tp2_dn8)) * assign26070_e27207) + (assign26070_e27204 * locals.var_temp2_dn8)) * assign26070_e27211) - (assign26070_e27208 * locals.var_temp4_dn8)) / (assign26070_e27211 * assign26070_e27211))), ((((((((((locals.var_igoveff_dn9 * locals.var_tp) + (locals.var_igoveff * locals.var_tp_dn9)) * assign26070_e27192) + (assign26070_e27190 * (locals.var_temp_dn9 / locals.var_temp))) * assign26070_e27196) + (assign26070_e27193 * locals.var_temp2_dn9)) * assign26070_e27200) - (assign26070_e27197 * locals.var_temp4_dn9)) / (assign26070_e27200 * assign26070_e27200)) - (((((((locals.var_igovefffowler_dn9 * locals.var_tp2) + (locals.var_igovefffowler * locals.var_tp2_dn9)) * assign26070_e27207) + (assign26070_e27204 * locals.var_temp2_dn9)) * assign26070_e27211) - (assign26070_e27208 * locals.var_temp4_dn9)) / (assign26070_e27211 * assign26070_e27211))),)
    } else {
        (locals.var_igsov, locals.var_igsov_dn4, locals.var_igsov_dn6, locals.var_igsov_dn7, locals.var_igsov_dn8, locals.var_igsov_dn9,)
    }
};
        locals.var_igsov = assign26070_e27215;
        locals.var_igsov_dn4 = assign26070_e27215_d_n4;
        locals.var_igsov_dn6 = assign26070_e27215_d_n6;
        locals.var_igsov_dn7 = assign26070_e27215_d_n7;
        locals.var_igsov_dn8 = assign26070_e27215_d_n8;
        locals.var_igsov_dn9 = assign26070_e27215_d_n9;

        let assign26080_e27222: f64 = if ((locals.var_igovinvd_i > 0.0) || (locals.var_igovaccd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard739 = assign26080_e27222;

        let (assign26090_e27230, assign26090_e27230_d_n4, assign26090_e27230_d_n6, assign26090_e27230_d_n7, assign26090_e27230_d_n8, assign26090_e27230_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26090_e27228: f64 = (locals.var_vovd + locals.var_dov);
        (assign26090_e27228, (locals.var_vovd_dn4 + locals.var_dov_dn4), (locals.var_vovd_dn6 + locals.var_dov_dn6), (locals.var_vovd_dn7 + locals.var_dov_dn7), (locals.var_vovd_dn8 + locals.var_dov_dn8), (locals.var_vovd_dn9 + locals.var_dov_dn9),)
    } else {
        (locals.var_arg2mina, locals.var_arg2mina_dn4, locals.var_arg2mina_dn6, locals.var_arg2mina_dn7, locals.var_arg2mina_dn8, locals.var_arg2mina_dn9,)
    }
};
        locals.var_arg2mina = assign26090_e27230;
        locals.var_arg2mina_dn4 = assign26090_e27230_d_n4;
        locals.var_arg2mina_dn6 = assign26090_e27230_d_n6;
        locals.var_arg2mina_dn7 = assign26090_e27230_d_n7;
        locals.var_arg2mina_dn8 = assign26090_e27230_d_n8;
        locals.var_arg2mina_dn9 = assign26090_e27230_d_n9;

        let (assign26100_e27251, assign26100_e27251_d_n4, assign26100_e27251_d_n6, assign26100_e27251_d_n7, assign26100_e27251_d_n8, assign26100_e27251_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26100_e27237: f64 = locals.var_arg2mina;
        let assign26100_e27240: f64 = (-locals.var_arg2mina);
        let assign26100_e27243: f64 = (-locals.var_arg2mina);
        let assign26100_e27244: f64 = (assign26100_e27240 * assign26100_e27243);
        let assign26100_e27246: f64 = (assign26100_e27244 + 0.01);
        let assign26100_e27247: f64 = (assign26100_e27246).sqrt();
        let assign26100_e27248: f64 = (assign26100_e27237 - assign26100_e27247);
        let assign26100_e27249: f64 = (0.5 * assign26100_e27248);
        (assign26100_e27249, (0.5 * (locals.var_arg2mina_dn4 - ((((-locals.var_arg2mina_dn4) * assign26100_e27243) + (assign26100_e27240 * (-locals.var_arg2mina_dn4))) / (2.0 * assign26100_e27247)))), (0.5 * (locals.var_arg2mina_dn6 - ((((-locals.var_arg2mina_dn6) * assign26100_e27243) + (assign26100_e27240 * (-locals.var_arg2mina_dn6))) / (2.0 * assign26100_e27247)))), (0.5 * (locals.var_arg2mina_dn7 - ((((-locals.var_arg2mina_dn7) * assign26100_e27243) + (assign26100_e27240 * (-locals.var_arg2mina_dn7))) / (2.0 * assign26100_e27247)))), (0.5 * (locals.var_arg2mina_dn8 - ((((-locals.var_arg2mina_dn8) * assign26100_e27243) + (assign26100_e27240 * (-locals.var_arg2mina_dn8))) / (2.0 * assign26100_e27247)))), (0.5 * (locals.var_arg2mina_dn9 - ((((-locals.var_arg2mina_dn9) * assign26100_e27243) + (assign26100_e27240 * (-locals.var_arg2mina_dn9))) / (2.0 * assign26100_e27247)))),)
    } else {
        (locals.var_psi_t, locals.var_psi_t_dn4, locals.var_psi_t_dn6, locals.var_psi_t_dn7, locals.var_psi_t_dn8, locals.var_psi_t_dn9,)
    }
};
        locals.var_psi_t = assign26100_e27251;
        locals.var_psi_t_dn4 = assign26100_e27251_d_n4;
        locals.var_psi_t_dn6 = assign26100_e27251_d_n6;
        locals.var_psi_t_dn7 = assign26100_e27251_d_n7;
        locals.var_psi_t_dn8 = assign26100_e27251_d_n8;
        locals.var_psi_t_dn9 = assign26100_e27251_d_n9;

        let (assign26110_e27264, assign26110_e27264_d_n4, assign26110_e27264_d_n6, assign26110_e27264_d_n7, assign26110_e27264_d_n8, assign26110_e27264_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26110_e27257: f64 = (locals.var_vovd * locals.var_vovd);
        let assign26110_e27259: f64 = (assign26110_e27257 + 0.0001);
        let assign26110_e27260: f64 = (assign26110_e27259).sqrt();
        let assign26110_e27262: f64 = (assign26110_e27260 * locals.var_inv_chib);
        (assign26110_e27262, ((((locals.var_vovd_dn4 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn4)) / (2.0 * assign26110_e27260)) * locals.var_inv_chib), ((((locals.var_vovd_dn6 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn6)) / (2.0 * assign26110_e27260)) * locals.var_inv_chib), ((((locals.var_vovd_dn7 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn7)) / (2.0 * assign26110_e27260)) * locals.var_inv_chib), ((((locals.var_vovd_dn8 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn8)) / (2.0 * assign26110_e27260)) * locals.var_inv_chib), ((((locals.var_vovd_dn9 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn9)) / (2.0 * assign26110_e27260)) * locals.var_inv_chib),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign26110_e27264;
        locals.var_zg_dn4 = assign26110_e27264_d_n4;
        locals.var_zg_dn6 = assign26110_e27264_d_n6;
        locals.var_zg_dn7 = assign26110_e27264_d_n7;
        locals.var_zg_dn8 = assign26110_e27264_d_n8;
        locals.var_zg_dn9 = assign26110_e27264_d_n9;

        let assign26120_e27267: f64 = (0.5 * locals.var_xgd_ov);
        let assign26120_e27268: f64 = (assign26120_e27267).abs();
        let assign26120_e27270: f64 = if assign26120_e27268 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard740 = assign26120_e27270;

        let (assign26130_e27281, assign26130_e27281_d_n4, assign26130_e27281_d_n6, assign26130_e27281_d_n7, assign26130_e27281_d_n8, assign26130_e27281_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign26130_e27278: f64 = (0.5 * locals.var_xgd_ov);
        let assign26130_e27279: f64 = (assign26130_e27278).exp();
        (assign26130_e27279, (assign26130_e27279 * (0.5 * locals.var_xgd_ov_dn4)), (assign26130_e27279 * (0.5 * locals.var_xgd_ov_dn6)), (assign26130_e27279 * (0.5 * locals.var_xgd_ov_dn7)), (assign26130_e27279 * (0.5 * locals.var_xgd_ov_dn8)), (assign26130_e27279 * (0.5 * locals.var_xgd_ov_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26130_e27281;
        locals.var_temp_dn4 = assign26130_e27281_d_n4;
        locals.var_temp_dn6 = assign26130_e27281_d_n6;
        locals.var_temp_dn7 = assign26130_e27281_d_n7;
        locals.var_temp_dn8 = assign26130_e27281_d_n8;
        locals.var_temp_dn9 = assign26130_e27281_d_n9;

        let assign26140_e27284: f64 = (0.5 * locals.var_xgd_ov);
        let assign26140_e27286: f64 = (-80.0);
        let assign26140_e27287: f64 = if assign26140_e27284 < assign26140_e27286 { 1.0 } else { 0.0 };
        locals.var_guard741 = assign26140_e27287;

        let (assign26150_e27329, assign26150_e27329_d_n4, assign26150_e27329_d_n6, assign26150_e27329_d_n7, assign26150_e27329_d_n8, assign26150_e27329_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard740 == 0.0)) && (locals.var_guard741 != 0.0)) {
        let assign26150_e27300: f64 = (0.5 * locals.var_xgd_ov);
        let assign26150_e27301: f64 = (-assign26150_e27300);
        let assign26150_e27303: f64 = (assign26150_e27301 - 80.0);
        let assign26150_e27308: f64 = (0.5 * locals.var_xgd_ov);
        let assign26150_e27309: f64 = (-assign26150_e27308);
        let assign26150_e27311: f64 = (assign26150_e27309 - 80.0);
        let assign26150_e27312: f64 = (0.5 * assign26150_e27311);
        let assign26150_e27316: f64 = (0.5 * locals.var_xgd_ov);
        let assign26150_e27317: f64 = (-assign26150_e27316);
        let assign26150_e27319: f64 = (assign26150_e27317 - 80.0);
        let assign26150_e27321: f64 = (assign26150_e27319 * 0.3333333333333);
        let assign26150_e27322: f64 = (1.0 + assign26150_e27321);
        let assign26150_e27323: f64 = (assign26150_e27312 * assign26150_e27322);
        let assign26150_e27324: f64 = (1.0 + assign26150_e27323);
        let assign26150_e27325: f64 = (assign26150_e27303 * assign26150_e27324);
        let assign26150_e27326: f64 = (1.0 + assign26150_e27325);
        let assign26150_e27327: f64 = (1.80485e-35 / assign26150_e27326);
        (assign26150_e27327, (-((1.80485e-35 * (((-(0.5 * locals.var_xgd_ov_dn4)) * assign26150_e27324) + (assign26150_e27303 * (((0.5 * (-(0.5 * locals.var_xgd_ov_dn4))) * assign26150_e27322) + (assign26150_e27312 * ((-(0.5 * locals.var_xgd_ov_dn4)) * 0.3333333333333)))))) / (assign26150_e27326 * assign26150_e27326))), (-((1.80485e-35 * (((-(0.5 * locals.var_xgd_ov_dn6)) * assign26150_e27324) + (assign26150_e27303 * (((0.5 * (-(0.5 * locals.var_xgd_ov_dn6))) * assign26150_e27322) + (assign26150_e27312 * ((-(0.5 * locals.var_xgd_ov_dn6)) * 0.3333333333333)))))) / (assign26150_e27326 * assign26150_e27326))), (-((1.80485e-35 * (((-(0.5 * locals.var_xgd_ov_dn7)) * assign26150_e27324) + (assign26150_e27303 * (((0.5 * (-(0.5 * locals.var_xgd_ov_dn7))) * assign26150_e27322) + (assign26150_e27312 * ((-(0.5 * locals.var_xgd_ov_dn7)) * 0.3333333333333)))))) / (assign26150_e27326 * assign26150_e27326))), (-((1.80485e-35 * (((-(0.5 * locals.var_xgd_ov_dn8)) * assign26150_e27324) + (assign26150_e27303 * (((0.5 * (-(0.5 * locals.var_xgd_ov_dn8))) * assign26150_e27322) + (assign26150_e27312 * ((-(0.5 * locals.var_xgd_ov_dn8)) * 0.3333333333333)))))) / (assign26150_e27326 * assign26150_e27326))), (-((1.80485e-35 * (((-(0.5 * locals.var_xgd_ov_dn9)) * assign26150_e27324) + (assign26150_e27303 * (((0.5 * (-(0.5 * locals.var_xgd_ov_dn9))) * assign26150_e27322) + (assign26150_e27312 * ((-(0.5 * locals.var_xgd_ov_dn9)) * 0.3333333333333)))))) / (assign26150_e27326 * assign26150_e27326))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26150_e27329;
        locals.var_temp_dn4 = assign26150_e27329_d_n4;
        locals.var_temp_dn6 = assign26150_e27329_d_n6;
        locals.var_temp_dn7 = assign26150_e27329_d_n7;
        locals.var_temp_dn8 = assign26150_e27329_d_n8;
        locals.var_temp_dn9 = assign26150_e27329_d_n9;

        let (assign26160_e27369, assign26160_e27369_d_n4, assign26160_e27369_d_n6, assign26160_e27369_d_n7, assign26160_e27369_d_n8, assign26160_e27369_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard740 == 0.0)) && (locals.var_guard741 == 0.0)) {
        let assign26160_e27343: f64 = (0.5 * locals.var_xgd_ov);
        let assign26160_e27345: f64 = (assign26160_e27343 - 80.0);
        let assign26160_e27350: f64 = (0.5 * locals.var_xgd_ov);
        let assign26160_e27352: f64 = (assign26160_e27350 - 80.0);
        let assign26160_e27353: f64 = (0.5 * assign26160_e27352);
        let assign26160_e27357: f64 = (0.5 * locals.var_xgd_ov);
        let assign26160_e27359: f64 = (assign26160_e27357 - 80.0);
        let assign26160_e27361: f64 = (assign26160_e27359 * 0.3333333333333);
        let assign26160_e27362: f64 = (1.0 + assign26160_e27361);
        let assign26160_e27363: f64 = (assign26160_e27353 * assign26160_e27362);
        let assign26160_e27364: f64 = (1.0 + assign26160_e27363);
        let assign26160_e27365: f64 = (assign26160_e27345 * assign26160_e27364);
        let assign26160_e27366: f64 = (1.0 + assign26160_e27365);
        let assign26160_e27367: f64 = (5.54062e34 * assign26160_e27366);
        (assign26160_e27367, (5.54062e34 * (((0.5 * locals.var_xgd_ov_dn4) * assign26160_e27364) + (assign26160_e27345 * (((0.5 * (0.5 * locals.var_xgd_ov_dn4)) * assign26160_e27362) + (assign26160_e27353 * ((0.5 * locals.var_xgd_ov_dn4) * 0.3333333333333)))))), (5.54062e34 * (((0.5 * locals.var_xgd_ov_dn6) * assign26160_e27364) + (assign26160_e27345 * (((0.5 * (0.5 * locals.var_xgd_ov_dn6)) * assign26160_e27362) + (assign26160_e27353 * ((0.5 * locals.var_xgd_ov_dn6) * 0.3333333333333)))))), (5.54062e34 * (((0.5 * locals.var_xgd_ov_dn7) * assign26160_e27364) + (assign26160_e27345 * (((0.5 * (0.5 * locals.var_xgd_ov_dn7)) * assign26160_e27362) + (assign26160_e27353 * ((0.5 * locals.var_xgd_ov_dn7) * 0.3333333333333)))))), (5.54062e34 * (((0.5 * locals.var_xgd_ov_dn8) * assign26160_e27364) + (assign26160_e27345 * (((0.5 * (0.5 * locals.var_xgd_ov_dn8)) * assign26160_e27362) + (assign26160_e27353 * ((0.5 * locals.var_xgd_ov_dn8) * 0.3333333333333)))))), (5.54062e34 * (((0.5 * locals.var_xgd_ov_dn9) * assign26160_e27364) + (assign26160_e27345 * (((0.5 * (0.5 * locals.var_xgd_ov_dn9)) * assign26160_e27362) + (assign26160_e27353 * ((0.5 * locals.var_xgd_ov_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26160_e27369;
        locals.var_temp_dn4 = assign26160_e27369_d_n4;
        locals.var_temp_dn6 = assign26160_e27369_d_n6;
        locals.var_temp_dn7 = assign26160_e27369_d_n7;
        locals.var_temp_dn8 = assign26160_e27369_d_n8;
        locals.var_temp_dn9 = assign26160_e27369_d_n9;

        let (assign26170_e27379, assign26170_e27379_d_n4, assign26170_e27379_d_n6, assign26170_e27379_d_n7, assign26170_e27379_d_n8, assign26170_e27379_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26170_e27376: f64 = (1.0 + locals.var_temp);
        let assign26170_e27377: f64 = (1.0 / assign26170_e27376);
        (assign26170_e27377, (-(locals.var_temp_dn4 / (assign26170_e27376 * assign26170_e27376))), (-(locals.var_temp_dn6 / (assign26170_e27376 * assign26170_e27376))), (-(locals.var_temp_dn7 / (assign26170_e27376 * assign26170_e27376))), (-(locals.var_temp_dn8 / (assign26170_e27376 * assign26170_e27376))), (-(locals.var_temp_dn9 / (assign26170_e27376 * assign26170_e27376))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign26170_e27379;
        locals.var_temp1_dn4 = assign26170_e27379_d_n4;
        locals.var_temp1_dn6 = assign26170_e27379_d_n6;
        locals.var_temp1_dn7 = assign26170_e27379_d_n7;
        locals.var_temp1_dn8 = assign26170_e27379_d_n8;
        locals.var_temp1_dn9 = assign26170_e27379_d_n9;

        let (assign26180_e27387, assign26180_e27387_d_n4, assign26180_e27387_d_n6, assign26180_e27387_d_n7, assign26180_e27387_d_n8, assign26180_e27387_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26180_e27385: f64 = (1.0 - locals.var_temp1);
        (assign26180_e27385, (-locals.var_temp1_dn4), (-locals.var_temp1_dn6), (-locals.var_temp1_dn7), (-locals.var_temp1_dn8), (-locals.var_temp1_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign26180_e27387;
        locals.var_temp2_dn4 = assign26180_e27387_d_n4;
        locals.var_temp2_dn6 = assign26180_e27387_d_n6;
        locals.var_temp2_dn7 = assign26180_e27387_d_n7;
        locals.var_temp2_dn8 = assign26180_e27387_d_n8;
        locals.var_temp2_dn9 = assign26180_e27387_d_n9;

        let (assign26190_e27399, assign26190_e27399_d_n4, assign26190_e27399_d_n6, assign26190_e27399_d_n7, assign26190_e27399_d_n8, assign26190_e27399_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26190_e27393: f64 = (locals.var_gc2ovacc_i * locals.var_temp1);
        let assign26190_e27396: f64 = (locals.var_gc2ovinv_i * locals.var_temp2);
        let assign26190_e27397: f64 = (assign26190_e27393 + assign26190_e27396);
        (assign26190_e27397, ((locals.var_gc2ovacc_i * locals.var_temp1_dn4) + (locals.var_gc2ovinv_i * locals.var_temp2_dn4)), ((locals.var_gc2ovacc_i * locals.var_temp1_dn6) + (locals.var_gc2ovinv_i * locals.var_temp2_dn6)), ((locals.var_gc2ovacc_i * locals.var_temp1_dn7) + (locals.var_gc2ovinv_i * locals.var_temp2_dn7)), ((locals.var_gc2ovacc_i * locals.var_temp1_dn8) + (locals.var_gc2ovinv_i * locals.var_temp2_dn8)), ((locals.var_gc2ovacc_i * locals.var_temp1_dn9) + (locals.var_gc2ovinv_i * locals.var_temp2_dn9)),)
    } else {
        (locals.var_gc2oveff, locals.var_gc2oveff_dn4, locals.var_gc2oveff_dn6, locals.var_gc2oveff_dn7, locals.var_gc2oveff_dn8, locals.var_gc2oveff_dn9,)
    }
};
        locals.var_gc2oveff = assign26190_e27399;
        locals.var_gc2oveff_dn4 = assign26190_e27399_d_n4;
        locals.var_gc2oveff_dn6 = assign26190_e27399_d_n6;
        locals.var_gc2oveff_dn7 = assign26190_e27399_d_n7;
        locals.var_gc2oveff_dn8 = assign26190_e27399_d_n8;
        locals.var_gc2oveff_dn9 = assign26190_e27399_d_n9;

        let (assign26200_e27411, assign26200_e27411_d_n4, assign26200_e27411_d_n6, assign26200_e27411_d_n7, assign26200_e27411_d_n8, assign26200_e27411_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26200_e27405: f64 = (locals.var_gc3ovacc_i * locals.var_temp1);
        let assign26200_e27408: f64 = (locals.var_gc3ovinv_i * locals.var_temp2);
        let assign26200_e27409: f64 = (assign26200_e27405 + assign26200_e27408);
        (assign26200_e27409, ((locals.var_gc3ovacc_i * locals.var_temp1_dn4) + (locals.var_gc3ovinv_i * locals.var_temp2_dn4)), ((locals.var_gc3ovacc_i * locals.var_temp1_dn6) + (locals.var_gc3ovinv_i * locals.var_temp2_dn6)), ((locals.var_gc3ovacc_i * locals.var_temp1_dn7) + (locals.var_gc3ovinv_i * locals.var_temp2_dn7)), ((locals.var_gc3ovacc_i * locals.var_temp1_dn8) + (locals.var_gc3ovinv_i * locals.var_temp2_dn8)), ((locals.var_gc3ovacc_i * locals.var_temp1_dn9) + (locals.var_gc3ovinv_i * locals.var_temp2_dn9)),)
    } else {
        (locals.var_gc3oveff, locals.var_gc3oveff_dn4, locals.var_gc3oveff_dn6, locals.var_gc3oveff_dn7, locals.var_gc3oveff_dn8, locals.var_gc3oveff_dn9,)
    }
};
        locals.var_gc3oveff = assign26200_e27411;
        locals.var_gc3oveff_dn4 = assign26200_e27411_d_n4;
        locals.var_gc3oveff_dn6 = assign26200_e27411_d_n6;
        locals.var_gc3oveff_dn7 = assign26200_e27411_d_n7;
        locals.var_gc3oveff_dn8 = assign26200_e27411_d_n8;
        locals.var_gc3oveff_dn9 = assign26200_e27411_d_n9;

        let (assign26210_e27423, assign26210_e27423_d_n4, assign26210_e27423_d_n6, assign26210_e27423_d_n7, assign26210_e27423_d_n8, assign26210_e27423_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26210_e27417: f64 = (locals.var_gcqovacc * locals.var_temp1);
        let assign26210_e27420: f64 = (locals.var_gcqovinv * locals.var_temp2);
        let assign26210_e27421: f64 = (assign26210_e27417 + assign26210_e27420);
        (assign26210_e27421, ((locals.var_gcqovacc * locals.var_temp1_dn4) + (locals.var_gcqovinv * locals.var_temp2_dn4)), ((locals.var_gcqovacc * locals.var_temp1_dn6) + (locals.var_gcqovinv * locals.var_temp2_dn6)), ((locals.var_gcqovacc * locals.var_temp1_dn7) + (locals.var_gcqovinv * locals.var_temp2_dn7)), ((locals.var_gcqovacc * locals.var_temp1_dn8) + (locals.var_gcqovinv * locals.var_temp2_dn8)), ((locals.var_gcqovacc * locals.var_temp1_dn9) + (locals.var_gcqovinv * locals.var_temp2_dn9)),)
    } else {
        (locals.var_gcqoveff, locals.var_gcqoveff_dn4, locals.var_gcqoveff_dn6, locals.var_gcqoveff_dn7, locals.var_gcqoveff_dn8, locals.var_gcqoveff_dn9,)
    }
};
        locals.var_gcqoveff = assign26210_e27423;
        locals.var_gcqoveff_dn4 = assign26210_e27423_d_n4;
        locals.var_gcqoveff_dn6 = assign26210_e27423_d_n6;
        locals.var_gcqoveff_dn7 = assign26210_e27423_d_n7;
        locals.var_gcqoveff_dn8 = assign26210_e27423_d_n8;
        locals.var_gcqoveff_dn9 = assign26210_e27423_d_n9;

        let (assign26220_e27435, assign26220_e27435_d_n4, assign26220_e27435_d_n6, assign26220_e27435_d_n7, assign26220_e27435_d_n8, assign26220_e27435_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26220_e27429: f64 = (locals.var_igovaccd_i * locals.var_temp1);
        let assign26220_e27432: f64 = (locals.var_igovinvd_i * locals.var_temp2);
        let assign26220_e27433: f64 = (assign26220_e27429 + assign26220_e27432);
        (assign26220_e27433, (((locals.var_igovaccd_i_dn4 * locals.var_temp1) + (locals.var_igovaccd_i * locals.var_temp1_dn4)) + ((locals.var_igovinvd_i_dn4 * locals.var_temp2) + (locals.var_igovinvd_i * locals.var_temp2_dn4))), (((locals.var_igovaccd_i_dn6 * locals.var_temp1) + (locals.var_igovaccd_i * locals.var_temp1_dn6)) + ((locals.var_igovinvd_i_dn6 * locals.var_temp2) + (locals.var_igovinvd_i * locals.var_temp2_dn6))), (((locals.var_igovaccd_i_dn7 * locals.var_temp1) + (locals.var_igovaccd_i * locals.var_temp1_dn7)) + ((locals.var_igovinvd_i_dn7 * locals.var_temp2) + (locals.var_igovinvd_i * locals.var_temp2_dn7))), (((locals.var_igovaccd_i_dn8 * locals.var_temp1) + (locals.var_igovaccd_i * locals.var_temp1_dn8)) + ((locals.var_igovinvd_i_dn8 * locals.var_temp2) + (locals.var_igovinvd_i * locals.var_temp2_dn8))), (((locals.var_igovaccd_i_dn9 * locals.var_temp1) + (locals.var_igovaccd_i * locals.var_temp1_dn9)) + ((locals.var_igovinvd_i_dn9 * locals.var_temp2) + (locals.var_igovinvd_i * locals.var_temp2_dn9))),)
    } else {
        (locals.var_igoveff, locals.var_igoveff_dn4, locals.var_igoveff_dn6, locals.var_igoveff_dn7, locals.var_igoveff_dn8, locals.var_igoveff_dn9,)
    }
};
        locals.var_igoveff = assign26220_e27435;
        locals.var_igoveff_dn4 = assign26220_e27435_d_n4;
        locals.var_igoveff_dn6 = assign26220_e27435_d_n6;
        locals.var_igoveff_dn7 = assign26220_e27435_d_n7;
        locals.var_igoveff_dn8 = assign26220_e27435_d_n8;
        locals.var_igoveff_dn9 = assign26220_e27435_d_n9;

        let (assign26230_e27445, assign26230_e27445_d_n4, assign26230_e27445_d_n6, assign26230_e27445_d_n7, assign26230_e27445_d_n8, assign26230_e27445_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26230_e27441: f64 = (locals.var_fnovinvd_i * locals.var_temp2);
        let assign26230_e27443: f64 = (assign26230_e27441 * 1e-6);
        (assign26230_e27443, (((locals.var_fnovinvd_i_dn4 * locals.var_temp2) + (locals.var_fnovinvd_i * locals.var_temp2_dn4)) * 1e-6), (((locals.var_fnovinvd_i_dn6 * locals.var_temp2) + (locals.var_fnovinvd_i * locals.var_temp2_dn6)) * 1e-6), (((locals.var_fnovinvd_i_dn7 * locals.var_temp2) + (locals.var_fnovinvd_i * locals.var_temp2_dn7)) * 1e-6), (((locals.var_fnovinvd_i_dn8 * locals.var_temp2) + (locals.var_fnovinvd_i * locals.var_temp2_dn8)) * 1e-6), (((locals.var_fnovinvd_i_dn9 * locals.var_temp2) + (locals.var_fnovinvd_i * locals.var_temp2_dn9)) * 1e-6),)
    } else {
        (locals.var_igovefffowler, locals.var_igovefffowler_dn4, locals.var_igovefffowler_dn6, locals.var_igovefffowler_dn7, locals.var_igovefffowler_dn8, locals.var_igovefffowler_dn9,)
    }
};
        locals.var_igovefffowler = assign26230_e27445;
        locals.var_igovefffowler_dn4 = assign26230_e27445_d_n4;
        locals.var_igovefffowler_dn6 = assign26230_e27445_d_n6;
        locals.var_igovefffowler_dn7 = assign26230_e27445_d_n7;
        locals.var_igovefffowler_dn8 = assign26230_e27445_d_n8;
        locals.var_igovefffowler_dn9 = assign26230_e27445_d_n9;

        let (assign26240_e27458, assign26240_e27458_d_n4, assign26240_e27458_d_n6, assign26240_e27458_d_n7, assign26240_e27458_d_n8, assign26240_e27458_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26240_e27451: f64 = (-1.0);
        let assign26240_e27453: f64 = (assign26240_e27451 * locals.var_gcovinvfn_i);
        let assign26240_e27455: f64 = (assign26240_e27453 / locals.var_zg);
        let assign26240_e27456: f64 = (locals.var_bov * assign26240_e27455);
        (assign26240_e27456, ((locals.var_bov_dn4 * assign26240_e27455) + (locals.var_bov * (-((assign26240_e27453 * locals.var_zg_dn4) / (locals.var_zg * locals.var_zg))))), ((locals.var_bov_dn6 * assign26240_e27455) + (locals.var_bov * (-((assign26240_e27453 * locals.var_zg_dn6) / (locals.var_zg * locals.var_zg))))), ((locals.var_bov_dn7 * assign26240_e27455) + (locals.var_bov * (-((assign26240_e27453 * locals.var_zg_dn7) / (locals.var_zg * locals.var_zg))))), ((locals.var_bov_dn8 * assign26240_e27455) + (locals.var_bov * (-((assign26240_e27453 * locals.var_zg_dn8) / (locals.var_zg * locals.var_zg))))), ((locals.var_bov_dn9 * assign26240_e27455) + (locals.var_bov * (-((assign26240_e27453 * locals.var_zg_dn9) / (locals.var_zg * locals.var_zg))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign26240_e27458;
        locals.var_temp1_dn4 = assign26240_e27458_d_n4;
        locals.var_temp1_dn6 = assign26240_e27458_d_n6;
        locals.var_temp1_dn7 = assign26240_e27458_d_n7;
        locals.var_temp1_dn8 = assign26240_e27458_d_n8;
        locals.var_temp1_dn9 = assign26240_e27458_d_n9;

        let assign26250_e27461: f64 = if locals.var_gc3oveff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard742 = assign26250_e27461;

        let (assign26260_e27484, assign26260_e27484_d_n4, assign26260_e27484_d_n6, assign26260_e27484_d_n7, assign26260_e27484_d_n8, assign26260_e27484_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard742 != 0.0)) {
        let assign26260_e27470: f64 = (locals.var_zg + locals.var_gcqoveff);
        let assign26260_e27473: f64 = (locals.var_zg - locals.var_gcqoveff);
        let assign26260_e27476: f64 = (locals.var_zg - locals.var_gcqoveff);
        let assign26260_e27477: f64 = (assign26260_e27473 * assign26260_e27476);
        let assign26260_e27479: f64 = (assign26260_e27477 + 1e-6);
        let assign26260_e27480: f64 = (assign26260_e27479).sqrt();
        let assign26260_e27481: f64 = (assign26260_e27470 - assign26260_e27480);
        let assign26260_e27482: f64 = (0.5 * assign26260_e27481);
        (assign26260_e27482, (0.5 * ((locals.var_zg_dn4 + locals.var_gcqoveff_dn4) - ((((locals.var_zg_dn4 - locals.var_gcqoveff_dn4) * assign26260_e27476) + (assign26260_e27473 * (locals.var_zg_dn4 - locals.var_gcqoveff_dn4))) / (2.0 * assign26260_e27480)))), (0.5 * ((locals.var_zg_dn6 + locals.var_gcqoveff_dn6) - ((((locals.var_zg_dn6 - locals.var_gcqoveff_dn6) * assign26260_e27476) + (assign26260_e27473 * (locals.var_zg_dn6 - locals.var_gcqoveff_dn6))) / (2.0 * assign26260_e27480)))), (0.5 * ((locals.var_zg_dn7 + locals.var_gcqoveff_dn7) - ((((locals.var_zg_dn7 - locals.var_gcqoveff_dn7) * assign26260_e27476) + (assign26260_e27473 * (locals.var_zg_dn7 - locals.var_gcqoveff_dn7))) / (2.0 * assign26260_e27480)))), (0.5 * ((locals.var_zg_dn8 + locals.var_gcqoveff_dn8) - ((((locals.var_zg_dn8 - locals.var_gcqoveff_dn8) * assign26260_e27476) + (assign26260_e27473 * (locals.var_zg_dn8 - locals.var_gcqoveff_dn8))) / (2.0 * assign26260_e27480)))), (0.5 * ((locals.var_zg_dn9 + locals.var_gcqoveff_dn9) - ((((locals.var_zg_dn9 - locals.var_gcqoveff_dn9) * assign26260_e27476) + (assign26260_e27473 * (locals.var_zg_dn9 - locals.var_gcqoveff_dn9))) / (2.0 * assign26260_e27480)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign26260_e27484;
        locals.var_zg_dn4 = assign26260_e27484_d_n4;
        locals.var_zg_dn6 = assign26260_e27484_d_n6;
        locals.var_zg_dn7 = assign26260_e27484_d_n7;
        locals.var_zg_dn8 = assign26260_e27484_d_n8;
        locals.var_zg_dn9 = assign26260_e27484_d_n9;

        let (assign26270_e27496, assign26270_e27496_d_n4, assign26270_e27496_d_n6, assign26270_e27496_d_n7, assign26270_e27496_d_n8, assign26270_e27496_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26270_e27490: f64 = (3.0 + locals.var_xd_ov);
        let assign26270_e27493: f64 = (locals.var_psi_t * locals.var_inv_phit0);
        let assign26270_e27494: f64 = (assign26270_e27490 + assign26270_e27493);
        (assign26270_e27494, (locals.var_xd_ov_dn4 + ((locals.var_psi_t_dn4 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn4))), (locals.var_xd_ov_dn6 + ((locals.var_psi_t_dn6 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn6))), (locals.var_xd_ov_dn7 + ((locals.var_psi_t_dn7 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn7))), (locals.var_xd_ov_dn8 + ((locals.var_psi_t_dn8 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn8))), (locals.var_xd_ov_dn9 + ((locals.var_psi_t_dn9 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn9))),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9,)
    }
};
        locals.var_arg1 = assign26270_e27496;
        locals.var_arg1_dn4 = assign26270_e27496_d_n4;
        locals.var_arg1_dn6 = assign26270_e27496_d_n6;
        locals.var_arg1_dn7 = assign26270_e27496_d_n7;
        locals.var_arg1_dn8 = assign26270_e27496_d_n8;
        locals.var_arg1_dn9 = assign26270_e27496_d_n9;

        let assign26280_e27498: f64 = (locals.var_arg1).abs();
        let assign26280_e27500: f64 = if assign26280_e27498 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard743 = assign26280_e27500;

        let (assign26290_e27509, assign26290_e27509_d_n4, assign26290_e27509_d_n6, assign26290_e27509_d_n7, assign26290_e27509_d_n8, assign26290_e27509_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard743 != 0.0)) {
        let assign26290_e27507: f64 = (locals.var_arg1).exp();
        (assign26290_e27507, (assign26290_e27507 * locals.var_arg1_dn4), (assign26290_e27507 * locals.var_arg1_dn6), (assign26290_e27507 * locals.var_arg1_dn7), (assign26290_e27507 * locals.var_arg1_dn8), (assign26290_e27507 * locals.var_arg1_dn9),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign26290_e27509;
        locals.var_dsi_dn4 = assign26290_e27509_d_n4;
        locals.var_dsi_dn6 = assign26290_e27509_d_n6;
        locals.var_dsi_dn7 = assign26290_e27509_d_n7;
        locals.var_dsi_dn8 = assign26290_e27509_d_n8;
        locals.var_dsi_dn9 = assign26290_e27509_d_n9;

        let assign26300_e27512: f64 = (-80.0);
        let assign26300_e27513: f64 = if locals.var_arg1 < assign26300_e27512 { 1.0 } else { 0.0 };
        locals.var_guard744 = assign26300_e27513;

        let (assign26310_e27549, assign26310_e27549_d_n4, assign26310_e27549_d_n6, assign26310_e27549_d_n7, assign26310_e27549_d_n8, assign26310_e27549_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard743 == 0.0)) && (locals.var_guard744 != 0.0)) {
        let assign26310_e27525: f64 = (-locals.var_arg1);
        let assign26310_e27527: f64 = (assign26310_e27525 - 80.0);
        let assign26310_e27531: f64 = (-locals.var_arg1);
        let assign26310_e27533: f64 = (assign26310_e27531 - 80.0);
        let assign26310_e27534: f64 = (0.5 * assign26310_e27533);
        let assign26310_e27537: f64 = (-locals.var_arg1);
        let assign26310_e27539: f64 = (assign26310_e27537 - 80.0);
        let assign26310_e27541: f64 = (assign26310_e27539 * 0.3333333333333);
        let assign26310_e27542: f64 = (1.0 + assign26310_e27541);
        let assign26310_e27543: f64 = (assign26310_e27534 * assign26310_e27542);
        let assign26310_e27544: f64 = (1.0 + assign26310_e27543);
        let assign26310_e27545: f64 = (assign26310_e27527 * assign26310_e27544);
        let assign26310_e27546: f64 = (1.0 + assign26310_e27545);
        let assign26310_e27547: f64 = (1.80485e-35 / assign26310_e27546);
        (assign26310_e27547, (-((1.80485e-35 * (((-locals.var_arg1_dn4) * assign26310_e27544) + (assign26310_e27527 * (((0.5 * (-locals.var_arg1_dn4)) * assign26310_e27542) + (assign26310_e27534 * ((-locals.var_arg1_dn4) * 0.3333333333333)))))) / (assign26310_e27546 * assign26310_e27546))), (-((1.80485e-35 * (((-locals.var_arg1_dn6) * assign26310_e27544) + (assign26310_e27527 * (((0.5 * (-locals.var_arg1_dn6)) * assign26310_e27542) + (assign26310_e27534 * ((-locals.var_arg1_dn6) * 0.3333333333333)))))) / (assign26310_e27546 * assign26310_e27546))), (-((1.80485e-35 * (((-locals.var_arg1_dn7) * assign26310_e27544) + (assign26310_e27527 * (((0.5 * (-locals.var_arg1_dn7)) * assign26310_e27542) + (assign26310_e27534 * ((-locals.var_arg1_dn7) * 0.3333333333333)))))) / (assign26310_e27546 * assign26310_e27546))), (-((1.80485e-35 * (((-locals.var_arg1_dn8) * assign26310_e27544) + (assign26310_e27527 * (((0.5 * (-locals.var_arg1_dn8)) * assign26310_e27542) + (assign26310_e27534 * ((-locals.var_arg1_dn8) * 0.3333333333333)))))) / (assign26310_e27546 * assign26310_e27546))), (-((1.80485e-35 * (((-locals.var_arg1_dn9) * assign26310_e27544) + (assign26310_e27527 * (((0.5 * (-locals.var_arg1_dn9)) * assign26310_e27542) + (assign26310_e27534 * ((-locals.var_arg1_dn9) * 0.3333333333333)))))) / (assign26310_e27546 * assign26310_e27546))),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign26310_e27549;
        locals.var_dsi_dn4 = assign26310_e27549_d_n4;
        locals.var_dsi_dn6 = assign26310_e27549_d_n6;
        locals.var_dsi_dn7 = assign26310_e27549_d_n7;
        locals.var_dsi_dn8 = assign26310_e27549_d_n8;
        locals.var_dsi_dn9 = assign26310_e27549_d_n9;

    }

    pub(super) fn stamp_transient_block_68(
        locals: &mut StampLocals,
    ) {
        let (assign26320_e27583, assign26320_e27583_d_n4, assign26320_e27583_d_n6, assign26320_e27583_d_n7, assign26320_e27583_d_n8, assign26320_e27583_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard743 == 0.0)) && (locals.var_guard744 == 0.0)) {
        let assign26320_e27563: f64 = (locals.var_arg1 - 80.0);
        let assign26320_e27568: f64 = (locals.var_arg1 - 80.0);
        let assign26320_e27569: f64 = (0.5 * assign26320_e27568);
        let assign26320_e27573: f64 = (locals.var_arg1 - 80.0);
        let assign26320_e27575: f64 = (assign26320_e27573 * 0.3333333333333);
        let assign26320_e27576: f64 = (1.0 + assign26320_e27575);
        let assign26320_e27577: f64 = (assign26320_e27569 * assign26320_e27576);
        let assign26320_e27578: f64 = (1.0 + assign26320_e27577);
        let assign26320_e27579: f64 = (assign26320_e27563 * assign26320_e27578);
        let assign26320_e27580: f64 = (1.0 + assign26320_e27579);
        let assign26320_e27581: f64 = (5.54062e34 * assign26320_e27580);
        (assign26320_e27581, (5.54062e34 * ((locals.var_arg1_dn4 * assign26320_e27578) + (assign26320_e27563 * (((0.5 * locals.var_arg1_dn4) * assign26320_e27576) + (assign26320_e27569 * (locals.var_arg1_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn6 * assign26320_e27578) + (assign26320_e27563 * (((0.5 * locals.var_arg1_dn6) * assign26320_e27576) + (assign26320_e27569 * (locals.var_arg1_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn7 * assign26320_e27578) + (assign26320_e27563 * (((0.5 * locals.var_arg1_dn7) * assign26320_e27576) + (assign26320_e27569 * (locals.var_arg1_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn8 * assign26320_e27578) + (assign26320_e27563 * (((0.5 * locals.var_arg1_dn8) * assign26320_e27576) + (assign26320_e27569 * (locals.var_arg1_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn9 * assign26320_e27578) + (assign26320_e27563 * (((0.5 * locals.var_arg1_dn9) * assign26320_e27576) + (assign26320_e27569 * (locals.var_arg1_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign26320_e27583;
        locals.var_dsi_dn4 = assign26320_e27583_d_n4;
        locals.var_dsi_dn6 = assign26320_e27583_d_n6;
        locals.var_dsi_dn7 = assign26320_e27583_d_n7;
        locals.var_dsi_dn8 = assign26320_e27583_d_n8;
        locals.var_dsi_dn9 = assign26320_e27583_d_n9;

        let (assign26330_e27597, assign26330_e27597_d_n4, assign26330_e27597_d_n6, assign26330_e27597_d_n7, assign26330_e27597_d_n8, assign26330_e27597_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26330_e27589: f64 = (3.0 + locals.var_xd_ov);
        let assign26330_e27592: f64 = (locals.var_psi_t * locals.var_inv_phit0);
        let assign26330_e27593: f64 = (assign26330_e27589 + assign26330_e27592);
        let assign26330_e27595: f64 = (assign26330_e27593 + locals.var_xgd_ov);
        (assign26330_e27595, ((locals.var_xd_ov_dn4 + ((locals.var_psi_t_dn4 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn4))) + locals.var_xgd_ov_dn4), ((locals.var_xd_ov_dn6 + ((locals.var_psi_t_dn6 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn6))) + locals.var_xgd_ov_dn6), ((locals.var_xd_ov_dn7 + ((locals.var_psi_t_dn7 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn7))) + locals.var_xgd_ov_dn7), ((locals.var_xd_ov_dn8 + ((locals.var_psi_t_dn8 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn8))) + locals.var_xgd_ov_dn8), ((locals.var_xd_ov_dn9 + ((locals.var_psi_t_dn9 * locals.var_inv_phit0) + (locals.var_psi_t * locals.var_inv_phit0_dn9))) + locals.var_xgd_ov_dn9),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9,)
    }
};
        locals.var_arg1 = assign26330_e27597;
        locals.var_arg1_dn4 = assign26330_e27597_d_n4;
        locals.var_arg1_dn6 = assign26330_e27597_d_n6;
        locals.var_arg1_dn7 = assign26330_e27597_d_n7;
        locals.var_arg1_dn8 = assign26330_e27597_d_n8;
        locals.var_arg1_dn9 = assign26330_e27597_d_n9;

        let assign26340_e27599: f64 = (locals.var_arg1).abs();
        let assign26340_e27601: f64 = if assign26340_e27599 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard745 = assign26340_e27601;

        let (assign26350_e27610, assign26350_e27610_d_n4, assign26350_e27610_d_n6, assign26350_e27610_d_n7, assign26350_e27610_d_n8, assign26350_e27610_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard745 != 0.0)) {
        let assign26350_e27608: f64 = (locals.var_arg1).exp();
        (assign26350_e27608, (assign26350_e27608 * locals.var_arg1_dn4), (assign26350_e27608 * locals.var_arg1_dn6), (assign26350_e27608 * locals.var_arg1_dn7), (assign26350_e27608 * locals.var_arg1_dn8), (assign26350_e27608 * locals.var_arg1_dn9),)
    } else {
        (locals.var_dgate, locals.var_dgate_dn4, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8, locals.var_dgate_dn9,)
    }
};
        locals.var_dgate = assign26350_e27610;
        locals.var_dgate_dn4 = assign26350_e27610_d_n4;
        locals.var_dgate_dn6 = assign26350_e27610_d_n6;
        locals.var_dgate_dn7 = assign26350_e27610_d_n7;
        locals.var_dgate_dn8 = assign26350_e27610_d_n8;
        locals.var_dgate_dn9 = assign26350_e27610_d_n9;

        let assign26360_e27613: f64 = (-80.0);
        let assign26360_e27614: f64 = if locals.var_arg1 < assign26360_e27613 { 1.0 } else { 0.0 };
        locals.var_guard746 = assign26360_e27614;

        let (assign26370_e27650, assign26370_e27650_d_n4, assign26370_e27650_d_n6, assign26370_e27650_d_n7, assign26370_e27650_d_n8, assign26370_e27650_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard746 != 0.0)) {
        let assign26370_e27626: f64 = (-locals.var_arg1);
        let assign26370_e27628: f64 = (assign26370_e27626 - 80.0);
        let assign26370_e27632: f64 = (-locals.var_arg1);
        let assign26370_e27634: f64 = (assign26370_e27632 - 80.0);
        let assign26370_e27635: f64 = (0.5 * assign26370_e27634);
        let assign26370_e27638: f64 = (-locals.var_arg1);
        let assign26370_e27640: f64 = (assign26370_e27638 - 80.0);
        let assign26370_e27642: f64 = (assign26370_e27640 * 0.3333333333333);
        let assign26370_e27643: f64 = (1.0 + assign26370_e27642);
        let assign26370_e27644: f64 = (assign26370_e27635 * assign26370_e27643);
        let assign26370_e27645: f64 = (1.0 + assign26370_e27644);
        let assign26370_e27646: f64 = (assign26370_e27628 * assign26370_e27645);
        let assign26370_e27647: f64 = (1.0 + assign26370_e27646);
        let assign26370_e27648: f64 = (1.80485e-35 / assign26370_e27647);
        (assign26370_e27648, (-((1.80485e-35 * (((-locals.var_arg1_dn4) * assign26370_e27645) + (assign26370_e27628 * (((0.5 * (-locals.var_arg1_dn4)) * assign26370_e27643) + (assign26370_e27635 * ((-locals.var_arg1_dn4) * 0.3333333333333)))))) / (assign26370_e27647 * assign26370_e27647))), (-((1.80485e-35 * (((-locals.var_arg1_dn6) * assign26370_e27645) + (assign26370_e27628 * (((0.5 * (-locals.var_arg1_dn6)) * assign26370_e27643) + (assign26370_e27635 * ((-locals.var_arg1_dn6) * 0.3333333333333)))))) / (assign26370_e27647 * assign26370_e27647))), (-((1.80485e-35 * (((-locals.var_arg1_dn7) * assign26370_e27645) + (assign26370_e27628 * (((0.5 * (-locals.var_arg1_dn7)) * assign26370_e27643) + (assign26370_e27635 * ((-locals.var_arg1_dn7) * 0.3333333333333)))))) / (assign26370_e27647 * assign26370_e27647))), (-((1.80485e-35 * (((-locals.var_arg1_dn8) * assign26370_e27645) + (assign26370_e27628 * (((0.5 * (-locals.var_arg1_dn8)) * assign26370_e27643) + (assign26370_e27635 * ((-locals.var_arg1_dn8) * 0.3333333333333)))))) / (assign26370_e27647 * assign26370_e27647))), (-((1.80485e-35 * (((-locals.var_arg1_dn9) * assign26370_e27645) + (assign26370_e27628 * (((0.5 * (-locals.var_arg1_dn9)) * assign26370_e27643) + (assign26370_e27635 * ((-locals.var_arg1_dn9) * 0.3333333333333)))))) / (assign26370_e27647 * assign26370_e27647))),)
    } else {
        (locals.var_dgate, locals.var_dgate_dn4, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8, locals.var_dgate_dn9,)
    }
};
        locals.var_dgate = assign26370_e27650;
        locals.var_dgate_dn4 = assign26370_e27650_d_n4;
        locals.var_dgate_dn6 = assign26370_e27650_d_n6;
        locals.var_dgate_dn7 = assign26370_e27650_d_n7;
        locals.var_dgate_dn8 = assign26370_e27650_d_n8;
        locals.var_dgate_dn9 = assign26370_e27650_d_n9;

        let (assign26380_e27684, assign26380_e27684_d_n4, assign26380_e27684_d_n6, assign26380_e27684_d_n7, assign26380_e27684_d_n8, assign26380_e27684_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard745 == 0.0)) && (locals.var_guard746 == 0.0)) {
        let assign26380_e27664: f64 = (locals.var_arg1 - 80.0);
        let assign26380_e27669: f64 = (locals.var_arg1 - 80.0);
        let assign26380_e27670: f64 = (0.5 * assign26380_e27669);
        let assign26380_e27674: f64 = (locals.var_arg1 - 80.0);
        let assign26380_e27676: f64 = (assign26380_e27674 * 0.3333333333333);
        let assign26380_e27677: f64 = (1.0 + assign26380_e27676);
        let assign26380_e27678: f64 = (assign26380_e27670 * assign26380_e27677);
        let assign26380_e27679: f64 = (1.0 + assign26380_e27678);
        let assign26380_e27680: f64 = (assign26380_e27664 * assign26380_e27679);
        let assign26380_e27681: f64 = (1.0 + assign26380_e27680);
        let assign26380_e27682: f64 = (5.54062e34 * assign26380_e27681);
        (assign26380_e27682, (5.54062e34 * ((locals.var_arg1_dn4 * assign26380_e27679) + (assign26380_e27664 * (((0.5 * locals.var_arg1_dn4) * assign26380_e27677) + (assign26380_e27670 * (locals.var_arg1_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn6 * assign26380_e27679) + (assign26380_e27664 * (((0.5 * locals.var_arg1_dn6) * assign26380_e27677) + (assign26380_e27670 * (locals.var_arg1_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn7 * assign26380_e27679) + (assign26380_e27664 * (((0.5 * locals.var_arg1_dn7) * assign26380_e27677) + (assign26380_e27670 * (locals.var_arg1_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn8 * assign26380_e27679) + (assign26380_e27664 * (((0.5 * locals.var_arg1_dn8) * assign26380_e27677) + (assign26380_e27670 * (locals.var_arg1_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn9 * assign26380_e27679) + (assign26380_e27664 * (((0.5 * locals.var_arg1_dn9) * assign26380_e27677) + (assign26380_e27670 * (locals.var_arg1_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_dgate, locals.var_dgate_dn4, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8, locals.var_dgate_dn9,)
    }
};
        locals.var_dgate = assign26380_e27684;
        locals.var_dgate_dn4 = assign26380_e27684_d_n4;
        locals.var_dgate_dn6 = assign26380_e27684_d_n6;
        locals.var_dgate_dn7 = assign26380_e27684_d_n7;
        locals.var_dgate_dn8 = assign26380_e27684_d_n8;
        locals.var_dgate_dn9 = assign26380_e27684_d_n9;

        let (assign26390_e27701, assign26390_e27701_d_n4, assign26390_e27701_d_n6, assign26390_e27701_d_n7, assign26390_e27701_d_n8, assign26390_e27701_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26390_e27690: f64 = (-1.5);
        let assign26390_e27695: f64 = (locals.var_gc3oveff * locals.var_zg);
        let assign26390_e27696: f64 = (locals.var_gc2oveff + assign26390_e27695);
        let assign26390_e27697: f64 = (locals.var_zg * assign26390_e27696);
        let assign26390_e27698: f64 = (assign26390_e27690 + assign26390_e27697);
        let assign26390_e27699: f64 = (locals.var_bov * assign26390_e27698);
        (assign26390_e27699, ((locals.var_bov_dn4 * assign26390_e27698) + (locals.var_bov * ((locals.var_zg_dn4 * assign26390_e27696) + (locals.var_zg * (locals.var_gc2oveff_dn4 + ((locals.var_gc3oveff_dn4 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn4))))))), ((locals.var_bov_dn6 * assign26390_e27698) + (locals.var_bov * ((locals.var_zg_dn6 * assign26390_e27696) + (locals.var_zg * (locals.var_gc2oveff_dn6 + ((locals.var_gc3oveff_dn6 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn6))))))), ((locals.var_bov_dn7 * assign26390_e27698) + (locals.var_bov * ((locals.var_zg_dn7 * assign26390_e27696) + (locals.var_zg * (locals.var_gc2oveff_dn7 + ((locals.var_gc3oveff_dn7 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn7))))))), ((locals.var_bov_dn8 * assign26390_e27698) + (locals.var_bov * ((locals.var_zg_dn8 * assign26390_e27696) + (locals.var_zg * (locals.var_gc2oveff_dn8 + ((locals.var_gc3oveff_dn8 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn8))))))), ((locals.var_bov_dn9 * assign26390_e27698) + (locals.var_bov * ((locals.var_zg_dn9 * assign26390_e27696) + (locals.var_zg * (locals.var_gc2oveff_dn9 + ((locals.var_gc3oveff_dn9 * locals.var_zg) + (locals.var_gc3oveff * locals.var_zg_dn9))))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26390_e27701;
        locals.var_temp_dn4 = assign26390_e27701_d_n4;
        locals.var_temp_dn6 = assign26390_e27701_d_n6;
        locals.var_temp_dn7 = assign26390_e27701_d_n7;
        locals.var_temp_dn8 = assign26390_e27701_d_n8;
        locals.var_temp_dn9 = assign26390_e27701_d_n9;

        let assign26400_e27704: f64 = if locals.var_temp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard747 = assign26400_e27704;

        let (assign26410_e27726, assign26410_e27726_d_n4, assign26410_e27726_d_n6, assign26410_e27726_d_n7, assign26410_e27726_d_n8, assign26410_e27726_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard747 != 0.0)) {
        let assign26410_e27715: f64 = (0.5 * locals.var_temp);
        let assign26410_e27719: f64 = (locals.var_temp * 0.3333333333333);
        let assign26410_e27720: f64 = (1.0 + assign26410_e27719);
        let assign26410_e27721: f64 = (assign26410_e27715 * assign26410_e27720);
        let assign26410_e27722: f64 = (1.0 + assign26410_e27721);
        let assign26410_e27723: f64 = (locals.var_temp * assign26410_e27722);
        let assign26410_e27724: f64 = (1.0 + assign26410_e27723);
        (assign26410_e27724, ((locals.var_temp_dn4 * assign26410_e27722) + (locals.var_temp * (((0.5 * locals.var_temp_dn4) * assign26410_e27720) + (assign26410_e27715 * (locals.var_temp_dn4 * 0.3333333333333))))), ((locals.var_temp_dn6 * assign26410_e27722) + (locals.var_temp * (((0.5 * locals.var_temp_dn6) * assign26410_e27720) + (assign26410_e27715 * (locals.var_temp_dn6 * 0.3333333333333))))), ((locals.var_temp_dn7 * assign26410_e27722) + (locals.var_temp * (((0.5 * locals.var_temp_dn7) * assign26410_e27720) + (assign26410_e27715 * (locals.var_temp_dn7 * 0.3333333333333))))), ((locals.var_temp_dn8 * assign26410_e27722) + (locals.var_temp * (((0.5 * locals.var_temp_dn8) * assign26410_e27720) + (assign26410_e27715 * (locals.var_temp_dn8 * 0.3333333333333))))), ((locals.var_temp_dn9 * assign26410_e27722) + (locals.var_temp * (((0.5 * locals.var_temp_dn9) * assign26410_e27720) + (assign26410_e27715 * (locals.var_temp_dn9 * 0.3333333333333))))),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign26410_e27726;
        locals.var_tp_dn4 = assign26410_e27726_d_n4;
        locals.var_tp_dn6 = assign26410_e27726_d_n6;
        locals.var_tp_dn7 = assign26410_e27726_d_n7;
        locals.var_tp_dn8 = assign26410_e27726_d_n8;
        locals.var_tp_dn9 = assign26410_e27726_d_n9;

        let assign26420_e27729: f64 = (-80.0);
        let assign26420_e27730: f64 = if locals.var_temp > assign26420_e27729 { 1.0 } else { 0.0 };
        locals.var_guard748 = assign26420_e27730;

        let (assign26430_e27742, assign26430_e27742_d_n4, assign26430_e27742_d_n6, assign26430_e27742_d_n7, assign26430_e27742_d_n8, assign26430_e27742_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard747 == 0.0)) && (locals.var_guard748 != 0.0)) {
        let assign26430_e27740: f64 = (locals.var_temp).exp();
        (assign26430_e27740, (assign26430_e27740 * locals.var_temp_dn4), (assign26430_e27740 * locals.var_temp_dn6), (assign26430_e27740 * locals.var_temp_dn7), (assign26430_e27740 * locals.var_temp_dn8), (assign26430_e27740 * locals.var_temp_dn9),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign26430_e27742;
        locals.var_tp_dn4 = assign26430_e27742_d_n4;
        locals.var_tp_dn6 = assign26430_e27742_d_n6;
        locals.var_tp_dn7 = assign26430_e27742_d_n7;
        locals.var_tp_dn8 = assign26430_e27742_d_n8;
        locals.var_tp_dn9 = assign26430_e27742_d_n9;

        let (assign26440_e27779, assign26440_e27779_d_n4, assign26440_e27779_d_n6, assign26440_e27779_d_n7, assign26440_e27779_d_n8, assign26440_e27779_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard747 == 0.0)) && (locals.var_guard748 == 0.0)) {
        let assign26440_e27755: f64 = (-locals.var_temp);
        let assign26440_e27757: f64 = (assign26440_e27755 - 80.0);
        let assign26440_e27761: f64 = (-locals.var_temp);
        let assign26440_e27763: f64 = (assign26440_e27761 - 80.0);
        let assign26440_e27764: f64 = (0.5 * assign26440_e27763);
        let assign26440_e27767: f64 = (-locals.var_temp);
        let assign26440_e27769: f64 = (assign26440_e27767 - 80.0);
        let assign26440_e27771: f64 = (assign26440_e27769 * 0.3333333333333);
        let assign26440_e27772: f64 = (1.0 + assign26440_e27771);
        let assign26440_e27773: f64 = (assign26440_e27764 * assign26440_e27772);
        let assign26440_e27774: f64 = (1.0 + assign26440_e27773);
        let assign26440_e27775: f64 = (assign26440_e27757 * assign26440_e27774);
        let assign26440_e27776: f64 = (1.0 + assign26440_e27775);
        let assign26440_e27777: f64 = (1.80485e-35 / assign26440_e27776);
        (assign26440_e27777, (-((1.80485e-35 * (((-locals.var_temp_dn4) * assign26440_e27774) + (assign26440_e27757 * (((0.5 * (-locals.var_temp_dn4)) * assign26440_e27772) + (assign26440_e27764 * ((-locals.var_temp_dn4) * 0.3333333333333)))))) / (assign26440_e27776 * assign26440_e27776))), (-((1.80485e-35 * (((-locals.var_temp_dn6) * assign26440_e27774) + (assign26440_e27757 * (((0.5 * (-locals.var_temp_dn6)) * assign26440_e27772) + (assign26440_e27764 * ((-locals.var_temp_dn6) * 0.3333333333333)))))) / (assign26440_e27776 * assign26440_e27776))), (-((1.80485e-35 * (((-locals.var_temp_dn7) * assign26440_e27774) + (assign26440_e27757 * (((0.5 * (-locals.var_temp_dn7)) * assign26440_e27772) + (assign26440_e27764 * ((-locals.var_temp_dn7) * 0.3333333333333)))))) / (assign26440_e27776 * assign26440_e27776))), (-((1.80485e-35 * (((-locals.var_temp_dn8) * assign26440_e27774) + (assign26440_e27757 * (((0.5 * (-locals.var_temp_dn8)) * assign26440_e27772) + (assign26440_e27764 * ((-locals.var_temp_dn8) * 0.3333333333333)))))) / (assign26440_e27776 * assign26440_e27776))), (-((1.80485e-35 * (((-locals.var_temp_dn9) * assign26440_e27774) + (assign26440_e27757 * (((0.5 * (-locals.var_temp_dn9)) * assign26440_e27772) + (assign26440_e27764 * ((-locals.var_temp_dn9) * 0.3333333333333)))))) / (assign26440_e27776 * assign26440_e27776))),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign26440_e27779;
        locals.var_tp_dn4 = assign26440_e27779_d_n4;
        locals.var_tp_dn6 = assign26440_e27779_d_n6;
        locals.var_tp_dn7 = assign26440_e27779_d_n7;
        locals.var_tp_dn8 = assign26440_e27779_d_n8;
        locals.var_tp_dn9 = assign26440_e27779_d_n9;

        let assign26450_e27782: f64 = if locals.var_temp1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard749 = assign26450_e27782;

        let (assign26460_e27804, assign26460_e27804_d_n4, assign26460_e27804_d_n6, assign26460_e27804_d_n7, assign26460_e27804_d_n8, assign26460_e27804_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard749 != 0.0)) {
        let assign26460_e27793: f64 = (0.5 * locals.var_temp1);
        let assign26460_e27797: f64 = (locals.var_temp1 * 0.3333333333333);
        let assign26460_e27798: f64 = (1.0 + assign26460_e27797);
        let assign26460_e27799: f64 = (assign26460_e27793 * assign26460_e27798);
        let assign26460_e27800: f64 = (1.0 + assign26460_e27799);
        let assign26460_e27801: f64 = (locals.var_temp1 * assign26460_e27800);
        let assign26460_e27802: f64 = (1.0 + assign26460_e27801);
        (assign26460_e27802, ((locals.var_temp1_dn4 * assign26460_e27800) + (locals.var_temp1 * (((0.5 * locals.var_temp1_dn4) * assign26460_e27798) + (assign26460_e27793 * (locals.var_temp1_dn4 * 0.3333333333333))))), ((locals.var_temp1_dn6 * assign26460_e27800) + (locals.var_temp1 * (((0.5 * locals.var_temp1_dn6) * assign26460_e27798) + (assign26460_e27793 * (locals.var_temp1_dn6 * 0.3333333333333))))), ((locals.var_temp1_dn7 * assign26460_e27800) + (locals.var_temp1 * (((0.5 * locals.var_temp1_dn7) * assign26460_e27798) + (assign26460_e27793 * (locals.var_temp1_dn7 * 0.3333333333333))))), ((locals.var_temp1_dn8 * assign26460_e27800) + (locals.var_temp1 * (((0.5 * locals.var_temp1_dn8) * assign26460_e27798) + (assign26460_e27793 * (locals.var_temp1_dn8 * 0.3333333333333))))), ((locals.var_temp1_dn9 * assign26460_e27800) + (locals.var_temp1 * (((0.5 * locals.var_temp1_dn9) * assign26460_e27798) + (assign26460_e27793 * (locals.var_temp1_dn9 * 0.3333333333333))))),)
    } else {
        (locals.var_tp2, locals.var_tp2_dn4, locals.var_tp2_dn6, locals.var_tp2_dn7, locals.var_tp2_dn8, locals.var_tp2_dn9,)
    }
};
        locals.var_tp2 = assign26460_e27804;
        locals.var_tp2_dn4 = assign26460_e27804_d_n4;
        locals.var_tp2_dn6 = assign26460_e27804_d_n6;
        locals.var_tp2_dn7 = assign26460_e27804_d_n7;
        locals.var_tp2_dn8 = assign26460_e27804_d_n8;
        locals.var_tp2_dn9 = assign26460_e27804_d_n9;

        let assign26470_e27807: f64 = (-80.0);
        let assign26470_e27808: f64 = if locals.var_temp1 > assign26470_e27807 { 1.0 } else { 0.0 };
        locals.var_guard750 = assign26470_e27808;

        let (assign26480_e27820, assign26480_e27820_d_n4, assign26480_e27820_d_n6, assign26480_e27820_d_n7, assign26480_e27820_d_n8, assign26480_e27820_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard749 == 0.0)) && (locals.var_guard750 != 0.0)) {
        let assign26480_e27818: f64 = (locals.var_temp1).exp();
        (assign26480_e27818, (assign26480_e27818 * locals.var_temp1_dn4), (assign26480_e27818 * locals.var_temp1_dn6), (assign26480_e27818 * locals.var_temp1_dn7), (assign26480_e27818 * locals.var_temp1_dn8), (assign26480_e27818 * locals.var_temp1_dn9),)
    } else {
        (locals.var_tp2, locals.var_tp2_dn4, locals.var_tp2_dn6, locals.var_tp2_dn7, locals.var_tp2_dn8, locals.var_tp2_dn9,)
    }
};
        locals.var_tp2 = assign26480_e27820;
        locals.var_tp2_dn4 = assign26480_e27820_d_n4;
        locals.var_tp2_dn6 = assign26480_e27820_d_n6;
        locals.var_tp2_dn7 = assign26480_e27820_d_n7;
        locals.var_tp2_dn8 = assign26480_e27820_d_n8;
        locals.var_tp2_dn9 = assign26480_e27820_d_n9;

        let (assign26490_e27857, assign26490_e27857_d_n4, assign26490_e27857_d_n6, assign26490_e27857_d_n7, assign26490_e27857_d_n8, assign26490_e27857_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard749 == 0.0)) && (locals.var_guard750 == 0.0)) {
        let assign26490_e27833: f64 = (-locals.var_temp1);
        let assign26490_e27835: f64 = (assign26490_e27833 - 80.0);
        let assign26490_e27839: f64 = (-locals.var_temp1);
        let assign26490_e27841: f64 = (assign26490_e27839 - 80.0);
        let assign26490_e27842: f64 = (0.5 * assign26490_e27841);
        let assign26490_e27845: f64 = (-locals.var_temp1);
        let assign26490_e27847: f64 = (assign26490_e27845 - 80.0);
        let assign26490_e27849: f64 = (assign26490_e27847 * 0.3333333333333);
        let assign26490_e27850: f64 = (1.0 + assign26490_e27849);
        let assign26490_e27851: f64 = (assign26490_e27842 * assign26490_e27850);
        let assign26490_e27852: f64 = (1.0 + assign26490_e27851);
        let assign26490_e27853: f64 = (assign26490_e27835 * assign26490_e27852);
        let assign26490_e27854: f64 = (1.0 + assign26490_e27853);
        let assign26490_e27855: f64 = (1.80485e-35 / assign26490_e27854);
        (assign26490_e27855, (-((1.80485e-35 * (((-locals.var_temp1_dn4) * assign26490_e27852) + (assign26490_e27835 * (((0.5 * (-locals.var_temp1_dn4)) * assign26490_e27850) + (assign26490_e27842 * ((-locals.var_temp1_dn4) * 0.3333333333333)))))) / (assign26490_e27854 * assign26490_e27854))), (-((1.80485e-35 * (((-locals.var_temp1_dn6) * assign26490_e27852) + (assign26490_e27835 * (((0.5 * (-locals.var_temp1_dn6)) * assign26490_e27850) + (assign26490_e27842 * ((-locals.var_temp1_dn6) * 0.3333333333333)))))) / (assign26490_e27854 * assign26490_e27854))), (-((1.80485e-35 * (((-locals.var_temp1_dn7) * assign26490_e27852) + (assign26490_e27835 * (((0.5 * (-locals.var_temp1_dn7)) * assign26490_e27850) + (assign26490_e27842 * ((-locals.var_temp1_dn7) * 0.3333333333333)))))) / (assign26490_e27854 * assign26490_e27854))), (-((1.80485e-35 * (((-locals.var_temp1_dn8) * assign26490_e27852) + (assign26490_e27835 * (((0.5 * (-locals.var_temp1_dn8)) * assign26490_e27850) + (assign26490_e27842 * ((-locals.var_temp1_dn8) * 0.3333333333333)))))) / (assign26490_e27854 * assign26490_e27854))), (-((1.80485e-35 * (((-locals.var_temp1_dn9) * assign26490_e27852) + (assign26490_e27835 * (((0.5 * (-locals.var_temp1_dn9)) * assign26490_e27850) + (assign26490_e27842 * ((-locals.var_temp1_dn9) * 0.3333333333333)))))) / (assign26490_e27854 * assign26490_e27854))),)
    } else {
        (locals.var_tp2, locals.var_tp2_dn4, locals.var_tp2_dn6, locals.var_tp2_dn7, locals.var_tp2_dn8, locals.var_tp2_dn9,)
    }
};
        locals.var_tp2 = assign26490_e27857;
        locals.var_tp2_dn4 = assign26490_e27857_d_n4;
        locals.var_tp2_dn6 = assign26490_e27857_d_n6;
        locals.var_tp2_dn7 = assign26490_e27857_d_n7;
        locals.var_tp2_dn8 = assign26490_e27857_d_n8;
        locals.var_tp2_dn9 = assign26490_e27857_d_n9;

        let (assign26500_e27869, assign26500_e27869_d_n4, assign26500_e27869_d_n6, assign26500_e27869_d_n7, assign26500_e27869_d_n8, assign26500_e27869_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26500_e27863: f64 = (1.0 + locals.var_dsi);
        let assign26500_e27866: f64 = (1.0 + locals.var_dgate);
        let assign26500_e27867: f64 = (assign26500_e27863 / assign26500_e27866);
        (assign26500_e27867, (((locals.var_dsi_dn4 * assign26500_e27866) - (assign26500_e27863 * locals.var_dgate_dn4)) / (assign26500_e27866 * assign26500_e27866)), (((locals.var_dsi_dn6 * assign26500_e27866) - (assign26500_e27863 * locals.var_dgate_dn6)) / (assign26500_e27866 * assign26500_e27866)), (((locals.var_dsi_dn7 * assign26500_e27866) - (assign26500_e27863 * locals.var_dgate_dn7)) / (assign26500_e27866 * assign26500_e27866)), (((locals.var_dsi_dn8 * assign26500_e27866) - (assign26500_e27863 * locals.var_dgate_dn8)) / (assign26500_e27866 * assign26500_e27866)), (((locals.var_dsi_dn9 * assign26500_e27866) - (assign26500_e27863 * locals.var_dgate_dn9)) / (assign26500_e27866 * assign26500_e27866)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26500_e27869;
        locals.var_temp_dn4 = assign26500_e27869_d_n4;
        locals.var_temp_dn6 = assign26500_e27869_d_n6;
        locals.var_temp_dn7 = assign26500_e27869_d_n7;
        locals.var_temp_dn8 = assign26500_e27869_d_n8;
        locals.var_temp_dn9 = assign26500_e27869_d_n9;

        let assign26510_e27872: f64 = if locals.var_temp < 1e-80 { 1.0 } else { 0.0 };
        locals.var_guard751 = assign26510_e27872;

        let (assign26520_e27880, assign26520_e27880_d_n4, assign26520_e27880_d_n6, assign26520_e27880_d_n7, assign26520_e27880_d_n8, assign26520_e27880_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard751 != 0.0)) {
        (1e-80, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26520_e27880;
        locals.var_temp_dn4 = assign26520_e27880_d_n4;
        locals.var_temp_dn6 = assign26520_e27880_d_n6;
        locals.var_temp_dn7 = assign26520_e27880_d_n7;
        locals.var_temp_dn8 = assign26520_e27880_d_n8;
        locals.var_temp_dn9 = assign26520_e27880_d_n9;

        let (assign26530_e27890, assign26530_e27890_d_n4, assign26530_e27890_d_n6, assign26530_e27890_d_n7, assign26530_e27890_d_n8, assign26530_e27890_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26530_e27887: f64 = (locals.var_vgsu - locals.var_gcvdov_i);
        let assign26530_e27888: f64 = (locals.var_gcdov_i * assign26530_e27887);
        (assign26530_e27888, 0.0, (locals.var_gcdov_i * locals.var_vgsu_dn6), 0.0, 0.0, (locals.var_gcdov_i * locals.var_vgsu_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign26530_e27890;
        locals.var_temp1_dn4 = assign26530_e27890_d_n4;
        locals.var_temp1_dn6 = assign26530_e27890_d_n6;
        locals.var_temp1_dn7 = assign26530_e27890_d_n7;
        locals.var_temp1_dn8 = assign26530_e27890_d_n8;
        locals.var_temp1_dn9 = assign26530_e27890_d_n9;

        let assign26540_e27892: f64 = (locals.var_temp1).abs();
        let assign26540_e27894: f64 = if assign26540_e27892 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard752 = assign26540_e27894;

        let (assign26550_e27903, assign26550_e27903_d_n4, assign26550_e27903_d_n6, assign26550_e27903_d_n7, assign26550_e27903_d_n8, assign26550_e27903_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard752 != 0.0)) {
        let assign26550_e27901: f64 = (locals.var_temp1).exp();
        (assign26550_e27901, (assign26550_e27901 * locals.var_temp1_dn4), (assign26550_e27901 * locals.var_temp1_dn6), (assign26550_e27901 * locals.var_temp1_dn7), (assign26550_e27901 * locals.var_temp1_dn8), (assign26550_e27901 * locals.var_temp1_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign26550_e27903;
        locals.var_temp2_dn4 = assign26550_e27903_d_n4;
        locals.var_temp2_dn6 = assign26550_e27903_d_n6;
        locals.var_temp2_dn7 = assign26550_e27903_d_n7;
        locals.var_temp2_dn8 = assign26550_e27903_d_n8;
        locals.var_temp2_dn9 = assign26550_e27903_d_n9;

        let assign26560_e27906: f64 = (-80.0);
        let assign26560_e27907: f64 = if locals.var_temp1 < assign26560_e27906 { 1.0 } else { 0.0 };
        locals.var_guard753 = assign26560_e27907;

        let (assign26570_e27943, assign26570_e27943_d_n4, assign26570_e27943_d_n6, assign26570_e27943_d_n7, assign26570_e27943_d_n8, assign26570_e27943_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard752 == 0.0)) && (locals.var_guard753 != 0.0)) {
        let assign26570_e27919: f64 = (-locals.var_temp1);
        let assign26570_e27921: f64 = (assign26570_e27919 - 80.0);
        let assign26570_e27925: f64 = (-locals.var_temp1);
        let assign26570_e27927: f64 = (assign26570_e27925 - 80.0);
        let assign26570_e27928: f64 = (0.5 * assign26570_e27927);
        let assign26570_e27931: f64 = (-locals.var_temp1);
        let assign26570_e27933: f64 = (assign26570_e27931 - 80.0);
        let assign26570_e27935: f64 = (assign26570_e27933 * 0.3333333333333);
        let assign26570_e27936: f64 = (1.0 + assign26570_e27935);
        let assign26570_e27937: f64 = (assign26570_e27928 * assign26570_e27936);
        let assign26570_e27938: f64 = (1.0 + assign26570_e27937);
        let assign26570_e27939: f64 = (assign26570_e27921 * assign26570_e27938);
        let assign26570_e27940: f64 = (1.0 + assign26570_e27939);
        let assign26570_e27941: f64 = (1.80485e-35 / assign26570_e27940);
        (assign26570_e27941, (-((1.80485e-35 * (((-locals.var_temp1_dn4) * assign26570_e27938) + (assign26570_e27921 * (((0.5 * (-locals.var_temp1_dn4)) * assign26570_e27936) + (assign26570_e27928 * ((-locals.var_temp1_dn4) * 0.3333333333333)))))) / (assign26570_e27940 * assign26570_e27940))), (-((1.80485e-35 * (((-locals.var_temp1_dn6) * assign26570_e27938) + (assign26570_e27921 * (((0.5 * (-locals.var_temp1_dn6)) * assign26570_e27936) + (assign26570_e27928 * ((-locals.var_temp1_dn6) * 0.3333333333333)))))) / (assign26570_e27940 * assign26570_e27940))), (-((1.80485e-35 * (((-locals.var_temp1_dn7) * assign26570_e27938) + (assign26570_e27921 * (((0.5 * (-locals.var_temp1_dn7)) * assign26570_e27936) + (assign26570_e27928 * ((-locals.var_temp1_dn7) * 0.3333333333333)))))) / (assign26570_e27940 * assign26570_e27940))), (-((1.80485e-35 * (((-locals.var_temp1_dn8) * assign26570_e27938) + (assign26570_e27921 * (((0.5 * (-locals.var_temp1_dn8)) * assign26570_e27936) + (assign26570_e27928 * ((-locals.var_temp1_dn8) * 0.3333333333333)))))) / (assign26570_e27940 * assign26570_e27940))), (-((1.80485e-35 * (((-locals.var_temp1_dn9) * assign26570_e27938) + (assign26570_e27921 * (((0.5 * (-locals.var_temp1_dn9)) * assign26570_e27936) + (assign26570_e27928 * ((-locals.var_temp1_dn9) * 0.3333333333333)))))) / (assign26570_e27940 * assign26570_e27940))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign26570_e27943;
        locals.var_temp2_dn4 = assign26570_e27943_d_n4;
        locals.var_temp2_dn6 = assign26570_e27943_d_n6;
        locals.var_temp2_dn7 = assign26570_e27943_d_n7;
        locals.var_temp2_dn8 = assign26570_e27943_d_n8;
        locals.var_temp2_dn9 = assign26570_e27943_d_n9;

        let (assign26580_e27977, assign26580_e27977_d_n4, assign26580_e27977_d_n6, assign26580_e27977_d_n7, assign26580_e27977_d_n8, assign26580_e27977_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard752 == 0.0)) && (locals.var_guard753 == 0.0)) {
        let assign26580_e27957: f64 = (locals.var_temp1 - 80.0);
        let assign26580_e27962: f64 = (locals.var_temp1 - 80.0);
        let assign26580_e27963: f64 = (0.5 * assign26580_e27962);
        let assign26580_e27967: f64 = (locals.var_temp1 - 80.0);
        let assign26580_e27969: f64 = (assign26580_e27967 * 0.3333333333333);
        let assign26580_e27970: f64 = (1.0 + assign26580_e27969);
        let assign26580_e27971: f64 = (assign26580_e27963 * assign26580_e27970);
        let assign26580_e27972: f64 = (1.0 + assign26580_e27971);
        let assign26580_e27973: f64 = (assign26580_e27957 * assign26580_e27972);
        let assign26580_e27974: f64 = (1.0 + assign26580_e27973);
        let assign26580_e27975: f64 = (5.54062e34 * assign26580_e27974);
        (assign26580_e27975, (5.54062e34 * ((locals.var_temp1_dn4 * assign26580_e27972) + (assign26580_e27957 * (((0.5 * locals.var_temp1_dn4) * assign26580_e27970) + (assign26580_e27963 * (locals.var_temp1_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp1_dn6 * assign26580_e27972) + (assign26580_e27957 * (((0.5 * locals.var_temp1_dn6) * assign26580_e27970) + (assign26580_e27963 * (locals.var_temp1_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp1_dn7 * assign26580_e27972) + (assign26580_e27957 * (((0.5 * locals.var_temp1_dn7) * assign26580_e27970) + (assign26580_e27963 * (locals.var_temp1_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp1_dn8 * assign26580_e27972) + (assign26580_e27957 * (((0.5 * locals.var_temp1_dn8) * assign26580_e27970) + (assign26580_e27963 * (locals.var_temp1_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp1_dn9 * assign26580_e27972) + (assign26580_e27957 * (((0.5 * locals.var_temp1_dn9) * assign26580_e27970) + (assign26580_e27963 * (locals.var_temp1_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign26580_e27977;
        locals.var_temp2_dn4 = assign26580_e27977_d_n4;
        locals.var_temp2_dn6 = assign26580_e27977_d_n6;
        locals.var_temp2_dn7 = assign26580_e27977_d_n7;
        locals.var_temp2_dn8 = assign26580_e27977_d_n8;
        locals.var_temp2_dn9 = assign26580_e27977_d_n9;

        let (assign26590_e27987, assign26590_e27987_d_n4, assign26590_e27987_d_n6, assign26590_e27987_d_n7, assign26590_e27987_d_n8, assign26590_e27987_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26590_e27983: f64 = (locals.var_gcdov_i * locals.var_vdsu);
        let assign26590_e27985: f64 = (assign26590_e27983 + locals.var_temp1);
        (assign26590_e27985, locals.var_temp1_dn4, ((locals.var_gcdov_i * locals.var_vdsu_dn6) + locals.var_temp1_dn6), ((locals.var_gcdov_i * locals.var_vdsu_dn7) + locals.var_temp1_dn7), locals.var_temp1_dn8, locals.var_temp1_dn9,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign26590_e27987;
        locals.var_temp3_dn4 = assign26590_e27987_d_n4;
        locals.var_temp3_dn6 = assign26590_e27987_d_n6;
        locals.var_temp3_dn7 = assign26590_e27987_d_n7;
        locals.var_temp3_dn8 = assign26590_e27987_d_n8;
        locals.var_temp3_dn9 = assign26590_e27987_d_n9;

        let assign26600_e27989: f64 = (locals.var_temp3).abs();
        let assign26600_e27991: f64 = if assign26600_e27989 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard754 = assign26600_e27991;

        let (assign26610_e28000, assign26610_e28000_d_n4, assign26610_e28000_d_n6, assign26610_e28000_d_n7, assign26610_e28000_d_n8, assign26610_e28000_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard754 != 0.0)) {
        let assign26610_e27998: f64 = (locals.var_temp3).exp();
        (assign26610_e27998, (assign26610_e27998 * locals.var_temp3_dn4), (assign26610_e27998 * locals.var_temp3_dn6), (assign26610_e27998 * locals.var_temp3_dn7), (assign26610_e27998 * locals.var_temp3_dn8), (assign26610_e27998 * locals.var_temp3_dn9),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign26610_e28000;
        locals.var_temp4_dn4 = assign26610_e28000_d_n4;
        locals.var_temp4_dn6 = assign26610_e28000_d_n6;
        locals.var_temp4_dn7 = assign26610_e28000_d_n7;
        locals.var_temp4_dn8 = assign26610_e28000_d_n8;
        locals.var_temp4_dn9 = assign26610_e28000_d_n9;

        let assign26620_e28003: f64 = (-80.0);
        let assign26620_e28004: f64 = if locals.var_temp3 < assign26620_e28003 { 1.0 } else { 0.0 };
        locals.var_guard755 = assign26620_e28004;

        let (assign26630_e28040, assign26630_e28040_d_n4, assign26630_e28040_d_n6, assign26630_e28040_d_n7, assign26630_e28040_d_n8, assign26630_e28040_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard754 == 0.0)) && (locals.var_guard755 != 0.0)) {
        let assign26630_e28016: f64 = (-locals.var_temp3);
        let assign26630_e28018: f64 = (assign26630_e28016 - 80.0);
        let assign26630_e28022: f64 = (-locals.var_temp3);
        let assign26630_e28024: f64 = (assign26630_e28022 - 80.0);
        let assign26630_e28025: f64 = (0.5 * assign26630_e28024);
        let assign26630_e28028: f64 = (-locals.var_temp3);
        let assign26630_e28030: f64 = (assign26630_e28028 - 80.0);
        let assign26630_e28032: f64 = (assign26630_e28030 * 0.3333333333333);
        let assign26630_e28033: f64 = (1.0 + assign26630_e28032);
        let assign26630_e28034: f64 = (assign26630_e28025 * assign26630_e28033);
        let assign26630_e28035: f64 = (1.0 + assign26630_e28034);
        let assign26630_e28036: f64 = (assign26630_e28018 * assign26630_e28035);
        let assign26630_e28037: f64 = (1.0 + assign26630_e28036);
        let assign26630_e28038: f64 = (1.80485e-35 / assign26630_e28037);
        (assign26630_e28038, (-((1.80485e-35 * (((-locals.var_temp3_dn4) * assign26630_e28035) + (assign26630_e28018 * (((0.5 * (-locals.var_temp3_dn4)) * assign26630_e28033) + (assign26630_e28025 * ((-locals.var_temp3_dn4) * 0.3333333333333)))))) / (assign26630_e28037 * assign26630_e28037))), (-((1.80485e-35 * (((-locals.var_temp3_dn6) * assign26630_e28035) + (assign26630_e28018 * (((0.5 * (-locals.var_temp3_dn6)) * assign26630_e28033) + (assign26630_e28025 * ((-locals.var_temp3_dn6) * 0.3333333333333)))))) / (assign26630_e28037 * assign26630_e28037))), (-((1.80485e-35 * (((-locals.var_temp3_dn7) * assign26630_e28035) + (assign26630_e28018 * (((0.5 * (-locals.var_temp3_dn7)) * assign26630_e28033) + (assign26630_e28025 * ((-locals.var_temp3_dn7) * 0.3333333333333)))))) / (assign26630_e28037 * assign26630_e28037))), (-((1.80485e-35 * (((-locals.var_temp3_dn8) * assign26630_e28035) + (assign26630_e28018 * (((0.5 * (-locals.var_temp3_dn8)) * assign26630_e28033) + (assign26630_e28025 * ((-locals.var_temp3_dn8) * 0.3333333333333)))))) / (assign26630_e28037 * assign26630_e28037))), (-((1.80485e-35 * (((-locals.var_temp3_dn9) * assign26630_e28035) + (assign26630_e28018 * (((0.5 * (-locals.var_temp3_dn9)) * assign26630_e28033) + (assign26630_e28025 * ((-locals.var_temp3_dn9) * 0.3333333333333)))))) / (assign26630_e28037 * assign26630_e28037))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign26630_e28040;
        locals.var_temp4_dn4 = assign26630_e28040_d_n4;
        locals.var_temp4_dn6 = assign26630_e28040_d_n6;
        locals.var_temp4_dn7 = assign26630_e28040_d_n7;
        locals.var_temp4_dn8 = assign26630_e28040_d_n8;
        locals.var_temp4_dn9 = assign26630_e28040_d_n9;

        let (assign26640_e28074, assign26640_e28074_d_n4, assign26640_e28074_d_n6, assign26640_e28074_d_n7, assign26640_e28074_d_n8, assign26640_e28074_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) && (locals.var_guard754 == 0.0)) && (locals.var_guard755 == 0.0)) {
        let assign26640_e28054: f64 = (locals.var_temp3 - 80.0);
        let assign26640_e28059: f64 = (locals.var_temp3 - 80.0);
        let assign26640_e28060: f64 = (0.5 * assign26640_e28059);
        let assign26640_e28064: f64 = (locals.var_temp3 - 80.0);
        let assign26640_e28066: f64 = (assign26640_e28064 * 0.3333333333333);
        let assign26640_e28067: f64 = (1.0 + assign26640_e28066);
        let assign26640_e28068: f64 = (assign26640_e28060 * assign26640_e28067);
        let assign26640_e28069: f64 = (1.0 + assign26640_e28068);
        let assign26640_e28070: f64 = (assign26640_e28054 * assign26640_e28069);
        let assign26640_e28071: f64 = (1.0 + assign26640_e28070);
        let assign26640_e28072: f64 = (5.54062e34 * assign26640_e28071);
        (assign26640_e28072, (5.54062e34 * ((locals.var_temp3_dn4 * assign26640_e28069) + (assign26640_e28054 * (((0.5 * locals.var_temp3_dn4) * assign26640_e28067) + (assign26640_e28060 * (locals.var_temp3_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn6 * assign26640_e28069) + (assign26640_e28054 * (((0.5 * locals.var_temp3_dn6) * assign26640_e28067) + (assign26640_e28060 * (locals.var_temp3_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn7 * assign26640_e28069) + (assign26640_e28054 * (((0.5 * locals.var_temp3_dn7) * assign26640_e28067) + (assign26640_e28060 * (locals.var_temp3_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn8 * assign26640_e28069) + (assign26640_e28054 * (((0.5 * locals.var_temp3_dn8) * assign26640_e28067) + (assign26640_e28060 * (locals.var_temp3_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn9 * assign26640_e28069) + (assign26640_e28054 * (((0.5 * locals.var_temp3_dn9) * assign26640_e28067) + (assign26640_e28060 * (locals.var_temp3_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign26640_e28074;
        locals.var_temp4_dn4 = assign26640_e28074_d_n4;
        locals.var_temp4_dn6 = assign26640_e28074_d_n6;
        locals.var_temp4_dn7 = assign26640_e28074_d_n7;
        locals.var_temp4_dn8 = assign26640_e28074_d_n8;
        locals.var_temp4_dn9 = assign26640_e28074_d_n9;

    }

    pub(super) fn stamp_transient_block_69(
        locals: &mut StampLocals,
    ) {
        let (assign26650_e28105, assign26650_e28105_d_n4, assign26650_e28105_d_n6, assign26650_e28105_d_n7, assign26650_e28105_d_n8, assign26650_e28105_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign26650_e28080: f64 = (locals.var_igoveff * locals.var_tp);
        let assign26650_e28082: f64 = (locals.var_temp).ln();
        let assign26650_e28083: f64 = (assign26650_e28080 * assign26650_e28082);
        let assign26650_e28086: f64 = (1.0 + locals.var_temp2);
        let assign26650_e28087: f64 = (assign26650_e28083 * assign26650_e28086);
        let assign26650_e28090: f64 = (1.0 + locals.var_temp4);
        let assign26650_e28091: f64 = (assign26650_e28087 / assign26650_e28090);
        let assign26650_e28094: f64 = (locals.var_igovefffowler * locals.var_tp2);
        let assign26650_e28097: f64 = (1.0 + locals.var_temp2);
        let assign26650_e28098: f64 = (assign26650_e28094 * assign26650_e28097);
        let assign26650_e28101: f64 = (1.0 + locals.var_temp4);
        let assign26650_e28102: f64 = (assign26650_e28098 / assign26650_e28101);
        let assign26650_e28103: f64 = (assign26650_e28091 - assign26650_e28102);
        (assign26650_e28103, ((((((((((locals.var_igoveff_dn4 * locals.var_tp) + (locals.var_igoveff * locals.var_tp_dn4)) * assign26650_e28082) + (assign26650_e28080 * (locals.var_temp_dn4 / locals.var_temp))) * assign26650_e28086) + (assign26650_e28083 * locals.var_temp2_dn4)) * assign26650_e28090) - (assign26650_e28087 * locals.var_temp4_dn4)) / (assign26650_e28090 * assign26650_e28090)) - (((((((locals.var_igovefffowler_dn4 * locals.var_tp2) + (locals.var_igovefffowler * locals.var_tp2_dn4)) * assign26650_e28097) + (assign26650_e28094 * locals.var_temp2_dn4)) * assign26650_e28101) - (assign26650_e28098 * locals.var_temp4_dn4)) / (assign26650_e28101 * assign26650_e28101))), ((((((((((locals.var_igoveff_dn6 * locals.var_tp) + (locals.var_igoveff * locals.var_tp_dn6)) * assign26650_e28082) + (assign26650_e28080 * (locals.var_temp_dn6 / locals.var_temp))) * assign26650_e28086) + (assign26650_e28083 * locals.var_temp2_dn6)) * assign26650_e28090) - (assign26650_e28087 * locals.var_temp4_dn6)) / (assign26650_e28090 * assign26650_e28090)) - (((((((locals.var_igovefffowler_dn6 * locals.var_tp2) + (locals.var_igovefffowler * locals.var_tp2_dn6)) * assign26650_e28097) + (assign26650_e28094 * locals.var_temp2_dn6)) * assign26650_e28101) - (assign26650_e28098 * locals.var_temp4_dn6)) / (assign26650_e28101 * assign26650_e28101))), ((((((((((locals.var_igoveff_dn7 * locals.var_tp) + (locals.var_igoveff * locals.var_tp_dn7)) * assign26650_e28082) + (assign26650_e28080 * (locals.var_temp_dn7 / locals.var_temp))) * assign26650_e28086) + (assign26650_e28083 * locals.var_temp2_dn7)) * assign26650_e28090) - (assign26650_e28087 * locals.var_temp4_dn7)) / (assign26650_e28090 * assign26650_e28090)) - (((((((locals.var_igovefffowler_dn7 * locals.var_tp2) + (locals.var_igovefffowler * locals.var_tp2_dn7)) * assign26650_e28097) + (assign26650_e28094 * locals.var_temp2_dn7)) * assign26650_e28101) - (assign26650_e28098 * locals.var_temp4_dn7)) / (assign26650_e28101 * assign26650_e28101))), ((((((((((locals.var_igoveff_dn8 * locals.var_tp) + (locals.var_igoveff * locals.var_tp_dn8)) * assign26650_e28082) + (assign26650_e28080 * (locals.var_temp_dn8 / locals.var_temp))) * assign26650_e28086) + (assign26650_e28083 * locals.var_temp2_dn8)) * assign26650_e28090) - (assign26650_e28087 * locals.var_temp4_dn8)) / (assign26650_e28090 * assign26650_e28090)) - (((((((locals.var_igovefffowler_dn8 * locals.var_tp2) + (locals.var_igovefffowler * locals.var_tp2_dn8)) * assign26650_e28097) + (assign26650_e28094 * locals.var_temp2_dn8)) * assign26650_e28101) - (assign26650_e28098 * locals.var_temp4_dn8)) / (assign26650_e28101 * assign26650_e28101))), ((((((((((locals.var_igoveff_dn9 * locals.var_tp) + (locals.var_igoveff * locals.var_tp_dn9)) * assign26650_e28082) + (assign26650_e28080 * (locals.var_temp_dn9 / locals.var_temp))) * assign26650_e28086) + (assign26650_e28083 * locals.var_temp2_dn9)) * assign26650_e28090) - (assign26650_e28087 * locals.var_temp4_dn9)) / (assign26650_e28090 * assign26650_e28090)) - (((((((locals.var_igovefffowler_dn9 * locals.var_tp2) + (locals.var_igovefffowler * locals.var_tp2_dn9)) * assign26650_e28097) + (assign26650_e28094 * locals.var_temp2_dn9)) * assign26650_e28101) - (assign26650_e28098 * locals.var_temp4_dn9)) / (assign26650_e28101 * assign26650_e28101))),)
    } else {
        (locals.var_igdov, locals.var_igdov_dn4, locals.var_igdov_dn6, locals.var_igdov_dn7, locals.var_igdov_dn8, locals.var_igdov_dn9,)
    }
};
        locals.var_igdov = assign26650_e28105;
        locals.var_igdov_dn4 = assign26650_e28105_d_n4;
        locals.var_igdov_dn6 = assign26650_e28105_d_n6;
        locals.var_igdov_dn7 = assign26650_e28105_d_n7;
        locals.var_igdov_dn8 = assign26650_e28105_d_n8;
        locals.var_igdov_dn9 = assign26650_e28105_d_n9;

        let assign26660_e28108: f64 = if locals.var_iginv_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard756 = assign26660_e28108;

        let (assign26670_e28117, assign26670_e28117_d_n4, assign26670_e28117_d_n6, assign26670_e28117_d_n7, assign26670_e28117_d_n8, assign26670_e28117_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26670_e28113: f64 = (-locals.var_delta_k1q1_dc);
        let assign26670_e28115: f64 = (assign26670_e28113 * locals.var_inv_k1_dc);
        (assign26670_e28115, (((-locals.var_delta_k1q1_dc_dn4) * locals.var_inv_k1_dc) + (assign26670_e28113 * locals.var_inv_k1_dc_dn4)), (((-locals.var_delta_k1q1_dc_dn6) * locals.var_inv_k1_dc) + (assign26670_e28113 * locals.var_inv_k1_dc_dn6)), (((-locals.var_delta_k1q1_dc_dn7) * locals.var_inv_k1_dc) + (assign26670_e28113 * locals.var_inv_k1_dc_dn7)), (((-locals.var_delta_k1q1_dc_dn8) * locals.var_inv_k1_dc) + (assign26670_e28113 * locals.var_inv_k1_dc_dn8)), (((-locals.var_delta_k1q1_dc_dn9) * locals.var_inv_k1_dc) + (assign26670_e28113 * locals.var_inv_k1_dc_dn9)),)
    } else {
        (locals.var_half_x_ds, locals.var_half_x_ds_dn4, locals.var_half_x_ds_dn6, locals.var_half_x_ds_dn7, locals.var_half_x_ds_dn8, locals.var_half_x_ds_dn9,)
    }
};
        locals.var_half_x_ds = assign26670_e28117;
        locals.var_half_x_ds_dn4 = assign26670_e28117_d_n4;
        locals.var_half_x_ds_dn6 = assign26670_e28117_d_n6;
        locals.var_half_x_ds_dn7 = assign26670_e28117_d_n7;
        locals.var_half_x_ds_dn8 = assign26670_e28117_d_n8;
        locals.var_half_x_ds_dn9 = assign26670_e28117_d_n9;

        let assign26680_e28120: f64 = (2.0 * locals.var_half_x_ds);
        let assign26680_e28122: f64 = (assign26680_e28120 - locals.var_xdeff_dc);
        let assign26680_e28123: f64 = (assign26680_e28122).abs();
        let assign26680_e28125: f64 = if assign26680_e28123 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard757 = assign26680_e28125;

        let (assign26690_e28138, assign26690_e28138_d_n4, assign26690_e28138_d_n6, assign26690_e28138_d_n7, assign26690_e28138_d_n8, assign26690_e28138_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard757 != 0.0)) {
        let assign26690_e28133: f64 = (2.0 * locals.var_half_x_ds);
        let assign26690_e28135: f64 = (assign26690_e28133 - locals.var_xdeff_dc);
        let assign26690_e28136: f64 = (assign26690_e28135).exp();
        (assign26690_e28136, (assign26690_e28136 * ((2.0 * locals.var_half_x_ds_dn4) - locals.var_xdeff_dc_dn4)), (assign26690_e28136 * ((2.0 * locals.var_half_x_ds_dn6) - locals.var_xdeff_dc_dn6)), (assign26690_e28136 * ((2.0 * locals.var_half_x_ds_dn7) - locals.var_xdeff_dc_dn7)), (assign26690_e28136 * ((2.0 * locals.var_half_x_ds_dn8) - locals.var_xdeff_dc_dn8)), (assign26690_e28136 * ((2.0 * locals.var_half_x_ds_dn9) - locals.var_xdeff_dc_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26690_e28138;
        locals.var_temp_dn4 = assign26690_e28138_d_n4;
        locals.var_temp_dn6 = assign26690_e28138_d_n6;
        locals.var_temp_dn7 = assign26690_e28138_d_n7;
        locals.var_temp_dn8 = assign26690_e28138_d_n8;
        locals.var_temp_dn9 = assign26690_e28138_d_n9;

        let assign26700_e28141: f64 = (2.0 * locals.var_half_x_ds);
        let assign26700_e28143: f64 = (assign26700_e28141 - locals.var_xdeff_dc);
        let assign26700_e28145: f64 = (-80.0);
        let assign26700_e28146: f64 = if assign26700_e28143 < assign26700_e28145 { 1.0 } else { 0.0 };
        locals.var_guard758 = assign26700_e28146;

        let (assign26710_e28194, assign26710_e28194_d_n4, assign26710_e28194_d_n6, assign26710_e28194_d_n7, assign26710_e28194_d_n8, assign26710_e28194_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard757 == 0.0)) && (locals.var_guard758 != 0.0)) {
        let assign26710_e28159: f64 = (2.0 * locals.var_half_x_ds);
        let assign26710_e28161: f64 = (assign26710_e28159 - locals.var_xdeff_dc);
        let assign26710_e28162: f64 = (-assign26710_e28161);
        let assign26710_e28164: f64 = (assign26710_e28162 - 80.0);
        let assign26710_e28169: f64 = (2.0 * locals.var_half_x_ds);
        let assign26710_e28171: f64 = (assign26710_e28169 - locals.var_xdeff_dc);
        let assign26710_e28172: f64 = (-assign26710_e28171);
        let assign26710_e28174: f64 = (assign26710_e28172 - 80.0);
        let assign26710_e28175: f64 = (0.5 * assign26710_e28174);
        let assign26710_e28179: f64 = (2.0 * locals.var_half_x_ds);
        let assign26710_e28181: f64 = (assign26710_e28179 - locals.var_xdeff_dc);
        let assign26710_e28182: f64 = (-assign26710_e28181);
        let assign26710_e28184: f64 = (assign26710_e28182 - 80.0);
        let assign26710_e28186: f64 = (assign26710_e28184 * 0.3333333333333);
        let assign26710_e28187: f64 = (1.0 + assign26710_e28186);
        let assign26710_e28188: f64 = (assign26710_e28175 * assign26710_e28187);
        let assign26710_e28189: f64 = (1.0 + assign26710_e28188);
        let assign26710_e28190: f64 = (assign26710_e28164 * assign26710_e28189);
        let assign26710_e28191: f64 = (1.0 + assign26710_e28190);
        let assign26710_e28192: f64 = (1.80485e-35 / assign26710_e28191);
        (assign26710_e28192, (-((1.80485e-35 * (((-((2.0 * locals.var_half_x_ds_dn4) - locals.var_xdeff_dc_dn4)) * assign26710_e28189) + (assign26710_e28164 * (((0.5 * (-((2.0 * locals.var_half_x_ds_dn4) - locals.var_xdeff_dc_dn4))) * assign26710_e28187) + (assign26710_e28175 * ((-((2.0 * locals.var_half_x_ds_dn4) - locals.var_xdeff_dc_dn4)) * 0.3333333333333)))))) / (assign26710_e28191 * assign26710_e28191))), (-((1.80485e-35 * (((-((2.0 * locals.var_half_x_ds_dn6) - locals.var_xdeff_dc_dn6)) * assign26710_e28189) + (assign26710_e28164 * (((0.5 * (-((2.0 * locals.var_half_x_ds_dn6) - locals.var_xdeff_dc_dn6))) * assign26710_e28187) + (assign26710_e28175 * ((-((2.0 * locals.var_half_x_ds_dn6) - locals.var_xdeff_dc_dn6)) * 0.3333333333333)))))) / (assign26710_e28191 * assign26710_e28191))), (-((1.80485e-35 * (((-((2.0 * locals.var_half_x_ds_dn7) - locals.var_xdeff_dc_dn7)) * assign26710_e28189) + (assign26710_e28164 * (((0.5 * (-((2.0 * locals.var_half_x_ds_dn7) - locals.var_xdeff_dc_dn7))) * assign26710_e28187) + (assign26710_e28175 * ((-((2.0 * locals.var_half_x_ds_dn7) - locals.var_xdeff_dc_dn7)) * 0.3333333333333)))))) / (assign26710_e28191 * assign26710_e28191))), (-((1.80485e-35 * (((-((2.0 * locals.var_half_x_ds_dn8) - locals.var_xdeff_dc_dn8)) * assign26710_e28189) + (assign26710_e28164 * (((0.5 * (-((2.0 * locals.var_half_x_ds_dn8) - locals.var_xdeff_dc_dn8))) * assign26710_e28187) + (assign26710_e28175 * ((-((2.0 * locals.var_half_x_ds_dn8) - locals.var_xdeff_dc_dn8)) * 0.3333333333333)))))) / (assign26710_e28191 * assign26710_e28191))), (-((1.80485e-35 * (((-((2.0 * locals.var_half_x_ds_dn9) - locals.var_xdeff_dc_dn9)) * assign26710_e28189) + (assign26710_e28164 * (((0.5 * (-((2.0 * locals.var_half_x_ds_dn9) - locals.var_xdeff_dc_dn9))) * assign26710_e28187) + (assign26710_e28175 * ((-((2.0 * locals.var_half_x_ds_dn9) - locals.var_xdeff_dc_dn9)) * 0.3333333333333)))))) / (assign26710_e28191 * assign26710_e28191))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26710_e28194;
        locals.var_temp_dn4 = assign26710_e28194_d_n4;
        locals.var_temp_dn6 = assign26710_e28194_d_n6;
        locals.var_temp_dn7 = assign26710_e28194_d_n7;
        locals.var_temp_dn8 = assign26710_e28194_d_n8;
        locals.var_temp_dn9 = assign26710_e28194_d_n9;

        let (assign26720_e28240, assign26720_e28240_d_n4, assign26720_e28240_d_n6, assign26720_e28240_d_n7, assign26720_e28240_d_n8, assign26720_e28240_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard757 == 0.0)) && (locals.var_guard758 == 0.0)) {
        let assign26720_e28208: f64 = (2.0 * locals.var_half_x_ds);
        let assign26720_e28210: f64 = (assign26720_e28208 - locals.var_xdeff_dc);
        let assign26720_e28212: f64 = (assign26720_e28210 - 80.0);
        let assign26720_e28217: f64 = (2.0 * locals.var_half_x_ds);
        let assign26720_e28219: f64 = (assign26720_e28217 - locals.var_xdeff_dc);
        let assign26720_e28221: f64 = (assign26720_e28219 - 80.0);
        let assign26720_e28222: f64 = (0.5 * assign26720_e28221);
        let assign26720_e28226: f64 = (2.0 * locals.var_half_x_ds);
        let assign26720_e28228: f64 = (assign26720_e28226 - locals.var_xdeff_dc);
        let assign26720_e28230: f64 = (assign26720_e28228 - 80.0);
        let assign26720_e28232: f64 = (assign26720_e28230 * 0.3333333333333);
        let assign26720_e28233: f64 = (1.0 + assign26720_e28232);
        let assign26720_e28234: f64 = (assign26720_e28222 * assign26720_e28233);
        let assign26720_e28235: f64 = (1.0 + assign26720_e28234);
        let assign26720_e28236: f64 = (assign26720_e28212 * assign26720_e28235);
        let assign26720_e28237: f64 = (1.0 + assign26720_e28236);
        let assign26720_e28238: f64 = (5.54062e34 * assign26720_e28237);
        (assign26720_e28238, (5.54062e34 * ((((2.0 * locals.var_half_x_ds_dn4) - locals.var_xdeff_dc_dn4) * assign26720_e28235) + (assign26720_e28212 * (((0.5 * ((2.0 * locals.var_half_x_ds_dn4) - locals.var_xdeff_dc_dn4)) * assign26720_e28233) + (assign26720_e28222 * (((2.0 * locals.var_half_x_ds_dn4) - locals.var_xdeff_dc_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((2.0 * locals.var_half_x_ds_dn6) - locals.var_xdeff_dc_dn6) * assign26720_e28235) + (assign26720_e28212 * (((0.5 * ((2.0 * locals.var_half_x_ds_dn6) - locals.var_xdeff_dc_dn6)) * assign26720_e28233) + (assign26720_e28222 * (((2.0 * locals.var_half_x_ds_dn6) - locals.var_xdeff_dc_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((2.0 * locals.var_half_x_ds_dn7) - locals.var_xdeff_dc_dn7) * assign26720_e28235) + (assign26720_e28212 * (((0.5 * ((2.0 * locals.var_half_x_ds_dn7) - locals.var_xdeff_dc_dn7)) * assign26720_e28233) + (assign26720_e28222 * (((2.0 * locals.var_half_x_ds_dn7) - locals.var_xdeff_dc_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((2.0 * locals.var_half_x_ds_dn8) - locals.var_xdeff_dc_dn8) * assign26720_e28235) + (assign26720_e28212 * (((0.5 * ((2.0 * locals.var_half_x_ds_dn8) - locals.var_xdeff_dc_dn8)) * assign26720_e28233) + (assign26720_e28222 * (((2.0 * locals.var_half_x_ds_dn8) - locals.var_xdeff_dc_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((2.0 * locals.var_half_x_ds_dn9) - locals.var_xdeff_dc_dn9) * assign26720_e28235) + (assign26720_e28212 * (((0.5 * ((2.0 * locals.var_half_x_ds_dn9) - locals.var_xdeff_dc_dn9)) * assign26720_e28233) + (assign26720_e28222 * (((2.0 * locals.var_half_x_ds_dn9) - locals.var_xdeff_dc_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26720_e28240;
        locals.var_temp_dn4 = assign26720_e28240_d_n4;
        locals.var_temp_dn6 = assign26720_e28240_d_n6;
        locals.var_temp_dn7 = assign26720_e28240_d_n7;
        locals.var_temp_dn8 = assign26720_e28240_d_n8;
        locals.var_temp_dn9 = assign26720_e28240_d_n9;

        let (assign26730_e28255, assign26730_e28255_d_n4, assign26730_e28255_d_n6, assign26730_e28255_d_n7, assign26730_e28255_d_n8, assign26730_e28255_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26730_e28247: f64 = (locals.var_half_x_ds + 0.6931471805599);
        let assign26730_e28250: f64 = (1.0 + locals.var_temp);
        let assign26730_e28251: f64 = (assign26730_e28250).ln();
        let assign26730_e28252: f64 = (assign26730_e28247 - assign26730_e28251);
        let assign26730_e28253: f64 = (locals.var_phit * assign26730_e28252);
        (assign26730_e28253, ((locals.var_phit_dn4 * assign26730_e28252) + (locals.var_phit * (locals.var_half_x_ds_dn4 - (locals.var_temp_dn4 / assign26730_e28250)))), ((locals.var_phit_dn6 * assign26730_e28252) + (locals.var_phit * (locals.var_half_x_ds_dn6 - (locals.var_temp_dn6 / assign26730_e28250)))), ((locals.var_phit_dn7 * assign26730_e28252) + (locals.var_phit * (locals.var_half_x_ds_dn7 - (locals.var_temp_dn7 / assign26730_e28250)))), ((locals.var_phit_dn8 * assign26730_e28252) + (locals.var_phit * (locals.var_half_x_ds_dn8 - (locals.var_temp_dn8 / assign26730_e28250)))), ((locals.var_phit_dn9 * assign26730_e28252) + (locals.var_phit * (locals.var_half_x_ds_dn9 - (locals.var_temp_dn9 / assign26730_e28250)))),)
    } else {
        (locals.var_vm, locals.var_vm_dn4, locals.var_vm_dn6, locals.var_vm_dn7, locals.var_vm_dn8, locals.var_vm_dn9,)
    }
};
        locals.var_vm = assign26730_e28255;
        locals.var_vm_dn4 = assign26730_e28255_d_n4;
        locals.var_vm_dn6 = assign26730_e28255_d_n6;
        locals.var_vm_dn7 = assign26730_e28255_d_n7;
        locals.var_vm_dn8 = assign26730_e28255_d_n8;
        locals.var_vm_dn9 = assign26730_e28255_d_n9;

        let (assign26740_e28265, assign26740_e28265_d_n4, assign26740_e28265_d_n6, assign26740_e28265_d_n7, assign26740_e28265_d_n8, assign26740_e28265_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26740_e28262: f64 = (locals.var_q1s_dc + locals.var_q1d_dc);
        let assign26740_e28263: f64 = (0.5 * assign26740_e28262);
        (assign26740_e28263, (0.5 * (locals.var_q1s_dc_dn4 + locals.var_q1d_dc_dn4)), (0.5 * (locals.var_q1s_dc_dn6 + locals.var_q1d_dc_dn6)), (0.5 * (locals.var_q1s_dc_dn7 + locals.var_q1d_dc_dn7)), (0.5 * (locals.var_q1s_dc_dn8 + locals.var_q1d_dc_dn8)), (0.5 * (locals.var_q1s_dc_dn9 + locals.var_q1d_dc_dn9)),)
    } else {
        (locals.var_q1m, locals.var_q1m_dn4, locals.var_q1m_dn6, locals.var_q1m_dn7, locals.var_q1m_dn8, locals.var_q1m_dn9,)
    }
};
        locals.var_q1m = assign26740_e28265;
        locals.var_q1m_dn4 = assign26740_e28265_d_n4;
        locals.var_q1m_dn6 = assign26740_e28265_d_n6;
        locals.var_q1m_dn7 = assign26740_e28265_d_n7;
        locals.var_q1m_dn8 = assign26740_e28265_d_n8;
        locals.var_q1m_dn9 = assign26740_e28265_d_n9;

        let (assign26750_e28273, assign26750_e28273_d_n4, assign26750_e28273_d_n6, assign26750_e28273_d_n7, assign26750_e28273_d_n8, assign26750_e28273_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26750_e28271: f64 = (locals.var_phit * locals.var_q1m);
        (assign26750_e28271, ((locals.var_phit_dn4 * locals.var_q1m) + (locals.var_phit * locals.var_q1m_dn4)), ((locals.var_phit_dn6 * locals.var_q1m) + (locals.var_phit * locals.var_q1m_dn6)), ((locals.var_phit_dn7 * locals.var_q1m) + (locals.var_phit * locals.var_q1m_dn7)), ((locals.var_phit_dn8 * locals.var_q1m) + (locals.var_phit * locals.var_q1m_dn8)), ((locals.var_phit_dn9 * locals.var_q1m) + (locals.var_phit * locals.var_q1m_dn9)),)
    } else {
        (locals.var_voxm, locals.var_voxm_dn4, locals.var_voxm_dn6, locals.var_voxm_dn7, locals.var_voxm_dn8, locals.var_voxm_dn9,)
    }
};
        locals.var_voxm = assign26750_e28273;
        locals.var_voxm_dn4 = assign26750_e28273_d_n4;
        locals.var_voxm_dn6 = assign26750_e28273_d_n6;
        locals.var_voxm_dn7 = assign26750_e28273_d_n7;
        locals.var_voxm_dn8 = assign26750_e28273_d_n8;
        locals.var_voxm_dn9 = assign26750_e28273_d_n9;

        let (assign26760_e28281, assign26760_e28281_d_n4, assign26760_e28281_d_n6, assign26760_e28281_d_n7, assign26760_e28281_d_n8, assign26760_e28281_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26760_e28279: f64 = (locals.var_voxm + locals.var_dch);
        (assign26760_e28279, (locals.var_voxm_dn4 + locals.var_dch_dn4), (locals.var_voxm_dn6 + locals.var_dch_dn6), (locals.var_voxm_dn7 + locals.var_dch_dn7), (locals.var_voxm_dn8 + locals.var_dch_dn8), (locals.var_voxm_dn9 + locals.var_dch_dn9),)
    } else {
        (locals.var_arg2mina, locals.var_arg2mina_dn4, locals.var_arg2mina_dn6, locals.var_arg2mina_dn7, locals.var_arg2mina_dn8, locals.var_arg2mina_dn9,)
    }
};
        locals.var_arg2mina = assign26760_e28281;
        locals.var_arg2mina_dn4 = assign26760_e28281_d_n4;
        locals.var_arg2mina_dn6 = assign26760_e28281_d_n6;
        locals.var_arg2mina_dn7 = assign26760_e28281_d_n7;
        locals.var_arg2mina_dn8 = assign26760_e28281_d_n8;
        locals.var_arg2mina_dn9 = assign26760_e28281_d_n9;

        let (assign26770_e28302, assign26770_e28302_d_n4, assign26770_e28302_d_n6, assign26770_e28302_d_n7, assign26770_e28302_d_n8, assign26770_e28302_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26770_e28288: f64 = locals.var_arg2mina;
        let assign26770_e28291: f64 = (-locals.var_arg2mina);
        let assign26770_e28294: f64 = (-locals.var_arg2mina);
        let assign26770_e28295: f64 = (assign26770_e28291 * assign26770_e28294);
        let assign26770_e28297: f64 = (assign26770_e28295 + 0.01);
        let assign26770_e28298: f64 = (assign26770_e28297).sqrt();
        let assign26770_e28299: f64 = (assign26770_e28288 - assign26770_e28298);
        let assign26770_e28300: f64 = (0.5 * assign26770_e28299);
        (assign26770_e28300, (0.5 * (locals.var_arg2mina_dn4 - ((((-locals.var_arg2mina_dn4) * assign26770_e28294) + (assign26770_e28291 * (-locals.var_arg2mina_dn4))) / (2.0 * assign26770_e28298)))), (0.5 * (locals.var_arg2mina_dn6 - ((((-locals.var_arg2mina_dn6) * assign26770_e28294) + (assign26770_e28291 * (-locals.var_arg2mina_dn6))) / (2.0 * assign26770_e28298)))), (0.5 * (locals.var_arg2mina_dn7 - ((((-locals.var_arg2mina_dn7) * assign26770_e28294) + (assign26770_e28291 * (-locals.var_arg2mina_dn7))) / (2.0 * assign26770_e28298)))), (0.5 * (locals.var_arg2mina_dn8 - ((((-locals.var_arg2mina_dn8) * assign26770_e28294) + (assign26770_e28291 * (-locals.var_arg2mina_dn8))) / (2.0 * assign26770_e28298)))), (0.5 * (locals.var_arg2mina_dn9 - ((((-locals.var_arg2mina_dn9) * assign26770_e28294) + (assign26770_e28291 * (-locals.var_arg2mina_dn9))) / (2.0 * assign26770_e28298)))),)
    } else {
        (locals.var_psi_t, locals.var_psi_t_dn4, locals.var_psi_t_dn6, locals.var_psi_t_dn7, locals.var_psi_t_dn8, locals.var_psi_t_dn9,)
    }
};
        locals.var_psi_t = assign26770_e28302;
        locals.var_psi_t_dn4 = assign26770_e28302_d_n4;
        locals.var_psi_t_dn6 = assign26770_e28302_d_n6;
        locals.var_psi_t_dn7 = assign26770_e28302_d_n7;
        locals.var_psi_t_dn8 = assign26770_e28302_d_n8;
        locals.var_psi_t_dn9 = assign26770_e28302_d_n9;

        let (assign26780_e28315, assign26780_e28315_d_n4, assign26780_e28315_d_n6, assign26780_e28315_d_n7, assign26780_e28315_d_n8, assign26780_e28315_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26780_e28308: f64 = (locals.var_voxm * locals.var_voxm);
        let assign26780_e28310: f64 = (assign26780_e28308 + 0.0001);
        let assign26780_e28311: f64 = (assign26780_e28310).sqrt();
        let assign26780_e28313: f64 = (assign26780_e28311 * locals.var_inv_chib);
        (assign26780_e28313, ((((locals.var_voxm_dn4 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn4)) / (2.0 * assign26780_e28311)) * locals.var_inv_chib), ((((locals.var_voxm_dn6 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn6)) / (2.0 * assign26780_e28311)) * locals.var_inv_chib), ((((locals.var_voxm_dn7 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn7)) / (2.0 * assign26780_e28311)) * locals.var_inv_chib), ((((locals.var_voxm_dn8 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn8)) / (2.0 * assign26780_e28311)) * locals.var_inv_chib), ((((locals.var_voxm_dn9 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn9)) / (2.0 * assign26780_e28311)) * locals.var_inv_chib),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign26780_e28315;
        locals.var_zg_dn4 = assign26780_e28315_d_n4;
        locals.var_zg_dn6 = assign26780_e28315_d_n6;
        locals.var_zg_dn7 = assign26780_e28315_d_n7;
        locals.var_zg_dn8 = assign26780_e28315_d_n8;
        locals.var_zg_dn9 = assign26780_e28315_d_n9;

        let assign26790_e28318: f64 = if locals.var_gc3ch_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard759 = assign26790_e28318;

        let (assign26800_e28341, assign26800_e28341_d_n4, assign26800_e28341_d_n6, assign26800_e28341_d_n7, assign26800_e28341_d_n8, assign26800_e28341_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard759 != 0.0)) {
        let assign26800_e28327: f64 = (locals.var_zg + locals.var_gcqch);
        let assign26800_e28330: f64 = (locals.var_zg - locals.var_gcqch);
        let assign26800_e28333: f64 = (locals.var_zg - locals.var_gcqch);
        let assign26800_e28334: f64 = (assign26800_e28330 * assign26800_e28333);
        let assign26800_e28336: f64 = (assign26800_e28334 + 1e-6);
        let assign26800_e28337: f64 = (assign26800_e28336).sqrt();
        let assign26800_e28338: f64 = (assign26800_e28327 - assign26800_e28337);
        let assign26800_e28339: f64 = (0.5 * assign26800_e28338);
        (assign26800_e28339, (0.5 * (locals.var_zg_dn4 - (((locals.var_zg_dn4 * assign26800_e28333) + (assign26800_e28330 * locals.var_zg_dn4)) / (2.0 * assign26800_e28337)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign26800_e28333) + (assign26800_e28330 * locals.var_zg_dn6)) / (2.0 * assign26800_e28337)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign26800_e28333) + (assign26800_e28330 * locals.var_zg_dn7)) / (2.0 * assign26800_e28337)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign26800_e28333) + (assign26800_e28330 * locals.var_zg_dn8)) / (2.0 * assign26800_e28337)))), (0.5 * (locals.var_zg_dn9 - (((locals.var_zg_dn9 * assign26800_e28333) + (assign26800_e28330 * locals.var_zg_dn9)) / (2.0 * assign26800_e28337)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign26800_e28341;
        locals.var_zg_dn4 = assign26800_e28341_d_n4;
        locals.var_zg_dn6 = assign26800_e28341_d_n6;
        locals.var_zg_dn7 = assign26800_e28341_d_n7;
        locals.var_zg_dn8 = assign26800_e28341_d_n8;
        locals.var_zg_dn9 = assign26800_e28341_d_n9;

        let (assign26810_e28349, assign26810_e28349_d_n4, assign26810_e28349_d_n6, assign26810_e28349_d_n7, assign26810_e28349_d_n8, assign26810_e28349_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26810_e28347: f64 = (locals.var_xg1x_dc + locals.var_eg_2phit0);
        (assign26810_e28347, (locals.var_xg1x_dc_dn4 + locals.var_eg_2phit0_dn4), (locals.var_xg1x_dc_dn6 + locals.var_eg_2phit0_dn6), (locals.var_xg1x_dc_dn7 + locals.var_eg_2phit0_dn7), (locals.var_xg1x_dc_dn8 + locals.var_eg_2phit0_dn8), (locals.var_xg1x_dc_dn9 + locals.var_eg_2phit0_dn9),)
    } else {
        (locals.var_xg1xshift, locals.var_xg1xshift_dn4, locals.var_xg1xshift_dn6, locals.var_xg1xshift_dn7, locals.var_xg1xshift_dn8, locals.var_xg1xshift_dn9,)
    }
};
        locals.var_xg1xshift = assign26810_e28349;
        locals.var_xg1xshift_dn4 = assign26810_e28349_d_n4;
        locals.var_xg1xshift_dn6 = assign26810_e28349_d_n6;
        locals.var_xg1xshift_dn7 = assign26810_e28349_d_n7;
        locals.var_xg1xshift_dn8 = assign26810_e28349_d_n8;
        locals.var_xg1xshift_dn9 = assign26810_e28349_d_n9;

        let (assign26820_e28357, assign26820_e28357_d_n4, assign26820_e28357_d_n6, assign26820_e28357_d_n7, assign26820_e28357_d_n8, assign26820_e28357_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26820_e28355: f64 = (locals.var_xg1xshift - locals.var_q1m);
        (assign26820_e28355, (locals.var_xg1xshift_dn4 - locals.var_q1m_dn4), (locals.var_xg1xshift_dn6 - locals.var_q1m_dn6), (locals.var_xg1xshift_dn7 - locals.var_q1m_dn7), (locals.var_xg1xshift_dn8 - locals.var_q1m_dn8), (locals.var_xg1xshift_dn9 - locals.var_q1m_dn9),)
    } else {
        (locals.var_x_m, locals.var_x_m_dn4, locals.var_x_m_dn6, locals.var_x_m_dn7, locals.var_x_m_dn8, locals.var_x_m_dn9,)
    }
};
        locals.var_x_m = assign26820_e28357;
        locals.var_x_m_dn4 = assign26820_e28357_d_n4;
        locals.var_x_m_dn6 = assign26820_e28357_d_n6;
        locals.var_x_m_dn7 = assign26820_e28357_d_n7;
        locals.var_x_m_dn8 = assign26820_e28357_d_n8;
        locals.var_x_m_dn9 = assign26820_e28357_d_n9;

        let (assign26830_e28373, assign26830_e28373_d_n4, assign26830_e28373_d_n6, assign26830_e28373_d_n7, assign26830_e28373_d_n8, assign26830_e28373_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26830_e28364: f64 = (locals.var_psi_t - locals.var_alpha_b);
        let assign26830_e28366: f64 = (assign26830_e28364 - locals.var_vm);
        let assign26830_e28368: f64 = (assign26830_e28366 * locals.var_inv_phit);
        let assign26830_e28369: f64 = (locals.var_x_m + assign26830_e28368);
        let assign26830_e28371: f64 = (assign26830_e28369 * locals.var_n_iginv);
        (assign26830_e28371, (((locals.var_x_m_dn4 + ((((locals.var_psi_t_dn4 - locals.var_alpha_b_dn4) - locals.var_vm_dn4) * locals.var_inv_phit) + (assign26830_e28366 * locals.var_inv_phit_dn4))) * locals.var_n_iginv) + (assign26830_e28369 * locals.var_n_iginv_dn4)), (((locals.var_x_m_dn6 + ((((locals.var_psi_t_dn6 - locals.var_alpha_b_dn6) - locals.var_vm_dn6) * locals.var_inv_phit) + (assign26830_e28366 * locals.var_inv_phit_dn6))) * locals.var_n_iginv) + (assign26830_e28369 * locals.var_n_iginv_dn6)), (((locals.var_x_m_dn7 + ((((locals.var_psi_t_dn7 - locals.var_alpha_b_dn7) - locals.var_vm_dn7) * locals.var_inv_phit) + (assign26830_e28366 * locals.var_inv_phit_dn7))) * locals.var_n_iginv) + (assign26830_e28369 * locals.var_n_iginv_dn7)), (((locals.var_x_m_dn8 + ((((locals.var_psi_t_dn8 - locals.var_alpha_b_dn8) - locals.var_vm_dn8) * locals.var_inv_phit) + (assign26830_e28366 * locals.var_inv_phit_dn8))) * locals.var_n_iginv) + (assign26830_e28369 * locals.var_n_iginv_dn8)), (((locals.var_x_m_dn9 + ((((locals.var_psi_t_dn9 - locals.var_alpha_b_dn9) - locals.var_vm_dn9) * locals.var_inv_phit) + (assign26830_e28366 * locals.var_inv_phit_dn9))) * locals.var_n_iginv) + (assign26830_e28369 * locals.var_n_iginv_dn9)),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9,)
    }
};
        locals.var_arg1 = assign26830_e28373;
        locals.var_arg1_dn4 = assign26830_e28373_d_n4;
        locals.var_arg1_dn6 = assign26830_e28373_d_n6;
        locals.var_arg1_dn7 = assign26830_e28373_d_n7;
        locals.var_arg1_dn8 = assign26830_e28373_d_n8;
        locals.var_arg1_dn9 = assign26830_e28373_d_n9;

        let assign26840_e28375: f64 = (locals.var_arg1).abs();
        let assign26840_e28377: f64 = if assign26840_e28375 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard760 = assign26840_e28377;

        let (assign26850_e28386, assign26850_e28386_d_n4, assign26850_e28386_d_n6, assign26850_e28386_d_n7, assign26850_e28386_d_n8, assign26850_e28386_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard760 != 0.0)) {
        let assign26850_e28384: f64 = (locals.var_arg1).exp();
        (assign26850_e28384, (assign26850_e28384 * locals.var_arg1_dn4), (assign26850_e28384 * locals.var_arg1_dn6), (assign26850_e28384 * locals.var_arg1_dn7), (assign26850_e28384 * locals.var_arg1_dn8), (assign26850_e28384 * locals.var_arg1_dn9),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign26850_e28386;
        locals.var_dsi_dn4 = assign26850_e28386_d_n4;
        locals.var_dsi_dn6 = assign26850_e28386_d_n6;
        locals.var_dsi_dn7 = assign26850_e28386_d_n7;
        locals.var_dsi_dn8 = assign26850_e28386_d_n8;
        locals.var_dsi_dn9 = assign26850_e28386_d_n9;

        let assign26860_e28389: f64 = (-80.0);
        let assign26860_e28390: f64 = if locals.var_arg1 < assign26860_e28389 { 1.0 } else { 0.0 };
        locals.var_guard761 = assign26860_e28390;

        let (assign26870_e28426, assign26870_e28426_d_n4, assign26870_e28426_d_n6, assign26870_e28426_d_n7, assign26870_e28426_d_n8, assign26870_e28426_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard760 == 0.0)) && (locals.var_guard761 != 0.0)) {
        let assign26870_e28402: f64 = (-locals.var_arg1);
        let assign26870_e28404: f64 = (assign26870_e28402 - 80.0);
        let assign26870_e28408: f64 = (-locals.var_arg1);
        let assign26870_e28410: f64 = (assign26870_e28408 - 80.0);
        let assign26870_e28411: f64 = (0.5 * assign26870_e28410);
        let assign26870_e28414: f64 = (-locals.var_arg1);
        let assign26870_e28416: f64 = (assign26870_e28414 - 80.0);
        let assign26870_e28418: f64 = (assign26870_e28416 * 0.3333333333333);
        let assign26870_e28419: f64 = (1.0 + assign26870_e28418);
        let assign26870_e28420: f64 = (assign26870_e28411 * assign26870_e28419);
        let assign26870_e28421: f64 = (1.0 + assign26870_e28420);
        let assign26870_e28422: f64 = (assign26870_e28404 * assign26870_e28421);
        let assign26870_e28423: f64 = (1.0 + assign26870_e28422);
        let assign26870_e28424: f64 = (1.80485e-35 / assign26870_e28423);
        (assign26870_e28424, (-((1.80485e-35 * (((-locals.var_arg1_dn4) * assign26870_e28421) + (assign26870_e28404 * (((0.5 * (-locals.var_arg1_dn4)) * assign26870_e28419) + (assign26870_e28411 * ((-locals.var_arg1_dn4) * 0.3333333333333)))))) / (assign26870_e28423 * assign26870_e28423))), (-((1.80485e-35 * (((-locals.var_arg1_dn6) * assign26870_e28421) + (assign26870_e28404 * (((0.5 * (-locals.var_arg1_dn6)) * assign26870_e28419) + (assign26870_e28411 * ((-locals.var_arg1_dn6) * 0.3333333333333)))))) / (assign26870_e28423 * assign26870_e28423))), (-((1.80485e-35 * (((-locals.var_arg1_dn7) * assign26870_e28421) + (assign26870_e28404 * (((0.5 * (-locals.var_arg1_dn7)) * assign26870_e28419) + (assign26870_e28411 * ((-locals.var_arg1_dn7) * 0.3333333333333)))))) / (assign26870_e28423 * assign26870_e28423))), (-((1.80485e-35 * (((-locals.var_arg1_dn8) * assign26870_e28421) + (assign26870_e28404 * (((0.5 * (-locals.var_arg1_dn8)) * assign26870_e28419) + (assign26870_e28411 * ((-locals.var_arg1_dn8) * 0.3333333333333)))))) / (assign26870_e28423 * assign26870_e28423))), (-((1.80485e-35 * (((-locals.var_arg1_dn9) * assign26870_e28421) + (assign26870_e28404 * (((0.5 * (-locals.var_arg1_dn9)) * assign26870_e28419) + (assign26870_e28411 * ((-locals.var_arg1_dn9) * 0.3333333333333)))))) / (assign26870_e28423 * assign26870_e28423))),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign26870_e28426;
        locals.var_dsi_dn4 = assign26870_e28426_d_n4;
        locals.var_dsi_dn6 = assign26870_e28426_d_n6;
        locals.var_dsi_dn7 = assign26870_e28426_d_n7;
        locals.var_dsi_dn8 = assign26870_e28426_d_n8;
        locals.var_dsi_dn9 = assign26870_e28426_d_n9;

        let (assign26880_e28460, assign26880_e28460_d_n4, assign26880_e28460_d_n6, assign26880_e28460_d_n7, assign26880_e28460_d_n8, assign26880_e28460_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard760 == 0.0)) && (locals.var_guard761 == 0.0)) {
        let assign26880_e28440: f64 = (locals.var_arg1 - 80.0);
        let assign26880_e28445: f64 = (locals.var_arg1 - 80.0);
        let assign26880_e28446: f64 = (0.5 * assign26880_e28445);
        let assign26880_e28450: f64 = (locals.var_arg1 - 80.0);
        let assign26880_e28452: f64 = (assign26880_e28450 * 0.3333333333333);
        let assign26880_e28453: f64 = (1.0 + assign26880_e28452);
        let assign26880_e28454: f64 = (assign26880_e28446 * assign26880_e28453);
        let assign26880_e28455: f64 = (1.0 + assign26880_e28454);
        let assign26880_e28456: f64 = (assign26880_e28440 * assign26880_e28455);
        let assign26880_e28457: f64 = (1.0 + assign26880_e28456);
        let assign26880_e28458: f64 = (5.54062e34 * assign26880_e28457);
        (assign26880_e28458, (5.54062e34 * ((locals.var_arg1_dn4 * assign26880_e28455) + (assign26880_e28440 * (((0.5 * locals.var_arg1_dn4) * assign26880_e28453) + (assign26880_e28446 * (locals.var_arg1_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn6 * assign26880_e28455) + (assign26880_e28440 * (((0.5 * locals.var_arg1_dn6) * assign26880_e28453) + (assign26880_e28446 * (locals.var_arg1_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn7 * assign26880_e28455) + (assign26880_e28440 * (((0.5 * locals.var_arg1_dn7) * assign26880_e28453) + (assign26880_e28446 * (locals.var_arg1_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn8 * assign26880_e28455) + (assign26880_e28440 * (((0.5 * locals.var_arg1_dn8) * assign26880_e28453) + (assign26880_e28446 * (locals.var_arg1_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn9 * assign26880_e28455) + (assign26880_e28440 * (((0.5 * locals.var_arg1_dn9) * assign26880_e28453) + (assign26880_e28446 * (locals.var_arg1_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign26880_e28460;
        locals.var_dsi_dn4 = assign26880_e28460_d_n4;
        locals.var_dsi_dn6 = assign26880_e28460_d_n6;
        locals.var_dsi_dn7 = assign26880_e28460_d_n7;
        locals.var_dsi_dn8 = assign26880_e28460_d_n8;
        locals.var_dsi_dn9 = assign26880_e28460_d_n9;

        let (assign26890_e28473, assign26890_e28473_d_n4, assign26890_e28473_d_n6, assign26890_e28473_d_n7, assign26890_e28473_d_n8, assign26890_e28473_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26890_e28466: f64 = (locals.var_vgs - locals.var_vm);
        let assign26890_e28467: f64 = (-assign26890_e28466);
        let assign26890_e28469: f64 = (assign26890_e28467 * locals.var_inv_phit);
        let assign26890_e28471: f64 = (assign26890_e28469 * locals.var_n_iginv);
        (assign26890_e28471, (((((-(-locals.var_vm_dn4)) * locals.var_inv_phit) + (assign26890_e28467 * locals.var_inv_phit_dn4)) * locals.var_n_iginv) + (assign26890_e28469 * locals.var_n_iginv_dn4)), (((((-(locals.var_vgs_dn6 - locals.var_vm_dn6)) * locals.var_inv_phit) + (assign26890_e28467 * locals.var_inv_phit_dn6)) * locals.var_n_iginv) + (assign26890_e28469 * locals.var_n_iginv_dn6)), (((((-(locals.var_vgs_dn7 - locals.var_vm_dn7)) * locals.var_inv_phit) + (assign26890_e28467 * locals.var_inv_phit_dn7)) * locals.var_n_iginv) + (assign26890_e28469 * locals.var_n_iginv_dn7)), (((((-(-locals.var_vm_dn8)) * locals.var_inv_phit) + (assign26890_e28467 * locals.var_inv_phit_dn8)) * locals.var_n_iginv) + (assign26890_e28469 * locals.var_n_iginv_dn8)), (((((-(locals.var_vgs_dn9 - locals.var_vm_dn9)) * locals.var_inv_phit) + (assign26890_e28467 * locals.var_inv_phit_dn9)) * locals.var_n_iginv) + (assign26890_e28469 * locals.var_n_iginv_dn9)),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9,)
    }
};
        locals.var_arg1 = assign26890_e28473;
        locals.var_arg1_dn4 = assign26890_e28473_d_n4;
        locals.var_arg1_dn6 = assign26890_e28473_d_n6;
        locals.var_arg1_dn7 = assign26890_e28473_d_n7;
        locals.var_arg1_dn8 = assign26890_e28473_d_n8;
        locals.var_arg1_dn9 = assign26890_e28473_d_n9;

        let assign26900_e28475: f64 = (locals.var_arg1).abs();
        let assign26900_e28477: f64 = if assign26900_e28475 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard762 = assign26900_e28477;

        let (assign26910_e28486, assign26910_e28486_d_n4, assign26910_e28486_d_n6, assign26910_e28486_d_n7, assign26910_e28486_d_n8, assign26910_e28486_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard762 != 0.0)) {
        let assign26910_e28484: f64 = (locals.var_arg1).exp();
        (assign26910_e28484, (assign26910_e28484 * locals.var_arg1_dn4), (assign26910_e28484 * locals.var_arg1_dn6), (assign26910_e28484 * locals.var_arg1_dn7), (assign26910_e28484 * locals.var_arg1_dn8), (assign26910_e28484 * locals.var_arg1_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26910_e28486;
        locals.var_temp_dn4 = assign26910_e28486_d_n4;
        locals.var_temp_dn6 = assign26910_e28486_d_n6;
        locals.var_temp_dn7 = assign26910_e28486_d_n7;
        locals.var_temp_dn8 = assign26910_e28486_d_n8;
        locals.var_temp_dn9 = assign26910_e28486_d_n9;

        let assign26920_e28489: f64 = (-80.0);
        let assign26920_e28490: f64 = if locals.var_arg1 < assign26920_e28489 { 1.0 } else { 0.0 };
        locals.var_guard763 = assign26920_e28490;

        let (assign26930_e28526, assign26930_e28526_d_n4, assign26930_e28526_d_n6, assign26930_e28526_d_n7, assign26930_e28526_d_n8, assign26930_e28526_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard762 == 0.0)) && (locals.var_guard763 != 0.0)) {
        let assign26930_e28502: f64 = (-locals.var_arg1);
        let assign26930_e28504: f64 = (assign26930_e28502 - 80.0);
        let assign26930_e28508: f64 = (-locals.var_arg1);
        let assign26930_e28510: f64 = (assign26930_e28508 - 80.0);
        let assign26930_e28511: f64 = (0.5 * assign26930_e28510);
        let assign26930_e28514: f64 = (-locals.var_arg1);
        let assign26930_e28516: f64 = (assign26930_e28514 - 80.0);
        let assign26930_e28518: f64 = (assign26930_e28516 * 0.3333333333333);
        let assign26930_e28519: f64 = (1.0 + assign26930_e28518);
        let assign26930_e28520: f64 = (assign26930_e28511 * assign26930_e28519);
        let assign26930_e28521: f64 = (1.0 + assign26930_e28520);
        let assign26930_e28522: f64 = (assign26930_e28504 * assign26930_e28521);
        let assign26930_e28523: f64 = (1.0 + assign26930_e28522);
        let assign26930_e28524: f64 = (1.80485e-35 / assign26930_e28523);
        (assign26930_e28524, (-((1.80485e-35 * (((-locals.var_arg1_dn4) * assign26930_e28521) + (assign26930_e28504 * (((0.5 * (-locals.var_arg1_dn4)) * assign26930_e28519) + (assign26930_e28511 * ((-locals.var_arg1_dn4) * 0.3333333333333)))))) / (assign26930_e28523 * assign26930_e28523))), (-((1.80485e-35 * (((-locals.var_arg1_dn6) * assign26930_e28521) + (assign26930_e28504 * (((0.5 * (-locals.var_arg1_dn6)) * assign26930_e28519) + (assign26930_e28511 * ((-locals.var_arg1_dn6) * 0.3333333333333)))))) / (assign26930_e28523 * assign26930_e28523))), (-((1.80485e-35 * (((-locals.var_arg1_dn7) * assign26930_e28521) + (assign26930_e28504 * (((0.5 * (-locals.var_arg1_dn7)) * assign26930_e28519) + (assign26930_e28511 * ((-locals.var_arg1_dn7) * 0.3333333333333)))))) / (assign26930_e28523 * assign26930_e28523))), (-((1.80485e-35 * (((-locals.var_arg1_dn8) * assign26930_e28521) + (assign26930_e28504 * (((0.5 * (-locals.var_arg1_dn8)) * assign26930_e28519) + (assign26930_e28511 * ((-locals.var_arg1_dn8) * 0.3333333333333)))))) / (assign26930_e28523 * assign26930_e28523))), (-((1.80485e-35 * (((-locals.var_arg1_dn9) * assign26930_e28521) + (assign26930_e28504 * (((0.5 * (-locals.var_arg1_dn9)) * assign26930_e28519) + (assign26930_e28511 * ((-locals.var_arg1_dn9) * 0.3333333333333)))))) / (assign26930_e28523 * assign26930_e28523))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26930_e28526;
        locals.var_temp_dn4 = assign26930_e28526_d_n4;
        locals.var_temp_dn6 = assign26930_e28526_d_n6;
        locals.var_temp_dn7 = assign26930_e28526_d_n7;
        locals.var_temp_dn8 = assign26930_e28526_d_n8;
        locals.var_temp_dn9 = assign26930_e28526_d_n9;

        let (assign26940_e28560, assign26940_e28560_d_n4, assign26940_e28560_d_n6, assign26940_e28560_d_n7, assign26940_e28560_d_n8, assign26940_e28560_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard762 == 0.0)) && (locals.var_guard763 == 0.0)) {
        let assign26940_e28540: f64 = (locals.var_arg1 - 80.0);
        let assign26940_e28545: f64 = (locals.var_arg1 - 80.0);
        let assign26940_e28546: f64 = (0.5 * assign26940_e28545);
        let assign26940_e28550: f64 = (locals.var_arg1 - 80.0);
        let assign26940_e28552: f64 = (assign26940_e28550 * 0.3333333333333);
        let assign26940_e28553: f64 = (1.0 + assign26940_e28552);
        let assign26940_e28554: f64 = (assign26940_e28546 * assign26940_e28553);
        let assign26940_e28555: f64 = (1.0 + assign26940_e28554);
        let assign26940_e28556: f64 = (assign26940_e28540 * assign26940_e28555);
        let assign26940_e28557: f64 = (1.0 + assign26940_e28556);
        let assign26940_e28558: f64 = (5.54062e34 * assign26940_e28557);
        (assign26940_e28558, (5.54062e34 * ((locals.var_arg1_dn4 * assign26940_e28555) + (assign26940_e28540 * (((0.5 * locals.var_arg1_dn4) * assign26940_e28553) + (assign26940_e28546 * (locals.var_arg1_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn6 * assign26940_e28555) + (assign26940_e28540 * (((0.5 * locals.var_arg1_dn6) * assign26940_e28553) + (assign26940_e28546 * (locals.var_arg1_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn7 * assign26940_e28555) + (assign26940_e28540 * (((0.5 * locals.var_arg1_dn7) * assign26940_e28553) + (assign26940_e28546 * (locals.var_arg1_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn8 * assign26940_e28555) + (assign26940_e28540 * (((0.5 * locals.var_arg1_dn8) * assign26940_e28553) + (assign26940_e28546 * (locals.var_arg1_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_arg1_dn9 * assign26940_e28555) + (assign26940_e28540 * (((0.5 * locals.var_arg1_dn9) * assign26940_e28553) + (assign26940_e28546 * (locals.var_arg1_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26940_e28560;
        locals.var_temp_dn4 = assign26940_e28560_d_n4;
        locals.var_temp_dn6 = assign26940_e28560_d_n6;
        locals.var_temp_dn7 = assign26940_e28560_d_n7;
        locals.var_temp_dn8 = assign26940_e28560_d_n8;
        locals.var_temp_dn9 = assign26940_e28560_d_n9;

        let (assign26950_e28568, assign26950_e28568_d_n4, assign26950_e28568_d_n6, assign26950_e28568_d_n7, assign26950_e28568_d_n8, assign26950_e28568_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26950_e28566: f64 = (locals.var_dsi * locals.var_temp);
        (assign26950_e28566, ((locals.var_dsi_dn4 * locals.var_temp) + (locals.var_dsi * locals.var_temp_dn4)), ((locals.var_dsi_dn6 * locals.var_temp) + (locals.var_dsi * locals.var_temp_dn6)), ((locals.var_dsi_dn7 * locals.var_temp) + (locals.var_dsi * locals.var_temp_dn7)), ((locals.var_dsi_dn8 * locals.var_temp) + (locals.var_dsi * locals.var_temp_dn8)), ((locals.var_dsi_dn9 * locals.var_temp) + (locals.var_dsi * locals.var_temp_dn9)),)
    } else {
        (locals.var_dgate, locals.var_dgate_dn4, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8, locals.var_dgate_dn9,)
    }
};
        locals.var_dgate = assign26950_e28568;
        locals.var_dgate_dn4 = assign26950_e28568_d_n4;
        locals.var_dgate_dn6 = assign26950_e28568_d_n6;
        locals.var_dgate_dn7 = assign26950_e28568_d_n7;
        locals.var_dgate_dn8 = assign26950_e28568_d_n8;
        locals.var_dgate_dn9 = assign26950_e28568_d_n9;

    }

    pub(super) fn stamp_transient_block_70(
        locals: &mut StampLocals,
    ) {
        let (assign26960_e28585, assign26960_e28585_d_n4, assign26960_e28585_d_n6, assign26960_e28585_d_n7, assign26960_e28585_d_n8, assign26960_e28585_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign26960_e28574: f64 = (-1.5);
        let assign26960_e28579: f64 = (locals.var_gc3ch_i * locals.var_zg);
        let assign26960_e28580: f64 = (locals.var_gc2ch_i + assign26960_e28579);
        let assign26960_e28581: f64 = (locals.var_zg * assign26960_e28580);
        let assign26960_e28582: f64 = (assign26960_e28574 + assign26960_e28581);
        let assign26960_e28583: f64 = (locals.var_bch * assign26960_e28582);
        (assign26960_e28583, ((locals.var_bch_dn4 * assign26960_e28582) + (locals.var_bch * ((locals.var_zg_dn4 * assign26960_e28580) + (locals.var_zg * (locals.var_gc3ch_i * locals.var_zg_dn4))))), ((locals.var_bch_dn6 * assign26960_e28582) + (locals.var_bch * ((locals.var_zg_dn6 * assign26960_e28580) + (locals.var_zg * (locals.var_gc3ch_i * locals.var_zg_dn6))))), ((locals.var_bch_dn7 * assign26960_e28582) + (locals.var_bch * ((locals.var_zg_dn7 * assign26960_e28580) + (locals.var_zg * (locals.var_gc3ch_i * locals.var_zg_dn7))))), ((locals.var_bch_dn8 * assign26960_e28582) + (locals.var_bch * ((locals.var_zg_dn8 * assign26960_e28580) + (locals.var_zg * (locals.var_gc3ch_i * locals.var_zg_dn8))))), ((locals.var_bch_dn9 * assign26960_e28582) + (locals.var_bch * ((locals.var_zg_dn9 * assign26960_e28580) + (locals.var_zg * (locals.var_gc3ch_i * locals.var_zg_dn9))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign26960_e28585;
        locals.var_temp_dn4 = assign26960_e28585_d_n4;
        locals.var_temp_dn6 = assign26960_e28585_d_n6;
        locals.var_temp_dn7 = assign26960_e28585_d_n7;
        locals.var_temp_dn8 = assign26960_e28585_d_n8;
        locals.var_temp_dn9 = assign26960_e28585_d_n9;

        let assign26970_e28588: f64 = if locals.var_temp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard764 = assign26970_e28588;

        let (assign26980_e28610, assign26980_e28610_d_n4, assign26980_e28610_d_n6, assign26980_e28610_d_n7, assign26980_e28610_d_n8, assign26980_e28610_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard764 != 0.0)) {
        let assign26980_e28599: f64 = (0.5 * locals.var_temp);
        let assign26980_e28603: f64 = (locals.var_temp * 0.3333333333333);
        let assign26980_e28604: f64 = (1.0 + assign26980_e28603);
        let assign26980_e28605: f64 = (assign26980_e28599 * assign26980_e28604);
        let assign26980_e28606: f64 = (1.0 + assign26980_e28605);
        let assign26980_e28607: f64 = (locals.var_temp * assign26980_e28606);
        let assign26980_e28608: f64 = (1.0 + assign26980_e28607);
        (assign26980_e28608, ((locals.var_temp_dn4 * assign26980_e28606) + (locals.var_temp * (((0.5 * locals.var_temp_dn4) * assign26980_e28604) + (assign26980_e28599 * (locals.var_temp_dn4 * 0.3333333333333))))), ((locals.var_temp_dn6 * assign26980_e28606) + (locals.var_temp * (((0.5 * locals.var_temp_dn6) * assign26980_e28604) + (assign26980_e28599 * (locals.var_temp_dn6 * 0.3333333333333))))), ((locals.var_temp_dn7 * assign26980_e28606) + (locals.var_temp * (((0.5 * locals.var_temp_dn7) * assign26980_e28604) + (assign26980_e28599 * (locals.var_temp_dn7 * 0.3333333333333))))), ((locals.var_temp_dn8 * assign26980_e28606) + (locals.var_temp * (((0.5 * locals.var_temp_dn8) * assign26980_e28604) + (assign26980_e28599 * (locals.var_temp_dn8 * 0.3333333333333))))), ((locals.var_temp_dn9 * assign26980_e28606) + (locals.var_temp * (((0.5 * locals.var_temp_dn9) * assign26980_e28604) + (assign26980_e28599 * (locals.var_temp_dn9 * 0.3333333333333))))),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign26980_e28610;
        locals.var_tp_dn4 = assign26980_e28610_d_n4;
        locals.var_tp_dn6 = assign26980_e28610_d_n6;
        locals.var_tp_dn7 = assign26980_e28610_d_n7;
        locals.var_tp_dn8 = assign26980_e28610_d_n8;
        locals.var_tp_dn9 = assign26980_e28610_d_n9;

        let assign26990_e28612: f64 = (locals.var_temp).abs();
        let assign26990_e28614: f64 = if assign26990_e28612 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard765 = assign26990_e28614;

        let (assign27000_e28626, assign27000_e28626_d_n4, assign27000_e28626_d_n6, assign27000_e28626_d_n7, assign27000_e28626_d_n8, assign27000_e28626_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 != 0.0)) {
        let assign27000_e28624: f64 = (locals.var_temp).exp();
        (assign27000_e28624, (assign27000_e28624 * locals.var_temp_dn4), (assign27000_e28624 * locals.var_temp_dn6), (assign27000_e28624 * locals.var_temp_dn7), (assign27000_e28624 * locals.var_temp_dn8), (assign27000_e28624 * locals.var_temp_dn9),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign27000_e28626;
        locals.var_tp_dn4 = assign27000_e28626_d_n4;
        locals.var_tp_dn6 = assign27000_e28626_d_n6;
        locals.var_tp_dn7 = assign27000_e28626_d_n7;
        locals.var_tp_dn8 = assign27000_e28626_d_n8;
        locals.var_tp_dn9 = assign27000_e28626_d_n9;

        let assign27010_e28629: f64 = (-80.0);
        let assign27010_e28630: f64 = if locals.var_temp < assign27010_e28629 { 1.0 } else { 0.0 };
        locals.var_guard766 = assign27010_e28630;

        let (assign27020_e28669, assign27020_e28669_d_n4, assign27020_e28669_d_n6, assign27020_e28669_d_n7, assign27020_e28669_d_n8, assign27020_e28669_d_n9,) = {
    if (((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 == 0.0)) && (locals.var_guard766 != 0.0)) {
        let assign27020_e28645: f64 = (-locals.var_temp);
        let assign27020_e28647: f64 = (assign27020_e28645 - 80.0);
        let assign27020_e28651: f64 = (-locals.var_temp);
        let assign27020_e28653: f64 = (assign27020_e28651 - 80.0);
        let assign27020_e28654: f64 = (0.5 * assign27020_e28653);
        let assign27020_e28657: f64 = (-locals.var_temp);
        let assign27020_e28659: f64 = (assign27020_e28657 - 80.0);
        let assign27020_e28661: f64 = (assign27020_e28659 * 0.3333333333333);
        let assign27020_e28662: f64 = (1.0 + assign27020_e28661);
        let assign27020_e28663: f64 = (assign27020_e28654 * assign27020_e28662);
        let assign27020_e28664: f64 = (1.0 + assign27020_e28663);
        let assign27020_e28665: f64 = (assign27020_e28647 * assign27020_e28664);
        let assign27020_e28666: f64 = (1.0 + assign27020_e28665);
        let assign27020_e28667: f64 = (1.80485e-35 / assign27020_e28666);
        (assign27020_e28667, (-((1.80485e-35 * (((-locals.var_temp_dn4) * assign27020_e28664) + (assign27020_e28647 * (((0.5 * (-locals.var_temp_dn4)) * assign27020_e28662) + (assign27020_e28654 * ((-locals.var_temp_dn4) * 0.3333333333333)))))) / (assign27020_e28666 * assign27020_e28666))), (-((1.80485e-35 * (((-locals.var_temp_dn6) * assign27020_e28664) + (assign27020_e28647 * (((0.5 * (-locals.var_temp_dn6)) * assign27020_e28662) + (assign27020_e28654 * ((-locals.var_temp_dn6) * 0.3333333333333)))))) / (assign27020_e28666 * assign27020_e28666))), (-((1.80485e-35 * (((-locals.var_temp_dn7) * assign27020_e28664) + (assign27020_e28647 * (((0.5 * (-locals.var_temp_dn7)) * assign27020_e28662) + (assign27020_e28654 * ((-locals.var_temp_dn7) * 0.3333333333333)))))) / (assign27020_e28666 * assign27020_e28666))), (-((1.80485e-35 * (((-locals.var_temp_dn8) * assign27020_e28664) + (assign27020_e28647 * (((0.5 * (-locals.var_temp_dn8)) * assign27020_e28662) + (assign27020_e28654 * ((-locals.var_temp_dn8) * 0.3333333333333)))))) / (assign27020_e28666 * assign27020_e28666))), (-((1.80485e-35 * (((-locals.var_temp_dn9) * assign27020_e28664) + (assign27020_e28647 * (((0.5 * (-locals.var_temp_dn9)) * assign27020_e28662) + (assign27020_e28654 * ((-locals.var_temp_dn9) * 0.3333333333333)))))) / (assign27020_e28666 * assign27020_e28666))),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign27020_e28669;
        locals.var_tp_dn4 = assign27020_e28669_d_n4;
        locals.var_tp_dn6 = assign27020_e28669_d_n6;
        locals.var_tp_dn7 = assign27020_e28669_d_n7;
        locals.var_tp_dn8 = assign27020_e28669_d_n8;
        locals.var_tp_dn9 = assign27020_e28669_d_n9;

        let (assign27030_e28706, assign27030_e28706_d_n4, assign27030_e28706_d_n6, assign27030_e28706_d_n7, assign27030_e28706_d_n8, assign27030_e28706_d_n9,) = {
    if (((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 == 0.0)) && (locals.var_guard766 == 0.0)) {
        let assign27030_e28686: f64 = (locals.var_temp - 80.0);
        let assign27030_e28691: f64 = (locals.var_temp - 80.0);
        let assign27030_e28692: f64 = (0.5 * assign27030_e28691);
        let assign27030_e28696: f64 = (locals.var_temp - 80.0);
        let assign27030_e28698: f64 = (assign27030_e28696 * 0.3333333333333);
        let assign27030_e28699: f64 = (1.0 + assign27030_e28698);
        let assign27030_e28700: f64 = (assign27030_e28692 * assign27030_e28699);
        let assign27030_e28701: f64 = (1.0 + assign27030_e28700);
        let assign27030_e28702: f64 = (assign27030_e28686 * assign27030_e28701);
        let assign27030_e28703: f64 = (1.0 + assign27030_e28702);
        let assign27030_e28704: f64 = (5.54062e34 * assign27030_e28703);
        (assign27030_e28704, (5.54062e34 * ((locals.var_temp_dn4 * assign27030_e28701) + (assign27030_e28686 * (((0.5 * locals.var_temp_dn4) * assign27030_e28699) + (assign27030_e28692 * (locals.var_temp_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn6 * assign27030_e28701) + (assign27030_e28686 * (((0.5 * locals.var_temp_dn6) * assign27030_e28699) + (assign27030_e28692 * (locals.var_temp_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn7 * assign27030_e28701) + (assign27030_e28686 * (((0.5 * locals.var_temp_dn7) * assign27030_e28699) + (assign27030_e28692 * (locals.var_temp_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn8 * assign27030_e28701) + (assign27030_e28686 * (((0.5 * locals.var_temp_dn8) * assign27030_e28699) + (assign27030_e28692 * (locals.var_temp_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn9 * assign27030_e28701) + (assign27030_e28686 * (((0.5 * locals.var_temp_dn9) * assign27030_e28699) + (assign27030_e28692 * (locals.var_temp_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign27030_e28706;
        locals.var_tp_dn4 = assign27030_e28706_d_n4;
        locals.var_tp_dn6 = assign27030_e28706_d_n6;
        locals.var_tp_dn7 = assign27030_e28706_d_n7;
        locals.var_tp_dn8 = assign27030_e28706_d_n8;
        locals.var_tp_dn9 = assign27030_e28706_d_n9;

        let (assign27040_e28723, assign27040_e28723_d_n4, assign27040_e28723_d_n6, assign27040_e28723_d_n7, assign27040_e28723_d_n8, assign27040_e28723_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign27040_e28712: f64 = (locals.var_iginv_i * locals.var_tp);
        let assign27040_e28715: f64 = (1.0 + locals.var_dsi);
        let assign27040_e28718: f64 = (1.0 + locals.var_dgate);
        let assign27040_e28719: f64 = (assign27040_e28715 / assign27040_e28718);
        let assign27040_e28720: f64 = (assign27040_e28719).ln();
        let assign27040_e28721: f64 = (assign27040_e28712 * assign27040_e28720);
        (assign27040_e28721, ((((locals.var_iginv_i_dn4 * locals.var_tp) + (locals.var_iginv_i * locals.var_tp_dn4)) * assign27040_e28720) + (assign27040_e28712 * ((((locals.var_dsi_dn4 * assign27040_e28718) - (assign27040_e28715 * locals.var_dgate_dn4)) / (assign27040_e28718 * assign27040_e28718)) / assign27040_e28719))), ((((locals.var_iginv_i_dn6 * locals.var_tp) + (locals.var_iginv_i * locals.var_tp_dn6)) * assign27040_e28720) + (assign27040_e28712 * ((((locals.var_dsi_dn6 * assign27040_e28718) - (assign27040_e28715 * locals.var_dgate_dn6)) / (assign27040_e28718 * assign27040_e28718)) / assign27040_e28719))), ((((locals.var_iginv_i_dn7 * locals.var_tp) + (locals.var_iginv_i * locals.var_tp_dn7)) * assign27040_e28720) + (assign27040_e28712 * ((((locals.var_dsi_dn7 * assign27040_e28718) - (assign27040_e28715 * locals.var_dgate_dn7)) / (assign27040_e28718 * assign27040_e28718)) / assign27040_e28719))), ((((locals.var_iginv_i_dn8 * locals.var_tp) + (locals.var_iginv_i * locals.var_tp_dn8)) * assign27040_e28720) + (assign27040_e28712 * ((((locals.var_dsi_dn8 * assign27040_e28718) - (assign27040_e28715 * locals.var_dgate_dn8)) / (assign27040_e28718 * assign27040_e28718)) / assign27040_e28719))), ((((locals.var_iginv_i_dn9 * locals.var_tp) + (locals.var_iginv_i * locals.var_tp_dn9)) * assign27040_e28720) + (assign27040_e28712 * ((((locals.var_dsi_dn9 * assign27040_e28718) - (assign27040_e28715 * locals.var_dgate_dn9)) / (assign27040_e28718 * assign27040_e28718)) / assign27040_e28719))),)
    } else {
        (locals.var_igc0, locals.var_igc0_dn4, locals.var_igc0_dn6, locals.var_igc0_dn7, locals.var_igc0_dn8, locals.var_igc0_dn9,)
    }
};
        locals.var_igc0 = assign27040_e28723;
        locals.var_igc0_dn4 = assign27040_e28723_d_n4;
        locals.var_igc0_dn6 = assign27040_e28723_d_n6;
        locals.var_igc0_dn7 = assign27040_e28723_d_n7;
        locals.var_igc0_dn8 = assign27040_e28723_d_n8;
        locals.var_igc0_dn9 = assign27040_e28723_d_n9;

        let assign27050_e28734: f64 = if ((locals.var_xg1xshift <= 0.0) || ((locals.var_gc2ch_i == 0.0) && (locals.var_gc3ch_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard767 = assign27050_e28734;

        let (assign27060_e28742, assign27060_e28742_d_n4, assign27060_e28742_d_n6, assign27060_e28742_d_n7, assign27060_e28742_d_n8, assign27060_e28742_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igc_1, locals.var_igc_1_dn4, locals.var_igc_1_dn6, locals.var_igc_1_dn7, locals.var_igc_1_dn8, locals.var_igc_1_dn9,)
    }
};
        locals.var_igc_1 = assign27060_e28742;
        locals.var_igc_1_dn4 = assign27060_e28742_d_n4;
        locals.var_igc_1_dn6 = assign27060_e28742_d_n6;
        locals.var_igc_1_dn7 = assign27060_e28742_d_n7;
        locals.var_igc_1_dn8 = assign27060_e28742_d_n8;
        locals.var_igc_1_dn9 = assign27060_e28742_d_n9;

        let (assign27070_e28750, assign27070_e28750_d_n4, assign27070_e28750_d_n6, assign27070_e28750_d_n7, assign27070_e28750_d_n8, assign27070_e28750_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 != 0.0)) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igcd_h, locals.var_igcd_h_dn4, locals.var_igcd_h_dn6, locals.var_igcd_h_dn7, locals.var_igcd_h_dn8, locals.var_igcd_h_dn9,)
    }
};
        locals.var_igcd_h = assign27070_e28750;
        locals.var_igcd_h_dn4 = assign27070_e28750_d_n4;
        locals.var_igcd_h_dn6 = assign27070_e28750_d_n6;
        locals.var_igcd_h_dn7 = assign27070_e28750_d_n7;
        locals.var_igcd_h_dn8 = assign27070_e28750_d_n8;
        locals.var_igcd_h_dn9 = assign27070_e28750_d_n9;

        let (assign27080_e28765, assign27080_e28765_d_n4, assign27080_e28765_d_n6, assign27080_e28765_d_n7, assign27080_e28765_d_n8, assign27080_e28765_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) {
        let assign27080_e28760: f64 = (2.0 * locals.var_gc3ch_i);
        let assign27080_e28762: f64 = (assign27080_e28760 * locals.var_zg);
        let assign27080_e28763: f64 = (locals.var_gc2ch_i + assign27080_e28762);
        (assign27080_e28763, (assign27080_e28760 * locals.var_zg_dn4), (assign27080_e28760 * locals.var_zg_dn6), (assign27080_e28760 * locals.var_zg_dn7), (assign27080_e28760 * locals.var_zg_dn8), (assign27080_e28760 * locals.var_zg_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign27080_e28765;
        locals.var_temp_dn4 = assign27080_e28765_d_n4;
        locals.var_temp_dn6 = assign27080_e28765_d_n6;
        locals.var_temp_dn7 = assign27080_e28765_d_n7;
        locals.var_temp_dn8 = assign27080_e28765_d_n8;
        locals.var_temp_dn9 = assign27080_e28765_d_n9;

        let (assign27090_e28780, assign27090_e28780_d_n4, assign27090_e28780_d_n6, assign27090_e28780_d_n7, assign27090_e28780_d_n8, assign27090_e28780_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) {
        let assign27090_e28775: f64 = (locals.var_temp * locals.var_bch);
        let assign27090_e28776: f64 = (locals.var_chib_i / assign27090_e28775);
        let assign27090_e28778: f64 = (assign27090_e28776 * locals.var_inv_phit);
        (assign27090_e28778, (((-((locals.var_chib_i * ((locals.var_temp_dn4 * locals.var_bch) + (locals.var_temp * locals.var_bch_dn4))) / (assign27090_e28775 * assign27090_e28775))) * locals.var_inv_phit) + (assign27090_e28776 * locals.var_inv_phit_dn4)), (((-((locals.var_chib_i * ((locals.var_temp_dn6 * locals.var_bch) + (locals.var_temp * locals.var_bch_dn6))) / (assign27090_e28775 * assign27090_e28775))) * locals.var_inv_phit) + (assign27090_e28776 * locals.var_inv_phit_dn6)), (((-((locals.var_chib_i * ((locals.var_temp_dn7 * locals.var_bch) + (locals.var_temp * locals.var_bch_dn7))) / (assign27090_e28775 * assign27090_e28775))) * locals.var_inv_phit) + (assign27090_e28776 * locals.var_inv_phit_dn7)), (((-((locals.var_chib_i * ((locals.var_temp_dn8 * locals.var_bch) + (locals.var_temp * locals.var_bch_dn8))) / (assign27090_e28775 * assign27090_e28775))) * locals.var_inv_phit) + (assign27090_e28776 * locals.var_inv_phit_dn8)), (((-((locals.var_chib_i * ((locals.var_temp_dn9 * locals.var_bch) + (locals.var_temp * locals.var_bch_dn9))) / (assign27090_e28775 * assign27090_e28775))) * locals.var_inv_phit) + (assign27090_e28776 * locals.var_inv_phit_dn9)),)
    } else {
        (locals.var_u0, locals.var_u0_dn4, locals.var_u0_dn6, locals.var_u0_dn7, locals.var_u0_dn8, locals.var_u0_dn9,)
    }
};
        locals.var_u0 = assign27090_e28780;
        locals.var_u0_dn4 = assign27090_e28780_d_n4;
        locals.var_u0_dn6 = assign27090_e28780_d_n6;
        locals.var_u0_dn7 = assign27090_e28780_d_n7;
        locals.var_u0_dn8 = assign27090_e28780_d_n8;
        locals.var_u0_dn9 = assign27090_e28780_d_n9;

        let (assign27100_e28791, assign27100_e28791_d_n4, assign27100_e28791_d_n6, assign27100_e28791_d_n7, assign27100_e28791_d_n8, assign27100_e28791_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) {
        let assign27100_e28789: f64 = (locals.var_half_x_ds / locals.var_u0);
        (assign27100_e28789, (((locals.var_half_x_ds_dn4 * locals.var_u0) - (locals.var_half_x_ds * locals.var_u0_dn4)) / (locals.var_u0 * locals.var_u0)), (((locals.var_half_x_ds_dn6 * locals.var_u0) - (locals.var_half_x_ds * locals.var_u0_dn6)) / (locals.var_u0 * locals.var_u0)), (((locals.var_half_x_ds_dn7 * locals.var_u0) - (locals.var_half_x_ds * locals.var_u0_dn7)) / (locals.var_u0 * locals.var_u0)), (((locals.var_half_x_ds_dn8 * locals.var_u0) - (locals.var_half_x_ds * locals.var_u0_dn8)) / (locals.var_u0 * locals.var_u0)), (((locals.var_half_x_ds_dn9 * locals.var_u0) - (locals.var_half_x_ds * locals.var_u0_dn9)) / (locals.var_u0 * locals.var_u0)),)
    } else {
        (locals.var_x, locals.var_x_dn4, locals.var_x_dn6, locals.var_x_dn7, locals.var_x_dn8, locals.var_x_dn9,)
    }
};
        locals.var_x = assign27100_e28791;
        locals.var_x_dn4 = assign27100_e28791_d_n4;
        locals.var_x_dn6 = assign27100_e28791_d_n6;
        locals.var_x_dn7 = assign27100_e28791_d_n7;
        locals.var_x_dn8 = assign27100_e28791_d_n8;
        locals.var_x_dn9 = assign27100_e28791_d_n9;

        let (assign27110_e28804, assign27110_e28804_d_n4, assign27110_e28804_d_n6, assign27110_e28804_d_n7, assign27110_e28804_d_n8, assign27110_e28804_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) {
        let assign27110_e28800: f64 = (locals.var_u0 * locals.var_inv_k1h1_dc);
        let assign27110_e28802: f64 = (assign27110_e28800 * locals.var_k1_dc);
        (assign27110_e28802, ((((locals.var_u0_dn4 * locals.var_inv_k1h1_dc) + (locals.var_u0 * locals.var_inv_k1h1_dc_dn4)) * locals.var_k1_dc) + (assign27110_e28800 * locals.var_k1_dc_dn4)), ((((locals.var_u0_dn6 * locals.var_inv_k1h1_dc) + (locals.var_u0 * locals.var_inv_k1h1_dc_dn6)) * locals.var_k1_dc) + (assign27110_e28800 * locals.var_k1_dc_dn6)), ((((locals.var_u0_dn7 * locals.var_inv_k1h1_dc) + (locals.var_u0 * locals.var_inv_k1h1_dc_dn7)) * locals.var_k1_dc) + (assign27110_e28800 * locals.var_k1_dc_dn7)), ((((locals.var_u0_dn8 * locals.var_inv_k1h1_dc) + (locals.var_u0 * locals.var_inv_k1h1_dc_dn8)) * locals.var_k1_dc) + (assign27110_e28800 * locals.var_k1_dc_dn8)), ((((locals.var_u0_dn9 * locals.var_inv_k1h1_dc) + (locals.var_u0 * locals.var_inv_k1h1_dc_dn9)) * locals.var_k1_dc) + (assign27110_e28800 * locals.var_k1_dc_dn9)),)
    } else {
        (locals.var_u0_div_h, locals.var_u0_div_h_dn4, locals.var_u0_div_h_dn6, locals.var_u0_div_h_dn7, locals.var_u0_div_h_dn8, locals.var_u0_div_h_dn9,)
    }
};
        locals.var_u0_div_h = assign27110_e28804;
        locals.var_u0_div_h_dn4 = assign27110_e28804_d_n4;
        locals.var_u0_div_h_dn6 = assign27110_e28804_d_n6;
        locals.var_u0_div_h_dn7 = assign27110_e28804_d_n7;
        locals.var_u0_div_h_dn8 = assign27110_e28804_d_n8;
        locals.var_u0_div_h_dn9 = assign27110_e28804_d_n9;

        let (assign27120_e28819, assign27120_e28819_d_n4, assign27120_e28819_d_n6, assign27120_e28819_d_n7, assign27120_e28819_d_n8, assign27120_e28819_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) {
        let assign27120_e28814: f64 = (1.0 - locals.var_u0_div_h);
        let assign27120_e28815: f64 = (locals.var_u0_div_h * assign27120_e28814);
        let assign27120_e28817: f64 = (assign27120_e28815 * 0.5);
        (assign27120_e28817, (((locals.var_u0_div_h_dn4 * assign27120_e28814) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn4))) * 0.5), (((locals.var_u0_div_h_dn6 * assign27120_e28814) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn6))) * 0.5), (((locals.var_u0_div_h_dn7 * assign27120_e28814) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn7))) * 0.5), (((locals.var_u0_div_h_dn8 * assign27120_e28814) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn8))) * 0.5), (((locals.var_u0_div_h_dn9 * assign27120_e28814) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn9))) * 0.5),)
    } else {
        (locals.var_bg, locals.var_bg_dn4, locals.var_bg_dn6, locals.var_bg_dn7, locals.var_bg_dn8, locals.var_bg_dn9,)
    }
};
        locals.var_bg = assign27120_e28819;
        locals.var_bg_dn4 = assign27120_e28819_d_n4;
        locals.var_bg_dn6 = assign27120_e28819_d_n6;
        locals.var_bg_dn7 = assign27120_e28819_d_n7;
        locals.var_bg_dn8 = assign27120_e28819_d_n8;
        locals.var_bg_dn9 = assign27120_e28819_d_n9;

        let (assign27130_e28832, assign27130_e28832_d_n4, assign27130_e28832_d_n6, assign27130_e28832_d_n7, assign27130_e28832_d_n8, assign27130_e28832_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) {
        let assign27130_e28829: f64 = (3.0 * locals.var_bg);
        let assign27130_e28830: f64 = (0.5 - assign27130_e28829);
        (assign27130_e28830, (-(3.0 * locals.var_bg_dn4)), (-(3.0 * locals.var_bg_dn6)), (-(3.0 * locals.var_bg_dn7)), (-(3.0 * locals.var_bg_dn8)), (-(3.0 * locals.var_bg_dn9)),)
    } else {
        (locals.var_ag, locals.var_ag_dn4, locals.var_ag_dn6, locals.var_ag_dn7, locals.var_ag_dn8, locals.var_ag_dn9,)
    }
};
        locals.var_ag = assign27130_e28832;
        locals.var_ag_dn4 = assign27130_e28832_d_n4;
        locals.var_ag_dn6 = assign27130_e28832_d_n6;
        locals.var_ag_dn7 = assign27130_e28832_d_n7;
        locals.var_ag_dn8 = assign27130_e28832_d_n8;
        locals.var_ag_dn9 = assign27130_e28832_d_n9;

        let assign27140_e28835: f64 = if locals.var_x < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard768 = assign27140_e28835;

        let (assign27150_e28848, assign27150_e28848_d_n4, assign27150_e28848_d_n6, assign27150_e28848_d_n7, assign27150_e28848_d_n8, assign27150_e28848_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 != 0.0)) {
        let assign27150_e28846: f64 = (locals.var_x * locals.var_x);
        (assign27150_e28846, ((locals.var_x_dn4 * locals.var_x) + (locals.var_x * locals.var_x_dn4)), ((locals.var_x_dn6 * locals.var_x) + (locals.var_x * locals.var_x_dn6)), ((locals.var_x_dn7 * locals.var_x) + (locals.var_x * locals.var_x_dn7)), ((locals.var_x_dn8 * locals.var_x) + (locals.var_x * locals.var_x_dn8)), ((locals.var_x_dn9 * locals.var_x) + (locals.var_x * locals.var_x_dn9)),)
    } else {
        (locals.var_xsq, locals.var_xsq_dn4, locals.var_xsq_dn6, locals.var_xsq_dn7, locals.var_xsq_dn8, locals.var_xsq_dn9,)
    }
};
        locals.var_xsq = assign27150_e28848;
        locals.var_xsq_dn4 = assign27150_e28848_d_n4;
        locals.var_xsq_dn6 = assign27150_e28848_d_n6;
        locals.var_xsq_dn7 = assign27150_e28848_d_n7;
        locals.var_xsq_dn8 = assign27150_e28848_d_n8;
        locals.var_xsq_dn9 = assign27150_e28848_d_n9;

        let (assign27160_e28877, assign27160_e28877_d_n4, assign27160_e28877_d_n6, assign27160_e28877_d_n7, assign27160_e28877_d_n8, assign27160_e28877_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 != 0.0)) {
        let assign27160_e28862: f64 = (locals.var_u0_div_h * 0.3333333333333);
        let assign27160_e28863: f64 = (0.1666666666667 + assign27160_e28862);
        let assign27160_e28866: f64 = (locals.var_xsq * 0.1666666666667);
        let assign27160_e28870: f64 = (0.2 * locals.var_u0_div_h);
        let assign27160_e28871: f64 = (0.05 + assign27160_e28870);
        let assign27160_e28872: f64 = (assign27160_e28866 * assign27160_e28871);
        let assign27160_e28873: f64 = (assign27160_e28863 + assign27160_e28872);
        let assign27160_e28874: f64 = (locals.var_xsq * assign27160_e28873);
        let assign27160_e28875: f64 = (1.0 + assign27160_e28874);
        (assign27160_e28875, ((locals.var_xsq_dn4 * assign27160_e28873) + (locals.var_xsq * ((locals.var_u0_div_h_dn4 * 0.3333333333333) + (((locals.var_xsq_dn4 * 0.1666666666667) * assign27160_e28871) + (assign27160_e28866 * (0.2 * locals.var_u0_div_h_dn4)))))), ((locals.var_xsq_dn6 * assign27160_e28873) + (locals.var_xsq * ((locals.var_u0_div_h_dn6 * 0.3333333333333) + (((locals.var_xsq_dn6 * 0.1666666666667) * assign27160_e28871) + (assign27160_e28866 * (0.2 * locals.var_u0_div_h_dn6)))))), ((locals.var_xsq_dn7 * assign27160_e28873) + (locals.var_xsq * ((locals.var_u0_div_h_dn7 * 0.3333333333333) + (((locals.var_xsq_dn7 * 0.1666666666667) * assign27160_e28871) + (assign27160_e28866 * (0.2 * locals.var_u0_div_h_dn7)))))), ((locals.var_xsq_dn8 * assign27160_e28873) + (locals.var_xsq * ((locals.var_u0_div_h_dn8 * 0.3333333333333) + (((locals.var_xsq_dn8 * 0.1666666666667) * assign27160_e28871) + (assign27160_e28866 * (0.2 * locals.var_u0_div_h_dn8)))))), ((locals.var_xsq_dn9 * assign27160_e28873) + (locals.var_xsq * ((locals.var_u0_div_h_dn9 * 0.3333333333333) + (((locals.var_xsq_dn9 * 0.1666666666667) * assign27160_e28871) + (assign27160_e28866 * (0.2 * locals.var_u0_div_h_dn9)))))),)
    } else {
        (locals.var_igc_1, locals.var_igc_1_dn4, locals.var_igc_1_dn6, locals.var_igc_1_dn7, locals.var_igc_1_dn8, locals.var_igc_1_dn9,)
    }
};
        locals.var_igc_1 = assign27160_e28877;
        locals.var_igc_1_dn4 = assign27160_e28877_d_n4;
        locals.var_igc_1_dn6 = assign27160_e28877_d_n6;
        locals.var_igc_1_dn7 = assign27160_e28877_d_n7;
        locals.var_igc_1_dn8 = assign27160_e28877_d_n8;
        locals.var_igc_1_dn9 = assign27160_e28877_d_n9;

        let (assign27170_e28912, assign27170_e28912_d_n4, assign27170_e28912_d_n6, assign27170_e28912_d_n7, assign27170_e28912_d_n8, assign27170_e28912_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 != 0.0)) {
        let assign27170_e28888: f64 = (0.5 * locals.var_igc_1);
        let assign27170_e28891: f64 = (locals.var_x * 0.1666666666667);
        let assign27170_e28897: f64 = (locals.var_bg + 0.25);
        let assign27170_e28898: f64 = (0.4 * assign27170_e28897);
        let assign27170_e28901: f64 = (0.0285714285714 * locals.var_xsq);
        let assign27170_e28904: f64 = (0.125 + locals.var_bg);
        let assign27170_e28905: f64 = (assign27170_e28901 * assign27170_e28904);
        let assign27170_e28906: f64 = (assign27170_e28898 + assign27170_e28905);
        let assign27170_e28907: f64 = (locals.var_xsq * assign27170_e28906);
        let assign27170_e28908: f64 = (1.0 + assign27170_e28907);
        let assign27170_e28909: f64 = (assign27170_e28891 * assign27170_e28908);
        let assign27170_e28910: f64 = (assign27170_e28888 - assign27170_e28909);
        (assign27170_e28910, ((0.5 * locals.var_igc_1_dn4) - (((locals.var_x_dn4 * 0.1666666666667) * assign27170_e28908) + (assign27170_e28891 * ((locals.var_xsq_dn4 * assign27170_e28906) + (locals.var_xsq * ((0.4 * locals.var_bg_dn4) + (((0.0285714285714 * locals.var_xsq_dn4) * assign27170_e28904) + (assign27170_e28901 * locals.var_bg_dn4)))))))), ((0.5 * locals.var_igc_1_dn6) - (((locals.var_x_dn6 * 0.1666666666667) * assign27170_e28908) + (assign27170_e28891 * ((locals.var_xsq_dn6 * assign27170_e28906) + (locals.var_xsq * ((0.4 * locals.var_bg_dn6) + (((0.0285714285714 * locals.var_xsq_dn6) * assign27170_e28904) + (assign27170_e28901 * locals.var_bg_dn6)))))))), ((0.5 * locals.var_igc_1_dn7) - (((locals.var_x_dn7 * 0.1666666666667) * assign27170_e28908) + (assign27170_e28891 * ((locals.var_xsq_dn7 * assign27170_e28906) + (locals.var_xsq * ((0.4 * locals.var_bg_dn7) + (((0.0285714285714 * locals.var_xsq_dn7) * assign27170_e28904) + (assign27170_e28901 * locals.var_bg_dn7)))))))), ((0.5 * locals.var_igc_1_dn8) - (((locals.var_x_dn8 * 0.1666666666667) * assign27170_e28908) + (assign27170_e28891 * ((locals.var_xsq_dn8 * assign27170_e28906) + (locals.var_xsq * ((0.4 * locals.var_bg_dn8) + (((0.0285714285714 * locals.var_xsq_dn8) * assign27170_e28904) + (assign27170_e28901 * locals.var_bg_dn8)))))))), ((0.5 * locals.var_igc_1_dn9) - (((locals.var_x_dn9 * 0.1666666666667) * assign27170_e28908) + (assign27170_e28891 * ((locals.var_xsq_dn9 * assign27170_e28906) + (locals.var_xsq * ((0.4 * locals.var_bg_dn9) + (((0.0285714285714 * locals.var_xsq_dn9) * assign27170_e28904) + (assign27170_e28901 * locals.var_bg_dn9)))))))),)
    } else {
        (locals.var_igcd_h, locals.var_igcd_h_dn4, locals.var_igcd_h_dn6, locals.var_igcd_h_dn7, locals.var_igcd_h_dn8, locals.var_igcd_h_dn9,)
    }
};
        locals.var_igcd_h = assign27170_e28912;
        locals.var_igcd_h_dn4 = assign27170_e28912_d_n4;
        locals.var_igcd_h_dn6 = assign27170_e28912_d_n6;
        locals.var_igcd_h_dn7 = assign27170_e28912_d_n7;
        locals.var_igcd_h_dn8 = assign27170_e28912_d_n8;
        locals.var_igcd_h_dn9 = assign27170_e28912_d_n9;

        let (assign27180_e28926, assign27180_e28926_d_n4, assign27180_e28926_d_n6, assign27180_e28926_d_n7, assign27180_e28926_d_n8, assign27180_e28926_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) {
        let assign27180_e28924: f64 = (1.0 / locals.var_x);
        (assign27180_e28924, (-(locals.var_x_dn4 / (locals.var_x * locals.var_x))), (-(locals.var_x_dn6 / (locals.var_x * locals.var_x))), (-(locals.var_x_dn7 / (locals.var_x * locals.var_x))), (-(locals.var_x_dn8 / (locals.var_x * locals.var_x))), (-(locals.var_x_dn9 / (locals.var_x * locals.var_x))),)
    } else {
        (locals.var_inv_x, locals.var_inv_x_dn4, locals.var_inv_x_dn6, locals.var_inv_x_dn7, locals.var_inv_x_dn8, locals.var_inv_x_dn9,)
    }
};
        locals.var_inv_x = assign27180_e28926;
        locals.var_inv_x_dn4 = assign27180_e28926_d_n4;
        locals.var_inv_x_dn6 = assign27180_e28926_d_n6;
        locals.var_inv_x_dn7 = assign27180_e28926_d_n7;
        locals.var_inv_x_dn8 = assign27180_e28926_d_n8;
        locals.var_inv_x_dn9 = assign27180_e28926_d_n9;

        let assign27190_e28928: f64 = (locals.var_x).abs();
        let assign27190_e28930: f64 = if assign27190_e28928 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard769 = assign27190_e28930;

        let (assign27200_e28945, assign27200_e28945_d_n4, assign27200_e28945_d_n6, assign27200_e28945_d_n7, assign27200_e28945_d_n8, assign27200_e28945_d_n9,) = {
    if (((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) && (locals.var_guard769 != 0.0)) {
        let assign27200_e28943: f64 = (locals.var_x).exp();
        (assign27200_e28943, (assign27200_e28943 * locals.var_x_dn4), (assign27200_e28943 * locals.var_x_dn6), (assign27200_e28943 * locals.var_x_dn7), (assign27200_e28943 * locals.var_x_dn8), (assign27200_e28943 * locals.var_x_dn9),)
    } else {
        (locals.var_ex, locals.var_ex_dn4, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, locals.var_ex_dn9,)
    }
};
        locals.var_ex = assign27200_e28945;
        locals.var_ex_dn4 = assign27200_e28945_d_n4;
        locals.var_ex_dn6 = assign27200_e28945_d_n6;
        locals.var_ex_dn7 = assign27200_e28945_d_n7;
        locals.var_ex_dn8 = assign27200_e28945_d_n8;
        locals.var_ex_dn9 = assign27200_e28945_d_n9;

        let assign27210_e28948: f64 = (-80.0);
        let assign27210_e28949: f64 = if locals.var_x < assign27210_e28948 { 1.0 } else { 0.0 };
        locals.var_guard770 = assign27210_e28949;

        let (assign27220_e28991, assign27220_e28991_d_n4, assign27220_e28991_d_n6, assign27220_e28991_d_n7, assign27220_e28991_d_n8, assign27220_e28991_d_n9,) = {
    if ((((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) && (locals.var_guard769 == 0.0)) && (locals.var_guard770 != 0.0)) {
        let assign27220_e28967: f64 = (-locals.var_x);
        let assign27220_e28969: f64 = (assign27220_e28967 - 80.0);
        let assign27220_e28973: f64 = (-locals.var_x);
        let assign27220_e28975: f64 = (assign27220_e28973 - 80.0);
        let assign27220_e28976: f64 = (0.5 * assign27220_e28975);
        let assign27220_e28979: f64 = (-locals.var_x);
        let assign27220_e28981: f64 = (assign27220_e28979 - 80.0);
        let assign27220_e28983: f64 = (assign27220_e28981 * 0.3333333333333);
        let assign27220_e28984: f64 = (1.0 + assign27220_e28983);
        let assign27220_e28985: f64 = (assign27220_e28976 * assign27220_e28984);
        let assign27220_e28986: f64 = (1.0 + assign27220_e28985);
        let assign27220_e28987: f64 = (assign27220_e28969 * assign27220_e28986);
        let assign27220_e28988: f64 = (1.0 + assign27220_e28987);
        let assign27220_e28989: f64 = (1.80485e-35 / assign27220_e28988);
        (assign27220_e28989, (-((1.80485e-35 * (((-locals.var_x_dn4) * assign27220_e28986) + (assign27220_e28969 * (((0.5 * (-locals.var_x_dn4)) * assign27220_e28984) + (assign27220_e28976 * ((-locals.var_x_dn4) * 0.3333333333333)))))) / (assign27220_e28988 * assign27220_e28988))), (-((1.80485e-35 * (((-locals.var_x_dn6) * assign27220_e28986) + (assign27220_e28969 * (((0.5 * (-locals.var_x_dn6)) * assign27220_e28984) + (assign27220_e28976 * ((-locals.var_x_dn6) * 0.3333333333333)))))) / (assign27220_e28988 * assign27220_e28988))), (-((1.80485e-35 * (((-locals.var_x_dn7) * assign27220_e28986) + (assign27220_e28969 * (((0.5 * (-locals.var_x_dn7)) * assign27220_e28984) + (assign27220_e28976 * ((-locals.var_x_dn7) * 0.3333333333333)))))) / (assign27220_e28988 * assign27220_e28988))), (-((1.80485e-35 * (((-locals.var_x_dn8) * assign27220_e28986) + (assign27220_e28969 * (((0.5 * (-locals.var_x_dn8)) * assign27220_e28984) + (assign27220_e28976 * ((-locals.var_x_dn8) * 0.3333333333333)))))) / (assign27220_e28988 * assign27220_e28988))), (-((1.80485e-35 * (((-locals.var_x_dn9) * assign27220_e28986) + (assign27220_e28969 * (((0.5 * (-locals.var_x_dn9)) * assign27220_e28984) + (assign27220_e28976 * ((-locals.var_x_dn9) * 0.3333333333333)))))) / (assign27220_e28988 * assign27220_e28988))),)
    } else {
        (locals.var_ex, locals.var_ex_dn4, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, locals.var_ex_dn9,)
    }
};
        locals.var_ex = assign27220_e28991;
        locals.var_ex_dn4 = assign27220_e28991_d_n4;
        locals.var_ex_dn6 = assign27220_e28991_d_n6;
        locals.var_ex_dn7 = assign27220_e28991_d_n7;
        locals.var_ex_dn8 = assign27220_e28991_d_n8;
        locals.var_ex_dn9 = assign27220_e28991_d_n9;

        let (assign27230_e29031, assign27230_e29031_d_n4, assign27230_e29031_d_n6, assign27230_e29031_d_n7, assign27230_e29031_d_n8, assign27230_e29031_d_n9,) = {
    if ((((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) && (locals.var_guard769 == 0.0)) && (locals.var_guard770 == 0.0)) {
        let assign27230_e29011: f64 = (locals.var_x - 80.0);
        let assign27230_e29016: f64 = (locals.var_x - 80.0);
        let assign27230_e29017: f64 = (0.5 * assign27230_e29016);
        let assign27230_e29021: f64 = (locals.var_x - 80.0);
        let assign27230_e29023: f64 = (assign27230_e29021 * 0.3333333333333);
        let assign27230_e29024: f64 = (1.0 + assign27230_e29023);
        let assign27230_e29025: f64 = (assign27230_e29017 * assign27230_e29024);
        let assign27230_e29026: f64 = (1.0 + assign27230_e29025);
        let assign27230_e29027: f64 = (assign27230_e29011 * assign27230_e29026);
        let assign27230_e29028: f64 = (1.0 + assign27230_e29027);
        let assign27230_e29029: f64 = (5.54062e34 * assign27230_e29028);
        (assign27230_e29029, (5.54062e34 * ((locals.var_x_dn4 * assign27230_e29026) + (assign27230_e29011 * (((0.5 * locals.var_x_dn4) * assign27230_e29024) + (assign27230_e29017 * (locals.var_x_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_x_dn6 * assign27230_e29026) + (assign27230_e29011 * (((0.5 * locals.var_x_dn6) * assign27230_e29024) + (assign27230_e29017 * (locals.var_x_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_x_dn7 * assign27230_e29026) + (assign27230_e29011 * (((0.5 * locals.var_x_dn7) * assign27230_e29024) + (assign27230_e29017 * (locals.var_x_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_x_dn8 * assign27230_e29026) + (assign27230_e29011 * (((0.5 * locals.var_x_dn8) * assign27230_e29024) + (assign27230_e29017 * (locals.var_x_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_x_dn9 * assign27230_e29026) + (assign27230_e29011 * (((0.5 * locals.var_x_dn9) * assign27230_e29024) + (assign27230_e29017 * (locals.var_x_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_ex, locals.var_ex_dn4, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, locals.var_ex_dn9,)
    }
};
        locals.var_ex = assign27230_e29031;
        locals.var_ex_dn4 = assign27230_e29031_d_n4;
        locals.var_ex_dn6 = assign27230_e29031_d_n6;
        locals.var_ex_dn7 = assign27230_e29031_d_n7;
        locals.var_ex_dn8 = assign27230_e29031_d_n8;
        locals.var_ex_dn9 = assign27230_e29031_d_n9;

        let (assign27240_e29045, assign27240_e29045_d_n4, assign27240_e29045_d_n6, assign27240_e29045_d_n7, assign27240_e29045_d_n8, assign27240_e29045_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) {
        let assign27240_e29043: f64 = (1.0 / locals.var_ex);
        (assign27240_e29043, (-(locals.var_ex_dn4 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn6 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn7 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn8 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn9 / (locals.var_ex * locals.var_ex))),)
    } else {
        (locals.var_inv_ex, locals.var_inv_ex_dn4, locals.var_inv_ex_dn6, locals.var_inv_ex_dn7, locals.var_inv_ex_dn8, locals.var_inv_ex_dn9,)
    }
};
        locals.var_inv_ex = assign27240_e29045;
        locals.var_inv_ex_dn4 = assign27240_e29045_d_n4;
        locals.var_inv_ex_dn6 = assign27240_e29045_d_n6;
        locals.var_inv_ex_dn7 = assign27240_e29045_d_n7;
        locals.var_inv_ex_dn8 = assign27240_e29045_d_n8;
        locals.var_inv_ex_dn9 = assign27240_e29045_d_n9;

        let (assign27250_e29059, assign27250_e29059_d_n4, assign27250_e29059_d_n6, assign27250_e29059_d_n7, assign27250_e29059_d_n8, assign27250_e29059_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) {
        let assign27250_e29057: f64 = (locals.var_ex - locals.var_inv_ex);
        (assign27250_e29057, (locals.var_ex_dn4 - locals.var_inv_ex_dn4), (locals.var_ex_dn6 - locals.var_inv_ex_dn6), (locals.var_ex_dn7 - locals.var_inv_ex_dn7), (locals.var_ex_dn8 - locals.var_inv_ex_dn8), (locals.var_ex_dn9 - locals.var_inv_ex_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign27250_e29059;
        locals.var_temp_dn4 = assign27250_e29059_d_n4;
        locals.var_temp_dn6 = assign27250_e29059_d_n6;
        locals.var_temp_dn7 = assign27250_e29059_d_n7;
        locals.var_temp_dn8 = assign27250_e29059_d_n8;
        locals.var_temp_dn9 = assign27250_e29059_d_n9;

        let (assign27260_e29073, assign27260_e29073_d_n4, assign27260_e29073_d_n6, assign27260_e29073_d_n7, assign27260_e29073_d_n8, assign27260_e29073_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) {
        let assign27260_e29071: f64 = (locals.var_ex + locals.var_inv_ex);
        (assign27260_e29071, (locals.var_ex_dn4 + locals.var_inv_ex_dn4), (locals.var_ex_dn6 + locals.var_inv_ex_dn6), (locals.var_ex_dn7 + locals.var_inv_ex_dn7), (locals.var_ex_dn8 + locals.var_inv_ex_dn8), (locals.var_ex_dn9 + locals.var_inv_ex_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27260_e29073;
        locals.var_temp2_dn4 = assign27260_e29073_d_n4;
        locals.var_temp2_dn6 = assign27260_e29073_d_n6;
        locals.var_temp2_dn7 = assign27260_e29073_d_n7;
        locals.var_temp2_dn8 = assign27260_e29073_d_n8;
        locals.var_temp2_dn9 = assign27260_e29073_d_n9;

        let (assign27270_e29097, assign27270_e29097_d_n4, assign27270_e29097_d_n6, assign27270_e29097_d_n7, assign27270_e29097_d_n8, assign27270_e29097_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) {
        let assign27270_e29086: f64 = (1.0 - locals.var_u0_div_h);
        let assign27270_e29088: f64 = (assign27270_e29086 * locals.var_temp);
        let assign27270_e29090: f64 = (assign27270_e29088 * locals.var_inv_x);
        let assign27270_e29093: f64 = (locals.var_u0_div_h * locals.var_temp2);
        let assign27270_e29094: f64 = (assign27270_e29090 + assign27270_e29093);
        let assign27270_e29095: f64 = (0.5 * assign27270_e29094);
        (assign27270_e29095, (0.5 * ((((((-locals.var_u0_div_h_dn4) * locals.var_temp) + (assign27270_e29086 * locals.var_temp_dn4)) * locals.var_inv_x) + (assign27270_e29088 * locals.var_inv_x_dn4)) + ((locals.var_u0_div_h_dn4 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn4)))), (0.5 * ((((((-locals.var_u0_div_h_dn6) * locals.var_temp) + (assign27270_e29086 * locals.var_temp_dn6)) * locals.var_inv_x) + (assign27270_e29088 * locals.var_inv_x_dn6)) + ((locals.var_u0_div_h_dn6 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn6)))), (0.5 * ((((((-locals.var_u0_div_h_dn7) * locals.var_temp) + (assign27270_e29086 * locals.var_temp_dn7)) * locals.var_inv_x) + (assign27270_e29088 * locals.var_inv_x_dn7)) + ((locals.var_u0_div_h_dn7 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn7)))), (0.5 * ((((((-locals.var_u0_div_h_dn8) * locals.var_temp) + (assign27270_e29086 * locals.var_temp_dn8)) * locals.var_inv_x) + (assign27270_e29088 * locals.var_inv_x_dn8)) + ((locals.var_u0_div_h_dn8 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn8)))), (0.5 * ((((((-locals.var_u0_div_h_dn9) * locals.var_temp) + (assign27270_e29086 * locals.var_temp_dn9)) * locals.var_inv_x) + (assign27270_e29088 * locals.var_inv_x_dn9)) + ((locals.var_u0_div_h_dn9 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn9)))),)
    } else {
        (locals.var_igc_1, locals.var_igc_1_dn4, locals.var_igc_1_dn6, locals.var_igc_1_dn7, locals.var_igc_1_dn8, locals.var_igc_1_dn9,)
    }
};
        locals.var_igc_1 = assign27270_e29097;
        locals.var_igc_1_dn4 = assign27270_e29097_d_n4;
        locals.var_igc_1_dn6 = assign27270_e29097_d_n6;
        locals.var_igc_1_dn7 = assign27270_e29097_d_n7;
        locals.var_igc_1_dn8 = assign27270_e29097_d_n8;
        locals.var_igc_1_dn9 = assign27270_e29097_d_n9;

    }

    pub(super) fn stamp_transient_block_71(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27280_e29127, assign27280_e29127_d_n4, assign27280_e29127_d_n6, assign27280_e29127_d_n7, assign27280_e29127_d_n8, assign27280_e29127_d_n9,) = {
    if ((((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) && (locals.var_guard767 == 0.0)) && (locals.var_guard768 == 0.0)) {
        let assign27280_e29113: f64 = (locals.var_ag * locals.var_inv_x);
        let assign27280_e29115: f64 = (assign27280_e29113 * locals.var_inv_x);
        let assign27280_e29116: f64 = (locals.var_bg - assign27280_e29115);
        let assign27280_e29117: f64 = (locals.var_temp * assign27280_e29116);
        let assign27280_e29118: f64 = (locals.var_igc_1 - assign27280_e29117);
        let assign27280_e29121: f64 = (locals.var_ag * locals.var_temp2);
        let assign27280_e29123: f64 = (assign27280_e29121 * locals.var_inv_x);
        let assign27280_e29124: f64 = (assign27280_e29118 - assign27280_e29123);
        let assign27280_e29125: f64 = (0.5 * assign27280_e29124);
        (assign27280_e29125, (0.5 * ((locals.var_igc_1_dn4 - ((locals.var_temp_dn4 * assign27280_e29116) + (locals.var_temp * (locals.var_bg_dn4 - ((((locals.var_ag_dn4 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn4)) * locals.var_inv_x) + (assign27280_e29113 * locals.var_inv_x_dn4)))))) - ((((locals.var_ag_dn4 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn4)) * locals.var_inv_x) + (assign27280_e29121 * locals.var_inv_x_dn4)))), (0.5 * ((locals.var_igc_1_dn6 - ((locals.var_temp_dn6 * assign27280_e29116) + (locals.var_temp * (locals.var_bg_dn6 - ((((locals.var_ag_dn6 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn6)) * locals.var_inv_x) + (assign27280_e29113 * locals.var_inv_x_dn6)))))) - ((((locals.var_ag_dn6 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn6)) * locals.var_inv_x) + (assign27280_e29121 * locals.var_inv_x_dn6)))), (0.5 * ((locals.var_igc_1_dn7 - ((locals.var_temp_dn7 * assign27280_e29116) + (locals.var_temp * (locals.var_bg_dn7 - ((((locals.var_ag_dn7 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn7)) * locals.var_inv_x) + (assign27280_e29113 * locals.var_inv_x_dn7)))))) - ((((locals.var_ag_dn7 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn7)) * locals.var_inv_x) + (assign27280_e29121 * locals.var_inv_x_dn7)))), (0.5 * ((locals.var_igc_1_dn8 - ((locals.var_temp_dn8 * assign27280_e29116) + (locals.var_temp * (locals.var_bg_dn8 - ((((locals.var_ag_dn8 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn8)) * locals.var_inv_x) + (assign27280_e29113 * locals.var_inv_x_dn8)))))) - ((((locals.var_ag_dn8 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn8)) * locals.var_inv_x) + (assign27280_e29121 * locals.var_inv_x_dn8)))), (0.5 * ((locals.var_igc_1_dn9 - ((locals.var_temp_dn9 * assign27280_e29116) + (locals.var_temp * (locals.var_bg_dn9 - ((((locals.var_ag_dn9 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn9)) * locals.var_inv_x) + (assign27280_e29113 * locals.var_inv_x_dn9)))))) - ((((locals.var_ag_dn9 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn9)) * locals.var_inv_x) + (assign27280_e29121 * locals.var_inv_x_dn9)))),)
    } else {
        (locals.var_igcd_h, locals.var_igcd_h_dn4, locals.var_igcd_h_dn6, locals.var_igcd_h_dn7, locals.var_igcd_h_dn8, locals.var_igcd_h_dn9,)
    }
};
        locals.var_igcd_h = assign27280_e29127;
        locals.var_igcd_h_dn4 = assign27280_e29127_d_n4;
        locals.var_igcd_h_dn6 = assign27280_e29127_d_n6;
        locals.var_igcd_h_dn7 = assign27280_e29127_d_n7;
        locals.var_igcd_h_dn8 = assign27280_e29127_d_n8;
        locals.var_igcd_h_dn9 = assign27280_e29127_d_n9;

        let (assign27290_e29135, assign27290_e29135_d_n4, assign27290_e29135_d_n6, assign27290_e29135_d_n7, assign27290_e29135_d_n8, assign27290_e29135_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign27290_e29133: f64 = (locals.var_igc0 * locals.var_igc_1);
        (assign27290_e29133, ((locals.var_igc0_dn4 * locals.var_igc_1) + (locals.var_igc0 * locals.var_igc_1_dn4)), ((locals.var_igc0_dn6 * locals.var_igc_1) + (locals.var_igc0 * locals.var_igc_1_dn6)), ((locals.var_igc0_dn7 * locals.var_igc_1) + (locals.var_igc0 * locals.var_igc_1_dn7)), ((locals.var_igc0_dn8 * locals.var_igc_1) + (locals.var_igc0 * locals.var_igc_1_dn8)), ((locals.var_igc0_dn9 * locals.var_igc_1) + (locals.var_igc0 * locals.var_igc_1_dn9)),)
    } else {
        (locals.var_igc, locals.var_igc_dn4, locals.var_igc_dn6, locals.var_igc_dn7, locals.var_igc_dn8, locals.var_igc_dn9,)
    }
};
        locals.var_igc = assign27290_e29135;
        locals.var_igc_dn4 = assign27290_e29135_d_n4;
        locals.var_igc_dn6 = assign27290_e29135_d_n6;
        locals.var_igc_dn7 = assign27290_e29135_d_n7;
        locals.var_igc_dn8 = assign27290_e29135_d_n8;
        locals.var_igc_dn9 = assign27290_e29135_d_n9;

        let (assign27300_e29143, assign27300_e29143_d_n4, assign27300_e29143_d_n6, assign27300_e29143_d_n7, assign27300_e29143_d_n8, assign27300_e29143_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign27300_e29141: f64 = (locals.var_igc0 * locals.var_igcd_h);
        (assign27300_e29141, ((locals.var_igc0_dn4 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn4)), ((locals.var_igc0_dn6 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn6)), ((locals.var_igc0_dn7 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn7)), ((locals.var_igc0_dn8 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn8)), ((locals.var_igc0_dn9 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn9)),)
    } else {
        (locals.var_igcd, locals.var_igcd_dn4, locals.var_igcd_dn6, locals.var_igcd_dn7, locals.var_igcd_dn8, locals.var_igcd_dn9,)
    }
};
        locals.var_igcd = assign27300_e29143;
        locals.var_igcd_dn4 = assign27300_e29143_d_n4;
        locals.var_igcd_dn6 = assign27300_e29143_d_n6;
        locals.var_igcd_dn7 = assign27300_e29143_d_n7;
        locals.var_igcd_dn8 = assign27300_e29143_d_n8;
        locals.var_igcd_dn9 = assign27300_e29143_d_n9;

        let (assign27310_e29151, assign27310_e29151_d_n4, assign27310_e29151_d_n6, assign27310_e29151_d_n7, assign27310_e29151_d_n8, assign27310_e29151_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard756 != 0.0)) {
        let assign27310_e29149: f64 = (locals.var_igc - locals.var_igcd);
        (assign27310_e29149, (locals.var_igc_dn4 - locals.var_igcd_dn4), (locals.var_igc_dn6 - locals.var_igcd_dn6), (locals.var_igc_dn7 - locals.var_igcd_dn7), (locals.var_igc_dn8 - locals.var_igcd_dn8), (locals.var_igc_dn9 - locals.var_igcd_dn9),)
    } else {
        (locals.var_igcs, locals.var_igcs_dn4, locals.var_igcs_dn6, locals.var_igcs_dn7, locals.var_igcs_dn8, locals.var_igcs_dn9,)
    }
};
        locals.var_igcs = assign27310_e29151;
        locals.var_igcs_dn4 = assign27310_e29151_d_n4;
        locals.var_igcs_dn6 = assign27310_e29151_d_n6;
        locals.var_igcs_dn7 = assign27310_e29151_d_n7;
        locals.var_igcs_dn8 = assign27310_e29151_d_n8;
        locals.var_igcs_dn9 = assign27310_e29151_d_n9;

        let assign27320_e29154: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard771 = assign27320_e29154;

        let (assign27330_e29162, assign27330_e29162_d_n4, assign27330_e29162_d_n6, assign27330_e29162_d_n7, assign27330_e29162_d_n8, assign27330_e29162_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard771 != 0.0)) {
        let assign27330_e29160: f64 = (locals.var_igcd + locals.var_igsov);
        (assign27330_e29160, (locals.var_igcd_dn4 + locals.var_igsov_dn4), (locals.var_igcd_dn6 + locals.var_igsov_dn6), (locals.var_igcd_dn7 + locals.var_igsov_dn7), (locals.var_igcd_dn8 + locals.var_igsov_dn8), (locals.var_igcd_dn9 + locals.var_igsov_dn9),)
    } else {
        (locals.var_igs, locals.var_igs_dn4, locals.var_igs_dn6, locals.var_igs_dn7, locals.var_igs_dn8, locals.var_igs_dn9,)
    }
};
        locals.var_igs = assign27330_e29162;
        locals.var_igs_dn4 = assign27330_e29162_d_n4;
        locals.var_igs_dn6 = assign27330_e29162_d_n6;
        locals.var_igs_dn7 = assign27330_e29162_d_n7;
        locals.var_igs_dn8 = assign27330_e29162_d_n8;
        locals.var_igs_dn9 = assign27330_e29162_d_n9;

        let (assign27340_e29170, assign27340_e29170_d_n4, assign27340_e29170_d_n6, assign27340_e29170_d_n7, assign27340_e29170_d_n8, assign27340_e29170_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard771 != 0.0)) {
        let assign27340_e29168: f64 = (locals.var_igcs + locals.var_igdov);
        (assign27340_e29168, (locals.var_igcs_dn4 + locals.var_igdov_dn4), (locals.var_igcs_dn6 + locals.var_igdov_dn6), (locals.var_igcs_dn7 + locals.var_igdov_dn7), (locals.var_igcs_dn8 + locals.var_igdov_dn8), (locals.var_igcs_dn9 + locals.var_igdov_dn9),)
    } else {
        (locals.var_igd, locals.var_igd_dn4, locals.var_igd_dn6, locals.var_igd_dn7, locals.var_igd_dn8, locals.var_igd_dn9,)
    }
};
        locals.var_igd = assign27340_e29170;
        locals.var_igd_dn4 = assign27340_e29170_d_n4;
        locals.var_igd_dn6 = assign27340_e29170_d_n6;
        locals.var_igd_dn7 = assign27340_e29170_d_n7;
        locals.var_igd_dn8 = assign27340_e29170_d_n8;
        locals.var_igd_dn9 = assign27340_e29170_d_n9;

        let (assign27350_e29179, assign27350_e29179_d_n4, assign27350_e29179_d_n6, assign27350_e29179_d_n7, assign27350_e29179_d_n8, assign27350_e29179_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard771 == 0.0)) {
        let assign27350_e29177: f64 = (locals.var_igcs + locals.var_igsov);
        (assign27350_e29177, (locals.var_igcs_dn4 + locals.var_igsov_dn4), (locals.var_igcs_dn6 + locals.var_igsov_dn6), (locals.var_igcs_dn7 + locals.var_igsov_dn7), (locals.var_igcs_dn8 + locals.var_igsov_dn8), (locals.var_igcs_dn9 + locals.var_igsov_dn9),)
    } else {
        (locals.var_igs, locals.var_igs_dn4, locals.var_igs_dn6, locals.var_igs_dn7, locals.var_igs_dn8, locals.var_igs_dn9,)
    }
};
        locals.var_igs = assign27350_e29179;
        locals.var_igs_dn4 = assign27350_e29179_d_n4;
        locals.var_igs_dn6 = assign27350_e29179_d_n6;
        locals.var_igs_dn7 = assign27350_e29179_d_n7;
        locals.var_igs_dn8 = assign27350_e29179_d_n8;
        locals.var_igs_dn9 = assign27350_e29179_d_n9;

        let (assign27360_e29188, assign27360_e29188_d_n4, assign27360_e29188_d_n6, assign27360_e29188_d_n7, assign27360_e29188_d_n8, assign27360_e29188_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard771 == 0.0)) {
        let assign27360_e29186: f64 = (locals.var_igcd + locals.var_igdov);
        (assign27360_e29186, (locals.var_igcd_dn4 + locals.var_igdov_dn4), (locals.var_igcd_dn6 + locals.var_igdov_dn6), (locals.var_igcd_dn7 + locals.var_igdov_dn7), (locals.var_igcd_dn8 + locals.var_igdov_dn8), (locals.var_igcd_dn9 + locals.var_igdov_dn9),)
    } else {
        (locals.var_igd, locals.var_igd_dn4, locals.var_igd_dn6, locals.var_igd_dn7, locals.var_igd_dn8, locals.var_igd_dn9,)
    }
};
        locals.var_igd = assign27360_e29188;
        locals.var_igd_dn4 = assign27360_e29188_d_n4;
        locals.var_igd_dn6 = assign27360_e29188_d_n6;
        locals.var_igd_dn7 = assign27360_e29188_d_n7;
        locals.var_igd_dn8 = assign27360_e29188_d_n8;
        locals.var_igd_dn9 = assign27360_e29188_d_n9;

        locals.var_igisl = 0.0;
        locals.var_igisl_dn4 = 0.0;
        locals.var_igisl_dn6 = 0.0;
        locals.var_igisl_dn7 = 0.0;
        locals.var_igisl_dn8 = 0.0;
        locals.var_igisl_dn9 = 0.0;

        let assign27380_e29200: f64 = if (((p.p4 > 0.0) && (locals.var_agidl_i > 0.0)) && (locals.var_vovs < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard772 = assign27380_e29200;

        let (assign27390_e29217, assign27390_e29217_d_n4, assign27390_e29217_d_n6, assign27390_e29217_d_n7, assign27390_e29217_d_n8, assign27390_e29217_d_n9,) = {
    if (locals.var_guard772 != 0.0) {
        let assign27390_e29204: f64 = (locals.var_vovs * locals.var_vovs);
        let assign27390_e29207: f64 = (locals.var_cgidl_i * locals.var_cgidl_i);
        let assign27390_e29209: f64 = (assign27390_e29207 * locals.var_vsbu);
        let assign27390_e29211: f64 = (assign27390_e29209 * locals.var_vsbu);
        let assign27390_e29212: f64 = (assign27390_e29204 + assign27390_e29211);
        let assign27390_e29214: f64 = (assign27390_e29212 + 1e-6);
        let assign27390_e29215: f64 = (assign27390_e29214).sqrt();
        (assign27390_e29215, (((locals.var_vovs_dn4 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn4)) / (2.0 * assign27390_e29215)), ((((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) + (((assign27390_e29207 * locals.var_vsbu_dn6) * locals.var_vsbu) + (assign27390_e29209 * locals.var_vsbu_dn6))) / (2.0 * assign27390_e29215)), (((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) / (2.0 * assign27390_e29215)), ((((locals.var_vovs_dn8 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn8)) + (((assign27390_e29207 * locals.var_vsbu_dn8) * locals.var_vsbu) + (assign27390_e29209 * locals.var_vsbu_dn8))) / (2.0 * assign27390_e29215)), (((locals.var_vovs_dn9 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn9)) / (2.0 * assign27390_e29215)),)
    } else {
        (locals.var_vtovs, locals.var_vtovs_dn4, locals.var_vtovs_dn6, locals.var_vtovs_dn7, locals.var_vtovs_dn8, locals.var_vtovs_dn9,)
    }
};
        locals.var_vtovs = assign27390_e29217;
        locals.var_vtovs_dn4 = assign27390_e29217_d_n4;
        locals.var_vtovs_dn6 = assign27390_e29217_d_n6;
        locals.var_vtovs_dn7 = assign27390_e29217_d_n7;
        locals.var_vtovs_dn8 = assign27390_e29217_d_n8;
        locals.var_vtovs_dn9 = assign27390_e29217_d_n9;

        let (assign27400_e29224, assign27400_e29224_d_n4, assign27400_e29224_d_n6, assign27400_e29224_d_n7, assign27400_e29224_d_n8, assign27400_e29224_d_n9,) = {
    if (locals.var_guard772 != 0.0) {
        let assign27400_e29220: f64 = (-locals.var_bgidl_i);
        let assign27400_e29222: f64 = (assign27400_e29220 / locals.var_vtovs);
        (assign27400_e29222, ((((-locals.var_bgidl_i_dn4) * locals.var_vtovs) - (assign27400_e29220 * locals.var_vtovs_dn4)) / (locals.var_vtovs * locals.var_vtovs)), ((((-locals.var_bgidl_i_dn6) * locals.var_vtovs) - (assign27400_e29220 * locals.var_vtovs_dn6)) / (locals.var_vtovs * locals.var_vtovs)), ((((-locals.var_bgidl_i_dn7) * locals.var_vtovs) - (assign27400_e29220 * locals.var_vtovs_dn7)) / (locals.var_vtovs * locals.var_vtovs)), ((((-locals.var_bgidl_i_dn8) * locals.var_vtovs) - (assign27400_e29220 * locals.var_vtovs_dn8)) / (locals.var_vtovs * locals.var_vtovs)), ((((-locals.var_bgidl_i_dn9) * locals.var_vtovs) - (assign27400_e29220 * locals.var_vtovs_dn9)) / (locals.var_vtovs * locals.var_vtovs)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign27400_e29224;
        locals.var_temp_dn4 = assign27400_e29224_d_n4;
        locals.var_temp_dn6 = assign27400_e29224_d_n6;
        locals.var_temp_dn7 = assign27400_e29224_d_n7;
        locals.var_temp_dn8 = assign27400_e29224_d_n8;
        locals.var_temp_dn9 = assign27400_e29224_d_n9;

        let assign27410_e29226: f64 = (locals.var_temp).abs();
        let assign27410_e29228: f64 = if assign27410_e29226 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard773 = assign27410_e29228;

        let (assign27420_e29235, assign27420_e29235_d_n4, assign27420_e29235_d_n6, assign27420_e29235_d_n7, assign27420_e29235_d_n8, assign27420_e29235_d_n9,) = {
    if ((locals.var_guard772 != 0.0) && (locals.var_guard773 != 0.0)) {
        let assign27420_e29233: f64 = (locals.var_temp).exp();
        (assign27420_e29233, (assign27420_e29233 * locals.var_temp_dn4), (assign27420_e29233 * locals.var_temp_dn6), (assign27420_e29233 * locals.var_temp_dn7), (assign27420_e29233 * locals.var_temp_dn8), (assign27420_e29233 * locals.var_temp_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27420_e29235;
        locals.var_temp2_dn4 = assign27420_e29235_d_n4;
        locals.var_temp2_dn6 = assign27420_e29235_d_n6;
        locals.var_temp2_dn7 = assign27420_e29235_d_n7;
        locals.var_temp2_dn8 = assign27420_e29235_d_n8;
        locals.var_temp2_dn9 = assign27420_e29235_d_n9;

        let assign27430_e29238: f64 = (-80.0);
        let assign27430_e29239: f64 = if locals.var_temp < assign27430_e29238 { 1.0 } else { 0.0 };
        locals.var_guard774 = assign27430_e29239;

        let (assign27440_e29273, assign27440_e29273_d_n4, assign27440_e29273_d_n6, assign27440_e29273_d_n7, assign27440_e29273_d_n8, assign27440_e29273_d_n9,) = {
    if (((locals.var_guard772 != 0.0) && (locals.var_guard773 == 0.0)) && (locals.var_guard774 != 0.0)) {
        let assign27440_e29249: f64 = (-locals.var_temp);
        let assign27440_e29251: f64 = (assign27440_e29249 - 80.0);
        let assign27440_e29255: f64 = (-locals.var_temp);
        let assign27440_e29257: f64 = (assign27440_e29255 - 80.0);
        let assign27440_e29258: f64 = (0.5 * assign27440_e29257);
        let assign27440_e29261: f64 = (-locals.var_temp);
        let assign27440_e29263: f64 = (assign27440_e29261 - 80.0);
        let assign27440_e29265: f64 = (assign27440_e29263 * 0.3333333333333);
        let assign27440_e29266: f64 = (1.0 + assign27440_e29265);
        let assign27440_e29267: f64 = (assign27440_e29258 * assign27440_e29266);
        let assign27440_e29268: f64 = (1.0 + assign27440_e29267);
        let assign27440_e29269: f64 = (assign27440_e29251 * assign27440_e29268);
        let assign27440_e29270: f64 = (1.0 + assign27440_e29269);
        let assign27440_e29271: f64 = (1.80485e-35 / assign27440_e29270);
        (assign27440_e29271, (-((1.80485e-35 * (((-locals.var_temp_dn4) * assign27440_e29268) + (assign27440_e29251 * (((0.5 * (-locals.var_temp_dn4)) * assign27440_e29266) + (assign27440_e29258 * ((-locals.var_temp_dn4) * 0.3333333333333)))))) / (assign27440_e29270 * assign27440_e29270))), (-((1.80485e-35 * (((-locals.var_temp_dn6) * assign27440_e29268) + (assign27440_e29251 * (((0.5 * (-locals.var_temp_dn6)) * assign27440_e29266) + (assign27440_e29258 * ((-locals.var_temp_dn6) * 0.3333333333333)))))) / (assign27440_e29270 * assign27440_e29270))), (-((1.80485e-35 * (((-locals.var_temp_dn7) * assign27440_e29268) + (assign27440_e29251 * (((0.5 * (-locals.var_temp_dn7)) * assign27440_e29266) + (assign27440_e29258 * ((-locals.var_temp_dn7) * 0.3333333333333)))))) / (assign27440_e29270 * assign27440_e29270))), (-((1.80485e-35 * (((-locals.var_temp_dn8) * assign27440_e29268) + (assign27440_e29251 * (((0.5 * (-locals.var_temp_dn8)) * assign27440_e29266) + (assign27440_e29258 * ((-locals.var_temp_dn8) * 0.3333333333333)))))) / (assign27440_e29270 * assign27440_e29270))), (-((1.80485e-35 * (((-locals.var_temp_dn9) * assign27440_e29268) + (assign27440_e29251 * (((0.5 * (-locals.var_temp_dn9)) * assign27440_e29266) + (assign27440_e29258 * ((-locals.var_temp_dn9) * 0.3333333333333)))))) / (assign27440_e29270 * assign27440_e29270))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27440_e29273;
        locals.var_temp2_dn4 = assign27440_e29273_d_n4;
        locals.var_temp2_dn6 = assign27440_e29273_d_n6;
        locals.var_temp2_dn7 = assign27440_e29273_d_n7;
        locals.var_temp2_dn8 = assign27440_e29273_d_n8;
        locals.var_temp2_dn9 = assign27440_e29273_d_n9;

        let (assign27450_e29305, assign27450_e29305_d_n4, assign27450_e29305_d_n6, assign27450_e29305_d_n7, assign27450_e29305_d_n8, assign27450_e29305_d_n9,) = {
    if (((locals.var_guard772 != 0.0) && (locals.var_guard773 == 0.0)) && (locals.var_guard774 == 0.0)) {
        let assign27450_e29285: f64 = (locals.var_temp - 80.0);
        let assign27450_e29290: f64 = (locals.var_temp - 80.0);
        let assign27450_e29291: f64 = (0.5 * assign27450_e29290);
        let assign27450_e29295: f64 = (locals.var_temp - 80.0);
        let assign27450_e29297: f64 = (assign27450_e29295 * 0.3333333333333);
        let assign27450_e29298: f64 = (1.0 + assign27450_e29297);
        let assign27450_e29299: f64 = (assign27450_e29291 * assign27450_e29298);
        let assign27450_e29300: f64 = (1.0 + assign27450_e29299);
        let assign27450_e29301: f64 = (assign27450_e29285 * assign27450_e29300);
        let assign27450_e29302: f64 = (1.0 + assign27450_e29301);
        let assign27450_e29303: f64 = (5.54062e34 * assign27450_e29302);
        (assign27450_e29303, (5.54062e34 * ((locals.var_temp_dn4 * assign27450_e29300) + (assign27450_e29285 * (((0.5 * locals.var_temp_dn4) * assign27450_e29298) + (assign27450_e29291 * (locals.var_temp_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn6 * assign27450_e29300) + (assign27450_e29285 * (((0.5 * locals.var_temp_dn6) * assign27450_e29298) + (assign27450_e29291 * (locals.var_temp_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn7 * assign27450_e29300) + (assign27450_e29285 * (((0.5 * locals.var_temp_dn7) * assign27450_e29298) + (assign27450_e29291 * (locals.var_temp_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn8 * assign27450_e29300) + (assign27450_e29285 * (((0.5 * locals.var_temp_dn8) * assign27450_e29298) + (assign27450_e29291 * (locals.var_temp_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn9 * assign27450_e29300) + (assign27450_e29285 * (((0.5 * locals.var_temp_dn9) * assign27450_e29298) + (assign27450_e29291 * (locals.var_temp_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27450_e29305;
        locals.var_temp2_dn4 = assign27450_e29305_d_n4;
        locals.var_temp2_dn6 = assign27450_e29305_d_n6;
        locals.var_temp2_dn7 = assign27450_e29305_d_n7;
        locals.var_temp2_dn8 = assign27450_e29305_d_n8;
        locals.var_temp2_dn9 = assign27450_e29305_d_n9;

        let (assign27460_e29311, assign27460_e29311_d_n4, assign27460_e29311_d_n6, assign27460_e29311_d_n7, assign27460_e29311_d_n8, assign27460_e29311_d_n9,) = {
    if (locals.var_guard772 != 0.0) {
        let assign27460_e29309: f64 = (locals.var_dgidl_i * locals.var_vsdu);
        (assign27460_e29309, 0.0, (locals.var_dgidl_i * locals.var_vsdu_dn6), (locals.var_dgidl_i * locals.var_vsdu_dn7), 0.0, 0.0,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign27460_e29311;
        locals.var_temp3_dn4 = assign27460_e29311_d_n4;
        locals.var_temp3_dn6 = assign27460_e29311_d_n6;
        locals.var_temp3_dn7 = assign27460_e29311_d_n7;
        locals.var_temp3_dn8 = assign27460_e29311_d_n8;
        locals.var_temp3_dn9 = assign27460_e29311_d_n9;

        let assign27470_e29313: f64 = (locals.var_temp3).abs();
        let assign27470_e29315: f64 = if assign27470_e29313 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard775 = assign27470_e29315;

        let (assign27480_e29322, assign27480_e29322_d_n4, assign27480_e29322_d_n6, assign27480_e29322_d_n7, assign27480_e29322_d_n8, assign27480_e29322_d_n9,) = {
    if ((locals.var_guard772 != 0.0) && (locals.var_guard775 != 0.0)) {
        let assign27480_e29320: f64 = (locals.var_temp3).exp();
        (assign27480_e29320, (assign27480_e29320 * locals.var_temp3_dn4), (assign27480_e29320 * locals.var_temp3_dn6), (assign27480_e29320 * locals.var_temp3_dn7), (assign27480_e29320 * locals.var_temp3_dn8), (assign27480_e29320 * locals.var_temp3_dn9),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign27480_e29322;
        locals.var_temp4_dn4 = assign27480_e29322_d_n4;
        locals.var_temp4_dn6 = assign27480_e29322_d_n6;
        locals.var_temp4_dn7 = assign27480_e29322_d_n7;
        locals.var_temp4_dn8 = assign27480_e29322_d_n8;
        locals.var_temp4_dn9 = assign27480_e29322_d_n9;

        let assign27490_e29325: f64 = (-80.0);
        let assign27490_e29326: f64 = if locals.var_temp3 < assign27490_e29325 { 1.0 } else { 0.0 };
        locals.var_guard776 = assign27490_e29326;

        let (assign27500_e29360, assign27500_e29360_d_n4, assign27500_e29360_d_n6, assign27500_e29360_d_n7, assign27500_e29360_d_n8, assign27500_e29360_d_n9,) = {
    if (((locals.var_guard772 != 0.0) && (locals.var_guard775 == 0.0)) && (locals.var_guard776 != 0.0)) {
        let assign27500_e29336: f64 = (-locals.var_temp3);
        let assign27500_e29338: f64 = (assign27500_e29336 - 80.0);
        let assign27500_e29342: f64 = (-locals.var_temp3);
        let assign27500_e29344: f64 = (assign27500_e29342 - 80.0);
        let assign27500_e29345: f64 = (0.5 * assign27500_e29344);
        let assign27500_e29348: f64 = (-locals.var_temp3);
        let assign27500_e29350: f64 = (assign27500_e29348 - 80.0);
        let assign27500_e29352: f64 = (assign27500_e29350 * 0.3333333333333);
        let assign27500_e29353: f64 = (1.0 + assign27500_e29352);
        let assign27500_e29354: f64 = (assign27500_e29345 * assign27500_e29353);
        let assign27500_e29355: f64 = (1.0 + assign27500_e29354);
        let assign27500_e29356: f64 = (assign27500_e29338 * assign27500_e29355);
        let assign27500_e29357: f64 = (1.0 + assign27500_e29356);
        let assign27500_e29358: f64 = (1.80485e-35 / assign27500_e29357);
        (assign27500_e29358, (-((1.80485e-35 * (((-locals.var_temp3_dn4) * assign27500_e29355) + (assign27500_e29338 * (((0.5 * (-locals.var_temp3_dn4)) * assign27500_e29353) + (assign27500_e29345 * ((-locals.var_temp3_dn4) * 0.3333333333333)))))) / (assign27500_e29357 * assign27500_e29357))), (-((1.80485e-35 * (((-locals.var_temp3_dn6) * assign27500_e29355) + (assign27500_e29338 * (((0.5 * (-locals.var_temp3_dn6)) * assign27500_e29353) + (assign27500_e29345 * ((-locals.var_temp3_dn6) * 0.3333333333333)))))) / (assign27500_e29357 * assign27500_e29357))), (-((1.80485e-35 * (((-locals.var_temp3_dn7) * assign27500_e29355) + (assign27500_e29338 * (((0.5 * (-locals.var_temp3_dn7)) * assign27500_e29353) + (assign27500_e29345 * ((-locals.var_temp3_dn7) * 0.3333333333333)))))) / (assign27500_e29357 * assign27500_e29357))), (-((1.80485e-35 * (((-locals.var_temp3_dn8) * assign27500_e29355) + (assign27500_e29338 * (((0.5 * (-locals.var_temp3_dn8)) * assign27500_e29353) + (assign27500_e29345 * ((-locals.var_temp3_dn8) * 0.3333333333333)))))) / (assign27500_e29357 * assign27500_e29357))), (-((1.80485e-35 * (((-locals.var_temp3_dn9) * assign27500_e29355) + (assign27500_e29338 * (((0.5 * (-locals.var_temp3_dn9)) * assign27500_e29353) + (assign27500_e29345 * ((-locals.var_temp3_dn9) * 0.3333333333333)))))) / (assign27500_e29357 * assign27500_e29357))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign27500_e29360;
        locals.var_temp4_dn4 = assign27500_e29360_d_n4;
        locals.var_temp4_dn6 = assign27500_e29360_d_n6;
        locals.var_temp4_dn7 = assign27500_e29360_d_n7;
        locals.var_temp4_dn8 = assign27500_e29360_d_n8;
        locals.var_temp4_dn9 = assign27500_e29360_d_n9;

        let (assign27510_e29392, assign27510_e29392_d_n4, assign27510_e29392_d_n6, assign27510_e29392_d_n7, assign27510_e29392_d_n8, assign27510_e29392_d_n9,) = {
    if (((locals.var_guard772 != 0.0) && (locals.var_guard775 == 0.0)) && (locals.var_guard776 == 0.0)) {
        let assign27510_e29372: f64 = (locals.var_temp3 - 80.0);
        let assign27510_e29377: f64 = (locals.var_temp3 - 80.0);
        let assign27510_e29378: f64 = (0.5 * assign27510_e29377);
        let assign27510_e29382: f64 = (locals.var_temp3 - 80.0);
        let assign27510_e29384: f64 = (assign27510_e29382 * 0.3333333333333);
        let assign27510_e29385: f64 = (1.0 + assign27510_e29384);
        let assign27510_e29386: f64 = (assign27510_e29378 * assign27510_e29385);
        let assign27510_e29387: f64 = (1.0 + assign27510_e29386);
        let assign27510_e29388: f64 = (assign27510_e29372 * assign27510_e29387);
        let assign27510_e29389: f64 = (1.0 + assign27510_e29388);
        let assign27510_e29390: f64 = (5.54062e34 * assign27510_e29389);
        (assign27510_e29390, (5.54062e34 * ((locals.var_temp3_dn4 * assign27510_e29387) + (assign27510_e29372 * (((0.5 * locals.var_temp3_dn4) * assign27510_e29385) + (assign27510_e29378 * (locals.var_temp3_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn6 * assign27510_e29387) + (assign27510_e29372 * (((0.5 * locals.var_temp3_dn6) * assign27510_e29385) + (assign27510_e29378 * (locals.var_temp3_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn7 * assign27510_e29387) + (assign27510_e29372 * (((0.5 * locals.var_temp3_dn7) * assign27510_e29385) + (assign27510_e29378 * (locals.var_temp3_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn8 * assign27510_e29387) + (assign27510_e29372 * (((0.5 * locals.var_temp3_dn8) * assign27510_e29385) + (assign27510_e29378 * (locals.var_temp3_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn9 * assign27510_e29387) + (assign27510_e29372 * (((0.5 * locals.var_temp3_dn9) * assign27510_e29385) + (assign27510_e29378 * (locals.var_temp3_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign27510_e29392;
        locals.var_temp4_dn4 = assign27510_e29392_d_n4;
        locals.var_temp4_dn6 = assign27510_e29392_d_n6;
        locals.var_temp4_dn7 = assign27510_e29392_d_n7;
        locals.var_temp4_dn8 = assign27510_e29392_d_n8;
        locals.var_temp4_dn9 = assign27510_e29392_d_n9;

        let (assign27520_e29411, assign27520_e29411_d_n4, assign27520_e29411_d_n6, assign27520_e29411_d_n7, assign27520_e29411_d_n8, assign27520_e29411_d_n9,) = {
    if (locals.var_guard772 != 0.0) {
        let assign27520_e29395: f64 = (-locals.var_agidl_i);
        let assign27520_e29397: f64 = (assign27520_e29395 * locals.var_vsdu);
        let assign27520_e29399: f64 = (assign27520_e29397 * locals.var_vovs);
        let assign27520_e29401: f64 = (assign27520_e29399 * locals.var_vtovs);
        let assign27520_e29403: f64 = (assign27520_e29401 * locals.var_temp2);
        let assign27520_e29405: f64 = (assign27520_e29403 * 0.5);
        let assign27520_e29408: f64 = (1.0 + locals.var_temp4);
        let assign27520_e29409: f64 = (assign27520_e29405 * assign27520_e29408);
        (assign27520_e29409, (((((((((((-locals.var_agidl_i_dn4) * locals.var_vsdu) * locals.var_vovs) + (assign27520_e29397 * locals.var_vovs_dn4)) * locals.var_vtovs) + (assign27520_e29399 * locals.var_vtovs_dn4)) * locals.var_temp2) + (assign27520_e29401 * locals.var_temp2_dn4)) * 0.5) * assign27520_e29408) + (assign27520_e29405 * locals.var_temp4_dn4)), ((((((((((((-locals.var_agidl_i_dn6) * locals.var_vsdu) + (assign27520_e29395 * locals.var_vsdu_dn6)) * locals.var_vovs) + (assign27520_e29397 * locals.var_vovs_dn6)) * locals.var_vtovs) + (assign27520_e29399 * locals.var_vtovs_dn6)) * locals.var_temp2) + (assign27520_e29401 * locals.var_temp2_dn6)) * 0.5) * assign27520_e29408) + (assign27520_e29405 * locals.var_temp4_dn6)), ((((((((((((-locals.var_agidl_i_dn7) * locals.var_vsdu) + (assign27520_e29395 * locals.var_vsdu_dn7)) * locals.var_vovs) + (assign27520_e29397 * locals.var_vovs_dn7)) * locals.var_vtovs) + (assign27520_e29399 * locals.var_vtovs_dn7)) * locals.var_temp2) + (assign27520_e29401 * locals.var_temp2_dn7)) * 0.5) * assign27520_e29408) + (assign27520_e29405 * locals.var_temp4_dn7)), (((((((((((-locals.var_agidl_i_dn8) * locals.var_vsdu) * locals.var_vovs) + (assign27520_e29397 * locals.var_vovs_dn8)) * locals.var_vtovs) + (assign27520_e29399 * locals.var_vtovs_dn8)) * locals.var_temp2) + (assign27520_e29401 * locals.var_temp2_dn8)) * 0.5) * assign27520_e29408) + (assign27520_e29405 * locals.var_temp4_dn8)), (((((((((((-locals.var_agidl_i_dn9) * locals.var_vsdu) * locals.var_vovs) + (assign27520_e29397 * locals.var_vovs_dn9)) * locals.var_vtovs) + (assign27520_e29399 * locals.var_vtovs_dn9)) * locals.var_temp2) + (assign27520_e29401 * locals.var_temp2_dn9)) * 0.5) * assign27520_e29408) + (assign27520_e29405 * locals.var_temp4_dn9)),)
    } else {
        (locals.var_igisl, locals.var_igisl_dn4, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn8, locals.var_igisl_dn9,)
    }
};
        locals.var_igisl = assign27520_e29411;
        locals.var_igisl_dn4 = assign27520_e29411_d_n4;
        locals.var_igisl_dn6 = assign27520_e29411_d_n6;
        locals.var_igisl_dn7 = assign27520_e29411_d_n7;
        locals.var_igisl_dn8 = assign27520_e29411_d_n8;
        locals.var_igisl_dn9 = assign27520_e29411_d_n9;

        locals.var_igidl = 0.0;
        locals.var_igidl_dn4 = 0.0;
        locals.var_igidl_dn6 = 0.0;
        locals.var_igidl_dn7 = 0.0;
        locals.var_igidl_dn8 = 0.0;
        locals.var_igidl_dn9 = 0.0;

        let assign27540_e29423: f64 = if (((p.p4 > 0.0) && (locals.var_agidld_i > 0.0)) && (locals.var_vovd < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard777 = assign27540_e29423;

        let (assign27550_e29440, assign27550_e29440_d_n4, assign27550_e29440_d_n6, assign27550_e29440_d_n7, assign27550_e29440_d_n8, assign27550_e29440_d_n9,) = {
    if (locals.var_guard777 != 0.0) {
        let assign27550_e29427: f64 = (locals.var_vovd * locals.var_vovd);
        let assign27550_e29430: f64 = (locals.var_cgidld_i * locals.var_cgidld_i);
        let assign27550_e29432: f64 = (assign27550_e29430 * locals.var_vdbu);
        let assign27550_e29434: f64 = (assign27550_e29432 * locals.var_vdbu);
        let assign27550_e29435: f64 = (assign27550_e29427 + assign27550_e29434);
        let assign27550_e29437: f64 = (assign27550_e29435 + 1e-6);
        let assign27550_e29438: f64 = (assign27550_e29437).sqrt();
        (assign27550_e29438, (((locals.var_vovd_dn4 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn4)) / (2.0 * assign27550_e29438)), ((((locals.var_vovd_dn6 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn6)) + (((assign27550_e29430 * locals.var_vdbu_dn6) * locals.var_vdbu) + (assign27550_e29432 * locals.var_vdbu_dn6))) / (2.0 * assign27550_e29438)), ((((locals.var_vovd_dn7 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn7)) + (((assign27550_e29430 * locals.var_vdbu_dn7) * locals.var_vdbu) + (assign27550_e29432 * locals.var_vdbu_dn7))) / (2.0 * assign27550_e29438)), ((((locals.var_vovd_dn8 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn8)) + (((assign27550_e29430 * locals.var_vdbu_dn8) * locals.var_vdbu) + (assign27550_e29432 * locals.var_vdbu_dn8))) / (2.0 * assign27550_e29438)), (((locals.var_vovd_dn9 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn9)) / (2.0 * assign27550_e29438)),)
    } else {
        (locals.var_vtovd, locals.var_vtovd_dn4, locals.var_vtovd_dn6, locals.var_vtovd_dn7, locals.var_vtovd_dn8, locals.var_vtovd_dn9,)
    }
};
        locals.var_vtovd = assign27550_e29440;
        locals.var_vtovd_dn4 = assign27550_e29440_d_n4;
        locals.var_vtovd_dn6 = assign27550_e29440_d_n6;
        locals.var_vtovd_dn7 = assign27550_e29440_d_n7;
        locals.var_vtovd_dn8 = assign27550_e29440_d_n8;
        locals.var_vtovd_dn9 = assign27550_e29440_d_n9;

        let (assign27560_e29447, assign27560_e29447_d_n4, assign27560_e29447_d_n6, assign27560_e29447_d_n7, assign27560_e29447_d_n8, assign27560_e29447_d_n9,) = {
    if (locals.var_guard777 != 0.0) {
        let assign27560_e29443: f64 = (-locals.var_bgidld_i);
        let assign27560_e29445: f64 = (assign27560_e29443 / locals.var_vtovd);
        (assign27560_e29445, ((((-locals.var_bgidld_i_dn4) * locals.var_vtovd) - (assign27560_e29443 * locals.var_vtovd_dn4)) / (locals.var_vtovd * locals.var_vtovd)), ((((-locals.var_bgidld_i_dn6) * locals.var_vtovd) - (assign27560_e29443 * locals.var_vtovd_dn6)) / (locals.var_vtovd * locals.var_vtovd)), ((((-locals.var_bgidld_i_dn7) * locals.var_vtovd) - (assign27560_e29443 * locals.var_vtovd_dn7)) / (locals.var_vtovd * locals.var_vtovd)), ((((-locals.var_bgidld_i_dn8) * locals.var_vtovd) - (assign27560_e29443 * locals.var_vtovd_dn8)) / (locals.var_vtovd * locals.var_vtovd)), ((((-locals.var_bgidld_i_dn9) * locals.var_vtovd) - (assign27560_e29443 * locals.var_vtovd_dn9)) / (locals.var_vtovd * locals.var_vtovd)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign27560_e29447;
        locals.var_temp_dn4 = assign27560_e29447_d_n4;
        locals.var_temp_dn6 = assign27560_e29447_d_n6;
        locals.var_temp_dn7 = assign27560_e29447_d_n7;
        locals.var_temp_dn8 = assign27560_e29447_d_n8;
        locals.var_temp_dn9 = assign27560_e29447_d_n9;

        let assign27570_e29449: f64 = (locals.var_temp).abs();
        let assign27570_e29451: f64 = if assign27570_e29449 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard778 = assign27570_e29451;

        let (assign27580_e29458, assign27580_e29458_d_n4, assign27580_e29458_d_n6, assign27580_e29458_d_n7, assign27580_e29458_d_n8, assign27580_e29458_d_n9,) = {
    if ((locals.var_guard777 != 0.0) && (locals.var_guard778 != 0.0)) {
        let assign27580_e29456: f64 = (locals.var_temp).exp();
        (assign27580_e29456, (assign27580_e29456 * locals.var_temp_dn4), (assign27580_e29456 * locals.var_temp_dn6), (assign27580_e29456 * locals.var_temp_dn7), (assign27580_e29456 * locals.var_temp_dn8), (assign27580_e29456 * locals.var_temp_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27580_e29458;
        locals.var_temp2_dn4 = assign27580_e29458_d_n4;
        locals.var_temp2_dn6 = assign27580_e29458_d_n6;
        locals.var_temp2_dn7 = assign27580_e29458_d_n7;
        locals.var_temp2_dn8 = assign27580_e29458_d_n8;
        locals.var_temp2_dn9 = assign27580_e29458_d_n9;

        let assign27590_e29461: f64 = (-80.0);
        let assign27590_e29462: f64 = if locals.var_temp < assign27590_e29461 { 1.0 } else { 0.0 };
        locals.var_guard779 = assign27590_e29462;

        let (assign27600_e29496, assign27600_e29496_d_n4, assign27600_e29496_d_n6, assign27600_e29496_d_n7, assign27600_e29496_d_n8, assign27600_e29496_d_n9,) = {
    if (((locals.var_guard777 != 0.0) && (locals.var_guard778 == 0.0)) && (locals.var_guard779 != 0.0)) {
        let assign27600_e29472: f64 = (-locals.var_temp);
        let assign27600_e29474: f64 = (assign27600_e29472 - 80.0);
        let assign27600_e29478: f64 = (-locals.var_temp);
        let assign27600_e29480: f64 = (assign27600_e29478 - 80.0);
        let assign27600_e29481: f64 = (0.5 * assign27600_e29480);
        let assign27600_e29484: f64 = (-locals.var_temp);
        let assign27600_e29486: f64 = (assign27600_e29484 - 80.0);
        let assign27600_e29488: f64 = (assign27600_e29486 * 0.3333333333333);
        let assign27600_e29489: f64 = (1.0 + assign27600_e29488);
        let assign27600_e29490: f64 = (assign27600_e29481 * assign27600_e29489);
        let assign27600_e29491: f64 = (1.0 + assign27600_e29490);
        let assign27600_e29492: f64 = (assign27600_e29474 * assign27600_e29491);
        let assign27600_e29493: f64 = (1.0 + assign27600_e29492);
        let assign27600_e29494: f64 = (1.80485e-35 / assign27600_e29493);
        (assign27600_e29494, (-((1.80485e-35 * (((-locals.var_temp_dn4) * assign27600_e29491) + (assign27600_e29474 * (((0.5 * (-locals.var_temp_dn4)) * assign27600_e29489) + (assign27600_e29481 * ((-locals.var_temp_dn4) * 0.3333333333333)))))) / (assign27600_e29493 * assign27600_e29493))), (-((1.80485e-35 * (((-locals.var_temp_dn6) * assign27600_e29491) + (assign27600_e29474 * (((0.5 * (-locals.var_temp_dn6)) * assign27600_e29489) + (assign27600_e29481 * ((-locals.var_temp_dn6) * 0.3333333333333)))))) / (assign27600_e29493 * assign27600_e29493))), (-((1.80485e-35 * (((-locals.var_temp_dn7) * assign27600_e29491) + (assign27600_e29474 * (((0.5 * (-locals.var_temp_dn7)) * assign27600_e29489) + (assign27600_e29481 * ((-locals.var_temp_dn7) * 0.3333333333333)))))) / (assign27600_e29493 * assign27600_e29493))), (-((1.80485e-35 * (((-locals.var_temp_dn8) * assign27600_e29491) + (assign27600_e29474 * (((0.5 * (-locals.var_temp_dn8)) * assign27600_e29489) + (assign27600_e29481 * ((-locals.var_temp_dn8) * 0.3333333333333)))))) / (assign27600_e29493 * assign27600_e29493))), (-((1.80485e-35 * (((-locals.var_temp_dn9) * assign27600_e29491) + (assign27600_e29474 * (((0.5 * (-locals.var_temp_dn9)) * assign27600_e29489) + (assign27600_e29481 * ((-locals.var_temp_dn9) * 0.3333333333333)))))) / (assign27600_e29493 * assign27600_e29493))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27600_e29496;
        locals.var_temp2_dn4 = assign27600_e29496_d_n4;
        locals.var_temp2_dn6 = assign27600_e29496_d_n6;
        locals.var_temp2_dn7 = assign27600_e29496_d_n7;
        locals.var_temp2_dn8 = assign27600_e29496_d_n8;
        locals.var_temp2_dn9 = assign27600_e29496_d_n9;

        let (assign27610_e29528, assign27610_e29528_d_n4, assign27610_e29528_d_n6, assign27610_e29528_d_n7, assign27610_e29528_d_n8, assign27610_e29528_d_n9,) = {
    if (((locals.var_guard777 != 0.0) && (locals.var_guard778 == 0.0)) && (locals.var_guard779 == 0.0)) {
        let assign27610_e29508: f64 = (locals.var_temp - 80.0);
        let assign27610_e29513: f64 = (locals.var_temp - 80.0);
        let assign27610_e29514: f64 = (0.5 * assign27610_e29513);
        let assign27610_e29518: f64 = (locals.var_temp - 80.0);
        let assign27610_e29520: f64 = (assign27610_e29518 * 0.3333333333333);
        let assign27610_e29521: f64 = (1.0 + assign27610_e29520);
        let assign27610_e29522: f64 = (assign27610_e29514 * assign27610_e29521);
        let assign27610_e29523: f64 = (1.0 + assign27610_e29522);
        let assign27610_e29524: f64 = (assign27610_e29508 * assign27610_e29523);
        let assign27610_e29525: f64 = (1.0 + assign27610_e29524);
        let assign27610_e29526: f64 = (5.54062e34 * assign27610_e29525);
        (assign27610_e29526, (5.54062e34 * ((locals.var_temp_dn4 * assign27610_e29523) + (assign27610_e29508 * (((0.5 * locals.var_temp_dn4) * assign27610_e29521) + (assign27610_e29514 * (locals.var_temp_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn6 * assign27610_e29523) + (assign27610_e29508 * (((0.5 * locals.var_temp_dn6) * assign27610_e29521) + (assign27610_e29514 * (locals.var_temp_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn7 * assign27610_e29523) + (assign27610_e29508 * (((0.5 * locals.var_temp_dn7) * assign27610_e29521) + (assign27610_e29514 * (locals.var_temp_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn8 * assign27610_e29523) + (assign27610_e29508 * (((0.5 * locals.var_temp_dn8) * assign27610_e29521) + (assign27610_e29514 * (locals.var_temp_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp_dn9 * assign27610_e29523) + (assign27610_e29508 * (((0.5 * locals.var_temp_dn9) * assign27610_e29521) + (assign27610_e29514 * (locals.var_temp_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27610_e29528;
        locals.var_temp2_dn4 = assign27610_e29528_d_n4;
        locals.var_temp2_dn6 = assign27610_e29528_d_n6;
        locals.var_temp2_dn7 = assign27610_e29528_d_n7;
        locals.var_temp2_dn8 = assign27610_e29528_d_n8;
        locals.var_temp2_dn9 = assign27610_e29528_d_n9;

        let (assign27620_e29534, assign27620_e29534_d_n4, assign27620_e29534_d_n6, assign27620_e29534_d_n7, assign27620_e29534_d_n8, assign27620_e29534_d_n9,) = {
    if (locals.var_guard777 != 0.0) {
        let assign27620_e29532: f64 = (locals.var_dgidld_i * locals.var_vdsu);
        (assign27620_e29532, 0.0, (locals.var_dgidld_i * locals.var_vdsu_dn6), (locals.var_dgidld_i * locals.var_vdsu_dn7), 0.0, 0.0,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign27620_e29534;
        locals.var_temp3_dn4 = assign27620_e29534_d_n4;
        locals.var_temp3_dn6 = assign27620_e29534_d_n6;
        locals.var_temp3_dn7 = assign27620_e29534_d_n7;
        locals.var_temp3_dn8 = assign27620_e29534_d_n8;
        locals.var_temp3_dn9 = assign27620_e29534_d_n9;

        let assign27630_e29536: f64 = (locals.var_temp3).abs();
        let assign27630_e29538: f64 = if assign27630_e29536 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard780 = assign27630_e29538;

    }

    pub(super) fn stamp_transient_block_72(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27640_e29545, assign27640_e29545_d_n4, assign27640_e29545_d_n6, assign27640_e29545_d_n7, assign27640_e29545_d_n8, assign27640_e29545_d_n9,) = {
    if ((locals.var_guard777 != 0.0) && (locals.var_guard780 != 0.0)) {
        let assign27640_e29543: f64 = (locals.var_temp3).exp();
        (assign27640_e29543, (assign27640_e29543 * locals.var_temp3_dn4), (assign27640_e29543 * locals.var_temp3_dn6), (assign27640_e29543 * locals.var_temp3_dn7), (assign27640_e29543 * locals.var_temp3_dn8), (assign27640_e29543 * locals.var_temp3_dn9),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign27640_e29545;
        locals.var_temp4_dn4 = assign27640_e29545_d_n4;
        locals.var_temp4_dn6 = assign27640_e29545_d_n6;
        locals.var_temp4_dn7 = assign27640_e29545_d_n7;
        locals.var_temp4_dn8 = assign27640_e29545_d_n8;
        locals.var_temp4_dn9 = assign27640_e29545_d_n9;

        let assign27650_e29548: f64 = (-80.0);
        let assign27650_e29549: f64 = if locals.var_temp3 < assign27650_e29548 { 1.0 } else { 0.0 };
        locals.var_guard781 = assign27650_e29549;

        let (assign27660_e29583, assign27660_e29583_d_n4, assign27660_e29583_d_n6, assign27660_e29583_d_n7, assign27660_e29583_d_n8, assign27660_e29583_d_n9,) = {
    if (((locals.var_guard777 != 0.0) && (locals.var_guard780 == 0.0)) && (locals.var_guard781 != 0.0)) {
        let assign27660_e29559: f64 = (-locals.var_temp3);
        let assign27660_e29561: f64 = (assign27660_e29559 - 80.0);
        let assign27660_e29565: f64 = (-locals.var_temp3);
        let assign27660_e29567: f64 = (assign27660_e29565 - 80.0);
        let assign27660_e29568: f64 = (0.5 * assign27660_e29567);
        let assign27660_e29571: f64 = (-locals.var_temp3);
        let assign27660_e29573: f64 = (assign27660_e29571 - 80.0);
        let assign27660_e29575: f64 = (assign27660_e29573 * 0.3333333333333);
        let assign27660_e29576: f64 = (1.0 + assign27660_e29575);
        let assign27660_e29577: f64 = (assign27660_e29568 * assign27660_e29576);
        let assign27660_e29578: f64 = (1.0 + assign27660_e29577);
        let assign27660_e29579: f64 = (assign27660_e29561 * assign27660_e29578);
        let assign27660_e29580: f64 = (1.0 + assign27660_e29579);
        let assign27660_e29581: f64 = (1.80485e-35 / assign27660_e29580);
        (assign27660_e29581, (-((1.80485e-35 * (((-locals.var_temp3_dn4) * assign27660_e29578) + (assign27660_e29561 * (((0.5 * (-locals.var_temp3_dn4)) * assign27660_e29576) + (assign27660_e29568 * ((-locals.var_temp3_dn4) * 0.3333333333333)))))) / (assign27660_e29580 * assign27660_e29580))), (-((1.80485e-35 * (((-locals.var_temp3_dn6) * assign27660_e29578) + (assign27660_e29561 * (((0.5 * (-locals.var_temp3_dn6)) * assign27660_e29576) + (assign27660_e29568 * ((-locals.var_temp3_dn6) * 0.3333333333333)))))) / (assign27660_e29580 * assign27660_e29580))), (-((1.80485e-35 * (((-locals.var_temp3_dn7) * assign27660_e29578) + (assign27660_e29561 * (((0.5 * (-locals.var_temp3_dn7)) * assign27660_e29576) + (assign27660_e29568 * ((-locals.var_temp3_dn7) * 0.3333333333333)))))) / (assign27660_e29580 * assign27660_e29580))), (-((1.80485e-35 * (((-locals.var_temp3_dn8) * assign27660_e29578) + (assign27660_e29561 * (((0.5 * (-locals.var_temp3_dn8)) * assign27660_e29576) + (assign27660_e29568 * ((-locals.var_temp3_dn8) * 0.3333333333333)))))) / (assign27660_e29580 * assign27660_e29580))), (-((1.80485e-35 * (((-locals.var_temp3_dn9) * assign27660_e29578) + (assign27660_e29561 * (((0.5 * (-locals.var_temp3_dn9)) * assign27660_e29576) + (assign27660_e29568 * ((-locals.var_temp3_dn9) * 0.3333333333333)))))) / (assign27660_e29580 * assign27660_e29580))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign27660_e29583;
        locals.var_temp4_dn4 = assign27660_e29583_d_n4;
        locals.var_temp4_dn6 = assign27660_e29583_d_n6;
        locals.var_temp4_dn7 = assign27660_e29583_d_n7;
        locals.var_temp4_dn8 = assign27660_e29583_d_n8;
        locals.var_temp4_dn9 = assign27660_e29583_d_n9;

        let (assign27670_e29615, assign27670_e29615_d_n4, assign27670_e29615_d_n6, assign27670_e29615_d_n7, assign27670_e29615_d_n8, assign27670_e29615_d_n9,) = {
    if (((locals.var_guard777 != 0.0) && (locals.var_guard780 == 0.0)) && (locals.var_guard781 == 0.0)) {
        let assign27670_e29595: f64 = (locals.var_temp3 - 80.0);
        let assign27670_e29600: f64 = (locals.var_temp3 - 80.0);
        let assign27670_e29601: f64 = (0.5 * assign27670_e29600);
        let assign27670_e29605: f64 = (locals.var_temp3 - 80.0);
        let assign27670_e29607: f64 = (assign27670_e29605 * 0.3333333333333);
        let assign27670_e29608: f64 = (1.0 + assign27670_e29607);
        let assign27670_e29609: f64 = (assign27670_e29601 * assign27670_e29608);
        let assign27670_e29610: f64 = (1.0 + assign27670_e29609);
        let assign27670_e29611: f64 = (assign27670_e29595 * assign27670_e29610);
        let assign27670_e29612: f64 = (1.0 + assign27670_e29611);
        let assign27670_e29613: f64 = (5.54062e34 * assign27670_e29612);
        (assign27670_e29613, (5.54062e34 * ((locals.var_temp3_dn4 * assign27670_e29610) + (assign27670_e29595 * (((0.5 * locals.var_temp3_dn4) * assign27670_e29608) + (assign27670_e29601 * (locals.var_temp3_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn6 * assign27670_e29610) + (assign27670_e29595 * (((0.5 * locals.var_temp3_dn6) * assign27670_e29608) + (assign27670_e29601 * (locals.var_temp3_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn7 * assign27670_e29610) + (assign27670_e29595 * (((0.5 * locals.var_temp3_dn7) * assign27670_e29608) + (assign27670_e29601 * (locals.var_temp3_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn8 * assign27670_e29610) + (assign27670_e29595 * (((0.5 * locals.var_temp3_dn8) * assign27670_e29608) + (assign27670_e29601 * (locals.var_temp3_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp3_dn9 * assign27670_e29610) + (assign27670_e29595 * (((0.5 * locals.var_temp3_dn9) * assign27670_e29608) + (assign27670_e29601 * (locals.var_temp3_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign27670_e29615;
        locals.var_temp4_dn4 = assign27670_e29615_d_n4;
        locals.var_temp4_dn6 = assign27670_e29615_d_n6;
        locals.var_temp4_dn7 = assign27670_e29615_d_n7;
        locals.var_temp4_dn8 = assign27670_e29615_d_n8;
        locals.var_temp4_dn9 = assign27670_e29615_d_n9;

        let (assign27680_e29634, assign27680_e29634_d_n4, assign27680_e29634_d_n6, assign27680_e29634_d_n7, assign27680_e29634_d_n8, assign27680_e29634_d_n9,) = {
    if (locals.var_guard777 != 0.0) {
        let assign27680_e29618: f64 = (-locals.var_agidld_i);
        let assign27680_e29620: f64 = (assign27680_e29618 * locals.var_vdsu);
        let assign27680_e29622: f64 = (assign27680_e29620 * locals.var_vovd);
        let assign27680_e29624: f64 = (assign27680_e29622 * locals.var_vtovd);
        let assign27680_e29626: f64 = (assign27680_e29624 * locals.var_temp2);
        let assign27680_e29628: f64 = (assign27680_e29626 * 0.5);
        let assign27680_e29631: f64 = (1.0 + locals.var_temp4);
        let assign27680_e29632: f64 = (assign27680_e29628 * assign27680_e29631);
        (assign27680_e29632, (((((((((((-locals.var_agidld_i_dn4) * locals.var_vdsu) * locals.var_vovd) + (assign27680_e29620 * locals.var_vovd_dn4)) * locals.var_vtovd) + (assign27680_e29622 * locals.var_vtovd_dn4)) * locals.var_temp2) + (assign27680_e29624 * locals.var_temp2_dn4)) * 0.5) * assign27680_e29631) + (assign27680_e29628 * locals.var_temp4_dn4)), ((((((((((((-locals.var_agidld_i_dn6) * locals.var_vdsu) + (assign27680_e29618 * locals.var_vdsu_dn6)) * locals.var_vovd) + (assign27680_e29620 * locals.var_vovd_dn6)) * locals.var_vtovd) + (assign27680_e29622 * locals.var_vtovd_dn6)) * locals.var_temp2) + (assign27680_e29624 * locals.var_temp2_dn6)) * 0.5) * assign27680_e29631) + (assign27680_e29628 * locals.var_temp4_dn6)), ((((((((((((-locals.var_agidld_i_dn7) * locals.var_vdsu) + (assign27680_e29618 * locals.var_vdsu_dn7)) * locals.var_vovd) + (assign27680_e29620 * locals.var_vovd_dn7)) * locals.var_vtovd) + (assign27680_e29622 * locals.var_vtovd_dn7)) * locals.var_temp2) + (assign27680_e29624 * locals.var_temp2_dn7)) * 0.5) * assign27680_e29631) + (assign27680_e29628 * locals.var_temp4_dn7)), (((((((((((-locals.var_agidld_i_dn8) * locals.var_vdsu) * locals.var_vovd) + (assign27680_e29620 * locals.var_vovd_dn8)) * locals.var_vtovd) + (assign27680_e29622 * locals.var_vtovd_dn8)) * locals.var_temp2) + (assign27680_e29624 * locals.var_temp2_dn8)) * 0.5) * assign27680_e29631) + (assign27680_e29628 * locals.var_temp4_dn8)), (((((((((((-locals.var_agidld_i_dn9) * locals.var_vdsu) * locals.var_vovd) + (assign27680_e29620 * locals.var_vovd_dn9)) * locals.var_vtovd) + (assign27680_e29622 * locals.var_vtovd_dn9)) * locals.var_temp2) + (assign27680_e29624 * locals.var_temp2_dn9)) * 0.5) * assign27680_e29631) + (assign27680_e29628 * locals.var_temp4_dn9)),)
    } else {
        (locals.var_igidl, locals.var_igidl_dn4, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn8, locals.var_igidl_dn9,)
    }
};
        locals.var_igidl = assign27680_e29634;
        locals.var_igidl_dn4 = assign27680_e29634_d_n4;
        locals.var_igidl_dn6 = assign27680_e29634_d_n6;
        locals.var_igidl_dn7 = assign27680_e29634_d_n7;
        locals.var_igidl_dn8 = assign27680_e29634_d_n8;
        locals.var_igidl_dn9 = assign27680_e29634_d_n9;

        locals.var_ids_edge = 0.0;
        locals.var_ids_edge_dn4 = 0.0;
        locals.var_ids_edge_dn6 = 0.0;
        locals.var_ids_edge_dn7 = 0.0;
        locals.var_ids_edge_dn8 = 0.0;
        locals.var_ids_edge_dn9 = 0.0;

        let assign27700_e29638: f64 = if p.p12 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard782 = assign27700_e29638;

        let (assign27710_e29644, assign27710_e29644_d_n4, assign27710_e29644_d_n6, assign27710_e29644_d_n7, assign27710_e29644_d_n8, assign27710_e29644_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27710_e29642: f64 = (locals.var_vds * locals.var_inv_phit_edge);
        (assign27710_e29642, (locals.var_vds * locals.var_inv_phit_edge_dn4), ((locals.var_vds_dn6 * locals.var_inv_phit_edge) + (locals.var_vds * locals.var_inv_phit_edge_dn6)), ((locals.var_vds_dn7 * locals.var_inv_phit_edge) + (locals.var_vds * locals.var_inv_phit_edge_dn7)), (locals.var_vds * locals.var_inv_phit_edge_dn8), (locals.var_vds * locals.var_inv_phit_edge_dn9),)
    } else {
        (locals.var_xd_edge, locals.var_xd_edge_dn4, locals.var_xd_edge_dn6, locals.var_xd_edge_dn7, locals.var_xd_edge_dn8, locals.var_xd_edge_dn9,)
    }
};
        locals.var_xd_edge = assign27710_e29644;
        locals.var_xd_edge_dn4 = assign27710_e29644_d_n4;
        locals.var_xd_edge_dn6 = assign27710_e29644_d_n6;
        locals.var_xd_edge_dn7 = assign27710_e29644_d_n7;
        locals.var_xd_edge_dn8 = assign27710_e29644_d_n8;
        locals.var_xd_edge_dn9 = assign27710_e29644_d_n9;

        let (assign27720_e29657, assign27720_e29657_d_n4, assign27720_e29657_d_n6, assign27720_e29657_d_n7, assign27720_e29657_d_n8, assign27720_e29657_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27720_e29648: f64 = (locals.var_vds * locals.var_vds);
        let assign27720_e29650: f64 = (assign27720_e29648 + 0.01);
        let assign27720_e29651: f64 = (assign27720_e29650).sqrt();
        let assign27720_e29653: f64 = (assign27720_e29651 - 0.1);
        let assign27720_e29655: f64 = (assign27720_e29653 * locals.var_inv_phit_edge);
        (assign27720_e29655, (assign27720_e29653 * locals.var_inv_phit_edge_dn4), (((((locals.var_vds_dn6 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn6)) / (2.0 * assign27720_e29651)) * locals.var_inv_phit_edge) + (assign27720_e29653 * locals.var_inv_phit_edge_dn6)), (((((locals.var_vds_dn7 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn7)) / (2.0 * assign27720_e29651)) * locals.var_inv_phit_edge) + (assign27720_e29653 * locals.var_inv_phit_edge_dn7)), (assign27720_e29653 * locals.var_inv_phit_edge_dn8), (assign27720_e29653 * locals.var_inv_phit_edge_dn9),)
    } else {
        (locals.var_xdsx_edge, locals.var_xdsx_edge_dn4, locals.var_xdsx_edge_dn6, locals.var_xdsx_edge_dn7, locals.var_xdsx_edge_dn8, locals.var_xdsx_edge_dn9,)
    }
};
        locals.var_xdsx_edge = assign27720_e29657;
        locals.var_xdsx_edge_dn4 = assign27720_e29657_d_n4;
        locals.var_xdsx_edge_dn6 = assign27720_e29657_d_n6;
        locals.var_xdsx_edge_dn7 = assign27720_e29657_d_n7;
        locals.var_xdsx_edge_dn8 = assign27720_e29657_d_n8;
        locals.var_xdsx_edge_dn9 = assign27720_e29657_d_n9;

        let (assign27730_e29665, assign27730_e29665_d_n4, assign27730_e29665_d_n6, assign27730_e29665_d_n7, assign27730_e29665_d_n8, assign27730_e29665_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27730_e29662: f64 = (locals.var_xd_edge - locals.var_xdsx_edge);
        let assign27730_e29663: f64 = (0.5 * assign27730_e29662);
        (assign27730_e29663, (0.5 * (locals.var_xd_edge_dn4 - locals.var_xdsx_edge_dn4)), (0.5 * (locals.var_xd_edge_dn6 - locals.var_xdsx_edge_dn6)), (0.5 * (locals.var_xd_edge_dn7 - locals.var_xdsx_edge_dn7)), (0.5 * (locals.var_xd_edge_dn8 - locals.var_xdsx_edge_dn8)), (0.5 * (locals.var_xd_edge_dn9 - locals.var_xdsx_edge_dn9)),)
    } else {
        (locals.var_dxdsx_edge, locals.var_dxdsx_edge_dn4, locals.var_dxdsx_edge_dn6, locals.var_dxdsx_edge_dn7, locals.var_dxdsx_edge_dn8, locals.var_dxdsx_edge_dn9,)
    }
};
        locals.var_dxdsx_edge = assign27730_e29665;
        locals.var_dxdsx_edge_dn4 = assign27730_e29665_d_n4;
        locals.var_dxdsx_edge_dn6 = assign27730_e29665_d_n6;
        locals.var_dxdsx_edge_dn7 = assign27730_e29665_d_n7;
        locals.var_dxdsx_edge_dn8 = assign27730_e29665_d_n8;
        locals.var_dxdsx_edge_dn9 = assign27730_e29665_d_n9;

        let (assign27740_e29677, assign27740_e29677_d_n4, assign27740_e29677_d_n6, assign27740_e29677_d_n7, assign27740_e29677_d_n8, assign27740_e29677_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27740_e29669: f64 = (locals.var_vgs - locals.var_vfb1edge_i);
        let assign27740_e29671: f64 = (assign27740_e29669 * locals.var_inv_phit_edge);
        let assign27740_e29673: f64 = (assign27740_e29671 - locals.var_dxdsx_edge);
        let assign27740_e29675: f64 = (assign27740_e29673 - locals.var_eg_2phit0);
        (assign27740_e29675, (((((-locals.var_vfb1edge_i_dn4) * locals.var_inv_phit_edge) + (assign27740_e29669 * locals.var_inv_phit_edge_dn4)) - locals.var_dxdsx_edge_dn4) - locals.var_eg_2phit0_dn4), (((((locals.var_vgs_dn6 - locals.var_vfb1edge_i_dn6) * locals.var_inv_phit_edge) + (assign27740_e29669 * locals.var_inv_phit_edge_dn6)) - locals.var_dxdsx_edge_dn6) - locals.var_eg_2phit0_dn6), (((((locals.var_vgs_dn7 - locals.var_vfb1edge_i_dn7) * locals.var_inv_phit_edge) + (assign27740_e29669 * locals.var_inv_phit_edge_dn7)) - locals.var_dxdsx_edge_dn7) - locals.var_eg_2phit0_dn7), (((((-locals.var_vfb1edge_i_dn8) * locals.var_inv_phit_edge) + (assign27740_e29669 * locals.var_inv_phit_edge_dn8)) - locals.var_dxdsx_edge_dn8) - locals.var_eg_2phit0_dn8), (((((locals.var_vgs_dn9 - locals.var_vfb1edge_i_dn9) * locals.var_inv_phit_edge) + (assign27740_e29669 * locals.var_inv_phit_edge_dn9)) - locals.var_dxdsx_edge_dn9) - locals.var_eg_2phit0_dn9),)
    } else {
        (locals.var_xg10_edge, locals.var_xg10_edge_dn4, locals.var_xg10_edge_dn6, locals.var_xg10_edge_dn7, locals.var_xg10_edge_dn8, locals.var_xg10_edge_dn9,)
    }
};
        locals.var_xg10_edge = assign27740_e29677;
        locals.var_xg10_edge_dn4 = assign27740_e29677_d_n4;
        locals.var_xg10_edge_dn6 = assign27740_e29677_d_n6;
        locals.var_xg10_edge_dn7 = assign27740_e29677_d_n7;
        locals.var_xg10_edge_dn8 = assign27740_e29677_d_n8;
        locals.var_xg10_edge_dn9 = assign27740_e29677_d_n9;

        let (assign27750_e29690, assign27750_e29690_d_n4, assign27750_e29690_d_n6, assign27750_e29690_d_n7, assign27750_e29690_d_n8, assign27750_e29690_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27750_e29680: f64 = (-locals.var_vsb);
        let assign27750_e29682: f64 = (assign27750_e29680 - locals.var_vfb2edge_i);
        let assign27750_e29684: f64 = (assign27750_e29682 * locals.var_inv_phit_edge);
        let assign27750_e29686: f64 = (assign27750_e29684 - locals.var_dxdsx_edge);
        let assign27750_e29688: f64 = (assign27750_e29686 - locals.var_eg_2phit0);
        (assign27750_e29688, (((((-locals.var_vfb2edge_i_dn4) * locals.var_inv_phit_edge) + (assign27750_e29682 * locals.var_inv_phit_edge_dn4)) - locals.var_dxdsx_edge_dn4) - locals.var_eg_2phit0_dn4), ((((((-locals.var_vsb_dn6) - locals.var_vfb2edge_i_dn6) * locals.var_inv_phit_edge) + (assign27750_e29682 * locals.var_inv_phit_edge_dn6)) - locals.var_dxdsx_edge_dn6) - locals.var_eg_2phit0_dn6), ((((((-locals.var_vsb_dn7) - locals.var_vfb2edge_i_dn7) * locals.var_inv_phit_edge) + (assign27750_e29682 * locals.var_inv_phit_edge_dn7)) - locals.var_dxdsx_edge_dn7) - locals.var_eg_2phit0_dn7), ((((((-locals.var_vsb_dn8) - locals.var_vfb2edge_i_dn8) * locals.var_inv_phit_edge) + (assign27750_e29682 * locals.var_inv_phit_edge_dn8)) - locals.var_dxdsx_edge_dn8) - locals.var_eg_2phit0_dn8), (((((-locals.var_vfb2edge_i_dn9) * locals.var_inv_phit_edge) + (assign27750_e29682 * locals.var_inv_phit_edge_dn9)) - locals.var_dxdsx_edge_dn9) - locals.var_eg_2phit0_dn9),)
    } else {
        (locals.var_xg20_edge, locals.var_xg20_edge_dn4, locals.var_xg20_edge_dn6, locals.var_xg20_edge_dn7, locals.var_xg20_edge_dn8, locals.var_xg20_edge_dn9,)
    }
};
        locals.var_xg20_edge = assign27750_e29690;
        locals.var_xg20_edge_dn4 = assign27750_e29690_d_n4;
        locals.var_xg20_edge_dn6 = assign27750_e29690_d_n6;
        locals.var_xg20_edge_dn7 = assign27750_e29690_d_n7;
        locals.var_xg20_edge_dn8 = assign27750_e29690_d_n8;
        locals.var_xg20_edge_dn9 = assign27750_e29690_d_n9;

        let (assign27760_e29698, assign27760_e29698_d_n4, assign27760_e29698_d_n6, assign27760_e29698_d_n7, assign27760_e29698_d_n8, assign27760_e29698_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27760_e29695: f64 = (1.0 + locals.var_psce1edge_i);
        let assign27760_e29696: f64 = (1.0 / assign27760_e29695);
        (assign27760_e29696, (-(locals.var_psce1edge_i_dn4 / (assign27760_e29695 * assign27760_e29695))), (-(locals.var_psce1edge_i_dn6 / (assign27760_e29695 * assign27760_e29695))), (-(locals.var_psce1edge_i_dn7 / (assign27760_e29695 * assign27760_e29695))), (-(locals.var_psce1edge_i_dn8 / (assign27760_e29695 * assign27760_e29695))), (-(locals.var_psce1edge_i_dn9 / (assign27760_e29695 * assign27760_e29695))),)
    } else {
        (locals.var_sce1_edge, locals.var_sce1_edge_dn4, locals.var_sce1_edge_dn6, locals.var_sce1_edge_dn7, locals.var_sce1_edge_dn8, locals.var_sce1_edge_dn9,)
    }
};
        locals.var_sce1_edge = assign27760_e29698;
        locals.var_sce1_edge_dn4 = assign27760_e29698_d_n4;
        locals.var_sce1_edge_dn6 = assign27760_e29698_d_n6;
        locals.var_sce1_edge_dn7 = assign27760_e29698_d_n7;
        locals.var_sce1_edge_dn8 = assign27760_e29698_d_n8;
        locals.var_sce1_edge_dn9 = assign27760_e29698_d_n9;

        let (assign27770_e29706, assign27770_e29706_d_n4, assign27770_e29706_d_n6, assign27770_e29706_d_n7, assign27770_e29706_d_n8, assign27770_e29706_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27770_e29703: f64 = (1.0 + locals.var_psce2edge_i);
        let assign27770_e29704: f64 = (1.0 / assign27770_e29703);
        (assign27770_e29704, (-(locals.var_psce2edge_i_dn4 / (assign27770_e29703 * assign27770_e29703))), (-(locals.var_psce2edge_i_dn6 / (assign27770_e29703 * assign27770_e29703))), (-(locals.var_psce2edge_i_dn7 / (assign27770_e29703 * assign27770_e29703))), (-(locals.var_psce2edge_i_dn8 / (assign27770_e29703 * assign27770_e29703))), (-(locals.var_psce2edge_i_dn9 / (assign27770_e29703 * assign27770_e29703))),)
    } else {
        (locals.var_sce2_edge, locals.var_sce2_edge_dn4, locals.var_sce2_edge_dn6, locals.var_sce2_edge_dn7, locals.var_sce2_edge_dn8, locals.var_sce2_edge_dn9,)
    }
};
        locals.var_sce2_edge = assign27770_e29706;
        locals.var_sce2_edge_dn4 = assign27770_e29706_d_n4;
        locals.var_sce2_edge_dn6 = assign27770_e29706_d_n6;
        locals.var_sce2_edge_dn7 = assign27770_e29706_d_n7;
        locals.var_sce2_edge_dn8 = assign27770_e29706_d_n8;
        locals.var_sce2_edge_dn9 = assign27770_e29706_d_n9;

        let (assign27780_e29712, assign27780_e29712_d_n4, assign27780_e29712_d_n6, assign27780_e29712_d_n7, assign27780_e29712_d_n8, assign27780_e29712_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27780_e29710: f64 = (locals.var_cfdedge_i * locals.var_inv_phit_edge);
        (assign27780_e29710, (locals.var_cfdedge_i * locals.var_inv_phit_edge_dn4), (locals.var_cfdedge_i * locals.var_inv_phit_edge_dn6), (locals.var_cfdedge_i * locals.var_inv_phit_edge_dn7), (locals.var_cfdedge_i * locals.var_inv_phit_edge_dn8), (locals.var_cfdedge_i * locals.var_inv_phit_edge_dn9),)
    } else {
        (locals.var_xd0_edge, locals.var_xd0_edge_dn4, locals.var_xd0_edge_dn6, locals.var_xd0_edge_dn7, locals.var_xd0_edge_dn8, locals.var_xd0_edge_dn9,)
    }
};
        locals.var_xd0_edge = assign27780_e29712;
        locals.var_xd0_edge_dn4 = assign27780_e29712_d_n4;
        locals.var_xd0_edge_dn6 = assign27780_e29712_d_n6;
        locals.var_xd0_edge_dn7 = assign27780_e29712_d_n7;
        locals.var_xd0_edge_dn8 = assign27780_e29712_d_n8;
        locals.var_xd0_edge_dn9 = assign27780_e29712_d_n9;

        let (assign27790_e29727, assign27790_e29727_d_n4, assign27790_e29727_d_n6, assign27790_e29727_d_n7, assign27790_e29727_d_n8, assign27790_e29727_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27790_e29716: f64 = (2.0 * locals.var_xd0_edge);
        let assign27790_e29720: f64 = (locals.var_xdsx_edge / locals.var_xd0_edge);
        let assign27790_e29721: f64 = (1.0 + assign27790_e29720);
        let assign27790_e29722: f64 = (assign27790_e29721).sqrt();
        let assign27790_e29724: f64 = (assign27790_e29722 - 1.0);
        let assign27790_e29725: f64 = (assign27790_e29716 * assign27790_e29724);
        (assign27790_e29725, (((2.0 * locals.var_xd0_edge_dn4) * assign27790_e29724) + (assign27790_e29716 * ((((locals.var_xdsx_edge_dn4 * locals.var_xd0_edge) - (locals.var_xdsx_edge * locals.var_xd0_edge_dn4)) / (locals.var_xd0_edge * locals.var_xd0_edge)) / (2.0 * assign27790_e29722)))), (((2.0 * locals.var_xd0_edge_dn6) * assign27790_e29724) + (assign27790_e29716 * ((((locals.var_xdsx_edge_dn6 * locals.var_xd0_edge) - (locals.var_xdsx_edge * locals.var_xd0_edge_dn6)) / (locals.var_xd0_edge * locals.var_xd0_edge)) / (2.0 * assign27790_e29722)))), (((2.0 * locals.var_xd0_edge_dn7) * assign27790_e29724) + (assign27790_e29716 * ((((locals.var_xdsx_edge_dn7 * locals.var_xd0_edge) - (locals.var_xdsx_edge * locals.var_xd0_edge_dn7)) / (locals.var_xd0_edge * locals.var_xd0_edge)) / (2.0 * assign27790_e29722)))), (((2.0 * locals.var_xd0_edge_dn8) * assign27790_e29724) + (assign27790_e29716 * ((((locals.var_xdsx_edge_dn8 * locals.var_xd0_edge) - (locals.var_xdsx_edge * locals.var_xd0_edge_dn8)) / (locals.var_xd0_edge * locals.var_xd0_edge)) / (2.0 * assign27790_e29722)))), (((2.0 * locals.var_xd0_edge_dn9) * assign27790_e29724) + (assign27790_e29716 * ((((locals.var_xdsx_edge_dn9 * locals.var_xd0_edge) - (locals.var_xdsx_edge * locals.var_xd0_edge_dn9)) / (locals.var_xd0_edge * locals.var_xd0_edge)) / (2.0 * assign27790_e29722)))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign27790_e29727;
        locals.var_temp_dn4 = assign27790_e29727_d_n4;
        locals.var_temp_dn6 = assign27790_e29727_d_n6;
        locals.var_temp_dn7 = assign27790_e29727_d_n7;
        locals.var_temp_dn8 = assign27790_e29727_d_n8;
        locals.var_temp_dn9 = assign27790_e29727_d_n9;

        let (assign27800_e29733, assign27800_e29733_d_n4, assign27800_e29733_d_n6, assign27800_e29733_d_n7, assign27800_e29733_d_n8, assign27800_e29733_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27800_e29731: f64 = (locals.var_cf1edge_i * locals.var_temp);
        (assign27800_e29731, ((locals.var_cf1edge_i_dn4 * locals.var_temp) + (locals.var_cf1edge_i * locals.var_temp_dn4)), ((locals.var_cf1edge_i_dn6 * locals.var_temp) + (locals.var_cf1edge_i * locals.var_temp_dn6)), ((locals.var_cf1edge_i_dn7 * locals.var_temp) + (locals.var_cf1edge_i * locals.var_temp_dn7)), ((locals.var_cf1edge_i_dn8 * locals.var_temp) + (locals.var_cf1edge_i * locals.var_temp_dn8)), ((locals.var_cf1edge_i_dn9 * locals.var_temp) + (locals.var_cf1edge_i * locals.var_temp_dn9)),)
    } else {
        (locals.var_dxg1_dibl_edge, locals.var_dxg1_dibl_edge_dn4, locals.var_dxg1_dibl_edge_dn6, locals.var_dxg1_dibl_edge_dn7, locals.var_dxg1_dibl_edge_dn8, locals.var_dxg1_dibl_edge_dn9,)
    }
};
        locals.var_dxg1_dibl_edge = assign27800_e29733;
        locals.var_dxg1_dibl_edge_dn4 = assign27800_e29733_d_n4;
        locals.var_dxg1_dibl_edge_dn6 = assign27800_e29733_d_n6;
        locals.var_dxg1_dibl_edge_dn7 = assign27800_e29733_d_n7;
        locals.var_dxg1_dibl_edge_dn8 = assign27800_e29733_d_n8;
        locals.var_dxg1_dibl_edge_dn9 = assign27800_e29733_d_n9;

        let (assign27810_e29739, assign27810_e29739_d_n4, assign27810_e29739_d_n6, assign27810_e29739_d_n7, assign27810_e29739_d_n8, assign27810_e29739_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27810_e29737: f64 = (locals.var_cf2edge_i * locals.var_temp);
        (assign27810_e29737, ((locals.var_cf2edge_i_dn4 * locals.var_temp) + (locals.var_cf2edge_i * locals.var_temp_dn4)), ((locals.var_cf2edge_i_dn6 * locals.var_temp) + (locals.var_cf2edge_i * locals.var_temp_dn6)), ((locals.var_cf2edge_i_dn7 * locals.var_temp) + (locals.var_cf2edge_i * locals.var_temp_dn7)), ((locals.var_cf2edge_i_dn8 * locals.var_temp) + (locals.var_cf2edge_i * locals.var_temp_dn8)), ((locals.var_cf2edge_i_dn9 * locals.var_temp) + (locals.var_cf2edge_i * locals.var_temp_dn9)),)
    } else {
        (locals.var_dxg2_dibl_edge, locals.var_dxg2_dibl_edge_dn4, locals.var_dxg2_dibl_edge_dn6, locals.var_dxg2_dibl_edge_dn7, locals.var_dxg2_dibl_edge_dn8, locals.var_dxg2_dibl_edge_dn9,)
    }
};
        locals.var_dxg2_dibl_edge = assign27810_e29739;
        locals.var_dxg2_dibl_edge_dn4 = assign27810_e29739_d_n4;
        locals.var_dxg2_dibl_edge_dn6 = assign27810_e29739_d_n6;
        locals.var_dxg2_dibl_edge_dn7 = assign27810_e29739_d_n7;
        locals.var_dxg2_dibl_edge_dn8 = assign27810_e29739_d_n8;
        locals.var_dxg2_dibl_edge_dn9 = assign27810_e29739_d_n9;

        let (assign27820_e29749, assign27820_e29749_d_n4, assign27820_e29749_d_n6, assign27820_e29749_d_n7, assign27820_e29749_d_n8, assign27820_e29749_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27820_e29743: f64 = (locals.var_xg10_edge + locals.var_dxg1_dibl_edge);
        let assign27820_e29745: f64 = (assign27820_e29743 * locals.var_sce1_edge);
        let assign27820_e29747: f64 = (assign27820_e29745 + locals.var_dxdsx_edge);
        (assign27820_e29747, ((((locals.var_xg10_edge_dn4 + locals.var_dxg1_dibl_edge_dn4) * locals.var_sce1_edge) + (assign27820_e29743 * locals.var_sce1_edge_dn4)) + locals.var_dxdsx_edge_dn4), ((((locals.var_xg10_edge_dn6 + locals.var_dxg1_dibl_edge_dn6) * locals.var_sce1_edge) + (assign27820_e29743 * locals.var_sce1_edge_dn6)) + locals.var_dxdsx_edge_dn6), ((((locals.var_xg10_edge_dn7 + locals.var_dxg1_dibl_edge_dn7) * locals.var_sce1_edge) + (assign27820_e29743 * locals.var_sce1_edge_dn7)) + locals.var_dxdsx_edge_dn7), ((((locals.var_xg10_edge_dn8 + locals.var_dxg1_dibl_edge_dn8) * locals.var_sce1_edge) + (assign27820_e29743 * locals.var_sce1_edge_dn8)) + locals.var_dxdsx_edge_dn8), ((((locals.var_xg10_edge_dn9 + locals.var_dxg1_dibl_edge_dn9) * locals.var_sce1_edge) + (assign27820_e29743 * locals.var_sce1_edge_dn9)) + locals.var_dxdsx_edge_dn9),)
    } else {
        (locals.var_xg1_edge, locals.var_xg1_edge_dn4, locals.var_xg1_edge_dn6, locals.var_xg1_edge_dn7, locals.var_xg1_edge_dn8, locals.var_xg1_edge_dn9,)
    }
};
        locals.var_xg1_edge = assign27820_e29749;
        locals.var_xg1_edge_dn4 = assign27820_e29749_d_n4;
        locals.var_xg1_edge_dn6 = assign27820_e29749_d_n6;
        locals.var_xg1_edge_dn7 = assign27820_e29749_d_n7;
        locals.var_xg1_edge_dn8 = assign27820_e29749_d_n8;
        locals.var_xg1_edge_dn9 = assign27820_e29749_d_n9;

        let (assign27830_e29759, assign27830_e29759_d_n4, assign27830_e29759_d_n6, assign27830_e29759_d_n7, assign27830_e29759_d_n8, assign27830_e29759_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27830_e29753: f64 = (locals.var_xg20_edge + locals.var_dxg2_dibl_edge);
        let assign27830_e29755: f64 = (assign27830_e29753 * locals.var_sce2_edge);
        let assign27830_e29757: f64 = (assign27830_e29755 + locals.var_dxdsx_edge);
        (assign27830_e29757, ((((locals.var_xg20_edge_dn4 + locals.var_dxg2_dibl_edge_dn4) * locals.var_sce2_edge) + (assign27830_e29753 * locals.var_sce2_edge_dn4)) + locals.var_dxdsx_edge_dn4), ((((locals.var_xg20_edge_dn6 + locals.var_dxg2_dibl_edge_dn6) * locals.var_sce2_edge) + (assign27830_e29753 * locals.var_sce2_edge_dn6)) + locals.var_dxdsx_edge_dn6), ((((locals.var_xg20_edge_dn7 + locals.var_dxg2_dibl_edge_dn7) * locals.var_sce2_edge) + (assign27830_e29753 * locals.var_sce2_edge_dn7)) + locals.var_dxdsx_edge_dn7), ((((locals.var_xg20_edge_dn8 + locals.var_dxg2_dibl_edge_dn8) * locals.var_sce2_edge) + (assign27830_e29753 * locals.var_sce2_edge_dn8)) + locals.var_dxdsx_edge_dn8), ((((locals.var_xg20_edge_dn9 + locals.var_dxg2_dibl_edge_dn9) * locals.var_sce2_edge) + (assign27830_e29753 * locals.var_sce2_edge_dn9)) + locals.var_dxdsx_edge_dn9),)
    } else {
        (locals.var_xg2_edge, locals.var_xg2_edge_dn4, locals.var_xg2_edge_dn6, locals.var_xg2_edge_dn7, locals.var_xg2_edge_dn8, locals.var_xg2_edge_dn9,)
    }
};
        locals.var_xg2_edge = assign27830_e29759;
        locals.var_xg2_edge_dn4 = assign27830_e29759_d_n4;
        locals.var_xg2_edge_dn6 = assign27830_e29759_d_n6;
        locals.var_xg2_edge_dn7 = assign27830_e29759_d_n7;
        locals.var_xg2_edge_dn8 = assign27830_e29759_d_n8;
        locals.var_xg2_edge_dn9 = assign27830_e29759_d_n9;

        let (assign27840_e29796, assign27840_e29796_d_n4, assign27840_e29796_d_n6, assign27840_e29796_d_n7, assign27840_e29796_d_n8, assign27840_e29796_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27840_e29766: f64 = (locals.var_xg1_edge - locals.var_xg2_edge);
        let assign27840_e29767: f64 = (locals.var_cic1edge_i * assign27840_e29766);
        let assign27840_e29768: f64 = (locals.var_xg2_edge + assign27840_e29767);
        let assign27840_e29770: f64 = (assign27840_e29768 + locals.var_xsatmax);
        let assign27840_e29775: f64 = (locals.var_xg1_edge - locals.var_xg2_edge);
        let assign27840_e29776: f64 = (locals.var_cic1edge_i * assign27840_e29775);
        let assign27840_e29777: f64 = (locals.var_xg2_edge + assign27840_e29776);
        let assign27840_e29779: f64 = (assign27840_e29777 - locals.var_xsatmax);
        let assign27840_e29784: f64 = (locals.var_xg1_edge - locals.var_xg2_edge);
        let assign27840_e29785: f64 = (locals.var_cic1edge_i * assign27840_e29784);
        let assign27840_e29786: f64 = (locals.var_xg2_edge + assign27840_e29785);
        let assign27840_e29788: f64 = (assign27840_e29786 - locals.var_xsatmax);
        let assign27840_e29789: f64 = (assign27840_e29779 * assign27840_e29788);
        let assign27840_e29791: f64 = (assign27840_e29789 + 0.01);
        let assign27840_e29792: f64 = (assign27840_e29791).sqrt();
        let assign27840_e29793: f64 = (assign27840_e29770 - assign27840_e29792);
        let assign27840_e29794: f64 = (0.5 * assign27840_e29793);
        (assign27840_e29794, (0.5 * (((locals.var_xg2_edge_dn4 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn4 - locals.var_xg2_edge_dn4))) + locals.var_xsatmax_dn4) - (((((locals.var_xg2_edge_dn4 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn4 - locals.var_xg2_edge_dn4))) - locals.var_xsatmax_dn4) * assign27840_e29788) + (assign27840_e29779 * ((locals.var_xg2_edge_dn4 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn4 - locals.var_xg2_edge_dn4))) - locals.var_xsatmax_dn4))) / (2.0 * assign27840_e29792)))), (0.5 * (((locals.var_xg2_edge_dn6 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn6 - locals.var_xg2_edge_dn6))) + locals.var_xsatmax_dn6) - (((((locals.var_xg2_edge_dn6 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn6 - locals.var_xg2_edge_dn6))) - locals.var_xsatmax_dn6) * assign27840_e29788) + (assign27840_e29779 * ((locals.var_xg2_edge_dn6 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn6 - locals.var_xg2_edge_dn6))) - locals.var_xsatmax_dn6))) / (2.0 * assign27840_e29792)))), (0.5 * (((locals.var_xg2_edge_dn7 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn7 - locals.var_xg2_edge_dn7))) + locals.var_xsatmax_dn7) - (((((locals.var_xg2_edge_dn7 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn7 - locals.var_xg2_edge_dn7))) - locals.var_xsatmax_dn7) * assign27840_e29788) + (assign27840_e29779 * ((locals.var_xg2_edge_dn7 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn7 - locals.var_xg2_edge_dn7))) - locals.var_xsatmax_dn7))) / (2.0 * assign27840_e29792)))), (0.5 * (((locals.var_xg2_edge_dn8 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn8 - locals.var_xg2_edge_dn8))) + locals.var_xsatmax_dn8) - (((((locals.var_xg2_edge_dn8 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn8 - locals.var_xg2_edge_dn8))) - locals.var_xsatmax_dn8) * assign27840_e29788) + (assign27840_e29779 * ((locals.var_xg2_edge_dn8 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn8 - locals.var_xg2_edge_dn8))) - locals.var_xsatmax_dn8))) / (2.0 * assign27840_e29792)))), (0.5 * (((locals.var_xg2_edge_dn9 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn9 - locals.var_xg2_edge_dn9))) + locals.var_xsatmax_dn9) - (((((locals.var_xg2_edge_dn9 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn9 - locals.var_xg2_edge_dn9))) - locals.var_xsatmax_dn9) * assign27840_e29788) + (assign27840_e29779 * ((locals.var_xg2_edge_dn9 + (locals.var_cic1edge_i * (locals.var_xg1_edge_dn9 - locals.var_xg2_edge_dn9))) - locals.var_xsatmax_dn9))) / (2.0 * assign27840_e29792)))),)
    } else {
        (locals.var_xg1x_edge, locals.var_xg1x_edge_dn4, locals.var_xg1x_edge_dn6, locals.var_xg1x_edge_dn7, locals.var_xg1x_edge_dn8, locals.var_xg1x_edge_dn9,)
    }
};
        locals.var_xg1x_edge = assign27840_e29796;
        locals.var_xg1x_edge_dn4 = assign27840_e29796_d_n4;
        locals.var_xg1x_edge_dn6 = assign27840_e29796_d_n6;
        locals.var_xg1x_edge_dn7 = assign27840_e29796_d_n7;
        locals.var_xg1x_edge_dn8 = assign27840_e29796_d_n8;
        locals.var_xg1x_edge_dn9 = assign27840_e29796_d_n9;

        let (assign27850_e29833, assign27850_e29833_d_n4, assign27850_e29833_d_n6, assign27850_e29833_d_n7, assign27850_e29833_d_n8, assign27850_e29833_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27850_e29803: f64 = (locals.var_xg2_edge - locals.var_xg1_edge);
        let assign27850_e29804: f64 = (locals.var_cic2edge_i * assign27850_e29803);
        let assign27850_e29805: f64 = (locals.var_xg1_edge + assign27850_e29804);
        let assign27850_e29807: f64 = (assign27850_e29805 + locals.var_xsatmax);
        let assign27850_e29812: f64 = (locals.var_xg2_edge - locals.var_xg1_edge);
        let assign27850_e29813: f64 = (locals.var_cic2edge_i * assign27850_e29812);
        let assign27850_e29814: f64 = (locals.var_xg1_edge + assign27850_e29813);
        let assign27850_e29816: f64 = (assign27850_e29814 - locals.var_xsatmax);
        let assign27850_e29821: f64 = (locals.var_xg2_edge - locals.var_xg1_edge);
        let assign27850_e29822: f64 = (locals.var_cic2edge_i * assign27850_e29821);
        let assign27850_e29823: f64 = (locals.var_xg1_edge + assign27850_e29822);
        let assign27850_e29825: f64 = (assign27850_e29823 - locals.var_xsatmax);
        let assign27850_e29826: f64 = (assign27850_e29816 * assign27850_e29825);
        let assign27850_e29828: f64 = (assign27850_e29826 + 0.01);
        let assign27850_e29829: f64 = (assign27850_e29828).sqrt();
        let assign27850_e29830: f64 = (assign27850_e29807 - assign27850_e29829);
        let assign27850_e29831: f64 = (0.5 * assign27850_e29830);
        (assign27850_e29831, (0.5 * (((locals.var_xg1_edge_dn4 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn4 - locals.var_xg1_edge_dn4))) + locals.var_xsatmax_dn4) - (((((locals.var_xg1_edge_dn4 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn4 - locals.var_xg1_edge_dn4))) - locals.var_xsatmax_dn4) * assign27850_e29825) + (assign27850_e29816 * ((locals.var_xg1_edge_dn4 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn4 - locals.var_xg1_edge_dn4))) - locals.var_xsatmax_dn4))) / (2.0 * assign27850_e29829)))), (0.5 * (((locals.var_xg1_edge_dn6 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn6 - locals.var_xg1_edge_dn6))) + locals.var_xsatmax_dn6) - (((((locals.var_xg1_edge_dn6 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn6 - locals.var_xg1_edge_dn6))) - locals.var_xsatmax_dn6) * assign27850_e29825) + (assign27850_e29816 * ((locals.var_xg1_edge_dn6 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn6 - locals.var_xg1_edge_dn6))) - locals.var_xsatmax_dn6))) / (2.0 * assign27850_e29829)))), (0.5 * (((locals.var_xg1_edge_dn7 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn7 - locals.var_xg1_edge_dn7))) + locals.var_xsatmax_dn7) - (((((locals.var_xg1_edge_dn7 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn7 - locals.var_xg1_edge_dn7))) - locals.var_xsatmax_dn7) * assign27850_e29825) + (assign27850_e29816 * ((locals.var_xg1_edge_dn7 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn7 - locals.var_xg1_edge_dn7))) - locals.var_xsatmax_dn7))) / (2.0 * assign27850_e29829)))), (0.5 * (((locals.var_xg1_edge_dn8 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn8 - locals.var_xg1_edge_dn8))) + locals.var_xsatmax_dn8) - (((((locals.var_xg1_edge_dn8 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn8 - locals.var_xg1_edge_dn8))) - locals.var_xsatmax_dn8) * assign27850_e29825) + (assign27850_e29816 * ((locals.var_xg1_edge_dn8 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn8 - locals.var_xg1_edge_dn8))) - locals.var_xsatmax_dn8))) / (2.0 * assign27850_e29829)))), (0.5 * (((locals.var_xg1_edge_dn9 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn9 - locals.var_xg1_edge_dn9))) + locals.var_xsatmax_dn9) - (((((locals.var_xg1_edge_dn9 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn9 - locals.var_xg1_edge_dn9))) - locals.var_xsatmax_dn9) * assign27850_e29825) + (assign27850_e29816 * ((locals.var_xg1_edge_dn9 + (locals.var_cic2edge_i * (locals.var_xg2_edge_dn9 - locals.var_xg1_edge_dn9))) - locals.var_xsatmax_dn9))) / (2.0 * assign27850_e29829)))),)
    } else {
        (locals.var_xg2x_edge, locals.var_xg2x_edge_dn4, locals.var_xg2x_edge_dn6, locals.var_xg2x_edge_dn7, locals.var_xg2x_edge_dn8, locals.var_xg2x_edge_dn9,)
    }
};
        locals.var_xg2x_edge = assign27850_e29833;
        locals.var_xg2x_edge_dn4 = assign27850_e29833_d_n4;
        locals.var_xg2x_edge_dn6 = assign27850_e29833_d_n6;
        locals.var_xg2x_edge_dn7 = assign27850_e29833_d_n7;
        locals.var_xg2x_edge_dn8 = assign27850_e29833_d_n8;
        locals.var_xg2x_edge_dn9 = assign27850_e29833_d_n9;

        let (assign27860_e29839, assign27860_e29839_d_n4, assign27860_e29839_d_n6, assign27860_e29839_d_n7, assign27860_e29839_d_n8, assign27860_e29839_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27860_e29837: f64 = (locals.var_k1_1d / locals.var_sce1_edge);
        (assign27860_e29837, (-((locals.var_k1_1d * locals.var_sce1_edge_dn4) / (locals.var_sce1_edge * locals.var_sce1_edge))), (-((locals.var_k1_1d * locals.var_sce1_edge_dn6) / (locals.var_sce1_edge * locals.var_sce1_edge))), (-((locals.var_k1_1d * locals.var_sce1_edge_dn7) / (locals.var_sce1_edge * locals.var_sce1_edge))), (-((locals.var_k1_1d * locals.var_sce1_edge_dn8) / (locals.var_sce1_edge * locals.var_sce1_edge))), (-((locals.var_k1_1d * locals.var_sce1_edge_dn9) / (locals.var_sce1_edge * locals.var_sce1_edge))),)
    } else {
        (locals.var_k1_edge, locals.var_k1_edge_dn4, locals.var_k1_edge_dn6, locals.var_k1_edge_dn7, locals.var_k1_edge_dn8, locals.var_k1_edge_dn9,)
    }
};
        locals.var_k1_edge = assign27860_e29839;
        locals.var_k1_edge_dn4 = assign27860_e29839_d_n4;
        locals.var_k1_edge_dn6 = assign27860_e29839_d_n6;
        locals.var_k1_edge_dn7 = assign27860_e29839_d_n7;
        locals.var_k1_edge_dn8 = assign27860_e29839_d_n8;
        locals.var_k1_edge_dn9 = assign27860_e29839_d_n9;

        let (assign27870_e29845, assign27870_e29845_d_n4, assign27870_e29845_d_n6, assign27870_e29845_d_n7, assign27870_e29845_d_n8, assign27870_e29845_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27870_e29843: f64 = (locals.var_k2_1d / locals.var_sce2_edge);
        (assign27870_e29843, (-((locals.var_k2_1d * locals.var_sce2_edge_dn4) / (locals.var_sce2_edge * locals.var_sce2_edge))), (-((locals.var_k2_1d * locals.var_sce2_edge_dn6) / (locals.var_sce2_edge * locals.var_sce2_edge))), (-((locals.var_k2_1d * locals.var_sce2_edge_dn7) / (locals.var_sce2_edge * locals.var_sce2_edge))), (-((locals.var_k2_1d * locals.var_sce2_edge_dn8) / (locals.var_sce2_edge * locals.var_sce2_edge))), (-((locals.var_k2_1d * locals.var_sce2_edge_dn9) / (locals.var_sce2_edge * locals.var_sce2_edge))),)
    } else {
        (locals.var_k2_edge, locals.var_k2_edge_dn4, locals.var_k2_edge_dn6, locals.var_k2_edge_dn7, locals.var_k2_edge_dn8, locals.var_k2_edge_dn9,)
    }
};
        locals.var_k2_edge = assign27870_e29845;
        locals.var_k2_edge_dn4 = assign27870_e29845_d_n4;
        locals.var_k2_edge_dn6 = assign27870_e29845_d_n6;
        locals.var_k2_edge_dn7 = assign27870_e29845_d_n7;
        locals.var_k2_edge_dn8 = assign27870_e29845_d_n8;
        locals.var_k2_edge_dn9 = assign27870_e29845_d_n9;

        let (assign27880_e29851, assign27880_e29851_d_n4, assign27880_e29851_d_n6, assign27880_e29851_d_n7, assign27880_e29851_d_n8, assign27880_e29851_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27880_e29849: f64 = (1.0 / locals.var_k1_edge);
        (assign27880_e29849, (-(locals.var_k1_edge_dn4 / (locals.var_k1_edge * locals.var_k1_edge))), (-(locals.var_k1_edge_dn6 / (locals.var_k1_edge * locals.var_k1_edge))), (-(locals.var_k1_edge_dn7 / (locals.var_k1_edge * locals.var_k1_edge))), (-(locals.var_k1_edge_dn8 / (locals.var_k1_edge * locals.var_k1_edge))), (-(locals.var_k1_edge_dn9 / (locals.var_k1_edge * locals.var_k1_edge))),)
    } else {
        (locals.var_inv_k1_edge, locals.var_inv_k1_edge_dn4, locals.var_inv_k1_edge_dn6, locals.var_inv_k1_edge_dn7, locals.var_inv_k1_edge_dn8, locals.var_inv_k1_edge_dn9,)
    }
};
        locals.var_inv_k1_edge = assign27880_e29851;
        locals.var_inv_k1_edge_dn4 = assign27880_e29851_d_n4;
        locals.var_inv_k1_edge_dn6 = assign27880_e29851_d_n6;
        locals.var_inv_k1_edge_dn7 = assign27880_e29851_d_n7;
        locals.var_inv_k1_edge_dn8 = assign27880_e29851_d_n8;
        locals.var_inv_k1_edge_dn9 = assign27880_e29851_d_n9;

        let (assign27890_e29857, assign27890_e29857_d_n4, assign27890_e29857_d_n6, assign27890_e29857_d_n7, assign27890_e29857_d_n8, assign27890_e29857_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27890_e29855: f64 = (1.0 / locals.var_k2_edge);
        (assign27890_e29855, (-(locals.var_k2_edge_dn4 / (locals.var_k2_edge * locals.var_k2_edge))), (-(locals.var_k2_edge_dn6 / (locals.var_k2_edge * locals.var_k2_edge))), (-(locals.var_k2_edge_dn7 / (locals.var_k2_edge * locals.var_k2_edge))), (-(locals.var_k2_edge_dn8 / (locals.var_k2_edge * locals.var_k2_edge))), (-(locals.var_k2_edge_dn9 / (locals.var_k2_edge * locals.var_k2_edge))),)
    } else {
        (locals.var_inv_k2_edge, locals.var_inv_k2_edge_dn4, locals.var_inv_k2_edge_dn6, locals.var_inv_k2_edge_dn7, locals.var_inv_k2_edge_dn8, locals.var_inv_k2_edge_dn9,)
    }
};
        locals.var_inv_k2_edge = assign27890_e29857;
        locals.var_inv_k2_edge_dn4 = assign27890_e29857_d_n4;
        locals.var_inv_k2_edge_dn6 = assign27890_e29857_d_n6;
        locals.var_inv_k2_edge_dn7 = assign27890_e29857_d_n7;
        locals.var_inv_k2_edge_dn8 = assign27890_e29857_d_n8;
        locals.var_inv_k2_edge_dn9 = assign27890_e29857_d_n9;

        let (assign27900_e29867, assign27900_e29867_d_n4, assign27900_e29867_d_n6, assign27900_e29867_d_n7, assign27900_e29867_d_n8, assign27900_e29867_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27900_e29862: f64 = (1.0 + locals.var_inv_k1_edge);
        let assign27900_e29864: f64 = (assign27900_e29862 + locals.var_inv_k2_edge);
        let assign27900_e29865: f64 = (1.0 / assign27900_e29864);
        (assign27900_e29865, (-((locals.var_inv_k1_edge_dn4 + locals.var_inv_k2_edge_dn4) / (assign27900_e29864 * assign27900_e29864))), (-((locals.var_inv_k1_edge_dn6 + locals.var_inv_k2_edge_dn6) / (assign27900_e29864 * assign27900_e29864))), (-((locals.var_inv_k1_edge_dn7 + locals.var_inv_k2_edge_dn7) / (assign27900_e29864 * assign27900_e29864))), (-((locals.var_inv_k1_edge_dn8 + locals.var_inv_k2_edge_dn8) / (assign27900_e29864 * assign27900_e29864))), (-((locals.var_inv_k1_edge_dn9 + locals.var_inv_k2_edge_dn9) / (assign27900_e29864 * assign27900_e29864))),)
    } else {
        (locals.var_keq_edge, locals.var_keq_edge_dn4, locals.var_keq_edge_dn6, locals.var_keq_edge_dn7, locals.var_keq_edge_dn8, locals.var_keq_edge_dn9,)
    }
};
        locals.var_keq_edge = assign27900_e29867;
        locals.var_keq_edge_dn4 = assign27900_e29867_d_n4;
        locals.var_keq_edge_dn6 = assign27900_e29867_d_n6;
        locals.var_keq_edge_dn7 = assign27900_e29867_d_n7;
        locals.var_keq_edge_dn8 = assign27900_e29867_d_n8;
        locals.var_keq_edge_dn9 = assign27900_e29867_d_n9;

        let (assign27910_e29875, assign27910_e29875_d_n4, assign27910_e29875_d_n6, assign27910_e29875_d_n7, assign27910_e29875_d_n8, assign27910_e29875_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27910_e29872: f64 = (locals.var_csiprime_dc * locals.var_csiprime_dc);
        let assign27910_e29873: f64 = (locals.var_a0_csisq_edge / assign27910_e29872);
        (assign27910_e29873, (((locals.var_a0_csisq_edge_dn4 * assign27910_e29872) - (locals.var_a0_csisq_edge * ((locals.var_csiprime_dc_dn4 * locals.var_csiprime_dc) + (locals.var_csiprime_dc * locals.var_csiprime_dc_dn4)))) / (assign27910_e29872 * assign27910_e29872)), (((locals.var_a0_csisq_edge_dn6 * assign27910_e29872) - (locals.var_a0_csisq_edge * ((locals.var_csiprime_dc_dn6 * locals.var_csiprime_dc) + (locals.var_csiprime_dc * locals.var_csiprime_dc_dn6)))) / (assign27910_e29872 * assign27910_e29872)), (((locals.var_a0_csisq_edge_dn7 * assign27910_e29872) - (locals.var_a0_csisq_edge * ((locals.var_csiprime_dc_dn7 * locals.var_csiprime_dc) + (locals.var_csiprime_dc * locals.var_csiprime_dc_dn7)))) / (assign27910_e29872 * assign27910_e29872)), (((locals.var_a0_csisq_edge_dn8 * assign27910_e29872) - (locals.var_a0_csisq_edge * ((locals.var_csiprime_dc_dn8 * locals.var_csiprime_dc) + (locals.var_csiprime_dc * locals.var_csiprime_dc_dn8)))) / (assign27910_e29872 * assign27910_e29872)), (((locals.var_a0_csisq_edge_dn9 * assign27910_e29872) - (locals.var_a0_csisq_edge * ((locals.var_csiprime_dc_dn9 * locals.var_csiprime_dc) + (locals.var_csiprime_dc * locals.var_csiprime_dc_dn9)))) / (assign27910_e29872 * assign27910_e29872)),)
    } else {
        (locals.var_a0_edge, locals.var_a0_edge_dn4, locals.var_a0_edge_dn6, locals.var_a0_edge_dn7, locals.var_a0_edge_dn8, locals.var_a0_edge_dn9,)
    }
};
        locals.var_a0_edge = assign27910_e29875;
        locals.var_a0_edge_dn4 = assign27910_e29875_d_n4;
        locals.var_a0_edge_dn6 = assign27910_e29875_d_n6;
        locals.var_a0_edge_dn7 = assign27910_e29875_d_n7;
        locals.var_a0_edge_dn8 = assign27910_e29875_d_n8;
        locals.var_a0_edge_dn9 = assign27910_e29875_d_n9;

        let (assign27920_e29883, assign27920_e29883_d_n4, assign27920_e29883_d_n6, assign27920_e29883_d_n7, assign27920_e29883_d_n8, assign27920_e29883_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign27920_e29880: f64 = (locals.var_xg1x_edge - locals.var_xg2x_edge);
        let assign27920_e29881: f64 = (locals.var_keq_edge * assign27920_e29880);
        (assign27920_e29881, ((locals.var_keq_edge_dn4 * assign27920_e29880) + (locals.var_keq_edge * (locals.var_xg1x_edge_dn4 - locals.var_xg2x_edge_dn4))), ((locals.var_keq_edge_dn6 * assign27920_e29880) + (locals.var_keq_edge * (locals.var_xg1x_edge_dn6 - locals.var_xg2x_edge_dn6))), ((locals.var_keq_edge_dn7 * assign27920_e29880) + (locals.var_keq_edge * (locals.var_xg1x_edge_dn7 - locals.var_xg2x_edge_dn7))), ((locals.var_keq_edge_dn8 * assign27920_e29880) + (locals.var_keq_edge * (locals.var_xg1x_edge_dn8 - locals.var_xg2x_edge_dn8))), ((locals.var_keq_edge_dn9 * assign27920_e29880) + (locals.var_keq_edge * (locals.var_xg1x_edge_dn9 - locals.var_xg2x_edge_dn9))),)
    } else {
        (locals.var_dx_wi_edge, locals.var_dx_wi_edge_dn4, locals.var_dx_wi_edge_dn6, locals.var_dx_wi_edge_dn7, locals.var_dx_wi_edge_dn8, locals.var_dx_wi_edge_dn9,)
    }
};
        locals.var_dx_wi_edge = assign27920_e29883;
        locals.var_dx_wi_edge_dn4 = assign27920_e29883_d_n4;
        locals.var_dx_wi_edge_dn6 = assign27920_e29883_d_n6;
        locals.var_dx_wi_edge_dn7 = assign27920_e29883_d_n7;
        locals.var_dx_wi_edge_dn8 = assign27920_e29883_d_n8;
        locals.var_dx_wi_edge_dn9 = assign27920_e29883_d_n9;

        let assign27930_e29886: f64 = (locals.var_xg2x_edge - locals.var_xg1x_edge);
        let assign27930_e29887: f64 = (assign27930_e29886).abs();
        let assign27930_e29889: f64 = if assign27930_e29887 <= 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard783 = assign27930_e29889;

    }

    pub(super) fn stamp_transient_block_73(
        locals: &mut StampLocals,
    ) {
        let (assign27940_e29903, assign27940_e29903_d_n4, assign27940_e29903_d_n6, assign27940_e29903_d_n7, assign27940_e29903_d_n8, assign27940_e29903_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard783 != 0.0)) {
        let assign27940_e29896: f64 = (locals.var_keq_edge * locals.var_inv_k1_edge);
        let assign27940_e29897: f64 = (1.0 - assign27940_e29896);
        let assign27940_e29900: f64 = (locals.var_keq_edge * locals.var_inv_k2_edge);
        let assign27940_e29901: f64 = (assign27940_e29897 - assign27940_e29900);
        (assign27940_e29901, ((-((locals.var_keq_edge_dn4 * locals.var_inv_k1_edge) + (locals.var_keq_edge * locals.var_inv_k1_edge_dn4))) - ((locals.var_keq_edge_dn4 * locals.var_inv_k2_edge) + (locals.var_keq_edge * locals.var_inv_k2_edge_dn4))), ((-((locals.var_keq_edge_dn6 * locals.var_inv_k1_edge) + (locals.var_keq_edge * locals.var_inv_k1_edge_dn6))) - ((locals.var_keq_edge_dn6 * locals.var_inv_k2_edge) + (locals.var_keq_edge * locals.var_inv_k2_edge_dn6))), ((-((locals.var_keq_edge_dn7 * locals.var_inv_k1_edge) + (locals.var_keq_edge * locals.var_inv_k1_edge_dn7))) - ((locals.var_keq_edge_dn7 * locals.var_inv_k2_edge) + (locals.var_keq_edge * locals.var_inv_k2_edge_dn7))), ((-((locals.var_keq_edge_dn8 * locals.var_inv_k1_edge) + (locals.var_keq_edge * locals.var_inv_k1_edge_dn8))) - ((locals.var_keq_edge_dn8 * locals.var_inv_k2_edge) + (locals.var_keq_edge * locals.var_inv_k2_edge_dn8))), ((-((locals.var_keq_edge_dn9 * locals.var_inv_k1_edge) + (locals.var_keq_edge * locals.var_inv_k1_edge_dn9))) - ((locals.var_keq_edge_dn9 * locals.var_inv_k2_edge) + (locals.var_keq_edge * locals.var_inv_k2_edge_dn9))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign27940_e29903;
        locals.var_temp1_dn4 = assign27940_e29903_d_n4;
        locals.var_temp1_dn6 = assign27940_e29903_d_n6;
        locals.var_temp1_dn7 = assign27940_e29903_d_n7;
        locals.var_temp1_dn8 = assign27940_e29903_d_n8;
        locals.var_temp1_dn9 = assign27940_e29903_d_n9;

        let (assign27950_e29931, assign27950_e29931_d_n4, assign27950_e29931_d_n6, assign27950_e29931_d_n7, assign27950_e29931_d_n8, assign27950_e29931_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard783 != 0.0)) {
        let assign27950_e29910: f64 = (0.5 * locals.var_inv_k1_edge);
        let assign27950_e29912: f64 = (assign27950_e29910 * locals.var_keq_edge);
        let assign27950_e29914: f64 = (assign27950_e29912 * locals.var_inv_k1_edge);
        let assign27950_e29915: f64 = (locals.var_inv_k2_edge + assign27950_e29914);
        let assign27950_e29918: f64 = (0.5 * locals.var_inv_k2_edge);
        let assign27950_e29920: f64 = (assign27950_e29918 * locals.var_keq_edge);
        let assign27950_e29922: f64 = (assign27950_e29920 * locals.var_inv_k2_edge);
        let assign27950_e29923: f64 = (assign27950_e29915 - assign27950_e29922);
        let assign27950_e29926: f64 = (0.5 / locals.var_keq_edge);
        let assign27950_e29927: f64 = (assign27950_e29923 - assign27950_e29926);
        let assign27950_e29929: f64 = (assign27950_e29927 * locals.var_dx_wi_edge);
        (assign27950_e29929, (((((locals.var_inv_k2_edge_dn4 + (((((0.5 * locals.var_inv_k1_edge_dn4) * locals.var_keq_edge) + (assign27950_e29910 * locals.var_keq_edge_dn4)) * locals.var_inv_k1_edge) + (assign27950_e29912 * locals.var_inv_k1_edge_dn4))) - (((((0.5 * locals.var_inv_k2_edge_dn4) * locals.var_keq_edge) + (assign27950_e29918 * locals.var_keq_edge_dn4)) * locals.var_inv_k2_edge) + (assign27950_e29920 * locals.var_inv_k2_edge_dn4))) - (-((0.5 * locals.var_keq_edge_dn4) / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27950_e29927 * locals.var_dx_wi_edge_dn4)), (((((locals.var_inv_k2_edge_dn6 + (((((0.5 * locals.var_inv_k1_edge_dn6) * locals.var_keq_edge) + (assign27950_e29910 * locals.var_keq_edge_dn6)) * locals.var_inv_k1_edge) + (assign27950_e29912 * locals.var_inv_k1_edge_dn6))) - (((((0.5 * locals.var_inv_k2_edge_dn6) * locals.var_keq_edge) + (assign27950_e29918 * locals.var_keq_edge_dn6)) * locals.var_inv_k2_edge) + (assign27950_e29920 * locals.var_inv_k2_edge_dn6))) - (-((0.5 * locals.var_keq_edge_dn6) / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27950_e29927 * locals.var_dx_wi_edge_dn6)), (((((locals.var_inv_k2_edge_dn7 + (((((0.5 * locals.var_inv_k1_edge_dn7) * locals.var_keq_edge) + (assign27950_e29910 * locals.var_keq_edge_dn7)) * locals.var_inv_k1_edge) + (assign27950_e29912 * locals.var_inv_k1_edge_dn7))) - (((((0.5 * locals.var_inv_k2_edge_dn7) * locals.var_keq_edge) + (assign27950_e29918 * locals.var_keq_edge_dn7)) * locals.var_inv_k2_edge) + (assign27950_e29920 * locals.var_inv_k2_edge_dn7))) - (-((0.5 * locals.var_keq_edge_dn7) / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27950_e29927 * locals.var_dx_wi_edge_dn7)), (((((locals.var_inv_k2_edge_dn8 + (((((0.5 * locals.var_inv_k1_edge_dn8) * locals.var_keq_edge) + (assign27950_e29910 * locals.var_keq_edge_dn8)) * locals.var_inv_k1_edge) + (assign27950_e29912 * locals.var_inv_k1_edge_dn8))) - (((((0.5 * locals.var_inv_k2_edge_dn8) * locals.var_keq_edge) + (assign27950_e29918 * locals.var_keq_edge_dn8)) * locals.var_inv_k2_edge) + (assign27950_e29920 * locals.var_inv_k2_edge_dn8))) - (-((0.5 * locals.var_keq_edge_dn8) / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27950_e29927 * locals.var_dx_wi_edge_dn8)), (((((locals.var_inv_k2_edge_dn9 + (((((0.5 * locals.var_inv_k1_edge_dn9) * locals.var_keq_edge) + (assign27950_e29910 * locals.var_keq_edge_dn9)) * locals.var_inv_k1_edge) + (assign27950_e29912 * locals.var_inv_k1_edge_dn9))) - (((((0.5 * locals.var_inv_k2_edge_dn9) * locals.var_keq_edge) + (assign27950_e29918 * locals.var_keq_edge_dn9)) * locals.var_inv_k2_edge) + (assign27950_e29920 * locals.var_inv_k2_edge_dn9))) - (-((0.5 * locals.var_keq_edge_dn9) / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27950_e29927 * locals.var_dx_wi_edge_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27950_e29931;
        locals.var_temp2_dn4 = assign27950_e29931_d_n4;
        locals.var_temp2_dn6 = assign27950_e29931_d_n6;
        locals.var_temp2_dn7 = assign27950_e29931_d_n7;
        locals.var_temp2_dn8 = assign27950_e29931_d_n8;
        locals.var_temp2_dn9 = assign27950_e29931_d_n9;

        let (assign27960_e29945, assign27960_e29945_d_n4, assign27960_e29945_d_n6, assign27960_e29945_d_n7, assign27960_e29945_d_n8, assign27960_e29945_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard783 != 0.0)) {
        let assign27960_e29938: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign27960_e29939: f64 = (0.5 * assign27960_e29938);
        let assign27960_e29941: f64 = (assign27960_e29939 * locals.var_a0_edge);
        let assign27960_e29943: f64 = (assign27960_e29941 / locals.var_keq_edge);
        (assign27960_e29943, ((((((0.5 * (locals.var_temp1_dn4 - locals.var_temp2_dn4)) * locals.var_a0_edge) + (assign27960_e29939 * locals.var_a0_edge_dn4)) * locals.var_keq_edge) - (assign27960_e29941 * locals.var_keq_edge_dn4)) / (locals.var_keq_edge * locals.var_keq_edge)), ((((((0.5 * (locals.var_temp1_dn6 - locals.var_temp2_dn6)) * locals.var_a0_edge) + (assign27960_e29939 * locals.var_a0_edge_dn6)) * locals.var_keq_edge) - (assign27960_e29941 * locals.var_keq_edge_dn6)) / (locals.var_keq_edge * locals.var_keq_edge)), ((((((0.5 * (locals.var_temp1_dn7 - locals.var_temp2_dn7)) * locals.var_a0_edge) + (assign27960_e29939 * locals.var_a0_edge_dn7)) * locals.var_keq_edge) - (assign27960_e29941 * locals.var_keq_edge_dn7)) / (locals.var_keq_edge * locals.var_keq_edge)), ((((((0.5 * (locals.var_temp1_dn8 - locals.var_temp2_dn8)) * locals.var_a0_edge) + (assign27960_e29939 * locals.var_a0_edge_dn8)) * locals.var_keq_edge) - (assign27960_e29941 * locals.var_keq_edge_dn8)) / (locals.var_keq_edge * locals.var_keq_edge)), ((((((0.5 * (locals.var_temp1_dn9 - locals.var_temp2_dn9)) * locals.var_a0_edge) + (assign27960_e29939 * locals.var_a0_edge_dn9)) * locals.var_keq_edge) - (assign27960_e29941 * locals.var_keq_edge_dn9)) / (locals.var_keq_edge * locals.var_keq_edge)),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign27960_e29945;
        locals.var_temp3_dn4 = assign27960_e29945_d_n4;
        locals.var_temp3_dn6 = assign27960_e29945_d_n6;
        locals.var_temp3_dn7 = assign27960_e29945_d_n7;
        locals.var_temp3_dn8 = assign27960_e29945_d_n8;
        locals.var_temp3_dn9 = assign27960_e29945_d_n9;

        let (assign27970_e29956, assign27970_e29956_d_n4, assign27970_e29956_d_n6, assign27970_e29956_d_n7, assign27970_e29956_d_n8, assign27970_e29956_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard783 == 0.0)) {
        let assign27970_e29951: f64 = (-locals.var_inv_k1_edge);
        let assign27970_e29953: f64 = (assign27970_e29951 * locals.var_dx_wi_edge);
        let assign27970_e29954: f64 = (assign27970_e29953).exp();
        (assign27970_e29954, (assign27970_e29954 * (((-locals.var_inv_k1_edge_dn4) * locals.var_dx_wi_edge) + (assign27970_e29951 * locals.var_dx_wi_edge_dn4))), (assign27970_e29954 * (((-locals.var_inv_k1_edge_dn6) * locals.var_dx_wi_edge) + (assign27970_e29951 * locals.var_dx_wi_edge_dn6))), (assign27970_e29954 * (((-locals.var_inv_k1_edge_dn7) * locals.var_dx_wi_edge) + (assign27970_e29951 * locals.var_dx_wi_edge_dn7))), (assign27970_e29954 * (((-locals.var_inv_k1_edge_dn8) * locals.var_dx_wi_edge) + (assign27970_e29951 * locals.var_dx_wi_edge_dn8))), (assign27970_e29954 * (((-locals.var_inv_k1_edge_dn9) * locals.var_dx_wi_edge) + (assign27970_e29951 * locals.var_dx_wi_edge_dn9))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign27970_e29956;
        locals.var_temp1_dn4 = assign27970_e29956_d_n4;
        locals.var_temp1_dn6 = assign27970_e29956_d_n6;
        locals.var_temp1_dn7 = assign27970_e29956_d_n7;
        locals.var_temp1_dn8 = assign27970_e29956_d_n8;
        locals.var_temp1_dn9 = assign27970_e29956_d_n9;

        let (assign27980_e29970, assign27980_e29970_d_n4, assign27980_e29970_d_n6, assign27980_e29970_d_n7, assign27980_e29970_d_n8, assign27980_e29970_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard783 == 0.0)) {
        let assign27980_e29964: f64 = (1.0 / locals.var_keq_edge);
        let assign27980_e29965: f64 = (locals.var_inv_k2_edge - assign27980_e29964);
        let assign27980_e29967: f64 = (assign27980_e29965 * locals.var_dx_wi_edge);
        let assign27980_e29968: f64 = (assign27980_e29967).exp();
        (assign27980_e29968, (assign27980_e29968 * (((locals.var_inv_k2_edge_dn4 - (-(locals.var_keq_edge_dn4 / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27980_e29965 * locals.var_dx_wi_edge_dn4))), (assign27980_e29968 * (((locals.var_inv_k2_edge_dn6 - (-(locals.var_keq_edge_dn6 / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27980_e29965 * locals.var_dx_wi_edge_dn6))), (assign27980_e29968 * (((locals.var_inv_k2_edge_dn7 - (-(locals.var_keq_edge_dn7 / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27980_e29965 * locals.var_dx_wi_edge_dn7))), (assign27980_e29968 * (((locals.var_inv_k2_edge_dn8 - (-(locals.var_keq_edge_dn8 / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27980_e29965 * locals.var_dx_wi_edge_dn8))), (assign27980_e29968 * (((locals.var_inv_k2_edge_dn9 - (-(locals.var_keq_edge_dn9 / (locals.var_keq_edge * locals.var_keq_edge)))) * locals.var_dx_wi_edge) + (assign27980_e29965 * locals.var_dx_wi_edge_dn9))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign27980_e29970;
        locals.var_temp2_dn4 = assign27980_e29970_d_n4;
        locals.var_temp2_dn6 = assign27980_e29970_d_n6;
        locals.var_temp2_dn7 = assign27980_e29970_d_n7;
        locals.var_temp2_dn8 = assign27980_e29970_d_n8;
        locals.var_temp2_dn9 = assign27980_e29970_d_n9;

        let (assign27990_e29985, assign27990_e29985_d_n4, assign27990_e29985_d_n6, assign27990_e29985_d_n7, assign27990_e29985_d_n8, assign27990_e29985_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard783 == 0.0)) {
        let assign27990_e29978: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign27990_e29979: f64 = (locals.var_a0_edge * assign27990_e29978);
        let assign27990_e29982: f64 = (2.0 * locals.var_dx_wi_edge);
        let assign27990_e29983: f64 = (assign27990_e29979 / assign27990_e29982);
        (assign27990_e29983, (((((locals.var_a0_edge_dn4 * assign27990_e29978) + (locals.var_a0_edge * (locals.var_temp1_dn4 - locals.var_temp2_dn4))) * assign27990_e29982) - (assign27990_e29979 * (2.0 * locals.var_dx_wi_edge_dn4))) / (assign27990_e29982 * assign27990_e29982)), (((((locals.var_a0_edge_dn6 * assign27990_e29978) + (locals.var_a0_edge * (locals.var_temp1_dn6 - locals.var_temp2_dn6))) * assign27990_e29982) - (assign27990_e29979 * (2.0 * locals.var_dx_wi_edge_dn6))) / (assign27990_e29982 * assign27990_e29982)), (((((locals.var_a0_edge_dn7 * assign27990_e29978) + (locals.var_a0_edge * (locals.var_temp1_dn7 - locals.var_temp2_dn7))) * assign27990_e29982) - (assign27990_e29979 * (2.0 * locals.var_dx_wi_edge_dn7))) / (assign27990_e29982 * assign27990_e29982)), (((((locals.var_a0_edge_dn8 * assign27990_e29978) + (locals.var_a0_edge * (locals.var_temp1_dn8 - locals.var_temp2_dn8))) * assign27990_e29982) - (assign27990_e29979 * (2.0 * locals.var_dx_wi_edge_dn8))) / (assign27990_e29982 * assign27990_e29982)), (((((locals.var_a0_edge_dn9 * assign27990_e29978) + (locals.var_a0_edge * (locals.var_temp1_dn9 - locals.var_temp2_dn9))) * assign27990_e29982) - (assign27990_e29979 * (2.0 * locals.var_dx_wi_edge_dn9))) / (assign27990_e29982 * assign27990_e29982)),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign27990_e29985;
        locals.var_temp3_dn4 = assign27990_e29985_d_n4;
        locals.var_temp3_dn6 = assign27990_e29985_d_n6;
        locals.var_temp3_dn7 = assign27990_e29985_d_n7;
        locals.var_temp3_dn8 = assign27990_e29985_d_n8;
        locals.var_temp3_dn9 = assign27990_e29985_d_n9;

        let (assign28000_e29989, assign28000_e29989_d_n4, assign28000_e29989_d_n6, assign28000_e29989_d_n7, assign28000_e29989_d_n8, assign28000_e29989_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    } else {
        (locals.var_prefac_qilow_edge, locals.var_prefac_qilow_edge_dn4, locals.var_prefac_qilow_edge_dn6, locals.var_prefac_qilow_edge_dn7, locals.var_prefac_qilow_edge_dn8, locals.var_prefac_qilow_edge_dn9,)
    }
};
        locals.var_prefac_qilow_edge = assign28000_e29989;
        locals.var_prefac_qilow_edge_dn4 = assign28000_e29989_d_n4;
        locals.var_prefac_qilow_edge_dn6 = assign28000_e29989_d_n6;
        locals.var_prefac_qilow_edge_dn7 = assign28000_e29989_d_n7;
        locals.var_prefac_qilow_edge_dn8 = assign28000_e29989_d_n8;
        locals.var_prefac_qilow_edge_dn9 = assign28000_e29989_d_n9;

        let assign28010_e29992: f64 = if locals.var_xg1x_edge < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard784 = assign28010_e29992;

        let (assign28020_e30004, assign28020_e30004_d_n4, assign28020_e30004_d_n6, assign28020_e30004_d_n7, assign28020_e30004_d_n8, assign28020_e30004_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard784 != 0.0)) {
        let assign28020_e29999: f64 = (locals.var_xg1x_edge).exp();
        let assign28020_e30000: f64 = (locals.var_prefac_qilow_edge * assign28020_e29999);
        let assign28020_e30001: f64 = (1.0 + assign28020_e30000);
        let assign28020_e30002: f64 = (assign28020_e30001).ln();
        (assign28020_e30002, (((locals.var_prefac_qilow_edge_dn4 * assign28020_e29999) + (locals.var_prefac_qilow_edge * (assign28020_e29999 * locals.var_xg1x_edge_dn4))) / assign28020_e30001), (((locals.var_prefac_qilow_edge_dn6 * assign28020_e29999) + (locals.var_prefac_qilow_edge * (assign28020_e29999 * locals.var_xg1x_edge_dn6))) / assign28020_e30001), (((locals.var_prefac_qilow_edge_dn7 * assign28020_e29999) + (locals.var_prefac_qilow_edge * (assign28020_e29999 * locals.var_xg1x_edge_dn7))) / assign28020_e30001), (((locals.var_prefac_qilow_edge_dn8 * assign28020_e29999) + (locals.var_prefac_qilow_edge * (assign28020_e29999 * locals.var_xg1x_edge_dn8))) / assign28020_e30001), (((locals.var_prefac_qilow_edge_dn9 * assign28020_e29999) + (locals.var_prefac_qilow_edge * (assign28020_e29999 * locals.var_xg1x_edge_dn9))) / assign28020_e30001),)
    } else {
        (locals.var_w_temp, locals.var_w_temp_dn4, locals.var_w_temp_dn6, locals.var_w_temp_dn7, locals.var_w_temp_dn8, locals.var_w_temp_dn9,)
    }
};
        locals.var_w_temp = assign28020_e30004;
        locals.var_w_temp_dn4 = assign28020_e30004_d_n4;
        locals.var_w_temp_dn6 = assign28020_e30004_d_n6;
        locals.var_w_temp_dn7 = assign28020_e30004_d_n7;
        locals.var_w_temp_dn8 = assign28020_e30004_d_n8;
        locals.var_w_temp_dn9 = assign28020_e30004_d_n9;

        let (assign28030_e30021, assign28030_e30021_d_n4, assign28030_e30021_d_n6, assign28030_e30021_d_n7, assign28030_e30021_d_n8, assign28030_e30021_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard784 != 0.0)) {
        let assign28030_e30012: f64 = (1.0 + locals.var_w_temp);
        let assign28030_e30013: f64 = (assign28030_e30012).ln();
        let assign28030_e30016: f64 = (2.0 + locals.var_w_temp);
        let assign28030_e30017: f64 = (assign28030_e30013 / assign28030_e30016);
        let assign28030_e30018: f64 = (1.0 - assign28030_e30017);
        let assign28030_e30019: f64 = (locals.var_w_temp * assign28030_e30018);
        (assign28030_e30019, ((locals.var_w_temp_dn4 * assign28030_e30018) + (locals.var_w_temp * (-((((locals.var_w_temp_dn4 / assign28030_e30012) * assign28030_e30016) - (assign28030_e30013 * locals.var_w_temp_dn4)) / (assign28030_e30016 * assign28030_e30016))))), ((locals.var_w_temp_dn6 * assign28030_e30018) + (locals.var_w_temp * (-((((locals.var_w_temp_dn6 / assign28030_e30012) * assign28030_e30016) - (assign28030_e30013 * locals.var_w_temp_dn6)) / (assign28030_e30016 * assign28030_e30016))))), ((locals.var_w_temp_dn7 * assign28030_e30018) + (locals.var_w_temp * (-((((locals.var_w_temp_dn7 / assign28030_e30012) * assign28030_e30016) - (assign28030_e30013 * locals.var_w_temp_dn7)) / (assign28030_e30016 * assign28030_e30016))))), ((locals.var_w_temp_dn8 * assign28030_e30018) + (locals.var_w_temp * (-((((locals.var_w_temp_dn8 / assign28030_e30012) * assign28030_e30016) - (assign28030_e30013 * locals.var_w_temp_dn8)) / (assign28030_e30016 * assign28030_e30016))))), ((locals.var_w_temp_dn9 * assign28030_e30018) + (locals.var_w_temp * (-((((locals.var_w_temp_dn9 / assign28030_e30012) * assign28030_e30016) - (assign28030_e30013 * locals.var_w_temp_dn9)) / (assign28030_e30016 * assign28030_e30016))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28030_e30021;
        locals.var_temp_dn4 = assign28030_e30021_d_n4;
        locals.var_temp_dn6 = assign28030_e30021_d_n6;
        locals.var_temp_dn7 = assign28030_e30021_d_n7;
        locals.var_temp_dn8 = assign28030_e30021_d_n8;
        locals.var_temp_dn9 = assign28030_e30021_d_n9;

        let assign28040_e30024: f64 = if locals.var_xg1x_edge < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard785 = assign28040_e30024;

        let assign28050_e30027: f64 = (-80.0);
        let assign28050_e30028: f64 = if locals.var_xg1x_edge > assign28050_e30027 { 1.0 } else { 0.0 };
        locals.var_guard786 = assign28050_e30028;

        let (assign28060_e30040, assign28060_e30040_d_n4, assign28060_e30040_d_n6, assign28060_e30040_d_n7, assign28060_e30040_d_n8, assign28060_e30040_d_n9,) = {
    if ((((locals.var_guard782 != 0.0) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 != 0.0)) && (locals.var_guard786 != 0.0)) {
        let assign28060_e30038: f64 = (locals.var_xg1x_edge).exp();
        (assign28060_e30038, (assign28060_e30038 * locals.var_xg1x_edge_dn4), (assign28060_e30038 * locals.var_xg1x_edge_dn6), (assign28060_e30038 * locals.var_xg1x_edge_dn7), (assign28060_e30038 * locals.var_xg1x_edge_dn8), (assign28060_e30038 * locals.var_xg1x_edge_dn9),)
    } else {
        (locals.var_w_temp, locals.var_w_temp_dn4, locals.var_w_temp_dn6, locals.var_w_temp_dn7, locals.var_w_temp_dn8, locals.var_w_temp_dn9,)
    }
};
        locals.var_w_temp = assign28060_e30040;
        locals.var_w_temp_dn4 = assign28060_e30040_d_n4;
        locals.var_w_temp_dn6 = assign28060_e30040_d_n6;
        locals.var_w_temp_dn7 = assign28060_e30040_d_n7;
        locals.var_w_temp_dn8 = assign28060_e30040_d_n8;
        locals.var_w_temp_dn9 = assign28060_e30040_d_n9;

        let (assign28070_e30077, assign28070_e30077_d_n4, assign28070_e30077_d_n6, assign28070_e30077_d_n7, assign28070_e30077_d_n8, assign28070_e30077_d_n9,) = {
    if ((((locals.var_guard782 != 0.0) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 != 0.0)) && (locals.var_guard786 == 0.0)) {
        let assign28070_e30053: f64 = (-locals.var_xg1x_edge);
        let assign28070_e30055: f64 = (assign28070_e30053 - 80.0);
        let assign28070_e30059: f64 = (-locals.var_xg1x_edge);
        let assign28070_e30061: f64 = (assign28070_e30059 - 80.0);
        let assign28070_e30062: f64 = (0.5 * assign28070_e30061);
        let assign28070_e30065: f64 = (-locals.var_xg1x_edge);
        let assign28070_e30067: f64 = (assign28070_e30065 - 80.0);
        let assign28070_e30069: f64 = (assign28070_e30067 * 0.3333333333333);
        let assign28070_e30070: f64 = (1.0 + assign28070_e30069);
        let assign28070_e30071: f64 = (assign28070_e30062 * assign28070_e30070);
        let assign28070_e30072: f64 = (1.0 + assign28070_e30071);
        let assign28070_e30073: f64 = (assign28070_e30055 * assign28070_e30072);
        let assign28070_e30074: f64 = (1.0 + assign28070_e30073);
        let assign28070_e30075: f64 = (1.80485e-35 / assign28070_e30074);
        (assign28070_e30075, (-((1.80485e-35 * (((-locals.var_xg1x_edge_dn4) * assign28070_e30072) + (assign28070_e30055 * (((0.5 * (-locals.var_xg1x_edge_dn4)) * assign28070_e30070) + (assign28070_e30062 * ((-locals.var_xg1x_edge_dn4) * 0.3333333333333)))))) / (assign28070_e30074 * assign28070_e30074))), (-((1.80485e-35 * (((-locals.var_xg1x_edge_dn6) * assign28070_e30072) + (assign28070_e30055 * (((0.5 * (-locals.var_xg1x_edge_dn6)) * assign28070_e30070) + (assign28070_e30062 * ((-locals.var_xg1x_edge_dn6) * 0.3333333333333)))))) / (assign28070_e30074 * assign28070_e30074))), (-((1.80485e-35 * (((-locals.var_xg1x_edge_dn7) * assign28070_e30072) + (assign28070_e30055 * (((0.5 * (-locals.var_xg1x_edge_dn7)) * assign28070_e30070) + (assign28070_e30062 * ((-locals.var_xg1x_edge_dn7) * 0.3333333333333)))))) / (assign28070_e30074 * assign28070_e30074))), (-((1.80485e-35 * (((-locals.var_xg1x_edge_dn8) * assign28070_e30072) + (assign28070_e30055 * (((0.5 * (-locals.var_xg1x_edge_dn8)) * assign28070_e30070) + (assign28070_e30062 * ((-locals.var_xg1x_edge_dn8) * 0.3333333333333)))))) / (assign28070_e30074 * assign28070_e30074))), (-((1.80485e-35 * (((-locals.var_xg1x_edge_dn9) * assign28070_e30072) + (assign28070_e30055 * (((0.5 * (-locals.var_xg1x_edge_dn9)) * assign28070_e30070) + (assign28070_e30062 * ((-locals.var_xg1x_edge_dn9) * 0.3333333333333)))))) / (assign28070_e30074 * assign28070_e30074))),)
    } else {
        (locals.var_w_temp, locals.var_w_temp_dn4, locals.var_w_temp_dn6, locals.var_w_temp_dn7, locals.var_w_temp_dn8, locals.var_w_temp_dn9,)
    }
};
        locals.var_w_temp = assign28070_e30077;
        locals.var_w_temp_dn4 = assign28070_e30077_d_n4;
        locals.var_w_temp_dn6 = assign28070_e30077_d_n6;
        locals.var_w_temp_dn7 = assign28070_e30077_d_n7;
        locals.var_w_temp_dn8 = assign28070_e30077_d_n8;
        locals.var_w_temp_dn9 = assign28070_e30077_d_n9;

        let (assign28080_e30088, assign28080_e30088_d_n4, assign28080_e30088_d_n6, assign28080_e30088_d_n7, assign28080_e30088_d_n8, assign28080_e30088_d_n9,) = {
    if (((locals.var_guard782 != 0.0) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 != 0.0)) {
        let assign28080_e30086: f64 = (locals.var_prefac_qilow_edge * locals.var_w_temp);
        (assign28080_e30086, ((locals.var_prefac_qilow_edge_dn4 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn4)), ((locals.var_prefac_qilow_edge_dn6 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn6)), ((locals.var_prefac_qilow_edge_dn7 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn7)), ((locals.var_prefac_qilow_edge_dn8 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn8)), ((locals.var_prefac_qilow_edge_dn9 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28080_e30088;
        locals.var_temp_dn4 = assign28080_e30088_d_n4;
        locals.var_temp_dn6 = assign28080_e30088_d_n6;
        locals.var_temp_dn7 = assign28080_e30088_d_n7;
        locals.var_temp_dn8 = assign28080_e30088_d_n8;
        locals.var_temp_dn9 = assign28080_e30088_d_n9;

        let (assign28090_e30101, assign28090_e30101_d_n4, assign28090_e30101_d_n6, assign28090_e30101_d_n7, assign28090_e30101_d_n8, assign28090_e30101_d_n9,) = {
    if (((locals.var_guard782 != 0.0) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 == 0.0)) {
        let assign28090_e30097: f64 = (locals.var_prefac_qilow_edge).ln();
        let assign28090_e30099: f64 = (assign28090_e30097 + locals.var_xg1x_edge);
        (assign28090_e30099, ((locals.var_prefac_qilow_edge_dn4 / locals.var_prefac_qilow_edge) + locals.var_xg1x_edge_dn4), ((locals.var_prefac_qilow_edge_dn6 / locals.var_prefac_qilow_edge) + locals.var_xg1x_edge_dn6), ((locals.var_prefac_qilow_edge_dn7 / locals.var_prefac_qilow_edge) + locals.var_xg1x_edge_dn7), ((locals.var_prefac_qilow_edge_dn8 / locals.var_prefac_qilow_edge) + locals.var_xg1x_edge_dn8), ((locals.var_prefac_qilow_edge_dn9 / locals.var_prefac_qilow_edge) + locals.var_xg1x_edge_dn9),)
    } else {
        (locals.var_w_temp, locals.var_w_temp_dn4, locals.var_w_temp_dn6, locals.var_w_temp_dn7, locals.var_w_temp_dn8, locals.var_w_temp_dn9,)
    }
};
        locals.var_w_temp = assign28090_e30101;
        locals.var_w_temp_dn4 = assign28090_e30101_d_n4;
        locals.var_w_temp_dn6 = assign28090_e30101_d_n6;
        locals.var_w_temp_dn7 = assign28090_e30101_d_n7;
        locals.var_w_temp_dn8 = assign28090_e30101_d_n8;
        locals.var_w_temp_dn9 = assign28090_e30101_d_n9;

        let (assign28100_e30122, assign28100_e30122_d_n4, assign28100_e30122_d_n6, assign28100_e30122_d_n7, assign28100_e30122_d_n8, assign28100_e30122_d_n9,) = {
    if (((locals.var_guard782 != 0.0) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 == 0.0)) {
        let assign28100_e30113: f64 = (1.0 + locals.var_w_temp);
        let assign28100_e30114: f64 = (assign28100_e30113).ln();
        let assign28100_e30117: f64 = (2.0 + locals.var_w_temp);
        let assign28100_e30118: f64 = (assign28100_e30114 / assign28100_e30117);
        let assign28100_e30119: f64 = (1.0 - assign28100_e30118);
        let assign28100_e30120: f64 = (locals.var_w_temp * assign28100_e30119);
        (assign28100_e30120, ((locals.var_w_temp_dn4 * assign28100_e30119) + (locals.var_w_temp * (-((((locals.var_w_temp_dn4 / assign28100_e30113) * assign28100_e30117) - (assign28100_e30114 * locals.var_w_temp_dn4)) / (assign28100_e30117 * assign28100_e30117))))), ((locals.var_w_temp_dn6 * assign28100_e30119) + (locals.var_w_temp * (-((((locals.var_w_temp_dn6 / assign28100_e30113) * assign28100_e30117) - (assign28100_e30114 * locals.var_w_temp_dn6)) / (assign28100_e30117 * assign28100_e30117))))), ((locals.var_w_temp_dn7 * assign28100_e30119) + (locals.var_w_temp * (-((((locals.var_w_temp_dn7 / assign28100_e30113) * assign28100_e30117) - (assign28100_e30114 * locals.var_w_temp_dn7)) / (assign28100_e30117 * assign28100_e30117))))), ((locals.var_w_temp_dn8 * assign28100_e30119) + (locals.var_w_temp * (-((((locals.var_w_temp_dn8 / assign28100_e30113) * assign28100_e30117) - (assign28100_e30114 * locals.var_w_temp_dn8)) / (assign28100_e30117 * assign28100_e30117))))), ((locals.var_w_temp_dn9 * assign28100_e30119) + (locals.var_w_temp * (-((((locals.var_w_temp_dn9 / assign28100_e30113) * assign28100_e30117) - (assign28100_e30114 * locals.var_w_temp_dn9)) / (assign28100_e30117 * assign28100_e30117))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28100_e30122;
        locals.var_temp_dn4 = assign28100_e30122_d_n4;
        locals.var_temp_dn6 = assign28100_e30122_d_n6;
        locals.var_temp_dn7 = assign28100_e30122_d_n7;
        locals.var_temp_dn8 = assign28100_e30122_d_n8;
        locals.var_temp_dn9 = assign28100_e30122_d_n9;

        let (assign28110_e30126, assign28110_e30126_d_n4, assign28110_e30126_d_n6, assign28110_e30126_d_n7, assign28110_e30126_d_n8, assign28110_e30126_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    } else {
        (locals.var_qis_edge, locals.var_qis_edge_dn4, locals.var_qis_edge_dn6, locals.var_qis_edge_dn7, locals.var_qis_edge_dn8, locals.var_qis_edge_dn9,)
    }
};
        locals.var_qis_edge = assign28110_e30126;
        locals.var_qis_edge_dn4 = assign28110_e30126_d_n4;
        locals.var_qis_edge_dn6 = assign28110_e30126_d_n6;
        locals.var_qis_edge_dn7 = assign28110_e30126_d_n7;
        locals.var_qis_edge_dn8 = assign28110_e30126_d_n8;
        locals.var_qis_edge_dn9 = assign28110_e30126_d_n9;

        let assign28120_e30129: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28120_e30131: f64 = if assign28120_e30129 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard787 = assign28120_e30131;

        let (assign28130_e30145, assign28130_e30145_d_n4, assign28130_e30145_d_n6, assign28130_e30145_d_n7, assign28130_e30145_d_n8, assign28130_e30145_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard787 != 0.0)) {
        let assign28130_e30139: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28130_e30140: f64 = (assign28130_e30139).exp();
        let assign28130_e30141: f64 = (locals.var_prefac_qilow_edge * assign28130_e30140);
        let assign28130_e30142: f64 = (1.0 + assign28130_e30141);
        let assign28130_e30143: f64 = (assign28130_e30142).ln();
        (assign28130_e30143, (((locals.var_prefac_qilow_edge_dn4 * assign28130_e30140) + (locals.var_prefac_qilow_edge * (assign28130_e30140 * (locals.var_xg1x_edge_dn4 - locals.var_xdeff_dc_dn4)))) / assign28130_e30142), (((locals.var_prefac_qilow_edge_dn6 * assign28130_e30140) + (locals.var_prefac_qilow_edge * (assign28130_e30140 * (locals.var_xg1x_edge_dn6 - locals.var_xdeff_dc_dn6)))) / assign28130_e30142), (((locals.var_prefac_qilow_edge_dn7 * assign28130_e30140) + (locals.var_prefac_qilow_edge * (assign28130_e30140 * (locals.var_xg1x_edge_dn7 - locals.var_xdeff_dc_dn7)))) / assign28130_e30142), (((locals.var_prefac_qilow_edge_dn8 * assign28130_e30140) + (locals.var_prefac_qilow_edge * (assign28130_e30140 * (locals.var_xg1x_edge_dn8 - locals.var_xdeff_dc_dn8)))) / assign28130_e30142), (((locals.var_prefac_qilow_edge_dn9 * assign28130_e30140) + (locals.var_prefac_qilow_edge * (assign28130_e30140 * (locals.var_xg1x_edge_dn9 - locals.var_xdeff_dc_dn9)))) / assign28130_e30142),)
    } else {
        (locals.var_w_temp, locals.var_w_temp_dn4, locals.var_w_temp_dn6, locals.var_w_temp_dn7, locals.var_w_temp_dn8, locals.var_w_temp_dn9,)
    }
};
        locals.var_w_temp = assign28130_e30145;
        locals.var_w_temp_dn4 = assign28130_e30145_d_n4;
        locals.var_w_temp_dn6 = assign28130_e30145_d_n6;
        locals.var_w_temp_dn7 = assign28130_e30145_d_n7;
        locals.var_w_temp_dn8 = assign28130_e30145_d_n8;
        locals.var_w_temp_dn9 = assign28130_e30145_d_n9;

        let (assign28140_e30162, assign28140_e30162_d_n4, assign28140_e30162_d_n6, assign28140_e30162_d_n7, assign28140_e30162_d_n8, assign28140_e30162_d_n9,) = {
    if ((locals.var_guard782 != 0.0) && (locals.var_guard787 != 0.0)) {
        let assign28140_e30153: f64 = (1.0 + locals.var_w_temp);
        let assign28140_e30154: f64 = (assign28140_e30153).ln();
        let assign28140_e30157: f64 = (2.0 + locals.var_w_temp);
        let assign28140_e30158: f64 = (assign28140_e30154 / assign28140_e30157);
        let assign28140_e30159: f64 = (1.0 - assign28140_e30158);
        let assign28140_e30160: f64 = (locals.var_w_temp * assign28140_e30159);
        (assign28140_e30160, ((locals.var_w_temp_dn4 * assign28140_e30159) + (locals.var_w_temp * (-((((locals.var_w_temp_dn4 / assign28140_e30153) * assign28140_e30157) - (assign28140_e30154 * locals.var_w_temp_dn4)) / (assign28140_e30157 * assign28140_e30157))))), ((locals.var_w_temp_dn6 * assign28140_e30159) + (locals.var_w_temp * (-((((locals.var_w_temp_dn6 / assign28140_e30153) * assign28140_e30157) - (assign28140_e30154 * locals.var_w_temp_dn6)) / (assign28140_e30157 * assign28140_e30157))))), ((locals.var_w_temp_dn7 * assign28140_e30159) + (locals.var_w_temp * (-((((locals.var_w_temp_dn7 / assign28140_e30153) * assign28140_e30157) - (assign28140_e30154 * locals.var_w_temp_dn7)) / (assign28140_e30157 * assign28140_e30157))))), ((locals.var_w_temp_dn8 * assign28140_e30159) + (locals.var_w_temp * (-((((locals.var_w_temp_dn8 / assign28140_e30153) * assign28140_e30157) - (assign28140_e30154 * locals.var_w_temp_dn8)) / (assign28140_e30157 * assign28140_e30157))))), ((locals.var_w_temp_dn9 * assign28140_e30159) + (locals.var_w_temp * (-((((locals.var_w_temp_dn9 / assign28140_e30153) * assign28140_e30157) - (assign28140_e30154 * locals.var_w_temp_dn9)) / (assign28140_e30157 * assign28140_e30157))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28140_e30162;
        locals.var_temp_dn4 = assign28140_e30162_d_n4;
        locals.var_temp_dn6 = assign28140_e30162_d_n6;
        locals.var_temp_dn7 = assign28140_e30162_d_n7;
        locals.var_temp_dn8 = assign28140_e30162_d_n8;
        locals.var_temp_dn9 = assign28140_e30162_d_n9;

        let assign28150_e30165: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28150_e30167: f64 = if assign28150_e30165 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard788 = assign28150_e30167;

        let assign28160_e30170: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28160_e30172: f64 = (-80.0);
        let assign28160_e30173: f64 = if assign28160_e30170 > assign28160_e30172 { 1.0 } else { 0.0 };
        locals.var_guard789 = assign28160_e30173;

        let (assign28170_e30187, assign28170_e30187_d_n4, assign28170_e30187_d_n6, assign28170_e30187_d_n7, assign28170_e30187_d_n8, assign28170_e30187_d_n9,) = {
    if ((((locals.var_guard782 != 0.0) && (locals.var_guard787 == 0.0)) && (locals.var_guard788 != 0.0)) && (locals.var_guard789 != 0.0)) {
        let assign28170_e30184: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28170_e30185: f64 = (assign28170_e30184).exp();
        (assign28170_e30185, (assign28170_e30185 * (locals.var_xg1x_edge_dn4 - locals.var_xdeff_dc_dn4)), (assign28170_e30185 * (locals.var_xg1x_edge_dn6 - locals.var_xdeff_dc_dn6)), (assign28170_e30185 * (locals.var_xg1x_edge_dn7 - locals.var_xdeff_dc_dn7)), (assign28170_e30185 * (locals.var_xg1x_edge_dn8 - locals.var_xdeff_dc_dn8)), (assign28170_e30185 * (locals.var_xg1x_edge_dn9 - locals.var_xdeff_dc_dn9)),)
    } else {
        (locals.var_w_temp, locals.var_w_temp_dn4, locals.var_w_temp_dn6, locals.var_w_temp_dn7, locals.var_w_temp_dn8, locals.var_w_temp_dn9,)
    }
};
        locals.var_w_temp = assign28170_e30187;
        locals.var_w_temp_dn4 = assign28170_e30187_d_n4;
        locals.var_w_temp_dn6 = assign28170_e30187_d_n6;
        locals.var_w_temp_dn7 = assign28170_e30187_d_n7;
        locals.var_w_temp_dn8 = assign28170_e30187_d_n8;
        locals.var_w_temp_dn9 = assign28170_e30187_d_n9;

        let (assign28180_e30230, assign28180_e30230_d_n4, assign28180_e30230_d_n6, assign28180_e30230_d_n7, assign28180_e30230_d_n8, assign28180_e30230_d_n9,) = {
    if ((((locals.var_guard782 != 0.0) && (locals.var_guard787 == 0.0)) && (locals.var_guard788 != 0.0)) && (locals.var_guard789 == 0.0)) {
        let assign28180_e30201: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28180_e30202: f64 = (-assign28180_e30201);
        let assign28180_e30204: f64 = (assign28180_e30202 - 80.0);
        let assign28180_e30209: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28180_e30210: f64 = (-assign28180_e30209);
        let assign28180_e30212: f64 = (assign28180_e30210 - 80.0);
        let assign28180_e30213: f64 = (0.5 * assign28180_e30212);
        let assign28180_e30217: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28180_e30218: f64 = (-assign28180_e30217);
        let assign28180_e30220: f64 = (assign28180_e30218 - 80.0);
        let assign28180_e30222: f64 = (assign28180_e30220 * 0.3333333333333);
        let assign28180_e30223: f64 = (1.0 + assign28180_e30222);
        let assign28180_e30224: f64 = (assign28180_e30213 * assign28180_e30223);
        let assign28180_e30225: f64 = (1.0 + assign28180_e30224);
        let assign28180_e30226: f64 = (assign28180_e30204 * assign28180_e30225);
        let assign28180_e30227: f64 = (1.0 + assign28180_e30226);
        let assign28180_e30228: f64 = (1.80485e-35 / assign28180_e30227);
        (assign28180_e30228, (-((1.80485e-35 * (((-(locals.var_xg1x_edge_dn4 - locals.var_xdeff_dc_dn4)) * assign28180_e30225) + (assign28180_e30204 * (((0.5 * (-(locals.var_xg1x_edge_dn4 - locals.var_xdeff_dc_dn4))) * assign28180_e30223) + (assign28180_e30213 * ((-(locals.var_xg1x_edge_dn4 - locals.var_xdeff_dc_dn4)) * 0.3333333333333)))))) / (assign28180_e30227 * assign28180_e30227))), (-((1.80485e-35 * (((-(locals.var_xg1x_edge_dn6 - locals.var_xdeff_dc_dn6)) * assign28180_e30225) + (assign28180_e30204 * (((0.5 * (-(locals.var_xg1x_edge_dn6 - locals.var_xdeff_dc_dn6))) * assign28180_e30223) + (assign28180_e30213 * ((-(locals.var_xg1x_edge_dn6 - locals.var_xdeff_dc_dn6)) * 0.3333333333333)))))) / (assign28180_e30227 * assign28180_e30227))), (-((1.80485e-35 * (((-(locals.var_xg1x_edge_dn7 - locals.var_xdeff_dc_dn7)) * assign28180_e30225) + (assign28180_e30204 * (((0.5 * (-(locals.var_xg1x_edge_dn7 - locals.var_xdeff_dc_dn7))) * assign28180_e30223) + (assign28180_e30213 * ((-(locals.var_xg1x_edge_dn7 - locals.var_xdeff_dc_dn7)) * 0.3333333333333)))))) / (assign28180_e30227 * assign28180_e30227))), (-((1.80485e-35 * (((-(locals.var_xg1x_edge_dn8 - locals.var_xdeff_dc_dn8)) * assign28180_e30225) + (assign28180_e30204 * (((0.5 * (-(locals.var_xg1x_edge_dn8 - locals.var_xdeff_dc_dn8))) * assign28180_e30223) + (assign28180_e30213 * ((-(locals.var_xg1x_edge_dn8 - locals.var_xdeff_dc_dn8)) * 0.3333333333333)))))) / (assign28180_e30227 * assign28180_e30227))), (-((1.80485e-35 * (((-(locals.var_xg1x_edge_dn9 - locals.var_xdeff_dc_dn9)) * assign28180_e30225) + (assign28180_e30204 * (((0.5 * (-(locals.var_xg1x_edge_dn9 - locals.var_xdeff_dc_dn9))) * assign28180_e30223) + (assign28180_e30213 * ((-(locals.var_xg1x_edge_dn9 - locals.var_xdeff_dc_dn9)) * 0.3333333333333)))))) / (assign28180_e30227 * assign28180_e30227))),)
    } else {
        (locals.var_w_temp, locals.var_w_temp_dn4, locals.var_w_temp_dn6, locals.var_w_temp_dn7, locals.var_w_temp_dn8, locals.var_w_temp_dn9,)
    }
};
        locals.var_w_temp = assign28180_e30230;
        locals.var_w_temp_dn4 = assign28180_e30230_d_n4;
        locals.var_w_temp_dn6 = assign28180_e30230_d_n6;
        locals.var_w_temp_dn7 = assign28180_e30230_d_n7;
        locals.var_w_temp_dn8 = assign28180_e30230_d_n8;
        locals.var_w_temp_dn9 = assign28180_e30230_d_n9;

        let (assign28190_e30241, assign28190_e30241_d_n4, assign28190_e30241_d_n6, assign28190_e30241_d_n7, assign28190_e30241_d_n8, assign28190_e30241_d_n9,) = {
    if (((locals.var_guard782 != 0.0) && (locals.var_guard787 == 0.0)) && (locals.var_guard788 != 0.0)) {
        let assign28190_e30239: f64 = (locals.var_prefac_qilow_edge * locals.var_w_temp);
        (assign28190_e30239, ((locals.var_prefac_qilow_edge_dn4 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn4)), ((locals.var_prefac_qilow_edge_dn6 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn6)), ((locals.var_prefac_qilow_edge_dn7 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn7)), ((locals.var_prefac_qilow_edge_dn8 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn8)), ((locals.var_prefac_qilow_edge_dn9 * locals.var_w_temp) + (locals.var_prefac_qilow_edge * locals.var_w_temp_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28190_e30241;
        locals.var_temp_dn4 = assign28190_e30241_d_n4;
        locals.var_temp_dn6 = assign28190_e30241_d_n6;
        locals.var_temp_dn7 = assign28190_e30241_d_n7;
        locals.var_temp_dn8 = assign28190_e30241_d_n8;
        locals.var_temp_dn9 = assign28190_e30241_d_n9;

        let (assign28200_e30256, assign28200_e30256_d_n4, assign28200_e30256_d_n6, assign28200_e30256_d_n7, assign28200_e30256_d_n8, assign28200_e30256_d_n9,) = {
    if (((locals.var_guard782 != 0.0) && (locals.var_guard787 == 0.0)) && (locals.var_guard788 == 0.0)) {
        let assign28200_e30250: f64 = (locals.var_prefac_qilow_edge).ln();
        let assign28200_e30253: f64 = (locals.var_xg1x_edge - locals.var_xdeff_dc);
        let assign28200_e30254: f64 = (assign28200_e30250 + assign28200_e30253);
        (assign28200_e30254, ((locals.var_prefac_qilow_edge_dn4 / locals.var_prefac_qilow_edge) + (locals.var_xg1x_edge_dn4 - locals.var_xdeff_dc_dn4)), ((locals.var_prefac_qilow_edge_dn6 / locals.var_prefac_qilow_edge) + (locals.var_xg1x_edge_dn6 - locals.var_xdeff_dc_dn6)), ((locals.var_prefac_qilow_edge_dn7 / locals.var_prefac_qilow_edge) + (locals.var_xg1x_edge_dn7 - locals.var_xdeff_dc_dn7)), ((locals.var_prefac_qilow_edge_dn8 / locals.var_prefac_qilow_edge) + (locals.var_xg1x_edge_dn8 - locals.var_xdeff_dc_dn8)), ((locals.var_prefac_qilow_edge_dn9 / locals.var_prefac_qilow_edge) + (locals.var_xg1x_edge_dn9 - locals.var_xdeff_dc_dn9)),)
    } else {
        (locals.var_w_temp, locals.var_w_temp_dn4, locals.var_w_temp_dn6, locals.var_w_temp_dn7, locals.var_w_temp_dn8, locals.var_w_temp_dn9,)
    }
};
        locals.var_w_temp = assign28200_e30256;
        locals.var_w_temp_dn4 = assign28200_e30256_d_n4;
        locals.var_w_temp_dn6 = assign28200_e30256_d_n6;
        locals.var_w_temp_dn7 = assign28200_e30256_d_n7;
        locals.var_w_temp_dn8 = assign28200_e30256_d_n8;
        locals.var_w_temp_dn9 = assign28200_e30256_d_n9;

        let (assign28210_e30277, assign28210_e30277_d_n4, assign28210_e30277_d_n6, assign28210_e30277_d_n7, assign28210_e30277_d_n8, assign28210_e30277_d_n9,) = {
    if (((locals.var_guard782 != 0.0) && (locals.var_guard787 == 0.0)) && (locals.var_guard788 == 0.0)) {
        let assign28210_e30268: f64 = (1.0 + locals.var_w_temp);
        let assign28210_e30269: f64 = (assign28210_e30268).ln();
        let assign28210_e30272: f64 = (2.0 + locals.var_w_temp);
        let assign28210_e30273: f64 = (assign28210_e30269 / assign28210_e30272);
        let assign28210_e30274: f64 = (1.0 - assign28210_e30273);
        let assign28210_e30275: f64 = (locals.var_w_temp * assign28210_e30274);
        (assign28210_e30275, ((locals.var_w_temp_dn4 * assign28210_e30274) + (locals.var_w_temp * (-((((locals.var_w_temp_dn4 / assign28210_e30268) * assign28210_e30272) - (assign28210_e30269 * locals.var_w_temp_dn4)) / (assign28210_e30272 * assign28210_e30272))))), ((locals.var_w_temp_dn6 * assign28210_e30274) + (locals.var_w_temp * (-((((locals.var_w_temp_dn6 / assign28210_e30268) * assign28210_e30272) - (assign28210_e30269 * locals.var_w_temp_dn6)) / (assign28210_e30272 * assign28210_e30272))))), ((locals.var_w_temp_dn7 * assign28210_e30274) + (locals.var_w_temp * (-((((locals.var_w_temp_dn7 / assign28210_e30268) * assign28210_e30272) - (assign28210_e30269 * locals.var_w_temp_dn7)) / (assign28210_e30272 * assign28210_e30272))))), ((locals.var_w_temp_dn8 * assign28210_e30274) + (locals.var_w_temp * (-((((locals.var_w_temp_dn8 / assign28210_e30268) * assign28210_e30272) - (assign28210_e30269 * locals.var_w_temp_dn8)) / (assign28210_e30272 * assign28210_e30272))))), ((locals.var_w_temp_dn9 * assign28210_e30274) + (locals.var_w_temp * (-((((locals.var_w_temp_dn9 / assign28210_e30268) * assign28210_e30272) - (assign28210_e30269 * locals.var_w_temp_dn9)) / (assign28210_e30272 * assign28210_e30272))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28210_e30277;
        locals.var_temp_dn4 = assign28210_e30277_d_n4;
        locals.var_temp_dn6 = assign28210_e30277_d_n6;
        locals.var_temp_dn7 = assign28210_e30277_d_n7;
        locals.var_temp_dn8 = assign28210_e30277_d_n8;
        locals.var_temp_dn9 = assign28210_e30277_d_n9;

        let (assign28220_e30281, assign28220_e30281_d_n4, assign28220_e30281_d_n6, assign28220_e30281_d_n7, assign28220_e30281_d_n8, assign28220_e30281_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    } else {
        (locals.var_qid_edge, locals.var_qid_edge_dn4, locals.var_qid_edge_dn6, locals.var_qid_edge_dn7, locals.var_qid_edge_dn8, locals.var_qid_edge_dn9,)
    }
};
        locals.var_qid_edge = assign28220_e30281;
        locals.var_qid_edge_dn4 = assign28220_e30281_d_n4;
        locals.var_qid_edge_dn6 = assign28220_e30281_d_n6;
        locals.var_qid_edge_dn7 = assign28220_e30281_d_n7;
        locals.var_qid_edge_dn8 = assign28220_e30281_d_n8;
        locals.var_qid_edge_dn9 = assign28220_e30281_d_n9;

        let (assign28230_e30295, assign28230_e30295_d_n4, assign28230_e30295_d_n6, assign28230_e30295_d_n7, assign28230_e30295_d_n8, assign28230_e30295_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign28230_e30286: f64 = (locals.var_qis_edge + locals.var_qid_edge);
        let assign28230_e30287: f64 = (0.5 * assign28230_e30286);
        let assign28230_e30289: f64 = (assign28230_e30287 + 1.0);
        let assign28230_e30292: f64 = (locals.var_qis_edge - locals.var_qid_edge);
        let assign28230_e30293: f64 = (assign28230_e30289 * assign28230_e30292);
        (assign28230_e30293, (((0.5 * (locals.var_qis_edge_dn4 + locals.var_qid_edge_dn4)) * assign28230_e30292) + (assign28230_e30289 * (locals.var_qis_edge_dn4 - locals.var_qid_edge_dn4))), (((0.5 * (locals.var_qis_edge_dn6 + locals.var_qid_edge_dn6)) * assign28230_e30292) + (assign28230_e30289 * (locals.var_qis_edge_dn6 - locals.var_qid_edge_dn6))), (((0.5 * (locals.var_qis_edge_dn7 + locals.var_qid_edge_dn7)) * assign28230_e30292) + (assign28230_e30289 * (locals.var_qis_edge_dn7 - locals.var_qid_edge_dn7))), (((0.5 * (locals.var_qis_edge_dn8 + locals.var_qid_edge_dn8)) * assign28230_e30292) + (assign28230_e30289 * (locals.var_qis_edge_dn8 - locals.var_qid_edge_dn8))), (((0.5 * (locals.var_qis_edge_dn9 + locals.var_qid_edge_dn9)) * assign28230_e30292) + (assign28230_e30289 * (locals.var_qis_edge_dn9 - locals.var_qid_edge_dn9))),)
    } else {
        (locals.var_norm_ids_edge, locals.var_norm_ids_edge_dn4, locals.var_norm_ids_edge_dn6, locals.var_norm_ids_edge_dn7, locals.var_norm_ids_edge_dn8, locals.var_norm_ids_edge_dn9,)
    }
};
        locals.var_norm_ids_edge = assign28230_e30295;
        locals.var_norm_ids_edge_dn4 = assign28230_e30295_d_n4;
        locals.var_norm_ids_edge_dn6 = assign28230_e30295_d_n6;
        locals.var_norm_ids_edge_dn7 = assign28230_e30295_d_n7;
        locals.var_norm_ids_edge_dn8 = assign28230_e30295_d_n8;
        locals.var_norm_ids_edge_dn9 = assign28230_e30295_d_n9;

        let (assign28240_e30303, assign28240_e30303_d_n4, assign28240_e30303_d_n6, assign28240_e30303_d_n7, assign28240_e30303_d_n8, assign28240_e30303_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign28240_e30299: f64 = (locals.var_phit_edge * locals.var_phit_edge);
        let assign28240_e30301: f64 = (assign28240_e30299 * locals.var_betnedge_i);
        (assign28240_e30301, ((((locals.var_phit_edge_dn4 * locals.var_phit_edge) + (locals.var_phit_edge * locals.var_phit_edge_dn4)) * locals.var_betnedge_i) + (assign28240_e30299 * locals.var_betnedge_i_dn4)), ((((locals.var_phit_edge_dn6 * locals.var_phit_edge) + (locals.var_phit_edge * locals.var_phit_edge_dn6)) * locals.var_betnedge_i) + (assign28240_e30299 * locals.var_betnedge_i_dn6)), ((((locals.var_phit_edge_dn7 * locals.var_phit_edge) + (locals.var_phit_edge * locals.var_phit_edge_dn7)) * locals.var_betnedge_i) + (assign28240_e30299 * locals.var_betnedge_i_dn7)), ((((locals.var_phit_edge_dn8 * locals.var_phit_edge) + (locals.var_phit_edge * locals.var_phit_edge_dn8)) * locals.var_betnedge_i) + (assign28240_e30299 * locals.var_betnedge_i_dn8)), ((((locals.var_phit_edge_dn9 * locals.var_phit_edge) + (locals.var_phit_edge * locals.var_phit_edge_dn9)) * locals.var_betnedge_i) + (assign28240_e30299 * locals.var_betnedge_i_dn9)),)
    } else {
        (locals.var_fact_ids_edge, locals.var_fact_ids_edge_dn4, locals.var_fact_ids_edge_dn6, locals.var_fact_ids_edge_dn7, locals.var_fact_ids_edge_dn8, locals.var_fact_ids_edge_dn9,)
    }
};
        locals.var_fact_ids_edge = assign28240_e30303;
        locals.var_fact_ids_edge_dn4 = assign28240_e30303_d_n4;
        locals.var_fact_ids_edge_dn6 = assign28240_e30303_d_n6;
        locals.var_fact_ids_edge_dn7 = assign28240_e30303_d_n7;
        locals.var_fact_ids_edge_dn8 = assign28240_e30303_d_n8;
        locals.var_fact_ids_edge_dn9 = assign28240_e30303_d_n9;

        let (assign28250_e30313, assign28250_e30313_d_n4, assign28250_e30313_d_n6, assign28250_e30313_d_n7, assign28250_e30313_d_n8, assign28250_e30313_d_n9,) = {
    if (locals.var_guard782 != 0.0) {
        let assign28250_e30307: f64 = (locals.var_fact_ids_edge * locals.var_cox1prime);
        let assign28250_e30309: f64 = (assign28250_e30307 * locals.var_norm_ids_edge);
        let assign28250_e30311: f64 = (assign28250_e30309 / locals.var_gmob_dc);
        (assign28250_e30311, ((((((locals.var_fact_ids_edge_dn4 * locals.var_cox1prime) * locals.var_norm_ids_edge) + (assign28250_e30307 * locals.var_norm_ids_edge_dn4)) * locals.var_gmob_dc) - (assign28250_e30309 * locals.var_gmob_dc_dn4)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((locals.var_fact_ids_edge_dn6 * locals.var_cox1prime) * locals.var_norm_ids_edge) + (assign28250_e30307 * locals.var_norm_ids_edge_dn6)) * locals.var_gmob_dc) - (assign28250_e30309 * locals.var_gmob_dc_dn6)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((locals.var_fact_ids_edge_dn7 * locals.var_cox1prime) * locals.var_norm_ids_edge) + (assign28250_e30307 * locals.var_norm_ids_edge_dn7)) * locals.var_gmob_dc) - (assign28250_e30309 * locals.var_gmob_dc_dn7)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((locals.var_fact_ids_edge_dn8 * locals.var_cox1prime) * locals.var_norm_ids_edge) + (assign28250_e30307 * locals.var_norm_ids_edge_dn8)) * locals.var_gmob_dc) - (assign28250_e30309 * locals.var_gmob_dc_dn8)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((locals.var_fact_ids_edge_dn9 * locals.var_cox1prime) * locals.var_norm_ids_edge) + (assign28250_e30307 * locals.var_norm_ids_edge_dn9)) * locals.var_gmob_dc) - (assign28250_e30309 * locals.var_gmob_dc_dn9)) / (locals.var_gmob_dc * locals.var_gmob_dc)),)
    } else {
        (locals.var_ids_edge, locals.var_ids_edge_dn4, locals.var_ids_edge_dn6, locals.var_ids_edge_dn7, locals.var_ids_edge_dn8, locals.var_ids_edge_dn9,)
    }
};
        locals.var_ids_edge = assign28250_e30313;
        locals.var_ids_edge_dn4 = assign28250_e30313_d_n4;
        locals.var_ids_edge_dn6 = assign28250_e30313_d_n6;
        locals.var_ids_edge_dn7 = assign28250_e30313_d_n7;
        locals.var_ids_edge_dn8 = assign28250_e30313_d_n8;
        locals.var_ids_edge_dn9 = assign28250_e30313_d_n9;

        locals.var_mavl = 0.0;
        locals.var_mavl_dn4 = 0.0;
        locals.var_mavl_dn6 = 0.0;
        locals.var_mavl_dn7 = 0.0;
        locals.var_mavl_dn8 = 0.0;
        locals.var_mavl_dn9 = 0.0;

    }

    pub(super) fn stamp_transient_block_74(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_iimpact = 0.0;
        locals.var_iimpact_dn4 = 0.0;
        locals.var_iimpact_dn6 = 0.0;
        locals.var_iimpact_dn7 = 0.0;
        locals.var_iimpact_dn8 = 0.0;
        locals.var_iimpact_dn9 = 0.0;

        let assign28280_e30318: f64 = if p.p8 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard790 = assign28280_e30318;

        let (assign28290_e30328, assign28290_e30328_d_n4, assign28290_e30328_d_n6, assign28290_e30328_d_n7, assign28290_e30328_d_n8, assign28290_e30328_d_n9,) = {
    if (locals.var_guard790 != 0.0) {
        let assign28290_e30323: f64 = (locals.var_a3_i * locals.var_xdeff_dc);
        let assign28290_e30324: f64 = (locals.var_xd - assign28290_e30323);
        let assign28290_e30326: f64 = (assign28290_e30324 / locals.var_inv_phit);
        (assign28290_e30326, ((((locals.var_xd_dn4 - (locals.var_a3_i * locals.var_xdeff_dc_dn4)) * locals.var_inv_phit) - (assign28290_e30324 * locals.var_inv_phit_dn4)) / (locals.var_inv_phit * locals.var_inv_phit)), ((((locals.var_xd_dn6 - (locals.var_a3_i * locals.var_xdeff_dc_dn6)) * locals.var_inv_phit) - (assign28290_e30324 * locals.var_inv_phit_dn6)) / (locals.var_inv_phit * locals.var_inv_phit)), ((((locals.var_xd_dn7 - (locals.var_a3_i * locals.var_xdeff_dc_dn7)) * locals.var_inv_phit) - (assign28290_e30324 * locals.var_inv_phit_dn7)) / (locals.var_inv_phit * locals.var_inv_phit)), ((((locals.var_xd_dn8 - (locals.var_a3_i * locals.var_xdeff_dc_dn8)) * locals.var_inv_phit) - (assign28290_e30324 * locals.var_inv_phit_dn8)) / (locals.var_inv_phit * locals.var_inv_phit)), ((((locals.var_xd_dn9 - (locals.var_a3_i * locals.var_xdeff_dc_dn9)) * locals.var_inv_phit) - (assign28290_e30324 * locals.var_inv_phit_dn9)) / (locals.var_inv_phit * locals.var_inv_phit)),)
    } else {
        (locals.var_delvsat, locals.var_delvsat_dn4, locals.var_delvsat_dn6, locals.var_delvsat_dn7, locals.var_delvsat_dn8, locals.var_delvsat_dn9,)
    }
};
        locals.var_delvsat = assign28290_e30328;
        locals.var_delvsat_dn4 = assign28290_e30328_d_n4;
        locals.var_delvsat_dn6 = assign28290_e30328_d_n6;
        locals.var_delvsat_dn7 = assign28290_e30328_d_n7;
        locals.var_delvsat_dn8 = assign28290_e30328_d_n8;
        locals.var_delvsat_dn9 = assign28290_e30328_d_n9;

        let assign28300_e30331: f64 = if locals.var_delvsat > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard791 = assign28300_e30331;

        let (assign28310_e30344, assign28310_e30344_d_n4, assign28310_e30344_d_n6, assign28310_e30344_d_n7, assign28310_e30344_d_n8, assign28310_e30344_d_n9,) = {
    if ((locals.var_guard790 != 0.0) && (locals.var_guard791 != 0.0)) {
        let assign28310_e30336: f64 = (-1.0);
        let assign28310_e30338: f64 = (assign28310_e30336 * locals.var_a2_i);
        let assign28310_e30341: f64 = (locals.var_delvsat + 1e-30);
        let assign28310_e30342: f64 = (assign28310_e30338 / assign28310_e30341);
        (assign28310_e30342, ((((assign28310_e30336 * locals.var_a2_i_dn4) * assign28310_e30341) - (assign28310_e30338 * locals.var_delvsat_dn4)) / (assign28310_e30341 * assign28310_e30341)), ((((assign28310_e30336 * locals.var_a2_i_dn6) * assign28310_e30341) - (assign28310_e30338 * locals.var_delvsat_dn6)) / (assign28310_e30341 * assign28310_e30341)), ((((assign28310_e30336 * locals.var_a2_i_dn7) * assign28310_e30341) - (assign28310_e30338 * locals.var_delvsat_dn7)) / (assign28310_e30341 * assign28310_e30341)), ((((assign28310_e30336 * locals.var_a2_i_dn8) * assign28310_e30341) - (assign28310_e30338 * locals.var_delvsat_dn8)) / (assign28310_e30341 * assign28310_e30341)), ((((assign28310_e30336 * locals.var_a2_i_dn9) * assign28310_e30341) - (assign28310_e30338 * locals.var_delvsat_dn9)) / (assign28310_e30341 * assign28310_e30341)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign28310_e30344;
        locals.var_temp2_dn4 = assign28310_e30344_d_n4;
        locals.var_temp2_dn6 = assign28310_e30344_d_n6;
        locals.var_temp2_dn7 = assign28310_e30344_d_n7;
        locals.var_temp2_dn8 = assign28310_e30344_d_n8;
        locals.var_temp2_dn9 = assign28310_e30344_d_n9;

        let assign28320_e30346: f64 = (locals.var_temp2).abs();
        let assign28320_e30348: f64 = if assign28320_e30346 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard792 = assign28320_e30348;

        let (assign28330_e30357, assign28330_e30357_d_n4, assign28330_e30357_d_n6, assign28330_e30357_d_n7, assign28330_e30357_d_n8, assign28330_e30357_d_n9,) = {
    if (((locals.var_guard790 != 0.0) && (locals.var_guard791 != 0.0)) && (locals.var_guard792 != 0.0)) {
        let assign28330_e30355: f64 = (locals.var_temp2).exp();
        (assign28330_e30355, (assign28330_e30355 * locals.var_temp2_dn4), (assign28330_e30355 * locals.var_temp2_dn6), (assign28330_e30355 * locals.var_temp2_dn7), (assign28330_e30355 * locals.var_temp2_dn8), (assign28330_e30355 * locals.var_temp2_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28330_e30357;
        locals.var_temp_dn4 = assign28330_e30357_d_n4;
        locals.var_temp_dn6 = assign28330_e30357_d_n6;
        locals.var_temp_dn7 = assign28330_e30357_d_n7;
        locals.var_temp_dn8 = assign28330_e30357_d_n8;
        locals.var_temp_dn9 = assign28330_e30357_d_n9;

        let assign28340_e30360: f64 = (-80.0);
        let assign28340_e30361: f64 = if locals.var_temp2 < assign28340_e30360 { 1.0 } else { 0.0 };
        locals.var_guard793 = assign28340_e30361;

        let (assign28350_e30397, assign28350_e30397_d_n4, assign28350_e30397_d_n6, assign28350_e30397_d_n7, assign28350_e30397_d_n8, assign28350_e30397_d_n9,) = {
    if ((((locals.var_guard790 != 0.0) && (locals.var_guard791 != 0.0)) && (locals.var_guard792 == 0.0)) && (locals.var_guard793 != 0.0)) {
        let assign28350_e30373: f64 = (-locals.var_temp2);
        let assign28350_e30375: f64 = (assign28350_e30373 - 80.0);
        let assign28350_e30379: f64 = (-locals.var_temp2);
        let assign28350_e30381: f64 = (assign28350_e30379 - 80.0);
        let assign28350_e30382: f64 = (0.5 * assign28350_e30381);
        let assign28350_e30385: f64 = (-locals.var_temp2);
        let assign28350_e30387: f64 = (assign28350_e30385 - 80.0);
        let assign28350_e30389: f64 = (assign28350_e30387 * 0.3333333333333);
        let assign28350_e30390: f64 = (1.0 + assign28350_e30389);
        let assign28350_e30391: f64 = (assign28350_e30382 * assign28350_e30390);
        let assign28350_e30392: f64 = (1.0 + assign28350_e30391);
        let assign28350_e30393: f64 = (assign28350_e30375 * assign28350_e30392);
        let assign28350_e30394: f64 = (1.0 + assign28350_e30393);
        let assign28350_e30395: f64 = (1.80485e-35 / assign28350_e30394);
        (assign28350_e30395, (-((1.80485e-35 * (((-locals.var_temp2_dn4) * assign28350_e30392) + (assign28350_e30375 * (((0.5 * (-locals.var_temp2_dn4)) * assign28350_e30390) + (assign28350_e30382 * ((-locals.var_temp2_dn4) * 0.3333333333333)))))) / (assign28350_e30394 * assign28350_e30394))), (-((1.80485e-35 * (((-locals.var_temp2_dn6) * assign28350_e30392) + (assign28350_e30375 * (((0.5 * (-locals.var_temp2_dn6)) * assign28350_e30390) + (assign28350_e30382 * ((-locals.var_temp2_dn6) * 0.3333333333333)))))) / (assign28350_e30394 * assign28350_e30394))), (-((1.80485e-35 * (((-locals.var_temp2_dn7) * assign28350_e30392) + (assign28350_e30375 * (((0.5 * (-locals.var_temp2_dn7)) * assign28350_e30390) + (assign28350_e30382 * ((-locals.var_temp2_dn7) * 0.3333333333333)))))) / (assign28350_e30394 * assign28350_e30394))), (-((1.80485e-35 * (((-locals.var_temp2_dn8) * assign28350_e30392) + (assign28350_e30375 * (((0.5 * (-locals.var_temp2_dn8)) * assign28350_e30390) + (assign28350_e30382 * ((-locals.var_temp2_dn8) * 0.3333333333333)))))) / (assign28350_e30394 * assign28350_e30394))), (-((1.80485e-35 * (((-locals.var_temp2_dn9) * assign28350_e30392) + (assign28350_e30375 * (((0.5 * (-locals.var_temp2_dn9)) * assign28350_e30390) + (assign28350_e30382 * ((-locals.var_temp2_dn9) * 0.3333333333333)))))) / (assign28350_e30394 * assign28350_e30394))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28350_e30397;
        locals.var_temp_dn4 = assign28350_e30397_d_n4;
        locals.var_temp_dn6 = assign28350_e30397_d_n6;
        locals.var_temp_dn7 = assign28350_e30397_d_n7;
        locals.var_temp_dn8 = assign28350_e30397_d_n8;
        locals.var_temp_dn9 = assign28350_e30397_d_n9;

        let (assign28360_e30431, assign28360_e30431_d_n4, assign28360_e30431_d_n6, assign28360_e30431_d_n7, assign28360_e30431_d_n8, assign28360_e30431_d_n9,) = {
    if ((((locals.var_guard790 != 0.0) && (locals.var_guard791 != 0.0)) && (locals.var_guard792 == 0.0)) && (locals.var_guard793 == 0.0)) {
        let assign28360_e30411: f64 = (locals.var_temp2 - 80.0);
        let assign28360_e30416: f64 = (locals.var_temp2 - 80.0);
        let assign28360_e30417: f64 = (0.5 * assign28360_e30416);
        let assign28360_e30421: f64 = (locals.var_temp2 - 80.0);
        let assign28360_e30423: f64 = (assign28360_e30421 * 0.3333333333333);
        let assign28360_e30424: f64 = (1.0 + assign28360_e30423);
        let assign28360_e30425: f64 = (assign28360_e30417 * assign28360_e30424);
        let assign28360_e30426: f64 = (1.0 + assign28360_e30425);
        let assign28360_e30427: f64 = (assign28360_e30411 * assign28360_e30426);
        let assign28360_e30428: f64 = (1.0 + assign28360_e30427);
        let assign28360_e30429: f64 = (5.54062e34 * assign28360_e30428);
        (assign28360_e30429, (5.54062e34 * ((locals.var_temp2_dn4 * assign28360_e30426) + (assign28360_e30411 * (((0.5 * locals.var_temp2_dn4) * assign28360_e30424) + (assign28360_e30417 * (locals.var_temp2_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn6 * assign28360_e30426) + (assign28360_e30411 * (((0.5 * locals.var_temp2_dn6) * assign28360_e30424) + (assign28360_e30417 * (locals.var_temp2_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn7 * assign28360_e30426) + (assign28360_e30411 * (((0.5 * locals.var_temp2_dn7) * assign28360_e30424) + (assign28360_e30417 * (locals.var_temp2_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn8 * assign28360_e30426) + (assign28360_e30411 * (((0.5 * locals.var_temp2_dn8) * assign28360_e30424) + (assign28360_e30417 * (locals.var_temp2_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn9 * assign28360_e30426) + (assign28360_e30411 * (((0.5 * locals.var_temp2_dn9) * assign28360_e30424) + (assign28360_e30417 * (locals.var_temp2_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28360_e30431;
        locals.var_temp_dn4 = assign28360_e30431_d_n4;
        locals.var_temp_dn6 = assign28360_e30431_d_n6;
        locals.var_temp_dn7 = assign28360_e30431_d_n7;
        locals.var_temp_dn8 = assign28360_e30431_d_n8;
        locals.var_temp_dn9 = assign28360_e30431_d_n9;

        let (assign28370_e30441, assign28370_e30441_d_n4, assign28370_e30441_d_n6, assign28370_e30441_d_n7, assign28370_e30441_d_n8, assign28370_e30441_d_n9,) = {
    if ((locals.var_guard790 != 0.0) && (locals.var_guard791 != 0.0)) {
        let assign28370_e30437: f64 = (locals.var_a1_i * locals.var_delvsat);
        let assign28370_e30439: f64 = (assign28370_e30437 * locals.var_temp);
        (assign28370_e30439, (((locals.var_a1_i * locals.var_delvsat_dn4) * locals.var_temp) + (assign28370_e30437 * locals.var_temp_dn4)), (((locals.var_a1_i * locals.var_delvsat_dn6) * locals.var_temp) + (assign28370_e30437 * locals.var_temp_dn6)), (((locals.var_a1_i * locals.var_delvsat_dn7) * locals.var_temp) + (assign28370_e30437 * locals.var_temp_dn7)), (((locals.var_a1_i * locals.var_delvsat_dn8) * locals.var_temp) + (assign28370_e30437 * locals.var_temp_dn8)), (((locals.var_a1_i * locals.var_delvsat_dn9) * locals.var_temp) + (assign28370_e30437 * locals.var_temp_dn9)),)
    } else {
        (locals.var_mavl, locals.var_mavl_dn4, locals.var_mavl_dn6, locals.var_mavl_dn7, locals.var_mavl_dn8, locals.var_mavl_dn9,)
    }
};
        locals.var_mavl = assign28370_e30441;
        locals.var_mavl_dn4 = assign28370_e30441_d_n4;
        locals.var_mavl_dn6 = assign28370_e30441_d_n6;
        locals.var_mavl_dn7 = assign28370_e30441_d_n7;
        locals.var_mavl_dn8 = assign28370_e30441_d_n8;
        locals.var_mavl_dn9 = assign28370_e30441_d_n9;

        let (assign28380_e30451, assign28380_e30451_d_n4, assign28380_e30451_d_n6, assign28380_e30451_d_n7, assign28380_e30451_d_n8, assign28380_e30451_d_n9,) = {
    if ((locals.var_guard790 != 0.0) && (locals.var_guard791 != 0.0)) {
        let assign28380_e30448: f64 = (locals.var_ids + locals.var_ids_edge);
        let assign28380_e30449: f64 = (locals.var_mavl * assign28380_e30448);
        (assign28380_e30449, ((locals.var_mavl_dn4 * assign28380_e30448) + (locals.var_mavl * (locals.var_ids_dn4 + locals.var_ids_edge_dn4))), ((locals.var_mavl_dn6 * assign28380_e30448) + (locals.var_mavl * (locals.var_ids_dn6 + locals.var_ids_edge_dn6))), ((locals.var_mavl_dn7 * assign28380_e30448) + (locals.var_mavl * (locals.var_ids_dn7 + locals.var_ids_edge_dn7))), ((locals.var_mavl_dn8 * assign28380_e30448) + (locals.var_mavl * (locals.var_ids_dn8 + locals.var_ids_edge_dn8))), ((locals.var_mavl_dn9 * assign28380_e30448) + (locals.var_mavl * (locals.var_ids_dn9 + locals.var_ids_edge_dn9))),)
    } else {
        (locals.var_iimpact, locals.var_iimpact_dn4, locals.var_iimpact_dn6, locals.var_iimpact_dn7, locals.var_iimpact_dn8, locals.var_iimpact_dn9,)
    }
};
        locals.var_iimpact = assign28380_e30451;
        locals.var_iimpact_dn4 = assign28380_e30451_d_n4;
        locals.var_iimpact_dn6 = assign28380_e30451_d_n6;
        locals.var_iimpact_dn7 = assign28380_e30451_d_n7;
        locals.var_iimpact_dn8 = assign28380_e30451_d_n8;
        locals.var_iimpact_dn9 = assign28380_e30451_d_n9;

        let assign28390_e30454: f64 = if locals.var_swshe_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard794 = assign28390_e30454;

        let (assign28400_e30465, assign28400_e30465_d_n4, assign28400_e30465_d_n6, assign28400_e30465_d_n7, assign28400_e30465_d_n8, assign28400_e30465_d_n9,) = {
    if (locals.var_guard794 != 0.0) {
        let assign28400_e30458: f64 = (locals.var_ids + locals.var_ids_edge);
        let assign28400_e30460: f64 = (assign28400_e30458 * locals.var_vds);
        let assign28400_e30461: f64 = (assign28400_e30460).abs();
        let assign28400_e30463: f64 = (assign28400_e30461 * locals.var_rth_i);
        (assign28400_e30463, ((if assign28400_e30460 >= 0.0 { ((locals.var_ids_dn4 + locals.var_ids_edge_dn4) * locals.var_vds) } else { (-((locals.var_ids_dn4 + locals.var_ids_edge_dn4) * locals.var_vds)) } * locals.var_rth_i) + (assign28400_e30461 * locals.var_rth_i_dn4)), ((if assign28400_e30460 >= 0.0 { (((locals.var_ids_dn6 + locals.var_ids_edge_dn6) * locals.var_vds) + (assign28400_e30458 * locals.var_vds_dn6)) } else { (-(((locals.var_ids_dn6 + locals.var_ids_edge_dn6) * locals.var_vds) + (assign28400_e30458 * locals.var_vds_dn6))) } * locals.var_rth_i) + (assign28400_e30461 * locals.var_rth_i_dn6)), ((if assign28400_e30460 >= 0.0 { (((locals.var_ids_dn7 + locals.var_ids_edge_dn7) * locals.var_vds) + (assign28400_e30458 * locals.var_vds_dn7)) } else { (-(((locals.var_ids_dn7 + locals.var_ids_edge_dn7) * locals.var_vds) + (assign28400_e30458 * locals.var_vds_dn7))) } * locals.var_rth_i) + (assign28400_e30461 * locals.var_rth_i_dn7)), ((if assign28400_e30460 >= 0.0 { ((locals.var_ids_dn8 + locals.var_ids_edge_dn8) * locals.var_vds) } else { (-((locals.var_ids_dn8 + locals.var_ids_edge_dn8) * locals.var_vds)) } * locals.var_rth_i) + (assign28400_e30461 * locals.var_rth_i_dn8)), ((if assign28400_e30460 >= 0.0 { ((locals.var_ids_dn9 + locals.var_ids_edge_dn9) * locals.var_vds) } else { (-((locals.var_ids_dn9 + locals.var_ids_edge_dn9) * locals.var_vds)) } * locals.var_rth_i) + (assign28400_e30461 * locals.var_rth_i_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28400_e30465;
        locals.var_temp_dn4 = assign28400_e30465_d_n4;
        locals.var_temp_dn6 = assign28400_e30465_d_n6;
        locals.var_temp_dn7 = assign28400_e30465_d_n7;
        locals.var_temp_dn8 = assign28400_e30465_d_n8;
        locals.var_temp_dn9 = assign28400_e30465_d_n9;

        let assign28410_e30469: f64 = (100000000.0 * p.p16);
        let assign28410_e30470: f64 = if locals.var_temp > assign28410_e30469 { 1.0 } else { 0.0 };
        locals.var_guard795 = assign28410_e30470;

        let (assign28420_e30483, assign28420_e30483_d_n4, assign28420_e30483_d_n6, assign28420_e30483_d_n7, assign28420_e30483_d_n8, assign28420_e30483_d_n9,) = {
    if ((locals.var_guard794 != 0.0) && (locals.var_guard795 != 0.0)) {
        let assign28420_e30477: f64 = (0.25 / p.p16);
        let assign28420_e30478: f64 = (p.p16 + assign28420_e30477);
        let assign28420_e30479: f64 = (-assign28420_e30478);
        let assign28420_e30481: f64 = (assign28420_e30479 / locals.var_rth_i);
        (assign28420_e30481, (-((assign28420_e30479 * locals.var_rth_i_dn4) / (locals.var_rth_i * locals.var_rth_i))), (-((assign28420_e30479 * locals.var_rth_i_dn6) / (locals.var_rth_i * locals.var_rth_i))), (-((assign28420_e30479 * locals.var_rth_i_dn7) / (locals.var_rth_i * locals.var_rth_i))), (-((assign28420_e30479 * locals.var_rth_i_dn8) / (locals.var_rth_i * locals.var_rth_i))), (-((assign28420_e30479 * locals.var_rth_i_dn9) / (locals.var_rth_i * locals.var_rth_i))),)
    } else {
        (locals.var_ithpwr, locals.var_ithpwr_dn4, locals.var_ithpwr_dn6, locals.var_ithpwr_dn7, locals.var_ithpwr_dn8, locals.var_ithpwr_dn9,)
    }
};
        locals.var_ithpwr = assign28420_e30483;
        locals.var_ithpwr_dn4 = assign28420_e30483_d_n4;
        locals.var_ithpwr_dn6 = assign28420_e30483_d_n6;
        locals.var_ithpwr_dn7 = assign28420_e30483_d_n7;
        locals.var_ithpwr_dn8 = assign28420_e30483_d_n8;
        locals.var_ithpwr_dn9 = assign28420_e30483_d_n9;

        let (assign28430_e30514, assign28430_e30514_d_n4, assign28430_e30514_d_n6, assign28430_e30514_d_n7, assign28430_e30514_d_n8, assign28430_e30514_d_n9,) = {
    if ((locals.var_guard794 != 0.0) && (locals.var_guard795 == 0.0)) {
        let assign28430_e30490: f64 = 0.5;
        let assign28430_e30493: f64 = (locals.var_temp + p.p16);
        let assign28430_e30496: f64 = (locals.var_temp - p.p16);
        let assign28430_e30499: f64 = (locals.var_temp - p.p16);
        let assign28430_e30500: f64 = (assign28430_e30496 * assign28430_e30499);
        let assign28430_e30502: f64 = (assign28430_e30500 + 1.0);
        let assign28430_e30503: f64 = (assign28430_e30502).sqrt();
        let assign28430_e30504: f64 = (assign28430_e30493 - assign28430_e30503);
        let assign28430_e30505: f64 = (assign28430_e30490 * assign28430_e30504);
        let assign28430_e30508: f64 = (0.25 / p.p16);
        let assign28430_e30509: f64 = (assign28430_e30505 + assign28430_e30508);
        let assign28430_e30510: f64 = (-assign28430_e30509);
        let assign28430_e30512: f64 = (assign28430_e30510 / locals.var_rth_i);
        (assign28430_e30512, ((((-(assign28430_e30490 * (locals.var_temp_dn4 - (((locals.var_temp_dn4 * assign28430_e30499) + (assign28430_e30496 * locals.var_temp_dn4)) / (2.0 * assign28430_e30503))))) * locals.var_rth_i) - (assign28430_e30510 * locals.var_rth_i_dn4)) / (locals.var_rth_i * locals.var_rth_i)), ((((-(assign28430_e30490 * (locals.var_temp_dn6 - (((locals.var_temp_dn6 * assign28430_e30499) + (assign28430_e30496 * locals.var_temp_dn6)) / (2.0 * assign28430_e30503))))) * locals.var_rth_i) - (assign28430_e30510 * locals.var_rth_i_dn6)) / (locals.var_rth_i * locals.var_rth_i)), ((((-(assign28430_e30490 * (locals.var_temp_dn7 - (((locals.var_temp_dn7 * assign28430_e30499) + (assign28430_e30496 * locals.var_temp_dn7)) / (2.0 * assign28430_e30503))))) * locals.var_rth_i) - (assign28430_e30510 * locals.var_rth_i_dn7)) / (locals.var_rth_i * locals.var_rth_i)), ((((-(assign28430_e30490 * (locals.var_temp_dn8 - (((locals.var_temp_dn8 * assign28430_e30499) + (assign28430_e30496 * locals.var_temp_dn8)) / (2.0 * assign28430_e30503))))) * locals.var_rth_i) - (assign28430_e30510 * locals.var_rth_i_dn8)) / (locals.var_rth_i * locals.var_rth_i)), ((((-(assign28430_e30490 * (locals.var_temp_dn9 - (((locals.var_temp_dn9 * assign28430_e30499) + (assign28430_e30496 * locals.var_temp_dn9)) / (2.0 * assign28430_e30503))))) * locals.var_rth_i) - (assign28430_e30510 * locals.var_rth_i_dn9)) / (locals.var_rth_i * locals.var_rth_i)),)
    } else {
        (locals.var_ithpwr, locals.var_ithpwr_dn4, locals.var_ithpwr_dn6, locals.var_ithpwr_dn7, locals.var_ithpwr_dn8, locals.var_ithpwr_dn9,)
    }
};
        locals.var_ithpwr = assign28430_e30514;
        locals.var_ithpwr_dn4 = assign28430_e30514_d_n4;
        locals.var_ithpwr_dn6 = assign28430_e30514_d_n6;
        locals.var_ithpwr_dn7 = assign28430_e30514_d_n7;
        locals.var_ithpwr_dn8 = assign28430_e30514_d_n8;
        locals.var_ithpwr_dn9 = assign28430_e30514_d_n9;

        let (assign28450_e30525, assign28450_e30525_d_n4, assign28450_e30525_d_n6, assign28450_e30525_d_n7, assign28450_e30525_d_n8, assign28450_e30525_d_n9,) = {
    if (locals.var_guard794 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ithpwr, locals.var_ithpwr_dn4, locals.var_ithpwr_dn6, locals.var_ithpwr_dn7, locals.var_ithpwr_dn8, locals.var_ithpwr_dn9,)
    }
};
        locals.var_ithpwr = assign28450_e30525;
        locals.var_ithpwr_dn4 = assign28450_e30525_d_n4;
        locals.var_ithpwr_dn6 = assign28450_e30525_d_n6;
        locals.var_ithpwr_dn7 = assign28450_e30525_d_n7;
        locals.var_ithpwr_dn8 = assign28450_e30525_d_n8;
        locals.var_ithpwr_dn9 = assign28450_e30525_d_n9;

        let assign28510_e30547: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1080 = assign28510_e30547;

        let (assign28520_e30551, assign28520_e30551_d_n4, assign28520_e30551_d_n6, assign28520_e30551_d_n7, assign28520_e30551_d_n8, assign28520_e30551_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_vfbac1_i, locals.var_vfbac1_i_dn4, locals.var_vfbac1_i_dn6, locals.var_vfbac1_i_dn7, locals.var_vfbac1_i_dn8, locals.var_vfbac1_i_dn9,)
    } else {
        (locals.var_vfb1_loc__blk890, locals.var_vfb1_loc__blk890_dn4, locals.var_vfb1_loc__blk890_dn6, locals.var_vfb1_loc__blk890_dn7, locals.var_vfb1_loc__blk890_dn8, locals.var_vfb1_loc__blk890_dn9,)
    }
};
        locals.var_vfb1_loc__blk890 = assign28520_e30551;
        locals.var_vfb1_loc__blk890_dn4 = assign28520_e30551_d_n4;
        locals.var_vfb1_loc__blk890_dn6 = assign28520_e30551_d_n6;
        locals.var_vfb1_loc__blk890_dn7 = assign28520_e30551_d_n7;
        locals.var_vfb1_loc__blk890_dn8 = assign28520_e30551_d_n8;
        locals.var_vfb1_loc__blk890_dn9 = assign28520_e30551_d_n9;

        let (assign28530_e30555, assign28530_e30555_d_n4, assign28530_e30555_d_n6, assign28530_e30555_d_n7, assign28530_e30555_d_n8, assign28530_e30555_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_vfbac2_i, locals.var_vfbac2_i_dn4, locals.var_vfbac2_i_dn6, locals.var_vfbac2_i_dn7, locals.var_vfbac2_i_dn8, locals.var_vfbac2_i_dn9,)
    } else {
        (locals.var_vfb2_loc__blk891, locals.var_vfb2_loc__blk891_dn4, locals.var_vfb2_loc__blk891_dn6, locals.var_vfb2_loc__blk891_dn7, locals.var_vfb2_loc__blk891_dn8, locals.var_vfb2_loc__blk891_dn9,)
    }
};
        locals.var_vfb2_loc__blk891 = assign28530_e30555;
        locals.var_vfb2_loc__blk891_dn4 = assign28530_e30555_d_n4;
        locals.var_vfb2_loc__blk891_dn6 = assign28530_e30555_d_n6;
        locals.var_vfb2_loc__blk891_dn7 = assign28530_e30555_d_n7;
        locals.var_vfb2_loc__blk891_dn8 = assign28530_e30555_d_n8;
        locals.var_vfb2_loc__blk891_dn9 = assign28530_e30555_d_n9;

        let (assign28540_e30559,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_psceac1_i,)
    } else {
        (locals.var_psce1_loc__blk892,)
    }
};
        locals.var_psce1_loc__blk892 = assign28540_e30559;

        let (assign28550_e30563,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_psceac2_i,)
    } else {
        (locals.var_psce2_loc__blk893,)
    }
};
        locals.var_psce2_loc__blk893 = assign28550_e30563;

        let (assign28560_e30567, assign28560_e30567_d_n4, assign28560_e30567_d_n6, assign28560_e30567_d_n7, assign28560_e30567_d_n8, assign28560_e30567_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_cfac1_i, locals.var_cfac1_i_dn4, locals.var_cfac1_i_dn6, locals.var_cfac1_i_dn7, locals.var_cfac1_i_dn8, locals.var_cfac1_i_dn9,)
    } else {
        (locals.var_cf1_loc__blk894, locals.var_cf1_loc__blk894_dn4, locals.var_cf1_loc__blk894_dn6, locals.var_cf1_loc__blk894_dn7, locals.var_cf1_loc__blk894_dn8, locals.var_cf1_loc__blk894_dn9,)
    }
};
        locals.var_cf1_loc__blk894 = assign28560_e30567;
        locals.var_cf1_loc__blk894_dn4 = assign28560_e30567_d_n4;
        locals.var_cf1_loc__blk894_dn6 = assign28560_e30567_d_n6;
        locals.var_cf1_loc__blk894_dn7 = assign28560_e30567_d_n7;
        locals.var_cf1_loc__blk894_dn8 = assign28560_e30567_d_n8;
        locals.var_cf1_loc__blk894_dn9 = assign28560_e30567_d_n9;

        let (assign28570_e30571, assign28570_e30571_d_n4, assign28570_e30571_d_n6, assign28570_e30571_d_n7, assign28570_e30571_d_n8, assign28570_e30571_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_cfac2_i, locals.var_cfac2_i_dn4, locals.var_cfac2_i_dn6, locals.var_cfac2_i_dn7, locals.var_cfac2_i_dn8, locals.var_cfac2_i_dn9,)
    } else {
        (locals.var_cf2_loc__blk895, locals.var_cf2_loc__blk895_dn4, locals.var_cf2_loc__blk895_dn6, locals.var_cf2_loc__blk895_dn7, locals.var_cf2_loc__blk895_dn8, locals.var_cf2_loc__blk895_dn9,)
    }
};
        locals.var_cf2_loc__blk895 = assign28570_e30571;
        locals.var_cf2_loc__blk895_dn4 = assign28570_e30571_d_n4;
        locals.var_cf2_loc__blk895_dn6 = assign28570_e30571_d_n6;
        locals.var_cf2_loc__blk895_dn7 = assign28570_e30571_d_n7;
        locals.var_cf2_loc__blk895_dn8 = assign28570_e30571_d_n8;
        locals.var_cf2_loc__blk895_dn9 = assign28570_e30571_d_n9;

        let (assign28580_e30575, assign28580_e30575_d_n4, assign28580_e30575_d_n6, assign28580_e30575_d_n7, assign28580_e30575_d_n8, assign28580_e30575_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_sat_phit_ac, locals.var_sat_phit_ac_dn4, locals.var_sat_phit_ac_dn6, locals.var_sat_phit_ac_dn7, locals.var_sat_phit_ac_dn8, locals.var_sat_phit_ac_dn9,)
    } else {
        (locals.var_sat_phit_loc__blk896, locals.var_sat_phit_loc__blk896_dn4, locals.var_sat_phit_loc__blk896_dn6, locals.var_sat_phit_loc__blk896_dn7, locals.var_sat_phit_loc__blk896_dn8, locals.var_sat_phit_loc__blk896_dn9,)
    }
};
        locals.var_sat_phit_loc__blk896 = assign28580_e30575;
        locals.var_sat_phit_loc__blk896_dn4 = assign28580_e30575_d_n4;
        locals.var_sat_phit_loc__blk896_dn6 = assign28580_e30575_d_n6;
        locals.var_sat_phit_loc__blk896_dn7 = assign28580_e30575_d_n7;
        locals.var_sat_phit_loc__blk896_dn8 = assign28580_e30575_d_n8;
        locals.var_sat_phit_loc__blk896_dn9 = assign28580_e30575_d_n9;

        let (assign28590_e30579,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_gamax_ac,)
    } else {
        (locals.var_gamax_loc__blk897,)
    }
};
        locals.var_gamax_loc__blk897 = assign28590_e30579;

        let (assign28600_e30583,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_alpac_i,)
    } else {
        (locals.var_alp_loc__blk898,)
    }
};
        locals.var_alp_loc__blk898 = assign28600_e30583;

        let (assign28610_e30595, assign28610_e30595_d_n4, assign28610_e30595_d_n6, assign28610_e30595_d_n7, assign28610_e30595_d_n8, assign28610_e30595_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign28610_e30587: f64 = (locals.var_vgs - locals.var_vfb1_loc__blk890);
        let assign28610_e30589: f64 = (assign28610_e30587 * locals.var_inv_phit);
        let assign28610_e30591: f64 = (assign28610_e30589 - locals.var_dxdsx);
        let assign28610_e30593: f64 = (assign28610_e30591 - locals.var_eg_2phit0);
        (assign28610_e30593, (((((-locals.var_vfb1_loc__blk890_dn4) * locals.var_inv_phit) + (assign28610_e30587 * locals.var_inv_phit_dn4)) - locals.var_dxdsx_dn4) - locals.var_eg_2phit0_dn4), (((((locals.var_vgs_dn6 - locals.var_vfb1_loc__blk890_dn6) * locals.var_inv_phit) + (assign28610_e30587 * locals.var_inv_phit_dn6)) - locals.var_dxdsx_dn6) - locals.var_eg_2phit0_dn6), (((((locals.var_vgs_dn7 - locals.var_vfb1_loc__blk890_dn7) * locals.var_inv_phit) + (assign28610_e30587 * locals.var_inv_phit_dn7)) - locals.var_dxdsx_dn7) - locals.var_eg_2phit0_dn7), (((((-locals.var_vfb1_loc__blk890_dn8) * locals.var_inv_phit) + (assign28610_e30587 * locals.var_inv_phit_dn8)) - locals.var_dxdsx_dn8) - locals.var_eg_2phit0_dn8), (((((locals.var_vgs_dn9 - locals.var_vfb1_loc__blk890_dn9) * locals.var_inv_phit) + (assign28610_e30587 * locals.var_inv_phit_dn9)) - locals.var_dxdsx_dn9) - locals.var_eg_2phit0_dn9),)
    } else {
        (locals.var_xg10__blk899, locals.var_xg10__blk899_dn4, locals.var_xg10__blk899_dn6, locals.var_xg10__blk899_dn7, locals.var_xg10__blk899_dn8, locals.var_xg10__blk899_dn9,)
    }
};
        locals.var_xg10__blk899 = assign28610_e30595;
        locals.var_xg10__blk899_dn4 = assign28610_e30595_d_n4;
        locals.var_xg10__blk899_dn6 = assign28610_e30595_d_n6;
        locals.var_xg10__blk899_dn7 = assign28610_e30595_d_n7;
        locals.var_xg10__blk899_dn8 = assign28610_e30595_d_n8;
        locals.var_xg10__blk899_dn9 = assign28610_e30595_d_n9;

        let (assign28620_e30606, assign28620_e30606_d_n4, assign28620_e30606_d_n6, assign28620_e30606_d_n7, assign28620_e30606_d_n8, assign28620_e30606_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign28620_e30598: f64 = (-locals.var_vsb);
        let assign28620_e30600: f64 = (assign28620_e30598 - locals.var_vfb2_loc__blk891);
        let assign28620_e30602: f64 = (assign28620_e30600 * locals.var_inv_phit);
        let assign28620_e30604: f64 = (assign28620_e30602 - locals.var_dxdsx);
        (assign28620_e30604, ((((-locals.var_vfb2_loc__blk891_dn4) * locals.var_inv_phit) + (assign28620_e30600 * locals.var_inv_phit_dn4)) - locals.var_dxdsx_dn4), (((((-locals.var_vsb_dn6) - locals.var_vfb2_loc__blk891_dn6) * locals.var_inv_phit) + (assign28620_e30600 * locals.var_inv_phit_dn6)) - locals.var_dxdsx_dn6), (((((-locals.var_vsb_dn7) - locals.var_vfb2_loc__blk891_dn7) * locals.var_inv_phit) + (assign28620_e30600 * locals.var_inv_phit_dn7)) - locals.var_dxdsx_dn7), (((((-locals.var_vsb_dn8) - locals.var_vfb2_loc__blk891_dn8) * locals.var_inv_phit) + (assign28620_e30600 * locals.var_inv_phit_dn8)) - locals.var_dxdsx_dn8), ((((-locals.var_vfb2_loc__blk891_dn9) * locals.var_inv_phit) + (assign28620_e30600 * locals.var_inv_phit_dn9)) - locals.var_dxdsx_dn9),)
    } else {
        (locals.var_xg20shift__blk900, locals.var_xg20shift__blk900_dn4, locals.var_xg20shift__blk900_dn6, locals.var_xg20shift__blk900_dn7, locals.var_xg20shift__blk900_dn8, locals.var_xg20shift__blk900_dn9,)
    }
};
        locals.var_xg20shift__blk900 = assign28620_e30606;
        locals.var_xg20shift__blk900_dn4 = assign28620_e30606_d_n4;
        locals.var_xg20shift__blk900_dn6 = assign28620_e30606_d_n6;
        locals.var_xg20shift__blk900_dn7 = assign28620_e30606_d_n7;
        locals.var_xg20shift__blk900_dn8 = assign28620_e30606_d_n8;
        locals.var_xg20shift__blk900_dn9 = assign28620_e30606_d_n9;

        let (assign28630_e30612, assign28630_e30612_d_n4, assign28630_e30612_d_n6, assign28630_e30612_d_n7, assign28630_e30612_d_n8, assign28630_e30612_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign28630_e30610: f64 = (locals.var_xg20shift__blk900 - locals.var_eg_2phit0);
        (assign28630_e30610, (locals.var_xg20shift__blk900_dn4 - locals.var_eg_2phit0_dn4), (locals.var_xg20shift__blk900_dn6 - locals.var_eg_2phit0_dn6), (locals.var_xg20shift__blk900_dn7 - locals.var_eg_2phit0_dn7), (locals.var_xg20shift__blk900_dn8 - locals.var_eg_2phit0_dn8), (locals.var_xg20shift__blk900_dn9 - locals.var_eg_2phit0_dn9),)
    } else {
        (locals.var_xg20__blk901, locals.var_xg20__blk901_dn4, locals.var_xg20__blk901_dn6, locals.var_xg20__blk901_dn7, locals.var_xg20__blk901_dn8, locals.var_xg20__blk901_dn9,)
    }
};
        locals.var_xg20__blk901 = assign28630_e30612;
        locals.var_xg20__blk901_dn4 = assign28630_e30612_d_n4;
        locals.var_xg20__blk901_dn6 = assign28630_e30612_d_n6;
        locals.var_xg20__blk901_dn7 = assign28630_e30612_d_n7;
        locals.var_xg20__blk901_dn8 = assign28630_e30612_d_n8;
        locals.var_xg20__blk901_dn9 = assign28630_e30612_d_n9;

        let assign28640_e30615: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1081 = assign28640_e30615;

        let (assign28650_e30623, assign28650_e30623_d_n4, assign28650_e30623_d_n6, assign28650_e30623_d_n7, assign28650_e30623_d_n8, assign28650_e30623_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28650_e30621: f64 = (p.p14 * locals.var_typesub_i);
        (assign28650_e30621, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign28650_e30623;
        locals.var_temp_dn4 = assign28650_e30623_d_n4;
        locals.var_temp_dn6 = assign28650_e30623_d_n6;
        locals.var_temp_dn7 = assign28650_e30623_d_n7;
        locals.var_temp_dn8 = assign28650_e30623_d_n8;
        locals.var_temp_dn9 = assign28650_e30623_d_n9;

        let (assign28660_e30635, assign28660_e30635_d_n4, assign28660_e30635_d_n6, assign28660_e30635_d_n7, assign28660_e30635_d_n8, assign28660_e30635_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28660_e30629: f64 = (1.0 + locals.var_k1_1d);
        let assign28660_e30632: f64 = (1.0 + locals.var_k2_1d);
        let assign28660_e30633: f64 = (assign28660_e30629 / assign28660_e30632);
        (assign28660_e30633, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_dxth__blk902, locals.var_exp_dxth__blk902_dn4, locals.var_exp_dxth__blk902_dn6, locals.var_exp_dxth__blk902_dn7, locals.var_exp_dxth__blk902_dn8, locals.var_exp_dxth__blk902_dn9,)
    }
};
        locals.var_exp_dxth__blk902 = assign28660_e30635;
        locals.var_exp_dxth__blk902_dn4 = assign28660_e30635_d_n4;
        locals.var_exp_dxth__blk902_dn6 = assign28660_e30635_d_n6;
        locals.var_exp_dxth__blk902_dn7 = assign28660_e30635_d_n7;
        locals.var_exp_dxth__blk902_dn8 = assign28660_e30635_d_n8;
        locals.var_exp_dxth__blk902_dn9 = assign28660_e30635_d_n9;

        let (assign28670_e30642, assign28670_e30642_d_n4, assign28670_e30642_d_n6, assign28670_e30642_d_n7, assign28670_e30642_d_n8, assign28670_e30642_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28670_e30640: f64 = (locals.var_exp_dxth__blk902).ln();
        (assign28670_e30640, (locals.var_exp_dxth__blk902_dn4 / locals.var_exp_dxth__blk902), (locals.var_exp_dxth__blk902_dn6 / locals.var_exp_dxth__blk902), (locals.var_exp_dxth__blk902_dn7 / locals.var_exp_dxth__blk902), (locals.var_exp_dxth__blk902_dn8 / locals.var_exp_dxth__blk902), (locals.var_exp_dxth__blk902_dn9 / locals.var_exp_dxth__blk902),)
    } else {
        (locals.var_dxth__blk903, locals.var_dxth__blk903_dn4, locals.var_dxth__blk903_dn6, locals.var_dxth__blk903_dn7, locals.var_dxth__blk903_dn8, locals.var_dxth__blk903_dn9,)
    }
};
        locals.var_dxth__blk903 = assign28670_e30642;
        locals.var_dxth__blk903_dn4 = assign28670_e30642_d_n4;
        locals.var_dxth__blk903_dn6 = assign28670_e30642_d_n6;
        locals.var_dxth__blk903_dn7 = assign28670_e30642_d_n7;
        locals.var_dxth__blk903_dn8 = assign28670_e30642_d_n8;
        locals.var_dxth__blk903_dn9 = assign28670_e30642_d_n9;

        let assign28680_e30645: f64 = if locals.var_dxth__blk903 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1082 = assign28680_e30645;

        let (assign28690_e30663, assign28690_e30663_d_n4, assign28690_e30663_d_n6, assign28690_e30663_d_n7, assign28690_e30663_d_n8, assign28690_e30663_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) {
        let assign28690_e30653: f64 = (2.0 * locals.var_dxth__blk903);
        let assign28690_e30656: f64 = (locals.var_exp_dxth__blk902 + 1.0);
        let assign28690_e30657: f64 = (assign28690_e30653 * assign28690_e30656);
        let assign28690_e30660: f64 = (locals.var_exp_dxth__blk902 - 1.0);
        let assign28690_e30661: f64 = (assign28690_e30657 / assign28690_e30660);
        (assign28690_e30661, ((((((2.0 * locals.var_dxth__blk903_dn4) * assign28690_e30656) + (assign28690_e30653 * locals.var_exp_dxth__blk902_dn4)) * assign28690_e30660) - (assign28690_e30657 * locals.var_exp_dxth__blk902_dn4)) / (assign28690_e30660 * assign28690_e30660)), ((((((2.0 * locals.var_dxth__blk903_dn6) * assign28690_e30656) + (assign28690_e30653 * locals.var_exp_dxth__blk902_dn6)) * assign28690_e30660) - (assign28690_e30657 * locals.var_exp_dxth__blk902_dn6)) / (assign28690_e30660 * assign28690_e30660)), ((((((2.0 * locals.var_dxth__blk903_dn7) * assign28690_e30656) + (assign28690_e30653 * locals.var_exp_dxth__blk902_dn7)) * assign28690_e30660) - (assign28690_e30657 * locals.var_exp_dxth__blk902_dn7)) / (assign28690_e30660 * assign28690_e30660)), ((((((2.0 * locals.var_dxth__blk903_dn8) * assign28690_e30656) + (assign28690_e30653 * locals.var_exp_dxth__blk902_dn8)) * assign28690_e30660) - (assign28690_e30657 * locals.var_exp_dxth__blk902_dn8)) / (assign28690_e30660 * assign28690_e30660)), ((((((2.0 * locals.var_dxth__blk903_dn9) * assign28690_e30656) + (assign28690_e30653 * locals.var_exp_dxth__blk902_dn9)) * assign28690_e30660) - (assign28690_e30657 * locals.var_exp_dxth__blk902_dn9)) / (assign28690_e30660 * assign28690_e30660)),)
    } else {
        (locals.var_diff_min__blk904, locals.var_diff_min__blk904_dn4, locals.var_diff_min__blk904_dn6, locals.var_diff_min__blk904_dn7, locals.var_diff_min__blk904_dn8, locals.var_diff_min__blk904_dn9,)
    }
};
        locals.var_diff_min__blk904 = assign28690_e30663;
        locals.var_diff_min__blk904_dn4 = assign28690_e30663_d_n4;
        locals.var_diff_min__blk904_dn6 = assign28690_e30663_d_n6;
        locals.var_diff_min__blk904_dn7 = assign28690_e30663_d_n7;
        locals.var_diff_min__blk904_dn8 = assign28690_e30663_d_n8;
        locals.var_diff_min__blk904_dn9 = assign28690_e30663_d_n9;

        let (assign28700_e30676, assign28700_e30676_d_n4, assign28700_e30676_d_n6, assign28700_e30676_d_n7, assign28700_e30676_d_n8, assign28700_e30676_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 == 0.0)) {
        let assign28700_e30673: f64 = (2.0 + locals.var_dxth__blk903);
        let assign28700_e30674: f64 = (2.0 * assign28700_e30673);
        (assign28700_e30674, (2.0 * locals.var_dxth__blk903_dn4), (2.0 * locals.var_dxth__blk903_dn6), (2.0 * locals.var_dxth__blk903_dn7), (2.0 * locals.var_dxth__blk903_dn8), (2.0 * locals.var_dxth__blk903_dn9),)
    } else {
        (locals.var_diff_min__blk904, locals.var_diff_min__blk904_dn4, locals.var_diff_min__blk904_dn6, locals.var_diff_min__blk904_dn7, locals.var_diff_min__blk904_dn8, locals.var_diff_min__blk904_dn9,)
    }
};
        locals.var_diff_min__blk904 = assign28700_e30676;
        locals.var_diff_min__blk904_dn4 = assign28700_e30676_d_n4;
        locals.var_diff_min__blk904_dn6 = assign28700_e30676_d_n6;
        locals.var_diff_min__blk904_dn7 = assign28700_e30676_d_n7;
        locals.var_diff_min__blk904_dn8 = assign28700_e30676_d_n8;
        locals.var_diff_min__blk904_dn9 = assign28700_e30676_d_n9;

        let (assign28710_e30686, assign28710_e30686_d_n4, assign28710_e30686_d_n6, assign28710_e30686_d_n7, assign28710_e30686_d_n8, assign28710_e30686_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28710_e30683: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
        let assign28710_e30684: f64 = (locals.var_a0_csisq / assign28710_e30683);
        (assign28710_e30684, (locals.var_a0_csisq_dn4 / assign28710_e30683), (locals.var_a0_csisq_dn6 / assign28710_e30683), (locals.var_a0_csisq_dn7 / assign28710_e30683), (locals.var_a0_csisq_dn8 / assign28710_e30683), (locals.var_a0_csisq_dn9 / assign28710_e30683),)
    } else {
        (locals.var_a0__blk905, locals.var_a0__blk905_dn4, locals.var_a0__blk905_dn6, locals.var_a0__blk905_dn7, locals.var_a0__blk905_dn8, locals.var_a0__blk905_dn9,)
    }
};
        locals.var_a0__blk905 = assign28710_e30686;
        locals.var_a0__blk905_dn4 = assign28710_e30686_d_n4;
        locals.var_a0__blk905_dn6 = assign28710_e30686_d_n6;
        locals.var_a0__blk905_dn7 = assign28710_e30686_d_n7;
        locals.var_a0__blk905_dn8 = assign28710_e30686_d_n8;
        locals.var_a0__blk905_dn9 = assign28710_e30686_d_n9;

    }

    pub(super) fn stamp_transient_block_75(
        locals: &mut StampLocals,
    ) {
        let (assign28720_e30694, assign28720_e30694_d_n4, assign28720_e30694_d_n6, assign28720_e30694_d_n7, assign28720_e30694_d_n8, assign28720_e30694_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28720_e30692: f64 = (1.0 / locals.var_k1_1d);
        (assign28720_e30692, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_inv_k1__blk906, locals.var_inv_k1__blk906_dn4, locals.var_inv_k1__blk906_dn6, locals.var_inv_k1__blk906_dn7, locals.var_inv_k1__blk906_dn8, locals.var_inv_k1__blk906_dn9,)
    }
};
        locals.var_inv_k1__blk906 = assign28720_e30694;
        locals.var_inv_k1__blk906_dn4 = assign28720_e30694_d_n4;
        locals.var_inv_k1__blk906_dn6 = assign28720_e30694_d_n6;
        locals.var_inv_k1__blk906_dn7 = assign28720_e30694_d_n7;
        locals.var_inv_k1__blk906_dn8 = assign28720_e30694_d_n8;
        locals.var_inv_k1__blk906_dn9 = assign28720_e30694_d_n9;

        let (assign28730_e30702, assign28730_e30702_d_n4, assign28730_e30702_d_n6, assign28730_e30702_d_n7, assign28730_e30702_d_n8, assign28730_e30702_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28730_e30700: f64 = (1.0 / locals.var_k2_1d);
        (assign28730_e30700, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_inv_k2__blk907, locals.var_inv_k2__blk907_dn4, locals.var_inv_k2__blk907_dn6, locals.var_inv_k2__blk907_dn7, locals.var_inv_k2__blk907_dn8, locals.var_inv_k2__blk907_dn9,)
    }
};
        locals.var_inv_k2__blk907 = assign28730_e30702;
        locals.var_inv_k2__blk907_dn4 = assign28730_e30702_d_n4;
        locals.var_inv_k2__blk907_dn6 = assign28730_e30702_d_n6;
        locals.var_inv_k2__blk907_dn7 = assign28730_e30702_d_n7;
        locals.var_inv_k2__blk907_dn8 = assign28730_e30702_d_n8;
        locals.var_inv_k2__blk907_dn9 = assign28730_e30702_d_n9;

        let (assign28740_e30714, assign28740_e30714_d_n4, assign28740_e30714_d_n6, assign28740_e30714_d_n7, assign28740_e30714_d_n8, assign28740_e30714_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28740_e30709: f64 = (1.0 + locals.var_inv_k1__blk906);
        let assign28740_e30711: f64 = (assign28740_e30709 + locals.var_inv_k2__blk907);
        let assign28740_e30712: f64 = (1.0 / assign28740_e30711);
        (assign28740_e30712, (-((locals.var_inv_k1__blk906_dn4 + locals.var_inv_k2__blk907_dn4) / (assign28740_e30711 * assign28740_e30711))), (-((locals.var_inv_k1__blk906_dn6 + locals.var_inv_k2__blk907_dn6) / (assign28740_e30711 * assign28740_e30711))), (-((locals.var_inv_k1__blk906_dn7 + locals.var_inv_k2__blk907_dn7) / (assign28740_e30711 * assign28740_e30711))), (-((locals.var_inv_k1__blk906_dn8 + locals.var_inv_k2__blk907_dn8) / (assign28740_e30711 * assign28740_e30711))), (-((locals.var_inv_k1__blk906_dn9 + locals.var_inv_k2__blk907_dn9) / (assign28740_e30711 * assign28740_e30711))),)
    } else {
        (locals.var_keq__blk934, locals.var_keq__blk934_dn4, locals.var_keq__blk934_dn6, locals.var_keq__blk934_dn7, locals.var_keq__blk934_dn8, locals.var_keq__blk934_dn9,)
    }
};
        locals.var_keq__blk934 = assign28740_e30714;
        locals.var_keq__blk934_dn4 = assign28740_e30714_d_n4;
        locals.var_keq__blk934_dn6 = assign28740_e30714_d_n6;
        locals.var_keq__blk934_dn7 = assign28740_e30714_d_n7;
        locals.var_keq__blk934_dn8 = assign28740_e30714_d_n8;
        locals.var_keq__blk934_dn9 = assign28740_e30714_d_n9;

        let (assign28750_e30724, assign28750_e30724_d_n4, assign28750_e30724_d_n6, assign28750_e30724_d_n7, assign28750_e30724_d_n8, assign28750_e30724_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28750_e30721: f64 = (locals.var_xg10__blk899 - locals.var_xg20__blk901);
        let assign28750_e30722: f64 = (locals.var_keq__blk934 * assign28750_e30721);
        (assign28750_e30722, ((locals.var_keq__blk934_dn4 * assign28750_e30721) + (locals.var_keq__blk934 * (locals.var_xg10__blk899_dn4 - locals.var_xg20__blk901_dn4))), ((locals.var_keq__blk934_dn6 * assign28750_e30721) + (locals.var_keq__blk934 * (locals.var_xg10__blk899_dn6 - locals.var_xg20__blk901_dn6))), ((locals.var_keq__blk934_dn7 * assign28750_e30721) + (locals.var_keq__blk934 * (locals.var_xg10__blk899_dn7 - locals.var_xg20__blk901_dn7))), ((locals.var_keq__blk934_dn8 * assign28750_e30721) + (locals.var_keq__blk934 * (locals.var_xg10__blk899_dn8 - locals.var_xg20__blk901_dn8))), ((locals.var_keq__blk934_dn9 * assign28750_e30721) + (locals.var_keq__blk934 * (locals.var_xg10__blk899_dn9 - locals.var_xg20__blk901_dn9))),)
    } else {
        (locals.var_dx_wi__blk935, locals.var_dx_wi__blk935_dn4, locals.var_dx_wi__blk935_dn6, locals.var_dx_wi__blk935_dn7, locals.var_dx_wi__blk935_dn8, locals.var_dx_wi__blk935_dn9,)
    }
};
        locals.var_dx_wi__blk935 = assign28750_e30724;
        locals.var_dx_wi__blk935_dn4 = assign28750_e30724_d_n4;
        locals.var_dx_wi__blk935_dn6 = assign28750_e30724_d_n6;
        locals.var_dx_wi__blk935_dn7 = assign28750_e30724_d_n7;
        locals.var_dx_wi__blk935_dn8 = assign28750_e30724_d_n8;
        locals.var_dx_wi__blk935_dn9 = assign28750_e30724_d_n9;

        let (assign28760_e30734, assign28760_e30734_d_n4, assign28760_e30734_d_n6, assign28760_e30734_d_n7, assign28760_e30734_d_n8, assign28760_e30734_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28760_e30731: f64 = (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906);
        let assign28760_e30732: f64 = (locals.var_xg10__blk899 - assign28760_e30731);
        (assign28760_e30732, (locals.var_xg10__blk899_dn4 - ((locals.var_dx_wi__blk935_dn4 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn4))), (locals.var_xg10__blk899_dn6 - ((locals.var_dx_wi__blk935_dn6 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn6))), (locals.var_xg10__blk899_dn7 - ((locals.var_dx_wi__blk935_dn7 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn7))), (locals.var_xg10__blk899_dn8 - ((locals.var_dx_wi__blk935_dn8 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn8))), (locals.var_xg10__blk899_dn9 - ((locals.var_dx_wi__blk935_dn9 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn9))),)
    } else {
        (locals.var_x1_wi0__blk908, locals.var_x1_wi0__blk908_dn4, locals.var_x1_wi0__blk908_dn6, locals.var_x1_wi0__blk908_dn7, locals.var_x1_wi0__blk908_dn8, locals.var_x1_wi0__blk908_dn9,)
    }
};
        locals.var_x1_wi0__blk908 = assign28760_e30734;
        locals.var_x1_wi0__blk908_dn4 = assign28760_e30734_d_n4;
        locals.var_x1_wi0__blk908_dn6 = assign28760_e30734_d_n6;
        locals.var_x1_wi0__blk908_dn7 = assign28760_e30734_d_n7;
        locals.var_x1_wi0__blk908_dn8 = assign28760_e30734_d_n8;
        locals.var_x1_wi0__blk908_dn9 = assign28760_e30734_d_n9;

        let (assign28770_e30744, assign28770_e30744_d_n4, assign28770_e30744_d_n6, assign28770_e30744_d_n7, assign28770_e30744_d_n8, assign28770_e30744_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28770_e30741: f64 = (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907);
        let assign28770_e30742: f64 = (locals.var_xg20__blk901 + assign28770_e30741);
        (assign28770_e30742, (locals.var_xg20__blk901_dn4 + ((locals.var_dx_wi__blk935_dn4 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn4))), (locals.var_xg20__blk901_dn6 + ((locals.var_dx_wi__blk935_dn6 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn6))), (locals.var_xg20__blk901_dn7 + ((locals.var_dx_wi__blk935_dn7 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn7))), (locals.var_xg20__blk901_dn8 + ((locals.var_dx_wi__blk935_dn8 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn8))), (locals.var_xg20__blk901_dn9 + ((locals.var_dx_wi__blk935_dn9 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn9))),)
    } else {
        (locals.var_x2_wi0__blk909, locals.var_x2_wi0__blk909_dn4, locals.var_x2_wi0__blk909_dn6, locals.var_x2_wi0__blk909_dn7, locals.var_x2_wi0__blk909_dn8, locals.var_x2_wi0__blk909_dn9,)
    }
};
        locals.var_x2_wi0__blk909 = assign28770_e30744;
        locals.var_x2_wi0__blk909_dn4 = assign28770_e30744_d_n4;
        locals.var_x2_wi0__blk909_dn6 = assign28770_e30744_d_n6;
        locals.var_x2_wi0__blk909_dn7 = assign28770_e30744_d_n7;
        locals.var_x2_wi0__blk909_dn8 = assign28770_e30744_d_n8;
        locals.var_x2_wi0__blk909_dn9 = assign28770_e30744_d_n9;

        let (assign28780_e30754, assign28780_e30754_d_n4, assign28780_e30754_d_n6, assign28780_e30754_d_n7, assign28780_e30754_d_n8, assign28780_e30754_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28780_e30751: f64 = (locals.var_k1_1d + 1.0);
        let assign28780_e30752: f64 = (1.0 / assign28780_e30751);
        (assign28780_e30752, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign28780_e30754;
        locals.var_q_temp1__blk814_dn4 = assign28780_e30754_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign28780_e30754_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign28780_e30754_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign28780_e30754_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign28780_e30754_d_n9;

        let (assign28790_e30764, assign28790_e30764_d_n4, assign28790_e30764_d_n6, assign28790_e30764_d_n7, assign28790_e30764_d_n8, assign28790_e30764_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28790_e30761: f64 = (locals.var_k2_1d + 1.0);
        let assign28790_e30762: f64 = (1.0 / assign28790_e30761);
        (assign28790_e30762, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign28790_e30764;
        locals.var_q_temp2__blk815_dn4 = assign28790_e30764_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign28790_e30764_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign28790_e30764_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign28790_e30764_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign28790_e30764_d_n9;

        let (assign28800_e30783, assign28800_e30783_d_n4, assign28800_e30783_d_n6, assign28800_e30783_d_n7, assign28800_e30783_d_n8, assign28800_e30783_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28800_e30771: f64 = (locals.var_k2_1d * locals.var_q_temp2__blk815);
        let assign28800_e30772: f64 = (locals.var_k1_1d + assign28800_e30771);
        let assign28800_e30774: f64 = (assign28800_e30772 * locals.var_diff_min__blk904);
        let assign28800_e30776: f64 = (assign28800_e30774 / locals.var_a0__blk905);
        let assign28800_e30777: f64 = (assign28800_e30776).ln();
        let assign28800_e30779: f64 = assign28800_e30777;
        let assign28800_e30781: f64 = (assign28800_e30779 + 1.5);
        (assign28800_e30781, (((((((locals.var_k2_1d * locals.var_q_temp2__blk815_dn4) * locals.var_diff_min__blk904) + (assign28800_e30772 * locals.var_diff_min__blk904_dn4)) * locals.var_a0__blk905) - (assign28800_e30774 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28800_e30776), (((((((locals.var_k2_1d * locals.var_q_temp2__blk815_dn6) * locals.var_diff_min__blk904) + (assign28800_e30772 * locals.var_diff_min__blk904_dn6)) * locals.var_a0__blk905) - (assign28800_e30774 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28800_e30776), (((((((locals.var_k2_1d * locals.var_q_temp2__blk815_dn7) * locals.var_diff_min__blk904) + (assign28800_e30772 * locals.var_diff_min__blk904_dn7)) * locals.var_a0__blk905) - (assign28800_e30774 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28800_e30776), (((((((locals.var_k2_1d * locals.var_q_temp2__blk815_dn8) * locals.var_diff_min__blk904) + (assign28800_e30772 * locals.var_diff_min__blk904_dn8)) * locals.var_a0__blk905) - (assign28800_e30774 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28800_e30776), (((((((locals.var_k2_1d * locals.var_q_temp2__blk815_dn9) * locals.var_diff_min__blk904) + (assign28800_e30772 * locals.var_diff_min__blk904_dn9)) * locals.var_a0__blk905) - (assign28800_e30774 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28800_e30776),)
    } else {
        (locals.var_q_x1sat__blk817, locals.var_q_x1sat__blk817_dn4, locals.var_q_x1sat__blk817_dn6, locals.var_q_x1sat__blk817_dn7, locals.var_q_x1sat__blk817_dn8, locals.var_q_x1sat__blk817_dn9,)
    }
};
        locals.var_q_x1sat__blk817 = assign28800_e30783;
        locals.var_q_x1sat__blk817_dn4 = assign28800_e30783_d_n4;
        locals.var_q_x1sat__blk817_dn6 = assign28800_e30783_d_n6;
        locals.var_q_x1sat__blk817_dn7 = assign28800_e30783_d_n7;
        locals.var_q_x1sat__blk817_dn8 = assign28800_e30783_d_n8;
        locals.var_q_x1sat__blk817_dn9 = assign28800_e30783_d_n9;

        let (assign28810_e30802, assign28810_e30802_d_n4, assign28810_e30802_d_n6, assign28810_e30802_d_n7, assign28810_e30802_d_n8, assign28810_e30802_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28810_e30790: f64 = (locals.var_k1_1d * locals.var_q_temp1__blk814);
        let assign28810_e30791: f64 = (locals.var_k2_1d + assign28810_e30790);
        let assign28810_e30793: f64 = (assign28810_e30791 * locals.var_diff_min__blk904);
        let assign28810_e30795: f64 = (assign28810_e30793 / locals.var_a0__blk905);
        let assign28810_e30796: f64 = (assign28810_e30795).ln();
        let assign28810_e30798: f64 = assign28810_e30796;
        let assign28810_e30800: f64 = (assign28810_e30798 + 1.5);
        (assign28810_e30800, (((((((locals.var_k1_1d * locals.var_q_temp1__blk814_dn4) * locals.var_diff_min__blk904) + (assign28810_e30791 * locals.var_diff_min__blk904_dn4)) * locals.var_a0__blk905) - (assign28810_e30793 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28810_e30795), (((((((locals.var_k1_1d * locals.var_q_temp1__blk814_dn6) * locals.var_diff_min__blk904) + (assign28810_e30791 * locals.var_diff_min__blk904_dn6)) * locals.var_a0__blk905) - (assign28810_e30793 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28810_e30795), (((((((locals.var_k1_1d * locals.var_q_temp1__blk814_dn7) * locals.var_diff_min__blk904) + (assign28810_e30791 * locals.var_diff_min__blk904_dn7)) * locals.var_a0__blk905) - (assign28810_e30793 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28810_e30795), (((((((locals.var_k1_1d * locals.var_q_temp1__blk814_dn8) * locals.var_diff_min__blk904) + (assign28810_e30791 * locals.var_diff_min__blk904_dn8)) * locals.var_a0__blk905) - (assign28810_e30793 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28810_e30795), (((((((locals.var_k1_1d * locals.var_q_temp1__blk814_dn9) * locals.var_diff_min__blk904) + (assign28810_e30791 * locals.var_diff_min__blk904_dn9)) * locals.var_a0__blk905) - (assign28810_e30793 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign28810_e30795),)
    } else {
        (locals.var_q_x2sat__blk818, locals.var_q_x2sat__blk818_dn4, locals.var_q_x2sat__blk818_dn6, locals.var_q_x2sat__blk818_dn7, locals.var_q_x2sat__blk818_dn8, locals.var_q_x2sat__blk818_dn9,)
    }
};
        locals.var_q_x2sat__blk818 = assign28810_e30802;
        locals.var_q_x2sat__blk818_dn4 = assign28810_e30802_d_n4;
        locals.var_q_x2sat__blk818_dn6 = assign28810_e30802_d_n6;
        locals.var_q_x2sat__blk818_dn7 = assign28810_e30802_d_n7;
        locals.var_q_x2sat__blk818_dn8 = assign28810_e30802_d_n8;
        locals.var_q_x2sat__blk818_dn9 = assign28810_e30802_d_n9;

        let assign28820_e30805: f64 = (locals.var_q_x1sat__blk817 - locals.var_x1_wi0__blk908);
        let assign28820_e30807: f64 = (assign28820_e30805 / 1.5);
        let assign28820_e30809: f64 = if assign28820_e30807 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1083 = assign28820_e30809;

        let (assign28830_e30825, assign28830_e30825_d_n4, assign28830_e30825_d_n6, assign28830_e30825_d_n7, assign28830_e30825_d_n8, assign28830_e30825_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1083 != 0.0)) {
        let assign28830_e30818: f64 = (locals.var_q_x1sat__blk817 - locals.var_x1_wi0__blk908);
        let assign28830_e30820: f64 = (assign28830_e30818 / 1.5);
        let assign28830_e30821: f64 = (assign28830_e30820).exp();
        let assign28830_e30822: f64 = (1.0 + assign28830_e30821);
        let assign28830_e30823: f64 = (assign28830_e30822).ln();
        (assign28830_e30823, ((assign28830_e30821 * ((locals.var_q_x1sat__blk817_dn4 - locals.var_x1_wi0__blk908_dn4) / 1.5)) / assign28830_e30822), ((assign28830_e30821 * ((locals.var_q_x1sat__blk817_dn6 - locals.var_x1_wi0__blk908_dn6) / 1.5)) / assign28830_e30822), ((assign28830_e30821 * ((locals.var_q_x1sat__blk817_dn7 - locals.var_x1_wi0__blk908_dn7) / 1.5)) / assign28830_e30822), ((assign28830_e30821 * ((locals.var_q_x1sat__blk817_dn8 - locals.var_x1_wi0__blk908_dn8) / 1.5)) / assign28830_e30822), ((assign28830_e30821 * ((locals.var_q_x1sat__blk817_dn9 - locals.var_x1_wi0__blk908_dn9) / 1.5)) / assign28830_e30822),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign28830_e30825;
        locals.var_q_temp3__blk816_dn4 = assign28830_e30825_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign28830_e30825_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign28830_e30825_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign28830_e30825_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign28830_e30825_d_n9;

        let (assign28840_e30838, assign28840_e30838_d_n4, assign28840_e30838_d_n6, assign28840_e30838_d_n7, assign28840_e30838_d_n8, assign28840_e30838_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1083 == 0.0)) {
        let assign28840_e30834: f64 = (locals.var_q_x1sat__blk817 - locals.var_x1_wi0__blk908);
        let assign28840_e30836: f64 = (assign28840_e30834 / 1.5);
        (assign28840_e30836, ((locals.var_q_x1sat__blk817_dn4 - locals.var_x1_wi0__blk908_dn4) / 1.5), ((locals.var_q_x1sat__blk817_dn6 - locals.var_x1_wi0__blk908_dn6) / 1.5), ((locals.var_q_x1sat__blk817_dn7 - locals.var_x1_wi0__blk908_dn7) / 1.5), ((locals.var_q_x1sat__blk817_dn8 - locals.var_x1_wi0__blk908_dn8) / 1.5), ((locals.var_q_x1sat__blk817_dn9 - locals.var_x1_wi0__blk908_dn9) / 1.5),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign28840_e30838;
        locals.var_q_temp3__blk816_dn4 = assign28840_e30838_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign28840_e30838_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign28840_e30838_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign28840_e30838_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign28840_e30838_d_n9;

        let (assign28850_e30848, assign28850_e30848_d_n4, assign28850_e30848_d_n6, assign28850_e30848_d_n7, assign28850_e30848_d_n8, assign28850_e30848_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28850_e30845: f64 = (1.5 * locals.var_q_temp3__blk816);
        let assign28850_e30846: f64 = (locals.var_q_x1sat__blk817 - assign28850_e30845);
        (assign28850_e30846, (locals.var_q_x1sat__blk817_dn4 - (1.5 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x1sat__blk817_dn6 - (1.5 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x1sat__blk817_dn7 - (1.5 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x1sat__blk817_dn8 - (1.5 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x1sat__blk817_dn9 - (1.5 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_x1__blk821, locals.var_q_x1__blk821_dn4, locals.var_q_x1__blk821_dn6, locals.var_q_x1__blk821_dn7, locals.var_q_x1__blk821_dn8, locals.var_q_x1__blk821_dn9,)
    }
};
        locals.var_q_x1__blk821 = assign28850_e30848;
        locals.var_q_x1__blk821_dn4 = assign28850_e30848_d_n4;
        locals.var_q_x1__blk821_dn6 = assign28850_e30848_d_n6;
        locals.var_q_x1__blk821_dn7 = assign28850_e30848_d_n7;
        locals.var_q_x1__blk821_dn8 = assign28850_e30848_d_n8;
        locals.var_q_x1__blk821_dn9 = assign28850_e30848_d_n9;

        let (assign28860_e30860, assign28860_e30860_d_n4, assign28860_e30860_d_n6, assign28860_e30860_d_n7, assign28860_e30860_d_n8, assign28860_e30860_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28860_e30854: f64 = (locals.var_k2_1d * locals.var_xg20__blk901);
        let assign28860_e30856: f64 = (assign28860_e30854 + locals.var_q_x1__blk821);
        let assign28860_e30858: f64 = (assign28860_e30856 * locals.var_q_temp2__blk815);
        (assign28860_e30858, ((((locals.var_k2_1d * locals.var_xg20__blk901_dn4) + locals.var_q_x1__blk821_dn4) * locals.var_q_temp2__blk815) + (assign28860_e30856 * locals.var_q_temp2__blk815_dn4)), ((((locals.var_k2_1d * locals.var_xg20__blk901_dn6) + locals.var_q_x1__blk821_dn6) * locals.var_q_temp2__blk815) + (assign28860_e30856 * locals.var_q_temp2__blk815_dn6)), ((((locals.var_k2_1d * locals.var_xg20__blk901_dn7) + locals.var_q_x1__blk821_dn7) * locals.var_q_temp2__blk815) + (assign28860_e30856 * locals.var_q_temp2__blk815_dn7)), ((((locals.var_k2_1d * locals.var_xg20__blk901_dn8) + locals.var_q_x1__blk821_dn8) * locals.var_q_temp2__blk815) + (assign28860_e30856 * locals.var_q_temp2__blk815_dn8)), ((((locals.var_k2_1d * locals.var_xg20__blk901_dn9) + locals.var_q_x1__blk821_dn9) * locals.var_q_temp2__blk815) + (assign28860_e30856 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_x2_wi__blk820, locals.var_q_x2_wi__blk820_dn4, locals.var_q_x2_wi__blk820_dn6, locals.var_q_x2_wi__blk820_dn7, locals.var_q_x2_wi__blk820_dn8, locals.var_q_x2_wi__blk820_dn9,)
    }
};
        locals.var_q_x2_wi__blk820 = assign28860_e30860;
        locals.var_q_x2_wi__blk820_dn4 = assign28860_e30860_d_n4;
        locals.var_q_x2_wi__blk820_dn6 = assign28860_e30860_d_n6;
        locals.var_q_x2_wi__blk820_dn7 = assign28860_e30860_d_n7;
        locals.var_q_x2_wi__blk820_dn8 = assign28860_e30860_d_n8;
        locals.var_q_x2_wi__blk820_dn9 = assign28860_e30860_d_n9;

        let assign28870_e30863: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign28870_e30865: f64 = (assign28870_e30863 / 1.5);
        let assign28870_e30867: f64 = if assign28870_e30865 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1084 = assign28870_e30867;

        let (assign28880_e30883, assign28880_e30883_d_n4, assign28880_e30883_d_n6, assign28880_e30883_d_n7, assign28880_e30883_d_n8, assign28880_e30883_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) {
        let assign28880_e30876: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign28880_e30878: f64 = (assign28880_e30876 / 1.5);
        let assign28880_e30879: f64 = (assign28880_e30878).exp();
        let assign28880_e30880: f64 = (1.0 + assign28880_e30879);
        let assign28880_e30881: f64 = (assign28880_e30880).ln();
        (assign28880_e30881, ((assign28880_e30879 * ((locals.var_q_x2sat__blk818_dn4 - locals.var_q_x2_wi__blk820_dn4) / 1.5)) / assign28880_e30880), ((assign28880_e30879 * ((locals.var_q_x2sat__blk818_dn6 - locals.var_q_x2_wi__blk820_dn6) / 1.5)) / assign28880_e30880), ((assign28880_e30879 * ((locals.var_q_x2sat__blk818_dn7 - locals.var_q_x2_wi__blk820_dn7) / 1.5)) / assign28880_e30880), ((assign28880_e30879 * ((locals.var_q_x2sat__blk818_dn8 - locals.var_q_x2_wi__blk820_dn8) / 1.5)) / assign28880_e30880), ((assign28880_e30879 * ((locals.var_q_x2sat__blk818_dn9 - locals.var_q_x2_wi__blk820_dn9) / 1.5)) / assign28880_e30880),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign28880_e30883;
        locals.var_q_temp3__blk816_dn4 = assign28880_e30883_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign28880_e30883_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign28880_e30883_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign28880_e30883_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign28880_e30883_d_n9;

        let (assign28890_e30896, assign28890_e30896_d_n4, assign28890_e30896_d_n6, assign28890_e30896_d_n7, assign28890_e30896_d_n8, assign28890_e30896_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 == 0.0)) {
        let assign28890_e30892: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign28890_e30894: f64 = (assign28890_e30892 / 1.5);
        (assign28890_e30894, ((locals.var_q_x2sat__blk818_dn4 - locals.var_q_x2_wi__blk820_dn4) / 1.5), ((locals.var_q_x2sat__blk818_dn6 - locals.var_q_x2_wi__blk820_dn6) / 1.5), ((locals.var_q_x2sat__blk818_dn7 - locals.var_q_x2_wi__blk820_dn7) / 1.5), ((locals.var_q_x2sat__blk818_dn8 - locals.var_q_x2_wi__blk820_dn8) / 1.5), ((locals.var_q_x2sat__blk818_dn9 - locals.var_q_x2_wi__blk820_dn9) / 1.5),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign28890_e30896;
        locals.var_q_temp3__blk816_dn4 = assign28890_e30896_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign28890_e30896_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign28890_e30896_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign28890_e30896_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign28890_e30896_d_n9;

        let (assign28900_e30906, assign28900_e30906_d_n4, assign28900_e30906_d_n6, assign28900_e30906_d_n7, assign28900_e30906_d_n8, assign28900_e30906_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28900_e30903: f64 = (1.5 * locals.var_q_temp3__blk816);
        let assign28900_e30904: f64 = (locals.var_q_x2sat__blk818 - assign28900_e30903);
        (assign28900_e30904, (locals.var_q_x2sat__blk818_dn4 - (1.5 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x2sat__blk818_dn6 - (1.5 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x2sat__blk818_dn7 - (1.5 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x2sat__blk818_dn8 - (1.5 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x2sat__blk818_dn9 - (1.5 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_temp0, locals.var_temp0_dn4, locals.var_temp0_dn6, locals.var_temp0_dn7, locals.var_temp0_dn8, locals.var_temp0_dn9,)
    }
};
        locals.var_temp0 = assign28900_e30906;
        locals.var_temp0_dn4 = assign28900_e30906_d_n4;
        locals.var_temp0_dn6 = assign28900_e30906_d_n6;
        locals.var_temp0_dn7 = assign28900_e30906_d_n7;
        locals.var_temp0_dn8 = assign28900_e30906_d_n8;
        locals.var_temp0_dn9 = assign28900_e30906_d_n9;

        let (assign28910_e30914, assign28910_e30914_d_n4, assign28910_e30914_d_n6, assign28910_e30914_d_n7, assign28910_e30914_d_n8, assign28910_e30914_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28910_e30912: f64 = (locals.var_temp * locals.var_temp0);
        (assign28910_e30912, ((locals.var_temp_dn4 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn4)), ((locals.var_temp_dn6 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn6)), ((locals.var_temp_dn7 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn7)), ((locals.var_temp_dn8 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn8)), ((locals.var_temp_dn9 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign28910_e30914;
        locals.var_temp1_dn4 = assign28910_e30914_d_n4;
        locals.var_temp1_dn6 = assign28910_e30914_d_n6;
        locals.var_temp1_dn7 = assign28910_e30914_d_n7;
        locals.var_temp1_dn8 = assign28910_e30914_d_n8;
        locals.var_temp1_dn9 = assign28910_e30914_d_n9;

        let (assign28920_e30922, assign28920_e30922_d_n4, assign28920_e30922_d_n6, assign28920_e30922_d_n7, assign28920_e30922_d_n8, assign28920_e30922_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28920_e30920: f64 = (locals.var_temp * locals.var_xg20__blk901);
        (assign28920_e30920, ((locals.var_temp_dn4 * locals.var_xg20__blk901) + (locals.var_temp * locals.var_xg20__blk901_dn4)), ((locals.var_temp_dn6 * locals.var_xg20__blk901) + (locals.var_temp * locals.var_xg20__blk901_dn6)), ((locals.var_temp_dn7 * locals.var_xg20__blk901) + (locals.var_temp * locals.var_xg20__blk901_dn7)), ((locals.var_temp_dn8 * locals.var_xg20__blk901) + (locals.var_temp * locals.var_xg20__blk901_dn8)), ((locals.var_temp_dn9 * locals.var_xg20__blk901) + (locals.var_temp * locals.var_xg20__blk901_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign28920_e30922;
        locals.var_temp2_dn4 = assign28920_e30922_d_n4;
        locals.var_temp2_dn6 = assign28920_e30922_d_n6;
        locals.var_temp2_dn7 = assign28920_e30922_d_n7;
        locals.var_temp2_dn8 = assign28920_e30922_d_n8;
        locals.var_temp2_dn9 = assign28920_e30922_d_n9;

        let (assign28930_e30930, assign28930_e30930_d_n4, assign28930_e30930_d_n6, assign28930_e30930_d_n7, assign28930_e30930_d_n8, assign28930_e30930_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign28930_e30928: f64 = (locals.var_temp1 - locals.var_temp2);
        (assign28930_e30928, (locals.var_temp1_dn4 - locals.var_temp2_dn4), (locals.var_temp1_dn6 - locals.var_temp2_dn6), (locals.var_temp1_dn7 - locals.var_temp2_dn7), (locals.var_temp1_dn8 - locals.var_temp2_dn8), (locals.var_temp1_dn9 - locals.var_temp2_dn9),)
    } else {
        (locals.var_spsub_xgb__blk866, locals.var_spsub_xgb__blk866_dn4, locals.var_spsub_xgb__blk866_dn6, locals.var_spsub_xgb__blk866_dn7, locals.var_spsub_xgb__blk866_dn8, locals.var_spsub_xgb__blk866_dn9,)
    }
};
        locals.var_spsub_xgb__blk866 = assign28930_e30930;
        locals.var_spsub_xgb__blk866_dn4 = assign28930_e30930_d_n4;
        locals.var_spsub_xgb__blk866_dn6 = assign28930_e30930_d_n6;
        locals.var_spsub_xgb__blk866_dn7 = assign28930_e30930_d_n7;
        locals.var_spsub_xgb__blk866_dn8 = assign28930_e30930_d_n8;
        locals.var_spsub_xgb__blk866_dn9 = assign28930_e30930_d_n9;

        let assign28940_e30932: f64 = (-locals.var_xn_sub);
        let assign28940_e30933: f64 = (assign28940_e30932).abs();
        let assign28940_e30935: f64 = if assign28940_e30933 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1085 = assign28940_e30935;

        let (assign28950_e30945, assign28950_e30945_d_n4, assign28950_e30945_d_n6, assign28950_e30945_d_n7, assign28950_e30945_d_n8, assign28950_e30945_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1085 != 0.0)) {
        let assign28950_e30942: f64 = (-locals.var_xn_sub);
        let assign28950_e30943: f64 = (assign28950_e30942).exp();
        (assign28950_e30943, (assign28950_e30943 * (-locals.var_xn_sub_dn4)), (assign28950_e30943 * (-locals.var_xn_sub_dn6)), (assign28950_e30943 * (-locals.var_xn_sub_dn7)), (assign28950_e30943 * (-locals.var_xn_sub_dn8)), (assign28950_e30943 * (-locals.var_xn_sub_dn9)),)
    } else {
        (locals.var_spsub_delta__blk867, locals.var_spsub_delta__blk867_dn4, locals.var_spsub_delta__blk867_dn6, locals.var_spsub_delta__blk867_dn7, locals.var_spsub_delta__blk867_dn8, locals.var_spsub_delta__blk867_dn9,)
    }
};
        locals.var_spsub_delta__blk867 = assign28950_e30945;
        locals.var_spsub_delta__blk867_dn4 = assign28950_e30945_d_n4;
        locals.var_spsub_delta__blk867_dn6 = assign28950_e30945_d_n6;
        locals.var_spsub_delta__blk867_dn7 = assign28950_e30945_d_n7;
        locals.var_spsub_delta__blk867_dn8 = assign28950_e30945_d_n8;
        locals.var_spsub_delta__blk867_dn9 = assign28950_e30945_d_n9;

        let assign28960_e30947: f64 = (-locals.var_xn_sub);
        let assign28960_e30949: f64 = (-80.0);
        let assign28960_e30950: f64 = if assign28960_e30947 < assign28960_e30949 { 1.0 } else { 0.0 };
        locals.var_guard1086 = assign28960_e30950;

        let (assign28970_e30989, assign28970_e30989_d_n4, assign28970_e30989_d_n6, assign28970_e30989_d_n7, assign28970_e30989_d_n8, assign28970_e30989_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1085 == 0.0)) && (locals.var_guard1086 != 0.0)) {
        let assign28970_e30962: f64 = (-locals.var_xn_sub);
        let assign28970_e30963: f64 = (-assign28970_e30962);
        let assign28970_e30965: f64 = (assign28970_e30963 - 80.0);
        let assign28970_e30969: f64 = (-locals.var_xn_sub);
        let assign28970_e30970: f64 = (-assign28970_e30969);
        let assign28970_e30972: f64 = (assign28970_e30970 - 80.0);
        let assign28970_e30973: f64 = (0.5 * assign28970_e30972);
        let assign28970_e30976: f64 = (-locals.var_xn_sub);
        let assign28970_e30977: f64 = (-assign28970_e30976);
        let assign28970_e30979: f64 = (assign28970_e30977 - 80.0);
        let assign28970_e30981: f64 = (assign28970_e30979 * 0.3333333333333);
        let assign28970_e30982: f64 = (1.0 + assign28970_e30981);
        let assign28970_e30983: f64 = (assign28970_e30973 * assign28970_e30982);
        let assign28970_e30984: f64 = (1.0 + assign28970_e30983);
        let assign28970_e30985: f64 = (assign28970_e30965 * assign28970_e30984);
        let assign28970_e30986: f64 = (1.0 + assign28970_e30985);
        let assign28970_e30987: f64 = (1.80485e-35 / assign28970_e30986);
        (assign28970_e30987, (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn4)) * assign28970_e30984) + (assign28970_e30965 * (((0.5 * (-(-locals.var_xn_sub_dn4))) * assign28970_e30982) + (assign28970_e30973 * ((-(-locals.var_xn_sub_dn4)) * 0.3333333333333)))))) / (assign28970_e30986 * assign28970_e30986))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn6)) * assign28970_e30984) + (assign28970_e30965 * (((0.5 * (-(-locals.var_xn_sub_dn6))) * assign28970_e30982) + (assign28970_e30973 * ((-(-locals.var_xn_sub_dn6)) * 0.3333333333333)))))) / (assign28970_e30986 * assign28970_e30986))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn7)) * assign28970_e30984) + (assign28970_e30965 * (((0.5 * (-(-locals.var_xn_sub_dn7))) * assign28970_e30982) + (assign28970_e30973 * ((-(-locals.var_xn_sub_dn7)) * 0.3333333333333)))))) / (assign28970_e30986 * assign28970_e30986))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn8)) * assign28970_e30984) + (assign28970_e30965 * (((0.5 * (-(-locals.var_xn_sub_dn8))) * assign28970_e30982) + (assign28970_e30973 * ((-(-locals.var_xn_sub_dn8)) * 0.3333333333333)))))) / (assign28970_e30986 * assign28970_e30986))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn9)) * assign28970_e30984) + (assign28970_e30965 * (((0.5 * (-(-locals.var_xn_sub_dn9))) * assign28970_e30982) + (assign28970_e30973 * ((-(-locals.var_xn_sub_dn9)) * 0.3333333333333)))))) / (assign28970_e30986 * assign28970_e30986))),)
    } else {
        (locals.var_spsub_delta__blk867, locals.var_spsub_delta__blk867_dn4, locals.var_spsub_delta__blk867_dn6, locals.var_spsub_delta__blk867_dn7, locals.var_spsub_delta__blk867_dn8, locals.var_spsub_delta__blk867_dn9,)
    }
};
        locals.var_spsub_delta__blk867 = assign28970_e30989;
        locals.var_spsub_delta__blk867_dn4 = assign28970_e30989_d_n4;
        locals.var_spsub_delta__blk867_dn6 = assign28970_e30989_d_n6;
        locals.var_spsub_delta__blk867_dn7 = assign28970_e30989_d_n7;
        locals.var_spsub_delta__blk867_dn8 = assign28970_e30989_d_n8;
        locals.var_spsub_delta__blk867_dn9 = assign28970_e30989_d_n9;

        let (assign28980_e31026, assign28980_e31026_d_n4, assign28980_e31026_d_n6, assign28980_e31026_d_n7, assign28980_e31026_d_n8, assign28980_e31026_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1085 == 0.0)) && (locals.var_guard1086 == 0.0)) {
        let assign28980_e31002: f64 = (-locals.var_xn_sub);
        let assign28980_e31004: f64 = (assign28980_e31002 - 80.0);
        let assign28980_e31008: f64 = (-locals.var_xn_sub);
        let assign28980_e31010: f64 = (assign28980_e31008 - 80.0);
        let assign28980_e31011: f64 = (0.5 * assign28980_e31010);
        let assign28980_e31014: f64 = (-locals.var_xn_sub);
        let assign28980_e31016: f64 = (assign28980_e31014 - 80.0);
        let assign28980_e31018: f64 = (assign28980_e31016 * 0.3333333333333);
        let assign28980_e31019: f64 = (1.0 + assign28980_e31018);
        let assign28980_e31020: f64 = (assign28980_e31011 * assign28980_e31019);
        let assign28980_e31021: f64 = (1.0 + assign28980_e31020);
        let assign28980_e31022: f64 = (assign28980_e31004 * assign28980_e31021);
        let assign28980_e31023: f64 = (1.0 + assign28980_e31022);
        let assign28980_e31024: f64 = (5.54062e34 * assign28980_e31023);
        (assign28980_e31024, (5.54062e34 * (((-locals.var_xn_sub_dn4) * assign28980_e31021) + (assign28980_e31004 * (((0.5 * (-locals.var_xn_sub_dn4)) * assign28980_e31019) + (assign28980_e31011 * ((-locals.var_xn_sub_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn6) * assign28980_e31021) + (assign28980_e31004 * (((0.5 * (-locals.var_xn_sub_dn6)) * assign28980_e31019) + (assign28980_e31011 * ((-locals.var_xn_sub_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn7) * assign28980_e31021) + (assign28980_e31004 * (((0.5 * (-locals.var_xn_sub_dn7)) * assign28980_e31019) + (assign28980_e31011 * ((-locals.var_xn_sub_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn8) * assign28980_e31021) + (assign28980_e31004 * (((0.5 * (-locals.var_xn_sub_dn8)) * assign28980_e31019) + (assign28980_e31011 * ((-locals.var_xn_sub_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn9) * assign28980_e31021) + (assign28980_e31004 * (((0.5 * (-locals.var_xn_sub_dn9)) * assign28980_e31019) + (assign28980_e31011 * ((-locals.var_xn_sub_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_spsub_delta__blk867, locals.var_spsub_delta__blk867_dn4, locals.var_spsub_delta__blk867_dn6, locals.var_spsub_delta__blk867_dn7, locals.var_spsub_delta__blk867_dn8, locals.var_spsub_delta__blk867_dn9,)
    }
};
        locals.var_spsub_delta__blk867 = assign28980_e31026;
        locals.var_spsub_delta__blk867_dn4 = assign28980_e31026_d_n4;
        locals.var_spsub_delta__blk867_dn6 = assign28980_e31026_d_n6;
        locals.var_spsub_delta__blk867_dn7 = assign28980_e31026_d_n7;
        locals.var_spsub_delta__blk867_dn8 = assign28980_e31026_d_n8;
        locals.var_spsub_delta__blk867_dn9 = assign28980_e31026_d_n9;

        let assign28990_e31028: f64 = (locals.var_spsub_xgb__blk866).abs();
        let assign28990_e31030: f64 = if assign28990_e31028 <= locals.var_margin_sub { 1.0 } else { 0.0 };
        locals.var_guard1087 = assign28990_e31030;

        let (assign29000_e31044, assign29000_e31044_d_n4, assign29000_e31044_d_n6, assign29000_e31044_d_n7, assign29000_e31044_d_n8, assign29000_e31044_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 != 0.0)) {
        let assign29000_e31038: f64 = (locals.var_inv_xisub * locals.var_inv_xisub);
        let assign29000_e31040: f64 = (assign29000_e31038 * 0.1666666666667);
        let assign29000_e31042: f64 = (assign29000_e31040 / 1.4142135623731);
        (assign29000_e31042, ((((locals.var_inv_xisub_dn4 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn4)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn6 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn6)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn7 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn7)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn8 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn8)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn9 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn9)) * 0.1666666666667) / 1.4142135623731),)
    } else {
        (locals.var_spsub_temp1__blk864, locals.var_spsub_temp1__blk864_dn4, locals.var_spsub_temp1__blk864_dn6, locals.var_spsub_temp1__blk864_dn7, locals.var_spsub_temp1__blk864_dn8, locals.var_spsub_temp1__blk864_dn9,)
    }
};
        locals.var_spsub_temp1__blk864 = assign29000_e31044;
        locals.var_spsub_temp1__blk864_dn4 = assign29000_e31044_d_n4;
        locals.var_spsub_temp1__blk864_dn6 = assign29000_e31044_d_n6;
        locals.var_spsub_temp1__blk864_dn7 = assign29000_e31044_d_n7;
        locals.var_spsub_temp1__blk864_dn8 = assign29000_e31044_d_n8;
        locals.var_spsub_temp1__blk864_dn9 = assign29000_e31044_d_n9;

        let (assign29010_e31066, assign29010_e31066_d_n4, assign29010_e31066_d_n6, assign29010_e31066_d_n7, assign29010_e31066_d_n8, assign29010_e31066_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 != 0.0)) {
        let assign29010_e31052: f64 = (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub);
        let assign29010_e31057: f64 = (1.0 - locals.var_spsub_delta__blk867);
        let assign29010_e31058: f64 = (locals.var_spsub_xgb__blk866 * assign29010_e31057);
        let assign29010_e31060: f64 = (assign29010_e31058 * locals.var_gfsub);
        let assign29010_e31062: f64 = (assign29010_e31060 * locals.var_spsub_temp1__blk864);
        let assign29010_e31063: f64 = (1.0 + assign29010_e31062);
        let assign29010_e31064: f64 = (assign29010_e31052 * assign29010_e31063);
        (assign29010_e31064, ((((locals.var_spsub_xgb__blk866_dn4 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn4)) * assign29010_e31063) + (assign29010_e31052 * ((((((locals.var_spsub_xgb__blk866_dn4 * assign29010_e31057) + (locals.var_spsub_xgb__blk866 * (-locals.var_spsub_delta__blk867_dn4))) * locals.var_gfsub) + (assign29010_e31058 * locals.var_gfsub_dn4)) * locals.var_spsub_temp1__blk864) + (assign29010_e31060 * locals.var_spsub_temp1__blk864_dn4)))), ((((locals.var_spsub_xgb__blk866_dn6 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn6)) * assign29010_e31063) + (assign29010_e31052 * ((((((locals.var_spsub_xgb__blk866_dn6 * assign29010_e31057) + (locals.var_spsub_xgb__blk866 * (-locals.var_spsub_delta__blk867_dn6))) * locals.var_gfsub) + (assign29010_e31058 * locals.var_gfsub_dn6)) * locals.var_spsub_temp1__blk864) + (assign29010_e31060 * locals.var_spsub_temp1__blk864_dn6)))), ((((locals.var_spsub_xgb__blk866_dn7 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn7)) * assign29010_e31063) + (assign29010_e31052 * ((((((locals.var_spsub_xgb__blk866_dn7 * assign29010_e31057) + (locals.var_spsub_xgb__blk866 * (-locals.var_spsub_delta__blk867_dn7))) * locals.var_gfsub) + (assign29010_e31058 * locals.var_gfsub_dn7)) * locals.var_spsub_temp1__blk864) + (assign29010_e31060 * locals.var_spsub_temp1__blk864_dn7)))), ((((locals.var_spsub_xgb__blk866_dn8 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn8)) * assign29010_e31063) + (assign29010_e31052 * ((((((locals.var_spsub_xgb__blk866_dn8 * assign29010_e31057) + (locals.var_spsub_xgb__blk866 * (-locals.var_spsub_delta__blk867_dn8))) * locals.var_gfsub) + (assign29010_e31058 * locals.var_gfsub_dn8)) * locals.var_spsub_temp1__blk864) + (assign29010_e31060 * locals.var_spsub_temp1__blk864_dn8)))), ((((locals.var_spsub_xgb__blk866_dn9 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn9)) * assign29010_e31063) + (assign29010_e31052 * ((((((locals.var_spsub_xgb__blk866_dn9 * assign29010_e31057) + (locals.var_spsub_xgb__blk866 * (-locals.var_spsub_delta__blk867_dn9))) * locals.var_gfsub) + (assign29010_e31058 * locals.var_gfsub_dn9)) * locals.var_spsub_temp1__blk864) + (assign29010_e31060 * locals.var_spsub_temp1__blk864_dn9)))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign29010_e31066;
        locals.var_temp3_dn4 = assign29010_e31066_d_n4;
        locals.var_temp3_dn6 = assign29010_e31066_d_n6;
        locals.var_temp3_dn7 = assign29010_e31066_d_n7;
        locals.var_temp3_dn8 = assign29010_e31066_d_n8;
        locals.var_temp3_dn9 = assign29010_e31066_d_n9;

        let assign29020_e31069: f64 = (-locals.var_margin_sub);
        let assign29020_e31070: f64 = if locals.var_spsub_xgb__blk866 < assign29020_e31069 { 1.0 } else { 0.0 };
        locals.var_guard1088 = assign29020_e31070;

        let (assign29030_e31082, assign29030_e31082_d_n4, assign29030_e31082_d_n6, assign29030_e31082_d_n7, assign29030_e31082_d_n8, assign29030_e31082_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29030_e31080: f64 = (-locals.var_spsub_xgb__blk866);
        (assign29030_e31080, (-locals.var_spsub_xgb__blk866_dn4), (-locals.var_spsub_xgb__blk866_dn6), (-locals.var_spsub_xgb__blk866_dn7), (-locals.var_spsub_xgb__blk866_dn8), (-locals.var_spsub_xgb__blk866_dn9),)
    } else {
        (locals.var_spsub_yg__blk868, locals.var_spsub_yg__blk868_dn4, locals.var_spsub_yg__blk868_dn6, locals.var_spsub_yg__blk868_dn7, locals.var_spsub_yg__blk868_dn8, locals.var_spsub_yg__blk868_dn9,)
    }
};
        locals.var_spsub_yg__blk868 = assign29030_e31082;
        locals.var_spsub_yg__blk868_dn4 = assign29030_e31082_d_n4;
        locals.var_spsub_yg__blk868_dn6 = assign29030_e31082_d_n6;
        locals.var_spsub_yg__blk868_dn7 = assign29030_e31082_d_n7;
        locals.var_spsub_yg__blk868_dn8 = assign29030_e31082_d_n8;
        locals.var_spsub_yg__blk868_dn9 = assign29030_e31082_d_n9;

        let (assign29040_e31097, assign29040_e31097_d_n4, assign29040_e31097_d_n6, assign29040_e31097_d_n7, assign29040_e31097_d_n8, assign29040_e31097_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29040_e31094: f64 = (locals.var_spsub_yg__blk868 * locals.var_inv_xisub);
        let assign29040_e31095: f64 = (1.25 * assign29040_e31094);
        (assign29040_e31095, (1.25 * ((locals.var_spsub_yg__blk868_dn4 * locals.var_inv_xisub) + (locals.var_spsub_yg__blk868 * locals.var_inv_xisub_dn4))), (1.25 * ((locals.var_spsub_yg__blk868_dn6 * locals.var_inv_xisub) + (locals.var_spsub_yg__blk868 * locals.var_inv_xisub_dn6))), (1.25 * ((locals.var_spsub_yg__blk868_dn7 * locals.var_inv_xisub) + (locals.var_spsub_yg__blk868 * locals.var_inv_xisub_dn7))), (1.25 * ((locals.var_spsub_yg__blk868_dn8 * locals.var_inv_xisub) + (locals.var_spsub_yg__blk868 * locals.var_inv_xisub_dn8))), (1.25 * ((locals.var_spsub_yg__blk868_dn9 * locals.var_inv_xisub) + (locals.var_spsub_yg__blk868 * locals.var_inv_xisub_dn9))),)
    } else {
        (locals.var_spsub_ysub__blk869, locals.var_spsub_ysub__blk869_dn4, locals.var_spsub_ysub__blk869_dn6, locals.var_spsub_ysub__blk869_dn7, locals.var_spsub_ysub__blk869_dn8, locals.var_spsub_ysub__blk869_dn9,)
    }
};
        locals.var_spsub_ysub__blk869 = assign29040_e31097;
        locals.var_spsub_ysub__blk869_dn4 = assign29040_e31097_d_n4;
        locals.var_spsub_ysub__blk869_dn6 = assign29040_e31097_d_n6;
        locals.var_spsub_ysub__blk869_dn7 = assign29040_e31097_d_n7;
        locals.var_spsub_ysub__blk869_dn8 = assign29040_e31097_d_n8;
        locals.var_spsub_ysub__blk869_dn9 = assign29040_e31097_d_n9;

    }

    pub(super) fn stamp_transient_block_76(
        locals: &mut StampLocals,
    ) {
        let (assign29050_e31123, assign29050_e31123_d_n4, assign29050_e31123_d_n6, assign29050_e31123_d_n7, assign29050_e31123_d_n8, assign29050_e31123_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29050_e31109: f64 = (locals.var_spsub_ysub__blk869 + 10.0);
        let assign29050_e31112: f64 = (locals.var_spsub_ysub__blk869 - 6.0);
        let assign29050_e31115: f64 = (locals.var_spsub_ysub__blk869 - 6.0);
        let assign29050_e31116: f64 = (assign29050_e31112 * assign29050_e31115);
        let assign29050_e31118: f64 = (assign29050_e31116 + 64.0);
        let assign29050_e31119: f64 = (assign29050_e31118).sqrt();
        let assign29050_e31120: f64 = (assign29050_e31109 - assign29050_e31119);
        let assign29050_e31121: f64 = (0.5 * assign29050_e31120);
        (assign29050_e31121, (0.5 * (locals.var_spsub_ysub__blk869_dn4 - (((locals.var_spsub_ysub__blk869_dn4 * assign29050_e31115) + (assign29050_e31112 * locals.var_spsub_ysub__blk869_dn4)) / (2.0 * assign29050_e31119)))), (0.5 * (locals.var_spsub_ysub__blk869_dn6 - (((locals.var_spsub_ysub__blk869_dn6 * assign29050_e31115) + (assign29050_e31112 * locals.var_spsub_ysub__blk869_dn6)) / (2.0 * assign29050_e31119)))), (0.5 * (locals.var_spsub_ysub__blk869_dn7 - (((locals.var_spsub_ysub__blk869_dn7 * assign29050_e31115) + (assign29050_e31112 * locals.var_spsub_ysub__blk869_dn7)) / (2.0 * assign29050_e31119)))), (0.5 * (locals.var_spsub_ysub__blk869_dn8 - (((locals.var_spsub_ysub__blk869_dn8 * assign29050_e31115) + (assign29050_e31112 * locals.var_spsub_ysub__blk869_dn8)) / (2.0 * assign29050_e31119)))), (0.5 * (locals.var_spsub_ysub__blk869_dn9 - (((locals.var_spsub_ysub__blk869_dn9 * assign29050_e31115) + (assign29050_e31112 * locals.var_spsub_ysub__blk869_dn9)) / (2.0 * assign29050_e31119)))),)
    } else {
        (locals.var_spsub_eta__blk870, locals.var_spsub_eta__blk870_dn4, locals.var_spsub_eta__blk870_dn6, locals.var_spsub_eta__blk870_dn7, locals.var_spsub_eta__blk870_dn8, locals.var_spsub_eta__blk870_dn9,)
    }
};
        locals.var_spsub_eta__blk870 = assign29050_e31123;
        locals.var_spsub_eta__blk870_dn4 = assign29050_e31123_d_n4;
        locals.var_spsub_eta__blk870_dn6 = assign29050_e31123_d_n6;
        locals.var_spsub_eta__blk870_dn7 = assign29050_e31123_d_n7;
        locals.var_spsub_eta__blk870_dn8 = assign29050_e31123_d_n8;
        locals.var_spsub_eta__blk870_dn9 = assign29050_e31123_d_n9;

        let (assign29060_e31136, assign29060_e31136_d_n4, assign29060_e31136_d_n6, assign29060_e31136_d_n7, assign29060_e31136_d_n8, assign29060_e31136_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29060_e31134: f64 = (locals.var_spsub_yg__blk868 - locals.var_spsub_eta__blk870);
        (assign29060_e31134, (locals.var_spsub_yg__blk868_dn4 - locals.var_spsub_eta__blk870_dn4), (locals.var_spsub_yg__blk868_dn6 - locals.var_spsub_eta__blk870_dn6), (locals.var_spsub_yg__blk868_dn7 - locals.var_spsub_eta__blk870_dn7), (locals.var_spsub_yg__blk868_dn8 - locals.var_spsub_eta__blk870_dn8), (locals.var_spsub_yg__blk868_dn9 - locals.var_spsub_eta__blk870_dn9),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29060_e31136;
        locals.var_spsub_temp__blk863_dn4 = assign29060_e31136_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29060_e31136_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29060_e31136_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29060_e31136_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29060_e31136_d_n9;

        let (assign29070_e31155, assign29070_e31155_d_n4, assign29070_e31155_d_n6, assign29070_e31155_d_n7, assign29070_e31155_d_n8, assign29070_e31155_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29070_e31147: f64 = (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863);
        let assign29070_e31151: f64 = (locals.var_spsub_eta__blk870 + 1.0);
        let assign29070_e31152: f64 = (locals.var_gfsub2 * assign29070_e31151);
        let assign29070_e31153: f64 = (assign29070_e31147 + assign29070_e31152);
        (assign29070_e31153, (((locals.var_spsub_temp__blk863_dn4 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn4)) + ((locals.var_gfsub2_dn4 * assign29070_e31151) + (locals.var_gfsub2 * locals.var_spsub_eta__blk870_dn4))), (((locals.var_spsub_temp__blk863_dn6 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn6)) + ((locals.var_gfsub2_dn6 * assign29070_e31151) + (locals.var_gfsub2 * locals.var_spsub_eta__blk870_dn6))), (((locals.var_spsub_temp__blk863_dn7 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn7)) + ((locals.var_gfsub2_dn7 * assign29070_e31151) + (locals.var_gfsub2 * locals.var_spsub_eta__blk870_dn7))), (((locals.var_spsub_temp__blk863_dn8 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn8)) + ((locals.var_gfsub2_dn8 * assign29070_e31151) + (locals.var_gfsub2 * locals.var_spsub_eta__blk870_dn8))), (((locals.var_spsub_temp__blk863_dn9 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn9)) + ((locals.var_gfsub2_dn9 * assign29070_e31151) + (locals.var_gfsub2 * locals.var_spsub_eta__blk870_dn9))),)
    } else {
        (locals.var_spsub_a__blk871, locals.var_spsub_a__blk871_dn4, locals.var_spsub_a__blk871_dn6, locals.var_spsub_a__blk871_dn7, locals.var_spsub_a__blk871_dn8, locals.var_spsub_a__blk871_dn9,)
    }
};
        locals.var_spsub_a__blk871 = assign29070_e31155;
        locals.var_spsub_a__blk871_dn4 = assign29070_e31155_d_n4;
        locals.var_spsub_a__blk871_dn6 = assign29070_e31155_d_n6;
        locals.var_spsub_a__blk871_dn7 = assign29070_e31155_d_n7;
        locals.var_spsub_a__blk871_dn8 = assign29070_e31155_d_n8;
        locals.var_spsub_a__blk871_dn9 = assign29070_e31155_d_n9;

        let (assign29080_e31170, assign29080_e31170_d_n4, assign29080_e31170_d_n6, assign29080_e31170_d_n7, assign29080_e31170_d_n8, assign29080_e31170_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29080_e31166: f64 = (2.0 * locals.var_spsub_temp__blk863);
        let assign29080_e31168: f64 = (assign29080_e31166 - locals.var_gfsub2);
        (assign29080_e31168, ((2.0 * locals.var_spsub_temp__blk863_dn4) - locals.var_gfsub2_dn4), ((2.0 * locals.var_spsub_temp__blk863_dn6) - locals.var_gfsub2_dn6), ((2.0 * locals.var_spsub_temp__blk863_dn7) - locals.var_gfsub2_dn7), ((2.0 * locals.var_spsub_temp__blk863_dn8) - locals.var_gfsub2_dn8), ((2.0 * locals.var_spsub_temp__blk863_dn9) - locals.var_gfsub2_dn9),)
    } else {
        (locals.var_spsub_c__blk873, locals.var_spsub_c__blk873_dn4, locals.var_spsub_c__blk873_dn6, locals.var_spsub_c__blk873_dn7, locals.var_spsub_c__blk873_dn8, locals.var_spsub_c__blk873_dn9,)
    }
};
        locals.var_spsub_c__blk873 = assign29080_e31170;
        locals.var_spsub_c__blk873_dn4 = assign29080_e31170_d_n4;
        locals.var_spsub_c__blk873_dn6 = assign29080_e31170_d_n6;
        locals.var_spsub_c__blk873_dn7 = assign29080_e31170_d_n7;
        locals.var_spsub_c__blk873_dn8 = assign29080_e31170_d_n8;
        locals.var_spsub_c__blk873_dn9 = assign29080_e31170_d_n9;

        let (assign29090_e31187, assign29090_e31187_d_n4, assign29090_e31187_d_n6, assign29090_e31187_d_n7, assign29090_e31187_d_n8, assign29090_e31187_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29090_e31180: f64 = (-locals.var_spsub_eta__blk870);
        let assign29090_e31183: f64 = (locals.var_spsub_a__blk871 * locals.var_inv_gfsub2);
        let assign29090_e31184: f64 = (assign29090_e31183).ln();
        let assign29090_e31185: f64 = (assign29090_e31180 + assign29090_e31184);
        (assign29090_e31185, ((-locals.var_spsub_eta__blk870_dn4) + (((locals.var_spsub_a__blk871_dn4 * locals.var_inv_gfsub2) + (locals.var_spsub_a__blk871 * locals.var_inv_gfsub2_dn4)) / assign29090_e31183)), ((-locals.var_spsub_eta__blk870_dn6) + (((locals.var_spsub_a__blk871_dn6 * locals.var_inv_gfsub2) + (locals.var_spsub_a__blk871 * locals.var_inv_gfsub2_dn6)) / assign29090_e31183)), ((-locals.var_spsub_eta__blk870_dn7) + (((locals.var_spsub_a__blk871_dn7 * locals.var_inv_gfsub2) + (locals.var_spsub_a__blk871 * locals.var_inv_gfsub2_dn7)) / assign29090_e31183)), ((-locals.var_spsub_eta__blk870_dn8) + (((locals.var_spsub_a__blk871_dn8 * locals.var_inv_gfsub2) + (locals.var_spsub_a__blk871 * locals.var_inv_gfsub2_dn8)) / assign29090_e31183)), ((-locals.var_spsub_eta__blk870_dn9) + (((locals.var_spsub_a__blk871_dn9 * locals.var_inv_gfsub2) + (locals.var_spsub_a__blk871 * locals.var_inv_gfsub2_dn9)) / assign29090_e31183)),)
    } else {
        (locals.var_spsub_tau__blk874, locals.var_spsub_tau__blk874_dn4, locals.var_spsub_tau__blk874_dn6, locals.var_spsub_tau__blk874_dn7, locals.var_spsub_tau__blk874_dn8, locals.var_spsub_tau__blk874_dn9,)
    }
};
        locals.var_spsub_tau__blk874 = assign29090_e31187;
        locals.var_spsub_tau__blk874_dn4 = assign29090_e31187_d_n4;
        locals.var_spsub_tau__blk874_dn6 = assign29090_e31187_d_n6;
        locals.var_spsub_tau__blk874_dn7 = assign29090_e31187_d_n7;
        locals.var_spsub_tau__blk874_dn8 = assign29090_e31187_d_n8;
        locals.var_spsub_tau__blk874_dn9 = assign29090_e31187_d_n9;

        let (assign29100_e31200, assign29100_e31200_d_n4, assign29100_e31200_d_n6, assign29100_e31200_d_n7, assign29100_e31200_d_n8, assign29100_e31200_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29100_e31198: f64 = (locals.var_spsub_a__blk871 + locals.var_spsub_c__blk873);
        (assign29100_e31198, (locals.var_spsub_a__blk871_dn4 + locals.var_spsub_c__blk873_dn4), (locals.var_spsub_a__blk871_dn6 + locals.var_spsub_c__blk873_dn6), (locals.var_spsub_a__blk871_dn7 + locals.var_spsub_c__blk873_dn7), (locals.var_spsub_a__blk871_dn8 + locals.var_spsub_c__blk873_dn8), (locals.var_spsub_a__blk871_dn9 + locals.var_spsub_c__blk873_dn9),)
    } else {
        (locals.var_nu__blk861, locals.var_nu__blk861_dn4, locals.var_nu__blk861_dn6, locals.var_nu__blk861_dn7, locals.var_nu__blk861_dn8, locals.var_nu__blk861_dn9,)
    }
};
        locals.var_nu__blk861 = assign29100_e31200;
        locals.var_nu__blk861_dn4 = assign29100_e31200_d_n4;
        locals.var_nu__blk861_dn6 = assign29100_e31200_d_n6;
        locals.var_nu__blk861_dn7 = assign29100_e31200_d_n7;
        locals.var_nu__blk861_dn8 = assign29100_e31200_d_n8;
        locals.var_nu__blk861_dn9 = assign29100_e31200_d_n9;

        let (assign29110_e31223, assign29110_e31223_d_n4, assign29110_e31223_d_n6, assign29110_e31223_d_n7, assign29110_e31223_d_n8, assign29110_e31223_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29110_e31211: f64 = (locals.var_nu__blk861 * locals.var_nu__blk861);
        let assign29110_e31215: f64 = (0.5 * locals.var_spsub_c__blk873);
        let assign29110_e31217: f64 = (assign29110_e31215 * locals.var_spsub_c__blk873);
        let assign29110_e31219: f64 = (assign29110_e31217 - locals.var_spsub_a__blk871);
        let assign29110_e31220: f64 = (locals.var_spsub_tau__blk874 * assign29110_e31219);
        let assign29110_e31221: f64 = (assign29110_e31211 + assign29110_e31220);
        (assign29110_e31221, (((locals.var_nu__blk861_dn4 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn4)) + ((locals.var_spsub_tau__blk874_dn4 * assign29110_e31219) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn4) * locals.var_spsub_c__blk873) + (assign29110_e31215 * locals.var_spsub_c__blk873_dn4)) - locals.var_spsub_a__blk871_dn4)))), (((locals.var_nu__blk861_dn6 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn6)) + ((locals.var_spsub_tau__blk874_dn6 * assign29110_e31219) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn6) * locals.var_spsub_c__blk873) + (assign29110_e31215 * locals.var_spsub_c__blk873_dn6)) - locals.var_spsub_a__blk871_dn6)))), (((locals.var_nu__blk861_dn7 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn7)) + ((locals.var_spsub_tau__blk874_dn7 * assign29110_e31219) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn7) * locals.var_spsub_c__blk873) + (assign29110_e31215 * locals.var_spsub_c__blk873_dn7)) - locals.var_spsub_a__blk871_dn7)))), (((locals.var_nu__blk861_dn8 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn8)) + ((locals.var_spsub_tau__blk874_dn8 * assign29110_e31219) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn8) * locals.var_spsub_c__blk873) + (assign29110_e31215 * locals.var_spsub_c__blk873_dn8)) - locals.var_spsub_a__blk871_dn8)))), (((locals.var_nu__blk861_dn9 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn9)) + ((locals.var_spsub_tau__blk874_dn9 * assign29110_e31219) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn9) * locals.var_spsub_c__blk873) + (assign29110_e31215 * locals.var_spsub_c__blk873_dn9)) - locals.var_spsub_a__blk871_dn9)))),)
    } else {
        (locals.var_mutau__blk862, locals.var_mutau__blk862_dn4, locals.var_mutau__blk862_dn6, locals.var_mutau__blk862_dn7, locals.var_mutau__blk862_dn8, locals.var_mutau__blk862_dn9,)
    }
};
        locals.var_mutau__blk862 = assign29110_e31223;
        locals.var_mutau__blk862_dn4 = assign29110_e31223_d_n4;
        locals.var_mutau__blk862_dn6 = assign29110_e31223_d_n6;
        locals.var_mutau__blk862_dn7 = assign29110_e31223_d_n7;
        locals.var_mutau__blk862_dn8 = assign29110_e31223_d_n8;
        locals.var_mutau__blk862_dn9 = assign29110_e31223_d_n9;

        let (assign29120_e31260, assign29120_e31260_d_n4, assign29120_e31260_d_n6, assign29120_e31260_d_n7, assign29120_e31260_d_n8, assign29120_e31260_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29120_e31235: f64 = (locals.var_spsub_a__blk871 * locals.var_nu__blk861);
        let assign29120_e31237: f64 = (assign29120_e31235 * locals.var_spsub_tau__blk874);
        let assign29120_e31241: f64 = (locals.var_nu__blk861 / locals.var_mutau__blk862);
        let assign29120_e31243: f64 = (assign29120_e31241 * locals.var_spsub_tau__blk874);
        let assign29120_e31245: f64 = (assign29120_e31243 * locals.var_spsub_tau__blk874);
        let assign29120_e31247: f64 = (assign29120_e31245 * locals.var_spsub_c__blk873);
        let assign29120_e31250: f64 = (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873);
        let assign29120_e31252: f64 = (assign29120_e31250 * 0.3333333333333);
        let assign29120_e31254: f64 = (assign29120_e31252 - locals.var_spsub_a__blk871);
        let assign29120_e31255: f64 = (assign29120_e31247 * assign29120_e31254);
        let assign29120_e31256: f64 = (locals.var_mutau__blk862 + assign29120_e31255);
        let assign29120_e31257: f64 = (assign29120_e31237 / assign29120_e31256);
        let assign29120_e31258: f64 = (locals.var_spsub_eta__blk870 + assign29120_e31257);
        (assign29120_e31258, (locals.var_spsub_eta__blk870_dn4 + (((((((locals.var_spsub_a__blk871_dn4 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn4)) * locals.var_spsub_tau__blk874) + (assign29120_e31235 * locals.var_spsub_tau__blk874_dn4)) * assign29120_e31256) - (assign29120_e31237 * (locals.var_mutau__blk862_dn4 + (((((((((((locals.var_nu__blk861_dn4 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn4)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29120_e31241 * locals.var_spsub_tau__blk874_dn4)) * locals.var_spsub_tau__blk874) + (assign29120_e31243 * locals.var_spsub_tau__blk874_dn4)) * locals.var_spsub_c__blk873) + (assign29120_e31245 * locals.var_spsub_c__blk873_dn4)) * assign29120_e31254) + (assign29120_e31247 * ((((locals.var_spsub_c__blk873_dn4 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn4)) * 0.3333333333333) - locals.var_spsub_a__blk871_dn4)))))) / (assign29120_e31256 * assign29120_e31256))), (locals.var_spsub_eta__blk870_dn6 + (((((((locals.var_spsub_a__blk871_dn6 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn6)) * locals.var_spsub_tau__blk874) + (assign29120_e31235 * locals.var_spsub_tau__blk874_dn6)) * assign29120_e31256) - (assign29120_e31237 * (locals.var_mutau__blk862_dn6 + (((((((((((locals.var_nu__blk861_dn6 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn6)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29120_e31241 * locals.var_spsub_tau__blk874_dn6)) * locals.var_spsub_tau__blk874) + (assign29120_e31243 * locals.var_spsub_tau__blk874_dn6)) * locals.var_spsub_c__blk873) + (assign29120_e31245 * locals.var_spsub_c__blk873_dn6)) * assign29120_e31254) + (assign29120_e31247 * ((((locals.var_spsub_c__blk873_dn6 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn6)) * 0.3333333333333) - locals.var_spsub_a__blk871_dn6)))))) / (assign29120_e31256 * assign29120_e31256))), (locals.var_spsub_eta__blk870_dn7 + (((((((locals.var_spsub_a__blk871_dn7 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn7)) * locals.var_spsub_tau__blk874) + (assign29120_e31235 * locals.var_spsub_tau__blk874_dn7)) * assign29120_e31256) - (assign29120_e31237 * (locals.var_mutau__blk862_dn7 + (((((((((((locals.var_nu__blk861_dn7 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn7)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29120_e31241 * locals.var_spsub_tau__blk874_dn7)) * locals.var_spsub_tau__blk874) + (assign29120_e31243 * locals.var_spsub_tau__blk874_dn7)) * locals.var_spsub_c__blk873) + (assign29120_e31245 * locals.var_spsub_c__blk873_dn7)) * assign29120_e31254) + (assign29120_e31247 * ((((locals.var_spsub_c__blk873_dn7 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn7)) * 0.3333333333333) - locals.var_spsub_a__blk871_dn7)))))) / (assign29120_e31256 * assign29120_e31256))), (locals.var_spsub_eta__blk870_dn8 + (((((((locals.var_spsub_a__blk871_dn8 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn8)) * locals.var_spsub_tau__blk874) + (assign29120_e31235 * locals.var_spsub_tau__blk874_dn8)) * assign29120_e31256) - (assign29120_e31237 * (locals.var_mutau__blk862_dn8 + (((((((((((locals.var_nu__blk861_dn8 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn8)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29120_e31241 * locals.var_spsub_tau__blk874_dn8)) * locals.var_spsub_tau__blk874) + (assign29120_e31243 * locals.var_spsub_tau__blk874_dn8)) * locals.var_spsub_c__blk873) + (assign29120_e31245 * locals.var_spsub_c__blk873_dn8)) * assign29120_e31254) + (assign29120_e31247 * ((((locals.var_spsub_c__blk873_dn8 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn8)) * 0.3333333333333) - locals.var_spsub_a__blk871_dn8)))))) / (assign29120_e31256 * assign29120_e31256))), (locals.var_spsub_eta__blk870_dn9 + (((((((locals.var_spsub_a__blk871_dn9 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn9)) * locals.var_spsub_tau__blk874) + (assign29120_e31235 * locals.var_spsub_tau__blk874_dn9)) * assign29120_e31256) - (assign29120_e31237 * (locals.var_mutau__blk862_dn9 + (((((((((((locals.var_nu__blk861_dn9 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn9)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29120_e31241 * locals.var_spsub_tau__blk874_dn9)) * locals.var_spsub_tau__blk874) + (assign29120_e31243 * locals.var_spsub_tau__blk874_dn9)) * locals.var_spsub_c__blk873) + (assign29120_e31245 * locals.var_spsub_c__blk873_dn9)) * assign29120_e31254) + (assign29120_e31247 * ((((locals.var_spsub_c__blk873_dn9 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn9)) * 0.3333333333333) - locals.var_spsub_a__blk871_dn9)))))) / (assign29120_e31256 * assign29120_e31256))),)
    } else {
        (locals.var_spsub_y0__blk875, locals.var_spsub_y0__blk875_dn4, locals.var_spsub_y0__blk875_dn6, locals.var_spsub_y0__blk875_dn7, locals.var_spsub_y0__blk875_dn8, locals.var_spsub_y0__blk875_dn9,)
    }
};
        locals.var_spsub_y0__blk875 = assign29120_e31260;
        locals.var_spsub_y0__blk875_dn4 = assign29120_e31260_d_n4;
        locals.var_spsub_y0__blk875_dn6 = assign29120_e31260_d_n6;
        locals.var_spsub_y0__blk875_dn7 = assign29120_e31260_d_n7;
        locals.var_spsub_y0__blk875_dn8 = assign29120_e31260_d_n8;
        locals.var_spsub_y0__blk875_dn9 = assign29120_e31260_d_n9;

        let assign29130_e31263: f64 = if locals.var_spsub_y0__blk875 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1089 = assign29130_e31263;

        let (assign29140_e31277, assign29140_e31277_d_n4, assign29140_e31277_d_n6, assign29140_e31277_d_n7, assign29140_e31277_d_n8, assign29140_e31277_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) && (locals.var_guard1089 != 0.0)) {
        let assign29140_e31275: f64 = (locals.var_spsub_y0__blk875).exp();
        (assign29140_e31275, (assign29140_e31275 * locals.var_spsub_y0__blk875_dn4), (assign29140_e31275 * locals.var_spsub_y0__blk875_dn6), (assign29140_e31275 * locals.var_spsub_y0__blk875_dn7), (assign29140_e31275 * locals.var_spsub_y0__blk875_dn8), (assign29140_e31275 * locals.var_spsub_y0__blk875_dn9),)
    } else {
        (locals.var_spsub_delta0__blk876, locals.var_spsub_delta0__blk876_dn4, locals.var_spsub_delta0__blk876_dn6, locals.var_spsub_delta0__blk876_dn7, locals.var_spsub_delta0__blk876_dn8, locals.var_spsub_delta0__blk876_dn9,)
    }
};
        locals.var_spsub_delta0__blk876 = assign29140_e31277;
        locals.var_spsub_delta0__blk876_dn4 = assign29140_e31277_d_n4;
        locals.var_spsub_delta0__blk876_dn6 = assign29140_e31277_d_n6;
        locals.var_spsub_delta0__blk876_dn7 = assign29140_e31277_d_n7;
        locals.var_spsub_delta0__blk876_dn8 = assign29140_e31277_d_n8;
        locals.var_spsub_delta0__blk876_dn9 = assign29140_e31277_d_n9;

        let (assign29150_e31313, assign29150_e31313_d_n4, assign29150_e31313_d_n6, assign29150_e31313_d_n7, assign29150_e31313_d_n8, assign29150_e31313_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) && (locals.var_guard1089 == 0.0)) {
        let assign29150_e31293: f64 = (locals.var_spsub_y0__blk875 - 80.0);
        let assign29150_e31298: f64 = (locals.var_spsub_y0__blk875 - 80.0);
        let assign29150_e31299: f64 = (0.5 * assign29150_e31298);
        let assign29150_e31303: f64 = (locals.var_spsub_y0__blk875 - 80.0);
        let assign29150_e31305: f64 = (assign29150_e31303 * 0.3333333333333);
        let assign29150_e31306: f64 = (1.0 + assign29150_e31305);
        let assign29150_e31307: f64 = (assign29150_e31299 * assign29150_e31306);
        let assign29150_e31308: f64 = (1.0 + assign29150_e31307);
        let assign29150_e31309: f64 = (assign29150_e31293 * assign29150_e31308);
        let assign29150_e31310: f64 = (1.0 + assign29150_e31309);
        let assign29150_e31311: f64 = (5.54062e34 * assign29150_e31310);
        (assign29150_e31311, (5.54062e34 * ((locals.var_spsub_y0__blk875_dn4 * assign29150_e31308) + (assign29150_e31293 * (((0.5 * locals.var_spsub_y0__blk875_dn4) * assign29150_e31306) + (assign29150_e31299 * (locals.var_spsub_y0__blk875_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0__blk875_dn6 * assign29150_e31308) + (assign29150_e31293 * (((0.5 * locals.var_spsub_y0__blk875_dn6) * assign29150_e31306) + (assign29150_e31299 * (locals.var_spsub_y0__blk875_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0__blk875_dn7 * assign29150_e31308) + (assign29150_e31293 * (((0.5 * locals.var_spsub_y0__blk875_dn7) * assign29150_e31306) + (assign29150_e31299 * (locals.var_spsub_y0__blk875_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0__blk875_dn8 * assign29150_e31308) + (assign29150_e31293 * (((0.5 * locals.var_spsub_y0__blk875_dn8) * assign29150_e31306) + (assign29150_e31299 * (locals.var_spsub_y0__blk875_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0__blk875_dn9 * assign29150_e31308) + (assign29150_e31293 * (((0.5 * locals.var_spsub_y0__blk875_dn9) * assign29150_e31306) + (assign29150_e31299 * (locals.var_spsub_y0__blk875_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_spsub_delta0__blk876, locals.var_spsub_delta0__blk876_dn4, locals.var_spsub_delta0__blk876_dn6, locals.var_spsub_delta0__blk876_dn7, locals.var_spsub_delta0__blk876_dn8, locals.var_spsub_delta0__blk876_dn9,)
    }
};
        locals.var_spsub_delta0__blk876 = assign29150_e31313;
        locals.var_spsub_delta0__blk876_dn4 = assign29150_e31313_d_n4;
        locals.var_spsub_delta0__blk876_dn6 = assign29150_e31313_d_n6;
        locals.var_spsub_delta0__blk876_dn7 = assign29150_e31313_d_n7;
        locals.var_spsub_delta0__blk876_dn8 = assign29150_e31313_d_n8;
        locals.var_spsub_delta0__blk876_dn9 = assign29150_e31313_d_n9;

        let (assign29160_e31326, assign29160_e31326_d_n4, assign29160_e31326_d_n6, assign29160_e31326_d_n7, assign29160_e31326_d_n8, assign29160_e31326_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29160_e31324: f64 = (1.0 / locals.var_spsub_delta0__blk876);
        (assign29160_e31324, (-(locals.var_spsub_delta0__blk876_dn4 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))), (-(locals.var_spsub_delta0__blk876_dn6 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))), (-(locals.var_spsub_delta0__blk876_dn7 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))), (-(locals.var_spsub_delta0__blk876_dn8 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))), (-(locals.var_spsub_delta0__blk876_dn9 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))),)
    } else {
        (locals.var_spsub_delta1__blk877, locals.var_spsub_delta1__blk877_dn4, locals.var_spsub_delta1__blk877_dn6, locals.var_spsub_delta1__blk877_dn7, locals.var_spsub_delta1__blk877_dn8, locals.var_spsub_delta1__blk877_dn9,)
    }
};
        locals.var_spsub_delta1__blk877 = assign29160_e31326;
        locals.var_spsub_delta1__blk877_dn4 = assign29160_e31326_d_n4;
        locals.var_spsub_delta1__blk877_dn6 = assign29160_e31326_d_n6;
        locals.var_spsub_delta1__blk877_dn7 = assign29160_e31326_d_n7;
        locals.var_spsub_delta1__blk877_dn8 = assign29160_e31326_d_n8;
        locals.var_spsub_delta1__blk877_dn9 = assign29160_e31326_d_n9;

        let (assign29170_e31343, assign29170_e31343_d_n4, assign29170_e31343_d_n6, assign29170_e31343_d_n7, assign29170_e31343_d_n8, assign29170_e31343_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29170_e31339: f64 = (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875);
        let assign29170_e31340: f64 = (2.0 + assign29170_e31339);
        let assign29170_e31341: f64 = (1.0 / assign29170_e31340);
        (assign29170_e31341, (-(((locals.var_spsub_y0__blk875_dn4 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn4)) / (assign29170_e31340 * assign29170_e31340))), (-(((locals.var_spsub_y0__blk875_dn6 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn6)) / (assign29170_e31340 * assign29170_e31340))), (-(((locals.var_spsub_y0__blk875_dn7 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn7)) / (assign29170_e31340 * assign29170_e31340))), (-(((locals.var_spsub_y0__blk875_dn8 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn8)) / (assign29170_e31340 * assign29170_e31340))), (-(((locals.var_spsub_y0__blk875_dn9 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn9)) / (assign29170_e31340 * assign29170_e31340))),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29170_e31343;
        locals.var_spsub_temp__blk863_dn4 = assign29170_e31343_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29170_e31343_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29170_e31343_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29170_e31343_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29170_e31343_d_n9;

        let (assign29180_e31358, assign29180_e31358_d_n4, assign29180_e31358_d_n6, assign29180_e31358_d_n7, assign29180_e31358_d_n8, assign29180_e31358_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29180_e31354: f64 = (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875);
        let assign29180_e31356: f64 = (assign29180_e31354 * locals.var_spsub_temp__blk863);
        (assign29180_e31356, ((((locals.var_spsub_y0__blk875_dn4 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn4)) * locals.var_spsub_temp__blk863) + (assign29180_e31354 * locals.var_spsub_temp__blk863_dn4)), ((((locals.var_spsub_y0__blk875_dn6 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn6)) * locals.var_spsub_temp__blk863) + (assign29180_e31354 * locals.var_spsub_temp__blk863_dn6)), ((((locals.var_spsub_y0__blk875_dn7 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn7)) * locals.var_spsub_temp__blk863) + (assign29180_e31354 * locals.var_spsub_temp__blk863_dn7)), ((((locals.var_spsub_y0__blk875_dn8 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn8)) * locals.var_spsub_temp__blk863) + (assign29180_e31354 * locals.var_spsub_temp__blk863_dn8)), ((((locals.var_spsub_y0__blk875_dn9 * locals.var_spsub_y0__blk875) + (locals.var_spsub_y0__blk875 * locals.var_spsub_y0__blk875_dn9)) * locals.var_spsub_temp__blk863) + (assign29180_e31354 * locals.var_spsub_temp__blk863_dn9)),)
    } else {
        (locals.var_spsub_xi0__blk878, locals.var_spsub_xi0__blk878_dn4, locals.var_spsub_xi0__blk878_dn6, locals.var_spsub_xi0__blk878_dn7, locals.var_spsub_xi0__blk878_dn8, locals.var_spsub_xi0__blk878_dn9,)
    }
};
        locals.var_spsub_xi0__blk878 = assign29180_e31358;
        locals.var_spsub_xi0__blk878_dn4 = assign29180_e31358_d_n4;
        locals.var_spsub_xi0__blk878_dn6 = assign29180_e31358_d_n6;
        locals.var_spsub_xi0__blk878_dn7 = assign29180_e31358_d_n7;
        locals.var_spsub_xi0__blk878_dn8 = assign29180_e31358_d_n8;
        locals.var_spsub_xi0__blk878_dn9 = assign29180_e31358_d_n9;

        let (assign29190_e31375, assign29190_e31375_d_n4, assign29190_e31375_d_n6, assign29190_e31375_d_n7, assign29190_e31375_d_n8, assign29190_e31375_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29190_e31370: f64 = (locals.var_spsub_y0__blk875 * locals.var_spsub_temp__blk863);
        let assign29190_e31372: f64 = (assign29190_e31370 * locals.var_spsub_temp__blk863);
        let assign29190_e31373: f64 = (4.0 * assign29190_e31372);
        (assign29190_e31373, (4.0 * ((((locals.var_spsub_y0__blk875_dn4 * locals.var_spsub_temp__blk863) + (locals.var_spsub_y0__blk875 * locals.var_spsub_temp__blk863_dn4)) * locals.var_spsub_temp__blk863) + (assign29190_e31370 * locals.var_spsub_temp__blk863_dn4))), (4.0 * ((((locals.var_spsub_y0__blk875_dn6 * locals.var_spsub_temp__blk863) + (locals.var_spsub_y0__blk875 * locals.var_spsub_temp__blk863_dn6)) * locals.var_spsub_temp__blk863) + (assign29190_e31370 * locals.var_spsub_temp__blk863_dn6))), (4.0 * ((((locals.var_spsub_y0__blk875_dn7 * locals.var_spsub_temp__blk863) + (locals.var_spsub_y0__blk875 * locals.var_spsub_temp__blk863_dn7)) * locals.var_spsub_temp__blk863) + (assign29190_e31370 * locals.var_spsub_temp__blk863_dn7))), (4.0 * ((((locals.var_spsub_y0__blk875_dn8 * locals.var_spsub_temp__blk863) + (locals.var_spsub_y0__blk875 * locals.var_spsub_temp__blk863_dn8)) * locals.var_spsub_temp__blk863) + (assign29190_e31370 * locals.var_spsub_temp__blk863_dn8))), (4.0 * ((((locals.var_spsub_y0__blk875_dn9 * locals.var_spsub_temp__blk863) + (locals.var_spsub_y0__blk875 * locals.var_spsub_temp__blk863_dn9)) * locals.var_spsub_temp__blk863) + (assign29190_e31370 * locals.var_spsub_temp__blk863_dn9))),)
    } else {
        (locals.var_spsub_xi1__blk879, locals.var_spsub_xi1__blk879_dn4, locals.var_spsub_xi1__blk879_dn6, locals.var_spsub_xi1__blk879_dn7, locals.var_spsub_xi1__blk879_dn8, locals.var_spsub_xi1__blk879_dn9,)
    }
};
        locals.var_spsub_xi1__blk879 = assign29190_e31375;
        locals.var_spsub_xi1__blk879_dn4 = assign29190_e31375_d_n4;
        locals.var_spsub_xi1__blk879_dn6 = assign29190_e31375_d_n6;
        locals.var_spsub_xi1__blk879_dn7 = assign29190_e31375_d_n7;
        locals.var_spsub_xi1__blk879_dn8 = assign29190_e31375_d_n8;
        locals.var_spsub_xi1__blk879_dn9 = assign29190_e31375_d_n9;

        let (assign29200_e31396, assign29200_e31396_d_n4, assign29200_e31396_d_n6, assign29200_e31396_d_n7, assign29200_e31396_d_n8, assign29200_e31396_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29200_e31386: f64 = (8.0 * locals.var_spsub_temp__blk863);
        let assign29200_e31389: f64 = (12.0 * locals.var_spsub_xi0__blk878);
        let assign29200_e31390: f64 = (assign29200_e31386 - assign29200_e31389);
        let assign29200_e31392: f64 = (assign29200_e31390 * locals.var_spsub_temp__blk863);
        let assign29200_e31394: f64 = (assign29200_e31392 * locals.var_spsub_temp__blk863);
        (assign29200_e31394, ((((((8.0 * locals.var_spsub_temp__blk863_dn4) - (12.0 * locals.var_spsub_xi0__blk878_dn4)) * locals.var_spsub_temp__blk863) + (assign29200_e31390 * locals.var_spsub_temp__blk863_dn4)) * locals.var_spsub_temp__blk863) + (assign29200_e31392 * locals.var_spsub_temp__blk863_dn4)), ((((((8.0 * locals.var_spsub_temp__blk863_dn6) - (12.0 * locals.var_spsub_xi0__blk878_dn6)) * locals.var_spsub_temp__blk863) + (assign29200_e31390 * locals.var_spsub_temp__blk863_dn6)) * locals.var_spsub_temp__blk863) + (assign29200_e31392 * locals.var_spsub_temp__blk863_dn6)), ((((((8.0 * locals.var_spsub_temp__blk863_dn7) - (12.0 * locals.var_spsub_xi0__blk878_dn7)) * locals.var_spsub_temp__blk863) + (assign29200_e31390 * locals.var_spsub_temp__blk863_dn7)) * locals.var_spsub_temp__blk863) + (assign29200_e31392 * locals.var_spsub_temp__blk863_dn7)), ((((((8.0 * locals.var_spsub_temp__blk863_dn8) - (12.0 * locals.var_spsub_xi0__blk878_dn8)) * locals.var_spsub_temp__blk863) + (assign29200_e31390 * locals.var_spsub_temp__blk863_dn8)) * locals.var_spsub_temp__blk863) + (assign29200_e31392 * locals.var_spsub_temp__blk863_dn8)), ((((((8.0 * locals.var_spsub_temp__blk863_dn9) - (12.0 * locals.var_spsub_xi0__blk878_dn9)) * locals.var_spsub_temp__blk863) + (assign29200_e31390 * locals.var_spsub_temp__blk863_dn9)) * locals.var_spsub_temp__blk863) + (assign29200_e31392 * locals.var_spsub_temp__blk863_dn9)),)
    } else {
        (locals.var_spsub_xi2__blk880, locals.var_spsub_xi2__blk880_dn4, locals.var_spsub_xi2__blk880_dn6, locals.var_spsub_xi2__blk880_dn7, locals.var_spsub_xi2__blk880_dn8, locals.var_spsub_xi2__blk880_dn9,)
    }
};
        locals.var_spsub_xi2__blk880 = assign29200_e31396;
        locals.var_spsub_xi2__blk880_dn4 = assign29200_e31396_d_n4;
        locals.var_spsub_xi2__blk880_dn6 = assign29200_e31396_d_n6;
        locals.var_spsub_xi2__blk880_dn7 = assign29200_e31396_d_n7;
        locals.var_spsub_xi2__blk880_dn8 = assign29200_e31396_d_n8;
        locals.var_spsub_xi2__blk880_dn9 = assign29200_e31396_d_n9;

        let (assign29210_e31409, assign29210_e31409_d_n4, assign29210_e31409_d_n6, assign29210_e31409_d_n7, assign29210_e31409_d_n8, assign29210_e31409_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29210_e31407: f64 = (locals.var_spsub_yg__blk868 - locals.var_spsub_y0__blk875);
        (assign29210_e31407, (locals.var_spsub_yg__blk868_dn4 - locals.var_spsub_y0__blk875_dn4), (locals.var_spsub_yg__blk868_dn6 - locals.var_spsub_y0__blk875_dn6), (locals.var_spsub_yg__blk868_dn7 - locals.var_spsub_y0__blk875_dn7), (locals.var_spsub_yg__blk868_dn8 - locals.var_spsub_y0__blk875_dn8), (locals.var_spsub_yg__blk868_dn9 - locals.var_spsub_y0__blk875_dn9),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29210_e31409;
        locals.var_spsub_temp__blk863_dn4 = assign29210_e31409_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29210_e31409_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29210_e31409_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29210_e31409_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29210_e31409_d_n9;

        let (assign29220_e31422, assign29220_e31422_d_n4, assign29220_e31422_d_n6, assign29220_e31422_d_n7, assign29220_e31422_d_n8, assign29220_e31422_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29220_e31420: f64 = (locals.var_spsub_delta__blk867 * locals.var_spsub_delta1__blk877);
        (assign29220_e31420, ((locals.var_spsub_delta__blk867_dn4 * locals.var_spsub_delta1__blk877) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta1__blk877_dn4)), ((locals.var_spsub_delta__blk867_dn6 * locals.var_spsub_delta1__blk877) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta1__blk877_dn6)), ((locals.var_spsub_delta__blk867_dn7 * locals.var_spsub_delta1__blk877) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta1__blk877_dn7)), ((locals.var_spsub_delta__blk867_dn8 * locals.var_spsub_delta1__blk877) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta1__blk877_dn8)), ((locals.var_spsub_delta__blk867_dn9 * locals.var_spsub_delta1__blk877) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta1__blk877_dn9)),)
    } else {
        (locals.var_spsub_temp1__blk864, locals.var_spsub_temp1__blk864_dn4, locals.var_spsub_temp1__blk864_dn6, locals.var_spsub_temp1__blk864_dn7, locals.var_spsub_temp1__blk864_dn8, locals.var_spsub_temp1__blk864_dn9,)
    }
};
        locals.var_spsub_temp1__blk864 = assign29220_e31422;
        locals.var_spsub_temp1__blk864_dn4 = assign29220_e31422_d_n4;
        locals.var_spsub_temp1__blk864_dn6 = assign29220_e31422_d_n6;
        locals.var_spsub_temp1__blk864_dn7 = assign29220_e31422_d_n7;
        locals.var_spsub_temp1__blk864_dn8 = assign29220_e31422_d_n8;
        locals.var_spsub_temp1__blk864_dn9 = assign29220_e31422_d_n9;

        let (assign29230_e31449, assign29230_e31449_d_n4, assign29230_e31449_d_n6, assign29230_e31449_d_n7, assign29230_e31449_d_n8, assign29230_e31449_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29230_e31433: f64 = (2.0 * locals.var_spsub_temp__blk863);
        let assign29230_e31437: f64 = (locals.var_spsub_delta0__blk876 - 1.0);
        let assign29230_e31439: f64 = (assign29230_e31437 - locals.var_spsub_temp1__blk864);
        let assign29230_e31443: f64 = (1.0 - locals.var_spsub_xi1__blk879);
        let assign29230_e31444: f64 = (locals.var_spsub_delta__blk867 * assign29230_e31443);
        let assign29230_e31445: f64 = (assign29230_e31439 + assign29230_e31444);
        let assign29230_e31446: f64 = (locals.var_gfsub2 * assign29230_e31445);
        let assign29230_e31447: f64 = (assign29230_e31433 + assign29230_e31446);
        (assign29230_e31447, ((2.0 * locals.var_spsub_temp__blk863_dn4) + ((locals.var_gfsub2_dn4 * assign29230_e31445) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn4 - locals.var_spsub_temp1__blk864_dn4) + ((locals.var_spsub_delta__blk867_dn4 * assign29230_e31443) + (locals.var_spsub_delta__blk867 * (-locals.var_spsub_xi1__blk879_dn4))))))), ((2.0 * locals.var_spsub_temp__blk863_dn6) + ((locals.var_gfsub2_dn6 * assign29230_e31445) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn6 - locals.var_spsub_temp1__blk864_dn6) + ((locals.var_spsub_delta__blk867_dn6 * assign29230_e31443) + (locals.var_spsub_delta__blk867 * (-locals.var_spsub_xi1__blk879_dn6))))))), ((2.0 * locals.var_spsub_temp__blk863_dn7) + ((locals.var_gfsub2_dn7 * assign29230_e31445) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn7 - locals.var_spsub_temp1__blk864_dn7) + ((locals.var_spsub_delta__blk867_dn7 * assign29230_e31443) + (locals.var_spsub_delta__blk867 * (-locals.var_spsub_xi1__blk879_dn7))))))), ((2.0 * locals.var_spsub_temp__blk863_dn8) + ((locals.var_gfsub2_dn8 * assign29230_e31445) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn8 - locals.var_spsub_temp1__blk864_dn8) + ((locals.var_spsub_delta__blk867_dn8 * assign29230_e31443) + (locals.var_spsub_delta__blk867 * (-locals.var_spsub_xi1__blk879_dn8))))))), ((2.0 * locals.var_spsub_temp__blk863_dn9) + ((locals.var_gfsub2_dn9 * assign29230_e31445) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn9 - locals.var_spsub_temp1__blk864_dn9) + ((locals.var_spsub_delta__blk867_dn9 * assign29230_e31443) + (locals.var_spsub_delta__blk867 * (-locals.var_spsub_xi1__blk879_dn9))))))),)
    } else {
        (locals.var_spsub_pc__blk881, locals.var_spsub_pc__blk881_dn4, locals.var_spsub_pc__blk881_dn6, locals.var_spsub_pc__blk881_dn7, locals.var_spsub_pc__blk881_dn8, locals.var_spsub_pc__blk881_dn9,)
    }
};
        locals.var_spsub_pc__blk881 = assign29230_e31449;
        locals.var_spsub_pc__blk881_dn4 = assign29230_e31449_d_n4;
        locals.var_spsub_pc__blk881_dn6 = assign29230_e31449_d_n6;
        locals.var_spsub_pc__blk881_dn7 = assign29230_e31449_d_n7;
        locals.var_spsub_pc__blk881_dn8 = assign29230_e31449_d_n8;
        locals.var_spsub_pc__blk881_dn9 = assign29230_e31449_d_n9;

        let (assign29240_e31480, assign29240_e31480_d_n4, assign29240_e31480_d_n6, assign29240_e31480_d_n7, assign29240_e31480_d_n8, assign29240_e31480_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29240_e31460: f64 = (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863);
        let assign29240_e31464: f64 = (locals.var_spsub_delta0__blk876 - locals.var_spsub_y0__blk875);
        let assign29240_e31466: f64 = (assign29240_e31464 - 1.0);
        let assign29240_e31468: f64 = (assign29240_e31466 + locals.var_spsub_temp1__blk864);
        let assign29240_e31472: f64 = (locals.var_spsub_y0__blk875 - 1.0);
        let assign29240_e31474: f64 = (assign29240_e31472 - locals.var_spsub_xi0__blk878);
        let assign29240_e31475: f64 = (locals.var_spsub_delta__blk867 * assign29240_e31474);
        let assign29240_e31476: f64 = (assign29240_e31468 + assign29240_e31475);
        let assign29240_e31477: f64 = (locals.var_gfsub2 * assign29240_e31476);
        let assign29240_e31478: f64 = (assign29240_e31460 - assign29240_e31477);
        (assign29240_e31478, (((locals.var_spsub_temp__blk863_dn4 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn4)) - ((locals.var_gfsub2_dn4 * assign29240_e31476) + (locals.var_gfsub2 * (((locals.var_spsub_delta0__blk876_dn4 - locals.var_spsub_y0__blk875_dn4) + locals.var_spsub_temp1__blk864_dn4) + ((locals.var_spsub_delta__blk867_dn4 * assign29240_e31474) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_y0__blk875_dn4 - locals.var_spsub_xi0__blk878_dn4))))))), (((locals.var_spsub_temp__blk863_dn6 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn6)) - ((locals.var_gfsub2_dn6 * assign29240_e31476) + (locals.var_gfsub2 * (((locals.var_spsub_delta0__blk876_dn6 - locals.var_spsub_y0__blk875_dn6) + locals.var_spsub_temp1__blk864_dn6) + ((locals.var_spsub_delta__blk867_dn6 * assign29240_e31474) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_y0__blk875_dn6 - locals.var_spsub_xi0__blk878_dn6))))))), (((locals.var_spsub_temp__blk863_dn7 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn7)) - ((locals.var_gfsub2_dn7 * assign29240_e31476) + (locals.var_gfsub2 * (((locals.var_spsub_delta0__blk876_dn7 - locals.var_spsub_y0__blk875_dn7) + locals.var_spsub_temp1__blk864_dn7) + ((locals.var_spsub_delta__blk867_dn7 * assign29240_e31474) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_y0__blk875_dn7 - locals.var_spsub_xi0__blk878_dn7))))))), (((locals.var_spsub_temp__blk863_dn8 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn8)) - ((locals.var_gfsub2_dn8 * assign29240_e31476) + (locals.var_gfsub2 * (((locals.var_spsub_delta0__blk876_dn8 - locals.var_spsub_y0__blk875_dn8) + locals.var_spsub_temp1__blk864_dn8) + ((locals.var_spsub_delta__blk867_dn8 * assign29240_e31474) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_y0__blk875_dn8 - locals.var_spsub_xi0__blk878_dn8))))))), (((locals.var_spsub_temp__blk863_dn9 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn9)) - ((locals.var_gfsub2_dn9 * assign29240_e31476) + (locals.var_gfsub2 * (((locals.var_spsub_delta0__blk876_dn9 - locals.var_spsub_y0__blk875_dn9) + locals.var_spsub_temp1__blk864_dn9) + ((locals.var_spsub_delta__blk867_dn9 * assign29240_e31474) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_y0__blk875_dn9 - locals.var_spsub_xi0__blk878_dn9))))))),)
    } else {
        (locals.var_spsub_qc__blk882, locals.var_spsub_qc__blk882_dn4, locals.var_spsub_qc__blk882_dn6, locals.var_spsub_qc__blk882_dn7, locals.var_spsub_qc__blk882_dn8, locals.var_spsub_qc__blk882_dn9,)
    }
};
        locals.var_spsub_qc__blk882 = assign29240_e31480;
        locals.var_spsub_qc__blk882_dn4 = assign29240_e31480_d_n4;
        locals.var_spsub_qc__blk882_dn6 = assign29240_e31480_d_n6;
        locals.var_spsub_qc__blk882_dn7 = assign29240_e31480_d_n7;
        locals.var_spsub_qc__blk882_dn8 = assign29240_e31480_d_n8;
        locals.var_spsub_qc__blk882_dn9 = assign29240_e31480_d_n9;

        let (assign29250_e31501, assign29250_e31501_d_n4, assign29250_e31501_d_n6, assign29250_e31501_d_n7, assign29250_e31501_d_n8, assign29250_e31501_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29250_e31493: f64 = (locals.var_spsub_delta0__blk876 + locals.var_spsub_temp1__blk864);
        let assign29250_e31496: f64 = (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880);
        let assign29250_e31497: f64 = (assign29250_e31493 - assign29250_e31496);
        let assign29250_e31498: f64 = (locals.var_gfsub2 * assign29250_e31497);
        let assign29250_e31499: f64 = (2.0 - assign29250_e31498);
        (assign29250_e31499, (-((locals.var_gfsub2_dn4 * assign29250_e31497) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn4 + locals.var_spsub_temp1__blk864_dn4) - ((locals.var_spsub_delta__blk867_dn4 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn4)))))), (-((locals.var_gfsub2_dn6 * assign29250_e31497) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn6 + locals.var_spsub_temp1__blk864_dn6) - ((locals.var_spsub_delta__blk867_dn6 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn6)))))), (-((locals.var_gfsub2_dn7 * assign29250_e31497) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn7 + locals.var_spsub_temp1__blk864_dn7) - ((locals.var_spsub_delta__blk867_dn7 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn7)))))), (-((locals.var_gfsub2_dn8 * assign29250_e31497) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn8 + locals.var_spsub_temp1__blk864_dn8) - ((locals.var_spsub_delta__blk867_dn8 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn8)))))), (-((locals.var_gfsub2_dn9 * assign29250_e31497) + (locals.var_gfsub2 * ((locals.var_spsub_delta0__blk876_dn9 + locals.var_spsub_temp1__blk864_dn9) - ((locals.var_spsub_delta__blk867_dn9 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn9)))))),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29250_e31501;
        locals.var_spsub_temp__blk863_dn4 = assign29250_e31501_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29250_e31501_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29250_e31501_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29250_e31501_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29250_e31501_d_n9;

        let (assign29260_e31520, assign29260_e31520_d_n4, assign29260_e31520_d_n6, assign29260_e31520_d_n7, assign29260_e31520_d_n8, assign29260_e31520_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29260_e31512: f64 = (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881);
        let assign29260_e31516: f64 = (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863);
        let assign29260_e31517: f64 = (2.0 * assign29260_e31516);
        let assign29260_e31518: f64 = (assign29260_e31512 - assign29260_e31517);
        (assign29260_e31518, (((locals.var_spsub_pc__blk881_dn4 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn4)) - (2.0 * ((locals.var_spsub_qc__blk882_dn4 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn4)))), (((locals.var_spsub_pc__blk881_dn6 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn6)) - (2.0 * ((locals.var_spsub_qc__blk882_dn6 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn6)))), (((locals.var_spsub_pc__blk881_dn7 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn7)) - (2.0 * ((locals.var_spsub_qc__blk882_dn7 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn7)))), (((locals.var_spsub_pc__blk881_dn8 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn8)) - (2.0 * ((locals.var_spsub_qc__blk882_dn8 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn8)))), (((locals.var_spsub_pc__blk881_dn9 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn9)) - (2.0 * ((locals.var_spsub_qc__blk882_dn9 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn9)))),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29260_e31520;
        locals.var_spsub_temp__blk863_dn4 = assign29260_e31520_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29260_e31520_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29260_e31520_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29260_e31520_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29260_e31520_d_n9;

        let (assign29270_e31541, assign29270_e31541_d_n4, assign29270_e31541_d_n6, assign29270_e31541_d_n7, assign29270_e31541_d_n8, assign29270_e31541_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 != 0.0)) {
        let assign29270_e31530: f64 = (-locals.var_spsub_y0__blk875);
        let assign29270_e31535: f64 = (locals.var_spsub_temp__blk863).sqrt();
        let assign29270_e31536: f64 = (locals.var_spsub_pc__blk881 + assign29270_e31535);
        let assign29270_e31537: f64 = (locals.var_spsub_qc__blk882 / assign29270_e31536);
        let assign29270_e31538: f64 = (2.0 * assign29270_e31537);
        let assign29270_e31539: f64 = (assign29270_e31530 - assign29270_e31538);
        (assign29270_e31539, ((-locals.var_spsub_y0__blk875_dn4) - (2.0 * (((locals.var_spsub_qc__blk882_dn4 * assign29270_e31536) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn4 + (locals.var_spsub_temp__blk863_dn4 / (2.0 * assign29270_e31535))))) / (assign29270_e31536 * assign29270_e31536)))), ((-locals.var_spsub_y0__blk875_dn6) - (2.0 * (((locals.var_spsub_qc__blk882_dn6 * assign29270_e31536) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn6 + (locals.var_spsub_temp__blk863_dn6 / (2.0 * assign29270_e31535))))) / (assign29270_e31536 * assign29270_e31536)))), ((-locals.var_spsub_y0__blk875_dn7) - (2.0 * (((locals.var_spsub_qc__blk882_dn7 * assign29270_e31536) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn7 + (locals.var_spsub_temp__blk863_dn7 / (2.0 * assign29270_e31535))))) / (assign29270_e31536 * assign29270_e31536)))), ((-locals.var_spsub_y0__blk875_dn8) - (2.0 * (((locals.var_spsub_qc__blk882_dn8 * assign29270_e31536) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn8 + (locals.var_spsub_temp__blk863_dn8 / (2.0 * assign29270_e31535))))) / (assign29270_e31536 * assign29270_e31536)))), ((-locals.var_spsub_y0__blk875_dn9) - (2.0 * (((locals.var_spsub_qc__blk882_dn9 * assign29270_e31536) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn9 + (locals.var_spsub_temp__blk863_dn9 / (2.0 * assign29270_e31535))))) / (assign29270_e31536 * assign29270_e31536)))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign29270_e31541;
        locals.var_temp3_dn4 = assign29270_e31541_d_n4;
        locals.var_temp3_dn6 = assign29270_e31541_d_n6;
        locals.var_temp3_dn7 = assign29270_e31541_d_n7;
        locals.var_temp3_dn8 = assign29270_e31541_d_n8;
        locals.var_temp3_dn9 = assign29270_e31541_d_n9;

        let (assign29280_e31559, assign29280_e31559_d_n4, assign29280_e31559_d_n6, assign29280_e31559_d_n7, assign29280_e31559_d_n8, assign29280_e31559_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29280_e31555: f64 = (locals.var_gfsub * 0.732464877560822);
        let assign29280_e31556: f64 = (1.25 + assign29280_e31555);
        let assign29280_e31557: f64 = (1.0 / assign29280_e31556);
        (assign29280_e31557, (-((locals.var_gfsub_dn4 * 0.732464877560822) / (assign29280_e31556 * assign29280_e31556))), (-((locals.var_gfsub_dn6 * 0.732464877560822) / (assign29280_e31556 * assign29280_e31556))), (-((locals.var_gfsub_dn7 * 0.732464877560822) / (assign29280_e31556 * assign29280_e31556))), (-((locals.var_gfsub_dn8 * 0.732464877560822) / (assign29280_e31556 * assign29280_e31556))), (-((locals.var_gfsub_dn9 * 0.732464877560822) / (assign29280_e31556 * assign29280_e31556))),)
    } else {
        (locals.var_spsub_xg1__blk883, locals.var_spsub_xg1__blk883_dn4, locals.var_spsub_xg1__blk883_dn6, locals.var_spsub_xg1__blk883_dn7, locals.var_spsub_xg1__blk883_dn8, locals.var_spsub_xg1__blk883_dn9,)
    }
};
        locals.var_spsub_xg1__blk883 = assign29280_e31559;
        locals.var_spsub_xg1__blk883_dn4 = assign29280_e31559_d_n4;
        locals.var_spsub_xg1__blk883_dn6 = assign29280_e31559_d_n6;
        locals.var_spsub_xg1__blk883_dn7 = assign29280_e31559_d_n7;
        locals.var_spsub_xg1__blk883_dn8 = assign29280_e31559_d_n8;
        locals.var_spsub_xg1__blk883_dn9 = assign29280_e31559_d_n9;

        let (assign29290_e31579, assign29290_e31579_d_n4, assign29290_e31579_d_n6, assign29290_e31579_d_n7, assign29290_e31579_d_n8, assign29290_e31579_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29290_e31571: f64 = (1.25 * locals.var_xisub);
        let assign29290_e31573: f64 = (assign29290_e31571 * locals.var_spsub_xg1__blk883);
        let assign29290_e31575: f64 = (assign29290_e31573 - 1.0);
        let assign29290_e31577: f64 = (assign29290_e31575 * locals.var_spsub_xg1__blk883);
        (assign29290_e31577, (((((1.25 * locals.var_xisub_dn4) * locals.var_spsub_xg1__blk883) + (assign29290_e31571 * locals.var_spsub_xg1__blk883_dn4)) * locals.var_spsub_xg1__blk883) + (assign29290_e31575 * locals.var_spsub_xg1__blk883_dn4)), (((((1.25 * locals.var_xisub_dn6) * locals.var_spsub_xg1__blk883) + (assign29290_e31571 * locals.var_spsub_xg1__blk883_dn6)) * locals.var_spsub_xg1__blk883) + (assign29290_e31575 * locals.var_spsub_xg1__blk883_dn6)), (((((1.25 * locals.var_xisub_dn7) * locals.var_spsub_xg1__blk883) + (assign29290_e31571 * locals.var_spsub_xg1__blk883_dn7)) * locals.var_spsub_xg1__blk883) + (assign29290_e31575 * locals.var_spsub_xg1__blk883_dn7)), (((((1.25 * locals.var_xisub_dn8) * locals.var_spsub_xg1__blk883) + (assign29290_e31571 * locals.var_spsub_xg1__blk883_dn8)) * locals.var_spsub_xg1__blk883) + (assign29290_e31575 * locals.var_spsub_xg1__blk883_dn8)), (((((1.25 * locals.var_xisub_dn9) * locals.var_spsub_xg1__blk883) + (assign29290_e31571 * locals.var_spsub_xg1__blk883_dn9)) * locals.var_spsub_xg1__blk883) + (assign29290_e31575 * locals.var_spsub_xg1__blk883_dn9)),)
    } else {
        (locals.var_spsub_a_fac__blk884, locals.var_spsub_a_fac__blk884_dn4, locals.var_spsub_a_fac__blk884_dn6, locals.var_spsub_a_fac__blk884_dn7, locals.var_spsub_a_fac__blk884_dn8, locals.var_spsub_a_fac__blk884_dn9,)
    }
};
        locals.var_spsub_a_fac__blk884 = assign29290_e31579;
        locals.var_spsub_a_fac__blk884_dn4 = assign29290_e31579_d_n4;
        locals.var_spsub_a_fac__blk884_dn6 = assign29290_e31579_d_n6;
        locals.var_spsub_a_fac__blk884_dn7 = assign29290_e31579_d_n7;
        locals.var_spsub_a_fac__blk884_dn8 = assign29290_e31579_d_n8;
        locals.var_spsub_a_fac__blk884_dn9 = assign29290_e31579_d_n9;

        let (assign29300_e31599, assign29300_e31599_d_n4, assign29300_e31599_d_n6, assign29300_e31599_d_n7, assign29300_e31599_d_n8, assign29300_e31599_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29300_e31591: f64 = (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub);
        let assign29300_e31595: f64 = (locals.var_spsub_a_fac__blk884 * locals.var_spsub_xgb__blk866);
        let assign29300_e31596: f64 = (1.0 + assign29300_e31595);
        let assign29300_e31597: f64 = (assign29300_e31591 * assign29300_e31596);
        (assign29300_e31597, ((((locals.var_spsub_xgb__blk866_dn4 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn4)) * assign29300_e31596) + (assign29300_e31591 * ((locals.var_spsub_a_fac__blk884_dn4 * locals.var_spsub_xgb__blk866) + (locals.var_spsub_a_fac__blk884 * locals.var_spsub_xgb__blk866_dn4)))), ((((locals.var_spsub_xgb__blk866_dn6 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn6)) * assign29300_e31596) + (assign29300_e31591 * ((locals.var_spsub_a_fac__blk884_dn6 * locals.var_spsub_xgb__blk866) + (locals.var_spsub_a_fac__blk884 * locals.var_spsub_xgb__blk866_dn6)))), ((((locals.var_spsub_xgb__blk866_dn7 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn7)) * assign29300_e31596) + (assign29300_e31591 * ((locals.var_spsub_a_fac__blk884_dn7 * locals.var_spsub_xgb__blk866) + (locals.var_spsub_a_fac__blk884 * locals.var_spsub_xgb__blk866_dn7)))), ((((locals.var_spsub_xgb__blk866_dn8 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn8)) * assign29300_e31596) + (assign29300_e31591 * ((locals.var_spsub_a_fac__blk884_dn8 * locals.var_spsub_xgb__blk866) + (locals.var_spsub_a_fac__blk884 * locals.var_spsub_xgb__blk866_dn8)))), ((((locals.var_spsub_xgb__blk866_dn9 * locals.var_inv_xisub) + (locals.var_spsub_xgb__blk866 * locals.var_inv_xisub_dn9)) * assign29300_e31596) + (assign29300_e31591 * ((locals.var_spsub_a_fac__blk884_dn9 * locals.var_spsub_xgb__blk866) + (locals.var_spsub_a_fac__blk884 * locals.var_spsub_xgb__blk866_dn9)))),)
    } else {
        (locals.var_spsub_xbar__blk885, locals.var_spsub_xbar__blk885_dn4, locals.var_spsub_xbar__blk885_dn6, locals.var_spsub_xbar__blk885_dn7, locals.var_spsub_xbar__blk885_dn8, locals.var_spsub_xbar__blk885_dn9,)
    }
};
        locals.var_spsub_xbar__blk885 = assign29300_e31599;
        locals.var_spsub_xbar__blk885_dn4 = assign29300_e31599_d_n4;
        locals.var_spsub_xbar__blk885_dn6 = assign29300_e31599_d_n6;
        locals.var_spsub_xbar__blk885_dn7 = assign29300_e31599_d_n7;
        locals.var_spsub_xbar__blk885_dn8 = assign29300_e31599_d_n8;
        locals.var_spsub_xbar__blk885_dn9 = assign29300_e31599_d_n9;

        let assign29310_e31601: f64 = (-locals.var_spsub_xbar__blk885);
        let assign29310_e31603: f64 = (-80.0);
        let assign29310_e31604: f64 = if assign29310_e31601 > assign29310_e31603 { 1.0 } else { 0.0 };
        locals.var_guard1090 = assign29310_e31604;

        let (assign29320_e31620, assign29320_e31620_d_n4, assign29320_e31620_d_n6, assign29320_e31620_d_n7, assign29320_e31620_d_n8, assign29320_e31620_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1090 != 0.0)) {
        let assign29320_e31617: f64 = (-locals.var_spsub_xbar__blk885);
        let assign29320_e31618: f64 = (assign29320_e31617).exp();
        (assign29320_e31618, (assign29320_e31618 * (-locals.var_spsub_xbar__blk885_dn4)), (assign29320_e31618 * (-locals.var_spsub_xbar__blk885_dn6)), (assign29320_e31618 * (-locals.var_spsub_xbar__blk885_dn7)), (assign29320_e31618 * (-locals.var_spsub_xbar__blk885_dn8)), (assign29320_e31618 * (-locals.var_spsub_xbar__blk885_dn9)),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29320_e31620;
        locals.var_spsub_temp__blk863_dn4 = assign29320_e31620_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29320_e31620_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29320_e31620_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29320_e31620_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29320_e31620_d_n9;

    }

    pub(super) fn stamp_transient_block_77(
        locals: &mut StampLocals,
    ) {
        let (assign29330_e31663, assign29330_e31663_d_n4, assign29330_e31663_d_n6, assign29330_e31663_d_n7, assign29330_e31663_d_n8, assign29330_e31663_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1090 == 0.0)) {
        let assign29330_e31636: f64 = (-locals.var_spsub_xbar__blk885);
        let assign29330_e31637: f64 = (-assign29330_e31636);
        let assign29330_e31639: f64 = (assign29330_e31637 - 80.0);
        let assign29330_e31643: f64 = (-locals.var_spsub_xbar__blk885);
        let assign29330_e31644: f64 = (-assign29330_e31643);
        let assign29330_e31646: f64 = (assign29330_e31644 - 80.0);
        let assign29330_e31647: f64 = (0.5 * assign29330_e31646);
        let assign29330_e31650: f64 = (-locals.var_spsub_xbar__blk885);
        let assign29330_e31651: f64 = (-assign29330_e31650);
        let assign29330_e31653: f64 = (assign29330_e31651 - 80.0);
        let assign29330_e31655: f64 = (assign29330_e31653 * 0.3333333333333);
        let assign29330_e31656: f64 = (1.0 + assign29330_e31655);
        let assign29330_e31657: f64 = (assign29330_e31647 * assign29330_e31656);
        let assign29330_e31658: f64 = (1.0 + assign29330_e31657);
        let assign29330_e31659: f64 = (assign29330_e31639 * assign29330_e31658);
        let assign29330_e31660: f64 = (1.0 + assign29330_e31659);
        let assign29330_e31661: f64 = (1.80485e-35 / assign29330_e31660);
        (assign29330_e31661, (-((1.80485e-35 * (((-(-locals.var_spsub_xbar__blk885_dn4)) * assign29330_e31658) + (assign29330_e31639 * (((0.5 * (-(-locals.var_spsub_xbar__blk885_dn4))) * assign29330_e31656) + (assign29330_e31647 * ((-(-locals.var_spsub_xbar__blk885_dn4)) * 0.3333333333333)))))) / (assign29330_e31660 * assign29330_e31660))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar__blk885_dn6)) * assign29330_e31658) + (assign29330_e31639 * (((0.5 * (-(-locals.var_spsub_xbar__blk885_dn6))) * assign29330_e31656) + (assign29330_e31647 * ((-(-locals.var_spsub_xbar__blk885_dn6)) * 0.3333333333333)))))) / (assign29330_e31660 * assign29330_e31660))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar__blk885_dn7)) * assign29330_e31658) + (assign29330_e31639 * (((0.5 * (-(-locals.var_spsub_xbar__blk885_dn7))) * assign29330_e31656) + (assign29330_e31647 * ((-(-locals.var_spsub_xbar__blk885_dn7)) * 0.3333333333333)))))) / (assign29330_e31660 * assign29330_e31660))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar__blk885_dn8)) * assign29330_e31658) + (assign29330_e31639 * (((0.5 * (-(-locals.var_spsub_xbar__blk885_dn8))) * assign29330_e31656) + (assign29330_e31647 * ((-(-locals.var_spsub_xbar__blk885_dn8)) * 0.3333333333333)))))) / (assign29330_e31660 * assign29330_e31660))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar__blk885_dn9)) * assign29330_e31658) + (assign29330_e31639 * (((0.5 * (-(-locals.var_spsub_xbar__blk885_dn9))) * assign29330_e31656) + (assign29330_e31647 * ((-(-locals.var_spsub_xbar__blk885_dn9)) * 0.3333333333333)))))) / (assign29330_e31660 * assign29330_e31660))),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29330_e31663;
        locals.var_spsub_temp__blk863_dn4 = assign29330_e31663_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29330_e31663_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29330_e31663_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29330_e31663_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29330_e31663_d_n9;

        let (assign29340_e31677, assign29340_e31677_d_n4, assign29340_e31677_d_n6, assign29340_e31677_d_n7, assign29340_e31677_d_n8, assign29340_e31677_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29340_e31675: f64 = (1.0 - locals.var_spsub_temp__blk863);
        (assign29340_e31675, (-locals.var_spsub_temp__blk863_dn4), (-locals.var_spsub_temp__blk863_dn6), (-locals.var_spsub_temp__blk863_dn7), (-locals.var_spsub_temp__blk863_dn8), (-locals.var_spsub_temp__blk863_dn9),)
    } else {
        (locals.var_spsub_w__blk886, locals.var_spsub_w__blk886_dn4, locals.var_spsub_w__blk886_dn6, locals.var_spsub_w__blk886_dn7, locals.var_spsub_w__blk886_dn8, locals.var_spsub_w__blk886_dn9,)
    }
};
        locals.var_spsub_w__blk886 = assign29340_e31677;
        locals.var_spsub_w__blk886_dn4 = assign29340_e31677_d_n4;
        locals.var_spsub_w__blk886_dn6 = assign29340_e31677_d_n6;
        locals.var_spsub_w__blk886_dn7 = assign29340_e31677_d_n7;
        locals.var_spsub_w__blk886_dn8 = assign29340_e31677_d_n8;
        locals.var_spsub_w__blk886_dn9 = assign29340_e31677_d_n9;

        let (assign29350_e31704, assign29350_e31704_d_n4, assign29350_e31704_d_n6, assign29350_e31704_d_n7, assign29350_e31704_d_n8, assign29350_e31704_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29350_e31690: f64 = (locals.var_gfsub2 * 0.5);
        let assign29350_e31691: f64 = (locals.var_spsub_xgb__blk866 + assign29350_e31690);
        let assign29350_e31696: f64 = (locals.var_gfsub2 * 0.25);
        let assign29350_e31697: f64 = (locals.var_spsub_xgb__blk866 + assign29350_e31696);
        let assign29350_e31699: f64 = (assign29350_e31697 - locals.var_spsub_w__blk886);
        let assign29350_e31700: f64 = (assign29350_e31699).sqrt();
        let assign29350_e31701: f64 = (locals.var_gfsub * assign29350_e31700);
        let assign29350_e31702: f64 = (assign29350_e31691 - assign29350_e31701);
        (assign29350_e31702, ((locals.var_spsub_xgb__blk866_dn4 + (locals.var_gfsub2_dn4 * 0.5)) - ((locals.var_gfsub_dn4 * assign29350_e31700) + (locals.var_gfsub * (((locals.var_spsub_xgb__blk866_dn4 + (locals.var_gfsub2_dn4 * 0.25)) - locals.var_spsub_w__blk886_dn4) / (2.0 * assign29350_e31700))))), ((locals.var_spsub_xgb__blk866_dn6 + (locals.var_gfsub2_dn6 * 0.5)) - ((locals.var_gfsub_dn6 * assign29350_e31700) + (locals.var_gfsub * (((locals.var_spsub_xgb__blk866_dn6 + (locals.var_gfsub2_dn6 * 0.25)) - locals.var_spsub_w__blk886_dn6) / (2.0 * assign29350_e31700))))), ((locals.var_spsub_xgb__blk866_dn7 + (locals.var_gfsub2_dn7 * 0.5)) - ((locals.var_gfsub_dn7 * assign29350_e31700) + (locals.var_gfsub * (((locals.var_spsub_xgb__blk866_dn7 + (locals.var_gfsub2_dn7 * 0.25)) - locals.var_spsub_w__blk886_dn7) / (2.0 * assign29350_e31700))))), ((locals.var_spsub_xgb__blk866_dn8 + (locals.var_gfsub2_dn8 * 0.5)) - ((locals.var_gfsub_dn8 * assign29350_e31700) + (locals.var_gfsub * (((locals.var_spsub_xgb__blk866_dn8 + (locals.var_gfsub2_dn8 * 0.25)) - locals.var_spsub_w__blk886_dn8) / (2.0 * assign29350_e31700))))), ((locals.var_spsub_xgb__blk866_dn9 + (locals.var_gfsub2_dn9 * 0.5)) - ((locals.var_gfsub_dn9 * assign29350_e31700) + (locals.var_gfsub * (((locals.var_spsub_xgb__blk866_dn9 + (locals.var_gfsub2_dn9 * 0.25)) - locals.var_spsub_w__blk886_dn9) / (2.0 * assign29350_e31700))))),)
    } else {
        (locals.var_spsub_x1__blk887, locals.var_spsub_x1__blk887_dn4, locals.var_spsub_x1__blk887_dn6, locals.var_spsub_x1__blk887_dn7, locals.var_spsub_x1__blk887_dn8, locals.var_spsub_x1__blk887_dn9,)
    }
};
        locals.var_spsub_x1__blk887 = assign29350_e31704;
        locals.var_spsub_x1__blk887_dn4 = assign29350_e31704_d_n4;
        locals.var_spsub_x1__blk887_dn6 = assign29350_e31704_d_n6;
        locals.var_spsub_x1__blk887_dn7 = assign29350_e31704_d_n7;
        locals.var_spsub_x1__blk887_dn8 = assign29350_e31704_d_n8;
        locals.var_spsub_x1__blk887_dn9 = assign29350_e31704_d_n9;

        let (assign29360_e31718, assign29360_e31718_d_n4, assign29360_e31718_d_n6, assign29360_e31718_d_n7, assign29360_e31718_d_n8, assign29360_e31718_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29360_e31716: f64 = (locals.var_xn_sub + 3.0);
        (assign29360_e31716, locals.var_xn_sub_dn4, locals.var_xn_sub_dn6, locals.var_xn_sub_dn7, locals.var_xn_sub_dn8, locals.var_xn_sub_dn9,)
    } else {
        (locals.var_spsub_bx__blk888, locals.var_spsub_bx__blk888_dn4, locals.var_spsub_bx__blk888_dn6, locals.var_spsub_bx__blk888_dn7, locals.var_spsub_bx__blk888_dn8, locals.var_spsub_bx__blk888_dn9,)
    }
};
        locals.var_spsub_bx__blk888 = assign29360_e31718;
        locals.var_spsub_bx__blk888_dn4 = assign29360_e31718_d_n4;
        locals.var_spsub_bx__blk888_dn6 = assign29360_e31718_d_n6;
        locals.var_spsub_bx__blk888_dn7 = assign29360_e31718_d_n7;
        locals.var_spsub_bx__blk888_dn8 = assign29360_e31718_d_n8;
        locals.var_spsub_bx__blk888_dn9 = assign29360_e31718_d_n9;

        let (assign29370_e31756, assign29370_e31756_d_n4, assign29370_e31756_d_n6, assign29370_e31756_d_n7, assign29370_e31756_d_n8, assign29370_e31756_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29370_e31731: f64 = (locals.var_spsub_x1__blk887 + locals.var_spsub_bx__blk888);
        let assign29370_e31734: f64 = (locals.var_spsub_x1__blk887 - locals.var_spsub_bx__blk888);
        let assign29370_e31737: f64 = (locals.var_spsub_x1__blk887 - locals.var_spsub_bx__blk888);
        let assign29370_e31738: f64 = (assign29370_e31734 * assign29370_e31737);
        let assign29370_e31740: f64 = (assign29370_e31738 + 5.0);
        let assign29370_e31741: f64 = (assign29370_e31740).sqrt();
        let assign29370_e31742: f64 = (assign29370_e31731 - assign29370_e31741);
        let assign29370_e31743: f64 = (0.5 * assign29370_e31742);
        let assign29370_e31748: f64 = (locals.var_spsub_bx__blk888 * locals.var_spsub_bx__blk888);
        let assign29370_e31750: f64 = (assign29370_e31748 + 5.0);
        let assign29370_e31751: f64 = (assign29370_e31750).sqrt();
        let assign29370_e31752: f64 = (locals.var_spsub_bx__blk888 - assign29370_e31751);
        let assign29370_e31753: f64 = (0.5 * assign29370_e31752);
        let assign29370_e31754: f64 = (assign29370_e31743 - assign29370_e31753);
        (assign29370_e31754, ((0.5 * ((locals.var_spsub_x1__blk887_dn4 + locals.var_spsub_bx__blk888_dn4) - ((((locals.var_spsub_x1__blk887_dn4 - locals.var_spsub_bx__blk888_dn4) * assign29370_e31737) + (assign29370_e31734 * (locals.var_spsub_x1__blk887_dn4 - locals.var_spsub_bx__blk888_dn4))) / (2.0 * assign29370_e31741)))) - (0.5 * (locals.var_spsub_bx__blk888_dn4 - (((locals.var_spsub_bx__blk888_dn4 * locals.var_spsub_bx__blk888) + (locals.var_spsub_bx__blk888 * locals.var_spsub_bx__blk888_dn4)) / (2.0 * assign29370_e31751))))), ((0.5 * ((locals.var_spsub_x1__blk887_dn6 + locals.var_spsub_bx__blk888_dn6) - ((((locals.var_spsub_x1__blk887_dn6 - locals.var_spsub_bx__blk888_dn6) * assign29370_e31737) + (assign29370_e31734 * (locals.var_spsub_x1__blk887_dn6 - locals.var_spsub_bx__blk888_dn6))) / (2.0 * assign29370_e31741)))) - (0.5 * (locals.var_spsub_bx__blk888_dn6 - (((locals.var_spsub_bx__blk888_dn6 * locals.var_spsub_bx__blk888) + (locals.var_spsub_bx__blk888 * locals.var_spsub_bx__blk888_dn6)) / (2.0 * assign29370_e31751))))), ((0.5 * ((locals.var_spsub_x1__blk887_dn7 + locals.var_spsub_bx__blk888_dn7) - ((((locals.var_spsub_x1__blk887_dn7 - locals.var_spsub_bx__blk888_dn7) * assign29370_e31737) + (assign29370_e31734 * (locals.var_spsub_x1__blk887_dn7 - locals.var_spsub_bx__blk888_dn7))) / (2.0 * assign29370_e31741)))) - (0.5 * (locals.var_spsub_bx__blk888_dn7 - (((locals.var_spsub_bx__blk888_dn7 * locals.var_spsub_bx__blk888) + (locals.var_spsub_bx__blk888 * locals.var_spsub_bx__blk888_dn7)) / (2.0 * assign29370_e31751))))), ((0.5 * ((locals.var_spsub_x1__blk887_dn8 + locals.var_spsub_bx__blk888_dn8) - ((((locals.var_spsub_x1__blk887_dn8 - locals.var_spsub_bx__blk888_dn8) * assign29370_e31737) + (assign29370_e31734 * (locals.var_spsub_x1__blk887_dn8 - locals.var_spsub_bx__blk888_dn8))) / (2.0 * assign29370_e31741)))) - (0.5 * (locals.var_spsub_bx__blk888_dn8 - (((locals.var_spsub_bx__blk888_dn8 * locals.var_spsub_bx__blk888) + (locals.var_spsub_bx__blk888 * locals.var_spsub_bx__blk888_dn8)) / (2.0 * assign29370_e31751))))), ((0.5 * ((locals.var_spsub_x1__blk887_dn9 + locals.var_spsub_bx__blk888_dn9) - ((((locals.var_spsub_x1__blk887_dn9 - locals.var_spsub_bx__blk888_dn9) * assign29370_e31737) + (assign29370_e31734 * (locals.var_spsub_x1__blk887_dn9 - locals.var_spsub_bx__blk888_dn9))) / (2.0 * assign29370_e31741)))) - (0.5 * (locals.var_spsub_bx__blk888_dn9 - (((locals.var_spsub_bx__blk888_dn9 * locals.var_spsub_bx__blk888) + (locals.var_spsub_bx__blk888 * locals.var_spsub_bx__blk888_dn9)) / (2.0 * assign29370_e31751))))),)
    } else {
        (locals.var_spsub_eta__blk870, locals.var_spsub_eta__blk870_dn4, locals.var_spsub_eta__blk870_dn6, locals.var_spsub_eta__blk870_dn7, locals.var_spsub_eta__blk870_dn8, locals.var_spsub_eta__blk870_dn9,)
    }
};
        locals.var_spsub_eta__blk870 = assign29370_e31756;
        locals.var_spsub_eta__blk870_dn4 = assign29370_e31756_d_n4;
        locals.var_spsub_eta__blk870_dn6 = assign29370_e31756_d_n6;
        locals.var_spsub_eta__blk870_dn7 = assign29370_e31756_d_n7;
        locals.var_spsub_eta__blk870_dn8 = assign29370_e31756_d_n8;
        locals.var_spsub_eta__blk870_dn9 = assign29370_e31756_d_n9;

        let (assign29380_e31770, assign29380_e31770_d_n4, assign29380_e31770_d_n6, assign29380_e31770_d_n7, assign29380_e31770_d_n8, assign29380_e31770_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29380_e31768: f64 = (locals.var_spsub_xgb__blk866 - locals.var_spsub_eta__blk870);
        (assign29380_e31768, (locals.var_spsub_xgb__blk866_dn4 - locals.var_spsub_eta__blk870_dn4), (locals.var_spsub_xgb__blk866_dn6 - locals.var_spsub_eta__blk870_dn6), (locals.var_spsub_xgb__blk866_dn7 - locals.var_spsub_eta__blk870_dn7), (locals.var_spsub_xgb__blk866_dn8 - locals.var_spsub_eta__blk870_dn8), (locals.var_spsub_xgb__blk866_dn9 - locals.var_spsub_eta__blk870_dn9),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29380_e31770;
        locals.var_spsub_temp__blk863_dn4 = assign29380_e31770_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29380_e31770_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29380_e31770_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29380_e31770_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29380_e31770_d_n9;

        let (assign29390_e31784, assign29390_e31784_d_n4, assign29390_e31784_d_n6, assign29390_e31784_d_n7, assign29390_e31784_d_n8, assign29390_e31784_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29390_e31781: f64 = (-locals.var_spsub_eta__blk870);
        let assign29390_e31782: f64 = (assign29390_e31781).exp();
        (assign29390_e31782, (assign29390_e31782 * (-locals.var_spsub_eta__blk870_dn4)), (assign29390_e31782 * (-locals.var_spsub_eta__blk870_dn6)), (assign29390_e31782 * (-locals.var_spsub_eta__blk870_dn7)), (assign29390_e31782 * (-locals.var_spsub_eta__blk870_dn8)), (assign29390_e31782 * (-locals.var_spsub_eta__blk870_dn9)),)
    } else {
        (locals.var_spsub_temp1__blk864, locals.var_spsub_temp1__blk864_dn4, locals.var_spsub_temp1__blk864_dn6, locals.var_spsub_temp1__blk864_dn7, locals.var_spsub_temp1__blk864_dn8, locals.var_spsub_temp1__blk864_dn9,)
    }
};
        locals.var_spsub_temp1__blk864 = assign29390_e31784;
        locals.var_spsub_temp1__blk864_dn4 = assign29390_e31784_d_n4;
        locals.var_spsub_temp1__blk864_dn6 = assign29390_e31784_d_n6;
        locals.var_spsub_temp1__blk864_dn7 = assign29390_e31784_d_n7;
        locals.var_spsub_temp1__blk864_dn8 = assign29390_e31784_d_n8;
        locals.var_spsub_temp1__blk864_dn9 = assign29390_e31784_d_n9;

        let (assign29400_e31802, assign29400_e31802_d_n4, assign29400_e31802_d_n6, assign29400_e31802_d_n7, assign29400_e31802_d_n8, assign29400_e31802_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29400_e31798: f64 = (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870);
        let assign29400_e31799: f64 = (2.0 + assign29400_e31798);
        let assign29400_e31800: f64 = (1.0 / assign29400_e31799);
        (assign29400_e31800, (-(((locals.var_spsub_eta__blk870_dn4 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn4)) / (assign29400_e31799 * assign29400_e31799))), (-(((locals.var_spsub_eta__blk870_dn6 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn6)) / (assign29400_e31799 * assign29400_e31799))), (-(((locals.var_spsub_eta__blk870_dn7 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn7)) / (assign29400_e31799 * assign29400_e31799))), (-(((locals.var_spsub_eta__blk870_dn8 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn8)) / (assign29400_e31799 * assign29400_e31799))), (-(((locals.var_spsub_eta__blk870_dn9 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn9)) / (assign29400_e31799 * assign29400_e31799))),)
    } else {
        (locals.var_spsub_temp2__blk865, locals.var_spsub_temp2__blk865_dn4, locals.var_spsub_temp2__blk865_dn6, locals.var_spsub_temp2__blk865_dn7, locals.var_spsub_temp2__blk865_dn8, locals.var_spsub_temp2__blk865_dn9,)
    }
};
        locals.var_spsub_temp2__blk865 = assign29400_e31802;
        locals.var_spsub_temp2__blk865_dn4 = assign29400_e31802_d_n4;
        locals.var_spsub_temp2__blk865_dn6 = assign29400_e31802_d_n6;
        locals.var_spsub_temp2__blk865_dn7 = assign29400_e31802_d_n7;
        locals.var_spsub_temp2__blk865_dn8 = assign29400_e31802_d_n8;
        locals.var_spsub_temp2__blk865_dn9 = assign29400_e31802_d_n9;

        let (assign29410_e31818, assign29410_e31818_d_n4, assign29410_e31818_d_n6, assign29410_e31818_d_n7, assign29410_e31818_d_n8, assign29410_e31818_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29410_e31814: f64 = (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870);
        let assign29410_e31816: f64 = (assign29410_e31814 * locals.var_spsub_temp2__blk865);
        (assign29410_e31816, ((((locals.var_spsub_eta__blk870_dn4 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn4)) * locals.var_spsub_temp2__blk865) + (assign29410_e31814 * locals.var_spsub_temp2__blk865_dn4)), ((((locals.var_spsub_eta__blk870_dn6 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn6)) * locals.var_spsub_temp2__blk865) + (assign29410_e31814 * locals.var_spsub_temp2__blk865_dn6)), ((((locals.var_spsub_eta__blk870_dn7 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn7)) * locals.var_spsub_temp2__blk865) + (assign29410_e31814 * locals.var_spsub_temp2__blk865_dn7)), ((((locals.var_spsub_eta__blk870_dn8 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn8)) * locals.var_spsub_temp2__blk865) + (assign29410_e31814 * locals.var_spsub_temp2__blk865_dn8)), ((((locals.var_spsub_eta__blk870_dn9 * locals.var_spsub_eta__blk870) + (locals.var_spsub_eta__blk870 * locals.var_spsub_eta__blk870_dn9)) * locals.var_spsub_temp2__blk865) + (assign29410_e31814 * locals.var_spsub_temp2__blk865_dn9)),)
    } else {
        (locals.var_spsub_xi0__blk878, locals.var_spsub_xi0__blk878_dn4, locals.var_spsub_xi0__blk878_dn6, locals.var_spsub_xi0__blk878_dn7, locals.var_spsub_xi0__blk878_dn8, locals.var_spsub_xi0__blk878_dn9,)
    }
};
        locals.var_spsub_xi0__blk878 = assign29410_e31818;
        locals.var_spsub_xi0__blk878_dn4 = assign29410_e31818_d_n4;
        locals.var_spsub_xi0__blk878_dn6 = assign29410_e31818_d_n6;
        locals.var_spsub_xi0__blk878_dn7 = assign29410_e31818_d_n7;
        locals.var_spsub_xi0__blk878_dn8 = assign29410_e31818_d_n8;
        locals.var_spsub_xi0__blk878_dn9 = assign29410_e31818_d_n9;

        let (assign29420_e31836, assign29420_e31836_d_n4, assign29420_e31836_d_n6, assign29420_e31836_d_n7, assign29420_e31836_d_n8, assign29420_e31836_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29420_e31831: f64 = (locals.var_spsub_eta__blk870 * locals.var_spsub_temp2__blk865);
        let assign29420_e31833: f64 = (assign29420_e31831 * locals.var_spsub_temp2__blk865);
        let assign29420_e31834: f64 = (4.0 * assign29420_e31833);
        (assign29420_e31834, (4.0 * ((((locals.var_spsub_eta__blk870_dn4 * locals.var_spsub_temp2__blk865) + (locals.var_spsub_eta__blk870 * locals.var_spsub_temp2__blk865_dn4)) * locals.var_spsub_temp2__blk865) + (assign29420_e31831 * locals.var_spsub_temp2__blk865_dn4))), (4.0 * ((((locals.var_spsub_eta__blk870_dn6 * locals.var_spsub_temp2__blk865) + (locals.var_spsub_eta__blk870 * locals.var_spsub_temp2__blk865_dn6)) * locals.var_spsub_temp2__blk865) + (assign29420_e31831 * locals.var_spsub_temp2__blk865_dn6))), (4.0 * ((((locals.var_spsub_eta__blk870_dn7 * locals.var_spsub_temp2__blk865) + (locals.var_spsub_eta__blk870 * locals.var_spsub_temp2__blk865_dn7)) * locals.var_spsub_temp2__blk865) + (assign29420_e31831 * locals.var_spsub_temp2__blk865_dn7))), (4.0 * ((((locals.var_spsub_eta__blk870_dn8 * locals.var_spsub_temp2__blk865) + (locals.var_spsub_eta__blk870 * locals.var_spsub_temp2__blk865_dn8)) * locals.var_spsub_temp2__blk865) + (assign29420_e31831 * locals.var_spsub_temp2__blk865_dn8))), (4.0 * ((((locals.var_spsub_eta__blk870_dn9 * locals.var_spsub_temp2__blk865) + (locals.var_spsub_eta__blk870 * locals.var_spsub_temp2__blk865_dn9)) * locals.var_spsub_temp2__blk865) + (assign29420_e31831 * locals.var_spsub_temp2__blk865_dn9))),)
    } else {
        (locals.var_spsub_xi1__blk879, locals.var_spsub_xi1__blk879_dn4, locals.var_spsub_xi1__blk879_dn6, locals.var_spsub_xi1__blk879_dn7, locals.var_spsub_xi1__blk879_dn8, locals.var_spsub_xi1__blk879_dn9,)
    }
};
        locals.var_spsub_xi1__blk879 = assign29420_e31836;
        locals.var_spsub_xi1__blk879_dn4 = assign29420_e31836_d_n4;
        locals.var_spsub_xi1__blk879_dn6 = assign29420_e31836_d_n6;
        locals.var_spsub_xi1__blk879_dn7 = assign29420_e31836_d_n7;
        locals.var_spsub_xi1__blk879_dn8 = assign29420_e31836_d_n8;
        locals.var_spsub_xi1__blk879_dn9 = assign29420_e31836_d_n9;

        let (assign29430_e31858, assign29430_e31858_d_n4, assign29430_e31858_d_n6, assign29430_e31858_d_n7, assign29430_e31858_d_n8, assign29430_e31858_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29430_e31848: f64 = (8.0 * locals.var_spsub_temp2__blk865);
        let assign29430_e31851: f64 = (12.0 * locals.var_spsub_xi0__blk878);
        let assign29430_e31852: f64 = (assign29430_e31848 - assign29430_e31851);
        let assign29430_e31854: f64 = (assign29430_e31852 * locals.var_spsub_temp2__blk865);
        let assign29430_e31856: f64 = (assign29430_e31854 * locals.var_spsub_temp2__blk865);
        (assign29430_e31856, ((((((8.0 * locals.var_spsub_temp2__blk865_dn4) - (12.0 * locals.var_spsub_xi0__blk878_dn4)) * locals.var_spsub_temp2__blk865) + (assign29430_e31852 * locals.var_spsub_temp2__blk865_dn4)) * locals.var_spsub_temp2__blk865) + (assign29430_e31854 * locals.var_spsub_temp2__blk865_dn4)), ((((((8.0 * locals.var_spsub_temp2__blk865_dn6) - (12.0 * locals.var_spsub_xi0__blk878_dn6)) * locals.var_spsub_temp2__blk865) + (assign29430_e31852 * locals.var_spsub_temp2__blk865_dn6)) * locals.var_spsub_temp2__blk865) + (assign29430_e31854 * locals.var_spsub_temp2__blk865_dn6)), ((((((8.0 * locals.var_spsub_temp2__blk865_dn7) - (12.0 * locals.var_spsub_xi0__blk878_dn7)) * locals.var_spsub_temp2__blk865) + (assign29430_e31852 * locals.var_spsub_temp2__blk865_dn7)) * locals.var_spsub_temp2__blk865) + (assign29430_e31854 * locals.var_spsub_temp2__blk865_dn7)), ((((((8.0 * locals.var_spsub_temp2__blk865_dn8) - (12.0 * locals.var_spsub_xi0__blk878_dn8)) * locals.var_spsub_temp2__blk865) + (assign29430_e31852 * locals.var_spsub_temp2__blk865_dn8)) * locals.var_spsub_temp2__blk865) + (assign29430_e31854 * locals.var_spsub_temp2__blk865_dn8)), ((((((8.0 * locals.var_spsub_temp2__blk865_dn9) - (12.0 * locals.var_spsub_xi0__blk878_dn9)) * locals.var_spsub_temp2__blk865) + (assign29430_e31852 * locals.var_spsub_temp2__blk865_dn9)) * locals.var_spsub_temp2__blk865) + (assign29430_e31854 * locals.var_spsub_temp2__blk865_dn9)),)
    } else {
        (locals.var_spsub_xi2__blk880, locals.var_spsub_xi2__blk880_dn4, locals.var_spsub_xi2__blk880_dn6, locals.var_spsub_xi2__blk880_dn7, locals.var_spsub_xi2__blk880_dn8, locals.var_spsub_xi2__blk880_dn9,)
    }
};
        locals.var_spsub_xi2__blk880 = assign29430_e31858;
        locals.var_spsub_xi2__blk880_dn4 = assign29430_e31858_d_n4;
        locals.var_spsub_xi2__blk880_dn6 = assign29430_e31858_d_n6;
        locals.var_spsub_xi2__blk880_dn7 = assign29430_e31858_d_n7;
        locals.var_spsub_xi2__blk880_dn8 = assign29430_e31858_d_n8;
        locals.var_spsub_xi2__blk880_dn9 = assign29430_e31858_d_n9;

        let (assign29440_e31890, assign29440_e31890_d_n4, assign29440_e31890_d_n6, assign29440_e31890_d_n7, assign29440_e31890_d_n8, assign29440_e31890_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29440_e31871: f64 = (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863);
        let assign29440_e31875: f64 = (locals.var_spsub_temp1__blk864 + locals.var_spsub_eta__blk870);
        let assign29440_e31877: f64 = (assign29440_e31875 - 1.0);
        let assign29440_e31881: f64 = (locals.var_spsub_eta__blk870 + 1.0);
        let assign29440_e31883: f64 = (assign29440_e31881 + locals.var_spsub_xi0__blk878);
        let assign29440_e31884: f64 = (locals.var_spsub_delta__blk867 * assign29440_e31883);
        let assign29440_e31885: f64 = (assign29440_e31877 - assign29440_e31884);
        let assign29440_e31886: f64 = (locals.var_gfsub2 * assign29440_e31885);
        let assign29440_e31887: f64 = (assign29440_e31871 - assign29440_e31886);
        let assign29440_e31888: f64 = (1e-40_f64).max(assign29440_e31887);
        (assign29440_e31888, if 1e-40 >= assign29440_e31887 { 0.0 } else { (((locals.var_spsub_temp__blk863_dn4 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn4)) - ((locals.var_gfsub2_dn4 * assign29440_e31885) + (locals.var_gfsub2 * ((locals.var_spsub_temp1__blk864_dn4 + locals.var_spsub_eta__blk870_dn4) - ((locals.var_spsub_delta__blk867_dn4 * assign29440_e31883) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_eta__blk870_dn4 + locals.var_spsub_xi0__blk878_dn4))))))) }, if 1e-40 >= assign29440_e31887 { 0.0 } else { (((locals.var_spsub_temp__blk863_dn6 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn6)) - ((locals.var_gfsub2_dn6 * assign29440_e31885) + (locals.var_gfsub2 * ((locals.var_spsub_temp1__blk864_dn6 + locals.var_spsub_eta__blk870_dn6) - ((locals.var_spsub_delta__blk867_dn6 * assign29440_e31883) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_eta__blk870_dn6 + locals.var_spsub_xi0__blk878_dn6))))))) }, if 1e-40 >= assign29440_e31887 { 0.0 } else { (((locals.var_spsub_temp__blk863_dn7 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn7)) - ((locals.var_gfsub2_dn7 * assign29440_e31885) + (locals.var_gfsub2 * ((locals.var_spsub_temp1__blk864_dn7 + locals.var_spsub_eta__blk870_dn7) - ((locals.var_spsub_delta__blk867_dn7 * assign29440_e31883) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_eta__blk870_dn7 + locals.var_spsub_xi0__blk878_dn7))))))) }, if 1e-40 >= assign29440_e31887 { 0.0 } else { (((locals.var_spsub_temp__blk863_dn8 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn8)) - ((locals.var_gfsub2_dn8 * assign29440_e31885) + (locals.var_gfsub2 * ((locals.var_spsub_temp1__blk864_dn8 + locals.var_spsub_eta__blk870_dn8) - ((locals.var_spsub_delta__blk867_dn8 * assign29440_e31883) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_eta__blk870_dn8 + locals.var_spsub_xi0__blk878_dn8))))))) }, if 1e-40 >= assign29440_e31887 { 0.0 } else { (((locals.var_spsub_temp__blk863_dn9 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn9)) - ((locals.var_gfsub2_dn9 * assign29440_e31885) + (locals.var_gfsub2 * ((locals.var_spsub_temp1__blk864_dn9 + locals.var_spsub_eta__blk870_dn9) - ((locals.var_spsub_delta__blk867_dn9 * assign29440_e31883) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_eta__blk870_dn9 + locals.var_spsub_xi0__blk878_dn9))))))) },)
    } else {
        (locals.var_spsub_a__blk871, locals.var_spsub_a__blk871_dn4, locals.var_spsub_a__blk871_dn6, locals.var_spsub_a__blk871_dn7, locals.var_spsub_a__blk871_dn8, locals.var_spsub_a__blk871_dn9,)
    }
};
        locals.var_spsub_a__blk871 = assign29440_e31890;
        locals.var_spsub_a__blk871_dn4 = assign29440_e31890_d_n4;
        locals.var_spsub_a__blk871_dn6 = assign29440_e31890_d_n6;
        locals.var_spsub_a__blk871_dn7 = assign29440_e31890_d_n7;
        locals.var_spsub_a__blk871_dn8 = assign29440_e31890_d_n8;
        locals.var_spsub_a__blk871_dn9 = assign29440_e31890_d_n9;

        let (assign29450_e31912, assign29450_e31912_d_n4, assign29450_e31912_d_n6, assign29450_e31912_d_n7, assign29450_e31912_d_n8, assign29450_e31912_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29450_e31906: f64 = (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880);
        let assign29450_e31907: f64 = (locals.var_spsub_temp1__blk864 - assign29450_e31906);
        let assign29450_e31908: f64 = (locals.var_gfsub2 * assign29450_e31907);
        let assign29450_e31909: f64 = (0.5 * assign29450_e31908);
        let assign29450_e31910: f64 = (1.0 - assign29450_e31909);
        (assign29450_e31910, (-(0.5 * ((locals.var_gfsub2_dn4 * assign29450_e31907) + (locals.var_gfsub2 * (locals.var_spsub_temp1__blk864_dn4 - ((locals.var_spsub_delta__blk867_dn4 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn4))))))), (-(0.5 * ((locals.var_gfsub2_dn6 * assign29450_e31907) + (locals.var_gfsub2 * (locals.var_spsub_temp1__blk864_dn6 - ((locals.var_spsub_delta__blk867_dn6 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn6))))))), (-(0.5 * ((locals.var_gfsub2_dn7 * assign29450_e31907) + (locals.var_gfsub2 * (locals.var_spsub_temp1__blk864_dn7 - ((locals.var_spsub_delta__blk867_dn7 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn7))))))), (-(0.5 * ((locals.var_gfsub2_dn8 * assign29450_e31907) + (locals.var_gfsub2 * (locals.var_spsub_temp1__blk864_dn8 - ((locals.var_spsub_delta__blk867_dn8 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn8))))))), (-(0.5 * ((locals.var_gfsub2_dn9 * assign29450_e31907) + (locals.var_gfsub2 * (locals.var_spsub_temp1__blk864_dn9 - ((locals.var_spsub_delta__blk867_dn9 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn9))))))),)
    } else {
        (locals.var_spsub_b__blk872, locals.var_spsub_b__blk872_dn4, locals.var_spsub_b__blk872_dn6, locals.var_spsub_b__blk872_dn7, locals.var_spsub_b__blk872_dn8, locals.var_spsub_b__blk872_dn9,)
    }
};
        locals.var_spsub_b__blk872 = assign29450_e31912;
        locals.var_spsub_b__blk872_dn4 = assign29450_e31912_d_n4;
        locals.var_spsub_b__blk872_dn6 = assign29450_e31912_d_n6;
        locals.var_spsub_b__blk872_dn7 = assign29450_e31912_d_n7;
        locals.var_spsub_b__blk872_dn8 = assign29450_e31912_d_n8;
        locals.var_spsub_b__blk872_dn9 = assign29450_e31912_d_n9;

        let (assign29460_e31938, assign29460_e31938_d_n4, assign29460_e31938_d_n6, assign29460_e31938_d_n7, assign29460_e31938_d_n8, assign29460_e31938_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29460_e31924: f64 = (2.0 * locals.var_spsub_temp__blk863);
        let assign29460_e31928: f64 = (1.0 - locals.var_spsub_temp1__blk864);
        let assign29460_e31932: f64 = (1.0 + locals.var_spsub_xi1__blk879);
        let assign29460_e31933: f64 = (locals.var_spsub_delta__blk867 * assign29460_e31932);
        let assign29460_e31934: f64 = (assign29460_e31928 - assign29460_e31933);
        let assign29460_e31935: f64 = (locals.var_gfsub2 * assign29460_e31934);
        let assign29460_e31936: f64 = (assign29460_e31924 + assign29460_e31935);
        (assign29460_e31936, ((2.0 * locals.var_spsub_temp__blk863_dn4) + ((locals.var_gfsub2_dn4 * assign29460_e31934) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1__blk864_dn4) - ((locals.var_spsub_delta__blk867_dn4 * assign29460_e31932) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn4)))))), ((2.0 * locals.var_spsub_temp__blk863_dn6) + ((locals.var_gfsub2_dn6 * assign29460_e31934) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1__blk864_dn6) - ((locals.var_spsub_delta__blk867_dn6 * assign29460_e31932) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn6)))))), ((2.0 * locals.var_spsub_temp__blk863_dn7) + ((locals.var_gfsub2_dn7 * assign29460_e31934) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1__blk864_dn7) - ((locals.var_spsub_delta__blk867_dn7 * assign29460_e31932) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn7)))))), ((2.0 * locals.var_spsub_temp__blk863_dn8) + ((locals.var_gfsub2_dn8 * assign29460_e31934) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1__blk864_dn8) - ((locals.var_spsub_delta__blk867_dn8 * assign29460_e31932) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn8)))))), ((2.0 * locals.var_spsub_temp__blk863_dn9) + ((locals.var_gfsub2_dn9 * assign29460_e31934) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1__blk864_dn9) - ((locals.var_spsub_delta__blk867_dn9 * assign29460_e31932) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn9)))))),)
    } else {
        (locals.var_spsub_c__blk873, locals.var_spsub_c__blk873_dn4, locals.var_spsub_c__blk873_dn6, locals.var_spsub_c__blk873_dn7, locals.var_spsub_c__blk873_dn8, locals.var_spsub_c__blk873_dn9,)
    }
};
        locals.var_spsub_c__blk873 = assign29460_e31938;
        locals.var_spsub_c__blk873_dn4 = assign29460_e31938_d_n4;
        locals.var_spsub_c__blk873_dn6 = assign29460_e31938_d_n6;
        locals.var_spsub_c__blk873_dn7 = assign29460_e31938_d_n7;
        locals.var_spsub_c__blk873_dn8 = assign29460_e31938_d_n8;
        locals.var_spsub_c__blk873_dn9 = assign29460_e31938_d_n9;

        let (assign29470_e31957, assign29470_e31957_d_n4, assign29470_e31957_d_n6, assign29470_e31957_d_n7, assign29470_e31957_d_n8, assign29470_e31957_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29470_e31950: f64 = (locals.var_xn_sub - locals.var_spsub_eta__blk870);
        let assign29470_e31953: f64 = (locals.var_spsub_a__blk871 / locals.var_gfsub2);
        let assign29470_e31954: f64 = (assign29470_e31953).ln();
        let assign29470_e31955: f64 = (assign29470_e31950 + assign29470_e31954);
        (assign29470_e31955, ((locals.var_xn_sub_dn4 - locals.var_spsub_eta__blk870_dn4) + ((((locals.var_spsub_a__blk871_dn4 * locals.var_gfsub2) - (locals.var_spsub_a__blk871 * locals.var_gfsub2_dn4)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign29470_e31953)), ((locals.var_xn_sub_dn6 - locals.var_spsub_eta__blk870_dn6) + ((((locals.var_spsub_a__blk871_dn6 * locals.var_gfsub2) - (locals.var_spsub_a__blk871 * locals.var_gfsub2_dn6)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign29470_e31953)), ((locals.var_xn_sub_dn7 - locals.var_spsub_eta__blk870_dn7) + ((((locals.var_spsub_a__blk871_dn7 * locals.var_gfsub2) - (locals.var_spsub_a__blk871 * locals.var_gfsub2_dn7)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign29470_e31953)), ((locals.var_xn_sub_dn8 - locals.var_spsub_eta__blk870_dn8) + ((((locals.var_spsub_a__blk871_dn8 * locals.var_gfsub2) - (locals.var_spsub_a__blk871 * locals.var_gfsub2_dn8)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign29470_e31953)), ((locals.var_xn_sub_dn9 - locals.var_spsub_eta__blk870_dn9) + ((((locals.var_spsub_a__blk871_dn9 * locals.var_gfsub2) - (locals.var_spsub_a__blk871 * locals.var_gfsub2_dn9)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign29470_e31953)),)
    } else {
        (locals.var_spsub_tau__blk874, locals.var_spsub_tau__blk874_dn4, locals.var_spsub_tau__blk874_dn6, locals.var_spsub_tau__blk874_dn7, locals.var_spsub_tau__blk874_dn8, locals.var_spsub_tau__blk874_dn9,)
    }
};
        locals.var_spsub_tau__blk874 = assign29470_e31957;
        locals.var_spsub_tau__blk874_dn4 = assign29470_e31957_d_n4;
        locals.var_spsub_tau__blk874_dn6 = assign29470_e31957_d_n6;
        locals.var_spsub_tau__blk874_dn7 = assign29470_e31957_d_n7;
        locals.var_spsub_tau__blk874_dn8 = assign29470_e31957_d_n8;
        locals.var_spsub_tau__blk874_dn9 = assign29470_e31957_d_n9;

        let (assign29480_e31971, assign29480_e31971_d_n4, assign29480_e31971_d_n6, assign29480_e31971_d_n7, assign29480_e31971_d_n8, assign29480_e31971_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29480_e31969: f64 = (locals.var_spsub_a__blk871 + locals.var_spsub_c__blk873);
        (assign29480_e31969, (locals.var_spsub_a__blk871_dn4 + locals.var_spsub_c__blk873_dn4), (locals.var_spsub_a__blk871_dn6 + locals.var_spsub_c__blk873_dn6), (locals.var_spsub_a__blk871_dn7 + locals.var_spsub_c__blk873_dn7), (locals.var_spsub_a__blk871_dn8 + locals.var_spsub_c__blk873_dn8), (locals.var_spsub_a__blk871_dn9 + locals.var_spsub_c__blk873_dn9),)
    } else {
        (locals.var_nu__blk861, locals.var_nu__blk861_dn4, locals.var_nu__blk861_dn6, locals.var_nu__blk861_dn7, locals.var_nu__blk861_dn8, locals.var_nu__blk861_dn9,)
    }
};
        locals.var_nu__blk861 = assign29480_e31971;
        locals.var_nu__blk861_dn4 = assign29480_e31971_d_n4;
        locals.var_nu__blk861_dn6 = assign29480_e31971_d_n6;
        locals.var_nu__blk861_dn7 = assign29480_e31971_d_n7;
        locals.var_nu__blk861_dn8 = assign29480_e31971_d_n8;
        locals.var_nu__blk861_dn9 = assign29480_e31971_d_n9;

        let (assign29490_e31997, assign29490_e31997_d_n4, assign29490_e31997_d_n6, assign29490_e31997_d_n7, assign29490_e31997_d_n8, assign29490_e31997_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29490_e31983: f64 = (locals.var_nu__blk861 * locals.var_nu__blk861);
        let assign29490_e31987: f64 = (0.5 * locals.var_spsub_c__blk873);
        let assign29490_e31989: f64 = (assign29490_e31987 * locals.var_spsub_c__blk873);
        let assign29490_e31992: f64 = (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872);
        let assign29490_e31993: f64 = (assign29490_e31989 - assign29490_e31992);
        let assign29490_e31994: f64 = (locals.var_spsub_tau__blk874 * assign29490_e31993);
        let assign29490_e31995: f64 = (assign29490_e31983 + assign29490_e31994);
        (assign29490_e31995, (((locals.var_nu__blk861_dn4 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn4)) + ((locals.var_spsub_tau__blk874_dn4 * assign29490_e31993) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn4) * locals.var_spsub_c__blk873) + (assign29490_e31987 * locals.var_spsub_c__blk873_dn4)) - ((locals.var_spsub_a__blk871_dn4 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn4)))))), (((locals.var_nu__blk861_dn6 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn6)) + ((locals.var_spsub_tau__blk874_dn6 * assign29490_e31993) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn6) * locals.var_spsub_c__blk873) + (assign29490_e31987 * locals.var_spsub_c__blk873_dn6)) - ((locals.var_spsub_a__blk871_dn6 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn6)))))), (((locals.var_nu__blk861_dn7 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn7)) + ((locals.var_spsub_tau__blk874_dn7 * assign29490_e31993) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn7) * locals.var_spsub_c__blk873) + (assign29490_e31987 * locals.var_spsub_c__blk873_dn7)) - ((locals.var_spsub_a__blk871_dn7 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn7)))))), (((locals.var_nu__blk861_dn8 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn8)) + ((locals.var_spsub_tau__blk874_dn8 * assign29490_e31993) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn8) * locals.var_spsub_c__blk873) + (assign29490_e31987 * locals.var_spsub_c__blk873_dn8)) - ((locals.var_spsub_a__blk871_dn8 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn8)))))), (((locals.var_nu__blk861_dn9 * locals.var_nu__blk861) + (locals.var_nu__blk861 * locals.var_nu__blk861_dn9)) + ((locals.var_spsub_tau__blk874_dn9 * assign29490_e31993) + (locals.var_spsub_tau__blk874 * ((((0.5 * locals.var_spsub_c__blk873_dn9) * locals.var_spsub_c__blk873) + (assign29490_e31987 * locals.var_spsub_c__blk873_dn9)) - ((locals.var_spsub_a__blk871_dn9 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn9)))))),)
    } else {
        (locals.var_mutau__blk862, locals.var_mutau__blk862_dn4, locals.var_mutau__blk862_dn6, locals.var_mutau__blk862_dn7, locals.var_mutau__blk862_dn8, locals.var_mutau__blk862_dn9,)
    }
};
        locals.var_mutau__blk862 = assign29490_e31997;
        locals.var_mutau__blk862_dn4 = assign29490_e31997_d_n4;
        locals.var_mutau__blk862_dn6 = assign29490_e31997_d_n6;
        locals.var_mutau__blk862_dn7 = assign29490_e31997_d_n7;
        locals.var_mutau__blk862_dn8 = assign29490_e31997_d_n8;
        locals.var_mutau__blk862_dn9 = assign29490_e31997_d_n9;

        let (assign29500_e32037, assign29500_e32037_d_n4, assign29500_e32037_d_n6, assign29500_e32037_d_n7, assign29500_e32037_d_n8, assign29500_e32037_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29500_e32010: f64 = (locals.var_spsub_a__blk871 * locals.var_nu__blk861);
        let assign29500_e32012: f64 = (assign29500_e32010 * locals.var_spsub_tau__blk874);
        let assign29500_e32016: f64 = (locals.var_nu__blk861 / locals.var_mutau__blk862);
        let assign29500_e32018: f64 = (assign29500_e32016 * locals.var_spsub_tau__blk874);
        let assign29500_e32020: f64 = (assign29500_e32018 * locals.var_spsub_tau__blk874);
        let assign29500_e32022: f64 = (assign29500_e32020 * locals.var_spsub_c__blk873);
        let assign29500_e32025: f64 = (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873);
        let assign29500_e32027: f64 = (assign29500_e32025 * 0.3333333333333);
        let assign29500_e32030: f64 = (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872);
        let assign29500_e32031: f64 = (assign29500_e32027 - assign29500_e32030);
        let assign29500_e32032: f64 = (assign29500_e32022 * assign29500_e32031);
        let assign29500_e32033: f64 = (locals.var_mutau__blk862 + assign29500_e32032);
        let assign29500_e32034: f64 = (assign29500_e32012 / assign29500_e32033);
        let assign29500_e32035: f64 = (locals.var_spsub_eta__blk870 + assign29500_e32034);
        (assign29500_e32035, (locals.var_spsub_eta__blk870_dn4 + (((((((locals.var_spsub_a__blk871_dn4 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn4)) * locals.var_spsub_tau__blk874) + (assign29500_e32010 * locals.var_spsub_tau__blk874_dn4)) * assign29500_e32033) - (assign29500_e32012 * (locals.var_mutau__blk862_dn4 + (((((((((((locals.var_nu__blk861_dn4 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn4)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29500_e32016 * locals.var_spsub_tau__blk874_dn4)) * locals.var_spsub_tau__blk874) + (assign29500_e32018 * locals.var_spsub_tau__blk874_dn4)) * locals.var_spsub_c__blk873) + (assign29500_e32020 * locals.var_spsub_c__blk873_dn4)) * assign29500_e32031) + (assign29500_e32022 * ((((locals.var_spsub_c__blk873_dn4 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn4)) * 0.3333333333333) - ((locals.var_spsub_a__blk871_dn4 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn4)))))))) / (assign29500_e32033 * assign29500_e32033))), (locals.var_spsub_eta__blk870_dn6 + (((((((locals.var_spsub_a__blk871_dn6 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn6)) * locals.var_spsub_tau__blk874) + (assign29500_e32010 * locals.var_spsub_tau__blk874_dn6)) * assign29500_e32033) - (assign29500_e32012 * (locals.var_mutau__blk862_dn6 + (((((((((((locals.var_nu__blk861_dn6 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn6)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29500_e32016 * locals.var_spsub_tau__blk874_dn6)) * locals.var_spsub_tau__blk874) + (assign29500_e32018 * locals.var_spsub_tau__blk874_dn6)) * locals.var_spsub_c__blk873) + (assign29500_e32020 * locals.var_spsub_c__blk873_dn6)) * assign29500_e32031) + (assign29500_e32022 * ((((locals.var_spsub_c__blk873_dn6 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn6)) * 0.3333333333333) - ((locals.var_spsub_a__blk871_dn6 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn6)))))))) / (assign29500_e32033 * assign29500_e32033))), (locals.var_spsub_eta__blk870_dn7 + (((((((locals.var_spsub_a__blk871_dn7 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn7)) * locals.var_spsub_tau__blk874) + (assign29500_e32010 * locals.var_spsub_tau__blk874_dn7)) * assign29500_e32033) - (assign29500_e32012 * (locals.var_mutau__blk862_dn7 + (((((((((((locals.var_nu__blk861_dn7 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn7)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29500_e32016 * locals.var_spsub_tau__blk874_dn7)) * locals.var_spsub_tau__blk874) + (assign29500_e32018 * locals.var_spsub_tau__blk874_dn7)) * locals.var_spsub_c__blk873) + (assign29500_e32020 * locals.var_spsub_c__blk873_dn7)) * assign29500_e32031) + (assign29500_e32022 * ((((locals.var_spsub_c__blk873_dn7 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn7)) * 0.3333333333333) - ((locals.var_spsub_a__blk871_dn7 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn7)))))))) / (assign29500_e32033 * assign29500_e32033))), (locals.var_spsub_eta__blk870_dn8 + (((((((locals.var_spsub_a__blk871_dn8 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn8)) * locals.var_spsub_tau__blk874) + (assign29500_e32010 * locals.var_spsub_tau__blk874_dn8)) * assign29500_e32033) - (assign29500_e32012 * (locals.var_mutau__blk862_dn8 + (((((((((((locals.var_nu__blk861_dn8 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn8)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29500_e32016 * locals.var_spsub_tau__blk874_dn8)) * locals.var_spsub_tau__blk874) + (assign29500_e32018 * locals.var_spsub_tau__blk874_dn8)) * locals.var_spsub_c__blk873) + (assign29500_e32020 * locals.var_spsub_c__blk873_dn8)) * assign29500_e32031) + (assign29500_e32022 * ((((locals.var_spsub_c__blk873_dn8 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn8)) * 0.3333333333333) - ((locals.var_spsub_a__blk871_dn8 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn8)))))))) / (assign29500_e32033 * assign29500_e32033))), (locals.var_spsub_eta__blk870_dn9 + (((((((locals.var_spsub_a__blk871_dn9 * locals.var_nu__blk861) + (locals.var_spsub_a__blk871 * locals.var_nu__blk861_dn9)) * locals.var_spsub_tau__blk874) + (assign29500_e32010 * locals.var_spsub_tau__blk874_dn9)) * assign29500_e32033) - (assign29500_e32012 * (locals.var_mutau__blk862_dn9 + (((((((((((locals.var_nu__blk861_dn9 * locals.var_mutau__blk862) - (locals.var_nu__blk861 * locals.var_mutau__blk862_dn9)) / (locals.var_mutau__blk862 * locals.var_mutau__blk862)) * locals.var_spsub_tau__blk874) + (assign29500_e32016 * locals.var_spsub_tau__blk874_dn9)) * locals.var_spsub_tau__blk874) + (assign29500_e32018 * locals.var_spsub_tau__blk874_dn9)) * locals.var_spsub_c__blk873) + (assign29500_e32020 * locals.var_spsub_c__blk873_dn9)) * assign29500_e32031) + (assign29500_e32022 * ((((locals.var_spsub_c__blk873_dn9 * locals.var_spsub_c__blk873) + (locals.var_spsub_c__blk873 * locals.var_spsub_c__blk873_dn9)) * 0.3333333333333) - ((locals.var_spsub_a__blk871_dn9 * locals.var_spsub_b__blk872) + (locals.var_spsub_a__blk871 * locals.var_spsub_b__blk872_dn9)))))))) / (assign29500_e32033 * assign29500_e32033))),)
    } else {
        (locals.var_spsub_x0__blk889, locals.var_spsub_x0__blk889_dn4, locals.var_spsub_x0__blk889_dn6, locals.var_spsub_x0__blk889_dn7, locals.var_spsub_x0__blk889_dn8, locals.var_spsub_x0__blk889_dn9,)
    }
};
        locals.var_spsub_x0__blk889 = assign29500_e32037;
        locals.var_spsub_x0__blk889_dn4 = assign29500_e32037_d_n4;
        locals.var_spsub_x0__blk889_dn6 = assign29500_e32037_d_n6;
        locals.var_spsub_x0__blk889_dn7 = assign29500_e32037_d_n7;
        locals.var_spsub_x0__blk889_dn8 = assign29500_e32037_d_n8;
        locals.var_spsub_x0__blk889_dn9 = assign29500_e32037_d_n9;

        let assign29510_e32040: f64 = if locals.var_spsub_x0__blk889 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1091 = assign29510_e32040;

        let (assign29520_e32055, assign29520_e32055_d_n4, assign29520_e32055_d_n6, assign29520_e32055_d_n7, assign29520_e32055_d_n8, assign29520_e32055_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1091 != 0.0)) {
        let assign29520_e32053: f64 = (locals.var_spsub_x0__blk889).exp();
        (assign29520_e32053, (assign29520_e32053 * locals.var_spsub_x0__blk889_dn4), (assign29520_e32053 * locals.var_spsub_x0__blk889_dn6), (assign29520_e32053 * locals.var_spsub_x0__blk889_dn7), (assign29520_e32053 * locals.var_spsub_x0__blk889_dn8), (assign29520_e32053 * locals.var_spsub_x0__blk889_dn9),)
    } else {
        (locals.var_spsub_delta0__blk876, locals.var_spsub_delta0__blk876_dn4, locals.var_spsub_delta0__blk876_dn6, locals.var_spsub_delta0__blk876_dn7, locals.var_spsub_delta0__blk876_dn8, locals.var_spsub_delta0__blk876_dn9,)
    }
};
        locals.var_spsub_delta0__blk876 = assign29520_e32055;
        locals.var_spsub_delta0__blk876_dn4 = assign29520_e32055_d_n4;
        locals.var_spsub_delta0__blk876_dn6 = assign29520_e32055_d_n6;
        locals.var_spsub_delta0__blk876_dn7 = assign29520_e32055_d_n7;
        locals.var_spsub_delta0__blk876_dn8 = assign29520_e32055_d_n8;
        locals.var_spsub_delta0__blk876_dn9 = assign29520_e32055_d_n9;

        let (assign29530_e32071, assign29530_e32071_d_n4, assign29530_e32071_d_n6, assign29530_e32071_d_n7, assign29530_e32071_d_n8, assign29530_e32071_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1091 != 0.0)) {
        let assign29530_e32069: f64 = (1.0 / locals.var_spsub_delta0__blk876);
        (assign29530_e32069, (-(locals.var_spsub_delta0__blk876_dn4 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))), (-(locals.var_spsub_delta0__blk876_dn6 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))), (-(locals.var_spsub_delta0__blk876_dn7 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))), (-(locals.var_spsub_delta0__blk876_dn8 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))), (-(locals.var_spsub_delta0__blk876_dn9 / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876))),)
    } else {
        (locals.var_spsub_delta1__blk877, locals.var_spsub_delta1__blk877_dn4, locals.var_spsub_delta1__blk877_dn6, locals.var_spsub_delta1__blk877_dn7, locals.var_spsub_delta1__blk877_dn8, locals.var_spsub_delta1__blk877_dn9,)
    }
};
        locals.var_spsub_delta1__blk877 = assign29530_e32071;
        locals.var_spsub_delta1__blk877_dn4 = assign29530_e32071_d_n4;
        locals.var_spsub_delta1__blk877_dn6 = assign29530_e32071_d_n6;
        locals.var_spsub_delta1__blk877_dn7 = assign29530_e32071_d_n7;
        locals.var_spsub_delta1__blk877_dn8 = assign29530_e32071_d_n8;
        locals.var_spsub_delta1__blk877_dn9 = assign29530_e32071_d_n9;

        let (assign29540_e32087, assign29540_e32087_d_n4, assign29540_e32087_d_n6, assign29540_e32087_d_n7, assign29540_e32087_d_n8, assign29540_e32087_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1091 != 0.0)) {
        let assign29540_e32085: f64 = (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876);
        (assign29540_e32085, ((locals.var_spsub_delta__blk867_dn4 * locals.var_spsub_delta0__blk876) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn4)), ((locals.var_spsub_delta__blk867_dn6 * locals.var_spsub_delta0__blk876) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn6)), ((locals.var_spsub_delta__blk867_dn7 * locals.var_spsub_delta0__blk876) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn7)), ((locals.var_spsub_delta__blk867_dn8 * locals.var_spsub_delta0__blk876) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn8)), ((locals.var_spsub_delta__blk867_dn9 * locals.var_spsub_delta0__blk876) + (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn9)),)
    } else {
        (locals.var_spsub_delta0__blk876, locals.var_spsub_delta0__blk876_dn4, locals.var_spsub_delta0__blk876_dn6, locals.var_spsub_delta0__blk876_dn7, locals.var_spsub_delta0__blk876_dn8, locals.var_spsub_delta0__blk876_dn9,)
    }
};
        locals.var_spsub_delta0__blk876 = assign29540_e32087;
        locals.var_spsub_delta0__blk876_dn4 = assign29540_e32087_d_n4;
        locals.var_spsub_delta0__blk876_dn6 = assign29540_e32087_d_n6;
        locals.var_spsub_delta0__blk876_dn7 = assign29540_e32087_d_n7;
        locals.var_spsub_delta0__blk876_dn8 = assign29540_e32087_d_n8;
        locals.var_spsub_delta0__blk876_dn9 = assign29540_e32087_d_n9;

        let assign29550_e32091: f64 = (locals.var_xn_sub - 80.0);
        let assign29550_e32092: f64 = if locals.var_spsub_x0__blk889 > assign29550_e32091 { 1.0 } else { 0.0 };
        locals.var_guard1092 = assign29550_e32092;

        let (assign29560_e32112, assign29560_e32112_d_n4, assign29560_e32112_d_n6, assign29560_e32112_d_n7, assign29560_e32112_d_n8, assign29560_e32112_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1091 == 0.0)) && (locals.var_guard1092 != 0.0)) {
        let assign29560_e32109: f64 = (locals.var_spsub_x0__blk889 - locals.var_xn_sub);
        let assign29560_e32110: f64 = (assign29560_e32109).exp();
        (assign29560_e32110, (assign29560_e32110 * (locals.var_spsub_x0__blk889_dn4 - locals.var_xn_sub_dn4)), (assign29560_e32110 * (locals.var_spsub_x0__blk889_dn6 - locals.var_xn_sub_dn6)), (assign29560_e32110 * (locals.var_spsub_x0__blk889_dn7 - locals.var_xn_sub_dn7)), (assign29560_e32110 * (locals.var_spsub_x0__blk889_dn8 - locals.var_xn_sub_dn8)), (assign29560_e32110 * (locals.var_spsub_x0__blk889_dn9 - locals.var_xn_sub_dn9)),)
    } else {
        (locals.var_spsub_delta0__blk876, locals.var_spsub_delta0__blk876_dn4, locals.var_spsub_delta0__blk876_dn6, locals.var_spsub_delta0__blk876_dn7, locals.var_spsub_delta0__blk876_dn8, locals.var_spsub_delta0__blk876_dn9,)
    }
};
        locals.var_spsub_delta0__blk876 = assign29560_e32112;
        locals.var_spsub_delta0__blk876_dn4 = assign29560_e32112_d_n4;
        locals.var_spsub_delta0__blk876_dn6 = assign29560_e32112_d_n6;
        locals.var_spsub_delta0__blk876_dn7 = assign29560_e32112_d_n7;
        locals.var_spsub_delta0__blk876_dn8 = assign29560_e32112_d_n8;
        locals.var_spsub_delta0__blk876_dn9 = assign29560_e32112_d_n9;

        let (assign29570_e32131, assign29570_e32131_d_n4, assign29570_e32131_d_n6, assign29570_e32131_d_n7, assign29570_e32131_d_n8, assign29570_e32131_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1091 == 0.0)) && (locals.var_guard1092 != 0.0)) {
        let assign29570_e32129: f64 = (locals.var_spsub_delta__blk867 / locals.var_spsub_delta0__blk876);
        (assign29570_e32129, (((locals.var_spsub_delta__blk867_dn4 * locals.var_spsub_delta0__blk876) - (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn4)) / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876)), (((locals.var_spsub_delta__blk867_dn6 * locals.var_spsub_delta0__blk876) - (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn6)) / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876)), (((locals.var_spsub_delta__blk867_dn7 * locals.var_spsub_delta0__blk876) - (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn7)) / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876)), (((locals.var_spsub_delta__blk867_dn8 * locals.var_spsub_delta0__blk876) - (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn8)) / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876)), (((locals.var_spsub_delta__blk867_dn9 * locals.var_spsub_delta0__blk876) - (locals.var_spsub_delta__blk867 * locals.var_spsub_delta0__blk876_dn9)) / (locals.var_spsub_delta0__blk876 * locals.var_spsub_delta0__blk876)),)
    } else {
        (locals.var_spsub_delta1__blk877, locals.var_spsub_delta1__blk877_dn4, locals.var_spsub_delta1__blk877_dn6, locals.var_spsub_delta1__blk877_dn7, locals.var_spsub_delta1__blk877_dn8, locals.var_spsub_delta1__blk877_dn9,)
    }
};
        locals.var_spsub_delta1__blk877 = assign29570_e32131;
        locals.var_spsub_delta1__blk877_dn4 = assign29570_e32131_d_n4;
        locals.var_spsub_delta1__blk877_dn6 = assign29570_e32131_d_n6;
        locals.var_spsub_delta1__blk877_dn7 = assign29570_e32131_d_n7;
        locals.var_spsub_delta1__blk877_dn8 = assign29570_e32131_d_n8;
        locals.var_spsub_delta1__blk877_dn9 = assign29570_e32131_d_n9;

        let (assign29580_e32177, assign29580_e32177_d_n4, assign29580_e32177_d_n6, assign29580_e32177_d_n7, assign29580_e32177_d_n8, assign29580_e32177_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1091 == 0.0)) && (locals.var_guard1092 == 0.0)) {
        let assign29580_e32151: f64 = (locals.var_xn_sub - locals.var_spsub_x0__blk889);
        let assign29580_e32153: f64 = (assign29580_e32151 - 80.0);
        let assign29580_e32158: f64 = (locals.var_xn_sub - locals.var_spsub_x0__blk889);
        let assign29580_e32160: f64 = (assign29580_e32158 - 80.0);
        let assign29580_e32161: f64 = (0.5 * assign29580_e32160);
        let assign29580_e32165: f64 = (locals.var_xn_sub - locals.var_spsub_x0__blk889);
        let assign29580_e32167: f64 = (assign29580_e32165 - 80.0);
        let assign29580_e32169: f64 = (assign29580_e32167 * 0.3333333333333);
        let assign29580_e32170: f64 = (1.0 + assign29580_e32169);
        let assign29580_e32171: f64 = (assign29580_e32161 * assign29580_e32170);
        let assign29580_e32172: f64 = (1.0 + assign29580_e32171);
        let assign29580_e32173: f64 = (assign29580_e32153 * assign29580_e32172);
        let assign29580_e32174: f64 = (1.0 + assign29580_e32173);
        let assign29580_e32175: f64 = (1.80485e-35 / assign29580_e32174);
        (assign29580_e32175, (-((1.80485e-35 * (((locals.var_xn_sub_dn4 - locals.var_spsub_x0__blk889_dn4) * assign29580_e32172) + (assign29580_e32153 * (((0.5 * (locals.var_xn_sub_dn4 - locals.var_spsub_x0__blk889_dn4)) * assign29580_e32170) + (assign29580_e32161 * ((locals.var_xn_sub_dn4 - locals.var_spsub_x0__blk889_dn4) * 0.3333333333333)))))) / (assign29580_e32174 * assign29580_e32174))), (-((1.80485e-35 * (((locals.var_xn_sub_dn6 - locals.var_spsub_x0__blk889_dn6) * assign29580_e32172) + (assign29580_e32153 * (((0.5 * (locals.var_xn_sub_dn6 - locals.var_spsub_x0__blk889_dn6)) * assign29580_e32170) + (assign29580_e32161 * ((locals.var_xn_sub_dn6 - locals.var_spsub_x0__blk889_dn6) * 0.3333333333333)))))) / (assign29580_e32174 * assign29580_e32174))), (-((1.80485e-35 * (((locals.var_xn_sub_dn7 - locals.var_spsub_x0__blk889_dn7) * assign29580_e32172) + (assign29580_e32153 * (((0.5 * (locals.var_xn_sub_dn7 - locals.var_spsub_x0__blk889_dn7)) * assign29580_e32170) + (assign29580_e32161 * ((locals.var_xn_sub_dn7 - locals.var_spsub_x0__blk889_dn7) * 0.3333333333333)))))) / (assign29580_e32174 * assign29580_e32174))), (-((1.80485e-35 * (((locals.var_xn_sub_dn8 - locals.var_spsub_x0__blk889_dn8) * assign29580_e32172) + (assign29580_e32153 * (((0.5 * (locals.var_xn_sub_dn8 - locals.var_spsub_x0__blk889_dn8)) * assign29580_e32170) + (assign29580_e32161 * ((locals.var_xn_sub_dn8 - locals.var_spsub_x0__blk889_dn8) * 0.3333333333333)))))) / (assign29580_e32174 * assign29580_e32174))), (-((1.80485e-35 * (((locals.var_xn_sub_dn9 - locals.var_spsub_x0__blk889_dn9) * assign29580_e32172) + (assign29580_e32153 * (((0.5 * (locals.var_xn_sub_dn9 - locals.var_spsub_x0__blk889_dn9)) * assign29580_e32170) + (assign29580_e32161 * ((locals.var_xn_sub_dn9 - locals.var_spsub_x0__blk889_dn9) * 0.3333333333333)))))) / (assign29580_e32174 * assign29580_e32174))),)
    } else {
        (locals.var_spsub_delta0__blk876, locals.var_spsub_delta0__blk876_dn4, locals.var_spsub_delta0__blk876_dn6, locals.var_spsub_delta0__blk876_dn7, locals.var_spsub_delta0__blk876_dn8, locals.var_spsub_delta0__blk876_dn9,)
    }
};
        locals.var_spsub_delta0__blk876 = assign29580_e32177;
        locals.var_spsub_delta0__blk876_dn4 = assign29580_e32177_d_n4;
        locals.var_spsub_delta0__blk876_dn6 = assign29580_e32177_d_n6;
        locals.var_spsub_delta0__blk876_dn7 = assign29580_e32177_d_n7;
        locals.var_spsub_delta0__blk876_dn8 = assign29580_e32177_d_n8;
        locals.var_spsub_delta0__blk876_dn9 = assign29580_e32177_d_n9;

        let (assign29590_e32217, assign29590_e32217_d_n4, assign29590_e32217_d_n6, assign29590_e32217_d_n7, assign29590_e32217_d_n8, assign29590_e32217_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) && (locals.var_guard1091 == 0.0)) && (locals.var_guard1092 == 0.0)) {
        let assign29590_e32197: f64 = (locals.var_spsub_x0__blk889 - 80.0);
        let assign29590_e32202: f64 = (locals.var_spsub_x0__blk889 - 80.0);
        let assign29590_e32203: f64 = (0.5 * assign29590_e32202);
        let assign29590_e32207: f64 = (locals.var_spsub_x0__blk889 - 80.0);
        let assign29590_e32209: f64 = (assign29590_e32207 * 0.3333333333333);
        let assign29590_e32210: f64 = (1.0 + assign29590_e32209);
        let assign29590_e32211: f64 = (assign29590_e32203 * assign29590_e32210);
        let assign29590_e32212: f64 = (1.0 + assign29590_e32211);
        let assign29590_e32213: f64 = (assign29590_e32197 * assign29590_e32212);
        let assign29590_e32214: f64 = (1.0 + assign29590_e32213);
        let assign29590_e32215: f64 = (1.80485e-35 / assign29590_e32214);
        (assign29590_e32215, (-((1.80485e-35 * ((locals.var_spsub_x0__blk889_dn4 * assign29590_e32212) + (assign29590_e32197 * (((0.5 * locals.var_spsub_x0__blk889_dn4) * assign29590_e32210) + (assign29590_e32203 * (locals.var_spsub_x0__blk889_dn4 * 0.3333333333333)))))) / (assign29590_e32214 * assign29590_e32214))), (-((1.80485e-35 * ((locals.var_spsub_x0__blk889_dn6 * assign29590_e32212) + (assign29590_e32197 * (((0.5 * locals.var_spsub_x0__blk889_dn6) * assign29590_e32210) + (assign29590_e32203 * (locals.var_spsub_x0__blk889_dn6 * 0.3333333333333)))))) / (assign29590_e32214 * assign29590_e32214))), (-((1.80485e-35 * ((locals.var_spsub_x0__blk889_dn7 * assign29590_e32212) + (assign29590_e32197 * (((0.5 * locals.var_spsub_x0__blk889_dn7) * assign29590_e32210) + (assign29590_e32203 * (locals.var_spsub_x0__blk889_dn7 * 0.3333333333333)))))) / (assign29590_e32214 * assign29590_e32214))), (-((1.80485e-35 * ((locals.var_spsub_x0__blk889_dn8 * assign29590_e32212) + (assign29590_e32197 * (((0.5 * locals.var_spsub_x0__blk889_dn8) * assign29590_e32210) + (assign29590_e32203 * (locals.var_spsub_x0__blk889_dn8 * 0.3333333333333)))))) / (assign29590_e32214 * assign29590_e32214))), (-((1.80485e-35 * ((locals.var_spsub_x0__blk889_dn9 * assign29590_e32212) + (assign29590_e32197 * (((0.5 * locals.var_spsub_x0__blk889_dn9) * assign29590_e32210) + (assign29590_e32203 * (locals.var_spsub_x0__blk889_dn9 * 0.3333333333333)))))) / (assign29590_e32214 * assign29590_e32214))),)
    } else {
        (locals.var_spsub_delta1__blk877, locals.var_spsub_delta1__blk877_dn4, locals.var_spsub_delta1__blk877_dn6, locals.var_spsub_delta1__blk877_dn7, locals.var_spsub_delta1__blk877_dn8, locals.var_spsub_delta1__blk877_dn9,)
    }
};
        locals.var_spsub_delta1__blk877 = assign29590_e32217;
        locals.var_spsub_delta1__blk877_dn4 = assign29590_e32217_d_n4;
        locals.var_spsub_delta1__blk877_dn6 = assign29590_e32217_d_n6;
        locals.var_spsub_delta1__blk877_dn7 = assign29590_e32217_d_n7;
        locals.var_spsub_delta1__blk877_dn8 = assign29590_e32217_d_n8;
        locals.var_spsub_delta1__blk877_dn9 = assign29590_e32217_d_n9;

        let (assign29600_e32235, assign29600_e32235_d_n4, assign29600_e32235_d_n6, assign29600_e32235_d_n7, assign29600_e32235_d_n8, assign29600_e32235_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29600_e32231: f64 = (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889);
        let assign29600_e32232: f64 = (2.0 + assign29600_e32231);
        let assign29600_e32233: f64 = (1.0 / assign29600_e32232);
        (assign29600_e32233, (-(((locals.var_spsub_x0__blk889_dn4 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn4)) / (assign29600_e32232 * assign29600_e32232))), (-(((locals.var_spsub_x0__blk889_dn6 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn6)) / (assign29600_e32232 * assign29600_e32232))), (-(((locals.var_spsub_x0__blk889_dn7 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn7)) / (assign29600_e32232 * assign29600_e32232))), (-(((locals.var_spsub_x0__blk889_dn8 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn8)) / (assign29600_e32232 * assign29600_e32232))), (-(((locals.var_spsub_x0__blk889_dn9 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn9)) / (assign29600_e32232 * assign29600_e32232))),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29600_e32235;
        locals.var_spsub_temp__blk863_dn4 = assign29600_e32235_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29600_e32235_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29600_e32235_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29600_e32235_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29600_e32235_d_n9;

    }

    pub(super) fn stamp_transient_block_78(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29610_e32251, assign29610_e32251_d_n4, assign29610_e32251_d_n6, assign29610_e32251_d_n7, assign29610_e32251_d_n8, assign29610_e32251_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29610_e32247: f64 = (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889);
        let assign29610_e32249: f64 = (assign29610_e32247 * locals.var_spsub_temp__blk863);
        (assign29610_e32249, ((((locals.var_spsub_x0__blk889_dn4 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn4)) * locals.var_spsub_temp__blk863) + (assign29610_e32247 * locals.var_spsub_temp__blk863_dn4)), ((((locals.var_spsub_x0__blk889_dn6 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn6)) * locals.var_spsub_temp__blk863) + (assign29610_e32247 * locals.var_spsub_temp__blk863_dn6)), ((((locals.var_spsub_x0__blk889_dn7 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn7)) * locals.var_spsub_temp__blk863) + (assign29610_e32247 * locals.var_spsub_temp__blk863_dn7)), ((((locals.var_spsub_x0__blk889_dn8 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn8)) * locals.var_spsub_temp__blk863) + (assign29610_e32247 * locals.var_spsub_temp__blk863_dn8)), ((((locals.var_spsub_x0__blk889_dn9 * locals.var_spsub_x0__blk889) + (locals.var_spsub_x0__blk889 * locals.var_spsub_x0__blk889_dn9)) * locals.var_spsub_temp__blk863) + (assign29610_e32247 * locals.var_spsub_temp__blk863_dn9)),)
    } else {
        (locals.var_spsub_xi0__blk878, locals.var_spsub_xi0__blk878_dn4, locals.var_spsub_xi0__blk878_dn6, locals.var_spsub_xi0__blk878_dn7, locals.var_spsub_xi0__blk878_dn8, locals.var_spsub_xi0__blk878_dn9,)
    }
};
        locals.var_spsub_xi0__blk878 = assign29610_e32251;
        locals.var_spsub_xi0__blk878_dn4 = assign29610_e32251_d_n4;
        locals.var_spsub_xi0__blk878_dn6 = assign29610_e32251_d_n6;
        locals.var_spsub_xi0__blk878_dn7 = assign29610_e32251_d_n7;
        locals.var_spsub_xi0__blk878_dn8 = assign29610_e32251_d_n8;
        locals.var_spsub_xi0__blk878_dn9 = assign29610_e32251_d_n9;

        let (assign29620_e32269, assign29620_e32269_d_n4, assign29620_e32269_d_n6, assign29620_e32269_d_n7, assign29620_e32269_d_n8, assign29620_e32269_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29620_e32264: f64 = (locals.var_spsub_x0__blk889 * locals.var_spsub_temp__blk863);
        let assign29620_e32266: f64 = (assign29620_e32264 * locals.var_spsub_temp__blk863);
        let assign29620_e32267: f64 = (4.0 * assign29620_e32266);
        (assign29620_e32267, (4.0 * ((((locals.var_spsub_x0__blk889_dn4 * locals.var_spsub_temp__blk863) + (locals.var_spsub_x0__blk889 * locals.var_spsub_temp__blk863_dn4)) * locals.var_spsub_temp__blk863) + (assign29620_e32264 * locals.var_spsub_temp__blk863_dn4))), (4.0 * ((((locals.var_spsub_x0__blk889_dn6 * locals.var_spsub_temp__blk863) + (locals.var_spsub_x0__blk889 * locals.var_spsub_temp__blk863_dn6)) * locals.var_spsub_temp__blk863) + (assign29620_e32264 * locals.var_spsub_temp__blk863_dn6))), (4.0 * ((((locals.var_spsub_x0__blk889_dn7 * locals.var_spsub_temp__blk863) + (locals.var_spsub_x0__blk889 * locals.var_spsub_temp__blk863_dn7)) * locals.var_spsub_temp__blk863) + (assign29620_e32264 * locals.var_spsub_temp__blk863_dn7))), (4.0 * ((((locals.var_spsub_x0__blk889_dn8 * locals.var_spsub_temp__blk863) + (locals.var_spsub_x0__blk889 * locals.var_spsub_temp__blk863_dn8)) * locals.var_spsub_temp__blk863) + (assign29620_e32264 * locals.var_spsub_temp__blk863_dn8))), (4.0 * ((((locals.var_spsub_x0__blk889_dn9 * locals.var_spsub_temp__blk863) + (locals.var_spsub_x0__blk889 * locals.var_spsub_temp__blk863_dn9)) * locals.var_spsub_temp__blk863) + (assign29620_e32264 * locals.var_spsub_temp__blk863_dn9))),)
    } else {
        (locals.var_spsub_xi1__blk879, locals.var_spsub_xi1__blk879_dn4, locals.var_spsub_xi1__blk879_dn6, locals.var_spsub_xi1__blk879_dn7, locals.var_spsub_xi1__blk879_dn8, locals.var_spsub_xi1__blk879_dn9,)
    }
};
        locals.var_spsub_xi1__blk879 = assign29620_e32269;
        locals.var_spsub_xi1__blk879_dn4 = assign29620_e32269_d_n4;
        locals.var_spsub_xi1__blk879_dn6 = assign29620_e32269_d_n6;
        locals.var_spsub_xi1__blk879_dn7 = assign29620_e32269_d_n7;
        locals.var_spsub_xi1__blk879_dn8 = assign29620_e32269_d_n8;
        locals.var_spsub_xi1__blk879_dn9 = assign29620_e32269_d_n9;

        let (assign29630_e32291, assign29630_e32291_d_n4, assign29630_e32291_d_n6, assign29630_e32291_d_n7, assign29630_e32291_d_n8, assign29630_e32291_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29630_e32281: f64 = (8.0 * locals.var_spsub_temp__blk863);
        let assign29630_e32284: f64 = (12.0 * locals.var_spsub_xi0__blk878);
        let assign29630_e32285: f64 = (assign29630_e32281 - assign29630_e32284);
        let assign29630_e32287: f64 = (assign29630_e32285 * locals.var_spsub_temp__blk863);
        let assign29630_e32289: f64 = (assign29630_e32287 * locals.var_spsub_temp__blk863);
        (assign29630_e32289, ((((((8.0 * locals.var_spsub_temp__blk863_dn4) - (12.0 * locals.var_spsub_xi0__blk878_dn4)) * locals.var_spsub_temp__blk863) + (assign29630_e32285 * locals.var_spsub_temp__blk863_dn4)) * locals.var_spsub_temp__blk863) + (assign29630_e32287 * locals.var_spsub_temp__blk863_dn4)), ((((((8.0 * locals.var_spsub_temp__blk863_dn6) - (12.0 * locals.var_spsub_xi0__blk878_dn6)) * locals.var_spsub_temp__blk863) + (assign29630_e32285 * locals.var_spsub_temp__blk863_dn6)) * locals.var_spsub_temp__blk863) + (assign29630_e32287 * locals.var_spsub_temp__blk863_dn6)), ((((((8.0 * locals.var_spsub_temp__blk863_dn7) - (12.0 * locals.var_spsub_xi0__blk878_dn7)) * locals.var_spsub_temp__blk863) + (assign29630_e32285 * locals.var_spsub_temp__blk863_dn7)) * locals.var_spsub_temp__blk863) + (assign29630_e32287 * locals.var_spsub_temp__blk863_dn7)), ((((((8.0 * locals.var_spsub_temp__blk863_dn8) - (12.0 * locals.var_spsub_xi0__blk878_dn8)) * locals.var_spsub_temp__blk863) + (assign29630_e32285 * locals.var_spsub_temp__blk863_dn8)) * locals.var_spsub_temp__blk863) + (assign29630_e32287 * locals.var_spsub_temp__blk863_dn8)), ((((((8.0 * locals.var_spsub_temp__blk863_dn9) - (12.0 * locals.var_spsub_xi0__blk878_dn9)) * locals.var_spsub_temp__blk863) + (assign29630_e32285 * locals.var_spsub_temp__blk863_dn9)) * locals.var_spsub_temp__blk863) + (assign29630_e32287 * locals.var_spsub_temp__blk863_dn9)),)
    } else {
        (locals.var_spsub_xi2__blk880, locals.var_spsub_xi2__blk880_dn4, locals.var_spsub_xi2__blk880_dn6, locals.var_spsub_xi2__blk880_dn7, locals.var_spsub_xi2__blk880_dn8, locals.var_spsub_xi2__blk880_dn9,)
    }
};
        locals.var_spsub_xi2__blk880 = assign29630_e32291;
        locals.var_spsub_xi2__blk880_dn4 = assign29630_e32291_d_n4;
        locals.var_spsub_xi2__blk880_dn6 = assign29630_e32291_d_n6;
        locals.var_spsub_xi2__blk880_dn7 = assign29630_e32291_d_n7;
        locals.var_spsub_xi2__blk880_dn8 = assign29630_e32291_d_n8;
        locals.var_spsub_xi2__blk880_dn9 = assign29630_e32291_d_n9;

        let (assign29640_e32305, assign29640_e32305_d_n4, assign29640_e32305_d_n6, assign29640_e32305_d_n7, assign29640_e32305_d_n8, assign29640_e32305_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29640_e32303: f64 = (locals.var_spsub_xgb__blk866 - locals.var_spsub_x0__blk889);
        (assign29640_e32303, (locals.var_spsub_xgb__blk866_dn4 - locals.var_spsub_x0__blk889_dn4), (locals.var_spsub_xgb__blk866_dn6 - locals.var_spsub_x0__blk889_dn6), (locals.var_spsub_xgb__blk866_dn7 - locals.var_spsub_x0__blk889_dn7), (locals.var_spsub_xgb__blk866_dn8 - locals.var_spsub_x0__blk889_dn8), (locals.var_spsub_xgb__blk866_dn9 - locals.var_spsub_x0__blk889_dn9),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29640_e32305;
        locals.var_spsub_temp__blk863_dn4 = assign29640_e32305_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29640_e32305_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29640_e32305_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29640_e32305_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29640_e32305_d_n9;

        let (assign29650_e32333, assign29650_e32333_d_n4, assign29650_e32333_d_n6, assign29650_e32333_d_n7, assign29650_e32333_d_n8, assign29650_e32333_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29650_e32317: f64 = (2.0 * locals.var_spsub_temp__blk863);
        let assign29650_e32321: f64 = (1.0 - locals.var_spsub_delta1__blk877);
        let assign29650_e32323: f64 = (assign29650_e32321 + locals.var_spsub_delta0__blk876);
        let assign29650_e32327: f64 = (1.0 + locals.var_spsub_xi1__blk879);
        let assign29650_e32328: f64 = (locals.var_spsub_delta__blk867 * assign29650_e32327);
        let assign29650_e32329: f64 = (assign29650_e32323 - assign29650_e32328);
        let assign29650_e32330: f64 = (locals.var_gfsub2 * assign29650_e32329);
        let assign29650_e32331: f64 = (assign29650_e32317 + assign29650_e32330);
        (assign29650_e32331, ((2.0 * locals.var_spsub_temp__blk863_dn4) + ((locals.var_gfsub2_dn4 * assign29650_e32329) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1__blk877_dn4) + locals.var_spsub_delta0__blk876_dn4) - ((locals.var_spsub_delta__blk867_dn4 * assign29650_e32327) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn4)))))), ((2.0 * locals.var_spsub_temp__blk863_dn6) + ((locals.var_gfsub2_dn6 * assign29650_e32329) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1__blk877_dn6) + locals.var_spsub_delta0__blk876_dn6) - ((locals.var_spsub_delta__blk867_dn6 * assign29650_e32327) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn6)))))), ((2.0 * locals.var_spsub_temp__blk863_dn7) + ((locals.var_gfsub2_dn7 * assign29650_e32329) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1__blk877_dn7) + locals.var_spsub_delta0__blk876_dn7) - ((locals.var_spsub_delta__blk867_dn7 * assign29650_e32327) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn7)))))), ((2.0 * locals.var_spsub_temp__blk863_dn8) + ((locals.var_gfsub2_dn8 * assign29650_e32329) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1__blk877_dn8) + locals.var_spsub_delta0__blk876_dn8) - ((locals.var_spsub_delta__blk867_dn8 * assign29650_e32327) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn8)))))), ((2.0 * locals.var_spsub_temp__blk863_dn9) + ((locals.var_gfsub2_dn9 * assign29650_e32329) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1__blk877_dn9) + locals.var_spsub_delta0__blk876_dn9) - ((locals.var_spsub_delta__blk867_dn9 * assign29650_e32327) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi1__blk879_dn9)))))),)
    } else {
        (locals.var_spsub_pc__blk881, locals.var_spsub_pc__blk881_dn4, locals.var_spsub_pc__blk881_dn6, locals.var_spsub_pc__blk881_dn7, locals.var_spsub_pc__blk881_dn8, locals.var_spsub_pc__blk881_dn9,)
    }
};
        locals.var_spsub_pc__blk881 = assign29650_e32333;
        locals.var_spsub_pc__blk881_dn4 = assign29650_e32333_d_n4;
        locals.var_spsub_pc__blk881_dn6 = assign29650_e32333_d_n6;
        locals.var_spsub_pc__blk881_dn7 = assign29650_e32333_d_n7;
        locals.var_spsub_pc__blk881_dn8 = assign29650_e32333_d_n8;
        locals.var_spsub_pc__blk881_dn9 = assign29650_e32333_d_n9;

        let (assign29660_e32365, assign29660_e32365_d_n4, assign29660_e32365_d_n6, assign29660_e32365_d_n7, assign29660_e32365_d_n8, assign29660_e32365_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29660_e32345: f64 = (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863);
        let assign29660_e32349: f64 = (locals.var_spsub_delta1__blk877 + locals.var_spsub_x0__blk889);
        let assign29660_e32351: f64 = (assign29660_e32349 - 1.0);
        let assign29660_e32353: f64 = (assign29660_e32351 + locals.var_spsub_delta0__blk876);
        let assign29660_e32357: f64 = (locals.var_spsub_x0__blk889 + 1.0);
        let assign29660_e32359: f64 = (assign29660_e32357 + locals.var_spsub_xi0__blk878);
        let assign29660_e32360: f64 = (locals.var_spsub_delta__blk867 * assign29660_e32359);
        let assign29660_e32361: f64 = (assign29660_e32353 - assign29660_e32360);
        let assign29660_e32362: f64 = (locals.var_gfsub2 * assign29660_e32361);
        let assign29660_e32363: f64 = (assign29660_e32345 - assign29660_e32362);
        (assign29660_e32363, (((locals.var_spsub_temp__blk863_dn4 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn4)) - ((locals.var_gfsub2_dn4 * assign29660_e32361) + (locals.var_gfsub2 * (((locals.var_spsub_delta1__blk877_dn4 + locals.var_spsub_x0__blk889_dn4) + locals.var_spsub_delta0__blk876_dn4) - ((locals.var_spsub_delta__blk867_dn4 * assign29660_e32359) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_x0__blk889_dn4 + locals.var_spsub_xi0__blk878_dn4))))))), (((locals.var_spsub_temp__blk863_dn6 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn6)) - ((locals.var_gfsub2_dn6 * assign29660_e32361) + (locals.var_gfsub2 * (((locals.var_spsub_delta1__blk877_dn6 + locals.var_spsub_x0__blk889_dn6) + locals.var_spsub_delta0__blk876_dn6) - ((locals.var_spsub_delta__blk867_dn6 * assign29660_e32359) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_x0__blk889_dn6 + locals.var_spsub_xi0__blk878_dn6))))))), (((locals.var_spsub_temp__blk863_dn7 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn7)) - ((locals.var_gfsub2_dn7 * assign29660_e32361) + (locals.var_gfsub2 * (((locals.var_spsub_delta1__blk877_dn7 + locals.var_spsub_x0__blk889_dn7) + locals.var_spsub_delta0__blk876_dn7) - ((locals.var_spsub_delta__blk867_dn7 * assign29660_e32359) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_x0__blk889_dn7 + locals.var_spsub_xi0__blk878_dn7))))))), (((locals.var_spsub_temp__blk863_dn8 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn8)) - ((locals.var_gfsub2_dn8 * assign29660_e32361) + (locals.var_gfsub2 * (((locals.var_spsub_delta1__blk877_dn8 + locals.var_spsub_x0__blk889_dn8) + locals.var_spsub_delta0__blk876_dn8) - ((locals.var_spsub_delta__blk867_dn8 * assign29660_e32359) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_x0__blk889_dn8 + locals.var_spsub_xi0__blk878_dn8))))))), (((locals.var_spsub_temp__blk863_dn9 * locals.var_spsub_temp__blk863) + (locals.var_spsub_temp__blk863 * locals.var_spsub_temp__blk863_dn9)) - ((locals.var_gfsub2_dn9 * assign29660_e32361) + (locals.var_gfsub2 * (((locals.var_spsub_delta1__blk877_dn9 + locals.var_spsub_x0__blk889_dn9) + locals.var_spsub_delta0__blk876_dn9) - ((locals.var_spsub_delta__blk867_dn9 * assign29660_e32359) + (locals.var_spsub_delta__blk867 * (locals.var_spsub_x0__blk889_dn9 + locals.var_spsub_xi0__blk878_dn9))))))),)
    } else {
        (locals.var_spsub_qc__blk882, locals.var_spsub_qc__blk882_dn4, locals.var_spsub_qc__blk882_dn6, locals.var_spsub_qc__blk882_dn7, locals.var_spsub_qc__blk882_dn8, locals.var_spsub_qc__blk882_dn9,)
    }
};
        locals.var_spsub_qc__blk882 = assign29660_e32365;
        locals.var_spsub_qc__blk882_dn4 = assign29660_e32365_d_n4;
        locals.var_spsub_qc__blk882_dn6 = assign29660_e32365_d_n6;
        locals.var_spsub_qc__blk882_dn7 = assign29660_e32365_d_n7;
        locals.var_spsub_qc__blk882_dn8 = assign29660_e32365_d_n8;
        locals.var_spsub_qc__blk882_dn9 = assign29660_e32365_d_n9;

        let (assign29670_e32387, assign29670_e32387_d_n4, assign29670_e32387_d_n6, assign29670_e32387_d_n7, assign29670_e32387_d_n8, assign29670_e32387_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29670_e32379: f64 = (locals.var_spsub_delta1__blk877 + locals.var_spsub_delta0__blk876);
        let assign29670_e32382: f64 = (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880);
        let assign29670_e32383: f64 = (assign29670_e32379 - assign29670_e32382);
        let assign29670_e32384: f64 = (locals.var_gfsub2 * assign29670_e32383);
        let assign29670_e32385: f64 = (2.0 - assign29670_e32384);
        (assign29670_e32385, (-((locals.var_gfsub2_dn4 * assign29670_e32383) + (locals.var_gfsub2 * ((locals.var_spsub_delta1__blk877_dn4 + locals.var_spsub_delta0__blk876_dn4) - ((locals.var_spsub_delta__blk867_dn4 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn4)))))), (-((locals.var_gfsub2_dn6 * assign29670_e32383) + (locals.var_gfsub2 * ((locals.var_spsub_delta1__blk877_dn6 + locals.var_spsub_delta0__blk876_dn6) - ((locals.var_spsub_delta__blk867_dn6 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn6)))))), (-((locals.var_gfsub2_dn7 * assign29670_e32383) + (locals.var_gfsub2 * ((locals.var_spsub_delta1__blk877_dn7 + locals.var_spsub_delta0__blk876_dn7) - ((locals.var_spsub_delta__blk867_dn7 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn7)))))), (-((locals.var_gfsub2_dn8 * assign29670_e32383) + (locals.var_gfsub2 * ((locals.var_spsub_delta1__blk877_dn8 + locals.var_spsub_delta0__blk876_dn8) - ((locals.var_spsub_delta__blk867_dn8 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn8)))))), (-((locals.var_gfsub2_dn9 * assign29670_e32383) + (locals.var_gfsub2 * ((locals.var_spsub_delta1__blk877_dn9 + locals.var_spsub_delta0__blk876_dn9) - ((locals.var_spsub_delta__blk867_dn9 * locals.var_spsub_xi2__blk880) + (locals.var_spsub_delta__blk867 * locals.var_spsub_xi2__blk880_dn9)))))),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29670_e32387;
        locals.var_spsub_temp__blk863_dn4 = assign29670_e32387_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29670_e32387_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29670_e32387_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29670_e32387_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29670_e32387_d_n9;

        let (assign29680_e32407, assign29680_e32407_d_n4, assign29680_e32407_d_n6, assign29680_e32407_d_n7, assign29680_e32407_d_n8, assign29680_e32407_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29680_e32399: f64 = (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881);
        let assign29680_e32403: f64 = (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863);
        let assign29680_e32404: f64 = (2.0 * assign29680_e32403);
        let assign29680_e32405: f64 = (assign29680_e32399 - assign29680_e32404);
        (assign29680_e32405, (((locals.var_spsub_pc__blk881_dn4 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn4)) - (2.0 * ((locals.var_spsub_qc__blk882_dn4 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn4)))), (((locals.var_spsub_pc__blk881_dn6 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn6)) - (2.0 * ((locals.var_spsub_qc__blk882_dn6 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn6)))), (((locals.var_spsub_pc__blk881_dn7 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn7)) - (2.0 * ((locals.var_spsub_qc__blk882_dn7 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn7)))), (((locals.var_spsub_pc__blk881_dn8 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn8)) - (2.0 * ((locals.var_spsub_qc__blk882_dn8 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn8)))), (((locals.var_spsub_pc__blk881_dn9 * locals.var_spsub_pc__blk881) + (locals.var_spsub_pc__blk881 * locals.var_spsub_pc__blk881_dn9)) - (2.0 * ((locals.var_spsub_qc__blk882_dn9 * locals.var_spsub_temp__blk863) + (locals.var_spsub_qc__blk882 * locals.var_spsub_temp__blk863_dn9)))),)
    } else {
        (locals.var_spsub_temp__blk863, locals.var_spsub_temp__blk863_dn4, locals.var_spsub_temp__blk863_dn6, locals.var_spsub_temp__blk863_dn7, locals.var_spsub_temp__blk863_dn8, locals.var_spsub_temp__blk863_dn9,)
    }
};
        locals.var_spsub_temp__blk863 = assign29680_e32407;
        locals.var_spsub_temp__blk863_dn4 = assign29680_e32407_d_n4;
        locals.var_spsub_temp__blk863_dn6 = assign29680_e32407_d_n6;
        locals.var_spsub_temp__blk863_dn7 = assign29680_e32407_d_n7;
        locals.var_spsub_temp__blk863_dn8 = assign29680_e32407_d_n8;
        locals.var_spsub_temp__blk863_dn9 = assign29680_e32407_d_n9;

        let (assign29690_e32428, assign29690_e32428_d_n4, assign29690_e32428_d_n6, assign29690_e32428_d_n7, assign29690_e32428_d_n8, assign29690_e32428_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1087 == 0.0)) && (locals.var_guard1088 == 0.0)) {
        let assign29690_e32422: f64 = (locals.var_spsub_temp__blk863).sqrt();
        let assign29690_e32423: f64 = (locals.var_spsub_pc__blk881 + assign29690_e32422);
        let assign29690_e32424: f64 = (locals.var_spsub_qc__blk882 / assign29690_e32423);
        let assign29690_e32425: f64 = (2.0 * assign29690_e32424);
        let assign29690_e32426: f64 = (locals.var_spsub_x0__blk889 + assign29690_e32425);
        (assign29690_e32426, (locals.var_spsub_x0__blk889_dn4 + (2.0 * (((locals.var_spsub_qc__blk882_dn4 * assign29690_e32423) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn4 + (locals.var_spsub_temp__blk863_dn4 / (2.0 * assign29690_e32422))))) / (assign29690_e32423 * assign29690_e32423)))), (locals.var_spsub_x0__blk889_dn6 + (2.0 * (((locals.var_spsub_qc__blk882_dn6 * assign29690_e32423) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn6 + (locals.var_spsub_temp__blk863_dn6 / (2.0 * assign29690_e32422))))) / (assign29690_e32423 * assign29690_e32423)))), (locals.var_spsub_x0__blk889_dn7 + (2.0 * (((locals.var_spsub_qc__blk882_dn7 * assign29690_e32423) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn7 + (locals.var_spsub_temp__blk863_dn7 / (2.0 * assign29690_e32422))))) / (assign29690_e32423 * assign29690_e32423)))), (locals.var_spsub_x0__blk889_dn8 + (2.0 * (((locals.var_spsub_qc__blk882_dn8 * assign29690_e32423) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn8 + (locals.var_spsub_temp__blk863_dn8 / (2.0 * assign29690_e32422))))) / (assign29690_e32423 * assign29690_e32423)))), (locals.var_spsub_x0__blk889_dn9 + (2.0 * (((locals.var_spsub_qc__blk882_dn9 * assign29690_e32423) - (locals.var_spsub_qc__blk882 * (locals.var_spsub_pc__blk881_dn9 + (locals.var_spsub_temp__blk863_dn9 / (2.0 * assign29690_e32422))))) / (assign29690_e32423 * assign29690_e32423)))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign29690_e32428;
        locals.var_temp3_dn4 = assign29690_e32428_d_n4;
        locals.var_temp3_dn6 = assign29690_e32428_d_n6;
        locals.var_temp3_dn7 = assign29690_e32428_d_n7;
        locals.var_temp3_dn8 = assign29690_e32428_d_n8;
        locals.var_temp3_dn9 = assign29690_e32428_d_n9;

        let (assign29700_e32438, assign29700_e32438_d_n4, assign29700_e32438_d_n6, assign29700_e32438_d_n7, assign29700_e32438_d_n8, assign29700_e32438_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 != 0.0)) {
        let assign29700_e32435: f64 = (locals.var_temp3 + locals.var_temp2);
        let assign29700_e32436: f64 = (locals.var_temp * assign29700_e32435);
        (assign29700_e32436, ((locals.var_temp_dn4 * assign29700_e32435) + (locals.var_temp * (locals.var_temp3_dn4 + locals.var_temp2_dn4))), ((locals.var_temp_dn6 * assign29700_e32435) + (locals.var_temp * (locals.var_temp3_dn6 + locals.var_temp2_dn6))), ((locals.var_temp_dn7 * assign29700_e32435) + (locals.var_temp * (locals.var_temp3_dn7 + locals.var_temp2_dn7))), ((locals.var_temp_dn8 * assign29700_e32435) + (locals.var_temp * (locals.var_temp3_dn8 + locals.var_temp2_dn8))), ((locals.var_temp_dn9 * assign29700_e32435) + (locals.var_temp * (locals.var_temp3_dn9 + locals.var_temp2_dn9))),)
    } else {
        (locals.var_xg2eff__blk910, locals.var_xg2eff__blk910_dn4, locals.var_xg2eff__blk910_dn6, locals.var_xg2eff__blk910_dn7, locals.var_xg2eff__blk910_dn8, locals.var_xg2eff__blk910_dn9,)
    }
};
        locals.var_xg2eff__blk910 = assign29700_e32438;
        locals.var_xg2eff__blk910_dn4 = assign29700_e32438_d_n4;
        locals.var_xg2eff__blk910_dn6 = assign29700_e32438_d_n6;
        locals.var_xg2eff__blk910_dn7 = assign29700_e32438_d_n7;
        locals.var_xg2eff__blk910_dn8 = assign29700_e32438_d_n8;
        locals.var_xg2eff__blk910_dn9 = assign29700_e32438_d_n9;

        let (assign29710_e32445, assign29710_e32445_d_n4, assign29710_e32445_d_n6, assign29710_e32445_d_n7, assign29710_e32445_d_n8, assign29710_e32445_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1081 == 0.0)) {
        (locals.var_xg20__blk901, locals.var_xg20__blk901_dn4, locals.var_xg20__blk901_dn6, locals.var_xg20__blk901_dn7, locals.var_xg20__blk901_dn8, locals.var_xg20__blk901_dn9,)
    } else {
        (locals.var_xg2eff__blk910, locals.var_xg2eff__blk910_dn4, locals.var_xg2eff__blk910_dn6, locals.var_xg2eff__blk910_dn7, locals.var_xg2eff__blk910_dn8, locals.var_xg2eff__blk910_dn9,)
    }
};
        locals.var_xg2eff__blk910 = assign29710_e32445;
        locals.var_xg2eff__blk910_dn4 = assign29710_e32445_d_n4;
        locals.var_xg2eff__blk910_dn6 = assign29710_e32445_d_n6;
        locals.var_xg2eff__blk910_dn7 = assign29710_e32445_d_n7;
        locals.var_xg2eff__blk910_dn8 = assign29710_e32445_d_n8;
        locals.var_xg2eff__blk910_dn9 = assign29710_e32445_d_n9;

        let (assign29720_e32453, assign29720_e32453_d_n4, assign29720_e32453_d_n6, assign29720_e32453_d_n7, assign29720_e32453_d_n8, assign29720_e32453_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign29720_e32450: f64 = (locals.var_xg10__blk899 - locals.var_xg2eff__blk910);
        let assign29720_e32451: f64 = (locals.var_keq_1d * assign29720_e32450);
        (assign29720_e32451, (locals.var_keq_1d * (locals.var_xg10__blk899_dn4 - locals.var_xg2eff__blk910_dn4)), (locals.var_keq_1d * (locals.var_xg10__blk899_dn6 - locals.var_xg2eff__blk910_dn6)), (locals.var_keq_1d * (locals.var_xg10__blk899_dn7 - locals.var_xg2eff__blk910_dn7)), (locals.var_keq_1d * (locals.var_xg10__blk899_dn8 - locals.var_xg2eff__blk910_dn8)), (locals.var_keq_1d * (locals.var_xg10__blk899_dn9 - locals.var_xg2eff__blk910_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign29720_e32453;
        locals.var_temp_dn4 = assign29720_e32453_d_n4;
        locals.var_temp_dn6 = assign29720_e32453_d_n6;
        locals.var_temp_dn7 = assign29720_e32453_d_n7;
        locals.var_temp_dn8 = assign29720_e32453_d_n8;
        locals.var_temp_dn9 = assign29720_e32453_d_n9;

        let assign29730_e32456: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1093 = assign29730_e32456;

        let (assign29740_e32479, assign29740_e32479_d_n4, assign29740_e32479_d_n6, assign29740_e32479_d_n7, assign29740_e32479_d_n8, assign29740_e32479_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29740_e32463: f64 = (locals.var_temp + locals.var_emin);
        let assign29740_e32466: f64 = (locals.var_temp - locals.var_emin);
        let assign29740_e32469: f64 = (locals.var_temp - locals.var_emin);
        let assign29740_e32470: f64 = (assign29740_e32466 * assign29740_e32469);
        let assign29740_e32473: f64 = (locals.var_emin * locals.var_emin);
        let assign29740_e32474: f64 = (assign29740_e32470 + assign29740_e32473);
        let assign29740_e32475: f64 = (assign29740_e32474).sqrt();
        let assign29740_e32476: f64 = (assign29740_e32463 + assign29740_e32475);
        let assign29740_e32477: f64 = (0.5 * assign29740_e32476);
        (assign29740_e32477, (0.5 * ((locals.var_temp_dn4 + locals.var_emin_dn4) + (((((locals.var_temp_dn4 - locals.var_emin_dn4) * assign29740_e32469) + (assign29740_e32466 * (locals.var_temp_dn4 - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign29740_e32475)))), (0.5 * ((locals.var_temp_dn6 + locals.var_emin_dn6) + (((((locals.var_temp_dn6 - locals.var_emin_dn6) * assign29740_e32469) + (assign29740_e32466 * (locals.var_temp_dn6 - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign29740_e32475)))), (0.5 * ((locals.var_temp_dn7 + locals.var_emin_dn7) + (((((locals.var_temp_dn7 - locals.var_emin_dn7) * assign29740_e32469) + (assign29740_e32466 * (locals.var_temp_dn7 - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign29740_e32475)))), (0.5 * ((locals.var_temp_dn8 + locals.var_emin_dn8) + (((((locals.var_temp_dn8 - locals.var_emin_dn8) * assign29740_e32469) + (assign29740_e32466 * (locals.var_temp_dn8 - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign29740_e32475)))), (0.5 * ((locals.var_temp_dn9 + locals.var_emin_dn9) + (((((locals.var_temp_dn9 - locals.var_emin_dn9) * assign29740_e32469) + (assign29740_e32466 * (locals.var_temp_dn9 - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign29740_e32475)))),)
    } else {
        (locals.var_e1__blk911, locals.var_e1__blk911_dn4, locals.var_e1__blk911_dn6, locals.var_e1__blk911_dn7, locals.var_e1__blk911_dn8, locals.var_e1__blk911_dn9,)
    }
};
        locals.var_e1__blk911 = assign29740_e32479;
        locals.var_e1__blk911_dn4 = assign29740_e32479_d_n4;
        locals.var_e1__blk911_dn6 = assign29740_e32479_d_n6;
        locals.var_e1__blk911_dn7 = assign29740_e32479_d_n7;
        locals.var_e1__blk911_dn8 = assign29740_e32479_d_n8;
        locals.var_e1__blk911_dn9 = assign29740_e32479_d_n9;

        let (assign29750_e32505, assign29750_e32505_d_n4, assign29750_e32505_d_n6, assign29750_e32505_d_n7, assign29750_e32505_d_n8, assign29750_e32505_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29750_e32485: f64 = (-locals.var_temp);
        let assign29750_e32487: f64 = (assign29750_e32485 + locals.var_emin);
        let assign29750_e32489: f64 = (-locals.var_temp);
        let assign29750_e32491: f64 = (assign29750_e32489 - locals.var_emin);
        let assign29750_e32493: f64 = (-locals.var_temp);
        let assign29750_e32495: f64 = (assign29750_e32493 - locals.var_emin);
        let assign29750_e32496: f64 = (assign29750_e32491 * assign29750_e32495);
        let assign29750_e32499: f64 = (locals.var_emin * locals.var_emin);
        let assign29750_e32500: f64 = (assign29750_e32496 + assign29750_e32499);
        let assign29750_e32501: f64 = (assign29750_e32500).sqrt();
        let assign29750_e32502: f64 = (assign29750_e32487 + assign29750_e32501);
        let assign29750_e32503: f64 = (0.5 * assign29750_e32502);
        (assign29750_e32503, (0.5 * (((-locals.var_temp_dn4) + locals.var_emin_dn4) + ((((((-locals.var_temp_dn4) - locals.var_emin_dn4) * assign29750_e32495) + (assign29750_e32491 * ((-locals.var_temp_dn4) - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign29750_e32501)))), (0.5 * (((-locals.var_temp_dn6) + locals.var_emin_dn6) + ((((((-locals.var_temp_dn6) - locals.var_emin_dn6) * assign29750_e32495) + (assign29750_e32491 * ((-locals.var_temp_dn6) - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign29750_e32501)))), (0.5 * (((-locals.var_temp_dn7) + locals.var_emin_dn7) + ((((((-locals.var_temp_dn7) - locals.var_emin_dn7) * assign29750_e32495) + (assign29750_e32491 * ((-locals.var_temp_dn7) - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign29750_e32501)))), (0.5 * (((-locals.var_temp_dn8) + locals.var_emin_dn8) + ((((((-locals.var_temp_dn8) - locals.var_emin_dn8) * assign29750_e32495) + (assign29750_e32491 * ((-locals.var_temp_dn8) - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign29750_e32501)))), (0.5 * (((-locals.var_temp_dn9) + locals.var_emin_dn9) + ((((((-locals.var_temp_dn9) - locals.var_emin_dn9) * assign29750_e32495) + (assign29750_e32491 * ((-locals.var_temp_dn9) - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign29750_e32501)))),)
    } else {
        (locals.var_e2__blk912, locals.var_e2__blk912_dn4, locals.var_e2__blk912_dn6, locals.var_e2__blk912_dn7, locals.var_e2__blk912_dn8, locals.var_e2__blk912_dn9,)
    }
};
        locals.var_e2__blk912 = assign29750_e32505;
        locals.var_e2__blk912_dn4 = assign29750_e32505_d_n4;
        locals.var_e2__blk912_dn6 = assign29750_e32505_d_n6;
        locals.var_e2__blk912_dn7 = assign29750_e32505_d_n7;
        locals.var_e2__blk912_dn8 = assign29750_e32505_d_n8;
        locals.var_e2__blk912_dn9 = assign29750_e32505_d_n9;

        let (assign29760_e32518, assign29760_e32518_d_n4, assign29760_e32518_d_n6, assign29760_e32518_d_n7, assign29760_e32518_d_n8, assign29760_e32518_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29760_e32511: f64 = (-0.3333333333333);
        let assign29760_e32513: f64 = (locals.var_e1__blk911).ln();
        let assign29760_e32514: f64 = (assign29760_e32511 * assign29760_e32513);
        let assign29760_e32515: f64 = (assign29760_e32514).exp();
        let assign29760_e32516: f64 = (locals.var_qq * assign29760_e32515);
        (assign29760_e32516, ((locals.var_qq_dn4 * assign29760_e32515) + (locals.var_qq * (assign29760_e32515 * (assign29760_e32511 * (locals.var_e1__blk911_dn4 / locals.var_e1__blk911))))), ((locals.var_qq_dn6 * assign29760_e32515) + (locals.var_qq * (assign29760_e32515 * (assign29760_e32511 * (locals.var_e1__blk911_dn6 / locals.var_e1__blk911))))), ((locals.var_qq_dn7 * assign29760_e32515) + (locals.var_qq * (assign29760_e32515 * (assign29760_e32511 * (locals.var_e1__blk911_dn7 / locals.var_e1__blk911))))), ((locals.var_qq_dn8 * assign29760_e32515) + (locals.var_qq * (assign29760_e32515 * (assign29760_e32511 * (locals.var_e1__blk911_dn8 / locals.var_e1__blk911))))), ((locals.var_qq_dn9 * assign29760_e32515) + (locals.var_qq * (assign29760_e32515 * (assign29760_e32511 * (locals.var_e1__blk911_dn9 / locals.var_e1__blk911))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign29760_e32518;
        locals.var_temp1_dn4 = assign29760_e32518_d_n4;
        locals.var_temp1_dn6 = assign29760_e32518_d_n6;
        locals.var_temp1_dn7 = assign29760_e32518_d_n7;
        locals.var_temp1_dn8 = assign29760_e32518_d_n8;
        locals.var_temp1_dn9 = assign29760_e32518_d_n9;

        let (assign29770_e32531, assign29770_e32531_d_n4, assign29770_e32531_d_n6, assign29770_e32531_d_n7, assign29770_e32531_d_n8, assign29770_e32531_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29770_e32524: f64 = (-0.3333333333333);
        let assign29770_e32526: f64 = (locals.var_e2__blk912).ln();
        let assign29770_e32527: f64 = (assign29770_e32524 * assign29770_e32526);
        let assign29770_e32528: f64 = (assign29770_e32527).exp();
        let assign29770_e32529: f64 = (locals.var_qq * assign29770_e32528);
        (assign29770_e32529, ((locals.var_qq_dn4 * assign29770_e32528) + (locals.var_qq * (assign29770_e32528 * (assign29770_e32524 * (locals.var_e2__blk912_dn4 / locals.var_e2__blk912))))), ((locals.var_qq_dn6 * assign29770_e32528) + (locals.var_qq * (assign29770_e32528 * (assign29770_e32524 * (locals.var_e2__blk912_dn6 / locals.var_e2__blk912))))), ((locals.var_qq_dn7 * assign29770_e32528) + (locals.var_qq * (assign29770_e32528 * (assign29770_e32524 * (locals.var_e2__blk912_dn7 / locals.var_e2__blk912))))), ((locals.var_qq_dn8 * assign29770_e32528) + (locals.var_qq * (assign29770_e32528 * (assign29770_e32524 * (locals.var_e2__blk912_dn8 / locals.var_e2__blk912))))), ((locals.var_qq_dn9 * assign29770_e32528) + (locals.var_qq * (assign29770_e32528 * (assign29770_e32524 * (locals.var_e2__blk912_dn9 / locals.var_e2__blk912))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign29770_e32531;
        locals.var_temp2_dn4 = assign29770_e32531_d_n4;
        locals.var_temp2_dn6 = assign29770_e32531_d_n6;
        locals.var_temp2_dn7 = assign29770_e32531_d_n7;
        locals.var_temp2_dn8 = assign29770_e32531_d_n8;
        locals.var_temp2_dn9 = assign29770_e32531_d_n9;

        let (assign29780_e32541, assign29780_e32541_d_n4, assign29780_e32541_d_n6, assign29780_e32541_d_n7, assign29780_e32541_d_n8, assign29780_e32541_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29780_e32537: f64 = (1.0 - locals.var_temp1);
        let assign29780_e32539: f64 = (assign29780_e32537 - locals.var_temp2);
        (assign29780_e32539, ((-locals.var_temp1_dn4) - locals.var_temp2_dn4), ((-locals.var_temp1_dn6) - locals.var_temp2_dn6), ((-locals.var_temp1_dn7) - locals.var_temp2_dn7), ((-locals.var_temp1_dn8) - locals.var_temp2_dn8), ((-locals.var_temp1_dn9) - locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign29780_e32541;
        locals.var_temp3_dn4 = assign29780_e32541_d_n4;
        locals.var_temp3_dn6 = assign29780_e32541_d_n6;
        locals.var_temp3_dn7 = assign29780_e32541_d_n7;
        locals.var_temp3_dn8 = assign29780_e32541_d_n8;
        locals.var_temp3_dn9 = assign29780_e32541_d_n9;

        let (assign29790_e32549, assign29790_e32549_d_n4, assign29790_e32549_d_n6, assign29790_e32549_d_n7, assign29790_e32549_d_n8, assign29790_e32549_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29790_e32547: f64 = (locals.var_csiprime_0 / locals.var_temp3);
        (assign29790_e32547, (-((locals.var_csiprime_0 * locals.var_temp3_dn4) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn6) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn7) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn8) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn9) / (locals.var_temp3 * locals.var_temp3))),)
    } else {
        (locals.var_csiprime__blk919, locals.var_csiprime__blk919_dn4, locals.var_csiprime__blk919_dn6, locals.var_csiprime__blk919_dn7, locals.var_csiprime__blk919_dn8, locals.var_csiprime__blk919_dn9,)
    }
};
        locals.var_csiprime__blk919 = assign29790_e32549;
        locals.var_csiprime__blk919_dn4 = assign29790_e32549_d_n4;
        locals.var_csiprime__blk919_dn6 = assign29790_e32549_d_n6;
        locals.var_csiprime__blk919_dn7 = assign29790_e32549_d_n7;
        locals.var_csiprime__blk919_dn8 = assign29790_e32549_d_n8;
        locals.var_csiprime__blk919_dn9 = assign29790_e32549_d_n9;

        let (assign29800_e32559, assign29800_e32559_d_n4, assign29800_e32559_d_n6, assign29800_e32559_d_n7, assign29800_e32559_d_n8, assign29800_e32559_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29800_e32556: f64 = (locals.var_k1_1d * locals.var_temp1);
        let assign29800_e32557: f64 = (1.0 + assign29800_e32556);
        (assign29800_e32557, (locals.var_k1_1d * locals.var_temp1_dn4), (locals.var_k1_1d * locals.var_temp1_dn6), (locals.var_k1_1d * locals.var_temp1_dn7), (locals.var_k1_1d * locals.var_temp1_dn8), (locals.var_k1_1d * locals.var_temp1_dn9),)
    } else {
        (locals.var_tox1fact__blk913, locals.var_tox1fact__blk913_dn4, locals.var_tox1fact__blk913_dn6, locals.var_tox1fact__blk913_dn7, locals.var_tox1fact__blk913_dn8, locals.var_tox1fact__blk913_dn9,)
    }
};
        locals.var_tox1fact__blk913 = assign29800_e32559;
        locals.var_tox1fact__blk913_dn4 = assign29800_e32559_d_n4;
        locals.var_tox1fact__blk913_dn6 = assign29800_e32559_d_n6;
        locals.var_tox1fact__blk913_dn7 = assign29800_e32559_d_n7;
        locals.var_tox1fact__blk913_dn8 = assign29800_e32559_d_n8;
        locals.var_tox1fact__blk913_dn9 = assign29800_e32559_d_n9;

        let (assign29810_e32569, assign29810_e32569_d_n4, assign29810_e32569_d_n6, assign29810_e32569_d_n7, assign29810_e32569_d_n8, assign29810_e32569_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29810_e32566: f64 = (locals.var_k2_1d * locals.var_temp2);
        let assign29810_e32567: f64 = (1.0 + assign29810_e32566);
        (assign29810_e32567, (locals.var_k2_1d * locals.var_temp2_dn4), (locals.var_k2_1d * locals.var_temp2_dn6), (locals.var_k2_1d * locals.var_temp2_dn7), (locals.var_k2_1d * locals.var_temp2_dn8), (locals.var_k2_1d * locals.var_temp2_dn9),)
    } else {
        (locals.var_tox2fact__blk914, locals.var_tox2fact__blk914_dn4, locals.var_tox2fact__blk914_dn6, locals.var_tox2fact__blk914_dn7, locals.var_tox2fact__blk914_dn8, locals.var_tox2fact__blk914_dn9,)
    }
};
        locals.var_tox2fact__blk914 = assign29810_e32569;
        locals.var_tox2fact__blk914_dn4 = assign29810_e32569_d_n4;
        locals.var_tox2fact__blk914_dn6 = assign29810_e32569_d_n6;
        locals.var_tox2fact__blk914_dn7 = assign29810_e32569_d_n7;
        locals.var_tox2fact__blk914_dn8 = assign29810_e32569_d_n8;
        locals.var_tox2fact__blk914_dn9 = assign29810_e32569_d_n9;

        let (assign29820_e32579, assign29820_e32579_d_n4, assign29820_e32579_d_n6, assign29820_e32579_d_n7, assign29820_e32579_d_n8, assign29820_e32579_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29820_e32575: f64 = (locals.var_k1_1d * locals.var_temp3);
        let assign29820_e32577: f64 = (assign29820_e32575 / locals.var_tox1fact__blk913);
        (assign29820_e32577, ((((locals.var_k1_1d * locals.var_temp3_dn4) * locals.var_tox1fact__blk913) - (assign29820_e32575 * locals.var_tox1fact__blk913_dn4)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), ((((locals.var_k1_1d * locals.var_temp3_dn6) * locals.var_tox1fact__blk913) - (assign29820_e32575 * locals.var_tox1fact__blk913_dn6)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), ((((locals.var_k1_1d * locals.var_temp3_dn7) * locals.var_tox1fact__blk913) - (assign29820_e32575 * locals.var_tox1fact__blk913_dn7)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), ((((locals.var_k1_1d * locals.var_temp3_dn8) * locals.var_tox1fact__blk913) - (assign29820_e32575 * locals.var_tox1fact__blk913_dn8)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), ((((locals.var_k1_1d * locals.var_temp3_dn9) * locals.var_tox1fact__blk913) - (assign29820_e32575 * locals.var_tox1fact__blk913_dn9)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)),)
    } else {
        (locals.var_k1_1d_qm__blk915, locals.var_k1_1d_qm__blk915_dn4, locals.var_k1_1d_qm__blk915_dn6, locals.var_k1_1d_qm__blk915_dn7, locals.var_k1_1d_qm__blk915_dn8, locals.var_k1_1d_qm__blk915_dn9,)
    }
};
        locals.var_k1_1d_qm__blk915 = assign29820_e32579;
        locals.var_k1_1d_qm__blk915_dn4 = assign29820_e32579_d_n4;
        locals.var_k1_1d_qm__blk915_dn6 = assign29820_e32579_d_n6;
        locals.var_k1_1d_qm__blk915_dn7 = assign29820_e32579_d_n7;
        locals.var_k1_1d_qm__blk915_dn8 = assign29820_e32579_d_n8;
        locals.var_k1_1d_qm__blk915_dn9 = assign29820_e32579_d_n9;

        let (assign29830_e32589, assign29830_e32589_d_n4, assign29830_e32589_d_n6, assign29830_e32589_d_n7, assign29830_e32589_d_n8, assign29830_e32589_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29830_e32585: f64 = (locals.var_k2_1d * locals.var_temp3);
        let assign29830_e32587: f64 = (assign29830_e32585 / locals.var_tox2fact__blk914);
        (assign29830_e32587, ((((locals.var_k2_1d * locals.var_temp3_dn4) * locals.var_tox2fact__blk914) - (assign29830_e32585 * locals.var_tox2fact__blk914_dn4)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), ((((locals.var_k2_1d * locals.var_temp3_dn6) * locals.var_tox2fact__blk914) - (assign29830_e32585 * locals.var_tox2fact__blk914_dn6)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), ((((locals.var_k2_1d * locals.var_temp3_dn7) * locals.var_tox2fact__blk914) - (assign29830_e32585 * locals.var_tox2fact__blk914_dn7)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), ((((locals.var_k2_1d * locals.var_temp3_dn8) * locals.var_tox2fact__blk914) - (assign29830_e32585 * locals.var_tox2fact__blk914_dn8)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), ((((locals.var_k2_1d * locals.var_temp3_dn9) * locals.var_tox2fact__blk914) - (assign29830_e32585 * locals.var_tox2fact__blk914_dn9)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)),)
    } else {
        (locals.var_k2_1d_qm__blk916, locals.var_k2_1d_qm__blk916_dn4, locals.var_k2_1d_qm__blk916_dn6, locals.var_k2_1d_qm__blk916_dn7, locals.var_k2_1d_qm__blk916_dn8, locals.var_k2_1d_qm__blk916_dn9,)
    }
};
        locals.var_k2_1d_qm__blk916 = assign29830_e32589;
        locals.var_k2_1d_qm__blk916_dn4 = assign29830_e32589_d_n4;
        locals.var_k2_1d_qm__blk916_dn6 = assign29830_e32589_d_n6;
        locals.var_k2_1d_qm__blk916_dn7 = assign29830_e32589_d_n7;
        locals.var_k2_1d_qm__blk916_dn8 = assign29830_e32589_d_n8;
        locals.var_k2_1d_qm__blk916_dn9 = assign29830_e32589_d_n9;

        let (assign29840_e32605, assign29840_e32605_d_n4, assign29840_e32605_d_n6, assign29840_e32605_d_n7, assign29840_e32605_d_n8, assign29840_e32605_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29840_e32597: f64 = (1.0 / locals.var_k1_1d_qm__blk915);
        let assign29840_e32598: f64 = (1.0 + assign29840_e32597);
        let assign29840_e32601: f64 = (1.0 / locals.var_k2_1d_qm__blk916);
        let assign29840_e32602: f64 = (assign29840_e32598 + assign29840_e32601);
        let assign29840_e32603: f64 = (1.0 / assign29840_e32602);
        (assign29840_e32603, (-(((-(locals.var_k1_1d_qm__blk915_dn4 / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + (-(locals.var_k2_1d_qm__blk916_dn4 / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916)))) / (assign29840_e32602 * assign29840_e32602))), (-(((-(locals.var_k1_1d_qm__blk915_dn6 / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + (-(locals.var_k2_1d_qm__blk916_dn6 / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916)))) / (assign29840_e32602 * assign29840_e32602))), (-(((-(locals.var_k1_1d_qm__blk915_dn7 / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + (-(locals.var_k2_1d_qm__blk916_dn7 / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916)))) / (assign29840_e32602 * assign29840_e32602))), (-(((-(locals.var_k1_1d_qm__blk915_dn8 / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + (-(locals.var_k2_1d_qm__blk916_dn8 / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916)))) / (assign29840_e32602 * assign29840_e32602))), (-(((-(locals.var_k1_1d_qm__blk915_dn9 / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + (-(locals.var_k2_1d_qm__blk916_dn9 / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916)))) / (assign29840_e32602 * assign29840_e32602))),)
    } else {
        (locals.var_keq_1d_qm__blk917, locals.var_keq_1d_qm__blk917_dn4, locals.var_keq_1d_qm__blk917_dn6, locals.var_keq_1d_qm__blk917_dn7, locals.var_keq_1d_qm__blk917_dn8, locals.var_keq_1d_qm__blk917_dn9,)
    }
};
        locals.var_keq_1d_qm__blk917 = assign29840_e32605;
        locals.var_keq_1d_qm__blk917_dn4 = assign29840_e32605_d_n4;
        locals.var_keq_1d_qm__blk917_dn6 = assign29840_e32605_d_n6;
        locals.var_keq_1d_qm__blk917_dn7 = assign29840_e32605_d_n7;
        locals.var_keq_1d_qm__blk917_dn8 = assign29840_e32605_d_n8;
        locals.var_keq_1d_qm__blk917_dn9 = assign29840_e32605_d_n9;

        let (assign29850_e32615, assign29850_e32615_d_n4, assign29850_e32615_d_n6, assign29850_e32615_d_n7, assign29850_e32615_d_n8, assign29850_e32615_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29850_e32612: f64 = (locals.var_k1_1d_qm__blk915 * locals.var_temp1);
        let assign29850_e32613: f64 = (1.0 + assign29850_e32612);
        (assign29850_e32613, ((locals.var_k1_1d_qm__blk915_dn4 * locals.var_temp1) + (locals.var_k1_1d_qm__blk915 * locals.var_temp1_dn4)), ((locals.var_k1_1d_qm__blk915_dn6 * locals.var_temp1) + (locals.var_k1_1d_qm__blk915 * locals.var_temp1_dn6)), ((locals.var_k1_1d_qm__blk915_dn7 * locals.var_temp1) + (locals.var_k1_1d_qm__blk915 * locals.var_temp1_dn7)), ((locals.var_k1_1d_qm__blk915_dn8 * locals.var_temp1) + (locals.var_k1_1d_qm__blk915 * locals.var_temp1_dn8)), ((locals.var_k1_1d_qm__blk915_dn9 * locals.var_temp1) + (locals.var_k1_1d_qm__blk915 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_tox1fact__blk913, locals.var_tox1fact__blk913_dn4, locals.var_tox1fact__blk913_dn6, locals.var_tox1fact__blk913_dn7, locals.var_tox1fact__blk913_dn8, locals.var_tox1fact__blk913_dn9,)
    }
};
        locals.var_tox1fact__blk913 = assign29850_e32615;
        locals.var_tox1fact__blk913_dn4 = assign29850_e32615_d_n4;
        locals.var_tox1fact__blk913_dn6 = assign29850_e32615_d_n6;
        locals.var_tox1fact__blk913_dn7 = assign29850_e32615_d_n7;
        locals.var_tox1fact__blk913_dn8 = assign29850_e32615_d_n8;
        locals.var_tox1fact__blk913_dn9 = assign29850_e32615_d_n9;

        let (assign29860_e32625, assign29860_e32625_d_n4, assign29860_e32625_d_n6, assign29860_e32625_d_n7, assign29860_e32625_d_n8, assign29860_e32625_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 != 0.0)) {
        let assign29860_e32622: f64 = (locals.var_k2_1d_qm__blk916 * locals.var_temp2);
        let assign29860_e32623: f64 = (1.0 + assign29860_e32622);
        (assign29860_e32623, ((locals.var_k2_1d_qm__blk916_dn4 * locals.var_temp2) + (locals.var_k2_1d_qm__blk916 * locals.var_temp2_dn4)), ((locals.var_k2_1d_qm__blk916_dn6 * locals.var_temp2) + (locals.var_k2_1d_qm__blk916 * locals.var_temp2_dn6)), ((locals.var_k2_1d_qm__blk916_dn7 * locals.var_temp2) + (locals.var_k2_1d_qm__blk916 * locals.var_temp2_dn7)), ((locals.var_k2_1d_qm__blk916_dn8 * locals.var_temp2) + (locals.var_k2_1d_qm__blk916 * locals.var_temp2_dn8)), ((locals.var_k2_1d_qm__blk916_dn9 * locals.var_temp2) + (locals.var_k2_1d_qm__blk916 * locals.var_temp2_dn9)),)
    } else {
        (locals.var_tox2fact__blk914, locals.var_tox2fact__blk914_dn4, locals.var_tox2fact__blk914_dn6, locals.var_tox2fact__blk914_dn7, locals.var_tox2fact__blk914_dn8, locals.var_tox2fact__blk914_dn9,)
    }
};
        locals.var_tox2fact__blk914 = assign29860_e32625;
        locals.var_tox2fact__blk914_dn4 = assign29860_e32625_d_n4;
        locals.var_tox2fact__blk914_dn6 = assign29860_e32625_d_n6;
        locals.var_tox2fact__blk914_dn7 = assign29860_e32625_d_n7;
        locals.var_tox2fact__blk914_dn8 = assign29860_e32625_d_n8;
        locals.var_tox2fact__blk914_dn9 = assign29860_e32625_d_n9;

        let (assign29870_e32632, assign29870_e32632_d_n4, assign29870_e32632_d_n6, assign29870_e32632_d_n7, assign29870_e32632_d_n8, assign29870_e32632_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 == 0.0)) {
        (locals.var_csiprime_0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_csiprime__blk919, locals.var_csiprime__blk919_dn4, locals.var_csiprime__blk919_dn6, locals.var_csiprime__blk919_dn7, locals.var_csiprime__blk919_dn8, locals.var_csiprime__blk919_dn9,)
    }
};
        locals.var_csiprime__blk919 = assign29870_e32632;
        locals.var_csiprime__blk919_dn4 = assign29870_e32632_d_n4;
        locals.var_csiprime__blk919_dn6 = assign29870_e32632_d_n6;
        locals.var_csiprime__blk919_dn7 = assign29870_e32632_d_n7;
        locals.var_csiprime__blk919_dn8 = assign29870_e32632_d_n8;
        locals.var_csiprime__blk919_dn9 = assign29870_e32632_d_n9;

        let (assign29880_e32639, assign29880_e32639_d_n4, assign29880_e32639_d_n6, assign29880_e32639_d_n7, assign29880_e32639_d_n8, assign29880_e32639_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 == 0.0)) {
        (locals.var_k1_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_k1_1d_qm__blk915, locals.var_k1_1d_qm__blk915_dn4, locals.var_k1_1d_qm__blk915_dn6, locals.var_k1_1d_qm__blk915_dn7, locals.var_k1_1d_qm__blk915_dn8, locals.var_k1_1d_qm__blk915_dn9,)
    }
};
        locals.var_k1_1d_qm__blk915 = assign29880_e32639;
        locals.var_k1_1d_qm__blk915_dn4 = assign29880_e32639_d_n4;
        locals.var_k1_1d_qm__blk915_dn6 = assign29880_e32639_d_n6;
        locals.var_k1_1d_qm__blk915_dn7 = assign29880_e32639_d_n7;
        locals.var_k1_1d_qm__blk915_dn8 = assign29880_e32639_d_n8;
        locals.var_k1_1d_qm__blk915_dn9 = assign29880_e32639_d_n9;

        let (assign29890_e32646, assign29890_e32646_d_n4, assign29890_e32646_d_n6, assign29890_e32646_d_n7, assign29890_e32646_d_n8, assign29890_e32646_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 == 0.0)) {
        (locals.var_k2_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_k2_1d_qm__blk916, locals.var_k2_1d_qm__blk916_dn4, locals.var_k2_1d_qm__blk916_dn6, locals.var_k2_1d_qm__blk916_dn7, locals.var_k2_1d_qm__blk916_dn8, locals.var_k2_1d_qm__blk916_dn9,)
    }
};
        locals.var_k2_1d_qm__blk916 = assign29890_e32646;
        locals.var_k2_1d_qm__blk916_dn4 = assign29890_e32646_d_n4;
        locals.var_k2_1d_qm__blk916_dn6 = assign29890_e32646_d_n6;
        locals.var_k2_1d_qm__blk916_dn7 = assign29890_e32646_d_n7;
        locals.var_k2_1d_qm__blk916_dn8 = assign29890_e32646_d_n8;
        locals.var_k2_1d_qm__blk916_dn9 = assign29890_e32646_d_n9;

        let (assign29900_e32653, assign29900_e32653_d_n4, assign29900_e32653_d_n6, assign29900_e32653_d_n7, assign29900_e32653_d_n8, assign29900_e32653_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 == 0.0)) {
        (locals.var_keq_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_keq_1d_qm__blk917, locals.var_keq_1d_qm__blk917_dn4, locals.var_keq_1d_qm__blk917_dn6, locals.var_keq_1d_qm__blk917_dn7, locals.var_keq_1d_qm__blk917_dn8, locals.var_keq_1d_qm__blk917_dn9,)
    }
};
        locals.var_keq_1d_qm__blk917 = assign29900_e32653;
        locals.var_keq_1d_qm__blk917_dn4 = assign29900_e32653_d_n4;
        locals.var_keq_1d_qm__blk917_dn6 = assign29900_e32653_d_n6;
        locals.var_keq_1d_qm__blk917_dn7 = assign29900_e32653_d_n7;
        locals.var_keq_1d_qm__blk917_dn8 = assign29900_e32653_d_n8;
        locals.var_keq_1d_qm__blk917_dn9 = assign29900_e32653_d_n9;

    }

    pub(super) fn stamp_transient_block_79(
        locals: &mut StampLocals,
    ) {
        let (assign29910_e32660, assign29910_e32660_d_n4, assign29910_e32660_d_n6, assign29910_e32660_d_n7, assign29910_e32660_d_n8, assign29910_e32660_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tox1fact__blk913, locals.var_tox1fact__blk913_dn4, locals.var_tox1fact__blk913_dn6, locals.var_tox1fact__blk913_dn7, locals.var_tox1fact__blk913_dn8, locals.var_tox1fact__blk913_dn9,)
    }
};
        locals.var_tox1fact__blk913 = assign29910_e32660;
        locals.var_tox1fact__blk913_dn4 = assign29910_e32660_d_n4;
        locals.var_tox1fact__blk913_dn6 = assign29910_e32660_d_n6;
        locals.var_tox1fact__blk913_dn7 = assign29910_e32660_d_n7;
        locals.var_tox1fact__blk913_dn8 = assign29910_e32660_d_n8;
        locals.var_tox1fact__blk913_dn9 = assign29910_e32660_d_n9;

        let (assign29920_e32667, assign29920_e32667_d_n4, assign29920_e32667_d_n6, assign29920_e32667_d_n7, assign29920_e32667_d_n8, assign29920_e32667_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1093 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tox2fact__blk914, locals.var_tox2fact__blk914_dn4, locals.var_tox2fact__blk914_dn6, locals.var_tox2fact__blk914_dn7, locals.var_tox2fact__blk914_dn8, locals.var_tox2fact__blk914_dn9,)
    }
};
        locals.var_tox2fact__blk914 = assign29920_e32667;
        locals.var_tox2fact__blk914_dn4 = assign29920_e32667_d_n4;
        locals.var_tox2fact__blk914_dn6 = assign29920_e32667_d_n6;
        locals.var_tox2fact__blk914_dn7 = assign29920_e32667_d_n7;
        locals.var_tox2fact__blk914_dn8 = assign29920_e32667_d_n8;
        locals.var_tox2fact__blk914_dn9 = assign29920_e32667_d_n9;

        let (assign29930_e32675, assign29930_e32675_d_n4, assign29930_e32675_d_n6, assign29930_e32675_d_n7, assign29930_e32675_d_n8, assign29930_e32675_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign29930_e32672: f64 = (locals.var_xg10__blk899 - locals.var_xg2eff__blk910);
        let assign29930_e32673: f64 = (locals.var_keq_1d_qm__blk917 * assign29930_e32672);
        (assign29930_e32673, ((locals.var_keq_1d_qm__blk917_dn4 * assign29930_e32672) + (locals.var_keq_1d_qm__blk917 * (locals.var_xg10__blk899_dn4 - locals.var_xg2eff__blk910_dn4))), ((locals.var_keq_1d_qm__blk917_dn6 * assign29930_e32672) + (locals.var_keq_1d_qm__blk917 * (locals.var_xg10__blk899_dn6 - locals.var_xg2eff__blk910_dn6))), ((locals.var_keq_1d_qm__blk917_dn7 * assign29930_e32672) + (locals.var_keq_1d_qm__blk917 * (locals.var_xg10__blk899_dn7 - locals.var_xg2eff__blk910_dn7))), ((locals.var_keq_1d_qm__blk917_dn8 * assign29930_e32672) + (locals.var_keq_1d_qm__blk917 * (locals.var_xg10__blk899_dn8 - locals.var_xg2eff__blk910_dn8))), ((locals.var_keq_1d_qm__blk917_dn9 * assign29930_e32672) + (locals.var_keq_1d_qm__blk917 * (locals.var_xg10__blk899_dn9 - locals.var_xg2eff__blk910_dn9))),)
    } else {
        (locals.var_dx_wi_1d__blk918, locals.var_dx_wi_1d__blk918_dn4, locals.var_dx_wi_1d__blk918_dn6, locals.var_dx_wi_1d__blk918_dn7, locals.var_dx_wi_1d__blk918_dn8, locals.var_dx_wi_1d__blk918_dn9,)
    }
};
        locals.var_dx_wi_1d__blk918 = assign29930_e32675;
        locals.var_dx_wi_1d__blk918_dn4 = assign29930_e32675_d_n4;
        locals.var_dx_wi_1d__blk918_dn6 = assign29930_e32675_d_n6;
        locals.var_dx_wi_1d__blk918_dn7 = assign29930_e32675_d_n7;
        locals.var_dx_wi_1d__blk918_dn8 = assign29930_e32675_d_n8;
        locals.var_dx_wi_1d__blk918_dn9 = assign29930_e32675_d_n9;

        let assign29940_e32678: f64 = if locals.var_dx_wi_1d__blk918 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1094 = assign29940_e32678;

        let assign29950_e32680: f64 = (-locals.var_dx_wi_1d__blk918);
        let assign29950_e32682: f64 = if assign29950_e32680 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1095 = assign29950_e32682;

        let (assign29960_e32695, assign29960_e32695_d_n4, assign29960_e32695_d_n6, assign29960_e32695_d_n7, assign29960_e32695_d_n8, assign29960_e32695_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1094 != 0.0)) && (locals.var_guard1095 != 0.0)) {
        let assign29960_e32690: f64 = (-locals.var_dx_wi_1d__blk918);
        let assign29960_e32691: f64 = (assign29960_e32690).exp();
        let assign29960_e32692: f64 = (1.0 + assign29960_e32691);
        let assign29960_e32693: f64 = (assign29960_e32692).ln();
        (assign29960_e32693, ((assign29960_e32691 * (-locals.var_dx_wi_1d__blk918_dn4)) / assign29960_e32692), ((assign29960_e32691 * (-locals.var_dx_wi_1d__blk918_dn6)) / assign29960_e32692), ((assign29960_e32691 * (-locals.var_dx_wi_1d__blk918_dn7)) / assign29960_e32692), ((assign29960_e32691 * (-locals.var_dx_wi_1d__blk918_dn8)) / assign29960_e32692), ((assign29960_e32691 * (-locals.var_dx_wi_1d__blk918_dn9)) / assign29960_e32692),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign29960_e32695;
        locals.var_temp_dn4 = assign29960_e32695_d_n4;
        locals.var_temp_dn6 = assign29960_e32695_d_n6;
        locals.var_temp_dn7 = assign29960_e32695_d_n7;
        locals.var_temp_dn8 = assign29960_e32695_d_n8;
        locals.var_temp_dn9 = assign29960_e32695_d_n9;

        let (assign29970_e32705, assign29970_e32705_d_n4, assign29970_e32705_d_n6, assign29970_e32705_d_n7, assign29970_e32705_d_n8, assign29970_e32705_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1094 != 0.0)) && (locals.var_guard1095 == 0.0)) {
        let assign29970_e32703: f64 = (-locals.var_dx_wi_1d__blk918);
        (assign29970_e32703, (-locals.var_dx_wi_1d__blk918_dn4), (-locals.var_dx_wi_1d__blk918_dn6), (-locals.var_dx_wi_1d__blk918_dn7), (-locals.var_dx_wi_1d__blk918_dn8), (-locals.var_dx_wi_1d__blk918_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign29970_e32705;
        locals.var_temp_dn4 = assign29970_e32705_d_n4;
        locals.var_temp_dn6 = assign29970_e32705_d_n6;
        locals.var_temp_dn7 = assign29970_e32705_d_n7;
        locals.var_temp_dn8 = assign29970_e32705_d_n8;
        locals.var_temp_dn9 = assign29970_e32705_d_n9;

        let (assign29980_e32719, assign29980_e32719_d_n4, assign29980_e32719_d_n6, assign29980_e32719_d_n7, assign29980_e32719_d_n8, assign29980_e32719_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1094 != 0.0)) {
        let assign29980_e32712: f64 = (locals.var_dx_wi_1d__blk918 / locals.var_k1_1d_qm__blk915);
        let assign29980_e32713: f64 = (locals.var_xg10__blk899 - assign29980_e32712);
        let assign29980_e32715: f64 = (assign29980_e32713 + locals.var_temp);
        let assign29980_e32717: f64 = (assign29980_e32715 - 0.6931471805599);
        (assign29980_e32717, ((locals.var_xg10__blk899_dn4 - (((locals.var_dx_wi_1d__blk918_dn4 * locals.var_k1_1d_qm__blk915) - (locals.var_dx_wi_1d__blk918 * locals.var_k1_1d_qm__blk915_dn4)) / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + locals.var_temp_dn4), ((locals.var_xg10__blk899_dn6 - (((locals.var_dx_wi_1d__blk918_dn6 * locals.var_k1_1d_qm__blk915) - (locals.var_dx_wi_1d__blk918 * locals.var_k1_1d_qm__blk915_dn6)) / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + locals.var_temp_dn6), ((locals.var_xg10__blk899_dn7 - (((locals.var_dx_wi_1d__blk918_dn7 * locals.var_k1_1d_qm__blk915) - (locals.var_dx_wi_1d__blk918 * locals.var_k1_1d_qm__blk915_dn7)) / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + locals.var_temp_dn7), ((locals.var_xg10__blk899_dn8 - (((locals.var_dx_wi_1d__blk918_dn8 * locals.var_k1_1d_qm__blk915) - (locals.var_dx_wi_1d__blk918 * locals.var_k1_1d_qm__blk915_dn8)) / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + locals.var_temp_dn8), ((locals.var_xg10__blk899_dn9 - (((locals.var_dx_wi_1d__blk918_dn9 * locals.var_k1_1d_qm__blk915) - (locals.var_dx_wi_1d__blk918 * locals.var_k1_1d_qm__blk915_dn9)) / (locals.var_k1_1d_qm__blk915 * locals.var_k1_1d_qm__blk915))) + locals.var_temp_dn9),)
    } else {
        (locals.var_x_wi_1d__blk920, locals.var_x_wi_1d__blk920_dn4, locals.var_x_wi_1d__blk920_dn6, locals.var_x_wi_1d__blk920_dn7, locals.var_x_wi_1d__blk920_dn8, locals.var_x_wi_1d__blk920_dn9,)
    }
};
        locals.var_x_wi_1d__blk920 = assign29980_e32719;
        locals.var_x_wi_1d__blk920_dn4 = assign29980_e32719_d_n4;
        locals.var_x_wi_1d__blk920_dn6 = assign29980_e32719_d_n6;
        locals.var_x_wi_1d__blk920_dn7 = assign29980_e32719_d_n7;
        locals.var_x_wi_1d__blk920_dn8 = assign29980_e32719_d_n8;
        locals.var_x_wi_1d__blk920_dn9 = assign29980_e32719_d_n9;

        let assign29990_e32722: f64 = if locals.var_dx_wi_1d__blk918 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1096 = assign29990_e32722;

        let (assign30000_e32735, assign30000_e32735_d_n4, assign30000_e32735_d_n6, assign30000_e32735_d_n7, assign30000_e32735_d_n8, assign30000_e32735_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1094 == 0.0)) && (locals.var_guard1096 != 0.0)) {
        let assign30000_e32731: f64 = (locals.var_dx_wi_1d__blk918).exp();
        let assign30000_e32732: f64 = (1.0 + assign30000_e32731);
        let assign30000_e32733: f64 = (assign30000_e32732).ln();
        (assign30000_e32733, ((assign30000_e32731 * locals.var_dx_wi_1d__blk918_dn4) / assign30000_e32732), ((assign30000_e32731 * locals.var_dx_wi_1d__blk918_dn6) / assign30000_e32732), ((assign30000_e32731 * locals.var_dx_wi_1d__blk918_dn7) / assign30000_e32732), ((assign30000_e32731 * locals.var_dx_wi_1d__blk918_dn8) / assign30000_e32732), ((assign30000_e32731 * locals.var_dx_wi_1d__blk918_dn9) / assign30000_e32732),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign30000_e32735;
        locals.var_temp_dn4 = assign30000_e32735_d_n4;
        locals.var_temp_dn6 = assign30000_e32735_d_n6;
        locals.var_temp_dn7 = assign30000_e32735_d_n7;
        locals.var_temp_dn8 = assign30000_e32735_d_n8;
        locals.var_temp_dn9 = assign30000_e32735_d_n9;

        let (assign30010_e32745, assign30010_e32745_d_n4, assign30010_e32745_d_n6, assign30010_e32745_d_n7, assign30010_e32745_d_n8, assign30010_e32745_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1094 == 0.0)) && (locals.var_guard1096 == 0.0)) {
        (locals.var_dx_wi_1d__blk918, locals.var_dx_wi_1d__blk918_dn4, locals.var_dx_wi_1d__blk918_dn6, locals.var_dx_wi_1d__blk918_dn7, locals.var_dx_wi_1d__blk918_dn8, locals.var_dx_wi_1d__blk918_dn9,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign30010_e32745;
        locals.var_temp_dn4 = assign30010_e32745_d_n4;
        locals.var_temp_dn6 = assign30010_e32745_d_n6;
        locals.var_temp_dn7 = assign30010_e32745_d_n7;
        locals.var_temp_dn8 = assign30010_e32745_d_n8;
        locals.var_temp_dn9 = assign30010_e32745_d_n9;

        let (assign30020_e32760, assign30020_e32760_d_n4, assign30020_e32760_d_n6, assign30020_e32760_d_n7, assign30020_e32760_d_n8, assign30020_e32760_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1094 == 0.0)) {
        let assign30020_e32753: f64 = (locals.var_dx_wi_1d__blk918 / locals.var_k2_1d_qm__blk916);
        let assign30020_e32754: f64 = (locals.var_xg2eff__blk910 + assign30020_e32753);
        let assign30020_e32756: f64 = (assign30020_e32754 + locals.var_temp);
        let assign30020_e32758: f64 = (assign30020_e32756 - 0.6931471805599);
        (assign30020_e32758, ((locals.var_xg2eff__blk910_dn4 + (((locals.var_dx_wi_1d__blk918_dn4 * locals.var_k2_1d_qm__blk916) - (locals.var_dx_wi_1d__blk918 * locals.var_k2_1d_qm__blk916_dn4)) / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916))) + locals.var_temp_dn4), ((locals.var_xg2eff__blk910_dn6 + (((locals.var_dx_wi_1d__blk918_dn6 * locals.var_k2_1d_qm__blk916) - (locals.var_dx_wi_1d__blk918 * locals.var_k2_1d_qm__blk916_dn6)) / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916))) + locals.var_temp_dn6), ((locals.var_xg2eff__blk910_dn7 + (((locals.var_dx_wi_1d__blk918_dn7 * locals.var_k2_1d_qm__blk916) - (locals.var_dx_wi_1d__blk918 * locals.var_k2_1d_qm__blk916_dn7)) / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916))) + locals.var_temp_dn7), ((locals.var_xg2eff__blk910_dn8 + (((locals.var_dx_wi_1d__blk918_dn8 * locals.var_k2_1d_qm__blk916) - (locals.var_dx_wi_1d__blk918 * locals.var_k2_1d_qm__blk916_dn8)) / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916))) + locals.var_temp_dn8), ((locals.var_xg2eff__blk910_dn9 + (((locals.var_dx_wi_1d__blk918_dn9 * locals.var_k2_1d_qm__blk916) - (locals.var_dx_wi_1d__blk918 * locals.var_k2_1d_qm__blk916_dn9)) / (locals.var_k2_1d_qm__blk916 * locals.var_k2_1d_qm__blk916))) + locals.var_temp_dn9),)
    } else {
        (locals.var_x_wi_1d__blk920, locals.var_x_wi_1d__blk920_dn4, locals.var_x_wi_1d__blk920_dn6, locals.var_x_wi_1d__blk920_dn7, locals.var_x_wi_1d__blk920_dn8, locals.var_x_wi_1d__blk920_dn9,)
    }
};
        locals.var_x_wi_1d__blk920 = assign30020_e32760;
        locals.var_x_wi_1d__blk920_dn4 = assign30020_e32760_d_n4;
        locals.var_x_wi_1d__blk920_dn6 = assign30020_e32760_d_n6;
        locals.var_x_wi_1d__blk920_dn7 = assign30020_e32760_d_n7;
        locals.var_x_wi_1d__blk920_dn8 = assign30020_e32760_d_n8;
        locals.var_x_wi_1d__blk920_dn9 = assign30020_e32760_d_n9;

        let (assign30030_e32779, assign30030_e32779_d_n4, assign30030_e32779_d_n6, assign30030_e32779_d_n7, assign30030_e32779_d_n8, assign30030_e32779_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30030_e32765: f64 = (locals.var_x_wi_1d__blk920 + locals.var_xth_1d);
        let assign30030_e32768: f64 = (locals.var_x_wi_1d__blk920 - locals.var_xth_1d);
        let assign30030_e32771: f64 = (locals.var_x_wi_1d__blk920 - locals.var_xth_1d);
        let assign30030_e32772: f64 = (assign30030_e32768 * assign30030_e32771);
        let assign30030_e32774: f64 = (assign30030_e32772 + 4.0);
        let assign30030_e32775: f64 = (assign30030_e32774).sqrt();
        let assign30030_e32776: f64 = (assign30030_e32765 - assign30030_e32775);
        let assign30030_e32777: f64 = (0.5 * assign30030_e32776);
        (assign30030_e32777, (0.5 * ((locals.var_x_wi_1d__blk920_dn4 + locals.var_xth_1d_dn4) - ((((locals.var_x_wi_1d__blk920_dn4 - locals.var_xth_1d_dn4) * assign30030_e32771) + (assign30030_e32768 * (locals.var_x_wi_1d__blk920_dn4 - locals.var_xth_1d_dn4))) / (2.0 * assign30030_e32775)))), (0.5 * ((locals.var_x_wi_1d__blk920_dn6 + locals.var_xth_1d_dn6) - ((((locals.var_x_wi_1d__blk920_dn6 - locals.var_xth_1d_dn6) * assign30030_e32771) + (assign30030_e32768 * (locals.var_x_wi_1d__blk920_dn6 - locals.var_xth_1d_dn6))) / (2.0 * assign30030_e32775)))), (0.5 * ((locals.var_x_wi_1d__blk920_dn7 + locals.var_xth_1d_dn7) - ((((locals.var_x_wi_1d__blk920_dn7 - locals.var_xth_1d_dn7) * assign30030_e32771) + (assign30030_e32768 * (locals.var_x_wi_1d__blk920_dn7 - locals.var_xth_1d_dn7))) / (2.0 * assign30030_e32775)))), (0.5 * ((locals.var_x_wi_1d__blk920_dn8 + locals.var_xth_1d_dn8) - ((((locals.var_x_wi_1d__blk920_dn8 - locals.var_xth_1d_dn8) * assign30030_e32771) + (assign30030_e32768 * (locals.var_x_wi_1d__blk920_dn8 - locals.var_xth_1d_dn8))) / (2.0 * assign30030_e32775)))), (0.5 * ((locals.var_x_wi_1d__blk920_dn9 + locals.var_xth_1d_dn9) - ((((locals.var_x_wi_1d__blk920_dn9 - locals.var_xth_1d_dn9) * assign30030_e32771) + (assign30030_e32768 * (locals.var_x_wi_1d__blk920_dn9 - locals.var_xth_1d_dn9))) / (2.0 * assign30030_e32775)))),)
    } else {
        (locals.var_x_1d__blk921, locals.var_x_1d__blk921_dn4, locals.var_x_1d__blk921_dn6, locals.var_x_1d__blk921_dn7, locals.var_x_1d__blk921_dn8, locals.var_x_1d__blk921_dn9,)
    }
};
        locals.var_x_1d__blk921 = assign30030_e32779;
        locals.var_x_1d__blk921_dn4 = assign30030_e32779_d_n4;
        locals.var_x_1d__blk921_dn6 = assign30030_e32779_d_n6;
        locals.var_x_1d__blk921_dn7 = assign30030_e32779_d_n7;
        locals.var_x_1d__blk921_dn8 = assign30030_e32779_d_n8;
        locals.var_x_1d__blk921_dn9 = assign30030_e32779_d_n9;

        let (assign30040_e32794, assign30040_e32794_d_n4, assign30040_e32794_d_n6, assign30040_e32794_d_n7, assign30040_e32794_d_n8, assign30040_e32794_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30040_e32785: f64 = (locals.var_xth_1d - locals.var_x_1d__blk921);
        let assign30040_e32786: f64 = (2.0 * assign30040_e32785);
        let assign30040_e32788: f64 = (assign30040_e32786 / locals.var_xsddep);
        let assign30040_e32789: f64 = (1.0 + assign30040_e32788);
        let assign30040_e32790: f64 = (assign30040_e32789).sqrt();
        let assign30040_e32792: f64 = (assign30040_e32790 - 1.0);
        (assign30040_e32792, (((((2.0 * (locals.var_xth_1d_dn4 - locals.var_x_1d__blk921_dn4)) * locals.var_xsddep) - (assign30040_e32786 * locals.var_xsddep_dn4)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign30040_e32790)), (((((2.0 * (locals.var_xth_1d_dn6 - locals.var_x_1d__blk921_dn6)) * locals.var_xsddep) - (assign30040_e32786 * locals.var_xsddep_dn6)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign30040_e32790)), (((((2.0 * (locals.var_xth_1d_dn7 - locals.var_x_1d__blk921_dn7)) * locals.var_xsddep) - (assign30040_e32786 * locals.var_xsddep_dn7)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign30040_e32790)), (((((2.0 * (locals.var_xth_1d_dn8 - locals.var_x_1d__blk921_dn8)) * locals.var_xsddep) - (assign30040_e32786 * locals.var_xsddep_dn8)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign30040_e32790)), (((((2.0 * (locals.var_xth_1d_dn9 - locals.var_x_1d__blk921_dn9)) * locals.var_xsddep) - (assign30040_e32786 * locals.var_xsddep_dn9)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign30040_e32790)),)
    } else {
        (locals.var_dleff__blk922, locals.var_dleff__blk922_dn4, locals.var_dleff__blk922_dn6, locals.var_dleff__blk922_dn7, locals.var_dleff__blk922_dn8, locals.var_dleff__blk922_dn9,)
    }
};
        locals.var_dleff__blk922 = assign30040_e32794;
        locals.var_dleff__blk922_dn4 = assign30040_e32794_d_n4;
        locals.var_dleff__blk922_dn6 = assign30040_e32794_d_n6;
        locals.var_dleff__blk922_dn7 = assign30040_e32794_d_n7;
        locals.var_dleff__blk922_dn8 = assign30040_e32794_d_n8;
        locals.var_dleff__blk922_dn9 = assign30040_e32794_d_n9;

        let (assign30050_e32802, assign30050_e32802_d_n4, assign30050_e32802_d_n6, assign30050_e32802_d_n7, assign30050_e32802_d_n8, assign30050_e32802_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30050_e32799: f64 = (locals.var_xsddep * locals.var_dleff__blk922);
        let assign30050_e32800: f64 = (locals.var_x_1d__blk921 + assign30050_e32799);
        (assign30050_e32800, (locals.var_x_1d__blk921_dn4 + ((locals.var_xsddep_dn4 * locals.var_dleff__blk922) + (locals.var_xsddep * locals.var_dleff__blk922_dn4))), (locals.var_x_1d__blk921_dn6 + ((locals.var_xsddep_dn6 * locals.var_dleff__blk922) + (locals.var_xsddep * locals.var_dleff__blk922_dn6))), (locals.var_x_1d__blk921_dn7 + ((locals.var_xsddep_dn7 * locals.var_dleff__blk922) + (locals.var_xsddep * locals.var_dleff__blk922_dn7))), (locals.var_x_1d__blk921_dn8 + ((locals.var_xsddep_dn8 * locals.var_dleff__blk922) + (locals.var_xsddep * locals.var_dleff__blk922_dn8))), (locals.var_x_1d__blk921_dn9 + ((locals.var_xsddep_dn9 * locals.var_dleff__blk922) + (locals.var_xsddep * locals.var_dleff__blk922_dn9))),)
    } else {
        (locals.var_xedge__blk923, locals.var_xedge__blk923_dn4, locals.var_xedge__blk923_dn6, locals.var_xedge__blk923_dn7, locals.var_xedge__blk923_dn8, locals.var_xedge__blk923_dn9,)
    }
};
        locals.var_xedge__blk923 = assign30050_e32802;
        locals.var_xedge__blk923_dn4 = assign30050_e32802_d_n4;
        locals.var_xedge__blk923_dn6 = assign30050_e32802_d_n6;
        locals.var_xedge__blk923_dn7 = assign30050_e32802_d_n7;
        locals.var_xedge__blk923_dn8 = assign30050_e32802_d_n8;
        locals.var_xedge__blk923_dn9 = assign30050_e32802_d_n9;

        let (assign30060_e32833, assign30060_e32833_d_n4, assign30060_e32833_d_n6, assign30060_e32833_d_n7, assign30060_e32833_d_n8, assign30060_e32833_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30060_e32808: f64 = (locals.var_pscedlb_i * locals.var_xg20shift__blk900);
        let assign30060_e32809: f64 = (1.0 + assign30060_e32808);
        let assign30060_e32811: f64 = (assign30060_e32809 + 0.5);
        let assign30060_e32815: f64 = (locals.var_pscedlb_i * locals.var_xg20shift__blk900);
        let assign30060_e32816: f64 = (1.0 + assign30060_e32815);
        let assign30060_e32818: f64 = (assign30060_e32816 - 0.5);
        let assign30060_e32822: f64 = (locals.var_pscedlb_i * locals.var_xg20shift__blk900);
        let assign30060_e32823: f64 = (1.0 + assign30060_e32822);
        let assign30060_e32825: f64 = (assign30060_e32823 - 0.5);
        let assign30060_e32826: f64 = (assign30060_e32818 * assign30060_e32825);
        let assign30060_e32828: f64 = (assign30060_e32826 + 0.01);
        let assign30060_e32829: f64 = (assign30060_e32828).sqrt();
        let assign30060_e32830: f64 = (assign30060_e32811 + assign30060_e32829);
        let assign30060_e32831: f64 = (0.5 * assign30060_e32830);
        (assign30060_e32831, (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn4) + ((((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn4) * assign30060_e32825) + (assign30060_e32818 * (locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn4))) / (2.0 * assign30060_e32829)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn6) + ((((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn6) * assign30060_e32825) + (assign30060_e32818 * (locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn6))) / (2.0 * assign30060_e32829)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn7) + ((((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn7) * assign30060_e32825) + (assign30060_e32818 * (locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn7))) / (2.0 * assign30060_e32829)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn8) + ((((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn8) * assign30060_e32825) + (assign30060_e32818 * (locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn8))) / (2.0 * assign30060_e32829)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn9) + ((((locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn9) * assign30060_e32825) + (assign30060_e32818 * (locals.var_pscedlb_i * locals.var_xg20shift__blk900_dn9))) / (2.0 * assign30060_e32829)))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign30060_e32833;
        locals.var_temp_dn4 = assign30060_e32833_d_n4;
        locals.var_temp_dn6 = assign30060_e32833_d_n6;
        locals.var_temp_dn7 = assign30060_e32833_d_n7;
        locals.var_temp_dn8 = assign30060_e32833_d_n8;
        locals.var_temp_dn9 = assign30060_e32833_d_n9;

        let (assign30070_e32843, assign30070_e32843_d_n4, assign30070_e32843_d_n6, assign30070_e32843_d_n7, assign30070_e32843_d_n8, assign30070_e32843_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30070_e32839: f64 = (locals.var_psce1_loc__blk892 * locals.var_temp);
        let assign30070_e32840: f64 = (1.0 + assign30070_e32839);
        let assign30070_e32841: f64 = (1.0 / assign30070_e32840);
        (assign30070_e32841, (-((locals.var_psce1_loc__blk892 * locals.var_temp_dn4) / (assign30070_e32840 * assign30070_e32840))), (-((locals.var_psce1_loc__blk892 * locals.var_temp_dn6) / (assign30070_e32840 * assign30070_e32840))), (-((locals.var_psce1_loc__blk892 * locals.var_temp_dn7) / (assign30070_e32840 * assign30070_e32840))), (-((locals.var_psce1_loc__blk892 * locals.var_temp_dn8) / (assign30070_e32840 * assign30070_e32840))), (-((locals.var_psce1_loc__blk892 * locals.var_temp_dn9) / (assign30070_e32840 * assign30070_e32840))),)
    } else {
        (locals.var_sce1__blk924, locals.var_sce1__blk924_dn4, locals.var_sce1__blk924_dn6, locals.var_sce1__blk924_dn7, locals.var_sce1__blk924_dn8, locals.var_sce1__blk924_dn9,)
    }
};
        locals.var_sce1__blk924 = assign30070_e32843;
        locals.var_sce1__blk924_dn4 = assign30070_e32843_d_n4;
        locals.var_sce1__blk924_dn6 = assign30070_e32843_d_n6;
        locals.var_sce1__blk924_dn7 = assign30070_e32843_d_n7;
        locals.var_sce1__blk924_dn8 = assign30070_e32843_d_n8;
        locals.var_sce1__blk924_dn9 = assign30070_e32843_d_n9;

        let (assign30080_e32853, assign30080_e32853_d_n4, assign30080_e32853_d_n6, assign30080_e32853_d_n7, assign30080_e32853_d_n8, assign30080_e32853_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30080_e32849: f64 = (locals.var_psce2_loc__blk893 * locals.var_temp);
        let assign30080_e32850: f64 = (1.0 + assign30080_e32849);
        let assign30080_e32851: f64 = (1.0 / assign30080_e32850);
        (assign30080_e32851, (-((locals.var_psce2_loc__blk893 * locals.var_temp_dn4) / (assign30080_e32850 * assign30080_e32850))), (-((locals.var_psce2_loc__blk893 * locals.var_temp_dn6) / (assign30080_e32850 * assign30080_e32850))), (-((locals.var_psce2_loc__blk893 * locals.var_temp_dn7) / (assign30080_e32850 * assign30080_e32850))), (-((locals.var_psce2_loc__blk893 * locals.var_temp_dn8) / (assign30080_e32850 * assign30080_e32850))), (-((locals.var_psce2_loc__blk893 * locals.var_temp_dn9) / (assign30080_e32850 * assign30080_e32850))),)
    } else {
        (locals.var_sce2__blk925, locals.var_sce2__blk925_dn4, locals.var_sce2__blk925_dn6, locals.var_sce2__blk925_dn7, locals.var_sce2__blk925_dn8, locals.var_sce2__blk925_dn9,)
    }
};
        locals.var_sce2__blk925 = assign30080_e32853;
        locals.var_sce2__blk925_dn4 = assign30080_e32853_d_n4;
        locals.var_sce2__blk925_dn6 = assign30080_e32853_d_n6;
        locals.var_sce2__blk925_dn7 = assign30080_e32853_d_n7;
        locals.var_sce2__blk925_dn8 = assign30080_e32853_d_n8;
        locals.var_sce2__blk925_dn9 = assign30080_e32853_d_n9;

        let (assign30090_e32880, assign30090_e32880_d_n4, assign30090_e32880_d_n6, assign30090_e32880_d_n7, assign30090_e32880_d_n8, assign30090_e32880_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30090_e32857: f64 = (2.0 * locals.var_xd0);
        let assign30090_e32861: f64 = (locals.var_xdsx / locals.var_xd0);
        let assign30090_e32862: f64 = (1.0 + assign30090_e32861);
        let assign30090_e32863: f64 = (assign30090_e32862).sqrt();
        let assign30090_e32865: f64 = (assign30090_e32863 - 1.0);
        let assign30090_e32866: f64 = (assign30090_e32857 * assign30090_e32865);
        let assign30090_e32870: f64 = (locals.var_cfdl_i * locals.var_dleff__blk922);
        let assign30090_e32871: f64 = (1.0 + assign30090_e32870);
        let assign30090_e32872: f64 = (assign30090_e32866 * assign30090_e32871);
        let assign30090_e32876: f64 = (locals.var_cfdlb_i * locals.var_xg20shift__blk900);
        let assign30090_e32877: f64 = (1.0 + assign30090_e32876);
        let assign30090_e32878: f64 = (assign30090_e32872 * assign30090_e32877);
        (assign30090_e32878, (((((((2.0 * locals.var_xd0_dn4) * assign30090_e32865) + (assign30090_e32857 * ((((locals.var_xdsx_dn4 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn4)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign30090_e32863)))) * assign30090_e32871) + (assign30090_e32866 * (locals.var_cfdl_i * locals.var_dleff__blk922_dn4))) * assign30090_e32877) + (assign30090_e32872 * (locals.var_cfdlb_i * locals.var_xg20shift__blk900_dn4))), (((((((2.0 * locals.var_xd0_dn6) * assign30090_e32865) + (assign30090_e32857 * ((((locals.var_xdsx_dn6 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn6)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign30090_e32863)))) * assign30090_e32871) + (assign30090_e32866 * (locals.var_cfdl_i * locals.var_dleff__blk922_dn6))) * assign30090_e32877) + (assign30090_e32872 * (locals.var_cfdlb_i * locals.var_xg20shift__blk900_dn6))), (((((((2.0 * locals.var_xd0_dn7) * assign30090_e32865) + (assign30090_e32857 * ((((locals.var_xdsx_dn7 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn7)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign30090_e32863)))) * assign30090_e32871) + (assign30090_e32866 * (locals.var_cfdl_i * locals.var_dleff__blk922_dn7))) * assign30090_e32877) + (assign30090_e32872 * (locals.var_cfdlb_i * locals.var_xg20shift__blk900_dn7))), (((((((2.0 * locals.var_xd0_dn8) * assign30090_e32865) + (assign30090_e32857 * ((((locals.var_xdsx_dn8 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn8)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign30090_e32863)))) * assign30090_e32871) + (assign30090_e32866 * (locals.var_cfdl_i * locals.var_dleff__blk922_dn8))) * assign30090_e32877) + (assign30090_e32872 * (locals.var_cfdlb_i * locals.var_xg20shift__blk900_dn8))), (((((((2.0 * locals.var_xd0_dn9) * assign30090_e32865) + (assign30090_e32857 * ((((locals.var_xdsx_dn9 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn9)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign30090_e32863)))) * assign30090_e32871) + (assign30090_e32866 * (locals.var_cfdl_i * locals.var_dleff__blk922_dn9))) * assign30090_e32877) + (assign30090_e32872 * (locals.var_cfdlb_i * locals.var_xg20shift__blk900_dn9))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign30090_e32880;
        locals.var_temp_dn4 = assign30090_e32880_d_n4;
        locals.var_temp_dn6 = assign30090_e32880_d_n6;
        locals.var_temp_dn7 = assign30090_e32880_d_n7;
        locals.var_temp_dn8 = assign30090_e32880_d_n8;
        locals.var_temp_dn9 = assign30090_e32880_d_n9;

        let (assign30100_e32886, assign30100_e32886_d_n4, assign30100_e32886_d_n6, assign30100_e32886_d_n7, assign30100_e32886_d_n8, assign30100_e32886_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30100_e32884: f64 = (locals.var_cf1_loc__blk894 * locals.var_temp);
        (assign30100_e32884, ((locals.var_cf1_loc__blk894_dn4 * locals.var_temp) + (locals.var_cf1_loc__blk894 * locals.var_temp_dn4)), ((locals.var_cf1_loc__blk894_dn6 * locals.var_temp) + (locals.var_cf1_loc__blk894 * locals.var_temp_dn6)), ((locals.var_cf1_loc__blk894_dn7 * locals.var_temp) + (locals.var_cf1_loc__blk894 * locals.var_temp_dn7)), ((locals.var_cf1_loc__blk894_dn8 * locals.var_temp) + (locals.var_cf1_loc__blk894 * locals.var_temp_dn8)), ((locals.var_cf1_loc__blk894_dn9 * locals.var_temp) + (locals.var_cf1_loc__blk894 * locals.var_temp_dn9)),)
    } else {
        (locals.var_dxg1_dibl__blk926, locals.var_dxg1_dibl__blk926_dn4, locals.var_dxg1_dibl__blk926_dn6, locals.var_dxg1_dibl__blk926_dn7, locals.var_dxg1_dibl__blk926_dn8, locals.var_dxg1_dibl__blk926_dn9,)
    }
};
        locals.var_dxg1_dibl__blk926 = assign30100_e32886;
        locals.var_dxg1_dibl__blk926_dn4 = assign30100_e32886_d_n4;
        locals.var_dxg1_dibl__blk926_dn6 = assign30100_e32886_d_n6;
        locals.var_dxg1_dibl__blk926_dn7 = assign30100_e32886_d_n7;
        locals.var_dxg1_dibl__blk926_dn8 = assign30100_e32886_d_n8;
        locals.var_dxg1_dibl__blk926_dn9 = assign30100_e32886_d_n9;

        let (assign30110_e32892, assign30110_e32892_d_n4, assign30110_e32892_d_n6, assign30110_e32892_d_n7, assign30110_e32892_d_n8, assign30110_e32892_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30110_e32890: f64 = (locals.var_cf2_loc__blk895 * locals.var_temp);
        (assign30110_e32890, ((locals.var_cf2_loc__blk895_dn4 * locals.var_temp) + (locals.var_cf2_loc__blk895 * locals.var_temp_dn4)), ((locals.var_cf2_loc__blk895_dn6 * locals.var_temp) + (locals.var_cf2_loc__blk895 * locals.var_temp_dn6)), ((locals.var_cf2_loc__blk895_dn7 * locals.var_temp) + (locals.var_cf2_loc__blk895 * locals.var_temp_dn7)), ((locals.var_cf2_loc__blk895_dn8 * locals.var_temp) + (locals.var_cf2_loc__blk895 * locals.var_temp_dn8)), ((locals.var_cf2_loc__blk895_dn9 * locals.var_temp) + (locals.var_cf2_loc__blk895 * locals.var_temp_dn9)),)
    } else {
        (locals.var_dxg2_dibl__blk927, locals.var_dxg2_dibl__blk927_dn4, locals.var_dxg2_dibl__blk927_dn6, locals.var_dxg2_dibl__blk927_dn7, locals.var_dxg2_dibl__blk927_dn8, locals.var_dxg2_dibl__blk927_dn9,)
    }
};
        locals.var_dxg2_dibl__blk927 = assign30110_e32892;
        locals.var_dxg2_dibl__blk927_dn4 = assign30110_e32892_d_n4;
        locals.var_dxg2_dibl__blk927_dn6 = assign30110_e32892_d_n6;
        locals.var_dxg2_dibl__blk927_dn7 = assign30110_e32892_d_n7;
        locals.var_dxg2_dibl__blk927_dn8 = assign30110_e32892_d_n8;
        locals.var_dxg2_dibl__blk927_dn9 = assign30110_e32892_d_n9;

        let (assign30120_e32906, assign30120_e32906_d_n4, assign30120_e32906_d_n6, assign30120_e32906_d_n7, assign30120_e32906_d_n8, assign30120_e32906_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30120_e32896: f64 = (locals.var_xg10__blk899 - locals.var_xedge__blk923);
        let assign30120_e32898: f64 = (assign30120_e32896 + locals.var_dxg1_dibl__blk926);
        let assign30120_e32900: f64 = (assign30120_e32898 * locals.var_sce1__blk924);
        let assign30120_e32902: f64 = (assign30120_e32900 + locals.var_xedge__blk923);
        let assign30120_e32904: f64 = (assign30120_e32902 + locals.var_dxdsx);
        (assign30120_e32904, ((((((locals.var_xg10__blk899_dn4 - locals.var_xedge__blk923_dn4) + locals.var_dxg1_dibl__blk926_dn4) * locals.var_sce1__blk924) + (assign30120_e32898 * locals.var_sce1__blk924_dn4)) + locals.var_xedge__blk923_dn4) + locals.var_dxdsx_dn4), ((((((locals.var_xg10__blk899_dn6 - locals.var_xedge__blk923_dn6) + locals.var_dxg1_dibl__blk926_dn6) * locals.var_sce1__blk924) + (assign30120_e32898 * locals.var_sce1__blk924_dn6)) + locals.var_xedge__blk923_dn6) + locals.var_dxdsx_dn6), ((((((locals.var_xg10__blk899_dn7 - locals.var_xedge__blk923_dn7) + locals.var_dxg1_dibl__blk926_dn7) * locals.var_sce1__blk924) + (assign30120_e32898 * locals.var_sce1__blk924_dn7)) + locals.var_xedge__blk923_dn7) + locals.var_dxdsx_dn7), ((((((locals.var_xg10__blk899_dn8 - locals.var_xedge__blk923_dn8) + locals.var_dxg1_dibl__blk926_dn8) * locals.var_sce1__blk924) + (assign30120_e32898 * locals.var_sce1__blk924_dn8)) + locals.var_xedge__blk923_dn8) + locals.var_dxdsx_dn8), ((((((locals.var_xg10__blk899_dn9 - locals.var_xedge__blk923_dn9) + locals.var_dxg1_dibl__blk926_dn9) * locals.var_sce1__blk924) + (assign30120_e32898 * locals.var_sce1__blk924_dn9)) + locals.var_xedge__blk923_dn9) + locals.var_dxdsx_dn9),)
    } else {
        (locals.var_xg1__blk928, locals.var_xg1__blk928_dn4, locals.var_xg1__blk928_dn6, locals.var_xg1__blk928_dn7, locals.var_xg1__blk928_dn8, locals.var_xg1__blk928_dn9,)
    }
};
        locals.var_xg1__blk928 = assign30120_e32906;
        locals.var_xg1__blk928_dn4 = assign30120_e32906_d_n4;
        locals.var_xg1__blk928_dn6 = assign30120_e32906_d_n6;
        locals.var_xg1__blk928_dn7 = assign30120_e32906_d_n7;
        locals.var_xg1__blk928_dn8 = assign30120_e32906_d_n8;
        locals.var_xg1__blk928_dn9 = assign30120_e32906_d_n9;

        let (assign30130_e32920, assign30130_e32920_d_n4, assign30130_e32920_d_n6, assign30130_e32920_d_n7, assign30130_e32920_d_n8, assign30130_e32920_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30130_e32910: f64 = (locals.var_xg2eff__blk910 - locals.var_xedge__blk923);
        let assign30130_e32912: f64 = (assign30130_e32910 + locals.var_dxg2_dibl__blk927);
        let assign30130_e32914: f64 = (assign30130_e32912 * locals.var_sce2__blk925);
        let assign30130_e32916: f64 = (assign30130_e32914 + locals.var_xedge__blk923);
        let assign30130_e32918: f64 = (assign30130_e32916 + locals.var_dxdsx);
        (assign30130_e32918, ((((((locals.var_xg2eff__blk910_dn4 - locals.var_xedge__blk923_dn4) + locals.var_dxg2_dibl__blk927_dn4) * locals.var_sce2__blk925) + (assign30130_e32912 * locals.var_sce2__blk925_dn4)) + locals.var_xedge__blk923_dn4) + locals.var_dxdsx_dn4), ((((((locals.var_xg2eff__blk910_dn6 - locals.var_xedge__blk923_dn6) + locals.var_dxg2_dibl__blk927_dn6) * locals.var_sce2__blk925) + (assign30130_e32912 * locals.var_sce2__blk925_dn6)) + locals.var_xedge__blk923_dn6) + locals.var_dxdsx_dn6), ((((((locals.var_xg2eff__blk910_dn7 - locals.var_xedge__blk923_dn7) + locals.var_dxg2_dibl__blk927_dn7) * locals.var_sce2__blk925) + (assign30130_e32912 * locals.var_sce2__blk925_dn7)) + locals.var_xedge__blk923_dn7) + locals.var_dxdsx_dn7), ((((((locals.var_xg2eff__blk910_dn8 - locals.var_xedge__blk923_dn8) + locals.var_dxg2_dibl__blk927_dn8) * locals.var_sce2__blk925) + (assign30130_e32912 * locals.var_sce2__blk925_dn8)) + locals.var_xedge__blk923_dn8) + locals.var_dxdsx_dn8), ((((((locals.var_xg2eff__blk910_dn9 - locals.var_xedge__blk923_dn9) + locals.var_dxg2_dibl__blk927_dn9) * locals.var_sce2__blk925) + (assign30130_e32912 * locals.var_sce2__blk925_dn9)) + locals.var_xedge__blk923_dn9) + locals.var_dxdsx_dn9),)
    } else {
        (locals.var_xg2__blk929, locals.var_xg2__blk929_dn4, locals.var_xg2__blk929_dn6, locals.var_xg2__blk929_dn7, locals.var_xg2__blk929_dn8, locals.var_xg2__blk929_dn9,)
    }
};
        locals.var_xg2__blk929 = assign30130_e32920;
        locals.var_xg2__blk929_dn4 = assign30130_e32920_d_n4;
        locals.var_xg2__blk929_dn6 = assign30130_e32920_d_n6;
        locals.var_xg2__blk929_dn7 = assign30130_e32920_d_n7;
        locals.var_xg2__blk929_dn8 = assign30130_e32920_d_n8;
        locals.var_xg2__blk929_dn9 = assign30130_e32920_d_n9;

        let (assign30140_e32957, assign30140_e32957_d_n4, assign30140_e32957_d_n6, assign30140_e32957_d_n7, assign30140_e32957_d_n8, assign30140_e32957_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30140_e32927: f64 = (locals.var_xg1__blk928 - locals.var_xg2__blk929);
        let assign30140_e32928: f64 = (locals.var_cic1_i * assign30140_e32927);
        let assign30140_e32929: f64 = (locals.var_xg2__blk929 + assign30140_e32928);
        let assign30140_e32931: f64 = (assign30140_e32929 + locals.var_xsatmax);
        let assign30140_e32936: f64 = (locals.var_xg1__blk928 - locals.var_xg2__blk929);
        let assign30140_e32937: f64 = (locals.var_cic1_i * assign30140_e32936);
        let assign30140_e32938: f64 = (locals.var_xg2__blk929 + assign30140_e32937);
        let assign30140_e32940: f64 = (assign30140_e32938 - locals.var_xsatmax);
        let assign30140_e32945: f64 = (locals.var_xg1__blk928 - locals.var_xg2__blk929);
        let assign30140_e32946: f64 = (locals.var_cic1_i * assign30140_e32945);
        let assign30140_e32947: f64 = (locals.var_xg2__blk929 + assign30140_e32946);
        let assign30140_e32949: f64 = (assign30140_e32947 - locals.var_xsatmax);
        let assign30140_e32950: f64 = (assign30140_e32940 * assign30140_e32949);
        let assign30140_e32952: f64 = (assign30140_e32950 + 0.01);
        let assign30140_e32953: f64 = (assign30140_e32952).sqrt();
        let assign30140_e32954: f64 = (assign30140_e32931 - assign30140_e32953);
        let assign30140_e32955: f64 = (0.5 * assign30140_e32954);
        (assign30140_e32955, (0.5 * (((locals.var_xg2__blk929_dn4 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn4 - locals.var_xg2__blk929_dn4))) + locals.var_xsatmax_dn4) - (((((locals.var_xg2__blk929_dn4 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn4 - locals.var_xg2__blk929_dn4))) - locals.var_xsatmax_dn4) * assign30140_e32949) + (assign30140_e32940 * ((locals.var_xg2__blk929_dn4 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn4 - locals.var_xg2__blk929_dn4))) - locals.var_xsatmax_dn4))) / (2.0 * assign30140_e32953)))), (0.5 * (((locals.var_xg2__blk929_dn6 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn6 - locals.var_xg2__blk929_dn6))) + locals.var_xsatmax_dn6) - (((((locals.var_xg2__blk929_dn6 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn6 - locals.var_xg2__blk929_dn6))) - locals.var_xsatmax_dn6) * assign30140_e32949) + (assign30140_e32940 * ((locals.var_xg2__blk929_dn6 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn6 - locals.var_xg2__blk929_dn6))) - locals.var_xsatmax_dn6))) / (2.0 * assign30140_e32953)))), (0.5 * (((locals.var_xg2__blk929_dn7 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn7 - locals.var_xg2__blk929_dn7))) + locals.var_xsatmax_dn7) - (((((locals.var_xg2__blk929_dn7 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn7 - locals.var_xg2__blk929_dn7))) - locals.var_xsatmax_dn7) * assign30140_e32949) + (assign30140_e32940 * ((locals.var_xg2__blk929_dn7 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn7 - locals.var_xg2__blk929_dn7))) - locals.var_xsatmax_dn7))) / (2.0 * assign30140_e32953)))), (0.5 * (((locals.var_xg2__blk929_dn8 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn8 - locals.var_xg2__blk929_dn8))) + locals.var_xsatmax_dn8) - (((((locals.var_xg2__blk929_dn8 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn8 - locals.var_xg2__blk929_dn8))) - locals.var_xsatmax_dn8) * assign30140_e32949) + (assign30140_e32940 * ((locals.var_xg2__blk929_dn8 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn8 - locals.var_xg2__blk929_dn8))) - locals.var_xsatmax_dn8))) / (2.0 * assign30140_e32953)))), (0.5 * (((locals.var_xg2__blk929_dn9 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn9 - locals.var_xg2__blk929_dn9))) + locals.var_xsatmax_dn9) - (((((locals.var_xg2__blk929_dn9 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn9 - locals.var_xg2__blk929_dn9))) - locals.var_xsatmax_dn9) * assign30140_e32949) + (assign30140_e32940 * ((locals.var_xg2__blk929_dn9 + (locals.var_cic1_i * (locals.var_xg1__blk928_dn9 - locals.var_xg2__blk929_dn9))) - locals.var_xsatmax_dn9))) / (2.0 * assign30140_e32953)))),)
    } else {
        (locals.var_xg1x__blk930, locals.var_xg1x__blk930_dn4, locals.var_xg1x__blk930_dn6, locals.var_xg1x__blk930_dn7, locals.var_xg1x__blk930_dn8, locals.var_xg1x__blk930_dn9,)
    }
};
        locals.var_xg1x__blk930 = assign30140_e32957;
        locals.var_xg1x__blk930_dn4 = assign30140_e32957_d_n4;
        locals.var_xg1x__blk930_dn6 = assign30140_e32957_d_n6;
        locals.var_xg1x__blk930_dn7 = assign30140_e32957_d_n7;
        locals.var_xg1x__blk930_dn8 = assign30140_e32957_d_n8;
        locals.var_xg1x__blk930_dn9 = assign30140_e32957_d_n9;

        let (assign30150_e32994, assign30150_e32994_d_n4, assign30150_e32994_d_n6, assign30150_e32994_d_n7, assign30150_e32994_d_n8, assign30150_e32994_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30150_e32964: f64 = (locals.var_xg2__blk929 - locals.var_xg1__blk928);
        let assign30150_e32965: f64 = (locals.var_cic2_i * assign30150_e32964);
        let assign30150_e32966: f64 = (locals.var_xg1__blk928 + assign30150_e32965);
        let assign30150_e32968: f64 = (assign30150_e32966 + locals.var_xsatmax);
        let assign30150_e32973: f64 = (locals.var_xg2__blk929 - locals.var_xg1__blk928);
        let assign30150_e32974: f64 = (locals.var_cic2_i * assign30150_e32973);
        let assign30150_e32975: f64 = (locals.var_xg1__blk928 + assign30150_e32974);
        let assign30150_e32977: f64 = (assign30150_e32975 - locals.var_xsatmax);
        let assign30150_e32982: f64 = (locals.var_xg2__blk929 - locals.var_xg1__blk928);
        let assign30150_e32983: f64 = (locals.var_cic2_i * assign30150_e32982);
        let assign30150_e32984: f64 = (locals.var_xg1__blk928 + assign30150_e32983);
        let assign30150_e32986: f64 = (assign30150_e32984 - locals.var_xsatmax);
        let assign30150_e32987: f64 = (assign30150_e32977 * assign30150_e32986);
        let assign30150_e32989: f64 = (assign30150_e32987 + 0.01);
        let assign30150_e32990: f64 = (assign30150_e32989).sqrt();
        let assign30150_e32991: f64 = (assign30150_e32968 - assign30150_e32990);
        let assign30150_e32992: f64 = (0.5 * assign30150_e32991);
        (assign30150_e32992, (0.5 * (((locals.var_xg1__blk928_dn4 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn4 - locals.var_xg1__blk928_dn4))) + locals.var_xsatmax_dn4) - (((((locals.var_xg1__blk928_dn4 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn4 - locals.var_xg1__blk928_dn4))) - locals.var_xsatmax_dn4) * assign30150_e32986) + (assign30150_e32977 * ((locals.var_xg1__blk928_dn4 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn4 - locals.var_xg1__blk928_dn4))) - locals.var_xsatmax_dn4))) / (2.0 * assign30150_e32990)))), (0.5 * (((locals.var_xg1__blk928_dn6 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn6 - locals.var_xg1__blk928_dn6))) + locals.var_xsatmax_dn6) - (((((locals.var_xg1__blk928_dn6 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn6 - locals.var_xg1__blk928_dn6))) - locals.var_xsatmax_dn6) * assign30150_e32986) + (assign30150_e32977 * ((locals.var_xg1__blk928_dn6 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn6 - locals.var_xg1__blk928_dn6))) - locals.var_xsatmax_dn6))) / (2.0 * assign30150_e32990)))), (0.5 * (((locals.var_xg1__blk928_dn7 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn7 - locals.var_xg1__blk928_dn7))) + locals.var_xsatmax_dn7) - (((((locals.var_xg1__blk928_dn7 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn7 - locals.var_xg1__blk928_dn7))) - locals.var_xsatmax_dn7) * assign30150_e32986) + (assign30150_e32977 * ((locals.var_xg1__blk928_dn7 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn7 - locals.var_xg1__blk928_dn7))) - locals.var_xsatmax_dn7))) / (2.0 * assign30150_e32990)))), (0.5 * (((locals.var_xg1__blk928_dn8 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn8 - locals.var_xg1__blk928_dn8))) + locals.var_xsatmax_dn8) - (((((locals.var_xg1__blk928_dn8 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn8 - locals.var_xg1__blk928_dn8))) - locals.var_xsatmax_dn8) * assign30150_e32986) + (assign30150_e32977 * ((locals.var_xg1__blk928_dn8 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn8 - locals.var_xg1__blk928_dn8))) - locals.var_xsatmax_dn8))) / (2.0 * assign30150_e32990)))), (0.5 * (((locals.var_xg1__blk928_dn9 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn9 - locals.var_xg1__blk928_dn9))) + locals.var_xsatmax_dn9) - (((((locals.var_xg1__blk928_dn9 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn9 - locals.var_xg1__blk928_dn9))) - locals.var_xsatmax_dn9) * assign30150_e32986) + (assign30150_e32977 * ((locals.var_xg1__blk928_dn9 + (locals.var_cic2_i * (locals.var_xg2__blk929_dn9 - locals.var_xg1__blk928_dn9))) - locals.var_xsatmax_dn9))) / (2.0 * assign30150_e32990)))),)
    } else {
        (locals.var_xg2x__blk931, locals.var_xg2x__blk931_dn4, locals.var_xg2x__blk931_dn6, locals.var_xg2x__blk931_dn7, locals.var_xg2x__blk931_dn8, locals.var_xg2x__blk931_dn9,)
    }
};
        locals.var_xg2x__blk931 = assign30150_e32994;
        locals.var_xg2x__blk931_dn4 = assign30150_e32994_d_n4;
        locals.var_xg2x__blk931_dn6 = assign30150_e32994_d_n6;
        locals.var_xg2x__blk931_dn7 = assign30150_e32994_d_n7;
        locals.var_xg2x__blk931_dn8 = assign30150_e32994_d_n8;
        locals.var_xg2x__blk931_dn9 = assign30150_e32994_d_n9;

        let (assign30160_e33000, assign30160_e33000_d_n4, assign30160_e33000_d_n6, assign30160_e33000_d_n7, assign30160_e33000_d_n8, assign30160_e33000_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30160_e32998: f64 = (locals.var_k1_1d_qm__blk915 / locals.var_sce1__blk924);
        (assign30160_e32998, (((locals.var_k1_1d_qm__blk915_dn4 * locals.var_sce1__blk924) - (locals.var_k1_1d_qm__blk915 * locals.var_sce1__blk924_dn4)) / (locals.var_sce1__blk924 * locals.var_sce1__blk924)), (((locals.var_k1_1d_qm__blk915_dn6 * locals.var_sce1__blk924) - (locals.var_k1_1d_qm__blk915 * locals.var_sce1__blk924_dn6)) / (locals.var_sce1__blk924 * locals.var_sce1__blk924)), (((locals.var_k1_1d_qm__blk915_dn7 * locals.var_sce1__blk924) - (locals.var_k1_1d_qm__blk915 * locals.var_sce1__blk924_dn7)) / (locals.var_sce1__blk924 * locals.var_sce1__blk924)), (((locals.var_k1_1d_qm__blk915_dn8 * locals.var_sce1__blk924) - (locals.var_k1_1d_qm__blk915 * locals.var_sce1__blk924_dn8)) / (locals.var_sce1__blk924 * locals.var_sce1__blk924)), (((locals.var_k1_1d_qm__blk915_dn9 * locals.var_sce1__blk924) - (locals.var_k1_1d_qm__blk915 * locals.var_sce1__blk924_dn9)) / (locals.var_sce1__blk924 * locals.var_sce1__blk924)),)
    } else {
        (locals.var_k1__blk932, locals.var_k1__blk932_dn4, locals.var_k1__blk932_dn6, locals.var_k1__blk932_dn7, locals.var_k1__blk932_dn8, locals.var_k1__blk932_dn9,)
    }
};
        locals.var_k1__blk932 = assign30160_e33000;
        locals.var_k1__blk932_dn4 = assign30160_e33000_d_n4;
        locals.var_k1__blk932_dn6 = assign30160_e33000_d_n6;
        locals.var_k1__blk932_dn7 = assign30160_e33000_d_n7;
        locals.var_k1__blk932_dn8 = assign30160_e33000_d_n8;
        locals.var_k1__blk932_dn9 = assign30160_e33000_d_n9;

        let (assign30170_e33006, assign30170_e33006_d_n4, assign30170_e33006_d_n6, assign30170_e33006_d_n7, assign30170_e33006_d_n8, assign30170_e33006_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30170_e33004: f64 = (locals.var_k2_1d_qm__blk916 / locals.var_sce2__blk925);
        (assign30170_e33004, (((locals.var_k2_1d_qm__blk916_dn4 * locals.var_sce2__blk925) - (locals.var_k2_1d_qm__blk916 * locals.var_sce2__blk925_dn4)) / (locals.var_sce2__blk925 * locals.var_sce2__blk925)), (((locals.var_k2_1d_qm__blk916_dn6 * locals.var_sce2__blk925) - (locals.var_k2_1d_qm__blk916 * locals.var_sce2__blk925_dn6)) / (locals.var_sce2__blk925 * locals.var_sce2__blk925)), (((locals.var_k2_1d_qm__blk916_dn7 * locals.var_sce2__blk925) - (locals.var_k2_1d_qm__blk916 * locals.var_sce2__blk925_dn7)) / (locals.var_sce2__blk925 * locals.var_sce2__blk925)), (((locals.var_k2_1d_qm__blk916_dn8 * locals.var_sce2__blk925) - (locals.var_k2_1d_qm__blk916 * locals.var_sce2__blk925_dn8)) / (locals.var_sce2__blk925 * locals.var_sce2__blk925)), (((locals.var_k2_1d_qm__blk916_dn9 * locals.var_sce2__blk925) - (locals.var_k2_1d_qm__blk916 * locals.var_sce2__blk925_dn9)) / (locals.var_sce2__blk925 * locals.var_sce2__blk925)),)
    } else {
        (locals.var_k2__blk933, locals.var_k2__blk933_dn4, locals.var_k2__blk933_dn6, locals.var_k2__blk933_dn7, locals.var_k2__blk933_dn8, locals.var_k2__blk933_dn9,)
    }
};
        locals.var_k2__blk933 = assign30170_e33006;
        locals.var_k2__blk933_dn4 = assign30170_e33006_d_n4;
        locals.var_k2__blk933_dn6 = assign30170_e33006_d_n6;
        locals.var_k2__blk933_dn7 = assign30170_e33006_d_n7;
        locals.var_k2__blk933_dn8 = assign30170_e33006_d_n8;
        locals.var_k2__blk933_dn9 = assign30170_e33006_d_n9;

        let (assign30180_e33012, assign30180_e33012_d_n4, assign30180_e33012_d_n6, assign30180_e33012_d_n7, assign30180_e33012_d_n8, assign30180_e33012_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30180_e33010: f64 = (1.0 / locals.var_k1__blk932);
        (assign30180_e33010, (-(locals.var_k1__blk932_dn4 / (locals.var_k1__blk932 * locals.var_k1__blk932))), (-(locals.var_k1__blk932_dn6 / (locals.var_k1__blk932 * locals.var_k1__blk932))), (-(locals.var_k1__blk932_dn7 / (locals.var_k1__blk932 * locals.var_k1__blk932))), (-(locals.var_k1__blk932_dn8 / (locals.var_k1__blk932 * locals.var_k1__blk932))), (-(locals.var_k1__blk932_dn9 / (locals.var_k1__blk932 * locals.var_k1__blk932))),)
    } else {
        (locals.var_inv_k1__blk906, locals.var_inv_k1__blk906_dn4, locals.var_inv_k1__blk906_dn6, locals.var_inv_k1__blk906_dn7, locals.var_inv_k1__blk906_dn8, locals.var_inv_k1__blk906_dn9,)
    }
};
        locals.var_inv_k1__blk906 = assign30180_e33012;
        locals.var_inv_k1__blk906_dn4 = assign30180_e33012_d_n4;
        locals.var_inv_k1__blk906_dn6 = assign30180_e33012_d_n6;
        locals.var_inv_k1__blk906_dn7 = assign30180_e33012_d_n7;
        locals.var_inv_k1__blk906_dn8 = assign30180_e33012_d_n8;
        locals.var_inv_k1__blk906_dn9 = assign30180_e33012_d_n9;

        let (assign30190_e33018, assign30190_e33018_d_n4, assign30190_e33018_d_n6, assign30190_e33018_d_n7, assign30190_e33018_d_n8, assign30190_e33018_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30190_e33016: f64 = (1.0 / locals.var_k2__blk933);
        (assign30190_e33016, (-(locals.var_k2__blk933_dn4 / (locals.var_k2__blk933 * locals.var_k2__blk933))), (-(locals.var_k2__blk933_dn6 / (locals.var_k2__blk933 * locals.var_k2__blk933))), (-(locals.var_k2__blk933_dn7 / (locals.var_k2__blk933 * locals.var_k2__blk933))), (-(locals.var_k2__blk933_dn8 / (locals.var_k2__blk933 * locals.var_k2__blk933))), (-(locals.var_k2__blk933_dn9 / (locals.var_k2__blk933 * locals.var_k2__blk933))),)
    } else {
        (locals.var_inv_k2__blk907, locals.var_inv_k2__blk907_dn4, locals.var_inv_k2__blk907_dn6, locals.var_inv_k2__blk907_dn7, locals.var_inv_k2__blk907_dn8, locals.var_inv_k2__blk907_dn9,)
    }
};
        locals.var_inv_k2__blk907 = assign30190_e33018;
        locals.var_inv_k2__blk907_dn4 = assign30190_e33018_d_n4;
        locals.var_inv_k2__blk907_dn6 = assign30190_e33018_d_n6;
        locals.var_inv_k2__blk907_dn7 = assign30190_e33018_d_n7;
        locals.var_inv_k2__blk907_dn8 = assign30190_e33018_d_n8;
        locals.var_inv_k2__blk907_dn9 = assign30190_e33018_d_n9;

        let (assign30200_e33028, assign30200_e33028_d_n4, assign30200_e33028_d_n6, assign30200_e33028_d_n7, assign30200_e33028_d_n8, assign30200_e33028_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30200_e33023: f64 = (1.0 + locals.var_inv_k1__blk906);
        let assign30200_e33025: f64 = (assign30200_e33023 + locals.var_inv_k2__blk907);
        let assign30200_e33026: f64 = (1.0 / assign30200_e33025);
        (assign30200_e33026, (-((locals.var_inv_k1__blk906_dn4 + locals.var_inv_k2__blk907_dn4) / (assign30200_e33025 * assign30200_e33025))), (-((locals.var_inv_k1__blk906_dn6 + locals.var_inv_k2__blk907_dn6) / (assign30200_e33025 * assign30200_e33025))), (-((locals.var_inv_k1__blk906_dn7 + locals.var_inv_k2__blk907_dn7) / (assign30200_e33025 * assign30200_e33025))), (-((locals.var_inv_k1__blk906_dn8 + locals.var_inv_k2__blk907_dn8) / (assign30200_e33025 * assign30200_e33025))), (-((locals.var_inv_k1__blk906_dn9 + locals.var_inv_k2__blk907_dn9) / (assign30200_e33025 * assign30200_e33025))),)
    } else {
        (locals.var_keq__blk934, locals.var_keq__blk934_dn4, locals.var_keq__blk934_dn6, locals.var_keq__blk934_dn7, locals.var_keq__blk934_dn8, locals.var_keq__blk934_dn9,)
    }
};
        locals.var_keq__blk934 = assign30200_e33028;
        locals.var_keq__blk934_dn4 = assign30200_e33028_d_n4;
        locals.var_keq__blk934_dn6 = assign30200_e33028_d_n6;
        locals.var_keq__blk934_dn7 = assign30200_e33028_d_n7;
        locals.var_keq__blk934_dn8 = assign30200_e33028_d_n8;
        locals.var_keq__blk934_dn9 = assign30200_e33028_d_n9;

    }
}
