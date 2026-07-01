#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_77(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27070_e40694, assign27070_e40694_d_n3, assign27070_e40694_d_n4, assign27070_e40694_d_n5, assign27070_e40694_d_n6, assign27070_e40694_d_n7, assign27070_e40694_d_n8, assign27070_e40694_d_n9, assign27070_e40694_d_n10, assign27070_e40694_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard565 != 0.0)) {
        let assign27070_e40692: f64 = (locals.var_t3 + 0.0001);
        (assign27070_e40692, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign27070_e40694;
        locals.var_t4_dn3 = assign27070_e40694_d_n3;
        locals.var_t4_dn4 = assign27070_e40694_d_n4;
        locals.var_t4_dn5 = assign27070_e40694_d_n5;
        locals.var_t4_dn6 = assign27070_e40694_d_n6;
        locals.var_t4_dn7 = assign27070_e40694_d_n7;
        locals.var_t4_dn8 = assign27070_e40694_d_n8;
        locals.var_t4_dn9 = assign27070_e40694_d_n9;
        locals.var_t4_dn10 = assign27070_e40694_d_n10;
        locals.var_t4_dn11 = assign27070_e40694_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign27080_e40727, assign27080_e40727_d_n3, assign27080_e40727_d_n4, assign27080_e40727_d_n5, assign27080_e40727_d_n6, assign27080_e40727_d_n7, assign27080_e40727_d_n8, assign27080_e40727_d_n9, assign27080_e40727_d_n10, assign27080_e40727_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard565 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4;
        let assign27080_e40707: f64 = (locals.var_kgisl_i * __rspice_inv_cse_0);
        let assign27080_e40710: f64 = (locals.var_kgisl_i * __rspice_inv_cse_0);
        let assign27080_e40713: f64 = (locals.var_kgisl_i * __rspice_inv_cse_0);
        let assign27080_e40714: f64 = (assign27080_e40710 * assign27080_e40713);
        let assign27080_e40717: f64 = (4.0 * 1e-6);
        let assign27080_e40719: f64 = (assign27080_e40717 * 1e-6);
        let assign27080_e40720: f64 = (assign27080_e40714 + assign27080_e40719);
        let assign27080_e40721: f64 = (assign27080_e40720).sqrt();
        let assign27080_e40722: f64 = (assign27080_e40707 + assign27080_e40721);
        let assign27080_e40723: f64 = (0.5 * assign27080_e40722);
        let assign27080_e40725: f64 = (assign27080_e40723 - 1e-6);
        (assign27080_e40725, (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) * assign27080_e40713) + (assign27080_e40710 * (-((locals.var_kgisl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign27080_e40721)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) * assign27080_e40713) + (assign27080_e40710 * (-((locals.var_kgisl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign27080_e40721)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) * assign27080_e40713) + (assign27080_e40710 * (-((locals.var_kgisl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign27080_e40721)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) * assign27080_e40713) + (assign27080_e40710 * (-((locals.var_kgisl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign27080_e40721)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) * assign27080_e40713) + (assign27080_e40710 * (-((locals.var_kgisl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign27080_e40721)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) * assign27080_e40713) + (assign27080_e40710 * (-((locals.var_kgisl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign27080_e40721)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) * assign27080_e40713) + (assign27080_e40710 * (-((locals.var_kgisl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign27080_e40721)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) * assign27080_e40713) + (assign27080_e40710 * (-((locals.var_kgisl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign27080_e40721)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) * assign27080_e40713) + (assign27080_e40710 * (-((locals.var_kgisl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign27080_e40721)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign27080_e40727;
        locals.var_t5_dn3 = assign27080_e40727_d_n3;
        locals.var_t5_dn4 = assign27080_e40727_d_n4;
        locals.var_t5_dn5 = assign27080_e40727_d_n5;
        locals.var_t5_dn6 = assign27080_e40727_d_n6;
        locals.var_t5_dn7 = assign27080_e40727_d_n7;
        locals.var_t5_dn8 = assign27080_e40727_d_n8;
        locals.var_t5_dn9 = assign27080_e40727_d_n9;
        locals.var_t5_dn10 = assign27080_e40727_d_n10;
        locals.var_t5_dn11 = assign27080_e40727_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign27090_e40740, assign27090_e40740_d_n3, assign27090_e40740_d_n4, assign27090_e40740_d_n5, assign27090_e40740_d_n6, assign27090_e40740_d_n7, assign27090_e40740_d_n8, assign27090_e40740_d_n9, assign27090_e40740_d_n10, assign27090_e40740_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard565 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign27090_e40740;
        locals.var_t5_dn3 = assign27090_e40740_d_n3;
        locals.var_t5_dn4 = assign27090_e40740_d_n4;
        locals.var_t5_dn5 = assign27090_e40740_d_n5;
        locals.var_t5_dn6 = assign27090_e40740_d_n6;
        locals.var_t5_dn7 = assign27090_e40740_d_n7;
        locals.var_t5_dn8 = assign27090_e40740_d_n8;
        locals.var_t5_dn9 = assign27090_e40740_d_n9;
        locals.var_t5_dn10 = assign27090_e40740_d_n10;
        locals.var_t5_dn11 = assign27090_e40740_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign27100_e40761, assign27100_e40761_d_n3, assign27100_e40761_d_n4, assign27100_e40761_d_n5, assign27100_e40761_d_n6, assign27100_e40761_d_n7, assign27100_e40761_d_n8, assign27100_e40761_d_n9, assign27100_e40761_d_n10, assign27100_e40761_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard564 == 0.0)) {
        let assign27100_e40750: f64 = (locals.var_agisl_i * locals.var_wdios);
        let assign27100_e40752: f64 = (assign27100_e40750 * locals.var_t1);
        let assign27100_e40754: f64 = (-locals.var_t2);
        let assign27100_e40755: f64 = { let limited_exp_arg = assign27100_e40754; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign27100_e40756: f64 = (assign27100_e40752 * assign27100_e40755);
        let assign27100_e40758: f64 = { let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign27100_e40759: f64 = (assign27100_e40756 * assign27100_e40758);
        (assign27100_e40759, (((((assign27100_e40750 * locals.var_t1_dn3) * assign27100_e40755) + (assign27100_e40752 * ({ let limited_exp_arg = assign27100_e40754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * assign27100_e40758) + (assign27100_e40756 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn3))), (((((assign27100_e40750 * locals.var_t1_dn4) * assign27100_e40755) + (assign27100_e40752 * ({ let limited_exp_arg = assign27100_e40754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * assign27100_e40758) + (assign27100_e40756 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn4))), (((((assign27100_e40750 * locals.var_t1_dn5) * assign27100_e40755) + (assign27100_e40752 * ({ let limited_exp_arg = assign27100_e40754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * assign27100_e40758) + (assign27100_e40756 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn5))), (((((assign27100_e40750 * locals.var_t1_dn6) * assign27100_e40755) + (assign27100_e40752 * ({ let limited_exp_arg = assign27100_e40754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * assign27100_e40758) + (assign27100_e40756 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn6))), (((((assign27100_e40750 * locals.var_t1_dn7) * assign27100_e40755) + (assign27100_e40752 * ({ let limited_exp_arg = assign27100_e40754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * assign27100_e40758) + (assign27100_e40756 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn7))), (((((assign27100_e40750 * locals.var_t1_dn8) * assign27100_e40755) + (assign27100_e40752 * ({ let limited_exp_arg = assign27100_e40754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * assign27100_e40758) + (assign27100_e40756 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn8))), (((((assign27100_e40750 * locals.var_t1_dn9) * assign27100_e40755) + (assign27100_e40752 * ({ let limited_exp_arg = assign27100_e40754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * assign27100_e40758) + (assign27100_e40756 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn9))), (((((assign27100_e40750 * locals.var_t1_dn10) * assign27100_e40755) + (assign27100_e40752 * ({ let limited_exp_arg = assign27100_e40754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * assign27100_e40758) + (assign27100_e40756 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn10))), (((((assign27100_e40750 * locals.var_t1_dn11) * assign27100_e40755) + (assign27100_e40752 * ({ let limited_exp_arg = assign27100_e40754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * assign27100_e40758) + (assign27100_e40756 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn11))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign27100_e40761;
        locals.var_t6_dn3 = assign27100_e40761_d_n3;
        locals.var_t6_dn4 = assign27100_e40761_d_n4;
        locals.var_t6_dn5 = assign27100_e40761_d_n5;
        locals.var_t6_dn6 = assign27100_e40761_d_n6;
        locals.var_t6_dn7 = assign27100_e40761_d_n7;
        locals.var_t6_dn8 = assign27100_e40761_d_n8;
        locals.var_t6_dn9 = assign27100_e40761_d_n9;
        locals.var_t6_dn10 = assign27100_e40761_d_n10;
        locals.var_t6_dn11 = assign27100_e40761_d_n11;
        locals.var_t6_rv = 0.0;

        let assign27140_e40787: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard566 = assign27140_e40787;
        locals.var_guard566_rv = 0.0;

        let assign27150_e40794: f64 = if ((locals.var_alpha0_i <= 0.0) || (locals.var_beta0_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard567 = assign27150_e40794;
        locals.var_guard567_rv = 0.0;

        let assign27170_e40806: f64 = (locals.var_beta0_t / 80.0);
        let assign27170_e40807: f64 = if locals.var_diffvds > assign27170_e40806 { 1.0 } else { 0.0 };
        locals.var_guard568 = assign27170_e40807;
        locals.var_guard568_rv = 0.0;

        let (assign27180_e40821, assign27180_e40821_d_n3, assign27180_e40821_d_n4, assign27180_e40821_d_n5, assign27180_e40821_d_n6, assign27180_e40821_d_n7, assign27180_e40821_d_n8, assign27180_e40821_d_n9, assign27180_e40821_d_n10, assign27180_e40821_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 != 0.0)) && (locals.var_guard567 == 0.0)) && (locals.var_guard568 != 0.0)) {
        let assign27180_e40817: f64 = (-locals.var_beta0_t);
        let assign27180_e40819: f64 = (assign27180_e40817 / locals.var_diffvds);
        (assign27180_e40819, (-((assign27180_e40817 * locals.var_diffvds_dn3) / (locals.var_diffvds * locals.var_diffvds))), ((((-locals.var_beta0_t_dn4) * locals.var_diffvds) - (assign27180_e40817 * locals.var_diffvds_dn4)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_t_dn5) * locals.var_diffvds) - (assign27180_e40817 * locals.var_diffvds_dn5)) / (locals.var_diffvds * locals.var_diffvds)), (-((assign27180_e40817 * locals.var_diffvds_dn6) / (locals.var_diffvds * locals.var_diffvds))), (-((assign27180_e40817 * locals.var_diffvds_dn7) / (locals.var_diffvds * locals.var_diffvds))), (-((assign27180_e40817 * locals.var_diffvds_dn8) / (locals.var_diffvds * locals.var_diffvds))), (-((assign27180_e40817 * locals.var_diffvds_dn9) / (locals.var_diffvds * locals.var_diffvds))), (-((assign27180_e40817 * locals.var_diffvds_dn10) / (locals.var_diffvds * locals.var_diffvds))), (-((assign27180_e40817 * locals.var_diffvds_dn11) / (locals.var_diffvds * locals.var_diffvds))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign27180_e40821;
        locals.var_t1_dn3 = assign27180_e40821_d_n3;
        locals.var_t1_dn4 = assign27180_e40821_d_n4;
        locals.var_t1_dn5 = assign27180_e40821_d_n5;
        locals.var_t1_dn6 = assign27180_e40821_d_n6;
        locals.var_t1_dn7 = assign27180_e40821_d_n7;
        locals.var_t1_dn8 = assign27180_e40821_d_n8;
        locals.var_t1_dn9 = assign27180_e40821_d_n9;
        locals.var_t1_dn10 = assign27180_e40821_d_n10;
        locals.var_t1_dn11 = assign27180_e40821_d_n11;
        locals.var_t1_rv = 0.0;

        let assign27210_e40864: f64 = if p.p44 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard569 = assign27210_e40864;
        locals.var_guard569_rv = 0.0;

        let assign27220_e40879: f64 = if ((locals.var_alpha0_i <= 0.0) || (((locals.var_beta2_i == 0.0) && (locals.var_beta1_i == 0.0)) && (locals.var_beta0_t == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard570 = assign27220_e40879;
        locals.var_guard570_rv = 0.0;

        let (assign27240_e40914, assign27240_e40914_d_n4, assign27240_e40914_d_n5,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) {
        let assign27240_e40905: f64 = (locals.var_tratio - 1.0);
        let assign27240_e40906: f64 = (p.p600 * assign27240_e40905);
        let assign27240_e40907: f64 = (1.0 + assign27240_e40906);
        let assign27240_e40908: f64 = (locals.var_vdsatii0_i * assign27240_e40907);
        let assign27240_e40911: f64 = (locals.var_lii_i / locals.var_leff);
        let assign27240_e40912: f64 = (assign27240_e40908 - assign27240_e40911);
        (assign27240_e40912, (locals.var_vdsatii0_i * (p.p600 * locals.var_tratio_dn4)), (locals.var_vdsatii0_i * (p.p600 * locals.var_tratio_dn5)),)
    } else {
        (locals.var_vdsatii0, locals.var_vdsatii0_dn4, locals.var_vdsatii0_dn5,)
    }
};
        locals.var_vdsatii0 = assign27240_e40914;
        locals.var_vdsatii0_dn4 = assign27240_e40914_d_n4;
        locals.var_vdsatii0_dn5 = assign27240_e40914_d_n5;
        locals.var_vdsatii0_rv = 0.0;

        let (assign27250_e40928, assign27250_e40928_d_n3, assign27250_e40928_d_n4, assign27250_e40928_d_n5, assign27250_e40928_d_n6, assign27250_e40928_d_n7, assign27250_e40928_d_n8, assign27250_e40928_d_n9, assign27250_e40928_d_n10, assign27250_e40928_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) {
        let assign27250_e40926: f64 = (locals.var_esatii_i * locals.var_leff);
        (assign27250_e40926, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign27250_e40928;
        locals.var_t0_dn3 = assign27250_e40928_d_n3;
        locals.var_t0_dn4 = assign27250_e40928_d_n4;
        locals.var_t0_dn5 = assign27250_e40928_d_n5;
        locals.var_t0_dn6 = assign27250_e40928_d_n6;
        locals.var_t0_dn7 = assign27250_e40928_d_n7;
        locals.var_t0_dn8 = assign27250_e40928_d_n8;
        locals.var_t0_dn9 = assign27250_e40928_d_n9;
        locals.var_t0_dn10 = assign27250_e40928_d_n10;
        locals.var_t0_dn11 = assign27250_e40928_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign27260_e40946, assign27260_e40946_d_n3, assign27260_e40946_d_n4, assign27260_e40946_d_n5, assign27260_e40946_d_n6, assign27260_e40946_d_n7, assign27260_e40946_d_n8, assign27260_e40946_d_n9, assign27260_e40946_d_n10, assign27260_e40946_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) {
        let assign27260_e40940: f64 = (locals.var_sii0_i * locals.var_t0);
        let assign27260_e40943: f64 = (1.0 + locals.var_t0);
        let assign27260_e40944: f64 = (assign27260_e40940 / assign27260_e40943);
        (assign27260_e40944, ((((locals.var_sii0_i * locals.var_t0_dn3) * assign27260_e40943) - (assign27260_e40940 * locals.var_t0_dn3)) / (assign27260_e40943 * assign27260_e40943)), ((((locals.var_sii0_i * locals.var_t0_dn4) * assign27260_e40943) - (assign27260_e40940 * locals.var_t0_dn4)) / (assign27260_e40943 * assign27260_e40943)), ((((locals.var_sii0_i * locals.var_t0_dn5) * assign27260_e40943) - (assign27260_e40940 * locals.var_t0_dn5)) / (assign27260_e40943 * assign27260_e40943)), ((((locals.var_sii0_i * locals.var_t0_dn6) * assign27260_e40943) - (assign27260_e40940 * locals.var_t0_dn6)) / (assign27260_e40943 * assign27260_e40943)), ((((locals.var_sii0_i * locals.var_t0_dn7) * assign27260_e40943) - (assign27260_e40940 * locals.var_t0_dn7)) / (assign27260_e40943 * assign27260_e40943)), ((((locals.var_sii0_i * locals.var_t0_dn8) * assign27260_e40943) - (assign27260_e40940 * locals.var_t0_dn8)) / (assign27260_e40943 * assign27260_e40943)), ((((locals.var_sii0_i * locals.var_t0_dn9) * assign27260_e40943) - (assign27260_e40940 * locals.var_t0_dn9)) / (assign27260_e40943 * assign27260_e40943)), ((((locals.var_sii0_i * locals.var_t0_dn10) * assign27260_e40943) - (assign27260_e40940 * locals.var_t0_dn10)) / (assign27260_e40943 * assign27260_e40943)), ((((locals.var_sii0_i * locals.var_t0_dn11) * assign27260_e40943) - (assign27260_e40940 * locals.var_t0_dn11)) / (assign27260_e40943 * assign27260_e40943)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign27260_e40946;
        locals.var_t1_dn3 = assign27260_e40946_d_n3;
        locals.var_t1_dn4 = assign27260_e40946_d_n4;
        locals.var_t1_dn5 = assign27260_e40946_d_n5;
        locals.var_t1_dn6 = assign27260_e40946_d_n6;
        locals.var_t1_dn7 = assign27260_e40946_d_n7;
        locals.var_t1_dn8 = assign27260_e40946_d_n8;
        locals.var_t1_dn9 = assign27260_e40946_d_n9;
        locals.var_t1_dn10 = assign27260_e40946_d_n10;
        locals.var_t1_dn11 = assign27260_e40946_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign27270_e40987, assign27270_e40987_d_n3, assign27270_e40987_d_n4, assign27270_e40987_d_n5, assign27270_e40987_d_n6, assign27270_e40987_d_n7, assign27270_e40987_d_n8, assign27270_e40987_d_n9, assign27270_e40987_d_n10, assign27270_e40987_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) {
        let assign27270_e40961: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign27270_e40963: f64 = (assign27270_e40961 * locals.var_nvt);
        let assign27270_e40966: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign27270_e40968: f64 = (assign27270_e40966 * locals.var_nvt);
        let assign27270_e40971: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign27270_e40973: f64 = (assign27270_e40971 * locals.var_nvt);
        let assign27270_e40974: f64 = (assign27270_e40968 * assign27270_e40973);
        let assign27270_e40977: f64 = (4.0 * p.p643);
        let assign27270_e40979: f64 = (assign27270_e40977 * p.p643);
        let assign27270_e40980: f64 = (assign27270_e40974 + assign27270_e40979);
        let assign27270_e40981: f64 = (assign27270_e40980).sqrt();
        let assign27270_e40982: f64 = (assign27270_e40963 + assign27270_e40981);
        let assign27270_e40983: f64 = (0.5 * assign27270_e40982);
        let assign27270_e40984: f64 = (1.0 + assign27270_e40983);
        let assign27270_e40985: f64 = (1.0 / assign27270_e40984);
        (assign27270_e40985, (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign27270_e40961 * locals.var_nvt_dn3)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign27270_e40966 * locals.var_nvt_dn3)) * assign27270_e40973) + (assign27270_e40968 * (((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign27270_e40971 * locals.var_nvt_dn3)))) / (2.0 * assign27270_e40981)))) / (assign27270_e40984 * assign27270_e40984))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign27270_e40961 * locals.var_nvt_dn4)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign27270_e40966 * locals.var_nvt_dn4)) * assign27270_e40973) + (assign27270_e40968 * (((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign27270_e40971 * locals.var_nvt_dn4)))) / (2.0 * assign27270_e40981)))) / (assign27270_e40984 * assign27270_e40984))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign27270_e40961 * locals.var_nvt_dn5)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign27270_e40966 * locals.var_nvt_dn5)) * assign27270_e40973) + (assign27270_e40968 * (((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign27270_e40971 * locals.var_nvt_dn5)))) / (2.0 * assign27270_e40981)))) / (assign27270_e40984 * assign27270_e40984))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign27270_e40961 * locals.var_nvt_dn6)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign27270_e40966 * locals.var_nvt_dn6)) * assign27270_e40973) + (assign27270_e40968 * (((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign27270_e40971 * locals.var_nvt_dn6)))) / (2.0 * assign27270_e40981)))) / (assign27270_e40984 * assign27270_e40984))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign27270_e40961 * locals.var_nvt_dn7)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign27270_e40966 * locals.var_nvt_dn7)) * assign27270_e40973) + (assign27270_e40968 * (((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign27270_e40971 * locals.var_nvt_dn7)))) / (2.0 * assign27270_e40981)))) / (assign27270_e40984 * assign27270_e40984))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign27270_e40961 * locals.var_nvt_dn8)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign27270_e40966 * locals.var_nvt_dn8)) * assign27270_e40973) + (assign27270_e40968 * (((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign27270_e40971 * locals.var_nvt_dn8)))) / (2.0 * assign27270_e40981)))) / (assign27270_e40984 * assign27270_e40984))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign27270_e40961 * locals.var_nvt_dn9)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign27270_e40966 * locals.var_nvt_dn9)) * assign27270_e40973) + (assign27270_e40968 * (((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign27270_e40971 * locals.var_nvt_dn9)))) / (2.0 * assign27270_e40981)))) / (assign27270_e40984 * assign27270_e40984))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign27270_e40961 * locals.var_nvt_dn10)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign27270_e40966 * locals.var_nvt_dn10)) * assign27270_e40973) + (assign27270_e40968 * (((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign27270_e40971 * locals.var_nvt_dn10)))) / (2.0 * assign27270_e40981)))) / (assign27270_e40984 * assign27270_e40984))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign27270_e40961 * locals.var_nvt_dn11)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign27270_e40966 * locals.var_nvt_dn11)) * assign27270_e40973) + (assign27270_e40968 * (((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign27270_e40971 * locals.var_nvt_dn11)))) / (2.0 * assign27270_e40981)))) / (assign27270_e40984 * assign27270_e40984))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign27270_e40987;
        locals.var_t0_dn3 = assign27270_e40987_d_n3;
        locals.var_t0_dn4 = assign27270_e40987_d_n4;
        locals.var_t0_dn5 = assign27270_e40987_d_n5;
        locals.var_t0_dn6 = assign27270_e40987_d_n6;
        locals.var_t0_dn7 = assign27270_e40987_d_n7;
        locals.var_t0_dn8 = assign27270_e40987_d_n8;
        locals.var_t0_dn9 = assign27270_e40987_d_n9;
        locals.var_t0_dn10 = assign27270_e40987_d_n10;
        locals.var_t0_dn11 = assign27270_e40987_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign27280_e41001, assign27280_e41001_d_n3, assign27280_e41001_d_n4, assign27280_e41001_d_n5, assign27280_e41001_d_n6, assign27280_e41001_d_n7, assign27280_e41001_d_n8, assign27280_e41001_d_n9, assign27280_e41001_d_n10, assign27280_e41001_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) {
        let assign27280_e40999: f64 = (locals.var_t0 + locals.var_sii2_i);
        (assign27280_e40999, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign27280_e41001;
        locals.var_t3_dn3 = assign27280_e41001_d_n3;
        locals.var_t3_dn4 = assign27280_e41001_d_n4;
        locals.var_t3_dn5 = assign27280_e41001_d_n5;
        locals.var_t3_dn6 = assign27280_e41001_d_n6;
        locals.var_t3_dn7 = assign27280_e41001_d_n7;
        locals.var_t3_dn8 = assign27280_e41001_d_n8;
        locals.var_t3_dn9 = assign27280_e41001_d_n9;
        locals.var_t3_dn10 = assign27280_e41001_d_n10;
        locals.var_t3_dn11 = assign27280_e41001_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign27290_e41038, assign27290_e41038_d_n3, assign27290_e41038_d_n4, assign27290_e41038_d_n5, assign27290_e41038_d_n6, assign27290_e41038_d_n7, assign27290_e41038_d_n8, assign27290_e41038_d_n9, assign27290_e41038_d_n10, assign27290_e41038_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) {
        let assign27290_e41014: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign27290_e41016: f64 = (assign27290_e41014 * locals.var_t3);
        let assign27290_e41019: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign27290_e41021: f64 = (assign27290_e41019 * locals.var_t3);
        let assign27290_e41024: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign27290_e41026: f64 = (assign27290_e41024 * locals.var_t3);
        let assign27290_e41027: f64 = (assign27290_e41021 * assign27290_e41026);
        let assign27290_e41030: f64 = (4.0 * p.p644);
        let assign27290_e41032: f64 = (assign27290_e41030 * p.p644);
        let assign27290_e41033: f64 = (assign27290_e41027 + assign27290_e41032);
        let assign27290_e41034: f64 = (assign27290_e41033).sqrt();
        let assign27290_e41035: f64 = (assign27290_e41016 + assign27290_e41034);
        let assign27290_e41036: f64 = (0.5 * assign27290_e41035);
        (assign27290_e41036, (0.5 * (((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign27290_e41014 * locals.var_t3_dn3)) + (((((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign27290_e41019 * locals.var_t3_dn3)) * assign27290_e41026) + (assign27290_e41021 * ((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign27290_e41024 * locals.var_t3_dn3)))) / (2.0 * assign27290_e41034)))), (0.5 * (((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign27290_e41014 * locals.var_t3_dn4)) + (((((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign27290_e41019 * locals.var_t3_dn4)) * assign27290_e41026) + (assign27290_e41021 * ((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign27290_e41024 * locals.var_t3_dn4)))) / (2.0 * assign27290_e41034)))), (0.5 * (((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign27290_e41014 * locals.var_t3_dn5)) + (((((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign27290_e41019 * locals.var_t3_dn5)) * assign27290_e41026) + (assign27290_e41021 * ((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign27290_e41024 * locals.var_t3_dn5)))) / (2.0 * assign27290_e41034)))), (0.5 * (((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign27290_e41014 * locals.var_t3_dn6)) + (((((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign27290_e41019 * locals.var_t3_dn6)) * assign27290_e41026) + (assign27290_e41021 * ((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign27290_e41024 * locals.var_t3_dn6)))) / (2.0 * assign27290_e41034)))), (0.5 * (((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign27290_e41014 * locals.var_t3_dn7)) + (((((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign27290_e41019 * locals.var_t3_dn7)) * assign27290_e41026) + (assign27290_e41021 * ((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign27290_e41024 * locals.var_t3_dn7)))) / (2.0 * assign27290_e41034)))), (0.5 * (((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign27290_e41014 * locals.var_t3_dn8)) + (((((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign27290_e41019 * locals.var_t3_dn8)) * assign27290_e41026) + (assign27290_e41021 * ((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign27290_e41024 * locals.var_t3_dn8)))) / (2.0 * assign27290_e41034)))), (0.5 * (((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign27290_e41014 * locals.var_t3_dn9)) + (((((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign27290_e41019 * locals.var_t3_dn9)) * assign27290_e41026) + (assign27290_e41021 * ((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign27290_e41024 * locals.var_t3_dn9)))) / (2.0 * assign27290_e41034)))), (0.5 * (((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign27290_e41014 * locals.var_t3_dn10)) + (((((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign27290_e41019 * locals.var_t3_dn10)) * assign27290_e41026) + (assign27290_e41021 * ((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign27290_e41024 * locals.var_t3_dn10)))) / (2.0 * assign27290_e41034)))), (0.5 * (((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign27290_e41014 * locals.var_t3_dn11)) + (((((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign27290_e41019 * locals.var_t3_dn11)) * assign27290_e41026) + (assign27290_e41021 * ((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign27290_e41024 * locals.var_t3_dn11)))) / (2.0 * assign27290_e41034)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign27290_e41038;
        locals.var_t2_dn3 = assign27290_e41038_d_n3;
        locals.var_t2_dn4 = assign27290_e41038_d_n4;
        locals.var_t2_dn5 = assign27290_e41038_d_n5;
        locals.var_t2_dn6 = assign27290_e41038_d_n6;
        locals.var_t2_dn7 = assign27290_e41038_d_n7;
        locals.var_t2_dn8 = assign27290_e41038_d_n8;
        locals.var_t2_dn9 = assign27290_e41038_d_n9;
        locals.var_t2_dn10 = assign27290_e41038_d_n10;
        locals.var_t2_dn11 = assign27290_e41038_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign27300_e41056, assign27300_e41056_d_n3, assign27300_e41056_d_n4, assign27300_e41056_d_n5, assign27300_e41056_d_n6, assign27300_e41056_d_n7, assign27300_e41056_d_n8, assign27300_e41056_d_n9, assign27300_e41056_d_n10, assign27300_e41056_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) {
        let assign27300_e41052: f64 = (locals.var_siid_i * locals.var_vdsx);
        let assign27300_e41053: f64 = (1.0 + assign27300_e41052);
        let assign27300_e41054: f64 = (1.0 / assign27300_e41053);
        (assign27300_e41054, (-((locals.var_siid_i * locals.var_vdsx_dn3) / (assign27300_e41053 * assign27300_e41053))), (-((locals.var_siid_i * locals.var_vdsx_dn4) / (assign27300_e41053 * assign27300_e41053))), (-((locals.var_siid_i * locals.var_vdsx_dn5) / (assign27300_e41053 * assign27300_e41053))), (-((locals.var_siid_i * locals.var_vdsx_dn6) / (assign27300_e41053 * assign27300_e41053))), (-((locals.var_siid_i * locals.var_vdsx_dn7) / (assign27300_e41053 * assign27300_e41053))), (-((locals.var_siid_i * locals.var_vdsx_dn8) / (assign27300_e41053 * assign27300_e41053))), (-((locals.var_siid_i * locals.var_vdsx_dn9) / (assign27300_e41053 * assign27300_e41053))), (-((locals.var_siid_i * locals.var_vdsx_dn10) / (assign27300_e41053 * assign27300_e41053))), (-((locals.var_siid_i * locals.var_vdsx_dn11) / (assign27300_e41053 * assign27300_e41053))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign27300_e41056;
        locals.var_t3_dn3 = assign27300_e41056_d_n3;
        locals.var_t3_dn4 = assign27300_e41056_d_n4;
        locals.var_t3_dn5 = assign27300_e41056_d_n5;
        locals.var_t3_dn6 = assign27300_e41056_d_n6;
        locals.var_t3_dn7 = assign27300_e41056_d_n7;
        locals.var_t3_dn8 = assign27300_e41056_d_n8;
        locals.var_t3_dn9 = assign27300_e41056_d_n9;
        locals.var_t3_dn10 = assign27300_e41056_d_n10;
        locals.var_t3_dn11 = assign27300_e41056_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign27310_e41072, assign27310_e41072_d_n3, assign27310_e41072_d_n4, assign27310_e41072_d_n5, assign27310_e41072_d_n6, assign27310_e41072_d_n7, assign27310_e41072_d_n8, assign27310_e41072_d_n9, assign27310_e41072_d_n10, assign27310_e41072_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) {
        let assign27310_e41068: f64 = (locals.var_t1 * locals.var_t2);
        let assign27310_e41070: f64 = (assign27310_e41068 * locals.var_t3);
        (assign27310_e41070, ((((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) * locals.var_t3) + (assign27310_e41068 * locals.var_t3_dn3)), ((((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) * locals.var_t3) + (assign27310_e41068 * locals.var_t3_dn4)), ((((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) * locals.var_t3) + (assign27310_e41068 * locals.var_t3_dn5)), ((((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) * locals.var_t3) + (assign27310_e41068 * locals.var_t3_dn6)), ((((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) * locals.var_t3) + (assign27310_e41068 * locals.var_t3_dn7)), ((((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) * locals.var_t3) + (assign27310_e41068 * locals.var_t3_dn8)), ((((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) * locals.var_t3) + (assign27310_e41068 * locals.var_t3_dn9)), ((((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) * locals.var_t3) + (assign27310_e41068 * locals.var_t3_dn10)), ((((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) * locals.var_t3) + (assign27310_e41068 * locals.var_t3_dn11)),)
    } else {
        (locals.var_vgsstep, locals.var_vgsstep_dn3, locals.var_vgsstep_dn4, locals.var_vgsstep_dn5, locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11,)
    }
};
        locals.var_vgsstep = assign27310_e41072;
        locals.var_vgsstep_dn3 = assign27310_e41072_d_n3;
        locals.var_vgsstep_dn4 = assign27310_e41072_d_n4;
        locals.var_vgsstep_dn5 = assign27310_e41072_d_n5;
        locals.var_vgsstep_dn6 = assign27310_e41072_d_n6;
        locals.var_vgsstep_dn7 = assign27310_e41072_d_n7;
        locals.var_vgsstep_dn8 = assign27310_e41072_d_n8;
        locals.var_vgsstep_dn9 = assign27310_e41072_d_n9;
        locals.var_vgsstep_dn10 = assign27310_e41072_d_n10;
        locals.var_vgsstep_dn11 = assign27310_e41072_d_n11;
        locals.var_vgsstep_rv = 0.0;

        let (assign27320_e41086, assign27320_e41086_d_n3, assign27320_e41086_d_n4, assign27320_e41086_d_n5, assign27320_e41086_d_n6, assign27320_e41086_d_n7, assign27320_e41086_d_n8, assign27320_e41086_d_n9, assign27320_e41086_d_n10, assign27320_e41086_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) {
        let assign27320_e41084: f64 = (locals.var_vdsatii0 + locals.var_vgsstep);
        (assign27320_e41084, locals.var_vgsstep_dn3, (locals.var_vdsatii0_dn4 + locals.var_vgsstep_dn4), (locals.var_vdsatii0_dn5 + locals.var_vgsstep_dn5), locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11,)
    } else {
        (locals.var_vdsatii, locals.var_vdsatii_dn3, locals.var_vdsatii_dn4, locals.var_vdsatii_dn5, locals.var_vdsatii_dn6, locals.var_vdsatii_dn7, locals.var_vdsatii_dn8, locals.var_vdsatii_dn9, locals.var_vdsatii_dn10, locals.var_vdsatii_dn11,)
    }
};
        locals.var_vdsatii = assign27320_e41086;
        locals.var_vdsatii_dn3 = assign27320_e41086_d_n3;
        locals.var_vdsatii_dn4 = assign27320_e41086_d_n4;
        locals.var_vdsatii_dn5 = assign27320_e41086_d_n5;
        locals.var_vdsatii_dn6 = assign27320_e41086_d_n6;
        locals.var_vdsatii_dn7 = assign27320_e41086_d_n7;
        locals.var_vdsatii_dn8 = assign27320_e41086_d_n8;
        locals.var_vdsatii_dn9 = assign27320_e41086_d_n9;
        locals.var_vdsatii_dn10 = assign27320_e41086_d_n10;
        locals.var_vdsatii_dn11 = assign27320_e41086_d_n11;
        locals.var_vdsatii_rv = 0.0;

        let (assign27330_e41100, assign27330_e41100_d_n3, assign27330_e41100_d_n4, assign27330_e41100_d_n5, assign27330_e41100_d_n6, assign27330_e41100_d_n7, assign27330_e41100_d_n8, assign27330_e41100_d_n9, assign27330_e41100_d_n10, assign27330_e41100_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) {
        let assign27330_e41098: f64 = (locals.var_vdsx - locals.var_vdsatii);
        (assign27330_e41098, (locals.var_vdsx_dn3 - locals.var_vdsatii_dn3), (locals.var_vdsx_dn4 - locals.var_vdsatii_dn4), (locals.var_vdsx_dn5 - locals.var_vdsatii_dn5), (locals.var_vdsx_dn6 - locals.var_vdsatii_dn6), (locals.var_vdsx_dn7 - locals.var_vdsatii_dn7), (locals.var_vdsx_dn8 - locals.var_vdsatii_dn8), (locals.var_vdsx_dn9 - locals.var_vdsatii_dn9), (locals.var_vdsx_dn10 - locals.var_vdsatii_dn10), (locals.var_vdsx_dn11 - locals.var_vdsatii_dn11),)
    } else {
        (locals.var_vdiff, locals.var_vdiff_dn3, locals.var_vdiff_dn4, locals.var_vdiff_dn5, locals.var_vdiff_dn6, locals.var_vdiff_dn7, locals.var_vdiff_dn8, locals.var_vdiff_dn9, locals.var_vdiff_dn10, locals.var_vdiff_dn11,)
    }
};
        locals.var_vdiff = assign27330_e41100;
        locals.var_vdiff_dn3 = assign27330_e41100_d_n3;
        locals.var_vdiff_dn4 = assign27330_e41100_d_n4;
        locals.var_vdiff_dn5 = assign27330_e41100_d_n5;
        locals.var_vdiff_dn6 = assign27330_e41100_d_n6;
        locals.var_vdiff_dn7 = assign27330_e41100_d_n7;
        locals.var_vdiff_dn8 = assign27330_e41100_d_n8;
        locals.var_vdiff_dn9 = assign27330_e41100_d_n9;
        locals.var_vdiff_dn10 = assign27330_e41100_d_n10;
        locals.var_vdiff_dn11 = assign27330_e41100_d_n11;
        locals.var_vdiff_rv = 0.0;

        let (assign27340_e41122, assign27340_e41122_d_n3, assign27340_e41122_d_n4, assign27340_e41122_d_n5, assign27340_e41122_d_n6, assign27340_e41122_d_n7, assign27340_e41122_d_n8, assign27340_e41122_d_n9, assign27340_e41122_d_n10, assign27340_e41122_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) {
        let assign27340_e41113: f64 = (locals.var_beta1_i * locals.var_vdiff);
        let assign27340_e41114: f64 = (locals.var_beta2_i + assign27340_e41113);
        let assign27340_e41117: f64 = (locals.var_beta0_t * locals.var_vdiff);
        let assign27340_e41119: f64 = (assign27340_e41117 * locals.var_vdiff);
        let assign27340_e41120: f64 = (assign27340_e41114 + assign27340_e41119);
        (assign27340_e41120, ((locals.var_beta1_i * locals.var_vdiff_dn3) + (((locals.var_beta0_t * locals.var_vdiff_dn3) * locals.var_vdiff) + (assign27340_e41117 * locals.var_vdiff_dn3))), ((locals.var_beta1_i * locals.var_vdiff_dn4) + ((((locals.var_beta0_t_dn4 * locals.var_vdiff) + (locals.var_beta0_t * locals.var_vdiff_dn4)) * locals.var_vdiff) + (assign27340_e41117 * locals.var_vdiff_dn4))), ((locals.var_beta1_i * locals.var_vdiff_dn5) + ((((locals.var_beta0_t_dn5 * locals.var_vdiff) + (locals.var_beta0_t * locals.var_vdiff_dn5)) * locals.var_vdiff) + (assign27340_e41117 * locals.var_vdiff_dn5))), ((locals.var_beta1_i * locals.var_vdiff_dn6) + (((locals.var_beta0_t * locals.var_vdiff_dn6) * locals.var_vdiff) + (assign27340_e41117 * locals.var_vdiff_dn6))), ((locals.var_beta1_i * locals.var_vdiff_dn7) + (((locals.var_beta0_t * locals.var_vdiff_dn7) * locals.var_vdiff) + (assign27340_e41117 * locals.var_vdiff_dn7))), ((locals.var_beta1_i * locals.var_vdiff_dn8) + (((locals.var_beta0_t * locals.var_vdiff_dn8) * locals.var_vdiff) + (assign27340_e41117 * locals.var_vdiff_dn8))), ((locals.var_beta1_i * locals.var_vdiff_dn9) + (((locals.var_beta0_t * locals.var_vdiff_dn9) * locals.var_vdiff) + (assign27340_e41117 * locals.var_vdiff_dn9))), ((locals.var_beta1_i * locals.var_vdiff_dn10) + (((locals.var_beta0_t * locals.var_vdiff_dn10) * locals.var_vdiff) + (assign27340_e41117 * locals.var_vdiff_dn10))), ((locals.var_beta1_i * locals.var_vdiff_dn11) + (((locals.var_beta0_t * locals.var_vdiff_dn11) * locals.var_vdiff) + (assign27340_e41117 * locals.var_vdiff_dn11))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign27340_e41122;
        locals.var_t0_dn3 = assign27340_e41122_d_n3;
        locals.var_t0_dn4 = assign27340_e41122_d_n4;
        locals.var_t0_dn5 = assign27340_e41122_d_n5;
        locals.var_t0_dn6 = assign27340_e41122_d_n6;
        locals.var_t0_dn7 = assign27340_e41122_d_n7;
        locals.var_t0_dn8 = assign27340_e41122_d_n8;
        locals.var_t0_dn9 = assign27340_e41122_d_n9;
        locals.var_t0_dn10 = assign27340_e41122_d_n10;
        locals.var_t0_dn11 = assign27340_e41122_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign27350_e41139, assign27350_e41139_d_n3, assign27350_e41139_d_n4, assign27350_e41139_d_n5, assign27350_e41139_d_n6, assign27350_e41139_d_n7, assign27350_e41139_d_n8, assign27350_e41139_d_n9, assign27350_e41139_d_n10, assign27350_e41139_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) {
        let assign27350_e41134: f64 = (locals.var_t0 * locals.var_t0);
        let assign27350_e41136: f64 = (assign27350_e41134 + 1e-10);
        let assign27350_e41137: f64 = (assign27350_e41136).sqrt();
        (assign27350_e41137, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign27350_e41137)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign27350_e41137)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign27350_e41137)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign27350_e41137)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign27350_e41137)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign27350_e41137)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign27350_e41137)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign27350_e41137)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign27350_e41137)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign27350_e41139;
        locals.var_t1_dn3 = assign27350_e41139_d_n3;
        locals.var_t1_dn4 = assign27350_e41139_d_n4;
        locals.var_t1_dn5 = assign27350_e41139_d_n5;
        locals.var_t1_dn6 = assign27350_e41139_d_n6;
        locals.var_t1_dn7 = assign27350_e41139_d_n7;
        locals.var_t1_dn8 = assign27350_e41139_d_n8;
        locals.var_t1_dn9 = assign27350_e41139_d_n9;
        locals.var_t1_dn10 = assign27350_e41139_d_n10;
        locals.var_t1_dn11 = assign27350_e41139_d_n11;
        locals.var_t1_rv = 0.0;

        let assign27380_e41237: f64 = if ((locals.var_alpha0_i <= 0.0) || (((locals.var_beta2_i == 0.0) && (locals.var_beta1_i == 0.0)) && (locals.var_beta0_t == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard571 = assign27380_e41237;
        locals.var_guard571_rv = 0.0;

        let (assign27400_e41274, assign27400_e41274_d_n4, assign27400_e41274_d_n5,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard571 == 0.0)) {
        let assign27400_e41265: f64 = (locals.var_tratio - 1.0);
        let assign27400_e41266: f64 = (p.p600 * assign27400_e41265);
        let assign27400_e41267: f64 = (1.0 + assign27400_e41266);
        let assign27400_e41268: f64 = (locals.var_vdsatii0_i * assign27400_e41267);
        let assign27400_e41271: f64 = (locals.var_lii_i / locals.var_leff);
        let assign27400_e41272: f64 = (assign27400_e41268 - assign27400_e41271);
        (assign27400_e41272, (locals.var_vdsatii0_i * (p.p600 * locals.var_tratio_dn4)), (locals.var_vdsatii0_i * (p.p600 * locals.var_tratio_dn5)),)
    } else {
        (locals.var_vdsatii0, locals.var_vdsatii0_dn4, locals.var_vdsatii0_dn5,)
    }
};
        locals.var_vdsatii0 = assign27400_e41274;
        locals.var_vdsatii0_dn4 = assign27400_e41274_d_n4;
        locals.var_vdsatii0_dn5 = assign27400_e41274_d_n5;
        locals.var_vdsatii0_rv = 0.0;

        let (assign27410_e41289, assign27410_e41289_d_n3, assign27410_e41289_d_n4, assign27410_e41289_d_n5, assign27410_e41289_d_n6, assign27410_e41289_d_n7, assign27410_e41289_d_n8, assign27410_e41289_d_n9, assign27410_e41289_d_n10, assign27410_e41289_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard571 == 0.0)) {
        let assign27410_e41287: f64 = (locals.var_esatii_i * locals.var_leff);
        (assign27410_e41287, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign27410_e41289;
        locals.var_t0_dn3 = assign27410_e41289_d_n3;
        locals.var_t0_dn4 = assign27410_e41289_d_n4;
        locals.var_t0_dn5 = assign27410_e41289_d_n5;
        locals.var_t0_dn6 = assign27410_e41289_d_n6;
        locals.var_t0_dn7 = assign27410_e41289_d_n7;
        locals.var_t0_dn8 = assign27410_e41289_d_n8;
        locals.var_t0_dn9 = assign27410_e41289_d_n9;
        locals.var_t0_dn10 = assign27410_e41289_d_n10;
        locals.var_t0_dn11 = assign27410_e41289_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign27420_e41308, assign27420_e41308_d_n3, assign27420_e41308_d_n4, assign27420_e41308_d_n5, assign27420_e41308_d_n6, assign27420_e41308_d_n7, assign27420_e41308_d_n8, assign27420_e41308_d_n9, assign27420_e41308_d_n10, assign27420_e41308_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard571 == 0.0)) {
        let assign27420_e41302: f64 = (locals.var_sii0_i * locals.var_t0);
        let assign27420_e41305: f64 = (1.0 + locals.var_t0);
        let assign27420_e41306: f64 = (assign27420_e41302 / assign27420_e41305);
        (assign27420_e41306, ((((locals.var_sii0_i * locals.var_t0_dn3) * assign27420_e41305) - (assign27420_e41302 * locals.var_t0_dn3)) / (assign27420_e41305 * assign27420_e41305)), ((((locals.var_sii0_i * locals.var_t0_dn4) * assign27420_e41305) - (assign27420_e41302 * locals.var_t0_dn4)) / (assign27420_e41305 * assign27420_e41305)), ((((locals.var_sii0_i * locals.var_t0_dn5) * assign27420_e41305) - (assign27420_e41302 * locals.var_t0_dn5)) / (assign27420_e41305 * assign27420_e41305)), ((((locals.var_sii0_i * locals.var_t0_dn6) * assign27420_e41305) - (assign27420_e41302 * locals.var_t0_dn6)) / (assign27420_e41305 * assign27420_e41305)), ((((locals.var_sii0_i * locals.var_t0_dn7) * assign27420_e41305) - (assign27420_e41302 * locals.var_t0_dn7)) / (assign27420_e41305 * assign27420_e41305)), ((((locals.var_sii0_i * locals.var_t0_dn8) * assign27420_e41305) - (assign27420_e41302 * locals.var_t0_dn8)) / (assign27420_e41305 * assign27420_e41305)), ((((locals.var_sii0_i * locals.var_t0_dn9) * assign27420_e41305) - (assign27420_e41302 * locals.var_t0_dn9)) / (assign27420_e41305 * assign27420_e41305)), ((((locals.var_sii0_i * locals.var_t0_dn10) * assign27420_e41305) - (assign27420_e41302 * locals.var_t0_dn10)) / (assign27420_e41305 * assign27420_e41305)), ((((locals.var_sii0_i * locals.var_t0_dn11) * assign27420_e41305) - (assign27420_e41302 * locals.var_t0_dn11)) / (assign27420_e41305 * assign27420_e41305)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign27420_e41308;
        locals.var_t1_dn3 = assign27420_e41308_d_n3;
        locals.var_t1_dn4 = assign27420_e41308_d_n4;
        locals.var_t1_dn5 = assign27420_e41308_d_n5;
        locals.var_t1_dn6 = assign27420_e41308_d_n6;
        locals.var_t1_dn7 = assign27420_e41308_d_n7;
        locals.var_t1_dn8 = assign27420_e41308_d_n8;
        locals.var_t1_dn9 = assign27420_e41308_d_n9;
        locals.var_t1_dn10 = assign27420_e41308_d_n10;
        locals.var_t1_dn11 = assign27420_e41308_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign27430_e41350, assign27430_e41350_d_n3, assign27430_e41350_d_n4, assign27430_e41350_d_n5, assign27430_e41350_d_n6, assign27430_e41350_d_n7, assign27430_e41350_d_n8, assign27430_e41350_d_n9, assign27430_e41350_d_n10, assign27430_e41350_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard571 == 0.0)) {
        let assign27430_e41324: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign27430_e41326: f64 = (assign27430_e41324 * locals.var_nvt);
        let assign27430_e41329: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign27430_e41331: f64 = (assign27430_e41329 * locals.var_nvt);
        let assign27430_e41334: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign27430_e41336: f64 = (assign27430_e41334 * locals.var_nvt);
        let assign27430_e41337: f64 = (assign27430_e41331 * assign27430_e41336);
        let assign27430_e41340: f64 = (4.0 * p.p643);
        let assign27430_e41342: f64 = (assign27430_e41340 * p.p643);
        let assign27430_e41343: f64 = (assign27430_e41337 + assign27430_e41342);
        let assign27430_e41344: f64 = (assign27430_e41343).sqrt();
        let assign27430_e41345: f64 = (assign27430_e41326 + assign27430_e41344);
        let assign27430_e41346: f64 = (0.5 * assign27430_e41345);
        let assign27430_e41347: f64 = (1.0 + assign27430_e41346);
        let assign27430_e41348: f64 = (1.0 / assign27430_e41347);
        (assign27430_e41348, (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign27430_e41324 * locals.var_nvt_dn3)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign27430_e41329 * locals.var_nvt_dn3)) * assign27430_e41336) + (assign27430_e41331 * (((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign27430_e41334 * locals.var_nvt_dn3)))) / (2.0 * assign27430_e41344)))) / (assign27430_e41347 * assign27430_e41347))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign27430_e41324 * locals.var_nvt_dn4)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign27430_e41329 * locals.var_nvt_dn4)) * assign27430_e41336) + (assign27430_e41331 * (((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign27430_e41334 * locals.var_nvt_dn4)))) / (2.0 * assign27430_e41344)))) / (assign27430_e41347 * assign27430_e41347))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign27430_e41324 * locals.var_nvt_dn5)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign27430_e41329 * locals.var_nvt_dn5)) * assign27430_e41336) + (assign27430_e41331 * (((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign27430_e41334 * locals.var_nvt_dn5)))) / (2.0 * assign27430_e41344)))) / (assign27430_e41347 * assign27430_e41347))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign27430_e41324 * locals.var_nvt_dn6)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign27430_e41329 * locals.var_nvt_dn6)) * assign27430_e41336) + (assign27430_e41331 * (((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign27430_e41334 * locals.var_nvt_dn6)))) / (2.0 * assign27430_e41344)))) / (assign27430_e41347 * assign27430_e41347))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign27430_e41324 * locals.var_nvt_dn7)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign27430_e41329 * locals.var_nvt_dn7)) * assign27430_e41336) + (assign27430_e41331 * (((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign27430_e41334 * locals.var_nvt_dn7)))) / (2.0 * assign27430_e41344)))) / (assign27430_e41347 * assign27430_e41347))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign27430_e41324 * locals.var_nvt_dn8)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign27430_e41329 * locals.var_nvt_dn8)) * assign27430_e41336) + (assign27430_e41331 * (((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign27430_e41334 * locals.var_nvt_dn8)))) / (2.0 * assign27430_e41344)))) / (assign27430_e41347 * assign27430_e41347))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign27430_e41324 * locals.var_nvt_dn9)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign27430_e41329 * locals.var_nvt_dn9)) * assign27430_e41336) + (assign27430_e41331 * (((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign27430_e41334 * locals.var_nvt_dn9)))) / (2.0 * assign27430_e41344)))) / (assign27430_e41347 * assign27430_e41347))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign27430_e41324 * locals.var_nvt_dn10)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign27430_e41329 * locals.var_nvt_dn10)) * assign27430_e41336) + (assign27430_e41331 * (((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign27430_e41334 * locals.var_nvt_dn10)))) / (2.0 * assign27430_e41344)))) / (assign27430_e41347 * assign27430_e41347))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign27430_e41324 * locals.var_nvt_dn11)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign27430_e41329 * locals.var_nvt_dn11)) * assign27430_e41336) + (assign27430_e41331 * (((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign27430_e41334 * locals.var_nvt_dn11)))) / (2.0 * assign27430_e41344)))) / (assign27430_e41347 * assign27430_e41347))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign27430_e41350;
        locals.var_t0_dn3 = assign27430_e41350_d_n3;
        locals.var_t0_dn4 = assign27430_e41350_d_n4;
        locals.var_t0_dn5 = assign27430_e41350_d_n5;
        locals.var_t0_dn6 = assign27430_e41350_d_n6;
        locals.var_t0_dn7 = assign27430_e41350_d_n7;
        locals.var_t0_dn8 = assign27430_e41350_d_n8;
        locals.var_t0_dn9 = assign27430_e41350_d_n9;
        locals.var_t0_dn10 = assign27430_e41350_d_n10;
        locals.var_t0_dn11 = assign27430_e41350_d_n11;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_78(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27440_e41365, assign27440_e41365_d_n3, assign27440_e41365_d_n4, assign27440_e41365_d_n5, assign27440_e41365_d_n6, assign27440_e41365_d_n7, assign27440_e41365_d_n8, assign27440_e41365_d_n9, assign27440_e41365_d_n10, assign27440_e41365_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard571 == 0.0)) {
        let assign27440_e41363: f64 = (locals.var_t0 + locals.var_sii2_i);
        (assign27440_e41363, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign27440_e41365;
        locals.var_t3_dn3 = assign27440_e41365_d_n3;
        locals.var_t3_dn4 = assign27440_e41365_d_n4;
        locals.var_t3_dn5 = assign27440_e41365_d_n5;
        locals.var_t3_dn6 = assign27440_e41365_d_n6;
        locals.var_t3_dn7 = assign27440_e41365_d_n7;
        locals.var_t3_dn8 = assign27440_e41365_d_n8;
        locals.var_t3_dn9 = assign27440_e41365_d_n9;
        locals.var_t3_dn10 = assign27440_e41365_d_n10;
        locals.var_t3_dn11 = assign27440_e41365_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign27450_e41403, assign27450_e41403_d_n3, assign27450_e41403_d_n4, assign27450_e41403_d_n5, assign27450_e41403_d_n6, assign27450_e41403_d_n7, assign27450_e41403_d_n8, assign27450_e41403_d_n9, assign27450_e41403_d_n10, assign27450_e41403_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard571 == 0.0)) {
        let assign27450_e41379: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign27450_e41381: f64 = (assign27450_e41379 * locals.var_t3);
        let assign27450_e41384: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign27450_e41386: f64 = (assign27450_e41384 * locals.var_t3);
        let assign27450_e41389: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign27450_e41391: f64 = (assign27450_e41389 * locals.var_t3);
        let assign27450_e41392: f64 = (assign27450_e41386 * assign27450_e41391);
        let assign27450_e41395: f64 = (4.0 * p.p644);
        let assign27450_e41397: f64 = (assign27450_e41395 * p.p644);
        let assign27450_e41398: f64 = (assign27450_e41392 + assign27450_e41397);
        let assign27450_e41399: f64 = (assign27450_e41398).sqrt();
        let assign27450_e41400: f64 = (assign27450_e41381 + assign27450_e41399);
        let assign27450_e41401: f64 = (0.5 * assign27450_e41400);
        (assign27450_e41401, (0.5 * (((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign27450_e41379 * locals.var_t3_dn3)) + (((((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign27450_e41384 * locals.var_t3_dn3)) * assign27450_e41391) + (assign27450_e41386 * ((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign27450_e41389 * locals.var_t3_dn3)))) / (2.0 * assign27450_e41399)))), (0.5 * (((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign27450_e41379 * locals.var_t3_dn4)) + (((((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign27450_e41384 * locals.var_t3_dn4)) * assign27450_e41391) + (assign27450_e41386 * ((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign27450_e41389 * locals.var_t3_dn4)))) / (2.0 * assign27450_e41399)))), (0.5 * (((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign27450_e41379 * locals.var_t3_dn5)) + (((((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign27450_e41384 * locals.var_t3_dn5)) * assign27450_e41391) + (assign27450_e41386 * ((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign27450_e41389 * locals.var_t3_dn5)))) / (2.0 * assign27450_e41399)))), (0.5 * (((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign27450_e41379 * locals.var_t3_dn6)) + (((((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign27450_e41384 * locals.var_t3_dn6)) * assign27450_e41391) + (assign27450_e41386 * ((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign27450_e41389 * locals.var_t3_dn6)))) / (2.0 * assign27450_e41399)))), (0.5 * (((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign27450_e41379 * locals.var_t3_dn7)) + (((((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign27450_e41384 * locals.var_t3_dn7)) * assign27450_e41391) + (assign27450_e41386 * ((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign27450_e41389 * locals.var_t3_dn7)))) / (2.0 * assign27450_e41399)))), (0.5 * (((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign27450_e41379 * locals.var_t3_dn8)) + (((((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign27450_e41384 * locals.var_t3_dn8)) * assign27450_e41391) + (assign27450_e41386 * ((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign27450_e41389 * locals.var_t3_dn8)))) / (2.0 * assign27450_e41399)))), (0.5 * (((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign27450_e41379 * locals.var_t3_dn9)) + (((((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign27450_e41384 * locals.var_t3_dn9)) * assign27450_e41391) + (assign27450_e41386 * ((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign27450_e41389 * locals.var_t3_dn9)))) / (2.0 * assign27450_e41399)))), (0.5 * (((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign27450_e41379 * locals.var_t3_dn10)) + (((((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign27450_e41384 * locals.var_t3_dn10)) * assign27450_e41391) + (assign27450_e41386 * ((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign27450_e41389 * locals.var_t3_dn10)))) / (2.0 * assign27450_e41399)))), (0.5 * (((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign27450_e41379 * locals.var_t3_dn11)) + (((((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign27450_e41384 * locals.var_t3_dn11)) * assign27450_e41391) + (assign27450_e41386 * ((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign27450_e41389 * locals.var_t3_dn11)))) / (2.0 * assign27450_e41399)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign27450_e41403;
        locals.var_t2_dn3 = assign27450_e41403_d_n3;
        locals.var_t2_dn4 = assign27450_e41403_d_n4;
        locals.var_t2_dn5 = assign27450_e41403_d_n5;
        locals.var_t2_dn6 = assign27450_e41403_d_n6;
        locals.var_t2_dn7 = assign27450_e41403_d_n7;
        locals.var_t2_dn8 = assign27450_e41403_d_n8;
        locals.var_t2_dn9 = assign27450_e41403_d_n9;
        locals.var_t2_dn10 = assign27450_e41403_d_n10;
        locals.var_t2_dn11 = assign27450_e41403_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign27460_e41422, assign27460_e41422_d_n3, assign27460_e41422_d_n4, assign27460_e41422_d_n5, assign27460_e41422_d_n6, assign27460_e41422_d_n7, assign27460_e41422_d_n8, assign27460_e41422_d_n9, assign27460_e41422_d_n10, assign27460_e41422_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard571 == 0.0)) {
        let assign27460_e41418: f64 = (locals.var_siid_i * locals.var_vdsx);
        let assign27460_e41419: f64 = (1.0 + assign27460_e41418);
        let assign27460_e41420: f64 = (1.0 / assign27460_e41419);
        (assign27460_e41420, (-((locals.var_siid_i * locals.var_vdsx_dn3) / (assign27460_e41419 * assign27460_e41419))), (-((locals.var_siid_i * locals.var_vdsx_dn4) / (assign27460_e41419 * assign27460_e41419))), (-((locals.var_siid_i * locals.var_vdsx_dn5) / (assign27460_e41419 * assign27460_e41419))), (-((locals.var_siid_i * locals.var_vdsx_dn6) / (assign27460_e41419 * assign27460_e41419))), (-((locals.var_siid_i * locals.var_vdsx_dn7) / (assign27460_e41419 * assign27460_e41419))), (-((locals.var_siid_i * locals.var_vdsx_dn8) / (assign27460_e41419 * assign27460_e41419))), (-((locals.var_siid_i * locals.var_vdsx_dn9) / (assign27460_e41419 * assign27460_e41419))), (-((locals.var_siid_i * locals.var_vdsx_dn10) / (assign27460_e41419 * assign27460_e41419))), (-((locals.var_siid_i * locals.var_vdsx_dn11) / (assign27460_e41419 * assign27460_e41419))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign27460_e41422;
        locals.var_t3_dn3 = assign27460_e41422_d_n3;
        locals.var_t3_dn4 = assign27460_e41422_d_n4;
        locals.var_t3_dn5 = assign27460_e41422_d_n5;
        locals.var_t3_dn6 = assign27460_e41422_d_n6;
        locals.var_t3_dn7 = assign27460_e41422_d_n7;
        locals.var_t3_dn8 = assign27460_e41422_d_n8;
        locals.var_t3_dn9 = assign27460_e41422_d_n9;
        locals.var_t3_dn10 = assign27460_e41422_d_n10;
        locals.var_t3_dn11 = assign27460_e41422_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign27470_e41439, assign27470_e41439_d_n3, assign27470_e41439_d_n4, assign27470_e41439_d_n5, assign27470_e41439_d_n6, assign27470_e41439_d_n7, assign27470_e41439_d_n8, assign27470_e41439_d_n9, assign27470_e41439_d_n10, assign27470_e41439_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard571 == 0.0)) {
        let assign27470_e41435: f64 = (locals.var_t1 * locals.var_t2);
        let assign27470_e41437: f64 = (assign27470_e41435 * locals.var_t3);
        (assign27470_e41437, ((((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) * locals.var_t3) + (assign27470_e41435 * locals.var_t3_dn3)), ((((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) * locals.var_t3) + (assign27470_e41435 * locals.var_t3_dn4)), ((((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) * locals.var_t3) + (assign27470_e41435 * locals.var_t3_dn5)), ((((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) * locals.var_t3) + (assign27470_e41435 * locals.var_t3_dn6)), ((((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) * locals.var_t3) + (assign27470_e41435 * locals.var_t3_dn7)), ((((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) * locals.var_t3) + (assign27470_e41435 * locals.var_t3_dn8)), ((((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) * locals.var_t3) + (assign27470_e41435 * locals.var_t3_dn9)), ((((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) * locals.var_t3) + (assign27470_e41435 * locals.var_t3_dn10)), ((((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) * locals.var_t3) + (assign27470_e41435 * locals.var_t3_dn11)),)
    } else {
        (locals.var_vgsstep, locals.var_vgsstep_dn3, locals.var_vgsstep_dn4, locals.var_vgsstep_dn5, locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11,)
    }
};
        locals.var_vgsstep = assign27470_e41439;
        locals.var_vgsstep_dn3 = assign27470_e41439_d_n3;
        locals.var_vgsstep_dn4 = assign27470_e41439_d_n4;
        locals.var_vgsstep_dn5 = assign27470_e41439_d_n5;
        locals.var_vgsstep_dn6 = assign27470_e41439_d_n6;
        locals.var_vgsstep_dn7 = assign27470_e41439_d_n7;
        locals.var_vgsstep_dn8 = assign27470_e41439_d_n8;
        locals.var_vgsstep_dn9 = assign27470_e41439_d_n9;
        locals.var_vgsstep_dn10 = assign27470_e41439_d_n10;
        locals.var_vgsstep_dn11 = assign27470_e41439_d_n11;
        locals.var_vgsstep_rv = 0.0;

        let (assign27480_e41454, assign27480_e41454_d_n3, assign27480_e41454_d_n4, assign27480_e41454_d_n5, assign27480_e41454_d_n6, assign27480_e41454_d_n7, assign27480_e41454_d_n8, assign27480_e41454_d_n9, assign27480_e41454_d_n10, assign27480_e41454_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard571 == 0.0)) {
        let assign27480_e41452: f64 = (locals.var_vdsatii0 + locals.var_vgsstep);
        (assign27480_e41452, locals.var_vgsstep_dn3, (locals.var_vdsatii0_dn4 + locals.var_vgsstep_dn4), (locals.var_vdsatii0_dn5 + locals.var_vgsstep_dn5), locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11,)
    } else {
        (locals.var_vdsatii, locals.var_vdsatii_dn3, locals.var_vdsatii_dn4, locals.var_vdsatii_dn5, locals.var_vdsatii_dn6, locals.var_vdsatii_dn7, locals.var_vdsatii_dn8, locals.var_vdsatii_dn9, locals.var_vdsatii_dn10, locals.var_vdsatii_dn11,)
    }
};
        locals.var_vdsatii = assign27480_e41454;
        locals.var_vdsatii_dn3 = assign27480_e41454_d_n3;
        locals.var_vdsatii_dn4 = assign27480_e41454_d_n4;
        locals.var_vdsatii_dn5 = assign27480_e41454_d_n5;
        locals.var_vdsatii_dn6 = assign27480_e41454_d_n6;
        locals.var_vdsatii_dn7 = assign27480_e41454_d_n7;
        locals.var_vdsatii_dn8 = assign27480_e41454_d_n8;
        locals.var_vdsatii_dn9 = assign27480_e41454_d_n9;
        locals.var_vdsatii_dn10 = assign27480_e41454_d_n10;
        locals.var_vdsatii_dn11 = assign27480_e41454_d_n11;
        locals.var_vdsatii_rv = 0.0;

        let (assign27490_e41469, assign27490_e41469_d_n3, assign27490_e41469_d_n4, assign27490_e41469_d_n5, assign27490_e41469_d_n6, assign27490_e41469_d_n7, assign27490_e41469_d_n8, assign27490_e41469_d_n9, assign27490_e41469_d_n10, assign27490_e41469_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard571 == 0.0)) {
        let assign27490_e41467: f64 = (locals.var_vdsx - locals.var_vdsatii);
        (assign27490_e41467, (locals.var_vdsx_dn3 - locals.var_vdsatii_dn3), (locals.var_vdsx_dn4 - locals.var_vdsatii_dn4), (locals.var_vdsx_dn5 - locals.var_vdsatii_dn5), (locals.var_vdsx_dn6 - locals.var_vdsatii_dn6), (locals.var_vdsx_dn7 - locals.var_vdsatii_dn7), (locals.var_vdsx_dn8 - locals.var_vdsatii_dn8), (locals.var_vdsx_dn9 - locals.var_vdsatii_dn9), (locals.var_vdsx_dn10 - locals.var_vdsatii_dn10), (locals.var_vdsx_dn11 - locals.var_vdsatii_dn11),)
    } else {
        (locals.var_vdiff, locals.var_vdiff_dn3, locals.var_vdiff_dn4, locals.var_vdiff_dn5, locals.var_vdiff_dn6, locals.var_vdiff_dn7, locals.var_vdiff_dn8, locals.var_vdiff_dn9, locals.var_vdiff_dn10, locals.var_vdiff_dn11,)
    }
};
        locals.var_vdiff = assign27490_e41469;
        locals.var_vdiff_dn3 = assign27490_e41469_d_n3;
        locals.var_vdiff_dn4 = assign27490_e41469_d_n4;
        locals.var_vdiff_dn5 = assign27490_e41469_d_n5;
        locals.var_vdiff_dn6 = assign27490_e41469_d_n6;
        locals.var_vdiff_dn7 = assign27490_e41469_d_n7;
        locals.var_vdiff_dn8 = assign27490_e41469_d_n8;
        locals.var_vdiff_dn9 = assign27490_e41469_d_n9;
        locals.var_vdiff_dn10 = assign27490_e41469_d_n10;
        locals.var_vdiff_dn11 = assign27490_e41469_d_n11;
        locals.var_vdiff_rv = 0.0;

        let (assign27500_e41492, assign27500_e41492_d_n3, assign27500_e41492_d_n4, assign27500_e41492_d_n5, assign27500_e41492_d_n6, assign27500_e41492_d_n7, assign27500_e41492_d_n8, assign27500_e41492_d_n9, assign27500_e41492_d_n10, assign27500_e41492_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard571 == 0.0)) {
        let assign27500_e41483: f64 = (locals.var_beta1_i * locals.var_vdiff);
        let assign27500_e41484: f64 = (locals.var_beta2_i + assign27500_e41483);
        let assign27500_e41487: f64 = (locals.var_beta0_t * locals.var_vdiff);
        let assign27500_e41489: f64 = (assign27500_e41487 * locals.var_vdiff);
        let assign27500_e41490: f64 = (assign27500_e41484 + assign27500_e41489);
        (assign27500_e41490, ((locals.var_beta1_i * locals.var_vdiff_dn3) + (((locals.var_beta0_t * locals.var_vdiff_dn3) * locals.var_vdiff) + (assign27500_e41487 * locals.var_vdiff_dn3))), ((locals.var_beta1_i * locals.var_vdiff_dn4) + ((((locals.var_beta0_t_dn4 * locals.var_vdiff) + (locals.var_beta0_t * locals.var_vdiff_dn4)) * locals.var_vdiff) + (assign27500_e41487 * locals.var_vdiff_dn4))), ((locals.var_beta1_i * locals.var_vdiff_dn5) + ((((locals.var_beta0_t_dn5 * locals.var_vdiff) + (locals.var_beta0_t * locals.var_vdiff_dn5)) * locals.var_vdiff) + (assign27500_e41487 * locals.var_vdiff_dn5))), ((locals.var_beta1_i * locals.var_vdiff_dn6) + (((locals.var_beta0_t * locals.var_vdiff_dn6) * locals.var_vdiff) + (assign27500_e41487 * locals.var_vdiff_dn6))), ((locals.var_beta1_i * locals.var_vdiff_dn7) + (((locals.var_beta0_t * locals.var_vdiff_dn7) * locals.var_vdiff) + (assign27500_e41487 * locals.var_vdiff_dn7))), ((locals.var_beta1_i * locals.var_vdiff_dn8) + (((locals.var_beta0_t * locals.var_vdiff_dn8) * locals.var_vdiff) + (assign27500_e41487 * locals.var_vdiff_dn8))), ((locals.var_beta1_i * locals.var_vdiff_dn9) + (((locals.var_beta0_t * locals.var_vdiff_dn9) * locals.var_vdiff) + (assign27500_e41487 * locals.var_vdiff_dn9))), ((locals.var_beta1_i * locals.var_vdiff_dn10) + (((locals.var_beta0_t * locals.var_vdiff_dn10) * locals.var_vdiff) + (assign27500_e41487 * locals.var_vdiff_dn10))), ((locals.var_beta1_i * locals.var_vdiff_dn11) + (((locals.var_beta0_t * locals.var_vdiff_dn11) * locals.var_vdiff) + (assign27500_e41487 * locals.var_vdiff_dn11))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign27500_e41492;
        locals.var_t0_dn3 = assign27500_e41492_d_n3;
        locals.var_t0_dn4 = assign27500_e41492_d_n4;
        locals.var_t0_dn5 = assign27500_e41492_d_n5;
        locals.var_t0_dn6 = assign27500_e41492_d_n6;
        locals.var_t0_dn7 = assign27500_e41492_d_n7;
        locals.var_t0_dn8 = assign27500_e41492_d_n8;
        locals.var_t0_dn9 = assign27500_e41492_d_n9;
        locals.var_t0_dn10 = assign27500_e41492_d_n10;
        locals.var_t0_dn11 = assign27500_e41492_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign27510_e41510, assign27510_e41510_d_n3, assign27510_e41510_d_n4, assign27510_e41510_d_n5, assign27510_e41510_d_n6, assign27510_e41510_d_n7, assign27510_e41510_d_n8, assign27510_e41510_d_n9, assign27510_e41510_d_n10, assign27510_e41510_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard571 == 0.0)) {
        let assign27510_e41505: f64 = (locals.var_t0 * locals.var_t0);
        let assign27510_e41507: f64 = (assign27510_e41505 + 1e-10);
        let assign27510_e41508: f64 = (assign27510_e41507).sqrt();
        (assign27510_e41508, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign27510_e41508)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign27510_e41508)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign27510_e41508)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign27510_e41508)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign27510_e41508)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign27510_e41508)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign27510_e41508)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign27510_e41508)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign27510_e41508)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign27510_e41510;
        locals.var_t1_dn3 = assign27510_e41510_d_n3;
        locals.var_t1_dn4 = assign27510_e41510_d_n4;
        locals.var_t1_dn5 = assign27510_e41510_d_n5;
        locals.var_t1_dn6 = assign27510_e41510_d_n6;
        locals.var_t1_dn7 = assign27510_e41510_d_n7;
        locals.var_t1_dn8 = assign27510_e41510_d_n8;
        locals.var_t1_dn9 = assign27510_e41510_d_n9;
        locals.var_t1_dn10 = assign27510_e41510_d_n10;
        locals.var_t1_dn11 = assign27510_e41510_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign27540_e41605, assign27540_e41605_d_n3, assign27540_e41605_d_n4, assign27540_e41605_d_n5, assign27540_e41605_d_n6, assign27540_e41605_d_n7, assign27540_e41605_d_n8, assign27540_e41605_d_n9, assign27540_e41605_d_n10, assign27540_e41605_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) {
        let assign27540_e41600: f64 = (locals.var_ebjtii_i * locals.var_leff);
        let assign27540_e41601: f64 = (locals.var_cbjtii_i + assign27540_e41600);
        let assign27540_e41603: f64 = (assign27540_e41601 / locals.var_leff);
        (assign27540_e41603, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign27540_e41605;
        locals.var_t0_dn3 = assign27540_e41605_d_n3;
        locals.var_t0_dn4 = assign27540_e41605_d_n4;
        locals.var_t0_dn5 = assign27540_e41605_d_n5;
        locals.var_t0_dn6 = assign27540_e41605_d_n6;
        locals.var_t0_dn7 = assign27540_e41605_d_n7;
        locals.var_t0_dn8 = assign27540_e41605_d_n8;
        locals.var_t0_dn9 = assign27540_e41605_d_n9;
        locals.var_t0_dn10 = assign27540_e41605_d_n10;
        locals.var_t0_dn11 = assign27540_e41605_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign27550_e41623, assign27550_e41623_d_n4, assign27550_e41623_d_n5,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) {
        let assign27550_e41618: f64 = (locals.var_tratio - 1.0);
        let assign27550_e41619: f64 = (p.p666 * assign27550_e41618);
        let assign27550_e41620: f64 = (1.0 + assign27550_e41619);
        let assign27550_e41621: f64 = (locals.var_vbci_i * assign27550_e41620);
        (assign27550_e41621, (locals.var_vbci_i * (p.p666 * locals.var_tratio_dn4)), (locals.var_vbci_i * (p.p666 * locals.var_tratio_dn5)),)
    } else {
        (locals.var_vbc, locals.var_vbc_dn4, locals.var_vbc_dn5,)
    }
};
        locals.var_vbc = assign27550_e41623;
        locals.var_vbc_dn4 = assign27550_e41623_d_n4;
        locals.var_vbc_dn5 = assign27550_e41623_d_n5;
        locals.var_vbc_rv = 0.0;

        let assign27560_e41626: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard572 = assign27560_e41626;
        locals.var_guard572_rv = 0.0;

        let (assign27570_e41640, assign27570_e41640_d_n3, assign27570_e41640_d_n4, assign27570_e41640_d_n5, assign27570_e41640_d_n6, assign27570_e41640_d_n7, assign27570_e41640_d_n8, assign27570_e41640_d_n9, assign27570_e41640_d_n10, assign27570_e41640_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard572 != 0.0)) {
        let assign27570_e41638: f64 = (locals.var_vbc - locals.var_vbd_jct);
        (assign27570_e41638, 0.0, locals.var_vbc_dn4, locals.var_vbc_dn5, (-locals.var_vbd_jct_dn6), 0.0, 0.0, 0.0, (-locals.var_vbd_jct_dn10), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign27570_e41640;
        locals.var_t1_dn3 = assign27570_e41640_d_n3;
        locals.var_t1_dn4 = assign27570_e41640_d_n4;
        locals.var_t1_dn5 = assign27570_e41640_d_n5;
        locals.var_t1_dn6 = assign27570_e41640_d_n6;
        locals.var_t1_dn7 = assign27570_e41640_d_n7;
        locals.var_t1_dn8 = assign27570_e41640_d_n8;
        locals.var_t1_dn9 = assign27570_e41640_d_n9;
        locals.var_t1_dn10 = assign27570_e41640_d_n10;
        locals.var_t1_dn11 = assign27570_e41640_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign27580_e41655, assign27580_e41655_d_n3, assign27580_e41655_d_n4, assign27580_e41655_d_n5, assign27580_e41655_d_n6, assign27580_e41655_d_n7, assign27580_e41655_d_n8, assign27580_e41655_d_n9, assign27580_e41655_d_n10, assign27580_e41655_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard572 == 0.0)) {
        let assign27580_e41653: f64 = (locals.var_vbc - locals.var_vbs_jct);
        (assign27580_e41653, 0.0, locals.var_vbc_dn4, locals.var_vbc_dn5, 0.0, (-locals.var_vbs_jct_dn7), 0.0, 0.0, (-locals.var_vbs_jct_dn10), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign27580_e41655;
        locals.var_t1_dn3 = assign27580_e41655_d_n3;
        locals.var_t1_dn4 = assign27580_e41655_d_n4;
        locals.var_t1_dn5 = assign27580_e41655_d_n5;
        locals.var_t1_dn6 = assign27580_e41655_d_n6;
        locals.var_t1_dn7 = assign27580_e41655_d_n7;
        locals.var_t1_dn8 = assign27580_e41655_d_n8;
        locals.var_t1_dn9 = assign27580_e41655_d_n9;
        locals.var_t1_dn10 = assign27580_e41655_d_n10;
        locals.var_t1_dn11 = assign27580_e41655_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign27590_e41667, assign27590_e41667_d_n3, assign27590_e41667_d_n4, assign27590_e41667_d_n5, assign27590_e41667_d_n6, assign27590_e41667_d_n7, assign27590_e41667_d_n8, assign27590_e41667_d_n9, assign27590_e41667_d_n10, assign27590_e41667_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) {
        let assign27590_e41665: f64 = (locals.var_mbjtii_i - 1.0);
        (assign27590_e41665, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign27590_e41667;
        locals.var_t2_dn3 = assign27590_e41667_d_n3;
        locals.var_t2_dn4 = assign27590_e41667_d_n4;
        locals.var_t2_dn5 = assign27590_e41667_d_n5;
        locals.var_t2_dn6 = assign27590_e41667_d_n6;
        locals.var_t2_dn7 = assign27590_e41667_d_n7;
        locals.var_t2_dn8 = assign27590_e41667_d_n8;
        locals.var_t2_dn9 = assign27590_e41667_d_n9;
        locals.var_t2_dn10 = assign27590_e41667_d_n10;
        locals.var_t2_dn11 = assign27590_e41667_d_n11;
        locals.var_t2_rv = 0.0;

        let assign27600_e41670: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard573 = assign27600_e41670;
        locals.var_guard573_rv = 0.0;

        let (assign27610_e41687, assign27610_e41687_d_n3, assign27610_e41687_d_n4, assign27610_e41687_d_n5, assign27610_e41687_d_n6, assign27610_e41687_d_n7, assign27610_e41687_d_n8, assign27610_e41687_d_n9, assign27610_e41687_d_n10, assign27610_e41687_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard573 != 0.0)) {
        let assign27610_e41681: f64 = (-locals.var_abjtii_i);
        let assign27610_e41684: f64 = (locals.var_t1).powf(locals.var_t2);
        let assign27610_e41685: f64 = (assign27610_e41681 * assign27610_e41684);
        (assign27610_e41685, (assign27610_e41681 * if locals.var_t2_dn3 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn3)) } } else { (assign27610_e41684 * ((locals.var_t2_dn3 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn3 / locals.var_t1)))) }), (assign27610_e41681 * if locals.var_t2_dn4 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn4)) } } else { (assign27610_e41684 * ((locals.var_t2_dn4 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn4 / locals.var_t1)))) }), (assign27610_e41681 * if locals.var_t2_dn5 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn5)) } } else { (assign27610_e41684 * ((locals.var_t2_dn5 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn5 / locals.var_t1)))) }), (assign27610_e41681 * if locals.var_t2_dn6 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn6)) } } else { (assign27610_e41684 * ((locals.var_t2_dn6 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn6 / locals.var_t1)))) }), (assign27610_e41681 * if locals.var_t2_dn7 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn7)) } } else { (assign27610_e41684 * ((locals.var_t2_dn7 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn7 / locals.var_t1)))) }), (assign27610_e41681 * if locals.var_t2_dn8 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn8)) } } else { (assign27610_e41684 * ((locals.var_t2_dn8 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn8 / locals.var_t1)))) }), (assign27610_e41681 * if locals.var_t2_dn9 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn9)) } } else { (assign27610_e41684 * ((locals.var_t2_dn9 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn9 / locals.var_t1)))) }), (assign27610_e41681 * if locals.var_t2_dn10 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn10)) } } else { (assign27610_e41684 * ((locals.var_t2_dn10 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn10 / locals.var_t1)))) }), (assign27610_e41681 * if locals.var_t2_dn11 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn11)) } } else { (assign27610_e41684 * ((locals.var_t2_dn11 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn11 / locals.var_t1)))) }),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign27610_e41687;
        locals.var_t3_dn3 = assign27610_e41687_d_n3;
        locals.var_t3_dn4 = assign27610_e41687_d_n4;
        locals.var_t3_dn5 = assign27610_e41687_d_n5;
        locals.var_t3_dn6 = assign27610_e41687_d_n6;
        locals.var_t3_dn7 = assign27610_e41687_d_n7;
        locals.var_t3_dn8 = assign27610_e41687_d_n8;
        locals.var_t3_dn9 = assign27610_e41687_d_n9;
        locals.var_t3_dn10 = assign27610_e41687_d_n10;
        locals.var_t3_dn11 = assign27610_e41687_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign27620_e41700, assign27620_e41700_d_n3, assign27620_e41700_d_n4, assign27620_e41700_d_n5, assign27620_e41700_d_n6, assign27620_e41700_d_n7, assign27620_e41700_d_n8, assign27620_e41700_d_n9, assign27620_e41700_d_n10, assign27620_e41700_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard573 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign27620_e41700;
        locals.var_t3_dn3 = assign27620_e41700_d_n3;
        locals.var_t3_dn4 = assign27620_e41700_d_n4;
        locals.var_t3_dn5 = assign27620_e41700_d_n5;
        locals.var_t3_dn6 = assign27620_e41700_d_n6;
        locals.var_t3_dn7 = assign27620_e41700_d_n7;
        locals.var_t3_dn8 = assign27620_e41700_d_n8;
        locals.var_t3_dn9 = assign27620_e41700_d_n9;
        locals.var_t3_dn10 = assign27620_e41700_d_n10;
        locals.var_t3_dn11 = assign27620_e41700_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign27630_e41711, assign27630_e41711_d_n3, assign27630_e41711_d_n4, assign27630_e41711_d_n5, assign27630_e41711_d_n6, assign27630_e41711_d_n7, assign27630_e41711_d_n8, assign27630_e41711_d_n9, assign27630_e41711_d_n10, assign27630_e41711_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard566 == 0.0)) && (locals.var_guard569 == 0.0)) {
        let assign27630_e41709: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign27630_e41709, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign27630_e41711;
        locals.var_t4_dn3 = assign27630_e41711_d_n3;
        locals.var_t4_dn4 = assign27630_e41711_d_n4;
        locals.var_t4_dn5 = assign27630_e41711_d_n5;
        locals.var_t4_dn6 = assign27630_e41711_d_n6;
        locals.var_t4_dn7 = assign27630_e41711_d_n7;
        locals.var_t4_dn8 = assign27630_e41711_d_n8;
        locals.var_t4_dn9 = assign27630_e41711_d_n9;
        locals.var_t4_dn10 = assign27630_e41711_d_n10;
        locals.var_t4_dn11 = assign27630_e41711_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign27670_e41753, assign27670_e41753_d_n3, assign27670_e41753_d_n4, assign27670_e41753_d_n5, assign27670_e41753_d_n6, assign27670_e41753_d_n7, assign27670_e41753_d_n8, assign27670_e41753_d_n9, assign27670_e41753_d_n10, assign27670_e41753_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign27670_e41751: f64 = (locals.var_cjs_t * locals.var_aseff);
        (assign27670_e41751, (locals.var_cjs_t * locals.var_aseff_dn3), ((locals.var_cjs_t_dn4 * locals.var_aseff) + (locals.var_cjs_t * locals.var_aseff_dn4)), ((locals.var_cjs_t_dn5 * locals.var_aseff) + (locals.var_cjs_t * locals.var_aseff_dn5)), (locals.var_cjs_t * locals.var_aseff_dn6), (locals.var_cjs_t * locals.var_aseff_dn7), (locals.var_cjs_t * locals.var_aseff_dn8), (locals.var_cjs_t * locals.var_aseff_dn9), (locals.var_cjs_t * locals.var_aseff_dn10), (locals.var_cjs_t * locals.var_aseff_dn11),)
    } else {
        (locals.var_czbs, locals.var_czbs_dn3, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11,)
    }
};
        locals.var_czbs = assign27670_e41753;
        locals.var_czbs_dn3 = assign27670_e41753_d_n3;
        locals.var_czbs_dn4 = assign27670_e41753_d_n4;
        locals.var_czbs_dn5 = assign27670_e41753_d_n5;
        locals.var_czbs_dn6 = assign27670_e41753_d_n6;
        locals.var_czbs_dn7 = assign27670_e41753_d_n7;
        locals.var_czbs_dn8 = assign27670_e41753_d_n8;
        locals.var_czbs_dn9 = assign27670_e41753_d_n9;
        locals.var_czbs_dn10 = assign27670_e41753_d_n10;
        locals.var_czbs_dn11 = assign27670_e41753_d_n11;
        locals.var_czbs_rv = 0.0;

        let (assign27680_e41759, assign27680_e41759_d_n3, assign27680_e41759_d_n4, assign27680_e41759_d_n5, assign27680_e41759_d_n6, assign27680_e41759_d_n7, assign27680_e41759_d_n8, assign27680_e41759_d_n9, assign27680_e41759_d_n10, assign27680_e41759_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign27680_e41757: f64 = (locals.var_cjsws_t * locals.var_pseff);
        (assign27680_e41757, (locals.var_cjsws_t * locals.var_pseff_dn3), ((locals.var_cjsws_t_dn4 * locals.var_pseff) + (locals.var_cjsws_t * locals.var_pseff_dn4)), ((locals.var_cjsws_t_dn5 * locals.var_pseff) + (locals.var_cjsws_t * locals.var_pseff_dn5)), (locals.var_cjsws_t * locals.var_pseff_dn6), (locals.var_cjsws_t * locals.var_pseff_dn7), (locals.var_cjsws_t * locals.var_pseff_dn8), (locals.var_cjsws_t * locals.var_pseff_dn9), (locals.var_cjsws_t * locals.var_pseff_dn10), (locals.var_cjsws_t * locals.var_pseff_dn11),)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn3, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11,)
    }
};
        locals.var_czbssw = assign27680_e41759;
        locals.var_czbssw_dn3 = assign27680_e41759_d_n3;
        locals.var_czbssw_dn4 = assign27680_e41759_d_n4;
        locals.var_czbssw_dn5 = assign27680_e41759_d_n5;
        locals.var_czbssw_dn6 = assign27680_e41759_d_n6;
        locals.var_czbssw_dn7 = assign27680_e41759_d_n7;
        locals.var_czbssw_dn8 = assign27680_e41759_d_n8;
        locals.var_czbssw_dn9 = assign27680_e41759_d_n9;
        locals.var_czbssw_dn10 = assign27680_e41759_d_n10;
        locals.var_czbssw_dn11 = assign27680_e41759_d_n11;
        locals.var_czbssw_rv = 0.0;

        let (assign27690_e41767, assign27690_e41767_d_n4, assign27690_e41767_d_n5,) = {
    if (locals.var_guard492 != 0.0) {
        let assign27690_e41763: f64 = (locals.var_cjswgs_t * locals.var_weffcj);
        let assign27690_e41765: f64 = (assign27690_e41763 * p.p2);
        (assign27690_e41765, ((locals.var_cjswgs_t_dn4 * locals.var_weffcj) * p.p2), ((locals.var_cjswgs_t_dn5 * locals.var_weffcj) * p.p2),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5,)
    }
};
        locals.var_czbsswg = assign27690_e41767;
        locals.var_czbsswg_dn4 = assign27690_e41767_d_n4;
        locals.var_czbsswg_dn5 = assign27690_e41767_d_n5;
        locals.var_czbsswg_rv = 0.0;

        let (assign27700_e41774,) = {
    if (locals.var_guard492 != 0.0) {
        let assign27700_e41771: f64 = (-p.p913);
        let assign27700_e41772: f64 = (0.1_f64).powf(assign27700_e41771);
        (assign27700_e41772,)
    } else {
        (locals.var_czbs_p1,)
    }
};
        locals.var_czbs_p1 = assign27700_e41774;
        locals.var_czbs_p1_rv = 0.0;

        let assign27710_e41777: f64 = if p.p913 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard574 = assign27710_e41777;
        locals.var_guard574_rv = 0.0;

        let (assign27720_e41786,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard574 != 0.0)) {
        let assign27720_e41783: f64 = (0.1_f64).ln();
        let assign27720_e41784: f64 = (1.5 - assign27720_e41783);
        (assign27720_e41784,)
    } else {
        (locals.var_czbs_p2,)
    }
};
        locals.var_czbs_p2 = assign27720_e41786;
        locals.var_czbs_p2_rv = 0.0;

        let (assign27730_e41809,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard574 == 0.0)) {
        let assign27730_e41794: f64 = (1.0 - p.p913);
        let assign27730_e41795: f64 = (1.0 / assign27730_e41794);
        let assign27730_e41799: f64 = (0.05 * p.p913);
        let assign27730_e41802: f64 = (1.0 + p.p913);
        let assign27730_e41803: f64 = (assign27730_e41799 * assign27730_e41802);
        let assign27730_e41805: f64 = (assign27730_e41803 * locals.var_czbs_p1);
        let assign27730_e41806: f64 = (1.0 - assign27730_e41805);
        let assign27730_e41807: f64 = (assign27730_e41795 * assign27730_e41806);
        (assign27730_e41807,)
    } else {
        (locals.var_czbs_p2,)
    }
};
        locals.var_czbs_p2 = assign27730_e41809;
        locals.var_czbs_p2_rv = 0.0;

        let (assign27740_e41816,) = {
    if (locals.var_guard492 != 0.0) {
        let assign27740_e41813: f64 = (-p.p915);
        let assign27740_e41814: f64 = (0.1_f64).powf(assign27740_e41813);
        (assign27740_e41814,)
    } else {
        (locals.var_czbssw_p1,)
    }
};
        locals.var_czbssw_p1 = assign27740_e41816;
        locals.var_czbssw_p1_rv = 0.0;

        let assign27750_e41819: f64 = if p.p915 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard575 = assign27750_e41819;
        locals.var_guard575_rv = 0.0;

        let (assign27760_e41828,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard575 != 0.0)) {
        let assign27760_e41825: f64 = (0.1_f64).ln();
        let assign27760_e41826: f64 = (1.5 - assign27760_e41825);
        (assign27760_e41826,)
    } else {
        (locals.var_czbssw_p2,)
    }
};
        locals.var_czbssw_p2 = assign27760_e41828;
        locals.var_czbssw_p2_rv = 0.0;

        let (assign27770_e41851,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard575 == 0.0)) {
        let assign27770_e41836: f64 = (1.0 - p.p915);
        let assign27770_e41837: f64 = (1.0 / assign27770_e41836);
        let assign27770_e41841: f64 = (0.05 * p.p915);
        let assign27770_e41844: f64 = (1.0 + p.p915);
        let assign27770_e41845: f64 = (assign27770_e41841 * assign27770_e41844);
        let assign27770_e41847: f64 = (assign27770_e41845 * locals.var_czbssw_p1);
        let assign27770_e41848: f64 = (1.0 - assign27770_e41847);
        let assign27770_e41849: f64 = (assign27770_e41837 * assign27770_e41848);
        (assign27770_e41849,)
    } else {
        (locals.var_czbssw_p2,)
    }
};
        locals.var_czbssw_p2 = assign27770_e41851;
        locals.var_czbssw_p2_rv = 0.0;

        let (assign27780_e41858,) = {
    if (locals.var_guard492 != 0.0) {
        let assign27780_e41855: f64 = (-p.p917);
        let assign27780_e41856: f64 = (0.1_f64).powf(assign27780_e41855);
        (assign27780_e41856,)
    } else {
        (locals.var_czbsswg_p1,)
    }
};
        locals.var_czbsswg_p1 = assign27780_e41858;
        locals.var_czbsswg_p1_rv = 0.0;

        let assign27790_e41861: f64 = if p.p917 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard576 = assign27790_e41861;
        locals.var_guard576_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_79(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27800_e41870,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard576 != 0.0)) {
        let assign27800_e41867: f64 = (0.1_f64).ln();
        let assign27800_e41868: f64 = (1.5 - assign27800_e41867);
        (assign27800_e41868,)
    } else {
        (locals.var_czbsswg_p2,)
    }
};
        locals.var_czbsswg_p2 = assign27800_e41870;
        locals.var_czbsswg_p2_rv = 0.0;

        let (assign27810_e41893,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard576 == 0.0)) {
        let assign27810_e41878: f64 = (1.0 - p.p917);
        let assign27810_e41879: f64 = (1.0 / assign27810_e41878);
        let assign27810_e41883: f64 = (0.05 * p.p917);
        let assign27810_e41886: f64 = (1.0 + p.p917);
        let assign27810_e41887: f64 = (assign27810_e41883 * assign27810_e41886);
        let assign27810_e41889: f64 = (assign27810_e41887 * locals.var_czbsswg_p1);
        let assign27810_e41890: f64 = (1.0 - assign27810_e41889);
        let assign27810_e41891: f64 = (assign27810_e41879 * assign27810_e41890);
        (assign27810_e41891,)
    } else {
        (locals.var_czbsswg_p2,)
    }
};
        locals.var_czbsswg_p2 = assign27810_e41893;
        locals.var_czbsswg_p2_rv = 0.0;

        let assign27820_e41896: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard577 = assign27820_e41896;
        locals.var_guard577_rv = 0.0;

        let (assign27830_e41904, assign27830_e41904_d_n3, assign27830_e41904_d_n4, assign27830_e41904_d_n5, assign27830_e41904_d_n6, assign27830_e41904_d_n7, assign27830_e41904_d_n8, assign27830_e41904_d_n9, assign27830_e41904_d_n10, assign27830_e41904_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard577 != 0.0)) {
        let assign27830_e41902: f64 = (locals.var_vbs_jct / locals.var_pbs_t);
        (assign27830_e41902, 0.0, (-((locals.var_vbs_jct * locals.var_pbs_t_dn4) / (locals.var_pbs_t * locals.var_pbs_t))), (-((locals.var_vbs_jct * locals.var_pbs_t_dn5) / (locals.var_pbs_t * locals.var_pbs_t))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_pbs_t), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_pbs_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign27830_e41904;
        locals.var_t1_dn3 = assign27830_e41904_d_n3;
        locals.var_t1_dn4 = assign27830_e41904_d_n4;
        locals.var_t1_dn5 = assign27830_e41904_d_n5;
        locals.var_t1_dn6 = assign27830_e41904_d_n6;
        locals.var_t1_dn7 = assign27830_e41904_d_n7;
        locals.var_t1_dn8 = assign27830_e41904_d_n8;
        locals.var_t1_dn9 = assign27830_e41904_d_n9;
        locals.var_t1_dn10 = assign27830_e41904_d_n10;
        locals.var_t1_dn11 = assign27830_e41904_d_n11;
        locals.var_t1_rv = 0.0;

        let assign27840_e41907: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard578 = assign27840_e41907;
        locals.var_guard578_rv = 0.0;

        let (assign27850_e41917, assign27850_e41917_d_n3, assign27850_e41917_d_n4, assign27850_e41917_d_n5, assign27850_e41917_d_n6, assign27850_e41917_d_n7, assign27850_e41917_d_n8, assign27850_e41917_d_n9, assign27850_e41917_d_n10, assign27850_e41917_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard577 != 0.0)) && (locals.var_guard578 != 0.0)) {
        let assign27850_e41915: f64 = (1.0 - locals.var_t1);
        (assign27850_e41915, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign27850_e41917;
        locals.var_arg_dn3 = assign27850_e41917_d_n3;
        locals.var_arg_dn4 = assign27850_e41917_d_n4;
        locals.var_arg_dn5 = assign27850_e41917_d_n5;
        locals.var_arg_dn6 = assign27850_e41917_d_n6;
        locals.var_arg_dn7 = assign27850_e41917_d_n7;
        locals.var_arg_dn8 = assign27850_e41917_d_n8;
        locals.var_arg_dn9 = assign27850_e41917_d_n9;
        locals.var_arg_dn10 = assign27850_e41917_d_n10;
        locals.var_arg_dn11 = assign27850_e41917_d_n11;
        locals.var_arg_rv = 0.0;

        let assign27860_e41920: f64 = if p.p913 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard579 = assign27860_e41920;
        locals.var_guard579_rv = 0.0;

        let assign27870_e41923: f64 = if p.p913 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard580 = assign27870_e41923;
        locals.var_guard580_rv = 0.0;

        let (assign27880_e41938, assign27880_e41938_d_n3, assign27880_e41938_d_n4, assign27880_e41938_d_n5, assign27880_e41938_d_n6, assign27880_e41938_d_n7, assign27880_e41938_d_n8, assign27880_e41938_d_n9, assign27880_e41938_d_n10, assign27880_e41938_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard577 != 0.0)) && (locals.var_guard578 != 0.0)) && (locals.var_guard579 != 0.0)) && (locals.var_guard580 != 0.0)) {
        let assign27880_e41935: f64 = (locals.var_arg).sqrt();
        let assign27880_e41936: f64 = (1.0 / assign27880_e41935);
        (assign27880_e41936, (-((locals.var_arg_dn3 / (2.0 * assign27880_e41935)) / (assign27880_e41935 * assign27880_e41935))), (-((locals.var_arg_dn4 / (2.0 * assign27880_e41935)) / (assign27880_e41935 * assign27880_e41935))), (-((locals.var_arg_dn5 / (2.0 * assign27880_e41935)) / (assign27880_e41935 * assign27880_e41935))), (-((locals.var_arg_dn6 / (2.0 * assign27880_e41935)) / (assign27880_e41935 * assign27880_e41935))), (-((locals.var_arg_dn7 / (2.0 * assign27880_e41935)) / (assign27880_e41935 * assign27880_e41935))), (-((locals.var_arg_dn8 / (2.0 * assign27880_e41935)) / (assign27880_e41935 * assign27880_e41935))), (-((locals.var_arg_dn9 / (2.0 * assign27880_e41935)) / (assign27880_e41935 * assign27880_e41935))), (-((locals.var_arg_dn10 / (2.0 * assign27880_e41935)) / (assign27880_e41935 * assign27880_e41935))), (-((locals.var_arg_dn11 / (2.0 * assign27880_e41935)) / (assign27880_e41935 * assign27880_e41935))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign27880_e41938;
        locals.var_sarg_dn3 = assign27880_e41938_d_n3;
        locals.var_sarg_dn4 = assign27880_e41938_d_n4;
        locals.var_sarg_dn5 = assign27880_e41938_d_n5;
        locals.var_sarg_dn6 = assign27880_e41938_d_n6;
        locals.var_sarg_dn7 = assign27880_e41938_d_n7;
        locals.var_sarg_dn8 = assign27880_e41938_d_n8;
        locals.var_sarg_dn9 = assign27880_e41938_d_n9;
        locals.var_sarg_dn10 = assign27880_e41938_d_n10;
        locals.var_sarg_dn11 = assign27880_e41938_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign27890_e41956, assign27890_e41956_d_n3, assign27890_e41956_d_n4, assign27890_e41956_d_n5, assign27890_e41956_d_n6, assign27890_e41956_d_n7, assign27890_e41956_d_n8, assign27890_e41956_d_n9, assign27890_e41956_d_n10, assign27890_e41956_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard577 != 0.0)) && (locals.var_guard578 != 0.0)) && (locals.var_guard579 != 0.0)) && (locals.var_guard580 == 0.0)) {
        let assign27890_e41950: f64 = (-p.p913);
        let assign27890_e41952: f64 = (locals.var_arg).ln();
        let assign27890_e41953: f64 = (assign27890_e41950 * assign27890_e41952);
        let assign27890_e41954: f64 = { let limited_exp_arg = assign27890_e41953; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign27890_e41954, ({ let limited_exp_arg = assign27890_e41953; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27890_e41950 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign27890_e41953; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27890_e41950 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign27890_e41953; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27890_e41950 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign27890_e41953; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27890_e41950 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign27890_e41953; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27890_e41950 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign27890_e41953; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27890_e41950 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign27890_e41953; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27890_e41950 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign27890_e41953; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27890_e41950 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign27890_e41953; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27890_e41950 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign27890_e41956;
        locals.var_sarg_dn3 = assign27890_e41956_d_n3;
        locals.var_sarg_dn4 = assign27890_e41956_d_n4;
        locals.var_sarg_dn5 = assign27890_e41956_d_n5;
        locals.var_sarg_dn6 = assign27890_e41956_d_n6;
        locals.var_sarg_dn7 = assign27890_e41956_d_n7;
        locals.var_sarg_dn8 = assign27890_e41956_d_n8;
        locals.var_sarg_dn9 = assign27890_e41956_d_n9;
        locals.var_sarg_dn10 = assign27890_e41956_d_n10;
        locals.var_sarg_dn11 = assign27890_e41956_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign27900_e41978, assign27900_e41978_d_n3, assign27900_e41978_d_n4, assign27900_e41978_d_n5, assign27900_e41978_d_n6, assign27900_e41978_d_n7, assign27900_e41978_d_n8, assign27900_e41978_d_n9, assign27900_e41978_d_n10, assign27900_e41978_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard577 != 0.0)) && (locals.var_guard578 != 0.0)) && (locals.var_guard579 != 0.0)) {
        let assign27900_e41966: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign27900_e41970: f64 = (locals.var_arg * locals.var_sarg);
        let assign27900_e41971: f64 = (1.0 - assign27900_e41970);
        let assign27900_e41972: f64 = (assign27900_e41966 * assign27900_e41971);
        let assign27900_e41975: f64 = (1.0 - p.p913);
        let assign27900_e41976: f64 = (assign27900_e41972 / assign27900_e41975);
        (assign27900_e41976, ((((locals.var_pbs_t * locals.var_czbs_dn3) * assign27900_e41971) + (assign27900_e41966 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign27900_e41975), (((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign27900_e41971) + (assign27900_e41966 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign27900_e41975), (((((locals.var_pbs_t_dn5 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn5)) * assign27900_e41971) + (assign27900_e41966 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign27900_e41975), ((((locals.var_pbs_t * locals.var_czbs_dn6) * assign27900_e41971) + (assign27900_e41966 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign27900_e41975), ((((locals.var_pbs_t * locals.var_czbs_dn7) * assign27900_e41971) + (assign27900_e41966 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign27900_e41975), ((((locals.var_pbs_t * locals.var_czbs_dn8) * assign27900_e41971) + (assign27900_e41966 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign27900_e41975), ((((locals.var_pbs_t * locals.var_czbs_dn9) * assign27900_e41971) + (assign27900_e41966 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign27900_e41975), ((((locals.var_pbs_t * locals.var_czbs_dn10) * assign27900_e41971) + (assign27900_e41966 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign27900_e41975), ((((locals.var_pbs_t * locals.var_czbs_dn11) * assign27900_e41971) + (assign27900_e41966 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign27900_e41975),)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11,)
    }
};
        locals.var_qbsj1 = assign27900_e41978;
        locals.var_qbsj1_dn3 = assign27900_e41978_d_n3;
        locals.var_qbsj1_dn4 = assign27900_e41978_d_n4;
        locals.var_qbsj1_dn5 = assign27900_e41978_d_n5;
        locals.var_qbsj1_dn6 = assign27900_e41978_d_n6;
        locals.var_qbsj1_dn7 = assign27900_e41978_d_n7;
        locals.var_qbsj1_dn8 = assign27900_e41978_d_n8;
        locals.var_qbsj1_dn9 = assign27900_e41978_d_n9;
        locals.var_qbsj1_dn10 = assign27900_e41978_d_n10;
        locals.var_qbsj1_dn11 = assign27900_e41978_d_n11;
        locals.var_qbsj1_rv = 0.0;

        let (assign27910_e41995, assign27910_e41995_d_n3, assign27910_e41995_d_n4, assign27910_e41995_d_n5, assign27910_e41995_d_n6, assign27910_e41995_d_n7, assign27910_e41995_d_n8, assign27910_e41995_d_n9, assign27910_e41995_d_n10, assign27910_e41995_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard577 != 0.0)) && (locals.var_guard578 != 0.0)) && (locals.var_guard579 == 0.0)) {
        let assign27910_e41989: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign27910_e41991: f64 = (locals.var_arg).ln();
        let assign27910_e41992: f64 = (-assign27910_e41991);
        let assign27910_e41993: f64 = (assign27910_e41989 * assign27910_e41992);
        (assign27910_e41993, (((locals.var_pbs_t * locals.var_czbs_dn3) * assign27910_e41992) + (assign27910_e41989 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign27910_e41992) + (assign27910_e41989 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbs_t_dn5 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn5)) * assign27910_e41992) + (assign27910_e41989 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn6) * assign27910_e41992) + (assign27910_e41989 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn7) * assign27910_e41992) + (assign27910_e41989 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn8) * assign27910_e41992) + (assign27910_e41989 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn9) * assign27910_e41992) + (assign27910_e41989 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn10) * assign27910_e41992) + (assign27910_e41989 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn11) * assign27910_e41992) + (assign27910_e41989 * (-(locals.var_arg_dn11 / locals.var_arg)))),)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11,)
    }
};
        locals.var_qbsj1 = assign27910_e41995;
        locals.var_qbsj1_dn3 = assign27910_e41995_d_n3;
        locals.var_qbsj1_dn4 = assign27910_e41995_d_n4;
        locals.var_qbsj1_dn5 = assign27910_e41995_d_n5;
        locals.var_qbsj1_dn6 = assign27910_e41995_d_n6;
        locals.var_qbsj1_dn7 = assign27910_e41995_d_n7;
        locals.var_qbsj1_dn8 = assign27910_e41995_d_n8;
        locals.var_qbsj1_dn9 = assign27910_e41995_d_n9;
        locals.var_qbsj1_dn10 = assign27910_e41995_d_n10;
        locals.var_qbsj1_dn11 = assign27910_e41995_d_n11;
        locals.var_qbsj1_rv = 0.0;

        let (assign27920_e42020, assign27920_e42020_d_n3, assign27920_e42020_d_n4, assign27920_e42020_d_n5, assign27920_e42020_d_n6, assign27920_e42020_d_n7, assign27920_e42020_d_n8, assign27920_e42020_d_n9, assign27920_e42020_d_n10, assign27920_e42020_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard577 != 0.0)) && (locals.var_guard578 == 0.0)) {
        let assign27920_e42005: f64 = (locals.var_t1 - 1.0);
        let assign27920_e42006: f64 = (locals.var_czbs_p1 * assign27920_e42005);
        let assign27920_e42009: f64 = (5.0 * p.p913);
        let assign27920_e42012: f64 = (locals.var_t1 - 1.0);
        let assign27920_e42013: f64 = (assign27920_e42009 * assign27920_e42012);
        let assign27920_e42016: f64 = (1.0 + p.p913);
        let assign27920_e42017: f64 = (assign27920_e42013 + assign27920_e42016);
        let assign27920_e42018: f64 = (assign27920_e42006 * assign27920_e42017);
        (assign27920_e42018, (((locals.var_czbs_p1 * locals.var_t1_dn3) * assign27920_e42017) + (assign27920_e42006 * (assign27920_e42009 * locals.var_t1_dn3))), (((locals.var_czbs_p1 * locals.var_t1_dn4) * assign27920_e42017) + (assign27920_e42006 * (assign27920_e42009 * locals.var_t1_dn4))), (((locals.var_czbs_p1 * locals.var_t1_dn5) * assign27920_e42017) + (assign27920_e42006 * (assign27920_e42009 * locals.var_t1_dn5))), (((locals.var_czbs_p1 * locals.var_t1_dn6) * assign27920_e42017) + (assign27920_e42006 * (assign27920_e42009 * locals.var_t1_dn6))), (((locals.var_czbs_p1 * locals.var_t1_dn7) * assign27920_e42017) + (assign27920_e42006 * (assign27920_e42009 * locals.var_t1_dn7))), (((locals.var_czbs_p1 * locals.var_t1_dn8) * assign27920_e42017) + (assign27920_e42006 * (assign27920_e42009 * locals.var_t1_dn8))), (((locals.var_czbs_p1 * locals.var_t1_dn9) * assign27920_e42017) + (assign27920_e42006 * (assign27920_e42009 * locals.var_t1_dn9))), (((locals.var_czbs_p1 * locals.var_t1_dn10) * assign27920_e42017) + (assign27920_e42006 * (assign27920_e42009 * locals.var_t1_dn10))), (((locals.var_czbs_p1 * locals.var_t1_dn11) * assign27920_e42017) + (assign27920_e42006 * (assign27920_e42009 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign27920_e42020;
        locals.var_t2_dn3 = assign27920_e42020_d_n3;
        locals.var_t2_dn4 = assign27920_e42020_d_n4;
        locals.var_t2_dn5 = assign27920_e42020_d_n5;
        locals.var_t2_dn6 = assign27920_e42020_d_n6;
        locals.var_t2_dn7 = assign27920_e42020_d_n7;
        locals.var_t2_dn8 = assign27920_e42020_d_n8;
        locals.var_t2_dn9 = assign27920_e42020_d_n9;
        locals.var_t2_dn10 = assign27920_e42020_d_n10;
        locals.var_t2_dn11 = assign27920_e42020_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign27930_e42035, assign27930_e42035_d_n3, assign27930_e42035_d_n4, assign27930_e42035_d_n5, assign27930_e42035_d_n6, assign27930_e42035_d_n7, assign27930_e42035_d_n8, assign27930_e42035_d_n9, assign27930_e42035_d_n10, assign27930_e42035_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard577 != 0.0)) && (locals.var_guard578 == 0.0)) {
        let assign27930_e42029: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign27930_e42032: f64 = (locals.var_t2 + locals.var_czbs_p2);
        let assign27930_e42033: f64 = (assign27930_e42029 * assign27930_e42032);
        (assign27930_e42033, (((locals.var_pbs_t * locals.var_czbs_dn3) * assign27930_e42032) + (assign27930_e42029 * locals.var_t2_dn3)), ((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign27930_e42032) + (assign27930_e42029 * locals.var_t2_dn4)), ((((locals.var_pbs_t_dn5 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn5)) * assign27930_e42032) + (assign27930_e42029 * locals.var_t2_dn5)), (((locals.var_pbs_t * locals.var_czbs_dn6) * assign27930_e42032) + (assign27930_e42029 * locals.var_t2_dn6)), (((locals.var_pbs_t * locals.var_czbs_dn7) * assign27930_e42032) + (assign27930_e42029 * locals.var_t2_dn7)), (((locals.var_pbs_t * locals.var_czbs_dn8) * assign27930_e42032) + (assign27930_e42029 * locals.var_t2_dn8)), (((locals.var_pbs_t * locals.var_czbs_dn9) * assign27930_e42032) + (assign27930_e42029 * locals.var_t2_dn9)), (((locals.var_pbs_t * locals.var_czbs_dn10) * assign27930_e42032) + (assign27930_e42029 * locals.var_t2_dn10)), (((locals.var_pbs_t * locals.var_czbs_dn11) * assign27930_e42032) + (assign27930_e42029 * locals.var_t2_dn11)),)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11,)
    }
};
        locals.var_qbsj1 = assign27930_e42035;
        locals.var_qbsj1_dn3 = assign27930_e42035_d_n3;
        locals.var_qbsj1_dn4 = assign27930_e42035_d_n4;
        locals.var_qbsj1_dn5 = assign27930_e42035_d_n5;
        locals.var_qbsj1_dn6 = assign27930_e42035_d_n6;
        locals.var_qbsj1_dn7 = assign27930_e42035_d_n7;
        locals.var_qbsj1_dn8 = assign27930_e42035_d_n8;
        locals.var_qbsj1_dn9 = assign27930_e42035_d_n9;
        locals.var_qbsj1_dn10 = assign27930_e42035_d_n10;
        locals.var_qbsj1_dn11 = assign27930_e42035_d_n11;
        locals.var_qbsj1_rv = 0.0;

        let (assign27940_e42042, assign27940_e42042_d_n3, assign27940_e42042_d_n4, assign27940_e42042_d_n5, assign27940_e42042_d_n6, assign27940_e42042_d_n7, assign27940_e42042_d_n8, assign27940_e42042_d_n9, assign27940_e42042_d_n10, assign27940_e42042_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard577 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11,)
    }
};
        locals.var_qbsj1 = assign27940_e42042;
        locals.var_qbsj1_dn3 = assign27940_e42042_d_n3;
        locals.var_qbsj1_dn4 = assign27940_e42042_d_n4;
        locals.var_qbsj1_dn5 = assign27940_e42042_d_n5;
        locals.var_qbsj1_dn6 = assign27940_e42042_d_n6;
        locals.var_qbsj1_dn7 = assign27940_e42042_d_n7;
        locals.var_qbsj1_dn8 = assign27940_e42042_d_n8;
        locals.var_qbsj1_dn9 = assign27940_e42042_d_n9;
        locals.var_qbsj1_dn10 = assign27940_e42042_d_n10;
        locals.var_qbsj1_dn11 = assign27940_e42042_d_n11;
        locals.var_qbsj1_rv = 0.0;

        let assign27950_e42045: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard581 = assign27950_e42045;
        locals.var_guard581_rv = 0.0;

        let (assign27960_e42053, assign27960_e42053_d_n3, assign27960_e42053_d_n4, assign27960_e42053_d_n5, assign27960_e42053_d_n6, assign27960_e42053_d_n7, assign27960_e42053_d_n8, assign27960_e42053_d_n9, assign27960_e42053_d_n10, assign27960_e42053_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard581 != 0.0)) {
        let assign27960_e42051: f64 = (locals.var_vbs_jct / locals.var_pbsws_t);
        (assign27960_e42051, 0.0, (-((locals.var_vbs_jct * locals.var_pbsws_t_dn4) / (locals.var_pbsws_t * locals.var_pbsws_t))), (-((locals.var_vbs_jct * locals.var_pbsws_t_dn5) / (locals.var_pbsws_t * locals.var_pbsws_t))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_pbsws_t), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_pbsws_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign27960_e42053;
        locals.var_t1_dn3 = assign27960_e42053_d_n3;
        locals.var_t1_dn4 = assign27960_e42053_d_n4;
        locals.var_t1_dn5 = assign27960_e42053_d_n5;
        locals.var_t1_dn6 = assign27960_e42053_d_n6;
        locals.var_t1_dn7 = assign27960_e42053_d_n7;
        locals.var_t1_dn8 = assign27960_e42053_d_n8;
        locals.var_t1_dn9 = assign27960_e42053_d_n9;
        locals.var_t1_dn10 = assign27960_e42053_d_n10;
        locals.var_t1_dn11 = assign27960_e42053_d_n11;
        locals.var_t1_rv = 0.0;

        let assign27970_e42056: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard582 = assign27970_e42056;
        locals.var_guard582_rv = 0.0;

        let (assign27980_e42066, assign27980_e42066_d_n3, assign27980_e42066_d_n4, assign27980_e42066_d_n5, assign27980_e42066_d_n6, assign27980_e42066_d_n7, assign27980_e42066_d_n8, assign27980_e42066_d_n9, assign27980_e42066_d_n10, assign27980_e42066_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard581 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign27980_e42064: f64 = (1.0 - locals.var_t1);
        (assign27980_e42064, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign27980_e42066;
        locals.var_arg_dn3 = assign27980_e42066_d_n3;
        locals.var_arg_dn4 = assign27980_e42066_d_n4;
        locals.var_arg_dn5 = assign27980_e42066_d_n5;
        locals.var_arg_dn6 = assign27980_e42066_d_n6;
        locals.var_arg_dn7 = assign27980_e42066_d_n7;
        locals.var_arg_dn8 = assign27980_e42066_d_n8;
        locals.var_arg_dn9 = assign27980_e42066_d_n9;
        locals.var_arg_dn10 = assign27980_e42066_d_n10;
        locals.var_arg_dn11 = assign27980_e42066_d_n11;
        locals.var_arg_rv = 0.0;

        let assign27990_e42069: f64 = if p.p915 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard583 = assign27990_e42069;
        locals.var_guard583_rv = 0.0;

        let assign28000_e42072: f64 = if p.p915 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard584 = assign28000_e42072;
        locals.var_guard584_rv = 0.0;

        let (assign28010_e42087, assign28010_e42087_d_n3, assign28010_e42087_d_n4, assign28010_e42087_d_n5, assign28010_e42087_d_n6, assign28010_e42087_d_n7, assign28010_e42087_d_n8, assign28010_e42087_d_n9, assign28010_e42087_d_n10, assign28010_e42087_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard581 != 0.0)) && (locals.var_guard582 != 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard584 != 0.0)) {
        let assign28010_e42084: f64 = (locals.var_arg).sqrt();
        let assign28010_e42085: f64 = (1.0 / assign28010_e42084);
        (assign28010_e42085, (-((locals.var_arg_dn3 / (2.0 * assign28010_e42084)) / (assign28010_e42084 * assign28010_e42084))), (-((locals.var_arg_dn4 / (2.0 * assign28010_e42084)) / (assign28010_e42084 * assign28010_e42084))), (-((locals.var_arg_dn5 / (2.0 * assign28010_e42084)) / (assign28010_e42084 * assign28010_e42084))), (-((locals.var_arg_dn6 / (2.0 * assign28010_e42084)) / (assign28010_e42084 * assign28010_e42084))), (-((locals.var_arg_dn7 / (2.0 * assign28010_e42084)) / (assign28010_e42084 * assign28010_e42084))), (-((locals.var_arg_dn8 / (2.0 * assign28010_e42084)) / (assign28010_e42084 * assign28010_e42084))), (-((locals.var_arg_dn9 / (2.0 * assign28010_e42084)) / (assign28010_e42084 * assign28010_e42084))), (-((locals.var_arg_dn10 / (2.0 * assign28010_e42084)) / (assign28010_e42084 * assign28010_e42084))), (-((locals.var_arg_dn11 / (2.0 * assign28010_e42084)) / (assign28010_e42084 * assign28010_e42084))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign28010_e42087;
        locals.var_sarg_dn3 = assign28010_e42087_d_n3;
        locals.var_sarg_dn4 = assign28010_e42087_d_n4;
        locals.var_sarg_dn5 = assign28010_e42087_d_n5;
        locals.var_sarg_dn6 = assign28010_e42087_d_n6;
        locals.var_sarg_dn7 = assign28010_e42087_d_n7;
        locals.var_sarg_dn8 = assign28010_e42087_d_n8;
        locals.var_sarg_dn9 = assign28010_e42087_d_n9;
        locals.var_sarg_dn10 = assign28010_e42087_d_n10;
        locals.var_sarg_dn11 = assign28010_e42087_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign28020_e42105, assign28020_e42105_d_n3, assign28020_e42105_d_n4, assign28020_e42105_d_n5, assign28020_e42105_d_n6, assign28020_e42105_d_n7, assign28020_e42105_d_n8, assign28020_e42105_d_n9, assign28020_e42105_d_n10, assign28020_e42105_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard581 != 0.0)) && (locals.var_guard582 != 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard584 == 0.0)) {
        let assign28020_e42099: f64 = (-p.p915);
        let assign28020_e42101: f64 = (locals.var_arg).ln();
        let assign28020_e42102: f64 = (assign28020_e42099 * assign28020_e42101);
        let assign28020_e42103: f64 = { let limited_exp_arg = assign28020_e42102; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign28020_e42103, ({ let limited_exp_arg = assign28020_e42102; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28020_e42099 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign28020_e42102; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28020_e42099 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign28020_e42102; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28020_e42099 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign28020_e42102; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28020_e42099 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign28020_e42102; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28020_e42099 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign28020_e42102; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28020_e42099 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign28020_e42102; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28020_e42099 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign28020_e42102; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28020_e42099 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign28020_e42102; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28020_e42099 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign28020_e42105;
        locals.var_sarg_dn3 = assign28020_e42105_d_n3;
        locals.var_sarg_dn4 = assign28020_e42105_d_n4;
        locals.var_sarg_dn5 = assign28020_e42105_d_n5;
        locals.var_sarg_dn6 = assign28020_e42105_d_n6;
        locals.var_sarg_dn7 = assign28020_e42105_d_n7;
        locals.var_sarg_dn8 = assign28020_e42105_d_n8;
        locals.var_sarg_dn9 = assign28020_e42105_d_n9;
        locals.var_sarg_dn10 = assign28020_e42105_d_n10;
        locals.var_sarg_dn11 = assign28020_e42105_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign28030_e42127, assign28030_e42127_d_n3, assign28030_e42127_d_n4, assign28030_e42127_d_n5, assign28030_e42127_d_n6, assign28030_e42127_d_n7, assign28030_e42127_d_n8, assign28030_e42127_d_n9, assign28030_e42127_d_n10, assign28030_e42127_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard581 != 0.0)) && (locals.var_guard582 != 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign28030_e42115: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign28030_e42119: f64 = (locals.var_arg * locals.var_sarg);
        let assign28030_e42120: f64 = (1.0 - assign28030_e42119);
        let assign28030_e42121: f64 = (assign28030_e42115 * assign28030_e42120);
        let assign28030_e42124: f64 = (1.0 - p.p915);
        let assign28030_e42125: f64 = (assign28030_e42121 / assign28030_e42124);
        (assign28030_e42125, ((((locals.var_pbsws_t * locals.var_czbssw_dn3) * assign28030_e42120) + (assign28030_e42115 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign28030_e42124), (((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign28030_e42120) + (assign28030_e42115 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign28030_e42124), (((((locals.var_pbsws_t_dn5 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn5)) * assign28030_e42120) + (assign28030_e42115 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign28030_e42124), ((((locals.var_pbsws_t * locals.var_czbssw_dn6) * assign28030_e42120) + (assign28030_e42115 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign28030_e42124), ((((locals.var_pbsws_t * locals.var_czbssw_dn7) * assign28030_e42120) + (assign28030_e42115 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign28030_e42124), ((((locals.var_pbsws_t * locals.var_czbssw_dn8) * assign28030_e42120) + (assign28030_e42115 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign28030_e42124), ((((locals.var_pbsws_t * locals.var_czbssw_dn9) * assign28030_e42120) + (assign28030_e42115 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign28030_e42124), ((((locals.var_pbsws_t * locals.var_czbssw_dn10) * assign28030_e42120) + (assign28030_e42115 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign28030_e42124), ((((locals.var_pbsws_t * locals.var_czbssw_dn11) * assign28030_e42120) + (assign28030_e42115 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign28030_e42124),)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11,)
    }
};
        locals.var_qbsj2 = assign28030_e42127;
        locals.var_qbsj2_dn3 = assign28030_e42127_d_n3;
        locals.var_qbsj2_dn4 = assign28030_e42127_d_n4;
        locals.var_qbsj2_dn5 = assign28030_e42127_d_n5;
        locals.var_qbsj2_dn6 = assign28030_e42127_d_n6;
        locals.var_qbsj2_dn7 = assign28030_e42127_d_n7;
        locals.var_qbsj2_dn8 = assign28030_e42127_d_n8;
        locals.var_qbsj2_dn9 = assign28030_e42127_d_n9;
        locals.var_qbsj2_dn10 = assign28030_e42127_d_n10;
        locals.var_qbsj2_dn11 = assign28030_e42127_d_n11;
        locals.var_qbsj2_rv = 0.0;

        let (assign28040_e42144, assign28040_e42144_d_n3, assign28040_e42144_d_n4, assign28040_e42144_d_n5, assign28040_e42144_d_n6, assign28040_e42144_d_n7, assign28040_e42144_d_n8, assign28040_e42144_d_n9, assign28040_e42144_d_n10, assign28040_e42144_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard581 != 0.0)) && (locals.var_guard582 != 0.0)) && (locals.var_guard583 == 0.0)) {
        let assign28040_e42138: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign28040_e42140: f64 = (locals.var_arg).ln();
        let assign28040_e42141: f64 = (-assign28040_e42140);
        let assign28040_e42142: f64 = (assign28040_e42138 * assign28040_e42141);
        (assign28040_e42142, (((locals.var_pbsws_t * locals.var_czbssw_dn3) * assign28040_e42141) + (assign28040_e42138 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign28040_e42141) + (assign28040_e42138 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbsws_t_dn5 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn5)) * assign28040_e42141) + (assign28040_e42138 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn6) * assign28040_e42141) + (assign28040_e42138 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn7) * assign28040_e42141) + (assign28040_e42138 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn8) * assign28040_e42141) + (assign28040_e42138 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn9) * assign28040_e42141) + (assign28040_e42138 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn10) * assign28040_e42141) + (assign28040_e42138 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn11) * assign28040_e42141) + (assign28040_e42138 * (-(locals.var_arg_dn11 / locals.var_arg)))),)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11,)
    }
};
        locals.var_qbsj2 = assign28040_e42144;
        locals.var_qbsj2_dn3 = assign28040_e42144_d_n3;
        locals.var_qbsj2_dn4 = assign28040_e42144_d_n4;
        locals.var_qbsj2_dn5 = assign28040_e42144_d_n5;
        locals.var_qbsj2_dn6 = assign28040_e42144_d_n6;
        locals.var_qbsj2_dn7 = assign28040_e42144_d_n7;
        locals.var_qbsj2_dn8 = assign28040_e42144_d_n8;
        locals.var_qbsj2_dn9 = assign28040_e42144_d_n9;
        locals.var_qbsj2_dn10 = assign28040_e42144_d_n10;
        locals.var_qbsj2_dn11 = assign28040_e42144_d_n11;
        locals.var_qbsj2_rv = 0.0;

        let (assign28050_e42169, assign28050_e42169_d_n3, assign28050_e42169_d_n4, assign28050_e42169_d_n5, assign28050_e42169_d_n6, assign28050_e42169_d_n7, assign28050_e42169_d_n8, assign28050_e42169_d_n9, assign28050_e42169_d_n10, assign28050_e42169_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard581 != 0.0)) && (locals.var_guard582 == 0.0)) {
        let assign28050_e42154: f64 = (locals.var_t1 - 1.0);
        let assign28050_e42155: f64 = (locals.var_czbssw_p1 * assign28050_e42154);
        let assign28050_e42158: f64 = (5.0 * p.p915);
        let assign28050_e42161: f64 = (locals.var_t1 - 1.0);
        let assign28050_e42162: f64 = (assign28050_e42158 * assign28050_e42161);
        let assign28050_e42165: f64 = (1.0 + p.p915);
        let assign28050_e42166: f64 = (assign28050_e42162 + assign28050_e42165);
        let assign28050_e42167: f64 = (assign28050_e42155 * assign28050_e42166);
        (assign28050_e42167, (((locals.var_czbssw_p1 * locals.var_t1_dn3) * assign28050_e42166) + (assign28050_e42155 * (assign28050_e42158 * locals.var_t1_dn3))), (((locals.var_czbssw_p1 * locals.var_t1_dn4) * assign28050_e42166) + (assign28050_e42155 * (assign28050_e42158 * locals.var_t1_dn4))), (((locals.var_czbssw_p1 * locals.var_t1_dn5) * assign28050_e42166) + (assign28050_e42155 * (assign28050_e42158 * locals.var_t1_dn5))), (((locals.var_czbssw_p1 * locals.var_t1_dn6) * assign28050_e42166) + (assign28050_e42155 * (assign28050_e42158 * locals.var_t1_dn6))), (((locals.var_czbssw_p1 * locals.var_t1_dn7) * assign28050_e42166) + (assign28050_e42155 * (assign28050_e42158 * locals.var_t1_dn7))), (((locals.var_czbssw_p1 * locals.var_t1_dn8) * assign28050_e42166) + (assign28050_e42155 * (assign28050_e42158 * locals.var_t1_dn8))), (((locals.var_czbssw_p1 * locals.var_t1_dn9) * assign28050_e42166) + (assign28050_e42155 * (assign28050_e42158 * locals.var_t1_dn9))), (((locals.var_czbssw_p1 * locals.var_t1_dn10) * assign28050_e42166) + (assign28050_e42155 * (assign28050_e42158 * locals.var_t1_dn10))), (((locals.var_czbssw_p1 * locals.var_t1_dn11) * assign28050_e42166) + (assign28050_e42155 * (assign28050_e42158 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign28050_e42169;
        locals.var_t2_dn3 = assign28050_e42169_d_n3;
        locals.var_t2_dn4 = assign28050_e42169_d_n4;
        locals.var_t2_dn5 = assign28050_e42169_d_n5;
        locals.var_t2_dn6 = assign28050_e42169_d_n6;
        locals.var_t2_dn7 = assign28050_e42169_d_n7;
        locals.var_t2_dn8 = assign28050_e42169_d_n8;
        locals.var_t2_dn9 = assign28050_e42169_d_n9;
        locals.var_t2_dn10 = assign28050_e42169_d_n10;
        locals.var_t2_dn11 = assign28050_e42169_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign28060_e42184, assign28060_e42184_d_n3, assign28060_e42184_d_n4, assign28060_e42184_d_n5, assign28060_e42184_d_n6, assign28060_e42184_d_n7, assign28060_e42184_d_n8, assign28060_e42184_d_n9, assign28060_e42184_d_n10, assign28060_e42184_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard581 != 0.0)) && (locals.var_guard582 == 0.0)) {
        let assign28060_e42178: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign28060_e42181: f64 = (locals.var_t2 + locals.var_czbssw_p2);
        let assign28060_e42182: f64 = (assign28060_e42178 * assign28060_e42181);
        (assign28060_e42182, (((locals.var_pbsws_t * locals.var_czbssw_dn3) * assign28060_e42181) + (assign28060_e42178 * locals.var_t2_dn3)), ((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign28060_e42181) + (assign28060_e42178 * locals.var_t2_dn4)), ((((locals.var_pbsws_t_dn5 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn5)) * assign28060_e42181) + (assign28060_e42178 * locals.var_t2_dn5)), (((locals.var_pbsws_t * locals.var_czbssw_dn6) * assign28060_e42181) + (assign28060_e42178 * locals.var_t2_dn6)), (((locals.var_pbsws_t * locals.var_czbssw_dn7) * assign28060_e42181) + (assign28060_e42178 * locals.var_t2_dn7)), (((locals.var_pbsws_t * locals.var_czbssw_dn8) * assign28060_e42181) + (assign28060_e42178 * locals.var_t2_dn8)), (((locals.var_pbsws_t * locals.var_czbssw_dn9) * assign28060_e42181) + (assign28060_e42178 * locals.var_t2_dn9)), (((locals.var_pbsws_t * locals.var_czbssw_dn10) * assign28060_e42181) + (assign28060_e42178 * locals.var_t2_dn10)), (((locals.var_pbsws_t * locals.var_czbssw_dn11) * assign28060_e42181) + (assign28060_e42178 * locals.var_t2_dn11)),)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11,)
    }
};
        locals.var_qbsj2 = assign28060_e42184;
        locals.var_qbsj2_dn3 = assign28060_e42184_d_n3;
        locals.var_qbsj2_dn4 = assign28060_e42184_d_n4;
        locals.var_qbsj2_dn5 = assign28060_e42184_d_n5;
        locals.var_qbsj2_dn6 = assign28060_e42184_d_n6;
        locals.var_qbsj2_dn7 = assign28060_e42184_d_n7;
        locals.var_qbsj2_dn8 = assign28060_e42184_d_n8;
        locals.var_qbsj2_dn9 = assign28060_e42184_d_n9;
        locals.var_qbsj2_dn10 = assign28060_e42184_d_n10;
        locals.var_qbsj2_dn11 = assign28060_e42184_d_n11;
        locals.var_qbsj2_rv = 0.0;

        let (assign28070_e42191, assign28070_e42191_d_n3, assign28070_e42191_d_n4, assign28070_e42191_d_n5, assign28070_e42191_d_n6, assign28070_e42191_d_n7, assign28070_e42191_d_n8, assign28070_e42191_d_n9, assign28070_e42191_d_n10, assign28070_e42191_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard581 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11,)
    }
};
        locals.var_qbsj2 = assign28070_e42191;
        locals.var_qbsj2_dn3 = assign28070_e42191_d_n3;
        locals.var_qbsj2_dn4 = assign28070_e42191_d_n4;
        locals.var_qbsj2_dn5 = assign28070_e42191_d_n5;
        locals.var_qbsj2_dn6 = assign28070_e42191_d_n6;
        locals.var_qbsj2_dn7 = assign28070_e42191_d_n7;
        locals.var_qbsj2_dn8 = assign28070_e42191_d_n8;
        locals.var_qbsj2_dn9 = assign28070_e42191_d_n9;
        locals.var_qbsj2_dn10 = assign28070_e42191_d_n10;
        locals.var_qbsj2_dn11 = assign28070_e42191_d_n11;
        locals.var_qbsj2_rv = 0.0;

        let assign28080_e42194: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard585 = assign28080_e42194;
        locals.var_guard585_rv = 0.0;

        let (assign28090_e42202, assign28090_e42202_d_n3, assign28090_e42202_d_n4, assign28090_e42202_d_n5, assign28090_e42202_d_n6, assign28090_e42202_d_n7, assign28090_e42202_d_n8, assign28090_e42202_d_n9, assign28090_e42202_d_n10, assign28090_e42202_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard585 != 0.0)) {
        let assign28090_e42200: f64 = (locals.var_vbs_jct / locals.var_pbswgs_t);
        (assign28090_e42200, 0.0, (-((locals.var_vbs_jct * locals.var_pbswgs_t_dn4) / (locals.var_pbswgs_t * locals.var_pbswgs_t))), (-((locals.var_vbs_jct * locals.var_pbswgs_t_dn5) / (locals.var_pbswgs_t * locals.var_pbswgs_t))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_pbswgs_t), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_pbswgs_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign28090_e42202;
        locals.var_t1_dn3 = assign28090_e42202_d_n3;
        locals.var_t1_dn4 = assign28090_e42202_d_n4;
        locals.var_t1_dn5 = assign28090_e42202_d_n5;
        locals.var_t1_dn6 = assign28090_e42202_d_n6;
        locals.var_t1_dn7 = assign28090_e42202_d_n7;
        locals.var_t1_dn8 = assign28090_e42202_d_n8;
        locals.var_t1_dn9 = assign28090_e42202_d_n9;
        locals.var_t1_dn10 = assign28090_e42202_d_n10;
        locals.var_t1_dn11 = assign28090_e42202_d_n11;
        locals.var_t1_rv = 0.0;

        let assign28100_e42205: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard586 = assign28100_e42205;
        locals.var_guard586_rv = 0.0;

        let (assign28110_e42215, assign28110_e42215_d_n3, assign28110_e42215_d_n4, assign28110_e42215_d_n5, assign28110_e42215_d_n6, assign28110_e42215_d_n7, assign28110_e42215_d_n8, assign28110_e42215_d_n9, assign28110_e42215_d_n10, assign28110_e42215_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) {
        let assign28110_e42213: f64 = (1.0 - locals.var_t1);
        (assign28110_e42213, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign28110_e42215;
        locals.var_arg_dn3 = assign28110_e42215_d_n3;
        locals.var_arg_dn4 = assign28110_e42215_d_n4;
        locals.var_arg_dn5 = assign28110_e42215_d_n5;
        locals.var_arg_dn6 = assign28110_e42215_d_n6;
        locals.var_arg_dn7 = assign28110_e42215_d_n7;
        locals.var_arg_dn8 = assign28110_e42215_d_n8;
        locals.var_arg_dn9 = assign28110_e42215_d_n9;
        locals.var_arg_dn10 = assign28110_e42215_d_n10;
        locals.var_arg_dn11 = assign28110_e42215_d_n11;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_80(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign28120_e42218: f64 = if p.p917 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard587 = assign28120_e42218;
        locals.var_guard587_rv = 0.0;

        let assign28130_e42221: f64 = if p.p917 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard588 = assign28130_e42221;
        locals.var_guard588_rv = 0.0;

        let (assign28140_e42236, assign28140_e42236_d_n3, assign28140_e42236_d_n4, assign28140_e42236_d_n5, assign28140_e42236_d_n6, assign28140_e42236_d_n7, assign28140_e42236_d_n8, assign28140_e42236_d_n9, assign28140_e42236_d_n10, assign28140_e42236_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 != 0.0)) && (locals.var_guard588 != 0.0)) {
        let assign28140_e42233: f64 = (locals.var_arg).sqrt();
        let assign28140_e42234: f64 = (1.0 / assign28140_e42233);
        (assign28140_e42234, (-((locals.var_arg_dn3 / (2.0 * assign28140_e42233)) / (assign28140_e42233 * assign28140_e42233))), (-((locals.var_arg_dn4 / (2.0 * assign28140_e42233)) / (assign28140_e42233 * assign28140_e42233))), (-((locals.var_arg_dn5 / (2.0 * assign28140_e42233)) / (assign28140_e42233 * assign28140_e42233))), (-((locals.var_arg_dn6 / (2.0 * assign28140_e42233)) / (assign28140_e42233 * assign28140_e42233))), (-((locals.var_arg_dn7 / (2.0 * assign28140_e42233)) / (assign28140_e42233 * assign28140_e42233))), (-((locals.var_arg_dn8 / (2.0 * assign28140_e42233)) / (assign28140_e42233 * assign28140_e42233))), (-((locals.var_arg_dn9 / (2.0 * assign28140_e42233)) / (assign28140_e42233 * assign28140_e42233))), (-((locals.var_arg_dn10 / (2.0 * assign28140_e42233)) / (assign28140_e42233 * assign28140_e42233))), (-((locals.var_arg_dn11 / (2.0 * assign28140_e42233)) / (assign28140_e42233 * assign28140_e42233))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign28140_e42236;
        locals.var_sarg_dn3 = assign28140_e42236_d_n3;
        locals.var_sarg_dn4 = assign28140_e42236_d_n4;
        locals.var_sarg_dn5 = assign28140_e42236_d_n5;
        locals.var_sarg_dn6 = assign28140_e42236_d_n6;
        locals.var_sarg_dn7 = assign28140_e42236_d_n7;
        locals.var_sarg_dn8 = assign28140_e42236_d_n8;
        locals.var_sarg_dn9 = assign28140_e42236_d_n9;
        locals.var_sarg_dn10 = assign28140_e42236_d_n10;
        locals.var_sarg_dn11 = assign28140_e42236_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign28150_e42254, assign28150_e42254_d_n3, assign28150_e42254_d_n4, assign28150_e42254_d_n5, assign28150_e42254_d_n6, assign28150_e42254_d_n7, assign28150_e42254_d_n8, assign28150_e42254_d_n9, assign28150_e42254_d_n10, assign28150_e42254_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 != 0.0)) && (locals.var_guard588 == 0.0)) {
        let assign28150_e42248: f64 = (-p.p917);
        let assign28150_e42250: f64 = (locals.var_arg).ln();
        let assign28150_e42251: f64 = (assign28150_e42248 * assign28150_e42250);
        let assign28150_e42252: f64 = { let limited_exp_arg = assign28150_e42251; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign28150_e42252, ({ let limited_exp_arg = assign28150_e42251; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28150_e42248 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign28150_e42251; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28150_e42248 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign28150_e42251; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28150_e42248 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign28150_e42251; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28150_e42248 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign28150_e42251; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28150_e42248 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign28150_e42251; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28150_e42248 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign28150_e42251; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28150_e42248 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign28150_e42251; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28150_e42248 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign28150_e42251; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28150_e42248 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign28150_e42254;
        locals.var_sarg_dn3 = assign28150_e42254_d_n3;
        locals.var_sarg_dn4 = assign28150_e42254_d_n4;
        locals.var_sarg_dn5 = assign28150_e42254_d_n5;
        locals.var_sarg_dn6 = assign28150_e42254_d_n6;
        locals.var_sarg_dn7 = assign28150_e42254_d_n7;
        locals.var_sarg_dn8 = assign28150_e42254_d_n8;
        locals.var_sarg_dn9 = assign28150_e42254_d_n9;
        locals.var_sarg_dn10 = assign28150_e42254_d_n10;
        locals.var_sarg_dn11 = assign28150_e42254_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign28160_e42276, assign28160_e42276_d_n3, assign28160_e42276_d_n4, assign28160_e42276_d_n5, assign28160_e42276_d_n6, assign28160_e42276_d_n7, assign28160_e42276_d_n8, assign28160_e42276_d_n9, assign28160_e42276_d_n10, assign28160_e42276_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 != 0.0)) {
        let assign28160_e42264: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign28160_e42268: f64 = (locals.var_arg * locals.var_sarg);
        let assign28160_e42269: f64 = (1.0 - assign28160_e42268);
        let assign28160_e42270: f64 = (assign28160_e42264 * assign28160_e42269);
        let assign28160_e42273: f64 = (1.0 - p.p917);
        let assign28160_e42274: f64 = (assign28160_e42270 / assign28160_e42273);
        (assign28160_e42274, ((assign28160_e42264 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign28160_e42273), (((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign28160_e42269) + (assign28160_e42264 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign28160_e42273), (((((locals.var_pbswgs_t_dn5 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn5)) * assign28160_e42269) + (assign28160_e42264 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign28160_e42273), ((assign28160_e42264 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign28160_e42273), ((assign28160_e42264 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7)))) / assign28160_e42273), ((assign28160_e42264 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8)))) / assign28160_e42273), ((assign28160_e42264 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9)))) / assign28160_e42273), ((assign28160_e42264 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10)))) / assign28160_e42273), ((assign28160_e42264 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11)))) / assign28160_e42273),)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11,)
    }
};
        locals.var_qbsj3 = assign28160_e42276;
        locals.var_qbsj3_dn3 = assign28160_e42276_d_n3;
        locals.var_qbsj3_dn4 = assign28160_e42276_d_n4;
        locals.var_qbsj3_dn5 = assign28160_e42276_d_n5;
        locals.var_qbsj3_dn6 = assign28160_e42276_d_n6;
        locals.var_qbsj3_dn7 = assign28160_e42276_d_n7;
        locals.var_qbsj3_dn8 = assign28160_e42276_d_n8;
        locals.var_qbsj3_dn9 = assign28160_e42276_d_n9;
        locals.var_qbsj3_dn10 = assign28160_e42276_d_n10;
        locals.var_qbsj3_dn11 = assign28160_e42276_d_n11;
        locals.var_qbsj3_rv = 0.0;

        let (assign28170_e42293, assign28170_e42293_d_n3, assign28170_e42293_d_n4, assign28170_e42293_d_n5, assign28170_e42293_d_n6, assign28170_e42293_d_n7, assign28170_e42293_d_n8, assign28170_e42293_d_n9, assign28170_e42293_d_n10, assign28170_e42293_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 != 0.0)) && (locals.var_guard587 == 0.0)) {
        let assign28170_e42287: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign28170_e42289: f64 = (locals.var_arg).ln();
        let assign28170_e42290: f64 = (-assign28170_e42289);
        let assign28170_e42291: f64 = (assign28170_e42287 * assign28170_e42290);
        (assign28170_e42291, (assign28170_e42287 * (-(locals.var_arg_dn3 / locals.var_arg))), ((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign28170_e42290) + (assign28170_e42287 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbswgs_t_dn5 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn5)) * assign28170_e42290) + (assign28170_e42287 * (-(locals.var_arg_dn5 / locals.var_arg)))), (assign28170_e42287 * (-(locals.var_arg_dn6 / locals.var_arg))), (assign28170_e42287 * (-(locals.var_arg_dn7 / locals.var_arg))), (assign28170_e42287 * (-(locals.var_arg_dn8 / locals.var_arg))), (assign28170_e42287 * (-(locals.var_arg_dn9 / locals.var_arg))), (assign28170_e42287 * (-(locals.var_arg_dn10 / locals.var_arg))), (assign28170_e42287 * (-(locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11,)
    }
};
        locals.var_qbsj3 = assign28170_e42293;
        locals.var_qbsj3_dn3 = assign28170_e42293_d_n3;
        locals.var_qbsj3_dn4 = assign28170_e42293_d_n4;
        locals.var_qbsj3_dn5 = assign28170_e42293_d_n5;
        locals.var_qbsj3_dn6 = assign28170_e42293_d_n6;
        locals.var_qbsj3_dn7 = assign28170_e42293_d_n7;
        locals.var_qbsj3_dn8 = assign28170_e42293_d_n8;
        locals.var_qbsj3_dn9 = assign28170_e42293_d_n9;
        locals.var_qbsj3_dn10 = assign28170_e42293_d_n10;
        locals.var_qbsj3_dn11 = assign28170_e42293_d_n11;
        locals.var_qbsj3_rv = 0.0;

        let (assign28180_e42318, assign28180_e42318_d_n3, assign28180_e42318_d_n4, assign28180_e42318_d_n5, assign28180_e42318_d_n6, assign28180_e42318_d_n7, assign28180_e42318_d_n8, assign28180_e42318_d_n9, assign28180_e42318_d_n10, assign28180_e42318_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 == 0.0)) {
        let assign28180_e42303: f64 = (locals.var_t1 - 1.0);
        let assign28180_e42304: f64 = (locals.var_czbsswg_p1 * assign28180_e42303);
        let assign28180_e42307: f64 = (5.0 * p.p917);
        let assign28180_e42310: f64 = (locals.var_t1 - 1.0);
        let assign28180_e42311: f64 = (assign28180_e42307 * assign28180_e42310);
        let assign28180_e42314: f64 = (1.0 + p.p917);
        let assign28180_e42315: f64 = (assign28180_e42311 + assign28180_e42314);
        let assign28180_e42316: f64 = (assign28180_e42304 * assign28180_e42315);
        (assign28180_e42316, (((locals.var_czbsswg_p1 * locals.var_t1_dn3) * assign28180_e42315) + (assign28180_e42304 * (assign28180_e42307 * locals.var_t1_dn3))), (((locals.var_czbsswg_p1 * locals.var_t1_dn4) * assign28180_e42315) + (assign28180_e42304 * (assign28180_e42307 * locals.var_t1_dn4))), (((locals.var_czbsswg_p1 * locals.var_t1_dn5) * assign28180_e42315) + (assign28180_e42304 * (assign28180_e42307 * locals.var_t1_dn5))), (((locals.var_czbsswg_p1 * locals.var_t1_dn6) * assign28180_e42315) + (assign28180_e42304 * (assign28180_e42307 * locals.var_t1_dn6))), (((locals.var_czbsswg_p1 * locals.var_t1_dn7) * assign28180_e42315) + (assign28180_e42304 * (assign28180_e42307 * locals.var_t1_dn7))), (((locals.var_czbsswg_p1 * locals.var_t1_dn8) * assign28180_e42315) + (assign28180_e42304 * (assign28180_e42307 * locals.var_t1_dn8))), (((locals.var_czbsswg_p1 * locals.var_t1_dn9) * assign28180_e42315) + (assign28180_e42304 * (assign28180_e42307 * locals.var_t1_dn9))), (((locals.var_czbsswg_p1 * locals.var_t1_dn10) * assign28180_e42315) + (assign28180_e42304 * (assign28180_e42307 * locals.var_t1_dn10))), (((locals.var_czbsswg_p1 * locals.var_t1_dn11) * assign28180_e42315) + (assign28180_e42304 * (assign28180_e42307 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign28180_e42318;
        locals.var_t2_dn3 = assign28180_e42318_d_n3;
        locals.var_t2_dn4 = assign28180_e42318_d_n4;
        locals.var_t2_dn5 = assign28180_e42318_d_n5;
        locals.var_t2_dn6 = assign28180_e42318_d_n6;
        locals.var_t2_dn7 = assign28180_e42318_d_n7;
        locals.var_t2_dn8 = assign28180_e42318_d_n8;
        locals.var_t2_dn9 = assign28180_e42318_d_n9;
        locals.var_t2_dn10 = assign28180_e42318_d_n10;
        locals.var_t2_dn11 = assign28180_e42318_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign28190_e42333, assign28190_e42333_d_n3, assign28190_e42333_d_n4, assign28190_e42333_d_n5, assign28190_e42333_d_n6, assign28190_e42333_d_n7, assign28190_e42333_d_n8, assign28190_e42333_d_n9, assign28190_e42333_d_n10, assign28190_e42333_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard585 != 0.0)) && (locals.var_guard586 == 0.0)) {
        let assign28190_e42327: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign28190_e42330: f64 = (locals.var_t2 + locals.var_czbsswg_p2);
        let assign28190_e42331: f64 = (assign28190_e42327 * assign28190_e42330);
        (assign28190_e42331, (assign28190_e42327 * locals.var_t2_dn3), ((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign28190_e42330) + (assign28190_e42327 * locals.var_t2_dn4)), ((((locals.var_pbswgs_t_dn5 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn5)) * assign28190_e42330) + (assign28190_e42327 * locals.var_t2_dn5)), (assign28190_e42327 * locals.var_t2_dn6), (assign28190_e42327 * locals.var_t2_dn7), (assign28190_e42327 * locals.var_t2_dn8), (assign28190_e42327 * locals.var_t2_dn9), (assign28190_e42327 * locals.var_t2_dn10), (assign28190_e42327 * locals.var_t2_dn11),)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11,)
    }
};
        locals.var_qbsj3 = assign28190_e42333;
        locals.var_qbsj3_dn3 = assign28190_e42333_d_n3;
        locals.var_qbsj3_dn4 = assign28190_e42333_d_n4;
        locals.var_qbsj3_dn5 = assign28190_e42333_d_n5;
        locals.var_qbsj3_dn6 = assign28190_e42333_d_n6;
        locals.var_qbsj3_dn7 = assign28190_e42333_d_n7;
        locals.var_qbsj3_dn8 = assign28190_e42333_d_n8;
        locals.var_qbsj3_dn9 = assign28190_e42333_d_n9;
        locals.var_qbsj3_dn10 = assign28190_e42333_d_n10;
        locals.var_qbsj3_dn11 = assign28190_e42333_d_n11;
        locals.var_qbsj3_rv = 0.0;

        let (assign28200_e42340, assign28200_e42340_d_n3, assign28200_e42340_d_n4, assign28200_e42340_d_n5, assign28200_e42340_d_n6, assign28200_e42340_d_n7, assign28200_e42340_d_n8, assign28200_e42340_d_n9, assign28200_e42340_d_n10, assign28200_e42340_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard585 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11,)
    }
};
        locals.var_qbsj3 = assign28200_e42340;
        locals.var_qbsj3_dn3 = assign28200_e42340_d_n3;
        locals.var_qbsj3_dn4 = assign28200_e42340_d_n4;
        locals.var_qbsj3_dn5 = assign28200_e42340_d_n5;
        locals.var_qbsj3_dn6 = assign28200_e42340_d_n6;
        locals.var_qbsj3_dn7 = assign28200_e42340_d_n7;
        locals.var_qbsj3_dn8 = assign28200_e42340_d_n8;
        locals.var_qbsj3_dn9 = assign28200_e42340_d_n9;
        locals.var_qbsj3_dn10 = assign28200_e42340_d_n10;
        locals.var_qbsj3_dn11 = assign28200_e42340_d_n11;
        locals.var_qbsj3_rv = 0.0;

        let (assign28210_e42348, assign28210_e42348_d_n3, assign28210_e42348_d_n4, assign28210_e42348_d_n5, assign28210_e42348_d_n6, assign28210_e42348_d_n7, assign28210_e42348_d_n8, assign28210_e42348_d_n9, assign28210_e42348_d_n10, assign28210_e42348_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28210_e42344: f64 = (p.p919 * locals.var_ibsdif);
        let assign28210_e42346: f64 = (assign28210_e42344 * p.p2);
        (assign28210_e42346, ((p.p919 * locals.var_ibsdif_dn3) * p.p2), ((p.p919 * locals.var_ibsdif_dn4) * p.p2), ((p.p919 * locals.var_ibsdif_dn5) * p.p2), ((p.p919 * locals.var_ibsdif_dn6) * p.p2), ((p.p919 * locals.var_ibsdif_dn7) * p.p2), ((p.p919 * locals.var_ibsdif_dn8) * p.p2), ((p.p919 * locals.var_ibsdif_dn9) * p.p2), ((p.p919 * locals.var_ibsdif_dn10) * p.p2), ((p.p919 * locals.var_ibsdif_dn11) * p.p2),)
    } else {
        (locals.var_qbsj4, locals.var_qbsj4_dn3, locals.var_qbsj4_dn4, locals.var_qbsj4_dn5, locals.var_qbsj4_dn6, locals.var_qbsj4_dn7, locals.var_qbsj4_dn8, locals.var_qbsj4_dn9, locals.var_qbsj4_dn10, locals.var_qbsj4_dn11,)
    }
};
        locals.var_qbsj4 = assign28210_e42348;
        locals.var_qbsj4_dn3 = assign28210_e42348_d_n3;
        locals.var_qbsj4_dn4 = assign28210_e42348_d_n4;
        locals.var_qbsj4_dn5 = assign28210_e42348_d_n5;
        locals.var_qbsj4_dn6 = assign28210_e42348_d_n6;
        locals.var_qbsj4_dn7 = assign28210_e42348_d_n7;
        locals.var_qbsj4_dn8 = assign28210_e42348_d_n8;
        locals.var_qbsj4_dn9 = assign28210_e42348_d_n9;
        locals.var_qbsj4_dn10 = assign28210_e42348_d_n10;
        locals.var_qbsj4_dn11 = assign28210_e42348_d_n11;
        locals.var_qbsj4_rv = 0.0;

        let (assign28220_e42358, assign28220_e42358_d_n3, assign28220_e42358_d_n4, assign28220_e42358_d_n5, assign28220_e42358_d_n6, assign28220_e42358_d_n7, assign28220_e42358_d_n8, assign28220_e42358_d_n9, assign28220_e42358_d_n10, assign28220_e42358_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28220_e42352: f64 = (locals.var_qbsj1 + locals.var_qbsj2);
        let assign28220_e42354: f64 = (assign28220_e42352 + locals.var_qbsj3);
        let assign28220_e42356: f64 = (assign28220_e42354 + locals.var_qbsj4);
        (assign28220_e42356, (((locals.var_qbsj1_dn3 + locals.var_qbsj2_dn3) + locals.var_qbsj3_dn3) + locals.var_qbsj4_dn3), (((locals.var_qbsj1_dn4 + locals.var_qbsj2_dn4) + locals.var_qbsj3_dn4) + locals.var_qbsj4_dn4), (((locals.var_qbsj1_dn5 + locals.var_qbsj2_dn5) + locals.var_qbsj3_dn5) + locals.var_qbsj4_dn5), (((locals.var_qbsj1_dn6 + locals.var_qbsj2_dn6) + locals.var_qbsj3_dn6) + locals.var_qbsj4_dn6), (((locals.var_qbsj1_dn7 + locals.var_qbsj2_dn7) + locals.var_qbsj3_dn7) + locals.var_qbsj4_dn7), (((locals.var_qbsj1_dn8 + locals.var_qbsj2_dn8) + locals.var_qbsj3_dn8) + locals.var_qbsj4_dn8), (((locals.var_qbsj1_dn9 + locals.var_qbsj2_dn9) + locals.var_qbsj3_dn9) + locals.var_qbsj4_dn9), (((locals.var_qbsj1_dn10 + locals.var_qbsj2_dn10) + locals.var_qbsj3_dn10) + locals.var_qbsj4_dn10), (((locals.var_qbsj1_dn11 + locals.var_qbsj2_dn11) + locals.var_qbsj3_dn11) + locals.var_qbsj4_dn11),)
    } else {
        (locals.var_qbsj, locals.var_qbsj_dn3, locals.var_qbsj_dn4, locals.var_qbsj_dn5, locals.var_qbsj_dn6, locals.var_qbsj_dn7, locals.var_qbsj_dn8, locals.var_qbsj_dn9, locals.var_qbsj_dn10, locals.var_qbsj_dn11,)
    }
};
        locals.var_qbsj = assign28220_e42358;
        locals.var_qbsj_dn3 = assign28220_e42358_d_n3;
        locals.var_qbsj_dn4 = assign28220_e42358_d_n4;
        locals.var_qbsj_dn5 = assign28220_e42358_d_n5;
        locals.var_qbsj_dn6 = assign28220_e42358_d_n6;
        locals.var_qbsj_dn7 = assign28220_e42358_d_n7;
        locals.var_qbsj_dn8 = assign28220_e42358_d_n8;
        locals.var_qbsj_dn9 = assign28220_e42358_d_n9;
        locals.var_qbsj_dn10 = assign28220_e42358_d_n10;
        locals.var_qbsj_dn11 = assign28220_e42358_d_n11;
        locals.var_qbsj_rv = 0.0;

        let (assign28230_e42364, assign28230_e42364_d_n3, assign28230_e42364_d_n4, assign28230_e42364_d_n5, assign28230_e42364_d_n6, assign28230_e42364_d_n7, assign28230_e42364_d_n8, assign28230_e42364_d_n9, assign28230_e42364_d_n10, assign28230_e42364_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28230_e42362: f64 = (locals.var_cjd_t * locals.var_adeff);
        (assign28230_e42362, (locals.var_cjd_t * locals.var_adeff_dn3), ((locals.var_cjd_t_dn4 * locals.var_adeff) + (locals.var_cjd_t * locals.var_adeff_dn4)), ((locals.var_cjd_t_dn5 * locals.var_adeff) + (locals.var_cjd_t * locals.var_adeff_dn5)), (locals.var_cjd_t * locals.var_adeff_dn6), (locals.var_cjd_t * locals.var_adeff_dn7), (locals.var_cjd_t * locals.var_adeff_dn8), (locals.var_cjd_t * locals.var_adeff_dn9), (locals.var_cjd_t * locals.var_adeff_dn10), (locals.var_cjd_t * locals.var_adeff_dn11),)
    } else {
        (locals.var_czbd, locals.var_czbd_dn3, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11,)
    }
};
        locals.var_czbd = assign28230_e42364;
        locals.var_czbd_dn3 = assign28230_e42364_d_n3;
        locals.var_czbd_dn4 = assign28230_e42364_d_n4;
        locals.var_czbd_dn5 = assign28230_e42364_d_n5;
        locals.var_czbd_dn6 = assign28230_e42364_d_n6;
        locals.var_czbd_dn7 = assign28230_e42364_d_n7;
        locals.var_czbd_dn8 = assign28230_e42364_d_n8;
        locals.var_czbd_dn9 = assign28230_e42364_d_n9;
        locals.var_czbd_dn10 = assign28230_e42364_d_n10;
        locals.var_czbd_dn11 = assign28230_e42364_d_n11;
        locals.var_czbd_rv = 0.0;

        let (assign28240_e42370, assign28240_e42370_d_n3, assign28240_e42370_d_n4, assign28240_e42370_d_n5, assign28240_e42370_d_n6, assign28240_e42370_d_n7, assign28240_e42370_d_n8, assign28240_e42370_d_n9, assign28240_e42370_d_n10, assign28240_e42370_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28240_e42368: f64 = (locals.var_cjswd_t * locals.var_pdeff);
        (assign28240_e42368, (locals.var_cjswd_t * locals.var_pdeff_dn3), ((locals.var_cjswd_t_dn4 * locals.var_pdeff) + (locals.var_cjswd_t * locals.var_pdeff_dn4)), ((locals.var_cjswd_t_dn5 * locals.var_pdeff) + (locals.var_cjswd_t * locals.var_pdeff_dn5)), (locals.var_cjswd_t * locals.var_pdeff_dn6), (locals.var_cjswd_t * locals.var_pdeff_dn7), (locals.var_cjswd_t * locals.var_pdeff_dn8), (locals.var_cjswd_t * locals.var_pdeff_dn9), (locals.var_cjswd_t * locals.var_pdeff_dn10), (locals.var_cjswd_t * locals.var_pdeff_dn11),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn3, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11,)
    }
};
        locals.var_czbdsw = assign28240_e42370;
        locals.var_czbdsw_dn3 = assign28240_e42370_d_n3;
        locals.var_czbdsw_dn4 = assign28240_e42370_d_n4;
        locals.var_czbdsw_dn5 = assign28240_e42370_d_n5;
        locals.var_czbdsw_dn6 = assign28240_e42370_d_n6;
        locals.var_czbdsw_dn7 = assign28240_e42370_d_n7;
        locals.var_czbdsw_dn8 = assign28240_e42370_d_n8;
        locals.var_czbdsw_dn9 = assign28240_e42370_d_n9;
        locals.var_czbdsw_dn10 = assign28240_e42370_d_n10;
        locals.var_czbdsw_dn11 = assign28240_e42370_d_n11;
        locals.var_czbdsw_rv = 0.0;

        let (assign28250_e42378, assign28250_e42378_d_n4, assign28250_e42378_d_n5,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28250_e42374: f64 = (locals.var_cjswgd_t * locals.var_weffcj);
        let assign28250_e42376: f64 = (assign28250_e42374 * p.p2);
        (assign28250_e42376, ((locals.var_cjswgd_t_dn4 * locals.var_weffcj) * p.p2), ((locals.var_cjswgd_t_dn5 * locals.var_weffcj) * p.p2),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5,)
    }
};
        locals.var_czbdswg = assign28250_e42378;
        locals.var_czbdswg_dn4 = assign28250_e42378_d_n4;
        locals.var_czbdswg_dn5 = assign28250_e42378_d_n5;
        locals.var_czbdswg_rv = 0.0;

        let (assign28260_e42385,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28260_e42382: f64 = (-p.p914);
        let assign28260_e42383: f64 = (0.1_f64).powf(assign28260_e42382);
        (assign28260_e42383,)
    } else {
        (locals.var_czbd_p1,)
    }
};
        locals.var_czbd_p1 = assign28260_e42385;
        locals.var_czbd_p1_rv = 0.0;

        let assign28270_e42388: f64 = if p.p914 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard589 = assign28270_e42388;
        locals.var_guard589_rv = 0.0;

        let (assign28280_e42397,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard589 != 0.0)) {
        let assign28280_e42394: f64 = (0.1_f64).ln();
        let assign28280_e42395: f64 = (1.5 - assign28280_e42394);
        (assign28280_e42395,)
    } else {
        (locals.var_czbd_p2,)
    }
};
        locals.var_czbd_p2 = assign28280_e42397;
        locals.var_czbd_p2_rv = 0.0;

        let (assign28290_e42420,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard589 == 0.0)) {
        let assign28290_e42405: f64 = (1.0 - p.p914);
        let assign28290_e42406: f64 = (1.0 / assign28290_e42405);
        let assign28290_e42410: f64 = (0.05 * p.p914);
        let assign28290_e42413: f64 = (1.0 + p.p914);
        let assign28290_e42414: f64 = (assign28290_e42410 * assign28290_e42413);
        let assign28290_e42416: f64 = (assign28290_e42414 * locals.var_czbd_p1);
        let assign28290_e42417: f64 = (1.0 - assign28290_e42416);
        let assign28290_e42418: f64 = (assign28290_e42406 * assign28290_e42417);
        (assign28290_e42418,)
    } else {
        (locals.var_czbd_p2,)
    }
};
        locals.var_czbd_p2 = assign28290_e42420;
        locals.var_czbd_p2_rv = 0.0;

        let (assign28300_e42427,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28300_e42424: f64 = (-p.p916);
        let assign28300_e42425: f64 = (0.1_f64).powf(assign28300_e42424);
        (assign28300_e42425,)
    } else {
        (locals.var_czbdsw_p1,)
    }
};
        locals.var_czbdsw_p1 = assign28300_e42427;
        locals.var_czbdsw_p1_rv = 0.0;

        let assign28310_e42430: f64 = if p.p916 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard590 = assign28310_e42430;
        locals.var_guard590_rv = 0.0;

        let (assign28320_e42439,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard590 != 0.0)) {
        let assign28320_e42436: f64 = (0.1_f64).ln();
        let assign28320_e42437: f64 = (1.5 - assign28320_e42436);
        (assign28320_e42437,)
    } else {
        (locals.var_czbdsw_p2,)
    }
};
        locals.var_czbdsw_p2 = assign28320_e42439;
        locals.var_czbdsw_p2_rv = 0.0;

        let (assign28330_e42462,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard590 == 0.0)) {
        let assign28330_e42447: f64 = (1.0 - p.p916);
        let assign28330_e42448: f64 = (1.0 / assign28330_e42447);
        let assign28330_e42452: f64 = (0.05 * p.p916);
        let assign28330_e42455: f64 = (1.0 + p.p916);
        let assign28330_e42456: f64 = (assign28330_e42452 * assign28330_e42455);
        let assign28330_e42458: f64 = (assign28330_e42456 * locals.var_czbdsw_p1);
        let assign28330_e42459: f64 = (1.0 - assign28330_e42458);
        let assign28330_e42460: f64 = (assign28330_e42448 * assign28330_e42459);
        (assign28330_e42460,)
    } else {
        (locals.var_czbdsw_p2,)
    }
};
        locals.var_czbdsw_p2 = assign28330_e42462;
        locals.var_czbdsw_p2_rv = 0.0;

        let (assign28340_e42469,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28340_e42466: f64 = (-p.p918);
        let assign28340_e42467: f64 = (0.1_f64).powf(assign28340_e42466);
        (assign28340_e42467,)
    } else {
        (locals.var_czbdswg_p1,)
    }
};
        locals.var_czbdswg_p1 = assign28340_e42469;
        locals.var_czbdswg_p1_rv = 0.0;

        let assign28350_e42472: f64 = if p.p918 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard591 = assign28350_e42472;
        locals.var_guard591_rv = 0.0;

        let (assign28360_e42481,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard591 != 0.0)) {
        let assign28360_e42478: f64 = (0.1_f64).ln();
        let assign28360_e42479: f64 = (1.5 - assign28360_e42478);
        (assign28360_e42479,)
    } else {
        (locals.var_czbdswg_p2,)
    }
};
        locals.var_czbdswg_p2 = assign28360_e42481;
        locals.var_czbdswg_p2_rv = 0.0;

        let (assign28370_e42504,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard591 == 0.0)) {
        let assign28370_e42489: f64 = (1.0 - p.p918);
        let assign28370_e42490: f64 = (1.0 / assign28370_e42489);
        let assign28370_e42494: f64 = (0.05 * p.p918);
        let assign28370_e42497: f64 = (1.0 + p.p918);
        let assign28370_e42498: f64 = (assign28370_e42494 * assign28370_e42497);
        let assign28370_e42500: f64 = (assign28370_e42498 * locals.var_czbdswg_p1);
        let assign28370_e42501: f64 = (1.0 - assign28370_e42500);
        let assign28370_e42502: f64 = (assign28370_e42490 * assign28370_e42501);
        (assign28370_e42502,)
    } else {
        (locals.var_czbdswg_p2,)
    }
};
        locals.var_czbdswg_p2 = assign28370_e42504;
        locals.var_czbdswg_p2_rv = 0.0;

        let assign28380_e42507: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard592 = assign28380_e42507;
        locals.var_guard592_rv = 0.0;

        let (assign28390_e42515, assign28390_e42515_d_n3, assign28390_e42515_d_n4, assign28390_e42515_d_n5, assign28390_e42515_d_n6, assign28390_e42515_d_n7, assign28390_e42515_d_n8, assign28390_e42515_d_n9, assign28390_e42515_d_n10, assign28390_e42515_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard592 != 0.0)) {
        let assign28390_e42513: f64 = (locals.var_vbd_jct / locals.var_pbd_t);
        (assign28390_e42513, 0.0, (-((locals.var_vbd_jct * locals.var_pbd_t_dn4) / (locals.var_pbd_t * locals.var_pbd_t))), (-((locals.var_vbd_jct * locals.var_pbd_t_dn5) / (locals.var_pbd_t * locals.var_pbd_t))), (locals.var_vbd_jct_dn6 / locals.var_pbd_t), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_pbd_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign28390_e42515;
        locals.var_t1_dn3 = assign28390_e42515_d_n3;
        locals.var_t1_dn4 = assign28390_e42515_d_n4;
        locals.var_t1_dn5 = assign28390_e42515_d_n5;
        locals.var_t1_dn6 = assign28390_e42515_d_n6;
        locals.var_t1_dn7 = assign28390_e42515_d_n7;
        locals.var_t1_dn8 = assign28390_e42515_d_n8;
        locals.var_t1_dn9 = assign28390_e42515_d_n9;
        locals.var_t1_dn10 = assign28390_e42515_d_n10;
        locals.var_t1_dn11 = assign28390_e42515_d_n11;
        locals.var_t1_rv = 0.0;

        let assign28400_e42518: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard593 = assign28400_e42518;
        locals.var_guard593_rv = 0.0;

        let (assign28410_e42528, assign28410_e42528_d_n3, assign28410_e42528_d_n4, assign28410_e42528_d_n5, assign28410_e42528_d_n6, assign28410_e42528_d_n7, assign28410_e42528_d_n8, assign28410_e42528_d_n9, assign28410_e42528_d_n10, assign28410_e42528_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 != 0.0)) {
        let assign28410_e42526: f64 = (1.0 - locals.var_t1);
        (assign28410_e42526, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign28410_e42528;
        locals.var_arg_dn3 = assign28410_e42528_d_n3;
        locals.var_arg_dn4 = assign28410_e42528_d_n4;
        locals.var_arg_dn5 = assign28410_e42528_d_n5;
        locals.var_arg_dn6 = assign28410_e42528_d_n6;
        locals.var_arg_dn7 = assign28410_e42528_d_n7;
        locals.var_arg_dn8 = assign28410_e42528_d_n8;
        locals.var_arg_dn9 = assign28410_e42528_d_n9;
        locals.var_arg_dn10 = assign28410_e42528_d_n10;
        locals.var_arg_dn11 = assign28410_e42528_d_n11;
        locals.var_arg_rv = 0.0;

        let assign28420_e42531: f64 = if p.p914 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard594 = assign28420_e42531;
        locals.var_guard594_rv = 0.0;

        let assign28430_e42534: f64 = if p.p914 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard595 = assign28430_e42534;
        locals.var_guard595_rv = 0.0;

        let (assign28440_e42549, assign28440_e42549_d_n3, assign28440_e42549_d_n4, assign28440_e42549_d_n5, assign28440_e42549_d_n6, assign28440_e42549_d_n7, assign28440_e42549_d_n8, assign28440_e42549_d_n9, assign28440_e42549_d_n10, assign28440_e42549_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 != 0.0)) && (locals.var_guard594 != 0.0)) && (locals.var_guard595 != 0.0)) {
        let assign28440_e42546: f64 = (locals.var_arg).sqrt();
        let assign28440_e42547: f64 = (1.0 / assign28440_e42546);
        (assign28440_e42547, (-((locals.var_arg_dn3 / (2.0 * assign28440_e42546)) / (assign28440_e42546 * assign28440_e42546))), (-((locals.var_arg_dn4 / (2.0 * assign28440_e42546)) / (assign28440_e42546 * assign28440_e42546))), (-((locals.var_arg_dn5 / (2.0 * assign28440_e42546)) / (assign28440_e42546 * assign28440_e42546))), (-((locals.var_arg_dn6 / (2.0 * assign28440_e42546)) / (assign28440_e42546 * assign28440_e42546))), (-((locals.var_arg_dn7 / (2.0 * assign28440_e42546)) / (assign28440_e42546 * assign28440_e42546))), (-((locals.var_arg_dn8 / (2.0 * assign28440_e42546)) / (assign28440_e42546 * assign28440_e42546))), (-((locals.var_arg_dn9 / (2.0 * assign28440_e42546)) / (assign28440_e42546 * assign28440_e42546))), (-((locals.var_arg_dn10 / (2.0 * assign28440_e42546)) / (assign28440_e42546 * assign28440_e42546))), (-((locals.var_arg_dn11 / (2.0 * assign28440_e42546)) / (assign28440_e42546 * assign28440_e42546))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign28440_e42549;
        locals.var_sarg_dn3 = assign28440_e42549_d_n3;
        locals.var_sarg_dn4 = assign28440_e42549_d_n4;
        locals.var_sarg_dn5 = assign28440_e42549_d_n5;
        locals.var_sarg_dn6 = assign28440_e42549_d_n6;
        locals.var_sarg_dn7 = assign28440_e42549_d_n7;
        locals.var_sarg_dn8 = assign28440_e42549_d_n8;
        locals.var_sarg_dn9 = assign28440_e42549_d_n9;
        locals.var_sarg_dn10 = assign28440_e42549_d_n10;
        locals.var_sarg_dn11 = assign28440_e42549_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign28450_e42567, assign28450_e42567_d_n3, assign28450_e42567_d_n4, assign28450_e42567_d_n5, assign28450_e42567_d_n6, assign28450_e42567_d_n7, assign28450_e42567_d_n8, assign28450_e42567_d_n9, assign28450_e42567_d_n10, assign28450_e42567_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 != 0.0)) && (locals.var_guard594 != 0.0)) && (locals.var_guard595 == 0.0)) {
        let assign28450_e42561: f64 = (-p.p914);
        let assign28450_e42563: f64 = (locals.var_arg).ln();
        let assign28450_e42564: f64 = (assign28450_e42561 * assign28450_e42563);
        let assign28450_e42565: f64 = { let limited_exp_arg = assign28450_e42564; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign28450_e42565, ({ let limited_exp_arg = assign28450_e42564; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28450_e42561 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign28450_e42564; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28450_e42561 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign28450_e42564; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28450_e42561 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign28450_e42564; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28450_e42561 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign28450_e42564; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28450_e42561 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign28450_e42564; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28450_e42561 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign28450_e42564; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28450_e42561 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign28450_e42564; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28450_e42561 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign28450_e42564; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28450_e42561 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign28450_e42567;
        locals.var_sarg_dn3 = assign28450_e42567_d_n3;
        locals.var_sarg_dn4 = assign28450_e42567_d_n4;
        locals.var_sarg_dn5 = assign28450_e42567_d_n5;
        locals.var_sarg_dn6 = assign28450_e42567_d_n6;
        locals.var_sarg_dn7 = assign28450_e42567_d_n7;
        locals.var_sarg_dn8 = assign28450_e42567_d_n8;
        locals.var_sarg_dn9 = assign28450_e42567_d_n9;
        locals.var_sarg_dn10 = assign28450_e42567_d_n10;
        locals.var_sarg_dn11 = assign28450_e42567_d_n11;
        locals.var_sarg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_81(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28460_e42589, assign28460_e42589_d_n3, assign28460_e42589_d_n4, assign28460_e42589_d_n5, assign28460_e42589_d_n6, assign28460_e42589_d_n7, assign28460_e42589_d_n8, assign28460_e42589_d_n9, assign28460_e42589_d_n10, assign28460_e42589_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 != 0.0)) && (locals.var_guard594 != 0.0)) {
        let assign28460_e42577: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign28460_e42581: f64 = (locals.var_arg * locals.var_sarg);
        let assign28460_e42582: f64 = (1.0 - assign28460_e42581);
        let assign28460_e42583: f64 = (assign28460_e42577 * assign28460_e42582);
        let assign28460_e42586: f64 = (1.0 - p.p914);
        let assign28460_e42587: f64 = (assign28460_e42583 / assign28460_e42586);
        (assign28460_e42587, ((((locals.var_pbd_t * locals.var_czbd_dn3) * assign28460_e42582) + (assign28460_e42577 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign28460_e42586), (((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign28460_e42582) + (assign28460_e42577 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign28460_e42586), (((((locals.var_pbd_t_dn5 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn5)) * assign28460_e42582) + (assign28460_e42577 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign28460_e42586), ((((locals.var_pbd_t * locals.var_czbd_dn6) * assign28460_e42582) + (assign28460_e42577 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign28460_e42586), ((((locals.var_pbd_t * locals.var_czbd_dn7) * assign28460_e42582) + (assign28460_e42577 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign28460_e42586), ((((locals.var_pbd_t * locals.var_czbd_dn8) * assign28460_e42582) + (assign28460_e42577 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign28460_e42586), ((((locals.var_pbd_t * locals.var_czbd_dn9) * assign28460_e42582) + (assign28460_e42577 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign28460_e42586), ((((locals.var_pbd_t * locals.var_czbd_dn10) * assign28460_e42582) + (assign28460_e42577 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign28460_e42586), ((((locals.var_pbd_t * locals.var_czbd_dn11) * assign28460_e42582) + (assign28460_e42577 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign28460_e42586),)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11,)
    }
};
        locals.var_qbdj1 = assign28460_e42589;
        locals.var_qbdj1_dn3 = assign28460_e42589_d_n3;
        locals.var_qbdj1_dn4 = assign28460_e42589_d_n4;
        locals.var_qbdj1_dn5 = assign28460_e42589_d_n5;
        locals.var_qbdj1_dn6 = assign28460_e42589_d_n6;
        locals.var_qbdj1_dn7 = assign28460_e42589_d_n7;
        locals.var_qbdj1_dn8 = assign28460_e42589_d_n8;
        locals.var_qbdj1_dn9 = assign28460_e42589_d_n9;
        locals.var_qbdj1_dn10 = assign28460_e42589_d_n10;
        locals.var_qbdj1_dn11 = assign28460_e42589_d_n11;
        locals.var_qbdj1_rv = 0.0;

        let (assign28470_e42606, assign28470_e42606_d_n3, assign28470_e42606_d_n4, assign28470_e42606_d_n5, assign28470_e42606_d_n6, assign28470_e42606_d_n7, assign28470_e42606_d_n8, assign28470_e42606_d_n9, assign28470_e42606_d_n10, assign28470_e42606_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 != 0.0)) && (locals.var_guard594 == 0.0)) {
        let assign28470_e42600: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign28470_e42602: f64 = (locals.var_arg).ln();
        let assign28470_e42603: f64 = (-assign28470_e42602);
        let assign28470_e42604: f64 = (assign28470_e42600 * assign28470_e42603);
        (assign28470_e42604, (((locals.var_pbd_t * locals.var_czbd_dn3) * assign28470_e42603) + (assign28470_e42600 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign28470_e42603) + (assign28470_e42600 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbd_t_dn5 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn5)) * assign28470_e42603) + (assign28470_e42600 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn6) * assign28470_e42603) + (assign28470_e42600 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn7) * assign28470_e42603) + (assign28470_e42600 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn8) * assign28470_e42603) + (assign28470_e42600 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn9) * assign28470_e42603) + (assign28470_e42600 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn10) * assign28470_e42603) + (assign28470_e42600 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn11) * assign28470_e42603) + (assign28470_e42600 * (-(locals.var_arg_dn11 / locals.var_arg)))),)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11,)
    }
};
        locals.var_qbdj1 = assign28470_e42606;
        locals.var_qbdj1_dn3 = assign28470_e42606_d_n3;
        locals.var_qbdj1_dn4 = assign28470_e42606_d_n4;
        locals.var_qbdj1_dn5 = assign28470_e42606_d_n5;
        locals.var_qbdj1_dn6 = assign28470_e42606_d_n6;
        locals.var_qbdj1_dn7 = assign28470_e42606_d_n7;
        locals.var_qbdj1_dn8 = assign28470_e42606_d_n8;
        locals.var_qbdj1_dn9 = assign28470_e42606_d_n9;
        locals.var_qbdj1_dn10 = assign28470_e42606_d_n10;
        locals.var_qbdj1_dn11 = assign28470_e42606_d_n11;
        locals.var_qbdj1_rv = 0.0;

        let (assign28480_e42631, assign28480_e42631_d_n3, assign28480_e42631_d_n4, assign28480_e42631_d_n5, assign28480_e42631_d_n6, assign28480_e42631_d_n7, assign28480_e42631_d_n8, assign28480_e42631_d_n9, assign28480_e42631_d_n10, assign28480_e42631_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) {
        let assign28480_e42616: f64 = (locals.var_t1 - 1.0);
        let assign28480_e42617: f64 = (locals.var_czbd_p1 * assign28480_e42616);
        let assign28480_e42620: f64 = (5.0 * p.p914);
        let assign28480_e42623: f64 = (locals.var_t1 - 1.0);
        let assign28480_e42624: f64 = (assign28480_e42620 * assign28480_e42623);
        let assign28480_e42627: f64 = (1.0 + p.p914);
        let assign28480_e42628: f64 = (assign28480_e42624 + assign28480_e42627);
        let assign28480_e42629: f64 = (assign28480_e42617 * assign28480_e42628);
        (assign28480_e42629, (((locals.var_czbd_p1 * locals.var_t1_dn3) * assign28480_e42628) + (assign28480_e42617 * (assign28480_e42620 * locals.var_t1_dn3))), (((locals.var_czbd_p1 * locals.var_t1_dn4) * assign28480_e42628) + (assign28480_e42617 * (assign28480_e42620 * locals.var_t1_dn4))), (((locals.var_czbd_p1 * locals.var_t1_dn5) * assign28480_e42628) + (assign28480_e42617 * (assign28480_e42620 * locals.var_t1_dn5))), (((locals.var_czbd_p1 * locals.var_t1_dn6) * assign28480_e42628) + (assign28480_e42617 * (assign28480_e42620 * locals.var_t1_dn6))), (((locals.var_czbd_p1 * locals.var_t1_dn7) * assign28480_e42628) + (assign28480_e42617 * (assign28480_e42620 * locals.var_t1_dn7))), (((locals.var_czbd_p1 * locals.var_t1_dn8) * assign28480_e42628) + (assign28480_e42617 * (assign28480_e42620 * locals.var_t1_dn8))), (((locals.var_czbd_p1 * locals.var_t1_dn9) * assign28480_e42628) + (assign28480_e42617 * (assign28480_e42620 * locals.var_t1_dn9))), (((locals.var_czbd_p1 * locals.var_t1_dn10) * assign28480_e42628) + (assign28480_e42617 * (assign28480_e42620 * locals.var_t1_dn10))), (((locals.var_czbd_p1 * locals.var_t1_dn11) * assign28480_e42628) + (assign28480_e42617 * (assign28480_e42620 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign28480_e42631;
        locals.var_t2_dn3 = assign28480_e42631_d_n3;
        locals.var_t2_dn4 = assign28480_e42631_d_n4;
        locals.var_t2_dn5 = assign28480_e42631_d_n5;
        locals.var_t2_dn6 = assign28480_e42631_d_n6;
        locals.var_t2_dn7 = assign28480_e42631_d_n7;
        locals.var_t2_dn8 = assign28480_e42631_d_n8;
        locals.var_t2_dn9 = assign28480_e42631_d_n9;
        locals.var_t2_dn10 = assign28480_e42631_d_n10;
        locals.var_t2_dn11 = assign28480_e42631_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign28490_e42646, assign28490_e42646_d_n3, assign28490_e42646_d_n4, assign28490_e42646_d_n5, assign28490_e42646_d_n6, assign28490_e42646_d_n7, assign28490_e42646_d_n8, assign28490_e42646_d_n9, assign28490_e42646_d_n10, assign28490_e42646_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard592 != 0.0)) && (locals.var_guard593 == 0.0)) {
        let assign28490_e42640: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign28490_e42643: f64 = (locals.var_t2 + locals.var_czbd_p2);
        let assign28490_e42644: f64 = (assign28490_e42640 * assign28490_e42643);
        (assign28490_e42644, (((locals.var_pbd_t * locals.var_czbd_dn3) * assign28490_e42643) + (assign28490_e42640 * locals.var_t2_dn3)), ((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign28490_e42643) + (assign28490_e42640 * locals.var_t2_dn4)), ((((locals.var_pbd_t_dn5 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn5)) * assign28490_e42643) + (assign28490_e42640 * locals.var_t2_dn5)), (((locals.var_pbd_t * locals.var_czbd_dn6) * assign28490_e42643) + (assign28490_e42640 * locals.var_t2_dn6)), (((locals.var_pbd_t * locals.var_czbd_dn7) * assign28490_e42643) + (assign28490_e42640 * locals.var_t2_dn7)), (((locals.var_pbd_t * locals.var_czbd_dn8) * assign28490_e42643) + (assign28490_e42640 * locals.var_t2_dn8)), (((locals.var_pbd_t * locals.var_czbd_dn9) * assign28490_e42643) + (assign28490_e42640 * locals.var_t2_dn9)), (((locals.var_pbd_t * locals.var_czbd_dn10) * assign28490_e42643) + (assign28490_e42640 * locals.var_t2_dn10)), (((locals.var_pbd_t * locals.var_czbd_dn11) * assign28490_e42643) + (assign28490_e42640 * locals.var_t2_dn11)),)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11,)
    }
};
        locals.var_qbdj1 = assign28490_e42646;
        locals.var_qbdj1_dn3 = assign28490_e42646_d_n3;
        locals.var_qbdj1_dn4 = assign28490_e42646_d_n4;
        locals.var_qbdj1_dn5 = assign28490_e42646_d_n5;
        locals.var_qbdj1_dn6 = assign28490_e42646_d_n6;
        locals.var_qbdj1_dn7 = assign28490_e42646_d_n7;
        locals.var_qbdj1_dn8 = assign28490_e42646_d_n8;
        locals.var_qbdj1_dn9 = assign28490_e42646_d_n9;
        locals.var_qbdj1_dn10 = assign28490_e42646_d_n10;
        locals.var_qbdj1_dn11 = assign28490_e42646_d_n11;
        locals.var_qbdj1_rv = 0.0;

        let (assign28500_e42653, assign28500_e42653_d_n3, assign28500_e42653_d_n4, assign28500_e42653_d_n5, assign28500_e42653_d_n6, assign28500_e42653_d_n7, assign28500_e42653_d_n8, assign28500_e42653_d_n9, assign28500_e42653_d_n10, assign28500_e42653_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard592 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11,)
    }
};
        locals.var_qbdj1 = assign28500_e42653;
        locals.var_qbdj1_dn3 = assign28500_e42653_d_n3;
        locals.var_qbdj1_dn4 = assign28500_e42653_d_n4;
        locals.var_qbdj1_dn5 = assign28500_e42653_d_n5;
        locals.var_qbdj1_dn6 = assign28500_e42653_d_n6;
        locals.var_qbdj1_dn7 = assign28500_e42653_d_n7;
        locals.var_qbdj1_dn8 = assign28500_e42653_d_n8;
        locals.var_qbdj1_dn9 = assign28500_e42653_d_n9;
        locals.var_qbdj1_dn10 = assign28500_e42653_d_n10;
        locals.var_qbdj1_dn11 = assign28500_e42653_d_n11;
        locals.var_qbdj1_rv = 0.0;

        let assign28510_e42656: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard596 = assign28510_e42656;
        locals.var_guard596_rv = 0.0;

        let (assign28520_e42664, assign28520_e42664_d_n3, assign28520_e42664_d_n4, assign28520_e42664_d_n5, assign28520_e42664_d_n6, assign28520_e42664_d_n7, assign28520_e42664_d_n8, assign28520_e42664_d_n9, assign28520_e42664_d_n10, assign28520_e42664_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard596 != 0.0)) {
        let assign28520_e42662: f64 = (locals.var_vbd_jct / locals.var_pbswd_t);
        (assign28520_e42662, 0.0, (-((locals.var_vbd_jct * locals.var_pbswd_t_dn4) / (locals.var_pbswd_t * locals.var_pbswd_t))), (-((locals.var_vbd_jct * locals.var_pbswd_t_dn5) / (locals.var_pbswd_t * locals.var_pbswd_t))), (locals.var_vbd_jct_dn6 / locals.var_pbswd_t), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_pbswd_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign28520_e42664;
        locals.var_t1_dn3 = assign28520_e42664_d_n3;
        locals.var_t1_dn4 = assign28520_e42664_d_n4;
        locals.var_t1_dn5 = assign28520_e42664_d_n5;
        locals.var_t1_dn6 = assign28520_e42664_d_n6;
        locals.var_t1_dn7 = assign28520_e42664_d_n7;
        locals.var_t1_dn8 = assign28520_e42664_d_n8;
        locals.var_t1_dn9 = assign28520_e42664_d_n9;
        locals.var_t1_dn10 = assign28520_e42664_d_n10;
        locals.var_t1_dn11 = assign28520_e42664_d_n11;
        locals.var_t1_rv = 0.0;

        let assign28530_e42667: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard597 = assign28530_e42667;
        locals.var_guard597_rv = 0.0;

        let (assign28540_e42677, assign28540_e42677_d_n3, assign28540_e42677_d_n4, assign28540_e42677_d_n5, assign28540_e42677_d_n6, assign28540_e42677_d_n7, assign28540_e42677_d_n8, assign28540_e42677_d_n9, assign28540_e42677_d_n10, assign28540_e42677_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 != 0.0)) {
        let assign28540_e42675: f64 = (1.0 - locals.var_t1);
        (assign28540_e42675, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign28540_e42677;
        locals.var_arg_dn3 = assign28540_e42677_d_n3;
        locals.var_arg_dn4 = assign28540_e42677_d_n4;
        locals.var_arg_dn5 = assign28540_e42677_d_n5;
        locals.var_arg_dn6 = assign28540_e42677_d_n6;
        locals.var_arg_dn7 = assign28540_e42677_d_n7;
        locals.var_arg_dn8 = assign28540_e42677_d_n8;
        locals.var_arg_dn9 = assign28540_e42677_d_n9;
        locals.var_arg_dn10 = assign28540_e42677_d_n10;
        locals.var_arg_dn11 = assign28540_e42677_d_n11;
        locals.var_arg_rv = 0.0;

        let assign28550_e42680: f64 = if p.p916 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard598 = assign28550_e42680;
        locals.var_guard598_rv = 0.0;

        let assign28560_e42683: f64 = if p.p916 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard599 = assign28560_e42683;
        locals.var_guard599_rv = 0.0;

        let (assign28570_e42698, assign28570_e42698_d_n3, assign28570_e42698_d_n4, assign28570_e42698_d_n5, assign28570_e42698_d_n6, assign28570_e42698_d_n7, assign28570_e42698_d_n8, assign28570_e42698_d_n9, assign28570_e42698_d_n10, assign28570_e42698_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 != 0.0)) && (locals.var_guard598 != 0.0)) && (locals.var_guard599 != 0.0)) {
        let assign28570_e42695: f64 = (locals.var_arg).sqrt();
        let assign28570_e42696: f64 = (1.0 / assign28570_e42695);
        (assign28570_e42696, (-((locals.var_arg_dn3 / (2.0 * assign28570_e42695)) / (assign28570_e42695 * assign28570_e42695))), (-((locals.var_arg_dn4 / (2.0 * assign28570_e42695)) / (assign28570_e42695 * assign28570_e42695))), (-((locals.var_arg_dn5 / (2.0 * assign28570_e42695)) / (assign28570_e42695 * assign28570_e42695))), (-((locals.var_arg_dn6 / (2.0 * assign28570_e42695)) / (assign28570_e42695 * assign28570_e42695))), (-((locals.var_arg_dn7 / (2.0 * assign28570_e42695)) / (assign28570_e42695 * assign28570_e42695))), (-((locals.var_arg_dn8 / (2.0 * assign28570_e42695)) / (assign28570_e42695 * assign28570_e42695))), (-((locals.var_arg_dn9 / (2.0 * assign28570_e42695)) / (assign28570_e42695 * assign28570_e42695))), (-((locals.var_arg_dn10 / (2.0 * assign28570_e42695)) / (assign28570_e42695 * assign28570_e42695))), (-((locals.var_arg_dn11 / (2.0 * assign28570_e42695)) / (assign28570_e42695 * assign28570_e42695))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign28570_e42698;
        locals.var_sarg_dn3 = assign28570_e42698_d_n3;
        locals.var_sarg_dn4 = assign28570_e42698_d_n4;
        locals.var_sarg_dn5 = assign28570_e42698_d_n5;
        locals.var_sarg_dn6 = assign28570_e42698_d_n6;
        locals.var_sarg_dn7 = assign28570_e42698_d_n7;
        locals.var_sarg_dn8 = assign28570_e42698_d_n8;
        locals.var_sarg_dn9 = assign28570_e42698_d_n9;
        locals.var_sarg_dn10 = assign28570_e42698_d_n10;
        locals.var_sarg_dn11 = assign28570_e42698_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign28580_e42716, assign28580_e42716_d_n3, assign28580_e42716_d_n4, assign28580_e42716_d_n5, assign28580_e42716_d_n6, assign28580_e42716_d_n7, assign28580_e42716_d_n8, assign28580_e42716_d_n9, assign28580_e42716_d_n10, assign28580_e42716_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 != 0.0)) && (locals.var_guard598 != 0.0)) && (locals.var_guard599 == 0.0)) {
        let assign28580_e42710: f64 = (-p.p916);
        let assign28580_e42712: f64 = (locals.var_arg).ln();
        let assign28580_e42713: f64 = (assign28580_e42710 * assign28580_e42712);
        let assign28580_e42714: f64 = { let limited_exp_arg = assign28580_e42713; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign28580_e42714, ({ let limited_exp_arg = assign28580_e42713; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28580_e42710 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign28580_e42713; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28580_e42710 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign28580_e42713; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28580_e42710 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign28580_e42713; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28580_e42710 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign28580_e42713; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28580_e42710 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign28580_e42713; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28580_e42710 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign28580_e42713; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28580_e42710 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign28580_e42713; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28580_e42710 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign28580_e42713; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28580_e42710 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign28580_e42716;
        locals.var_sarg_dn3 = assign28580_e42716_d_n3;
        locals.var_sarg_dn4 = assign28580_e42716_d_n4;
        locals.var_sarg_dn5 = assign28580_e42716_d_n5;
        locals.var_sarg_dn6 = assign28580_e42716_d_n6;
        locals.var_sarg_dn7 = assign28580_e42716_d_n7;
        locals.var_sarg_dn8 = assign28580_e42716_d_n8;
        locals.var_sarg_dn9 = assign28580_e42716_d_n9;
        locals.var_sarg_dn10 = assign28580_e42716_d_n10;
        locals.var_sarg_dn11 = assign28580_e42716_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign28590_e42738, assign28590_e42738_d_n3, assign28590_e42738_d_n4, assign28590_e42738_d_n5, assign28590_e42738_d_n6, assign28590_e42738_d_n7, assign28590_e42738_d_n8, assign28590_e42738_d_n9, assign28590_e42738_d_n10, assign28590_e42738_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 != 0.0)) && (locals.var_guard598 != 0.0)) {
        let assign28590_e42726: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign28590_e42730: f64 = (locals.var_arg * locals.var_sarg);
        let assign28590_e42731: f64 = (1.0 - assign28590_e42730);
        let assign28590_e42732: f64 = (assign28590_e42726 * assign28590_e42731);
        let assign28590_e42735: f64 = (1.0 - p.p916);
        let assign28590_e42736: f64 = (assign28590_e42732 / assign28590_e42735);
        (assign28590_e42736, ((((locals.var_pbswd_t * locals.var_czbdsw_dn3) * assign28590_e42731) + (assign28590_e42726 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign28590_e42735), (((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign28590_e42731) + (assign28590_e42726 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign28590_e42735), (((((locals.var_pbswd_t_dn5 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn5)) * assign28590_e42731) + (assign28590_e42726 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign28590_e42735), ((((locals.var_pbswd_t * locals.var_czbdsw_dn6) * assign28590_e42731) + (assign28590_e42726 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign28590_e42735), ((((locals.var_pbswd_t * locals.var_czbdsw_dn7) * assign28590_e42731) + (assign28590_e42726 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign28590_e42735), ((((locals.var_pbswd_t * locals.var_czbdsw_dn8) * assign28590_e42731) + (assign28590_e42726 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign28590_e42735), ((((locals.var_pbswd_t * locals.var_czbdsw_dn9) * assign28590_e42731) + (assign28590_e42726 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign28590_e42735), ((((locals.var_pbswd_t * locals.var_czbdsw_dn10) * assign28590_e42731) + (assign28590_e42726 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign28590_e42735), ((((locals.var_pbswd_t * locals.var_czbdsw_dn11) * assign28590_e42731) + (assign28590_e42726 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign28590_e42735),)
    } else {
        (locals.var_qbdj2, locals.var_qbdj2_dn3, locals.var_qbdj2_dn4, locals.var_qbdj2_dn5, locals.var_qbdj2_dn6, locals.var_qbdj2_dn7, locals.var_qbdj2_dn8, locals.var_qbdj2_dn9, locals.var_qbdj2_dn10, locals.var_qbdj2_dn11,)
    }
};
        locals.var_qbdj2 = assign28590_e42738;
        locals.var_qbdj2_dn3 = assign28590_e42738_d_n3;
        locals.var_qbdj2_dn4 = assign28590_e42738_d_n4;
        locals.var_qbdj2_dn5 = assign28590_e42738_d_n5;
        locals.var_qbdj2_dn6 = assign28590_e42738_d_n6;
        locals.var_qbdj2_dn7 = assign28590_e42738_d_n7;
        locals.var_qbdj2_dn8 = assign28590_e42738_d_n8;
        locals.var_qbdj2_dn9 = assign28590_e42738_d_n9;
        locals.var_qbdj2_dn10 = assign28590_e42738_d_n10;
        locals.var_qbdj2_dn11 = assign28590_e42738_d_n11;
        locals.var_qbdj2_rv = 0.0;

        let (assign28600_e42755, assign28600_e42755_d_n3, assign28600_e42755_d_n4, assign28600_e42755_d_n5, assign28600_e42755_d_n6, assign28600_e42755_d_n7, assign28600_e42755_d_n8, assign28600_e42755_d_n9, assign28600_e42755_d_n10, assign28600_e42755_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 != 0.0)) && (locals.var_guard598 == 0.0)) {
        let assign28600_e42749: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign28600_e42751: f64 = (locals.var_arg).ln();
        let assign28600_e42752: f64 = (-assign28600_e42751);
        let assign28600_e42753: f64 = (assign28600_e42749 * assign28600_e42752);
        (assign28600_e42753, (((locals.var_pbswd_t * locals.var_czbdsw_dn3) * assign28600_e42752) + (assign28600_e42749 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign28600_e42752) + (assign28600_e42749 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbswd_t_dn5 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn5)) * assign28600_e42752) + (assign28600_e42749 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn6) * assign28600_e42752) + (assign28600_e42749 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn7) * assign28600_e42752) + (assign28600_e42749 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn8) * assign28600_e42752) + (assign28600_e42749 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn9) * assign28600_e42752) + (assign28600_e42749 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn10) * assign28600_e42752) + (assign28600_e42749 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn11) * assign28600_e42752) + (assign28600_e42749 * (-(locals.var_arg_dn11 / locals.var_arg)))),)
    } else {
        (locals.var_qbdj2, locals.var_qbdj2_dn3, locals.var_qbdj2_dn4, locals.var_qbdj2_dn5, locals.var_qbdj2_dn6, locals.var_qbdj2_dn7, locals.var_qbdj2_dn8, locals.var_qbdj2_dn9, locals.var_qbdj2_dn10, locals.var_qbdj2_dn11,)
    }
};
        locals.var_qbdj2 = assign28600_e42755;
        locals.var_qbdj2_dn3 = assign28600_e42755_d_n3;
        locals.var_qbdj2_dn4 = assign28600_e42755_d_n4;
        locals.var_qbdj2_dn5 = assign28600_e42755_d_n5;
        locals.var_qbdj2_dn6 = assign28600_e42755_d_n6;
        locals.var_qbdj2_dn7 = assign28600_e42755_d_n7;
        locals.var_qbdj2_dn8 = assign28600_e42755_d_n8;
        locals.var_qbdj2_dn9 = assign28600_e42755_d_n9;
        locals.var_qbdj2_dn10 = assign28600_e42755_d_n10;
        locals.var_qbdj2_dn11 = assign28600_e42755_d_n11;
        locals.var_qbdj2_rv = 0.0;

        let (assign28610_e42780, assign28610_e42780_d_n3, assign28610_e42780_d_n4, assign28610_e42780_d_n5, assign28610_e42780_d_n6, assign28610_e42780_d_n7, assign28610_e42780_d_n8, assign28610_e42780_d_n9, assign28610_e42780_d_n10, assign28610_e42780_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 == 0.0)) {
        let assign28610_e42765: f64 = (locals.var_t1 - 1.0);
        let assign28610_e42766: f64 = (locals.var_czbdsw_p1 * assign28610_e42765);
        let assign28610_e42769: f64 = (5.0 * p.p916);
        let assign28610_e42772: f64 = (locals.var_t1 - 1.0);
        let assign28610_e42773: f64 = (assign28610_e42769 * assign28610_e42772);
        let assign28610_e42776: f64 = (1.0 + p.p916);
        let assign28610_e42777: f64 = (assign28610_e42773 + assign28610_e42776);
        let assign28610_e42778: f64 = (assign28610_e42766 * assign28610_e42777);
        (assign28610_e42778, (((locals.var_czbdsw_p1 * locals.var_t1_dn3) * assign28610_e42777) + (assign28610_e42766 * (assign28610_e42769 * locals.var_t1_dn3))), (((locals.var_czbdsw_p1 * locals.var_t1_dn4) * assign28610_e42777) + (assign28610_e42766 * (assign28610_e42769 * locals.var_t1_dn4))), (((locals.var_czbdsw_p1 * locals.var_t1_dn5) * assign28610_e42777) + (assign28610_e42766 * (assign28610_e42769 * locals.var_t1_dn5))), (((locals.var_czbdsw_p1 * locals.var_t1_dn6) * assign28610_e42777) + (assign28610_e42766 * (assign28610_e42769 * locals.var_t1_dn6))), (((locals.var_czbdsw_p1 * locals.var_t1_dn7) * assign28610_e42777) + (assign28610_e42766 * (assign28610_e42769 * locals.var_t1_dn7))), (((locals.var_czbdsw_p1 * locals.var_t1_dn8) * assign28610_e42777) + (assign28610_e42766 * (assign28610_e42769 * locals.var_t1_dn8))), (((locals.var_czbdsw_p1 * locals.var_t1_dn9) * assign28610_e42777) + (assign28610_e42766 * (assign28610_e42769 * locals.var_t1_dn9))), (((locals.var_czbdsw_p1 * locals.var_t1_dn10) * assign28610_e42777) + (assign28610_e42766 * (assign28610_e42769 * locals.var_t1_dn10))), (((locals.var_czbdsw_p1 * locals.var_t1_dn11) * assign28610_e42777) + (assign28610_e42766 * (assign28610_e42769 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign28610_e42780;
        locals.var_t2_dn3 = assign28610_e42780_d_n3;
        locals.var_t2_dn4 = assign28610_e42780_d_n4;
        locals.var_t2_dn5 = assign28610_e42780_d_n5;
        locals.var_t2_dn6 = assign28610_e42780_d_n6;
        locals.var_t2_dn7 = assign28610_e42780_d_n7;
        locals.var_t2_dn8 = assign28610_e42780_d_n8;
        locals.var_t2_dn9 = assign28610_e42780_d_n9;
        locals.var_t2_dn10 = assign28610_e42780_d_n10;
        locals.var_t2_dn11 = assign28610_e42780_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign28620_e42795, assign28620_e42795_d_n3, assign28620_e42795_d_n4, assign28620_e42795_d_n5, assign28620_e42795_d_n6, assign28620_e42795_d_n7, assign28620_e42795_d_n8, assign28620_e42795_d_n9, assign28620_e42795_d_n10, assign28620_e42795_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 == 0.0)) {
        let assign28620_e42789: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign28620_e42792: f64 = (locals.var_t2 + locals.var_czbdsw_p2);
        let assign28620_e42793: f64 = (assign28620_e42789 * assign28620_e42792);
        (assign28620_e42793, (((locals.var_pbswd_t * locals.var_czbdsw_dn3) * assign28620_e42792) + (assign28620_e42789 * locals.var_t2_dn3)), ((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign28620_e42792) + (assign28620_e42789 * locals.var_t2_dn4)), ((((locals.var_pbswd_t_dn5 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn5)) * assign28620_e42792) + (assign28620_e42789 * locals.var_t2_dn5)), (((locals.var_pbswd_t * locals.var_czbdsw_dn6) * assign28620_e42792) + (assign28620_e42789 * locals.var_t2_dn6)), (((locals.var_pbswd_t * locals.var_czbdsw_dn7) * assign28620_e42792) + (assign28620_e42789 * locals.var_t2_dn7)), (((locals.var_pbswd_t * locals.var_czbdsw_dn8) * assign28620_e42792) + (assign28620_e42789 * locals.var_t2_dn8)), (((locals.var_pbswd_t * locals.var_czbdsw_dn9) * assign28620_e42792) + (assign28620_e42789 * locals.var_t2_dn9)), (((locals.var_pbswd_t * locals.var_czbdsw_dn10) * assign28620_e42792) + (assign28620_e42789 * locals.var_t2_dn10)), (((locals.var_pbswd_t * locals.var_czbdsw_dn11) * assign28620_e42792) + (assign28620_e42789 * locals.var_t2_dn11)),)
    } else {
        (locals.var_qbdj2, locals.var_qbdj2_dn3, locals.var_qbdj2_dn4, locals.var_qbdj2_dn5, locals.var_qbdj2_dn6, locals.var_qbdj2_dn7, locals.var_qbdj2_dn8, locals.var_qbdj2_dn9, locals.var_qbdj2_dn10, locals.var_qbdj2_dn11,)
    }
};
        locals.var_qbdj2 = assign28620_e42795;
        locals.var_qbdj2_dn3 = assign28620_e42795_d_n3;
        locals.var_qbdj2_dn4 = assign28620_e42795_d_n4;
        locals.var_qbdj2_dn5 = assign28620_e42795_d_n5;
        locals.var_qbdj2_dn6 = assign28620_e42795_d_n6;
        locals.var_qbdj2_dn7 = assign28620_e42795_d_n7;
        locals.var_qbdj2_dn8 = assign28620_e42795_d_n8;
        locals.var_qbdj2_dn9 = assign28620_e42795_d_n9;
        locals.var_qbdj2_dn10 = assign28620_e42795_d_n10;
        locals.var_qbdj2_dn11 = assign28620_e42795_d_n11;
        locals.var_qbdj2_rv = 0.0;

        let (assign28630_e42802, assign28630_e42802_d_n3, assign28630_e42802_d_n4, assign28630_e42802_d_n5, assign28630_e42802_d_n6, assign28630_e42802_d_n7, assign28630_e42802_d_n8, assign28630_e42802_d_n9, assign28630_e42802_d_n10, assign28630_e42802_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard596 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdj2, locals.var_qbdj2_dn3, locals.var_qbdj2_dn4, locals.var_qbdj2_dn5, locals.var_qbdj2_dn6, locals.var_qbdj2_dn7, locals.var_qbdj2_dn8, locals.var_qbdj2_dn9, locals.var_qbdj2_dn10, locals.var_qbdj2_dn11,)
    }
};
        locals.var_qbdj2 = assign28630_e42802;
        locals.var_qbdj2_dn3 = assign28630_e42802_d_n3;
        locals.var_qbdj2_dn4 = assign28630_e42802_d_n4;
        locals.var_qbdj2_dn5 = assign28630_e42802_d_n5;
        locals.var_qbdj2_dn6 = assign28630_e42802_d_n6;
        locals.var_qbdj2_dn7 = assign28630_e42802_d_n7;
        locals.var_qbdj2_dn8 = assign28630_e42802_d_n8;
        locals.var_qbdj2_dn9 = assign28630_e42802_d_n9;
        locals.var_qbdj2_dn10 = assign28630_e42802_d_n10;
        locals.var_qbdj2_dn11 = assign28630_e42802_d_n11;
        locals.var_qbdj2_rv = 0.0;

        let assign28640_e42805: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard600 = assign28640_e42805;
        locals.var_guard600_rv = 0.0;

        let (assign28650_e42813, assign28650_e42813_d_n3, assign28650_e42813_d_n4, assign28650_e42813_d_n5, assign28650_e42813_d_n6, assign28650_e42813_d_n7, assign28650_e42813_d_n8, assign28650_e42813_d_n9, assign28650_e42813_d_n10, assign28650_e42813_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard600 != 0.0)) {
        let assign28650_e42811: f64 = (locals.var_vbd_jct / locals.var_pbswgd_t);
        (assign28650_e42811, 0.0, (-((locals.var_vbd_jct * locals.var_pbswgd_t_dn4) / (locals.var_pbswgd_t * locals.var_pbswgd_t))), (-((locals.var_vbd_jct * locals.var_pbswgd_t_dn5) / (locals.var_pbswgd_t * locals.var_pbswgd_t))), (locals.var_vbd_jct_dn6 / locals.var_pbswgd_t), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_pbswgd_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign28650_e42813;
        locals.var_t1_dn3 = assign28650_e42813_d_n3;
        locals.var_t1_dn4 = assign28650_e42813_d_n4;
        locals.var_t1_dn5 = assign28650_e42813_d_n5;
        locals.var_t1_dn6 = assign28650_e42813_d_n6;
        locals.var_t1_dn7 = assign28650_e42813_d_n7;
        locals.var_t1_dn8 = assign28650_e42813_d_n8;
        locals.var_t1_dn9 = assign28650_e42813_d_n9;
        locals.var_t1_dn10 = assign28650_e42813_d_n10;
        locals.var_t1_dn11 = assign28650_e42813_d_n11;
        locals.var_t1_rv = 0.0;

        let assign28660_e42816: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard601 = assign28660_e42816;
        locals.var_guard601_rv = 0.0;

        let (assign28670_e42826, assign28670_e42826_d_n3, assign28670_e42826_d_n4, assign28670_e42826_d_n5, assign28670_e42826_d_n6, assign28670_e42826_d_n7, assign28670_e42826_d_n8, assign28670_e42826_d_n9, assign28670_e42826_d_n10, assign28670_e42826_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard600 != 0.0)) && (locals.var_guard601 != 0.0)) {
        let assign28670_e42824: f64 = (1.0 - locals.var_t1);
        (assign28670_e42824, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign28670_e42826;
        locals.var_arg_dn3 = assign28670_e42826_d_n3;
        locals.var_arg_dn4 = assign28670_e42826_d_n4;
        locals.var_arg_dn5 = assign28670_e42826_d_n5;
        locals.var_arg_dn6 = assign28670_e42826_d_n6;
        locals.var_arg_dn7 = assign28670_e42826_d_n7;
        locals.var_arg_dn8 = assign28670_e42826_d_n8;
        locals.var_arg_dn9 = assign28670_e42826_d_n9;
        locals.var_arg_dn10 = assign28670_e42826_d_n10;
        locals.var_arg_dn11 = assign28670_e42826_d_n11;
        locals.var_arg_rv = 0.0;

        let assign28680_e42829: f64 = if p.p918 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard602 = assign28680_e42829;
        locals.var_guard602_rv = 0.0;

        let assign28690_e42832: f64 = if p.p918 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard603 = assign28690_e42832;
        locals.var_guard603_rv = 0.0;

        let (assign28700_e42847, assign28700_e42847_d_n3, assign28700_e42847_d_n4, assign28700_e42847_d_n5, assign28700_e42847_d_n6, assign28700_e42847_d_n7, assign28700_e42847_d_n8, assign28700_e42847_d_n9, assign28700_e42847_d_n10, assign28700_e42847_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard600 != 0.0)) && (locals.var_guard601 != 0.0)) && (locals.var_guard602 != 0.0)) && (locals.var_guard603 != 0.0)) {
        let assign28700_e42844: f64 = (locals.var_arg).sqrt();
        let assign28700_e42845: f64 = (1.0 / assign28700_e42844);
        (assign28700_e42845, (-((locals.var_arg_dn3 / (2.0 * assign28700_e42844)) / (assign28700_e42844 * assign28700_e42844))), (-((locals.var_arg_dn4 / (2.0 * assign28700_e42844)) / (assign28700_e42844 * assign28700_e42844))), (-((locals.var_arg_dn5 / (2.0 * assign28700_e42844)) / (assign28700_e42844 * assign28700_e42844))), (-((locals.var_arg_dn6 / (2.0 * assign28700_e42844)) / (assign28700_e42844 * assign28700_e42844))), (-((locals.var_arg_dn7 / (2.0 * assign28700_e42844)) / (assign28700_e42844 * assign28700_e42844))), (-((locals.var_arg_dn8 / (2.0 * assign28700_e42844)) / (assign28700_e42844 * assign28700_e42844))), (-((locals.var_arg_dn9 / (2.0 * assign28700_e42844)) / (assign28700_e42844 * assign28700_e42844))), (-((locals.var_arg_dn10 / (2.0 * assign28700_e42844)) / (assign28700_e42844 * assign28700_e42844))), (-((locals.var_arg_dn11 / (2.0 * assign28700_e42844)) / (assign28700_e42844 * assign28700_e42844))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign28700_e42847;
        locals.var_sarg_dn3 = assign28700_e42847_d_n3;
        locals.var_sarg_dn4 = assign28700_e42847_d_n4;
        locals.var_sarg_dn5 = assign28700_e42847_d_n5;
        locals.var_sarg_dn6 = assign28700_e42847_d_n6;
        locals.var_sarg_dn7 = assign28700_e42847_d_n7;
        locals.var_sarg_dn8 = assign28700_e42847_d_n8;
        locals.var_sarg_dn9 = assign28700_e42847_d_n9;
        locals.var_sarg_dn10 = assign28700_e42847_d_n10;
        locals.var_sarg_dn11 = assign28700_e42847_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign28710_e42865, assign28710_e42865_d_n3, assign28710_e42865_d_n4, assign28710_e42865_d_n5, assign28710_e42865_d_n6, assign28710_e42865_d_n7, assign28710_e42865_d_n8, assign28710_e42865_d_n9, assign28710_e42865_d_n10, assign28710_e42865_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard600 != 0.0)) && (locals.var_guard601 != 0.0)) && (locals.var_guard602 != 0.0)) && (locals.var_guard603 == 0.0)) {
        let assign28710_e42859: f64 = (-p.p918);
        let assign28710_e42861: f64 = (locals.var_arg).ln();
        let assign28710_e42862: f64 = (assign28710_e42859 * assign28710_e42861);
        let assign28710_e42863: f64 = { let limited_exp_arg = assign28710_e42862; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign28710_e42863, ({ let limited_exp_arg = assign28710_e42862; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28710_e42859 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign28710_e42862; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28710_e42859 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign28710_e42862; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28710_e42859 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign28710_e42862; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28710_e42859 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign28710_e42862; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28710_e42859 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign28710_e42862; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28710_e42859 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign28710_e42862; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28710_e42859 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign28710_e42862; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28710_e42859 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign28710_e42862; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign28710_e42859 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign28710_e42865;
        locals.var_sarg_dn3 = assign28710_e42865_d_n3;
        locals.var_sarg_dn4 = assign28710_e42865_d_n4;
        locals.var_sarg_dn5 = assign28710_e42865_d_n5;
        locals.var_sarg_dn6 = assign28710_e42865_d_n6;
        locals.var_sarg_dn7 = assign28710_e42865_d_n7;
        locals.var_sarg_dn8 = assign28710_e42865_d_n8;
        locals.var_sarg_dn9 = assign28710_e42865_d_n9;
        locals.var_sarg_dn10 = assign28710_e42865_d_n10;
        locals.var_sarg_dn11 = assign28710_e42865_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign28720_e42887, assign28720_e42887_d_n3, assign28720_e42887_d_n4, assign28720_e42887_d_n5, assign28720_e42887_d_n6, assign28720_e42887_d_n7, assign28720_e42887_d_n8, assign28720_e42887_d_n9, assign28720_e42887_d_n10, assign28720_e42887_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard600 != 0.0)) && (locals.var_guard601 != 0.0)) && (locals.var_guard602 != 0.0)) {
        let assign28720_e42875: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign28720_e42879: f64 = (locals.var_arg * locals.var_sarg);
        let assign28720_e42880: f64 = (1.0 - assign28720_e42879);
        let assign28720_e42881: f64 = (assign28720_e42875 * assign28720_e42880);
        let assign28720_e42884: f64 = (1.0 - p.p918);
        let assign28720_e42885: f64 = (assign28720_e42881 / assign28720_e42884);
        (assign28720_e42885, ((assign28720_e42875 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign28720_e42884), (((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign28720_e42880) + (assign28720_e42875 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign28720_e42884), (((((locals.var_pbswgd_t_dn5 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn5)) * assign28720_e42880) + (assign28720_e42875 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign28720_e42884), ((assign28720_e42875 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign28720_e42884), ((assign28720_e42875 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7)))) / assign28720_e42884), ((assign28720_e42875 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8)))) / assign28720_e42884), ((assign28720_e42875 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9)))) / assign28720_e42884), ((assign28720_e42875 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10)))) / assign28720_e42884), ((assign28720_e42875 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11)))) / assign28720_e42884),)
    } else {
        (locals.var_qbdj3, locals.var_qbdj3_dn3, locals.var_qbdj3_dn4, locals.var_qbdj3_dn5, locals.var_qbdj3_dn6, locals.var_qbdj3_dn7, locals.var_qbdj3_dn8, locals.var_qbdj3_dn9, locals.var_qbdj3_dn10, locals.var_qbdj3_dn11,)
    }
};
        locals.var_qbdj3 = assign28720_e42887;
        locals.var_qbdj3_dn3 = assign28720_e42887_d_n3;
        locals.var_qbdj3_dn4 = assign28720_e42887_d_n4;
        locals.var_qbdj3_dn5 = assign28720_e42887_d_n5;
        locals.var_qbdj3_dn6 = assign28720_e42887_d_n6;
        locals.var_qbdj3_dn7 = assign28720_e42887_d_n7;
        locals.var_qbdj3_dn8 = assign28720_e42887_d_n8;
        locals.var_qbdj3_dn9 = assign28720_e42887_d_n9;
        locals.var_qbdj3_dn10 = assign28720_e42887_d_n10;
        locals.var_qbdj3_dn11 = assign28720_e42887_d_n11;
        locals.var_qbdj3_rv = 0.0;

        let (assign28730_e42904, assign28730_e42904_d_n3, assign28730_e42904_d_n4, assign28730_e42904_d_n5, assign28730_e42904_d_n6, assign28730_e42904_d_n7, assign28730_e42904_d_n8, assign28730_e42904_d_n9, assign28730_e42904_d_n10, assign28730_e42904_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard600 != 0.0)) && (locals.var_guard601 != 0.0)) && (locals.var_guard602 == 0.0)) {
        let assign28730_e42898: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign28730_e42900: f64 = (locals.var_arg).ln();
        let assign28730_e42901: f64 = (-assign28730_e42900);
        let assign28730_e42902: f64 = (assign28730_e42898 * assign28730_e42901);
        (assign28730_e42902, (assign28730_e42898 * (-(locals.var_arg_dn3 / locals.var_arg))), ((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign28730_e42901) + (assign28730_e42898 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbswgd_t_dn5 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn5)) * assign28730_e42901) + (assign28730_e42898 * (-(locals.var_arg_dn5 / locals.var_arg)))), (assign28730_e42898 * (-(locals.var_arg_dn6 / locals.var_arg))), (assign28730_e42898 * (-(locals.var_arg_dn7 / locals.var_arg))), (assign28730_e42898 * (-(locals.var_arg_dn8 / locals.var_arg))), (assign28730_e42898 * (-(locals.var_arg_dn9 / locals.var_arg))), (assign28730_e42898 * (-(locals.var_arg_dn10 / locals.var_arg))), (assign28730_e42898 * (-(locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_qbdj3, locals.var_qbdj3_dn3, locals.var_qbdj3_dn4, locals.var_qbdj3_dn5, locals.var_qbdj3_dn6, locals.var_qbdj3_dn7, locals.var_qbdj3_dn8, locals.var_qbdj3_dn9, locals.var_qbdj3_dn10, locals.var_qbdj3_dn11,)
    }
};
        locals.var_qbdj3 = assign28730_e42904;
        locals.var_qbdj3_dn3 = assign28730_e42904_d_n3;
        locals.var_qbdj3_dn4 = assign28730_e42904_d_n4;
        locals.var_qbdj3_dn5 = assign28730_e42904_d_n5;
        locals.var_qbdj3_dn6 = assign28730_e42904_d_n6;
        locals.var_qbdj3_dn7 = assign28730_e42904_d_n7;
        locals.var_qbdj3_dn8 = assign28730_e42904_d_n8;
        locals.var_qbdj3_dn9 = assign28730_e42904_d_n9;
        locals.var_qbdj3_dn10 = assign28730_e42904_d_n10;
        locals.var_qbdj3_dn11 = assign28730_e42904_d_n11;
        locals.var_qbdj3_rv = 0.0;

        let (assign28740_e42929, assign28740_e42929_d_n3, assign28740_e42929_d_n4, assign28740_e42929_d_n5, assign28740_e42929_d_n6, assign28740_e42929_d_n7, assign28740_e42929_d_n8, assign28740_e42929_d_n9, assign28740_e42929_d_n10, assign28740_e42929_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard600 != 0.0)) && (locals.var_guard601 == 0.0)) {
        let assign28740_e42914: f64 = (locals.var_t1 - 1.0);
        let assign28740_e42915: f64 = (locals.var_czbdswg_p1 * assign28740_e42914);
        let assign28740_e42918: f64 = (5.0 * p.p918);
        let assign28740_e42921: f64 = (locals.var_t1 - 1.0);
        let assign28740_e42922: f64 = (assign28740_e42918 * assign28740_e42921);
        let assign28740_e42925: f64 = (1.0 + p.p918);
        let assign28740_e42926: f64 = (assign28740_e42922 + assign28740_e42925);
        let assign28740_e42927: f64 = (assign28740_e42915 * assign28740_e42926);
        (assign28740_e42927, (((locals.var_czbdswg_p1 * locals.var_t1_dn3) * assign28740_e42926) + (assign28740_e42915 * (assign28740_e42918 * locals.var_t1_dn3))), (((locals.var_czbdswg_p1 * locals.var_t1_dn4) * assign28740_e42926) + (assign28740_e42915 * (assign28740_e42918 * locals.var_t1_dn4))), (((locals.var_czbdswg_p1 * locals.var_t1_dn5) * assign28740_e42926) + (assign28740_e42915 * (assign28740_e42918 * locals.var_t1_dn5))), (((locals.var_czbdswg_p1 * locals.var_t1_dn6) * assign28740_e42926) + (assign28740_e42915 * (assign28740_e42918 * locals.var_t1_dn6))), (((locals.var_czbdswg_p1 * locals.var_t1_dn7) * assign28740_e42926) + (assign28740_e42915 * (assign28740_e42918 * locals.var_t1_dn7))), (((locals.var_czbdswg_p1 * locals.var_t1_dn8) * assign28740_e42926) + (assign28740_e42915 * (assign28740_e42918 * locals.var_t1_dn8))), (((locals.var_czbdswg_p1 * locals.var_t1_dn9) * assign28740_e42926) + (assign28740_e42915 * (assign28740_e42918 * locals.var_t1_dn9))), (((locals.var_czbdswg_p1 * locals.var_t1_dn10) * assign28740_e42926) + (assign28740_e42915 * (assign28740_e42918 * locals.var_t1_dn10))), (((locals.var_czbdswg_p1 * locals.var_t1_dn11) * assign28740_e42926) + (assign28740_e42915 * (assign28740_e42918 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign28740_e42929;
        locals.var_t2_dn3 = assign28740_e42929_d_n3;
        locals.var_t2_dn4 = assign28740_e42929_d_n4;
        locals.var_t2_dn5 = assign28740_e42929_d_n5;
        locals.var_t2_dn6 = assign28740_e42929_d_n6;
        locals.var_t2_dn7 = assign28740_e42929_d_n7;
        locals.var_t2_dn8 = assign28740_e42929_d_n8;
        locals.var_t2_dn9 = assign28740_e42929_d_n9;
        locals.var_t2_dn10 = assign28740_e42929_d_n10;
        locals.var_t2_dn11 = assign28740_e42929_d_n11;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_82(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28750_e42944, assign28750_e42944_d_n3, assign28750_e42944_d_n4, assign28750_e42944_d_n5, assign28750_e42944_d_n6, assign28750_e42944_d_n7, assign28750_e42944_d_n8, assign28750_e42944_d_n9, assign28750_e42944_d_n10, assign28750_e42944_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard600 != 0.0)) && (locals.var_guard601 == 0.0)) {
        let assign28750_e42938: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign28750_e42941: f64 = (locals.var_t2 + locals.var_czbdswg_p2);
        let assign28750_e42942: f64 = (assign28750_e42938 * assign28750_e42941);
        (assign28750_e42942, (assign28750_e42938 * locals.var_t2_dn3), ((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign28750_e42941) + (assign28750_e42938 * locals.var_t2_dn4)), ((((locals.var_pbswgd_t_dn5 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn5)) * assign28750_e42941) + (assign28750_e42938 * locals.var_t2_dn5)), (assign28750_e42938 * locals.var_t2_dn6), (assign28750_e42938 * locals.var_t2_dn7), (assign28750_e42938 * locals.var_t2_dn8), (assign28750_e42938 * locals.var_t2_dn9), (assign28750_e42938 * locals.var_t2_dn10), (assign28750_e42938 * locals.var_t2_dn11),)
    } else {
        (locals.var_qbdj3, locals.var_qbdj3_dn3, locals.var_qbdj3_dn4, locals.var_qbdj3_dn5, locals.var_qbdj3_dn6, locals.var_qbdj3_dn7, locals.var_qbdj3_dn8, locals.var_qbdj3_dn9, locals.var_qbdj3_dn10, locals.var_qbdj3_dn11,)
    }
};
        locals.var_qbdj3 = assign28750_e42944;
        locals.var_qbdj3_dn3 = assign28750_e42944_d_n3;
        locals.var_qbdj3_dn4 = assign28750_e42944_d_n4;
        locals.var_qbdj3_dn5 = assign28750_e42944_d_n5;
        locals.var_qbdj3_dn6 = assign28750_e42944_d_n6;
        locals.var_qbdj3_dn7 = assign28750_e42944_d_n7;
        locals.var_qbdj3_dn8 = assign28750_e42944_d_n8;
        locals.var_qbdj3_dn9 = assign28750_e42944_d_n9;
        locals.var_qbdj3_dn10 = assign28750_e42944_d_n10;
        locals.var_qbdj3_dn11 = assign28750_e42944_d_n11;
        locals.var_qbdj3_rv = 0.0;

        let (assign28760_e42951, assign28760_e42951_d_n3, assign28760_e42951_d_n4, assign28760_e42951_d_n5, assign28760_e42951_d_n6, assign28760_e42951_d_n7, assign28760_e42951_d_n8, assign28760_e42951_d_n9, assign28760_e42951_d_n10, assign28760_e42951_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard600 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdj3, locals.var_qbdj3_dn3, locals.var_qbdj3_dn4, locals.var_qbdj3_dn5, locals.var_qbdj3_dn6, locals.var_qbdj3_dn7, locals.var_qbdj3_dn8, locals.var_qbdj3_dn9, locals.var_qbdj3_dn10, locals.var_qbdj3_dn11,)
    }
};
        locals.var_qbdj3 = assign28760_e42951;
        locals.var_qbdj3_dn3 = assign28760_e42951_d_n3;
        locals.var_qbdj3_dn4 = assign28760_e42951_d_n4;
        locals.var_qbdj3_dn5 = assign28760_e42951_d_n5;
        locals.var_qbdj3_dn6 = assign28760_e42951_d_n6;
        locals.var_qbdj3_dn7 = assign28760_e42951_d_n7;
        locals.var_qbdj3_dn8 = assign28760_e42951_d_n8;
        locals.var_qbdj3_dn9 = assign28760_e42951_d_n9;
        locals.var_qbdj3_dn10 = assign28760_e42951_d_n10;
        locals.var_qbdj3_dn11 = assign28760_e42951_d_n11;
        locals.var_qbdj3_rv = 0.0;

        let (assign28770_e42959, assign28770_e42959_d_n3, assign28770_e42959_d_n4, assign28770_e42959_d_n5, assign28770_e42959_d_n6, assign28770_e42959_d_n7, assign28770_e42959_d_n8, assign28770_e42959_d_n9, assign28770_e42959_d_n10, assign28770_e42959_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28770_e42955: f64 = (p.p919 * locals.var_ibddif);
        let assign28770_e42957: f64 = (assign28770_e42955 * p.p2);
        (assign28770_e42957, ((p.p919 * locals.var_ibddif_dn3) * p.p2), ((p.p919 * locals.var_ibddif_dn4) * p.p2), ((p.p919 * locals.var_ibddif_dn5) * p.p2), ((p.p919 * locals.var_ibddif_dn6) * p.p2), ((p.p919 * locals.var_ibddif_dn7) * p.p2), ((p.p919 * locals.var_ibddif_dn8) * p.p2), ((p.p919 * locals.var_ibddif_dn9) * p.p2), ((p.p919 * locals.var_ibddif_dn10) * p.p2), ((p.p919 * locals.var_ibddif_dn11) * p.p2),)
    } else {
        (locals.var_qbdj4, locals.var_qbdj4_dn3, locals.var_qbdj4_dn4, locals.var_qbdj4_dn5, locals.var_qbdj4_dn6, locals.var_qbdj4_dn7, locals.var_qbdj4_dn8, locals.var_qbdj4_dn9, locals.var_qbdj4_dn10, locals.var_qbdj4_dn11,)
    }
};
        locals.var_qbdj4 = assign28770_e42959;
        locals.var_qbdj4_dn3 = assign28770_e42959_d_n3;
        locals.var_qbdj4_dn4 = assign28770_e42959_d_n4;
        locals.var_qbdj4_dn5 = assign28770_e42959_d_n5;
        locals.var_qbdj4_dn6 = assign28770_e42959_d_n6;
        locals.var_qbdj4_dn7 = assign28770_e42959_d_n7;
        locals.var_qbdj4_dn8 = assign28770_e42959_d_n8;
        locals.var_qbdj4_dn9 = assign28770_e42959_d_n9;
        locals.var_qbdj4_dn10 = assign28770_e42959_d_n10;
        locals.var_qbdj4_dn11 = assign28770_e42959_d_n11;
        locals.var_qbdj4_rv = 0.0;

        let (assign28780_e42969, assign28780_e42969_d_n3, assign28780_e42969_d_n4, assign28780_e42969_d_n5, assign28780_e42969_d_n6, assign28780_e42969_d_n7, assign28780_e42969_d_n8, assign28780_e42969_d_n9, assign28780_e42969_d_n10, assign28780_e42969_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28780_e42963: f64 = (locals.var_qbdj1 + locals.var_qbdj2);
        let assign28780_e42965: f64 = (assign28780_e42963 + locals.var_qbdj3);
        let assign28780_e42967: f64 = (assign28780_e42965 + locals.var_qbdj4);
        (assign28780_e42967, (((locals.var_qbdj1_dn3 + locals.var_qbdj2_dn3) + locals.var_qbdj3_dn3) + locals.var_qbdj4_dn3), (((locals.var_qbdj1_dn4 + locals.var_qbdj2_dn4) + locals.var_qbdj3_dn4) + locals.var_qbdj4_dn4), (((locals.var_qbdj1_dn5 + locals.var_qbdj2_dn5) + locals.var_qbdj3_dn5) + locals.var_qbdj4_dn5), (((locals.var_qbdj1_dn6 + locals.var_qbdj2_dn6) + locals.var_qbdj3_dn6) + locals.var_qbdj4_dn6), (((locals.var_qbdj1_dn7 + locals.var_qbdj2_dn7) + locals.var_qbdj3_dn7) + locals.var_qbdj4_dn7), (((locals.var_qbdj1_dn8 + locals.var_qbdj2_dn8) + locals.var_qbdj3_dn8) + locals.var_qbdj4_dn8), (((locals.var_qbdj1_dn9 + locals.var_qbdj2_dn9) + locals.var_qbdj3_dn9) + locals.var_qbdj4_dn9), (((locals.var_qbdj1_dn10 + locals.var_qbdj2_dn10) + locals.var_qbdj3_dn10) + locals.var_qbdj4_dn10), (((locals.var_qbdj1_dn11 + locals.var_qbdj2_dn11) + locals.var_qbdj3_dn11) + locals.var_qbdj4_dn11),)
    } else {
        (locals.var_qbdj, locals.var_qbdj_dn3, locals.var_qbdj_dn4, locals.var_qbdj_dn5, locals.var_qbdj_dn6, locals.var_qbdj_dn7, locals.var_qbdj_dn8, locals.var_qbdj_dn9, locals.var_qbdj_dn10, locals.var_qbdj_dn11,)
    }
};
        locals.var_qbdj = assign28780_e42969;
        locals.var_qbdj_dn3 = assign28780_e42969_d_n3;
        locals.var_qbdj_dn4 = assign28780_e42969_d_n4;
        locals.var_qbdj_dn5 = assign28780_e42969_d_n5;
        locals.var_qbdj_dn6 = assign28780_e42969_d_n6;
        locals.var_qbdj_dn7 = assign28780_e42969_d_n7;
        locals.var_qbdj_dn8 = assign28780_e42969_d_n8;
        locals.var_qbdj_dn9 = assign28780_e42969_d_n9;
        locals.var_qbdj_dn10 = assign28780_e42969_d_n10;
        locals.var_qbdj_dn11 = assign28780_e42969_d_n11;
        locals.var_qbdj_rv = 0.0;

        let assign28790_e42972: f64 = if locals.var_x7_s <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard604 = assign28790_e42972;
        locals.var_guard604_rv = 0.0;

        let (assign28800_e42978, assign28800_e42978_d_n3, assign28800_e42978_d_n4, assign28800_e42978_d_n5, assign28800_e42978_d_n6, assign28800_e42978_d_n7, assign28800_e42978_d_n8, assign28800_e42978_d_n9, assign28800_e42978_d_n10, assign28800_e42978_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard604 != 0.0)) {
        (locals.var_voxm, locals.var_voxm_dn3, locals.var_voxm_dn4, locals.var_voxm_dn5, locals.var_voxm_dn6, locals.var_voxm_dn7, locals.var_voxm_dn8, locals.var_voxm_dn9, locals.var_voxm_dn10, locals.var_voxm_dn11,)
    } else {
        (locals.var_qg, locals.var_qg_dn3, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn11,)
    }
};
        locals.var_qg = assign28800_e42978;
        locals.var_qg_dn3 = assign28800_e42978_d_n3;
        locals.var_qg_dn4 = assign28800_e42978_d_n4;
        locals.var_qg_dn5 = assign28800_e42978_d_n5;
        locals.var_qg_dn6 = assign28800_e42978_d_n6;
        locals.var_qg_dn7 = assign28800_e42978_d_n7;
        locals.var_qg_dn8 = assign28800_e42978_d_n8;
        locals.var_qg_dn9 = assign28800_e42978_d_n9;
        locals.var_qg_dn10 = assign28800_e42978_d_n10;
        locals.var_qg_dn11 = assign28800_e42978_d_n11;
        locals.var_qg_rv = 0.0;

        let (assign28810_e42984, assign28810_e42984_d_n3, assign28810_e42984_d_n4, assign28810_e42984_d_n5, assign28810_e42984_d_n6, assign28810_e42984_d_n7, assign28810_e42984_d_n8, assign28810_e42984_d_n9, assign28810_e42984_d_n10, assign28810_e42984_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard604 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd_1, locals.var_qd_1_dn3, locals.var_qd_1_dn4, locals.var_qd_1_dn5, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8, locals.var_qd_1_dn9, locals.var_qd_1_dn10, locals.var_qd_1_dn11,)
    }
};
        locals.var_qd_1 = assign28810_e42984;
        locals.var_qd_1_dn3 = assign28810_e42984_d_n3;
        locals.var_qd_1_dn4 = assign28810_e42984_d_n4;
        locals.var_qd_1_dn5 = assign28810_e42984_d_n5;
        locals.var_qd_1_dn6 = assign28810_e42984_d_n6;
        locals.var_qd_1_dn7 = assign28810_e42984_d_n7;
        locals.var_qd_1_dn8 = assign28810_e42984_d_n8;
        locals.var_qd_1_dn9 = assign28810_e42984_d_n9;
        locals.var_qd_1_dn10 = assign28810_e42984_d_n10;
        locals.var_qd_1_dn11 = assign28810_e42984_d_n11;
        locals.var_qd_1_rv = 0.0;

        let (assign28820_e42990, assign28820_e42990_d_n3, assign28820_e42990_d_n4, assign28820_e42990_d_n5, assign28820_e42990_d_n6, assign28820_e42990_d_n7, assign28820_e42990_d_n8, assign28820_e42990_d_n9, assign28820_e42990_d_n10, assign28820_e42990_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard604 != 0.0)) {
        (locals.var_qg, locals.var_qg_dn3, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn11,)
    } else {
        (locals.var_qb_2, locals.var_qb_2_dn3, locals.var_qb_2_dn4, locals.var_qb_2_dn5, locals.var_qb_2_dn6, locals.var_qb_2_dn7, locals.var_qb_2_dn8, locals.var_qb_2_dn9, locals.var_qb_2_dn10, locals.var_qb_2_dn11,)
    }
};
        locals.var_qb_2 = assign28820_e42990;
        locals.var_qb_2_dn3 = assign28820_e42990_d_n3;
        locals.var_qb_2_dn4 = assign28820_e42990_d_n4;
        locals.var_qb_2_dn5 = assign28820_e42990_d_n5;
        locals.var_qb_2_dn6 = assign28820_e42990_d_n6;
        locals.var_qb_2_dn7 = assign28820_e42990_d_n7;
        locals.var_qb_2_dn8 = assign28820_e42990_d_n8;
        locals.var_qb_2_dn9 = assign28820_e42990_d_n9;
        locals.var_qb_2_dn10 = assign28820_e42990_d_n10;
        locals.var_qb_2_dn11 = assign28820_e42990_d_n11;
        locals.var_qb_2_rv = 0.0;

        let (assign28830_e42996, assign28830_e42996_d_n3, assign28830_e42996_d_n4, assign28830_e42996_d_n5, assign28830_e42996_d_n6, assign28830_e42996_d_n7, assign28830_e42996_d_n8, assign28830_e42996_d_n9, assign28830_e42996_d_n10, assign28830_e42996_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard604 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qs_2, locals.var_qs_2_dn3, locals.var_qs_2_dn4, locals.var_qs_2_dn5, locals.var_qs_2_dn6, locals.var_qs_2_dn7, locals.var_qs_2_dn8, locals.var_qs_2_dn9, locals.var_qs_2_dn10, locals.var_qs_2_dn11,)
    }
};
        locals.var_qs_2 = assign28830_e42996;
        locals.var_qs_2_dn3 = assign28830_e42996_d_n3;
        locals.var_qs_2_dn4 = assign28830_e42996_d_n4;
        locals.var_qs_2_dn5 = assign28830_e42996_d_n5;
        locals.var_qs_2_dn6 = assign28830_e42996_d_n6;
        locals.var_qs_2_dn7 = assign28830_e42996_d_n7;
        locals.var_qs_2_dn8 = assign28830_e42996_d_n8;
        locals.var_qs_2_dn9 = assign28830_e42996_d_n9;
        locals.var_qs_2_dn10 = assign28830_e42996_d_n10;
        locals.var_qs_2_dn11 = assign28830_e42996_d_n11;
        locals.var_qs_2_rv = 0.0;

        let (assign28840_e43007, assign28840_e43007_d_n3, assign28840_e43007_d_n4, assign28840_e43007_d_n5, assign28840_e43007_d_n6, assign28840_e43007_d_n7, assign28840_e43007_d_n8, assign28840_e43007_d_n9, assign28840_e43007_d_n10, assign28840_e43007_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard604 == 0.0)) {
        let assign28840_e43004: f64 = (locals.var_dps / locals.var_h_fact);
        let assign28840_e43005: f64 = (0.5 * assign28840_e43004);
        (assign28840_e43005, (0.5 * (((locals.var_dps_dn3 * locals.var_h_fact) - (locals.var_dps * locals.var_h_fact_dn3)) / (locals.var_h_fact * locals.var_h_fact))), (0.5 * (((locals.var_dps_dn4 * locals.var_h_fact) - (locals.var_dps * locals.var_h_fact_dn4)) / (locals.var_h_fact * locals.var_h_fact))), (0.5 * (((locals.var_dps_dn5 * locals.var_h_fact) - (locals.var_dps * locals.var_h_fact_dn5)) / (locals.var_h_fact * locals.var_h_fact))), (0.5 * (((locals.var_dps_dn6 * locals.var_h_fact) - (locals.var_dps * locals.var_h_fact_dn6)) / (locals.var_h_fact * locals.var_h_fact))), (0.5 * (((locals.var_dps_dn7 * locals.var_h_fact) - (locals.var_dps * locals.var_h_fact_dn7)) / (locals.var_h_fact * locals.var_h_fact))), (0.5 * (((locals.var_dps_dn8 * locals.var_h_fact) - (locals.var_dps * locals.var_h_fact_dn8)) / (locals.var_h_fact * locals.var_h_fact))), (0.5 * (((locals.var_dps_dn9 * locals.var_h_fact) - (locals.var_dps * locals.var_h_fact_dn9)) / (locals.var_h_fact * locals.var_h_fact))), (0.5 * (((locals.var_dps_dn10 * locals.var_h_fact) - (locals.var_dps * locals.var_h_fact_dn10)) / (locals.var_h_fact * locals.var_h_fact))), (0.5 * (((locals.var_dps_dn11 * locals.var_h_fact) - (locals.var_dps * locals.var_h_fact_dn11)) / (locals.var_h_fact * locals.var_h_fact))),)
    } else {
        (locals.var_fj, locals.var_fj_dn3, locals.var_fj_dn4, locals.var_fj_dn5, locals.var_fj_dn6, locals.var_fj_dn7, locals.var_fj_dn8, locals.var_fj_dn9, locals.var_fj_dn10, locals.var_fj_dn11,)
    }
};
        locals.var_fj = assign28840_e43007;
        locals.var_fj_dn3 = assign28840_e43007_d_n3;
        locals.var_fj_dn4 = assign28840_e43007_d_n4;
        locals.var_fj_dn5 = assign28840_e43007_d_n5;
        locals.var_fj_dn6 = assign28840_e43007_d_n6;
        locals.var_fj_dn7 = assign28840_e43007_d_n7;
        locals.var_fj_dn8 = assign28840_e43007_d_n8;
        locals.var_fj_dn9 = assign28840_e43007_d_n9;
        locals.var_fj_dn10 = assign28840_e43007_d_n10;
        locals.var_fj_dn11 = assign28840_e43007_d_n11;
        locals.var_fj_rv = 0.0;

        let (assign28850_e43016, assign28850_e43016_d_n3, assign28850_e43016_d_n4, assign28850_e43016_d_n5, assign28850_e43016_d_n6, assign28850_e43016_d_n7, assign28850_e43016_d_n8, assign28850_e43016_d_n9, assign28850_e43016_d_n10, assign28850_e43016_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard604 == 0.0)) {
        let assign28850_e43014: f64 = (locals.var_fj * locals.var_fj);
        (assign28850_e43014, ((locals.var_fj_dn3 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn3)), ((locals.var_fj_dn4 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn4)), ((locals.var_fj_dn5 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn5)), ((locals.var_fj_dn6 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn6)), ((locals.var_fj_dn7 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn7)), ((locals.var_fj_dn8 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn8)), ((locals.var_fj_dn9 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn9)), ((locals.var_fj_dn10 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn10)), ((locals.var_fj_dn11 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn11)),)
    } else {
        (locals.var_fj2, locals.var_fj2_dn3, locals.var_fj2_dn4, locals.var_fj2_dn5, locals.var_fj2_dn6, locals.var_fj2_dn7, locals.var_fj2_dn8, locals.var_fj2_dn9, locals.var_fj2_dn10, locals.var_fj2_dn11,)
    }
};
        locals.var_fj2 = assign28850_e43016;
        locals.var_fj2_dn3 = assign28850_e43016_d_n3;
        locals.var_fj2_dn4 = assign28850_e43016_d_n4;
        locals.var_fj2_dn5 = assign28850_e43016_d_n5;
        locals.var_fj2_dn6 = assign28850_e43016_d_n6;
        locals.var_fj2_dn7 = assign28850_e43016_d_n7;
        locals.var_fj2_dn8 = assign28850_e43016_d_n8;
        locals.var_fj2_dn9 = assign28850_e43016_d_n9;
        locals.var_fj2_dn10 = assign28850_e43016_d_n10;
        locals.var_fj2_dn11 = assign28850_e43016_d_n11;
        locals.var_fj2_rv = 0.0;

        let (assign28860_e43033, assign28860_e43033_d_n3, assign28860_e43033_d_n4, assign28860_e43033_d_n5, assign28860_e43033_d_n6, assign28860_e43033_d_n7, assign28860_e43033_d_n8, assign28860_e43033_d_n9, assign28860_e43033_d_n10, assign28860_e43033_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard604 == 0.0)) {
        let assign28860_e43023: f64 = (1.0 - locals.var_ddl);
        let assign28860_e43028: f64 = (locals.var_alpha_dd * locals.var_dps);
        let assign28860_e43029: f64 = (0.5 * assign28860_e43028);
        let assign28860_e43030: f64 = (locals.var_qim - assign28860_e43029);
        let assign28860_e43031: f64 = (assign28860_e43023 * assign28860_e43030);
        (assign28860_e43031, (((-locals.var_ddl_dn3) * assign28860_e43030) + (assign28860_e43023 * (locals.var_qim_dn3 - (0.5 * ((locals.var_alpha_dd_dn3 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn3)))))), (((-locals.var_ddl_dn4) * assign28860_e43030) + (assign28860_e43023 * (locals.var_qim_dn4 - (0.5 * ((locals.var_alpha_dd_dn4 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn4)))))), (((-locals.var_ddl_dn5) * assign28860_e43030) + (assign28860_e43023 * (locals.var_qim_dn5 - (0.5 * ((locals.var_alpha_dd_dn5 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn5)))))), (((-locals.var_ddl_dn6) * assign28860_e43030) + (assign28860_e43023 * (locals.var_qim_dn6 - (0.5 * ((locals.var_alpha_dd_dn6 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn6)))))), (((-locals.var_ddl_dn7) * assign28860_e43030) + (assign28860_e43023 * (locals.var_qim_dn7 - (0.5 * ((locals.var_alpha_dd_dn7 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn7)))))), (((-locals.var_ddl_dn8) * assign28860_e43030) + (assign28860_e43023 * (locals.var_qim_dn8 - (0.5 * ((locals.var_alpha_dd_dn8 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn8)))))), (((-locals.var_ddl_dn9) * assign28860_e43030) + (assign28860_e43023 * (locals.var_qim_dn9 - (0.5 * ((locals.var_alpha_dd_dn9 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn9)))))), (((-locals.var_ddl_dn10) * assign28860_e43030) + (assign28860_e43023 * (locals.var_qim_dn10 - (0.5 * ((locals.var_alpha_dd_dn10 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn10)))))), (((-locals.var_ddl_dn11) * assign28860_e43030) + (assign28860_e43023 * (locals.var_qim_dn11 - (0.5 * ((locals.var_alpha_dd_dn11 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn11)))))),)
    } else {
        (locals.var_qclm, locals.var_qclm_dn3, locals.var_qclm_dn4, locals.var_qclm_dn5, locals.var_qclm_dn6, locals.var_qclm_dn7, locals.var_qclm_dn8, locals.var_qclm_dn9, locals.var_qclm_dn10, locals.var_qclm_dn11,)
    }
};
        locals.var_qclm = assign28860_e43033;
        locals.var_qclm_dn3 = assign28860_e43033_d_n3;
        locals.var_qclm_dn4 = assign28860_e43033_d_n4;
        locals.var_qclm_dn5 = assign28860_e43033_d_n5;
        locals.var_qclm_dn6 = assign28860_e43033_d_n6;
        locals.var_qclm_dn7 = assign28860_e43033_d_n7;
        locals.var_qclm_dn8 = assign28860_e43033_d_n8;
        locals.var_qclm_dn9 = assign28860_e43033_d_n9;
        locals.var_qclm_dn10 = assign28860_e43033_d_n10;
        locals.var_qclm_dn11 = assign28860_e43033_d_n11;
        locals.var_qclm_rv = 0.0;

        let (assign28870_e43056, assign28870_e43056_d_n3, assign28870_e43056_d_n4, assign28870_e43056_d_n5, assign28870_e43056_d_n6, assign28870_e43056_d_n7, assign28870_e43056_d_n8, assign28870_e43056_d_n9, assign28870_e43056_d_n10, assign28870_e43056_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard604 == 0.0)) {
        let assign28870_e43042: f64 = (locals.var_eta_p * locals.var_dps);
        let assign28870_e43045: f64 = (locals.var_fj * locals.var_ddl);
        let assign28870_e43047: f64 = (assign28870_e43045 * 0.3333333333333333);
        let assign28870_e43049: f64 = (assign28870_e43047 - 1.0);
        let assign28870_e43051: f64 = (assign28870_e43049 + locals.var_ddl);
        let assign28870_e43052: f64 = (assign28870_e43042 * assign28870_e43051);
        let assign28870_e43053: f64 = (0.5 * assign28870_e43052);
        let assign28870_e43054: f64 = (locals.var_voxm + assign28870_e43053);
        (assign28870_e43054, (locals.var_voxm_dn3 + (0.5 * ((((locals.var_eta_p_dn3 * locals.var_dps) + (locals.var_eta_p * locals.var_dps_dn3)) * assign28870_e43051) + (assign28870_e43042 * ((((locals.var_fj_dn3 * locals.var_ddl) + (locals.var_fj * locals.var_ddl_dn3)) * 0.3333333333333333) + locals.var_ddl_dn3))))), (locals.var_voxm_dn4 + (0.5 * ((((locals.var_eta_p_dn4 * locals.var_dps) + (locals.var_eta_p * locals.var_dps_dn4)) * assign28870_e43051) + (assign28870_e43042 * ((((locals.var_fj_dn4 * locals.var_ddl) + (locals.var_fj * locals.var_ddl_dn4)) * 0.3333333333333333) + locals.var_ddl_dn4))))), (locals.var_voxm_dn5 + (0.5 * ((((locals.var_eta_p_dn5 * locals.var_dps) + (locals.var_eta_p * locals.var_dps_dn5)) * assign28870_e43051) + (assign28870_e43042 * ((((locals.var_fj_dn5 * locals.var_ddl) + (locals.var_fj * locals.var_ddl_dn5)) * 0.3333333333333333) + locals.var_ddl_dn5))))), (locals.var_voxm_dn6 + (0.5 * ((((locals.var_eta_p_dn6 * locals.var_dps) + (locals.var_eta_p * locals.var_dps_dn6)) * assign28870_e43051) + (assign28870_e43042 * ((((locals.var_fj_dn6 * locals.var_ddl) + (locals.var_fj * locals.var_ddl_dn6)) * 0.3333333333333333) + locals.var_ddl_dn6))))), (locals.var_voxm_dn7 + (0.5 * ((((locals.var_eta_p_dn7 * locals.var_dps) + (locals.var_eta_p * locals.var_dps_dn7)) * assign28870_e43051) + (assign28870_e43042 * ((((locals.var_fj_dn7 * locals.var_ddl) + (locals.var_fj * locals.var_ddl_dn7)) * 0.3333333333333333) + locals.var_ddl_dn7))))), (locals.var_voxm_dn8 + (0.5 * ((((locals.var_eta_p_dn8 * locals.var_dps) + (locals.var_eta_p * locals.var_dps_dn8)) * assign28870_e43051) + (assign28870_e43042 * ((((locals.var_fj_dn8 * locals.var_ddl) + (locals.var_fj * locals.var_ddl_dn8)) * 0.3333333333333333) + locals.var_ddl_dn8))))), (locals.var_voxm_dn9 + (0.5 * ((((locals.var_eta_p_dn9 * locals.var_dps) + (locals.var_eta_p * locals.var_dps_dn9)) * assign28870_e43051) + (assign28870_e43042 * ((((locals.var_fj_dn9 * locals.var_ddl) + (locals.var_fj * locals.var_ddl_dn9)) * 0.3333333333333333) + locals.var_ddl_dn9))))), (locals.var_voxm_dn10 + (0.5 * ((((locals.var_eta_p_dn10 * locals.var_dps) + (locals.var_eta_p * locals.var_dps_dn10)) * assign28870_e43051) + (assign28870_e43042 * ((((locals.var_fj_dn10 * locals.var_ddl) + (locals.var_fj * locals.var_ddl_dn10)) * 0.3333333333333333) + locals.var_ddl_dn10))))), (locals.var_voxm_dn11 + (0.5 * ((((locals.var_eta_p_dn11 * locals.var_dps) + (locals.var_eta_p * locals.var_dps_dn11)) * assign28870_e43051) + (assign28870_e43042 * ((((locals.var_fj_dn11 * locals.var_ddl) + (locals.var_fj * locals.var_ddl_dn11)) * 0.3333333333333333) + locals.var_ddl_dn11))))),)
    } else {
        (locals.var_qg, locals.var_qg_dn3, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn11,)
    }
};
        locals.var_qg = assign28870_e43056;
        locals.var_qg_dn3 = assign28870_e43056_d_n3;
        locals.var_qg_dn4 = assign28870_e43056_d_n4;
        locals.var_qg_dn5 = assign28870_e43056_d_n5;
        locals.var_qg_dn6 = assign28870_e43056_d_n6;
        locals.var_qg_dn7 = assign28870_e43056_d_n7;
        locals.var_qg_dn8 = assign28870_e43056_d_n8;
        locals.var_qg_dn9 = assign28870_e43056_d_n9;
        locals.var_qg_dn10 = assign28870_e43056_d_n10;
        locals.var_qg_dn11 = assign28870_e43056_d_n11;
        locals.var_qg_rv = 0.0;

        let (assign28880_e43067, assign28880_e43067_d_n3, assign28880_e43067_d_n4, assign28880_e43067_d_n5, assign28880_e43067_d_n6, assign28880_e43067_d_n7, assign28880_e43067_d_n8, assign28880_e43067_d_n9, assign28880_e43067_d_n10, assign28880_e43067_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard604 == 0.0)) {
        let assign28880_e43063: f64 = (locals.var_alpha_dd * locals.var_dps);
        let assign28880_e43065: f64 = (assign28880_e43063 * 0.16666666666666666);
        (assign28880_e43065, (((locals.var_alpha_dd_dn3 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn3)) * 0.16666666666666666), (((locals.var_alpha_dd_dn4 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn4)) * 0.16666666666666666), (((locals.var_alpha_dd_dn5 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn5)) * 0.16666666666666666), (((locals.var_alpha_dd_dn6 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn6)) * 0.16666666666666666), (((locals.var_alpha_dd_dn7 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn7)) * 0.16666666666666666), (((locals.var_alpha_dd_dn8 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn8)) * 0.16666666666666666), (((locals.var_alpha_dd_dn9 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn9)) * 0.16666666666666666), (((locals.var_alpha_dd_dn10 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn10)) * 0.16666666666666666), (((locals.var_alpha_dd_dn11 * locals.var_dps) + (locals.var_alpha_dd * locals.var_dps_dn11)) * 0.16666666666666666),)
    } else {
        (locals.var_tempc, locals.var_tempc_dn3, locals.var_tempc_dn4, locals.var_tempc_dn5, locals.var_tempc_dn6, locals.var_tempc_dn7, locals.var_tempc_dn8, locals.var_tempc_dn9, locals.var_tempc_dn10, locals.var_tempc_dn11,)
    }
};
        locals.var_tempc = assign28880_e43067;
        locals.var_tempc_dn3 = assign28880_e43067_d_n3;
        locals.var_tempc_dn4 = assign28880_e43067_d_n4;
        locals.var_tempc_dn5 = assign28880_e43067_d_n5;
        locals.var_tempc_dn6 = assign28880_e43067_d_n6;
        locals.var_tempc_dn7 = assign28880_e43067_d_n7;
        locals.var_tempc_dn8 = assign28880_e43067_d_n8;
        locals.var_tempc_dn9 = assign28880_e43067_d_n9;
        locals.var_tempc_dn10 = assign28880_e43067_d_n10;
        locals.var_tempc_dn11 = assign28880_e43067_d_n11;
        locals.var_tempc_rv = 0.0;

        let (assign28890_e43082, assign28890_e43082_d_n3, assign28890_e43082_d_n4, assign28890_e43082_d_n5, assign28890_e43082_d_n6, assign28890_e43082_d_n7, assign28890_e43082_d_n8, assign28890_e43082_d_n9, assign28890_e43082_d_n10, assign28890_e43082_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard604 == 0.0)) {
        let assign28890_e43076: f64 = (locals.var_tempc * locals.var_fj);
        let assign28890_e43077: f64 = (locals.var_qim + assign28890_e43076);
        let assign28890_e43078: f64 = (locals.var_ddl * assign28890_e43077);
        let assign28890_e43080: f64 = (assign28890_e43078 + locals.var_qclm);
        (assign28890_e43080, (((locals.var_ddl_dn3 * assign28890_e43077) + (locals.var_ddl * (locals.var_qim_dn3 + ((locals.var_tempc_dn3 * locals.var_fj) + (locals.var_tempc * locals.var_fj_dn3))))) + locals.var_qclm_dn3), (((locals.var_ddl_dn4 * assign28890_e43077) + (locals.var_ddl * (locals.var_qim_dn4 + ((locals.var_tempc_dn4 * locals.var_fj) + (locals.var_tempc * locals.var_fj_dn4))))) + locals.var_qclm_dn4), (((locals.var_ddl_dn5 * assign28890_e43077) + (locals.var_ddl * (locals.var_qim_dn5 + ((locals.var_tempc_dn5 * locals.var_fj) + (locals.var_tempc * locals.var_fj_dn5))))) + locals.var_qclm_dn5), (((locals.var_ddl_dn6 * assign28890_e43077) + (locals.var_ddl * (locals.var_qim_dn6 + ((locals.var_tempc_dn6 * locals.var_fj) + (locals.var_tempc * locals.var_fj_dn6))))) + locals.var_qclm_dn6), (((locals.var_ddl_dn7 * assign28890_e43077) + (locals.var_ddl * (locals.var_qim_dn7 + ((locals.var_tempc_dn7 * locals.var_fj) + (locals.var_tempc * locals.var_fj_dn7))))) + locals.var_qclm_dn7), (((locals.var_ddl_dn8 * assign28890_e43077) + (locals.var_ddl * (locals.var_qim_dn8 + ((locals.var_tempc_dn8 * locals.var_fj) + (locals.var_tempc * locals.var_fj_dn8))))) + locals.var_qclm_dn8), (((locals.var_ddl_dn9 * assign28890_e43077) + (locals.var_ddl * (locals.var_qim_dn9 + ((locals.var_tempc_dn9 * locals.var_fj) + (locals.var_tempc * locals.var_fj_dn9))))) + locals.var_qclm_dn9), (((locals.var_ddl_dn10 * assign28890_e43077) + (locals.var_ddl * (locals.var_qim_dn10 + ((locals.var_tempc_dn10 * locals.var_fj) + (locals.var_tempc * locals.var_fj_dn10))))) + locals.var_qclm_dn10), (((locals.var_ddl_dn11 * assign28890_e43077) + (locals.var_ddl * (locals.var_qim_dn11 + ((locals.var_tempc_dn11 * locals.var_fj) + (locals.var_tempc * locals.var_fj_dn11))))) + locals.var_qclm_dn11),)
    } else {
        (locals.var_qi, locals.var_qi_dn3, locals.var_qi_dn4, locals.var_qi_dn5, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn8, locals.var_qi_dn9, locals.var_qi_dn10, locals.var_qi_dn11,)
    }
};
        locals.var_qi = assign28890_e43082;
        locals.var_qi_dn3 = assign28890_e43082_d_n3;
        locals.var_qi_dn4 = assign28890_e43082_d_n4;
        locals.var_qi_dn5 = assign28890_e43082_d_n5;
        locals.var_qi_dn6 = assign28890_e43082_d_n6;
        locals.var_qi_dn7 = assign28890_e43082_d_n7;
        locals.var_qi_dn8 = assign28890_e43082_d_n8;
        locals.var_qi_dn9 = assign28890_e43082_d_n9;
        locals.var_qi_dn10 = assign28890_e43082_d_n10;
        locals.var_qi_dn11 = assign28890_e43082_d_n11;
        locals.var_qi_rv = 0.0;

        let (assign28900_e43111, assign28900_e43111_d_n3, assign28900_e43111_d_n4, assign28900_e43111_d_n5, assign28900_e43111_d_n6, assign28900_e43111_d_n7, assign28900_e43111_d_n8, assign28900_e43111_d_n9, assign28900_e43111_d_n10, assign28900_e43111_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard604 == 0.0)) {
        let assign28900_e43090: f64 = (locals.var_ddl * locals.var_ddl);
        let assign28900_e43095: f64 = (1.0 - locals.var_fj);
        let assign28900_e43098: f64 = (0.2 * locals.var_fj2);
        let assign28900_e43099: f64 = (assign28900_e43095 - assign28900_e43098);
        let assign28900_e43100: f64 = (locals.var_tempc * assign28900_e43099);
        let assign28900_e43101: f64 = (locals.var_qim - assign28900_e43100);
        let assign28900_e43102: f64 = (assign28900_e43090 * assign28900_e43101);
        let assign28900_e43106: f64 = (1.0 + locals.var_ddl);
        let assign28900_e43107: f64 = (locals.var_qclm * assign28900_e43106);
        let assign28900_e43108: f64 = (assign28900_e43102 + assign28900_e43107);
        let assign28900_e43109: f64 = (0.5 * assign28900_e43108);
        (assign28900_e43109, (0.5 * (((((locals.var_ddl_dn3 * locals.var_ddl) + (locals.var_ddl * locals.var_ddl_dn3)) * assign28900_e43101) + (assign28900_e43090 * (locals.var_qim_dn3 - ((locals.var_tempc_dn3 * assign28900_e43099) + (locals.var_tempc * ((-locals.var_fj_dn3) - (0.2 * locals.var_fj2_dn3))))))) + ((locals.var_qclm_dn3 * assign28900_e43106) + (locals.var_qclm * locals.var_ddl_dn3)))), (0.5 * (((((locals.var_ddl_dn4 * locals.var_ddl) + (locals.var_ddl * locals.var_ddl_dn4)) * assign28900_e43101) + (assign28900_e43090 * (locals.var_qim_dn4 - ((locals.var_tempc_dn4 * assign28900_e43099) + (locals.var_tempc * ((-locals.var_fj_dn4) - (0.2 * locals.var_fj2_dn4))))))) + ((locals.var_qclm_dn4 * assign28900_e43106) + (locals.var_qclm * locals.var_ddl_dn4)))), (0.5 * (((((locals.var_ddl_dn5 * locals.var_ddl) + (locals.var_ddl * locals.var_ddl_dn5)) * assign28900_e43101) + (assign28900_e43090 * (locals.var_qim_dn5 - ((locals.var_tempc_dn5 * assign28900_e43099) + (locals.var_tempc * ((-locals.var_fj_dn5) - (0.2 * locals.var_fj2_dn5))))))) + ((locals.var_qclm_dn5 * assign28900_e43106) + (locals.var_qclm * locals.var_ddl_dn5)))), (0.5 * (((((locals.var_ddl_dn6 * locals.var_ddl) + (locals.var_ddl * locals.var_ddl_dn6)) * assign28900_e43101) + (assign28900_e43090 * (locals.var_qim_dn6 - ((locals.var_tempc_dn6 * assign28900_e43099) + (locals.var_tempc * ((-locals.var_fj_dn6) - (0.2 * locals.var_fj2_dn6))))))) + ((locals.var_qclm_dn6 * assign28900_e43106) + (locals.var_qclm * locals.var_ddl_dn6)))), (0.5 * (((((locals.var_ddl_dn7 * locals.var_ddl) + (locals.var_ddl * locals.var_ddl_dn7)) * assign28900_e43101) + (assign28900_e43090 * (locals.var_qim_dn7 - ((locals.var_tempc_dn7 * assign28900_e43099) + (locals.var_tempc * ((-locals.var_fj_dn7) - (0.2 * locals.var_fj2_dn7))))))) + ((locals.var_qclm_dn7 * assign28900_e43106) + (locals.var_qclm * locals.var_ddl_dn7)))), (0.5 * (((((locals.var_ddl_dn8 * locals.var_ddl) + (locals.var_ddl * locals.var_ddl_dn8)) * assign28900_e43101) + (assign28900_e43090 * (locals.var_qim_dn8 - ((locals.var_tempc_dn8 * assign28900_e43099) + (locals.var_tempc * ((-locals.var_fj_dn8) - (0.2 * locals.var_fj2_dn8))))))) + ((locals.var_qclm_dn8 * assign28900_e43106) + (locals.var_qclm * locals.var_ddl_dn8)))), (0.5 * (((((locals.var_ddl_dn9 * locals.var_ddl) + (locals.var_ddl * locals.var_ddl_dn9)) * assign28900_e43101) + (assign28900_e43090 * (locals.var_qim_dn9 - ((locals.var_tempc_dn9 * assign28900_e43099) + (locals.var_tempc * ((-locals.var_fj_dn9) - (0.2 * locals.var_fj2_dn9))))))) + ((locals.var_qclm_dn9 * assign28900_e43106) + (locals.var_qclm * locals.var_ddl_dn9)))), (0.5 * (((((locals.var_ddl_dn10 * locals.var_ddl) + (locals.var_ddl * locals.var_ddl_dn10)) * assign28900_e43101) + (assign28900_e43090 * (locals.var_qim_dn10 - ((locals.var_tempc_dn10 * assign28900_e43099) + (locals.var_tempc * ((-locals.var_fj_dn10) - (0.2 * locals.var_fj2_dn10))))))) + ((locals.var_qclm_dn10 * assign28900_e43106) + (locals.var_qclm * locals.var_ddl_dn10)))), (0.5 * (((((locals.var_ddl_dn11 * locals.var_ddl) + (locals.var_ddl * locals.var_ddl_dn11)) * assign28900_e43101) + (assign28900_e43090 * (locals.var_qim_dn11 - ((locals.var_tempc_dn11 * assign28900_e43099) + (locals.var_tempc * ((-locals.var_fj_dn11) - (0.2 * locals.var_fj2_dn11))))))) + ((locals.var_qclm_dn11 * assign28900_e43106) + (locals.var_qclm * locals.var_ddl_dn11)))),)
    } else {
        (locals.var_qd_1, locals.var_qd_1_dn3, locals.var_qd_1_dn4, locals.var_qd_1_dn5, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8, locals.var_qd_1_dn9, locals.var_qd_1_dn10, locals.var_qd_1_dn11,)
    }
};
        locals.var_qd_1 = assign28900_e43111;
        locals.var_qd_1_dn3 = assign28900_e43111_d_n3;
        locals.var_qd_1_dn4 = assign28900_e43111_d_n4;
        locals.var_qd_1_dn5 = assign28900_e43111_d_n5;
        locals.var_qd_1_dn6 = assign28900_e43111_d_n6;
        locals.var_qd_1_dn7 = assign28900_e43111_d_n7;
        locals.var_qd_1_dn8 = assign28900_e43111_d_n8;
        locals.var_qd_1_dn9 = assign28900_e43111_d_n9;
        locals.var_qd_1_dn10 = assign28900_e43111_d_n10;
        locals.var_qd_1_dn11 = assign28900_e43111_d_n11;
        locals.var_qd_1_rv = 0.0;

        let (assign28910_e43120, assign28910_e43120_d_n3, assign28910_e43120_d_n4, assign28910_e43120_d_n5, assign28910_e43120_d_n6, assign28910_e43120_d_n7, assign28910_e43120_d_n8, assign28910_e43120_d_n9, assign28910_e43120_d_n10, assign28910_e43120_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard604 == 0.0)) {
        let assign28910_e43118: f64 = (locals.var_qg - locals.var_qi);
        (assign28910_e43118, (locals.var_qg_dn3 - locals.var_qi_dn3), (locals.var_qg_dn4 - locals.var_qi_dn4), (locals.var_qg_dn5 - locals.var_qi_dn5), (locals.var_qg_dn6 - locals.var_qi_dn6), (locals.var_qg_dn7 - locals.var_qi_dn7), (locals.var_qg_dn8 - locals.var_qi_dn8), (locals.var_qg_dn9 - locals.var_qi_dn9), (locals.var_qg_dn10 - locals.var_qi_dn10), (locals.var_qg_dn11 - locals.var_qi_dn11),)
    } else {
        (locals.var_qb_2, locals.var_qb_2_dn3, locals.var_qb_2_dn4, locals.var_qb_2_dn5, locals.var_qb_2_dn6, locals.var_qb_2_dn7, locals.var_qb_2_dn8, locals.var_qb_2_dn9, locals.var_qb_2_dn10, locals.var_qb_2_dn11,)
    }
};
        locals.var_qb_2 = assign28910_e43120;
        locals.var_qb_2_dn3 = assign28910_e43120_d_n3;
        locals.var_qb_2_dn4 = assign28910_e43120_d_n4;
        locals.var_qb_2_dn5 = assign28910_e43120_d_n5;
        locals.var_qb_2_dn6 = assign28910_e43120_d_n6;
        locals.var_qb_2_dn7 = assign28910_e43120_d_n7;
        locals.var_qb_2_dn8 = assign28910_e43120_d_n8;
        locals.var_qb_2_dn9 = assign28910_e43120_d_n9;
        locals.var_qb_2_dn10 = assign28910_e43120_d_n10;
        locals.var_qb_2_dn11 = assign28910_e43120_d_n11;
        locals.var_qb_2_rv = 0.0;

        let (assign28920_e43131, assign28920_e43131_d_n3, assign28920_e43131_d_n4, assign28920_e43131_d_n5, assign28920_e43131_d_n6, assign28920_e43131_d_n7, assign28920_e43131_d_n8, assign28920_e43131_d_n9, assign28920_e43131_d_n10, assign28920_e43131_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard604 == 0.0)) {
        let assign28920_e43127: f64 = (locals.var_qg - locals.var_qb_2);
        let assign28920_e43129: f64 = (assign28920_e43127 - locals.var_qd_1);
        (assign28920_e43129, ((locals.var_qg_dn3 - locals.var_qb_2_dn3) - locals.var_qd_1_dn3), ((locals.var_qg_dn4 - locals.var_qb_2_dn4) - locals.var_qd_1_dn4), ((locals.var_qg_dn5 - locals.var_qb_2_dn5) - locals.var_qd_1_dn5), ((locals.var_qg_dn6 - locals.var_qb_2_dn6) - locals.var_qd_1_dn6), ((locals.var_qg_dn7 - locals.var_qb_2_dn7) - locals.var_qd_1_dn7), ((locals.var_qg_dn8 - locals.var_qb_2_dn8) - locals.var_qd_1_dn8), ((locals.var_qg_dn9 - locals.var_qb_2_dn9) - locals.var_qd_1_dn9), ((locals.var_qg_dn10 - locals.var_qb_2_dn10) - locals.var_qd_1_dn10), ((locals.var_qg_dn11 - locals.var_qb_2_dn11) - locals.var_qd_1_dn11),)
    } else {
        (locals.var_qs_2, locals.var_qs_2_dn3, locals.var_qs_2_dn4, locals.var_qs_2_dn5, locals.var_qs_2_dn6, locals.var_qs_2_dn7, locals.var_qs_2_dn8, locals.var_qs_2_dn9, locals.var_qs_2_dn10, locals.var_qs_2_dn11,)
    }
};
        locals.var_qs_2 = assign28920_e43131;
        locals.var_qs_2_dn3 = assign28920_e43131_d_n3;
        locals.var_qs_2_dn4 = assign28920_e43131_d_n4;
        locals.var_qs_2_dn5 = assign28920_e43131_d_n5;
        locals.var_qs_2_dn6 = assign28920_e43131_d_n6;
        locals.var_qs_2_dn7 = assign28920_e43131_d_n7;
        locals.var_qs_2_dn8 = assign28920_e43131_d_n8;
        locals.var_qs_2_dn9 = assign28920_e43131_d_n9;
        locals.var_qs_2_dn10 = assign28920_e43131_d_n10;
        locals.var_qs_2_dn11 = assign28920_e43131_d_n11;
        locals.var_qs_2_rv = 0.0;

        let (assign28930_e43154, assign28930_e43154_d_n3, assign28930_e43154_d_n4, assign28930_e43154_d_n5, assign28930_e43154_d_n6, assign28930_e43154_d_n7, assign28930_e43154_d_n8, assign28930_e43154_d_n9, assign28930_e43154_d_n10, assign28930_e43154_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28930_e43136: f64 = locals.var_qb_2;
        let assign28930_e43139: f64 = locals.var_qb_2;
        let assign28930_e43142: f64 = locals.var_qb_2;
        let assign28930_e43143: f64 = (assign28930_e43139 * assign28930_e43142);
        let assign28930_e43146: f64 = (0.25 * 0.1);
        let assign28930_e43148: f64 = (assign28930_e43146 * 0.1);
        let assign28930_e43149: f64 = (assign28930_e43143 + assign28930_e43148);
        let assign28930_e43150: f64 = (assign28930_e43149).sqrt();
        let assign28930_e43151: f64 = (assign28930_e43136 + assign28930_e43150);
        let assign28930_e43152: f64 = (0.5 * assign28930_e43151);
        (assign28930_e43152, (0.5 * (locals.var_qb_2_dn3 + (((locals.var_qb_2_dn3 * assign28930_e43142) + (assign28930_e43139 * locals.var_qb_2_dn3)) / (2.0 * assign28930_e43150)))), (0.5 * (locals.var_qb_2_dn4 + (((locals.var_qb_2_dn4 * assign28930_e43142) + (assign28930_e43139 * locals.var_qb_2_dn4)) / (2.0 * assign28930_e43150)))), (0.5 * (locals.var_qb_2_dn5 + (((locals.var_qb_2_dn5 * assign28930_e43142) + (assign28930_e43139 * locals.var_qb_2_dn5)) / (2.0 * assign28930_e43150)))), (0.5 * (locals.var_qb_2_dn6 + (((locals.var_qb_2_dn6 * assign28930_e43142) + (assign28930_e43139 * locals.var_qb_2_dn6)) / (2.0 * assign28930_e43150)))), (0.5 * (locals.var_qb_2_dn7 + (((locals.var_qb_2_dn7 * assign28930_e43142) + (assign28930_e43139 * locals.var_qb_2_dn7)) / (2.0 * assign28930_e43150)))), (0.5 * (locals.var_qb_2_dn8 + (((locals.var_qb_2_dn8 * assign28930_e43142) + (assign28930_e43139 * locals.var_qb_2_dn8)) / (2.0 * assign28930_e43150)))), (0.5 * (locals.var_qb_2_dn9 + (((locals.var_qb_2_dn9 * assign28930_e43142) + (assign28930_e43139 * locals.var_qb_2_dn9)) / (2.0 * assign28930_e43150)))), (0.5 * (locals.var_qb_2_dn10 + (((locals.var_qb_2_dn10 * assign28930_e43142) + (assign28930_e43139 * locals.var_qb_2_dn10)) / (2.0 * assign28930_e43150)))), (0.5 * (locals.var_qb_2_dn11 + (((locals.var_qb_2_dn11 * assign28930_e43142) + (assign28930_e43139 * locals.var_qb_2_dn11)) / (2.0 * assign28930_e43150)))),)
    } else {
        (locals.var_qbacv, locals.var_qbacv_dn3, locals.var_qbacv_dn4, locals.var_qbacv_dn5, locals.var_qbacv_dn6, locals.var_qbacv_dn7, locals.var_qbacv_dn8, locals.var_qbacv_dn9, locals.var_qbacv_dn10, locals.var_qbacv_dn11,)
    }
};
        locals.var_qbacv = assign28930_e43154;
        locals.var_qbacv_dn3 = assign28930_e43154_d_n3;
        locals.var_qbacv_dn4 = assign28930_e43154_d_n4;
        locals.var_qbacv_dn5 = assign28930_e43154_d_n5;
        locals.var_qbacv_dn6 = assign28930_e43154_d_n6;
        locals.var_qbacv_dn7 = assign28930_e43154_d_n7;
        locals.var_qbacv_dn8 = assign28930_e43154_d_n8;
        locals.var_qbacv_dn9 = assign28930_e43154_d_n9;
        locals.var_qbacv_dn10 = assign28930_e43154_d_n10;
        locals.var_qbacv_dn11 = assign28930_e43154_d_n11;
        locals.var_qbacv_rv = 0.0;

        let (assign28940_e43160, assign28940_e43160_d_n3, assign28940_e43160_d_n4, assign28940_e43160_d_n5, assign28940_e43160_d_n6, assign28940_e43160_d_n7, assign28940_e43160_d_n8, assign28940_e43160_d_n9, assign28940_e43160_d_n10, assign28940_e43160_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28940_e43158: f64 = (locals.var_qs_2 + locals.var_qd_1);
        (assign28940_e43158, (locals.var_qs_2_dn3 + locals.var_qd_1_dn3), (locals.var_qs_2_dn4 + locals.var_qd_1_dn4), (locals.var_qs_2_dn5 + locals.var_qd_1_dn5), (locals.var_qs_2_dn6 + locals.var_qd_1_dn6), (locals.var_qs_2_dn7 + locals.var_qd_1_dn7), (locals.var_qs_2_dn8 + locals.var_qd_1_dn8), (locals.var_qs_2_dn9 + locals.var_qd_1_dn9), (locals.var_qs_2_dn10 + locals.var_qd_1_dn10), (locals.var_qs_2_dn11 + locals.var_qd_1_dn11),)
    } else {
        (locals.var_qiacv, locals.var_qiacv_dn3, locals.var_qiacv_dn4, locals.var_qiacv_dn5, locals.var_qiacv_dn6, locals.var_qiacv_dn7, locals.var_qiacv_dn8, locals.var_qiacv_dn9, locals.var_qiacv_dn10, locals.var_qiacv_dn11,)
    }
};
        locals.var_qiacv = assign28940_e43160;
        locals.var_qiacv_dn3 = assign28940_e43160_d_n3;
        locals.var_qiacv_dn4 = assign28940_e43160_d_n4;
        locals.var_qiacv_dn5 = assign28940_e43160_d_n5;
        locals.var_qiacv_dn6 = assign28940_e43160_d_n6;
        locals.var_qiacv_dn7 = assign28940_e43160_d_n7;
        locals.var_qiacv_dn8 = assign28940_e43160_d_n8;
        locals.var_qiacv_dn9 = assign28940_e43160_d_n9;
        locals.var_qiacv_dn10 = assign28940_e43160_d_n10;
        locals.var_qiacv_dn11 = assign28940_e43160_d_n11;
        locals.var_qiacv_rv = 0.0;

        let (assign28950_e43170, assign28950_e43170_d_n3, assign28950_e43170_d_n4, assign28950_e43170_d_n5, assign28950_e43170_d_n6, assign28950_e43170_d_n7, assign28950_e43170_d_n8, assign28950_e43170_d_n9, assign28950_e43170_d_n10, assign28950_e43170_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28950_e43165: f64 = (p.p231 * locals.var_qbacv);
        let assign28950_e43166: f64 = (locals.var_qiacv + assign28950_e43165);
        let assign28950_e43168: f64 = (assign28950_e43166 / p.p230);
        (assign28950_e43168, ((locals.var_qiacv_dn3 + (p.p231 * locals.var_qbacv_dn3)) / p.p230), ((locals.var_qiacv_dn4 + (p.p231 * locals.var_qbacv_dn4)) / p.p230), ((locals.var_qiacv_dn5 + (p.p231 * locals.var_qbacv_dn5)) / p.p230), ((locals.var_qiacv_dn6 + (p.p231 * locals.var_qbacv_dn6)) / p.p230), ((locals.var_qiacv_dn7 + (p.p231 * locals.var_qbacv_dn7)) / p.p230), ((locals.var_qiacv_dn8 + (p.p231 * locals.var_qbacv_dn8)) / p.p230), ((locals.var_qiacv_dn9 + (p.p231 * locals.var_qbacv_dn9)) / p.p230), ((locals.var_qiacv_dn10 + (p.p231 * locals.var_qbacv_dn10)) / p.p230), ((locals.var_qiacv_dn11 + (p.p231 * locals.var_qbacv_dn11)) / p.p230),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign28950_e43170;
        locals.var_t0_dn3 = assign28950_e43170_d_n3;
        locals.var_t0_dn4 = assign28950_e43170_d_n4;
        locals.var_t0_dn5 = assign28950_e43170_d_n5;
        locals.var_t0_dn6 = assign28950_e43170_d_n6;
        locals.var_t0_dn7 = assign28950_e43170_d_n7;
        locals.var_t0_dn8 = assign28950_e43170_d_n8;
        locals.var_t0_dn9 = assign28950_e43170_d_n9;
        locals.var_t0_dn10 = assign28950_e43170_d_n10;
        locals.var_t0_dn11 = assign28950_e43170_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign28960_e43187, assign28960_e43187_d_n3, assign28960_e43187_d_n4, assign28960_e43187_d_n5, assign28960_e43187_d_n6, assign28960_e43187_d_n7, assign28960_e43187_d_n8, assign28960_e43187_d_n9, assign28960_e43187_d_n10, assign28960_e43187_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28960_e43176: f64 = (locals.var_t0 * locals.var_t0);
        let assign28960_e43179: f64 = (4.0 * 0.001);
        let assign28960_e43181: f64 = (assign28960_e43179 * 0.001);
        let assign28960_e43182: f64 = (assign28960_e43176 + assign28960_e43181);
        let assign28960_e43183: f64 = (assign28960_e43182).sqrt();
        let assign28960_e43184: f64 = (locals.var_t0 + assign28960_e43183);
        let assign28960_e43185: f64 = (0.5 * assign28960_e43184);
        (assign28960_e43185, (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign28960_e43183)))), (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign28960_e43183)))), (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign28960_e43183)))), (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign28960_e43183)))), (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign28960_e43183)))), (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign28960_e43183)))), (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign28960_e43183)))), (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign28960_e43183)))), (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign28960_e43183)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign28960_e43187;
        locals.var_t0_dn3 = assign28960_e43187_d_n3;
        locals.var_t0_dn4 = assign28960_e43187_d_n4;
        locals.var_t0_dn5 = assign28960_e43187_d_n5;
        locals.var_t0_dn6 = assign28960_e43187_d_n6;
        locals.var_t0_dn7 = assign28960_e43187_d_n7;
        locals.var_t0_dn8 = assign28960_e43187_d_n8;
        locals.var_t0_dn9 = assign28960_e43187_d_n9;
        locals.var_t0_dn10 = assign28960_e43187_d_n10;
        locals.var_t0_dn11 = assign28960_e43187_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign28970_e43197, assign28970_e43197_d_n3, assign28970_e43197_d_n4, assign28970_e43197_d_n5, assign28970_e43197_d_n6, assign28970_e43197_d_n7, assign28970_e43197_d_n8, assign28970_e43197_d_n9, assign28970_e43197_d_n10, assign28970_e43197_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28970_e43193: f64 = (0.7 * p.p229);
        let assign28970_e43194: f64 = (locals.var_t0).powf(assign28970_e43193);
        let assign28970_e43195: f64 = (1.0 + assign28970_e43194);
        (assign28970_e43195, if 0.0 == 0.0 && ((assign28970_e43193) as f64).is_finite() && ((assign28970_e43193) as f64).fract() == 0.0 { if assign28970_e43193 == 0.0 { 0.0 } else { (assign28970_e43193 * ((locals.var_t0).powf(assign28970_e43193 - 1.0) * locals.var_t0_dn3)) } } else { (assign28970_e43194 * (assign28970_e43193 * (locals.var_t0_dn3 / locals.var_t0))) }, if 0.0 == 0.0 && ((assign28970_e43193) as f64).is_finite() && ((assign28970_e43193) as f64).fract() == 0.0 { if assign28970_e43193 == 0.0 { 0.0 } else { (assign28970_e43193 * ((locals.var_t0).powf(assign28970_e43193 - 1.0) * locals.var_t0_dn4)) } } else { (assign28970_e43194 * (assign28970_e43193 * (locals.var_t0_dn4 / locals.var_t0))) }, if 0.0 == 0.0 && ((assign28970_e43193) as f64).is_finite() && ((assign28970_e43193) as f64).fract() == 0.0 { if assign28970_e43193 == 0.0 { 0.0 } else { (assign28970_e43193 * ((locals.var_t0).powf(assign28970_e43193 - 1.0) * locals.var_t0_dn5)) } } else { (assign28970_e43194 * (assign28970_e43193 * (locals.var_t0_dn5 / locals.var_t0))) }, if 0.0 == 0.0 && ((assign28970_e43193) as f64).is_finite() && ((assign28970_e43193) as f64).fract() == 0.0 { if assign28970_e43193 == 0.0 { 0.0 } else { (assign28970_e43193 * ((locals.var_t0).powf(assign28970_e43193 - 1.0) * locals.var_t0_dn6)) } } else { (assign28970_e43194 * (assign28970_e43193 * (locals.var_t0_dn6 / locals.var_t0))) }, if 0.0 == 0.0 && ((assign28970_e43193) as f64).is_finite() && ((assign28970_e43193) as f64).fract() == 0.0 { if assign28970_e43193 == 0.0 { 0.0 } else { (assign28970_e43193 * ((locals.var_t0).powf(assign28970_e43193 - 1.0) * locals.var_t0_dn7)) } } else { (assign28970_e43194 * (assign28970_e43193 * (locals.var_t0_dn7 / locals.var_t0))) }, if 0.0 == 0.0 && ((assign28970_e43193) as f64).is_finite() && ((assign28970_e43193) as f64).fract() == 0.0 { if assign28970_e43193 == 0.0 { 0.0 } else { (assign28970_e43193 * ((locals.var_t0).powf(assign28970_e43193 - 1.0) * locals.var_t0_dn8)) } } else { (assign28970_e43194 * (assign28970_e43193 * (locals.var_t0_dn8 / locals.var_t0))) }, if 0.0 == 0.0 && ((assign28970_e43193) as f64).is_finite() && ((assign28970_e43193) as f64).fract() == 0.0 { if assign28970_e43193 == 0.0 { 0.0 } else { (assign28970_e43193 * ((locals.var_t0).powf(assign28970_e43193 - 1.0) * locals.var_t0_dn9)) } } else { (assign28970_e43194 * (assign28970_e43193 * (locals.var_t0_dn9 / locals.var_t0))) }, if 0.0 == 0.0 && ((assign28970_e43193) as f64).is_finite() && ((assign28970_e43193) as f64).fract() == 0.0 { if assign28970_e43193 == 0.0 { 0.0 } else { (assign28970_e43193 * ((locals.var_t0).powf(assign28970_e43193 - 1.0) * locals.var_t0_dn10)) } } else { (assign28970_e43194 * (assign28970_e43193 * (locals.var_t0_dn10 / locals.var_t0))) }, if 0.0 == 0.0 && ((assign28970_e43193) as f64).is_finite() && ((assign28970_e43193) as f64).fract() == 0.0 { if assign28970_e43193 == 0.0 { 0.0 } else { (assign28970_e43193 * ((locals.var_t0).powf(assign28970_e43193 - 1.0) * locals.var_t0_dn11)) } } else { (assign28970_e43194 * (assign28970_e43193 * (locals.var_t0_dn11 / locals.var_t0))) },)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign28970_e43197;
        locals.var_t1_dn3 = assign28970_e43197_d_n3;
        locals.var_t1_dn4 = assign28970_e43197_d_n4;
        locals.var_t1_dn5 = assign28970_e43197_d_n5;
        locals.var_t1_dn6 = assign28970_e43197_d_n6;
        locals.var_t1_dn7 = assign28970_e43197_d_n7;
        locals.var_t1_dn8 = assign28970_e43197_d_n8;
        locals.var_t1_dn9 = assign28970_e43197_d_n9;
        locals.var_t1_dn10 = assign28970_e43197_d_n10;
        locals.var_t1_dn11 = assign28970_e43197_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign28980_e43205, assign28980_e43205_d_n3, assign28980_e43205_d_n4, assign28980_e43205_d_n5, assign28980_e43205_d_n6, assign28980_e43205_d_n7, assign28980_e43205_d_n8, assign28980_e43205_d_n9, assign28980_e43205_d_n10, assign28980_e43205_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28980_e43201: f64 = (p.p228 * 1.9e-9);
        let assign28980_e43203: f64 = (assign28980_e43201 / locals.var_t1);
        (assign28980_e43203, (-((assign28980_e43201 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1))), (-((assign28980_e43201 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((assign28980_e43201 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((assign28980_e43201 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign28980_e43201 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((assign28980_e43201 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((assign28980_e43201 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((assign28980_e43201 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign28980_e43201 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_xdcinv, locals.var_xdcinv_dn3, locals.var_xdcinv_dn4, locals.var_xdcinv_dn5, locals.var_xdcinv_dn6, locals.var_xdcinv_dn7, locals.var_xdcinv_dn8, locals.var_xdcinv_dn9, locals.var_xdcinv_dn10, locals.var_xdcinv_dn11,)
    }
};
        locals.var_xdcinv = assign28980_e43205;
        locals.var_xdcinv_dn3 = assign28980_e43205_d_n3;
        locals.var_xdcinv_dn4 = assign28980_e43205_d_n4;
        locals.var_xdcinv_dn5 = assign28980_e43205_d_n5;
        locals.var_xdcinv_dn6 = assign28980_e43205_d_n6;
        locals.var_xdcinv_dn7 = assign28980_e43205_d_n7;
        locals.var_xdcinv_dn8 = assign28980_e43205_d_n8;
        locals.var_xdcinv_dn9 = assign28980_e43205_d_n9;
        locals.var_xdcinv_dn10 = assign28980_e43205_d_n10;
        locals.var_xdcinv_dn11 = assign28980_e43205_d_n11;
        locals.var_xdcinv_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_83(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign28990_e43221, assign28990_e43221_d_n3, assign28990_e43221_d_n4, assign28990_e43221_d_n5, assign28990_e43221_d_n6, assign28990_e43221_d_n7, assign28990_e43221_d_n8, assign28990_e43221_d_n9, assign28990_e43221_d_n10, assign28990_e43221_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign28990_e43209: f64 = (3.9 * 8.8541878128e-12);
        let assign28990_e43212: f64 = (locals.var_bsimbulktoxp * 3.9);
        let assign28990_e43214: f64 = (assign28990_e43212 / p.p110);
        let assign28990_e43217: f64 = (locals.var_xdcinv / locals.var_epsratio);
        let assign28990_e43218: f64 = (assign28990_e43214 + assign28990_e43217);
        let assign28990_e43219: f64 = (assign28990_e43209 / assign28990_e43218);
        (assign28990_e43219, (-((assign28990_e43209 * (locals.var_xdcinv_dn3 / locals.var_epsratio)) / (assign28990_e43218 * assign28990_e43218))), (-((assign28990_e43209 * (locals.var_xdcinv_dn4 / locals.var_epsratio)) / (assign28990_e43218 * assign28990_e43218))), (-((assign28990_e43209 * (locals.var_xdcinv_dn5 / locals.var_epsratio)) / (assign28990_e43218 * assign28990_e43218))), (-((assign28990_e43209 * (locals.var_xdcinv_dn6 / locals.var_epsratio)) / (assign28990_e43218 * assign28990_e43218))), (-((assign28990_e43209 * (locals.var_xdcinv_dn7 / locals.var_epsratio)) / (assign28990_e43218 * assign28990_e43218))), (-((assign28990_e43209 * (locals.var_xdcinv_dn8 / locals.var_epsratio)) / (assign28990_e43218 * assign28990_e43218))), (-((assign28990_e43209 * (locals.var_xdcinv_dn9 / locals.var_epsratio)) / (assign28990_e43218 * assign28990_e43218))), (-((assign28990_e43209 * (locals.var_xdcinv_dn10 / locals.var_epsratio)) / (assign28990_e43218 * assign28990_e43218))), (-((assign28990_e43209 * (locals.var_xdcinv_dn11 / locals.var_epsratio)) / (assign28990_e43218 * assign28990_e43218))),)
    } else {
        (locals.var_coxeffinv, locals.var_coxeffinv_dn3, locals.var_coxeffinv_dn4, locals.var_coxeffinv_dn5, locals.var_coxeffinv_dn6, locals.var_coxeffinv_dn7, locals.var_coxeffinv_dn8, locals.var_coxeffinv_dn9, locals.var_coxeffinv_dn10, locals.var_coxeffinv_dn11,)
    }
};
        locals.var_coxeffinv = assign28990_e43221;
        locals.var_coxeffinv_dn3 = assign28990_e43221_d_n3;
        locals.var_coxeffinv_dn4 = assign28990_e43221_d_n4;
        locals.var_coxeffinv_dn5 = assign28990_e43221_d_n5;
        locals.var_coxeffinv_dn6 = assign28990_e43221_d_n6;
        locals.var_coxeffinv_dn7 = assign28990_e43221_d_n7;
        locals.var_coxeffinv_dn8 = assign28990_e43221_d_n8;
        locals.var_coxeffinv_dn9 = assign28990_e43221_d_n9;
        locals.var_coxeffinv_dn10 = assign28990_e43221_d_n10;
        locals.var_coxeffinv_dn11 = assign28990_e43221_d_n11;
        locals.var_coxeffinv_rv = 0.0;

        let (assign29000_e43240, assign29000_e43240_d_n3, assign29000_e43240_d_n4, assign29000_e43240_d_n5, assign29000_e43240_d_n6, assign29000_e43240_d_n7, assign29000_e43240_d_n8, assign29000_e43240_d_n9, assign29000_e43240_d_n10, assign29000_e43240_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29000_e43225: f64 = (p.p2 * locals.var_wact);
        let assign29000_e43227: f64 = (assign29000_e43225 * locals.var_lact);
        let assign29000_e43229: f64 = (assign29000_e43227 + p.p1379);
        let assign29000_e43230: f64 = (-assign29000_e43229);
        let assign29000_e43233: f64 = (8.8541878128e-12 * p.p110);
        let assign29000_e43235: f64 = (assign29000_e43233 / locals.var_bsimbulktoxp);
        let assign29000_e43236: f64 = (assign29000_e43230 * assign29000_e43235);
        let assign29000_e43238: f64 = (assign29000_e43236 * locals.var_qb_2);
        (assign29000_e43238, (assign29000_e43236 * locals.var_qb_2_dn3), (assign29000_e43236 * locals.var_qb_2_dn4), (assign29000_e43236 * locals.var_qb_2_dn5), (assign29000_e43236 * locals.var_qb_2_dn6), (assign29000_e43236 * locals.var_qb_2_dn7), (assign29000_e43236 * locals.var_qb_2_dn8), (assign29000_e43236 * locals.var_qb_2_dn9), (assign29000_e43236 * locals.var_qb_2_dn10), (assign29000_e43236 * locals.var_qb_2_dn11),)
    } else {
        (locals.var_qbi, locals.var_qbi_dn3, locals.var_qbi_dn4, locals.var_qbi_dn5, locals.var_qbi_dn6, locals.var_qbi_dn7, locals.var_qbi_dn8, locals.var_qbi_dn9, locals.var_qbi_dn10, locals.var_qbi_dn11,)
    }
};
        locals.var_qbi = assign29000_e43240;
        locals.var_qbi_dn3 = assign29000_e43240_d_n3;
        locals.var_qbi_dn4 = assign29000_e43240_d_n4;
        locals.var_qbi_dn5 = assign29000_e43240_d_n5;
        locals.var_qbi_dn6 = assign29000_e43240_d_n6;
        locals.var_qbi_dn7 = assign29000_e43240_d_n7;
        locals.var_qbi_dn8 = assign29000_e43240_d_n8;
        locals.var_qbi_dn9 = assign29000_e43240_d_n9;
        locals.var_qbi_dn10 = assign29000_e43240_d_n10;
        locals.var_qbi_dn11 = assign29000_e43240_d_n11;
        locals.var_qbi_rv = 0.0;

        let (assign29010_e43252, assign29010_e43252_d_n3, assign29010_e43252_d_n4, assign29010_e43252_d_n5, assign29010_e43252_d_n6, assign29010_e43252_d_n7, assign29010_e43252_d_n8, assign29010_e43252_d_n9, assign29010_e43252_d_n10, assign29010_e43252_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29010_e43244: f64 = (p.p2 * locals.var_wact);
        let assign29010_e43246: f64 = (assign29010_e43244 * locals.var_lact);
        let assign29010_e43248: f64 = (assign29010_e43246 + p.p1379);
        let assign29010_e43250: f64 = (assign29010_e43248 * locals.var_coxeffinv);
        (assign29010_e43250, (assign29010_e43248 * locals.var_coxeffinv_dn3), (assign29010_e43248 * locals.var_coxeffinv_dn4), (assign29010_e43248 * locals.var_coxeffinv_dn5), (assign29010_e43248 * locals.var_coxeffinv_dn6), (assign29010_e43248 * locals.var_coxeffinv_dn7), (assign29010_e43248 * locals.var_coxeffinv_dn8), (assign29010_e43248 * locals.var_coxeffinv_dn9), (assign29010_e43248 * locals.var_coxeffinv_dn10), (assign29010_e43248 * locals.var_coxeffinv_dn11),)
    } else {
        (locals.var_wlcoxvtinv, locals.var_wlcoxvtinv_dn3, locals.var_wlcoxvtinv_dn4, locals.var_wlcoxvtinv_dn5, locals.var_wlcoxvtinv_dn6, locals.var_wlcoxvtinv_dn7, locals.var_wlcoxvtinv_dn8, locals.var_wlcoxvtinv_dn9, locals.var_wlcoxvtinv_dn10, locals.var_wlcoxvtinv_dn11,)
    }
};
        locals.var_wlcoxvtinv = assign29010_e43252;
        locals.var_wlcoxvtinv_dn3 = assign29010_e43252_d_n3;
        locals.var_wlcoxvtinv_dn4 = assign29010_e43252_d_n4;
        locals.var_wlcoxvtinv_dn5 = assign29010_e43252_d_n5;
        locals.var_wlcoxvtinv_dn6 = assign29010_e43252_d_n6;
        locals.var_wlcoxvtinv_dn7 = assign29010_e43252_d_n7;
        locals.var_wlcoxvtinv_dn8 = assign29010_e43252_d_n8;
        locals.var_wlcoxvtinv_dn9 = assign29010_e43252_d_n9;
        locals.var_wlcoxvtinv_dn10 = assign29010_e43252_d_n10;
        locals.var_wlcoxvtinv_dn11 = assign29010_e43252_d_n11;
        locals.var_wlcoxvtinv_rv = 0.0;

        let assign29020_e43255: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard605 = assign29020_e43255;
        locals.var_guard605_rv = 0.0;

        let (assign29030_e43264, assign29030_e43264_d_n3, assign29030_e43264_d_n4, assign29030_e43264_d_n5, assign29030_e43264_d_n6, assign29030_e43264_d_n7, assign29030_e43264_d_n8, assign29030_e43264_d_n9, assign29030_e43264_d_n10, assign29030_e43264_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard605 != 0.0)) {
        let assign29030_e43260: f64 = (-locals.var_wlcoxvtinv);
        let assign29030_e43262: f64 = (assign29030_e43260 * locals.var_qs_2);
        (assign29030_e43262, (((-locals.var_wlcoxvtinv_dn3) * locals.var_qs_2) + (assign29030_e43260 * locals.var_qs_2_dn3)), (((-locals.var_wlcoxvtinv_dn4) * locals.var_qs_2) + (assign29030_e43260 * locals.var_qs_2_dn4)), (((-locals.var_wlcoxvtinv_dn5) * locals.var_qs_2) + (assign29030_e43260 * locals.var_qs_2_dn5)), (((-locals.var_wlcoxvtinv_dn6) * locals.var_qs_2) + (assign29030_e43260 * locals.var_qs_2_dn6)), (((-locals.var_wlcoxvtinv_dn7) * locals.var_qs_2) + (assign29030_e43260 * locals.var_qs_2_dn7)), (((-locals.var_wlcoxvtinv_dn8) * locals.var_qs_2) + (assign29030_e43260 * locals.var_qs_2_dn8)), (((-locals.var_wlcoxvtinv_dn9) * locals.var_qs_2) + (assign29030_e43260 * locals.var_qs_2_dn9)), (((-locals.var_wlcoxvtinv_dn10) * locals.var_qs_2) + (assign29030_e43260 * locals.var_qs_2_dn10)), (((-locals.var_wlcoxvtinv_dn11) * locals.var_qs_2) + (assign29030_e43260 * locals.var_qs_2_dn11)),)
    } else {
        (locals.var_qsi, locals.var_qsi_dn3, locals.var_qsi_dn4, locals.var_qsi_dn5, locals.var_qsi_dn6, locals.var_qsi_dn7, locals.var_qsi_dn8, locals.var_qsi_dn9, locals.var_qsi_dn10, locals.var_qsi_dn11,)
    }
};
        locals.var_qsi = assign29030_e43264;
        locals.var_qsi_dn3 = assign29030_e43264_d_n3;
        locals.var_qsi_dn4 = assign29030_e43264_d_n4;
        locals.var_qsi_dn5 = assign29030_e43264_d_n5;
        locals.var_qsi_dn6 = assign29030_e43264_d_n6;
        locals.var_qsi_dn7 = assign29030_e43264_d_n7;
        locals.var_qsi_dn8 = assign29030_e43264_d_n8;
        locals.var_qsi_dn9 = assign29030_e43264_d_n9;
        locals.var_qsi_dn10 = assign29030_e43264_d_n10;
        locals.var_qsi_dn11 = assign29030_e43264_d_n11;
        locals.var_qsi_rv = 0.0;

        let (assign29040_e43273, assign29040_e43273_d_n3, assign29040_e43273_d_n4, assign29040_e43273_d_n5, assign29040_e43273_d_n6, assign29040_e43273_d_n7, assign29040_e43273_d_n8, assign29040_e43273_d_n9, assign29040_e43273_d_n10, assign29040_e43273_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard605 != 0.0)) {
        let assign29040_e43269: f64 = (-locals.var_wlcoxvtinv);
        let assign29040_e43271: f64 = (assign29040_e43269 * locals.var_qd_1);
        (assign29040_e43271, (((-locals.var_wlcoxvtinv_dn3) * locals.var_qd_1) + (assign29040_e43269 * locals.var_qd_1_dn3)), (((-locals.var_wlcoxvtinv_dn4) * locals.var_qd_1) + (assign29040_e43269 * locals.var_qd_1_dn4)), (((-locals.var_wlcoxvtinv_dn5) * locals.var_qd_1) + (assign29040_e43269 * locals.var_qd_1_dn5)), (((-locals.var_wlcoxvtinv_dn6) * locals.var_qd_1) + (assign29040_e43269 * locals.var_qd_1_dn6)), (((-locals.var_wlcoxvtinv_dn7) * locals.var_qd_1) + (assign29040_e43269 * locals.var_qd_1_dn7)), (((-locals.var_wlcoxvtinv_dn8) * locals.var_qd_1) + (assign29040_e43269 * locals.var_qd_1_dn8)), (((-locals.var_wlcoxvtinv_dn9) * locals.var_qd_1) + (assign29040_e43269 * locals.var_qd_1_dn9)), (((-locals.var_wlcoxvtinv_dn10) * locals.var_qd_1) + (assign29040_e43269 * locals.var_qd_1_dn10)), (((-locals.var_wlcoxvtinv_dn11) * locals.var_qd_1) + (assign29040_e43269 * locals.var_qd_1_dn11)),)
    } else {
        (locals.var_qdi, locals.var_qdi_dn3, locals.var_qdi_dn4, locals.var_qdi_dn5, locals.var_qdi_dn6, locals.var_qdi_dn7, locals.var_qdi_dn8, locals.var_qdi_dn9, locals.var_qdi_dn10, locals.var_qdi_dn11,)
    }
};
        locals.var_qdi = assign29040_e43273;
        locals.var_qdi_dn3 = assign29040_e43273_d_n3;
        locals.var_qdi_dn4 = assign29040_e43273_d_n4;
        locals.var_qdi_dn5 = assign29040_e43273_d_n5;
        locals.var_qdi_dn6 = assign29040_e43273_d_n6;
        locals.var_qdi_dn7 = assign29040_e43273_d_n7;
        locals.var_qdi_dn8 = assign29040_e43273_d_n8;
        locals.var_qdi_dn9 = assign29040_e43273_d_n9;
        locals.var_qdi_dn10 = assign29040_e43273_d_n10;
        locals.var_qdi_dn11 = assign29040_e43273_d_n11;
        locals.var_qdi_rv = 0.0;

        let (assign29050_e43283, assign29050_e43283_d_n3, assign29050_e43283_d_n4, assign29050_e43283_d_n5, assign29050_e43283_d_n6, assign29050_e43283_d_n7, assign29050_e43283_d_n8, assign29050_e43283_d_n9, assign29050_e43283_d_n10, assign29050_e43283_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard605 == 0.0)) {
        let assign29050_e43279: f64 = (-locals.var_wlcoxvtinv);
        let assign29050_e43281: f64 = (assign29050_e43279 * locals.var_qd_1);
        (assign29050_e43281, (((-locals.var_wlcoxvtinv_dn3) * locals.var_qd_1) + (assign29050_e43279 * locals.var_qd_1_dn3)), (((-locals.var_wlcoxvtinv_dn4) * locals.var_qd_1) + (assign29050_e43279 * locals.var_qd_1_dn4)), (((-locals.var_wlcoxvtinv_dn5) * locals.var_qd_1) + (assign29050_e43279 * locals.var_qd_1_dn5)), (((-locals.var_wlcoxvtinv_dn6) * locals.var_qd_1) + (assign29050_e43279 * locals.var_qd_1_dn6)), (((-locals.var_wlcoxvtinv_dn7) * locals.var_qd_1) + (assign29050_e43279 * locals.var_qd_1_dn7)), (((-locals.var_wlcoxvtinv_dn8) * locals.var_qd_1) + (assign29050_e43279 * locals.var_qd_1_dn8)), (((-locals.var_wlcoxvtinv_dn9) * locals.var_qd_1) + (assign29050_e43279 * locals.var_qd_1_dn9)), (((-locals.var_wlcoxvtinv_dn10) * locals.var_qd_1) + (assign29050_e43279 * locals.var_qd_1_dn10)), (((-locals.var_wlcoxvtinv_dn11) * locals.var_qd_1) + (assign29050_e43279 * locals.var_qd_1_dn11)),)
    } else {
        (locals.var_qsi, locals.var_qsi_dn3, locals.var_qsi_dn4, locals.var_qsi_dn5, locals.var_qsi_dn6, locals.var_qsi_dn7, locals.var_qsi_dn8, locals.var_qsi_dn9, locals.var_qsi_dn10, locals.var_qsi_dn11,)
    }
};
        locals.var_qsi = assign29050_e43283;
        locals.var_qsi_dn3 = assign29050_e43283_d_n3;
        locals.var_qsi_dn4 = assign29050_e43283_d_n4;
        locals.var_qsi_dn5 = assign29050_e43283_d_n5;
        locals.var_qsi_dn6 = assign29050_e43283_d_n6;
        locals.var_qsi_dn7 = assign29050_e43283_d_n7;
        locals.var_qsi_dn8 = assign29050_e43283_d_n8;
        locals.var_qsi_dn9 = assign29050_e43283_d_n9;
        locals.var_qsi_dn10 = assign29050_e43283_d_n10;
        locals.var_qsi_dn11 = assign29050_e43283_d_n11;
        locals.var_qsi_rv = 0.0;

        let (assign29060_e43293, assign29060_e43293_d_n3, assign29060_e43293_d_n4, assign29060_e43293_d_n5, assign29060_e43293_d_n6, assign29060_e43293_d_n7, assign29060_e43293_d_n8, assign29060_e43293_d_n9, assign29060_e43293_d_n10, assign29060_e43293_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard605 == 0.0)) {
        let assign29060_e43289: f64 = (-locals.var_wlcoxvtinv);
        let assign29060_e43291: f64 = (assign29060_e43289 * locals.var_qs_2);
        (assign29060_e43291, (((-locals.var_wlcoxvtinv_dn3) * locals.var_qs_2) + (assign29060_e43289 * locals.var_qs_2_dn3)), (((-locals.var_wlcoxvtinv_dn4) * locals.var_qs_2) + (assign29060_e43289 * locals.var_qs_2_dn4)), (((-locals.var_wlcoxvtinv_dn5) * locals.var_qs_2) + (assign29060_e43289 * locals.var_qs_2_dn5)), (((-locals.var_wlcoxvtinv_dn6) * locals.var_qs_2) + (assign29060_e43289 * locals.var_qs_2_dn6)), (((-locals.var_wlcoxvtinv_dn7) * locals.var_qs_2) + (assign29060_e43289 * locals.var_qs_2_dn7)), (((-locals.var_wlcoxvtinv_dn8) * locals.var_qs_2) + (assign29060_e43289 * locals.var_qs_2_dn8)), (((-locals.var_wlcoxvtinv_dn9) * locals.var_qs_2) + (assign29060_e43289 * locals.var_qs_2_dn9)), (((-locals.var_wlcoxvtinv_dn10) * locals.var_qs_2) + (assign29060_e43289 * locals.var_qs_2_dn10)), (((-locals.var_wlcoxvtinv_dn11) * locals.var_qs_2) + (assign29060_e43289 * locals.var_qs_2_dn11)),)
    } else {
        (locals.var_qdi, locals.var_qdi_dn3, locals.var_qdi_dn4, locals.var_qdi_dn5, locals.var_qdi_dn6, locals.var_qdi_dn7, locals.var_qdi_dn8, locals.var_qdi_dn9, locals.var_qdi_dn10, locals.var_qdi_dn11,)
    }
};
        locals.var_qdi = assign29060_e43293;
        locals.var_qdi_dn3 = assign29060_e43293_d_n3;
        locals.var_qdi_dn4 = assign29060_e43293_d_n4;
        locals.var_qdi_dn5 = assign29060_e43293_d_n5;
        locals.var_qdi_dn6 = assign29060_e43293_d_n6;
        locals.var_qdi_dn7 = assign29060_e43293_d_n7;
        locals.var_qdi_dn8 = assign29060_e43293_d_n8;
        locals.var_qdi_dn9 = assign29060_e43293_d_n9;
        locals.var_qdi_dn10 = assign29060_e43293_d_n10;
        locals.var_qdi_dn11 = assign29060_e43293_d_n11;
        locals.var_qdi_rv = 0.0;

        let (assign29070_e43302, assign29070_e43302_d_n3, assign29070_e43302_d_n4, assign29070_e43302_d_n5, assign29070_e43302_d_n6, assign29070_e43302_d_n7, assign29070_e43302_d_n8, assign29070_e43302_d_n9, assign29070_e43302_d_n10, assign29070_e43302_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29070_e43297: f64 = (locals.var_qbi + locals.var_qsi);
        let assign29070_e43299: f64 = (assign29070_e43297 + locals.var_qdi);
        let assign29070_e43300: f64 = (-assign29070_e43299);
        (assign29070_e43300, (-((locals.var_qbi_dn3 + locals.var_qsi_dn3) + locals.var_qdi_dn3)), (-((locals.var_qbi_dn4 + locals.var_qsi_dn4) + locals.var_qdi_dn4)), (-((locals.var_qbi_dn5 + locals.var_qsi_dn5) + locals.var_qdi_dn5)), (-((locals.var_qbi_dn6 + locals.var_qsi_dn6) + locals.var_qdi_dn6)), (-((locals.var_qbi_dn7 + locals.var_qsi_dn7) + locals.var_qdi_dn7)), (-((locals.var_qbi_dn8 + locals.var_qsi_dn8) + locals.var_qdi_dn8)), (-((locals.var_qbi_dn9 + locals.var_qsi_dn9) + locals.var_qdi_dn9)), (-((locals.var_qbi_dn10 + locals.var_qsi_dn10) + locals.var_qdi_dn10)), (-((locals.var_qbi_dn11 + locals.var_qsi_dn11) + locals.var_qdi_dn11)),)
    } else {
        (locals.var_qgi, locals.var_qgi_dn3, locals.var_qgi_dn4, locals.var_qgi_dn5, locals.var_qgi_dn6, locals.var_qgi_dn7, locals.var_qgi_dn8, locals.var_qgi_dn9, locals.var_qgi_dn10, locals.var_qgi_dn11,)
    }
};
        locals.var_qgi = assign29070_e43302;
        locals.var_qgi_dn3 = assign29070_e43302_d_n3;
        locals.var_qgi_dn4 = assign29070_e43302_d_n4;
        locals.var_qgi_dn5 = assign29070_e43302_d_n5;
        locals.var_qgi_dn6 = assign29070_e43302_d_n6;
        locals.var_qgi_dn7 = assign29070_e43302_d_n7;
        locals.var_qgi_dn8 = assign29070_e43302_d_n8;
        locals.var_qgi_dn9 = assign29070_e43302_d_n9;
        locals.var_qgi_dn10 = assign29070_e43302_d_n10;
        locals.var_qgi_dn11 = assign29070_e43302_d_n11;
        locals.var_qgi_rv = 0.0;

        let assign29080_e43305: f64 = if (!param_given[867]) { 1.0 } else { 0.0 };
        locals.var_guard606 = assign29080_e43305;
        locals.var_guard606_rv = 0.0;

        let (assign29090_e43328,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard606 != 0.0)) {
        let assign29090_e43311: f64 = (2.0 * p.p110);
        let assign29090_e43313: f64 = (assign29090_e43311 * 8.8541878128e-12);
        let assign29090_e43315: f64 = (assign29090_e43313 / 3.141592653589793);
        let assign29090_e43320: f64 = (4e-7 / p.p76);
        let assign29090_e43321: f64 = (1.0 + assign29090_e43320);
        let assign29090_e43322: f64 = (p.p871 * assign29090_e43321);
        let assign29090_e43324: f64 = (assign29090_e43322).max(1e-38);
        let assign29090_e43325: f64 = (assign29090_e43324).ln();
        let assign29090_e43326: f64 = (assign29090_e43315 * assign29090_e43325);
        (assign29090_e43326,)
    } else {
        (locals.var_cf_i,)
    }
};
        locals.var_cf_i = assign29090_e43328;
        locals.var_cf_i_rv = 0.0;

        let (assign29100_e43334,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29100_e43332: f64 = (p.p872 + locals.var_cf_i);
        (assign29100_e43332,)
    } else {
        (locals.var_cgsof,)
    }
};
        locals.var_cgsof = assign29100_e43334;
        locals.var_cgsof_rv = 0.0;

        let (assign29110_e43340,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29110_e43338: f64 = (p.p873 + locals.var_cf_i);
        (assign29110_e43338,)
    } else {
        (locals.var_cgdof,)
    }
};
        locals.var_cgdof = assign29110_e43340;
        locals.var_cgdof_rv = 0.0;

        let (assign29120_e43348,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29120_e43344: f64 = (locals.var_wact / p.p1373);
        let assign29120_e43346: f64 = (assign29120_e43344 + p.p1378);
        (assign29120_e43346,)
    } else {
        (locals.var_wdioscv,)
    }
};
        locals.var_wdioscv = assign29120_e43348;
        locals.var_wdioscv_rv = 0.0;

        let (assign29130_e43356,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29130_e43352: f64 = (locals.var_wact / p.p1373);
        let assign29130_e43354: f64 = (assign29130_e43352 + p.p1377);
        (assign29130_e43354,)
    } else {
        (locals.var_wdiodcv,)
    }
};
        locals.var_wdiodcv = assign29130_e43356;
        locals.var_wdiodcv_rv = 0.0;

        let assign29140_e43359: f64 = if p.p32 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard607 = assign29140_e43359;
        locals.var_guard607_rv = 0.0;

        let (assign29150_e43372, assign29150_e43372_d_n3, assign29150_e43372_d_n4, assign29150_e43372_d_n5, assign29150_e43372_d_n6, assign29150_e43372_d_n7, assign29150_e43372_d_n8, assign29150_e43372_d_n9, assign29150_e43372_d_n10, assign29150_e43372_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard607 != 0.0)) {
        let assign29150_e43364: f64 = (-locals.var_wdioscv);
        let assign29150_e43366: f64 = (assign29150_e43364 * p.p2);
        let assign29150_e43368: f64 = (assign29150_e43366 * locals.var_cgsof);
        let assign29150_e43370: f64 = (assign29150_e43368 * locals.var_vgs_ov_noswap);
        (assign29150_e43370, 0.0, 0.0, 0.0, 0.0, (assign29150_e43368 * locals.var_vgs_ov_noswap_dn7), 0.0, (assign29150_e43368 * locals.var_vgs_ov_noswap_dn9), 0.0, 0.0,)
    } else {
        (locals.var_qovs, locals.var_qovs_dn3, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11,)
    }
};
        locals.var_qovs = assign29150_e43372;
        locals.var_qovs_dn3 = assign29150_e43372_d_n3;
        locals.var_qovs_dn4 = assign29150_e43372_d_n4;
        locals.var_qovs_dn5 = assign29150_e43372_d_n5;
        locals.var_qovs_dn6 = assign29150_e43372_d_n6;
        locals.var_qovs_dn7 = assign29150_e43372_d_n7;
        locals.var_qovs_dn8 = assign29150_e43372_d_n8;
        locals.var_qovs_dn9 = assign29150_e43372_d_n9;
        locals.var_qovs_dn10 = assign29150_e43372_d_n10;
        locals.var_qovs_dn11 = assign29150_e43372_d_n11;
        locals.var_qovs_rv = 0.0;

        let (assign29160_e43385, assign29160_e43385_d_n3, assign29160_e43385_d_n4, assign29160_e43385_d_n5, assign29160_e43385_d_n6, assign29160_e43385_d_n7, assign29160_e43385_d_n8, assign29160_e43385_d_n9, assign29160_e43385_d_n10, assign29160_e43385_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard607 != 0.0)) {
        let assign29160_e43377: f64 = (-locals.var_wdiodcv);
        let assign29160_e43379: f64 = (assign29160_e43377 * p.p2);
        let assign29160_e43381: f64 = (assign29160_e43379 * locals.var_cgdof);
        let assign29160_e43383: f64 = (assign29160_e43381 * locals.var_vgd_ov_noswap);
        (assign29160_e43383, 0.0, 0.0, 0.0, (assign29160_e43381 * locals.var_vgd_ov_noswap_dn6), 0.0, 0.0, (assign29160_e43381 * locals.var_vgd_ov_noswap_dn9), 0.0, 0.0,)
    } else {
        (locals.var_qovd, locals.var_qovd_dn3, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11,)
    }
};
        locals.var_qovd = assign29160_e43385;
        locals.var_qovd_dn3 = assign29160_e43385_d_n3;
        locals.var_qovd_dn4 = assign29160_e43385_d_n4;
        locals.var_qovd_dn5 = assign29160_e43385_d_n5;
        locals.var_qovd_dn6 = assign29160_e43385_d_n6;
        locals.var_qovd_dn7 = assign29160_e43385_d_n7;
        locals.var_qovd_dn8 = assign29160_e43385_d_n8;
        locals.var_qovd_dn9 = assign29160_e43385_d_n9;
        locals.var_qovd_dn10 = assign29160_e43385_d_n10;
        locals.var_qovd_dn11 = assign29160_e43385_d_n11;
        locals.var_qovd_rv = 0.0;

        let (assign29170_e43407, assign29170_e43407_d_n3, assign29170_e43407_d_n4, assign29170_e43407_d_n5, assign29170_e43407_d_n6, assign29170_e43407_d_n7, assign29170_e43407_d_n8, assign29170_e43407_d_n9, assign29170_e43407_d_n10, assign29170_e43407_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign29170_e43392: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign29170_e43394: f64 = (assign29170_e43392 + 0.02);
        let assign29170_e43397: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign29170_e43399: f64 = (assign29170_e43397 + 0.02);
        let assign29170_e43400: f64 = (assign29170_e43394 * assign29170_e43399);
        let assign29170_e43403: f64 = (4.0 * 0.02);
        let assign29170_e43404: f64 = (assign29170_e43400 + assign29170_e43403);
        let assign29170_e43405: f64 = (assign29170_e43404).sqrt();
        (assign29170_e43405, 0.0, ((((-locals.var_vfbsdr_dn4) * assign29170_e43399) + (assign29170_e43394 * (-locals.var_vfbsdr_dn4))) / (2.0 * assign29170_e43405)), ((((-locals.var_vfbsdr_dn5) * assign29170_e43399) + (assign29170_e43394 * (-locals.var_vfbsdr_dn5))) / (2.0 * assign29170_e43405)), 0.0, (((locals.var_vgs_ov_noswap_dn7 * assign29170_e43399) + (assign29170_e43394 * locals.var_vgs_ov_noswap_dn7)) / (2.0 * assign29170_e43405)), 0.0, (((locals.var_vgs_ov_noswap_dn9 * assign29170_e43399) + (assign29170_e43394 * locals.var_vgs_ov_noswap_dn9)) / (2.0 * assign29170_e43405)), 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign29170_e43407;
        locals.var_t0_dn3 = assign29170_e43407_d_n3;
        locals.var_t0_dn4 = assign29170_e43407_d_n4;
        locals.var_t0_dn5 = assign29170_e43407_d_n5;
        locals.var_t0_dn6 = assign29170_e43407_d_n6;
        locals.var_t0_dn7 = assign29170_e43407_d_n7;
        locals.var_t0_dn8 = assign29170_e43407_d_n8;
        locals.var_t0_dn9 = assign29170_e43407_d_n9;
        locals.var_t0_dn10 = assign29170_e43407_d_n10;
        locals.var_t0_dn11 = assign29170_e43407_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign29180_e43422, assign29180_e43422_d_n3, assign29180_e43422_d_n4, assign29180_e43422_d_n5, assign29180_e43422_d_n6, assign29180_e43422_d_n7, assign29180_e43422_d_n8, assign29180_e43422_d_n9, assign29180_e43422_d_n10, assign29180_e43422_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign29180_e43415: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign29180_e43417: f64 = (assign29180_e43415 + 0.02);
        let assign29180_e43419: f64 = (assign29180_e43417 - locals.var_t0);
        let assign29180_e43420: f64 = (0.5 * assign29180_e43419);
        (assign29180_e43420, (0.5 * (-locals.var_t0_dn3)), (0.5 * ((-locals.var_vfbsdr_dn4) - locals.var_t0_dn4)), (0.5 * ((-locals.var_vfbsdr_dn5) - locals.var_t0_dn5)), (0.5 * (-locals.var_t0_dn6)), (0.5 * (locals.var_vgs_ov_noswap_dn7 - locals.var_t0_dn7)), (0.5 * (-locals.var_t0_dn8)), (0.5 * (locals.var_vgs_ov_noswap_dn9 - locals.var_t0_dn9)), (0.5 * (-locals.var_t0_dn10)), (0.5 * (-locals.var_t0_dn11)),)
    } else {
        (locals.var_vgsov, locals.var_vgsov_dn3, locals.var_vgsov_dn4, locals.var_vgsov_dn5, locals.var_vgsov_dn6, locals.var_vgsov_dn7, locals.var_vgsov_dn8, locals.var_vgsov_dn9, locals.var_vgsov_dn10, locals.var_vgsov_dn11,)
    }
};
        locals.var_vgsov = assign29180_e43422;
        locals.var_vgsov_dn3 = assign29180_e43422_d_n3;
        locals.var_vgsov_dn4 = assign29180_e43422_d_n4;
        locals.var_vgsov_dn5 = assign29180_e43422_d_n5;
        locals.var_vgsov_dn6 = assign29180_e43422_d_n6;
        locals.var_vgsov_dn7 = assign29180_e43422_d_n7;
        locals.var_vgsov_dn8 = assign29180_e43422_d_n8;
        locals.var_vgsov_dn9 = assign29180_e43422_d_n9;
        locals.var_vgsov_dn10 = assign29180_e43422_d_n10;
        locals.var_vgsov_dn11 = assign29180_e43422_d_n11;
        locals.var_vgsov_rv = 0.0;

        let (assign29190_e43442, assign29190_e43442_d_n3, assign29190_e43442_d_n4, assign29190_e43442_d_n5, assign29190_e43442_d_n6, assign29190_e43442_d_n7, assign29190_e43442_d_n8, assign29190_e43442_d_n9, assign29190_e43442_d_n10, assign29190_e43442_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign29190_e43430: f64 = (-locals.var_vgsov);
        let assign29190_e43432: f64 = (assign29190_e43430 / p.p893);
        let assign29190_e43434: f64 = (assign29190_e43432).powf(p.p894);
        let assign29190_e43435: f64 = (1.0 + assign29190_e43434);
        let assign29190_e43438: f64 = (1.0 / p.p894);
        let assign29190_e43439: f64 = (assign29190_e43435).powf(assign29190_e43438);
        let assign29190_e43440: f64 = (locals.var_vgsov / assign29190_e43439);
        (assign29190_e43440, (((locals.var_vgsov_dn3 * assign29190_e43439) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign29190_e43438) as f64).is_finite() && ((assign29190_e43438) as f64).fract() == 0.0 { if assign29190_e43438 == 0.0 { 0.0 } else { (assign29190_e43438 * ((assign29190_e43435).powf(assign29190_e43438 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn3) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn3) / p.p893) / assign29190_e43432))) })) } } else { (assign29190_e43439 * (assign29190_e43438 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn3) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn3) / p.p893) / assign29190_e43432))) } / assign29190_e43435))) })) / (assign29190_e43439 * assign29190_e43439)), (((locals.var_vgsov_dn4 * assign29190_e43439) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign29190_e43438) as f64).is_finite() && ((assign29190_e43438) as f64).fract() == 0.0 { if assign29190_e43438 == 0.0 { 0.0 } else { (assign29190_e43438 * ((assign29190_e43435).powf(assign29190_e43438 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn4) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn4) / p.p893) / assign29190_e43432))) })) } } else { (assign29190_e43439 * (assign29190_e43438 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn4) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn4) / p.p893) / assign29190_e43432))) } / assign29190_e43435))) })) / (assign29190_e43439 * assign29190_e43439)), (((locals.var_vgsov_dn5 * assign29190_e43439) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign29190_e43438) as f64).is_finite() && ((assign29190_e43438) as f64).fract() == 0.0 { if assign29190_e43438 == 0.0 { 0.0 } else { (assign29190_e43438 * ((assign29190_e43435).powf(assign29190_e43438 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn5) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn5) / p.p893) / assign29190_e43432))) })) } } else { (assign29190_e43439 * (assign29190_e43438 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn5) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn5) / p.p893) / assign29190_e43432))) } / assign29190_e43435))) })) / (assign29190_e43439 * assign29190_e43439)), (((locals.var_vgsov_dn6 * assign29190_e43439) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign29190_e43438) as f64).is_finite() && ((assign29190_e43438) as f64).fract() == 0.0 { if assign29190_e43438 == 0.0 { 0.0 } else { (assign29190_e43438 * ((assign29190_e43435).powf(assign29190_e43438 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn6) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn6) / p.p893) / assign29190_e43432))) })) } } else { (assign29190_e43439 * (assign29190_e43438 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn6) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn6) / p.p893) / assign29190_e43432))) } / assign29190_e43435))) })) / (assign29190_e43439 * assign29190_e43439)), (((locals.var_vgsov_dn7 * assign29190_e43439) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign29190_e43438) as f64).is_finite() && ((assign29190_e43438) as f64).fract() == 0.0 { if assign29190_e43438 == 0.0 { 0.0 } else { (assign29190_e43438 * ((assign29190_e43435).powf(assign29190_e43438 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn7) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn7) / p.p893) / assign29190_e43432))) })) } } else { (assign29190_e43439 * (assign29190_e43438 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn7) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn7) / p.p893) / assign29190_e43432))) } / assign29190_e43435))) })) / (assign29190_e43439 * assign29190_e43439)), (((locals.var_vgsov_dn8 * assign29190_e43439) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign29190_e43438) as f64).is_finite() && ((assign29190_e43438) as f64).fract() == 0.0 { if assign29190_e43438 == 0.0 { 0.0 } else { (assign29190_e43438 * ((assign29190_e43435).powf(assign29190_e43438 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn8) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn8) / p.p893) / assign29190_e43432))) })) } } else { (assign29190_e43439 * (assign29190_e43438 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn8) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn8) / p.p893) / assign29190_e43432))) } / assign29190_e43435))) })) / (assign29190_e43439 * assign29190_e43439)), (((locals.var_vgsov_dn9 * assign29190_e43439) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign29190_e43438) as f64).is_finite() && ((assign29190_e43438) as f64).fract() == 0.0 { if assign29190_e43438 == 0.0 { 0.0 } else { (assign29190_e43438 * ((assign29190_e43435).powf(assign29190_e43438 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn9) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn9) / p.p893) / assign29190_e43432))) })) } } else { (assign29190_e43439 * (assign29190_e43438 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn9) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn9) / p.p893) / assign29190_e43432))) } / assign29190_e43435))) })) / (assign29190_e43439 * assign29190_e43439)), (((locals.var_vgsov_dn10 * assign29190_e43439) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign29190_e43438) as f64).is_finite() && ((assign29190_e43438) as f64).fract() == 0.0 { if assign29190_e43438 == 0.0 { 0.0 } else { (assign29190_e43438 * ((assign29190_e43435).powf(assign29190_e43438 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn10) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn10) / p.p893) / assign29190_e43432))) })) } } else { (assign29190_e43439 * (assign29190_e43438 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn10) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn10) / p.p893) / assign29190_e43432))) } / assign29190_e43435))) })) / (assign29190_e43439 * assign29190_e43439)), (((locals.var_vgsov_dn11 * assign29190_e43439) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign29190_e43438) as f64).is_finite() && ((assign29190_e43438) as f64).fract() == 0.0 { if assign29190_e43438 == 0.0 { 0.0 } else { (assign29190_e43438 * ((assign29190_e43435).powf(assign29190_e43438 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn11) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn11) / p.p893) / assign29190_e43432))) })) } } else { (assign29190_e43439 * (assign29190_e43438 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign29190_e43432).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn11) / p.p893))) } } else { (assign29190_e43434 * (p.p894 * (((-locals.var_vgsov_dn11) / p.p893) / assign29190_e43432))) } / assign29190_e43435))) })) / (assign29190_e43439 * assign29190_e43439)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign29190_e43442;
        locals.var_t6_dn3 = assign29190_e43442_d_n3;
        locals.var_t6_dn4 = assign29190_e43442_d_n4;
        locals.var_t6_dn5 = assign29190_e43442_d_n5;
        locals.var_t6_dn6 = assign29190_e43442_d_n6;
        locals.var_t6_dn7 = assign29190_e43442_d_n7;
        locals.var_t6_dn8 = assign29190_e43442_d_n8;
        locals.var_t6_dn9 = assign29190_e43442_d_n9;
        locals.var_t6_dn10 = assign29190_e43442_d_n10;
        locals.var_t6_dn11 = assign29190_e43442_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign29200_e43456, assign29200_e43456_d_n3, assign29200_e43456_d_n4, assign29200_e43456_d_n5, assign29200_e43456_d_n6, assign29200_e43456_d_n7, assign29200_e43456_d_n8, assign29200_e43456_d_n9, assign29200_e43456_d_n10, assign29200_e43456_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign29200_e43450: f64 = (4.0 * locals.var_t6);
        let assign29200_e43452: f64 = (assign29200_e43450 / locals.var_ckappas_i);
        let assign29200_e43453: f64 = (1.0 - assign29200_e43452);
        let assign29200_e43454: f64 = (assign29200_e43453).sqrt();
        (assign29200_e43454, ((-((4.0 * locals.var_t6_dn3) / locals.var_ckappas_i)) / (2.0 * assign29200_e43454)), ((-((4.0 * locals.var_t6_dn4) / locals.var_ckappas_i)) / (2.0 * assign29200_e43454)), ((-((4.0 * locals.var_t6_dn5) / locals.var_ckappas_i)) / (2.0 * assign29200_e43454)), ((-((4.0 * locals.var_t6_dn6) / locals.var_ckappas_i)) / (2.0 * assign29200_e43454)), ((-((4.0 * locals.var_t6_dn7) / locals.var_ckappas_i)) / (2.0 * assign29200_e43454)), ((-((4.0 * locals.var_t6_dn8) / locals.var_ckappas_i)) / (2.0 * assign29200_e43454)), ((-((4.0 * locals.var_t6_dn9) / locals.var_ckappas_i)) / (2.0 * assign29200_e43454)), ((-((4.0 * locals.var_t6_dn10) / locals.var_ckappas_i)) / (2.0 * assign29200_e43454)), ((-((4.0 * locals.var_t6_dn11) / locals.var_ckappas_i)) / (2.0 * assign29200_e43454)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign29200_e43456;
        locals.var_t1_dn3 = assign29200_e43456_d_n3;
        locals.var_t1_dn4 = assign29200_e43456_d_n4;
        locals.var_t1_dn5 = assign29200_e43456_d_n5;
        locals.var_t1_dn6 = assign29200_e43456_d_n6;
        locals.var_t1_dn7 = assign29200_e43456_d_n7;
        locals.var_t1_dn8 = assign29200_e43456_d_n8;
        locals.var_t1_dn9 = assign29200_e43456_d_n9;
        locals.var_t1_dn10 = assign29200_e43456_d_n10;
        locals.var_t1_dn11 = assign29200_e43456_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign29210_e43487, assign29210_e43487_d_n3, assign29210_e43487_d_n4, assign29210_e43487_d_n5, assign29210_e43487_d_n6, assign29210_e43487_d_n7, assign29210_e43487_d_n8, assign29210_e43487_d_n9, assign29210_e43487_d_n10, assign29210_e43487_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign29210_e43462: f64 = (-locals.var_wdioscv);
        let assign29210_e43464: f64 = (assign29210_e43462 * p.p2);
        let assign29210_e43467: f64 = (locals.var_cgsof * locals.var_vgs_ov_noswap);
        let assign29210_e43471: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign29210_e43473: f64 = (assign29210_e43471 - locals.var_vgsov);
        let assign29210_e43476: f64 = (0.5 * locals.var_ckappas_i);
        let assign29210_e43478: f64 = (-1.0);
        let assign29210_e43480: f64 = (assign29210_e43478 + locals.var_t1);
        let assign29210_e43481: f64 = (assign29210_e43476 * assign29210_e43480);
        let assign29210_e43482: f64 = (assign29210_e43473 - assign29210_e43481);
        let assign29210_e43483: f64 = (locals.var_cgsl_i * assign29210_e43482);
        let assign29210_e43484: f64 = (assign29210_e43467 + assign29210_e43483);
        let assign29210_e43485: f64 = (assign29210_e43464 * assign29210_e43484);
        (assign29210_e43485, (assign29210_e43464 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn3) - (assign29210_e43476 * locals.var_t1_dn3)))), (assign29210_e43464 * (locals.var_cgsl_i * (((-locals.var_vfbsdr_dn4) - locals.var_vgsov_dn4) - (assign29210_e43476 * locals.var_t1_dn4)))), (assign29210_e43464 * (locals.var_cgsl_i * (((-locals.var_vfbsdr_dn5) - locals.var_vgsov_dn5) - (assign29210_e43476 * locals.var_t1_dn5)))), (assign29210_e43464 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn6) - (assign29210_e43476 * locals.var_t1_dn6)))), (assign29210_e43464 * ((locals.var_cgsof * locals.var_vgs_ov_noswap_dn7) + (locals.var_cgsl_i * ((locals.var_vgs_ov_noswap_dn7 - locals.var_vgsov_dn7) - (assign29210_e43476 * locals.var_t1_dn7))))), (assign29210_e43464 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn8) - (assign29210_e43476 * locals.var_t1_dn8)))), (assign29210_e43464 * ((locals.var_cgsof * locals.var_vgs_ov_noswap_dn9) + (locals.var_cgsl_i * ((locals.var_vgs_ov_noswap_dn9 - locals.var_vgsov_dn9) - (assign29210_e43476 * locals.var_t1_dn9))))), (assign29210_e43464 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn10) - (assign29210_e43476 * locals.var_t1_dn10)))), (assign29210_e43464 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn11) - (assign29210_e43476 * locals.var_t1_dn11)))),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn3, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11,)
    }
};
        locals.var_qovs = assign29210_e43487;
        locals.var_qovs_dn3 = assign29210_e43487_d_n3;
        locals.var_qovs_dn4 = assign29210_e43487_d_n4;
        locals.var_qovs_dn5 = assign29210_e43487_d_n5;
        locals.var_qovs_dn6 = assign29210_e43487_d_n6;
        locals.var_qovs_dn7 = assign29210_e43487_d_n7;
        locals.var_qovs_dn8 = assign29210_e43487_d_n8;
        locals.var_qovs_dn9 = assign29210_e43487_d_n9;
        locals.var_qovs_dn10 = assign29210_e43487_d_n10;
        locals.var_qovs_dn11 = assign29210_e43487_d_n11;
        locals.var_qovs_rv = 0.0;

        let (assign29220_e43509, assign29220_e43509_d_n3, assign29220_e43509_d_n4, assign29220_e43509_d_n5, assign29220_e43509_d_n6, assign29220_e43509_d_n7, assign29220_e43509_d_n8, assign29220_e43509_d_n9, assign29220_e43509_d_n10, assign29220_e43509_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign29220_e43494: f64 = (locals.var_vgd_ov_noswap - locals.var_vfbsdr);
        let assign29220_e43496: f64 = (assign29220_e43494 + 0.02);
        let assign29220_e43499: f64 = (locals.var_vgd_ov_noswap - locals.var_vfbsdr);
        let assign29220_e43501: f64 = (assign29220_e43499 + 0.02);
        let assign29220_e43502: f64 = (assign29220_e43496 * assign29220_e43501);
        let assign29220_e43505: f64 = (4.0 * 0.02);
        let assign29220_e43506: f64 = (assign29220_e43502 + assign29220_e43505);
        let assign29220_e43507: f64 = (assign29220_e43506).sqrt();
        (assign29220_e43507, 0.0, ((((-locals.var_vfbsdr_dn4) * assign29220_e43501) + (assign29220_e43496 * (-locals.var_vfbsdr_dn4))) / (2.0 * assign29220_e43507)), ((((-locals.var_vfbsdr_dn5) * assign29220_e43501) + (assign29220_e43496 * (-locals.var_vfbsdr_dn5))) / (2.0 * assign29220_e43507)), (((locals.var_vgd_ov_noswap_dn6 * assign29220_e43501) + (assign29220_e43496 * locals.var_vgd_ov_noswap_dn6)) / (2.0 * assign29220_e43507)), 0.0, 0.0, (((locals.var_vgd_ov_noswap_dn9 * assign29220_e43501) + (assign29220_e43496 * locals.var_vgd_ov_noswap_dn9)) / (2.0 * assign29220_e43507)), 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign29220_e43509;
        locals.var_t0_dn3 = assign29220_e43509_d_n3;
        locals.var_t0_dn4 = assign29220_e43509_d_n4;
        locals.var_t0_dn5 = assign29220_e43509_d_n5;
        locals.var_t0_dn6 = assign29220_e43509_d_n6;
        locals.var_t0_dn7 = assign29220_e43509_d_n7;
        locals.var_t0_dn8 = assign29220_e43509_d_n8;
        locals.var_t0_dn9 = assign29220_e43509_d_n9;
        locals.var_t0_dn10 = assign29220_e43509_d_n10;
        locals.var_t0_dn11 = assign29220_e43509_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign29230_e43524, assign29230_e43524_d_n3, assign29230_e43524_d_n4, assign29230_e43524_d_n5, assign29230_e43524_d_n6, assign29230_e43524_d_n7, assign29230_e43524_d_n8, assign29230_e43524_d_n9, assign29230_e43524_d_n10, assign29230_e43524_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign29230_e43517: f64 = (locals.var_vgd_ov_noswap - locals.var_vfbsdr);
        let assign29230_e43519: f64 = (assign29230_e43517 + 0.02);
        let assign29230_e43521: f64 = (assign29230_e43519 - locals.var_t0);
        let assign29230_e43522: f64 = (0.5 * assign29230_e43521);
        (assign29230_e43522, (0.5 * (-locals.var_t0_dn3)), (0.5 * ((-locals.var_vfbsdr_dn4) - locals.var_t0_dn4)), (0.5 * ((-locals.var_vfbsdr_dn5) - locals.var_t0_dn5)), (0.5 * (locals.var_vgd_ov_noswap_dn6 - locals.var_t0_dn6)), (0.5 * (-locals.var_t0_dn7)), (0.5 * (-locals.var_t0_dn8)), (0.5 * (locals.var_vgd_ov_noswap_dn9 - locals.var_t0_dn9)), (0.5 * (-locals.var_t0_dn10)), (0.5 * (-locals.var_t0_dn11)),)
    } else {
        (locals.var_vgdov, locals.var_vgdov_dn3, locals.var_vgdov_dn4, locals.var_vgdov_dn5, locals.var_vgdov_dn6, locals.var_vgdov_dn7, locals.var_vgdov_dn8, locals.var_vgdov_dn9, locals.var_vgdov_dn10, locals.var_vgdov_dn11,)
    }
};
        locals.var_vgdov = assign29230_e43524;
        locals.var_vgdov_dn3 = assign29230_e43524_d_n3;
        locals.var_vgdov_dn4 = assign29230_e43524_d_n4;
        locals.var_vgdov_dn5 = assign29230_e43524_d_n5;
        locals.var_vgdov_dn6 = assign29230_e43524_d_n6;
        locals.var_vgdov_dn7 = assign29230_e43524_d_n7;
        locals.var_vgdov_dn8 = assign29230_e43524_d_n8;
        locals.var_vgdov_dn9 = assign29230_e43524_d_n9;
        locals.var_vgdov_dn10 = assign29230_e43524_d_n10;
        locals.var_vgdov_dn11 = assign29230_e43524_d_n11;
        locals.var_vgdov_rv = 0.0;

        let (assign29240_e43544, assign29240_e43544_d_n3, assign29240_e43544_d_n4, assign29240_e43544_d_n5, assign29240_e43544_d_n6, assign29240_e43544_d_n7, assign29240_e43544_d_n8, assign29240_e43544_d_n9, assign29240_e43544_d_n10, assign29240_e43544_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign29240_e43532: f64 = (-locals.var_vgdov);
        let assign29240_e43534: f64 = (assign29240_e43532 / p.p891);
        let assign29240_e43536: f64 = (assign29240_e43534).powf(p.p892);
        let assign29240_e43537: f64 = (1.0 + assign29240_e43536);
        let assign29240_e43540: f64 = (1.0 / p.p892);
        let assign29240_e43541: f64 = (assign29240_e43537).powf(assign29240_e43540);
        let assign29240_e43542: f64 = (locals.var_vgdov / assign29240_e43541);
        (assign29240_e43542, (((locals.var_vgdov_dn3 * assign29240_e43541) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign29240_e43540) as f64).is_finite() && ((assign29240_e43540) as f64).fract() == 0.0 { if assign29240_e43540 == 0.0 { 0.0 } else { (assign29240_e43540 * ((assign29240_e43537).powf(assign29240_e43540 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn3) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn3) / p.p891) / assign29240_e43534))) })) } } else { (assign29240_e43541 * (assign29240_e43540 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn3) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn3) / p.p891) / assign29240_e43534))) } / assign29240_e43537))) })) / (assign29240_e43541 * assign29240_e43541)), (((locals.var_vgdov_dn4 * assign29240_e43541) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign29240_e43540) as f64).is_finite() && ((assign29240_e43540) as f64).fract() == 0.0 { if assign29240_e43540 == 0.0 { 0.0 } else { (assign29240_e43540 * ((assign29240_e43537).powf(assign29240_e43540 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn4) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn4) / p.p891) / assign29240_e43534))) })) } } else { (assign29240_e43541 * (assign29240_e43540 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn4) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn4) / p.p891) / assign29240_e43534))) } / assign29240_e43537))) })) / (assign29240_e43541 * assign29240_e43541)), (((locals.var_vgdov_dn5 * assign29240_e43541) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign29240_e43540) as f64).is_finite() && ((assign29240_e43540) as f64).fract() == 0.0 { if assign29240_e43540 == 0.0 { 0.0 } else { (assign29240_e43540 * ((assign29240_e43537).powf(assign29240_e43540 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn5) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn5) / p.p891) / assign29240_e43534))) })) } } else { (assign29240_e43541 * (assign29240_e43540 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn5) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn5) / p.p891) / assign29240_e43534))) } / assign29240_e43537))) })) / (assign29240_e43541 * assign29240_e43541)), (((locals.var_vgdov_dn6 * assign29240_e43541) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign29240_e43540) as f64).is_finite() && ((assign29240_e43540) as f64).fract() == 0.0 { if assign29240_e43540 == 0.0 { 0.0 } else { (assign29240_e43540 * ((assign29240_e43537).powf(assign29240_e43540 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn6) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn6) / p.p891) / assign29240_e43534))) })) } } else { (assign29240_e43541 * (assign29240_e43540 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn6) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn6) / p.p891) / assign29240_e43534))) } / assign29240_e43537))) })) / (assign29240_e43541 * assign29240_e43541)), (((locals.var_vgdov_dn7 * assign29240_e43541) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign29240_e43540) as f64).is_finite() && ((assign29240_e43540) as f64).fract() == 0.0 { if assign29240_e43540 == 0.0 { 0.0 } else { (assign29240_e43540 * ((assign29240_e43537).powf(assign29240_e43540 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn7) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn7) / p.p891) / assign29240_e43534))) })) } } else { (assign29240_e43541 * (assign29240_e43540 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn7) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn7) / p.p891) / assign29240_e43534))) } / assign29240_e43537))) })) / (assign29240_e43541 * assign29240_e43541)), (((locals.var_vgdov_dn8 * assign29240_e43541) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign29240_e43540) as f64).is_finite() && ((assign29240_e43540) as f64).fract() == 0.0 { if assign29240_e43540 == 0.0 { 0.0 } else { (assign29240_e43540 * ((assign29240_e43537).powf(assign29240_e43540 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn8) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn8) / p.p891) / assign29240_e43534))) })) } } else { (assign29240_e43541 * (assign29240_e43540 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn8) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn8) / p.p891) / assign29240_e43534))) } / assign29240_e43537))) })) / (assign29240_e43541 * assign29240_e43541)), (((locals.var_vgdov_dn9 * assign29240_e43541) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign29240_e43540) as f64).is_finite() && ((assign29240_e43540) as f64).fract() == 0.0 { if assign29240_e43540 == 0.0 { 0.0 } else { (assign29240_e43540 * ((assign29240_e43537).powf(assign29240_e43540 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn9) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn9) / p.p891) / assign29240_e43534))) })) } } else { (assign29240_e43541 * (assign29240_e43540 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn9) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn9) / p.p891) / assign29240_e43534))) } / assign29240_e43537))) })) / (assign29240_e43541 * assign29240_e43541)), (((locals.var_vgdov_dn10 * assign29240_e43541) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign29240_e43540) as f64).is_finite() && ((assign29240_e43540) as f64).fract() == 0.0 { if assign29240_e43540 == 0.0 { 0.0 } else { (assign29240_e43540 * ((assign29240_e43537).powf(assign29240_e43540 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn10) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn10) / p.p891) / assign29240_e43534))) })) } } else { (assign29240_e43541 * (assign29240_e43540 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn10) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn10) / p.p891) / assign29240_e43534))) } / assign29240_e43537))) })) / (assign29240_e43541 * assign29240_e43541)), (((locals.var_vgdov_dn11 * assign29240_e43541) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign29240_e43540) as f64).is_finite() && ((assign29240_e43540) as f64).fract() == 0.0 { if assign29240_e43540 == 0.0 { 0.0 } else { (assign29240_e43540 * ((assign29240_e43537).powf(assign29240_e43540 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn11) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn11) / p.p891) / assign29240_e43534))) })) } } else { (assign29240_e43541 * (assign29240_e43540 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign29240_e43534).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn11) / p.p891))) } } else { (assign29240_e43536 * (p.p892 * (((-locals.var_vgdov_dn11) / p.p891) / assign29240_e43534))) } / assign29240_e43537))) })) / (assign29240_e43541 * assign29240_e43541)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign29240_e43544;
        locals.var_t6_dn3 = assign29240_e43544_d_n3;
        locals.var_t6_dn4 = assign29240_e43544_d_n4;
        locals.var_t6_dn5 = assign29240_e43544_d_n5;
        locals.var_t6_dn6 = assign29240_e43544_d_n6;
        locals.var_t6_dn7 = assign29240_e43544_d_n7;
        locals.var_t6_dn8 = assign29240_e43544_d_n8;
        locals.var_t6_dn9 = assign29240_e43544_d_n9;
        locals.var_t6_dn10 = assign29240_e43544_d_n10;
        locals.var_t6_dn11 = assign29240_e43544_d_n11;
        locals.var_t6_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_84(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (assign29250_e43558, assign29250_e43558_d_n3, assign29250_e43558_d_n4, assign29250_e43558_d_n5, assign29250_e43558_d_n6, assign29250_e43558_d_n7, assign29250_e43558_d_n8, assign29250_e43558_d_n9, assign29250_e43558_d_n10, assign29250_e43558_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign29250_e43552: f64 = (4.0 * locals.var_t6);
        let assign29250_e43554: f64 = (assign29250_e43552 / locals.var_ckappad_i);
        let assign29250_e43555: f64 = (1.0 - assign29250_e43554);
        let assign29250_e43556: f64 = (assign29250_e43555).sqrt();
        (assign29250_e43556, ((-((4.0 * locals.var_t6_dn3) / locals.var_ckappad_i)) / (2.0 * assign29250_e43556)), ((-((4.0 * locals.var_t6_dn4) / locals.var_ckappad_i)) / (2.0 * assign29250_e43556)), ((-((4.0 * locals.var_t6_dn5) / locals.var_ckappad_i)) / (2.0 * assign29250_e43556)), ((-((4.0 * locals.var_t6_dn6) / locals.var_ckappad_i)) / (2.0 * assign29250_e43556)), ((-((4.0 * locals.var_t6_dn7) / locals.var_ckappad_i)) / (2.0 * assign29250_e43556)), ((-((4.0 * locals.var_t6_dn8) / locals.var_ckappad_i)) / (2.0 * assign29250_e43556)), ((-((4.0 * locals.var_t6_dn9) / locals.var_ckappad_i)) / (2.0 * assign29250_e43556)), ((-((4.0 * locals.var_t6_dn10) / locals.var_ckappad_i)) / (2.0 * assign29250_e43556)), ((-((4.0 * locals.var_t6_dn11) / locals.var_ckappad_i)) / (2.0 * assign29250_e43556)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign29250_e43558;
        locals.var_t2_dn3 = assign29250_e43558_d_n3;
        locals.var_t2_dn4 = assign29250_e43558_d_n4;
        locals.var_t2_dn5 = assign29250_e43558_d_n5;
        locals.var_t2_dn6 = assign29250_e43558_d_n6;
        locals.var_t2_dn7 = assign29250_e43558_d_n7;
        locals.var_t2_dn8 = assign29250_e43558_d_n8;
        locals.var_t2_dn9 = assign29250_e43558_d_n9;
        locals.var_t2_dn10 = assign29250_e43558_d_n10;
        locals.var_t2_dn11 = assign29250_e43558_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign29260_e43589, assign29260_e43589_d_n3, assign29260_e43589_d_n4, assign29260_e43589_d_n5, assign29260_e43589_d_n6, assign29260_e43589_d_n7, assign29260_e43589_d_n8, assign29260_e43589_d_n9, assign29260_e43589_d_n10, assign29260_e43589_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard607 == 0.0)) {
        let assign29260_e43564: f64 = (-locals.var_wdiodcv);
        let assign29260_e43566: f64 = (assign29260_e43564 * p.p2);
        let assign29260_e43569: f64 = (locals.var_cgdof * locals.var_vgd_ov_noswap);
        let assign29260_e43573: f64 = (locals.var_vgd_ov_noswap - locals.var_vfbsdr);
        let assign29260_e43575: f64 = (assign29260_e43573 - locals.var_vgdov);
        let assign29260_e43578: f64 = (0.5 * locals.var_ckappad_i);
        let assign29260_e43580: f64 = (-1.0);
        let assign29260_e43582: f64 = (assign29260_e43580 + locals.var_t2);
        let assign29260_e43583: f64 = (assign29260_e43578 * assign29260_e43582);
        let assign29260_e43584: f64 = (assign29260_e43575 - assign29260_e43583);
        let assign29260_e43585: f64 = (locals.var_cgdl_i * assign29260_e43584);
        let assign29260_e43586: f64 = (assign29260_e43569 + assign29260_e43585);
        let assign29260_e43587: f64 = (assign29260_e43566 * assign29260_e43586);
        (assign29260_e43587, (assign29260_e43566 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn3) - (assign29260_e43578 * locals.var_t2_dn3)))), (assign29260_e43566 * (locals.var_cgdl_i * (((-locals.var_vfbsdr_dn4) - locals.var_vgdov_dn4) - (assign29260_e43578 * locals.var_t2_dn4)))), (assign29260_e43566 * (locals.var_cgdl_i * (((-locals.var_vfbsdr_dn5) - locals.var_vgdov_dn5) - (assign29260_e43578 * locals.var_t2_dn5)))), (assign29260_e43566 * ((locals.var_cgdof * locals.var_vgd_ov_noswap_dn6) + (locals.var_cgdl_i * ((locals.var_vgd_ov_noswap_dn6 - locals.var_vgdov_dn6) - (assign29260_e43578 * locals.var_t2_dn6))))), (assign29260_e43566 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn7) - (assign29260_e43578 * locals.var_t2_dn7)))), (assign29260_e43566 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn8) - (assign29260_e43578 * locals.var_t2_dn8)))), (assign29260_e43566 * ((locals.var_cgdof * locals.var_vgd_ov_noswap_dn9) + (locals.var_cgdl_i * ((locals.var_vgd_ov_noswap_dn9 - locals.var_vgdov_dn9) - (assign29260_e43578 * locals.var_t2_dn9))))), (assign29260_e43566 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn10) - (assign29260_e43578 * locals.var_t2_dn10)))), (assign29260_e43566 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn11) - (assign29260_e43578 * locals.var_t2_dn11)))),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn3, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11,)
    }
};
        locals.var_qovd = assign29260_e43589;
        locals.var_qovd_dn3 = assign29260_e43589_d_n3;
        locals.var_qovd_dn4 = assign29260_e43589_d_n4;
        locals.var_qovd_dn5 = assign29260_e43589_d_n5;
        locals.var_qovd_dn6 = assign29260_e43589_d_n6;
        locals.var_qovd_dn7 = assign29260_e43589_d_n7;
        locals.var_qovd_dn8 = assign29260_e43589_d_n8;
        locals.var_qovd_dn9 = assign29260_e43589_d_n9;
        locals.var_qovd_dn10 = assign29260_e43589_d_n10;
        locals.var_qovd_dn11 = assign29260_e43589_d_n11;
        locals.var_qovd_rv = 0.0;

        let (assign29270_e43602, assign29270_e43602_d_n9, assign29270_e43602_d_n10,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29270_e43592: f64 = (-locals.var_devsign);
        let assign29270_e43594: f64 = (assign29270_e43592 * p.p2);
        let assign29270_e43596: f64 = (assign29270_e43594 * locals.var_lact);
        let assign29270_e43598: f64 = (assign29270_e43596 * p.p874);
        let assign29270_e43600: f64 = (assign29270_e43598 * (nv9 - nv10));
        (assign29270_e43600, assign29270_e43598, (-assign29270_e43598),)
    } else {
        (locals.var_qovb, locals.var_qovb_dn9, locals.var_qovb_dn10,)
    }
};
        locals.var_qovb = assign29270_e43602;
        locals.var_qovb_dn9 = assign29270_e43602_d_n9;
        locals.var_qovb_dn10 = assign29270_e43602_d_n10;
        locals.var_qovb_rv = 0.0;

        let (assign29280_e43611, assign29280_e43611_d_n3, assign29280_e43611_d_n4, assign29280_e43611_d_n5, assign29280_e43611_d_n6, assign29280_e43611_d_n7, assign29280_e43611_d_n8, assign29280_e43611_d_n9, assign29280_e43611_d_n10, assign29280_e43611_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29280_e43606: f64 = (locals.var_qovs + locals.var_qovd);
        let assign29280_e43608: f64 = (assign29280_e43606 + locals.var_qovb);
        let assign29280_e43609: f64 = (-assign29280_e43608);
        (assign29280_e43609, (-(locals.var_qovs_dn3 + locals.var_qovd_dn3)), (-(locals.var_qovs_dn4 + locals.var_qovd_dn4)), (-(locals.var_qovs_dn5 + locals.var_qovd_dn5)), (-(locals.var_qovs_dn6 + locals.var_qovd_dn6)), (-(locals.var_qovs_dn7 + locals.var_qovd_dn7)), (-(locals.var_qovs_dn8 + locals.var_qovd_dn8)), (-((locals.var_qovs_dn9 + locals.var_qovd_dn9) + locals.var_qovb_dn9)), (-((locals.var_qovs_dn10 + locals.var_qovd_dn10) + locals.var_qovb_dn10)), (-(locals.var_qovs_dn11 + locals.var_qovd_dn11)),)
    } else {
        (locals.var_qovg, locals.var_qovg_dn3, locals.var_qovg_dn4, locals.var_qovg_dn5, locals.var_qovg_dn6, locals.var_qovg_dn7, locals.var_qovg_dn8, locals.var_qovg_dn9, locals.var_qovg_dn10, locals.var_qovg_dn11,)
    }
};
        locals.var_qovg = assign29280_e43611;
        locals.var_qovg_dn3 = assign29280_e43611_d_n3;
        locals.var_qovg_dn4 = assign29280_e43611_d_n4;
        locals.var_qovg_dn5 = assign29280_e43611_d_n5;
        locals.var_qovg_dn6 = assign29280_e43611_d_n6;
        locals.var_qovg_dn7 = assign29280_e43611_d_n7;
        locals.var_qovg_dn8 = assign29280_e43611_d_n8;
        locals.var_qovg_dn9 = assign29280_e43611_d_n9;
        locals.var_qovg_dn10 = assign29280_e43611_d_n10;
        locals.var_qovg_dn11 = assign29280_e43611_d_n11;
        locals.var_qovg_rv = 0.0;

        let (assign29290_e43621,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29290_e43616: f64 = (2.0 * locals.var_dlcv);
        let assign29290_e43617: f64 = (locals.var_lnew - assign29290_e43616);
        let assign29290_e43619: f64 = (assign29290_e43617 - p.p1394);
        (assign29290_e43619,)
    } else {
        (locals.var_leffcvb,)
    }
};
        locals.var_leffcvb = assign29290_e43621;
        locals.var_leffcvb_rv = 0.0;

        let (assign29300_e43629,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29300_e43626: f64 = (2.0 * p.p1393);
        let assign29300_e43627: f64 = (locals.var_leffcvb + assign29300_e43626);
        (assign29300_e43627,)
    } else {
        (locals.var_leffcvbg,)
    }
};
        locals.var_leffcvbg = assign29300_e43629;
        locals.var_leffcvbg_rv = 0.0;

        let assign29310_e43632: f64 = if locals.var_nsub_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard608 = assign29310_e43632;
        locals.var_guard608_rv = 0.0;

        let (assign29320_e43643, assign29320_e43643_d_n3, assign29320_e43643_d_n4, assign29320_e43643_d_n5, assign29320_e43643_d_n6, assign29320_e43643_d_n7, assign29320_e43643_d_n8, assign29320_e43643_d_n9, assign29320_e43643_d_n10, assign29320_e43643_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard608 != 0.0)) {
        let assign29320_e43638: f64 = (locals.var_ndep_i / locals.var_nsub_i);
        let assign29320_e43640: f64 = (assign29320_e43638).max(1e-38);
        let assign29320_e43641: f64 = (assign29320_e43640).ln();
        (assign29320_e43641, (if assign29320_e43638 >= 1e-38 { (locals.var_ndep_i_dn3 / locals.var_nsub_i) } else { 0.0 } / assign29320_e43640), (if assign29320_e43638 >= 1e-38 { (locals.var_ndep_i_dn4 / locals.var_nsub_i) } else { 0.0 } / assign29320_e43640), (if assign29320_e43638 >= 1e-38 { (locals.var_ndep_i_dn5 / locals.var_nsub_i) } else { 0.0 } / assign29320_e43640), (if assign29320_e43638 >= 1e-38 { (locals.var_ndep_i_dn6 / locals.var_nsub_i) } else { 0.0 } / assign29320_e43640), (if assign29320_e43638 >= 1e-38 { (locals.var_ndep_i_dn7 / locals.var_nsub_i) } else { 0.0 } / assign29320_e43640), (if assign29320_e43638 >= 1e-38 { (locals.var_ndep_i_dn8 / locals.var_nsub_i) } else { 0.0 } / assign29320_e43640), (if assign29320_e43638 >= 1e-38 { (locals.var_ndep_i_dn9 / locals.var_nsub_i) } else { 0.0 } / assign29320_e43640), (if assign29320_e43638 >= 1e-38 { (locals.var_ndep_i_dn10 / locals.var_nsub_i) } else { 0.0 } / assign29320_e43640), (if assign29320_e43638 >= 1e-38 { (locals.var_ndep_i_dn11 / locals.var_nsub_i) } else { 0.0 } / assign29320_e43640),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign29320_e43643;
        locals.var_t0_dn3 = assign29320_e43643_d_n3;
        locals.var_t0_dn4 = assign29320_e43643_d_n4;
        locals.var_t0_dn5 = assign29320_e43643_d_n5;
        locals.var_t0_dn6 = assign29320_e43643_d_n6;
        locals.var_t0_dn7 = assign29320_e43643_d_n7;
        locals.var_t0_dn8 = assign29320_e43643_d_n8;
        locals.var_t0_dn9 = assign29320_e43643_d_n9;
        locals.var_t0_dn10 = assign29320_e43643_d_n10;
        locals.var_t0_dn11 = assign29320_e43643_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign29330_e43654, assign29330_e43654_d_n3, assign29330_e43654_d_n4, assign29330_e43654_d_n5, assign29330_e43654_d_n6, assign29330_e43654_d_n7, assign29330_e43654_d_n8, assign29330_e43654_d_n9, assign29330_e43654_d_n10, assign29330_e43654_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard608 != 0.0)) {
        let assign29330_e43648: f64 = (-locals.var_devsign);
        let assign29330_e43650: f64 = (assign29330_e43648 * locals.var_vtm);
        let assign29330_e43652: f64 = (assign29330_e43650 * locals.var_t0);
        (assign29330_e43652, (assign29330_e43650 * locals.var_t0_dn3), (((assign29330_e43648 * locals.var_vtm_dn4) * locals.var_t0) + (assign29330_e43650 * locals.var_t0_dn4)), (((assign29330_e43648 * locals.var_vtm_dn5) * locals.var_t0) + (assign29330_e43650 * locals.var_t0_dn5)), (assign29330_e43650 * locals.var_t0_dn6), (assign29330_e43650 * locals.var_t0_dn7), (assign29330_e43650 * locals.var_t0_dn8), (assign29330_e43650 * locals.var_t0_dn9), (assign29330_e43650 * locals.var_t0_dn10), (assign29330_e43650 * locals.var_t0_dn11),)
    } else {
        (locals.var_vfbb, locals.var_vfbb_dn3, locals.var_vfbb_dn4, locals.var_vfbb_dn5, locals.var_vfbb_dn6, locals.var_vfbb_dn7, locals.var_vfbb_dn8, locals.var_vfbb_dn9, locals.var_vfbb_dn10, locals.var_vfbb_dn11,)
    }
};
        locals.var_vfbb = assign29330_e43654;
        locals.var_vfbb_dn3 = assign29330_e43654_d_n3;
        locals.var_vfbb_dn4 = assign29330_e43654_d_n4;
        locals.var_vfbb_dn5 = assign29330_e43654_d_n5;
        locals.var_vfbb_dn6 = assign29330_e43654_d_n6;
        locals.var_vfbb_dn7 = assign29330_e43654_d_n7;
        locals.var_vfbb_dn8 = assign29330_e43654_d_n8;
        locals.var_vfbb_dn9 = assign29330_e43654_d_n9;
        locals.var_vfbb_dn10 = assign29330_e43654_d_n10;
        locals.var_vfbb_dn11 = assign29330_e43654_d_n11;
        locals.var_vfbb_rv = 0.0;

        let (assign29340_e43671, assign29340_e43671_d_n3, assign29340_e43671_d_n4, assign29340_e43671_d_n5, assign29340_e43671_d_n6, assign29340_e43671_d_n7, assign29340_e43671_d_n8, assign29340_e43671_d_n9, assign29340_e43671_d_n10, assign29340_e43671_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard608 == 0.0)) {
        let assign29340_e43660: f64 = (-locals.var_ndep_i);
        let assign29340_e43662: f64 = (assign29340_e43660 * locals.var_nsub_i);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_ni;
        let assign29340_e43664: f64 = (assign29340_e43662 * __rspice_inv_cse_0);
        let assign29340_e43666: f64 = (assign29340_e43664 * __rspice_inv_cse_0);
        let assign29340_e43668: f64 = (assign29340_e43666).max(1e-38);
        let assign29340_e43669: f64 = (assign29340_e43668).ln();
        (assign29340_e43669, (if assign29340_e43666 >= 1e-38 { ((((((((-locals.var_ndep_i_dn3) * locals.var_nsub_i) * locals.var_ni) - (assign29340_e43662 * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign29340_e43664 * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign29340_e43668), (if assign29340_e43666 >= 1e-38 { ((((((((-locals.var_ndep_i_dn4) * locals.var_nsub_i) * locals.var_ni) - (assign29340_e43662 * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign29340_e43664 * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign29340_e43668), (if assign29340_e43666 >= 1e-38 { ((((((((-locals.var_ndep_i_dn5) * locals.var_nsub_i) * locals.var_ni) - (assign29340_e43662 * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign29340_e43664 * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign29340_e43668), (if assign29340_e43666 >= 1e-38 { ((((((((-locals.var_ndep_i_dn6) * locals.var_nsub_i) * locals.var_ni) - (assign29340_e43662 * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign29340_e43664 * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign29340_e43668), (if assign29340_e43666 >= 1e-38 { ((((((((-locals.var_ndep_i_dn7) * locals.var_nsub_i) * locals.var_ni) - (assign29340_e43662 * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign29340_e43664 * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign29340_e43668), (if assign29340_e43666 >= 1e-38 { ((((((((-locals.var_ndep_i_dn8) * locals.var_nsub_i) * locals.var_ni) - (assign29340_e43662 * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign29340_e43664 * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign29340_e43668), (if assign29340_e43666 >= 1e-38 { ((((((((-locals.var_ndep_i_dn9) * locals.var_nsub_i) * locals.var_ni) - (assign29340_e43662 * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign29340_e43664 * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign29340_e43668), (if assign29340_e43666 >= 1e-38 { ((((((((-locals.var_ndep_i_dn10) * locals.var_nsub_i) * locals.var_ni) - (assign29340_e43662 * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign29340_e43664 * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign29340_e43668), (if assign29340_e43666 >= 1e-38 { ((((((((-locals.var_ndep_i_dn11) * locals.var_nsub_i) * locals.var_ni) - (assign29340_e43662 * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign29340_e43664 * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign29340_e43668),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign29340_e43671;
        locals.var_t0_dn3 = assign29340_e43671_d_n3;
        locals.var_t0_dn4 = assign29340_e43671_d_n4;
        locals.var_t0_dn5 = assign29340_e43671_d_n5;
        locals.var_t0_dn6 = assign29340_e43671_d_n6;
        locals.var_t0_dn7 = assign29340_e43671_d_n7;
        locals.var_t0_dn8 = assign29340_e43671_d_n8;
        locals.var_t0_dn9 = assign29340_e43671_d_n9;
        locals.var_t0_dn10 = assign29340_e43671_d_n10;
        locals.var_t0_dn11 = assign29340_e43671_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign29350_e43683, assign29350_e43683_d_n3, assign29350_e43683_d_n4, assign29350_e43683_d_n5, assign29350_e43683_d_n6, assign29350_e43683_d_n7, assign29350_e43683_d_n8, assign29350_e43683_d_n9, assign29350_e43683_d_n10, assign29350_e43683_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard608 == 0.0)) {
        let assign29350_e43677: f64 = (-locals.var_devsign);
        let assign29350_e43679: f64 = (assign29350_e43677 * locals.var_vtm);
        let assign29350_e43681: f64 = (assign29350_e43679 * locals.var_t0);
        (assign29350_e43681, (assign29350_e43679 * locals.var_t0_dn3), (((assign29350_e43677 * locals.var_vtm_dn4) * locals.var_t0) + (assign29350_e43679 * locals.var_t0_dn4)), (((assign29350_e43677 * locals.var_vtm_dn5) * locals.var_t0) + (assign29350_e43679 * locals.var_t0_dn5)), (assign29350_e43679 * locals.var_t0_dn6), (assign29350_e43679 * locals.var_t0_dn7), (assign29350_e43679 * locals.var_t0_dn8), (assign29350_e43679 * locals.var_t0_dn9), (assign29350_e43679 * locals.var_t0_dn10), (assign29350_e43679 * locals.var_t0_dn11),)
    } else {
        (locals.var_vfbb, locals.var_vfbb_dn3, locals.var_vfbb_dn4, locals.var_vfbb_dn5, locals.var_vfbb_dn6, locals.var_vfbb_dn7, locals.var_vfbb_dn8, locals.var_vfbb_dn9, locals.var_vfbb_dn10, locals.var_vfbb_dn11,)
    }
};
        locals.var_vfbb = assign29350_e43683;
        locals.var_vfbb_dn3 = assign29350_e43683_d_n3;
        locals.var_vfbb_dn4 = assign29350_e43683_d_n4;
        locals.var_vfbb_dn5 = assign29350_e43683_d_n5;
        locals.var_vfbb_dn6 = assign29350_e43683_d_n6;
        locals.var_vfbb_dn7 = assign29350_e43683_d_n7;
        locals.var_vfbb_dn8 = assign29350_e43683_d_n8;
        locals.var_vfbb_dn9 = assign29350_e43683_d_n9;
        locals.var_vfbb_dn10 = assign29350_e43683_d_n10;
        locals.var_vfbb_dn11 = assign29350_e43683_d_n11;
        locals.var_vfbb_rv = 0.0;

        let (assign29360_e43689, assign29360_e43689_d_n3, assign29360_e43689_d_n4, assign29360_e43689_d_n5, assign29360_e43689_d_n6, assign29360_e43689_d_n7, assign29360_e43689_d_n8, assign29360_e43689_d_n9, assign29360_e43689_d_n10, assign29360_e43689_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29360_e43687: f64 = (locals.var_ves - locals.var_vfbb);
        (assign29360_e43687, (locals.var_ves_dn3 - locals.var_vfbb_dn3), (-locals.var_vfbb_dn4), (-locals.var_vfbb_dn5), (locals.var_ves_dn6 - locals.var_vfbb_dn6), (locals.var_ves_dn7 - locals.var_vfbb_dn7), (-locals.var_vfbb_dn8), (-locals.var_vfbb_dn9), (locals.var_ves_dn10 - locals.var_vfbb_dn10), (-locals.var_vfbb_dn11),)
    } else {
        (locals.var_vesfb, locals.var_vesfb_dn3, locals.var_vesfb_dn4, locals.var_vesfb_dn5, locals.var_vesfb_dn6, locals.var_vesfb_dn7, locals.var_vesfb_dn8, locals.var_vesfb_dn9, locals.var_vesfb_dn10, locals.var_vesfb_dn11,)
    }
};
        locals.var_vesfb = assign29360_e43689;
        locals.var_vesfb_dn3 = assign29360_e43689_d_n3;
        locals.var_vesfb_dn4 = assign29360_e43689_d_n4;
        locals.var_vesfb_dn5 = assign29360_e43689_d_n5;
        locals.var_vesfb_dn6 = assign29360_e43689_d_n6;
        locals.var_vesfb_dn7 = assign29360_e43689_d_n7;
        locals.var_vesfb_dn8 = assign29360_e43689_d_n8;
        locals.var_vesfb_dn9 = assign29360_e43689_d_n9;
        locals.var_vesfb_dn10 = assign29360_e43689_d_n10;
        locals.var_vesfb_dn11 = assign29360_e43689_d_n11;
        locals.var_vesfb_rv = 0.0;

        let (assign29370_e43695,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29370_e43693: f64 = (3.453133e-11 / p.p75);
        (assign29370_e43693,)
    } else {
        (locals.var_cbox_1,)
    }
};
        locals.var_cbox_1 = assign29370_e43695;
        locals.var_cbox_1_rv = 0.0;

        let (assign29380_e43713,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29380_e43699: f64 = (locals.var_kb1_i * p.p1388);
        let assign29380_e43701: f64 = (assign29380_e43699 * locals.var_cbox_1);
        let assign29380_e43704: f64 = (locals.var_wact / p.p1373);
        let assign29380_e43706: f64 = (assign29380_e43704 * p.p2);
        let assign29380_e43708: f64 = (assign29380_e43706 * locals.var_leffcvbg);
        let assign29380_e43710: f64 = (assign29380_e43708 + p.p1382);
        let assign29380_e43711: f64 = (assign29380_e43701 * assign29380_e43710);
        (assign29380_e43711,)
    } else {
        (locals.var_cboxwl,)
    }
};
        locals.var_cboxwl = assign29380_e43713;
        locals.var_cboxwl_rv = 0.0;

        let (assign29390_e43721, assign29390_e43721_d_n3, assign29390_e43721_d_n4, assign29390_e43721_d_n5, assign29390_e43721_d_n6, assign29390_e43721_d_n7, assign29390_e43721_d_n8, assign29390_e43721_d_n9, assign29390_e43721_d_n10, assign29390_e43721_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign29390_e43718: f64 = (locals.var_vesfb - locals.var_vbs);
        let assign29390_e43719: f64 = (locals.var_cboxwl * assign29390_e43718);
        (assign29390_e43719, (locals.var_cboxwl * locals.var_vesfb_dn3), (locals.var_cboxwl * locals.var_vesfb_dn4), (locals.var_cboxwl * locals.var_vesfb_dn5), (locals.var_cboxwl * (locals.var_vesfb_dn6 - locals.var_vbs_dn6)), (locals.var_cboxwl * (locals.var_vesfb_dn7 - locals.var_vbs_dn7)), (locals.var_cboxwl * locals.var_vesfb_dn8), (locals.var_cboxwl * locals.var_vesfb_dn9), (locals.var_cboxwl * (locals.var_vesfb_dn10 - locals.var_vbs_dn10)), (locals.var_cboxwl * locals.var_vesfb_dn11),)
    } else {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11,)
    }
};
        locals.var_qe1 = assign29390_e43721;
        locals.var_qe1_dn3 = assign29390_e43721_d_n3;
        locals.var_qe1_dn4 = assign29390_e43721_d_n4;
        locals.var_qe1_dn5 = assign29390_e43721_d_n5;
        locals.var_qe1_dn6 = assign29390_e43721_d_n6;
        locals.var_qe1_dn7 = assign29390_e43721_d_n7;
        locals.var_qe1_dn8 = assign29390_e43721_d_n8;
        locals.var_qe1_dn9 = assign29390_e43721_d_n9;
        locals.var_qe1_dn10 = assign29390_e43721_d_n10;
        locals.var_qe1_dn11 = assign29390_e43721_d_n11;
        locals.var_qe1_rv = 0.0;

        let (assign29400_e43725, assign29400_e43725_d_n3, assign29400_e43725_d_n4, assign29400_e43725_d_n5, assign29400_e43725_d_n6, assign29400_e43725_d_n7, assign29400_e43725_d_n8, assign29400_e43725_d_n9, assign29400_e43725_d_n10, assign29400_e43725_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11,)
    } else {
        (locals.var_qsub, locals.var_qsub_dn3, locals.var_qsub_dn4, locals.var_qsub_dn5, locals.var_qsub_dn6, locals.var_qsub_dn7, locals.var_qsub_dn8, locals.var_qsub_dn9, locals.var_qsub_dn10, locals.var_qsub_dn11,)
    }
};
        locals.var_qsub = assign29400_e43725;
        locals.var_qsub_dn3 = assign29400_e43725_d_n3;
        locals.var_qsub_dn4 = assign29400_e43725_d_n4;
        locals.var_qsub_dn5 = assign29400_e43725_d_n5;
        locals.var_qsub_dn6 = assign29400_e43725_d_n6;
        locals.var_qsub_dn7 = assign29400_e43725_d_n7;
        locals.var_qsub_dn8 = assign29400_e43725_d_n8;
        locals.var_qsub_dn9 = assign29400_e43725_d_n9;
        locals.var_qsub_dn10 = assign29400_e43725_d_n10;
        locals.var_qsub_dn11 = assign29400_e43725_d_n11;
        locals.var_qsub_rv = 0.0;

        let assign29410_e43728: f64 = if p.p47 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard609 = assign29410_e43728;
        locals.var_guard609_rv = 0.0;

        let (assign29420_e43745, assign29420_e43745_d_n3, assign29420_e43745_d_n4, assign29420_e43745_d_n5, assign29420_e43745_d_n6, assign29420_e43745_d_n7, assign29420_e43745_d_n8, assign29420_e43745_d_n9, assign29420_e43745_d_n10, assign29420_e43745_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign29420_e43737: f64 = (p.p74 / p.p75);
        let assign29420_e43738: f64 = (1.0 + assign29420_e43737);
        let assign29420_e43739: f64 = (p.p871 * assign29420_e43738);
        let assign29420_e43741: f64 = (assign29420_e43739).max(1e-38);
        let assign29420_e43742: f64 = (assign29420_e43741).ln();
        let assign29420_e43743: f64 = (p.p1395 * assign29420_e43742);
        (assign29420_e43743, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign29420_e43745;
        locals.var_t0_dn3 = assign29420_e43745_d_n3;
        locals.var_t0_dn4 = assign29420_e43745_d_n4;
        locals.var_t0_dn5 = assign29420_e43745_d_n5;
        locals.var_t0_dn6 = assign29420_e43745_d_n6;
        locals.var_t0_dn7 = assign29420_e43745_d_n7;
        locals.var_t0_dn8 = assign29420_e43745_d_n8;
        locals.var_t0_dn9 = assign29420_e43745_d_n9;
        locals.var_t0_dn10 = assign29420_e43745_d_n10;
        locals.var_t0_dn11 = assign29420_e43745_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign29430_e43753, assign29430_e43753_d_n3, assign29430_e43753_d_n4, assign29430_e43753_d_n5, assign29430_e43753_d_n6, assign29430_e43753_d_n7, assign29430_e43753_d_n8, assign29430_e43753_d_n9, assign29430_e43753_d_n10, assign29430_e43753_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign29430_e43751: f64 = (p.p19 - p.p1);
        (assign29430_e43751, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign29430_e43753;
        locals.var_t1_dn3 = assign29430_e43753_d_n3;
        locals.var_t1_dn4 = assign29430_e43753_d_n4;
        locals.var_t1_dn5 = assign29430_e43753_d_n5;
        locals.var_t1_dn6 = assign29430_e43753_d_n6;
        locals.var_t1_dn7 = assign29430_e43753_d_n7;
        locals.var_t1_dn8 = assign29430_e43753_d_n8;
        locals.var_t1_dn9 = assign29430_e43753_d_n9;
        locals.var_t1_dn10 = assign29430_e43753_d_n10;
        locals.var_t1_dn11 = assign29430_e43753_d_n11;
        locals.var_t1_rv = 0.0;

        let assign29440_e43756: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard610 = assign29440_e43756;
        locals.var_guard610_rv = 0.0;

        let (assign29450_e43766, assign29450_e43766_d_n3, assign29450_e43766_d_n4, assign29450_e43766_d_n5, assign29450_e43766_d_n6, assign29450_e43766_d_n7, assign29450_e43766_d_n8, assign29450_e43766_d_n9, assign29450_e43766_d_n10, assign29450_e43766_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) && (locals.var_guard610 != 0.0)) {
        let assign29450_e43764: f64 = (locals.var_t0 * locals.var_t1);
        (assign29450_e43764, ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)), ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)), ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)), ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)), ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)), ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)), ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)), ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)), ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)),)
    } else {
        (locals.var_csesw, locals.var_csesw_dn3, locals.var_csesw_dn4, locals.var_csesw_dn5, locals.var_csesw_dn6, locals.var_csesw_dn7, locals.var_csesw_dn8, locals.var_csesw_dn9, locals.var_csesw_dn10, locals.var_csesw_dn11,)
    }
};
        locals.var_csesw = assign29450_e43766;
        locals.var_csesw_dn3 = assign29450_e43766_d_n3;
        locals.var_csesw_dn4 = assign29450_e43766_d_n4;
        locals.var_csesw_dn5 = assign29450_e43766_d_n5;
        locals.var_csesw_dn6 = assign29450_e43766_d_n6;
        locals.var_csesw_dn7 = assign29450_e43766_d_n7;
        locals.var_csesw_dn8 = assign29450_e43766_d_n8;
        locals.var_csesw_dn9 = assign29450_e43766_d_n9;
        locals.var_csesw_dn10 = assign29450_e43766_d_n10;
        locals.var_csesw_dn11 = assign29450_e43766_d_n11;
        locals.var_csesw_rv = 0.0;

        let (assign29460_e43775, assign29460_e43775_d_n3, assign29460_e43775_d_n4, assign29460_e43775_d_n5, assign29460_e43775_d_n6, assign29460_e43775_d_n7, assign29460_e43775_d_n8, assign29460_e43775_d_n9, assign29460_e43775_d_n10, assign29460_e43775_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) && (locals.var_guard610 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_csesw, locals.var_csesw_dn3, locals.var_csesw_dn4, locals.var_csesw_dn5, locals.var_csesw_dn6, locals.var_csesw_dn7, locals.var_csesw_dn8, locals.var_csesw_dn9, locals.var_csesw_dn10, locals.var_csesw_dn11,)
    }
};
        locals.var_csesw = assign29460_e43775;
        locals.var_csesw_dn3 = assign29460_e43775_d_n3;
        locals.var_csesw_dn4 = assign29460_e43775_d_n4;
        locals.var_csesw_dn5 = assign29460_e43775_d_n5;
        locals.var_csesw_dn6 = assign29460_e43775_d_n6;
        locals.var_csesw_dn7 = assign29460_e43775_d_n7;
        locals.var_csesw_dn8 = assign29460_e43775_d_n8;
        locals.var_csesw_dn9 = assign29460_e43775_d_n9;
        locals.var_csesw_dn10 = assign29460_e43775_d_n10;
        locals.var_csesw_dn11 = assign29460_e43775_d_n11;
        locals.var_csesw_rv = 0.0;

        let (assign29470_e43783, assign29470_e43783_d_n3, assign29470_e43783_d_n4, assign29470_e43783_d_n5, assign29470_e43783_d_n6, assign29470_e43783_d_n7, assign29470_e43783_d_n8, assign29470_e43783_d_n9, assign29470_e43783_d_n10, assign29470_e43783_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign29470_e43781: f64 = (p.p20 - p.p1);
        (assign29470_e43781, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign29470_e43783;
        locals.var_t1_dn3 = assign29470_e43783_d_n3;
        locals.var_t1_dn4 = assign29470_e43783_d_n4;
        locals.var_t1_dn5 = assign29470_e43783_d_n5;
        locals.var_t1_dn6 = assign29470_e43783_d_n6;
        locals.var_t1_dn7 = assign29470_e43783_d_n7;
        locals.var_t1_dn8 = assign29470_e43783_d_n8;
        locals.var_t1_dn9 = assign29470_e43783_d_n9;
        locals.var_t1_dn10 = assign29470_e43783_d_n10;
        locals.var_t1_dn11 = assign29470_e43783_d_n11;
        locals.var_t1_rv = 0.0;

        let assign29480_e43786: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard611 = assign29480_e43786;
        locals.var_guard611_rv = 0.0;

        let (assign29490_e43796, assign29490_e43796_d_n3, assign29490_e43796_d_n4, assign29490_e43796_d_n5, assign29490_e43796_d_n6, assign29490_e43796_d_n7, assign29490_e43796_d_n8, assign29490_e43796_d_n9, assign29490_e43796_d_n10, assign29490_e43796_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) && (locals.var_guard611 != 0.0)) {
        let assign29490_e43794: f64 = (locals.var_t0 * locals.var_t1);
        (assign29490_e43794, ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)), ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)), ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)), ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)), ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)), ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)), ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)), ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)), ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)),)
    } else {
        (locals.var_cdesw, locals.var_cdesw_dn3, locals.var_cdesw_dn4, locals.var_cdesw_dn5, locals.var_cdesw_dn6, locals.var_cdesw_dn7, locals.var_cdesw_dn8, locals.var_cdesw_dn9, locals.var_cdesw_dn10, locals.var_cdesw_dn11,)
    }
};
        locals.var_cdesw = assign29490_e43796;
        locals.var_cdesw_dn3 = assign29490_e43796_d_n3;
        locals.var_cdesw_dn4 = assign29490_e43796_d_n4;
        locals.var_cdesw_dn5 = assign29490_e43796_d_n5;
        locals.var_cdesw_dn6 = assign29490_e43796_d_n6;
        locals.var_cdesw_dn7 = assign29490_e43796_d_n7;
        locals.var_cdesw_dn8 = assign29490_e43796_d_n8;
        locals.var_cdesw_dn9 = assign29490_e43796_d_n9;
        locals.var_cdesw_dn10 = assign29490_e43796_d_n10;
        locals.var_cdesw_dn11 = assign29490_e43796_d_n11;
        locals.var_cdesw_rv = 0.0;

        let (assign29500_e43805, assign29500_e43805_d_n3, assign29500_e43805_d_n4, assign29500_e43805_d_n5, assign29500_e43805_d_n6, assign29500_e43805_d_n7, assign29500_e43805_d_n8, assign29500_e43805_d_n9, assign29500_e43805_d_n10, assign29500_e43805_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) && (locals.var_guard611 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cdesw, locals.var_cdesw_dn3, locals.var_cdesw_dn4, locals.var_cdesw_dn5, locals.var_cdesw_dn6, locals.var_cdesw_dn7, locals.var_cdesw_dn8, locals.var_cdesw_dn9, locals.var_cdesw_dn10, locals.var_cdesw_dn11,)
    }
};
        locals.var_cdesw = assign29500_e43805;
        locals.var_cdesw_dn3 = assign29500_e43805_d_n3;
        locals.var_cdesw_dn4 = assign29500_e43805_d_n4;
        locals.var_cdesw_dn5 = assign29500_e43805_d_n5;
        locals.var_cdesw_dn6 = assign29500_e43805_d_n6;
        locals.var_cdesw_dn7 = assign29500_e43805_d_n7;
        locals.var_cdesw_dn8 = assign29500_e43805_d_n8;
        locals.var_cdesw_dn9 = assign29500_e43805_d_n9;
        locals.var_cdesw_dn10 = assign29500_e43805_d_n10;
        locals.var_cdesw_dn11 = assign29500_e43805_d_n11;
        locals.var_cdesw_rv = 0.0;

        let (assign29510_e43813,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign29510_e43811: f64 = (locals.var_cbox_1 * p.p17);
        (assign29510_e43811,)
    } else {
        (locals.var_csbox,)
    }
};
        locals.var_csbox = assign29510_e43813;
        locals.var_csbox_rv = 0.0;

        let (assign29520_e43821,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign29520_e43819: f64 = (p.p1396 * p.p17);
        (assign29520_e43819,)
    } else {
        (locals.var_csmin,)
    }
};
        locals.var_csmin = assign29520_e43821;
        locals.var_csmin_rv = 0.0;

        let (assign29530_e43829,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign29530_e43827: f64 = (locals.var_cbox_1 * p.p18);
        (assign29530_e43827,)
    } else {
        (locals.var_cdbox,)
    }
};
        locals.var_cdbox = assign29530_e43829;
        locals.var_cdbox_rv = 0.0;

        let (assign29540_e43837,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign29540_e43835: f64 = (p.p1396 * p.p18);
        (assign29540_e43835,)
    } else {
        (locals.var_cdmin,)
    }
};
        locals.var_cdmin = assign29540_e43837;
        locals.var_cdmin_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_85(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29550_e43846, assign29550_e43846_d_n3, assign29550_e43846_d_n4, assign29550_e43846_d_n5, assign29550_e43846_d_n6, assign29550_e43846_d_n7, assign29550_e43846_d_n8, assign29550_e43846_d_n9, assign29550_e43846_d_n10, assign29550_e43846_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign29550_e43842: f64 = (-locals.var_devsign);
        let assign29550_e43844: f64 = (assign29550_e43842 * locals.var_ves_1);
        (assign29550_e43844, (assign29550_e43842 * locals.var_ves_1_dn3), 0.0, 0.0, (assign29550_e43842 * locals.var_ves_1_dn6), (assign29550_e43842 * locals.var_ves_1_dn7), 0.0, 0.0, (assign29550_e43842 * locals.var_ves_1_dn10), 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign29550_e43846;
        locals.var_t10_dn3 = assign29550_e43846_d_n3;
        locals.var_t10_dn4 = assign29550_e43846_d_n4;
        locals.var_t10_dn5 = assign29550_e43846_d_n5;
        locals.var_t10_dn6 = assign29550_e43846_d_n6;
        locals.var_t10_dn7 = assign29550_e43846_d_n7;
        locals.var_t10_dn8 = assign29550_e43846_d_n8;
        locals.var_t10_dn9 = assign29550_e43846_d_n9;
        locals.var_t10_dn10 = assign29550_e43846_d_n10;
        locals.var_t10_dn11 = assign29550_e43846_d_n11;
        locals.var_t10_rv = 0.0;

        let (assign29560_e43855, assign29560_e43855_d_n3, assign29560_e43855_d_n4, assign29560_e43855_d_n5, assign29560_e43855_d_n6, assign29560_e43855_d_n7, assign29560_e43855_d_n8, assign29560_e43855_d_n9, assign29560_e43855_d_n10, assign29560_e43855_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign29560_e43851: f64 = (-locals.var_devsign);
        let assign29560_e43853: f64 = (assign29560_e43851 * locals.var_ved);
        (assign29560_e43853, (assign29560_e43851 * locals.var_ved_dn3), 0.0, 0.0, (assign29560_e43851 * locals.var_ved_dn6), (assign29560_e43851 * locals.var_ved_dn7), 0.0, 0.0, (assign29560_e43851 * locals.var_ved_dn10), 0.0,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign29560_e43855;
        locals.var_t11_dn3 = assign29560_e43855_d_n3;
        locals.var_t11_dn4 = assign29560_e43855_d_n4;
        locals.var_t11_dn5 = assign29560_e43855_d_n5;
        locals.var_t11_dn6 = assign29560_e43855_d_n6;
        locals.var_t11_dn7 = assign29560_e43855_d_n7;
        locals.var_t11_dn8 = assign29560_e43855_d_n8;
        locals.var_t11_dn9 = assign29560_e43855_d_n9;
        locals.var_t11_dn10 = assign29560_e43855_d_n10;
        locals.var_t11_dn11 = assign29560_e43855_d_n11;
        locals.var_t11_rv = 0.0;

        let assign29570_e43858: f64 = if p.p1396 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard612 = assign29570_e43858;
        locals.var_guard612_rv = 0.0;

        let (assign29580_e43873, assign29580_e43873_d_n3, assign29580_e43873_d_n4, assign29580_e43873_d_n5, assign29580_e43873_d_n6, assign29580_e43873_d_n7, assign29580_e43873_d_n8, assign29580_e43873_d_n9, assign29580_e43873_d_n10, assign29580_e43873_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) && (locals.var_guard612 != 0.0)) {
        let assign29580_e43865: f64 = (-0.5);
        let assign29580_e43868: f64 = (locals.var_cdbox - locals.var_cdmin);
        let assign29580_e43869: f64 = (assign29580_e43865 * assign29580_e43868);
        let assign29580_e43871: f64 = (assign29580_e43869 / p.p1399);
        (assign29580_e43871, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign29580_e43873;
        locals.var_t1_dn3 = assign29580_e43873_d_n3;
        locals.var_t1_dn4 = assign29580_e43873_d_n4;
        locals.var_t1_dn5 = assign29580_e43873_d_n5;
        locals.var_t1_dn6 = assign29580_e43873_d_n6;
        locals.var_t1_dn7 = assign29580_e43873_d_n7;
        locals.var_t1_dn8 = assign29580_e43873_d_n8;
        locals.var_t1_dn9 = assign29580_e43873_d_n9;
        locals.var_t1_dn10 = assign29580_e43873_d_n10;
        locals.var_t1_dn11 = assign29580_e43873_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign29590_e43890, assign29590_e43890_d_n3, assign29590_e43890_d_n4, assign29590_e43890_d_n5, assign29590_e43890_d_n6, assign29590_e43890_d_n7, assign29590_e43890_d_n8, assign29590_e43890_d_n9, assign29590_e43890_d_n10, assign29590_e43890_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) && (locals.var_guard612 != 0.0)) {
        let assign29590_e43880: f64 = (-p.p1399);
        let assign29590_e43882: f64 = (assign29590_e43880 * locals.var_t11);
        let assign29590_e43884: f64 = (assign29590_e43882 + p.p1400);
        let assign29590_e43885: f64 = (assign29590_e43884).cosh();
        let assign29590_e43887: f64 = (assign29590_e43885).max(1e-38);
        let assign29590_e43888: f64 = (assign29590_e43887).ln();
        (assign29590_e43888, (if assign29590_e43885 >= 1e-38 { ((assign29590_e43884).sinh() * (assign29590_e43880 * locals.var_t11_dn3)) } else { 0.0 } / assign29590_e43887), (if assign29590_e43885 >= 1e-38 { ((assign29590_e43884).sinh() * (assign29590_e43880 * locals.var_t11_dn4)) } else { 0.0 } / assign29590_e43887), (if assign29590_e43885 >= 1e-38 { ((assign29590_e43884).sinh() * (assign29590_e43880 * locals.var_t11_dn5)) } else { 0.0 } / assign29590_e43887), (if assign29590_e43885 >= 1e-38 { ((assign29590_e43884).sinh() * (assign29590_e43880 * locals.var_t11_dn6)) } else { 0.0 } / assign29590_e43887), (if assign29590_e43885 >= 1e-38 { ((assign29590_e43884).sinh() * (assign29590_e43880 * locals.var_t11_dn7)) } else { 0.0 } / assign29590_e43887), (if assign29590_e43885 >= 1e-38 { ((assign29590_e43884).sinh() * (assign29590_e43880 * locals.var_t11_dn8)) } else { 0.0 } / assign29590_e43887), (if assign29590_e43885 >= 1e-38 { ((assign29590_e43884).sinh() * (assign29590_e43880 * locals.var_t11_dn9)) } else { 0.0 } / assign29590_e43887), (if assign29590_e43885 >= 1e-38 { ((assign29590_e43884).sinh() * (assign29590_e43880 * locals.var_t11_dn10)) } else { 0.0 } / assign29590_e43887), (if assign29590_e43885 >= 1e-38 { ((assign29590_e43884).sinh() * (assign29590_e43880 * locals.var_t11_dn11)) } else { 0.0 } / assign29590_e43887),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign29590_e43890;
        locals.var_t2_dn3 = assign29590_e43890_d_n3;
        locals.var_t2_dn4 = assign29590_e43890_d_n4;
        locals.var_t2_dn5 = assign29590_e43890_d_n5;
        locals.var_t2_dn6 = assign29590_e43890_d_n6;
        locals.var_t2_dn7 = assign29590_e43890_d_n7;
        locals.var_t2_dn8 = assign29590_e43890_d_n8;
        locals.var_t2_dn9 = assign29590_e43890_d_n9;
        locals.var_t2_dn10 = assign29590_e43890_d_n10;
        locals.var_t2_dn11 = assign29590_e43890_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign29600_e43904, assign29600_e43904_d_n3, assign29600_e43904_d_n4, assign29600_e43904_d_n5, assign29600_e43904_d_n6, assign29600_e43904_d_n7, assign29600_e43904_d_n8, assign29600_e43904_d_n9, assign29600_e43904_d_n10, assign29600_e43904_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) && (locals.var_guard612 != 0.0)) {
        let assign29600_e43899: f64 = (locals.var_cdbox + locals.var_cdmin);
        let assign29600_e43900: f64 = (0.5 * assign29600_e43899);
        let assign29600_e43902: f64 = (assign29600_e43900 * locals.var_t11);
        (assign29600_e43902, (assign29600_e43900 * locals.var_t11_dn3), (assign29600_e43900 * locals.var_t11_dn4), (assign29600_e43900 * locals.var_t11_dn5), (assign29600_e43900 * locals.var_t11_dn6), (assign29600_e43900 * locals.var_t11_dn7), (assign29600_e43900 * locals.var_t11_dn8), (assign29600_e43900 * locals.var_t11_dn9), (assign29600_e43900 * locals.var_t11_dn10), (assign29600_e43900 * locals.var_t11_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign29600_e43904;
        locals.var_t3_dn3 = assign29600_e43904_d_n3;
        locals.var_t3_dn4 = assign29600_e43904_d_n4;
        locals.var_t3_dn5 = assign29600_e43904_d_n5;
        locals.var_t3_dn6 = assign29600_e43904_d_n6;
        locals.var_t3_dn7 = assign29600_e43904_d_n7;
        locals.var_t3_dn8 = assign29600_e43904_d_n8;
        locals.var_t3_dn9 = assign29600_e43904_d_n9;
        locals.var_t3_dn10 = assign29600_e43904_d_n10;
        locals.var_t3_dn11 = assign29600_e43904_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign29610_e43916, assign29610_e43916_d_n3, assign29610_e43916_d_n4, assign29610_e43916_d_n5, assign29610_e43916_d_n6, assign29610_e43916_d_n7, assign29610_e43916_d_n8, assign29610_e43916_d_n9, assign29610_e43916_d_n10, assign29610_e43916_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) && (locals.var_guard612 != 0.0)) {
        let assign29610_e43912: f64 = (locals.var_t1 * locals.var_t2);
        let assign29610_e43914: f64 = (assign29610_e43912 + locals.var_t3);
        (assign29610_e43914, (((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) + locals.var_t3_dn3), (((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) + locals.var_t3_dn4), (((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) + locals.var_t3_dn5), (((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) + locals.var_t3_dn6), (((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) + locals.var_t3_dn7), (((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) + locals.var_t3_dn8), (((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) + locals.var_t3_dn9), (((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) + locals.var_t3_dn10), (((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) + locals.var_t3_dn11),)
    } else {
        (locals.var_qde, locals.var_qde_dn3, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11,)
    }
};
        locals.var_qde = assign29610_e43916;
        locals.var_qde_dn3 = assign29610_e43916_d_n3;
        locals.var_qde_dn4 = assign29610_e43916_d_n4;
        locals.var_qde_dn5 = assign29610_e43916_d_n5;
        locals.var_qde_dn6 = assign29610_e43916_d_n6;
        locals.var_qde_dn7 = assign29610_e43916_d_n7;
        locals.var_qde_dn8 = assign29610_e43916_d_n8;
        locals.var_qde_dn9 = assign29610_e43916_d_n9;
        locals.var_qde_dn10 = assign29610_e43916_d_n10;
        locals.var_qde_dn11 = assign29610_e43916_d_n11;
        locals.var_qde_rv = 0.0;

        let (assign29620_e43931, assign29620_e43931_d_n3, assign29620_e43931_d_n4, assign29620_e43931_d_n5, assign29620_e43931_d_n6, assign29620_e43931_d_n7, assign29620_e43931_d_n8, assign29620_e43931_d_n9, assign29620_e43931_d_n10, assign29620_e43931_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) && (locals.var_guard612 != 0.0)) {
        let assign29620_e43923: f64 = (-0.5);
        let assign29620_e43926: f64 = (locals.var_csbox - locals.var_csmin);
        let assign29620_e43927: f64 = (assign29620_e43923 * assign29620_e43926);
        let assign29620_e43929: f64 = (assign29620_e43927 / p.p1397);
        (assign29620_e43929, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign29620_e43931;
        locals.var_t1_dn3 = assign29620_e43931_d_n3;
        locals.var_t1_dn4 = assign29620_e43931_d_n4;
        locals.var_t1_dn5 = assign29620_e43931_d_n5;
        locals.var_t1_dn6 = assign29620_e43931_d_n6;
        locals.var_t1_dn7 = assign29620_e43931_d_n7;
        locals.var_t1_dn8 = assign29620_e43931_d_n8;
        locals.var_t1_dn9 = assign29620_e43931_d_n9;
        locals.var_t1_dn10 = assign29620_e43931_d_n10;
        locals.var_t1_dn11 = assign29620_e43931_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign29630_e43948, assign29630_e43948_d_n3, assign29630_e43948_d_n4, assign29630_e43948_d_n5, assign29630_e43948_d_n6, assign29630_e43948_d_n7, assign29630_e43948_d_n8, assign29630_e43948_d_n9, assign29630_e43948_d_n10, assign29630_e43948_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) && (locals.var_guard612 != 0.0)) {
        let assign29630_e43938: f64 = (-p.p1397);
        let assign29630_e43940: f64 = (assign29630_e43938 * locals.var_t10);
        let assign29630_e43942: f64 = (assign29630_e43940 + p.p1398);
        let assign29630_e43943: f64 = (assign29630_e43942).cosh();
        let assign29630_e43945: f64 = (assign29630_e43943).max(1e-38);
        let assign29630_e43946: f64 = (assign29630_e43945).ln();
        (assign29630_e43946, (if assign29630_e43943 >= 1e-38 { ((assign29630_e43942).sinh() * (assign29630_e43938 * locals.var_t10_dn3)) } else { 0.0 } / assign29630_e43945), (if assign29630_e43943 >= 1e-38 { ((assign29630_e43942).sinh() * (assign29630_e43938 * locals.var_t10_dn4)) } else { 0.0 } / assign29630_e43945), (if assign29630_e43943 >= 1e-38 { ((assign29630_e43942).sinh() * (assign29630_e43938 * locals.var_t10_dn5)) } else { 0.0 } / assign29630_e43945), (if assign29630_e43943 >= 1e-38 { ((assign29630_e43942).sinh() * (assign29630_e43938 * locals.var_t10_dn6)) } else { 0.0 } / assign29630_e43945), (if assign29630_e43943 >= 1e-38 { ((assign29630_e43942).sinh() * (assign29630_e43938 * locals.var_t10_dn7)) } else { 0.0 } / assign29630_e43945), (if assign29630_e43943 >= 1e-38 { ((assign29630_e43942).sinh() * (assign29630_e43938 * locals.var_t10_dn8)) } else { 0.0 } / assign29630_e43945), (if assign29630_e43943 >= 1e-38 { ((assign29630_e43942).sinh() * (assign29630_e43938 * locals.var_t10_dn9)) } else { 0.0 } / assign29630_e43945), (if assign29630_e43943 >= 1e-38 { ((assign29630_e43942).sinh() * (assign29630_e43938 * locals.var_t10_dn10)) } else { 0.0 } / assign29630_e43945), (if assign29630_e43943 >= 1e-38 { ((assign29630_e43942).sinh() * (assign29630_e43938 * locals.var_t10_dn11)) } else { 0.0 } / assign29630_e43945),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign29630_e43948;
        locals.var_t2_dn3 = assign29630_e43948_d_n3;
        locals.var_t2_dn4 = assign29630_e43948_d_n4;
        locals.var_t2_dn5 = assign29630_e43948_d_n5;
        locals.var_t2_dn6 = assign29630_e43948_d_n6;
        locals.var_t2_dn7 = assign29630_e43948_d_n7;
        locals.var_t2_dn8 = assign29630_e43948_d_n8;
        locals.var_t2_dn9 = assign29630_e43948_d_n9;
        locals.var_t2_dn10 = assign29630_e43948_d_n10;
        locals.var_t2_dn11 = assign29630_e43948_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign29640_e43962, assign29640_e43962_d_n3, assign29640_e43962_d_n4, assign29640_e43962_d_n5, assign29640_e43962_d_n6, assign29640_e43962_d_n7, assign29640_e43962_d_n8, assign29640_e43962_d_n9, assign29640_e43962_d_n10, assign29640_e43962_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) && (locals.var_guard612 != 0.0)) {
        let assign29640_e43957: f64 = (locals.var_csbox + locals.var_csmin);
        let assign29640_e43958: f64 = (0.5 * assign29640_e43957);
        let assign29640_e43960: f64 = (assign29640_e43958 * locals.var_t10);
        (assign29640_e43960, (assign29640_e43958 * locals.var_t10_dn3), (assign29640_e43958 * locals.var_t10_dn4), (assign29640_e43958 * locals.var_t10_dn5), (assign29640_e43958 * locals.var_t10_dn6), (assign29640_e43958 * locals.var_t10_dn7), (assign29640_e43958 * locals.var_t10_dn8), (assign29640_e43958 * locals.var_t10_dn9), (assign29640_e43958 * locals.var_t10_dn10), (assign29640_e43958 * locals.var_t10_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign29640_e43962;
        locals.var_t3_dn3 = assign29640_e43962_d_n3;
        locals.var_t3_dn4 = assign29640_e43962_d_n4;
        locals.var_t3_dn5 = assign29640_e43962_d_n5;
        locals.var_t3_dn6 = assign29640_e43962_d_n6;
        locals.var_t3_dn7 = assign29640_e43962_d_n7;
        locals.var_t3_dn8 = assign29640_e43962_d_n8;
        locals.var_t3_dn9 = assign29640_e43962_d_n9;
        locals.var_t3_dn10 = assign29640_e43962_d_n10;
        locals.var_t3_dn11 = assign29640_e43962_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign29650_e43974, assign29650_e43974_d_n3, assign29650_e43974_d_n4, assign29650_e43974_d_n5, assign29650_e43974_d_n6, assign29650_e43974_d_n7, assign29650_e43974_d_n8, assign29650_e43974_d_n9, assign29650_e43974_d_n10, assign29650_e43974_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) && (locals.var_guard612 != 0.0)) {
        let assign29650_e43970: f64 = (locals.var_t1 * locals.var_t2);
        let assign29650_e43972: f64 = (assign29650_e43970 + locals.var_t3);
        (assign29650_e43972, (((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) + locals.var_t3_dn3), (((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) + locals.var_t3_dn4), (((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) + locals.var_t3_dn5), (((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) + locals.var_t3_dn6), (((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) + locals.var_t3_dn7), (((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) + locals.var_t3_dn8), (((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) + locals.var_t3_dn9), (((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) + locals.var_t3_dn10), (((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) + locals.var_t3_dn11),)
    } else {
        (locals.var_qse, locals.var_qse_dn3, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11,)
    }
};
        locals.var_qse = assign29650_e43974;
        locals.var_qse_dn3 = assign29650_e43974_d_n3;
        locals.var_qse_dn4 = assign29650_e43974_d_n4;
        locals.var_qse_dn5 = assign29650_e43974_d_n5;
        locals.var_qse_dn6 = assign29650_e43974_d_n6;
        locals.var_qse_dn7 = assign29650_e43974_d_n7;
        locals.var_qse_dn8 = assign29650_e43974_d_n8;
        locals.var_qse_dn9 = assign29650_e43974_d_n9;
        locals.var_qse_dn10 = assign29650_e43974_d_n10;
        locals.var_qse_dn11 = assign29650_e43974_d_n11;
        locals.var_qse_rv = 0.0;

        let (assign29660_e43985, assign29660_e43985_d_n3, assign29660_e43985_d_n4, assign29660_e43985_d_n5, assign29660_e43985_d_n6, assign29660_e43985_d_n7, assign29660_e43985_d_n8, assign29660_e43985_d_n9, assign29660_e43985_d_n10, assign29660_e43985_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) && (locals.var_guard612 == 0.0)) {
        let assign29660_e43983: f64 = (locals.var_csbox * locals.var_t10);
        (assign29660_e43983, (locals.var_csbox * locals.var_t10_dn3), (locals.var_csbox * locals.var_t10_dn4), (locals.var_csbox * locals.var_t10_dn5), (locals.var_csbox * locals.var_t10_dn6), (locals.var_csbox * locals.var_t10_dn7), (locals.var_csbox * locals.var_t10_dn8), (locals.var_csbox * locals.var_t10_dn9), (locals.var_csbox * locals.var_t10_dn10), (locals.var_csbox * locals.var_t10_dn11),)
    } else {
        (locals.var_qse, locals.var_qse_dn3, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11,)
    }
};
        locals.var_qse = assign29660_e43985;
        locals.var_qse_dn3 = assign29660_e43985_d_n3;
        locals.var_qse_dn4 = assign29660_e43985_d_n4;
        locals.var_qse_dn5 = assign29660_e43985_d_n5;
        locals.var_qse_dn6 = assign29660_e43985_d_n6;
        locals.var_qse_dn7 = assign29660_e43985_d_n7;
        locals.var_qse_dn8 = assign29660_e43985_d_n8;
        locals.var_qse_dn9 = assign29660_e43985_d_n9;
        locals.var_qse_dn10 = assign29660_e43985_d_n10;
        locals.var_qse_dn11 = assign29660_e43985_d_n11;
        locals.var_qse_rv = 0.0;

        let (assign29670_e43996, assign29670_e43996_d_n3, assign29670_e43996_d_n4, assign29670_e43996_d_n5, assign29670_e43996_d_n6, assign29670_e43996_d_n7, assign29670_e43996_d_n8, assign29670_e43996_d_n9, assign29670_e43996_d_n10, assign29670_e43996_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) && (locals.var_guard612 == 0.0)) {
        let assign29670_e43994: f64 = (locals.var_cdbox * locals.var_t11);
        (assign29670_e43994, (locals.var_cdbox * locals.var_t11_dn3), (locals.var_cdbox * locals.var_t11_dn4), (locals.var_cdbox * locals.var_t11_dn5), (locals.var_cdbox * locals.var_t11_dn6), (locals.var_cdbox * locals.var_t11_dn7), (locals.var_cdbox * locals.var_t11_dn8), (locals.var_cdbox * locals.var_t11_dn9), (locals.var_cdbox * locals.var_t11_dn10), (locals.var_cdbox * locals.var_t11_dn11),)
    } else {
        (locals.var_qde, locals.var_qde_dn3, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11,)
    }
};
        locals.var_qde = assign29670_e43996;
        locals.var_qde_dn3 = assign29670_e43996_d_n3;
        locals.var_qde_dn4 = assign29670_e43996_d_n4;
        locals.var_qde_dn5 = assign29670_e43996_d_n5;
        locals.var_qde_dn6 = assign29670_e43996_d_n6;
        locals.var_qde_dn7 = assign29670_e43996_d_n7;
        locals.var_qde_dn8 = assign29670_e43996_d_n8;
        locals.var_qde_dn9 = assign29670_e43996_d_n9;
        locals.var_qde_dn10 = assign29670_e43996_d_n10;
        locals.var_qde_dn11 = assign29670_e43996_d_n11;
        locals.var_qde_rv = 0.0;

        let (assign29680_e44006, assign29680_e44006_d_n3, assign29680_e44006_d_n4, assign29680_e44006_d_n5, assign29680_e44006_d_n6, assign29680_e44006_d_n7, assign29680_e44006_d_n8, assign29680_e44006_d_n9, assign29680_e44006_d_n10, assign29680_e44006_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign29680_e44003: f64 = (locals.var_csesw * locals.var_t10);
        let assign29680_e44004: f64 = (locals.var_qse + assign29680_e44003);
        (assign29680_e44004, (locals.var_qse_dn3 + ((locals.var_csesw_dn3 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn3))), (locals.var_qse_dn4 + ((locals.var_csesw_dn4 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn4))), (locals.var_qse_dn5 + ((locals.var_csesw_dn5 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn5))), (locals.var_qse_dn6 + ((locals.var_csesw_dn6 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn6))), (locals.var_qse_dn7 + ((locals.var_csesw_dn7 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn7))), (locals.var_qse_dn8 + ((locals.var_csesw_dn8 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn8))), (locals.var_qse_dn9 + ((locals.var_csesw_dn9 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn9))), (locals.var_qse_dn10 + ((locals.var_csesw_dn10 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn10))), (locals.var_qse_dn11 + ((locals.var_csesw_dn11 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn11))),)
    } else {
        (locals.var_qse, locals.var_qse_dn3, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11,)
    }
};
        locals.var_qse = assign29680_e44006;
        locals.var_qse_dn3 = assign29680_e44006_d_n3;
        locals.var_qse_dn4 = assign29680_e44006_d_n4;
        locals.var_qse_dn5 = assign29680_e44006_d_n5;
        locals.var_qse_dn6 = assign29680_e44006_d_n6;
        locals.var_qse_dn7 = assign29680_e44006_d_n7;
        locals.var_qse_dn8 = assign29680_e44006_d_n8;
        locals.var_qse_dn9 = assign29680_e44006_d_n9;
        locals.var_qse_dn10 = assign29680_e44006_d_n10;
        locals.var_qse_dn11 = assign29680_e44006_d_n11;
        locals.var_qse_rv = 0.0;

        let (assign29690_e44016, assign29690_e44016_d_n3, assign29690_e44016_d_n4, assign29690_e44016_d_n5, assign29690_e44016_d_n6, assign29690_e44016_d_n7, assign29690_e44016_d_n8, assign29690_e44016_d_n9, assign29690_e44016_d_n10, assign29690_e44016_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 != 0.0)) {
        let assign29690_e44013: f64 = (locals.var_cdesw * locals.var_t11);
        let assign29690_e44014: f64 = (locals.var_qde + assign29690_e44013);
        (assign29690_e44014, (locals.var_qde_dn3 + ((locals.var_cdesw_dn3 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn3))), (locals.var_qde_dn4 + ((locals.var_cdesw_dn4 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn4))), (locals.var_qde_dn5 + ((locals.var_cdesw_dn5 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn5))), (locals.var_qde_dn6 + ((locals.var_cdesw_dn6 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn6))), (locals.var_qde_dn7 + ((locals.var_cdesw_dn7 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn7))), (locals.var_qde_dn8 + ((locals.var_cdesw_dn8 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn8))), (locals.var_qde_dn9 + ((locals.var_cdesw_dn9 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn9))), (locals.var_qde_dn10 + ((locals.var_cdesw_dn10 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn10))), (locals.var_qde_dn11 + ((locals.var_cdesw_dn11 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn11))),)
    } else {
        (locals.var_qde, locals.var_qde_dn3, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11,)
    }
};
        locals.var_qde = assign29690_e44016;
        locals.var_qde_dn3 = assign29690_e44016_d_n3;
        locals.var_qde_dn4 = assign29690_e44016_d_n4;
        locals.var_qde_dn5 = assign29690_e44016_d_n5;
        locals.var_qde_dn6 = assign29690_e44016_d_n6;
        locals.var_qde_dn7 = assign29690_e44016_d_n7;
        locals.var_qde_dn8 = assign29690_e44016_d_n8;
        locals.var_qde_dn9 = assign29690_e44016_d_n9;
        locals.var_qde_dn10 = assign29690_e44016_d_n10;
        locals.var_qde_dn11 = assign29690_e44016_d_n11;
        locals.var_qde_rv = 0.0;

        let (assign29700_e44023, assign29700_e44023_d_n3, assign29700_e44023_d_n4, assign29700_e44023_d_n5, assign29700_e44023_d_n6, assign29700_e44023_d_n7, assign29700_e44023_d_n8, assign29700_e44023_d_n9, assign29700_e44023_d_n10, assign29700_e44023_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qse, locals.var_qse_dn3, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11,)
    }
};
        locals.var_qse = assign29700_e44023;
        locals.var_qse_dn3 = assign29700_e44023_d_n3;
        locals.var_qse_dn4 = assign29700_e44023_d_n4;
        locals.var_qse_dn5 = assign29700_e44023_d_n5;
        locals.var_qse_dn6 = assign29700_e44023_d_n6;
        locals.var_qse_dn7 = assign29700_e44023_d_n7;
        locals.var_qse_dn8 = assign29700_e44023_d_n8;
        locals.var_qse_dn9 = assign29700_e44023_d_n9;
        locals.var_qse_dn10 = assign29700_e44023_d_n10;
        locals.var_qse_dn11 = assign29700_e44023_d_n11;
        locals.var_qse_rv = 0.0;

        let (assign29710_e44030, assign29710_e44030_d_n3, assign29710_e44030_d_n4, assign29710_e44030_d_n5, assign29710_e44030_d_n6, assign29710_e44030_d_n7, assign29710_e44030_d_n8, assign29710_e44030_d_n9, assign29710_e44030_d_n10, assign29710_e44030_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard609 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qde, locals.var_qde_dn3, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11,)
    }
};
        locals.var_qde = assign29710_e44030;
        locals.var_qde_dn3 = assign29710_e44030_d_n3;
        locals.var_qde_dn4 = assign29710_e44030_d_n4;
        locals.var_qde_dn5 = assign29710_e44030_d_n5;
        locals.var_qde_dn6 = assign29710_e44030_d_n6;
        locals.var_qde_dn7 = assign29710_e44030_d_n7;
        locals.var_qde_dn8 = assign29710_e44030_d_n8;
        locals.var_qde_dn9 = assign29710_e44030_d_n9;
        locals.var_qde_dn10 = assign29710_e44030_d_n10;
        locals.var_qde_dn11 = assign29710_e44030_d_n11;
        locals.var_qde_rv = 0.0;

        let assign29720_e44033: f64 = if p.p45 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard613 = assign29720_e44033;
        locals.var_guard613_rv = 0.0;

        let (assign29730_e44041,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29730_e44039: f64 = (p.p140 + p.p25);
        (assign29730_e44039,)
    } else {
        (locals.var_vfbagbcp2_i,)
    }
};
        locals.var_vfbagbcp2_i = assign29730_e44041;
        locals.var_vfbagbcp2_i_rv = 0.0;

        let (assign29740_e44049, assign29740_e44049_d_n4, assign29740_e44049_d_n5, assign29740_e44049_d_n8, assign29740_e44049_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29740_e44047: f64 = (locals.var_vg1 * locals.var_inv_vt);
        (assign29740_e44047, (locals.var_vg1 * locals.var_inv_vt_dn4), (locals.var_vg1 * locals.var_inv_vt_dn5), (locals.var_vg1_dn8 * locals.var_inv_vt), (locals.var_vg1_dn11 * locals.var_inv_vt),)
    } else {
        (locals.var_vg1_1, locals.var_vg1_1_dn4, locals.var_vg1_1_dn5, locals.var_vg1_1_dn8, locals.var_vg1_1_dn11,)
    }
};
        locals.var_vg1_1 = assign29740_e44049;
        locals.var_vg1_1_dn4 = assign29740_e44049_d_n4;
        locals.var_vg1_1_dn5 = assign29740_e44049_d_n5;
        locals.var_vg1_1_dn8 = assign29740_e44049_d_n8;
        locals.var_vg1_1_dn11 = assign29740_e44049_d_n11;
        locals.var_vg1_1_rv = 0.0;

        let (assign29750_e44057, assign29750_e44057_d_n4, assign29750_e44057_d_n5, assign29750_e44057_d_n6, assign29750_e44057_d_n7, assign29750_e44057_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29750_e44055: f64 = (locals.var_vs1 * locals.var_inv_vt);
        (assign29750_e44055, (locals.var_vs1 * locals.var_inv_vt_dn4), (locals.var_vs1 * locals.var_inv_vt_dn5), (locals.var_vs1_dn6 * locals.var_inv_vt), (locals.var_vs1_dn7 * locals.var_inv_vt), (locals.var_vs1_dn11 * locals.var_inv_vt),)
    } else {
        (locals.var_vs1_1, locals.var_vs1_1_dn4, locals.var_vs1_1_dn5, locals.var_vs1_1_dn6, locals.var_vs1_1_dn7, locals.var_vs1_1_dn11,)
    }
};
        locals.var_vs1_1 = assign29750_e44057;
        locals.var_vs1_1_dn4 = assign29750_e44057_d_n4;
        locals.var_vs1_1_dn5 = assign29750_e44057_d_n5;
        locals.var_vs1_1_dn6 = assign29750_e44057_d_n6;
        locals.var_vs1_1_dn7 = assign29750_e44057_d_n7;
        locals.var_vs1_1_dn11 = assign29750_e44057_d_n11;
        locals.var_vs1_1_rv = 0.0;

        let (assign29760_e44065, assign29760_e44065_d_n3, assign29760_e44065_d_n4, assign29760_e44065_d_n5, assign29760_e44065_d_n6, assign29760_e44065_d_n7, assign29760_e44065_d_n8, assign29760_e44065_d_n9, assign29760_e44065_d_n10, assign29760_e44065_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29760_e44063: f64 = (locals.var_vfbagbcp2_i * locals.var_inv_vt);
        (assign29760_e44063, 0.0, (locals.var_vfbagbcp2_i * locals.var_inv_vt_dn4), (locals.var_vfbagbcp2_i * locals.var_inv_vt_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfb, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11,)
    }
};
        locals.var_vfb = assign29760_e44065;
        locals.var_vfb_dn3 = assign29760_e44065_d_n3;
        locals.var_vfb_dn4 = assign29760_e44065_d_n4;
        locals.var_vfb_dn5 = assign29760_e44065_d_n5;
        locals.var_vfb_dn6 = assign29760_e44065_d_n6;
        locals.var_vfb_dn7 = assign29760_e44065_d_n7;
        locals.var_vfb_dn8 = assign29760_e44065_d_n8;
        locals.var_vfb_dn9 = assign29760_e44065_d_n9;
        locals.var_vfb_dn10 = assign29760_e44065_d_n10;
        locals.var_vfb_dn11 = assign29760_e44065_d_n11;
        locals.var_vfb_rv = 0.0;

        let (assign29770_e44073, assign29770_e44073_d_n3, assign29770_e44073_d_n4, assign29770_e44073_d_n5, assign29770_e44073_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29770_e44071: f64 = (locals.var_ve1 * locals.var_inv_vt);
        (assign29770_e44071, (locals.var_ve1_dn3 * locals.var_inv_vt), (locals.var_ve1 * locals.var_inv_vt_dn4), (locals.var_ve1 * locals.var_inv_vt_dn5), (locals.var_ve1_dn11 * locals.var_inv_vt),)
    } else {
        (locals.var_ve1_1, locals.var_ve1_1_dn3, locals.var_ve1_1_dn4, locals.var_ve1_1_dn5, locals.var_ve1_1_dn11,)
    }
};
        locals.var_ve1_1 = assign29770_e44073;
        locals.var_ve1_1_dn3 = assign29770_e44073_d_n3;
        locals.var_ve1_1_dn4 = assign29770_e44073_d_n4;
        locals.var_ve1_1_dn5 = assign29770_e44073_d_n5;
        locals.var_ve1_1_dn11 = assign29770_e44073_d_n11;
        locals.var_ve1_1_rv = 0.0;

        let (assign29780_e44081, assign29780_e44081_d_n3, assign29780_e44081_d_n4, assign29780_e44081_d_n5, assign29780_e44081_d_n6, assign29780_e44081_d_n7, assign29780_e44081_d_n8, assign29780_e44081_d_n9, assign29780_e44081_d_n10, assign29780_e44081_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29780_e44079: f64 = (locals.var_vg1_1 - locals.var_vfb);
        (assign29780_e44079, (-locals.var_vfb_dn3), (locals.var_vg1_1_dn4 - locals.var_vfb_dn4), (locals.var_vg1_1_dn5 - locals.var_vfb_dn5), (-locals.var_vfb_dn6), (-locals.var_vfb_dn7), (locals.var_vg1_1_dn8 - locals.var_vfb_dn8), (-locals.var_vfb_dn9), (-locals.var_vfb_dn10), (locals.var_vg1_1_dn11 - locals.var_vfb_dn11),)
    } else {
        (locals.var_vgfbagbcp2, locals.var_vgfbagbcp2_dn3, locals.var_vgfbagbcp2_dn4, locals.var_vgfbagbcp2_dn5, locals.var_vgfbagbcp2_dn6, locals.var_vgfbagbcp2_dn7, locals.var_vgfbagbcp2_dn8, locals.var_vgfbagbcp2_dn9, locals.var_vgfbagbcp2_dn10, locals.var_vgfbagbcp2_dn11,)
    }
};
        locals.var_vgfbagbcp2 = assign29780_e44081;
        locals.var_vgfbagbcp2_dn3 = assign29780_e44081_d_n3;
        locals.var_vgfbagbcp2_dn4 = assign29780_e44081_d_n4;
        locals.var_vgfbagbcp2_dn5 = assign29780_e44081_d_n5;
        locals.var_vgfbagbcp2_dn6 = assign29780_e44081_d_n6;
        locals.var_vgfbagbcp2_dn7 = assign29780_e44081_d_n7;
        locals.var_vgfbagbcp2_dn8 = assign29780_e44081_d_n8;
        locals.var_vgfbagbcp2_dn9 = assign29780_e44081_d_n9;
        locals.var_vgfbagbcp2_dn10 = assign29780_e44081_d_n10;
        locals.var_vgfbagbcp2_dn11 = assign29780_e44081_d_n11;
        locals.var_vgfbagbcp2_rv = 0.0;

        let (assign29790_e44092, assign29790_e44092_d_n3, assign29790_e44092_d_n4, assign29790_e44092_d_n5, assign29790_e44092_d_n6, assign29790_e44092_d_n7, assign29790_e44092_d_n8, assign29790_e44092_d_n9, assign29790_e44092_d_n10, assign29790_e44092_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29790_e44087: f64 = (p.p141 / locals.var_ni);
        let assign29790_e44089: f64 = (assign29790_e44087).max(1e-38);
        let assign29790_e44090: f64 = (assign29790_e44089).ln();
        (assign29790_e44090, (if assign29790_e44087 >= 1e-38 { (-((p.p141 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign29790_e44089), (if assign29790_e44087 >= 1e-38 { (-((p.p141 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign29790_e44089), (if assign29790_e44087 >= 1e-38 { (-((p.p141 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign29790_e44089), (if assign29790_e44087 >= 1e-38 { (-((p.p141 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign29790_e44089), (if assign29790_e44087 >= 1e-38 { (-((p.p141 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign29790_e44089), (if assign29790_e44087 >= 1e-38 { (-((p.p141 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign29790_e44089), (if assign29790_e44087 >= 1e-38 { (-((p.p141 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign29790_e44089), (if assign29790_e44087 >= 1e-38 { (-((p.p141 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign29790_e44089), (if assign29790_e44087 >= 1e-38 { (-((p.p141 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign29790_e44089),)
    } else {
        (locals.var_phibagbcp2, locals.var_phibagbcp2_dn3, locals.var_phibagbcp2_dn4, locals.var_phibagbcp2_dn5, locals.var_phibagbcp2_dn6, locals.var_phibagbcp2_dn7, locals.var_phibagbcp2_dn8, locals.var_phibagbcp2_dn9, locals.var_phibagbcp2_dn10, locals.var_phibagbcp2_dn11,)
    }
};
        locals.var_phibagbcp2 = assign29790_e44092;
        locals.var_phibagbcp2_dn3 = assign29790_e44092_d_n3;
        locals.var_phibagbcp2_dn4 = assign29790_e44092_d_n4;
        locals.var_phibagbcp2_dn5 = assign29790_e44092_d_n5;
        locals.var_phibagbcp2_dn6 = assign29790_e44092_d_n6;
        locals.var_phibagbcp2_dn7 = assign29790_e44092_d_n7;
        locals.var_phibagbcp2_dn8 = assign29790_e44092_d_n8;
        locals.var_phibagbcp2_dn9 = assign29790_e44092_d_n9;
        locals.var_phibagbcp2_dn10 = assign29790_e44092_d_n10;
        locals.var_phibagbcp2_dn11 = assign29790_e44092_d_n11;
        locals.var_phibagbcp2_rv = 0.0;

        let (assign29800_e44109, assign29800_e44109_d_n4, assign29800_e44109_d_n5,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29800_e44098: f64 = (2.0 * 1.602176462e-19);
        let assign29800_e44100: f64 = (assign29800_e44098 * locals.var_epssi);
        let assign29800_e44102: f64 = (assign29800_e44100 * p.p141);
        let assign29800_e44104: f64 = (assign29800_e44102 * locals.var_inv_vt);
        let assign29800_e44105: f64 = (assign29800_e44104).sqrt();
        let assign29800_e44107: f64 = (assign29800_e44105 / locals.var_cox);
        (assign29800_e44107, (((assign29800_e44102 * locals.var_inv_vt_dn4) / (2.0 * assign29800_e44105)) / locals.var_cox), (((assign29800_e44102 * locals.var_inv_vt_dn5) / (2.0 * assign29800_e44105)) / locals.var_cox),)
    } else {
        (locals.var_gamagbcp2, locals.var_gamagbcp2_dn4, locals.var_gamagbcp2_dn5,)
    }
};
        locals.var_gamagbcp2 = assign29800_e44109;
        locals.var_gamagbcp2_dn4 = assign29800_e44109_d_n4;
        locals.var_gamagbcp2_dn5 = assign29800_e44109_d_n5;
        locals.var_gamagbcp2_rv = 0.0;

        let (assign29810_e44115, assign29810_e44115_d_n3, assign29810_e44115_d_n4, assign29810_e44115_d_n5, assign29810_e44115_d_n6, assign29810_e44115_d_n7, assign29810_e44115_d_n8, assign29810_e44115_d_n9, assign29810_e44115_d_n10, assign29810_e44115_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        (locals.var_gamagbcp2, 0.0, locals.var_gamagbcp2_dn4, locals.var_gamagbcp2_dn5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gam, locals.var_gam_dn3, locals.var_gam_dn4, locals.var_gam_dn5, locals.var_gam_dn6, locals.var_gam_dn7, locals.var_gam_dn8, locals.var_gam_dn9, locals.var_gam_dn10, locals.var_gam_dn11,)
    }
};
        locals.var_gam = assign29810_e44115;
        locals.var_gam_dn3 = assign29810_e44115_d_n3;
        locals.var_gam_dn4 = assign29810_e44115_d_n4;
        locals.var_gam_dn5 = assign29810_e44115_d_n5;
        locals.var_gam_dn6 = assign29810_e44115_d_n6;
        locals.var_gam_dn7 = assign29810_e44115_d_n7;
        locals.var_gam_dn8 = assign29810_e44115_d_n8;
        locals.var_gam_dn9 = assign29810_e44115_d_n9;
        locals.var_gam_dn10 = assign29810_e44115_d_n10;
        locals.var_gam_dn11 = assign29810_e44115_d_n11;
        locals.var_gam_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_86(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29820_e44121, assign29820_e44121_d_n3, assign29820_e44121_d_n4, assign29820_e44121_d_n5, assign29820_e44121_d_n6, assign29820_e44121_d_n7, assign29820_e44121_d_n8, assign29820_e44121_d_n9, assign29820_e44121_d_n10, assign29820_e44121_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        (locals.var_vgfbagbcp2, locals.var_vgfbagbcp2_dn3, locals.var_vgfbagbcp2_dn4, locals.var_vgfbagbcp2_dn5, locals.var_vgfbagbcp2_dn6, locals.var_vgfbagbcp2_dn7, locals.var_vgfbagbcp2_dn8, locals.var_vgfbagbcp2_dn9, locals.var_vgfbagbcp2_dn10, locals.var_vgfbagbcp2_dn11,)
    } else {
        (locals.var_vgfb1, locals.var_vgfb1_dn3, locals.var_vgfb1_dn4, locals.var_vgfb1_dn5, locals.var_vgfb1_dn6, locals.var_vgfb1_dn7, locals.var_vgfb1_dn8, locals.var_vgfb1_dn9, locals.var_vgfb1_dn10, locals.var_vgfb1_dn11,)
    }
};
        locals.var_vgfb1 = assign29820_e44121;
        locals.var_vgfb1_dn3 = assign29820_e44121_d_n3;
        locals.var_vgfb1_dn4 = assign29820_e44121_d_n4;
        locals.var_vgfb1_dn5 = assign29820_e44121_d_n5;
        locals.var_vgfb1_dn6 = assign29820_e44121_d_n6;
        locals.var_vgfb1_dn7 = assign29820_e44121_d_n7;
        locals.var_vgfb1_dn8 = assign29820_e44121_d_n8;
        locals.var_vgfb1_dn9 = assign29820_e44121_d_n9;
        locals.var_vgfb1_dn10 = assign29820_e44121_d_n10;
        locals.var_vgfb1_dn11 = assign29820_e44121_d_n11;
        locals.var_vgfb1_rv = 0.0;

        let (assign29830_e44129, assign29830_e44129_d_n3, assign29830_e44129_d_n4, assign29830_e44129_d_n5, assign29830_e44129_d_n6, assign29830_e44129_d_n7, assign29830_e44129_d_n8, assign29830_e44129_d_n9, assign29830_e44129_d_n10, assign29830_e44129_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29830_e44127: f64 = (locals.var_vfbb_i * locals.var_inv_vt);
        (assign29830_e44127, 0.0, (locals.var_vfbb_i * locals.var_inv_vt_dn4), (locals.var_vfbb_i * locals.var_inv_vt_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfbb, locals.var_vfbb_dn3, locals.var_vfbb_dn4, locals.var_vfbb_dn5, locals.var_vfbb_dn6, locals.var_vfbb_dn7, locals.var_vfbb_dn8, locals.var_vfbb_dn9, locals.var_vfbb_dn10, locals.var_vfbb_dn11,)
    }
};
        locals.var_vfbb = assign29830_e44129;
        locals.var_vfbb_dn3 = assign29830_e44129_d_n3;
        locals.var_vfbb_dn4 = assign29830_e44129_d_n4;
        locals.var_vfbb_dn5 = assign29830_e44129_d_n5;
        locals.var_vfbb_dn6 = assign29830_e44129_d_n6;
        locals.var_vfbb_dn7 = assign29830_e44129_d_n7;
        locals.var_vfbb_dn8 = assign29830_e44129_d_n8;
        locals.var_vfbb_dn9 = assign29830_e44129_d_n9;
        locals.var_vfbb_dn10 = assign29830_e44129_d_n10;
        locals.var_vfbb_dn11 = assign29830_e44129_d_n11;
        locals.var_vfbb_rv = 0.0;

        let (assign29840_e44137, assign29840_e44137_d_n3, assign29840_e44137_d_n4, assign29840_e44137_d_n5, assign29840_e44137_d_n6, assign29840_e44137_d_n7, assign29840_e44137_d_n8, assign29840_e44137_d_n9, assign29840_e44137_d_n10, assign29840_e44137_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29840_e44135: f64 = (locals.var_ve1_1 - locals.var_vfbb);
        (assign29840_e44135, (locals.var_ve1_1_dn3 - locals.var_vfbb_dn3), (locals.var_ve1_1_dn4 - locals.var_vfbb_dn4), (locals.var_ve1_1_dn5 - locals.var_vfbb_dn5), (-locals.var_vfbb_dn6), (-locals.var_vfbb_dn7), (-locals.var_vfbb_dn8), (-locals.var_vfbb_dn9), (-locals.var_vfbb_dn10), (locals.var_ve1_1_dn11 - locals.var_vfbb_dn11),)
    } else {
        (locals.var_vgfbb, locals.var_vgfbb_dn3, locals.var_vgfbb_dn4, locals.var_vgfbb_dn5, locals.var_vgfbb_dn6, locals.var_vgfbb_dn7, locals.var_vgfbb_dn8, locals.var_vgfbb_dn9, locals.var_vgfbb_dn10, locals.var_vgfbb_dn11,)
    }
};
        locals.var_vgfbb = assign29840_e44137;
        locals.var_vgfbb_dn3 = assign29840_e44137_d_n3;
        locals.var_vgfbb_dn4 = assign29840_e44137_d_n4;
        locals.var_vgfbb_dn5 = assign29840_e44137_d_n5;
        locals.var_vgfbb_dn6 = assign29840_e44137_d_n6;
        locals.var_vgfbb_dn7 = assign29840_e44137_d_n7;
        locals.var_vgfbb_dn8 = assign29840_e44137_d_n8;
        locals.var_vgfbb_dn9 = assign29840_e44137_d_n9;
        locals.var_vgfbb_dn10 = assign29840_e44137_d_n10;
        locals.var_vgfbb_dn11 = assign29840_e44137_d_n11;
        locals.var_vgfbb_rv = 0.0;

        let (assign29850_e44145, assign29850_e44145_d_n3, assign29850_e44145_d_n4, assign29850_e44145_d_n5, assign29850_e44145_d_n6, assign29850_e44145_d_n7, assign29850_e44145_d_n8, assign29850_e44145_d_n9, assign29850_e44145_d_n10, assign29850_e44145_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29850_e44143: f64 = (1.0 / locals.var_gam);
        (assign29850_e44143, (-(locals.var_gam_dn3 / (locals.var_gam * locals.var_gam))), (-(locals.var_gam_dn4 / (locals.var_gam * locals.var_gam))), (-(locals.var_gam_dn5 / (locals.var_gam * locals.var_gam))), (-(locals.var_gam_dn6 / (locals.var_gam * locals.var_gam))), (-(locals.var_gam_dn7 / (locals.var_gam * locals.var_gam))), (-(locals.var_gam_dn8 / (locals.var_gam * locals.var_gam))), (-(locals.var_gam_dn9 / (locals.var_gam * locals.var_gam))), (-(locals.var_gam_dn10 / (locals.var_gam * locals.var_gam))), (-(locals.var_gam_dn11 / (locals.var_gam * locals.var_gam))),)
    } else {
        (locals.var_inv_gam, locals.var_inv_gam_dn3, locals.var_inv_gam_dn4, locals.var_inv_gam_dn5, locals.var_inv_gam_dn6, locals.var_inv_gam_dn7, locals.var_inv_gam_dn8, locals.var_inv_gam_dn9, locals.var_inv_gam_dn10, locals.var_inv_gam_dn11,)
    }
};
        locals.var_inv_gam = assign29850_e44145;
        locals.var_inv_gam_dn3 = assign29850_e44145_d_n3;
        locals.var_inv_gam_dn4 = assign29850_e44145_d_n4;
        locals.var_inv_gam_dn5 = assign29850_e44145_d_n5;
        locals.var_inv_gam_dn6 = assign29850_e44145_d_n6;
        locals.var_inv_gam_dn7 = assign29850_e44145_d_n7;
        locals.var_inv_gam_dn8 = assign29850_e44145_d_n8;
        locals.var_inv_gam_dn9 = assign29850_e44145_d_n9;
        locals.var_inv_gam_dn10 = assign29850_e44145_d_n10;
        locals.var_inv_gam_dn11 = assign29850_e44145_d_n11;
        locals.var_inv_gam_rv = 0.0;

        let (assign29860_e44153, assign29860_e44153_d_n3, assign29860_e44153_d_n4, assign29860_e44153_d_n5, assign29860_e44153_d_n6, assign29860_e44153_d_n7, assign29860_e44153_d_n8, assign29860_e44153_d_n9, assign29860_e44153_d_n10, assign29860_e44153_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29860_e44151: f64 = (locals.var_gam * locals.var_gam);
        (assign29860_e44151, ((locals.var_gam_dn3 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn3)), ((locals.var_gam_dn4 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn4)), ((locals.var_gam_dn5 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn5)), ((locals.var_gam_dn6 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn6)), ((locals.var_gam_dn7 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn7)), ((locals.var_gam_dn8 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn8)), ((locals.var_gam_dn9 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn9)), ((locals.var_gam_dn10 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn10)), ((locals.var_gam_dn11 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn11)),)
    } else {
        (locals.var_gam2, locals.var_gam2_dn3, locals.var_gam2_dn4, locals.var_gam2_dn5, locals.var_gam2_dn6, locals.var_gam2_dn7, locals.var_gam2_dn8, locals.var_gam2_dn9, locals.var_gam2_dn10, locals.var_gam2_dn11,)
    }
};
        locals.var_gam2 = assign29860_e44153;
        locals.var_gam2_dn3 = assign29860_e44153_d_n3;
        locals.var_gam2_dn4 = assign29860_e44153_d_n4;
        locals.var_gam2_dn5 = assign29860_e44153_d_n5;
        locals.var_gam2_dn6 = assign29860_e44153_d_n6;
        locals.var_gam2_dn7 = assign29860_e44153_d_n7;
        locals.var_gam2_dn8 = assign29860_e44153_d_n8;
        locals.var_gam2_dn9 = assign29860_e44153_d_n9;
        locals.var_gam2_dn10 = assign29860_e44153_d_n10;
        locals.var_gam2_dn11 = assign29860_e44153_d_n11;
        locals.var_gam2_rv = 0.0;

        let (assign29870_e44161, assign29870_e44161_d_n3, assign29870_e44161_d_n4, assign29870_e44161_d_n5, assign29870_e44161_d_n6, assign29870_e44161_d_n7, assign29870_e44161_d_n8, assign29870_e44161_d_n9, assign29870_e44161_d_n10, assign29870_e44161_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29870_e44159: f64 = (1.0 / locals.var_gam2);
        (assign29870_e44159, (-(locals.var_gam2_dn3 / (locals.var_gam2 * locals.var_gam2))), (-(locals.var_gam2_dn4 / (locals.var_gam2 * locals.var_gam2))), (-(locals.var_gam2_dn5 / (locals.var_gam2 * locals.var_gam2))), (-(locals.var_gam2_dn6 / (locals.var_gam2 * locals.var_gam2))), (-(locals.var_gam2_dn7 / (locals.var_gam2 * locals.var_gam2))), (-(locals.var_gam2_dn8 / (locals.var_gam2 * locals.var_gam2))), (-(locals.var_gam2_dn9 / (locals.var_gam2 * locals.var_gam2))), (-(locals.var_gam2_dn10 / (locals.var_gam2 * locals.var_gam2))), (-(locals.var_gam2_dn11 / (locals.var_gam2 * locals.var_gam2))),)
    } else {
        (locals.var_inv_gam2, locals.var_inv_gam2_dn3, locals.var_inv_gam2_dn4, locals.var_inv_gam2_dn5, locals.var_inv_gam2_dn6, locals.var_inv_gam2_dn7, locals.var_inv_gam2_dn8, locals.var_inv_gam2_dn9, locals.var_inv_gam2_dn10, locals.var_inv_gam2_dn11,)
    }
};
        locals.var_inv_gam2 = assign29870_e44161;
        locals.var_inv_gam2_dn3 = assign29870_e44161_d_n3;
        locals.var_inv_gam2_dn4 = assign29870_e44161_d_n4;
        locals.var_inv_gam2_dn5 = assign29870_e44161_d_n5;
        locals.var_inv_gam2_dn6 = assign29870_e44161_d_n6;
        locals.var_inv_gam2_dn7 = assign29870_e44161_d_n7;
        locals.var_inv_gam2_dn8 = assign29870_e44161_d_n8;
        locals.var_inv_gam2_dn9 = assign29870_e44161_d_n9;
        locals.var_inv_gam2_dn10 = assign29870_e44161_d_n10;
        locals.var_inv_gam2_dn11 = assign29870_e44161_d_n11;
        locals.var_inv_gam2_rv = 0.0;

        let (assign29880_e44167, assign29880_e44167_d_n3, assign29880_e44167_d_n4, assign29880_e44167_d_n5, assign29880_e44167_d_n6, assign29880_e44167_d_n7, assign29880_e44167_d_n8, assign29880_e44167_d_n9, assign29880_e44167_d_n10, assign29880_e44167_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        (locals.var_phibagbcp2, locals.var_phibagbcp2_dn3, locals.var_phibagbcp2_dn4, locals.var_phibagbcp2_dn5, locals.var_phibagbcp2_dn6, locals.var_phibagbcp2_dn7, locals.var_phibagbcp2_dn8, locals.var_phibagbcp2_dn9, locals.var_phibagbcp2_dn10, locals.var_phibagbcp2_dn11,)
    } else {
        (locals.var_phib, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11,)
    }
};
        locals.var_phib = assign29880_e44167;
        locals.var_phib_dn3 = assign29880_e44167_d_n3;
        locals.var_phib_dn4 = assign29880_e44167_d_n4;
        locals.var_phib_dn5 = assign29880_e44167_d_n5;
        locals.var_phib_dn6 = assign29880_e44167_d_n6;
        locals.var_phib_dn7 = assign29880_e44167_d_n7;
        locals.var_phib_dn8 = assign29880_e44167_d_n8;
        locals.var_phib_dn9 = assign29880_e44167_d_n9;
        locals.var_phib_dn10 = assign29880_e44167_d_n10;
        locals.var_phib_dn11 = assign29880_e44167_d_n11;
        locals.var_phib_rv = 0.0;

        let (assign29890_e44173, assign29890_e44173_d_n3, assign29890_e44173_d_n4, assign29890_e44173_d_n5, assign29890_e44173_d_n6, assign29890_e44173_d_n7, assign29890_e44173_d_n8, assign29890_e44173_d_n9, assign29890_e44173_d_n10, assign29890_e44173_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        (p.p141, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ndep_i, locals.var_ndep_i_dn3, locals.var_ndep_i_dn4, locals.var_ndep_i_dn5, locals.var_ndep_i_dn6, locals.var_ndep_i_dn7, locals.var_ndep_i_dn8, locals.var_ndep_i_dn9, locals.var_ndep_i_dn10, locals.var_ndep_i_dn11,)
    }
};
        locals.var_ndep_i = assign29890_e44173;
        locals.var_ndep_i_dn3 = assign29890_e44173_d_n3;
        locals.var_ndep_i_dn4 = assign29890_e44173_d_n4;
        locals.var_ndep_i_dn5 = assign29890_e44173_d_n5;
        locals.var_ndep_i_dn6 = assign29890_e44173_d_n6;
        locals.var_ndep_i_dn7 = assign29890_e44173_d_n7;
        locals.var_ndep_i_dn8 = assign29890_e44173_d_n8;
        locals.var_ndep_i_dn9 = assign29890_e44173_d_n9;
        locals.var_ndep_i_dn10 = assign29890_e44173_d_n10;
        locals.var_ndep_i_dn11 = assign29890_e44173_d_n11;
        locals.var_ndep_i_rv = 0.0;

        let (assign29900_e44181, assign29900_e44181_d_n3, assign29900_e44181_d_n4, assign29900_e44181_d_n5, assign29900_e44181_d_n6, assign29900_e44181_d_n7, assign29900_e44181_d_n8, assign29900_e44181_d_n9, assign29900_e44181_d_n10, assign29900_e44181_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29900_e44179: f64 = (locals.var_gam / locals.var_rt);
        (assign29900_e44179, (locals.var_gam_dn3 / locals.var_rt), (locals.var_gam_dn4 / locals.var_rt), (locals.var_gam_dn5 / locals.var_rt), (locals.var_gam_dn6 / locals.var_rt), (locals.var_gam_dn7 / locals.var_rt), (locals.var_gam_dn8 / locals.var_rt), (locals.var_gam_dn9 / locals.var_rt), (locals.var_gam_dn10 / locals.var_rt), (locals.var_gam_dn11 / locals.var_rt),)
    } else {
        (locals.var_gam_sb, locals.var_gam_sb_dn3, locals.var_gam_sb_dn4, locals.var_gam_sb_dn5, locals.var_gam_sb_dn6, locals.var_gam_sb_dn7, locals.var_gam_sb_dn8, locals.var_gam_sb_dn9, locals.var_gam_sb_dn10, locals.var_gam_sb_dn11,)
    }
};
        locals.var_gam_sb = assign29900_e44181;
        locals.var_gam_sb_dn3 = assign29900_e44181_d_n3;
        locals.var_gam_sb_dn4 = assign29900_e44181_d_n4;
        locals.var_gam_sb_dn5 = assign29900_e44181_d_n5;
        locals.var_gam_sb_dn6 = assign29900_e44181_d_n6;
        locals.var_gam_sb_dn7 = assign29900_e44181_d_n7;
        locals.var_gam_sb_dn8 = assign29900_e44181_d_n8;
        locals.var_gam_sb_dn9 = assign29900_e44181_d_n9;
        locals.var_gam_sb_dn10 = assign29900_e44181_d_n10;
        locals.var_gam_sb_dn11 = assign29900_e44181_d_n11;
        locals.var_gam_sb_rv = 0.0;

        let (assign29910_e44191, assign29910_e44191_d_n3, assign29910_e44191_d_n4, assign29910_e44191_d_n5, assign29910_e44191_d_n6, assign29910_e44191_d_n7, assign29910_e44191_d_n8, assign29910_e44191_d_n9, assign29910_e44191_d_n10, assign29910_e44191_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29910_e44188: f64 = (locals.var_gam_sb * 0.7071067811865475);
        let assign29910_e44189: f64 = (1.0 + assign29910_e44188);
        (assign29910_e44189, (locals.var_gam_sb_dn3 * 0.7071067811865475), (locals.var_gam_sb_dn4 * 0.7071067811865475), (locals.var_gam_sb_dn5 * 0.7071067811865475), (locals.var_gam_sb_dn6 * 0.7071067811865475), (locals.var_gam_sb_dn7 * 0.7071067811865475), (locals.var_gam_sb_dn8 * 0.7071067811865475), (locals.var_gam_sb_dn9 * 0.7071067811865475), (locals.var_gam_sb_dn10 * 0.7071067811865475), (locals.var_gam_sb_dn11 * 0.7071067811865475),)
    } else {
        (locals.var_x1_sb, locals.var_x1_sb_dn3, locals.var_x1_sb_dn4, locals.var_x1_sb_dn5, locals.var_x1_sb_dn6, locals.var_x1_sb_dn7, locals.var_x1_sb_dn8, locals.var_x1_sb_dn9, locals.var_x1_sb_dn10, locals.var_x1_sb_dn11,)
    }
};
        locals.var_x1_sb = assign29910_e44191;
        locals.var_x1_sb_dn3 = assign29910_e44191_d_n3;
        locals.var_x1_sb_dn4 = assign29910_e44191_d_n4;
        locals.var_x1_sb_dn5 = assign29910_e44191_d_n5;
        locals.var_x1_sb_dn6 = assign29910_e44191_d_n6;
        locals.var_x1_sb_dn7 = assign29910_e44191_d_n7;
        locals.var_x1_sb_dn8 = assign29910_e44191_d_n8;
        locals.var_x1_sb_dn9 = assign29910_e44191_d_n9;
        locals.var_x1_sb_dn10 = assign29910_e44191_d_n10;
        locals.var_x1_sb_dn11 = assign29910_e44191_d_n11;
        locals.var_x1_sb_rv = 0.0;

        let (assign29920_e44199, assign29920_e44199_d_n3, assign29920_e44199_d_n4, assign29920_e44199_d_n5, assign29920_e44199_d_n6, assign29920_e44199_d_n7, assign29920_e44199_d_n8, assign29920_e44199_d_n9, assign29920_e44199_d_n10, assign29920_e44199_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29920_e44197: f64 = (1e-7 * locals.var_x1_sb);
        (assign29920_e44197, (1e-7 * locals.var_x1_sb_dn3), (1e-7 * locals.var_x1_sb_dn4), (1e-7 * locals.var_x1_sb_dn5), (1e-7 * locals.var_x1_sb_dn6), (1e-7 * locals.var_x1_sb_dn7), (1e-7 * locals.var_x1_sb_dn8), (1e-7 * locals.var_x1_sb_dn9), (1e-7 * locals.var_x1_sb_dn10), (1e-7 * locals.var_x1_sb_dn11),)
    } else {
        (locals.var_limit_sb, locals.var_limit_sb_dn3, locals.var_limit_sb_dn4, locals.var_limit_sb_dn5, locals.var_limit_sb_dn6, locals.var_limit_sb_dn7, locals.var_limit_sb_dn8, locals.var_limit_sb_dn9, locals.var_limit_sb_dn10, locals.var_limit_sb_dn11,)
    }
};
        locals.var_limit_sb = assign29920_e44199;
        locals.var_limit_sb_dn3 = assign29920_e44199_d_n3;
        locals.var_limit_sb_dn4 = assign29920_e44199_d_n4;
        locals.var_limit_sb_dn5 = assign29920_e44199_d_n5;
        locals.var_limit_sb_dn6 = assign29920_e44199_d_n6;
        locals.var_limit_sb_dn7 = assign29920_e44199_d_n7;
        locals.var_limit_sb_dn8 = assign29920_e44199_d_n8;
        locals.var_limit_sb_dn9 = assign29920_e44199_d_n9;
        locals.var_limit_sb_dn10 = assign29920_e44199_d_n10;
        locals.var_limit_sb_dn11 = assign29920_e44199_d_n11;
        locals.var_limit_sb_rv = 0.0;

        let (assign29930_e44207,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29930_e44205: f64 = (5.0 / 4.0);
        (assign29930_e44205,)
    } else {
        (locals.var_x1_csb,)
    }
};
        locals.var_x1_csb = assign29930_e44207;
        locals.var_x1_csb_rv = 0.0;

        let (assign29940_e44215, assign29940_e44215_d_n3, assign29940_e44215_d_n4, assign29940_e44215_d_n5, assign29940_e44215_d_n6, assign29940_e44215_d_n7, assign29940_e44215_d_n8, assign29940_e44215_d_n9, assign29940_e44215_d_n10, assign29940_e44215_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29940_e44213: f64 = (1.0 / locals.var_gam_sb);
        (assign29940_e44213, (-(locals.var_gam_sb_dn3 / (locals.var_gam_sb * locals.var_gam_sb))), (-(locals.var_gam_sb_dn4 / (locals.var_gam_sb * locals.var_gam_sb))), (-(locals.var_gam_sb_dn5 / (locals.var_gam_sb * locals.var_gam_sb))), (-(locals.var_gam_sb_dn6 / (locals.var_gam_sb * locals.var_gam_sb))), (-(locals.var_gam_sb_dn7 / (locals.var_gam_sb * locals.var_gam_sb))), (-(locals.var_gam_sb_dn8 / (locals.var_gam_sb * locals.var_gam_sb))), (-(locals.var_gam_sb_dn9 / (locals.var_gam_sb * locals.var_gam_sb))), (-(locals.var_gam_sb_dn10 / (locals.var_gam_sb * locals.var_gam_sb))), (-(locals.var_gam_sb_dn11 / (locals.var_gam_sb * locals.var_gam_sb))),)
    } else {
        (locals.var_inv_xi_sb, locals.var_inv_xi_sb_dn3, locals.var_inv_xi_sb_dn4, locals.var_inv_xi_sb_dn5, locals.var_inv_xi_sb_dn6, locals.var_inv_xi_sb_dn7, locals.var_inv_xi_sb_dn8, locals.var_inv_xi_sb_dn9, locals.var_inv_xi_sb_dn10, locals.var_inv_xi_sb_dn11,)
    }
};
        locals.var_inv_xi_sb = assign29940_e44215;
        locals.var_inv_xi_sb_dn3 = assign29940_e44215_d_n3;
        locals.var_inv_xi_sb_dn4 = assign29940_e44215_d_n4;
        locals.var_inv_xi_sb_dn5 = assign29940_e44215_d_n5;
        locals.var_inv_xi_sb_dn6 = assign29940_e44215_d_n6;
        locals.var_inv_xi_sb_dn7 = assign29940_e44215_d_n7;
        locals.var_inv_xi_sb_dn8 = assign29940_e44215_d_n8;
        locals.var_inv_xi_sb_dn9 = assign29940_e44215_d_n9;
        locals.var_inv_xi_sb_dn10 = assign29940_e44215_d_n10;
        locals.var_inv_xi_sb_dn11 = assign29940_e44215_d_n11;
        locals.var_inv_xi_sb_rv = 0.0;

        let (assign29950_e44223, assign29950_e44223_d_n3, assign29950_e44223_d_n4, assign29950_e44223_d_n5, assign29950_e44223_d_n6, assign29950_e44223_d_n7, assign29950_e44223_d_n8, assign29950_e44223_d_n9, assign29950_e44223_d_n10, assign29950_e44223_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29950_e44221: f64 = (locals.var_gam_sb * locals.var_gam_sb);
        (assign29950_e44221, ((locals.var_gam_sb_dn3 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn3)), ((locals.var_gam_sb_dn4 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn4)), ((locals.var_gam_sb_dn5 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn5)), ((locals.var_gam_sb_dn6 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn6)), ((locals.var_gam_sb_dn7 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn7)), ((locals.var_gam_sb_dn8 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn8)), ((locals.var_gam_sb_dn9 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn9)), ((locals.var_gam_sb_dn10 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn10)), ((locals.var_gam_sb_dn11 * locals.var_gam_sb) + (locals.var_gam_sb * locals.var_gam_sb_dn11)),)
    } else {
        (locals.var_gam_sb2, locals.var_gam_sb2_dn3, locals.var_gam_sb2_dn4, locals.var_gam_sb2_dn5, locals.var_gam_sb2_dn6, locals.var_gam_sb2_dn7, locals.var_gam_sb2_dn8, locals.var_gam_sb2_dn9, locals.var_gam_sb2_dn10, locals.var_gam_sb2_dn11,)
    }
};
        locals.var_gam_sb2 = assign29950_e44223;
        locals.var_gam_sb2_dn3 = assign29950_e44223_d_n3;
        locals.var_gam_sb2_dn4 = assign29950_e44223_d_n4;
        locals.var_gam_sb2_dn5 = assign29950_e44223_d_n5;
        locals.var_gam_sb2_dn6 = assign29950_e44223_d_n6;
        locals.var_gam_sb2_dn7 = assign29950_e44223_d_n7;
        locals.var_gam_sb2_dn8 = assign29950_e44223_d_n8;
        locals.var_gam_sb2_dn9 = assign29950_e44223_d_n9;
        locals.var_gam_sb2_dn10 = assign29950_e44223_d_n10;
        locals.var_gam_sb2_dn11 = assign29950_e44223_d_n11;
        locals.var_gam_sb2_rv = 0.0;

        let (assign29960_e44235, assign29960_e44235_d_n3, assign29960_e44235_d_n4, assign29960_e44235_d_n5, assign29960_e44235_d_n6, assign29960_e44235_d_n7, assign29960_e44235_d_n8, assign29960_e44235_d_n9, assign29960_e44235_d_n10, assign29960_e44235_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign29960_e44231: f64 = (locals.var_gam_sb * 0.7324648775608221);
        let assign29960_e44232: f64 = (locals.var_x1_csb + assign29960_e44231);
        let assign29960_e44233: f64 = (1.0 / assign29960_e44232);
        (assign29960_e44233, (-((locals.var_gam_sb_dn3 * 0.7324648775608221) / (assign29960_e44232 * assign29960_e44232))), (-((locals.var_gam_sb_dn4 * 0.7324648775608221) / (assign29960_e44232 * assign29960_e44232))), (-((locals.var_gam_sb_dn5 * 0.7324648775608221) / (assign29960_e44232 * assign29960_e44232))), (-((locals.var_gam_sb_dn6 * 0.7324648775608221) / (assign29960_e44232 * assign29960_e44232))), (-((locals.var_gam_sb_dn7 * 0.7324648775608221) / (assign29960_e44232 * assign29960_e44232))), (-((locals.var_gam_sb_dn8 * 0.7324648775608221) / (assign29960_e44232 * assign29960_e44232))), (-((locals.var_gam_sb_dn9 * 0.7324648775608221) / (assign29960_e44232 * assign29960_e44232))), (-((locals.var_gam_sb_dn10 * 0.7324648775608221) / (assign29960_e44232 * assign29960_e44232))), (-((locals.var_gam_sb_dn11 * 0.7324648775608221) / (assign29960_e44232 * assign29960_e44232))),)
    } else {
        (locals.var_inv_xg1_sb, locals.var_inv_xg1_sb_dn3, locals.var_inv_xg1_sb_dn4, locals.var_inv_xg1_sb_dn5, locals.var_inv_xg1_sb_dn6, locals.var_inv_xg1_sb_dn7, locals.var_inv_xg1_sb_dn8, locals.var_inv_xg1_sb_dn9, locals.var_inv_xg1_sb_dn10, locals.var_inv_xg1_sb_dn11,)
    }
};
        locals.var_inv_xg1_sb = assign29960_e44235;
        locals.var_inv_xg1_sb_dn3 = assign29960_e44235_d_n3;
        locals.var_inv_xg1_sb_dn4 = assign29960_e44235_d_n4;
        locals.var_inv_xg1_sb_dn5 = assign29960_e44235_d_n5;
        locals.var_inv_xg1_sb_dn6 = assign29960_e44235_d_n6;
        locals.var_inv_xg1_sb_dn7 = assign29960_e44235_d_n7;
        locals.var_inv_xg1_sb_dn8 = assign29960_e44235_d_n8;
        locals.var_inv_xg1_sb_dn9 = assign29960_e44235_d_n9;
        locals.var_inv_xg1_sb_dn10 = assign29960_e44235_d_n10;
        locals.var_inv_xg1_sb_dn11 = assign29960_e44235_d_n11;
        locals.var_inv_xg1_sb_rv = 0.0;

        let assign29970_e44237: f64 = (locals.var_vgfbb).abs();
        let assign29970_e44239: f64 = if assign29970_e44237 <= locals.var_limit_sb { 1.0 } else { 0.0 };
        locals.var_guard614 = assign29970_e44239;
        locals.var_guard614_rv = 0.0;

        let (assign29980_e44266, assign29980_e44266_d_n3, assign29980_e44266_d_n4, assign29980_e44266_d_n5, assign29980_e44266_d_n6, assign29980_e44266_d_n7, assign29980_e44266_d_n8, assign29980_e44266_d_n9, assign29980_e44266_d_n10, assign29980_e44266_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 != 0.0)) {
        let assign29980_e44246: f64 = (-locals.var_vgfbb);
        let assign29980_e44248: f64 = (assign29980_e44246 * locals.var_inv_xi_sb);
        let assign29980_e44252: f64 = (-locals.var_vgfbb);
        let assign29980_e44255: f64 = (2.0_f64).sqrt();
        let assign29980_e44256: f64 = (6.0 * assign29980_e44255);
        let assign29980_e44258: f64 = (assign29980_e44256 * locals.var_x1_sb);
        let assign29980_e44260: f64 = (assign29980_e44258 * locals.var_x1_sb);
        let assign29980_e44261: f64 = (assign29980_e44252 / assign29980_e44260);
        let assign29980_e44262: f64 = (locals.var_gam_sb * assign29980_e44261);
        let assign29980_e44263: f64 = (1.0 + assign29980_e44262);
        let assign29980_e44264: f64 = (assign29980_e44248 * assign29980_e44263);
        (assign29980_e44264, (((((-locals.var_vgfbb_dn3) * locals.var_inv_xi_sb) + (assign29980_e44246 * locals.var_inv_xi_sb_dn3)) * assign29980_e44263) + (assign29980_e44248 * ((locals.var_gam_sb_dn3 * assign29980_e44261) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn3) * assign29980_e44260) - (assign29980_e44252 * (((assign29980_e44256 * locals.var_x1_sb_dn3) * locals.var_x1_sb) + (assign29980_e44258 * locals.var_x1_sb_dn3)))) / (assign29980_e44260 * assign29980_e44260)))))), (((((-locals.var_vgfbb_dn4) * locals.var_inv_xi_sb) + (assign29980_e44246 * locals.var_inv_xi_sb_dn4)) * assign29980_e44263) + (assign29980_e44248 * ((locals.var_gam_sb_dn4 * assign29980_e44261) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn4) * assign29980_e44260) - (assign29980_e44252 * (((assign29980_e44256 * locals.var_x1_sb_dn4) * locals.var_x1_sb) + (assign29980_e44258 * locals.var_x1_sb_dn4)))) / (assign29980_e44260 * assign29980_e44260)))))), (((((-locals.var_vgfbb_dn5) * locals.var_inv_xi_sb) + (assign29980_e44246 * locals.var_inv_xi_sb_dn5)) * assign29980_e44263) + (assign29980_e44248 * ((locals.var_gam_sb_dn5 * assign29980_e44261) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn5) * assign29980_e44260) - (assign29980_e44252 * (((assign29980_e44256 * locals.var_x1_sb_dn5) * locals.var_x1_sb) + (assign29980_e44258 * locals.var_x1_sb_dn5)))) / (assign29980_e44260 * assign29980_e44260)))))), (((((-locals.var_vgfbb_dn6) * locals.var_inv_xi_sb) + (assign29980_e44246 * locals.var_inv_xi_sb_dn6)) * assign29980_e44263) + (assign29980_e44248 * ((locals.var_gam_sb_dn6 * assign29980_e44261) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn6) * assign29980_e44260) - (assign29980_e44252 * (((assign29980_e44256 * locals.var_x1_sb_dn6) * locals.var_x1_sb) + (assign29980_e44258 * locals.var_x1_sb_dn6)))) / (assign29980_e44260 * assign29980_e44260)))))), (((((-locals.var_vgfbb_dn7) * locals.var_inv_xi_sb) + (assign29980_e44246 * locals.var_inv_xi_sb_dn7)) * assign29980_e44263) + (assign29980_e44248 * ((locals.var_gam_sb_dn7 * assign29980_e44261) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn7) * assign29980_e44260) - (assign29980_e44252 * (((assign29980_e44256 * locals.var_x1_sb_dn7) * locals.var_x1_sb) + (assign29980_e44258 * locals.var_x1_sb_dn7)))) / (assign29980_e44260 * assign29980_e44260)))))), (((((-locals.var_vgfbb_dn8) * locals.var_inv_xi_sb) + (assign29980_e44246 * locals.var_inv_xi_sb_dn8)) * assign29980_e44263) + (assign29980_e44248 * ((locals.var_gam_sb_dn8 * assign29980_e44261) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn8) * assign29980_e44260) - (assign29980_e44252 * (((assign29980_e44256 * locals.var_x1_sb_dn8) * locals.var_x1_sb) + (assign29980_e44258 * locals.var_x1_sb_dn8)))) / (assign29980_e44260 * assign29980_e44260)))))), (((((-locals.var_vgfbb_dn9) * locals.var_inv_xi_sb) + (assign29980_e44246 * locals.var_inv_xi_sb_dn9)) * assign29980_e44263) + (assign29980_e44248 * ((locals.var_gam_sb_dn9 * assign29980_e44261) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn9) * assign29980_e44260) - (assign29980_e44252 * (((assign29980_e44256 * locals.var_x1_sb_dn9) * locals.var_x1_sb) + (assign29980_e44258 * locals.var_x1_sb_dn9)))) / (assign29980_e44260 * assign29980_e44260)))))), (((((-locals.var_vgfbb_dn10) * locals.var_inv_xi_sb) + (assign29980_e44246 * locals.var_inv_xi_sb_dn10)) * assign29980_e44263) + (assign29980_e44248 * ((locals.var_gam_sb_dn10 * assign29980_e44261) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn10) * assign29980_e44260) - (assign29980_e44252 * (((assign29980_e44256 * locals.var_x1_sb_dn10) * locals.var_x1_sb) + (assign29980_e44258 * locals.var_x1_sb_dn10)))) / (assign29980_e44260 * assign29980_e44260)))))), (((((-locals.var_vgfbb_dn11) * locals.var_inv_xi_sb) + (assign29980_e44246 * locals.var_inv_xi_sb_dn11)) * assign29980_e44263) + (assign29980_e44248 * ((locals.var_gam_sb_dn11 * assign29980_e44261) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn11) * assign29980_e44260) - (assign29980_e44252 * (((assign29980_e44256 * locals.var_x1_sb_dn11) * locals.var_x1_sb) + (assign29980_e44258 * locals.var_x1_sb_dn11)))) / (assign29980_e44260 * assign29980_e44260)))))),)
    } else {
        (locals.var_pd_sb, locals.var_pd_sb_dn3, locals.var_pd_sb_dn4, locals.var_pd_sb_dn5, locals.var_pd_sb_dn6, locals.var_pd_sb_dn7, locals.var_pd_sb_dn8, locals.var_pd_sb_dn9, locals.var_pd_sb_dn10, locals.var_pd_sb_dn11,)
    }
};
        locals.var_pd_sb = assign29980_e44266;
        locals.var_pd_sb_dn3 = assign29980_e44266_d_n3;
        locals.var_pd_sb_dn4 = assign29980_e44266_d_n4;
        locals.var_pd_sb_dn5 = assign29980_e44266_d_n5;
        locals.var_pd_sb_dn6 = assign29980_e44266_d_n6;
        locals.var_pd_sb_dn7 = assign29980_e44266_d_n7;
        locals.var_pd_sb_dn8 = assign29980_e44266_d_n8;
        locals.var_pd_sb_dn9 = assign29980_e44266_d_n9;
        locals.var_pd_sb_dn10 = assign29980_e44266_d_n10;
        locals.var_pd_sb_dn11 = assign29980_e44266_d_n11;
        locals.var_pd_sb_rv = 0.0;

        let assign29990_e44269: f64 = (-locals.var_limit_sb);
        let assign29990_e44270: f64 = if locals.var_vgfbb < assign29990_e44269 { 1.0 } else { 0.0 };
        locals.var_guard615 = assign29990_e44270;
        locals.var_guard615_rv = 0.0;

        let (assign30000_e44282, assign30000_e44282_d_n3, assign30000_e44282_d_n4, assign30000_e44282_d_n5, assign30000_e44282_d_n6, assign30000_e44282_d_n7, assign30000_e44282_d_n8, assign30000_e44282_d_n9, assign30000_e44282_d_n10, assign30000_e44282_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30000_e44280: f64 = (-locals.var_vgfbb);
        (assign30000_e44280, (-locals.var_vgfbb_dn3), (-locals.var_vgfbb_dn4), (-locals.var_vgfbb_dn5), (-locals.var_vgfbb_dn6), (-locals.var_vgfbb_dn7), (-locals.var_vgfbb_dn8), (-locals.var_vgfbb_dn9), (-locals.var_vgfbb_dn10), (-locals.var_vgfbb_dn11),)
    } else {
        (locals.var_pd_yg, locals.var_pd_yg_dn3, locals.var_pd_yg_dn4, locals.var_pd_yg_dn5, locals.var_pd_yg_dn6, locals.var_pd_yg_dn7, locals.var_pd_yg_dn8, locals.var_pd_yg_dn9, locals.var_pd_yg_dn10, locals.var_pd_yg_dn11,)
    }
};
        locals.var_pd_yg = assign30000_e44282;
        locals.var_pd_yg_dn3 = assign30000_e44282_d_n3;
        locals.var_pd_yg_dn4 = assign30000_e44282_d_n4;
        locals.var_pd_yg_dn5 = assign30000_e44282_d_n5;
        locals.var_pd_yg_dn6 = assign30000_e44282_d_n6;
        locals.var_pd_yg_dn7 = assign30000_e44282_d_n7;
        locals.var_pd_yg_dn8 = assign30000_e44282_d_n8;
        locals.var_pd_yg_dn9 = assign30000_e44282_d_n9;
        locals.var_pd_yg_dn10 = assign30000_e44282_d_n10;
        locals.var_pd_yg_dn11 = assign30000_e44282_d_n11;
        locals.var_pd_yg_rv = 0.0;

        let (assign30010_e44297, assign30010_e44297_d_n3, assign30010_e44297_d_n4, assign30010_e44297_d_n5, assign30010_e44297_d_n6, assign30010_e44297_d_n7, assign30010_e44297_d_n8, assign30010_e44297_d_n9, assign30010_e44297_d_n10, assign30010_e44297_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30010_e44293: f64 = (locals.var_x1_csb * locals.var_pd_yg);
        let assign30010_e44295: f64 = (assign30010_e44293 * locals.var_inv_xi_sb);
        (assign30010_e44295, (((locals.var_x1_csb * locals.var_pd_yg_dn3) * locals.var_inv_xi_sb) + (assign30010_e44293 * locals.var_inv_xi_sb_dn3)), (((locals.var_x1_csb * locals.var_pd_yg_dn4) * locals.var_inv_xi_sb) + (assign30010_e44293 * locals.var_inv_xi_sb_dn4)), (((locals.var_x1_csb * locals.var_pd_yg_dn5) * locals.var_inv_xi_sb) + (assign30010_e44293 * locals.var_inv_xi_sb_dn5)), (((locals.var_x1_csb * locals.var_pd_yg_dn6) * locals.var_inv_xi_sb) + (assign30010_e44293 * locals.var_inv_xi_sb_dn6)), (((locals.var_x1_csb * locals.var_pd_yg_dn7) * locals.var_inv_xi_sb) + (assign30010_e44293 * locals.var_inv_xi_sb_dn7)), (((locals.var_x1_csb * locals.var_pd_yg_dn8) * locals.var_inv_xi_sb) + (assign30010_e44293 * locals.var_inv_xi_sb_dn8)), (((locals.var_x1_csb * locals.var_pd_yg_dn9) * locals.var_inv_xi_sb) + (assign30010_e44293 * locals.var_inv_xi_sb_dn9)), (((locals.var_x1_csb * locals.var_pd_yg_dn10) * locals.var_inv_xi_sb) + (assign30010_e44293 * locals.var_inv_xi_sb_dn10)), (((locals.var_x1_csb * locals.var_pd_yg_dn11) * locals.var_inv_xi_sb) + (assign30010_e44293 * locals.var_inv_xi_sb_dn11)),)
    } else {
        (locals.var_pd_z, locals.var_pd_z_dn3, locals.var_pd_z_dn4, locals.var_pd_z_dn5, locals.var_pd_z_dn6, locals.var_pd_z_dn7, locals.var_pd_z_dn8, locals.var_pd_z_dn9, locals.var_pd_z_dn10, locals.var_pd_z_dn11,)
    }
};
        locals.var_pd_z = assign30010_e44297;
        locals.var_pd_z_dn3 = assign30010_e44297_d_n3;
        locals.var_pd_z_dn4 = assign30010_e44297_d_n4;
        locals.var_pd_z_dn5 = assign30010_e44297_d_n5;
        locals.var_pd_z_dn6 = assign30010_e44297_d_n6;
        locals.var_pd_z_dn7 = assign30010_e44297_d_n7;
        locals.var_pd_z_dn8 = assign30010_e44297_d_n8;
        locals.var_pd_z_dn9 = assign30010_e44297_d_n9;
        locals.var_pd_z_dn10 = assign30010_e44297_d_n10;
        locals.var_pd_z_dn11 = assign30010_e44297_d_n11;
        locals.var_pd_z_rv = 0.0;

        let (assign30020_e44323, assign30020_e44323_d_n3, assign30020_e44323_d_n4, assign30020_e44323_d_n5, assign30020_e44323_d_n6, assign30020_e44323_d_n7, assign30020_e44323_d_n8, assign30020_e44323_d_n9, assign30020_e44323_d_n10, assign30020_e44323_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30020_e44309: f64 = (locals.var_pd_z + 10.0);
        let assign30020_e44312: f64 = (locals.var_pd_z - 6.0);
        let assign30020_e44315: f64 = (locals.var_pd_z - 6.0);
        let assign30020_e44316: f64 = (assign30020_e44312 * assign30020_e44315);
        let assign30020_e44318: f64 = (assign30020_e44316 + 64.0);
        let assign30020_e44319: f64 = (assign30020_e44318).sqrt();
        let assign30020_e44320: f64 = (assign30020_e44309 - assign30020_e44319);
        let assign30020_e44321: f64 = (0.5 * assign30020_e44320);
        (assign30020_e44321, (0.5 * (locals.var_pd_z_dn3 - (((locals.var_pd_z_dn3 * assign30020_e44315) + (assign30020_e44312 * locals.var_pd_z_dn3)) / (2.0 * assign30020_e44319)))), (0.5 * (locals.var_pd_z_dn4 - (((locals.var_pd_z_dn4 * assign30020_e44315) + (assign30020_e44312 * locals.var_pd_z_dn4)) / (2.0 * assign30020_e44319)))), (0.5 * (locals.var_pd_z_dn5 - (((locals.var_pd_z_dn5 * assign30020_e44315) + (assign30020_e44312 * locals.var_pd_z_dn5)) / (2.0 * assign30020_e44319)))), (0.5 * (locals.var_pd_z_dn6 - (((locals.var_pd_z_dn6 * assign30020_e44315) + (assign30020_e44312 * locals.var_pd_z_dn6)) / (2.0 * assign30020_e44319)))), (0.5 * (locals.var_pd_z_dn7 - (((locals.var_pd_z_dn7 * assign30020_e44315) + (assign30020_e44312 * locals.var_pd_z_dn7)) / (2.0 * assign30020_e44319)))), (0.5 * (locals.var_pd_z_dn8 - (((locals.var_pd_z_dn8 * assign30020_e44315) + (assign30020_e44312 * locals.var_pd_z_dn8)) / (2.0 * assign30020_e44319)))), (0.5 * (locals.var_pd_z_dn9 - (((locals.var_pd_z_dn9 * assign30020_e44315) + (assign30020_e44312 * locals.var_pd_z_dn9)) / (2.0 * assign30020_e44319)))), (0.5 * (locals.var_pd_z_dn10 - (((locals.var_pd_z_dn10 * assign30020_e44315) + (assign30020_e44312 * locals.var_pd_z_dn10)) / (2.0 * assign30020_e44319)))), (0.5 * (locals.var_pd_z_dn11 - (((locals.var_pd_z_dn11 * assign30020_e44315) + (assign30020_e44312 * locals.var_pd_z_dn11)) / (2.0 * assign30020_e44319)))),)
    } else {
        (locals.var_pd_eta, locals.var_pd_eta_dn3, locals.var_pd_eta_dn4, locals.var_pd_eta_dn5, locals.var_pd_eta_dn6, locals.var_pd_eta_dn7, locals.var_pd_eta_dn8, locals.var_pd_eta_dn9, locals.var_pd_eta_dn10, locals.var_pd_eta_dn11,)
    }
};
        locals.var_pd_eta = assign30020_e44323;
        locals.var_pd_eta_dn3 = assign30020_e44323_d_n3;
        locals.var_pd_eta_dn4 = assign30020_e44323_d_n4;
        locals.var_pd_eta_dn5 = assign30020_e44323_d_n5;
        locals.var_pd_eta_dn6 = assign30020_e44323_d_n6;
        locals.var_pd_eta_dn7 = assign30020_e44323_d_n7;
        locals.var_pd_eta_dn8 = assign30020_e44323_d_n8;
        locals.var_pd_eta_dn9 = assign30020_e44323_d_n9;
        locals.var_pd_eta_dn10 = assign30020_e44323_d_n10;
        locals.var_pd_eta_dn11 = assign30020_e44323_d_n11;
        locals.var_pd_eta_rv = 0.0;

        let (assign30030_e44346, assign30030_e44346_d_n3, assign30030_e44346_d_n4, assign30030_e44346_d_n5, assign30030_e44346_d_n6, assign30030_e44346_d_n7, assign30030_e44346_d_n8, assign30030_e44346_d_n9, assign30030_e44346_d_n10, assign30030_e44346_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30030_e44334: f64 = (locals.var_pd_yg - locals.var_pd_eta);
        let assign30030_e44337: f64 = (locals.var_pd_yg - locals.var_pd_eta);
        let assign30030_e44338: f64 = (assign30030_e44334 * assign30030_e44337);
        let assign30030_e44342: f64 = (locals.var_pd_eta + 1.0);
        let assign30030_e44343: f64 = (locals.var_gam_sb2 * assign30030_e44342);
        let assign30030_e44344: f64 = (assign30030_e44338 + assign30030_e44343);
        (assign30030_e44344, ((((locals.var_pd_yg_dn3 - locals.var_pd_eta_dn3) * assign30030_e44337) + (assign30030_e44334 * (locals.var_pd_yg_dn3 - locals.var_pd_eta_dn3))) + ((locals.var_gam_sb2_dn3 * assign30030_e44342) + (locals.var_gam_sb2 * locals.var_pd_eta_dn3))), ((((locals.var_pd_yg_dn4 - locals.var_pd_eta_dn4) * assign30030_e44337) + (assign30030_e44334 * (locals.var_pd_yg_dn4 - locals.var_pd_eta_dn4))) + ((locals.var_gam_sb2_dn4 * assign30030_e44342) + (locals.var_gam_sb2 * locals.var_pd_eta_dn4))), ((((locals.var_pd_yg_dn5 - locals.var_pd_eta_dn5) * assign30030_e44337) + (assign30030_e44334 * (locals.var_pd_yg_dn5 - locals.var_pd_eta_dn5))) + ((locals.var_gam_sb2_dn5 * assign30030_e44342) + (locals.var_gam_sb2 * locals.var_pd_eta_dn5))), ((((locals.var_pd_yg_dn6 - locals.var_pd_eta_dn6) * assign30030_e44337) + (assign30030_e44334 * (locals.var_pd_yg_dn6 - locals.var_pd_eta_dn6))) + ((locals.var_gam_sb2_dn6 * assign30030_e44342) + (locals.var_gam_sb2 * locals.var_pd_eta_dn6))), ((((locals.var_pd_yg_dn7 - locals.var_pd_eta_dn7) * assign30030_e44337) + (assign30030_e44334 * (locals.var_pd_yg_dn7 - locals.var_pd_eta_dn7))) + ((locals.var_gam_sb2_dn7 * assign30030_e44342) + (locals.var_gam_sb2 * locals.var_pd_eta_dn7))), ((((locals.var_pd_yg_dn8 - locals.var_pd_eta_dn8) * assign30030_e44337) + (assign30030_e44334 * (locals.var_pd_yg_dn8 - locals.var_pd_eta_dn8))) + ((locals.var_gam_sb2_dn8 * assign30030_e44342) + (locals.var_gam_sb2 * locals.var_pd_eta_dn8))), ((((locals.var_pd_yg_dn9 - locals.var_pd_eta_dn9) * assign30030_e44337) + (assign30030_e44334 * (locals.var_pd_yg_dn9 - locals.var_pd_eta_dn9))) + ((locals.var_gam_sb2_dn9 * assign30030_e44342) + (locals.var_gam_sb2 * locals.var_pd_eta_dn9))), ((((locals.var_pd_yg_dn10 - locals.var_pd_eta_dn10) * assign30030_e44337) + (assign30030_e44334 * (locals.var_pd_yg_dn10 - locals.var_pd_eta_dn10))) + ((locals.var_gam_sb2_dn10 * assign30030_e44342) + (locals.var_gam_sb2 * locals.var_pd_eta_dn10))), ((((locals.var_pd_yg_dn11 - locals.var_pd_eta_dn11) * assign30030_e44337) + (assign30030_e44334 * (locals.var_pd_yg_dn11 - locals.var_pd_eta_dn11))) + ((locals.var_gam_sb2_dn11 * assign30030_e44342) + (locals.var_gam_sb2 * locals.var_pd_eta_dn11))),)
    } else {
        (locals.var_pd_a, locals.var_pd_a_dn3, locals.var_pd_a_dn4, locals.var_pd_a_dn5, locals.var_pd_a_dn6, locals.var_pd_a_dn7, locals.var_pd_a_dn8, locals.var_pd_a_dn9, locals.var_pd_a_dn10, locals.var_pd_a_dn11,)
    }
};
        locals.var_pd_a = assign30030_e44346;
        locals.var_pd_a_dn3 = assign30030_e44346_d_n3;
        locals.var_pd_a_dn4 = assign30030_e44346_d_n4;
        locals.var_pd_a_dn5 = assign30030_e44346_d_n5;
        locals.var_pd_a_dn6 = assign30030_e44346_d_n6;
        locals.var_pd_a_dn7 = assign30030_e44346_d_n7;
        locals.var_pd_a_dn8 = assign30030_e44346_d_n8;
        locals.var_pd_a_dn9 = assign30030_e44346_d_n9;
        locals.var_pd_a_dn10 = assign30030_e44346_d_n10;
        locals.var_pd_a_dn11 = assign30030_e44346_d_n11;
        locals.var_pd_a_rv = 0.0;

        let (assign30040_e44363, assign30040_e44363_d_n3, assign30040_e44363_d_n4, assign30040_e44363_d_n5, assign30040_e44363_d_n6, assign30040_e44363_d_n7, assign30040_e44363_d_n8, assign30040_e44363_d_n9, assign30040_e44363_d_n10, assign30040_e44363_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30040_e44358: f64 = (locals.var_pd_yg - locals.var_pd_eta);
        let assign30040_e44359: f64 = (2.0 * assign30040_e44358);
        let assign30040_e44361: f64 = (assign30040_e44359 - locals.var_gam_sb2);
        (assign30040_e44361, ((2.0 * (locals.var_pd_yg_dn3 - locals.var_pd_eta_dn3)) - locals.var_gam_sb2_dn3), ((2.0 * (locals.var_pd_yg_dn4 - locals.var_pd_eta_dn4)) - locals.var_gam_sb2_dn4), ((2.0 * (locals.var_pd_yg_dn5 - locals.var_pd_eta_dn5)) - locals.var_gam_sb2_dn5), ((2.0 * (locals.var_pd_yg_dn6 - locals.var_pd_eta_dn6)) - locals.var_gam_sb2_dn6), ((2.0 * (locals.var_pd_yg_dn7 - locals.var_pd_eta_dn7)) - locals.var_gam_sb2_dn7), ((2.0 * (locals.var_pd_yg_dn8 - locals.var_pd_eta_dn8)) - locals.var_gam_sb2_dn8), ((2.0 * (locals.var_pd_yg_dn9 - locals.var_pd_eta_dn9)) - locals.var_gam_sb2_dn9), ((2.0 * (locals.var_pd_yg_dn10 - locals.var_pd_eta_dn10)) - locals.var_gam_sb2_dn10), ((2.0 * (locals.var_pd_yg_dn11 - locals.var_pd_eta_dn11)) - locals.var_gam_sb2_dn11),)
    } else {
        (locals.var_pd_c, locals.var_pd_c_dn3, locals.var_pd_c_dn4, locals.var_pd_c_dn5, locals.var_pd_c_dn6, locals.var_pd_c_dn7, locals.var_pd_c_dn8, locals.var_pd_c_dn9, locals.var_pd_c_dn10, locals.var_pd_c_dn11,)
    }
};
        locals.var_pd_c = assign30040_e44363;
        locals.var_pd_c_dn3 = assign30040_e44363_d_n3;
        locals.var_pd_c_dn4 = assign30040_e44363_d_n4;
        locals.var_pd_c_dn5 = assign30040_e44363_d_n5;
        locals.var_pd_c_dn6 = assign30040_e44363_d_n6;
        locals.var_pd_c_dn7 = assign30040_e44363_d_n7;
        locals.var_pd_c_dn8 = assign30040_e44363_d_n8;
        locals.var_pd_c_dn9 = assign30040_e44363_d_n9;
        locals.var_pd_c_dn10 = assign30040_e44363_d_n10;
        locals.var_pd_c_dn11 = assign30040_e44363_d_n11;
        locals.var_pd_c_rv = 0.0;

        let (assign30050_e44381, assign30050_e44381_d_n3, assign30050_e44381_d_n4, assign30050_e44381_d_n5, assign30050_e44381_d_n6, assign30050_e44381_d_n7, assign30050_e44381_d_n8, assign30050_e44381_d_n9, assign30050_e44381_d_n10, assign30050_e44381_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30050_e44374: f64 = (locals.var_pd_a / locals.var_gam_sb2);
        let assign30050_e44376: f64 = (assign30050_e44374).max(1e-38);
        let assign30050_e44377: f64 = (assign30050_e44376).ln();
        let assign30050_e44379: f64 = (assign30050_e44377 - locals.var_pd_eta);
        (assign30050_e44379, ((if assign30050_e44374 >= 1e-38 { (((locals.var_pd_a_dn3 * locals.var_gam_sb2) - (locals.var_pd_a * locals.var_gam_sb2_dn3)) / (locals.var_gam_sb2 * locals.var_gam_sb2)) } else { 0.0 } / assign30050_e44376) - locals.var_pd_eta_dn3), ((if assign30050_e44374 >= 1e-38 { (((locals.var_pd_a_dn4 * locals.var_gam_sb2) - (locals.var_pd_a * locals.var_gam_sb2_dn4)) / (locals.var_gam_sb2 * locals.var_gam_sb2)) } else { 0.0 } / assign30050_e44376) - locals.var_pd_eta_dn4), ((if assign30050_e44374 >= 1e-38 { (((locals.var_pd_a_dn5 * locals.var_gam_sb2) - (locals.var_pd_a * locals.var_gam_sb2_dn5)) / (locals.var_gam_sb2 * locals.var_gam_sb2)) } else { 0.0 } / assign30050_e44376) - locals.var_pd_eta_dn5), ((if assign30050_e44374 >= 1e-38 { (((locals.var_pd_a_dn6 * locals.var_gam_sb2) - (locals.var_pd_a * locals.var_gam_sb2_dn6)) / (locals.var_gam_sb2 * locals.var_gam_sb2)) } else { 0.0 } / assign30050_e44376) - locals.var_pd_eta_dn6), ((if assign30050_e44374 >= 1e-38 { (((locals.var_pd_a_dn7 * locals.var_gam_sb2) - (locals.var_pd_a * locals.var_gam_sb2_dn7)) / (locals.var_gam_sb2 * locals.var_gam_sb2)) } else { 0.0 } / assign30050_e44376) - locals.var_pd_eta_dn7), ((if assign30050_e44374 >= 1e-38 { (((locals.var_pd_a_dn8 * locals.var_gam_sb2) - (locals.var_pd_a * locals.var_gam_sb2_dn8)) / (locals.var_gam_sb2 * locals.var_gam_sb2)) } else { 0.0 } / assign30050_e44376) - locals.var_pd_eta_dn8), ((if assign30050_e44374 >= 1e-38 { (((locals.var_pd_a_dn9 * locals.var_gam_sb2) - (locals.var_pd_a * locals.var_gam_sb2_dn9)) / (locals.var_gam_sb2 * locals.var_gam_sb2)) } else { 0.0 } / assign30050_e44376) - locals.var_pd_eta_dn9), ((if assign30050_e44374 >= 1e-38 { (((locals.var_pd_a_dn10 * locals.var_gam_sb2) - (locals.var_pd_a * locals.var_gam_sb2_dn10)) / (locals.var_gam_sb2 * locals.var_gam_sb2)) } else { 0.0 } / assign30050_e44376) - locals.var_pd_eta_dn10), ((if assign30050_e44374 >= 1e-38 { (((locals.var_pd_a_dn11 * locals.var_gam_sb2) - (locals.var_pd_a * locals.var_gam_sb2_dn11)) / (locals.var_gam_sb2 * locals.var_gam_sb2)) } else { 0.0 } / assign30050_e44376) - locals.var_pd_eta_dn11),)
    } else {
        (locals.var_pd_tau, locals.var_pd_tau_dn3, locals.var_pd_tau_dn4, locals.var_pd_tau_dn5, locals.var_pd_tau_dn6, locals.var_pd_tau_dn7, locals.var_pd_tau_dn8, locals.var_pd_tau_dn9, locals.var_pd_tau_dn10, locals.var_pd_tau_dn11,)
    }
};
        locals.var_pd_tau = assign30050_e44381;
        locals.var_pd_tau_dn3 = assign30050_e44381_d_n3;
        locals.var_pd_tau_dn4 = assign30050_e44381_d_n4;
        locals.var_pd_tau_dn5 = assign30050_e44381_d_n5;
        locals.var_pd_tau_dn6 = assign30050_e44381_d_n6;
        locals.var_pd_tau_dn7 = assign30050_e44381_d_n7;
        locals.var_pd_tau_dn8 = assign30050_e44381_d_n8;
        locals.var_pd_tau_dn9 = assign30050_e44381_d_n9;
        locals.var_pd_tau_dn10 = assign30050_e44381_d_n10;
        locals.var_pd_tau_dn11 = assign30050_e44381_d_n11;
        locals.var_pd_tau_rv = 0.0;

        let (assign30060_e44394, assign30060_e44394_d_n3, assign30060_e44394_d_n4, assign30060_e44394_d_n5, assign30060_e44394_d_n6, assign30060_e44394_d_n7, assign30060_e44394_d_n8, assign30060_e44394_d_n9, assign30060_e44394_d_n10, assign30060_e44394_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30060_e44392: f64 = (locals.var_pd_a + locals.var_pd_c);
        (assign30060_e44392, (locals.var_pd_a_dn3 + locals.var_pd_c_dn3), (locals.var_pd_a_dn4 + locals.var_pd_c_dn4), (locals.var_pd_a_dn5 + locals.var_pd_c_dn5), (locals.var_pd_a_dn6 + locals.var_pd_c_dn6), (locals.var_pd_a_dn7 + locals.var_pd_c_dn7), (locals.var_pd_a_dn8 + locals.var_pd_c_dn8), (locals.var_pd_a_dn9 + locals.var_pd_c_dn9), (locals.var_pd_a_dn10 + locals.var_pd_c_dn10), (locals.var_pd_a_dn11 + locals.var_pd_c_dn11),)
    } else {
        (locals.var_nu, locals.var_nu_dn3, locals.var_nu_dn4, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9, locals.var_nu_dn10, locals.var_nu_dn11,)
    }
};
        locals.var_nu = assign30060_e44394;
        locals.var_nu_dn3 = assign30060_e44394_d_n3;
        locals.var_nu_dn4 = assign30060_e44394_d_n4;
        locals.var_nu_dn5 = assign30060_e44394_d_n5;
        locals.var_nu_dn6 = assign30060_e44394_d_n6;
        locals.var_nu_dn7 = assign30060_e44394_d_n7;
        locals.var_nu_dn8 = assign30060_e44394_d_n8;
        locals.var_nu_dn9 = assign30060_e44394_d_n9;
        locals.var_nu_dn10 = assign30060_e44394_d_n10;
        locals.var_nu_dn11 = assign30060_e44394_d_n11;
        locals.var_nu_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_87(
        locals: &mut StampLocals,
    ) {
        let (assign30070_e44417, assign30070_e44417_d_n3, assign30070_e44417_d_n4, assign30070_e44417_d_n5, assign30070_e44417_d_n6, assign30070_e44417_d_n7, assign30070_e44417_d_n8, assign30070_e44417_d_n9, assign30070_e44417_d_n10, assign30070_e44417_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30070_e44405: f64 = (locals.var_nu * locals.var_nu);
        let assign30070_e44410: f64 = (locals.var_pd_c * locals.var_pd_c);
        let assign30070_e44411: f64 = (0.5 * assign30070_e44410);
        let assign30070_e44413: f64 = (assign30070_e44411 - locals.var_pd_a);
        let assign30070_e44414: f64 = (locals.var_pd_tau * assign30070_e44413);
        let assign30070_e44415: f64 = (assign30070_e44405 + assign30070_e44414);
        (assign30070_e44415, (((locals.var_nu_dn3 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn3)) + ((locals.var_pd_tau_dn3 * assign30070_e44413) + (locals.var_pd_tau * ((0.5 * ((locals.var_pd_c_dn3 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn3))) - locals.var_pd_a_dn3)))), (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_pd_tau_dn4 * assign30070_e44413) + (locals.var_pd_tau * ((0.5 * ((locals.var_pd_c_dn4 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn4))) - locals.var_pd_a_dn4)))), (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_pd_tau_dn5 * assign30070_e44413) + (locals.var_pd_tau * ((0.5 * ((locals.var_pd_c_dn5 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn5))) - locals.var_pd_a_dn5)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_pd_tau_dn6 * assign30070_e44413) + (locals.var_pd_tau * ((0.5 * ((locals.var_pd_c_dn6 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn6))) - locals.var_pd_a_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_pd_tau_dn7 * assign30070_e44413) + (locals.var_pd_tau * ((0.5 * ((locals.var_pd_c_dn7 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn7))) - locals.var_pd_a_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_pd_tau_dn8 * assign30070_e44413) + (locals.var_pd_tau * ((0.5 * ((locals.var_pd_c_dn8 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn8))) - locals.var_pd_a_dn8)))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_pd_tau_dn9 * assign30070_e44413) + (locals.var_pd_tau * ((0.5 * ((locals.var_pd_c_dn9 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn9))) - locals.var_pd_a_dn9)))), (((locals.var_nu_dn10 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn10)) + ((locals.var_pd_tau_dn10 * assign30070_e44413) + (locals.var_pd_tau * ((0.5 * ((locals.var_pd_c_dn10 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn10))) - locals.var_pd_a_dn10)))), (((locals.var_nu_dn11 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn11)) + ((locals.var_pd_tau_dn11 * assign30070_e44413) + (locals.var_pd_tau * ((0.5 * ((locals.var_pd_c_dn11 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn11))) - locals.var_pd_a_dn11)))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn3, locals.var_mutau_dn4, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9, locals.var_mutau_dn10, locals.var_mutau_dn11,)
    }
};
        locals.var_mutau = assign30070_e44417;
        locals.var_mutau_dn3 = assign30070_e44417_d_n3;
        locals.var_mutau_dn4 = assign30070_e44417_d_n4;
        locals.var_mutau_dn5 = assign30070_e44417_d_n5;
        locals.var_mutau_dn6 = assign30070_e44417_d_n6;
        locals.var_mutau_dn7 = assign30070_e44417_d_n7;
        locals.var_mutau_dn8 = assign30070_e44417_d_n8;
        locals.var_mutau_dn9 = assign30070_e44417_d_n9;
        locals.var_mutau_dn10 = assign30070_e44417_d_n10;
        locals.var_mutau_dn11 = assign30070_e44417_d_n11;
        locals.var_mutau_rv = 0.0;

        let (assign30080_e44454, assign30080_e44454_d_n3, assign30080_e44454_d_n4, assign30080_e44454_d_n5, assign30080_e44454_d_n6, assign30080_e44454_d_n7, assign30080_e44454_d_n8, assign30080_e44454_d_n9, assign30080_e44454_d_n10, assign30080_e44454_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30080_e44429: f64 = (locals.var_pd_a * locals.var_nu);
        let assign30080_e44431: f64 = (assign30080_e44429 * locals.var_pd_tau);
        let assign30080_e44435: f64 = (locals.var_nu / locals.var_mutau);
        let assign30080_e44437: f64 = (assign30080_e44435 * locals.var_pd_tau);
        let assign30080_e44439: f64 = (assign30080_e44437 * locals.var_pd_tau);
        let assign30080_e44441: f64 = (assign30080_e44439 * locals.var_pd_c);
        let assign30080_e44444: f64 = (locals.var_pd_c * locals.var_pd_c);
        let assign30080_e44446: f64 = (assign30080_e44444 * 0.3333333333333333);
        let assign30080_e44448: f64 = (assign30080_e44446 - locals.var_pd_a);
        let assign30080_e44449: f64 = (assign30080_e44441 * assign30080_e44448);
        let assign30080_e44450: f64 = (locals.var_mutau + assign30080_e44449);
        let assign30080_e44451: f64 = (assign30080_e44431 / assign30080_e44450);
        let assign30080_e44452: f64 = (locals.var_pd_eta + assign30080_e44451);
        (assign30080_e44452, (locals.var_pd_eta_dn3 + (((((((locals.var_pd_a_dn3 * locals.var_nu) + (locals.var_pd_a * locals.var_nu_dn3)) * locals.var_pd_tau) + (assign30080_e44429 * locals.var_pd_tau_dn3)) * assign30080_e44450) - (assign30080_e44431 * (locals.var_mutau_dn3 + (((((((((((locals.var_nu_dn3 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn3)) / (locals.var_mutau * locals.var_mutau)) * locals.var_pd_tau) + (assign30080_e44435 * locals.var_pd_tau_dn3)) * locals.var_pd_tau) + (assign30080_e44437 * locals.var_pd_tau_dn3)) * locals.var_pd_c) + (assign30080_e44439 * locals.var_pd_c_dn3)) * assign30080_e44448) + (assign30080_e44441 * ((((locals.var_pd_c_dn3 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn3)) * 0.3333333333333333) - locals.var_pd_a_dn3)))))) / (assign30080_e44450 * assign30080_e44450))), (locals.var_pd_eta_dn4 + (((((((locals.var_pd_a_dn4 * locals.var_nu) + (locals.var_pd_a * locals.var_nu_dn4)) * locals.var_pd_tau) + (assign30080_e44429 * locals.var_pd_tau_dn4)) * assign30080_e44450) - (assign30080_e44431 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_pd_tau) + (assign30080_e44435 * locals.var_pd_tau_dn4)) * locals.var_pd_tau) + (assign30080_e44437 * locals.var_pd_tau_dn4)) * locals.var_pd_c) + (assign30080_e44439 * locals.var_pd_c_dn4)) * assign30080_e44448) + (assign30080_e44441 * ((((locals.var_pd_c_dn4 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn4)) * 0.3333333333333333) - locals.var_pd_a_dn4)))))) / (assign30080_e44450 * assign30080_e44450))), (locals.var_pd_eta_dn5 + (((((((locals.var_pd_a_dn5 * locals.var_nu) + (locals.var_pd_a * locals.var_nu_dn5)) * locals.var_pd_tau) + (assign30080_e44429 * locals.var_pd_tau_dn5)) * assign30080_e44450) - (assign30080_e44431 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_pd_tau) + (assign30080_e44435 * locals.var_pd_tau_dn5)) * locals.var_pd_tau) + (assign30080_e44437 * locals.var_pd_tau_dn5)) * locals.var_pd_c) + (assign30080_e44439 * locals.var_pd_c_dn5)) * assign30080_e44448) + (assign30080_e44441 * ((((locals.var_pd_c_dn5 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn5)) * 0.3333333333333333) - locals.var_pd_a_dn5)))))) / (assign30080_e44450 * assign30080_e44450))), (locals.var_pd_eta_dn6 + (((((((locals.var_pd_a_dn6 * locals.var_nu) + (locals.var_pd_a * locals.var_nu_dn6)) * locals.var_pd_tau) + (assign30080_e44429 * locals.var_pd_tau_dn6)) * assign30080_e44450) - (assign30080_e44431 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_pd_tau) + (assign30080_e44435 * locals.var_pd_tau_dn6)) * locals.var_pd_tau) + (assign30080_e44437 * locals.var_pd_tau_dn6)) * locals.var_pd_c) + (assign30080_e44439 * locals.var_pd_c_dn6)) * assign30080_e44448) + (assign30080_e44441 * ((((locals.var_pd_c_dn6 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn6)) * 0.3333333333333333) - locals.var_pd_a_dn6)))))) / (assign30080_e44450 * assign30080_e44450))), (locals.var_pd_eta_dn7 + (((((((locals.var_pd_a_dn7 * locals.var_nu) + (locals.var_pd_a * locals.var_nu_dn7)) * locals.var_pd_tau) + (assign30080_e44429 * locals.var_pd_tau_dn7)) * assign30080_e44450) - (assign30080_e44431 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_pd_tau) + (assign30080_e44435 * locals.var_pd_tau_dn7)) * locals.var_pd_tau) + (assign30080_e44437 * locals.var_pd_tau_dn7)) * locals.var_pd_c) + (assign30080_e44439 * locals.var_pd_c_dn7)) * assign30080_e44448) + (assign30080_e44441 * ((((locals.var_pd_c_dn7 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn7)) * 0.3333333333333333) - locals.var_pd_a_dn7)))))) / (assign30080_e44450 * assign30080_e44450))), (locals.var_pd_eta_dn8 + (((((((locals.var_pd_a_dn8 * locals.var_nu) + (locals.var_pd_a * locals.var_nu_dn8)) * locals.var_pd_tau) + (assign30080_e44429 * locals.var_pd_tau_dn8)) * assign30080_e44450) - (assign30080_e44431 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_pd_tau) + (assign30080_e44435 * locals.var_pd_tau_dn8)) * locals.var_pd_tau) + (assign30080_e44437 * locals.var_pd_tau_dn8)) * locals.var_pd_c) + (assign30080_e44439 * locals.var_pd_c_dn8)) * assign30080_e44448) + (assign30080_e44441 * ((((locals.var_pd_c_dn8 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn8)) * 0.3333333333333333) - locals.var_pd_a_dn8)))))) / (assign30080_e44450 * assign30080_e44450))), (locals.var_pd_eta_dn9 + (((((((locals.var_pd_a_dn9 * locals.var_nu) + (locals.var_pd_a * locals.var_nu_dn9)) * locals.var_pd_tau) + (assign30080_e44429 * locals.var_pd_tau_dn9)) * assign30080_e44450) - (assign30080_e44431 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_pd_tau) + (assign30080_e44435 * locals.var_pd_tau_dn9)) * locals.var_pd_tau) + (assign30080_e44437 * locals.var_pd_tau_dn9)) * locals.var_pd_c) + (assign30080_e44439 * locals.var_pd_c_dn9)) * assign30080_e44448) + (assign30080_e44441 * ((((locals.var_pd_c_dn9 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn9)) * 0.3333333333333333) - locals.var_pd_a_dn9)))))) / (assign30080_e44450 * assign30080_e44450))), (locals.var_pd_eta_dn10 + (((((((locals.var_pd_a_dn10 * locals.var_nu) + (locals.var_pd_a * locals.var_nu_dn10)) * locals.var_pd_tau) + (assign30080_e44429 * locals.var_pd_tau_dn10)) * assign30080_e44450) - (assign30080_e44431 * (locals.var_mutau_dn10 + (((((((((((locals.var_nu_dn10 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn10)) / (locals.var_mutau * locals.var_mutau)) * locals.var_pd_tau) + (assign30080_e44435 * locals.var_pd_tau_dn10)) * locals.var_pd_tau) + (assign30080_e44437 * locals.var_pd_tau_dn10)) * locals.var_pd_c) + (assign30080_e44439 * locals.var_pd_c_dn10)) * assign30080_e44448) + (assign30080_e44441 * ((((locals.var_pd_c_dn10 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn10)) * 0.3333333333333333) - locals.var_pd_a_dn10)))))) / (assign30080_e44450 * assign30080_e44450))), (locals.var_pd_eta_dn11 + (((((((locals.var_pd_a_dn11 * locals.var_nu) + (locals.var_pd_a * locals.var_nu_dn11)) * locals.var_pd_tau) + (assign30080_e44429 * locals.var_pd_tau_dn11)) * assign30080_e44450) - (assign30080_e44431 * (locals.var_mutau_dn11 + (((((((((((locals.var_nu_dn11 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn11)) / (locals.var_mutau * locals.var_mutau)) * locals.var_pd_tau) + (assign30080_e44435 * locals.var_pd_tau_dn11)) * locals.var_pd_tau) + (assign30080_e44437 * locals.var_pd_tau_dn11)) * locals.var_pd_c) + (assign30080_e44439 * locals.var_pd_c_dn11)) * assign30080_e44448) + (assign30080_e44441 * ((((locals.var_pd_c_dn11 * locals.var_pd_c) + (locals.var_pd_c * locals.var_pd_c_dn11)) * 0.3333333333333333) - locals.var_pd_a_dn11)))))) / (assign30080_e44450 * assign30080_e44450))),)
    } else {
        (locals.var_pd_y0, locals.var_pd_y0_dn3, locals.var_pd_y0_dn4, locals.var_pd_y0_dn5, locals.var_pd_y0_dn6, locals.var_pd_y0_dn7, locals.var_pd_y0_dn8, locals.var_pd_y0_dn9, locals.var_pd_y0_dn10, locals.var_pd_y0_dn11,)
    }
};
        locals.var_pd_y0 = assign30080_e44454;
        locals.var_pd_y0_dn3 = assign30080_e44454_d_n3;
        locals.var_pd_y0_dn4 = assign30080_e44454_d_n4;
        locals.var_pd_y0_dn5 = assign30080_e44454_d_n5;
        locals.var_pd_y0_dn6 = assign30080_e44454_d_n6;
        locals.var_pd_y0_dn7 = assign30080_e44454_d_n7;
        locals.var_pd_y0_dn8 = assign30080_e44454_d_n8;
        locals.var_pd_y0_dn9 = assign30080_e44454_d_n9;
        locals.var_pd_y0_dn10 = assign30080_e44454_d_n10;
        locals.var_pd_y0_dn11 = assign30080_e44454_d_n11;
        locals.var_pd_y0_rv = 0.0;

        let (assign30090_e44466, assign30090_e44466_d_n3, assign30090_e44466_d_n4, assign30090_e44466_d_n5, assign30090_e44466_d_n6, assign30090_e44466_d_n7, assign30090_e44466_d_n8, assign30090_e44466_d_n9, assign30090_e44466_d_n10, assign30090_e44466_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30090_e44464: f64 = { let limited_exp_arg = locals.var_pd_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign30090_e44464, ({ let limited_exp_arg = locals.var_pd_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_pd_y0_dn3), ({ let limited_exp_arg = locals.var_pd_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_pd_y0_dn4), ({ let limited_exp_arg = locals.var_pd_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_pd_y0_dn5), ({ let limited_exp_arg = locals.var_pd_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_pd_y0_dn6), ({ let limited_exp_arg = locals.var_pd_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_pd_y0_dn7), ({ let limited_exp_arg = locals.var_pd_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_pd_y0_dn8), ({ let limited_exp_arg = locals.var_pd_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_pd_y0_dn9), ({ let limited_exp_arg = locals.var_pd_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_pd_y0_dn10), ({ let limited_exp_arg = locals.var_pd_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_pd_y0_dn11),)
    } else {
        (locals.var_pd_d0, locals.var_pd_d0_dn3, locals.var_pd_d0_dn4, locals.var_pd_d0_dn5, locals.var_pd_d0_dn6, locals.var_pd_d0_dn7, locals.var_pd_d0_dn8, locals.var_pd_d0_dn9, locals.var_pd_d0_dn10, locals.var_pd_d0_dn11,)
    }
};
        locals.var_pd_d0 = assign30090_e44466;
        locals.var_pd_d0_dn3 = assign30090_e44466_d_n3;
        locals.var_pd_d0_dn4 = assign30090_e44466_d_n4;
        locals.var_pd_d0_dn5 = assign30090_e44466_d_n5;
        locals.var_pd_d0_dn6 = assign30090_e44466_d_n6;
        locals.var_pd_d0_dn7 = assign30090_e44466_d_n7;
        locals.var_pd_d0_dn8 = assign30090_e44466_d_n8;
        locals.var_pd_d0_dn9 = assign30090_e44466_d_n9;
        locals.var_pd_d0_dn10 = assign30090_e44466_d_n10;
        locals.var_pd_d0_dn11 = assign30090_e44466_d_n11;
        locals.var_pd_d0_rv = 0.0;

        let (assign30100_e44479, assign30100_e44479_d_n3, assign30100_e44479_d_n4, assign30100_e44479_d_n5, assign30100_e44479_d_n6, assign30100_e44479_d_n7, assign30100_e44479_d_n8, assign30100_e44479_d_n9, assign30100_e44479_d_n10, assign30100_e44479_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30100_e44477: f64 = (locals.var_pd_yg - locals.var_pd_y0);
        (assign30100_e44477, (locals.var_pd_yg_dn3 - locals.var_pd_y0_dn3), (locals.var_pd_yg_dn4 - locals.var_pd_y0_dn4), (locals.var_pd_yg_dn5 - locals.var_pd_y0_dn5), (locals.var_pd_yg_dn6 - locals.var_pd_y0_dn6), (locals.var_pd_yg_dn7 - locals.var_pd_y0_dn7), (locals.var_pd_yg_dn8 - locals.var_pd_y0_dn8), (locals.var_pd_yg_dn9 - locals.var_pd_y0_dn9), (locals.var_pd_yg_dn10 - locals.var_pd_y0_dn10), (locals.var_pd_yg_dn11 - locals.var_pd_y0_dn11),)
    } else {
        (locals.var_pd_temp, locals.var_pd_temp_dn3, locals.var_pd_temp_dn4, locals.var_pd_temp_dn5, locals.var_pd_temp_dn6, locals.var_pd_temp_dn7, locals.var_pd_temp_dn8, locals.var_pd_temp_dn9, locals.var_pd_temp_dn10, locals.var_pd_temp_dn11,)
    }
};
        locals.var_pd_temp = assign30100_e44479;
        locals.var_pd_temp_dn3 = assign30100_e44479_d_n3;
        locals.var_pd_temp_dn4 = assign30100_e44479_d_n4;
        locals.var_pd_temp_dn5 = assign30100_e44479_d_n5;
        locals.var_pd_temp_dn6 = assign30100_e44479_d_n6;
        locals.var_pd_temp_dn7 = assign30100_e44479_d_n7;
        locals.var_pd_temp_dn8 = assign30100_e44479_d_n8;
        locals.var_pd_temp_dn9 = assign30100_e44479_d_n9;
        locals.var_pd_temp_dn10 = assign30100_e44479_d_n10;
        locals.var_pd_temp_dn11 = assign30100_e44479_d_n11;
        locals.var_pd_temp_rv = 0.0;

        let (assign30110_e44498, assign30110_e44498_d_n3, assign30110_e44498_d_n4, assign30110_e44498_d_n5, assign30110_e44498_d_n6, assign30110_e44498_d_n7, assign30110_e44498_d_n8, assign30110_e44498_d_n9, assign30110_e44498_d_n10, assign30110_e44498_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30110_e44490: f64 = (2.0 * locals.var_pd_temp);
        let assign30110_e44494: f64 = (locals.var_pd_d0 - 1.0);
        let assign30110_e44495: f64 = (locals.var_gam_sb2 * assign30110_e44494);
        let assign30110_e44496: f64 = (assign30110_e44490 + assign30110_e44495);
        (assign30110_e44496, ((2.0 * locals.var_pd_temp_dn3) + ((locals.var_gam_sb2_dn3 * assign30110_e44494) + (locals.var_gam_sb2 * locals.var_pd_d0_dn3))), ((2.0 * locals.var_pd_temp_dn4) + ((locals.var_gam_sb2_dn4 * assign30110_e44494) + (locals.var_gam_sb2 * locals.var_pd_d0_dn4))), ((2.0 * locals.var_pd_temp_dn5) + ((locals.var_gam_sb2_dn5 * assign30110_e44494) + (locals.var_gam_sb2 * locals.var_pd_d0_dn5))), ((2.0 * locals.var_pd_temp_dn6) + ((locals.var_gam_sb2_dn6 * assign30110_e44494) + (locals.var_gam_sb2 * locals.var_pd_d0_dn6))), ((2.0 * locals.var_pd_temp_dn7) + ((locals.var_gam_sb2_dn7 * assign30110_e44494) + (locals.var_gam_sb2 * locals.var_pd_d0_dn7))), ((2.0 * locals.var_pd_temp_dn8) + ((locals.var_gam_sb2_dn8 * assign30110_e44494) + (locals.var_gam_sb2 * locals.var_pd_d0_dn8))), ((2.0 * locals.var_pd_temp_dn9) + ((locals.var_gam_sb2_dn9 * assign30110_e44494) + (locals.var_gam_sb2 * locals.var_pd_d0_dn9))), ((2.0 * locals.var_pd_temp_dn10) + ((locals.var_gam_sb2_dn10 * assign30110_e44494) + (locals.var_gam_sb2 * locals.var_pd_d0_dn10))), ((2.0 * locals.var_pd_temp_dn11) + ((locals.var_gam_sb2_dn11 * assign30110_e44494) + (locals.var_gam_sb2 * locals.var_pd_d0_dn11))),)
    } else {
        (locals.var_pd_p, locals.var_pd_p_dn3, locals.var_pd_p_dn4, locals.var_pd_p_dn5, locals.var_pd_p_dn6, locals.var_pd_p_dn7, locals.var_pd_p_dn8, locals.var_pd_p_dn9, locals.var_pd_p_dn10, locals.var_pd_p_dn11,)
    }
};
        locals.var_pd_p = assign30110_e44498;
        locals.var_pd_p_dn3 = assign30110_e44498_d_n3;
        locals.var_pd_p_dn4 = assign30110_e44498_d_n4;
        locals.var_pd_p_dn5 = assign30110_e44498_d_n5;
        locals.var_pd_p_dn6 = assign30110_e44498_d_n6;
        locals.var_pd_p_dn7 = assign30110_e44498_d_n7;
        locals.var_pd_p_dn8 = assign30110_e44498_d_n8;
        locals.var_pd_p_dn9 = assign30110_e44498_d_n9;
        locals.var_pd_p_dn10 = assign30110_e44498_d_n10;
        locals.var_pd_p_dn11 = assign30110_e44498_d_n11;
        locals.var_pd_p_rv = 0.0;

        let (assign30120_e44519, assign30120_e44519_d_n3, assign30120_e44519_d_n4, assign30120_e44519_d_n5, assign30120_e44519_d_n6, assign30120_e44519_d_n7, assign30120_e44519_d_n8, assign30120_e44519_d_n9, assign30120_e44519_d_n10, assign30120_e44519_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30120_e44509: f64 = (locals.var_pd_temp * locals.var_pd_temp);
        let assign30120_e44513: f64 = (locals.var_pd_y0 + 1.0);
        let assign30120_e44515: f64 = (assign30120_e44513 - locals.var_pd_d0);
        let assign30120_e44516: f64 = (locals.var_gam_sb2 * assign30120_e44515);
        let assign30120_e44517: f64 = (assign30120_e44509 + assign30120_e44516);
        (assign30120_e44517, (((locals.var_pd_temp_dn3 * locals.var_pd_temp) + (locals.var_pd_temp * locals.var_pd_temp_dn3)) + ((locals.var_gam_sb2_dn3 * assign30120_e44515) + (locals.var_gam_sb2 * (locals.var_pd_y0_dn3 - locals.var_pd_d0_dn3)))), (((locals.var_pd_temp_dn4 * locals.var_pd_temp) + (locals.var_pd_temp * locals.var_pd_temp_dn4)) + ((locals.var_gam_sb2_dn4 * assign30120_e44515) + (locals.var_gam_sb2 * (locals.var_pd_y0_dn4 - locals.var_pd_d0_dn4)))), (((locals.var_pd_temp_dn5 * locals.var_pd_temp) + (locals.var_pd_temp * locals.var_pd_temp_dn5)) + ((locals.var_gam_sb2_dn5 * assign30120_e44515) + (locals.var_gam_sb2 * (locals.var_pd_y0_dn5 - locals.var_pd_d0_dn5)))), (((locals.var_pd_temp_dn6 * locals.var_pd_temp) + (locals.var_pd_temp * locals.var_pd_temp_dn6)) + ((locals.var_gam_sb2_dn6 * assign30120_e44515) + (locals.var_gam_sb2 * (locals.var_pd_y0_dn6 - locals.var_pd_d0_dn6)))), (((locals.var_pd_temp_dn7 * locals.var_pd_temp) + (locals.var_pd_temp * locals.var_pd_temp_dn7)) + ((locals.var_gam_sb2_dn7 * assign30120_e44515) + (locals.var_gam_sb2 * (locals.var_pd_y0_dn7 - locals.var_pd_d0_dn7)))), (((locals.var_pd_temp_dn8 * locals.var_pd_temp) + (locals.var_pd_temp * locals.var_pd_temp_dn8)) + ((locals.var_gam_sb2_dn8 * assign30120_e44515) + (locals.var_gam_sb2 * (locals.var_pd_y0_dn8 - locals.var_pd_d0_dn8)))), (((locals.var_pd_temp_dn9 * locals.var_pd_temp) + (locals.var_pd_temp * locals.var_pd_temp_dn9)) + ((locals.var_gam_sb2_dn9 * assign30120_e44515) + (locals.var_gam_sb2 * (locals.var_pd_y0_dn9 - locals.var_pd_d0_dn9)))), (((locals.var_pd_temp_dn10 * locals.var_pd_temp) + (locals.var_pd_temp * locals.var_pd_temp_dn10)) + ((locals.var_gam_sb2_dn10 * assign30120_e44515) + (locals.var_gam_sb2 * (locals.var_pd_y0_dn10 - locals.var_pd_d0_dn10)))), (((locals.var_pd_temp_dn11 * locals.var_pd_temp) + (locals.var_pd_temp * locals.var_pd_temp_dn11)) + ((locals.var_gam_sb2_dn11 * assign30120_e44515) + (locals.var_gam_sb2 * (locals.var_pd_y0_dn11 - locals.var_pd_d0_dn11)))),)
    } else {
        (locals.var_pd_q, locals.var_pd_q_dn3, locals.var_pd_q_dn4, locals.var_pd_q_dn5, locals.var_pd_q_dn6, locals.var_pd_q_dn7, locals.var_pd_q_dn8, locals.var_pd_q_dn9, locals.var_pd_q_dn10, locals.var_pd_q_dn11,)
    }
};
        locals.var_pd_q = assign30120_e44519;
        locals.var_pd_q_dn3 = assign30120_e44519_d_n3;
        locals.var_pd_q_dn4 = assign30120_e44519_d_n4;
        locals.var_pd_q_dn5 = assign30120_e44519_d_n5;
        locals.var_pd_q_dn6 = assign30120_e44519_d_n6;
        locals.var_pd_q_dn7 = assign30120_e44519_d_n7;
        locals.var_pd_q_dn8 = assign30120_e44519_d_n8;
        locals.var_pd_q_dn9 = assign30120_e44519_d_n9;
        locals.var_pd_q_dn10 = assign30120_e44519_d_n10;
        locals.var_pd_q_dn11 = assign30120_e44519_d_n11;
        locals.var_pd_q_rv = 0.0;

        let (assign30130_e44536, assign30130_e44536_d_n3, assign30130_e44536_d_n4, assign30130_e44536_d_n5, assign30130_e44536_d_n6, assign30130_e44536_d_n7, assign30130_e44536_d_n8, assign30130_e44536_d_n9, assign30130_e44536_d_n10, assign30130_e44536_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30130_e44531: f64 = (locals.var_gam_sb2 * 0.5);
        let assign30130_e44533: f64 = (assign30130_e44531 * locals.var_pd_d0);
        let assign30130_e44534: f64 = (1.0 - assign30130_e44533);
        (assign30130_e44534, (-(((locals.var_gam_sb2_dn3 * 0.5) * locals.var_pd_d0) + (assign30130_e44531 * locals.var_pd_d0_dn3))), (-(((locals.var_gam_sb2_dn4 * 0.5) * locals.var_pd_d0) + (assign30130_e44531 * locals.var_pd_d0_dn4))), (-(((locals.var_gam_sb2_dn5 * 0.5) * locals.var_pd_d0) + (assign30130_e44531 * locals.var_pd_d0_dn5))), (-(((locals.var_gam_sb2_dn6 * 0.5) * locals.var_pd_d0) + (assign30130_e44531 * locals.var_pd_d0_dn6))), (-(((locals.var_gam_sb2_dn7 * 0.5) * locals.var_pd_d0) + (assign30130_e44531 * locals.var_pd_d0_dn7))), (-(((locals.var_gam_sb2_dn8 * 0.5) * locals.var_pd_d0) + (assign30130_e44531 * locals.var_pd_d0_dn8))), (-(((locals.var_gam_sb2_dn9 * 0.5) * locals.var_pd_d0) + (assign30130_e44531 * locals.var_pd_d0_dn9))), (-(((locals.var_gam_sb2_dn10 * 0.5) * locals.var_pd_d0) + (assign30130_e44531 * locals.var_pd_d0_dn10))), (-(((locals.var_gam_sb2_dn11 * 0.5) * locals.var_pd_d0) + (assign30130_e44531 * locals.var_pd_d0_dn11))),)
    } else {
        (locals.var_pd_xi, locals.var_pd_xi_dn3, locals.var_pd_xi_dn4, locals.var_pd_xi_dn5, locals.var_pd_xi_dn6, locals.var_pd_xi_dn7, locals.var_pd_xi_dn8, locals.var_pd_xi_dn9, locals.var_pd_xi_dn10, locals.var_pd_xi_dn11,)
    }
};
        locals.var_pd_xi = assign30130_e44536;
        locals.var_pd_xi_dn3 = assign30130_e44536_d_n3;
        locals.var_pd_xi_dn4 = assign30130_e44536_d_n4;
        locals.var_pd_xi_dn5 = assign30130_e44536_d_n5;
        locals.var_pd_xi_dn6 = assign30130_e44536_d_n6;
        locals.var_pd_xi_dn7 = assign30130_e44536_d_n7;
        locals.var_pd_xi_dn8 = assign30130_e44536_d_n8;
        locals.var_pd_xi_dn9 = assign30130_e44536_d_n9;
        locals.var_pd_xi_dn10 = assign30130_e44536_d_n10;
        locals.var_pd_xi_dn11 = assign30130_e44536_d_n11;
        locals.var_pd_xi_rv = 0.0;

        let (assign30140_e44555, assign30140_e44555_d_n3, assign30140_e44555_d_n4, assign30140_e44555_d_n5, assign30140_e44555_d_n6, assign30140_e44555_d_n7, assign30140_e44555_d_n8, assign30140_e44555_d_n9, assign30140_e44555_d_n10, assign30140_e44555_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30140_e44547: f64 = (locals.var_pd_p * locals.var_pd_p);
        let assign30140_e44551: f64 = (locals.var_pd_xi * locals.var_pd_q);
        let assign30140_e44552: f64 = (4.0 * assign30140_e44551);
        let assign30140_e44553: f64 = (assign30140_e44547 - assign30140_e44552);
        (assign30140_e44553, (((locals.var_pd_p_dn3 * locals.var_pd_p) + (locals.var_pd_p * locals.var_pd_p_dn3)) - (4.0 * ((locals.var_pd_xi_dn3 * locals.var_pd_q) + (locals.var_pd_xi * locals.var_pd_q_dn3)))), (((locals.var_pd_p_dn4 * locals.var_pd_p) + (locals.var_pd_p * locals.var_pd_p_dn4)) - (4.0 * ((locals.var_pd_xi_dn4 * locals.var_pd_q) + (locals.var_pd_xi * locals.var_pd_q_dn4)))), (((locals.var_pd_p_dn5 * locals.var_pd_p) + (locals.var_pd_p * locals.var_pd_p_dn5)) - (4.0 * ((locals.var_pd_xi_dn5 * locals.var_pd_q) + (locals.var_pd_xi * locals.var_pd_q_dn5)))), (((locals.var_pd_p_dn6 * locals.var_pd_p) + (locals.var_pd_p * locals.var_pd_p_dn6)) - (4.0 * ((locals.var_pd_xi_dn6 * locals.var_pd_q) + (locals.var_pd_xi * locals.var_pd_q_dn6)))), (((locals.var_pd_p_dn7 * locals.var_pd_p) + (locals.var_pd_p * locals.var_pd_p_dn7)) - (4.0 * ((locals.var_pd_xi_dn7 * locals.var_pd_q) + (locals.var_pd_xi * locals.var_pd_q_dn7)))), (((locals.var_pd_p_dn8 * locals.var_pd_p) + (locals.var_pd_p * locals.var_pd_p_dn8)) - (4.0 * ((locals.var_pd_xi_dn8 * locals.var_pd_q) + (locals.var_pd_xi * locals.var_pd_q_dn8)))), (((locals.var_pd_p_dn9 * locals.var_pd_p) + (locals.var_pd_p * locals.var_pd_p_dn9)) - (4.0 * ((locals.var_pd_xi_dn9 * locals.var_pd_q) + (locals.var_pd_xi * locals.var_pd_q_dn9)))), (((locals.var_pd_p_dn10 * locals.var_pd_p) + (locals.var_pd_p * locals.var_pd_p_dn10)) - (4.0 * ((locals.var_pd_xi_dn10 * locals.var_pd_q) + (locals.var_pd_xi * locals.var_pd_q_dn10)))), (((locals.var_pd_p_dn11 * locals.var_pd_p) + (locals.var_pd_p * locals.var_pd_p_dn11)) - (4.0 * ((locals.var_pd_xi_dn11 * locals.var_pd_q) + (locals.var_pd_xi * locals.var_pd_q_dn11)))),)
    } else {
        (locals.var_pd_temp, locals.var_pd_temp_dn3, locals.var_pd_temp_dn4, locals.var_pd_temp_dn5, locals.var_pd_temp_dn6, locals.var_pd_temp_dn7, locals.var_pd_temp_dn8, locals.var_pd_temp_dn9, locals.var_pd_temp_dn10, locals.var_pd_temp_dn11,)
    }
};
        locals.var_pd_temp = assign30140_e44555;
        locals.var_pd_temp_dn3 = assign30140_e44555_d_n3;
        locals.var_pd_temp_dn4 = assign30140_e44555_d_n4;
        locals.var_pd_temp_dn5 = assign30140_e44555_d_n5;
        locals.var_pd_temp_dn6 = assign30140_e44555_d_n6;
        locals.var_pd_temp_dn7 = assign30140_e44555_d_n7;
        locals.var_pd_temp_dn8 = assign30140_e44555_d_n8;
        locals.var_pd_temp_dn9 = assign30140_e44555_d_n9;
        locals.var_pd_temp_dn10 = assign30140_e44555_d_n10;
        locals.var_pd_temp_dn11 = assign30140_e44555_d_n11;
        locals.var_pd_temp_rv = 0.0;

        let (assign30150_e44573, assign30150_e44573_d_n3, assign30150_e44573_d_n4, assign30150_e44573_d_n5, assign30150_e44573_d_n6, assign30150_e44573_d_n7, assign30150_e44573_d_n8, assign30150_e44573_d_n9, assign30150_e44573_d_n10, assign30150_e44573_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30150_e44568: f64 = (locals.var_pd_temp).sqrt();
        let assign30150_e44569: f64 = (locals.var_pd_p + assign30150_e44568);
        let assign30150_e44570: f64 = (locals.var_pd_q / assign30150_e44569);
        let assign30150_e44571: f64 = (2.0 * assign30150_e44570);
        (assign30150_e44571, (2.0 * (((locals.var_pd_q_dn3 * assign30150_e44569) - (locals.var_pd_q * (locals.var_pd_p_dn3 + (locals.var_pd_temp_dn3 / (2.0 * assign30150_e44568))))) / (assign30150_e44569 * assign30150_e44569))), (2.0 * (((locals.var_pd_q_dn4 * assign30150_e44569) - (locals.var_pd_q * (locals.var_pd_p_dn4 + (locals.var_pd_temp_dn4 / (2.0 * assign30150_e44568))))) / (assign30150_e44569 * assign30150_e44569))), (2.0 * (((locals.var_pd_q_dn5 * assign30150_e44569) - (locals.var_pd_q * (locals.var_pd_p_dn5 + (locals.var_pd_temp_dn5 / (2.0 * assign30150_e44568))))) / (assign30150_e44569 * assign30150_e44569))), (2.0 * (((locals.var_pd_q_dn6 * assign30150_e44569) - (locals.var_pd_q * (locals.var_pd_p_dn6 + (locals.var_pd_temp_dn6 / (2.0 * assign30150_e44568))))) / (assign30150_e44569 * assign30150_e44569))), (2.0 * (((locals.var_pd_q_dn7 * assign30150_e44569) - (locals.var_pd_q * (locals.var_pd_p_dn7 + (locals.var_pd_temp_dn7 / (2.0 * assign30150_e44568))))) / (assign30150_e44569 * assign30150_e44569))), (2.0 * (((locals.var_pd_q_dn8 * assign30150_e44569) - (locals.var_pd_q * (locals.var_pd_p_dn8 + (locals.var_pd_temp_dn8 / (2.0 * assign30150_e44568))))) / (assign30150_e44569 * assign30150_e44569))), (2.0 * (((locals.var_pd_q_dn9 * assign30150_e44569) - (locals.var_pd_q * (locals.var_pd_p_dn9 + (locals.var_pd_temp_dn9 / (2.0 * assign30150_e44568))))) / (assign30150_e44569 * assign30150_e44569))), (2.0 * (((locals.var_pd_q_dn10 * assign30150_e44569) - (locals.var_pd_q * (locals.var_pd_p_dn10 + (locals.var_pd_temp_dn10 / (2.0 * assign30150_e44568))))) / (assign30150_e44569 * assign30150_e44569))), (2.0 * (((locals.var_pd_q_dn11 * assign30150_e44569) - (locals.var_pd_q * (locals.var_pd_p_dn11 + (locals.var_pd_temp_dn11 / (2.0 * assign30150_e44568))))) / (assign30150_e44569 * assign30150_e44569))),)
    } else {
        (locals.var_pd_w, locals.var_pd_w_dn3, locals.var_pd_w_dn4, locals.var_pd_w_dn5, locals.var_pd_w_dn6, locals.var_pd_w_dn7, locals.var_pd_w_dn8, locals.var_pd_w_dn9, locals.var_pd_w_dn10, locals.var_pd_w_dn11,)
    }
};
        locals.var_pd_w = assign30150_e44573;
        locals.var_pd_w_dn3 = assign30150_e44573_d_n3;
        locals.var_pd_w_dn4 = assign30150_e44573_d_n4;
        locals.var_pd_w_dn5 = assign30150_e44573_d_n5;
        locals.var_pd_w_dn6 = assign30150_e44573_d_n6;
        locals.var_pd_w_dn7 = assign30150_e44573_d_n7;
        locals.var_pd_w_dn8 = assign30150_e44573_d_n8;
        locals.var_pd_w_dn9 = assign30150_e44573_d_n9;
        locals.var_pd_w_dn10 = assign30150_e44573_d_n10;
        locals.var_pd_w_dn11 = assign30150_e44573_d_n11;
        locals.var_pd_w_rv = 0.0;

        let (assign30160_e44587, assign30160_e44587_d_n3, assign30160_e44587_d_n4, assign30160_e44587_d_n5, assign30160_e44587_d_n6, assign30160_e44587_d_n7, assign30160_e44587_d_n8, assign30160_e44587_d_n9, assign30160_e44587_d_n10, assign30160_e44587_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 != 0.0)) {
        let assign30160_e44584: f64 = (locals.var_pd_y0 + locals.var_pd_w);
        let assign30160_e44585: f64 = (-assign30160_e44584);
        (assign30160_e44585, (-(locals.var_pd_y0_dn3 + locals.var_pd_w_dn3)), (-(locals.var_pd_y0_dn4 + locals.var_pd_w_dn4)), (-(locals.var_pd_y0_dn5 + locals.var_pd_w_dn5)), (-(locals.var_pd_y0_dn6 + locals.var_pd_w_dn6)), (-(locals.var_pd_y0_dn7 + locals.var_pd_w_dn7)), (-(locals.var_pd_y0_dn8 + locals.var_pd_w_dn8)), (-(locals.var_pd_y0_dn9 + locals.var_pd_w_dn9)), (-(locals.var_pd_y0_dn10 + locals.var_pd_w_dn10)), (-(locals.var_pd_y0_dn11 + locals.var_pd_w_dn11)),)
    } else {
        (locals.var_pd_sb, locals.var_pd_sb_dn3, locals.var_pd_sb_dn4, locals.var_pd_sb_dn5, locals.var_pd_sb_dn6, locals.var_pd_sb_dn7, locals.var_pd_sb_dn8, locals.var_pd_sb_dn9, locals.var_pd_sb_dn10, locals.var_pd_sb_dn11,)
    }
};
        locals.var_pd_sb = assign30160_e44587;
        locals.var_pd_sb_dn3 = assign30160_e44587_d_n3;
        locals.var_pd_sb_dn4 = assign30160_e44587_d_n4;
        locals.var_pd_sb_dn5 = assign30160_e44587_d_n5;
        locals.var_pd_sb_dn6 = assign30160_e44587_d_n6;
        locals.var_pd_sb_dn7 = assign30160_e44587_d_n7;
        locals.var_pd_sb_dn8 = assign30160_e44587_d_n8;
        locals.var_pd_sb_dn9 = assign30160_e44587_d_n9;
        locals.var_pd_sb_dn10 = assign30160_e44587_d_n10;
        locals.var_pd_sb_dn11 = assign30160_e44587_d_n11;
        locals.var_pd_sb_rv = 0.0;

        let (assign30170_e44607, assign30170_e44607_d_n3, assign30170_e44607_d_n4, assign30170_e44607_d_n5, assign30170_e44607_d_n6, assign30170_e44607_d_n7, assign30170_e44607_d_n8, assign30170_e44607_d_n9, assign30170_e44607_d_n10, assign30170_e44607_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 == 0.0)) {
        let assign30170_e44599: f64 = (locals.var_x1_sb * locals.var_x1_csb);
        let assign30170_e44601: f64 = (assign30170_e44599 * locals.var_inv_xg1_sb);
        let assign30170_e44603: f64 = (assign30170_e44601 - 1.0);
        let assign30170_e44605: f64 = (assign30170_e44603 * locals.var_inv_xg1_sb);
        (assign30170_e44605, (((((locals.var_x1_sb_dn3 * locals.var_x1_csb) * locals.var_inv_xg1_sb) + (assign30170_e44599 * locals.var_inv_xg1_sb_dn3)) * locals.var_inv_xg1_sb) + (assign30170_e44603 * locals.var_inv_xg1_sb_dn3)), (((((locals.var_x1_sb_dn4 * locals.var_x1_csb) * locals.var_inv_xg1_sb) + (assign30170_e44599 * locals.var_inv_xg1_sb_dn4)) * locals.var_inv_xg1_sb) + (assign30170_e44603 * locals.var_inv_xg1_sb_dn4)), (((((locals.var_x1_sb_dn5 * locals.var_x1_csb) * locals.var_inv_xg1_sb) + (assign30170_e44599 * locals.var_inv_xg1_sb_dn5)) * locals.var_inv_xg1_sb) + (assign30170_e44603 * locals.var_inv_xg1_sb_dn5)), (((((locals.var_x1_sb_dn6 * locals.var_x1_csb) * locals.var_inv_xg1_sb) + (assign30170_e44599 * locals.var_inv_xg1_sb_dn6)) * locals.var_inv_xg1_sb) + (assign30170_e44603 * locals.var_inv_xg1_sb_dn6)), (((((locals.var_x1_sb_dn7 * locals.var_x1_csb) * locals.var_inv_xg1_sb) + (assign30170_e44599 * locals.var_inv_xg1_sb_dn7)) * locals.var_inv_xg1_sb) + (assign30170_e44603 * locals.var_inv_xg1_sb_dn7)), (((((locals.var_x1_sb_dn8 * locals.var_x1_csb) * locals.var_inv_xg1_sb) + (assign30170_e44599 * locals.var_inv_xg1_sb_dn8)) * locals.var_inv_xg1_sb) + (assign30170_e44603 * locals.var_inv_xg1_sb_dn8)), (((((locals.var_x1_sb_dn9 * locals.var_x1_csb) * locals.var_inv_xg1_sb) + (assign30170_e44599 * locals.var_inv_xg1_sb_dn9)) * locals.var_inv_xg1_sb) + (assign30170_e44603 * locals.var_inv_xg1_sb_dn9)), (((((locals.var_x1_sb_dn10 * locals.var_x1_csb) * locals.var_inv_xg1_sb) + (assign30170_e44599 * locals.var_inv_xg1_sb_dn10)) * locals.var_inv_xg1_sb) + (assign30170_e44603 * locals.var_inv_xg1_sb_dn10)), (((((locals.var_x1_sb_dn11 * locals.var_x1_csb) * locals.var_inv_xg1_sb) + (assign30170_e44599 * locals.var_inv_xg1_sb_dn11)) * locals.var_inv_xg1_sb) + (assign30170_e44603 * locals.var_inv_xg1_sb_dn11)),)
    } else {
        (locals.var_pd_afac, locals.var_pd_afac_dn3, locals.var_pd_afac_dn4, locals.var_pd_afac_dn5, locals.var_pd_afac_dn6, locals.var_pd_afac_dn7, locals.var_pd_afac_dn8, locals.var_pd_afac_dn9, locals.var_pd_afac_dn10, locals.var_pd_afac_dn11,)
    }
};
        locals.var_pd_afac = assign30170_e44607;
        locals.var_pd_afac_dn3 = assign30170_e44607_d_n3;
        locals.var_pd_afac_dn4 = assign30170_e44607_d_n4;
        locals.var_pd_afac_dn5 = assign30170_e44607_d_n5;
        locals.var_pd_afac_dn6 = assign30170_e44607_d_n6;
        locals.var_pd_afac_dn7 = assign30170_e44607_d_n7;
        locals.var_pd_afac_dn8 = assign30170_e44607_d_n8;
        locals.var_pd_afac_dn9 = assign30170_e44607_d_n9;
        locals.var_pd_afac_dn10 = assign30170_e44607_d_n10;
        locals.var_pd_afac_dn11 = assign30170_e44607_d_n11;
        locals.var_pd_afac_rv = 0.0;

        let (assign30180_e44627, assign30180_e44627_d_n3, assign30180_e44627_d_n4, assign30180_e44627_d_n5, assign30180_e44627_d_n6, assign30180_e44627_d_n7, assign30180_e44627_d_n8, assign30180_e44627_d_n9, assign30180_e44627_d_n10, assign30180_e44627_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 == 0.0)) {
        let assign30180_e44619: f64 = (locals.var_vgfbb * locals.var_inv_xi_sb);
        let assign30180_e44623: f64 = (locals.var_pd_afac * locals.var_vgfbb);
        let assign30180_e44624: f64 = (1.0 + assign30180_e44623);
        let assign30180_e44625: f64 = (assign30180_e44619 * assign30180_e44624);
        (assign30180_e44625, ((((locals.var_vgfbb_dn3 * locals.var_inv_xi_sb) + (locals.var_vgfbb * locals.var_inv_xi_sb_dn3)) * assign30180_e44624) + (assign30180_e44619 * ((locals.var_pd_afac_dn3 * locals.var_vgfbb) + (locals.var_pd_afac * locals.var_vgfbb_dn3)))), ((((locals.var_vgfbb_dn4 * locals.var_inv_xi_sb) + (locals.var_vgfbb * locals.var_inv_xi_sb_dn4)) * assign30180_e44624) + (assign30180_e44619 * ((locals.var_pd_afac_dn4 * locals.var_vgfbb) + (locals.var_pd_afac * locals.var_vgfbb_dn4)))), ((((locals.var_vgfbb_dn5 * locals.var_inv_xi_sb) + (locals.var_vgfbb * locals.var_inv_xi_sb_dn5)) * assign30180_e44624) + (assign30180_e44619 * ((locals.var_pd_afac_dn5 * locals.var_vgfbb) + (locals.var_pd_afac * locals.var_vgfbb_dn5)))), ((((locals.var_vgfbb_dn6 * locals.var_inv_xi_sb) + (locals.var_vgfbb * locals.var_inv_xi_sb_dn6)) * assign30180_e44624) + (assign30180_e44619 * ((locals.var_pd_afac_dn6 * locals.var_vgfbb) + (locals.var_pd_afac * locals.var_vgfbb_dn6)))), ((((locals.var_vgfbb_dn7 * locals.var_inv_xi_sb) + (locals.var_vgfbb * locals.var_inv_xi_sb_dn7)) * assign30180_e44624) + (assign30180_e44619 * ((locals.var_pd_afac_dn7 * locals.var_vgfbb) + (locals.var_pd_afac * locals.var_vgfbb_dn7)))), ((((locals.var_vgfbb_dn8 * locals.var_inv_xi_sb) + (locals.var_vgfbb * locals.var_inv_xi_sb_dn8)) * assign30180_e44624) + (assign30180_e44619 * ((locals.var_pd_afac_dn8 * locals.var_vgfbb) + (locals.var_pd_afac * locals.var_vgfbb_dn8)))), ((((locals.var_vgfbb_dn9 * locals.var_inv_xi_sb) + (locals.var_vgfbb * locals.var_inv_xi_sb_dn9)) * assign30180_e44624) + (assign30180_e44619 * ((locals.var_pd_afac_dn9 * locals.var_vgfbb) + (locals.var_pd_afac * locals.var_vgfbb_dn9)))), ((((locals.var_vgfbb_dn10 * locals.var_inv_xi_sb) + (locals.var_vgfbb * locals.var_inv_xi_sb_dn10)) * assign30180_e44624) + (assign30180_e44619 * ((locals.var_pd_afac_dn10 * locals.var_vgfbb) + (locals.var_pd_afac * locals.var_vgfbb_dn10)))), ((((locals.var_vgfbb_dn11 * locals.var_inv_xi_sb) + (locals.var_vgfbb * locals.var_inv_xi_sb_dn11)) * assign30180_e44624) + (assign30180_e44619 * ((locals.var_pd_afac_dn11 * locals.var_vgfbb) + (locals.var_pd_afac * locals.var_vgfbb_dn11)))),)
    } else {
        (locals.var_pd_xbar, locals.var_pd_xbar_dn3, locals.var_pd_xbar_dn4, locals.var_pd_xbar_dn5, locals.var_pd_xbar_dn6, locals.var_pd_xbar_dn7, locals.var_pd_xbar_dn8, locals.var_pd_xbar_dn9, locals.var_pd_xbar_dn10, locals.var_pd_xbar_dn11,)
    }
};
        locals.var_pd_xbar = assign30180_e44627;
        locals.var_pd_xbar_dn3 = assign30180_e44627_d_n3;
        locals.var_pd_xbar_dn4 = assign30180_e44627_d_n4;
        locals.var_pd_xbar_dn5 = assign30180_e44627_d_n5;
        locals.var_pd_xbar_dn6 = assign30180_e44627_d_n6;
        locals.var_pd_xbar_dn7 = assign30180_e44627_d_n7;
        locals.var_pd_xbar_dn8 = assign30180_e44627_d_n8;
        locals.var_pd_xbar_dn9 = assign30180_e44627_d_n9;
        locals.var_pd_xbar_dn10 = assign30180_e44627_d_n10;
        locals.var_pd_xbar_dn11 = assign30180_e44627_d_n11;
        locals.var_pd_xbar_rv = 0.0;

        let (assign30190_e44641, assign30190_e44641_d_n3, assign30190_e44641_d_n4, assign30190_e44641_d_n5, assign30190_e44641_d_n6, assign30190_e44641_d_n7, assign30190_e44641_d_n8, assign30190_e44641_d_n9, assign30190_e44641_d_n10, assign30190_e44641_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 == 0.0)) {
        let assign30190_e44638: f64 = (-locals.var_pd_xbar);
        let assign30190_e44639: f64 = { let limited_exp_arg = assign30190_e44638; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign30190_e44639, ({ let limited_exp_arg = assign30190_e44638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_xbar_dn3)), ({ let limited_exp_arg = assign30190_e44638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_xbar_dn4)), ({ let limited_exp_arg = assign30190_e44638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_xbar_dn5)), ({ let limited_exp_arg = assign30190_e44638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_xbar_dn6)), ({ let limited_exp_arg = assign30190_e44638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_xbar_dn7)), ({ let limited_exp_arg = assign30190_e44638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_xbar_dn8)), ({ let limited_exp_arg = assign30190_e44638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_xbar_dn9)), ({ let limited_exp_arg = assign30190_e44638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_xbar_dn10)), ({ let limited_exp_arg = assign30190_e44638; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_xbar_dn11)),)
    } else {
        (locals.var_pd_temp1, locals.var_pd_temp1_dn3, locals.var_pd_temp1_dn4, locals.var_pd_temp1_dn5, locals.var_pd_temp1_dn6, locals.var_pd_temp1_dn7, locals.var_pd_temp1_dn8, locals.var_pd_temp1_dn9, locals.var_pd_temp1_dn10, locals.var_pd_temp1_dn11,)
    }
};
        locals.var_pd_temp1 = assign30190_e44641;
        locals.var_pd_temp1_dn3 = assign30190_e44641_d_n3;
        locals.var_pd_temp1_dn4 = assign30190_e44641_d_n4;
        locals.var_pd_temp1_dn5 = assign30190_e44641_d_n5;
        locals.var_pd_temp1_dn6 = assign30190_e44641_d_n6;
        locals.var_pd_temp1_dn7 = assign30190_e44641_d_n7;
        locals.var_pd_temp1_dn8 = assign30190_e44641_d_n8;
        locals.var_pd_temp1_dn9 = assign30190_e44641_d_n9;
        locals.var_pd_temp1_dn10 = assign30190_e44641_d_n10;
        locals.var_pd_temp1_dn11 = assign30190_e44641_d_n11;
        locals.var_pd_temp1_rv = 0.0;

        let (assign30200_e44655, assign30200_e44655_d_n3, assign30200_e44655_d_n4, assign30200_e44655_d_n5, assign30200_e44655_d_n6, assign30200_e44655_d_n7, assign30200_e44655_d_n8, assign30200_e44655_d_n9, assign30200_e44655_d_n10, assign30200_e44655_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 == 0.0)) {
        let assign30200_e44653: f64 = (1.0 - locals.var_pd_temp1);
        (assign30200_e44653, (-locals.var_pd_temp1_dn3), (-locals.var_pd_temp1_dn4), (-locals.var_pd_temp1_dn5), (-locals.var_pd_temp1_dn6), (-locals.var_pd_temp1_dn7), (-locals.var_pd_temp1_dn8), (-locals.var_pd_temp1_dn9), (-locals.var_pd_temp1_dn10), (-locals.var_pd_temp1_dn11),)
    } else {
        (locals.var_pd_w1, locals.var_pd_w1_dn3, locals.var_pd_w1_dn4, locals.var_pd_w1_dn5, locals.var_pd_w1_dn6, locals.var_pd_w1_dn7, locals.var_pd_w1_dn8, locals.var_pd_w1_dn9, locals.var_pd_w1_dn10, locals.var_pd_w1_dn11,)
    }
};
        locals.var_pd_w1 = assign30200_e44655;
        locals.var_pd_w1_dn3 = assign30200_e44655_d_n3;
        locals.var_pd_w1_dn4 = assign30200_e44655_d_n4;
        locals.var_pd_w1_dn5 = assign30200_e44655_d_n5;
        locals.var_pd_w1_dn6 = assign30200_e44655_d_n6;
        locals.var_pd_w1_dn7 = assign30200_e44655_d_n7;
        locals.var_pd_w1_dn8 = assign30200_e44655_d_n8;
        locals.var_pd_w1_dn9 = assign30200_e44655_d_n9;
        locals.var_pd_w1_dn10 = assign30200_e44655_d_n10;
        locals.var_pd_w1_dn11 = assign30200_e44655_d_n11;
        locals.var_pd_w1_rv = 0.0;

        let (assign30210_e44682, assign30210_e44682_d_n3, assign30210_e44682_d_n4, assign30210_e44682_d_n5, assign30210_e44682_d_n6, assign30210_e44682_d_n7, assign30210_e44682_d_n8, assign30210_e44682_d_n9, assign30210_e44682_d_n10, assign30210_e44682_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 == 0.0)) {
        let assign30210_e44668: f64 = (locals.var_gam_sb2 * 0.5);
        let assign30210_e44669: f64 = (locals.var_vgfbb + assign30210_e44668);
        let assign30210_e44674: f64 = (locals.var_gam_sb2 * 0.25);
        let assign30210_e44675: f64 = (locals.var_vgfbb + assign30210_e44674);
        let assign30210_e44677: f64 = (assign30210_e44675 - locals.var_pd_w1);
        let assign30210_e44678: f64 = (assign30210_e44677).sqrt();
        let assign30210_e44679: f64 = (locals.var_gam_sb * assign30210_e44678);
        let assign30210_e44680: f64 = (assign30210_e44669 - assign30210_e44679);
        (assign30210_e44680, ((locals.var_vgfbb_dn3 + (locals.var_gam_sb2_dn3 * 0.5)) - ((locals.var_gam_sb_dn3 * assign30210_e44678) + (locals.var_gam_sb * (((locals.var_vgfbb_dn3 + (locals.var_gam_sb2_dn3 * 0.25)) - locals.var_pd_w1_dn3) / (2.0 * assign30210_e44678))))), ((locals.var_vgfbb_dn4 + (locals.var_gam_sb2_dn4 * 0.5)) - ((locals.var_gam_sb_dn4 * assign30210_e44678) + (locals.var_gam_sb * (((locals.var_vgfbb_dn4 + (locals.var_gam_sb2_dn4 * 0.25)) - locals.var_pd_w1_dn4) / (2.0 * assign30210_e44678))))), ((locals.var_vgfbb_dn5 + (locals.var_gam_sb2_dn5 * 0.5)) - ((locals.var_gam_sb_dn5 * assign30210_e44678) + (locals.var_gam_sb * (((locals.var_vgfbb_dn5 + (locals.var_gam_sb2_dn5 * 0.25)) - locals.var_pd_w1_dn5) / (2.0 * assign30210_e44678))))), ((locals.var_vgfbb_dn6 + (locals.var_gam_sb2_dn6 * 0.5)) - ((locals.var_gam_sb_dn6 * assign30210_e44678) + (locals.var_gam_sb * (((locals.var_vgfbb_dn6 + (locals.var_gam_sb2_dn6 * 0.25)) - locals.var_pd_w1_dn6) / (2.0 * assign30210_e44678))))), ((locals.var_vgfbb_dn7 + (locals.var_gam_sb2_dn7 * 0.5)) - ((locals.var_gam_sb_dn7 * assign30210_e44678) + (locals.var_gam_sb * (((locals.var_vgfbb_dn7 + (locals.var_gam_sb2_dn7 * 0.25)) - locals.var_pd_w1_dn7) / (2.0 * assign30210_e44678))))), ((locals.var_vgfbb_dn8 + (locals.var_gam_sb2_dn8 * 0.5)) - ((locals.var_gam_sb_dn8 * assign30210_e44678) + (locals.var_gam_sb * (((locals.var_vgfbb_dn8 + (locals.var_gam_sb2_dn8 * 0.25)) - locals.var_pd_w1_dn8) / (2.0 * assign30210_e44678))))), ((locals.var_vgfbb_dn9 + (locals.var_gam_sb2_dn9 * 0.5)) - ((locals.var_gam_sb_dn9 * assign30210_e44678) + (locals.var_gam_sb * (((locals.var_vgfbb_dn9 + (locals.var_gam_sb2_dn9 * 0.25)) - locals.var_pd_w1_dn9) / (2.0 * assign30210_e44678))))), ((locals.var_vgfbb_dn10 + (locals.var_gam_sb2_dn10 * 0.5)) - ((locals.var_gam_sb_dn10 * assign30210_e44678) + (locals.var_gam_sb * (((locals.var_vgfbb_dn10 + (locals.var_gam_sb2_dn10 * 0.25)) - locals.var_pd_w1_dn10) / (2.0 * assign30210_e44678))))), ((locals.var_vgfbb_dn11 + (locals.var_gam_sb2_dn11 * 0.5)) - ((locals.var_gam_sb_dn11 * assign30210_e44678) + (locals.var_gam_sb * (((locals.var_vgfbb_dn11 + (locals.var_gam_sb2_dn11 * 0.25)) - locals.var_pd_w1_dn11) / (2.0 * assign30210_e44678))))),)
    } else {
        (locals.var_pd_x0, locals.var_pd_x0_dn3, locals.var_pd_x0_dn4, locals.var_pd_x0_dn5, locals.var_pd_x0_dn6, locals.var_pd_x0_dn7, locals.var_pd_x0_dn8, locals.var_pd_x0_dn9, locals.var_pd_x0_dn10, locals.var_pd_x0_dn11,)
    }
};
        locals.var_pd_x0 = assign30210_e44682;
        locals.var_pd_x0_dn3 = assign30210_e44682_d_n3;
        locals.var_pd_x0_dn4 = assign30210_e44682_d_n4;
        locals.var_pd_x0_dn5 = assign30210_e44682_d_n5;
        locals.var_pd_x0_dn6 = assign30210_e44682_d_n6;
        locals.var_pd_x0_dn7 = assign30210_e44682_d_n7;
        locals.var_pd_x0_dn8 = assign30210_e44682_d_n8;
        locals.var_pd_x0_dn9 = assign30210_e44682_d_n9;
        locals.var_pd_x0_dn10 = assign30210_e44682_d_n10;
        locals.var_pd_x0_dn11 = assign30210_e44682_d_n11;
        locals.var_pd_x0_rv = 0.0;

        let (assign30220_e44696, assign30220_e44696_d_n3, assign30220_e44696_d_n4, assign30220_e44696_d_n5, assign30220_e44696_d_n6, assign30220_e44696_d_n7, assign30220_e44696_d_n8, assign30220_e44696_d_n9, assign30220_e44696_d_n10, assign30220_e44696_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 == 0.0)) {
        let assign30220_e44693: f64 = (-locals.var_pd_x0);
        let assign30220_e44694: f64 = { let limited_exp_arg = assign30220_e44693; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign30220_e44694, ({ let limited_exp_arg = assign30220_e44693; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_x0_dn3)), ({ let limited_exp_arg = assign30220_e44693; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_x0_dn4)), ({ let limited_exp_arg = assign30220_e44693; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_x0_dn5)), ({ let limited_exp_arg = assign30220_e44693; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_x0_dn6)), ({ let limited_exp_arg = assign30220_e44693; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_x0_dn7)), ({ let limited_exp_arg = assign30220_e44693; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_x0_dn8)), ({ let limited_exp_arg = assign30220_e44693; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_x0_dn9)), ({ let limited_exp_arg = assign30220_e44693; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_x0_dn10)), ({ let limited_exp_arg = assign30220_e44693; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_x0_dn11)),)
    } else {
        (locals.var_pd_d01, locals.var_pd_d01_dn3, locals.var_pd_d01_dn4, locals.var_pd_d01_dn5, locals.var_pd_d01_dn6, locals.var_pd_d01_dn7, locals.var_pd_d01_dn8, locals.var_pd_d01_dn9, locals.var_pd_d01_dn10, locals.var_pd_d01_dn11,)
    }
};
        locals.var_pd_d01 = assign30220_e44696;
        locals.var_pd_d01_dn3 = assign30220_e44696_d_n3;
        locals.var_pd_d01_dn4 = assign30220_e44696_d_n4;
        locals.var_pd_d01_dn5 = assign30220_e44696_d_n5;
        locals.var_pd_d01_dn6 = assign30220_e44696_d_n6;
        locals.var_pd_d01_dn7 = assign30220_e44696_d_n7;
        locals.var_pd_d01_dn8 = assign30220_e44696_d_n8;
        locals.var_pd_d01_dn9 = assign30220_e44696_d_n9;
        locals.var_pd_d01_dn10 = assign30220_e44696_d_n10;
        locals.var_pd_d01_dn11 = assign30220_e44696_d_n11;
        locals.var_pd_d01_rv = 0.0;

        let (assign30230_e44718, assign30230_e44718_d_n3, assign30230_e44718_d_n4, assign30230_e44718_d_n5, assign30230_e44718_d_n6, assign30230_e44718_d_n7, assign30230_e44718_d_n8, assign30230_e44718_d_n9, assign30230_e44718_d_n10, assign30230_e44718_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 == 0.0)) {
        let assign30230_e44709: f64 = (locals.var_vgfbb - locals.var_pd_x0);
        let assign30230_e44710: f64 = (2.0 * assign30230_e44709);
        let assign30230_e44714: f64 = (1.0 - locals.var_pd_d01);
        let assign30230_e44715: f64 = (locals.var_gam_sb2 * assign30230_e44714);
        let assign30230_e44716: f64 = (assign30230_e44710 + assign30230_e44715);
        (assign30230_e44716, ((2.0 * (locals.var_vgfbb_dn3 - locals.var_pd_x0_dn3)) + ((locals.var_gam_sb2_dn3 * assign30230_e44714) + (locals.var_gam_sb2 * (-locals.var_pd_d01_dn3)))), ((2.0 * (locals.var_vgfbb_dn4 - locals.var_pd_x0_dn4)) + ((locals.var_gam_sb2_dn4 * assign30230_e44714) + (locals.var_gam_sb2 * (-locals.var_pd_d01_dn4)))), ((2.0 * (locals.var_vgfbb_dn5 - locals.var_pd_x0_dn5)) + ((locals.var_gam_sb2_dn5 * assign30230_e44714) + (locals.var_gam_sb2 * (-locals.var_pd_d01_dn5)))), ((2.0 * (locals.var_vgfbb_dn6 - locals.var_pd_x0_dn6)) + ((locals.var_gam_sb2_dn6 * assign30230_e44714) + (locals.var_gam_sb2 * (-locals.var_pd_d01_dn6)))), ((2.0 * (locals.var_vgfbb_dn7 - locals.var_pd_x0_dn7)) + ((locals.var_gam_sb2_dn7 * assign30230_e44714) + (locals.var_gam_sb2 * (-locals.var_pd_d01_dn7)))), ((2.0 * (locals.var_vgfbb_dn8 - locals.var_pd_x0_dn8)) + ((locals.var_gam_sb2_dn8 * assign30230_e44714) + (locals.var_gam_sb2 * (-locals.var_pd_d01_dn8)))), ((2.0 * (locals.var_vgfbb_dn9 - locals.var_pd_x0_dn9)) + ((locals.var_gam_sb2_dn9 * assign30230_e44714) + (locals.var_gam_sb2 * (-locals.var_pd_d01_dn9)))), ((2.0 * (locals.var_vgfbb_dn10 - locals.var_pd_x0_dn10)) + ((locals.var_gam_sb2_dn10 * assign30230_e44714) + (locals.var_gam_sb2 * (-locals.var_pd_d01_dn10)))), ((2.0 * (locals.var_vgfbb_dn11 - locals.var_pd_x0_dn11)) + ((locals.var_gam_sb2_dn11 * assign30230_e44714) + (locals.var_gam_sb2 * (-locals.var_pd_d01_dn11)))),)
    } else {
        (locals.var_pd_p1, locals.var_pd_p1_dn3, locals.var_pd_p1_dn4, locals.var_pd_p1_dn5, locals.var_pd_p1_dn6, locals.var_pd_p1_dn7, locals.var_pd_p1_dn8, locals.var_pd_p1_dn9, locals.var_pd_p1_dn10, locals.var_pd_p1_dn11,)
    }
};
        locals.var_pd_p1 = assign30230_e44718;
        locals.var_pd_p1_dn3 = assign30230_e44718_d_n3;
        locals.var_pd_p1_dn4 = assign30230_e44718_d_n4;
        locals.var_pd_p1_dn5 = assign30230_e44718_d_n5;
        locals.var_pd_p1_dn6 = assign30230_e44718_d_n6;
        locals.var_pd_p1_dn7 = assign30230_e44718_d_n7;
        locals.var_pd_p1_dn8 = assign30230_e44718_d_n8;
        locals.var_pd_p1_dn9 = assign30230_e44718_d_n9;
        locals.var_pd_p1_dn10 = assign30230_e44718_d_n10;
        locals.var_pd_p1_dn11 = assign30230_e44718_d_n11;
        locals.var_pd_p1_rv = 0.0;

        let (assign30240_e44744, assign30240_e44744_d_n3, assign30240_e44744_d_n4, assign30240_e44744_d_n5, assign30240_e44744_d_n6, assign30240_e44744_d_n7, assign30240_e44744_d_n8, assign30240_e44744_d_n9, assign30240_e44744_d_n10, assign30240_e44744_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 == 0.0)) {
        let assign30240_e44730: f64 = (locals.var_vgfbb - locals.var_pd_x0);
        let assign30240_e44733: f64 = (locals.var_vgfbb - locals.var_pd_x0);
        let assign30240_e44734: f64 = (assign30240_e44730 * assign30240_e44733);
        let assign30240_e44738: f64 = (locals.var_pd_x0 - 1.0);
        let assign30240_e44740: f64 = (assign30240_e44738 + locals.var_pd_d01);
        let assign30240_e44741: f64 = (locals.var_gam_sb2 * assign30240_e44740);
        let assign30240_e44742: f64 = (assign30240_e44734 - assign30240_e44741);
        (assign30240_e44742, ((((locals.var_vgfbb_dn3 - locals.var_pd_x0_dn3) * assign30240_e44733) + (assign30240_e44730 * (locals.var_vgfbb_dn3 - locals.var_pd_x0_dn3))) - ((locals.var_gam_sb2_dn3 * assign30240_e44740) + (locals.var_gam_sb2 * (locals.var_pd_x0_dn3 + locals.var_pd_d01_dn3)))), ((((locals.var_vgfbb_dn4 - locals.var_pd_x0_dn4) * assign30240_e44733) + (assign30240_e44730 * (locals.var_vgfbb_dn4 - locals.var_pd_x0_dn4))) - ((locals.var_gam_sb2_dn4 * assign30240_e44740) + (locals.var_gam_sb2 * (locals.var_pd_x0_dn4 + locals.var_pd_d01_dn4)))), ((((locals.var_vgfbb_dn5 - locals.var_pd_x0_dn5) * assign30240_e44733) + (assign30240_e44730 * (locals.var_vgfbb_dn5 - locals.var_pd_x0_dn5))) - ((locals.var_gam_sb2_dn5 * assign30240_e44740) + (locals.var_gam_sb2 * (locals.var_pd_x0_dn5 + locals.var_pd_d01_dn5)))), ((((locals.var_vgfbb_dn6 - locals.var_pd_x0_dn6) * assign30240_e44733) + (assign30240_e44730 * (locals.var_vgfbb_dn6 - locals.var_pd_x0_dn6))) - ((locals.var_gam_sb2_dn6 * assign30240_e44740) + (locals.var_gam_sb2 * (locals.var_pd_x0_dn6 + locals.var_pd_d01_dn6)))), ((((locals.var_vgfbb_dn7 - locals.var_pd_x0_dn7) * assign30240_e44733) + (assign30240_e44730 * (locals.var_vgfbb_dn7 - locals.var_pd_x0_dn7))) - ((locals.var_gam_sb2_dn7 * assign30240_e44740) + (locals.var_gam_sb2 * (locals.var_pd_x0_dn7 + locals.var_pd_d01_dn7)))), ((((locals.var_vgfbb_dn8 - locals.var_pd_x0_dn8) * assign30240_e44733) + (assign30240_e44730 * (locals.var_vgfbb_dn8 - locals.var_pd_x0_dn8))) - ((locals.var_gam_sb2_dn8 * assign30240_e44740) + (locals.var_gam_sb2 * (locals.var_pd_x0_dn8 + locals.var_pd_d01_dn8)))), ((((locals.var_vgfbb_dn9 - locals.var_pd_x0_dn9) * assign30240_e44733) + (assign30240_e44730 * (locals.var_vgfbb_dn9 - locals.var_pd_x0_dn9))) - ((locals.var_gam_sb2_dn9 * assign30240_e44740) + (locals.var_gam_sb2 * (locals.var_pd_x0_dn9 + locals.var_pd_d01_dn9)))), ((((locals.var_vgfbb_dn10 - locals.var_pd_x0_dn10) * assign30240_e44733) + (assign30240_e44730 * (locals.var_vgfbb_dn10 - locals.var_pd_x0_dn10))) - ((locals.var_gam_sb2_dn10 * assign30240_e44740) + (locals.var_gam_sb2 * (locals.var_pd_x0_dn10 + locals.var_pd_d01_dn10)))), ((((locals.var_vgfbb_dn11 - locals.var_pd_x0_dn11) * assign30240_e44733) + (assign30240_e44730 * (locals.var_vgfbb_dn11 - locals.var_pd_x0_dn11))) - ((locals.var_gam_sb2_dn11 * assign30240_e44740) + (locals.var_gam_sb2 * (locals.var_pd_x0_dn11 + locals.var_pd_d01_dn11)))),)
    } else {
        (locals.var_pd_q1, locals.var_pd_q1_dn3, locals.var_pd_q1_dn4, locals.var_pd_q1_dn5, locals.var_pd_q1_dn6, locals.var_pd_q1_dn7, locals.var_pd_q1_dn8, locals.var_pd_q1_dn9, locals.var_pd_q1_dn10, locals.var_pd_q1_dn11,)
    }
};
        locals.var_pd_q1 = assign30240_e44744;
        locals.var_pd_q1_dn3 = assign30240_e44744_d_n3;
        locals.var_pd_q1_dn4 = assign30240_e44744_d_n4;
        locals.var_pd_q1_dn5 = assign30240_e44744_d_n5;
        locals.var_pd_q1_dn6 = assign30240_e44744_d_n6;
        locals.var_pd_q1_dn7 = assign30240_e44744_d_n7;
        locals.var_pd_q1_dn8 = assign30240_e44744_d_n8;
        locals.var_pd_q1_dn9 = assign30240_e44744_d_n9;
        locals.var_pd_q1_dn10 = assign30240_e44744_d_n10;
        locals.var_pd_q1_dn11 = assign30240_e44744_d_n11;
        locals.var_pd_q1_rv = 0.0;

        let (assign30250_e44762, assign30250_e44762_d_n3, assign30250_e44762_d_n4, assign30250_e44762_d_n5, assign30250_e44762_d_n6, assign30250_e44762_d_n7, assign30250_e44762_d_n8, assign30250_e44762_d_n9, assign30250_e44762_d_n10, assign30250_e44762_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 == 0.0)) {
        let assign30250_e44757: f64 = (locals.var_gam_sb2 * 0.5);
        let assign30250_e44759: f64 = (assign30250_e44757 * locals.var_pd_d01);
        let assign30250_e44760: f64 = (1.0 - assign30250_e44759);
        (assign30250_e44760, (-(((locals.var_gam_sb2_dn3 * 0.5) * locals.var_pd_d01) + (assign30250_e44757 * locals.var_pd_d01_dn3))), (-(((locals.var_gam_sb2_dn4 * 0.5) * locals.var_pd_d01) + (assign30250_e44757 * locals.var_pd_d01_dn4))), (-(((locals.var_gam_sb2_dn5 * 0.5) * locals.var_pd_d01) + (assign30250_e44757 * locals.var_pd_d01_dn5))), (-(((locals.var_gam_sb2_dn6 * 0.5) * locals.var_pd_d01) + (assign30250_e44757 * locals.var_pd_d01_dn6))), (-(((locals.var_gam_sb2_dn7 * 0.5) * locals.var_pd_d01) + (assign30250_e44757 * locals.var_pd_d01_dn7))), (-(((locals.var_gam_sb2_dn8 * 0.5) * locals.var_pd_d01) + (assign30250_e44757 * locals.var_pd_d01_dn8))), (-(((locals.var_gam_sb2_dn9 * 0.5) * locals.var_pd_d01) + (assign30250_e44757 * locals.var_pd_d01_dn9))), (-(((locals.var_gam_sb2_dn10 * 0.5) * locals.var_pd_d01) + (assign30250_e44757 * locals.var_pd_d01_dn10))), (-(((locals.var_gam_sb2_dn11 * 0.5) * locals.var_pd_d01) + (assign30250_e44757 * locals.var_pd_d01_dn11))),)
    } else {
        (locals.var_pd_xi1, locals.var_pd_xi1_dn3, locals.var_pd_xi1_dn4, locals.var_pd_xi1_dn5, locals.var_pd_xi1_dn6, locals.var_pd_xi1_dn7, locals.var_pd_xi1_dn8, locals.var_pd_xi1_dn9, locals.var_pd_xi1_dn10, locals.var_pd_xi1_dn11,)
    }
};
        locals.var_pd_xi1 = assign30250_e44762;
        locals.var_pd_xi1_dn3 = assign30250_e44762_d_n3;
        locals.var_pd_xi1_dn4 = assign30250_e44762_d_n4;
        locals.var_pd_xi1_dn5 = assign30250_e44762_d_n5;
        locals.var_pd_xi1_dn6 = assign30250_e44762_d_n6;
        locals.var_pd_xi1_dn7 = assign30250_e44762_d_n7;
        locals.var_pd_xi1_dn8 = assign30250_e44762_d_n8;
        locals.var_pd_xi1_dn9 = assign30250_e44762_d_n9;
        locals.var_pd_xi1_dn10 = assign30250_e44762_d_n10;
        locals.var_pd_xi1_dn11 = assign30250_e44762_d_n11;
        locals.var_pd_xi1_rv = 0.0;

        let (assign30260_e44782, assign30260_e44782_d_n3, assign30260_e44782_d_n4, assign30260_e44782_d_n5, assign30260_e44782_d_n6, assign30260_e44782_d_n7, assign30260_e44782_d_n8, assign30260_e44782_d_n9, assign30260_e44782_d_n10, assign30260_e44782_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 == 0.0)) {
        let assign30260_e44774: f64 = (locals.var_pd_p1 * locals.var_pd_p1);
        let assign30260_e44778: f64 = (locals.var_pd_xi1 * locals.var_pd_q1);
        let assign30260_e44779: f64 = (4.0 * assign30260_e44778);
        let assign30260_e44780: f64 = (assign30260_e44774 - assign30260_e44779);
        (assign30260_e44780, (((locals.var_pd_p1_dn3 * locals.var_pd_p1) + (locals.var_pd_p1 * locals.var_pd_p1_dn3)) - (4.0 * ((locals.var_pd_xi1_dn3 * locals.var_pd_q1) + (locals.var_pd_xi1 * locals.var_pd_q1_dn3)))), (((locals.var_pd_p1_dn4 * locals.var_pd_p1) + (locals.var_pd_p1 * locals.var_pd_p1_dn4)) - (4.0 * ((locals.var_pd_xi1_dn4 * locals.var_pd_q1) + (locals.var_pd_xi1 * locals.var_pd_q1_dn4)))), (((locals.var_pd_p1_dn5 * locals.var_pd_p1) + (locals.var_pd_p1 * locals.var_pd_p1_dn5)) - (4.0 * ((locals.var_pd_xi1_dn5 * locals.var_pd_q1) + (locals.var_pd_xi1 * locals.var_pd_q1_dn5)))), (((locals.var_pd_p1_dn6 * locals.var_pd_p1) + (locals.var_pd_p1 * locals.var_pd_p1_dn6)) - (4.0 * ((locals.var_pd_xi1_dn6 * locals.var_pd_q1) + (locals.var_pd_xi1 * locals.var_pd_q1_dn6)))), (((locals.var_pd_p1_dn7 * locals.var_pd_p1) + (locals.var_pd_p1 * locals.var_pd_p1_dn7)) - (4.0 * ((locals.var_pd_xi1_dn7 * locals.var_pd_q1) + (locals.var_pd_xi1 * locals.var_pd_q1_dn7)))), (((locals.var_pd_p1_dn8 * locals.var_pd_p1) + (locals.var_pd_p1 * locals.var_pd_p1_dn8)) - (4.0 * ((locals.var_pd_xi1_dn8 * locals.var_pd_q1) + (locals.var_pd_xi1 * locals.var_pd_q1_dn8)))), (((locals.var_pd_p1_dn9 * locals.var_pd_p1) + (locals.var_pd_p1 * locals.var_pd_p1_dn9)) - (4.0 * ((locals.var_pd_xi1_dn9 * locals.var_pd_q1) + (locals.var_pd_xi1 * locals.var_pd_q1_dn9)))), (((locals.var_pd_p1_dn10 * locals.var_pd_p1) + (locals.var_pd_p1 * locals.var_pd_p1_dn10)) - (4.0 * ((locals.var_pd_xi1_dn10 * locals.var_pd_q1) + (locals.var_pd_xi1 * locals.var_pd_q1_dn10)))), (((locals.var_pd_p1_dn11 * locals.var_pd_p1) + (locals.var_pd_p1 * locals.var_pd_p1_dn11)) - (4.0 * ((locals.var_pd_xi1_dn11 * locals.var_pd_q1) + (locals.var_pd_xi1 * locals.var_pd_q1_dn11)))),)
    } else {
        (locals.var_pd_temp1, locals.var_pd_temp1_dn3, locals.var_pd_temp1_dn4, locals.var_pd_temp1_dn5, locals.var_pd_temp1_dn6, locals.var_pd_temp1_dn7, locals.var_pd_temp1_dn8, locals.var_pd_temp1_dn9, locals.var_pd_temp1_dn10, locals.var_pd_temp1_dn11,)
    }
};
        locals.var_pd_temp1 = assign30260_e44782;
        locals.var_pd_temp1_dn3 = assign30260_e44782_d_n3;
        locals.var_pd_temp1_dn4 = assign30260_e44782_d_n4;
        locals.var_pd_temp1_dn5 = assign30260_e44782_d_n5;
        locals.var_pd_temp1_dn6 = assign30260_e44782_d_n6;
        locals.var_pd_temp1_dn7 = assign30260_e44782_d_n7;
        locals.var_pd_temp1_dn8 = assign30260_e44782_d_n8;
        locals.var_pd_temp1_dn9 = assign30260_e44782_d_n9;
        locals.var_pd_temp1_dn10 = assign30260_e44782_d_n10;
        locals.var_pd_temp1_dn11 = assign30260_e44782_d_n11;
        locals.var_pd_temp1_rv = 0.0;

        let (assign30270_e44801, assign30270_e44801_d_n3, assign30270_e44801_d_n4, assign30270_e44801_d_n5, assign30270_e44801_d_n6, assign30270_e44801_d_n7, assign30270_e44801_d_n8, assign30270_e44801_d_n9, assign30270_e44801_d_n10, assign30270_e44801_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 == 0.0)) {
        let assign30270_e44796: f64 = (locals.var_pd_temp1).sqrt();
        let assign30270_e44797: f64 = (locals.var_pd_p1 + assign30270_e44796);
        let assign30270_e44798: f64 = (locals.var_pd_q1 / assign30270_e44797);
        let assign30270_e44799: f64 = (2.0 * assign30270_e44798);
        (assign30270_e44799, (2.0 * (((locals.var_pd_q1_dn3 * assign30270_e44797) - (locals.var_pd_q1 * (locals.var_pd_p1_dn3 + (locals.var_pd_temp1_dn3 / (2.0 * assign30270_e44796))))) / (assign30270_e44797 * assign30270_e44797))), (2.0 * (((locals.var_pd_q1_dn4 * assign30270_e44797) - (locals.var_pd_q1 * (locals.var_pd_p1_dn4 + (locals.var_pd_temp1_dn4 / (2.0 * assign30270_e44796))))) / (assign30270_e44797 * assign30270_e44797))), (2.0 * (((locals.var_pd_q1_dn5 * assign30270_e44797) - (locals.var_pd_q1 * (locals.var_pd_p1_dn5 + (locals.var_pd_temp1_dn5 / (2.0 * assign30270_e44796))))) / (assign30270_e44797 * assign30270_e44797))), (2.0 * (((locals.var_pd_q1_dn6 * assign30270_e44797) - (locals.var_pd_q1 * (locals.var_pd_p1_dn6 + (locals.var_pd_temp1_dn6 / (2.0 * assign30270_e44796))))) / (assign30270_e44797 * assign30270_e44797))), (2.0 * (((locals.var_pd_q1_dn7 * assign30270_e44797) - (locals.var_pd_q1 * (locals.var_pd_p1_dn7 + (locals.var_pd_temp1_dn7 / (2.0 * assign30270_e44796))))) / (assign30270_e44797 * assign30270_e44797))), (2.0 * (((locals.var_pd_q1_dn8 * assign30270_e44797) - (locals.var_pd_q1 * (locals.var_pd_p1_dn8 + (locals.var_pd_temp1_dn8 / (2.0 * assign30270_e44796))))) / (assign30270_e44797 * assign30270_e44797))), (2.0 * (((locals.var_pd_q1_dn9 * assign30270_e44797) - (locals.var_pd_q1 * (locals.var_pd_p1_dn9 + (locals.var_pd_temp1_dn9 / (2.0 * assign30270_e44796))))) / (assign30270_e44797 * assign30270_e44797))), (2.0 * (((locals.var_pd_q1_dn10 * assign30270_e44797) - (locals.var_pd_q1 * (locals.var_pd_p1_dn10 + (locals.var_pd_temp1_dn10 / (2.0 * assign30270_e44796))))) / (assign30270_e44797 * assign30270_e44797))), (2.0 * (((locals.var_pd_q1_dn11 * assign30270_e44797) - (locals.var_pd_q1 * (locals.var_pd_p1_dn11 + (locals.var_pd_temp1_dn11 / (2.0 * assign30270_e44796))))) / (assign30270_e44797 * assign30270_e44797))),)
    } else {
        (locals.var_pd_u, locals.var_pd_u_dn3, locals.var_pd_u_dn4, locals.var_pd_u_dn5, locals.var_pd_u_dn6, locals.var_pd_u_dn7, locals.var_pd_u_dn8, locals.var_pd_u_dn9, locals.var_pd_u_dn10, locals.var_pd_u_dn11,)
    }
};
        locals.var_pd_u = assign30270_e44801;
        locals.var_pd_u_dn3 = assign30270_e44801_d_n3;
        locals.var_pd_u_dn4 = assign30270_e44801_d_n4;
        locals.var_pd_u_dn5 = assign30270_e44801_d_n5;
        locals.var_pd_u_dn6 = assign30270_e44801_d_n6;
        locals.var_pd_u_dn7 = assign30270_e44801_d_n7;
        locals.var_pd_u_dn8 = assign30270_e44801_d_n8;
        locals.var_pd_u_dn9 = assign30270_e44801_d_n9;
        locals.var_pd_u_dn10 = assign30270_e44801_d_n10;
        locals.var_pd_u_dn11 = assign30270_e44801_d_n11;
        locals.var_pd_u_rv = 0.0;

        let (assign30280_e44815, assign30280_e44815_d_n3, assign30280_e44815_d_n4, assign30280_e44815_d_n5, assign30280_e44815_d_n6, assign30280_e44815_d_n7, assign30280_e44815_d_n8, assign30280_e44815_d_n9, assign30280_e44815_d_n10, assign30280_e44815_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard614 == 0.0)) && (locals.var_guard615 == 0.0)) {
        let assign30280_e44813: f64 = (locals.var_pd_x0 + locals.var_pd_u);
        (assign30280_e44813, (locals.var_pd_x0_dn3 + locals.var_pd_u_dn3), (locals.var_pd_x0_dn4 + locals.var_pd_u_dn4), (locals.var_pd_x0_dn5 + locals.var_pd_u_dn5), (locals.var_pd_x0_dn6 + locals.var_pd_u_dn6), (locals.var_pd_x0_dn7 + locals.var_pd_u_dn7), (locals.var_pd_x0_dn8 + locals.var_pd_u_dn8), (locals.var_pd_x0_dn9 + locals.var_pd_u_dn9), (locals.var_pd_x0_dn10 + locals.var_pd_u_dn10), (locals.var_pd_x0_dn11 + locals.var_pd_u_dn11),)
    } else {
        (locals.var_pd_sb, locals.var_pd_sb_dn3, locals.var_pd_sb_dn4, locals.var_pd_sb_dn5, locals.var_pd_sb_dn6, locals.var_pd_sb_dn7, locals.var_pd_sb_dn8, locals.var_pd_sb_dn9, locals.var_pd_sb_dn10, locals.var_pd_sb_dn11,)
    }
};
        locals.var_pd_sb = assign30280_e44815;
        locals.var_pd_sb_dn3 = assign30280_e44815_d_n3;
        locals.var_pd_sb_dn4 = assign30280_e44815_d_n4;
        locals.var_pd_sb_dn5 = assign30280_e44815_d_n5;
        locals.var_pd_sb_dn6 = assign30280_e44815_d_n6;
        locals.var_pd_sb_dn7 = assign30280_e44815_d_n7;
        locals.var_pd_sb_dn8 = assign30280_e44815_d_n8;
        locals.var_pd_sb_dn9 = assign30280_e44815_d_n9;
        locals.var_pd_sb_dn10 = assign30280_e44815_d_n10;
        locals.var_pd_sb_dn11 = assign30280_e44815_d_n11;
        locals.var_pd_sb_rv = 0.0;

        let assign30290_e44817: f64 = (locals.var_vgfbb).abs();
        let assign30290_e44819: f64 = if assign30290_e44817 < locals.var_limit_sb { 1.0 } else { 0.0 };
        locals.var_guard616 = assign30290_e44819;
        locals.var_guard616_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_88(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30300_e44846, assign30300_e44846_d_n3, assign30300_e44846_d_n4, assign30300_e44846_d_n5, assign30300_e44846_d_n6, assign30300_e44846_d_n7, assign30300_e44846_d_n8, assign30300_e44846_d_n9, assign30300_e44846_d_n10, assign30300_e44846_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard616 != 0.0)) {
        let assign30300_e44826: f64 = (-locals.var_vgfbb);
        let assign30300_e44828: f64 = (assign30300_e44826 * locals.var_inv_xi_sb);
        let assign30300_e44832: f64 = (-locals.var_vgfbb);
        let assign30300_e44835: f64 = (2.0_f64).sqrt();
        let assign30300_e44836: f64 = (6.0 * assign30300_e44835);
        let assign30300_e44838: f64 = (assign30300_e44836 * locals.var_x1_sb);
        let assign30300_e44840: f64 = (assign30300_e44838 * locals.var_x1_sb);
        let assign30300_e44841: f64 = (assign30300_e44832 / assign30300_e44840);
        let assign30300_e44842: f64 = (locals.var_gam_sb * assign30300_e44841);
        let assign30300_e44843: f64 = (1.0 + assign30300_e44842);
        let assign30300_e44844: f64 = (assign30300_e44828 * assign30300_e44843);
        (assign30300_e44844, (((((-locals.var_vgfbb_dn3) * locals.var_inv_xi_sb) + (assign30300_e44826 * locals.var_inv_xi_sb_dn3)) * assign30300_e44843) + (assign30300_e44828 * ((locals.var_gam_sb_dn3 * assign30300_e44841) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn3) * assign30300_e44840) - (assign30300_e44832 * (((assign30300_e44836 * locals.var_x1_sb_dn3) * locals.var_x1_sb) + (assign30300_e44838 * locals.var_x1_sb_dn3)))) / (assign30300_e44840 * assign30300_e44840)))))), (((((-locals.var_vgfbb_dn4) * locals.var_inv_xi_sb) + (assign30300_e44826 * locals.var_inv_xi_sb_dn4)) * assign30300_e44843) + (assign30300_e44828 * ((locals.var_gam_sb_dn4 * assign30300_e44841) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn4) * assign30300_e44840) - (assign30300_e44832 * (((assign30300_e44836 * locals.var_x1_sb_dn4) * locals.var_x1_sb) + (assign30300_e44838 * locals.var_x1_sb_dn4)))) / (assign30300_e44840 * assign30300_e44840)))))), (((((-locals.var_vgfbb_dn5) * locals.var_inv_xi_sb) + (assign30300_e44826 * locals.var_inv_xi_sb_dn5)) * assign30300_e44843) + (assign30300_e44828 * ((locals.var_gam_sb_dn5 * assign30300_e44841) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn5) * assign30300_e44840) - (assign30300_e44832 * (((assign30300_e44836 * locals.var_x1_sb_dn5) * locals.var_x1_sb) + (assign30300_e44838 * locals.var_x1_sb_dn5)))) / (assign30300_e44840 * assign30300_e44840)))))), (((((-locals.var_vgfbb_dn6) * locals.var_inv_xi_sb) + (assign30300_e44826 * locals.var_inv_xi_sb_dn6)) * assign30300_e44843) + (assign30300_e44828 * ((locals.var_gam_sb_dn6 * assign30300_e44841) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn6) * assign30300_e44840) - (assign30300_e44832 * (((assign30300_e44836 * locals.var_x1_sb_dn6) * locals.var_x1_sb) + (assign30300_e44838 * locals.var_x1_sb_dn6)))) / (assign30300_e44840 * assign30300_e44840)))))), (((((-locals.var_vgfbb_dn7) * locals.var_inv_xi_sb) + (assign30300_e44826 * locals.var_inv_xi_sb_dn7)) * assign30300_e44843) + (assign30300_e44828 * ((locals.var_gam_sb_dn7 * assign30300_e44841) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn7) * assign30300_e44840) - (assign30300_e44832 * (((assign30300_e44836 * locals.var_x1_sb_dn7) * locals.var_x1_sb) + (assign30300_e44838 * locals.var_x1_sb_dn7)))) / (assign30300_e44840 * assign30300_e44840)))))), (((((-locals.var_vgfbb_dn8) * locals.var_inv_xi_sb) + (assign30300_e44826 * locals.var_inv_xi_sb_dn8)) * assign30300_e44843) + (assign30300_e44828 * ((locals.var_gam_sb_dn8 * assign30300_e44841) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn8) * assign30300_e44840) - (assign30300_e44832 * (((assign30300_e44836 * locals.var_x1_sb_dn8) * locals.var_x1_sb) + (assign30300_e44838 * locals.var_x1_sb_dn8)))) / (assign30300_e44840 * assign30300_e44840)))))), (((((-locals.var_vgfbb_dn9) * locals.var_inv_xi_sb) + (assign30300_e44826 * locals.var_inv_xi_sb_dn9)) * assign30300_e44843) + (assign30300_e44828 * ((locals.var_gam_sb_dn9 * assign30300_e44841) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn9) * assign30300_e44840) - (assign30300_e44832 * (((assign30300_e44836 * locals.var_x1_sb_dn9) * locals.var_x1_sb) + (assign30300_e44838 * locals.var_x1_sb_dn9)))) / (assign30300_e44840 * assign30300_e44840)))))), (((((-locals.var_vgfbb_dn10) * locals.var_inv_xi_sb) + (assign30300_e44826 * locals.var_inv_xi_sb_dn10)) * assign30300_e44843) + (assign30300_e44828 * ((locals.var_gam_sb_dn10 * assign30300_e44841) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn10) * assign30300_e44840) - (assign30300_e44832 * (((assign30300_e44836 * locals.var_x1_sb_dn10) * locals.var_x1_sb) + (assign30300_e44838 * locals.var_x1_sb_dn10)))) / (assign30300_e44840 * assign30300_e44840)))))), (((((-locals.var_vgfbb_dn11) * locals.var_inv_xi_sb) + (assign30300_e44826 * locals.var_inv_xi_sb_dn11)) * assign30300_e44843) + (assign30300_e44828 * ((locals.var_gam_sb_dn11 * assign30300_e44841) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn11) * assign30300_e44840) - (assign30300_e44832 * (((assign30300_e44836 * locals.var_x1_sb_dn11) * locals.var_x1_sb) + (assign30300_e44838 * locals.var_x1_sb_dn11)))) / (assign30300_e44840 * assign30300_e44840)))))),)
    } else {
        (locals.var_phisb0, locals.var_phisb0_dn3, locals.var_phisb0_dn4, locals.var_phisb0_dn5, locals.var_phisb0_dn6, locals.var_phisb0_dn7, locals.var_phisb0_dn8, locals.var_phisb0_dn9, locals.var_phisb0_dn10, locals.var_phisb0_dn11,)
    }
};
        locals.var_phisb0 = assign30300_e44846;
        locals.var_phisb0_dn3 = assign30300_e44846_d_n3;
        locals.var_phisb0_dn4 = assign30300_e44846_d_n4;
        locals.var_phisb0_dn5 = assign30300_e44846_d_n5;
        locals.var_phisb0_dn6 = assign30300_e44846_d_n6;
        locals.var_phisb0_dn7 = assign30300_e44846_d_n7;
        locals.var_phisb0_dn8 = assign30300_e44846_d_n8;
        locals.var_phisb0_dn9 = assign30300_e44846_d_n9;
        locals.var_phisb0_dn10 = assign30300_e44846_d_n10;
        locals.var_phisb0_dn11 = assign30300_e44846_d_n11;
        locals.var_phisb0_rv = 0.0;

        let (assign30310_e44873, assign30310_e44873_d_n3, assign30310_e44873_d_n4, assign30310_e44873_d_n5, assign30310_e44873_d_n6, assign30310_e44873_d_n7, assign30310_e44873_d_n8, assign30310_e44873_d_n9, assign30310_e44873_d_n10, assign30310_e44873_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard616 != 0.0)) {
        let assign30310_e44853: f64 = (-locals.var_vgfbb);
        let assign30310_e44855: f64 = (assign30310_e44853 * locals.var_inv_xi_sb);
        let assign30310_e44859: f64 = (-locals.var_vgfbb);
        let assign30310_e44862: f64 = (2.0_f64).sqrt();
        let assign30310_e44863: f64 = (6.0 * assign30310_e44862);
        let assign30310_e44865: f64 = (assign30310_e44863 * locals.var_x1_sb);
        let assign30310_e44867: f64 = (assign30310_e44865 * locals.var_x1_sb);
        let assign30310_e44868: f64 = (assign30310_e44859 / assign30310_e44867);
        let assign30310_e44869: f64 = (locals.var_gam_sb * assign30310_e44868);
        let assign30310_e44870: f64 = (1.0 + assign30310_e44869);
        let assign30310_e44871: f64 = (assign30310_e44855 * assign30310_e44870);
        (assign30310_e44871, (((((-locals.var_vgfbb_dn3) * locals.var_inv_xi_sb) + (assign30310_e44853 * locals.var_inv_xi_sb_dn3)) * assign30310_e44870) + (assign30310_e44855 * ((locals.var_gam_sb_dn3 * assign30310_e44868) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn3) * assign30310_e44867) - (assign30310_e44859 * (((assign30310_e44863 * locals.var_x1_sb_dn3) * locals.var_x1_sb) + (assign30310_e44865 * locals.var_x1_sb_dn3)))) / (assign30310_e44867 * assign30310_e44867)))))), (((((-locals.var_vgfbb_dn4) * locals.var_inv_xi_sb) + (assign30310_e44853 * locals.var_inv_xi_sb_dn4)) * assign30310_e44870) + (assign30310_e44855 * ((locals.var_gam_sb_dn4 * assign30310_e44868) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn4) * assign30310_e44867) - (assign30310_e44859 * (((assign30310_e44863 * locals.var_x1_sb_dn4) * locals.var_x1_sb) + (assign30310_e44865 * locals.var_x1_sb_dn4)))) / (assign30310_e44867 * assign30310_e44867)))))), (((((-locals.var_vgfbb_dn5) * locals.var_inv_xi_sb) + (assign30310_e44853 * locals.var_inv_xi_sb_dn5)) * assign30310_e44870) + (assign30310_e44855 * ((locals.var_gam_sb_dn5 * assign30310_e44868) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn5) * assign30310_e44867) - (assign30310_e44859 * (((assign30310_e44863 * locals.var_x1_sb_dn5) * locals.var_x1_sb) + (assign30310_e44865 * locals.var_x1_sb_dn5)))) / (assign30310_e44867 * assign30310_e44867)))))), (((((-locals.var_vgfbb_dn6) * locals.var_inv_xi_sb) + (assign30310_e44853 * locals.var_inv_xi_sb_dn6)) * assign30310_e44870) + (assign30310_e44855 * ((locals.var_gam_sb_dn6 * assign30310_e44868) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn6) * assign30310_e44867) - (assign30310_e44859 * (((assign30310_e44863 * locals.var_x1_sb_dn6) * locals.var_x1_sb) + (assign30310_e44865 * locals.var_x1_sb_dn6)))) / (assign30310_e44867 * assign30310_e44867)))))), (((((-locals.var_vgfbb_dn7) * locals.var_inv_xi_sb) + (assign30310_e44853 * locals.var_inv_xi_sb_dn7)) * assign30310_e44870) + (assign30310_e44855 * ((locals.var_gam_sb_dn7 * assign30310_e44868) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn7) * assign30310_e44867) - (assign30310_e44859 * (((assign30310_e44863 * locals.var_x1_sb_dn7) * locals.var_x1_sb) + (assign30310_e44865 * locals.var_x1_sb_dn7)))) / (assign30310_e44867 * assign30310_e44867)))))), (((((-locals.var_vgfbb_dn8) * locals.var_inv_xi_sb) + (assign30310_e44853 * locals.var_inv_xi_sb_dn8)) * assign30310_e44870) + (assign30310_e44855 * ((locals.var_gam_sb_dn8 * assign30310_e44868) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn8) * assign30310_e44867) - (assign30310_e44859 * (((assign30310_e44863 * locals.var_x1_sb_dn8) * locals.var_x1_sb) + (assign30310_e44865 * locals.var_x1_sb_dn8)))) / (assign30310_e44867 * assign30310_e44867)))))), (((((-locals.var_vgfbb_dn9) * locals.var_inv_xi_sb) + (assign30310_e44853 * locals.var_inv_xi_sb_dn9)) * assign30310_e44870) + (assign30310_e44855 * ((locals.var_gam_sb_dn9 * assign30310_e44868) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn9) * assign30310_e44867) - (assign30310_e44859 * (((assign30310_e44863 * locals.var_x1_sb_dn9) * locals.var_x1_sb) + (assign30310_e44865 * locals.var_x1_sb_dn9)))) / (assign30310_e44867 * assign30310_e44867)))))), (((((-locals.var_vgfbb_dn10) * locals.var_inv_xi_sb) + (assign30310_e44853 * locals.var_inv_xi_sb_dn10)) * assign30310_e44870) + (assign30310_e44855 * ((locals.var_gam_sb_dn10 * assign30310_e44868) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn10) * assign30310_e44867) - (assign30310_e44859 * (((assign30310_e44863 * locals.var_x1_sb_dn10) * locals.var_x1_sb) + (assign30310_e44865 * locals.var_x1_sb_dn10)))) / (assign30310_e44867 * assign30310_e44867)))))), (((((-locals.var_vgfbb_dn11) * locals.var_inv_xi_sb) + (assign30310_e44853 * locals.var_inv_xi_sb_dn11)) * assign30310_e44870) + (assign30310_e44855 * ((locals.var_gam_sb_dn11 * assign30310_e44868) + (locals.var_gam_sb * ((((-locals.var_vgfbb_dn11) * assign30310_e44867) - (assign30310_e44859 * (((assign30310_e44863 * locals.var_x1_sb_dn11) * locals.var_x1_sb) + (assign30310_e44865 * locals.var_x1_sb_dn11)))) / (assign30310_e44867 * assign30310_e44867)))))),)
    } else {
        (locals.var_pd_sb, locals.var_pd_sb_dn3, locals.var_pd_sb_dn4, locals.var_pd_sb_dn5, locals.var_pd_sb_dn6, locals.var_pd_sb_dn7, locals.var_pd_sb_dn8, locals.var_pd_sb_dn9, locals.var_pd_sb_dn10, locals.var_pd_sb_dn11,)
    }
};
        locals.var_pd_sb = assign30310_e44873;
        locals.var_pd_sb_dn3 = assign30310_e44873_d_n3;
        locals.var_pd_sb_dn4 = assign30310_e44873_d_n4;
        locals.var_pd_sb_dn5 = assign30310_e44873_d_n5;
        locals.var_pd_sb_dn6 = assign30310_e44873_d_n6;
        locals.var_pd_sb_dn7 = assign30310_e44873_d_n7;
        locals.var_pd_sb_dn8 = assign30310_e44873_d_n8;
        locals.var_pd_sb_dn9 = assign30310_e44873_d_n9;
        locals.var_pd_sb_dn10 = assign30310_e44873_d_n10;
        locals.var_pd_sb_dn11 = assign30310_e44873_d_n11;
        locals.var_pd_sb_rv = 0.0;

        let (assign30320_e44908, assign30320_e44908_d_n3, assign30320_e44908_d_n4, assign30320_e44908_d_n5, assign30320_e44908_d_n6, assign30320_e44908_d_n7, assign30320_e44908_d_n8, assign30320_e44908_d_n9, assign30320_e44908_d_n10, assign30320_e44908_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard616 == 0.0)) {
        let assign30320_e44882: f64 = (locals.var_rt * locals.var_rt);
        let assign30320_e44885: f64 = (locals.var_vgfbb - locals.var_pd_sb);
        let assign30320_e44886: f64 = (assign30320_e44882 * assign30320_e44885);
        let assign30320_e44889: f64 = (locals.var_vgfbb - locals.var_pd_sb);
        let assign30320_e44890: f64 = (assign30320_e44886 * assign30320_e44889);
        let assign30320_e44893: f64 = (1.0 / locals.var_gam);
        let assign30320_e44894: f64 = (assign30320_e44890 * assign30320_e44893);
        let assign30320_e44897: f64 = (1.0 / locals.var_gam);
        let assign30320_e44898: f64 = (assign30320_e44894 * assign30320_e44897);
        let assign30320_e44900: f64 = (-locals.var_pd_sb);
        let assign30320_e44901: f64 = { let limited_exp_arg = assign30320_e44900; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30320_e44903: f64 = (assign30320_e44901 + locals.var_pd_sb);
        let assign30320_e44905: f64 = (assign30320_e44903 - 1.0);
        let assign30320_e44906: f64 = (assign30320_e44898 - assign30320_e44905);
        (assign30320_e44906, ((((((((assign30320_e44882 * (locals.var_vgfbb_dn3 - locals.var_pd_sb_dn3)) * assign30320_e44889) + (assign30320_e44886 * (locals.var_vgfbb_dn3 - locals.var_pd_sb_dn3))) * assign30320_e44893) + (assign30320_e44890 * (-(locals.var_gam_dn3 / (locals.var_gam * locals.var_gam))))) * assign30320_e44897) + (assign30320_e44894 * (-(locals.var_gam_dn3 / (locals.var_gam * locals.var_gam))))) - (({ let limited_exp_arg = assign30320_e44900; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn3)) + locals.var_pd_sb_dn3)), ((((((((assign30320_e44882 * (locals.var_vgfbb_dn4 - locals.var_pd_sb_dn4)) * assign30320_e44889) + (assign30320_e44886 * (locals.var_vgfbb_dn4 - locals.var_pd_sb_dn4))) * assign30320_e44893) + (assign30320_e44890 * (-(locals.var_gam_dn4 / (locals.var_gam * locals.var_gam))))) * assign30320_e44897) + (assign30320_e44894 * (-(locals.var_gam_dn4 / (locals.var_gam * locals.var_gam))))) - (({ let limited_exp_arg = assign30320_e44900; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn4)) + locals.var_pd_sb_dn4)), ((((((((assign30320_e44882 * (locals.var_vgfbb_dn5 - locals.var_pd_sb_dn5)) * assign30320_e44889) + (assign30320_e44886 * (locals.var_vgfbb_dn5 - locals.var_pd_sb_dn5))) * assign30320_e44893) + (assign30320_e44890 * (-(locals.var_gam_dn5 / (locals.var_gam * locals.var_gam))))) * assign30320_e44897) + (assign30320_e44894 * (-(locals.var_gam_dn5 / (locals.var_gam * locals.var_gam))))) - (({ let limited_exp_arg = assign30320_e44900; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn5)) + locals.var_pd_sb_dn5)), ((((((((assign30320_e44882 * (locals.var_vgfbb_dn6 - locals.var_pd_sb_dn6)) * assign30320_e44889) + (assign30320_e44886 * (locals.var_vgfbb_dn6 - locals.var_pd_sb_dn6))) * assign30320_e44893) + (assign30320_e44890 * (-(locals.var_gam_dn6 / (locals.var_gam * locals.var_gam))))) * assign30320_e44897) + (assign30320_e44894 * (-(locals.var_gam_dn6 / (locals.var_gam * locals.var_gam))))) - (({ let limited_exp_arg = assign30320_e44900; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn6)) + locals.var_pd_sb_dn6)), ((((((((assign30320_e44882 * (locals.var_vgfbb_dn7 - locals.var_pd_sb_dn7)) * assign30320_e44889) + (assign30320_e44886 * (locals.var_vgfbb_dn7 - locals.var_pd_sb_dn7))) * assign30320_e44893) + (assign30320_e44890 * (-(locals.var_gam_dn7 / (locals.var_gam * locals.var_gam))))) * assign30320_e44897) + (assign30320_e44894 * (-(locals.var_gam_dn7 / (locals.var_gam * locals.var_gam))))) - (({ let limited_exp_arg = assign30320_e44900; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn7)) + locals.var_pd_sb_dn7)), ((((((((assign30320_e44882 * (locals.var_vgfbb_dn8 - locals.var_pd_sb_dn8)) * assign30320_e44889) + (assign30320_e44886 * (locals.var_vgfbb_dn8 - locals.var_pd_sb_dn8))) * assign30320_e44893) + (assign30320_e44890 * (-(locals.var_gam_dn8 / (locals.var_gam * locals.var_gam))))) * assign30320_e44897) + (assign30320_e44894 * (-(locals.var_gam_dn8 / (locals.var_gam * locals.var_gam))))) - (({ let limited_exp_arg = assign30320_e44900; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn8)) + locals.var_pd_sb_dn8)), ((((((((assign30320_e44882 * (locals.var_vgfbb_dn9 - locals.var_pd_sb_dn9)) * assign30320_e44889) + (assign30320_e44886 * (locals.var_vgfbb_dn9 - locals.var_pd_sb_dn9))) * assign30320_e44893) + (assign30320_e44890 * (-(locals.var_gam_dn9 / (locals.var_gam * locals.var_gam))))) * assign30320_e44897) + (assign30320_e44894 * (-(locals.var_gam_dn9 / (locals.var_gam * locals.var_gam))))) - (({ let limited_exp_arg = assign30320_e44900; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn9)) + locals.var_pd_sb_dn9)), ((((((((assign30320_e44882 * (locals.var_vgfbb_dn10 - locals.var_pd_sb_dn10)) * assign30320_e44889) + (assign30320_e44886 * (locals.var_vgfbb_dn10 - locals.var_pd_sb_dn10))) * assign30320_e44893) + (assign30320_e44890 * (-(locals.var_gam_dn10 / (locals.var_gam * locals.var_gam))))) * assign30320_e44897) + (assign30320_e44894 * (-(locals.var_gam_dn10 / (locals.var_gam * locals.var_gam))))) - (({ let limited_exp_arg = assign30320_e44900; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn10)) + locals.var_pd_sb_dn10)), ((((((((assign30320_e44882 * (locals.var_vgfbb_dn11 - locals.var_pd_sb_dn11)) * assign30320_e44889) + (assign30320_e44886 * (locals.var_vgfbb_dn11 - locals.var_pd_sb_dn11))) * assign30320_e44893) + (assign30320_e44890 * (-(locals.var_gam_dn11 / (locals.var_gam * locals.var_gam))))) * assign30320_e44897) + (assign30320_e44894 * (-(locals.var_gam_dn11 / (locals.var_gam * locals.var_gam))))) - (({ let limited_exp_arg = assign30320_e44900; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn11)) + locals.var_pd_sb_dn11)),)
    } else {
        (locals.var_fx, locals.var_fx_dn3, locals.var_fx_dn4, locals.var_fx_dn5, locals.var_fx_dn6, locals.var_fx_dn7, locals.var_fx_dn8, locals.var_fx_dn9, locals.var_fx_dn10, locals.var_fx_dn11,)
    }
};
        locals.var_fx = assign30320_e44908;
        locals.var_fx_dn3 = assign30320_e44908_d_n3;
        locals.var_fx_dn4 = assign30320_e44908_d_n4;
        locals.var_fx_dn5 = assign30320_e44908_d_n5;
        locals.var_fx_dn6 = assign30320_e44908_d_n6;
        locals.var_fx_dn7 = assign30320_e44908_d_n7;
        locals.var_fx_dn8 = assign30320_e44908_d_n8;
        locals.var_fx_dn9 = assign30320_e44908_d_n9;
        locals.var_fx_dn10 = assign30320_e44908_d_n10;
        locals.var_fx_dn11 = assign30320_e44908_d_n11;
        locals.var_fx_rv = 0.0;

        let (assign30330_e44937, assign30330_e44937_d_n3, assign30330_e44937_d_n4, assign30330_e44937_d_n5, assign30330_e44937_d_n6, assign30330_e44937_d_n7, assign30330_e44937_d_n8, assign30330_e44937_d_n9, assign30330_e44937_d_n10, assign30330_e44937_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard616 == 0.0)) {
        let assign30330_e44916: f64 = (-locals.var_pd_sb);
        let assign30330_e44917: f64 = { let limited_exp_arg = assign30330_e44916; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30330_e44920: f64 = (locals.var_rt * locals.var_rt);
        let assign30330_e44923: f64 = (2.0 * locals.var_pd_sb);
        let assign30330_e44926: f64 = (2.0 * locals.var_vgfbb);
        let assign30330_e44927: f64 = (assign30330_e44923 - assign30330_e44926);
        let assign30330_e44928: f64 = (assign30330_e44920 * assign30330_e44927);
        let assign30330_e44931: f64 = (locals.var_gam * locals.var_gam);
        let assign30330_e44932: f64 = (assign30330_e44928 / assign30330_e44931);
        let assign30330_e44933: f64 = (assign30330_e44917 + assign30330_e44932);
        let assign30330_e44935: f64 = (assign30330_e44933 - 1.0);
        (assign30330_e44935, (({ let limited_exp_arg = assign30330_e44916; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn3)) + ((((assign30330_e44920 * ((2.0 * locals.var_pd_sb_dn3) - (2.0 * locals.var_vgfbb_dn3))) * assign30330_e44931) - (assign30330_e44928 * ((locals.var_gam_dn3 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn3)))) / (assign30330_e44931 * assign30330_e44931))), (({ let limited_exp_arg = assign30330_e44916; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn4)) + ((((assign30330_e44920 * ((2.0 * locals.var_pd_sb_dn4) - (2.0 * locals.var_vgfbb_dn4))) * assign30330_e44931) - (assign30330_e44928 * ((locals.var_gam_dn4 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn4)))) / (assign30330_e44931 * assign30330_e44931))), (({ let limited_exp_arg = assign30330_e44916; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn5)) + ((((assign30330_e44920 * ((2.0 * locals.var_pd_sb_dn5) - (2.0 * locals.var_vgfbb_dn5))) * assign30330_e44931) - (assign30330_e44928 * ((locals.var_gam_dn5 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn5)))) / (assign30330_e44931 * assign30330_e44931))), (({ let limited_exp_arg = assign30330_e44916; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn6)) + ((((assign30330_e44920 * ((2.0 * locals.var_pd_sb_dn6) - (2.0 * locals.var_vgfbb_dn6))) * assign30330_e44931) - (assign30330_e44928 * ((locals.var_gam_dn6 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn6)))) / (assign30330_e44931 * assign30330_e44931))), (({ let limited_exp_arg = assign30330_e44916; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn7)) + ((((assign30330_e44920 * ((2.0 * locals.var_pd_sb_dn7) - (2.0 * locals.var_vgfbb_dn7))) * assign30330_e44931) - (assign30330_e44928 * ((locals.var_gam_dn7 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn7)))) / (assign30330_e44931 * assign30330_e44931))), (({ let limited_exp_arg = assign30330_e44916; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn8)) + ((((assign30330_e44920 * ((2.0 * locals.var_pd_sb_dn8) - (2.0 * locals.var_vgfbb_dn8))) * assign30330_e44931) - (assign30330_e44928 * ((locals.var_gam_dn8 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn8)))) / (assign30330_e44931 * assign30330_e44931))), (({ let limited_exp_arg = assign30330_e44916; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn9)) + ((((assign30330_e44920 * ((2.0 * locals.var_pd_sb_dn9) - (2.0 * locals.var_vgfbb_dn9))) * assign30330_e44931) - (assign30330_e44928 * ((locals.var_gam_dn9 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn9)))) / (assign30330_e44931 * assign30330_e44931))), (({ let limited_exp_arg = assign30330_e44916; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn10)) + ((((assign30330_e44920 * ((2.0 * locals.var_pd_sb_dn10) - (2.0 * locals.var_vgfbb_dn10))) * assign30330_e44931) - (assign30330_e44928 * ((locals.var_gam_dn10 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn10)))) / (assign30330_e44931 * assign30330_e44931))), (({ let limited_exp_arg = assign30330_e44916; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_pd_sb_dn11)) + ((((assign30330_e44920 * ((2.0 * locals.var_pd_sb_dn11) - (2.0 * locals.var_vgfbb_dn11))) * assign30330_e44931) - (assign30330_e44928 * ((locals.var_gam_dn11 * locals.var_gam) + (locals.var_gam * locals.var_gam_dn11)))) / (assign30330_e44931 * assign30330_e44931))),)
    } else {
        (locals.var_f_x, locals.var_f_x_dn3, locals.var_f_x_dn4, locals.var_f_x_dn5, locals.var_f_x_dn6, locals.var_f_x_dn7, locals.var_f_x_dn8, locals.var_f_x_dn9, locals.var_f_x_dn10, locals.var_f_x_dn11,)
    }
};
        locals.var_f_x = assign30330_e44937;
        locals.var_f_x_dn3 = assign30330_e44937_d_n3;
        locals.var_f_x_dn4 = assign30330_e44937_d_n4;
        locals.var_f_x_dn5 = assign30330_e44937_d_n5;
        locals.var_f_x_dn6 = assign30330_e44937_d_n6;
        locals.var_f_x_dn7 = assign30330_e44937_d_n7;
        locals.var_f_x_dn8 = assign30330_e44937_d_n8;
        locals.var_f_x_dn9 = assign30330_e44937_d_n9;
        locals.var_f_x_dn10 = assign30330_e44937_d_n10;
        locals.var_f_x_dn11 = assign30330_e44937_d_n11;
        locals.var_f_x_rv = 0.0;

        let (assign30340_e44950, assign30340_e44950_d_n3, assign30340_e44950_d_n4, assign30340_e44950_d_n5, assign30340_e44950_d_n6, assign30340_e44950_d_n7, assign30340_e44950_d_n8, assign30340_e44950_d_n9, assign30340_e44950_d_n10, assign30340_e44950_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard616 == 0.0)) {
        let assign30340_e44947: f64 = (locals.var_fx / locals.var_f_x);
        let assign30340_e44948: f64 = (locals.var_pd_sb - assign30340_e44947);
        (assign30340_e44948, (locals.var_pd_sb_dn3 - (((locals.var_fx_dn3 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn3)) / (locals.var_f_x * locals.var_f_x))), (locals.var_pd_sb_dn4 - (((locals.var_fx_dn4 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn4)) / (locals.var_f_x * locals.var_f_x))), (locals.var_pd_sb_dn5 - (((locals.var_fx_dn5 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn5)) / (locals.var_f_x * locals.var_f_x))), (locals.var_pd_sb_dn6 - (((locals.var_fx_dn6 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn6)) / (locals.var_f_x * locals.var_f_x))), (locals.var_pd_sb_dn7 - (((locals.var_fx_dn7 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn7)) / (locals.var_f_x * locals.var_f_x))), (locals.var_pd_sb_dn8 - (((locals.var_fx_dn8 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn8)) / (locals.var_f_x * locals.var_f_x))), (locals.var_pd_sb_dn9 - (((locals.var_fx_dn9 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn9)) / (locals.var_f_x * locals.var_f_x))), (locals.var_pd_sb_dn10 - (((locals.var_fx_dn10 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn10)) / (locals.var_f_x * locals.var_f_x))), (locals.var_pd_sb_dn11 - (((locals.var_fx_dn11 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn11)) / (locals.var_f_x * locals.var_f_x))),)
    } else {
        (locals.var_phisb0, locals.var_phisb0_dn3, locals.var_phisb0_dn4, locals.var_phisb0_dn5, locals.var_phisb0_dn6, locals.var_phisb0_dn7, locals.var_phisb0_dn8, locals.var_phisb0_dn9, locals.var_phisb0_dn10, locals.var_phisb0_dn11,)
    }
};
        locals.var_phisb0 = assign30340_e44950;
        locals.var_phisb0_dn3 = assign30340_e44950_d_n3;
        locals.var_phisb0_dn4 = assign30340_e44950_d_n4;
        locals.var_phisb0_dn5 = assign30340_e44950_d_n5;
        locals.var_phisb0_dn6 = assign30340_e44950_d_n6;
        locals.var_phisb0_dn7 = assign30340_e44950_d_n7;
        locals.var_phisb0_dn8 = assign30340_e44950_d_n8;
        locals.var_phisb0_dn9 = assign30340_e44950_d_n9;
        locals.var_phisb0_dn10 = assign30340_e44950_d_n10;
        locals.var_phisb0_dn11 = assign30340_e44950_d_n11;
        locals.var_phisb0_rv = 0.0;

        let (assign30350_e44958, assign30350_e44958_d_n3, assign30350_e44958_d_n4, assign30350_e44958_d_n5, assign30350_e44958_d_n6, assign30350_e44958_d_n7, assign30350_e44958_d_n8, assign30350_e44958_d_n9, assign30350_e44958_d_n10, assign30350_e44958_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign30350_e44956: f64 = (locals.var_phisb0 * locals.var_vt);
        (assign30350_e44956, (locals.var_phisb0_dn3 * locals.var_vt), ((locals.var_phisb0_dn4 * locals.var_vt) + (locals.var_phisb0 * locals.var_vt_dn4)), ((locals.var_phisb0_dn5 * locals.var_vt) + (locals.var_phisb0 * locals.var_vt_dn5)), (locals.var_phisb0_dn6 * locals.var_vt), (locals.var_phisb0_dn7 * locals.var_vt), (locals.var_phisb0_dn8 * locals.var_vt), (locals.var_phisb0_dn9 * locals.var_vt), (locals.var_phisb0_dn10 * locals.var_vt), (locals.var_phisb0_dn11 * locals.var_vt),)
    } else {
        (locals.var_phisb0, locals.var_phisb0_dn3, locals.var_phisb0_dn4, locals.var_phisb0_dn5, locals.var_phisb0_dn6, locals.var_phisb0_dn7, locals.var_phisb0_dn8, locals.var_phisb0_dn9, locals.var_phisb0_dn10, locals.var_phisb0_dn11,)
    }
};
        locals.var_phisb0 = assign30350_e44958;
        locals.var_phisb0_dn3 = assign30350_e44958_d_n3;
        locals.var_phisb0_dn4 = assign30350_e44958_d_n4;
        locals.var_phisb0_dn5 = assign30350_e44958_d_n5;
        locals.var_phisb0_dn6 = assign30350_e44958_d_n6;
        locals.var_phisb0_dn7 = assign30350_e44958_d_n7;
        locals.var_phisb0_dn8 = assign30350_e44958_d_n8;
        locals.var_phisb0_dn9 = assign30350_e44958_d_n9;
        locals.var_phisb0_dn10 = assign30350_e44958_d_n10;
        locals.var_phisb0_dn11 = assign30350_e44958_d_n11;
        locals.var_phisb0_rv = 0.0;

        let (assign30360_e44968, assign30360_e44968_d_n3, assign30360_e44968_d_n4, assign30360_e44968_d_n5, assign30360_e44968_d_n6, assign30360_e44968_d_n7, assign30360_e44968_d_n8, assign30360_e44968_d_n9, assign30360_e44968_d_n10, assign30360_e44968_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign30360_e44965: f64 = (locals.var_gam * 0.7071067811865475);
        let assign30360_e44966: f64 = (1.0 + assign30360_e44965);
        (assign30360_e44966, (locals.var_gam_dn3 * 0.7071067811865475), (locals.var_gam_dn4 * 0.7071067811865475), (locals.var_gam_dn5 * 0.7071067811865475), (locals.var_gam_dn6 * 0.7071067811865475), (locals.var_gam_dn7 * 0.7071067811865475), (locals.var_gam_dn8 * 0.7071067811865475), (locals.var_gam_dn9 * 0.7071067811865475), (locals.var_gam_dn10 * 0.7071067811865475), (locals.var_gam_dn11 * 0.7071067811865475),)
    } else {
        (locals.var_x1, locals.var_x1_dn3, locals.var_x1_dn4, locals.var_x1_dn5, locals.var_x1_dn6, locals.var_x1_dn7, locals.var_x1_dn8, locals.var_x1_dn9, locals.var_x1_dn10, locals.var_x1_dn11,)
    }
};
        locals.var_x1 = assign30360_e44968;
        locals.var_x1_dn3 = assign30360_e44968_d_n3;
        locals.var_x1_dn4 = assign30360_e44968_d_n4;
        locals.var_x1_dn5 = assign30360_e44968_d_n5;
        locals.var_x1_dn6 = assign30360_e44968_d_n6;
        locals.var_x1_dn7 = assign30360_e44968_d_n7;
        locals.var_x1_dn8 = assign30360_e44968_d_n8;
        locals.var_x1_dn9 = assign30360_e44968_d_n9;
        locals.var_x1_dn10 = assign30360_e44968_d_n10;
        locals.var_x1_dn11 = assign30360_e44968_d_n11;
        locals.var_x1_rv = 0.0;

        let (assign30370_e44976, assign30370_e44976_d_n3, assign30370_e44976_d_n4, assign30370_e44976_d_n5, assign30370_e44976_d_n6, assign30370_e44976_d_n7, assign30370_e44976_d_n8, assign30370_e44976_d_n9, assign30370_e44976_d_n10, assign30370_e44976_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign30370_e44974: f64 = (1.0 / locals.var_x1);
        (assign30370_e44974, (-(locals.var_x1_dn3 / (locals.var_x1 * locals.var_x1))), (-(locals.var_x1_dn4 / (locals.var_x1 * locals.var_x1))), (-(locals.var_x1_dn5 / (locals.var_x1 * locals.var_x1))), (-(locals.var_x1_dn6 / (locals.var_x1 * locals.var_x1))), (-(locals.var_x1_dn7 / (locals.var_x1 * locals.var_x1))), (-(locals.var_x1_dn8 / (locals.var_x1 * locals.var_x1))), (-(locals.var_x1_dn9 / (locals.var_x1 * locals.var_x1))), (-(locals.var_x1_dn10 / (locals.var_x1 * locals.var_x1))), (-(locals.var_x1_dn11 / (locals.var_x1 * locals.var_x1))),)
    } else {
        (locals.var_inv_x1, locals.var_inv_x1_dn3, locals.var_inv_x1_dn4, locals.var_inv_x1_dn5, locals.var_inv_x1_dn6, locals.var_inv_x1_dn7, locals.var_inv_x1_dn8, locals.var_inv_x1_dn9, locals.var_inv_x1_dn10, locals.var_inv_x1_dn11,)
    }
};
        locals.var_inv_x1 = assign30370_e44976;
        locals.var_inv_x1_dn3 = assign30370_e44976_d_n3;
        locals.var_inv_x1_dn4 = assign30370_e44976_d_n4;
        locals.var_inv_x1_dn5 = assign30370_e44976_d_n5;
        locals.var_inv_x1_dn6 = assign30370_e44976_d_n6;
        locals.var_inv_x1_dn7 = assign30370_e44976_d_n7;
        locals.var_inv_x1_dn8 = assign30370_e44976_d_n8;
        locals.var_inv_x1_dn9 = assign30370_e44976_d_n9;
        locals.var_inv_x1_dn10 = assign30370_e44976_d_n10;
        locals.var_inv_x1_dn11 = assign30370_e44976_d_n11;
        locals.var_inv_x1_rv = 0.0;

        let (assign30380_e44988, assign30380_e44988_d_n3, assign30380_e44988_d_n4, assign30380_e44988_d_n5, assign30380_e44988_d_n6, assign30380_e44988_d_n7, assign30380_e44988_d_n8, assign30380_e44988_d_n9, assign30380_e44988_d_n10, assign30380_e44988_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign30380_e44982: f64 = (2.0 * locals.var_phib);
        let assign30380_e44984: f64 = (assign30380_e44982 / locals.var_n);
        let assign30380_e44986: f64 = (assign30380_e44984 + locals.var_vs1_1);
        (assign30380_e44986, ((((2.0 * locals.var_phib_dn3) * locals.var_n) - (assign30380_e44982 * locals.var_n_dn3)) / (locals.var_n * locals.var_n)), (((((2.0 * locals.var_phib_dn4) * locals.var_n) - (assign30380_e44982 * locals.var_n_dn4)) / (locals.var_n * locals.var_n)) + locals.var_vs1_1_dn4), (((((2.0 * locals.var_phib_dn5) * locals.var_n) - (assign30380_e44982 * locals.var_n_dn5)) / (locals.var_n * locals.var_n)) + locals.var_vs1_1_dn5), (((((2.0 * locals.var_phib_dn6) * locals.var_n) - (assign30380_e44982 * locals.var_n_dn6)) / (locals.var_n * locals.var_n)) + locals.var_vs1_1_dn6), (((((2.0 * locals.var_phib_dn7) * locals.var_n) - (assign30380_e44982 * locals.var_n_dn7)) / (locals.var_n * locals.var_n)) + locals.var_vs1_1_dn7), ((((2.0 * locals.var_phib_dn8) * locals.var_n) - (assign30380_e44982 * locals.var_n_dn8)) / (locals.var_n * locals.var_n)), ((((2.0 * locals.var_phib_dn9) * locals.var_n) - (assign30380_e44982 * locals.var_n_dn9)) / (locals.var_n * locals.var_n)), ((((2.0 * locals.var_phib_dn10) * locals.var_n) - (assign30380_e44982 * locals.var_n_dn10)) / (locals.var_n * locals.var_n)), (((((2.0 * locals.var_phib_dn11) * locals.var_n) - (assign30380_e44982 * locals.var_n_dn11)) / (locals.var_n * locals.var_n)) + locals.var_vs1_1_dn11),)
    } else {
        (locals.var_phisf, locals.var_phisf_dn3, locals.var_phisf_dn4, locals.var_phisf_dn5, locals.var_phisf_dn6, locals.var_phisf_dn7, locals.var_phisf_dn8, locals.var_phisf_dn9, locals.var_phisf_dn10, locals.var_phisf_dn11,)
    }
};
        locals.var_phisf = assign30380_e44988;
        locals.var_phisf_dn3 = assign30380_e44988_d_n3;
        locals.var_phisf_dn4 = assign30380_e44988_d_n4;
        locals.var_phisf_dn5 = assign30380_e44988_d_n5;
        locals.var_phisf_dn6 = assign30380_e44988_d_n6;
        locals.var_phisf_dn7 = assign30380_e44988_d_n7;
        locals.var_phisf_dn8 = assign30380_e44988_d_n8;
        locals.var_phisf_dn9 = assign30380_e44988_d_n9;
        locals.var_phisf_dn10 = assign30380_e44988_d_n10;
        locals.var_phisf_dn11 = assign30380_e44988_d_n11;
        locals.var_phisf_rv = 0.0;

        let (assign30390_e44996, assign30390_e44996_d_n3, assign30390_e44996_d_n4, assign30390_e44996_d_n5, assign30390_e44996_d_n6, assign30390_e44996_d_n7, assign30390_e44996_d_n8, assign30390_e44996_d_n9, assign30390_e44996_d_n10, assign30390_e44996_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign30390_e44993: f64 = (-locals.var_phisf);
        let assign30390_e44994: f64 = { let limited_exp_arg = assign30390_e44993; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign30390_e44994, ({ let limited_exp_arg = assign30390_e44993; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phisf_dn3)), ({ let limited_exp_arg = assign30390_e44993; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phisf_dn4)), ({ let limited_exp_arg = assign30390_e44993; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phisf_dn5)), ({ let limited_exp_arg = assign30390_e44993; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phisf_dn6)), ({ let limited_exp_arg = assign30390_e44993; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phisf_dn7)), ({ let limited_exp_arg = assign30390_e44993; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phisf_dn8)), ({ let limited_exp_arg = assign30390_e44993; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phisf_dn9)), ({ let limited_exp_arg = assign30390_e44993; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phisf_dn10)), ({ let limited_exp_arg = assign30390_e44993; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phisf_dn11)),)
    } else {
        (locals.var_exp_ns, locals.var_exp_ns_dn3, locals.var_exp_ns_dn4, locals.var_exp_ns_dn5, locals.var_exp_ns_dn6, locals.var_exp_ns_dn7, locals.var_exp_ns_dn8, locals.var_exp_ns_dn9, locals.var_exp_ns_dn10, locals.var_exp_ns_dn11,)
    }
};
        locals.var_exp_ns = assign30390_e44996;
        locals.var_exp_ns_dn3 = assign30390_e44996_d_n3;
        locals.var_exp_ns_dn4 = assign30390_e44996_d_n4;
        locals.var_exp_ns_dn5 = assign30390_e44996_d_n5;
        locals.var_exp_ns_dn6 = assign30390_e44996_d_n6;
        locals.var_exp_ns_dn7 = assign30390_e44996_d_n7;
        locals.var_exp_ns_dn8 = assign30390_e44996_d_n8;
        locals.var_exp_ns_dn9 = assign30390_e44996_d_n9;
        locals.var_exp_ns_dn10 = assign30390_e44996_d_n10;
        locals.var_exp_ns_dn11 = assign30390_e44996_d_n11;
        locals.var_exp_ns_rv = 0.0;

        let (assign30400_e45004, assign30400_e45004_d_n3, assign30400_e45004_d_n4, assign30400_e45004_d_n5, assign30400_e45004_d_n6, assign30400_e45004_d_n7, assign30400_e45004_d_n8, assign30400_e45004_d_n9, assign30400_e45004_d_n10, assign30400_e45004_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign30400_e45002: f64 = (0.001 * locals.var_x1);
        (assign30400_e45002, (0.001 * locals.var_x1_dn3), (0.001 * locals.var_x1_dn4), (0.001 * locals.var_x1_dn5), (0.001 * locals.var_x1_dn6), (0.001 * locals.var_x1_dn7), (0.001 * locals.var_x1_dn8), (0.001 * locals.var_x1_dn9), (0.001 * locals.var_x1_dn10), (0.001 * locals.var_x1_dn11),)
    } else {
        (locals.var_limit, locals.var_limit_dn3, locals.var_limit_dn4, locals.var_limit_dn5, locals.var_limit_dn6, locals.var_limit_dn7, locals.var_limit_dn8, locals.var_limit_dn9, locals.var_limit_dn10, locals.var_limit_dn11,)
    }
};
        locals.var_limit = assign30400_e45004;
        locals.var_limit_dn3 = assign30400_e45004_d_n3;
        locals.var_limit_dn4 = assign30400_e45004_d_n4;
        locals.var_limit_dn5 = assign30400_e45004_d_n5;
        locals.var_limit_dn6 = assign30400_e45004_d_n6;
        locals.var_limit_dn7 = assign30400_e45004_d_n7;
        locals.var_limit_dn8 = assign30400_e45004_d_n8;
        locals.var_limit_dn9 = assign30400_e45004_d_n9;
        locals.var_limit_dn10 = assign30400_e45004_d_n10;
        locals.var_limit_dn11 = assign30400_e45004_d_n11;
        locals.var_limit_rv = 0.0;

        let (assign30410_e45015, assign30410_e45015_d_n3, assign30410_e45015_d_n4, assign30410_e45015_d_n5, assign30410_e45015_d_n6, assign30410_e45015_d_n7, assign30410_e45015_d_n8, assign30410_e45015_d_n9, assign30410_e45015_d_n10, assign30410_e45015_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign30410_e45009: f64 = (-locals.var_dvbd1_i);
        let assign30410_e45011: f64 = (assign30410_e45009 * locals.var_leff);
        let assign30410_e45013: f64 = (assign30410_e45011 / locals.var_litl);
        (assign30410_e45013, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign30410_e45015;
        locals.var_t0_dn3 = assign30410_e45015_d_n3;
        locals.var_t0_dn4 = assign30410_e45015_d_n4;
        locals.var_t0_dn5 = assign30410_e45015_d_n5;
        locals.var_t0_dn6 = assign30410_e45015_d_n6;
        locals.var_t0_dn7 = assign30410_e45015_d_n7;
        locals.var_t0_dn8 = assign30410_e45015_d_n8;
        locals.var_t0_dn9 = assign30410_e45015_d_n9;
        locals.var_t0_dn10 = assign30410_e45015_d_n10;
        locals.var_t0_dn11 = assign30410_e45015_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign30420_e45047, assign30420_e45047_d_n3, assign30420_e45047_d_n4, assign30420_e45047_d_n5, assign30420_e45047_d_n6, assign30420_e45047_d_n7, assign30420_e45047_d_n8, assign30420_e45047_d_n9, assign30420_e45047_d_n10, assign30420_e45047_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign30420_e45022: f64 = (locals.var_lpe1_i / locals.var_leff);
        let assign30420_e45023: f64 = (1.0 + assign30420_e45022);
        let assign30420_e45026: f64 = (1.602176462e-19 * locals.var_ndep_i);
        let assign30420_e45028: f64 = (assign30420_e45026 * p.p74);
        let assign30420_e45030: f64 = (assign30420_e45028 * p.p74);
        let assign30420_e45031: f64 = (assign30420_e45023 * assign30420_e45030);
        let assign30420_e45034: f64 = (2.0 * locals.var_epssi);
        let assign30420_e45036: f64 = (assign30420_e45034 * locals.var_vt);
        let assign30420_e45037: f64 = (assign30420_e45031 / assign30420_e45036);
        let assign30420_e45040: f64 = (p.p294 / locals.var_vt);
        let assign30420_e45041: f64 = (assign30420_e45037 + assign30420_e45040);
        let assign30420_e45044: f64 = (locals.var_rc * locals.var_vgfbb);
        let assign30420_e45045: f64 = (assign30420_e45041 - assign30420_e45044);
        (assign30420_e45045, (((assign30420_e45023 * (((1.602176462e-19 * locals.var_ndep_i_dn3) * p.p74) * p.p74)) / assign30420_e45036) - (locals.var_rc * locals.var_vgfbb_dn3)), ((((((assign30420_e45023 * (((1.602176462e-19 * locals.var_ndep_i_dn4) * p.p74) * p.p74)) * assign30420_e45036) - (assign30420_e45031 * (assign30420_e45034 * locals.var_vt_dn4))) / (assign30420_e45036 * assign30420_e45036)) + (-((p.p294 * locals.var_vt_dn4) / (locals.var_vt * locals.var_vt)))) - (locals.var_rc * locals.var_vgfbb_dn4)), ((((((assign30420_e45023 * (((1.602176462e-19 * locals.var_ndep_i_dn5) * p.p74) * p.p74)) * assign30420_e45036) - (assign30420_e45031 * (assign30420_e45034 * locals.var_vt_dn5))) / (assign30420_e45036 * assign30420_e45036)) + (-((p.p294 * locals.var_vt_dn5) / (locals.var_vt * locals.var_vt)))) - (locals.var_rc * locals.var_vgfbb_dn5)), (((assign30420_e45023 * (((1.602176462e-19 * locals.var_ndep_i_dn6) * p.p74) * p.p74)) / assign30420_e45036) - (locals.var_rc * locals.var_vgfbb_dn6)), (((assign30420_e45023 * (((1.602176462e-19 * locals.var_ndep_i_dn7) * p.p74) * p.p74)) / assign30420_e45036) - (locals.var_rc * locals.var_vgfbb_dn7)), (((assign30420_e45023 * (((1.602176462e-19 * locals.var_ndep_i_dn8) * p.p74) * p.p74)) / assign30420_e45036) - (locals.var_rc * locals.var_vgfbb_dn8)), (((assign30420_e45023 * (((1.602176462e-19 * locals.var_ndep_i_dn9) * p.p74) * p.p74)) / assign30420_e45036) - (locals.var_rc * locals.var_vgfbb_dn9)), (((assign30420_e45023 * (((1.602176462e-19 * locals.var_ndep_i_dn10) * p.p74) * p.p74)) / assign30420_e45036) - (locals.var_rc * locals.var_vgfbb_dn10)), (((assign30420_e45023 * (((1.602176462e-19 * locals.var_ndep_i_dn11) * p.p74) * p.p74)) / assign30420_e45036) - (locals.var_rc * locals.var_vgfbb_dn11)),)
    } else {
        (locals.var_phic_star, locals.var_phic_star_dn3, locals.var_phic_star_dn4, locals.var_phic_star_dn5, locals.var_phic_star_dn6, locals.var_phic_star_dn7, locals.var_phic_star_dn8, locals.var_phic_star_dn9, locals.var_phic_star_dn10, locals.var_phic_star_dn11,)
    }
};
        locals.var_phic_star = assign30420_e45047;
        locals.var_phic_star_dn3 = assign30420_e45047_d_n3;
        locals.var_phic_star_dn4 = assign30420_e45047_d_n4;
        locals.var_phic_star_dn5 = assign30420_e45047_d_n5;
        locals.var_phic_star_dn6 = assign30420_e45047_d_n6;
        locals.var_phic_star_dn7 = assign30420_e45047_d_n7;
        locals.var_phic_star_dn8 = assign30420_e45047_d_n8;
        locals.var_phic_star_dn9 = assign30420_e45047_d_n9;
        locals.var_phic_star_dn10 = assign30420_e45047_d_n10;
        locals.var_phic_star_dn11 = assign30420_e45047_d_n11;
        locals.var_phic_star_rv = 0.0;

        let (assign30430_e45064, assign30430_e45064_d_n3, assign30430_e45064_d_n4, assign30430_e45064_d_n5, assign30430_e45064_d_n6, assign30430_e45064_d_n7, assign30430_e45064_d_n8, assign30430_e45064_d_n9, assign30430_e45064_d_n10, assign30430_e45064_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign30430_e45054: f64 = (-locals.var_phic_star);
        let assign30430_e45055: f64 = { let limited_exp_arg = assign30430_e45054; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30430_e45057: f64 = (assign30430_e45055 + locals.var_phic_star);
        let assign30430_e45059: f64 = (assign30430_e45057 - 1.0);
        let assign30430_e45060: f64 = (assign30430_e45059).sqrt();
        let assign30430_e45061: f64 = (locals.var_gam * assign30430_e45060);
        let assign30430_e45062: f64 = (locals.var_phic_star + assign30430_e45061);
        (assign30430_e45062, (locals.var_phic_star_dn3 + ((locals.var_gam_dn3 * assign30430_e45060) + (locals.var_gam * ((({ let limited_exp_arg = assign30430_e45054; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phic_star_dn3)) + locals.var_phic_star_dn3) / (2.0 * assign30430_e45060))))), (locals.var_phic_star_dn4 + ((locals.var_gam_dn4 * assign30430_e45060) + (locals.var_gam * ((({ let limited_exp_arg = assign30430_e45054; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phic_star_dn4)) + locals.var_phic_star_dn4) / (2.0 * assign30430_e45060))))), (locals.var_phic_star_dn5 + ((locals.var_gam_dn5 * assign30430_e45060) + (locals.var_gam * ((({ let limited_exp_arg = assign30430_e45054; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phic_star_dn5)) + locals.var_phic_star_dn5) / (2.0 * assign30430_e45060))))), (locals.var_phic_star_dn6 + ((locals.var_gam_dn6 * assign30430_e45060) + (locals.var_gam * ((({ let limited_exp_arg = assign30430_e45054; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phic_star_dn6)) + locals.var_phic_star_dn6) / (2.0 * assign30430_e45060))))), (locals.var_phic_star_dn7 + ((locals.var_gam_dn7 * assign30430_e45060) + (locals.var_gam * ((({ let limited_exp_arg = assign30430_e45054; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phic_star_dn7)) + locals.var_phic_star_dn7) / (2.0 * assign30430_e45060))))), (locals.var_phic_star_dn8 + ((locals.var_gam_dn8 * assign30430_e45060) + (locals.var_gam * ((({ let limited_exp_arg = assign30430_e45054; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phic_star_dn8)) + locals.var_phic_star_dn8) / (2.0 * assign30430_e45060))))), (locals.var_phic_star_dn9 + ((locals.var_gam_dn9 * assign30430_e45060) + (locals.var_gam * ((({ let limited_exp_arg = assign30430_e45054; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phic_star_dn9)) + locals.var_phic_star_dn9) / (2.0 * assign30430_e45060))))), (locals.var_phic_star_dn10 + ((locals.var_gam_dn10 * assign30430_e45060) + (locals.var_gam * ((({ let limited_exp_arg = assign30430_e45054; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phic_star_dn10)) + locals.var_phic_star_dn10) / (2.0 * assign30430_e45060))))), (locals.var_phic_star_dn11 + ((locals.var_gam_dn11 * assign30430_e45060) + (locals.var_gam * ((({ let limited_exp_arg = assign30430_e45054; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phic_star_dn11)) + locals.var_phic_star_dn11) / (2.0 * assign30430_e45060))))),)
    } else {
        (locals.var_vg_fd, locals.var_vg_fd_dn3, locals.var_vg_fd_dn4, locals.var_vg_fd_dn5, locals.var_vg_fd_dn6, locals.var_vg_fd_dn7, locals.var_vg_fd_dn8, locals.var_vg_fd_dn9, locals.var_vg_fd_dn10, locals.var_vg_fd_dn11,)
    }
};
        locals.var_vg_fd = assign30430_e45064;
        locals.var_vg_fd_dn3 = assign30430_e45064_d_n3;
        locals.var_vg_fd_dn4 = assign30430_e45064_d_n4;
        locals.var_vg_fd_dn5 = assign30430_e45064_d_n5;
        locals.var_vg_fd_dn6 = assign30430_e45064_d_n6;
        locals.var_vg_fd_dn7 = assign30430_e45064_d_n7;
        locals.var_vg_fd_dn8 = assign30430_e45064_d_n8;
        locals.var_vg_fd_dn9 = assign30430_e45064_d_n9;
        locals.var_vg_fd_dn10 = assign30430_e45064_d_n10;
        locals.var_vg_fd_dn11 = assign30430_e45064_d_n11;
        locals.var_vg_fd_rv = 0.0;

        let assign30440_e45067: f64 = if locals.var_phic_star < locals.var_phisf { 1.0 } else { 0.0 };
        locals.var_guard617 = assign30440_e45067;
        locals.var_guard617_rv = 0.0;

        let assign30450_e45070: f64 = if locals.var_vgfb1 < locals.var_vg_fd { 1.0 } else { 0.0 };
        locals.var_guard618 = assign30450_e45070;
        locals.var_guard618_rv = 0.0;

        let assign30460_e45072: f64 = (locals.var_vgfb1).abs();
        let assign30460_e45074: f64 = if assign30460_e45072 <= locals.var_limit { 1.0 } else { 0.0 };
        locals.var_guard619 = assign30460_e45074;
        locals.var_guard619_rv = 0.0;

        let (assign30470_e45092, assign30470_e45092_d_n3, assign30470_e45092_d_n4, assign30470_e45092_d_n5, assign30470_e45092_d_n6, assign30470_e45092_d_n7, assign30470_e45092_d_n8, assign30470_e45092_d_n9, assign30470_e45092_d_n10, assign30470_e45092_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign30470_e45086: f64 = (locals.var_inv_x1 * locals.var_inv_x1);
        let assign30470_e45088: f64 = (assign30470_e45086 * 0.16666666666666666);
        let assign30470_e45090: f64 = (assign30470_e45088 * 0.7071067811865475);
        (assign30470_e45090, ((((locals.var_inv_x1_dn3 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn3)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_x1_dn4 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn4)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_x1_dn5 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_x1_dn6 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_x1_dn7 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_x1_dn8 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn8)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_x1_dn9 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn9)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_x1_dn10 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn10)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_x1_dn11 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn11)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign30470_e45092;
        locals.var_t0_dn3 = assign30470_e45092_d_n3;
        locals.var_t0_dn4 = assign30470_e45092_d_n4;
        locals.var_t0_dn5 = assign30470_e45092_d_n5;
        locals.var_t0_dn6 = assign30470_e45092_d_n6;
        locals.var_t0_dn7 = assign30470_e45092_d_n7;
        locals.var_t0_dn8 = assign30470_e45092_d_n8;
        locals.var_t0_dn9 = assign30470_e45092_d_n9;
        locals.var_t0_dn10 = assign30470_e45092_d_n10;
        locals.var_t0_dn11 = assign30470_e45092_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign30480_e45118, assign30480_e45118_d_n3, assign30480_e45118_d_n4, assign30480_e45118_d_n5, assign30480_e45118_d_n6, assign30480_e45118_d_n7, assign30480_e45118_d_n8, assign30480_e45118_d_n9, assign30480_e45118_d_n10, assign30480_e45118_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 != 0.0)) {
        let assign30480_e45104: f64 = (locals.var_vgfb1 * locals.var_inv_x1);
        let assign30480_e45109: f64 = (1.0 - locals.var_exp_ns);
        let assign30480_e45110: f64 = (locals.var_vgfb1 * assign30480_e45109);
        let assign30480_e45112: f64 = (assign30480_e45110 * locals.var_gam);
        let assign30480_e45114: f64 = (assign30480_e45112 * locals.var_t0);
        let assign30480_e45115: f64 = (1.0 + assign30480_e45114);
        let assign30480_e45116: f64 = (assign30480_e45104 * assign30480_e45115);
        (assign30480_e45116, ((((locals.var_vgfb1_dn3 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn3)) * assign30480_e45115) + (assign30480_e45104 * ((((((locals.var_vgfb1_dn3 * assign30480_e45109) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn3))) * locals.var_gam) + (assign30480_e45110 * locals.var_gam_dn3)) * locals.var_t0) + (assign30480_e45112 * locals.var_t0_dn3)))), ((((locals.var_vgfb1_dn4 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn4)) * assign30480_e45115) + (assign30480_e45104 * ((((((locals.var_vgfb1_dn4 * assign30480_e45109) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn4))) * locals.var_gam) + (assign30480_e45110 * locals.var_gam_dn4)) * locals.var_t0) + (assign30480_e45112 * locals.var_t0_dn4)))), ((((locals.var_vgfb1_dn5 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn5)) * assign30480_e45115) + (assign30480_e45104 * ((((((locals.var_vgfb1_dn5 * assign30480_e45109) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn5))) * locals.var_gam) + (assign30480_e45110 * locals.var_gam_dn5)) * locals.var_t0) + (assign30480_e45112 * locals.var_t0_dn5)))), ((((locals.var_vgfb1_dn6 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn6)) * assign30480_e45115) + (assign30480_e45104 * ((((((locals.var_vgfb1_dn6 * assign30480_e45109) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn6))) * locals.var_gam) + (assign30480_e45110 * locals.var_gam_dn6)) * locals.var_t0) + (assign30480_e45112 * locals.var_t0_dn6)))), ((((locals.var_vgfb1_dn7 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn7)) * assign30480_e45115) + (assign30480_e45104 * ((((((locals.var_vgfb1_dn7 * assign30480_e45109) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn7))) * locals.var_gam) + (assign30480_e45110 * locals.var_gam_dn7)) * locals.var_t0) + (assign30480_e45112 * locals.var_t0_dn7)))), ((((locals.var_vgfb1_dn8 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn8)) * assign30480_e45115) + (assign30480_e45104 * ((((((locals.var_vgfb1_dn8 * assign30480_e45109) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn8))) * locals.var_gam) + (assign30480_e45110 * locals.var_gam_dn8)) * locals.var_t0) + (assign30480_e45112 * locals.var_t0_dn8)))), ((((locals.var_vgfb1_dn9 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn9)) * assign30480_e45115) + (assign30480_e45104 * ((((((locals.var_vgfb1_dn9 * assign30480_e45109) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn9))) * locals.var_gam) + (assign30480_e45110 * locals.var_gam_dn9)) * locals.var_t0) + (assign30480_e45112 * locals.var_t0_dn9)))), ((((locals.var_vgfb1_dn10 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn10)) * assign30480_e45115) + (assign30480_e45104 * ((((((locals.var_vgfb1_dn10 * assign30480_e45109) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn10))) * locals.var_gam) + (assign30480_e45110 * locals.var_gam_dn10)) * locals.var_t0) + (assign30480_e45112 * locals.var_t0_dn10)))), ((((locals.var_vgfb1_dn11 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn11)) * assign30480_e45115) + (assign30480_e45104 * ((((((locals.var_vgfb1_dn11 * assign30480_e45109) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn11))) * locals.var_gam) + (assign30480_e45110 * locals.var_gam_dn11)) * locals.var_t0) + (assign30480_e45112 * locals.var_t0_dn11)))),)
    } else {
        (locals.var_sp_dd, locals.var_sp_dd_dn3, locals.var_sp_dd_dn4, locals.var_sp_dd_dn5, locals.var_sp_dd_dn6, locals.var_sp_dd_dn7, locals.var_sp_dd_dn8, locals.var_sp_dd_dn9, locals.var_sp_dd_dn10, locals.var_sp_dd_dn11,)
    }
};
        locals.var_sp_dd = assign30480_e45118;
        locals.var_sp_dd_dn3 = assign30480_e45118_d_n3;
        locals.var_sp_dd_dn4 = assign30480_e45118_d_n4;
        locals.var_sp_dd_dn5 = assign30480_e45118_d_n5;
        locals.var_sp_dd_dn6 = assign30480_e45118_d_n6;
        locals.var_sp_dd_dn7 = assign30480_e45118_d_n7;
        locals.var_sp_dd_dn8 = assign30480_e45118_d_n8;
        locals.var_sp_dd_dn9 = assign30480_e45118_d_n9;
        locals.var_sp_dd_dn10 = assign30480_e45118_d_n10;
        locals.var_sp_dd_dn11 = assign30480_e45118_d_n11;
        locals.var_sp_dd_rv = 0.0;

        let assign30490_e45121: f64 = (-locals.var_limit);
        let assign30490_e45122: f64 = if locals.var_vgfb1 < assign30490_e45121 { 1.0 } else { 0.0 };
        locals.var_guard620 = assign30490_e45122;
        locals.var_guard620_rv = 0.0;

        let (assign30500_e45138, assign30500_e45138_d_n3, assign30500_e45138_d_n4, assign30500_e45138_d_n5, assign30500_e45138_d_n6, assign30500_e45138_d_n7, assign30500_e45138_d_n8, assign30500_e45138_d_n9, assign30500_e45138_d_n10, assign30500_e45138_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30500_e45136: f64 = (-locals.var_vgfb1);
        (assign30500_e45136, (-locals.var_vgfb1_dn3), (-locals.var_vgfb1_dn4), (-locals.var_vgfb1_dn5), (-locals.var_vgfb1_dn6), (-locals.var_vgfb1_dn7), (-locals.var_vgfb1_dn8), (-locals.var_vgfb1_dn9), (-locals.var_vgfb1_dn10), (-locals.var_vgfb1_dn11),)
    } else {
        (locals.var_sp_s_yg, locals.var_sp_s_yg_dn3, locals.var_sp_s_yg_dn4, locals.var_sp_s_yg_dn5, locals.var_sp_s_yg_dn6, locals.var_sp_s_yg_dn7, locals.var_sp_s_yg_dn8, locals.var_sp_s_yg_dn9, locals.var_sp_s_yg_dn10, locals.var_sp_s_yg_dn11,)
    }
};
        locals.var_sp_s_yg = assign30500_e45138;
        locals.var_sp_s_yg_dn3 = assign30500_e45138_d_n3;
        locals.var_sp_s_yg_dn4 = assign30500_e45138_d_n4;
        locals.var_sp_s_yg_dn5 = assign30500_e45138_d_n5;
        locals.var_sp_s_yg_dn6 = assign30500_e45138_d_n6;
        locals.var_sp_s_yg_dn7 = assign30500_e45138_d_n7;
        locals.var_sp_s_yg_dn8 = assign30500_e45138_d_n8;
        locals.var_sp_s_yg_dn9 = assign30500_e45138_d_n9;
        locals.var_sp_s_yg_dn10 = assign30500_e45138_d_n10;
        locals.var_sp_s_yg_dn11 = assign30500_e45138_d_n11;
        locals.var_sp_s_yg_rv = 0.0;

        let (assign30510_e45157, assign30510_e45157_d_n3, assign30510_e45157_d_n4, assign30510_e45157_d_n5, assign30510_e45157_d_n6, assign30510_e45157_d_n7, assign30510_e45157_d_n8, assign30510_e45157_d_n9, assign30510_e45157_d_n10, assign30510_e45157_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30510_e45154: f64 = (locals.var_sp_s_yg * locals.var_inv_x1);
        let assign30510_e45155: f64 = (1.25 * assign30510_e45154);
        (assign30510_e45155, (1.25 * ((locals.var_sp_s_yg_dn3 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn3))), (1.25 * ((locals.var_sp_s_yg_dn4 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn4))), (1.25 * ((locals.var_sp_s_yg_dn5 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn5))), (1.25 * ((locals.var_sp_s_yg_dn6 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn6))), (1.25 * ((locals.var_sp_s_yg_dn7 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn7))), (1.25 * ((locals.var_sp_s_yg_dn8 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn8))), (1.25 * ((locals.var_sp_s_yg_dn9 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn9))), (1.25 * ((locals.var_sp_s_yg_dn10 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn10))), (1.25 * ((locals.var_sp_s_yg_dn11 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn11))),)
    } else {
        (locals.var_sp_s_ysub, locals.var_sp_s_ysub_dn3, locals.var_sp_s_ysub_dn4, locals.var_sp_s_ysub_dn5, locals.var_sp_s_ysub_dn6, locals.var_sp_s_ysub_dn7, locals.var_sp_s_ysub_dn8, locals.var_sp_s_ysub_dn9, locals.var_sp_s_ysub_dn10, locals.var_sp_s_ysub_dn11,)
    }
};
        locals.var_sp_s_ysub = assign30510_e45157;
        locals.var_sp_s_ysub_dn3 = assign30510_e45157_d_n3;
        locals.var_sp_s_ysub_dn4 = assign30510_e45157_d_n4;
        locals.var_sp_s_ysub_dn5 = assign30510_e45157_d_n5;
        locals.var_sp_s_ysub_dn6 = assign30510_e45157_d_n6;
        locals.var_sp_s_ysub_dn7 = assign30510_e45157_d_n7;
        locals.var_sp_s_ysub_dn8 = assign30510_e45157_d_n8;
        locals.var_sp_s_ysub_dn9 = assign30510_e45157_d_n9;
        locals.var_sp_s_ysub_dn10 = assign30510_e45157_d_n10;
        locals.var_sp_s_ysub_dn11 = assign30510_e45157_d_n11;
        locals.var_sp_s_ysub_rv = 0.0;

        let (assign30520_e45187, assign30520_e45187_d_n3, assign30520_e45187_d_n4, assign30520_e45187_d_n5, assign30520_e45187_d_n6, assign30520_e45187_d_n7, assign30520_e45187_d_n8, assign30520_e45187_d_n9, assign30520_e45187_d_n10, assign30520_e45187_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30520_e45173: f64 = (locals.var_sp_s_ysub + 10.0);
        let assign30520_e45176: f64 = (locals.var_sp_s_ysub - 6.0);
        let assign30520_e45179: f64 = (locals.var_sp_s_ysub - 6.0);
        let assign30520_e45180: f64 = (assign30520_e45176 * assign30520_e45179);
        let assign30520_e45182: f64 = (assign30520_e45180 + 64.0);
        let assign30520_e45183: f64 = (assign30520_e45182).sqrt();
        let assign30520_e45184: f64 = (assign30520_e45173 - assign30520_e45183);
        let assign30520_e45185: f64 = (0.5 * assign30520_e45184);
        (assign30520_e45185, (0.5 * (locals.var_sp_s_ysub_dn3 - (((locals.var_sp_s_ysub_dn3 * assign30520_e45179) + (assign30520_e45176 * locals.var_sp_s_ysub_dn3)) / (2.0 * assign30520_e45183)))), (0.5 * (locals.var_sp_s_ysub_dn4 - (((locals.var_sp_s_ysub_dn4 * assign30520_e45179) + (assign30520_e45176 * locals.var_sp_s_ysub_dn4)) / (2.0 * assign30520_e45183)))), (0.5 * (locals.var_sp_s_ysub_dn5 - (((locals.var_sp_s_ysub_dn5 * assign30520_e45179) + (assign30520_e45176 * locals.var_sp_s_ysub_dn5)) / (2.0 * assign30520_e45183)))), (0.5 * (locals.var_sp_s_ysub_dn6 - (((locals.var_sp_s_ysub_dn6 * assign30520_e45179) + (assign30520_e45176 * locals.var_sp_s_ysub_dn6)) / (2.0 * assign30520_e45183)))), (0.5 * (locals.var_sp_s_ysub_dn7 - (((locals.var_sp_s_ysub_dn7 * assign30520_e45179) + (assign30520_e45176 * locals.var_sp_s_ysub_dn7)) / (2.0 * assign30520_e45183)))), (0.5 * (locals.var_sp_s_ysub_dn8 - (((locals.var_sp_s_ysub_dn8 * assign30520_e45179) + (assign30520_e45176 * locals.var_sp_s_ysub_dn8)) / (2.0 * assign30520_e45183)))), (0.5 * (locals.var_sp_s_ysub_dn9 - (((locals.var_sp_s_ysub_dn9 * assign30520_e45179) + (assign30520_e45176 * locals.var_sp_s_ysub_dn9)) / (2.0 * assign30520_e45183)))), (0.5 * (locals.var_sp_s_ysub_dn10 - (((locals.var_sp_s_ysub_dn10 * assign30520_e45179) + (assign30520_e45176 * locals.var_sp_s_ysub_dn10)) / (2.0 * assign30520_e45183)))), (0.5 * (locals.var_sp_s_ysub_dn11 - (((locals.var_sp_s_ysub_dn11 * assign30520_e45179) + (assign30520_e45176 * locals.var_sp_s_ysub_dn11)) / (2.0 * assign30520_e45183)))),)
    } else {
        (locals.var_sp_s_eta, locals.var_sp_s_eta_dn3, locals.var_sp_s_eta_dn4, locals.var_sp_s_eta_dn5, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8, locals.var_sp_s_eta_dn9, locals.var_sp_s_eta_dn10, locals.var_sp_s_eta_dn11,)
    }
};
        locals.var_sp_s_eta = assign30520_e45187;
        locals.var_sp_s_eta_dn3 = assign30520_e45187_d_n3;
        locals.var_sp_s_eta_dn4 = assign30520_e45187_d_n4;
        locals.var_sp_s_eta_dn5 = assign30520_e45187_d_n5;
        locals.var_sp_s_eta_dn6 = assign30520_e45187_d_n6;
        locals.var_sp_s_eta_dn7 = assign30520_e45187_d_n7;
        locals.var_sp_s_eta_dn8 = assign30520_e45187_d_n8;
        locals.var_sp_s_eta_dn9 = assign30520_e45187_d_n9;
        locals.var_sp_s_eta_dn10 = assign30520_e45187_d_n10;
        locals.var_sp_s_eta_dn11 = assign30520_e45187_d_n11;
        locals.var_sp_s_eta_rv = 0.0;

        let (assign30530_e45204, assign30530_e45204_d_n3, assign30530_e45204_d_n4, assign30530_e45204_d_n5, assign30530_e45204_d_n6, assign30530_e45204_d_n7, assign30530_e45204_d_n8, assign30530_e45204_d_n9, assign30530_e45204_d_n10, assign30530_e45204_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30530_e45202: f64 = (locals.var_sp_s_yg - locals.var_sp_s_eta);
        (assign30530_e45202, (locals.var_sp_s_yg_dn3 - locals.var_sp_s_eta_dn3), (locals.var_sp_s_yg_dn4 - locals.var_sp_s_eta_dn4), (locals.var_sp_s_yg_dn5 - locals.var_sp_s_eta_dn5), (locals.var_sp_s_yg_dn6 - locals.var_sp_s_eta_dn6), (locals.var_sp_s_yg_dn7 - locals.var_sp_s_eta_dn7), (locals.var_sp_s_yg_dn8 - locals.var_sp_s_eta_dn8), (locals.var_sp_s_yg_dn9 - locals.var_sp_s_eta_dn9), (locals.var_sp_s_yg_dn10 - locals.var_sp_s_eta_dn10), (locals.var_sp_s_yg_dn11 - locals.var_sp_s_eta_dn11),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn3, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9, locals.var_sp_s_temp_dn10, locals.var_sp_s_temp_dn11,)
    }
};
        locals.var_sp_s_temp = assign30530_e45204;
        locals.var_sp_s_temp_dn3 = assign30530_e45204_d_n3;
        locals.var_sp_s_temp_dn4 = assign30530_e45204_d_n4;
        locals.var_sp_s_temp_dn5 = assign30530_e45204_d_n5;
        locals.var_sp_s_temp_dn6 = assign30530_e45204_d_n6;
        locals.var_sp_s_temp_dn7 = assign30530_e45204_d_n7;
        locals.var_sp_s_temp_dn8 = assign30530_e45204_d_n8;
        locals.var_sp_s_temp_dn9 = assign30530_e45204_d_n9;
        locals.var_sp_s_temp_dn10 = assign30530_e45204_d_n10;
        locals.var_sp_s_temp_dn11 = assign30530_e45204_d_n11;
        locals.var_sp_s_temp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_89(
        locals: &mut StampLocals,
    ) {
        let (assign30540_e45227, assign30540_e45227_d_n3, assign30540_e45227_d_n4, assign30540_e45227_d_n5, assign30540_e45227_d_n6, assign30540_e45227_d_n7, assign30540_e45227_d_n8, assign30540_e45227_d_n9, assign30540_e45227_d_n10, assign30540_e45227_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30540_e45219: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign30540_e45223: f64 = (locals.var_sp_s_eta + 1.0);
        let assign30540_e45224: f64 = (locals.var_gam2 * assign30540_e45223);
        let assign30540_e45225: f64 = (assign30540_e45219 + assign30540_e45224);
        (assign30540_e45225, (((locals.var_sp_s_temp_dn3 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn3)) + ((locals.var_gam2_dn3 * assign30540_e45223) + (locals.var_gam2 * locals.var_sp_s_eta_dn3))), (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) + ((locals.var_gam2_dn4 * assign30540_e45223) + (locals.var_gam2 * locals.var_sp_s_eta_dn4))), (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) + ((locals.var_gam2_dn5 * assign30540_e45223) + (locals.var_gam2 * locals.var_sp_s_eta_dn5))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) + ((locals.var_gam2_dn6 * assign30540_e45223) + (locals.var_gam2 * locals.var_sp_s_eta_dn6))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) + ((locals.var_gam2_dn7 * assign30540_e45223) + (locals.var_gam2 * locals.var_sp_s_eta_dn7))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) + ((locals.var_gam2_dn8 * assign30540_e45223) + (locals.var_gam2 * locals.var_sp_s_eta_dn8))), (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) + ((locals.var_gam2_dn9 * assign30540_e45223) + (locals.var_gam2 * locals.var_sp_s_eta_dn9))), (((locals.var_sp_s_temp_dn10 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn10)) + ((locals.var_gam2_dn10 * assign30540_e45223) + (locals.var_gam2 * locals.var_sp_s_eta_dn10))), (((locals.var_sp_s_temp_dn11 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn11)) + ((locals.var_gam2_dn11 * assign30540_e45223) + (locals.var_gam2 * locals.var_sp_s_eta_dn11))),)
    } else {
        (locals.var_sp_s_a, locals.var_sp_s_a_dn3, locals.var_sp_s_a_dn4, locals.var_sp_s_a_dn5, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8, locals.var_sp_s_a_dn9, locals.var_sp_s_a_dn10, locals.var_sp_s_a_dn11,)
    }
};
        locals.var_sp_s_a = assign30540_e45227;
        locals.var_sp_s_a_dn3 = assign30540_e45227_d_n3;
        locals.var_sp_s_a_dn4 = assign30540_e45227_d_n4;
        locals.var_sp_s_a_dn5 = assign30540_e45227_d_n5;
        locals.var_sp_s_a_dn6 = assign30540_e45227_d_n6;
        locals.var_sp_s_a_dn7 = assign30540_e45227_d_n7;
        locals.var_sp_s_a_dn8 = assign30540_e45227_d_n8;
        locals.var_sp_s_a_dn9 = assign30540_e45227_d_n9;
        locals.var_sp_s_a_dn10 = assign30540_e45227_d_n10;
        locals.var_sp_s_a_dn11 = assign30540_e45227_d_n11;
        locals.var_sp_s_a_rv = 0.0;

        let (assign30550_e45246, assign30550_e45246_d_n3, assign30550_e45246_d_n4, assign30550_e45246_d_n5, assign30550_e45246_d_n6, assign30550_e45246_d_n7, assign30550_e45246_d_n8, assign30550_e45246_d_n9, assign30550_e45246_d_n10, assign30550_e45246_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30550_e45242: f64 = (2.0 * locals.var_sp_s_temp);
        let assign30550_e45244: f64 = (assign30550_e45242 - locals.var_gam2);
        (assign30550_e45244, ((2.0 * locals.var_sp_s_temp_dn3) - locals.var_gam2_dn3), ((2.0 * locals.var_sp_s_temp_dn4) - locals.var_gam2_dn4), ((2.0 * locals.var_sp_s_temp_dn5) - locals.var_gam2_dn5), ((2.0 * locals.var_sp_s_temp_dn6) - locals.var_gam2_dn6), ((2.0 * locals.var_sp_s_temp_dn7) - locals.var_gam2_dn7), ((2.0 * locals.var_sp_s_temp_dn8) - locals.var_gam2_dn8), ((2.0 * locals.var_sp_s_temp_dn9) - locals.var_gam2_dn9), ((2.0 * locals.var_sp_s_temp_dn10) - locals.var_gam2_dn10), ((2.0 * locals.var_sp_s_temp_dn11) - locals.var_gam2_dn11),)
    } else {
        (locals.var_sp_s_c, locals.var_sp_s_c_dn3, locals.var_sp_s_c_dn4, locals.var_sp_s_c_dn5, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8, locals.var_sp_s_c_dn9, locals.var_sp_s_c_dn10, locals.var_sp_s_c_dn11,)
    }
};
        locals.var_sp_s_c = assign30550_e45246;
        locals.var_sp_s_c_dn3 = assign30550_e45246_d_n3;
        locals.var_sp_s_c_dn4 = assign30550_e45246_d_n4;
        locals.var_sp_s_c_dn5 = assign30550_e45246_d_n5;
        locals.var_sp_s_c_dn6 = assign30550_e45246_d_n6;
        locals.var_sp_s_c_dn7 = assign30550_e45246_d_n7;
        locals.var_sp_s_c_dn8 = assign30550_e45246_d_n8;
        locals.var_sp_s_c_dn9 = assign30550_e45246_d_n9;
        locals.var_sp_s_c_dn10 = assign30550_e45246_d_n10;
        locals.var_sp_s_c_dn11 = assign30550_e45246_d_n11;
        locals.var_sp_s_c_rv = 0.0;

        let (assign30560_e45269, assign30560_e45269_d_n3, assign30560_e45269_d_n4, assign30560_e45269_d_n5, assign30560_e45269_d_n6, assign30560_e45269_d_n7, assign30560_e45269_d_n8, assign30560_e45269_d_n9, assign30560_e45269_d_n10, assign30560_e45269_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30560_e45260: f64 = (-locals.var_sp_s_eta);
        let assign30560_e45263: f64 = (locals.var_sp_s_a * locals.var_inv_gam2);
        let assign30560_e45265: f64 = (assign30560_e45263).max(1e-38);
        let assign30560_e45266: f64 = (assign30560_e45265).ln();
        let assign30560_e45267: f64 = (assign30560_e45260 + assign30560_e45266);
        (assign30560_e45267, ((-locals.var_sp_s_eta_dn3) + (if assign30560_e45263 >= 1e-38 { ((locals.var_sp_s_a_dn3 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn3)) } else { 0.0 } / assign30560_e45265)), ((-locals.var_sp_s_eta_dn4) + (if assign30560_e45263 >= 1e-38 { ((locals.var_sp_s_a_dn4 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn4)) } else { 0.0 } / assign30560_e45265)), ((-locals.var_sp_s_eta_dn5) + (if assign30560_e45263 >= 1e-38 { ((locals.var_sp_s_a_dn5 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn5)) } else { 0.0 } / assign30560_e45265)), ((-locals.var_sp_s_eta_dn6) + (if assign30560_e45263 >= 1e-38 { ((locals.var_sp_s_a_dn6 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn6)) } else { 0.0 } / assign30560_e45265)), ((-locals.var_sp_s_eta_dn7) + (if assign30560_e45263 >= 1e-38 { ((locals.var_sp_s_a_dn7 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn7)) } else { 0.0 } / assign30560_e45265)), ((-locals.var_sp_s_eta_dn8) + (if assign30560_e45263 >= 1e-38 { ((locals.var_sp_s_a_dn8 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn8)) } else { 0.0 } / assign30560_e45265)), ((-locals.var_sp_s_eta_dn9) + (if assign30560_e45263 >= 1e-38 { ((locals.var_sp_s_a_dn9 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn9)) } else { 0.0 } / assign30560_e45265)), ((-locals.var_sp_s_eta_dn10) + (if assign30560_e45263 >= 1e-38 { ((locals.var_sp_s_a_dn10 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn10)) } else { 0.0 } / assign30560_e45265)), ((-locals.var_sp_s_eta_dn11) + (if assign30560_e45263 >= 1e-38 { ((locals.var_sp_s_a_dn11 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn11)) } else { 0.0 } / assign30560_e45265)),)
    } else {
        (locals.var_sp_s_tau, locals.var_sp_s_tau_dn3, locals.var_sp_s_tau_dn4, locals.var_sp_s_tau_dn5, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8, locals.var_sp_s_tau_dn9, locals.var_sp_s_tau_dn10, locals.var_sp_s_tau_dn11,)
    }
};
        locals.var_sp_s_tau = assign30560_e45269;
        locals.var_sp_s_tau_dn3 = assign30560_e45269_d_n3;
        locals.var_sp_s_tau_dn4 = assign30560_e45269_d_n4;
        locals.var_sp_s_tau_dn5 = assign30560_e45269_d_n5;
        locals.var_sp_s_tau_dn6 = assign30560_e45269_d_n6;
        locals.var_sp_s_tau_dn7 = assign30560_e45269_d_n7;
        locals.var_sp_s_tau_dn8 = assign30560_e45269_d_n8;
        locals.var_sp_s_tau_dn9 = assign30560_e45269_d_n9;
        locals.var_sp_s_tau_dn10 = assign30560_e45269_d_n10;
        locals.var_sp_s_tau_dn11 = assign30560_e45269_d_n11;
        locals.var_sp_s_tau_rv = 0.0;

        let (assign30570_e45286, assign30570_e45286_d_n3, assign30570_e45286_d_n4, assign30570_e45286_d_n5, assign30570_e45286_d_n6, assign30570_e45286_d_n7, assign30570_e45286_d_n8, assign30570_e45286_d_n9, assign30570_e45286_d_n10, assign30570_e45286_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30570_e45284: f64 = (locals.var_sp_s_a + locals.var_sp_s_c);
        (assign30570_e45284, (locals.var_sp_s_a_dn3 + locals.var_sp_s_c_dn3), (locals.var_sp_s_a_dn4 + locals.var_sp_s_c_dn4), (locals.var_sp_s_a_dn5 + locals.var_sp_s_c_dn5), (locals.var_sp_s_a_dn6 + locals.var_sp_s_c_dn6), (locals.var_sp_s_a_dn7 + locals.var_sp_s_c_dn7), (locals.var_sp_s_a_dn8 + locals.var_sp_s_c_dn8), (locals.var_sp_s_a_dn9 + locals.var_sp_s_c_dn9), (locals.var_sp_s_a_dn10 + locals.var_sp_s_c_dn10), (locals.var_sp_s_a_dn11 + locals.var_sp_s_c_dn11),)
    } else {
        (locals.var_nu, locals.var_nu_dn3, locals.var_nu_dn4, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9, locals.var_nu_dn10, locals.var_nu_dn11,)
    }
};
        locals.var_nu = assign30570_e45286;
        locals.var_nu_dn3 = assign30570_e45286_d_n3;
        locals.var_nu_dn4 = assign30570_e45286_d_n4;
        locals.var_nu_dn5 = assign30570_e45286_d_n5;
        locals.var_nu_dn6 = assign30570_e45286_d_n6;
        locals.var_nu_dn7 = assign30570_e45286_d_n7;
        locals.var_nu_dn8 = assign30570_e45286_d_n8;
        locals.var_nu_dn9 = assign30570_e45286_d_n9;
        locals.var_nu_dn10 = assign30570_e45286_d_n10;
        locals.var_nu_dn11 = assign30570_e45286_d_n11;
        locals.var_nu_rv = 0.0;

        let (assign30580_e45313, assign30580_e45313_d_n3, assign30580_e45313_d_n4, assign30580_e45313_d_n5, assign30580_e45313_d_n6, assign30580_e45313_d_n7, assign30580_e45313_d_n8, assign30580_e45313_d_n9, assign30580_e45313_d_n10, assign30580_e45313_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30580_e45301: f64 = (locals.var_nu * locals.var_nu);
        let assign30580_e45306: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign30580_e45307: f64 = (0.5 * assign30580_e45306);
        let assign30580_e45309: f64 = (assign30580_e45307 - locals.var_sp_s_a);
        let assign30580_e45310: f64 = (locals.var_sp_s_tau * assign30580_e45309);
        let assign30580_e45311: f64 = (assign30580_e45301 + assign30580_e45310);
        (assign30580_e45311, (((locals.var_nu_dn3 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn3)) + ((locals.var_sp_s_tau_dn3 * assign30580_e45309) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn3 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn3))) - locals.var_sp_s_a_dn3)))), (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau_dn4 * assign30580_e45309) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn4 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn4))) - locals.var_sp_s_a_dn4)))), (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau_dn5 * assign30580_e45309) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5))) - locals.var_sp_s_a_dn5)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau_dn6 * assign30580_e45309) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6))) - locals.var_sp_s_a_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau_dn7 * assign30580_e45309) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7))) - locals.var_sp_s_a_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau_dn8 * assign30580_e45309) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8))) - locals.var_sp_s_a_dn8)))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau_dn9 * assign30580_e45309) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn9 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn9))) - locals.var_sp_s_a_dn9)))), (((locals.var_nu_dn10 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn10)) + ((locals.var_sp_s_tau_dn10 * assign30580_e45309) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn10 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn10))) - locals.var_sp_s_a_dn10)))), (((locals.var_nu_dn11 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn11)) + ((locals.var_sp_s_tau_dn11 * assign30580_e45309) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn11 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn11))) - locals.var_sp_s_a_dn11)))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn3, locals.var_mutau_dn4, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9, locals.var_mutau_dn10, locals.var_mutau_dn11,)
    }
};
        locals.var_mutau = assign30580_e45313;
        locals.var_mutau_dn3 = assign30580_e45313_d_n3;
        locals.var_mutau_dn4 = assign30580_e45313_d_n4;
        locals.var_mutau_dn5 = assign30580_e45313_d_n5;
        locals.var_mutau_dn6 = assign30580_e45313_d_n6;
        locals.var_mutau_dn7 = assign30580_e45313_d_n7;
        locals.var_mutau_dn8 = assign30580_e45313_d_n8;
        locals.var_mutau_dn9 = assign30580_e45313_d_n9;
        locals.var_mutau_dn10 = assign30580_e45313_d_n10;
        locals.var_mutau_dn11 = assign30580_e45313_d_n11;
        locals.var_mutau_rv = 0.0;

        let (assign30590_e45354, assign30590_e45354_d_n3, assign30590_e45354_d_n4, assign30590_e45354_d_n5, assign30590_e45354_d_n6, assign30590_e45354_d_n7, assign30590_e45354_d_n8, assign30590_e45354_d_n9, assign30590_e45354_d_n10, assign30590_e45354_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30590_e45329: f64 = (locals.var_sp_s_a * locals.var_nu);
        let assign30590_e45331: f64 = (assign30590_e45329 * locals.var_sp_s_tau);
        let assign30590_e45335: f64 = (locals.var_nu / locals.var_mutau);
        let assign30590_e45337: f64 = (assign30590_e45335 * locals.var_sp_s_tau);
        let assign30590_e45339: f64 = (assign30590_e45337 * locals.var_sp_s_tau);
        let assign30590_e45341: f64 = (assign30590_e45339 * locals.var_sp_s_c);
        let assign30590_e45344: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign30590_e45346: f64 = (assign30590_e45344 * 0.3333333333333333);
        let assign30590_e45348: f64 = (assign30590_e45346 - locals.var_sp_s_a);
        let assign30590_e45349: f64 = (assign30590_e45341 * assign30590_e45348);
        let assign30590_e45350: f64 = (locals.var_mutau + assign30590_e45349);
        let assign30590_e45351: f64 = (assign30590_e45331 / assign30590_e45350);
        let assign30590_e45352: f64 = (locals.var_sp_s_eta + assign30590_e45351);
        (assign30590_e45352, (locals.var_sp_s_eta_dn3 + (((((((locals.var_sp_s_a_dn3 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn3)) * locals.var_sp_s_tau) + (assign30590_e45329 * locals.var_sp_s_tau_dn3)) * assign30590_e45350) - (assign30590_e45331 * (locals.var_mutau_dn3 + (((((((((((locals.var_nu_dn3 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn3)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30590_e45335 * locals.var_sp_s_tau_dn3)) * locals.var_sp_s_tau) + (assign30590_e45337 * locals.var_sp_s_tau_dn3)) * locals.var_sp_s_c) + (assign30590_e45339 * locals.var_sp_s_c_dn3)) * assign30590_e45348) + (assign30590_e45341 * ((((locals.var_sp_s_c_dn3 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn3)) * 0.3333333333333333) - locals.var_sp_s_a_dn3)))))) / (assign30590_e45350 * assign30590_e45350))), (locals.var_sp_s_eta_dn4 + (((((((locals.var_sp_s_a_dn4 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn4)) * locals.var_sp_s_tau) + (assign30590_e45329 * locals.var_sp_s_tau_dn4)) * assign30590_e45350) - (assign30590_e45331 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30590_e45335 * locals.var_sp_s_tau_dn4)) * locals.var_sp_s_tau) + (assign30590_e45337 * locals.var_sp_s_tau_dn4)) * locals.var_sp_s_c) + (assign30590_e45339 * locals.var_sp_s_c_dn4)) * assign30590_e45348) + (assign30590_e45341 * ((((locals.var_sp_s_c_dn4 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn4)) * 0.3333333333333333) - locals.var_sp_s_a_dn4)))))) / (assign30590_e45350 * assign30590_e45350))), (locals.var_sp_s_eta_dn5 + (((((((locals.var_sp_s_a_dn5 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn5)) * locals.var_sp_s_tau) + (assign30590_e45329 * locals.var_sp_s_tau_dn5)) * assign30590_e45350) - (assign30590_e45331 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30590_e45335 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_tau) + (assign30590_e45337 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_c) + (assign30590_e45339 * locals.var_sp_s_c_dn5)) * assign30590_e45348) + (assign30590_e45341 * ((((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5)) * 0.3333333333333333) - locals.var_sp_s_a_dn5)))))) / (assign30590_e45350 * assign30590_e45350))), (locals.var_sp_s_eta_dn6 + (((((((locals.var_sp_s_a_dn6 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn6)) * locals.var_sp_s_tau) + (assign30590_e45329 * locals.var_sp_s_tau_dn6)) * assign30590_e45350) - (assign30590_e45331 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30590_e45335 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_tau) + (assign30590_e45337 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_c) + (assign30590_e45339 * locals.var_sp_s_c_dn6)) * assign30590_e45348) + (assign30590_e45341 * ((((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6)) * 0.3333333333333333) - locals.var_sp_s_a_dn6)))))) / (assign30590_e45350 * assign30590_e45350))), (locals.var_sp_s_eta_dn7 + (((((((locals.var_sp_s_a_dn7 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn7)) * locals.var_sp_s_tau) + (assign30590_e45329 * locals.var_sp_s_tau_dn7)) * assign30590_e45350) - (assign30590_e45331 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30590_e45335 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_tau) + (assign30590_e45337 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_c) + (assign30590_e45339 * locals.var_sp_s_c_dn7)) * assign30590_e45348) + (assign30590_e45341 * ((((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7)) * 0.3333333333333333) - locals.var_sp_s_a_dn7)))))) / (assign30590_e45350 * assign30590_e45350))), (locals.var_sp_s_eta_dn8 + (((((((locals.var_sp_s_a_dn8 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn8)) * locals.var_sp_s_tau) + (assign30590_e45329 * locals.var_sp_s_tau_dn8)) * assign30590_e45350) - (assign30590_e45331 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30590_e45335 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_tau) + (assign30590_e45337 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_c) + (assign30590_e45339 * locals.var_sp_s_c_dn8)) * assign30590_e45348) + (assign30590_e45341 * ((((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8)) * 0.3333333333333333) - locals.var_sp_s_a_dn8)))))) / (assign30590_e45350 * assign30590_e45350))), (locals.var_sp_s_eta_dn9 + (((((((locals.var_sp_s_a_dn9 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn9)) * locals.var_sp_s_tau) + (assign30590_e45329 * locals.var_sp_s_tau_dn9)) * assign30590_e45350) - (assign30590_e45331 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30590_e45335 * locals.var_sp_s_tau_dn9)) * locals.var_sp_s_tau) + (assign30590_e45337 * locals.var_sp_s_tau_dn9)) * locals.var_sp_s_c) + (assign30590_e45339 * locals.var_sp_s_c_dn9)) * assign30590_e45348) + (assign30590_e45341 * ((((locals.var_sp_s_c_dn9 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn9)) * 0.3333333333333333) - locals.var_sp_s_a_dn9)))))) / (assign30590_e45350 * assign30590_e45350))), (locals.var_sp_s_eta_dn10 + (((((((locals.var_sp_s_a_dn10 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn10)) * locals.var_sp_s_tau) + (assign30590_e45329 * locals.var_sp_s_tau_dn10)) * assign30590_e45350) - (assign30590_e45331 * (locals.var_mutau_dn10 + (((((((((((locals.var_nu_dn10 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn10)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30590_e45335 * locals.var_sp_s_tau_dn10)) * locals.var_sp_s_tau) + (assign30590_e45337 * locals.var_sp_s_tau_dn10)) * locals.var_sp_s_c) + (assign30590_e45339 * locals.var_sp_s_c_dn10)) * assign30590_e45348) + (assign30590_e45341 * ((((locals.var_sp_s_c_dn10 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn10)) * 0.3333333333333333) - locals.var_sp_s_a_dn10)))))) / (assign30590_e45350 * assign30590_e45350))), (locals.var_sp_s_eta_dn11 + (((((((locals.var_sp_s_a_dn11 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn11)) * locals.var_sp_s_tau) + (assign30590_e45329 * locals.var_sp_s_tau_dn11)) * assign30590_e45350) - (assign30590_e45331 * (locals.var_mutau_dn11 + (((((((((((locals.var_nu_dn11 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn11)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30590_e45335 * locals.var_sp_s_tau_dn11)) * locals.var_sp_s_tau) + (assign30590_e45337 * locals.var_sp_s_tau_dn11)) * locals.var_sp_s_c) + (assign30590_e45339 * locals.var_sp_s_c_dn11)) * assign30590_e45348) + (assign30590_e45341 * ((((locals.var_sp_s_c_dn11 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn11)) * 0.3333333333333333) - locals.var_sp_s_a_dn11)))))) / (assign30590_e45350 * assign30590_e45350))),)
    } else {
        (locals.var_sp_s_y0, locals.var_sp_s_y0_dn3, locals.var_sp_s_y0_dn4, locals.var_sp_s_y0_dn5, locals.var_sp_s_y0_dn6, locals.var_sp_s_y0_dn7, locals.var_sp_s_y0_dn8, locals.var_sp_s_y0_dn9, locals.var_sp_s_y0_dn10, locals.var_sp_s_y0_dn11,)
    }
};
        locals.var_sp_s_y0 = assign30590_e45354;
        locals.var_sp_s_y0_dn3 = assign30590_e45354_d_n3;
        locals.var_sp_s_y0_dn4 = assign30590_e45354_d_n4;
        locals.var_sp_s_y0_dn5 = assign30590_e45354_d_n5;
        locals.var_sp_s_y0_dn6 = assign30590_e45354_d_n6;
        locals.var_sp_s_y0_dn7 = assign30590_e45354_d_n7;
        locals.var_sp_s_y0_dn8 = assign30590_e45354_d_n8;
        locals.var_sp_s_y0_dn9 = assign30590_e45354_d_n9;
        locals.var_sp_s_y0_dn10 = assign30590_e45354_d_n10;
        locals.var_sp_s_y0_dn11 = assign30590_e45354_d_n11;
        locals.var_sp_s_y0_rv = 0.0;

        let (assign30600_e45370, assign30600_e45370_d_n3, assign30600_e45370_d_n4, assign30600_e45370_d_n5, assign30600_e45370_d_n6, assign30600_e45370_d_n7, assign30600_e45370_d_n8, assign30600_e45370_d_n9, assign30600_e45370_d_n10, assign30600_e45370_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30600_e45368: f64 = { let limited_exp_arg = locals.var_sp_s_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign30600_e45368, ({ let limited_exp_arg = locals.var_sp_s_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_y0_dn3), ({ let limited_exp_arg = locals.var_sp_s_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_y0_dn4), ({ let limited_exp_arg = locals.var_sp_s_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_y0_dn5), ({ let limited_exp_arg = locals.var_sp_s_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_y0_dn6), ({ let limited_exp_arg = locals.var_sp_s_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_y0_dn7), ({ let limited_exp_arg = locals.var_sp_s_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_y0_dn8), ({ let limited_exp_arg = locals.var_sp_s_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_y0_dn9), ({ let limited_exp_arg = locals.var_sp_s_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_y0_dn10), ({ let limited_exp_arg = locals.var_sp_s_y0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_y0_dn11),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn3, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9, locals.var_sp_s_delta0_dn10, locals.var_sp_s_delta0_dn11,)
    }
};
        locals.var_sp_s_delta0 = assign30600_e45370;
        locals.var_sp_s_delta0_dn3 = assign30600_e45370_d_n3;
        locals.var_sp_s_delta0_dn4 = assign30600_e45370_d_n4;
        locals.var_sp_s_delta0_dn5 = assign30600_e45370_d_n5;
        locals.var_sp_s_delta0_dn6 = assign30600_e45370_d_n6;
        locals.var_sp_s_delta0_dn7 = assign30600_e45370_d_n7;
        locals.var_sp_s_delta0_dn8 = assign30600_e45370_d_n8;
        locals.var_sp_s_delta0_dn9 = assign30600_e45370_d_n9;
        locals.var_sp_s_delta0_dn10 = assign30600_e45370_d_n10;
        locals.var_sp_s_delta0_dn11 = assign30600_e45370_d_n11;
        locals.var_sp_s_delta0_rv = 0.0;

        let (assign30610_e45387, assign30610_e45387_d_n3, assign30610_e45387_d_n4, assign30610_e45387_d_n5, assign30610_e45387_d_n6, assign30610_e45387_d_n7, assign30610_e45387_d_n8, assign30610_e45387_d_n9, assign30610_e45387_d_n10, assign30610_e45387_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30610_e45385: f64 = (1.0 / locals.var_sp_s_delta0);
        (assign30610_e45385, (-(locals.var_sp_s_delta0_dn3 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn4 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn5 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn6 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn7 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn8 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn9 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn10 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn11 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn3, locals.var_sp_s_delta1_dn4, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn9, locals.var_sp_s_delta1_dn10, locals.var_sp_s_delta1_dn11,)
    }
};
        locals.var_sp_s_delta1 = assign30610_e45387;
        locals.var_sp_s_delta1_dn3 = assign30610_e45387_d_n3;
        locals.var_sp_s_delta1_dn4 = assign30610_e45387_d_n4;
        locals.var_sp_s_delta1_dn5 = assign30610_e45387_d_n5;
        locals.var_sp_s_delta1_dn6 = assign30610_e45387_d_n6;
        locals.var_sp_s_delta1_dn7 = assign30610_e45387_d_n7;
        locals.var_sp_s_delta1_dn8 = assign30610_e45387_d_n8;
        locals.var_sp_s_delta1_dn9 = assign30610_e45387_d_n9;
        locals.var_sp_s_delta1_dn10 = assign30610_e45387_d_n10;
        locals.var_sp_s_delta1_dn11 = assign30610_e45387_d_n11;
        locals.var_sp_s_delta1_rv = 0.0;

        let (assign30620_e45408, assign30620_e45408_d_n3, assign30620_e45408_d_n4, assign30620_e45408_d_n5, assign30620_e45408_d_n6, assign30620_e45408_d_n7, assign30620_e45408_d_n8, assign30620_e45408_d_n9, assign30620_e45408_d_n10, assign30620_e45408_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30620_e45404: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_y0);
        let assign30620_e45405: f64 = (2.0 + assign30620_e45404);
        let assign30620_e45406: f64 = (1.0 / assign30620_e45405);
        (assign30620_e45406, (-(((locals.var_sp_s_y0_dn3 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn3)) / (assign30620_e45405 * assign30620_e45405))), (-(((locals.var_sp_s_y0_dn4 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn4)) / (assign30620_e45405 * assign30620_e45405))), (-(((locals.var_sp_s_y0_dn5 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn5)) / (assign30620_e45405 * assign30620_e45405))), (-(((locals.var_sp_s_y0_dn6 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn6)) / (assign30620_e45405 * assign30620_e45405))), (-(((locals.var_sp_s_y0_dn7 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn7)) / (assign30620_e45405 * assign30620_e45405))), (-(((locals.var_sp_s_y0_dn8 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn8)) / (assign30620_e45405 * assign30620_e45405))), (-(((locals.var_sp_s_y0_dn9 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn9)) / (assign30620_e45405 * assign30620_e45405))), (-(((locals.var_sp_s_y0_dn10 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn10)) / (assign30620_e45405 * assign30620_e45405))), (-(((locals.var_sp_s_y0_dn11 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn11)) / (assign30620_e45405 * assign30620_e45405))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn3, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9, locals.var_sp_s_temp_dn10, locals.var_sp_s_temp_dn11,)
    }
};
        locals.var_sp_s_temp = assign30620_e45408;
        locals.var_sp_s_temp_dn3 = assign30620_e45408_d_n3;
        locals.var_sp_s_temp_dn4 = assign30620_e45408_d_n4;
        locals.var_sp_s_temp_dn5 = assign30620_e45408_d_n5;
        locals.var_sp_s_temp_dn6 = assign30620_e45408_d_n6;
        locals.var_sp_s_temp_dn7 = assign30620_e45408_d_n7;
        locals.var_sp_s_temp_dn8 = assign30620_e45408_d_n8;
        locals.var_sp_s_temp_dn9 = assign30620_e45408_d_n9;
        locals.var_sp_s_temp_dn10 = assign30620_e45408_d_n10;
        locals.var_sp_s_temp_dn11 = assign30620_e45408_d_n11;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign30630_e45427, assign30630_e45427_d_n3, assign30630_e45427_d_n4, assign30630_e45427_d_n5, assign30630_e45427_d_n6, assign30630_e45427_d_n7, assign30630_e45427_d_n8, assign30630_e45427_d_n9, assign30630_e45427_d_n10, assign30630_e45427_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30630_e45423: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_y0);
        let assign30630_e45425: f64 = (assign30630_e45423 * locals.var_sp_s_temp);
        (assign30630_e45425, ((((locals.var_sp_s_y0_dn3 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn3)) * locals.var_sp_s_temp) + (assign30630_e45423 * locals.var_sp_s_temp_dn3)), ((((locals.var_sp_s_y0_dn4 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn4)) * locals.var_sp_s_temp) + (assign30630_e45423 * locals.var_sp_s_temp_dn4)), ((((locals.var_sp_s_y0_dn5 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn5)) * locals.var_sp_s_temp) + (assign30630_e45423 * locals.var_sp_s_temp_dn5)), ((((locals.var_sp_s_y0_dn6 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn6)) * locals.var_sp_s_temp) + (assign30630_e45423 * locals.var_sp_s_temp_dn6)), ((((locals.var_sp_s_y0_dn7 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn7)) * locals.var_sp_s_temp) + (assign30630_e45423 * locals.var_sp_s_temp_dn7)), ((((locals.var_sp_s_y0_dn8 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn8)) * locals.var_sp_s_temp) + (assign30630_e45423 * locals.var_sp_s_temp_dn8)), ((((locals.var_sp_s_y0_dn9 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn9)) * locals.var_sp_s_temp) + (assign30630_e45423 * locals.var_sp_s_temp_dn9)), ((((locals.var_sp_s_y0_dn10 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn10)) * locals.var_sp_s_temp) + (assign30630_e45423 * locals.var_sp_s_temp_dn10)), ((((locals.var_sp_s_y0_dn11 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn11)) * locals.var_sp_s_temp) + (assign30630_e45423 * locals.var_sp_s_temp_dn11)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn3, locals.var_sp_s_xi0_dn4, locals.var_sp_s_xi0_dn5, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, locals.var_sp_s_xi0_dn9, locals.var_sp_s_xi0_dn10, locals.var_sp_s_xi0_dn11,)
    }
};
        locals.var_sp_s_xi0 = assign30630_e45427;
        locals.var_sp_s_xi0_dn3 = assign30630_e45427_d_n3;
        locals.var_sp_s_xi0_dn4 = assign30630_e45427_d_n4;
        locals.var_sp_s_xi0_dn5 = assign30630_e45427_d_n5;
        locals.var_sp_s_xi0_dn6 = assign30630_e45427_d_n6;
        locals.var_sp_s_xi0_dn7 = assign30630_e45427_d_n7;
        locals.var_sp_s_xi0_dn8 = assign30630_e45427_d_n8;
        locals.var_sp_s_xi0_dn9 = assign30630_e45427_d_n9;
        locals.var_sp_s_xi0_dn10 = assign30630_e45427_d_n10;
        locals.var_sp_s_xi0_dn11 = assign30630_e45427_d_n11;
        locals.var_sp_s_xi0_rv = 0.0;

        let (assign30640_e45448, assign30640_e45448_d_n3, assign30640_e45448_d_n4, assign30640_e45448_d_n5, assign30640_e45448_d_n6, assign30640_e45448_d_n7, assign30640_e45448_d_n8, assign30640_e45448_d_n9, assign30640_e45448_d_n10, assign30640_e45448_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30640_e45443: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_temp);
        let assign30640_e45445: f64 = (assign30640_e45443 * locals.var_sp_s_temp);
        let assign30640_e45446: f64 = (4.0 * assign30640_e45445);
        (assign30640_e45446, (4.0 * ((((locals.var_sp_s_y0_dn3 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn3)) * locals.var_sp_s_temp) + (assign30640_e45443 * locals.var_sp_s_temp_dn3))), (4.0 * ((((locals.var_sp_s_y0_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn4)) * locals.var_sp_s_temp) + (assign30640_e45443 * locals.var_sp_s_temp_dn4))), (4.0 * ((((locals.var_sp_s_y0_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign30640_e45443 * locals.var_sp_s_temp_dn5))), (4.0 * ((((locals.var_sp_s_y0_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign30640_e45443 * locals.var_sp_s_temp_dn6))), (4.0 * ((((locals.var_sp_s_y0_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign30640_e45443 * locals.var_sp_s_temp_dn7))), (4.0 * ((((locals.var_sp_s_y0_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign30640_e45443 * locals.var_sp_s_temp_dn8))), (4.0 * ((((locals.var_sp_s_y0_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn9)) * locals.var_sp_s_temp) + (assign30640_e45443 * locals.var_sp_s_temp_dn9))), (4.0 * ((((locals.var_sp_s_y0_dn10 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn10)) * locals.var_sp_s_temp) + (assign30640_e45443 * locals.var_sp_s_temp_dn10))), (4.0 * ((((locals.var_sp_s_y0_dn11 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn11)) * locals.var_sp_s_temp) + (assign30640_e45443 * locals.var_sp_s_temp_dn11))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn3, locals.var_sp_s_xi1_dn4, locals.var_sp_s_xi1_dn5, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, locals.var_sp_s_xi1_dn9, locals.var_sp_s_xi1_dn10, locals.var_sp_s_xi1_dn11,)
    }
};
        locals.var_sp_s_xi1 = assign30640_e45448;
        locals.var_sp_s_xi1_dn3 = assign30640_e45448_d_n3;
        locals.var_sp_s_xi1_dn4 = assign30640_e45448_d_n4;
        locals.var_sp_s_xi1_dn5 = assign30640_e45448_d_n5;
        locals.var_sp_s_xi1_dn6 = assign30640_e45448_d_n6;
        locals.var_sp_s_xi1_dn7 = assign30640_e45448_d_n7;
        locals.var_sp_s_xi1_dn8 = assign30640_e45448_d_n8;
        locals.var_sp_s_xi1_dn9 = assign30640_e45448_d_n9;
        locals.var_sp_s_xi1_dn10 = assign30640_e45448_d_n10;
        locals.var_sp_s_xi1_dn11 = assign30640_e45448_d_n11;
        locals.var_sp_s_xi1_rv = 0.0;

        let (assign30650_e45473, assign30650_e45473_d_n3, assign30650_e45473_d_n4, assign30650_e45473_d_n5, assign30650_e45473_d_n6, assign30650_e45473_d_n7, assign30650_e45473_d_n8, assign30650_e45473_d_n9, assign30650_e45473_d_n10, assign30650_e45473_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30650_e45463: f64 = (8.0 * locals.var_sp_s_temp);
        let assign30650_e45466: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign30650_e45467: f64 = (assign30650_e45463 - assign30650_e45466);
        let assign30650_e45469: f64 = (assign30650_e45467 * locals.var_sp_s_temp);
        let assign30650_e45471: f64 = (assign30650_e45469 * locals.var_sp_s_temp);
        (assign30650_e45471, ((((((8.0 * locals.var_sp_s_temp_dn3) - (12.0 * locals.var_sp_s_xi0_dn3)) * locals.var_sp_s_temp) + (assign30650_e45467 * locals.var_sp_s_temp_dn3)) * locals.var_sp_s_temp) + (assign30650_e45469 * locals.var_sp_s_temp_dn3)), ((((((8.0 * locals.var_sp_s_temp_dn4) - (12.0 * locals.var_sp_s_xi0_dn4)) * locals.var_sp_s_temp) + (assign30650_e45467 * locals.var_sp_s_temp_dn4)) * locals.var_sp_s_temp) + (assign30650_e45469 * locals.var_sp_s_temp_dn4)), ((((((8.0 * locals.var_sp_s_temp_dn5) - (12.0 * locals.var_sp_s_xi0_dn5)) * locals.var_sp_s_temp) + (assign30650_e45467 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign30650_e45469 * locals.var_sp_s_temp_dn5)), ((((((8.0 * locals.var_sp_s_temp_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp) + (assign30650_e45467 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign30650_e45469 * locals.var_sp_s_temp_dn6)), ((((((8.0 * locals.var_sp_s_temp_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp) + (assign30650_e45467 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign30650_e45469 * locals.var_sp_s_temp_dn7)), ((((((8.0 * locals.var_sp_s_temp_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp) + (assign30650_e45467 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign30650_e45469 * locals.var_sp_s_temp_dn8)), ((((((8.0 * locals.var_sp_s_temp_dn9) - (12.0 * locals.var_sp_s_xi0_dn9)) * locals.var_sp_s_temp) + (assign30650_e45467 * locals.var_sp_s_temp_dn9)) * locals.var_sp_s_temp) + (assign30650_e45469 * locals.var_sp_s_temp_dn9)), ((((((8.0 * locals.var_sp_s_temp_dn10) - (12.0 * locals.var_sp_s_xi0_dn10)) * locals.var_sp_s_temp) + (assign30650_e45467 * locals.var_sp_s_temp_dn10)) * locals.var_sp_s_temp) + (assign30650_e45469 * locals.var_sp_s_temp_dn10)), ((((((8.0 * locals.var_sp_s_temp_dn11) - (12.0 * locals.var_sp_s_xi0_dn11)) * locals.var_sp_s_temp) + (assign30650_e45467 * locals.var_sp_s_temp_dn11)) * locals.var_sp_s_temp) + (assign30650_e45469 * locals.var_sp_s_temp_dn11)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn3, locals.var_sp_s_xi2_dn4, locals.var_sp_s_xi2_dn5, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, locals.var_sp_s_xi2_dn9, locals.var_sp_s_xi2_dn10, locals.var_sp_s_xi2_dn11,)
    }
};
        locals.var_sp_s_xi2 = assign30650_e45473;
        locals.var_sp_s_xi2_dn3 = assign30650_e45473_d_n3;
        locals.var_sp_s_xi2_dn4 = assign30650_e45473_d_n4;
        locals.var_sp_s_xi2_dn5 = assign30650_e45473_d_n5;
        locals.var_sp_s_xi2_dn6 = assign30650_e45473_d_n6;
        locals.var_sp_s_xi2_dn7 = assign30650_e45473_d_n7;
        locals.var_sp_s_xi2_dn8 = assign30650_e45473_d_n8;
        locals.var_sp_s_xi2_dn9 = assign30650_e45473_d_n9;
        locals.var_sp_s_xi2_dn10 = assign30650_e45473_d_n10;
        locals.var_sp_s_xi2_dn11 = assign30650_e45473_d_n11;
        locals.var_sp_s_xi2_rv = 0.0;

        let (assign30660_e45490, assign30660_e45490_d_n3, assign30660_e45490_d_n4, assign30660_e45490_d_n5, assign30660_e45490_d_n6, assign30660_e45490_d_n7, assign30660_e45490_d_n8, assign30660_e45490_d_n9, assign30660_e45490_d_n10, assign30660_e45490_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30660_e45488: f64 = (locals.var_sp_s_yg - locals.var_sp_s_y0);
        (assign30660_e45488, (locals.var_sp_s_yg_dn3 - locals.var_sp_s_y0_dn3), (locals.var_sp_s_yg_dn4 - locals.var_sp_s_y0_dn4), (locals.var_sp_s_yg_dn5 - locals.var_sp_s_y0_dn5), (locals.var_sp_s_yg_dn6 - locals.var_sp_s_y0_dn6), (locals.var_sp_s_yg_dn7 - locals.var_sp_s_y0_dn7), (locals.var_sp_s_yg_dn8 - locals.var_sp_s_y0_dn8), (locals.var_sp_s_yg_dn9 - locals.var_sp_s_y0_dn9), (locals.var_sp_s_yg_dn10 - locals.var_sp_s_y0_dn10), (locals.var_sp_s_yg_dn11 - locals.var_sp_s_y0_dn11),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn3, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9, locals.var_sp_s_temp_dn10, locals.var_sp_s_temp_dn11,)
    }
};
        locals.var_sp_s_temp = assign30660_e45490;
        locals.var_sp_s_temp_dn3 = assign30660_e45490_d_n3;
        locals.var_sp_s_temp_dn4 = assign30660_e45490_d_n4;
        locals.var_sp_s_temp_dn5 = assign30660_e45490_d_n5;
        locals.var_sp_s_temp_dn6 = assign30660_e45490_d_n6;
        locals.var_sp_s_temp_dn7 = assign30660_e45490_d_n7;
        locals.var_sp_s_temp_dn8 = assign30660_e45490_d_n8;
        locals.var_sp_s_temp_dn9 = assign30660_e45490_d_n9;
        locals.var_sp_s_temp_dn10 = assign30660_e45490_d_n10;
        locals.var_sp_s_temp_dn11 = assign30660_e45490_d_n11;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign30670_e45507, assign30670_e45507_d_n3, assign30670_e45507_d_n4, assign30670_e45507_d_n5, assign30670_e45507_d_n6, assign30670_e45507_d_n7, assign30670_e45507_d_n8, assign30670_e45507_d_n9, assign30670_e45507_d_n10, assign30670_e45507_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30670_e45505: f64 = (locals.var_exp_ns * locals.var_sp_s_delta1);
        (assign30670_e45505, ((locals.var_exp_ns_dn3 * locals.var_sp_s_delta1) + (locals.var_exp_ns * locals.var_sp_s_delta1_dn3)), ((locals.var_exp_ns_dn4 * locals.var_sp_s_delta1) + (locals.var_exp_ns * locals.var_sp_s_delta1_dn4)), ((locals.var_exp_ns_dn5 * locals.var_sp_s_delta1) + (locals.var_exp_ns * locals.var_sp_s_delta1_dn5)), ((locals.var_exp_ns_dn6 * locals.var_sp_s_delta1) + (locals.var_exp_ns * locals.var_sp_s_delta1_dn6)), ((locals.var_exp_ns_dn7 * locals.var_sp_s_delta1) + (locals.var_exp_ns * locals.var_sp_s_delta1_dn7)), ((locals.var_exp_ns_dn8 * locals.var_sp_s_delta1) + (locals.var_exp_ns * locals.var_sp_s_delta1_dn8)), ((locals.var_exp_ns_dn9 * locals.var_sp_s_delta1) + (locals.var_exp_ns * locals.var_sp_s_delta1_dn9)), ((locals.var_exp_ns_dn10 * locals.var_sp_s_delta1) + (locals.var_exp_ns * locals.var_sp_s_delta1_dn10)), ((locals.var_exp_ns_dn11 * locals.var_sp_s_delta1) + (locals.var_exp_ns * locals.var_sp_s_delta1_dn11)),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn3, locals.var_sp_s_temp1_dn4, locals.var_sp_s_temp1_dn5, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, locals.var_sp_s_temp1_dn9, locals.var_sp_s_temp1_dn10, locals.var_sp_s_temp1_dn11,)
    }
};
        locals.var_sp_s_temp1 = assign30670_e45507;
        locals.var_sp_s_temp1_dn3 = assign30670_e45507_d_n3;
        locals.var_sp_s_temp1_dn4 = assign30670_e45507_d_n4;
        locals.var_sp_s_temp1_dn5 = assign30670_e45507_d_n5;
        locals.var_sp_s_temp1_dn6 = assign30670_e45507_d_n6;
        locals.var_sp_s_temp1_dn7 = assign30670_e45507_d_n7;
        locals.var_sp_s_temp1_dn8 = assign30670_e45507_d_n8;
        locals.var_sp_s_temp1_dn9 = assign30670_e45507_d_n9;
        locals.var_sp_s_temp1_dn10 = assign30670_e45507_d_n10;
        locals.var_sp_s_temp1_dn11 = assign30670_e45507_d_n11;
        locals.var_sp_s_temp1_rv = 0.0;

        let (assign30680_e45538, assign30680_e45538_d_n3, assign30680_e45538_d_n4, assign30680_e45538_d_n5, assign30680_e45538_d_n6, assign30680_e45538_d_n7, assign30680_e45538_d_n8, assign30680_e45538_d_n9, assign30680_e45538_d_n10, assign30680_e45538_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30680_e45522: f64 = (2.0 * locals.var_sp_s_temp);
        let assign30680_e45526: f64 = (locals.var_sp_s_delta0 - 1.0);
        let assign30680_e45528: f64 = (assign30680_e45526 - locals.var_sp_s_temp1);
        let assign30680_e45532: f64 = (1.0 - locals.var_sp_s_xi1);
        let assign30680_e45533: f64 = (locals.var_exp_ns * assign30680_e45532);
        let assign30680_e45534: f64 = (assign30680_e45528 + assign30680_e45533);
        let assign30680_e45535: f64 = (locals.var_gam2 * assign30680_e45534);
        let assign30680_e45536: f64 = (assign30680_e45522 + assign30680_e45535);
        (assign30680_e45536, ((2.0 * locals.var_sp_s_temp_dn3) + ((locals.var_gam2_dn3 * assign30680_e45534) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn3 - locals.var_sp_s_temp1_dn3) + ((locals.var_exp_ns_dn3 * assign30680_e45532) + (locals.var_exp_ns * (-locals.var_sp_s_xi1_dn3))))))), ((2.0 * locals.var_sp_s_temp_dn4) + ((locals.var_gam2_dn4 * assign30680_e45534) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn4 - locals.var_sp_s_temp1_dn4) + ((locals.var_exp_ns_dn4 * assign30680_e45532) + (locals.var_exp_ns * (-locals.var_sp_s_xi1_dn4))))))), ((2.0 * locals.var_sp_s_temp_dn5) + ((locals.var_gam2_dn5 * assign30680_e45534) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn5 - locals.var_sp_s_temp1_dn5) + ((locals.var_exp_ns_dn5 * assign30680_e45532) + (locals.var_exp_ns * (-locals.var_sp_s_xi1_dn5))))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gam2_dn6 * assign30680_e45534) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn6 - locals.var_sp_s_temp1_dn6) + ((locals.var_exp_ns_dn6 * assign30680_e45532) + (locals.var_exp_ns * (-locals.var_sp_s_xi1_dn6))))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gam2_dn7 * assign30680_e45534) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn7 - locals.var_sp_s_temp1_dn7) + ((locals.var_exp_ns_dn7 * assign30680_e45532) + (locals.var_exp_ns * (-locals.var_sp_s_xi1_dn7))))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gam2_dn8 * assign30680_e45534) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn8 - locals.var_sp_s_temp1_dn8) + ((locals.var_exp_ns_dn8 * assign30680_e45532) + (locals.var_exp_ns * (-locals.var_sp_s_xi1_dn8))))))), ((2.0 * locals.var_sp_s_temp_dn9) + ((locals.var_gam2_dn9 * assign30680_e45534) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn9 - locals.var_sp_s_temp1_dn9) + ((locals.var_exp_ns_dn9 * assign30680_e45532) + (locals.var_exp_ns * (-locals.var_sp_s_xi1_dn9))))))), ((2.0 * locals.var_sp_s_temp_dn10) + ((locals.var_gam2_dn10 * assign30680_e45534) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn10 - locals.var_sp_s_temp1_dn10) + ((locals.var_exp_ns_dn10 * assign30680_e45532) + (locals.var_exp_ns * (-locals.var_sp_s_xi1_dn10))))))), ((2.0 * locals.var_sp_s_temp_dn11) + ((locals.var_gam2_dn11 * assign30680_e45534) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn11 - locals.var_sp_s_temp1_dn11) + ((locals.var_exp_ns_dn11 * assign30680_e45532) + (locals.var_exp_ns * (-locals.var_sp_s_xi1_dn11))))))),)
    } else {
        (locals.var_sp_s_pc, locals.var_sp_s_pc_dn3, locals.var_sp_s_pc_dn4, locals.var_sp_s_pc_dn5, locals.var_sp_s_pc_dn6, locals.var_sp_s_pc_dn7, locals.var_sp_s_pc_dn8, locals.var_sp_s_pc_dn9, locals.var_sp_s_pc_dn10, locals.var_sp_s_pc_dn11,)
    }
};
        locals.var_sp_s_pc = assign30680_e45538;
        locals.var_sp_s_pc_dn3 = assign30680_e45538_d_n3;
        locals.var_sp_s_pc_dn4 = assign30680_e45538_d_n4;
        locals.var_sp_s_pc_dn5 = assign30680_e45538_d_n5;
        locals.var_sp_s_pc_dn6 = assign30680_e45538_d_n6;
        locals.var_sp_s_pc_dn7 = assign30680_e45538_d_n7;
        locals.var_sp_s_pc_dn8 = assign30680_e45538_d_n8;
        locals.var_sp_s_pc_dn9 = assign30680_e45538_d_n9;
        locals.var_sp_s_pc_dn10 = assign30680_e45538_d_n10;
        locals.var_sp_s_pc_dn11 = assign30680_e45538_d_n11;
        locals.var_sp_s_pc_rv = 0.0;

        let (assign30690_e45573, assign30690_e45573_d_n3, assign30690_e45573_d_n4, assign30690_e45573_d_n5, assign30690_e45573_d_n6, assign30690_e45573_d_n7, assign30690_e45573_d_n8, assign30690_e45573_d_n9, assign30690_e45573_d_n10, assign30690_e45573_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30690_e45553: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign30690_e45557: f64 = (locals.var_sp_s_delta0 - locals.var_sp_s_y0);
        let assign30690_e45559: f64 = (assign30690_e45557 - 1.0);
        let assign30690_e45561: f64 = (assign30690_e45559 + locals.var_sp_s_temp1);
        let assign30690_e45565: f64 = (locals.var_sp_s_y0 - 1.0);
        let assign30690_e45567: f64 = (assign30690_e45565 - locals.var_sp_s_xi0);
        let assign30690_e45568: f64 = (locals.var_exp_ns * assign30690_e45567);
        let assign30690_e45569: f64 = (assign30690_e45561 + assign30690_e45568);
        let assign30690_e45570: f64 = (locals.var_gam2 * assign30690_e45569);
        let assign30690_e45571: f64 = (assign30690_e45553 - assign30690_e45570);
        (assign30690_e45571, (((locals.var_sp_s_temp_dn3 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn3)) - ((locals.var_gam2_dn3 * assign30690_e45569) + (locals.var_gam2 * (((locals.var_sp_s_delta0_dn3 - locals.var_sp_s_y0_dn3) + locals.var_sp_s_temp1_dn3) + ((locals.var_exp_ns_dn3 * assign30690_e45567) + (locals.var_exp_ns * (locals.var_sp_s_y0_dn3 - locals.var_sp_s_xi0_dn3))))))), (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) - ((locals.var_gam2_dn4 * assign30690_e45569) + (locals.var_gam2 * (((locals.var_sp_s_delta0_dn4 - locals.var_sp_s_y0_dn4) + locals.var_sp_s_temp1_dn4) + ((locals.var_exp_ns_dn4 * assign30690_e45567) + (locals.var_exp_ns * (locals.var_sp_s_y0_dn4 - locals.var_sp_s_xi0_dn4))))))), (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) - ((locals.var_gam2_dn5 * assign30690_e45569) + (locals.var_gam2 * (((locals.var_sp_s_delta0_dn5 - locals.var_sp_s_y0_dn5) + locals.var_sp_s_temp1_dn5) + ((locals.var_exp_ns_dn5 * assign30690_e45567) + (locals.var_exp_ns * (locals.var_sp_s_y0_dn5 - locals.var_sp_s_xi0_dn5))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gam2_dn6 * assign30690_e45569) + (locals.var_gam2 * (((locals.var_sp_s_delta0_dn6 - locals.var_sp_s_y0_dn6) + locals.var_sp_s_temp1_dn6) + ((locals.var_exp_ns_dn6 * assign30690_e45567) + (locals.var_exp_ns * (locals.var_sp_s_y0_dn6 - locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gam2_dn7 * assign30690_e45569) + (locals.var_gam2 * (((locals.var_sp_s_delta0_dn7 - locals.var_sp_s_y0_dn7) + locals.var_sp_s_temp1_dn7) + ((locals.var_exp_ns_dn7 * assign30690_e45567) + (locals.var_exp_ns * (locals.var_sp_s_y0_dn7 - locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gam2_dn8 * assign30690_e45569) + (locals.var_gam2 * (((locals.var_sp_s_delta0_dn8 - locals.var_sp_s_y0_dn8) + locals.var_sp_s_temp1_dn8) + ((locals.var_exp_ns_dn8 * assign30690_e45567) + (locals.var_exp_ns * (locals.var_sp_s_y0_dn8 - locals.var_sp_s_xi0_dn8))))))), (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) - ((locals.var_gam2_dn9 * assign30690_e45569) + (locals.var_gam2 * (((locals.var_sp_s_delta0_dn9 - locals.var_sp_s_y0_dn9) + locals.var_sp_s_temp1_dn9) + ((locals.var_exp_ns_dn9 * assign30690_e45567) + (locals.var_exp_ns * (locals.var_sp_s_y0_dn9 - locals.var_sp_s_xi0_dn9))))))), (((locals.var_sp_s_temp_dn10 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn10)) - ((locals.var_gam2_dn10 * assign30690_e45569) + (locals.var_gam2 * (((locals.var_sp_s_delta0_dn10 - locals.var_sp_s_y0_dn10) + locals.var_sp_s_temp1_dn10) + ((locals.var_exp_ns_dn10 * assign30690_e45567) + (locals.var_exp_ns * (locals.var_sp_s_y0_dn10 - locals.var_sp_s_xi0_dn10))))))), (((locals.var_sp_s_temp_dn11 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn11)) - ((locals.var_gam2_dn11 * assign30690_e45569) + (locals.var_gam2 * (((locals.var_sp_s_delta0_dn11 - locals.var_sp_s_y0_dn11) + locals.var_sp_s_temp1_dn11) + ((locals.var_exp_ns_dn11 * assign30690_e45567) + (locals.var_exp_ns * (locals.var_sp_s_y0_dn11 - locals.var_sp_s_xi0_dn11))))))),)
    } else {
        (locals.var_sp_s_qc, locals.var_sp_s_qc_dn3, locals.var_sp_s_qc_dn4, locals.var_sp_s_qc_dn5, locals.var_sp_s_qc_dn6, locals.var_sp_s_qc_dn7, locals.var_sp_s_qc_dn8, locals.var_sp_s_qc_dn9, locals.var_sp_s_qc_dn10, locals.var_sp_s_qc_dn11,)
    }
};
        locals.var_sp_s_qc = assign30690_e45573;
        locals.var_sp_s_qc_dn3 = assign30690_e45573_d_n3;
        locals.var_sp_s_qc_dn4 = assign30690_e45573_d_n4;
        locals.var_sp_s_qc_dn5 = assign30690_e45573_d_n5;
        locals.var_sp_s_qc_dn6 = assign30690_e45573_d_n6;
        locals.var_sp_s_qc_dn7 = assign30690_e45573_d_n7;
        locals.var_sp_s_qc_dn8 = assign30690_e45573_d_n8;
        locals.var_sp_s_qc_dn9 = assign30690_e45573_d_n9;
        locals.var_sp_s_qc_dn10 = assign30690_e45573_d_n10;
        locals.var_sp_s_qc_dn11 = assign30690_e45573_d_n11;
        locals.var_sp_s_qc_rv = 0.0;

        let (assign30700_e45598, assign30700_e45598_d_n3, assign30700_e45598_d_n4, assign30700_e45598_d_n5, assign30700_e45598_d_n6, assign30700_e45598_d_n7, assign30700_e45598_d_n8, assign30700_e45598_d_n9, assign30700_e45598_d_n10, assign30700_e45598_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30700_e45590: f64 = (locals.var_sp_s_delta0 + locals.var_sp_s_temp1);
        let assign30700_e45593: f64 = (locals.var_exp_ns * locals.var_sp_s_xi2);
        let assign30700_e45594: f64 = (assign30700_e45590 - assign30700_e45593);
        let assign30700_e45595: f64 = (locals.var_gam2 * assign30700_e45594);
        let assign30700_e45596: f64 = (2.0 - assign30700_e45595);
        (assign30700_e45596, (-((locals.var_gam2_dn3 * assign30700_e45594) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn3 + locals.var_sp_s_temp1_dn3) - ((locals.var_exp_ns_dn3 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn3)))))), (-((locals.var_gam2_dn4 * assign30700_e45594) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn4 + locals.var_sp_s_temp1_dn4) - ((locals.var_exp_ns_dn4 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn4)))))), (-((locals.var_gam2_dn5 * assign30700_e45594) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn5 + locals.var_sp_s_temp1_dn5) - ((locals.var_exp_ns_dn5 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn5)))))), (-((locals.var_gam2_dn6 * assign30700_e45594) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn6 + locals.var_sp_s_temp1_dn6) - ((locals.var_exp_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn6)))))), (-((locals.var_gam2_dn7 * assign30700_e45594) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn7 + locals.var_sp_s_temp1_dn7) - ((locals.var_exp_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn7)))))), (-((locals.var_gam2_dn8 * assign30700_e45594) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn8 + locals.var_sp_s_temp1_dn8) - ((locals.var_exp_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn8)))))), (-((locals.var_gam2_dn9 * assign30700_e45594) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn9 + locals.var_sp_s_temp1_dn9) - ((locals.var_exp_ns_dn9 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn9)))))), (-((locals.var_gam2_dn10 * assign30700_e45594) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn10 + locals.var_sp_s_temp1_dn10) - ((locals.var_exp_ns_dn10 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn10)))))), (-((locals.var_gam2_dn11 * assign30700_e45594) + (locals.var_gam2 * ((locals.var_sp_s_delta0_dn11 + locals.var_sp_s_temp1_dn11) - ((locals.var_exp_ns_dn11 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn11)))))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn3, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9, locals.var_sp_s_temp_dn10, locals.var_sp_s_temp_dn11,)
    }
};
        locals.var_sp_s_temp = assign30700_e45598;
        locals.var_sp_s_temp_dn3 = assign30700_e45598_d_n3;
        locals.var_sp_s_temp_dn4 = assign30700_e45598_d_n4;
        locals.var_sp_s_temp_dn5 = assign30700_e45598_d_n5;
        locals.var_sp_s_temp_dn6 = assign30700_e45598_d_n6;
        locals.var_sp_s_temp_dn7 = assign30700_e45598_d_n7;
        locals.var_sp_s_temp_dn8 = assign30700_e45598_d_n8;
        locals.var_sp_s_temp_dn9 = assign30700_e45598_d_n9;
        locals.var_sp_s_temp_dn10 = assign30700_e45598_d_n10;
        locals.var_sp_s_temp_dn11 = assign30700_e45598_d_n11;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign30710_e45621, assign30710_e45621_d_n3, assign30710_e45621_d_n4, assign30710_e45621_d_n5, assign30710_e45621_d_n6, assign30710_e45621_d_n7, assign30710_e45621_d_n8, assign30710_e45621_d_n9, assign30710_e45621_d_n10, assign30710_e45621_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30710_e45613: f64 = (locals.var_sp_s_pc * locals.var_sp_s_pc);
        let assign30710_e45617: f64 = (locals.var_sp_s_qc * locals.var_sp_s_temp);
        let assign30710_e45618: f64 = (2.0 * assign30710_e45617);
        let assign30710_e45619: f64 = (assign30710_e45613 - assign30710_e45618);
        (assign30710_e45619, (((locals.var_sp_s_pc_dn3 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn3)) - (2.0 * ((locals.var_sp_s_qc_dn3 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn3)))), (((locals.var_sp_s_pc_dn4 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn4)) - (2.0 * ((locals.var_sp_s_qc_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn4)))), (((locals.var_sp_s_pc_dn5 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn5)) - (2.0 * ((locals.var_sp_s_qc_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn5)))), (((locals.var_sp_s_pc_dn6 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn6)) - (2.0 * ((locals.var_sp_s_qc_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn6)))), (((locals.var_sp_s_pc_dn7 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn7)) - (2.0 * ((locals.var_sp_s_qc_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn7)))), (((locals.var_sp_s_pc_dn8 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn8)) - (2.0 * ((locals.var_sp_s_qc_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn8)))), (((locals.var_sp_s_pc_dn9 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn9)) - (2.0 * ((locals.var_sp_s_qc_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn9)))), (((locals.var_sp_s_pc_dn10 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn10)) - (2.0 * ((locals.var_sp_s_qc_dn10 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn10)))), (((locals.var_sp_s_pc_dn11 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn11)) - (2.0 * ((locals.var_sp_s_qc_dn11 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn11)))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn3, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9, locals.var_sp_s_temp_dn10, locals.var_sp_s_temp_dn11,)
    }
};
        locals.var_sp_s_temp = assign30710_e45621;
        locals.var_sp_s_temp_dn3 = assign30710_e45621_d_n3;
        locals.var_sp_s_temp_dn4 = assign30710_e45621_d_n4;
        locals.var_sp_s_temp_dn5 = assign30710_e45621_d_n5;
        locals.var_sp_s_temp_dn6 = assign30710_e45621_d_n6;
        locals.var_sp_s_temp_dn7 = assign30710_e45621_d_n7;
        locals.var_sp_s_temp_dn8 = assign30710_e45621_d_n8;
        locals.var_sp_s_temp_dn9 = assign30710_e45621_d_n9;
        locals.var_sp_s_temp_dn10 = assign30710_e45621_d_n10;
        locals.var_sp_s_temp_dn11 = assign30710_e45621_d_n11;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign30720_e45646, assign30720_e45646_d_n3, assign30720_e45646_d_n4, assign30720_e45646_d_n5, assign30720_e45646_d_n6, assign30720_e45646_d_n7, assign30720_e45646_d_n8, assign30720_e45646_d_n9, assign30720_e45646_d_n10, assign30720_e45646_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 != 0.0)) {
        let assign30720_e45635: f64 = (-locals.var_sp_s_y0);
        let assign30720_e45640: f64 = (locals.var_sp_s_temp).sqrt();
        let assign30720_e45641: f64 = (locals.var_sp_s_pc + assign30720_e45640);
        let assign30720_e45642: f64 = (locals.var_sp_s_qc / assign30720_e45641);
        let assign30720_e45643: f64 = (2.0 * assign30720_e45642);
        let assign30720_e45644: f64 = (assign30720_e45635 - assign30720_e45643);
        (assign30720_e45644, ((-locals.var_sp_s_y0_dn3) - (2.0 * (((locals.var_sp_s_qc_dn3 * assign30720_e45641) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn3 + (locals.var_sp_s_temp_dn3 / (2.0 * assign30720_e45640))))) / (assign30720_e45641 * assign30720_e45641)))), ((-locals.var_sp_s_y0_dn4) - (2.0 * (((locals.var_sp_s_qc_dn4 * assign30720_e45641) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn4 + (locals.var_sp_s_temp_dn4 / (2.0 * assign30720_e45640))))) / (assign30720_e45641 * assign30720_e45641)))), ((-locals.var_sp_s_y0_dn5) - (2.0 * (((locals.var_sp_s_qc_dn5 * assign30720_e45641) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn5 + (locals.var_sp_s_temp_dn5 / (2.0 * assign30720_e45640))))) / (assign30720_e45641 * assign30720_e45641)))), ((-locals.var_sp_s_y0_dn6) - (2.0 * (((locals.var_sp_s_qc_dn6 * assign30720_e45641) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn6 + (locals.var_sp_s_temp_dn6 / (2.0 * assign30720_e45640))))) / (assign30720_e45641 * assign30720_e45641)))), ((-locals.var_sp_s_y0_dn7) - (2.0 * (((locals.var_sp_s_qc_dn7 * assign30720_e45641) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn7 + (locals.var_sp_s_temp_dn7 / (2.0 * assign30720_e45640))))) / (assign30720_e45641 * assign30720_e45641)))), ((-locals.var_sp_s_y0_dn8) - (2.0 * (((locals.var_sp_s_qc_dn8 * assign30720_e45641) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn8 + (locals.var_sp_s_temp_dn8 / (2.0 * assign30720_e45640))))) / (assign30720_e45641 * assign30720_e45641)))), ((-locals.var_sp_s_y0_dn9) - (2.0 * (((locals.var_sp_s_qc_dn9 * assign30720_e45641) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn9 + (locals.var_sp_s_temp_dn9 / (2.0 * assign30720_e45640))))) / (assign30720_e45641 * assign30720_e45641)))), ((-locals.var_sp_s_y0_dn10) - (2.0 * (((locals.var_sp_s_qc_dn10 * assign30720_e45641) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn10 + (locals.var_sp_s_temp_dn10 / (2.0 * assign30720_e45640))))) / (assign30720_e45641 * assign30720_e45641)))), ((-locals.var_sp_s_y0_dn11) - (2.0 * (((locals.var_sp_s_qc_dn11 * assign30720_e45641) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn11 + (locals.var_sp_s_temp_dn11 / (2.0 * assign30720_e45640))))) / (assign30720_e45641 * assign30720_e45641)))),)
    } else {
        (locals.var_sp_dd, locals.var_sp_dd_dn3, locals.var_sp_dd_dn4, locals.var_sp_dd_dn5, locals.var_sp_dd_dn6, locals.var_sp_dd_dn7, locals.var_sp_dd_dn8, locals.var_sp_dd_dn9, locals.var_sp_dd_dn10, locals.var_sp_dd_dn11,)
    }
};
        locals.var_sp_dd = assign30720_e45646;
        locals.var_sp_dd_dn3 = assign30720_e45646_d_n3;
        locals.var_sp_dd_dn4 = assign30720_e45646_d_n4;
        locals.var_sp_dd_dn5 = assign30720_e45646_d_n5;
        locals.var_sp_dd_dn6 = assign30720_e45646_d_n6;
        locals.var_sp_dd_dn7 = assign30720_e45646_d_n7;
        locals.var_sp_dd_dn8 = assign30720_e45646_d_n8;
        locals.var_sp_dd_dn9 = assign30720_e45646_d_n9;
        locals.var_sp_dd_dn10 = assign30720_e45646_d_n10;
        locals.var_sp_dd_dn11 = assign30720_e45646_d_n11;
        locals.var_sp_dd_rv = 0.0;

        let (assign30730_e45668, assign30730_e45668_d_n3, assign30730_e45668_d_n4, assign30730_e45668_d_n5, assign30730_e45668_d_n6, assign30730_e45668_d_n7, assign30730_e45668_d_n8, assign30730_e45668_d_n9, assign30730_e45668_d_n10, assign30730_e45668_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30730_e45664: f64 = (locals.var_gam * 0.7324648775608221);
        let assign30730_e45665: f64 = (1.25 + assign30730_e45664);
        let assign30730_e45666: f64 = (1.0 / assign30730_e45665);
        (assign30730_e45666, (-((locals.var_gam_dn3 * 0.7324648775608221) / (assign30730_e45665 * assign30730_e45665))), (-((locals.var_gam_dn4 * 0.7324648775608221) / (assign30730_e45665 * assign30730_e45665))), (-((locals.var_gam_dn5 * 0.7324648775608221) / (assign30730_e45665 * assign30730_e45665))), (-((locals.var_gam_dn6 * 0.7324648775608221) / (assign30730_e45665 * assign30730_e45665))), (-((locals.var_gam_dn7 * 0.7324648775608221) / (assign30730_e45665 * assign30730_e45665))), (-((locals.var_gam_dn8 * 0.7324648775608221) / (assign30730_e45665 * assign30730_e45665))), (-((locals.var_gam_dn9 * 0.7324648775608221) / (assign30730_e45665 * assign30730_e45665))), (-((locals.var_gam_dn10 * 0.7324648775608221) / (assign30730_e45665 * assign30730_e45665))), (-((locals.var_gam_dn11 * 0.7324648775608221) / (assign30730_e45665 * assign30730_e45665))),)
    } else {
        (locals.var_sp_xg1, locals.var_sp_xg1_dn3, locals.var_sp_xg1_dn4, locals.var_sp_xg1_dn5, locals.var_sp_xg1_dn6, locals.var_sp_xg1_dn7, locals.var_sp_xg1_dn8, locals.var_sp_xg1_dn9, locals.var_sp_xg1_dn10, locals.var_sp_xg1_dn11,)
    }
};
        locals.var_sp_xg1 = assign30730_e45668;
        locals.var_sp_xg1_dn3 = assign30730_e45668_d_n3;
        locals.var_sp_xg1_dn4 = assign30730_e45668_d_n4;
        locals.var_sp_xg1_dn5 = assign30730_e45668_d_n5;
        locals.var_sp_xg1_dn6 = assign30730_e45668_d_n6;
        locals.var_sp_xg1_dn7 = assign30730_e45668_d_n7;
        locals.var_sp_xg1_dn8 = assign30730_e45668_d_n8;
        locals.var_sp_xg1_dn9 = assign30730_e45668_d_n9;
        locals.var_sp_xg1_dn10 = assign30730_e45668_d_n10;
        locals.var_sp_xg1_dn11 = assign30730_e45668_d_n11;
        locals.var_sp_xg1_rv = 0.0;

        let (assign30740_e45692, assign30740_e45692_d_n3, assign30740_e45692_d_n4, assign30740_e45692_d_n5, assign30740_e45692_d_n6, assign30740_e45692_d_n7, assign30740_e45692_d_n8, assign30740_e45692_d_n9, assign30740_e45692_d_n10, assign30740_e45692_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30740_e45684: f64 = (locals.var_x1 * 1.25);
        let assign30740_e45686: f64 = (assign30740_e45684 * locals.var_sp_xg1);
        let assign30740_e45688: f64 = (assign30740_e45686 - 1.0);
        let assign30740_e45690: f64 = (assign30740_e45688 * locals.var_sp_xg1);
        (assign30740_e45690, (((((locals.var_x1_dn3 * 1.25) * locals.var_sp_xg1) + (assign30740_e45684 * locals.var_sp_xg1_dn3)) * locals.var_sp_xg1) + (assign30740_e45688 * locals.var_sp_xg1_dn3)), (((((locals.var_x1_dn4 * 1.25) * locals.var_sp_xg1) + (assign30740_e45684 * locals.var_sp_xg1_dn4)) * locals.var_sp_xg1) + (assign30740_e45688 * locals.var_sp_xg1_dn4)), (((((locals.var_x1_dn5 * 1.25) * locals.var_sp_xg1) + (assign30740_e45684 * locals.var_sp_xg1_dn5)) * locals.var_sp_xg1) + (assign30740_e45688 * locals.var_sp_xg1_dn5)), (((((locals.var_x1_dn6 * 1.25) * locals.var_sp_xg1) + (assign30740_e45684 * locals.var_sp_xg1_dn6)) * locals.var_sp_xg1) + (assign30740_e45688 * locals.var_sp_xg1_dn6)), (((((locals.var_x1_dn7 * 1.25) * locals.var_sp_xg1) + (assign30740_e45684 * locals.var_sp_xg1_dn7)) * locals.var_sp_xg1) + (assign30740_e45688 * locals.var_sp_xg1_dn7)), (((((locals.var_x1_dn8 * 1.25) * locals.var_sp_xg1) + (assign30740_e45684 * locals.var_sp_xg1_dn8)) * locals.var_sp_xg1) + (assign30740_e45688 * locals.var_sp_xg1_dn8)), (((((locals.var_x1_dn9 * 1.25) * locals.var_sp_xg1) + (assign30740_e45684 * locals.var_sp_xg1_dn9)) * locals.var_sp_xg1) + (assign30740_e45688 * locals.var_sp_xg1_dn9)), (((((locals.var_x1_dn10 * 1.25) * locals.var_sp_xg1) + (assign30740_e45684 * locals.var_sp_xg1_dn10)) * locals.var_sp_xg1) + (assign30740_e45688 * locals.var_sp_xg1_dn10)), (((((locals.var_x1_dn11 * 1.25) * locals.var_sp_xg1) + (assign30740_e45684 * locals.var_sp_xg1_dn11)) * locals.var_sp_xg1) + (assign30740_e45688 * locals.var_sp_xg1_dn11)),)
    } else {
        (locals.var_sp_s_a_fac, locals.var_sp_s_a_fac_dn3, locals.var_sp_s_a_fac_dn4, locals.var_sp_s_a_fac_dn5, locals.var_sp_s_a_fac_dn6, locals.var_sp_s_a_fac_dn7, locals.var_sp_s_a_fac_dn8, locals.var_sp_s_a_fac_dn9, locals.var_sp_s_a_fac_dn10, locals.var_sp_s_a_fac_dn11,)
    }
};
        locals.var_sp_s_a_fac = assign30740_e45692;
        locals.var_sp_s_a_fac_dn3 = assign30740_e45692_d_n3;
        locals.var_sp_s_a_fac_dn4 = assign30740_e45692_d_n4;
        locals.var_sp_s_a_fac_dn5 = assign30740_e45692_d_n5;
        locals.var_sp_s_a_fac_dn6 = assign30740_e45692_d_n6;
        locals.var_sp_s_a_fac_dn7 = assign30740_e45692_d_n7;
        locals.var_sp_s_a_fac_dn8 = assign30740_e45692_d_n8;
        locals.var_sp_s_a_fac_dn9 = assign30740_e45692_d_n9;
        locals.var_sp_s_a_fac_dn10 = assign30740_e45692_d_n10;
        locals.var_sp_s_a_fac_dn11 = assign30740_e45692_d_n11;
        locals.var_sp_s_a_fac_rv = 0.0;

        let (assign30750_e45716, assign30750_e45716_d_n3, assign30750_e45716_d_n4, assign30750_e45716_d_n5, assign30750_e45716_d_n6, assign30750_e45716_d_n7, assign30750_e45716_d_n8, assign30750_e45716_d_n9, assign30750_e45716_d_n10, assign30750_e45716_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30750_e45708: f64 = (locals.var_vgfb1 * locals.var_inv_x1);
        let assign30750_e45712: f64 = (locals.var_sp_s_a_fac * locals.var_vgfb1);
        let assign30750_e45713: f64 = (1.0 + assign30750_e45712);
        let assign30750_e45714: f64 = (assign30750_e45708 * assign30750_e45713);
        (assign30750_e45714, ((((locals.var_vgfb1_dn3 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn3)) * assign30750_e45713) + (assign30750_e45708 * ((locals.var_sp_s_a_fac_dn3 * locals.var_vgfb1) + (locals.var_sp_s_a_fac * locals.var_vgfb1_dn3)))), ((((locals.var_vgfb1_dn4 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn4)) * assign30750_e45713) + (assign30750_e45708 * ((locals.var_sp_s_a_fac_dn4 * locals.var_vgfb1) + (locals.var_sp_s_a_fac * locals.var_vgfb1_dn4)))), ((((locals.var_vgfb1_dn5 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn5)) * assign30750_e45713) + (assign30750_e45708 * ((locals.var_sp_s_a_fac_dn5 * locals.var_vgfb1) + (locals.var_sp_s_a_fac * locals.var_vgfb1_dn5)))), ((((locals.var_vgfb1_dn6 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn6)) * assign30750_e45713) + (assign30750_e45708 * ((locals.var_sp_s_a_fac_dn6 * locals.var_vgfb1) + (locals.var_sp_s_a_fac * locals.var_vgfb1_dn6)))), ((((locals.var_vgfb1_dn7 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn7)) * assign30750_e45713) + (assign30750_e45708 * ((locals.var_sp_s_a_fac_dn7 * locals.var_vgfb1) + (locals.var_sp_s_a_fac * locals.var_vgfb1_dn7)))), ((((locals.var_vgfb1_dn8 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn8)) * assign30750_e45713) + (assign30750_e45708 * ((locals.var_sp_s_a_fac_dn8 * locals.var_vgfb1) + (locals.var_sp_s_a_fac * locals.var_vgfb1_dn8)))), ((((locals.var_vgfb1_dn9 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn9)) * assign30750_e45713) + (assign30750_e45708 * ((locals.var_sp_s_a_fac_dn9 * locals.var_vgfb1) + (locals.var_sp_s_a_fac * locals.var_vgfb1_dn9)))), ((((locals.var_vgfb1_dn10 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn10)) * assign30750_e45713) + (assign30750_e45708 * ((locals.var_sp_s_a_fac_dn10 * locals.var_vgfb1) + (locals.var_sp_s_a_fac * locals.var_vgfb1_dn10)))), ((((locals.var_vgfb1_dn11 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn11)) * assign30750_e45713) + (assign30750_e45708 * ((locals.var_sp_s_a_fac_dn11 * locals.var_vgfb1) + (locals.var_sp_s_a_fac * locals.var_vgfb1_dn11)))),)
    } else {
        (locals.var_sp_s_xbar, locals.var_sp_s_xbar_dn3, locals.var_sp_s_xbar_dn4, locals.var_sp_s_xbar_dn5, locals.var_sp_s_xbar_dn6, locals.var_sp_s_xbar_dn7, locals.var_sp_s_xbar_dn8, locals.var_sp_s_xbar_dn9, locals.var_sp_s_xbar_dn10, locals.var_sp_s_xbar_dn11,)
    }
};
        locals.var_sp_s_xbar = assign30750_e45716;
        locals.var_sp_s_xbar_dn3 = assign30750_e45716_d_n3;
        locals.var_sp_s_xbar_dn4 = assign30750_e45716_d_n4;
        locals.var_sp_s_xbar_dn5 = assign30750_e45716_d_n5;
        locals.var_sp_s_xbar_dn6 = assign30750_e45716_d_n6;
        locals.var_sp_s_xbar_dn7 = assign30750_e45716_d_n7;
        locals.var_sp_s_xbar_dn8 = assign30750_e45716_d_n8;
        locals.var_sp_s_xbar_dn9 = assign30750_e45716_d_n9;
        locals.var_sp_s_xbar_dn10 = assign30750_e45716_d_n10;
        locals.var_sp_s_xbar_dn11 = assign30750_e45716_d_n11;
        locals.var_sp_s_xbar_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_90(
        locals: &mut StampLocals,
    ) {
        let (assign30760_e45734, assign30760_e45734_d_n3, assign30760_e45734_d_n4, assign30760_e45734_d_n5, assign30760_e45734_d_n6, assign30760_e45734_d_n7, assign30760_e45734_d_n8, assign30760_e45734_d_n9, assign30760_e45734_d_n10, assign30760_e45734_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30760_e45731: f64 = (-locals.var_sp_s_xbar);
        let assign30760_e45732: f64 = { let limited_exp_arg = assign30760_e45731; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign30760_e45732, ({ let limited_exp_arg = assign30760_e45731; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_xbar_dn3)), ({ let limited_exp_arg = assign30760_e45731; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_xbar_dn4)), ({ let limited_exp_arg = assign30760_e45731; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_xbar_dn5)), ({ let limited_exp_arg = assign30760_e45731; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_xbar_dn6)), ({ let limited_exp_arg = assign30760_e45731; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_xbar_dn7)), ({ let limited_exp_arg = assign30760_e45731; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_xbar_dn8)), ({ let limited_exp_arg = assign30760_e45731; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_xbar_dn9)), ({ let limited_exp_arg = assign30760_e45731; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_xbar_dn10)), ({ let limited_exp_arg = assign30760_e45731; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_xbar_dn11)),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn3, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9, locals.var_sp_s_temp_dn10, locals.var_sp_s_temp_dn11,)
    }
};
        locals.var_sp_s_temp = assign30760_e45734;
        locals.var_sp_s_temp_dn3 = assign30760_e45734_d_n3;
        locals.var_sp_s_temp_dn4 = assign30760_e45734_d_n4;
        locals.var_sp_s_temp_dn5 = assign30760_e45734_d_n5;
        locals.var_sp_s_temp_dn6 = assign30760_e45734_d_n6;
        locals.var_sp_s_temp_dn7 = assign30760_e45734_d_n7;
        locals.var_sp_s_temp_dn8 = assign30760_e45734_d_n8;
        locals.var_sp_s_temp_dn9 = assign30760_e45734_d_n9;
        locals.var_sp_s_temp_dn10 = assign30760_e45734_d_n10;
        locals.var_sp_s_temp_dn11 = assign30760_e45734_d_n11;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign30770_e45752, assign30770_e45752_d_n3, assign30770_e45752_d_n4, assign30770_e45752_d_n5, assign30770_e45752_d_n6, assign30770_e45752_d_n7, assign30770_e45752_d_n8, assign30770_e45752_d_n9, assign30770_e45752_d_n10, assign30770_e45752_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30770_e45750: f64 = (1.0 - locals.var_sp_s_temp);
        (assign30770_e45750, (-locals.var_sp_s_temp_dn3), (-locals.var_sp_s_temp_dn4), (-locals.var_sp_s_temp_dn5), (-locals.var_sp_s_temp_dn6), (-locals.var_sp_s_temp_dn7), (-locals.var_sp_s_temp_dn8), (-locals.var_sp_s_temp_dn9), (-locals.var_sp_s_temp_dn10), (-locals.var_sp_s_temp_dn11),)
    } else {
        (locals.var_sp_s_w, locals.var_sp_s_w_dn3, locals.var_sp_s_w_dn4, locals.var_sp_s_w_dn5, locals.var_sp_s_w_dn6, locals.var_sp_s_w_dn7, locals.var_sp_s_w_dn8, locals.var_sp_s_w_dn9, locals.var_sp_s_w_dn10, locals.var_sp_s_w_dn11,)
    }
};
        locals.var_sp_s_w = assign30770_e45752;
        locals.var_sp_s_w_dn3 = assign30770_e45752_d_n3;
        locals.var_sp_s_w_dn4 = assign30770_e45752_d_n4;
        locals.var_sp_s_w_dn5 = assign30770_e45752_d_n5;
        locals.var_sp_s_w_dn6 = assign30770_e45752_d_n6;
        locals.var_sp_s_w_dn7 = assign30770_e45752_d_n7;
        locals.var_sp_s_w_dn8 = assign30770_e45752_d_n8;
        locals.var_sp_s_w_dn9 = assign30770_e45752_d_n9;
        locals.var_sp_s_w_dn10 = assign30770_e45752_d_n10;
        locals.var_sp_s_w_dn11 = assign30770_e45752_d_n11;
        locals.var_sp_s_w_rv = 0.0;

        let (assign30780_e45783, assign30780_e45783_d_n3, assign30780_e45783_d_n4, assign30780_e45783_d_n5, assign30780_e45783_d_n6, assign30780_e45783_d_n7, assign30780_e45783_d_n8, assign30780_e45783_d_n9, assign30780_e45783_d_n10, assign30780_e45783_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30780_e45769: f64 = (locals.var_gam2 * 0.5);
        let assign30780_e45770: f64 = (locals.var_vgfb1 + assign30780_e45769);
        let assign30780_e45775: f64 = (locals.var_gam2 * 0.25);
        let assign30780_e45776: f64 = (locals.var_vgfb1 + assign30780_e45775);
        let assign30780_e45778: f64 = (assign30780_e45776 - locals.var_sp_s_w);
        let assign30780_e45779: f64 = (assign30780_e45778).sqrt();
        let assign30780_e45780: f64 = (locals.var_gam * assign30780_e45779);
        let assign30780_e45781: f64 = (assign30780_e45770 - assign30780_e45780);
        (assign30780_e45781, ((locals.var_vgfb1_dn3 + (locals.var_gam2_dn3 * 0.5)) - ((locals.var_gam_dn3 * assign30780_e45779) + (locals.var_gam * (((locals.var_vgfb1_dn3 + (locals.var_gam2_dn3 * 0.25)) - locals.var_sp_s_w_dn3) / (2.0 * assign30780_e45779))))), ((locals.var_vgfb1_dn4 + (locals.var_gam2_dn4 * 0.5)) - ((locals.var_gam_dn4 * assign30780_e45779) + (locals.var_gam * (((locals.var_vgfb1_dn4 + (locals.var_gam2_dn4 * 0.25)) - locals.var_sp_s_w_dn4) / (2.0 * assign30780_e45779))))), ((locals.var_vgfb1_dn5 + (locals.var_gam2_dn5 * 0.5)) - ((locals.var_gam_dn5 * assign30780_e45779) + (locals.var_gam * (((locals.var_vgfb1_dn5 + (locals.var_gam2_dn5 * 0.25)) - locals.var_sp_s_w_dn5) / (2.0 * assign30780_e45779))))), ((locals.var_vgfb1_dn6 + (locals.var_gam2_dn6 * 0.5)) - ((locals.var_gam_dn6 * assign30780_e45779) + (locals.var_gam * (((locals.var_vgfb1_dn6 + (locals.var_gam2_dn6 * 0.25)) - locals.var_sp_s_w_dn6) / (2.0 * assign30780_e45779))))), ((locals.var_vgfb1_dn7 + (locals.var_gam2_dn7 * 0.5)) - ((locals.var_gam_dn7 * assign30780_e45779) + (locals.var_gam * (((locals.var_vgfb1_dn7 + (locals.var_gam2_dn7 * 0.25)) - locals.var_sp_s_w_dn7) / (2.0 * assign30780_e45779))))), ((locals.var_vgfb1_dn8 + (locals.var_gam2_dn8 * 0.5)) - ((locals.var_gam_dn8 * assign30780_e45779) + (locals.var_gam * (((locals.var_vgfb1_dn8 + (locals.var_gam2_dn8 * 0.25)) - locals.var_sp_s_w_dn8) / (2.0 * assign30780_e45779))))), ((locals.var_vgfb1_dn9 + (locals.var_gam2_dn9 * 0.5)) - ((locals.var_gam_dn9 * assign30780_e45779) + (locals.var_gam * (((locals.var_vgfb1_dn9 + (locals.var_gam2_dn9 * 0.25)) - locals.var_sp_s_w_dn9) / (2.0 * assign30780_e45779))))), ((locals.var_vgfb1_dn10 + (locals.var_gam2_dn10 * 0.5)) - ((locals.var_gam_dn10 * assign30780_e45779) + (locals.var_gam * (((locals.var_vgfb1_dn10 + (locals.var_gam2_dn10 * 0.25)) - locals.var_sp_s_w_dn10) / (2.0 * assign30780_e45779))))), ((locals.var_vgfb1_dn11 + (locals.var_gam2_dn11 * 0.5)) - ((locals.var_gam_dn11 * assign30780_e45779) + (locals.var_gam * (((locals.var_vgfb1_dn11 + (locals.var_gam2_dn11 * 0.25)) - locals.var_sp_s_w_dn11) / (2.0 * assign30780_e45779))))),)
    } else {
        (locals.var_sp_s_x1, locals.var_sp_s_x1_dn3, locals.var_sp_s_x1_dn4, locals.var_sp_s_x1_dn5, locals.var_sp_s_x1_dn6, locals.var_sp_s_x1_dn7, locals.var_sp_s_x1_dn8, locals.var_sp_s_x1_dn9, locals.var_sp_s_x1_dn10, locals.var_sp_s_x1_dn11,)
    }
};
        locals.var_sp_s_x1 = assign30780_e45783;
        locals.var_sp_s_x1_dn3 = assign30780_e45783_d_n3;
        locals.var_sp_s_x1_dn4 = assign30780_e45783_d_n4;
        locals.var_sp_s_x1_dn5 = assign30780_e45783_d_n5;
        locals.var_sp_s_x1_dn6 = assign30780_e45783_d_n6;
        locals.var_sp_s_x1_dn7 = assign30780_e45783_d_n7;
        locals.var_sp_s_x1_dn8 = assign30780_e45783_d_n8;
        locals.var_sp_s_x1_dn9 = assign30780_e45783_d_n9;
        locals.var_sp_s_x1_dn10 = assign30780_e45783_d_n10;
        locals.var_sp_s_x1_dn11 = assign30780_e45783_d_n11;
        locals.var_sp_s_x1_rv = 0.0;

        let (assign30790_e45801, assign30790_e45801_d_n3, assign30790_e45801_d_n4, assign30790_e45801_d_n5, assign30790_e45801_d_n6, assign30790_e45801_d_n7, assign30790_e45801_d_n8, assign30790_e45801_d_n9, assign30790_e45801_d_n10, assign30790_e45801_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30790_e45799: f64 = (locals.var_phisf + 3.0);
        (assign30790_e45799, locals.var_phisf_dn3, locals.var_phisf_dn4, locals.var_phisf_dn5, locals.var_phisf_dn6, locals.var_phisf_dn7, locals.var_phisf_dn8, locals.var_phisf_dn9, locals.var_phisf_dn10, locals.var_phisf_dn11,)
    } else {
        (locals.var_sp_s_bx, locals.var_sp_s_bx_dn3, locals.var_sp_s_bx_dn4, locals.var_sp_s_bx_dn5, locals.var_sp_s_bx_dn6, locals.var_sp_s_bx_dn7, locals.var_sp_s_bx_dn8, locals.var_sp_s_bx_dn9, locals.var_sp_s_bx_dn10, locals.var_sp_s_bx_dn11,)
    }
};
        locals.var_sp_s_bx = assign30790_e45801;
        locals.var_sp_s_bx_dn3 = assign30790_e45801_d_n3;
        locals.var_sp_s_bx_dn4 = assign30790_e45801_d_n4;
        locals.var_sp_s_bx_dn5 = assign30790_e45801_d_n5;
        locals.var_sp_s_bx_dn6 = assign30790_e45801_d_n6;
        locals.var_sp_s_bx_dn7 = assign30790_e45801_d_n7;
        locals.var_sp_s_bx_dn8 = assign30790_e45801_d_n8;
        locals.var_sp_s_bx_dn9 = assign30790_e45801_d_n9;
        locals.var_sp_s_bx_dn10 = assign30790_e45801_d_n10;
        locals.var_sp_s_bx_dn11 = assign30790_e45801_d_n11;
        locals.var_sp_s_bx_rv = 0.0;

        let (assign30800_e45843, assign30800_e45843_d_n3, assign30800_e45843_d_n4, assign30800_e45843_d_n5, assign30800_e45843_d_n6, assign30800_e45843_d_n7, assign30800_e45843_d_n8, assign30800_e45843_d_n9, assign30800_e45843_d_n10, assign30800_e45843_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30800_e45818: f64 = (locals.var_sp_s_x1 + locals.var_sp_s_bx);
        let assign30800_e45821: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
        let assign30800_e45824: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
        let assign30800_e45825: f64 = (assign30800_e45821 * assign30800_e45824);
        let assign30800_e45827: f64 = (assign30800_e45825 + 5.0);
        let assign30800_e45828: f64 = (assign30800_e45827).sqrt();
        let assign30800_e45829: f64 = (assign30800_e45818 - assign30800_e45828);
        let assign30800_e45830: f64 = (0.5 * assign30800_e45829);
        let assign30800_e45835: f64 = (locals.var_sp_s_bx * locals.var_sp_s_bx);
        let assign30800_e45837: f64 = (assign30800_e45835 + 5.0);
        let assign30800_e45838: f64 = (assign30800_e45837).sqrt();
        let assign30800_e45839: f64 = (locals.var_sp_s_bx - assign30800_e45838);
        let assign30800_e45840: f64 = (0.5 * assign30800_e45839);
        let assign30800_e45841: f64 = (assign30800_e45830 - assign30800_e45840);
        (assign30800_e45841, ((0.5 * ((locals.var_sp_s_x1_dn3 + locals.var_sp_s_bx_dn3) - ((((locals.var_sp_s_x1_dn3 - locals.var_sp_s_bx_dn3) * assign30800_e45824) + (assign30800_e45821 * (locals.var_sp_s_x1_dn3 - locals.var_sp_s_bx_dn3))) / (2.0 * assign30800_e45828)))) - (0.5 * (locals.var_sp_s_bx_dn3 - (((locals.var_sp_s_bx_dn3 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn3)) / (2.0 * assign30800_e45838))))), ((0.5 * ((locals.var_sp_s_x1_dn4 + locals.var_sp_s_bx_dn4) - ((((locals.var_sp_s_x1_dn4 - locals.var_sp_s_bx_dn4) * assign30800_e45824) + (assign30800_e45821 * (locals.var_sp_s_x1_dn4 - locals.var_sp_s_bx_dn4))) / (2.0 * assign30800_e45828)))) - (0.5 * (locals.var_sp_s_bx_dn4 - (((locals.var_sp_s_bx_dn4 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn4)) / (2.0 * assign30800_e45838))))), ((0.5 * ((locals.var_sp_s_x1_dn5 + locals.var_sp_s_bx_dn5) - ((((locals.var_sp_s_x1_dn5 - locals.var_sp_s_bx_dn5) * assign30800_e45824) + (assign30800_e45821 * (locals.var_sp_s_x1_dn5 - locals.var_sp_s_bx_dn5))) / (2.0 * assign30800_e45828)))) - (0.5 * (locals.var_sp_s_bx_dn5 - (((locals.var_sp_s_bx_dn5 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn5)) / (2.0 * assign30800_e45838))))), ((0.5 * ((locals.var_sp_s_x1_dn6 + locals.var_sp_s_bx_dn6) - ((((locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6) * assign30800_e45824) + (assign30800_e45821 * (locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6))) / (2.0 * assign30800_e45828)))) - (0.5 * (locals.var_sp_s_bx_dn6 - (((locals.var_sp_s_bx_dn6 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn6)) / (2.0 * assign30800_e45838))))), ((0.5 * ((locals.var_sp_s_x1_dn7 + locals.var_sp_s_bx_dn7) - ((((locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7) * assign30800_e45824) + (assign30800_e45821 * (locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7))) / (2.0 * assign30800_e45828)))) - (0.5 * (locals.var_sp_s_bx_dn7 - (((locals.var_sp_s_bx_dn7 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn7)) / (2.0 * assign30800_e45838))))), ((0.5 * ((locals.var_sp_s_x1_dn8 + locals.var_sp_s_bx_dn8) - ((((locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8) * assign30800_e45824) + (assign30800_e45821 * (locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8))) / (2.0 * assign30800_e45828)))) - (0.5 * (locals.var_sp_s_bx_dn8 - (((locals.var_sp_s_bx_dn8 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn8)) / (2.0 * assign30800_e45838))))), ((0.5 * ((locals.var_sp_s_x1_dn9 + locals.var_sp_s_bx_dn9) - ((((locals.var_sp_s_x1_dn9 - locals.var_sp_s_bx_dn9) * assign30800_e45824) + (assign30800_e45821 * (locals.var_sp_s_x1_dn9 - locals.var_sp_s_bx_dn9))) / (2.0 * assign30800_e45828)))) - (0.5 * (locals.var_sp_s_bx_dn9 - (((locals.var_sp_s_bx_dn9 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn9)) / (2.0 * assign30800_e45838))))), ((0.5 * ((locals.var_sp_s_x1_dn10 + locals.var_sp_s_bx_dn10) - ((((locals.var_sp_s_x1_dn10 - locals.var_sp_s_bx_dn10) * assign30800_e45824) + (assign30800_e45821 * (locals.var_sp_s_x1_dn10 - locals.var_sp_s_bx_dn10))) / (2.0 * assign30800_e45828)))) - (0.5 * (locals.var_sp_s_bx_dn10 - (((locals.var_sp_s_bx_dn10 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn10)) / (2.0 * assign30800_e45838))))), ((0.5 * ((locals.var_sp_s_x1_dn11 + locals.var_sp_s_bx_dn11) - ((((locals.var_sp_s_x1_dn11 - locals.var_sp_s_bx_dn11) * assign30800_e45824) + (assign30800_e45821 * (locals.var_sp_s_x1_dn11 - locals.var_sp_s_bx_dn11))) / (2.0 * assign30800_e45828)))) - (0.5 * (locals.var_sp_s_bx_dn11 - (((locals.var_sp_s_bx_dn11 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn11)) / (2.0 * assign30800_e45838))))),)
    } else {
        (locals.var_sp_s_eta, locals.var_sp_s_eta_dn3, locals.var_sp_s_eta_dn4, locals.var_sp_s_eta_dn5, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8, locals.var_sp_s_eta_dn9, locals.var_sp_s_eta_dn10, locals.var_sp_s_eta_dn11,)
    }
};
        locals.var_sp_s_eta = assign30800_e45843;
        locals.var_sp_s_eta_dn3 = assign30800_e45843_d_n3;
        locals.var_sp_s_eta_dn4 = assign30800_e45843_d_n4;
        locals.var_sp_s_eta_dn5 = assign30800_e45843_d_n5;
        locals.var_sp_s_eta_dn6 = assign30800_e45843_d_n6;
        locals.var_sp_s_eta_dn7 = assign30800_e45843_d_n7;
        locals.var_sp_s_eta_dn8 = assign30800_e45843_d_n8;
        locals.var_sp_s_eta_dn9 = assign30800_e45843_d_n9;
        locals.var_sp_s_eta_dn10 = assign30800_e45843_d_n10;
        locals.var_sp_s_eta_dn11 = assign30800_e45843_d_n11;
        locals.var_sp_s_eta_rv = 0.0;

        let (assign30810_e45861, assign30810_e45861_d_n3, assign30810_e45861_d_n4, assign30810_e45861_d_n5, assign30810_e45861_d_n6, assign30810_e45861_d_n7, assign30810_e45861_d_n8, assign30810_e45861_d_n9, assign30810_e45861_d_n10, assign30810_e45861_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30810_e45859: f64 = (locals.var_vgfb1 - locals.var_sp_s_eta);
        (assign30810_e45859, (locals.var_vgfb1_dn3 - locals.var_sp_s_eta_dn3), (locals.var_vgfb1_dn4 - locals.var_sp_s_eta_dn4), (locals.var_vgfb1_dn5 - locals.var_sp_s_eta_dn5), (locals.var_vgfb1_dn6 - locals.var_sp_s_eta_dn6), (locals.var_vgfb1_dn7 - locals.var_sp_s_eta_dn7), (locals.var_vgfb1_dn8 - locals.var_sp_s_eta_dn8), (locals.var_vgfb1_dn9 - locals.var_sp_s_eta_dn9), (locals.var_vgfb1_dn10 - locals.var_sp_s_eta_dn10), (locals.var_vgfb1_dn11 - locals.var_sp_s_eta_dn11),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn3, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9, locals.var_sp_s_temp_dn10, locals.var_sp_s_temp_dn11,)
    }
};
        locals.var_sp_s_temp = assign30810_e45861;
        locals.var_sp_s_temp_dn3 = assign30810_e45861_d_n3;
        locals.var_sp_s_temp_dn4 = assign30810_e45861_d_n4;
        locals.var_sp_s_temp_dn5 = assign30810_e45861_d_n5;
        locals.var_sp_s_temp_dn6 = assign30810_e45861_d_n6;
        locals.var_sp_s_temp_dn7 = assign30810_e45861_d_n7;
        locals.var_sp_s_temp_dn8 = assign30810_e45861_d_n8;
        locals.var_sp_s_temp_dn9 = assign30810_e45861_d_n9;
        locals.var_sp_s_temp_dn10 = assign30810_e45861_d_n10;
        locals.var_sp_s_temp_dn11 = assign30810_e45861_d_n11;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign30820_e45879, assign30820_e45879_d_n3, assign30820_e45879_d_n4, assign30820_e45879_d_n5, assign30820_e45879_d_n6, assign30820_e45879_d_n7, assign30820_e45879_d_n8, assign30820_e45879_d_n9, assign30820_e45879_d_n10, assign30820_e45879_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30820_e45876: f64 = (-locals.var_sp_s_eta);
        let assign30820_e45877: f64 = { let limited_exp_arg = assign30820_e45876; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign30820_e45877, ({ let limited_exp_arg = assign30820_e45876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_eta_dn3)), ({ let limited_exp_arg = assign30820_e45876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_eta_dn4)), ({ let limited_exp_arg = assign30820_e45876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_eta_dn5)), ({ let limited_exp_arg = assign30820_e45876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_eta_dn6)), ({ let limited_exp_arg = assign30820_e45876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_eta_dn7)), ({ let limited_exp_arg = assign30820_e45876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_eta_dn8)), ({ let limited_exp_arg = assign30820_e45876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_eta_dn9)), ({ let limited_exp_arg = assign30820_e45876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_eta_dn10)), ({ let limited_exp_arg = assign30820_e45876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_sp_s_eta_dn11)),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn3, locals.var_sp_s_temp1_dn4, locals.var_sp_s_temp1_dn5, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, locals.var_sp_s_temp1_dn9, locals.var_sp_s_temp1_dn10, locals.var_sp_s_temp1_dn11,)
    }
};
        locals.var_sp_s_temp1 = assign30820_e45879;
        locals.var_sp_s_temp1_dn3 = assign30820_e45879_d_n3;
        locals.var_sp_s_temp1_dn4 = assign30820_e45879_d_n4;
        locals.var_sp_s_temp1_dn5 = assign30820_e45879_d_n5;
        locals.var_sp_s_temp1_dn6 = assign30820_e45879_d_n6;
        locals.var_sp_s_temp1_dn7 = assign30820_e45879_d_n7;
        locals.var_sp_s_temp1_dn8 = assign30820_e45879_d_n8;
        locals.var_sp_s_temp1_dn9 = assign30820_e45879_d_n9;
        locals.var_sp_s_temp1_dn10 = assign30820_e45879_d_n10;
        locals.var_sp_s_temp1_dn11 = assign30820_e45879_d_n11;
        locals.var_sp_s_temp1_rv = 0.0;

        let (assign30830_e45901, assign30830_e45901_d_n3, assign30830_e45901_d_n4, assign30830_e45901_d_n5, assign30830_e45901_d_n6, assign30830_e45901_d_n7, assign30830_e45901_d_n8, assign30830_e45901_d_n9, assign30830_e45901_d_n10, assign30830_e45901_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30830_e45897: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
        let assign30830_e45898: f64 = (2.0 + assign30830_e45897);
        let assign30830_e45899: f64 = (1.0 / assign30830_e45898);
        (assign30830_e45899, (-(((locals.var_sp_s_eta_dn3 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn3)) / (assign30830_e45898 * assign30830_e45898))), (-(((locals.var_sp_s_eta_dn4 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn4)) / (assign30830_e45898 * assign30830_e45898))), (-(((locals.var_sp_s_eta_dn5 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn5)) / (assign30830_e45898 * assign30830_e45898))), (-(((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) / (assign30830_e45898 * assign30830_e45898))), (-(((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) / (assign30830_e45898 * assign30830_e45898))), (-(((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) / (assign30830_e45898 * assign30830_e45898))), (-(((locals.var_sp_s_eta_dn9 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn9)) / (assign30830_e45898 * assign30830_e45898))), (-(((locals.var_sp_s_eta_dn10 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn10)) / (assign30830_e45898 * assign30830_e45898))), (-(((locals.var_sp_s_eta_dn11 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn11)) / (assign30830_e45898 * assign30830_e45898))),)
    } else {
        (locals.var_sp_s_temp2, locals.var_sp_s_temp2_dn3, locals.var_sp_s_temp2_dn4, locals.var_sp_s_temp2_dn5, locals.var_sp_s_temp2_dn6, locals.var_sp_s_temp2_dn7, locals.var_sp_s_temp2_dn8, locals.var_sp_s_temp2_dn9, locals.var_sp_s_temp2_dn10, locals.var_sp_s_temp2_dn11,)
    }
};
        locals.var_sp_s_temp2 = assign30830_e45901;
        locals.var_sp_s_temp2_dn3 = assign30830_e45901_d_n3;
        locals.var_sp_s_temp2_dn4 = assign30830_e45901_d_n4;
        locals.var_sp_s_temp2_dn5 = assign30830_e45901_d_n5;
        locals.var_sp_s_temp2_dn6 = assign30830_e45901_d_n6;
        locals.var_sp_s_temp2_dn7 = assign30830_e45901_d_n7;
        locals.var_sp_s_temp2_dn8 = assign30830_e45901_d_n8;
        locals.var_sp_s_temp2_dn9 = assign30830_e45901_d_n9;
        locals.var_sp_s_temp2_dn10 = assign30830_e45901_d_n10;
        locals.var_sp_s_temp2_dn11 = assign30830_e45901_d_n11;
        locals.var_sp_s_temp2_rv = 0.0;

        let (assign30840_e45921, assign30840_e45921_d_n3, assign30840_e45921_d_n4, assign30840_e45921_d_n5, assign30840_e45921_d_n6, assign30840_e45921_d_n7, assign30840_e45921_d_n8, assign30840_e45921_d_n9, assign30840_e45921_d_n10, assign30840_e45921_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30840_e45917: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
        let assign30840_e45919: f64 = (assign30840_e45917 * locals.var_sp_s_temp2);
        (assign30840_e45919, ((((locals.var_sp_s_eta_dn3 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn3)) * locals.var_sp_s_temp2) + (assign30840_e45917 * locals.var_sp_s_temp2_dn3)), ((((locals.var_sp_s_eta_dn4 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn4)) * locals.var_sp_s_temp2) + (assign30840_e45917 * locals.var_sp_s_temp2_dn4)), ((((locals.var_sp_s_eta_dn5 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn5)) * locals.var_sp_s_temp2) + (assign30840_e45917 * locals.var_sp_s_temp2_dn5)), ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) * locals.var_sp_s_temp2) + (assign30840_e45917 * locals.var_sp_s_temp2_dn6)), ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) * locals.var_sp_s_temp2) + (assign30840_e45917 * locals.var_sp_s_temp2_dn7)), ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) * locals.var_sp_s_temp2) + (assign30840_e45917 * locals.var_sp_s_temp2_dn8)), ((((locals.var_sp_s_eta_dn9 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn9)) * locals.var_sp_s_temp2) + (assign30840_e45917 * locals.var_sp_s_temp2_dn9)), ((((locals.var_sp_s_eta_dn10 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn10)) * locals.var_sp_s_temp2) + (assign30840_e45917 * locals.var_sp_s_temp2_dn10)), ((((locals.var_sp_s_eta_dn11 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn11)) * locals.var_sp_s_temp2) + (assign30840_e45917 * locals.var_sp_s_temp2_dn11)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn3, locals.var_sp_s_xi0_dn4, locals.var_sp_s_xi0_dn5, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, locals.var_sp_s_xi0_dn9, locals.var_sp_s_xi0_dn10, locals.var_sp_s_xi0_dn11,)
    }
};
        locals.var_sp_s_xi0 = assign30840_e45921;
        locals.var_sp_s_xi0_dn3 = assign30840_e45921_d_n3;
        locals.var_sp_s_xi0_dn4 = assign30840_e45921_d_n4;
        locals.var_sp_s_xi0_dn5 = assign30840_e45921_d_n5;
        locals.var_sp_s_xi0_dn6 = assign30840_e45921_d_n6;
        locals.var_sp_s_xi0_dn7 = assign30840_e45921_d_n7;
        locals.var_sp_s_xi0_dn8 = assign30840_e45921_d_n8;
        locals.var_sp_s_xi0_dn9 = assign30840_e45921_d_n9;
        locals.var_sp_s_xi0_dn10 = assign30840_e45921_d_n10;
        locals.var_sp_s_xi0_dn11 = assign30840_e45921_d_n11;
        locals.var_sp_s_xi0_rv = 0.0;

        let (assign30850_e45943, assign30850_e45943_d_n3, assign30850_e45943_d_n4, assign30850_e45943_d_n5, assign30850_e45943_d_n6, assign30850_e45943_d_n7, assign30850_e45943_d_n8, assign30850_e45943_d_n9, assign30850_e45943_d_n10, assign30850_e45943_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30850_e45938: f64 = (locals.var_sp_s_eta * locals.var_sp_s_temp2);
        let assign30850_e45940: f64 = (assign30850_e45938 * locals.var_sp_s_temp2);
        let assign30850_e45941: f64 = (4.0 * assign30850_e45940);
        (assign30850_e45941, (4.0 * ((((locals.var_sp_s_eta_dn3 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn3)) * locals.var_sp_s_temp2) + (assign30850_e45938 * locals.var_sp_s_temp2_dn3))), (4.0 * ((((locals.var_sp_s_eta_dn4 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn4)) * locals.var_sp_s_temp2) + (assign30850_e45938 * locals.var_sp_s_temp2_dn4))), (4.0 * ((((locals.var_sp_s_eta_dn5 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn5)) * locals.var_sp_s_temp2) + (assign30850_e45938 * locals.var_sp_s_temp2_dn5))), (4.0 * ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign30850_e45938 * locals.var_sp_s_temp2_dn6))), (4.0 * ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign30850_e45938 * locals.var_sp_s_temp2_dn7))), (4.0 * ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign30850_e45938 * locals.var_sp_s_temp2_dn8))), (4.0 * ((((locals.var_sp_s_eta_dn9 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn9)) * locals.var_sp_s_temp2) + (assign30850_e45938 * locals.var_sp_s_temp2_dn9))), (4.0 * ((((locals.var_sp_s_eta_dn10 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn10)) * locals.var_sp_s_temp2) + (assign30850_e45938 * locals.var_sp_s_temp2_dn10))), (4.0 * ((((locals.var_sp_s_eta_dn11 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn11)) * locals.var_sp_s_temp2) + (assign30850_e45938 * locals.var_sp_s_temp2_dn11))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn3, locals.var_sp_s_xi1_dn4, locals.var_sp_s_xi1_dn5, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, locals.var_sp_s_xi1_dn9, locals.var_sp_s_xi1_dn10, locals.var_sp_s_xi1_dn11,)
    }
};
        locals.var_sp_s_xi1 = assign30850_e45943;
        locals.var_sp_s_xi1_dn3 = assign30850_e45943_d_n3;
        locals.var_sp_s_xi1_dn4 = assign30850_e45943_d_n4;
        locals.var_sp_s_xi1_dn5 = assign30850_e45943_d_n5;
        locals.var_sp_s_xi1_dn6 = assign30850_e45943_d_n6;
        locals.var_sp_s_xi1_dn7 = assign30850_e45943_d_n7;
        locals.var_sp_s_xi1_dn8 = assign30850_e45943_d_n8;
        locals.var_sp_s_xi1_dn9 = assign30850_e45943_d_n9;
        locals.var_sp_s_xi1_dn10 = assign30850_e45943_d_n10;
        locals.var_sp_s_xi1_dn11 = assign30850_e45943_d_n11;
        locals.var_sp_s_xi1_rv = 0.0;

        let (assign30860_e45969, assign30860_e45969_d_n3, assign30860_e45969_d_n4, assign30860_e45969_d_n5, assign30860_e45969_d_n6, assign30860_e45969_d_n7, assign30860_e45969_d_n8, assign30860_e45969_d_n9, assign30860_e45969_d_n10, assign30860_e45969_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30860_e45959: f64 = (8.0 * locals.var_sp_s_temp2);
        let assign30860_e45962: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign30860_e45963: f64 = (assign30860_e45959 - assign30860_e45962);
        let assign30860_e45965: f64 = (assign30860_e45963 * locals.var_sp_s_temp2);
        let assign30860_e45967: f64 = (assign30860_e45965 * locals.var_sp_s_temp2);
        (assign30860_e45967, ((((((8.0 * locals.var_sp_s_temp2_dn3) - (12.0 * locals.var_sp_s_xi0_dn3)) * locals.var_sp_s_temp2) + (assign30860_e45963 * locals.var_sp_s_temp2_dn3)) * locals.var_sp_s_temp2) + (assign30860_e45965 * locals.var_sp_s_temp2_dn3)), ((((((8.0 * locals.var_sp_s_temp2_dn4) - (12.0 * locals.var_sp_s_xi0_dn4)) * locals.var_sp_s_temp2) + (assign30860_e45963 * locals.var_sp_s_temp2_dn4)) * locals.var_sp_s_temp2) + (assign30860_e45965 * locals.var_sp_s_temp2_dn4)), ((((((8.0 * locals.var_sp_s_temp2_dn5) - (12.0 * locals.var_sp_s_xi0_dn5)) * locals.var_sp_s_temp2) + (assign30860_e45963 * locals.var_sp_s_temp2_dn5)) * locals.var_sp_s_temp2) + (assign30860_e45965 * locals.var_sp_s_temp2_dn5)), ((((((8.0 * locals.var_sp_s_temp2_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp2) + (assign30860_e45963 * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign30860_e45965 * locals.var_sp_s_temp2_dn6)), ((((((8.0 * locals.var_sp_s_temp2_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp2) + (assign30860_e45963 * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign30860_e45965 * locals.var_sp_s_temp2_dn7)), ((((((8.0 * locals.var_sp_s_temp2_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp2) + (assign30860_e45963 * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign30860_e45965 * locals.var_sp_s_temp2_dn8)), ((((((8.0 * locals.var_sp_s_temp2_dn9) - (12.0 * locals.var_sp_s_xi0_dn9)) * locals.var_sp_s_temp2) + (assign30860_e45963 * locals.var_sp_s_temp2_dn9)) * locals.var_sp_s_temp2) + (assign30860_e45965 * locals.var_sp_s_temp2_dn9)), ((((((8.0 * locals.var_sp_s_temp2_dn10) - (12.0 * locals.var_sp_s_xi0_dn10)) * locals.var_sp_s_temp2) + (assign30860_e45963 * locals.var_sp_s_temp2_dn10)) * locals.var_sp_s_temp2) + (assign30860_e45965 * locals.var_sp_s_temp2_dn10)), ((((((8.0 * locals.var_sp_s_temp2_dn11) - (12.0 * locals.var_sp_s_xi0_dn11)) * locals.var_sp_s_temp2) + (assign30860_e45963 * locals.var_sp_s_temp2_dn11)) * locals.var_sp_s_temp2) + (assign30860_e45965 * locals.var_sp_s_temp2_dn11)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn3, locals.var_sp_s_xi2_dn4, locals.var_sp_s_xi2_dn5, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, locals.var_sp_s_xi2_dn9, locals.var_sp_s_xi2_dn10, locals.var_sp_s_xi2_dn11,)
    }
};
        locals.var_sp_s_xi2 = assign30860_e45969;
        locals.var_sp_s_xi2_dn3 = assign30860_e45969_d_n3;
        locals.var_sp_s_xi2_dn4 = assign30860_e45969_d_n4;
        locals.var_sp_s_xi2_dn5 = assign30860_e45969_d_n5;
        locals.var_sp_s_xi2_dn6 = assign30860_e45969_d_n6;
        locals.var_sp_s_xi2_dn7 = assign30860_e45969_d_n7;
        locals.var_sp_s_xi2_dn8 = assign30860_e45969_d_n8;
        locals.var_sp_s_xi2_dn9 = assign30860_e45969_d_n9;
        locals.var_sp_s_xi2_dn10 = assign30860_e45969_d_n10;
        locals.var_sp_s_xi2_dn11 = assign30860_e45969_d_n11;
        locals.var_sp_s_xi2_rv = 0.0;

        let (assign30870_e46005, assign30870_e46005_d_n3, assign30870_e46005_d_n4, assign30870_e46005_d_n5, assign30870_e46005_d_n6, assign30870_e46005_d_n7, assign30870_e46005_d_n8, assign30870_e46005_d_n9, assign30870_e46005_d_n10, assign30870_e46005_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30870_e45986: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign30870_e45990: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
        let assign30870_e45992: f64 = (assign30870_e45990 - 1.0);
        let assign30870_e45996: f64 = (locals.var_sp_s_eta + 1.0);
        let assign30870_e45998: f64 = (assign30870_e45996 + locals.var_sp_s_xi0);
        let assign30870_e45999: f64 = (locals.var_exp_ns * assign30870_e45998);
        let assign30870_e46000: f64 = (assign30870_e45992 - assign30870_e45999);
        let assign30870_e46001: f64 = (locals.var_gam2 * assign30870_e46000);
        let assign30870_e46002: f64 = (assign30870_e45986 - assign30870_e46001);
        let assign30870_e46003: f64 = (1e-40_f64).max(assign30870_e46002);
        (assign30870_e46003, if 1e-40 >= assign30870_e46002 { 0.0 } else { (((locals.var_sp_s_temp_dn3 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn3)) - ((locals.var_gam2_dn3 * assign30870_e46000) + (locals.var_gam2 * ((locals.var_sp_s_temp1_dn3 + locals.var_sp_s_eta_dn3) - ((locals.var_exp_ns_dn3 * assign30870_e45998) + (locals.var_exp_ns * (locals.var_sp_s_eta_dn3 + locals.var_sp_s_xi0_dn3))))))) }, if 1e-40 >= assign30870_e46002 { 0.0 } else { (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) - ((locals.var_gam2_dn4 * assign30870_e46000) + (locals.var_gam2 * ((locals.var_sp_s_temp1_dn4 + locals.var_sp_s_eta_dn4) - ((locals.var_exp_ns_dn4 * assign30870_e45998) + (locals.var_exp_ns * (locals.var_sp_s_eta_dn4 + locals.var_sp_s_xi0_dn4))))))) }, if 1e-40 >= assign30870_e46002 { 0.0 } else { (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) - ((locals.var_gam2_dn5 * assign30870_e46000) + (locals.var_gam2 * ((locals.var_sp_s_temp1_dn5 + locals.var_sp_s_eta_dn5) - ((locals.var_exp_ns_dn5 * assign30870_e45998) + (locals.var_exp_ns * (locals.var_sp_s_eta_dn5 + locals.var_sp_s_xi0_dn5))))))) }, if 1e-40 >= assign30870_e46002 { 0.0 } else { (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gam2_dn6 * assign30870_e46000) + (locals.var_gam2 * ((locals.var_sp_s_temp1_dn6 + locals.var_sp_s_eta_dn6) - ((locals.var_exp_ns_dn6 * assign30870_e45998) + (locals.var_exp_ns * (locals.var_sp_s_eta_dn6 + locals.var_sp_s_xi0_dn6))))))) }, if 1e-40 >= assign30870_e46002 { 0.0 } else { (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gam2_dn7 * assign30870_e46000) + (locals.var_gam2 * ((locals.var_sp_s_temp1_dn7 + locals.var_sp_s_eta_dn7) - ((locals.var_exp_ns_dn7 * assign30870_e45998) + (locals.var_exp_ns * (locals.var_sp_s_eta_dn7 + locals.var_sp_s_xi0_dn7))))))) }, if 1e-40 >= assign30870_e46002 { 0.0 } else { (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gam2_dn8 * assign30870_e46000) + (locals.var_gam2 * ((locals.var_sp_s_temp1_dn8 + locals.var_sp_s_eta_dn8) - ((locals.var_exp_ns_dn8 * assign30870_e45998) + (locals.var_exp_ns * (locals.var_sp_s_eta_dn8 + locals.var_sp_s_xi0_dn8))))))) }, if 1e-40 >= assign30870_e46002 { 0.0 } else { (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) - ((locals.var_gam2_dn9 * assign30870_e46000) + (locals.var_gam2 * ((locals.var_sp_s_temp1_dn9 + locals.var_sp_s_eta_dn9) - ((locals.var_exp_ns_dn9 * assign30870_e45998) + (locals.var_exp_ns * (locals.var_sp_s_eta_dn9 + locals.var_sp_s_xi0_dn9))))))) }, if 1e-40 >= assign30870_e46002 { 0.0 } else { (((locals.var_sp_s_temp_dn10 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn10)) - ((locals.var_gam2_dn10 * assign30870_e46000) + (locals.var_gam2 * ((locals.var_sp_s_temp1_dn10 + locals.var_sp_s_eta_dn10) - ((locals.var_exp_ns_dn10 * assign30870_e45998) + (locals.var_exp_ns * (locals.var_sp_s_eta_dn10 + locals.var_sp_s_xi0_dn10))))))) }, if 1e-40 >= assign30870_e46002 { 0.0 } else { (((locals.var_sp_s_temp_dn11 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn11)) - ((locals.var_gam2_dn11 * assign30870_e46000) + (locals.var_gam2 * ((locals.var_sp_s_temp1_dn11 + locals.var_sp_s_eta_dn11) - ((locals.var_exp_ns_dn11 * assign30870_e45998) + (locals.var_exp_ns * (locals.var_sp_s_eta_dn11 + locals.var_sp_s_xi0_dn11))))))) },)
    } else {
        (locals.var_sp_s_a, locals.var_sp_s_a_dn3, locals.var_sp_s_a_dn4, locals.var_sp_s_a_dn5, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8, locals.var_sp_s_a_dn9, locals.var_sp_s_a_dn10, locals.var_sp_s_a_dn11,)
    }
};
        locals.var_sp_s_a = assign30870_e46005;
        locals.var_sp_s_a_dn3 = assign30870_e46005_d_n3;
        locals.var_sp_s_a_dn4 = assign30870_e46005_d_n4;
        locals.var_sp_s_a_dn5 = assign30870_e46005_d_n5;
        locals.var_sp_s_a_dn6 = assign30870_e46005_d_n6;
        locals.var_sp_s_a_dn7 = assign30870_e46005_d_n7;
        locals.var_sp_s_a_dn8 = assign30870_e46005_d_n8;
        locals.var_sp_s_a_dn9 = assign30870_e46005_d_n9;
        locals.var_sp_s_a_dn10 = assign30870_e46005_d_n10;
        locals.var_sp_s_a_dn11 = assign30870_e46005_d_n11;
        locals.var_sp_s_a_rv = 0.0;

        let (assign30880_e46031, assign30880_e46031_d_n3, assign30880_e46031_d_n4, assign30880_e46031_d_n5, assign30880_e46031_d_n6, assign30880_e46031_d_n7, assign30880_e46031_d_n8, assign30880_e46031_d_n9, assign30880_e46031_d_n10, assign30880_e46031_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30880_e46025: f64 = (locals.var_exp_ns * locals.var_sp_s_xi2);
        let assign30880_e46026: f64 = (locals.var_sp_s_temp1 - assign30880_e46025);
        let assign30880_e46027: f64 = (locals.var_gam2 * assign30880_e46026);
        let assign30880_e46028: f64 = (0.5 * assign30880_e46027);
        let assign30880_e46029: f64 = (1.0 - assign30880_e46028);
        (assign30880_e46029, (-(0.5 * ((locals.var_gam2_dn3 * assign30880_e46026) + (locals.var_gam2 * (locals.var_sp_s_temp1_dn3 - ((locals.var_exp_ns_dn3 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn3))))))), (-(0.5 * ((locals.var_gam2_dn4 * assign30880_e46026) + (locals.var_gam2 * (locals.var_sp_s_temp1_dn4 - ((locals.var_exp_ns_dn4 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn4))))))), (-(0.5 * ((locals.var_gam2_dn5 * assign30880_e46026) + (locals.var_gam2 * (locals.var_sp_s_temp1_dn5 - ((locals.var_exp_ns_dn5 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn5))))))), (-(0.5 * ((locals.var_gam2_dn6 * assign30880_e46026) + (locals.var_gam2 * (locals.var_sp_s_temp1_dn6 - ((locals.var_exp_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn6))))))), (-(0.5 * ((locals.var_gam2_dn7 * assign30880_e46026) + (locals.var_gam2 * (locals.var_sp_s_temp1_dn7 - ((locals.var_exp_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn7))))))), (-(0.5 * ((locals.var_gam2_dn8 * assign30880_e46026) + (locals.var_gam2 * (locals.var_sp_s_temp1_dn8 - ((locals.var_exp_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn8))))))), (-(0.5 * ((locals.var_gam2_dn9 * assign30880_e46026) + (locals.var_gam2 * (locals.var_sp_s_temp1_dn9 - ((locals.var_exp_ns_dn9 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn9))))))), (-(0.5 * ((locals.var_gam2_dn10 * assign30880_e46026) + (locals.var_gam2 * (locals.var_sp_s_temp1_dn10 - ((locals.var_exp_ns_dn10 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn10))))))), (-(0.5 * ((locals.var_gam2_dn11 * assign30880_e46026) + (locals.var_gam2 * (locals.var_sp_s_temp1_dn11 - ((locals.var_exp_ns_dn11 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn11))))))),)
    } else {
        (locals.var_sp_s_b, locals.var_sp_s_b_dn3, locals.var_sp_s_b_dn4, locals.var_sp_s_b_dn5, locals.var_sp_s_b_dn6, locals.var_sp_s_b_dn7, locals.var_sp_s_b_dn8, locals.var_sp_s_b_dn9, locals.var_sp_s_b_dn10, locals.var_sp_s_b_dn11,)
    }
};
        locals.var_sp_s_b = assign30880_e46031;
        locals.var_sp_s_b_dn3 = assign30880_e46031_d_n3;
        locals.var_sp_s_b_dn4 = assign30880_e46031_d_n4;
        locals.var_sp_s_b_dn5 = assign30880_e46031_d_n5;
        locals.var_sp_s_b_dn6 = assign30880_e46031_d_n6;
        locals.var_sp_s_b_dn7 = assign30880_e46031_d_n7;
        locals.var_sp_s_b_dn8 = assign30880_e46031_d_n8;
        locals.var_sp_s_b_dn9 = assign30880_e46031_d_n9;
        locals.var_sp_s_b_dn10 = assign30880_e46031_d_n10;
        locals.var_sp_s_b_dn11 = assign30880_e46031_d_n11;
        locals.var_sp_s_b_rv = 0.0;

        let (assign30890_e46061, assign30890_e46061_d_n3, assign30890_e46061_d_n4, assign30890_e46061_d_n5, assign30890_e46061_d_n6, assign30890_e46061_d_n7, assign30890_e46061_d_n8, assign30890_e46061_d_n9, assign30890_e46061_d_n10, assign30890_e46061_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30890_e46047: f64 = (2.0 * locals.var_sp_s_temp);
        let assign30890_e46051: f64 = (1.0 - locals.var_sp_s_temp1);
        let assign30890_e46055: f64 = (1.0 + locals.var_sp_s_xi1);
        let assign30890_e46056: f64 = (locals.var_exp_ns * assign30890_e46055);
        let assign30890_e46057: f64 = (assign30890_e46051 - assign30890_e46056);
        let assign30890_e46058: f64 = (locals.var_gam2 * assign30890_e46057);
        let assign30890_e46059: f64 = (assign30890_e46047 + assign30890_e46058);
        (assign30890_e46059, ((2.0 * locals.var_sp_s_temp_dn3) + ((locals.var_gam2_dn3 * assign30890_e46057) + (locals.var_gam2 * ((-locals.var_sp_s_temp1_dn3) - ((locals.var_exp_ns_dn3 * assign30890_e46055) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn3)))))), ((2.0 * locals.var_sp_s_temp_dn4) + ((locals.var_gam2_dn4 * assign30890_e46057) + (locals.var_gam2 * ((-locals.var_sp_s_temp1_dn4) - ((locals.var_exp_ns_dn4 * assign30890_e46055) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn4)))))), ((2.0 * locals.var_sp_s_temp_dn5) + ((locals.var_gam2_dn5 * assign30890_e46057) + (locals.var_gam2 * ((-locals.var_sp_s_temp1_dn5) - ((locals.var_exp_ns_dn5 * assign30890_e46055) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn5)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gam2_dn6 * assign30890_e46057) + (locals.var_gam2 * ((-locals.var_sp_s_temp1_dn6) - ((locals.var_exp_ns_dn6 * assign30890_e46055) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gam2_dn7 * assign30890_e46057) + (locals.var_gam2 * ((-locals.var_sp_s_temp1_dn7) - ((locals.var_exp_ns_dn7 * assign30890_e46055) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gam2_dn8 * assign30890_e46057) + (locals.var_gam2 * ((-locals.var_sp_s_temp1_dn8) - ((locals.var_exp_ns_dn8 * assign30890_e46055) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn8)))))), ((2.0 * locals.var_sp_s_temp_dn9) + ((locals.var_gam2_dn9 * assign30890_e46057) + (locals.var_gam2 * ((-locals.var_sp_s_temp1_dn9) - ((locals.var_exp_ns_dn9 * assign30890_e46055) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn9)))))), ((2.0 * locals.var_sp_s_temp_dn10) + ((locals.var_gam2_dn10 * assign30890_e46057) + (locals.var_gam2 * ((-locals.var_sp_s_temp1_dn10) - ((locals.var_exp_ns_dn10 * assign30890_e46055) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn10)))))), ((2.0 * locals.var_sp_s_temp_dn11) + ((locals.var_gam2_dn11 * assign30890_e46057) + (locals.var_gam2 * ((-locals.var_sp_s_temp1_dn11) - ((locals.var_exp_ns_dn11 * assign30890_e46055) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn11)))))),)
    } else {
        (locals.var_sp_s_c, locals.var_sp_s_c_dn3, locals.var_sp_s_c_dn4, locals.var_sp_s_c_dn5, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8, locals.var_sp_s_c_dn9, locals.var_sp_s_c_dn10, locals.var_sp_s_c_dn11,)
    }
};
        locals.var_sp_s_c = assign30890_e46061;
        locals.var_sp_s_c_dn3 = assign30890_e46061_d_n3;
        locals.var_sp_s_c_dn4 = assign30890_e46061_d_n4;
        locals.var_sp_s_c_dn5 = assign30890_e46061_d_n5;
        locals.var_sp_s_c_dn6 = assign30890_e46061_d_n6;
        locals.var_sp_s_c_dn7 = assign30890_e46061_d_n7;
        locals.var_sp_s_c_dn8 = assign30890_e46061_d_n8;
        locals.var_sp_s_c_dn9 = assign30890_e46061_d_n9;
        locals.var_sp_s_c_dn10 = assign30890_e46061_d_n10;
        locals.var_sp_s_c_dn11 = assign30890_e46061_d_n11;
        locals.var_sp_s_c_rv = 0.0;

        let (assign30900_e46086, assign30900_e46086_d_n3, assign30900_e46086_d_n4, assign30900_e46086_d_n5, assign30900_e46086_d_n6, assign30900_e46086_d_n7, assign30900_e46086_d_n8, assign30900_e46086_d_n9, assign30900_e46086_d_n10, assign30900_e46086_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30900_e46077: f64 = (locals.var_phisf - locals.var_sp_s_eta);
        let assign30900_e46080: f64 = (locals.var_sp_s_a / locals.var_gam2);
        let assign30900_e46082: f64 = (assign30900_e46080).max(1e-38);
        let assign30900_e46083: f64 = (assign30900_e46082).ln();
        let assign30900_e46084: f64 = (assign30900_e46077 + assign30900_e46083);
        (assign30900_e46084, ((locals.var_phisf_dn3 - locals.var_sp_s_eta_dn3) + (if assign30900_e46080 >= 1e-38 { (((locals.var_sp_s_a_dn3 * locals.var_gam2) - (locals.var_sp_s_a * locals.var_gam2_dn3)) / (locals.var_gam2 * locals.var_gam2)) } else { 0.0 } / assign30900_e46082)), ((locals.var_phisf_dn4 - locals.var_sp_s_eta_dn4) + (if assign30900_e46080 >= 1e-38 { (((locals.var_sp_s_a_dn4 * locals.var_gam2) - (locals.var_sp_s_a * locals.var_gam2_dn4)) / (locals.var_gam2 * locals.var_gam2)) } else { 0.0 } / assign30900_e46082)), ((locals.var_phisf_dn5 - locals.var_sp_s_eta_dn5) + (if assign30900_e46080 >= 1e-38 { (((locals.var_sp_s_a_dn5 * locals.var_gam2) - (locals.var_sp_s_a * locals.var_gam2_dn5)) / (locals.var_gam2 * locals.var_gam2)) } else { 0.0 } / assign30900_e46082)), ((locals.var_phisf_dn6 - locals.var_sp_s_eta_dn6) + (if assign30900_e46080 >= 1e-38 { (((locals.var_sp_s_a_dn6 * locals.var_gam2) - (locals.var_sp_s_a * locals.var_gam2_dn6)) / (locals.var_gam2 * locals.var_gam2)) } else { 0.0 } / assign30900_e46082)), ((locals.var_phisf_dn7 - locals.var_sp_s_eta_dn7) + (if assign30900_e46080 >= 1e-38 { (((locals.var_sp_s_a_dn7 * locals.var_gam2) - (locals.var_sp_s_a * locals.var_gam2_dn7)) / (locals.var_gam2 * locals.var_gam2)) } else { 0.0 } / assign30900_e46082)), ((locals.var_phisf_dn8 - locals.var_sp_s_eta_dn8) + (if assign30900_e46080 >= 1e-38 { (((locals.var_sp_s_a_dn8 * locals.var_gam2) - (locals.var_sp_s_a * locals.var_gam2_dn8)) / (locals.var_gam2 * locals.var_gam2)) } else { 0.0 } / assign30900_e46082)), ((locals.var_phisf_dn9 - locals.var_sp_s_eta_dn9) + (if assign30900_e46080 >= 1e-38 { (((locals.var_sp_s_a_dn9 * locals.var_gam2) - (locals.var_sp_s_a * locals.var_gam2_dn9)) / (locals.var_gam2 * locals.var_gam2)) } else { 0.0 } / assign30900_e46082)), ((locals.var_phisf_dn10 - locals.var_sp_s_eta_dn10) + (if assign30900_e46080 >= 1e-38 { (((locals.var_sp_s_a_dn10 * locals.var_gam2) - (locals.var_sp_s_a * locals.var_gam2_dn10)) / (locals.var_gam2 * locals.var_gam2)) } else { 0.0 } / assign30900_e46082)), ((locals.var_phisf_dn11 - locals.var_sp_s_eta_dn11) + (if assign30900_e46080 >= 1e-38 { (((locals.var_sp_s_a_dn11 * locals.var_gam2) - (locals.var_sp_s_a * locals.var_gam2_dn11)) / (locals.var_gam2 * locals.var_gam2)) } else { 0.0 } / assign30900_e46082)),)
    } else {
        (locals.var_sp_s_tau, locals.var_sp_s_tau_dn3, locals.var_sp_s_tau_dn4, locals.var_sp_s_tau_dn5, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8, locals.var_sp_s_tau_dn9, locals.var_sp_s_tau_dn10, locals.var_sp_s_tau_dn11,)
    }
};
        locals.var_sp_s_tau = assign30900_e46086;
        locals.var_sp_s_tau_dn3 = assign30900_e46086_d_n3;
        locals.var_sp_s_tau_dn4 = assign30900_e46086_d_n4;
        locals.var_sp_s_tau_dn5 = assign30900_e46086_d_n5;
        locals.var_sp_s_tau_dn6 = assign30900_e46086_d_n6;
        locals.var_sp_s_tau_dn7 = assign30900_e46086_d_n7;
        locals.var_sp_s_tau_dn8 = assign30900_e46086_d_n8;
        locals.var_sp_s_tau_dn9 = assign30900_e46086_d_n9;
        locals.var_sp_s_tau_dn10 = assign30900_e46086_d_n10;
        locals.var_sp_s_tau_dn11 = assign30900_e46086_d_n11;
        locals.var_sp_s_tau_rv = 0.0;

        let (assign30910_e46104, assign30910_e46104_d_n3, assign30910_e46104_d_n4, assign30910_e46104_d_n5, assign30910_e46104_d_n6, assign30910_e46104_d_n7, assign30910_e46104_d_n8, assign30910_e46104_d_n9, assign30910_e46104_d_n10, assign30910_e46104_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30910_e46102: f64 = (locals.var_sp_s_a + locals.var_sp_s_c);
        (assign30910_e46102, (locals.var_sp_s_a_dn3 + locals.var_sp_s_c_dn3), (locals.var_sp_s_a_dn4 + locals.var_sp_s_c_dn4), (locals.var_sp_s_a_dn5 + locals.var_sp_s_c_dn5), (locals.var_sp_s_a_dn6 + locals.var_sp_s_c_dn6), (locals.var_sp_s_a_dn7 + locals.var_sp_s_c_dn7), (locals.var_sp_s_a_dn8 + locals.var_sp_s_c_dn8), (locals.var_sp_s_a_dn9 + locals.var_sp_s_c_dn9), (locals.var_sp_s_a_dn10 + locals.var_sp_s_c_dn10), (locals.var_sp_s_a_dn11 + locals.var_sp_s_c_dn11),)
    } else {
        (locals.var_nu, locals.var_nu_dn3, locals.var_nu_dn4, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9, locals.var_nu_dn10, locals.var_nu_dn11,)
    }
};
        locals.var_nu = assign30910_e46104;
        locals.var_nu_dn3 = assign30910_e46104_d_n3;
        locals.var_nu_dn4 = assign30910_e46104_d_n4;
        locals.var_nu_dn5 = assign30910_e46104_d_n5;
        locals.var_nu_dn6 = assign30910_e46104_d_n6;
        locals.var_nu_dn7 = assign30910_e46104_d_n7;
        locals.var_nu_dn8 = assign30910_e46104_d_n8;
        locals.var_nu_dn9 = assign30910_e46104_d_n9;
        locals.var_nu_dn10 = assign30910_e46104_d_n10;
        locals.var_nu_dn11 = assign30910_e46104_d_n11;
        locals.var_nu_rv = 0.0;

        let (assign30920_e46134, assign30920_e46134_d_n3, assign30920_e46134_d_n4, assign30920_e46134_d_n5, assign30920_e46134_d_n6, assign30920_e46134_d_n7, assign30920_e46134_d_n8, assign30920_e46134_d_n9, assign30920_e46134_d_n10, assign30920_e46134_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30920_e46120: f64 = (locals.var_nu * locals.var_nu);
        let assign30920_e46125: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign30920_e46126: f64 = (0.5 * assign30920_e46125);
        let assign30920_e46129: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
        let assign30920_e46130: f64 = (assign30920_e46126 - assign30920_e46129);
        let assign30920_e46131: f64 = (locals.var_sp_s_tau * assign30920_e46130);
        let assign30920_e46132: f64 = (assign30920_e46120 + assign30920_e46131);
        (assign30920_e46132, (((locals.var_nu_dn3 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn3)) + ((locals.var_sp_s_tau_dn3 * assign30920_e46130) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn3 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn3))) - ((locals.var_sp_s_a_dn3 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn3)))))), (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau_dn4 * assign30920_e46130) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn4 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn4))) - ((locals.var_sp_s_a_dn4 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn4)))))), (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau_dn5 * assign30920_e46130) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5))) - ((locals.var_sp_s_a_dn5 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn5)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau_dn6 * assign30920_e46130) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6))) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau_dn7 * assign30920_e46130) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7))) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau_dn8 * assign30920_e46130) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8))) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau_dn9 * assign30920_e46130) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn9 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn9))) - ((locals.var_sp_s_a_dn9 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn9)))))), (((locals.var_nu_dn10 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn10)) + ((locals.var_sp_s_tau_dn10 * assign30920_e46130) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn10 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn10))) - ((locals.var_sp_s_a_dn10 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn10)))))), (((locals.var_nu_dn11 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn11)) + ((locals.var_sp_s_tau_dn11 * assign30920_e46130) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn11 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn11))) - ((locals.var_sp_s_a_dn11 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn11)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn3, locals.var_mutau_dn4, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9, locals.var_mutau_dn10, locals.var_mutau_dn11,)
    }
};
        locals.var_mutau = assign30920_e46134;
        locals.var_mutau_dn3 = assign30920_e46134_d_n3;
        locals.var_mutau_dn4 = assign30920_e46134_d_n4;
        locals.var_mutau_dn5 = assign30920_e46134_d_n5;
        locals.var_mutau_dn6 = assign30920_e46134_d_n6;
        locals.var_mutau_dn7 = assign30920_e46134_d_n7;
        locals.var_mutau_dn8 = assign30920_e46134_d_n8;
        locals.var_mutau_dn9 = assign30920_e46134_d_n9;
        locals.var_mutau_dn10 = assign30920_e46134_d_n10;
        locals.var_mutau_dn11 = assign30920_e46134_d_n11;
        locals.var_mutau_rv = 0.0;

        let (assign30930_e46178, assign30930_e46178_d_n3, assign30930_e46178_d_n4, assign30930_e46178_d_n5, assign30930_e46178_d_n6, assign30930_e46178_d_n7, assign30930_e46178_d_n8, assign30930_e46178_d_n9, assign30930_e46178_d_n10, assign30930_e46178_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30930_e46151: f64 = (locals.var_sp_s_a * locals.var_nu);
        let assign30930_e46153: f64 = (assign30930_e46151 * locals.var_sp_s_tau);
        let assign30930_e46157: f64 = (locals.var_nu / locals.var_mutau);
        let assign30930_e46159: f64 = (assign30930_e46157 * locals.var_sp_s_tau);
        let assign30930_e46161: f64 = (assign30930_e46159 * locals.var_sp_s_tau);
        let assign30930_e46163: f64 = (assign30930_e46161 * locals.var_sp_s_c);
        let assign30930_e46166: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign30930_e46168: f64 = (assign30930_e46166 * 0.3333333333333333);
        let assign30930_e46171: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
        let assign30930_e46172: f64 = (assign30930_e46168 - assign30930_e46171);
        let assign30930_e46173: f64 = (assign30930_e46163 * assign30930_e46172);
        let assign30930_e46174: f64 = (locals.var_mutau + assign30930_e46173);
        let assign30930_e46175: f64 = (assign30930_e46153 / assign30930_e46174);
        let assign30930_e46176: f64 = (locals.var_sp_s_eta + assign30930_e46175);
        (assign30930_e46176, (locals.var_sp_s_eta_dn3 + (((((((locals.var_sp_s_a_dn3 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn3)) * locals.var_sp_s_tau) + (assign30930_e46151 * locals.var_sp_s_tau_dn3)) * assign30930_e46174) - (assign30930_e46153 * (locals.var_mutau_dn3 + (((((((((((locals.var_nu_dn3 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn3)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30930_e46157 * locals.var_sp_s_tau_dn3)) * locals.var_sp_s_tau) + (assign30930_e46159 * locals.var_sp_s_tau_dn3)) * locals.var_sp_s_c) + (assign30930_e46161 * locals.var_sp_s_c_dn3)) * assign30930_e46172) + (assign30930_e46163 * ((((locals.var_sp_s_c_dn3 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn3)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn3 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn3)))))))) / (assign30930_e46174 * assign30930_e46174))), (locals.var_sp_s_eta_dn4 + (((((((locals.var_sp_s_a_dn4 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn4)) * locals.var_sp_s_tau) + (assign30930_e46151 * locals.var_sp_s_tau_dn4)) * assign30930_e46174) - (assign30930_e46153 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30930_e46157 * locals.var_sp_s_tau_dn4)) * locals.var_sp_s_tau) + (assign30930_e46159 * locals.var_sp_s_tau_dn4)) * locals.var_sp_s_c) + (assign30930_e46161 * locals.var_sp_s_c_dn4)) * assign30930_e46172) + (assign30930_e46163 * ((((locals.var_sp_s_c_dn4 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn4)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn4 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn4)))))))) / (assign30930_e46174 * assign30930_e46174))), (locals.var_sp_s_eta_dn5 + (((((((locals.var_sp_s_a_dn5 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn5)) * locals.var_sp_s_tau) + (assign30930_e46151 * locals.var_sp_s_tau_dn5)) * assign30930_e46174) - (assign30930_e46153 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30930_e46157 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_tau) + (assign30930_e46159 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_c) + (assign30930_e46161 * locals.var_sp_s_c_dn5)) * assign30930_e46172) + (assign30930_e46163 * ((((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn5 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn5)))))))) / (assign30930_e46174 * assign30930_e46174))), (locals.var_sp_s_eta_dn6 + (((((((locals.var_sp_s_a_dn6 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn6)) * locals.var_sp_s_tau) + (assign30930_e46151 * locals.var_sp_s_tau_dn6)) * assign30930_e46174) - (assign30930_e46153 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30930_e46157 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_tau) + (assign30930_e46159 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_c) + (assign30930_e46161 * locals.var_sp_s_c_dn6)) * assign30930_e46172) + (assign30930_e46163 * ((((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))))) / (assign30930_e46174 * assign30930_e46174))), (locals.var_sp_s_eta_dn7 + (((((((locals.var_sp_s_a_dn7 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn7)) * locals.var_sp_s_tau) + (assign30930_e46151 * locals.var_sp_s_tau_dn7)) * assign30930_e46174) - (assign30930_e46153 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30930_e46157 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_tau) + (assign30930_e46159 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_c) + (assign30930_e46161 * locals.var_sp_s_c_dn7)) * assign30930_e46172) + (assign30930_e46163 * ((((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))))) / (assign30930_e46174 * assign30930_e46174))), (locals.var_sp_s_eta_dn8 + (((((((locals.var_sp_s_a_dn8 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn8)) * locals.var_sp_s_tau) + (assign30930_e46151 * locals.var_sp_s_tau_dn8)) * assign30930_e46174) - (assign30930_e46153 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30930_e46157 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_tau) + (assign30930_e46159 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_c) + (assign30930_e46161 * locals.var_sp_s_c_dn8)) * assign30930_e46172) + (assign30930_e46163 * ((((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))))) / (assign30930_e46174 * assign30930_e46174))), (locals.var_sp_s_eta_dn9 + (((((((locals.var_sp_s_a_dn9 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn9)) * locals.var_sp_s_tau) + (assign30930_e46151 * locals.var_sp_s_tau_dn9)) * assign30930_e46174) - (assign30930_e46153 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30930_e46157 * locals.var_sp_s_tau_dn9)) * locals.var_sp_s_tau) + (assign30930_e46159 * locals.var_sp_s_tau_dn9)) * locals.var_sp_s_c) + (assign30930_e46161 * locals.var_sp_s_c_dn9)) * assign30930_e46172) + (assign30930_e46163 * ((((locals.var_sp_s_c_dn9 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn9)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn9 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn9)))))))) / (assign30930_e46174 * assign30930_e46174))), (locals.var_sp_s_eta_dn10 + (((((((locals.var_sp_s_a_dn10 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn10)) * locals.var_sp_s_tau) + (assign30930_e46151 * locals.var_sp_s_tau_dn10)) * assign30930_e46174) - (assign30930_e46153 * (locals.var_mutau_dn10 + (((((((((((locals.var_nu_dn10 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn10)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30930_e46157 * locals.var_sp_s_tau_dn10)) * locals.var_sp_s_tau) + (assign30930_e46159 * locals.var_sp_s_tau_dn10)) * locals.var_sp_s_c) + (assign30930_e46161 * locals.var_sp_s_c_dn10)) * assign30930_e46172) + (assign30930_e46163 * ((((locals.var_sp_s_c_dn10 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn10)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn10 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn10)))))))) / (assign30930_e46174 * assign30930_e46174))), (locals.var_sp_s_eta_dn11 + (((((((locals.var_sp_s_a_dn11 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn11)) * locals.var_sp_s_tau) + (assign30930_e46151 * locals.var_sp_s_tau_dn11)) * assign30930_e46174) - (assign30930_e46153 * (locals.var_mutau_dn11 + (((((((((((locals.var_nu_dn11 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn11)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign30930_e46157 * locals.var_sp_s_tau_dn11)) * locals.var_sp_s_tau) + (assign30930_e46159 * locals.var_sp_s_tau_dn11)) * locals.var_sp_s_c) + (assign30930_e46161 * locals.var_sp_s_c_dn11)) * assign30930_e46172) + (assign30930_e46163 * ((((locals.var_sp_s_c_dn11 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn11)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn11 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn11)))))))) / (assign30930_e46174 * assign30930_e46174))),)
    } else {
        (locals.var_sp_s_x0, locals.var_sp_s_x0_dn3, locals.var_sp_s_x0_dn4, locals.var_sp_s_x0_dn5, locals.var_sp_s_x0_dn6, locals.var_sp_s_x0_dn7, locals.var_sp_s_x0_dn8, locals.var_sp_s_x0_dn9, locals.var_sp_s_x0_dn10, locals.var_sp_s_x0_dn11,)
    }
};
        locals.var_sp_s_x0 = assign30930_e46178;
        locals.var_sp_s_x0_dn3 = assign30930_e46178_d_n3;
        locals.var_sp_s_x0_dn4 = assign30930_e46178_d_n4;
        locals.var_sp_s_x0_dn5 = assign30930_e46178_d_n5;
        locals.var_sp_s_x0_dn6 = assign30930_e46178_d_n6;
        locals.var_sp_s_x0_dn7 = assign30930_e46178_d_n7;
        locals.var_sp_s_x0_dn8 = assign30930_e46178_d_n8;
        locals.var_sp_s_x0_dn9 = assign30930_e46178_d_n9;
        locals.var_sp_s_x0_dn10 = assign30930_e46178_d_n10;
        locals.var_sp_s_x0_dn11 = assign30930_e46178_d_n11;
        locals.var_sp_s_x0_rv = 0.0;

        let (assign30940_e46195, assign30940_e46195_d_n3, assign30940_e46195_d_n4, assign30940_e46195_d_n5, assign30940_e46195_d_n6, assign30940_e46195_d_n7, assign30940_e46195_d_n8, assign30940_e46195_d_n9, assign30940_e46195_d_n10, assign30940_e46195_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30940_e46193: f64 = { let limited_exp_arg = locals.var_sp_s_x0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign30940_e46193, ({ let limited_exp_arg = locals.var_sp_s_x0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_x0_dn3), ({ let limited_exp_arg = locals.var_sp_s_x0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_x0_dn4), ({ let limited_exp_arg = locals.var_sp_s_x0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_x0_dn5), ({ let limited_exp_arg = locals.var_sp_s_x0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_x0_dn6), ({ let limited_exp_arg = locals.var_sp_s_x0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_x0_dn7), ({ let limited_exp_arg = locals.var_sp_s_x0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_x0_dn8), ({ let limited_exp_arg = locals.var_sp_s_x0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_x0_dn9), ({ let limited_exp_arg = locals.var_sp_s_x0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_x0_dn10), ({ let limited_exp_arg = locals.var_sp_s_x0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_sp_s_x0_dn11),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn3, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9, locals.var_sp_s_delta0_dn10, locals.var_sp_s_delta0_dn11,)
    }
};
        locals.var_sp_s_delta0 = assign30940_e46195;
        locals.var_sp_s_delta0_dn3 = assign30940_e46195_d_n3;
        locals.var_sp_s_delta0_dn4 = assign30940_e46195_d_n4;
        locals.var_sp_s_delta0_dn5 = assign30940_e46195_d_n5;
        locals.var_sp_s_delta0_dn6 = assign30940_e46195_d_n6;
        locals.var_sp_s_delta0_dn7 = assign30940_e46195_d_n7;
        locals.var_sp_s_delta0_dn8 = assign30940_e46195_d_n8;
        locals.var_sp_s_delta0_dn9 = assign30940_e46195_d_n9;
        locals.var_sp_s_delta0_dn10 = assign30940_e46195_d_n10;
        locals.var_sp_s_delta0_dn11 = assign30940_e46195_d_n11;
        locals.var_sp_s_delta0_rv = 0.0;

        let (assign30950_e46213, assign30950_e46213_d_n3, assign30950_e46213_d_n4, assign30950_e46213_d_n5, assign30950_e46213_d_n6, assign30950_e46213_d_n7, assign30950_e46213_d_n8, assign30950_e46213_d_n9, assign30950_e46213_d_n10, assign30950_e46213_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30950_e46211: f64 = (1.0 / locals.var_sp_s_delta0);
        (assign30950_e46211, (-(locals.var_sp_s_delta0_dn3 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn4 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn5 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn6 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn7 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn8 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn9 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn10 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn11 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn3, locals.var_sp_s_delta1_dn4, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn9, locals.var_sp_s_delta1_dn10, locals.var_sp_s_delta1_dn11,)
    }
};
        locals.var_sp_s_delta1 = assign30950_e46213;
        locals.var_sp_s_delta1_dn3 = assign30950_e46213_d_n3;
        locals.var_sp_s_delta1_dn4 = assign30950_e46213_d_n4;
        locals.var_sp_s_delta1_dn5 = assign30950_e46213_d_n5;
        locals.var_sp_s_delta1_dn6 = assign30950_e46213_d_n6;
        locals.var_sp_s_delta1_dn7 = assign30950_e46213_d_n7;
        locals.var_sp_s_delta1_dn8 = assign30950_e46213_d_n8;
        locals.var_sp_s_delta1_dn9 = assign30950_e46213_d_n9;
        locals.var_sp_s_delta1_dn10 = assign30950_e46213_d_n10;
        locals.var_sp_s_delta1_dn11 = assign30950_e46213_d_n11;
        locals.var_sp_s_delta1_rv = 0.0;

        let (assign30960_e46232, assign30960_e46232_d_n3, assign30960_e46232_d_n4, assign30960_e46232_d_n5, assign30960_e46232_d_n6, assign30960_e46232_d_n7, assign30960_e46232_d_n8, assign30960_e46232_d_n9, assign30960_e46232_d_n10, assign30960_e46232_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30960_e46229: f64 = (locals.var_sp_s_x0 - locals.var_phisf);
        let assign30960_e46230: f64 = { let limited_exp_arg = assign30960_e46229; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign30960_e46230, ({ let limited_exp_arg = assign30960_e46229; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_sp_s_x0_dn3 - locals.var_phisf_dn3)), ({ let limited_exp_arg = assign30960_e46229; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_sp_s_x0_dn4 - locals.var_phisf_dn4)), ({ let limited_exp_arg = assign30960_e46229; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_sp_s_x0_dn5 - locals.var_phisf_dn5)), ({ let limited_exp_arg = assign30960_e46229; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_sp_s_x0_dn6 - locals.var_phisf_dn6)), ({ let limited_exp_arg = assign30960_e46229; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_sp_s_x0_dn7 - locals.var_phisf_dn7)), ({ let limited_exp_arg = assign30960_e46229; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_sp_s_x0_dn8 - locals.var_phisf_dn8)), ({ let limited_exp_arg = assign30960_e46229; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_sp_s_x0_dn9 - locals.var_phisf_dn9)), ({ let limited_exp_arg = assign30960_e46229; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_sp_s_x0_dn10 - locals.var_phisf_dn10)), ({ let limited_exp_arg = assign30960_e46229; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_sp_s_x0_dn11 - locals.var_phisf_dn11)),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn3, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9, locals.var_sp_s_delta0_dn10, locals.var_sp_s_delta0_dn11,)
    }
};
        locals.var_sp_s_delta0 = assign30960_e46232;
        locals.var_sp_s_delta0_dn3 = assign30960_e46232_d_n3;
        locals.var_sp_s_delta0_dn4 = assign30960_e46232_d_n4;
        locals.var_sp_s_delta0_dn5 = assign30960_e46232_d_n5;
        locals.var_sp_s_delta0_dn6 = assign30960_e46232_d_n6;
        locals.var_sp_s_delta0_dn7 = assign30960_e46232_d_n7;
        locals.var_sp_s_delta0_dn8 = assign30960_e46232_d_n8;
        locals.var_sp_s_delta0_dn9 = assign30960_e46232_d_n9;
        locals.var_sp_s_delta0_dn10 = assign30960_e46232_d_n10;
        locals.var_sp_s_delta0_dn11 = assign30960_e46232_d_n11;
        locals.var_sp_s_delta0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_91(
        locals: &mut StampLocals,
    ) {
        let (assign30970_e46254, assign30970_e46254_d_n3, assign30970_e46254_d_n4, assign30970_e46254_d_n5, assign30970_e46254_d_n6, assign30970_e46254_d_n7, assign30970_e46254_d_n8, assign30970_e46254_d_n9, assign30970_e46254_d_n10, assign30970_e46254_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30970_e46250: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
        let assign30970_e46251: f64 = (2.0 + assign30970_e46250);
        let assign30970_e46252: f64 = (1.0 / assign30970_e46251);
        (assign30970_e46252, (-(((locals.var_sp_s_x0_dn3 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn3)) / (assign30970_e46251 * assign30970_e46251))), (-(((locals.var_sp_s_x0_dn4 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn4)) / (assign30970_e46251 * assign30970_e46251))), (-(((locals.var_sp_s_x0_dn5 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn5)) / (assign30970_e46251 * assign30970_e46251))), (-(((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) / (assign30970_e46251 * assign30970_e46251))), (-(((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) / (assign30970_e46251 * assign30970_e46251))), (-(((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) / (assign30970_e46251 * assign30970_e46251))), (-(((locals.var_sp_s_x0_dn9 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn9)) / (assign30970_e46251 * assign30970_e46251))), (-(((locals.var_sp_s_x0_dn10 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn10)) / (assign30970_e46251 * assign30970_e46251))), (-(((locals.var_sp_s_x0_dn11 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn11)) / (assign30970_e46251 * assign30970_e46251))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn3, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9, locals.var_sp_s_temp_dn10, locals.var_sp_s_temp_dn11,)
    }
};
        locals.var_sp_s_temp = assign30970_e46254;
        locals.var_sp_s_temp_dn3 = assign30970_e46254_d_n3;
        locals.var_sp_s_temp_dn4 = assign30970_e46254_d_n4;
        locals.var_sp_s_temp_dn5 = assign30970_e46254_d_n5;
        locals.var_sp_s_temp_dn6 = assign30970_e46254_d_n6;
        locals.var_sp_s_temp_dn7 = assign30970_e46254_d_n7;
        locals.var_sp_s_temp_dn8 = assign30970_e46254_d_n8;
        locals.var_sp_s_temp_dn9 = assign30970_e46254_d_n9;
        locals.var_sp_s_temp_dn10 = assign30970_e46254_d_n10;
        locals.var_sp_s_temp_dn11 = assign30970_e46254_d_n11;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign30980_e46274, assign30980_e46274_d_n3, assign30980_e46274_d_n4, assign30980_e46274_d_n5, assign30980_e46274_d_n6, assign30980_e46274_d_n7, assign30980_e46274_d_n8, assign30980_e46274_d_n9, assign30980_e46274_d_n10, assign30980_e46274_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30980_e46270: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
        let assign30980_e46272: f64 = (assign30980_e46270 * locals.var_sp_s_temp);
        (assign30980_e46272, ((((locals.var_sp_s_x0_dn3 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn3)) * locals.var_sp_s_temp) + (assign30980_e46270 * locals.var_sp_s_temp_dn3)), ((((locals.var_sp_s_x0_dn4 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn4)) * locals.var_sp_s_temp) + (assign30980_e46270 * locals.var_sp_s_temp_dn4)), ((((locals.var_sp_s_x0_dn5 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn5)) * locals.var_sp_s_temp) + (assign30980_e46270 * locals.var_sp_s_temp_dn5)), ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) * locals.var_sp_s_temp) + (assign30980_e46270 * locals.var_sp_s_temp_dn6)), ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) * locals.var_sp_s_temp) + (assign30980_e46270 * locals.var_sp_s_temp_dn7)), ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) * locals.var_sp_s_temp) + (assign30980_e46270 * locals.var_sp_s_temp_dn8)), ((((locals.var_sp_s_x0_dn9 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn9)) * locals.var_sp_s_temp) + (assign30980_e46270 * locals.var_sp_s_temp_dn9)), ((((locals.var_sp_s_x0_dn10 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn10)) * locals.var_sp_s_temp) + (assign30980_e46270 * locals.var_sp_s_temp_dn10)), ((((locals.var_sp_s_x0_dn11 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn11)) * locals.var_sp_s_temp) + (assign30980_e46270 * locals.var_sp_s_temp_dn11)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn3, locals.var_sp_s_xi0_dn4, locals.var_sp_s_xi0_dn5, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, locals.var_sp_s_xi0_dn9, locals.var_sp_s_xi0_dn10, locals.var_sp_s_xi0_dn11,)
    }
};
        locals.var_sp_s_xi0 = assign30980_e46274;
        locals.var_sp_s_xi0_dn3 = assign30980_e46274_d_n3;
        locals.var_sp_s_xi0_dn4 = assign30980_e46274_d_n4;
        locals.var_sp_s_xi0_dn5 = assign30980_e46274_d_n5;
        locals.var_sp_s_xi0_dn6 = assign30980_e46274_d_n6;
        locals.var_sp_s_xi0_dn7 = assign30980_e46274_d_n7;
        locals.var_sp_s_xi0_dn8 = assign30980_e46274_d_n8;
        locals.var_sp_s_xi0_dn9 = assign30980_e46274_d_n9;
        locals.var_sp_s_xi0_dn10 = assign30980_e46274_d_n10;
        locals.var_sp_s_xi0_dn11 = assign30980_e46274_d_n11;
        locals.var_sp_s_xi0_rv = 0.0;

        let (assign30990_e46296, assign30990_e46296_d_n3, assign30990_e46296_d_n4, assign30990_e46296_d_n5, assign30990_e46296_d_n6, assign30990_e46296_d_n7, assign30990_e46296_d_n8, assign30990_e46296_d_n9, assign30990_e46296_d_n10, assign30990_e46296_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign30990_e46291: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_temp);
        let assign30990_e46293: f64 = (assign30990_e46291 * locals.var_sp_s_temp);
        let assign30990_e46294: f64 = (4.0 * assign30990_e46293);
        (assign30990_e46294, (4.0 * ((((locals.var_sp_s_x0_dn3 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn3)) * locals.var_sp_s_temp) + (assign30990_e46291 * locals.var_sp_s_temp_dn3))), (4.0 * ((((locals.var_sp_s_x0_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn4)) * locals.var_sp_s_temp) + (assign30990_e46291 * locals.var_sp_s_temp_dn4))), (4.0 * ((((locals.var_sp_s_x0_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign30990_e46291 * locals.var_sp_s_temp_dn5))), (4.0 * ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign30990_e46291 * locals.var_sp_s_temp_dn6))), (4.0 * ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign30990_e46291 * locals.var_sp_s_temp_dn7))), (4.0 * ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign30990_e46291 * locals.var_sp_s_temp_dn8))), (4.0 * ((((locals.var_sp_s_x0_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn9)) * locals.var_sp_s_temp) + (assign30990_e46291 * locals.var_sp_s_temp_dn9))), (4.0 * ((((locals.var_sp_s_x0_dn10 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn10)) * locals.var_sp_s_temp) + (assign30990_e46291 * locals.var_sp_s_temp_dn10))), (4.0 * ((((locals.var_sp_s_x0_dn11 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn11)) * locals.var_sp_s_temp) + (assign30990_e46291 * locals.var_sp_s_temp_dn11))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn3, locals.var_sp_s_xi1_dn4, locals.var_sp_s_xi1_dn5, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, locals.var_sp_s_xi1_dn9, locals.var_sp_s_xi1_dn10, locals.var_sp_s_xi1_dn11,)
    }
};
        locals.var_sp_s_xi1 = assign30990_e46296;
        locals.var_sp_s_xi1_dn3 = assign30990_e46296_d_n3;
        locals.var_sp_s_xi1_dn4 = assign30990_e46296_d_n4;
        locals.var_sp_s_xi1_dn5 = assign30990_e46296_d_n5;
        locals.var_sp_s_xi1_dn6 = assign30990_e46296_d_n6;
        locals.var_sp_s_xi1_dn7 = assign30990_e46296_d_n7;
        locals.var_sp_s_xi1_dn8 = assign30990_e46296_d_n8;
        locals.var_sp_s_xi1_dn9 = assign30990_e46296_d_n9;
        locals.var_sp_s_xi1_dn10 = assign30990_e46296_d_n10;
        locals.var_sp_s_xi1_dn11 = assign30990_e46296_d_n11;
        locals.var_sp_s_xi1_rv = 0.0;

        let (assign31000_e46322, assign31000_e46322_d_n3, assign31000_e46322_d_n4, assign31000_e46322_d_n5, assign31000_e46322_d_n6, assign31000_e46322_d_n7, assign31000_e46322_d_n8, assign31000_e46322_d_n9, assign31000_e46322_d_n10, assign31000_e46322_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign31000_e46312: f64 = (8.0 * locals.var_sp_s_temp);
        let assign31000_e46315: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign31000_e46316: f64 = (assign31000_e46312 - assign31000_e46315);
        let assign31000_e46318: f64 = (assign31000_e46316 * locals.var_sp_s_temp);
        let assign31000_e46320: f64 = (assign31000_e46318 * locals.var_sp_s_temp);
        (assign31000_e46320, ((((((8.0 * locals.var_sp_s_temp_dn3) - (12.0 * locals.var_sp_s_xi0_dn3)) * locals.var_sp_s_temp) + (assign31000_e46316 * locals.var_sp_s_temp_dn3)) * locals.var_sp_s_temp) + (assign31000_e46318 * locals.var_sp_s_temp_dn3)), ((((((8.0 * locals.var_sp_s_temp_dn4) - (12.0 * locals.var_sp_s_xi0_dn4)) * locals.var_sp_s_temp) + (assign31000_e46316 * locals.var_sp_s_temp_dn4)) * locals.var_sp_s_temp) + (assign31000_e46318 * locals.var_sp_s_temp_dn4)), ((((((8.0 * locals.var_sp_s_temp_dn5) - (12.0 * locals.var_sp_s_xi0_dn5)) * locals.var_sp_s_temp) + (assign31000_e46316 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign31000_e46318 * locals.var_sp_s_temp_dn5)), ((((((8.0 * locals.var_sp_s_temp_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp) + (assign31000_e46316 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign31000_e46318 * locals.var_sp_s_temp_dn6)), ((((((8.0 * locals.var_sp_s_temp_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp) + (assign31000_e46316 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign31000_e46318 * locals.var_sp_s_temp_dn7)), ((((((8.0 * locals.var_sp_s_temp_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp) + (assign31000_e46316 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign31000_e46318 * locals.var_sp_s_temp_dn8)), ((((((8.0 * locals.var_sp_s_temp_dn9) - (12.0 * locals.var_sp_s_xi0_dn9)) * locals.var_sp_s_temp) + (assign31000_e46316 * locals.var_sp_s_temp_dn9)) * locals.var_sp_s_temp) + (assign31000_e46318 * locals.var_sp_s_temp_dn9)), ((((((8.0 * locals.var_sp_s_temp_dn10) - (12.0 * locals.var_sp_s_xi0_dn10)) * locals.var_sp_s_temp) + (assign31000_e46316 * locals.var_sp_s_temp_dn10)) * locals.var_sp_s_temp) + (assign31000_e46318 * locals.var_sp_s_temp_dn10)), ((((((8.0 * locals.var_sp_s_temp_dn11) - (12.0 * locals.var_sp_s_xi0_dn11)) * locals.var_sp_s_temp) + (assign31000_e46316 * locals.var_sp_s_temp_dn11)) * locals.var_sp_s_temp) + (assign31000_e46318 * locals.var_sp_s_temp_dn11)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn3, locals.var_sp_s_xi2_dn4, locals.var_sp_s_xi2_dn5, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, locals.var_sp_s_xi2_dn9, locals.var_sp_s_xi2_dn10, locals.var_sp_s_xi2_dn11,)
    }
};
        locals.var_sp_s_xi2 = assign31000_e46322;
        locals.var_sp_s_xi2_dn3 = assign31000_e46322_d_n3;
        locals.var_sp_s_xi2_dn4 = assign31000_e46322_d_n4;
        locals.var_sp_s_xi2_dn5 = assign31000_e46322_d_n5;
        locals.var_sp_s_xi2_dn6 = assign31000_e46322_d_n6;
        locals.var_sp_s_xi2_dn7 = assign31000_e46322_d_n7;
        locals.var_sp_s_xi2_dn8 = assign31000_e46322_d_n8;
        locals.var_sp_s_xi2_dn9 = assign31000_e46322_d_n9;
        locals.var_sp_s_xi2_dn10 = assign31000_e46322_d_n10;
        locals.var_sp_s_xi2_dn11 = assign31000_e46322_d_n11;
        locals.var_sp_s_xi2_rv = 0.0;

        let (assign31010_e46340, assign31010_e46340_d_n3, assign31010_e46340_d_n4, assign31010_e46340_d_n5, assign31010_e46340_d_n6, assign31010_e46340_d_n7, assign31010_e46340_d_n8, assign31010_e46340_d_n9, assign31010_e46340_d_n10, assign31010_e46340_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign31010_e46338: f64 = (locals.var_vgfb1 - locals.var_sp_s_x0);
        (assign31010_e46338, (locals.var_vgfb1_dn3 - locals.var_sp_s_x0_dn3), (locals.var_vgfb1_dn4 - locals.var_sp_s_x0_dn4), (locals.var_vgfb1_dn5 - locals.var_sp_s_x0_dn5), (locals.var_vgfb1_dn6 - locals.var_sp_s_x0_dn6), (locals.var_vgfb1_dn7 - locals.var_sp_s_x0_dn7), (locals.var_vgfb1_dn8 - locals.var_sp_s_x0_dn8), (locals.var_vgfb1_dn9 - locals.var_sp_s_x0_dn9), (locals.var_vgfb1_dn10 - locals.var_sp_s_x0_dn10), (locals.var_vgfb1_dn11 - locals.var_sp_s_x0_dn11),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn3, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9, locals.var_sp_s_temp_dn10, locals.var_sp_s_temp_dn11,)
    }
};
        locals.var_sp_s_temp = assign31010_e46340;
        locals.var_sp_s_temp_dn3 = assign31010_e46340_d_n3;
        locals.var_sp_s_temp_dn4 = assign31010_e46340_d_n4;
        locals.var_sp_s_temp_dn5 = assign31010_e46340_d_n5;
        locals.var_sp_s_temp_dn6 = assign31010_e46340_d_n6;
        locals.var_sp_s_temp_dn7 = assign31010_e46340_d_n7;
        locals.var_sp_s_temp_dn8 = assign31010_e46340_d_n8;
        locals.var_sp_s_temp_dn9 = assign31010_e46340_d_n9;
        locals.var_sp_s_temp_dn10 = assign31010_e46340_d_n10;
        locals.var_sp_s_temp_dn11 = assign31010_e46340_d_n11;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign31020_e46372, assign31020_e46372_d_n3, assign31020_e46372_d_n4, assign31020_e46372_d_n5, assign31020_e46372_d_n6, assign31020_e46372_d_n7, assign31020_e46372_d_n8, assign31020_e46372_d_n9, assign31020_e46372_d_n10, assign31020_e46372_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign31020_e46356: f64 = (2.0 * locals.var_sp_s_temp);
        let assign31020_e46360: f64 = (1.0 - locals.var_sp_s_delta1);
        let assign31020_e46362: f64 = (assign31020_e46360 + locals.var_sp_s_delta0);
        let assign31020_e46366: f64 = (1.0 + locals.var_sp_s_xi1);
        let assign31020_e46367: f64 = (locals.var_exp_ns * assign31020_e46366);
        let assign31020_e46368: f64 = (assign31020_e46362 - assign31020_e46367);
        let assign31020_e46369: f64 = (locals.var_gam2 * assign31020_e46368);
        let assign31020_e46370: f64 = (assign31020_e46356 + assign31020_e46369);
        (assign31020_e46370, ((2.0 * locals.var_sp_s_temp_dn3) + ((locals.var_gam2_dn3 * assign31020_e46368) + (locals.var_gam2 * (((-locals.var_sp_s_delta1_dn3) + locals.var_sp_s_delta0_dn3) - ((locals.var_exp_ns_dn3 * assign31020_e46366) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn3)))))), ((2.0 * locals.var_sp_s_temp_dn4) + ((locals.var_gam2_dn4 * assign31020_e46368) + (locals.var_gam2 * (((-locals.var_sp_s_delta1_dn4) + locals.var_sp_s_delta0_dn4) - ((locals.var_exp_ns_dn4 * assign31020_e46366) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn4)))))), ((2.0 * locals.var_sp_s_temp_dn5) + ((locals.var_gam2_dn5 * assign31020_e46368) + (locals.var_gam2 * (((-locals.var_sp_s_delta1_dn5) + locals.var_sp_s_delta0_dn5) - ((locals.var_exp_ns_dn5 * assign31020_e46366) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn5)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gam2_dn6 * assign31020_e46368) + (locals.var_gam2 * (((-locals.var_sp_s_delta1_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_exp_ns_dn6 * assign31020_e46366) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gam2_dn7 * assign31020_e46368) + (locals.var_gam2 * (((-locals.var_sp_s_delta1_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_exp_ns_dn7 * assign31020_e46366) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gam2_dn8 * assign31020_e46368) + (locals.var_gam2 * (((-locals.var_sp_s_delta1_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_exp_ns_dn8 * assign31020_e46366) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn8)))))), ((2.0 * locals.var_sp_s_temp_dn9) + ((locals.var_gam2_dn9 * assign31020_e46368) + (locals.var_gam2 * (((-locals.var_sp_s_delta1_dn9) + locals.var_sp_s_delta0_dn9) - ((locals.var_exp_ns_dn9 * assign31020_e46366) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn9)))))), ((2.0 * locals.var_sp_s_temp_dn10) + ((locals.var_gam2_dn10 * assign31020_e46368) + (locals.var_gam2 * (((-locals.var_sp_s_delta1_dn10) + locals.var_sp_s_delta0_dn10) - ((locals.var_exp_ns_dn10 * assign31020_e46366) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn10)))))), ((2.0 * locals.var_sp_s_temp_dn11) + ((locals.var_gam2_dn11 * assign31020_e46368) + (locals.var_gam2 * (((-locals.var_sp_s_delta1_dn11) + locals.var_sp_s_delta0_dn11) - ((locals.var_exp_ns_dn11 * assign31020_e46366) + (locals.var_exp_ns * locals.var_sp_s_xi1_dn11)))))),)
    } else {
        (locals.var_sp_s_pc, locals.var_sp_s_pc_dn3, locals.var_sp_s_pc_dn4, locals.var_sp_s_pc_dn5, locals.var_sp_s_pc_dn6, locals.var_sp_s_pc_dn7, locals.var_sp_s_pc_dn8, locals.var_sp_s_pc_dn9, locals.var_sp_s_pc_dn10, locals.var_sp_s_pc_dn11,)
    }
};
        locals.var_sp_s_pc = assign31020_e46372;
        locals.var_sp_s_pc_dn3 = assign31020_e46372_d_n3;
        locals.var_sp_s_pc_dn4 = assign31020_e46372_d_n4;
        locals.var_sp_s_pc_dn5 = assign31020_e46372_d_n5;
        locals.var_sp_s_pc_dn6 = assign31020_e46372_d_n6;
        locals.var_sp_s_pc_dn7 = assign31020_e46372_d_n7;
        locals.var_sp_s_pc_dn8 = assign31020_e46372_d_n8;
        locals.var_sp_s_pc_dn9 = assign31020_e46372_d_n9;
        locals.var_sp_s_pc_dn10 = assign31020_e46372_d_n10;
        locals.var_sp_s_pc_dn11 = assign31020_e46372_d_n11;
        locals.var_sp_s_pc_rv = 0.0;

        let (assign31030_e46408, assign31030_e46408_d_n3, assign31030_e46408_d_n4, assign31030_e46408_d_n5, assign31030_e46408_d_n6, assign31030_e46408_d_n7, assign31030_e46408_d_n8, assign31030_e46408_d_n9, assign31030_e46408_d_n10, assign31030_e46408_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign31030_e46388: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign31030_e46392: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_x0);
        let assign31030_e46394: f64 = (assign31030_e46392 - 1.0);
        let assign31030_e46396: f64 = (assign31030_e46394 + locals.var_sp_s_delta0);
        let assign31030_e46400: f64 = (locals.var_sp_s_x0 + 1.0);
        let assign31030_e46402: f64 = (assign31030_e46400 + locals.var_sp_s_xi0);
        let assign31030_e46403: f64 = (locals.var_exp_ns * assign31030_e46402);
        let assign31030_e46404: f64 = (assign31030_e46396 - assign31030_e46403);
        let assign31030_e46405: f64 = (locals.var_gam2 * assign31030_e46404);
        let assign31030_e46406: f64 = (assign31030_e46388 - assign31030_e46405);
        (assign31030_e46406, (((locals.var_sp_s_temp_dn3 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn3)) - ((locals.var_gam2_dn3 * assign31030_e46404) + (locals.var_gam2 * (((locals.var_sp_s_delta1_dn3 + locals.var_sp_s_x0_dn3) + locals.var_sp_s_delta0_dn3) - ((locals.var_exp_ns_dn3 * assign31030_e46402) + (locals.var_exp_ns * (locals.var_sp_s_x0_dn3 + locals.var_sp_s_xi0_dn3))))))), (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) - ((locals.var_gam2_dn4 * assign31030_e46404) + (locals.var_gam2 * (((locals.var_sp_s_delta1_dn4 + locals.var_sp_s_x0_dn4) + locals.var_sp_s_delta0_dn4) - ((locals.var_exp_ns_dn4 * assign31030_e46402) + (locals.var_exp_ns * (locals.var_sp_s_x0_dn4 + locals.var_sp_s_xi0_dn4))))))), (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) - ((locals.var_gam2_dn5 * assign31030_e46404) + (locals.var_gam2 * (((locals.var_sp_s_delta1_dn5 + locals.var_sp_s_x0_dn5) + locals.var_sp_s_delta0_dn5) - ((locals.var_exp_ns_dn5 * assign31030_e46402) + (locals.var_exp_ns * (locals.var_sp_s_x0_dn5 + locals.var_sp_s_xi0_dn5))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gam2_dn6 * assign31030_e46404) + (locals.var_gam2 * (((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_x0_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_exp_ns_dn6 * assign31030_e46402) + (locals.var_exp_ns * (locals.var_sp_s_x0_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gam2_dn7 * assign31030_e46404) + (locals.var_gam2 * (((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_x0_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_exp_ns_dn7 * assign31030_e46402) + (locals.var_exp_ns * (locals.var_sp_s_x0_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gam2_dn8 * assign31030_e46404) + (locals.var_gam2 * (((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_x0_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_exp_ns_dn8 * assign31030_e46402) + (locals.var_exp_ns * (locals.var_sp_s_x0_dn8 + locals.var_sp_s_xi0_dn8))))))), (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) - ((locals.var_gam2_dn9 * assign31030_e46404) + (locals.var_gam2 * (((locals.var_sp_s_delta1_dn9 + locals.var_sp_s_x0_dn9) + locals.var_sp_s_delta0_dn9) - ((locals.var_exp_ns_dn9 * assign31030_e46402) + (locals.var_exp_ns * (locals.var_sp_s_x0_dn9 + locals.var_sp_s_xi0_dn9))))))), (((locals.var_sp_s_temp_dn10 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn10)) - ((locals.var_gam2_dn10 * assign31030_e46404) + (locals.var_gam2 * (((locals.var_sp_s_delta1_dn10 + locals.var_sp_s_x0_dn10) + locals.var_sp_s_delta0_dn10) - ((locals.var_exp_ns_dn10 * assign31030_e46402) + (locals.var_exp_ns * (locals.var_sp_s_x0_dn10 + locals.var_sp_s_xi0_dn10))))))), (((locals.var_sp_s_temp_dn11 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn11)) - ((locals.var_gam2_dn11 * assign31030_e46404) + (locals.var_gam2 * (((locals.var_sp_s_delta1_dn11 + locals.var_sp_s_x0_dn11) + locals.var_sp_s_delta0_dn11) - ((locals.var_exp_ns_dn11 * assign31030_e46402) + (locals.var_exp_ns * (locals.var_sp_s_x0_dn11 + locals.var_sp_s_xi0_dn11))))))),)
    } else {
        (locals.var_sp_s_qc, locals.var_sp_s_qc_dn3, locals.var_sp_s_qc_dn4, locals.var_sp_s_qc_dn5, locals.var_sp_s_qc_dn6, locals.var_sp_s_qc_dn7, locals.var_sp_s_qc_dn8, locals.var_sp_s_qc_dn9, locals.var_sp_s_qc_dn10, locals.var_sp_s_qc_dn11,)
    }
};
        locals.var_sp_s_qc = assign31030_e46408;
        locals.var_sp_s_qc_dn3 = assign31030_e46408_d_n3;
        locals.var_sp_s_qc_dn4 = assign31030_e46408_d_n4;
        locals.var_sp_s_qc_dn5 = assign31030_e46408_d_n5;
        locals.var_sp_s_qc_dn6 = assign31030_e46408_d_n6;
        locals.var_sp_s_qc_dn7 = assign31030_e46408_d_n7;
        locals.var_sp_s_qc_dn8 = assign31030_e46408_d_n8;
        locals.var_sp_s_qc_dn9 = assign31030_e46408_d_n9;
        locals.var_sp_s_qc_dn10 = assign31030_e46408_d_n10;
        locals.var_sp_s_qc_dn11 = assign31030_e46408_d_n11;
        locals.var_sp_s_qc_rv = 0.0;

        let (assign31040_e46434, assign31040_e46434_d_n3, assign31040_e46434_d_n4, assign31040_e46434_d_n5, assign31040_e46434_d_n6, assign31040_e46434_d_n7, assign31040_e46434_d_n8, assign31040_e46434_d_n9, assign31040_e46434_d_n10, assign31040_e46434_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign31040_e46426: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_delta0);
        let assign31040_e46429: f64 = (locals.var_exp_ns * locals.var_sp_s_xi2);
        let assign31040_e46430: f64 = (assign31040_e46426 - assign31040_e46429);
        let assign31040_e46431: f64 = (locals.var_gam2 * assign31040_e46430);
        let assign31040_e46432: f64 = (2.0 - assign31040_e46431);
        (assign31040_e46432, (-((locals.var_gam2_dn3 * assign31040_e46430) + (locals.var_gam2 * ((locals.var_sp_s_delta1_dn3 + locals.var_sp_s_delta0_dn3) - ((locals.var_exp_ns_dn3 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn3)))))), (-((locals.var_gam2_dn4 * assign31040_e46430) + (locals.var_gam2 * ((locals.var_sp_s_delta1_dn4 + locals.var_sp_s_delta0_dn4) - ((locals.var_exp_ns_dn4 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn4)))))), (-((locals.var_gam2_dn5 * assign31040_e46430) + (locals.var_gam2 * ((locals.var_sp_s_delta1_dn5 + locals.var_sp_s_delta0_dn5) - ((locals.var_exp_ns_dn5 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn5)))))), (-((locals.var_gam2_dn6 * assign31040_e46430) + (locals.var_gam2 * ((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_delta0_dn6) - ((locals.var_exp_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn6)))))), (-((locals.var_gam2_dn7 * assign31040_e46430) + (locals.var_gam2 * ((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_delta0_dn7) - ((locals.var_exp_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn7)))))), (-((locals.var_gam2_dn8 * assign31040_e46430) + (locals.var_gam2 * ((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_delta0_dn8) - ((locals.var_exp_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn8)))))), (-((locals.var_gam2_dn9 * assign31040_e46430) + (locals.var_gam2 * ((locals.var_sp_s_delta1_dn9 + locals.var_sp_s_delta0_dn9) - ((locals.var_exp_ns_dn9 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn9)))))), (-((locals.var_gam2_dn10 * assign31040_e46430) + (locals.var_gam2 * ((locals.var_sp_s_delta1_dn10 + locals.var_sp_s_delta0_dn10) - ((locals.var_exp_ns_dn10 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn10)))))), (-((locals.var_gam2_dn11 * assign31040_e46430) + (locals.var_gam2 * ((locals.var_sp_s_delta1_dn11 + locals.var_sp_s_delta0_dn11) - ((locals.var_exp_ns_dn11 * locals.var_sp_s_xi2) + (locals.var_exp_ns * locals.var_sp_s_xi2_dn11)))))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn3, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9, locals.var_sp_s_temp_dn10, locals.var_sp_s_temp_dn11,)
    }
};
        locals.var_sp_s_temp = assign31040_e46434;
        locals.var_sp_s_temp_dn3 = assign31040_e46434_d_n3;
        locals.var_sp_s_temp_dn4 = assign31040_e46434_d_n4;
        locals.var_sp_s_temp_dn5 = assign31040_e46434_d_n5;
        locals.var_sp_s_temp_dn6 = assign31040_e46434_d_n6;
        locals.var_sp_s_temp_dn7 = assign31040_e46434_d_n7;
        locals.var_sp_s_temp_dn8 = assign31040_e46434_d_n8;
        locals.var_sp_s_temp_dn9 = assign31040_e46434_d_n9;
        locals.var_sp_s_temp_dn10 = assign31040_e46434_d_n10;
        locals.var_sp_s_temp_dn11 = assign31040_e46434_d_n11;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign31050_e46458, assign31050_e46458_d_n3, assign31050_e46458_d_n4, assign31050_e46458_d_n5, assign31050_e46458_d_n6, assign31050_e46458_d_n7, assign31050_e46458_d_n8, assign31050_e46458_d_n9, assign31050_e46458_d_n10, assign31050_e46458_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign31050_e46450: f64 = (locals.var_sp_s_pc * locals.var_sp_s_pc);
        let assign31050_e46454: f64 = (locals.var_sp_s_qc * locals.var_sp_s_temp);
        let assign31050_e46455: f64 = (2.0 * assign31050_e46454);
        let assign31050_e46456: f64 = (assign31050_e46450 - assign31050_e46455);
        (assign31050_e46456, (((locals.var_sp_s_pc_dn3 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn3)) - (2.0 * ((locals.var_sp_s_qc_dn3 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn3)))), (((locals.var_sp_s_pc_dn4 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn4)) - (2.0 * ((locals.var_sp_s_qc_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn4)))), (((locals.var_sp_s_pc_dn5 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn5)) - (2.0 * ((locals.var_sp_s_qc_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn5)))), (((locals.var_sp_s_pc_dn6 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn6)) - (2.0 * ((locals.var_sp_s_qc_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn6)))), (((locals.var_sp_s_pc_dn7 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn7)) - (2.0 * ((locals.var_sp_s_qc_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn7)))), (((locals.var_sp_s_pc_dn8 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn8)) - (2.0 * ((locals.var_sp_s_qc_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn8)))), (((locals.var_sp_s_pc_dn9 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn9)) - (2.0 * ((locals.var_sp_s_qc_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn9)))), (((locals.var_sp_s_pc_dn10 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn10)) - (2.0 * ((locals.var_sp_s_qc_dn10 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn10)))), (((locals.var_sp_s_pc_dn11 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn11)) - (2.0 * ((locals.var_sp_s_qc_dn11 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn11)))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn3, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9, locals.var_sp_s_temp_dn10, locals.var_sp_s_temp_dn11,)
    }
};
        locals.var_sp_s_temp = assign31050_e46458;
        locals.var_sp_s_temp_dn3 = assign31050_e46458_d_n3;
        locals.var_sp_s_temp_dn4 = assign31050_e46458_d_n4;
        locals.var_sp_s_temp_dn5 = assign31050_e46458_d_n5;
        locals.var_sp_s_temp_dn6 = assign31050_e46458_d_n6;
        locals.var_sp_s_temp_dn7 = assign31050_e46458_d_n7;
        locals.var_sp_s_temp_dn8 = assign31050_e46458_d_n8;
        locals.var_sp_s_temp_dn9 = assign31050_e46458_d_n9;
        locals.var_sp_s_temp_dn10 = assign31050_e46458_d_n10;
        locals.var_sp_s_temp_dn11 = assign31050_e46458_d_n11;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign31060_e46483, assign31060_e46483_d_n3, assign31060_e46483_d_n4, assign31060_e46483_d_n5, assign31060_e46483_d_n6, assign31060_e46483_d_n7, assign31060_e46483_d_n8, assign31060_e46483_d_n9, assign31060_e46483_d_n10, assign31060_e46483_d_n11,) = {
    if ((((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 != 0.0)) && (locals.var_guard619 == 0.0)) && (locals.var_guard620 == 0.0)) {
        let assign31060_e46477: f64 = (locals.var_sp_s_temp).sqrt();
        let assign31060_e46478: f64 = (locals.var_sp_s_pc + assign31060_e46477);
        let assign31060_e46479: f64 = (locals.var_sp_s_qc / assign31060_e46478);
        let assign31060_e46480: f64 = (2.0 * assign31060_e46479);
        let assign31060_e46481: f64 = (locals.var_sp_s_x0 + assign31060_e46480);
        (assign31060_e46481, (locals.var_sp_s_x0_dn3 + (2.0 * (((locals.var_sp_s_qc_dn3 * assign31060_e46478) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn3 + (locals.var_sp_s_temp_dn3 / (2.0 * assign31060_e46477))))) / (assign31060_e46478 * assign31060_e46478)))), (locals.var_sp_s_x0_dn4 + (2.0 * (((locals.var_sp_s_qc_dn4 * assign31060_e46478) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn4 + (locals.var_sp_s_temp_dn4 / (2.0 * assign31060_e46477))))) / (assign31060_e46478 * assign31060_e46478)))), (locals.var_sp_s_x0_dn5 + (2.0 * (((locals.var_sp_s_qc_dn5 * assign31060_e46478) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn5 + (locals.var_sp_s_temp_dn5 / (2.0 * assign31060_e46477))))) / (assign31060_e46478 * assign31060_e46478)))), (locals.var_sp_s_x0_dn6 + (2.0 * (((locals.var_sp_s_qc_dn6 * assign31060_e46478) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn6 + (locals.var_sp_s_temp_dn6 / (2.0 * assign31060_e46477))))) / (assign31060_e46478 * assign31060_e46478)))), (locals.var_sp_s_x0_dn7 + (2.0 * (((locals.var_sp_s_qc_dn7 * assign31060_e46478) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn7 + (locals.var_sp_s_temp_dn7 / (2.0 * assign31060_e46477))))) / (assign31060_e46478 * assign31060_e46478)))), (locals.var_sp_s_x0_dn8 + (2.0 * (((locals.var_sp_s_qc_dn8 * assign31060_e46478) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn8 + (locals.var_sp_s_temp_dn8 / (2.0 * assign31060_e46477))))) / (assign31060_e46478 * assign31060_e46478)))), (locals.var_sp_s_x0_dn9 + (2.0 * (((locals.var_sp_s_qc_dn9 * assign31060_e46478) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn9 + (locals.var_sp_s_temp_dn9 / (2.0 * assign31060_e46477))))) / (assign31060_e46478 * assign31060_e46478)))), (locals.var_sp_s_x0_dn10 + (2.0 * (((locals.var_sp_s_qc_dn10 * assign31060_e46478) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn10 + (locals.var_sp_s_temp_dn10 / (2.0 * assign31060_e46477))))) / (assign31060_e46478 * assign31060_e46478)))), (locals.var_sp_s_x0_dn11 + (2.0 * (((locals.var_sp_s_qc_dn11 * assign31060_e46478) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn11 + (locals.var_sp_s_temp_dn11 / (2.0 * assign31060_e46477))))) / (assign31060_e46478 * assign31060_e46478)))),)
    } else {
        (locals.var_sp_dd, locals.var_sp_dd_dn3, locals.var_sp_dd_dn4, locals.var_sp_dd_dn5, locals.var_sp_dd_dn6, locals.var_sp_dd_dn7, locals.var_sp_dd_dn8, locals.var_sp_dd_dn9, locals.var_sp_dd_dn10, locals.var_sp_dd_dn11,)
    }
};
        locals.var_sp_dd = assign31060_e46483;
        locals.var_sp_dd_dn3 = assign31060_e46483_d_n3;
        locals.var_sp_dd_dn4 = assign31060_e46483_d_n4;
        locals.var_sp_dd_dn5 = assign31060_e46483_d_n5;
        locals.var_sp_dd_dn6 = assign31060_e46483_d_n6;
        locals.var_sp_dd_dn7 = assign31060_e46483_d_n7;
        locals.var_sp_dd_dn8 = assign31060_e46483_d_n8;
        locals.var_sp_dd_dn9 = assign31060_e46483_d_n9;
        locals.var_sp_dd_dn10 = assign31060_e46483_d_n10;
        locals.var_sp_dd_dn11 = assign31060_e46483_d_n11;
        locals.var_sp_dd_rv = 0.0;

        let (assign31070_e46494,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        (locals.var_rt,)
    } else {
        (locals.var_zeta,)
    }
};
        locals.var_zeta = assign31070_e46494;
        locals.var_zeta_rv = 0.0;

        let (assign31080_e46507,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31080_e46505: f64 = (locals.var_zeta * locals.var_zeta);
        (assign31080_e46505,)
    } else {
        (locals.var_zeta2,)
    }
};
        locals.var_zeta2 = assign31080_e46507;
        locals.var_zeta2_rv = 0.0;

        let (assign31090_e46522, assign31090_e46522_d_n3, assign31090_e46522_d_n4, assign31090_e46522_d_n5, assign31090_e46522_d_n6, assign31090_e46522_d_n7, assign31090_e46522_d_n8, assign31090_e46522_d_n9, assign31090_e46522_d_n10, assign31090_e46522_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31090_e46519: f64 = (locals.var_phisb0 * locals.var_inv_vt);
        let assign31090_e46520: f64 = (locals.var_phic_star - assign31090_e46519);
        (assign31090_e46520, (locals.var_phic_star_dn3 - (locals.var_phisb0_dn3 * locals.var_inv_vt)), (locals.var_phic_star_dn4 - ((locals.var_phisb0_dn4 * locals.var_inv_vt) + (locals.var_phisb0 * locals.var_inv_vt_dn4))), (locals.var_phic_star_dn5 - ((locals.var_phisb0_dn5 * locals.var_inv_vt) + (locals.var_phisb0 * locals.var_inv_vt_dn5))), (locals.var_phic_star_dn6 - (locals.var_phisb0_dn6 * locals.var_inv_vt)), (locals.var_phic_star_dn7 - (locals.var_phisb0_dn7 * locals.var_inv_vt)), (locals.var_phic_star_dn8 - (locals.var_phisb0_dn8 * locals.var_inv_vt)), (locals.var_phic_star_dn9 - (locals.var_phisb0_dn9 * locals.var_inv_vt)), (locals.var_phic_star_dn10 - (locals.var_phisb0_dn10 * locals.var_inv_vt)), (locals.var_phic_star_dn11 - (locals.var_phisb0_dn11 * locals.var_inv_vt)),)
    } else {
        (locals.var_u_crit, locals.var_u_crit_dn3, locals.var_u_crit_dn4, locals.var_u_crit_dn5, locals.var_u_crit_dn6, locals.var_u_crit_dn7, locals.var_u_crit_dn8, locals.var_u_crit_dn9, locals.var_u_crit_dn10, locals.var_u_crit_dn11,)
    }
};
        locals.var_u_crit = assign31090_e46522;
        locals.var_u_crit_dn3 = assign31090_e46522_d_n3;
        locals.var_u_crit_dn4 = assign31090_e46522_d_n4;
        locals.var_u_crit_dn5 = assign31090_e46522_d_n5;
        locals.var_u_crit_dn6 = assign31090_e46522_d_n6;
        locals.var_u_crit_dn7 = assign31090_e46522_d_n7;
        locals.var_u_crit_dn8 = assign31090_e46522_d_n8;
        locals.var_u_crit_dn9 = assign31090_e46522_d_n9;
        locals.var_u_crit_dn10 = assign31090_e46522_d_n10;
        locals.var_u_crit_dn11 = assign31090_e46522_d_n11;
        locals.var_u_crit_rv = 0.0;

        let (assign31100_e46544, assign31100_e46544_d_n3, assign31100_e46544_d_n4, assign31100_e46544_d_n5, assign31100_e46544_d_n6, assign31100_e46544_d_n7, assign31100_e46544_d_n8, assign31100_e46544_d_n9, assign31100_e46544_d_n10, assign31100_e46544_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31100_e46534: f64 = (-locals.var_u_crit);
        let assign31100_e46535: f64 = { let limited_exp_arg = assign31100_e46534; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign31100_e46537: f64 = (assign31100_e46535 + locals.var_u_crit);
        let assign31100_e46539: f64 = (assign31100_e46537 - 1.0);
        let assign31100_e46540: f64 = (assign31100_e46539).sqrt();
        let assign31100_e46541: f64 = (locals.var_gam * assign31100_e46540);
        let assign31100_e46542: f64 = (locals.var_vgfb1 - assign31100_e46541);
        (assign31100_e46542, (locals.var_vgfb1_dn3 - ((locals.var_gam_dn3 * assign31100_e46540) + (locals.var_gam * ((({ let limited_exp_arg = assign31100_e46534; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_u_crit_dn3)) + locals.var_u_crit_dn3) / (2.0 * assign31100_e46540))))), (locals.var_vgfb1_dn4 - ((locals.var_gam_dn4 * assign31100_e46540) + (locals.var_gam * ((({ let limited_exp_arg = assign31100_e46534; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_u_crit_dn4)) + locals.var_u_crit_dn4) / (2.0 * assign31100_e46540))))), (locals.var_vgfb1_dn5 - ((locals.var_gam_dn5 * assign31100_e46540) + (locals.var_gam * ((({ let limited_exp_arg = assign31100_e46534; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_u_crit_dn5)) + locals.var_u_crit_dn5) / (2.0 * assign31100_e46540))))), (locals.var_vgfb1_dn6 - ((locals.var_gam_dn6 * assign31100_e46540) + (locals.var_gam * ((({ let limited_exp_arg = assign31100_e46534; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_u_crit_dn6)) + locals.var_u_crit_dn6) / (2.0 * assign31100_e46540))))), (locals.var_vgfb1_dn7 - ((locals.var_gam_dn7 * assign31100_e46540) + (locals.var_gam * ((({ let limited_exp_arg = assign31100_e46534; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_u_crit_dn7)) + locals.var_u_crit_dn7) / (2.0 * assign31100_e46540))))), (locals.var_vgfb1_dn8 - ((locals.var_gam_dn8 * assign31100_e46540) + (locals.var_gam * ((({ let limited_exp_arg = assign31100_e46534; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_u_crit_dn8)) + locals.var_u_crit_dn8) / (2.0 * assign31100_e46540))))), (locals.var_vgfb1_dn9 - ((locals.var_gam_dn9 * assign31100_e46540) + (locals.var_gam * ((({ let limited_exp_arg = assign31100_e46534; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_u_crit_dn9)) + locals.var_u_crit_dn9) / (2.0 * assign31100_e46540))))), (locals.var_vgfb1_dn10 - ((locals.var_gam_dn10 * assign31100_e46540) + (locals.var_gam * ((({ let limited_exp_arg = assign31100_e46534; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_u_crit_dn10)) + locals.var_u_crit_dn10) / (2.0 * assign31100_e46540))))), (locals.var_vgfb1_dn11 - ((locals.var_gam_dn11 * assign31100_e46540) + (locals.var_gam * ((({ let limited_exp_arg = assign31100_e46534; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_u_crit_dn11)) + locals.var_u_crit_dn11) / (2.0 * assign31100_e46540))))),)
    } else {
        (locals.var_usf_fd, locals.var_usf_fd_dn3, locals.var_usf_fd_dn4, locals.var_usf_fd_dn5, locals.var_usf_fd_dn6, locals.var_usf_fd_dn7, locals.var_usf_fd_dn8, locals.var_usf_fd_dn9, locals.var_usf_fd_dn10, locals.var_usf_fd_dn11,)
    }
};
        locals.var_usf_fd = assign31100_e46544;
        locals.var_usf_fd_dn3 = assign31100_e46544_d_n3;
        locals.var_usf_fd_dn4 = assign31100_e46544_d_n4;
        locals.var_usf_fd_dn5 = assign31100_e46544_d_n5;
        locals.var_usf_fd_dn6 = assign31100_e46544_d_n6;
        locals.var_usf_fd_dn7 = assign31100_e46544_d_n7;
        locals.var_usf_fd_dn8 = assign31100_e46544_d_n8;
        locals.var_usf_fd_dn9 = assign31100_e46544_d_n9;
        locals.var_usf_fd_dn10 = assign31100_e46544_d_n10;
        locals.var_usf_fd_dn11 = assign31100_e46544_d_n11;
        locals.var_usf_fd_rv = 0.0;

        let (assign31110_e46557, assign31110_e46557_d_n3, assign31110_e46557_d_n4, assign31110_e46557_d_n5, assign31110_e46557_d_n6, assign31110_e46557_d_n7, assign31110_e46557_d_n8, assign31110_e46557_d_n9, assign31110_e46557_d_n10, assign31110_e46557_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31110_e46555: f64 = (locals.var_phisf + 3.0);
        (assign31110_e46555, locals.var_phisf_dn3, locals.var_phisf_dn4, locals.var_phisf_dn5, locals.var_phisf_dn6, locals.var_phisf_dn7, locals.var_phisf_dn8, locals.var_phisf_dn9, locals.var_phisf_dn10, locals.var_phisf_dn11,)
    } else {
        (locals.var_sp_s_bx, locals.var_sp_s_bx_dn3, locals.var_sp_s_bx_dn4, locals.var_sp_s_bx_dn5, locals.var_sp_s_bx_dn6, locals.var_sp_s_bx_dn7, locals.var_sp_s_bx_dn8, locals.var_sp_s_bx_dn9, locals.var_sp_s_bx_dn10, locals.var_sp_s_bx_dn11,)
    }
};
        locals.var_sp_s_bx = assign31110_e46557;
        locals.var_sp_s_bx_dn3 = assign31110_e46557_d_n3;
        locals.var_sp_s_bx_dn4 = assign31110_e46557_d_n4;
        locals.var_sp_s_bx_dn5 = assign31110_e46557_d_n5;
        locals.var_sp_s_bx_dn6 = assign31110_e46557_d_n6;
        locals.var_sp_s_bx_dn7 = assign31110_e46557_d_n7;
        locals.var_sp_s_bx_dn8 = assign31110_e46557_d_n8;
        locals.var_sp_s_bx_dn9 = assign31110_e46557_d_n9;
        locals.var_sp_s_bx_dn10 = assign31110_e46557_d_n10;
        locals.var_sp_s_bx_dn11 = assign31110_e46557_d_n11;
        locals.var_sp_s_bx_rv = 0.0;

        let (assign31120_e46583, assign31120_e46583_d_n3, assign31120_e46583_d_n4, assign31120_e46583_d_n5, assign31120_e46583_d_n6, assign31120_e46583_d_n7, assign31120_e46583_d_n8, assign31120_e46583_d_n9, assign31120_e46583_d_n10, assign31120_e46583_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31120_e46569: f64 = (locals.var_usf_fd + locals.var_sp_s_bx);
        let assign31120_e46572: f64 = (locals.var_usf_fd - locals.var_sp_s_bx);
        let assign31120_e46575: f64 = (locals.var_usf_fd - locals.var_sp_s_bx);
        let assign31120_e46576: f64 = (assign31120_e46572 * assign31120_e46575);
        let assign31120_e46578: f64 = (assign31120_e46576 + 40.0);
        let assign31120_e46579: f64 = (assign31120_e46578).sqrt();
        let assign31120_e46580: f64 = (assign31120_e46569 - assign31120_e46579);
        let assign31120_e46581: f64 = (0.5 * assign31120_e46580);
        (assign31120_e46581, (0.5 * ((locals.var_usf_fd_dn3 + locals.var_sp_s_bx_dn3) - ((((locals.var_usf_fd_dn3 - locals.var_sp_s_bx_dn3) * assign31120_e46575) + (assign31120_e46572 * (locals.var_usf_fd_dn3 - locals.var_sp_s_bx_dn3))) / (2.0 * assign31120_e46579)))), (0.5 * ((locals.var_usf_fd_dn4 + locals.var_sp_s_bx_dn4) - ((((locals.var_usf_fd_dn4 - locals.var_sp_s_bx_dn4) * assign31120_e46575) + (assign31120_e46572 * (locals.var_usf_fd_dn4 - locals.var_sp_s_bx_dn4))) / (2.0 * assign31120_e46579)))), (0.5 * ((locals.var_usf_fd_dn5 + locals.var_sp_s_bx_dn5) - ((((locals.var_usf_fd_dn5 - locals.var_sp_s_bx_dn5) * assign31120_e46575) + (assign31120_e46572 * (locals.var_usf_fd_dn5 - locals.var_sp_s_bx_dn5))) / (2.0 * assign31120_e46579)))), (0.5 * ((locals.var_usf_fd_dn6 + locals.var_sp_s_bx_dn6) - ((((locals.var_usf_fd_dn6 - locals.var_sp_s_bx_dn6) * assign31120_e46575) + (assign31120_e46572 * (locals.var_usf_fd_dn6 - locals.var_sp_s_bx_dn6))) / (2.0 * assign31120_e46579)))), (0.5 * ((locals.var_usf_fd_dn7 + locals.var_sp_s_bx_dn7) - ((((locals.var_usf_fd_dn7 - locals.var_sp_s_bx_dn7) * assign31120_e46575) + (assign31120_e46572 * (locals.var_usf_fd_dn7 - locals.var_sp_s_bx_dn7))) / (2.0 * assign31120_e46579)))), (0.5 * ((locals.var_usf_fd_dn8 + locals.var_sp_s_bx_dn8) - ((((locals.var_usf_fd_dn8 - locals.var_sp_s_bx_dn8) * assign31120_e46575) + (assign31120_e46572 * (locals.var_usf_fd_dn8 - locals.var_sp_s_bx_dn8))) / (2.0 * assign31120_e46579)))), (0.5 * ((locals.var_usf_fd_dn9 + locals.var_sp_s_bx_dn9) - ((((locals.var_usf_fd_dn9 - locals.var_sp_s_bx_dn9) * assign31120_e46575) + (assign31120_e46572 * (locals.var_usf_fd_dn9 - locals.var_sp_s_bx_dn9))) / (2.0 * assign31120_e46579)))), (0.5 * ((locals.var_usf_fd_dn10 + locals.var_sp_s_bx_dn10) - ((((locals.var_usf_fd_dn10 - locals.var_sp_s_bx_dn10) * assign31120_e46575) + (assign31120_e46572 * (locals.var_usf_fd_dn10 - locals.var_sp_s_bx_dn10))) / (2.0 * assign31120_e46579)))), (0.5 * ((locals.var_usf_fd_dn11 + locals.var_sp_s_bx_dn11) - ((((locals.var_usf_fd_dn11 - locals.var_sp_s_bx_dn11) * assign31120_e46575) + (assign31120_e46572 * (locals.var_usf_fd_dn11 - locals.var_sp_s_bx_dn11))) / (2.0 * assign31120_e46579)))),)
    } else {
        (locals.var_sp_s_z0, locals.var_sp_s_z0_dn3, locals.var_sp_s_z0_dn4, locals.var_sp_s_z0_dn5, locals.var_sp_s_z0_dn6, locals.var_sp_s_z0_dn7, locals.var_sp_s_z0_dn8, locals.var_sp_s_z0_dn9, locals.var_sp_s_z0_dn10, locals.var_sp_s_z0_dn11,)
    }
};
        locals.var_sp_s_z0 = assign31120_e46583;
        locals.var_sp_s_z0_dn3 = assign31120_e46583_d_n3;
        locals.var_sp_s_z0_dn4 = assign31120_e46583_d_n4;
        locals.var_sp_s_z0_dn5 = assign31120_e46583_d_n5;
        locals.var_sp_s_z0_dn6 = assign31120_e46583_d_n6;
        locals.var_sp_s_z0_dn7 = assign31120_e46583_d_n7;
        locals.var_sp_s_z0_dn8 = assign31120_e46583_d_n8;
        locals.var_sp_s_z0_dn9 = assign31120_e46583_d_n9;
        locals.var_sp_s_z0_dn10 = assign31120_e46583_d_n10;
        locals.var_sp_s_z0_dn11 = assign31120_e46583_d_n11;
        locals.var_sp_s_z0_rv = 0.0;

        let (assign31130_e46618, assign31130_e46618_d_n3, assign31130_e46618_d_n4, assign31130_e46618_d_n5, assign31130_e46618_d_n6, assign31130_e46618_d_n7, assign31130_e46618_d_n8, assign31130_e46618_d_n9, assign31130_e46618_d_n10, assign31130_e46618_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31130_e46594: f64 = (locals.var_vgfb1 - locals.var_sp_s_z0);
        let assign31130_e46597: f64 = (locals.var_vgfb1 - locals.var_sp_s_z0);
        let assign31130_e46598: f64 = (assign31130_e46594 * assign31130_e46597);
        let assign31130_e46602: f64 = (locals.var_vgfbb - locals.var_sp_s_z0);
        let assign31130_e46604: f64 = (assign31130_e46602 + locals.var_phic_star);
        let assign31130_e46605: f64 = (locals.var_zeta2 * assign31130_e46604);
        let assign31130_e46608: f64 = (locals.var_vgfbb - locals.var_sp_s_z0);
        let assign31130_e46610: f64 = (assign31130_e46608 + locals.var_phic_star);
        let assign31130_e46611: f64 = (assign31130_e46605 * assign31130_e46610);
        let assign31130_e46612: f64 = (assign31130_e46598 - assign31130_e46611);
        let assign31130_e46615: f64 = (locals.var_gam2 * locals.var_phic_star);
        let assign31130_e46616: f64 = (assign31130_e46612 - assign31130_e46615);
        (assign31130_e46616, (((((locals.var_vgfb1_dn3 - locals.var_sp_s_z0_dn3) * assign31130_e46597) + (assign31130_e46594 * (locals.var_vgfb1_dn3 - locals.var_sp_s_z0_dn3))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn3 - locals.var_sp_s_z0_dn3) + locals.var_phic_star_dn3)) * assign31130_e46610) + (assign31130_e46605 * ((locals.var_vgfbb_dn3 - locals.var_sp_s_z0_dn3) + locals.var_phic_star_dn3)))) - ((locals.var_gam2_dn3 * locals.var_phic_star) + (locals.var_gam2 * locals.var_phic_star_dn3))), (((((locals.var_vgfb1_dn4 - locals.var_sp_s_z0_dn4) * assign31130_e46597) + (assign31130_e46594 * (locals.var_vgfb1_dn4 - locals.var_sp_s_z0_dn4))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn4 - locals.var_sp_s_z0_dn4) + locals.var_phic_star_dn4)) * assign31130_e46610) + (assign31130_e46605 * ((locals.var_vgfbb_dn4 - locals.var_sp_s_z0_dn4) + locals.var_phic_star_dn4)))) - ((locals.var_gam2_dn4 * locals.var_phic_star) + (locals.var_gam2 * locals.var_phic_star_dn4))), (((((locals.var_vgfb1_dn5 - locals.var_sp_s_z0_dn5) * assign31130_e46597) + (assign31130_e46594 * (locals.var_vgfb1_dn5 - locals.var_sp_s_z0_dn5))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn5 - locals.var_sp_s_z0_dn5) + locals.var_phic_star_dn5)) * assign31130_e46610) + (assign31130_e46605 * ((locals.var_vgfbb_dn5 - locals.var_sp_s_z0_dn5) + locals.var_phic_star_dn5)))) - ((locals.var_gam2_dn5 * locals.var_phic_star) + (locals.var_gam2 * locals.var_phic_star_dn5))), (((((locals.var_vgfb1_dn6 - locals.var_sp_s_z0_dn6) * assign31130_e46597) + (assign31130_e46594 * (locals.var_vgfb1_dn6 - locals.var_sp_s_z0_dn6))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn6 - locals.var_sp_s_z0_dn6) + locals.var_phic_star_dn6)) * assign31130_e46610) + (assign31130_e46605 * ((locals.var_vgfbb_dn6 - locals.var_sp_s_z0_dn6) + locals.var_phic_star_dn6)))) - ((locals.var_gam2_dn6 * locals.var_phic_star) + (locals.var_gam2 * locals.var_phic_star_dn6))), (((((locals.var_vgfb1_dn7 - locals.var_sp_s_z0_dn7) * assign31130_e46597) + (assign31130_e46594 * (locals.var_vgfb1_dn7 - locals.var_sp_s_z0_dn7))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn7 - locals.var_sp_s_z0_dn7) + locals.var_phic_star_dn7)) * assign31130_e46610) + (assign31130_e46605 * ((locals.var_vgfbb_dn7 - locals.var_sp_s_z0_dn7) + locals.var_phic_star_dn7)))) - ((locals.var_gam2_dn7 * locals.var_phic_star) + (locals.var_gam2 * locals.var_phic_star_dn7))), (((((locals.var_vgfb1_dn8 - locals.var_sp_s_z0_dn8) * assign31130_e46597) + (assign31130_e46594 * (locals.var_vgfb1_dn8 - locals.var_sp_s_z0_dn8))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn8 - locals.var_sp_s_z0_dn8) + locals.var_phic_star_dn8)) * assign31130_e46610) + (assign31130_e46605 * ((locals.var_vgfbb_dn8 - locals.var_sp_s_z0_dn8) + locals.var_phic_star_dn8)))) - ((locals.var_gam2_dn8 * locals.var_phic_star) + (locals.var_gam2 * locals.var_phic_star_dn8))), (((((locals.var_vgfb1_dn9 - locals.var_sp_s_z0_dn9) * assign31130_e46597) + (assign31130_e46594 * (locals.var_vgfb1_dn9 - locals.var_sp_s_z0_dn9))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn9 - locals.var_sp_s_z0_dn9) + locals.var_phic_star_dn9)) * assign31130_e46610) + (assign31130_e46605 * ((locals.var_vgfbb_dn9 - locals.var_sp_s_z0_dn9) + locals.var_phic_star_dn9)))) - ((locals.var_gam2_dn9 * locals.var_phic_star) + (locals.var_gam2 * locals.var_phic_star_dn9))), (((((locals.var_vgfb1_dn10 - locals.var_sp_s_z0_dn10) * assign31130_e46597) + (assign31130_e46594 * (locals.var_vgfb1_dn10 - locals.var_sp_s_z0_dn10))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn10 - locals.var_sp_s_z0_dn10) + locals.var_phic_star_dn10)) * assign31130_e46610) + (assign31130_e46605 * ((locals.var_vgfbb_dn10 - locals.var_sp_s_z0_dn10) + locals.var_phic_star_dn10)))) - ((locals.var_gam2_dn10 * locals.var_phic_star) + (locals.var_gam2 * locals.var_phic_star_dn10))), (((((locals.var_vgfb1_dn11 - locals.var_sp_s_z0_dn11) * assign31130_e46597) + (assign31130_e46594 * (locals.var_vgfb1_dn11 - locals.var_sp_s_z0_dn11))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn11 - locals.var_sp_s_z0_dn11) + locals.var_phic_star_dn11)) * assign31130_e46610) + (assign31130_e46605 * ((locals.var_vgfbb_dn11 - locals.var_sp_s_z0_dn11) + locals.var_phic_star_dn11)))) - ((locals.var_gam2_dn11 * locals.var_phic_star) + (locals.var_gam2 * locals.var_phic_star_dn11))),)
    } else {
        (locals.var_a_fd, locals.var_a_fd_dn3, locals.var_a_fd_dn4, locals.var_a_fd_dn5, locals.var_a_fd_dn6, locals.var_a_fd_dn7, locals.var_a_fd_dn8, locals.var_a_fd_dn9, locals.var_a_fd_dn10, locals.var_a_fd_dn11,)
    }
};
        locals.var_a_fd = assign31130_e46618;
        locals.var_a_fd_dn3 = assign31130_e46618_d_n3;
        locals.var_a_fd_dn4 = assign31130_e46618_d_n4;
        locals.var_a_fd_dn5 = assign31130_e46618_d_n5;
        locals.var_a_fd_dn6 = assign31130_e46618_d_n6;
        locals.var_a_fd_dn7 = assign31130_e46618_d_n7;
        locals.var_a_fd_dn8 = assign31130_e46618_d_n8;
        locals.var_a_fd_dn9 = assign31130_e46618_d_n9;
        locals.var_a_fd_dn10 = assign31130_e46618_d_n10;
        locals.var_a_fd_dn11 = assign31130_e46618_d_n11;
        locals.var_a_fd_rv = 0.0;

        let (assign31140_e46643, assign31140_e46643_d_n3, assign31140_e46643_d_n4, assign31140_e46643_d_n5, assign31140_e46643_d_n6, assign31140_e46643_d_n7, assign31140_e46643_d_n8, assign31140_e46643_d_n9, assign31140_e46643_d_n10, assign31140_e46643_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31140_e46630: f64 = (locals.var_vgfb1 - locals.var_sp_s_z0);
        let assign31140_e46631: f64 = (2.0 * assign31140_e46630);
        let assign31140_e46634: f64 = (2.0 * locals.var_zeta2);
        let assign31140_e46637: f64 = (locals.var_vgfbb - locals.var_sp_s_z0);
        let assign31140_e46639: f64 = (assign31140_e46637 + locals.var_phic_star);
        let assign31140_e46640: f64 = (assign31140_e46634 * assign31140_e46639);
        let assign31140_e46641: f64 = (assign31140_e46631 - assign31140_e46640);
        (assign31140_e46641, ((2.0 * (locals.var_vgfb1_dn3 - locals.var_sp_s_z0_dn3)) - (assign31140_e46634 * ((locals.var_vgfbb_dn3 - locals.var_sp_s_z0_dn3) + locals.var_phic_star_dn3))), ((2.0 * (locals.var_vgfb1_dn4 - locals.var_sp_s_z0_dn4)) - (assign31140_e46634 * ((locals.var_vgfbb_dn4 - locals.var_sp_s_z0_dn4) + locals.var_phic_star_dn4))), ((2.0 * (locals.var_vgfb1_dn5 - locals.var_sp_s_z0_dn5)) - (assign31140_e46634 * ((locals.var_vgfbb_dn5 - locals.var_sp_s_z0_dn5) + locals.var_phic_star_dn5))), ((2.0 * (locals.var_vgfb1_dn6 - locals.var_sp_s_z0_dn6)) - (assign31140_e46634 * ((locals.var_vgfbb_dn6 - locals.var_sp_s_z0_dn6) + locals.var_phic_star_dn6))), ((2.0 * (locals.var_vgfb1_dn7 - locals.var_sp_s_z0_dn7)) - (assign31140_e46634 * ((locals.var_vgfbb_dn7 - locals.var_sp_s_z0_dn7) + locals.var_phic_star_dn7))), ((2.0 * (locals.var_vgfb1_dn8 - locals.var_sp_s_z0_dn8)) - (assign31140_e46634 * ((locals.var_vgfbb_dn8 - locals.var_sp_s_z0_dn8) + locals.var_phic_star_dn8))), ((2.0 * (locals.var_vgfb1_dn9 - locals.var_sp_s_z0_dn9)) - (assign31140_e46634 * ((locals.var_vgfbb_dn9 - locals.var_sp_s_z0_dn9) + locals.var_phic_star_dn9))), ((2.0 * (locals.var_vgfb1_dn10 - locals.var_sp_s_z0_dn10)) - (assign31140_e46634 * ((locals.var_vgfbb_dn10 - locals.var_sp_s_z0_dn10) + locals.var_phic_star_dn10))), ((2.0 * (locals.var_vgfb1_dn11 - locals.var_sp_s_z0_dn11)) - (assign31140_e46634 * ((locals.var_vgfbb_dn11 - locals.var_sp_s_z0_dn11) + locals.var_phic_star_dn11))),)
    } else {
        (locals.var_c_fd, locals.var_c_fd_dn3, locals.var_c_fd_dn4, locals.var_c_fd_dn5, locals.var_c_fd_dn6, locals.var_c_fd_dn7, locals.var_c_fd_dn8, locals.var_c_fd_dn9, locals.var_c_fd_dn10, locals.var_c_fd_dn11,)
    }
};
        locals.var_c_fd = assign31140_e46643;
        locals.var_c_fd_dn3 = assign31140_e46643_d_n3;
        locals.var_c_fd_dn4 = assign31140_e46643_d_n4;
        locals.var_c_fd_dn5 = assign31140_e46643_d_n5;
        locals.var_c_fd_dn6 = assign31140_e46643_d_n6;
        locals.var_c_fd_dn7 = assign31140_e46643_d_n7;
        locals.var_c_fd_dn8 = assign31140_e46643_d_n8;
        locals.var_c_fd_dn9 = assign31140_e46643_d_n9;
        locals.var_c_fd_dn10 = assign31140_e46643_d_n10;
        locals.var_c_fd_dn11 = assign31140_e46643_d_n11;
        locals.var_c_fd_rv = 0.0;

        let (assign31150_e46656, assign31150_e46656_d_n3, assign31150_e46656_d_n4, assign31150_e46656_d_n5, assign31150_e46656_d_n6, assign31150_e46656_d_n7, assign31150_e46656_d_n8, assign31150_e46656_d_n9, assign31150_e46656_d_n10, assign31150_e46656_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31150_e46654: f64 = (locals.var_c_fd * locals.var_c_fd);
        (assign31150_e46654, ((locals.var_c_fd_dn3 * locals.var_c_fd) + (locals.var_c_fd * locals.var_c_fd_dn3)), ((locals.var_c_fd_dn4 * locals.var_c_fd) + (locals.var_c_fd * locals.var_c_fd_dn4)), ((locals.var_c_fd_dn5 * locals.var_c_fd) + (locals.var_c_fd * locals.var_c_fd_dn5)), ((locals.var_c_fd_dn6 * locals.var_c_fd) + (locals.var_c_fd * locals.var_c_fd_dn6)), ((locals.var_c_fd_dn7 * locals.var_c_fd) + (locals.var_c_fd * locals.var_c_fd_dn7)), ((locals.var_c_fd_dn8 * locals.var_c_fd) + (locals.var_c_fd * locals.var_c_fd_dn8)), ((locals.var_c_fd_dn9 * locals.var_c_fd) + (locals.var_c_fd * locals.var_c_fd_dn9)), ((locals.var_c_fd_dn10 * locals.var_c_fd) + (locals.var_c_fd * locals.var_c_fd_dn10)), ((locals.var_c_fd_dn11 * locals.var_c_fd) + (locals.var_c_fd * locals.var_c_fd_dn11)),)
    } else {
        (locals.var_c_fd2, locals.var_c_fd2_dn3, locals.var_c_fd2_dn4, locals.var_c_fd2_dn5, locals.var_c_fd2_dn6, locals.var_c_fd2_dn7, locals.var_c_fd2_dn8, locals.var_c_fd2_dn9, locals.var_c_fd2_dn10, locals.var_c_fd2_dn11,)
    }
};
        locals.var_c_fd2 = assign31150_e46656;
        locals.var_c_fd2_dn3 = assign31150_e46656_d_n3;
        locals.var_c_fd2_dn4 = assign31150_e46656_d_n4;
        locals.var_c_fd2_dn5 = assign31150_e46656_d_n5;
        locals.var_c_fd2_dn6 = assign31150_e46656_d_n6;
        locals.var_c_fd2_dn7 = assign31150_e46656_d_n7;
        locals.var_c_fd2_dn8 = assign31150_e46656_d_n8;
        locals.var_c_fd2_dn9 = assign31150_e46656_d_n9;
        locals.var_c_fd2_dn10 = assign31150_e46656_d_n10;
        locals.var_c_fd2_dn11 = assign31150_e46656_d_n11;
        locals.var_c_fd2_rv = 0.0;

        let (assign31160_e46669,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31160_e46667: f64 = (1.0 - locals.var_zeta2);
        (assign31160_e46667,)
    } else {
        (locals.var_d_fd,)
    }
};
        locals.var_d_fd = assign31160_e46669;
        locals.var_d_fd_rv = 0.0;

        let assign31170_e46672: f64 = if locals.var_a_fd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard621 = assign31170_e46672;
        locals.var_guard621_rv = 0.0;

        let (assign31180_e46685, assign31180_e46685_d_n3, assign31180_e46685_d_n4, assign31180_e46685_d_n5, assign31180_e46685_d_n6, assign31180_e46685_d_n7, assign31180_e46685_d_n8, assign31180_e46685_d_n9, assign31180_e46685_d_n10, assign31180_e46685_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) && (locals.var_guard621 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_a_fd, locals.var_a_fd_dn3, locals.var_a_fd_dn4, locals.var_a_fd_dn5, locals.var_a_fd_dn6, locals.var_a_fd_dn7, locals.var_a_fd_dn8, locals.var_a_fd_dn9, locals.var_a_fd_dn10, locals.var_a_fd_dn11,)
    }
};
        locals.var_a_fd = assign31180_e46685;
        locals.var_a_fd_dn3 = assign31180_e46685_d_n3;
        locals.var_a_fd_dn4 = assign31180_e46685_d_n4;
        locals.var_a_fd_dn5 = assign31180_e46685_d_n5;
        locals.var_a_fd_dn6 = assign31180_e46685_d_n6;
        locals.var_a_fd_dn7 = assign31180_e46685_d_n7;
        locals.var_a_fd_dn8 = assign31180_e46685_d_n8;
        locals.var_a_fd_dn9 = assign31180_e46685_d_n9;
        locals.var_a_fd_dn10 = assign31180_e46685_d_n10;
        locals.var_a_fd_dn11 = assign31180_e46685_d_n11;
        locals.var_a_fd_rv = 0.0;

        let (assign31190_e46705, assign31190_e46705_d_n3, assign31190_e46705_d_n4, assign31190_e46705_d_n5, assign31190_e46705_d_n6, assign31190_e46705_d_n7, assign31190_e46705_d_n8, assign31190_e46705_d_n9, assign31190_e46705_d_n10, assign31190_e46705_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31190_e46696: f64 = (locals.var_phisf - locals.var_sp_s_z0);
        let assign31190_e46699: f64 = (locals.var_a_fd * locals.var_inv_gam2);
        let assign31190_e46701: f64 = (assign31190_e46699).max(1e-38);
        let assign31190_e46702: f64 = (assign31190_e46701).ln();
        let assign31190_e46703: f64 = (assign31190_e46696 + assign31190_e46702);
        (assign31190_e46703, ((locals.var_phisf_dn3 - locals.var_sp_s_z0_dn3) + (if assign31190_e46699 >= 1e-38 { ((locals.var_a_fd_dn3 * locals.var_inv_gam2) + (locals.var_a_fd * locals.var_inv_gam2_dn3)) } else { 0.0 } / assign31190_e46701)), ((locals.var_phisf_dn4 - locals.var_sp_s_z0_dn4) + (if assign31190_e46699 >= 1e-38 { ((locals.var_a_fd_dn4 * locals.var_inv_gam2) + (locals.var_a_fd * locals.var_inv_gam2_dn4)) } else { 0.0 } / assign31190_e46701)), ((locals.var_phisf_dn5 - locals.var_sp_s_z0_dn5) + (if assign31190_e46699 >= 1e-38 { ((locals.var_a_fd_dn5 * locals.var_inv_gam2) + (locals.var_a_fd * locals.var_inv_gam2_dn5)) } else { 0.0 } / assign31190_e46701)), ((locals.var_phisf_dn6 - locals.var_sp_s_z0_dn6) + (if assign31190_e46699 >= 1e-38 { ((locals.var_a_fd_dn6 * locals.var_inv_gam2) + (locals.var_a_fd * locals.var_inv_gam2_dn6)) } else { 0.0 } / assign31190_e46701)), ((locals.var_phisf_dn7 - locals.var_sp_s_z0_dn7) + (if assign31190_e46699 >= 1e-38 { ((locals.var_a_fd_dn7 * locals.var_inv_gam2) + (locals.var_a_fd * locals.var_inv_gam2_dn7)) } else { 0.0 } / assign31190_e46701)), ((locals.var_phisf_dn8 - locals.var_sp_s_z0_dn8) + (if assign31190_e46699 >= 1e-38 { ((locals.var_a_fd_dn8 * locals.var_inv_gam2) + (locals.var_a_fd * locals.var_inv_gam2_dn8)) } else { 0.0 } / assign31190_e46701)), ((locals.var_phisf_dn9 - locals.var_sp_s_z0_dn9) + (if assign31190_e46699 >= 1e-38 { ((locals.var_a_fd_dn9 * locals.var_inv_gam2) + (locals.var_a_fd * locals.var_inv_gam2_dn9)) } else { 0.0 } / assign31190_e46701)), ((locals.var_phisf_dn10 - locals.var_sp_s_z0_dn10) + (if assign31190_e46699 >= 1e-38 { ((locals.var_a_fd_dn10 * locals.var_inv_gam2) + (locals.var_a_fd * locals.var_inv_gam2_dn10)) } else { 0.0 } / assign31190_e46701)), ((locals.var_phisf_dn11 - locals.var_sp_s_z0_dn11) + (if assign31190_e46699 >= 1e-38 { ((locals.var_a_fd_dn11 * locals.var_inv_gam2) + (locals.var_a_fd * locals.var_inv_gam2_dn11)) } else { 0.0 } / assign31190_e46701)),)
    } else {
        (locals.var_tau_fd, locals.var_tau_fd_dn3, locals.var_tau_fd_dn4, locals.var_tau_fd_dn5, locals.var_tau_fd_dn6, locals.var_tau_fd_dn7, locals.var_tau_fd_dn8, locals.var_tau_fd_dn9, locals.var_tau_fd_dn10, locals.var_tau_fd_dn11,)
    }
};
        locals.var_tau_fd = assign31190_e46705;
        locals.var_tau_fd_dn3 = assign31190_e46705_d_n3;
        locals.var_tau_fd_dn4 = assign31190_e46705_d_n4;
        locals.var_tau_fd_dn5 = assign31190_e46705_d_n5;
        locals.var_tau_fd_dn6 = assign31190_e46705_d_n6;
        locals.var_tau_fd_dn7 = assign31190_e46705_d_n7;
        locals.var_tau_fd_dn8 = assign31190_e46705_d_n8;
        locals.var_tau_fd_dn9 = assign31190_e46705_d_n9;
        locals.var_tau_fd_dn10 = assign31190_e46705_d_n10;
        locals.var_tau_fd_dn11 = assign31190_e46705_d_n11;
        locals.var_tau_fd_rv = 0.0;

        let (assign31200_e46718, assign31200_e46718_d_n3, assign31200_e46718_d_n4, assign31200_e46718_d_n5, assign31200_e46718_d_n6, assign31200_e46718_d_n7, assign31200_e46718_d_n8, assign31200_e46718_d_n9, assign31200_e46718_d_n10, assign31200_e46718_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31200_e46716: f64 = (locals.var_a_fd + locals.var_c_fd);
        (assign31200_e46716, (locals.var_a_fd_dn3 + locals.var_c_fd_dn3), (locals.var_a_fd_dn4 + locals.var_c_fd_dn4), (locals.var_a_fd_dn5 + locals.var_c_fd_dn5), (locals.var_a_fd_dn6 + locals.var_c_fd_dn6), (locals.var_a_fd_dn7 + locals.var_c_fd_dn7), (locals.var_a_fd_dn8 + locals.var_c_fd_dn8), (locals.var_a_fd_dn9 + locals.var_c_fd_dn9), (locals.var_a_fd_dn10 + locals.var_c_fd_dn10), (locals.var_a_fd_dn11 + locals.var_c_fd_dn11),)
    } else {
        (locals.var_v_fd, locals.var_v_fd_dn3, locals.var_v_fd_dn4, locals.var_v_fd_dn5, locals.var_v_fd_dn6, locals.var_v_fd_dn7, locals.var_v_fd_dn8, locals.var_v_fd_dn9, locals.var_v_fd_dn10, locals.var_v_fd_dn11,)
    }
};
        locals.var_v_fd = assign31200_e46718;
        locals.var_v_fd_dn3 = assign31200_e46718_d_n3;
        locals.var_v_fd_dn4 = assign31200_e46718_d_n4;
        locals.var_v_fd_dn5 = assign31200_e46718_d_n5;
        locals.var_v_fd_dn6 = assign31200_e46718_d_n6;
        locals.var_v_fd_dn7 = assign31200_e46718_d_n7;
        locals.var_v_fd_dn8 = assign31200_e46718_d_n8;
        locals.var_v_fd_dn9 = assign31200_e46718_d_n9;
        locals.var_v_fd_dn10 = assign31200_e46718_d_n10;
        locals.var_v_fd_dn11 = assign31200_e46718_d_n11;
        locals.var_v_fd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_92(
        locals: &mut StampLocals,
    ) {
        let (assign31210_e46731, assign31210_e46731_d_n3, assign31210_e46731_d_n4, assign31210_e46731_d_n5, assign31210_e46731_d_n6, assign31210_e46731_d_n7, assign31210_e46731_d_n8, assign31210_e46731_d_n9, assign31210_e46731_d_n10, assign31210_e46731_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31210_e46729: f64 = (locals.var_v_fd * locals.var_v_fd);
        (assign31210_e46729, ((locals.var_v_fd_dn3 * locals.var_v_fd) + (locals.var_v_fd * locals.var_v_fd_dn3)), ((locals.var_v_fd_dn4 * locals.var_v_fd) + (locals.var_v_fd * locals.var_v_fd_dn4)), ((locals.var_v_fd_dn5 * locals.var_v_fd) + (locals.var_v_fd * locals.var_v_fd_dn5)), ((locals.var_v_fd_dn6 * locals.var_v_fd) + (locals.var_v_fd * locals.var_v_fd_dn6)), ((locals.var_v_fd_dn7 * locals.var_v_fd) + (locals.var_v_fd * locals.var_v_fd_dn7)), ((locals.var_v_fd_dn8 * locals.var_v_fd) + (locals.var_v_fd * locals.var_v_fd_dn8)), ((locals.var_v_fd_dn9 * locals.var_v_fd) + (locals.var_v_fd * locals.var_v_fd_dn9)), ((locals.var_v_fd_dn10 * locals.var_v_fd) + (locals.var_v_fd * locals.var_v_fd_dn10)), ((locals.var_v_fd_dn11 * locals.var_v_fd) + (locals.var_v_fd * locals.var_v_fd_dn11)),)
    } else {
        (locals.var_v_fd2, locals.var_v_fd2_dn3, locals.var_v_fd2_dn4, locals.var_v_fd2_dn5, locals.var_v_fd2_dn6, locals.var_v_fd2_dn7, locals.var_v_fd2_dn8, locals.var_v_fd2_dn9, locals.var_v_fd2_dn10, locals.var_v_fd2_dn11,)
    }
};
        locals.var_v_fd2 = assign31210_e46731;
        locals.var_v_fd2_dn3 = assign31210_e46731_d_n3;
        locals.var_v_fd2_dn4 = assign31210_e46731_d_n4;
        locals.var_v_fd2_dn5 = assign31210_e46731_d_n5;
        locals.var_v_fd2_dn6 = assign31210_e46731_d_n6;
        locals.var_v_fd2_dn7 = assign31210_e46731_d_n7;
        locals.var_v_fd2_dn8 = assign31210_e46731_d_n8;
        locals.var_v_fd2_dn9 = assign31210_e46731_d_n9;
        locals.var_v_fd2_dn10 = assign31210_e46731_d_n10;
        locals.var_v_fd2_dn11 = assign31210_e46731_d_n11;
        locals.var_v_fd2_rv = 0.0;

        let (assign31220_e46752, assign31220_e46752_d_n3, assign31220_e46752_d_n4, assign31220_e46752_d_n5, assign31220_e46752_d_n6, assign31220_e46752_d_n7, assign31220_e46752_d_n8, assign31220_e46752_d_n9, assign31220_e46752_d_n10, assign31220_e46752_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31220_e46742: f64 = (locals.var_v_fd2 / locals.var_tau_fd);
        let assign31220_e46745: f64 = (0.5 * locals.var_c_fd2);
        let assign31220_e46746: f64 = (assign31220_e46742 + assign31220_e46745);
        let assign31220_e46749: f64 = (locals.var_a_fd * locals.var_d_fd);
        let assign31220_e46750: f64 = (assign31220_e46746 - assign31220_e46749);
        (assign31220_e46750, (((((locals.var_v_fd2_dn3 * locals.var_tau_fd) - (locals.var_v_fd2 * locals.var_tau_fd_dn3)) / (locals.var_tau_fd * locals.var_tau_fd)) + (0.5 * locals.var_c_fd2_dn3)) - (locals.var_a_fd_dn3 * locals.var_d_fd)), (((((locals.var_v_fd2_dn4 * locals.var_tau_fd) - (locals.var_v_fd2 * locals.var_tau_fd_dn4)) / (locals.var_tau_fd * locals.var_tau_fd)) + (0.5 * locals.var_c_fd2_dn4)) - (locals.var_a_fd_dn4 * locals.var_d_fd)), (((((locals.var_v_fd2_dn5 * locals.var_tau_fd) - (locals.var_v_fd2 * locals.var_tau_fd_dn5)) / (locals.var_tau_fd * locals.var_tau_fd)) + (0.5 * locals.var_c_fd2_dn5)) - (locals.var_a_fd_dn5 * locals.var_d_fd)), (((((locals.var_v_fd2_dn6 * locals.var_tau_fd) - (locals.var_v_fd2 * locals.var_tau_fd_dn6)) / (locals.var_tau_fd * locals.var_tau_fd)) + (0.5 * locals.var_c_fd2_dn6)) - (locals.var_a_fd_dn6 * locals.var_d_fd)), (((((locals.var_v_fd2_dn7 * locals.var_tau_fd) - (locals.var_v_fd2 * locals.var_tau_fd_dn7)) / (locals.var_tau_fd * locals.var_tau_fd)) + (0.5 * locals.var_c_fd2_dn7)) - (locals.var_a_fd_dn7 * locals.var_d_fd)), (((((locals.var_v_fd2_dn8 * locals.var_tau_fd) - (locals.var_v_fd2 * locals.var_tau_fd_dn8)) / (locals.var_tau_fd * locals.var_tau_fd)) + (0.5 * locals.var_c_fd2_dn8)) - (locals.var_a_fd_dn8 * locals.var_d_fd)), (((((locals.var_v_fd2_dn9 * locals.var_tau_fd) - (locals.var_v_fd2 * locals.var_tau_fd_dn9)) / (locals.var_tau_fd * locals.var_tau_fd)) + (0.5 * locals.var_c_fd2_dn9)) - (locals.var_a_fd_dn9 * locals.var_d_fd)), (((((locals.var_v_fd2_dn10 * locals.var_tau_fd) - (locals.var_v_fd2 * locals.var_tau_fd_dn10)) / (locals.var_tau_fd * locals.var_tau_fd)) + (0.5 * locals.var_c_fd2_dn10)) - (locals.var_a_fd_dn10 * locals.var_d_fd)), (((((locals.var_v_fd2_dn11 * locals.var_tau_fd) - (locals.var_v_fd2 * locals.var_tau_fd_dn11)) / (locals.var_tau_fd * locals.var_tau_fd)) + (0.5 * locals.var_c_fd2_dn11)) - (locals.var_a_fd_dn11 * locals.var_d_fd)),)
    } else {
        (locals.var_mu_fd, locals.var_mu_fd_dn3, locals.var_mu_fd_dn4, locals.var_mu_fd_dn5, locals.var_mu_fd_dn6, locals.var_mu_fd_dn7, locals.var_mu_fd_dn8, locals.var_mu_fd_dn9, locals.var_mu_fd_dn10, locals.var_mu_fd_dn11,)
    }
};
        locals.var_mu_fd = assign31220_e46752;
        locals.var_mu_fd_dn3 = assign31220_e46752_d_n3;
        locals.var_mu_fd_dn4 = assign31220_e46752_d_n4;
        locals.var_mu_fd_dn5 = assign31220_e46752_d_n5;
        locals.var_mu_fd_dn6 = assign31220_e46752_d_n6;
        locals.var_mu_fd_dn7 = assign31220_e46752_d_n7;
        locals.var_mu_fd_dn8 = assign31220_e46752_d_n8;
        locals.var_mu_fd_dn9 = assign31220_e46752_d_n9;
        locals.var_mu_fd_dn10 = assign31220_e46752_d_n10;
        locals.var_mu_fd_dn11 = assign31220_e46752_d_n11;
        locals.var_mu_fd_rv = 0.0;

        let (assign31230_e46767, assign31230_e46767_d_n3, assign31230_e46767_d_n4, assign31230_e46767_d_n5, assign31230_e46767_d_n6, assign31230_e46767_d_n7, assign31230_e46767_d_n8, assign31230_e46767_d_n9, assign31230_e46767_d_n10, assign31230_e46767_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31230_e46763: f64 = (locals.var_c_fd * locals.var_v_fd);
        let assign31230_e46765: f64 = (assign31230_e46763 / locals.var_mu_fd);
        (assign31230_e46765, (((((locals.var_c_fd_dn3 * locals.var_v_fd) + (locals.var_c_fd * locals.var_v_fd_dn3)) * locals.var_mu_fd) - (assign31230_e46763 * locals.var_mu_fd_dn3)) / (locals.var_mu_fd * locals.var_mu_fd)), (((((locals.var_c_fd_dn4 * locals.var_v_fd) + (locals.var_c_fd * locals.var_v_fd_dn4)) * locals.var_mu_fd) - (assign31230_e46763 * locals.var_mu_fd_dn4)) / (locals.var_mu_fd * locals.var_mu_fd)), (((((locals.var_c_fd_dn5 * locals.var_v_fd) + (locals.var_c_fd * locals.var_v_fd_dn5)) * locals.var_mu_fd) - (assign31230_e46763 * locals.var_mu_fd_dn5)) / (locals.var_mu_fd * locals.var_mu_fd)), (((((locals.var_c_fd_dn6 * locals.var_v_fd) + (locals.var_c_fd * locals.var_v_fd_dn6)) * locals.var_mu_fd) - (assign31230_e46763 * locals.var_mu_fd_dn6)) / (locals.var_mu_fd * locals.var_mu_fd)), (((((locals.var_c_fd_dn7 * locals.var_v_fd) + (locals.var_c_fd * locals.var_v_fd_dn7)) * locals.var_mu_fd) - (assign31230_e46763 * locals.var_mu_fd_dn7)) / (locals.var_mu_fd * locals.var_mu_fd)), (((((locals.var_c_fd_dn8 * locals.var_v_fd) + (locals.var_c_fd * locals.var_v_fd_dn8)) * locals.var_mu_fd) - (assign31230_e46763 * locals.var_mu_fd_dn8)) / (locals.var_mu_fd * locals.var_mu_fd)), (((((locals.var_c_fd_dn9 * locals.var_v_fd) + (locals.var_c_fd * locals.var_v_fd_dn9)) * locals.var_mu_fd) - (assign31230_e46763 * locals.var_mu_fd_dn9)) / (locals.var_mu_fd * locals.var_mu_fd)), (((((locals.var_c_fd_dn10 * locals.var_v_fd) + (locals.var_c_fd * locals.var_v_fd_dn10)) * locals.var_mu_fd) - (assign31230_e46763 * locals.var_mu_fd_dn10)) / (locals.var_mu_fd * locals.var_mu_fd)), (((((locals.var_c_fd_dn11 * locals.var_v_fd) + (locals.var_c_fd * locals.var_v_fd_dn11)) * locals.var_mu_fd) - (assign31230_e46763 * locals.var_mu_fd_dn11)) / (locals.var_mu_fd * locals.var_mu_fd)),)
    } else {
        (locals.var_temp1_fd, locals.var_temp1_fd_dn3, locals.var_temp1_fd_dn4, locals.var_temp1_fd_dn5, locals.var_temp1_fd_dn6, locals.var_temp1_fd_dn7, locals.var_temp1_fd_dn8, locals.var_temp1_fd_dn9, locals.var_temp1_fd_dn10, locals.var_temp1_fd_dn11,)
    }
};
        locals.var_temp1_fd = assign31230_e46767;
        locals.var_temp1_fd_dn3 = assign31230_e46767_d_n3;
        locals.var_temp1_fd_dn4 = assign31230_e46767_d_n4;
        locals.var_temp1_fd_dn5 = assign31230_e46767_d_n5;
        locals.var_temp1_fd_dn6 = assign31230_e46767_d_n6;
        locals.var_temp1_fd_dn7 = assign31230_e46767_d_n7;
        locals.var_temp1_fd_dn8 = assign31230_e46767_d_n8;
        locals.var_temp1_fd_dn9 = assign31230_e46767_d_n9;
        locals.var_temp1_fd_dn10 = assign31230_e46767_d_n10;
        locals.var_temp1_fd_dn11 = assign31230_e46767_d_n11;
        locals.var_temp1_fd_rv = 0.0;

        let (assign31240_e46784, assign31240_e46784_d_n3, assign31240_e46784_d_n4, assign31240_e46784_d_n5, assign31240_e46784_d_n6, assign31240_e46784_d_n7, assign31240_e46784_d_n8, assign31240_e46784_d_n9, assign31240_e46784_d_n10, assign31240_e46784_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31240_e46778: f64 = (0.3333333333333333 * locals.var_c_fd2);
        let assign31240_e46781: f64 = (locals.var_a_fd * locals.var_d_fd);
        let assign31240_e46782: f64 = (assign31240_e46778 - assign31240_e46781);
        (assign31240_e46782, ((0.3333333333333333 * locals.var_c_fd2_dn3) - (locals.var_a_fd_dn3 * locals.var_d_fd)), ((0.3333333333333333 * locals.var_c_fd2_dn4) - (locals.var_a_fd_dn4 * locals.var_d_fd)), ((0.3333333333333333 * locals.var_c_fd2_dn5) - (locals.var_a_fd_dn5 * locals.var_d_fd)), ((0.3333333333333333 * locals.var_c_fd2_dn6) - (locals.var_a_fd_dn6 * locals.var_d_fd)), ((0.3333333333333333 * locals.var_c_fd2_dn7) - (locals.var_a_fd_dn7 * locals.var_d_fd)), ((0.3333333333333333 * locals.var_c_fd2_dn8) - (locals.var_a_fd_dn8 * locals.var_d_fd)), ((0.3333333333333333 * locals.var_c_fd2_dn9) - (locals.var_a_fd_dn9 * locals.var_d_fd)), ((0.3333333333333333 * locals.var_c_fd2_dn10) - (locals.var_a_fd_dn10 * locals.var_d_fd)), ((0.3333333333333333 * locals.var_c_fd2_dn11) - (locals.var_a_fd_dn11 * locals.var_d_fd)),)
    } else {
        (locals.var_temp2_fd, locals.var_temp2_fd_dn3, locals.var_temp2_fd_dn4, locals.var_temp2_fd_dn5, locals.var_temp2_fd_dn6, locals.var_temp2_fd_dn7, locals.var_temp2_fd_dn8, locals.var_temp2_fd_dn9, locals.var_temp2_fd_dn10, locals.var_temp2_fd_dn11,)
    }
};
        locals.var_temp2_fd = assign31240_e46784;
        locals.var_temp2_fd_dn3 = assign31240_e46784_d_n3;
        locals.var_temp2_fd_dn4 = assign31240_e46784_d_n4;
        locals.var_temp2_fd_dn5 = assign31240_e46784_d_n5;
        locals.var_temp2_fd_dn6 = assign31240_e46784_d_n6;
        locals.var_temp2_fd_dn7 = assign31240_e46784_d_n7;
        locals.var_temp2_fd_dn8 = assign31240_e46784_d_n8;
        locals.var_temp2_fd_dn9 = assign31240_e46784_d_n9;
        locals.var_temp2_fd_dn10 = assign31240_e46784_d_n10;
        locals.var_temp2_fd_dn11 = assign31240_e46784_d_n11;
        locals.var_temp2_fd_rv = 0.0;

        let (assign31250_e46803, assign31250_e46803_d_n3, assign31250_e46803_d_n4, assign31250_e46803_d_n5, assign31250_e46803_d_n6, assign31250_e46803_d_n7, assign31250_e46803_d_n8, assign31250_e46803_d_n9, assign31250_e46803_d_n10, assign31250_e46803_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31250_e46795: f64 = (locals.var_v_fd * locals.var_a_fd);
        let assign31250_e46799: f64 = (locals.var_temp1_fd * locals.var_temp2_fd);
        let assign31250_e46800: f64 = (locals.var_mu_fd + assign31250_e46799);
        let assign31250_e46801: f64 = (assign31250_e46795 / assign31250_e46800);
        (assign31250_e46801, (((((locals.var_v_fd_dn3 * locals.var_a_fd) + (locals.var_v_fd * locals.var_a_fd_dn3)) * assign31250_e46800) - (assign31250_e46795 * (locals.var_mu_fd_dn3 + ((locals.var_temp1_fd_dn3 * locals.var_temp2_fd) + (locals.var_temp1_fd * locals.var_temp2_fd_dn3))))) / (assign31250_e46800 * assign31250_e46800)), (((((locals.var_v_fd_dn4 * locals.var_a_fd) + (locals.var_v_fd * locals.var_a_fd_dn4)) * assign31250_e46800) - (assign31250_e46795 * (locals.var_mu_fd_dn4 + ((locals.var_temp1_fd_dn4 * locals.var_temp2_fd) + (locals.var_temp1_fd * locals.var_temp2_fd_dn4))))) / (assign31250_e46800 * assign31250_e46800)), (((((locals.var_v_fd_dn5 * locals.var_a_fd) + (locals.var_v_fd * locals.var_a_fd_dn5)) * assign31250_e46800) - (assign31250_e46795 * (locals.var_mu_fd_dn5 + ((locals.var_temp1_fd_dn5 * locals.var_temp2_fd) + (locals.var_temp1_fd * locals.var_temp2_fd_dn5))))) / (assign31250_e46800 * assign31250_e46800)), (((((locals.var_v_fd_dn6 * locals.var_a_fd) + (locals.var_v_fd * locals.var_a_fd_dn6)) * assign31250_e46800) - (assign31250_e46795 * (locals.var_mu_fd_dn6 + ((locals.var_temp1_fd_dn6 * locals.var_temp2_fd) + (locals.var_temp1_fd * locals.var_temp2_fd_dn6))))) / (assign31250_e46800 * assign31250_e46800)), (((((locals.var_v_fd_dn7 * locals.var_a_fd) + (locals.var_v_fd * locals.var_a_fd_dn7)) * assign31250_e46800) - (assign31250_e46795 * (locals.var_mu_fd_dn7 + ((locals.var_temp1_fd_dn7 * locals.var_temp2_fd) + (locals.var_temp1_fd * locals.var_temp2_fd_dn7))))) / (assign31250_e46800 * assign31250_e46800)), (((((locals.var_v_fd_dn8 * locals.var_a_fd) + (locals.var_v_fd * locals.var_a_fd_dn8)) * assign31250_e46800) - (assign31250_e46795 * (locals.var_mu_fd_dn8 + ((locals.var_temp1_fd_dn8 * locals.var_temp2_fd) + (locals.var_temp1_fd * locals.var_temp2_fd_dn8))))) / (assign31250_e46800 * assign31250_e46800)), (((((locals.var_v_fd_dn9 * locals.var_a_fd) + (locals.var_v_fd * locals.var_a_fd_dn9)) * assign31250_e46800) - (assign31250_e46795 * (locals.var_mu_fd_dn9 + ((locals.var_temp1_fd_dn9 * locals.var_temp2_fd) + (locals.var_temp1_fd * locals.var_temp2_fd_dn9))))) / (assign31250_e46800 * assign31250_e46800)), (((((locals.var_v_fd_dn10 * locals.var_a_fd) + (locals.var_v_fd * locals.var_a_fd_dn10)) * assign31250_e46800) - (assign31250_e46795 * (locals.var_mu_fd_dn10 + ((locals.var_temp1_fd_dn10 * locals.var_temp2_fd) + (locals.var_temp1_fd * locals.var_temp2_fd_dn10))))) / (assign31250_e46800 * assign31250_e46800)), (((((locals.var_v_fd_dn11 * locals.var_a_fd) + (locals.var_v_fd * locals.var_a_fd_dn11)) * assign31250_e46800) - (assign31250_e46795 * (locals.var_mu_fd_dn11 + ((locals.var_temp1_fd_dn11 * locals.var_temp2_fd) + (locals.var_temp1_fd * locals.var_temp2_fd_dn11))))) / (assign31250_e46800 * assign31250_e46800)),)
    } else {
        (locals.var_u_first_r, locals.var_u_first_r_dn3, locals.var_u_first_r_dn4, locals.var_u_first_r_dn5, locals.var_u_first_r_dn6, locals.var_u_first_r_dn7, locals.var_u_first_r_dn8, locals.var_u_first_r_dn9, locals.var_u_first_r_dn10, locals.var_u_first_r_dn11,)
    }
};
        locals.var_u_first_r = assign31250_e46803;
        locals.var_u_first_r_dn3 = assign31250_e46803_d_n3;
        locals.var_u_first_r_dn4 = assign31250_e46803_d_n4;
        locals.var_u_first_r_dn5 = assign31250_e46803_d_n5;
        locals.var_u_first_r_dn6 = assign31250_e46803_d_n6;
        locals.var_u_first_r_dn7 = assign31250_e46803_d_n7;
        locals.var_u_first_r_dn8 = assign31250_e46803_d_n8;
        locals.var_u_first_r_dn9 = assign31250_e46803_d_n9;
        locals.var_u_first_r_dn10 = assign31250_e46803_d_n10;
        locals.var_u_first_r_dn11 = assign31250_e46803_d_n11;
        locals.var_u_first_r_rv = 0.0;

        let (assign31260_e46816, assign31260_e46816_d_n3, assign31260_e46816_d_n4, assign31260_e46816_d_n5, assign31260_e46816_d_n6, assign31260_e46816_d_n7, assign31260_e46816_d_n8, assign31260_e46816_d_n9, assign31260_e46816_d_n10, assign31260_e46816_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31260_e46814: f64 = (locals.var_sp_s_z0 + locals.var_u_first_r);
        (assign31260_e46814, (locals.var_sp_s_z0_dn3 + locals.var_u_first_r_dn3), (locals.var_sp_s_z0_dn4 + locals.var_u_first_r_dn4), (locals.var_sp_s_z0_dn5 + locals.var_u_first_r_dn5), (locals.var_sp_s_z0_dn6 + locals.var_u_first_r_dn6), (locals.var_sp_s_z0_dn7 + locals.var_u_first_r_dn7), (locals.var_sp_s_z0_dn8 + locals.var_u_first_r_dn8), (locals.var_sp_s_z0_dn9 + locals.var_u_first_r_dn9), (locals.var_sp_s_z0_dn10 + locals.var_u_first_r_dn10), (locals.var_sp_s_z0_dn11 + locals.var_u_first_r_dn11),)
    } else {
        (locals.var_u0_first_fd, locals.var_u0_first_fd_dn3, locals.var_u0_first_fd_dn4, locals.var_u0_first_fd_dn5, locals.var_u0_first_fd_dn6, locals.var_u0_first_fd_dn7, locals.var_u0_first_fd_dn8, locals.var_u0_first_fd_dn9, locals.var_u0_first_fd_dn10, locals.var_u0_first_fd_dn11,)
    }
};
        locals.var_u0_first_fd = assign31260_e46816;
        locals.var_u0_first_fd_dn3 = assign31260_e46816_d_n3;
        locals.var_u0_first_fd_dn4 = assign31260_e46816_d_n4;
        locals.var_u0_first_fd_dn5 = assign31260_e46816_d_n5;
        locals.var_u0_first_fd_dn6 = assign31260_e46816_d_n6;
        locals.var_u0_first_fd_dn7 = assign31260_e46816_d_n7;
        locals.var_u0_first_fd_dn8 = assign31260_e46816_d_n8;
        locals.var_u0_first_fd_dn9 = assign31260_e46816_d_n9;
        locals.var_u0_first_fd_dn10 = assign31260_e46816_d_n10;
        locals.var_u0_first_fd_dn11 = assign31260_e46816_d_n11;
        locals.var_u0_first_fd_rv = 0.0;

        let (assign31270_e46830, assign31270_e46830_d_n3, assign31270_e46830_d_n4, assign31270_e46830_d_n5, assign31270_e46830_d_n6, assign31270_e46830_d_n7, assign31270_e46830_d_n8, assign31270_e46830_d_n9, assign31270_e46830_d_n10, assign31270_e46830_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31270_e46827: f64 = (locals.var_u0_first_fd - locals.var_phisf);
        let assign31270_e46828: f64 = { let limited_exp_arg = assign31270_e46827; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign31270_e46828, ({ let limited_exp_arg = assign31270_e46827; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_u0_first_fd_dn3 - locals.var_phisf_dn3)), ({ let limited_exp_arg = assign31270_e46827; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_u0_first_fd_dn4 - locals.var_phisf_dn4)), ({ let limited_exp_arg = assign31270_e46827; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_u0_first_fd_dn5 - locals.var_phisf_dn5)), ({ let limited_exp_arg = assign31270_e46827; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_u0_first_fd_dn6 - locals.var_phisf_dn6)), ({ let limited_exp_arg = assign31270_e46827; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_u0_first_fd_dn7 - locals.var_phisf_dn7)), ({ let limited_exp_arg = assign31270_e46827; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_u0_first_fd_dn8 - locals.var_phisf_dn8)), ({ let limited_exp_arg = assign31270_e46827; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_u0_first_fd_dn9 - locals.var_phisf_dn9)), ({ let limited_exp_arg = assign31270_e46827; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_u0_first_fd_dn10 - locals.var_phisf_dn10)), ({ let limited_exp_arg = assign31270_e46827; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_u0_first_fd_dn11 - locals.var_phisf_dn11)),)
    } else {
        (locals.var_delta0_fd, locals.var_delta0_fd_dn3, locals.var_delta0_fd_dn4, locals.var_delta0_fd_dn5, locals.var_delta0_fd_dn6, locals.var_delta0_fd_dn7, locals.var_delta0_fd_dn8, locals.var_delta0_fd_dn9, locals.var_delta0_fd_dn10, locals.var_delta0_fd_dn11,)
    }
};
        locals.var_delta0_fd = assign31270_e46830;
        locals.var_delta0_fd_dn3 = assign31270_e46830_d_n3;
        locals.var_delta0_fd_dn4 = assign31270_e46830_d_n4;
        locals.var_delta0_fd_dn5 = assign31270_e46830_d_n5;
        locals.var_delta0_fd_dn6 = assign31270_e46830_d_n6;
        locals.var_delta0_fd_dn7 = assign31270_e46830_d_n7;
        locals.var_delta0_fd_dn8 = assign31270_e46830_d_n8;
        locals.var_delta0_fd_dn9 = assign31270_e46830_d_n9;
        locals.var_delta0_fd_dn10 = assign31270_e46830_d_n10;
        locals.var_delta0_fd_dn11 = assign31270_e46830_d_n11;
        locals.var_delta0_fd_rv = 0.0;

        let (assign31280_e46859, assign31280_e46859_d_n3, assign31280_e46859_d_n4, assign31280_e46859_d_n5, assign31280_e46859_d_n6, assign31280_e46859_d_n7, assign31280_e46859_d_n8, assign31280_e46859_d_n9, assign31280_e46859_d_n10, assign31280_e46859_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31280_e46842: f64 = (locals.var_vgfb1 - locals.var_u0_first_fd);
        let assign31280_e46843: f64 = (2.0 * assign31280_e46842);
        let assign31280_e46846: f64 = (2.0 * locals.var_zeta2);
        let assign31280_e46849: f64 = (locals.var_vgfbb - locals.var_u0_first_fd);
        let assign31280_e46851: f64 = (assign31280_e46849 + locals.var_phic_star);
        let assign31280_e46852: f64 = (assign31280_e46846 * assign31280_e46851);
        let assign31280_e46853: f64 = (assign31280_e46843 - assign31280_e46852);
        let assign31280_e46856: f64 = (locals.var_gam2 * locals.var_delta0_fd);
        let assign31280_e46857: f64 = (assign31280_e46853 + assign31280_e46856);
        (assign31280_e46857, (((2.0 * (locals.var_vgfb1_dn3 - locals.var_u0_first_fd_dn3)) - (assign31280_e46846 * ((locals.var_vgfbb_dn3 - locals.var_u0_first_fd_dn3) + locals.var_phic_star_dn3))) + ((locals.var_gam2_dn3 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn3))), (((2.0 * (locals.var_vgfb1_dn4 - locals.var_u0_first_fd_dn4)) - (assign31280_e46846 * ((locals.var_vgfbb_dn4 - locals.var_u0_first_fd_dn4) + locals.var_phic_star_dn4))) + ((locals.var_gam2_dn4 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn4))), (((2.0 * (locals.var_vgfb1_dn5 - locals.var_u0_first_fd_dn5)) - (assign31280_e46846 * ((locals.var_vgfbb_dn5 - locals.var_u0_first_fd_dn5) + locals.var_phic_star_dn5))) + ((locals.var_gam2_dn5 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn5))), (((2.0 * (locals.var_vgfb1_dn6 - locals.var_u0_first_fd_dn6)) - (assign31280_e46846 * ((locals.var_vgfbb_dn6 - locals.var_u0_first_fd_dn6) + locals.var_phic_star_dn6))) + ((locals.var_gam2_dn6 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn6))), (((2.0 * (locals.var_vgfb1_dn7 - locals.var_u0_first_fd_dn7)) - (assign31280_e46846 * ((locals.var_vgfbb_dn7 - locals.var_u0_first_fd_dn7) + locals.var_phic_star_dn7))) + ((locals.var_gam2_dn7 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn7))), (((2.0 * (locals.var_vgfb1_dn8 - locals.var_u0_first_fd_dn8)) - (assign31280_e46846 * ((locals.var_vgfbb_dn8 - locals.var_u0_first_fd_dn8) + locals.var_phic_star_dn8))) + ((locals.var_gam2_dn8 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn8))), (((2.0 * (locals.var_vgfb1_dn9 - locals.var_u0_first_fd_dn9)) - (assign31280_e46846 * ((locals.var_vgfbb_dn9 - locals.var_u0_first_fd_dn9) + locals.var_phic_star_dn9))) + ((locals.var_gam2_dn9 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn9))), (((2.0 * (locals.var_vgfb1_dn10 - locals.var_u0_first_fd_dn10)) - (assign31280_e46846 * ((locals.var_vgfbb_dn10 - locals.var_u0_first_fd_dn10) + locals.var_phic_star_dn10))) + ((locals.var_gam2_dn10 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn10))), (((2.0 * (locals.var_vgfb1_dn11 - locals.var_u0_first_fd_dn11)) - (assign31280_e46846 * ((locals.var_vgfbb_dn11 - locals.var_u0_first_fd_dn11) + locals.var_phic_star_dn11))) + ((locals.var_gam2_dn11 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn11))),)
    } else {
        (locals.var_p_fd, locals.var_p_fd_dn3, locals.var_p_fd_dn4, locals.var_p_fd_dn5, locals.var_p_fd_dn6, locals.var_p_fd_dn7, locals.var_p_fd_dn8, locals.var_p_fd_dn9, locals.var_p_fd_dn10, locals.var_p_fd_dn11,)
    }
};
        locals.var_p_fd = assign31280_e46859;
        locals.var_p_fd_dn3 = assign31280_e46859_d_n3;
        locals.var_p_fd_dn4 = assign31280_e46859_d_n4;
        locals.var_p_fd_dn5 = assign31280_e46859_d_n5;
        locals.var_p_fd_dn6 = assign31280_e46859_d_n6;
        locals.var_p_fd_dn7 = assign31280_e46859_d_n7;
        locals.var_p_fd_dn8 = assign31280_e46859_d_n8;
        locals.var_p_fd_dn9 = assign31280_e46859_d_n9;
        locals.var_p_fd_dn10 = assign31280_e46859_d_n10;
        locals.var_p_fd_dn11 = assign31280_e46859_d_n11;
        locals.var_p_fd_rv = 0.0;

        let (assign31290_e46896, assign31290_e46896_d_n3, assign31290_e46896_d_n4, assign31290_e46896_d_n5, assign31290_e46896_d_n6, assign31290_e46896_d_n7, assign31290_e46896_d_n8, assign31290_e46896_d_n9, assign31290_e46896_d_n10, assign31290_e46896_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31290_e46870: f64 = (locals.var_vgfb1 - locals.var_u0_first_fd);
        let assign31290_e46873: f64 = (locals.var_vgfb1 - locals.var_u0_first_fd);
        let assign31290_e46874: f64 = (assign31290_e46870 * assign31290_e46873);
        let assign31290_e46878: f64 = (locals.var_vgfbb - locals.var_u0_first_fd);
        let assign31290_e46880: f64 = (assign31290_e46878 + locals.var_phic_star);
        let assign31290_e46881: f64 = (locals.var_zeta2 * assign31290_e46880);
        let assign31290_e46884: f64 = (locals.var_vgfbb - locals.var_u0_first_fd);
        let assign31290_e46886: f64 = (assign31290_e46884 + locals.var_phic_star);
        let assign31290_e46887: f64 = (assign31290_e46881 * assign31290_e46886);
        let assign31290_e46888: f64 = (assign31290_e46874 - assign31290_e46887);
        let assign31290_e46892: f64 = (locals.var_phic_star + locals.var_delta0_fd);
        let assign31290_e46893: f64 = (locals.var_gam2 * assign31290_e46892);
        let assign31290_e46894: f64 = (assign31290_e46888 - assign31290_e46893);
        (assign31290_e46894, (((((locals.var_vgfb1_dn3 - locals.var_u0_first_fd_dn3) * assign31290_e46873) + (assign31290_e46870 * (locals.var_vgfb1_dn3 - locals.var_u0_first_fd_dn3))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn3 - locals.var_u0_first_fd_dn3) + locals.var_phic_star_dn3)) * assign31290_e46886) + (assign31290_e46881 * ((locals.var_vgfbb_dn3 - locals.var_u0_first_fd_dn3) + locals.var_phic_star_dn3)))) - ((locals.var_gam2_dn3 * assign31290_e46892) + (locals.var_gam2 * (locals.var_phic_star_dn3 + locals.var_delta0_fd_dn3)))), (((((locals.var_vgfb1_dn4 - locals.var_u0_first_fd_dn4) * assign31290_e46873) + (assign31290_e46870 * (locals.var_vgfb1_dn4 - locals.var_u0_first_fd_dn4))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn4 - locals.var_u0_first_fd_dn4) + locals.var_phic_star_dn4)) * assign31290_e46886) + (assign31290_e46881 * ((locals.var_vgfbb_dn4 - locals.var_u0_first_fd_dn4) + locals.var_phic_star_dn4)))) - ((locals.var_gam2_dn4 * assign31290_e46892) + (locals.var_gam2 * (locals.var_phic_star_dn4 + locals.var_delta0_fd_dn4)))), (((((locals.var_vgfb1_dn5 - locals.var_u0_first_fd_dn5) * assign31290_e46873) + (assign31290_e46870 * (locals.var_vgfb1_dn5 - locals.var_u0_first_fd_dn5))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn5 - locals.var_u0_first_fd_dn5) + locals.var_phic_star_dn5)) * assign31290_e46886) + (assign31290_e46881 * ((locals.var_vgfbb_dn5 - locals.var_u0_first_fd_dn5) + locals.var_phic_star_dn5)))) - ((locals.var_gam2_dn5 * assign31290_e46892) + (locals.var_gam2 * (locals.var_phic_star_dn5 + locals.var_delta0_fd_dn5)))), (((((locals.var_vgfb1_dn6 - locals.var_u0_first_fd_dn6) * assign31290_e46873) + (assign31290_e46870 * (locals.var_vgfb1_dn6 - locals.var_u0_first_fd_dn6))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn6 - locals.var_u0_first_fd_dn6) + locals.var_phic_star_dn6)) * assign31290_e46886) + (assign31290_e46881 * ((locals.var_vgfbb_dn6 - locals.var_u0_first_fd_dn6) + locals.var_phic_star_dn6)))) - ((locals.var_gam2_dn6 * assign31290_e46892) + (locals.var_gam2 * (locals.var_phic_star_dn6 + locals.var_delta0_fd_dn6)))), (((((locals.var_vgfb1_dn7 - locals.var_u0_first_fd_dn7) * assign31290_e46873) + (assign31290_e46870 * (locals.var_vgfb1_dn7 - locals.var_u0_first_fd_dn7))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn7 - locals.var_u0_first_fd_dn7) + locals.var_phic_star_dn7)) * assign31290_e46886) + (assign31290_e46881 * ((locals.var_vgfbb_dn7 - locals.var_u0_first_fd_dn7) + locals.var_phic_star_dn7)))) - ((locals.var_gam2_dn7 * assign31290_e46892) + (locals.var_gam2 * (locals.var_phic_star_dn7 + locals.var_delta0_fd_dn7)))), (((((locals.var_vgfb1_dn8 - locals.var_u0_first_fd_dn8) * assign31290_e46873) + (assign31290_e46870 * (locals.var_vgfb1_dn8 - locals.var_u0_first_fd_dn8))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn8 - locals.var_u0_first_fd_dn8) + locals.var_phic_star_dn8)) * assign31290_e46886) + (assign31290_e46881 * ((locals.var_vgfbb_dn8 - locals.var_u0_first_fd_dn8) + locals.var_phic_star_dn8)))) - ((locals.var_gam2_dn8 * assign31290_e46892) + (locals.var_gam2 * (locals.var_phic_star_dn8 + locals.var_delta0_fd_dn8)))), (((((locals.var_vgfb1_dn9 - locals.var_u0_first_fd_dn9) * assign31290_e46873) + (assign31290_e46870 * (locals.var_vgfb1_dn9 - locals.var_u0_first_fd_dn9))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn9 - locals.var_u0_first_fd_dn9) + locals.var_phic_star_dn9)) * assign31290_e46886) + (assign31290_e46881 * ((locals.var_vgfbb_dn9 - locals.var_u0_first_fd_dn9) + locals.var_phic_star_dn9)))) - ((locals.var_gam2_dn9 * assign31290_e46892) + (locals.var_gam2 * (locals.var_phic_star_dn9 + locals.var_delta0_fd_dn9)))), (((((locals.var_vgfb1_dn10 - locals.var_u0_first_fd_dn10) * assign31290_e46873) + (assign31290_e46870 * (locals.var_vgfb1_dn10 - locals.var_u0_first_fd_dn10))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn10 - locals.var_u0_first_fd_dn10) + locals.var_phic_star_dn10)) * assign31290_e46886) + (assign31290_e46881 * ((locals.var_vgfbb_dn10 - locals.var_u0_first_fd_dn10) + locals.var_phic_star_dn10)))) - ((locals.var_gam2_dn10 * assign31290_e46892) + (locals.var_gam2 * (locals.var_phic_star_dn10 + locals.var_delta0_fd_dn10)))), (((((locals.var_vgfb1_dn11 - locals.var_u0_first_fd_dn11) * assign31290_e46873) + (assign31290_e46870 * (locals.var_vgfb1_dn11 - locals.var_u0_first_fd_dn11))) - (((locals.var_zeta2 * ((locals.var_vgfbb_dn11 - locals.var_u0_first_fd_dn11) + locals.var_phic_star_dn11)) * assign31290_e46886) + (assign31290_e46881 * ((locals.var_vgfbb_dn11 - locals.var_u0_first_fd_dn11) + locals.var_phic_star_dn11)))) - ((locals.var_gam2_dn11 * assign31290_e46892) + (locals.var_gam2 * (locals.var_phic_star_dn11 + locals.var_delta0_fd_dn11)))),)
    } else {
        (locals.var_q_fd, locals.var_q_fd_dn3, locals.var_q_fd_dn4, locals.var_q_fd_dn5, locals.var_q_fd_dn6, locals.var_q_fd_dn7, locals.var_q_fd_dn8, locals.var_q_fd_dn9, locals.var_q_fd_dn10, locals.var_q_fd_dn11,)
    }
};
        locals.var_q_fd = assign31290_e46896;
        locals.var_q_fd_dn3 = assign31290_e46896_d_n3;
        locals.var_q_fd_dn4 = assign31290_e46896_d_n4;
        locals.var_q_fd_dn5 = assign31290_e46896_d_n5;
        locals.var_q_fd_dn6 = assign31290_e46896_d_n6;
        locals.var_q_fd_dn7 = assign31290_e46896_d_n7;
        locals.var_q_fd_dn8 = assign31290_e46896_d_n8;
        locals.var_q_fd_dn9 = assign31290_e46896_d_n9;
        locals.var_q_fd_dn10 = assign31290_e46896_d_n10;
        locals.var_q_fd_dn11 = assign31290_e46896_d_n11;
        locals.var_q_fd_rv = 0.0;

        let (assign31300_e46919, assign31300_e46919_d_n3, assign31300_e46919_d_n4, assign31300_e46919_d_n5, assign31300_e46919_d_n6, assign31300_e46919_d_n7, assign31300_e46919_d_n8, assign31300_e46919_d_n9, assign31300_e46919_d_n10, assign31300_e46919_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31300_e46907: f64 = (2.0 * locals.var_q_fd);
        let assign31300_e46911: f64 = (2.0 * locals.var_zeta2);
        let assign31300_e46912: f64 = (2.0 - assign31300_e46911);
        let assign31300_e46915: f64 = (locals.var_gam2 * locals.var_delta0_fd);
        let assign31300_e46916: f64 = (assign31300_e46912 - assign31300_e46915);
        let assign31300_e46917: f64 = (assign31300_e46907 * assign31300_e46916);
        (assign31300_e46917, (((2.0 * locals.var_q_fd_dn3) * assign31300_e46916) + (assign31300_e46907 * (-((locals.var_gam2_dn3 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn3))))), (((2.0 * locals.var_q_fd_dn4) * assign31300_e46916) + (assign31300_e46907 * (-((locals.var_gam2_dn4 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn4))))), (((2.0 * locals.var_q_fd_dn5) * assign31300_e46916) + (assign31300_e46907 * (-((locals.var_gam2_dn5 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn5))))), (((2.0 * locals.var_q_fd_dn6) * assign31300_e46916) + (assign31300_e46907 * (-((locals.var_gam2_dn6 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn6))))), (((2.0 * locals.var_q_fd_dn7) * assign31300_e46916) + (assign31300_e46907 * (-((locals.var_gam2_dn7 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn7))))), (((2.0 * locals.var_q_fd_dn8) * assign31300_e46916) + (assign31300_e46907 * (-((locals.var_gam2_dn8 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn8))))), (((2.0 * locals.var_q_fd_dn9) * assign31300_e46916) + (assign31300_e46907 * (-((locals.var_gam2_dn9 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn9))))), (((2.0 * locals.var_q_fd_dn10) * assign31300_e46916) + (assign31300_e46907 * (-((locals.var_gam2_dn10 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn10))))), (((2.0 * locals.var_q_fd_dn11) * assign31300_e46916) + (assign31300_e46907 * (-((locals.var_gam2_dn11 * locals.var_delta0_fd) + (locals.var_gam2 * locals.var_delta0_fd_dn11))))),)
    } else {
        (locals.var_temp3_fd, locals.var_temp3_fd_dn3, locals.var_temp3_fd_dn4, locals.var_temp3_fd_dn5, locals.var_temp3_fd_dn6, locals.var_temp3_fd_dn7, locals.var_temp3_fd_dn8, locals.var_temp3_fd_dn9, locals.var_temp3_fd_dn10, locals.var_temp3_fd_dn11,)
    }
};
        locals.var_temp3_fd = assign31300_e46919;
        locals.var_temp3_fd_dn3 = assign31300_e46919_d_n3;
        locals.var_temp3_fd_dn4 = assign31300_e46919_d_n4;
        locals.var_temp3_fd_dn5 = assign31300_e46919_d_n5;
        locals.var_temp3_fd_dn6 = assign31300_e46919_d_n6;
        locals.var_temp3_fd_dn7 = assign31300_e46919_d_n7;
        locals.var_temp3_fd_dn8 = assign31300_e46919_d_n8;
        locals.var_temp3_fd_dn9 = assign31300_e46919_d_n9;
        locals.var_temp3_fd_dn10 = assign31300_e46919_d_n10;
        locals.var_temp3_fd_dn11 = assign31300_e46919_d_n11;
        locals.var_temp3_fd_rv = 0.0;

        let (assign31310_e46941, assign31310_e46941_d_n3, assign31310_e46941_d_n4, assign31310_e46941_d_n5, assign31310_e46941_d_n6, assign31310_e46941_d_n7, assign31310_e46941_d_n8, assign31310_e46941_d_n9, assign31310_e46941_d_n10, assign31310_e46941_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31310_e46930: f64 = (2.0 * locals.var_q_fd);
        let assign31310_e46934: f64 = (locals.var_p_fd * locals.var_p_fd);
        let assign31310_e46936: f64 = (assign31310_e46934 - locals.var_temp3_fd);
        let assign31310_e46937: f64 = (assign31310_e46936).sqrt();
        let assign31310_e46938: f64 = (locals.var_p_fd + assign31310_e46937);
        let assign31310_e46939: f64 = (assign31310_e46930 / assign31310_e46938);
        (assign31310_e46939, ((((2.0 * locals.var_q_fd_dn3) * assign31310_e46938) - (assign31310_e46930 * (locals.var_p_fd_dn3 + ((((locals.var_p_fd_dn3 * locals.var_p_fd) + (locals.var_p_fd * locals.var_p_fd_dn3)) - locals.var_temp3_fd_dn3) / (2.0 * assign31310_e46937))))) / (assign31310_e46938 * assign31310_e46938)), ((((2.0 * locals.var_q_fd_dn4) * assign31310_e46938) - (assign31310_e46930 * (locals.var_p_fd_dn4 + ((((locals.var_p_fd_dn4 * locals.var_p_fd) + (locals.var_p_fd * locals.var_p_fd_dn4)) - locals.var_temp3_fd_dn4) / (2.0 * assign31310_e46937))))) / (assign31310_e46938 * assign31310_e46938)), ((((2.0 * locals.var_q_fd_dn5) * assign31310_e46938) - (assign31310_e46930 * (locals.var_p_fd_dn5 + ((((locals.var_p_fd_dn5 * locals.var_p_fd) + (locals.var_p_fd * locals.var_p_fd_dn5)) - locals.var_temp3_fd_dn5) / (2.0 * assign31310_e46937))))) / (assign31310_e46938 * assign31310_e46938)), ((((2.0 * locals.var_q_fd_dn6) * assign31310_e46938) - (assign31310_e46930 * (locals.var_p_fd_dn6 + ((((locals.var_p_fd_dn6 * locals.var_p_fd) + (locals.var_p_fd * locals.var_p_fd_dn6)) - locals.var_temp3_fd_dn6) / (2.0 * assign31310_e46937))))) / (assign31310_e46938 * assign31310_e46938)), ((((2.0 * locals.var_q_fd_dn7) * assign31310_e46938) - (assign31310_e46930 * (locals.var_p_fd_dn7 + ((((locals.var_p_fd_dn7 * locals.var_p_fd) + (locals.var_p_fd * locals.var_p_fd_dn7)) - locals.var_temp3_fd_dn7) / (2.0 * assign31310_e46937))))) / (assign31310_e46938 * assign31310_e46938)), ((((2.0 * locals.var_q_fd_dn8) * assign31310_e46938) - (assign31310_e46930 * (locals.var_p_fd_dn8 + ((((locals.var_p_fd_dn8 * locals.var_p_fd) + (locals.var_p_fd * locals.var_p_fd_dn8)) - locals.var_temp3_fd_dn8) / (2.0 * assign31310_e46937))))) / (assign31310_e46938 * assign31310_e46938)), ((((2.0 * locals.var_q_fd_dn9) * assign31310_e46938) - (assign31310_e46930 * (locals.var_p_fd_dn9 + ((((locals.var_p_fd_dn9 * locals.var_p_fd) + (locals.var_p_fd * locals.var_p_fd_dn9)) - locals.var_temp3_fd_dn9) / (2.0 * assign31310_e46937))))) / (assign31310_e46938 * assign31310_e46938)), ((((2.0 * locals.var_q_fd_dn10) * assign31310_e46938) - (assign31310_e46930 * (locals.var_p_fd_dn10 + ((((locals.var_p_fd_dn10 * locals.var_p_fd) + (locals.var_p_fd * locals.var_p_fd_dn10)) - locals.var_temp3_fd_dn10) / (2.0 * assign31310_e46937))))) / (assign31310_e46938 * assign31310_e46938)), ((((2.0 * locals.var_q_fd_dn11) * assign31310_e46938) - (assign31310_e46930 * (locals.var_p_fd_dn11 + ((((locals.var_p_fd_dn11 * locals.var_p_fd) + (locals.var_p_fd * locals.var_p_fd_dn11)) - locals.var_temp3_fd_dn11) / (2.0 * assign31310_e46937))))) / (assign31310_e46938 * assign31310_e46938)),)
    } else {
        (locals.var_w_fd, locals.var_w_fd_dn3, locals.var_w_fd_dn4, locals.var_w_fd_dn5, locals.var_w_fd_dn6, locals.var_w_fd_dn7, locals.var_w_fd_dn8, locals.var_w_fd_dn9, locals.var_w_fd_dn10, locals.var_w_fd_dn11,)
    }
};
        locals.var_w_fd = assign31310_e46941;
        locals.var_w_fd_dn3 = assign31310_e46941_d_n3;
        locals.var_w_fd_dn4 = assign31310_e46941_d_n4;
        locals.var_w_fd_dn5 = assign31310_e46941_d_n5;
        locals.var_w_fd_dn6 = assign31310_e46941_d_n6;
        locals.var_w_fd_dn7 = assign31310_e46941_d_n7;
        locals.var_w_fd_dn8 = assign31310_e46941_d_n8;
        locals.var_w_fd_dn9 = assign31310_e46941_d_n9;
        locals.var_w_fd_dn10 = assign31310_e46941_d_n10;
        locals.var_w_fd_dn11 = assign31310_e46941_d_n11;
        locals.var_w_fd_rv = 0.0;

        let (assign31320_e46954, assign31320_e46954_d_n3, assign31320_e46954_d_n4, assign31320_e46954_d_n5, assign31320_e46954_d_n6, assign31320_e46954_d_n7, assign31320_e46954_d_n8, assign31320_e46954_d_n9, assign31320_e46954_d_n10, assign31320_e46954_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 != 0.0)) && (locals.var_guard618 == 0.0)) {
        let assign31320_e46952: f64 = (locals.var_u0_first_fd + locals.var_w_fd);
        (assign31320_e46952, (locals.var_u0_first_fd_dn3 + locals.var_w_fd_dn3), (locals.var_u0_first_fd_dn4 + locals.var_w_fd_dn4), (locals.var_u0_first_fd_dn5 + locals.var_w_fd_dn5), (locals.var_u0_first_fd_dn6 + locals.var_w_fd_dn6), (locals.var_u0_first_fd_dn7 + locals.var_w_fd_dn7), (locals.var_u0_first_fd_dn8 + locals.var_w_fd_dn8), (locals.var_u0_first_fd_dn9 + locals.var_w_fd_dn9), (locals.var_u0_first_fd_dn10 + locals.var_w_fd_dn10), (locals.var_u0_first_fd_dn11 + locals.var_w_fd_dn11),)
    } else {
        (locals.var_sp_dd, locals.var_sp_dd_dn3, locals.var_sp_dd_dn4, locals.var_sp_dd_dn5, locals.var_sp_dd_dn6, locals.var_sp_dd_dn7, locals.var_sp_dd_dn8, locals.var_sp_dd_dn9, locals.var_sp_dd_dn10, locals.var_sp_dd_dn11,)
    }
};
        locals.var_sp_dd = assign31320_e46954;
        locals.var_sp_dd_dn3 = assign31320_e46954_d_n3;
        locals.var_sp_dd_dn4 = assign31320_e46954_d_n4;
        locals.var_sp_dd_dn5 = assign31320_e46954_d_n5;
        locals.var_sp_dd_dn6 = assign31320_e46954_d_n6;
        locals.var_sp_dd_dn7 = assign31320_e46954_d_n7;
        locals.var_sp_dd_dn8 = assign31320_e46954_d_n8;
        locals.var_sp_dd_dn9 = assign31320_e46954_d_n9;
        locals.var_sp_dd_dn10 = assign31320_e46954_d_n10;
        locals.var_sp_dd_dn11 = assign31320_e46954_d_n11;
        locals.var_sp_dd_rv = 0.0;

        let assign31330_e46956: f64 = (locals.var_vgfb1).abs();
        let assign31330_e46958: f64 = if assign31330_e46956 <= locals.var_limit { 1.0 } else { 0.0 };
        locals.var_guard622 = assign31330_e46958;
        locals.var_guard622_rv = 0.0;

        let (assign31340_e46975, assign31340_e46975_d_n3, assign31340_e46975_d_n4, assign31340_e46975_d_n5, assign31340_e46975_d_n6, assign31340_e46975_d_n7, assign31340_e46975_d_n8, assign31340_e46975_d_n9, assign31340_e46975_d_n10, assign31340_e46975_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 == 0.0)) && (locals.var_guard622 != 0.0)) {
        let assign31340_e46969: f64 = (locals.var_inv_x1 * locals.var_inv_x1);
        let assign31340_e46971: f64 = (assign31340_e46969 * 0.16666666666666666);
        let assign31340_e46973: f64 = (assign31340_e46971 * 0.7071067811865475);
        (assign31340_e46973, ((((locals.var_inv_x1_dn3 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn3)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_x1_dn4 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn4)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_x1_dn5 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_x1_dn6 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_x1_dn7 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_x1_dn8 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn8)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_x1_dn9 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn9)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_x1_dn10 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn10)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_x1_dn11 * locals.var_inv_x1) + (locals.var_inv_x1 * locals.var_inv_x1_dn11)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign31340_e46975;
        locals.var_t0_dn3 = assign31340_e46975_d_n3;
        locals.var_t0_dn4 = assign31340_e46975_d_n4;
        locals.var_t0_dn5 = assign31340_e46975_d_n5;
        locals.var_t0_dn6 = assign31340_e46975_d_n6;
        locals.var_t0_dn7 = assign31340_e46975_d_n7;
        locals.var_t0_dn8 = assign31340_e46975_d_n8;
        locals.var_t0_dn9 = assign31340_e46975_d_n9;
        locals.var_t0_dn10 = assign31340_e46975_d_n10;
        locals.var_t0_dn11 = assign31340_e46975_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign31350_e47000, assign31350_e47000_d_n3, assign31350_e47000_d_n4, assign31350_e47000_d_n5, assign31350_e47000_d_n6, assign31350_e47000_d_n7, assign31350_e47000_d_n8, assign31350_e47000_d_n9, assign31350_e47000_d_n10, assign31350_e47000_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 == 0.0)) && (locals.var_guard622 != 0.0)) {
        let assign31350_e46986: f64 = (locals.var_vgfb1 * locals.var_inv_x1);
        let assign31350_e46991: f64 = (1.0 - locals.var_exp_ns);
        let assign31350_e46992: f64 = (locals.var_vgfb1 * assign31350_e46991);
        let assign31350_e46994: f64 = (assign31350_e46992 * locals.var_gam);
        let assign31350_e46996: f64 = (assign31350_e46994 * locals.var_t0);
        let assign31350_e46997: f64 = (1.0 + assign31350_e46996);
        let assign31350_e46998: f64 = (assign31350_e46986 * assign31350_e46997);
        (assign31350_e46998, ((((locals.var_vgfb1_dn3 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn3)) * assign31350_e46997) + (assign31350_e46986 * ((((((locals.var_vgfb1_dn3 * assign31350_e46991) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn3))) * locals.var_gam) + (assign31350_e46992 * locals.var_gam_dn3)) * locals.var_t0) + (assign31350_e46994 * locals.var_t0_dn3)))), ((((locals.var_vgfb1_dn4 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn4)) * assign31350_e46997) + (assign31350_e46986 * ((((((locals.var_vgfb1_dn4 * assign31350_e46991) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn4))) * locals.var_gam) + (assign31350_e46992 * locals.var_gam_dn4)) * locals.var_t0) + (assign31350_e46994 * locals.var_t0_dn4)))), ((((locals.var_vgfb1_dn5 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn5)) * assign31350_e46997) + (assign31350_e46986 * ((((((locals.var_vgfb1_dn5 * assign31350_e46991) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn5))) * locals.var_gam) + (assign31350_e46992 * locals.var_gam_dn5)) * locals.var_t0) + (assign31350_e46994 * locals.var_t0_dn5)))), ((((locals.var_vgfb1_dn6 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn6)) * assign31350_e46997) + (assign31350_e46986 * ((((((locals.var_vgfb1_dn6 * assign31350_e46991) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn6))) * locals.var_gam) + (assign31350_e46992 * locals.var_gam_dn6)) * locals.var_t0) + (assign31350_e46994 * locals.var_t0_dn6)))), ((((locals.var_vgfb1_dn7 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn7)) * assign31350_e46997) + (assign31350_e46986 * ((((((locals.var_vgfb1_dn7 * assign31350_e46991) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn7))) * locals.var_gam) + (assign31350_e46992 * locals.var_gam_dn7)) * locals.var_t0) + (assign31350_e46994 * locals.var_t0_dn7)))), ((((locals.var_vgfb1_dn8 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn8)) * assign31350_e46997) + (assign31350_e46986 * ((((((locals.var_vgfb1_dn8 * assign31350_e46991) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn8))) * locals.var_gam) + (assign31350_e46992 * locals.var_gam_dn8)) * locals.var_t0) + (assign31350_e46994 * locals.var_t0_dn8)))), ((((locals.var_vgfb1_dn9 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn9)) * assign31350_e46997) + (assign31350_e46986 * ((((((locals.var_vgfb1_dn9 * assign31350_e46991) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn9))) * locals.var_gam) + (assign31350_e46992 * locals.var_gam_dn9)) * locals.var_t0) + (assign31350_e46994 * locals.var_t0_dn9)))), ((((locals.var_vgfb1_dn10 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn10)) * assign31350_e46997) + (assign31350_e46986 * ((((((locals.var_vgfb1_dn10 * assign31350_e46991) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn10))) * locals.var_gam) + (assign31350_e46992 * locals.var_gam_dn10)) * locals.var_t0) + (assign31350_e46994 * locals.var_t0_dn10)))), ((((locals.var_vgfb1_dn11 * locals.var_inv_x1) + (locals.var_vgfb1 * locals.var_inv_x1_dn11)) * assign31350_e46997) + (assign31350_e46986 * ((((((locals.var_vgfb1_dn11 * assign31350_e46991) + (locals.var_vgfb1 * (-locals.var_exp_ns_dn11))) * locals.var_gam) + (assign31350_e46992 * locals.var_gam_dn11)) * locals.var_t0) + (assign31350_e46994 * locals.var_t0_dn11)))),)
    } else {
        (locals.var_sp_dd, locals.var_sp_dd_dn3, locals.var_sp_dd_dn4, locals.var_sp_dd_dn5, locals.var_sp_dd_dn6, locals.var_sp_dd_dn7, locals.var_sp_dd_dn8, locals.var_sp_dd_dn9, locals.var_sp_dd_dn10, locals.var_sp_dd_dn11,)
    }
};
        locals.var_sp_dd = assign31350_e47000;
        locals.var_sp_dd_dn3 = assign31350_e47000_d_n3;
        locals.var_sp_dd_dn4 = assign31350_e47000_d_n4;
        locals.var_sp_dd_dn5 = assign31350_e47000_d_n5;
        locals.var_sp_dd_dn6 = assign31350_e47000_d_n6;
        locals.var_sp_dd_dn7 = assign31350_e47000_d_n7;
        locals.var_sp_dd_dn8 = assign31350_e47000_d_n8;
        locals.var_sp_dd_dn9 = assign31350_e47000_d_n9;
        locals.var_sp_dd_dn10 = assign31350_e47000_d_n10;
        locals.var_sp_dd_dn11 = assign31350_e47000_d_n11;
        locals.var_sp_dd_rv = 0.0;

        let assign31360_e47003: f64 = (-locals.var_limit);
        let assign31360_e47004: f64 = if locals.var_vgfb1 < assign31360_e47003 { 1.0 } else { 0.0 };
        locals.var_guard623 = assign31360_e47004;
        locals.var_guard623_rv = 0.0;

        let (assign31370_e47019, assign31370_e47019_d_n3, assign31370_e47019_d_n4, assign31370_e47019_d_n5, assign31370_e47019_d_n6, assign31370_e47019_d_n7, assign31370_e47019_d_n8, assign31370_e47019_d_n9, assign31370_e47019_d_n10, assign31370_e47019_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 == 0.0)) && (locals.var_guard622 == 0.0)) && (locals.var_guard623 != 0.0)) {
        let assign31370_e47017: f64 = (-locals.var_vgfb1);
        (assign31370_e47017, (-locals.var_vgfb1_dn3), (-locals.var_vgfb1_dn4), (-locals.var_vgfb1_dn5), (-locals.var_vgfb1_dn6), (-locals.var_vgfb1_dn7), (-locals.var_vgfb1_dn8), (-locals.var_vgfb1_dn9), (-locals.var_vgfb1_dn10), (-locals.var_vgfb1_dn11),)
    } else {
        (locals.var_sp_s_yg, locals.var_sp_s_yg_dn3, locals.var_sp_s_yg_dn4, locals.var_sp_s_yg_dn5, locals.var_sp_s_yg_dn6, locals.var_sp_s_yg_dn7, locals.var_sp_s_yg_dn8, locals.var_sp_s_yg_dn9, locals.var_sp_s_yg_dn10, locals.var_sp_s_yg_dn11,)
    }
};
        locals.var_sp_s_yg = assign31370_e47019;
        locals.var_sp_s_yg_dn3 = assign31370_e47019_d_n3;
        locals.var_sp_s_yg_dn4 = assign31370_e47019_d_n4;
        locals.var_sp_s_yg_dn5 = assign31370_e47019_d_n5;
        locals.var_sp_s_yg_dn6 = assign31370_e47019_d_n6;
        locals.var_sp_s_yg_dn7 = assign31370_e47019_d_n7;
        locals.var_sp_s_yg_dn8 = assign31370_e47019_d_n8;
        locals.var_sp_s_yg_dn9 = assign31370_e47019_d_n9;
        locals.var_sp_s_yg_dn10 = assign31370_e47019_d_n10;
        locals.var_sp_s_yg_dn11 = assign31370_e47019_d_n11;
        locals.var_sp_s_yg_rv = 0.0;

        let (assign31380_e47037, assign31380_e47037_d_n3, assign31380_e47037_d_n4, assign31380_e47037_d_n5, assign31380_e47037_d_n6, assign31380_e47037_d_n7, assign31380_e47037_d_n8, assign31380_e47037_d_n9, assign31380_e47037_d_n10, assign31380_e47037_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 == 0.0)) && (locals.var_guard622 == 0.0)) && (locals.var_guard623 != 0.0)) {
        let assign31380_e47034: f64 = (locals.var_sp_s_yg * locals.var_inv_x1);
        let assign31380_e47035: f64 = (1.25 * assign31380_e47034);
        (assign31380_e47035, (1.25 * ((locals.var_sp_s_yg_dn3 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn3))), (1.25 * ((locals.var_sp_s_yg_dn4 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn4))), (1.25 * ((locals.var_sp_s_yg_dn5 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn5))), (1.25 * ((locals.var_sp_s_yg_dn6 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn6))), (1.25 * ((locals.var_sp_s_yg_dn7 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn7))), (1.25 * ((locals.var_sp_s_yg_dn8 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn8))), (1.25 * ((locals.var_sp_s_yg_dn9 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn9))), (1.25 * ((locals.var_sp_s_yg_dn10 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn10))), (1.25 * ((locals.var_sp_s_yg_dn11 * locals.var_inv_x1) + (locals.var_sp_s_yg * locals.var_inv_x1_dn11))),)
    } else {
        (locals.var_sp_s_ysub, locals.var_sp_s_ysub_dn3, locals.var_sp_s_ysub_dn4, locals.var_sp_s_ysub_dn5, locals.var_sp_s_ysub_dn6, locals.var_sp_s_ysub_dn7, locals.var_sp_s_ysub_dn8, locals.var_sp_s_ysub_dn9, locals.var_sp_s_ysub_dn10, locals.var_sp_s_ysub_dn11,)
    }
};
        locals.var_sp_s_ysub = assign31380_e47037;
        locals.var_sp_s_ysub_dn3 = assign31380_e47037_d_n3;
        locals.var_sp_s_ysub_dn4 = assign31380_e47037_d_n4;
        locals.var_sp_s_ysub_dn5 = assign31380_e47037_d_n5;
        locals.var_sp_s_ysub_dn6 = assign31380_e47037_d_n6;
        locals.var_sp_s_ysub_dn7 = assign31380_e47037_d_n7;
        locals.var_sp_s_ysub_dn8 = assign31380_e47037_d_n8;
        locals.var_sp_s_ysub_dn9 = assign31380_e47037_d_n9;
        locals.var_sp_s_ysub_dn10 = assign31380_e47037_d_n10;
        locals.var_sp_s_ysub_dn11 = assign31380_e47037_d_n11;
        locals.var_sp_s_ysub_rv = 0.0;

        let (assign31390_e47066, assign31390_e47066_d_n3, assign31390_e47066_d_n4, assign31390_e47066_d_n5, assign31390_e47066_d_n6, assign31390_e47066_d_n7, assign31390_e47066_d_n8, assign31390_e47066_d_n9, assign31390_e47066_d_n10, assign31390_e47066_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 == 0.0)) && (locals.var_guard622 == 0.0)) && (locals.var_guard623 != 0.0)) {
        let assign31390_e47052: f64 = (locals.var_sp_s_ysub + 10.0);
        let assign31390_e47055: f64 = (locals.var_sp_s_ysub - 6.0);
        let assign31390_e47058: f64 = (locals.var_sp_s_ysub - 6.0);
        let assign31390_e47059: f64 = (assign31390_e47055 * assign31390_e47058);
        let assign31390_e47061: f64 = (assign31390_e47059 + 64.0);
        let assign31390_e47062: f64 = (assign31390_e47061).sqrt();
        let assign31390_e47063: f64 = (assign31390_e47052 - assign31390_e47062);
        let assign31390_e47064: f64 = (0.5 * assign31390_e47063);
        (assign31390_e47064, (0.5 * (locals.var_sp_s_ysub_dn3 - (((locals.var_sp_s_ysub_dn3 * assign31390_e47058) + (assign31390_e47055 * locals.var_sp_s_ysub_dn3)) / (2.0 * assign31390_e47062)))), (0.5 * (locals.var_sp_s_ysub_dn4 - (((locals.var_sp_s_ysub_dn4 * assign31390_e47058) + (assign31390_e47055 * locals.var_sp_s_ysub_dn4)) / (2.0 * assign31390_e47062)))), (0.5 * (locals.var_sp_s_ysub_dn5 - (((locals.var_sp_s_ysub_dn5 * assign31390_e47058) + (assign31390_e47055 * locals.var_sp_s_ysub_dn5)) / (2.0 * assign31390_e47062)))), (0.5 * (locals.var_sp_s_ysub_dn6 - (((locals.var_sp_s_ysub_dn6 * assign31390_e47058) + (assign31390_e47055 * locals.var_sp_s_ysub_dn6)) / (2.0 * assign31390_e47062)))), (0.5 * (locals.var_sp_s_ysub_dn7 - (((locals.var_sp_s_ysub_dn7 * assign31390_e47058) + (assign31390_e47055 * locals.var_sp_s_ysub_dn7)) / (2.0 * assign31390_e47062)))), (0.5 * (locals.var_sp_s_ysub_dn8 - (((locals.var_sp_s_ysub_dn8 * assign31390_e47058) + (assign31390_e47055 * locals.var_sp_s_ysub_dn8)) / (2.0 * assign31390_e47062)))), (0.5 * (locals.var_sp_s_ysub_dn9 - (((locals.var_sp_s_ysub_dn9 * assign31390_e47058) + (assign31390_e47055 * locals.var_sp_s_ysub_dn9)) / (2.0 * assign31390_e47062)))), (0.5 * (locals.var_sp_s_ysub_dn10 - (((locals.var_sp_s_ysub_dn10 * assign31390_e47058) + (assign31390_e47055 * locals.var_sp_s_ysub_dn10)) / (2.0 * assign31390_e47062)))), (0.5 * (locals.var_sp_s_ysub_dn11 - (((locals.var_sp_s_ysub_dn11 * assign31390_e47058) + (assign31390_e47055 * locals.var_sp_s_ysub_dn11)) / (2.0 * assign31390_e47062)))),)
    } else {
        (locals.var_sp_s_eta, locals.var_sp_s_eta_dn3, locals.var_sp_s_eta_dn4, locals.var_sp_s_eta_dn5, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8, locals.var_sp_s_eta_dn9, locals.var_sp_s_eta_dn10, locals.var_sp_s_eta_dn11,)
    }
};
        locals.var_sp_s_eta = assign31390_e47066;
        locals.var_sp_s_eta_dn3 = assign31390_e47066_d_n3;
        locals.var_sp_s_eta_dn4 = assign31390_e47066_d_n4;
        locals.var_sp_s_eta_dn5 = assign31390_e47066_d_n5;
        locals.var_sp_s_eta_dn6 = assign31390_e47066_d_n6;
        locals.var_sp_s_eta_dn7 = assign31390_e47066_d_n7;
        locals.var_sp_s_eta_dn8 = assign31390_e47066_d_n8;
        locals.var_sp_s_eta_dn9 = assign31390_e47066_d_n9;
        locals.var_sp_s_eta_dn10 = assign31390_e47066_d_n10;
        locals.var_sp_s_eta_dn11 = assign31390_e47066_d_n11;
        locals.var_sp_s_eta_rv = 0.0;

        let (assign31400_e47082, assign31400_e47082_d_n3, assign31400_e47082_d_n4, assign31400_e47082_d_n5, assign31400_e47082_d_n6, assign31400_e47082_d_n7, assign31400_e47082_d_n8, assign31400_e47082_d_n9, assign31400_e47082_d_n10, assign31400_e47082_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 == 0.0)) && (locals.var_guard622 == 0.0)) && (locals.var_guard623 != 0.0)) {
        let assign31400_e47080: f64 = (locals.var_sp_s_yg - locals.var_sp_s_eta);
        (assign31400_e47080, (locals.var_sp_s_yg_dn3 - locals.var_sp_s_eta_dn3), (locals.var_sp_s_yg_dn4 - locals.var_sp_s_eta_dn4), (locals.var_sp_s_yg_dn5 - locals.var_sp_s_eta_dn5), (locals.var_sp_s_yg_dn6 - locals.var_sp_s_eta_dn6), (locals.var_sp_s_yg_dn7 - locals.var_sp_s_eta_dn7), (locals.var_sp_s_yg_dn8 - locals.var_sp_s_eta_dn8), (locals.var_sp_s_yg_dn9 - locals.var_sp_s_eta_dn9), (locals.var_sp_s_yg_dn10 - locals.var_sp_s_eta_dn10), (locals.var_sp_s_yg_dn11 - locals.var_sp_s_eta_dn11),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn3, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9, locals.var_sp_s_temp_dn10, locals.var_sp_s_temp_dn11,)
    }
};
        locals.var_sp_s_temp = assign31400_e47082;
        locals.var_sp_s_temp_dn3 = assign31400_e47082_d_n3;
        locals.var_sp_s_temp_dn4 = assign31400_e47082_d_n4;
        locals.var_sp_s_temp_dn5 = assign31400_e47082_d_n5;
        locals.var_sp_s_temp_dn6 = assign31400_e47082_d_n6;
        locals.var_sp_s_temp_dn7 = assign31400_e47082_d_n7;
        locals.var_sp_s_temp_dn8 = assign31400_e47082_d_n8;
        locals.var_sp_s_temp_dn9 = assign31400_e47082_d_n9;
        locals.var_sp_s_temp_dn10 = assign31400_e47082_d_n10;
        locals.var_sp_s_temp_dn11 = assign31400_e47082_d_n11;
        locals.var_sp_s_temp_rv = 0.0;

        let (assign31410_e47104, assign31410_e47104_d_n3, assign31410_e47104_d_n4, assign31410_e47104_d_n5, assign31410_e47104_d_n6, assign31410_e47104_d_n7, assign31410_e47104_d_n8, assign31410_e47104_d_n9, assign31410_e47104_d_n10, assign31410_e47104_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 == 0.0)) && (locals.var_guard622 == 0.0)) && (locals.var_guard623 != 0.0)) {
        let assign31410_e47096: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign31410_e47100: f64 = (locals.var_sp_s_eta + 1.0);
        let assign31410_e47101: f64 = (locals.var_gam2 * assign31410_e47100);
        let assign31410_e47102: f64 = (assign31410_e47096 + assign31410_e47101);
        (assign31410_e47102, (((locals.var_sp_s_temp_dn3 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn3)) + ((locals.var_gam2_dn3 * assign31410_e47100) + (locals.var_gam2 * locals.var_sp_s_eta_dn3))), (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) + ((locals.var_gam2_dn4 * assign31410_e47100) + (locals.var_gam2 * locals.var_sp_s_eta_dn4))), (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) + ((locals.var_gam2_dn5 * assign31410_e47100) + (locals.var_gam2 * locals.var_sp_s_eta_dn5))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) + ((locals.var_gam2_dn6 * assign31410_e47100) + (locals.var_gam2 * locals.var_sp_s_eta_dn6))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) + ((locals.var_gam2_dn7 * assign31410_e47100) + (locals.var_gam2 * locals.var_sp_s_eta_dn7))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) + ((locals.var_gam2_dn8 * assign31410_e47100) + (locals.var_gam2 * locals.var_sp_s_eta_dn8))), (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) + ((locals.var_gam2_dn9 * assign31410_e47100) + (locals.var_gam2 * locals.var_sp_s_eta_dn9))), (((locals.var_sp_s_temp_dn10 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn10)) + ((locals.var_gam2_dn10 * assign31410_e47100) + (locals.var_gam2 * locals.var_sp_s_eta_dn10))), (((locals.var_sp_s_temp_dn11 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn11)) + ((locals.var_gam2_dn11 * assign31410_e47100) + (locals.var_gam2 * locals.var_sp_s_eta_dn11))),)
    } else {
        (locals.var_sp_s_a, locals.var_sp_s_a_dn3, locals.var_sp_s_a_dn4, locals.var_sp_s_a_dn5, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8, locals.var_sp_s_a_dn9, locals.var_sp_s_a_dn10, locals.var_sp_s_a_dn11,)
    }
};
        locals.var_sp_s_a = assign31410_e47104;
        locals.var_sp_s_a_dn3 = assign31410_e47104_d_n3;
        locals.var_sp_s_a_dn4 = assign31410_e47104_d_n4;
        locals.var_sp_s_a_dn5 = assign31410_e47104_d_n5;
        locals.var_sp_s_a_dn6 = assign31410_e47104_d_n6;
        locals.var_sp_s_a_dn7 = assign31410_e47104_d_n7;
        locals.var_sp_s_a_dn8 = assign31410_e47104_d_n8;
        locals.var_sp_s_a_dn9 = assign31410_e47104_d_n9;
        locals.var_sp_s_a_dn10 = assign31410_e47104_d_n10;
        locals.var_sp_s_a_dn11 = assign31410_e47104_d_n11;
        locals.var_sp_s_a_rv = 0.0;

        let (assign31420_e47122, assign31420_e47122_d_n3, assign31420_e47122_d_n4, assign31420_e47122_d_n5, assign31420_e47122_d_n6, assign31420_e47122_d_n7, assign31420_e47122_d_n8, assign31420_e47122_d_n9, assign31420_e47122_d_n10, assign31420_e47122_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 == 0.0)) && (locals.var_guard622 == 0.0)) && (locals.var_guard623 != 0.0)) {
        let assign31420_e47118: f64 = (2.0 * locals.var_sp_s_temp);
        let assign31420_e47120: f64 = (assign31420_e47118 - locals.var_gam2);
        (assign31420_e47120, ((2.0 * locals.var_sp_s_temp_dn3) - locals.var_gam2_dn3), ((2.0 * locals.var_sp_s_temp_dn4) - locals.var_gam2_dn4), ((2.0 * locals.var_sp_s_temp_dn5) - locals.var_gam2_dn5), ((2.0 * locals.var_sp_s_temp_dn6) - locals.var_gam2_dn6), ((2.0 * locals.var_sp_s_temp_dn7) - locals.var_gam2_dn7), ((2.0 * locals.var_sp_s_temp_dn8) - locals.var_gam2_dn8), ((2.0 * locals.var_sp_s_temp_dn9) - locals.var_gam2_dn9), ((2.0 * locals.var_sp_s_temp_dn10) - locals.var_gam2_dn10), ((2.0 * locals.var_sp_s_temp_dn11) - locals.var_gam2_dn11),)
    } else {
        (locals.var_sp_s_c, locals.var_sp_s_c_dn3, locals.var_sp_s_c_dn4, locals.var_sp_s_c_dn5, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8, locals.var_sp_s_c_dn9, locals.var_sp_s_c_dn10, locals.var_sp_s_c_dn11,)
    }
};
        locals.var_sp_s_c = assign31420_e47122;
        locals.var_sp_s_c_dn3 = assign31420_e47122_d_n3;
        locals.var_sp_s_c_dn4 = assign31420_e47122_d_n4;
        locals.var_sp_s_c_dn5 = assign31420_e47122_d_n5;
        locals.var_sp_s_c_dn6 = assign31420_e47122_d_n6;
        locals.var_sp_s_c_dn7 = assign31420_e47122_d_n7;
        locals.var_sp_s_c_dn8 = assign31420_e47122_d_n8;
        locals.var_sp_s_c_dn9 = assign31420_e47122_d_n9;
        locals.var_sp_s_c_dn10 = assign31420_e47122_d_n10;
        locals.var_sp_s_c_dn11 = assign31420_e47122_d_n11;
        locals.var_sp_s_c_rv = 0.0;

        let (assign31430_e47144, assign31430_e47144_d_n3, assign31430_e47144_d_n4, assign31430_e47144_d_n5, assign31430_e47144_d_n6, assign31430_e47144_d_n7, assign31430_e47144_d_n8, assign31430_e47144_d_n9, assign31430_e47144_d_n10, assign31430_e47144_d_n11,) = {
    if (((((locals.var_guard492 != 0.0) && (locals.var_guard613 != 0.0)) && (locals.var_guard617 == 0.0)) && (locals.var_guard622 == 0.0)) && (locals.var_guard623 != 0.0)) {
        let assign31430_e47135: f64 = (-locals.var_sp_s_eta);
        let assign31430_e47138: f64 = (locals.var_sp_s_a * locals.var_inv_gam2);
        let assign31430_e47140: f64 = (assign31430_e47138).max(1e-38);
        let assign31430_e47141: f64 = (assign31430_e47140).ln();
        let assign31430_e47142: f64 = (assign31430_e47135 + assign31430_e47141);
        (assign31430_e47142, ((-locals.var_sp_s_eta_dn3) + (if assign31430_e47138 >= 1e-38 { ((locals.var_sp_s_a_dn3 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn3)) } else { 0.0 } / assign31430_e47140)), ((-locals.var_sp_s_eta_dn4) + (if assign31430_e47138 >= 1e-38 { ((locals.var_sp_s_a_dn4 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn4)) } else { 0.0 } / assign31430_e47140)), ((-locals.var_sp_s_eta_dn5) + (if assign31430_e47138 >= 1e-38 { ((locals.var_sp_s_a_dn5 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn5)) } else { 0.0 } / assign31430_e47140)), ((-locals.var_sp_s_eta_dn6) + (if assign31430_e47138 >= 1e-38 { ((locals.var_sp_s_a_dn6 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn6)) } else { 0.0 } / assign31430_e47140)), ((-locals.var_sp_s_eta_dn7) + (if assign31430_e47138 >= 1e-38 { ((locals.var_sp_s_a_dn7 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn7)) } else { 0.0 } / assign31430_e47140)), ((-locals.var_sp_s_eta_dn8) + (if assign31430_e47138 >= 1e-38 { ((locals.var_sp_s_a_dn8 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn8)) } else { 0.0 } / assign31430_e47140)), ((-locals.var_sp_s_eta_dn9) + (if assign31430_e47138 >= 1e-38 { ((locals.var_sp_s_a_dn9 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn9)) } else { 0.0 } / assign31430_e47140)), ((-locals.var_sp_s_eta_dn10) + (if assign31430_e47138 >= 1e-38 { ((locals.var_sp_s_a_dn10 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn10)) } else { 0.0 } / assign31430_e47140)), ((-locals.var_sp_s_eta_dn11) + (if assign31430_e47138 >= 1e-38 { ((locals.var_sp_s_a_dn11 * locals.var_inv_gam2) + (locals.var_sp_s_a * locals.var_inv_gam2_dn11)) } else { 0.0 } / assign31430_e47140)),)
    } else {
        (locals.var_sp_s_tau, locals.var_sp_s_tau_dn3, locals.var_sp_s_tau_dn4, locals.var_sp_s_tau_dn5, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8, locals.var_sp_s_tau_dn9, locals.var_sp_s_tau_dn10, locals.var_sp_s_tau_dn11,)
    }
};
        locals.var_sp_s_tau = assign31430_e47144;
        locals.var_sp_s_tau_dn3 = assign31430_e47144_d_n3;
        locals.var_sp_s_tau_dn4 = assign31430_e47144_d_n4;
        locals.var_sp_s_tau_dn5 = assign31430_e47144_d_n5;
        locals.var_sp_s_tau_dn6 = assign31430_e47144_d_n6;
        locals.var_sp_s_tau_dn7 = assign31430_e47144_d_n7;
        locals.var_sp_s_tau_dn8 = assign31430_e47144_d_n8;
        locals.var_sp_s_tau_dn9 = assign31430_e47144_d_n9;
        locals.var_sp_s_tau_dn10 = assign31430_e47144_d_n10;
        locals.var_sp_s_tau_dn11 = assign31430_e47144_d_n11;
        locals.var_sp_s_tau_rv = 0.0;

    }
}
