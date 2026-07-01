#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        p: &Parameters,
        var_exp_vmax_over_phitd_bot: f64,
        var_exp_vmax_over_phitd_bot_dn0: f64,
        var_exp_vmax_over_phitd_bot_dn2: f64,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_ndigat_i: f64,
        var_ndisti_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_vak: f64,
        var_vak_dn0: f64,
        var_vak_dn2: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_guard495_slot: &mut f64,
        var_guard498_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_vha1_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_guard495: f64 = *var_guard495_slot;
        let mut var_guard498: f64 = *var_guard498_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_vha1: f64 = *var_vha1_slot;

        let (assign30690_e45931, assign30690_e45931_d_n0, assign30690_e45931_d_n2,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30690_e45915: f64 = (var_vmax * var_dnj1_dv);
        let assign30690_e45916: f64 = (var_nj1 - assign30690_e45915);
        let assign30690_e45919: f64 = (var_nj1 * var_nj1);
        let assign30690_e45920: f64 = (assign30690_e45916 / assign30690_e45919);
        let assign30690_e45923: f64 = (var_vha1 * var_dnj1_dv);
        let assign30690_e45926: f64 = (var_nj0 * p.p85);
        let assign30690_e45927: f64 = (assign30690_e45923 / assign30690_e45926);
        let assign30690_e45928: f64 = (assign30690_e45920 + assign30690_e45927);
        let assign30690_e45929: f64 = (var_phitdinv * assign30690_e45928);
        (assign30690_e45929, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign30690_e45919 * assign30690_e45919)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_dn0 * p.p85))) / (assign30690_e45926 * assign30690_e45926)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign30690_e45919) - (assign30690_e45916 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign30690_e45919 * assign30690_e45919)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign30690_e45926) - (assign30690_e45923 * (var_nj0_dn2 * p.p85))) / (assign30690_e45926 * assign30690_e45926)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign30690_e45931;
        var_dvmax_over_phitd_dv_dn0 = assign30690_e45931_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign30690_e45931_d_n2;

        let (assign30700_e45949, assign30700_e45949_d_n0, assign30700_e45949_d_n2,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30700_e45942: f64 = (var_vak - var_vmax);
        let assign30700_e45944: f64 = (assign30700_e45942 * var_dvmax_over_phitd_dv);
        let assign30700_e45945: f64 = (1.0 + assign30700_e45944);
        let assign30700_e45947: f64 = (assign30700_e45945 * var_exp_vmax_over_phitd_bot);
        (assign30700_e45947, ((((var_vak_dn0 * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_dn0)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_dn0)), ((((var_vak_dn2 * var_dvmax_over_phitd_dv) + (assign30700_e45942 * var_dvmax_over_phitd_dv_dn2)) * var_exp_vmax_over_phitd_bot) + (assign30700_e45945 * var_exp_vmax_over_phitd_bot_dn2)),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign30700_e45949;
        var_idmultbot_dn0 = assign30700_e45949_d_n0;
        var_idmultbot_dn2 = assign30700_e45949_d_n2;

        let (assign30710_e45963,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30710_e45959: f64 = (var_nin * var_nin);
        let assign30710_e45961: f64 = (assign30710_e45959 / var_ndisti_i);
        (assign30710_e45961,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign30710_e45963;

        let (assign30720_e45980,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30720_e45973: f64 = (var_nfasti_i / var_phitdinv);
        let assign30720_e45976: f64 = (var_ndisti_i / var_pnn0);
        let assign30720_e45977: f64 = (assign30720_e45976).ln();
        let assign30720_e45978: f64 = (assign30720_e45973 * assign30720_e45977);
        (assign30720_e45978,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign30720_e45980;

        let assign30730_e45983: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard495 = assign30730_e45983;

        let (assign30740_e46001, assign30740_e46001_d_n0, assign30740_e46001_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30740_e45996: f64 = (var_vmax - var_vha1);
        let assign30740_e45997: f64 = (p.p86 * assign30740_e45996);
        let assign30740_e45999: f64 = (assign30740_e45997 + var_nfasti_i);
        (assign30740_e45999, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign30740_e46001;
        var_nja10_dn0 = assign30740_e46001_d_n0;
        var_nja10_dn2 = assign30740_e46001_d_n2;

        let (assign30750_e46017, assign30750_e46017_d_n0, assign30750_e46017_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30750_e46014: f64 = (p.p86 * var_vha1);
        let assign30750_e46015: f64 = (var_nfasti_i - assign30750_e46014);
        (assign30750_e46015, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign30750_e46017;
        var_nj0_dn0 = assign30750_e46017_d_n0;
        var_nj0_dn2 = assign30750_e46017_d_n2;

        let (assign30760_e46033, assign30760_e46033_d_n0, assign30760_e46033_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30760_e46029: f64 = (p.p85 - var_nja10);
        let assign30760_e46031: f64 = (assign30760_e46029 - 0.01);
        (assign30760_e46031, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign30760_e46033;
        var_tmf1_dn0 = assign30760_e46033_d_n0;
        var_tmf1_dn2 = assign30760_e46033_d_n2;

        let (assign30770_e46049, assign30770_e46049_d_n0, assign30770_e46049_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30770_e46045: f64 = (4.0 * p.p85);
        let assign30770_e46047: f64 = (assign30770_e46045 * 0.01);
        (assign30770_e46047, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30770_e46049;
        var_tmf2_dn0 = assign30770_e46049_d_n0;
        var_tmf2_dn2 = assign30770_e46049_d_n2;

        let (assign30780_e46067, assign30780_e46067_d_n0, assign30780_e46067_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let (assign30780_e46065, assign30780_e46065_d_n0, assign30780_e46065_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign30780_e46064: f64 = (-var_tmf2);
                (assign30780_e46064, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign30780_e46065, assign30780_e46065_d_n0, assign30780_e46065_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30780_e46067;
        var_tmf2_dn0 = assign30780_e46067_d_n0;
        var_tmf2_dn2 = assign30780_e46067_d_n2;

        let (assign30790_e46084, assign30790_e46084_d_n0, assign30790_e46084_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30790_e46079: f64 = (var_tmf1 * var_tmf1);
        let assign30790_e46081: f64 = (assign30790_e46079 + var_tmf2);
        let assign30790_e46082: f64 = (assign30790_e46081).sqrt();
        (assign30790_e46082, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30790_e46082)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30790_e46082)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30790_e46084;
        var_tmf2_dn0 = assign30790_e46084_d_n0;
        var_tmf2_dn2 = assign30790_e46084_d_n2;

        let (assign30800_e46102, assign30800_e46102_d_n0, assign30800_e46102_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30800_e46098: f64 = (var_tmf1 / var_tmf2);
        let assign30800_e46099: f64 = (1.0 + assign30800_e46098);
        let assign30800_e46100: f64 = (0.5 * assign30800_e46099);
        (assign30800_e46100, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign30800_e46102;
        var_dfn_su_dn0 = assign30800_e46102_d_n0;
        var_dfn_su_dn2 = assign30800_e46102_d_n2;

        let (assign30810_e46120, assign30810_e46120_d_n0, assign30810_e46120_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30810_e46116: f64 = (var_tmf1 + var_tmf2);
        let assign30810_e46117: f64 = (0.5 * assign30810_e46116);
        let assign30810_e46118: f64 = (p.p85 - assign30810_e46117);
        (assign30810_e46118, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign30810_e46120;
        var_nja11_dn0 = assign30810_e46120_d_n0;
        var_nja11_dn2 = assign30810_e46120_d_n2;

        let (assign30820_e46136, assign30820_e46136_d_n0, assign30820_e46136_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30820_e46132: f64 = (var_nja11 - var_nfasti_i);
        let assign30820_e46134: f64 = (assign30820_e46132 - 0.01);
        (assign30820_e46134, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign30820_e46136;
        var_tmf1_dn0 = assign30820_e46136_d_n0;
        var_tmf1_dn2 = assign30820_e46136_d_n2;

        let (assign30830_e46152, assign30830_e46152_d_n0, assign30830_e46152_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30830_e46148: f64 = (4.0 * var_nfasti_i);
        let assign30830_e46150: f64 = (assign30830_e46148 * 0.01);
        (assign30830_e46150, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30830_e46152;
        var_tmf2_dn0 = assign30830_e46152_d_n0;
        var_tmf2_dn2 = assign30830_e46152_d_n2;

        let (assign30840_e46170, assign30840_e46170_d_n0, assign30840_e46170_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let (assign30840_e46168, assign30840_e46168_d_n0, assign30840_e46168_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign30840_e46167: f64 = (-var_tmf2);
                (assign30840_e46167, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign30840_e46168, assign30840_e46168_d_n0, assign30840_e46168_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30840_e46170;
        var_tmf2_dn0 = assign30840_e46170_d_n0;
        var_tmf2_dn2 = assign30840_e46170_d_n2;

        let (assign30850_e46187, assign30850_e46187_d_n0, assign30850_e46187_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30850_e46182: f64 = (var_tmf1 * var_tmf1);
        let assign30850_e46184: f64 = (assign30850_e46182 + var_tmf2);
        let assign30850_e46185: f64 = (assign30850_e46184).sqrt();
        (assign30850_e46185, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30850_e46185)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30850_e46185)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30850_e46187;
        var_tmf2_dn0 = assign30850_e46187_d_n0;
        var_tmf2_dn2 = assign30850_e46187_d_n2;

        let (assign30860_e46205, assign30860_e46205_d_n0, assign30860_e46205_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30860_e46201: f64 = (var_tmf1 / var_tmf2);
        let assign30860_e46202: f64 = (1.0 + assign30860_e46201);
        let assign30860_e46203: f64 = (0.5 * assign30860_e46202);
        (assign30860_e46203, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign30860_e46205;
        var_dfn_sl_dn0 = assign30860_e46205_d_n0;
        var_dfn_sl_dn2 = assign30860_e46205_d_n2;

        let (assign30870_e46223, assign30870_e46223_d_n0, assign30870_e46223_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30870_e46219: f64 = (var_tmf1 + var_tmf2);
        let assign30870_e46220: f64 = (0.5 * assign30870_e46219);
        let assign30870_e46221: f64 = (var_nfasti_i + assign30870_e46220);
        (assign30870_e46221, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign30870_e46223;
        var_nj1_dn0 = assign30870_e46223_d_n0;
        var_nj1_dn2 = assign30870_e46223_d_n2;

        let (assign30880_e46239, assign30880_e46239_d_n0, assign30880_e46239_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30880_e46235: f64 = (p.p85 - var_nj0);
        let assign30880_e46237: f64 = (assign30880_e46235 - 0.01);
        (assign30880_e46237, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign30880_e46239;
        var_tmf1_dn0 = assign30880_e46239_d_n0;
        var_tmf1_dn2 = assign30880_e46239_d_n2;

        let (assign30890_e46255, assign30890_e46255_d_n0, assign30890_e46255_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30890_e46251: f64 = (4.0 * p.p85);
        let assign30890_e46253: f64 = (assign30890_e46251 * 0.01);
        (assign30890_e46253, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30890_e46255;
        var_tmf2_dn0 = assign30890_e46255_d_n0;
        var_tmf2_dn2 = assign30890_e46255_d_n2;

        let (assign30900_e46273, assign30900_e46273_d_n0, assign30900_e46273_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let (assign30900_e46271, assign30900_e46271_d_n0, assign30900_e46271_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign30900_e46270: f64 = (-var_tmf2);
                (assign30900_e46270, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign30900_e46271, assign30900_e46271_d_n0, assign30900_e46271_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30900_e46273;
        var_tmf2_dn0 = assign30900_e46273_d_n0;
        var_tmf2_dn2 = assign30900_e46273_d_n2;

        let (assign30910_e46290, assign30910_e46290_d_n0, assign30910_e46290_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30910_e46285: f64 = (var_tmf1 * var_tmf1);
        let assign30910_e46287: f64 = (assign30910_e46285 + var_tmf2);
        let assign30910_e46288: f64 = (assign30910_e46287).sqrt();
        (assign30910_e46288, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30910_e46288)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30910_e46288)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30910_e46290;
        var_tmf2_dn0 = assign30910_e46290_d_n0;
        var_tmf2_dn2 = assign30910_e46290_d_n2;

        let (assign30920_e46308, assign30920_e46308_d_n0, assign30920_e46308_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30920_e46304: f64 = (var_tmf1 + var_tmf2);
        let assign30920_e46305: f64 = (0.5 * assign30920_e46304);
        let assign30920_e46306: f64 = (p.p85 - assign30920_e46305);
        (assign30920_e46306, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign30920_e46308;
        var_nj0_dn0 = assign30920_e46308_d_n0;
        var_nj0_dn2 = assign30920_e46308_d_n2;

        let (assign30930_e46324, assign30930_e46324_d_n0, assign30930_e46324_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30930_e46320: f64 = (var_nj0 - var_nfasti_i);
        let assign30930_e46322: f64 = (assign30930_e46320 - 0.01);
        (assign30930_e46322, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign30930_e46324;
        var_tmf1_dn0 = assign30930_e46324_d_n0;
        var_tmf1_dn2 = assign30930_e46324_d_n2;

        let (assign30940_e46340, assign30940_e46340_d_n0, assign30940_e46340_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30940_e46336: f64 = (4.0 * var_nfasti_i);
        let assign30940_e46338: f64 = (assign30940_e46336 * 0.01);
        (assign30940_e46338, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30940_e46340;
        var_tmf2_dn0 = assign30940_e46340_d_n0;
        var_tmf2_dn2 = assign30940_e46340_d_n2;

        let (assign30950_e46358, assign30950_e46358_d_n0, assign30950_e46358_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let (assign30950_e46356, assign30950_e46356_d_n0, assign30950_e46356_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign30950_e46355: f64 = (-var_tmf2);
                (assign30950_e46355, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign30950_e46356, assign30950_e46356_d_n0, assign30950_e46356_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30950_e46358;
        var_tmf2_dn0 = assign30950_e46358_d_n0;
        var_tmf2_dn2 = assign30950_e46358_d_n2;

        let (assign30960_e46375, assign30960_e46375_d_n0, assign30960_e46375_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30960_e46370: f64 = (var_tmf1 * var_tmf1);
        let assign30960_e46372: f64 = (assign30960_e46370 + var_tmf2);
        let assign30960_e46373: f64 = (assign30960_e46372).sqrt();
        (assign30960_e46373, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30960_e46373)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30960_e46373)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30960_e46375;
        var_tmf2_dn0 = assign30960_e46375_d_n0;
        var_tmf2_dn2 = assign30960_e46375_d_n2;

        let (assign30970_e46393, assign30970_e46393_d_n0, assign30970_e46393_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30970_e46389: f64 = (var_tmf1 + var_tmf2);
        let assign30970_e46390: f64 = (0.5 * assign30970_e46389);
        let assign30970_e46391: f64 = (var_nfasti_i + assign30970_e46390);
        (assign30970_e46391, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign30970_e46393;
        var_nj0_dn0 = assign30970_e46393_d_n0;
        var_nj0_dn2 = assign30970_e46393_d_n2;

        let (assign30980_e46409, assign30980_e46409_d_n0, assign30980_e46409_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 != 0.0)) {
        let assign30980_e46405: f64 = (p.p86 * var_dfn_su);
        let assign30980_e46407: f64 = (assign30980_e46405 * var_dfn_sl);
        (assign30980_e46407, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign30980_e46405 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign30980_e46409;
        var_dnj1_dv_dn0 = assign30980_e46409_d_n0;
        var_dnj1_dv_dn2 = assign30980_e46409_d_n2;

        let (assign30990_e46422, assign30990_e46422_d_n0, assign30990_e46422_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign30990_e46422;
        var_nj0_dn0 = assign30990_e46422_d_n0;
        var_nj0_dn2 = assign30990_e46422_d_n2;

        let (assign31000_e46435, assign31000_e46435_d_n0, assign31000_e46435_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign31000_e46435;
        var_nj1_dn0 = assign31000_e46435_d_n0;
        var_nj1_dn2 = assign31000_e46435_d_n2;

        let (assign31010_e46448, assign31010_e46448_d_n0, assign31010_e46448_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard495 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign31010_e46448;
        var_dnj1_dv_dn0 = assign31010_e46448_d_n0;
        var_dnj1_dv_dn2 = assign31010_e46448_d_n2;

        let (assign31070_e46701, assign31070_e46701_d_n0, assign31070_e46701_d_n2,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign31070_e46685: f64 = (var_vmax * var_dnj1_dv);
        let assign31070_e46686: f64 = (var_nj1 - assign31070_e46685);
        let assign31070_e46689: f64 = (var_nj1 * var_nj1);
        let assign31070_e46690: f64 = (assign31070_e46686 / assign31070_e46689);
        let assign31070_e46693: f64 = (var_vha1 * var_dnj1_dv);
        let assign31070_e46696: f64 = (var_nj0 * p.p85);
        let assign31070_e46697: f64 = (assign31070_e46693 / assign31070_e46696);
        let assign31070_e46698: f64 = (assign31070_e46690 + assign31070_e46697);
        let assign31070_e46699: f64 = (var_phitdinv * assign31070_e46698);
        (assign31070_e46699, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign31070_e46689 * assign31070_e46689)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_dn0 * p.p85))) / (assign31070_e46696 * assign31070_e46696)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign31070_e46689) - (assign31070_e46686 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign31070_e46689 * assign31070_e46689)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign31070_e46696) - (assign31070_e46693 * (var_nj0_dn2 * p.p85))) / (assign31070_e46696 * assign31070_e46696)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign31070_e46701;
        var_dvmax_over_phitd_dv_dn0 = assign31070_e46701_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign31070_e46701_d_n2;

        let (assign31090_e46733,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign31090_e46729: f64 = (var_nin * var_nin);
        let assign31090_e46731: f64 = (assign31090_e46729 / var_ndigat_i);
        (assign31090_e46731,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign31090_e46733;

        let (assign31100_e46750,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign31100_e46743: f64 = (var_nfagat_i / var_phitdinv);
        let assign31100_e46746: f64 = (var_ndigat_i / var_pnn0);
        let assign31100_e46747: f64 = (assign31100_e46746).ln();
        let assign31100_e46748: f64 = (assign31100_e46743 * assign31100_e46747);
        (assign31100_e46748,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign31100_e46750;

        let assign31110_e46753: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard498 = assign31110_e46753;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_guard495_slot = var_guard495;
        *var_guard498_slot = var_guard498;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_pnn0_slot = var_pnn0;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_vha1_slot = var_vha1;
    }

    pub(super) fn stamp_transient_block_33(
        p: &Parameters,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard498: f64,
        var_nfagat_i: f64,
        var_njl: f64,
        var_phitdinv: f64,
        var_v_hk: f64,
        var_vak: f64,
        var_vak_dn0: f64,
        var_vak_dn2: f64,
        var_vha1: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_guard558_slot: &mut f64,
        var_guard559_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj_k0_slot: &mut f64,
        var_nj_k0_dn0_slot: &mut f64,
        var_nj_k0_dn2_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_guard558: f64 = *var_guard558_slot;
        let mut var_guard559: f64 = *var_guard559_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj_k0: f64 = *var_nj_k0_slot;
        let mut var_nj_k0_dn0: f64 = *var_nj_k0_dn0_slot;
        let mut var_nj_k0_dn2: f64 = *var_nj_k0_dn2_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;

        let (assign31120_e46771, assign31120_e46771_d_n0, assign31120_e46771_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31120_e46766: f64 = (var_vmax - var_vha1);
        let assign31120_e46767: f64 = (p.p86 * assign31120_e46766);
        let assign31120_e46769: f64 = (assign31120_e46767 + var_nfagat_i);
        (assign31120_e46769, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign31120_e46771;
        var_nja10_dn0 = assign31120_e46771_d_n0;
        var_nja10_dn2 = assign31120_e46771_d_n2;

        let (assign31130_e46787, assign31130_e46787_d_n0, assign31130_e46787_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31130_e46784: f64 = (p.p86 * var_vha1);
        let assign31130_e46785: f64 = (var_nfagat_i - assign31130_e46784);
        (assign31130_e46785, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign31130_e46787;
        var_nj0_dn0 = assign31130_e46787_d_n0;
        var_nj0_dn2 = assign31130_e46787_d_n2;

        let (assign31140_e46803, assign31140_e46803_d_n0, assign31140_e46803_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31140_e46799: f64 = (p.p85 - var_nja10);
        let assign31140_e46801: f64 = (assign31140_e46799 - 0.01);
        (assign31140_e46801, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign31140_e46803;
        var_tmf1_dn0 = assign31140_e46803_d_n0;
        var_tmf1_dn2 = assign31140_e46803_d_n2;

        let (assign31150_e46819, assign31150_e46819_d_n0, assign31150_e46819_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31150_e46815: f64 = (4.0 * p.p85);
        let assign31150_e46817: f64 = (assign31150_e46815 * 0.01);
        (assign31150_e46817, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign31150_e46819;
        var_tmf2_dn0 = assign31150_e46819_d_n0;
        var_tmf2_dn2 = assign31150_e46819_d_n2;

        let (assign31160_e46837, assign31160_e46837_d_n0, assign31160_e46837_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let (assign31160_e46835, assign31160_e46835_d_n0, assign31160_e46835_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign31160_e46834: f64 = (-var_tmf2);
                (assign31160_e46834, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign31160_e46835, assign31160_e46835_d_n0, assign31160_e46835_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign31160_e46837;
        var_tmf2_dn0 = assign31160_e46837_d_n0;
        var_tmf2_dn2 = assign31160_e46837_d_n2;

        let (assign31170_e46854, assign31170_e46854_d_n0, assign31170_e46854_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31170_e46849: f64 = (var_tmf1 * var_tmf1);
        let assign31170_e46851: f64 = (assign31170_e46849 + var_tmf2);
        let assign31170_e46852: f64 = (assign31170_e46851).sqrt();
        (assign31170_e46852, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign31170_e46852)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign31170_e46852)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign31170_e46854;
        var_tmf2_dn0 = assign31170_e46854_d_n0;
        var_tmf2_dn2 = assign31170_e46854_d_n2;

        let (assign31180_e46872, assign31180_e46872_d_n0, assign31180_e46872_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31180_e46868: f64 = (var_tmf1 / var_tmf2);
        let assign31180_e46869: f64 = (1.0 + assign31180_e46868);
        let assign31180_e46870: f64 = (0.5 * assign31180_e46869);
        (assign31180_e46870, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign31180_e46872;
        var_dfn_su_dn0 = assign31180_e46872_d_n0;
        var_dfn_su_dn2 = assign31180_e46872_d_n2;

        let (assign31190_e46890, assign31190_e46890_d_n0, assign31190_e46890_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31190_e46886: f64 = (var_tmf1 + var_tmf2);
        let assign31190_e46887: f64 = (0.5 * assign31190_e46886);
        let assign31190_e46888: f64 = (p.p85 - assign31190_e46887);
        (assign31190_e46888, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign31190_e46890;
        var_nja11_dn0 = assign31190_e46890_d_n0;
        var_nja11_dn2 = assign31190_e46890_d_n2;

        let (assign31200_e46906, assign31200_e46906_d_n0, assign31200_e46906_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31200_e46902: f64 = (var_nja11 - var_nfagat_i);
        let assign31200_e46904: f64 = (assign31200_e46902 - 0.01);
        (assign31200_e46904, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign31200_e46906;
        var_tmf1_dn0 = assign31200_e46906_d_n0;
        var_tmf1_dn2 = assign31200_e46906_d_n2;

        let (assign31210_e46922, assign31210_e46922_d_n0, assign31210_e46922_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31210_e46918: f64 = (4.0 * var_nfagat_i);
        let assign31210_e46920: f64 = (assign31210_e46918 * 0.01);
        (assign31210_e46920, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign31210_e46922;
        var_tmf2_dn0 = assign31210_e46922_d_n0;
        var_tmf2_dn2 = assign31210_e46922_d_n2;

        let (assign31220_e46940, assign31220_e46940_d_n0, assign31220_e46940_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let (assign31220_e46938, assign31220_e46938_d_n0, assign31220_e46938_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign31220_e46937: f64 = (-var_tmf2);
                (assign31220_e46937, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign31220_e46938, assign31220_e46938_d_n0, assign31220_e46938_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign31220_e46940;
        var_tmf2_dn0 = assign31220_e46940_d_n0;
        var_tmf2_dn2 = assign31220_e46940_d_n2;

        let (assign31230_e46957, assign31230_e46957_d_n0, assign31230_e46957_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31230_e46952: f64 = (var_tmf1 * var_tmf1);
        let assign31230_e46954: f64 = (assign31230_e46952 + var_tmf2);
        let assign31230_e46955: f64 = (assign31230_e46954).sqrt();
        (assign31230_e46955, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign31230_e46955)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign31230_e46955)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign31230_e46957;
        var_tmf2_dn0 = assign31230_e46957_d_n0;
        var_tmf2_dn2 = assign31230_e46957_d_n2;

        let (assign31240_e46975, assign31240_e46975_d_n0, assign31240_e46975_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31240_e46971: f64 = (var_tmf1 / var_tmf2);
        let assign31240_e46972: f64 = (1.0 + assign31240_e46971);
        let assign31240_e46973: f64 = (0.5 * assign31240_e46972);
        (assign31240_e46973, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign31240_e46975;
        var_dfn_sl_dn0 = assign31240_e46975_d_n0;
        var_dfn_sl_dn2 = assign31240_e46975_d_n2;

        let (assign31250_e46993, assign31250_e46993_d_n0, assign31250_e46993_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31250_e46989: f64 = (var_tmf1 + var_tmf2);
        let assign31250_e46990: f64 = (0.5 * assign31250_e46989);
        let assign31250_e46991: f64 = (var_nfagat_i + assign31250_e46990);
        (assign31250_e46991, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign31250_e46993;
        var_nj1_dn0 = assign31250_e46993_d_n0;
        var_nj1_dn2 = assign31250_e46993_d_n2;

        let (assign31260_e47009, assign31260_e47009_d_n0, assign31260_e47009_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31260_e47005: f64 = (p.p85 - var_nj0);
        let assign31260_e47007: f64 = (assign31260_e47005 - 0.01);
        (assign31260_e47007, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign31260_e47009;
        var_tmf1_dn0 = assign31260_e47009_d_n0;
        var_tmf1_dn2 = assign31260_e47009_d_n2;

        let (assign31270_e47025, assign31270_e47025_d_n0, assign31270_e47025_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31270_e47021: f64 = (4.0 * p.p85);
        let assign31270_e47023: f64 = (assign31270_e47021 * 0.01);
        (assign31270_e47023, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign31270_e47025;
        var_tmf2_dn0 = assign31270_e47025_d_n0;
        var_tmf2_dn2 = assign31270_e47025_d_n2;

        let (assign31280_e47043, assign31280_e47043_d_n0, assign31280_e47043_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let (assign31280_e47041, assign31280_e47041_d_n0, assign31280_e47041_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign31280_e47040: f64 = (-var_tmf2);
                (assign31280_e47040, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign31280_e47041, assign31280_e47041_d_n0, assign31280_e47041_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign31280_e47043;
        var_tmf2_dn0 = assign31280_e47043_d_n0;
        var_tmf2_dn2 = assign31280_e47043_d_n2;

        let (assign31290_e47060, assign31290_e47060_d_n0, assign31290_e47060_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31290_e47055: f64 = (var_tmf1 * var_tmf1);
        let assign31290_e47057: f64 = (assign31290_e47055 + var_tmf2);
        let assign31290_e47058: f64 = (assign31290_e47057).sqrt();
        (assign31290_e47058, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign31290_e47058)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign31290_e47058)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign31290_e47060;
        var_tmf2_dn0 = assign31290_e47060_d_n0;
        var_tmf2_dn2 = assign31290_e47060_d_n2;

        let (assign31300_e47078, assign31300_e47078_d_n0, assign31300_e47078_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31300_e47074: f64 = (var_tmf1 + var_tmf2);
        let assign31300_e47075: f64 = (0.5 * assign31300_e47074);
        let assign31300_e47076: f64 = (p.p85 - assign31300_e47075);
        (assign31300_e47076, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign31300_e47078;
        var_nj0_dn0 = assign31300_e47078_d_n0;
        var_nj0_dn2 = assign31300_e47078_d_n2;

        let (assign31310_e47094, assign31310_e47094_d_n0, assign31310_e47094_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31310_e47090: f64 = (var_nj0 - var_nfagat_i);
        let assign31310_e47092: f64 = (assign31310_e47090 - 0.01);
        (assign31310_e47092, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign31310_e47094;
        var_tmf1_dn0 = assign31310_e47094_d_n0;
        var_tmf1_dn2 = assign31310_e47094_d_n2;

        let (assign31320_e47110, assign31320_e47110_d_n0, assign31320_e47110_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31320_e47106: f64 = (4.0 * var_nfagat_i);
        let assign31320_e47108: f64 = (assign31320_e47106 * 0.01);
        (assign31320_e47108, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign31320_e47110;
        var_tmf2_dn0 = assign31320_e47110_d_n0;
        var_tmf2_dn2 = assign31320_e47110_d_n2;

        let (assign31330_e47128, assign31330_e47128_d_n0, assign31330_e47128_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let (assign31330_e47126, assign31330_e47126_d_n0, assign31330_e47126_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign31330_e47125: f64 = (-var_tmf2);
                (assign31330_e47125, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign31330_e47126, assign31330_e47126_d_n0, assign31330_e47126_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign31330_e47128;
        var_tmf2_dn0 = assign31330_e47128_d_n0;
        var_tmf2_dn2 = assign31330_e47128_d_n2;

        let (assign31340_e47145, assign31340_e47145_d_n0, assign31340_e47145_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31340_e47140: f64 = (var_tmf1 * var_tmf1);
        let assign31340_e47142: f64 = (assign31340_e47140 + var_tmf2);
        let assign31340_e47143: f64 = (assign31340_e47142).sqrt();
        (assign31340_e47143, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign31340_e47143)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign31340_e47143)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign31340_e47145;
        var_tmf2_dn0 = assign31340_e47145_d_n0;
        var_tmf2_dn2 = assign31340_e47145_d_n2;

        let (assign31350_e47163, assign31350_e47163_d_n0, assign31350_e47163_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31350_e47159: f64 = (var_tmf1 + var_tmf2);
        let assign31350_e47160: f64 = (0.5 * assign31350_e47159);
        let assign31350_e47161: f64 = (var_nfagat_i + assign31350_e47160);
        (assign31350_e47161, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign31350_e47163;
        var_nj0_dn0 = assign31350_e47163_d_n0;
        var_nj0_dn2 = assign31350_e47163_d_n2;

        let (assign31360_e47179, assign31360_e47179_d_n0, assign31360_e47179_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 != 0.0)) {
        let assign31360_e47175: f64 = (p.p86 * var_dfn_su);
        let assign31360_e47177: f64 = (assign31360_e47175 * var_dfn_sl);
        (assign31360_e47177, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign31360_e47175 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign31360_e47179;
        var_dnj1_dv_dn0 = assign31360_e47179_d_n0;
        var_dnj1_dv_dn2 = assign31360_e47179_d_n2;

        let (assign31370_e47192, assign31370_e47192_d_n0, assign31370_e47192_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign31370_e47192;
        var_nj0_dn0 = assign31370_e47192_d_n0;
        var_nj0_dn2 = assign31370_e47192_d_n2;

        let (assign31380_e47205, assign31380_e47205_d_n0, assign31380_e47205_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign31380_e47205;
        var_nj1_dn0 = assign31380_e47205_d_n0;
        var_nj1_dn2 = assign31380_e47205_d_n2;

        let (assign31390_e47218, assign31390_e47218_d_n0, assign31390_e47218_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard498 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign31390_e47218;
        var_dnj1_dv_dn0 = assign31390_e47218_d_n0;
        var_dnj1_dv_dn2 = assign31390_e47218_d_n2;

        let (assign31450_e47471, assign31450_e47471_d_n0, assign31450_e47471_d_n2,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign31450_e47455: f64 = (var_vmax * var_dnj1_dv);
        let assign31450_e47456: f64 = (var_nj1 - assign31450_e47455);
        let assign31450_e47459: f64 = (var_nj1 * var_nj1);
        let assign31450_e47460: f64 = (assign31450_e47456 / assign31450_e47459);
        let assign31450_e47463: f64 = (var_vha1 * var_dnj1_dv);
        let assign31450_e47466: f64 = (var_nj0 * p.p85);
        let assign31450_e47467: f64 = (assign31450_e47463 / assign31450_e47466);
        let assign31450_e47468: f64 = (assign31450_e47460 + assign31450_e47467);
        let assign31450_e47469: f64 = (var_phitdinv * assign31450_e47468);
        (assign31450_e47469, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign31450_e47459 * assign31450_e47459)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_dn0 * p.p85))) / (assign31450_e47466 * assign31450_e47466)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign31450_e47459) - (assign31450_e47456 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign31450_e47459 * assign31450_e47459)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign31450_e47466) - (assign31450_e47463 * (var_nj0_dn2 * p.p85))) / (assign31450_e47466 * assign31450_e47466)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign31450_e47471;
        var_dvmax_over_phitd_dv_dn0 = assign31450_e47471_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign31450_e47471_d_n2;

        let (assign31470_e47498, assign31470_e47498_d_n0, assign31470_e47498_d_n2,) = {
    if ((var_guard471 == 0.0) && (var_guard479 != 0.0)) {
        let assign31470_e47496: f64 = (var_idmultbot - 1.0);
        (assign31470_e47496, var_idmultbot_dn0, var_idmultbot_dn2,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign31470_e47498;
        var_idmultbot_dn0 = assign31470_e47498_d_n0;
        var_idmultbot_dn2 = assign31470_e47498_d_n2;

        let (assign31580_e47681, assign31580_e47681_d_n0, assign31580_e47681_d_n2,) = {
    if ((var_guard471 == 0.0) && (var_guard479 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign31580_e47681;
        var_idmultbot_dn0 = assign31580_e47681_d_n0;
        var_idmultbot_dn2 = assign31580_e47681_d_n2;

        let assign34170_e51465: f64 = if p.p84 > 0.0 { 1.0 } else { 0.0 };
        var_guard558 = assign34170_e51465;

        let assign34180_e51468: f64 = if var_njl < p.p85 { 1.0 } else { 0.0 };
        var_guard559 = assign34180_e51468;

        let (assign34190_e51480, assign34190_e51480_d_n0, assign34190_e51480_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34190_e51475: f64 = (var_vak - var_v_hk);
        let assign34190_e51476: f64 = (p.p86 * assign34190_e51475);
        let assign34190_e51478: f64 = (assign34190_e51476 + var_njl);
        (assign34190_e51478, (p.p86 * var_vak_dn0), (p.p86 * var_vak_dn2),)
    } else {
        (var_nj_k0, var_nj_k0_dn0, var_nj_k0_dn2,)
    }
};
        var_nj_k0 = assign34190_e51480;
        var_nj_k0_dn0 = assign34190_e51480_d_n0;
        var_nj_k0_dn2 = assign34190_e51480_d_n2;

        let (assign34200_e51490, assign34200_e51490_d_n0, assign34200_e51490_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34200_e51487: f64 = (p.p86 * var_v_hk);
        let assign34200_e51488: f64 = (var_njl - assign34200_e51487);
        (assign34200_e51488, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign34200_e51490;
        var_nj0_dn0 = assign34200_e51490_d_n0;
        var_nj0_dn2 = assign34200_e51490_d_n2;

        let (assign34210_e51500, assign34210_e51500_d_n0, assign34210_e51500_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34210_e51496: f64 = (p.p85 - var_nj_k0);
        let assign34210_e51498: f64 = (assign34210_e51496 - 0.01);
        (assign34210_e51498, (-var_nj_k0_dn0), (-var_nj_k0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign34210_e51500;
        var_tmf1_dn0 = assign34210_e51500_d_n0;
        var_tmf1_dn2 = assign34210_e51500_d_n2;

        let (assign34220_e51510, assign34220_e51510_d_n0, assign34220_e51510_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34220_e51506: f64 = (4.0 * p.p85);
        let assign34220_e51508: f64 = (assign34220_e51506 * 0.01);
        (assign34220_e51508, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign34220_e51510;
        var_tmf2_dn0 = assign34220_e51510_d_n0;
        var_tmf2_dn2 = assign34220_e51510_d_n2;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_guard558_slot = var_guard558;
        *var_guard559_slot = var_guard559;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj_k0_slot = var_nj_k0;
        *var_nj_k0_dn0_slot = var_nj_k0_dn0;
        *var_nj_k0_dn2_slot = var_nj_k0_dn2;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
    }

    pub(super) fn stamp_transient_block_34(
        p: &Parameters,
        var_guard558: f64,
        var_guard559: f64,
        var_idmultbot: f64,
        var_idmultbot_dn0: f64,
        var_idmultbot_dn2: f64,
        var_njl: f64,
        var_phitdinv: f64,
        var_tkd: f64,
        var_tkr: f64,
        var_v_ha: f64,
        var_v_hk: f64,
        var_vak: f64,
        var_vak_dn0: f64,
        var_vak_dn2: f64,
        var_exp_a_slot: &mut f64,
        var_exp_a2_slot: &mut f64,
        var_exp_a2_dn0_slot: &mut f64,
        var_exp_a2_dn2_slot: &mut f64,
        var_exp_a_dn0_slot: &mut f64,
        var_exp_a_dn2_slot: &mut f64,
        var_exp_k_slot: &mut f64,
        var_exp_k_dn0_slot: &mut f64,
        var_exp_k_dn2_slot: &mut f64,
        var_guard560_slot: &mut f64,
        var_guard561_slot: &mut f64,
        var_guard562_slot: &mut f64,
        var_guard563_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj_k_slot: &mut f64,
        var_nj_k1_slot: &mut f64,
        var_nj_k1_dn0_slot: &mut f64,
        var_nj_k1_dn2_slot: &mut f64,
        var_nj_k_dn0_slot: &mut f64,
        var_nj_k_dn2_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
    ) {
        let mut var_exp_a: f64 = *var_exp_a_slot;
        let mut var_exp_a2: f64 = *var_exp_a2_slot;
        let mut var_exp_a2_dn0: f64 = *var_exp_a2_dn0_slot;
        let mut var_exp_a2_dn2: f64 = *var_exp_a2_dn2_slot;
        let mut var_exp_a_dn0: f64 = *var_exp_a_dn0_slot;
        let mut var_exp_a_dn2: f64 = *var_exp_a_dn2_slot;
        let mut var_exp_k: f64 = *var_exp_k_slot;
        let mut var_exp_k_dn0: f64 = *var_exp_k_dn0_slot;
        let mut var_exp_k_dn2: f64 = *var_exp_k_dn2_slot;
        let mut var_guard560: f64 = *var_guard560_slot;
        let mut var_guard561: f64 = *var_guard561_slot;
        let mut var_guard562: f64 = *var_guard562_slot;
        let mut var_guard563: f64 = *var_guard563_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj_k: f64 = *var_nj_k_slot;
        let mut var_nj_k1: f64 = *var_nj_k1_slot;
        let mut var_nj_k1_dn0: f64 = *var_nj_k1_dn0_slot;
        let mut var_nj_k1_dn2: f64 = *var_nj_k1_dn2_slot;
        let mut var_nj_k_dn0: f64 = *var_nj_k_dn0_slot;
        let mut var_nj_k_dn2: f64 = *var_nj_k_dn2_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;

        let (assign34230_e51522, assign34230_e51522_d_n0, assign34230_e51522_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let (assign34230_e51520, assign34230_e51520_d_n0, assign34230_e51520_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign34230_e51519: f64 = (-var_tmf2);
                (assign34230_e51519, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign34230_e51520, assign34230_e51520_d_n0, assign34230_e51520_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign34230_e51522;
        var_tmf2_dn0 = assign34230_e51522_d_n0;
        var_tmf2_dn2 = assign34230_e51522_d_n2;

        let (assign34240_e51533, assign34240_e51533_d_n0, assign34240_e51533_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34240_e51528: f64 = (var_tmf1 * var_tmf1);
        let assign34240_e51530: f64 = (assign34240_e51528 + var_tmf2);
        let assign34240_e51531: f64 = (assign34240_e51530).sqrt();
        (assign34240_e51531, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign34240_e51531)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign34240_e51531)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign34240_e51533;
        var_tmf2_dn0 = assign34240_e51533_d_n0;
        var_tmf2_dn2 = assign34240_e51533_d_n2;

        let (assign34250_e51545, assign34250_e51545_d_n0, assign34250_e51545_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34250_e51541: f64 = (var_tmf1 + var_tmf2);
        let assign34250_e51542: f64 = (0.5 * assign34250_e51541);
        let assign34250_e51543: f64 = (p.p85 - assign34250_e51542);
        (assign34250_e51543, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj_k1, var_nj_k1_dn0, var_nj_k1_dn2,)
    }
};
        var_nj_k1 = assign34250_e51545;
        var_nj_k1_dn0 = assign34250_e51545_d_n0;
        var_nj_k1_dn2 = assign34250_e51545_d_n2;

        let (assign34260_e51555, assign34260_e51555_d_n0, assign34260_e51555_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34260_e51551: f64 = (var_nj_k1 - var_njl);
        let assign34260_e51553: f64 = (assign34260_e51551 - 0.01);
        (assign34260_e51553, var_nj_k1_dn0, var_nj_k1_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign34260_e51555;
        var_tmf1_dn0 = assign34260_e51555_d_n0;
        var_tmf1_dn2 = assign34260_e51555_d_n2;

        let (assign34270_e51565, assign34270_e51565_d_n0, assign34270_e51565_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34270_e51561: f64 = (4.0 * var_njl);
        let assign34270_e51563: f64 = (assign34270_e51561 * 0.01);
        (assign34270_e51563, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign34270_e51565;
        var_tmf2_dn0 = assign34270_e51565_d_n0;
        var_tmf2_dn2 = assign34270_e51565_d_n2;

        let (assign34280_e51577, assign34280_e51577_d_n0, assign34280_e51577_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let (assign34280_e51575, assign34280_e51575_d_n0, assign34280_e51575_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign34280_e51574: f64 = (-var_tmf2);
                (assign34280_e51574, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign34280_e51575, assign34280_e51575_d_n0, assign34280_e51575_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign34280_e51577;
        var_tmf2_dn0 = assign34280_e51577_d_n0;
        var_tmf2_dn2 = assign34280_e51577_d_n2;

        let (assign34290_e51588, assign34290_e51588_d_n0, assign34290_e51588_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34290_e51583: f64 = (var_tmf1 * var_tmf1);
        let assign34290_e51585: f64 = (assign34290_e51583 + var_tmf2);
        let assign34290_e51586: f64 = (assign34290_e51585).sqrt();
        (assign34290_e51586, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign34290_e51586)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign34290_e51586)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign34290_e51588;
        var_tmf2_dn0 = assign34290_e51588_d_n0;
        var_tmf2_dn2 = assign34290_e51588_d_n2;

        let (assign34300_e51600, assign34300_e51600_d_n0, assign34300_e51600_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34300_e51596: f64 = (var_tmf1 + var_tmf2);
        let assign34300_e51597: f64 = (0.5 * assign34300_e51596);
        let assign34300_e51598: f64 = (var_njl + assign34300_e51597);
        (assign34300_e51598, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj_k, var_nj_k_dn0, var_nj_k_dn2,)
    }
};
        var_nj_k = assign34300_e51600;
        var_nj_k_dn0 = assign34300_e51600_d_n0;
        var_nj_k_dn2 = assign34300_e51600_d_n2;

        let (assign34310_e51610, assign34310_e51610_d_n0, assign34310_e51610_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34310_e51606: f64 = (p.p85 - var_nj0);
        let assign34310_e51608: f64 = (assign34310_e51606 - 0.01);
        (assign34310_e51608, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign34310_e51610;
        var_tmf1_dn0 = assign34310_e51610_d_n0;
        var_tmf1_dn2 = assign34310_e51610_d_n2;

        let (assign34320_e51620, assign34320_e51620_d_n0, assign34320_e51620_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34320_e51616: f64 = (4.0 * p.p85);
        let assign34320_e51618: f64 = (assign34320_e51616 * 0.01);
        (assign34320_e51618, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign34320_e51620;
        var_tmf2_dn0 = assign34320_e51620_d_n0;
        var_tmf2_dn2 = assign34320_e51620_d_n2;

        let (assign34330_e51632, assign34330_e51632_d_n0, assign34330_e51632_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let (assign34330_e51630, assign34330_e51630_d_n0, assign34330_e51630_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign34330_e51629: f64 = (-var_tmf2);
                (assign34330_e51629, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign34330_e51630, assign34330_e51630_d_n0, assign34330_e51630_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign34330_e51632;
        var_tmf2_dn0 = assign34330_e51632_d_n0;
        var_tmf2_dn2 = assign34330_e51632_d_n2;

        let (assign34340_e51643, assign34340_e51643_d_n0, assign34340_e51643_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34340_e51638: f64 = (var_tmf1 * var_tmf1);
        let assign34340_e51640: f64 = (assign34340_e51638 + var_tmf2);
        let assign34340_e51641: f64 = (assign34340_e51640).sqrt();
        (assign34340_e51641, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign34340_e51641)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign34340_e51641)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign34340_e51643;
        var_tmf2_dn0 = assign34340_e51643_d_n0;
        var_tmf2_dn2 = assign34340_e51643_d_n2;

        let (assign34350_e51655, assign34350_e51655_d_n0, assign34350_e51655_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34350_e51651: f64 = (var_tmf1 + var_tmf2);
        let assign34350_e51652: f64 = (0.5 * assign34350_e51651);
        let assign34350_e51653: f64 = (p.p85 - assign34350_e51652);
        (assign34350_e51653, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign34350_e51655;
        var_nj0_dn0 = assign34350_e51655_d_n0;
        var_nj0_dn2 = assign34350_e51655_d_n2;

        let (assign34360_e51665, assign34360_e51665_d_n0, assign34360_e51665_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34360_e51661: f64 = (var_nj0 - var_njl);
        let assign34360_e51663: f64 = (assign34360_e51661 - 0.01);
        (assign34360_e51663, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign34360_e51665;
        var_tmf1_dn0 = assign34360_e51665_d_n0;
        var_tmf1_dn2 = assign34360_e51665_d_n2;

        let (assign34370_e51675, assign34370_e51675_d_n0, assign34370_e51675_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34370_e51671: f64 = (4.0 * var_njl);
        let assign34370_e51673: f64 = (assign34370_e51671 * 0.01);
        (assign34370_e51673, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign34370_e51675;
        var_tmf2_dn0 = assign34370_e51675_d_n0;
        var_tmf2_dn2 = assign34370_e51675_d_n2;

        let (assign34380_e51687, assign34380_e51687_d_n0, assign34380_e51687_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let (assign34380_e51685, assign34380_e51685_d_n0, assign34380_e51685_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign34380_e51684: f64 = (-var_tmf2);
                (assign34380_e51684, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign34380_e51685, assign34380_e51685_d_n0, assign34380_e51685_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign34380_e51687;
        var_tmf2_dn0 = assign34380_e51687_d_n0;
        var_tmf2_dn2 = assign34380_e51687_d_n2;

        let (assign34390_e51698, assign34390_e51698_d_n0, assign34390_e51698_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34390_e51693: f64 = (var_tmf1 * var_tmf1);
        let assign34390_e51695: f64 = (assign34390_e51693 + var_tmf2);
        let assign34390_e51696: f64 = (assign34390_e51695).sqrt();
        (assign34390_e51696, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign34390_e51696)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign34390_e51696)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign34390_e51698;
        var_tmf2_dn0 = assign34390_e51698_d_n0;
        var_tmf2_dn2 = assign34390_e51698_d_n2;

        let (assign34400_e51710, assign34400_e51710_d_n0, assign34400_e51710_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 != 0.0)) {
        let assign34400_e51706: f64 = (var_tmf1 + var_tmf2);
        let assign34400_e51707: f64 = (0.5 * assign34400_e51706);
        let assign34400_e51708: f64 = (var_njl + assign34400_e51707);
        (assign34400_e51708, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign34400_e51710;
        var_nj0_dn0 = assign34400_e51710_d_n0;
        var_nj0_dn2 = assign34400_e51710_d_n2;

        let (assign34410_e51717, assign34410_e51717_d_n0, assign34410_e51717_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 == 0.0)) {
        (var_njl, 0.0, 0.0,)
    } else {
        (var_nj_k, var_nj_k_dn0, var_nj_k_dn2,)
    }
};
        var_nj_k = assign34410_e51717;
        var_nj_k_dn0 = assign34410_e51717_d_n0;
        var_nj_k_dn2 = assign34410_e51717_d_n2;

        let (assign34420_e51724, assign34420_e51724_d_n0, assign34420_e51724_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard559 == 0.0)) {
        (var_njl, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign34420_e51724;
        var_nj0_dn0 = assign34420_e51724_d_n0;
        var_nj0_dn2 = assign34420_e51724_d_n2;

        let (assign34430_e51728, assign34430_e51728_d_n0, assign34430_e51728_d_n2,) = {
    if (var_guard558 != 0.0) {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    } else {
        (var_exp_a, var_exp_a_dn0, var_exp_a_dn2,)
    }
};
        var_exp_a = assign34430_e51728;
        var_exp_a_dn0 = assign34430_e51728_d_n0;
        var_exp_a_dn2 = assign34430_e51728_d_n2;

        let assign34440_e51732: f64 = (var_v_hk - var_v_ha);
        let assign34440_e51733: f64 = (var_vak - assign34440_e51732);
        let assign34440_e51735: f64 = if assign34440_e51733 > 0.0 { 1.0 } else { 0.0 };
        var_guard560 = assign34440_e51735;

        let assign34450_e51739: f64 = (var_vak / var_nj_k);
        let assign34450_e51742: f64 = (var_v_hk - var_v_ha);
        let assign34450_e51744: f64 = (assign34450_e51742 / var_nj_k);
        let assign34450_e51745: f64 = (assign34450_e51739 - assign34450_e51744);
        let assign34450_e51749: f64 = (var_nj_k - var_nj0);
        let assign34450_e51750: f64 = (var_v_hk * assign34450_e51749);
        let assign34450_e51753: f64 = (var_nj0 * p.p85);
        let assign34450_e51754: f64 = (assign34450_e51750 / assign34450_e51753);
        let assign34450_e51755: f64 = (assign34450_e51745 + assign34450_e51754);
        let assign34450_e51756: f64 = (var_phitdinv * assign34450_e51755);
        let assign34450_e51757: f64 = (assign34450_e51756).abs();
        let assign34450_e51759: f64 = if assign34450_e51757 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard561 = assign34450_e51759;

        let (assign34460_e51788, assign34460_e51788_d_n0, assign34460_e51788_d_n2,) = {
    if (((var_guard558 != 0.0) && (var_guard560 != 0.0)) && (var_guard561 != 0.0)) {
        let assign34460_e51768: f64 = (var_vak / var_nj_k);
        let assign34460_e51771: f64 = (var_v_hk - var_v_ha);
        let assign34460_e51773: f64 = (assign34460_e51771 / var_nj_k);
        let assign34460_e51774: f64 = (assign34460_e51768 - assign34460_e51773);
        let assign34460_e51778: f64 = (var_nj_k - var_nj0);
        let assign34460_e51779: f64 = (var_v_hk * assign34460_e51778);
        let assign34460_e51782: f64 = (var_nj0 * p.p85);
        let assign34460_e51783: f64 = (assign34460_e51779 / assign34460_e51782);
        let assign34460_e51784: f64 = (assign34460_e51774 + assign34460_e51783);
        let assign34460_e51785: f64 = (var_phitdinv * assign34460_e51784);
        let assign34460_e51786: f64 = (assign34460_e51785).exp();
        (assign34460_e51786, (assign34460_e51786 * (var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - (-((assign34460_e51771 * var_nj_k_dn0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn0 - var_nj0_dn0)) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_dn0 * p.p85))) / (assign34460_e51782 * assign34460_e51782))))), (assign34460_e51786 * (var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - (-((assign34460_e51771 * var_nj_k_dn2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn2 - var_nj0_dn2)) * assign34460_e51782) - (assign34460_e51779 * (var_nj0_dn2 * p.p85))) / (assign34460_e51782 * assign34460_e51782))))),)
    } else {
        (var_exp_k, var_exp_k_dn0, var_exp_k_dn2,)
    }
};
        var_exp_k = assign34460_e51788;
        var_exp_k_dn0 = assign34460_e51788_d_n0;
        var_exp_k_dn2 = assign34460_e51788_d_n2;

        let assign34470_e51792: f64 = (var_vak / var_nj_k);
        let assign34470_e51795: f64 = (var_v_hk - var_v_ha);
        let assign34470_e51797: f64 = (assign34470_e51795 / var_nj_k);
        let assign34470_e51798: f64 = (assign34470_e51792 - assign34470_e51797);
        let assign34470_e51802: f64 = (var_nj_k - var_nj0);
        let assign34470_e51803: f64 = (var_v_hk * assign34470_e51802);
        let assign34470_e51806: f64 = (var_nj0 * p.p85);
        let assign34470_e51807: f64 = (assign34470_e51803 / assign34470_e51806);
        let assign34470_e51808: f64 = (assign34470_e51798 + assign34470_e51807);
        let assign34470_e51809: f64 = (var_phitdinv * assign34470_e51808);
        let assign34470_e51811: f64 = (-230.25850929940458);
        let assign34470_e51812: f64 = if assign34470_e51809 < assign34470_e51811 { 1.0 } else { 0.0 };
        var_guard562 = assign34470_e51812;

        let (assign34480_e51908, assign34480_e51908_d_n0, assign34480_e51908_d_n2,) = {
    if ((((var_guard558 != 0.0) && (var_guard560 != 0.0)) && (var_guard561 == 0.0)) && (var_guard562 != 0.0)) {
        let assign34480_e51824: f64 = (-230.25850929940458);
        let assign34480_e51828: f64 = (var_vak / var_nj_k);
        let assign34480_e51831: f64 = (var_v_hk - var_v_ha);
        let assign34480_e51833: f64 = (assign34480_e51831 / var_nj_k);
        let assign34480_e51834: f64 = (assign34480_e51828 - assign34480_e51833);
        let assign34480_e51838: f64 = (var_nj_k - var_nj0);
        let assign34480_e51839: f64 = (var_v_hk * assign34480_e51838);
        let assign34480_e51842: f64 = (var_nj0 * p.p85);
        let assign34480_e51843: f64 = (assign34480_e51839 / assign34480_e51842);
        let assign34480_e51844: f64 = (assign34480_e51834 + assign34480_e51843);
        let assign34480_e51845: f64 = (var_phitdinv * assign34480_e51844);
        let assign34480_e51846: f64 = (assign34480_e51824 - assign34480_e51845);
        let assign34480_e51850: f64 = (-230.25850929940458);
        let assign34480_e51854: f64 = (var_vak / var_nj_k);
        let assign34480_e51857: f64 = (var_v_hk - var_v_ha);
        let assign34480_e51859: f64 = (assign34480_e51857 / var_nj_k);
        let assign34480_e51860: f64 = (assign34480_e51854 - assign34480_e51859);
        let assign34480_e51864: f64 = (var_nj_k - var_nj0);
        let assign34480_e51865: f64 = (var_v_hk * assign34480_e51864);
        let assign34480_e51868: f64 = (var_nj0 * p.p85);
        let assign34480_e51869: f64 = (assign34480_e51865 / assign34480_e51868);
        let assign34480_e51870: f64 = (assign34480_e51860 + assign34480_e51869);
        let assign34480_e51871: f64 = (var_phitdinv * assign34480_e51870);
        let assign34480_e51872: f64 = (assign34480_e51850 - assign34480_e51871);
        let assign34480_e51875: f64 = (-230.25850929940458);
        let assign34480_e51879: f64 = (var_vak / var_nj_k);
        let assign34480_e51882: f64 = (var_v_hk - var_v_ha);
        let assign34480_e51884: f64 = (assign34480_e51882 / var_nj_k);
        let assign34480_e51885: f64 = (assign34480_e51879 - assign34480_e51884);
        let assign34480_e51889: f64 = (var_nj_k - var_nj0);
        let assign34480_e51890: f64 = (var_v_hk * assign34480_e51889);
        let assign34480_e51893: f64 = (var_nj0 * p.p85);
        let assign34480_e51894: f64 = (assign34480_e51890 / assign34480_e51893);
        let assign34480_e51895: f64 = (assign34480_e51885 + assign34480_e51894);
        let assign34480_e51896: f64 = (var_phitdinv * assign34480_e51895);
        let assign34480_e51897: f64 = (assign34480_e51875 - assign34480_e51896);
        let assign34480_e51899: f64 = (assign34480_e51897 * 0.3333333333333333);
        let assign34480_e51900: f64 = (1.0 + assign34480_e51899);
        let assign34480_e51901: f64 = (assign34480_e51872 * assign34480_e51900);
        let assign34480_e51902: f64 = (0.5 * assign34480_e51901);
        let assign34480_e51903: f64 = (1.0 + assign34480_e51902);
        let assign34480_e51904: f64 = (assign34480_e51846 * assign34480_e51903);
        let assign34480_e51905: f64 = (1.0 + assign34480_e51904);
        let assign34480_e51906: f64 = (1e-100 / assign34480_e51905);
        (assign34480_e51906, (-((1e-100 * (((-(var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51831 * var_nj_k_dn0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn0 - var_nj0_dn0)) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_dn0 * p.p85))) / (assign34480_e51842 * assign34480_e51842))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-(var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51857 * var_nj_k_dn0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn0 - var_nj0_dn0)) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_dn0 * p.p85))) / (assign34480_e51868 * assign34480_e51868))))) * assign34480_e51900) + (assign34480_e51872 * ((-(var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51882 * var_nj_k_dn0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn0 - var_nj0_dn0)) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_dn0 * p.p85))) / (assign34480_e51893 * assign34480_e51893))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))), (-((1e-100 * (((-(var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51831 * var_nj_k_dn2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn2 - var_nj0_dn2)) * assign34480_e51842) - (assign34480_e51839 * (var_nj0_dn2 * p.p85))) / (assign34480_e51842 * assign34480_e51842))))) * assign34480_e51903) + (assign34480_e51846 * (0.5 * (((-(var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51857 * var_nj_k_dn2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn2 - var_nj0_dn2)) * assign34480_e51868) - (assign34480_e51865 * (var_nj0_dn2 * p.p85))) / (assign34480_e51868 * assign34480_e51868))))) * assign34480_e51900) + (assign34480_e51872 * ((-(var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - (-((assign34480_e51882 * var_nj_k_dn2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn2 - var_nj0_dn2)) * assign34480_e51893) - (assign34480_e51890 * (var_nj0_dn2 * p.p85))) / (assign34480_e51893 * assign34480_e51893))))) * 0.3333333333333333))))))) / (assign34480_e51905 * assign34480_e51905))),)
    } else {
        (var_exp_k, var_exp_k_dn0, var_exp_k_dn2,)
    }
};
        var_exp_k = assign34480_e51908;
        var_exp_k_dn0 = assign34480_e51908_d_n0;
        var_exp_k_dn2 = assign34480_e51908_d_n2;

        let (assign34490_e52002, assign34490_e52002_d_n0, assign34490_e52002_d_n2,) = {
    if ((((var_guard558 != 0.0) && (var_guard560 != 0.0)) && (var_guard561 == 0.0)) && (var_guard562 == 0.0)) {
        let assign34490_e51923: f64 = (var_vak / var_nj_k);
        let assign34490_e51926: f64 = (var_v_hk - var_v_ha);
        let assign34490_e51928: f64 = (assign34490_e51926 / var_nj_k);
        let assign34490_e51929: f64 = (assign34490_e51923 - assign34490_e51928);
        let assign34490_e51933: f64 = (var_nj_k - var_nj0);
        let assign34490_e51934: f64 = (var_v_hk * assign34490_e51933);
        let assign34490_e51937: f64 = (var_nj0 * p.p85);
        let assign34490_e51938: f64 = (assign34490_e51934 / assign34490_e51937);
        let assign34490_e51939: f64 = (assign34490_e51929 + assign34490_e51938);
        let assign34490_e51940: f64 = (var_phitdinv * assign34490_e51939);
        let assign34490_e51942: f64 = (assign34490_e51940 - 230.25850929940458);
        let assign34490_e51948: f64 = (var_vak / var_nj_k);
        let assign34490_e51951: f64 = (var_v_hk - var_v_ha);
        let assign34490_e51953: f64 = (assign34490_e51951 / var_nj_k);
        let assign34490_e51954: f64 = (assign34490_e51948 - assign34490_e51953);
        let assign34490_e51958: f64 = (var_nj_k - var_nj0);
        let assign34490_e51959: f64 = (var_v_hk * assign34490_e51958);
        let assign34490_e51962: f64 = (var_nj0 * p.p85);
        let assign34490_e51963: f64 = (assign34490_e51959 / assign34490_e51962);
        let assign34490_e51964: f64 = (assign34490_e51954 + assign34490_e51963);
        let assign34490_e51965: f64 = (var_phitdinv * assign34490_e51964);
        let assign34490_e51967: f64 = (assign34490_e51965 - 230.25850929940458);
        let assign34490_e51972: f64 = (var_vak / var_nj_k);
        let assign34490_e51975: f64 = (var_v_hk - var_v_ha);
        let assign34490_e51977: f64 = (assign34490_e51975 / var_nj_k);
        let assign34490_e51978: f64 = (assign34490_e51972 - assign34490_e51977);
        let assign34490_e51982: f64 = (var_nj_k - var_nj0);
        let assign34490_e51983: f64 = (var_v_hk * assign34490_e51982);
        let assign34490_e51986: f64 = (var_nj0 * p.p85);
        let assign34490_e51987: f64 = (assign34490_e51983 / assign34490_e51986);
        let assign34490_e51988: f64 = (assign34490_e51978 + assign34490_e51987);
        let assign34490_e51989: f64 = (var_phitdinv * assign34490_e51988);
        let assign34490_e51991: f64 = (assign34490_e51989 - 230.25850929940458);
        let assign34490_e51993: f64 = (assign34490_e51991 * 0.3333333333333333);
        let assign34490_e51994: f64 = (1.0 + assign34490_e51993);
        let assign34490_e51995: f64 = (assign34490_e51967 * assign34490_e51994);
        let assign34490_e51996: f64 = (0.5 * assign34490_e51995);
        let assign34490_e51997: f64 = (1.0 + assign34490_e51996);
        let assign34490_e51998: f64 = (assign34490_e51942 * assign34490_e51997);
        let assign34490_e51999: f64 = (1.0 + assign34490_e51998);
        let assign34490_e52000: f64 = (1e100 * assign34490_e51999);
        (assign34490_e52000, (1e100 * (((var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51926 * var_nj_k_dn0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn0 - var_nj0_dn0)) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_dn0 * p.p85))) / (assign34490_e51937 * assign34490_e51937)))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * (((var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51951 * var_nj_k_dn0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn0 - var_nj0_dn0)) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_dn0 * p.p85))) / (assign34490_e51962 * assign34490_e51962)))) * assign34490_e51994) + (assign34490_e51967 * ((var_phitdinv * (((((var_vak_dn0 * var_nj_k) - (var_vak * var_nj_k_dn0)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51975 * var_nj_k_dn0) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn0 - var_nj0_dn0)) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_dn0 * p.p85))) / (assign34490_e51986 * assign34490_e51986)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51926 * var_nj_k_dn2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn2 - var_nj0_dn2)) * assign34490_e51937) - (assign34490_e51934 * (var_nj0_dn2 * p.p85))) / (assign34490_e51937 * assign34490_e51937)))) * assign34490_e51997) + (assign34490_e51942 * (0.5 * (((var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51951 * var_nj_k_dn2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn2 - var_nj0_dn2)) * assign34490_e51962) - (assign34490_e51959 * (var_nj0_dn2 * p.p85))) / (assign34490_e51962 * assign34490_e51962)))) * assign34490_e51994) + (assign34490_e51967 * ((var_phitdinv * (((((var_vak_dn2 * var_nj_k) - (var_vak * var_nj_k_dn2)) / (var_nj_k * var_nj_k)) - (-((assign34490_e51975 * var_nj_k_dn2) / (var_nj_k * var_nj_k)))) + ((((var_v_hk * (var_nj_k_dn2 - var_nj0_dn2)) * assign34490_e51986) - (assign34490_e51983 * (var_nj0_dn2 * p.p85))) / (assign34490_e51986 * assign34490_e51986)))) * 0.3333333333333333))))))),)
    } else {
        (var_exp_k, var_exp_k_dn0, var_exp_k_dn2,)
    }
};
        var_exp_k = assign34490_e52002;
        var_exp_k_dn0 = assign34490_e52002_d_n0;
        var_exp_k_dn2 = assign34490_e52002_d_n2;

        let (assign34500_e52009, assign34500_e52009_d_n0, assign34500_e52009_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard560 == 0.0)) {
        (1.0, 0.0, 0.0,)
    } else {
        (var_exp_k, var_exp_k_dn0, var_exp_k_dn2,)
    }
};
        var_exp_k = assign34500_e52009;
        var_exp_k_dn0 = assign34500_e52009_d_n0;
        var_exp_k_dn2 = assign34500_e52009_d_n2;

        let assign34510_e52016: f64 = if ((p.p91 == 0.0) || (var_vak < var_v_ha)) { 1.0 } else { 0.0 };
        var_guard563 = assign34510_e52016;

        let (assign34520_e52024, assign34520_e52024_d_n0, assign34520_e52024_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard563 != 0.0)) {
        let assign34520_e52022: f64 = (var_exp_a * p.p90);
        (assign34520_e52022, (var_exp_a_dn0 * p.p90), (var_exp_a_dn2 * p.p90),)
    } else {
        (var_exp_a2, var_exp_a2_dn0, var_exp_a2_dn2,)
    }
};
        var_exp_a2 = assign34520_e52024;
        var_exp_a2_dn0 = assign34520_e52024_d_n0;
        var_exp_a2_dn2 = assign34520_e52024_d_n2;

        let (assign34530_e52053, assign34530_e52053_d_n0, assign34530_e52053_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard563 == 0.0)) {
        let assign34530_e52031: f64 = (var_exp_a * p.p90);
        let assign34530_e52033: f64 = (-p.p91);
        let assign34530_e52036: f64 = (var_vak - var_v_ha);
        let assign34530_e52037: f64 = (assign34530_e52033 * assign34530_e52036);
        let assign34530_e52040: f64 = (var_vak - var_v_ha);
        let assign34530_e52041: f64 = (assign34530_e52037 * assign34530_e52040);
        let assign34530_e52045: f64 = (var_tkr / var_tkd);
        let assign34530_e52046: f64 = (assign34530_e52045).ln();
        let assign34530_e52047: f64 = (p.p98 * assign34530_e52046);
        let assign34530_e52048: f64 = (assign34530_e52047).exp();
        let assign34530_e52049: f64 = (assign34530_e52041 * assign34530_e52048);
        let assign34530_e52050: f64 = (assign34530_e52049).exp();
        let assign34530_e52051: f64 = (assign34530_e52031 * assign34530_e52050);
        (assign34530_e52051, (((var_exp_a_dn0 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * ((((assign34530_e52033 * var_vak_dn0) * assign34530_e52040) + (assign34530_e52037 * var_vak_dn0)) * assign34530_e52048)))), (((var_exp_a_dn2 * p.p90) * assign34530_e52050) + (assign34530_e52031 * (assign34530_e52050 * ((((assign34530_e52033 * var_vak_dn2) * assign34530_e52040) + (assign34530_e52037 * var_vak_dn2)) * assign34530_e52048)))),)
    } else {
        (var_exp_a2, var_exp_a2_dn0, var_exp_a2_dn2,)
    }
};
        var_exp_a2 = assign34530_e52053;
        var_exp_a2_dn0 = assign34530_e52053_d_n0;
        var_exp_a2_dn2 = assign34530_e52053_d_n2;

        *var_exp_a_slot = var_exp_a;
        *var_exp_a2_slot = var_exp_a2;
        *var_exp_a2_dn0_slot = var_exp_a2_dn0;
        *var_exp_a2_dn2_slot = var_exp_a2_dn2;
        *var_exp_a_dn0_slot = var_exp_a_dn0;
        *var_exp_a_dn2_slot = var_exp_a_dn2;
        *var_exp_k_slot = var_exp_k;
        *var_exp_k_dn0_slot = var_exp_k_dn0;
        *var_exp_k_dn2_slot = var_exp_k_dn2;
        *var_guard560_slot = var_guard560;
        *var_guard561_slot = var_guard561;
        *var_guard562_slot = var_guard562;
        *var_guard563_slot = var_guard563;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj_k_slot = var_nj_k;
        *var_nj_k1_slot = var_nj_k1;
        *var_nj_k1_dn0_slot = var_nj_k1_dn0;
        *var_nj_k1_dn2_slot = var_nj_k1_dn2;
        *var_nj_k_dn0_slot = var_nj_k_dn0;
        *var_nj_k_dn2_slot = var_nj_k_dn2;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
    }

    pub(super) fn stamp_transient_block_35(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_ab_i: f64,
        var_epssi: f64,
        var_exp_k: f64,
        var_exp_k_dn0: f64,
        var_exp_k_dn2: f64,
        var_guard558: f64,
        var_juncdlt: f64,
        var_ndi_i: f64,
        var_pb: f64,
        var_pn0: f64,
        var_q_pex0: f64,
        var_tkd: f64,
        var_tkr: f64,
        var_v_hk: f64,
        var_vak: f64,
        var_vak_dn0: f64,
        var_vak_dn2: f64,
        var_w_depa0: f64,
        var_w_depa0_dn0: f64,
        var_w_depa0_dn2: f64,
        var_exp_a2_slot: &mut f64,
        var_exp_a2_dn0_slot: &mut f64,
        var_exp_a2_dn2_slot: &mut f64,
        var_exp_k2_slot: &mut f64,
        var_exp_k2_dn0_slot: &mut f64,
        var_exp_k2_dn2_slot: &mut f64,
        var_guard564_slot: &mut f64,
        var_guard565_slot: &mut f64,
        var_guard566_slot: &mut f64,
        var_guard567_slot: &mut f64,
        var_guard568_slot: &mut f64,
        var_guard571_slot: &mut f64,
        var_guard572_slot: &mut f64,
        var_inqs0_a_slot: &mut f64,
        var_inqs0_a_dn0_slot: &mut f64,
        var_inqs0_a_dn2_slot: &mut f64,
        var_inqs0_a_dn3_slot: &mut f64,
        var_inqs0_k_slot: &mut f64,
        var_inqs0_k_dn0_slot: &mut f64,
        var_inqs0_k_dn2_slot: &mut f64,
        var_inqs0_k_dn4_slot: &mut f64,
        var_iwnqs0_a_slot: &mut f64,
        var_iwnqs0_a_dn0_slot: &mut f64,
        var_iwnqs0_a_dn2_slot: &mut f64,
        var_iwnqs0_a_dn5_slot: &mut f64,
        var_p_na_slot: &mut f64,
        var_p_na_dn0_slot: &mut f64,
        var_p_na_dn2_slot: &mut f64,
        var_p_nk_slot: &mut f64,
        var_p_nk_dn0_slot: &mut f64,
        var_p_nk_dn2_slot: &mut f64,
        var_q_nqs_a_slot: &mut f64,
        var_q_nqs_a_dn3_slot: &mut f64,
        var_q_nqs_k_slot: &mut f64,
        var_q_nqs_k_dn4_slot: &mut f64,
        var_q_pexa_slot: &mut f64,
        var_q_pexa_dn0_slot: &mut f64,
        var_q_pexa_dn2_slot: &mut f64,
        var_q_pexk_slot: &mut f64,
        var_q_pexk_dn0_slot: &mut f64,
        var_q_pexk_dn2_slot: &mut f64,
        var_q_qs_a_slot: &mut f64,
        var_q_qs_a_dn0_slot: &mut f64,
        var_q_qs_a_dn2_slot: &mut f64,
        var_q_qs_k_slot: &mut f64,
        var_q_qs_k_dn0_slot: &mut f64,
        var_q_qs_k_dn2_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_vjunc_a_slot: &mut f64,
        var_vjunc_a_dn0_slot: &mut f64,
        var_vjunc_a_dn2_slot: &mut f64,
        var_w_depa_slot: &mut f64,
        var_w_depa_dn0_slot: &mut f64,
        var_w_depa_dn2_slot: &mut f64,
        var_w_nqs_a_slot: &mut f64,
        var_w_nqs_a_dn5_slot: &mut f64,
        var_w_qs_a_slot: &mut f64,
        var_w_qs_a_dn0_slot: &mut f64,
        var_w_qs_a_dn2_slot: &mut f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let mut var_exp_a2: f64 = *var_exp_a2_slot;
        let mut var_exp_a2_dn0: f64 = *var_exp_a2_dn0_slot;
        let mut var_exp_a2_dn2: f64 = *var_exp_a2_dn2_slot;
        let mut var_exp_k2: f64 = *var_exp_k2_slot;
        let mut var_exp_k2_dn0: f64 = *var_exp_k2_dn0_slot;
        let mut var_exp_k2_dn2: f64 = *var_exp_k2_dn2_slot;
        let mut var_guard564: f64 = *var_guard564_slot;
        let mut var_guard565: f64 = *var_guard565_slot;
        let mut var_guard566: f64 = *var_guard566_slot;
        let mut var_guard567: f64 = *var_guard567_slot;
        let mut var_guard568: f64 = *var_guard568_slot;
        let mut var_guard571: f64 = *var_guard571_slot;
        let mut var_guard572: f64 = *var_guard572_slot;
        let mut var_inqs0_a: f64 = *var_inqs0_a_slot;
        let mut var_inqs0_a_dn0: f64 = *var_inqs0_a_dn0_slot;
        let mut var_inqs0_a_dn2: f64 = *var_inqs0_a_dn2_slot;
        let mut var_inqs0_a_dn3: f64 = *var_inqs0_a_dn3_slot;
        let mut var_inqs0_k: f64 = *var_inqs0_k_slot;
        let mut var_inqs0_k_dn0: f64 = *var_inqs0_k_dn0_slot;
        let mut var_inqs0_k_dn2: f64 = *var_inqs0_k_dn2_slot;
        let mut var_inqs0_k_dn4: f64 = *var_inqs0_k_dn4_slot;
        let mut var_iwnqs0_a: f64 = *var_iwnqs0_a_slot;
        let mut var_iwnqs0_a_dn0: f64 = *var_iwnqs0_a_dn0_slot;
        let mut var_iwnqs0_a_dn2: f64 = *var_iwnqs0_a_dn2_slot;
        let mut var_iwnqs0_a_dn5: f64 = *var_iwnqs0_a_dn5_slot;
        let mut var_p_na: f64 = *var_p_na_slot;
        let mut var_p_na_dn0: f64 = *var_p_na_dn0_slot;
        let mut var_p_na_dn2: f64 = *var_p_na_dn2_slot;
        let mut var_p_nk: f64 = *var_p_nk_slot;
        let mut var_p_nk_dn0: f64 = *var_p_nk_dn0_slot;
        let mut var_p_nk_dn2: f64 = *var_p_nk_dn2_slot;
        let mut var_q_nqs_a: f64 = *var_q_nqs_a_slot;
        let mut var_q_nqs_a_dn3: f64 = *var_q_nqs_a_dn3_slot;
        let mut var_q_nqs_k: f64 = *var_q_nqs_k_slot;
        let mut var_q_nqs_k_dn4: f64 = *var_q_nqs_k_dn4_slot;
        let mut var_q_pexa: f64 = *var_q_pexa_slot;
        let mut var_q_pexa_dn0: f64 = *var_q_pexa_dn0_slot;
        let mut var_q_pexa_dn2: f64 = *var_q_pexa_dn2_slot;
        let mut var_q_pexk: f64 = *var_q_pexk_slot;
        let mut var_q_pexk_dn0: f64 = *var_q_pexk_dn0_slot;
        let mut var_q_pexk_dn2: f64 = *var_q_pexk_dn2_slot;
        let mut var_q_qs_a: f64 = *var_q_qs_a_slot;
        let mut var_q_qs_a_dn0: f64 = *var_q_qs_a_dn0_slot;
        let mut var_q_qs_a_dn2: f64 = *var_q_qs_a_dn2_slot;
        let mut var_q_qs_k: f64 = *var_q_qs_k_slot;
        let mut var_q_qs_k_dn0: f64 = *var_q_qs_k_dn0_slot;
        let mut var_q_qs_k_dn2: f64 = *var_q_qs_k_dn2_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_vjunc_a: f64 = *var_vjunc_a_slot;
        let mut var_vjunc_a_dn0: f64 = *var_vjunc_a_dn0_slot;
        let mut var_vjunc_a_dn2: f64 = *var_vjunc_a_dn2_slot;
        let mut var_w_depa: f64 = *var_w_depa_slot;
        let mut var_w_depa_dn0: f64 = *var_w_depa_dn0_slot;
        let mut var_w_depa_dn2: f64 = *var_w_depa_dn2_slot;
        let mut var_w_nqs_a: f64 = *var_w_nqs_a_slot;
        let mut var_w_nqs_a_dn5: f64 = *var_w_nqs_a_dn5_slot;
        let mut var_w_qs_a: f64 = *var_w_qs_a_slot;
        let mut var_w_qs_a_dn0: f64 = *var_w_qs_a_dn0_slot;
        let mut var_w_qs_a_dn2: f64 = *var_w_qs_a_dn2_slot;

        let (assign34540_e52062, assign34540_e52062_d_n0, assign34540_e52062_d_n2,) = {
    if (var_guard558 != 0.0) {
        let (assign34540_e52060, assign34540_e52060_d_n0, assign34540_e52060_d_n2,) = {
            if (var_exp_a2 > p.p79) {
                (p.p79, 0.0, 0.0,)
            } else {
                (var_exp_a2, var_exp_a2_dn0, var_exp_a2_dn2,)
            }
        };
        (assign34540_e52060, assign34540_e52060_d_n0, assign34540_e52060_d_n2,)
    } else {
        (var_exp_a2, var_exp_a2_dn0, var_exp_a2_dn2,)
    }
};
        var_exp_a2 = assign34540_e52062;
        var_exp_a2_dn0 = assign34540_e52062_d_n0;
        var_exp_a2_dn2 = assign34540_e52062_d_n2;

        let (assign34550_e52068, assign34550_e52068_d_n0, assign34550_e52068_d_n2,) = {
    if (var_guard558 != 0.0) {
        let assign34550_e52066: f64 = (var_pn0 * var_exp_a2);
        (assign34550_e52066, (var_pn0 * var_exp_a2_dn0), (var_pn0 * var_exp_a2_dn2),)
    } else {
        (var_p_na, var_p_na_dn0, var_p_na_dn2,)
    }
};
        var_p_na = assign34550_e52068;
        var_p_na_dn0 = assign34550_e52068_d_n0;
        var_p_na_dn2 = assign34550_e52068_d_n2;

        let (assign34560_e52078, assign34560_e52078_d_n0, assign34560_e52078_d_n2,) = {
    if (var_guard558 != 0.0) {
        let assign34560_e52072: f64 = (1.6021918e-19 * var_ab_i);
        let assign34560_e52075: f64 = (var_p_na - var_pn0);
        let assign34560_e52076: f64 = (assign34560_e52072 * assign34560_e52075);
        (assign34560_e52076, (assign34560_e52072 * var_p_na_dn0), (assign34560_e52072 * var_p_na_dn2),)
    } else {
        (var_q_pexa, var_q_pexa_dn0, var_q_pexa_dn2,)
    }
};
        var_q_pexa = assign34560_e52078;
        var_q_pexa_dn0 = assign34560_e52078_d_n0;
        var_q_pexa_dn2 = assign34560_e52078_d_n2;

        let assign34570_e52081: f64 = if p.p92 > 0.0 { 1.0 } else { 0.0 };
        var_guard564 = assign34570_e52081;

        let (assign34580_e52091, assign34580_e52091_d_n0, assign34580_e52091_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard564 != 0.0)) {
        let assign34580_e52088: f64 = (1e-23 / var_q_pex0);
        let assign34580_e52089: f64 = (var_q_pexa * assign34580_e52088);
        (assign34580_e52089, (var_q_pexa_dn0 * assign34580_e52088), (var_q_pexa_dn2 * assign34580_e52088),)
    } else {
        (var_q_qs_a, var_q_qs_a_dn0, var_q_qs_a_dn2,)
    }
};
        var_q_qs_a = assign34580_e52091;
        var_q_qs_a_dn0 = assign34580_e52091_d_n0;
        var_q_qs_a_dn2 = assign34580_e52091_d_n2;

        let (assign34590_e52099, assign34590_e52099_d_n3,) = {
    if ((var_guard558 != 0.0) && (var_guard564 != 0.0)) {
        let assign34590_e52097: f64 = (nv3 - 0.0);
        (assign34590_e52097, 1.0,)
    } else {
        (var_q_nqs_a, var_q_nqs_a_dn3,)
    }
};
        var_q_nqs_a = assign34590_e52099;
        var_q_nqs_a_dn3 = assign34590_e52099_d_n3;

        let (assign34600_e52109, assign34600_e52109_d_n0, assign34600_e52109_d_n2, assign34600_e52109_d_n3,) = {
    if ((var_guard558 != 0.0) && (var_guard564 != 0.0)) {
        let assign34600_e52105: f64 = (var_q_nqs_a - var_q_qs_a);
        let assign34600_e52107: f64 = (assign34600_e52105 / p.p92);
        (assign34600_e52107, ((-var_q_qs_a_dn0) / p.p92), ((-var_q_qs_a_dn2) / p.p92), (var_q_nqs_a_dn3 / p.p92),)
    } else {
        (var_inqs0_a, var_inqs0_a_dn0, var_inqs0_a_dn2, var_inqs0_a_dn3,)
    }
};
        var_inqs0_a = assign34600_e52109;
        var_inqs0_a_dn0 = assign34600_e52109_d_n0;
        var_inqs0_a_dn2 = assign34600_e52109_d_n2;
        var_inqs0_a_dn3 = assign34600_e52109_d_n3;

        let (assign34620_e52126, assign34620_e52126_d_n0, assign34620_e52126_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard564 == 0.0)) {
        (var_q_pexa, var_q_pexa_dn0, var_q_pexa_dn2,)
    } else {
        (var_q_qs_a, var_q_qs_a_dn0, var_q_qs_a_dn2,)
    }
};
        var_q_qs_a = assign34620_e52126;
        var_q_qs_a_dn0 = assign34620_e52126_d_n0;
        var_q_qs_a_dn2 = assign34620_e52126_d_n2;

        let assign34640_e52140: f64 = if ((p.p91 == 0.0) || (var_vak < var_v_hk)) { 1.0 } else { 0.0 };
        var_guard565 = assign34640_e52140;

        let (assign34650_e52148, assign34650_e52148_d_n0, assign34650_e52148_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard565 != 0.0)) {
        let assign34650_e52146: f64 = (var_exp_k * p.p90);
        (assign34650_e52146, (var_exp_k_dn0 * p.p90), (var_exp_k_dn2 * p.p90),)
    } else {
        (var_exp_k2, var_exp_k2_dn0, var_exp_k2_dn2,)
    }
};
        var_exp_k2 = assign34650_e52148;
        var_exp_k2_dn0 = assign34650_e52148_d_n0;
        var_exp_k2_dn2 = assign34650_e52148_d_n2;

        let (assign34660_e52177, assign34660_e52177_d_n0, assign34660_e52177_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard565 == 0.0)) {
        let assign34660_e52155: f64 = (var_exp_k * p.p90);
        let assign34660_e52157: f64 = (-p.p91);
        let assign34660_e52160: f64 = (var_vak - var_v_hk);
        let assign34660_e52161: f64 = (assign34660_e52157 * assign34660_e52160);
        let assign34660_e52164: f64 = (var_vak - var_v_hk);
        let assign34660_e52165: f64 = (assign34660_e52161 * assign34660_e52164);
        let assign34660_e52169: f64 = (var_tkr / var_tkd);
        let assign34660_e52170: f64 = (assign34660_e52169).ln();
        let assign34660_e52171: f64 = (p.p98 * assign34660_e52170);
        let assign34660_e52172: f64 = (assign34660_e52171).exp();
        let assign34660_e52173: f64 = (assign34660_e52165 * assign34660_e52172);
        let assign34660_e52174: f64 = (assign34660_e52173).exp();
        let assign34660_e52175: f64 = (assign34660_e52155 * assign34660_e52174);
        (assign34660_e52175, (((var_exp_k_dn0 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * ((((assign34660_e52157 * var_vak_dn0) * assign34660_e52164) + (assign34660_e52161 * var_vak_dn0)) * assign34660_e52172)))), (((var_exp_k_dn2 * p.p90) * assign34660_e52174) + (assign34660_e52155 * (assign34660_e52174 * ((((assign34660_e52157 * var_vak_dn2) * assign34660_e52164) + (assign34660_e52161 * var_vak_dn2)) * assign34660_e52172)))),)
    } else {
        (var_exp_k2, var_exp_k2_dn0, var_exp_k2_dn2,)
    }
};
        var_exp_k2 = assign34660_e52177;
        var_exp_k2_dn0 = assign34660_e52177_d_n0;
        var_exp_k2_dn2 = assign34660_e52177_d_n2;

        let (assign34670_e52186, assign34670_e52186_d_n0, assign34670_e52186_d_n2,) = {
    if (var_guard558 != 0.0) {
        let (assign34670_e52184, assign34670_e52184_d_n0, assign34670_e52184_d_n2,) = {
            if (var_exp_k2 > p.p79) {
                (p.p79, 0.0, 0.0,)
            } else {
                (var_exp_k2, var_exp_k2_dn0, var_exp_k2_dn2,)
            }
        };
        (assign34670_e52184, assign34670_e52184_d_n0, assign34670_e52184_d_n2,)
    } else {
        (var_exp_k2, var_exp_k2_dn0, var_exp_k2_dn2,)
    }
};
        var_exp_k2 = assign34670_e52186;
        var_exp_k2_dn0 = assign34670_e52186_d_n0;
        var_exp_k2_dn2 = assign34670_e52186_d_n2;

        let (assign34680_e52192, assign34680_e52192_d_n0, assign34680_e52192_d_n2,) = {
    if (var_guard558 != 0.0) {
        let assign34680_e52190: f64 = (var_pn0 * var_exp_k2);
        (assign34680_e52190, (var_pn0 * var_exp_k2_dn0), (var_pn0 * var_exp_k2_dn2),)
    } else {
        (var_p_nk, var_p_nk_dn0, var_p_nk_dn2,)
    }
};
        var_p_nk = assign34680_e52192;
        var_p_nk_dn0 = assign34680_e52192_d_n0;
        var_p_nk_dn2 = assign34680_e52192_d_n2;

        let (assign34690_e52202, assign34690_e52202_d_n0, assign34690_e52202_d_n2,) = {
    if (var_guard558 != 0.0) {
        let assign34690_e52196: f64 = (1.6021918e-19 * var_ab_i);
        let assign34690_e52199: f64 = (var_p_nk - var_pn0);
        let assign34690_e52200: f64 = (assign34690_e52196 * assign34690_e52199);
        (assign34690_e52200, (assign34690_e52196 * var_p_nk_dn0), (assign34690_e52196 * var_p_nk_dn2),)
    } else {
        (var_q_pexk, var_q_pexk_dn0, var_q_pexk_dn2,)
    }
};
        var_q_pexk = assign34690_e52202;
        var_q_pexk_dn0 = assign34690_e52202_d_n0;
        var_q_pexk_dn2 = assign34690_e52202_d_n2;

        let assign34700_e52205: f64 = if p.p92 > 0.0 { 1.0 } else { 0.0 };
        var_guard566 = assign34700_e52205;

        let (assign34710_e52215, assign34710_e52215_d_n0, assign34710_e52215_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard566 != 0.0)) {
        let assign34710_e52212: f64 = (1e-23 / var_q_pex0);
        let assign34710_e52213: f64 = (var_q_pexk * assign34710_e52212);
        (assign34710_e52213, (var_q_pexk_dn0 * assign34710_e52212), (var_q_pexk_dn2 * assign34710_e52212),)
    } else {
        (var_q_qs_k, var_q_qs_k_dn0, var_q_qs_k_dn2,)
    }
};
        var_q_qs_k = assign34710_e52215;
        var_q_qs_k_dn0 = assign34710_e52215_d_n0;
        var_q_qs_k_dn2 = assign34710_e52215_d_n2;

        let (assign34720_e52223, assign34720_e52223_d_n4,) = {
    if ((var_guard558 != 0.0) && (var_guard566 != 0.0)) {
        let assign34720_e52221: f64 = (nv4 - 0.0);
        (assign34720_e52221, 1.0,)
    } else {
        (var_q_nqs_k, var_q_nqs_k_dn4,)
    }
};
        var_q_nqs_k = assign34720_e52223;
        var_q_nqs_k_dn4 = assign34720_e52223_d_n4;

        let (assign34730_e52233, assign34730_e52233_d_n0, assign34730_e52233_d_n2, assign34730_e52233_d_n4,) = {
    if ((var_guard558 != 0.0) && (var_guard566 != 0.0)) {
        let assign34730_e52229: f64 = (var_q_nqs_k - var_q_qs_k);
        let assign34730_e52231: f64 = (assign34730_e52229 / p.p92);
        (assign34730_e52231, ((-var_q_qs_k_dn0) / p.p92), ((-var_q_qs_k_dn2) / p.p92), (var_q_nqs_k_dn4 / p.p92),)
    } else {
        (var_inqs0_k, var_inqs0_k_dn0, var_inqs0_k_dn2, var_inqs0_k_dn4,)
    }
};
        var_inqs0_k = assign34730_e52233;
        var_inqs0_k_dn0 = assign34730_e52233_d_n0;
        var_inqs0_k_dn2 = assign34730_e52233_d_n2;
        var_inqs0_k_dn4 = assign34730_e52233_d_n4;

        let (assign34750_e52250, assign34750_e52250_d_n0, assign34750_e52250_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard566 == 0.0)) {
        (var_q_pexk, var_q_pexk_dn0, var_q_pexk_dn2,)
    } else {
        (var_q_qs_k, var_q_qs_k_dn0, var_q_qs_k_dn2,)
    }
};
        var_q_qs_k = assign34750_e52250;
        var_q_qs_k_dn0 = assign34750_e52250_d_n0;
        var_q_qs_k_dn2 = assign34750_e52250_d_n2;

        let (assign34770_e52263, assign34770_e52263_d_n0, assign34770_e52263_d_n2,) = {
    if (var_guard558 != 0.0) {
        let assign34770_e52261: f64 = (var_pb - var_vak);
        (assign34770_e52261, (-var_vak_dn0), (-var_vak_dn2),)
    } else {
        (var_vjunc_a, var_vjunc_a_dn0, var_vjunc_a_dn2,)
    }
};
        var_vjunc_a = assign34770_e52263;
        var_vjunc_a_dn0 = assign34770_e52263_d_n0;
        var_vjunc_a_dn2 = assign34770_e52263_d_n2;

        let (assign34780_e52276, assign34780_e52276_d_n0, assign34780_e52276_d_n2,) = {
    if (var_guard558 != 0.0) {
        let assign34780_e52267: f64 = (var_vjunc_a * var_vjunc_a);
        let assign34780_e52270: f64 = (4.0 * var_juncdlt);
        let assign34780_e52272: f64 = (assign34780_e52270 * var_juncdlt);
        let assign34780_e52273: f64 = (assign34780_e52267 + assign34780_e52272);
        let assign34780_e52274: f64 = (assign34780_e52273).sqrt();
        (assign34780_e52274, (((var_vjunc_a_dn0 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_dn0)) / (2.0 * assign34780_e52274)), (((var_vjunc_a_dn2 * var_vjunc_a) + (var_vjunc_a * var_vjunc_a_dn2)) / (2.0 * assign34780_e52274)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign34780_e52276;
        var_tmf2_dn0 = assign34780_e52276_d_n0;
        var_tmf2_dn2 = assign34780_e52276_d_n2;

        let (assign34790_e52284, assign34790_e52284_d_n0, assign34790_e52284_d_n2,) = {
    if (var_guard558 != 0.0) {
        let assign34790_e52281: f64 = (var_vjunc_a + var_tmf2);
        let assign34790_e52282: f64 = (0.5 * assign34790_e52281);
        (assign34790_e52282, (0.5 * (var_vjunc_a_dn0 + var_tmf2_dn0)), (0.5 * (var_vjunc_a_dn2 + var_tmf2_dn2)),)
    } else {
        (var_vjunc_a, var_vjunc_a_dn0, var_vjunc_a_dn2,)
    }
};
        var_vjunc_a = assign34790_e52284;
        var_vjunc_a_dn0 = assign34790_e52284_d_n0;
        var_vjunc_a_dn2 = assign34790_e52284_d_n2;

        let assign34800_e52287: f64 = if var_vjunc_a < 0.0 { 1.0 } else { 0.0 };
        var_guard567 = assign34800_e52287;

        let (assign34810_e52293, assign34810_e52293_d_n0, assign34810_e52293_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard567 != 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_vjunc_a, var_vjunc_a_dn0, var_vjunc_a_dn2,)
    }
};
        var_vjunc_a = assign34810_e52293;
        var_vjunc_a_dn0 = assign34810_e52293_d_n0;
        var_vjunc_a_dn2 = assign34810_e52293_d_n2;

        let (assign34820_e52306, assign34820_e52306_d_n0, assign34820_e52306_d_n2,) = {
    if (var_guard558 != 0.0) {
        let assign34820_e52297: f64 = (2.0 * var_epssi);
        let assign34820_e52299: f64 = (assign34820_e52297 * var_vjunc_a);
        let assign34820_e52302: f64 = (1.6021918e-19 * var_ndi_i);
        let assign34820_e52303: f64 = (assign34820_e52299 / assign34820_e52302);
        let assign34820_e52304: f64 = (assign34820_e52303).sqrt();
        (assign34820_e52304, (((assign34820_e52297 * var_vjunc_a_dn0) / assign34820_e52302) / (2.0 * assign34820_e52304)), (((assign34820_e52297 * var_vjunc_a_dn2) / assign34820_e52302) / (2.0 * assign34820_e52304)),)
    } else {
        (var_w_depa, var_w_depa_dn0, var_w_depa_dn2,)
    }
};
        var_w_depa = assign34820_e52306;
        var_w_depa_dn0 = assign34820_e52306_d_n0;
        var_w_depa_dn2 = assign34820_e52306_d_n2;

        let (assign34830_e52314, assign34830_e52314_d_n0, assign34830_e52314_d_n2,) = {
    if (var_guard558 != 0.0) {
        let assign34830_e52310: f64 = (p.p94 - var_w_depa);
        let assign34830_e52312: f64 = (assign34830_e52310 - 1e-7);
        (assign34830_e52312, (-var_w_depa_dn0), (-var_w_depa_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign34830_e52314;
        var_tmf1_dn0 = assign34830_e52314_d_n0;
        var_tmf1_dn2 = assign34830_e52314_d_n2;

        let (assign34840_e52322, assign34840_e52322_d_n0, assign34840_e52322_d_n2,) = {
    if (var_guard558 != 0.0) {
        let assign34840_e52318: f64 = (4.0 * p.p94);
        let assign34840_e52320: f64 = (assign34840_e52318 * 1e-7);
        (assign34840_e52320, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign34840_e52322;
        var_tmf2_dn0 = assign34840_e52322_d_n0;
        var_tmf2_dn2 = assign34840_e52322_d_n2;

        let (assign34850_e52332, assign34850_e52332_d_n0, assign34850_e52332_d_n2,) = {
    if (var_guard558 != 0.0) {
        let (assign34850_e52330, assign34850_e52330_d_n0, assign34850_e52330_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign34850_e52329: f64 = (-var_tmf2);
                (assign34850_e52329, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign34850_e52330, assign34850_e52330_d_n0, assign34850_e52330_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign34850_e52332;
        var_tmf2_dn0 = assign34850_e52332_d_n0;
        var_tmf2_dn2 = assign34850_e52332_d_n2;

        let (assign34860_e52341, assign34860_e52341_d_n0, assign34860_e52341_d_n2,) = {
    if (var_guard558 != 0.0) {
        let assign34860_e52336: f64 = (var_tmf1 * var_tmf1);
        let assign34860_e52338: f64 = (assign34860_e52336 + var_tmf2);
        let assign34860_e52339: f64 = (assign34860_e52338).sqrt();
        (assign34860_e52339, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign34860_e52339)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign34860_e52339)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign34860_e52341;
        var_tmf2_dn0 = assign34860_e52341_d_n0;
        var_tmf2_dn2 = assign34860_e52341_d_n2;

        let (assign34870_e52351, assign34870_e52351_d_n0, assign34870_e52351_d_n2,) = {
    if (var_guard558 != 0.0) {
        let assign34870_e52347: f64 = (var_tmf1 + var_tmf2);
        let assign34870_e52348: f64 = (0.5 * assign34870_e52347);
        let assign34870_e52349: f64 = (p.p94 - assign34870_e52348);
        (assign34870_e52349, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_w_depa, var_w_depa_dn0, var_w_depa_dn2,)
    }
};
        var_w_depa = assign34870_e52351;
        var_w_depa_dn0 = assign34870_e52351_d_n0;
        var_w_depa_dn2 = assign34870_e52351_d_n2;

        let assign34880_e52354: f64 = if p.p95 > 0.0 { 1.0 } else { 0.0 };
        var_guard568 = assign34880_e52354;

        let (assign34890_e52364, assign34890_e52364_d_n0, assign34890_e52364_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard568 != 0.0)) {
        let assign34890_e52361: f64 = (1.0 / var_w_depa0);
        let assign34890_e52362: f64 = (var_w_depa * assign34890_e52361);
        (assign34890_e52362, ((var_w_depa_dn0 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_dn0 / (var_w_depa0 * var_w_depa0))))), ((var_w_depa_dn2 * assign34890_e52361) + (var_w_depa * (-(var_w_depa0_dn2 / (var_w_depa0 * var_w_depa0))))),)
    } else {
        (var_w_qs_a, var_w_qs_a_dn0, var_w_qs_a_dn2,)
    }
};
        var_w_qs_a = assign34890_e52364;
        var_w_qs_a_dn0 = assign34890_e52364_d_n0;
        var_w_qs_a_dn2 = assign34890_e52364_d_n2;

        let (assign34900_e52372, assign34900_e52372_d_n5,) = {
    if ((var_guard558 != 0.0) && (var_guard568 != 0.0)) {
        let assign34900_e52370: f64 = (nv5 - 0.0);
        (assign34900_e52370, 1.0,)
    } else {
        (var_w_nqs_a, var_w_nqs_a_dn5,)
    }
};
        var_w_nqs_a = assign34900_e52372;
        var_w_nqs_a_dn5 = assign34900_e52372_d_n5;

        let (assign34910_e52382, assign34910_e52382_d_n0, assign34910_e52382_d_n2, assign34910_e52382_d_n5,) = {
    if ((var_guard558 != 0.0) && (var_guard568 != 0.0)) {
        let assign34910_e52378: f64 = (var_w_nqs_a - var_w_qs_a);
        let assign34910_e52380: f64 = (assign34910_e52378 / p.p95);
        (assign34910_e52380, ((-var_w_qs_a_dn0) / p.p95), ((-var_w_qs_a_dn2) / p.p95), (var_w_nqs_a_dn5 / p.p95),)
    } else {
        (var_iwnqs0_a, var_iwnqs0_a_dn0, var_iwnqs0_a_dn2, var_iwnqs0_a_dn5,)
    }
};
        var_iwnqs0_a = assign34910_e52382;
        var_iwnqs0_a_dn0 = assign34910_e52382_d_n0;
        var_iwnqs0_a_dn2 = assign34910_e52382_d_n2;
        var_iwnqs0_a_dn5 = assign34910_e52382_d_n5;

        let (assign34930_e52399, assign34930_e52399_d_n0, assign34930_e52399_d_n2,) = {
    if ((var_guard558 != 0.0) && (var_guard568 == 0.0)) {
        (var_w_depa, var_w_depa_dn0, var_w_depa_dn2,)
    } else {
        (var_w_qs_a, var_w_qs_a_dn0, var_w_qs_a_dn2,)
    }
};
        var_w_qs_a = assign34930_e52399;
        var_w_qs_a_dn0 = assign34930_e52399_d_n0;
        var_w_qs_a_dn2 = assign34930_e52399_d_n2;

        let assign35080_e52535: f64 = if ((p.p84 > 0.0) && (p.p92 > 0.0)) { 1.0 } else { 0.0 };
        var_guard571 = assign35080_e52535;

        let assign35090_e52542: f64 = if ((p.p84 > 0.0) && (p.p95 > 0.0)) { 1.0 } else { 0.0 };
        var_guard572 = assign35090_e52542;

        *var_exp_a2_slot = var_exp_a2;
        *var_exp_a2_dn0_slot = var_exp_a2_dn0;
        *var_exp_a2_dn2_slot = var_exp_a2_dn2;
        *var_exp_k2_slot = var_exp_k2;
        *var_exp_k2_dn0_slot = var_exp_k2_dn0;
        *var_exp_k2_dn2_slot = var_exp_k2_dn2;
        *var_guard564_slot = var_guard564;
        *var_guard565_slot = var_guard565;
        *var_guard566_slot = var_guard566;
        *var_guard567_slot = var_guard567;
        *var_guard568_slot = var_guard568;
        *var_guard571_slot = var_guard571;
        *var_guard572_slot = var_guard572;
        *var_inqs0_a_slot = var_inqs0_a;
        *var_inqs0_a_dn0_slot = var_inqs0_a_dn0;
        *var_inqs0_a_dn2_slot = var_inqs0_a_dn2;
        *var_inqs0_a_dn3_slot = var_inqs0_a_dn3;
        *var_inqs0_k_slot = var_inqs0_k;
        *var_inqs0_k_dn0_slot = var_inqs0_k_dn0;
        *var_inqs0_k_dn2_slot = var_inqs0_k_dn2;
        *var_inqs0_k_dn4_slot = var_inqs0_k_dn4;
        *var_iwnqs0_a_slot = var_iwnqs0_a;
        *var_iwnqs0_a_dn0_slot = var_iwnqs0_a_dn0;
        *var_iwnqs0_a_dn2_slot = var_iwnqs0_a_dn2;
        *var_iwnqs0_a_dn5_slot = var_iwnqs0_a_dn5;
        *var_p_na_slot = var_p_na;
        *var_p_na_dn0_slot = var_p_na_dn0;
        *var_p_na_dn2_slot = var_p_na_dn2;
        *var_p_nk_slot = var_p_nk;
        *var_p_nk_dn0_slot = var_p_nk_dn0;
        *var_p_nk_dn2_slot = var_p_nk_dn2;
        *var_q_nqs_a_slot = var_q_nqs_a;
        *var_q_nqs_a_dn3_slot = var_q_nqs_a_dn3;
        *var_q_nqs_k_slot = var_q_nqs_k;
        *var_q_nqs_k_dn4_slot = var_q_nqs_k_dn4;
        *var_q_pexa_slot = var_q_pexa;
        *var_q_pexa_dn0_slot = var_q_pexa_dn0;
        *var_q_pexa_dn2_slot = var_q_pexa_dn2;
        *var_q_pexk_slot = var_q_pexk;
        *var_q_pexk_dn0_slot = var_q_pexk_dn0;
        *var_q_pexk_dn2_slot = var_q_pexk_dn2;
        *var_q_qs_a_slot = var_q_qs_a;
        *var_q_qs_a_dn0_slot = var_q_qs_a_dn0;
        *var_q_qs_a_dn2_slot = var_q_qs_a_dn2;
        *var_q_qs_k_slot = var_q_qs_k;
        *var_q_qs_k_dn0_slot = var_q_qs_k_dn0;
        *var_q_qs_k_dn2_slot = var_q_qs_k_dn2;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_vjunc_a_slot = var_vjunc_a;
        *var_vjunc_a_dn0_slot = var_vjunc_a_dn0;
        *var_vjunc_a_dn2_slot = var_vjunc_a_dn2;
        *var_w_depa_slot = var_w_depa;
        *var_w_depa_dn0_slot = var_w_depa_dn0;
        *var_w_depa_dn2_slot = var_w_depa_dn2;
        *var_w_nqs_a_slot = var_w_nqs_a;
        *var_w_nqs_a_dn5_slot = var_w_nqs_a_dn5;
        *var_w_qs_a_slot = var_w_qs_a;
        *var_w_qs_a_dn0_slot = var_w_qs_a_dn0;
        *var_w_qs_a_dn2_slot = var_w_qs_a_dn2;
    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_ab_i_slot: &mut f64,
        var_ab_i_rv_slot: &mut f64,
        var_auxt_slot: &mut f64,
        var_auxt_rv_slot: &mut f64,
        var_da_slot: &mut f64,
        var_da_rv_slot: &mut f64,
        var_deltaphigd_slot: &mut f64,
        var_deltaphigd_rv_slot: &mut f64,
        var_deltaphigr_slot: &mut f64,
        var_deltaphigr_rv_slot: &mut f64,
        var_dn_slot: &mut f64,
        var_dn_rv_slot: &mut f64,
        var_dp_slot: &mut f64,
        var_dp_rv_slot: &mut f64,
        var_epssi_slot: &mut f64,
        var_epssi_rv_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rv_slot: &mut f64,
        var_ftdbot2_slot: &mut f64,
        var_ftdbot2_rv_slot: &mut f64,
        var_ftdgat2_slot: &mut f64,
        var_ftdgat2_rv_slot: &mut f64,
        var_ftdsti2_slot: &mut f64,
        var_ftdsti2_rv_slot: &mut f64,
        var_guard1_slot: &mut f64,
        var_guard1_rv_slot: &mut f64,
        var_guard2_slot: &mut f64,
        var_guard2_rv_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_guard6_rv_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_guard7_rv_slot: &mut f64,
        var_idsatbot_slot: &mut f64,
        var_idsatbot_rv_slot: &mut f64,
        var_idsatgat_slot: &mut f64,
        var_idsatgat_rv_slot: &mut f64,
        var_idsatrbot_i_slot: &mut f64,
        var_idsatrbot_i_rv_slot: &mut f64,
        var_idsatrgat_i_slot: &mut f64,
        var_idsatrgat_i_rv_slot: &mut f64,
        var_idsatrsti_i_slot: &mut f64,
        var_idsatrsti_i_rv_slot: &mut f64,
        var_idsatsti_slot: &mut f64,
        var_idsatsti_rv_slot: &mut f64,
        var_imax_i_slot: &mut f64,
        var_imax_i_rv_slot: &mut f64,
        var_juncdlt_slot: &mut f64,
        var_juncdlt_rv_slot: &mut f64,
        var_kbol_over_qele_slot: &mut f64,
        var_kbol_over_qele_rv_slot: &mut f64,
        var_la_slot: &mut f64,
        var_la_rv_slot: &mut f64,
        var_lg_i_slot: &mut f64,
        var_lg_i_rv_slot: &mut f64,
        var_ls_i_slot: &mut f64,
        var_ls_i_rv_slot: &mut f64,
        var_muen_i_slot: &mut f64,
        var_muen_i_rv_slot: &mut f64,
        var_muep_i_slot: &mut f64,
        var_muep_i_rv_slot: &mut f64,
        var_ndi_i_slot: &mut f64,
        var_ndi_i_rv_slot: &mut f64,
        var_ndibot_i_slot: &mut f64,
        var_ndibot_i_rv_slot: &mut f64,
        var_ndigat_i_slot: &mut f64,
        var_ndigat_i_rv_slot: &mut f64,
        var_ndisti_i_slot: &mut f64,
        var_ndisti_i_rv_slot: &mut f64,
        var_nfabot_i_slot: &mut f64,
        var_nfabot_i_rv_slot: &mut f64,
        var_nfagat_i_slot: &mut f64,
        var_nfagat_i_rv_slot: &mut f64,
        var_nfasti_i_slot: &mut f64,
        var_nfasti_i_rv_slot: &mut f64,
        var_nin_slot: &mut f64,
        var_nin_rv_slot: &mut f64,
        var_njl_slot: &mut f64,
        var_njl_rv_slot: &mut f64,
        var_pb_slot: &mut f64,
        var_pb_rv_slot: &mut f64,
        var_phigbot_i_slot: &mut f64,
        var_phigbot_i_rv_slot: &mut f64,
        var_phigdbot_slot: &mut f64,
        var_phigdbot_rv_slot: &mut f64,
        var_phigdgat_slot: &mut f64,
        var_phigdgat_rv_slot: &mut f64,
        var_phigdsti_slot: &mut f64,
        var_phigdsti_rv_slot: &mut f64,
        var_phiggat_i_slot: &mut f64,
        var_phiggat_i_rv_slot: &mut f64,
        var_phigrbot_slot: &mut f64,
        var_phigrbot_rv_slot: &mut f64,
        var_phigrgat_slot: &mut f64,
        var_phigrgat_rv_slot: &mut f64,
        var_phigrsti_slot: &mut f64,
        var_phigrsti_rv_slot: &mut f64,
        var_phigsti_i_slot: &mut f64,
        var_phigsti_i_rv_slot: &mut f64,
        var_phitd_slot: &mut f64,
        var_phitd_rv_slot: &mut f64,
        var_phitdinv_slot: &mut f64,
        var_phitdinv_rv_slot: &mut f64,
        var_phitr_slot: &mut f64,
        var_phitr_rv_slot: &mut f64,
        var_phitrinv_slot: &mut f64,
        var_phitrinv_rv_slot: &mut f64,
        var_pn0_slot: &mut f64,
        var_pn0_rv_slot: &mut f64,
        var_scale_i_slot: &mut f64,
        var_scale_i_rv_slot: &mut f64,
        var_shrink_i_slot: &mut f64,
        var_shrink_i_rv_slot: &mut f64,
        var_shrinkl_slot: &mut f64,
        var_shrinkl_rv_slot: &mut f64,
        var_swjunexp_i_slot: &mut f64,
        var_swjunexp_i_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_tau_hl_slot: &mut f64,
        var_tau_hl_rv_slot: &mut f64,
        var_tkd_slot: &mut f64,
        var_tkd_rv_slot: &mut f64,
        var_tkr_slot: &mut f64,
        var_tkr_rv_slot: &mut f64,
        var_trj_i_slot: &mut f64,
        var_trj_i_rv_slot: &mut f64,
        var_v_ha_slot: &mut f64,
        var_v_ha_rv_slot: &mut f64,
        var_v_hk_slot: &mut f64,
        var_v_hk_rv_slot: &mut f64,
        var_vjunref_i_slot: &mut f64,
        var_vjunref_i_rv_slot: &mut f64,
        var_vmaxbot_slot: &mut f64,
        var_vmaxbot_rv_slot: &mut f64,
        var_vmaxsti_slot: &mut f64,
        var_vmaxsti_rv_slot: &mut f64,
        var_xti_i_slot: &mut f64,
        var_xti_i_rv_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let mut var_ab_i: f64 = *var_ab_i_slot;
        let mut var_ab_i_rv: f64 = *var_ab_i_rv_slot;
        let mut var_auxt: f64 = *var_auxt_slot;
        let mut var_auxt_rv: f64 = *var_auxt_rv_slot;
        let mut var_da: f64 = *var_da_slot;
        let mut var_da_rv: f64 = *var_da_rv_slot;
        let mut var_deltaphigd: f64 = *var_deltaphigd_slot;
        let mut var_deltaphigd_rv: f64 = *var_deltaphigd_rv_slot;
        let mut var_deltaphigr: f64 = *var_deltaphigr_slot;
        let mut var_deltaphigr_rv: f64 = *var_deltaphigr_rv_slot;
        let mut var_dn: f64 = *var_dn_slot;
        let mut var_dn_rv: f64 = *var_dn_rv_slot;
        let mut var_dp: f64 = *var_dp_slot;
        let mut var_dp_rv: f64 = *var_dp_rv_slot;
        let mut var_epssi: f64 = *var_epssi_slot;
        let mut var_epssi_rv: f64 = *var_epssi_rv_slot;
        let mut var_exp_vmax_over_phitd_bot: f64 = *var_exp_vmax_over_phitd_bot_slot;
        let mut var_exp_vmax_over_phitd_bot_dn0: f64 = *var_exp_vmax_over_phitd_bot_dn0_slot;
        let mut var_exp_vmax_over_phitd_bot_dn2: f64 = *var_exp_vmax_over_phitd_bot_dn2_slot;
        let mut var_exp_vmax_over_phitd_bot_rv: f64 = *var_exp_vmax_over_phitd_bot_rv_slot;
        let mut var_ftdbot2: f64 = *var_ftdbot2_slot;
        let mut var_ftdbot2_rv: f64 = *var_ftdbot2_rv_slot;
        let mut var_ftdgat2: f64 = *var_ftdgat2_slot;
        let mut var_ftdgat2_rv: f64 = *var_ftdgat2_rv_slot;
        let mut var_ftdsti2: f64 = *var_ftdsti2_slot;
        let mut var_ftdsti2_rv: f64 = *var_ftdsti2_rv_slot;
        let mut var_guard1: f64 = *var_guard1_slot;
        let mut var_guard1_rv: f64 = *var_guard1_rv_slot;
        let mut var_guard2: f64 = *var_guard2_slot;
        let mut var_guard2_rv: f64 = *var_guard2_rv_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_guard6_rv: f64 = *var_guard6_rv_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_guard7_rv: f64 = *var_guard7_rv_slot;
        let mut var_idsatbot: f64 = *var_idsatbot_slot;
        let mut var_idsatbot_rv: f64 = *var_idsatbot_rv_slot;
        let mut var_idsatgat: f64 = *var_idsatgat_slot;
        let mut var_idsatgat_rv: f64 = *var_idsatgat_rv_slot;
        let mut var_idsatrbot_i: f64 = *var_idsatrbot_i_slot;
        let mut var_idsatrbot_i_rv: f64 = *var_idsatrbot_i_rv_slot;
        let mut var_idsatrgat_i: f64 = *var_idsatrgat_i_slot;
        let mut var_idsatrgat_i_rv: f64 = *var_idsatrgat_i_rv_slot;
        let mut var_idsatrsti_i: f64 = *var_idsatrsti_i_slot;
        let mut var_idsatrsti_i_rv: f64 = *var_idsatrsti_i_rv_slot;
        let mut var_idsatsti: f64 = *var_idsatsti_slot;
        let mut var_idsatsti_rv: f64 = *var_idsatsti_rv_slot;
        let mut var_imax_i: f64 = *var_imax_i_slot;
        let mut var_imax_i_rv: f64 = *var_imax_i_rv_slot;
        let mut var_juncdlt: f64 = *var_juncdlt_slot;
        let mut var_juncdlt_rv: f64 = *var_juncdlt_rv_slot;
        let mut var_kbol_over_qele: f64 = *var_kbol_over_qele_slot;
        let mut var_kbol_over_qele_rv: f64 = *var_kbol_over_qele_rv_slot;
        let mut var_la: f64 = *var_la_slot;
        let mut var_la_rv: f64 = *var_la_rv_slot;
        let mut var_lg_i: f64 = *var_lg_i_slot;
        let mut var_lg_i_rv: f64 = *var_lg_i_rv_slot;
        let mut var_ls_i: f64 = *var_ls_i_slot;
        let mut var_ls_i_rv: f64 = *var_ls_i_rv_slot;
        let mut var_muen_i: f64 = *var_muen_i_slot;
        let mut var_muen_i_rv: f64 = *var_muen_i_rv_slot;
        let mut var_muep_i: f64 = *var_muep_i_slot;
        let mut var_muep_i_rv: f64 = *var_muep_i_rv_slot;
        let mut var_ndi_i: f64 = *var_ndi_i_slot;
        let mut var_ndi_i_rv: f64 = *var_ndi_i_rv_slot;
        let mut var_ndibot_i: f64 = *var_ndibot_i_slot;
        let mut var_ndibot_i_rv: f64 = *var_ndibot_i_rv_slot;
        let mut var_ndigat_i: f64 = *var_ndigat_i_slot;
        let mut var_ndigat_i_rv: f64 = *var_ndigat_i_rv_slot;
        let mut var_ndisti_i: f64 = *var_ndisti_i_slot;
        let mut var_ndisti_i_rv: f64 = *var_ndisti_i_rv_slot;
        let mut var_nfabot_i: f64 = *var_nfabot_i_slot;
        let mut var_nfabot_i_rv: f64 = *var_nfabot_i_rv_slot;
        let mut var_nfagat_i: f64 = *var_nfagat_i_slot;
        let mut var_nfagat_i_rv: f64 = *var_nfagat_i_rv_slot;
        let mut var_nfasti_i: f64 = *var_nfasti_i_slot;
        let mut var_nfasti_i_rv: f64 = *var_nfasti_i_rv_slot;
        let mut var_nin: f64 = *var_nin_slot;
        let mut var_nin_rv: f64 = *var_nin_rv_slot;
        let mut var_njl: f64 = *var_njl_slot;
        let mut var_njl_rv: f64 = *var_njl_rv_slot;
        let mut var_pb: f64 = *var_pb_slot;
        let mut var_pb_rv: f64 = *var_pb_rv_slot;
        let mut var_phigbot_i: f64 = *var_phigbot_i_slot;
        let mut var_phigbot_i_rv: f64 = *var_phigbot_i_rv_slot;
        let mut var_phigdbot: f64 = *var_phigdbot_slot;
        let mut var_phigdbot_rv: f64 = *var_phigdbot_rv_slot;
        let mut var_phigdgat: f64 = *var_phigdgat_slot;
        let mut var_phigdgat_rv: f64 = *var_phigdgat_rv_slot;
        let mut var_phigdsti: f64 = *var_phigdsti_slot;
        let mut var_phigdsti_rv: f64 = *var_phigdsti_rv_slot;
        let mut var_phiggat_i: f64 = *var_phiggat_i_slot;
        let mut var_phiggat_i_rv: f64 = *var_phiggat_i_rv_slot;
        let mut var_phigrbot: f64 = *var_phigrbot_slot;
        let mut var_phigrbot_rv: f64 = *var_phigrbot_rv_slot;
        let mut var_phigrgat: f64 = *var_phigrgat_slot;
        let mut var_phigrgat_rv: f64 = *var_phigrgat_rv_slot;
        let mut var_phigrsti: f64 = *var_phigrsti_slot;
        let mut var_phigrsti_rv: f64 = *var_phigrsti_rv_slot;
        let mut var_phigsti_i: f64 = *var_phigsti_i_slot;
        let mut var_phigsti_i_rv: f64 = *var_phigsti_i_rv_slot;
        let mut var_phitd: f64 = *var_phitd_slot;
        let mut var_phitd_rv: f64 = *var_phitd_rv_slot;
        let mut var_phitdinv: f64 = *var_phitdinv_slot;
        let mut var_phitdinv_rv: f64 = *var_phitdinv_rv_slot;
        let mut var_phitr: f64 = *var_phitr_slot;
        let mut var_phitr_rv: f64 = *var_phitr_rv_slot;
        let mut var_phitrinv: f64 = *var_phitrinv_slot;
        let mut var_phitrinv_rv: f64 = *var_phitrinv_rv_slot;
        let mut var_pn0: f64 = *var_pn0_slot;
        let mut var_pn0_rv: f64 = *var_pn0_rv_slot;
        let mut var_scale_i: f64 = *var_scale_i_slot;
        let mut var_scale_i_rv: f64 = *var_scale_i_rv_slot;
        let mut var_shrink_i: f64 = *var_shrink_i_slot;
        let mut var_shrink_i_rv: f64 = *var_shrink_i_rv_slot;
        let mut var_shrinkl: f64 = *var_shrinkl_slot;
        let mut var_shrinkl_rv: f64 = *var_shrinkl_rv_slot;
        let mut var_swjunexp_i: f64 = *var_swjunexp_i_slot;
        let mut var_swjunexp_i_rv: f64 = *var_swjunexp_i_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_tau_hl: f64 = *var_tau_hl_slot;
        let mut var_tau_hl_rv: f64 = *var_tau_hl_rv_slot;
        let mut var_tkd: f64 = *var_tkd_slot;
        let mut var_tkd_rv: f64 = *var_tkd_rv_slot;
        let mut var_tkr: f64 = *var_tkr_slot;
        let mut var_tkr_rv: f64 = *var_tkr_rv_slot;
        let mut var_trj_i: f64 = *var_trj_i_slot;
        let mut var_trj_i_rv: f64 = *var_trj_i_rv_slot;
        let mut var_v_ha: f64 = *var_v_ha_slot;
        let mut var_v_ha_rv: f64 = *var_v_ha_rv_slot;
        let mut var_v_hk: f64 = *var_v_hk_slot;
        let mut var_v_hk_rv: f64 = *var_v_hk_rv_slot;
        let mut var_vjunref_i: f64 = *var_vjunref_i_slot;
        let mut var_vjunref_i_rv: f64 = *var_vjunref_i_rv_slot;
        let mut var_vmaxbot: f64 = *var_vmaxbot_slot;
        let mut var_vmaxbot_rv: f64 = *var_vmaxbot_rv_slot;
        let mut var_vmaxsti: f64 = *var_vmaxsti_slot;
        let mut var_vmaxsti_rv: f64 = *var_vmaxsti_rv_slot;
        let mut var_xti_i: f64 = *var_xti_i_slot;
        let mut var_xti_i_rv: f64 = *var_xti_i_rv_slot;

        let assign00_e188: f64 = (8.8541878176e-12 * 11.8);
        var_epssi = assign00_e188;
        var_epssi_rv = 0.0;

        let assign10_e191: f64 = (-250.0);
        let (assign10_e196,) = {
    if (p.p6 > assign10_e191) {
        (p.p6,)
    } else {
        let assign10_e195: f64 = (-250.0);
        (assign10_e195,)
    }
};
        var_trj_i = assign10_e196;
        var_trj_i_rv = 0.0;

        let assign20_e202: f64 = if ((!param_given[6]) && param_given[96]) { 1.0 } else { 0.0 };
        var_guard1 = assign20_e202;
        var_guard1_rv = 0.0;

        let (assign30_e213,) = {
    if (var_guard1 != 0.0) {
        let assign30_e206: f64 = (-250.0);
        let (assign30_e211,) = {
            if (p.p96 > assign30_e206) {
                (p.p96,)
            } else {
                let assign30_e210: f64 = (-250.0);
                (assign30_e210,)
            }
        };
        (assign30_e211,)
    } else {
        (var_trj_i,)
    }
};
        var_trj_i = assign30_e213;
        var_trj_i_rv = 0.0;

        let (assign40_e219,) = {
    if (p.p5 > 1e-12) {
        (p.p5,)
    } else {
        (1e-12,)
    }
};
        var_imax_i = assign40_e219;
        var_imax_i_rv = 0.0;

        var_phigbot_i = p.p17;
        var_phigbot_i_rv = 0.0;

        var_phigsti_i = p.p18;
        var_phigsti_i_rv = 0.0;

        var_phiggat_i = p.p19;
        var_phiggat_i_rv = 0.0;

        let (assign170_e297,) = {
    if (p.p20 > 0.0) {
        (p.p20,)
    } else {
        (0.0,)
    }
};
        var_idsatrbot_i = assign170_e297;
        var_idsatrbot_i_rv = 0.0;

        let (assign180_e303,) = {
    if (p.p21 > 0.0) {
        (p.p21,)
    } else {
        (0.0,)
    }
};
        var_idsatrsti_i = assign180_e303;
        var_idsatrsti_i_rv = 0.0;

        let (assign190_e309,) = {
    if (p.p22 > 0.0) {
        (p.p22,)
    } else {
        (0.0,)
    }
};
        var_idsatrgat_i = assign190_e309;
        var_idsatrgat_i_rv = 0.0;

        let (assign610_e496,) = {
    if (p.p63 > 0.1) {
        (p.p63,)
    } else {
        (0.1,)
    }
};
        var_nfabot_i = assign610_e496;
        var_nfabot_i_rv = 0.0;

        let (assign620_e502,) = {
    if (p.p64 > 0.1) {
        (p.p64,)
    } else {
        (0.1,)
    }
};
        var_nfasti_i = assign620_e502;
        var_nfasti_i_rv = 0.0;

        let (assign630_e508,) = {
    if (p.p65 > 0.1) {
        (p.p65,)
    } else {
        (0.1,)
    }
};
        var_nfagat_i = assign630_e508;
        var_nfagat_i_rv = 0.0;

        let (assign740_e578,) = {
    if (p.p76 > 0.1) {
        (p.p76,)
    } else {
        (0.1,)
    }
};
        var_xti_i = assign740_e578;
        var_xti_i_rv = 0.0;

        let (assign750_e584,) = {
    if (p.p77 > 0.0) {
        (p.p77,)
    } else {
        (0.0,)
    }
};
        var_scale_i = assign750_e584;
        var_scale_i_rv = 0.0;

        let (assign760_e590,) = {
    if (p.p78 > 0.0) {
        (p.p78,)
    } else {
        (0.0,)
    }
};
        var_shrink_i = assign760_e590;
        var_shrink_i_rv = 0.0;

        var_swjunexp_i = 0.0;
        var_swjunexp_i_rv = 0.0;

        let assign780_e594: f64 = if p.p81 > 0.5 { 1.0 } else { 0.0 };
        var_guard2 = assign780_e594;
        var_guard2_rv = 0.0;

        let (assign790_e598,) = {
    if (var_guard2 != 0.0) {
        (1.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign790_e598;
        var_swjunexp_i_rv = 0.0;

        let (assign800_e603,) = {
    if (var_guard2 == 0.0) {
        (0.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign800_e603;
        var_swjunexp_i_rv = 0.0;

        let (assign810_e609,) = {
    if (p.p82 > 0.5) {
        (p.p82,)
    } else {
        (0.5,)
    }
};
        var_vjunref_i = assign810_e609;
        var_vjunref_i_rv = 0.0;

        let assign830_e618: f64 = (273.15 + var_trj_i);
        var_tkr = assign830_e618;
        var_tkr_rv = 0.0;

        let assign840_e619: f64 = ctx_temp;
        let assign840_e621: f64 = (assign840_e619 + p.p102);
        let assign840_e624: f64 = (-250.0);
        let assign840_e625: f64 = (273.15 + assign840_e624);
        let assign840_e626: f64 = (assign840_e621).max(assign840_e625);
        var_tkd = assign840_e626;
        var_tkd_rv = 0.0;

        let assign850_e629: f64 = (var_tkd / var_tkr);
        var_auxt = assign850_e629;
        var_auxt_rv = 0.0;

        let assign860_e632: f64 = (1.3806505e-23 / 1.6021918e-19);
        var_kbol_over_qele = assign860_e632;
        var_kbol_over_qele_rv = 0.0;

        let assign870_e635: f64 = (var_kbol_over_qele * var_tkr);
        var_phitr = assign870_e635;
        var_phitr_rv = 0.0;

        let assign880_e638: f64 = (1.0 / var_phitr);
        var_phitrinv = assign880_e638;
        var_phitrinv_rv = 0.0;

        let assign890_e641: f64 = (var_kbol_over_qele * var_tkd);
        var_phitd = assign890_e641;
        var_phitd_rv = 0.0;

        let assign900_e644: f64 = (1.0 / var_phitd);
        var_phitdinv = assign900_e644;
        var_phitdinv_rv = 0.0;

        let assign910_e647: f64 = (0.000702 * var_tkr);
        let assign910_e649: f64 = (assign910_e647 * var_tkr);
        let assign910_e650: f64 = (-assign910_e649);
        let assign910_e653: f64 = (1108.0 + var_tkr);
        let assign910_e654: f64 = (assign910_e650 / assign910_e653);
        var_deltaphigr = assign910_e654;
        var_deltaphigr_rv = 0.0;

        let assign920_e657: f64 = (var_phigbot_i + var_deltaphigr);
        var_phigrbot = assign920_e657;
        var_phigrbot_rv = 0.0;

        let assign930_e660: f64 = (var_phigsti_i + var_deltaphigr);
        var_phigrsti = assign930_e660;
        var_phigrsti_rv = 0.0;

        let assign940_e663: f64 = (var_phiggat_i + var_deltaphigr);
        var_phigrgat = assign940_e663;
        var_phigrgat_rv = 0.0;

        let assign950_e666: f64 = (0.000702 * var_tkd);
        let assign950_e668: f64 = (assign950_e666 * var_tkd);
        let assign950_e669: f64 = (-assign950_e668);
        let assign950_e672: f64 = (1108.0 + var_tkd);
        let assign950_e673: f64 = (assign950_e669 / assign950_e672);
        var_deltaphigd = assign950_e673;
        var_deltaphigd_rv = 0.0;

        let assign960_e676: f64 = (var_phigbot_i + var_deltaphigd);
        var_phigdbot = assign960_e676;
        var_phigdbot_rv = 0.0;

        let assign970_e679: f64 = (var_phigsti_i + var_deltaphigd);
        var_phigdsti = assign970_e679;
        var_phigdsti_rv = 0.0;

        let assign980_e682: f64 = (var_phiggat_i + var_deltaphigd);
        var_phigdgat = assign980_e682;
        var_phigdgat_rv = 0.0;

        let assign1020_e734: f64 = (var_xti_i / 2.0);
        let assign1020_e736: f64 = (assign1020_e734 / var_nfabot_i);
        let assign1020_e737: f64 = (var_auxt).powf(assign1020_e736);
        let assign1020_e741: f64 = (var_phigrbot * var_phitrinv);
        let assign1020_e744: f64 = (var_phigdbot * var_phitdinv);
        let assign1020_e745: f64 = (assign1020_e741 - assign1020_e744);
        let assign1020_e746: f64 = (0.5 * assign1020_e745);
        let assign1020_e748: f64 = (assign1020_e746 / var_nfabot_i);
        let assign1020_e749: f64 = (assign1020_e748).exp();
        let assign1020_e750: f64 = (assign1020_e737 * assign1020_e749);
        var_ftdbot2 = assign1020_e750;
        var_ftdbot2_rv = 0.0;

        let assign1030_e754: f64 = (var_xti_i / 2.0);
        let assign1030_e756: f64 = (assign1030_e754 / var_nfasti_i);
        let assign1030_e757: f64 = (var_auxt).powf(assign1030_e756);
        let assign1030_e761: f64 = (var_phigrsti * var_phitrinv);
        let assign1030_e764: f64 = (var_phigdsti * var_phitdinv);
        let assign1030_e765: f64 = (assign1030_e761 - assign1030_e764);
        let assign1030_e766: f64 = (0.5 * assign1030_e765);
        let assign1030_e768: f64 = (assign1030_e766 / var_nfasti_i);
        let assign1030_e769: f64 = (assign1030_e768).exp();
        let assign1030_e770: f64 = (assign1030_e757 * assign1030_e769);
        var_ftdsti2 = assign1030_e770;
        var_ftdsti2_rv = 0.0;

        let assign1040_e774: f64 = (var_xti_i / 2.0);
        let assign1040_e776: f64 = (assign1040_e774 / var_nfagat_i);
        let assign1040_e777: f64 = (var_auxt).powf(assign1040_e776);
        let assign1040_e781: f64 = (var_phigrgat * var_phitrinv);
        let assign1040_e784: f64 = (var_phigdgat * var_phitdinv);
        let assign1040_e785: f64 = (assign1040_e781 - assign1040_e784);
        let assign1040_e786: f64 = (0.5 * assign1040_e785);
        let assign1040_e788: f64 = (assign1040_e786 / var_nfagat_i);
        let assign1040_e789: f64 = (assign1040_e788).exp();
        let assign1040_e790: f64 = (assign1040_e777 * assign1040_e789);
        var_ftdgat2 = assign1040_e790;
        var_ftdgat2_rv = 0.0;

        let assign1050_e793: f64 = (var_idsatrbot_i * var_ftdbot2);
        let assign1050_e795: f64 = (assign1050_e793 * var_ftdbot2);
        var_idsatbot = assign1050_e795;
        var_idsatbot_rv = 0.0;

        let assign1060_e798: f64 = (var_idsatrsti_i * var_ftdsti2);
        let assign1060_e800: f64 = (assign1060_e798 * var_ftdsti2);
        var_idsatsti = assign1060_e800;
        var_idsatsti_rv = 0.0;

        let assign1070_e803: f64 = (var_idsatrgat_i * var_ftdgat2);
        let assign1070_e805: f64 = (assign1070_e803 * var_ftdgat2);
        var_idsatgat = assign1070_e805;
        var_idsatgat_rv = 0.0;

        let assign1780_e1250: f64 = (0.01 * var_shrink_i);
        let assign1780_e1251: f64 = (1.0 - assign1780_e1250);
        var_shrinkl = assign1780_e1251;
        var_shrinkl_rv = 0.0;

        let assign1860_e1316: f64 = (p.p87 * 1000000.0);
        var_ndibot_i = assign1860_e1316;
        var_ndibot_i_rv = 0.0;

        let assign1870_e1319: f64 = (p.p89 * 1000000.0);
        var_ndisti_i = assign1870_e1319;
        var_ndisti_i_rv = 0.0;

        let assign1880_e1322: f64 = (p.p88 * 1000000.0);
        var_ndigat_i = assign1880_e1322;
        var_ndigat_i_rv = 0.0;

        var_ndi_i = var_ndibot_i;
        var_ndi_i_rv = 0.0;

        var_njl = var_nfabot_i;
        var_njl_rv = 0.0;

        let assign1910_e1327: f64 = (1450.0 * 0.0001);
        var_muen_i = assign1910_e1327;
        var_muen_i_rv = 0.0;

        let assign1920_e1330: f64 = (500.0 * 0.0001);
        var_muep_i = assign1920_e1330;
        var_muep_i_rv = 0.0;

        var_pb = 0.6;
        var_pb_rv = 0.0;

        var_juncdlt = 0.001;
        var_juncdlt_rv = 0.0;

        let assign1950_e1335: f64 = (1.45e16 * var_ftdbot2);
        var_nin = assign1950_e1335;
        var_nin_rv = 0.0;

        let assign1960_e1338: f64 = (var_nin * var_nin);
        let assign1960_e1340: f64 = (assign1960_e1338 / var_ndi_i);
        var_pn0 = assign1960_e1340;
        var_pn0_rv = 0.0;

        let assign1970_e1343: f64 = (-1.5);
        let assign1970_e1344: f64 = (var_auxt).powf(assign1970_e1343);
        var_t1 = assign1970_e1344;
        var_t1_rv = 0.0;

        let assign1980_e1347: f64 = (var_muen_i * var_t1);
        let assign1980_e1349: f64 = (assign1980_e1347 / var_phitdinv);
        var_dn = assign1980_e1349;
        var_dn_rv = 0.0;

        let assign1990_e1352: f64 = (var_muep_i * var_t1);
        let assign1990_e1354: f64 = (assign1990_e1352 / var_phitdinv);
        var_dp = assign1990_e1354;
        var_dp_rv = 0.0;

        let assign2000_e1357: f64 = (2.0 * var_dn);
        let assign2000_e1359: f64 = (assign2000_e1357 * var_dp);
        let assign2000_e1362: f64 = (var_dn + var_dp);
        let assign2000_e1363: f64 = (assign2000_e1359 / assign2000_e1362);
        var_da = assign2000_e1363;
        var_da_rv = 0.0;

        let assign2010_e1366: f64 = (var_auxt).powf(p.p97);
        var_t2 = assign2010_e1366;
        var_t2_rv = 0.0;

        let assign2020_e1369: f64 = (p.p93 * var_t2);
        var_tau_hl = assign2020_e1369;
        var_tau_hl_rv = 0.0;

        let assign2030_e1372: f64 = (var_tau_hl * var_da);
        let assign2030_e1373: f64 = (assign2030_e1372).sqrt();
        var_la = assign2030_e1373;
        var_la_rv = 0.0;

        let assign2040_e1376: f64 = (var_njl / var_phitdinv);
        let assign2040_e1379: f64 = (var_ndi_i / var_pn0);
        let assign2040_e1380: f64 = (assign2040_e1379).ln();
        let assign2040_e1381: f64 = (assign2040_e1376 * assign2040_e1380);
        var_v_ha = assign2040_e1381;
        var_v_ha_rv = 0.0;

        let assign2050_e1384: f64 = (var_njl / var_phitdinv);
        let assign2050_e1387: f64 = (var_ndi_i / var_pn0);
        let assign2050_e1388: f64 = (assign2050_e1387).ln();
        let assign2050_e1391: f64 = (p.p94 / var_la);
        let assign2050_e1392: f64 = (assign2050_e1388 + assign2050_e1391);
        let assign2050_e1393: f64 = (assign2050_e1384 * assign2050_e1392);
        var_v_hk = assign2050_e1393;
        var_v_hk_rv = 0.0;

        let (assign2060_e1399,) = {
    if (p.p99 > 0.0) {
        (p.p99,)
    } else {
        (0.0,)
    }
};
        let assign2060_e1401: f64 = (assign2060_e1399 * var_scale_i);
        let assign2060_e1403: f64 = (assign2060_e1401 * var_scale_i);
        let assign2060_e1405: f64 = (assign2060_e1403 * var_shrinkl);
        let assign2060_e1407: f64 = (assign2060_e1405 * var_shrinkl);
        var_ab_i = assign2060_e1407;
        var_ab_i_rv = 0.0;

        let (assign2070_e1413,) = {
    if (p.p100 > 0.0) {
        (p.p100,)
    } else {
        (0.0,)
    }
};
        let assign2070_e1415: f64 = (assign2070_e1413 * var_scale_i);
        let assign2070_e1417: f64 = (assign2070_e1415 * var_shrinkl);
        var_ls_i = assign2070_e1417;
        var_ls_i_rv = 0.0;

        let (assign2080_e1423,) = {
    if (p.p101 > 0.0) {
        (p.p101,)
    } else {
        (0.0,)
    }
};
        let assign2080_e1425: f64 = (assign2080_e1423 * var_scale_i);
        let assign2080_e1427: f64 = (assign2080_e1425 * var_shrinkl);
        var_lg_i = assign2080_e1427;
        var_lg_i_rv = 0.0;

        var_exp_vmax_over_phitd_bot = 0.0;
        var_exp_vmax_over_phitd_bot_dn0 = 0.0;
        var_exp_vmax_over_phitd_bot_dn2 = 0.0;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let assign2130_e1434: f64 = (var_idsatbot * var_ab_i);
        let assign2130_e1436: f64 = if assign2130_e1434 > 0.0 { 1.0 } else { 0.0 };
        var_guard6 = assign2130_e1436;
        var_guard6_rv = 0.0;

        let (assign2140_e1451,) = {
    if (var_guard6 != 0.0) {
        let assign2140_e1442: f64 = (var_idsatbot * var_ab_i);
        let assign2140_e1443: f64 = (var_imax_i / assign2140_e1442);
        let assign2140_e1445: f64 = (assign2140_e1443 + 1.0);
        let assign2140_e1446: f64 = (assign2140_e1445).ln();
        let assign2140_e1447: f64 = (var_phitd * assign2140_e1446);
        let assign2140_e1449: f64 = (assign2140_e1447 * var_nfabot_i);
        (assign2140_e1449,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign2140_e1451;
        var_vmaxbot_rv = 0.0;

        let (assign2150_e1456,) = {
    if (var_guard6 == 0.0) {
        (100000000.0,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign2150_e1456;
        var_vmaxbot_rv = 0.0;

        let assign2160_e1459: f64 = (var_idsatsti * var_ls_i);
        let assign2160_e1461: f64 = if assign2160_e1459 > 0.0 { 1.0 } else { 0.0 };
        var_guard7 = assign2160_e1461;
        var_guard7_rv = 0.0;

        let (assign2170_e1476,) = {
    if (var_guard7 != 0.0) {
        let assign2170_e1467: f64 = (var_idsatsti * var_ls_i);
        let assign2170_e1468: f64 = (var_imax_i / assign2170_e1467);
        let assign2170_e1470: f64 = (assign2170_e1468 + 1.0);
        let assign2170_e1471: f64 = (assign2170_e1470).ln();
        let assign2170_e1472: f64 = (var_phitd * assign2170_e1471);
        let assign2170_e1474: f64 = (assign2170_e1472 * var_nfasti_i);
        (assign2170_e1474,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign2170_e1476;
        var_vmaxsti_rv = 0.0;

        *var_ab_i_slot = var_ab_i;
        *var_ab_i_rv_slot = var_ab_i_rv;
        *var_auxt_slot = var_auxt;
        *var_auxt_rv_slot = var_auxt_rv;
        *var_da_slot = var_da;
        *var_da_rv_slot = var_da_rv;
        *var_deltaphigd_slot = var_deltaphigd;
        *var_deltaphigd_rv_slot = var_deltaphigd_rv;
        *var_deltaphigr_slot = var_deltaphigr;
        *var_deltaphigr_rv_slot = var_deltaphigr_rv;
        *var_dn_slot = var_dn;
        *var_dn_rv_slot = var_dn_rv;
        *var_dp_slot = var_dp;
        *var_dp_rv_slot = var_dp_rv;
        *var_epssi_slot = var_epssi;
        *var_epssi_rv_slot = var_epssi_rv;
        *var_exp_vmax_over_phitd_bot_slot = var_exp_vmax_over_phitd_bot;
        *var_exp_vmax_over_phitd_bot_dn0_slot = var_exp_vmax_over_phitd_bot_dn0;
        *var_exp_vmax_over_phitd_bot_dn2_slot = var_exp_vmax_over_phitd_bot_dn2;
        *var_exp_vmax_over_phitd_bot_rv_slot = var_exp_vmax_over_phitd_bot_rv;
        *var_ftdbot2_slot = var_ftdbot2;
        *var_ftdbot2_rv_slot = var_ftdbot2_rv;
        *var_ftdgat2_slot = var_ftdgat2;
        *var_ftdgat2_rv_slot = var_ftdgat2_rv;
        *var_ftdsti2_slot = var_ftdsti2;
        *var_ftdsti2_rv_slot = var_ftdsti2_rv;
        *var_guard1_slot = var_guard1;
        *var_guard1_rv_slot = var_guard1_rv;
        *var_guard2_slot = var_guard2;
        *var_guard2_rv_slot = var_guard2_rv;
        *var_guard6_slot = var_guard6;
        *var_guard6_rv_slot = var_guard6_rv;
        *var_guard7_slot = var_guard7;
        *var_guard7_rv_slot = var_guard7_rv;
        *var_idsatbot_slot = var_idsatbot;
        *var_idsatbot_rv_slot = var_idsatbot_rv;
        *var_idsatgat_slot = var_idsatgat;
        *var_idsatgat_rv_slot = var_idsatgat_rv;
        *var_idsatrbot_i_slot = var_idsatrbot_i;
        *var_idsatrbot_i_rv_slot = var_idsatrbot_i_rv;
        *var_idsatrgat_i_slot = var_idsatrgat_i;
        *var_idsatrgat_i_rv_slot = var_idsatrgat_i_rv;
        *var_idsatrsti_i_slot = var_idsatrsti_i;
        *var_idsatrsti_i_rv_slot = var_idsatrsti_i_rv;
        *var_idsatsti_slot = var_idsatsti;
        *var_idsatsti_rv_slot = var_idsatsti_rv;
        *var_imax_i_slot = var_imax_i;
        *var_imax_i_rv_slot = var_imax_i_rv;
        *var_juncdlt_slot = var_juncdlt;
        *var_juncdlt_rv_slot = var_juncdlt_rv;
        *var_kbol_over_qele_slot = var_kbol_over_qele;
        *var_kbol_over_qele_rv_slot = var_kbol_over_qele_rv;
        *var_la_slot = var_la;
        *var_la_rv_slot = var_la_rv;
        *var_lg_i_slot = var_lg_i;
        *var_lg_i_rv_slot = var_lg_i_rv;
        *var_ls_i_slot = var_ls_i;
        *var_ls_i_rv_slot = var_ls_i_rv;
        *var_muen_i_slot = var_muen_i;
        *var_muen_i_rv_slot = var_muen_i_rv;
        *var_muep_i_slot = var_muep_i;
        *var_muep_i_rv_slot = var_muep_i_rv;
        *var_ndi_i_slot = var_ndi_i;
        *var_ndi_i_rv_slot = var_ndi_i_rv;
        *var_ndibot_i_slot = var_ndibot_i;
        *var_ndibot_i_rv_slot = var_ndibot_i_rv;
        *var_ndigat_i_slot = var_ndigat_i;
        *var_ndigat_i_rv_slot = var_ndigat_i_rv;
        *var_ndisti_i_slot = var_ndisti_i;
        *var_ndisti_i_rv_slot = var_ndisti_i_rv;
        *var_nfabot_i_slot = var_nfabot_i;
        *var_nfabot_i_rv_slot = var_nfabot_i_rv;
        *var_nfagat_i_slot = var_nfagat_i;
        *var_nfagat_i_rv_slot = var_nfagat_i_rv;
        *var_nfasti_i_slot = var_nfasti_i;
        *var_nfasti_i_rv_slot = var_nfasti_i_rv;
        *var_nin_slot = var_nin;
        *var_nin_rv_slot = var_nin_rv;
        *var_njl_slot = var_njl;
        *var_njl_rv_slot = var_njl_rv;
        *var_pb_slot = var_pb;
        *var_pb_rv_slot = var_pb_rv;
        *var_phigbot_i_slot = var_phigbot_i;
        *var_phigbot_i_rv_slot = var_phigbot_i_rv;
        *var_phigdbot_slot = var_phigdbot;
        *var_phigdbot_rv_slot = var_phigdbot_rv;
        *var_phigdgat_slot = var_phigdgat;
        *var_phigdgat_rv_slot = var_phigdgat_rv;
        *var_phigdsti_slot = var_phigdsti;
        *var_phigdsti_rv_slot = var_phigdsti_rv;
        *var_phiggat_i_slot = var_phiggat_i;
        *var_phiggat_i_rv_slot = var_phiggat_i_rv;
        *var_phigrbot_slot = var_phigrbot;
        *var_phigrbot_rv_slot = var_phigrbot_rv;
        *var_phigrgat_slot = var_phigrgat;
        *var_phigrgat_rv_slot = var_phigrgat_rv;
        *var_phigrsti_slot = var_phigrsti;
        *var_phigrsti_rv_slot = var_phigrsti_rv;
        *var_phigsti_i_slot = var_phigsti_i;
        *var_phigsti_i_rv_slot = var_phigsti_i_rv;
        *var_phitd_slot = var_phitd;
        *var_phitd_rv_slot = var_phitd_rv;
        *var_phitdinv_slot = var_phitdinv;
        *var_phitdinv_rv_slot = var_phitdinv_rv;
        *var_phitr_slot = var_phitr;
        *var_phitr_rv_slot = var_phitr_rv;
        *var_phitrinv_slot = var_phitrinv;
        *var_phitrinv_rv_slot = var_phitrinv_rv;
        *var_pn0_slot = var_pn0;
        *var_pn0_rv_slot = var_pn0_rv;
        *var_scale_i_slot = var_scale_i;
        *var_scale_i_rv_slot = var_scale_i_rv;
        *var_shrink_i_slot = var_shrink_i;
        *var_shrink_i_rv_slot = var_shrink_i_rv;
        *var_shrinkl_slot = var_shrinkl;
        *var_shrinkl_rv_slot = var_shrinkl_rv;
        *var_swjunexp_i_slot = var_swjunexp_i;
        *var_swjunexp_i_rv_slot = var_swjunexp_i_rv;
        *var_t1_slot = var_t1;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_rv_slot = var_t2_rv;
        *var_tau_hl_slot = var_tau_hl;
        *var_tau_hl_rv_slot = var_tau_hl_rv;
        *var_tkd_slot = var_tkd;
        *var_tkd_rv_slot = var_tkd_rv;
        *var_tkr_slot = var_tkr;
        *var_tkr_rv_slot = var_tkr_rv;
        *var_trj_i_slot = var_trj_i;
        *var_trj_i_rv_slot = var_trj_i_rv;
        *var_v_ha_slot = var_v_ha;
        *var_v_ha_rv_slot = var_v_ha_rv;
        *var_v_hk_slot = var_v_hk;
        *var_v_hk_rv_slot = var_v_hk_rv;
        *var_vjunref_i_slot = var_vjunref_i;
        *var_vjunref_i_rv_slot = var_vjunref_i_rv;
        *var_vmaxbot_slot = var_vmaxbot;
        *var_vmaxbot_rv_slot = var_vmaxbot_rv;
        *var_vmaxsti_slot = var_vmaxsti;
        *var_vmaxsti_rv_slot = var_vmaxsti_rv;
        *var_xti_i_slot = var_xti_i;
        *var_xti_i_rv_slot = var_xti_i_rv;
    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        var_ab_i: f64,
        var_epssi: f64,
        var_guard7: f64,
        var_idsatgat: f64,
        var_imax_i: f64,
        var_lg_i: f64,
        var_ls_i: f64,
        var_ndi_i: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_vjunref_i: f64,
        var_vmaxbot: f64,
        var_fraci_slot: &mut f64,
        var_fraci_rv_slot: &mut f64,
        var_fracna_slot: &mut f64,
        var_fracna_rv_slot: &mut f64,
        var_fracnb_slot: &mut f64,
        var_fracnb_rv_slot: &mut f64,
        var_guard26_slot: &mut f64,
        var_guard26_rv_slot: &mut f64,
        var_guard27_slot: &mut f64,
        var_guard27_rv_slot: &mut f64,
        var_guard31_slot: &mut f64,
        var_guard31_rv_slot: &mut f64,
        var_guard76_slot: &mut f64,
        var_guard76_rv_slot: &mut f64,
        var_guard77_slot: &mut f64,
        var_guard77_rv_slot: &mut f64,
        var_guard8_slot: &mut f64,
        var_guard80_slot: &mut f64,
        var_guard80_rv_slot: &mut f64,
        var_guard8_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_inqs0_a_slot: &mut f64,
        var_inqs0_a_dn0_slot: &mut f64,
        var_inqs0_a_dn2_slot: &mut f64,
        var_inqs0_a_dn3_slot: &mut f64,
        var_inqs0_a_rv_slot: &mut f64,
        var_inqs0_k_slot: &mut f64,
        var_inqs0_k_dn0_slot: &mut f64,
        var_inqs0_k_dn2_slot: &mut f64,
        var_inqs0_k_dn4_slot: &mut f64,
        var_inqs0_k_rv_slot: &mut f64,
        var_iwnqs0_a_slot: &mut f64,
        var_iwnqs0_a_dn0_slot: &mut f64,
        var_iwnqs0_a_dn2_slot: &mut f64,
        var_iwnqs0_a_dn5_slot: &mut f64,
        var_iwnqs0_a_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_q_nqs_a_slot: &mut f64,
        var_q_nqs_a_dn3_slot: &mut f64,
        var_q_nqs_a_rv_slot: &mut f64,
        var_q_nqs_k_slot: &mut f64,
        var_q_nqs_k_dn4_slot: &mut f64,
        var_q_nqs_k_rv_slot: &mut f64,
        var_q_pex0_slot: &mut f64,
        var_q_pex0_rv_slot: &mut f64,
        var_swjunexp_i_slot: &mut f64,
        var_swjunexp_i_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_v1_slot: &mut f64,
        var_v1_rv_slot: &mut f64,
        var_v2_slot: &mut f64,
        var_v2_rv_slot: &mut f64,
        var_v3_slot: &mut f64,
        var_v3_rv_slot: &mut f64,
        var_v4_slot: &mut f64,
        var_v4_rv_slot: &mut f64,
        var_v5_slot: &mut f64,
        var_v5_rv_slot: &mut f64,
        var_vak_slot: &mut f64,
        var_vak_dn0_slot: &mut f64,
        var_vak_dn2_slot: &mut f64,
        var_vak_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
        var_vmax_slot: &mut f64,
        var_vmax_rv_slot: &mut f64,
        var_vmaxgat_slot: &mut f64,
        var_vmaxgat_rv_slot: &mut f64,
        var_vmaxsti_slot: &mut f64,
        var_vmaxsti_rv_slot: &mut f64,
        var_w_depa0_slot: &mut f64,
        var_w_depa0_dn0_slot: &mut f64,
        var_w_depa0_dn2_slot: &mut f64,
        var_w_depa0_rv_slot: &mut f64,
        var_w_nqs_a_slot: &mut f64,
        var_w_nqs_a_dn5_slot: &mut f64,
        var_w_nqs_a_rv_slot: &mut f64,
    ) {
        let mut var_fraci: f64 = *var_fraci_slot;
        let mut var_fraci_rv: f64 = *var_fraci_rv_slot;
        let mut var_fracna: f64 = *var_fracna_slot;
        let mut var_fracna_rv: f64 = *var_fracna_rv_slot;
        let mut var_fracnb: f64 = *var_fracnb_slot;
        let mut var_fracnb_rv: f64 = *var_fracnb_rv_slot;
        let mut var_guard26: f64 = *var_guard26_slot;
        let mut var_guard26_rv: f64 = *var_guard26_rv_slot;
        let mut var_guard27: f64 = *var_guard27_slot;
        let mut var_guard27_rv: f64 = *var_guard27_rv_slot;
        let mut var_guard31: f64 = *var_guard31_slot;
        let mut var_guard31_rv: f64 = *var_guard31_rv_slot;
        let mut var_guard76: f64 = *var_guard76_slot;
        let mut var_guard76_rv: f64 = *var_guard76_rv_slot;
        let mut var_guard77: f64 = *var_guard77_slot;
        let mut var_guard77_rv: f64 = *var_guard77_rv_slot;
        let mut var_guard8: f64 = *var_guard8_slot;
        let mut var_guard80: f64 = *var_guard80_slot;
        let mut var_guard80_rv: f64 = *var_guard80_rv_slot;
        let mut var_guard8_rv: f64 = *var_guard8_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_inqs0_a: f64 = *var_inqs0_a_slot;
        let mut var_inqs0_a_dn0: f64 = *var_inqs0_a_dn0_slot;
        let mut var_inqs0_a_dn2: f64 = *var_inqs0_a_dn2_slot;
        let mut var_inqs0_a_dn3: f64 = *var_inqs0_a_dn3_slot;
        let mut var_inqs0_a_rv: f64 = *var_inqs0_a_rv_slot;
        let mut var_inqs0_k: f64 = *var_inqs0_k_slot;
        let mut var_inqs0_k_dn0: f64 = *var_inqs0_k_dn0_slot;
        let mut var_inqs0_k_dn2: f64 = *var_inqs0_k_dn2_slot;
        let mut var_inqs0_k_dn4: f64 = *var_inqs0_k_dn4_slot;
        let mut var_inqs0_k_rv: f64 = *var_inqs0_k_rv_slot;
        let mut var_iwnqs0_a: f64 = *var_iwnqs0_a_slot;
        let mut var_iwnqs0_a_dn0: f64 = *var_iwnqs0_a_dn0_slot;
        let mut var_iwnqs0_a_dn2: f64 = *var_iwnqs0_a_dn2_slot;
        let mut var_iwnqs0_a_dn5: f64 = *var_iwnqs0_a_dn5_slot;
        let mut var_iwnqs0_a_rv: f64 = *var_iwnqs0_a_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_q_nqs_a: f64 = *var_q_nqs_a_slot;
        let mut var_q_nqs_a_dn3: f64 = *var_q_nqs_a_dn3_slot;
        let mut var_q_nqs_a_rv: f64 = *var_q_nqs_a_rv_slot;
        let mut var_q_nqs_k: f64 = *var_q_nqs_k_slot;
        let mut var_q_nqs_k_dn4: f64 = *var_q_nqs_k_dn4_slot;
        let mut var_q_nqs_k_rv: f64 = *var_q_nqs_k_rv_slot;
        let mut var_q_pex0: f64 = *var_q_pex0_slot;
        let mut var_q_pex0_rv: f64 = *var_q_pex0_rv_slot;
        let mut var_swjunexp_i: f64 = *var_swjunexp_i_slot;
        let mut var_swjunexp_i_rv: f64 = *var_swjunexp_i_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_v1: f64 = *var_v1_slot;
        let mut var_v1_rv: f64 = *var_v1_rv_slot;
        let mut var_v2: f64 = *var_v2_slot;
        let mut var_v2_rv: f64 = *var_v2_rv_slot;
        let mut var_v3: f64 = *var_v3_slot;
        let mut var_v3_rv: f64 = *var_v3_rv_slot;
        let mut var_v4: f64 = *var_v4_slot;
        let mut var_v4_rv: f64 = *var_v4_rv_slot;
        let mut var_v5: f64 = *var_v5_slot;
        let mut var_v5_rv: f64 = *var_v5_rv_slot;
        let mut var_vak: f64 = *var_vak_slot;
        let mut var_vak_dn0: f64 = *var_vak_dn0_slot;
        let mut var_vak_dn2: f64 = *var_vak_dn2_slot;
        let mut var_vak_rv: f64 = *var_vak_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;
        let mut var_vmax: f64 = *var_vmax_slot;
        let mut var_vmax_rv: f64 = *var_vmax_rv_slot;
        let mut var_vmaxgat: f64 = *var_vmaxgat_slot;
        let mut var_vmaxgat_rv: f64 = *var_vmaxgat_rv_slot;
        let mut var_vmaxsti: f64 = *var_vmaxsti_slot;
        let mut var_vmaxsti_rv: f64 = *var_vmaxsti_rv_slot;
        let mut var_w_depa0: f64 = *var_w_depa0_slot;
        let mut var_w_depa0_dn0: f64 = *var_w_depa0_dn0_slot;
        let mut var_w_depa0_dn2: f64 = *var_w_depa0_dn2_slot;
        let mut var_w_depa0_rv: f64 = *var_w_depa0_rv_slot;
        let mut var_w_nqs_a: f64 = *var_w_nqs_a_slot;
        let mut var_w_nqs_a_dn5: f64 = *var_w_nqs_a_dn5_slot;
        let mut var_w_nqs_a_rv: f64 = *var_w_nqs_a_rv_slot;

        let (assign2180_e1481,) = {
    if (var_guard7 == 0.0) {
        (100000000.0,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign2180_e1481;
        var_vmaxsti_rv = 0.0;

        let assign2190_e1484: f64 = (var_idsatgat * var_lg_i);
        let assign2190_e1486: f64 = if assign2190_e1484 > 0.0 { 1.0 } else { 0.0 };
        var_guard8 = assign2190_e1486;
        var_guard8_rv = 0.0;

        let (assign2200_e1501,) = {
    if (var_guard8 != 0.0) {
        let assign2200_e1492: f64 = (var_idsatgat * var_lg_i);
        let assign2200_e1493: f64 = (var_imax_i / assign2200_e1492);
        let assign2200_e1495: f64 = (assign2200_e1493 + 1.0);
        let assign2200_e1496: f64 = (assign2200_e1495).ln();
        let assign2200_e1497: f64 = (var_phitd * assign2200_e1496);
        let assign2200_e1499: f64 = (assign2200_e1497 * var_nfagat_i);
        (assign2200_e1499,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign2200_e1501;
        var_vmaxgat_rv = 0.0;

        let (assign2210_e1506,) = {
    if (var_guard8 == 0.0) {
        (100000000.0,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign2210_e1506;
        var_vmaxgat_rv = 0.0;

        let assign2220_e1509: f64 = (var_vmaxbot).min(var_vmaxsti);
        let assign2220_e1511: f64 = (assign2220_e1509).min(var_vmaxgat);
        var_vmax = assign2220_e1511;
        var_vmax_rv = 0.0;

        var_idmultbot = 0.0;
        var_idmultbot_dn0 = 0.0;
        var_idmultbot_dn2 = 0.0;
        var_idmultbot_rv = 0.0;

        var_iwnqs0_a = 0.0;
        var_iwnqs0_a_dn0 = 0.0;
        var_iwnqs0_a_dn2 = 0.0;
        var_iwnqs0_a_dn5 = 0.0;
        var_iwnqs0_a_rv = 0.0;

        var_inqs0_a = 0.0;
        var_inqs0_a_dn0 = 0.0;
        var_inqs0_a_dn2 = 0.0;
        var_inqs0_a_dn3 = 0.0;
        var_inqs0_a_rv = 0.0;

        var_inqs0_k = 0.0;
        var_inqs0_k_dn0 = 0.0;
        var_inqs0_k_dn2 = 0.0;
        var_inqs0_k_dn4 = 0.0;
        var_inqs0_k_rv = 0.0;

        var_q_nqs_a = 0.0;
        var_q_nqs_a_dn3 = 0.0;
        var_q_nqs_a_rv = 0.0;

        var_q_nqs_k = 0.0;
        var_q_nqs_k_dn4 = 0.0;
        var_q_nqs_k_rv = 0.0;

        var_w_nqs_a = 0.0;
        var_w_nqs_a_dn5 = 0.0;
        var_w_nqs_a_rv = 0.0;

        let assign3120_e1859: f64 = (1.6021918e-19 * var_ab_i);
        var_q_pex0 = assign3120_e1859;
        var_q_pex0_rv = 0.0;

        let assign3130_e1862: f64 = (2.0 * var_epssi);
        let assign3130_e1865: f64 = (1.6021918e-19 * var_ndi_i);
        let assign3130_e1866: f64 = (assign3130_e1862 / assign3130_e1865);
        let assign3130_e1867: f64 = (assign3130_e1866).sqrt();
        var_w_depa0 = assign3130_e1867;
        var_w_depa0_dn0 = 0.0;
        var_w_depa0_dn2 = 0.0;
        var_w_depa0_rv = 0.0;

        let assign3140_e1870: f64 = (p.p94 - var_w_depa0);
        let assign3140_e1872: f64 = (assign3140_e1870 - 1e-7);
        var_tmf1 = assign3140_e1872;
        var_tmf1_dn0 = (-var_w_depa0_dn0);
        var_tmf1_dn2 = (-var_w_depa0_dn2);
        var_tmf1_rv = 0.0;

        let assign3150_e1875: f64 = (4.0 * p.p94);
        let assign3150_e1877: f64 = (assign3150_e1875 * 1e-7);
        var_tmf2 = assign3150_e1877;
        var_tmf2_dn0 = 0.0;
        var_tmf2_dn2 = 0.0;
        var_tmf2_rv = 0.0;

        let (assign3160_e1884, assign3160_e1884_d_n0, assign3160_e1884_d_n2,) = {
    if (var_tmf2 > 0.0) {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    } else {
        let assign3160_e1883: f64 = (-var_tmf2);
        (assign3160_e1883, (-var_tmf2_dn0), (-var_tmf2_dn2),)
    }
};
        var_tmf2 = assign3160_e1884;
        var_tmf2_dn0 = assign3160_e1884_d_n0;
        var_tmf2_dn2 = assign3160_e1884_d_n2;
        var_tmf2_rv = 0.0;

        let assign3170_e1887: f64 = (var_tmf1 * var_tmf1);
        let assign3170_e1889: f64 = (assign3170_e1887 + var_tmf2);
        let assign3170_e1890: f64 = (assign3170_e1889).sqrt();
        var_tmf2 = assign3170_e1890;
        var_tmf2_dn0 = ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign3170_e1890));
        var_tmf2_dn2 = ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign3170_e1890));
        var_tmf2_rv = 0.0;

        let assign3180_e1895: f64 = (var_tmf1 + var_tmf2);
        let assign3180_e1896: f64 = (0.5 * assign3180_e1895);
        let assign3180_e1897: f64 = (p.p94 - assign3180_e1896);
        var_w_depa0 = assign3180_e1897;
        var_w_depa0_dn0 = (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0)));
        var_w_depa0_dn2 = (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2)));
        var_w_depa0_rv = 0.0;

        let assign3190_e1900: f64 = if var_swjunexp_i > 0.9 { 1.0 } else { 0.0 };
        var_guard26 = assign3190_e1900;
        var_guard26_rv = 0.0;

        let assign3200_e1903: f64 = (var_nfabot_i - var_nfagat_i);
        let assign3200_e1904: f64 = (assign3200_e1903).abs();
        let assign3200_e1917: f64 = (var_nfabot_i - var_nfasti_i);
        let assign3200_e1918: f64 = (assign3200_e1917).abs();
        let assign3200_e1932: f64 = (var_nfagat_i - var_nfasti_i);
        let assign3200_e1933: f64 = (assign3200_e1932).abs();
        let assign3200_e1944: f64 = if (((((assign3200_e1904 > 1e-6) && (var_ab_i > 0.0)) && (var_lg_i > 0.0)) || (((assign3200_e1918 > 1e-6) && (var_ab_i > 0.0)) && (var_ls_i > 0.0))) || (((assign3200_e1933 > 1e-6) && (var_lg_i > 0.0)) && (var_ls_i > 0.0))) { 1.0 } else { 0.0 };
        var_guard27 = assign3200_e1944;
        var_guard27_rv = 0.0;

        let (assign3210_e1950,) = {
    if ((var_guard26 != 0.0) && (var_guard27 != 0.0)) {
        (0.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign3210_e1950;
        var_swjunexp_i_rv = 0.0;

        let assign3280_e1989: f64 = if var_swjunexp_i == 1.0 { 1.0 } else { 0.0 };
        var_guard31 = assign3280_e1989;
        var_guard31_rv = 0.0;

        let (assign3380_e2029, assign3380_e2029_d_n0, assign3380_e2029_d_n2,) = {
    if (var_guard31 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_vak, var_vak_dn0, var_vak_dn2,)
    }
};
        var_vak = assign3380_e2029;
        var_vak_dn0 = assign3380_e2029_d_n0;
        var_vak_dn2 = assign3380_e2029_d_n2;
        var_vak_rv = 0.0;

        let (assign3770_e2185,) = {
    if (var_guard31 != 0.0) {
        (0.4,)
    } else {
        (var_fracna,)
    }
};
        var_fracna = assign3770_e2185;
        var_fracna_rv = 0.0;

        let (assign3780_e2189,) = {
    if (var_guard31 != 0.0) {
        (0.65,)
    } else {
        (var_fracnb,)
    }
};
        var_fracnb = assign3780_e2189;
        var_fracnb_rv = 0.0;

        let (assign3790_e2193,) = {
    if (var_guard31 != 0.0) {
        (0.8,)
    } else {
        (var_fraci,)
    }
};
        var_fraci = assign3790_e2193;
        var_fraci_rv = 0.0;

        let (assign3800_e2200,) = {
    if (var_guard31 != 0.0) {
        let assign3800_e2196: f64 = (-var_fracna);
        let assign3800_e2198: f64 = (assign3800_e2196 * var_vjunref_i);
        (assign3800_e2198,)
    } else {
        (var_v1,)
    }
};
        var_v1 = assign3800_e2200;
        var_v1_rv = 0.0;

        let (assign3810_e2207,) = {
    if (var_guard31 != 0.0) {
        let assign3810_e2203: f64 = (-var_fracnb);
        let assign3810_e2205: f64 = (assign3810_e2203 * var_vjunref_i);
        (assign3810_e2205,)
    } else {
        (var_v2,)
    }
};
        var_v2 = assign3810_e2207;
        var_v2_rv = 0.0;

        let (assign3820_e2214,) = {
    if (var_guard31 != 0.0) {
        let assign3820_e2210: f64 = (-var_fraci);
        let assign3820_e2212: f64 = (assign3820_e2210 * var_vjunref_i);
        (assign3820_e2212,)
    } else {
        (var_v3,)
    }
};
        var_v3 = assign3820_e2214;
        var_v3_rv = 0.0;

        let (assign3830_e2218,) = {
    if (var_guard31 != 0.0) {
        (0.1,)
    } else {
        (var_v4,)
    }
};
        var_v4 = assign3830_e2218;
        var_v4_rv = 0.0;

        let (assign3840_e2222,) = {
    if (var_guard31 != 0.0) {
        (0.2,)
    } else {
        (var_v5,)
    }
};
        var_v5 = assign3840_e2222;
        var_v5_rv = 0.0;

        let assign3850_e2234: f64 = if (!(((var_ab_i == 0.0) && (var_ls_i == 0.0)) && (var_lg_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard76 = assign3850_e2234;
        var_guard76_rv = 0.0;

        let assign3930_e2306: f64 = if var_v1 < var_vmax { 1.0 } else { 0.0 };
        var_guard77 = assign3930_e2306;
        var_guard77_rv = 0.0;

        let (assign3990_e2447,) = {
    if (((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) {
        let assign3990_e2443: f64 = (var_nin * var_nin);
        let assign3990_e2445: f64 = (assign3990_e2443 / var_ndibot_i);
        (assign3990_e2445,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign3990_e2447;
        var_pnn0_rv = 0.0;

        let (assign4000_e2462,) = {
    if (((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) {
        let assign4000_e2455: f64 = (var_nfabot_i / var_phitdinv);
        let assign4000_e2458: f64 = (var_ndibot_i / var_pnn0);
        let assign4000_e2459: f64 = (assign4000_e2458).ln();
        let assign4000_e2460: f64 = (assign4000_e2455 * assign4000_e2459);
        (assign4000_e2460,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign4000_e2462;
        var_vha1_rv = 0.0;

        let assign4010_e2465: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard80 = assign4010_e2465;
        var_guard80_rv = 0.0;

        let (assign4020_e2481, assign4020_e2481_d_n0, assign4020_e2481_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4020_e2476: f64 = (var_v1 - var_vha1);
        let assign4020_e2477: f64 = (p.p86 * assign4020_e2476);
        let assign4020_e2479: f64 = (assign4020_e2477 + var_nfabot_i);
        (assign4020_e2479, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign4020_e2481;
        var_nja10_dn0 = assign4020_e2481_d_n0;
        var_nja10_dn2 = assign4020_e2481_d_n2;
        var_nja10_rv = 0.0;

        let (assign4030_e2495, assign4030_e2495_d_n0, assign4030_e2495_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4030_e2492: f64 = (p.p86 * var_vha1);
        let assign4030_e2493: f64 = (var_nfabot_i - assign4030_e2492);
        (assign4030_e2493, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign4030_e2495;
        var_nj0_dn0 = assign4030_e2495_d_n0;
        var_nj0_dn2 = assign4030_e2495_d_n2;
        var_nj0_rv = 0.0;

        let (assign4040_e2509, assign4040_e2509_d_n0, assign4040_e2509_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4040_e2505: f64 = (p.p85 - var_nja10);
        let assign4040_e2507: f64 = (assign4040_e2505 - 0.01);
        (assign4040_e2507, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign4040_e2509;
        var_tmf1_dn0 = assign4040_e2509_d_n0;
        var_tmf1_dn2 = assign4040_e2509_d_n2;
        var_tmf1_rv = 0.0;

        let (assign4050_e2523, assign4050_e2523_d_n0, assign4050_e2523_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4050_e2519: f64 = (4.0 * p.p85);
        let assign4050_e2521: f64 = (assign4050_e2519 * 0.01);
        (assign4050_e2521, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4050_e2523;
        var_tmf2_dn0 = assign4050_e2523_d_n0;
        var_tmf2_dn2 = assign4050_e2523_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4060_e2539, assign4060_e2539_d_n0, assign4060_e2539_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let (assign4060_e2537, assign4060_e2537_d_n0, assign4060_e2537_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign4060_e2536: f64 = (-var_tmf2);
                (assign4060_e2536, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign4060_e2537, assign4060_e2537_d_n0, assign4060_e2537_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4060_e2539;
        var_tmf2_dn0 = assign4060_e2539_d_n0;
        var_tmf2_dn2 = assign4060_e2539_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4070_e2554, assign4070_e2554_d_n0, assign4070_e2554_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4070_e2549: f64 = (var_tmf1 * var_tmf1);
        let assign4070_e2551: f64 = (assign4070_e2549 + var_tmf2);
        let assign4070_e2552: f64 = (assign4070_e2551).sqrt();
        (assign4070_e2552, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign4070_e2552)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign4070_e2552)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4070_e2554;
        var_tmf2_dn0 = assign4070_e2554_d_n0;
        var_tmf2_dn2 = assign4070_e2554_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4080_e2570, assign4080_e2570_d_n0, assign4080_e2570_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4080_e2566: f64 = (var_tmf1 + var_tmf2);
        let assign4080_e2567: f64 = (0.5 * assign4080_e2566);
        let assign4080_e2568: f64 = (p.p85 - assign4080_e2567);
        (assign4080_e2568, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign4080_e2570;
        var_nja11_dn0 = assign4080_e2570_d_n0;
        var_nja11_dn2 = assign4080_e2570_d_n2;
        var_nja11_rv = 0.0;

        let (assign4090_e2584, assign4090_e2584_d_n0, assign4090_e2584_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4090_e2580: f64 = (var_nja11 - var_nfabot_i);
        let assign4090_e2582: f64 = (assign4090_e2580 - 0.01);
        (assign4090_e2582, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign4090_e2584;
        var_tmf1_dn0 = assign4090_e2584_d_n0;
        var_tmf1_dn2 = assign4090_e2584_d_n2;
        var_tmf1_rv = 0.0;

        let (assign4100_e2598, assign4100_e2598_d_n0, assign4100_e2598_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4100_e2594: f64 = (4.0 * var_nfabot_i);
        let assign4100_e2596: f64 = (assign4100_e2594 * 0.01);
        (assign4100_e2596, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4100_e2598;
        var_tmf2_dn0 = assign4100_e2598_d_n0;
        var_tmf2_dn2 = assign4100_e2598_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4110_e2614, assign4110_e2614_d_n0, assign4110_e2614_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let (assign4110_e2612, assign4110_e2612_d_n0, assign4110_e2612_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign4110_e2611: f64 = (-var_tmf2);
                (assign4110_e2611, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign4110_e2612, assign4110_e2612_d_n0, assign4110_e2612_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4110_e2614;
        var_tmf2_dn0 = assign4110_e2614_d_n0;
        var_tmf2_dn2 = assign4110_e2614_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4120_e2629, assign4120_e2629_d_n0, assign4120_e2629_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4120_e2624: f64 = (var_tmf1 * var_tmf1);
        let assign4120_e2626: f64 = (assign4120_e2624 + var_tmf2);
        let assign4120_e2627: f64 = (assign4120_e2626).sqrt();
        (assign4120_e2627, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign4120_e2627)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign4120_e2627)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4120_e2629;
        var_tmf2_dn0 = assign4120_e2629_d_n0;
        var_tmf2_dn2 = assign4120_e2629_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4130_e2645, assign4130_e2645_d_n0, assign4130_e2645_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4130_e2641: f64 = (var_tmf1 + var_tmf2);
        let assign4130_e2642: f64 = (0.5 * assign4130_e2641);
        let assign4130_e2643: f64 = (var_nfabot_i + assign4130_e2642);
        (assign4130_e2643, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign4130_e2645;
        var_nj1_dn0 = assign4130_e2645_d_n0;
        var_nj1_dn2 = assign4130_e2645_d_n2;
        var_nj1_rv = 0.0;

        let (assign4140_e2659, assign4140_e2659_d_n0, assign4140_e2659_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4140_e2655: f64 = (p.p85 - var_nj0);
        let assign4140_e2657: f64 = (assign4140_e2655 - 0.01);
        (assign4140_e2657, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign4140_e2659;
        var_tmf1_dn0 = assign4140_e2659_d_n0;
        var_tmf1_dn2 = assign4140_e2659_d_n2;
        var_tmf1_rv = 0.0;

        *var_fraci_slot = var_fraci;
        *var_fraci_rv_slot = var_fraci_rv;
        *var_fracna_slot = var_fracna;
        *var_fracna_rv_slot = var_fracna_rv;
        *var_fracnb_slot = var_fracnb;
        *var_fracnb_rv_slot = var_fracnb_rv;
        *var_guard26_slot = var_guard26;
        *var_guard26_rv_slot = var_guard26_rv;
        *var_guard27_slot = var_guard27;
        *var_guard27_rv_slot = var_guard27_rv;
        *var_guard31_slot = var_guard31;
        *var_guard31_rv_slot = var_guard31_rv;
        *var_guard76_slot = var_guard76;
        *var_guard76_rv_slot = var_guard76_rv;
        *var_guard77_slot = var_guard77;
        *var_guard77_rv_slot = var_guard77_rv;
        *var_guard8_slot = var_guard8;
        *var_guard80_slot = var_guard80;
        *var_guard80_rv_slot = var_guard80_rv;
        *var_guard8_rv_slot = var_guard8_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_inqs0_a_slot = var_inqs0_a;
        *var_inqs0_a_dn0_slot = var_inqs0_a_dn0;
        *var_inqs0_a_dn2_slot = var_inqs0_a_dn2;
        *var_inqs0_a_dn3_slot = var_inqs0_a_dn3;
        *var_inqs0_a_rv_slot = var_inqs0_a_rv;
        *var_inqs0_k_slot = var_inqs0_k;
        *var_inqs0_k_dn0_slot = var_inqs0_k_dn0;
        *var_inqs0_k_dn2_slot = var_inqs0_k_dn2;
        *var_inqs0_k_dn4_slot = var_inqs0_k_dn4;
        *var_inqs0_k_rv_slot = var_inqs0_k_rv;
        *var_iwnqs0_a_slot = var_iwnqs0_a;
        *var_iwnqs0_a_dn0_slot = var_iwnqs0_a_dn0;
        *var_iwnqs0_a_dn2_slot = var_iwnqs0_a_dn2;
        *var_iwnqs0_a_dn5_slot = var_iwnqs0_a_dn5;
        *var_iwnqs0_a_rv_slot = var_iwnqs0_a_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_q_nqs_a_slot = var_q_nqs_a;
        *var_q_nqs_a_dn3_slot = var_q_nqs_a_dn3;
        *var_q_nqs_a_rv_slot = var_q_nqs_a_rv;
        *var_q_nqs_k_slot = var_q_nqs_k;
        *var_q_nqs_k_dn4_slot = var_q_nqs_k_dn4;
        *var_q_nqs_k_rv_slot = var_q_nqs_k_rv;
        *var_q_pex0_slot = var_q_pex0;
        *var_q_pex0_rv_slot = var_q_pex0_rv;
        *var_swjunexp_i_slot = var_swjunexp_i;
        *var_swjunexp_i_rv_slot = var_swjunexp_i_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_v1_slot = var_v1;
        *var_v1_rv_slot = var_v1_rv;
        *var_v2_slot = var_v2;
        *var_v2_rv_slot = var_v2_rv;
        *var_v3_slot = var_v3;
        *var_v3_rv_slot = var_v3_rv;
        *var_v4_slot = var_v4;
        *var_v4_rv_slot = var_v4_rv;
        *var_v5_slot = var_v5;
        *var_v5_rv_slot = var_v5_rv;
        *var_vak_slot = var_vak;
        *var_vak_dn0_slot = var_vak_dn0;
        *var_vak_dn2_slot = var_vak_dn2;
        *var_vak_rv_slot = var_vak_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
        *var_vmax_slot = var_vmax;
        *var_vmax_rv_slot = var_vmax_rv;
        *var_vmaxgat_slot = var_vmaxgat;
        *var_vmaxgat_rv_slot = var_vmaxgat_rv;
        *var_vmaxsti_slot = var_vmaxsti;
        *var_vmaxsti_rv_slot = var_vmaxsti_rv;
        *var_w_depa0_slot = var_w_depa0;
        *var_w_depa0_dn0_slot = var_w_depa0_dn0;
        *var_w_depa0_dn2_slot = var_w_depa0_dn2;
        *var_w_depa0_rv_slot = var_w_depa0_rv;
        *var_w_nqs_a_slot = var_w_nqs_a;
        *var_w_nqs_a_dn5_slot = var_w_nqs_a_dn5;
        *var_w_nqs_a_rv_slot = var_w_nqs_a_rv;
    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        var_guard31: f64,
        var_guard76: f64,
        var_guard77: f64,
        var_guard80: f64,
        var_ndisti_i: f64,
        var_nfabot_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v1: f64,
        var_guard81_slot: &mut f64,
        var_guard81_rv_slot: &mut f64,
        var_guard82_slot: &mut f64,
        var_guard82_rv_slot: &mut f64,
        var_guard83_slot: &mut f64,
        var_guard83_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_guard81: f64 = *var_guard81_slot;
        let mut var_guard81_rv: f64 = *var_guard81_rv_slot;
        let mut var_guard82: f64 = *var_guard82_slot;
        let mut var_guard82_rv: f64 = *var_guard82_rv_slot;
        let mut var_guard83: f64 = *var_guard83_slot;
        let mut var_guard83_rv: f64 = *var_guard83_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign4150_e2673, assign4150_e2673_d_n0, assign4150_e2673_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4150_e2669: f64 = (4.0 * p.p85);
        let assign4150_e2671: f64 = (assign4150_e2669 * 0.01);
        (assign4150_e2671, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4150_e2673;
        var_tmf2_dn0 = assign4150_e2673_d_n0;
        var_tmf2_dn2 = assign4150_e2673_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4160_e2689, assign4160_e2689_d_n0, assign4160_e2689_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let (assign4160_e2687, assign4160_e2687_d_n0, assign4160_e2687_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign4160_e2686: f64 = (-var_tmf2);
                (assign4160_e2686, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign4160_e2687, assign4160_e2687_d_n0, assign4160_e2687_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4160_e2689;
        var_tmf2_dn0 = assign4160_e2689_d_n0;
        var_tmf2_dn2 = assign4160_e2689_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4170_e2704, assign4170_e2704_d_n0, assign4170_e2704_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4170_e2699: f64 = (var_tmf1 * var_tmf1);
        let assign4170_e2701: f64 = (assign4170_e2699 + var_tmf2);
        let assign4170_e2702: f64 = (assign4170_e2701).sqrt();
        (assign4170_e2702, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign4170_e2702)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign4170_e2702)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4170_e2704;
        var_tmf2_dn0 = assign4170_e2704_d_n0;
        var_tmf2_dn2 = assign4170_e2704_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4180_e2720, assign4180_e2720_d_n0, assign4180_e2720_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4180_e2716: f64 = (var_tmf1 + var_tmf2);
        let assign4180_e2717: f64 = (0.5 * assign4180_e2716);
        let assign4180_e2718: f64 = (p.p85 - assign4180_e2717);
        (assign4180_e2718, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign4180_e2720;
        var_nj0_dn0 = assign4180_e2720_d_n0;
        var_nj0_dn2 = assign4180_e2720_d_n2;
        var_nj0_rv = 0.0;

        let (assign4190_e2734, assign4190_e2734_d_n0, assign4190_e2734_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4190_e2730: f64 = (var_nj0 - var_nfabot_i);
        let assign4190_e2732: f64 = (assign4190_e2730 - 0.01);
        (assign4190_e2732, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign4190_e2734;
        var_tmf1_dn0 = assign4190_e2734_d_n0;
        var_tmf1_dn2 = assign4190_e2734_d_n2;
        var_tmf1_rv = 0.0;

        let (assign4200_e2748, assign4200_e2748_d_n0, assign4200_e2748_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4200_e2744: f64 = (4.0 * var_nfabot_i);
        let assign4200_e2746: f64 = (assign4200_e2744 * 0.01);
        (assign4200_e2746, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4200_e2748;
        var_tmf2_dn0 = assign4200_e2748_d_n0;
        var_tmf2_dn2 = assign4200_e2748_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4210_e2764, assign4210_e2764_d_n0, assign4210_e2764_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let (assign4210_e2762, assign4210_e2762_d_n0, assign4210_e2762_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign4210_e2761: f64 = (-var_tmf2);
                (assign4210_e2761, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign4210_e2762, assign4210_e2762_d_n0, assign4210_e2762_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4210_e2764;
        var_tmf2_dn0 = assign4210_e2764_d_n0;
        var_tmf2_dn2 = assign4210_e2764_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4220_e2779, assign4220_e2779_d_n0, assign4220_e2779_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4220_e2774: f64 = (var_tmf1 * var_tmf1);
        let assign4220_e2776: f64 = (assign4220_e2774 + var_tmf2);
        let assign4220_e2777: f64 = (assign4220_e2776).sqrt();
        (assign4220_e2777, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign4220_e2777)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign4220_e2777)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4220_e2779;
        var_tmf2_dn0 = assign4220_e2779_d_n0;
        var_tmf2_dn2 = assign4220_e2779_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4230_e2795, assign4230_e2795_d_n0, assign4230_e2795_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 != 0.0)) {
        let assign4230_e2791: f64 = (var_tmf1 + var_tmf2);
        let assign4230_e2792: f64 = (0.5 * assign4230_e2791);
        let assign4230_e2793: f64 = (var_nfabot_i + assign4230_e2792);
        (assign4230_e2793, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign4230_e2795;
        var_nj0_dn0 = assign4230_e2795_d_n0;
        var_nj0_dn2 = assign4230_e2795_d_n2;
        var_nj0_rv = 0.0;

        let (assign4240_e2806, assign4240_e2806_d_n0, assign4240_e2806_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign4240_e2806;
        var_nj0_dn0 = assign4240_e2806_d_n0;
        var_nj0_dn2 = assign4240_e2806_d_n2;
        var_nj0_rv = 0.0;

        let (assign4250_e2817, assign4250_e2817_d_n0, assign4250_e2817_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard80 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign4250_e2817;
        var_nj1_dn0 = assign4250_e2817_d_n0;
        var_nj1_dn2 = assign4250_e2817_d_n2;
        var_nj1_rv = 0.0;

        let assign4260_e2821: f64 = (var_v1 / var_nj1);
        let assign4260_e2825: f64 = (var_nj1 - var_nj0);
        let assign4260_e2826: f64 = (var_vha1 * assign4260_e2825);
        let assign4260_e2829: f64 = (var_nj0 * p.p85);
        let assign4260_e2830: f64 = (assign4260_e2826 / assign4260_e2829);
        let assign4260_e2831: f64 = (assign4260_e2821 + assign4260_e2830);
        let assign4260_e2832: f64 = (var_phitdinv * assign4260_e2831);
        let assign4260_e2833: f64 = (assign4260_e2832).abs();
        let assign4260_e2835: f64 = if assign4260_e2833 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard81 = assign4260_e2835;
        var_guard81_rv = 0.0;

        let (assign4270_e2860, assign4270_e2860_d_n0, assign4270_e2860_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard81 != 0.0)) {
        let assign4270_e2846: f64 = (var_v1 / var_nj1);
        let assign4270_e2850: f64 = (var_nj1 - var_nj0);
        let assign4270_e2851: f64 = (var_vha1 * assign4270_e2850);
        let assign4270_e2854: f64 = (var_nj0 * p.p85);
        let assign4270_e2855: f64 = (assign4270_e2851 / assign4270_e2854);
        let assign4270_e2856: f64 = (assign4270_e2846 + assign4270_e2855);
        let assign4270_e2857: f64 = (var_phitdinv * assign4270_e2856);
        let assign4270_e2858: f64 = (assign4270_e2857).exp();
        (assign4270_e2858, (assign4270_e2858 * (var_phitdinv * ((-((var_v1 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign4270_e2854) - (assign4270_e2851 * (var_nj0_dn0 * p.p85))) / (assign4270_e2854 * assign4270_e2854))))), (assign4270_e2858 * (var_phitdinv * ((-((var_v1 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign4270_e2854) - (assign4270_e2851 * (var_nj0_dn2 * p.p85))) / (assign4270_e2854 * assign4270_e2854))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign4270_e2860;
        var_idmultbot_dn0 = assign4270_e2860_d_n0;
        var_idmultbot_dn2 = assign4270_e2860_d_n2;
        var_idmultbot_rv = 0.0;

        let assign4280_e2864: f64 = (var_v1 / var_nj1);
        let assign4280_e2868: f64 = (var_nj1 - var_nj0);
        let assign4280_e2869: f64 = (var_vha1 * assign4280_e2868);
        let assign4280_e2872: f64 = (var_nj0 * p.p85);
        let assign4280_e2873: f64 = (assign4280_e2869 / assign4280_e2872);
        let assign4280_e2874: f64 = (assign4280_e2864 + assign4280_e2873);
        let assign4280_e2875: f64 = (var_phitdinv * assign4280_e2874);
        let assign4280_e2877: f64 = (-230.25850929940458);
        let assign4280_e2878: f64 = if assign4280_e2875 < assign4280_e2877 { 1.0 } else { 0.0 };
        var_guard82 = assign4280_e2878;
        var_guard82_rv = 0.0;

        let (assign4290_e2958, assign4290_e2958_d_n0, assign4290_e2958_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard81 == 0.0)) && (var_guard82 != 0.0)) {
        let assign4290_e2892: f64 = (-230.25850929940458);
        let assign4290_e2896: f64 = (var_v1 / var_nj1);
        let assign4290_e2900: f64 = (var_nj1 - var_nj0);
        let assign4290_e2901: f64 = (var_vha1 * assign4290_e2900);
        let assign4290_e2904: f64 = (var_nj0 * p.p85);
        let assign4290_e2905: f64 = (assign4290_e2901 / assign4290_e2904);
        let assign4290_e2906: f64 = (assign4290_e2896 + assign4290_e2905);
        let assign4290_e2907: f64 = (var_phitdinv * assign4290_e2906);
        let assign4290_e2908: f64 = (assign4290_e2892 - assign4290_e2907);
        let assign4290_e2912: f64 = (-230.25850929940458);
        let assign4290_e2916: f64 = (var_v1 / var_nj1);
        let assign4290_e2920: f64 = (var_nj1 - var_nj0);
        let assign4290_e2921: f64 = (var_vha1 * assign4290_e2920);
        let assign4290_e2924: f64 = (var_nj0 * p.p85);
        let assign4290_e2925: f64 = (assign4290_e2921 / assign4290_e2924);
        let assign4290_e2926: f64 = (assign4290_e2916 + assign4290_e2925);
        let assign4290_e2927: f64 = (var_phitdinv * assign4290_e2926);
        let assign4290_e2928: f64 = (assign4290_e2912 - assign4290_e2927);
        let assign4290_e2931: f64 = (-230.25850929940458);
        let assign4290_e2935: f64 = (var_v1 / var_nj1);
        let assign4290_e2939: f64 = (var_nj1 - var_nj0);
        let assign4290_e2940: f64 = (var_vha1 * assign4290_e2939);
        let assign4290_e2943: f64 = (var_nj0 * p.p85);
        let assign4290_e2944: f64 = (assign4290_e2940 / assign4290_e2943);
        let assign4290_e2945: f64 = (assign4290_e2935 + assign4290_e2944);
        let assign4290_e2946: f64 = (var_phitdinv * assign4290_e2945);
        let assign4290_e2947: f64 = (assign4290_e2931 - assign4290_e2946);
        let assign4290_e2949: f64 = (assign4290_e2947 * 0.3333333333333333);
        let assign4290_e2950: f64 = (1.0 + assign4290_e2949);
        let assign4290_e2951: f64 = (assign4290_e2928 * assign4290_e2950);
        let assign4290_e2952: f64 = (0.5 * assign4290_e2951);
        let assign4290_e2953: f64 = (1.0 + assign4290_e2952);
        let assign4290_e2954: f64 = (assign4290_e2908 * assign4290_e2953);
        let assign4290_e2955: f64 = (1.0 + assign4290_e2954);
        let assign4290_e2956: f64 = (1e-100 / assign4290_e2955);
        (assign4290_e2956, (-((1e-100 * (((-(var_phitdinv * ((-((var_v1 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign4290_e2904) - (assign4290_e2901 * (var_nj0_dn0 * p.p85))) / (assign4290_e2904 * assign4290_e2904))))) * assign4290_e2953) + (assign4290_e2908 * (0.5 * (((-(var_phitdinv * ((-((var_v1 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign4290_e2924) - (assign4290_e2921 * (var_nj0_dn0 * p.p85))) / (assign4290_e2924 * assign4290_e2924))))) * assign4290_e2950) + (assign4290_e2928 * ((-(var_phitdinv * ((-((var_v1 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign4290_e2943) - (assign4290_e2940 * (var_nj0_dn0 * p.p85))) / (assign4290_e2943 * assign4290_e2943))))) * 0.3333333333333333))))))) / (assign4290_e2955 * assign4290_e2955))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v1 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign4290_e2904) - (assign4290_e2901 * (var_nj0_dn2 * p.p85))) / (assign4290_e2904 * assign4290_e2904))))) * assign4290_e2953) + (assign4290_e2908 * (0.5 * (((-(var_phitdinv * ((-((var_v1 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign4290_e2924) - (assign4290_e2921 * (var_nj0_dn2 * p.p85))) / (assign4290_e2924 * assign4290_e2924))))) * assign4290_e2950) + (assign4290_e2928 * ((-(var_phitdinv * ((-((var_v1 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign4290_e2943) - (assign4290_e2940 * (var_nj0_dn2 * p.p85))) / (assign4290_e2943 * assign4290_e2943))))) * 0.3333333333333333))))))) / (assign4290_e2955 * assign4290_e2955))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign4290_e2958;
        var_idmultbot_dn0 = assign4290_e2958_d_n0;
        var_idmultbot_dn2 = assign4290_e2958_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign4300_e3036, assign4300_e3036_d_n0, assign4300_e3036_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard81 == 0.0)) && (var_guard82 == 0.0)) {
        let assign4300_e2975: f64 = (var_v1 / var_nj1);
        let assign4300_e2979: f64 = (var_nj1 - var_nj0);
        let assign4300_e2980: f64 = (var_vha1 * assign4300_e2979);
        let assign4300_e2983: f64 = (var_nj0 * p.p85);
        let assign4300_e2984: f64 = (assign4300_e2980 / assign4300_e2983);
        let assign4300_e2985: f64 = (assign4300_e2975 + assign4300_e2984);
        let assign4300_e2986: f64 = (var_phitdinv * assign4300_e2985);
        let assign4300_e2988: f64 = (assign4300_e2986 - 230.25850929940458);
        let assign4300_e2994: f64 = (var_v1 / var_nj1);
        let assign4300_e2998: f64 = (var_nj1 - var_nj0);
        let assign4300_e2999: f64 = (var_vha1 * assign4300_e2998);
        let assign4300_e3002: f64 = (var_nj0 * p.p85);
        let assign4300_e3003: f64 = (assign4300_e2999 / assign4300_e3002);
        let assign4300_e3004: f64 = (assign4300_e2994 + assign4300_e3003);
        let assign4300_e3005: f64 = (var_phitdinv * assign4300_e3004);
        let assign4300_e3007: f64 = (assign4300_e3005 - 230.25850929940458);
        let assign4300_e3012: f64 = (var_v1 / var_nj1);
        let assign4300_e3016: f64 = (var_nj1 - var_nj0);
        let assign4300_e3017: f64 = (var_vha1 * assign4300_e3016);
        let assign4300_e3020: f64 = (var_nj0 * p.p85);
        let assign4300_e3021: f64 = (assign4300_e3017 / assign4300_e3020);
        let assign4300_e3022: f64 = (assign4300_e3012 + assign4300_e3021);
        let assign4300_e3023: f64 = (var_phitdinv * assign4300_e3022);
        let assign4300_e3025: f64 = (assign4300_e3023 - 230.25850929940458);
        let assign4300_e3027: f64 = (assign4300_e3025 * 0.3333333333333333);
        let assign4300_e3028: f64 = (1.0 + assign4300_e3027);
        let assign4300_e3029: f64 = (assign4300_e3007 * assign4300_e3028);
        let assign4300_e3030: f64 = (0.5 * assign4300_e3029);
        let assign4300_e3031: f64 = (1.0 + assign4300_e3030);
        let assign4300_e3032: f64 = (assign4300_e2988 * assign4300_e3031);
        let assign4300_e3033: f64 = (1.0 + assign4300_e3032);
        let assign4300_e3034: f64 = (1e100 * assign4300_e3033);
        (assign4300_e3034, (1e100 * (((var_phitdinv * ((-((var_v1 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign4300_e2983) - (assign4300_e2980 * (var_nj0_dn0 * p.p85))) / (assign4300_e2983 * assign4300_e2983)))) * assign4300_e3031) + (assign4300_e2988 * (0.5 * (((var_phitdinv * ((-((var_v1 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign4300_e3002) - (assign4300_e2999 * (var_nj0_dn0 * p.p85))) / (assign4300_e3002 * assign4300_e3002)))) * assign4300_e3028) + (assign4300_e3007 * ((var_phitdinv * ((-((var_v1 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign4300_e3020) - (assign4300_e3017 * (var_nj0_dn0 * p.p85))) / (assign4300_e3020 * assign4300_e3020)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v1 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign4300_e2983) - (assign4300_e2980 * (var_nj0_dn2 * p.p85))) / (assign4300_e2983 * assign4300_e2983)))) * assign4300_e3031) + (assign4300_e2988 * (0.5 * (((var_phitdinv * ((-((var_v1 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign4300_e3002) - (assign4300_e2999 * (var_nj0_dn2 * p.p85))) / (assign4300_e3002 * assign4300_e3002)))) * assign4300_e3028) + (assign4300_e3007 * ((var_phitdinv * ((-((var_v1 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign4300_e3020) - (assign4300_e3017 * (var_nj0_dn2 * p.p85))) / (assign4300_e3020 * assign4300_e3020)))) * 0.3333333333333333))))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign4300_e3036;
        var_idmultbot_dn0 = assign4300_e3036_d_n0;
        var_idmultbot_dn2 = assign4300_e3036_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign4310_e3048,) = {
    if (((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) {
        let assign4310_e3044: f64 = (var_nin * var_nin);
        let assign4310_e3046: f64 = (assign4310_e3044 / var_ndisti_i);
        (assign4310_e3046,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign4310_e3048;
        var_pnn0_rv = 0.0;

        let (assign4320_e3063,) = {
    if (((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) {
        let assign4320_e3056: f64 = (var_nfasti_i / var_phitdinv);
        let assign4320_e3059: f64 = (var_ndisti_i / var_pnn0);
        let assign4320_e3060: f64 = (assign4320_e3059).ln();
        let assign4320_e3061: f64 = (assign4320_e3056 * assign4320_e3060);
        (assign4320_e3061,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign4320_e3063;
        var_vha1_rv = 0.0;

        let assign4330_e3066: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard83 = assign4330_e3066;
        var_guard83_rv = 0.0;

        let (assign4340_e3082, assign4340_e3082_d_n0, assign4340_e3082_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4340_e3077: f64 = (var_v1 - var_vha1);
        let assign4340_e3078: f64 = (p.p86 * assign4340_e3077);
        let assign4340_e3080: f64 = (assign4340_e3078 + var_nfasti_i);
        (assign4340_e3080, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign4340_e3082;
        var_nja10_dn0 = assign4340_e3082_d_n0;
        var_nja10_dn2 = assign4340_e3082_d_n2;
        var_nja10_rv = 0.0;

        let (assign4350_e3096, assign4350_e3096_d_n0, assign4350_e3096_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4350_e3093: f64 = (p.p86 * var_vha1);
        let assign4350_e3094: f64 = (var_nfasti_i - assign4350_e3093);
        (assign4350_e3094, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign4350_e3096;
        var_nj0_dn0 = assign4350_e3096_d_n0;
        var_nj0_dn2 = assign4350_e3096_d_n2;
        var_nj0_rv = 0.0;

        let (assign4360_e3110, assign4360_e3110_d_n0, assign4360_e3110_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4360_e3106: f64 = (p.p85 - var_nja10);
        let assign4360_e3108: f64 = (assign4360_e3106 - 0.01);
        (assign4360_e3108, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign4360_e3110;
        var_tmf1_dn0 = assign4360_e3110_d_n0;
        var_tmf1_dn2 = assign4360_e3110_d_n2;
        var_tmf1_rv = 0.0;

        let (assign4370_e3124, assign4370_e3124_d_n0, assign4370_e3124_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4370_e3120: f64 = (4.0 * p.p85);
        let assign4370_e3122: f64 = (assign4370_e3120 * 0.01);
        (assign4370_e3122, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4370_e3124;
        var_tmf2_dn0 = assign4370_e3124_d_n0;
        var_tmf2_dn2 = assign4370_e3124_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4380_e3140, assign4380_e3140_d_n0, assign4380_e3140_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let (assign4380_e3138, assign4380_e3138_d_n0, assign4380_e3138_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign4380_e3137: f64 = (-var_tmf2);
                (assign4380_e3137, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign4380_e3138, assign4380_e3138_d_n0, assign4380_e3138_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4380_e3140;
        var_tmf2_dn0 = assign4380_e3140_d_n0;
        var_tmf2_dn2 = assign4380_e3140_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4390_e3155, assign4390_e3155_d_n0, assign4390_e3155_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4390_e3150: f64 = (var_tmf1 * var_tmf1);
        let assign4390_e3152: f64 = (assign4390_e3150 + var_tmf2);
        let assign4390_e3153: f64 = (assign4390_e3152).sqrt();
        (assign4390_e3153, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign4390_e3153)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign4390_e3153)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4390_e3155;
        var_tmf2_dn0 = assign4390_e3155_d_n0;
        var_tmf2_dn2 = assign4390_e3155_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4400_e3171, assign4400_e3171_d_n0, assign4400_e3171_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4400_e3167: f64 = (var_tmf1 + var_tmf2);
        let assign4400_e3168: f64 = (0.5 * assign4400_e3167);
        let assign4400_e3169: f64 = (p.p85 - assign4400_e3168);
        (assign4400_e3169, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign4400_e3171;
        var_nja11_dn0 = assign4400_e3171_d_n0;
        var_nja11_dn2 = assign4400_e3171_d_n2;
        var_nja11_rv = 0.0;

        let (assign4410_e3185, assign4410_e3185_d_n0, assign4410_e3185_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4410_e3181: f64 = (var_nja11 - var_nfasti_i);
        let assign4410_e3183: f64 = (assign4410_e3181 - 0.01);
        (assign4410_e3183, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign4410_e3185;
        var_tmf1_dn0 = assign4410_e3185_d_n0;
        var_tmf1_dn2 = assign4410_e3185_d_n2;
        var_tmf1_rv = 0.0;

        let (assign4420_e3199, assign4420_e3199_d_n0, assign4420_e3199_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4420_e3195: f64 = (4.0 * var_nfasti_i);
        let assign4420_e3197: f64 = (assign4420_e3195 * 0.01);
        (assign4420_e3197, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4420_e3199;
        var_tmf2_dn0 = assign4420_e3199_d_n0;
        var_tmf2_dn2 = assign4420_e3199_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4430_e3215, assign4430_e3215_d_n0, assign4430_e3215_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let (assign4430_e3213, assign4430_e3213_d_n0, assign4430_e3213_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign4430_e3212: f64 = (-var_tmf2);
                (assign4430_e3212, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign4430_e3213, assign4430_e3213_d_n0, assign4430_e3213_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4430_e3215;
        var_tmf2_dn0 = assign4430_e3215_d_n0;
        var_tmf2_dn2 = assign4430_e3215_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4440_e3230, assign4440_e3230_d_n0, assign4440_e3230_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4440_e3225: f64 = (var_tmf1 * var_tmf1);
        let assign4440_e3227: f64 = (assign4440_e3225 + var_tmf2);
        let assign4440_e3228: f64 = (assign4440_e3227).sqrt();
        (assign4440_e3228, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign4440_e3228)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign4440_e3228)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4440_e3230;
        var_tmf2_dn0 = assign4440_e3230_d_n0;
        var_tmf2_dn2 = assign4440_e3230_d_n2;
        var_tmf2_rv = 0.0;

        *var_guard81_slot = var_guard81;
        *var_guard81_rv_slot = var_guard81_rv;
        *var_guard82_slot = var_guard82;
        *var_guard82_rv_slot = var_guard82_rv;
        *var_guard83_slot = var_guard83;
        *var_guard83_rv_slot = var_guard83_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        var_guard31: f64,
        var_guard76: f64,
        var_guard77: f64,
        var_guard83: f64,
        var_ndigat_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v1: f64,
        var_guard86_slot: &mut f64,
        var_guard86_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_guard86: f64 = *var_guard86_slot;
        let mut var_guard86_rv: f64 = *var_guard86_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign4450_e3246, assign4450_e3246_d_n0, assign4450_e3246_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4450_e3242: f64 = (var_tmf1 + var_tmf2);
        let assign4450_e3243: f64 = (0.5 * assign4450_e3242);
        let assign4450_e3244: f64 = (var_nfasti_i + assign4450_e3243);
        (assign4450_e3244, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign4450_e3246;
        var_nj1_dn0 = assign4450_e3246_d_n0;
        var_nj1_dn2 = assign4450_e3246_d_n2;
        var_nj1_rv = 0.0;

        let (assign4460_e3260, assign4460_e3260_d_n0, assign4460_e3260_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4460_e3256: f64 = (p.p85 - var_nj0);
        let assign4460_e3258: f64 = (assign4460_e3256 - 0.01);
        (assign4460_e3258, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign4460_e3260;
        var_tmf1_dn0 = assign4460_e3260_d_n0;
        var_tmf1_dn2 = assign4460_e3260_d_n2;
        var_tmf1_rv = 0.0;

        let (assign4470_e3274, assign4470_e3274_d_n0, assign4470_e3274_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4470_e3270: f64 = (4.0 * p.p85);
        let assign4470_e3272: f64 = (assign4470_e3270 * 0.01);
        (assign4470_e3272, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4470_e3274;
        var_tmf2_dn0 = assign4470_e3274_d_n0;
        var_tmf2_dn2 = assign4470_e3274_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4480_e3290, assign4480_e3290_d_n0, assign4480_e3290_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let (assign4480_e3288, assign4480_e3288_d_n0, assign4480_e3288_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign4480_e3287: f64 = (-var_tmf2);
                (assign4480_e3287, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign4480_e3288, assign4480_e3288_d_n0, assign4480_e3288_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4480_e3290;
        var_tmf2_dn0 = assign4480_e3290_d_n0;
        var_tmf2_dn2 = assign4480_e3290_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4490_e3305, assign4490_e3305_d_n0, assign4490_e3305_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4490_e3300: f64 = (var_tmf1 * var_tmf1);
        let assign4490_e3302: f64 = (assign4490_e3300 + var_tmf2);
        let assign4490_e3303: f64 = (assign4490_e3302).sqrt();
        (assign4490_e3303, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign4490_e3303)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign4490_e3303)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4490_e3305;
        var_tmf2_dn0 = assign4490_e3305_d_n0;
        var_tmf2_dn2 = assign4490_e3305_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4500_e3321, assign4500_e3321_d_n0, assign4500_e3321_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4500_e3317: f64 = (var_tmf1 + var_tmf2);
        let assign4500_e3318: f64 = (0.5 * assign4500_e3317);
        let assign4500_e3319: f64 = (p.p85 - assign4500_e3318);
        (assign4500_e3319, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign4500_e3321;
        var_nj0_dn0 = assign4500_e3321_d_n0;
        var_nj0_dn2 = assign4500_e3321_d_n2;
        var_nj0_rv = 0.0;

        let (assign4510_e3335, assign4510_e3335_d_n0, assign4510_e3335_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4510_e3331: f64 = (var_nj0 - var_nfasti_i);
        let assign4510_e3333: f64 = (assign4510_e3331 - 0.01);
        (assign4510_e3333, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign4510_e3335;
        var_tmf1_dn0 = assign4510_e3335_d_n0;
        var_tmf1_dn2 = assign4510_e3335_d_n2;
        var_tmf1_rv = 0.0;

        let (assign4520_e3349, assign4520_e3349_d_n0, assign4520_e3349_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4520_e3345: f64 = (4.0 * var_nfasti_i);
        let assign4520_e3347: f64 = (assign4520_e3345 * 0.01);
        (assign4520_e3347, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4520_e3349;
        var_tmf2_dn0 = assign4520_e3349_d_n0;
        var_tmf2_dn2 = assign4520_e3349_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4530_e3365, assign4530_e3365_d_n0, assign4530_e3365_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let (assign4530_e3363, assign4530_e3363_d_n0, assign4530_e3363_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign4530_e3362: f64 = (-var_tmf2);
                (assign4530_e3362, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign4530_e3363, assign4530_e3363_d_n0, assign4530_e3363_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4530_e3365;
        var_tmf2_dn0 = assign4530_e3365_d_n0;
        var_tmf2_dn2 = assign4530_e3365_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4540_e3380, assign4540_e3380_d_n0, assign4540_e3380_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4540_e3375: f64 = (var_tmf1 * var_tmf1);
        let assign4540_e3377: f64 = (assign4540_e3375 + var_tmf2);
        let assign4540_e3378: f64 = (assign4540_e3377).sqrt();
        (assign4540_e3378, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign4540_e3378)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign4540_e3378)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4540_e3380;
        var_tmf2_dn0 = assign4540_e3380_d_n0;
        var_tmf2_dn2 = assign4540_e3380_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4550_e3396, assign4550_e3396_d_n0, assign4550_e3396_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 != 0.0)) {
        let assign4550_e3392: f64 = (var_tmf1 + var_tmf2);
        let assign4550_e3393: f64 = (0.5 * assign4550_e3392);
        let assign4550_e3394: f64 = (var_nfasti_i + assign4550_e3393);
        (assign4550_e3394, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign4550_e3396;
        var_nj0_dn0 = assign4550_e3396_d_n0;
        var_nj0_dn2 = assign4550_e3396_d_n2;
        var_nj0_rv = 0.0;

        let (assign4560_e3407, assign4560_e3407_d_n0, assign4560_e3407_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign4560_e3407;
        var_nj0_dn0 = assign4560_e3407_d_n0;
        var_nj0_dn2 = assign4560_e3407_d_n2;
        var_nj0_rv = 0.0;

        let (assign4570_e3418, assign4570_e3418_d_n0, assign4570_e3418_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard83 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign4570_e3418;
        var_nj1_dn0 = assign4570_e3418_d_n0;
        var_nj1_dn2 = assign4570_e3418_d_n2;
        var_nj1_rv = 0.0;

        let (assign4630_e3649,) = {
    if (((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) {
        let assign4630_e3645: f64 = (var_nin * var_nin);
        let assign4630_e3647: f64 = (assign4630_e3645 / var_ndigat_i);
        (assign4630_e3647,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign4630_e3649;
        var_pnn0_rv = 0.0;

        let (assign4640_e3664,) = {
    if (((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) {
        let assign4640_e3657: f64 = (var_nfagat_i / var_phitdinv);
        let assign4640_e3660: f64 = (var_ndigat_i / var_pnn0);
        let assign4640_e3661: f64 = (assign4640_e3660).ln();
        let assign4640_e3662: f64 = (assign4640_e3657 * assign4640_e3661);
        (assign4640_e3662,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign4640_e3664;
        var_vha1_rv = 0.0;

        let assign4650_e3667: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard86 = assign4650_e3667;
        var_guard86_rv = 0.0;

        let (assign4660_e3683, assign4660_e3683_d_n0, assign4660_e3683_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4660_e3678: f64 = (var_v1 - var_vha1);
        let assign4660_e3679: f64 = (p.p86 * assign4660_e3678);
        let assign4660_e3681: f64 = (assign4660_e3679 + var_nfagat_i);
        (assign4660_e3681, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign4660_e3683;
        var_nja10_dn0 = assign4660_e3683_d_n0;
        var_nja10_dn2 = assign4660_e3683_d_n2;
        var_nja10_rv = 0.0;

        let (assign4670_e3697, assign4670_e3697_d_n0, assign4670_e3697_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4670_e3694: f64 = (p.p86 * var_vha1);
        let assign4670_e3695: f64 = (var_nfagat_i - assign4670_e3694);
        (assign4670_e3695, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign4670_e3697;
        var_nj0_dn0 = assign4670_e3697_d_n0;
        var_nj0_dn2 = assign4670_e3697_d_n2;
        var_nj0_rv = 0.0;

        let (assign4680_e3711, assign4680_e3711_d_n0, assign4680_e3711_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4680_e3707: f64 = (p.p85 - var_nja10);
        let assign4680_e3709: f64 = (assign4680_e3707 - 0.01);
        (assign4680_e3709, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign4680_e3711;
        var_tmf1_dn0 = assign4680_e3711_d_n0;
        var_tmf1_dn2 = assign4680_e3711_d_n2;
        var_tmf1_rv = 0.0;

        let (assign4690_e3725, assign4690_e3725_d_n0, assign4690_e3725_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4690_e3721: f64 = (4.0 * p.p85);
        let assign4690_e3723: f64 = (assign4690_e3721 * 0.01);
        (assign4690_e3723, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4690_e3725;
        var_tmf2_dn0 = assign4690_e3725_d_n0;
        var_tmf2_dn2 = assign4690_e3725_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4700_e3741, assign4700_e3741_d_n0, assign4700_e3741_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let (assign4700_e3739, assign4700_e3739_d_n0, assign4700_e3739_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign4700_e3738: f64 = (-var_tmf2);
                (assign4700_e3738, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign4700_e3739, assign4700_e3739_d_n0, assign4700_e3739_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4700_e3741;
        var_tmf2_dn0 = assign4700_e3741_d_n0;
        var_tmf2_dn2 = assign4700_e3741_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4710_e3756, assign4710_e3756_d_n0, assign4710_e3756_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4710_e3751: f64 = (var_tmf1 * var_tmf1);
        let assign4710_e3753: f64 = (assign4710_e3751 + var_tmf2);
        let assign4710_e3754: f64 = (assign4710_e3753).sqrt();
        (assign4710_e3754, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign4710_e3754)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign4710_e3754)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4710_e3756;
        var_tmf2_dn0 = assign4710_e3756_d_n0;
        var_tmf2_dn2 = assign4710_e3756_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4720_e3772, assign4720_e3772_d_n0, assign4720_e3772_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4720_e3768: f64 = (var_tmf1 + var_tmf2);
        let assign4720_e3769: f64 = (0.5 * assign4720_e3768);
        let assign4720_e3770: f64 = (p.p85 - assign4720_e3769);
        (assign4720_e3770, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign4720_e3772;
        var_nja11_dn0 = assign4720_e3772_d_n0;
        var_nja11_dn2 = assign4720_e3772_d_n2;
        var_nja11_rv = 0.0;

        let (assign4730_e3786, assign4730_e3786_d_n0, assign4730_e3786_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4730_e3782: f64 = (var_nja11 - var_nfagat_i);
        let assign4730_e3784: f64 = (assign4730_e3782 - 0.01);
        (assign4730_e3784, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign4730_e3786;
        var_tmf1_dn0 = assign4730_e3786_d_n0;
        var_tmf1_dn2 = assign4730_e3786_d_n2;
        var_tmf1_rv = 0.0;

        let (assign4740_e3800, assign4740_e3800_d_n0, assign4740_e3800_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4740_e3796: f64 = (4.0 * var_nfagat_i);
        let assign4740_e3798: f64 = (assign4740_e3796 * 0.01);
        (assign4740_e3798, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4740_e3800;
        var_tmf2_dn0 = assign4740_e3800_d_n0;
        var_tmf2_dn2 = assign4740_e3800_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4750_e3816, assign4750_e3816_d_n0, assign4750_e3816_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let (assign4750_e3814, assign4750_e3814_d_n0, assign4750_e3814_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign4750_e3813: f64 = (-var_tmf2);
                (assign4750_e3813, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign4750_e3814, assign4750_e3814_d_n0, assign4750_e3814_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4750_e3816;
        var_tmf2_dn0 = assign4750_e3816_d_n0;
        var_tmf2_dn2 = assign4750_e3816_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4760_e3831, assign4760_e3831_d_n0, assign4760_e3831_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4760_e3826: f64 = (var_tmf1 * var_tmf1);
        let assign4760_e3828: f64 = (assign4760_e3826 + var_tmf2);
        let assign4760_e3829: f64 = (assign4760_e3828).sqrt();
        (assign4760_e3829, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign4760_e3829)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign4760_e3829)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4760_e3831;
        var_tmf2_dn0 = assign4760_e3831_d_n0;
        var_tmf2_dn2 = assign4760_e3831_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4770_e3847, assign4770_e3847_d_n0, assign4770_e3847_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4770_e3843: f64 = (var_tmf1 + var_tmf2);
        let assign4770_e3844: f64 = (0.5 * assign4770_e3843);
        let assign4770_e3845: f64 = (var_nfagat_i + assign4770_e3844);
        (assign4770_e3845, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign4770_e3847;
        var_nj1_dn0 = assign4770_e3847_d_n0;
        var_nj1_dn2 = assign4770_e3847_d_n2;
        var_nj1_rv = 0.0;

        let (assign4780_e3861, assign4780_e3861_d_n0, assign4780_e3861_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4780_e3857: f64 = (p.p85 - var_nj0);
        let assign4780_e3859: f64 = (assign4780_e3857 - 0.01);
        (assign4780_e3859, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign4780_e3861;
        var_tmf1_dn0 = assign4780_e3861_d_n0;
        var_tmf1_dn2 = assign4780_e3861_d_n2;
        var_tmf1_rv = 0.0;

        let (assign4790_e3875, assign4790_e3875_d_n0, assign4790_e3875_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4790_e3871: f64 = (4.0 * p.p85);
        let assign4790_e3873: f64 = (assign4790_e3871 * 0.01);
        (assign4790_e3873, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4790_e3875;
        var_tmf2_dn0 = assign4790_e3875_d_n0;
        var_tmf2_dn2 = assign4790_e3875_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4800_e3891, assign4800_e3891_d_n0, assign4800_e3891_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let (assign4800_e3889, assign4800_e3889_d_n0, assign4800_e3889_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign4800_e3888: f64 = (-var_tmf2);
                (assign4800_e3888, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign4800_e3889, assign4800_e3889_d_n0, assign4800_e3889_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4800_e3891;
        var_tmf2_dn0 = assign4800_e3891_d_n0;
        var_tmf2_dn2 = assign4800_e3891_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4810_e3906, assign4810_e3906_d_n0, assign4810_e3906_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4810_e3901: f64 = (var_tmf1 * var_tmf1);
        let assign4810_e3903: f64 = (assign4810_e3901 + var_tmf2);
        let assign4810_e3904: f64 = (assign4810_e3903).sqrt();
        (assign4810_e3904, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign4810_e3904)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign4810_e3904)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4810_e3906;
        var_tmf2_dn0 = assign4810_e3906_d_n0;
        var_tmf2_dn2 = assign4810_e3906_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4820_e3922, assign4820_e3922_d_n0, assign4820_e3922_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4820_e3918: f64 = (var_tmf1 + var_tmf2);
        let assign4820_e3919: f64 = (0.5 * assign4820_e3918);
        let assign4820_e3920: f64 = (p.p85 - assign4820_e3919);
        (assign4820_e3920, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign4820_e3922;
        var_nj0_dn0 = assign4820_e3922_d_n0;
        var_nj0_dn2 = assign4820_e3922_d_n2;
        var_nj0_rv = 0.0;

        let (assign4830_e3936, assign4830_e3936_d_n0, assign4830_e3936_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4830_e3932: f64 = (var_nj0 - var_nfagat_i);
        let assign4830_e3934: f64 = (assign4830_e3932 - 0.01);
        (assign4830_e3934, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign4830_e3936;
        var_tmf1_dn0 = assign4830_e3936_d_n0;
        var_tmf1_dn2 = assign4830_e3936_d_n2;
        var_tmf1_rv = 0.0;

        *var_guard86_slot = var_guard86;
        *var_guard86_rv_slot = var_guard86_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        var_guard31: f64,
        var_guard76: f64,
        var_guard77: f64,
        var_guard86: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nfagat_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_rv_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_guard89_slot: &mut f64,
        var_guard89_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_rv: f64 = *var_dfn_su_rv_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_guard89: f64 = *var_guard89_slot;
        let mut var_guard89_rv: f64 = *var_guard89_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign4840_e3950, assign4840_e3950_d_n0, assign4840_e3950_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4840_e3946: f64 = (4.0 * var_nfagat_i);
        let assign4840_e3948: f64 = (assign4840_e3946 * 0.01);
        (assign4840_e3948, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4840_e3950;
        var_tmf2_dn0 = assign4840_e3950_d_n0;
        var_tmf2_dn2 = assign4840_e3950_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4850_e3966, assign4850_e3966_d_n0, assign4850_e3966_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let (assign4850_e3964, assign4850_e3964_d_n0, assign4850_e3964_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign4850_e3963: f64 = (-var_tmf2);
                (assign4850_e3963, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign4850_e3964, assign4850_e3964_d_n0, assign4850_e3964_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4850_e3966;
        var_tmf2_dn0 = assign4850_e3966_d_n0;
        var_tmf2_dn2 = assign4850_e3966_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4860_e3981, assign4860_e3981_d_n0, assign4860_e3981_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4860_e3976: f64 = (var_tmf1 * var_tmf1);
        let assign4860_e3978: f64 = (assign4860_e3976 + var_tmf2);
        let assign4860_e3979: f64 = (assign4860_e3978).sqrt();
        (assign4860_e3979, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign4860_e3979)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign4860_e3979)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign4860_e3981;
        var_tmf2_dn0 = assign4860_e3981_d_n0;
        var_tmf2_dn2 = assign4860_e3981_d_n2;
        var_tmf2_rv = 0.0;

        let (assign4870_e3997, assign4870_e3997_d_n0, assign4870_e3997_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 != 0.0)) {
        let assign4870_e3993: f64 = (var_tmf1 + var_tmf2);
        let assign4870_e3994: f64 = (0.5 * assign4870_e3993);
        let assign4870_e3995: f64 = (var_nfagat_i + assign4870_e3994);
        (assign4870_e3995, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign4870_e3997;
        var_nj0_dn0 = assign4870_e3997_d_n0;
        var_nj0_dn2 = assign4870_e3997_d_n2;
        var_nj0_rv = 0.0;

        let (assign4880_e4008, assign4880_e4008_d_n0, assign4880_e4008_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign4880_e4008;
        var_nj0_dn0 = assign4880_e4008_d_n0;
        var_nj0_dn2 = assign4880_e4008_d_n2;
        var_nj0_rv = 0.0;

        let (assign4890_e4019, assign4890_e4019_d_n0, assign4890_e4019_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 != 0.0)) && (var_guard86 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign4890_e4019;
        var_nj1_dn0 = assign4890_e4019_d_n0;
        var_nj1_dn2 = assign4890_e4019_d_n2;
        var_nj1_rv = 0.0;

        let (assign4960_e4269,) = {
    if (((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) {
        let assign4960_e4265: f64 = (var_nin * var_nin);
        let assign4960_e4267: f64 = (assign4960_e4265 / var_ndibot_i);
        (assign4960_e4267,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign4960_e4269;
        var_pnn0_rv = 0.0;

        let (assign4970_e4285,) = {
    if (((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) {
        let assign4970_e4278: f64 = (var_nfabot_i / var_phitdinv);
        let assign4970_e4281: f64 = (var_ndibot_i / var_pnn0);
        let assign4970_e4282: f64 = (assign4970_e4281).ln();
        let assign4970_e4283: f64 = (assign4970_e4278 * assign4970_e4282);
        (assign4970_e4283,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign4970_e4285;
        var_vha1_rv = 0.0;

        let assign4980_e4288: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard89 = assign4980_e4288;
        var_guard89_rv = 0.0;

        let (assign4990_e4305, assign4990_e4305_d_n0, assign4990_e4305_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign4990_e4300: f64 = (var_vmax - var_vha1);
        let assign4990_e4301: f64 = (p.p86 * assign4990_e4300);
        let assign4990_e4303: f64 = (assign4990_e4301 + var_nfabot_i);
        (assign4990_e4303, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign4990_e4305;
        var_nja10_dn0 = assign4990_e4305_d_n0;
        var_nja10_dn2 = assign4990_e4305_d_n2;
        var_nja10_rv = 0.0;

        let (assign5000_e4320, assign5000_e4320_d_n0, assign5000_e4320_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5000_e4317: f64 = (p.p86 * var_vha1);
        let assign5000_e4318: f64 = (var_nfabot_i - assign5000_e4317);
        (assign5000_e4318, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign5000_e4320;
        var_nj0_dn0 = assign5000_e4320_d_n0;
        var_nj0_dn2 = assign5000_e4320_d_n2;
        var_nj0_rv = 0.0;

        let (assign5010_e4335, assign5010_e4335_d_n0, assign5010_e4335_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5010_e4331: f64 = (p.p85 - var_nja10);
        let assign5010_e4333: f64 = (assign5010_e4331 - 0.01);
        (assign5010_e4333, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign5010_e4335;
        var_tmf1_dn0 = assign5010_e4335_d_n0;
        var_tmf1_dn2 = assign5010_e4335_d_n2;
        var_tmf1_rv = 0.0;

        let (assign5020_e4350, assign5020_e4350_d_n0, assign5020_e4350_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5020_e4346: f64 = (4.0 * p.p85);
        let assign5020_e4348: f64 = (assign5020_e4346 * 0.01);
        (assign5020_e4348, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5020_e4350;
        var_tmf2_dn0 = assign5020_e4350_d_n0;
        var_tmf2_dn2 = assign5020_e4350_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5030_e4367, assign5030_e4367_d_n0, assign5030_e4367_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let (assign5030_e4365, assign5030_e4365_d_n0, assign5030_e4365_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign5030_e4364: f64 = (-var_tmf2);
                (assign5030_e4364, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign5030_e4365, assign5030_e4365_d_n0, assign5030_e4365_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5030_e4367;
        var_tmf2_dn0 = assign5030_e4367_d_n0;
        var_tmf2_dn2 = assign5030_e4367_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5040_e4383, assign5040_e4383_d_n0, assign5040_e4383_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5040_e4378: f64 = (var_tmf1 * var_tmf1);
        let assign5040_e4380: f64 = (assign5040_e4378 + var_tmf2);
        let assign5040_e4381: f64 = (assign5040_e4380).sqrt();
        (assign5040_e4381, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5040_e4381)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5040_e4381)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5040_e4383;
        var_tmf2_dn0 = assign5040_e4383_d_n0;
        var_tmf2_dn2 = assign5040_e4383_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5050_e4400, assign5050_e4400_d_n0, assign5050_e4400_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5050_e4396: f64 = (var_tmf1 / var_tmf2);
        let assign5050_e4397: f64 = (1.0 + assign5050_e4396);
        let assign5050_e4398: f64 = (0.5 * assign5050_e4397);
        (assign5050_e4398, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign5050_e4400;
        var_dfn_su_dn0 = assign5050_e4400_d_n0;
        var_dfn_su_dn2 = assign5050_e4400_d_n2;
        var_dfn_su_rv = 0.0;

        let (assign5060_e4417, assign5060_e4417_d_n0, assign5060_e4417_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5060_e4413: f64 = (var_tmf1 + var_tmf2);
        let assign5060_e4414: f64 = (0.5 * assign5060_e4413);
        let assign5060_e4415: f64 = (p.p85 - assign5060_e4414);
        (assign5060_e4415, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign5060_e4417;
        var_nja11_dn0 = assign5060_e4417_d_n0;
        var_nja11_dn2 = assign5060_e4417_d_n2;
        var_nja11_rv = 0.0;

        let (assign5070_e4432, assign5070_e4432_d_n0, assign5070_e4432_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5070_e4428: f64 = (var_nja11 - var_nfabot_i);
        let assign5070_e4430: f64 = (assign5070_e4428 - 0.01);
        (assign5070_e4430, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign5070_e4432;
        var_tmf1_dn0 = assign5070_e4432_d_n0;
        var_tmf1_dn2 = assign5070_e4432_d_n2;
        var_tmf1_rv = 0.0;

        let (assign5080_e4447, assign5080_e4447_d_n0, assign5080_e4447_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5080_e4443: f64 = (4.0 * var_nfabot_i);
        let assign5080_e4445: f64 = (assign5080_e4443 * 0.01);
        (assign5080_e4445, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5080_e4447;
        var_tmf2_dn0 = assign5080_e4447_d_n0;
        var_tmf2_dn2 = assign5080_e4447_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5090_e4464, assign5090_e4464_d_n0, assign5090_e4464_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let (assign5090_e4462, assign5090_e4462_d_n0, assign5090_e4462_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign5090_e4461: f64 = (-var_tmf2);
                (assign5090_e4461, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign5090_e4462, assign5090_e4462_d_n0, assign5090_e4462_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5090_e4464;
        var_tmf2_dn0 = assign5090_e4464_d_n0;
        var_tmf2_dn2 = assign5090_e4464_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5100_e4480, assign5100_e4480_d_n0, assign5100_e4480_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5100_e4475: f64 = (var_tmf1 * var_tmf1);
        let assign5100_e4477: f64 = (assign5100_e4475 + var_tmf2);
        let assign5100_e4478: f64 = (assign5100_e4477).sqrt();
        (assign5100_e4478, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5100_e4478)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5100_e4478)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5100_e4480;
        var_tmf2_dn0 = assign5100_e4480_d_n0;
        var_tmf2_dn2 = assign5100_e4480_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5110_e4497, assign5110_e4497_d_n0, assign5110_e4497_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5110_e4493: f64 = (var_tmf1 / var_tmf2);
        let assign5110_e4494: f64 = (1.0 + assign5110_e4493);
        let assign5110_e4495: f64 = (0.5 * assign5110_e4494);
        (assign5110_e4495, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign5110_e4497;
        var_dfn_sl_dn0 = assign5110_e4497_d_n0;
        var_dfn_sl_dn2 = assign5110_e4497_d_n2;
        var_dfn_sl_rv = 0.0;

        let (assign5120_e4514, assign5120_e4514_d_n0, assign5120_e4514_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5120_e4510: f64 = (var_tmf1 + var_tmf2);
        let assign5120_e4511: f64 = (0.5 * assign5120_e4510);
        let assign5120_e4512: f64 = (var_nfabot_i + assign5120_e4511);
        (assign5120_e4512, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign5120_e4514;
        var_nj1_dn0 = assign5120_e4514_d_n0;
        var_nj1_dn2 = assign5120_e4514_d_n2;
        var_nj1_rv = 0.0;

        let (assign5130_e4529, assign5130_e4529_d_n0, assign5130_e4529_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5130_e4525: f64 = (p.p85 - var_nj0);
        let assign5130_e4527: f64 = (assign5130_e4525 - 0.01);
        (assign5130_e4527, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign5130_e4529;
        var_tmf1_dn0 = assign5130_e4529_d_n0;
        var_tmf1_dn2 = assign5130_e4529_d_n2;
        var_tmf1_rv = 0.0;

        let (assign5140_e4544, assign5140_e4544_d_n0, assign5140_e4544_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5140_e4540: f64 = (4.0 * p.p85);
        let assign5140_e4542: f64 = (assign5140_e4540 * 0.01);
        (assign5140_e4542, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5140_e4544;
        var_tmf2_dn0 = assign5140_e4544_d_n0;
        var_tmf2_dn2 = assign5140_e4544_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5150_e4561, assign5150_e4561_d_n0, assign5150_e4561_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let (assign5150_e4559, assign5150_e4559_d_n0, assign5150_e4559_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign5150_e4558: f64 = (-var_tmf2);
                (assign5150_e4558, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign5150_e4559, assign5150_e4559_d_n0, assign5150_e4559_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5150_e4561;
        var_tmf2_dn0 = assign5150_e4561_d_n0;
        var_tmf2_dn2 = assign5150_e4561_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5160_e4577, assign5160_e4577_d_n0, assign5160_e4577_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5160_e4572: f64 = (var_tmf1 * var_tmf1);
        let assign5160_e4574: f64 = (assign5160_e4572 + var_tmf2);
        let assign5160_e4575: f64 = (assign5160_e4574).sqrt();
        (assign5160_e4575, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5160_e4575)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5160_e4575)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5160_e4577;
        var_tmf2_dn0 = assign5160_e4577_d_n0;
        var_tmf2_dn2 = assign5160_e4577_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5170_e4594, assign5170_e4594_d_n0, assign5170_e4594_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5170_e4590: f64 = (var_tmf1 + var_tmf2);
        let assign5170_e4591: f64 = (0.5 * assign5170_e4590);
        let assign5170_e4592: f64 = (p.p85 - assign5170_e4591);
        (assign5170_e4592, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign5170_e4594;
        var_nj0_dn0 = assign5170_e4594_d_n0;
        var_nj0_dn2 = assign5170_e4594_d_n2;
        var_nj0_rv = 0.0;

        let (assign5180_e4609, assign5180_e4609_d_n0, assign5180_e4609_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5180_e4605: f64 = (var_nj0 - var_nfabot_i);
        let assign5180_e4607: f64 = (assign5180_e4605 - 0.01);
        (assign5180_e4607, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign5180_e4609;
        var_tmf1_dn0 = assign5180_e4609_d_n0;
        var_tmf1_dn2 = assign5180_e4609_d_n2;
        var_tmf1_rv = 0.0;

        let (assign5190_e4624, assign5190_e4624_d_n0, assign5190_e4624_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5190_e4620: f64 = (4.0 * var_nfabot_i);
        let assign5190_e4622: f64 = (assign5190_e4620 * 0.01);
        (assign5190_e4622, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5190_e4624;
        var_tmf2_dn0 = assign5190_e4624_d_n0;
        var_tmf2_dn2 = assign5190_e4624_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5200_e4641, assign5200_e4641_d_n0, assign5200_e4641_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let (assign5200_e4639, assign5200_e4639_d_n0, assign5200_e4639_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign5200_e4638: f64 = (-var_tmf2);
                (assign5200_e4638, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign5200_e4639, assign5200_e4639_d_n0, assign5200_e4639_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5200_e4641;
        var_tmf2_dn0 = assign5200_e4641_d_n0;
        var_tmf2_dn2 = assign5200_e4641_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5210_e4657, assign5210_e4657_d_n0, assign5210_e4657_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5210_e4652: f64 = (var_tmf1 * var_tmf1);
        let assign5210_e4654: f64 = (assign5210_e4652 + var_tmf2);
        let assign5210_e4655: f64 = (assign5210_e4654).sqrt();
        (assign5210_e4655, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5210_e4655)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5210_e4655)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5210_e4657;
        var_tmf2_dn0 = assign5210_e4657_d_n0;
        var_tmf2_dn2 = assign5210_e4657_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5220_e4674, assign5220_e4674_d_n0, assign5220_e4674_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5220_e4670: f64 = (var_tmf1 + var_tmf2);
        let assign5220_e4671: f64 = (0.5 * assign5220_e4670);
        let assign5220_e4672: f64 = (var_nfabot_i + assign5220_e4671);
        (assign5220_e4672, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign5220_e4674;
        var_nj0_dn0 = assign5220_e4674_d_n0;
        var_nj0_dn2 = assign5220_e4674_d_n2;
        var_nj0_rv = 0.0;

        let (assign5230_e4689, assign5230_e4689_d_n0, assign5230_e4689_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 != 0.0)) {
        let assign5230_e4685: f64 = (p.p86 * var_dfn_su);
        let assign5230_e4687: f64 = (assign5230_e4685 * var_dfn_sl);
        (assign5230_e4687, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign5230_e4685 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign5230_e4685 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign5230_e4689;
        var_dnj1_dv_dn0 = assign5230_e4689_d_n0;
        var_dnj1_dv_dn2 = assign5230_e4689_d_n2;
        var_dnj1_dv_rv = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_guard89_slot = var_guard89;
        *var_guard89_rv_slot = var_guard89_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        var_guard31: f64,
        var_guard76: f64,
        var_guard77: f64,
        var_guard89: f64,
        var_ndisti_i: f64,
        var_nfabot_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v1: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_rv_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rv_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rv_slot: &mut f64,
        var_guard90_slot: &mut f64,
        var_guard90_rv_slot: &mut f64,
        var_guard91_slot: &mut f64,
        var_guard91_rv_slot: &mut f64,
        var_guard92_slot: &mut f64,
        var_guard92_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_rv: f64 = *var_dfn_su_rv_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_rv: f64 = *var_dvmax_over_phitd_dv_rv_slot;
        let mut var_exp_vmax_over_phitd_bot: f64 = *var_exp_vmax_over_phitd_bot_slot;
        let mut var_exp_vmax_over_phitd_bot_dn0: f64 = *var_exp_vmax_over_phitd_bot_dn0_slot;
        let mut var_exp_vmax_over_phitd_bot_dn2: f64 = *var_exp_vmax_over_phitd_bot_dn2_slot;
        let mut var_exp_vmax_over_phitd_bot_rv: f64 = *var_exp_vmax_over_phitd_bot_rv_slot;
        let mut var_guard90: f64 = *var_guard90_slot;
        let mut var_guard90_rv: f64 = *var_guard90_rv_slot;
        let mut var_guard91: f64 = *var_guard91_slot;
        let mut var_guard91_rv: f64 = *var_guard91_rv_slot;
        let mut var_guard92: f64 = *var_guard92_slot;
        let mut var_guard92_rv: f64 = *var_guard92_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign5240_e4701, assign5240_e4701_d_n0, assign5240_e4701_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign5240_e4701;
        var_nj0_dn0 = assign5240_e4701_d_n0;
        var_nj0_dn2 = assign5240_e4701_d_n2;
        var_nj0_rv = 0.0;

        let (assign5250_e4713, assign5250_e4713_d_n0, assign5250_e4713_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign5250_e4713;
        var_nj1_dn0 = assign5250_e4713_d_n0;
        var_nj1_dn2 = assign5250_e4713_d_n2;
        var_nj1_rv = 0.0;

        let (assign5260_e4725, assign5260_e4725_d_n0, assign5260_e4725_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard89 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign5260_e4725;
        var_dnj1_dv_dn0 = assign5260_e4725_d_n0;
        var_dnj1_dv_dn2 = assign5260_e4725_d_n2;
        var_dnj1_dv_rv = 0.0;

        let assign5270_e4729: f64 = (var_vmax / var_nj1);
        let assign5270_e4733: f64 = (var_nj1 - var_nj0);
        let assign5270_e4734: f64 = (var_vha1 * assign5270_e4733);
        let assign5270_e4737: f64 = (var_nj0 * p.p85);
        let assign5270_e4738: f64 = (assign5270_e4734 / assign5270_e4737);
        let assign5270_e4739: f64 = (assign5270_e4729 + assign5270_e4738);
        let assign5270_e4740: f64 = (var_phitdinv * assign5270_e4739);
        let assign5270_e4741: f64 = (assign5270_e4740).abs();
        let assign5270_e4743: f64 = if assign5270_e4741 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard90 = assign5270_e4743;
        var_guard90_rv = 0.0;

        let (assign5280_e4769, assign5280_e4769_d_n0, assign5280_e4769_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard90 != 0.0)) {
        let assign5280_e4755: f64 = (var_vmax / var_nj1);
        let assign5280_e4759: f64 = (var_nj1 - var_nj0);
        let assign5280_e4760: f64 = (var_vha1 * assign5280_e4759);
        let assign5280_e4763: f64 = (var_nj0 * p.p85);
        let assign5280_e4764: f64 = (assign5280_e4760 / assign5280_e4763);
        let assign5280_e4765: f64 = (assign5280_e4755 + assign5280_e4764);
        let assign5280_e4766: f64 = (var_phitdinv * assign5280_e4765);
        let assign5280_e4767: f64 = (assign5280_e4766).exp();
        (assign5280_e4767, (assign5280_e4767 * (var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign5280_e4763) - (assign5280_e4760 * (var_nj0_dn0 * p.p85))) / (assign5280_e4763 * assign5280_e4763))))), (assign5280_e4767 * (var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign5280_e4763) - (assign5280_e4760 * (var_nj0_dn2 * p.p85))) / (assign5280_e4763 * assign5280_e4763))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign5280_e4769;
        var_exp_vmax_over_phitd_bot_dn0 = assign5280_e4769_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign5280_e4769_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let assign5290_e4773: f64 = (var_vmax / var_nj1);
        let assign5290_e4777: f64 = (var_nj1 - var_nj0);
        let assign5290_e4778: f64 = (var_vha1 * assign5290_e4777);
        let assign5290_e4781: f64 = (var_nj0 * p.p85);
        let assign5290_e4782: f64 = (assign5290_e4778 / assign5290_e4781);
        let assign5290_e4783: f64 = (assign5290_e4773 + assign5290_e4782);
        let assign5290_e4784: f64 = (var_phitdinv * assign5290_e4783);
        let assign5290_e4786: f64 = (-230.25850929940458);
        let assign5290_e4787: f64 = if assign5290_e4784 < assign5290_e4786 { 1.0 } else { 0.0 };
        var_guard91 = assign5290_e4787;
        var_guard91_rv = 0.0;

        let (assign5300_e4868, assign5300_e4868_d_n0, assign5300_e4868_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard90 == 0.0)) && (var_guard91 != 0.0)) {
        let assign5300_e4802: f64 = (-230.25850929940458);
        let assign5300_e4806: f64 = (var_vmax / var_nj1);
        let assign5300_e4810: f64 = (var_nj1 - var_nj0);
        let assign5300_e4811: f64 = (var_vha1 * assign5300_e4810);
        let assign5300_e4814: f64 = (var_nj0 * p.p85);
        let assign5300_e4815: f64 = (assign5300_e4811 / assign5300_e4814);
        let assign5300_e4816: f64 = (assign5300_e4806 + assign5300_e4815);
        let assign5300_e4817: f64 = (var_phitdinv * assign5300_e4816);
        let assign5300_e4818: f64 = (assign5300_e4802 - assign5300_e4817);
        let assign5300_e4822: f64 = (-230.25850929940458);
        let assign5300_e4826: f64 = (var_vmax / var_nj1);
        let assign5300_e4830: f64 = (var_nj1 - var_nj0);
        let assign5300_e4831: f64 = (var_vha1 * assign5300_e4830);
        let assign5300_e4834: f64 = (var_nj0 * p.p85);
        let assign5300_e4835: f64 = (assign5300_e4831 / assign5300_e4834);
        let assign5300_e4836: f64 = (assign5300_e4826 + assign5300_e4835);
        let assign5300_e4837: f64 = (var_phitdinv * assign5300_e4836);
        let assign5300_e4838: f64 = (assign5300_e4822 - assign5300_e4837);
        let assign5300_e4841: f64 = (-230.25850929940458);
        let assign5300_e4845: f64 = (var_vmax / var_nj1);
        let assign5300_e4849: f64 = (var_nj1 - var_nj0);
        let assign5300_e4850: f64 = (var_vha1 * assign5300_e4849);
        let assign5300_e4853: f64 = (var_nj0 * p.p85);
        let assign5300_e4854: f64 = (assign5300_e4850 / assign5300_e4853);
        let assign5300_e4855: f64 = (assign5300_e4845 + assign5300_e4854);
        let assign5300_e4856: f64 = (var_phitdinv * assign5300_e4855);
        let assign5300_e4857: f64 = (assign5300_e4841 - assign5300_e4856);
        let assign5300_e4859: f64 = (assign5300_e4857 * 0.3333333333333333);
        let assign5300_e4860: f64 = (1.0 + assign5300_e4859);
        let assign5300_e4861: f64 = (assign5300_e4838 * assign5300_e4860);
        let assign5300_e4862: f64 = (0.5 * assign5300_e4861);
        let assign5300_e4863: f64 = (1.0 + assign5300_e4862);
        let assign5300_e4864: f64 = (assign5300_e4818 * assign5300_e4863);
        let assign5300_e4865: f64 = (1.0 + assign5300_e4864);
        let assign5300_e4866: f64 = (1e-100 / assign5300_e4865);
        (assign5300_e4866, (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign5300_e4814) - (assign5300_e4811 * (var_nj0_dn0 * p.p85))) / (assign5300_e4814 * assign5300_e4814))))) * assign5300_e4863) + (assign5300_e4818 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign5300_e4834) - (assign5300_e4831 * (var_nj0_dn0 * p.p85))) / (assign5300_e4834 * assign5300_e4834))))) * assign5300_e4860) + (assign5300_e4838 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign5300_e4853) - (assign5300_e4850 * (var_nj0_dn0 * p.p85))) / (assign5300_e4853 * assign5300_e4853))))) * 0.3333333333333333))))))) / (assign5300_e4865 * assign5300_e4865))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign5300_e4814) - (assign5300_e4811 * (var_nj0_dn2 * p.p85))) / (assign5300_e4814 * assign5300_e4814))))) * assign5300_e4863) + (assign5300_e4818 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign5300_e4834) - (assign5300_e4831 * (var_nj0_dn2 * p.p85))) / (assign5300_e4834 * assign5300_e4834))))) * assign5300_e4860) + (assign5300_e4838 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign5300_e4853) - (assign5300_e4850 * (var_nj0_dn2 * p.p85))) / (assign5300_e4853 * assign5300_e4853))))) * 0.3333333333333333))))))) / (assign5300_e4865 * assign5300_e4865))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign5300_e4868;
        var_exp_vmax_over_phitd_bot_dn0 = assign5300_e4868_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign5300_e4868_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let (assign5310_e4947, assign5310_e4947_d_n0, assign5310_e4947_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard90 == 0.0)) && (var_guard91 == 0.0)) {
        let assign5310_e4886: f64 = (var_vmax / var_nj1);
        let assign5310_e4890: f64 = (var_nj1 - var_nj0);
        let assign5310_e4891: f64 = (var_vha1 * assign5310_e4890);
        let assign5310_e4894: f64 = (var_nj0 * p.p85);
        let assign5310_e4895: f64 = (assign5310_e4891 / assign5310_e4894);
        let assign5310_e4896: f64 = (assign5310_e4886 + assign5310_e4895);
        let assign5310_e4897: f64 = (var_phitdinv * assign5310_e4896);
        let assign5310_e4899: f64 = (assign5310_e4897 - 230.25850929940458);
        let assign5310_e4905: f64 = (var_vmax / var_nj1);
        let assign5310_e4909: f64 = (var_nj1 - var_nj0);
        let assign5310_e4910: f64 = (var_vha1 * assign5310_e4909);
        let assign5310_e4913: f64 = (var_nj0 * p.p85);
        let assign5310_e4914: f64 = (assign5310_e4910 / assign5310_e4913);
        let assign5310_e4915: f64 = (assign5310_e4905 + assign5310_e4914);
        let assign5310_e4916: f64 = (var_phitdinv * assign5310_e4915);
        let assign5310_e4918: f64 = (assign5310_e4916 - 230.25850929940458);
        let assign5310_e4923: f64 = (var_vmax / var_nj1);
        let assign5310_e4927: f64 = (var_nj1 - var_nj0);
        let assign5310_e4928: f64 = (var_vha1 * assign5310_e4927);
        let assign5310_e4931: f64 = (var_nj0 * p.p85);
        let assign5310_e4932: f64 = (assign5310_e4928 / assign5310_e4931);
        let assign5310_e4933: f64 = (assign5310_e4923 + assign5310_e4932);
        let assign5310_e4934: f64 = (var_phitdinv * assign5310_e4933);
        let assign5310_e4936: f64 = (assign5310_e4934 - 230.25850929940458);
        let assign5310_e4938: f64 = (assign5310_e4936 * 0.3333333333333333);
        let assign5310_e4939: f64 = (1.0 + assign5310_e4938);
        let assign5310_e4940: f64 = (assign5310_e4918 * assign5310_e4939);
        let assign5310_e4941: f64 = (0.5 * assign5310_e4940);
        let assign5310_e4942: f64 = (1.0 + assign5310_e4941);
        let assign5310_e4943: f64 = (assign5310_e4899 * assign5310_e4942);
        let assign5310_e4944: f64 = (1.0 + assign5310_e4943);
        let assign5310_e4945: f64 = (1e100 * assign5310_e4944);
        (assign5310_e4945, (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign5310_e4894) - (assign5310_e4891 * (var_nj0_dn0 * p.p85))) / (assign5310_e4894 * assign5310_e4894)))) * assign5310_e4942) + (assign5310_e4899 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign5310_e4913) - (assign5310_e4910 * (var_nj0_dn0 * p.p85))) / (assign5310_e4913 * assign5310_e4913)))) * assign5310_e4939) + (assign5310_e4918 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign5310_e4931) - (assign5310_e4928 * (var_nj0_dn0 * p.p85))) / (assign5310_e4931 * assign5310_e4931)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign5310_e4894) - (assign5310_e4891 * (var_nj0_dn2 * p.p85))) / (assign5310_e4894 * assign5310_e4894)))) * assign5310_e4942) + (assign5310_e4899 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign5310_e4913) - (assign5310_e4910 * (var_nj0_dn2 * p.p85))) / (assign5310_e4913 * assign5310_e4913)))) * assign5310_e4939) + (assign5310_e4918 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign5310_e4931) - (assign5310_e4928 * (var_nj0_dn2 * p.p85))) / (assign5310_e4931 * assign5310_e4931)))) * 0.3333333333333333))))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign5310_e4947;
        var_exp_vmax_over_phitd_bot_dn0 = assign5310_e4947_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign5310_e4947_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let (assign5320_e4974, assign5320_e4974_d_n0, assign5320_e4974_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) {
        let assign5320_e4958: f64 = (var_vmax * var_dnj1_dv);
        let assign5320_e4959: f64 = (var_nj1 - assign5320_e4958);
        let assign5320_e4962: f64 = (var_nj1 * var_nj1);
        let assign5320_e4963: f64 = (assign5320_e4959 / assign5320_e4962);
        let assign5320_e4966: f64 = (var_vha1 * var_dnj1_dv);
        let assign5320_e4969: f64 = (var_nj0 * p.p85);
        let assign5320_e4970: f64 = (assign5320_e4966 / assign5320_e4969);
        let assign5320_e4971: f64 = (assign5320_e4963 + assign5320_e4970);
        let assign5320_e4972: f64 = (var_phitdinv * assign5320_e4971);
        (assign5320_e4972, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign5320_e4962) - (assign5320_e4959 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign5320_e4962 * assign5320_e4962)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign5320_e4969) - (assign5320_e4966 * (var_nj0_dn0 * p.p85))) / (assign5320_e4969 * assign5320_e4969)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign5320_e4962) - (assign5320_e4959 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign5320_e4962 * assign5320_e4962)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign5320_e4969) - (assign5320_e4966 * (var_nj0_dn2 * p.p85))) / (assign5320_e4969 * assign5320_e4969)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign5320_e4974;
        var_dvmax_over_phitd_dv_dn0 = assign5320_e4974_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign5320_e4974_d_n2;
        var_dvmax_over_phitd_dv_rv = 0.0;

        let (assign5330_e4991, assign5330_e4991_d_n0, assign5330_e4991_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) {
        let assign5330_e4984: f64 = (var_v1 - var_vmax);
        let assign5330_e4986: f64 = (assign5330_e4984 * var_dvmax_over_phitd_dv);
        let assign5330_e4987: f64 = (1.0 + assign5330_e4986);
        let assign5330_e4989: f64 = (assign5330_e4987 * var_exp_vmax_over_phitd_bot);
        (assign5330_e4989, (((assign5330_e4984 * var_dvmax_over_phitd_dv_dn0) * var_exp_vmax_over_phitd_bot) + (assign5330_e4987 * var_exp_vmax_over_phitd_bot_dn0)), (((assign5330_e4984 * var_dvmax_over_phitd_dv_dn2) * var_exp_vmax_over_phitd_bot) + (assign5330_e4987 * var_exp_vmax_over_phitd_bot_dn2)),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign5330_e4991;
        var_idmultbot_dn0 = assign5330_e4991_d_n0;
        var_idmultbot_dn2 = assign5330_e4991_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign5340_e5004,) = {
    if (((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) {
        let assign5340_e5000: f64 = (var_nin * var_nin);
        let assign5340_e5002: f64 = (assign5340_e5000 / var_ndisti_i);
        (assign5340_e5002,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign5340_e5004;
        var_pnn0_rv = 0.0;

        let (assign5350_e5020,) = {
    if (((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) {
        let assign5350_e5013: f64 = (var_nfasti_i / var_phitdinv);
        let assign5350_e5016: f64 = (var_ndisti_i / var_pnn0);
        let assign5350_e5017: f64 = (assign5350_e5016).ln();
        let assign5350_e5018: f64 = (assign5350_e5013 * assign5350_e5017);
        (assign5350_e5018,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign5350_e5020;
        var_vha1_rv = 0.0;

        let assign5360_e5023: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard92 = assign5360_e5023;
        var_guard92_rv = 0.0;

        let (assign5370_e5040, assign5370_e5040_d_n0, assign5370_e5040_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5370_e5035: f64 = (var_vmax - var_vha1);
        let assign5370_e5036: f64 = (p.p86 * assign5370_e5035);
        let assign5370_e5038: f64 = (assign5370_e5036 + var_nfasti_i);
        (assign5370_e5038, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign5370_e5040;
        var_nja10_dn0 = assign5370_e5040_d_n0;
        var_nja10_dn2 = assign5370_e5040_d_n2;
        var_nja10_rv = 0.0;

        let (assign5380_e5055, assign5380_e5055_d_n0, assign5380_e5055_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5380_e5052: f64 = (p.p86 * var_vha1);
        let assign5380_e5053: f64 = (var_nfasti_i - assign5380_e5052);
        (assign5380_e5053, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign5380_e5055;
        var_nj0_dn0 = assign5380_e5055_d_n0;
        var_nj0_dn2 = assign5380_e5055_d_n2;
        var_nj0_rv = 0.0;

        let (assign5390_e5070, assign5390_e5070_d_n0, assign5390_e5070_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5390_e5066: f64 = (p.p85 - var_nja10);
        let assign5390_e5068: f64 = (assign5390_e5066 - 0.01);
        (assign5390_e5068, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign5390_e5070;
        var_tmf1_dn0 = assign5390_e5070_d_n0;
        var_tmf1_dn2 = assign5390_e5070_d_n2;
        var_tmf1_rv = 0.0;

        let (assign5400_e5085, assign5400_e5085_d_n0, assign5400_e5085_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5400_e5081: f64 = (4.0 * p.p85);
        let assign5400_e5083: f64 = (assign5400_e5081 * 0.01);
        (assign5400_e5083, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5400_e5085;
        var_tmf2_dn0 = assign5400_e5085_d_n0;
        var_tmf2_dn2 = assign5400_e5085_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5410_e5102, assign5410_e5102_d_n0, assign5410_e5102_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let (assign5410_e5100, assign5410_e5100_d_n0, assign5410_e5100_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign5410_e5099: f64 = (-var_tmf2);
                (assign5410_e5099, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign5410_e5100, assign5410_e5100_d_n0, assign5410_e5100_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5410_e5102;
        var_tmf2_dn0 = assign5410_e5102_d_n0;
        var_tmf2_dn2 = assign5410_e5102_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5420_e5118, assign5420_e5118_d_n0, assign5420_e5118_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5420_e5113: f64 = (var_tmf1 * var_tmf1);
        let assign5420_e5115: f64 = (assign5420_e5113 + var_tmf2);
        let assign5420_e5116: f64 = (assign5420_e5115).sqrt();
        (assign5420_e5116, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5420_e5116)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5420_e5116)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5420_e5118;
        var_tmf2_dn0 = assign5420_e5118_d_n0;
        var_tmf2_dn2 = assign5420_e5118_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5430_e5135, assign5430_e5135_d_n0, assign5430_e5135_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5430_e5131: f64 = (var_tmf1 / var_tmf2);
        let assign5430_e5132: f64 = (1.0 + assign5430_e5131);
        let assign5430_e5133: f64 = (0.5 * assign5430_e5132);
        (assign5430_e5133, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign5430_e5135;
        var_dfn_su_dn0 = assign5430_e5135_d_n0;
        var_dfn_su_dn2 = assign5430_e5135_d_n2;
        var_dfn_su_rv = 0.0;

        let (assign5440_e5152, assign5440_e5152_d_n0, assign5440_e5152_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5440_e5148: f64 = (var_tmf1 + var_tmf2);
        let assign5440_e5149: f64 = (0.5 * assign5440_e5148);
        let assign5440_e5150: f64 = (p.p85 - assign5440_e5149);
        (assign5440_e5150, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign5440_e5152;
        var_nja11_dn0 = assign5440_e5152_d_n0;
        var_nja11_dn2 = assign5440_e5152_d_n2;
        var_nja11_rv = 0.0;

        let (assign5450_e5167, assign5450_e5167_d_n0, assign5450_e5167_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5450_e5163: f64 = (var_nja11 - var_nfasti_i);
        let assign5450_e5165: f64 = (assign5450_e5163 - 0.01);
        (assign5450_e5165, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign5450_e5167;
        var_tmf1_dn0 = assign5450_e5167_d_n0;
        var_tmf1_dn2 = assign5450_e5167_d_n2;
        var_tmf1_rv = 0.0;

        let (assign5460_e5182, assign5460_e5182_d_n0, assign5460_e5182_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5460_e5178: f64 = (4.0 * var_nfasti_i);
        let assign5460_e5180: f64 = (assign5460_e5178 * 0.01);
        (assign5460_e5180, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5460_e5182;
        var_tmf2_dn0 = assign5460_e5182_d_n0;
        var_tmf2_dn2 = assign5460_e5182_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5470_e5199, assign5470_e5199_d_n0, assign5470_e5199_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let (assign5470_e5197, assign5470_e5197_d_n0, assign5470_e5197_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign5470_e5196: f64 = (-var_tmf2);
                (assign5470_e5196, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign5470_e5197, assign5470_e5197_d_n0, assign5470_e5197_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5470_e5199;
        var_tmf2_dn0 = assign5470_e5199_d_n0;
        var_tmf2_dn2 = assign5470_e5199_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5480_e5215, assign5480_e5215_d_n0, assign5480_e5215_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5480_e5210: f64 = (var_tmf1 * var_tmf1);
        let assign5480_e5212: f64 = (assign5480_e5210 + var_tmf2);
        let assign5480_e5213: f64 = (assign5480_e5212).sqrt();
        (assign5480_e5213, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5480_e5213)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5480_e5213)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5480_e5215;
        var_tmf2_dn0 = assign5480_e5215_d_n0;
        var_tmf2_dn2 = assign5480_e5215_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5490_e5232, assign5490_e5232_d_n0, assign5490_e5232_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5490_e5228: f64 = (var_tmf1 / var_tmf2);
        let assign5490_e5229: f64 = (1.0 + assign5490_e5228);
        let assign5490_e5230: f64 = (0.5 * assign5490_e5229);
        (assign5490_e5230, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign5490_e5232;
        var_dfn_sl_dn0 = assign5490_e5232_d_n0;
        var_dfn_sl_dn2 = assign5490_e5232_d_n2;
        var_dfn_sl_rv = 0.0;

        let (assign5500_e5249, assign5500_e5249_d_n0, assign5500_e5249_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5500_e5245: f64 = (var_tmf1 + var_tmf2);
        let assign5500_e5246: f64 = (0.5 * assign5500_e5245);
        let assign5500_e5247: f64 = (var_nfasti_i + assign5500_e5246);
        (assign5500_e5247, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign5500_e5249;
        var_nj1_dn0 = assign5500_e5249_d_n0;
        var_nj1_dn2 = assign5500_e5249_d_n2;
        var_nj1_rv = 0.0;

        let (assign5510_e5264, assign5510_e5264_d_n0, assign5510_e5264_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5510_e5260: f64 = (p.p85 - var_nj0);
        let assign5510_e5262: f64 = (assign5510_e5260 - 0.01);
        (assign5510_e5262, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign5510_e5264;
        var_tmf1_dn0 = assign5510_e5264_d_n0;
        var_tmf1_dn2 = assign5510_e5264_d_n2;
        var_tmf1_rv = 0.0;

        let (assign5520_e5279, assign5520_e5279_d_n0, assign5520_e5279_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5520_e5275: f64 = (4.0 * p.p85);
        let assign5520_e5277: f64 = (assign5520_e5275 * 0.01);
        (assign5520_e5277, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5520_e5279;
        var_tmf2_dn0 = assign5520_e5279_d_n0;
        var_tmf2_dn2 = assign5520_e5279_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5530_e5296, assign5530_e5296_d_n0, assign5530_e5296_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let (assign5530_e5294, assign5530_e5294_d_n0, assign5530_e5294_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign5530_e5293: f64 = (-var_tmf2);
                (assign5530_e5293, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign5530_e5294, assign5530_e5294_d_n0, assign5530_e5294_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5530_e5296;
        var_tmf2_dn0 = assign5530_e5296_d_n0;
        var_tmf2_dn2 = assign5530_e5296_d_n2;
        var_tmf2_rv = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_rv_slot = var_dvmax_over_phitd_dv_rv;
        *var_exp_vmax_over_phitd_bot_slot = var_exp_vmax_over_phitd_bot;
        *var_exp_vmax_over_phitd_bot_dn0_slot = var_exp_vmax_over_phitd_bot_dn0;
        *var_exp_vmax_over_phitd_bot_dn2_slot = var_exp_vmax_over_phitd_bot_dn2;
        *var_exp_vmax_over_phitd_bot_rv_slot = var_exp_vmax_over_phitd_bot_rv;
        *var_guard90_slot = var_guard90;
        *var_guard90_rv_slot = var_guard90_rv;
        *var_guard91_slot = var_guard91;
        *var_guard91_rv_slot = var_guard91_rv;
        *var_guard92_slot = var_guard92;
        *var_guard92_rv_slot = var_guard92_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        var_guard31: f64,
        var_guard76: f64,
        var_guard77: f64,
        var_guard92: f64,
        var_ndigat_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_rv_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rv_slot: &mut f64,
        var_guard95_slot: &mut f64,
        var_guard95_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_rv: f64 = *var_dfn_su_rv_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_rv: f64 = *var_dvmax_over_phitd_dv_rv_slot;
        let mut var_guard95: f64 = *var_guard95_slot;
        let mut var_guard95_rv: f64 = *var_guard95_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign5540_e5312, assign5540_e5312_d_n0, assign5540_e5312_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5540_e5307: f64 = (var_tmf1 * var_tmf1);
        let assign5540_e5309: f64 = (assign5540_e5307 + var_tmf2);
        let assign5540_e5310: f64 = (assign5540_e5309).sqrt();
        (assign5540_e5310, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5540_e5310)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5540_e5310)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5540_e5312;
        var_tmf2_dn0 = assign5540_e5312_d_n0;
        var_tmf2_dn2 = assign5540_e5312_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5550_e5329, assign5550_e5329_d_n0, assign5550_e5329_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5550_e5325: f64 = (var_tmf1 + var_tmf2);
        let assign5550_e5326: f64 = (0.5 * assign5550_e5325);
        let assign5550_e5327: f64 = (p.p85 - assign5550_e5326);
        (assign5550_e5327, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign5550_e5329;
        var_nj0_dn0 = assign5550_e5329_d_n0;
        var_nj0_dn2 = assign5550_e5329_d_n2;
        var_nj0_rv = 0.0;

        let (assign5560_e5344, assign5560_e5344_d_n0, assign5560_e5344_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5560_e5340: f64 = (var_nj0 - var_nfasti_i);
        let assign5560_e5342: f64 = (assign5560_e5340 - 0.01);
        (assign5560_e5342, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign5560_e5344;
        var_tmf1_dn0 = assign5560_e5344_d_n0;
        var_tmf1_dn2 = assign5560_e5344_d_n2;
        var_tmf1_rv = 0.0;

        let (assign5570_e5359, assign5570_e5359_d_n0, assign5570_e5359_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5570_e5355: f64 = (4.0 * var_nfasti_i);
        let assign5570_e5357: f64 = (assign5570_e5355 * 0.01);
        (assign5570_e5357, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5570_e5359;
        var_tmf2_dn0 = assign5570_e5359_d_n0;
        var_tmf2_dn2 = assign5570_e5359_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5580_e5376, assign5580_e5376_d_n0, assign5580_e5376_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let (assign5580_e5374, assign5580_e5374_d_n0, assign5580_e5374_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign5580_e5373: f64 = (-var_tmf2);
                (assign5580_e5373, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign5580_e5374, assign5580_e5374_d_n0, assign5580_e5374_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5580_e5376;
        var_tmf2_dn0 = assign5580_e5376_d_n0;
        var_tmf2_dn2 = assign5580_e5376_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5590_e5392, assign5590_e5392_d_n0, assign5590_e5392_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5590_e5387: f64 = (var_tmf1 * var_tmf1);
        let assign5590_e5389: f64 = (assign5590_e5387 + var_tmf2);
        let assign5590_e5390: f64 = (assign5590_e5389).sqrt();
        (assign5590_e5390, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5590_e5390)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5590_e5390)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5590_e5392;
        var_tmf2_dn0 = assign5590_e5392_d_n0;
        var_tmf2_dn2 = assign5590_e5392_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5600_e5409, assign5600_e5409_d_n0, assign5600_e5409_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5600_e5405: f64 = (var_tmf1 + var_tmf2);
        let assign5600_e5406: f64 = (0.5 * assign5600_e5405);
        let assign5600_e5407: f64 = (var_nfasti_i + assign5600_e5406);
        (assign5600_e5407, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign5600_e5409;
        var_nj0_dn0 = assign5600_e5409_d_n0;
        var_nj0_dn2 = assign5600_e5409_d_n2;
        var_nj0_rv = 0.0;

        let (assign5610_e5424, assign5610_e5424_d_n0, assign5610_e5424_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 != 0.0)) {
        let assign5610_e5420: f64 = (p.p86 * var_dfn_su);
        let assign5610_e5422: f64 = (assign5610_e5420 * var_dfn_sl);
        (assign5610_e5422, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign5610_e5420 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign5610_e5420 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign5610_e5424;
        var_dnj1_dv_dn0 = assign5610_e5424_d_n0;
        var_dnj1_dv_dn2 = assign5610_e5424_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign5620_e5436, assign5620_e5436_d_n0, assign5620_e5436_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign5620_e5436;
        var_nj0_dn0 = assign5620_e5436_d_n0;
        var_nj0_dn2 = assign5620_e5436_d_n2;
        var_nj0_rv = 0.0;

        let (assign5630_e5448, assign5630_e5448_d_n0, assign5630_e5448_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign5630_e5448;
        var_nj1_dn0 = assign5630_e5448_d_n0;
        var_nj1_dn2 = assign5630_e5448_d_n2;
        var_nj1_rv = 0.0;

        let (assign5640_e5460, assign5640_e5460_d_n0, assign5640_e5460_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard92 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign5640_e5460;
        var_dnj1_dv_dn0 = assign5640_e5460_d_n0;
        var_dnj1_dv_dn2 = assign5640_e5460_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign5700_e5709, assign5700_e5709_d_n0, assign5700_e5709_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) {
        let assign5700_e5693: f64 = (var_vmax * var_dnj1_dv);
        let assign5700_e5694: f64 = (var_nj1 - assign5700_e5693);
        let assign5700_e5697: f64 = (var_nj1 * var_nj1);
        let assign5700_e5698: f64 = (assign5700_e5694 / assign5700_e5697);
        let assign5700_e5701: f64 = (var_vha1 * var_dnj1_dv);
        let assign5700_e5704: f64 = (var_nj0 * p.p85);
        let assign5700_e5705: f64 = (assign5700_e5701 / assign5700_e5704);
        let assign5700_e5706: f64 = (assign5700_e5698 + assign5700_e5705);
        let assign5700_e5707: f64 = (var_phitdinv * assign5700_e5706);
        (assign5700_e5707, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign5700_e5697) - (assign5700_e5694 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign5700_e5697 * assign5700_e5697)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign5700_e5704) - (assign5700_e5701 * (var_nj0_dn0 * p.p85))) / (assign5700_e5704 * assign5700_e5704)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign5700_e5697) - (assign5700_e5694 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign5700_e5697 * assign5700_e5697)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign5700_e5704) - (assign5700_e5701 * (var_nj0_dn2 * p.p85))) / (assign5700_e5704 * assign5700_e5704)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign5700_e5709;
        var_dvmax_over_phitd_dv_dn0 = assign5700_e5709_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign5700_e5709_d_n2;
        var_dvmax_over_phitd_dv_rv = 0.0;

        let (assign5720_e5739,) = {
    if (((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) {
        let assign5720_e5735: f64 = (var_nin * var_nin);
        let assign5720_e5737: f64 = (assign5720_e5735 / var_ndigat_i);
        (assign5720_e5737,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign5720_e5739;
        var_pnn0_rv = 0.0;

        let (assign5730_e5755,) = {
    if (((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) {
        let assign5730_e5748: f64 = (var_nfagat_i / var_phitdinv);
        let assign5730_e5751: f64 = (var_ndigat_i / var_pnn0);
        let assign5730_e5752: f64 = (assign5730_e5751).ln();
        let assign5730_e5753: f64 = (assign5730_e5748 * assign5730_e5752);
        (assign5730_e5753,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign5730_e5755;
        var_vha1_rv = 0.0;

        let assign5740_e5758: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard95 = assign5740_e5758;
        var_guard95_rv = 0.0;

        let (assign5750_e5775, assign5750_e5775_d_n0, assign5750_e5775_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5750_e5770: f64 = (var_vmax - var_vha1);
        let assign5750_e5771: f64 = (p.p86 * assign5750_e5770);
        let assign5750_e5773: f64 = (assign5750_e5771 + var_nfagat_i);
        (assign5750_e5773, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign5750_e5775;
        var_nja10_dn0 = assign5750_e5775_d_n0;
        var_nja10_dn2 = assign5750_e5775_d_n2;
        var_nja10_rv = 0.0;

        let (assign5760_e5790, assign5760_e5790_d_n0, assign5760_e5790_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5760_e5787: f64 = (p.p86 * var_vha1);
        let assign5760_e5788: f64 = (var_nfagat_i - assign5760_e5787);
        (assign5760_e5788, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign5760_e5790;
        var_nj0_dn0 = assign5760_e5790_d_n0;
        var_nj0_dn2 = assign5760_e5790_d_n2;
        var_nj0_rv = 0.0;

        let (assign5770_e5805, assign5770_e5805_d_n0, assign5770_e5805_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5770_e5801: f64 = (p.p85 - var_nja10);
        let assign5770_e5803: f64 = (assign5770_e5801 - 0.01);
        (assign5770_e5803, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign5770_e5805;
        var_tmf1_dn0 = assign5770_e5805_d_n0;
        var_tmf1_dn2 = assign5770_e5805_d_n2;
        var_tmf1_rv = 0.0;

        let (assign5780_e5820, assign5780_e5820_d_n0, assign5780_e5820_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5780_e5816: f64 = (4.0 * p.p85);
        let assign5780_e5818: f64 = (assign5780_e5816 * 0.01);
        (assign5780_e5818, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5780_e5820;
        var_tmf2_dn0 = assign5780_e5820_d_n0;
        var_tmf2_dn2 = assign5780_e5820_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5790_e5837, assign5790_e5837_d_n0, assign5790_e5837_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let (assign5790_e5835, assign5790_e5835_d_n0, assign5790_e5835_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign5790_e5834: f64 = (-var_tmf2);
                (assign5790_e5834, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign5790_e5835, assign5790_e5835_d_n0, assign5790_e5835_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5790_e5837;
        var_tmf2_dn0 = assign5790_e5837_d_n0;
        var_tmf2_dn2 = assign5790_e5837_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5800_e5853, assign5800_e5853_d_n0, assign5800_e5853_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5800_e5848: f64 = (var_tmf1 * var_tmf1);
        let assign5800_e5850: f64 = (assign5800_e5848 + var_tmf2);
        let assign5800_e5851: f64 = (assign5800_e5850).sqrt();
        (assign5800_e5851, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5800_e5851)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5800_e5851)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5800_e5853;
        var_tmf2_dn0 = assign5800_e5853_d_n0;
        var_tmf2_dn2 = assign5800_e5853_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5810_e5870, assign5810_e5870_d_n0, assign5810_e5870_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5810_e5866: f64 = (var_tmf1 / var_tmf2);
        let assign5810_e5867: f64 = (1.0 + assign5810_e5866);
        let assign5810_e5868: f64 = (0.5 * assign5810_e5867);
        (assign5810_e5868, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign5810_e5870;
        var_dfn_su_dn0 = assign5810_e5870_d_n0;
        var_dfn_su_dn2 = assign5810_e5870_d_n2;
        var_dfn_su_rv = 0.0;

        let (assign5820_e5887, assign5820_e5887_d_n0, assign5820_e5887_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5820_e5883: f64 = (var_tmf1 + var_tmf2);
        let assign5820_e5884: f64 = (0.5 * assign5820_e5883);
        let assign5820_e5885: f64 = (p.p85 - assign5820_e5884);
        (assign5820_e5885, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign5820_e5887;
        var_nja11_dn0 = assign5820_e5887_d_n0;
        var_nja11_dn2 = assign5820_e5887_d_n2;
        var_nja11_rv = 0.0;

        let (assign5830_e5902, assign5830_e5902_d_n0, assign5830_e5902_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5830_e5898: f64 = (var_nja11 - var_nfagat_i);
        let assign5830_e5900: f64 = (assign5830_e5898 - 0.01);
        (assign5830_e5900, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign5830_e5902;
        var_tmf1_dn0 = assign5830_e5902_d_n0;
        var_tmf1_dn2 = assign5830_e5902_d_n2;
        var_tmf1_rv = 0.0;

        let (assign5840_e5917, assign5840_e5917_d_n0, assign5840_e5917_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5840_e5913: f64 = (4.0 * var_nfagat_i);
        let assign5840_e5915: f64 = (assign5840_e5913 * 0.01);
        (assign5840_e5915, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5840_e5917;
        var_tmf2_dn0 = assign5840_e5917_d_n0;
        var_tmf2_dn2 = assign5840_e5917_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5850_e5934, assign5850_e5934_d_n0, assign5850_e5934_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let (assign5850_e5932, assign5850_e5932_d_n0, assign5850_e5932_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign5850_e5931: f64 = (-var_tmf2);
                (assign5850_e5931, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign5850_e5932, assign5850_e5932_d_n0, assign5850_e5932_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5850_e5934;
        var_tmf2_dn0 = assign5850_e5934_d_n0;
        var_tmf2_dn2 = assign5850_e5934_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5860_e5950, assign5860_e5950_d_n0, assign5860_e5950_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5860_e5945: f64 = (var_tmf1 * var_tmf1);
        let assign5860_e5947: f64 = (assign5860_e5945 + var_tmf2);
        let assign5860_e5948: f64 = (assign5860_e5947).sqrt();
        (assign5860_e5948, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5860_e5948)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5860_e5948)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5860_e5950;
        var_tmf2_dn0 = assign5860_e5950_d_n0;
        var_tmf2_dn2 = assign5860_e5950_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5870_e5967, assign5870_e5967_d_n0, assign5870_e5967_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5870_e5963: f64 = (var_tmf1 / var_tmf2);
        let assign5870_e5964: f64 = (1.0 + assign5870_e5963);
        let assign5870_e5965: f64 = (0.5 * assign5870_e5964);
        (assign5870_e5965, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign5870_e5967;
        var_dfn_sl_dn0 = assign5870_e5967_d_n0;
        var_dfn_sl_dn2 = assign5870_e5967_d_n2;
        var_dfn_sl_rv = 0.0;

        let (assign5880_e5984, assign5880_e5984_d_n0, assign5880_e5984_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5880_e5980: f64 = (var_tmf1 + var_tmf2);
        let assign5880_e5981: f64 = (0.5 * assign5880_e5980);
        let assign5880_e5982: f64 = (var_nfagat_i + assign5880_e5981);
        (assign5880_e5982, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign5880_e5984;
        var_nj1_dn0 = assign5880_e5984_d_n0;
        var_nj1_dn2 = assign5880_e5984_d_n2;
        var_nj1_rv = 0.0;

        let (assign5890_e5999, assign5890_e5999_d_n0, assign5890_e5999_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5890_e5995: f64 = (p.p85 - var_nj0);
        let assign5890_e5997: f64 = (assign5890_e5995 - 0.01);
        (assign5890_e5997, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign5890_e5999;
        var_tmf1_dn0 = assign5890_e5999_d_n0;
        var_tmf1_dn2 = assign5890_e5999_d_n2;
        var_tmf1_rv = 0.0;

        let (assign5900_e6014, assign5900_e6014_d_n0, assign5900_e6014_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5900_e6010: f64 = (4.0 * p.p85);
        let assign5900_e6012: f64 = (assign5900_e6010 * 0.01);
        (assign5900_e6012, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5900_e6014;
        var_tmf2_dn0 = assign5900_e6014_d_n0;
        var_tmf2_dn2 = assign5900_e6014_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5910_e6031, assign5910_e6031_d_n0, assign5910_e6031_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let (assign5910_e6029, assign5910_e6029_d_n0, assign5910_e6029_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign5910_e6028: f64 = (-var_tmf2);
                (assign5910_e6028, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign5910_e6029, assign5910_e6029_d_n0, assign5910_e6029_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5910_e6031;
        var_tmf2_dn0 = assign5910_e6031_d_n0;
        var_tmf2_dn2 = assign5910_e6031_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5920_e6047, assign5920_e6047_d_n0, assign5920_e6047_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5920_e6042: f64 = (var_tmf1 * var_tmf1);
        let assign5920_e6044: f64 = (assign5920_e6042 + var_tmf2);
        let assign5920_e6045: f64 = (assign5920_e6044).sqrt();
        (assign5920_e6045, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5920_e6045)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5920_e6045)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5920_e6047;
        var_tmf2_dn0 = assign5920_e6047_d_n0;
        var_tmf2_dn2 = assign5920_e6047_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5930_e6064, assign5930_e6064_d_n0, assign5930_e6064_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5930_e6060: f64 = (var_tmf1 + var_tmf2);
        let assign5930_e6061: f64 = (0.5 * assign5930_e6060);
        let assign5930_e6062: f64 = (p.p85 - assign5930_e6061);
        (assign5930_e6062, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign5930_e6064;
        var_nj0_dn0 = assign5930_e6064_d_n0;
        var_nj0_dn2 = assign5930_e6064_d_n2;
        var_nj0_rv = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_rv_slot = var_dvmax_over_phitd_dv_rv;
        *var_guard95_slot = var_guard95;
        *var_guard95_rv_slot = var_guard95_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        var_ab_i: f64,
        var_dfn_sl: f64,
        var_dfn_sl_dn0: f64,
        var_dfn_sl_dn2: f64,
        var_dfn_su: f64,
        var_dfn_su_dn0: f64,
        var_dfn_su_dn2: f64,
        var_guard31: f64,
        var_guard76: f64,
        var_guard77: f64,
        var_guard95: f64,
        var_lg_i: f64,
        var_ls_i: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nfagat_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v2: f64,
        var_vmax: f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rv_slot: &mut f64,
        var_guard153_slot: &mut f64,
        var_guard153_rv_slot: &mut f64,
        var_guard154_slot: &mut f64,
        var_guard154_rv_slot: &mut f64,
        var_guard157_slot: &mut f64,
        var_guard157_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_rv: f64 = *var_dvmax_over_phitd_dv_rv_slot;
        let mut var_guard153: f64 = *var_guard153_slot;
        let mut var_guard153_rv: f64 = *var_guard153_rv_slot;
        let mut var_guard154: f64 = *var_guard154_slot;
        let mut var_guard154_rv: f64 = *var_guard154_rv_slot;
        let mut var_guard157: f64 = *var_guard157_slot;
        let mut var_guard157_rv: f64 = *var_guard157_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign5940_e6079, assign5940_e6079_d_n0, assign5940_e6079_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5940_e6075: f64 = (var_nj0 - var_nfagat_i);
        let assign5940_e6077: f64 = (assign5940_e6075 - 0.01);
        (assign5940_e6077, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign5940_e6079;
        var_tmf1_dn0 = assign5940_e6079_d_n0;
        var_tmf1_dn2 = assign5940_e6079_d_n2;
        var_tmf1_rv = 0.0;

        let (assign5950_e6094, assign5950_e6094_d_n0, assign5950_e6094_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5950_e6090: f64 = (4.0 * var_nfagat_i);
        let assign5950_e6092: f64 = (assign5950_e6090 * 0.01);
        (assign5950_e6092, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5950_e6094;
        var_tmf2_dn0 = assign5950_e6094_d_n0;
        var_tmf2_dn2 = assign5950_e6094_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5960_e6111, assign5960_e6111_d_n0, assign5960_e6111_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let (assign5960_e6109, assign5960_e6109_d_n0, assign5960_e6109_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign5960_e6108: f64 = (-var_tmf2);
                (assign5960_e6108, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign5960_e6109, assign5960_e6109_d_n0, assign5960_e6109_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5960_e6111;
        var_tmf2_dn0 = assign5960_e6111_d_n0;
        var_tmf2_dn2 = assign5960_e6111_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5970_e6127, assign5970_e6127_d_n0, assign5970_e6127_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5970_e6122: f64 = (var_tmf1 * var_tmf1);
        let assign5970_e6124: f64 = (assign5970_e6122 + var_tmf2);
        let assign5970_e6125: f64 = (assign5970_e6124).sqrt();
        (assign5970_e6125, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5970_e6125)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5970_e6125)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign5970_e6127;
        var_tmf2_dn0 = assign5970_e6127_d_n0;
        var_tmf2_dn2 = assign5970_e6127_d_n2;
        var_tmf2_rv = 0.0;

        let (assign5980_e6144, assign5980_e6144_d_n0, assign5980_e6144_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5980_e6140: f64 = (var_tmf1 + var_tmf2);
        let assign5980_e6141: f64 = (0.5 * assign5980_e6140);
        let assign5980_e6142: f64 = (var_nfagat_i + assign5980_e6141);
        (assign5980_e6142, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign5980_e6144;
        var_nj0_dn0 = assign5980_e6144_d_n0;
        var_nj0_dn2 = assign5980_e6144_d_n2;
        var_nj0_rv = 0.0;

        let (assign5990_e6159, assign5990_e6159_d_n0, assign5990_e6159_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 != 0.0)) {
        let assign5990_e6155: f64 = (p.p86 * var_dfn_su);
        let assign5990_e6157: f64 = (assign5990_e6155 * var_dfn_sl);
        (assign5990_e6157, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign5990_e6155 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign5990_e6155 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign5990_e6159;
        var_dnj1_dv_dn0 = assign5990_e6159_d_n0;
        var_dnj1_dv_dn2 = assign5990_e6159_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign6000_e6171, assign6000_e6171_d_n0, assign6000_e6171_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign6000_e6171;
        var_nj0_dn0 = assign6000_e6171_d_n0;
        var_nj0_dn2 = assign6000_e6171_d_n2;
        var_nj0_rv = 0.0;

        let (assign6010_e6183, assign6010_e6183_d_n0, assign6010_e6183_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign6010_e6183;
        var_nj1_dn0 = assign6010_e6183_d_n0;
        var_nj1_dn2 = assign6010_e6183_d_n2;
        var_nj1_rv = 0.0;

        let (assign6020_e6195, assign6020_e6195_d_n0, assign6020_e6195_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) && (var_guard95 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign6020_e6195;
        var_dnj1_dv_dn0 = assign6020_e6195_d_n0;
        var_dnj1_dv_dn2 = assign6020_e6195_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign6080_e6444, assign6080_e6444_d_n0, assign6080_e6444_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard76 != 0.0)) && (var_guard77 == 0.0)) {
        let assign6080_e6428: f64 = (var_vmax * var_dnj1_dv);
        let assign6080_e6429: f64 = (var_nj1 - assign6080_e6428);
        let assign6080_e6432: f64 = (var_nj1 * var_nj1);
        let assign6080_e6433: f64 = (assign6080_e6429 / assign6080_e6432);
        let assign6080_e6436: f64 = (var_vha1 * var_dnj1_dv);
        let assign6080_e6439: f64 = (var_nj0 * p.p85);
        let assign6080_e6440: f64 = (assign6080_e6436 / assign6080_e6439);
        let assign6080_e6441: f64 = (assign6080_e6433 + assign6080_e6440);
        let assign6080_e6442: f64 = (var_phitdinv * assign6080_e6441);
        (assign6080_e6442, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign6080_e6432) - (assign6080_e6429 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign6080_e6432 * assign6080_e6432)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign6080_e6439) - (assign6080_e6436 * (var_nj0_dn0 * p.p85))) / (assign6080_e6439 * assign6080_e6439)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign6080_e6432) - (assign6080_e6429 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign6080_e6432 * assign6080_e6432)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign6080_e6439) - (assign6080_e6436 * (var_nj0_dn2 * p.p85))) / (assign6080_e6439 * assign6080_e6439)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign6080_e6444;
        var_dvmax_over_phitd_dv_dn0 = assign6080_e6444_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign6080_e6444_d_n2;
        var_dvmax_over_phitd_dv_rv = 0.0;

        let (assign6100_e6469, assign6100_e6469_d_n0, assign6100_e6469_d_n2,) = {
    if ((var_guard31 != 0.0) && (var_guard76 != 0.0)) {
        let assign6100_e6467: f64 = (var_idmultbot - 1.0);
        (assign6100_e6467, var_idmultbot_dn0, var_idmultbot_dn2,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign6100_e6469;
        var_idmultbot_dn0 = assign6100_e6469_d_n0;
        var_idmultbot_dn2 = assign6100_e6469_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign6210_e6642, assign6210_e6642_d_n0, assign6210_e6642_d_n2,) = {
    if ((var_guard31 != 0.0) && (var_guard76 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign6210_e6642;
        var_idmultbot_dn0 = assign6210_e6642_d_n0;
        var_idmultbot_dn2 = assign6210_e6642_d_n2;
        var_idmultbot_rv = 0.0;

        let assign8740_e10192: f64 = if (!(((var_ab_i == 0.0) && (var_ls_i == 0.0)) && (var_lg_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard153 = assign8740_e10192;
        var_guard153_rv = 0.0;

        let assign8820_e10264: f64 = if var_v2 < var_vmax { 1.0 } else { 0.0 };
        var_guard154 = assign8820_e10264;
        var_guard154_rv = 0.0;

        let (assign8880_e10405,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) {
        let assign8880_e10401: f64 = (var_nin * var_nin);
        let assign8880_e10403: f64 = (assign8880_e10401 / var_ndibot_i);
        (assign8880_e10403,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign8880_e10405;
        var_pnn0_rv = 0.0;

        let (assign8890_e10420,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) {
        let assign8890_e10413: f64 = (var_nfabot_i / var_phitdinv);
        let assign8890_e10416: f64 = (var_ndibot_i / var_pnn0);
        let assign8890_e10417: f64 = (assign8890_e10416).ln();
        let assign8890_e10418: f64 = (assign8890_e10413 * assign8890_e10417);
        (assign8890_e10418,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign8890_e10420;
        var_vha1_rv = 0.0;

        let assign8900_e10423: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard157 = assign8900_e10423;
        var_guard157_rv = 0.0;

        let (assign8910_e10439, assign8910_e10439_d_n0, assign8910_e10439_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign8910_e10434: f64 = (var_v2 - var_vha1);
        let assign8910_e10435: f64 = (p.p86 * assign8910_e10434);
        let assign8910_e10437: f64 = (assign8910_e10435 + var_nfabot_i);
        (assign8910_e10437, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign8910_e10439;
        var_nja10_dn0 = assign8910_e10439_d_n0;
        var_nja10_dn2 = assign8910_e10439_d_n2;
        var_nja10_rv = 0.0;

        let (assign8920_e10453, assign8920_e10453_d_n0, assign8920_e10453_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign8920_e10450: f64 = (p.p86 * var_vha1);
        let assign8920_e10451: f64 = (var_nfabot_i - assign8920_e10450);
        (assign8920_e10451, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign8920_e10453;
        var_nj0_dn0 = assign8920_e10453_d_n0;
        var_nj0_dn2 = assign8920_e10453_d_n2;
        var_nj0_rv = 0.0;

        let (assign8930_e10467, assign8930_e10467_d_n0, assign8930_e10467_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign8930_e10463: f64 = (p.p85 - var_nja10);
        let assign8930_e10465: f64 = (assign8930_e10463 - 0.01);
        (assign8930_e10465, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign8930_e10467;
        var_tmf1_dn0 = assign8930_e10467_d_n0;
        var_tmf1_dn2 = assign8930_e10467_d_n2;
        var_tmf1_rv = 0.0;

        let (assign8940_e10481, assign8940_e10481_d_n0, assign8940_e10481_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign8940_e10477: f64 = (4.0 * p.p85);
        let assign8940_e10479: f64 = (assign8940_e10477 * 0.01);
        (assign8940_e10479, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign8940_e10481;
        var_tmf2_dn0 = assign8940_e10481_d_n0;
        var_tmf2_dn2 = assign8940_e10481_d_n2;
        var_tmf2_rv = 0.0;

        let (assign8950_e10497, assign8950_e10497_d_n0, assign8950_e10497_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let (assign8950_e10495, assign8950_e10495_d_n0, assign8950_e10495_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign8950_e10494: f64 = (-var_tmf2);
                (assign8950_e10494, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign8950_e10495, assign8950_e10495_d_n0, assign8950_e10495_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign8950_e10497;
        var_tmf2_dn0 = assign8950_e10497_d_n0;
        var_tmf2_dn2 = assign8950_e10497_d_n2;
        var_tmf2_rv = 0.0;

        let (assign8960_e10512, assign8960_e10512_d_n0, assign8960_e10512_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign8960_e10507: f64 = (var_tmf1 * var_tmf1);
        let assign8960_e10509: f64 = (assign8960_e10507 + var_tmf2);
        let assign8960_e10510: f64 = (assign8960_e10509).sqrt();
        (assign8960_e10510, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign8960_e10510)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign8960_e10510)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign8960_e10512;
        var_tmf2_dn0 = assign8960_e10512_d_n0;
        var_tmf2_dn2 = assign8960_e10512_d_n2;
        var_tmf2_rv = 0.0;

        let (assign8970_e10528, assign8970_e10528_d_n0, assign8970_e10528_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign8970_e10524: f64 = (var_tmf1 + var_tmf2);
        let assign8970_e10525: f64 = (0.5 * assign8970_e10524);
        let assign8970_e10526: f64 = (p.p85 - assign8970_e10525);
        (assign8970_e10526, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign8970_e10528;
        var_nja11_dn0 = assign8970_e10528_d_n0;
        var_nja11_dn2 = assign8970_e10528_d_n2;
        var_nja11_rv = 0.0;

        let (assign8980_e10542, assign8980_e10542_d_n0, assign8980_e10542_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign8980_e10538: f64 = (var_nja11 - var_nfabot_i);
        let assign8980_e10540: f64 = (assign8980_e10538 - 0.01);
        (assign8980_e10540, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign8980_e10542;
        var_tmf1_dn0 = assign8980_e10542_d_n0;
        var_tmf1_dn2 = assign8980_e10542_d_n2;
        var_tmf1_rv = 0.0;

        let (assign8990_e10556, assign8990_e10556_d_n0, assign8990_e10556_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign8990_e10552: f64 = (4.0 * var_nfabot_i);
        let assign8990_e10554: f64 = (assign8990_e10552 * 0.01);
        (assign8990_e10554, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign8990_e10556;
        var_tmf2_dn0 = assign8990_e10556_d_n0;
        var_tmf2_dn2 = assign8990_e10556_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9000_e10572, assign9000_e10572_d_n0, assign9000_e10572_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let (assign9000_e10570, assign9000_e10570_d_n0, assign9000_e10570_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign9000_e10569: f64 = (-var_tmf2);
                (assign9000_e10569, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign9000_e10570, assign9000_e10570_d_n0, assign9000_e10570_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9000_e10572;
        var_tmf2_dn0 = assign9000_e10572_d_n0;
        var_tmf2_dn2 = assign9000_e10572_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9010_e10587, assign9010_e10587_d_n0, assign9010_e10587_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign9010_e10582: f64 = (var_tmf1 * var_tmf1);
        let assign9010_e10584: f64 = (assign9010_e10582 + var_tmf2);
        let assign9010_e10585: f64 = (assign9010_e10584).sqrt();
        (assign9010_e10585, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign9010_e10585)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign9010_e10585)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9010_e10587;
        var_tmf2_dn0 = assign9010_e10587_d_n0;
        var_tmf2_dn2 = assign9010_e10587_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9020_e10603, assign9020_e10603_d_n0, assign9020_e10603_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign9020_e10599: f64 = (var_tmf1 + var_tmf2);
        let assign9020_e10600: f64 = (0.5 * assign9020_e10599);
        let assign9020_e10601: f64 = (var_nfabot_i + assign9020_e10600);
        (assign9020_e10601, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign9020_e10603;
        var_nj1_dn0 = assign9020_e10603_d_n0;
        var_nj1_dn2 = assign9020_e10603_d_n2;
        var_nj1_rv = 0.0;

        let (assign9030_e10617, assign9030_e10617_d_n0, assign9030_e10617_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign9030_e10613: f64 = (p.p85 - var_nj0);
        let assign9030_e10615: f64 = (assign9030_e10613 - 0.01);
        (assign9030_e10615, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign9030_e10617;
        var_tmf1_dn0 = assign9030_e10617_d_n0;
        var_tmf1_dn2 = assign9030_e10617_d_n2;
        var_tmf1_rv = 0.0;

        let (assign9040_e10631, assign9040_e10631_d_n0, assign9040_e10631_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign9040_e10627: f64 = (4.0 * p.p85);
        let assign9040_e10629: f64 = (assign9040_e10627 * 0.01);
        (assign9040_e10629, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9040_e10631;
        var_tmf2_dn0 = assign9040_e10631_d_n0;
        var_tmf2_dn2 = assign9040_e10631_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9050_e10647, assign9050_e10647_d_n0, assign9050_e10647_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let (assign9050_e10645, assign9050_e10645_d_n0, assign9050_e10645_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign9050_e10644: f64 = (-var_tmf2);
                (assign9050_e10644, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign9050_e10645, assign9050_e10645_d_n0, assign9050_e10645_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9050_e10647;
        var_tmf2_dn0 = assign9050_e10647_d_n0;
        var_tmf2_dn2 = assign9050_e10647_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9060_e10662, assign9060_e10662_d_n0, assign9060_e10662_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign9060_e10657: f64 = (var_tmf1 * var_tmf1);
        let assign9060_e10659: f64 = (assign9060_e10657 + var_tmf2);
        let assign9060_e10660: f64 = (assign9060_e10659).sqrt();
        (assign9060_e10660, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign9060_e10660)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign9060_e10660)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9060_e10662;
        var_tmf2_dn0 = assign9060_e10662_d_n0;
        var_tmf2_dn2 = assign9060_e10662_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9070_e10678, assign9070_e10678_d_n0, assign9070_e10678_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign9070_e10674: f64 = (var_tmf1 + var_tmf2);
        let assign9070_e10675: f64 = (0.5 * assign9070_e10674);
        let assign9070_e10676: f64 = (p.p85 - assign9070_e10675);
        (assign9070_e10676, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign9070_e10678;
        var_nj0_dn0 = assign9070_e10678_d_n0;
        var_nj0_dn2 = assign9070_e10678_d_n2;
        var_nj0_rv = 0.0;

        let (assign9080_e10692, assign9080_e10692_d_n0, assign9080_e10692_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign9080_e10688: f64 = (var_nj0 - var_nfabot_i);
        let assign9080_e10690: f64 = (assign9080_e10688 - 0.01);
        (assign9080_e10690, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign9080_e10692;
        var_tmf1_dn0 = assign9080_e10692_d_n0;
        var_tmf1_dn2 = assign9080_e10692_d_n2;
        var_tmf1_rv = 0.0;

        let (assign9090_e10706, assign9090_e10706_d_n0, assign9090_e10706_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign9090_e10702: f64 = (4.0 * var_nfabot_i);
        let assign9090_e10704: f64 = (assign9090_e10702 * 0.01);
        (assign9090_e10704, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9090_e10706;
        var_tmf2_dn0 = assign9090_e10706_d_n0;
        var_tmf2_dn2 = assign9090_e10706_d_n2;
        var_tmf2_rv = 0.0;

        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_rv_slot = var_dvmax_over_phitd_dv_rv;
        *var_guard153_slot = var_guard153;
        *var_guard153_rv_slot = var_guard153_rv;
        *var_guard154_slot = var_guard154;
        *var_guard154_rv_slot = var_guard154_rv;
        *var_guard157_slot = var_guard157;
        *var_guard157_rv_slot = var_guard157_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        var_guard153: f64,
        var_guard154: f64,
        var_guard157: f64,
        var_guard31: f64,
        var_ndisti_i: f64,
        var_nfabot_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v2: f64,
        var_guard158_slot: &mut f64,
        var_guard158_rv_slot: &mut f64,
        var_guard159_slot: &mut f64,
        var_guard159_rv_slot: &mut f64,
        var_guard160_slot: &mut f64,
        var_guard160_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_guard158: f64 = *var_guard158_slot;
        let mut var_guard158_rv: f64 = *var_guard158_rv_slot;
        let mut var_guard159: f64 = *var_guard159_slot;
        let mut var_guard159_rv: f64 = *var_guard159_rv_slot;
        let mut var_guard160: f64 = *var_guard160_slot;
        let mut var_guard160_rv: f64 = *var_guard160_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign9100_e10722, assign9100_e10722_d_n0, assign9100_e10722_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let (assign9100_e10720, assign9100_e10720_d_n0, assign9100_e10720_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign9100_e10719: f64 = (-var_tmf2);
                (assign9100_e10719, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign9100_e10720, assign9100_e10720_d_n0, assign9100_e10720_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9100_e10722;
        var_tmf2_dn0 = assign9100_e10722_d_n0;
        var_tmf2_dn2 = assign9100_e10722_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9110_e10737, assign9110_e10737_d_n0, assign9110_e10737_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign9110_e10732: f64 = (var_tmf1 * var_tmf1);
        let assign9110_e10734: f64 = (assign9110_e10732 + var_tmf2);
        let assign9110_e10735: f64 = (assign9110_e10734).sqrt();
        (assign9110_e10735, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign9110_e10735)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign9110_e10735)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9110_e10737;
        var_tmf2_dn0 = assign9110_e10737_d_n0;
        var_tmf2_dn2 = assign9110_e10737_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9120_e10753, assign9120_e10753_d_n0, assign9120_e10753_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 != 0.0)) {
        let assign9120_e10749: f64 = (var_tmf1 + var_tmf2);
        let assign9120_e10750: f64 = (0.5 * assign9120_e10749);
        let assign9120_e10751: f64 = (var_nfabot_i + assign9120_e10750);
        (assign9120_e10751, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign9120_e10753;
        var_nj0_dn0 = assign9120_e10753_d_n0;
        var_nj0_dn2 = assign9120_e10753_d_n2;
        var_nj0_rv = 0.0;

        let (assign9130_e10764, assign9130_e10764_d_n0, assign9130_e10764_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign9130_e10764;
        var_nj0_dn0 = assign9130_e10764_d_n0;
        var_nj0_dn2 = assign9130_e10764_d_n2;
        var_nj0_rv = 0.0;

        let (assign9140_e10775, assign9140_e10775_d_n0, assign9140_e10775_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard157 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign9140_e10775;
        var_nj1_dn0 = assign9140_e10775_d_n0;
        var_nj1_dn2 = assign9140_e10775_d_n2;
        var_nj1_rv = 0.0;

        let assign9150_e10779: f64 = (var_v2 / var_nj1);
        let assign9150_e10783: f64 = (var_nj1 - var_nj0);
        let assign9150_e10784: f64 = (var_vha1 * assign9150_e10783);
        let assign9150_e10787: f64 = (var_nj0 * p.p85);
        let assign9150_e10788: f64 = (assign9150_e10784 / assign9150_e10787);
        let assign9150_e10789: f64 = (assign9150_e10779 + assign9150_e10788);
        let assign9150_e10790: f64 = (var_phitdinv * assign9150_e10789);
        let assign9150_e10791: f64 = (assign9150_e10790).abs();
        let assign9150_e10793: f64 = if assign9150_e10791 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard158 = assign9150_e10793;
        var_guard158_rv = 0.0;

        let (assign9160_e10818, assign9160_e10818_d_n0, assign9160_e10818_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard158 != 0.0)) {
        let assign9160_e10804: f64 = (var_v2 / var_nj1);
        let assign9160_e10808: f64 = (var_nj1 - var_nj0);
        let assign9160_e10809: f64 = (var_vha1 * assign9160_e10808);
        let assign9160_e10812: f64 = (var_nj0 * p.p85);
        let assign9160_e10813: f64 = (assign9160_e10809 / assign9160_e10812);
        let assign9160_e10814: f64 = (assign9160_e10804 + assign9160_e10813);
        let assign9160_e10815: f64 = (var_phitdinv * assign9160_e10814);
        let assign9160_e10816: f64 = (assign9160_e10815).exp();
        (assign9160_e10816, (assign9160_e10816 * (var_phitdinv * ((-((var_v2 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign9160_e10812) - (assign9160_e10809 * (var_nj0_dn0 * p.p85))) / (assign9160_e10812 * assign9160_e10812))))), (assign9160_e10816 * (var_phitdinv * ((-((var_v2 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign9160_e10812) - (assign9160_e10809 * (var_nj0_dn2 * p.p85))) / (assign9160_e10812 * assign9160_e10812))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign9160_e10818;
        var_idmultbot_dn0 = assign9160_e10818_d_n0;
        var_idmultbot_dn2 = assign9160_e10818_d_n2;
        var_idmultbot_rv = 0.0;

        let assign9170_e10822: f64 = (var_v2 / var_nj1);
        let assign9170_e10826: f64 = (var_nj1 - var_nj0);
        let assign9170_e10827: f64 = (var_vha1 * assign9170_e10826);
        let assign9170_e10830: f64 = (var_nj0 * p.p85);
        let assign9170_e10831: f64 = (assign9170_e10827 / assign9170_e10830);
        let assign9170_e10832: f64 = (assign9170_e10822 + assign9170_e10831);
        let assign9170_e10833: f64 = (var_phitdinv * assign9170_e10832);
        let assign9170_e10835: f64 = (-230.25850929940458);
        let assign9170_e10836: f64 = if assign9170_e10833 < assign9170_e10835 { 1.0 } else { 0.0 };
        var_guard159 = assign9170_e10836;
        var_guard159_rv = 0.0;

        let (assign9180_e10916, assign9180_e10916_d_n0, assign9180_e10916_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard158 == 0.0)) && (var_guard159 != 0.0)) {
        let assign9180_e10850: f64 = (-230.25850929940458);
        let assign9180_e10854: f64 = (var_v2 / var_nj1);
        let assign9180_e10858: f64 = (var_nj1 - var_nj0);
        let assign9180_e10859: f64 = (var_vha1 * assign9180_e10858);
        let assign9180_e10862: f64 = (var_nj0 * p.p85);
        let assign9180_e10863: f64 = (assign9180_e10859 / assign9180_e10862);
        let assign9180_e10864: f64 = (assign9180_e10854 + assign9180_e10863);
        let assign9180_e10865: f64 = (var_phitdinv * assign9180_e10864);
        let assign9180_e10866: f64 = (assign9180_e10850 - assign9180_e10865);
        let assign9180_e10870: f64 = (-230.25850929940458);
        let assign9180_e10874: f64 = (var_v2 / var_nj1);
        let assign9180_e10878: f64 = (var_nj1 - var_nj0);
        let assign9180_e10879: f64 = (var_vha1 * assign9180_e10878);
        let assign9180_e10882: f64 = (var_nj0 * p.p85);
        let assign9180_e10883: f64 = (assign9180_e10879 / assign9180_e10882);
        let assign9180_e10884: f64 = (assign9180_e10874 + assign9180_e10883);
        let assign9180_e10885: f64 = (var_phitdinv * assign9180_e10884);
        let assign9180_e10886: f64 = (assign9180_e10870 - assign9180_e10885);
        let assign9180_e10889: f64 = (-230.25850929940458);
        let assign9180_e10893: f64 = (var_v2 / var_nj1);
        let assign9180_e10897: f64 = (var_nj1 - var_nj0);
        let assign9180_e10898: f64 = (var_vha1 * assign9180_e10897);
        let assign9180_e10901: f64 = (var_nj0 * p.p85);
        let assign9180_e10902: f64 = (assign9180_e10898 / assign9180_e10901);
        let assign9180_e10903: f64 = (assign9180_e10893 + assign9180_e10902);
        let assign9180_e10904: f64 = (var_phitdinv * assign9180_e10903);
        let assign9180_e10905: f64 = (assign9180_e10889 - assign9180_e10904);
        let assign9180_e10907: f64 = (assign9180_e10905 * 0.3333333333333333);
        let assign9180_e10908: f64 = (1.0 + assign9180_e10907);
        let assign9180_e10909: f64 = (assign9180_e10886 * assign9180_e10908);
        let assign9180_e10910: f64 = (0.5 * assign9180_e10909);
        let assign9180_e10911: f64 = (1.0 + assign9180_e10910);
        let assign9180_e10912: f64 = (assign9180_e10866 * assign9180_e10911);
        let assign9180_e10913: f64 = (1.0 + assign9180_e10912);
        let assign9180_e10914: f64 = (1e-100 / assign9180_e10913);
        (assign9180_e10914, (-((1e-100 * (((-(var_phitdinv * ((-((var_v2 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign9180_e10862) - (assign9180_e10859 * (var_nj0_dn0 * p.p85))) / (assign9180_e10862 * assign9180_e10862))))) * assign9180_e10911) + (assign9180_e10866 * (0.5 * (((-(var_phitdinv * ((-((var_v2 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign9180_e10882) - (assign9180_e10879 * (var_nj0_dn0 * p.p85))) / (assign9180_e10882 * assign9180_e10882))))) * assign9180_e10908) + (assign9180_e10886 * ((-(var_phitdinv * ((-((var_v2 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign9180_e10901) - (assign9180_e10898 * (var_nj0_dn0 * p.p85))) / (assign9180_e10901 * assign9180_e10901))))) * 0.3333333333333333))))))) / (assign9180_e10913 * assign9180_e10913))), (-((1e-100 * (((-(var_phitdinv * ((-((var_v2 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign9180_e10862) - (assign9180_e10859 * (var_nj0_dn2 * p.p85))) / (assign9180_e10862 * assign9180_e10862))))) * assign9180_e10911) + (assign9180_e10866 * (0.5 * (((-(var_phitdinv * ((-((var_v2 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign9180_e10882) - (assign9180_e10879 * (var_nj0_dn2 * p.p85))) / (assign9180_e10882 * assign9180_e10882))))) * assign9180_e10908) + (assign9180_e10886 * ((-(var_phitdinv * ((-((var_v2 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign9180_e10901) - (assign9180_e10898 * (var_nj0_dn2 * p.p85))) / (assign9180_e10901 * assign9180_e10901))))) * 0.3333333333333333))))))) / (assign9180_e10913 * assign9180_e10913))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign9180_e10916;
        var_idmultbot_dn0 = assign9180_e10916_d_n0;
        var_idmultbot_dn2 = assign9180_e10916_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign9190_e10994, assign9190_e10994_d_n0, assign9190_e10994_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard158 == 0.0)) && (var_guard159 == 0.0)) {
        let assign9190_e10933: f64 = (var_v2 / var_nj1);
        let assign9190_e10937: f64 = (var_nj1 - var_nj0);
        let assign9190_e10938: f64 = (var_vha1 * assign9190_e10937);
        let assign9190_e10941: f64 = (var_nj0 * p.p85);
        let assign9190_e10942: f64 = (assign9190_e10938 / assign9190_e10941);
        let assign9190_e10943: f64 = (assign9190_e10933 + assign9190_e10942);
        let assign9190_e10944: f64 = (var_phitdinv * assign9190_e10943);
        let assign9190_e10946: f64 = (assign9190_e10944 - 230.25850929940458);
        let assign9190_e10952: f64 = (var_v2 / var_nj1);
        let assign9190_e10956: f64 = (var_nj1 - var_nj0);
        let assign9190_e10957: f64 = (var_vha1 * assign9190_e10956);
        let assign9190_e10960: f64 = (var_nj0 * p.p85);
        let assign9190_e10961: f64 = (assign9190_e10957 / assign9190_e10960);
        let assign9190_e10962: f64 = (assign9190_e10952 + assign9190_e10961);
        let assign9190_e10963: f64 = (var_phitdinv * assign9190_e10962);
        let assign9190_e10965: f64 = (assign9190_e10963 - 230.25850929940458);
        let assign9190_e10970: f64 = (var_v2 / var_nj1);
        let assign9190_e10974: f64 = (var_nj1 - var_nj0);
        let assign9190_e10975: f64 = (var_vha1 * assign9190_e10974);
        let assign9190_e10978: f64 = (var_nj0 * p.p85);
        let assign9190_e10979: f64 = (assign9190_e10975 / assign9190_e10978);
        let assign9190_e10980: f64 = (assign9190_e10970 + assign9190_e10979);
        let assign9190_e10981: f64 = (var_phitdinv * assign9190_e10980);
        let assign9190_e10983: f64 = (assign9190_e10981 - 230.25850929940458);
        let assign9190_e10985: f64 = (assign9190_e10983 * 0.3333333333333333);
        let assign9190_e10986: f64 = (1.0 + assign9190_e10985);
        let assign9190_e10987: f64 = (assign9190_e10965 * assign9190_e10986);
        let assign9190_e10988: f64 = (0.5 * assign9190_e10987);
        let assign9190_e10989: f64 = (1.0 + assign9190_e10988);
        let assign9190_e10990: f64 = (assign9190_e10946 * assign9190_e10989);
        let assign9190_e10991: f64 = (1.0 + assign9190_e10990);
        let assign9190_e10992: f64 = (1e100 * assign9190_e10991);
        (assign9190_e10992, (1e100 * (((var_phitdinv * ((-((var_v2 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign9190_e10941) - (assign9190_e10938 * (var_nj0_dn0 * p.p85))) / (assign9190_e10941 * assign9190_e10941)))) * assign9190_e10989) + (assign9190_e10946 * (0.5 * (((var_phitdinv * ((-((var_v2 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign9190_e10960) - (assign9190_e10957 * (var_nj0_dn0 * p.p85))) / (assign9190_e10960 * assign9190_e10960)))) * assign9190_e10986) + (assign9190_e10965 * ((var_phitdinv * ((-((var_v2 * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign9190_e10978) - (assign9190_e10975 * (var_nj0_dn0 * p.p85))) / (assign9190_e10978 * assign9190_e10978)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_v2 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign9190_e10941) - (assign9190_e10938 * (var_nj0_dn2 * p.p85))) / (assign9190_e10941 * assign9190_e10941)))) * assign9190_e10989) + (assign9190_e10946 * (0.5 * (((var_phitdinv * ((-((var_v2 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign9190_e10960) - (assign9190_e10957 * (var_nj0_dn2 * p.p85))) / (assign9190_e10960 * assign9190_e10960)))) * assign9190_e10986) + (assign9190_e10965 * ((var_phitdinv * ((-((var_v2 * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign9190_e10978) - (assign9190_e10975 * (var_nj0_dn2 * p.p85))) / (assign9190_e10978 * assign9190_e10978)))) * 0.3333333333333333))))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign9190_e10994;
        var_idmultbot_dn0 = assign9190_e10994_d_n0;
        var_idmultbot_dn2 = assign9190_e10994_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign9200_e11006,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) {
        let assign9200_e11002: f64 = (var_nin * var_nin);
        let assign9200_e11004: f64 = (assign9200_e11002 / var_ndisti_i);
        (assign9200_e11004,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign9200_e11006;
        var_pnn0_rv = 0.0;

        let (assign9210_e11021,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) {
        let assign9210_e11014: f64 = (var_nfasti_i / var_phitdinv);
        let assign9210_e11017: f64 = (var_ndisti_i / var_pnn0);
        let assign9210_e11018: f64 = (assign9210_e11017).ln();
        let assign9210_e11019: f64 = (assign9210_e11014 * assign9210_e11018);
        (assign9210_e11019,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign9210_e11021;
        var_vha1_rv = 0.0;

        let assign9220_e11024: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard160 = assign9220_e11024;
        var_guard160_rv = 0.0;

        let (assign9230_e11040, assign9230_e11040_d_n0, assign9230_e11040_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9230_e11035: f64 = (var_v2 - var_vha1);
        let assign9230_e11036: f64 = (p.p86 * assign9230_e11035);
        let assign9230_e11038: f64 = (assign9230_e11036 + var_nfasti_i);
        (assign9230_e11038, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign9230_e11040;
        var_nja10_dn0 = assign9230_e11040_d_n0;
        var_nja10_dn2 = assign9230_e11040_d_n2;
        var_nja10_rv = 0.0;

        let (assign9240_e11054, assign9240_e11054_d_n0, assign9240_e11054_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9240_e11051: f64 = (p.p86 * var_vha1);
        let assign9240_e11052: f64 = (var_nfasti_i - assign9240_e11051);
        (assign9240_e11052, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign9240_e11054;
        var_nj0_dn0 = assign9240_e11054_d_n0;
        var_nj0_dn2 = assign9240_e11054_d_n2;
        var_nj0_rv = 0.0;

        let (assign9250_e11068, assign9250_e11068_d_n0, assign9250_e11068_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9250_e11064: f64 = (p.p85 - var_nja10);
        let assign9250_e11066: f64 = (assign9250_e11064 - 0.01);
        (assign9250_e11066, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign9250_e11068;
        var_tmf1_dn0 = assign9250_e11068_d_n0;
        var_tmf1_dn2 = assign9250_e11068_d_n2;
        var_tmf1_rv = 0.0;

        let (assign9260_e11082, assign9260_e11082_d_n0, assign9260_e11082_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9260_e11078: f64 = (4.0 * p.p85);
        let assign9260_e11080: f64 = (assign9260_e11078 * 0.01);
        (assign9260_e11080, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9260_e11082;
        var_tmf2_dn0 = assign9260_e11082_d_n0;
        var_tmf2_dn2 = assign9260_e11082_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9270_e11098, assign9270_e11098_d_n0, assign9270_e11098_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let (assign9270_e11096, assign9270_e11096_d_n0, assign9270_e11096_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign9270_e11095: f64 = (-var_tmf2);
                (assign9270_e11095, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign9270_e11096, assign9270_e11096_d_n0, assign9270_e11096_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9270_e11098;
        var_tmf2_dn0 = assign9270_e11098_d_n0;
        var_tmf2_dn2 = assign9270_e11098_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9280_e11113, assign9280_e11113_d_n0, assign9280_e11113_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9280_e11108: f64 = (var_tmf1 * var_tmf1);
        let assign9280_e11110: f64 = (assign9280_e11108 + var_tmf2);
        let assign9280_e11111: f64 = (assign9280_e11110).sqrt();
        (assign9280_e11111, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign9280_e11111)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign9280_e11111)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9280_e11113;
        var_tmf2_dn0 = assign9280_e11113_d_n0;
        var_tmf2_dn2 = assign9280_e11113_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9290_e11129, assign9290_e11129_d_n0, assign9290_e11129_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9290_e11125: f64 = (var_tmf1 + var_tmf2);
        let assign9290_e11126: f64 = (0.5 * assign9290_e11125);
        let assign9290_e11127: f64 = (p.p85 - assign9290_e11126);
        (assign9290_e11127, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign9290_e11129;
        var_nja11_dn0 = assign9290_e11129_d_n0;
        var_nja11_dn2 = assign9290_e11129_d_n2;
        var_nja11_rv = 0.0;

        let (assign9300_e11143, assign9300_e11143_d_n0, assign9300_e11143_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9300_e11139: f64 = (var_nja11 - var_nfasti_i);
        let assign9300_e11141: f64 = (assign9300_e11139 - 0.01);
        (assign9300_e11141, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign9300_e11143;
        var_tmf1_dn0 = assign9300_e11143_d_n0;
        var_tmf1_dn2 = assign9300_e11143_d_n2;
        var_tmf1_rv = 0.0;

        let (assign9310_e11157, assign9310_e11157_d_n0, assign9310_e11157_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9310_e11153: f64 = (4.0 * var_nfasti_i);
        let assign9310_e11155: f64 = (assign9310_e11153 * 0.01);
        (assign9310_e11155, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9310_e11157;
        var_tmf2_dn0 = assign9310_e11157_d_n0;
        var_tmf2_dn2 = assign9310_e11157_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9320_e11173, assign9320_e11173_d_n0, assign9320_e11173_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let (assign9320_e11171, assign9320_e11171_d_n0, assign9320_e11171_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign9320_e11170: f64 = (-var_tmf2);
                (assign9320_e11170, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign9320_e11171, assign9320_e11171_d_n0, assign9320_e11171_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9320_e11173;
        var_tmf2_dn0 = assign9320_e11173_d_n0;
        var_tmf2_dn2 = assign9320_e11173_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9330_e11188, assign9330_e11188_d_n0, assign9330_e11188_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9330_e11183: f64 = (var_tmf1 * var_tmf1);
        let assign9330_e11185: f64 = (assign9330_e11183 + var_tmf2);
        let assign9330_e11186: f64 = (assign9330_e11185).sqrt();
        (assign9330_e11186, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign9330_e11186)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign9330_e11186)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9330_e11188;
        var_tmf2_dn0 = assign9330_e11188_d_n0;
        var_tmf2_dn2 = assign9330_e11188_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9340_e11204, assign9340_e11204_d_n0, assign9340_e11204_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9340_e11200: f64 = (var_tmf1 + var_tmf2);
        let assign9340_e11201: f64 = (0.5 * assign9340_e11200);
        let assign9340_e11202: f64 = (var_nfasti_i + assign9340_e11201);
        (assign9340_e11202, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign9340_e11204;
        var_nj1_dn0 = assign9340_e11204_d_n0;
        var_nj1_dn2 = assign9340_e11204_d_n2;
        var_nj1_rv = 0.0;

        let (assign9350_e11218, assign9350_e11218_d_n0, assign9350_e11218_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9350_e11214: f64 = (p.p85 - var_nj0);
        let assign9350_e11216: f64 = (assign9350_e11214 - 0.01);
        (assign9350_e11216, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign9350_e11218;
        var_tmf1_dn0 = assign9350_e11218_d_n0;
        var_tmf1_dn2 = assign9350_e11218_d_n2;
        var_tmf1_rv = 0.0;

        let (assign9360_e11232, assign9360_e11232_d_n0, assign9360_e11232_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9360_e11228: f64 = (4.0 * p.p85);
        let assign9360_e11230: f64 = (assign9360_e11228 * 0.01);
        (assign9360_e11230, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9360_e11232;
        var_tmf2_dn0 = assign9360_e11232_d_n0;
        var_tmf2_dn2 = assign9360_e11232_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9370_e11248, assign9370_e11248_d_n0, assign9370_e11248_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let (assign9370_e11246, assign9370_e11246_d_n0, assign9370_e11246_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign9370_e11245: f64 = (-var_tmf2);
                (assign9370_e11245, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign9370_e11246, assign9370_e11246_d_n0, assign9370_e11246_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9370_e11248;
        var_tmf2_dn0 = assign9370_e11248_d_n0;
        var_tmf2_dn2 = assign9370_e11248_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9380_e11263, assign9380_e11263_d_n0, assign9380_e11263_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9380_e11258: f64 = (var_tmf1 * var_tmf1);
        let assign9380_e11260: f64 = (assign9380_e11258 + var_tmf2);
        let assign9380_e11261: f64 = (assign9380_e11260).sqrt();
        (assign9380_e11261, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign9380_e11261)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign9380_e11261)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9380_e11263;
        var_tmf2_dn0 = assign9380_e11263_d_n0;
        var_tmf2_dn2 = assign9380_e11263_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9390_e11279, assign9390_e11279_d_n0, assign9390_e11279_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9390_e11275: f64 = (var_tmf1 + var_tmf2);
        let assign9390_e11276: f64 = (0.5 * assign9390_e11275);
        let assign9390_e11277: f64 = (p.p85 - assign9390_e11276);
        (assign9390_e11277, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign9390_e11279;
        var_nj0_dn0 = assign9390_e11279_d_n0;
        var_nj0_dn2 = assign9390_e11279_d_n2;
        var_nj0_rv = 0.0;

        *var_guard158_slot = var_guard158;
        *var_guard158_rv_slot = var_guard158_rv;
        *var_guard159_slot = var_guard159;
        *var_guard159_rv_slot = var_guard159_rv;
        *var_guard160_slot = var_guard160;
        *var_guard160_rv_slot = var_guard160_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        var_guard153: f64,
        var_guard154: f64,
        var_guard160: f64,
        var_guard31: f64,
        var_ndibot_i: f64,
        var_ndigat_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v2: f64,
        var_guard163_slot: &mut f64,
        var_guard163_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_guard163: f64 = *var_guard163_slot;
        let mut var_guard163_rv: f64 = *var_guard163_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign9400_e11293, assign9400_e11293_d_n0, assign9400_e11293_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9400_e11289: f64 = (var_nj0 - var_nfasti_i);
        let assign9400_e11291: f64 = (assign9400_e11289 - 0.01);
        (assign9400_e11291, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign9400_e11293;
        var_tmf1_dn0 = assign9400_e11293_d_n0;
        var_tmf1_dn2 = assign9400_e11293_d_n2;
        var_tmf1_rv = 0.0;

        let (assign9410_e11307, assign9410_e11307_d_n0, assign9410_e11307_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9410_e11303: f64 = (4.0 * var_nfasti_i);
        let assign9410_e11305: f64 = (assign9410_e11303 * 0.01);
        (assign9410_e11305, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9410_e11307;
        var_tmf2_dn0 = assign9410_e11307_d_n0;
        var_tmf2_dn2 = assign9410_e11307_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9420_e11323, assign9420_e11323_d_n0, assign9420_e11323_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let (assign9420_e11321, assign9420_e11321_d_n0, assign9420_e11321_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign9420_e11320: f64 = (-var_tmf2);
                (assign9420_e11320, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign9420_e11321, assign9420_e11321_d_n0, assign9420_e11321_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9420_e11323;
        var_tmf2_dn0 = assign9420_e11323_d_n0;
        var_tmf2_dn2 = assign9420_e11323_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9430_e11338, assign9430_e11338_d_n0, assign9430_e11338_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9430_e11333: f64 = (var_tmf1 * var_tmf1);
        let assign9430_e11335: f64 = (assign9430_e11333 + var_tmf2);
        let assign9430_e11336: f64 = (assign9430_e11335).sqrt();
        (assign9430_e11336, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign9430_e11336)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign9430_e11336)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9430_e11338;
        var_tmf2_dn0 = assign9430_e11338_d_n0;
        var_tmf2_dn2 = assign9430_e11338_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9440_e11354, assign9440_e11354_d_n0, assign9440_e11354_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 != 0.0)) {
        let assign9440_e11350: f64 = (var_tmf1 + var_tmf2);
        let assign9440_e11351: f64 = (0.5 * assign9440_e11350);
        let assign9440_e11352: f64 = (var_nfasti_i + assign9440_e11351);
        (assign9440_e11352, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign9440_e11354;
        var_nj0_dn0 = assign9440_e11354_d_n0;
        var_nj0_dn2 = assign9440_e11354_d_n2;
        var_nj0_rv = 0.0;

        let (assign9450_e11365, assign9450_e11365_d_n0, assign9450_e11365_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign9450_e11365;
        var_nj0_dn0 = assign9450_e11365_d_n0;
        var_nj0_dn2 = assign9450_e11365_d_n2;
        var_nj0_rv = 0.0;

        let (assign9460_e11376, assign9460_e11376_d_n0, assign9460_e11376_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard160 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign9460_e11376;
        var_nj1_dn0 = assign9460_e11376_d_n0;
        var_nj1_dn2 = assign9460_e11376_d_n2;
        var_nj1_rv = 0.0;

        let (assign9520_e11607,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) {
        let assign9520_e11603: f64 = (var_nin * var_nin);
        let assign9520_e11605: f64 = (assign9520_e11603 / var_ndigat_i);
        (assign9520_e11605,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign9520_e11607;
        var_pnn0_rv = 0.0;

        let (assign9530_e11622,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) {
        let assign9530_e11615: f64 = (var_nfagat_i / var_phitdinv);
        let assign9530_e11618: f64 = (var_ndigat_i / var_pnn0);
        let assign9530_e11619: f64 = (assign9530_e11618).ln();
        let assign9530_e11620: f64 = (assign9530_e11615 * assign9530_e11619);
        (assign9530_e11620,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign9530_e11622;
        var_vha1_rv = 0.0;

        let assign9540_e11625: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard163 = assign9540_e11625;
        var_guard163_rv = 0.0;

        let (assign9550_e11641, assign9550_e11641_d_n0, assign9550_e11641_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9550_e11636: f64 = (var_v2 - var_vha1);
        let assign9550_e11637: f64 = (p.p86 * assign9550_e11636);
        let assign9550_e11639: f64 = (assign9550_e11637 + var_nfagat_i);
        (assign9550_e11639, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign9550_e11641;
        var_nja10_dn0 = assign9550_e11641_d_n0;
        var_nja10_dn2 = assign9550_e11641_d_n2;
        var_nja10_rv = 0.0;

        let (assign9560_e11655, assign9560_e11655_d_n0, assign9560_e11655_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9560_e11652: f64 = (p.p86 * var_vha1);
        let assign9560_e11653: f64 = (var_nfagat_i - assign9560_e11652);
        (assign9560_e11653, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign9560_e11655;
        var_nj0_dn0 = assign9560_e11655_d_n0;
        var_nj0_dn2 = assign9560_e11655_d_n2;
        var_nj0_rv = 0.0;

        let (assign9570_e11669, assign9570_e11669_d_n0, assign9570_e11669_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9570_e11665: f64 = (p.p85 - var_nja10);
        let assign9570_e11667: f64 = (assign9570_e11665 - 0.01);
        (assign9570_e11667, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign9570_e11669;
        var_tmf1_dn0 = assign9570_e11669_d_n0;
        var_tmf1_dn2 = assign9570_e11669_d_n2;
        var_tmf1_rv = 0.0;

        let (assign9580_e11683, assign9580_e11683_d_n0, assign9580_e11683_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9580_e11679: f64 = (4.0 * p.p85);
        let assign9580_e11681: f64 = (assign9580_e11679 * 0.01);
        (assign9580_e11681, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9580_e11683;
        var_tmf2_dn0 = assign9580_e11683_d_n0;
        var_tmf2_dn2 = assign9580_e11683_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9590_e11699, assign9590_e11699_d_n0, assign9590_e11699_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let (assign9590_e11697, assign9590_e11697_d_n0, assign9590_e11697_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign9590_e11696: f64 = (-var_tmf2);
                (assign9590_e11696, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign9590_e11697, assign9590_e11697_d_n0, assign9590_e11697_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9590_e11699;
        var_tmf2_dn0 = assign9590_e11699_d_n0;
        var_tmf2_dn2 = assign9590_e11699_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9600_e11714, assign9600_e11714_d_n0, assign9600_e11714_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9600_e11709: f64 = (var_tmf1 * var_tmf1);
        let assign9600_e11711: f64 = (assign9600_e11709 + var_tmf2);
        let assign9600_e11712: f64 = (assign9600_e11711).sqrt();
        (assign9600_e11712, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign9600_e11712)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign9600_e11712)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9600_e11714;
        var_tmf2_dn0 = assign9600_e11714_d_n0;
        var_tmf2_dn2 = assign9600_e11714_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9610_e11730, assign9610_e11730_d_n0, assign9610_e11730_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9610_e11726: f64 = (var_tmf1 + var_tmf2);
        let assign9610_e11727: f64 = (0.5 * assign9610_e11726);
        let assign9610_e11728: f64 = (p.p85 - assign9610_e11727);
        (assign9610_e11728, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign9610_e11730;
        var_nja11_dn0 = assign9610_e11730_d_n0;
        var_nja11_dn2 = assign9610_e11730_d_n2;
        var_nja11_rv = 0.0;

        let (assign9620_e11744, assign9620_e11744_d_n0, assign9620_e11744_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9620_e11740: f64 = (var_nja11 - var_nfagat_i);
        let assign9620_e11742: f64 = (assign9620_e11740 - 0.01);
        (assign9620_e11742, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign9620_e11744;
        var_tmf1_dn0 = assign9620_e11744_d_n0;
        var_tmf1_dn2 = assign9620_e11744_d_n2;
        var_tmf1_rv = 0.0;

        let (assign9630_e11758, assign9630_e11758_d_n0, assign9630_e11758_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9630_e11754: f64 = (4.0 * var_nfagat_i);
        let assign9630_e11756: f64 = (assign9630_e11754 * 0.01);
        (assign9630_e11756, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9630_e11758;
        var_tmf2_dn0 = assign9630_e11758_d_n0;
        var_tmf2_dn2 = assign9630_e11758_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9640_e11774, assign9640_e11774_d_n0, assign9640_e11774_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let (assign9640_e11772, assign9640_e11772_d_n0, assign9640_e11772_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign9640_e11771: f64 = (-var_tmf2);
                (assign9640_e11771, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign9640_e11772, assign9640_e11772_d_n0, assign9640_e11772_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9640_e11774;
        var_tmf2_dn0 = assign9640_e11774_d_n0;
        var_tmf2_dn2 = assign9640_e11774_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9650_e11789, assign9650_e11789_d_n0, assign9650_e11789_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9650_e11784: f64 = (var_tmf1 * var_tmf1);
        let assign9650_e11786: f64 = (assign9650_e11784 + var_tmf2);
        let assign9650_e11787: f64 = (assign9650_e11786).sqrt();
        (assign9650_e11787, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign9650_e11787)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign9650_e11787)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9650_e11789;
        var_tmf2_dn0 = assign9650_e11789_d_n0;
        var_tmf2_dn2 = assign9650_e11789_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9660_e11805, assign9660_e11805_d_n0, assign9660_e11805_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9660_e11801: f64 = (var_tmf1 + var_tmf2);
        let assign9660_e11802: f64 = (0.5 * assign9660_e11801);
        let assign9660_e11803: f64 = (var_nfagat_i + assign9660_e11802);
        (assign9660_e11803, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign9660_e11805;
        var_nj1_dn0 = assign9660_e11805_d_n0;
        var_nj1_dn2 = assign9660_e11805_d_n2;
        var_nj1_rv = 0.0;

        let (assign9670_e11819, assign9670_e11819_d_n0, assign9670_e11819_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9670_e11815: f64 = (p.p85 - var_nj0);
        let assign9670_e11817: f64 = (assign9670_e11815 - 0.01);
        (assign9670_e11817, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign9670_e11819;
        var_tmf1_dn0 = assign9670_e11819_d_n0;
        var_tmf1_dn2 = assign9670_e11819_d_n2;
        var_tmf1_rv = 0.0;

        let (assign9680_e11833, assign9680_e11833_d_n0, assign9680_e11833_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9680_e11829: f64 = (4.0 * p.p85);
        let assign9680_e11831: f64 = (assign9680_e11829 * 0.01);
        (assign9680_e11831, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9680_e11833;
        var_tmf2_dn0 = assign9680_e11833_d_n0;
        var_tmf2_dn2 = assign9680_e11833_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9690_e11849, assign9690_e11849_d_n0, assign9690_e11849_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let (assign9690_e11847, assign9690_e11847_d_n0, assign9690_e11847_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign9690_e11846: f64 = (-var_tmf2);
                (assign9690_e11846, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign9690_e11847, assign9690_e11847_d_n0, assign9690_e11847_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9690_e11849;
        var_tmf2_dn0 = assign9690_e11849_d_n0;
        var_tmf2_dn2 = assign9690_e11849_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9700_e11864, assign9700_e11864_d_n0, assign9700_e11864_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9700_e11859: f64 = (var_tmf1 * var_tmf1);
        let assign9700_e11861: f64 = (assign9700_e11859 + var_tmf2);
        let assign9700_e11862: f64 = (assign9700_e11861).sqrt();
        (assign9700_e11862, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign9700_e11862)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign9700_e11862)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9700_e11864;
        var_tmf2_dn0 = assign9700_e11864_d_n0;
        var_tmf2_dn2 = assign9700_e11864_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9710_e11880, assign9710_e11880_d_n0, assign9710_e11880_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9710_e11876: f64 = (var_tmf1 + var_tmf2);
        let assign9710_e11877: f64 = (0.5 * assign9710_e11876);
        let assign9710_e11878: f64 = (p.p85 - assign9710_e11877);
        (assign9710_e11878, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign9710_e11880;
        var_nj0_dn0 = assign9710_e11880_d_n0;
        var_nj0_dn2 = assign9710_e11880_d_n2;
        var_nj0_rv = 0.0;

        let (assign9720_e11894, assign9720_e11894_d_n0, assign9720_e11894_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9720_e11890: f64 = (var_nj0 - var_nfagat_i);
        let assign9720_e11892: f64 = (assign9720_e11890 - 0.01);
        (assign9720_e11892, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign9720_e11894;
        var_tmf1_dn0 = assign9720_e11894_d_n0;
        var_tmf1_dn2 = assign9720_e11894_d_n2;
        var_tmf1_rv = 0.0;

        let (assign9730_e11908, assign9730_e11908_d_n0, assign9730_e11908_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9730_e11904: f64 = (4.0 * var_nfagat_i);
        let assign9730_e11906: f64 = (assign9730_e11904 * 0.01);
        (assign9730_e11906, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9730_e11908;
        var_tmf2_dn0 = assign9730_e11908_d_n0;
        var_tmf2_dn2 = assign9730_e11908_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9740_e11924, assign9740_e11924_d_n0, assign9740_e11924_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let (assign9740_e11922, assign9740_e11922_d_n0, assign9740_e11922_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign9740_e11921: f64 = (-var_tmf2);
                (assign9740_e11921, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign9740_e11922, assign9740_e11922_d_n0, assign9740_e11922_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9740_e11924;
        var_tmf2_dn0 = assign9740_e11924_d_n0;
        var_tmf2_dn2 = assign9740_e11924_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9750_e11939, assign9750_e11939_d_n0, assign9750_e11939_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9750_e11934: f64 = (var_tmf1 * var_tmf1);
        let assign9750_e11936: f64 = (assign9750_e11934 + var_tmf2);
        let assign9750_e11937: f64 = (assign9750_e11936).sqrt();
        (assign9750_e11937, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign9750_e11937)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign9750_e11937)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9750_e11939;
        var_tmf2_dn0 = assign9750_e11939_d_n0;
        var_tmf2_dn2 = assign9750_e11939_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9760_e11955, assign9760_e11955_d_n0, assign9760_e11955_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 != 0.0)) {
        let assign9760_e11951: f64 = (var_tmf1 + var_tmf2);
        let assign9760_e11952: f64 = (0.5 * assign9760_e11951);
        let assign9760_e11953: f64 = (var_nfagat_i + assign9760_e11952);
        (assign9760_e11953, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign9760_e11955;
        var_nj0_dn0 = assign9760_e11955_d_n0;
        var_nj0_dn2 = assign9760_e11955_d_n2;
        var_nj0_rv = 0.0;

        let (assign9770_e11966, assign9770_e11966_d_n0, assign9770_e11966_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign9770_e11966;
        var_nj0_dn0 = assign9770_e11966_d_n0;
        var_nj0_dn2 = assign9770_e11966_d_n2;
        var_nj0_rv = 0.0;

        let (assign9780_e11977, assign9780_e11977_d_n0, assign9780_e11977_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 != 0.0)) && (var_guard163 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign9780_e11977;
        var_nj1_dn0 = assign9780_e11977_d_n0;
        var_nj1_dn2 = assign9780_e11977_d_n2;
        var_nj1_rv = 0.0;

        let (assign9850_e12227,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) {
        let assign9850_e12223: f64 = (var_nin * var_nin);
        let assign9850_e12225: f64 = (assign9850_e12223 / var_ndibot_i);
        (assign9850_e12225,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign9850_e12227;
        var_pnn0_rv = 0.0;

        *var_guard163_slot = var_guard163;
        *var_guard163_rv_slot = var_guard163_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        var_guard153: f64,
        var_guard154: f64,
        var_guard31: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_phitdinv: f64,
        var_pnn0: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_rv_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rv_slot: &mut f64,
        var_guard166_slot: &mut f64,
        var_guard166_rv_slot: &mut f64,
        var_guard167_slot: &mut f64,
        var_guard167_rv_slot: &mut f64,
        var_guard168_slot: &mut f64,
        var_guard168_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_rv: f64 = *var_dfn_su_rv_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_exp_vmax_over_phitd_bot: f64 = *var_exp_vmax_over_phitd_bot_slot;
        let mut var_exp_vmax_over_phitd_bot_dn0: f64 = *var_exp_vmax_over_phitd_bot_dn0_slot;
        let mut var_exp_vmax_over_phitd_bot_dn2: f64 = *var_exp_vmax_over_phitd_bot_dn2_slot;
        let mut var_exp_vmax_over_phitd_bot_rv: f64 = *var_exp_vmax_over_phitd_bot_rv_slot;
        let mut var_guard166: f64 = *var_guard166_slot;
        let mut var_guard166_rv: f64 = *var_guard166_rv_slot;
        let mut var_guard167: f64 = *var_guard167_slot;
        let mut var_guard167_rv: f64 = *var_guard167_rv_slot;
        let mut var_guard168: f64 = *var_guard168_slot;
        let mut var_guard168_rv: f64 = *var_guard168_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign9860_e12243,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) {
        let assign9860_e12236: f64 = (var_nfabot_i / var_phitdinv);
        let assign9860_e12239: f64 = (var_ndibot_i / var_pnn0);
        let assign9860_e12240: f64 = (assign9860_e12239).ln();
        let assign9860_e12241: f64 = (assign9860_e12236 * assign9860_e12240);
        (assign9860_e12241,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign9860_e12243;
        var_vha1_rv = 0.0;

        let assign9870_e12246: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard166 = assign9870_e12246;
        var_guard166_rv = 0.0;

        let (assign9880_e12263, assign9880_e12263_d_n0, assign9880_e12263_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign9880_e12258: f64 = (var_vmax - var_vha1);
        let assign9880_e12259: f64 = (p.p86 * assign9880_e12258);
        let assign9880_e12261: f64 = (assign9880_e12259 + var_nfabot_i);
        (assign9880_e12261, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign9880_e12263;
        var_nja10_dn0 = assign9880_e12263_d_n0;
        var_nja10_dn2 = assign9880_e12263_d_n2;
        var_nja10_rv = 0.0;

        let (assign9890_e12278, assign9890_e12278_d_n0, assign9890_e12278_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign9890_e12275: f64 = (p.p86 * var_vha1);
        let assign9890_e12276: f64 = (var_nfabot_i - assign9890_e12275);
        (assign9890_e12276, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign9890_e12278;
        var_nj0_dn0 = assign9890_e12278_d_n0;
        var_nj0_dn2 = assign9890_e12278_d_n2;
        var_nj0_rv = 0.0;

        let (assign9900_e12293, assign9900_e12293_d_n0, assign9900_e12293_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign9900_e12289: f64 = (p.p85 - var_nja10);
        let assign9900_e12291: f64 = (assign9900_e12289 - 0.01);
        (assign9900_e12291, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign9900_e12293;
        var_tmf1_dn0 = assign9900_e12293_d_n0;
        var_tmf1_dn2 = assign9900_e12293_d_n2;
        var_tmf1_rv = 0.0;

        let (assign9910_e12308, assign9910_e12308_d_n0, assign9910_e12308_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign9910_e12304: f64 = (4.0 * p.p85);
        let assign9910_e12306: f64 = (assign9910_e12304 * 0.01);
        (assign9910_e12306, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9910_e12308;
        var_tmf2_dn0 = assign9910_e12308_d_n0;
        var_tmf2_dn2 = assign9910_e12308_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9920_e12325, assign9920_e12325_d_n0, assign9920_e12325_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let (assign9920_e12323, assign9920_e12323_d_n0, assign9920_e12323_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign9920_e12322: f64 = (-var_tmf2);
                (assign9920_e12322, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign9920_e12323, assign9920_e12323_d_n0, assign9920_e12323_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9920_e12325;
        var_tmf2_dn0 = assign9920_e12325_d_n0;
        var_tmf2_dn2 = assign9920_e12325_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9930_e12341, assign9930_e12341_d_n0, assign9930_e12341_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign9930_e12336: f64 = (var_tmf1 * var_tmf1);
        let assign9930_e12338: f64 = (assign9930_e12336 + var_tmf2);
        let assign9930_e12339: f64 = (assign9930_e12338).sqrt();
        (assign9930_e12339, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign9930_e12339)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign9930_e12339)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9930_e12341;
        var_tmf2_dn0 = assign9930_e12341_d_n0;
        var_tmf2_dn2 = assign9930_e12341_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9940_e12358, assign9940_e12358_d_n0, assign9940_e12358_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign9940_e12354: f64 = (var_tmf1 / var_tmf2);
        let assign9940_e12355: f64 = (1.0 + assign9940_e12354);
        let assign9940_e12356: f64 = (0.5 * assign9940_e12355);
        (assign9940_e12356, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign9940_e12358;
        var_dfn_su_dn0 = assign9940_e12358_d_n0;
        var_dfn_su_dn2 = assign9940_e12358_d_n2;
        var_dfn_su_rv = 0.0;

        let (assign9950_e12375, assign9950_e12375_d_n0, assign9950_e12375_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign9950_e12371: f64 = (var_tmf1 + var_tmf2);
        let assign9950_e12372: f64 = (0.5 * assign9950_e12371);
        let assign9950_e12373: f64 = (p.p85 - assign9950_e12372);
        (assign9950_e12373, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign9950_e12375;
        var_nja11_dn0 = assign9950_e12375_d_n0;
        var_nja11_dn2 = assign9950_e12375_d_n2;
        var_nja11_rv = 0.0;

        let (assign9960_e12390, assign9960_e12390_d_n0, assign9960_e12390_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign9960_e12386: f64 = (var_nja11 - var_nfabot_i);
        let assign9960_e12388: f64 = (assign9960_e12386 - 0.01);
        (assign9960_e12388, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign9960_e12390;
        var_tmf1_dn0 = assign9960_e12390_d_n0;
        var_tmf1_dn2 = assign9960_e12390_d_n2;
        var_tmf1_rv = 0.0;

        let (assign9970_e12405, assign9970_e12405_d_n0, assign9970_e12405_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign9970_e12401: f64 = (4.0 * var_nfabot_i);
        let assign9970_e12403: f64 = (assign9970_e12401 * 0.01);
        (assign9970_e12403, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9970_e12405;
        var_tmf2_dn0 = assign9970_e12405_d_n0;
        var_tmf2_dn2 = assign9970_e12405_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9980_e12422, assign9980_e12422_d_n0, assign9980_e12422_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let (assign9980_e12420, assign9980_e12420_d_n0, assign9980_e12420_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign9980_e12419: f64 = (-var_tmf2);
                (assign9980_e12419, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign9980_e12420, assign9980_e12420_d_n0, assign9980_e12420_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9980_e12422;
        var_tmf2_dn0 = assign9980_e12422_d_n0;
        var_tmf2_dn2 = assign9980_e12422_d_n2;
        var_tmf2_rv = 0.0;

        let (assign9990_e12438, assign9990_e12438_d_n0, assign9990_e12438_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign9990_e12433: f64 = (var_tmf1 * var_tmf1);
        let assign9990_e12435: f64 = (assign9990_e12433 + var_tmf2);
        let assign9990_e12436: f64 = (assign9990_e12435).sqrt();
        (assign9990_e12436, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign9990_e12436)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign9990_e12436)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign9990_e12438;
        var_tmf2_dn0 = assign9990_e12438_d_n0;
        var_tmf2_dn2 = assign9990_e12438_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10000_e12455, assign10000_e12455_d_n0, assign10000_e12455_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign10000_e12451: f64 = (var_tmf1 / var_tmf2);
        let assign10000_e12452: f64 = (1.0 + assign10000_e12451);
        let assign10000_e12453: f64 = (0.5 * assign10000_e12452);
        (assign10000_e12453, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign10000_e12455;
        var_dfn_sl_dn0 = assign10000_e12455_d_n0;
        var_dfn_sl_dn2 = assign10000_e12455_d_n2;
        var_dfn_sl_rv = 0.0;

        let (assign10010_e12472, assign10010_e12472_d_n0, assign10010_e12472_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign10010_e12468: f64 = (var_tmf1 + var_tmf2);
        let assign10010_e12469: f64 = (0.5 * assign10010_e12468);
        let assign10010_e12470: f64 = (var_nfabot_i + assign10010_e12469);
        (assign10010_e12470, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign10010_e12472;
        var_nj1_dn0 = assign10010_e12472_d_n0;
        var_nj1_dn2 = assign10010_e12472_d_n2;
        var_nj1_rv = 0.0;

        let (assign10020_e12487, assign10020_e12487_d_n0, assign10020_e12487_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign10020_e12483: f64 = (p.p85 - var_nj0);
        let assign10020_e12485: f64 = (assign10020_e12483 - 0.01);
        (assign10020_e12485, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign10020_e12487;
        var_tmf1_dn0 = assign10020_e12487_d_n0;
        var_tmf1_dn2 = assign10020_e12487_d_n2;
        var_tmf1_rv = 0.0;

        let (assign10030_e12502, assign10030_e12502_d_n0, assign10030_e12502_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign10030_e12498: f64 = (4.0 * p.p85);
        let assign10030_e12500: f64 = (assign10030_e12498 * 0.01);
        (assign10030_e12500, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10030_e12502;
        var_tmf2_dn0 = assign10030_e12502_d_n0;
        var_tmf2_dn2 = assign10030_e12502_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10040_e12519, assign10040_e12519_d_n0, assign10040_e12519_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let (assign10040_e12517, assign10040_e12517_d_n0, assign10040_e12517_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign10040_e12516: f64 = (-var_tmf2);
                (assign10040_e12516, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign10040_e12517, assign10040_e12517_d_n0, assign10040_e12517_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10040_e12519;
        var_tmf2_dn0 = assign10040_e12519_d_n0;
        var_tmf2_dn2 = assign10040_e12519_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10050_e12535, assign10050_e12535_d_n0, assign10050_e12535_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign10050_e12530: f64 = (var_tmf1 * var_tmf1);
        let assign10050_e12532: f64 = (assign10050_e12530 + var_tmf2);
        let assign10050_e12533: f64 = (assign10050_e12532).sqrt();
        (assign10050_e12533, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign10050_e12533)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign10050_e12533)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10050_e12535;
        var_tmf2_dn0 = assign10050_e12535_d_n0;
        var_tmf2_dn2 = assign10050_e12535_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10060_e12552, assign10060_e12552_d_n0, assign10060_e12552_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign10060_e12548: f64 = (var_tmf1 + var_tmf2);
        let assign10060_e12549: f64 = (0.5 * assign10060_e12548);
        let assign10060_e12550: f64 = (p.p85 - assign10060_e12549);
        (assign10060_e12550, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign10060_e12552;
        var_nj0_dn0 = assign10060_e12552_d_n0;
        var_nj0_dn2 = assign10060_e12552_d_n2;
        var_nj0_rv = 0.0;

        let (assign10070_e12567, assign10070_e12567_d_n0, assign10070_e12567_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign10070_e12563: f64 = (var_nj0 - var_nfabot_i);
        let assign10070_e12565: f64 = (assign10070_e12563 - 0.01);
        (assign10070_e12565, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign10070_e12567;
        var_tmf1_dn0 = assign10070_e12567_d_n0;
        var_tmf1_dn2 = assign10070_e12567_d_n2;
        var_tmf1_rv = 0.0;

        let (assign10080_e12582, assign10080_e12582_d_n0, assign10080_e12582_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign10080_e12578: f64 = (4.0 * var_nfabot_i);
        let assign10080_e12580: f64 = (assign10080_e12578 * 0.01);
        (assign10080_e12580, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10080_e12582;
        var_tmf2_dn0 = assign10080_e12582_d_n0;
        var_tmf2_dn2 = assign10080_e12582_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10090_e12599, assign10090_e12599_d_n0, assign10090_e12599_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let (assign10090_e12597, assign10090_e12597_d_n0, assign10090_e12597_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign10090_e12596: f64 = (-var_tmf2);
                (assign10090_e12596, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign10090_e12597, assign10090_e12597_d_n0, assign10090_e12597_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10090_e12599;
        var_tmf2_dn0 = assign10090_e12599_d_n0;
        var_tmf2_dn2 = assign10090_e12599_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10100_e12615, assign10100_e12615_d_n0, assign10100_e12615_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign10100_e12610: f64 = (var_tmf1 * var_tmf1);
        let assign10100_e12612: f64 = (assign10100_e12610 + var_tmf2);
        let assign10100_e12613: f64 = (assign10100_e12612).sqrt();
        (assign10100_e12613, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign10100_e12613)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign10100_e12613)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10100_e12615;
        var_tmf2_dn0 = assign10100_e12615_d_n0;
        var_tmf2_dn2 = assign10100_e12615_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10110_e12632, assign10110_e12632_d_n0, assign10110_e12632_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign10110_e12628: f64 = (var_tmf1 + var_tmf2);
        let assign10110_e12629: f64 = (0.5 * assign10110_e12628);
        let assign10110_e12630: f64 = (var_nfabot_i + assign10110_e12629);
        (assign10110_e12630, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign10110_e12632;
        var_nj0_dn0 = assign10110_e12632_d_n0;
        var_nj0_dn2 = assign10110_e12632_d_n2;
        var_nj0_rv = 0.0;

        let (assign10120_e12647, assign10120_e12647_d_n0, assign10120_e12647_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 != 0.0)) {
        let assign10120_e12643: f64 = (p.p86 * var_dfn_su);
        let assign10120_e12645: f64 = (assign10120_e12643 * var_dfn_sl);
        (assign10120_e12645, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign10120_e12643 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign10120_e12643 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign10120_e12647;
        var_dnj1_dv_dn0 = assign10120_e12647_d_n0;
        var_dnj1_dv_dn2 = assign10120_e12647_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign10130_e12659, assign10130_e12659_d_n0, assign10130_e12659_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign10130_e12659;
        var_nj0_dn0 = assign10130_e12659_d_n0;
        var_nj0_dn2 = assign10130_e12659_d_n2;
        var_nj0_rv = 0.0;

        let (assign10140_e12671, assign10140_e12671_d_n0, assign10140_e12671_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign10140_e12671;
        var_nj1_dn0 = assign10140_e12671_d_n0;
        var_nj1_dn2 = assign10140_e12671_d_n2;
        var_nj1_rv = 0.0;

        let (assign10150_e12683, assign10150_e12683_d_n0, assign10150_e12683_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard166 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign10150_e12683;
        var_dnj1_dv_dn0 = assign10150_e12683_d_n0;
        var_dnj1_dv_dn2 = assign10150_e12683_d_n2;
        var_dnj1_dv_rv = 0.0;

        let assign10160_e12687: f64 = (var_vmax / var_nj1);
        let assign10160_e12691: f64 = (var_nj1 - var_nj0);
        let assign10160_e12692: f64 = (var_vha1 * assign10160_e12691);
        let assign10160_e12695: f64 = (var_nj0 * p.p85);
        let assign10160_e12696: f64 = (assign10160_e12692 / assign10160_e12695);
        let assign10160_e12697: f64 = (assign10160_e12687 + assign10160_e12696);
        let assign10160_e12698: f64 = (var_phitdinv * assign10160_e12697);
        let assign10160_e12699: f64 = (assign10160_e12698).abs();
        let assign10160_e12701: f64 = if assign10160_e12699 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard167 = assign10160_e12701;
        var_guard167_rv = 0.0;

        let (assign10170_e12727, assign10170_e12727_d_n0, assign10170_e12727_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard167 != 0.0)) {
        let assign10170_e12713: f64 = (var_vmax / var_nj1);
        let assign10170_e12717: f64 = (var_nj1 - var_nj0);
        let assign10170_e12718: f64 = (var_vha1 * assign10170_e12717);
        let assign10170_e12721: f64 = (var_nj0 * p.p85);
        let assign10170_e12722: f64 = (assign10170_e12718 / assign10170_e12721);
        let assign10170_e12723: f64 = (assign10170_e12713 + assign10170_e12722);
        let assign10170_e12724: f64 = (var_phitdinv * assign10170_e12723);
        let assign10170_e12725: f64 = (assign10170_e12724).exp();
        (assign10170_e12725, (assign10170_e12725 * (var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign10170_e12721) - (assign10170_e12718 * (var_nj0_dn0 * p.p85))) / (assign10170_e12721 * assign10170_e12721))))), (assign10170_e12725 * (var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign10170_e12721) - (assign10170_e12718 * (var_nj0_dn2 * p.p85))) / (assign10170_e12721 * assign10170_e12721))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign10170_e12727;
        var_exp_vmax_over_phitd_bot_dn0 = assign10170_e12727_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign10170_e12727_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let assign10180_e12731: f64 = (var_vmax / var_nj1);
        let assign10180_e12735: f64 = (var_nj1 - var_nj0);
        let assign10180_e12736: f64 = (var_vha1 * assign10180_e12735);
        let assign10180_e12739: f64 = (var_nj0 * p.p85);
        let assign10180_e12740: f64 = (assign10180_e12736 / assign10180_e12739);
        let assign10180_e12741: f64 = (assign10180_e12731 + assign10180_e12740);
        let assign10180_e12742: f64 = (var_phitdinv * assign10180_e12741);
        let assign10180_e12744: f64 = (-230.25850929940458);
        let assign10180_e12745: f64 = if assign10180_e12742 < assign10180_e12744 { 1.0 } else { 0.0 };
        var_guard168 = assign10180_e12745;
        var_guard168_rv = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_exp_vmax_over_phitd_bot_slot = var_exp_vmax_over_phitd_bot;
        *var_exp_vmax_over_phitd_bot_dn0_slot = var_exp_vmax_over_phitd_bot_dn0;
        *var_exp_vmax_over_phitd_bot_dn2_slot = var_exp_vmax_over_phitd_bot_dn2;
        *var_exp_vmax_over_phitd_bot_rv_slot = var_exp_vmax_over_phitd_bot_rv;
        *var_guard166_slot = var_guard166;
        *var_guard166_rv_slot = var_guard166_rv;
        *var_guard167_slot = var_guard167;
        *var_guard167_rv_slot = var_guard167_rv;
        *var_guard168_slot = var_guard168;
        *var_guard168_rv_slot = var_guard168_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        var_dnj1_dv: f64,
        var_dnj1_dv_dn0: f64,
        var_dnj1_dv_dn2: f64,
        var_guard153: f64,
        var_guard154: f64,
        var_guard167: f64,
        var_guard168: f64,
        var_guard31: f64,
        var_ndisti_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v2: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dfn_su_slot: &mut f64,
        var_dfn_su_dn0_slot: &mut f64,
        var_dfn_su_dn2_slot: &mut f64,
        var_dfn_su_rv_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rv_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn0_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_dn2_slot: &mut f64,
        var_exp_vmax_over_phitd_bot_rv_slot: &mut f64,
        var_guard169_slot: &mut f64,
        var_guard169_rv_slot: &mut f64,
        var_idmultbot_slot: &mut f64,
        var_idmultbot_dn0_slot: &mut f64,
        var_idmultbot_dn2_slot: &mut f64,
        var_idmultbot_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj1_slot: &mut f64,
        var_nj1_dn0_slot: &mut f64,
        var_nj1_dn2_slot: &mut f64,
        var_nj1_rv_slot: &mut f64,
        var_nja10_slot: &mut f64,
        var_nja10_dn0_slot: &mut f64,
        var_nja10_dn2_slot: &mut f64,
        var_nja10_rv_slot: &mut f64,
        var_nja11_slot: &mut f64,
        var_nja11_dn0_slot: &mut f64,
        var_nja11_dn2_slot: &mut f64,
        var_nja11_rv_slot: &mut f64,
        var_pnn0_slot: &mut f64,
        var_pnn0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dfn_su: f64 = *var_dfn_su_slot;
        let mut var_dfn_su_dn0: f64 = *var_dfn_su_dn0_slot;
        let mut var_dfn_su_dn2: f64 = *var_dfn_su_dn2_slot;
        let mut var_dfn_su_rv: f64 = *var_dfn_su_rv_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_rv: f64 = *var_dvmax_over_phitd_dv_rv_slot;
        let mut var_exp_vmax_over_phitd_bot: f64 = *var_exp_vmax_over_phitd_bot_slot;
        let mut var_exp_vmax_over_phitd_bot_dn0: f64 = *var_exp_vmax_over_phitd_bot_dn0_slot;
        let mut var_exp_vmax_over_phitd_bot_dn2: f64 = *var_exp_vmax_over_phitd_bot_dn2_slot;
        let mut var_exp_vmax_over_phitd_bot_rv: f64 = *var_exp_vmax_over_phitd_bot_rv_slot;
        let mut var_guard169: f64 = *var_guard169_slot;
        let mut var_guard169_rv: f64 = *var_guard169_rv_slot;
        let mut var_idmultbot: f64 = *var_idmultbot_slot;
        let mut var_idmultbot_dn0: f64 = *var_idmultbot_dn0_slot;
        let mut var_idmultbot_dn2: f64 = *var_idmultbot_dn2_slot;
        let mut var_idmultbot_rv: f64 = *var_idmultbot_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj1: f64 = *var_nj1_slot;
        let mut var_nj1_dn0: f64 = *var_nj1_dn0_slot;
        let mut var_nj1_dn2: f64 = *var_nj1_dn2_slot;
        let mut var_nj1_rv: f64 = *var_nj1_rv_slot;
        let mut var_nja10: f64 = *var_nja10_slot;
        let mut var_nja10_dn0: f64 = *var_nja10_dn0_slot;
        let mut var_nja10_dn2: f64 = *var_nja10_dn2_slot;
        let mut var_nja10_rv: f64 = *var_nja10_rv_slot;
        let mut var_nja11: f64 = *var_nja11_slot;
        let mut var_nja11_dn0: f64 = *var_nja11_dn0_slot;
        let mut var_nja11_dn2: f64 = *var_nja11_dn2_slot;
        let mut var_nja11_rv: f64 = *var_nja11_rv_slot;
        let mut var_pnn0: f64 = *var_pnn0_slot;
        let mut var_pnn0_rv: f64 = *var_pnn0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign10190_e12826, assign10190_e12826_d_n0, assign10190_e12826_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard167 == 0.0)) && (var_guard168 != 0.0)) {
        let assign10190_e12760: f64 = (-230.25850929940458);
        let assign10190_e12764: f64 = (var_vmax / var_nj1);
        let assign10190_e12768: f64 = (var_nj1 - var_nj0);
        let assign10190_e12769: f64 = (var_vha1 * assign10190_e12768);
        let assign10190_e12772: f64 = (var_nj0 * p.p85);
        let assign10190_e12773: f64 = (assign10190_e12769 / assign10190_e12772);
        let assign10190_e12774: f64 = (assign10190_e12764 + assign10190_e12773);
        let assign10190_e12775: f64 = (var_phitdinv * assign10190_e12774);
        let assign10190_e12776: f64 = (assign10190_e12760 - assign10190_e12775);
        let assign10190_e12780: f64 = (-230.25850929940458);
        let assign10190_e12784: f64 = (var_vmax / var_nj1);
        let assign10190_e12788: f64 = (var_nj1 - var_nj0);
        let assign10190_e12789: f64 = (var_vha1 * assign10190_e12788);
        let assign10190_e12792: f64 = (var_nj0 * p.p85);
        let assign10190_e12793: f64 = (assign10190_e12789 / assign10190_e12792);
        let assign10190_e12794: f64 = (assign10190_e12784 + assign10190_e12793);
        let assign10190_e12795: f64 = (var_phitdinv * assign10190_e12794);
        let assign10190_e12796: f64 = (assign10190_e12780 - assign10190_e12795);
        let assign10190_e12799: f64 = (-230.25850929940458);
        let assign10190_e12803: f64 = (var_vmax / var_nj1);
        let assign10190_e12807: f64 = (var_nj1 - var_nj0);
        let assign10190_e12808: f64 = (var_vha1 * assign10190_e12807);
        let assign10190_e12811: f64 = (var_nj0 * p.p85);
        let assign10190_e12812: f64 = (assign10190_e12808 / assign10190_e12811);
        let assign10190_e12813: f64 = (assign10190_e12803 + assign10190_e12812);
        let assign10190_e12814: f64 = (var_phitdinv * assign10190_e12813);
        let assign10190_e12815: f64 = (assign10190_e12799 - assign10190_e12814);
        let assign10190_e12817: f64 = (assign10190_e12815 * 0.3333333333333333);
        let assign10190_e12818: f64 = (1.0 + assign10190_e12817);
        let assign10190_e12819: f64 = (assign10190_e12796 * assign10190_e12818);
        let assign10190_e12820: f64 = (0.5 * assign10190_e12819);
        let assign10190_e12821: f64 = (1.0 + assign10190_e12820);
        let assign10190_e12822: f64 = (assign10190_e12776 * assign10190_e12821);
        let assign10190_e12823: f64 = (1.0 + assign10190_e12822);
        let assign10190_e12824: f64 = (1e-100 / assign10190_e12823);
        (assign10190_e12824, (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign10190_e12772) - (assign10190_e12769 * (var_nj0_dn0 * p.p85))) / (assign10190_e12772 * assign10190_e12772))))) * assign10190_e12821) + (assign10190_e12776 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign10190_e12792) - (assign10190_e12789 * (var_nj0_dn0 * p.p85))) / (assign10190_e12792 * assign10190_e12792))))) * assign10190_e12818) + (assign10190_e12796 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign10190_e12811) - (assign10190_e12808 * (var_nj0_dn0 * p.p85))) / (assign10190_e12811 * assign10190_e12811))))) * 0.3333333333333333))))))) / (assign10190_e12823 * assign10190_e12823))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign10190_e12772) - (assign10190_e12769 * (var_nj0_dn2 * p.p85))) / (assign10190_e12772 * assign10190_e12772))))) * assign10190_e12821) + (assign10190_e12776 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign10190_e12792) - (assign10190_e12789 * (var_nj0_dn2 * p.p85))) / (assign10190_e12792 * assign10190_e12792))))) * assign10190_e12818) + (assign10190_e12796 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign10190_e12811) - (assign10190_e12808 * (var_nj0_dn2 * p.p85))) / (assign10190_e12811 * assign10190_e12811))))) * 0.3333333333333333))))))) / (assign10190_e12823 * assign10190_e12823))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign10190_e12826;
        var_exp_vmax_over_phitd_bot_dn0 = assign10190_e12826_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign10190_e12826_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let (assign10200_e12905, assign10200_e12905_d_n0, assign10200_e12905_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard167 == 0.0)) && (var_guard168 == 0.0)) {
        let assign10200_e12844: f64 = (var_vmax / var_nj1);
        let assign10200_e12848: f64 = (var_nj1 - var_nj0);
        let assign10200_e12849: f64 = (var_vha1 * assign10200_e12848);
        let assign10200_e12852: f64 = (var_nj0 * p.p85);
        let assign10200_e12853: f64 = (assign10200_e12849 / assign10200_e12852);
        let assign10200_e12854: f64 = (assign10200_e12844 + assign10200_e12853);
        let assign10200_e12855: f64 = (var_phitdinv * assign10200_e12854);
        let assign10200_e12857: f64 = (assign10200_e12855 - 230.25850929940458);
        let assign10200_e12863: f64 = (var_vmax / var_nj1);
        let assign10200_e12867: f64 = (var_nj1 - var_nj0);
        let assign10200_e12868: f64 = (var_vha1 * assign10200_e12867);
        let assign10200_e12871: f64 = (var_nj0 * p.p85);
        let assign10200_e12872: f64 = (assign10200_e12868 / assign10200_e12871);
        let assign10200_e12873: f64 = (assign10200_e12863 + assign10200_e12872);
        let assign10200_e12874: f64 = (var_phitdinv * assign10200_e12873);
        let assign10200_e12876: f64 = (assign10200_e12874 - 230.25850929940458);
        let assign10200_e12881: f64 = (var_vmax / var_nj1);
        let assign10200_e12885: f64 = (var_nj1 - var_nj0);
        let assign10200_e12886: f64 = (var_vha1 * assign10200_e12885);
        let assign10200_e12889: f64 = (var_nj0 * p.p85);
        let assign10200_e12890: f64 = (assign10200_e12886 / assign10200_e12889);
        let assign10200_e12891: f64 = (assign10200_e12881 + assign10200_e12890);
        let assign10200_e12892: f64 = (var_phitdinv * assign10200_e12891);
        let assign10200_e12894: f64 = (assign10200_e12892 - 230.25850929940458);
        let assign10200_e12896: f64 = (assign10200_e12894 * 0.3333333333333333);
        let assign10200_e12897: f64 = (1.0 + assign10200_e12896);
        let assign10200_e12898: f64 = (assign10200_e12876 * assign10200_e12897);
        let assign10200_e12899: f64 = (0.5 * assign10200_e12898);
        let assign10200_e12900: f64 = (1.0 + assign10200_e12899);
        let assign10200_e12901: f64 = (assign10200_e12857 * assign10200_e12900);
        let assign10200_e12902: f64 = (1.0 + assign10200_e12901);
        let assign10200_e12903: f64 = (1e100 * assign10200_e12902);
        (assign10200_e12903, (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign10200_e12852) - (assign10200_e12849 * (var_nj0_dn0 * p.p85))) / (assign10200_e12852 * assign10200_e12852)))) * assign10200_e12900) + (assign10200_e12857 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign10200_e12871) - (assign10200_e12868 * (var_nj0_dn0 * p.p85))) / (assign10200_e12871 * assign10200_e12871)))) * assign10200_e12897) + (assign10200_e12876 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign10200_e12889) - (assign10200_e12886 * (var_nj0_dn0 * p.p85))) / (assign10200_e12889 * assign10200_e12889)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign10200_e12852) - (assign10200_e12849 * (var_nj0_dn2 * p.p85))) / (assign10200_e12852 * assign10200_e12852)))) * assign10200_e12900) + (assign10200_e12857 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign10200_e12871) - (assign10200_e12868 * (var_nj0_dn2 * p.p85))) / (assign10200_e12871 * assign10200_e12871)))) * assign10200_e12897) + (assign10200_e12876 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign10200_e12889) - (assign10200_e12886 * (var_nj0_dn2 * p.p85))) / (assign10200_e12889 * assign10200_e12889)))) * 0.3333333333333333))))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign10200_e12905;
        var_exp_vmax_over_phitd_bot_dn0 = assign10200_e12905_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign10200_e12905_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let (assign10210_e12932, assign10210_e12932_d_n0, assign10210_e12932_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) {
        let assign10210_e12916: f64 = (var_vmax * var_dnj1_dv);
        let assign10210_e12917: f64 = (var_nj1 - assign10210_e12916);
        let assign10210_e12920: f64 = (var_nj1 * var_nj1);
        let assign10210_e12921: f64 = (assign10210_e12917 / assign10210_e12920);
        let assign10210_e12924: f64 = (var_vha1 * var_dnj1_dv);
        let assign10210_e12927: f64 = (var_nj0 * p.p85);
        let assign10210_e12928: f64 = (assign10210_e12924 / assign10210_e12927);
        let assign10210_e12929: f64 = (assign10210_e12921 + assign10210_e12928);
        let assign10210_e12930: f64 = (var_phitdinv * assign10210_e12929);
        (assign10210_e12930, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign10210_e12920) - (assign10210_e12917 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign10210_e12920 * assign10210_e12920)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign10210_e12927) - (assign10210_e12924 * (var_nj0_dn0 * p.p85))) / (assign10210_e12927 * assign10210_e12927)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign10210_e12920) - (assign10210_e12917 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign10210_e12920 * assign10210_e12920)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign10210_e12927) - (assign10210_e12924 * (var_nj0_dn2 * p.p85))) / (assign10210_e12927 * assign10210_e12927)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign10210_e12932;
        var_dvmax_over_phitd_dv_dn0 = assign10210_e12932_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign10210_e12932_d_n2;
        var_dvmax_over_phitd_dv_rv = 0.0;

        let (assign10220_e12949, assign10220_e12949_d_n0, assign10220_e12949_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) {
        let assign10220_e12942: f64 = (var_v2 - var_vmax);
        let assign10220_e12944: f64 = (assign10220_e12942 * var_dvmax_over_phitd_dv);
        let assign10220_e12945: f64 = (1.0 + assign10220_e12944);
        let assign10220_e12947: f64 = (assign10220_e12945 * var_exp_vmax_over_phitd_bot);
        (assign10220_e12947, (((assign10220_e12942 * var_dvmax_over_phitd_dv_dn0) * var_exp_vmax_over_phitd_bot) + (assign10220_e12945 * var_exp_vmax_over_phitd_bot_dn0)), (((assign10220_e12942 * var_dvmax_over_phitd_dv_dn2) * var_exp_vmax_over_phitd_bot) + (assign10220_e12945 * var_exp_vmax_over_phitd_bot_dn2)),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign10220_e12949;
        var_idmultbot_dn0 = assign10220_e12949_d_n0;
        var_idmultbot_dn2 = assign10220_e12949_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign10230_e12962,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) {
        let assign10230_e12958: f64 = (var_nin * var_nin);
        let assign10230_e12960: f64 = (assign10230_e12958 / var_ndisti_i);
        (assign10230_e12960,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign10230_e12962;
        var_pnn0_rv = 0.0;

        let (assign10240_e12978,) = {
    if (((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) {
        let assign10240_e12971: f64 = (var_nfasti_i / var_phitdinv);
        let assign10240_e12974: f64 = (var_ndisti_i / var_pnn0);
        let assign10240_e12975: f64 = (assign10240_e12974).ln();
        let assign10240_e12976: f64 = (assign10240_e12971 * assign10240_e12975);
        (assign10240_e12976,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign10240_e12978;
        var_vha1_rv = 0.0;

        let assign10250_e12981: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard169 = assign10250_e12981;
        var_guard169_rv = 0.0;

        let (assign10260_e12998, assign10260_e12998_d_n0, assign10260_e12998_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10260_e12993: f64 = (var_vmax - var_vha1);
        let assign10260_e12994: f64 = (p.p86 * assign10260_e12993);
        let assign10260_e12996: f64 = (assign10260_e12994 + var_nfasti_i);
        (assign10260_e12996, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign10260_e12998;
        var_nja10_dn0 = assign10260_e12998_d_n0;
        var_nja10_dn2 = assign10260_e12998_d_n2;
        var_nja10_rv = 0.0;

        let (assign10270_e13013, assign10270_e13013_d_n0, assign10270_e13013_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10270_e13010: f64 = (p.p86 * var_vha1);
        let assign10270_e13011: f64 = (var_nfasti_i - assign10270_e13010);
        (assign10270_e13011, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign10270_e13013;
        var_nj0_dn0 = assign10270_e13013_d_n0;
        var_nj0_dn2 = assign10270_e13013_d_n2;
        var_nj0_rv = 0.0;

        let (assign10280_e13028, assign10280_e13028_d_n0, assign10280_e13028_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10280_e13024: f64 = (p.p85 - var_nja10);
        let assign10280_e13026: f64 = (assign10280_e13024 - 0.01);
        (assign10280_e13026, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign10280_e13028;
        var_tmf1_dn0 = assign10280_e13028_d_n0;
        var_tmf1_dn2 = assign10280_e13028_d_n2;
        var_tmf1_rv = 0.0;

        let (assign10290_e13043, assign10290_e13043_d_n0, assign10290_e13043_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10290_e13039: f64 = (4.0 * p.p85);
        let assign10290_e13041: f64 = (assign10290_e13039 * 0.01);
        (assign10290_e13041, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10290_e13043;
        var_tmf2_dn0 = assign10290_e13043_d_n0;
        var_tmf2_dn2 = assign10290_e13043_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10300_e13060, assign10300_e13060_d_n0, assign10300_e13060_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let (assign10300_e13058, assign10300_e13058_d_n0, assign10300_e13058_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign10300_e13057: f64 = (-var_tmf2);
                (assign10300_e13057, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign10300_e13058, assign10300_e13058_d_n0, assign10300_e13058_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10300_e13060;
        var_tmf2_dn0 = assign10300_e13060_d_n0;
        var_tmf2_dn2 = assign10300_e13060_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10310_e13076, assign10310_e13076_d_n0, assign10310_e13076_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10310_e13071: f64 = (var_tmf1 * var_tmf1);
        let assign10310_e13073: f64 = (assign10310_e13071 + var_tmf2);
        let assign10310_e13074: f64 = (assign10310_e13073).sqrt();
        (assign10310_e13074, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign10310_e13074)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign10310_e13074)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10310_e13076;
        var_tmf2_dn0 = assign10310_e13076_d_n0;
        var_tmf2_dn2 = assign10310_e13076_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10320_e13093, assign10320_e13093_d_n0, assign10320_e13093_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10320_e13089: f64 = (var_tmf1 / var_tmf2);
        let assign10320_e13090: f64 = (1.0 + assign10320_e13089);
        let assign10320_e13091: f64 = (0.5 * assign10320_e13090);
        (assign10320_e13091, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign10320_e13093;
        var_dfn_su_dn0 = assign10320_e13093_d_n0;
        var_dfn_su_dn2 = assign10320_e13093_d_n2;
        var_dfn_su_rv = 0.0;

        let (assign10330_e13110, assign10330_e13110_d_n0, assign10330_e13110_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10330_e13106: f64 = (var_tmf1 + var_tmf2);
        let assign10330_e13107: f64 = (0.5 * assign10330_e13106);
        let assign10330_e13108: f64 = (p.p85 - assign10330_e13107);
        (assign10330_e13108, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign10330_e13110;
        var_nja11_dn0 = assign10330_e13110_d_n0;
        var_nja11_dn2 = assign10330_e13110_d_n2;
        var_nja11_rv = 0.0;

        let (assign10340_e13125, assign10340_e13125_d_n0, assign10340_e13125_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10340_e13121: f64 = (var_nja11 - var_nfasti_i);
        let assign10340_e13123: f64 = (assign10340_e13121 - 0.01);
        (assign10340_e13123, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign10340_e13125;
        var_tmf1_dn0 = assign10340_e13125_d_n0;
        var_tmf1_dn2 = assign10340_e13125_d_n2;
        var_tmf1_rv = 0.0;

        let (assign10350_e13140, assign10350_e13140_d_n0, assign10350_e13140_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10350_e13136: f64 = (4.0 * var_nfasti_i);
        let assign10350_e13138: f64 = (assign10350_e13136 * 0.01);
        (assign10350_e13138, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10350_e13140;
        var_tmf2_dn0 = assign10350_e13140_d_n0;
        var_tmf2_dn2 = assign10350_e13140_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10360_e13157, assign10360_e13157_d_n0, assign10360_e13157_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let (assign10360_e13155, assign10360_e13155_d_n0, assign10360_e13155_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign10360_e13154: f64 = (-var_tmf2);
                (assign10360_e13154, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign10360_e13155, assign10360_e13155_d_n0, assign10360_e13155_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10360_e13157;
        var_tmf2_dn0 = assign10360_e13157_d_n0;
        var_tmf2_dn2 = assign10360_e13157_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10370_e13173, assign10370_e13173_d_n0, assign10370_e13173_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10370_e13168: f64 = (var_tmf1 * var_tmf1);
        let assign10370_e13170: f64 = (assign10370_e13168 + var_tmf2);
        let assign10370_e13171: f64 = (assign10370_e13170).sqrt();
        (assign10370_e13171, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign10370_e13171)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign10370_e13171)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10370_e13173;
        var_tmf2_dn0 = assign10370_e13173_d_n0;
        var_tmf2_dn2 = assign10370_e13173_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10380_e13190, assign10380_e13190_d_n0, assign10380_e13190_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10380_e13186: f64 = (var_tmf1 / var_tmf2);
        let assign10380_e13187: f64 = (1.0 + assign10380_e13186);
        let assign10380_e13188: f64 = (0.5 * assign10380_e13187);
        (assign10380_e13188, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign10380_e13190;
        var_dfn_sl_dn0 = assign10380_e13190_d_n0;
        var_dfn_sl_dn2 = assign10380_e13190_d_n2;
        var_dfn_sl_rv = 0.0;

        let (assign10390_e13207, assign10390_e13207_d_n0, assign10390_e13207_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10390_e13203: f64 = (var_tmf1 + var_tmf2);
        let assign10390_e13204: f64 = (0.5 * assign10390_e13203);
        let assign10390_e13205: f64 = (var_nfasti_i + assign10390_e13204);
        (assign10390_e13205, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign10390_e13207;
        var_nj1_dn0 = assign10390_e13207_d_n0;
        var_nj1_dn2 = assign10390_e13207_d_n2;
        var_nj1_rv = 0.0;

        let (assign10400_e13222, assign10400_e13222_d_n0, assign10400_e13222_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10400_e13218: f64 = (p.p85 - var_nj0);
        let assign10400_e13220: f64 = (assign10400_e13218 - 0.01);
        (assign10400_e13220, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign10400_e13222;
        var_tmf1_dn0 = assign10400_e13222_d_n0;
        var_tmf1_dn2 = assign10400_e13222_d_n2;
        var_tmf1_rv = 0.0;

        let (assign10410_e13237, assign10410_e13237_d_n0, assign10410_e13237_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10410_e13233: f64 = (4.0 * p.p85);
        let assign10410_e13235: f64 = (assign10410_e13233 * 0.01);
        (assign10410_e13235, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10410_e13237;
        var_tmf2_dn0 = assign10410_e13237_d_n0;
        var_tmf2_dn2 = assign10410_e13237_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10420_e13254, assign10420_e13254_d_n0, assign10420_e13254_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let (assign10420_e13252, assign10420_e13252_d_n0, assign10420_e13252_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign10420_e13251: f64 = (-var_tmf2);
                (assign10420_e13251, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign10420_e13252, assign10420_e13252_d_n0, assign10420_e13252_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10420_e13254;
        var_tmf2_dn0 = assign10420_e13254_d_n0;
        var_tmf2_dn2 = assign10420_e13254_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10430_e13270, assign10430_e13270_d_n0, assign10430_e13270_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10430_e13265: f64 = (var_tmf1 * var_tmf1);
        let assign10430_e13267: f64 = (assign10430_e13265 + var_tmf2);
        let assign10430_e13268: f64 = (assign10430_e13267).sqrt();
        (assign10430_e13268, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign10430_e13268)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign10430_e13268)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10430_e13270;
        var_tmf2_dn0 = assign10430_e13270_d_n0;
        var_tmf2_dn2 = assign10430_e13270_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10440_e13287, assign10440_e13287_d_n0, assign10440_e13287_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10440_e13283: f64 = (var_tmf1 + var_tmf2);
        let assign10440_e13284: f64 = (0.5 * assign10440_e13283);
        let assign10440_e13285: f64 = (p.p85 - assign10440_e13284);
        (assign10440_e13285, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign10440_e13287;
        var_nj0_dn0 = assign10440_e13287_d_n0;
        var_nj0_dn2 = assign10440_e13287_d_n2;
        var_nj0_rv = 0.0;

        let (assign10450_e13302, assign10450_e13302_d_n0, assign10450_e13302_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10450_e13298: f64 = (var_nj0 - var_nfasti_i);
        let assign10450_e13300: f64 = (assign10450_e13298 - 0.01);
        (assign10450_e13300, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign10450_e13302;
        var_tmf1_dn0 = assign10450_e13302_d_n0;
        var_tmf1_dn2 = assign10450_e13302_d_n2;
        var_tmf1_rv = 0.0;

        let (assign10460_e13317, assign10460_e13317_d_n0, assign10460_e13317_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let assign10460_e13313: f64 = (4.0 * var_nfasti_i);
        let assign10460_e13315: f64 = (assign10460_e13313 * 0.01);
        (assign10460_e13315, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10460_e13317;
        var_tmf2_dn0 = assign10460_e13317_d_n0;
        var_tmf2_dn2 = assign10460_e13317_d_n2;
        var_tmf2_rv = 0.0;

        let (assign10470_e13334, assign10470_e13334_d_n0, assign10470_e13334_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard153 != 0.0)) && (var_guard154 == 0.0)) && (var_guard169 != 0.0)) {
        let (assign10470_e13332, assign10470_e13332_d_n0, assign10470_e13332_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign10470_e13331: f64 = (-var_tmf2);
                (assign10470_e13331, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign10470_e13332, assign10470_e13332_d_n0, assign10470_e13332_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign10470_e13334;
        var_tmf2_dn0 = assign10470_e13334_d_n0;
        var_tmf2_dn2 = assign10470_e13334_d_n2;
        var_tmf2_rv = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_rv_slot = var_dvmax_over_phitd_dv_rv;
        *var_exp_vmax_over_phitd_bot_slot = var_exp_vmax_over_phitd_bot;
        *var_exp_vmax_over_phitd_bot_dn0_slot = var_exp_vmax_over_phitd_bot_dn0;
        *var_exp_vmax_over_phitd_bot_dn2_slot = var_exp_vmax_over_phitd_bot_dn2;
        *var_exp_vmax_over_phitd_bot_rv_slot = var_exp_vmax_over_phitd_bot_rv;
        *var_guard169_slot = var_guard169;
        *var_guard169_rv_slot = var_guard169_rv;
        *var_idmultbot_slot = var_idmultbot;
        *var_idmultbot_dn0_slot = var_idmultbot_dn0;
        *var_idmultbot_dn2_slot = var_idmultbot_dn2;
        *var_idmultbot_rv_slot = var_idmultbot_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj1_slot = var_nj1;
        *var_nj1_dn0_slot = var_nj1_dn0;
        *var_nj1_dn2_slot = var_nj1_dn2;
        *var_nj1_rv_slot = var_nj1_rv;
        *var_nja10_slot = var_nja10;
        *var_nja10_dn0_slot = var_nja10_dn0;
        *var_nja10_dn2_slot = var_nja10_dn2;
        *var_nja10_rv_slot = var_nja10_rv;
        *var_nja11_slot = var_nja11;
        *var_nja11_dn0_slot = var_nja11_dn0;
        *var_nja11_dn2_slot = var_nja11_dn2;
        *var_nja11_rv_slot = var_nja11_rv;
        *var_pnn0_slot = var_pnn0;
        *var_pnn0_rv_slot = var_pnn0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }
}
