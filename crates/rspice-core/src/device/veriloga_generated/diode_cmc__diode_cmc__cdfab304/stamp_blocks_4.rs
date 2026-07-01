#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_28(
        p: &Parameters,
        var_dfn_sl: f64,
        var_dfn_sl_dn0: f64,
        var_dfn_sl_dn2: f64,
        var_guard31: f64,
        var_guard384: f64,
        var_guard385: f64,
        var_guard397: f64,
        var_ndisti_i: f64,
        var_nfabot_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_v5: f64,
        var_vmax: f64,
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
        var_guard398_slot: &mut f64,
        var_guard398_rv_slot: &mut f64,
        var_guard399_slot: &mut f64,
        var_guard399_rv_slot: &mut f64,
        var_guard400_slot: &mut f64,
        var_guard400_rv_slot: &mut f64,
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
        let mut var_guard398: f64 = *var_guard398_slot;
        let mut var_guard398_rv: f64 = *var_guard398_rv_slot;
        let mut var_guard399: f64 = *var_guard399_slot;
        let mut var_guard399_rv: f64 = *var_guard399_rv_slot;
        let mut var_guard400: f64 = *var_guard400_slot;
        let mut var_guard400_rv: f64 = *var_guard400_rv_slot;
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

        let (assign24710_e36393, assign24710_e36393_d_n0, assign24710_e36393_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let (assign24710_e36391, assign24710_e36391_d_n0, assign24710_e36391_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign24710_e36390: f64 = (-var_tmf2);
                (assign24710_e36390, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign24710_e36391, assign24710_e36391_d_n0, assign24710_e36391_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24710_e36393;
        var_tmf2_dn0 = assign24710_e36393_d_n0;
        var_tmf2_dn2 = assign24710_e36393_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24720_e36409, assign24720_e36409_d_n0, assign24720_e36409_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24720_e36404: f64 = (var_tmf1 * var_tmf1);
        let assign24720_e36406: f64 = (assign24720_e36404 + var_tmf2);
        let assign24720_e36407: f64 = (assign24720_e36406).sqrt();
        (assign24720_e36407, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24720_e36407)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24720_e36407)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24720_e36409;
        var_tmf2_dn0 = assign24720_e36409_d_n0;
        var_tmf2_dn2 = assign24720_e36409_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24730_e36426, assign24730_e36426_d_n0, assign24730_e36426_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24730_e36422: f64 = (var_tmf1 + var_tmf2);
        let assign24730_e36423: f64 = (0.5 * assign24730_e36422);
        let assign24730_e36424: f64 = (p.p85 - assign24730_e36423);
        (assign24730_e36424, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign24730_e36426;
        var_nj0_dn0 = assign24730_e36426_d_n0;
        var_nj0_dn2 = assign24730_e36426_d_n2;
        var_nj0_rv = 0.0;

        let (assign24740_e36441, assign24740_e36441_d_n0, assign24740_e36441_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24740_e36437: f64 = (var_nj0 - var_nfabot_i);
        let assign24740_e36439: f64 = (assign24740_e36437 - 0.01);
        (assign24740_e36439, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign24740_e36441;
        var_tmf1_dn0 = assign24740_e36441_d_n0;
        var_tmf1_dn2 = assign24740_e36441_d_n2;
        var_tmf1_rv = 0.0;

        let (assign24750_e36456, assign24750_e36456_d_n0, assign24750_e36456_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24750_e36452: f64 = (4.0 * var_nfabot_i);
        let assign24750_e36454: f64 = (assign24750_e36452 * 0.01);
        (assign24750_e36454, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24750_e36456;
        var_tmf2_dn0 = assign24750_e36456_d_n0;
        var_tmf2_dn2 = assign24750_e36456_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24760_e36473, assign24760_e36473_d_n0, assign24760_e36473_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let (assign24760_e36471, assign24760_e36471_d_n0, assign24760_e36471_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign24760_e36470: f64 = (-var_tmf2);
                (assign24760_e36470, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign24760_e36471, assign24760_e36471_d_n0, assign24760_e36471_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24760_e36473;
        var_tmf2_dn0 = assign24760_e36473_d_n0;
        var_tmf2_dn2 = assign24760_e36473_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24770_e36489, assign24770_e36489_d_n0, assign24770_e36489_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24770_e36484: f64 = (var_tmf1 * var_tmf1);
        let assign24770_e36486: f64 = (assign24770_e36484 + var_tmf2);
        let assign24770_e36487: f64 = (assign24770_e36486).sqrt();
        (assign24770_e36487, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24770_e36487)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24770_e36487)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24770_e36489;
        var_tmf2_dn0 = assign24770_e36489_d_n0;
        var_tmf2_dn2 = assign24770_e36489_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24780_e36506, assign24780_e36506_d_n0, assign24780_e36506_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24780_e36502: f64 = (var_tmf1 + var_tmf2);
        let assign24780_e36503: f64 = (0.5 * assign24780_e36502);
        let assign24780_e36504: f64 = (var_nfabot_i + assign24780_e36503);
        (assign24780_e36504, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign24780_e36506;
        var_nj0_dn0 = assign24780_e36506_d_n0;
        var_nj0_dn2 = assign24780_e36506_d_n2;
        var_nj0_rv = 0.0;

        let (assign24790_e36521, assign24790_e36521_d_n0, assign24790_e36521_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 != 0.0)) {
        let assign24790_e36517: f64 = (p.p86 * var_dfn_su);
        let assign24790_e36519: f64 = (assign24790_e36517 * var_dfn_sl);
        (assign24790_e36519, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign24790_e36517 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign24790_e36517 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign24790_e36521;
        var_dnj1_dv_dn0 = assign24790_e36521_d_n0;
        var_dnj1_dv_dn2 = assign24790_e36521_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign24800_e36533, assign24800_e36533_d_n0, assign24800_e36533_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign24800_e36533;
        var_nj0_dn0 = assign24800_e36533_d_n0;
        var_nj0_dn2 = assign24800_e36533_d_n2;
        var_nj0_rv = 0.0;

        let (assign24810_e36545, assign24810_e36545_d_n0, assign24810_e36545_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign24810_e36545;
        var_nj1_dn0 = assign24810_e36545_d_n0;
        var_nj1_dn2 = assign24810_e36545_d_n2;
        var_nj1_rv = 0.0;

        let (assign24820_e36557, assign24820_e36557_d_n0, assign24820_e36557_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard397 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign24820_e36557;
        var_dnj1_dv_dn0 = assign24820_e36557_d_n0;
        var_dnj1_dv_dn2 = assign24820_e36557_d_n2;
        var_dnj1_dv_rv = 0.0;

        let assign24830_e36561: f64 = (var_vmax / var_nj1);
        let assign24830_e36565: f64 = (var_nj1 - var_nj0);
        let assign24830_e36566: f64 = (var_vha1 * assign24830_e36565);
        let assign24830_e36569: f64 = (var_nj0 * p.p85);
        let assign24830_e36570: f64 = (assign24830_e36566 / assign24830_e36569);
        let assign24830_e36571: f64 = (assign24830_e36561 + assign24830_e36570);
        let assign24830_e36572: f64 = (var_phitdinv * assign24830_e36571);
        let assign24830_e36573: f64 = (assign24830_e36572).abs();
        let assign24830_e36575: f64 = if assign24830_e36573 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard398 = assign24830_e36575;
        var_guard398_rv = 0.0;

        let (assign24840_e36601, assign24840_e36601_d_n0, assign24840_e36601_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard398 != 0.0)) {
        let assign24840_e36587: f64 = (var_vmax / var_nj1);
        let assign24840_e36591: f64 = (var_nj1 - var_nj0);
        let assign24840_e36592: f64 = (var_vha1 * assign24840_e36591);
        let assign24840_e36595: f64 = (var_nj0 * p.p85);
        let assign24840_e36596: f64 = (assign24840_e36592 / assign24840_e36595);
        let assign24840_e36597: f64 = (assign24840_e36587 + assign24840_e36596);
        let assign24840_e36598: f64 = (var_phitdinv * assign24840_e36597);
        let assign24840_e36599: f64 = (assign24840_e36598).exp();
        (assign24840_e36599, (assign24840_e36599 * (var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign24840_e36595) - (assign24840_e36592 * (var_nj0_dn0 * p.p85))) / (assign24840_e36595 * assign24840_e36595))))), (assign24840_e36599 * (var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign24840_e36595) - (assign24840_e36592 * (var_nj0_dn2 * p.p85))) / (assign24840_e36595 * assign24840_e36595))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign24840_e36601;
        var_exp_vmax_over_phitd_bot_dn0 = assign24840_e36601_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign24840_e36601_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let assign24850_e36605: f64 = (var_vmax / var_nj1);
        let assign24850_e36609: f64 = (var_nj1 - var_nj0);
        let assign24850_e36610: f64 = (var_vha1 * assign24850_e36609);
        let assign24850_e36613: f64 = (var_nj0 * p.p85);
        let assign24850_e36614: f64 = (assign24850_e36610 / assign24850_e36613);
        let assign24850_e36615: f64 = (assign24850_e36605 + assign24850_e36614);
        let assign24850_e36616: f64 = (var_phitdinv * assign24850_e36615);
        let assign24850_e36618: f64 = (-230.25850929940458);
        let assign24850_e36619: f64 = if assign24850_e36616 < assign24850_e36618 { 1.0 } else { 0.0 };
        var_guard399 = assign24850_e36619;
        var_guard399_rv = 0.0;

        let (assign24860_e36700, assign24860_e36700_d_n0, assign24860_e36700_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard398 == 0.0)) && (var_guard399 != 0.0)) {
        let assign24860_e36634: f64 = (-230.25850929940458);
        let assign24860_e36638: f64 = (var_vmax / var_nj1);
        let assign24860_e36642: f64 = (var_nj1 - var_nj0);
        let assign24860_e36643: f64 = (var_vha1 * assign24860_e36642);
        let assign24860_e36646: f64 = (var_nj0 * p.p85);
        let assign24860_e36647: f64 = (assign24860_e36643 / assign24860_e36646);
        let assign24860_e36648: f64 = (assign24860_e36638 + assign24860_e36647);
        let assign24860_e36649: f64 = (var_phitdinv * assign24860_e36648);
        let assign24860_e36650: f64 = (assign24860_e36634 - assign24860_e36649);
        let assign24860_e36654: f64 = (-230.25850929940458);
        let assign24860_e36658: f64 = (var_vmax / var_nj1);
        let assign24860_e36662: f64 = (var_nj1 - var_nj0);
        let assign24860_e36663: f64 = (var_vha1 * assign24860_e36662);
        let assign24860_e36666: f64 = (var_nj0 * p.p85);
        let assign24860_e36667: f64 = (assign24860_e36663 / assign24860_e36666);
        let assign24860_e36668: f64 = (assign24860_e36658 + assign24860_e36667);
        let assign24860_e36669: f64 = (var_phitdinv * assign24860_e36668);
        let assign24860_e36670: f64 = (assign24860_e36654 - assign24860_e36669);
        let assign24860_e36673: f64 = (-230.25850929940458);
        let assign24860_e36677: f64 = (var_vmax / var_nj1);
        let assign24860_e36681: f64 = (var_nj1 - var_nj0);
        let assign24860_e36682: f64 = (var_vha1 * assign24860_e36681);
        let assign24860_e36685: f64 = (var_nj0 * p.p85);
        let assign24860_e36686: f64 = (assign24860_e36682 / assign24860_e36685);
        let assign24860_e36687: f64 = (assign24860_e36677 + assign24860_e36686);
        let assign24860_e36688: f64 = (var_phitdinv * assign24860_e36687);
        let assign24860_e36689: f64 = (assign24860_e36673 - assign24860_e36688);
        let assign24860_e36691: f64 = (assign24860_e36689 * 0.3333333333333333);
        let assign24860_e36692: f64 = (1.0 + assign24860_e36691);
        let assign24860_e36693: f64 = (assign24860_e36670 * assign24860_e36692);
        let assign24860_e36694: f64 = (0.5 * assign24860_e36693);
        let assign24860_e36695: f64 = (1.0 + assign24860_e36694);
        let assign24860_e36696: f64 = (assign24860_e36650 * assign24860_e36695);
        let assign24860_e36697: f64 = (1.0 + assign24860_e36696);
        let assign24860_e36698: f64 = (1e-100 / assign24860_e36697);
        (assign24860_e36698, (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign24860_e36646) - (assign24860_e36643 * (var_nj0_dn0 * p.p85))) / (assign24860_e36646 * assign24860_e36646))))) * assign24860_e36695) + (assign24860_e36650 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign24860_e36666) - (assign24860_e36663 * (var_nj0_dn0 * p.p85))) / (assign24860_e36666 * assign24860_e36666))))) * assign24860_e36692) + (assign24860_e36670 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign24860_e36685) - (assign24860_e36682 * (var_nj0_dn0 * p.p85))) / (assign24860_e36685 * assign24860_e36685))))) * 0.3333333333333333))))))) / (assign24860_e36697 * assign24860_e36697))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign24860_e36646) - (assign24860_e36643 * (var_nj0_dn2 * p.p85))) / (assign24860_e36646 * assign24860_e36646))))) * assign24860_e36695) + (assign24860_e36650 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign24860_e36666) - (assign24860_e36663 * (var_nj0_dn2 * p.p85))) / (assign24860_e36666 * assign24860_e36666))))) * assign24860_e36692) + (assign24860_e36670 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign24860_e36685) - (assign24860_e36682 * (var_nj0_dn2 * p.p85))) / (assign24860_e36685 * assign24860_e36685))))) * 0.3333333333333333))))))) / (assign24860_e36697 * assign24860_e36697))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign24860_e36700;
        var_exp_vmax_over_phitd_bot_dn0 = assign24860_e36700_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign24860_e36700_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let (assign24870_e36779, assign24870_e36779_d_n0, assign24870_e36779_d_n2,) = {
    if (((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard398 == 0.0)) && (var_guard399 == 0.0)) {
        let assign24870_e36718: f64 = (var_vmax / var_nj1);
        let assign24870_e36722: f64 = (var_nj1 - var_nj0);
        let assign24870_e36723: f64 = (var_vha1 * assign24870_e36722);
        let assign24870_e36726: f64 = (var_nj0 * p.p85);
        let assign24870_e36727: f64 = (assign24870_e36723 / assign24870_e36726);
        let assign24870_e36728: f64 = (assign24870_e36718 + assign24870_e36727);
        let assign24870_e36729: f64 = (var_phitdinv * assign24870_e36728);
        let assign24870_e36731: f64 = (assign24870_e36729 - 230.25850929940458);
        let assign24870_e36737: f64 = (var_vmax / var_nj1);
        let assign24870_e36741: f64 = (var_nj1 - var_nj0);
        let assign24870_e36742: f64 = (var_vha1 * assign24870_e36741);
        let assign24870_e36745: f64 = (var_nj0 * p.p85);
        let assign24870_e36746: f64 = (assign24870_e36742 / assign24870_e36745);
        let assign24870_e36747: f64 = (assign24870_e36737 + assign24870_e36746);
        let assign24870_e36748: f64 = (var_phitdinv * assign24870_e36747);
        let assign24870_e36750: f64 = (assign24870_e36748 - 230.25850929940458);
        let assign24870_e36755: f64 = (var_vmax / var_nj1);
        let assign24870_e36759: f64 = (var_nj1 - var_nj0);
        let assign24870_e36760: f64 = (var_vha1 * assign24870_e36759);
        let assign24870_e36763: f64 = (var_nj0 * p.p85);
        let assign24870_e36764: f64 = (assign24870_e36760 / assign24870_e36763);
        let assign24870_e36765: f64 = (assign24870_e36755 + assign24870_e36764);
        let assign24870_e36766: f64 = (var_phitdinv * assign24870_e36765);
        let assign24870_e36768: f64 = (assign24870_e36766 - 230.25850929940458);
        let assign24870_e36770: f64 = (assign24870_e36768 * 0.3333333333333333);
        let assign24870_e36771: f64 = (1.0 + assign24870_e36770);
        let assign24870_e36772: f64 = (assign24870_e36750 * assign24870_e36771);
        let assign24870_e36773: f64 = (0.5 * assign24870_e36772);
        let assign24870_e36774: f64 = (1.0 + assign24870_e36773);
        let assign24870_e36775: f64 = (assign24870_e36731 * assign24870_e36774);
        let assign24870_e36776: f64 = (1.0 + assign24870_e36775);
        let assign24870_e36777: f64 = (1e100 * assign24870_e36776);
        (assign24870_e36777, (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign24870_e36726) - (assign24870_e36723 * (var_nj0_dn0 * p.p85))) / (assign24870_e36726 * assign24870_e36726)))) * assign24870_e36774) + (assign24870_e36731 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign24870_e36745) - (assign24870_e36742 * (var_nj0_dn0 * p.p85))) / (assign24870_e36745 * assign24870_e36745)))) * assign24870_e36771) + (assign24870_e36750 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign24870_e36763) - (assign24870_e36760 * (var_nj0_dn0 * p.p85))) / (assign24870_e36763 * assign24870_e36763)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign24870_e36726) - (assign24870_e36723 * (var_nj0_dn2 * p.p85))) / (assign24870_e36726 * assign24870_e36726)))) * assign24870_e36774) + (assign24870_e36731 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign24870_e36745) - (assign24870_e36742 * (var_nj0_dn2 * p.p85))) / (assign24870_e36745 * assign24870_e36745)))) * assign24870_e36771) + (assign24870_e36750 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign24870_e36763) - (assign24870_e36760 * (var_nj0_dn2 * p.p85))) / (assign24870_e36763 * assign24870_e36763)))) * 0.3333333333333333))))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign24870_e36779;
        var_exp_vmax_over_phitd_bot_dn0 = assign24870_e36779_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign24870_e36779_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let (assign24880_e36806, assign24880_e36806_d_n0, assign24880_e36806_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign24880_e36790: f64 = (var_vmax * var_dnj1_dv);
        let assign24880_e36791: f64 = (var_nj1 - assign24880_e36790);
        let assign24880_e36794: f64 = (var_nj1 * var_nj1);
        let assign24880_e36795: f64 = (assign24880_e36791 / assign24880_e36794);
        let assign24880_e36798: f64 = (var_vha1 * var_dnj1_dv);
        let assign24880_e36801: f64 = (var_nj0 * p.p85);
        let assign24880_e36802: f64 = (assign24880_e36798 / assign24880_e36801);
        let assign24880_e36803: f64 = (assign24880_e36795 + assign24880_e36802);
        let assign24880_e36804: f64 = (var_phitdinv * assign24880_e36803);
        (assign24880_e36804, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign24880_e36794) - (assign24880_e36791 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign24880_e36794 * assign24880_e36794)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign24880_e36801) - (assign24880_e36798 * (var_nj0_dn0 * p.p85))) / (assign24880_e36801 * assign24880_e36801)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign24880_e36794) - (assign24880_e36791 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign24880_e36794 * assign24880_e36794)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign24880_e36801) - (assign24880_e36798 * (var_nj0_dn2 * p.p85))) / (assign24880_e36801 * assign24880_e36801)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign24880_e36806;
        var_dvmax_over_phitd_dv_dn0 = assign24880_e36806_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign24880_e36806_d_n2;
        var_dvmax_over_phitd_dv_rv = 0.0;

        let (assign24890_e36823, assign24890_e36823_d_n0, assign24890_e36823_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign24890_e36816: f64 = (var_v5 - var_vmax);
        let assign24890_e36818: f64 = (assign24890_e36816 * var_dvmax_over_phitd_dv);
        let assign24890_e36819: f64 = (1.0 + assign24890_e36818);
        let assign24890_e36821: f64 = (assign24890_e36819 * var_exp_vmax_over_phitd_bot);
        (assign24890_e36821, (((assign24890_e36816 * var_dvmax_over_phitd_dv_dn0) * var_exp_vmax_over_phitd_bot) + (assign24890_e36819 * var_exp_vmax_over_phitd_bot_dn0)), (((assign24890_e36816 * var_dvmax_over_phitd_dv_dn2) * var_exp_vmax_over_phitd_bot) + (assign24890_e36819 * var_exp_vmax_over_phitd_bot_dn2)),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign24890_e36823;
        var_idmultbot_dn0 = assign24890_e36823_d_n0;
        var_idmultbot_dn2 = assign24890_e36823_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign24900_e36836,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign24900_e36832: f64 = (var_nin * var_nin);
        let assign24900_e36834: f64 = (assign24900_e36832 / var_ndisti_i);
        (assign24900_e36834,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign24900_e36836;
        var_pnn0_rv = 0.0;

        let (assign24910_e36852,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign24910_e36845: f64 = (var_nfasti_i / var_phitdinv);
        let assign24910_e36848: f64 = (var_ndisti_i / var_pnn0);
        let assign24910_e36849: f64 = (assign24910_e36848).ln();
        let assign24910_e36850: f64 = (assign24910_e36845 * assign24910_e36849);
        (assign24910_e36850,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign24910_e36852;
        var_vha1_rv = 0.0;

        let assign24920_e36855: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard400 = assign24920_e36855;
        var_guard400_rv = 0.0;

        let (assign24930_e36872, assign24930_e36872_d_n0, assign24930_e36872_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign24930_e36867: f64 = (var_vmax - var_vha1);
        let assign24930_e36868: f64 = (p.p86 * assign24930_e36867);
        let assign24930_e36870: f64 = (assign24930_e36868 + var_nfasti_i);
        (assign24930_e36870, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign24930_e36872;
        var_nja10_dn0 = assign24930_e36872_d_n0;
        var_nja10_dn2 = assign24930_e36872_d_n2;
        var_nja10_rv = 0.0;

        let (assign24940_e36887, assign24940_e36887_d_n0, assign24940_e36887_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign24940_e36884: f64 = (p.p86 * var_vha1);
        let assign24940_e36885: f64 = (var_nfasti_i - assign24940_e36884);
        (assign24940_e36885, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign24940_e36887;
        var_nj0_dn0 = assign24940_e36887_d_n0;
        var_nj0_dn2 = assign24940_e36887_d_n2;
        var_nj0_rv = 0.0;

        let (assign24950_e36902, assign24950_e36902_d_n0, assign24950_e36902_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign24950_e36898: f64 = (p.p85 - var_nja10);
        let assign24950_e36900: f64 = (assign24950_e36898 - 0.01);
        (assign24950_e36900, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign24950_e36902;
        var_tmf1_dn0 = assign24950_e36902_d_n0;
        var_tmf1_dn2 = assign24950_e36902_d_n2;
        var_tmf1_rv = 0.0;

        let (assign24960_e36917, assign24960_e36917_d_n0, assign24960_e36917_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign24960_e36913: f64 = (4.0 * p.p85);
        let assign24960_e36915: f64 = (assign24960_e36913 * 0.01);
        (assign24960_e36915, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24960_e36917;
        var_tmf2_dn0 = assign24960_e36917_d_n0;
        var_tmf2_dn2 = assign24960_e36917_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24970_e36934, assign24970_e36934_d_n0, assign24970_e36934_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let (assign24970_e36932, assign24970_e36932_d_n0, assign24970_e36932_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign24970_e36931: f64 = (-var_tmf2);
                (assign24970_e36931, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign24970_e36932, assign24970_e36932_d_n0, assign24970_e36932_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24970_e36934;
        var_tmf2_dn0 = assign24970_e36934_d_n0;
        var_tmf2_dn2 = assign24970_e36934_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24980_e36950, assign24980_e36950_d_n0, assign24980_e36950_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign24980_e36945: f64 = (var_tmf1 * var_tmf1);
        let assign24980_e36947: f64 = (assign24980_e36945 + var_tmf2);
        let assign24980_e36948: f64 = (assign24980_e36947).sqrt();
        (assign24980_e36948, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign24980_e36948)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign24980_e36948)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign24980_e36950;
        var_tmf2_dn0 = assign24980_e36950_d_n0;
        var_tmf2_dn2 = assign24980_e36950_d_n2;
        var_tmf2_rv = 0.0;

        let (assign24990_e36967, assign24990_e36967_d_n0, assign24990_e36967_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign24990_e36963: f64 = (var_tmf1 / var_tmf2);
        let assign24990_e36964: f64 = (1.0 + assign24990_e36963);
        let assign24990_e36965: f64 = (0.5 * assign24990_e36964);
        (assign24990_e36965, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign24990_e36967;
        var_dfn_su_dn0 = assign24990_e36967_d_n0;
        var_dfn_su_dn2 = assign24990_e36967_d_n2;
        var_dfn_su_rv = 0.0;

        let (assign25000_e36984, assign25000_e36984_d_n0, assign25000_e36984_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25000_e36980: f64 = (var_tmf1 + var_tmf2);
        let assign25000_e36981: f64 = (0.5 * assign25000_e36980);
        let assign25000_e36982: f64 = (p.p85 - assign25000_e36981);
        (assign25000_e36982, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign25000_e36984;
        var_nja11_dn0 = assign25000_e36984_d_n0;
        var_nja11_dn2 = assign25000_e36984_d_n2;
        var_nja11_rv = 0.0;

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
        *var_guard398_slot = var_guard398;
        *var_guard398_rv_slot = var_guard398_rv;
        *var_guard399_slot = var_guard399;
        *var_guard399_rv_slot = var_guard399_rv;
        *var_guard400_slot = var_guard400;
        *var_guard400_rv_slot = var_guard400_rv;
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

    pub(super) fn stamp_reactive_block_29(
        p: &Parameters,
        var_guard31: f64,
        var_guard384: f64,
        var_guard385: f64,
        var_guard400: f64,
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
        var_guard403_slot: &mut f64,
        var_guard403_rv_slot: &mut f64,
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
        let mut var_guard403: f64 = *var_guard403_slot;
        let mut var_guard403_rv: f64 = *var_guard403_rv_slot;
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

        let (assign25010_e36999, assign25010_e36999_d_n0, assign25010_e36999_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25010_e36995: f64 = (var_nja11 - var_nfasti_i);
        let assign25010_e36997: f64 = (assign25010_e36995 - 0.01);
        (assign25010_e36997, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign25010_e36999;
        var_tmf1_dn0 = assign25010_e36999_d_n0;
        var_tmf1_dn2 = assign25010_e36999_d_n2;
        var_tmf1_rv = 0.0;

        let (assign25020_e37014, assign25020_e37014_d_n0, assign25020_e37014_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25020_e37010: f64 = (4.0 * var_nfasti_i);
        let assign25020_e37012: f64 = (assign25020_e37010 * 0.01);
        (assign25020_e37012, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25020_e37014;
        var_tmf2_dn0 = assign25020_e37014_d_n0;
        var_tmf2_dn2 = assign25020_e37014_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25030_e37031, assign25030_e37031_d_n0, assign25030_e37031_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let (assign25030_e37029, assign25030_e37029_d_n0, assign25030_e37029_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign25030_e37028: f64 = (-var_tmf2);
                (assign25030_e37028, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign25030_e37029, assign25030_e37029_d_n0, assign25030_e37029_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25030_e37031;
        var_tmf2_dn0 = assign25030_e37031_d_n0;
        var_tmf2_dn2 = assign25030_e37031_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25040_e37047, assign25040_e37047_d_n0, assign25040_e37047_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25040_e37042: f64 = (var_tmf1 * var_tmf1);
        let assign25040_e37044: f64 = (assign25040_e37042 + var_tmf2);
        let assign25040_e37045: f64 = (assign25040_e37044).sqrt();
        (assign25040_e37045, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign25040_e37045)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign25040_e37045)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25040_e37047;
        var_tmf2_dn0 = assign25040_e37047_d_n0;
        var_tmf2_dn2 = assign25040_e37047_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25050_e37064, assign25050_e37064_d_n0, assign25050_e37064_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25050_e37060: f64 = (var_tmf1 / var_tmf2);
        let assign25050_e37061: f64 = (1.0 + assign25050_e37060);
        let assign25050_e37062: f64 = (0.5 * assign25050_e37061);
        (assign25050_e37062, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign25050_e37064;
        var_dfn_sl_dn0 = assign25050_e37064_d_n0;
        var_dfn_sl_dn2 = assign25050_e37064_d_n2;
        var_dfn_sl_rv = 0.0;

        let (assign25060_e37081, assign25060_e37081_d_n0, assign25060_e37081_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25060_e37077: f64 = (var_tmf1 + var_tmf2);
        let assign25060_e37078: f64 = (0.5 * assign25060_e37077);
        let assign25060_e37079: f64 = (var_nfasti_i + assign25060_e37078);
        (assign25060_e37079, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign25060_e37081;
        var_nj1_dn0 = assign25060_e37081_d_n0;
        var_nj1_dn2 = assign25060_e37081_d_n2;
        var_nj1_rv = 0.0;

        let (assign25070_e37096, assign25070_e37096_d_n0, assign25070_e37096_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25070_e37092: f64 = (p.p85 - var_nj0);
        let assign25070_e37094: f64 = (assign25070_e37092 - 0.01);
        (assign25070_e37094, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign25070_e37096;
        var_tmf1_dn0 = assign25070_e37096_d_n0;
        var_tmf1_dn2 = assign25070_e37096_d_n2;
        var_tmf1_rv = 0.0;

        let (assign25080_e37111, assign25080_e37111_d_n0, assign25080_e37111_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25080_e37107: f64 = (4.0 * p.p85);
        let assign25080_e37109: f64 = (assign25080_e37107 * 0.01);
        (assign25080_e37109, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25080_e37111;
        var_tmf2_dn0 = assign25080_e37111_d_n0;
        var_tmf2_dn2 = assign25080_e37111_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25090_e37128, assign25090_e37128_d_n0, assign25090_e37128_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let (assign25090_e37126, assign25090_e37126_d_n0, assign25090_e37126_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign25090_e37125: f64 = (-var_tmf2);
                (assign25090_e37125, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign25090_e37126, assign25090_e37126_d_n0, assign25090_e37126_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25090_e37128;
        var_tmf2_dn0 = assign25090_e37128_d_n0;
        var_tmf2_dn2 = assign25090_e37128_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25100_e37144, assign25100_e37144_d_n0, assign25100_e37144_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25100_e37139: f64 = (var_tmf1 * var_tmf1);
        let assign25100_e37141: f64 = (assign25100_e37139 + var_tmf2);
        let assign25100_e37142: f64 = (assign25100_e37141).sqrt();
        (assign25100_e37142, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign25100_e37142)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign25100_e37142)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25100_e37144;
        var_tmf2_dn0 = assign25100_e37144_d_n0;
        var_tmf2_dn2 = assign25100_e37144_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25110_e37161, assign25110_e37161_d_n0, assign25110_e37161_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25110_e37157: f64 = (var_tmf1 + var_tmf2);
        let assign25110_e37158: f64 = (0.5 * assign25110_e37157);
        let assign25110_e37159: f64 = (p.p85 - assign25110_e37158);
        (assign25110_e37159, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign25110_e37161;
        var_nj0_dn0 = assign25110_e37161_d_n0;
        var_nj0_dn2 = assign25110_e37161_d_n2;
        var_nj0_rv = 0.0;

        let (assign25120_e37176, assign25120_e37176_d_n0, assign25120_e37176_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25120_e37172: f64 = (var_nj0 - var_nfasti_i);
        let assign25120_e37174: f64 = (assign25120_e37172 - 0.01);
        (assign25120_e37174, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign25120_e37176;
        var_tmf1_dn0 = assign25120_e37176_d_n0;
        var_tmf1_dn2 = assign25120_e37176_d_n2;
        var_tmf1_rv = 0.0;

        let (assign25130_e37191, assign25130_e37191_d_n0, assign25130_e37191_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25130_e37187: f64 = (4.0 * var_nfasti_i);
        let assign25130_e37189: f64 = (assign25130_e37187 * 0.01);
        (assign25130_e37189, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25130_e37191;
        var_tmf2_dn0 = assign25130_e37191_d_n0;
        var_tmf2_dn2 = assign25130_e37191_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25140_e37208, assign25140_e37208_d_n0, assign25140_e37208_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let (assign25140_e37206, assign25140_e37206_d_n0, assign25140_e37206_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign25140_e37205: f64 = (-var_tmf2);
                (assign25140_e37205, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign25140_e37206, assign25140_e37206_d_n0, assign25140_e37206_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25140_e37208;
        var_tmf2_dn0 = assign25140_e37208_d_n0;
        var_tmf2_dn2 = assign25140_e37208_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25150_e37224, assign25150_e37224_d_n0, assign25150_e37224_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25150_e37219: f64 = (var_tmf1 * var_tmf1);
        let assign25150_e37221: f64 = (assign25150_e37219 + var_tmf2);
        let assign25150_e37222: f64 = (assign25150_e37221).sqrt();
        (assign25150_e37222, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign25150_e37222)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign25150_e37222)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25150_e37224;
        var_tmf2_dn0 = assign25150_e37224_d_n0;
        var_tmf2_dn2 = assign25150_e37224_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25160_e37241, assign25160_e37241_d_n0, assign25160_e37241_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25160_e37237: f64 = (var_tmf1 + var_tmf2);
        let assign25160_e37238: f64 = (0.5 * assign25160_e37237);
        let assign25160_e37239: f64 = (var_nfasti_i + assign25160_e37238);
        (assign25160_e37239, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign25160_e37241;
        var_nj0_dn0 = assign25160_e37241_d_n0;
        var_nj0_dn2 = assign25160_e37241_d_n2;
        var_nj0_rv = 0.0;

        let (assign25170_e37256, assign25170_e37256_d_n0, assign25170_e37256_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 != 0.0)) {
        let assign25170_e37252: f64 = (p.p86 * var_dfn_su);
        let assign25170_e37254: f64 = (assign25170_e37252 * var_dfn_sl);
        (assign25170_e37254, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign25170_e37252 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign25170_e37252 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign25170_e37256;
        var_dnj1_dv_dn0 = assign25170_e37256_d_n0;
        var_dnj1_dv_dn2 = assign25170_e37256_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign25180_e37268, assign25180_e37268_d_n0, assign25180_e37268_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign25180_e37268;
        var_nj0_dn0 = assign25180_e37268_d_n0;
        var_nj0_dn2 = assign25180_e37268_d_n2;
        var_nj0_rv = 0.0;

        let (assign25190_e37280, assign25190_e37280_d_n0, assign25190_e37280_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign25190_e37280;
        var_nj1_dn0 = assign25190_e37280_d_n0;
        var_nj1_dn2 = assign25190_e37280_d_n2;
        var_nj1_rv = 0.0;

        let (assign25200_e37292, assign25200_e37292_d_n0, assign25200_e37292_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard400 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign25200_e37292;
        var_dnj1_dv_dn0 = assign25200_e37292_d_n0;
        var_dnj1_dv_dn2 = assign25200_e37292_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign25260_e37541, assign25260_e37541_d_n0, assign25260_e37541_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign25260_e37525: f64 = (var_vmax * var_dnj1_dv);
        let assign25260_e37526: f64 = (var_nj1 - assign25260_e37525);
        let assign25260_e37529: f64 = (var_nj1 * var_nj1);
        let assign25260_e37530: f64 = (assign25260_e37526 / assign25260_e37529);
        let assign25260_e37533: f64 = (var_vha1 * var_dnj1_dv);
        let assign25260_e37536: f64 = (var_nj0 * p.p85);
        let assign25260_e37537: f64 = (assign25260_e37533 / assign25260_e37536);
        let assign25260_e37538: f64 = (assign25260_e37530 + assign25260_e37537);
        let assign25260_e37539: f64 = (var_phitdinv * assign25260_e37538);
        (assign25260_e37539, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign25260_e37529) - (assign25260_e37526 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign25260_e37529 * assign25260_e37529)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign25260_e37536) - (assign25260_e37533 * (var_nj0_dn0 * p.p85))) / (assign25260_e37536 * assign25260_e37536)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign25260_e37529) - (assign25260_e37526 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign25260_e37529 * assign25260_e37529)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign25260_e37536) - (assign25260_e37533 * (var_nj0_dn2 * p.p85))) / (assign25260_e37536 * assign25260_e37536)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign25260_e37541;
        var_dvmax_over_phitd_dv_dn0 = assign25260_e37541_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign25260_e37541_d_n2;
        var_dvmax_over_phitd_dv_rv = 0.0;

        let (assign25280_e37571,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign25280_e37567: f64 = (var_nin * var_nin);
        let assign25280_e37569: f64 = (assign25280_e37567 / var_ndigat_i);
        (assign25280_e37569,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign25280_e37571;
        var_pnn0_rv = 0.0;

        let (assign25290_e37587,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign25290_e37580: f64 = (var_nfagat_i / var_phitdinv);
        let assign25290_e37583: f64 = (var_ndigat_i / var_pnn0);
        let assign25290_e37584: f64 = (assign25290_e37583).ln();
        let assign25290_e37585: f64 = (assign25290_e37580 * assign25290_e37584);
        (assign25290_e37585,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign25290_e37587;
        var_vha1_rv = 0.0;

        let assign25300_e37590: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard403 = assign25300_e37590;
        var_guard403_rv = 0.0;

        let (assign25310_e37607, assign25310_e37607_d_n0, assign25310_e37607_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25310_e37602: f64 = (var_vmax - var_vha1);
        let assign25310_e37603: f64 = (p.p86 * assign25310_e37602);
        let assign25310_e37605: f64 = (assign25310_e37603 + var_nfagat_i);
        (assign25310_e37605, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign25310_e37607;
        var_nja10_dn0 = assign25310_e37607_d_n0;
        var_nja10_dn2 = assign25310_e37607_d_n2;
        var_nja10_rv = 0.0;

        let (assign25320_e37622, assign25320_e37622_d_n0, assign25320_e37622_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25320_e37619: f64 = (p.p86 * var_vha1);
        let assign25320_e37620: f64 = (var_nfagat_i - assign25320_e37619);
        (assign25320_e37620, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign25320_e37622;
        var_nj0_dn0 = assign25320_e37622_d_n0;
        var_nj0_dn2 = assign25320_e37622_d_n2;
        var_nj0_rv = 0.0;

        let (assign25330_e37637, assign25330_e37637_d_n0, assign25330_e37637_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25330_e37633: f64 = (p.p85 - var_nja10);
        let assign25330_e37635: f64 = (assign25330_e37633 - 0.01);
        (assign25330_e37635, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign25330_e37637;
        var_tmf1_dn0 = assign25330_e37637_d_n0;
        var_tmf1_dn2 = assign25330_e37637_d_n2;
        var_tmf1_rv = 0.0;

        let (assign25340_e37652, assign25340_e37652_d_n0, assign25340_e37652_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25340_e37648: f64 = (4.0 * p.p85);
        let assign25340_e37650: f64 = (assign25340_e37648 * 0.01);
        (assign25340_e37650, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25340_e37652;
        var_tmf2_dn0 = assign25340_e37652_d_n0;
        var_tmf2_dn2 = assign25340_e37652_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25350_e37669, assign25350_e37669_d_n0, assign25350_e37669_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let (assign25350_e37667, assign25350_e37667_d_n0, assign25350_e37667_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign25350_e37666: f64 = (-var_tmf2);
                (assign25350_e37666, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign25350_e37667, assign25350_e37667_d_n0, assign25350_e37667_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25350_e37669;
        var_tmf2_dn0 = assign25350_e37669_d_n0;
        var_tmf2_dn2 = assign25350_e37669_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25360_e37685, assign25360_e37685_d_n0, assign25360_e37685_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25360_e37680: f64 = (var_tmf1 * var_tmf1);
        let assign25360_e37682: f64 = (assign25360_e37680 + var_tmf2);
        let assign25360_e37683: f64 = (assign25360_e37682).sqrt();
        (assign25360_e37683, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign25360_e37683)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign25360_e37683)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25360_e37685;
        var_tmf2_dn0 = assign25360_e37685_d_n0;
        var_tmf2_dn2 = assign25360_e37685_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25370_e37702, assign25370_e37702_d_n0, assign25370_e37702_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25370_e37698: f64 = (var_tmf1 / var_tmf2);
        let assign25370_e37699: f64 = (1.0 + assign25370_e37698);
        let assign25370_e37700: f64 = (0.5 * assign25370_e37699);
        (assign25370_e37700, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign25370_e37702;
        var_dfn_su_dn0 = assign25370_e37702_d_n0;
        var_dfn_su_dn2 = assign25370_e37702_d_n2;
        var_dfn_su_rv = 0.0;

        let (assign25380_e37719, assign25380_e37719_d_n0, assign25380_e37719_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25380_e37715: f64 = (var_tmf1 + var_tmf2);
        let assign25380_e37716: f64 = (0.5 * assign25380_e37715);
        let assign25380_e37717: f64 = (p.p85 - assign25380_e37716);
        (assign25380_e37717, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign25380_e37719;
        var_nja11_dn0 = assign25380_e37719_d_n0;
        var_nja11_dn2 = assign25380_e37719_d_n2;
        var_nja11_rv = 0.0;

        let (assign25390_e37734, assign25390_e37734_d_n0, assign25390_e37734_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25390_e37730: f64 = (var_nja11 - var_nfagat_i);
        let assign25390_e37732: f64 = (assign25390_e37730 - 0.01);
        (assign25390_e37732, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign25390_e37734;
        var_tmf1_dn0 = assign25390_e37734_d_n0;
        var_tmf1_dn2 = assign25390_e37734_d_n2;
        var_tmf1_rv = 0.0;

        let (assign25400_e37749, assign25400_e37749_d_n0, assign25400_e37749_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25400_e37745: f64 = (4.0 * var_nfagat_i);
        let assign25400_e37747: f64 = (assign25400_e37745 * 0.01);
        (assign25400_e37747, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25400_e37749;
        var_tmf2_dn0 = assign25400_e37749_d_n0;
        var_tmf2_dn2 = assign25400_e37749_d_n2;
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
        *var_guard403_slot = var_guard403;
        *var_guard403_rv_slot = var_guard403_rv;
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

    pub(super) fn stamp_reactive_block_30(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_ab_i: f64,
        var_dfn_su: f64,
        var_dfn_su_dn0: f64,
        var_dfn_su_dn2: f64,
        var_guard31: f64,
        var_guard384: f64,
        var_guard385: f64,
        var_guard403: f64,
        var_lg_i: f64,
        var_ls_i: f64,
        var_ndibot_i: f64,
        var_nfabot_i: f64,
        var_nfagat_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_swjunexp_i: f64,
        var_vmax: f64,
        var_dfn_sl_slot: &mut f64,
        var_dfn_sl_dn0_slot: &mut f64,
        var_dfn_sl_dn2_slot: &mut f64,
        var_dfn_sl_rv_slot: &mut f64,
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rv_slot: &mut f64,
        var_guard471_slot: &mut f64,
        var_guard471_rv_slot: &mut f64,
        var_guard479_slot: &mut f64,
        var_guard479_rv_slot: &mut f64,
        var_guard480_slot: &mut f64,
        var_guard480_rv_slot: &mut f64,
        var_guard483_slot: &mut f64,
        var_guard483_rv_slot: &mut f64,
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
        var_vak_slot: &mut f64,
        var_vak_dn0_slot: &mut f64,
        var_vak_dn2_slot: &mut f64,
        var_vak_rv_slot: &mut f64,
        var_vha1_slot: &mut f64,
        var_vha1_rv_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let mut var_dfn_sl: f64 = *var_dfn_sl_slot;
        let mut var_dfn_sl_dn0: f64 = *var_dfn_sl_dn0_slot;
        let mut var_dfn_sl_dn2: f64 = *var_dfn_sl_dn2_slot;
        let mut var_dfn_sl_rv: f64 = *var_dfn_sl_rv_slot;
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_rv: f64 = *var_dvmax_over_phitd_dv_rv_slot;
        let mut var_guard471: f64 = *var_guard471_slot;
        let mut var_guard471_rv: f64 = *var_guard471_rv_slot;
        let mut var_guard479: f64 = *var_guard479_slot;
        let mut var_guard479_rv: f64 = *var_guard479_rv_slot;
        let mut var_guard480: f64 = *var_guard480_slot;
        let mut var_guard480_rv: f64 = *var_guard480_rv_slot;
        let mut var_guard483: f64 = *var_guard483_slot;
        let mut var_guard483_rv: f64 = *var_guard483_rv_slot;
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
        let mut var_vak: f64 = *var_vak_slot;
        let mut var_vak_dn0: f64 = *var_vak_dn0_slot;
        let mut var_vak_dn2: f64 = *var_vak_dn2_slot;
        let mut var_vak_rv: f64 = *var_vak_rv_slot;
        let mut var_vha1: f64 = *var_vha1_slot;
        let mut var_vha1_rv: f64 = *var_vha1_rv_slot;

        let (assign25410_e37766, assign25410_e37766_d_n0, assign25410_e37766_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let (assign25410_e37764, assign25410_e37764_d_n0, assign25410_e37764_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign25410_e37763: f64 = (-var_tmf2);
                (assign25410_e37763, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign25410_e37764, assign25410_e37764_d_n0, assign25410_e37764_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25410_e37766;
        var_tmf2_dn0 = assign25410_e37766_d_n0;
        var_tmf2_dn2 = assign25410_e37766_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25420_e37782, assign25420_e37782_d_n0, assign25420_e37782_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25420_e37777: f64 = (var_tmf1 * var_tmf1);
        let assign25420_e37779: f64 = (assign25420_e37777 + var_tmf2);
        let assign25420_e37780: f64 = (assign25420_e37779).sqrt();
        (assign25420_e37780, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign25420_e37780)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign25420_e37780)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25420_e37782;
        var_tmf2_dn0 = assign25420_e37782_d_n0;
        var_tmf2_dn2 = assign25420_e37782_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25430_e37799, assign25430_e37799_d_n0, assign25430_e37799_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25430_e37795: f64 = (var_tmf1 / var_tmf2);
        let assign25430_e37796: f64 = (1.0 + assign25430_e37795);
        let assign25430_e37797: f64 = (0.5 * assign25430_e37796);
        (assign25430_e37797, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign25430_e37799;
        var_dfn_sl_dn0 = assign25430_e37799_d_n0;
        var_dfn_sl_dn2 = assign25430_e37799_d_n2;
        var_dfn_sl_rv = 0.0;

        let (assign25440_e37816, assign25440_e37816_d_n0, assign25440_e37816_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25440_e37812: f64 = (var_tmf1 + var_tmf2);
        let assign25440_e37813: f64 = (0.5 * assign25440_e37812);
        let assign25440_e37814: f64 = (var_nfagat_i + assign25440_e37813);
        (assign25440_e37814, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign25440_e37816;
        var_nj1_dn0 = assign25440_e37816_d_n0;
        var_nj1_dn2 = assign25440_e37816_d_n2;
        var_nj1_rv = 0.0;

        let (assign25450_e37831, assign25450_e37831_d_n0, assign25450_e37831_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25450_e37827: f64 = (p.p85 - var_nj0);
        let assign25450_e37829: f64 = (assign25450_e37827 - 0.01);
        (assign25450_e37829, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign25450_e37831;
        var_tmf1_dn0 = assign25450_e37831_d_n0;
        var_tmf1_dn2 = assign25450_e37831_d_n2;
        var_tmf1_rv = 0.0;

        let (assign25460_e37846, assign25460_e37846_d_n0, assign25460_e37846_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25460_e37842: f64 = (4.0 * p.p85);
        let assign25460_e37844: f64 = (assign25460_e37842 * 0.01);
        (assign25460_e37844, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25460_e37846;
        var_tmf2_dn0 = assign25460_e37846_d_n0;
        var_tmf2_dn2 = assign25460_e37846_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25470_e37863, assign25470_e37863_d_n0, assign25470_e37863_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let (assign25470_e37861, assign25470_e37861_d_n0, assign25470_e37861_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign25470_e37860: f64 = (-var_tmf2);
                (assign25470_e37860, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign25470_e37861, assign25470_e37861_d_n0, assign25470_e37861_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25470_e37863;
        var_tmf2_dn0 = assign25470_e37863_d_n0;
        var_tmf2_dn2 = assign25470_e37863_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25480_e37879, assign25480_e37879_d_n0, assign25480_e37879_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25480_e37874: f64 = (var_tmf1 * var_tmf1);
        let assign25480_e37876: f64 = (assign25480_e37874 + var_tmf2);
        let assign25480_e37877: f64 = (assign25480_e37876).sqrt();
        (assign25480_e37877, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign25480_e37877)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign25480_e37877)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25480_e37879;
        var_tmf2_dn0 = assign25480_e37879_d_n0;
        var_tmf2_dn2 = assign25480_e37879_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25490_e37896, assign25490_e37896_d_n0, assign25490_e37896_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25490_e37892: f64 = (var_tmf1 + var_tmf2);
        let assign25490_e37893: f64 = (0.5 * assign25490_e37892);
        let assign25490_e37894: f64 = (p.p85 - assign25490_e37893);
        (assign25490_e37894, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign25490_e37896;
        var_nj0_dn0 = assign25490_e37896_d_n0;
        var_nj0_dn2 = assign25490_e37896_d_n2;
        var_nj0_rv = 0.0;

        let (assign25500_e37911, assign25500_e37911_d_n0, assign25500_e37911_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25500_e37907: f64 = (var_nj0 - var_nfagat_i);
        let assign25500_e37909: f64 = (assign25500_e37907 - 0.01);
        (assign25500_e37909, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign25500_e37911;
        var_tmf1_dn0 = assign25500_e37911_d_n0;
        var_tmf1_dn2 = assign25500_e37911_d_n2;
        var_tmf1_rv = 0.0;

        let (assign25510_e37926, assign25510_e37926_d_n0, assign25510_e37926_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25510_e37922: f64 = (4.0 * var_nfagat_i);
        let assign25510_e37924: f64 = (assign25510_e37922 * 0.01);
        (assign25510_e37924, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25510_e37926;
        var_tmf2_dn0 = assign25510_e37926_d_n0;
        var_tmf2_dn2 = assign25510_e37926_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25520_e37943, assign25520_e37943_d_n0, assign25520_e37943_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let (assign25520_e37941, assign25520_e37941_d_n0, assign25520_e37941_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign25520_e37940: f64 = (-var_tmf2);
                (assign25520_e37940, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign25520_e37941, assign25520_e37941_d_n0, assign25520_e37941_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25520_e37943;
        var_tmf2_dn0 = assign25520_e37943_d_n0;
        var_tmf2_dn2 = assign25520_e37943_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25530_e37959, assign25530_e37959_d_n0, assign25530_e37959_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25530_e37954: f64 = (var_tmf1 * var_tmf1);
        let assign25530_e37956: f64 = (assign25530_e37954 + var_tmf2);
        let assign25530_e37957: f64 = (assign25530_e37956).sqrt();
        (assign25530_e37957, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign25530_e37957)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign25530_e37957)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign25530_e37959;
        var_tmf2_dn0 = assign25530_e37959_d_n0;
        var_tmf2_dn2 = assign25530_e37959_d_n2;
        var_tmf2_rv = 0.0;

        let (assign25540_e37976, assign25540_e37976_d_n0, assign25540_e37976_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25540_e37972: f64 = (var_tmf1 + var_tmf2);
        let assign25540_e37973: f64 = (0.5 * assign25540_e37972);
        let assign25540_e37974: f64 = (var_nfagat_i + assign25540_e37973);
        (assign25540_e37974, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign25540_e37976;
        var_nj0_dn0 = assign25540_e37976_d_n0;
        var_nj0_dn2 = assign25540_e37976_d_n2;
        var_nj0_rv = 0.0;

        let (assign25550_e37991, assign25550_e37991_d_n0, assign25550_e37991_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 != 0.0)) {
        let assign25550_e37987: f64 = (p.p86 * var_dfn_su);
        let assign25550_e37989: f64 = (assign25550_e37987 * var_dfn_sl);
        (assign25550_e37989, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign25550_e37987 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign25550_e37987 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign25550_e37991;
        var_dnj1_dv_dn0 = assign25550_e37991_d_n0;
        var_dnj1_dv_dn2 = assign25550_e37991_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign25560_e38003, assign25560_e38003_d_n0, assign25560_e38003_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign25560_e38003;
        var_nj0_dn0 = assign25560_e38003_d_n0;
        var_nj0_dn2 = assign25560_e38003_d_n2;
        var_nj0_rv = 0.0;

        let (assign25570_e38015, assign25570_e38015_d_n0, assign25570_e38015_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign25570_e38015;
        var_nj1_dn0 = assign25570_e38015_d_n0;
        var_nj1_dn2 = assign25570_e38015_d_n2;
        var_nj1_rv = 0.0;

        let (assign25580_e38027, assign25580_e38027_d_n0, assign25580_e38027_d_n2,) = {
    if ((((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) && (var_guard403 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign25580_e38027;
        var_dnj1_dv_dn0 = assign25580_e38027_d_n0;
        var_dnj1_dv_dn2 = assign25580_e38027_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign25640_e38276, assign25640_e38276_d_n0, assign25640_e38276_d_n2,) = {
    if (((var_guard31 != 0.0) && (var_guard384 != 0.0)) && (var_guard385 == 0.0)) {
        let assign25640_e38260: f64 = (var_vmax * var_dnj1_dv);
        let assign25640_e38261: f64 = (var_nj1 - assign25640_e38260);
        let assign25640_e38264: f64 = (var_nj1 * var_nj1);
        let assign25640_e38265: f64 = (assign25640_e38261 / assign25640_e38264);
        let assign25640_e38268: f64 = (var_vha1 * var_dnj1_dv);
        let assign25640_e38271: f64 = (var_nj0 * p.p85);
        let assign25640_e38272: f64 = (assign25640_e38268 / assign25640_e38271);
        let assign25640_e38273: f64 = (assign25640_e38265 + assign25640_e38272);
        let assign25640_e38274: f64 = (var_phitdinv * assign25640_e38273);
        (assign25640_e38274, (var_phitdinv * (((((var_nj1_dn0 - (var_vmax * var_dnj1_dv_dn0)) * assign25640_e38264) - (assign25640_e38261 * ((var_nj1_dn0 * var_nj1) + (var_nj1 * var_nj1_dn0)))) / (assign25640_e38264 * assign25640_e38264)) + ((((var_vha1 * var_dnj1_dv_dn0) * assign25640_e38271) - (assign25640_e38268 * (var_nj0_dn0 * p.p85))) / (assign25640_e38271 * assign25640_e38271)))), (var_phitdinv * (((((var_nj1_dn2 - (var_vmax * var_dnj1_dv_dn2)) * assign25640_e38264) - (assign25640_e38261 * ((var_nj1_dn2 * var_nj1) + (var_nj1 * var_nj1_dn2)))) / (assign25640_e38264 * assign25640_e38264)) + ((((var_vha1 * var_dnj1_dv_dn2) * assign25640_e38271) - (assign25640_e38268 * (var_nj0_dn2 * p.p85))) / (assign25640_e38271 * assign25640_e38271)))),)
    } else {
        (var_dvmax_over_phitd_dv, var_dvmax_over_phitd_dv_dn0, var_dvmax_over_phitd_dv_dn2,)
    }
};
        var_dvmax_over_phitd_dv = assign25640_e38276;
        var_dvmax_over_phitd_dv_dn0 = assign25640_e38276_d_n0;
        var_dvmax_over_phitd_dv_dn2 = assign25640_e38276_d_n2;
        var_dvmax_over_phitd_dv_rv = 0.0;

        let (assign25660_e38301, assign25660_e38301_d_n0, assign25660_e38301_d_n2,) = {
    if ((var_guard31 != 0.0) && (var_guard384 != 0.0)) {
        let assign25660_e38299: f64 = (var_idmultbot - 1.0);
        (assign25660_e38299, var_idmultbot_dn0, var_idmultbot_dn2,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign25660_e38301;
        var_idmultbot_dn0 = assign25660_e38301_d_n0;
        var_idmultbot_dn2 = assign25660_e38301_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign25770_e38474, assign25770_e38474_d_n0, assign25770_e38474_d_n2,) = {
    if ((var_guard31 != 0.0) && (var_guard384 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign25770_e38474;
        var_idmultbot_dn0 = assign25770_e38474_d_n0;
        var_idmultbot_dn2 = assign25770_e38474_d_n2;
        var_idmultbot_rv = 0.0;

        var_vak = (nv0 - nv2);
        var_vak_dn0 = 1.0;
        var_vak_dn2 = -1.0;
        var_vak_rv = 0.0;

        let assign28760_e42618: f64 = if var_swjunexp_i == 1.0 { 1.0 } else { 0.0 };
        var_guard471 = assign28760_e42618;
        var_guard471_rv = 0.0;

        let assign29220_e43059: f64 = if (!(((var_ab_i == 0.0) && (var_ls_i == 0.0)) && (var_lg_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard479 = assign29220_e43059;
        var_guard479_rv = 0.0;

        let assign29300_e43138: f64 = if var_vak < var_vmax { 1.0 } else { 0.0 };
        var_guard480 = assign29300_e43138;
        var_guard480_rv = 0.0;

        let (assign29360_e43283,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) {
        let assign29360_e43279: f64 = (var_nin * var_nin);
        let assign29360_e43281: f64 = (assign29360_e43279 / var_ndibot_i);
        (assign29360_e43281,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign29360_e43283;
        var_pnn0_rv = 0.0;

        let (assign29370_e43299,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) {
        let assign29370_e43292: f64 = (var_nfabot_i / var_phitdinv);
        let assign29370_e43295: f64 = (var_ndibot_i / var_pnn0);
        let assign29370_e43296: f64 = (assign29370_e43295).ln();
        let assign29370_e43297: f64 = (assign29370_e43292 * assign29370_e43296);
        (assign29370_e43297,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign29370_e43299;
        var_vha1_rv = 0.0;

        let assign29380_e43302: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard483 = assign29380_e43302;
        var_guard483_rv = 0.0;

        let (assign29390_e43319, assign29390_e43319_d_n0, assign29390_e43319_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29390_e43314: f64 = (var_vak - var_vha1);
        let assign29390_e43315: f64 = (p.p86 * assign29390_e43314);
        let assign29390_e43317: f64 = (assign29390_e43315 + var_nfabot_i);
        (assign29390_e43317, (p.p86 * var_vak_dn0), (p.p86 * var_vak_dn2),)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign29390_e43319;
        var_nja10_dn0 = assign29390_e43319_d_n0;
        var_nja10_dn2 = assign29390_e43319_d_n2;
        var_nja10_rv = 0.0;

        let (assign29400_e43334, assign29400_e43334_d_n0, assign29400_e43334_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29400_e43331: f64 = (p.p86 * var_vha1);
        let assign29400_e43332: f64 = (var_nfabot_i - assign29400_e43331);
        (assign29400_e43332, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign29400_e43334;
        var_nj0_dn0 = assign29400_e43334_d_n0;
        var_nj0_dn2 = assign29400_e43334_d_n2;
        var_nj0_rv = 0.0;

        let (assign29410_e43349, assign29410_e43349_d_n0, assign29410_e43349_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29410_e43345: f64 = (p.p85 - var_nja10);
        let assign29410_e43347: f64 = (assign29410_e43345 - 0.01);
        (assign29410_e43347, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign29410_e43349;
        var_tmf1_dn0 = assign29410_e43349_d_n0;
        var_tmf1_dn2 = assign29410_e43349_d_n2;
        var_tmf1_rv = 0.0;

        let (assign29420_e43364, assign29420_e43364_d_n0, assign29420_e43364_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29420_e43360: f64 = (4.0 * p.p85);
        let assign29420_e43362: f64 = (assign29420_e43360 * 0.01);
        (assign29420_e43362, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29420_e43364;
        var_tmf2_dn0 = assign29420_e43364_d_n0;
        var_tmf2_dn2 = assign29420_e43364_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29430_e43381, assign29430_e43381_d_n0, assign29430_e43381_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let (assign29430_e43379, assign29430_e43379_d_n0, assign29430_e43379_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign29430_e43378: f64 = (-var_tmf2);
                (assign29430_e43378, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign29430_e43379, assign29430_e43379_d_n0, assign29430_e43379_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29430_e43381;
        var_tmf2_dn0 = assign29430_e43381_d_n0;
        var_tmf2_dn2 = assign29430_e43381_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29440_e43397, assign29440_e43397_d_n0, assign29440_e43397_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29440_e43392: f64 = (var_tmf1 * var_tmf1);
        let assign29440_e43394: f64 = (assign29440_e43392 + var_tmf2);
        let assign29440_e43395: f64 = (assign29440_e43394).sqrt();
        (assign29440_e43395, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign29440_e43395)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign29440_e43395)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29440_e43397;
        var_tmf2_dn0 = assign29440_e43397_d_n0;
        var_tmf2_dn2 = assign29440_e43397_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29450_e43414, assign29450_e43414_d_n0, assign29450_e43414_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29450_e43410: f64 = (var_tmf1 + var_tmf2);
        let assign29450_e43411: f64 = (0.5 * assign29450_e43410);
        let assign29450_e43412: f64 = (p.p85 - assign29450_e43411);
        (assign29450_e43412, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign29450_e43414;
        var_nja11_dn0 = assign29450_e43414_d_n0;
        var_nja11_dn2 = assign29450_e43414_d_n2;
        var_nja11_rv = 0.0;

        let (assign29460_e43429, assign29460_e43429_d_n0, assign29460_e43429_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29460_e43425: f64 = (var_nja11 - var_nfabot_i);
        let assign29460_e43427: f64 = (assign29460_e43425 - 0.01);
        (assign29460_e43427, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign29460_e43429;
        var_tmf1_dn0 = assign29460_e43429_d_n0;
        var_tmf1_dn2 = assign29460_e43429_d_n2;
        var_tmf1_rv = 0.0;

        let (assign29470_e43444, assign29470_e43444_d_n0, assign29470_e43444_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29470_e43440: f64 = (4.0 * var_nfabot_i);
        let assign29470_e43442: f64 = (assign29470_e43440 * 0.01);
        (assign29470_e43442, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29470_e43444;
        var_tmf2_dn0 = assign29470_e43444_d_n0;
        var_tmf2_dn2 = assign29470_e43444_d_n2;
        var_tmf2_rv = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_rv_slot = var_dvmax_over_phitd_dv_rv;
        *var_guard471_slot = var_guard471;
        *var_guard471_rv_slot = var_guard471_rv;
        *var_guard479_slot = var_guard479;
        *var_guard479_rv_slot = var_guard479_rv;
        *var_guard480_slot = var_guard480;
        *var_guard480_rv_slot = var_guard480_rv;
        *var_guard483_slot = var_guard483;
        *var_guard483_rv_slot = var_guard483_rv;
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
        *var_vak_slot = var_vak;
        *var_vak_dn0_slot = var_vak_dn0;
        *var_vak_dn2_slot = var_vak_dn2;
        *var_vak_rv_slot = var_vak_rv;
        *var_vha1_slot = var_vha1;
        *var_vha1_rv_slot = var_vha1_rv;
    }

    pub(super) fn stamp_reactive_block_31(
        p: &Parameters,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard483: f64,
        var_ndisti_i: f64,
        var_nfabot_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_vak: f64,
        var_vak_dn0: f64,
        var_vak_dn2: f64,
        var_guard484_slot: &mut f64,
        var_guard484_rv_slot: &mut f64,
        var_guard485_slot: &mut f64,
        var_guard485_rv_slot: &mut f64,
        var_guard486_slot: &mut f64,
        var_guard486_rv_slot: &mut f64,
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
        let mut var_guard484: f64 = *var_guard484_slot;
        let mut var_guard484_rv: f64 = *var_guard484_rv_slot;
        let mut var_guard485: f64 = *var_guard485_slot;
        let mut var_guard485_rv: f64 = *var_guard485_rv_slot;
        let mut var_guard486: f64 = *var_guard486_slot;
        let mut var_guard486_rv: f64 = *var_guard486_rv_slot;
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

        let (assign29480_e43461, assign29480_e43461_d_n0, assign29480_e43461_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let (assign29480_e43459, assign29480_e43459_d_n0, assign29480_e43459_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign29480_e43458: f64 = (-var_tmf2);
                (assign29480_e43458, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign29480_e43459, assign29480_e43459_d_n0, assign29480_e43459_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29480_e43461;
        var_tmf2_dn0 = assign29480_e43461_d_n0;
        var_tmf2_dn2 = assign29480_e43461_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29490_e43477, assign29490_e43477_d_n0, assign29490_e43477_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29490_e43472: f64 = (var_tmf1 * var_tmf1);
        let assign29490_e43474: f64 = (assign29490_e43472 + var_tmf2);
        let assign29490_e43475: f64 = (assign29490_e43474).sqrt();
        (assign29490_e43475, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign29490_e43475)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign29490_e43475)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29490_e43477;
        var_tmf2_dn0 = assign29490_e43477_d_n0;
        var_tmf2_dn2 = assign29490_e43477_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29500_e43494, assign29500_e43494_d_n0, assign29500_e43494_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29500_e43490: f64 = (var_tmf1 + var_tmf2);
        let assign29500_e43491: f64 = (0.5 * assign29500_e43490);
        let assign29500_e43492: f64 = (var_nfabot_i + assign29500_e43491);
        (assign29500_e43492, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign29500_e43494;
        var_nj1_dn0 = assign29500_e43494_d_n0;
        var_nj1_dn2 = assign29500_e43494_d_n2;
        var_nj1_rv = 0.0;

        let (assign29510_e43509, assign29510_e43509_d_n0, assign29510_e43509_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29510_e43505: f64 = (p.p85 - var_nj0);
        let assign29510_e43507: f64 = (assign29510_e43505 - 0.01);
        (assign29510_e43507, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign29510_e43509;
        var_tmf1_dn0 = assign29510_e43509_d_n0;
        var_tmf1_dn2 = assign29510_e43509_d_n2;
        var_tmf1_rv = 0.0;

        let (assign29520_e43524, assign29520_e43524_d_n0, assign29520_e43524_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29520_e43520: f64 = (4.0 * p.p85);
        let assign29520_e43522: f64 = (assign29520_e43520 * 0.01);
        (assign29520_e43522, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29520_e43524;
        var_tmf2_dn0 = assign29520_e43524_d_n0;
        var_tmf2_dn2 = assign29520_e43524_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29530_e43541, assign29530_e43541_d_n0, assign29530_e43541_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let (assign29530_e43539, assign29530_e43539_d_n0, assign29530_e43539_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign29530_e43538: f64 = (-var_tmf2);
                (assign29530_e43538, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign29530_e43539, assign29530_e43539_d_n0, assign29530_e43539_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29530_e43541;
        var_tmf2_dn0 = assign29530_e43541_d_n0;
        var_tmf2_dn2 = assign29530_e43541_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29540_e43557, assign29540_e43557_d_n0, assign29540_e43557_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29540_e43552: f64 = (var_tmf1 * var_tmf1);
        let assign29540_e43554: f64 = (assign29540_e43552 + var_tmf2);
        let assign29540_e43555: f64 = (assign29540_e43554).sqrt();
        (assign29540_e43555, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign29540_e43555)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign29540_e43555)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29540_e43557;
        var_tmf2_dn0 = assign29540_e43557_d_n0;
        var_tmf2_dn2 = assign29540_e43557_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29550_e43574, assign29550_e43574_d_n0, assign29550_e43574_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29550_e43570: f64 = (var_tmf1 + var_tmf2);
        let assign29550_e43571: f64 = (0.5 * assign29550_e43570);
        let assign29550_e43572: f64 = (p.p85 - assign29550_e43571);
        (assign29550_e43572, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign29550_e43574;
        var_nj0_dn0 = assign29550_e43574_d_n0;
        var_nj0_dn2 = assign29550_e43574_d_n2;
        var_nj0_rv = 0.0;

        let (assign29560_e43589, assign29560_e43589_d_n0, assign29560_e43589_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29560_e43585: f64 = (var_nj0 - var_nfabot_i);
        let assign29560_e43587: f64 = (assign29560_e43585 - 0.01);
        (assign29560_e43587, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign29560_e43589;
        var_tmf1_dn0 = assign29560_e43589_d_n0;
        var_tmf1_dn2 = assign29560_e43589_d_n2;
        var_tmf1_rv = 0.0;

        let (assign29570_e43604, assign29570_e43604_d_n0, assign29570_e43604_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29570_e43600: f64 = (4.0 * var_nfabot_i);
        let assign29570_e43602: f64 = (assign29570_e43600 * 0.01);
        (assign29570_e43602, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29570_e43604;
        var_tmf2_dn0 = assign29570_e43604_d_n0;
        var_tmf2_dn2 = assign29570_e43604_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29580_e43621, assign29580_e43621_d_n0, assign29580_e43621_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let (assign29580_e43619, assign29580_e43619_d_n0, assign29580_e43619_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign29580_e43618: f64 = (-var_tmf2);
                (assign29580_e43618, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign29580_e43619, assign29580_e43619_d_n0, assign29580_e43619_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29580_e43621;
        var_tmf2_dn0 = assign29580_e43621_d_n0;
        var_tmf2_dn2 = assign29580_e43621_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29590_e43637, assign29590_e43637_d_n0, assign29590_e43637_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29590_e43632: f64 = (var_tmf1 * var_tmf1);
        let assign29590_e43634: f64 = (assign29590_e43632 + var_tmf2);
        let assign29590_e43635: f64 = (assign29590_e43634).sqrt();
        (assign29590_e43635, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign29590_e43635)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign29590_e43635)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29590_e43637;
        var_tmf2_dn0 = assign29590_e43637_d_n0;
        var_tmf2_dn2 = assign29590_e43637_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29600_e43654, assign29600_e43654_d_n0, assign29600_e43654_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 != 0.0)) {
        let assign29600_e43650: f64 = (var_tmf1 + var_tmf2);
        let assign29600_e43651: f64 = (0.5 * assign29600_e43650);
        let assign29600_e43652: f64 = (var_nfabot_i + assign29600_e43651);
        (assign29600_e43652, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign29600_e43654;
        var_nj0_dn0 = assign29600_e43654_d_n0;
        var_nj0_dn2 = assign29600_e43654_d_n2;
        var_nj0_rv = 0.0;

        let (assign29610_e43666, assign29610_e43666_d_n0, assign29610_e43666_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign29610_e43666;
        var_nj0_dn0 = assign29610_e43666_d_n0;
        var_nj0_dn2 = assign29610_e43666_d_n2;
        var_nj0_rv = 0.0;

        let (assign29620_e43678, assign29620_e43678_d_n0, assign29620_e43678_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard483 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign29620_e43678;
        var_nj1_dn0 = assign29620_e43678_d_n0;
        var_nj1_dn2 = assign29620_e43678_d_n2;
        var_nj1_rv = 0.0;

        let assign29630_e43682: f64 = (var_vak / var_nj1);
        let assign29630_e43686: f64 = (var_nj1 - var_nj0);
        let assign29630_e43687: f64 = (var_vha1 * assign29630_e43686);
        let assign29630_e43690: f64 = (var_nj0 * p.p85);
        let assign29630_e43691: f64 = (assign29630_e43687 / assign29630_e43690);
        let assign29630_e43692: f64 = (assign29630_e43682 + assign29630_e43691);
        let assign29630_e43693: f64 = (var_phitdinv * assign29630_e43692);
        let assign29630_e43694: f64 = (assign29630_e43693).abs();
        let assign29630_e43696: f64 = if assign29630_e43694 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard484 = assign29630_e43696;
        var_guard484_rv = 0.0;

        let (assign29640_e43722, assign29640_e43722_d_n0, assign29640_e43722_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard484 != 0.0)) {
        let assign29640_e43708: f64 = (var_vak / var_nj1);
        let assign29640_e43712: f64 = (var_nj1 - var_nj0);
        let assign29640_e43713: f64 = (var_vha1 * assign29640_e43712);
        let assign29640_e43716: f64 = (var_nj0 * p.p85);
        let assign29640_e43717: f64 = (assign29640_e43713 / assign29640_e43716);
        let assign29640_e43718: f64 = (assign29640_e43708 + assign29640_e43717);
        let assign29640_e43719: f64 = (var_phitdinv * assign29640_e43718);
        let assign29640_e43720: f64 = (assign29640_e43719).exp();
        (assign29640_e43720, (assign29640_e43720 * (var_phitdinv * ((((var_vak_dn0 * var_nj1) - (var_vak * var_nj1_dn0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign29640_e43716) - (assign29640_e43713 * (var_nj0_dn0 * p.p85))) / (assign29640_e43716 * assign29640_e43716))))), (assign29640_e43720 * (var_phitdinv * ((((var_vak_dn2 * var_nj1) - (var_vak * var_nj1_dn2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign29640_e43716) - (assign29640_e43713 * (var_nj0_dn2 * p.p85))) / (assign29640_e43716 * assign29640_e43716))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign29640_e43722;
        var_idmultbot_dn0 = assign29640_e43722_d_n0;
        var_idmultbot_dn2 = assign29640_e43722_d_n2;
        var_idmultbot_rv = 0.0;

        let assign29650_e43726: f64 = (var_vak / var_nj1);
        let assign29650_e43730: f64 = (var_nj1 - var_nj0);
        let assign29650_e43731: f64 = (var_vha1 * assign29650_e43730);
        let assign29650_e43734: f64 = (var_nj0 * p.p85);
        let assign29650_e43735: f64 = (assign29650_e43731 / assign29650_e43734);
        let assign29650_e43736: f64 = (assign29650_e43726 + assign29650_e43735);
        let assign29650_e43737: f64 = (var_phitdinv * assign29650_e43736);
        let assign29650_e43739: f64 = (-230.25850929940458);
        let assign29650_e43740: f64 = if assign29650_e43737 < assign29650_e43739 { 1.0 } else { 0.0 };
        var_guard485 = assign29650_e43740;
        var_guard485_rv = 0.0;

        let (assign29660_e43821, assign29660_e43821_d_n0, assign29660_e43821_d_n2,) = {
    if (((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard484 == 0.0)) && (var_guard485 != 0.0)) {
        let assign29660_e43755: f64 = (-230.25850929940458);
        let assign29660_e43759: f64 = (var_vak / var_nj1);
        let assign29660_e43763: f64 = (var_nj1 - var_nj0);
        let assign29660_e43764: f64 = (var_vha1 * assign29660_e43763);
        let assign29660_e43767: f64 = (var_nj0 * p.p85);
        let assign29660_e43768: f64 = (assign29660_e43764 / assign29660_e43767);
        let assign29660_e43769: f64 = (assign29660_e43759 + assign29660_e43768);
        let assign29660_e43770: f64 = (var_phitdinv * assign29660_e43769);
        let assign29660_e43771: f64 = (assign29660_e43755 - assign29660_e43770);
        let assign29660_e43775: f64 = (-230.25850929940458);
        let assign29660_e43779: f64 = (var_vak / var_nj1);
        let assign29660_e43783: f64 = (var_nj1 - var_nj0);
        let assign29660_e43784: f64 = (var_vha1 * assign29660_e43783);
        let assign29660_e43787: f64 = (var_nj0 * p.p85);
        let assign29660_e43788: f64 = (assign29660_e43784 / assign29660_e43787);
        let assign29660_e43789: f64 = (assign29660_e43779 + assign29660_e43788);
        let assign29660_e43790: f64 = (var_phitdinv * assign29660_e43789);
        let assign29660_e43791: f64 = (assign29660_e43775 - assign29660_e43790);
        let assign29660_e43794: f64 = (-230.25850929940458);
        let assign29660_e43798: f64 = (var_vak / var_nj1);
        let assign29660_e43802: f64 = (var_nj1 - var_nj0);
        let assign29660_e43803: f64 = (var_vha1 * assign29660_e43802);
        let assign29660_e43806: f64 = (var_nj0 * p.p85);
        let assign29660_e43807: f64 = (assign29660_e43803 / assign29660_e43806);
        let assign29660_e43808: f64 = (assign29660_e43798 + assign29660_e43807);
        let assign29660_e43809: f64 = (var_phitdinv * assign29660_e43808);
        let assign29660_e43810: f64 = (assign29660_e43794 - assign29660_e43809);
        let assign29660_e43812: f64 = (assign29660_e43810 * 0.3333333333333333);
        let assign29660_e43813: f64 = (1.0 + assign29660_e43812);
        let assign29660_e43814: f64 = (assign29660_e43791 * assign29660_e43813);
        let assign29660_e43815: f64 = (0.5 * assign29660_e43814);
        let assign29660_e43816: f64 = (1.0 + assign29660_e43815);
        let assign29660_e43817: f64 = (assign29660_e43771 * assign29660_e43816);
        let assign29660_e43818: f64 = (1.0 + assign29660_e43817);
        let assign29660_e43819: f64 = (1e-100 / assign29660_e43818);
        (assign29660_e43819, (-((1e-100 * (((-(var_phitdinv * ((((var_vak_dn0 * var_nj1) - (var_vak * var_nj1_dn0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign29660_e43767) - (assign29660_e43764 * (var_nj0_dn0 * p.p85))) / (assign29660_e43767 * assign29660_e43767))))) * assign29660_e43816) + (assign29660_e43771 * (0.5 * (((-(var_phitdinv * ((((var_vak_dn0 * var_nj1) - (var_vak * var_nj1_dn0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign29660_e43787) - (assign29660_e43784 * (var_nj0_dn0 * p.p85))) / (assign29660_e43787 * assign29660_e43787))))) * assign29660_e43813) + (assign29660_e43791 * ((-(var_phitdinv * ((((var_vak_dn0 * var_nj1) - (var_vak * var_nj1_dn0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign29660_e43806) - (assign29660_e43803 * (var_nj0_dn0 * p.p85))) / (assign29660_e43806 * assign29660_e43806))))) * 0.3333333333333333))))))) / (assign29660_e43818 * assign29660_e43818))), (-((1e-100 * (((-(var_phitdinv * ((((var_vak_dn2 * var_nj1) - (var_vak * var_nj1_dn2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign29660_e43767) - (assign29660_e43764 * (var_nj0_dn2 * p.p85))) / (assign29660_e43767 * assign29660_e43767))))) * assign29660_e43816) + (assign29660_e43771 * (0.5 * (((-(var_phitdinv * ((((var_vak_dn2 * var_nj1) - (var_vak * var_nj1_dn2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign29660_e43787) - (assign29660_e43784 * (var_nj0_dn2 * p.p85))) / (assign29660_e43787 * assign29660_e43787))))) * assign29660_e43813) + (assign29660_e43791 * ((-(var_phitdinv * ((((var_vak_dn2 * var_nj1) - (var_vak * var_nj1_dn2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign29660_e43806) - (assign29660_e43803 * (var_nj0_dn2 * p.p85))) / (assign29660_e43806 * assign29660_e43806))))) * 0.3333333333333333))))))) / (assign29660_e43818 * assign29660_e43818))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign29660_e43821;
        var_idmultbot_dn0 = assign29660_e43821_d_n0;
        var_idmultbot_dn2 = assign29660_e43821_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign29670_e43900, assign29670_e43900_d_n0, assign29670_e43900_d_n2,) = {
    if (((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard484 == 0.0)) && (var_guard485 == 0.0)) {
        let assign29670_e43839: f64 = (var_vak / var_nj1);
        let assign29670_e43843: f64 = (var_nj1 - var_nj0);
        let assign29670_e43844: f64 = (var_vha1 * assign29670_e43843);
        let assign29670_e43847: f64 = (var_nj0 * p.p85);
        let assign29670_e43848: f64 = (assign29670_e43844 / assign29670_e43847);
        let assign29670_e43849: f64 = (assign29670_e43839 + assign29670_e43848);
        let assign29670_e43850: f64 = (var_phitdinv * assign29670_e43849);
        let assign29670_e43852: f64 = (assign29670_e43850 - 230.25850929940458);
        let assign29670_e43858: f64 = (var_vak / var_nj1);
        let assign29670_e43862: f64 = (var_nj1 - var_nj0);
        let assign29670_e43863: f64 = (var_vha1 * assign29670_e43862);
        let assign29670_e43866: f64 = (var_nj0 * p.p85);
        let assign29670_e43867: f64 = (assign29670_e43863 / assign29670_e43866);
        let assign29670_e43868: f64 = (assign29670_e43858 + assign29670_e43867);
        let assign29670_e43869: f64 = (var_phitdinv * assign29670_e43868);
        let assign29670_e43871: f64 = (assign29670_e43869 - 230.25850929940458);
        let assign29670_e43876: f64 = (var_vak / var_nj1);
        let assign29670_e43880: f64 = (var_nj1 - var_nj0);
        let assign29670_e43881: f64 = (var_vha1 * assign29670_e43880);
        let assign29670_e43884: f64 = (var_nj0 * p.p85);
        let assign29670_e43885: f64 = (assign29670_e43881 / assign29670_e43884);
        let assign29670_e43886: f64 = (assign29670_e43876 + assign29670_e43885);
        let assign29670_e43887: f64 = (var_phitdinv * assign29670_e43886);
        let assign29670_e43889: f64 = (assign29670_e43887 - 230.25850929940458);
        let assign29670_e43891: f64 = (assign29670_e43889 * 0.3333333333333333);
        let assign29670_e43892: f64 = (1.0 + assign29670_e43891);
        let assign29670_e43893: f64 = (assign29670_e43871 * assign29670_e43892);
        let assign29670_e43894: f64 = (0.5 * assign29670_e43893);
        let assign29670_e43895: f64 = (1.0 + assign29670_e43894);
        let assign29670_e43896: f64 = (assign29670_e43852 * assign29670_e43895);
        let assign29670_e43897: f64 = (1.0 + assign29670_e43896);
        let assign29670_e43898: f64 = (1e100 * assign29670_e43897);
        (assign29670_e43898, (1e100 * (((var_phitdinv * ((((var_vak_dn0 * var_nj1) - (var_vak * var_nj1_dn0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign29670_e43847) - (assign29670_e43844 * (var_nj0_dn0 * p.p85))) / (assign29670_e43847 * assign29670_e43847)))) * assign29670_e43895) + (assign29670_e43852 * (0.5 * (((var_phitdinv * ((((var_vak_dn0 * var_nj1) - (var_vak * var_nj1_dn0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign29670_e43866) - (assign29670_e43863 * (var_nj0_dn0 * p.p85))) / (assign29670_e43866 * assign29670_e43866)))) * assign29670_e43892) + (assign29670_e43871 * ((var_phitdinv * ((((var_vak_dn0 * var_nj1) - (var_vak * var_nj1_dn0)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign29670_e43884) - (assign29670_e43881 * (var_nj0_dn0 * p.p85))) / (assign29670_e43884 * assign29670_e43884)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((((var_vak_dn2 * var_nj1) - (var_vak * var_nj1_dn2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign29670_e43847) - (assign29670_e43844 * (var_nj0_dn2 * p.p85))) / (assign29670_e43847 * assign29670_e43847)))) * assign29670_e43895) + (assign29670_e43852 * (0.5 * (((var_phitdinv * ((((var_vak_dn2 * var_nj1) - (var_vak * var_nj1_dn2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign29670_e43866) - (assign29670_e43863 * (var_nj0_dn2 * p.p85))) / (assign29670_e43866 * assign29670_e43866)))) * assign29670_e43892) + (assign29670_e43871 * ((var_phitdinv * ((((var_vak_dn2 * var_nj1) - (var_vak * var_nj1_dn2)) / (var_nj1 * var_nj1)) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign29670_e43884) - (assign29670_e43881 * (var_nj0_dn2 * p.p85))) / (assign29670_e43884 * assign29670_e43884)))) * 0.3333333333333333))))))),)
    } else {
        (var_idmultbot, var_idmultbot_dn0, var_idmultbot_dn2,)
    }
};
        var_idmultbot = assign29670_e43900;
        var_idmultbot_dn0 = assign29670_e43900_d_n0;
        var_idmultbot_dn2 = assign29670_e43900_d_n2;
        var_idmultbot_rv = 0.0;

        let (assign29680_e43913,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) {
        let assign29680_e43909: f64 = (var_nin * var_nin);
        let assign29680_e43911: f64 = (assign29680_e43909 / var_ndisti_i);
        (assign29680_e43911,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign29680_e43913;
        var_pnn0_rv = 0.0;

        let (assign29690_e43929,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) {
        let assign29690_e43922: f64 = (var_nfasti_i / var_phitdinv);
        let assign29690_e43925: f64 = (var_ndisti_i / var_pnn0);
        let assign29690_e43926: f64 = (assign29690_e43925).ln();
        let assign29690_e43927: f64 = (assign29690_e43922 * assign29690_e43926);
        (assign29690_e43927,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign29690_e43929;
        var_vha1_rv = 0.0;

        let assign29700_e43932: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard486 = assign29700_e43932;
        var_guard486_rv = 0.0;

        let (assign29710_e43949, assign29710_e43949_d_n0, assign29710_e43949_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29710_e43944: f64 = (var_vak - var_vha1);
        let assign29710_e43945: f64 = (p.p86 * assign29710_e43944);
        let assign29710_e43947: f64 = (assign29710_e43945 + var_nfasti_i);
        (assign29710_e43947, (p.p86 * var_vak_dn0), (p.p86 * var_vak_dn2),)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign29710_e43949;
        var_nja10_dn0 = assign29710_e43949_d_n0;
        var_nja10_dn2 = assign29710_e43949_d_n2;
        var_nja10_rv = 0.0;

        let (assign29720_e43964, assign29720_e43964_d_n0, assign29720_e43964_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29720_e43961: f64 = (p.p86 * var_vha1);
        let assign29720_e43962: f64 = (var_nfasti_i - assign29720_e43961);
        (assign29720_e43962, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign29720_e43964;
        var_nj0_dn0 = assign29720_e43964_d_n0;
        var_nj0_dn2 = assign29720_e43964_d_n2;
        var_nj0_rv = 0.0;

        let (assign29730_e43979, assign29730_e43979_d_n0, assign29730_e43979_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29730_e43975: f64 = (p.p85 - var_nja10);
        let assign29730_e43977: f64 = (assign29730_e43975 - 0.01);
        (assign29730_e43977, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign29730_e43979;
        var_tmf1_dn0 = assign29730_e43979_d_n0;
        var_tmf1_dn2 = assign29730_e43979_d_n2;
        var_tmf1_rv = 0.0;

        let (assign29740_e43994, assign29740_e43994_d_n0, assign29740_e43994_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29740_e43990: f64 = (4.0 * p.p85);
        let assign29740_e43992: f64 = (assign29740_e43990 * 0.01);
        (assign29740_e43992, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29740_e43994;
        var_tmf2_dn0 = assign29740_e43994_d_n0;
        var_tmf2_dn2 = assign29740_e43994_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29750_e44011, assign29750_e44011_d_n0, assign29750_e44011_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let (assign29750_e44009, assign29750_e44009_d_n0, assign29750_e44009_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign29750_e44008: f64 = (-var_tmf2);
                (assign29750_e44008, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign29750_e44009, assign29750_e44009_d_n0, assign29750_e44009_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29750_e44011;
        var_tmf2_dn0 = assign29750_e44011_d_n0;
        var_tmf2_dn2 = assign29750_e44011_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29760_e44027, assign29760_e44027_d_n0, assign29760_e44027_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29760_e44022: f64 = (var_tmf1 * var_tmf1);
        let assign29760_e44024: f64 = (assign29760_e44022 + var_tmf2);
        let assign29760_e44025: f64 = (assign29760_e44024).sqrt();
        (assign29760_e44025, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign29760_e44025)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign29760_e44025)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29760_e44027;
        var_tmf2_dn0 = assign29760_e44027_d_n0;
        var_tmf2_dn2 = assign29760_e44027_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29770_e44044, assign29770_e44044_d_n0, assign29770_e44044_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29770_e44040: f64 = (var_tmf1 + var_tmf2);
        let assign29770_e44041: f64 = (0.5 * assign29770_e44040);
        let assign29770_e44042: f64 = (p.p85 - assign29770_e44041);
        (assign29770_e44042, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign29770_e44044;
        var_nja11_dn0 = assign29770_e44044_d_n0;
        var_nja11_dn2 = assign29770_e44044_d_n2;
        var_nja11_rv = 0.0;

        *var_guard484_slot = var_guard484;
        *var_guard484_rv_slot = var_guard484_rv;
        *var_guard485_slot = var_guard485;
        *var_guard485_rv_slot = var_guard485_rv;
        *var_guard486_slot = var_guard486;
        *var_guard486_rv_slot = var_guard486_rv;
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

    pub(super) fn stamp_reactive_block_32(
        p: &Parameters,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard486: f64,
        var_ndigat_i: f64,
        var_nfagat_i: f64,
        var_nfasti_i: f64,
        var_nin: f64,
        var_phitdinv: f64,
        var_vak: f64,
        var_vak_dn0: f64,
        var_vak_dn2: f64,
        var_guard489_slot: &mut f64,
        var_guard489_rv_slot: &mut f64,
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
        let mut var_guard489: f64 = *var_guard489_slot;
        let mut var_guard489_rv: f64 = *var_guard489_rv_slot;
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

        let (assign29780_e44059, assign29780_e44059_d_n0, assign29780_e44059_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29780_e44055: f64 = (var_nja11 - var_nfasti_i);
        let assign29780_e44057: f64 = (assign29780_e44055 - 0.01);
        (assign29780_e44057, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign29780_e44059;
        var_tmf1_dn0 = assign29780_e44059_d_n0;
        var_tmf1_dn2 = assign29780_e44059_d_n2;
        var_tmf1_rv = 0.0;

        let (assign29790_e44074, assign29790_e44074_d_n0, assign29790_e44074_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29790_e44070: f64 = (4.0 * var_nfasti_i);
        let assign29790_e44072: f64 = (assign29790_e44070 * 0.01);
        (assign29790_e44072, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29790_e44074;
        var_tmf2_dn0 = assign29790_e44074_d_n0;
        var_tmf2_dn2 = assign29790_e44074_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29800_e44091, assign29800_e44091_d_n0, assign29800_e44091_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let (assign29800_e44089, assign29800_e44089_d_n0, assign29800_e44089_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign29800_e44088: f64 = (-var_tmf2);
                (assign29800_e44088, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign29800_e44089, assign29800_e44089_d_n0, assign29800_e44089_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29800_e44091;
        var_tmf2_dn0 = assign29800_e44091_d_n0;
        var_tmf2_dn2 = assign29800_e44091_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29810_e44107, assign29810_e44107_d_n0, assign29810_e44107_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29810_e44102: f64 = (var_tmf1 * var_tmf1);
        let assign29810_e44104: f64 = (assign29810_e44102 + var_tmf2);
        let assign29810_e44105: f64 = (assign29810_e44104).sqrt();
        (assign29810_e44105, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign29810_e44105)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign29810_e44105)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29810_e44107;
        var_tmf2_dn0 = assign29810_e44107_d_n0;
        var_tmf2_dn2 = assign29810_e44107_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29820_e44124, assign29820_e44124_d_n0, assign29820_e44124_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29820_e44120: f64 = (var_tmf1 + var_tmf2);
        let assign29820_e44121: f64 = (0.5 * assign29820_e44120);
        let assign29820_e44122: f64 = (var_nfasti_i + assign29820_e44121);
        (assign29820_e44122, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign29820_e44124;
        var_nj1_dn0 = assign29820_e44124_d_n0;
        var_nj1_dn2 = assign29820_e44124_d_n2;
        var_nj1_rv = 0.0;

        let (assign29830_e44139, assign29830_e44139_d_n0, assign29830_e44139_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29830_e44135: f64 = (p.p85 - var_nj0);
        let assign29830_e44137: f64 = (assign29830_e44135 - 0.01);
        (assign29830_e44137, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign29830_e44139;
        var_tmf1_dn0 = assign29830_e44139_d_n0;
        var_tmf1_dn2 = assign29830_e44139_d_n2;
        var_tmf1_rv = 0.0;

        let (assign29840_e44154, assign29840_e44154_d_n0, assign29840_e44154_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29840_e44150: f64 = (4.0 * p.p85);
        let assign29840_e44152: f64 = (assign29840_e44150 * 0.01);
        (assign29840_e44152, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29840_e44154;
        var_tmf2_dn0 = assign29840_e44154_d_n0;
        var_tmf2_dn2 = assign29840_e44154_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29850_e44171, assign29850_e44171_d_n0, assign29850_e44171_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let (assign29850_e44169, assign29850_e44169_d_n0, assign29850_e44169_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign29850_e44168: f64 = (-var_tmf2);
                (assign29850_e44168, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign29850_e44169, assign29850_e44169_d_n0, assign29850_e44169_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29850_e44171;
        var_tmf2_dn0 = assign29850_e44171_d_n0;
        var_tmf2_dn2 = assign29850_e44171_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29860_e44187, assign29860_e44187_d_n0, assign29860_e44187_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29860_e44182: f64 = (var_tmf1 * var_tmf1);
        let assign29860_e44184: f64 = (assign29860_e44182 + var_tmf2);
        let assign29860_e44185: f64 = (assign29860_e44184).sqrt();
        (assign29860_e44185, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign29860_e44185)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign29860_e44185)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29860_e44187;
        var_tmf2_dn0 = assign29860_e44187_d_n0;
        var_tmf2_dn2 = assign29860_e44187_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29870_e44204, assign29870_e44204_d_n0, assign29870_e44204_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29870_e44200: f64 = (var_tmf1 + var_tmf2);
        let assign29870_e44201: f64 = (0.5 * assign29870_e44200);
        let assign29870_e44202: f64 = (p.p85 - assign29870_e44201);
        (assign29870_e44202, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign29870_e44204;
        var_nj0_dn0 = assign29870_e44204_d_n0;
        var_nj0_dn2 = assign29870_e44204_d_n2;
        var_nj0_rv = 0.0;

        let (assign29880_e44219, assign29880_e44219_d_n0, assign29880_e44219_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29880_e44215: f64 = (var_nj0 - var_nfasti_i);
        let assign29880_e44217: f64 = (assign29880_e44215 - 0.01);
        (assign29880_e44217, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign29880_e44219;
        var_tmf1_dn0 = assign29880_e44219_d_n0;
        var_tmf1_dn2 = assign29880_e44219_d_n2;
        var_tmf1_rv = 0.0;

        let (assign29890_e44234, assign29890_e44234_d_n0, assign29890_e44234_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29890_e44230: f64 = (4.0 * var_nfasti_i);
        let assign29890_e44232: f64 = (assign29890_e44230 * 0.01);
        (assign29890_e44232, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29890_e44234;
        var_tmf2_dn0 = assign29890_e44234_d_n0;
        var_tmf2_dn2 = assign29890_e44234_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29900_e44251, assign29900_e44251_d_n0, assign29900_e44251_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let (assign29900_e44249, assign29900_e44249_d_n0, assign29900_e44249_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign29900_e44248: f64 = (-var_tmf2);
                (assign29900_e44248, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign29900_e44249, assign29900_e44249_d_n0, assign29900_e44249_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29900_e44251;
        var_tmf2_dn0 = assign29900_e44251_d_n0;
        var_tmf2_dn2 = assign29900_e44251_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29910_e44267, assign29910_e44267_d_n0, assign29910_e44267_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29910_e44262: f64 = (var_tmf1 * var_tmf1);
        let assign29910_e44264: f64 = (assign29910_e44262 + var_tmf2);
        let assign29910_e44265: f64 = (assign29910_e44264).sqrt();
        (assign29910_e44265, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign29910_e44265)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign29910_e44265)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign29910_e44267;
        var_tmf2_dn0 = assign29910_e44267_d_n0;
        var_tmf2_dn2 = assign29910_e44267_d_n2;
        var_tmf2_rv = 0.0;

        let (assign29920_e44284, assign29920_e44284_d_n0, assign29920_e44284_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 != 0.0)) {
        let assign29920_e44280: f64 = (var_tmf1 + var_tmf2);
        let assign29920_e44281: f64 = (0.5 * assign29920_e44280);
        let assign29920_e44282: f64 = (var_nfasti_i + assign29920_e44281);
        (assign29920_e44282, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign29920_e44284;
        var_nj0_dn0 = assign29920_e44284_d_n0;
        var_nj0_dn2 = assign29920_e44284_d_n2;
        var_nj0_rv = 0.0;

        let (assign29930_e44296, assign29930_e44296_d_n0, assign29930_e44296_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign29930_e44296;
        var_nj0_dn0 = assign29930_e44296_d_n0;
        var_nj0_dn2 = assign29930_e44296_d_n2;
        var_nj0_rv = 0.0;

        let (assign29940_e44308, assign29940_e44308_d_n0, assign29940_e44308_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard486 == 0.0)) {
        (var_nfasti_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign29940_e44308;
        var_nj1_dn0 = assign29940_e44308_d_n0;
        var_nj1_dn2 = assign29940_e44308_d_n2;
        var_nj1_rv = 0.0;

        let (assign30000_e44543,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) {
        let assign30000_e44539: f64 = (var_nin * var_nin);
        let assign30000_e44541: f64 = (assign30000_e44539 / var_ndigat_i);
        (assign30000_e44541,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign30000_e44543;
        var_pnn0_rv = 0.0;

        let (assign30010_e44559,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) {
        let assign30010_e44552: f64 = (var_nfagat_i / var_phitdinv);
        let assign30010_e44555: f64 = (var_ndigat_i / var_pnn0);
        let assign30010_e44556: f64 = (assign30010_e44555).ln();
        let assign30010_e44557: f64 = (assign30010_e44552 * assign30010_e44556);
        (assign30010_e44557,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign30010_e44559;
        var_vha1_rv = 0.0;

        let assign30020_e44562: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard489 = assign30020_e44562;
        var_guard489_rv = 0.0;

        let (assign30030_e44579, assign30030_e44579_d_n0, assign30030_e44579_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30030_e44574: f64 = (var_vak - var_vha1);
        let assign30030_e44575: f64 = (p.p86 * assign30030_e44574);
        let assign30030_e44577: f64 = (assign30030_e44575 + var_nfagat_i);
        (assign30030_e44577, (p.p86 * var_vak_dn0), (p.p86 * var_vak_dn2),)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign30030_e44579;
        var_nja10_dn0 = assign30030_e44579_d_n0;
        var_nja10_dn2 = assign30030_e44579_d_n2;
        var_nja10_rv = 0.0;

        let (assign30040_e44594, assign30040_e44594_d_n0, assign30040_e44594_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30040_e44591: f64 = (p.p86 * var_vha1);
        let assign30040_e44592: f64 = (var_nfagat_i - assign30040_e44591);
        (assign30040_e44592, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign30040_e44594;
        var_nj0_dn0 = assign30040_e44594_d_n0;
        var_nj0_dn2 = assign30040_e44594_d_n2;
        var_nj0_rv = 0.0;

        let (assign30050_e44609, assign30050_e44609_d_n0, assign30050_e44609_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30050_e44605: f64 = (p.p85 - var_nja10);
        let assign30050_e44607: f64 = (assign30050_e44605 - 0.01);
        (assign30050_e44607, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign30050_e44609;
        var_tmf1_dn0 = assign30050_e44609_d_n0;
        var_tmf1_dn2 = assign30050_e44609_d_n2;
        var_tmf1_rv = 0.0;

        let (assign30060_e44624, assign30060_e44624_d_n0, assign30060_e44624_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30060_e44620: f64 = (4.0 * p.p85);
        let assign30060_e44622: f64 = (assign30060_e44620 * 0.01);
        (assign30060_e44622, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30060_e44624;
        var_tmf2_dn0 = assign30060_e44624_d_n0;
        var_tmf2_dn2 = assign30060_e44624_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30070_e44641, assign30070_e44641_d_n0, assign30070_e44641_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let (assign30070_e44639, assign30070_e44639_d_n0, assign30070_e44639_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign30070_e44638: f64 = (-var_tmf2);
                (assign30070_e44638, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign30070_e44639, assign30070_e44639_d_n0, assign30070_e44639_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30070_e44641;
        var_tmf2_dn0 = assign30070_e44641_d_n0;
        var_tmf2_dn2 = assign30070_e44641_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30080_e44657, assign30080_e44657_d_n0, assign30080_e44657_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30080_e44652: f64 = (var_tmf1 * var_tmf1);
        let assign30080_e44654: f64 = (assign30080_e44652 + var_tmf2);
        let assign30080_e44655: f64 = (assign30080_e44654).sqrt();
        (assign30080_e44655, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30080_e44655)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30080_e44655)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30080_e44657;
        var_tmf2_dn0 = assign30080_e44657_d_n0;
        var_tmf2_dn2 = assign30080_e44657_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30090_e44674, assign30090_e44674_d_n0, assign30090_e44674_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30090_e44670: f64 = (var_tmf1 + var_tmf2);
        let assign30090_e44671: f64 = (0.5 * assign30090_e44670);
        let assign30090_e44672: f64 = (p.p85 - assign30090_e44671);
        (assign30090_e44672, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign30090_e44674;
        var_nja11_dn0 = assign30090_e44674_d_n0;
        var_nja11_dn2 = assign30090_e44674_d_n2;
        var_nja11_rv = 0.0;

        let (assign30100_e44689, assign30100_e44689_d_n0, assign30100_e44689_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30100_e44685: f64 = (var_nja11 - var_nfagat_i);
        let assign30100_e44687: f64 = (assign30100_e44685 - 0.01);
        (assign30100_e44687, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign30100_e44689;
        var_tmf1_dn0 = assign30100_e44689_d_n0;
        var_tmf1_dn2 = assign30100_e44689_d_n2;
        var_tmf1_rv = 0.0;

        let (assign30110_e44704, assign30110_e44704_d_n0, assign30110_e44704_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30110_e44700: f64 = (4.0 * var_nfagat_i);
        let assign30110_e44702: f64 = (assign30110_e44700 * 0.01);
        (assign30110_e44702, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30110_e44704;
        var_tmf2_dn0 = assign30110_e44704_d_n0;
        var_tmf2_dn2 = assign30110_e44704_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30120_e44721, assign30120_e44721_d_n0, assign30120_e44721_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let (assign30120_e44719, assign30120_e44719_d_n0, assign30120_e44719_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign30120_e44718: f64 = (-var_tmf2);
                (assign30120_e44718, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign30120_e44719, assign30120_e44719_d_n0, assign30120_e44719_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30120_e44721;
        var_tmf2_dn0 = assign30120_e44721_d_n0;
        var_tmf2_dn2 = assign30120_e44721_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30130_e44737, assign30130_e44737_d_n0, assign30130_e44737_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30130_e44732: f64 = (var_tmf1 * var_tmf1);
        let assign30130_e44734: f64 = (assign30130_e44732 + var_tmf2);
        let assign30130_e44735: f64 = (assign30130_e44734).sqrt();
        (assign30130_e44735, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30130_e44735)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30130_e44735)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30130_e44737;
        var_tmf2_dn0 = assign30130_e44737_d_n0;
        var_tmf2_dn2 = assign30130_e44737_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30140_e44754, assign30140_e44754_d_n0, assign30140_e44754_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30140_e44750: f64 = (var_tmf1 + var_tmf2);
        let assign30140_e44751: f64 = (0.5 * assign30140_e44750);
        let assign30140_e44752: f64 = (var_nfagat_i + assign30140_e44751);
        (assign30140_e44752, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign30140_e44754;
        var_nj1_dn0 = assign30140_e44754_d_n0;
        var_nj1_dn2 = assign30140_e44754_d_n2;
        var_nj1_rv = 0.0;

        let (assign30150_e44769, assign30150_e44769_d_n0, assign30150_e44769_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30150_e44765: f64 = (p.p85 - var_nj0);
        let assign30150_e44767: f64 = (assign30150_e44765 - 0.01);
        (assign30150_e44767, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign30150_e44769;
        var_tmf1_dn0 = assign30150_e44769_d_n0;
        var_tmf1_dn2 = assign30150_e44769_d_n2;
        var_tmf1_rv = 0.0;

        let (assign30160_e44784, assign30160_e44784_d_n0, assign30160_e44784_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30160_e44780: f64 = (4.0 * p.p85);
        let assign30160_e44782: f64 = (assign30160_e44780 * 0.01);
        (assign30160_e44782, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30160_e44784;
        var_tmf2_dn0 = assign30160_e44784_d_n0;
        var_tmf2_dn2 = assign30160_e44784_d_n2;
        var_tmf2_rv = 0.0;

        *var_guard489_slot = var_guard489;
        *var_guard489_rv_slot = var_guard489_rv;
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

    pub(super) fn stamp_reactive_block_33(
        p: &Parameters,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard489: f64,
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
        var_guard492_slot: &mut f64,
        var_guard492_rv_slot: &mut f64,
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
        let mut var_guard492: f64 = *var_guard492_slot;
        let mut var_guard492_rv: f64 = *var_guard492_rv_slot;
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

        let (assign30170_e44801, assign30170_e44801_d_n0, assign30170_e44801_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let (assign30170_e44799, assign30170_e44799_d_n0, assign30170_e44799_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign30170_e44798: f64 = (-var_tmf2);
                (assign30170_e44798, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign30170_e44799, assign30170_e44799_d_n0, assign30170_e44799_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30170_e44801;
        var_tmf2_dn0 = assign30170_e44801_d_n0;
        var_tmf2_dn2 = assign30170_e44801_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30180_e44817, assign30180_e44817_d_n0, assign30180_e44817_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30180_e44812: f64 = (var_tmf1 * var_tmf1);
        let assign30180_e44814: f64 = (assign30180_e44812 + var_tmf2);
        let assign30180_e44815: f64 = (assign30180_e44814).sqrt();
        (assign30180_e44815, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30180_e44815)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30180_e44815)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30180_e44817;
        var_tmf2_dn0 = assign30180_e44817_d_n0;
        var_tmf2_dn2 = assign30180_e44817_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30190_e44834, assign30190_e44834_d_n0, assign30190_e44834_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30190_e44830: f64 = (var_tmf1 + var_tmf2);
        let assign30190_e44831: f64 = (0.5 * assign30190_e44830);
        let assign30190_e44832: f64 = (p.p85 - assign30190_e44831);
        (assign30190_e44832, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign30190_e44834;
        var_nj0_dn0 = assign30190_e44834_d_n0;
        var_nj0_dn2 = assign30190_e44834_d_n2;
        var_nj0_rv = 0.0;

        let (assign30200_e44849, assign30200_e44849_d_n0, assign30200_e44849_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30200_e44845: f64 = (var_nj0 - var_nfagat_i);
        let assign30200_e44847: f64 = (assign30200_e44845 - 0.01);
        (assign30200_e44847, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign30200_e44849;
        var_tmf1_dn0 = assign30200_e44849_d_n0;
        var_tmf1_dn2 = assign30200_e44849_d_n2;
        var_tmf1_rv = 0.0;

        let (assign30210_e44864, assign30210_e44864_d_n0, assign30210_e44864_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30210_e44860: f64 = (4.0 * var_nfagat_i);
        let assign30210_e44862: f64 = (assign30210_e44860 * 0.01);
        (assign30210_e44862, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30210_e44864;
        var_tmf2_dn0 = assign30210_e44864_d_n0;
        var_tmf2_dn2 = assign30210_e44864_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30220_e44881, assign30220_e44881_d_n0, assign30220_e44881_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let (assign30220_e44879, assign30220_e44879_d_n0, assign30220_e44879_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign30220_e44878: f64 = (-var_tmf2);
                (assign30220_e44878, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign30220_e44879, assign30220_e44879_d_n0, assign30220_e44879_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30220_e44881;
        var_tmf2_dn0 = assign30220_e44881_d_n0;
        var_tmf2_dn2 = assign30220_e44881_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30230_e44897, assign30230_e44897_d_n0, assign30230_e44897_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30230_e44892: f64 = (var_tmf1 * var_tmf1);
        let assign30230_e44894: f64 = (assign30230_e44892 + var_tmf2);
        let assign30230_e44895: f64 = (assign30230_e44894).sqrt();
        (assign30230_e44895, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30230_e44895)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30230_e44895)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30230_e44897;
        var_tmf2_dn0 = assign30230_e44897_d_n0;
        var_tmf2_dn2 = assign30230_e44897_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30240_e44914, assign30240_e44914_d_n0, assign30240_e44914_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 != 0.0)) {
        let assign30240_e44910: f64 = (var_tmf1 + var_tmf2);
        let assign30240_e44911: f64 = (0.5 * assign30240_e44910);
        let assign30240_e44912: f64 = (var_nfagat_i + assign30240_e44911);
        (assign30240_e44912, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign30240_e44914;
        var_nj0_dn0 = assign30240_e44914_d_n0;
        var_nj0_dn2 = assign30240_e44914_d_n2;
        var_nj0_rv = 0.0;

        let (assign30250_e44926, assign30250_e44926_d_n0, assign30250_e44926_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign30250_e44926;
        var_nj0_dn0 = assign30250_e44926_d_n0;
        var_nj0_dn2 = assign30250_e44926_d_n2;
        var_nj0_rv = 0.0;

        let (assign30260_e44938, assign30260_e44938_d_n0, assign30260_e44938_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 != 0.0)) && (var_guard489 == 0.0)) {
        (var_nfagat_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign30260_e44938;
        var_nj1_dn0 = assign30260_e44938_d_n0;
        var_nj1_dn2 = assign30260_e44938_d_n2;
        var_nj1_rv = 0.0;

        let (assign30330_e45193,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30330_e45189: f64 = (var_nin * var_nin);
        let assign30330_e45191: f64 = (assign30330_e45189 / var_ndibot_i);
        (assign30330_e45191,)
    } else {
        (var_pnn0,)
    }
};
        var_pnn0 = assign30330_e45193;
        var_pnn0_rv = 0.0;

        let (assign30340_e45210,) = {
    if (((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) {
        let assign30340_e45203: f64 = (var_nfabot_i / var_phitdinv);
        let assign30340_e45206: f64 = (var_ndibot_i / var_pnn0);
        let assign30340_e45207: f64 = (assign30340_e45206).ln();
        let assign30340_e45208: f64 = (assign30340_e45203 * assign30340_e45207);
        (assign30340_e45208,)
    } else {
        (var_vha1,)
    }
};
        var_vha1 = assign30340_e45210;
        var_vha1_rv = 0.0;

        let assign30350_e45213: f64 = if var_nfabot_i < p.p85 { 1.0 } else { 0.0 };
        var_guard492 = assign30350_e45213;
        var_guard492_rv = 0.0;

        let (assign30360_e45231, assign30360_e45231_d_n0, assign30360_e45231_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30360_e45226: f64 = (var_vmax - var_vha1);
        let assign30360_e45227: f64 = (p.p86 * assign30360_e45226);
        let assign30360_e45229: f64 = (assign30360_e45227 + var_nfabot_i);
        (assign30360_e45229, 0.0, 0.0,)
    } else {
        (var_nja10, var_nja10_dn0, var_nja10_dn2,)
    }
};
        var_nja10 = assign30360_e45231;
        var_nja10_dn0 = assign30360_e45231_d_n0;
        var_nja10_dn2 = assign30360_e45231_d_n2;
        var_nja10_rv = 0.0;

        let (assign30370_e45247, assign30370_e45247_d_n0, assign30370_e45247_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30370_e45244: f64 = (p.p86 * var_vha1);
        let assign30370_e45245: f64 = (var_nfabot_i - assign30370_e45244);
        (assign30370_e45245, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign30370_e45247;
        var_nj0_dn0 = assign30370_e45247_d_n0;
        var_nj0_dn2 = assign30370_e45247_d_n2;
        var_nj0_rv = 0.0;

        let (assign30380_e45263, assign30380_e45263_d_n0, assign30380_e45263_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30380_e45259: f64 = (p.p85 - var_nja10);
        let assign30380_e45261: f64 = (assign30380_e45259 - 0.01);
        (assign30380_e45261, (-var_nja10_dn0), (-var_nja10_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign30380_e45263;
        var_tmf1_dn0 = assign30380_e45263_d_n0;
        var_tmf1_dn2 = assign30380_e45263_d_n2;
        var_tmf1_rv = 0.0;

        let (assign30390_e45279, assign30390_e45279_d_n0, assign30390_e45279_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30390_e45275: f64 = (4.0 * p.p85);
        let assign30390_e45277: f64 = (assign30390_e45275 * 0.01);
        (assign30390_e45277, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30390_e45279;
        var_tmf2_dn0 = assign30390_e45279_d_n0;
        var_tmf2_dn2 = assign30390_e45279_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30400_e45297, assign30400_e45297_d_n0, assign30400_e45297_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let (assign30400_e45295, assign30400_e45295_d_n0, assign30400_e45295_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign30400_e45294: f64 = (-var_tmf2);
                (assign30400_e45294, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign30400_e45295, assign30400_e45295_d_n0, assign30400_e45295_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30400_e45297;
        var_tmf2_dn0 = assign30400_e45297_d_n0;
        var_tmf2_dn2 = assign30400_e45297_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30410_e45314, assign30410_e45314_d_n0, assign30410_e45314_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30410_e45309: f64 = (var_tmf1 * var_tmf1);
        let assign30410_e45311: f64 = (assign30410_e45309 + var_tmf2);
        let assign30410_e45312: f64 = (assign30410_e45311).sqrt();
        (assign30410_e45312, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30410_e45312)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30410_e45312)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30410_e45314;
        var_tmf2_dn0 = assign30410_e45314_d_n0;
        var_tmf2_dn2 = assign30410_e45314_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30420_e45332, assign30420_e45332_d_n0, assign30420_e45332_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30420_e45328: f64 = (var_tmf1 / var_tmf2);
        let assign30420_e45329: f64 = (1.0 + assign30420_e45328);
        let assign30420_e45330: f64 = (0.5 * assign30420_e45329);
        (assign30420_e45330, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_su, var_dfn_su_dn0, var_dfn_su_dn2,)
    }
};
        var_dfn_su = assign30420_e45332;
        var_dfn_su_dn0 = assign30420_e45332_d_n0;
        var_dfn_su_dn2 = assign30420_e45332_d_n2;
        var_dfn_su_rv = 0.0;

        let (assign30430_e45350, assign30430_e45350_d_n0, assign30430_e45350_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30430_e45346: f64 = (var_tmf1 + var_tmf2);
        let assign30430_e45347: f64 = (0.5 * assign30430_e45346);
        let assign30430_e45348: f64 = (p.p85 - assign30430_e45347);
        (assign30430_e45348, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nja11, var_nja11_dn0, var_nja11_dn2,)
    }
};
        var_nja11 = assign30430_e45350;
        var_nja11_dn0 = assign30430_e45350_d_n0;
        var_nja11_dn2 = assign30430_e45350_d_n2;
        var_nja11_rv = 0.0;

        let (assign30440_e45366, assign30440_e45366_d_n0, assign30440_e45366_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30440_e45362: f64 = (var_nja11 - var_nfabot_i);
        let assign30440_e45364: f64 = (assign30440_e45362 - 0.01);
        (assign30440_e45364, var_nja11_dn0, var_nja11_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign30440_e45366;
        var_tmf1_dn0 = assign30440_e45366_d_n0;
        var_tmf1_dn2 = assign30440_e45366_d_n2;
        var_tmf1_rv = 0.0;

        let (assign30450_e45382, assign30450_e45382_d_n0, assign30450_e45382_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30450_e45378: f64 = (4.0 * var_nfabot_i);
        let assign30450_e45380: f64 = (assign30450_e45378 * 0.01);
        (assign30450_e45380, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30450_e45382;
        var_tmf2_dn0 = assign30450_e45382_d_n0;
        var_tmf2_dn2 = assign30450_e45382_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30460_e45400, assign30460_e45400_d_n0, assign30460_e45400_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let (assign30460_e45398, assign30460_e45398_d_n0, assign30460_e45398_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign30460_e45397: f64 = (-var_tmf2);
                (assign30460_e45397, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign30460_e45398, assign30460_e45398_d_n0, assign30460_e45398_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30460_e45400;
        var_tmf2_dn0 = assign30460_e45400_d_n0;
        var_tmf2_dn2 = assign30460_e45400_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30470_e45417, assign30470_e45417_d_n0, assign30470_e45417_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30470_e45412: f64 = (var_tmf1 * var_tmf1);
        let assign30470_e45414: f64 = (assign30470_e45412 + var_tmf2);
        let assign30470_e45415: f64 = (assign30470_e45414).sqrt();
        (assign30470_e45415, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30470_e45415)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30470_e45415)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30470_e45417;
        var_tmf2_dn0 = assign30470_e45417_d_n0;
        var_tmf2_dn2 = assign30470_e45417_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30480_e45435, assign30480_e45435_d_n0, assign30480_e45435_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30480_e45431: f64 = (var_tmf1 / var_tmf2);
        let assign30480_e45432: f64 = (1.0 + assign30480_e45431);
        let assign30480_e45433: f64 = (0.5 * assign30480_e45432);
        (assign30480_e45433, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_dfn_sl, var_dfn_sl_dn0, var_dfn_sl_dn2,)
    }
};
        var_dfn_sl = assign30480_e45435;
        var_dfn_sl_dn0 = assign30480_e45435_d_n0;
        var_dfn_sl_dn2 = assign30480_e45435_d_n2;
        var_dfn_sl_rv = 0.0;

        let (assign30490_e45453, assign30490_e45453_d_n0, assign30490_e45453_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30490_e45449: f64 = (var_tmf1 + var_tmf2);
        let assign30490_e45450: f64 = (0.5 * assign30490_e45449);
        let assign30490_e45451: f64 = (var_nfabot_i + assign30490_e45450);
        (assign30490_e45451, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign30490_e45453;
        var_nj1_dn0 = assign30490_e45453_d_n0;
        var_nj1_dn2 = assign30490_e45453_d_n2;
        var_nj1_rv = 0.0;

        let (assign30500_e45469, assign30500_e45469_d_n0, assign30500_e45469_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30500_e45465: f64 = (p.p85 - var_nj0);
        let assign30500_e45467: f64 = (assign30500_e45465 - 0.01);
        (assign30500_e45467, (-var_nj0_dn0), (-var_nj0_dn2),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign30500_e45469;
        var_tmf1_dn0 = assign30500_e45469_d_n0;
        var_tmf1_dn2 = assign30500_e45469_d_n2;
        var_tmf1_rv = 0.0;

        let (assign30510_e45485, assign30510_e45485_d_n0, assign30510_e45485_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30510_e45481: f64 = (4.0 * p.p85);
        let assign30510_e45483: f64 = (assign30510_e45481 * 0.01);
        (assign30510_e45483, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30510_e45485;
        var_tmf2_dn0 = assign30510_e45485_d_n0;
        var_tmf2_dn2 = assign30510_e45485_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30520_e45503, assign30520_e45503_d_n0, assign30520_e45503_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let (assign30520_e45501, assign30520_e45501_d_n0, assign30520_e45501_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign30520_e45500: f64 = (-var_tmf2);
                (assign30520_e45500, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign30520_e45501, assign30520_e45501_d_n0, assign30520_e45501_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30520_e45503;
        var_tmf2_dn0 = assign30520_e45503_d_n0;
        var_tmf2_dn2 = assign30520_e45503_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30530_e45520, assign30530_e45520_d_n0, assign30530_e45520_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30530_e45515: f64 = (var_tmf1 * var_tmf1);
        let assign30530_e45517: f64 = (assign30530_e45515 + var_tmf2);
        let assign30530_e45518: f64 = (assign30530_e45517).sqrt();
        (assign30530_e45518, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30530_e45518)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30530_e45518)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30530_e45520;
        var_tmf2_dn0 = assign30530_e45520_d_n0;
        var_tmf2_dn2 = assign30530_e45520_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30540_e45538, assign30540_e45538_d_n0, assign30540_e45538_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30540_e45534: f64 = (var_tmf1 + var_tmf2);
        let assign30540_e45535: f64 = (0.5 * assign30540_e45534);
        let assign30540_e45536: f64 = (p.p85 - assign30540_e45535);
        (assign30540_e45536, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign30540_e45538;
        var_nj0_dn0 = assign30540_e45538_d_n0;
        var_nj0_dn2 = assign30540_e45538_d_n2;
        var_nj0_rv = 0.0;

        let (assign30550_e45554, assign30550_e45554_d_n0, assign30550_e45554_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30550_e45550: f64 = (var_nj0 - var_nfabot_i);
        let assign30550_e45552: f64 = (assign30550_e45550 - 0.01);
        (assign30550_e45552, var_nj0_dn0, var_nj0_dn2,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2,)
    }
};
        var_tmf1 = assign30550_e45554;
        var_tmf1_dn0 = assign30550_e45554_d_n0;
        var_tmf1_dn2 = assign30550_e45554_d_n2;
        var_tmf1_rv = 0.0;

        let (assign30560_e45570, assign30560_e45570_d_n0, assign30560_e45570_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30560_e45566: f64 = (4.0 * var_nfabot_i);
        let assign30560_e45568: f64 = (assign30560_e45566 * 0.01);
        (assign30560_e45568, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30560_e45570;
        var_tmf2_dn0 = assign30560_e45570_d_n0;
        var_tmf2_dn2 = assign30560_e45570_d_n2;
        var_tmf2_rv = 0.0;

        *var_dfn_sl_slot = var_dfn_sl;
        *var_dfn_sl_dn0_slot = var_dfn_sl_dn0;
        *var_dfn_sl_dn2_slot = var_dfn_sl_dn2;
        *var_dfn_sl_rv_slot = var_dfn_sl_rv;
        *var_dfn_su_slot = var_dfn_su;
        *var_dfn_su_dn0_slot = var_dfn_su_dn0;
        *var_dfn_su_dn2_slot = var_dfn_su_dn2;
        *var_dfn_su_rv_slot = var_dfn_su_rv;
        *var_guard492_slot = var_guard492;
        *var_guard492_rv_slot = var_guard492_rv;
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

    pub(super) fn stamp_reactive_block_34(
        p: &Parameters,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard492: f64,
        var_ndisti_i: f64,
        var_nfabot_i: f64,
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
        var_guard493_slot: &mut f64,
        var_guard493_rv_slot: &mut f64,
        var_guard494_slot: &mut f64,
        var_guard494_rv_slot: &mut f64,
        var_guard495_slot: &mut f64,
        var_guard495_rv_slot: &mut f64,
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
        let mut var_guard493: f64 = *var_guard493_slot;
        let mut var_guard493_rv: f64 = *var_guard493_rv_slot;
        let mut var_guard494: f64 = *var_guard494_slot;
        let mut var_guard494_rv: f64 = *var_guard494_rv_slot;
        let mut var_guard495: f64 = *var_guard495_slot;
        let mut var_guard495_rv: f64 = *var_guard495_rv_slot;
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

        let (assign30570_e45588, assign30570_e45588_d_n0, assign30570_e45588_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let (assign30570_e45586, assign30570_e45586_d_n0, assign30570_e45586_d_n2,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
            } else {
                let assign30570_e45585: f64 = (-var_tmf2);
                (assign30570_e45585, (-var_tmf2_dn0), (-var_tmf2_dn2),)
            }
        };
        (assign30570_e45586, assign30570_e45586_d_n0, assign30570_e45586_d_n2,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30570_e45588;
        var_tmf2_dn0 = assign30570_e45588_d_n0;
        var_tmf2_dn2 = assign30570_e45588_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30580_e45605, assign30580_e45605_d_n0, assign30580_e45605_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30580_e45600: f64 = (var_tmf1 * var_tmf1);
        let assign30580_e45602: f64 = (assign30580_e45600 + var_tmf2);
        let assign30580_e45603: f64 = (assign30580_e45602).sqrt();
        (assign30580_e45603, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign30580_e45603)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign30580_e45603)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2,)
    }
};
        var_tmf2 = assign30580_e45605;
        var_tmf2_dn0 = assign30580_e45605_d_n0;
        var_tmf2_dn2 = assign30580_e45605_d_n2;
        var_tmf2_rv = 0.0;

        let (assign30590_e45623, assign30590_e45623_d_n0, assign30590_e45623_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30590_e45619: f64 = (var_tmf1 + var_tmf2);
        let assign30590_e45620: f64 = (0.5 * assign30590_e45619);
        let assign30590_e45621: f64 = (var_nfabot_i + assign30590_e45620);
        (assign30590_e45621, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)),)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign30590_e45623;
        var_nj0_dn0 = assign30590_e45623_d_n0;
        var_nj0_dn2 = assign30590_e45623_d_n2;
        var_nj0_rv = 0.0;

        let (assign30600_e45639, assign30600_e45639_d_n0, assign30600_e45639_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 != 0.0)) {
        let assign30600_e45635: f64 = (p.p86 * var_dfn_su);
        let assign30600_e45637: f64 = (assign30600_e45635 * var_dfn_sl);
        (assign30600_e45637, (((p.p86 * var_dfn_su_dn0) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_dn0)), (((p.p86 * var_dfn_su_dn2) * var_dfn_sl) + (assign30600_e45635 * var_dfn_sl_dn2)),)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign30600_e45639;
        var_dnj1_dv_dn0 = assign30600_e45639_d_n0;
        var_dnj1_dv_dn2 = assign30600_e45639_d_n2;
        var_dnj1_dv_rv = 0.0;

        let (assign30610_e45652, assign30610_e45652_d_n0, assign30610_e45652_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj0, var_nj0_dn0, var_nj0_dn2,)
    }
};
        var_nj0 = assign30610_e45652;
        var_nj0_dn0 = assign30610_e45652_d_n0;
        var_nj0_dn2 = assign30610_e45652_d_n2;
        var_nj0_rv = 0.0;

        let (assign30620_e45665, assign30620_e45665_d_n0, assign30620_e45665_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 == 0.0)) {
        (var_nfabot_i, 0.0, 0.0,)
    } else {
        (var_nj1, var_nj1_dn0, var_nj1_dn2,)
    }
};
        var_nj1 = assign30620_e45665;
        var_nj1_dn0 = assign30620_e45665_d_n0;
        var_nj1_dn2 = assign30620_e45665_d_n2;
        var_nj1_rv = 0.0;

        let (assign30630_e45678, assign30630_e45678_d_n0, assign30630_e45678_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard492 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_dnj1_dv, var_dnj1_dv_dn0, var_dnj1_dv_dn2,)
    }
};
        var_dnj1_dv = assign30630_e45678;
        var_dnj1_dv_dn0 = assign30630_e45678_d_n0;
        var_dnj1_dv_dn2 = assign30630_e45678_d_n2;
        var_dnj1_dv_rv = 0.0;

        let assign30640_e45682: f64 = (var_vmax / var_nj1);
        let assign30640_e45686: f64 = (var_nj1 - var_nj0);
        let assign30640_e45687: f64 = (var_vha1 * assign30640_e45686);
        let assign30640_e45690: f64 = (var_nj0 * p.p85);
        let assign30640_e45691: f64 = (assign30640_e45687 / assign30640_e45690);
        let assign30640_e45692: f64 = (assign30640_e45682 + assign30640_e45691);
        let assign30640_e45693: f64 = (var_phitdinv * assign30640_e45692);
        let assign30640_e45694: f64 = (assign30640_e45693).abs();
        let assign30640_e45696: f64 = if assign30640_e45694 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard493 = assign30640_e45696;
        var_guard493_rv = 0.0;

        let (assign30650_e45723, assign30650_e45723_d_n0, assign30650_e45723_d_n2,) = {
    if ((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard493 != 0.0)) {
        let assign30650_e45709: f64 = (var_vmax / var_nj1);
        let assign30650_e45713: f64 = (var_nj1 - var_nj0);
        let assign30650_e45714: f64 = (var_vha1 * assign30650_e45713);
        let assign30650_e45717: f64 = (var_nj0 * p.p85);
        let assign30650_e45718: f64 = (assign30650_e45714 / assign30650_e45717);
        let assign30650_e45719: f64 = (assign30650_e45709 + assign30650_e45718);
        let assign30650_e45720: f64 = (var_phitdinv * assign30650_e45719);
        let assign30650_e45721: f64 = (assign30650_e45720).exp();
        (assign30650_e45721, (assign30650_e45721 * (var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_dn0 * p.p85))) / (assign30650_e45717 * assign30650_e45717))))), (assign30650_e45721 * (var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign30650_e45717) - (assign30650_e45714 * (var_nj0_dn2 * p.p85))) / (assign30650_e45717 * assign30650_e45717))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign30650_e45723;
        var_exp_vmax_over_phitd_bot_dn0 = assign30650_e45723_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign30650_e45723_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let assign30660_e45727: f64 = (var_vmax / var_nj1);
        let assign30660_e45731: f64 = (var_nj1 - var_nj0);
        let assign30660_e45732: f64 = (var_vha1 * assign30660_e45731);
        let assign30660_e45735: f64 = (var_nj0 * p.p85);
        let assign30660_e45736: f64 = (assign30660_e45732 / assign30660_e45735);
        let assign30660_e45737: f64 = (assign30660_e45727 + assign30660_e45736);
        let assign30660_e45738: f64 = (var_phitdinv * assign30660_e45737);
        let assign30660_e45740: f64 = (-230.25850929940458);
        let assign30660_e45741: f64 = if assign30660_e45738 < assign30660_e45740 { 1.0 } else { 0.0 };
        var_guard494 = assign30660_e45741;
        var_guard494_rv = 0.0;

        let (assign30670_e45823, assign30670_e45823_d_n0, assign30670_e45823_d_n2,) = {
    if (((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard493 == 0.0)) && (var_guard494 != 0.0)) {
        let assign30670_e45757: f64 = (-230.25850929940458);
        let assign30670_e45761: f64 = (var_vmax / var_nj1);
        let assign30670_e45765: f64 = (var_nj1 - var_nj0);
        let assign30670_e45766: f64 = (var_vha1 * assign30670_e45765);
        let assign30670_e45769: f64 = (var_nj0 * p.p85);
        let assign30670_e45770: f64 = (assign30670_e45766 / assign30670_e45769);
        let assign30670_e45771: f64 = (assign30670_e45761 + assign30670_e45770);
        let assign30670_e45772: f64 = (var_phitdinv * assign30670_e45771);
        let assign30670_e45773: f64 = (assign30670_e45757 - assign30670_e45772);
        let assign30670_e45777: f64 = (-230.25850929940458);
        let assign30670_e45781: f64 = (var_vmax / var_nj1);
        let assign30670_e45785: f64 = (var_nj1 - var_nj0);
        let assign30670_e45786: f64 = (var_vha1 * assign30670_e45785);
        let assign30670_e45789: f64 = (var_nj0 * p.p85);
        let assign30670_e45790: f64 = (assign30670_e45786 / assign30670_e45789);
        let assign30670_e45791: f64 = (assign30670_e45781 + assign30670_e45790);
        let assign30670_e45792: f64 = (var_phitdinv * assign30670_e45791);
        let assign30670_e45793: f64 = (assign30670_e45777 - assign30670_e45792);
        let assign30670_e45796: f64 = (-230.25850929940458);
        let assign30670_e45800: f64 = (var_vmax / var_nj1);
        let assign30670_e45804: f64 = (var_nj1 - var_nj0);
        let assign30670_e45805: f64 = (var_vha1 * assign30670_e45804);
        let assign30670_e45808: f64 = (var_nj0 * p.p85);
        let assign30670_e45809: f64 = (assign30670_e45805 / assign30670_e45808);
        let assign30670_e45810: f64 = (assign30670_e45800 + assign30670_e45809);
        let assign30670_e45811: f64 = (var_phitdinv * assign30670_e45810);
        let assign30670_e45812: f64 = (assign30670_e45796 - assign30670_e45811);
        let assign30670_e45814: f64 = (assign30670_e45812 * 0.3333333333333333);
        let assign30670_e45815: f64 = (1.0 + assign30670_e45814);
        let assign30670_e45816: f64 = (assign30670_e45793 * assign30670_e45815);
        let assign30670_e45817: f64 = (0.5 * assign30670_e45816);
        let assign30670_e45818: f64 = (1.0 + assign30670_e45817);
        let assign30670_e45819: f64 = (assign30670_e45773 * assign30670_e45818);
        let assign30670_e45820: f64 = (1.0 + assign30670_e45819);
        let assign30670_e45821: f64 = (1e-100 / assign30670_e45820);
        (assign30670_e45821, (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_dn0 * p.p85))) / (assign30670_e45769 * assign30670_e45769))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_dn0 * p.p85))) / (assign30670_e45789 * assign30670_e45789))))) * assign30670_e45815) + (assign30670_e45793 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_dn0 * p.p85))) / (assign30670_e45808 * assign30670_e45808))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))), (-((1e-100 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign30670_e45769) - (assign30670_e45766 * (var_nj0_dn2 * p.p85))) / (assign30670_e45769 * assign30670_e45769))))) * assign30670_e45818) + (assign30670_e45773 * (0.5 * (((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign30670_e45789) - (assign30670_e45786 * (var_nj0_dn2 * p.p85))) / (assign30670_e45789 * assign30670_e45789))))) * assign30670_e45815) + (assign30670_e45793 * ((-(var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign30670_e45808) - (assign30670_e45805 * (var_nj0_dn2 * p.p85))) / (assign30670_e45808 * assign30670_e45808))))) * 0.3333333333333333))))))) / (assign30670_e45820 * assign30670_e45820))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign30670_e45823;
        var_exp_vmax_over_phitd_bot_dn0 = assign30670_e45823_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign30670_e45823_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

        let (assign30680_e45903, assign30680_e45903_d_n0, assign30680_e45903_d_n2,) = {
    if (((((var_guard471 == 0.0) && (var_guard479 != 0.0)) && (var_guard480 == 0.0)) && (var_guard493 == 0.0)) && (var_guard494 == 0.0)) {
        let assign30680_e45842: f64 = (var_vmax / var_nj1);
        let assign30680_e45846: f64 = (var_nj1 - var_nj0);
        let assign30680_e45847: f64 = (var_vha1 * assign30680_e45846);
        let assign30680_e45850: f64 = (var_nj0 * p.p85);
        let assign30680_e45851: f64 = (assign30680_e45847 / assign30680_e45850);
        let assign30680_e45852: f64 = (assign30680_e45842 + assign30680_e45851);
        let assign30680_e45853: f64 = (var_phitdinv * assign30680_e45852);
        let assign30680_e45855: f64 = (assign30680_e45853 - 230.25850929940458);
        let assign30680_e45861: f64 = (var_vmax / var_nj1);
        let assign30680_e45865: f64 = (var_nj1 - var_nj0);
        let assign30680_e45866: f64 = (var_vha1 * assign30680_e45865);
        let assign30680_e45869: f64 = (var_nj0 * p.p85);
        let assign30680_e45870: f64 = (assign30680_e45866 / assign30680_e45869);
        let assign30680_e45871: f64 = (assign30680_e45861 + assign30680_e45870);
        let assign30680_e45872: f64 = (var_phitdinv * assign30680_e45871);
        let assign30680_e45874: f64 = (assign30680_e45872 - 230.25850929940458);
        let assign30680_e45879: f64 = (var_vmax / var_nj1);
        let assign30680_e45883: f64 = (var_nj1 - var_nj0);
        let assign30680_e45884: f64 = (var_vha1 * assign30680_e45883);
        let assign30680_e45887: f64 = (var_nj0 * p.p85);
        let assign30680_e45888: f64 = (assign30680_e45884 / assign30680_e45887);
        let assign30680_e45889: f64 = (assign30680_e45879 + assign30680_e45888);
        let assign30680_e45890: f64 = (var_phitdinv * assign30680_e45889);
        let assign30680_e45892: f64 = (assign30680_e45890 - 230.25850929940458);
        let assign30680_e45894: f64 = (assign30680_e45892 * 0.3333333333333333);
        let assign30680_e45895: f64 = (1.0 + assign30680_e45894);
        let assign30680_e45896: f64 = (assign30680_e45874 * assign30680_e45895);
        let assign30680_e45897: f64 = (0.5 * assign30680_e45896);
        let assign30680_e45898: f64 = (1.0 + assign30680_e45897);
        let assign30680_e45899: f64 = (assign30680_e45855 * assign30680_e45898);
        let assign30680_e45900: f64 = (1.0 + assign30680_e45899);
        let assign30680_e45901: f64 = (1e100 * assign30680_e45900);
        (assign30680_e45901, (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_dn0 * p.p85))) / (assign30680_e45850 * assign30680_e45850)))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_dn0 * p.p85))) / (assign30680_e45869 * assign30680_e45869)))) * assign30680_e45895) + (assign30680_e45874 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn0) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn0 - var_nj0_dn0)) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_dn0 * p.p85))) / (assign30680_e45887 * assign30680_e45887)))) * 0.3333333333333333))))))), (1e100 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign30680_e45850) - (assign30680_e45847 * (var_nj0_dn2 * p.p85))) / (assign30680_e45850 * assign30680_e45850)))) * assign30680_e45898) + (assign30680_e45855 * (0.5 * (((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign30680_e45869) - (assign30680_e45866 * (var_nj0_dn2 * p.p85))) / (assign30680_e45869 * assign30680_e45869)))) * assign30680_e45895) + (assign30680_e45874 * ((var_phitdinv * ((-((var_vmax * var_nj1_dn2) / (var_nj1 * var_nj1))) + ((((var_vha1 * (var_nj1_dn2 - var_nj0_dn2)) * assign30680_e45887) - (assign30680_e45884 * (var_nj0_dn2 * p.p85))) / (assign30680_e45887 * assign30680_e45887)))) * 0.3333333333333333))))))),)
    } else {
        (var_exp_vmax_over_phitd_bot, var_exp_vmax_over_phitd_bot_dn0, var_exp_vmax_over_phitd_bot_dn2,)
    }
};
        var_exp_vmax_over_phitd_bot = assign30680_e45903;
        var_exp_vmax_over_phitd_bot_dn0 = assign30680_e45903_d_n0;
        var_exp_vmax_over_phitd_bot_dn2 = assign30680_e45903_d_n2;
        var_exp_vmax_over_phitd_bot_rv = 0.0;

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
        var_dvmax_over_phitd_dv_rv = 0.0;

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
        var_idmultbot_rv = 0.0;

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
        var_pnn0_rv = 0.0;

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
        var_vha1_rv = 0.0;

        let assign30730_e45983: f64 = if var_nfasti_i < p.p85 { 1.0 } else { 0.0 };
        var_guard495 = assign30730_e45983;
        var_guard495_rv = 0.0;

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
        var_nja10_rv = 0.0;

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
        var_nj0_rv = 0.0;

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
        var_tmf1_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_dfn_su_rv = 0.0;

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
        var_nja11_rv = 0.0;

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
        var_tmf1_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_dfn_sl_rv = 0.0;

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
        *var_guard493_slot = var_guard493;
        *var_guard493_rv_slot = var_guard493_rv;
        *var_guard494_slot = var_guard494;
        *var_guard494_rv_slot = var_guard494_rv;
        *var_guard495_slot = var_guard495;
        *var_guard495_rv_slot = var_guard495_rv;
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

    pub(super) fn stamp_reactive_block_35(
        p: &Parameters,
        var_guard471: f64,
        var_guard479: f64,
        var_guard480: f64,
        var_guard495: f64,
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
        var_guard498_slot: &mut f64,
        var_guard498_rv_slot: &mut f64,
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
        let mut var_guard498: f64 = *var_guard498_slot;
        let mut var_guard498_rv: f64 = *var_guard498_rv_slot;
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
        var_nj1_rv = 0.0;

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
        var_tmf1_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_nj0_rv = 0.0;

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
        var_tmf1_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_nj0_rv = 0.0;

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
        var_dnj1_dv_rv = 0.0;

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
        var_nj0_rv = 0.0;

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
        var_nj1_rv = 0.0;

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
        var_dnj1_dv_rv = 0.0;

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
        var_dvmax_over_phitd_dv_rv = 0.0;

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
        var_pnn0_rv = 0.0;

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
        var_vha1_rv = 0.0;

        let assign31110_e46753: f64 = if var_nfagat_i < p.p85 { 1.0 } else { 0.0 };
        var_guard498 = assign31110_e46753;
        var_guard498_rv = 0.0;

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
        var_nja10_rv = 0.0;

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
        var_nj0_rv = 0.0;

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
        var_tmf1_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_dfn_su_rv = 0.0;

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
        var_nja11_rv = 0.0;

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
        var_tmf1_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_dfn_sl_rv = 0.0;

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
        var_nj1_rv = 0.0;

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
        var_tmf1_rv = 0.0;

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
        *var_guard498_slot = var_guard498;
        *var_guard498_rv_slot = var_guard498_rv;
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

    pub(super) fn stamp_reactive_block_36(
        p: &Parameters,
        var_dfn_sl: f64,
        var_dfn_sl_dn0: f64,
        var_dfn_sl_dn2: f64,
        var_dfn_su: f64,
        var_dfn_su_dn0: f64,
        var_dfn_su_dn2: f64,
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
        var_dnj1_dv_slot: &mut f64,
        var_dnj1_dv_dn0_slot: &mut f64,
        var_dnj1_dv_dn2_slot: &mut f64,
        var_dnj1_dv_rv_slot: &mut f64,
        var_dvmax_over_phitd_dv_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn0_slot: &mut f64,
        var_dvmax_over_phitd_dv_dn2_slot: &mut f64,
        var_dvmax_over_phitd_dv_rv_slot: &mut f64,
        var_guard558_slot: &mut f64,
        var_guard558_rv_slot: &mut f64,
        var_guard559_slot: &mut f64,
        var_guard559_rv_slot: &mut f64,
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
        var_nj_k_slot: &mut f64,
        var_nj_k0_slot: &mut f64,
        var_nj_k0_dn0_slot: &mut f64,
        var_nj_k0_dn2_slot: &mut f64,
        var_nj_k0_rv_slot: &mut f64,
        var_nj_k1_slot: &mut f64,
        var_nj_k1_dn0_slot: &mut f64,
        var_nj_k1_dn2_slot: &mut f64,
        var_nj_k1_rv_slot: &mut f64,
        var_nj_k_dn0_slot: &mut f64,
        var_nj_k_dn2_slot: &mut f64,
        var_nj_k_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_dnj1_dv: f64 = *var_dnj1_dv_slot;
        let mut var_dnj1_dv_dn0: f64 = *var_dnj1_dv_dn0_slot;
        let mut var_dnj1_dv_dn2: f64 = *var_dnj1_dv_dn2_slot;
        let mut var_dnj1_dv_rv: f64 = *var_dnj1_dv_rv_slot;
        let mut var_dvmax_over_phitd_dv: f64 = *var_dvmax_over_phitd_dv_slot;
        let mut var_dvmax_over_phitd_dv_dn0: f64 = *var_dvmax_over_phitd_dv_dn0_slot;
        let mut var_dvmax_over_phitd_dv_dn2: f64 = *var_dvmax_over_phitd_dv_dn2_slot;
        let mut var_dvmax_over_phitd_dv_rv: f64 = *var_dvmax_over_phitd_dv_rv_slot;
        let mut var_guard558: f64 = *var_guard558_slot;
        let mut var_guard558_rv: f64 = *var_guard558_rv_slot;
        let mut var_guard559: f64 = *var_guard559_slot;
        let mut var_guard559_rv: f64 = *var_guard559_rv_slot;
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
        let mut var_nj_k: f64 = *var_nj_k_slot;
        let mut var_nj_k0: f64 = *var_nj_k0_slot;
        let mut var_nj_k0_dn0: f64 = *var_nj_k0_dn0_slot;
        let mut var_nj_k0_dn2: f64 = *var_nj_k0_dn2_slot;
        let mut var_nj_k0_rv: f64 = *var_nj_k0_rv_slot;
        let mut var_nj_k1: f64 = *var_nj_k1_slot;
        let mut var_nj_k1_dn0: f64 = *var_nj_k1_dn0_slot;
        let mut var_nj_k1_dn2: f64 = *var_nj_k1_dn2_slot;
        let mut var_nj_k1_rv: f64 = *var_nj_k1_rv_slot;
        let mut var_nj_k_dn0: f64 = *var_nj_k_dn0_slot;
        let mut var_nj_k_dn2: f64 = *var_nj_k_dn2_slot;
        let mut var_nj_k_rv: f64 = *var_nj_k_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_nj0_rv = 0.0;

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
        var_tmf1_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_nj0_rv = 0.0;

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
        var_dnj1_dv_rv = 0.0;

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
        var_nj0_rv = 0.0;

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
        var_nj1_rv = 0.0;

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
        var_dnj1_dv_rv = 0.0;

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
        var_dvmax_over_phitd_dv_rv = 0.0;

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
        var_idmultbot_rv = 0.0;

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
        var_idmultbot_rv = 0.0;

        let assign34170_e51465: f64 = if p.p84 > 0.0 { 1.0 } else { 0.0 };
        var_guard558 = assign34170_e51465;
        var_guard558_rv = 0.0;

        let assign34180_e51468: f64 = if var_njl < p.p85 { 1.0 } else { 0.0 };
        var_guard559 = assign34180_e51468;
        var_guard559_rv = 0.0;

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
        var_nj_k0_rv = 0.0;

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
        var_nj0_rv = 0.0;

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
        var_tmf1_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_nj_k1_rv = 0.0;

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
        var_tmf1_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_nj_k_rv = 0.0;

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
        var_tmf1_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_nj0_rv = 0.0;

        *var_dnj1_dv_slot = var_dnj1_dv;
        *var_dnj1_dv_dn0_slot = var_dnj1_dv_dn0;
        *var_dnj1_dv_dn2_slot = var_dnj1_dv_dn2;
        *var_dnj1_dv_rv_slot = var_dnj1_dv_rv;
        *var_dvmax_over_phitd_dv_slot = var_dvmax_over_phitd_dv;
        *var_dvmax_over_phitd_dv_dn0_slot = var_dvmax_over_phitd_dv_dn0;
        *var_dvmax_over_phitd_dv_dn2_slot = var_dvmax_over_phitd_dv_dn2;
        *var_dvmax_over_phitd_dv_rv_slot = var_dvmax_over_phitd_dv_rv;
        *var_guard558_slot = var_guard558;
        *var_guard558_rv_slot = var_guard558_rv;
        *var_guard559_slot = var_guard559;
        *var_guard559_rv_slot = var_guard559_rv;
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
        *var_nj_k_slot = var_nj_k;
        *var_nj_k0_slot = var_nj_k0;
        *var_nj_k0_dn0_slot = var_nj_k0_dn0;
        *var_nj_k0_dn2_slot = var_nj_k0_dn2;
        *var_nj_k0_rv_slot = var_nj_k0_rv;
        *var_nj_k1_slot = var_nj_k1;
        *var_nj_k1_dn0_slot = var_nj_k1_dn0;
        *var_nj_k1_dn2_slot = var_nj_k1_dn2;
        *var_nj_k1_rv_slot = var_nj_k1_rv;
        *var_nj_k_dn0_slot = var_nj_k_dn0;
        *var_nj_k_dn2_slot = var_nj_k_dn2;
        *var_nj_k_rv_slot = var_nj_k_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_37(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_ab_i: f64,
        var_guard558: f64,
        var_guard559: f64,
        var_idmultbot: f64,
        var_idmultbot_dn0: f64,
        var_idmultbot_dn2: f64,
        var_njl: f64,
        var_phitdinv: f64,
        var_pn0: f64,
        var_q_pex0: f64,
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
        var_exp_a2_rv_slot: &mut f64,
        var_exp_a_dn0_slot: &mut f64,
        var_exp_a_dn2_slot: &mut f64,
        var_exp_a_rv_slot: &mut f64,
        var_exp_k_slot: &mut f64,
        var_exp_k2_slot: &mut f64,
        var_exp_k2_dn0_slot: &mut f64,
        var_exp_k2_dn2_slot: &mut f64,
        var_exp_k2_rv_slot: &mut f64,
        var_exp_k_dn0_slot: &mut f64,
        var_exp_k_dn2_slot: &mut f64,
        var_exp_k_rv_slot: &mut f64,
        var_guard560_slot: &mut f64,
        var_guard560_rv_slot: &mut f64,
        var_guard561_slot: &mut f64,
        var_guard561_rv_slot: &mut f64,
        var_guard562_slot: &mut f64,
        var_guard562_rv_slot: &mut f64,
        var_guard563_slot: &mut f64,
        var_guard563_rv_slot: &mut f64,
        var_guard564_slot: &mut f64,
        var_guard564_rv_slot: &mut f64,
        var_guard565_slot: &mut f64,
        var_guard565_rv_slot: &mut f64,
        var_inqs0_a_slot: &mut f64,
        var_inqs0_a_dn0_slot: &mut f64,
        var_inqs0_a_dn2_slot: &mut f64,
        var_inqs0_a_dn3_slot: &mut f64,
        var_inqs0_a_rv_slot: &mut f64,
        var_nj0_slot: &mut f64,
        var_nj0_dn0_slot: &mut f64,
        var_nj0_dn2_slot: &mut f64,
        var_nj0_rv_slot: &mut f64,
        var_nj_k_slot: &mut f64,
        var_nj_k_dn0_slot: &mut f64,
        var_nj_k_dn2_slot: &mut f64,
        var_nj_k_rv_slot: &mut f64,
        var_p_na_slot: &mut f64,
        var_p_na_dn0_slot: &mut f64,
        var_p_na_dn2_slot: &mut f64,
        var_p_na_rv_slot: &mut f64,
        var_q_nqs_a_slot: &mut f64,
        var_q_nqs_a_dn3_slot: &mut f64,
        var_q_nqs_a_rv_slot: &mut f64,
        var_q_pexa_slot: &mut f64,
        var_q_pexa_dn0_slot: &mut f64,
        var_q_pexa_dn2_slot: &mut f64,
        var_q_pexa_rv_slot: &mut f64,
        var_q_qs_a_slot: &mut f64,
        var_q_qs_a_dn0_slot: &mut f64,
        var_q_qs_a_dn2_slot: &mut f64,
        var_q_qs_a_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let mut var_exp_a: f64 = *var_exp_a_slot;
        let mut var_exp_a2: f64 = *var_exp_a2_slot;
        let mut var_exp_a2_dn0: f64 = *var_exp_a2_dn0_slot;
        let mut var_exp_a2_dn2: f64 = *var_exp_a2_dn2_slot;
        let mut var_exp_a2_rv: f64 = *var_exp_a2_rv_slot;
        let mut var_exp_a_dn0: f64 = *var_exp_a_dn0_slot;
        let mut var_exp_a_dn2: f64 = *var_exp_a_dn2_slot;
        let mut var_exp_a_rv: f64 = *var_exp_a_rv_slot;
        let mut var_exp_k: f64 = *var_exp_k_slot;
        let mut var_exp_k2: f64 = *var_exp_k2_slot;
        let mut var_exp_k2_dn0: f64 = *var_exp_k2_dn0_slot;
        let mut var_exp_k2_dn2: f64 = *var_exp_k2_dn2_slot;
        let mut var_exp_k2_rv: f64 = *var_exp_k2_rv_slot;
        let mut var_exp_k_dn0: f64 = *var_exp_k_dn0_slot;
        let mut var_exp_k_dn2: f64 = *var_exp_k_dn2_slot;
        let mut var_exp_k_rv: f64 = *var_exp_k_rv_slot;
        let mut var_guard560: f64 = *var_guard560_slot;
        let mut var_guard560_rv: f64 = *var_guard560_rv_slot;
        let mut var_guard561: f64 = *var_guard561_slot;
        let mut var_guard561_rv: f64 = *var_guard561_rv_slot;
        let mut var_guard562: f64 = *var_guard562_slot;
        let mut var_guard562_rv: f64 = *var_guard562_rv_slot;
        let mut var_guard563: f64 = *var_guard563_slot;
        let mut var_guard563_rv: f64 = *var_guard563_rv_slot;
        let mut var_guard564: f64 = *var_guard564_slot;
        let mut var_guard564_rv: f64 = *var_guard564_rv_slot;
        let mut var_guard565: f64 = *var_guard565_slot;
        let mut var_guard565_rv: f64 = *var_guard565_rv_slot;
        let mut var_inqs0_a: f64 = *var_inqs0_a_slot;
        let mut var_inqs0_a_dn0: f64 = *var_inqs0_a_dn0_slot;
        let mut var_inqs0_a_dn2: f64 = *var_inqs0_a_dn2_slot;
        let mut var_inqs0_a_dn3: f64 = *var_inqs0_a_dn3_slot;
        let mut var_inqs0_a_rv: f64 = *var_inqs0_a_rv_slot;
        let mut var_nj0: f64 = *var_nj0_slot;
        let mut var_nj0_dn0: f64 = *var_nj0_dn0_slot;
        let mut var_nj0_dn2: f64 = *var_nj0_dn2_slot;
        let mut var_nj0_rv: f64 = *var_nj0_rv_slot;
        let mut var_nj_k: f64 = *var_nj_k_slot;
        let mut var_nj_k_dn0: f64 = *var_nj_k_dn0_slot;
        let mut var_nj_k_dn2: f64 = *var_nj_k_dn2_slot;
        let mut var_nj_k_rv: f64 = *var_nj_k_rv_slot;
        let mut var_p_na: f64 = *var_p_na_slot;
        let mut var_p_na_dn0: f64 = *var_p_na_dn0_slot;
        let mut var_p_na_dn2: f64 = *var_p_na_dn2_slot;
        let mut var_p_na_rv: f64 = *var_p_na_rv_slot;
        let mut var_q_nqs_a: f64 = *var_q_nqs_a_slot;
        let mut var_q_nqs_a_dn3: f64 = *var_q_nqs_a_dn3_slot;
        let mut var_q_nqs_a_rv: f64 = *var_q_nqs_a_rv_slot;
        let mut var_q_pexa: f64 = *var_q_pexa_slot;
        let mut var_q_pexa_dn0: f64 = *var_q_pexa_dn0_slot;
        let mut var_q_pexa_dn2: f64 = *var_q_pexa_dn2_slot;
        let mut var_q_pexa_rv: f64 = *var_q_pexa_rv_slot;
        let mut var_q_qs_a: f64 = *var_q_qs_a_slot;
        let mut var_q_qs_a_dn0: f64 = *var_q_qs_a_dn0_slot;
        let mut var_q_qs_a_dn2: f64 = *var_q_qs_a_dn2_slot;
        let mut var_q_qs_a_rv: f64 = *var_q_qs_a_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

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
        var_tmf1_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_nj0_rv = 0.0;

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
        var_nj_k_rv = 0.0;

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
        var_nj0_rv = 0.0;

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
        var_exp_a_rv = 0.0;

        let assign34440_e51732: f64 = (var_v_hk - var_v_ha);
        let assign34440_e51733: f64 = (var_vak - assign34440_e51732);
        let assign34440_e51735: f64 = if assign34440_e51733 > 0.0 { 1.0 } else { 0.0 };
        var_guard560 = assign34440_e51735;
        var_guard560_rv = 0.0;

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
        var_guard561_rv = 0.0;

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
        var_exp_k_rv = 0.0;

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
        var_guard562_rv = 0.0;

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
        var_exp_k_rv = 0.0;

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
        var_exp_k_rv = 0.0;

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
        var_exp_k_rv = 0.0;

        let assign34510_e52016: f64 = if ((p.p91 == 0.0) || (var_vak < var_v_ha)) { 1.0 } else { 0.0 };
        var_guard563 = assign34510_e52016;
        var_guard563_rv = 0.0;

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
        var_exp_a2_rv = 0.0;

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
        var_exp_a2_rv = 0.0;

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
        var_exp_a2_rv = 0.0;

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
        var_p_na_rv = 0.0;

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
        var_q_pexa_rv = 0.0;

        let assign34570_e52081: f64 = if p.p92 > 0.0 { 1.0 } else { 0.0 };
        var_guard564 = assign34570_e52081;
        var_guard564_rv = 0.0;

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
        var_q_qs_a_rv = 0.0;

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
        var_q_nqs_a_rv = 0.0;

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
        var_inqs0_a_rv = 0.0;

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
        var_q_qs_a_rv = 0.0;

        let assign34640_e52140: f64 = if ((p.p91 == 0.0) || (var_vak < var_v_hk)) { 1.0 } else { 0.0 };
        var_guard565 = assign34640_e52140;
        var_guard565_rv = 0.0;

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
        var_exp_k2_rv = 0.0;

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
        var_exp_k2_rv = 0.0;

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
        var_exp_k2_rv = 0.0;

        *var_exp_a_slot = var_exp_a;
        *var_exp_a2_slot = var_exp_a2;
        *var_exp_a2_dn0_slot = var_exp_a2_dn0;
        *var_exp_a2_dn2_slot = var_exp_a2_dn2;
        *var_exp_a2_rv_slot = var_exp_a2_rv;
        *var_exp_a_dn0_slot = var_exp_a_dn0;
        *var_exp_a_dn2_slot = var_exp_a_dn2;
        *var_exp_a_rv_slot = var_exp_a_rv;
        *var_exp_k_slot = var_exp_k;
        *var_exp_k2_slot = var_exp_k2;
        *var_exp_k2_dn0_slot = var_exp_k2_dn0;
        *var_exp_k2_dn2_slot = var_exp_k2_dn2;
        *var_exp_k2_rv_slot = var_exp_k2_rv;
        *var_exp_k_dn0_slot = var_exp_k_dn0;
        *var_exp_k_dn2_slot = var_exp_k_dn2;
        *var_exp_k_rv_slot = var_exp_k_rv;
        *var_guard560_slot = var_guard560;
        *var_guard560_rv_slot = var_guard560_rv;
        *var_guard561_slot = var_guard561;
        *var_guard561_rv_slot = var_guard561_rv;
        *var_guard562_slot = var_guard562;
        *var_guard562_rv_slot = var_guard562_rv;
        *var_guard563_slot = var_guard563;
        *var_guard563_rv_slot = var_guard563_rv;
        *var_guard564_slot = var_guard564;
        *var_guard564_rv_slot = var_guard564_rv;
        *var_guard565_slot = var_guard565;
        *var_guard565_rv_slot = var_guard565_rv;
        *var_inqs0_a_slot = var_inqs0_a;
        *var_inqs0_a_dn0_slot = var_inqs0_a_dn0;
        *var_inqs0_a_dn2_slot = var_inqs0_a_dn2;
        *var_inqs0_a_dn3_slot = var_inqs0_a_dn3;
        *var_inqs0_a_rv_slot = var_inqs0_a_rv;
        *var_nj0_slot = var_nj0;
        *var_nj0_dn0_slot = var_nj0_dn0;
        *var_nj0_dn2_slot = var_nj0_dn2;
        *var_nj0_rv_slot = var_nj0_rv;
        *var_nj_k_slot = var_nj_k;
        *var_nj_k_dn0_slot = var_nj_k_dn0;
        *var_nj_k_dn2_slot = var_nj_k_dn2;
        *var_nj_k_rv_slot = var_nj_k_rv;
        *var_p_na_slot = var_p_na;
        *var_p_na_dn0_slot = var_p_na_dn0;
        *var_p_na_dn2_slot = var_p_na_dn2;
        *var_p_na_rv_slot = var_p_na_rv;
        *var_q_nqs_a_slot = var_q_nqs_a;
        *var_q_nqs_a_dn3_slot = var_q_nqs_a_dn3;
        *var_q_nqs_a_rv_slot = var_q_nqs_a_rv;
        *var_q_pexa_slot = var_q_pexa;
        *var_q_pexa_dn0_slot = var_q_pexa_dn0;
        *var_q_pexa_dn2_slot = var_q_pexa_dn2;
        *var_q_pexa_rv_slot = var_q_pexa_rv;
        *var_q_qs_a_slot = var_q_qs_a;
        *var_q_qs_a_dn0_slot = var_q_qs_a_dn0;
        *var_q_qs_a_dn2_slot = var_q_qs_a_dn2;
        *var_q_qs_a_rv_slot = var_q_qs_a_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_38(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_ab_i: f64,
        var_epssi: f64,
        var_exp_k2: f64,
        var_exp_k2_dn0: f64,
        var_exp_k2_dn2: f64,
        var_guard558: f64,
        var_juncdlt: f64,
        var_ndi_i: f64,
        var_pb: f64,
        var_pn0: f64,
        var_q_pex0: f64,
        var_vak: f64,
        var_vak_dn0: f64,
        var_vak_dn2: f64,
        var_w_depa0: f64,
        var_w_depa0_dn0: f64,
        var_w_depa0_dn2: f64,
        var_guard566_slot: &mut f64,
        var_guard566_rv_slot: &mut f64,
        var_guard567_slot: &mut f64,
        var_guard567_rv_slot: &mut f64,
        var_guard568_slot: &mut f64,
        var_guard568_rv_slot: &mut f64,
        var_guard571_slot: &mut f64,
        var_guard571_rv_slot: &mut f64,
        var_guard572_slot: &mut f64,
        var_guard572_rv_slot: &mut f64,
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
        var_p_nk_slot: &mut f64,
        var_p_nk_dn0_slot: &mut f64,
        var_p_nk_dn2_slot: &mut f64,
        var_p_nk_rv_slot: &mut f64,
        var_q_nqs_k_slot: &mut f64,
        var_q_nqs_k_dn4_slot: &mut f64,
        var_q_nqs_k_rv_slot: &mut f64,
        var_q_pexk_slot: &mut f64,
        var_q_pexk_dn0_slot: &mut f64,
        var_q_pexk_dn2_slot: &mut f64,
        var_q_pexk_rv_slot: &mut f64,
        var_q_qs_k_slot: &mut f64,
        var_q_qs_k_dn0_slot: &mut f64,
        var_q_qs_k_dn2_slot: &mut f64,
        var_q_qs_k_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vjunc_a_slot: &mut f64,
        var_vjunc_a_dn0_slot: &mut f64,
        var_vjunc_a_dn2_slot: &mut f64,
        var_vjunc_a_rv_slot: &mut f64,
        var_w_depa_slot: &mut f64,
        var_w_depa_dn0_slot: &mut f64,
        var_w_depa_dn2_slot: &mut f64,
        var_w_depa_rv_slot: &mut f64,
        var_w_nqs_a_slot: &mut f64,
        var_w_nqs_a_dn5_slot: &mut f64,
        var_w_nqs_a_rv_slot: &mut f64,
        var_w_qs_a_slot: &mut f64,
        var_w_qs_a_dn0_slot: &mut f64,
        var_w_qs_a_dn2_slot: &mut f64,
        var_w_qs_a_rv_slot: &mut f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let mut var_guard566: f64 = *var_guard566_slot;
        let mut var_guard566_rv: f64 = *var_guard566_rv_slot;
        let mut var_guard567: f64 = *var_guard567_slot;
        let mut var_guard567_rv: f64 = *var_guard567_rv_slot;
        let mut var_guard568: f64 = *var_guard568_slot;
        let mut var_guard568_rv: f64 = *var_guard568_rv_slot;
        let mut var_guard571: f64 = *var_guard571_slot;
        let mut var_guard571_rv: f64 = *var_guard571_rv_slot;
        let mut var_guard572: f64 = *var_guard572_slot;
        let mut var_guard572_rv: f64 = *var_guard572_rv_slot;
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
        let mut var_p_nk: f64 = *var_p_nk_slot;
        let mut var_p_nk_dn0: f64 = *var_p_nk_dn0_slot;
        let mut var_p_nk_dn2: f64 = *var_p_nk_dn2_slot;
        let mut var_p_nk_rv: f64 = *var_p_nk_rv_slot;
        let mut var_q_nqs_k: f64 = *var_q_nqs_k_slot;
        let mut var_q_nqs_k_dn4: f64 = *var_q_nqs_k_dn4_slot;
        let mut var_q_nqs_k_rv: f64 = *var_q_nqs_k_rv_slot;
        let mut var_q_pexk: f64 = *var_q_pexk_slot;
        let mut var_q_pexk_dn0: f64 = *var_q_pexk_dn0_slot;
        let mut var_q_pexk_dn2: f64 = *var_q_pexk_dn2_slot;
        let mut var_q_pexk_rv: f64 = *var_q_pexk_rv_slot;
        let mut var_q_qs_k: f64 = *var_q_qs_k_slot;
        let mut var_q_qs_k_dn0: f64 = *var_q_qs_k_dn0_slot;
        let mut var_q_qs_k_dn2: f64 = *var_q_qs_k_dn2_slot;
        let mut var_q_qs_k_rv: f64 = *var_q_qs_k_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vjunc_a: f64 = *var_vjunc_a_slot;
        let mut var_vjunc_a_dn0: f64 = *var_vjunc_a_dn0_slot;
        let mut var_vjunc_a_dn2: f64 = *var_vjunc_a_dn2_slot;
        let mut var_vjunc_a_rv: f64 = *var_vjunc_a_rv_slot;
        let mut var_w_depa: f64 = *var_w_depa_slot;
        let mut var_w_depa_dn0: f64 = *var_w_depa_dn0_slot;
        let mut var_w_depa_dn2: f64 = *var_w_depa_dn2_slot;
        let mut var_w_depa_rv: f64 = *var_w_depa_rv_slot;
        let mut var_w_nqs_a: f64 = *var_w_nqs_a_slot;
        let mut var_w_nqs_a_dn5: f64 = *var_w_nqs_a_dn5_slot;
        let mut var_w_nqs_a_rv: f64 = *var_w_nqs_a_rv_slot;
        let mut var_w_qs_a: f64 = *var_w_qs_a_slot;
        let mut var_w_qs_a_dn0: f64 = *var_w_qs_a_dn0_slot;
        let mut var_w_qs_a_dn2: f64 = *var_w_qs_a_dn2_slot;
        let mut var_w_qs_a_rv: f64 = *var_w_qs_a_rv_slot;

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
        var_p_nk_rv = 0.0;

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
        var_q_pexk_rv = 0.0;

        let assign34700_e52205: f64 = if p.p92 > 0.0 { 1.0 } else { 0.0 };
        var_guard566 = assign34700_e52205;
        var_guard566_rv = 0.0;

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
        var_q_qs_k_rv = 0.0;

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
        var_q_nqs_k_rv = 0.0;

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
        var_inqs0_k_rv = 0.0;

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
        var_q_qs_k_rv = 0.0;

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
        var_vjunc_a_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_vjunc_a_rv = 0.0;

        let assign34800_e52287: f64 = if var_vjunc_a < 0.0 { 1.0 } else { 0.0 };
        var_guard567 = assign34800_e52287;
        var_guard567_rv = 0.0;

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
        var_vjunc_a_rv = 0.0;

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
        var_w_depa_rv = 0.0;

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
        var_tmf1_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_tmf2_rv = 0.0;

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
        var_w_depa_rv = 0.0;

        let assign34880_e52354: f64 = if p.p95 > 0.0 { 1.0 } else { 0.0 };
        var_guard568 = assign34880_e52354;
        var_guard568_rv = 0.0;

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
        var_w_qs_a_rv = 0.0;

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
        var_w_nqs_a_rv = 0.0;

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
        var_iwnqs0_a_rv = 0.0;

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
        var_w_qs_a_rv = 0.0;

        let assign35080_e52535: f64 = if ((p.p84 > 0.0) && (p.p92 > 0.0)) { 1.0 } else { 0.0 };
        var_guard571 = assign35080_e52535;
        var_guard571_rv = 0.0;

        let assign35090_e52542: f64 = if ((p.p84 > 0.0) && (p.p95 > 0.0)) { 1.0 } else { 0.0 };
        var_guard572 = assign35090_e52542;
        var_guard572_rv = 0.0;

        *var_guard566_slot = var_guard566;
        *var_guard566_rv_slot = var_guard566_rv;
        *var_guard567_slot = var_guard567;
        *var_guard567_rv_slot = var_guard567_rv;
        *var_guard568_slot = var_guard568;
        *var_guard568_rv_slot = var_guard568_rv;
        *var_guard571_slot = var_guard571;
        *var_guard571_rv_slot = var_guard571_rv;
        *var_guard572_slot = var_guard572;
        *var_guard572_rv_slot = var_guard572_rv;
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
        *var_p_nk_slot = var_p_nk;
        *var_p_nk_dn0_slot = var_p_nk_dn0;
        *var_p_nk_dn2_slot = var_p_nk_dn2;
        *var_p_nk_rv_slot = var_p_nk_rv;
        *var_q_nqs_k_slot = var_q_nqs_k;
        *var_q_nqs_k_dn4_slot = var_q_nqs_k_dn4;
        *var_q_nqs_k_rv_slot = var_q_nqs_k_rv;
        *var_q_pexk_slot = var_q_pexk;
        *var_q_pexk_dn0_slot = var_q_pexk_dn0;
        *var_q_pexk_dn2_slot = var_q_pexk_dn2;
        *var_q_pexk_rv_slot = var_q_pexk_rv;
        *var_q_qs_k_slot = var_q_qs_k;
        *var_q_qs_k_dn0_slot = var_q_qs_k_dn0;
        *var_q_qs_k_dn2_slot = var_q_qs_k_dn2;
        *var_q_qs_k_rv_slot = var_q_qs_k_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vjunc_a_slot = var_vjunc_a;
        *var_vjunc_a_dn0_slot = var_vjunc_a_dn0;
        *var_vjunc_a_dn2_slot = var_vjunc_a_dn2;
        *var_vjunc_a_rv_slot = var_vjunc_a_rv;
        *var_w_depa_slot = var_w_depa;
        *var_w_depa_dn0_slot = var_w_depa_dn0;
        *var_w_depa_dn2_slot = var_w_depa_dn2;
        *var_w_depa_rv_slot = var_w_depa_rv;
        *var_w_nqs_a_slot = var_w_nqs_a;
        *var_w_nqs_a_dn5_slot = var_w_nqs_a_dn5;
        *var_w_nqs_a_rv_slot = var_w_nqs_a_rv;
        *var_w_qs_a_slot = var_w_qs_a;
        *var_w_qs_a_dn0_slot = var_w_qs_a_dn0;
        *var_w_qs_a_dn2_slot = var_w_qs_a_dn2;
        *var_w_qs_a_rv_slot = var_w_qs_a_rv;
    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
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
        var_guard571: f64,
        var_guard572: f64,
        var_inqs0_a: f64,
        var_inqs0_a_dn0: f64,
        var_inqs0_a_dn2: f64,
        var_inqs0_a_dn3: f64,
        var_inqs0_k: f64,
        var_inqs0_k_dn0: f64,
        var_inqs0_k_dn2: f64,
        var_inqs0_k_dn4: f64,
        var_iwnqs0_a: f64,
        var_iwnqs0_a_dn0: f64,
        var_iwnqs0_a_dn2: f64,
        var_iwnqs0_a_dn5: f64,
        var_q_nqs_a: f64,
        var_q_nqs_a_dn3: f64,
        var_q_nqs_k: f64,
        var_q_nqs_k_dn4: f64,
        var_w_nqs_a: f64,
        var_w_nqs_a_dn5: f64,
    ) {
        let (eq7_e144, eq7_e144_d_n0, eq7_e144_d_n2, eq7_e144_d_n3,) = {
    if (var_guard571 != 0.0) {
        let eq7_e140: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, var_q_nqs_a);
        let eq7_e141: f64 = (var_inqs0_a + eq7_e140);
        let eq7_e141_d_n3: f64 = (var_inqs0_a_dn3 + (var_q_nqs_a_dn3 * ddt_scale));
        let eq7_e142: f64 = (1e-12 * eq7_e141);
        let eq7_e142_d_n0: f64 = (1e-12 * var_inqs0_a_dn0);
        let eq7_e142_d_n2: f64 = (1e-12 * var_inqs0_a_dn2);
        let eq7_e142_d_n3: f64 = (1e-12 * eq7_e141_d_n3);
        (eq7_e142, eq7_e142_d_n0, eq7_e142_d_n2, eq7_e142_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e144;
        stamper.stamp_current_node3_local(
            Some(3),
            None,
            multiplicity * (eq7_value),
            0,
            multiplicity * (eq7_e144_d_n0),
            2,
            multiplicity * (eq7_e144_d_n2),
            3,
            multiplicity * (eq7_e144_d_n3),
        );
        let (eq8_e153, eq8_e153_d_n0, eq8_e153_d_n2, eq8_e153_d_n4,) = {
    if (var_guard571 != 0.0) {
        let eq8_e149: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, var_q_nqs_k);
        let eq8_e150: f64 = (var_inqs0_k + eq8_e149);
        let eq8_e150_d_n4: f64 = (var_inqs0_k_dn4 + (var_q_nqs_k_dn4 * ddt_scale));
        let eq8_e151: f64 = (1e-12 * eq8_e150);
        let eq8_e151_d_n0: f64 = (1e-12 * var_inqs0_k_dn0);
        let eq8_e151_d_n2: f64 = (1e-12 * var_inqs0_k_dn2);
        let eq8_e151_d_n4: f64 = (1e-12 * eq8_e150_d_n4);
        (eq8_e151, eq8_e151_d_n0, eq8_e151_d_n2, eq8_e151_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e153;
        stamper.stamp_current_node3_local(
            Some(4),
            None,
            multiplicity * (eq8_value),
            0,
            multiplicity * (eq8_e153_d_n0),
            2,
            multiplicity * (eq8_e153_d_n2),
            4,
            multiplicity * (eq8_e153_d_n4),
        );
        let (eq11_e172, eq11_e172_d_n0, eq11_e172_d_n2, eq11_e172_d_n5,) = {
    if (var_guard572 != 0.0) {
        let eq11_e168: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_w_nqs_a);
        let eq11_e169: f64 = (var_iwnqs0_a + eq11_e168);
        let eq11_e169_d_n5: f64 = (var_iwnqs0_a_dn5 + (var_w_nqs_a_dn5 * ddt_scale));
        let eq11_e170: f64 = (1e-13 * eq11_e169);
        let eq11_e170_d_n0: f64 = (1e-13 * var_iwnqs0_a_dn0);
        let eq11_e170_d_n2: f64 = (1e-13 * var_iwnqs0_a_dn2);
        let eq11_e170_d_n5: f64 = (1e-13 * eq11_e169_d_n5);
        (eq11_e170, eq11_e170_d_n0, eq11_e170_d_n2, eq11_e170_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e172;
        stamper.stamp_current_node3_local(
            Some(5),
            None,
            multiplicity * (eq11_value),
            0,
            multiplicity * (eq11_e172_d_n0),
            2,
            multiplicity * (eq11_e172_d_n2),
            5,
            multiplicity * (eq11_e172_d_n5),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        var_guard571: f64,
        var_guard572: f64,
        var_inqs0_a: f64,
        var_inqs0_a_dn0: f64,
        var_inqs0_a_dn2: f64,
        var_inqs0_a_dn3: f64,
        var_inqs0_k: f64,
        var_inqs0_k_dn0: f64,
        var_inqs0_k_dn2: f64,
        var_inqs0_k_dn4: f64,
        var_iwnqs0_a: f64,
        var_iwnqs0_a_dn0: f64,
        var_iwnqs0_a_dn2: f64,
        var_iwnqs0_a_dn5: f64,
        var_q_nqs_a: f64,
        var_q_nqs_a_dn3: f64,
        var_q_nqs_k: f64,
        var_q_nqs_k_dn4: f64,
        var_w_nqs_a: f64,
        var_w_nqs_a_dn5: f64,
    ) {
        let (eq7_e144, eq7_e144_d_n0, eq7_e144_d_n2, eq7_e144_d_n3, eq7_e144_q, eq7_e144_q_d_n3,) = {
    if (var_guard571 != 0.0) {
        let eq7_e140_q: f64 = var_q_nqs_a;
        let eq7_e141: f64 = (var_inqs0_a + var_q_nqs_a);
        let eq7_e141_d_n3: f64 = (var_inqs0_a_dn3 + var_q_nqs_a_dn3);
        let eq7_e141_q: f64 = eq7_e140_q;
        let eq7_e142: f64 = (1e-12 * eq7_e141);
        let eq7_e142_d_n0: f64 = (1e-12 * var_inqs0_a_dn0);
        let eq7_e142_d_n2: f64 = (1e-12 * var_inqs0_a_dn2);
        let eq7_e142_d_n3: f64 = (1e-12 * eq7_e141_d_n3);
        let eq7_e142_q: f64 = (1e-12 * eq7_e141_q);
        let eq7_e142_q_d_n3: f64 = (1e-12 * var_q_nqs_a_dn3);
        (eq7_e142, eq7_e142_d_n0, eq7_e142_d_n2, eq7_e142_d_n3, eq7_e142_q, eq7_e142_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (eq7_e144_q_d_n3),
        );
        let (eq8_e153, eq8_e153_d_n0, eq8_e153_d_n2, eq8_e153_d_n4, eq8_e153_q, eq8_e153_q_d_n4,) = {
    if (var_guard571 != 0.0) {
        let eq8_e149_q: f64 = var_q_nqs_k;
        let eq8_e150: f64 = (var_inqs0_k + var_q_nqs_k);
        let eq8_e150_d_n4: f64 = (var_inqs0_k_dn4 + var_q_nqs_k_dn4);
        let eq8_e150_q: f64 = eq8_e149_q;
        let eq8_e151: f64 = (1e-12 * eq8_e150);
        let eq8_e151_d_n0: f64 = (1e-12 * var_inqs0_k_dn0);
        let eq8_e151_d_n2: f64 = (1e-12 * var_inqs0_k_dn2);
        let eq8_e151_d_n4: f64 = (1e-12 * eq8_e150_d_n4);
        let eq8_e151_q: f64 = (1e-12 * eq8_e150_q);
        let eq8_e151_q_d_n4: f64 = (1e-12 * var_q_nqs_k_dn4);
        (eq8_e151, eq8_e151_d_n0, eq8_e151_d_n2, eq8_e151_d_n4, eq8_e151_q, eq8_e151_q_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq8_e153_q_d_n4),
        );
        let (eq11_e172, eq11_e172_d_n0, eq11_e172_d_n2, eq11_e172_d_n5, eq11_e172_q, eq11_e172_q_d_n5,) = {
    if (var_guard572 != 0.0) {
        let eq11_e168_q: f64 = var_w_nqs_a;
        let eq11_e169: f64 = (var_iwnqs0_a + var_w_nqs_a);
        let eq11_e169_d_n5: f64 = (var_iwnqs0_a_dn5 + var_w_nqs_a_dn5);
        let eq11_e169_q: f64 = eq11_e168_q;
        let eq11_e170: f64 = (1e-13 * eq11_e169);
        let eq11_e170_d_n0: f64 = (1e-13 * var_iwnqs0_a_dn0);
        let eq11_e170_d_n2: f64 = (1e-13 * var_iwnqs0_a_dn2);
        let eq11_e170_d_n5: f64 = (1e-13 * eq11_e169_d_n5);
        let eq11_e170_q: f64 = (1e-13 * eq11_e169_q);
        let eq11_e170_q_d_n5: f64 = (1e-13 * var_w_nqs_a_dn5);
        (eq11_e170, eq11_e170_d_n0, eq11_e170_d_n2, eq11_e170_d_n5, eq11_e170_q, eq11_e170_q_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * (eq11_e172_q_d_n5),
        );
    }
}
