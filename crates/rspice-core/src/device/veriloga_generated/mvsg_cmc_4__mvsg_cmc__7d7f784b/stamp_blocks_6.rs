#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_96(
        var_fn445_calc_ig__fracin: f64,
        var_fn445_calc_ig__ijin: f64,
        var_fn445_calc_ig__kbdgatein: f64,
        var_fn445_calc_ig__ngf: f64,
        var_fn445_calc_ig__pbdgin: f64,
        var_fn445_calc_ig__pg_param1: f64,
        var_fn445_calc_ig__pg_paramin: f64,
        var_fn445_calc_ig__phitin: f64,
        var_fn445_calc_ig__phitin_dn4: f64,
        var_fn445_calc_ig__tfacdiodein: f64,
        var_fn445_calc_ig__tfacdiodein_dn4: f64,
        var_fn445_calc_ig__type: f64,
        var_fn445_calc_ig__vbdgin: f64,
        var_fn445_calc_ig__vgin: f64,
        var_fn445_calc_ig__vgin_dn5: f64,
        var_fn445_calc_ig__vgin_dn8: f64,
        var_fn445_calc_ig__vgsatin: f64,
        var_fn445_calc_ig__vjg: f64,
        var_fn445_calc_ig__w: f64,
        var_guard417: f64,
        var_guard439: f64,
        var_fn445_calc_ig__expbd1_slot: &mut f64,
        var_fn445_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn445_calc_ig__expbd1_dn5_slot: &mut f64,
        var_fn445_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn445_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn445_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__expbd2_slot: &mut f64,
        var_fn445_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn445_calc_ig__expbdarg1_slot: &mut f64,
        var_fn445_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn445_calc_ig__expbdarg1_dn5_slot: &mut f64,
        var_fn445_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn445_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn445_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__expbdarg2_slot: &mut f64,
        var_fn445_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn445_calc_ig__expifor_slot: &mut f64,
        var_fn445_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn445_calc_ig__expifor_dn5_slot: &mut f64,
        var_fn445_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn445_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn445_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn445_calc_ig__expifor_hinj_dn5_slot: &mut f64,
        var_fn445_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn445_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn445_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn445_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_dn5_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_hinj_dn5_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__expphib_slot: &mut f64,
        var_fn445_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn445_calc_ig__iginbd_slot: &mut f64,
        var_fn445_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn445_calc_ig__iginbd_dn5_slot: &mut f64,
        var_fn445_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn445_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn445_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__igindiode_slot: &mut f64,
        var_fn445_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn445_calc_ig__igindiode_dn5_slot: &mut f64,
        var_fn445_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn445_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn445_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn445_calc_ig__igindiode_hinj_dn5_slot: &mut f64,
        var_fn445_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn445_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn445_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn445_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn445_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn445_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn445_calc_ig__igindiode_nohinj_dn5_slot: &mut f64,
        var_fn445_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn445_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__isdiodeout_slot: &mut f64,
        var_fn445_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn445_calc_ig__t0_slot: &mut f64,
        var_fn445_calc_ig__t0_dn4_slot: &mut f64,
        var_guard446_slot: &mut f64,
        var_guard447_slot: &mut f64,
    ) {
        let mut var_fn445_calc_ig__expbd1: f64 = *var_fn445_calc_ig__expbd1_slot;
        let mut var_fn445_calc_ig__expbd1_dn4: f64 = *var_fn445_calc_ig__expbd1_dn4_slot;
        let mut var_fn445_calc_ig__expbd1_dn5: f64 = *var_fn445_calc_ig__expbd1_dn5_slot;
        let mut var_fn445_calc_ig__expbd1_dn8: f64 = *var_fn445_calc_ig__expbd1_dn8_slot;
        let mut var_fn445_calc_ig__expbd1_vgsat: f64 = *var_fn445_calc_ig__expbd1_vgsat_slot;
        let mut var_fn445_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn445_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__expbd2: f64 = *var_fn445_calc_ig__expbd2_slot;
        let mut var_fn445_calc_ig__expbd2_dn4: f64 = *var_fn445_calc_ig__expbd2_dn4_slot;
        let mut var_fn445_calc_ig__expbdarg1: f64 = *var_fn445_calc_ig__expbdarg1_slot;
        let mut var_fn445_calc_ig__expbdarg1_dn4: f64 = *var_fn445_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn445_calc_ig__expbdarg1_dn5: f64 = *var_fn445_calc_ig__expbdarg1_dn5_slot;
        let mut var_fn445_calc_ig__expbdarg1_dn8: f64 = *var_fn445_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn445_calc_ig__expbdarg1_vgsat: f64 = *var_fn445_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn445_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn445_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__expbdarg2: f64 = *var_fn445_calc_ig__expbdarg2_slot;
        let mut var_fn445_calc_ig__expbdarg2_dn4: f64 = *var_fn445_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn445_calc_ig__expifor: f64 = *var_fn445_calc_ig__expifor_slot;
        let mut var_fn445_calc_ig__expifor_dn4: f64 = *var_fn445_calc_ig__expifor_dn4_slot;
        let mut var_fn445_calc_ig__expifor_dn5: f64 = *var_fn445_calc_ig__expifor_dn5_slot;
        let mut var_fn445_calc_ig__expifor_dn8: f64 = *var_fn445_calc_ig__expifor_dn8_slot;
        let mut var_fn445_calc_ig__expifor_hinj: f64 = *var_fn445_calc_ig__expifor_hinj_slot;
        let mut var_fn445_calc_ig__expifor_hinj_dn4: f64 = *var_fn445_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn445_calc_ig__expifor_hinj_dn5: f64 = *var_fn445_calc_ig__expifor_hinj_dn5_slot;
        let mut var_fn445_calc_ig__expifor_hinj_dn8: f64 = *var_fn445_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn445_calc_ig__expifor_hinj_vgsat: f64 = *var_fn445_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn445_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn445_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn445_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn445_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn445_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__expiforarg: f64 = *var_fn445_calc_ig__expiforarg_slot;
        let mut var_fn445_calc_ig__expiforarg_dn4: f64 = *var_fn445_calc_ig__expiforarg_dn4_slot;
        let mut var_fn445_calc_ig__expiforarg_dn5: f64 = *var_fn445_calc_ig__expiforarg_dn5_slot;
        let mut var_fn445_calc_ig__expiforarg_dn8: f64 = *var_fn445_calc_ig__expiforarg_dn8_slot;
        let mut var_fn445_calc_ig__expiforarg_hinj: f64 = *var_fn445_calc_ig__expiforarg_hinj_slot;
        let mut var_fn445_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn445_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn445_calc_ig__expiforarg_hinj_dn5: f64 = *var_fn445_calc_ig__expiforarg_hinj_dn5_slot;
        let mut var_fn445_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn445_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn445_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn445_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn445_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn445_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn445_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn445_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn445_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__expphib: f64 = *var_fn445_calc_ig__expphib_slot;
        let mut var_fn445_calc_ig__expphib_dn4: f64 = *var_fn445_calc_ig__expphib_dn4_slot;
        let mut var_fn445_calc_ig__iginbd: f64 = *var_fn445_calc_ig__iginbd_slot;
        let mut var_fn445_calc_ig__iginbd_dn4: f64 = *var_fn445_calc_ig__iginbd_dn4_slot;
        let mut var_fn445_calc_ig__iginbd_dn5: f64 = *var_fn445_calc_ig__iginbd_dn5_slot;
        let mut var_fn445_calc_ig__iginbd_dn8: f64 = *var_fn445_calc_ig__iginbd_dn8_slot;
        let mut var_fn445_calc_ig__iginbd_vgsat: f64 = *var_fn445_calc_ig__iginbd_vgsat_slot;
        let mut var_fn445_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn445_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__igindiode: f64 = *var_fn445_calc_ig__igindiode_slot;
        let mut var_fn445_calc_ig__igindiode_dn4: f64 = *var_fn445_calc_ig__igindiode_dn4_slot;
        let mut var_fn445_calc_ig__igindiode_dn5: f64 = *var_fn445_calc_ig__igindiode_dn5_slot;
        let mut var_fn445_calc_ig__igindiode_dn8: f64 = *var_fn445_calc_ig__igindiode_dn8_slot;
        let mut var_fn445_calc_ig__igindiode_hinj: f64 = *var_fn445_calc_ig__igindiode_hinj_slot;
        let mut var_fn445_calc_ig__igindiode_hinj_dn4: f64 = *var_fn445_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn445_calc_ig__igindiode_hinj_dn5: f64 = *var_fn445_calc_ig__igindiode_hinj_dn5_slot;
        let mut var_fn445_calc_ig__igindiode_hinj_dn8: f64 = *var_fn445_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn445_calc_ig__igindiode_hinj_pre: f64 = *var_fn445_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn445_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn445_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn445_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn445_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn445_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn445_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__igindiode_nohinj: f64 = *var_fn445_calc_ig__igindiode_nohinj_slot;
        let mut var_fn445_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn445_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn445_calc_ig__igindiode_nohinj_dn5: f64 = *var_fn445_calc_ig__igindiode_nohinj_dn5_slot;
        let mut var_fn445_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn445_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn445_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn445_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__isdiodeout: f64 = *var_fn445_calc_ig__isdiodeout_slot;
        let mut var_fn445_calc_ig__isdiodeout_dn4: f64 = *var_fn445_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn445_calc_ig__t0: f64 = *var_fn445_calc_ig__t0_slot;
        let mut var_fn445_calc_ig__t0_dn4: f64 = *var_fn445_calc_ig__t0_dn4_slot;
        let mut var_guard446: f64 = *var_guard446_slot;
        let mut var_guard447: f64 = *var_guard447_slot;

        let (assign38710_e36043, assign38710_e36043_d_n4, assign38710_e36043_d_n5, assign38710_e36043_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__igindiode_nohinj, var_fn445_calc_ig__igindiode_nohinj_dn4, var_fn445_calc_ig__igindiode_nohinj_dn5, var_fn445_calc_ig__igindiode_nohinj_dn8,)
    }
};
        var_fn445_calc_ig__igindiode_nohinj = assign38710_e36043;
        var_fn445_calc_ig__igindiode_nohinj_dn4 = assign38710_e36043_d_n4;
        var_fn445_calc_ig__igindiode_nohinj_dn5 = assign38710_e36043_d_n5;
        var_fn445_calc_ig__igindiode_nohinj_dn8 = assign38710_e36043_d_n8;

        let (assign38720_e36049, assign38720_e36049_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expiforarg_hinj_vgsat, var_fn445_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__expiforarg_hinj_vgsat = assign38720_e36049;
        var_fn445_calc_ig__expiforarg_hinj_vgsat_dn4 = assign38720_e36049_d_n4;

        let (assign38730_e36055, assign38730_e36055_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expifor_hinj_vgsat, var_fn445_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__expifor_hinj_vgsat = assign38730_e36055;
        var_fn445_calc_ig__expifor_hinj_vgsat_dn4 = assign38730_e36055_d_n4;

        let (assign38740_e36061, assign38740_e36061_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__igindiode_hinj_vgsat, var_fn445_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__igindiode_hinj_vgsat = assign38740_e36061;
        var_fn445_calc_ig__igindiode_hinj_vgsat_dn4 = assign38740_e36061_d_n4;

        let (assign38750_e36067, assign38750_e36067_d_n4, assign38750_e36067_d_n5, assign38750_e36067_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expiforarg_hinj, var_fn445_calc_ig__expiforarg_hinj_dn4, var_fn445_calc_ig__expiforarg_hinj_dn5, var_fn445_calc_ig__expiforarg_hinj_dn8,)
    }
};
        var_fn445_calc_ig__expiforarg_hinj = assign38750_e36067;
        var_fn445_calc_ig__expiforarg_hinj_dn4 = assign38750_e36067_d_n4;
        var_fn445_calc_ig__expiforarg_hinj_dn5 = assign38750_e36067_d_n5;
        var_fn445_calc_ig__expiforarg_hinj_dn8 = assign38750_e36067_d_n8;

        let (assign38760_e36073, assign38760_e36073_d_n4, assign38760_e36073_d_n5, assign38760_e36073_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expifor_hinj, var_fn445_calc_ig__expifor_hinj_dn4, var_fn445_calc_ig__expifor_hinj_dn5, var_fn445_calc_ig__expifor_hinj_dn8,)
    }
};
        var_fn445_calc_ig__expifor_hinj = assign38760_e36073;
        var_fn445_calc_ig__expifor_hinj_dn4 = assign38760_e36073_d_n4;
        var_fn445_calc_ig__expifor_hinj_dn5 = assign38760_e36073_d_n5;
        var_fn445_calc_ig__expifor_hinj_dn8 = assign38760_e36073_d_n8;

        let (assign38770_e36079, assign38770_e36079_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__igindiode_hinj_pre, var_fn445_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn445_calc_ig__igindiode_hinj_pre = assign38770_e36079;
        var_fn445_calc_ig__igindiode_hinj_pre_dn4 = assign38770_e36079_d_n4;

        let (assign38780_e36085, assign38780_e36085_d_n4, assign38780_e36085_d_n5, assign38780_e36085_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__igindiode_hinj, var_fn445_calc_ig__igindiode_hinj_dn4, var_fn445_calc_ig__igindiode_hinj_dn5, var_fn445_calc_ig__igindiode_hinj_dn8,)
    }
};
        var_fn445_calc_ig__igindiode_hinj = assign38780_e36085;
        var_fn445_calc_ig__igindiode_hinj_dn4 = assign38780_e36085_d_n4;
        var_fn445_calc_ig__igindiode_hinj_dn5 = assign38780_e36085_d_n5;
        var_fn445_calc_ig__igindiode_hinj_dn8 = assign38780_e36085_d_n8;

        let (assign38790_e36096, assign38790_e36096_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38790_e36091: f64 = (var_fn445_calc_ig__pg_param1 / var_fn445_calc_ig__phitin);
        let assign38790_e36093: f64 = (-var_fn445_calc_ig__vjg);
        let assign38790_e36094: f64 = (assign38790_e36091 * assign38790_e36093);
        (assign38790_e36094, ((-((var_fn445_calc_ig__pg_param1 * var_fn445_calc_ig__phitin_dn4) / (var_fn445_calc_ig__phitin * var_fn445_calc_ig__phitin))) * assign38790_e36093),)
    } else {
        (var_fn445_calc_ig__expphib, var_fn445_calc_ig__expphib_dn4,)
    }
};
        var_fn445_calc_ig__expphib = assign38790_e36096;
        var_fn445_calc_ig__expphib_dn4 = assign38790_e36096_d_n4;

        let (assign38800_e36140, assign38800_e36140_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38800_e36106: f64 = (-50.0);
        let (assign38800_e36138, assign38800_e36138_d_n4,) = {
            if ((!(var_fn445_calc_ig__expphib > 50.0)) && (!(var_fn445_calc_ig__expphib < assign38800_e36106))) {
                let assign38800_e36111: f64 = (var_fn445_calc_ig__expphib).exp();
                (assign38800_e36111, (assign38800_e36111 * var_fn445_calc_ig__expphib_dn4),)
            } else {
                let assign38800_e36118: f64 = (-50.0);
                let (assign38800_e36137, assign38800_e36137_d_n4,) = {
                    if ((!(var_fn445_calc_ig__expphib > 50.0)) && (var_fn445_calc_ig__expphib < assign38800_e36118)) {
                        let assign38800_e36122: f64 = (-50.0);
                        let assign38800_e36123: f64 = (assign38800_e36122).exp();
                        (assign38800_e36123, 0.0,)
                    } else {
                        let (assign38800_e36136, assign38800_e36136_d_n4,) = {
                            if (var_fn445_calc_ig__expphib > 50.0) {
                                let assign38800_e36128: f64 = (50.0_f64).exp();
                                let assign38800_e36132: f64 = (var_fn445_calc_ig__expphib - 50.0);
                                let assign38800_e36133: f64 = (1.0 + assign38800_e36132);
                                let assign38800_e36134: f64 = (assign38800_e36128 * assign38800_e36133);
                                (assign38800_e36134, (assign38800_e36128 * var_fn445_calc_ig__expphib_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign38800_e36136, assign38800_e36136_d_n4,)
                    }
                };
                (assign38800_e36137, assign38800_e36137_d_n4,)
            }
        };
        (assign38800_e36138, assign38800_e36138_d_n4,)
    } else {
        (var_fn445_calc_ig__t0, var_fn445_calc_ig__t0_dn4,)
    }
};
        var_fn445_calc_ig__t0 = assign38800_e36140;
        var_fn445_calc_ig__t0_dn4 = assign38800_e36140_d_n4;

        let (assign38810_e36153, assign38810_e36153_d_n4, assign38810_e36153_d_n5, assign38810_e36153_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38810_e36146: f64 = (-var_fn445_calc_ig__vgin);
        let assign38810_e36148: f64 = (assign38810_e36146 - var_fn445_calc_ig__vbdgin);
        let assign38810_e36149: f64 = (var_fn445_calc_ig__pbdgin * assign38810_e36148);
        let assign38810_e36151: f64 = (assign38810_e36149 + var_fn445_calc_ig__expphib);
        (assign38810_e36151, var_fn445_calc_ig__expphib_dn4, (var_fn445_calc_ig__pbdgin * (-var_fn445_calc_ig__vgin_dn5)), (var_fn445_calc_ig__pbdgin * (-var_fn445_calc_ig__vgin_dn8)),)
    } else {
        (var_fn445_calc_ig__expbdarg1, var_fn445_calc_ig__expbdarg1_dn4, var_fn445_calc_ig__expbdarg1_dn5, var_fn445_calc_ig__expbdarg1_dn8,)
    }
};
        var_fn445_calc_ig__expbdarg1 = assign38810_e36153;
        var_fn445_calc_ig__expbdarg1_dn4 = assign38810_e36153_d_n4;
        var_fn445_calc_ig__expbdarg1_dn5 = assign38810_e36153_d_n5;
        var_fn445_calc_ig__expbdarg1_dn8 = assign38810_e36153_d_n8;

        let (assign38820_e36164, assign38820_e36164_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38820_e36158: f64 = (-var_fn445_calc_ig__pbdgin);
        let assign38820_e36160: f64 = (assign38820_e36158 * var_fn445_calc_ig__vbdgin);
        let assign38820_e36162: f64 = (assign38820_e36160 + var_fn445_calc_ig__expphib);
        (assign38820_e36162, var_fn445_calc_ig__expphib_dn4,)
    } else {
        (var_fn445_calc_ig__expbdarg2, var_fn445_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn445_calc_ig__expbdarg2 = assign38820_e36164;
        var_fn445_calc_ig__expbdarg2_dn4 = assign38820_e36164_d_n4;

        let (assign38830_e36208, assign38830_e36208_d_n4, assign38830_e36208_d_n5, assign38830_e36208_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38830_e36174: f64 = (-50.0);
        let (assign38830_e36206, assign38830_e36206_d_n4, assign38830_e36206_d_n5, assign38830_e36206_d_n8,) = {
            if ((!(var_fn445_calc_ig__expbdarg1 > 50.0)) && (!(var_fn445_calc_ig__expbdarg1 < assign38830_e36174))) {
                let assign38830_e36179: f64 = (var_fn445_calc_ig__expbdarg1).exp();
                (assign38830_e36179, (assign38830_e36179 * var_fn445_calc_ig__expbdarg1_dn4), (assign38830_e36179 * var_fn445_calc_ig__expbdarg1_dn5), (assign38830_e36179 * var_fn445_calc_ig__expbdarg1_dn8),)
            } else {
                let assign38830_e36186: f64 = (-50.0);
                let (assign38830_e36205, assign38830_e36205_d_n4, assign38830_e36205_d_n5, assign38830_e36205_d_n8,) = {
                    if ((!(var_fn445_calc_ig__expbdarg1 > 50.0)) && (var_fn445_calc_ig__expbdarg1 < assign38830_e36186)) {
                        let assign38830_e36190: f64 = (-50.0);
                        let assign38830_e36191: f64 = (assign38830_e36190).exp();
                        (assign38830_e36191, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign38830_e36204, assign38830_e36204_d_n4, assign38830_e36204_d_n5, assign38830_e36204_d_n8,) = {
                            if (var_fn445_calc_ig__expbdarg1 > 50.0) {
                                let assign38830_e36196: f64 = (50.0_f64).exp();
                                let assign38830_e36200: f64 = (var_fn445_calc_ig__expbdarg1 - 50.0);
                                let assign38830_e36201: f64 = (1.0 + assign38830_e36200);
                                let assign38830_e36202: f64 = (assign38830_e36196 * assign38830_e36201);
                                (assign38830_e36202, (assign38830_e36196 * var_fn445_calc_ig__expbdarg1_dn4), (assign38830_e36196 * var_fn445_calc_ig__expbdarg1_dn5), (assign38830_e36196 * var_fn445_calc_ig__expbdarg1_dn8),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign38830_e36204, assign38830_e36204_d_n4, assign38830_e36204_d_n5, assign38830_e36204_d_n8,)
                    }
                };
                (assign38830_e36205, assign38830_e36205_d_n4, assign38830_e36205_d_n5, assign38830_e36205_d_n8,)
            }
        };
        (assign38830_e36206, assign38830_e36206_d_n4, assign38830_e36206_d_n5, assign38830_e36206_d_n8,)
    } else {
        (var_fn445_calc_ig__expbd1, var_fn445_calc_ig__expbd1_dn4, var_fn445_calc_ig__expbd1_dn5, var_fn445_calc_ig__expbd1_dn8,)
    }
};
        var_fn445_calc_ig__expbd1 = assign38830_e36208;
        var_fn445_calc_ig__expbd1_dn4 = assign38830_e36208_d_n4;
        var_fn445_calc_ig__expbd1_dn5 = assign38830_e36208_d_n5;
        var_fn445_calc_ig__expbd1_dn8 = assign38830_e36208_d_n8;

        let (assign38840_e36252, assign38840_e36252_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38840_e36218: f64 = (-50.0);
        let (assign38840_e36250, assign38840_e36250_d_n4,) = {
            if ((!(var_fn445_calc_ig__expbdarg2 > 50.0)) && (!(var_fn445_calc_ig__expbdarg2 < assign38840_e36218))) {
                let assign38840_e36223: f64 = (var_fn445_calc_ig__expbdarg2).exp();
                (assign38840_e36223, (assign38840_e36223 * var_fn445_calc_ig__expbdarg2_dn4),)
            } else {
                let assign38840_e36230: f64 = (-50.0);
                let (assign38840_e36249, assign38840_e36249_d_n4,) = {
                    if ((!(var_fn445_calc_ig__expbdarg2 > 50.0)) && (var_fn445_calc_ig__expbdarg2 < assign38840_e36230)) {
                        let assign38840_e36234: f64 = (-50.0);
                        let assign38840_e36235: f64 = (assign38840_e36234).exp();
                        (assign38840_e36235, 0.0,)
                    } else {
                        let (assign38840_e36248, assign38840_e36248_d_n4,) = {
                            if (var_fn445_calc_ig__expbdarg2 > 50.0) {
                                let assign38840_e36240: f64 = (50.0_f64).exp();
                                let assign38840_e36244: f64 = (var_fn445_calc_ig__expbdarg2 - 50.0);
                                let assign38840_e36245: f64 = (1.0 + assign38840_e36244);
                                let assign38840_e36246: f64 = (assign38840_e36240 * assign38840_e36245);
                                (assign38840_e36246, (assign38840_e36240 * var_fn445_calc_ig__expbdarg2_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign38840_e36248, assign38840_e36248_d_n4,)
                    }
                };
                (assign38840_e36249, assign38840_e36249_d_n4,)
            }
        };
        (assign38840_e36250, assign38840_e36250_d_n4,)
    } else {
        (var_fn445_calc_ig__expbd2, var_fn445_calc_ig__expbd2_dn4,)
    }
};
        var_fn445_calc_ig__expbd2 = assign38840_e36252;
        var_fn445_calc_ig__expbd2_dn4 = assign38840_e36252_d_n4;

        let (assign38850_e36260, assign38850_e36260_d_n4, assign38850_e36260_d_n5, assign38850_e36260_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38850_e36258: f64 = (var_fn445_calc_ig__expbd1 - var_fn445_calc_ig__expbd2);
        (assign38850_e36258, (var_fn445_calc_ig__expbd1_dn4 - var_fn445_calc_ig__expbd2_dn4), var_fn445_calc_ig__expbd1_dn5, var_fn445_calc_ig__expbd1_dn8,)
    } else {
        (var_fn445_calc_ig__iginbd, var_fn445_calc_ig__iginbd_dn4, var_fn445_calc_ig__iginbd_dn5, var_fn445_calc_ig__iginbd_dn8,)
    }
};
        var_fn445_calc_ig__iginbd = assign38850_e36260;
        var_fn445_calc_ig__iginbd_dn4 = assign38850_e36260_d_n4;
        var_fn445_calc_ig__iginbd_dn5 = assign38850_e36260_d_n5;
        var_fn445_calc_ig__iginbd_dn8 = assign38850_e36260_d_n8;

        let (assign38860_e36274, assign38860_e36274_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38860_e36266: f64 = (var_fn445_calc_ig__type * var_fn445_calc_ig__w);
        let assign38860_e36268: f64 = (assign38860_e36266 * var_fn445_calc_ig__ngf);
        let assign38860_e36270: f64 = (assign38860_e36268 * var_fn445_calc_ig__ijin);
        let assign38860_e36272: f64 = (assign38860_e36270 * var_fn445_calc_ig__tfacdiodein);
        (assign38860_e36272, (assign38860_e36270 * var_fn445_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn445_calc_ig__isdiodeout, var_fn445_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn445_calc_ig__isdiodeout = assign38860_e36274;
        var_fn445_calc_ig__isdiodeout_dn4 = assign38860_e36274_d_n4;

        let (assign38870_e36286, assign38870_e36286_d_n4, assign38870_e36286_d_n5, assign38870_e36286_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38870_e36280: f64 = (var_fn445_calc_ig__pg_paramin / var_fn445_calc_ig__phitin);
        let assign38870_e36282: f64 = (assign38870_e36280 * var_fn445_calc_ig__vgin);
        let assign38870_e36284: f64 = (assign38870_e36282 + var_fn445_calc_ig__expphib);
        (assign38870_e36284, (((-((var_fn445_calc_ig__pg_paramin * var_fn445_calc_ig__phitin_dn4) / (var_fn445_calc_ig__phitin * var_fn445_calc_ig__phitin))) * var_fn445_calc_ig__vgin) + var_fn445_calc_ig__expphib_dn4), (assign38870_e36280 * var_fn445_calc_ig__vgin_dn5), (assign38870_e36280 * var_fn445_calc_ig__vgin_dn8),)
    } else {
        (var_fn445_calc_ig__expiforarg, var_fn445_calc_ig__expiforarg_dn4, var_fn445_calc_ig__expiforarg_dn5, var_fn445_calc_ig__expiforarg_dn8,)
    }
};
        var_fn445_calc_ig__expiforarg = assign38870_e36286;
        var_fn445_calc_ig__expiforarg_dn4 = assign38870_e36286_d_n4;
        var_fn445_calc_ig__expiforarg_dn5 = assign38870_e36286_d_n5;
        var_fn445_calc_ig__expiforarg_dn8 = assign38870_e36286_d_n8;

        let (assign38880_e36330, assign38880_e36330_d_n4, assign38880_e36330_d_n5, assign38880_e36330_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38880_e36296: f64 = (-50.0);
        let (assign38880_e36328, assign38880_e36328_d_n4, assign38880_e36328_d_n5, assign38880_e36328_d_n8,) = {
            if ((!(var_fn445_calc_ig__expiforarg > 50.0)) && (!(var_fn445_calc_ig__expiforarg < assign38880_e36296))) {
                let assign38880_e36301: f64 = (var_fn445_calc_ig__expiforarg).exp();
                (assign38880_e36301, (assign38880_e36301 * var_fn445_calc_ig__expiforarg_dn4), (assign38880_e36301 * var_fn445_calc_ig__expiforarg_dn5), (assign38880_e36301 * var_fn445_calc_ig__expiforarg_dn8),)
            } else {
                let assign38880_e36308: f64 = (-50.0);
                let (assign38880_e36327, assign38880_e36327_d_n4, assign38880_e36327_d_n5, assign38880_e36327_d_n8,) = {
                    if ((!(var_fn445_calc_ig__expiforarg > 50.0)) && (var_fn445_calc_ig__expiforarg < assign38880_e36308)) {
                        let assign38880_e36312: f64 = (-50.0);
                        let assign38880_e36313: f64 = (assign38880_e36312).exp();
                        (assign38880_e36313, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign38880_e36326, assign38880_e36326_d_n4, assign38880_e36326_d_n5, assign38880_e36326_d_n8,) = {
                            if (var_fn445_calc_ig__expiforarg > 50.0) {
                                let assign38880_e36318: f64 = (50.0_f64).exp();
                                let assign38880_e36322: f64 = (var_fn445_calc_ig__expiforarg - 50.0);
                                let assign38880_e36323: f64 = (1.0 + assign38880_e36322);
                                let assign38880_e36324: f64 = (assign38880_e36318 * assign38880_e36323);
                                (assign38880_e36324, (assign38880_e36318 * var_fn445_calc_ig__expiforarg_dn4), (assign38880_e36318 * var_fn445_calc_ig__expiforarg_dn5), (assign38880_e36318 * var_fn445_calc_ig__expiforarg_dn8),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign38880_e36326, assign38880_e36326_d_n4, assign38880_e36326_d_n5, assign38880_e36326_d_n8,)
                    }
                };
                (assign38880_e36327, assign38880_e36327_d_n4, assign38880_e36327_d_n5, assign38880_e36327_d_n8,)
            }
        };
        (assign38880_e36328, assign38880_e36328_d_n4, assign38880_e36328_d_n5, assign38880_e36328_d_n8,)
    } else {
        (var_fn445_calc_ig__expifor, var_fn445_calc_ig__expifor_dn4, var_fn445_calc_ig__expifor_dn5, var_fn445_calc_ig__expifor_dn8,)
    }
};
        var_fn445_calc_ig__expifor = assign38880_e36330;
        var_fn445_calc_ig__expifor_dn4 = assign38880_e36330_d_n4;
        var_fn445_calc_ig__expifor_dn5 = assign38880_e36330_d_n5;
        var_fn445_calc_ig__expifor_dn8 = assign38880_e36330_d_n8;

        let assign38890_e36333: f64 = if var_fn445_calc_ig__fracin == 1.0 { 1.0 } else { 0.0 };
        var_guard446 = assign38890_e36333;

        let (assign38900_e36349, assign38900_e36349_d_n4, assign38900_e36349_d_n5, assign38900_e36349_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 != 0.0)) {
        let assign38900_e36343: f64 = (var_fn445_calc_ig__kbdgatein * var_fn445_calc_ig__iginbd);
        let assign38900_e36344: f64 = (var_fn445_calc_ig__expifor - assign38900_e36343);
        let assign38900_e36346: f64 = (assign38900_e36344 - var_fn445_calc_ig__t0);
        let assign38900_e36347: f64 = (var_fn445_calc_ig__isdiodeout * assign38900_e36346);
        (assign38900_e36347, ((var_fn445_calc_ig__isdiodeout_dn4 * assign38900_e36346) + (var_fn445_calc_ig__isdiodeout * ((var_fn445_calc_ig__expifor_dn4 - (var_fn445_calc_ig__kbdgatein * var_fn445_calc_ig__iginbd_dn4)) - var_fn445_calc_ig__t0_dn4))), (var_fn445_calc_ig__isdiodeout * (var_fn445_calc_ig__expifor_dn5 - (var_fn445_calc_ig__kbdgatein * var_fn445_calc_ig__iginbd_dn5))), (var_fn445_calc_ig__isdiodeout * (var_fn445_calc_ig__expifor_dn8 - (var_fn445_calc_ig__kbdgatein * var_fn445_calc_ig__iginbd_dn8))),)
    } else {
        (var_fn445_calc_ig__igindiode, var_fn445_calc_ig__igindiode_dn4, var_fn445_calc_ig__igindiode_dn5, var_fn445_calc_ig__igindiode_dn8,)
    }
};
        var_fn445_calc_ig__igindiode = assign38900_e36349;
        var_fn445_calc_ig__igindiode_dn4 = assign38900_e36349_d_n4;
        var_fn445_calc_ig__igindiode_dn5 = assign38900_e36349_d_n5;
        var_fn445_calc_ig__igindiode_dn8 = assign38900_e36349_d_n8;

        let (assign38910_e36365, assign38910_e36365_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) {
        let assign38910_e36358: f64 = (-var_fn445_calc_ig__vgsatin);
        let assign38910_e36360: f64 = (assign38910_e36358 - var_fn445_calc_ig__vbdgin);
        let assign38910_e36361: f64 = (var_fn445_calc_ig__pbdgin * assign38910_e36360);
        let assign38910_e36363: f64 = (assign38910_e36361 + var_fn445_calc_ig__expphib);
        (assign38910_e36363, var_fn445_calc_ig__expphib_dn4,)
    } else {
        (var_fn445_calc_ig__expbdarg1_vgsat, var_fn445_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__expbdarg1_vgsat = assign38910_e36365;
        var_fn445_calc_ig__expbdarg1_vgsat_dn4 = assign38910_e36365_d_n4;

        let (assign38920_e36412, assign38920_e36412_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) {
        let assign38920_e36378: f64 = (-50.0);
        let (assign38920_e36410, assign38920_e36410_d_n4,) = {
            if ((!(var_fn445_calc_ig__expbdarg1_vgsat > 50.0)) && (!(var_fn445_calc_ig__expbdarg1_vgsat < assign38920_e36378))) {
                let assign38920_e36383: f64 = (var_fn445_calc_ig__expbdarg1_vgsat).exp();
                (assign38920_e36383, (assign38920_e36383 * var_fn445_calc_ig__expbdarg1_vgsat_dn4),)
            } else {
                let assign38920_e36390: f64 = (-50.0);
                let (assign38920_e36409, assign38920_e36409_d_n4,) = {
                    if ((!(var_fn445_calc_ig__expbdarg1_vgsat > 50.0)) && (var_fn445_calc_ig__expbdarg1_vgsat < assign38920_e36390)) {
                        let assign38920_e36394: f64 = (-50.0);
                        let assign38920_e36395: f64 = (assign38920_e36394).exp();
                        (assign38920_e36395, 0.0,)
                    } else {
                        let (assign38920_e36408, assign38920_e36408_d_n4,) = {
                            if (var_fn445_calc_ig__expbdarg1_vgsat > 50.0) {
                                let assign38920_e36400: f64 = (50.0_f64).exp();
                                let assign38920_e36404: f64 = (var_fn445_calc_ig__expbdarg1_vgsat - 50.0);
                                let assign38920_e36405: f64 = (1.0 + assign38920_e36404);
                                let assign38920_e36406: f64 = (assign38920_e36400 * assign38920_e36405);
                                (assign38920_e36406, (assign38920_e36400 * var_fn445_calc_ig__expbdarg1_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign38920_e36408, assign38920_e36408_d_n4,)
                    }
                };
                (assign38920_e36409, assign38920_e36409_d_n4,)
            }
        };
        (assign38920_e36410, assign38920_e36410_d_n4,)
    } else {
        (var_fn445_calc_ig__expbd1_vgsat, var_fn445_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__expbd1_vgsat = assign38920_e36412;
        var_fn445_calc_ig__expbd1_vgsat_dn4 = assign38920_e36412_d_n4;

        let (assign38930_e36423, assign38930_e36423_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) {
        let assign38930_e36421: f64 = (var_fn445_calc_ig__expbd1_vgsat - var_fn445_calc_ig__expbd2);
        (assign38930_e36421, (var_fn445_calc_ig__expbd1_vgsat_dn4 - var_fn445_calc_ig__expbd2_dn4),)
    } else {
        (var_fn445_calc_ig__iginbd_vgsat, var_fn445_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__iginbd_vgsat = assign38930_e36423;
        var_fn445_calc_ig__iginbd_vgsat_dn4 = assign38930_e36423_d_n4;

        let (assign38940_e36438, assign38940_e36438_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) {
        let assign38940_e36432: f64 = (var_fn445_calc_ig__pg_paramin / var_fn445_calc_ig__phitin);
        let assign38940_e36434: f64 = (assign38940_e36432 * var_fn445_calc_ig__vgsatin);
        let assign38940_e36436: f64 = (assign38940_e36434 + var_fn445_calc_ig__expphib);
        (assign38940_e36436, (((-((var_fn445_calc_ig__pg_paramin * var_fn445_calc_ig__phitin_dn4) / (var_fn445_calc_ig__phitin * var_fn445_calc_ig__phitin))) * var_fn445_calc_ig__vgsatin) + var_fn445_calc_ig__expphib_dn4),)
    } else {
        (var_fn445_calc_ig__expiforarg_nohinj_vgsat, var_fn445_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__expiforarg_nohinj_vgsat = assign38940_e36438;
        var_fn445_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign38940_e36438_d_n4;

        let (assign38950_e36485, assign38950_e36485_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) {
        let assign38950_e36451: f64 = (-50.0);
        let (assign38950_e36483, assign38950_e36483_d_n4,) = {
            if ((!(var_fn445_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (!(var_fn445_calc_ig__expiforarg_nohinj_vgsat < assign38950_e36451))) {
                let assign38950_e36456: f64 = (var_fn445_calc_ig__expiforarg_nohinj_vgsat).exp();
                (assign38950_e36456, (assign38950_e36456 * var_fn445_calc_ig__expiforarg_nohinj_vgsat_dn4),)
            } else {
                let assign38950_e36463: f64 = (-50.0);
                let (assign38950_e36482, assign38950_e36482_d_n4,) = {
                    if ((!(var_fn445_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (var_fn445_calc_ig__expiforarg_nohinj_vgsat < assign38950_e36463)) {
                        let assign38950_e36467: f64 = (-50.0);
                        let assign38950_e36468: f64 = (assign38950_e36467).exp();
                        (assign38950_e36468, 0.0,)
                    } else {
                        let (assign38950_e36481, assign38950_e36481_d_n4,) = {
                            if (var_fn445_calc_ig__expiforarg_nohinj_vgsat > 50.0) {
                                let assign38950_e36473: f64 = (50.0_f64).exp();
                                let assign38950_e36477: f64 = (var_fn445_calc_ig__expiforarg_nohinj_vgsat - 50.0);
                                let assign38950_e36478: f64 = (1.0 + assign38950_e36477);
                                let assign38950_e36479: f64 = (assign38950_e36473 * assign38950_e36478);
                                (assign38950_e36479, (assign38950_e36473 * var_fn445_calc_ig__expiforarg_nohinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign38950_e36481, assign38950_e36481_d_n4,)
                    }
                };
                (assign38950_e36482, assign38950_e36482_d_n4,)
            }
        };
        (assign38950_e36483, assign38950_e36483_d_n4,)
    } else {
        (var_fn445_calc_ig__expifor_nohinj_vgsat, var_fn445_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__expifor_nohinj_vgsat = assign38950_e36485;
        var_fn445_calc_ig__expifor_nohinj_vgsat_dn4 = assign38950_e36485_d_n4;

        let (assign38960_e36500, assign38960_e36500_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) {
        let assign38960_e36495: f64 = (var_fn445_calc_ig__kbdgatein * var_fn445_calc_ig__iginbd_vgsat);
        let assign38960_e36496: f64 = (var_fn445_calc_ig__expifor_nohinj_vgsat - assign38960_e36495);
        let assign38960_e36498: f64 = (assign38960_e36496 - var_fn445_calc_ig__t0);
        (assign38960_e36498, ((var_fn445_calc_ig__expifor_nohinj_vgsat_dn4 - (var_fn445_calc_ig__kbdgatein * var_fn445_calc_ig__iginbd_vgsat_dn4)) - var_fn445_calc_ig__t0_dn4),)
    } else {
        (var_fn445_calc_ig__igindiode_nohinj_vgsat, var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__igindiode_nohinj_vgsat = assign38960_e36500;
        var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4 = assign38960_e36500_d_n4;

        let (assign38970_e36517, assign38970_e36517_d_n4, assign38970_e36517_d_n5, assign38970_e36517_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) {
        let assign38970_e36511: f64 = (var_fn445_calc_ig__kbdgatein * var_fn445_calc_ig__iginbd);
        let assign38970_e36512: f64 = (var_fn445_calc_ig__expifor - assign38970_e36511);
        let assign38970_e36514: f64 = (assign38970_e36512 - var_fn445_calc_ig__t0);
        let assign38970_e36515: f64 = (var_fn445_calc_ig__isdiodeout * assign38970_e36514);
        (assign38970_e36515, ((var_fn445_calc_ig__isdiodeout_dn4 * assign38970_e36514) + (var_fn445_calc_ig__isdiodeout * ((var_fn445_calc_ig__expifor_dn4 - (var_fn445_calc_ig__kbdgatein * var_fn445_calc_ig__iginbd_dn4)) - var_fn445_calc_ig__t0_dn4))), (var_fn445_calc_ig__isdiodeout * (var_fn445_calc_ig__expifor_dn5 - (var_fn445_calc_ig__kbdgatein * var_fn445_calc_ig__iginbd_dn5))), (var_fn445_calc_ig__isdiodeout * (var_fn445_calc_ig__expifor_dn8 - (var_fn445_calc_ig__kbdgatein * var_fn445_calc_ig__iginbd_dn8))),)
    } else {
        (var_fn445_calc_ig__igindiode_nohinj, var_fn445_calc_ig__igindiode_nohinj_dn4, var_fn445_calc_ig__igindiode_nohinj_dn5, var_fn445_calc_ig__igindiode_nohinj_dn8,)
    }
};
        var_fn445_calc_ig__igindiode_nohinj = assign38970_e36517;
        var_fn445_calc_ig__igindiode_nohinj_dn4 = assign38970_e36517_d_n4;
        var_fn445_calc_ig__igindiode_nohinj_dn5 = assign38970_e36517_d_n5;
        var_fn445_calc_ig__igindiode_nohinj_dn8 = assign38970_e36517_d_n8;

        let assign38980_e36520: f64 = if var_fn445_calc_ig__fracin > 0.0 { 1.0 } else { 0.0 };
        var_guard447 = assign38980_e36520;

        *var_fn445_calc_ig__expbd1_slot = var_fn445_calc_ig__expbd1;
        *var_fn445_calc_ig__expbd1_dn4_slot = var_fn445_calc_ig__expbd1_dn4;
        *var_fn445_calc_ig__expbd1_dn5_slot = var_fn445_calc_ig__expbd1_dn5;
        *var_fn445_calc_ig__expbd1_dn8_slot = var_fn445_calc_ig__expbd1_dn8;
        *var_fn445_calc_ig__expbd1_vgsat_slot = var_fn445_calc_ig__expbd1_vgsat;
        *var_fn445_calc_ig__expbd1_vgsat_dn4_slot = var_fn445_calc_ig__expbd1_vgsat_dn4;
        *var_fn445_calc_ig__expbd2_slot = var_fn445_calc_ig__expbd2;
        *var_fn445_calc_ig__expbd2_dn4_slot = var_fn445_calc_ig__expbd2_dn4;
        *var_fn445_calc_ig__expbdarg1_slot = var_fn445_calc_ig__expbdarg1;
        *var_fn445_calc_ig__expbdarg1_dn4_slot = var_fn445_calc_ig__expbdarg1_dn4;
        *var_fn445_calc_ig__expbdarg1_dn5_slot = var_fn445_calc_ig__expbdarg1_dn5;
        *var_fn445_calc_ig__expbdarg1_dn8_slot = var_fn445_calc_ig__expbdarg1_dn8;
        *var_fn445_calc_ig__expbdarg1_vgsat_slot = var_fn445_calc_ig__expbdarg1_vgsat;
        *var_fn445_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn445_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn445_calc_ig__expbdarg2_slot = var_fn445_calc_ig__expbdarg2;
        *var_fn445_calc_ig__expbdarg2_dn4_slot = var_fn445_calc_ig__expbdarg2_dn4;
        *var_fn445_calc_ig__expifor_slot = var_fn445_calc_ig__expifor;
        *var_fn445_calc_ig__expifor_dn4_slot = var_fn445_calc_ig__expifor_dn4;
        *var_fn445_calc_ig__expifor_dn5_slot = var_fn445_calc_ig__expifor_dn5;
        *var_fn445_calc_ig__expifor_dn8_slot = var_fn445_calc_ig__expifor_dn8;
        *var_fn445_calc_ig__expifor_hinj_slot = var_fn445_calc_ig__expifor_hinj;
        *var_fn445_calc_ig__expifor_hinj_dn4_slot = var_fn445_calc_ig__expifor_hinj_dn4;
        *var_fn445_calc_ig__expifor_hinj_dn5_slot = var_fn445_calc_ig__expifor_hinj_dn5;
        *var_fn445_calc_ig__expifor_hinj_dn8_slot = var_fn445_calc_ig__expifor_hinj_dn8;
        *var_fn445_calc_ig__expifor_hinj_vgsat_slot = var_fn445_calc_ig__expifor_hinj_vgsat;
        *var_fn445_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn445_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn445_calc_ig__expifor_nohinj_vgsat_slot = var_fn445_calc_ig__expifor_nohinj_vgsat;
        *var_fn445_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn445_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn445_calc_ig__expiforarg_slot = var_fn445_calc_ig__expiforarg;
        *var_fn445_calc_ig__expiforarg_dn4_slot = var_fn445_calc_ig__expiforarg_dn4;
        *var_fn445_calc_ig__expiforarg_dn5_slot = var_fn445_calc_ig__expiforarg_dn5;
        *var_fn445_calc_ig__expiforarg_dn8_slot = var_fn445_calc_ig__expiforarg_dn8;
        *var_fn445_calc_ig__expiforarg_hinj_slot = var_fn445_calc_ig__expiforarg_hinj;
        *var_fn445_calc_ig__expiforarg_hinj_dn4_slot = var_fn445_calc_ig__expiforarg_hinj_dn4;
        *var_fn445_calc_ig__expiforarg_hinj_dn5_slot = var_fn445_calc_ig__expiforarg_hinj_dn5;
        *var_fn445_calc_ig__expiforarg_hinj_dn8_slot = var_fn445_calc_ig__expiforarg_hinj_dn8;
        *var_fn445_calc_ig__expiforarg_hinj_vgsat_slot = var_fn445_calc_ig__expiforarg_hinj_vgsat;
        *var_fn445_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn445_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn445_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn445_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn445_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn445_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn445_calc_ig__expphib_slot = var_fn445_calc_ig__expphib;
        *var_fn445_calc_ig__expphib_dn4_slot = var_fn445_calc_ig__expphib_dn4;
        *var_fn445_calc_ig__iginbd_slot = var_fn445_calc_ig__iginbd;
        *var_fn445_calc_ig__iginbd_dn4_slot = var_fn445_calc_ig__iginbd_dn4;
        *var_fn445_calc_ig__iginbd_dn5_slot = var_fn445_calc_ig__iginbd_dn5;
        *var_fn445_calc_ig__iginbd_dn8_slot = var_fn445_calc_ig__iginbd_dn8;
        *var_fn445_calc_ig__iginbd_vgsat_slot = var_fn445_calc_ig__iginbd_vgsat;
        *var_fn445_calc_ig__iginbd_vgsat_dn4_slot = var_fn445_calc_ig__iginbd_vgsat_dn4;
        *var_fn445_calc_ig__igindiode_slot = var_fn445_calc_ig__igindiode;
        *var_fn445_calc_ig__igindiode_dn4_slot = var_fn445_calc_ig__igindiode_dn4;
        *var_fn445_calc_ig__igindiode_dn5_slot = var_fn445_calc_ig__igindiode_dn5;
        *var_fn445_calc_ig__igindiode_dn8_slot = var_fn445_calc_ig__igindiode_dn8;
        *var_fn445_calc_ig__igindiode_hinj_slot = var_fn445_calc_ig__igindiode_hinj;
        *var_fn445_calc_ig__igindiode_hinj_dn4_slot = var_fn445_calc_ig__igindiode_hinj_dn4;
        *var_fn445_calc_ig__igindiode_hinj_dn5_slot = var_fn445_calc_ig__igindiode_hinj_dn5;
        *var_fn445_calc_ig__igindiode_hinj_dn8_slot = var_fn445_calc_ig__igindiode_hinj_dn8;
        *var_fn445_calc_ig__igindiode_hinj_pre_slot = var_fn445_calc_ig__igindiode_hinj_pre;
        *var_fn445_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn445_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn445_calc_ig__igindiode_hinj_vgsat_slot = var_fn445_calc_ig__igindiode_hinj_vgsat;
        *var_fn445_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn445_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn445_calc_ig__igindiode_nohinj_slot = var_fn445_calc_ig__igindiode_nohinj;
        *var_fn445_calc_ig__igindiode_nohinj_dn4_slot = var_fn445_calc_ig__igindiode_nohinj_dn4;
        *var_fn445_calc_ig__igindiode_nohinj_dn5_slot = var_fn445_calc_ig__igindiode_nohinj_dn5;
        *var_fn445_calc_ig__igindiode_nohinj_dn8_slot = var_fn445_calc_ig__igindiode_nohinj_dn8;
        *var_fn445_calc_ig__igindiode_nohinj_vgsat_slot = var_fn445_calc_ig__igindiode_nohinj_vgsat;
        *var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn445_calc_ig__isdiodeout_slot = var_fn445_calc_ig__isdiodeout;
        *var_fn445_calc_ig__isdiodeout_dn4_slot = var_fn445_calc_ig__isdiodeout_dn4;
        *var_fn445_calc_ig__t0_slot = var_fn445_calc_ig__t0;
        *var_fn445_calc_ig__t0_dn4_slot = var_fn445_calc_ig__t0_dn4;
        *var_guard446_slot = var_guard446;
        *var_guard447_slot = var_guard447;
    }

    pub(super) fn stamp_transient_block_97(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_fn445_calc_ig__alphagin: f64,
        var_fn445_calc_ig__betarecin: f64,
        var_fn445_calc_ig__expphib: f64,
        var_fn445_calc_ig__expphib_dn4: f64,
        var_fn445_calc_ig__fracin: f64,
        var_fn445_calc_ig__iginbd: f64,
        var_fn445_calc_ig__iginbd_dn4: f64,
        var_fn445_calc_ig__iginbd_dn5: f64,
        var_fn445_calc_ig__iginbd_dn8: f64,
        var_fn445_calc_ig__iginbd_vgsat: f64,
        var_fn445_calc_ig__iginbd_vgsat_dn4: f64,
        var_fn445_calc_ig__igindiode_nohinj: f64,
        var_fn445_calc_ig__igindiode_nohinj_dn4: f64,
        var_fn445_calc_ig__igindiode_nohinj_dn5: f64,
        var_fn445_calc_ig__igindiode_nohinj_dn8: f64,
        var_fn445_calc_ig__igindiode_nohinj_vgsat: f64,
        var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4: f64,
        var_fn445_calc_ig__irecin: f64,
        var_fn445_calc_ig__isdiodeout: f64,
        var_fn445_calc_ig__isdiodeout_dn4: f64,
        var_fn445_calc_ig__kbdgatein: f64,
        var_fn445_calc_ig__ngf: f64,
        var_fn445_calc_ig__pg_paramin: f64,
        var_fn445_calc_ig__pgsrecin: f64,
        var_fn445_calc_ig__phitin: f64,
        var_fn445_calc_ig__phitin_dn4: f64,
        var_fn445_calc_ig__t0: f64,
        var_fn445_calc_ig__t0_dn4: f64,
        var_fn445_calc_ig__tfacdiodein: f64,
        var_fn445_calc_ig__tfacdiodein_dn4: f64,
        var_fn445_calc_ig__type: f64,
        var_fn445_calc_ig__vgin: f64,
        var_fn445_calc_ig__vgin_dn5: f64,
        var_fn445_calc_ig__vgin_dn8: f64,
        var_fn445_calc_ig__vgsatin: f64,
        var_fn445_calc_ig__vgsatqin: f64,
        var_fn445_calc_ig__w: f64,
        var_guard417: f64,
        var_guard439: f64,
        var_guard446: f64,
        var_guard447: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_fn445_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn445_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn445_calc_ig__expffvarg_slot: &mut f64,
        var_fn445_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn445_calc_ig__expffvarg_dn5_slot: &mut f64,
        var_fn445_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn445_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn445_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn445_calc_ig__expifor_hinj_dn5_slot: &mut f64,
        var_fn445_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn445_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn445_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_hinj_dn5_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__expirev_slot: &mut f64,
        var_fn445_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn445_calc_ig__expirev_dn5_slot: &mut f64,
        var_fn445_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn445_calc_ig__expirevarg_slot: &mut f64,
        var_fn445_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn445_calc_ig__expirevarg_dn5_slot: &mut f64,
        var_fn445_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn445_calc_ig__ffvgin_slot: &mut f64,
        var_fn445_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn445_calc_ig__ffvgin_dn5_slot: &mut f64,
        var_fn445_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn445_calc_ig__frecgin_slot: &mut f64,
        var_fn445_calc_ig__frecgin_dn5_slot: &mut f64,
        var_fn445_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn445_calc_ig__igindiode_slot: &mut f64,
        var_fn445_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn445_calc_ig__igindiode_dn5_slot: &mut f64,
        var_fn445_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn445_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn445_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn445_calc_ig__igindiode_hinj_dn5_slot: &mut f64,
        var_fn445_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn445_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn445_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn445_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn445_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__iginrec_slot: &mut f64,
        var_fn445_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn445_calc_ig__iginrec_dn5_slot: &mut f64,
        var_fn445_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn445_calc_ig__igout_slot: &mut f64,
        var_fn445_calc_ig__igout_dn4_slot: &mut f64,
        var_fn445_calc_ig__igout_dn5_slot: &mut f64,
        var_fn445_calc_ig__igout_dn8_slot: &mut f64,
        var_fn445_calc_ig__isrecout_slot: &mut f64,
        var_fn445_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn445_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_fn445_calc_ig__return_slot: &mut f64,
        var_fn445_calc_ig__return_dn4_slot: &mut f64,
        var_fn445_calc_ig__return_dn5_slot: &mut f64,
        var_fn445_calc_ig__return_dn8_slot: &mut f64,
        var_fn451_calc_ig__alphagin_slot: &mut f64,
        var_fn451_calc_ig__isdiodeout_slot: &mut f64,
        var_fn451_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn451_calc_ig__isrecout_slot: &mut f64,
        var_fn451_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn451_calc_ig__phitin_slot: &mut f64,
        var_fn451_calc_ig__phitin_dn4_slot: &mut f64,
        var_fn451_calc_ig__return_slot: &mut f64,
        var_fn451_calc_ig__return_dn4_slot: &mut f64,
        var_fn451_calc_ig__return_dn8_slot: &mut f64,
        var_fn451_calc_ig__return_dn9_slot: &mut f64,
        var_fn451_calc_ig__vgin_slot: &mut f64,
        var_fn451_calc_ig__vgin_dn8_slot: &mut f64,
        var_fn451_calc_ig__vgin_dn9_slot: &mut f64,
        var_fn451_calc_ig__vgsatin_slot: &mut f64,
        var_guard448_slot: &mut f64,
        var_guard449_slot: &mut f64,
        var_guard450_slot: &mut f64,
        var_igdidb_slot: &mut f64,
        var_igdidb_dn4_slot: &mut f64,
        var_igdidb_dn5_slot: &mut f64,
        var_igdidb_dn8_slot: &mut f64,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let mut var_fn445_calc_ig__alpha2_phit: f64 = *var_fn445_calc_ig__alpha2_phit_slot;
        let mut var_fn445_calc_ig__alpha2_phit_dn4: f64 = *var_fn445_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn445_calc_ig__expffvarg: f64 = *var_fn445_calc_ig__expffvarg_slot;
        let mut var_fn445_calc_ig__expffvarg_dn4: f64 = *var_fn445_calc_ig__expffvarg_dn4_slot;
        let mut var_fn445_calc_ig__expffvarg_dn5: f64 = *var_fn445_calc_ig__expffvarg_dn5_slot;
        let mut var_fn445_calc_ig__expffvarg_dn8: f64 = *var_fn445_calc_ig__expffvarg_dn8_slot;
        let mut var_fn445_calc_ig__expifor_hinj: f64 = *var_fn445_calc_ig__expifor_hinj_slot;
        let mut var_fn445_calc_ig__expifor_hinj_dn4: f64 = *var_fn445_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn445_calc_ig__expifor_hinj_dn5: f64 = *var_fn445_calc_ig__expifor_hinj_dn5_slot;
        let mut var_fn445_calc_ig__expifor_hinj_dn8: f64 = *var_fn445_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn445_calc_ig__expifor_hinj_vgsat: f64 = *var_fn445_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn445_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn445_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__expiforarg_hinj: f64 = *var_fn445_calc_ig__expiforarg_hinj_slot;
        let mut var_fn445_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn445_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn445_calc_ig__expiforarg_hinj_dn5: f64 = *var_fn445_calc_ig__expiforarg_hinj_dn5_slot;
        let mut var_fn445_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn445_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn445_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn445_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn445_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn445_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__expirev: f64 = *var_fn445_calc_ig__expirev_slot;
        let mut var_fn445_calc_ig__expirev_dn4: f64 = *var_fn445_calc_ig__expirev_dn4_slot;
        let mut var_fn445_calc_ig__expirev_dn5: f64 = *var_fn445_calc_ig__expirev_dn5_slot;
        let mut var_fn445_calc_ig__expirev_dn8: f64 = *var_fn445_calc_ig__expirev_dn8_slot;
        let mut var_fn445_calc_ig__expirevarg: f64 = *var_fn445_calc_ig__expirevarg_slot;
        let mut var_fn445_calc_ig__expirevarg_dn4: f64 = *var_fn445_calc_ig__expirevarg_dn4_slot;
        let mut var_fn445_calc_ig__expirevarg_dn5: f64 = *var_fn445_calc_ig__expirevarg_dn5_slot;
        let mut var_fn445_calc_ig__expirevarg_dn8: f64 = *var_fn445_calc_ig__expirevarg_dn8_slot;
        let mut var_fn445_calc_ig__ffvgin: f64 = *var_fn445_calc_ig__ffvgin_slot;
        let mut var_fn445_calc_ig__ffvgin_dn4: f64 = *var_fn445_calc_ig__ffvgin_dn4_slot;
        let mut var_fn445_calc_ig__ffvgin_dn5: f64 = *var_fn445_calc_ig__ffvgin_dn5_slot;
        let mut var_fn445_calc_ig__ffvgin_dn8: f64 = *var_fn445_calc_ig__ffvgin_dn8_slot;
        let mut var_fn445_calc_ig__frecgin: f64 = *var_fn445_calc_ig__frecgin_slot;
        let mut var_fn445_calc_ig__frecgin_dn5: f64 = *var_fn445_calc_ig__frecgin_dn5_slot;
        let mut var_fn445_calc_ig__frecgin_dn8: f64 = *var_fn445_calc_ig__frecgin_dn8_slot;
        let mut var_fn445_calc_ig__igindiode: f64 = *var_fn445_calc_ig__igindiode_slot;
        let mut var_fn445_calc_ig__igindiode_dn4: f64 = *var_fn445_calc_ig__igindiode_dn4_slot;
        let mut var_fn445_calc_ig__igindiode_dn5: f64 = *var_fn445_calc_ig__igindiode_dn5_slot;
        let mut var_fn445_calc_ig__igindiode_dn8: f64 = *var_fn445_calc_ig__igindiode_dn8_slot;
        let mut var_fn445_calc_ig__igindiode_hinj: f64 = *var_fn445_calc_ig__igindiode_hinj_slot;
        let mut var_fn445_calc_ig__igindiode_hinj_dn4: f64 = *var_fn445_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn445_calc_ig__igindiode_hinj_dn5: f64 = *var_fn445_calc_ig__igindiode_hinj_dn5_slot;
        let mut var_fn445_calc_ig__igindiode_hinj_dn8: f64 = *var_fn445_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn445_calc_ig__igindiode_hinj_pre: f64 = *var_fn445_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn445_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn445_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn445_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn445_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn445_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn445_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__iginrec: f64 = *var_fn445_calc_ig__iginrec_slot;
        let mut var_fn445_calc_ig__iginrec_dn4: f64 = *var_fn445_calc_ig__iginrec_dn4_slot;
        let mut var_fn445_calc_ig__iginrec_dn5: f64 = *var_fn445_calc_ig__iginrec_dn5_slot;
        let mut var_fn445_calc_ig__iginrec_dn8: f64 = *var_fn445_calc_ig__iginrec_dn8_slot;
        let mut var_fn445_calc_ig__igout: f64 = *var_fn445_calc_ig__igout_slot;
        let mut var_fn445_calc_ig__igout_dn4: f64 = *var_fn445_calc_ig__igout_dn4_slot;
        let mut var_fn445_calc_ig__igout_dn5: f64 = *var_fn445_calc_ig__igout_dn5_slot;
        let mut var_fn445_calc_ig__igout_dn8: f64 = *var_fn445_calc_ig__igout_dn8_slot;
        let mut var_fn445_calc_ig__isrecout: f64 = *var_fn445_calc_ig__isrecout_slot;
        let mut var_fn445_calc_ig__isrecout_dn4: f64 = *var_fn445_calc_ig__isrecout_dn4_slot;
        let mut var_fn445_calc_ig__pg_paramin_hinj: f64 = *var_fn445_calc_ig__pg_paramin_hinj_slot;
        let mut var_fn445_calc_ig__return: f64 = *var_fn445_calc_ig__return_slot;
        let mut var_fn445_calc_ig__return_dn4: f64 = *var_fn445_calc_ig__return_dn4_slot;
        let mut var_fn445_calc_ig__return_dn5: f64 = *var_fn445_calc_ig__return_dn5_slot;
        let mut var_fn445_calc_ig__return_dn8: f64 = *var_fn445_calc_ig__return_dn8_slot;
        let mut var_fn451_calc_ig__alphagin: f64 = *var_fn451_calc_ig__alphagin_slot;
        let mut var_fn451_calc_ig__isdiodeout: f64 = *var_fn451_calc_ig__isdiodeout_slot;
        let mut var_fn451_calc_ig__isdiodeout_dn4: f64 = *var_fn451_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn451_calc_ig__isrecout: f64 = *var_fn451_calc_ig__isrecout_slot;
        let mut var_fn451_calc_ig__isrecout_dn4: f64 = *var_fn451_calc_ig__isrecout_dn4_slot;
        let mut var_fn451_calc_ig__phitin: f64 = *var_fn451_calc_ig__phitin_slot;
        let mut var_fn451_calc_ig__phitin_dn4: f64 = *var_fn451_calc_ig__phitin_dn4_slot;
        let mut var_fn451_calc_ig__return: f64 = *var_fn451_calc_ig__return_slot;
        let mut var_fn451_calc_ig__return_dn4: f64 = *var_fn451_calc_ig__return_dn4_slot;
        let mut var_fn451_calc_ig__return_dn8: f64 = *var_fn451_calc_ig__return_dn8_slot;
        let mut var_fn451_calc_ig__return_dn9: f64 = *var_fn451_calc_ig__return_dn9_slot;
        let mut var_fn451_calc_ig__vgin: f64 = *var_fn451_calc_ig__vgin_slot;
        let mut var_fn451_calc_ig__vgin_dn8: f64 = *var_fn451_calc_ig__vgin_dn8_slot;
        let mut var_fn451_calc_ig__vgin_dn9: f64 = *var_fn451_calc_ig__vgin_dn9_slot;
        let mut var_fn451_calc_ig__vgsatin: f64 = *var_fn451_calc_ig__vgsatin_slot;
        let mut var_guard448: f64 = *var_guard448_slot;
        let mut var_guard449: f64 = *var_guard449_slot;
        let mut var_guard450: f64 = *var_guard450_slot;
        let mut var_igdidb: f64 = *var_igdidb_slot;
        let mut var_igdidb_dn4: f64 = *var_igdidb_dn4_slot;
        let mut var_igdidb_dn5: f64 = *var_igdidb_dn5_slot;
        let mut var_igdidb_dn8: f64 = *var_igdidb_dn8_slot;

        let (assign38990_e36533,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) && (var_guard447 != 0.0)) {
        let assign38990_e36531: f64 = (var_fn445_calc_ig__fracin * var_fn445_calc_ig__pg_paramin);
        (assign38990_e36531,)
    } else {
        (var_fn445_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn445_calc_ig__pg_paramin_hinj = assign38990_e36533;

        let (assign39000_e36550, assign39000_e36550_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) && (var_guard447 != 0.0)) {
        let assign39000_e36544: f64 = (var_fn445_calc_ig__pg_paramin_hinj / var_fn445_calc_ig__phitin);
        let assign39000_e36546: f64 = (assign39000_e36544 * var_fn445_calc_ig__vgsatin);
        let assign39000_e36548: f64 = (assign39000_e36546 + var_fn445_calc_ig__expphib);
        (assign39000_e36548, (((-((var_fn445_calc_ig__pg_paramin_hinj * var_fn445_calc_ig__phitin_dn4) / (var_fn445_calc_ig__phitin * var_fn445_calc_ig__phitin))) * var_fn445_calc_ig__vgsatin) + var_fn445_calc_ig__expphib_dn4),)
    } else {
        (var_fn445_calc_ig__expiforarg_hinj_vgsat, var_fn445_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__expiforarg_hinj_vgsat = assign39000_e36550;
        var_fn445_calc_ig__expiforarg_hinj_vgsat_dn4 = assign39000_e36550_d_n4;

        let (assign39010_e36599, assign39010_e36599_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) && (var_guard447 != 0.0)) {
        let assign39010_e36565: f64 = (-50.0);
        let (assign39010_e36597, assign39010_e36597_d_n4,) = {
            if ((!(var_fn445_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (!(var_fn445_calc_ig__expiforarg_hinj_vgsat < assign39010_e36565))) {
                let assign39010_e36570: f64 = (var_fn445_calc_ig__expiforarg_hinj_vgsat).exp();
                (assign39010_e36570, (assign39010_e36570 * var_fn445_calc_ig__expiforarg_hinj_vgsat_dn4),)
            } else {
                let assign39010_e36577: f64 = (-50.0);
                let (assign39010_e36596, assign39010_e36596_d_n4,) = {
                    if ((!(var_fn445_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (var_fn445_calc_ig__expiforarg_hinj_vgsat < assign39010_e36577)) {
                        let assign39010_e36581: f64 = (-50.0);
                        let assign39010_e36582: f64 = (assign39010_e36581).exp();
                        (assign39010_e36582, 0.0,)
                    } else {
                        let (assign39010_e36595, assign39010_e36595_d_n4,) = {
                            if (var_fn445_calc_ig__expiforarg_hinj_vgsat > 50.0) {
                                let assign39010_e36587: f64 = (50.0_f64).exp();
                                let assign39010_e36591: f64 = (var_fn445_calc_ig__expiforarg_hinj_vgsat - 50.0);
                                let assign39010_e36592: f64 = (1.0 + assign39010_e36591);
                                let assign39010_e36593: f64 = (assign39010_e36587 * assign39010_e36592);
                                (assign39010_e36593, (assign39010_e36587 * var_fn445_calc_ig__expiforarg_hinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign39010_e36595, assign39010_e36595_d_n4,)
                    }
                };
                (assign39010_e36596, assign39010_e36596_d_n4,)
            }
        };
        (assign39010_e36597, assign39010_e36597_d_n4,)
    } else {
        (var_fn445_calc_ig__expifor_hinj_vgsat, var_fn445_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__expifor_hinj_vgsat = assign39010_e36599;
        var_fn445_calc_ig__expifor_hinj_vgsat_dn4 = assign39010_e36599_d_n4;

        let (assign39020_e36616, assign39020_e36616_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) && (var_guard447 != 0.0)) {
        let assign39020_e36611: f64 = (var_fn445_calc_ig__kbdgatein * var_fn445_calc_ig__iginbd_vgsat);
        let assign39020_e36612: f64 = (var_fn445_calc_ig__expifor_hinj_vgsat - assign39020_e36611);
        let assign39020_e36614: f64 = (assign39020_e36612 - var_fn445_calc_ig__t0);
        (assign39020_e36614, ((var_fn445_calc_ig__expifor_hinj_vgsat_dn4 - (var_fn445_calc_ig__kbdgatein * var_fn445_calc_ig__iginbd_vgsat_dn4)) - var_fn445_calc_ig__t0_dn4),)
    } else {
        (var_fn445_calc_ig__igindiode_hinj_vgsat, var_fn445_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__igindiode_hinj_vgsat = assign39020_e36616;
        var_fn445_calc_ig__igindiode_hinj_vgsat_dn4 = assign39020_e36616_d_n4;

        let (assign39030_e36633, assign39030_e36633_d_n4, assign39030_e36633_d_n5, assign39030_e36633_d_n8,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) && (var_guard447 != 0.0)) {
        let assign39030_e36627: f64 = (var_fn445_calc_ig__pg_paramin_hinj / var_fn445_calc_ig__phitin);
        let assign39030_e36629: f64 = (assign39030_e36627 * var_fn445_calc_ig__vgin);
        let assign39030_e36631: f64 = (assign39030_e36629 + var_fn445_calc_ig__expphib);
        (assign39030_e36631, (((-((var_fn445_calc_ig__pg_paramin_hinj * var_fn445_calc_ig__phitin_dn4) / (var_fn445_calc_ig__phitin * var_fn445_calc_ig__phitin))) * var_fn445_calc_ig__vgin) + var_fn445_calc_ig__expphib_dn4), (assign39030_e36627 * var_fn445_calc_ig__vgin_dn5), (assign39030_e36627 * var_fn445_calc_ig__vgin_dn8),)
    } else {
        (var_fn445_calc_ig__expiforarg_hinj, var_fn445_calc_ig__expiforarg_hinj_dn4, var_fn445_calc_ig__expiforarg_hinj_dn5, var_fn445_calc_ig__expiforarg_hinj_dn8,)
    }
};
        var_fn445_calc_ig__expiforarg_hinj = assign39030_e36633;
        var_fn445_calc_ig__expiforarg_hinj_dn4 = assign39030_e36633_d_n4;
        var_fn445_calc_ig__expiforarg_hinj_dn5 = assign39030_e36633_d_n5;
        var_fn445_calc_ig__expiforarg_hinj_dn8 = assign39030_e36633_d_n8;

        let (assign39040_e36682, assign39040_e36682_d_n4, assign39040_e36682_d_n5, assign39040_e36682_d_n8,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) && (var_guard447 != 0.0)) {
        let assign39040_e36648: f64 = (-50.0);
        let (assign39040_e36680, assign39040_e36680_d_n4, assign39040_e36680_d_n5, assign39040_e36680_d_n8,) = {
            if ((!(var_fn445_calc_ig__expiforarg_hinj > 50.0)) && (!(var_fn445_calc_ig__expiforarg_hinj < assign39040_e36648))) {
                let assign39040_e36653: f64 = (var_fn445_calc_ig__expiforarg_hinj).exp();
                (assign39040_e36653, (assign39040_e36653 * var_fn445_calc_ig__expiforarg_hinj_dn4), (assign39040_e36653 * var_fn445_calc_ig__expiforarg_hinj_dn5), (assign39040_e36653 * var_fn445_calc_ig__expiforarg_hinj_dn8),)
            } else {
                let assign39040_e36660: f64 = (-50.0);
                let (assign39040_e36679, assign39040_e36679_d_n4, assign39040_e36679_d_n5, assign39040_e36679_d_n8,) = {
                    if ((!(var_fn445_calc_ig__expiforarg_hinj > 50.0)) && (var_fn445_calc_ig__expiforarg_hinj < assign39040_e36660)) {
                        let assign39040_e36664: f64 = (-50.0);
                        let assign39040_e36665: f64 = (assign39040_e36664).exp();
                        (assign39040_e36665, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign39040_e36678, assign39040_e36678_d_n4, assign39040_e36678_d_n5, assign39040_e36678_d_n8,) = {
                            if (var_fn445_calc_ig__expiforarg_hinj > 50.0) {
                                let assign39040_e36670: f64 = (50.0_f64).exp();
                                let assign39040_e36674: f64 = (var_fn445_calc_ig__expiforarg_hinj - 50.0);
                                let assign39040_e36675: f64 = (1.0 + assign39040_e36674);
                                let assign39040_e36676: f64 = (assign39040_e36670 * assign39040_e36675);
                                (assign39040_e36676, (assign39040_e36670 * var_fn445_calc_ig__expiforarg_hinj_dn4), (assign39040_e36670 * var_fn445_calc_ig__expiforarg_hinj_dn5), (assign39040_e36670 * var_fn445_calc_ig__expiforarg_hinj_dn8),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign39040_e36678, assign39040_e36678_d_n4, assign39040_e36678_d_n5, assign39040_e36678_d_n8,)
                    }
                };
                (assign39040_e36679, assign39040_e36679_d_n4, assign39040_e36679_d_n5, assign39040_e36679_d_n8,)
            }
        };
        (assign39040_e36680, assign39040_e36680_d_n4, assign39040_e36680_d_n5, assign39040_e36680_d_n8,)
    } else {
        (var_fn445_calc_ig__expifor_hinj, var_fn445_calc_ig__expifor_hinj_dn4, var_fn445_calc_ig__expifor_hinj_dn5, var_fn445_calc_ig__expifor_hinj_dn8,)
    }
};
        var_fn445_calc_ig__expifor_hinj = assign39040_e36682;
        var_fn445_calc_ig__expifor_hinj_dn4 = assign39040_e36682_d_n4;
        var_fn445_calc_ig__expifor_hinj_dn5 = assign39040_e36682_d_n5;
        var_fn445_calc_ig__expifor_hinj_dn8 = assign39040_e36682_d_n8;

        let (assign39050_e36697, assign39050_e36697_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) && (var_guard447 != 0.0)) {
        let assign39050_e36693: f64 = (var_fn445_calc_ig__isdiodeout * var_fn445_calc_ig__igindiode_nohinj_vgsat);
        let assign39050_e36695: f64 = (assign39050_e36693 / var_fn445_calc_ig__igindiode_hinj_vgsat);
        (assign39050_e36695, (((((var_fn445_calc_ig__isdiodeout_dn4 * var_fn445_calc_ig__igindiode_nohinj_vgsat) + (var_fn445_calc_ig__isdiodeout * var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4)) * var_fn445_calc_ig__igindiode_hinj_vgsat) - (assign39050_e36693 * var_fn445_calc_ig__igindiode_hinj_vgsat_dn4)) / (var_fn445_calc_ig__igindiode_hinj_vgsat * var_fn445_calc_ig__igindiode_hinj_vgsat)),)
    } else {
        (var_fn445_calc_ig__igindiode_hinj_pre, var_fn445_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn445_calc_ig__igindiode_hinj_pre = assign39050_e36697;
        var_fn445_calc_ig__igindiode_hinj_pre_dn4 = assign39050_e36697_d_n4;

        let (assign39060_e36716, assign39060_e36716_d_n4, assign39060_e36716_d_n5, assign39060_e36716_d_n8,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) && (var_guard447 != 0.0)) {
        let assign39060_e36710: f64 = (var_fn445_calc_ig__kbdgatein * var_fn445_calc_ig__iginbd);
        let assign39060_e36711: f64 = (var_fn445_calc_ig__expifor_hinj - assign39060_e36710);
        let assign39060_e36713: f64 = (assign39060_e36711 - var_fn445_calc_ig__t0);
        let assign39060_e36714: f64 = (var_fn445_calc_ig__igindiode_hinj_pre * assign39060_e36713);
        (assign39060_e36714, ((var_fn445_calc_ig__igindiode_hinj_pre_dn4 * assign39060_e36713) + (var_fn445_calc_ig__igindiode_hinj_pre * ((var_fn445_calc_ig__expifor_hinj_dn4 - (var_fn445_calc_ig__kbdgatein * var_fn445_calc_ig__iginbd_dn4)) - var_fn445_calc_ig__t0_dn4))), (var_fn445_calc_ig__igindiode_hinj_pre * (var_fn445_calc_ig__expifor_hinj_dn5 - (var_fn445_calc_ig__kbdgatein * var_fn445_calc_ig__iginbd_dn5))), (var_fn445_calc_ig__igindiode_hinj_pre * (var_fn445_calc_ig__expifor_hinj_dn8 - (var_fn445_calc_ig__kbdgatein * var_fn445_calc_ig__iginbd_dn8))),)
    } else {
        (var_fn445_calc_ig__igindiode_hinj, var_fn445_calc_ig__igindiode_hinj_dn4, var_fn445_calc_ig__igindiode_hinj_dn5, var_fn445_calc_ig__igindiode_hinj_dn8,)
    }
};
        var_fn445_calc_ig__igindiode_hinj = assign39060_e36716;
        var_fn445_calc_ig__igindiode_hinj_dn4 = assign39060_e36716_d_n4;
        var_fn445_calc_ig__igindiode_hinj_dn5 = assign39060_e36716_d_n5;
        var_fn445_calc_ig__igindiode_hinj_dn8 = assign39060_e36716_d_n8;

        let (assign39070_e36730, assign39070_e36730_d_n4, assign39070_e36730_d_n5, assign39070_e36730_d_n8,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) && (var_guard447 == 0.0)) {
        let assign39070_e36728: f64 = (var_fn445_calc_ig__isdiodeout * var_fn445_calc_ig__igindiode_nohinj_vgsat);
        (assign39070_e36728, ((var_fn445_calc_ig__isdiodeout_dn4 * var_fn445_calc_ig__igindiode_nohinj_vgsat) + (var_fn445_calc_ig__isdiodeout * var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4)), 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__igindiode_hinj, var_fn445_calc_ig__igindiode_hinj_dn4, var_fn445_calc_ig__igindiode_hinj_dn5, var_fn445_calc_ig__igindiode_hinj_dn8,)
    }
};
        var_fn445_calc_ig__igindiode_hinj = assign39070_e36730;
        var_fn445_calc_ig__igindiode_hinj_dn4 = assign39070_e36730_d_n4;
        var_fn445_calc_ig__igindiode_hinj_dn5 = assign39070_e36730_d_n5;
        var_fn445_calc_ig__igindiode_hinj_dn8 = assign39070_e36730_d_n8;

        let (assign39080_e36743, assign39080_e36743_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) {
        let assign39080_e36739: f64 = (var_fn445_calc_ig__alphagin * var_fn445_calc_ig__alphagin);
        let assign39080_e36741: f64 = (assign39080_e36739 * var_fn445_calc_ig__phitin);
        (assign39080_e36741, (assign39080_e36739 * var_fn445_calc_ig__phitin_dn4),)
    } else {
        (var_fn445_calc_ig__alpha2_phit, var_fn445_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn445_calc_ig__alpha2_phit = assign39080_e36743;
        var_fn445_calc_ig__alpha2_phit_dn4 = assign39080_e36743_d_n4;

        let (assign39090_e36760, assign39090_e36760_d_n4, assign39090_e36760_d_n5, assign39090_e36760_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) {
        let assign39090_e36754: f64 = (var_fn445_calc_ig__alpha2_phit / 2.0);
        let assign39090_e36755: f64 = (var_fn445_calc_ig__vgsatin - assign39090_e36754);
        let assign39090_e36756: f64 = (var_fn445_calc_ig__vgin - assign39090_e36755);
        let assign39090_e36758: f64 = (assign39090_e36756 / var_fn445_calc_ig__alpha2_phit);
        (assign39090_e36758, ((((-(-(var_fn445_calc_ig__alpha2_phit_dn4 / 2.0))) * var_fn445_calc_ig__alpha2_phit) - (assign39090_e36756 * var_fn445_calc_ig__alpha2_phit_dn4)) / (var_fn445_calc_ig__alpha2_phit * var_fn445_calc_ig__alpha2_phit)), (var_fn445_calc_ig__vgin_dn5 / var_fn445_calc_ig__alpha2_phit), (var_fn445_calc_ig__vgin_dn8 / var_fn445_calc_ig__alpha2_phit),)
    } else {
        (var_fn445_calc_ig__expffvarg, var_fn445_calc_ig__expffvarg_dn4, var_fn445_calc_ig__expffvarg_dn5, var_fn445_calc_ig__expffvarg_dn8,)
    }
};
        var_fn445_calc_ig__expffvarg = assign39090_e36760;
        var_fn445_calc_ig__expffvarg_dn4 = assign39090_e36760_d_n4;
        var_fn445_calc_ig__expffvarg_dn5 = assign39090_e36760_d_n5;
        var_fn445_calc_ig__expffvarg_dn8 = assign39090_e36760_d_n8;

        let assign39100_e36763: f64 = if var_fn445_calc_ig__expffvarg > 50.0 { 1.0 } else { 0.0 };
        var_guard448 = assign39100_e36763;

        let (assign39110_e36774, assign39110_e36774_d_n4, assign39110_e36774_d_n5, assign39110_e36774_d_n8,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) && (var_guard448 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__ffvgin, var_fn445_calc_ig__ffvgin_dn4, var_fn445_calc_ig__ffvgin_dn5, var_fn445_calc_ig__ffvgin_dn8,)
    }
};
        var_fn445_calc_ig__ffvgin = assign39110_e36774;
        var_fn445_calc_ig__ffvgin_dn4 = assign39110_e36774_d_n4;
        var_fn445_calc_ig__ffvgin_dn5 = assign39110_e36774_d_n5;
        var_fn445_calc_ig__ffvgin_dn8 = assign39110_e36774_d_n8;

        let assign39120_e36777: f64 = (-50.0);
        let assign39120_e36778: f64 = if var_fn445_calc_ig__expffvarg < assign39120_e36777 { 1.0 } else { 0.0 };
        var_guard449 = assign39120_e36778;

        let (assign39130_e36792, assign39130_e36792_d_n4, assign39130_e36792_d_n5, assign39130_e36792_d_n8,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) && (var_guard448 == 0.0)) && (var_guard449 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__ffvgin, var_fn445_calc_ig__ffvgin_dn4, var_fn445_calc_ig__ffvgin_dn5, var_fn445_calc_ig__ffvgin_dn8,)
    }
};
        var_fn445_calc_ig__ffvgin = assign39130_e36792;
        var_fn445_calc_ig__ffvgin_dn4 = assign39130_e36792_d_n4;
        var_fn445_calc_ig__ffvgin_dn5 = assign39130_e36792_d_n5;
        var_fn445_calc_ig__ffvgin_dn8 = assign39130_e36792_d_n8;

        let (assign39140_e36812, assign39140_e36812_d_n4, assign39140_e36812_d_n5, assign39140_e36812_d_n8,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) && (var_guard448 == 0.0)) && (var_guard449 == 0.0)) {
        let assign39140_e36808: f64 = (var_fn445_calc_ig__expffvarg).exp();
        let assign39140_e36809: f64 = (1.0 + assign39140_e36808);
        let assign39140_e36810: f64 = (1.0 / assign39140_e36809);
        (assign39140_e36810, (-((assign39140_e36808 * var_fn445_calc_ig__expffvarg_dn4) / (assign39140_e36809 * assign39140_e36809))), (-((assign39140_e36808 * var_fn445_calc_ig__expffvarg_dn5) / (assign39140_e36809 * assign39140_e36809))), (-((assign39140_e36808 * var_fn445_calc_ig__expffvarg_dn8) / (assign39140_e36809 * assign39140_e36809))),)
    } else {
        (var_fn445_calc_ig__ffvgin, var_fn445_calc_ig__ffvgin_dn4, var_fn445_calc_ig__ffvgin_dn5, var_fn445_calc_ig__ffvgin_dn8,)
    }
};
        var_fn445_calc_ig__ffvgin = assign39140_e36812;
        var_fn445_calc_ig__ffvgin_dn4 = assign39140_e36812_d_n4;
        var_fn445_calc_ig__ffvgin_dn5 = assign39140_e36812_d_n5;
        var_fn445_calc_ig__ffvgin_dn8 = assign39140_e36812_d_n8;

        let (assign39150_e36829, assign39150_e36829_d_n4, assign39150_e36829_d_n5, assign39150_e36829_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard446 == 0.0)) {
        let assign39150_e36821: f64 = (var_fn445_calc_ig__ffvgin * var_fn445_calc_ig__igindiode_nohinj);
        let assign39150_e36824: f64 = (1.0 - var_fn445_calc_ig__ffvgin);
        let assign39150_e36826: f64 = (assign39150_e36824 * var_fn445_calc_ig__igindiode_hinj);
        let assign39150_e36827: f64 = (assign39150_e36821 + assign39150_e36826);
        (assign39150_e36827, (((var_fn445_calc_ig__ffvgin_dn4 * var_fn445_calc_ig__igindiode_nohinj) + (var_fn445_calc_ig__ffvgin * var_fn445_calc_ig__igindiode_nohinj_dn4)) + (((-var_fn445_calc_ig__ffvgin_dn4) * var_fn445_calc_ig__igindiode_hinj) + (assign39150_e36824 * var_fn445_calc_ig__igindiode_hinj_dn4))), (((var_fn445_calc_ig__ffvgin_dn5 * var_fn445_calc_ig__igindiode_nohinj) + (var_fn445_calc_ig__ffvgin * var_fn445_calc_ig__igindiode_nohinj_dn5)) + (((-var_fn445_calc_ig__ffvgin_dn5) * var_fn445_calc_ig__igindiode_hinj) + (assign39150_e36824 * var_fn445_calc_ig__igindiode_hinj_dn5))), (((var_fn445_calc_ig__ffvgin_dn8 * var_fn445_calc_ig__igindiode_nohinj) + (var_fn445_calc_ig__ffvgin * var_fn445_calc_ig__igindiode_nohinj_dn8)) + (((-var_fn445_calc_ig__ffvgin_dn8) * var_fn445_calc_ig__igindiode_hinj) + (assign39150_e36824 * var_fn445_calc_ig__igindiode_hinj_dn8))),)
    } else {
        (var_fn445_calc_ig__igindiode, var_fn445_calc_ig__igindiode_dn4, var_fn445_calc_ig__igindiode_dn5, var_fn445_calc_ig__igindiode_dn8,)
    }
};
        var_fn445_calc_ig__igindiode = assign39150_e36829;
        var_fn445_calc_ig__igindiode_dn4 = assign39150_e36829_d_n4;
        var_fn445_calc_ig__igindiode_dn5 = assign39150_e36829_d_n5;
        var_fn445_calc_ig__igindiode_dn8 = assign39150_e36829_d_n8;

        let (assign39160_e36877, assign39160_e36877_d_n5, assign39160_e36877_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign39160_e36834: f64 = (-var_fn445_calc_ig__vgin);
        let (assign39160_e36867, assign39160_e36867_d_n5, assign39160_e36867_d_n8,) = {
            if (p.p52 != 0.0) {
                let assign39160_e36842: f64 = (var_fn445_calc_ig__vgin / var_fn445_calc_ig__vgsatqin);
                let assign39160_e36845: f64 = (0.001 / p.p53);
                let assign39160_e36848: f64 = (var_fn445_calc_ig__vgin / var_fn445_calc_ig__vgsatqin);
                let assign39160_e36849: f64 = (assign39160_e36845 * assign39160_e36848);
                let assign39160_e36850: f64 = (assign39160_e36849).tanh();
                let assign39160_e36851: f64 = (assign39160_e36842 * assign39160_e36850);
                (assign39160_e36851, (((var_fn445_calc_ig__vgin_dn5 / var_fn445_calc_ig__vgsatqin) * assign39160_e36850) + (assign39160_e36842 * ((assign39160_e36845 * (var_fn445_calc_ig__vgin_dn5 / var_fn445_calc_ig__vgsatqin)) / ((assign39160_e36849).cosh() * (assign39160_e36849).cosh())))), (((var_fn445_calc_ig__vgin_dn8 / var_fn445_calc_ig__vgsatqin) * assign39160_e36850) + (assign39160_e36842 * ((assign39160_e36845 * (var_fn445_calc_ig__vgin_dn8 / var_fn445_calc_ig__vgsatqin)) / ((assign39160_e36849).cosh() * (assign39160_e36849).cosh())))),)
            } else {
                let (assign39160_e36866, assign39160_e36866_d_n5, assign39160_e36866_d_n8,) = {
                    if (p.p52 == 0.0) {
                        let __rspice_inv_cse_0: f64 = 1.0 / var_fn445_calc_ig__vgsatqin;
                        let assign39160_e36857: f64 = (var_fn445_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign39160_e36860: f64 = (var_fn445_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign39160_e36861: f64 = (assign39160_e36857 * assign39160_e36860);
                        let assign39160_e36863: f64 = (assign39160_e36861 + p.p53);
                        let assign39160_e36864: f64 = (assign39160_e36863).sqrt();
                        (assign39160_e36864, ((((var_fn445_calc_ig__vgin_dn5 / var_fn445_calc_ig__vgsatqin) * assign39160_e36860) + (assign39160_e36857 * (var_fn445_calc_ig__vgin_dn5 / var_fn445_calc_ig__vgsatqin))) / (2.0 * assign39160_e36864)), ((((var_fn445_calc_ig__vgin_dn8 / var_fn445_calc_ig__vgsatqin) * assign39160_e36860) + (assign39160_e36857 * (var_fn445_calc_ig__vgin_dn8 / var_fn445_calc_ig__vgsatqin))) / (2.0 * assign39160_e36864)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign39160_e36866, assign39160_e36866_d_n5, assign39160_e36866_d_n8,)
            }
        };
        let assign39160_e36869: f64 = (assign39160_e36867).powf(var_fn445_calc_ig__betarecin);
        let assign39160_e36870: f64 = (1.0 + assign39160_e36869);
        let assign39160_e36873: f64 = (1.0 / var_fn445_calc_ig__betarecin);
        let assign39160_e36874: f64 = (assign39160_e36870).powf(assign39160_e36873);
        let assign39160_e36875: f64 = (assign39160_e36834 / assign39160_e36874);
        (assign39160_e36875, ((((-var_fn445_calc_ig__vgin_dn5) * assign39160_e36874) - (assign39160_e36834 * if 0.0 == 0.0 && ((assign39160_e36873) as f64).is_finite() && ((assign39160_e36873) as f64).fract() == 0.0 { if assign39160_e36873 == 0.0 { 0.0 } else { (assign39160_e36873 * ((assign39160_e36870).powf(assign39160_e36873 - 1.0) * if 0.0 == 0.0 && ((var_fn445_calc_ig__betarecin) as f64).is_finite() && ((var_fn445_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn445_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn445_calc_ig__betarecin * ((assign39160_e36867).powf(var_fn445_calc_ig__betarecin - 1.0) * assign39160_e36867_d_n5)) } } else { (assign39160_e36869 * (var_fn445_calc_ig__betarecin * (assign39160_e36867_d_n5 / assign39160_e36867))) })) } } else { (assign39160_e36874 * (assign39160_e36873 * (if 0.0 == 0.0 && ((var_fn445_calc_ig__betarecin) as f64).is_finite() && ((var_fn445_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn445_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn445_calc_ig__betarecin * ((assign39160_e36867).powf(var_fn445_calc_ig__betarecin - 1.0) * assign39160_e36867_d_n5)) } } else { (assign39160_e36869 * (var_fn445_calc_ig__betarecin * (assign39160_e36867_d_n5 / assign39160_e36867))) } / assign39160_e36870))) })) / (assign39160_e36874 * assign39160_e36874)), ((((-var_fn445_calc_ig__vgin_dn8) * assign39160_e36874) - (assign39160_e36834 * if 0.0 == 0.0 && ((assign39160_e36873) as f64).is_finite() && ((assign39160_e36873) as f64).fract() == 0.0 { if assign39160_e36873 == 0.0 { 0.0 } else { (assign39160_e36873 * ((assign39160_e36870).powf(assign39160_e36873 - 1.0) * if 0.0 == 0.0 && ((var_fn445_calc_ig__betarecin) as f64).is_finite() && ((var_fn445_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn445_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn445_calc_ig__betarecin * ((assign39160_e36867).powf(var_fn445_calc_ig__betarecin - 1.0) * assign39160_e36867_d_n8)) } } else { (assign39160_e36869 * (var_fn445_calc_ig__betarecin * (assign39160_e36867_d_n8 / assign39160_e36867))) })) } } else { (assign39160_e36874 * (assign39160_e36873 * (if 0.0 == 0.0 && ((var_fn445_calc_ig__betarecin) as f64).is_finite() && ((var_fn445_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn445_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn445_calc_ig__betarecin * ((assign39160_e36867).powf(var_fn445_calc_ig__betarecin - 1.0) * assign39160_e36867_d_n8)) } } else { (assign39160_e36869 * (var_fn445_calc_ig__betarecin * (assign39160_e36867_d_n8 / assign39160_e36867))) } / assign39160_e36870))) })) / (assign39160_e36874 * assign39160_e36874)),)
    } else {
        (var_fn445_calc_ig__frecgin, var_fn445_calc_ig__frecgin_dn5, var_fn445_calc_ig__frecgin_dn8,)
    }
};
        var_fn445_calc_ig__frecgin = assign39160_e36877;
        var_fn445_calc_ig__frecgin_dn5 = assign39160_e36877_d_n5;
        var_fn445_calc_ig__frecgin_dn8 = assign39160_e36877_d_n8;

        let (assign39170_e36894, assign39170_e36894_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign39170_e36882: f64 = (-var_fn445_calc_ig__type);
        let assign39170_e36884: f64 = (assign39170_e36882 * var_fn445_calc_ig__w);
        let assign39170_e36886: f64 = (assign39170_e36884 * var_fn445_calc_ig__ngf);
        let assign39170_e36888: f64 = (assign39170_e36886 * var_fn445_calc_ig__irecin);
        let assign39170_e36890: f64 = (assign39170_e36888 * var_fn445_calc_ig__tfacdiodein);
        let assign39170_e36892: f64 = assign39170_e36890;
        (assign39170_e36892, (assign39170_e36888 * var_fn445_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn445_calc_ig__isrecout, var_fn445_calc_ig__isrecout_dn4,)
    }
};
        var_fn445_calc_ig__isrecout = assign39170_e36894;
        var_fn445_calc_ig__isrecout_dn4 = assign39170_e36894_d_n4;

        let (assign39180_e36904, assign39180_e36904_d_n4, assign39180_e36904_d_n5, assign39180_e36904_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign39180_e36900: f64 = (var_fn445_calc_ig__pgsrecin / var_fn445_calc_ig__phitin);
        let assign39180_e36902: f64 = (assign39180_e36900 * var_fn445_calc_ig__frecgin);
        (assign39180_e36902, ((-((var_fn445_calc_ig__pgsrecin * var_fn445_calc_ig__phitin_dn4) / (var_fn445_calc_ig__phitin * var_fn445_calc_ig__phitin))) * var_fn445_calc_ig__frecgin), (assign39180_e36900 * var_fn445_calc_ig__frecgin_dn5), (assign39180_e36900 * var_fn445_calc_ig__frecgin_dn8),)
    } else {
        (var_fn445_calc_ig__expirevarg, var_fn445_calc_ig__expirevarg_dn4, var_fn445_calc_ig__expirevarg_dn5, var_fn445_calc_ig__expirevarg_dn8,)
    }
};
        var_fn445_calc_ig__expirevarg = assign39180_e36904;
        var_fn445_calc_ig__expirevarg_dn4 = assign39180_e36904_d_n4;
        var_fn445_calc_ig__expirevarg_dn5 = assign39180_e36904_d_n5;
        var_fn445_calc_ig__expirevarg_dn8 = assign39180_e36904_d_n8;

        let (assign39190_e36948, assign39190_e36948_d_n4, assign39190_e36948_d_n5, assign39190_e36948_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign39190_e36914: f64 = (-50.0);
        let (assign39190_e36946, assign39190_e36946_d_n4, assign39190_e36946_d_n5, assign39190_e36946_d_n8,) = {
            if ((!(var_fn445_calc_ig__expirevarg > 50.0)) && (!(var_fn445_calc_ig__expirevarg < assign39190_e36914))) {
                let assign39190_e36919: f64 = (var_fn445_calc_ig__expirevarg).exp();
                (assign39190_e36919, (assign39190_e36919 * var_fn445_calc_ig__expirevarg_dn4), (assign39190_e36919 * var_fn445_calc_ig__expirevarg_dn5), (assign39190_e36919 * var_fn445_calc_ig__expirevarg_dn8),)
            } else {
                let assign39190_e36926: f64 = (-50.0);
                let (assign39190_e36945, assign39190_e36945_d_n4, assign39190_e36945_d_n5, assign39190_e36945_d_n8,) = {
                    if ((!(var_fn445_calc_ig__expirevarg > 50.0)) && (var_fn445_calc_ig__expirevarg < assign39190_e36926)) {
                        let assign39190_e36930: f64 = (-50.0);
                        let assign39190_e36931: f64 = (assign39190_e36930).exp();
                        (assign39190_e36931, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign39190_e36944, assign39190_e36944_d_n4, assign39190_e36944_d_n5, assign39190_e36944_d_n8,) = {
                            if (var_fn445_calc_ig__expirevarg > 50.0) {
                                let assign39190_e36936: f64 = (50.0_f64).exp();
                                let assign39190_e36940: f64 = (var_fn445_calc_ig__expirevarg - 50.0);
                                let assign39190_e36941: f64 = (1.0 + assign39190_e36940);
                                let assign39190_e36942: f64 = (assign39190_e36936 * assign39190_e36941);
                                (assign39190_e36942, (assign39190_e36936 * var_fn445_calc_ig__expirevarg_dn4), (assign39190_e36936 * var_fn445_calc_ig__expirevarg_dn5), (assign39190_e36936 * var_fn445_calc_ig__expirevarg_dn8),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign39190_e36944, assign39190_e36944_d_n4, assign39190_e36944_d_n5, assign39190_e36944_d_n8,)
                    }
                };
                (assign39190_e36945, assign39190_e36945_d_n4, assign39190_e36945_d_n5, assign39190_e36945_d_n8,)
            }
        };
        (assign39190_e36946, assign39190_e36946_d_n4, assign39190_e36946_d_n5, assign39190_e36946_d_n8,)
    } else {
        (var_fn445_calc_ig__expirev, var_fn445_calc_ig__expirev_dn4, var_fn445_calc_ig__expirev_dn5, var_fn445_calc_ig__expirev_dn8,)
    }
};
        var_fn445_calc_ig__expirev = assign39190_e36948;
        var_fn445_calc_ig__expirev_dn4 = assign39190_e36948_d_n4;
        var_fn445_calc_ig__expirev_dn5 = assign39190_e36948_d_n5;
        var_fn445_calc_ig__expirev_dn8 = assign39190_e36948_d_n8;

        let (assign39200_e36958, assign39200_e36958_d_n4, assign39200_e36958_d_n5, assign39200_e36958_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign39200_e36955: f64 = (var_fn445_calc_ig__expirev - 1.0);
        let assign39200_e36956: f64 = (var_fn445_calc_ig__isrecout * assign39200_e36955);
        (assign39200_e36956, ((var_fn445_calc_ig__isrecout_dn4 * assign39200_e36955) + (var_fn445_calc_ig__isrecout * var_fn445_calc_ig__expirev_dn4)), (var_fn445_calc_ig__isrecout * var_fn445_calc_ig__expirev_dn5), (var_fn445_calc_ig__isrecout * var_fn445_calc_ig__expirev_dn8),)
    } else {
        (var_fn445_calc_ig__iginrec, var_fn445_calc_ig__iginrec_dn4, var_fn445_calc_ig__iginrec_dn5, var_fn445_calc_ig__iginrec_dn8,)
    }
};
        var_fn445_calc_ig__iginrec = assign39200_e36958;
        var_fn445_calc_ig__iginrec_dn4 = assign39200_e36958_d_n4;
        var_fn445_calc_ig__iginrec_dn5 = assign39200_e36958_d_n5;
        var_fn445_calc_ig__iginrec_dn8 = assign39200_e36958_d_n8;

        let (assign39210_e36966, assign39210_e36966_d_n4, assign39210_e36966_d_n5, assign39210_e36966_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign39210_e36964: f64 = (var_fn445_calc_ig__igindiode + var_fn445_calc_ig__iginrec);
        (assign39210_e36964, (var_fn445_calc_ig__igindiode_dn4 + var_fn445_calc_ig__iginrec_dn4), (var_fn445_calc_ig__igindiode_dn5 + var_fn445_calc_ig__iginrec_dn5), (var_fn445_calc_ig__igindiode_dn8 + var_fn445_calc_ig__iginrec_dn8),)
    } else {
        (var_fn445_calc_ig__igout, var_fn445_calc_ig__igout_dn4, var_fn445_calc_ig__igout_dn5, var_fn445_calc_ig__igout_dn8,)
    }
};
        var_fn445_calc_ig__igout = assign39210_e36966;
        var_fn445_calc_ig__igout_dn4 = assign39210_e36966_d_n4;
        var_fn445_calc_ig__igout_dn5 = assign39210_e36966_d_n5;
        var_fn445_calc_ig__igout_dn8 = assign39210_e36966_d_n8;

        let (assign39220_e36972, assign39220_e36972_d_n4, assign39220_e36972_d_n5, assign39220_e36972_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (var_fn445_calc_ig__igout, var_fn445_calc_ig__igout_dn4, var_fn445_calc_ig__igout_dn5, var_fn445_calc_ig__igout_dn8,)
    } else {
        (var_fn445_calc_ig__return, var_fn445_calc_ig__return_dn4, var_fn445_calc_ig__return_dn5, var_fn445_calc_ig__return_dn8,)
    }
};
        var_fn445_calc_ig__return = assign39220_e36972;
        var_fn445_calc_ig__return_dn4 = assign39220_e36972_d_n4;
        var_fn445_calc_ig__return_dn5 = assign39220_e36972_d_n5;
        var_fn445_calc_ig__return_dn8 = assign39220_e36972_d_n8;

        let (assign39250_e36990, assign39250_e36990_d_n4, assign39250_e36990_d_n5, assign39250_e36990_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (var_fn445_calc_ig__return, var_fn445_calc_ig__return_dn4, var_fn445_calc_ig__return_dn5, var_fn445_calc_ig__return_dn8,)
    } else {
        (var_igdidb, var_igdidb_dn4, var_igdidb_dn5, var_igdidb_dn8,)
    }
};
        var_igdidb = assign39250_e36990;
        var_igdidb_dn4 = assign39250_e36990_d_n4;
        var_igdidb_dn5 = assign39250_e36990_d_n5;
        var_igdidb_dn8 = assign39250_e36990_d_n8;

        let assign39260_e36993: f64 = if p.p282 == 1.0 { 1.0 } else { 0.0 };
        var_guard450 = assign39260_e36993;

        let (assign39270_e37001, assign39270_e37001_d_n4, assign39270_e37001_d_n8, assign39270_e37001_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__return, var_fn451_calc_ig__return_dn4, var_fn451_calc_ig__return_dn8, var_fn451_calc_ig__return_dn9,)
    }
};
        var_fn451_calc_ig__return = assign39270_e37001;
        var_fn451_calc_ig__return_dn4 = assign39270_e37001_d_n4;
        var_fn451_calc_ig__return_dn8 = assign39270_e37001_d_n8;
        var_fn451_calc_ig__return_dn9 = assign39270_e37001_d_n9;

        let (assign39280_e37009, assign39280_e37009_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__isdiodeout, var_fn451_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn451_calc_ig__isdiodeout = assign39280_e37009;
        var_fn451_calc_ig__isdiodeout_dn4 = assign39280_e37009_d_n4;

        let (assign39290_e37017, assign39290_e37017_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__isrecout, var_fn451_calc_ig__isrecout_dn4,)
    }
};
        var_fn451_calc_ig__isrecout = assign39290_e37017;
        var_fn451_calc_ig__isrecout_dn4 = assign39290_e37017_d_n4;

        let (assign39300_e37027, assign39300_e37027_d_n8, assign39300_e37027_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign39300_e37025: f64 = (p.p6 * (nv8 - nv9));
        (assign39300_e37025, p.p6, (-p.p6),)
    } else {
        (var_fn451_calc_ig__vgin, var_fn451_calc_ig__vgin_dn8, var_fn451_calc_ig__vgin_dn9,)
    }
};
        var_fn451_calc_ig__vgin = assign39300_e37027;
        var_fn451_calc_ig__vgin_dn8 = assign39300_e37027_d_n8;
        var_fn451_calc_ig__vgin_dn9 = assign39300_e37027_d_n9;

        let (assign39310_e37035, assign39310_e37035_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (var_phit, var_phit_dn4,)
    } else {
        (var_fn451_calc_ig__phitin, var_fn451_calc_ig__phitin_dn4,)
    }
};
        var_fn451_calc_ig__phitin = assign39310_e37035;
        var_fn451_calc_ig__phitin_dn4 = assign39310_e37035_d_n4;

        let (assign39320_e37043,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p260,)
    } else {
        (var_fn451_calc_ig__vgsatin,)
    }
};
        var_fn451_calc_ig__vgsatin = assign39320_e37043;

        let (assign39330_e37051,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p262,)
    } else {
        (var_fn451_calc_ig__alphagin,)
    }
};
        var_fn451_calc_ig__alphagin = assign39330_e37051;

        *var_fn445_calc_ig__alpha2_phit_slot = var_fn445_calc_ig__alpha2_phit;
        *var_fn445_calc_ig__alpha2_phit_dn4_slot = var_fn445_calc_ig__alpha2_phit_dn4;
        *var_fn445_calc_ig__expffvarg_slot = var_fn445_calc_ig__expffvarg;
        *var_fn445_calc_ig__expffvarg_dn4_slot = var_fn445_calc_ig__expffvarg_dn4;
        *var_fn445_calc_ig__expffvarg_dn5_slot = var_fn445_calc_ig__expffvarg_dn5;
        *var_fn445_calc_ig__expffvarg_dn8_slot = var_fn445_calc_ig__expffvarg_dn8;
        *var_fn445_calc_ig__expifor_hinj_slot = var_fn445_calc_ig__expifor_hinj;
        *var_fn445_calc_ig__expifor_hinj_dn4_slot = var_fn445_calc_ig__expifor_hinj_dn4;
        *var_fn445_calc_ig__expifor_hinj_dn5_slot = var_fn445_calc_ig__expifor_hinj_dn5;
        *var_fn445_calc_ig__expifor_hinj_dn8_slot = var_fn445_calc_ig__expifor_hinj_dn8;
        *var_fn445_calc_ig__expifor_hinj_vgsat_slot = var_fn445_calc_ig__expifor_hinj_vgsat;
        *var_fn445_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn445_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn445_calc_ig__expiforarg_hinj_slot = var_fn445_calc_ig__expiforarg_hinj;
        *var_fn445_calc_ig__expiforarg_hinj_dn4_slot = var_fn445_calc_ig__expiforarg_hinj_dn4;
        *var_fn445_calc_ig__expiforarg_hinj_dn5_slot = var_fn445_calc_ig__expiforarg_hinj_dn5;
        *var_fn445_calc_ig__expiforarg_hinj_dn8_slot = var_fn445_calc_ig__expiforarg_hinj_dn8;
        *var_fn445_calc_ig__expiforarg_hinj_vgsat_slot = var_fn445_calc_ig__expiforarg_hinj_vgsat;
        *var_fn445_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn445_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn445_calc_ig__expirev_slot = var_fn445_calc_ig__expirev;
        *var_fn445_calc_ig__expirev_dn4_slot = var_fn445_calc_ig__expirev_dn4;
        *var_fn445_calc_ig__expirev_dn5_slot = var_fn445_calc_ig__expirev_dn5;
        *var_fn445_calc_ig__expirev_dn8_slot = var_fn445_calc_ig__expirev_dn8;
        *var_fn445_calc_ig__expirevarg_slot = var_fn445_calc_ig__expirevarg;
        *var_fn445_calc_ig__expirevarg_dn4_slot = var_fn445_calc_ig__expirevarg_dn4;
        *var_fn445_calc_ig__expirevarg_dn5_slot = var_fn445_calc_ig__expirevarg_dn5;
        *var_fn445_calc_ig__expirevarg_dn8_slot = var_fn445_calc_ig__expirevarg_dn8;
        *var_fn445_calc_ig__ffvgin_slot = var_fn445_calc_ig__ffvgin;
        *var_fn445_calc_ig__ffvgin_dn4_slot = var_fn445_calc_ig__ffvgin_dn4;
        *var_fn445_calc_ig__ffvgin_dn5_slot = var_fn445_calc_ig__ffvgin_dn5;
        *var_fn445_calc_ig__ffvgin_dn8_slot = var_fn445_calc_ig__ffvgin_dn8;
        *var_fn445_calc_ig__frecgin_slot = var_fn445_calc_ig__frecgin;
        *var_fn445_calc_ig__frecgin_dn5_slot = var_fn445_calc_ig__frecgin_dn5;
        *var_fn445_calc_ig__frecgin_dn8_slot = var_fn445_calc_ig__frecgin_dn8;
        *var_fn445_calc_ig__igindiode_slot = var_fn445_calc_ig__igindiode;
        *var_fn445_calc_ig__igindiode_dn4_slot = var_fn445_calc_ig__igindiode_dn4;
        *var_fn445_calc_ig__igindiode_dn5_slot = var_fn445_calc_ig__igindiode_dn5;
        *var_fn445_calc_ig__igindiode_dn8_slot = var_fn445_calc_ig__igindiode_dn8;
        *var_fn445_calc_ig__igindiode_hinj_slot = var_fn445_calc_ig__igindiode_hinj;
        *var_fn445_calc_ig__igindiode_hinj_dn4_slot = var_fn445_calc_ig__igindiode_hinj_dn4;
        *var_fn445_calc_ig__igindiode_hinj_dn5_slot = var_fn445_calc_ig__igindiode_hinj_dn5;
        *var_fn445_calc_ig__igindiode_hinj_dn8_slot = var_fn445_calc_ig__igindiode_hinj_dn8;
        *var_fn445_calc_ig__igindiode_hinj_pre_slot = var_fn445_calc_ig__igindiode_hinj_pre;
        *var_fn445_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn445_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn445_calc_ig__igindiode_hinj_vgsat_slot = var_fn445_calc_ig__igindiode_hinj_vgsat;
        *var_fn445_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn445_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn445_calc_ig__iginrec_slot = var_fn445_calc_ig__iginrec;
        *var_fn445_calc_ig__iginrec_dn4_slot = var_fn445_calc_ig__iginrec_dn4;
        *var_fn445_calc_ig__iginrec_dn5_slot = var_fn445_calc_ig__iginrec_dn5;
        *var_fn445_calc_ig__iginrec_dn8_slot = var_fn445_calc_ig__iginrec_dn8;
        *var_fn445_calc_ig__igout_slot = var_fn445_calc_ig__igout;
        *var_fn445_calc_ig__igout_dn4_slot = var_fn445_calc_ig__igout_dn4;
        *var_fn445_calc_ig__igout_dn5_slot = var_fn445_calc_ig__igout_dn5;
        *var_fn445_calc_ig__igout_dn8_slot = var_fn445_calc_ig__igout_dn8;
        *var_fn445_calc_ig__isrecout_slot = var_fn445_calc_ig__isrecout;
        *var_fn445_calc_ig__isrecout_dn4_slot = var_fn445_calc_ig__isrecout_dn4;
        *var_fn445_calc_ig__pg_paramin_hinj_slot = var_fn445_calc_ig__pg_paramin_hinj;
        *var_fn445_calc_ig__return_slot = var_fn445_calc_ig__return;
        *var_fn445_calc_ig__return_dn4_slot = var_fn445_calc_ig__return_dn4;
        *var_fn445_calc_ig__return_dn5_slot = var_fn445_calc_ig__return_dn5;
        *var_fn445_calc_ig__return_dn8_slot = var_fn445_calc_ig__return_dn8;
        *var_fn451_calc_ig__alphagin_slot = var_fn451_calc_ig__alphagin;
        *var_fn451_calc_ig__isdiodeout_slot = var_fn451_calc_ig__isdiodeout;
        *var_fn451_calc_ig__isdiodeout_dn4_slot = var_fn451_calc_ig__isdiodeout_dn4;
        *var_fn451_calc_ig__isrecout_slot = var_fn451_calc_ig__isrecout;
        *var_fn451_calc_ig__isrecout_dn4_slot = var_fn451_calc_ig__isrecout_dn4;
        *var_fn451_calc_ig__phitin_slot = var_fn451_calc_ig__phitin;
        *var_fn451_calc_ig__phitin_dn4_slot = var_fn451_calc_ig__phitin_dn4;
        *var_fn451_calc_ig__return_slot = var_fn451_calc_ig__return;
        *var_fn451_calc_ig__return_dn4_slot = var_fn451_calc_ig__return_dn4;
        *var_fn451_calc_ig__return_dn8_slot = var_fn451_calc_ig__return_dn8;
        *var_fn451_calc_ig__return_dn9_slot = var_fn451_calc_ig__return_dn9;
        *var_fn451_calc_ig__vgin_slot = var_fn451_calc_ig__vgin;
        *var_fn451_calc_ig__vgin_dn8_slot = var_fn451_calc_ig__vgin_dn8;
        *var_fn451_calc_ig__vgin_dn9_slot = var_fn451_calc_ig__vgin_dn9;
        *var_fn451_calc_ig__vgsatin_slot = var_fn451_calc_ig__vgsatin;
        *var_guard448_slot = var_guard448;
        *var_guard449_slot = var_guard449;
        *var_guard450_slot = var_guard450;
        *var_igdidb_slot = var_igdidb;
        *var_igdidb_dn4_slot = var_igdidb_dn4;
        *var_igdidb_dn5_slot = var_igdidb_dn5;
        *var_igdidb_dn8_slot = var_igdidb_dn8;
    }

    pub(super) fn stamp_transient_block_98(
        p: &Parameters,
        var_guard417: f64,
        var_guard439: f64,
        var_guard450: f64,
        var_tfacdiode: f64,
        var_tfacdiode_dn4: f64,
        var_fn451_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn451_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn451_calc_ig__betarecin_slot: &mut f64,
        var_fn451_calc_ig__expbd1_slot: &mut f64,
        var_fn451_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn451_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn451_calc_ig__expbd1_dn9_slot: &mut f64,
        var_fn451_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn451_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__expbd2_slot: &mut f64,
        var_fn451_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn451_calc_ig__expbdarg1_slot: &mut f64,
        var_fn451_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn451_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn451_calc_ig__expbdarg1_dn9_slot: &mut f64,
        var_fn451_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn451_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__expbdarg2_slot: &mut f64,
        var_fn451_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn451_calc_ig__expffvarg_slot: &mut f64,
        var_fn451_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn451_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn451_calc_ig__expffvarg_dn9_slot: &mut f64,
        var_fn451_calc_ig__expifor_slot: &mut f64,
        var_fn451_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn451_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn451_calc_ig__expifor_dn9_slot: &mut f64,
        var_fn451_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn451_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn451_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn451_calc_ig__expifor_hinj_dn9_slot: &mut f64,
        var_fn451_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn451_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn451_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_dn9_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_hinj_dn9_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__expirev_slot: &mut f64,
        var_fn451_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn451_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn451_calc_ig__expirev_dn9_slot: &mut f64,
        var_fn451_calc_ig__expirevarg_slot: &mut f64,
        var_fn451_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn451_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn451_calc_ig__expirevarg_dn9_slot: &mut f64,
        var_fn451_calc_ig__expphib_slot: &mut f64,
        var_fn451_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn451_calc_ig__ffvgin_slot: &mut f64,
        var_fn451_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn451_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn451_calc_ig__ffvgin_dn9_slot: &mut f64,
        var_fn451_calc_ig__fracin_slot: &mut f64,
        var_fn451_calc_ig__frecgin_slot: &mut f64,
        var_fn451_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn451_calc_ig__frecgin_dn9_slot: &mut f64,
        var_fn451_calc_ig__iginbd_slot: &mut f64,
        var_fn451_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn451_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn451_calc_ig__iginbd_dn9_slot: &mut f64,
        var_fn451_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn451_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__igindiode_slot: &mut f64,
        var_fn451_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn451_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn451_calc_ig__igindiode_dn9_slot: &mut f64,
        var_fn451_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn451_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn451_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn451_calc_ig__igindiode_hinj_dn9_slot: &mut f64,
        var_fn451_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn451_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn451_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn451_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn451_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn451_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn451_calc_ig__igindiode_nohinj_dn9_slot: &mut f64,
        var_fn451_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__iginrec_slot: &mut f64,
        var_fn451_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn451_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn451_calc_ig__iginrec_dn9_slot: &mut f64,
        var_fn451_calc_ig__igout_slot: &mut f64,
        var_fn451_calc_ig__igout_dn4_slot: &mut f64,
        var_fn451_calc_ig__igout_dn8_slot: &mut f64,
        var_fn451_calc_ig__igout_dn9_slot: &mut f64,
        var_fn451_calc_ig__ijin_slot: &mut f64,
        var_fn451_calc_ig__irecin_slot: &mut f64,
        var_fn451_calc_ig__kbdgatein_slot: &mut f64,
        var_fn451_calc_ig__ngf_slot: &mut f64,
        var_fn451_calc_ig__pbdgin_slot: &mut f64,
        var_fn451_calc_ig__pg_param1_slot: &mut f64,
        var_fn451_calc_ig__pg_paramin_slot: &mut f64,
        var_fn451_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_fn451_calc_ig__pgsrecin_slot: &mut f64,
        var_fn451_calc_ig__t0_slot: &mut f64,
        var_fn451_calc_ig__t0_dn4_slot: &mut f64,
        var_fn451_calc_ig__tfacdiodein_slot: &mut f64,
        var_fn451_calc_ig__tfacdiodein_dn4_slot: &mut f64,
        var_fn451_calc_ig__type_slot: &mut f64,
        var_fn451_calc_ig__vbdgin_slot: &mut f64,
        var_fn451_calc_ig__vgsatqin_slot: &mut f64,
        var_fn451_calc_ig__vjg_slot: &mut f64,
        var_fn451_calc_ig__w_slot: &mut f64,
    ) {
        let mut var_fn451_calc_ig__alpha2_phit: f64 = *var_fn451_calc_ig__alpha2_phit_slot;
        let mut var_fn451_calc_ig__alpha2_phit_dn4: f64 = *var_fn451_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn451_calc_ig__betarecin: f64 = *var_fn451_calc_ig__betarecin_slot;
        let mut var_fn451_calc_ig__expbd1: f64 = *var_fn451_calc_ig__expbd1_slot;
        let mut var_fn451_calc_ig__expbd1_dn4: f64 = *var_fn451_calc_ig__expbd1_dn4_slot;
        let mut var_fn451_calc_ig__expbd1_dn8: f64 = *var_fn451_calc_ig__expbd1_dn8_slot;
        let mut var_fn451_calc_ig__expbd1_dn9: f64 = *var_fn451_calc_ig__expbd1_dn9_slot;
        let mut var_fn451_calc_ig__expbd1_vgsat: f64 = *var_fn451_calc_ig__expbd1_vgsat_slot;
        let mut var_fn451_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn451_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__expbd2: f64 = *var_fn451_calc_ig__expbd2_slot;
        let mut var_fn451_calc_ig__expbd2_dn4: f64 = *var_fn451_calc_ig__expbd2_dn4_slot;
        let mut var_fn451_calc_ig__expbdarg1: f64 = *var_fn451_calc_ig__expbdarg1_slot;
        let mut var_fn451_calc_ig__expbdarg1_dn4: f64 = *var_fn451_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn451_calc_ig__expbdarg1_dn8: f64 = *var_fn451_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn451_calc_ig__expbdarg1_dn9: f64 = *var_fn451_calc_ig__expbdarg1_dn9_slot;
        let mut var_fn451_calc_ig__expbdarg1_vgsat: f64 = *var_fn451_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn451_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn451_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__expbdarg2: f64 = *var_fn451_calc_ig__expbdarg2_slot;
        let mut var_fn451_calc_ig__expbdarg2_dn4: f64 = *var_fn451_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn451_calc_ig__expffvarg: f64 = *var_fn451_calc_ig__expffvarg_slot;
        let mut var_fn451_calc_ig__expffvarg_dn4: f64 = *var_fn451_calc_ig__expffvarg_dn4_slot;
        let mut var_fn451_calc_ig__expffvarg_dn8: f64 = *var_fn451_calc_ig__expffvarg_dn8_slot;
        let mut var_fn451_calc_ig__expffvarg_dn9: f64 = *var_fn451_calc_ig__expffvarg_dn9_slot;
        let mut var_fn451_calc_ig__expifor: f64 = *var_fn451_calc_ig__expifor_slot;
        let mut var_fn451_calc_ig__expifor_dn4: f64 = *var_fn451_calc_ig__expifor_dn4_slot;
        let mut var_fn451_calc_ig__expifor_dn8: f64 = *var_fn451_calc_ig__expifor_dn8_slot;
        let mut var_fn451_calc_ig__expifor_dn9: f64 = *var_fn451_calc_ig__expifor_dn9_slot;
        let mut var_fn451_calc_ig__expifor_hinj: f64 = *var_fn451_calc_ig__expifor_hinj_slot;
        let mut var_fn451_calc_ig__expifor_hinj_dn4: f64 = *var_fn451_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn451_calc_ig__expifor_hinj_dn8: f64 = *var_fn451_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn451_calc_ig__expifor_hinj_dn9: f64 = *var_fn451_calc_ig__expifor_hinj_dn9_slot;
        let mut var_fn451_calc_ig__expifor_hinj_vgsat: f64 = *var_fn451_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn451_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn451_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn451_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn451_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn451_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__expiforarg: f64 = *var_fn451_calc_ig__expiforarg_slot;
        let mut var_fn451_calc_ig__expiforarg_dn4: f64 = *var_fn451_calc_ig__expiforarg_dn4_slot;
        let mut var_fn451_calc_ig__expiforarg_dn8: f64 = *var_fn451_calc_ig__expiforarg_dn8_slot;
        let mut var_fn451_calc_ig__expiforarg_dn9: f64 = *var_fn451_calc_ig__expiforarg_dn9_slot;
        let mut var_fn451_calc_ig__expiforarg_hinj: f64 = *var_fn451_calc_ig__expiforarg_hinj_slot;
        let mut var_fn451_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn451_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn451_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn451_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn451_calc_ig__expiforarg_hinj_dn9: f64 = *var_fn451_calc_ig__expiforarg_hinj_dn9_slot;
        let mut var_fn451_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn451_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn451_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn451_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn451_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn451_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn451_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__expirev: f64 = *var_fn451_calc_ig__expirev_slot;
        let mut var_fn451_calc_ig__expirev_dn4: f64 = *var_fn451_calc_ig__expirev_dn4_slot;
        let mut var_fn451_calc_ig__expirev_dn8: f64 = *var_fn451_calc_ig__expirev_dn8_slot;
        let mut var_fn451_calc_ig__expirev_dn9: f64 = *var_fn451_calc_ig__expirev_dn9_slot;
        let mut var_fn451_calc_ig__expirevarg: f64 = *var_fn451_calc_ig__expirevarg_slot;
        let mut var_fn451_calc_ig__expirevarg_dn4: f64 = *var_fn451_calc_ig__expirevarg_dn4_slot;
        let mut var_fn451_calc_ig__expirevarg_dn8: f64 = *var_fn451_calc_ig__expirevarg_dn8_slot;
        let mut var_fn451_calc_ig__expirevarg_dn9: f64 = *var_fn451_calc_ig__expirevarg_dn9_slot;
        let mut var_fn451_calc_ig__expphib: f64 = *var_fn451_calc_ig__expphib_slot;
        let mut var_fn451_calc_ig__expphib_dn4: f64 = *var_fn451_calc_ig__expphib_dn4_slot;
        let mut var_fn451_calc_ig__ffvgin: f64 = *var_fn451_calc_ig__ffvgin_slot;
        let mut var_fn451_calc_ig__ffvgin_dn4: f64 = *var_fn451_calc_ig__ffvgin_dn4_slot;
        let mut var_fn451_calc_ig__ffvgin_dn8: f64 = *var_fn451_calc_ig__ffvgin_dn8_slot;
        let mut var_fn451_calc_ig__ffvgin_dn9: f64 = *var_fn451_calc_ig__ffvgin_dn9_slot;
        let mut var_fn451_calc_ig__fracin: f64 = *var_fn451_calc_ig__fracin_slot;
        let mut var_fn451_calc_ig__frecgin: f64 = *var_fn451_calc_ig__frecgin_slot;
        let mut var_fn451_calc_ig__frecgin_dn8: f64 = *var_fn451_calc_ig__frecgin_dn8_slot;
        let mut var_fn451_calc_ig__frecgin_dn9: f64 = *var_fn451_calc_ig__frecgin_dn9_slot;
        let mut var_fn451_calc_ig__iginbd: f64 = *var_fn451_calc_ig__iginbd_slot;
        let mut var_fn451_calc_ig__iginbd_dn4: f64 = *var_fn451_calc_ig__iginbd_dn4_slot;
        let mut var_fn451_calc_ig__iginbd_dn8: f64 = *var_fn451_calc_ig__iginbd_dn8_slot;
        let mut var_fn451_calc_ig__iginbd_dn9: f64 = *var_fn451_calc_ig__iginbd_dn9_slot;
        let mut var_fn451_calc_ig__iginbd_vgsat: f64 = *var_fn451_calc_ig__iginbd_vgsat_slot;
        let mut var_fn451_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn451_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__igindiode: f64 = *var_fn451_calc_ig__igindiode_slot;
        let mut var_fn451_calc_ig__igindiode_dn4: f64 = *var_fn451_calc_ig__igindiode_dn4_slot;
        let mut var_fn451_calc_ig__igindiode_dn8: f64 = *var_fn451_calc_ig__igindiode_dn8_slot;
        let mut var_fn451_calc_ig__igindiode_dn9: f64 = *var_fn451_calc_ig__igindiode_dn9_slot;
        let mut var_fn451_calc_ig__igindiode_hinj: f64 = *var_fn451_calc_ig__igindiode_hinj_slot;
        let mut var_fn451_calc_ig__igindiode_hinj_dn4: f64 = *var_fn451_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn451_calc_ig__igindiode_hinj_dn8: f64 = *var_fn451_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn451_calc_ig__igindiode_hinj_dn9: f64 = *var_fn451_calc_ig__igindiode_hinj_dn9_slot;
        let mut var_fn451_calc_ig__igindiode_hinj_pre: f64 = *var_fn451_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn451_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn451_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn451_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn451_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn451_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn451_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__igindiode_nohinj: f64 = *var_fn451_calc_ig__igindiode_nohinj_slot;
        let mut var_fn451_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn451_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn451_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn451_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn451_calc_ig__igindiode_nohinj_dn9: f64 = *var_fn451_calc_ig__igindiode_nohinj_dn9_slot;
        let mut var_fn451_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn451_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__iginrec: f64 = *var_fn451_calc_ig__iginrec_slot;
        let mut var_fn451_calc_ig__iginrec_dn4: f64 = *var_fn451_calc_ig__iginrec_dn4_slot;
        let mut var_fn451_calc_ig__iginrec_dn8: f64 = *var_fn451_calc_ig__iginrec_dn8_slot;
        let mut var_fn451_calc_ig__iginrec_dn9: f64 = *var_fn451_calc_ig__iginrec_dn9_slot;
        let mut var_fn451_calc_ig__igout: f64 = *var_fn451_calc_ig__igout_slot;
        let mut var_fn451_calc_ig__igout_dn4: f64 = *var_fn451_calc_ig__igout_dn4_slot;
        let mut var_fn451_calc_ig__igout_dn8: f64 = *var_fn451_calc_ig__igout_dn8_slot;
        let mut var_fn451_calc_ig__igout_dn9: f64 = *var_fn451_calc_ig__igout_dn9_slot;
        let mut var_fn451_calc_ig__ijin: f64 = *var_fn451_calc_ig__ijin_slot;
        let mut var_fn451_calc_ig__irecin: f64 = *var_fn451_calc_ig__irecin_slot;
        let mut var_fn451_calc_ig__kbdgatein: f64 = *var_fn451_calc_ig__kbdgatein_slot;
        let mut var_fn451_calc_ig__ngf: f64 = *var_fn451_calc_ig__ngf_slot;
        let mut var_fn451_calc_ig__pbdgin: f64 = *var_fn451_calc_ig__pbdgin_slot;
        let mut var_fn451_calc_ig__pg_param1: f64 = *var_fn451_calc_ig__pg_param1_slot;
        let mut var_fn451_calc_ig__pg_paramin: f64 = *var_fn451_calc_ig__pg_paramin_slot;
        let mut var_fn451_calc_ig__pg_paramin_hinj: f64 = *var_fn451_calc_ig__pg_paramin_hinj_slot;
        let mut var_fn451_calc_ig__pgsrecin: f64 = *var_fn451_calc_ig__pgsrecin_slot;
        let mut var_fn451_calc_ig__t0: f64 = *var_fn451_calc_ig__t0_slot;
        let mut var_fn451_calc_ig__t0_dn4: f64 = *var_fn451_calc_ig__t0_dn4_slot;
        let mut var_fn451_calc_ig__tfacdiodein: f64 = *var_fn451_calc_ig__tfacdiodein_slot;
        let mut var_fn451_calc_ig__tfacdiodein_dn4: f64 = *var_fn451_calc_ig__tfacdiodein_dn4_slot;
        let mut var_fn451_calc_ig__type: f64 = *var_fn451_calc_ig__type_slot;
        let mut var_fn451_calc_ig__vbdgin: f64 = *var_fn451_calc_ig__vbdgin_slot;
        let mut var_fn451_calc_ig__vgsatqin: f64 = *var_fn451_calc_ig__vgsatqin_slot;
        let mut var_fn451_calc_ig__vjg: f64 = *var_fn451_calc_ig__vjg_slot;
        let mut var_fn451_calc_ig__w: f64 = *var_fn451_calc_ig__w_slot;

        let (assign39340_e37059,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (1.0,)
    } else {
        (var_fn451_calc_ig__fracin,)
    }
};
        var_fn451_calc_ig__fracin = assign39340_e37059;

        let (assign39350_e37067,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p258,)
    } else {
        (var_fn451_calc_ig__pg_paramin,)
    }
};
        var_fn451_calc_ig__pg_paramin = assign39350_e37067;

        let (assign39360_e37075,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p278,)
    } else {
        (var_fn451_calc_ig__pbdgin,)
    }
};
        var_fn451_calc_ig__pbdgin = assign39360_e37075;

        let (assign39370_e37083,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p277,)
    } else {
        (var_fn451_calc_ig__vbdgin,)
    }
};
        var_fn451_calc_ig__vbdgin = assign39370_e37083;

        let (assign39380_e37091, assign39380_e37091_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (var_tfacdiode, var_tfacdiode_dn4,)
    } else {
        (var_fn451_calc_ig__tfacdiodein, var_fn451_calc_ig__tfacdiodein_dn4,)
    }
};
        var_fn451_calc_ig__tfacdiodein = assign39380_e37091;
        var_fn451_calc_ig__tfacdiodein_dn4 = assign39380_e37091_d_n4;

        let (assign39390_e37099,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p0,)
    } else {
        (var_fn451_calc_ig__w,)
    }
};
        var_fn451_calc_ig__w = assign39390_e37099;

        let (assign39400_e37107,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p2,)
    } else {
        (var_fn451_calc_ig__ngf,)
    }
};
        var_fn451_calc_ig__ngf = assign39400_e37107;

        let (assign39410_e37115,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0,)
    } else {
        (var_fn451_calc_ig__ijin,)
    }
};
        var_fn451_calc_ig__ijin = assign39410_e37115;

        let (assign39420_e37123,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0,)
    } else {
        (var_fn451_calc_ig__kbdgatein,)
    }
};
        var_fn451_calc_ig__kbdgatein = assign39420_e37123;

        let (assign39430_e37131,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p285,)
    } else {
        (var_fn451_calc_ig__vgsatqin,)
    }
};
        var_fn451_calc_ig__vgsatqin = assign39430_e37131;

        let (assign39440_e37139,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p286,)
    } else {
        (var_fn451_calc_ig__betarecin,)
    }
};
        var_fn451_calc_ig__betarecin = assign39440_e37139;

        let (assign39450_e37149,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign39450_e37147: f64 = (p.p255 * p.p284);
        (assign39450_e37147,)
    } else {
        (var_fn451_calc_ig__irecin,)
    }
};
        var_fn451_calc_ig__irecin = assign39450_e37149;

        let (assign39460_e37157,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p283,)
    } else {
        (var_fn451_calc_ig__pgsrecin,)
    }
};
        var_fn451_calc_ig__pgsrecin = assign39460_e37157;

        let (assign39470_e37165,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p257,)
    } else {
        (var_fn451_calc_ig__pg_param1,)
    }
};
        var_fn451_calc_ig__pg_param1 = assign39470_e37165;

        let (assign39480_e37173,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p256,)
    } else {
        (var_fn451_calc_ig__vjg,)
    }
};
        var_fn451_calc_ig__vjg = assign39480_e37173;

        let (assign39490_e37181,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p6,)
    } else {
        (var_fn451_calc_ig__type,)
    }
};
        var_fn451_calc_ig__type = assign39490_e37181;

        let (assign39500_e37189, assign39500_e37189_d_n4, assign39500_e37189_d_n8, assign39500_e37189_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__igout, var_fn451_calc_ig__igout_dn4, var_fn451_calc_ig__igout_dn8, var_fn451_calc_ig__igout_dn9,)
    }
};
        var_fn451_calc_ig__igout = assign39500_e37189;
        var_fn451_calc_ig__igout_dn4 = assign39500_e37189_d_n4;
        var_fn451_calc_ig__igout_dn8 = assign39500_e37189_d_n8;
        var_fn451_calc_ig__igout_dn9 = assign39500_e37189_d_n9;

        let (assign39510_e37197, assign39510_e37197_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__alpha2_phit, var_fn451_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn451_calc_ig__alpha2_phit = assign39510_e37197;
        var_fn451_calc_ig__alpha2_phit_dn4 = assign39510_e37197_d_n4;

        let (assign39520_e37205, assign39520_e37205_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__t0, var_fn451_calc_ig__t0_dn4,)
    }
};
        var_fn451_calc_ig__t0 = assign39520_e37205;
        var_fn451_calc_ig__t0_dn4 = assign39520_e37205_d_n4;

        let (assign39530_e37213, assign39530_e37213_d_n4, assign39530_e37213_d_n8, assign39530_e37213_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__ffvgin, var_fn451_calc_ig__ffvgin_dn4, var_fn451_calc_ig__ffvgin_dn8, var_fn451_calc_ig__ffvgin_dn9,)
    }
};
        var_fn451_calc_ig__ffvgin = assign39530_e37213;
        var_fn451_calc_ig__ffvgin_dn4 = assign39530_e37213_d_n4;
        var_fn451_calc_ig__ffvgin_dn8 = assign39530_e37213_d_n8;
        var_fn451_calc_ig__ffvgin_dn9 = assign39530_e37213_d_n9;

        let (assign39540_e37221, assign39540_e37221_d_n4, assign39540_e37221_d_n8, assign39540_e37221_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__iginbd, var_fn451_calc_ig__iginbd_dn4, var_fn451_calc_ig__iginbd_dn8, var_fn451_calc_ig__iginbd_dn9,)
    }
};
        var_fn451_calc_ig__iginbd = assign39540_e37221;
        var_fn451_calc_ig__iginbd_dn4 = assign39540_e37221_d_n4;
        var_fn451_calc_ig__iginbd_dn8 = assign39540_e37221_d_n8;
        var_fn451_calc_ig__iginbd_dn9 = assign39540_e37221_d_n9;

        let (assign39550_e37229, assign39550_e37229_d_n4, assign39550_e37229_d_n8, assign39550_e37229_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__igindiode, var_fn451_calc_ig__igindiode_dn4, var_fn451_calc_ig__igindiode_dn8, var_fn451_calc_ig__igindiode_dn9,)
    }
};
        var_fn451_calc_ig__igindiode = assign39550_e37229;
        var_fn451_calc_ig__igindiode_dn4 = assign39550_e37229_d_n4;
        var_fn451_calc_ig__igindiode_dn8 = assign39550_e37229_d_n8;
        var_fn451_calc_ig__igindiode_dn9 = assign39550_e37229_d_n9;

        let (assign39560_e37237, assign39560_e37237_d_n8, assign39560_e37237_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__frecgin, var_fn451_calc_ig__frecgin_dn8, var_fn451_calc_ig__frecgin_dn9,)
    }
};
        var_fn451_calc_ig__frecgin = assign39560_e37237;
        var_fn451_calc_ig__frecgin_dn8 = assign39560_e37237_d_n8;
        var_fn451_calc_ig__frecgin_dn9 = assign39560_e37237_d_n9;

        let (assign39570_e37245, assign39570_e37245_d_n4, assign39570_e37245_d_n8, assign39570_e37245_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__iginrec, var_fn451_calc_ig__iginrec_dn4, var_fn451_calc_ig__iginrec_dn8, var_fn451_calc_ig__iginrec_dn9,)
    }
};
        var_fn451_calc_ig__iginrec = assign39570_e37245;
        var_fn451_calc_ig__iginrec_dn4 = assign39570_e37245_d_n4;
        var_fn451_calc_ig__iginrec_dn8 = assign39570_e37245_d_n8;
        var_fn451_calc_ig__iginrec_dn9 = assign39570_e37245_d_n9;

        let (assign39580_e37253, assign39580_e37253_d_n4, assign39580_e37253_d_n8, assign39580_e37253_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expbdarg1, var_fn451_calc_ig__expbdarg1_dn4, var_fn451_calc_ig__expbdarg1_dn8, var_fn451_calc_ig__expbdarg1_dn9,)
    }
};
        var_fn451_calc_ig__expbdarg1 = assign39580_e37253;
        var_fn451_calc_ig__expbdarg1_dn4 = assign39580_e37253_d_n4;
        var_fn451_calc_ig__expbdarg1_dn8 = assign39580_e37253_d_n8;
        var_fn451_calc_ig__expbdarg1_dn9 = assign39580_e37253_d_n9;

        let (assign39590_e37261, assign39590_e37261_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expbdarg2, var_fn451_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn451_calc_ig__expbdarg2 = assign39590_e37261;
        var_fn451_calc_ig__expbdarg2_dn4 = assign39590_e37261_d_n4;

        let (assign39600_e37269, assign39600_e37269_d_n4, assign39600_e37269_d_n8, assign39600_e37269_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expbd1, var_fn451_calc_ig__expbd1_dn4, var_fn451_calc_ig__expbd1_dn8, var_fn451_calc_ig__expbd1_dn9,)
    }
};
        var_fn451_calc_ig__expbd1 = assign39600_e37269;
        var_fn451_calc_ig__expbd1_dn4 = assign39600_e37269_d_n4;
        var_fn451_calc_ig__expbd1_dn8 = assign39600_e37269_d_n8;
        var_fn451_calc_ig__expbd1_dn9 = assign39600_e37269_d_n9;

        let (assign39610_e37277, assign39610_e37277_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expbd2, var_fn451_calc_ig__expbd2_dn4,)
    }
};
        var_fn451_calc_ig__expbd2 = assign39610_e37277;
        var_fn451_calc_ig__expbd2_dn4 = assign39610_e37277_d_n4;

        let (assign39620_e37285, assign39620_e37285_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expphib, var_fn451_calc_ig__expphib_dn4,)
    }
};
        var_fn451_calc_ig__expphib = assign39620_e37285;
        var_fn451_calc_ig__expphib_dn4 = assign39620_e37285_d_n4;

        let (assign39630_e37293, assign39630_e37293_d_n4, assign39630_e37293_d_n8, assign39630_e37293_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expffvarg, var_fn451_calc_ig__expffvarg_dn4, var_fn451_calc_ig__expffvarg_dn8, var_fn451_calc_ig__expffvarg_dn9,)
    }
};
        var_fn451_calc_ig__expffvarg = assign39630_e37293;
        var_fn451_calc_ig__expffvarg_dn4 = assign39630_e37293_d_n4;
        var_fn451_calc_ig__expffvarg_dn8 = assign39630_e37293_d_n8;
        var_fn451_calc_ig__expffvarg_dn9 = assign39630_e37293_d_n9;

        let (assign39640_e37301, assign39640_e37301_d_n4, assign39640_e37301_d_n8, assign39640_e37301_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expiforarg, var_fn451_calc_ig__expiforarg_dn4, var_fn451_calc_ig__expiforarg_dn8, var_fn451_calc_ig__expiforarg_dn9,)
    }
};
        var_fn451_calc_ig__expiforarg = assign39640_e37301;
        var_fn451_calc_ig__expiforarg_dn4 = assign39640_e37301_d_n4;
        var_fn451_calc_ig__expiforarg_dn8 = assign39640_e37301_d_n8;
        var_fn451_calc_ig__expiforarg_dn9 = assign39640_e37301_d_n9;

        let (assign39650_e37309, assign39650_e37309_d_n4, assign39650_e37309_d_n8, assign39650_e37309_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expifor, var_fn451_calc_ig__expifor_dn4, var_fn451_calc_ig__expifor_dn8, var_fn451_calc_ig__expifor_dn9,)
    }
};
        var_fn451_calc_ig__expifor = assign39650_e37309;
        var_fn451_calc_ig__expifor_dn4 = assign39650_e37309_d_n4;
        var_fn451_calc_ig__expifor_dn8 = assign39650_e37309_d_n8;
        var_fn451_calc_ig__expifor_dn9 = assign39650_e37309_d_n9;

        let (assign39660_e37317, assign39660_e37317_d_n4, assign39660_e37317_d_n8, assign39660_e37317_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expirevarg, var_fn451_calc_ig__expirevarg_dn4, var_fn451_calc_ig__expirevarg_dn8, var_fn451_calc_ig__expirevarg_dn9,)
    }
};
        var_fn451_calc_ig__expirevarg = assign39660_e37317;
        var_fn451_calc_ig__expirevarg_dn4 = assign39660_e37317_d_n4;
        var_fn451_calc_ig__expirevarg_dn8 = assign39660_e37317_d_n8;
        var_fn451_calc_ig__expirevarg_dn9 = assign39660_e37317_d_n9;

        let (assign39670_e37325, assign39670_e37325_d_n4, assign39670_e37325_d_n8, assign39670_e37325_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expirev, var_fn451_calc_ig__expirev_dn4, var_fn451_calc_ig__expirev_dn8, var_fn451_calc_ig__expirev_dn9,)
    }
};
        var_fn451_calc_ig__expirev = assign39670_e37325;
        var_fn451_calc_ig__expirev_dn4 = assign39670_e37325_d_n4;
        var_fn451_calc_ig__expirev_dn8 = assign39670_e37325_d_n8;
        var_fn451_calc_ig__expirev_dn9 = assign39670_e37325_d_n9;

        let (assign39680_e37333,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0,)
    } else {
        (var_fn451_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn451_calc_ig__pg_paramin_hinj = assign39680_e37333;

        let (assign39690_e37341, assign39690_e37341_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expbdarg1_vgsat, var_fn451_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__expbdarg1_vgsat = assign39690_e37341;
        var_fn451_calc_ig__expbdarg1_vgsat_dn4 = assign39690_e37341_d_n4;

        let (assign39700_e37349, assign39700_e37349_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expbd1_vgsat, var_fn451_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__expbd1_vgsat = assign39700_e37349;
        var_fn451_calc_ig__expbd1_vgsat_dn4 = assign39700_e37349_d_n4;

        let (assign39710_e37357, assign39710_e37357_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__iginbd_vgsat, var_fn451_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__iginbd_vgsat = assign39710_e37357;
        var_fn451_calc_ig__iginbd_vgsat_dn4 = assign39710_e37357_d_n4;

        let (assign39720_e37365, assign39720_e37365_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expiforarg_nohinj_vgsat, var_fn451_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__expiforarg_nohinj_vgsat = assign39720_e37365;
        var_fn451_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign39720_e37365_d_n4;

        let (assign39730_e37373, assign39730_e37373_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expifor_nohinj_vgsat, var_fn451_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__expifor_nohinj_vgsat = assign39730_e37373;
        var_fn451_calc_ig__expifor_nohinj_vgsat_dn4 = assign39730_e37373_d_n4;

        let (assign39740_e37381, assign39740_e37381_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__igindiode_nohinj_vgsat, var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__igindiode_nohinj_vgsat = assign39740_e37381;
        var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4 = assign39740_e37381_d_n4;

        let (assign39750_e37389, assign39750_e37389_d_n4, assign39750_e37389_d_n8, assign39750_e37389_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__igindiode_nohinj, var_fn451_calc_ig__igindiode_nohinj_dn4, var_fn451_calc_ig__igindiode_nohinj_dn8, var_fn451_calc_ig__igindiode_nohinj_dn9,)
    }
};
        var_fn451_calc_ig__igindiode_nohinj = assign39750_e37389;
        var_fn451_calc_ig__igindiode_nohinj_dn4 = assign39750_e37389_d_n4;
        var_fn451_calc_ig__igindiode_nohinj_dn8 = assign39750_e37389_d_n8;
        var_fn451_calc_ig__igindiode_nohinj_dn9 = assign39750_e37389_d_n9;

        let (assign39760_e37397, assign39760_e37397_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expiforarg_hinj_vgsat, var_fn451_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__expiforarg_hinj_vgsat = assign39760_e37397;
        var_fn451_calc_ig__expiforarg_hinj_vgsat_dn4 = assign39760_e37397_d_n4;

        let (assign39770_e37405, assign39770_e37405_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expifor_hinj_vgsat, var_fn451_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__expifor_hinj_vgsat = assign39770_e37405;
        var_fn451_calc_ig__expifor_hinj_vgsat_dn4 = assign39770_e37405_d_n4;

        let (assign39780_e37413, assign39780_e37413_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__igindiode_hinj_vgsat, var_fn451_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__igindiode_hinj_vgsat = assign39780_e37413;
        var_fn451_calc_ig__igindiode_hinj_vgsat_dn4 = assign39780_e37413_d_n4;

        let (assign39790_e37421, assign39790_e37421_d_n4, assign39790_e37421_d_n8, assign39790_e37421_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expiforarg_hinj, var_fn451_calc_ig__expiforarg_hinj_dn4, var_fn451_calc_ig__expiforarg_hinj_dn8, var_fn451_calc_ig__expiforarg_hinj_dn9,)
    }
};
        var_fn451_calc_ig__expiforarg_hinj = assign39790_e37421;
        var_fn451_calc_ig__expiforarg_hinj_dn4 = assign39790_e37421_d_n4;
        var_fn451_calc_ig__expiforarg_hinj_dn8 = assign39790_e37421_d_n8;
        var_fn451_calc_ig__expiforarg_hinj_dn9 = assign39790_e37421_d_n9;

        let (assign39800_e37429, assign39800_e37429_d_n4, assign39800_e37429_d_n8, assign39800_e37429_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__expifor_hinj, var_fn451_calc_ig__expifor_hinj_dn4, var_fn451_calc_ig__expifor_hinj_dn8, var_fn451_calc_ig__expifor_hinj_dn9,)
    }
};
        var_fn451_calc_ig__expifor_hinj = assign39800_e37429;
        var_fn451_calc_ig__expifor_hinj_dn4 = assign39800_e37429_d_n4;
        var_fn451_calc_ig__expifor_hinj_dn8 = assign39800_e37429_d_n8;
        var_fn451_calc_ig__expifor_hinj_dn9 = assign39800_e37429_d_n9;

        let (assign39810_e37437, assign39810_e37437_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__igindiode_hinj_pre, var_fn451_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn451_calc_ig__igindiode_hinj_pre = assign39810_e37437;
        var_fn451_calc_ig__igindiode_hinj_pre_dn4 = assign39810_e37437_d_n4;

        let (assign39820_e37445, assign39820_e37445_d_n4, assign39820_e37445_d_n8, assign39820_e37445_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__igindiode_hinj, var_fn451_calc_ig__igindiode_hinj_dn4, var_fn451_calc_ig__igindiode_hinj_dn8, var_fn451_calc_ig__igindiode_hinj_dn9,)
    }
};
        var_fn451_calc_ig__igindiode_hinj = assign39820_e37445;
        var_fn451_calc_ig__igindiode_hinj_dn4 = assign39820_e37445_d_n4;
        var_fn451_calc_ig__igindiode_hinj_dn8 = assign39820_e37445_d_n8;
        var_fn451_calc_ig__igindiode_hinj_dn9 = assign39820_e37445_d_n9;

        *var_fn451_calc_ig__alpha2_phit_slot = var_fn451_calc_ig__alpha2_phit;
        *var_fn451_calc_ig__alpha2_phit_dn4_slot = var_fn451_calc_ig__alpha2_phit_dn4;
        *var_fn451_calc_ig__betarecin_slot = var_fn451_calc_ig__betarecin;
        *var_fn451_calc_ig__expbd1_slot = var_fn451_calc_ig__expbd1;
        *var_fn451_calc_ig__expbd1_dn4_slot = var_fn451_calc_ig__expbd1_dn4;
        *var_fn451_calc_ig__expbd1_dn8_slot = var_fn451_calc_ig__expbd1_dn8;
        *var_fn451_calc_ig__expbd1_dn9_slot = var_fn451_calc_ig__expbd1_dn9;
        *var_fn451_calc_ig__expbd1_vgsat_slot = var_fn451_calc_ig__expbd1_vgsat;
        *var_fn451_calc_ig__expbd1_vgsat_dn4_slot = var_fn451_calc_ig__expbd1_vgsat_dn4;
        *var_fn451_calc_ig__expbd2_slot = var_fn451_calc_ig__expbd2;
        *var_fn451_calc_ig__expbd2_dn4_slot = var_fn451_calc_ig__expbd2_dn4;
        *var_fn451_calc_ig__expbdarg1_slot = var_fn451_calc_ig__expbdarg1;
        *var_fn451_calc_ig__expbdarg1_dn4_slot = var_fn451_calc_ig__expbdarg1_dn4;
        *var_fn451_calc_ig__expbdarg1_dn8_slot = var_fn451_calc_ig__expbdarg1_dn8;
        *var_fn451_calc_ig__expbdarg1_dn9_slot = var_fn451_calc_ig__expbdarg1_dn9;
        *var_fn451_calc_ig__expbdarg1_vgsat_slot = var_fn451_calc_ig__expbdarg1_vgsat;
        *var_fn451_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn451_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn451_calc_ig__expbdarg2_slot = var_fn451_calc_ig__expbdarg2;
        *var_fn451_calc_ig__expbdarg2_dn4_slot = var_fn451_calc_ig__expbdarg2_dn4;
        *var_fn451_calc_ig__expffvarg_slot = var_fn451_calc_ig__expffvarg;
        *var_fn451_calc_ig__expffvarg_dn4_slot = var_fn451_calc_ig__expffvarg_dn4;
        *var_fn451_calc_ig__expffvarg_dn8_slot = var_fn451_calc_ig__expffvarg_dn8;
        *var_fn451_calc_ig__expffvarg_dn9_slot = var_fn451_calc_ig__expffvarg_dn9;
        *var_fn451_calc_ig__expifor_slot = var_fn451_calc_ig__expifor;
        *var_fn451_calc_ig__expifor_dn4_slot = var_fn451_calc_ig__expifor_dn4;
        *var_fn451_calc_ig__expifor_dn8_slot = var_fn451_calc_ig__expifor_dn8;
        *var_fn451_calc_ig__expifor_dn9_slot = var_fn451_calc_ig__expifor_dn9;
        *var_fn451_calc_ig__expifor_hinj_slot = var_fn451_calc_ig__expifor_hinj;
        *var_fn451_calc_ig__expifor_hinj_dn4_slot = var_fn451_calc_ig__expifor_hinj_dn4;
        *var_fn451_calc_ig__expifor_hinj_dn8_slot = var_fn451_calc_ig__expifor_hinj_dn8;
        *var_fn451_calc_ig__expifor_hinj_dn9_slot = var_fn451_calc_ig__expifor_hinj_dn9;
        *var_fn451_calc_ig__expifor_hinj_vgsat_slot = var_fn451_calc_ig__expifor_hinj_vgsat;
        *var_fn451_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn451_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn451_calc_ig__expifor_nohinj_vgsat_slot = var_fn451_calc_ig__expifor_nohinj_vgsat;
        *var_fn451_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn451_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn451_calc_ig__expiforarg_slot = var_fn451_calc_ig__expiforarg;
        *var_fn451_calc_ig__expiforarg_dn4_slot = var_fn451_calc_ig__expiforarg_dn4;
        *var_fn451_calc_ig__expiforarg_dn8_slot = var_fn451_calc_ig__expiforarg_dn8;
        *var_fn451_calc_ig__expiforarg_dn9_slot = var_fn451_calc_ig__expiforarg_dn9;
        *var_fn451_calc_ig__expiforarg_hinj_slot = var_fn451_calc_ig__expiforarg_hinj;
        *var_fn451_calc_ig__expiforarg_hinj_dn4_slot = var_fn451_calc_ig__expiforarg_hinj_dn4;
        *var_fn451_calc_ig__expiforarg_hinj_dn8_slot = var_fn451_calc_ig__expiforarg_hinj_dn8;
        *var_fn451_calc_ig__expiforarg_hinj_dn9_slot = var_fn451_calc_ig__expiforarg_hinj_dn9;
        *var_fn451_calc_ig__expiforarg_hinj_vgsat_slot = var_fn451_calc_ig__expiforarg_hinj_vgsat;
        *var_fn451_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn451_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn451_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn451_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn451_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn451_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn451_calc_ig__expirev_slot = var_fn451_calc_ig__expirev;
        *var_fn451_calc_ig__expirev_dn4_slot = var_fn451_calc_ig__expirev_dn4;
        *var_fn451_calc_ig__expirev_dn8_slot = var_fn451_calc_ig__expirev_dn8;
        *var_fn451_calc_ig__expirev_dn9_slot = var_fn451_calc_ig__expirev_dn9;
        *var_fn451_calc_ig__expirevarg_slot = var_fn451_calc_ig__expirevarg;
        *var_fn451_calc_ig__expirevarg_dn4_slot = var_fn451_calc_ig__expirevarg_dn4;
        *var_fn451_calc_ig__expirevarg_dn8_slot = var_fn451_calc_ig__expirevarg_dn8;
        *var_fn451_calc_ig__expirevarg_dn9_slot = var_fn451_calc_ig__expirevarg_dn9;
        *var_fn451_calc_ig__expphib_slot = var_fn451_calc_ig__expphib;
        *var_fn451_calc_ig__expphib_dn4_slot = var_fn451_calc_ig__expphib_dn4;
        *var_fn451_calc_ig__ffvgin_slot = var_fn451_calc_ig__ffvgin;
        *var_fn451_calc_ig__ffvgin_dn4_slot = var_fn451_calc_ig__ffvgin_dn4;
        *var_fn451_calc_ig__ffvgin_dn8_slot = var_fn451_calc_ig__ffvgin_dn8;
        *var_fn451_calc_ig__ffvgin_dn9_slot = var_fn451_calc_ig__ffvgin_dn9;
        *var_fn451_calc_ig__fracin_slot = var_fn451_calc_ig__fracin;
        *var_fn451_calc_ig__frecgin_slot = var_fn451_calc_ig__frecgin;
        *var_fn451_calc_ig__frecgin_dn8_slot = var_fn451_calc_ig__frecgin_dn8;
        *var_fn451_calc_ig__frecgin_dn9_slot = var_fn451_calc_ig__frecgin_dn9;
        *var_fn451_calc_ig__iginbd_slot = var_fn451_calc_ig__iginbd;
        *var_fn451_calc_ig__iginbd_dn4_slot = var_fn451_calc_ig__iginbd_dn4;
        *var_fn451_calc_ig__iginbd_dn8_slot = var_fn451_calc_ig__iginbd_dn8;
        *var_fn451_calc_ig__iginbd_dn9_slot = var_fn451_calc_ig__iginbd_dn9;
        *var_fn451_calc_ig__iginbd_vgsat_slot = var_fn451_calc_ig__iginbd_vgsat;
        *var_fn451_calc_ig__iginbd_vgsat_dn4_slot = var_fn451_calc_ig__iginbd_vgsat_dn4;
        *var_fn451_calc_ig__igindiode_slot = var_fn451_calc_ig__igindiode;
        *var_fn451_calc_ig__igindiode_dn4_slot = var_fn451_calc_ig__igindiode_dn4;
        *var_fn451_calc_ig__igindiode_dn8_slot = var_fn451_calc_ig__igindiode_dn8;
        *var_fn451_calc_ig__igindiode_dn9_slot = var_fn451_calc_ig__igindiode_dn9;
        *var_fn451_calc_ig__igindiode_hinj_slot = var_fn451_calc_ig__igindiode_hinj;
        *var_fn451_calc_ig__igindiode_hinj_dn4_slot = var_fn451_calc_ig__igindiode_hinj_dn4;
        *var_fn451_calc_ig__igindiode_hinj_dn8_slot = var_fn451_calc_ig__igindiode_hinj_dn8;
        *var_fn451_calc_ig__igindiode_hinj_dn9_slot = var_fn451_calc_ig__igindiode_hinj_dn9;
        *var_fn451_calc_ig__igindiode_hinj_pre_slot = var_fn451_calc_ig__igindiode_hinj_pre;
        *var_fn451_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn451_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn451_calc_ig__igindiode_hinj_vgsat_slot = var_fn451_calc_ig__igindiode_hinj_vgsat;
        *var_fn451_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn451_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn451_calc_ig__igindiode_nohinj_slot = var_fn451_calc_ig__igindiode_nohinj;
        *var_fn451_calc_ig__igindiode_nohinj_dn4_slot = var_fn451_calc_ig__igindiode_nohinj_dn4;
        *var_fn451_calc_ig__igindiode_nohinj_dn8_slot = var_fn451_calc_ig__igindiode_nohinj_dn8;
        *var_fn451_calc_ig__igindiode_nohinj_dn9_slot = var_fn451_calc_ig__igindiode_nohinj_dn9;
        *var_fn451_calc_ig__igindiode_nohinj_vgsat_slot = var_fn451_calc_ig__igindiode_nohinj_vgsat;
        *var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn451_calc_ig__iginrec_slot = var_fn451_calc_ig__iginrec;
        *var_fn451_calc_ig__iginrec_dn4_slot = var_fn451_calc_ig__iginrec_dn4;
        *var_fn451_calc_ig__iginrec_dn8_slot = var_fn451_calc_ig__iginrec_dn8;
        *var_fn451_calc_ig__iginrec_dn9_slot = var_fn451_calc_ig__iginrec_dn9;
        *var_fn451_calc_ig__igout_slot = var_fn451_calc_ig__igout;
        *var_fn451_calc_ig__igout_dn4_slot = var_fn451_calc_ig__igout_dn4;
        *var_fn451_calc_ig__igout_dn8_slot = var_fn451_calc_ig__igout_dn8;
        *var_fn451_calc_ig__igout_dn9_slot = var_fn451_calc_ig__igout_dn9;
        *var_fn451_calc_ig__ijin_slot = var_fn451_calc_ig__ijin;
        *var_fn451_calc_ig__irecin_slot = var_fn451_calc_ig__irecin;
        *var_fn451_calc_ig__kbdgatein_slot = var_fn451_calc_ig__kbdgatein;
        *var_fn451_calc_ig__ngf_slot = var_fn451_calc_ig__ngf;
        *var_fn451_calc_ig__pbdgin_slot = var_fn451_calc_ig__pbdgin;
        *var_fn451_calc_ig__pg_param1_slot = var_fn451_calc_ig__pg_param1;
        *var_fn451_calc_ig__pg_paramin_slot = var_fn451_calc_ig__pg_paramin;
        *var_fn451_calc_ig__pg_paramin_hinj_slot = var_fn451_calc_ig__pg_paramin_hinj;
        *var_fn451_calc_ig__pgsrecin_slot = var_fn451_calc_ig__pgsrecin;
        *var_fn451_calc_ig__t0_slot = var_fn451_calc_ig__t0;
        *var_fn451_calc_ig__t0_dn4_slot = var_fn451_calc_ig__t0_dn4;
        *var_fn451_calc_ig__tfacdiodein_slot = var_fn451_calc_ig__tfacdiodein;
        *var_fn451_calc_ig__tfacdiodein_dn4_slot = var_fn451_calc_ig__tfacdiodein_dn4;
        *var_fn451_calc_ig__type_slot = var_fn451_calc_ig__type;
        *var_fn451_calc_ig__vbdgin_slot = var_fn451_calc_ig__vbdgin;
        *var_fn451_calc_ig__vgsatqin_slot = var_fn451_calc_ig__vgsatqin;
        *var_fn451_calc_ig__vjg_slot = var_fn451_calc_ig__vjg;
        *var_fn451_calc_ig__w_slot = var_fn451_calc_ig__w;
    }

    pub(super) fn stamp_transient_block_99(
        var_fn451_calc_ig__fracin: f64,
        var_fn451_calc_ig__ijin: f64,
        var_fn451_calc_ig__kbdgatein: f64,
        var_fn451_calc_ig__ngf: f64,
        var_fn451_calc_ig__pbdgin: f64,
        var_fn451_calc_ig__pg_param1: f64,
        var_fn451_calc_ig__pg_paramin: f64,
        var_fn451_calc_ig__phitin: f64,
        var_fn451_calc_ig__phitin_dn4: f64,
        var_fn451_calc_ig__tfacdiodein: f64,
        var_fn451_calc_ig__tfacdiodein_dn4: f64,
        var_fn451_calc_ig__type: f64,
        var_fn451_calc_ig__vbdgin: f64,
        var_fn451_calc_ig__vgin: f64,
        var_fn451_calc_ig__vgin_dn8: f64,
        var_fn451_calc_ig__vgin_dn9: f64,
        var_fn451_calc_ig__vgsatin: f64,
        var_fn451_calc_ig__vjg: f64,
        var_fn451_calc_ig__w: f64,
        var_guard417: f64,
        var_guard439: f64,
        var_guard450: f64,
        var_fn451_calc_ig__expbd1_slot: &mut f64,
        var_fn451_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn451_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn451_calc_ig__expbd1_dn9_slot: &mut f64,
        var_fn451_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn451_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__expbd2_slot: &mut f64,
        var_fn451_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn451_calc_ig__expbdarg1_slot: &mut f64,
        var_fn451_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn451_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn451_calc_ig__expbdarg1_dn9_slot: &mut f64,
        var_fn451_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn451_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__expbdarg2_slot: &mut f64,
        var_fn451_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn451_calc_ig__expifor_slot: &mut f64,
        var_fn451_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn451_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn451_calc_ig__expifor_dn9_slot: &mut f64,
        var_fn451_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn451_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn451_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_dn9_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_hinj_dn9_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn451_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__expphib_slot: &mut f64,
        var_fn451_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn451_calc_ig__iginbd_slot: &mut f64,
        var_fn451_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn451_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn451_calc_ig__iginbd_dn9_slot: &mut f64,
        var_fn451_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn451_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__igindiode_slot: &mut f64,
        var_fn451_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn451_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn451_calc_ig__igindiode_dn9_slot: &mut f64,
        var_fn451_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn451_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn451_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn451_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn451_calc_ig__igindiode_nohinj_dn9_slot: &mut f64,
        var_fn451_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn451_calc_ig__isdiodeout_slot: &mut f64,
        var_fn451_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn451_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_fn451_calc_ig__t0_slot: &mut f64,
        var_fn451_calc_ig__t0_dn4_slot: &mut f64,
        var_guard452_slot: &mut f64,
        var_guard453_slot: &mut f64,
    ) {
        let mut var_fn451_calc_ig__expbd1: f64 = *var_fn451_calc_ig__expbd1_slot;
        let mut var_fn451_calc_ig__expbd1_dn4: f64 = *var_fn451_calc_ig__expbd1_dn4_slot;
        let mut var_fn451_calc_ig__expbd1_dn8: f64 = *var_fn451_calc_ig__expbd1_dn8_slot;
        let mut var_fn451_calc_ig__expbd1_dn9: f64 = *var_fn451_calc_ig__expbd1_dn9_slot;
        let mut var_fn451_calc_ig__expbd1_vgsat: f64 = *var_fn451_calc_ig__expbd1_vgsat_slot;
        let mut var_fn451_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn451_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__expbd2: f64 = *var_fn451_calc_ig__expbd2_slot;
        let mut var_fn451_calc_ig__expbd2_dn4: f64 = *var_fn451_calc_ig__expbd2_dn4_slot;
        let mut var_fn451_calc_ig__expbdarg1: f64 = *var_fn451_calc_ig__expbdarg1_slot;
        let mut var_fn451_calc_ig__expbdarg1_dn4: f64 = *var_fn451_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn451_calc_ig__expbdarg1_dn8: f64 = *var_fn451_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn451_calc_ig__expbdarg1_dn9: f64 = *var_fn451_calc_ig__expbdarg1_dn9_slot;
        let mut var_fn451_calc_ig__expbdarg1_vgsat: f64 = *var_fn451_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn451_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn451_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__expbdarg2: f64 = *var_fn451_calc_ig__expbdarg2_slot;
        let mut var_fn451_calc_ig__expbdarg2_dn4: f64 = *var_fn451_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn451_calc_ig__expifor: f64 = *var_fn451_calc_ig__expifor_slot;
        let mut var_fn451_calc_ig__expifor_dn4: f64 = *var_fn451_calc_ig__expifor_dn4_slot;
        let mut var_fn451_calc_ig__expifor_dn8: f64 = *var_fn451_calc_ig__expifor_dn8_slot;
        let mut var_fn451_calc_ig__expifor_dn9: f64 = *var_fn451_calc_ig__expifor_dn9_slot;
        let mut var_fn451_calc_ig__expifor_hinj_vgsat: f64 = *var_fn451_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn451_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn451_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn451_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn451_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn451_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__expiforarg: f64 = *var_fn451_calc_ig__expiforarg_slot;
        let mut var_fn451_calc_ig__expiforarg_dn4: f64 = *var_fn451_calc_ig__expiforarg_dn4_slot;
        let mut var_fn451_calc_ig__expiforarg_dn8: f64 = *var_fn451_calc_ig__expiforarg_dn8_slot;
        let mut var_fn451_calc_ig__expiforarg_dn9: f64 = *var_fn451_calc_ig__expiforarg_dn9_slot;
        let mut var_fn451_calc_ig__expiforarg_hinj: f64 = *var_fn451_calc_ig__expiforarg_hinj_slot;
        let mut var_fn451_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn451_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn451_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn451_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn451_calc_ig__expiforarg_hinj_dn9: f64 = *var_fn451_calc_ig__expiforarg_hinj_dn9_slot;
        let mut var_fn451_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn451_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn451_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn451_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn451_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn451_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn451_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__expphib: f64 = *var_fn451_calc_ig__expphib_slot;
        let mut var_fn451_calc_ig__expphib_dn4: f64 = *var_fn451_calc_ig__expphib_dn4_slot;
        let mut var_fn451_calc_ig__iginbd: f64 = *var_fn451_calc_ig__iginbd_slot;
        let mut var_fn451_calc_ig__iginbd_dn4: f64 = *var_fn451_calc_ig__iginbd_dn4_slot;
        let mut var_fn451_calc_ig__iginbd_dn8: f64 = *var_fn451_calc_ig__iginbd_dn8_slot;
        let mut var_fn451_calc_ig__iginbd_dn9: f64 = *var_fn451_calc_ig__iginbd_dn9_slot;
        let mut var_fn451_calc_ig__iginbd_vgsat: f64 = *var_fn451_calc_ig__iginbd_vgsat_slot;
        let mut var_fn451_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn451_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__igindiode: f64 = *var_fn451_calc_ig__igindiode_slot;
        let mut var_fn451_calc_ig__igindiode_dn4: f64 = *var_fn451_calc_ig__igindiode_dn4_slot;
        let mut var_fn451_calc_ig__igindiode_dn8: f64 = *var_fn451_calc_ig__igindiode_dn8_slot;
        let mut var_fn451_calc_ig__igindiode_dn9: f64 = *var_fn451_calc_ig__igindiode_dn9_slot;
        let mut var_fn451_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn451_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn451_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn451_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__igindiode_nohinj: f64 = *var_fn451_calc_ig__igindiode_nohinj_slot;
        let mut var_fn451_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn451_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn451_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn451_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn451_calc_ig__igindiode_nohinj_dn9: f64 = *var_fn451_calc_ig__igindiode_nohinj_dn9_slot;
        let mut var_fn451_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn451_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn451_calc_ig__isdiodeout: f64 = *var_fn451_calc_ig__isdiodeout_slot;
        let mut var_fn451_calc_ig__isdiodeout_dn4: f64 = *var_fn451_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn451_calc_ig__pg_paramin_hinj: f64 = *var_fn451_calc_ig__pg_paramin_hinj_slot;
        let mut var_fn451_calc_ig__t0: f64 = *var_fn451_calc_ig__t0_slot;
        let mut var_fn451_calc_ig__t0_dn4: f64 = *var_fn451_calc_ig__t0_dn4_slot;
        let mut var_guard452: f64 = *var_guard452_slot;
        let mut var_guard453: f64 = *var_guard453_slot;

        let (assign39830_e37458, assign39830_e37458_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign39830_e37453: f64 = (var_fn451_calc_ig__pg_param1 / var_fn451_calc_ig__phitin);
        let assign39830_e37455: f64 = (-var_fn451_calc_ig__vjg);
        let assign39830_e37456: f64 = (assign39830_e37453 * assign39830_e37455);
        (assign39830_e37456, ((-((var_fn451_calc_ig__pg_param1 * var_fn451_calc_ig__phitin_dn4) / (var_fn451_calc_ig__phitin * var_fn451_calc_ig__phitin))) * assign39830_e37455),)
    } else {
        (var_fn451_calc_ig__expphib, var_fn451_calc_ig__expphib_dn4,)
    }
};
        var_fn451_calc_ig__expphib = assign39830_e37458;
        var_fn451_calc_ig__expphib_dn4 = assign39830_e37458_d_n4;

        let (assign39840_e37504, assign39840_e37504_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign39840_e37470: f64 = (-50.0);
        let (assign39840_e37502, assign39840_e37502_d_n4,) = {
            if ((!(var_fn451_calc_ig__expphib > 50.0)) && (!(var_fn451_calc_ig__expphib < assign39840_e37470))) {
                let assign39840_e37475: f64 = (var_fn451_calc_ig__expphib).exp();
                (assign39840_e37475, (assign39840_e37475 * var_fn451_calc_ig__expphib_dn4),)
            } else {
                let assign39840_e37482: f64 = (-50.0);
                let (assign39840_e37501, assign39840_e37501_d_n4,) = {
                    if ((!(var_fn451_calc_ig__expphib > 50.0)) && (var_fn451_calc_ig__expphib < assign39840_e37482)) {
                        let assign39840_e37486: f64 = (-50.0);
                        let assign39840_e37487: f64 = (assign39840_e37486).exp();
                        (assign39840_e37487, 0.0,)
                    } else {
                        let (assign39840_e37500, assign39840_e37500_d_n4,) = {
                            if (var_fn451_calc_ig__expphib > 50.0) {
                                let assign39840_e37492: f64 = (50.0_f64).exp();
                                let assign39840_e37496: f64 = (var_fn451_calc_ig__expphib - 50.0);
                                let assign39840_e37497: f64 = (1.0 + assign39840_e37496);
                                let assign39840_e37498: f64 = (assign39840_e37492 * assign39840_e37497);
                                (assign39840_e37498, (assign39840_e37492 * var_fn451_calc_ig__expphib_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign39840_e37500, assign39840_e37500_d_n4,)
                    }
                };
                (assign39840_e37501, assign39840_e37501_d_n4,)
            }
        };
        (assign39840_e37502, assign39840_e37502_d_n4,)
    } else {
        (var_fn451_calc_ig__t0, var_fn451_calc_ig__t0_dn4,)
    }
};
        var_fn451_calc_ig__t0 = assign39840_e37504;
        var_fn451_calc_ig__t0_dn4 = assign39840_e37504_d_n4;

        let (assign39850_e37519, assign39850_e37519_d_n4, assign39850_e37519_d_n8, assign39850_e37519_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign39850_e37512: f64 = (-var_fn451_calc_ig__vgin);
        let assign39850_e37514: f64 = (assign39850_e37512 - var_fn451_calc_ig__vbdgin);
        let assign39850_e37515: f64 = (var_fn451_calc_ig__pbdgin * assign39850_e37514);
        let assign39850_e37517: f64 = (assign39850_e37515 + var_fn451_calc_ig__expphib);
        (assign39850_e37517, var_fn451_calc_ig__expphib_dn4, (var_fn451_calc_ig__pbdgin * (-var_fn451_calc_ig__vgin_dn8)), (var_fn451_calc_ig__pbdgin * (-var_fn451_calc_ig__vgin_dn9)),)
    } else {
        (var_fn451_calc_ig__expbdarg1, var_fn451_calc_ig__expbdarg1_dn4, var_fn451_calc_ig__expbdarg1_dn8, var_fn451_calc_ig__expbdarg1_dn9,)
    }
};
        var_fn451_calc_ig__expbdarg1 = assign39850_e37519;
        var_fn451_calc_ig__expbdarg1_dn4 = assign39850_e37519_d_n4;
        var_fn451_calc_ig__expbdarg1_dn8 = assign39850_e37519_d_n8;
        var_fn451_calc_ig__expbdarg1_dn9 = assign39850_e37519_d_n9;

        let (assign39860_e37532, assign39860_e37532_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign39860_e37526: f64 = (-var_fn451_calc_ig__pbdgin);
        let assign39860_e37528: f64 = (assign39860_e37526 * var_fn451_calc_ig__vbdgin);
        let assign39860_e37530: f64 = (assign39860_e37528 + var_fn451_calc_ig__expphib);
        (assign39860_e37530, var_fn451_calc_ig__expphib_dn4,)
    } else {
        (var_fn451_calc_ig__expbdarg2, var_fn451_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn451_calc_ig__expbdarg2 = assign39860_e37532;
        var_fn451_calc_ig__expbdarg2_dn4 = assign39860_e37532_d_n4;

        let (assign39870_e37578, assign39870_e37578_d_n4, assign39870_e37578_d_n8, assign39870_e37578_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign39870_e37544: f64 = (-50.0);
        let (assign39870_e37576, assign39870_e37576_d_n4, assign39870_e37576_d_n8, assign39870_e37576_d_n9,) = {
            if ((!(var_fn451_calc_ig__expbdarg1 > 50.0)) && (!(var_fn451_calc_ig__expbdarg1 < assign39870_e37544))) {
                let assign39870_e37549: f64 = (var_fn451_calc_ig__expbdarg1).exp();
                (assign39870_e37549, (assign39870_e37549 * var_fn451_calc_ig__expbdarg1_dn4), (assign39870_e37549 * var_fn451_calc_ig__expbdarg1_dn8), (assign39870_e37549 * var_fn451_calc_ig__expbdarg1_dn9),)
            } else {
                let assign39870_e37556: f64 = (-50.0);
                let (assign39870_e37575, assign39870_e37575_d_n4, assign39870_e37575_d_n8, assign39870_e37575_d_n9,) = {
                    if ((!(var_fn451_calc_ig__expbdarg1 > 50.0)) && (var_fn451_calc_ig__expbdarg1 < assign39870_e37556)) {
                        let assign39870_e37560: f64 = (-50.0);
                        let assign39870_e37561: f64 = (assign39870_e37560).exp();
                        (assign39870_e37561, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign39870_e37574, assign39870_e37574_d_n4, assign39870_e37574_d_n8, assign39870_e37574_d_n9,) = {
                            if (var_fn451_calc_ig__expbdarg1 > 50.0) {
                                let assign39870_e37566: f64 = (50.0_f64).exp();
                                let assign39870_e37570: f64 = (var_fn451_calc_ig__expbdarg1 - 50.0);
                                let assign39870_e37571: f64 = (1.0 + assign39870_e37570);
                                let assign39870_e37572: f64 = (assign39870_e37566 * assign39870_e37571);
                                (assign39870_e37572, (assign39870_e37566 * var_fn451_calc_ig__expbdarg1_dn4), (assign39870_e37566 * var_fn451_calc_ig__expbdarg1_dn8), (assign39870_e37566 * var_fn451_calc_ig__expbdarg1_dn9),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign39870_e37574, assign39870_e37574_d_n4, assign39870_e37574_d_n8, assign39870_e37574_d_n9,)
                    }
                };
                (assign39870_e37575, assign39870_e37575_d_n4, assign39870_e37575_d_n8, assign39870_e37575_d_n9,)
            }
        };
        (assign39870_e37576, assign39870_e37576_d_n4, assign39870_e37576_d_n8, assign39870_e37576_d_n9,)
    } else {
        (var_fn451_calc_ig__expbd1, var_fn451_calc_ig__expbd1_dn4, var_fn451_calc_ig__expbd1_dn8, var_fn451_calc_ig__expbd1_dn9,)
    }
};
        var_fn451_calc_ig__expbd1 = assign39870_e37578;
        var_fn451_calc_ig__expbd1_dn4 = assign39870_e37578_d_n4;
        var_fn451_calc_ig__expbd1_dn8 = assign39870_e37578_d_n8;
        var_fn451_calc_ig__expbd1_dn9 = assign39870_e37578_d_n9;

        let (assign39880_e37624, assign39880_e37624_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign39880_e37590: f64 = (-50.0);
        let (assign39880_e37622, assign39880_e37622_d_n4,) = {
            if ((!(var_fn451_calc_ig__expbdarg2 > 50.0)) && (!(var_fn451_calc_ig__expbdarg2 < assign39880_e37590))) {
                let assign39880_e37595: f64 = (var_fn451_calc_ig__expbdarg2).exp();
                (assign39880_e37595, (assign39880_e37595 * var_fn451_calc_ig__expbdarg2_dn4),)
            } else {
                let assign39880_e37602: f64 = (-50.0);
                let (assign39880_e37621, assign39880_e37621_d_n4,) = {
                    if ((!(var_fn451_calc_ig__expbdarg2 > 50.0)) && (var_fn451_calc_ig__expbdarg2 < assign39880_e37602)) {
                        let assign39880_e37606: f64 = (-50.0);
                        let assign39880_e37607: f64 = (assign39880_e37606).exp();
                        (assign39880_e37607, 0.0,)
                    } else {
                        let (assign39880_e37620, assign39880_e37620_d_n4,) = {
                            if (var_fn451_calc_ig__expbdarg2 > 50.0) {
                                let assign39880_e37612: f64 = (50.0_f64).exp();
                                let assign39880_e37616: f64 = (var_fn451_calc_ig__expbdarg2 - 50.0);
                                let assign39880_e37617: f64 = (1.0 + assign39880_e37616);
                                let assign39880_e37618: f64 = (assign39880_e37612 * assign39880_e37617);
                                (assign39880_e37618, (assign39880_e37612 * var_fn451_calc_ig__expbdarg2_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign39880_e37620, assign39880_e37620_d_n4,)
                    }
                };
                (assign39880_e37621, assign39880_e37621_d_n4,)
            }
        };
        (assign39880_e37622, assign39880_e37622_d_n4,)
    } else {
        (var_fn451_calc_ig__expbd2, var_fn451_calc_ig__expbd2_dn4,)
    }
};
        var_fn451_calc_ig__expbd2 = assign39880_e37624;
        var_fn451_calc_ig__expbd2_dn4 = assign39880_e37624_d_n4;

        let (assign39890_e37634, assign39890_e37634_d_n4, assign39890_e37634_d_n8, assign39890_e37634_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign39890_e37632: f64 = (var_fn451_calc_ig__expbd1 - var_fn451_calc_ig__expbd2);
        (assign39890_e37632, (var_fn451_calc_ig__expbd1_dn4 - var_fn451_calc_ig__expbd2_dn4), var_fn451_calc_ig__expbd1_dn8, var_fn451_calc_ig__expbd1_dn9,)
    } else {
        (var_fn451_calc_ig__iginbd, var_fn451_calc_ig__iginbd_dn4, var_fn451_calc_ig__iginbd_dn8, var_fn451_calc_ig__iginbd_dn9,)
    }
};
        var_fn451_calc_ig__iginbd = assign39890_e37634;
        var_fn451_calc_ig__iginbd_dn4 = assign39890_e37634_d_n4;
        var_fn451_calc_ig__iginbd_dn8 = assign39890_e37634_d_n8;
        var_fn451_calc_ig__iginbd_dn9 = assign39890_e37634_d_n9;

        let (assign39900_e37650, assign39900_e37650_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign39900_e37642: f64 = (var_fn451_calc_ig__type * var_fn451_calc_ig__w);
        let assign39900_e37644: f64 = (assign39900_e37642 * var_fn451_calc_ig__ngf);
        let assign39900_e37646: f64 = (assign39900_e37644 * var_fn451_calc_ig__ijin);
        let assign39900_e37648: f64 = (assign39900_e37646 * var_fn451_calc_ig__tfacdiodein);
        (assign39900_e37648, (assign39900_e37646 * var_fn451_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn451_calc_ig__isdiodeout, var_fn451_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn451_calc_ig__isdiodeout = assign39900_e37650;
        var_fn451_calc_ig__isdiodeout_dn4 = assign39900_e37650_d_n4;

        let (assign39910_e37664, assign39910_e37664_d_n4, assign39910_e37664_d_n8, assign39910_e37664_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign39910_e37658: f64 = (var_fn451_calc_ig__pg_paramin / var_fn451_calc_ig__phitin);
        let assign39910_e37660: f64 = (assign39910_e37658 * var_fn451_calc_ig__vgin);
        let assign39910_e37662: f64 = (assign39910_e37660 + var_fn451_calc_ig__expphib);
        (assign39910_e37662, (((-((var_fn451_calc_ig__pg_paramin * var_fn451_calc_ig__phitin_dn4) / (var_fn451_calc_ig__phitin * var_fn451_calc_ig__phitin))) * var_fn451_calc_ig__vgin) + var_fn451_calc_ig__expphib_dn4), (assign39910_e37658 * var_fn451_calc_ig__vgin_dn8), (assign39910_e37658 * var_fn451_calc_ig__vgin_dn9),)
    } else {
        (var_fn451_calc_ig__expiforarg, var_fn451_calc_ig__expiforarg_dn4, var_fn451_calc_ig__expiforarg_dn8, var_fn451_calc_ig__expiforarg_dn9,)
    }
};
        var_fn451_calc_ig__expiforarg = assign39910_e37664;
        var_fn451_calc_ig__expiforarg_dn4 = assign39910_e37664_d_n4;
        var_fn451_calc_ig__expiforarg_dn8 = assign39910_e37664_d_n8;
        var_fn451_calc_ig__expiforarg_dn9 = assign39910_e37664_d_n9;

        let (assign39920_e37710, assign39920_e37710_d_n4, assign39920_e37710_d_n8, assign39920_e37710_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign39920_e37676: f64 = (-50.0);
        let (assign39920_e37708, assign39920_e37708_d_n4, assign39920_e37708_d_n8, assign39920_e37708_d_n9,) = {
            if ((!(var_fn451_calc_ig__expiforarg > 50.0)) && (!(var_fn451_calc_ig__expiforarg < assign39920_e37676))) {
                let assign39920_e37681: f64 = (var_fn451_calc_ig__expiforarg).exp();
                (assign39920_e37681, (assign39920_e37681 * var_fn451_calc_ig__expiforarg_dn4), (assign39920_e37681 * var_fn451_calc_ig__expiforarg_dn8), (assign39920_e37681 * var_fn451_calc_ig__expiforarg_dn9),)
            } else {
                let assign39920_e37688: f64 = (-50.0);
                let (assign39920_e37707, assign39920_e37707_d_n4, assign39920_e37707_d_n8, assign39920_e37707_d_n9,) = {
                    if ((!(var_fn451_calc_ig__expiforarg > 50.0)) && (var_fn451_calc_ig__expiforarg < assign39920_e37688)) {
                        let assign39920_e37692: f64 = (-50.0);
                        let assign39920_e37693: f64 = (assign39920_e37692).exp();
                        (assign39920_e37693, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign39920_e37706, assign39920_e37706_d_n4, assign39920_e37706_d_n8, assign39920_e37706_d_n9,) = {
                            if (var_fn451_calc_ig__expiforarg > 50.0) {
                                let assign39920_e37698: f64 = (50.0_f64).exp();
                                let assign39920_e37702: f64 = (var_fn451_calc_ig__expiforarg - 50.0);
                                let assign39920_e37703: f64 = (1.0 + assign39920_e37702);
                                let assign39920_e37704: f64 = (assign39920_e37698 * assign39920_e37703);
                                (assign39920_e37704, (assign39920_e37698 * var_fn451_calc_ig__expiforarg_dn4), (assign39920_e37698 * var_fn451_calc_ig__expiforarg_dn8), (assign39920_e37698 * var_fn451_calc_ig__expiforarg_dn9),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign39920_e37706, assign39920_e37706_d_n4, assign39920_e37706_d_n8, assign39920_e37706_d_n9,)
                    }
                };
                (assign39920_e37707, assign39920_e37707_d_n4, assign39920_e37707_d_n8, assign39920_e37707_d_n9,)
            }
        };
        (assign39920_e37708, assign39920_e37708_d_n4, assign39920_e37708_d_n8, assign39920_e37708_d_n9,)
    } else {
        (var_fn451_calc_ig__expifor, var_fn451_calc_ig__expifor_dn4, var_fn451_calc_ig__expifor_dn8, var_fn451_calc_ig__expifor_dn9,)
    }
};
        var_fn451_calc_ig__expifor = assign39920_e37710;
        var_fn451_calc_ig__expifor_dn4 = assign39920_e37710_d_n4;
        var_fn451_calc_ig__expifor_dn8 = assign39920_e37710_d_n8;
        var_fn451_calc_ig__expifor_dn9 = assign39920_e37710_d_n9;

        let assign39930_e37713: f64 = if var_fn451_calc_ig__fracin == 1.0 { 1.0 } else { 0.0 };
        var_guard452 = assign39930_e37713;

        let (assign39940_e37731, assign39940_e37731_d_n4, assign39940_e37731_d_n8, assign39940_e37731_d_n9,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 != 0.0)) {
        let assign39940_e37725: f64 = (var_fn451_calc_ig__kbdgatein * var_fn451_calc_ig__iginbd);
        let assign39940_e37726: f64 = (var_fn451_calc_ig__expifor - assign39940_e37725);
        let assign39940_e37728: f64 = (assign39940_e37726 - var_fn451_calc_ig__t0);
        let assign39940_e37729: f64 = (var_fn451_calc_ig__isdiodeout * assign39940_e37728);
        (assign39940_e37729, ((var_fn451_calc_ig__isdiodeout_dn4 * assign39940_e37728) + (var_fn451_calc_ig__isdiodeout * ((var_fn451_calc_ig__expifor_dn4 - (var_fn451_calc_ig__kbdgatein * var_fn451_calc_ig__iginbd_dn4)) - var_fn451_calc_ig__t0_dn4))), (var_fn451_calc_ig__isdiodeout * (var_fn451_calc_ig__expifor_dn8 - (var_fn451_calc_ig__kbdgatein * var_fn451_calc_ig__iginbd_dn8))), (var_fn451_calc_ig__isdiodeout * (var_fn451_calc_ig__expifor_dn9 - (var_fn451_calc_ig__kbdgatein * var_fn451_calc_ig__iginbd_dn9))),)
    } else {
        (var_fn451_calc_ig__igindiode, var_fn451_calc_ig__igindiode_dn4, var_fn451_calc_ig__igindiode_dn8, var_fn451_calc_ig__igindiode_dn9,)
    }
};
        var_fn451_calc_ig__igindiode = assign39940_e37731;
        var_fn451_calc_ig__igindiode_dn4 = assign39940_e37731_d_n4;
        var_fn451_calc_ig__igindiode_dn8 = assign39940_e37731_d_n8;
        var_fn451_calc_ig__igindiode_dn9 = assign39940_e37731_d_n9;

        let (assign39950_e37749, assign39950_e37749_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) {
        let assign39950_e37742: f64 = (-var_fn451_calc_ig__vgsatin);
        let assign39950_e37744: f64 = (assign39950_e37742 - var_fn451_calc_ig__vbdgin);
        let assign39950_e37745: f64 = (var_fn451_calc_ig__pbdgin * assign39950_e37744);
        let assign39950_e37747: f64 = (assign39950_e37745 + var_fn451_calc_ig__expphib);
        (assign39950_e37747, var_fn451_calc_ig__expphib_dn4,)
    } else {
        (var_fn451_calc_ig__expbdarg1_vgsat, var_fn451_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__expbdarg1_vgsat = assign39950_e37749;
        var_fn451_calc_ig__expbdarg1_vgsat_dn4 = assign39950_e37749_d_n4;

        let (assign39960_e37798, assign39960_e37798_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) {
        let assign39960_e37764: f64 = (-50.0);
        let (assign39960_e37796, assign39960_e37796_d_n4,) = {
            if ((!(var_fn451_calc_ig__expbdarg1_vgsat > 50.0)) && (!(var_fn451_calc_ig__expbdarg1_vgsat < assign39960_e37764))) {
                let assign39960_e37769: f64 = (var_fn451_calc_ig__expbdarg1_vgsat).exp();
                (assign39960_e37769, (assign39960_e37769 * var_fn451_calc_ig__expbdarg1_vgsat_dn4),)
            } else {
                let assign39960_e37776: f64 = (-50.0);
                let (assign39960_e37795, assign39960_e37795_d_n4,) = {
                    if ((!(var_fn451_calc_ig__expbdarg1_vgsat > 50.0)) && (var_fn451_calc_ig__expbdarg1_vgsat < assign39960_e37776)) {
                        let assign39960_e37780: f64 = (-50.0);
                        let assign39960_e37781: f64 = (assign39960_e37780).exp();
                        (assign39960_e37781, 0.0,)
                    } else {
                        let (assign39960_e37794, assign39960_e37794_d_n4,) = {
                            if (var_fn451_calc_ig__expbdarg1_vgsat > 50.0) {
                                let assign39960_e37786: f64 = (50.0_f64).exp();
                                let assign39960_e37790: f64 = (var_fn451_calc_ig__expbdarg1_vgsat - 50.0);
                                let assign39960_e37791: f64 = (1.0 + assign39960_e37790);
                                let assign39960_e37792: f64 = (assign39960_e37786 * assign39960_e37791);
                                (assign39960_e37792, (assign39960_e37786 * var_fn451_calc_ig__expbdarg1_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign39960_e37794, assign39960_e37794_d_n4,)
                    }
                };
                (assign39960_e37795, assign39960_e37795_d_n4,)
            }
        };
        (assign39960_e37796, assign39960_e37796_d_n4,)
    } else {
        (var_fn451_calc_ig__expbd1_vgsat, var_fn451_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__expbd1_vgsat = assign39960_e37798;
        var_fn451_calc_ig__expbd1_vgsat_dn4 = assign39960_e37798_d_n4;

        let (assign39970_e37811, assign39970_e37811_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) {
        let assign39970_e37809: f64 = (var_fn451_calc_ig__expbd1_vgsat - var_fn451_calc_ig__expbd2);
        (assign39970_e37809, (var_fn451_calc_ig__expbd1_vgsat_dn4 - var_fn451_calc_ig__expbd2_dn4),)
    } else {
        (var_fn451_calc_ig__iginbd_vgsat, var_fn451_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__iginbd_vgsat = assign39970_e37811;
        var_fn451_calc_ig__iginbd_vgsat_dn4 = assign39970_e37811_d_n4;

        let (assign39980_e37828, assign39980_e37828_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) {
        let assign39980_e37822: f64 = (var_fn451_calc_ig__pg_paramin / var_fn451_calc_ig__phitin);
        let assign39980_e37824: f64 = (assign39980_e37822 * var_fn451_calc_ig__vgsatin);
        let assign39980_e37826: f64 = (assign39980_e37824 + var_fn451_calc_ig__expphib);
        (assign39980_e37826, (((-((var_fn451_calc_ig__pg_paramin * var_fn451_calc_ig__phitin_dn4) / (var_fn451_calc_ig__phitin * var_fn451_calc_ig__phitin))) * var_fn451_calc_ig__vgsatin) + var_fn451_calc_ig__expphib_dn4),)
    } else {
        (var_fn451_calc_ig__expiforarg_nohinj_vgsat, var_fn451_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__expiforarg_nohinj_vgsat = assign39980_e37828;
        var_fn451_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign39980_e37828_d_n4;

        let (assign39990_e37877, assign39990_e37877_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) {
        let assign39990_e37843: f64 = (-50.0);
        let (assign39990_e37875, assign39990_e37875_d_n4,) = {
            if ((!(var_fn451_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (!(var_fn451_calc_ig__expiforarg_nohinj_vgsat < assign39990_e37843))) {
                let assign39990_e37848: f64 = (var_fn451_calc_ig__expiforarg_nohinj_vgsat).exp();
                (assign39990_e37848, (assign39990_e37848 * var_fn451_calc_ig__expiforarg_nohinj_vgsat_dn4),)
            } else {
                let assign39990_e37855: f64 = (-50.0);
                let (assign39990_e37874, assign39990_e37874_d_n4,) = {
                    if ((!(var_fn451_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (var_fn451_calc_ig__expiforarg_nohinj_vgsat < assign39990_e37855)) {
                        let assign39990_e37859: f64 = (-50.0);
                        let assign39990_e37860: f64 = (assign39990_e37859).exp();
                        (assign39990_e37860, 0.0,)
                    } else {
                        let (assign39990_e37873, assign39990_e37873_d_n4,) = {
                            if (var_fn451_calc_ig__expiforarg_nohinj_vgsat > 50.0) {
                                let assign39990_e37865: f64 = (50.0_f64).exp();
                                let assign39990_e37869: f64 = (var_fn451_calc_ig__expiforarg_nohinj_vgsat - 50.0);
                                let assign39990_e37870: f64 = (1.0 + assign39990_e37869);
                                let assign39990_e37871: f64 = (assign39990_e37865 * assign39990_e37870);
                                (assign39990_e37871, (assign39990_e37865 * var_fn451_calc_ig__expiforarg_nohinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign39990_e37873, assign39990_e37873_d_n4,)
                    }
                };
                (assign39990_e37874, assign39990_e37874_d_n4,)
            }
        };
        (assign39990_e37875, assign39990_e37875_d_n4,)
    } else {
        (var_fn451_calc_ig__expifor_nohinj_vgsat, var_fn451_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__expifor_nohinj_vgsat = assign39990_e37877;
        var_fn451_calc_ig__expifor_nohinj_vgsat_dn4 = assign39990_e37877_d_n4;

        let (assign40000_e37894, assign40000_e37894_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) {
        let assign40000_e37889: f64 = (var_fn451_calc_ig__kbdgatein * var_fn451_calc_ig__iginbd_vgsat);
        let assign40000_e37890: f64 = (var_fn451_calc_ig__expifor_nohinj_vgsat - assign40000_e37889);
        let assign40000_e37892: f64 = (assign40000_e37890 - var_fn451_calc_ig__t0);
        (assign40000_e37892, ((var_fn451_calc_ig__expifor_nohinj_vgsat_dn4 - (var_fn451_calc_ig__kbdgatein * var_fn451_calc_ig__iginbd_vgsat_dn4)) - var_fn451_calc_ig__t0_dn4),)
    } else {
        (var_fn451_calc_ig__igindiode_nohinj_vgsat, var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__igindiode_nohinj_vgsat = assign40000_e37894;
        var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4 = assign40000_e37894_d_n4;

        let (assign40010_e37913, assign40010_e37913_d_n4, assign40010_e37913_d_n8, assign40010_e37913_d_n9,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) {
        let assign40010_e37907: f64 = (var_fn451_calc_ig__kbdgatein * var_fn451_calc_ig__iginbd);
        let assign40010_e37908: f64 = (var_fn451_calc_ig__expifor - assign40010_e37907);
        let assign40010_e37910: f64 = (assign40010_e37908 - var_fn451_calc_ig__t0);
        let assign40010_e37911: f64 = (var_fn451_calc_ig__isdiodeout * assign40010_e37910);
        (assign40010_e37911, ((var_fn451_calc_ig__isdiodeout_dn4 * assign40010_e37910) + (var_fn451_calc_ig__isdiodeout * ((var_fn451_calc_ig__expifor_dn4 - (var_fn451_calc_ig__kbdgatein * var_fn451_calc_ig__iginbd_dn4)) - var_fn451_calc_ig__t0_dn4))), (var_fn451_calc_ig__isdiodeout * (var_fn451_calc_ig__expifor_dn8 - (var_fn451_calc_ig__kbdgatein * var_fn451_calc_ig__iginbd_dn8))), (var_fn451_calc_ig__isdiodeout * (var_fn451_calc_ig__expifor_dn9 - (var_fn451_calc_ig__kbdgatein * var_fn451_calc_ig__iginbd_dn9))),)
    } else {
        (var_fn451_calc_ig__igindiode_nohinj, var_fn451_calc_ig__igindiode_nohinj_dn4, var_fn451_calc_ig__igindiode_nohinj_dn8, var_fn451_calc_ig__igindiode_nohinj_dn9,)
    }
};
        var_fn451_calc_ig__igindiode_nohinj = assign40010_e37913;
        var_fn451_calc_ig__igindiode_nohinj_dn4 = assign40010_e37913_d_n4;
        var_fn451_calc_ig__igindiode_nohinj_dn8 = assign40010_e37913_d_n8;
        var_fn451_calc_ig__igindiode_nohinj_dn9 = assign40010_e37913_d_n9;

        let assign40020_e37916: f64 = if var_fn451_calc_ig__fracin > 0.0 { 1.0 } else { 0.0 };
        var_guard453 = assign40020_e37916;

        let (assign40030_e37931,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) && (var_guard453 != 0.0)) {
        let assign40030_e37929: f64 = (var_fn451_calc_ig__fracin * var_fn451_calc_ig__pg_paramin);
        (assign40030_e37929,)
    } else {
        (var_fn451_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn451_calc_ig__pg_paramin_hinj = assign40030_e37931;

        let (assign40040_e37950, assign40040_e37950_d_n4,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) && (var_guard453 != 0.0)) {
        let assign40040_e37944: f64 = (var_fn451_calc_ig__pg_paramin_hinj / var_fn451_calc_ig__phitin);
        let assign40040_e37946: f64 = (assign40040_e37944 * var_fn451_calc_ig__vgsatin);
        let assign40040_e37948: f64 = (assign40040_e37946 + var_fn451_calc_ig__expphib);
        (assign40040_e37948, (((-((var_fn451_calc_ig__pg_paramin_hinj * var_fn451_calc_ig__phitin_dn4) / (var_fn451_calc_ig__phitin * var_fn451_calc_ig__phitin))) * var_fn451_calc_ig__vgsatin) + var_fn451_calc_ig__expphib_dn4),)
    } else {
        (var_fn451_calc_ig__expiforarg_hinj_vgsat, var_fn451_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__expiforarg_hinj_vgsat = assign40040_e37950;
        var_fn451_calc_ig__expiforarg_hinj_vgsat_dn4 = assign40040_e37950_d_n4;

        let (assign40050_e38001, assign40050_e38001_d_n4,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) && (var_guard453 != 0.0)) {
        let assign40050_e37967: f64 = (-50.0);
        let (assign40050_e37999, assign40050_e37999_d_n4,) = {
            if ((!(var_fn451_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (!(var_fn451_calc_ig__expiforarg_hinj_vgsat < assign40050_e37967))) {
                let assign40050_e37972: f64 = (var_fn451_calc_ig__expiforarg_hinj_vgsat).exp();
                (assign40050_e37972, (assign40050_e37972 * var_fn451_calc_ig__expiforarg_hinj_vgsat_dn4),)
            } else {
                let assign40050_e37979: f64 = (-50.0);
                let (assign40050_e37998, assign40050_e37998_d_n4,) = {
                    if ((!(var_fn451_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (var_fn451_calc_ig__expiforarg_hinj_vgsat < assign40050_e37979)) {
                        let assign40050_e37983: f64 = (-50.0);
                        let assign40050_e37984: f64 = (assign40050_e37983).exp();
                        (assign40050_e37984, 0.0,)
                    } else {
                        let (assign40050_e37997, assign40050_e37997_d_n4,) = {
                            if (var_fn451_calc_ig__expiforarg_hinj_vgsat > 50.0) {
                                let assign40050_e37989: f64 = (50.0_f64).exp();
                                let assign40050_e37993: f64 = (var_fn451_calc_ig__expiforarg_hinj_vgsat - 50.0);
                                let assign40050_e37994: f64 = (1.0 + assign40050_e37993);
                                let assign40050_e37995: f64 = (assign40050_e37989 * assign40050_e37994);
                                (assign40050_e37995, (assign40050_e37989 * var_fn451_calc_ig__expiforarg_hinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign40050_e37997, assign40050_e37997_d_n4,)
                    }
                };
                (assign40050_e37998, assign40050_e37998_d_n4,)
            }
        };
        (assign40050_e37999, assign40050_e37999_d_n4,)
    } else {
        (var_fn451_calc_ig__expifor_hinj_vgsat, var_fn451_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__expifor_hinj_vgsat = assign40050_e38001;
        var_fn451_calc_ig__expifor_hinj_vgsat_dn4 = assign40050_e38001_d_n4;

        let (assign40060_e38020, assign40060_e38020_d_n4,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) && (var_guard453 != 0.0)) {
        let assign40060_e38015: f64 = (var_fn451_calc_ig__kbdgatein * var_fn451_calc_ig__iginbd_vgsat);
        let assign40060_e38016: f64 = (var_fn451_calc_ig__expifor_hinj_vgsat - assign40060_e38015);
        let assign40060_e38018: f64 = (assign40060_e38016 - var_fn451_calc_ig__t0);
        (assign40060_e38018, ((var_fn451_calc_ig__expifor_hinj_vgsat_dn4 - (var_fn451_calc_ig__kbdgatein * var_fn451_calc_ig__iginbd_vgsat_dn4)) - var_fn451_calc_ig__t0_dn4),)
    } else {
        (var_fn451_calc_ig__igindiode_hinj_vgsat, var_fn451_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn451_calc_ig__igindiode_hinj_vgsat = assign40060_e38020;
        var_fn451_calc_ig__igindiode_hinj_vgsat_dn4 = assign40060_e38020_d_n4;

        let (assign40070_e38039, assign40070_e38039_d_n4, assign40070_e38039_d_n8, assign40070_e38039_d_n9,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) && (var_guard453 != 0.0)) {
        let assign40070_e38033: f64 = (var_fn451_calc_ig__pg_paramin_hinj / var_fn451_calc_ig__phitin);
        let assign40070_e38035: f64 = (assign40070_e38033 * var_fn451_calc_ig__vgin);
        let assign40070_e38037: f64 = (assign40070_e38035 + var_fn451_calc_ig__expphib);
        (assign40070_e38037, (((-((var_fn451_calc_ig__pg_paramin_hinj * var_fn451_calc_ig__phitin_dn4) / (var_fn451_calc_ig__phitin * var_fn451_calc_ig__phitin))) * var_fn451_calc_ig__vgin) + var_fn451_calc_ig__expphib_dn4), (assign40070_e38033 * var_fn451_calc_ig__vgin_dn8), (assign40070_e38033 * var_fn451_calc_ig__vgin_dn9),)
    } else {
        (var_fn451_calc_ig__expiforarg_hinj, var_fn451_calc_ig__expiforarg_hinj_dn4, var_fn451_calc_ig__expiforarg_hinj_dn8, var_fn451_calc_ig__expiforarg_hinj_dn9,)
    }
};
        var_fn451_calc_ig__expiforarg_hinj = assign40070_e38039;
        var_fn451_calc_ig__expiforarg_hinj_dn4 = assign40070_e38039_d_n4;
        var_fn451_calc_ig__expiforarg_hinj_dn8 = assign40070_e38039_d_n8;
        var_fn451_calc_ig__expiforarg_hinj_dn9 = assign40070_e38039_d_n9;

        *var_fn451_calc_ig__expbd1_slot = var_fn451_calc_ig__expbd1;
        *var_fn451_calc_ig__expbd1_dn4_slot = var_fn451_calc_ig__expbd1_dn4;
        *var_fn451_calc_ig__expbd1_dn8_slot = var_fn451_calc_ig__expbd1_dn8;
        *var_fn451_calc_ig__expbd1_dn9_slot = var_fn451_calc_ig__expbd1_dn9;
        *var_fn451_calc_ig__expbd1_vgsat_slot = var_fn451_calc_ig__expbd1_vgsat;
        *var_fn451_calc_ig__expbd1_vgsat_dn4_slot = var_fn451_calc_ig__expbd1_vgsat_dn4;
        *var_fn451_calc_ig__expbd2_slot = var_fn451_calc_ig__expbd2;
        *var_fn451_calc_ig__expbd2_dn4_slot = var_fn451_calc_ig__expbd2_dn4;
        *var_fn451_calc_ig__expbdarg1_slot = var_fn451_calc_ig__expbdarg1;
        *var_fn451_calc_ig__expbdarg1_dn4_slot = var_fn451_calc_ig__expbdarg1_dn4;
        *var_fn451_calc_ig__expbdarg1_dn8_slot = var_fn451_calc_ig__expbdarg1_dn8;
        *var_fn451_calc_ig__expbdarg1_dn9_slot = var_fn451_calc_ig__expbdarg1_dn9;
        *var_fn451_calc_ig__expbdarg1_vgsat_slot = var_fn451_calc_ig__expbdarg1_vgsat;
        *var_fn451_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn451_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn451_calc_ig__expbdarg2_slot = var_fn451_calc_ig__expbdarg2;
        *var_fn451_calc_ig__expbdarg2_dn4_slot = var_fn451_calc_ig__expbdarg2_dn4;
        *var_fn451_calc_ig__expifor_slot = var_fn451_calc_ig__expifor;
        *var_fn451_calc_ig__expifor_dn4_slot = var_fn451_calc_ig__expifor_dn4;
        *var_fn451_calc_ig__expifor_dn8_slot = var_fn451_calc_ig__expifor_dn8;
        *var_fn451_calc_ig__expifor_dn9_slot = var_fn451_calc_ig__expifor_dn9;
        *var_fn451_calc_ig__expifor_hinj_vgsat_slot = var_fn451_calc_ig__expifor_hinj_vgsat;
        *var_fn451_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn451_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn451_calc_ig__expifor_nohinj_vgsat_slot = var_fn451_calc_ig__expifor_nohinj_vgsat;
        *var_fn451_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn451_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn451_calc_ig__expiforarg_slot = var_fn451_calc_ig__expiforarg;
        *var_fn451_calc_ig__expiforarg_dn4_slot = var_fn451_calc_ig__expiforarg_dn4;
        *var_fn451_calc_ig__expiforarg_dn8_slot = var_fn451_calc_ig__expiforarg_dn8;
        *var_fn451_calc_ig__expiforarg_dn9_slot = var_fn451_calc_ig__expiforarg_dn9;
        *var_fn451_calc_ig__expiforarg_hinj_slot = var_fn451_calc_ig__expiforarg_hinj;
        *var_fn451_calc_ig__expiforarg_hinj_dn4_slot = var_fn451_calc_ig__expiforarg_hinj_dn4;
        *var_fn451_calc_ig__expiforarg_hinj_dn8_slot = var_fn451_calc_ig__expiforarg_hinj_dn8;
        *var_fn451_calc_ig__expiforarg_hinj_dn9_slot = var_fn451_calc_ig__expiforarg_hinj_dn9;
        *var_fn451_calc_ig__expiforarg_hinj_vgsat_slot = var_fn451_calc_ig__expiforarg_hinj_vgsat;
        *var_fn451_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn451_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn451_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn451_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn451_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn451_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn451_calc_ig__expphib_slot = var_fn451_calc_ig__expphib;
        *var_fn451_calc_ig__expphib_dn4_slot = var_fn451_calc_ig__expphib_dn4;
        *var_fn451_calc_ig__iginbd_slot = var_fn451_calc_ig__iginbd;
        *var_fn451_calc_ig__iginbd_dn4_slot = var_fn451_calc_ig__iginbd_dn4;
        *var_fn451_calc_ig__iginbd_dn8_slot = var_fn451_calc_ig__iginbd_dn8;
        *var_fn451_calc_ig__iginbd_dn9_slot = var_fn451_calc_ig__iginbd_dn9;
        *var_fn451_calc_ig__iginbd_vgsat_slot = var_fn451_calc_ig__iginbd_vgsat;
        *var_fn451_calc_ig__iginbd_vgsat_dn4_slot = var_fn451_calc_ig__iginbd_vgsat_dn4;
        *var_fn451_calc_ig__igindiode_slot = var_fn451_calc_ig__igindiode;
        *var_fn451_calc_ig__igindiode_dn4_slot = var_fn451_calc_ig__igindiode_dn4;
        *var_fn451_calc_ig__igindiode_dn8_slot = var_fn451_calc_ig__igindiode_dn8;
        *var_fn451_calc_ig__igindiode_dn9_slot = var_fn451_calc_ig__igindiode_dn9;
        *var_fn451_calc_ig__igindiode_hinj_vgsat_slot = var_fn451_calc_ig__igindiode_hinj_vgsat;
        *var_fn451_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn451_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn451_calc_ig__igindiode_nohinj_slot = var_fn451_calc_ig__igindiode_nohinj;
        *var_fn451_calc_ig__igindiode_nohinj_dn4_slot = var_fn451_calc_ig__igindiode_nohinj_dn4;
        *var_fn451_calc_ig__igindiode_nohinj_dn8_slot = var_fn451_calc_ig__igindiode_nohinj_dn8;
        *var_fn451_calc_ig__igindiode_nohinj_dn9_slot = var_fn451_calc_ig__igindiode_nohinj_dn9;
        *var_fn451_calc_ig__igindiode_nohinj_vgsat_slot = var_fn451_calc_ig__igindiode_nohinj_vgsat;
        *var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn451_calc_ig__isdiodeout_slot = var_fn451_calc_ig__isdiodeout;
        *var_fn451_calc_ig__isdiodeout_dn4_slot = var_fn451_calc_ig__isdiodeout_dn4;
        *var_fn451_calc_ig__pg_paramin_hinj_slot = var_fn451_calc_ig__pg_paramin_hinj;
        *var_fn451_calc_ig__t0_slot = var_fn451_calc_ig__t0;
        *var_fn451_calc_ig__t0_dn4_slot = var_fn451_calc_ig__t0_dn4;
        *var_guard452_slot = var_guard452;
        *var_guard453_slot = var_guard453;
    }

    pub(super) fn stamp_transient_block_100(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_fn451_calc_ig__alphagin: f64,
        var_fn451_calc_ig__betarecin: f64,
        var_fn451_calc_ig__expiforarg_hinj: f64,
        var_fn451_calc_ig__expiforarg_hinj_dn4: f64,
        var_fn451_calc_ig__expiforarg_hinj_dn8: f64,
        var_fn451_calc_ig__expiforarg_hinj_dn9: f64,
        var_fn451_calc_ig__iginbd: f64,
        var_fn451_calc_ig__iginbd_dn4: f64,
        var_fn451_calc_ig__iginbd_dn8: f64,
        var_fn451_calc_ig__iginbd_dn9: f64,
        var_fn451_calc_ig__igindiode_hinj_vgsat: f64,
        var_fn451_calc_ig__igindiode_hinj_vgsat_dn4: f64,
        var_fn451_calc_ig__igindiode_nohinj: f64,
        var_fn451_calc_ig__igindiode_nohinj_dn4: f64,
        var_fn451_calc_ig__igindiode_nohinj_dn8: f64,
        var_fn451_calc_ig__igindiode_nohinj_dn9: f64,
        var_fn451_calc_ig__igindiode_nohinj_vgsat: f64,
        var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4: f64,
        var_fn451_calc_ig__irecin: f64,
        var_fn451_calc_ig__isdiodeout: f64,
        var_fn451_calc_ig__isdiodeout_dn4: f64,
        var_fn451_calc_ig__kbdgatein: f64,
        var_fn451_calc_ig__ngf: f64,
        var_fn451_calc_ig__pgsrecin: f64,
        var_fn451_calc_ig__phitin: f64,
        var_fn451_calc_ig__phitin_dn4: f64,
        var_fn451_calc_ig__t0: f64,
        var_fn451_calc_ig__t0_dn4: f64,
        var_fn451_calc_ig__tfacdiodein: f64,
        var_fn451_calc_ig__tfacdiodein_dn4: f64,
        var_fn451_calc_ig__type: f64,
        var_fn451_calc_ig__vgin: f64,
        var_fn451_calc_ig__vgin_dn8: f64,
        var_fn451_calc_ig__vgin_dn9: f64,
        var_fn451_calc_ig__vgsatin: f64,
        var_fn451_calc_ig__vgsatqin: f64,
        var_fn451_calc_ig__w: f64,
        var_guard417: f64,
        var_guard439: f64,
        var_guard450: f64,
        var_guard452: f64,
        var_guard453: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_tfacdiode: f64,
        var_tfacdiode_dn4: f64,
        var_fn451_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn451_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn451_calc_ig__expffvarg_slot: &mut f64,
        var_fn451_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn451_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn451_calc_ig__expffvarg_dn9_slot: &mut f64,
        var_fn451_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn451_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn451_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn451_calc_ig__expifor_hinj_dn9_slot: &mut f64,
        var_fn451_calc_ig__expirev_slot: &mut f64,
        var_fn451_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn451_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn451_calc_ig__expirev_dn9_slot: &mut f64,
        var_fn451_calc_ig__expirevarg_slot: &mut f64,
        var_fn451_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn451_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn451_calc_ig__expirevarg_dn9_slot: &mut f64,
        var_fn451_calc_ig__ffvgin_slot: &mut f64,
        var_fn451_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn451_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn451_calc_ig__ffvgin_dn9_slot: &mut f64,
        var_fn451_calc_ig__frecgin_slot: &mut f64,
        var_fn451_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn451_calc_ig__frecgin_dn9_slot: &mut f64,
        var_fn451_calc_ig__igindiode_slot: &mut f64,
        var_fn451_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn451_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn451_calc_ig__igindiode_dn9_slot: &mut f64,
        var_fn451_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn451_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn451_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn451_calc_ig__igindiode_hinj_dn9_slot: &mut f64,
        var_fn451_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn451_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn451_calc_ig__iginrec_slot: &mut f64,
        var_fn451_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn451_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn451_calc_ig__iginrec_dn9_slot: &mut f64,
        var_fn451_calc_ig__igout_slot: &mut f64,
        var_fn451_calc_ig__igout_dn4_slot: &mut f64,
        var_fn451_calc_ig__igout_dn8_slot: &mut f64,
        var_fn451_calc_ig__igout_dn9_slot: &mut f64,
        var_fn451_calc_ig__isrecout_slot: &mut f64,
        var_fn451_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn451_calc_ig__return_slot: &mut f64,
        var_fn451_calc_ig__return_dn4_slot: &mut f64,
        var_fn451_calc_ig__return_dn8_slot: &mut f64,
        var_fn451_calc_ig__return_dn9_slot: &mut f64,
        var_fn456_calc_ig__alphagin_slot: &mut f64,
        var_fn456_calc_ig__betarecin_slot: &mut f64,
        var_fn456_calc_ig__fracin_slot: &mut f64,
        var_fn456_calc_ig__ijin_slot: &mut f64,
        var_fn456_calc_ig__isdiodeout_slot: &mut f64,
        var_fn456_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn456_calc_ig__isrecout_slot: &mut f64,
        var_fn456_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn456_calc_ig__kbdgatein_slot: &mut f64,
        var_fn456_calc_ig__ngf_slot: &mut f64,
        var_fn456_calc_ig__pbdgin_slot: &mut f64,
        var_fn456_calc_ig__pg_paramin_slot: &mut f64,
        var_fn456_calc_ig__phitin_slot: &mut f64,
        var_fn456_calc_ig__phitin_dn4_slot: &mut f64,
        var_fn456_calc_ig__return_slot: &mut f64,
        var_fn456_calc_ig__return_dn4_slot: &mut f64,
        var_fn456_calc_ig__return_dn5_slot: &mut f64,
        var_fn456_calc_ig__return_dn8_slot: &mut f64,
        var_fn456_calc_ig__tfacdiodein_slot: &mut f64,
        var_fn456_calc_ig__tfacdiodein_dn4_slot: &mut f64,
        var_fn456_calc_ig__vbdgin_slot: &mut f64,
        var_fn456_calc_ig__vgin_slot: &mut f64,
        var_fn456_calc_ig__vgin_dn5_slot: &mut f64,
        var_fn456_calc_ig__vgin_dn8_slot: &mut f64,
        var_fn456_calc_ig__vgsatin_slot: &mut f64,
        var_fn456_calc_ig__vgsatqin_slot: &mut f64,
        var_fn456_calc_ig__w_slot: &mut f64,
        var_guard454_slot: &mut f64,
        var_guard455_slot: &mut f64,
        var_igsi2db_slot: &mut f64,
        var_igsi2db_dn4_slot: &mut f64,
        var_igsi2db_dn8_slot: &mut f64,
        var_igsi2db_dn9_slot: &mut f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let mut var_fn451_calc_ig__alpha2_phit: f64 = *var_fn451_calc_ig__alpha2_phit_slot;
        let mut var_fn451_calc_ig__alpha2_phit_dn4: f64 = *var_fn451_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn451_calc_ig__expffvarg: f64 = *var_fn451_calc_ig__expffvarg_slot;
        let mut var_fn451_calc_ig__expffvarg_dn4: f64 = *var_fn451_calc_ig__expffvarg_dn4_slot;
        let mut var_fn451_calc_ig__expffvarg_dn8: f64 = *var_fn451_calc_ig__expffvarg_dn8_slot;
        let mut var_fn451_calc_ig__expffvarg_dn9: f64 = *var_fn451_calc_ig__expffvarg_dn9_slot;
        let mut var_fn451_calc_ig__expifor_hinj: f64 = *var_fn451_calc_ig__expifor_hinj_slot;
        let mut var_fn451_calc_ig__expifor_hinj_dn4: f64 = *var_fn451_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn451_calc_ig__expifor_hinj_dn8: f64 = *var_fn451_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn451_calc_ig__expifor_hinj_dn9: f64 = *var_fn451_calc_ig__expifor_hinj_dn9_slot;
        let mut var_fn451_calc_ig__expirev: f64 = *var_fn451_calc_ig__expirev_slot;
        let mut var_fn451_calc_ig__expirev_dn4: f64 = *var_fn451_calc_ig__expirev_dn4_slot;
        let mut var_fn451_calc_ig__expirev_dn8: f64 = *var_fn451_calc_ig__expirev_dn8_slot;
        let mut var_fn451_calc_ig__expirev_dn9: f64 = *var_fn451_calc_ig__expirev_dn9_slot;
        let mut var_fn451_calc_ig__expirevarg: f64 = *var_fn451_calc_ig__expirevarg_slot;
        let mut var_fn451_calc_ig__expirevarg_dn4: f64 = *var_fn451_calc_ig__expirevarg_dn4_slot;
        let mut var_fn451_calc_ig__expirevarg_dn8: f64 = *var_fn451_calc_ig__expirevarg_dn8_slot;
        let mut var_fn451_calc_ig__expirevarg_dn9: f64 = *var_fn451_calc_ig__expirevarg_dn9_slot;
        let mut var_fn451_calc_ig__ffvgin: f64 = *var_fn451_calc_ig__ffvgin_slot;
        let mut var_fn451_calc_ig__ffvgin_dn4: f64 = *var_fn451_calc_ig__ffvgin_dn4_slot;
        let mut var_fn451_calc_ig__ffvgin_dn8: f64 = *var_fn451_calc_ig__ffvgin_dn8_slot;
        let mut var_fn451_calc_ig__ffvgin_dn9: f64 = *var_fn451_calc_ig__ffvgin_dn9_slot;
        let mut var_fn451_calc_ig__frecgin: f64 = *var_fn451_calc_ig__frecgin_slot;
        let mut var_fn451_calc_ig__frecgin_dn8: f64 = *var_fn451_calc_ig__frecgin_dn8_slot;
        let mut var_fn451_calc_ig__frecgin_dn9: f64 = *var_fn451_calc_ig__frecgin_dn9_slot;
        let mut var_fn451_calc_ig__igindiode: f64 = *var_fn451_calc_ig__igindiode_slot;
        let mut var_fn451_calc_ig__igindiode_dn4: f64 = *var_fn451_calc_ig__igindiode_dn4_slot;
        let mut var_fn451_calc_ig__igindiode_dn8: f64 = *var_fn451_calc_ig__igindiode_dn8_slot;
        let mut var_fn451_calc_ig__igindiode_dn9: f64 = *var_fn451_calc_ig__igindiode_dn9_slot;
        let mut var_fn451_calc_ig__igindiode_hinj: f64 = *var_fn451_calc_ig__igindiode_hinj_slot;
        let mut var_fn451_calc_ig__igindiode_hinj_dn4: f64 = *var_fn451_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn451_calc_ig__igindiode_hinj_dn8: f64 = *var_fn451_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn451_calc_ig__igindiode_hinj_dn9: f64 = *var_fn451_calc_ig__igindiode_hinj_dn9_slot;
        let mut var_fn451_calc_ig__igindiode_hinj_pre: f64 = *var_fn451_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn451_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn451_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn451_calc_ig__iginrec: f64 = *var_fn451_calc_ig__iginrec_slot;
        let mut var_fn451_calc_ig__iginrec_dn4: f64 = *var_fn451_calc_ig__iginrec_dn4_slot;
        let mut var_fn451_calc_ig__iginrec_dn8: f64 = *var_fn451_calc_ig__iginrec_dn8_slot;
        let mut var_fn451_calc_ig__iginrec_dn9: f64 = *var_fn451_calc_ig__iginrec_dn9_slot;
        let mut var_fn451_calc_ig__igout: f64 = *var_fn451_calc_ig__igout_slot;
        let mut var_fn451_calc_ig__igout_dn4: f64 = *var_fn451_calc_ig__igout_dn4_slot;
        let mut var_fn451_calc_ig__igout_dn8: f64 = *var_fn451_calc_ig__igout_dn8_slot;
        let mut var_fn451_calc_ig__igout_dn9: f64 = *var_fn451_calc_ig__igout_dn9_slot;
        let mut var_fn451_calc_ig__isrecout: f64 = *var_fn451_calc_ig__isrecout_slot;
        let mut var_fn451_calc_ig__isrecout_dn4: f64 = *var_fn451_calc_ig__isrecout_dn4_slot;
        let mut var_fn451_calc_ig__return: f64 = *var_fn451_calc_ig__return_slot;
        let mut var_fn451_calc_ig__return_dn4: f64 = *var_fn451_calc_ig__return_dn4_slot;
        let mut var_fn451_calc_ig__return_dn8: f64 = *var_fn451_calc_ig__return_dn8_slot;
        let mut var_fn451_calc_ig__return_dn9: f64 = *var_fn451_calc_ig__return_dn9_slot;
        let mut var_fn456_calc_ig__alphagin: f64 = *var_fn456_calc_ig__alphagin_slot;
        let mut var_fn456_calc_ig__betarecin: f64 = *var_fn456_calc_ig__betarecin_slot;
        let mut var_fn456_calc_ig__fracin: f64 = *var_fn456_calc_ig__fracin_slot;
        let mut var_fn456_calc_ig__ijin: f64 = *var_fn456_calc_ig__ijin_slot;
        let mut var_fn456_calc_ig__isdiodeout: f64 = *var_fn456_calc_ig__isdiodeout_slot;
        let mut var_fn456_calc_ig__isdiodeout_dn4: f64 = *var_fn456_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn456_calc_ig__isrecout: f64 = *var_fn456_calc_ig__isrecout_slot;
        let mut var_fn456_calc_ig__isrecout_dn4: f64 = *var_fn456_calc_ig__isrecout_dn4_slot;
        let mut var_fn456_calc_ig__kbdgatein: f64 = *var_fn456_calc_ig__kbdgatein_slot;
        let mut var_fn456_calc_ig__ngf: f64 = *var_fn456_calc_ig__ngf_slot;
        let mut var_fn456_calc_ig__pbdgin: f64 = *var_fn456_calc_ig__pbdgin_slot;
        let mut var_fn456_calc_ig__pg_paramin: f64 = *var_fn456_calc_ig__pg_paramin_slot;
        let mut var_fn456_calc_ig__phitin: f64 = *var_fn456_calc_ig__phitin_slot;
        let mut var_fn456_calc_ig__phitin_dn4: f64 = *var_fn456_calc_ig__phitin_dn4_slot;
        let mut var_fn456_calc_ig__return: f64 = *var_fn456_calc_ig__return_slot;
        let mut var_fn456_calc_ig__return_dn4: f64 = *var_fn456_calc_ig__return_dn4_slot;
        let mut var_fn456_calc_ig__return_dn5: f64 = *var_fn456_calc_ig__return_dn5_slot;
        let mut var_fn456_calc_ig__return_dn8: f64 = *var_fn456_calc_ig__return_dn8_slot;
        let mut var_fn456_calc_ig__tfacdiodein: f64 = *var_fn456_calc_ig__tfacdiodein_slot;
        let mut var_fn456_calc_ig__tfacdiodein_dn4: f64 = *var_fn456_calc_ig__tfacdiodein_dn4_slot;
        let mut var_fn456_calc_ig__vbdgin: f64 = *var_fn456_calc_ig__vbdgin_slot;
        let mut var_fn456_calc_ig__vgin: f64 = *var_fn456_calc_ig__vgin_slot;
        let mut var_fn456_calc_ig__vgin_dn5: f64 = *var_fn456_calc_ig__vgin_dn5_slot;
        let mut var_fn456_calc_ig__vgin_dn8: f64 = *var_fn456_calc_ig__vgin_dn8_slot;
        let mut var_fn456_calc_ig__vgsatin: f64 = *var_fn456_calc_ig__vgsatin_slot;
        let mut var_fn456_calc_ig__vgsatqin: f64 = *var_fn456_calc_ig__vgsatqin_slot;
        let mut var_fn456_calc_ig__w: f64 = *var_fn456_calc_ig__w_slot;
        let mut var_guard454: f64 = *var_guard454_slot;
        let mut var_guard455: f64 = *var_guard455_slot;
        let mut var_igsi2db: f64 = *var_igsi2db_slot;
        let mut var_igsi2db_dn4: f64 = *var_igsi2db_dn4_slot;
        let mut var_igsi2db_dn8: f64 = *var_igsi2db_dn8_slot;
        let mut var_igsi2db_dn9: f64 = *var_igsi2db_dn9_slot;

        let (assign40080_e38090, assign40080_e38090_d_n4, assign40080_e38090_d_n8, assign40080_e38090_d_n9,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) && (var_guard453 != 0.0)) {
        let assign40080_e38056: f64 = (-50.0);
        let (assign40080_e38088, assign40080_e38088_d_n4, assign40080_e38088_d_n8, assign40080_e38088_d_n9,) = {
            if ((!(var_fn451_calc_ig__expiforarg_hinj > 50.0)) && (!(var_fn451_calc_ig__expiforarg_hinj < assign40080_e38056))) {
                let assign40080_e38061: f64 = (var_fn451_calc_ig__expiforarg_hinj).exp();
                (assign40080_e38061, (assign40080_e38061 * var_fn451_calc_ig__expiforarg_hinj_dn4), (assign40080_e38061 * var_fn451_calc_ig__expiforarg_hinj_dn8), (assign40080_e38061 * var_fn451_calc_ig__expiforarg_hinj_dn9),)
            } else {
                let assign40080_e38068: f64 = (-50.0);
                let (assign40080_e38087, assign40080_e38087_d_n4, assign40080_e38087_d_n8, assign40080_e38087_d_n9,) = {
                    if ((!(var_fn451_calc_ig__expiforarg_hinj > 50.0)) && (var_fn451_calc_ig__expiforarg_hinj < assign40080_e38068)) {
                        let assign40080_e38072: f64 = (-50.0);
                        let assign40080_e38073: f64 = (assign40080_e38072).exp();
                        (assign40080_e38073, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign40080_e38086, assign40080_e38086_d_n4, assign40080_e38086_d_n8, assign40080_e38086_d_n9,) = {
                            if (var_fn451_calc_ig__expiforarg_hinj > 50.0) {
                                let assign40080_e38078: f64 = (50.0_f64).exp();
                                let assign40080_e38082: f64 = (var_fn451_calc_ig__expiforarg_hinj - 50.0);
                                let assign40080_e38083: f64 = (1.0 + assign40080_e38082);
                                let assign40080_e38084: f64 = (assign40080_e38078 * assign40080_e38083);
                                (assign40080_e38084, (assign40080_e38078 * var_fn451_calc_ig__expiforarg_hinj_dn4), (assign40080_e38078 * var_fn451_calc_ig__expiforarg_hinj_dn8), (assign40080_e38078 * var_fn451_calc_ig__expiforarg_hinj_dn9),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign40080_e38086, assign40080_e38086_d_n4, assign40080_e38086_d_n8, assign40080_e38086_d_n9,)
                    }
                };
                (assign40080_e38087, assign40080_e38087_d_n4, assign40080_e38087_d_n8, assign40080_e38087_d_n9,)
            }
        };
        (assign40080_e38088, assign40080_e38088_d_n4, assign40080_e38088_d_n8, assign40080_e38088_d_n9,)
    } else {
        (var_fn451_calc_ig__expifor_hinj, var_fn451_calc_ig__expifor_hinj_dn4, var_fn451_calc_ig__expifor_hinj_dn8, var_fn451_calc_ig__expifor_hinj_dn9,)
    }
};
        var_fn451_calc_ig__expifor_hinj = assign40080_e38090;
        var_fn451_calc_ig__expifor_hinj_dn4 = assign40080_e38090_d_n4;
        var_fn451_calc_ig__expifor_hinj_dn8 = assign40080_e38090_d_n8;
        var_fn451_calc_ig__expifor_hinj_dn9 = assign40080_e38090_d_n9;

        let (assign40090_e38107, assign40090_e38107_d_n4,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) && (var_guard453 != 0.0)) {
        let assign40090_e38103: f64 = (var_fn451_calc_ig__isdiodeout * var_fn451_calc_ig__igindiode_nohinj_vgsat);
        let assign40090_e38105: f64 = (assign40090_e38103 / var_fn451_calc_ig__igindiode_hinj_vgsat);
        (assign40090_e38105, (((((var_fn451_calc_ig__isdiodeout_dn4 * var_fn451_calc_ig__igindiode_nohinj_vgsat) + (var_fn451_calc_ig__isdiodeout * var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4)) * var_fn451_calc_ig__igindiode_hinj_vgsat) - (assign40090_e38103 * var_fn451_calc_ig__igindiode_hinj_vgsat_dn4)) / (var_fn451_calc_ig__igindiode_hinj_vgsat * var_fn451_calc_ig__igindiode_hinj_vgsat)),)
    } else {
        (var_fn451_calc_ig__igindiode_hinj_pre, var_fn451_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn451_calc_ig__igindiode_hinj_pre = assign40090_e38107;
        var_fn451_calc_ig__igindiode_hinj_pre_dn4 = assign40090_e38107_d_n4;

        let (assign40100_e38128, assign40100_e38128_d_n4, assign40100_e38128_d_n8, assign40100_e38128_d_n9,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) && (var_guard453 != 0.0)) {
        let assign40100_e38122: f64 = (var_fn451_calc_ig__kbdgatein * var_fn451_calc_ig__iginbd);
        let assign40100_e38123: f64 = (var_fn451_calc_ig__expifor_hinj - assign40100_e38122);
        let assign40100_e38125: f64 = (assign40100_e38123 - var_fn451_calc_ig__t0);
        let assign40100_e38126: f64 = (var_fn451_calc_ig__igindiode_hinj_pre * assign40100_e38125);
        (assign40100_e38126, ((var_fn451_calc_ig__igindiode_hinj_pre_dn4 * assign40100_e38125) + (var_fn451_calc_ig__igindiode_hinj_pre * ((var_fn451_calc_ig__expifor_hinj_dn4 - (var_fn451_calc_ig__kbdgatein * var_fn451_calc_ig__iginbd_dn4)) - var_fn451_calc_ig__t0_dn4))), (var_fn451_calc_ig__igindiode_hinj_pre * (var_fn451_calc_ig__expifor_hinj_dn8 - (var_fn451_calc_ig__kbdgatein * var_fn451_calc_ig__iginbd_dn8))), (var_fn451_calc_ig__igindiode_hinj_pre * (var_fn451_calc_ig__expifor_hinj_dn9 - (var_fn451_calc_ig__kbdgatein * var_fn451_calc_ig__iginbd_dn9))),)
    } else {
        (var_fn451_calc_ig__igindiode_hinj, var_fn451_calc_ig__igindiode_hinj_dn4, var_fn451_calc_ig__igindiode_hinj_dn8, var_fn451_calc_ig__igindiode_hinj_dn9,)
    }
};
        var_fn451_calc_ig__igindiode_hinj = assign40100_e38128;
        var_fn451_calc_ig__igindiode_hinj_dn4 = assign40100_e38128_d_n4;
        var_fn451_calc_ig__igindiode_hinj_dn8 = assign40100_e38128_d_n8;
        var_fn451_calc_ig__igindiode_hinj_dn9 = assign40100_e38128_d_n9;

        let (assign40110_e38144, assign40110_e38144_d_n4, assign40110_e38144_d_n8, assign40110_e38144_d_n9,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) && (var_guard453 == 0.0)) {
        let assign40110_e38142: f64 = (var_fn451_calc_ig__isdiodeout * var_fn451_calc_ig__igindiode_nohinj_vgsat);
        (assign40110_e38142, ((var_fn451_calc_ig__isdiodeout_dn4 * var_fn451_calc_ig__igindiode_nohinj_vgsat) + (var_fn451_calc_ig__isdiodeout * var_fn451_calc_ig__igindiode_nohinj_vgsat_dn4)), 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__igindiode_hinj, var_fn451_calc_ig__igindiode_hinj_dn4, var_fn451_calc_ig__igindiode_hinj_dn8, var_fn451_calc_ig__igindiode_hinj_dn9,)
    }
};
        var_fn451_calc_ig__igindiode_hinj = assign40110_e38144;
        var_fn451_calc_ig__igindiode_hinj_dn4 = assign40110_e38144_d_n4;
        var_fn451_calc_ig__igindiode_hinj_dn8 = assign40110_e38144_d_n8;
        var_fn451_calc_ig__igindiode_hinj_dn9 = assign40110_e38144_d_n9;

        let (assign40120_e38159, assign40120_e38159_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) {
        let assign40120_e38155: f64 = (var_fn451_calc_ig__alphagin * var_fn451_calc_ig__alphagin);
        let assign40120_e38157: f64 = (assign40120_e38155 * var_fn451_calc_ig__phitin);
        (assign40120_e38157, (assign40120_e38155 * var_fn451_calc_ig__phitin_dn4),)
    } else {
        (var_fn451_calc_ig__alpha2_phit, var_fn451_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn451_calc_ig__alpha2_phit = assign40120_e38159;
        var_fn451_calc_ig__alpha2_phit_dn4 = assign40120_e38159_d_n4;

        let (assign40130_e38178, assign40130_e38178_d_n4, assign40130_e38178_d_n8, assign40130_e38178_d_n9,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) {
        let assign40130_e38172: f64 = (var_fn451_calc_ig__alpha2_phit / 2.0);
        let assign40130_e38173: f64 = (var_fn451_calc_ig__vgsatin - assign40130_e38172);
        let assign40130_e38174: f64 = (var_fn451_calc_ig__vgin - assign40130_e38173);
        let assign40130_e38176: f64 = (assign40130_e38174 / var_fn451_calc_ig__alpha2_phit);
        (assign40130_e38176, ((((-(-(var_fn451_calc_ig__alpha2_phit_dn4 / 2.0))) * var_fn451_calc_ig__alpha2_phit) - (assign40130_e38174 * var_fn451_calc_ig__alpha2_phit_dn4)) / (var_fn451_calc_ig__alpha2_phit * var_fn451_calc_ig__alpha2_phit)), (var_fn451_calc_ig__vgin_dn8 / var_fn451_calc_ig__alpha2_phit), (var_fn451_calc_ig__vgin_dn9 / var_fn451_calc_ig__alpha2_phit),)
    } else {
        (var_fn451_calc_ig__expffvarg, var_fn451_calc_ig__expffvarg_dn4, var_fn451_calc_ig__expffvarg_dn8, var_fn451_calc_ig__expffvarg_dn9,)
    }
};
        var_fn451_calc_ig__expffvarg = assign40130_e38178;
        var_fn451_calc_ig__expffvarg_dn4 = assign40130_e38178_d_n4;
        var_fn451_calc_ig__expffvarg_dn8 = assign40130_e38178_d_n8;
        var_fn451_calc_ig__expffvarg_dn9 = assign40130_e38178_d_n9;

        let assign40140_e38181: f64 = if var_fn451_calc_ig__expffvarg > 50.0 { 1.0 } else { 0.0 };
        var_guard454 = assign40140_e38181;

        let (assign40150_e38194, assign40150_e38194_d_n4, assign40150_e38194_d_n8, assign40150_e38194_d_n9,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) && (var_guard454 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__ffvgin, var_fn451_calc_ig__ffvgin_dn4, var_fn451_calc_ig__ffvgin_dn8, var_fn451_calc_ig__ffvgin_dn9,)
    }
};
        var_fn451_calc_ig__ffvgin = assign40150_e38194;
        var_fn451_calc_ig__ffvgin_dn4 = assign40150_e38194_d_n4;
        var_fn451_calc_ig__ffvgin_dn8 = assign40150_e38194_d_n8;
        var_fn451_calc_ig__ffvgin_dn9 = assign40150_e38194_d_n9;

        let assign40160_e38197: f64 = (-50.0);
        let assign40160_e38198: f64 = if var_fn451_calc_ig__expffvarg < assign40160_e38197 { 1.0 } else { 0.0 };
        var_guard455 = assign40160_e38198;

        let (assign40170_e38214, assign40170_e38214_d_n4, assign40170_e38214_d_n8, assign40170_e38214_d_n9,) = {
    if ((((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) && (var_guard454 == 0.0)) && (var_guard455 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn451_calc_ig__ffvgin, var_fn451_calc_ig__ffvgin_dn4, var_fn451_calc_ig__ffvgin_dn8, var_fn451_calc_ig__ffvgin_dn9,)
    }
};
        var_fn451_calc_ig__ffvgin = assign40170_e38214;
        var_fn451_calc_ig__ffvgin_dn4 = assign40170_e38214_d_n4;
        var_fn451_calc_ig__ffvgin_dn8 = assign40170_e38214_d_n8;
        var_fn451_calc_ig__ffvgin_dn9 = assign40170_e38214_d_n9;

        let (assign40180_e38236, assign40180_e38236_d_n4, assign40180_e38236_d_n8, assign40180_e38236_d_n9,) = {
    if ((((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) && (var_guard454 == 0.0)) && (var_guard455 == 0.0)) {
        let assign40180_e38232: f64 = (var_fn451_calc_ig__expffvarg).exp();
        let assign40180_e38233: f64 = (1.0 + assign40180_e38232);
        let assign40180_e38234: f64 = (1.0 / assign40180_e38233);
        (assign40180_e38234, (-((assign40180_e38232 * var_fn451_calc_ig__expffvarg_dn4) / (assign40180_e38233 * assign40180_e38233))), (-((assign40180_e38232 * var_fn451_calc_ig__expffvarg_dn8) / (assign40180_e38233 * assign40180_e38233))), (-((assign40180_e38232 * var_fn451_calc_ig__expffvarg_dn9) / (assign40180_e38233 * assign40180_e38233))),)
    } else {
        (var_fn451_calc_ig__ffvgin, var_fn451_calc_ig__ffvgin_dn4, var_fn451_calc_ig__ffvgin_dn8, var_fn451_calc_ig__ffvgin_dn9,)
    }
};
        var_fn451_calc_ig__ffvgin = assign40180_e38236;
        var_fn451_calc_ig__ffvgin_dn4 = assign40180_e38236_d_n4;
        var_fn451_calc_ig__ffvgin_dn8 = assign40180_e38236_d_n8;
        var_fn451_calc_ig__ffvgin_dn9 = assign40180_e38236_d_n9;

        let (assign40190_e38255, assign40190_e38255_d_n4, assign40190_e38255_d_n8, assign40190_e38255_d_n9,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard452 == 0.0)) {
        let assign40190_e38247: f64 = (var_fn451_calc_ig__ffvgin * var_fn451_calc_ig__igindiode_nohinj);
        let assign40190_e38250: f64 = (1.0 - var_fn451_calc_ig__ffvgin);
        let assign40190_e38252: f64 = (assign40190_e38250 * var_fn451_calc_ig__igindiode_hinj);
        let assign40190_e38253: f64 = (assign40190_e38247 + assign40190_e38252);
        (assign40190_e38253, (((var_fn451_calc_ig__ffvgin_dn4 * var_fn451_calc_ig__igindiode_nohinj) + (var_fn451_calc_ig__ffvgin * var_fn451_calc_ig__igindiode_nohinj_dn4)) + (((-var_fn451_calc_ig__ffvgin_dn4) * var_fn451_calc_ig__igindiode_hinj) + (assign40190_e38250 * var_fn451_calc_ig__igindiode_hinj_dn4))), (((var_fn451_calc_ig__ffvgin_dn8 * var_fn451_calc_ig__igindiode_nohinj) + (var_fn451_calc_ig__ffvgin * var_fn451_calc_ig__igindiode_nohinj_dn8)) + (((-var_fn451_calc_ig__ffvgin_dn8) * var_fn451_calc_ig__igindiode_hinj) + (assign40190_e38250 * var_fn451_calc_ig__igindiode_hinj_dn8))), (((var_fn451_calc_ig__ffvgin_dn9 * var_fn451_calc_ig__igindiode_nohinj) + (var_fn451_calc_ig__ffvgin * var_fn451_calc_ig__igindiode_nohinj_dn9)) + (((-var_fn451_calc_ig__ffvgin_dn9) * var_fn451_calc_ig__igindiode_hinj) + (assign40190_e38250 * var_fn451_calc_ig__igindiode_hinj_dn9))),)
    } else {
        (var_fn451_calc_ig__igindiode, var_fn451_calc_ig__igindiode_dn4, var_fn451_calc_ig__igindiode_dn8, var_fn451_calc_ig__igindiode_dn9,)
    }
};
        var_fn451_calc_ig__igindiode = assign40190_e38255;
        var_fn451_calc_ig__igindiode_dn4 = assign40190_e38255_d_n4;
        var_fn451_calc_ig__igindiode_dn8 = assign40190_e38255_d_n8;
        var_fn451_calc_ig__igindiode_dn9 = assign40190_e38255_d_n9;

        let (assign40200_e38305, assign40200_e38305_d_n8, assign40200_e38305_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40200_e38262: f64 = (-var_fn451_calc_ig__vgin);
        let (assign40200_e38295, assign40200_e38295_d_n8, assign40200_e38295_d_n9,) = {
            if (p.p52 != 0.0) {
                let assign40200_e38270: f64 = (var_fn451_calc_ig__vgin / var_fn451_calc_ig__vgsatqin);
                let assign40200_e38273: f64 = (0.001 / p.p53);
                let assign40200_e38276: f64 = (var_fn451_calc_ig__vgin / var_fn451_calc_ig__vgsatqin);
                let assign40200_e38277: f64 = (assign40200_e38273 * assign40200_e38276);
                let assign40200_e38278: f64 = (assign40200_e38277).tanh();
                let assign40200_e38279: f64 = (assign40200_e38270 * assign40200_e38278);
                (assign40200_e38279, (((var_fn451_calc_ig__vgin_dn8 / var_fn451_calc_ig__vgsatqin) * assign40200_e38278) + (assign40200_e38270 * ((assign40200_e38273 * (var_fn451_calc_ig__vgin_dn8 / var_fn451_calc_ig__vgsatqin)) / ((assign40200_e38277).cosh() * (assign40200_e38277).cosh())))), (((var_fn451_calc_ig__vgin_dn9 / var_fn451_calc_ig__vgsatqin) * assign40200_e38278) + (assign40200_e38270 * ((assign40200_e38273 * (var_fn451_calc_ig__vgin_dn9 / var_fn451_calc_ig__vgsatqin)) / ((assign40200_e38277).cosh() * (assign40200_e38277).cosh())))),)
            } else {
                let (assign40200_e38294, assign40200_e38294_d_n8, assign40200_e38294_d_n9,) = {
                    if (p.p52 == 0.0) {
                        let __rspice_inv_cse_0: f64 = 1.0 / var_fn451_calc_ig__vgsatqin;
                        let assign40200_e38285: f64 = (var_fn451_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign40200_e38288: f64 = (var_fn451_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign40200_e38289: f64 = (assign40200_e38285 * assign40200_e38288);
                        let assign40200_e38291: f64 = (assign40200_e38289 + p.p53);
                        let assign40200_e38292: f64 = (assign40200_e38291).sqrt();
                        (assign40200_e38292, ((((var_fn451_calc_ig__vgin_dn8 / var_fn451_calc_ig__vgsatqin) * assign40200_e38288) + (assign40200_e38285 * (var_fn451_calc_ig__vgin_dn8 / var_fn451_calc_ig__vgsatqin))) / (2.0 * assign40200_e38292)), ((((var_fn451_calc_ig__vgin_dn9 / var_fn451_calc_ig__vgsatqin) * assign40200_e38288) + (assign40200_e38285 * (var_fn451_calc_ig__vgin_dn9 / var_fn451_calc_ig__vgsatqin))) / (2.0 * assign40200_e38292)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign40200_e38294, assign40200_e38294_d_n8, assign40200_e38294_d_n9,)
            }
        };
        let assign40200_e38297: f64 = (assign40200_e38295).powf(var_fn451_calc_ig__betarecin);
        let assign40200_e38298: f64 = (1.0 + assign40200_e38297);
        let assign40200_e38301: f64 = (1.0 / var_fn451_calc_ig__betarecin);
        let assign40200_e38302: f64 = (assign40200_e38298).powf(assign40200_e38301);
        let assign40200_e38303: f64 = (assign40200_e38262 / assign40200_e38302);
        (assign40200_e38303, ((((-var_fn451_calc_ig__vgin_dn8) * assign40200_e38302) - (assign40200_e38262 * if 0.0 == 0.0 && ((assign40200_e38301) as f64).is_finite() && ((assign40200_e38301) as f64).fract() == 0.0 { if assign40200_e38301 == 0.0 { 0.0 } else { (assign40200_e38301 * ((assign40200_e38298).powf(assign40200_e38301 - 1.0) * if 0.0 == 0.0 && ((var_fn451_calc_ig__betarecin) as f64).is_finite() && ((var_fn451_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn451_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn451_calc_ig__betarecin * ((assign40200_e38295).powf(var_fn451_calc_ig__betarecin - 1.0) * assign40200_e38295_d_n8)) } } else { (assign40200_e38297 * (var_fn451_calc_ig__betarecin * (assign40200_e38295_d_n8 / assign40200_e38295))) })) } } else { (assign40200_e38302 * (assign40200_e38301 * (if 0.0 == 0.0 && ((var_fn451_calc_ig__betarecin) as f64).is_finite() && ((var_fn451_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn451_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn451_calc_ig__betarecin * ((assign40200_e38295).powf(var_fn451_calc_ig__betarecin - 1.0) * assign40200_e38295_d_n8)) } } else { (assign40200_e38297 * (var_fn451_calc_ig__betarecin * (assign40200_e38295_d_n8 / assign40200_e38295))) } / assign40200_e38298))) })) / (assign40200_e38302 * assign40200_e38302)), ((((-var_fn451_calc_ig__vgin_dn9) * assign40200_e38302) - (assign40200_e38262 * if 0.0 == 0.0 && ((assign40200_e38301) as f64).is_finite() && ((assign40200_e38301) as f64).fract() == 0.0 { if assign40200_e38301 == 0.0 { 0.0 } else { (assign40200_e38301 * ((assign40200_e38298).powf(assign40200_e38301 - 1.0) * if 0.0 == 0.0 && ((var_fn451_calc_ig__betarecin) as f64).is_finite() && ((var_fn451_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn451_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn451_calc_ig__betarecin * ((assign40200_e38295).powf(var_fn451_calc_ig__betarecin - 1.0) * assign40200_e38295_d_n9)) } } else { (assign40200_e38297 * (var_fn451_calc_ig__betarecin * (assign40200_e38295_d_n9 / assign40200_e38295))) })) } } else { (assign40200_e38302 * (assign40200_e38301 * (if 0.0 == 0.0 && ((var_fn451_calc_ig__betarecin) as f64).is_finite() && ((var_fn451_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn451_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn451_calc_ig__betarecin * ((assign40200_e38295).powf(var_fn451_calc_ig__betarecin - 1.0) * assign40200_e38295_d_n9)) } } else { (assign40200_e38297 * (var_fn451_calc_ig__betarecin * (assign40200_e38295_d_n9 / assign40200_e38295))) } / assign40200_e38298))) })) / (assign40200_e38302 * assign40200_e38302)),)
    } else {
        (var_fn451_calc_ig__frecgin, var_fn451_calc_ig__frecgin_dn8, var_fn451_calc_ig__frecgin_dn9,)
    }
};
        var_fn451_calc_ig__frecgin = assign40200_e38305;
        var_fn451_calc_ig__frecgin_dn8 = assign40200_e38305_d_n8;
        var_fn451_calc_ig__frecgin_dn9 = assign40200_e38305_d_n9;

        let (assign40210_e38324, assign40210_e38324_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40210_e38312: f64 = (-var_fn451_calc_ig__type);
        let assign40210_e38314: f64 = (assign40210_e38312 * var_fn451_calc_ig__w);
        let assign40210_e38316: f64 = (assign40210_e38314 * var_fn451_calc_ig__ngf);
        let assign40210_e38318: f64 = (assign40210_e38316 * var_fn451_calc_ig__irecin);
        let assign40210_e38320: f64 = (assign40210_e38318 * var_fn451_calc_ig__tfacdiodein);
        let assign40210_e38322: f64 = assign40210_e38320;
        (assign40210_e38322, (assign40210_e38318 * var_fn451_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn451_calc_ig__isrecout, var_fn451_calc_ig__isrecout_dn4,)
    }
};
        var_fn451_calc_ig__isrecout = assign40210_e38324;
        var_fn451_calc_ig__isrecout_dn4 = assign40210_e38324_d_n4;

        let (assign40220_e38336, assign40220_e38336_d_n4, assign40220_e38336_d_n8, assign40220_e38336_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40220_e38332: f64 = (var_fn451_calc_ig__pgsrecin / var_fn451_calc_ig__phitin);
        let assign40220_e38334: f64 = (assign40220_e38332 * var_fn451_calc_ig__frecgin);
        (assign40220_e38334, ((-((var_fn451_calc_ig__pgsrecin * var_fn451_calc_ig__phitin_dn4) / (var_fn451_calc_ig__phitin * var_fn451_calc_ig__phitin))) * var_fn451_calc_ig__frecgin), (assign40220_e38332 * var_fn451_calc_ig__frecgin_dn8), (assign40220_e38332 * var_fn451_calc_ig__frecgin_dn9),)
    } else {
        (var_fn451_calc_ig__expirevarg, var_fn451_calc_ig__expirevarg_dn4, var_fn451_calc_ig__expirevarg_dn8, var_fn451_calc_ig__expirevarg_dn9,)
    }
};
        var_fn451_calc_ig__expirevarg = assign40220_e38336;
        var_fn451_calc_ig__expirevarg_dn4 = assign40220_e38336_d_n4;
        var_fn451_calc_ig__expirevarg_dn8 = assign40220_e38336_d_n8;
        var_fn451_calc_ig__expirevarg_dn9 = assign40220_e38336_d_n9;

        let (assign40230_e38382, assign40230_e38382_d_n4, assign40230_e38382_d_n8, assign40230_e38382_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40230_e38348: f64 = (-50.0);
        let (assign40230_e38380, assign40230_e38380_d_n4, assign40230_e38380_d_n8, assign40230_e38380_d_n9,) = {
            if ((!(var_fn451_calc_ig__expirevarg > 50.0)) && (!(var_fn451_calc_ig__expirevarg < assign40230_e38348))) {
                let assign40230_e38353: f64 = (var_fn451_calc_ig__expirevarg).exp();
                (assign40230_e38353, (assign40230_e38353 * var_fn451_calc_ig__expirevarg_dn4), (assign40230_e38353 * var_fn451_calc_ig__expirevarg_dn8), (assign40230_e38353 * var_fn451_calc_ig__expirevarg_dn9),)
            } else {
                let assign40230_e38360: f64 = (-50.0);
                let (assign40230_e38379, assign40230_e38379_d_n4, assign40230_e38379_d_n8, assign40230_e38379_d_n9,) = {
                    if ((!(var_fn451_calc_ig__expirevarg > 50.0)) && (var_fn451_calc_ig__expirevarg < assign40230_e38360)) {
                        let assign40230_e38364: f64 = (-50.0);
                        let assign40230_e38365: f64 = (assign40230_e38364).exp();
                        (assign40230_e38365, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign40230_e38378, assign40230_e38378_d_n4, assign40230_e38378_d_n8, assign40230_e38378_d_n9,) = {
                            if (var_fn451_calc_ig__expirevarg > 50.0) {
                                let assign40230_e38370: f64 = (50.0_f64).exp();
                                let assign40230_e38374: f64 = (var_fn451_calc_ig__expirevarg - 50.0);
                                let assign40230_e38375: f64 = (1.0 + assign40230_e38374);
                                let assign40230_e38376: f64 = (assign40230_e38370 * assign40230_e38375);
                                (assign40230_e38376, (assign40230_e38370 * var_fn451_calc_ig__expirevarg_dn4), (assign40230_e38370 * var_fn451_calc_ig__expirevarg_dn8), (assign40230_e38370 * var_fn451_calc_ig__expirevarg_dn9),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign40230_e38378, assign40230_e38378_d_n4, assign40230_e38378_d_n8, assign40230_e38378_d_n9,)
                    }
                };
                (assign40230_e38379, assign40230_e38379_d_n4, assign40230_e38379_d_n8, assign40230_e38379_d_n9,)
            }
        };
        (assign40230_e38380, assign40230_e38380_d_n4, assign40230_e38380_d_n8, assign40230_e38380_d_n9,)
    } else {
        (var_fn451_calc_ig__expirev, var_fn451_calc_ig__expirev_dn4, var_fn451_calc_ig__expirev_dn8, var_fn451_calc_ig__expirev_dn9,)
    }
};
        var_fn451_calc_ig__expirev = assign40230_e38382;
        var_fn451_calc_ig__expirev_dn4 = assign40230_e38382_d_n4;
        var_fn451_calc_ig__expirev_dn8 = assign40230_e38382_d_n8;
        var_fn451_calc_ig__expirev_dn9 = assign40230_e38382_d_n9;

        let (assign40240_e38394, assign40240_e38394_d_n4, assign40240_e38394_d_n8, assign40240_e38394_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40240_e38391: f64 = (var_fn451_calc_ig__expirev - 1.0);
        let assign40240_e38392: f64 = (var_fn451_calc_ig__isrecout * assign40240_e38391);
        (assign40240_e38392, ((var_fn451_calc_ig__isrecout_dn4 * assign40240_e38391) + (var_fn451_calc_ig__isrecout * var_fn451_calc_ig__expirev_dn4)), (var_fn451_calc_ig__isrecout * var_fn451_calc_ig__expirev_dn8), (var_fn451_calc_ig__isrecout * var_fn451_calc_ig__expirev_dn9),)
    } else {
        (var_fn451_calc_ig__iginrec, var_fn451_calc_ig__iginrec_dn4, var_fn451_calc_ig__iginrec_dn8, var_fn451_calc_ig__iginrec_dn9,)
    }
};
        var_fn451_calc_ig__iginrec = assign40240_e38394;
        var_fn451_calc_ig__iginrec_dn4 = assign40240_e38394_d_n4;
        var_fn451_calc_ig__iginrec_dn8 = assign40240_e38394_d_n8;
        var_fn451_calc_ig__iginrec_dn9 = assign40240_e38394_d_n9;

        let (assign40250_e38404, assign40250_e38404_d_n4, assign40250_e38404_d_n8, assign40250_e38404_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40250_e38402: f64 = (var_fn451_calc_ig__igindiode + var_fn451_calc_ig__iginrec);
        (assign40250_e38402, (var_fn451_calc_ig__igindiode_dn4 + var_fn451_calc_ig__iginrec_dn4), (var_fn451_calc_ig__igindiode_dn8 + var_fn451_calc_ig__iginrec_dn8), (var_fn451_calc_ig__igindiode_dn9 + var_fn451_calc_ig__iginrec_dn9),)
    } else {
        (var_fn451_calc_ig__igout, var_fn451_calc_ig__igout_dn4, var_fn451_calc_ig__igout_dn8, var_fn451_calc_ig__igout_dn9,)
    }
};
        var_fn451_calc_ig__igout = assign40250_e38404;
        var_fn451_calc_ig__igout_dn4 = assign40250_e38404_d_n4;
        var_fn451_calc_ig__igout_dn8 = assign40250_e38404_d_n8;
        var_fn451_calc_ig__igout_dn9 = assign40250_e38404_d_n9;

        let (assign40260_e38412, assign40260_e38412_d_n4, assign40260_e38412_d_n8, assign40260_e38412_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (var_fn451_calc_ig__igout, var_fn451_calc_ig__igout_dn4, var_fn451_calc_ig__igout_dn8, var_fn451_calc_ig__igout_dn9,)
    } else {
        (var_fn451_calc_ig__return, var_fn451_calc_ig__return_dn4, var_fn451_calc_ig__return_dn8, var_fn451_calc_ig__return_dn9,)
    }
};
        var_fn451_calc_ig__return = assign40260_e38412;
        var_fn451_calc_ig__return_dn4 = assign40260_e38412_d_n4;
        var_fn451_calc_ig__return_dn8 = assign40260_e38412_d_n8;
        var_fn451_calc_ig__return_dn9 = assign40260_e38412_d_n9;

        let (assign40290_e38436, assign40290_e38436_d_n4, assign40290_e38436_d_n8, assign40290_e38436_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (var_fn451_calc_ig__return, var_fn451_calc_ig__return_dn4, var_fn451_calc_ig__return_dn8, var_fn451_calc_ig__return_dn9,)
    } else {
        (var_igsi2db, var_igsi2db_dn4, var_igsi2db_dn8, var_igsi2db_dn9,)
    }
};
        var_igsi2db = assign40290_e38436;
        var_igsi2db_dn4 = assign40290_e38436_d_n4;
        var_igsi2db_dn8 = assign40290_e38436_d_n8;
        var_igsi2db_dn9 = assign40290_e38436_d_n9;

        let (assign40300_e38444, assign40300_e38444_d_n4, assign40300_e38444_d_n5, assign40300_e38444_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__return, var_fn456_calc_ig__return_dn4, var_fn456_calc_ig__return_dn5, var_fn456_calc_ig__return_dn8,)
    }
};
        var_fn456_calc_ig__return = assign40300_e38444;
        var_fn456_calc_ig__return_dn4 = assign40300_e38444_d_n4;
        var_fn456_calc_ig__return_dn5 = assign40300_e38444_d_n5;
        var_fn456_calc_ig__return_dn8 = assign40300_e38444_d_n8;

        let (assign40310_e38452, assign40310_e38452_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__isdiodeout, var_fn456_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn456_calc_ig__isdiodeout = assign40310_e38452;
        var_fn456_calc_ig__isdiodeout_dn4 = assign40310_e38452_d_n4;

        let (assign40320_e38460, assign40320_e38460_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__isrecout, var_fn456_calc_ig__isrecout_dn4,)
    }
};
        var_fn456_calc_ig__isrecout = assign40320_e38460;
        var_fn456_calc_ig__isrecout_dn4 = assign40320_e38460_d_n4;

        let (assign40330_e38470, assign40330_e38470_d_n5, assign40330_e38470_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40330_e38468: f64 = (p.p6 * (nv8 - nv5));
        (assign40330_e38468, (-p.p6), p.p6,)
    } else {
        (var_fn456_calc_ig__vgin, var_fn456_calc_ig__vgin_dn5, var_fn456_calc_ig__vgin_dn8,)
    }
};
        var_fn456_calc_ig__vgin = assign40330_e38470;
        var_fn456_calc_ig__vgin_dn5 = assign40330_e38470_d_n5;
        var_fn456_calc_ig__vgin_dn8 = assign40330_e38470_d_n8;

        let (assign40340_e38478, assign40340_e38478_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (var_phit, var_phit_dn4,)
    } else {
        (var_fn456_calc_ig__phitin, var_fn456_calc_ig__phitin_dn4,)
    }
};
        var_fn456_calc_ig__phitin = assign40340_e38478;
        var_fn456_calc_ig__phitin_dn4 = assign40340_e38478_d_n4;

        let (assign40350_e38486,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p265,)
    } else {
        (var_fn456_calc_ig__vgsatin,)
    }
};
        var_fn456_calc_ig__vgsatin = assign40350_e38486;

        let (assign40360_e38494,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p267,)
    } else {
        (var_fn456_calc_ig__alphagin,)
    }
};
        var_fn456_calc_ig__alphagin = assign40360_e38494;

        let (assign40370_e38502,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (1.0,)
    } else {
        (var_fn456_calc_ig__fracin,)
    }
};
        var_fn456_calc_ig__fracin = assign40370_e38502;

        let (assign40380_e38510,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p263,)
    } else {
        (var_fn456_calc_ig__pg_paramin,)
    }
};
        var_fn456_calc_ig__pg_paramin = assign40380_e38510;

        let (assign40390_e38518,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p281,)
    } else {
        (var_fn456_calc_ig__pbdgin,)
    }
};
        var_fn456_calc_ig__pbdgin = assign40390_e38518;

        let (assign40400_e38526,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p280,)
    } else {
        (var_fn456_calc_ig__vbdgin,)
    }
};
        var_fn456_calc_ig__vbdgin = assign40400_e38526;

        let (assign40410_e38534, assign40410_e38534_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (var_tfacdiode, var_tfacdiode_dn4,)
    } else {
        (var_fn456_calc_ig__tfacdiodein, var_fn456_calc_ig__tfacdiodein_dn4,)
    }
};
        var_fn456_calc_ig__tfacdiodein = assign40410_e38534;
        var_fn456_calc_ig__tfacdiodein_dn4 = assign40410_e38534_d_n4;

        let (assign40420_e38542,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p0,)
    } else {
        (var_fn456_calc_ig__w,)
    }
};
        var_fn456_calc_ig__w = assign40420_e38542;

        let (assign40430_e38550,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p2,)
    } else {
        (var_fn456_calc_ig__ngf,)
    }
};
        var_fn456_calc_ig__ngf = assign40430_e38550;

        let (assign40440_e38558,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0,)
    } else {
        (var_fn456_calc_ig__ijin,)
    }
};
        var_fn456_calc_ig__ijin = assign40440_e38558;

        let (assign40450_e38566,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0,)
    } else {
        (var_fn456_calc_ig__kbdgatein,)
    }
};
        var_fn456_calc_ig__kbdgatein = assign40450_e38566;

        let (assign40460_e38574,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p289,)
    } else {
        (var_fn456_calc_ig__vgsatqin,)
    }
};
        var_fn456_calc_ig__vgsatqin = assign40460_e38574;

        let (assign40470_e38582,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p290,)
    } else {
        (var_fn456_calc_ig__betarecin,)
    }
};
        var_fn456_calc_ig__betarecin = assign40470_e38582;

        *var_fn451_calc_ig__alpha2_phit_slot = var_fn451_calc_ig__alpha2_phit;
        *var_fn451_calc_ig__alpha2_phit_dn4_slot = var_fn451_calc_ig__alpha2_phit_dn4;
        *var_fn451_calc_ig__expffvarg_slot = var_fn451_calc_ig__expffvarg;
        *var_fn451_calc_ig__expffvarg_dn4_slot = var_fn451_calc_ig__expffvarg_dn4;
        *var_fn451_calc_ig__expffvarg_dn8_slot = var_fn451_calc_ig__expffvarg_dn8;
        *var_fn451_calc_ig__expffvarg_dn9_slot = var_fn451_calc_ig__expffvarg_dn9;
        *var_fn451_calc_ig__expifor_hinj_slot = var_fn451_calc_ig__expifor_hinj;
        *var_fn451_calc_ig__expifor_hinj_dn4_slot = var_fn451_calc_ig__expifor_hinj_dn4;
        *var_fn451_calc_ig__expifor_hinj_dn8_slot = var_fn451_calc_ig__expifor_hinj_dn8;
        *var_fn451_calc_ig__expifor_hinj_dn9_slot = var_fn451_calc_ig__expifor_hinj_dn9;
        *var_fn451_calc_ig__expirev_slot = var_fn451_calc_ig__expirev;
        *var_fn451_calc_ig__expirev_dn4_slot = var_fn451_calc_ig__expirev_dn4;
        *var_fn451_calc_ig__expirev_dn8_slot = var_fn451_calc_ig__expirev_dn8;
        *var_fn451_calc_ig__expirev_dn9_slot = var_fn451_calc_ig__expirev_dn9;
        *var_fn451_calc_ig__expirevarg_slot = var_fn451_calc_ig__expirevarg;
        *var_fn451_calc_ig__expirevarg_dn4_slot = var_fn451_calc_ig__expirevarg_dn4;
        *var_fn451_calc_ig__expirevarg_dn8_slot = var_fn451_calc_ig__expirevarg_dn8;
        *var_fn451_calc_ig__expirevarg_dn9_slot = var_fn451_calc_ig__expirevarg_dn9;
        *var_fn451_calc_ig__ffvgin_slot = var_fn451_calc_ig__ffvgin;
        *var_fn451_calc_ig__ffvgin_dn4_slot = var_fn451_calc_ig__ffvgin_dn4;
        *var_fn451_calc_ig__ffvgin_dn8_slot = var_fn451_calc_ig__ffvgin_dn8;
        *var_fn451_calc_ig__ffvgin_dn9_slot = var_fn451_calc_ig__ffvgin_dn9;
        *var_fn451_calc_ig__frecgin_slot = var_fn451_calc_ig__frecgin;
        *var_fn451_calc_ig__frecgin_dn8_slot = var_fn451_calc_ig__frecgin_dn8;
        *var_fn451_calc_ig__frecgin_dn9_slot = var_fn451_calc_ig__frecgin_dn9;
        *var_fn451_calc_ig__igindiode_slot = var_fn451_calc_ig__igindiode;
        *var_fn451_calc_ig__igindiode_dn4_slot = var_fn451_calc_ig__igindiode_dn4;
        *var_fn451_calc_ig__igindiode_dn8_slot = var_fn451_calc_ig__igindiode_dn8;
        *var_fn451_calc_ig__igindiode_dn9_slot = var_fn451_calc_ig__igindiode_dn9;
        *var_fn451_calc_ig__igindiode_hinj_slot = var_fn451_calc_ig__igindiode_hinj;
        *var_fn451_calc_ig__igindiode_hinj_dn4_slot = var_fn451_calc_ig__igindiode_hinj_dn4;
        *var_fn451_calc_ig__igindiode_hinj_dn8_slot = var_fn451_calc_ig__igindiode_hinj_dn8;
        *var_fn451_calc_ig__igindiode_hinj_dn9_slot = var_fn451_calc_ig__igindiode_hinj_dn9;
        *var_fn451_calc_ig__igindiode_hinj_pre_slot = var_fn451_calc_ig__igindiode_hinj_pre;
        *var_fn451_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn451_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn451_calc_ig__iginrec_slot = var_fn451_calc_ig__iginrec;
        *var_fn451_calc_ig__iginrec_dn4_slot = var_fn451_calc_ig__iginrec_dn4;
        *var_fn451_calc_ig__iginrec_dn8_slot = var_fn451_calc_ig__iginrec_dn8;
        *var_fn451_calc_ig__iginrec_dn9_slot = var_fn451_calc_ig__iginrec_dn9;
        *var_fn451_calc_ig__igout_slot = var_fn451_calc_ig__igout;
        *var_fn451_calc_ig__igout_dn4_slot = var_fn451_calc_ig__igout_dn4;
        *var_fn451_calc_ig__igout_dn8_slot = var_fn451_calc_ig__igout_dn8;
        *var_fn451_calc_ig__igout_dn9_slot = var_fn451_calc_ig__igout_dn9;
        *var_fn451_calc_ig__isrecout_slot = var_fn451_calc_ig__isrecout;
        *var_fn451_calc_ig__isrecout_dn4_slot = var_fn451_calc_ig__isrecout_dn4;
        *var_fn451_calc_ig__return_slot = var_fn451_calc_ig__return;
        *var_fn451_calc_ig__return_dn4_slot = var_fn451_calc_ig__return_dn4;
        *var_fn451_calc_ig__return_dn8_slot = var_fn451_calc_ig__return_dn8;
        *var_fn451_calc_ig__return_dn9_slot = var_fn451_calc_ig__return_dn9;
        *var_fn456_calc_ig__alphagin_slot = var_fn456_calc_ig__alphagin;
        *var_fn456_calc_ig__betarecin_slot = var_fn456_calc_ig__betarecin;
        *var_fn456_calc_ig__fracin_slot = var_fn456_calc_ig__fracin;
        *var_fn456_calc_ig__ijin_slot = var_fn456_calc_ig__ijin;
        *var_fn456_calc_ig__isdiodeout_slot = var_fn456_calc_ig__isdiodeout;
        *var_fn456_calc_ig__isdiodeout_dn4_slot = var_fn456_calc_ig__isdiodeout_dn4;
        *var_fn456_calc_ig__isrecout_slot = var_fn456_calc_ig__isrecout;
        *var_fn456_calc_ig__isrecout_dn4_slot = var_fn456_calc_ig__isrecout_dn4;
        *var_fn456_calc_ig__kbdgatein_slot = var_fn456_calc_ig__kbdgatein;
        *var_fn456_calc_ig__ngf_slot = var_fn456_calc_ig__ngf;
        *var_fn456_calc_ig__pbdgin_slot = var_fn456_calc_ig__pbdgin;
        *var_fn456_calc_ig__pg_paramin_slot = var_fn456_calc_ig__pg_paramin;
        *var_fn456_calc_ig__phitin_slot = var_fn456_calc_ig__phitin;
        *var_fn456_calc_ig__phitin_dn4_slot = var_fn456_calc_ig__phitin_dn4;
        *var_fn456_calc_ig__return_slot = var_fn456_calc_ig__return;
        *var_fn456_calc_ig__return_dn4_slot = var_fn456_calc_ig__return_dn4;
        *var_fn456_calc_ig__return_dn5_slot = var_fn456_calc_ig__return_dn5;
        *var_fn456_calc_ig__return_dn8_slot = var_fn456_calc_ig__return_dn8;
        *var_fn456_calc_ig__tfacdiodein_slot = var_fn456_calc_ig__tfacdiodein;
        *var_fn456_calc_ig__tfacdiodein_dn4_slot = var_fn456_calc_ig__tfacdiodein_dn4;
        *var_fn456_calc_ig__vbdgin_slot = var_fn456_calc_ig__vbdgin;
        *var_fn456_calc_ig__vgin_slot = var_fn456_calc_ig__vgin;
        *var_fn456_calc_ig__vgin_dn5_slot = var_fn456_calc_ig__vgin_dn5;
        *var_fn456_calc_ig__vgin_dn8_slot = var_fn456_calc_ig__vgin_dn8;
        *var_fn456_calc_ig__vgsatin_slot = var_fn456_calc_ig__vgsatin;
        *var_fn456_calc_ig__vgsatqin_slot = var_fn456_calc_ig__vgsatqin;
        *var_fn456_calc_ig__w_slot = var_fn456_calc_ig__w;
        *var_guard454_slot = var_guard454;
        *var_guard455_slot = var_guard455;
        *var_igsi2db_slot = var_igsi2db;
        *var_igsi2db_dn4_slot = var_igsi2db_dn4;
        *var_igsi2db_dn8_slot = var_igsi2db_dn8;
        *var_igsi2db_dn9_slot = var_igsi2db_dn9;
    }

    pub(super) fn stamp_transient_block_101(
        p: &Parameters,
        var_fn456_calc_ig__pbdgin: f64,
        var_fn456_calc_ig__phitin: f64,
        var_fn456_calc_ig__phitin_dn4: f64,
        var_fn456_calc_ig__vbdgin: f64,
        var_fn456_calc_ig__vgin: f64,
        var_fn456_calc_ig__vgin_dn5: f64,
        var_fn456_calc_ig__vgin_dn8: f64,
        var_guard417: f64,
        var_guard439: f64,
        var_guard450: f64,
        var_fn456_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn456_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn456_calc_ig__expbd1_slot: &mut f64,
        var_fn456_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn456_calc_ig__expbd1_dn5_slot: &mut f64,
        var_fn456_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn456_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn456_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__expbd2_slot: &mut f64,
        var_fn456_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn456_calc_ig__expbdarg1_slot: &mut f64,
        var_fn456_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn456_calc_ig__expbdarg1_dn5_slot: &mut f64,
        var_fn456_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn456_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn456_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__expbdarg2_slot: &mut f64,
        var_fn456_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn456_calc_ig__expffvarg_slot: &mut f64,
        var_fn456_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn456_calc_ig__expffvarg_dn5_slot: &mut f64,
        var_fn456_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn456_calc_ig__expifor_slot: &mut f64,
        var_fn456_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn456_calc_ig__expifor_dn5_slot: &mut f64,
        var_fn456_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn456_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn456_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn456_calc_ig__expifor_hinj_dn5_slot: &mut f64,
        var_fn456_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn456_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn456_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn456_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_dn5_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_hinj_dn5_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__expirev_slot: &mut f64,
        var_fn456_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn456_calc_ig__expirev_dn5_slot: &mut f64,
        var_fn456_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn456_calc_ig__expirevarg_slot: &mut f64,
        var_fn456_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn456_calc_ig__expirevarg_dn5_slot: &mut f64,
        var_fn456_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn456_calc_ig__expphib_slot: &mut f64,
        var_fn456_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn456_calc_ig__ffvgin_slot: &mut f64,
        var_fn456_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn456_calc_ig__ffvgin_dn5_slot: &mut f64,
        var_fn456_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn456_calc_ig__frecgin_slot: &mut f64,
        var_fn456_calc_ig__frecgin_dn5_slot: &mut f64,
        var_fn456_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn456_calc_ig__iginbd_slot: &mut f64,
        var_fn456_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn456_calc_ig__iginbd_dn5_slot: &mut f64,
        var_fn456_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn456_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn456_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__igindiode_slot: &mut f64,
        var_fn456_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn456_calc_ig__igindiode_dn5_slot: &mut f64,
        var_fn456_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn456_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn456_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn456_calc_ig__igindiode_hinj_dn5_slot: &mut f64,
        var_fn456_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn456_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn456_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn456_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn456_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn456_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn456_calc_ig__igindiode_nohinj_dn5_slot: &mut f64,
        var_fn456_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn456_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn456_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__iginrec_slot: &mut f64,
        var_fn456_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn456_calc_ig__iginrec_dn5_slot: &mut f64,
        var_fn456_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn456_calc_ig__igout_slot: &mut f64,
        var_fn456_calc_ig__igout_dn4_slot: &mut f64,
        var_fn456_calc_ig__igout_dn5_slot: &mut f64,
        var_fn456_calc_ig__igout_dn8_slot: &mut f64,
        var_fn456_calc_ig__irecin_slot: &mut f64,
        var_fn456_calc_ig__pg_param1_slot: &mut f64,
        var_fn456_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_fn456_calc_ig__pgsrecin_slot: &mut f64,
        var_fn456_calc_ig__t0_slot: &mut f64,
        var_fn456_calc_ig__t0_dn4_slot: &mut f64,
        var_fn456_calc_ig__type_slot: &mut f64,
        var_fn456_calc_ig__vjg_slot: &mut f64,
    ) {
        let mut var_fn456_calc_ig__alpha2_phit: f64 = *var_fn456_calc_ig__alpha2_phit_slot;
        let mut var_fn456_calc_ig__alpha2_phit_dn4: f64 = *var_fn456_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn456_calc_ig__expbd1: f64 = *var_fn456_calc_ig__expbd1_slot;
        let mut var_fn456_calc_ig__expbd1_dn4: f64 = *var_fn456_calc_ig__expbd1_dn4_slot;
        let mut var_fn456_calc_ig__expbd1_dn5: f64 = *var_fn456_calc_ig__expbd1_dn5_slot;
        let mut var_fn456_calc_ig__expbd1_dn8: f64 = *var_fn456_calc_ig__expbd1_dn8_slot;
        let mut var_fn456_calc_ig__expbd1_vgsat: f64 = *var_fn456_calc_ig__expbd1_vgsat_slot;
        let mut var_fn456_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn456_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__expbd2: f64 = *var_fn456_calc_ig__expbd2_slot;
        let mut var_fn456_calc_ig__expbd2_dn4: f64 = *var_fn456_calc_ig__expbd2_dn4_slot;
        let mut var_fn456_calc_ig__expbdarg1: f64 = *var_fn456_calc_ig__expbdarg1_slot;
        let mut var_fn456_calc_ig__expbdarg1_dn4: f64 = *var_fn456_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn456_calc_ig__expbdarg1_dn5: f64 = *var_fn456_calc_ig__expbdarg1_dn5_slot;
        let mut var_fn456_calc_ig__expbdarg1_dn8: f64 = *var_fn456_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn456_calc_ig__expbdarg1_vgsat: f64 = *var_fn456_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn456_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn456_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__expbdarg2: f64 = *var_fn456_calc_ig__expbdarg2_slot;
        let mut var_fn456_calc_ig__expbdarg2_dn4: f64 = *var_fn456_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn456_calc_ig__expffvarg: f64 = *var_fn456_calc_ig__expffvarg_slot;
        let mut var_fn456_calc_ig__expffvarg_dn4: f64 = *var_fn456_calc_ig__expffvarg_dn4_slot;
        let mut var_fn456_calc_ig__expffvarg_dn5: f64 = *var_fn456_calc_ig__expffvarg_dn5_slot;
        let mut var_fn456_calc_ig__expffvarg_dn8: f64 = *var_fn456_calc_ig__expffvarg_dn8_slot;
        let mut var_fn456_calc_ig__expifor: f64 = *var_fn456_calc_ig__expifor_slot;
        let mut var_fn456_calc_ig__expifor_dn4: f64 = *var_fn456_calc_ig__expifor_dn4_slot;
        let mut var_fn456_calc_ig__expifor_dn5: f64 = *var_fn456_calc_ig__expifor_dn5_slot;
        let mut var_fn456_calc_ig__expifor_dn8: f64 = *var_fn456_calc_ig__expifor_dn8_slot;
        let mut var_fn456_calc_ig__expifor_hinj: f64 = *var_fn456_calc_ig__expifor_hinj_slot;
        let mut var_fn456_calc_ig__expifor_hinj_dn4: f64 = *var_fn456_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn456_calc_ig__expifor_hinj_dn5: f64 = *var_fn456_calc_ig__expifor_hinj_dn5_slot;
        let mut var_fn456_calc_ig__expifor_hinj_dn8: f64 = *var_fn456_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn456_calc_ig__expifor_hinj_vgsat: f64 = *var_fn456_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn456_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn456_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn456_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn456_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn456_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__expiforarg: f64 = *var_fn456_calc_ig__expiforarg_slot;
        let mut var_fn456_calc_ig__expiforarg_dn4: f64 = *var_fn456_calc_ig__expiforarg_dn4_slot;
        let mut var_fn456_calc_ig__expiforarg_dn5: f64 = *var_fn456_calc_ig__expiforarg_dn5_slot;
        let mut var_fn456_calc_ig__expiforarg_dn8: f64 = *var_fn456_calc_ig__expiforarg_dn8_slot;
        let mut var_fn456_calc_ig__expiforarg_hinj: f64 = *var_fn456_calc_ig__expiforarg_hinj_slot;
        let mut var_fn456_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn456_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn456_calc_ig__expiforarg_hinj_dn5: f64 = *var_fn456_calc_ig__expiforarg_hinj_dn5_slot;
        let mut var_fn456_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn456_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn456_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn456_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn456_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn456_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn456_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn456_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn456_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__expirev: f64 = *var_fn456_calc_ig__expirev_slot;
        let mut var_fn456_calc_ig__expirev_dn4: f64 = *var_fn456_calc_ig__expirev_dn4_slot;
        let mut var_fn456_calc_ig__expirev_dn5: f64 = *var_fn456_calc_ig__expirev_dn5_slot;
        let mut var_fn456_calc_ig__expirev_dn8: f64 = *var_fn456_calc_ig__expirev_dn8_slot;
        let mut var_fn456_calc_ig__expirevarg: f64 = *var_fn456_calc_ig__expirevarg_slot;
        let mut var_fn456_calc_ig__expirevarg_dn4: f64 = *var_fn456_calc_ig__expirevarg_dn4_slot;
        let mut var_fn456_calc_ig__expirevarg_dn5: f64 = *var_fn456_calc_ig__expirevarg_dn5_slot;
        let mut var_fn456_calc_ig__expirevarg_dn8: f64 = *var_fn456_calc_ig__expirevarg_dn8_slot;
        let mut var_fn456_calc_ig__expphib: f64 = *var_fn456_calc_ig__expphib_slot;
        let mut var_fn456_calc_ig__expphib_dn4: f64 = *var_fn456_calc_ig__expphib_dn4_slot;
        let mut var_fn456_calc_ig__ffvgin: f64 = *var_fn456_calc_ig__ffvgin_slot;
        let mut var_fn456_calc_ig__ffvgin_dn4: f64 = *var_fn456_calc_ig__ffvgin_dn4_slot;
        let mut var_fn456_calc_ig__ffvgin_dn5: f64 = *var_fn456_calc_ig__ffvgin_dn5_slot;
        let mut var_fn456_calc_ig__ffvgin_dn8: f64 = *var_fn456_calc_ig__ffvgin_dn8_slot;
        let mut var_fn456_calc_ig__frecgin: f64 = *var_fn456_calc_ig__frecgin_slot;
        let mut var_fn456_calc_ig__frecgin_dn5: f64 = *var_fn456_calc_ig__frecgin_dn5_slot;
        let mut var_fn456_calc_ig__frecgin_dn8: f64 = *var_fn456_calc_ig__frecgin_dn8_slot;
        let mut var_fn456_calc_ig__iginbd: f64 = *var_fn456_calc_ig__iginbd_slot;
        let mut var_fn456_calc_ig__iginbd_dn4: f64 = *var_fn456_calc_ig__iginbd_dn4_slot;
        let mut var_fn456_calc_ig__iginbd_dn5: f64 = *var_fn456_calc_ig__iginbd_dn5_slot;
        let mut var_fn456_calc_ig__iginbd_dn8: f64 = *var_fn456_calc_ig__iginbd_dn8_slot;
        let mut var_fn456_calc_ig__iginbd_vgsat: f64 = *var_fn456_calc_ig__iginbd_vgsat_slot;
        let mut var_fn456_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn456_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__igindiode: f64 = *var_fn456_calc_ig__igindiode_slot;
        let mut var_fn456_calc_ig__igindiode_dn4: f64 = *var_fn456_calc_ig__igindiode_dn4_slot;
        let mut var_fn456_calc_ig__igindiode_dn5: f64 = *var_fn456_calc_ig__igindiode_dn5_slot;
        let mut var_fn456_calc_ig__igindiode_dn8: f64 = *var_fn456_calc_ig__igindiode_dn8_slot;
        let mut var_fn456_calc_ig__igindiode_hinj: f64 = *var_fn456_calc_ig__igindiode_hinj_slot;
        let mut var_fn456_calc_ig__igindiode_hinj_dn4: f64 = *var_fn456_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn456_calc_ig__igindiode_hinj_dn5: f64 = *var_fn456_calc_ig__igindiode_hinj_dn5_slot;
        let mut var_fn456_calc_ig__igindiode_hinj_dn8: f64 = *var_fn456_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn456_calc_ig__igindiode_hinj_pre: f64 = *var_fn456_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn456_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn456_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn456_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn456_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn456_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn456_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__igindiode_nohinj: f64 = *var_fn456_calc_ig__igindiode_nohinj_slot;
        let mut var_fn456_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn456_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn456_calc_ig__igindiode_nohinj_dn5: f64 = *var_fn456_calc_ig__igindiode_nohinj_dn5_slot;
        let mut var_fn456_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn456_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn456_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn456_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn456_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn456_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__iginrec: f64 = *var_fn456_calc_ig__iginrec_slot;
        let mut var_fn456_calc_ig__iginrec_dn4: f64 = *var_fn456_calc_ig__iginrec_dn4_slot;
        let mut var_fn456_calc_ig__iginrec_dn5: f64 = *var_fn456_calc_ig__iginrec_dn5_slot;
        let mut var_fn456_calc_ig__iginrec_dn8: f64 = *var_fn456_calc_ig__iginrec_dn8_slot;
        let mut var_fn456_calc_ig__igout: f64 = *var_fn456_calc_ig__igout_slot;
        let mut var_fn456_calc_ig__igout_dn4: f64 = *var_fn456_calc_ig__igout_dn4_slot;
        let mut var_fn456_calc_ig__igout_dn5: f64 = *var_fn456_calc_ig__igout_dn5_slot;
        let mut var_fn456_calc_ig__igout_dn8: f64 = *var_fn456_calc_ig__igout_dn8_slot;
        let mut var_fn456_calc_ig__irecin: f64 = *var_fn456_calc_ig__irecin_slot;
        let mut var_fn456_calc_ig__pg_param1: f64 = *var_fn456_calc_ig__pg_param1_slot;
        let mut var_fn456_calc_ig__pg_paramin_hinj: f64 = *var_fn456_calc_ig__pg_paramin_hinj_slot;
        let mut var_fn456_calc_ig__pgsrecin: f64 = *var_fn456_calc_ig__pgsrecin_slot;
        let mut var_fn456_calc_ig__t0: f64 = *var_fn456_calc_ig__t0_slot;
        let mut var_fn456_calc_ig__t0_dn4: f64 = *var_fn456_calc_ig__t0_dn4_slot;
        let mut var_fn456_calc_ig__type: f64 = *var_fn456_calc_ig__type_slot;
        let mut var_fn456_calc_ig__vjg: f64 = *var_fn456_calc_ig__vjg_slot;

        let (assign40480_e38592,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40480_e38590: f64 = (p.p255 * p.p288);
        (assign40480_e38590,)
    } else {
        (var_fn456_calc_ig__irecin,)
    }
};
        var_fn456_calc_ig__irecin = assign40480_e38592;

        let (assign40490_e38600,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p287,)
    } else {
        (var_fn456_calc_ig__pgsrecin,)
    }
};
        var_fn456_calc_ig__pgsrecin = assign40490_e38600;

        let (assign40500_e38608,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p257,)
    } else {
        (var_fn456_calc_ig__pg_param1,)
    }
};
        var_fn456_calc_ig__pg_param1 = assign40500_e38608;

        let (assign40510_e38616,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p256,)
    } else {
        (var_fn456_calc_ig__vjg,)
    }
};
        var_fn456_calc_ig__vjg = assign40510_e38616;

        let (assign40520_e38624,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (p.p6,)
    } else {
        (var_fn456_calc_ig__type,)
    }
};
        var_fn456_calc_ig__type = assign40520_e38624;

        let (assign40530_e38632, assign40530_e38632_d_n4, assign40530_e38632_d_n5, assign40530_e38632_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__igout, var_fn456_calc_ig__igout_dn4, var_fn456_calc_ig__igout_dn5, var_fn456_calc_ig__igout_dn8,)
    }
};
        var_fn456_calc_ig__igout = assign40530_e38632;
        var_fn456_calc_ig__igout_dn4 = assign40530_e38632_d_n4;
        var_fn456_calc_ig__igout_dn5 = assign40530_e38632_d_n5;
        var_fn456_calc_ig__igout_dn8 = assign40530_e38632_d_n8;

        let (assign40540_e38640, assign40540_e38640_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__alpha2_phit, var_fn456_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn456_calc_ig__alpha2_phit = assign40540_e38640;
        var_fn456_calc_ig__alpha2_phit_dn4 = assign40540_e38640_d_n4;

        let (assign40550_e38648, assign40550_e38648_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__t0, var_fn456_calc_ig__t0_dn4,)
    }
};
        var_fn456_calc_ig__t0 = assign40550_e38648;
        var_fn456_calc_ig__t0_dn4 = assign40550_e38648_d_n4;

        let (assign40560_e38656, assign40560_e38656_d_n4, assign40560_e38656_d_n5, assign40560_e38656_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__ffvgin, var_fn456_calc_ig__ffvgin_dn4, var_fn456_calc_ig__ffvgin_dn5, var_fn456_calc_ig__ffvgin_dn8,)
    }
};
        var_fn456_calc_ig__ffvgin = assign40560_e38656;
        var_fn456_calc_ig__ffvgin_dn4 = assign40560_e38656_d_n4;
        var_fn456_calc_ig__ffvgin_dn5 = assign40560_e38656_d_n5;
        var_fn456_calc_ig__ffvgin_dn8 = assign40560_e38656_d_n8;

        let (assign40570_e38664, assign40570_e38664_d_n4, assign40570_e38664_d_n5, assign40570_e38664_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__iginbd, var_fn456_calc_ig__iginbd_dn4, var_fn456_calc_ig__iginbd_dn5, var_fn456_calc_ig__iginbd_dn8,)
    }
};
        var_fn456_calc_ig__iginbd = assign40570_e38664;
        var_fn456_calc_ig__iginbd_dn4 = assign40570_e38664_d_n4;
        var_fn456_calc_ig__iginbd_dn5 = assign40570_e38664_d_n5;
        var_fn456_calc_ig__iginbd_dn8 = assign40570_e38664_d_n8;

        let (assign40580_e38672, assign40580_e38672_d_n4, assign40580_e38672_d_n5, assign40580_e38672_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__igindiode, var_fn456_calc_ig__igindiode_dn4, var_fn456_calc_ig__igindiode_dn5, var_fn456_calc_ig__igindiode_dn8,)
    }
};
        var_fn456_calc_ig__igindiode = assign40580_e38672;
        var_fn456_calc_ig__igindiode_dn4 = assign40580_e38672_d_n4;
        var_fn456_calc_ig__igindiode_dn5 = assign40580_e38672_d_n5;
        var_fn456_calc_ig__igindiode_dn8 = assign40580_e38672_d_n8;

        let (assign40590_e38680, assign40590_e38680_d_n5, assign40590_e38680_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__frecgin, var_fn456_calc_ig__frecgin_dn5, var_fn456_calc_ig__frecgin_dn8,)
    }
};
        var_fn456_calc_ig__frecgin = assign40590_e38680;
        var_fn456_calc_ig__frecgin_dn5 = assign40590_e38680_d_n5;
        var_fn456_calc_ig__frecgin_dn8 = assign40590_e38680_d_n8;

        let (assign40600_e38688, assign40600_e38688_d_n4, assign40600_e38688_d_n5, assign40600_e38688_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__iginrec, var_fn456_calc_ig__iginrec_dn4, var_fn456_calc_ig__iginrec_dn5, var_fn456_calc_ig__iginrec_dn8,)
    }
};
        var_fn456_calc_ig__iginrec = assign40600_e38688;
        var_fn456_calc_ig__iginrec_dn4 = assign40600_e38688_d_n4;
        var_fn456_calc_ig__iginrec_dn5 = assign40600_e38688_d_n5;
        var_fn456_calc_ig__iginrec_dn8 = assign40600_e38688_d_n8;

        let (assign40610_e38696, assign40610_e38696_d_n4, assign40610_e38696_d_n5, assign40610_e38696_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expbdarg1, var_fn456_calc_ig__expbdarg1_dn4, var_fn456_calc_ig__expbdarg1_dn5, var_fn456_calc_ig__expbdarg1_dn8,)
    }
};
        var_fn456_calc_ig__expbdarg1 = assign40610_e38696;
        var_fn456_calc_ig__expbdarg1_dn4 = assign40610_e38696_d_n4;
        var_fn456_calc_ig__expbdarg1_dn5 = assign40610_e38696_d_n5;
        var_fn456_calc_ig__expbdarg1_dn8 = assign40610_e38696_d_n8;

        let (assign40620_e38704, assign40620_e38704_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expbdarg2, var_fn456_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn456_calc_ig__expbdarg2 = assign40620_e38704;
        var_fn456_calc_ig__expbdarg2_dn4 = assign40620_e38704_d_n4;

        let (assign40630_e38712, assign40630_e38712_d_n4, assign40630_e38712_d_n5, assign40630_e38712_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expbd1, var_fn456_calc_ig__expbd1_dn4, var_fn456_calc_ig__expbd1_dn5, var_fn456_calc_ig__expbd1_dn8,)
    }
};
        var_fn456_calc_ig__expbd1 = assign40630_e38712;
        var_fn456_calc_ig__expbd1_dn4 = assign40630_e38712_d_n4;
        var_fn456_calc_ig__expbd1_dn5 = assign40630_e38712_d_n5;
        var_fn456_calc_ig__expbd1_dn8 = assign40630_e38712_d_n8;

        let (assign40640_e38720, assign40640_e38720_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expbd2, var_fn456_calc_ig__expbd2_dn4,)
    }
};
        var_fn456_calc_ig__expbd2 = assign40640_e38720;
        var_fn456_calc_ig__expbd2_dn4 = assign40640_e38720_d_n4;

        let (assign40650_e38728, assign40650_e38728_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expphib, var_fn456_calc_ig__expphib_dn4,)
    }
};
        var_fn456_calc_ig__expphib = assign40650_e38728;
        var_fn456_calc_ig__expphib_dn4 = assign40650_e38728_d_n4;

        let (assign40660_e38736, assign40660_e38736_d_n4, assign40660_e38736_d_n5, assign40660_e38736_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expffvarg, var_fn456_calc_ig__expffvarg_dn4, var_fn456_calc_ig__expffvarg_dn5, var_fn456_calc_ig__expffvarg_dn8,)
    }
};
        var_fn456_calc_ig__expffvarg = assign40660_e38736;
        var_fn456_calc_ig__expffvarg_dn4 = assign40660_e38736_d_n4;
        var_fn456_calc_ig__expffvarg_dn5 = assign40660_e38736_d_n5;
        var_fn456_calc_ig__expffvarg_dn8 = assign40660_e38736_d_n8;

        let (assign40670_e38744, assign40670_e38744_d_n4, assign40670_e38744_d_n5, assign40670_e38744_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expiforarg, var_fn456_calc_ig__expiforarg_dn4, var_fn456_calc_ig__expiforarg_dn5, var_fn456_calc_ig__expiforarg_dn8,)
    }
};
        var_fn456_calc_ig__expiforarg = assign40670_e38744;
        var_fn456_calc_ig__expiforarg_dn4 = assign40670_e38744_d_n4;
        var_fn456_calc_ig__expiforarg_dn5 = assign40670_e38744_d_n5;
        var_fn456_calc_ig__expiforarg_dn8 = assign40670_e38744_d_n8;

        let (assign40680_e38752, assign40680_e38752_d_n4, assign40680_e38752_d_n5, assign40680_e38752_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expifor, var_fn456_calc_ig__expifor_dn4, var_fn456_calc_ig__expifor_dn5, var_fn456_calc_ig__expifor_dn8,)
    }
};
        var_fn456_calc_ig__expifor = assign40680_e38752;
        var_fn456_calc_ig__expifor_dn4 = assign40680_e38752_d_n4;
        var_fn456_calc_ig__expifor_dn5 = assign40680_e38752_d_n5;
        var_fn456_calc_ig__expifor_dn8 = assign40680_e38752_d_n8;

        let (assign40690_e38760, assign40690_e38760_d_n4, assign40690_e38760_d_n5, assign40690_e38760_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expirevarg, var_fn456_calc_ig__expirevarg_dn4, var_fn456_calc_ig__expirevarg_dn5, var_fn456_calc_ig__expirevarg_dn8,)
    }
};
        var_fn456_calc_ig__expirevarg = assign40690_e38760;
        var_fn456_calc_ig__expirevarg_dn4 = assign40690_e38760_d_n4;
        var_fn456_calc_ig__expirevarg_dn5 = assign40690_e38760_d_n5;
        var_fn456_calc_ig__expirevarg_dn8 = assign40690_e38760_d_n8;

        let (assign40700_e38768, assign40700_e38768_d_n4, assign40700_e38768_d_n5, assign40700_e38768_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expirev, var_fn456_calc_ig__expirev_dn4, var_fn456_calc_ig__expirev_dn5, var_fn456_calc_ig__expirev_dn8,)
    }
};
        var_fn456_calc_ig__expirev = assign40700_e38768;
        var_fn456_calc_ig__expirev_dn4 = assign40700_e38768_d_n4;
        var_fn456_calc_ig__expirev_dn5 = assign40700_e38768_d_n5;
        var_fn456_calc_ig__expirev_dn8 = assign40700_e38768_d_n8;

        let (assign40710_e38776,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0,)
    } else {
        (var_fn456_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn456_calc_ig__pg_paramin_hinj = assign40710_e38776;

        let (assign40720_e38784, assign40720_e38784_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expbdarg1_vgsat, var_fn456_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__expbdarg1_vgsat = assign40720_e38784;
        var_fn456_calc_ig__expbdarg1_vgsat_dn4 = assign40720_e38784_d_n4;

        let (assign40730_e38792, assign40730_e38792_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expbd1_vgsat, var_fn456_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__expbd1_vgsat = assign40730_e38792;
        var_fn456_calc_ig__expbd1_vgsat_dn4 = assign40730_e38792_d_n4;

        let (assign40740_e38800, assign40740_e38800_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__iginbd_vgsat, var_fn456_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__iginbd_vgsat = assign40740_e38800;
        var_fn456_calc_ig__iginbd_vgsat_dn4 = assign40740_e38800_d_n4;

        let (assign40750_e38808, assign40750_e38808_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expiforarg_nohinj_vgsat, var_fn456_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__expiforarg_nohinj_vgsat = assign40750_e38808;
        var_fn456_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign40750_e38808_d_n4;

        let (assign40760_e38816, assign40760_e38816_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expifor_nohinj_vgsat, var_fn456_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__expifor_nohinj_vgsat = assign40760_e38816;
        var_fn456_calc_ig__expifor_nohinj_vgsat_dn4 = assign40760_e38816_d_n4;

        let (assign40770_e38824, assign40770_e38824_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__igindiode_nohinj_vgsat, var_fn456_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__igindiode_nohinj_vgsat = assign40770_e38824;
        var_fn456_calc_ig__igindiode_nohinj_vgsat_dn4 = assign40770_e38824_d_n4;

        let (assign40780_e38832, assign40780_e38832_d_n4, assign40780_e38832_d_n5, assign40780_e38832_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__igindiode_nohinj, var_fn456_calc_ig__igindiode_nohinj_dn4, var_fn456_calc_ig__igindiode_nohinj_dn5, var_fn456_calc_ig__igindiode_nohinj_dn8,)
    }
};
        var_fn456_calc_ig__igindiode_nohinj = assign40780_e38832;
        var_fn456_calc_ig__igindiode_nohinj_dn4 = assign40780_e38832_d_n4;
        var_fn456_calc_ig__igindiode_nohinj_dn5 = assign40780_e38832_d_n5;
        var_fn456_calc_ig__igindiode_nohinj_dn8 = assign40780_e38832_d_n8;

        let (assign40790_e38840, assign40790_e38840_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expiforarg_hinj_vgsat, var_fn456_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__expiforarg_hinj_vgsat = assign40790_e38840;
        var_fn456_calc_ig__expiforarg_hinj_vgsat_dn4 = assign40790_e38840_d_n4;

        let (assign40800_e38848, assign40800_e38848_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expifor_hinj_vgsat, var_fn456_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__expifor_hinj_vgsat = assign40800_e38848;
        var_fn456_calc_ig__expifor_hinj_vgsat_dn4 = assign40800_e38848_d_n4;

        let (assign40810_e38856, assign40810_e38856_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__igindiode_hinj_vgsat, var_fn456_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__igindiode_hinj_vgsat = assign40810_e38856;
        var_fn456_calc_ig__igindiode_hinj_vgsat_dn4 = assign40810_e38856_d_n4;

        let (assign40820_e38864, assign40820_e38864_d_n4, assign40820_e38864_d_n5, assign40820_e38864_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expiforarg_hinj, var_fn456_calc_ig__expiforarg_hinj_dn4, var_fn456_calc_ig__expiforarg_hinj_dn5, var_fn456_calc_ig__expiforarg_hinj_dn8,)
    }
};
        var_fn456_calc_ig__expiforarg_hinj = assign40820_e38864;
        var_fn456_calc_ig__expiforarg_hinj_dn4 = assign40820_e38864_d_n4;
        var_fn456_calc_ig__expiforarg_hinj_dn5 = assign40820_e38864_d_n5;
        var_fn456_calc_ig__expiforarg_hinj_dn8 = assign40820_e38864_d_n8;

        let (assign40830_e38872, assign40830_e38872_d_n4, assign40830_e38872_d_n5, assign40830_e38872_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__expifor_hinj, var_fn456_calc_ig__expifor_hinj_dn4, var_fn456_calc_ig__expifor_hinj_dn5, var_fn456_calc_ig__expifor_hinj_dn8,)
    }
};
        var_fn456_calc_ig__expifor_hinj = assign40830_e38872;
        var_fn456_calc_ig__expifor_hinj_dn4 = assign40830_e38872_d_n4;
        var_fn456_calc_ig__expifor_hinj_dn5 = assign40830_e38872_d_n5;
        var_fn456_calc_ig__expifor_hinj_dn8 = assign40830_e38872_d_n8;

        let (assign40840_e38880, assign40840_e38880_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__igindiode_hinj_pre, var_fn456_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn456_calc_ig__igindiode_hinj_pre = assign40840_e38880;
        var_fn456_calc_ig__igindiode_hinj_pre_dn4 = assign40840_e38880_d_n4;

        let (assign40850_e38888, assign40850_e38888_d_n4, assign40850_e38888_d_n5, assign40850_e38888_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__igindiode_hinj, var_fn456_calc_ig__igindiode_hinj_dn4, var_fn456_calc_ig__igindiode_hinj_dn5, var_fn456_calc_ig__igindiode_hinj_dn8,)
    }
};
        var_fn456_calc_ig__igindiode_hinj = assign40850_e38888;
        var_fn456_calc_ig__igindiode_hinj_dn4 = assign40850_e38888_d_n4;
        var_fn456_calc_ig__igindiode_hinj_dn5 = assign40850_e38888_d_n5;
        var_fn456_calc_ig__igindiode_hinj_dn8 = assign40850_e38888_d_n8;

        let (assign40860_e38901, assign40860_e38901_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40860_e38896: f64 = (var_fn456_calc_ig__pg_param1 / var_fn456_calc_ig__phitin);
        let assign40860_e38898: f64 = (-var_fn456_calc_ig__vjg);
        let assign40860_e38899: f64 = (assign40860_e38896 * assign40860_e38898);
        (assign40860_e38899, ((-((var_fn456_calc_ig__pg_param1 * var_fn456_calc_ig__phitin_dn4) / (var_fn456_calc_ig__phitin * var_fn456_calc_ig__phitin))) * assign40860_e38898),)
    } else {
        (var_fn456_calc_ig__expphib, var_fn456_calc_ig__expphib_dn4,)
    }
};
        var_fn456_calc_ig__expphib = assign40860_e38901;
        var_fn456_calc_ig__expphib_dn4 = assign40860_e38901_d_n4;

        let (assign40870_e38947, assign40870_e38947_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40870_e38913: f64 = (-50.0);
        let (assign40870_e38945, assign40870_e38945_d_n4,) = {
            if ((!(var_fn456_calc_ig__expphib > 50.0)) && (!(var_fn456_calc_ig__expphib < assign40870_e38913))) {
                let assign40870_e38918: f64 = (var_fn456_calc_ig__expphib).exp();
                (assign40870_e38918, (assign40870_e38918 * var_fn456_calc_ig__expphib_dn4),)
            } else {
                let assign40870_e38925: f64 = (-50.0);
                let (assign40870_e38944, assign40870_e38944_d_n4,) = {
                    if ((!(var_fn456_calc_ig__expphib > 50.0)) && (var_fn456_calc_ig__expphib < assign40870_e38925)) {
                        let assign40870_e38929: f64 = (-50.0);
                        let assign40870_e38930: f64 = (assign40870_e38929).exp();
                        (assign40870_e38930, 0.0,)
                    } else {
                        let (assign40870_e38943, assign40870_e38943_d_n4,) = {
                            if (var_fn456_calc_ig__expphib > 50.0) {
                                let assign40870_e38935: f64 = (50.0_f64).exp();
                                let assign40870_e38939: f64 = (var_fn456_calc_ig__expphib - 50.0);
                                let assign40870_e38940: f64 = (1.0 + assign40870_e38939);
                                let assign40870_e38941: f64 = (assign40870_e38935 * assign40870_e38940);
                                (assign40870_e38941, (assign40870_e38935 * var_fn456_calc_ig__expphib_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign40870_e38943, assign40870_e38943_d_n4,)
                    }
                };
                (assign40870_e38944, assign40870_e38944_d_n4,)
            }
        };
        (assign40870_e38945, assign40870_e38945_d_n4,)
    } else {
        (var_fn456_calc_ig__t0, var_fn456_calc_ig__t0_dn4,)
    }
};
        var_fn456_calc_ig__t0 = assign40870_e38947;
        var_fn456_calc_ig__t0_dn4 = assign40870_e38947_d_n4;

        let (assign40880_e38962, assign40880_e38962_d_n4, assign40880_e38962_d_n5, assign40880_e38962_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40880_e38955: f64 = (-var_fn456_calc_ig__vgin);
        let assign40880_e38957: f64 = (assign40880_e38955 - var_fn456_calc_ig__vbdgin);
        let assign40880_e38958: f64 = (var_fn456_calc_ig__pbdgin * assign40880_e38957);
        let assign40880_e38960: f64 = (assign40880_e38958 + var_fn456_calc_ig__expphib);
        (assign40880_e38960, var_fn456_calc_ig__expphib_dn4, (var_fn456_calc_ig__pbdgin * (-var_fn456_calc_ig__vgin_dn5)), (var_fn456_calc_ig__pbdgin * (-var_fn456_calc_ig__vgin_dn8)),)
    } else {
        (var_fn456_calc_ig__expbdarg1, var_fn456_calc_ig__expbdarg1_dn4, var_fn456_calc_ig__expbdarg1_dn5, var_fn456_calc_ig__expbdarg1_dn8,)
    }
};
        var_fn456_calc_ig__expbdarg1 = assign40880_e38962;
        var_fn456_calc_ig__expbdarg1_dn4 = assign40880_e38962_d_n4;
        var_fn456_calc_ig__expbdarg1_dn5 = assign40880_e38962_d_n5;
        var_fn456_calc_ig__expbdarg1_dn8 = assign40880_e38962_d_n8;

        let (assign40890_e38975, assign40890_e38975_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40890_e38969: f64 = (-var_fn456_calc_ig__pbdgin);
        let assign40890_e38971: f64 = (assign40890_e38969 * var_fn456_calc_ig__vbdgin);
        let assign40890_e38973: f64 = (assign40890_e38971 + var_fn456_calc_ig__expphib);
        (assign40890_e38973, var_fn456_calc_ig__expphib_dn4,)
    } else {
        (var_fn456_calc_ig__expbdarg2, var_fn456_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn456_calc_ig__expbdarg2 = assign40890_e38975;
        var_fn456_calc_ig__expbdarg2_dn4 = assign40890_e38975_d_n4;

        *var_fn456_calc_ig__alpha2_phit_slot = var_fn456_calc_ig__alpha2_phit;
        *var_fn456_calc_ig__alpha2_phit_dn4_slot = var_fn456_calc_ig__alpha2_phit_dn4;
        *var_fn456_calc_ig__expbd1_slot = var_fn456_calc_ig__expbd1;
        *var_fn456_calc_ig__expbd1_dn4_slot = var_fn456_calc_ig__expbd1_dn4;
        *var_fn456_calc_ig__expbd1_dn5_slot = var_fn456_calc_ig__expbd1_dn5;
        *var_fn456_calc_ig__expbd1_dn8_slot = var_fn456_calc_ig__expbd1_dn8;
        *var_fn456_calc_ig__expbd1_vgsat_slot = var_fn456_calc_ig__expbd1_vgsat;
        *var_fn456_calc_ig__expbd1_vgsat_dn4_slot = var_fn456_calc_ig__expbd1_vgsat_dn4;
        *var_fn456_calc_ig__expbd2_slot = var_fn456_calc_ig__expbd2;
        *var_fn456_calc_ig__expbd2_dn4_slot = var_fn456_calc_ig__expbd2_dn4;
        *var_fn456_calc_ig__expbdarg1_slot = var_fn456_calc_ig__expbdarg1;
        *var_fn456_calc_ig__expbdarg1_dn4_slot = var_fn456_calc_ig__expbdarg1_dn4;
        *var_fn456_calc_ig__expbdarg1_dn5_slot = var_fn456_calc_ig__expbdarg1_dn5;
        *var_fn456_calc_ig__expbdarg1_dn8_slot = var_fn456_calc_ig__expbdarg1_dn8;
        *var_fn456_calc_ig__expbdarg1_vgsat_slot = var_fn456_calc_ig__expbdarg1_vgsat;
        *var_fn456_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn456_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn456_calc_ig__expbdarg2_slot = var_fn456_calc_ig__expbdarg2;
        *var_fn456_calc_ig__expbdarg2_dn4_slot = var_fn456_calc_ig__expbdarg2_dn4;
        *var_fn456_calc_ig__expffvarg_slot = var_fn456_calc_ig__expffvarg;
        *var_fn456_calc_ig__expffvarg_dn4_slot = var_fn456_calc_ig__expffvarg_dn4;
        *var_fn456_calc_ig__expffvarg_dn5_slot = var_fn456_calc_ig__expffvarg_dn5;
        *var_fn456_calc_ig__expffvarg_dn8_slot = var_fn456_calc_ig__expffvarg_dn8;
        *var_fn456_calc_ig__expifor_slot = var_fn456_calc_ig__expifor;
        *var_fn456_calc_ig__expifor_dn4_slot = var_fn456_calc_ig__expifor_dn4;
        *var_fn456_calc_ig__expifor_dn5_slot = var_fn456_calc_ig__expifor_dn5;
        *var_fn456_calc_ig__expifor_dn8_slot = var_fn456_calc_ig__expifor_dn8;
        *var_fn456_calc_ig__expifor_hinj_slot = var_fn456_calc_ig__expifor_hinj;
        *var_fn456_calc_ig__expifor_hinj_dn4_slot = var_fn456_calc_ig__expifor_hinj_dn4;
        *var_fn456_calc_ig__expifor_hinj_dn5_slot = var_fn456_calc_ig__expifor_hinj_dn5;
        *var_fn456_calc_ig__expifor_hinj_dn8_slot = var_fn456_calc_ig__expifor_hinj_dn8;
        *var_fn456_calc_ig__expifor_hinj_vgsat_slot = var_fn456_calc_ig__expifor_hinj_vgsat;
        *var_fn456_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn456_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn456_calc_ig__expifor_nohinj_vgsat_slot = var_fn456_calc_ig__expifor_nohinj_vgsat;
        *var_fn456_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn456_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn456_calc_ig__expiforarg_slot = var_fn456_calc_ig__expiforarg;
        *var_fn456_calc_ig__expiforarg_dn4_slot = var_fn456_calc_ig__expiforarg_dn4;
        *var_fn456_calc_ig__expiforarg_dn5_slot = var_fn456_calc_ig__expiforarg_dn5;
        *var_fn456_calc_ig__expiforarg_dn8_slot = var_fn456_calc_ig__expiforarg_dn8;
        *var_fn456_calc_ig__expiforarg_hinj_slot = var_fn456_calc_ig__expiforarg_hinj;
        *var_fn456_calc_ig__expiforarg_hinj_dn4_slot = var_fn456_calc_ig__expiforarg_hinj_dn4;
        *var_fn456_calc_ig__expiforarg_hinj_dn5_slot = var_fn456_calc_ig__expiforarg_hinj_dn5;
        *var_fn456_calc_ig__expiforarg_hinj_dn8_slot = var_fn456_calc_ig__expiforarg_hinj_dn8;
        *var_fn456_calc_ig__expiforarg_hinj_vgsat_slot = var_fn456_calc_ig__expiforarg_hinj_vgsat;
        *var_fn456_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn456_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn456_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn456_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn456_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn456_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn456_calc_ig__expirev_slot = var_fn456_calc_ig__expirev;
        *var_fn456_calc_ig__expirev_dn4_slot = var_fn456_calc_ig__expirev_dn4;
        *var_fn456_calc_ig__expirev_dn5_slot = var_fn456_calc_ig__expirev_dn5;
        *var_fn456_calc_ig__expirev_dn8_slot = var_fn456_calc_ig__expirev_dn8;
        *var_fn456_calc_ig__expirevarg_slot = var_fn456_calc_ig__expirevarg;
        *var_fn456_calc_ig__expirevarg_dn4_slot = var_fn456_calc_ig__expirevarg_dn4;
        *var_fn456_calc_ig__expirevarg_dn5_slot = var_fn456_calc_ig__expirevarg_dn5;
        *var_fn456_calc_ig__expirevarg_dn8_slot = var_fn456_calc_ig__expirevarg_dn8;
        *var_fn456_calc_ig__expphib_slot = var_fn456_calc_ig__expphib;
        *var_fn456_calc_ig__expphib_dn4_slot = var_fn456_calc_ig__expphib_dn4;
        *var_fn456_calc_ig__ffvgin_slot = var_fn456_calc_ig__ffvgin;
        *var_fn456_calc_ig__ffvgin_dn4_slot = var_fn456_calc_ig__ffvgin_dn4;
        *var_fn456_calc_ig__ffvgin_dn5_slot = var_fn456_calc_ig__ffvgin_dn5;
        *var_fn456_calc_ig__ffvgin_dn8_slot = var_fn456_calc_ig__ffvgin_dn8;
        *var_fn456_calc_ig__frecgin_slot = var_fn456_calc_ig__frecgin;
        *var_fn456_calc_ig__frecgin_dn5_slot = var_fn456_calc_ig__frecgin_dn5;
        *var_fn456_calc_ig__frecgin_dn8_slot = var_fn456_calc_ig__frecgin_dn8;
        *var_fn456_calc_ig__iginbd_slot = var_fn456_calc_ig__iginbd;
        *var_fn456_calc_ig__iginbd_dn4_slot = var_fn456_calc_ig__iginbd_dn4;
        *var_fn456_calc_ig__iginbd_dn5_slot = var_fn456_calc_ig__iginbd_dn5;
        *var_fn456_calc_ig__iginbd_dn8_slot = var_fn456_calc_ig__iginbd_dn8;
        *var_fn456_calc_ig__iginbd_vgsat_slot = var_fn456_calc_ig__iginbd_vgsat;
        *var_fn456_calc_ig__iginbd_vgsat_dn4_slot = var_fn456_calc_ig__iginbd_vgsat_dn4;
        *var_fn456_calc_ig__igindiode_slot = var_fn456_calc_ig__igindiode;
        *var_fn456_calc_ig__igindiode_dn4_slot = var_fn456_calc_ig__igindiode_dn4;
        *var_fn456_calc_ig__igindiode_dn5_slot = var_fn456_calc_ig__igindiode_dn5;
        *var_fn456_calc_ig__igindiode_dn8_slot = var_fn456_calc_ig__igindiode_dn8;
        *var_fn456_calc_ig__igindiode_hinj_slot = var_fn456_calc_ig__igindiode_hinj;
        *var_fn456_calc_ig__igindiode_hinj_dn4_slot = var_fn456_calc_ig__igindiode_hinj_dn4;
        *var_fn456_calc_ig__igindiode_hinj_dn5_slot = var_fn456_calc_ig__igindiode_hinj_dn5;
        *var_fn456_calc_ig__igindiode_hinj_dn8_slot = var_fn456_calc_ig__igindiode_hinj_dn8;
        *var_fn456_calc_ig__igindiode_hinj_pre_slot = var_fn456_calc_ig__igindiode_hinj_pre;
        *var_fn456_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn456_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn456_calc_ig__igindiode_hinj_vgsat_slot = var_fn456_calc_ig__igindiode_hinj_vgsat;
        *var_fn456_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn456_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn456_calc_ig__igindiode_nohinj_slot = var_fn456_calc_ig__igindiode_nohinj;
        *var_fn456_calc_ig__igindiode_nohinj_dn4_slot = var_fn456_calc_ig__igindiode_nohinj_dn4;
        *var_fn456_calc_ig__igindiode_nohinj_dn5_slot = var_fn456_calc_ig__igindiode_nohinj_dn5;
        *var_fn456_calc_ig__igindiode_nohinj_dn8_slot = var_fn456_calc_ig__igindiode_nohinj_dn8;
        *var_fn456_calc_ig__igindiode_nohinj_vgsat_slot = var_fn456_calc_ig__igindiode_nohinj_vgsat;
        *var_fn456_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn456_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn456_calc_ig__iginrec_slot = var_fn456_calc_ig__iginrec;
        *var_fn456_calc_ig__iginrec_dn4_slot = var_fn456_calc_ig__iginrec_dn4;
        *var_fn456_calc_ig__iginrec_dn5_slot = var_fn456_calc_ig__iginrec_dn5;
        *var_fn456_calc_ig__iginrec_dn8_slot = var_fn456_calc_ig__iginrec_dn8;
        *var_fn456_calc_ig__igout_slot = var_fn456_calc_ig__igout;
        *var_fn456_calc_ig__igout_dn4_slot = var_fn456_calc_ig__igout_dn4;
        *var_fn456_calc_ig__igout_dn5_slot = var_fn456_calc_ig__igout_dn5;
        *var_fn456_calc_ig__igout_dn8_slot = var_fn456_calc_ig__igout_dn8;
        *var_fn456_calc_ig__irecin_slot = var_fn456_calc_ig__irecin;
        *var_fn456_calc_ig__pg_param1_slot = var_fn456_calc_ig__pg_param1;
        *var_fn456_calc_ig__pg_paramin_hinj_slot = var_fn456_calc_ig__pg_paramin_hinj;
        *var_fn456_calc_ig__pgsrecin_slot = var_fn456_calc_ig__pgsrecin;
        *var_fn456_calc_ig__t0_slot = var_fn456_calc_ig__t0;
        *var_fn456_calc_ig__t0_dn4_slot = var_fn456_calc_ig__t0_dn4;
        *var_fn456_calc_ig__type_slot = var_fn456_calc_ig__type;
        *var_fn456_calc_ig__vjg_slot = var_fn456_calc_ig__vjg;
    }

    pub(super) fn stamp_transient_block_102(
        var_fn456_calc_ig__expbdarg1: f64,
        var_fn456_calc_ig__expbdarg1_dn4: f64,
        var_fn456_calc_ig__expbdarg1_dn5: f64,
        var_fn456_calc_ig__expbdarg1_dn8: f64,
        var_fn456_calc_ig__expbdarg2: f64,
        var_fn456_calc_ig__expbdarg2_dn4: f64,
        var_fn456_calc_ig__expphib: f64,
        var_fn456_calc_ig__expphib_dn4: f64,
        var_fn456_calc_ig__fracin: f64,
        var_fn456_calc_ig__ijin: f64,
        var_fn456_calc_ig__kbdgatein: f64,
        var_fn456_calc_ig__ngf: f64,
        var_fn456_calc_ig__pbdgin: f64,
        var_fn456_calc_ig__pg_paramin: f64,
        var_fn456_calc_ig__phitin: f64,
        var_fn456_calc_ig__phitin_dn4: f64,
        var_fn456_calc_ig__t0: f64,
        var_fn456_calc_ig__t0_dn4: f64,
        var_fn456_calc_ig__tfacdiodein: f64,
        var_fn456_calc_ig__tfacdiodein_dn4: f64,
        var_fn456_calc_ig__type: f64,
        var_fn456_calc_ig__vbdgin: f64,
        var_fn456_calc_ig__vgin: f64,
        var_fn456_calc_ig__vgin_dn5: f64,
        var_fn456_calc_ig__vgin_dn8: f64,
        var_fn456_calc_ig__vgsatin: f64,
        var_fn456_calc_ig__w: f64,
        var_guard417: f64,
        var_guard439: f64,
        var_guard450: f64,
        var_fn456_calc_ig__expbd1_slot: &mut f64,
        var_fn456_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn456_calc_ig__expbd1_dn5_slot: &mut f64,
        var_fn456_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn456_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn456_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__expbd2_slot: &mut f64,
        var_fn456_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn456_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn456_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__expifor_slot: &mut f64,
        var_fn456_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn456_calc_ig__expifor_dn5_slot: &mut f64,
        var_fn456_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn456_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn456_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn456_calc_ig__expifor_hinj_dn5_slot: &mut f64,
        var_fn456_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn456_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn456_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn456_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_dn5_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_hinj_dn5_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn456_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__iginbd_slot: &mut f64,
        var_fn456_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn456_calc_ig__iginbd_dn5_slot: &mut f64,
        var_fn456_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn456_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn456_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__igindiode_slot: &mut f64,
        var_fn456_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn456_calc_ig__igindiode_dn5_slot: &mut f64,
        var_fn456_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn456_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn456_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn456_calc_ig__igindiode_hinj_dn5_slot: &mut f64,
        var_fn456_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn456_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn456_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn456_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn456_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn456_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn456_calc_ig__igindiode_nohinj_dn5_slot: &mut f64,
        var_fn456_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn456_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn456_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn456_calc_ig__isdiodeout_slot: &mut f64,
        var_fn456_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn456_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_guard457_slot: &mut f64,
        var_guard458_slot: &mut f64,
    ) {
        let mut var_fn456_calc_ig__expbd1: f64 = *var_fn456_calc_ig__expbd1_slot;
        let mut var_fn456_calc_ig__expbd1_dn4: f64 = *var_fn456_calc_ig__expbd1_dn4_slot;
        let mut var_fn456_calc_ig__expbd1_dn5: f64 = *var_fn456_calc_ig__expbd1_dn5_slot;
        let mut var_fn456_calc_ig__expbd1_dn8: f64 = *var_fn456_calc_ig__expbd1_dn8_slot;
        let mut var_fn456_calc_ig__expbd1_vgsat: f64 = *var_fn456_calc_ig__expbd1_vgsat_slot;
        let mut var_fn456_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn456_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__expbd2: f64 = *var_fn456_calc_ig__expbd2_slot;
        let mut var_fn456_calc_ig__expbd2_dn4: f64 = *var_fn456_calc_ig__expbd2_dn4_slot;
        let mut var_fn456_calc_ig__expbdarg1_vgsat: f64 = *var_fn456_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn456_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn456_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__expifor: f64 = *var_fn456_calc_ig__expifor_slot;
        let mut var_fn456_calc_ig__expifor_dn4: f64 = *var_fn456_calc_ig__expifor_dn4_slot;
        let mut var_fn456_calc_ig__expifor_dn5: f64 = *var_fn456_calc_ig__expifor_dn5_slot;
        let mut var_fn456_calc_ig__expifor_dn8: f64 = *var_fn456_calc_ig__expifor_dn8_slot;
        let mut var_fn456_calc_ig__expifor_hinj: f64 = *var_fn456_calc_ig__expifor_hinj_slot;
        let mut var_fn456_calc_ig__expifor_hinj_dn4: f64 = *var_fn456_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn456_calc_ig__expifor_hinj_dn5: f64 = *var_fn456_calc_ig__expifor_hinj_dn5_slot;
        let mut var_fn456_calc_ig__expifor_hinj_dn8: f64 = *var_fn456_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn456_calc_ig__expifor_hinj_vgsat: f64 = *var_fn456_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn456_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn456_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn456_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn456_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn456_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__expiforarg: f64 = *var_fn456_calc_ig__expiforarg_slot;
        let mut var_fn456_calc_ig__expiforarg_dn4: f64 = *var_fn456_calc_ig__expiforarg_dn4_slot;
        let mut var_fn456_calc_ig__expiforarg_dn5: f64 = *var_fn456_calc_ig__expiforarg_dn5_slot;
        let mut var_fn456_calc_ig__expiforarg_dn8: f64 = *var_fn456_calc_ig__expiforarg_dn8_slot;
        let mut var_fn456_calc_ig__expiforarg_hinj: f64 = *var_fn456_calc_ig__expiforarg_hinj_slot;
        let mut var_fn456_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn456_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn456_calc_ig__expiforarg_hinj_dn5: f64 = *var_fn456_calc_ig__expiforarg_hinj_dn5_slot;
        let mut var_fn456_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn456_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn456_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn456_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn456_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn456_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn456_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn456_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn456_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__iginbd: f64 = *var_fn456_calc_ig__iginbd_slot;
        let mut var_fn456_calc_ig__iginbd_dn4: f64 = *var_fn456_calc_ig__iginbd_dn4_slot;
        let mut var_fn456_calc_ig__iginbd_dn5: f64 = *var_fn456_calc_ig__iginbd_dn5_slot;
        let mut var_fn456_calc_ig__iginbd_dn8: f64 = *var_fn456_calc_ig__iginbd_dn8_slot;
        let mut var_fn456_calc_ig__iginbd_vgsat: f64 = *var_fn456_calc_ig__iginbd_vgsat_slot;
        let mut var_fn456_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn456_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__igindiode: f64 = *var_fn456_calc_ig__igindiode_slot;
        let mut var_fn456_calc_ig__igindiode_dn4: f64 = *var_fn456_calc_ig__igindiode_dn4_slot;
        let mut var_fn456_calc_ig__igindiode_dn5: f64 = *var_fn456_calc_ig__igindiode_dn5_slot;
        let mut var_fn456_calc_ig__igindiode_dn8: f64 = *var_fn456_calc_ig__igindiode_dn8_slot;
        let mut var_fn456_calc_ig__igindiode_hinj: f64 = *var_fn456_calc_ig__igindiode_hinj_slot;
        let mut var_fn456_calc_ig__igindiode_hinj_dn4: f64 = *var_fn456_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn456_calc_ig__igindiode_hinj_dn5: f64 = *var_fn456_calc_ig__igindiode_hinj_dn5_slot;
        let mut var_fn456_calc_ig__igindiode_hinj_dn8: f64 = *var_fn456_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn456_calc_ig__igindiode_hinj_pre: f64 = *var_fn456_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn456_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn456_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn456_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn456_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn456_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn456_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__igindiode_nohinj: f64 = *var_fn456_calc_ig__igindiode_nohinj_slot;
        let mut var_fn456_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn456_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn456_calc_ig__igindiode_nohinj_dn5: f64 = *var_fn456_calc_ig__igindiode_nohinj_dn5_slot;
        let mut var_fn456_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn456_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn456_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn456_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn456_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn456_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn456_calc_ig__isdiodeout: f64 = *var_fn456_calc_ig__isdiodeout_slot;
        let mut var_fn456_calc_ig__isdiodeout_dn4: f64 = *var_fn456_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn456_calc_ig__pg_paramin_hinj: f64 = *var_fn456_calc_ig__pg_paramin_hinj_slot;
        let mut var_guard457: f64 = *var_guard457_slot;
        let mut var_guard458: f64 = *var_guard458_slot;

        let (assign40900_e39021, assign40900_e39021_d_n4, assign40900_e39021_d_n5, assign40900_e39021_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40900_e38987: f64 = (-50.0);
        let (assign40900_e39019, assign40900_e39019_d_n4, assign40900_e39019_d_n5, assign40900_e39019_d_n8,) = {
            if ((!(var_fn456_calc_ig__expbdarg1 > 50.0)) && (!(var_fn456_calc_ig__expbdarg1 < assign40900_e38987))) {
                let assign40900_e38992: f64 = (var_fn456_calc_ig__expbdarg1).exp();
                (assign40900_e38992, (assign40900_e38992 * var_fn456_calc_ig__expbdarg1_dn4), (assign40900_e38992 * var_fn456_calc_ig__expbdarg1_dn5), (assign40900_e38992 * var_fn456_calc_ig__expbdarg1_dn8),)
            } else {
                let assign40900_e38999: f64 = (-50.0);
                let (assign40900_e39018, assign40900_e39018_d_n4, assign40900_e39018_d_n5, assign40900_e39018_d_n8,) = {
                    if ((!(var_fn456_calc_ig__expbdarg1 > 50.0)) && (var_fn456_calc_ig__expbdarg1 < assign40900_e38999)) {
                        let assign40900_e39003: f64 = (-50.0);
                        let assign40900_e39004: f64 = (assign40900_e39003).exp();
                        (assign40900_e39004, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign40900_e39017, assign40900_e39017_d_n4, assign40900_e39017_d_n5, assign40900_e39017_d_n8,) = {
                            if (var_fn456_calc_ig__expbdarg1 > 50.0) {
                                let assign40900_e39009: f64 = (50.0_f64).exp();
                                let assign40900_e39013: f64 = (var_fn456_calc_ig__expbdarg1 - 50.0);
                                let assign40900_e39014: f64 = (1.0 + assign40900_e39013);
                                let assign40900_e39015: f64 = (assign40900_e39009 * assign40900_e39014);
                                (assign40900_e39015, (assign40900_e39009 * var_fn456_calc_ig__expbdarg1_dn4), (assign40900_e39009 * var_fn456_calc_ig__expbdarg1_dn5), (assign40900_e39009 * var_fn456_calc_ig__expbdarg1_dn8),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign40900_e39017, assign40900_e39017_d_n4, assign40900_e39017_d_n5, assign40900_e39017_d_n8,)
                    }
                };
                (assign40900_e39018, assign40900_e39018_d_n4, assign40900_e39018_d_n5, assign40900_e39018_d_n8,)
            }
        };
        (assign40900_e39019, assign40900_e39019_d_n4, assign40900_e39019_d_n5, assign40900_e39019_d_n8,)
    } else {
        (var_fn456_calc_ig__expbd1, var_fn456_calc_ig__expbd1_dn4, var_fn456_calc_ig__expbd1_dn5, var_fn456_calc_ig__expbd1_dn8,)
    }
};
        var_fn456_calc_ig__expbd1 = assign40900_e39021;
        var_fn456_calc_ig__expbd1_dn4 = assign40900_e39021_d_n4;
        var_fn456_calc_ig__expbd1_dn5 = assign40900_e39021_d_n5;
        var_fn456_calc_ig__expbd1_dn8 = assign40900_e39021_d_n8;

        let (assign40910_e39067, assign40910_e39067_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40910_e39033: f64 = (-50.0);
        let (assign40910_e39065, assign40910_e39065_d_n4,) = {
            if ((!(var_fn456_calc_ig__expbdarg2 > 50.0)) && (!(var_fn456_calc_ig__expbdarg2 < assign40910_e39033))) {
                let assign40910_e39038: f64 = (var_fn456_calc_ig__expbdarg2).exp();
                (assign40910_e39038, (assign40910_e39038 * var_fn456_calc_ig__expbdarg2_dn4),)
            } else {
                let assign40910_e39045: f64 = (-50.0);
                let (assign40910_e39064, assign40910_e39064_d_n4,) = {
                    if ((!(var_fn456_calc_ig__expbdarg2 > 50.0)) && (var_fn456_calc_ig__expbdarg2 < assign40910_e39045)) {
                        let assign40910_e39049: f64 = (-50.0);
                        let assign40910_e39050: f64 = (assign40910_e39049).exp();
                        (assign40910_e39050, 0.0,)
                    } else {
                        let (assign40910_e39063, assign40910_e39063_d_n4,) = {
                            if (var_fn456_calc_ig__expbdarg2 > 50.0) {
                                let assign40910_e39055: f64 = (50.0_f64).exp();
                                let assign40910_e39059: f64 = (var_fn456_calc_ig__expbdarg2 - 50.0);
                                let assign40910_e39060: f64 = (1.0 + assign40910_e39059);
                                let assign40910_e39061: f64 = (assign40910_e39055 * assign40910_e39060);
                                (assign40910_e39061, (assign40910_e39055 * var_fn456_calc_ig__expbdarg2_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign40910_e39063, assign40910_e39063_d_n4,)
                    }
                };
                (assign40910_e39064, assign40910_e39064_d_n4,)
            }
        };
        (assign40910_e39065, assign40910_e39065_d_n4,)
    } else {
        (var_fn456_calc_ig__expbd2, var_fn456_calc_ig__expbd2_dn4,)
    }
};
        var_fn456_calc_ig__expbd2 = assign40910_e39067;
        var_fn456_calc_ig__expbd2_dn4 = assign40910_e39067_d_n4;

        let (assign40920_e39077, assign40920_e39077_d_n4, assign40920_e39077_d_n5, assign40920_e39077_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40920_e39075: f64 = (var_fn456_calc_ig__expbd1 - var_fn456_calc_ig__expbd2);
        (assign40920_e39075, (var_fn456_calc_ig__expbd1_dn4 - var_fn456_calc_ig__expbd2_dn4), var_fn456_calc_ig__expbd1_dn5, var_fn456_calc_ig__expbd1_dn8,)
    } else {
        (var_fn456_calc_ig__iginbd, var_fn456_calc_ig__iginbd_dn4, var_fn456_calc_ig__iginbd_dn5, var_fn456_calc_ig__iginbd_dn8,)
    }
};
        var_fn456_calc_ig__iginbd = assign40920_e39077;
        var_fn456_calc_ig__iginbd_dn4 = assign40920_e39077_d_n4;
        var_fn456_calc_ig__iginbd_dn5 = assign40920_e39077_d_n5;
        var_fn456_calc_ig__iginbd_dn8 = assign40920_e39077_d_n8;

        let (assign40930_e39093, assign40930_e39093_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40930_e39085: f64 = (var_fn456_calc_ig__type * var_fn456_calc_ig__w);
        let assign40930_e39087: f64 = (assign40930_e39085 * var_fn456_calc_ig__ngf);
        let assign40930_e39089: f64 = (assign40930_e39087 * var_fn456_calc_ig__ijin);
        let assign40930_e39091: f64 = (assign40930_e39089 * var_fn456_calc_ig__tfacdiodein);
        (assign40930_e39091, (assign40930_e39089 * var_fn456_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn456_calc_ig__isdiodeout, var_fn456_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn456_calc_ig__isdiodeout = assign40930_e39093;
        var_fn456_calc_ig__isdiodeout_dn4 = assign40930_e39093_d_n4;

        let (assign40940_e39107, assign40940_e39107_d_n4, assign40940_e39107_d_n5, assign40940_e39107_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40940_e39101: f64 = (var_fn456_calc_ig__pg_paramin / var_fn456_calc_ig__phitin);
        let assign40940_e39103: f64 = (assign40940_e39101 * var_fn456_calc_ig__vgin);
        let assign40940_e39105: f64 = (assign40940_e39103 + var_fn456_calc_ig__expphib);
        (assign40940_e39105, (((-((var_fn456_calc_ig__pg_paramin * var_fn456_calc_ig__phitin_dn4) / (var_fn456_calc_ig__phitin * var_fn456_calc_ig__phitin))) * var_fn456_calc_ig__vgin) + var_fn456_calc_ig__expphib_dn4), (assign40940_e39101 * var_fn456_calc_ig__vgin_dn5), (assign40940_e39101 * var_fn456_calc_ig__vgin_dn8),)
    } else {
        (var_fn456_calc_ig__expiforarg, var_fn456_calc_ig__expiforarg_dn4, var_fn456_calc_ig__expiforarg_dn5, var_fn456_calc_ig__expiforarg_dn8,)
    }
};
        var_fn456_calc_ig__expiforarg = assign40940_e39107;
        var_fn456_calc_ig__expiforarg_dn4 = assign40940_e39107_d_n4;
        var_fn456_calc_ig__expiforarg_dn5 = assign40940_e39107_d_n5;
        var_fn456_calc_ig__expiforarg_dn8 = assign40940_e39107_d_n8;

        let (assign40950_e39153, assign40950_e39153_d_n4, assign40950_e39153_d_n5, assign40950_e39153_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign40950_e39119: f64 = (-50.0);
        let (assign40950_e39151, assign40950_e39151_d_n4, assign40950_e39151_d_n5, assign40950_e39151_d_n8,) = {
            if ((!(var_fn456_calc_ig__expiforarg > 50.0)) && (!(var_fn456_calc_ig__expiforarg < assign40950_e39119))) {
                let assign40950_e39124: f64 = (var_fn456_calc_ig__expiforarg).exp();
                (assign40950_e39124, (assign40950_e39124 * var_fn456_calc_ig__expiforarg_dn4), (assign40950_e39124 * var_fn456_calc_ig__expiforarg_dn5), (assign40950_e39124 * var_fn456_calc_ig__expiforarg_dn8),)
            } else {
                let assign40950_e39131: f64 = (-50.0);
                let (assign40950_e39150, assign40950_e39150_d_n4, assign40950_e39150_d_n5, assign40950_e39150_d_n8,) = {
                    if ((!(var_fn456_calc_ig__expiforarg > 50.0)) && (var_fn456_calc_ig__expiforarg < assign40950_e39131)) {
                        let assign40950_e39135: f64 = (-50.0);
                        let assign40950_e39136: f64 = (assign40950_e39135).exp();
                        (assign40950_e39136, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign40950_e39149, assign40950_e39149_d_n4, assign40950_e39149_d_n5, assign40950_e39149_d_n8,) = {
                            if (var_fn456_calc_ig__expiforarg > 50.0) {
                                let assign40950_e39141: f64 = (50.0_f64).exp();
                                let assign40950_e39145: f64 = (var_fn456_calc_ig__expiforarg - 50.0);
                                let assign40950_e39146: f64 = (1.0 + assign40950_e39145);
                                let assign40950_e39147: f64 = (assign40950_e39141 * assign40950_e39146);
                                (assign40950_e39147, (assign40950_e39141 * var_fn456_calc_ig__expiforarg_dn4), (assign40950_e39141 * var_fn456_calc_ig__expiforarg_dn5), (assign40950_e39141 * var_fn456_calc_ig__expiforarg_dn8),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign40950_e39149, assign40950_e39149_d_n4, assign40950_e39149_d_n5, assign40950_e39149_d_n8,)
                    }
                };
                (assign40950_e39150, assign40950_e39150_d_n4, assign40950_e39150_d_n5, assign40950_e39150_d_n8,)
            }
        };
        (assign40950_e39151, assign40950_e39151_d_n4, assign40950_e39151_d_n5, assign40950_e39151_d_n8,)
    } else {
        (var_fn456_calc_ig__expifor, var_fn456_calc_ig__expifor_dn4, var_fn456_calc_ig__expifor_dn5, var_fn456_calc_ig__expifor_dn8,)
    }
};
        var_fn456_calc_ig__expifor = assign40950_e39153;
        var_fn456_calc_ig__expifor_dn4 = assign40950_e39153_d_n4;
        var_fn456_calc_ig__expifor_dn5 = assign40950_e39153_d_n5;
        var_fn456_calc_ig__expifor_dn8 = assign40950_e39153_d_n8;

        let assign40960_e39156: f64 = if var_fn456_calc_ig__fracin == 1.0 { 1.0 } else { 0.0 };
        var_guard457 = assign40960_e39156;

        let (assign40970_e39174, assign40970_e39174_d_n4, assign40970_e39174_d_n5, assign40970_e39174_d_n8,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 != 0.0)) {
        let assign40970_e39168: f64 = (var_fn456_calc_ig__kbdgatein * var_fn456_calc_ig__iginbd);
        let assign40970_e39169: f64 = (var_fn456_calc_ig__expifor - assign40970_e39168);
        let assign40970_e39171: f64 = (assign40970_e39169 - var_fn456_calc_ig__t0);
        let assign40970_e39172: f64 = (var_fn456_calc_ig__isdiodeout * assign40970_e39171);
        (assign40970_e39172, ((var_fn456_calc_ig__isdiodeout_dn4 * assign40970_e39171) + (var_fn456_calc_ig__isdiodeout * ((var_fn456_calc_ig__expifor_dn4 - (var_fn456_calc_ig__kbdgatein * var_fn456_calc_ig__iginbd_dn4)) - var_fn456_calc_ig__t0_dn4))), (var_fn456_calc_ig__isdiodeout * (var_fn456_calc_ig__expifor_dn5 - (var_fn456_calc_ig__kbdgatein * var_fn456_calc_ig__iginbd_dn5))), (var_fn456_calc_ig__isdiodeout * (var_fn456_calc_ig__expifor_dn8 - (var_fn456_calc_ig__kbdgatein * var_fn456_calc_ig__iginbd_dn8))),)
    } else {
        (var_fn456_calc_ig__igindiode, var_fn456_calc_ig__igindiode_dn4, var_fn456_calc_ig__igindiode_dn5, var_fn456_calc_ig__igindiode_dn8,)
    }
};
        var_fn456_calc_ig__igindiode = assign40970_e39174;
        var_fn456_calc_ig__igindiode_dn4 = assign40970_e39174_d_n4;
        var_fn456_calc_ig__igindiode_dn5 = assign40970_e39174_d_n5;
        var_fn456_calc_ig__igindiode_dn8 = assign40970_e39174_d_n8;

        let (assign40980_e39192, assign40980_e39192_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) {
        let assign40980_e39185: f64 = (-var_fn456_calc_ig__vgsatin);
        let assign40980_e39187: f64 = (assign40980_e39185 - var_fn456_calc_ig__vbdgin);
        let assign40980_e39188: f64 = (var_fn456_calc_ig__pbdgin * assign40980_e39187);
        let assign40980_e39190: f64 = (assign40980_e39188 + var_fn456_calc_ig__expphib);
        (assign40980_e39190, var_fn456_calc_ig__expphib_dn4,)
    } else {
        (var_fn456_calc_ig__expbdarg1_vgsat, var_fn456_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__expbdarg1_vgsat = assign40980_e39192;
        var_fn456_calc_ig__expbdarg1_vgsat_dn4 = assign40980_e39192_d_n4;

        let (assign40990_e39241, assign40990_e39241_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) {
        let assign40990_e39207: f64 = (-50.0);
        let (assign40990_e39239, assign40990_e39239_d_n4,) = {
            if ((!(var_fn456_calc_ig__expbdarg1_vgsat > 50.0)) && (!(var_fn456_calc_ig__expbdarg1_vgsat < assign40990_e39207))) {
                let assign40990_e39212: f64 = (var_fn456_calc_ig__expbdarg1_vgsat).exp();
                (assign40990_e39212, (assign40990_e39212 * var_fn456_calc_ig__expbdarg1_vgsat_dn4),)
            } else {
                let assign40990_e39219: f64 = (-50.0);
                let (assign40990_e39238, assign40990_e39238_d_n4,) = {
                    if ((!(var_fn456_calc_ig__expbdarg1_vgsat > 50.0)) && (var_fn456_calc_ig__expbdarg1_vgsat < assign40990_e39219)) {
                        let assign40990_e39223: f64 = (-50.0);
                        let assign40990_e39224: f64 = (assign40990_e39223).exp();
                        (assign40990_e39224, 0.0,)
                    } else {
                        let (assign40990_e39237, assign40990_e39237_d_n4,) = {
                            if (var_fn456_calc_ig__expbdarg1_vgsat > 50.0) {
                                let assign40990_e39229: f64 = (50.0_f64).exp();
                                let assign40990_e39233: f64 = (var_fn456_calc_ig__expbdarg1_vgsat - 50.0);
                                let assign40990_e39234: f64 = (1.0 + assign40990_e39233);
                                let assign40990_e39235: f64 = (assign40990_e39229 * assign40990_e39234);
                                (assign40990_e39235, (assign40990_e39229 * var_fn456_calc_ig__expbdarg1_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign40990_e39237, assign40990_e39237_d_n4,)
                    }
                };
                (assign40990_e39238, assign40990_e39238_d_n4,)
            }
        };
        (assign40990_e39239, assign40990_e39239_d_n4,)
    } else {
        (var_fn456_calc_ig__expbd1_vgsat, var_fn456_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__expbd1_vgsat = assign40990_e39241;
        var_fn456_calc_ig__expbd1_vgsat_dn4 = assign40990_e39241_d_n4;

        let (assign41000_e39254, assign41000_e39254_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) {
        let assign41000_e39252: f64 = (var_fn456_calc_ig__expbd1_vgsat - var_fn456_calc_ig__expbd2);
        (assign41000_e39252, (var_fn456_calc_ig__expbd1_vgsat_dn4 - var_fn456_calc_ig__expbd2_dn4),)
    } else {
        (var_fn456_calc_ig__iginbd_vgsat, var_fn456_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__iginbd_vgsat = assign41000_e39254;
        var_fn456_calc_ig__iginbd_vgsat_dn4 = assign41000_e39254_d_n4;

        let (assign41010_e39271, assign41010_e39271_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) {
        let assign41010_e39265: f64 = (var_fn456_calc_ig__pg_paramin / var_fn456_calc_ig__phitin);
        let assign41010_e39267: f64 = (assign41010_e39265 * var_fn456_calc_ig__vgsatin);
        let assign41010_e39269: f64 = (assign41010_e39267 + var_fn456_calc_ig__expphib);
        (assign41010_e39269, (((-((var_fn456_calc_ig__pg_paramin * var_fn456_calc_ig__phitin_dn4) / (var_fn456_calc_ig__phitin * var_fn456_calc_ig__phitin))) * var_fn456_calc_ig__vgsatin) + var_fn456_calc_ig__expphib_dn4),)
    } else {
        (var_fn456_calc_ig__expiforarg_nohinj_vgsat, var_fn456_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__expiforarg_nohinj_vgsat = assign41010_e39271;
        var_fn456_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign41010_e39271_d_n4;

        let (assign41020_e39320, assign41020_e39320_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) {
        let assign41020_e39286: f64 = (-50.0);
        let (assign41020_e39318, assign41020_e39318_d_n4,) = {
            if ((!(var_fn456_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (!(var_fn456_calc_ig__expiforarg_nohinj_vgsat < assign41020_e39286))) {
                let assign41020_e39291: f64 = (var_fn456_calc_ig__expiforarg_nohinj_vgsat).exp();
                (assign41020_e39291, (assign41020_e39291 * var_fn456_calc_ig__expiforarg_nohinj_vgsat_dn4),)
            } else {
                let assign41020_e39298: f64 = (-50.0);
                let (assign41020_e39317, assign41020_e39317_d_n4,) = {
                    if ((!(var_fn456_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (var_fn456_calc_ig__expiforarg_nohinj_vgsat < assign41020_e39298)) {
                        let assign41020_e39302: f64 = (-50.0);
                        let assign41020_e39303: f64 = (assign41020_e39302).exp();
                        (assign41020_e39303, 0.0,)
                    } else {
                        let (assign41020_e39316, assign41020_e39316_d_n4,) = {
                            if (var_fn456_calc_ig__expiforarg_nohinj_vgsat > 50.0) {
                                let assign41020_e39308: f64 = (50.0_f64).exp();
                                let assign41020_e39312: f64 = (var_fn456_calc_ig__expiforarg_nohinj_vgsat - 50.0);
                                let assign41020_e39313: f64 = (1.0 + assign41020_e39312);
                                let assign41020_e39314: f64 = (assign41020_e39308 * assign41020_e39313);
                                (assign41020_e39314, (assign41020_e39308 * var_fn456_calc_ig__expiforarg_nohinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign41020_e39316, assign41020_e39316_d_n4,)
                    }
                };
                (assign41020_e39317, assign41020_e39317_d_n4,)
            }
        };
        (assign41020_e39318, assign41020_e39318_d_n4,)
    } else {
        (var_fn456_calc_ig__expifor_nohinj_vgsat, var_fn456_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__expifor_nohinj_vgsat = assign41020_e39320;
        var_fn456_calc_ig__expifor_nohinj_vgsat_dn4 = assign41020_e39320_d_n4;

        let (assign41030_e39337, assign41030_e39337_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) {
        let assign41030_e39332: f64 = (var_fn456_calc_ig__kbdgatein * var_fn456_calc_ig__iginbd_vgsat);
        let assign41030_e39333: f64 = (var_fn456_calc_ig__expifor_nohinj_vgsat - assign41030_e39332);
        let assign41030_e39335: f64 = (assign41030_e39333 - var_fn456_calc_ig__t0);
        (assign41030_e39335, ((var_fn456_calc_ig__expifor_nohinj_vgsat_dn4 - (var_fn456_calc_ig__kbdgatein * var_fn456_calc_ig__iginbd_vgsat_dn4)) - var_fn456_calc_ig__t0_dn4),)
    } else {
        (var_fn456_calc_ig__igindiode_nohinj_vgsat, var_fn456_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__igindiode_nohinj_vgsat = assign41030_e39337;
        var_fn456_calc_ig__igindiode_nohinj_vgsat_dn4 = assign41030_e39337_d_n4;

        let (assign41040_e39356, assign41040_e39356_d_n4, assign41040_e39356_d_n5, assign41040_e39356_d_n8,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) {
        let assign41040_e39350: f64 = (var_fn456_calc_ig__kbdgatein * var_fn456_calc_ig__iginbd);
        let assign41040_e39351: f64 = (var_fn456_calc_ig__expifor - assign41040_e39350);
        let assign41040_e39353: f64 = (assign41040_e39351 - var_fn456_calc_ig__t0);
        let assign41040_e39354: f64 = (var_fn456_calc_ig__isdiodeout * assign41040_e39353);
        (assign41040_e39354, ((var_fn456_calc_ig__isdiodeout_dn4 * assign41040_e39353) + (var_fn456_calc_ig__isdiodeout * ((var_fn456_calc_ig__expifor_dn4 - (var_fn456_calc_ig__kbdgatein * var_fn456_calc_ig__iginbd_dn4)) - var_fn456_calc_ig__t0_dn4))), (var_fn456_calc_ig__isdiodeout * (var_fn456_calc_ig__expifor_dn5 - (var_fn456_calc_ig__kbdgatein * var_fn456_calc_ig__iginbd_dn5))), (var_fn456_calc_ig__isdiodeout * (var_fn456_calc_ig__expifor_dn8 - (var_fn456_calc_ig__kbdgatein * var_fn456_calc_ig__iginbd_dn8))),)
    } else {
        (var_fn456_calc_ig__igindiode_nohinj, var_fn456_calc_ig__igindiode_nohinj_dn4, var_fn456_calc_ig__igindiode_nohinj_dn5, var_fn456_calc_ig__igindiode_nohinj_dn8,)
    }
};
        var_fn456_calc_ig__igindiode_nohinj = assign41040_e39356;
        var_fn456_calc_ig__igindiode_nohinj_dn4 = assign41040_e39356_d_n4;
        var_fn456_calc_ig__igindiode_nohinj_dn5 = assign41040_e39356_d_n5;
        var_fn456_calc_ig__igindiode_nohinj_dn8 = assign41040_e39356_d_n8;

        let assign41050_e39359: f64 = if var_fn456_calc_ig__fracin > 0.0 { 1.0 } else { 0.0 };
        var_guard458 = assign41050_e39359;

        let (assign41060_e39374,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) && (var_guard458 != 0.0)) {
        let assign41060_e39372: f64 = (var_fn456_calc_ig__fracin * var_fn456_calc_ig__pg_paramin);
        (assign41060_e39372,)
    } else {
        (var_fn456_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn456_calc_ig__pg_paramin_hinj = assign41060_e39374;

        let (assign41070_e39393, assign41070_e39393_d_n4,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) && (var_guard458 != 0.0)) {
        let assign41070_e39387: f64 = (var_fn456_calc_ig__pg_paramin_hinj / var_fn456_calc_ig__phitin);
        let assign41070_e39389: f64 = (assign41070_e39387 * var_fn456_calc_ig__vgsatin);
        let assign41070_e39391: f64 = (assign41070_e39389 + var_fn456_calc_ig__expphib);
        (assign41070_e39391, (((-((var_fn456_calc_ig__pg_paramin_hinj * var_fn456_calc_ig__phitin_dn4) / (var_fn456_calc_ig__phitin * var_fn456_calc_ig__phitin))) * var_fn456_calc_ig__vgsatin) + var_fn456_calc_ig__expphib_dn4),)
    } else {
        (var_fn456_calc_ig__expiforarg_hinj_vgsat, var_fn456_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__expiforarg_hinj_vgsat = assign41070_e39393;
        var_fn456_calc_ig__expiforarg_hinj_vgsat_dn4 = assign41070_e39393_d_n4;

        let (assign41080_e39444, assign41080_e39444_d_n4,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) && (var_guard458 != 0.0)) {
        let assign41080_e39410: f64 = (-50.0);
        let (assign41080_e39442, assign41080_e39442_d_n4,) = {
            if ((!(var_fn456_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (!(var_fn456_calc_ig__expiforarg_hinj_vgsat < assign41080_e39410))) {
                let assign41080_e39415: f64 = (var_fn456_calc_ig__expiforarg_hinj_vgsat).exp();
                (assign41080_e39415, (assign41080_e39415 * var_fn456_calc_ig__expiforarg_hinj_vgsat_dn4),)
            } else {
                let assign41080_e39422: f64 = (-50.0);
                let (assign41080_e39441, assign41080_e39441_d_n4,) = {
                    if ((!(var_fn456_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (var_fn456_calc_ig__expiforarg_hinj_vgsat < assign41080_e39422)) {
                        let assign41080_e39426: f64 = (-50.0);
                        let assign41080_e39427: f64 = (assign41080_e39426).exp();
                        (assign41080_e39427, 0.0,)
                    } else {
                        let (assign41080_e39440, assign41080_e39440_d_n4,) = {
                            if (var_fn456_calc_ig__expiforarg_hinj_vgsat > 50.0) {
                                let assign41080_e39432: f64 = (50.0_f64).exp();
                                let assign41080_e39436: f64 = (var_fn456_calc_ig__expiforarg_hinj_vgsat - 50.0);
                                let assign41080_e39437: f64 = (1.0 + assign41080_e39436);
                                let assign41080_e39438: f64 = (assign41080_e39432 * assign41080_e39437);
                                (assign41080_e39438, (assign41080_e39432 * var_fn456_calc_ig__expiforarg_hinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign41080_e39440, assign41080_e39440_d_n4,)
                    }
                };
                (assign41080_e39441, assign41080_e39441_d_n4,)
            }
        };
        (assign41080_e39442, assign41080_e39442_d_n4,)
    } else {
        (var_fn456_calc_ig__expifor_hinj_vgsat, var_fn456_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__expifor_hinj_vgsat = assign41080_e39444;
        var_fn456_calc_ig__expifor_hinj_vgsat_dn4 = assign41080_e39444_d_n4;

        let (assign41090_e39463, assign41090_e39463_d_n4,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) && (var_guard458 != 0.0)) {
        let assign41090_e39458: f64 = (var_fn456_calc_ig__kbdgatein * var_fn456_calc_ig__iginbd_vgsat);
        let assign41090_e39459: f64 = (var_fn456_calc_ig__expifor_hinj_vgsat - assign41090_e39458);
        let assign41090_e39461: f64 = (assign41090_e39459 - var_fn456_calc_ig__t0);
        (assign41090_e39461, ((var_fn456_calc_ig__expifor_hinj_vgsat_dn4 - (var_fn456_calc_ig__kbdgatein * var_fn456_calc_ig__iginbd_vgsat_dn4)) - var_fn456_calc_ig__t0_dn4),)
    } else {
        (var_fn456_calc_ig__igindiode_hinj_vgsat, var_fn456_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn456_calc_ig__igindiode_hinj_vgsat = assign41090_e39463;
        var_fn456_calc_ig__igindiode_hinj_vgsat_dn4 = assign41090_e39463_d_n4;

        let (assign41100_e39482, assign41100_e39482_d_n4, assign41100_e39482_d_n5, assign41100_e39482_d_n8,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) && (var_guard458 != 0.0)) {
        let assign41100_e39476: f64 = (var_fn456_calc_ig__pg_paramin_hinj / var_fn456_calc_ig__phitin);
        let assign41100_e39478: f64 = (assign41100_e39476 * var_fn456_calc_ig__vgin);
        let assign41100_e39480: f64 = (assign41100_e39478 + var_fn456_calc_ig__expphib);
        (assign41100_e39480, (((-((var_fn456_calc_ig__pg_paramin_hinj * var_fn456_calc_ig__phitin_dn4) / (var_fn456_calc_ig__phitin * var_fn456_calc_ig__phitin))) * var_fn456_calc_ig__vgin) + var_fn456_calc_ig__expphib_dn4), (assign41100_e39476 * var_fn456_calc_ig__vgin_dn5), (assign41100_e39476 * var_fn456_calc_ig__vgin_dn8),)
    } else {
        (var_fn456_calc_ig__expiforarg_hinj, var_fn456_calc_ig__expiforarg_hinj_dn4, var_fn456_calc_ig__expiforarg_hinj_dn5, var_fn456_calc_ig__expiforarg_hinj_dn8,)
    }
};
        var_fn456_calc_ig__expiforarg_hinj = assign41100_e39482;
        var_fn456_calc_ig__expiforarg_hinj_dn4 = assign41100_e39482_d_n4;
        var_fn456_calc_ig__expiforarg_hinj_dn5 = assign41100_e39482_d_n5;
        var_fn456_calc_ig__expiforarg_hinj_dn8 = assign41100_e39482_d_n8;

        let (assign41110_e39533, assign41110_e39533_d_n4, assign41110_e39533_d_n5, assign41110_e39533_d_n8,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) && (var_guard458 != 0.0)) {
        let assign41110_e39499: f64 = (-50.0);
        let (assign41110_e39531, assign41110_e39531_d_n4, assign41110_e39531_d_n5, assign41110_e39531_d_n8,) = {
            if ((!(var_fn456_calc_ig__expiforarg_hinj > 50.0)) && (!(var_fn456_calc_ig__expiforarg_hinj < assign41110_e39499))) {
                let assign41110_e39504: f64 = (var_fn456_calc_ig__expiforarg_hinj).exp();
                (assign41110_e39504, (assign41110_e39504 * var_fn456_calc_ig__expiforarg_hinj_dn4), (assign41110_e39504 * var_fn456_calc_ig__expiforarg_hinj_dn5), (assign41110_e39504 * var_fn456_calc_ig__expiforarg_hinj_dn8),)
            } else {
                let assign41110_e39511: f64 = (-50.0);
                let (assign41110_e39530, assign41110_e39530_d_n4, assign41110_e39530_d_n5, assign41110_e39530_d_n8,) = {
                    if ((!(var_fn456_calc_ig__expiforarg_hinj > 50.0)) && (var_fn456_calc_ig__expiforarg_hinj < assign41110_e39511)) {
                        let assign41110_e39515: f64 = (-50.0);
                        let assign41110_e39516: f64 = (assign41110_e39515).exp();
                        (assign41110_e39516, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign41110_e39529, assign41110_e39529_d_n4, assign41110_e39529_d_n5, assign41110_e39529_d_n8,) = {
                            if (var_fn456_calc_ig__expiforarg_hinj > 50.0) {
                                let assign41110_e39521: f64 = (50.0_f64).exp();
                                let assign41110_e39525: f64 = (var_fn456_calc_ig__expiforarg_hinj - 50.0);
                                let assign41110_e39526: f64 = (1.0 + assign41110_e39525);
                                let assign41110_e39527: f64 = (assign41110_e39521 * assign41110_e39526);
                                (assign41110_e39527, (assign41110_e39521 * var_fn456_calc_ig__expiforarg_hinj_dn4), (assign41110_e39521 * var_fn456_calc_ig__expiforarg_hinj_dn5), (assign41110_e39521 * var_fn456_calc_ig__expiforarg_hinj_dn8),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign41110_e39529, assign41110_e39529_d_n4, assign41110_e39529_d_n5, assign41110_e39529_d_n8,)
                    }
                };
                (assign41110_e39530, assign41110_e39530_d_n4, assign41110_e39530_d_n5, assign41110_e39530_d_n8,)
            }
        };
        (assign41110_e39531, assign41110_e39531_d_n4, assign41110_e39531_d_n5, assign41110_e39531_d_n8,)
    } else {
        (var_fn456_calc_ig__expifor_hinj, var_fn456_calc_ig__expifor_hinj_dn4, var_fn456_calc_ig__expifor_hinj_dn5, var_fn456_calc_ig__expifor_hinj_dn8,)
    }
};
        var_fn456_calc_ig__expifor_hinj = assign41110_e39533;
        var_fn456_calc_ig__expifor_hinj_dn4 = assign41110_e39533_d_n4;
        var_fn456_calc_ig__expifor_hinj_dn5 = assign41110_e39533_d_n5;
        var_fn456_calc_ig__expifor_hinj_dn8 = assign41110_e39533_d_n8;

        let (assign41120_e39550, assign41120_e39550_d_n4,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) && (var_guard458 != 0.0)) {
        let assign41120_e39546: f64 = (var_fn456_calc_ig__isdiodeout * var_fn456_calc_ig__igindiode_nohinj_vgsat);
        let assign41120_e39548: f64 = (assign41120_e39546 / var_fn456_calc_ig__igindiode_hinj_vgsat);
        (assign41120_e39548, (((((var_fn456_calc_ig__isdiodeout_dn4 * var_fn456_calc_ig__igindiode_nohinj_vgsat) + (var_fn456_calc_ig__isdiodeout * var_fn456_calc_ig__igindiode_nohinj_vgsat_dn4)) * var_fn456_calc_ig__igindiode_hinj_vgsat) - (assign41120_e39546 * var_fn456_calc_ig__igindiode_hinj_vgsat_dn4)) / (var_fn456_calc_ig__igindiode_hinj_vgsat * var_fn456_calc_ig__igindiode_hinj_vgsat)),)
    } else {
        (var_fn456_calc_ig__igindiode_hinj_pre, var_fn456_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn456_calc_ig__igindiode_hinj_pre = assign41120_e39550;
        var_fn456_calc_ig__igindiode_hinj_pre_dn4 = assign41120_e39550_d_n4;

        let (assign41130_e39571, assign41130_e39571_d_n4, assign41130_e39571_d_n5, assign41130_e39571_d_n8,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) && (var_guard458 != 0.0)) {
        let assign41130_e39565: f64 = (var_fn456_calc_ig__kbdgatein * var_fn456_calc_ig__iginbd);
        let assign41130_e39566: f64 = (var_fn456_calc_ig__expifor_hinj - assign41130_e39565);
        let assign41130_e39568: f64 = (assign41130_e39566 - var_fn456_calc_ig__t0);
        let assign41130_e39569: f64 = (var_fn456_calc_ig__igindiode_hinj_pre * assign41130_e39568);
        (assign41130_e39569, ((var_fn456_calc_ig__igindiode_hinj_pre_dn4 * assign41130_e39568) + (var_fn456_calc_ig__igindiode_hinj_pre * ((var_fn456_calc_ig__expifor_hinj_dn4 - (var_fn456_calc_ig__kbdgatein * var_fn456_calc_ig__iginbd_dn4)) - var_fn456_calc_ig__t0_dn4))), (var_fn456_calc_ig__igindiode_hinj_pre * (var_fn456_calc_ig__expifor_hinj_dn5 - (var_fn456_calc_ig__kbdgatein * var_fn456_calc_ig__iginbd_dn5))), (var_fn456_calc_ig__igindiode_hinj_pre * (var_fn456_calc_ig__expifor_hinj_dn8 - (var_fn456_calc_ig__kbdgatein * var_fn456_calc_ig__iginbd_dn8))),)
    } else {
        (var_fn456_calc_ig__igindiode_hinj, var_fn456_calc_ig__igindiode_hinj_dn4, var_fn456_calc_ig__igindiode_hinj_dn5, var_fn456_calc_ig__igindiode_hinj_dn8,)
    }
};
        var_fn456_calc_ig__igindiode_hinj = assign41130_e39571;
        var_fn456_calc_ig__igindiode_hinj_dn4 = assign41130_e39571_d_n4;
        var_fn456_calc_ig__igindiode_hinj_dn5 = assign41130_e39571_d_n5;
        var_fn456_calc_ig__igindiode_hinj_dn8 = assign41130_e39571_d_n8;

        let (assign41140_e39587, assign41140_e39587_d_n4, assign41140_e39587_d_n5, assign41140_e39587_d_n8,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) && (var_guard458 == 0.0)) {
        let assign41140_e39585: f64 = (var_fn456_calc_ig__isdiodeout * var_fn456_calc_ig__igindiode_nohinj_vgsat);
        (assign41140_e39585, ((var_fn456_calc_ig__isdiodeout_dn4 * var_fn456_calc_ig__igindiode_nohinj_vgsat) + (var_fn456_calc_ig__isdiodeout * var_fn456_calc_ig__igindiode_nohinj_vgsat_dn4)), 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__igindiode_hinj, var_fn456_calc_ig__igindiode_hinj_dn4, var_fn456_calc_ig__igindiode_hinj_dn5, var_fn456_calc_ig__igindiode_hinj_dn8,)
    }
};
        var_fn456_calc_ig__igindiode_hinj = assign41140_e39587;
        var_fn456_calc_ig__igindiode_hinj_dn4 = assign41140_e39587_d_n4;
        var_fn456_calc_ig__igindiode_hinj_dn5 = assign41140_e39587_d_n5;
        var_fn456_calc_ig__igindiode_hinj_dn8 = assign41140_e39587_d_n8;

        *var_fn456_calc_ig__expbd1_slot = var_fn456_calc_ig__expbd1;
        *var_fn456_calc_ig__expbd1_dn4_slot = var_fn456_calc_ig__expbd1_dn4;
        *var_fn456_calc_ig__expbd1_dn5_slot = var_fn456_calc_ig__expbd1_dn5;
        *var_fn456_calc_ig__expbd1_dn8_slot = var_fn456_calc_ig__expbd1_dn8;
        *var_fn456_calc_ig__expbd1_vgsat_slot = var_fn456_calc_ig__expbd1_vgsat;
        *var_fn456_calc_ig__expbd1_vgsat_dn4_slot = var_fn456_calc_ig__expbd1_vgsat_dn4;
        *var_fn456_calc_ig__expbd2_slot = var_fn456_calc_ig__expbd2;
        *var_fn456_calc_ig__expbd2_dn4_slot = var_fn456_calc_ig__expbd2_dn4;
        *var_fn456_calc_ig__expbdarg1_vgsat_slot = var_fn456_calc_ig__expbdarg1_vgsat;
        *var_fn456_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn456_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn456_calc_ig__expifor_slot = var_fn456_calc_ig__expifor;
        *var_fn456_calc_ig__expifor_dn4_slot = var_fn456_calc_ig__expifor_dn4;
        *var_fn456_calc_ig__expifor_dn5_slot = var_fn456_calc_ig__expifor_dn5;
        *var_fn456_calc_ig__expifor_dn8_slot = var_fn456_calc_ig__expifor_dn8;
        *var_fn456_calc_ig__expifor_hinj_slot = var_fn456_calc_ig__expifor_hinj;
        *var_fn456_calc_ig__expifor_hinj_dn4_slot = var_fn456_calc_ig__expifor_hinj_dn4;
        *var_fn456_calc_ig__expifor_hinj_dn5_slot = var_fn456_calc_ig__expifor_hinj_dn5;
        *var_fn456_calc_ig__expifor_hinj_dn8_slot = var_fn456_calc_ig__expifor_hinj_dn8;
        *var_fn456_calc_ig__expifor_hinj_vgsat_slot = var_fn456_calc_ig__expifor_hinj_vgsat;
        *var_fn456_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn456_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn456_calc_ig__expifor_nohinj_vgsat_slot = var_fn456_calc_ig__expifor_nohinj_vgsat;
        *var_fn456_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn456_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn456_calc_ig__expiforarg_slot = var_fn456_calc_ig__expiforarg;
        *var_fn456_calc_ig__expiforarg_dn4_slot = var_fn456_calc_ig__expiforarg_dn4;
        *var_fn456_calc_ig__expiforarg_dn5_slot = var_fn456_calc_ig__expiforarg_dn5;
        *var_fn456_calc_ig__expiforarg_dn8_slot = var_fn456_calc_ig__expiforarg_dn8;
        *var_fn456_calc_ig__expiforarg_hinj_slot = var_fn456_calc_ig__expiforarg_hinj;
        *var_fn456_calc_ig__expiforarg_hinj_dn4_slot = var_fn456_calc_ig__expiforarg_hinj_dn4;
        *var_fn456_calc_ig__expiforarg_hinj_dn5_slot = var_fn456_calc_ig__expiforarg_hinj_dn5;
        *var_fn456_calc_ig__expiforarg_hinj_dn8_slot = var_fn456_calc_ig__expiforarg_hinj_dn8;
        *var_fn456_calc_ig__expiforarg_hinj_vgsat_slot = var_fn456_calc_ig__expiforarg_hinj_vgsat;
        *var_fn456_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn456_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn456_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn456_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn456_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn456_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn456_calc_ig__iginbd_slot = var_fn456_calc_ig__iginbd;
        *var_fn456_calc_ig__iginbd_dn4_slot = var_fn456_calc_ig__iginbd_dn4;
        *var_fn456_calc_ig__iginbd_dn5_slot = var_fn456_calc_ig__iginbd_dn5;
        *var_fn456_calc_ig__iginbd_dn8_slot = var_fn456_calc_ig__iginbd_dn8;
        *var_fn456_calc_ig__iginbd_vgsat_slot = var_fn456_calc_ig__iginbd_vgsat;
        *var_fn456_calc_ig__iginbd_vgsat_dn4_slot = var_fn456_calc_ig__iginbd_vgsat_dn4;
        *var_fn456_calc_ig__igindiode_slot = var_fn456_calc_ig__igindiode;
        *var_fn456_calc_ig__igindiode_dn4_slot = var_fn456_calc_ig__igindiode_dn4;
        *var_fn456_calc_ig__igindiode_dn5_slot = var_fn456_calc_ig__igindiode_dn5;
        *var_fn456_calc_ig__igindiode_dn8_slot = var_fn456_calc_ig__igindiode_dn8;
        *var_fn456_calc_ig__igindiode_hinj_slot = var_fn456_calc_ig__igindiode_hinj;
        *var_fn456_calc_ig__igindiode_hinj_dn4_slot = var_fn456_calc_ig__igindiode_hinj_dn4;
        *var_fn456_calc_ig__igindiode_hinj_dn5_slot = var_fn456_calc_ig__igindiode_hinj_dn5;
        *var_fn456_calc_ig__igindiode_hinj_dn8_slot = var_fn456_calc_ig__igindiode_hinj_dn8;
        *var_fn456_calc_ig__igindiode_hinj_pre_slot = var_fn456_calc_ig__igindiode_hinj_pre;
        *var_fn456_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn456_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn456_calc_ig__igindiode_hinj_vgsat_slot = var_fn456_calc_ig__igindiode_hinj_vgsat;
        *var_fn456_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn456_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn456_calc_ig__igindiode_nohinj_slot = var_fn456_calc_ig__igindiode_nohinj;
        *var_fn456_calc_ig__igindiode_nohinj_dn4_slot = var_fn456_calc_ig__igindiode_nohinj_dn4;
        *var_fn456_calc_ig__igindiode_nohinj_dn5_slot = var_fn456_calc_ig__igindiode_nohinj_dn5;
        *var_fn456_calc_ig__igindiode_nohinj_dn8_slot = var_fn456_calc_ig__igindiode_nohinj_dn8;
        *var_fn456_calc_ig__igindiode_nohinj_vgsat_slot = var_fn456_calc_ig__igindiode_nohinj_vgsat;
        *var_fn456_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn456_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn456_calc_ig__isdiodeout_slot = var_fn456_calc_ig__isdiodeout;
        *var_fn456_calc_ig__isdiodeout_dn4_slot = var_fn456_calc_ig__isdiodeout_dn4;
        *var_fn456_calc_ig__pg_paramin_hinj_slot = var_fn456_calc_ig__pg_paramin_hinj;
        *var_guard457_slot = var_guard457;
        *var_guard458_slot = var_guard458;
    }

    pub(super) fn stamp_transient_block_103(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_fn456_calc_ig__alphagin: f64,
        var_fn456_calc_ig__betarecin: f64,
        var_fn456_calc_ig__igindiode_hinj: f64,
        var_fn456_calc_ig__igindiode_hinj_dn4: f64,
        var_fn456_calc_ig__igindiode_hinj_dn5: f64,
        var_fn456_calc_ig__igindiode_hinj_dn8: f64,
        var_fn456_calc_ig__igindiode_nohinj: f64,
        var_fn456_calc_ig__igindiode_nohinj_dn4: f64,
        var_fn456_calc_ig__igindiode_nohinj_dn5: f64,
        var_fn456_calc_ig__igindiode_nohinj_dn8: f64,
        var_fn456_calc_ig__irecin: f64,
        var_fn456_calc_ig__ngf: f64,
        var_fn456_calc_ig__pgsrecin: f64,
        var_fn456_calc_ig__phitin: f64,
        var_fn456_calc_ig__phitin_dn4: f64,
        var_fn456_calc_ig__tfacdiodein: f64,
        var_fn456_calc_ig__tfacdiodein_dn4: f64,
        var_fn456_calc_ig__type: f64,
        var_fn456_calc_ig__vgin: f64,
        var_fn456_calc_ig__vgin_dn5: f64,
        var_fn456_calc_ig__vgin_dn8: f64,
        var_fn456_calc_ig__vgsatin: f64,
        var_fn456_calc_ig__vgsatqin: f64,
        var_fn456_calc_ig__w: f64,
        var_guard417: f64,
        var_guard439: f64,
        var_guard450: f64,
        var_guard457: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_tfacdiode: f64,
        var_tfacdiode_dn4: f64,
        var_fn456_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn456_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn456_calc_ig__expffvarg_slot: &mut f64,
        var_fn456_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn456_calc_ig__expffvarg_dn5_slot: &mut f64,
        var_fn456_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn456_calc_ig__expirev_slot: &mut f64,
        var_fn456_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn456_calc_ig__expirev_dn5_slot: &mut f64,
        var_fn456_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn456_calc_ig__expirevarg_slot: &mut f64,
        var_fn456_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn456_calc_ig__expirevarg_dn5_slot: &mut f64,
        var_fn456_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn456_calc_ig__ffvgin_slot: &mut f64,
        var_fn456_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn456_calc_ig__ffvgin_dn5_slot: &mut f64,
        var_fn456_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn456_calc_ig__frecgin_slot: &mut f64,
        var_fn456_calc_ig__frecgin_dn5_slot: &mut f64,
        var_fn456_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn456_calc_ig__igindiode_slot: &mut f64,
        var_fn456_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn456_calc_ig__igindiode_dn5_slot: &mut f64,
        var_fn456_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn456_calc_ig__iginrec_slot: &mut f64,
        var_fn456_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn456_calc_ig__iginrec_dn5_slot: &mut f64,
        var_fn456_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn456_calc_ig__igout_slot: &mut f64,
        var_fn456_calc_ig__igout_dn4_slot: &mut f64,
        var_fn456_calc_ig__igout_dn5_slot: &mut f64,
        var_fn456_calc_ig__igout_dn8_slot: &mut f64,
        var_fn456_calc_ig__isrecout_slot: &mut f64,
        var_fn456_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn456_calc_ig__return_slot: &mut f64,
        var_fn456_calc_ig__return_dn4_slot: &mut f64,
        var_fn456_calc_ig__return_dn5_slot: &mut f64,
        var_fn456_calc_ig__return_dn8_slot: &mut f64,
        var_fn462_calc_ig__alphagin_slot: &mut f64,
        var_fn462_calc_ig__betarecin_slot: &mut f64,
        var_fn462_calc_ig__fracin_slot: &mut f64,
        var_fn462_calc_ig__ijin_slot: &mut f64,
        var_fn462_calc_ig__irecin_slot: &mut f64,
        var_fn462_calc_ig__isdiodeout_slot: &mut f64,
        var_fn462_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn462_calc_ig__isrecout_slot: &mut f64,
        var_fn462_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn462_calc_ig__kbdgatein_slot: &mut f64,
        var_fn462_calc_ig__ngf_slot: &mut f64,
        var_fn462_calc_ig__pbdgin_slot: &mut f64,
        var_fn462_calc_ig__pg_paramin_slot: &mut f64,
        var_fn462_calc_ig__phitin_slot: &mut f64,
        var_fn462_calc_ig__phitin_dn4_slot: &mut f64,
        var_fn462_calc_ig__return_slot: &mut f64,
        var_fn462_calc_ig__return_dn4_slot: &mut f64,
        var_fn462_calc_ig__return_dn7_slot: &mut f64,
        var_fn462_calc_ig__return_dn8_slot: &mut f64,
        var_fn462_calc_ig__tfacdiodein_slot: &mut f64,
        var_fn462_calc_ig__tfacdiodein_dn4_slot: &mut f64,
        var_fn462_calc_ig__vbdgin_slot: &mut f64,
        var_fn462_calc_ig__vgin_slot: &mut f64,
        var_fn462_calc_ig__vgin_dn7_slot: &mut f64,
        var_fn462_calc_ig__vgin_dn8_slot: &mut f64,
        var_fn462_calc_ig__vgsatin_slot: &mut f64,
        var_fn462_calc_ig__vgsatqin_slot: &mut f64,
        var_fn462_calc_ig__w_slot: &mut f64,
        var_guard459_slot: &mut f64,
        var_guard460_slot: &mut f64,
        var_guard461_slot: &mut f64,
        var_idsch_slot: &mut f64,
        var_idsch2_slot: &mut f64,
        var_idsch2_dn4_slot: &mut f64,
        var_idsch2_dn7_slot: &mut f64,
        var_idsch2_dn8_slot: &mut f64,
        var_idsch_dn4_slot: &mut f64,
        var_idsch_dn7_slot: &mut f64,
        var_idsch_dn8_slot: &mut f64,
        var_igdi2db_slot: &mut f64,
        var_igdi2db_dn4_slot: &mut f64,
        var_igdi2db_dn5_slot: &mut f64,
        var_igdi2db_dn8_slot: &mut f64,
        var_qsch_slot: &mut f64,
        var_qsch0_slot: &mut f64,
        var_qsch1_slot: &mut f64,
        var_qsch1_dn7_slot: &mut f64,
        var_qsch1_dn8_slot: &mut f64,
        var_qsch2_slot: &mut f64,
        var_qsch2_dn7_slot: &mut f64,
        var_qsch2_dn8_slot: &mut f64,
        var_qsch3_slot: &mut f64,
        var_qsch3_dn7_slot: &mut f64,
        var_qsch3_dn8_slot: &mut f64,
        var_qsch4_slot: &mut f64,
        var_qsch4_dn7_slot: &mut f64,
        var_qsch4_dn8_slot: &mut f64,
        var_qsch5_slot: &mut f64,
        var_qsch5_dn7_slot: &mut f64,
        var_qsch5_dn8_slot: &mut f64,
        var_qsch_dn7_slot: &mut f64,
        var_qsch_dn8_slot: &mut f64,
        var_vsch_slot: &mut f64,
        var_vsch_dn7_slot: &mut f64,
        var_vsch_dn8_slot: &mut f64,
        var_vschfc1_slot: &mut f64,
        var_vschfc1_dn7_slot: &mut f64,
        var_vschfc1_dn8_slot: &mut f64,
        var_vschfc2_slot: &mut f64,
        var_vschfc2_dn7_slot: &mut f64,
        var_vschfc2_dn8_slot: &mut f64,
        var_vschfc3_slot: &mut f64,
        var_vschfc3_dn7_slot: &mut f64,
        var_vschfc3_dn8_slot: &mut f64,
        var_vschfc4_slot: &mut f64,
        var_vschfc4_dn7_slot: &mut f64,
        var_vschfc4_dn8_slot: &mut f64,
        var_vschfc5_slot: &mut f64,
        var_vschfc5_dn7_slot: &mut f64,
        var_vschfc5_dn8_slot: &mut f64,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let mut var_fn456_calc_ig__alpha2_phit: f64 = *var_fn456_calc_ig__alpha2_phit_slot;
        let mut var_fn456_calc_ig__alpha2_phit_dn4: f64 = *var_fn456_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn456_calc_ig__expffvarg: f64 = *var_fn456_calc_ig__expffvarg_slot;
        let mut var_fn456_calc_ig__expffvarg_dn4: f64 = *var_fn456_calc_ig__expffvarg_dn4_slot;
        let mut var_fn456_calc_ig__expffvarg_dn5: f64 = *var_fn456_calc_ig__expffvarg_dn5_slot;
        let mut var_fn456_calc_ig__expffvarg_dn8: f64 = *var_fn456_calc_ig__expffvarg_dn8_slot;
        let mut var_fn456_calc_ig__expirev: f64 = *var_fn456_calc_ig__expirev_slot;
        let mut var_fn456_calc_ig__expirev_dn4: f64 = *var_fn456_calc_ig__expirev_dn4_slot;
        let mut var_fn456_calc_ig__expirev_dn5: f64 = *var_fn456_calc_ig__expirev_dn5_slot;
        let mut var_fn456_calc_ig__expirev_dn8: f64 = *var_fn456_calc_ig__expirev_dn8_slot;
        let mut var_fn456_calc_ig__expirevarg: f64 = *var_fn456_calc_ig__expirevarg_slot;
        let mut var_fn456_calc_ig__expirevarg_dn4: f64 = *var_fn456_calc_ig__expirevarg_dn4_slot;
        let mut var_fn456_calc_ig__expirevarg_dn5: f64 = *var_fn456_calc_ig__expirevarg_dn5_slot;
        let mut var_fn456_calc_ig__expirevarg_dn8: f64 = *var_fn456_calc_ig__expirevarg_dn8_slot;
        let mut var_fn456_calc_ig__ffvgin: f64 = *var_fn456_calc_ig__ffvgin_slot;
        let mut var_fn456_calc_ig__ffvgin_dn4: f64 = *var_fn456_calc_ig__ffvgin_dn4_slot;
        let mut var_fn456_calc_ig__ffvgin_dn5: f64 = *var_fn456_calc_ig__ffvgin_dn5_slot;
        let mut var_fn456_calc_ig__ffvgin_dn8: f64 = *var_fn456_calc_ig__ffvgin_dn8_slot;
        let mut var_fn456_calc_ig__frecgin: f64 = *var_fn456_calc_ig__frecgin_slot;
        let mut var_fn456_calc_ig__frecgin_dn5: f64 = *var_fn456_calc_ig__frecgin_dn5_slot;
        let mut var_fn456_calc_ig__frecgin_dn8: f64 = *var_fn456_calc_ig__frecgin_dn8_slot;
        let mut var_fn456_calc_ig__igindiode: f64 = *var_fn456_calc_ig__igindiode_slot;
        let mut var_fn456_calc_ig__igindiode_dn4: f64 = *var_fn456_calc_ig__igindiode_dn4_slot;
        let mut var_fn456_calc_ig__igindiode_dn5: f64 = *var_fn456_calc_ig__igindiode_dn5_slot;
        let mut var_fn456_calc_ig__igindiode_dn8: f64 = *var_fn456_calc_ig__igindiode_dn8_slot;
        let mut var_fn456_calc_ig__iginrec: f64 = *var_fn456_calc_ig__iginrec_slot;
        let mut var_fn456_calc_ig__iginrec_dn4: f64 = *var_fn456_calc_ig__iginrec_dn4_slot;
        let mut var_fn456_calc_ig__iginrec_dn5: f64 = *var_fn456_calc_ig__iginrec_dn5_slot;
        let mut var_fn456_calc_ig__iginrec_dn8: f64 = *var_fn456_calc_ig__iginrec_dn8_slot;
        let mut var_fn456_calc_ig__igout: f64 = *var_fn456_calc_ig__igout_slot;
        let mut var_fn456_calc_ig__igout_dn4: f64 = *var_fn456_calc_ig__igout_dn4_slot;
        let mut var_fn456_calc_ig__igout_dn5: f64 = *var_fn456_calc_ig__igout_dn5_slot;
        let mut var_fn456_calc_ig__igout_dn8: f64 = *var_fn456_calc_ig__igout_dn8_slot;
        let mut var_fn456_calc_ig__isrecout: f64 = *var_fn456_calc_ig__isrecout_slot;
        let mut var_fn456_calc_ig__isrecout_dn4: f64 = *var_fn456_calc_ig__isrecout_dn4_slot;
        let mut var_fn456_calc_ig__return: f64 = *var_fn456_calc_ig__return_slot;
        let mut var_fn456_calc_ig__return_dn4: f64 = *var_fn456_calc_ig__return_dn4_slot;
        let mut var_fn456_calc_ig__return_dn5: f64 = *var_fn456_calc_ig__return_dn5_slot;
        let mut var_fn456_calc_ig__return_dn8: f64 = *var_fn456_calc_ig__return_dn8_slot;
        let mut var_fn462_calc_ig__alphagin: f64 = *var_fn462_calc_ig__alphagin_slot;
        let mut var_fn462_calc_ig__betarecin: f64 = *var_fn462_calc_ig__betarecin_slot;
        let mut var_fn462_calc_ig__fracin: f64 = *var_fn462_calc_ig__fracin_slot;
        let mut var_fn462_calc_ig__ijin: f64 = *var_fn462_calc_ig__ijin_slot;
        let mut var_fn462_calc_ig__irecin: f64 = *var_fn462_calc_ig__irecin_slot;
        let mut var_fn462_calc_ig__isdiodeout: f64 = *var_fn462_calc_ig__isdiodeout_slot;
        let mut var_fn462_calc_ig__isdiodeout_dn4: f64 = *var_fn462_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn462_calc_ig__isrecout: f64 = *var_fn462_calc_ig__isrecout_slot;
        let mut var_fn462_calc_ig__isrecout_dn4: f64 = *var_fn462_calc_ig__isrecout_dn4_slot;
        let mut var_fn462_calc_ig__kbdgatein: f64 = *var_fn462_calc_ig__kbdgatein_slot;
        let mut var_fn462_calc_ig__ngf: f64 = *var_fn462_calc_ig__ngf_slot;
        let mut var_fn462_calc_ig__pbdgin: f64 = *var_fn462_calc_ig__pbdgin_slot;
        let mut var_fn462_calc_ig__pg_paramin: f64 = *var_fn462_calc_ig__pg_paramin_slot;
        let mut var_fn462_calc_ig__phitin: f64 = *var_fn462_calc_ig__phitin_slot;
        let mut var_fn462_calc_ig__phitin_dn4: f64 = *var_fn462_calc_ig__phitin_dn4_slot;
        let mut var_fn462_calc_ig__return: f64 = *var_fn462_calc_ig__return_slot;
        let mut var_fn462_calc_ig__return_dn4: f64 = *var_fn462_calc_ig__return_dn4_slot;
        let mut var_fn462_calc_ig__return_dn7: f64 = *var_fn462_calc_ig__return_dn7_slot;
        let mut var_fn462_calc_ig__return_dn8: f64 = *var_fn462_calc_ig__return_dn8_slot;
        let mut var_fn462_calc_ig__tfacdiodein: f64 = *var_fn462_calc_ig__tfacdiodein_slot;
        let mut var_fn462_calc_ig__tfacdiodein_dn4: f64 = *var_fn462_calc_ig__tfacdiodein_dn4_slot;
        let mut var_fn462_calc_ig__vbdgin: f64 = *var_fn462_calc_ig__vbdgin_slot;
        let mut var_fn462_calc_ig__vgin: f64 = *var_fn462_calc_ig__vgin_slot;
        let mut var_fn462_calc_ig__vgin_dn7: f64 = *var_fn462_calc_ig__vgin_dn7_slot;
        let mut var_fn462_calc_ig__vgin_dn8: f64 = *var_fn462_calc_ig__vgin_dn8_slot;
        let mut var_fn462_calc_ig__vgsatin: f64 = *var_fn462_calc_ig__vgsatin_slot;
        let mut var_fn462_calc_ig__vgsatqin: f64 = *var_fn462_calc_ig__vgsatqin_slot;
        let mut var_fn462_calc_ig__w: f64 = *var_fn462_calc_ig__w_slot;
        let mut var_guard459: f64 = *var_guard459_slot;
        let mut var_guard460: f64 = *var_guard460_slot;
        let mut var_guard461: f64 = *var_guard461_slot;
        let mut var_idsch: f64 = *var_idsch_slot;
        let mut var_idsch2: f64 = *var_idsch2_slot;
        let mut var_idsch2_dn4: f64 = *var_idsch2_dn4_slot;
        let mut var_idsch2_dn7: f64 = *var_idsch2_dn7_slot;
        let mut var_idsch2_dn8: f64 = *var_idsch2_dn8_slot;
        let mut var_idsch_dn4: f64 = *var_idsch_dn4_slot;
        let mut var_idsch_dn7: f64 = *var_idsch_dn7_slot;
        let mut var_idsch_dn8: f64 = *var_idsch_dn8_slot;
        let mut var_igdi2db: f64 = *var_igdi2db_slot;
        let mut var_igdi2db_dn4: f64 = *var_igdi2db_dn4_slot;
        let mut var_igdi2db_dn5: f64 = *var_igdi2db_dn5_slot;
        let mut var_igdi2db_dn8: f64 = *var_igdi2db_dn8_slot;
        let mut var_qsch: f64 = *var_qsch_slot;
        let mut var_qsch0: f64 = *var_qsch0_slot;
        let mut var_qsch1: f64 = *var_qsch1_slot;
        let mut var_qsch1_dn7: f64 = *var_qsch1_dn7_slot;
        let mut var_qsch1_dn8: f64 = *var_qsch1_dn8_slot;
        let mut var_qsch2: f64 = *var_qsch2_slot;
        let mut var_qsch2_dn7: f64 = *var_qsch2_dn7_slot;
        let mut var_qsch2_dn8: f64 = *var_qsch2_dn8_slot;
        let mut var_qsch3: f64 = *var_qsch3_slot;
        let mut var_qsch3_dn7: f64 = *var_qsch3_dn7_slot;
        let mut var_qsch3_dn8: f64 = *var_qsch3_dn8_slot;
        let mut var_qsch4: f64 = *var_qsch4_slot;
        let mut var_qsch4_dn7: f64 = *var_qsch4_dn7_slot;
        let mut var_qsch4_dn8: f64 = *var_qsch4_dn8_slot;
        let mut var_qsch5: f64 = *var_qsch5_slot;
        let mut var_qsch5_dn7: f64 = *var_qsch5_dn7_slot;
        let mut var_qsch5_dn8: f64 = *var_qsch5_dn8_slot;
        let mut var_qsch_dn7: f64 = *var_qsch_dn7_slot;
        let mut var_qsch_dn8: f64 = *var_qsch_dn8_slot;
        let mut var_vsch: f64 = *var_vsch_slot;
        let mut var_vsch_dn7: f64 = *var_vsch_dn7_slot;
        let mut var_vsch_dn8: f64 = *var_vsch_dn8_slot;
        let mut var_vschfc1: f64 = *var_vschfc1_slot;
        let mut var_vschfc1_dn7: f64 = *var_vschfc1_dn7_slot;
        let mut var_vschfc1_dn8: f64 = *var_vschfc1_dn8_slot;
        let mut var_vschfc2: f64 = *var_vschfc2_slot;
        let mut var_vschfc2_dn7: f64 = *var_vschfc2_dn7_slot;
        let mut var_vschfc2_dn8: f64 = *var_vschfc2_dn8_slot;
        let mut var_vschfc3: f64 = *var_vschfc3_slot;
        let mut var_vschfc3_dn7: f64 = *var_vschfc3_dn7_slot;
        let mut var_vschfc3_dn8: f64 = *var_vschfc3_dn8_slot;
        let mut var_vschfc4: f64 = *var_vschfc4_slot;
        let mut var_vschfc4_dn7: f64 = *var_vschfc4_dn7_slot;
        let mut var_vschfc4_dn8: f64 = *var_vschfc4_dn8_slot;
        let mut var_vschfc5: f64 = *var_vschfc5_slot;
        let mut var_vschfc5_dn7: f64 = *var_vschfc5_dn7_slot;
        let mut var_vschfc5_dn8: f64 = *var_vschfc5_dn8_slot;

        let (assign41150_e39602, assign41150_e39602_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) {
        let assign41150_e39598: f64 = (var_fn456_calc_ig__alphagin * var_fn456_calc_ig__alphagin);
        let assign41150_e39600: f64 = (assign41150_e39598 * var_fn456_calc_ig__phitin);
        (assign41150_e39600, (assign41150_e39598 * var_fn456_calc_ig__phitin_dn4),)
    } else {
        (var_fn456_calc_ig__alpha2_phit, var_fn456_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn456_calc_ig__alpha2_phit = assign41150_e39602;
        var_fn456_calc_ig__alpha2_phit_dn4 = assign41150_e39602_d_n4;

        let (assign41160_e39621, assign41160_e39621_d_n4, assign41160_e39621_d_n5, assign41160_e39621_d_n8,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) {
        let assign41160_e39615: f64 = (var_fn456_calc_ig__alpha2_phit / 2.0);
        let assign41160_e39616: f64 = (var_fn456_calc_ig__vgsatin - assign41160_e39615);
        let assign41160_e39617: f64 = (var_fn456_calc_ig__vgin - assign41160_e39616);
        let assign41160_e39619: f64 = (assign41160_e39617 / var_fn456_calc_ig__alpha2_phit);
        (assign41160_e39619, ((((-(-(var_fn456_calc_ig__alpha2_phit_dn4 / 2.0))) * var_fn456_calc_ig__alpha2_phit) - (assign41160_e39617 * var_fn456_calc_ig__alpha2_phit_dn4)) / (var_fn456_calc_ig__alpha2_phit * var_fn456_calc_ig__alpha2_phit)), (var_fn456_calc_ig__vgin_dn5 / var_fn456_calc_ig__alpha2_phit), (var_fn456_calc_ig__vgin_dn8 / var_fn456_calc_ig__alpha2_phit),)
    } else {
        (var_fn456_calc_ig__expffvarg, var_fn456_calc_ig__expffvarg_dn4, var_fn456_calc_ig__expffvarg_dn5, var_fn456_calc_ig__expffvarg_dn8,)
    }
};
        var_fn456_calc_ig__expffvarg = assign41160_e39621;
        var_fn456_calc_ig__expffvarg_dn4 = assign41160_e39621_d_n4;
        var_fn456_calc_ig__expffvarg_dn5 = assign41160_e39621_d_n5;
        var_fn456_calc_ig__expffvarg_dn8 = assign41160_e39621_d_n8;

        let assign41170_e39624: f64 = if var_fn456_calc_ig__expffvarg > 50.0 { 1.0 } else { 0.0 };
        var_guard459 = assign41170_e39624;

        let (assign41180_e39637, assign41180_e39637_d_n4, assign41180_e39637_d_n5, assign41180_e39637_d_n8,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) && (var_guard459 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__ffvgin, var_fn456_calc_ig__ffvgin_dn4, var_fn456_calc_ig__ffvgin_dn5, var_fn456_calc_ig__ffvgin_dn8,)
    }
};
        var_fn456_calc_ig__ffvgin = assign41180_e39637;
        var_fn456_calc_ig__ffvgin_dn4 = assign41180_e39637_d_n4;
        var_fn456_calc_ig__ffvgin_dn5 = assign41180_e39637_d_n5;
        var_fn456_calc_ig__ffvgin_dn8 = assign41180_e39637_d_n8;

        let assign41190_e39640: f64 = (-50.0);
        let assign41190_e39641: f64 = if var_fn456_calc_ig__expffvarg < assign41190_e39640 { 1.0 } else { 0.0 };
        var_guard460 = assign41190_e39641;

        let (assign41200_e39657, assign41200_e39657_d_n4, assign41200_e39657_d_n5, assign41200_e39657_d_n8,) = {
    if ((((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) && (var_guard459 == 0.0)) && (var_guard460 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn456_calc_ig__ffvgin, var_fn456_calc_ig__ffvgin_dn4, var_fn456_calc_ig__ffvgin_dn5, var_fn456_calc_ig__ffvgin_dn8,)
    }
};
        var_fn456_calc_ig__ffvgin = assign41200_e39657;
        var_fn456_calc_ig__ffvgin_dn4 = assign41200_e39657_d_n4;
        var_fn456_calc_ig__ffvgin_dn5 = assign41200_e39657_d_n5;
        var_fn456_calc_ig__ffvgin_dn8 = assign41200_e39657_d_n8;

        let (assign41210_e39679, assign41210_e39679_d_n4, assign41210_e39679_d_n5, assign41210_e39679_d_n8,) = {
    if ((((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) && (var_guard459 == 0.0)) && (var_guard460 == 0.0)) {
        let assign41210_e39675: f64 = (var_fn456_calc_ig__expffvarg).exp();
        let assign41210_e39676: f64 = (1.0 + assign41210_e39675);
        let assign41210_e39677: f64 = (1.0 / assign41210_e39676);
        (assign41210_e39677, (-((assign41210_e39675 * var_fn456_calc_ig__expffvarg_dn4) / (assign41210_e39676 * assign41210_e39676))), (-((assign41210_e39675 * var_fn456_calc_ig__expffvarg_dn5) / (assign41210_e39676 * assign41210_e39676))), (-((assign41210_e39675 * var_fn456_calc_ig__expffvarg_dn8) / (assign41210_e39676 * assign41210_e39676))),)
    } else {
        (var_fn456_calc_ig__ffvgin, var_fn456_calc_ig__ffvgin_dn4, var_fn456_calc_ig__ffvgin_dn5, var_fn456_calc_ig__ffvgin_dn8,)
    }
};
        var_fn456_calc_ig__ffvgin = assign41210_e39679;
        var_fn456_calc_ig__ffvgin_dn4 = assign41210_e39679_d_n4;
        var_fn456_calc_ig__ffvgin_dn5 = assign41210_e39679_d_n5;
        var_fn456_calc_ig__ffvgin_dn8 = assign41210_e39679_d_n8;

        let (assign41220_e39698, assign41220_e39698_d_n4, assign41220_e39698_d_n5, assign41220_e39698_d_n8,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) && (var_guard457 == 0.0)) {
        let assign41220_e39690: f64 = (var_fn456_calc_ig__ffvgin * var_fn456_calc_ig__igindiode_nohinj);
        let assign41220_e39693: f64 = (1.0 - var_fn456_calc_ig__ffvgin);
        let assign41220_e39695: f64 = (assign41220_e39693 * var_fn456_calc_ig__igindiode_hinj);
        let assign41220_e39696: f64 = (assign41220_e39690 + assign41220_e39695);
        (assign41220_e39696, (((var_fn456_calc_ig__ffvgin_dn4 * var_fn456_calc_ig__igindiode_nohinj) + (var_fn456_calc_ig__ffvgin * var_fn456_calc_ig__igindiode_nohinj_dn4)) + (((-var_fn456_calc_ig__ffvgin_dn4) * var_fn456_calc_ig__igindiode_hinj) + (assign41220_e39693 * var_fn456_calc_ig__igindiode_hinj_dn4))), (((var_fn456_calc_ig__ffvgin_dn5 * var_fn456_calc_ig__igindiode_nohinj) + (var_fn456_calc_ig__ffvgin * var_fn456_calc_ig__igindiode_nohinj_dn5)) + (((-var_fn456_calc_ig__ffvgin_dn5) * var_fn456_calc_ig__igindiode_hinj) + (assign41220_e39693 * var_fn456_calc_ig__igindiode_hinj_dn5))), (((var_fn456_calc_ig__ffvgin_dn8 * var_fn456_calc_ig__igindiode_nohinj) + (var_fn456_calc_ig__ffvgin * var_fn456_calc_ig__igindiode_nohinj_dn8)) + (((-var_fn456_calc_ig__ffvgin_dn8) * var_fn456_calc_ig__igindiode_hinj) + (assign41220_e39693 * var_fn456_calc_ig__igindiode_hinj_dn8))),)
    } else {
        (var_fn456_calc_ig__igindiode, var_fn456_calc_ig__igindiode_dn4, var_fn456_calc_ig__igindiode_dn5, var_fn456_calc_ig__igindiode_dn8,)
    }
};
        var_fn456_calc_ig__igindiode = assign41220_e39698;
        var_fn456_calc_ig__igindiode_dn4 = assign41220_e39698_d_n4;
        var_fn456_calc_ig__igindiode_dn5 = assign41220_e39698_d_n5;
        var_fn456_calc_ig__igindiode_dn8 = assign41220_e39698_d_n8;

        let (assign41230_e39748, assign41230_e39748_d_n5, assign41230_e39748_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign41230_e39705: f64 = (-var_fn456_calc_ig__vgin);
        let (assign41230_e39738, assign41230_e39738_d_n5, assign41230_e39738_d_n8,) = {
            if (p.p52 != 0.0) {
                let assign41230_e39713: f64 = (var_fn456_calc_ig__vgin / var_fn456_calc_ig__vgsatqin);
                let assign41230_e39716: f64 = (0.001 / p.p53);
                let assign41230_e39719: f64 = (var_fn456_calc_ig__vgin / var_fn456_calc_ig__vgsatqin);
                let assign41230_e39720: f64 = (assign41230_e39716 * assign41230_e39719);
                let assign41230_e39721: f64 = (assign41230_e39720).tanh();
                let assign41230_e39722: f64 = (assign41230_e39713 * assign41230_e39721);
                (assign41230_e39722, (((var_fn456_calc_ig__vgin_dn5 / var_fn456_calc_ig__vgsatqin) * assign41230_e39721) + (assign41230_e39713 * ((assign41230_e39716 * (var_fn456_calc_ig__vgin_dn5 / var_fn456_calc_ig__vgsatqin)) / ((assign41230_e39720).cosh() * (assign41230_e39720).cosh())))), (((var_fn456_calc_ig__vgin_dn8 / var_fn456_calc_ig__vgsatqin) * assign41230_e39721) + (assign41230_e39713 * ((assign41230_e39716 * (var_fn456_calc_ig__vgin_dn8 / var_fn456_calc_ig__vgsatqin)) / ((assign41230_e39720).cosh() * (assign41230_e39720).cosh())))),)
            } else {
                let (assign41230_e39737, assign41230_e39737_d_n5, assign41230_e39737_d_n8,) = {
                    if (p.p52 == 0.0) {
                        let __rspice_inv_cse_0: f64 = 1.0 / var_fn456_calc_ig__vgsatqin;
                        let assign41230_e39728: f64 = (var_fn456_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign41230_e39731: f64 = (var_fn456_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign41230_e39732: f64 = (assign41230_e39728 * assign41230_e39731);
                        let assign41230_e39734: f64 = (assign41230_e39732 + p.p53);
                        let assign41230_e39735: f64 = (assign41230_e39734).sqrt();
                        (assign41230_e39735, ((((var_fn456_calc_ig__vgin_dn5 / var_fn456_calc_ig__vgsatqin) * assign41230_e39731) + (assign41230_e39728 * (var_fn456_calc_ig__vgin_dn5 / var_fn456_calc_ig__vgsatqin))) / (2.0 * assign41230_e39735)), ((((var_fn456_calc_ig__vgin_dn8 / var_fn456_calc_ig__vgsatqin) * assign41230_e39731) + (assign41230_e39728 * (var_fn456_calc_ig__vgin_dn8 / var_fn456_calc_ig__vgsatqin))) / (2.0 * assign41230_e39735)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign41230_e39737, assign41230_e39737_d_n5, assign41230_e39737_d_n8,)
            }
        };
        let assign41230_e39740: f64 = (assign41230_e39738).powf(var_fn456_calc_ig__betarecin);
        let assign41230_e39741: f64 = (1.0 + assign41230_e39740);
        let assign41230_e39744: f64 = (1.0 / var_fn456_calc_ig__betarecin);
        let assign41230_e39745: f64 = (assign41230_e39741).powf(assign41230_e39744);
        let assign41230_e39746: f64 = (assign41230_e39705 / assign41230_e39745);
        (assign41230_e39746, ((((-var_fn456_calc_ig__vgin_dn5) * assign41230_e39745) - (assign41230_e39705 * if 0.0 == 0.0 && ((assign41230_e39744) as f64).is_finite() && ((assign41230_e39744) as f64).fract() == 0.0 { if assign41230_e39744 == 0.0 { 0.0 } else { (assign41230_e39744 * ((assign41230_e39741).powf(assign41230_e39744 - 1.0) * if 0.0 == 0.0 && ((var_fn456_calc_ig__betarecin) as f64).is_finite() && ((var_fn456_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn456_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn456_calc_ig__betarecin * ((assign41230_e39738).powf(var_fn456_calc_ig__betarecin - 1.0) * assign41230_e39738_d_n5)) } } else { (assign41230_e39740 * (var_fn456_calc_ig__betarecin * (assign41230_e39738_d_n5 / assign41230_e39738))) })) } } else { (assign41230_e39745 * (assign41230_e39744 * (if 0.0 == 0.0 && ((var_fn456_calc_ig__betarecin) as f64).is_finite() && ((var_fn456_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn456_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn456_calc_ig__betarecin * ((assign41230_e39738).powf(var_fn456_calc_ig__betarecin - 1.0) * assign41230_e39738_d_n5)) } } else { (assign41230_e39740 * (var_fn456_calc_ig__betarecin * (assign41230_e39738_d_n5 / assign41230_e39738))) } / assign41230_e39741))) })) / (assign41230_e39745 * assign41230_e39745)), ((((-var_fn456_calc_ig__vgin_dn8) * assign41230_e39745) - (assign41230_e39705 * if 0.0 == 0.0 && ((assign41230_e39744) as f64).is_finite() && ((assign41230_e39744) as f64).fract() == 0.0 { if assign41230_e39744 == 0.0 { 0.0 } else { (assign41230_e39744 * ((assign41230_e39741).powf(assign41230_e39744 - 1.0) * if 0.0 == 0.0 && ((var_fn456_calc_ig__betarecin) as f64).is_finite() && ((var_fn456_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn456_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn456_calc_ig__betarecin * ((assign41230_e39738).powf(var_fn456_calc_ig__betarecin - 1.0) * assign41230_e39738_d_n8)) } } else { (assign41230_e39740 * (var_fn456_calc_ig__betarecin * (assign41230_e39738_d_n8 / assign41230_e39738))) })) } } else { (assign41230_e39745 * (assign41230_e39744 * (if 0.0 == 0.0 && ((var_fn456_calc_ig__betarecin) as f64).is_finite() && ((var_fn456_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn456_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn456_calc_ig__betarecin * ((assign41230_e39738).powf(var_fn456_calc_ig__betarecin - 1.0) * assign41230_e39738_d_n8)) } } else { (assign41230_e39740 * (var_fn456_calc_ig__betarecin * (assign41230_e39738_d_n8 / assign41230_e39738))) } / assign41230_e39741))) })) / (assign41230_e39745 * assign41230_e39745)),)
    } else {
        (var_fn456_calc_ig__frecgin, var_fn456_calc_ig__frecgin_dn5, var_fn456_calc_ig__frecgin_dn8,)
    }
};
        var_fn456_calc_ig__frecgin = assign41230_e39748;
        var_fn456_calc_ig__frecgin_dn5 = assign41230_e39748_d_n5;
        var_fn456_calc_ig__frecgin_dn8 = assign41230_e39748_d_n8;

        let (assign41240_e39767, assign41240_e39767_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign41240_e39755: f64 = (-var_fn456_calc_ig__type);
        let assign41240_e39757: f64 = (assign41240_e39755 * var_fn456_calc_ig__w);
        let assign41240_e39759: f64 = (assign41240_e39757 * var_fn456_calc_ig__ngf);
        let assign41240_e39761: f64 = (assign41240_e39759 * var_fn456_calc_ig__irecin);
        let assign41240_e39763: f64 = (assign41240_e39761 * var_fn456_calc_ig__tfacdiodein);
        let assign41240_e39765: f64 = assign41240_e39763;
        (assign41240_e39765, (assign41240_e39761 * var_fn456_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn456_calc_ig__isrecout, var_fn456_calc_ig__isrecout_dn4,)
    }
};
        var_fn456_calc_ig__isrecout = assign41240_e39767;
        var_fn456_calc_ig__isrecout_dn4 = assign41240_e39767_d_n4;

        let (assign41250_e39779, assign41250_e39779_d_n4, assign41250_e39779_d_n5, assign41250_e39779_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign41250_e39775: f64 = (var_fn456_calc_ig__pgsrecin / var_fn456_calc_ig__phitin);
        let assign41250_e39777: f64 = (assign41250_e39775 * var_fn456_calc_ig__frecgin);
        (assign41250_e39777, ((-((var_fn456_calc_ig__pgsrecin * var_fn456_calc_ig__phitin_dn4) / (var_fn456_calc_ig__phitin * var_fn456_calc_ig__phitin))) * var_fn456_calc_ig__frecgin), (assign41250_e39775 * var_fn456_calc_ig__frecgin_dn5), (assign41250_e39775 * var_fn456_calc_ig__frecgin_dn8),)
    } else {
        (var_fn456_calc_ig__expirevarg, var_fn456_calc_ig__expirevarg_dn4, var_fn456_calc_ig__expirevarg_dn5, var_fn456_calc_ig__expirevarg_dn8,)
    }
};
        var_fn456_calc_ig__expirevarg = assign41250_e39779;
        var_fn456_calc_ig__expirevarg_dn4 = assign41250_e39779_d_n4;
        var_fn456_calc_ig__expirevarg_dn5 = assign41250_e39779_d_n5;
        var_fn456_calc_ig__expirevarg_dn8 = assign41250_e39779_d_n8;

        let (assign41260_e39825, assign41260_e39825_d_n4, assign41260_e39825_d_n5, assign41260_e39825_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign41260_e39791: f64 = (-50.0);
        let (assign41260_e39823, assign41260_e39823_d_n4, assign41260_e39823_d_n5, assign41260_e39823_d_n8,) = {
            if ((!(var_fn456_calc_ig__expirevarg > 50.0)) && (!(var_fn456_calc_ig__expirevarg < assign41260_e39791))) {
                let assign41260_e39796: f64 = (var_fn456_calc_ig__expirevarg).exp();
                (assign41260_e39796, (assign41260_e39796 * var_fn456_calc_ig__expirevarg_dn4), (assign41260_e39796 * var_fn456_calc_ig__expirevarg_dn5), (assign41260_e39796 * var_fn456_calc_ig__expirevarg_dn8),)
            } else {
                let assign41260_e39803: f64 = (-50.0);
                let (assign41260_e39822, assign41260_e39822_d_n4, assign41260_e39822_d_n5, assign41260_e39822_d_n8,) = {
                    if ((!(var_fn456_calc_ig__expirevarg > 50.0)) && (var_fn456_calc_ig__expirevarg < assign41260_e39803)) {
                        let assign41260_e39807: f64 = (-50.0);
                        let assign41260_e39808: f64 = (assign41260_e39807).exp();
                        (assign41260_e39808, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign41260_e39821, assign41260_e39821_d_n4, assign41260_e39821_d_n5, assign41260_e39821_d_n8,) = {
                            if (var_fn456_calc_ig__expirevarg > 50.0) {
                                let assign41260_e39813: f64 = (50.0_f64).exp();
                                let assign41260_e39817: f64 = (var_fn456_calc_ig__expirevarg - 50.0);
                                let assign41260_e39818: f64 = (1.0 + assign41260_e39817);
                                let assign41260_e39819: f64 = (assign41260_e39813 * assign41260_e39818);
                                (assign41260_e39819, (assign41260_e39813 * var_fn456_calc_ig__expirevarg_dn4), (assign41260_e39813 * var_fn456_calc_ig__expirevarg_dn5), (assign41260_e39813 * var_fn456_calc_ig__expirevarg_dn8),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign41260_e39821, assign41260_e39821_d_n4, assign41260_e39821_d_n5, assign41260_e39821_d_n8,)
                    }
                };
                (assign41260_e39822, assign41260_e39822_d_n4, assign41260_e39822_d_n5, assign41260_e39822_d_n8,)
            }
        };
        (assign41260_e39823, assign41260_e39823_d_n4, assign41260_e39823_d_n5, assign41260_e39823_d_n8,)
    } else {
        (var_fn456_calc_ig__expirev, var_fn456_calc_ig__expirev_dn4, var_fn456_calc_ig__expirev_dn5, var_fn456_calc_ig__expirev_dn8,)
    }
};
        var_fn456_calc_ig__expirev = assign41260_e39825;
        var_fn456_calc_ig__expirev_dn4 = assign41260_e39825_d_n4;
        var_fn456_calc_ig__expirev_dn5 = assign41260_e39825_d_n5;
        var_fn456_calc_ig__expirev_dn8 = assign41260_e39825_d_n8;

        let (assign41270_e39837, assign41270_e39837_d_n4, assign41270_e39837_d_n5, assign41270_e39837_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign41270_e39834: f64 = (var_fn456_calc_ig__expirev - 1.0);
        let assign41270_e39835: f64 = (var_fn456_calc_ig__isrecout * assign41270_e39834);
        (assign41270_e39835, ((var_fn456_calc_ig__isrecout_dn4 * assign41270_e39834) + (var_fn456_calc_ig__isrecout * var_fn456_calc_ig__expirev_dn4)), (var_fn456_calc_ig__isrecout * var_fn456_calc_ig__expirev_dn5), (var_fn456_calc_ig__isrecout * var_fn456_calc_ig__expirev_dn8),)
    } else {
        (var_fn456_calc_ig__iginrec, var_fn456_calc_ig__iginrec_dn4, var_fn456_calc_ig__iginrec_dn5, var_fn456_calc_ig__iginrec_dn8,)
    }
};
        var_fn456_calc_ig__iginrec = assign41270_e39837;
        var_fn456_calc_ig__iginrec_dn4 = assign41270_e39837_d_n4;
        var_fn456_calc_ig__iginrec_dn5 = assign41270_e39837_d_n5;
        var_fn456_calc_ig__iginrec_dn8 = assign41270_e39837_d_n8;

        let (assign41280_e39847, assign41280_e39847_d_n4, assign41280_e39847_d_n5, assign41280_e39847_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let assign41280_e39845: f64 = (var_fn456_calc_ig__igindiode + var_fn456_calc_ig__iginrec);
        (assign41280_e39845, (var_fn456_calc_ig__igindiode_dn4 + var_fn456_calc_ig__iginrec_dn4), (var_fn456_calc_ig__igindiode_dn5 + var_fn456_calc_ig__iginrec_dn5), (var_fn456_calc_ig__igindiode_dn8 + var_fn456_calc_ig__iginrec_dn8),)
    } else {
        (var_fn456_calc_ig__igout, var_fn456_calc_ig__igout_dn4, var_fn456_calc_ig__igout_dn5, var_fn456_calc_ig__igout_dn8,)
    }
};
        var_fn456_calc_ig__igout = assign41280_e39847;
        var_fn456_calc_ig__igout_dn4 = assign41280_e39847_d_n4;
        var_fn456_calc_ig__igout_dn5 = assign41280_e39847_d_n5;
        var_fn456_calc_ig__igout_dn8 = assign41280_e39847_d_n8;

        let (assign41290_e39855, assign41290_e39855_d_n4, assign41290_e39855_d_n5, assign41290_e39855_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (var_fn456_calc_ig__igout, var_fn456_calc_ig__igout_dn4, var_fn456_calc_ig__igout_dn5, var_fn456_calc_ig__igout_dn8,)
    } else {
        (var_fn456_calc_ig__return, var_fn456_calc_ig__return_dn4, var_fn456_calc_ig__return_dn5, var_fn456_calc_ig__return_dn8,)
    }
};
        var_fn456_calc_ig__return = assign41290_e39855;
        var_fn456_calc_ig__return_dn4 = assign41290_e39855_d_n4;
        var_fn456_calc_ig__return_dn5 = assign41290_e39855_d_n5;
        var_fn456_calc_ig__return_dn8 = assign41290_e39855_d_n8;

        let (assign41320_e39879, assign41320_e39879_d_n4, assign41320_e39879_d_n5, assign41320_e39879_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        (var_fn456_calc_ig__return, var_fn456_calc_ig__return_dn4, var_fn456_calc_ig__return_dn5, var_fn456_calc_ig__return_dn8,)
    } else {
        (var_igdi2db, var_igdi2db_dn4, var_igdi2db_dn5, var_igdi2db_dn8,)
    }
};
        var_igdi2db = assign41320_e39879;
        var_igdi2db_dn4 = assign41320_e39879_d_n4;
        var_igdi2db_dn5 = assign41320_e39879_d_n5;
        var_igdi2db_dn8 = assign41320_e39879_d_n8;

        var_vsch = 0.0;
        var_vsch_dn7 = 0.0;
        var_vsch_dn8 = 0.0;

        var_idsch = 0.0;
        var_idsch_dn4 = 0.0;
        var_idsch_dn7 = 0.0;
        var_idsch_dn8 = 0.0;

        var_idsch2 = 0.0;
        var_idsch2_dn4 = 0.0;
        var_idsch2_dn7 = 0.0;
        var_idsch2_dn8 = 0.0;

        var_qsch = 0.0;
        var_qsch_dn7 = 0.0;
        var_qsch_dn8 = 0.0;

        var_qsch0 = 0.0;

        var_qsch1 = 0.0;
        var_qsch1_dn7 = 0.0;
        var_qsch1_dn8 = 0.0;

        var_qsch2 = 0.0;
        var_qsch2_dn7 = 0.0;
        var_qsch2_dn8 = 0.0;

        var_qsch3 = 0.0;
        var_qsch3_dn7 = 0.0;
        var_qsch3_dn8 = 0.0;

        var_qsch4 = 0.0;
        var_qsch4_dn7 = 0.0;
        var_qsch4_dn8 = 0.0;

        var_qsch5 = 0.0;
        var_qsch5_dn7 = 0.0;
        var_qsch5_dn8 = 0.0;

        var_vschfc1 = 0.0;
        var_vschfc1_dn7 = 0.0;
        var_vschfc1_dn8 = 0.0;

        var_vschfc2 = 0.0;
        var_vschfc2_dn7 = 0.0;
        var_vschfc2_dn8 = 0.0;

        var_vschfc3 = 0.0;
        var_vschfc3_dn7 = 0.0;
        var_vschfc3_dn8 = 0.0;

        var_vschfc4 = 0.0;
        var_vschfc4_dn7 = 0.0;
        var_vschfc4_dn8 = 0.0;

        var_vschfc5 = 0.0;
        var_vschfc5_dn7 = 0.0;
        var_vschfc5_dn8 = 0.0;

        let assign41530_e39902: f64 = if p.p291 == 1.0 { 1.0 } else { 0.0 };
        var_guard461 = assign41530_e39902;

        let (assign41540_e39908, assign41540_e39908_d_n7, assign41540_e39908_d_n8,) = {
    if (var_guard461 != 0.0) {
        let assign41540_e39906: f64 = (p.p6 * (nv8 - nv7));
        (assign41540_e39906, (-p.p6), p.p6,)
    } else {
        (var_vsch, var_vsch_dn7, var_vsch_dn8,)
    }
};
        var_vsch = assign41540_e39908;
        var_vsch_dn7 = assign41540_e39908_d_n7;
        var_vsch_dn8 = assign41540_e39908_d_n8;

        let (assign41550_e39912, assign41550_e39912_d_n4, assign41550_e39912_d_n7, assign41550_e39912_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__return, var_fn462_calc_ig__return_dn4, var_fn462_calc_ig__return_dn7, var_fn462_calc_ig__return_dn8,)
    }
};
        var_fn462_calc_ig__return = assign41550_e39912;
        var_fn462_calc_ig__return_dn4 = assign41550_e39912_d_n4;
        var_fn462_calc_ig__return_dn7 = assign41550_e39912_d_n7;
        var_fn462_calc_ig__return_dn8 = assign41550_e39912_d_n8;

        let (assign41560_e39916, assign41560_e39916_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__isdiodeout, var_fn462_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn462_calc_ig__isdiodeout = assign41560_e39916;
        var_fn462_calc_ig__isdiodeout_dn4 = assign41560_e39916_d_n4;

        let (assign41570_e39920, assign41570_e39920_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__isrecout, var_fn462_calc_ig__isrecout_dn4,)
    }
};
        var_fn462_calc_ig__isrecout = assign41570_e39920;
        var_fn462_calc_ig__isrecout_dn4 = assign41570_e39920_d_n4;

        let (assign41580_e39924, assign41580_e39924_d_n7, assign41580_e39924_d_n8,) = {
    if (var_guard461 != 0.0) {
        (var_vsch, var_vsch_dn7, var_vsch_dn8,)
    } else {
        (var_fn462_calc_ig__vgin, var_fn462_calc_ig__vgin_dn7, var_fn462_calc_ig__vgin_dn8,)
    }
};
        var_fn462_calc_ig__vgin = assign41580_e39924;
        var_fn462_calc_ig__vgin_dn7 = assign41580_e39924_d_n7;
        var_fn462_calc_ig__vgin_dn8 = assign41580_e39924_d_n8;

        let (assign41590_e39928, assign41590_e39928_d_n4,) = {
    if (var_guard461 != 0.0) {
        (var_phit, var_phit_dn4,)
    } else {
        (var_fn462_calc_ig__phitin, var_fn462_calc_ig__phitin_dn4,)
    }
};
        var_fn462_calc_ig__phitin = assign41590_e39928;
        var_fn462_calc_ig__phitin_dn4 = assign41590_e39928_d_n4;

        let (assign41600_e39932,) = {
    if (var_guard461 != 0.0) {
        (p.p294,)
    } else {
        (var_fn462_calc_ig__vgsatin,)
    }
};
        var_fn462_calc_ig__vgsatin = assign41600_e39932;

        let (assign41610_e39936,) = {
    if (var_guard461 != 0.0) {
        (p.p296,)
    } else {
        (var_fn462_calc_ig__alphagin,)
    }
};
        var_fn462_calc_ig__alphagin = assign41610_e39936;

        let (assign41620_e39940,) = {
    if (var_guard461 != 0.0) {
        (p.p295,)
    } else {
        (var_fn462_calc_ig__fracin,)
    }
};
        var_fn462_calc_ig__fracin = assign41620_e39940;

        let (assign41630_e39944,) = {
    if (var_guard461 != 0.0) {
        (p.p292,)
    } else {
        (var_fn462_calc_ig__pg_paramin,)
    }
};
        var_fn462_calc_ig__pg_paramin = assign41630_e39944;

        let (assign41640_e39948,) = {
    if (var_guard461 != 0.0) {
        (4.0,)
    } else {
        (var_fn462_calc_ig__pbdgin,)
    }
};
        var_fn462_calc_ig__pbdgin = assign41640_e39948;

        let (assign41650_e39952,) = {
    if (var_guard461 != 0.0) {
        (600.0,)
    } else {
        (var_fn462_calc_ig__vbdgin,)
    }
};
        var_fn462_calc_ig__vbdgin = assign41650_e39952;

        let (assign41660_e39956, assign41660_e39956_d_n4,) = {
    if (var_guard461 != 0.0) {
        (var_tfacdiode, var_tfacdiode_dn4,)
    } else {
        (var_fn462_calc_ig__tfacdiodein, var_fn462_calc_ig__tfacdiodein_dn4,)
    }
};
        var_fn462_calc_ig__tfacdiodein = assign41660_e39956;
        var_fn462_calc_ig__tfacdiodein_dn4 = assign41660_e39956_d_n4;

        let (assign41670_e39964,) = {
    if (var_guard461 != 0.0) {
        let assign41670_e39961: f64 = (1.0 - p.p311);
        let assign41670_e39962: f64 = (p.p0 * assign41670_e39961);
        (assign41670_e39962,)
    } else {
        (var_fn462_calc_ig__w,)
    }
};
        var_fn462_calc_ig__w = assign41670_e39964;

        let (assign41680_e39968,) = {
    if (var_guard461 != 0.0) {
        (p.p2,)
    } else {
        (var_fn462_calc_ig__ngf,)
    }
};
        var_fn462_calc_ig__ngf = assign41680_e39968;

        let (assign41690_e39972,) = {
    if (var_guard461 != 0.0) {
        (p.p293,)
    } else {
        (var_fn462_calc_ig__ijin,)
    }
};
        var_fn462_calc_ig__ijin = assign41690_e39972;

        let (assign41700_e39976,) = {
    if (var_guard461 != 0.0) {
        (0.0,)
    } else {
        (var_fn462_calc_ig__kbdgatein,)
    }
};
        var_fn462_calc_ig__kbdgatein = assign41700_e39976;

        let (assign41710_e39980,) = {
    if (var_guard461 != 0.0) {
        (p.p299,)
    } else {
        (var_fn462_calc_ig__vgsatqin,)
    }
};
        var_fn462_calc_ig__vgsatqin = assign41710_e39980;

        let (assign41720_e39984,) = {
    if (var_guard461 != 0.0) {
        (p.p300,)
    } else {
        (var_fn462_calc_ig__betarecin,)
    }
};
        var_fn462_calc_ig__betarecin = assign41720_e39984;

        let (assign41730_e39988,) = {
    if (var_guard461 != 0.0) {
        (p.p298,)
    } else {
        (var_fn462_calc_ig__irecin,)
    }
};
        var_fn462_calc_ig__irecin = assign41730_e39988;

        *var_fn456_calc_ig__alpha2_phit_slot = var_fn456_calc_ig__alpha2_phit;
        *var_fn456_calc_ig__alpha2_phit_dn4_slot = var_fn456_calc_ig__alpha2_phit_dn4;
        *var_fn456_calc_ig__expffvarg_slot = var_fn456_calc_ig__expffvarg;
        *var_fn456_calc_ig__expffvarg_dn4_slot = var_fn456_calc_ig__expffvarg_dn4;
        *var_fn456_calc_ig__expffvarg_dn5_slot = var_fn456_calc_ig__expffvarg_dn5;
        *var_fn456_calc_ig__expffvarg_dn8_slot = var_fn456_calc_ig__expffvarg_dn8;
        *var_fn456_calc_ig__expirev_slot = var_fn456_calc_ig__expirev;
        *var_fn456_calc_ig__expirev_dn4_slot = var_fn456_calc_ig__expirev_dn4;
        *var_fn456_calc_ig__expirev_dn5_slot = var_fn456_calc_ig__expirev_dn5;
        *var_fn456_calc_ig__expirev_dn8_slot = var_fn456_calc_ig__expirev_dn8;
        *var_fn456_calc_ig__expirevarg_slot = var_fn456_calc_ig__expirevarg;
        *var_fn456_calc_ig__expirevarg_dn4_slot = var_fn456_calc_ig__expirevarg_dn4;
        *var_fn456_calc_ig__expirevarg_dn5_slot = var_fn456_calc_ig__expirevarg_dn5;
        *var_fn456_calc_ig__expirevarg_dn8_slot = var_fn456_calc_ig__expirevarg_dn8;
        *var_fn456_calc_ig__ffvgin_slot = var_fn456_calc_ig__ffvgin;
        *var_fn456_calc_ig__ffvgin_dn4_slot = var_fn456_calc_ig__ffvgin_dn4;
        *var_fn456_calc_ig__ffvgin_dn5_slot = var_fn456_calc_ig__ffvgin_dn5;
        *var_fn456_calc_ig__ffvgin_dn8_slot = var_fn456_calc_ig__ffvgin_dn8;
        *var_fn456_calc_ig__frecgin_slot = var_fn456_calc_ig__frecgin;
        *var_fn456_calc_ig__frecgin_dn5_slot = var_fn456_calc_ig__frecgin_dn5;
        *var_fn456_calc_ig__frecgin_dn8_slot = var_fn456_calc_ig__frecgin_dn8;
        *var_fn456_calc_ig__igindiode_slot = var_fn456_calc_ig__igindiode;
        *var_fn456_calc_ig__igindiode_dn4_slot = var_fn456_calc_ig__igindiode_dn4;
        *var_fn456_calc_ig__igindiode_dn5_slot = var_fn456_calc_ig__igindiode_dn5;
        *var_fn456_calc_ig__igindiode_dn8_slot = var_fn456_calc_ig__igindiode_dn8;
        *var_fn456_calc_ig__iginrec_slot = var_fn456_calc_ig__iginrec;
        *var_fn456_calc_ig__iginrec_dn4_slot = var_fn456_calc_ig__iginrec_dn4;
        *var_fn456_calc_ig__iginrec_dn5_slot = var_fn456_calc_ig__iginrec_dn5;
        *var_fn456_calc_ig__iginrec_dn8_slot = var_fn456_calc_ig__iginrec_dn8;
        *var_fn456_calc_ig__igout_slot = var_fn456_calc_ig__igout;
        *var_fn456_calc_ig__igout_dn4_slot = var_fn456_calc_ig__igout_dn4;
        *var_fn456_calc_ig__igout_dn5_slot = var_fn456_calc_ig__igout_dn5;
        *var_fn456_calc_ig__igout_dn8_slot = var_fn456_calc_ig__igout_dn8;
        *var_fn456_calc_ig__isrecout_slot = var_fn456_calc_ig__isrecout;
        *var_fn456_calc_ig__isrecout_dn4_slot = var_fn456_calc_ig__isrecout_dn4;
        *var_fn456_calc_ig__return_slot = var_fn456_calc_ig__return;
        *var_fn456_calc_ig__return_dn4_slot = var_fn456_calc_ig__return_dn4;
        *var_fn456_calc_ig__return_dn5_slot = var_fn456_calc_ig__return_dn5;
        *var_fn456_calc_ig__return_dn8_slot = var_fn456_calc_ig__return_dn8;
        *var_fn462_calc_ig__alphagin_slot = var_fn462_calc_ig__alphagin;
        *var_fn462_calc_ig__betarecin_slot = var_fn462_calc_ig__betarecin;
        *var_fn462_calc_ig__fracin_slot = var_fn462_calc_ig__fracin;
        *var_fn462_calc_ig__ijin_slot = var_fn462_calc_ig__ijin;
        *var_fn462_calc_ig__irecin_slot = var_fn462_calc_ig__irecin;
        *var_fn462_calc_ig__isdiodeout_slot = var_fn462_calc_ig__isdiodeout;
        *var_fn462_calc_ig__isdiodeout_dn4_slot = var_fn462_calc_ig__isdiodeout_dn4;
        *var_fn462_calc_ig__isrecout_slot = var_fn462_calc_ig__isrecout;
        *var_fn462_calc_ig__isrecout_dn4_slot = var_fn462_calc_ig__isrecout_dn4;
        *var_fn462_calc_ig__kbdgatein_slot = var_fn462_calc_ig__kbdgatein;
        *var_fn462_calc_ig__ngf_slot = var_fn462_calc_ig__ngf;
        *var_fn462_calc_ig__pbdgin_slot = var_fn462_calc_ig__pbdgin;
        *var_fn462_calc_ig__pg_paramin_slot = var_fn462_calc_ig__pg_paramin;
        *var_fn462_calc_ig__phitin_slot = var_fn462_calc_ig__phitin;
        *var_fn462_calc_ig__phitin_dn4_slot = var_fn462_calc_ig__phitin_dn4;
        *var_fn462_calc_ig__return_slot = var_fn462_calc_ig__return;
        *var_fn462_calc_ig__return_dn4_slot = var_fn462_calc_ig__return_dn4;
        *var_fn462_calc_ig__return_dn7_slot = var_fn462_calc_ig__return_dn7;
        *var_fn462_calc_ig__return_dn8_slot = var_fn462_calc_ig__return_dn8;
        *var_fn462_calc_ig__tfacdiodein_slot = var_fn462_calc_ig__tfacdiodein;
        *var_fn462_calc_ig__tfacdiodein_dn4_slot = var_fn462_calc_ig__tfacdiodein_dn4;
        *var_fn462_calc_ig__vbdgin_slot = var_fn462_calc_ig__vbdgin;
        *var_fn462_calc_ig__vgin_slot = var_fn462_calc_ig__vgin;
        *var_fn462_calc_ig__vgin_dn7_slot = var_fn462_calc_ig__vgin_dn7;
        *var_fn462_calc_ig__vgin_dn8_slot = var_fn462_calc_ig__vgin_dn8;
        *var_fn462_calc_ig__vgsatin_slot = var_fn462_calc_ig__vgsatin;
        *var_fn462_calc_ig__vgsatqin_slot = var_fn462_calc_ig__vgsatqin;
        *var_fn462_calc_ig__w_slot = var_fn462_calc_ig__w;
        *var_guard459_slot = var_guard459;
        *var_guard460_slot = var_guard460;
        *var_guard461_slot = var_guard461;
        *var_idsch_slot = var_idsch;
        *var_idsch2_slot = var_idsch2;
        *var_idsch2_dn4_slot = var_idsch2_dn4;
        *var_idsch2_dn7_slot = var_idsch2_dn7;
        *var_idsch2_dn8_slot = var_idsch2_dn8;
        *var_idsch_dn4_slot = var_idsch_dn4;
        *var_idsch_dn7_slot = var_idsch_dn7;
        *var_idsch_dn8_slot = var_idsch_dn8;
        *var_igdi2db_slot = var_igdi2db;
        *var_igdi2db_dn4_slot = var_igdi2db_dn4;
        *var_igdi2db_dn5_slot = var_igdi2db_dn5;
        *var_igdi2db_dn8_slot = var_igdi2db_dn8;
        *var_qsch_slot = var_qsch;
        *var_qsch0_slot = var_qsch0;
        *var_qsch1_slot = var_qsch1;
        *var_qsch1_dn7_slot = var_qsch1_dn7;
        *var_qsch1_dn8_slot = var_qsch1_dn8;
        *var_qsch2_slot = var_qsch2;
        *var_qsch2_dn7_slot = var_qsch2_dn7;
        *var_qsch2_dn8_slot = var_qsch2_dn8;
        *var_qsch3_slot = var_qsch3;
        *var_qsch3_dn7_slot = var_qsch3_dn7;
        *var_qsch3_dn8_slot = var_qsch3_dn8;
        *var_qsch4_slot = var_qsch4;
        *var_qsch4_dn7_slot = var_qsch4_dn7;
        *var_qsch4_dn8_slot = var_qsch4_dn8;
        *var_qsch5_slot = var_qsch5;
        *var_qsch5_dn7_slot = var_qsch5_dn7;
        *var_qsch5_dn8_slot = var_qsch5_dn8;
        *var_qsch_dn7_slot = var_qsch_dn7;
        *var_qsch_dn8_slot = var_qsch_dn8;
        *var_vsch_slot = var_vsch;
        *var_vsch_dn7_slot = var_vsch_dn7;
        *var_vsch_dn8_slot = var_vsch_dn8;
        *var_vschfc1_slot = var_vschfc1;
        *var_vschfc1_dn7_slot = var_vschfc1_dn7;
        *var_vschfc1_dn8_slot = var_vschfc1_dn8;
        *var_vschfc2_slot = var_vschfc2;
        *var_vschfc2_dn7_slot = var_vschfc2_dn7;
        *var_vschfc2_dn8_slot = var_vschfc2_dn8;
        *var_vschfc3_slot = var_vschfc3;
        *var_vschfc3_dn7_slot = var_vschfc3_dn7;
        *var_vschfc3_dn8_slot = var_vschfc3_dn8;
        *var_vschfc4_slot = var_vschfc4;
        *var_vschfc4_dn7_slot = var_vschfc4_dn7;
        *var_vschfc4_dn8_slot = var_vschfc4_dn8;
        *var_vschfc5_slot = var_vschfc5;
        *var_vschfc5_dn7_slot = var_vschfc5_dn7;
        *var_vschfc5_dn8_slot = var_vschfc5_dn8;
    }

    pub(super) fn stamp_transient_block_104(
        p: &Parameters,
        var_fn462_calc_ig__pbdgin: f64,
        var_fn462_calc_ig__phitin: f64,
        var_fn462_calc_ig__phitin_dn4: f64,
        var_fn462_calc_ig__vbdgin: f64,
        var_fn462_calc_ig__vgin: f64,
        var_fn462_calc_ig__vgin_dn7: f64,
        var_fn462_calc_ig__vgin_dn8: f64,
        var_guard461: f64,
        var_fn462_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn462_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn462_calc_ig__expbd1_slot: &mut f64,
        var_fn462_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn462_calc_ig__expbd1_dn7_slot: &mut f64,
        var_fn462_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn462_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn462_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__expbd2_slot: &mut f64,
        var_fn462_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn462_calc_ig__expbdarg1_slot: &mut f64,
        var_fn462_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn462_calc_ig__expbdarg1_dn7_slot: &mut f64,
        var_fn462_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn462_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn462_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__expbdarg2_slot: &mut f64,
        var_fn462_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn462_calc_ig__expffvarg_slot: &mut f64,
        var_fn462_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn462_calc_ig__expffvarg_dn7_slot: &mut f64,
        var_fn462_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn462_calc_ig__expifor_slot: &mut f64,
        var_fn462_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn462_calc_ig__expifor_dn7_slot: &mut f64,
        var_fn462_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn462_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn462_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn462_calc_ig__expifor_hinj_dn7_slot: &mut f64,
        var_fn462_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn462_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn462_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn462_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_dn7_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_hinj_dn7_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__expirev_slot: &mut f64,
        var_fn462_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn462_calc_ig__expirev_dn7_slot: &mut f64,
        var_fn462_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn462_calc_ig__expirevarg_slot: &mut f64,
        var_fn462_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn462_calc_ig__expirevarg_dn7_slot: &mut f64,
        var_fn462_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn462_calc_ig__expphib_slot: &mut f64,
        var_fn462_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn462_calc_ig__ffvgin_slot: &mut f64,
        var_fn462_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn462_calc_ig__ffvgin_dn7_slot: &mut f64,
        var_fn462_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn462_calc_ig__frecgin_slot: &mut f64,
        var_fn462_calc_ig__frecgin_dn7_slot: &mut f64,
        var_fn462_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn462_calc_ig__iginbd_slot: &mut f64,
        var_fn462_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn462_calc_ig__iginbd_dn7_slot: &mut f64,
        var_fn462_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn462_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn462_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__igindiode_slot: &mut f64,
        var_fn462_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn462_calc_ig__igindiode_dn7_slot: &mut f64,
        var_fn462_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn462_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn462_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn462_calc_ig__igindiode_hinj_dn7_slot: &mut f64,
        var_fn462_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn462_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn462_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn462_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn462_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn462_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn462_calc_ig__igindiode_nohinj_dn7_slot: &mut f64,
        var_fn462_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn462_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn462_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__iginrec_slot: &mut f64,
        var_fn462_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn462_calc_ig__iginrec_dn7_slot: &mut f64,
        var_fn462_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn462_calc_ig__igout_slot: &mut f64,
        var_fn462_calc_ig__igout_dn4_slot: &mut f64,
        var_fn462_calc_ig__igout_dn7_slot: &mut f64,
        var_fn462_calc_ig__igout_dn8_slot: &mut f64,
        var_fn462_calc_ig__pg_param1_slot: &mut f64,
        var_fn462_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_fn462_calc_ig__pgsrecin_slot: &mut f64,
        var_fn462_calc_ig__t0_slot: &mut f64,
        var_fn462_calc_ig__t0_dn4_slot: &mut f64,
        var_fn462_calc_ig__type_slot: &mut f64,
        var_fn462_calc_ig__vjg_slot: &mut f64,
    ) {
        let mut var_fn462_calc_ig__alpha2_phit: f64 = *var_fn462_calc_ig__alpha2_phit_slot;
        let mut var_fn462_calc_ig__alpha2_phit_dn4: f64 = *var_fn462_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn462_calc_ig__expbd1: f64 = *var_fn462_calc_ig__expbd1_slot;
        let mut var_fn462_calc_ig__expbd1_dn4: f64 = *var_fn462_calc_ig__expbd1_dn4_slot;
        let mut var_fn462_calc_ig__expbd1_dn7: f64 = *var_fn462_calc_ig__expbd1_dn7_slot;
        let mut var_fn462_calc_ig__expbd1_dn8: f64 = *var_fn462_calc_ig__expbd1_dn8_slot;
        let mut var_fn462_calc_ig__expbd1_vgsat: f64 = *var_fn462_calc_ig__expbd1_vgsat_slot;
        let mut var_fn462_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn462_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__expbd2: f64 = *var_fn462_calc_ig__expbd2_slot;
        let mut var_fn462_calc_ig__expbd2_dn4: f64 = *var_fn462_calc_ig__expbd2_dn4_slot;
        let mut var_fn462_calc_ig__expbdarg1: f64 = *var_fn462_calc_ig__expbdarg1_slot;
        let mut var_fn462_calc_ig__expbdarg1_dn4: f64 = *var_fn462_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn462_calc_ig__expbdarg1_dn7: f64 = *var_fn462_calc_ig__expbdarg1_dn7_slot;
        let mut var_fn462_calc_ig__expbdarg1_dn8: f64 = *var_fn462_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn462_calc_ig__expbdarg1_vgsat: f64 = *var_fn462_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn462_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn462_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__expbdarg2: f64 = *var_fn462_calc_ig__expbdarg2_slot;
        let mut var_fn462_calc_ig__expbdarg2_dn4: f64 = *var_fn462_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn462_calc_ig__expffvarg: f64 = *var_fn462_calc_ig__expffvarg_slot;
        let mut var_fn462_calc_ig__expffvarg_dn4: f64 = *var_fn462_calc_ig__expffvarg_dn4_slot;
        let mut var_fn462_calc_ig__expffvarg_dn7: f64 = *var_fn462_calc_ig__expffvarg_dn7_slot;
        let mut var_fn462_calc_ig__expffvarg_dn8: f64 = *var_fn462_calc_ig__expffvarg_dn8_slot;
        let mut var_fn462_calc_ig__expifor: f64 = *var_fn462_calc_ig__expifor_slot;
        let mut var_fn462_calc_ig__expifor_dn4: f64 = *var_fn462_calc_ig__expifor_dn4_slot;
        let mut var_fn462_calc_ig__expifor_dn7: f64 = *var_fn462_calc_ig__expifor_dn7_slot;
        let mut var_fn462_calc_ig__expifor_dn8: f64 = *var_fn462_calc_ig__expifor_dn8_slot;
        let mut var_fn462_calc_ig__expifor_hinj: f64 = *var_fn462_calc_ig__expifor_hinj_slot;
        let mut var_fn462_calc_ig__expifor_hinj_dn4: f64 = *var_fn462_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn462_calc_ig__expifor_hinj_dn7: f64 = *var_fn462_calc_ig__expifor_hinj_dn7_slot;
        let mut var_fn462_calc_ig__expifor_hinj_dn8: f64 = *var_fn462_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn462_calc_ig__expifor_hinj_vgsat: f64 = *var_fn462_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn462_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn462_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn462_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn462_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn462_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__expiforarg: f64 = *var_fn462_calc_ig__expiforarg_slot;
        let mut var_fn462_calc_ig__expiforarg_dn4: f64 = *var_fn462_calc_ig__expiforarg_dn4_slot;
        let mut var_fn462_calc_ig__expiforarg_dn7: f64 = *var_fn462_calc_ig__expiforarg_dn7_slot;
        let mut var_fn462_calc_ig__expiforarg_dn8: f64 = *var_fn462_calc_ig__expiforarg_dn8_slot;
        let mut var_fn462_calc_ig__expiforarg_hinj: f64 = *var_fn462_calc_ig__expiforarg_hinj_slot;
        let mut var_fn462_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn462_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn462_calc_ig__expiforarg_hinj_dn7: f64 = *var_fn462_calc_ig__expiforarg_hinj_dn7_slot;
        let mut var_fn462_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn462_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn462_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn462_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn462_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn462_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn462_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn462_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn462_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__expirev: f64 = *var_fn462_calc_ig__expirev_slot;
        let mut var_fn462_calc_ig__expirev_dn4: f64 = *var_fn462_calc_ig__expirev_dn4_slot;
        let mut var_fn462_calc_ig__expirev_dn7: f64 = *var_fn462_calc_ig__expirev_dn7_slot;
        let mut var_fn462_calc_ig__expirev_dn8: f64 = *var_fn462_calc_ig__expirev_dn8_slot;
        let mut var_fn462_calc_ig__expirevarg: f64 = *var_fn462_calc_ig__expirevarg_slot;
        let mut var_fn462_calc_ig__expirevarg_dn4: f64 = *var_fn462_calc_ig__expirevarg_dn4_slot;
        let mut var_fn462_calc_ig__expirevarg_dn7: f64 = *var_fn462_calc_ig__expirevarg_dn7_slot;
        let mut var_fn462_calc_ig__expirevarg_dn8: f64 = *var_fn462_calc_ig__expirevarg_dn8_slot;
        let mut var_fn462_calc_ig__expphib: f64 = *var_fn462_calc_ig__expphib_slot;
        let mut var_fn462_calc_ig__expphib_dn4: f64 = *var_fn462_calc_ig__expphib_dn4_slot;
        let mut var_fn462_calc_ig__ffvgin: f64 = *var_fn462_calc_ig__ffvgin_slot;
        let mut var_fn462_calc_ig__ffvgin_dn4: f64 = *var_fn462_calc_ig__ffvgin_dn4_slot;
        let mut var_fn462_calc_ig__ffvgin_dn7: f64 = *var_fn462_calc_ig__ffvgin_dn7_slot;
        let mut var_fn462_calc_ig__ffvgin_dn8: f64 = *var_fn462_calc_ig__ffvgin_dn8_slot;
        let mut var_fn462_calc_ig__frecgin: f64 = *var_fn462_calc_ig__frecgin_slot;
        let mut var_fn462_calc_ig__frecgin_dn7: f64 = *var_fn462_calc_ig__frecgin_dn7_slot;
        let mut var_fn462_calc_ig__frecgin_dn8: f64 = *var_fn462_calc_ig__frecgin_dn8_slot;
        let mut var_fn462_calc_ig__iginbd: f64 = *var_fn462_calc_ig__iginbd_slot;
        let mut var_fn462_calc_ig__iginbd_dn4: f64 = *var_fn462_calc_ig__iginbd_dn4_slot;
        let mut var_fn462_calc_ig__iginbd_dn7: f64 = *var_fn462_calc_ig__iginbd_dn7_slot;
        let mut var_fn462_calc_ig__iginbd_dn8: f64 = *var_fn462_calc_ig__iginbd_dn8_slot;
        let mut var_fn462_calc_ig__iginbd_vgsat: f64 = *var_fn462_calc_ig__iginbd_vgsat_slot;
        let mut var_fn462_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn462_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__igindiode: f64 = *var_fn462_calc_ig__igindiode_slot;
        let mut var_fn462_calc_ig__igindiode_dn4: f64 = *var_fn462_calc_ig__igindiode_dn4_slot;
        let mut var_fn462_calc_ig__igindiode_dn7: f64 = *var_fn462_calc_ig__igindiode_dn7_slot;
        let mut var_fn462_calc_ig__igindiode_dn8: f64 = *var_fn462_calc_ig__igindiode_dn8_slot;
        let mut var_fn462_calc_ig__igindiode_hinj: f64 = *var_fn462_calc_ig__igindiode_hinj_slot;
        let mut var_fn462_calc_ig__igindiode_hinj_dn4: f64 = *var_fn462_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn462_calc_ig__igindiode_hinj_dn7: f64 = *var_fn462_calc_ig__igindiode_hinj_dn7_slot;
        let mut var_fn462_calc_ig__igindiode_hinj_dn8: f64 = *var_fn462_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn462_calc_ig__igindiode_hinj_pre: f64 = *var_fn462_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn462_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn462_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn462_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn462_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn462_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn462_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__igindiode_nohinj: f64 = *var_fn462_calc_ig__igindiode_nohinj_slot;
        let mut var_fn462_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn462_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn462_calc_ig__igindiode_nohinj_dn7: f64 = *var_fn462_calc_ig__igindiode_nohinj_dn7_slot;
        let mut var_fn462_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn462_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn462_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn462_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn462_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn462_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__iginrec: f64 = *var_fn462_calc_ig__iginrec_slot;
        let mut var_fn462_calc_ig__iginrec_dn4: f64 = *var_fn462_calc_ig__iginrec_dn4_slot;
        let mut var_fn462_calc_ig__iginrec_dn7: f64 = *var_fn462_calc_ig__iginrec_dn7_slot;
        let mut var_fn462_calc_ig__iginrec_dn8: f64 = *var_fn462_calc_ig__iginrec_dn8_slot;
        let mut var_fn462_calc_ig__igout: f64 = *var_fn462_calc_ig__igout_slot;
        let mut var_fn462_calc_ig__igout_dn4: f64 = *var_fn462_calc_ig__igout_dn4_slot;
        let mut var_fn462_calc_ig__igout_dn7: f64 = *var_fn462_calc_ig__igout_dn7_slot;
        let mut var_fn462_calc_ig__igout_dn8: f64 = *var_fn462_calc_ig__igout_dn8_slot;
        let mut var_fn462_calc_ig__pg_param1: f64 = *var_fn462_calc_ig__pg_param1_slot;
        let mut var_fn462_calc_ig__pg_paramin_hinj: f64 = *var_fn462_calc_ig__pg_paramin_hinj_slot;
        let mut var_fn462_calc_ig__pgsrecin: f64 = *var_fn462_calc_ig__pgsrecin_slot;
        let mut var_fn462_calc_ig__t0: f64 = *var_fn462_calc_ig__t0_slot;
        let mut var_fn462_calc_ig__t0_dn4: f64 = *var_fn462_calc_ig__t0_dn4_slot;
        let mut var_fn462_calc_ig__type: f64 = *var_fn462_calc_ig__type_slot;
        let mut var_fn462_calc_ig__vjg: f64 = *var_fn462_calc_ig__vjg_slot;

        let (assign41740_e39992,) = {
    if (var_guard461 != 0.0) {
        (p.p297,)
    } else {
        (var_fn462_calc_ig__pgsrecin,)
    }
};
        var_fn462_calc_ig__pgsrecin = assign41740_e39992;

        let (assign41750_e39996,) = {
    if (var_guard461 != 0.0) {
        (0.0,)
    } else {
        (var_fn462_calc_ig__pg_param1,)
    }
};
        var_fn462_calc_ig__pg_param1 = assign41750_e39996;

        let (assign41760_e40000,) = {
    if (var_guard461 != 0.0) {
        (0.0,)
    } else {
        (var_fn462_calc_ig__vjg,)
    }
};
        var_fn462_calc_ig__vjg = assign41760_e40000;

        let (assign41770_e40004,) = {
    if (var_guard461 != 0.0) {
        (p.p6,)
    } else {
        (var_fn462_calc_ig__type,)
    }
};
        var_fn462_calc_ig__type = assign41770_e40004;

        let (assign41780_e40008, assign41780_e40008_d_n4, assign41780_e40008_d_n7, assign41780_e40008_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__igout, var_fn462_calc_ig__igout_dn4, var_fn462_calc_ig__igout_dn7, var_fn462_calc_ig__igout_dn8,)
    }
};
        var_fn462_calc_ig__igout = assign41780_e40008;
        var_fn462_calc_ig__igout_dn4 = assign41780_e40008_d_n4;
        var_fn462_calc_ig__igout_dn7 = assign41780_e40008_d_n7;
        var_fn462_calc_ig__igout_dn8 = assign41780_e40008_d_n8;

        let (assign41790_e40012, assign41790_e40012_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__alpha2_phit, var_fn462_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn462_calc_ig__alpha2_phit = assign41790_e40012;
        var_fn462_calc_ig__alpha2_phit_dn4 = assign41790_e40012_d_n4;

        let (assign41800_e40016, assign41800_e40016_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__t0, var_fn462_calc_ig__t0_dn4,)
    }
};
        var_fn462_calc_ig__t0 = assign41800_e40016;
        var_fn462_calc_ig__t0_dn4 = assign41800_e40016_d_n4;

        let (assign41810_e40020, assign41810_e40020_d_n4, assign41810_e40020_d_n7, assign41810_e40020_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__ffvgin, var_fn462_calc_ig__ffvgin_dn4, var_fn462_calc_ig__ffvgin_dn7, var_fn462_calc_ig__ffvgin_dn8,)
    }
};
        var_fn462_calc_ig__ffvgin = assign41810_e40020;
        var_fn462_calc_ig__ffvgin_dn4 = assign41810_e40020_d_n4;
        var_fn462_calc_ig__ffvgin_dn7 = assign41810_e40020_d_n7;
        var_fn462_calc_ig__ffvgin_dn8 = assign41810_e40020_d_n8;

        let (assign41820_e40024, assign41820_e40024_d_n4, assign41820_e40024_d_n7, assign41820_e40024_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__iginbd, var_fn462_calc_ig__iginbd_dn4, var_fn462_calc_ig__iginbd_dn7, var_fn462_calc_ig__iginbd_dn8,)
    }
};
        var_fn462_calc_ig__iginbd = assign41820_e40024;
        var_fn462_calc_ig__iginbd_dn4 = assign41820_e40024_d_n4;
        var_fn462_calc_ig__iginbd_dn7 = assign41820_e40024_d_n7;
        var_fn462_calc_ig__iginbd_dn8 = assign41820_e40024_d_n8;

        let (assign41830_e40028, assign41830_e40028_d_n4, assign41830_e40028_d_n7, assign41830_e40028_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__igindiode, var_fn462_calc_ig__igindiode_dn4, var_fn462_calc_ig__igindiode_dn7, var_fn462_calc_ig__igindiode_dn8,)
    }
};
        var_fn462_calc_ig__igindiode = assign41830_e40028;
        var_fn462_calc_ig__igindiode_dn4 = assign41830_e40028_d_n4;
        var_fn462_calc_ig__igindiode_dn7 = assign41830_e40028_d_n7;
        var_fn462_calc_ig__igindiode_dn8 = assign41830_e40028_d_n8;

        let (assign41840_e40032, assign41840_e40032_d_n7, assign41840_e40032_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__frecgin, var_fn462_calc_ig__frecgin_dn7, var_fn462_calc_ig__frecgin_dn8,)
    }
};
        var_fn462_calc_ig__frecgin = assign41840_e40032;
        var_fn462_calc_ig__frecgin_dn7 = assign41840_e40032_d_n7;
        var_fn462_calc_ig__frecgin_dn8 = assign41840_e40032_d_n8;

        let (assign41850_e40036, assign41850_e40036_d_n4, assign41850_e40036_d_n7, assign41850_e40036_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__iginrec, var_fn462_calc_ig__iginrec_dn4, var_fn462_calc_ig__iginrec_dn7, var_fn462_calc_ig__iginrec_dn8,)
    }
};
        var_fn462_calc_ig__iginrec = assign41850_e40036;
        var_fn462_calc_ig__iginrec_dn4 = assign41850_e40036_d_n4;
        var_fn462_calc_ig__iginrec_dn7 = assign41850_e40036_d_n7;
        var_fn462_calc_ig__iginrec_dn8 = assign41850_e40036_d_n8;

        let (assign41860_e40040, assign41860_e40040_d_n4, assign41860_e40040_d_n7, assign41860_e40040_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expbdarg1, var_fn462_calc_ig__expbdarg1_dn4, var_fn462_calc_ig__expbdarg1_dn7, var_fn462_calc_ig__expbdarg1_dn8,)
    }
};
        var_fn462_calc_ig__expbdarg1 = assign41860_e40040;
        var_fn462_calc_ig__expbdarg1_dn4 = assign41860_e40040_d_n4;
        var_fn462_calc_ig__expbdarg1_dn7 = assign41860_e40040_d_n7;
        var_fn462_calc_ig__expbdarg1_dn8 = assign41860_e40040_d_n8;

        let (assign41870_e40044, assign41870_e40044_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expbdarg2, var_fn462_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn462_calc_ig__expbdarg2 = assign41870_e40044;
        var_fn462_calc_ig__expbdarg2_dn4 = assign41870_e40044_d_n4;

        let (assign41880_e40048, assign41880_e40048_d_n4, assign41880_e40048_d_n7, assign41880_e40048_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expbd1, var_fn462_calc_ig__expbd1_dn4, var_fn462_calc_ig__expbd1_dn7, var_fn462_calc_ig__expbd1_dn8,)
    }
};
        var_fn462_calc_ig__expbd1 = assign41880_e40048;
        var_fn462_calc_ig__expbd1_dn4 = assign41880_e40048_d_n4;
        var_fn462_calc_ig__expbd1_dn7 = assign41880_e40048_d_n7;
        var_fn462_calc_ig__expbd1_dn8 = assign41880_e40048_d_n8;

        let (assign41890_e40052, assign41890_e40052_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expbd2, var_fn462_calc_ig__expbd2_dn4,)
    }
};
        var_fn462_calc_ig__expbd2 = assign41890_e40052;
        var_fn462_calc_ig__expbd2_dn4 = assign41890_e40052_d_n4;

        let (assign41900_e40056, assign41900_e40056_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expphib, var_fn462_calc_ig__expphib_dn4,)
    }
};
        var_fn462_calc_ig__expphib = assign41900_e40056;
        var_fn462_calc_ig__expphib_dn4 = assign41900_e40056_d_n4;

        let (assign41910_e40060, assign41910_e40060_d_n4, assign41910_e40060_d_n7, assign41910_e40060_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expffvarg, var_fn462_calc_ig__expffvarg_dn4, var_fn462_calc_ig__expffvarg_dn7, var_fn462_calc_ig__expffvarg_dn8,)
    }
};
        var_fn462_calc_ig__expffvarg = assign41910_e40060;
        var_fn462_calc_ig__expffvarg_dn4 = assign41910_e40060_d_n4;
        var_fn462_calc_ig__expffvarg_dn7 = assign41910_e40060_d_n7;
        var_fn462_calc_ig__expffvarg_dn8 = assign41910_e40060_d_n8;

        let (assign41920_e40064, assign41920_e40064_d_n4, assign41920_e40064_d_n7, assign41920_e40064_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expiforarg, var_fn462_calc_ig__expiforarg_dn4, var_fn462_calc_ig__expiforarg_dn7, var_fn462_calc_ig__expiforarg_dn8,)
    }
};
        var_fn462_calc_ig__expiforarg = assign41920_e40064;
        var_fn462_calc_ig__expiforarg_dn4 = assign41920_e40064_d_n4;
        var_fn462_calc_ig__expiforarg_dn7 = assign41920_e40064_d_n7;
        var_fn462_calc_ig__expiforarg_dn8 = assign41920_e40064_d_n8;

        let (assign41930_e40068, assign41930_e40068_d_n4, assign41930_e40068_d_n7, assign41930_e40068_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expifor, var_fn462_calc_ig__expifor_dn4, var_fn462_calc_ig__expifor_dn7, var_fn462_calc_ig__expifor_dn8,)
    }
};
        var_fn462_calc_ig__expifor = assign41930_e40068;
        var_fn462_calc_ig__expifor_dn4 = assign41930_e40068_d_n4;
        var_fn462_calc_ig__expifor_dn7 = assign41930_e40068_d_n7;
        var_fn462_calc_ig__expifor_dn8 = assign41930_e40068_d_n8;

        let (assign41940_e40072, assign41940_e40072_d_n4, assign41940_e40072_d_n7, assign41940_e40072_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expirevarg, var_fn462_calc_ig__expirevarg_dn4, var_fn462_calc_ig__expirevarg_dn7, var_fn462_calc_ig__expirevarg_dn8,)
    }
};
        var_fn462_calc_ig__expirevarg = assign41940_e40072;
        var_fn462_calc_ig__expirevarg_dn4 = assign41940_e40072_d_n4;
        var_fn462_calc_ig__expirevarg_dn7 = assign41940_e40072_d_n7;
        var_fn462_calc_ig__expirevarg_dn8 = assign41940_e40072_d_n8;

        let (assign41950_e40076, assign41950_e40076_d_n4, assign41950_e40076_d_n7, assign41950_e40076_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expirev, var_fn462_calc_ig__expirev_dn4, var_fn462_calc_ig__expirev_dn7, var_fn462_calc_ig__expirev_dn8,)
    }
};
        var_fn462_calc_ig__expirev = assign41950_e40076;
        var_fn462_calc_ig__expirev_dn4 = assign41950_e40076_d_n4;
        var_fn462_calc_ig__expirev_dn7 = assign41950_e40076_d_n7;
        var_fn462_calc_ig__expirev_dn8 = assign41950_e40076_d_n8;

        let (assign41960_e40080,) = {
    if (var_guard461 != 0.0) {
        (0.0,)
    } else {
        (var_fn462_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn462_calc_ig__pg_paramin_hinj = assign41960_e40080;

        let (assign41970_e40084, assign41970_e40084_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expbdarg1_vgsat, var_fn462_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__expbdarg1_vgsat = assign41970_e40084;
        var_fn462_calc_ig__expbdarg1_vgsat_dn4 = assign41970_e40084_d_n4;

        let (assign41980_e40088, assign41980_e40088_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expbd1_vgsat, var_fn462_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__expbd1_vgsat = assign41980_e40088;
        var_fn462_calc_ig__expbd1_vgsat_dn4 = assign41980_e40088_d_n4;

        let (assign41990_e40092, assign41990_e40092_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__iginbd_vgsat, var_fn462_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__iginbd_vgsat = assign41990_e40092;
        var_fn462_calc_ig__iginbd_vgsat_dn4 = assign41990_e40092_d_n4;

        let (assign42000_e40096, assign42000_e40096_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expiforarg_nohinj_vgsat, var_fn462_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__expiforarg_nohinj_vgsat = assign42000_e40096;
        var_fn462_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign42000_e40096_d_n4;

        let (assign42010_e40100, assign42010_e40100_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expifor_nohinj_vgsat, var_fn462_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__expifor_nohinj_vgsat = assign42010_e40100;
        var_fn462_calc_ig__expifor_nohinj_vgsat_dn4 = assign42010_e40100_d_n4;

        let (assign42020_e40104, assign42020_e40104_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__igindiode_nohinj_vgsat, var_fn462_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__igindiode_nohinj_vgsat = assign42020_e40104;
        var_fn462_calc_ig__igindiode_nohinj_vgsat_dn4 = assign42020_e40104_d_n4;

        let (assign42030_e40108, assign42030_e40108_d_n4, assign42030_e40108_d_n7, assign42030_e40108_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__igindiode_nohinj, var_fn462_calc_ig__igindiode_nohinj_dn4, var_fn462_calc_ig__igindiode_nohinj_dn7, var_fn462_calc_ig__igindiode_nohinj_dn8,)
    }
};
        var_fn462_calc_ig__igindiode_nohinj = assign42030_e40108;
        var_fn462_calc_ig__igindiode_nohinj_dn4 = assign42030_e40108_d_n4;
        var_fn462_calc_ig__igindiode_nohinj_dn7 = assign42030_e40108_d_n7;
        var_fn462_calc_ig__igindiode_nohinj_dn8 = assign42030_e40108_d_n8;

        let (assign42040_e40112, assign42040_e40112_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expiforarg_hinj_vgsat, var_fn462_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__expiforarg_hinj_vgsat = assign42040_e40112;
        var_fn462_calc_ig__expiforarg_hinj_vgsat_dn4 = assign42040_e40112_d_n4;

        let (assign42050_e40116, assign42050_e40116_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expifor_hinj_vgsat, var_fn462_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__expifor_hinj_vgsat = assign42050_e40116;
        var_fn462_calc_ig__expifor_hinj_vgsat_dn4 = assign42050_e40116_d_n4;

        let (assign42060_e40120, assign42060_e40120_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__igindiode_hinj_vgsat, var_fn462_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__igindiode_hinj_vgsat = assign42060_e40120;
        var_fn462_calc_ig__igindiode_hinj_vgsat_dn4 = assign42060_e40120_d_n4;

        let (assign42070_e40124, assign42070_e40124_d_n4, assign42070_e40124_d_n7, assign42070_e40124_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expiforarg_hinj, var_fn462_calc_ig__expiforarg_hinj_dn4, var_fn462_calc_ig__expiforarg_hinj_dn7, var_fn462_calc_ig__expiforarg_hinj_dn8,)
    }
};
        var_fn462_calc_ig__expiforarg_hinj = assign42070_e40124;
        var_fn462_calc_ig__expiforarg_hinj_dn4 = assign42070_e40124_d_n4;
        var_fn462_calc_ig__expiforarg_hinj_dn7 = assign42070_e40124_d_n7;
        var_fn462_calc_ig__expiforarg_hinj_dn8 = assign42070_e40124_d_n8;

        let (assign42080_e40128, assign42080_e40128_d_n4, assign42080_e40128_d_n7, assign42080_e40128_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__expifor_hinj, var_fn462_calc_ig__expifor_hinj_dn4, var_fn462_calc_ig__expifor_hinj_dn7, var_fn462_calc_ig__expifor_hinj_dn8,)
    }
};
        var_fn462_calc_ig__expifor_hinj = assign42080_e40128;
        var_fn462_calc_ig__expifor_hinj_dn4 = assign42080_e40128_d_n4;
        var_fn462_calc_ig__expifor_hinj_dn7 = assign42080_e40128_d_n7;
        var_fn462_calc_ig__expifor_hinj_dn8 = assign42080_e40128_d_n8;

        let (assign42090_e40132, assign42090_e40132_d_n4,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__igindiode_hinj_pre, var_fn462_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn462_calc_ig__igindiode_hinj_pre = assign42090_e40132;
        var_fn462_calc_ig__igindiode_hinj_pre_dn4 = assign42090_e40132_d_n4;

        let (assign42100_e40136, assign42100_e40136_d_n4, assign42100_e40136_d_n7, assign42100_e40136_d_n8,) = {
    if (var_guard461 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__igindiode_hinj, var_fn462_calc_ig__igindiode_hinj_dn4, var_fn462_calc_ig__igindiode_hinj_dn7, var_fn462_calc_ig__igindiode_hinj_dn8,)
    }
};
        var_fn462_calc_ig__igindiode_hinj = assign42100_e40136;
        var_fn462_calc_ig__igindiode_hinj_dn4 = assign42100_e40136_d_n4;
        var_fn462_calc_ig__igindiode_hinj_dn7 = assign42100_e40136_d_n7;
        var_fn462_calc_ig__igindiode_hinj_dn8 = assign42100_e40136_d_n8;

        let (assign42110_e40145, assign42110_e40145_d_n4,) = {
    if (var_guard461 != 0.0) {
        let assign42110_e40140: f64 = (var_fn462_calc_ig__pg_param1 / var_fn462_calc_ig__phitin);
        let assign42110_e40142: f64 = (-var_fn462_calc_ig__vjg);
        let assign42110_e40143: f64 = (assign42110_e40140 * assign42110_e40142);
        (assign42110_e40143, ((-((var_fn462_calc_ig__pg_param1 * var_fn462_calc_ig__phitin_dn4) / (var_fn462_calc_ig__phitin * var_fn462_calc_ig__phitin))) * assign42110_e40142),)
    } else {
        (var_fn462_calc_ig__expphib, var_fn462_calc_ig__expphib_dn4,)
    }
};
        var_fn462_calc_ig__expphib = assign42110_e40145;
        var_fn462_calc_ig__expphib_dn4 = assign42110_e40145_d_n4;

        let (assign42120_e40187, assign42120_e40187_d_n4,) = {
    if (var_guard461 != 0.0) {
        let assign42120_e40153: f64 = (-50.0);
        let (assign42120_e40185, assign42120_e40185_d_n4,) = {
            if ((!(var_fn462_calc_ig__expphib > 50.0)) && (!(var_fn462_calc_ig__expphib < assign42120_e40153))) {
                let assign42120_e40158: f64 = (var_fn462_calc_ig__expphib).exp();
                (assign42120_e40158, (assign42120_e40158 * var_fn462_calc_ig__expphib_dn4),)
            } else {
                let assign42120_e40165: f64 = (-50.0);
                let (assign42120_e40184, assign42120_e40184_d_n4,) = {
                    if ((!(var_fn462_calc_ig__expphib > 50.0)) && (var_fn462_calc_ig__expphib < assign42120_e40165)) {
                        let assign42120_e40169: f64 = (-50.0);
                        let assign42120_e40170: f64 = (assign42120_e40169).exp();
                        (assign42120_e40170, 0.0,)
                    } else {
                        let (assign42120_e40183, assign42120_e40183_d_n4,) = {
                            if (var_fn462_calc_ig__expphib > 50.0) {
                                let assign42120_e40175: f64 = (50.0_f64).exp();
                                let assign42120_e40179: f64 = (var_fn462_calc_ig__expphib - 50.0);
                                let assign42120_e40180: f64 = (1.0 + assign42120_e40179);
                                let assign42120_e40181: f64 = (assign42120_e40175 * assign42120_e40180);
                                (assign42120_e40181, (assign42120_e40175 * var_fn462_calc_ig__expphib_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign42120_e40183, assign42120_e40183_d_n4,)
                    }
                };
                (assign42120_e40184, assign42120_e40184_d_n4,)
            }
        };
        (assign42120_e40185, assign42120_e40185_d_n4,)
    } else {
        (var_fn462_calc_ig__t0, var_fn462_calc_ig__t0_dn4,)
    }
};
        var_fn462_calc_ig__t0 = assign42120_e40187;
        var_fn462_calc_ig__t0_dn4 = assign42120_e40187_d_n4;

        let (assign42130_e40198, assign42130_e40198_d_n4, assign42130_e40198_d_n7, assign42130_e40198_d_n8,) = {
    if (var_guard461 != 0.0) {
        let assign42130_e40191: f64 = (-var_fn462_calc_ig__vgin);
        let assign42130_e40193: f64 = (assign42130_e40191 - var_fn462_calc_ig__vbdgin);
        let assign42130_e40194: f64 = (var_fn462_calc_ig__pbdgin * assign42130_e40193);
        let assign42130_e40196: f64 = (assign42130_e40194 + var_fn462_calc_ig__expphib);
        (assign42130_e40196, var_fn462_calc_ig__expphib_dn4, (var_fn462_calc_ig__pbdgin * (-var_fn462_calc_ig__vgin_dn7)), (var_fn462_calc_ig__pbdgin * (-var_fn462_calc_ig__vgin_dn8)),)
    } else {
        (var_fn462_calc_ig__expbdarg1, var_fn462_calc_ig__expbdarg1_dn4, var_fn462_calc_ig__expbdarg1_dn7, var_fn462_calc_ig__expbdarg1_dn8,)
    }
};
        var_fn462_calc_ig__expbdarg1 = assign42130_e40198;
        var_fn462_calc_ig__expbdarg1_dn4 = assign42130_e40198_d_n4;
        var_fn462_calc_ig__expbdarg1_dn7 = assign42130_e40198_d_n7;
        var_fn462_calc_ig__expbdarg1_dn8 = assign42130_e40198_d_n8;

        let (assign42140_e40207, assign42140_e40207_d_n4,) = {
    if (var_guard461 != 0.0) {
        let assign42140_e40201: f64 = (-var_fn462_calc_ig__pbdgin);
        let assign42140_e40203: f64 = (assign42140_e40201 * var_fn462_calc_ig__vbdgin);
        let assign42140_e40205: f64 = (assign42140_e40203 + var_fn462_calc_ig__expphib);
        (assign42140_e40205, var_fn462_calc_ig__expphib_dn4,)
    } else {
        (var_fn462_calc_ig__expbdarg2, var_fn462_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn462_calc_ig__expbdarg2 = assign42140_e40207;
        var_fn462_calc_ig__expbdarg2_dn4 = assign42140_e40207_d_n4;

        *var_fn462_calc_ig__alpha2_phit_slot = var_fn462_calc_ig__alpha2_phit;
        *var_fn462_calc_ig__alpha2_phit_dn4_slot = var_fn462_calc_ig__alpha2_phit_dn4;
        *var_fn462_calc_ig__expbd1_slot = var_fn462_calc_ig__expbd1;
        *var_fn462_calc_ig__expbd1_dn4_slot = var_fn462_calc_ig__expbd1_dn4;
        *var_fn462_calc_ig__expbd1_dn7_slot = var_fn462_calc_ig__expbd1_dn7;
        *var_fn462_calc_ig__expbd1_dn8_slot = var_fn462_calc_ig__expbd1_dn8;
        *var_fn462_calc_ig__expbd1_vgsat_slot = var_fn462_calc_ig__expbd1_vgsat;
        *var_fn462_calc_ig__expbd1_vgsat_dn4_slot = var_fn462_calc_ig__expbd1_vgsat_dn4;
        *var_fn462_calc_ig__expbd2_slot = var_fn462_calc_ig__expbd2;
        *var_fn462_calc_ig__expbd2_dn4_slot = var_fn462_calc_ig__expbd2_dn4;
        *var_fn462_calc_ig__expbdarg1_slot = var_fn462_calc_ig__expbdarg1;
        *var_fn462_calc_ig__expbdarg1_dn4_slot = var_fn462_calc_ig__expbdarg1_dn4;
        *var_fn462_calc_ig__expbdarg1_dn7_slot = var_fn462_calc_ig__expbdarg1_dn7;
        *var_fn462_calc_ig__expbdarg1_dn8_slot = var_fn462_calc_ig__expbdarg1_dn8;
        *var_fn462_calc_ig__expbdarg1_vgsat_slot = var_fn462_calc_ig__expbdarg1_vgsat;
        *var_fn462_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn462_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn462_calc_ig__expbdarg2_slot = var_fn462_calc_ig__expbdarg2;
        *var_fn462_calc_ig__expbdarg2_dn4_slot = var_fn462_calc_ig__expbdarg2_dn4;
        *var_fn462_calc_ig__expffvarg_slot = var_fn462_calc_ig__expffvarg;
        *var_fn462_calc_ig__expffvarg_dn4_slot = var_fn462_calc_ig__expffvarg_dn4;
        *var_fn462_calc_ig__expffvarg_dn7_slot = var_fn462_calc_ig__expffvarg_dn7;
        *var_fn462_calc_ig__expffvarg_dn8_slot = var_fn462_calc_ig__expffvarg_dn8;
        *var_fn462_calc_ig__expifor_slot = var_fn462_calc_ig__expifor;
        *var_fn462_calc_ig__expifor_dn4_slot = var_fn462_calc_ig__expifor_dn4;
        *var_fn462_calc_ig__expifor_dn7_slot = var_fn462_calc_ig__expifor_dn7;
        *var_fn462_calc_ig__expifor_dn8_slot = var_fn462_calc_ig__expifor_dn8;
        *var_fn462_calc_ig__expifor_hinj_slot = var_fn462_calc_ig__expifor_hinj;
        *var_fn462_calc_ig__expifor_hinj_dn4_slot = var_fn462_calc_ig__expifor_hinj_dn4;
        *var_fn462_calc_ig__expifor_hinj_dn7_slot = var_fn462_calc_ig__expifor_hinj_dn7;
        *var_fn462_calc_ig__expifor_hinj_dn8_slot = var_fn462_calc_ig__expifor_hinj_dn8;
        *var_fn462_calc_ig__expifor_hinj_vgsat_slot = var_fn462_calc_ig__expifor_hinj_vgsat;
        *var_fn462_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn462_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn462_calc_ig__expifor_nohinj_vgsat_slot = var_fn462_calc_ig__expifor_nohinj_vgsat;
        *var_fn462_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn462_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn462_calc_ig__expiforarg_slot = var_fn462_calc_ig__expiforarg;
        *var_fn462_calc_ig__expiforarg_dn4_slot = var_fn462_calc_ig__expiforarg_dn4;
        *var_fn462_calc_ig__expiforarg_dn7_slot = var_fn462_calc_ig__expiforarg_dn7;
        *var_fn462_calc_ig__expiforarg_dn8_slot = var_fn462_calc_ig__expiforarg_dn8;
        *var_fn462_calc_ig__expiforarg_hinj_slot = var_fn462_calc_ig__expiforarg_hinj;
        *var_fn462_calc_ig__expiforarg_hinj_dn4_slot = var_fn462_calc_ig__expiforarg_hinj_dn4;
        *var_fn462_calc_ig__expiforarg_hinj_dn7_slot = var_fn462_calc_ig__expiforarg_hinj_dn7;
        *var_fn462_calc_ig__expiforarg_hinj_dn8_slot = var_fn462_calc_ig__expiforarg_hinj_dn8;
        *var_fn462_calc_ig__expiforarg_hinj_vgsat_slot = var_fn462_calc_ig__expiforarg_hinj_vgsat;
        *var_fn462_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn462_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn462_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn462_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn462_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn462_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn462_calc_ig__expirev_slot = var_fn462_calc_ig__expirev;
        *var_fn462_calc_ig__expirev_dn4_slot = var_fn462_calc_ig__expirev_dn4;
        *var_fn462_calc_ig__expirev_dn7_slot = var_fn462_calc_ig__expirev_dn7;
        *var_fn462_calc_ig__expirev_dn8_slot = var_fn462_calc_ig__expirev_dn8;
        *var_fn462_calc_ig__expirevarg_slot = var_fn462_calc_ig__expirevarg;
        *var_fn462_calc_ig__expirevarg_dn4_slot = var_fn462_calc_ig__expirevarg_dn4;
        *var_fn462_calc_ig__expirevarg_dn7_slot = var_fn462_calc_ig__expirevarg_dn7;
        *var_fn462_calc_ig__expirevarg_dn8_slot = var_fn462_calc_ig__expirevarg_dn8;
        *var_fn462_calc_ig__expphib_slot = var_fn462_calc_ig__expphib;
        *var_fn462_calc_ig__expphib_dn4_slot = var_fn462_calc_ig__expphib_dn4;
        *var_fn462_calc_ig__ffvgin_slot = var_fn462_calc_ig__ffvgin;
        *var_fn462_calc_ig__ffvgin_dn4_slot = var_fn462_calc_ig__ffvgin_dn4;
        *var_fn462_calc_ig__ffvgin_dn7_slot = var_fn462_calc_ig__ffvgin_dn7;
        *var_fn462_calc_ig__ffvgin_dn8_slot = var_fn462_calc_ig__ffvgin_dn8;
        *var_fn462_calc_ig__frecgin_slot = var_fn462_calc_ig__frecgin;
        *var_fn462_calc_ig__frecgin_dn7_slot = var_fn462_calc_ig__frecgin_dn7;
        *var_fn462_calc_ig__frecgin_dn8_slot = var_fn462_calc_ig__frecgin_dn8;
        *var_fn462_calc_ig__iginbd_slot = var_fn462_calc_ig__iginbd;
        *var_fn462_calc_ig__iginbd_dn4_slot = var_fn462_calc_ig__iginbd_dn4;
        *var_fn462_calc_ig__iginbd_dn7_slot = var_fn462_calc_ig__iginbd_dn7;
        *var_fn462_calc_ig__iginbd_dn8_slot = var_fn462_calc_ig__iginbd_dn8;
        *var_fn462_calc_ig__iginbd_vgsat_slot = var_fn462_calc_ig__iginbd_vgsat;
        *var_fn462_calc_ig__iginbd_vgsat_dn4_slot = var_fn462_calc_ig__iginbd_vgsat_dn4;
        *var_fn462_calc_ig__igindiode_slot = var_fn462_calc_ig__igindiode;
        *var_fn462_calc_ig__igindiode_dn4_slot = var_fn462_calc_ig__igindiode_dn4;
        *var_fn462_calc_ig__igindiode_dn7_slot = var_fn462_calc_ig__igindiode_dn7;
        *var_fn462_calc_ig__igindiode_dn8_slot = var_fn462_calc_ig__igindiode_dn8;
        *var_fn462_calc_ig__igindiode_hinj_slot = var_fn462_calc_ig__igindiode_hinj;
        *var_fn462_calc_ig__igindiode_hinj_dn4_slot = var_fn462_calc_ig__igindiode_hinj_dn4;
        *var_fn462_calc_ig__igindiode_hinj_dn7_slot = var_fn462_calc_ig__igindiode_hinj_dn7;
        *var_fn462_calc_ig__igindiode_hinj_dn8_slot = var_fn462_calc_ig__igindiode_hinj_dn8;
        *var_fn462_calc_ig__igindiode_hinj_pre_slot = var_fn462_calc_ig__igindiode_hinj_pre;
        *var_fn462_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn462_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn462_calc_ig__igindiode_hinj_vgsat_slot = var_fn462_calc_ig__igindiode_hinj_vgsat;
        *var_fn462_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn462_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn462_calc_ig__igindiode_nohinj_slot = var_fn462_calc_ig__igindiode_nohinj;
        *var_fn462_calc_ig__igindiode_nohinj_dn4_slot = var_fn462_calc_ig__igindiode_nohinj_dn4;
        *var_fn462_calc_ig__igindiode_nohinj_dn7_slot = var_fn462_calc_ig__igindiode_nohinj_dn7;
        *var_fn462_calc_ig__igindiode_nohinj_dn8_slot = var_fn462_calc_ig__igindiode_nohinj_dn8;
        *var_fn462_calc_ig__igindiode_nohinj_vgsat_slot = var_fn462_calc_ig__igindiode_nohinj_vgsat;
        *var_fn462_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn462_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn462_calc_ig__iginrec_slot = var_fn462_calc_ig__iginrec;
        *var_fn462_calc_ig__iginrec_dn4_slot = var_fn462_calc_ig__iginrec_dn4;
        *var_fn462_calc_ig__iginrec_dn7_slot = var_fn462_calc_ig__iginrec_dn7;
        *var_fn462_calc_ig__iginrec_dn8_slot = var_fn462_calc_ig__iginrec_dn8;
        *var_fn462_calc_ig__igout_slot = var_fn462_calc_ig__igout;
        *var_fn462_calc_ig__igout_dn4_slot = var_fn462_calc_ig__igout_dn4;
        *var_fn462_calc_ig__igout_dn7_slot = var_fn462_calc_ig__igout_dn7;
        *var_fn462_calc_ig__igout_dn8_slot = var_fn462_calc_ig__igout_dn8;
        *var_fn462_calc_ig__pg_param1_slot = var_fn462_calc_ig__pg_param1;
        *var_fn462_calc_ig__pg_paramin_hinj_slot = var_fn462_calc_ig__pg_paramin_hinj;
        *var_fn462_calc_ig__pgsrecin_slot = var_fn462_calc_ig__pgsrecin;
        *var_fn462_calc_ig__t0_slot = var_fn462_calc_ig__t0;
        *var_fn462_calc_ig__t0_dn4_slot = var_fn462_calc_ig__t0_dn4;
        *var_fn462_calc_ig__type_slot = var_fn462_calc_ig__type;
        *var_fn462_calc_ig__vjg_slot = var_fn462_calc_ig__vjg;
    }

    pub(super) fn stamp_transient_block_105(
        var_fn462_calc_ig__expbdarg1: f64,
        var_fn462_calc_ig__expbdarg1_dn4: f64,
        var_fn462_calc_ig__expbdarg1_dn7: f64,
        var_fn462_calc_ig__expbdarg1_dn8: f64,
        var_fn462_calc_ig__expbdarg2: f64,
        var_fn462_calc_ig__expbdarg2_dn4: f64,
        var_fn462_calc_ig__expphib: f64,
        var_fn462_calc_ig__expphib_dn4: f64,
        var_fn462_calc_ig__fracin: f64,
        var_fn462_calc_ig__ijin: f64,
        var_fn462_calc_ig__kbdgatein: f64,
        var_fn462_calc_ig__ngf: f64,
        var_fn462_calc_ig__pbdgin: f64,
        var_fn462_calc_ig__pg_paramin: f64,
        var_fn462_calc_ig__phitin: f64,
        var_fn462_calc_ig__phitin_dn4: f64,
        var_fn462_calc_ig__t0: f64,
        var_fn462_calc_ig__t0_dn4: f64,
        var_fn462_calc_ig__tfacdiodein: f64,
        var_fn462_calc_ig__tfacdiodein_dn4: f64,
        var_fn462_calc_ig__type: f64,
        var_fn462_calc_ig__vbdgin: f64,
        var_fn462_calc_ig__vgin: f64,
        var_fn462_calc_ig__vgin_dn7: f64,
        var_fn462_calc_ig__vgin_dn8: f64,
        var_fn462_calc_ig__vgsatin: f64,
        var_fn462_calc_ig__w: f64,
        var_guard461: f64,
        var_fn462_calc_ig__expbd1_slot: &mut f64,
        var_fn462_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn462_calc_ig__expbd1_dn7_slot: &mut f64,
        var_fn462_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn462_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn462_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__expbd2_slot: &mut f64,
        var_fn462_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn462_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn462_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__expifor_slot: &mut f64,
        var_fn462_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn462_calc_ig__expifor_dn7_slot: &mut f64,
        var_fn462_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn462_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn462_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn462_calc_ig__expifor_hinj_dn7_slot: &mut f64,
        var_fn462_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn462_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn462_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn462_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_dn7_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_hinj_dn7_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn462_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__iginbd_slot: &mut f64,
        var_fn462_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn462_calc_ig__iginbd_dn7_slot: &mut f64,
        var_fn462_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn462_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn462_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__igindiode_slot: &mut f64,
        var_fn462_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn462_calc_ig__igindiode_dn7_slot: &mut f64,
        var_fn462_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn462_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn462_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn462_calc_ig__igindiode_hinj_dn7_slot: &mut f64,
        var_fn462_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn462_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn462_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn462_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn462_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn462_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn462_calc_ig__igindiode_nohinj_dn7_slot: &mut f64,
        var_fn462_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn462_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn462_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn462_calc_ig__isdiodeout_slot: &mut f64,
        var_fn462_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn462_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_guard463_slot: &mut f64,
        var_guard464_slot: &mut f64,
    ) {
        let mut var_fn462_calc_ig__expbd1: f64 = *var_fn462_calc_ig__expbd1_slot;
        let mut var_fn462_calc_ig__expbd1_dn4: f64 = *var_fn462_calc_ig__expbd1_dn4_slot;
        let mut var_fn462_calc_ig__expbd1_dn7: f64 = *var_fn462_calc_ig__expbd1_dn7_slot;
        let mut var_fn462_calc_ig__expbd1_dn8: f64 = *var_fn462_calc_ig__expbd1_dn8_slot;
        let mut var_fn462_calc_ig__expbd1_vgsat: f64 = *var_fn462_calc_ig__expbd1_vgsat_slot;
        let mut var_fn462_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn462_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__expbd2: f64 = *var_fn462_calc_ig__expbd2_slot;
        let mut var_fn462_calc_ig__expbd2_dn4: f64 = *var_fn462_calc_ig__expbd2_dn4_slot;
        let mut var_fn462_calc_ig__expbdarg1_vgsat: f64 = *var_fn462_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn462_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn462_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__expifor: f64 = *var_fn462_calc_ig__expifor_slot;
        let mut var_fn462_calc_ig__expifor_dn4: f64 = *var_fn462_calc_ig__expifor_dn4_slot;
        let mut var_fn462_calc_ig__expifor_dn7: f64 = *var_fn462_calc_ig__expifor_dn7_slot;
        let mut var_fn462_calc_ig__expifor_dn8: f64 = *var_fn462_calc_ig__expifor_dn8_slot;
        let mut var_fn462_calc_ig__expifor_hinj: f64 = *var_fn462_calc_ig__expifor_hinj_slot;
        let mut var_fn462_calc_ig__expifor_hinj_dn4: f64 = *var_fn462_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn462_calc_ig__expifor_hinj_dn7: f64 = *var_fn462_calc_ig__expifor_hinj_dn7_slot;
        let mut var_fn462_calc_ig__expifor_hinj_dn8: f64 = *var_fn462_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn462_calc_ig__expifor_hinj_vgsat: f64 = *var_fn462_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn462_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn462_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn462_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn462_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn462_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__expiforarg: f64 = *var_fn462_calc_ig__expiforarg_slot;
        let mut var_fn462_calc_ig__expiforarg_dn4: f64 = *var_fn462_calc_ig__expiforarg_dn4_slot;
        let mut var_fn462_calc_ig__expiforarg_dn7: f64 = *var_fn462_calc_ig__expiforarg_dn7_slot;
        let mut var_fn462_calc_ig__expiforarg_dn8: f64 = *var_fn462_calc_ig__expiforarg_dn8_slot;
        let mut var_fn462_calc_ig__expiforarg_hinj: f64 = *var_fn462_calc_ig__expiforarg_hinj_slot;
        let mut var_fn462_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn462_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn462_calc_ig__expiforarg_hinj_dn7: f64 = *var_fn462_calc_ig__expiforarg_hinj_dn7_slot;
        let mut var_fn462_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn462_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn462_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn462_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn462_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn462_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn462_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn462_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn462_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__iginbd: f64 = *var_fn462_calc_ig__iginbd_slot;
        let mut var_fn462_calc_ig__iginbd_dn4: f64 = *var_fn462_calc_ig__iginbd_dn4_slot;
        let mut var_fn462_calc_ig__iginbd_dn7: f64 = *var_fn462_calc_ig__iginbd_dn7_slot;
        let mut var_fn462_calc_ig__iginbd_dn8: f64 = *var_fn462_calc_ig__iginbd_dn8_slot;
        let mut var_fn462_calc_ig__iginbd_vgsat: f64 = *var_fn462_calc_ig__iginbd_vgsat_slot;
        let mut var_fn462_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn462_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__igindiode: f64 = *var_fn462_calc_ig__igindiode_slot;
        let mut var_fn462_calc_ig__igindiode_dn4: f64 = *var_fn462_calc_ig__igindiode_dn4_slot;
        let mut var_fn462_calc_ig__igindiode_dn7: f64 = *var_fn462_calc_ig__igindiode_dn7_slot;
        let mut var_fn462_calc_ig__igindiode_dn8: f64 = *var_fn462_calc_ig__igindiode_dn8_slot;
        let mut var_fn462_calc_ig__igindiode_hinj: f64 = *var_fn462_calc_ig__igindiode_hinj_slot;
        let mut var_fn462_calc_ig__igindiode_hinj_dn4: f64 = *var_fn462_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn462_calc_ig__igindiode_hinj_dn7: f64 = *var_fn462_calc_ig__igindiode_hinj_dn7_slot;
        let mut var_fn462_calc_ig__igindiode_hinj_dn8: f64 = *var_fn462_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn462_calc_ig__igindiode_hinj_pre: f64 = *var_fn462_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn462_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn462_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn462_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn462_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn462_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn462_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__igindiode_nohinj: f64 = *var_fn462_calc_ig__igindiode_nohinj_slot;
        let mut var_fn462_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn462_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn462_calc_ig__igindiode_nohinj_dn7: f64 = *var_fn462_calc_ig__igindiode_nohinj_dn7_slot;
        let mut var_fn462_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn462_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn462_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn462_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn462_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn462_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn462_calc_ig__isdiodeout: f64 = *var_fn462_calc_ig__isdiodeout_slot;
        let mut var_fn462_calc_ig__isdiodeout_dn4: f64 = *var_fn462_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn462_calc_ig__pg_paramin_hinj: f64 = *var_fn462_calc_ig__pg_paramin_hinj_slot;
        let mut var_guard463: f64 = *var_guard463_slot;
        let mut var_guard464: f64 = *var_guard464_slot;

        let (assign42150_e40249, assign42150_e40249_d_n4, assign42150_e40249_d_n7, assign42150_e40249_d_n8,) = {
    if (var_guard461 != 0.0) {
        let assign42150_e40215: f64 = (-50.0);
        let (assign42150_e40247, assign42150_e40247_d_n4, assign42150_e40247_d_n7, assign42150_e40247_d_n8,) = {
            if ((!(var_fn462_calc_ig__expbdarg1 > 50.0)) && (!(var_fn462_calc_ig__expbdarg1 < assign42150_e40215))) {
                let assign42150_e40220: f64 = (var_fn462_calc_ig__expbdarg1).exp();
                (assign42150_e40220, (assign42150_e40220 * var_fn462_calc_ig__expbdarg1_dn4), (assign42150_e40220 * var_fn462_calc_ig__expbdarg1_dn7), (assign42150_e40220 * var_fn462_calc_ig__expbdarg1_dn8),)
            } else {
                let assign42150_e40227: f64 = (-50.0);
                let (assign42150_e40246, assign42150_e40246_d_n4, assign42150_e40246_d_n7, assign42150_e40246_d_n8,) = {
                    if ((!(var_fn462_calc_ig__expbdarg1 > 50.0)) && (var_fn462_calc_ig__expbdarg1 < assign42150_e40227)) {
                        let assign42150_e40231: f64 = (-50.0);
                        let assign42150_e40232: f64 = (assign42150_e40231).exp();
                        (assign42150_e40232, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign42150_e40245, assign42150_e40245_d_n4, assign42150_e40245_d_n7, assign42150_e40245_d_n8,) = {
                            if (var_fn462_calc_ig__expbdarg1 > 50.0) {
                                let assign42150_e40237: f64 = (50.0_f64).exp();
                                let assign42150_e40241: f64 = (var_fn462_calc_ig__expbdarg1 - 50.0);
                                let assign42150_e40242: f64 = (1.0 + assign42150_e40241);
                                let assign42150_e40243: f64 = (assign42150_e40237 * assign42150_e40242);
                                (assign42150_e40243, (assign42150_e40237 * var_fn462_calc_ig__expbdarg1_dn4), (assign42150_e40237 * var_fn462_calc_ig__expbdarg1_dn7), (assign42150_e40237 * var_fn462_calc_ig__expbdarg1_dn8),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign42150_e40245, assign42150_e40245_d_n4, assign42150_e40245_d_n7, assign42150_e40245_d_n8,)
                    }
                };
                (assign42150_e40246, assign42150_e40246_d_n4, assign42150_e40246_d_n7, assign42150_e40246_d_n8,)
            }
        };
        (assign42150_e40247, assign42150_e40247_d_n4, assign42150_e40247_d_n7, assign42150_e40247_d_n8,)
    } else {
        (var_fn462_calc_ig__expbd1, var_fn462_calc_ig__expbd1_dn4, var_fn462_calc_ig__expbd1_dn7, var_fn462_calc_ig__expbd1_dn8,)
    }
};
        var_fn462_calc_ig__expbd1 = assign42150_e40249;
        var_fn462_calc_ig__expbd1_dn4 = assign42150_e40249_d_n4;
        var_fn462_calc_ig__expbd1_dn7 = assign42150_e40249_d_n7;
        var_fn462_calc_ig__expbd1_dn8 = assign42150_e40249_d_n8;

        let (assign42160_e40291, assign42160_e40291_d_n4,) = {
    if (var_guard461 != 0.0) {
        let assign42160_e40257: f64 = (-50.0);
        let (assign42160_e40289, assign42160_e40289_d_n4,) = {
            if ((!(var_fn462_calc_ig__expbdarg2 > 50.0)) && (!(var_fn462_calc_ig__expbdarg2 < assign42160_e40257))) {
                let assign42160_e40262: f64 = (var_fn462_calc_ig__expbdarg2).exp();
                (assign42160_e40262, (assign42160_e40262 * var_fn462_calc_ig__expbdarg2_dn4),)
            } else {
                let assign42160_e40269: f64 = (-50.0);
                let (assign42160_e40288, assign42160_e40288_d_n4,) = {
                    if ((!(var_fn462_calc_ig__expbdarg2 > 50.0)) && (var_fn462_calc_ig__expbdarg2 < assign42160_e40269)) {
                        let assign42160_e40273: f64 = (-50.0);
                        let assign42160_e40274: f64 = (assign42160_e40273).exp();
                        (assign42160_e40274, 0.0,)
                    } else {
                        let (assign42160_e40287, assign42160_e40287_d_n4,) = {
                            if (var_fn462_calc_ig__expbdarg2 > 50.0) {
                                let assign42160_e40279: f64 = (50.0_f64).exp();
                                let assign42160_e40283: f64 = (var_fn462_calc_ig__expbdarg2 - 50.0);
                                let assign42160_e40284: f64 = (1.0 + assign42160_e40283);
                                let assign42160_e40285: f64 = (assign42160_e40279 * assign42160_e40284);
                                (assign42160_e40285, (assign42160_e40279 * var_fn462_calc_ig__expbdarg2_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign42160_e40287, assign42160_e40287_d_n4,)
                    }
                };
                (assign42160_e40288, assign42160_e40288_d_n4,)
            }
        };
        (assign42160_e40289, assign42160_e40289_d_n4,)
    } else {
        (var_fn462_calc_ig__expbd2, var_fn462_calc_ig__expbd2_dn4,)
    }
};
        var_fn462_calc_ig__expbd2 = assign42160_e40291;
        var_fn462_calc_ig__expbd2_dn4 = assign42160_e40291_d_n4;

        let (assign42170_e40297, assign42170_e40297_d_n4, assign42170_e40297_d_n7, assign42170_e40297_d_n8,) = {
    if (var_guard461 != 0.0) {
        let assign42170_e40295: f64 = (var_fn462_calc_ig__expbd1 - var_fn462_calc_ig__expbd2);
        (assign42170_e40295, (var_fn462_calc_ig__expbd1_dn4 - var_fn462_calc_ig__expbd2_dn4), var_fn462_calc_ig__expbd1_dn7, var_fn462_calc_ig__expbd1_dn8,)
    } else {
        (var_fn462_calc_ig__iginbd, var_fn462_calc_ig__iginbd_dn4, var_fn462_calc_ig__iginbd_dn7, var_fn462_calc_ig__iginbd_dn8,)
    }
};
        var_fn462_calc_ig__iginbd = assign42170_e40297;
        var_fn462_calc_ig__iginbd_dn4 = assign42170_e40297_d_n4;
        var_fn462_calc_ig__iginbd_dn7 = assign42170_e40297_d_n7;
        var_fn462_calc_ig__iginbd_dn8 = assign42170_e40297_d_n8;

        let (assign42180_e40309, assign42180_e40309_d_n4,) = {
    if (var_guard461 != 0.0) {
        let assign42180_e40301: f64 = (var_fn462_calc_ig__type * var_fn462_calc_ig__w);
        let assign42180_e40303: f64 = (assign42180_e40301 * var_fn462_calc_ig__ngf);
        let assign42180_e40305: f64 = (assign42180_e40303 * var_fn462_calc_ig__ijin);
        let assign42180_e40307: f64 = (assign42180_e40305 * var_fn462_calc_ig__tfacdiodein);
        (assign42180_e40307, (assign42180_e40305 * var_fn462_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn462_calc_ig__isdiodeout, var_fn462_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn462_calc_ig__isdiodeout = assign42180_e40309;
        var_fn462_calc_ig__isdiodeout_dn4 = assign42180_e40309_d_n4;

        let (assign42190_e40319, assign42190_e40319_d_n4, assign42190_e40319_d_n7, assign42190_e40319_d_n8,) = {
    if (var_guard461 != 0.0) {
        let assign42190_e40313: f64 = (var_fn462_calc_ig__pg_paramin / var_fn462_calc_ig__phitin);
        let assign42190_e40315: f64 = (assign42190_e40313 * var_fn462_calc_ig__vgin);
        let assign42190_e40317: f64 = (assign42190_e40315 + var_fn462_calc_ig__expphib);
        (assign42190_e40317, (((-((var_fn462_calc_ig__pg_paramin * var_fn462_calc_ig__phitin_dn4) / (var_fn462_calc_ig__phitin * var_fn462_calc_ig__phitin))) * var_fn462_calc_ig__vgin) + var_fn462_calc_ig__expphib_dn4), (assign42190_e40313 * var_fn462_calc_ig__vgin_dn7), (assign42190_e40313 * var_fn462_calc_ig__vgin_dn8),)
    } else {
        (var_fn462_calc_ig__expiforarg, var_fn462_calc_ig__expiforarg_dn4, var_fn462_calc_ig__expiforarg_dn7, var_fn462_calc_ig__expiforarg_dn8,)
    }
};
        var_fn462_calc_ig__expiforarg = assign42190_e40319;
        var_fn462_calc_ig__expiforarg_dn4 = assign42190_e40319_d_n4;
        var_fn462_calc_ig__expiforarg_dn7 = assign42190_e40319_d_n7;
        var_fn462_calc_ig__expiforarg_dn8 = assign42190_e40319_d_n8;

        let (assign42200_e40361, assign42200_e40361_d_n4, assign42200_e40361_d_n7, assign42200_e40361_d_n8,) = {
    if (var_guard461 != 0.0) {
        let assign42200_e40327: f64 = (-50.0);
        let (assign42200_e40359, assign42200_e40359_d_n4, assign42200_e40359_d_n7, assign42200_e40359_d_n8,) = {
            if ((!(var_fn462_calc_ig__expiforarg > 50.0)) && (!(var_fn462_calc_ig__expiforarg < assign42200_e40327))) {
                let assign42200_e40332: f64 = (var_fn462_calc_ig__expiforarg).exp();
                (assign42200_e40332, (assign42200_e40332 * var_fn462_calc_ig__expiforarg_dn4), (assign42200_e40332 * var_fn462_calc_ig__expiforarg_dn7), (assign42200_e40332 * var_fn462_calc_ig__expiforarg_dn8),)
            } else {
                let assign42200_e40339: f64 = (-50.0);
                let (assign42200_e40358, assign42200_e40358_d_n4, assign42200_e40358_d_n7, assign42200_e40358_d_n8,) = {
                    if ((!(var_fn462_calc_ig__expiforarg > 50.0)) && (var_fn462_calc_ig__expiforarg < assign42200_e40339)) {
                        let assign42200_e40343: f64 = (-50.0);
                        let assign42200_e40344: f64 = (assign42200_e40343).exp();
                        (assign42200_e40344, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign42200_e40357, assign42200_e40357_d_n4, assign42200_e40357_d_n7, assign42200_e40357_d_n8,) = {
                            if (var_fn462_calc_ig__expiforarg > 50.0) {
                                let assign42200_e40349: f64 = (50.0_f64).exp();
                                let assign42200_e40353: f64 = (var_fn462_calc_ig__expiforarg - 50.0);
                                let assign42200_e40354: f64 = (1.0 + assign42200_e40353);
                                let assign42200_e40355: f64 = (assign42200_e40349 * assign42200_e40354);
                                (assign42200_e40355, (assign42200_e40349 * var_fn462_calc_ig__expiforarg_dn4), (assign42200_e40349 * var_fn462_calc_ig__expiforarg_dn7), (assign42200_e40349 * var_fn462_calc_ig__expiforarg_dn8),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign42200_e40357, assign42200_e40357_d_n4, assign42200_e40357_d_n7, assign42200_e40357_d_n8,)
                    }
                };
                (assign42200_e40358, assign42200_e40358_d_n4, assign42200_e40358_d_n7, assign42200_e40358_d_n8,)
            }
        };
        (assign42200_e40359, assign42200_e40359_d_n4, assign42200_e40359_d_n7, assign42200_e40359_d_n8,)
    } else {
        (var_fn462_calc_ig__expifor, var_fn462_calc_ig__expifor_dn4, var_fn462_calc_ig__expifor_dn7, var_fn462_calc_ig__expifor_dn8,)
    }
};
        var_fn462_calc_ig__expifor = assign42200_e40361;
        var_fn462_calc_ig__expifor_dn4 = assign42200_e40361_d_n4;
        var_fn462_calc_ig__expifor_dn7 = assign42200_e40361_d_n7;
        var_fn462_calc_ig__expifor_dn8 = assign42200_e40361_d_n8;

        let assign42210_e40364: f64 = if var_fn462_calc_ig__fracin == 1.0 { 1.0 } else { 0.0 };
        var_guard463 = assign42210_e40364;

        let (assign42220_e40378, assign42220_e40378_d_n4, assign42220_e40378_d_n7, assign42220_e40378_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard463 != 0.0)) {
        let assign42220_e40372: f64 = (var_fn462_calc_ig__kbdgatein * var_fn462_calc_ig__iginbd);
        let assign42220_e40373: f64 = (var_fn462_calc_ig__expifor - assign42220_e40372);
        let assign42220_e40375: f64 = (assign42220_e40373 - var_fn462_calc_ig__t0);
        let assign42220_e40376: f64 = (var_fn462_calc_ig__isdiodeout * assign42220_e40375);
        (assign42220_e40376, ((var_fn462_calc_ig__isdiodeout_dn4 * assign42220_e40375) + (var_fn462_calc_ig__isdiodeout * ((var_fn462_calc_ig__expifor_dn4 - (var_fn462_calc_ig__kbdgatein * var_fn462_calc_ig__iginbd_dn4)) - var_fn462_calc_ig__t0_dn4))), (var_fn462_calc_ig__isdiodeout * (var_fn462_calc_ig__expifor_dn7 - (var_fn462_calc_ig__kbdgatein * var_fn462_calc_ig__iginbd_dn7))), (var_fn462_calc_ig__isdiodeout * (var_fn462_calc_ig__expifor_dn8 - (var_fn462_calc_ig__kbdgatein * var_fn462_calc_ig__iginbd_dn8))),)
    } else {
        (var_fn462_calc_ig__igindiode, var_fn462_calc_ig__igindiode_dn4, var_fn462_calc_ig__igindiode_dn7, var_fn462_calc_ig__igindiode_dn8,)
    }
};
        var_fn462_calc_ig__igindiode = assign42220_e40378;
        var_fn462_calc_ig__igindiode_dn4 = assign42220_e40378_d_n4;
        var_fn462_calc_ig__igindiode_dn7 = assign42220_e40378_d_n7;
        var_fn462_calc_ig__igindiode_dn8 = assign42220_e40378_d_n8;

        let (assign42230_e40392, assign42230_e40392_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard463 == 0.0)) {
        let assign42230_e40385: f64 = (-var_fn462_calc_ig__vgsatin);
        let assign42230_e40387: f64 = (assign42230_e40385 - var_fn462_calc_ig__vbdgin);
        let assign42230_e40388: f64 = (var_fn462_calc_ig__pbdgin * assign42230_e40387);
        let assign42230_e40390: f64 = (assign42230_e40388 + var_fn462_calc_ig__expphib);
        (assign42230_e40390, var_fn462_calc_ig__expphib_dn4,)
    } else {
        (var_fn462_calc_ig__expbdarg1_vgsat, var_fn462_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__expbdarg1_vgsat = assign42230_e40392;
        var_fn462_calc_ig__expbdarg1_vgsat_dn4 = assign42230_e40392_d_n4;

        let (assign42240_e40437, assign42240_e40437_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard463 == 0.0)) {
        let assign42240_e40403: f64 = (-50.0);
        let (assign42240_e40435, assign42240_e40435_d_n4,) = {
            if ((!(var_fn462_calc_ig__expbdarg1_vgsat > 50.0)) && (!(var_fn462_calc_ig__expbdarg1_vgsat < assign42240_e40403))) {
                let assign42240_e40408: f64 = (var_fn462_calc_ig__expbdarg1_vgsat).exp();
                (assign42240_e40408, (assign42240_e40408 * var_fn462_calc_ig__expbdarg1_vgsat_dn4),)
            } else {
                let assign42240_e40415: f64 = (-50.0);
                let (assign42240_e40434, assign42240_e40434_d_n4,) = {
                    if ((!(var_fn462_calc_ig__expbdarg1_vgsat > 50.0)) && (var_fn462_calc_ig__expbdarg1_vgsat < assign42240_e40415)) {
                        let assign42240_e40419: f64 = (-50.0);
                        let assign42240_e40420: f64 = (assign42240_e40419).exp();
                        (assign42240_e40420, 0.0,)
                    } else {
                        let (assign42240_e40433, assign42240_e40433_d_n4,) = {
                            if (var_fn462_calc_ig__expbdarg1_vgsat > 50.0) {
                                let assign42240_e40425: f64 = (50.0_f64).exp();
                                let assign42240_e40429: f64 = (var_fn462_calc_ig__expbdarg1_vgsat - 50.0);
                                let assign42240_e40430: f64 = (1.0 + assign42240_e40429);
                                let assign42240_e40431: f64 = (assign42240_e40425 * assign42240_e40430);
                                (assign42240_e40431, (assign42240_e40425 * var_fn462_calc_ig__expbdarg1_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign42240_e40433, assign42240_e40433_d_n4,)
                    }
                };
                (assign42240_e40434, assign42240_e40434_d_n4,)
            }
        };
        (assign42240_e40435, assign42240_e40435_d_n4,)
    } else {
        (var_fn462_calc_ig__expbd1_vgsat, var_fn462_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__expbd1_vgsat = assign42240_e40437;
        var_fn462_calc_ig__expbd1_vgsat_dn4 = assign42240_e40437_d_n4;

        let (assign42250_e40446, assign42250_e40446_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard463 == 0.0)) {
        let assign42250_e40444: f64 = (var_fn462_calc_ig__expbd1_vgsat - var_fn462_calc_ig__expbd2);
        (assign42250_e40444, (var_fn462_calc_ig__expbd1_vgsat_dn4 - var_fn462_calc_ig__expbd2_dn4),)
    } else {
        (var_fn462_calc_ig__iginbd_vgsat, var_fn462_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__iginbd_vgsat = assign42250_e40446;
        var_fn462_calc_ig__iginbd_vgsat_dn4 = assign42250_e40446_d_n4;

        let (assign42260_e40459, assign42260_e40459_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard463 == 0.0)) {
        let assign42260_e40453: f64 = (var_fn462_calc_ig__pg_paramin / var_fn462_calc_ig__phitin);
        let assign42260_e40455: f64 = (assign42260_e40453 * var_fn462_calc_ig__vgsatin);
        let assign42260_e40457: f64 = (assign42260_e40455 + var_fn462_calc_ig__expphib);
        (assign42260_e40457, (((-((var_fn462_calc_ig__pg_paramin * var_fn462_calc_ig__phitin_dn4) / (var_fn462_calc_ig__phitin * var_fn462_calc_ig__phitin))) * var_fn462_calc_ig__vgsatin) + var_fn462_calc_ig__expphib_dn4),)
    } else {
        (var_fn462_calc_ig__expiforarg_nohinj_vgsat, var_fn462_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__expiforarg_nohinj_vgsat = assign42260_e40459;
        var_fn462_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign42260_e40459_d_n4;

        let (assign42270_e40504, assign42270_e40504_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard463 == 0.0)) {
        let assign42270_e40470: f64 = (-50.0);
        let (assign42270_e40502, assign42270_e40502_d_n4,) = {
            if ((!(var_fn462_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (!(var_fn462_calc_ig__expiforarg_nohinj_vgsat < assign42270_e40470))) {
                let assign42270_e40475: f64 = (var_fn462_calc_ig__expiforarg_nohinj_vgsat).exp();
                (assign42270_e40475, (assign42270_e40475 * var_fn462_calc_ig__expiforarg_nohinj_vgsat_dn4),)
            } else {
                let assign42270_e40482: f64 = (-50.0);
                let (assign42270_e40501, assign42270_e40501_d_n4,) = {
                    if ((!(var_fn462_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (var_fn462_calc_ig__expiforarg_nohinj_vgsat < assign42270_e40482)) {
                        let assign42270_e40486: f64 = (-50.0);
                        let assign42270_e40487: f64 = (assign42270_e40486).exp();
                        (assign42270_e40487, 0.0,)
                    } else {
                        let (assign42270_e40500, assign42270_e40500_d_n4,) = {
                            if (var_fn462_calc_ig__expiforarg_nohinj_vgsat > 50.0) {
                                let assign42270_e40492: f64 = (50.0_f64).exp();
                                let assign42270_e40496: f64 = (var_fn462_calc_ig__expiforarg_nohinj_vgsat - 50.0);
                                let assign42270_e40497: f64 = (1.0 + assign42270_e40496);
                                let assign42270_e40498: f64 = (assign42270_e40492 * assign42270_e40497);
                                (assign42270_e40498, (assign42270_e40492 * var_fn462_calc_ig__expiforarg_nohinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign42270_e40500, assign42270_e40500_d_n4,)
                    }
                };
                (assign42270_e40501, assign42270_e40501_d_n4,)
            }
        };
        (assign42270_e40502, assign42270_e40502_d_n4,)
    } else {
        (var_fn462_calc_ig__expifor_nohinj_vgsat, var_fn462_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__expifor_nohinj_vgsat = assign42270_e40504;
        var_fn462_calc_ig__expifor_nohinj_vgsat_dn4 = assign42270_e40504_d_n4;

        let (assign42280_e40517, assign42280_e40517_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard463 == 0.0)) {
        let assign42280_e40512: f64 = (var_fn462_calc_ig__kbdgatein * var_fn462_calc_ig__iginbd_vgsat);
        let assign42280_e40513: f64 = (var_fn462_calc_ig__expifor_nohinj_vgsat - assign42280_e40512);
        let assign42280_e40515: f64 = (assign42280_e40513 - var_fn462_calc_ig__t0);
        (assign42280_e40515, ((var_fn462_calc_ig__expifor_nohinj_vgsat_dn4 - (var_fn462_calc_ig__kbdgatein * var_fn462_calc_ig__iginbd_vgsat_dn4)) - var_fn462_calc_ig__t0_dn4),)
    } else {
        (var_fn462_calc_ig__igindiode_nohinj_vgsat, var_fn462_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__igindiode_nohinj_vgsat = assign42280_e40517;
        var_fn462_calc_ig__igindiode_nohinj_vgsat_dn4 = assign42280_e40517_d_n4;

        let (assign42290_e40532, assign42290_e40532_d_n4, assign42290_e40532_d_n7, assign42290_e40532_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard463 == 0.0)) {
        let assign42290_e40526: f64 = (var_fn462_calc_ig__kbdgatein * var_fn462_calc_ig__iginbd);
        let assign42290_e40527: f64 = (var_fn462_calc_ig__expifor - assign42290_e40526);
        let assign42290_e40529: f64 = (assign42290_e40527 - var_fn462_calc_ig__t0);
        let assign42290_e40530: f64 = (var_fn462_calc_ig__isdiodeout * assign42290_e40529);
        (assign42290_e40530, ((var_fn462_calc_ig__isdiodeout_dn4 * assign42290_e40529) + (var_fn462_calc_ig__isdiodeout * ((var_fn462_calc_ig__expifor_dn4 - (var_fn462_calc_ig__kbdgatein * var_fn462_calc_ig__iginbd_dn4)) - var_fn462_calc_ig__t0_dn4))), (var_fn462_calc_ig__isdiodeout * (var_fn462_calc_ig__expifor_dn7 - (var_fn462_calc_ig__kbdgatein * var_fn462_calc_ig__iginbd_dn7))), (var_fn462_calc_ig__isdiodeout * (var_fn462_calc_ig__expifor_dn8 - (var_fn462_calc_ig__kbdgatein * var_fn462_calc_ig__iginbd_dn8))),)
    } else {
        (var_fn462_calc_ig__igindiode_nohinj, var_fn462_calc_ig__igindiode_nohinj_dn4, var_fn462_calc_ig__igindiode_nohinj_dn7, var_fn462_calc_ig__igindiode_nohinj_dn8,)
    }
};
        var_fn462_calc_ig__igindiode_nohinj = assign42290_e40532;
        var_fn462_calc_ig__igindiode_nohinj_dn4 = assign42290_e40532_d_n4;
        var_fn462_calc_ig__igindiode_nohinj_dn7 = assign42290_e40532_d_n7;
        var_fn462_calc_ig__igindiode_nohinj_dn8 = assign42290_e40532_d_n8;

        let assign42300_e40535: f64 = if var_fn462_calc_ig__fracin > 0.0 { 1.0 } else { 0.0 };
        var_guard464 = assign42300_e40535;

        let (assign42310_e40546,) = {
    if (((var_guard461 != 0.0) && (var_guard463 == 0.0)) && (var_guard464 != 0.0)) {
        let assign42310_e40544: f64 = (var_fn462_calc_ig__fracin * var_fn462_calc_ig__pg_paramin);
        (assign42310_e40544,)
    } else {
        (var_fn462_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn462_calc_ig__pg_paramin_hinj = assign42310_e40546;

        let (assign42320_e40561, assign42320_e40561_d_n4,) = {
    if (((var_guard461 != 0.0) && (var_guard463 == 0.0)) && (var_guard464 != 0.0)) {
        let assign42320_e40555: f64 = (var_fn462_calc_ig__pg_paramin_hinj / var_fn462_calc_ig__phitin);
        let assign42320_e40557: f64 = (assign42320_e40555 * var_fn462_calc_ig__vgsatin);
        let assign42320_e40559: f64 = (assign42320_e40557 + var_fn462_calc_ig__expphib);
        (assign42320_e40559, (((-((var_fn462_calc_ig__pg_paramin_hinj * var_fn462_calc_ig__phitin_dn4) / (var_fn462_calc_ig__phitin * var_fn462_calc_ig__phitin))) * var_fn462_calc_ig__vgsatin) + var_fn462_calc_ig__expphib_dn4),)
    } else {
        (var_fn462_calc_ig__expiforarg_hinj_vgsat, var_fn462_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__expiforarg_hinj_vgsat = assign42320_e40561;
        var_fn462_calc_ig__expiforarg_hinj_vgsat_dn4 = assign42320_e40561_d_n4;

        let (assign42330_e40608, assign42330_e40608_d_n4,) = {
    if (((var_guard461 != 0.0) && (var_guard463 == 0.0)) && (var_guard464 != 0.0)) {
        let assign42330_e40574: f64 = (-50.0);
        let (assign42330_e40606, assign42330_e40606_d_n4,) = {
            if ((!(var_fn462_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (!(var_fn462_calc_ig__expiforarg_hinj_vgsat < assign42330_e40574))) {
                let assign42330_e40579: f64 = (var_fn462_calc_ig__expiforarg_hinj_vgsat).exp();
                (assign42330_e40579, (assign42330_e40579 * var_fn462_calc_ig__expiforarg_hinj_vgsat_dn4),)
            } else {
                let assign42330_e40586: f64 = (-50.0);
                let (assign42330_e40605, assign42330_e40605_d_n4,) = {
                    if ((!(var_fn462_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (var_fn462_calc_ig__expiforarg_hinj_vgsat < assign42330_e40586)) {
                        let assign42330_e40590: f64 = (-50.0);
                        let assign42330_e40591: f64 = (assign42330_e40590).exp();
                        (assign42330_e40591, 0.0,)
                    } else {
                        let (assign42330_e40604, assign42330_e40604_d_n4,) = {
                            if (var_fn462_calc_ig__expiforarg_hinj_vgsat > 50.0) {
                                let assign42330_e40596: f64 = (50.0_f64).exp();
                                let assign42330_e40600: f64 = (var_fn462_calc_ig__expiforarg_hinj_vgsat - 50.0);
                                let assign42330_e40601: f64 = (1.0 + assign42330_e40600);
                                let assign42330_e40602: f64 = (assign42330_e40596 * assign42330_e40601);
                                (assign42330_e40602, (assign42330_e40596 * var_fn462_calc_ig__expiforarg_hinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign42330_e40604, assign42330_e40604_d_n4,)
                    }
                };
                (assign42330_e40605, assign42330_e40605_d_n4,)
            }
        };
        (assign42330_e40606, assign42330_e40606_d_n4,)
    } else {
        (var_fn462_calc_ig__expifor_hinj_vgsat, var_fn462_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__expifor_hinj_vgsat = assign42330_e40608;
        var_fn462_calc_ig__expifor_hinj_vgsat_dn4 = assign42330_e40608_d_n4;

        let (assign42340_e40623, assign42340_e40623_d_n4,) = {
    if (((var_guard461 != 0.0) && (var_guard463 == 0.0)) && (var_guard464 != 0.0)) {
        let assign42340_e40618: f64 = (var_fn462_calc_ig__kbdgatein * var_fn462_calc_ig__iginbd_vgsat);
        let assign42340_e40619: f64 = (var_fn462_calc_ig__expifor_hinj_vgsat - assign42340_e40618);
        let assign42340_e40621: f64 = (assign42340_e40619 - var_fn462_calc_ig__t0);
        (assign42340_e40621, ((var_fn462_calc_ig__expifor_hinj_vgsat_dn4 - (var_fn462_calc_ig__kbdgatein * var_fn462_calc_ig__iginbd_vgsat_dn4)) - var_fn462_calc_ig__t0_dn4),)
    } else {
        (var_fn462_calc_ig__igindiode_hinj_vgsat, var_fn462_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn462_calc_ig__igindiode_hinj_vgsat = assign42340_e40623;
        var_fn462_calc_ig__igindiode_hinj_vgsat_dn4 = assign42340_e40623_d_n4;

        let (assign42350_e40638, assign42350_e40638_d_n4, assign42350_e40638_d_n7, assign42350_e40638_d_n8,) = {
    if (((var_guard461 != 0.0) && (var_guard463 == 0.0)) && (var_guard464 != 0.0)) {
        let assign42350_e40632: f64 = (var_fn462_calc_ig__pg_paramin_hinj / var_fn462_calc_ig__phitin);
        let assign42350_e40634: f64 = (assign42350_e40632 * var_fn462_calc_ig__vgin);
        let assign42350_e40636: f64 = (assign42350_e40634 + var_fn462_calc_ig__expphib);
        (assign42350_e40636, (((-((var_fn462_calc_ig__pg_paramin_hinj * var_fn462_calc_ig__phitin_dn4) / (var_fn462_calc_ig__phitin * var_fn462_calc_ig__phitin))) * var_fn462_calc_ig__vgin) + var_fn462_calc_ig__expphib_dn4), (assign42350_e40632 * var_fn462_calc_ig__vgin_dn7), (assign42350_e40632 * var_fn462_calc_ig__vgin_dn8),)
    } else {
        (var_fn462_calc_ig__expiforarg_hinj, var_fn462_calc_ig__expiforarg_hinj_dn4, var_fn462_calc_ig__expiforarg_hinj_dn7, var_fn462_calc_ig__expiforarg_hinj_dn8,)
    }
};
        var_fn462_calc_ig__expiforarg_hinj = assign42350_e40638;
        var_fn462_calc_ig__expiforarg_hinj_dn4 = assign42350_e40638_d_n4;
        var_fn462_calc_ig__expiforarg_hinj_dn7 = assign42350_e40638_d_n7;
        var_fn462_calc_ig__expiforarg_hinj_dn8 = assign42350_e40638_d_n8;

        let (assign42360_e40685, assign42360_e40685_d_n4, assign42360_e40685_d_n7, assign42360_e40685_d_n8,) = {
    if (((var_guard461 != 0.0) && (var_guard463 == 0.0)) && (var_guard464 != 0.0)) {
        let assign42360_e40651: f64 = (-50.0);
        let (assign42360_e40683, assign42360_e40683_d_n4, assign42360_e40683_d_n7, assign42360_e40683_d_n8,) = {
            if ((!(var_fn462_calc_ig__expiforarg_hinj > 50.0)) && (!(var_fn462_calc_ig__expiforarg_hinj < assign42360_e40651))) {
                let assign42360_e40656: f64 = (var_fn462_calc_ig__expiforarg_hinj).exp();
                (assign42360_e40656, (assign42360_e40656 * var_fn462_calc_ig__expiforarg_hinj_dn4), (assign42360_e40656 * var_fn462_calc_ig__expiforarg_hinj_dn7), (assign42360_e40656 * var_fn462_calc_ig__expiforarg_hinj_dn8),)
            } else {
                let assign42360_e40663: f64 = (-50.0);
                let (assign42360_e40682, assign42360_e40682_d_n4, assign42360_e40682_d_n7, assign42360_e40682_d_n8,) = {
                    if ((!(var_fn462_calc_ig__expiforarg_hinj > 50.0)) && (var_fn462_calc_ig__expiforarg_hinj < assign42360_e40663)) {
                        let assign42360_e40667: f64 = (-50.0);
                        let assign42360_e40668: f64 = (assign42360_e40667).exp();
                        (assign42360_e40668, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign42360_e40681, assign42360_e40681_d_n4, assign42360_e40681_d_n7, assign42360_e40681_d_n8,) = {
                            if (var_fn462_calc_ig__expiforarg_hinj > 50.0) {
                                let assign42360_e40673: f64 = (50.0_f64).exp();
                                let assign42360_e40677: f64 = (var_fn462_calc_ig__expiforarg_hinj - 50.0);
                                let assign42360_e40678: f64 = (1.0 + assign42360_e40677);
                                let assign42360_e40679: f64 = (assign42360_e40673 * assign42360_e40678);
                                (assign42360_e40679, (assign42360_e40673 * var_fn462_calc_ig__expiforarg_hinj_dn4), (assign42360_e40673 * var_fn462_calc_ig__expiforarg_hinj_dn7), (assign42360_e40673 * var_fn462_calc_ig__expiforarg_hinj_dn8),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign42360_e40681, assign42360_e40681_d_n4, assign42360_e40681_d_n7, assign42360_e40681_d_n8,)
                    }
                };
                (assign42360_e40682, assign42360_e40682_d_n4, assign42360_e40682_d_n7, assign42360_e40682_d_n8,)
            }
        };
        (assign42360_e40683, assign42360_e40683_d_n4, assign42360_e40683_d_n7, assign42360_e40683_d_n8,)
    } else {
        (var_fn462_calc_ig__expifor_hinj, var_fn462_calc_ig__expifor_hinj_dn4, var_fn462_calc_ig__expifor_hinj_dn7, var_fn462_calc_ig__expifor_hinj_dn8,)
    }
};
        var_fn462_calc_ig__expifor_hinj = assign42360_e40685;
        var_fn462_calc_ig__expifor_hinj_dn4 = assign42360_e40685_d_n4;
        var_fn462_calc_ig__expifor_hinj_dn7 = assign42360_e40685_d_n7;
        var_fn462_calc_ig__expifor_hinj_dn8 = assign42360_e40685_d_n8;

        let (assign42370_e40698, assign42370_e40698_d_n4,) = {
    if (((var_guard461 != 0.0) && (var_guard463 == 0.0)) && (var_guard464 != 0.0)) {
        let assign42370_e40694: f64 = (var_fn462_calc_ig__isdiodeout * var_fn462_calc_ig__igindiode_nohinj_vgsat);
        let assign42370_e40696: f64 = (assign42370_e40694 / var_fn462_calc_ig__igindiode_hinj_vgsat);
        (assign42370_e40696, (((((var_fn462_calc_ig__isdiodeout_dn4 * var_fn462_calc_ig__igindiode_nohinj_vgsat) + (var_fn462_calc_ig__isdiodeout * var_fn462_calc_ig__igindiode_nohinj_vgsat_dn4)) * var_fn462_calc_ig__igindiode_hinj_vgsat) - (assign42370_e40694 * var_fn462_calc_ig__igindiode_hinj_vgsat_dn4)) / (var_fn462_calc_ig__igindiode_hinj_vgsat * var_fn462_calc_ig__igindiode_hinj_vgsat)),)
    } else {
        (var_fn462_calc_ig__igindiode_hinj_pre, var_fn462_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn462_calc_ig__igindiode_hinj_pre = assign42370_e40698;
        var_fn462_calc_ig__igindiode_hinj_pre_dn4 = assign42370_e40698_d_n4;

        let (assign42380_e40715, assign42380_e40715_d_n4, assign42380_e40715_d_n7, assign42380_e40715_d_n8,) = {
    if (((var_guard461 != 0.0) && (var_guard463 == 0.0)) && (var_guard464 != 0.0)) {
        let assign42380_e40709: f64 = (var_fn462_calc_ig__kbdgatein * var_fn462_calc_ig__iginbd);
        let assign42380_e40710: f64 = (var_fn462_calc_ig__expifor_hinj - assign42380_e40709);
        let assign42380_e40712: f64 = (assign42380_e40710 - var_fn462_calc_ig__t0);
        let assign42380_e40713: f64 = (var_fn462_calc_ig__igindiode_hinj_pre * assign42380_e40712);
        (assign42380_e40713, ((var_fn462_calc_ig__igindiode_hinj_pre_dn4 * assign42380_e40712) + (var_fn462_calc_ig__igindiode_hinj_pre * ((var_fn462_calc_ig__expifor_hinj_dn4 - (var_fn462_calc_ig__kbdgatein * var_fn462_calc_ig__iginbd_dn4)) - var_fn462_calc_ig__t0_dn4))), (var_fn462_calc_ig__igindiode_hinj_pre * (var_fn462_calc_ig__expifor_hinj_dn7 - (var_fn462_calc_ig__kbdgatein * var_fn462_calc_ig__iginbd_dn7))), (var_fn462_calc_ig__igindiode_hinj_pre * (var_fn462_calc_ig__expifor_hinj_dn8 - (var_fn462_calc_ig__kbdgatein * var_fn462_calc_ig__iginbd_dn8))),)
    } else {
        (var_fn462_calc_ig__igindiode_hinj, var_fn462_calc_ig__igindiode_hinj_dn4, var_fn462_calc_ig__igindiode_hinj_dn7, var_fn462_calc_ig__igindiode_hinj_dn8,)
    }
};
        var_fn462_calc_ig__igindiode_hinj = assign42380_e40715;
        var_fn462_calc_ig__igindiode_hinj_dn4 = assign42380_e40715_d_n4;
        var_fn462_calc_ig__igindiode_hinj_dn7 = assign42380_e40715_d_n7;
        var_fn462_calc_ig__igindiode_hinj_dn8 = assign42380_e40715_d_n8;

        let (assign42390_e40727, assign42390_e40727_d_n4, assign42390_e40727_d_n7, assign42390_e40727_d_n8,) = {
    if (((var_guard461 != 0.0) && (var_guard463 == 0.0)) && (var_guard464 == 0.0)) {
        let assign42390_e40725: f64 = (var_fn462_calc_ig__isdiodeout * var_fn462_calc_ig__igindiode_nohinj_vgsat);
        (assign42390_e40725, ((var_fn462_calc_ig__isdiodeout_dn4 * var_fn462_calc_ig__igindiode_nohinj_vgsat) + (var_fn462_calc_ig__isdiodeout * var_fn462_calc_ig__igindiode_nohinj_vgsat_dn4)), 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__igindiode_hinj, var_fn462_calc_ig__igindiode_hinj_dn4, var_fn462_calc_ig__igindiode_hinj_dn7, var_fn462_calc_ig__igindiode_hinj_dn8,)
    }
};
        var_fn462_calc_ig__igindiode_hinj = assign42390_e40727;
        var_fn462_calc_ig__igindiode_hinj_dn4 = assign42390_e40727_d_n4;
        var_fn462_calc_ig__igindiode_hinj_dn7 = assign42390_e40727_d_n7;
        var_fn462_calc_ig__igindiode_hinj_dn8 = assign42390_e40727_d_n8;

        *var_fn462_calc_ig__expbd1_slot = var_fn462_calc_ig__expbd1;
        *var_fn462_calc_ig__expbd1_dn4_slot = var_fn462_calc_ig__expbd1_dn4;
        *var_fn462_calc_ig__expbd1_dn7_slot = var_fn462_calc_ig__expbd1_dn7;
        *var_fn462_calc_ig__expbd1_dn8_slot = var_fn462_calc_ig__expbd1_dn8;
        *var_fn462_calc_ig__expbd1_vgsat_slot = var_fn462_calc_ig__expbd1_vgsat;
        *var_fn462_calc_ig__expbd1_vgsat_dn4_slot = var_fn462_calc_ig__expbd1_vgsat_dn4;
        *var_fn462_calc_ig__expbd2_slot = var_fn462_calc_ig__expbd2;
        *var_fn462_calc_ig__expbd2_dn4_slot = var_fn462_calc_ig__expbd2_dn4;
        *var_fn462_calc_ig__expbdarg1_vgsat_slot = var_fn462_calc_ig__expbdarg1_vgsat;
        *var_fn462_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn462_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn462_calc_ig__expifor_slot = var_fn462_calc_ig__expifor;
        *var_fn462_calc_ig__expifor_dn4_slot = var_fn462_calc_ig__expifor_dn4;
        *var_fn462_calc_ig__expifor_dn7_slot = var_fn462_calc_ig__expifor_dn7;
        *var_fn462_calc_ig__expifor_dn8_slot = var_fn462_calc_ig__expifor_dn8;
        *var_fn462_calc_ig__expifor_hinj_slot = var_fn462_calc_ig__expifor_hinj;
        *var_fn462_calc_ig__expifor_hinj_dn4_slot = var_fn462_calc_ig__expifor_hinj_dn4;
        *var_fn462_calc_ig__expifor_hinj_dn7_slot = var_fn462_calc_ig__expifor_hinj_dn7;
        *var_fn462_calc_ig__expifor_hinj_dn8_slot = var_fn462_calc_ig__expifor_hinj_dn8;
        *var_fn462_calc_ig__expifor_hinj_vgsat_slot = var_fn462_calc_ig__expifor_hinj_vgsat;
        *var_fn462_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn462_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn462_calc_ig__expifor_nohinj_vgsat_slot = var_fn462_calc_ig__expifor_nohinj_vgsat;
        *var_fn462_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn462_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn462_calc_ig__expiforarg_slot = var_fn462_calc_ig__expiforarg;
        *var_fn462_calc_ig__expiforarg_dn4_slot = var_fn462_calc_ig__expiforarg_dn4;
        *var_fn462_calc_ig__expiforarg_dn7_slot = var_fn462_calc_ig__expiforarg_dn7;
        *var_fn462_calc_ig__expiforarg_dn8_slot = var_fn462_calc_ig__expiforarg_dn8;
        *var_fn462_calc_ig__expiforarg_hinj_slot = var_fn462_calc_ig__expiforarg_hinj;
        *var_fn462_calc_ig__expiforarg_hinj_dn4_slot = var_fn462_calc_ig__expiforarg_hinj_dn4;
        *var_fn462_calc_ig__expiforarg_hinj_dn7_slot = var_fn462_calc_ig__expiforarg_hinj_dn7;
        *var_fn462_calc_ig__expiforarg_hinj_dn8_slot = var_fn462_calc_ig__expiforarg_hinj_dn8;
        *var_fn462_calc_ig__expiforarg_hinj_vgsat_slot = var_fn462_calc_ig__expiforarg_hinj_vgsat;
        *var_fn462_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn462_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn462_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn462_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn462_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn462_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn462_calc_ig__iginbd_slot = var_fn462_calc_ig__iginbd;
        *var_fn462_calc_ig__iginbd_dn4_slot = var_fn462_calc_ig__iginbd_dn4;
        *var_fn462_calc_ig__iginbd_dn7_slot = var_fn462_calc_ig__iginbd_dn7;
        *var_fn462_calc_ig__iginbd_dn8_slot = var_fn462_calc_ig__iginbd_dn8;
        *var_fn462_calc_ig__iginbd_vgsat_slot = var_fn462_calc_ig__iginbd_vgsat;
        *var_fn462_calc_ig__iginbd_vgsat_dn4_slot = var_fn462_calc_ig__iginbd_vgsat_dn4;
        *var_fn462_calc_ig__igindiode_slot = var_fn462_calc_ig__igindiode;
        *var_fn462_calc_ig__igindiode_dn4_slot = var_fn462_calc_ig__igindiode_dn4;
        *var_fn462_calc_ig__igindiode_dn7_slot = var_fn462_calc_ig__igindiode_dn7;
        *var_fn462_calc_ig__igindiode_dn8_slot = var_fn462_calc_ig__igindiode_dn8;
        *var_fn462_calc_ig__igindiode_hinj_slot = var_fn462_calc_ig__igindiode_hinj;
        *var_fn462_calc_ig__igindiode_hinj_dn4_slot = var_fn462_calc_ig__igindiode_hinj_dn4;
        *var_fn462_calc_ig__igindiode_hinj_dn7_slot = var_fn462_calc_ig__igindiode_hinj_dn7;
        *var_fn462_calc_ig__igindiode_hinj_dn8_slot = var_fn462_calc_ig__igindiode_hinj_dn8;
        *var_fn462_calc_ig__igindiode_hinj_pre_slot = var_fn462_calc_ig__igindiode_hinj_pre;
        *var_fn462_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn462_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn462_calc_ig__igindiode_hinj_vgsat_slot = var_fn462_calc_ig__igindiode_hinj_vgsat;
        *var_fn462_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn462_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn462_calc_ig__igindiode_nohinj_slot = var_fn462_calc_ig__igindiode_nohinj;
        *var_fn462_calc_ig__igindiode_nohinj_dn4_slot = var_fn462_calc_ig__igindiode_nohinj_dn4;
        *var_fn462_calc_ig__igindiode_nohinj_dn7_slot = var_fn462_calc_ig__igindiode_nohinj_dn7;
        *var_fn462_calc_ig__igindiode_nohinj_dn8_slot = var_fn462_calc_ig__igindiode_nohinj_dn8;
        *var_fn462_calc_ig__igindiode_nohinj_vgsat_slot = var_fn462_calc_ig__igindiode_nohinj_vgsat;
        *var_fn462_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn462_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn462_calc_ig__isdiodeout_slot = var_fn462_calc_ig__isdiodeout;
        *var_fn462_calc_ig__isdiodeout_dn4_slot = var_fn462_calc_ig__isdiodeout_dn4;
        *var_fn462_calc_ig__pg_paramin_hinj_slot = var_fn462_calc_ig__pg_paramin_hinj;
        *var_guard463_slot = var_guard463;
        *var_guard464_slot = var_guard464;
    }

    pub(super) fn stamp_transient_block_106(
        p: &Parameters,
        var_fn462_calc_ig__alphagin: f64,
        var_fn462_calc_ig__betarecin: f64,
        var_fn462_calc_ig__igindiode_hinj: f64,
        var_fn462_calc_ig__igindiode_hinj_dn4: f64,
        var_fn462_calc_ig__igindiode_hinj_dn7: f64,
        var_fn462_calc_ig__igindiode_hinj_dn8: f64,
        var_fn462_calc_ig__igindiode_nohinj: f64,
        var_fn462_calc_ig__igindiode_nohinj_dn4: f64,
        var_fn462_calc_ig__igindiode_nohinj_dn7: f64,
        var_fn462_calc_ig__igindiode_nohinj_dn8: f64,
        var_fn462_calc_ig__irecin: f64,
        var_fn462_calc_ig__ngf: f64,
        var_fn462_calc_ig__pgsrecin: f64,
        var_fn462_calc_ig__phitin: f64,
        var_fn462_calc_ig__phitin_dn4: f64,
        var_fn462_calc_ig__tfacdiodein: f64,
        var_fn462_calc_ig__tfacdiodein_dn4: f64,
        var_fn462_calc_ig__type: f64,
        var_fn462_calc_ig__vgin: f64,
        var_fn462_calc_ig__vgin_dn7: f64,
        var_fn462_calc_ig__vgin_dn8: f64,
        var_fn462_calc_ig__vgsatin: f64,
        var_fn462_calc_ig__vgsatqin: f64,
        var_fn462_calc_ig__w: f64,
        var_guard461: f64,
        var_guard463: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_tfacdiode: f64,
        var_tfacdiode_dn4: f64,
        var_vsch: f64,
        var_vsch_dn7: f64,
        var_vsch_dn8: f64,
        var_fn462_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn462_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn462_calc_ig__expffvarg_slot: &mut f64,
        var_fn462_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn462_calc_ig__expffvarg_dn7_slot: &mut f64,
        var_fn462_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn462_calc_ig__expirev_slot: &mut f64,
        var_fn462_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn462_calc_ig__expirev_dn7_slot: &mut f64,
        var_fn462_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn462_calc_ig__expirevarg_slot: &mut f64,
        var_fn462_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn462_calc_ig__expirevarg_dn7_slot: &mut f64,
        var_fn462_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn462_calc_ig__ffvgin_slot: &mut f64,
        var_fn462_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn462_calc_ig__ffvgin_dn7_slot: &mut f64,
        var_fn462_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn462_calc_ig__frecgin_slot: &mut f64,
        var_fn462_calc_ig__frecgin_dn7_slot: &mut f64,
        var_fn462_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn462_calc_ig__igindiode_slot: &mut f64,
        var_fn462_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn462_calc_ig__igindiode_dn7_slot: &mut f64,
        var_fn462_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn462_calc_ig__iginrec_slot: &mut f64,
        var_fn462_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn462_calc_ig__iginrec_dn7_slot: &mut f64,
        var_fn462_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn462_calc_ig__igout_slot: &mut f64,
        var_fn462_calc_ig__igout_dn4_slot: &mut f64,
        var_fn462_calc_ig__igout_dn7_slot: &mut f64,
        var_fn462_calc_ig__igout_dn8_slot: &mut f64,
        var_fn462_calc_ig__isrecout_slot: &mut f64,
        var_fn462_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn462_calc_ig__return_slot: &mut f64,
        var_fn462_calc_ig__return_dn4_slot: &mut f64,
        var_fn462_calc_ig__return_dn7_slot: &mut f64,
        var_fn462_calc_ig__return_dn8_slot: &mut f64,
        var_fn468_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn468_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn468_calc_ig__alphagin_slot: &mut f64,
        var_fn468_calc_ig__betarecin_slot: &mut f64,
        var_fn468_calc_ig__fracin_slot: &mut f64,
        var_fn468_calc_ig__igout_slot: &mut f64,
        var_fn468_calc_ig__igout_dn4_slot: &mut f64,
        var_fn468_calc_ig__igout_dn7_slot: &mut f64,
        var_fn468_calc_ig__igout_dn8_slot: &mut f64,
        var_fn468_calc_ig__ijin_slot: &mut f64,
        var_fn468_calc_ig__irecin_slot: &mut f64,
        var_fn468_calc_ig__isdiodeout_slot: &mut f64,
        var_fn468_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn468_calc_ig__isrecout_slot: &mut f64,
        var_fn468_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn468_calc_ig__kbdgatein_slot: &mut f64,
        var_fn468_calc_ig__ngf_slot: &mut f64,
        var_fn468_calc_ig__pbdgin_slot: &mut f64,
        var_fn468_calc_ig__pg_param1_slot: &mut f64,
        var_fn468_calc_ig__pg_paramin_slot: &mut f64,
        var_fn468_calc_ig__pgsrecin_slot: &mut f64,
        var_fn468_calc_ig__phitin_slot: &mut f64,
        var_fn468_calc_ig__phitin_dn4_slot: &mut f64,
        var_fn468_calc_ig__return_slot: &mut f64,
        var_fn468_calc_ig__return_dn4_slot: &mut f64,
        var_fn468_calc_ig__return_dn7_slot: &mut f64,
        var_fn468_calc_ig__return_dn8_slot: &mut f64,
        var_fn468_calc_ig__t0_slot: &mut f64,
        var_fn468_calc_ig__t0_dn4_slot: &mut f64,
        var_fn468_calc_ig__tfacdiodein_slot: &mut f64,
        var_fn468_calc_ig__tfacdiodein_dn4_slot: &mut f64,
        var_fn468_calc_ig__type_slot: &mut f64,
        var_fn468_calc_ig__vbdgin_slot: &mut f64,
        var_fn468_calc_ig__vgin_slot: &mut f64,
        var_fn468_calc_ig__vgin_dn7_slot: &mut f64,
        var_fn468_calc_ig__vgin_dn8_slot: &mut f64,
        var_fn468_calc_ig__vgsatin_slot: &mut f64,
        var_fn468_calc_ig__vgsatqin_slot: &mut f64,
        var_fn468_calc_ig__vjg_slot: &mut f64,
        var_fn468_calc_ig__w_slot: &mut f64,
        var_guard465_slot: &mut f64,
        var_guard466_slot: &mut f64,
        var_guard467_slot: &mut f64,
        var_idsch_slot: &mut f64,
        var_idsch_dn4_slot: &mut f64,
        var_idsch_dn7_slot: &mut f64,
        var_idsch_dn8_slot: &mut f64,
    ) {
        let mut var_fn462_calc_ig__alpha2_phit: f64 = *var_fn462_calc_ig__alpha2_phit_slot;
        let mut var_fn462_calc_ig__alpha2_phit_dn4: f64 = *var_fn462_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn462_calc_ig__expffvarg: f64 = *var_fn462_calc_ig__expffvarg_slot;
        let mut var_fn462_calc_ig__expffvarg_dn4: f64 = *var_fn462_calc_ig__expffvarg_dn4_slot;
        let mut var_fn462_calc_ig__expffvarg_dn7: f64 = *var_fn462_calc_ig__expffvarg_dn7_slot;
        let mut var_fn462_calc_ig__expffvarg_dn8: f64 = *var_fn462_calc_ig__expffvarg_dn8_slot;
        let mut var_fn462_calc_ig__expirev: f64 = *var_fn462_calc_ig__expirev_slot;
        let mut var_fn462_calc_ig__expirev_dn4: f64 = *var_fn462_calc_ig__expirev_dn4_slot;
        let mut var_fn462_calc_ig__expirev_dn7: f64 = *var_fn462_calc_ig__expirev_dn7_slot;
        let mut var_fn462_calc_ig__expirev_dn8: f64 = *var_fn462_calc_ig__expirev_dn8_slot;
        let mut var_fn462_calc_ig__expirevarg: f64 = *var_fn462_calc_ig__expirevarg_slot;
        let mut var_fn462_calc_ig__expirevarg_dn4: f64 = *var_fn462_calc_ig__expirevarg_dn4_slot;
        let mut var_fn462_calc_ig__expirevarg_dn7: f64 = *var_fn462_calc_ig__expirevarg_dn7_slot;
        let mut var_fn462_calc_ig__expirevarg_dn8: f64 = *var_fn462_calc_ig__expirevarg_dn8_slot;
        let mut var_fn462_calc_ig__ffvgin: f64 = *var_fn462_calc_ig__ffvgin_slot;
        let mut var_fn462_calc_ig__ffvgin_dn4: f64 = *var_fn462_calc_ig__ffvgin_dn4_slot;
        let mut var_fn462_calc_ig__ffvgin_dn7: f64 = *var_fn462_calc_ig__ffvgin_dn7_slot;
        let mut var_fn462_calc_ig__ffvgin_dn8: f64 = *var_fn462_calc_ig__ffvgin_dn8_slot;
        let mut var_fn462_calc_ig__frecgin: f64 = *var_fn462_calc_ig__frecgin_slot;
        let mut var_fn462_calc_ig__frecgin_dn7: f64 = *var_fn462_calc_ig__frecgin_dn7_slot;
        let mut var_fn462_calc_ig__frecgin_dn8: f64 = *var_fn462_calc_ig__frecgin_dn8_slot;
        let mut var_fn462_calc_ig__igindiode: f64 = *var_fn462_calc_ig__igindiode_slot;
        let mut var_fn462_calc_ig__igindiode_dn4: f64 = *var_fn462_calc_ig__igindiode_dn4_slot;
        let mut var_fn462_calc_ig__igindiode_dn7: f64 = *var_fn462_calc_ig__igindiode_dn7_slot;
        let mut var_fn462_calc_ig__igindiode_dn8: f64 = *var_fn462_calc_ig__igindiode_dn8_slot;
        let mut var_fn462_calc_ig__iginrec: f64 = *var_fn462_calc_ig__iginrec_slot;
        let mut var_fn462_calc_ig__iginrec_dn4: f64 = *var_fn462_calc_ig__iginrec_dn4_slot;
        let mut var_fn462_calc_ig__iginrec_dn7: f64 = *var_fn462_calc_ig__iginrec_dn7_slot;
        let mut var_fn462_calc_ig__iginrec_dn8: f64 = *var_fn462_calc_ig__iginrec_dn8_slot;
        let mut var_fn462_calc_ig__igout: f64 = *var_fn462_calc_ig__igout_slot;
        let mut var_fn462_calc_ig__igout_dn4: f64 = *var_fn462_calc_ig__igout_dn4_slot;
        let mut var_fn462_calc_ig__igout_dn7: f64 = *var_fn462_calc_ig__igout_dn7_slot;
        let mut var_fn462_calc_ig__igout_dn8: f64 = *var_fn462_calc_ig__igout_dn8_slot;
        let mut var_fn462_calc_ig__isrecout: f64 = *var_fn462_calc_ig__isrecout_slot;
        let mut var_fn462_calc_ig__isrecout_dn4: f64 = *var_fn462_calc_ig__isrecout_dn4_slot;
        let mut var_fn462_calc_ig__return: f64 = *var_fn462_calc_ig__return_slot;
        let mut var_fn462_calc_ig__return_dn4: f64 = *var_fn462_calc_ig__return_dn4_slot;
        let mut var_fn462_calc_ig__return_dn7: f64 = *var_fn462_calc_ig__return_dn7_slot;
        let mut var_fn462_calc_ig__return_dn8: f64 = *var_fn462_calc_ig__return_dn8_slot;
        let mut var_fn468_calc_ig__alpha2_phit: f64 = *var_fn468_calc_ig__alpha2_phit_slot;
        let mut var_fn468_calc_ig__alpha2_phit_dn4: f64 = *var_fn468_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn468_calc_ig__alphagin: f64 = *var_fn468_calc_ig__alphagin_slot;
        let mut var_fn468_calc_ig__betarecin: f64 = *var_fn468_calc_ig__betarecin_slot;
        let mut var_fn468_calc_ig__fracin: f64 = *var_fn468_calc_ig__fracin_slot;
        let mut var_fn468_calc_ig__igout: f64 = *var_fn468_calc_ig__igout_slot;
        let mut var_fn468_calc_ig__igout_dn4: f64 = *var_fn468_calc_ig__igout_dn4_slot;
        let mut var_fn468_calc_ig__igout_dn7: f64 = *var_fn468_calc_ig__igout_dn7_slot;
        let mut var_fn468_calc_ig__igout_dn8: f64 = *var_fn468_calc_ig__igout_dn8_slot;
        let mut var_fn468_calc_ig__ijin: f64 = *var_fn468_calc_ig__ijin_slot;
        let mut var_fn468_calc_ig__irecin: f64 = *var_fn468_calc_ig__irecin_slot;
        let mut var_fn468_calc_ig__isdiodeout: f64 = *var_fn468_calc_ig__isdiodeout_slot;
        let mut var_fn468_calc_ig__isdiodeout_dn4: f64 = *var_fn468_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn468_calc_ig__isrecout: f64 = *var_fn468_calc_ig__isrecout_slot;
        let mut var_fn468_calc_ig__isrecout_dn4: f64 = *var_fn468_calc_ig__isrecout_dn4_slot;
        let mut var_fn468_calc_ig__kbdgatein: f64 = *var_fn468_calc_ig__kbdgatein_slot;
        let mut var_fn468_calc_ig__ngf: f64 = *var_fn468_calc_ig__ngf_slot;
        let mut var_fn468_calc_ig__pbdgin: f64 = *var_fn468_calc_ig__pbdgin_slot;
        let mut var_fn468_calc_ig__pg_param1: f64 = *var_fn468_calc_ig__pg_param1_slot;
        let mut var_fn468_calc_ig__pg_paramin: f64 = *var_fn468_calc_ig__pg_paramin_slot;
        let mut var_fn468_calc_ig__pgsrecin: f64 = *var_fn468_calc_ig__pgsrecin_slot;
        let mut var_fn468_calc_ig__phitin: f64 = *var_fn468_calc_ig__phitin_slot;
        let mut var_fn468_calc_ig__phitin_dn4: f64 = *var_fn468_calc_ig__phitin_dn4_slot;
        let mut var_fn468_calc_ig__return: f64 = *var_fn468_calc_ig__return_slot;
        let mut var_fn468_calc_ig__return_dn4: f64 = *var_fn468_calc_ig__return_dn4_slot;
        let mut var_fn468_calc_ig__return_dn7: f64 = *var_fn468_calc_ig__return_dn7_slot;
        let mut var_fn468_calc_ig__return_dn8: f64 = *var_fn468_calc_ig__return_dn8_slot;
        let mut var_fn468_calc_ig__t0: f64 = *var_fn468_calc_ig__t0_slot;
        let mut var_fn468_calc_ig__t0_dn4: f64 = *var_fn468_calc_ig__t0_dn4_slot;
        let mut var_fn468_calc_ig__tfacdiodein: f64 = *var_fn468_calc_ig__tfacdiodein_slot;
        let mut var_fn468_calc_ig__tfacdiodein_dn4: f64 = *var_fn468_calc_ig__tfacdiodein_dn4_slot;
        let mut var_fn468_calc_ig__type: f64 = *var_fn468_calc_ig__type_slot;
        let mut var_fn468_calc_ig__vbdgin: f64 = *var_fn468_calc_ig__vbdgin_slot;
        let mut var_fn468_calc_ig__vgin: f64 = *var_fn468_calc_ig__vgin_slot;
        let mut var_fn468_calc_ig__vgin_dn7: f64 = *var_fn468_calc_ig__vgin_dn7_slot;
        let mut var_fn468_calc_ig__vgin_dn8: f64 = *var_fn468_calc_ig__vgin_dn8_slot;
        let mut var_fn468_calc_ig__vgsatin: f64 = *var_fn468_calc_ig__vgsatin_slot;
        let mut var_fn468_calc_ig__vgsatqin: f64 = *var_fn468_calc_ig__vgsatqin_slot;
        let mut var_fn468_calc_ig__vjg: f64 = *var_fn468_calc_ig__vjg_slot;
        let mut var_fn468_calc_ig__w: f64 = *var_fn468_calc_ig__w_slot;
        let mut var_guard465: f64 = *var_guard465_slot;
        let mut var_guard466: f64 = *var_guard466_slot;
        let mut var_guard467: f64 = *var_guard467_slot;
        let mut var_idsch: f64 = *var_idsch_slot;
        let mut var_idsch_dn4: f64 = *var_idsch_dn4_slot;
        let mut var_idsch_dn7: f64 = *var_idsch_dn7_slot;
        let mut var_idsch_dn8: f64 = *var_idsch_dn8_slot;

        let (assign42400_e40738, assign42400_e40738_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard463 == 0.0)) {
        let assign42400_e40734: f64 = (var_fn462_calc_ig__alphagin * var_fn462_calc_ig__alphagin);
        let assign42400_e40736: f64 = (assign42400_e40734 * var_fn462_calc_ig__phitin);
        (assign42400_e40736, (assign42400_e40734 * var_fn462_calc_ig__phitin_dn4),)
    } else {
        (var_fn462_calc_ig__alpha2_phit, var_fn462_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn462_calc_ig__alpha2_phit = assign42400_e40738;
        var_fn462_calc_ig__alpha2_phit_dn4 = assign42400_e40738_d_n4;

        let (assign42410_e40753, assign42410_e40753_d_n4, assign42410_e40753_d_n7, assign42410_e40753_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard463 == 0.0)) {
        let assign42410_e40747: f64 = (var_fn462_calc_ig__alpha2_phit / 2.0);
        let assign42410_e40748: f64 = (var_fn462_calc_ig__vgsatin - assign42410_e40747);
        let assign42410_e40749: f64 = (var_fn462_calc_ig__vgin - assign42410_e40748);
        let assign42410_e40751: f64 = (assign42410_e40749 / var_fn462_calc_ig__alpha2_phit);
        (assign42410_e40751, ((((-(-(var_fn462_calc_ig__alpha2_phit_dn4 / 2.0))) * var_fn462_calc_ig__alpha2_phit) - (assign42410_e40749 * var_fn462_calc_ig__alpha2_phit_dn4)) / (var_fn462_calc_ig__alpha2_phit * var_fn462_calc_ig__alpha2_phit)), (var_fn462_calc_ig__vgin_dn7 / var_fn462_calc_ig__alpha2_phit), (var_fn462_calc_ig__vgin_dn8 / var_fn462_calc_ig__alpha2_phit),)
    } else {
        (var_fn462_calc_ig__expffvarg, var_fn462_calc_ig__expffvarg_dn4, var_fn462_calc_ig__expffvarg_dn7, var_fn462_calc_ig__expffvarg_dn8,)
    }
};
        var_fn462_calc_ig__expffvarg = assign42410_e40753;
        var_fn462_calc_ig__expffvarg_dn4 = assign42410_e40753_d_n4;
        var_fn462_calc_ig__expffvarg_dn7 = assign42410_e40753_d_n7;
        var_fn462_calc_ig__expffvarg_dn8 = assign42410_e40753_d_n8;

        let assign42420_e40756: f64 = if var_fn462_calc_ig__expffvarg > 50.0 { 1.0 } else { 0.0 };
        var_guard465 = assign42420_e40756;

        let (assign42430_e40765, assign42430_e40765_d_n4, assign42430_e40765_d_n7, assign42430_e40765_d_n8,) = {
    if (((var_guard461 != 0.0) && (var_guard463 == 0.0)) && (var_guard465 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__ffvgin, var_fn462_calc_ig__ffvgin_dn4, var_fn462_calc_ig__ffvgin_dn7, var_fn462_calc_ig__ffvgin_dn8,)
    }
};
        var_fn462_calc_ig__ffvgin = assign42430_e40765;
        var_fn462_calc_ig__ffvgin_dn4 = assign42430_e40765_d_n4;
        var_fn462_calc_ig__ffvgin_dn7 = assign42430_e40765_d_n7;
        var_fn462_calc_ig__ffvgin_dn8 = assign42430_e40765_d_n8;

        let assign42440_e40768: f64 = (-50.0);
        let assign42440_e40769: f64 = if var_fn462_calc_ig__expffvarg < assign42440_e40768 { 1.0 } else { 0.0 };
        var_guard466 = assign42440_e40769;

        let (assign42450_e40781, assign42450_e40781_d_n4, assign42450_e40781_d_n7, assign42450_e40781_d_n8,) = {
    if ((((var_guard461 != 0.0) && (var_guard463 == 0.0)) && (var_guard465 == 0.0)) && (var_guard466 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn462_calc_ig__ffvgin, var_fn462_calc_ig__ffvgin_dn4, var_fn462_calc_ig__ffvgin_dn7, var_fn462_calc_ig__ffvgin_dn8,)
    }
};
        var_fn462_calc_ig__ffvgin = assign42450_e40781;
        var_fn462_calc_ig__ffvgin_dn4 = assign42450_e40781_d_n4;
        var_fn462_calc_ig__ffvgin_dn7 = assign42450_e40781_d_n7;
        var_fn462_calc_ig__ffvgin_dn8 = assign42450_e40781_d_n8;

        let (assign42460_e40799, assign42460_e40799_d_n4, assign42460_e40799_d_n7, assign42460_e40799_d_n8,) = {
    if ((((var_guard461 != 0.0) && (var_guard463 == 0.0)) && (var_guard465 == 0.0)) && (var_guard466 == 0.0)) {
        let assign42460_e40795: f64 = (var_fn462_calc_ig__expffvarg).exp();
        let assign42460_e40796: f64 = (1.0 + assign42460_e40795);
        let assign42460_e40797: f64 = (1.0 / assign42460_e40796);
        (assign42460_e40797, (-((assign42460_e40795 * var_fn462_calc_ig__expffvarg_dn4) / (assign42460_e40796 * assign42460_e40796))), (-((assign42460_e40795 * var_fn462_calc_ig__expffvarg_dn7) / (assign42460_e40796 * assign42460_e40796))), (-((assign42460_e40795 * var_fn462_calc_ig__expffvarg_dn8) / (assign42460_e40796 * assign42460_e40796))),)
    } else {
        (var_fn462_calc_ig__ffvgin, var_fn462_calc_ig__ffvgin_dn4, var_fn462_calc_ig__ffvgin_dn7, var_fn462_calc_ig__ffvgin_dn8,)
    }
};
        var_fn462_calc_ig__ffvgin = assign42460_e40799;
        var_fn462_calc_ig__ffvgin_dn4 = assign42460_e40799_d_n4;
        var_fn462_calc_ig__ffvgin_dn7 = assign42460_e40799_d_n7;
        var_fn462_calc_ig__ffvgin_dn8 = assign42460_e40799_d_n8;

        let (assign42470_e40814, assign42470_e40814_d_n4, assign42470_e40814_d_n7, assign42470_e40814_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard463 == 0.0)) {
        let assign42470_e40806: f64 = (var_fn462_calc_ig__ffvgin * var_fn462_calc_ig__igindiode_nohinj);
        let assign42470_e40809: f64 = (1.0 - var_fn462_calc_ig__ffvgin);
        let assign42470_e40811: f64 = (assign42470_e40809 * var_fn462_calc_ig__igindiode_hinj);
        let assign42470_e40812: f64 = (assign42470_e40806 + assign42470_e40811);
        (assign42470_e40812, (((var_fn462_calc_ig__ffvgin_dn4 * var_fn462_calc_ig__igindiode_nohinj) + (var_fn462_calc_ig__ffvgin * var_fn462_calc_ig__igindiode_nohinj_dn4)) + (((-var_fn462_calc_ig__ffvgin_dn4) * var_fn462_calc_ig__igindiode_hinj) + (assign42470_e40809 * var_fn462_calc_ig__igindiode_hinj_dn4))), (((var_fn462_calc_ig__ffvgin_dn7 * var_fn462_calc_ig__igindiode_nohinj) + (var_fn462_calc_ig__ffvgin * var_fn462_calc_ig__igindiode_nohinj_dn7)) + (((-var_fn462_calc_ig__ffvgin_dn7) * var_fn462_calc_ig__igindiode_hinj) + (assign42470_e40809 * var_fn462_calc_ig__igindiode_hinj_dn7))), (((var_fn462_calc_ig__ffvgin_dn8 * var_fn462_calc_ig__igindiode_nohinj) + (var_fn462_calc_ig__ffvgin * var_fn462_calc_ig__igindiode_nohinj_dn8)) + (((-var_fn462_calc_ig__ffvgin_dn8) * var_fn462_calc_ig__igindiode_hinj) + (assign42470_e40809 * var_fn462_calc_ig__igindiode_hinj_dn8))),)
    } else {
        (var_fn462_calc_ig__igindiode, var_fn462_calc_ig__igindiode_dn4, var_fn462_calc_ig__igindiode_dn7, var_fn462_calc_ig__igindiode_dn8,)
    }
};
        var_fn462_calc_ig__igindiode = assign42470_e40814;
        var_fn462_calc_ig__igindiode_dn4 = assign42470_e40814_d_n4;
        var_fn462_calc_ig__igindiode_dn7 = assign42470_e40814_d_n7;
        var_fn462_calc_ig__igindiode_dn8 = assign42470_e40814_d_n8;

        let (assign42480_e40860, assign42480_e40860_d_n7, assign42480_e40860_d_n8,) = {
    if (var_guard461 != 0.0) {
        let assign42480_e40817: f64 = (-var_fn462_calc_ig__vgin);
        let (assign42480_e40850, assign42480_e40850_d_n7, assign42480_e40850_d_n8,) = {
            if (p.p52 != 0.0) {
                let assign42480_e40825: f64 = (var_fn462_calc_ig__vgin / var_fn462_calc_ig__vgsatqin);
                let assign42480_e40828: f64 = (0.001 / p.p53);
                let assign42480_e40831: f64 = (var_fn462_calc_ig__vgin / var_fn462_calc_ig__vgsatqin);
                let assign42480_e40832: f64 = (assign42480_e40828 * assign42480_e40831);
                let assign42480_e40833: f64 = (assign42480_e40832).tanh();
                let assign42480_e40834: f64 = (assign42480_e40825 * assign42480_e40833);
                (assign42480_e40834, (((var_fn462_calc_ig__vgin_dn7 / var_fn462_calc_ig__vgsatqin) * assign42480_e40833) + (assign42480_e40825 * ((assign42480_e40828 * (var_fn462_calc_ig__vgin_dn7 / var_fn462_calc_ig__vgsatqin)) / ((assign42480_e40832).cosh() * (assign42480_e40832).cosh())))), (((var_fn462_calc_ig__vgin_dn8 / var_fn462_calc_ig__vgsatqin) * assign42480_e40833) + (assign42480_e40825 * ((assign42480_e40828 * (var_fn462_calc_ig__vgin_dn8 / var_fn462_calc_ig__vgsatqin)) / ((assign42480_e40832).cosh() * (assign42480_e40832).cosh())))),)
            } else {
                let (assign42480_e40849, assign42480_e40849_d_n7, assign42480_e40849_d_n8,) = {
                    if (p.p52 == 0.0) {
                        let __rspice_inv_cse_0: f64 = 1.0 / var_fn462_calc_ig__vgsatqin;
                        let assign42480_e40840: f64 = (var_fn462_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign42480_e40843: f64 = (var_fn462_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign42480_e40844: f64 = (assign42480_e40840 * assign42480_e40843);
                        let assign42480_e40846: f64 = (assign42480_e40844 + p.p53);
                        let assign42480_e40847: f64 = (assign42480_e40846).sqrt();
                        (assign42480_e40847, ((((var_fn462_calc_ig__vgin_dn7 / var_fn462_calc_ig__vgsatqin) * assign42480_e40843) + (assign42480_e40840 * (var_fn462_calc_ig__vgin_dn7 / var_fn462_calc_ig__vgsatqin))) / (2.0 * assign42480_e40847)), ((((var_fn462_calc_ig__vgin_dn8 / var_fn462_calc_ig__vgsatqin) * assign42480_e40843) + (assign42480_e40840 * (var_fn462_calc_ig__vgin_dn8 / var_fn462_calc_ig__vgsatqin))) / (2.0 * assign42480_e40847)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign42480_e40849, assign42480_e40849_d_n7, assign42480_e40849_d_n8,)
            }
        };
        let assign42480_e40852: f64 = (assign42480_e40850).powf(var_fn462_calc_ig__betarecin);
        let assign42480_e40853: f64 = (1.0 + assign42480_e40852);
        let assign42480_e40856: f64 = (1.0 / var_fn462_calc_ig__betarecin);
        let assign42480_e40857: f64 = (assign42480_e40853).powf(assign42480_e40856);
        let assign42480_e40858: f64 = (assign42480_e40817 / assign42480_e40857);
        (assign42480_e40858, ((((-var_fn462_calc_ig__vgin_dn7) * assign42480_e40857) - (assign42480_e40817 * if 0.0 == 0.0 && ((assign42480_e40856) as f64).is_finite() && ((assign42480_e40856) as f64).fract() == 0.0 { if assign42480_e40856 == 0.0 { 0.0 } else { (assign42480_e40856 * ((assign42480_e40853).powf(assign42480_e40856 - 1.0) * if 0.0 == 0.0 && ((var_fn462_calc_ig__betarecin) as f64).is_finite() && ((var_fn462_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn462_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn462_calc_ig__betarecin * ((assign42480_e40850).powf(var_fn462_calc_ig__betarecin - 1.0) * assign42480_e40850_d_n7)) } } else { (assign42480_e40852 * (var_fn462_calc_ig__betarecin * (assign42480_e40850_d_n7 / assign42480_e40850))) })) } } else { (assign42480_e40857 * (assign42480_e40856 * (if 0.0 == 0.0 && ((var_fn462_calc_ig__betarecin) as f64).is_finite() && ((var_fn462_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn462_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn462_calc_ig__betarecin * ((assign42480_e40850).powf(var_fn462_calc_ig__betarecin - 1.0) * assign42480_e40850_d_n7)) } } else { (assign42480_e40852 * (var_fn462_calc_ig__betarecin * (assign42480_e40850_d_n7 / assign42480_e40850))) } / assign42480_e40853))) })) / (assign42480_e40857 * assign42480_e40857)), ((((-var_fn462_calc_ig__vgin_dn8) * assign42480_e40857) - (assign42480_e40817 * if 0.0 == 0.0 && ((assign42480_e40856) as f64).is_finite() && ((assign42480_e40856) as f64).fract() == 0.0 { if assign42480_e40856 == 0.0 { 0.0 } else { (assign42480_e40856 * ((assign42480_e40853).powf(assign42480_e40856 - 1.0) * if 0.0 == 0.0 && ((var_fn462_calc_ig__betarecin) as f64).is_finite() && ((var_fn462_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn462_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn462_calc_ig__betarecin * ((assign42480_e40850).powf(var_fn462_calc_ig__betarecin - 1.0) * assign42480_e40850_d_n8)) } } else { (assign42480_e40852 * (var_fn462_calc_ig__betarecin * (assign42480_e40850_d_n8 / assign42480_e40850))) })) } } else { (assign42480_e40857 * (assign42480_e40856 * (if 0.0 == 0.0 && ((var_fn462_calc_ig__betarecin) as f64).is_finite() && ((var_fn462_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn462_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn462_calc_ig__betarecin * ((assign42480_e40850).powf(var_fn462_calc_ig__betarecin - 1.0) * assign42480_e40850_d_n8)) } } else { (assign42480_e40852 * (var_fn462_calc_ig__betarecin * (assign42480_e40850_d_n8 / assign42480_e40850))) } / assign42480_e40853))) })) / (assign42480_e40857 * assign42480_e40857)),)
    } else {
        (var_fn462_calc_ig__frecgin, var_fn462_calc_ig__frecgin_dn7, var_fn462_calc_ig__frecgin_dn8,)
    }
};
        var_fn462_calc_ig__frecgin = assign42480_e40860;
        var_fn462_calc_ig__frecgin_dn7 = assign42480_e40860_d_n7;
        var_fn462_calc_ig__frecgin_dn8 = assign42480_e40860_d_n8;

        let (assign42490_e40875, assign42490_e40875_d_n4,) = {
    if (var_guard461 != 0.0) {
        let assign42490_e40863: f64 = (-var_fn462_calc_ig__type);
        let assign42490_e40865: f64 = (assign42490_e40863 * var_fn462_calc_ig__w);
        let assign42490_e40867: f64 = (assign42490_e40865 * var_fn462_calc_ig__ngf);
        let assign42490_e40869: f64 = (assign42490_e40867 * var_fn462_calc_ig__irecin);
        let assign42490_e40871: f64 = (assign42490_e40869 * var_fn462_calc_ig__tfacdiodein);
        let assign42490_e40873: f64 = assign42490_e40871;
        (assign42490_e40873, (assign42490_e40869 * var_fn462_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn462_calc_ig__isrecout, var_fn462_calc_ig__isrecout_dn4,)
    }
};
        var_fn462_calc_ig__isrecout = assign42490_e40875;
        var_fn462_calc_ig__isrecout_dn4 = assign42490_e40875_d_n4;

        let (assign42500_e40883, assign42500_e40883_d_n4, assign42500_e40883_d_n7, assign42500_e40883_d_n8,) = {
    if (var_guard461 != 0.0) {
        let assign42500_e40879: f64 = (var_fn462_calc_ig__pgsrecin / var_fn462_calc_ig__phitin);
        let assign42500_e40881: f64 = (assign42500_e40879 * var_fn462_calc_ig__frecgin);
        (assign42500_e40881, ((-((var_fn462_calc_ig__pgsrecin * var_fn462_calc_ig__phitin_dn4) / (var_fn462_calc_ig__phitin * var_fn462_calc_ig__phitin))) * var_fn462_calc_ig__frecgin), (assign42500_e40879 * var_fn462_calc_ig__frecgin_dn7), (assign42500_e40879 * var_fn462_calc_ig__frecgin_dn8),)
    } else {
        (var_fn462_calc_ig__expirevarg, var_fn462_calc_ig__expirevarg_dn4, var_fn462_calc_ig__expirevarg_dn7, var_fn462_calc_ig__expirevarg_dn8,)
    }
};
        var_fn462_calc_ig__expirevarg = assign42500_e40883;
        var_fn462_calc_ig__expirevarg_dn4 = assign42500_e40883_d_n4;
        var_fn462_calc_ig__expirevarg_dn7 = assign42500_e40883_d_n7;
        var_fn462_calc_ig__expirevarg_dn8 = assign42500_e40883_d_n8;

        let (assign42510_e40925, assign42510_e40925_d_n4, assign42510_e40925_d_n7, assign42510_e40925_d_n8,) = {
    if (var_guard461 != 0.0) {
        let assign42510_e40891: f64 = (-50.0);
        let (assign42510_e40923, assign42510_e40923_d_n4, assign42510_e40923_d_n7, assign42510_e40923_d_n8,) = {
            if ((!(var_fn462_calc_ig__expirevarg > 50.0)) && (!(var_fn462_calc_ig__expirevarg < assign42510_e40891))) {
                let assign42510_e40896: f64 = (var_fn462_calc_ig__expirevarg).exp();
                (assign42510_e40896, (assign42510_e40896 * var_fn462_calc_ig__expirevarg_dn4), (assign42510_e40896 * var_fn462_calc_ig__expirevarg_dn7), (assign42510_e40896 * var_fn462_calc_ig__expirevarg_dn8),)
            } else {
                let assign42510_e40903: f64 = (-50.0);
                let (assign42510_e40922, assign42510_e40922_d_n4, assign42510_e40922_d_n7, assign42510_e40922_d_n8,) = {
                    if ((!(var_fn462_calc_ig__expirevarg > 50.0)) && (var_fn462_calc_ig__expirevarg < assign42510_e40903)) {
                        let assign42510_e40907: f64 = (-50.0);
                        let assign42510_e40908: f64 = (assign42510_e40907).exp();
                        (assign42510_e40908, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign42510_e40921, assign42510_e40921_d_n4, assign42510_e40921_d_n7, assign42510_e40921_d_n8,) = {
                            if (var_fn462_calc_ig__expirevarg > 50.0) {
                                let assign42510_e40913: f64 = (50.0_f64).exp();
                                let assign42510_e40917: f64 = (var_fn462_calc_ig__expirevarg - 50.0);
                                let assign42510_e40918: f64 = (1.0 + assign42510_e40917);
                                let assign42510_e40919: f64 = (assign42510_e40913 * assign42510_e40918);
                                (assign42510_e40919, (assign42510_e40913 * var_fn462_calc_ig__expirevarg_dn4), (assign42510_e40913 * var_fn462_calc_ig__expirevarg_dn7), (assign42510_e40913 * var_fn462_calc_ig__expirevarg_dn8),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign42510_e40921, assign42510_e40921_d_n4, assign42510_e40921_d_n7, assign42510_e40921_d_n8,)
                    }
                };
                (assign42510_e40922, assign42510_e40922_d_n4, assign42510_e40922_d_n7, assign42510_e40922_d_n8,)
            }
        };
        (assign42510_e40923, assign42510_e40923_d_n4, assign42510_e40923_d_n7, assign42510_e40923_d_n8,)
    } else {
        (var_fn462_calc_ig__expirev, var_fn462_calc_ig__expirev_dn4, var_fn462_calc_ig__expirev_dn7, var_fn462_calc_ig__expirev_dn8,)
    }
};
        var_fn462_calc_ig__expirev = assign42510_e40925;
        var_fn462_calc_ig__expirev_dn4 = assign42510_e40925_d_n4;
        var_fn462_calc_ig__expirev_dn7 = assign42510_e40925_d_n7;
        var_fn462_calc_ig__expirev_dn8 = assign42510_e40925_d_n8;

        let (assign42520_e40933, assign42520_e40933_d_n4, assign42520_e40933_d_n7, assign42520_e40933_d_n8,) = {
    if (var_guard461 != 0.0) {
        let assign42520_e40930: f64 = (var_fn462_calc_ig__expirev - 1.0);
        let assign42520_e40931: f64 = (var_fn462_calc_ig__isrecout * assign42520_e40930);
        (assign42520_e40931, ((var_fn462_calc_ig__isrecout_dn4 * assign42520_e40930) + (var_fn462_calc_ig__isrecout * var_fn462_calc_ig__expirev_dn4)), (var_fn462_calc_ig__isrecout * var_fn462_calc_ig__expirev_dn7), (var_fn462_calc_ig__isrecout * var_fn462_calc_ig__expirev_dn8),)
    } else {
        (var_fn462_calc_ig__iginrec, var_fn462_calc_ig__iginrec_dn4, var_fn462_calc_ig__iginrec_dn7, var_fn462_calc_ig__iginrec_dn8,)
    }
};
        var_fn462_calc_ig__iginrec = assign42520_e40933;
        var_fn462_calc_ig__iginrec_dn4 = assign42520_e40933_d_n4;
        var_fn462_calc_ig__iginrec_dn7 = assign42520_e40933_d_n7;
        var_fn462_calc_ig__iginrec_dn8 = assign42520_e40933_d_n8;

        let (assign42530_e40939, assign42530_e40939_d_n4, assign42530_e40939_d_n7, assign42530_e40939_d_n8,) = {
    if (var_guard461 != 0.0) {
        let assign42530_e40937: f64 = (var_fn462_calc_ig__igindiode + var_fn462_calc_ig__iginrec);
        (assign42530_e40937, (var_fn462_calc_ig__igindiode_dn4 + var_fn462_calc_ig__iginrec_dn4), (var_fn462_calc_ig__igindiode_dn7 + var_fn462_calc_ig__iginrec_dn7), (var_fn462_calc_ig__igindiode_dn8 + var_fn462_calc_ig__iginrec_dn8),)
    } else {
        (var_fn462_calc_ig__igout, var_fn462_calc_ig__igout_dn4, var_fn462_calc_ig__igout_dn7, var_fn462_calc_ig__igout_dn8,)
    }
};
        var_fn462_calc_ig__igout = assign42530_e40939;
        var_fn462_calc_ig__igout_dn4 = assign42530_e40939_d_n4;
        var_fn462_calc_ig__igout_dn7 = assign42530_e40939_d_n7;
        var_fn462_calc_ig__igout_dn8 = assign42530_e40939_d_n8;

        let (assign42540_e40943, assign42540_e40943_d_n4, assign42540_e40943_d_n7, assign42540_e40943_d_n8,) = {
    if (var_guard461 != 0.0) {
        (var_fn462_calc_ig__igout, var_fn462_calc_ig__igout_dn4, var_fn462_calc_ig__igout_dn7, var_fn462_calc_ig__igout_dn8,)
    } else {
        (var_fn462_calc_ig__return, var_fn462_calc_ig__return_dn4, var_fn462_calc_ig__return_dn7, var_fn462_calc_ig__return_dn8,)
    }
};
        var_fn462_calc_ig__return = assign42540_e40943;
        var_fn462_calc_ig__return_dn4 = assign42540_e40943_d_n4;
        var_fn462_calc_ig__return_dn7 = assign42540_e40943_d_n7;
        var_fn462_calc_ig__return_dn8 = assign42540_e40943_d_n8;

        let (assign42570_e40955, assign42570_e40955_d_n4, assign42570_e40955_d_n7, assign42570_e40955_d_n8,) = {
    if (var_guard461 != 0.0) {
        (var_fn462_calc_ig__return, var_fn462_calc_ig__return_dn4, var_fn462_calc_ig__return_dn7, var_fn462_calc_ig__return_dn8,)
    } else {
        (var_idsch, var_idsch_dn4, var_idsch_dn7, var_idsch_dn8,)
    }
};
        var_idsch = assign42570_e40955;
        var_idsch_dn4 = assign42570_e40955_d_n4;
        var_idsch_dn7 = assign42570_e40955_d_n7;
        var_idsch_dn8 = assign42570_e40955_d_n8;

        let assign42580_e40958: f64 = if p.p301 == 1.0 { 1.0 } else { 0.0 };
        var_guard467 = assign42580_e40958;

        let (assign42590_e40964, assign42590_e40964_d_n4, assign42590_e40964_d_n7, assign42590_e40964_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__return, var_fn468_calc_ig__return_dn4, var_fn468_calc_ig__return_dn7, var_fn468_calc_ig__return_dn8,)
    }
};
        var_fn468_calc_ig__return = assign42590_e40964;
        var_fn468_calc_ig__return_dn4 = assign42590_e40964_d_n4;
        var_fn468_calc_ig__return_dn7 = assign42590_e40964_d_n7;
        var_fn468_calc_ig__return_dn8 = assign42590_e40964_d_n8;

        let (assign42600_e40970, assign42600_e40970_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__isdiodeout, var_fn468_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn468_calc_ig__isdiodeout = assign42600_e40970;
        var_fn468_calc_ig__isdiodeout_dn4 = assign42600_e40970_d_n4;

        let (assign42610_e40976, assign42610_e40976_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__isrecout, var_fn468_calc_ig__isrecout_dn4,)
    }
};
        var_fn468_calc_ig__isrecout = assign42610_e40976;
        var_fn468_calc_ig__isrecout_dn4 = assign42610_e40976_d_n4;

        let (assign42620_e40982, assign42620_e40982_d_n7, assign42620_e40982_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (var_vsch, var_vsch_dn7, var_vsch_dn8,)
    } else {
        (var_fn468_calc_ig__vgin, var_fn468_calc_ig__vgin_dn7, var_fn468_calc_ig__vgin_dn8,)
    }
};
        var_fn468_calc_ig__vgin = assign42620_e40982;
        var_fn468_calc_ig__vgin_dn7 = assign42620_e40982_d_n7;
        var_fn468_calc_ig__vgin_dn8 = assign42620_e40982_d_n8;

        let (assign42630_e40988, assign42630_e40988_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (var_phit, var_phit_dn4,)
    } else {
        (var_fn468_calc_ig__phitin, var_fn468_calc_ig__phitin_dn4,)
    }
};
        var_fn468_calc_ig__phitin = assign42630_e40988;
        var_fn468_calc_ig__phitin_dn4 = assign42630_e40988_d_n4;

        let (assign42640_e40994,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (1.0,)
    } else {
        (var_fn468_calc_ig__vgsatin,)
    }
};
        var_fn468_calc_ig__vgsatin = assign42640_e40994;

        let (assign42650_e41000,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (10.0,)
    } else {
        (var_fn468_calc_ig__alphagin,)
    }
};
        var_fn468_calc_ig__alphagin = assign42650_e41000;

        let (assign42660_e41006,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (1.0,)
    } else {
        (var_fn468_calc_ig__fracin,)
    }
};
        var_fn468_calc_ig__fracin = assign42660_e41006;

        let (assign42670_e41012,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0,)
    } else {
        (var_fn468_calc_ig__pg_paramin,)
    }
};
        var_fn468_calc_ig__pg_paramin = assign42670_e41012;

        let (assign42680_e41018,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (4.0,)
    } else {
        (var_fn468_calc_ig__pbdgin,)
    }
};
        var_fn468_calc_ig__pbdgin = assign42680_e41018;

        let (assign42690_e41024,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (600.0,)
    } else {
        (var_fn468_calc_ig__vbdgin,)
    }
};
        var_fn468_calc_ig__vbdgin = assign42690_e41024;

        let (assign42700_e41030, assign42700_e41030_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (var_tfacdiode, var_tfacdiode_dn4,)
    } else {
        (var_fn468_calc_ig__tfacdiodein, var_fn468_calc_ig__tfacdiodein_dn4,)
    }
};
        var_fn468_calc_ig__tfacdiodein = assign42700_e41030;
        var_fn468_calc_ig__tfacdiodein_dn4 = assign42700_e41030_d_n4;

        let (assign42710_e41040,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign42710_e41037: f64 = (1.0 - p.p311);
        let assign42710_e41038: f64 = (p.p0 * assign42710_e41037);
        (assign42710_e41038,)
    } else {
        (var_fn468_calc_ig__w,)
    }
};
        var_fn468_calc_ig__w = assign42710_e41040;

        let (assign42720_e41046,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (p.p2,)
    } else {
        (var_fn468_calc_ig__ngf,)
    }
};
        var_fn468_calc_ig__ngf = assign42720_e41046;

        let (assign42730_e41052,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0,)
    } else {
        (var_fn468_calc_ig__ijin,)
    }
};
        var_fn468_calc_ig__ijin = assign42730_e41052;

        let (assign42740_e41058,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0,)
    } else {
        (var_fn468_calc_ig__kbdgatein,)
    }
};
        var_fn468_calc_ig__kbdgatein = assign42740_e41058;

        let (assign42750_e41064,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (p.p304,)
    } else {
        (var_fn468_calc_ig__vgsatqin,)
    }
};
        var_fn468_calc_ig__vgsatqin = assign42750_e41064;

        let (assign42760_e41070,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (p.p305,)
    } else {
        (var_fn468_calc_ig__betarecin,)
    }
};
        var_fn468_calc_ig__betarecin = assign42760_e41070;

        let (assign42770_e41076,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (p.p303,)
    } else {
        (var_fn468_calc_ig__irecin,)
    }
};
        var_fn468_calc_ig__irecin = assign42770_e41076;

        let (assign42780_e41082,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (p.p302,)
    } else {
        (var_fn468_calc_ig__pgsrecin,)
    }
};
        var_fn468_calc_ig__pgsrecin = assign42780_e41082;

        let (assign42790_e41088,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0,)
    } else {
        (var_fn468_calc_ig__pg_param1,)
    }
};
        var_fn468_calc_ig__pg_param1 = assign42790_e41088;

        let (assign42800_e41094,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0,)
    } else {
        (var_fn468_calc_ig__vjg,)
    }
};
        var_fn468_calc_ig__vjg = assign42800_e41094;

        let (assign42810_e41100,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (p.p6,)
    } else {
        (var_fn468_calc_ig__type,)
    }
};
        var_fn468_calc_ig__type = assign42810_e41100;

        let (assign42820_e41106, assign42820_e41106_d_n4, assign42820_e41106_d_n7, assign42820_e41106_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__igout, var_fn468_calc_ig__igout_dn4, var_fn468_calc_ig__igout_dn7, var_fn468_calc_ig__igout_dn8,)
    }
};
        var_fn468_calc_ig__igout = assign42820_e41106;
        var_fn468_calc_ig__igout_dn4 = assign42820_e41106_d_n4;
        var_fn468_calc_ig__igout_dn7 = assign42820_e41106_d_n7;
        var_fn468_calc_ig__igout_dn8 = assign42820_e41106_d_n8;

        let (assign42830_e41112, assign42830_e41112_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__alpha2_phit, var_fn468_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn468_calc_ig__alpha2_phit = assign42830_e41112;
        var_fn468_calc_ig__alpha2_phit_dn4 = assign42830_e41112_d_n4;

        let (assign42840_e41118, assign42840_e41118_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__t0, var_fn468_calc_ig__t0_dn4,)
    }
};
        var_fn468_calc_ig__t0 = assign42840_e41118;
        var_fn468_calc_ig__t0_dn4 = assign42840_e41118_d_n4;

        *var_fn462_calc_ig__alpha2_phit_slot = var_fn462_calc_ig__alpha2_phit;
        *var_fn462_calc_ig__alpha2_phit_dn4_slot = var_fn462_calc_ig__alpha2_phit_dn4;
        *var_fn462_calc_ig__expffvarg_slot = var_fn462_calc_ig__expffvarg;
        *var_fn462_calc_ig__expffvarg_dn4_slot = var_fn462_calc_ig__expffvarg_dn4;
        *var_fn462_calc_ig__expffvarg_dn7_slot = var_fn462_calc_ig__expffvarg_dn7;
        *var_fn462_calc_ig__expffvarg_dn8_slot = var_fn462_calc_ig__expffvarg_dn8;
        *var_fn462_calc_ig__expirev_slot = var_fn462_calc_ig__expirev;
        *var_fn462_calc_ig__expirev_dn4_slot = var_fn462_calc_ig__expirev_dn4;
        *var_fn462_calc_ig__expirev_dn7_slot = var_fn462_calc_ig__expirev_dn7;
        *var_fn462_calc_ig__expirev_dn8_slot = var_fn462_calc_ig__expirev_dn8;
        *var_fn462_calc_ig__expirevarg_slot = var_fn462_calc_ig__expirevarg;
        *var_fn462_calc_ig__expirevarg_dn4_slot = var_fn462_calc_ig__expirevarg_dn4;
        *var_fn462_calc_ig__expirevarg_dn7_slot = var_fn462_calc_ig__expirevarg_dn7;
        *var_fn462_calc_ig__expirevarg_dn8_slot = var_fn462_calc_ig__expirevarg_dn8;
        *var_fn462_calc_ig__ffvgin_slot = var_fn462_calc_ig__ffvgin;
        *var_fn462_calc_ig__ffvgin_dn4_slot = var_fn462_calc_ig__ffvgin_dn4;
        *var_fn462_calc_ig__ffvgin_dn7_slot = var_fn462_calc_ig__ffvgin_dn7;
        *var_fn462_calc_ig__ffvgin_dn8_slot = var_fn462_calc_ig__ffvgin_dn8;
        *var_fn462_calc_ig__frecgin_slot = var_fn462_calc_ig__frecgin;
        *var_fn462_calc_ig__frecgin_dn7_slot = var_fn462_calc_ig__frecgin_dn7;
        *var_fn462_calc_ig__frecgin_dn8_slot = var_fn462_calc_ig__frecgin_dn8;
        *var_fn462_calc_ig__igindiode_slot = var_fn462_calc_ig__igindiode;
        *var_fn462_calc_ig__igindiode_dn4_slot = var_fn462_calc_ig__igindiode_dn4;
        *var_fn462_calc_ig__igindiode_dn7_slot = var_fn462_calc_ig__igindiode_dn7;
        *var_fn462_calc_ig__igindiode_dn8_slot = var_fn462_calc_ig__igindiode_dn8;
        *var_fn462_calc_ig__iginrec_slot = var_fn462_calc_ig__iginrec;
        *var_fn462_calc_ig__iginrec_dn4_slot = var_fn462_calc_ig__iginrec_dn4;
        *var_fn462_calc_ig__iginrec_dn7_slot = var_fn462_calc_ig__iginrec_dn7;
        *var_fn462_calc_ig__iginrec_dn8_slot = var_fn462_calc_ig__iginrec_dn8;
        *var_fn462_calc_ig__igout_slot = var_fn462_calc_ig__igout;
        *var_fn462_calc_ig__igout_dn4_slot = var_fn462_calc_ig__igout_dn4;
        *var_fn462_calc_ig__igout_dn7_slot = var_fn462_calc_ig__igout_dn7;
        *var_fn462_calc_ig__igout_dn8_slot = var_fn462_calc_ig__igout_dn8;
        *var_fn462_calc_ig__isrecout_slot = var_fn462_calc_ig__isrecout;
        *var_fn462_calc_ig__isrecout_dn4_slot = var_fn462_calc_ig__isrecout_dn4;
        *var_fn462_calc_ig__return_slot = var_fn462_calc_ig__return;
        *var_fn462_calc_ig__return_dn4_slot = var_fn462_calc_ig__return_dn4;
        *var_fn462_calc_ig__return_dn7_slot = var_fn462_calc_ig__return_dn7;
        *var_fn462_calc_ig__return_dn8_slot = var_fn462_calc_ig__return_dn8;
        *var_fn468_calc_ig__alpha2_phit_slot = var_fn468_calc_ig__alpha2_phit;
        *var_fn468_calc_ig__alpha2_phit_dn4_slot = var_fn468_calc_ig__alpha2_phit_dn4;
        *var_fn468_calc_ig__alphagin_slot = var_fn468_calc_ig__alphagin;
        *var_fn468_calc_ig__betarecin_slot = var_fn468_calc_ig__betarecin;
        *var_fn468_calc_ig__fracin_slot = var_fn468_calc_ig__fracin;
        *var_fn468_calc_ig__igout_slot = var_fn468_calc_ig__igout;
        *var_fn468_calc_ig__igout_dn4_slot = var_fn468_calc_ig__igout_dn4;
        *var_fn468_calc_ig__igout_dn7_slot = var_fn468_calc_ig__igout_dn7;
        *var_fn468_calc_ig__igout_dn8_slot = var_fn468_calc_ig__igout_dn8;
        *var_fn468_calc_ig__ijin_slot = var_fn468_calc_ig__ijin;
        *var_fn468_calc_ig__irecin_slot = var_fn468_calc_ig__irecin;
        *var_fn468_calc_ig__isdiodeout_slot = var_fn468_calc_ig__isdiodeout;
        *var_fn468_calc_ig__isdiodeout_dn4_slot = var_fn468_calc_ig__isdiodeout_dn4;
        *var_fn468_calc_ig__isrecout_slot = var_fn468_calc_ig__isrecout;
        *var_fn468_calc_ig__isrecout_dn4_slot = var_fn468_calc_ig__isrecout_dn4;
        *var_fn468_calc_ig__kbdgatein_slot = var_fn468_calc_ig__kbdgatein;
        *var_fn468_calc_ig__ngf_slot = var_fn468_calc_ig__ngf;
        *var_fn468_calc_ig__pbdgin_slot = var_fn468_calc_ig__pbdgin;
        *var_fn468_calc_ig__pg_param1_slot = var_fn468_calc_ig__pg_param1;
        *var_fn468_calc_ig__pg_paramin_slot = var_fn468_calc_ig__pg_paramin;
        *var_fn468_calc_ig__pgsrecin_slot = var_fn468_calc_ig__pgsrecin;
        *var_fn468_calc_ig__phitin_slot = var_fn468_calc_ig__phitin;
        *var_fn468_calc_ig__phitin_dn4_slot = var_fn468_calc_ig__phitin_dn4;
        *var_fn468_calc_ig__return_slot = var_fn468_calc_ig__return;
        *var_fn468_calc_ig__return_dn4_slot = var_fn468_calc_ig__return_dn4;
        *var_fn468_calc_ig__return_dn7_slot = var_fn468_calc_ig__return_dn7;
        *var_fn468_calc_ig__return_dn8_slot = var_fn468_calc_ig__return_dn8;
        *var_fn468_calc_ig__t0_slot = var_fn468_calc_ig__t0;
        *var_fn468_calc_ig__t0_dn4_slot = var_fn468_calc_ig__t0_dn4;
        *var_fn468_calc_ig__tfacdiodein_slot = var_fn468_calc_ig__tfacdiodein;
        *var_fn468_calc_ig__tfacdiodein_dn4_slot = var_fn468_calc_ig__tfacdiodein_dn4;
        *var_fn468_calc_ig__type_slot = var_fn468_calc_ig__type;
        *var_fn468_calc_ig__vbdgin_slot = var_fn468_calc_ig__vbdgin;
        *var_fn468_calc_ig__vgin_slot = var_fn468_calc_ig__vgin;
        *var_fn468_calc_ig__vgin_dn7_slot = var_fn468_calc_ig__vgin_dn7;
        *var_fn468_calc_ig__vgin_dn8_slot = var_fn468_calc_ig__vgin_dn8;
        *var_fn468_calc_ig__vgsatin_slot = var_fn468_calc_ig__vgsatin;
        *var_fn468_calc_ig__vgsatqin_slot = var_fn468_calc_ig__vgsatqin;
        *var_fn468_calc_ig__vjg_slot = var_fn468_calc_ig__vjg;
        *var_fn468_calc_ig__w_slot = var_fn468_calc_ig__w;
        *var_guard465_slot = var_guard465;
        *var_guard466_slot = var_guard466;
        *var_guard467_slot = var_guard467;
        *var_idsch_slot = var_idsch;
        *var_idsch_dn4_slot = var_idsch_dn4;
        *var_idsch_dn7_slot = var_idsch_dn7;
        *var_idsch_dn8_slot = var_idsch_dn8;
    }

    pub(super) fn stamp_transient_block_107(
        var_fn468_calc_ig__pbdgin: f64,
        var_fn468_calc_ig__pg_param1: f64,
        var_fn468_calc_ig__phitin: f64,
        var_fn468_calc_ig__phitin_dn4: f64,
        var_fn468_calc_ig__vbdgin: f64,
        var_fn468_calc_ig__vgin: f64,
        var_fn468_calc_ig__vgin_dn7: f64,
        var_fn468_calc_ig__vgin_dn8: f64,
        var_fn468_calc_ig__vjg: f64,
        var_guard461: f64,
        var_guard467: f64,
        var_fn468_calc_ig__expbd1_slot: &mut f64,
        var_fn468_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn468_calc_ig__expbd1_dn7_slot: &mut f64,
        var_fn468_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn468_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn468_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__expbd2_slot: &mut f64,
        var_fn468_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn468_calc_ig__expbdarg1_slot: &mut f64,
        var_fn468_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn468_calc_ig__expbdarg1_dn7_slot: &mut f64,
        var_fn468_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn468_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn468_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__expbdarg2_slot: &mut f64,
        var_fn468_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn468_calc_ig__expffvarg_slot: &mut f64,
        var_fn468_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn468_calc_ig__expffvarg_dn7_slot: &mut f64,
        var_fn468_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn468_calc_ig__expifor_slot: &mut f64,
        var_fn468_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn468_calc_ig__expifor_dn7_slot: &mut f64,
        var_fn468_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn468_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn468_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn468_calc_ig__expifor_hinj_dn7_slot: &mut f64,
        var_fn468_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn468_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn468_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn468_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_dn7_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_hinj_dn7_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__expirev_slot: &mut f64,
        var_fn468_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn468_calc_ig__expirev_dn7_slot: &mut f64,
        var_fn468_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn468_calc_ig__expirevarg_slot: &mut f64,
        var_fn468_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn468_calc_ig__expirevarg_dn7_slot: &mut f64,
        var_fn468_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn468_calc_ig__expphib_slot: &mut f64,
        var_fn468_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn468_calc_ig__ffvgin_slot: &mut f64,
        var_fn468_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn468_calc_ig__ffvgin_dn7_slot: &mut f64,
        var_fn468_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn468_calc_ig__frecgin_slot: &mut f64,
        var_fn468_calc_ig__frecgin_dn7_slot: &mut f64,
        var_fn468_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn468_calc_ig__iginbd_slot: &mut f64,
        var_fn468_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn468_calc_ig__iginbd_dn7_slot: &mut f64,
        var_fn468_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn468_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn468_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__igindiode_slot: &mut f64,
        var_fn468_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn468_calc_ig__igindiode_dn7_slot: &mut f64,
        var_fn468_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn468_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn468_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn468_calc_ig__igindiode_hinj_dn7_slot: &mut f64,
        var_fn468_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn468_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn468_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn468_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn468_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn468_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn468_calc_ig__igindiode_nohinj_dn7_slot: &mut f64,
        var_fn468_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn468_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn468_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__iginrec_slot: &mut f64,
        var_fn468_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn468_calc_ig__iginrec_dn7_slot: &mut f64,
        var_fn468_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn468_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_fn468_calc_ig__t0_slot: &mut f64,
        var_fn468_calc_ig__t0_dn4_slot: &mut f64,
    ) {
        let mut var_fn468_calc_ig__expbd1: f64 = *var_fn468_calc_ig__expbd1_slot;
        let mut var_fn468_calc_ig__expbd1_dn4: f64 = *var_fn468_calc_ig__expbd1_dn4_slot;
        let mut var_fn468_calc_ig__expbd1_dn7: f64 = *var_fn468_calc_ig__expbd1_dn7_slot;
        let mut var_fn468_calc_ig__expbd1_dn8: f64 = *var_fn468_calc_ig__expbd1_dn8_slot;
        let mut var_fn468_calc_ig__expbd1_vgsat: f64 = *var_fn468_calc_ig__expbd1_vgsat_slot;
        let mut var_fn468_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn468_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__expbd2: f64 = *var_fn468_calc_ig__expbd2_slot;
        let mut var_fn468_calc_ig__expbd2_dn4: f64 = *var_fn468_calc_ig__expbd2_dn4_slot;
        let mut var_fn468_calc_ig__expbdarg1: f64 = *var_fn468_calc_ig__expbdarg1_slot;
        let mut var_fn468_calc_ig__expbdarg1_dn4: f64 = *var_fn468_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn468_calc_ig__expbdarg1_dn7: f64 = *var_fn468_calc_ig__expbdarg1_dn7_slot;
        let mut var_fn468_calc_ig__expbdarg1_dn8: f64 = *var_fn468_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn468_calc_ig__expbdarg1_vgsat: f64 = *var_fn468_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn468_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn468_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__expbdarg2: f64 = *var_fn468_calc_ig__expbdarg2_slot;
        let mut var_fn468_calc_ig__expbdarg2_dn4: f64 = *var_fn468_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn468_calc_ig__expffvarg: f64 = *var_fn468_calc_ig__expffvarg_slot;
        let mut var_fn468_calc_ig__expffvarg_dn4: f64 = *var_fn468_calc_ig__expffvarg_dn4_slot;
        let mut var_fn468_calc_ig__expffvarg_dn7: f64 = *var_fn468_calc_ig__expffvarg_dn7_slot;
        let mut var_fn468_calc_ig__expffvarg_dn8: f64 = *var_fn468_calc_ig__expffvarg_dn8_slot;
        let mut var_fn468_calc_ig__expifor: f64 = *var_fn468_calc_ig__expifor_slot;
        let mut var_fn468_calc_ig__expifor_dn4: f64 = *var_fn468_calc_ig__expifor_dn4_slot;
        let mut var_fn468_calc_ig__expifor_dn7: f64 = *var_fn468_calc_ig__expifor_dn7_slot;
        let mut var_fn468_calc_ig__expifor_dn8: f64 = *var_fn468_calc_ig__expifor_dn8_slot;
        let mut var_fn468_calc_ig__expifor_hinj: f64 = *var_fn468_calc_ig__expifor_hinj_slot;
        let mut var_fn468_calc_ig__expifor_hinj_dn4: f64 = *var_fn468_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn468_calc_ig__expifor_hinj_dn7: f64 = *var_fn468_calc_ig__expifor_hinj_dn7_slot;
        let mut var_fn468_calc_ig__expifor_hinj_dn8: f64 = *var_fn468_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn468_calc_ig__expifor_hinj_vgsat: f64 = *var_fn468_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn468_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn468_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn468_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn468_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn468_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__expiforarg: f64 = *var_fn468_calc_ig__expiforarg_slot;
        let mut var_fn468_calc_ig__expiforarg_dn4: f64 = *var_fn468_calc_ig__expiforarg_dn4_slot;
        let mut var_fn468_calc_ig__expiforarg_dn7: f64 = *var_fn468_calc_ig__expiforarg_dn7_slot;
        let mut var_fn468_calc_ig__expiforarg_dn8: f64 = *var_fn468_calc_ig__expiforarg_dn8_slot;
        let mut var_fn468_calc_ig__expiforarg_hinj: f64 = *var_fn468_calc_ig__expiforarg_hinj_slot;
        let mut var_fn468_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn468_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn468_calc_ig__expiforarg_hinj_dn7: f64 = *var_fn468_calc_ig__expiforarg_hinj_dn7_slot;
        let mut var_fn468_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn468_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn468_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn468_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn468_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn468_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn468_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn468_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn468_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__expirev: f64 = *var_fn468_calc_ig__expirev_slot;
        let mut var_fn468_calc_ig__expirev_dn4: f64 = *var_fn468_calc_ig__expirev_dn4_slot;
        let mut var_fn468_calc_ig__expirev_dn7: f64 = *var_fn468_calc_ig__expirev_dn7_slot;
        let mut var_fn468_calc_ig__expirev_dn8: f64 = *var_fn468_calc_ig__expirev_dn8_slot;
        let mut var_fn468_calc_ig__expirevarg: f64 = *var_fn468_calc_ig__expirevarg_slot;
        let mut var_fn468_calc_ig__expirevarg_dn4: f64 = *var_fn468_calc_ig__expirevarg_dn4_slot;
        let mut var_fn468_calc_ig__expirevarg_dn7: f64 = *var_fn468_calc_ig__expirevarg_dn7_slot;
        let mut var_fn468_calc_ig__expirevarg_dn8: f64 = *var_fn468_calc_ig__expirevarg_dn8_slot;
        let mut var_fn468_calc_ig__expphib: f64 = *var_fn468_calc_ig__expphib_slot;
        let mut var_fn468_calc_ig__expphib_dn4: f64 = *var_fn468_calc_ig__expphib_dn4_slot;
        let mut var_fn468_calc_ig__ffvgin: f64 = *var_fn468_calc_ig__ffvgin_slot;
        let mut var_fn468_calc_ig__ffvgin_dn4: f64 = *var_fn468_calc_ig__ffvgin_dn4_slot;
        let mut var_fn468_calc_ig__ffvgin_dn7: f64 = *var_fn468_calc_ig__ffvgin_dn7_slot;
        let mut var_fn468_calc_ig__ffvgin_dn8: f64 = *var_fn468_calc_ig__ffvgin_dn8_slot;
        let mut var_fn468_calc_ig__frecgin: f64 = *var_fn468_calc_ig__frecgin_slot;
        let mut var_fn468_calc_ig__frecgin_dn7: f64 = *var_fn468_calc_ig__frecgin_dn7_slot;
        let mut var_fn468_calc_ig__frecgin_dn8: f64 = *var_fn468_calc_ig__frecgin_dn8_slot;
        let mut var_fn468_calc_ig__iginbd: f64 = *var_fn468_calc_ig__iginbd_slot;
        let mut var_fn468_calc_ig__iginbd_dn4: f64 = *var_fn468_calc_ig__iginbd_dn4_slot;
        let mut var_fn468_calc_ig__iginbd_dn7: f64 = *var_fn468_calc_ig__iginbd_dn7_slot;
        let mut var_fn468_calc_ig__iginbd_dn8: f64 = *var_fn468_calc_ig__iginbd_dn8_slot;
        let mut var_fn468_calc_ig__iginbd_vgsat: f64 = *var_fn468_calc_ig__iginbd_vgsat_slot;
        let mut var_fn468_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn468_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__igindiode: f64 = *var_fn468_calc_ig__igindiode_slot;
        let mut var_fn468_calc_ig__igindiode_dn4: f64 = *var_fn468_calc_ig__igindiode_dn4_slot;
        let mut var_fn468_calc_ig__igindiode_dn7: f64 = *var_fn468_calc_ig__igindiode_dn7_slot;
        let mut var_fn468_calc_ig__igindiode_dn8: f64 = *var_fn468_calc_ig__igindiode_dn8_slot;
        let mut var_fn468_calc_ig__igindiode_hinj: f64 = *var_fn468_calc_ig__igindiode_hinj_slot;
        let mut var_fn468_calc_ig__igindiode_hinj_dn4: f64 = *var_fn468_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn468_calc_ig__igindiode_hinj_dn7: f64 = *var_fn468_calc_ig__igindiode_hinj_dn7_slot;
        let mut var_fn468_calc_ig__igindiode_hinj_dn8: f64 = *var_fn468_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn468_calc_ig__igindiode_hinj_pre: f64 = *var_fn468_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn468_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn468_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn468_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn468_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn468_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn468_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__igindiode_nohinj: f64 = *var_fn468_calc_ig__igindiode_nohinj_slot;
        let mut var_fn468_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn468_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn468_calc_ig__igindiode_nohinj_dn7: f64 = *var_fn468_calc_ig__igindiode_nohinj_dn7_slot;
        let mut var_fn468_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn468_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn468_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn468_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn468_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn468_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__iginrec: f64 = *var_fn468_calc_ig__iginrec_slot;
        let mut var_fn468_calc_ig__iginrec_dn4: f64 = *var_fn468_calc_ig__iginrec_dn4_slot;
        let mut var_fn468_calc_ig__iginrec_dn7: f64 = *var_fn468_calc_ig__iginrec_dn7_slot;
        let mut var_fn468_calc_ig__iginrec_dn8: f64 = *var_fn468_calc_ig__iginrec_dn8_slot;
        let mut var_fn468_calc_ig__pg_paramin_hinj: f64 = *var_fn468_calc_ig__pg_paramin_hinj_slot;
        let mut var_fn468_calc_ig__t0: f64 = *var_fn468_calc_ig__t0_slot;
        let mut var_fn468_calc_ig__t0_dn4: f64 = *var_fn468_calc_ig__t0_dn4_slot;

        let (assign42850_e41124, assign42850_e41124_d_n4, assign42850_e41124_d_n7, assign42850_e41124_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__ffvgin, var_fn468_calc_ig__ffvgin_dn4, var_fn468_calc_ig__ffvgin_dn7, var_fn468_calc_ig__ffvgin_dn8,)
    }
};
        var_fn468_calc_ig__ffvgin = assign42850_e41124;
        var_fn468_calc_ig__ffvgin_dn4 = assign42850_e41124_d_n4;
        var_fn468_calc_ig__ffvgin_dn7 = assign42850_e41124_d_n7;
        var_fn468_calc_ig__ffvgin_dn8 = assign42850_e41124_d_n8;

        let (assign42860_e41130, assign42860_e41130_d_n4, assign42860_e41130_d_n7, assign42860_e41130_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__iginbd, var_fn468_calc_ig__iginbd_dn4, var_fn468_calc_ig__iginbd_dn7, var_fn468_calc_ig__iginbd_dn8,)
    }
};
        var_fn468_calc_ig__iginbd = assign42860_e41130;
        var_fn468_calc_ig__iginbd_dn4 = assign42860_e41130_d_n4;
        var_fn468_calc_ig__iginbd_dn7 = assign42860_e41130_d_n7;
        var_fn468_calc_ig__iginbd_dn8 = assign42860_e41130_d_n8;

        let (assign42870_e41136, assign42870_e41136_d_n4, assign42870_e41136_d_n7, assign42870_e41136_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__igindiode, var_fn468_calc_ig__igindiode_dn4, var_fn468_calc_ig__igindiode_dn7, var_fn468_calc_ig__igindiode_dn8,)
    }
};
        var_fn468_calc_ig__igindiode = assign42870_e41136;
        var_fn468_calc_ig__igindiode_dn4 = assign42870_e41136_d_n4;
        var_fn468_calc_ig__igindiode_dn7 = assign42870_e41136_d_n7;
        var_fn468_calc_ig__igindiode_dn8 = assign42870_e41136_d_n8;

        let (assign42880_e41142, assign42880_e41142_d_n7, assign42880_e41142_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__frecgin, var_fn468_calc_ig__frecgin_dn7, var_fn468_calc_ig__frecgin_dn8,)
    }
};
        var_fn468_calc_ig__frecgin = assign42880_e41142;
        var_fn468_calc_ig__frecgin_dn7 = assign42880_e41142_d_n7;
        var_fn468_calc_ig__frecgin_dn8 = assign42880_e41142_d_n8;

        let (assign42890_e41148, assign42890_e41148_d_n4, assign42890_e41148_d_n7, assign42890_e41148_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__iginrec, var_fn468_calc_ig__iginrec_dn4, var_fn468_calc_ig__iginrec_dn7, var_fn468_calc_ig__iginrec_dn8,)
    }
};
        var_fn468_calc_ig__iginrec = assign42890_e41148;
        var_fn468_calc_ig__iginrec_dn4 = assign42890_e41148_d_n4;
        var_fn468_calc_ig__iginrec_dn7 = assign42890_e41148_d_n7;
        var_fn468_calc_ig__iginrec_dn8 = assign42890_e41148_d_n8;

        let (assign42900_e41154, assign42900_e41154_d_n4, assign42900_e41154_d_n7, assign42900_e41154_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expbdarg1, var_fn468_calc_ig__expbdarg1_dn4, var_fn468_calc_ig__expbdarg1_dn7, var_fn468_calc_ig__expbdarg1_dn8,)
    }
};
        var_fn468_calc_ig__expbdarg1 = assign42900_e41154;
        var_fn468_calc_ig__expbdarg1_dn4 = assign42900_e41154_d_n4;
        var_fn468_calc_ig__expbdarg1_dn7 = assign42900_e41154_d_n7;
        var_fn468_calc_ig__expbdarg1_dn8 = assign42900_e41154_d_n8;

        let (assign42910_e41160, assign42910_e41160_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expbdarg2, var_fn468_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn468_calc_ig__expbdarg2 = assign42910_e41160;
        var_fn468_calc_ig__expbdarg2_dn4 = assign42910_e41160_d_n4;

        let (assign42920_e41166, assign42920_e41166_d_n4, assign42920_e41166_d_n7, assign42920_e41166_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expbd1, var_fn468_calc_ig__expbd1_dn4, var_fn468_calc_ig__expbd1_dn7, var_fn468_calc_ig__expbd1_dn8,)
    }
};
        var_fn468_calc_ig__expbd1 = assign42920_e41166;
        var_fn468_calc_ig__expbd1_dn4 = assign42920_e41166_d_n4;
        var_fn468_calc_ig__expbd1_dn7 = assign42920_e41166_d_n7;
        var_fn468_calc_ig__expbd1_dn8 = assign42920_e41166_d_n8;

        let (assign42930_e41172, assign42930_e41172_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expbd2, var_fn468_calc_ig__expbd2_dn4,)
    }
};
        var_fn468_calc_ig__expbd2 = assign42930_e41172;
        var_fn468_calc_ig__expbd2_dn4 = assign42930_e41172_d_n4;

        let (assign42940_e41178, assign42940_e41178_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expphib, var_fn468_calc_ig__expphib_dn4,)
    }
};
        var_fn468_calc_ig__expphib = assign42940_e41178;
        var_fn468_calc_ig__expphib_dn4 = assign42940_e41178_d_n4;

        let (assign42950_e41184, assign42950_e41184_d_n4, assign42950_e41184_d_n7, assign42950_e41184_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expffvarg, var_fn468_calc_ig__expffvarg_dn4, var_fn468_calc_ig__expffvarg_dn7, var_fn468_calc_ig__expffvarg_dn8,)
    }
};
        var_fn468_calc_ig__expffvarg = assign42950_e41184;
        var_fn468_calc_ig__expffvarg_dn4 = assign42950_e41184_d_n4;
        var_fn468_calc_ig__expffvarg_dn7 = assign42950_e41184_d_n7;
        var_fn468_calc_ig__expffvarg_dn8 = assign42950_e41184_d_n8;

        let (assign42960_e41190, assign42960_e41190_d_n4, assign42960_e41190_d_n7, assign42960_e41190_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expiforarg, var_fn468_calc_ig__expiforarg_dn4, var_fn468_calc_ig__expiforarg_dn7, var_fn468_calc_ig__expiforarg_dn8,)
    }
};
        var_fn468_calc_ig__expiforarg = assign42960_e41190;
        var_fn468_calc_ig__expiforarg_dn4 = assign42960_e41190_d_n4;
        var_fn468_calc_ig__expiforarg_dn7 = assign42960_e41190_d_n7;
        var_fn468_calc_ig__expiforarg_dn8 = assign42960_e41190_d_n8;

        let (assign42970_e41196, assign42970_e41196_d_n4, assign42970_e41196_d_n7, assign42970_e41196_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expifor, var_fn468_calc_ig__expifor_dn4, var_fn468_calc_ig__expifor_dn7, var_fn468_calc_ig__expifor_dn8,)
    }
};
        var_fn468_calc_ig__expifor = assign42970_e41196;
        var_fn468_calc_ig__expifor_dn4 = assign42970_e41196_d_n4;
        var_fn468_calc_ig__expifor_dn7 = assign42970_e41196_d_n7;
        var_fn468_calc_ig__expifor_dn8 = assign42970_e41196_d_n8;

        let (assign42980_e41202, assign42980_e41202_d_n4, assign42980_e41202_d_n7, assign42980_e41202_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expirevarg, var_fn468_calc_ig__expirevarg_dn4, var_fn468_calc_ig__expirevarg_dn7, var_fn468_calc_ig__expirevarg_dn8,)
    }
};
        var_fn468_calc_ig__expirevarg = assign42980_e41202;
        var_fn468_calc_ig__expirevarg_dn4 = assign42980_e41202_d_n4;
        var_fn468_calc_ig__expirevarg_dn7 = assign42980_e41202_d_n7;
        var_fn468_calc_ig__expirevarg_dn8 = assign42980_e41202_d_n8;

        let (assign42990_e41208, assign42990_e41208_d_n4, assign42990_e41208_d_n7, assign42990_e41208_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expirev, var_fn468_calc_ig__expirev_dn4, var_fn468_calc_ig__expirev_dn7, var_fn468_calc_ig__expirev_dn8,)
    }
};
        var_fn468_calc_ig__expirev = assign42990_e41208;
        var_fn468_calc_ig__expirev_dn4 = assign42990_e41208_d_n4;
        var_fn468_calc_ig__expirev_dn7 = assign42990_e41208_d_n7;
        var_fn468_calc_ig__expirev_dn8 = assign42990_e41208_d_n8;

        let (assign43000_e41214,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0,)
    } else {
        (var_fn468_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn468_calc_ig__pg_paramin_hinj = assign43000_e41214;

        let (assign43010_e41220, assign43010_e41220_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expbdarg1_vgsat, var_fn468_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__expbdarg1_vgsat = assign43010_e41220;
        var_fn468_calc_ig__expbdarg1_vgsat_dn4 = assign43010_e41220_d_n4;

        let (assign43020_e41226, assign43020_e41226_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expbd1_vgsat, var_fn468_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__expbd1_vgsat = assign43020_e41226;
        var_fn468_calc_ig__expbd1_vgsat_dn4 = assign43020_e41226_d_n4;

        let (assign43030_e41232, assign43030_e41232_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__iginbd_vgsat, var_fn468_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__iginbd_vgsat = assign43030_e41232;
        var_fn468_calc_ig__iginbd_vgsat_dn4 = assign43030_e41232_d_n4;

        let (assign43040_e41238, assign43040_e41238_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expiforarg_nohinj_vgsat, var_fn468_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__expiforarg_nohinj_vgsat = assign43040_e41238;
        var_fn468_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign43040_e41238_d_n4;

        let (assign43050_e41244, assign43050_e41244_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expifor_nohinj_vgsat, var_fn468_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__expifor_nohinj_vgsat = assign43050_e41244;
        var_fn468_calc_ig__expifor_nohinj_vgsat_dn4 = assign43050_e41244_d_n4;

        let (assign43060_e41250, assign43060_e41250_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__igindiode_nohinj_vgsat, var_fn468_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__igindiode_nohinj_vgsat = assign43060_e41250;
        var_fn468_calc_ig__igindiode_nohinj_vgsat_dn4 = assign43060_e41250_d_n4;

        let (assign43070_e41256, assign43070_e41256_d_n4, assign43070_e41256_d_n7, assign43070_e41256_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__igindiode_nohinj, var_fn468_calc_ig__igindiode_nohinj_dn4, var_fn468_calc_ig__igindiode_nohinj_dn7, var_fn468_calc_ig__igindiode_nohinj_dn8,)
    }
};
        var_fn468_calc_ig__igindiode_nohinj = assign43070_e41256;
        var_fn468_calc_ig__igindiode_nohinj_dn4 = assign43070_e41256_d_n4;
        var_fn468_calc_ig__igindiode_nohinj_dn7 = assign43070_e41256_d_n7;
        var_fn468_calc_ig__igindiode_nohinj_dn8 = assign43070_e41256_d_n8;

        let (assign43080_e41262, assign43080_e41262_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expiforarg_hinj_vgsat, var_fn468_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__expiforarg_hinj_vgsat = assign43080_e41262;
        var_fn468_calc_ig__expiforarg_hinj_vgsat_dn4 = assign43080_e41262_d_n4;

        let (assign43090_e41268, assign43090_e41268_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expifor_hinj_vgsat, var_fn468_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__expifor_hinj_vgsat = assign43090_e41268;
        var_fn468_calc_ig__expifor_hinj_vgsat_dn4 = assign43090_e41268_d_n4;

        let (assign43100_e41274, assign43100_e41274_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__igindiode_hinj_vgsat, var_fn468_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__igindiode_hinj_vgsat = assign43100_e41274;
        var_fn468_calc_ig__igindiode_hinj_vgsat_dn4 = assign43100_e41274_d_n4;

        let (assign43110_e41280, assign43110_e41280_d_n4, assign43110_e41280_d_n7, assign43110_e41280_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expiforarg_hinj, var_fn468_calc_ig__expiforarg_hinj_dn4, var_fn468_calc_ig__expiforarg_hinj_dn7, var_fn468_calc_ig__expiforarg_hinj_dn8,)
    }
};
        var_fn468_calc_ig__expiforarg_hinj = assign43110_e41280;
        var_fn468_calc_ig__expiforarg_hinj_dn4 = assign43110_e41280_d_n4;
        var_fn468_calc_ig__expiforarg_hinj_dn7 = assign43110_e41280_d_n7;
        var_fn468_calc_ig__expiforarg_hinj_dn8 = assign43110_e41280_d_n8;

        let (assign43120_e41286, assign43120_e41286_d_n4, assign43120_e41286_d_n7, assign43120_e41286_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__expifor_hinj, var_fn468_calc_ig__expifor_hinj_dn4, var_fn468_calc_ig__expifor_hinj_dn7, var_fn468_calc_ig__expifor_hinj_dn8,)
    }
};
        var_fn468_calc_ig__expifor_hinj = assign43120_e41286;
        var_fn468_calc_ig__expifor_hinj_dn4 = assign43120_e41286_d_n4;
        var_fn468_calc_ig__expifor_hinj_dn7 = assign43120_e41286_d_n7;
        var_fn468_calc_ig__expifor_hinj_dn8 = assign43120_e41286_d_n8;

        let (assign43130_e41292, assign43130_e41292_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__igindiode_hinj_pre, var_fn468_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn468_calc_ig__igindiode_hinj_pre = assign43130_e41292;
        var_fn468_calc_ig__igindiode_hinj_pre_dn4 = assign43130_e41292_d_n4;

        let (assign43140_e41298, assign43140_e41298_d_n4, assign43140_e41298_d_n7, assign43140_e41298_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__igindiode_hinj, var_fn468_calc_ig__igindiode_hinj_dn4, var_fn468_calc_ig__igindiode_hinj_dn7, var_fn468_calc_ig__igindiode_hinj_dn8,)
    }
};
        var_fn468_calc_ig__igindiode_hinj = assign43140_e41298;
        var_fn468_calc_ig__igindiode_hinj_dn4 = assign43140_e41298_d_n4;
        var_fn468_calc_ig__igindiode_hinj_dn7 = assign43140_e41298_d_n7;
        var_fn468_calc_ig__igindiode_hinj_dn8 = assign43140_e41298_d_n8;

        let (assign43150_e41309, assign43150_e41309_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign43150_e41304: f64 = (var_fn468_calc_ig__pg_param1 / var_fn468_calc_ig__phitin);
        let assign43150_e41306: f64 = (-var_fn468_calc_ig__vjg);
        let assign43150_e41307: f64 = (assign43150_e41304 * assign43150_e41306);
        (assign43150_e41307, ((-((var_fn468_calc_ig__pg_param1 * var_fn468_calc_ig__phitin_dn4) / (var_fn468_calc_ig__phitin * var_fn468_calc_ig__phitin))) * assign43150_e41306),)
    } else {
        (var_fn468_calc_ig__expphib, var_fn468_calc_ig__expphib_dn4,)
    }
};
        var_fn468_calc_ig__expphib = assign43150_e41309;
        var_fn468_calc_ig__expphib_dn4 = assign43150_e41309_d_n4;

        let (assign43160_e41353, assign43160_e41353_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign43160_e41319: f64 = (-50.0);
        let (assign43160_e41351, assign43160_e41351_d_n4,) = {
            if ((!(var_fn468_calc_ig__expphib > 50.0)) && (!(var_fn468_calc_ig__expphib < assign43160_e41319))) {
                let assign43160_e41324: f64 = (var_fn468_calc_ig__expphib).exp();
                (assign43160_e41324, (assign43160_e41324 * var_fn468_calc_ig__expphib_dn4),)
            } else {
                let assign43160_e41331: f64 = (-50.0);
                let (assign43160_e41350, assign43160_e41350_d_n4,) = {
                    if ((!(var_fn468_calc_ig__expphib > 50.0)) && (var_fn468_calc_ig__expphib < assign43160_e41331)) {
                        let assign43160_e41335: f64 = (-50.0);
                        let assign43160_e41336: f64 = (assign43160_e41335).exp();
                        (assign43160_e41336, 0.0,)
                    } else {
                        let (assign43160_e41349, assign43160_e41349_d_n4,) = {
                            if (var_fn468_calc_ig__expphib > 50.0) {
                                let assign43160_e41341: f64 = (50.0_f64).exp();
                                let assign43160_e41345: f64 = (var_fn468_calc_ig__expphib - 50.0);
                                let assign43160_e41346: f64 = (1.0 + assign43160_e41345);
                                let assign43160_e41347: f64 = (assign43160_e41341 * assign43160_e41346);
                                (assign43160_e41347, (assign43160_e41341 * var_fn468_calc_ig__expphib_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign43160_e41349, assign43160_e41349_d_n4,)
                    }
                };
                (assign43160_e41350, assign43160_e41350_d_n4,)
            }
        };
        (assign43160_e41351, assign43160_e41351_d_n4,)
    } else {
        (var_fn468_calc_ig__t0, var_fn468_calc_ig__t0_dn4,)
    }
};
        var_fn468_calc_ig__t0 = assign43160_e41353;
        var_fn468_calc_ig__t0_dn4 = assign43160_e41353_d_n4;

        let (assign43170_e41366, assign43170_e41366_d_n4, assign43170_e41366_d_n7, assign43170_e41366_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign43170_e41359: f64 = (-var_fn468_calc_ig__vgin);
        let assign43170_e41361: f64 = (assign43170_e41359 - var_fn468_calc_ig__vbdgin);
        let assign43170_e41362: f64 = (var_fn468_calc_ig__pbdgin * assign43170_e41361);
        let assign43170_e41364: f64 = (assign43170_e41362 + var_fn468_calc_ig__expphib);
        (assign43170_e41364, var_fn468_calc_ig__expphib_dn4, (var_fn468_calc_ig__pbdgin * (-var_fn468_calc_ig__vgin_dn7)), (var_fn468_calc_ig__pbdgin * (-var_fn468_calc_ig__vgin_dn8)),)
    } else {
        (var_fn468_calc_ig__expbdarg1, var_fn468_calc_ig__expbdarg1_dn4, var_fn468_calc_ig__expbdarg1_dn7, var_fn468_calc_ig__expbdarg1_dn8,)
    }
};
        var_fn468_calc_ig__expbdarg1 = assign43170_e41366;
        var_fn468_calc_ig__expbdarg1_dn4 = assign43170_e41366_d_n4;
        var_fn468_calc_ig__expbdarg1_dn7 = assign43170_e41366_d_n7;
        var_fn468_calc_ig__expbdarg1_dn8 = assign43170_e41366_d_n8;

        let (assign43180_e41377, assign43180_e41377_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign43180_e41371: f64 = (-var_fn468_calc_ig__pbdgin);
        let assign43180_e41373: f64 = (assign43180_e41371 * var_fn468_calc_ig__vbdgin);
        let assign43180_e41375: f64 = (assign43180_e41373 + var_fn468_calc_ig__expphib);
        (assign43180_e41375, var_fn468_calc_ig__expphib_dn4,)
    } else {
        (var_fn468_calc_ig__expbdarg2, var_fn468_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn468_calc_ig__expbdarg2 = assign43180_e41377;
        var_fn468_calc_ig__expbdarg2_dn4 = assign43180_e41377_d_n4;

        let (assign43190_e41421, assign43190_e41421_d_n4, assign43190_e41421_d_n7, assign43190_e41421_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign43190_e41387: f64 = (-50.0);
        let (assign43190_e41419, assign43190_e41419_d_n4, assign43190_e41419_d_n7, assign43190_e41419_d_n8,) = {
            if ((!(var_fn468_calc_ig__expbdarg1 > 50.0)) && (!(var_fn468_calc_ig__expbdarg1 < assign43190_e41387))) {
                let assign43190_e41392: f64 = (var_fn468_calc_ig__expbdarg1).exp();
                (assign43190_e41392, (assign43190_e41392 * var_fn468_calc_ig__expbdarg1_dn4), (assign43190_e41392 * var_fn468_calc_ig__expbdarg1_dn7), (assign43190_e41392 * var_fn468_calc_ig__expbdarg1_dn8),)
            } else {
                let assign43190_e41399: f64 = (-50.0);
                let (assign43190_e41418, assign43190_e41418_d_n4, assign43190_e41418_d_n7, assign43190_e41418_d_n8,) = {
                    if ((!(var_fn468_calc_ig__expbdarg1 > 50.0)) && (var_fn468_calc_ig__expbdarg1 < assign43190_e41399)) {
                        let assign43190_e41403: f64 = (-50.0);
                        let assign43190_e41404: f64 = (assign43190_e41403).exp();
                        (assign43190_e41404, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign43190_e41417, assign43190_e41417_d_n4, assign43190_e41417_d_n7, assign43190_e41417_d_n8,) = {
                            if (var_fn468_calc_ig__expbdarg1 > 50.0) {
                                let assign43190_e41409: f64 = (50.0_f64).exp();
                                let assign43190_e41413: f64 = (var_fn468_calc_ig__expbdarg1 - 50.0);
                                let assign43190_e41414: f64 = (1.0 + assign43190_e41413);
                                let assign43190_e41415: f64 = (assign43190_e41409 * assign43190_e41414);
                                (assign43190_e41415, (assign43190_e41409 * var_fn468_calc_ig__expbdarg1_dn4), (assign43190_e41409 * var_fn468_calc_ig__expbdarg1_dn7), (assign43190_e41409 * var_fn468_calc_ig__expbdarg1_dn8),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign43190_e41417, assign43190_e41417_d_n4, assign43190_e41417_d_n7, assign43190_e41417_d_n8,)
                    }
                };
                (assign43190_e41418, assign43190_e41418_d_n4, assign43190_e41418_d_n7, assign43190_e41418_d_n8,)
            }
        };
        (assign43190_e41419, assign43190_e41419_d_n4, assign43190_e41419_d_n7, assign43190_e41419_d_n8,)
    } else {
        (var_fn468_calc_ig__expbd1, var_fn468_calc_ig__expbd1_dn4, var_fn468_calc_ig__expbd1_dn7, var_fn468_calc_ig__expbd1_dn8,)
    }
};
        var_fn468_calc_ig__expbd1 = assign43190_e41421;
        var_fn468_calc_ig__expbd1_dn4 = assign43190_e41421_d_n4;
        var_fn468_calc_ig__expbd1_dn7 = assign43190_e41421_d_n7;
        var_fn468_calc_ig__expbd1_dn8 = assign43190_e41421_d_n8;

        let (assign43200_e41465, assign43200_e41465_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign43200_e41431: f64 = (-50.0);
        let (assign43200_e41463, assign43200_e41463_d_n4,) = {
            if ((!(var_fn468_calc_ig__expbdarg2 > 50.0)) && (!(var_fn468_calc_ig__expbdarg2 < assign43200_e41431))) {
                let assign43200_e41436: f64 = (var_fn468_calc_ig__expbdarg2).exp();
                (assign43200_e41436, (assign43200_e41436 * var_fn468_calc_ig__expbdarg2_dn4),)
            } else {
                let assign43200_e41443: f64 = (-50.0);
                let (assign43200_e41462, assign43200_e41462_d_n4,) = {
                    if ((!(var_fn468_calc_ig__expbdarg2 > 50.0)) && (var_fn468_calc_ig__expbdarg2 < assign43200_e41443)) {
                        let assign43200_e41447: f64 = (-50.0);
                        let assign43200_e41448: f64 = (assign43200_e41447).exp();
                        (assign43200_e41448, 0.0,)
                    } else {
                        let (assign43200_e41461, assign43200_e41461_d_n4,) = {
                            if (var_fn468_calc_ig__expbdarg2 > 50.0) {
                                let assign43200_e41453: f64 = (50.0_f64).exp();
                                let assign43200_e41457: f64 = (var_fn468_calc_ig__expbdarg2 - 50.0);
                                let assign43200_e41458: f64 = (1.0 + assign43200_e41457);
                                let assign43200_e41459: f64 = (assign43200_e41453 * assign43200_e41458);
                                (assign43200_e41459, (assign43200_e41453 * var_fn468_calc_ig__expbdarg2_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign43200_e41461, assign43200_e41461_d_n4,)
                    }
                };
                (assign43200_e41462, assign43200_e41462_d_n4,)
            }
        };
        (assign43200_e41463, assign43200_e41463_d_n4,)
    } else {
        (var_fn468_calc_ig__expbd2, var_fn468_calc_ig__expbd2_dn4,)
    }
};
        var_fn468_calc_ig__expbd2 = assign43200_e41465;
        var_fn468_calc_ig__expbd2_dn4 = assign43200_e41465_d_n4;

        let (assign43210_e41473, assign43210_e41473_d_n4, assign43210_e41473_d_n7, assign43210_e41473_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign43210_e41471: f64 = (var_fn468_calc_ig__expbd1 - var_fn468_calc_ig__expbd2);
        (assign43210_e41471, (var_fn468_calc_ig__expbd1_dn4 - var_fn468_calc_ig__expbd2_dn4), var_fn468_calc_ig__expbd1_dn7, var_fn468_calc_ig__expbd1_dn8,)
    } else {
        (var_fn468_calc_ig__iginbd, var_fn468_calc_ig__iginbd_dn4, var_fn468_calc_ig__iginbd_dn7, var_fn468_calc_ig__iginbd_dn8,)
    }
};
        var_fn468_calc_ig__iginbd = assign43210_e41473;
        var_fn468_calc_ig__iginbd_dn4 = assign43210_e41473_d_n4;
        var_fn468_calc_ig__iginbd_dn7 = assign43210_e41473_d_n7;
        var_fn468_calc_ig__iginbd_dn8 = assign43210_e41473_d_n8;

        *var_fn468_calc_ig__expbd1_slot = var_fn468_calc_ig__expbd1;
        *var_fn468_calc_ig__expbd1_dn4_slot = var_fn468_calc_ig__expbd1_dn4;
        *var_fn468_calc_ig__expbd1_dn7_slot = var_fn468_calc_ig__expbd1_dn7;
        *var_fn468_calc_ig__expbd1_dn8_slot = var_fn468_calc_ig__expbd1_dn8;
        *var_fn468_calc_ig__expbd1_vgsat_slot = var_fn468_calc_ig__expbd1_vgsat;
        *var_fn468_calc_ig__expbd1_vgsat_dn4_slot = var_fn468_calc_ig__expbd1_vgsat_dn4;
        *var_fn468_calc_ig__expbd2_slot = var_fn468_calc_ig__expbd2;
        *var_fn468_calc_ig__expbd2_dn4_slot = var_fn468_calc_ig__expbd2_dn4;
        *var_fn468_calc_ig__expbdarg1_slot = var_fn468_calc_ig__expbdarg1;
        *var_fn468_calc_ig__expbdarg1_dn4_slot = var_fn468_calc_ig__expbdarg1_dn4;
        *var_fn468_calc_ig__expbdarg1_dn7_slot = var_fn468_calc_ig__expbdarg1_dn7;
        *var_fn468_calc_ig__expbdarg1_dn8_slot = var_fn468_calc_ig__expbdarg1_dn8;
        *var_fn468_calc_ig__expbdarg1_vgsat_slot = var_fn468_calc_ig__expbdarg1_vgsat;
        *var_fn468_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn468_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn468_calc_ig__expbdarg2_slot = var_fn468_calc_ig__expbdarg2;
        *var_fn468_calc_ig__expbdarg2_dn4_slot = var_fn468_calc_ig__expbdarg2_dn4;
        *var_fn468_calc_ig__expffvarg_slot = var_fn468_calc_ig__expffvarg;
        *var_fn468_calc_ig__expffvarg_dn4_slot = var_fn468_calc_ig__expffvarg_dn4;
        *var_fn468_calc_ig__expffvarg_dn7_slot = var_fn468_calc_ig__expffvarg_dn7;
        *var_fn468_calc_ig__expffvarg_dn8_slot = var_fn468_calc_ig__expffvarg_dn8;
        *var_fn468_calc_ig__expifor_slot = var_fn468_calc_ig__expifor;
        *var_fn468_calc_ig__expifor_dn4_slot = var_fn468_calc_ig__expifor_dn4;
        *var_fn468_calc_ig__expifor_dn7_slot = var_fn468_calc_ig__expifor_dn7;
        *var_fn468_calc_ig__expifor_dn8_slot = var_fn468_calc_ig__expifor_dn8;
        *var_fn468_calc_ig__expifor_hinj_slot = var_fn468_calc_ig__expifor_hinj;
        *var_fn468_calc_ig__expifor_hinj_dn4_slot = var_fn468_calc_ig__expifor_hinj_dn4;
        *var_fn468_calc_ig__expifor_hinj_dn7_slot = var_fn468_calc_ig__expifor_hinj_dn7;
        *var_fn468_calc_ig__expifor_hinj_dn8_slot = var_fn468_calc_ig__expifor_hinj_dn8;
        *var_fn468_calc_ig__expifor_hinj_vgsat_slot = var_fn468_calc_ig__expifor_hinj_vgsat;
        *var_fn468_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn468_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn468_calc_ig__expifor_nohinj_vgsat_slot = var_fn468_calc_ig__expifor_nohinj_vgsat;
        *var_fn468_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn468_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn468_calc_ig__expiforarg_slot = var_fn468_calc_ig__expiforarg;
        *var_fn468_calc_ig__expiforarg_dn4_slot = var_fn468_calc_ig__expiforarg_dn4;
        *var_fn468_calc_ig__expiforarg_dn7_slot = var_fn468_calc_ig__expiforarg_dn7;
        *var_fn468_calc_ig__expiforarg_dn8_slot = var_fn468_calc_ig__expiforarg_dn8;
        *var_fn468_calc_ig__expiforarg_hinj_slot = var_fn468_calc_ig__expiforarg_hinj;
        *var_fn468_calc_ig__expiforarg_hinj_dn4_slot = var_fn468_calc_ig__expiforarg_hinj_dn4;
        *var_fn468_calc_ig__expiforarg_hinj_dn7_slot = var_fn468_calc_ig__expiforarg_hinj_dn7;
        *var_fn468_calc_ig__expiforarg_hinj_dn8_slot = var_fn468_calc_ig__expiforarg_hinj_dn8;
        *var_fn468_calc_ig__expiforarg_hinj_vgsat_slot = var_fn468_calc_ig__expiforarg_hinj_vgsat;
        *var_fn468_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn468_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn468_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn468_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn468_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn468_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn468_calc_ig__expirev_slot = var_fn468_calc_ig__expirev;
        *var_fn468_calc_ig__expirev_dn4_slot = var_fn468_calc_ig__expirev_dn4;
        *var_fn468_calc_ig__expirev_dn7_slot = var_fn468_calc_ig__expirev_dn7;
        *var_fn468_calc_ig__expirev_dn8_slot = var_fn468_calc_ig__expirev_dn8;
        *var_fn468_calc_ig__expirevarg_slot = var_fn468_calc_ig__expirevarg;
        *var_fn468_calc_ig__expirevarg_dn4_slot = var_fn468_calc_ig__expirevarg_dn4;
        *var_fn468_calc_ig__expirevarg_dn7_slot = var_fn468_calc_ig__expirevarg_dn7;
        *var_fn468_calc_ig__expirevarg_dn8_slot = var_fn468_calc_ig__expirevarg_dn8;
        *var_fn468_calc_ig__expphib_slot = var_fn468_calc_ig__expphib;
        *var_fn468_calc_ig__expphib_dn4_slot = var_fn468_calc_ig__expphib_dn4;
        *var_fn468_calc_ig__ffvgin_slot = var_fn468_calc_ig__ffvgin;
        *var_fn468_calc_ig__ffvgin_dn4_slot = var_fn468_calc_ig__ffvgin_dn4;
        *var_fn468_calc_ig__ffvgin_dn7_slot = var_fn468_calc_ig__ffvgin_dn7;
        *var_fn468_calc_ig__ffvgin_dn8_slot = var_fn468_calc_ig__ffvgin_dn8;
        *var_fn468_calc_ig__frecgin_slot = var_fn468_calc_ig__frecgin;
        *var_fn468_calc_ig__frecgin_dn7_slot = var_fn468_calc_ig__frecgin_dn7;
        *var_fn468_calc_ig__frecgin_dn8_slot = var_fn468_calc_ig__frecgin_dn8;
        *var_fn468_calc_ig__iginbd_slot = var_fn468_calc_ig__iginbd;
        *var_fn468_calc_ig__iginbd_dn4_slot = var_fn468_calc_ig__iginbd_dn4;
        *var_fn468_calc_ig__iginbd_dn7_slot = var_fn468_calc_ig__iginbd_dn7;
        *var_fn468_calc_ig__iginbd_dn8_slot = var_fn468_calc_ig__iginbd_dn8;
        *var_fn468_calc_ig__iginbd_vgsat_slot = var_fn468_calc_ig__iginbd_vgsat;
        *var_fn468_calc_ig__iginbd_vgsat_dn4_slot = var_fn468_calc_ig__iginbd_vgsat_dn4;
        *var_fn468_calc_ig__igindiode_slot = var_fn468_calc_ig__igindiode;
        *var_fn468_calc_ig__igindiode_dn4_slot = var_fn468_calc_ig__igindiode_dn4;
        *var_fn468_calc_ig__igindiode_dn7_slot = var_fn468_calc_ig__igindiode_dn7;
        *var_fn468_calc_ig__igindiode_dn8_slot = var_fn468_calc_ig__igindiode_dn8;
        *var_fn468_calc_ig__igindiode_hinj_slot = var_fn468_calc_ig__igindiode_hinj;
        *var_fn468_calc_ig__igindiode_hinj_dn4_slot = var_fn468_calc_ig__igindiode_hinj_dn4;
        *var_fn468_calc_ig__igindiode_hinj_dn7_slot = var_fn468_calc_ig__igindiode_hinj_dn7;
        *var_fn468_calc_ig__igindiode_hinj_dn8_slot = var_fn468_calc_ig__igindiode_hinj_dn8;
        *var_fn468_calc_ig__igindiode_hinj_pre_slot = var_fn468_calc_ig__igindiode_hinj_pre;
        *var_fn468_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn468_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn468_calc_ig__igindiode_hinj_vgsat_slot = var_fn468_calc_ig__igindiode_hinj_vgsat;
        *var_fn468_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn468_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn468_calc_ig__igindiode_nohinj_slot = var_fn468_calc_ig__igindiode_nohinj;
        *var_fn468_calc_ig__igindiode_nohinj_dn4_slot = var_fn468_calc_ig__igindiode_nohinj_dn4;
        *var_fn468_calc_ig__igindiode_nohinj_dn7_slot = var_fn468_calc_ig__igindiode_nohinj_dn7;
        *var_fn468_calc_ig__igindiode_nohinj_dn8_slot = var_fn468_calc_ig__igindiode_nohinj_dn8;
        *var_fn468_calc_ig__igindiode_nohinj_vgsat_slot = var_fn468_calc_ig__igindiode_nohinj_vgsat;
        *var_fn468_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn468_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn468_calc_ig__iginrec_slot = var_fn468_calc_ig__iginrec;
        *var_fn468_calc_ig__iginrec_dn4_slot = var_fn468_calc_ig__iginrec_dn4;
        *var_fn468_calc_ig__iginrec_dn7_slot = var_fn468_calc_ig__iginrec_dn7;
        *var_fn468_calc_ig__iginrec_dn8_slot = var_fn468_calc_ig__iginrec_dn8;
        *var_fn468_calc_ig__pg_paramin_hinj_slot = var_fn468_calc_ig__pg_paramin_hinj;
        *var_fn468_calc_ig__t0_slot = var_fn468_calc_ig__t0;
        *var_fn468_calc_ig__t0_dn4_slot = var_fn468_calc_ig__t0_dn4;
    }

    pub(super) fn stamp_transient_block_108(
        var_fn468_calc_ig__alphagin: f64,
        var_fn468_calc_ig__expbd2: f64,
        var_fn468_calc_ig__expbd2_dn4: f64,
        var_fn468_calc_ig__expphib: f64,
        var_fn468_calc_ig__expphib_dn4: f64,
        var_fn468_calc_ig__fracin: f64,
        var_fn468_calc_ig__iginbd: f64,
        var_fn468_calc_ig__iginbd_dn4: f64,
        var_fn468_calc_ig__iginbd_dn7: f64,
        var_fn468_calc_ig__iginbd_dn8: f64,
        var_fn468_calc_ig__ijin: f64,
        var_fn468_calc_ig__kbdgatein: f64,
        var_fn468_calc_ig__ngf: f64,
        var_fn468_calc_ig__pbdgin: f64,
        var_fn468_calc_ig__pg_paramin: f64,
        var_fn468_calc_ig__phitin: f64,
        var_fn468_calc_ig__phitin_dn4: f64,
        var_fn468_calc_ig__t0: f64,
        var_fn468_calc_ig__t0_dn4: f64,
        var_fn468_calc_ig__tfacdiodein: f64,
        var_fn468_calc_ig__tfacdiodein_dn4: f64,
        var_fn468_calc_ig__type: f64,
        var_fn468_calc_ig__vbdgin: f64,
        var_fn468_calc_ig__vgin: f64,
        var_fn468_calc_ig__vgin_dn7: f64,
        var_fn468_calc_ig__vgin_dn8: f64,
        var_fn468_calc_ig__vgsatin: f64,
        var_fn468_calc_ig__w: f64,
        var_guard461: f64,
        var_guard467: f64,
        var_fn468_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn468_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn468_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn468_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn468_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__expffvarg_slot: &mut f64,
        var_fn468_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn468_calc_ig__expffvarg_dn7_slot: &mut f64,
        var_fn468_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn468_calc_ig__expifor_slot: &mut f64,
        var_fn468_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn468_calc_ig__expifor_dn7_slot: &mut f64,
        var_fn468_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn468_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn468_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn468_calc_ig__expifor_hinj_dn7_slot: &mut f64,
        var_fn468_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn468_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn468_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn468_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_dn7_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_hinj_dn7_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn468_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__ffvgin_slot: &mut f64,
        var_fn468_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn468_calc_ig__ffvgin_dn7_slot: &mut f64,
        var_fn468_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn468_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn468_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__igindiode_slot: &mut f64,
        var_fn468_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn468_calc_ig__igindiode_dn7_slot: &mut f64,
        var_fn468_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn468_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn468_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn468_calc_ig__igindiode_hinj_dn7_slot: &mut f64,
        var_fn468_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn468_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn468_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn468_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn468_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn468_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn468_calc_ig__igindiode_nohinj_dn7_slot: &mut f64,
        var_fn468_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn468_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn468_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn468_calc_ig__isdiodeout_slot: &mut f64,
        var_fn468_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn468_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_guard469_slot: &mut f64,
        var_guard470_slot: &mut f64,
        var_guard471_slot: &mut f64,
        var_guard472_slot: &mut f64,
    ) {
        let mut var_fn468_calc_ig__alpha2_phit: f64 = *var_fn468_calc_ig__alpha2_phit_slot;
        let mut var_fn468_calc_ig__alpha2_phit_dn4: f64 = *var_fn468_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn468_calc_ig__expbd1_vgsat: f64 = *var_fn468_calc_ig__expbd1_vgsat_slot;
        let mut var_fn468_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn468_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__expbdarg1_vgsat: f64 = *var_fn468_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn468_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn468_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__expffvarg: f64 = *var_fn468_calc_ig__expffvarg_slot;
        let mut var_fn468_calc_ig__expffvarg_dn4: f64 = *var_fn468_calc_ig__expffvarg_dn4_slot;
        let mut var_fn468_calc_ig__expffvarg_dn7: f64 = *var_fn468_calc_ig__expffvarg_dn7_slot;
        let mut var_fn468_calc_ig__expffvarg_dn8: f64 = *var_fn468_calc_ig__expffvarg_dn8_slot;
        let mut var_fn468_calc_ig__expifor: f64 = *var_fn468_calc_ig__expifor_slot;
        let mut var_fn468_calc_ig__expifor_dn4: f64 = *var_fn468_calc_ig__expifor_dn4_slot;
        let mut var_fn468_calc_ig__expifor_dn7: f64 = *var_fn468_calc_ig__expifor_dn7_slot;
        let mut var_fn468_calc_ig__expifor_dn8: f64 = *var_fn468_calc_ig__expifor_dn8_slot;
        let mut var_fn468_calc_ig__expifor_hinj: f64 = *var_fn468_calc_ig__expifor_hinj_slot;
        let mut var_fn468_calc_ig__expifor_hinj_dn4: f64 = *var_fn468_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn468_calc_ig__expifor_hinj_dn7: f64 = *var_fn468_calc_ig__expifor_hinj_dn7_slot;
        let mut var_fn468_calc_ig__expifor_hinj_dn8: f64 = *var_fn468_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn468_calc_ig__expifor_hinj_vgsat: f64 = *var_fn468_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn468_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn468_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn468_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn468_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn468_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__expiforarg: f64 = *var_fn468_calc_ig__expiforarg_slot;
        let mut var_fn468_calc_ig__expiforarg_dn4: f64 = *var_fn468_calc_ig__expiforarg_dn4_slot;
        let mut var_fn468_calc_ig__expiforarg_dn7: f64 = *var_fn468_calc_ig__expiforarg_dn7_slot;
        let mut var_fn468_calc_ig__expiforarg_dn8: f64 = *var_fn468_calc_ig__expiforarg_dn8_slot;
        let mut var_fn468_calc_ig__expiforarg_hinj: f64 = *var_fn468_calc_ig__expiforarg_hinj_slot;
        let mut var_fn468_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn468_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn468_calc_ig__expiforarg_hinj_dn7: f64 = *var_fn468_calc_ig__expiforarg_hinj_dn7_slot;
        let mut var_fn468_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn468_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn468_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn468_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn468_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn468_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn468_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn468_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn468_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__ffvgin: f64 = *var_fn468_calc_ig__ffvgin_slot;
        let mut var_fn468_calc_ig__ffvgin_dn4: f64 = *var_fn468_calc_ig__ffvgin_dn4_slot;
        let mut var_fn468_calc_ig__ffvgin_dn7: f64 = *var_fn468_calc_ig__ffvgin_dn7_slot;
        let mut var_fn468_calc_ig__ffvgin_dn8: f64 = *var_fn468_calc_ig__ffvgin_dn8_slot;
        let mut var_fn468_calc_ig__iginbd_vgsat: f64 = *var_fn468_calc_ig__iginbd_vgsat_slot;
        let mut var_fn468_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn468_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__igindiode: f64 = *var_fn468_calc_ig__igindiode_slot;
        let mut var_fn468_calc_ig__igindiode_dn4: f64 = *var_fn468_calc_ig__igindiode_dn4_slot;
        let mut var_fn468_calc_ig__igindiode_dn7: f64 = *var_fn468_calc_ig__igindiode_dn7_slot;
        let mut var_fn468_calc_ig__igindiode_dn8: f64 = *var_fn468_calc_ig__igindiode_dn8_slot;
        let mut var_fn468_calc_ig__igindiode_hinj: f64 = *var_fn468_calc_ig__igindiode_hinj_slot;
        let mut var_fn468_calc_ig__igindiode_hinj_dn4: f64 = *var_fn468_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn468_calc_ig__igindiode_hinj_dn7: f64 = *var_fn468_calc_ig__igindiode_hinj_dn7_slot;
        let mut var_fn468_calc_ig__igindiode_hinj_dn8: f64 = *var_fn468_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn468_calc_ig__igindiode_hinj_pre: f64 = *var_fn468_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn468_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn468_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn468_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn468_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn468_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn468_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__igindiode_nohinj: f64 = *var_fn468_calc_ig__igindiode_nohinj_slot;
        let mut var_fn468_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn468_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn468_calc_ig__igindiode_nohinj_dn7: f64 = *var_fn468_calc_ig__igindiode_nohinj_dn7_slot;
        let mut var_fn468_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn468_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn468_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn468_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn468_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn468_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn468_calc_ig__isdiodeout: f64 = *var_fn468_calc_ig__isdiodeout_slot;
        let mut var_fn468_calc_ig__isdiodeout_dn4: f64 = *var_fn468_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn468_calc_ig__pg_paramin_hinj: f64 = *var_fn468_calc_ig__pg_paramin_hinj_slot;
        let mut var_guard469: f64 = *var_guard469_slot;
        let mut var_guard470: f64 = *var_guard470_slot;
        let mut var_guard471: f64 = *var_guard471_slot;
        let mut var_guard472: f64 = *var_guard472_slot;

        let (assign43220_e41487, assign43220_e41487_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign43220_e41479: f64 = (var_fn468_calc_ig__type * var_fn468_calc_ig__w);
        let assign43220_e41481: f64 = (assign43220_e41479 * var_fn468_calc_ig__ngf);
        let assign43220_e41483: f64 = (assign43220_e41481 * var_fn468_calc_ig__ijin);
        let assign43220_e41485: f64 = (assign43220_e41483 * var_fn468_calc_ig__tfacdiodein);
        (assign43220_e41485, (assign43220_e41483 * var_fn468_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn468_calc_ig__isdiodeout, var_fn468_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn468_calc_ig__isdiodeout = assign43220_e41487;
        var_fn468_calc_ig__isdiodeout_dn4 = assign43220_e41487_d_n4;

        let (assign43230_e41499, assign43230_e41499_d_n4, assign43230_e41499_d_n7, assign43230_e41499_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign43230_e41493: f64 = (var_fn468_calc_ig__pg_paramin / var_fn468_calc_ig__phitin);
        let assign43230_e41495: f64 = (assign43230_e41493 * var_fn468_calc_ig__vgin);
        let assign43230_e41497: f64 = (assign43230_e41495 + var_fn468_calc_ig__expphib);
        (assign43230_e41497, (((-((var_fn468_calc_ig__pg_paramin * var_fn468_calc_ig__phitin_dn4) / (var_fn468_calc_ig__phitin * var_fn468_calc_ig__phitin))) * var_fn468_calc_ig__vgin) + var_fn468_calc_ig__expphib_dn4), (assign43230_e41493 * var_fn468_calc_ig__vgin_dn7), (assign43230_e41493 * var_fn468_calc_ig__vgin_dn8),)
    } else {
        (var_fn468_calc_ig__expiforarg, var_fn468_calc_ig__expiforarg_dn4, var_fn468_calc_ig__expiforarg_dn7, var_fn468_calc_ig__expiforarg_dn8,)
    }
};
        var_fn468_calc_ig__expiforarg = assign43230_e41499;
        var_fn468_calc_ig__expiforarg_dn4 = assign43230_e41499_d_n4;
        var_fn468_calc_ig__expiforarg_dn7 = assign43230_e41499_d_n7;
        var_fn468_calc_ig__expiforarg_dn8 = assign43230_e41499_d_n8;

        let (assign43240_e41543, assign43240_e41543_d_n4, assign43240_e41543_d_n7, assign43240_e41543_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign43240_e41509: f64 = (-50.0);
        let (assign43240_e41541, assign43240_e41541_d_n4, assign43240_e41541_d_n7, assign43240_e41541_d_n8,) = {
            if ((!(var_fn468_calc_ig__expiforarg > 50.0)) && (!(var_fn468_calc_ig__expiforarg < assign43240_e41509))) {
                let assign43240_e41514: f64 = (var_fn468_calc_ig__expiforarg).exp();
                (assign43240_e41514, (assign43240_e41514 * var_fn468_calc_ig__expiforarg_dn4), (assign43240_e41514 * var_fn468_calc_ig__expiforarg_dn7), (assign43240_e41514 * var_fn468_calc_ig__expiforarg_dn8),)
            } else {
                let assign43240_e41521: f64 = (-50.0);
                let (assign43240_e41540, assign43240_e41540_d_n4, assign43240_e41540_d_n7, assign43240_e41540_d_n8,) = {
                    if ((!(var_fn468_calc_ig__expiforarg > 50.0)) && (var_fn468_calc_ig__expiforarg < assign43240_e41521)) {
                        let assign43240_e41525: f64 = (-50.0);
                        let assign43240_e41526: f64 = (assign43240_e41525).exp();
                        (assign43240_e41526, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign43240_e41539, assign43240_e41539_d_n4, assign43240_e41539_d_n7, assign43240_e41539_d_n8,) = {
                            if (var_fn468_calc_ig__expiforarg > 50.0) {
                                let assign43240_e41531: f64 = (50.0_f64).exp();
                                let assign43240_e41535: f64 = (var_fn468_calc_ig__expiforarg - 50.0);
                                let assign43240_e41536: f64 = (1.0 + assign43240_e41535);
                                let assign43240_e41537: f64 = (assign43240_e41531 * assign43240_e41536);
                                (assign43240_e41537, (assign43240_e41531 * var_fn468_calc_ig__expiforarg_dn4), (assign43240_e41531 * var_fn468_calc_ig__expiforarg_dn7), (assign43240_e41531 * var_fn468_calc_ig__expiforarg_dn8),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign43240_e41539, assign43240_e41539_d_n4, assign43240_e41539_d_n7, assign43240_e41539_d_n8,)
                    }
                };
                (assign43240_e41540, assign43240_e41540_d_n4, assign43240_e41540_d_n7, assign43240_e41540_d_n8,)
            }
        };
        (assign43240_e41541, assign43240_e41541_d_n4, assign43240_e41541_d_n7, assign43240_e41541_d_n8,)
    } else {
        (var_fn468_calc_ig__expifor, var_fn468_calc_ig__expifor_dn4, var_fn468_calc_ig__expifor_dn7, var_fn468_calc_ig__expifor_dn8,)
    }
};
        var_fn468_calc_ig__expifor = assign43240_e41543;
        var_fn468_calc_ig__expifor_dn4 = assign43240_e41543_d_n4;
        var_fn468_calc_ig__expifor_dn7 = assign43240_e41543_d_n7;
        var_fn468_calc_ig__expifor_dn8 = assign43240_e41543_d_n8;

        let assign43250_e41546: f64 = if var_fn468_calc_ig__fracin == 1.0 { 1.0 } else { 0.0 };
        var_guard469 = assign43250_e41546;

        let (assign43260_e41562, assign43260_e41562_d_n4, assign43260_e41562_d_n7, assign43260_e41562_d_n8,) = {
    if (((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 != 0.0)) {
        let assign43260_e41556: f64 = (var_fn468_calc_ig__kbdgatein * var_fn468_calc_ig__iginbd);
        let assign43260_e41557: f64 = (var_fn468_calc_ig__expifor - assign43260_e41556);
        let assign43260_e41559: f64 = (assign43260_e41557 - var_fn468_calc_ig__t0);
        let assign43260_e41560: f64 = (var_fn468_calc_ig__isdiodeout * assign43260_e41559);
        (assign43260_e41560, ((var_fn468_calc_ig__isdiodeout_dn4 * assign43260_e41559) + (var_fn468_calc_ig__isdiodeout * ((var_fn468_calc_ig__expifor_dn4 - (var_fn468_calc_ig__kbdgatein * var_fn468_calc_ig__iginbd_dn4)) - var_fn468_calc_ig__t0_dn4))), (var_fn468_calc_ig__isdiodeout * (var_fn468_calc_ig__expifor_dn7 - (var_fn468_calc_ig__kbdgatein * var_fn468_calc_ig__iginbd_dn7))), (var_fn468_calc_ig__isdiodeout * (var_fn468_calc_ig__expifor_dn8 - (var_fn468_calc_ig__kbdgatein * var_fn468_calc_ig__iginbd_dn8))),)
    } else {
        (var_fn468_calc_ig__igindiode, var_fn468_calc_ig__igindiode_dn4, var_fn468_calc_ig__igindiode_dn7, var_fn468_calc_ig__igindiode_dn8,)
    }
};
        var_fn468_calc_ig__igindiode = assign43260_e41562;
        var_fn468_calc_ig__igindiode_dn4 = assign43260_e41562_d_n4;
        var_fn468_calc_ig__igindiode_dn7 = assign43260_e41562_d_n7;
        var_fn468_calc_ig__igindiode_dn8 = assign43260_e41562_d_n8;

        let (assign43270_e41578, assign43270_e41578_d_n4,) = {
    if (((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) {
        let assign43270_e41571: f64 = (-var_fn468_calc_ig__vgsatin);
        let assign43270_e41573: f64 = (assign43270_e41571 - var_fn468_calc_ig__vbdgin);
        let assign43270_e41574: f64 = (var_fn468_calc_ig__pbdgin * assign43270_e41573);
        let assign43270_e41576: f64 = (assign43270_e41574 + var_fn468_calc_ig__expphib);
        (assign43270_e41576, var_fn468_calc_ig__expphib_dn4,)
    } else {
        (var_fn468_calc_ig__expbdarg1_vgsat, var_fn468_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__expbdarg1_vgsat = assign43270_e41578;
        var_fn468_calc_ig__expbdarg1_vgsat_dn4 = assign43270_e41578_d_n4;

        let (assign43280_e41625, assign43280_e41625_d_n4,) = {
    if (((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) {
        let assign43280_e41591: f64 = (-50.0);
        let (assign43280_e41623, assign43280_e41623_d_n4,) = {
            if ((!(var_fn468_calc_ig__expbdarg1_vgsat > 50.0)) && (!(var_fn468_calc_ig__expbdarg1_vgsat < assign43280_e41591))) {
                let assign43280_e41596: f64 = (var_fn468_calc_ig__expbdarg1_vgsat).exp();
                (assign43280_e41596, (assign43280_e41596 * var_fn468_calc_ig__expbdarg1_vgsat_dn4),)
            } else {
                let assign43280_e41603: f64 = (-50.0);
                let (assign43280_e41622, assign43280_e41622_d_n4,) = {
                    if ((!(var_fn468_calc_ig__expbdarg1_vgsat > 50.0)) && (var_fn468_calc_ig__expbdarg1_vgsat < assign43280_e41603)) {
                        let assign43280_e41607: f64 = (-50.0);
                        let assign43280_e41608: f64 = (assign43280_e41607).exp();
                        (assign43280_e41608, 0.0,)
                    } else {
                        let (assign43280_e41621, assign43280_e41621_d_n4,) = {
                            if (var_fn468_calc_ig__expbdarg1_vgsat > 50.0) {
                                let assign43280_e41613: f64 = (50.0_f64).exp();
                                let assign43280_e41617: f64 = (var_fn468_calc_ig__expbdarg1_vgsat - 50.0);
                                let assign43280_e41618: f64 = (1.0 + assign43280_e41617);
                                let assign43280_e41619: f64 = (assign43280_e41613 * assign43280_e41618);
                                (assign43280_e41619, (assign43280_e41613 * var_fn468_calc_ig__expbdarg1_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign43280_e41621, assign43280_e41621_d_n4,)
                    }
                };
                (assign43280_e41622, assign43280_e41622_d_n4,)
            }
        };
        (assign43280_e41623, assign43280_e41623_d_n4,)
    } else {
        (var_fn468_calc_ig__expbd1_vgsat, var_fn468_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__expbd1_vgsat = assign43280_e41625;
        var_fn468_calc_ig__expbd1_vgsat_dn4 = assign43280_e41625_d_n4;

        let (assign43290_e41636, assign43290_e41636_d_n4,) = {
    if (((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) {
        let assign43290_e41634: f64 = (var_fn468_calc_ig__expbd1_vgsat - var_fn468_calc_ig__expbd2);
        (assign43290_e41634, (var_fn468_calc_ig__expbd1_vgsat_dn4 - var_fn468_calc_ig__expbd2_dn4),)
    } else {
        (var_fn468_calc_ig__iginbd_vgsat, var_fn468_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__iginbd_vgsat = assign43290_e41636;
        var_fn468_calc_ig__iginbd_vgsat_dn4 = assign43290_e41636_d_n4;

        let (assign43300_e41651, assign43300_e41651_d_n4,) = {
    if (((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) {
        let assign43300_e41645: f64 = (var_fn468_calc_ig__pg_paramin / var_fn468_calc_ig__phitin);
        let assign43300_e41647: f64 = (assign43300_e41645 * var_fn468_calc_ig__vgsatin);
        let assign43300_e41649: f64 = (assign43300_e41647 + var_fn468_calc_ig__expphib);
        (assign43300_e41649, (((-((var_fn468_calc_ig__pg_paramin * var_fn468_calc_ig__phitin_dn4) / (var_fn468_calc_ig__phitin * var_fn468_calc_ig__phitin))) * var_fn468_calc_ig__vgsatin) + var_fn468_calc_ig__expphib_dn4),)
    } else {
        (var_fn468_calc_ig__expiforarg_nohinj_vgsat, var_fn468_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__expiforarg_nohinj_vgsat = assign43300_e41651;
        var_fn468_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign43300_e41651_d_n4;

        let (assign43310_e41698, assign43310_e41698_d_n4,) = {
    if (((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) {
        let assign43310_e41664: f64 = (-50.0);
        let (assign43310_e41696, assign43310_e41696_d_n4,) = {
            if ((!(var_fn468_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (!(var_fn468_calc_ig__expiforarg_nohinj_vgsat < assign43310_e41664))) {
                let assign43310_e41669: f64 = (var_fn468_calc_ig__expiforarg_nohinj_vgsat).exp();
                (assign43310_e41669, (assign43310_e41669 * var_fn468_calc_ig__expiforarg_nohinj_vgsat_dn4),)
            } else {
                let assign43310_e41676: f64 = (-50.0);
                let (assign43310_e41695, assign43310_e41695_d_n4,) = {
                    if ((!(var_fn468_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (var_fn468_calc_ig__expiforarg_nohinj_vgsat < assign43310_e41676)) {
                        let assign43310_e41680: f64 = (-50.0);
                        let assign43310_e41681: f64 = (assign43310_e41680).exp();
                        (assign43310_e41681, 0.0,)
                    } else {
                        let (assign43310_e41694, assign43310_e41694_d_n4,) = {
                            if (var_fn468_calc_ig__expiforarg_nohinj_vgsat > 50.0) {
                                let assign43310_e41686: f64 = (50.0_f64).exp();
                                let assign43310_e41690: f64 = (var_fn468_calc_ig__expiforarg_nohinj_vgsat - 50.0);
                                let assign43310_e41691: f64 = (1.0 + assign43310_e41690);
                                let assign43310_e41692: f64 = (assign43310_e41686 * assign43310_e41691);
                                (assign43310_e41692, (assign43310_e41686 * var_fn468_calc_ig__expiforarg_nohinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign43310_e41694, assign43310_e41694_d_n4,)
                    }
                };
                (assign43310_e41695, assign43310_e41695_d_n4,)
            }
        };
        (assign43310_e41696, assign43310_e41696_d_n4,)
    } else {
        (var_fn468_calc_ig__expifor_nohinj_vgsat, var_fn468_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__expifor_nohinj_vgsat = assign43310_e41698;
        var_fn468_calc_ig__expifor_nohinj_vgsat_dn4 = assign43310_e41698_d_n4;

        let (assign43320_e41713, assign43320_e41713_d_n4,) = {
    if (((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) {
        let assign43320_e41708: f64 = (var_fn468_calc_ig__kbdgatein * var_fn468_calc_ig__iginbd_vgsat);
        let assign43320_e41709: f64 = (var_fn468_calc_ig__expifor_nohinj_vgsat - assign43320_e41708);
        let assign43320_e41711: f64 = (assign43320_e41709 - var_fn468_calc_ig__t0);
        (assign43320_e41711, ((var_fn468_calc_ig__expifor_nohinj_vgsat_dn4 - (var_fn468_calc_ig__kbdgatein * var_fn468_calc_ig__iginbd_vgsat_dn4)) - var_fn468_calc_ig__t0_dn4),)
    } else {
        (var_fn468_calc_ig__igindiode_nohinj_vgsat, var_fn468_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__igindiode_nohinj_vgsat = assign43320_e41713;
        var_fn468_calc_ig__igindiode_nohinj_vgsat_dn4 = assign43320_e41713_d_n4;

        let (assign43330_e41730, assign43330_e41730_d_n4, assign43330_e41730_d_n7, assign43330_e41730_d_n8,) = {
    if (((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) {
        let assign43330_e41724: f64 = (var_fn468_calc_ig__kbdgatein * var_fn468_calc_ig__iginbd);
        let assign43330_e41725: f64 = (var_fn468_calc_ig__expifor - assign43330_e41724);
        let assign43330_e41727: f64 = (assign43330_e41725 - var_fn468_calc_ig__t0);
        let assign43330_e41728: f64 = (var_fn468_calc_ig__isdiodeout * assign43330_e41727);
        (assign43330_e41728, ((var_fn468_calc_ig__isdiodeout_dn4 * assign43330_e41727) + (var_fn468_calc_ig__isdiodeout * ((var_fn468_calc_ig__expifor_dn4 - (var_fn468_calc_ig__kbdgatein * var_fn468_calc_ig__iginbd_dn4)) - var_fn468_calc_ig__t0_dn4))), (var_fn468_calc_ig__isdiodeout * (var_fn468_calc_ig__expifor_dn7 - (var_fn468_calc_ig__kbdgatein * var_fn468_calc_ig__iginbd_dn7))), (var_fn468_calc_ig__isdiodeout * (var_fn468_calc_ig__expifor_dn8 - (var_fn468_calc_ig__kbdgatein * var_fn468_calc_ig__iginbd_dn8))),)
    } else {
        (var_fn468_calc_ig__igindiode_nohinj, var_fn468_calc_ig__igindiode_nohinj_dn4, var_fn468_calc_ig__igindiode_nohinj_dn7, var_fn468_calc_ig__igindiode_nohinj_dn8,)
    }
};
        var_fn468_calc_ig__igindiode_nohinj = assign43330_e41730;
        var_fn468_calc_ig__igindiode_nohinj_dn4 = assign43330_e41730_d_n4;
        var_fn468_calc_ig__igindiode_nohinj_dn7 = assign43330_e41730_d_n7;
        var_fn468_calc_ig__igindiode_nohinj_dn8 = assign43330_e41730_d_n8;

        let assign43340_e41733: f64 = if var_fn468_calc_ig__fracin > 0.0 { 1.0 } else { 0.0 };
        var_guard470 = assign43340_e41733;

        let (assign43350_e41746,) = {
    if ((((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) && (var_guard470 != 0.0)) {
        let assign43350_e41744: f64 = (var_fn468_calc_ig__fracin * var_fn468_calc_ig__pg_paramin);
        (assign43350_e41744,)
    } else {
        (var_fn468_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn468_calc_ig__pg_paramin_hinj = assign43350_e41746;

        let (assign43360_e41763, assign43360_e41763_d_n4,) = {
    if ((((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) && (var_guard470 != 0.0)) {
        let assign43360_e41757: f64 = (var_fn468_calc_ig__pg_paramin_hinj / var_fn468_calc_ig__phitin);
        let assign43360_e41759: f64 = (assign43360_e41757 * var_fn468_calc_ig__vgsatin);
        let assign43360_e41761: f64 = (assign43360_e41759 + var_fn468_calc_ig__expphib);
        (assign43360_e41761, (((-((var_fn468_calc_ig__pg_paramin_hinj * var_fn468_calc_ig__phitin_dn4) / (var_fn468_calc_ig__phitin * var_fn468_calc_ig__phitin))) * var_fn468_calc_ig__vgsatin) + var_fn468_calc_ig__expphib_dn4),)
    } else {
        (var_fn468_calc_ig__expiforarg_hinj_vgsat, var_fn468_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__expiforarg_hinj_vgsat = assign43360_e41763;
        var_fn468_calc_ig__expiforarg_hinj_vgsat_dn4 = assign43360_e41763_d_n4;

        let (assign43370_e41812, assign43370_e41812_d_n4,) = {
    if ((((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) && (var_guard470 != 0.0)) {
        let assign43370_e41778: f64 = (-50.0);
        let (assign43370_e41810, assign43370_e41810_d_n4,) = {
            if ((!(var_fn468_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (!(var_fn468_calc_ig__expiforarg_hinj_vgsat < assign43370_e41778))) {
                let assign43370_e41783: f64 = (var_fn468_calc_ig__expiforarg_hinj_vgsat).exp();
                (assign43370_e41783, (assign43370_e41783 * var_fn468_calc_ig__expiforarg_hinj_vgsat_dn4),)
            } else {
                let assign43370_e41790: f64 = (-50.0);
                let (assign43370_e41809, assign43370_e41809_d_n4,) = {
                    if ((!(var_fn468_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (var_fn468_calc_ig__expiforarg_hinj_vgsat < assign43370_e41790)) {
                        let assign43370_e41794: f64 = (-50.0);
                        let assign43370_e41795: f64 = (assign43370_e41794).exp();
                        (assign43370_e41795, 0.0,)
                    } else {
                        let (assign43370_e41808, assign43370_e41808_d_n4,) = {
                            if (var_fn468_calc_ig__expiforarg_hinj_vgsat > 50.0) {
                                let assign43370_e41800: f64 = (50.0_f64).exp();
                                let assign43370_e41804: f64 = (var_fn468_calc_ig__expiforarg_hinj_vgsat - 50.0);
                                let assign43370_e41805: f64 = (1.0 + assign43370_e41804);
                                let assign43370_e41806: f64 = (assign43370_e41800 * assign43370_e41805);
                                (assign43370_e41806, (assign43370_e41800 * var_fn468_calc_ig__expiforarg_hinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign43370_e41808, assign43370_e41808_d_n4,)
                    }
                };
                (assign43370_e41809, assign43370_e41809_d_n4,)
            }
        };
        (assign43370_e41810, assign43370_e41810_d_n4,)
    } else {
        (var_fn468_calc_ig__expifor_hinj_vgsat, var_fn468_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__expifor_hinj_vgsat = assign43370_e41812;
        var_fn468_calc_ig__expifor_hinj_vgsat_dn4 = assign43370_e41812_d_n4;

        let (assign43380_e41829, assign43380_e41829_d_n4,) = {
    if ((((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) && (var_guard470 != 0.0)) {
        let assign43380_e41824: f64 = (var_fn468_calc_ig__kbdgatein * var_fn468_calc_ig__iginbd_vgsat);
        let assign43380_e41825: f64 = (var_fn468_calc_ig__expifor_hinj_vgsat - assign43380_e41824);
        let assign43380_e41827: f64 = (assign43380_e41825 - var_fn468_calc_ig__t0);
        (assign43380_e41827, ((var_fn468_calc_ig__expifor_hinj_vgsat_dn4 - (var_fn468_calc_ig__kbdgatein * var_fn468_calc_ig__iginbd_vgsat_dn4)) - var_fn468_calc_ig__t0_dn4),)
    } else {
        (var_fn468_calc_ig__igindiode_hinj_vgsat, var_fn468_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn468_calc_ig__igindiode_hinj_vgsat = assign43380_e41829;
        var_fn468_calc_ig__igindiode_hinj_vgsat_dn4 = assign43380_e41829_d_n4;

        let (assign43390_e41846, assign43390_e41846_d_n4, assign43390_e41846_d_n7, assign43390_e41846_d_n8,) = {
    if ((((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) && (var_guard470 != 0.0)) {
        let assign43390_e41840: f64 = (var_fn468_calc_ig__pg_paramin_hinj / var_fn468_calc_ig__phitin);
        let assign43390_e41842: f64 = (assign43390_e41840 * var_fn468_calc_ig__vgin);
        let assign43390_e41844: f64 = (assign43390_e41842 + var_fn468_calc_ig__expphib);
        (assign43390_e41844, (((-((var_fn468_calc_ig__pg_paramin_hinj * var_fn468_calc_ig__phitin_dn4) / (var_fn468_calc_ig__phitin * var_fn468_calc_ig__phitin))) * var_fn468_calc_ig__vgin) + var_fn468_calc_ig__expphib_dn4), (assign43390_e41840 * var_fn468_calc_ig__vgin_dn7), (assign43390_e41840 * var_fn468_calc_ig__vgin_dn8),)
    } else {
        (var_fn468_calc_ig__expiforarg_hinj, var_fn468_calc_ig__expiforarg_hinj_dn4, var_fn468_calc_ig__expiforarg_hinj_dn7, var_fn468_calc_ig__expiforarg_hinj_dn8,)
    }
};
        var_fn468_calc_ig__expiforarg_hinj = assign43390_e41846;
        var_fn468_calc_ig__expiforarg_hinj_dn4 = assign43390_e41846_d_n4;
        var_fn468_calc_ig__expiforarg_hinj_dn7 = assign43390_e41846_d_n7;
        var_fn468_calc_ig__expiforarg_hinj_dn8 = assign43390_e41846_d_n8;

        let (assign43400_e41895, assign43400_e41895_d_n4, assign43400_e41895_d_n7, assign43400_e41895_d_n8,) = {
    if ((((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) && (var_guard470 != 0.0)) {
        let assign43400_e41861: f64 = (-50.0);
        let (assign43400_e41893, assign43400_e41893_d_n4, assign43400_e41893_d_n7, assign43400_e41893_d_n8,) = {
            if ((!(var_fn468_calc_ig__expiforarg_hinj > 50.0)) && (!(var_fn468_calc_ig__expiforarg_hinj < assign43400_e41861))) {
                let assign43400_e41866: f64 = (var_fn468_calc_ig__expiforarg_hinj).exp();
                (assign43400_e41866, (assign43400_e41866 * var_fn468_calc_ig__expiforarg_hinj_dn4), (assign43400_e41866 * var_fn468_calc_ig__expiforarg_hinj_dn7), (assign43400_e41866 * var_fn468_calc_ig__expiforarg_hinj_dn8),)
            } else {
                let assign43400_e41873: f64 = (-50.0);
                let (assign43400_e41892, assign43400_e41892_d_n4, assign43400_e41892_d_n7, assign43400_e41892_d_n8,) = {
                    if ((!(var_fn468_calc_ig__expiforarg_hinj > 50.0)) && (var_fn468_calc_ig__expiforarg_hinj < assign43400_e41873)) {
                        let assign43400_e41877: f64 = (-50.0);
                        let assign43400_e41878: f64 = (assign43400_e41877).exp();
                        (assign43400_e41878, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign43400_e41891, assign43400_e41891_d_n4, assign43400_e41891_d_n7, assign43400_e41891_d_n8,) = {
                            if (var_fn468_calc_ig__expiforarg_hinj > 50.0) {
                                let assign43400_e41883: f64 = (50.0_f64).exp();
                                let assign43400_e41887: f64 = (var_fn468_calc_ig__expiforarg_hinj - 50.0);
                                let assign43400_e41888: f64 = (1.0 + assign43400_e41887);
                                let assign43400_e41889: f64 = (assign43400_e41883 * assign43400_e41888);
                                (assign43400_e41889, (assign43400_e41883 * var_fn468_calc_ig__expiforarg_hinj_dn4), (assign43400_e41883 * var_fn468_calc_ig__expiforarg_hinj_dn7), (assign43400_e41883 * var_fn468_calc_ig__expiforarg_hinj_dn8),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign43400_e41891, assign43400_e41891_d_n4, assign43400_e41891_d_n7, assign43400_e41891_d_n8,)
                    }
                };
                (assign43400_e41892, assign43400_e41892_d_n4, assign43400_e41892_d_n7, assign43400_e41892_d_n8,)
            }
        };
        (assign43400_e41893, assign43400_e41893_d_n4, assign43400_e41893_d_n7, assign43400_e41893_d_n8,)
    } else {
        (var_fn468_calc_ig__expifor_hinj, var_fn468_calc_ig__expifor_hinj_dn4, var_fn468_calc_ig__expifor_hinj_dn7, var_fn468_calc_ig__expifor_hinj_dn8,)
    }
};
        var_fn468_calc_ig__expifor_hinj = assign43400_e41895;
        var_fn468_calc_ig__expifor_hinj_dn4 = assign43400_e41895_d_n4;
        var_fn468_calc_ig__expifor_hinj_dn7 = assign43400_e41895_d_n7;
        var_fn468_calc_ig__expifor_hinj_dn8 = assign43400_e41895_d_n8;

        let (assign43410_e41910, assign43410_e41910_d_n4,) = {
    if ((((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) && (var_guard470 != 0.0)) {
        let assign43410_e41906: f64 = (var_fn468_calc_ig__isdiodeout * var_fn468_calc_ig__igindiode_nohinj_vgsat);
        let assign43410_e41908: f64 = (assign43410_e41906 / var_fn468_calc_ig__igindiode_hinj_vgsat);
        (assign43410_e41908, (((((var_fn468_calc_ig__isdiodeout_dn4 * var_fn468_calc_ig__igindiode_nohinj_vgsat) + (var_fn468_calc_ig__isdiodeout * var_fn468_calc_ig__igindiode_nohinj_vgsat_dn4)) * var_fn468_calc_ig__igindiode_hinj_vgsat) - (assign43410_e41906 * var_fn468_calc_ig__igindiode_hinj_vgsat_dn4)) / (var_fn468_calc_ig__igindiode_hinj_vgsat * var_fn468_calc_ig__igindiode_hinj_vgsat)),)
    } else {
        (var_fn468_calc_ig__igindiode_hinj_pre, var_fn468_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn468_calc_ig__igindiode_hinj_pre = assign43410_e41910;
        var_fn468_calc_ig__igindiode_hinj_pre_dn4 = assign43410_e41910_d_n4;

        let (assign43420_e41929, assign43420_e41929_d_n4, assign43420_e41929_d_n7, assign43420_e41929_d_n8,) = {
    if ((((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) && (var_guard470 != 0.0)) {
        let assign43420_e41923: f64 = (var_fn468_calc_ig__kbdgatein * var_fn468_calc_ig__iginbd);
        let assign43420_e41924: f64 = (var_fn468_calc_ig__expifor_hinj - assign43420_e41923);
        let assign43420_e41926: f64 = (assign43420_e41924 - var_fn468_calc_ig__t0);
        let assign43420_e41927: f64 = (var_fn468_calc_ig__igindiode_hinj_pre * assign43420_e41926);
        (assign43420_e41927, ((var_fn468_calc_ig__igindiode_hinj_pre_dn4 * assign43420_e41926) + (var_fn468_calc_ig__igindiode_hinj_pre * ((var_fn468_calc_ig__expifor_hinj_dn4 - (var_fn468_calc_ig__kbdgatein * var_fn468_calc_ig__iginbd_dn4)) - var_fn468_calc_ig__t0_dn4))), (var_fn468_calc_ig__igindiode_hinj_pre * (var_fn468_calc_ig__expifor_hinj_dn7 - (var_fn468_calc_ig__kbdgatein * var_fn468_calc_ig__iginbd_dn7))), (var_fn468_calc_ig__igindiode_hinj_pre * (var_fn468_calc_ig__expifor_hinj_dn8 - (var_fn468_calc_ig__kbdgatein * var_fn468_calc_ig__iginbd_dn8))),)
    } else {
        (var_fn468_calc_ig__igindiode_hinj, var_fn468_calc_ig__igindiode_hinj_dn4, var_fn468_calc_ig__igindiode_hinj_dn7, var_fn468_calc_ig__igindiode_hinj_dn8,)
    }
};
        var_fn468_calc_ig__igindiode_hinj = assign43420_e41929;
        var_fn468_calc_ig__igindiode_hinj_dn4 = assign43420_e41929_d_n4;
        var_fn468_calc_ig__igindiode_hinj_dn7 = assign43420_e41929_d_n7;
        var_fn468_calc_ig__igindiode_hinj_dn8 = assign43420_e41929_d_n8;

        let (assign43430_e41943, assign43430_e41943_d_n4, assign43430_e41943_d_n7, assign43430_e41943_d_n8,) = {
    if ((((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) && (var_guard470 == 0.0)) {
        let assign43430_e41941: f64 = (var_fn468_calc_ig__isdiodeout * var_fn468_calc_ig__igindiode_nohinj_vgsat);
        (assign43430_e41941, ((var_fn468_calc_ig__isdiodeout_dn4 * var_fn468_calc_ig__igindiode_nohinj_vgsat) + (var_fn468_calc_ig__isdiodeout * var_fn468_calc_ig__igindiode_nohinj_vgsat_dn4)), 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__igindiode_hinj, var_fn468_calc_ig__igindiode_hinj_dn4, var_fn468_calc_ig__igindiode_hinj_dn7, var_fn468_calc_ig__igindiode_hinj_dn8,)
    }
};
        var_fn468_calc_ig__igindiode_hinj = assign43430_e41943;
        var_fn468_calc_ig__igindiode_hinj_dn4 = assign43430_e41943_d_n4;
        var_fn468_calc_ig__igindiode_hinj_dn7 = assign43430_e41943_d_n7;
        var_fn468_calc_ig__igindiode_hinj_dn8 = assign43430_e41943_d_n8;

        let (assign43440_e41956, assign43440_e41956_d_n4,) = {
    if (((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) {
        let assign43440_e41952: f64 = (var_fn468_calc_ig__alphagin * var_fn468_calc_ig__alphagin);
        let assign43440_e41954: f64 = (assign43440_e41952 * var_fn468_calc_ig__phitin);
        (assign43440_e41954, (assign43440_e41952 * var_fn468_calc_ig__phitin_dn4),)
    } else {
        (var_fn468_calc_ig__alpha2_phit, var_fn468_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn468_calc_ig__alpha2_phit = assign43440_e41956;
        var_fn468_calc_ig__alpha2_phit_dn4 = assign43440_e41956_d_n4;

        let (assign43450_e41973, assign43450_e41973_d_n4, assign43450_e41973_d_n7, assign43450_e41973_d_n8,) = {
    if (((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) {
        let assign43450_e41967: f64 = (var_fn468_calc_ig__alpha2_phit / 2.0);
        let assign43450_e41968: f64 = (var_fn468_calc_ig__vgsatin - assign43450_e41967);
        let assign43450_e41969: f64 = (var_fn468_calc_ig__vgin - assign43450_e41968);
        let assign43450_e41971: f64 = (assign43450_e41969 / var_fn468_calc_ig__alpha2_phit);
        (assign43450_e41971, ((((-(-(var_fn468_calc_ig__alpha2_phit_dn4 / 2.0))) * var_fn468_calc_ig__alpha2_phit) - (assign43450_e41969 * var_fn468_calc_ig__alpha2_phit_dn4)) / (var_fn468_calc_ig__alpha2_phit * var_fn468_calc_ig__alpha2_phit)), (var_fn468_calc_ig__vgin_dn7 / var_fn468_calc_ig__alpha2_phit), (var_fn468_calc_ig__vgin_dn8 / var_fn468_calc_ig__alpha2_phit),)
    } else {
        (var_fn468_calc_ig__expffvarg, var_fn468_calc_ig__expffvarg_dn4, var_fn468_calc_ig__expffvarg_dn7, var_fn468_calc_ig__expffvarg_dn8,)
    }
};
        var_fn468_calc_ig__expffvarg = assign43450_e41973;
        var_fn468_calc_ig__expffvarg_dn4 = assign43450_e41973_d_n4;
        var_fn468_calc_ig__expffvarg_dn7 = assign43450_e41973_d_n7;
        var_fn468_calc_ig__expffvarg_dn8 = assign43450_e41973_d_n8;

        let assign43460_e41976: f64 = if var_fn468_calc_ig__expffvarg > 50.0 { 1.0 } else { 0.0 };
        var_guard471 = assign43460_e41976;

        let (assign43470_e41987, assign43470_e41987_d_n4, assign43470_e41987_d_n7, assign43470_e41987_d_n8,) = {
    if ((((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) && (var_guard471 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__ffvgin, var_fn468_calc_ig__ffvgin_dn4, var_fn468_calc_ig__ffvgin_dn7, var_fn468_calc_ig__ffvgin_dn8,)
    }
};
        var_fn468_calc_ig__ffvgin = assign43470_e41987;
        var_fn468_calc_ig__ffvgin_dn4 = assign43470_e41987_d_n4;
        var_fn468_calc_ig__ffvgin_dn7 = assign43470_e41987_d_n7;
        var_fn468_calc_ig__ffvgin_dn8 = assign43470_e41987_d_n8;

        let assign43480_e41990: f64 = (-50.0);
        let assign43480_e41991: f64 = if var_fn468_calc_ig__expffvarg < assign43480_e41990 { 1.0 } else { 0.0 };
        var_guard472 = assign43480_e41991;

        let (assign43490_e42005, assign43490_e42005_d_n4, assign43490_e42005_d_n7, assign43490_e42005_d_n8,) = {
    if (((((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) && (var_guard471 == 0.0)) && (var_guard472 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn468_calc_ig__ffvgin, var_fn468_calc_ig__ffvgin_dn4, var_fn468_calc_ig__ffvgin_dn7, var_fn468_calc_ig__ffvgin_dn8,)
    }
};
        var_fn468_calc_ig__ffvgin = assign43490_e42005;
        var_fn468_calc_ig__ffvgin_dn4 = assign43490_e42005_d_n4;
        var_fn468_calc_ig__ffvgin_dn7 = assign43490_e42005_d_n7;
        var_fn468_calc_ig__ffvgin_dn8 = assign43490_e42005_d_n8;

        let (assign43500_e42025, assign43500_e42025_d_n4, assign43500_e42025_d_n7, assign43500_e42025_d_n8,) = {
    if (((((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) && (var_guard471 == 0.0)) && (var_guard472 == 0.0)) {
        let assign43500_e42021: f64 = (var_fn468_calc_ig__expffvarg).exp();
        let assign43500_e42022: f64 = (1.0 + assign43500_e42021);
        let assign43500_e42023: f64 = (1.0 / assign43500_e42022);
        (assign43500_e42023, (-((assign43500_e42021 * var_fn468_calc_ig__expffvarg_dn4) / (assign43500_e42022 * assign43500_e42022))), (-((assign43500_e42021 * var_fn468_calc_ig__expffvarg_dn7) / (assign43500_e42022 * assign43500_e42022))), (-((assign43500_e42021 * var_fn468_calc_ig__expffvarg_dn8) / (assign43500_e42022 * assign43500_e42022))),)
    } else {
        (var_fn468_calc_ig__ffvgin, var_fn468_calc_ig__ffvgin_dn4, var_fn468_calc_ig__ffvgin_dn7, var_fn468_calc_ig__ffvgin_dn8,)
    }
};
        var_fn468_calc_ig__ffvgin = assign43500_e42025;
        var_fn468_calc_ig__ffvgin_dn4 = assign43500_e42025_d_n4;
        var_fn468_calc_ig__ffvgin_dn7 = assign43500_e42025_d_n7;
        var_fn468_calc_ig__ffvgin_dn8 = assign43500_e42025_d_n8;

        let (assign43510_e42042, assign43510_e42042_d_n4, assign43510_e42042_d_n7, assign43510_e42042_d_n8,) = {
    if (((var_guard461 != 0.0) && (var_guard467 != 0.0)) && (var_guard469 == 0.0)) {
        let assign43510_e42034: f64 = (var_fn468_calc_ig__ffvgin * var_fn468_calc_ig__igindiode_nohinj);
        let assign43510_e42037: f64 = (1.0 - var_fn468_calc_ig__ffvgin);
        let assign43510_e42039: f64 = (assign43510_e42037 * var_fn468_calc_ig__igindiode_hinj);
        let assign43510_e42040: f64 = (assign43510_e42034 + assign43510_e42039);
        (assign43510_e42040, (((var_fn468_calc_ig__ffvgin_dn4 * var_fn468_calc_ig__igindiode_nohinj) + (var_fn468_calc_ig__ffvgin * var_fn468_calc_ig__igindiode_nohinj_dn4)) + (((-var_fn468_calc_ig__ffvgin_dn4) * var_fn468_calc_ig__igindiode_hinj) + (assign43510_e42037 * var_fn468_calc_ig__igindiode_hinj_dn4))), (((var_fn468_calc_ig__ffvgin_dn7 * var_fn468_calc_ig__igindiode_nohinj) + (var_fn468_calc_ig__ffvgin * var_fn468_calc_ig__igindiode_nohinj_dn7)) + (((-var_fn468_calc_ig__ffvgin_dn7) * var_fn468_calc_ig__igindiode_hinj) + (assign43510_e42037 * var_fn468_calc_ig__igindiode_hinj_dn7))), (((var_fn468_calc_ig__ffvgin_dn8 * var_fn468_calc_ig__igindiode_nohinj) + (var_fn468_calc_ig__ffvgin * var_fn468_calc_ig__igindiode_nohinj_dn8)) + (((-var_fn468_calc_ig__ffvgin_dn8) * var_fn468_calc_ig__igindiode_hinj) + (assign43510_e42037 * var_fn468_calc_ig__igindiode_hinj_dn8))),)
    } else {
        (var_fn468_calc_ig__igindiode, var_fn468_calc_ig__igindiode_dn4, var_fn468_calc_ig__igindiode_dn7, var_fn468_calc_ig__igindiode_dn8,)
    }
};
        var_fn468_calc_ig__igindiode = assign43510_e42042;
        var_fn468_calc_ig__igindiode_dn4 = assign43510_e42042_d_n4;
        var_fn468_calc_ig__igindiode_dn7 = assign43510_e42042_d_n7;
        var_fn468_calc_ig__igindiode_dn8 = assign43510_e42042_d_n8;

        *var_fn468_calc_ig__alpha2_phit_slot = var_fn468_calc_ig__alpha2_phit;
        *var_fn468_calc_ig__alpha2_phit_dn4_slot = var_fn468_calc_ig__alpha2_phit_dn4;
        *var_fn468_calc_ig__expbd1_vgsat_slot = var_fn468_calc_ig__expbd1_vgsat;
        *var_fn468_calc_ig__expbd1_vgsat_dn4_slot = var_fn468_calc_ig__expbd1_vgsat_dn4;
        *var_fn468_calc_ig__expbdarg1_vgsat_slot = var_fn468_calc_ig__expbdarg1_vgsat;
        *var_fn468_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn468_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn468_calc_ig__expffvarg_slot = var_fn468_calc_ig__expffvarg;
        *var_fn468_calc_ig__expffvarg_dn4_slot = var_fn468_calc_ig__expffvarg_dn4;
        *var_fn468_calc_ig__expffvarg_dn7_slot = var_fn468_calc_ig__expffvarg_dn7;
        *var_fn468_calc_ig__expffvarg_dn8_slot = var_fn468_calc_ig__expffvarg_dn8;
        *var_fn468_calc_ig__expifor_slot = var_fn468_calc_ig__expifor;
        *var_fn468_calc_ig__expifor_dn4_slot = var_fn468_calc_ig__expifor_dn4;
        *var_fn468_calc_ig__expifor_dn7_slot = var_fn468_calc_ig__expifor_dn7;
        *var_fn468_calc_ig__expifor_dn8_slot = var_fn468_calc_ig__expifor_dn8;
        *var_fn468_calc_ig__expifor_hinj_slot = var_fn468_calc_ig__expifor_hinj;
        *var_fn468_calc_ig__expifor_hinj_dn4_slot = var_fn468_calc_ig__expifor_hinj_dn4;
        *var_fn468_calc_ig__expifor_hinj_dn7_slot = var_fn468_calc_ig__expifor_hinj_dn7;
        *var_fn468_calc_ig__expifor_hinj_dn8_slot = var_fn468_calc_ig__expifor_hinj_dn8;
        *var_fn468_calc_ig__expifor_hinj_vgsat_slot = var_fn468_calc_ig__expifor_hinj_vgsat;
        *var_fn468_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn468_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn468_calc_ig__expifor_nohinj_vgsat_slot = var_fn468_calc_ig__expifor_nohinj_vgsat;
        *var_fn468_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn468_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn468_calc_ig__expiforarg_slot = var_fn468_calc_ig__expiforarg;
        *var_fn468_calc_ig__expiforarg_dn4_slot = var_fn468_calc_ig__expiforarg_dn4;
        *var_fn468_calc_ig__expiforarg_dn7_slot = var_fn468_calc_ig__expiforarg_dn7;
        *var_fn468_calc_ig__expiforarg_dn8_slot = var_fn468_calc_ig__expiforarg_dn8;
        *var_fn468_calc_ig__expiforarg_hinj_slot = var_fn468_calc_ig__expiforarg_hinj;
        *var_fn468_calc_ig__expiforarg_hinj_dn4_slot = var_fn468_calc_ig__expiforarg_hinj_dn4;
        *var_fn468_calc_ig__expiforarg_hinj_dn7_slot = var_fn468_calc_ig__expiforarg_hinj_dn7;
        *var_fn468_calc_ig__expiforarg_hinj_dn8_slot = var_fn468_calc_ig__expiforarg_hinj_dn8;
        *var_fn468_calc_ig__expiforarg_hinj_vgsat_slot = var_fn468_calc_ig__expiforarg_hinj_vgsat;
        *var_fn468_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn468_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn468_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn468_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn468_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn468_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn468_calc_ig__ffvgin_slot = var_fn468_calc_ig__ffvgin;
        *var_fn468_calc_ig__ffvgin_dn4_slot = var_fn468_calc_ig__ffvgin_dn4;
        *var_fn468_calc_ig__ffvgin_dn7_slot = var_fn468_calc_ig__ffvgin_dn7;
        *var_fn468_calc_ig__ffvgin_dn8_slot = var_fn468_calc_ig__ffvgin_dn8;
        *var_fn468_calc_ig__iginbd_vgsat_slot = var_fn468_calc_ig__iginbd_vgsat;
        *var_fn468_calc_ig__iginbd_vgsat_dn4_slot = var_fn468_calc_ig__iginbd_vgsat_dn4;
        *var_fn468_calc_ig__igindiode_slot = var_fn468_calc_ig__igindiode;
        *var_fn468_calc_ig__igindiode_dn4_slot = var_fn468_calc_ig__igindiode_dn4;
        *var_fn468_calc_ig__igindiode_dn7_slot = var_fn468_calc_ig__igindiode_dn7;
        *var_fn468_calc_ig__igindiode_dn8_slot = var_fn468_calc_ig__igindiode_dn8;
        *var_fn468_calc_ig__igindiode_hinj_slot = var_fn468_calc_ig__igindiode_hinj;
        *var_fn468_calc_ig__igindiode_hinj_dn4_slot = var_fn468_calc_ig__igindiode_hinj_dn4;
        *var_fn468_calc_ig__igindiode_hinj_dn7_slot = var_fn468_calc_ig__igindiode_hinj_dn7;
        *var_fn468_calc_ig__igindiode_hinj_dn8_slot = var_fn468_calc_ig__igindiode_hinj_dn8;
        *var_fn468_calc_ig__igindiode_hinj_pre_slot = var_fn468_calc_ig__igindiode_hinj_pre;
        *var_fn468_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn468_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn468_calc_ig__igindiode_hinj_vgsat_slot = var_fn468_calc_ig__igindiode_hinj_vgsat;
        *var_fn468_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn468_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn468_calc_ig__igindiode_nohinj_slot = var_fn468_calc_ig__igindiode_nohinj;
        *var_fn468_calc_ig__igindiode_nohinj_dn4_slot = var_fn468_calc_ig__igindiode_nohinj_dn4;
        *var_fn468_calc_ig__igindiode_nohinj_dn7_slot = var_fn468_calc_ig__igindiode_nohinj_dn7;
        *var_fn468_calc_ig__igindiode_nohinj_dn8_slot = var_fn468_calc_ig__igindiode_nohinj_dn8;
        *var_fn468_calc_ig__igindiode_nohinj_vgsat_slot = var_fn468_calc_ig__igindiode_nohinj_vgsat;
        *var_fn468_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn468_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn468_calc_ig__isdiodeout_slot = var_fn468_calc_ig__isdiodeout;
        *var_fn468_calc_ig__isdiodeout_dn4_slot = var_fn468_calc_ig__isdiodeout_dn4;
        *var_fn468_calc_ig__pg_paramin_hinj_slot = var_fn468_calc_ig__pg_paramin_hinj;
        *var_guard469_slot = var_guard469;
        *var_guard470_slot = var_guard470;
        *var_guard471_slot = var_guard471;
        *var_guard472_slot = var_guard472;
    }

    pub(super) fn stamp_transient_block_109(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_fn468_calc_ig__betarecin: f64,
        var_fn468_calc_ig__igindiode: f64,
        var_fn468_calc_ig__igindiode_dn4: f64,
        var_fn468_calc_ig__igindiode_dn7: f64,
        var_fn468_calc_ig__igindiode_dn8: f64,
        var_fn468_calc_ig__irecin: f64,
        var_fn468_calc_ig__ngf: f64,
        var_fn468_calc_ig__pgsrecin: f64,
        var_fn468_calc_ig__phitin: f64,
        var_fn468_calc_ig__phitin_dn4: f64,
        var_fn468_calc_ig__tfacdiodein: f64,
        var_fn468_calc_ig__tfacdiodein_dn4: f64,
        var_fn468_calc_ig__type: f64,
        var_fn468_calc_ig__vgin: f64,
        var_fn468_calc_ig__vgin_dn7: f64,
        var_fn468_calc_ig__vgin_dn8: f64,
        var_fn468_calc_ig__vgsatqin: f64,
        var_fn468_calc_ig__w: f64,
        var_guard461: f64,
        var_guard467: f64,
        var_vsch: f64,
        var_vsch_dn7: f64,
        var_vsch_dn8: f64,
        var_fn468_calc_ig__expirev_slot: &mut f64,
        var_fn468_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn468_calc_ig__expirev_dn7_slot: &mut f64,
        var_fn468_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn468_calc_ig__expirevarg_slot: &mut f64,
        var_fn468_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn468_calc_ig__expirevarg_dn7_slot: &mut f64,
        var_fn468_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn468_calc_ig__frecgin_slot: &mut f64,
        var_fn468_calc_ig__frecgin_dn7_slot: &mut f64,
        var_fn468_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn468_calc_ig__iginrec_slot: &mut f64,
        var_fn468_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn468_calc_ig__iginrec_dn7_slot: &mut f64,
        var_fn468_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn468_calc_ig__igout_slot: &mut f64,
        var_fn468_calc_ig__igout_dn4_slot: &mut f64,
        var_fn468_calc_ig__igout_dn7_slot: &mut f64,
        var_fn468_calc_ig__igout_dn8_slot: &mut f64,
        var_fn468_calc_ig__isrecout_slot: &mut f64,
        var_fn468_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn468_calc_ig__return_slot: &mut f64,
        var_fn468_calc_ig__return_dn4_slot: &mut f64,
        var_fn468_calc_ig__return_dn7_slot: &mut f64,
        var_fn468_calc_ig__return_dn8_slot: &mut f64,
        var_guard473_slot: &mut f64,
        var_guard474_slot: &mut f64,
        var_guard475_slot: &mut f64,
        var_guard476_slot: &mut f64,
        var_guard477_slot: &mut f64,
        var_guard478_slot: &mut f64,
        var_idsch2_slot: &mut f64,
        var_idsch2_dn4_slot: &mut f64,
        var_idsch2_dn7_slot: &mut f64,
        var_idsch2_dn8_slot: &mut f64,
        var_igdcbd_slot: &mut f64,
        var_igdcbd_dn0_slot: &mut f64,
        var_igdcbd_dn18_slot: &mut f64,
        var_igdcbd_dn19_slot: &mut f64,
        var_igdcbd_dn2_slot: &mut f64,
        var_igdcbd_dn4_slot: &mut f64,
        var_igdcbd_dn8_slot: &mut f64,
        var_igscbd_slot: &mut f64,
        var_igscbd_dn0_slot: &mut f64,
        var_igscbd_dn18_slot: &mut f64,
        var_igscbd_dn19_slot: &mut f64,
        var_igscbd_dn2_slot: &mut f64,
        var_igscbd_dn4_slot: &mut f64,
        var_igscbd_dn8_slot: &mut f64,
        var_qsch_slot: &mut f64,
        var_qsch0_slot: &mut f64,
        var_qsch1_slot: &mut f64,
        var_qsch1_dn7_slot: &mut f64,
        var_qsch1_dn8_slot: &mut f64,
        var_qsch1c_slot: &mut f64,
        var_qsch2_slot: &mut f64,
        var_qsch2_dn7_slot: &mut f64,
        var_qsch2_dn8_slot: &mut f64,
        var_qsch2c_slot: &mut f64,
        var_qsch3_slot: &mut f64,
        var_qsch3_dn7_slot: &mut f64,
        var_qsch3_dn8_slot: &mut f64,
        var_qsch3c_slot: &mut f64,
        var_qsch4_slot: &mut f64,
        var_qsch4_dn7_slot: &mut f64,
        var_qsch4_dn8_slot: &mut f64,
        var_qsch4c_slot: &mut f64,
        var_qsch5_slot: &mut f64,
        var_qsch5_dn7_slot: &mut f64,
        var_qsch5_dn8_slot: &mut f64,
        var_qsch5c_slot: &mut f64,
        var_qsch_dn7_slot: &mut f64,
        var_qsch_dn8_slot: &mut f64,
        var_vindcbd_slot: &mut f64,
        var_vindcbd_dn0_slot: &mut f64,
        var_vindcbd_dn18_slot: &mut f64,
        var_vindcbd_dn19_slot: &mut f64,
        var_vindcbd_dn2_slot: &mut f64,
        var_vindcbd_dn8_slot: &mut f64,
        var_vinscbd_slot: &mut f64,
        var_vinscbd_dn0_slot: &mut f64,
        var_vinscbd_dn18_slot: &mut f64,
        var_vinscbd_dn19_slot: &mut f64,
        var_vinscbd_dn2_slot: &mut f64,
        var_vinscbd_dn8_slot: &mut f64,
        var_vschfc1_slot: &mut f64,
        var_vschfc1_dn7_slot: &mut f64,
        var_vschfc1_dn8_slot: &mut f64,
        var_vschfc2_slot: &mut f64,
        var_vschfc2_dn7_slot: &mut f64,
        var_vschfc2_dn8_slot: &mut f64,
        var_vschfc3_slot: &mut f64,
        var_vschfc3_dn7_slot: &mut f64,
        var_vschfc3_dn8_slot: &mut f64,
        var_vschfc4_slot: &mut f64,
        var_vschfc4_dn7_slot: &mut f64,
        var_vschfc4_dn8_slot: &mut f64,
        var_vschfc5_slot: &mut f64,
        var_vschfc5_dn7_slot: &mut f64,
        var_vschfc5_dn8_slot: &mut f64,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let nv19 = ctx.node_voltage(nodes[19]);
        let mut var_fn468_calc_ig__expirev: f64 = *var_fn468_calc_ig__expirev_slot;
        let mut var_fn468_calc_ig__expirev_dn4: f64 = *var_fn468_calc_ig__expirev_dn4_slot;
        let mut var_fn468_calc_ig__expirev_dn7: f64 = *var_fn468_calc_ig__expirev_dn7_slot;
        let mut var_fn468_calc_ig__expirev_dn8: f64 = *var_fn468_calc_ig__expirev_dn8_slot;
        let mut var_fn468_calc_ig__expirevarg: f64 = *var_fn468_calc_ig__expirevarg_slot;
        let mut var_fn468_calc_ig__expirevarg_dn4: f64 = *var_fn468_calc_ig__expirevarg_dn4_slot;
        let mut var_fn468_calc_ig__expirevarg_dn7: f64 = *var_fn468_calc_ig__expirevarg_dn7_slot;
        let mut var_fn468_calc_ig__expirevarg_dn8: f64 = *var_fn468_calc_ig__expirevarg_dn8_slot;
        let mut var_fn468_calc_ig__frecgin: f64 = *var_fn468_calc_ig__frecgin_slot;
        let mut var_fn468_calc_ig__frecgin_dn7: f64 = *var_fn468_calc_ig__frecgin_dn7_slot;
        let mut var_fn468_calc_ig__frecgin_dn8: f64 = *var_fn468_calc_ig__frecgin_dn8_slot;
        let mut var_fn468_calc_ig__iginrec: f64 = *var_fn468_calc_ig__iginrec_slot;
        let mut var_fn468_calc_ig__iginrec_dn4: f64 = *var_fn468_calc_ig__iginrec_dn4_slot;
        let mut var_fn468_calc_ig__iginrec_dn7: f64 = *var_fn468_calc_ig__iginrec_dn7_slot;
        let mut var_fn468_calc_ig__iginrec_dn8: f64 = *var_fn468_calc_ig__iginrec_dn8_slot;
        let mut var_fn468_calc_ig__igout: f64 = *var_fn468_calc_ig__igout_slot;
        let mut var_fn468_calc_ig__igout_dn4: f64 = *var_fn468_calc_ig__igout_dn4_slot;
        let mut var_fn468_calc_ig__igout_dn7: f64 = *var_fn468_calc_ig__igout_dn7_slot;
        let mut var_fn468_calc_ig__igout_dn8: f64 = *var_fn468_calc_ig__igout_dn8_slot;
        let mut var_fn468_calc_ig__isrecout: f64 = *var_fn468_calc_ig__isrecout_slot;
        let mut var_fn468_calc_ig__isrecout_dn4: f64 = *var_fn468_calc_ig__isrecout_dn4_slot;
        let mut var_fn468_calc_ig__return: f64 = *var_fn468_calc_ig__return_slot;
        let mut var_fn468_calc_ig__return_dn4: f64 = *var_fn468_calc_ig__return_dn4_slot;
        let mut var_fn468_calc_ig__return_dn7: f64 = *var_fn468_calc_ig__return_dn7_slot;
        let mut var_fn468_calc_ig__return_dn8: f64 = *var_fn468_calc_ig__return_dn8_slot;
        let mut var_guard473: f64 = *var_guard473_slot;
        let mut var_guard474: f64 = *var_guard474_slot;
        let mut var_guard475: f64 = *var_guard475_slot;
        let mut var_guard476: f64 = *var_guard476_slot;
        let mut var_guard477: f64 = *var_guard477_slot;
        let mut var_guard478: f64 = *var_guard478_slot;
        let mut var_idsch2: f64 = *var_idsch2_slot;
        let mut var_idsch2_dn4: f64 = *var_idsch2_dn4_slot;
        let mut var_idsch2_dn7: f64 = *var_idsch2_dn7_slot;
        let mut var_idsch2_dn8: f64 = *var_idsch2_dn8_slot;
        let mut var_igdcbd: f64 = *var_igdcbd_slot;
        let mut var_igdcbd_dn0: f64 = *var_igdcbd_dn0_slot;
        let mut var_igdcbd_dn18: f64 = *var_igdcbd_dn18_slot;
        let mut var_igdcbd_dn19: f64 = *var_igdcbd_dn19_slot;
        let mut var_igdcbd_dn2: f64 = *var_igdcbd_dn2_slot;
        let mut var_igdcbd_dn4: f64 = *var_igdcbd_dn4_slot;
        let mut var_igdcbd_dn8: f64 = *var_igdcbd_dn8_slot;
        let mut var_igscbd: f64 = *var_igscbd_slot;
        let mut var_igscbd_dn0: f64 = *var_igscbd_dn0_slot;
        let mut var_igscbd_dn18: f64 = *var_igscbd_dn18_slot;
        let mut var_igscbd_dn19: f64 = *var_igscbd_dn19_slot;
        let mut var_igscbd_dn2: f64 = *var_igscbd_dn2_slot;
        let mut var_igscbd_dn4: f64 = *var_igscbd_dn4_slot;
        let mut var_igscbd_dn8: f64 = *var_igscbd_dn8_slot;
        let mut var_qsch: f64 = *var_qsch_slot;
        let mut var_qsch0: f64 = *var_qsch0_slot;
        let mut var_qsch1: f64 = *var_qsch1_slot;
        let mut var_qsch1_dn7: f64 = *var_qsch1_dn7_slot;
        let mut var_qsch1_dn8: f64 = *var_qsch1_dn8_slot;
        let mut var_qsch1c: f64 = *var_qsch1c_slot;
        let mut var_qsch2: f64 = *var_qsch2_slot;
        let mut var_qsch2_dn7: f64 = *var_qsch2_dn7_slot;
        let mut var_qsch2_dn8: f64 = *var_qsch2_dn8_slot;
        let mut var_qsch2c: f64 = *var_qsch2c_slot;
        let mut var_qsch3: f64 = *var_qsch3_slot;
        let mut var_qsch3_dn7: f64 = *var_qsch3_dn7_slot;
        let mut var_qsch3_dn8: f64 = *var_qsch3_dn8_slot;
        let mut var_qsch3c: f64 = *var_qsch3c_slot;
        let mut var_qsch4: f64 = *var_qsch4_slot;
        let mut var_qsch4_dn7: f64 = *var_qsch4_dn7_slot;
        let mut var_qsch4_dn8: f64 = *var_qsch4_dn8_slot;
        let mut var_qsch4c: f64 = *var_qsch4c_slot;
        let mut var_qsch5: f64 = *var_qsch5_slot;
        let mut var_qsch5_dn7: f64 = *var_qsch5_dn7_slot;
        let mut var_qsch5_dn8: f64 = *var_qsch5_dn8_slot;
        let mut var_qsch5c: f64 = *var_qsch5c_slot;
        let mut var_qsch_dn7: f64 = *var_qsch_dn7_slot;
        let mut var_qsch_dn8: f64 = *var_qsch_dn8_slot;
        let mut var_vindcbd: f64 = *var_vindcbd_slot;
        let mut var_vindcbd_dn0: f64 = *var_vindcbd_dn0_slot;
        let mut var_vindcbd_dn18: f64 = *var_vindcbd_dn18_slot;
        let mut var_vindcbd_dn19: f64 = *var_vindcbd_dn19_slot;
        let mut var_vindcbd_dn2: f64 = *var_vindcbd_dn2_slot;
        let mut var_vindcbd_dn8: f64 = *var_vindcbd_dn8_slot;
        let mut var_vinscbd: f64 = *var_vinscbd_slot;
        let mut var_vinscbd_dn0: f64 = *var_vinscbd_dn0_slot;
        let mut var_vinscbd_dn18: f64 = *var_vinscbd_dn18_slot;
        let mut var_vinscbd_dn19: f64 = *var_vinscbd_dn19_slot;
        let mut var_vinscbd_dn2: f64 = *var_vinscbd_dn2_slot;
        let mut var_vinscbd_dn8: f64 = *var_vinscbd_dn8_slot;
        let mut var_vschfc1: f64 = *var_vschfc1_slot;
        let mut var_vschfc1_dn7: f64 = *var_vschfc1_dn7_slot;
        let mut var_vschfc1_dn8: f64 = *var_vschfc1_dn8_slot;
        let mut var_vschfc2: f64 = *var_vschfc2_slot;
        let mut var_vschfc2_dn7: f64 = *var_vschfc2_dn7_slot;
        let mut var_vschfc2_dn8: f64 = *var_vschfc2_dn8_slot;
        let mut var_vschfc3: f64 = *var_vschfc3_slot;
        let mut var_vschfc3_dn7: f64 = *var_vschfc3_dn7_slot;
        let mut var_vschfc3_dn8: f64 = *var_vschfc3_dn8_slot;
        let mut var_vschfc4: f64 = *var_vschfc4_slot;
        let mut var_vschfc4_dn7: f64 = *var_vschfc4_dn7_slot;
        let mut var_vschfc4_dn8: f64 = *var_vschfc4_dn8_slot;
        let mut var_vschfc5: f64 = *var_vschfc5_slot;
        let mut var_vschfc5_dn7: f64 = *var_vschfc5_dn7_slot;
        let mut var_vschfc5_dn8: f64 = *var_vschfc5_dn8_slot;

        let (assign43520_e42090, assign43520_e42090_d_n7, assign43520_e42090_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign43520_e42047: f64 = (-var_fn468_calc_ig__vgin);
        let (assign43520_e42080, assign43520_e42080_d_n7, assign43520_e42080_d_n8,) = {
            if (p.p52 != 0.0) {
                let assign43520_e42055: f64 = (var_fn468_calc_ig__vgin / var_fn468_calc_ig__vgsatqin);
                let assign43520_e42058: f64 = (0.001 / p.p53);
                let assign43520_e42061: f64 = (var_fn468_calc_ig__vgin / var_fn468_calc_ig__vgsatqin);
                let assign43520_e42062: f64 = (assign43520_e42058 * assign43520_e42061);
                let assign43520_e42063: f64 = (assign43520_e42062).tanh();
                let assign43520_e42064: f64 = (assign43520_e42055 * assign43520_e42063);
                (assign43520_e42064, (((var_fn468_calc_ig__vgin_dn7 / var_fn468_calc_ig__vgsatqin) * assign43520_e42063) + (assign43520_e42055 * ((assign43520_e42058 * (var_fn468_calc_ig__vgin_dn7 / var_fn468_calc_ig__vgsatqin)) / ((assign43520_e42062).cosh() * (assign43520_e42062).cosh())))), (((var_fn468_calc_ig__vgin_dn8 / var_fn468_calc_ig__vgsatqin) * assign43520_e42063) + (assign43520_e42055 * ((assign43520_e42058 * (var_fn468_calc_ig__vgin_dn8 / var_fn468_calc_ig__vgsatqin)) / ((assign43520_e42062).cosh() * (assign43520_e42062).cosh())))),)
            } else {
                let (assign43520_e42079, assign43520_e42079_d_n7, assign43520_e42079_d_n8,) = {
                    if (p.p52 == 0.0) {
                        let __rspice_inv_cse_0: f64 = 1.0 / var_fn468_calc_ig__vgsatqin;
                        let assign43520_e42070: f64 = (var_fn468_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign43520_e42073: f64 = (var_fn468_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign43520_e42074: f64 = (assign43520_e42070 * assign43520_e42073);
                        let assign43520_e42076: f64 = (assign43520_e42074 + p.p53);
                        let assign43520_e42077: f64 = (assign43520_e42076).sqrt();
                        (assign43520_e42077, ((((var_fn468_calc_ig__vgin_dn7 / var_fn468_calc_ig__vgsatqin) * assign43520_e42073) + (assign43520_e42070 * (var_fn468_calc_ig__vgin_dn7 / var_fn468_calc_ig__vgsatqin))) / (2.0 * assign43520_e42077)), ((((var_fn468_calc_ig__vgin_dn8 / var_fn468_calc_ig__vgsatqin) * assign43520_e42073) + (assign43520_e42070 * (var_fn468_calc_ig__vgin_dn8 / var_fn468_calc_ig__vgsatqin))) / (2.0 * assign43520_e42077)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign43520_e42079, assign43520_e42079_d_n7, assign43520_e42079_d_n8,)
            }
        };
        let assign43520_e42082: f64 = (assign43520_e42080).powf(var_fn468_calc_ig__betarecin);
        let assign43520_e42083: f64 = (1.0 + assign43520_e42082);
        let assign43520_e42086: f64 = (1.0 / var_fn468_calc_ig__betarecin);
        let assign43520_e42087: f64 = (assign43520_e42083).powf(assign43520_e42086);
        let assign43520_e42088: f64 = (assign43520_e42047 / assign43520_e42087);
        (assign43520_e42088, ((((-var_fn468_calc_ig__vgin_dn7) * assign43520_e42087) - (assign43520_e42047 * if 0.0 == 0.0 && ((assign43520_e42086) as f64).is_finite() && ((assign43520_e42086) as f64).fract() == 0.0 { if assign43520_e42086 == 0.0 { 0.0 } else { (assign43520_e42086 * ((assign43520_e42083).powf(assign43520_e42086 - 1.0) * if 0.0 == 0.0 && ((var_fn468_calc_ig__betarecin) as f64).is_finite() && ((var_fn468_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn468_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn468_calc_ig__betarecin * ((assign43520_e42080).powf(var_fn468_calc_ig__betarecin - 1.0) * assign43520_e42080_d_n7)) } } else { (assign43520_e42082 * (var_fn468_calc_ig__betarecin * (assign43520_e42080_d_n7 / assign43520_e42080))) })) } } else { (assign43520_e42087 * (assign43520_e42086 * (if 0.0 == 0.0 && ((var_fn468_calc_ig__betarecin) as f64).is_finite() && ((var_fn468_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn468_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn468_calc_ig__betarecin * ((assign43520_e42080).powf(var_fn468_calc_ig__betarecin - 1.0) * assign43520_e42080_d_n7)) } } else { (assign43520_e42082 * (var_fn468_calc_ig__betarecin * (assign43520_e42080_d_n7 / assign43520_e42080))) } / assign43520_e42083))) })) / (assign43520_e42087 * assign43520_e42087)), ((((-var_fn468_calc_ig__vgin_dn8) * assign43520_e42087) - (assign43520_e42047 * if 0.0 == 0.0 && ((assign43520_e42086) as f64).is_finite() && ((assign43520_e42086) as f64).fract() == 0.0 { if assign43520_e42086 == 0.0 { 0.0 } else { (assign43520_e42086 * ((assign43520_e42083).powf(assign43520_e42086 - 1.0) * if 0.0 == 0.0 && ((var_fn468_calc_ig__betarecin) as f64).is_finite() && ((var_fn468_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn468_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn468_calc_ig__betarecin * ((assign43520_e42080).powf(var_fn468_calc_ig__betarecin - 1.0) * assign43520_e42080_d_n8)) } } else { (assign43520_e42082 * (var_fn468_calc_ig__betarecin * (assign43520_e42080_d_n8 / assign43520_e42080))) })) } } else { (assign43520_e42087 * (assign43520_e42086 * (if 0.0 == 0.0 && ((var_fn468_calc_ig__betarecin) as f64).is_finite() && ((var_fn468_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn468_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn468_calc_ig__betarecin * ((assign43520_e42080).powf(var_fn468_calc_ig__betarecin - 1.0) * assign43520_e42080_d_n8)) } } else { (assign43520_e42082 * (var_fn468_calc_ig__betarecin * (assign43520_e42080_d_n8 / assign43520_e42080))) } / assign43520_e42083))) })) / (assign43520_e42087 * assign43520_e42087)),)
    } else {
        (var_fn468_calc_ig__frecgin, var_fn468_calc_ig__frecgin_dn7, var_fn468_calc_ig__frecgin_dn8,)
    }
};
        var_fn468_calc_ig__frecgin = assign43520_e42090;
        var_fn468_calc_ig__frecgin_dn7 = assign43520_e42090_d_n7;
        var_fn468_calc_ig__frecgin_dn8 = assign43520_e42090_d_n8;

        let (assign43530_e42107, assign43530_e42107_d_n4,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign43530_e42095: f64 = (-var_fn468_calc_ig__type);
        let assign43530_e42097: f64 = (assign43530_e42095 * var_fn468_calc_ig__w);
        let assign43530_e42099: f64 = (assign43530_e42097 * var_fn468_calc_ig__ngf);
        let assign43530_e42101: f64 = (assign43530_e42099 * var_fn468_calc_ig__irecin);
        let assign43530_e42103: f64 = (assign43530_e42101 * var_fn468_calc_ig__tfacdiodein);
        let assign43530_e42105: f64 = assign43530_e42103;
        (assign43530_e42105, (assign43530_e42101 * var_fn468_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn468_calc_ig__isrecout, var_fn468_calc_ig__isrecout_dn4,)
    }
};
        var_fn468_calc_ig__isrecout = assign43530_e42107;
        var_fn468_calc_ig__isrecout_dn4 = assign43530_e42107_d_n4;

        let (assign43540_e42117, assign43540_e42117_d_n4, assign43540_e42117_d_n7, assign43540_e42117_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign43540_e42113: f64 = (var_fn468_calc_ig__pgsrecin / var_fn468_calc_ig__phitin);
        let assign43540_e42115: f64 = (assign43540_e42113 * var_fn468_calc_ig__frecgin);
        (assign43540_e42115, ((-((var_fn468_calc_ig__pgsrecin * var_fn468_calc_ig__phitin_dn4) / (var_fn468_calc_ig__phitin * var_fn468_calc_ig__phitin))) * var_fn468_calc_ig__frecgin), (assign43540_e42113 * var_fn468_calc_ig__frecgin_dn7), (assign43540_e42113 * var_fn468_calc_ig__frecgin_dn8),)
    } else {
        (var_fn468_calc_ig__expirevarg, var_fn468_calc_ig__expirevarg_dn4, var_fn468_calc_ig__expirevarg_dn7, var_fn468_calc_ig__expirevarg_dn8,)
    }
};
        var_fn468_calc_ig__expirevarg = assign43540_e42117;
        var_fn468_calc_ig__expirevarg_dn4 = assign43540_e42117_d_n4;
        var_fn468_calc_ig__expirevarg_dn7 = assign43540_e42117_d_n7;
        var_fn468_calc_ig__expirevarg_dn8 = assign43540_e42117_d_n8;

        let (assign43550_e42161, assign43550_e42161_d_n4, assign43550_e42161_d_n7, assign43550_e42161_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign43550_e42127: f64 = (-50.0);
        let (assign43550_e42159, assign43550_e42159_d_n4, assign43550_e42159_d_n7, assign43550_e42159_d_n8,) = {
            if ((!(var_fn468_calc_ig__expirevarg > 50.0)) && (!(var_fn468_calc_ig__expirevarg < assign43550_e42127))) {
                let assign43550_e42132: f64 = (var_fn468_calc_ig__expirevarg).exp();
                (assign43550_e42132, (assign43550_e42132 * var_fn468_calc_ig__expirevarg_dn4), (assign43550_e42132 * var_fn468_calc_ig__expirevarg_dn7), (assign43550_e42132 * var_fn468_calc_ig__expirevarg_dn8),)
            } else {
                let assign43550_e42139: f64 = (-50.0);
                let (assign43550_e42158, assign43550_e42158_d_n4, assign43550_e42158_d_n7, assign43550_e42158_d_n8,) = {
                    if ((!(var_fn468_calc_ig__expirevarg > 50.0)) && (var_fn468_calc_ig__expirevarg < assign43550_e42139)) {
                        let assign43550_e42143: f64 = (-50.0);
                        let assign43550_e42144: f64 = (assign43550_e42143).exp();
                        (assign43550_e42144, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign43550_e42157, assign43550_e42157_d_n4, assign43550_e42157_d_n7, assign43550_e42157_d_n8,) = {
                            if (var_fn468_calc_ig__expirevarg > 50.0) {
                                let assign43550_e42149: f64 = (50.0_f64).exp();
                                let assign43550_e42153: f64 = (var_fn468_calc_ig__expirevarg - 50.0);
                                let assign43550_e42154: f64 = (1.0 + assign43550_e42153);
                                let assign43550_e42155: f64 = (assign43550_e42149 * assign43550_e42154);
                                (assign43550_e42155, (assign43550_e42149 * var_fn468_calc_ig__expirevarg_dn4), (assign43550_e42149 * var_fn468_calc_ig__expirevarg_dn7), (assign43550_e42149 * var_fn468_calc_ig__expirevarg_dn8),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign43550_e42157, assign43550_e42157_d_n4, assign43550_e42157_d_n7, assign43550_e42157_d_n8,)
                    }
                };
                (assign43550_e42158, assign43550_e42158_d_n4, assign43550_e42158_d_n7, assign43550_e42158_d_n8,)
            }
        };
        (assign43550_e42159, assign43550_e42159_d_n4, assign43550_e42159_d_n7, assign43550_e42159_d_n8,)
    } else {
        (var_fn468_calc_ig__expirev, var_fn468_calc_ig__expirev_dn4, var_fn468_calc_ig__expirev_dn7, var_fn468_calc_ig__expirev_dn8,)
    }
};
        var_fn468_calc_ig__expirev = assign43550_e42161;
        var_fn468_calc_ig__expirev_dn4 = assign43550_e42161_d_n4;
        var_fn468_calc_ig__expirev_dn7 = assign43550_e42161_d_n7;
        var_fn468_calc_ig__expirev_dn8 = assign43550_e42161_d_n8;

        let (assign43560_e42171, assign43560_e42171_d_n4, assign43560_e42171_d_n7, assign43560_e42171_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign43560_e42168: f64 = (var_fn468_calc_ig__expirev - 1.0);
        let assign43560_e42169: f64 = (var_fn468_calc_ig__isrecout * assign43560_e42168);
        (assign43560_e42169, ((var_fn468_calc_ig__isrecout_dn4 * assign43560_e42168) + (var_fn468_calc_ig__isrecout * var_fn468_calc_ig__expirev_dn4)), (var_fn468_calc_ig__isrecout * var_fn468_calc_ig__expirev_dn7), (var_fn468_calc_ig__isrecout * var_fn468_calc_ig__expirev_dn8),)
    } else {
        (var_fn468_calc_ig__iginrec, var_fn468_calc_ig__iginrec_dn4, var_fn468_calc_ig__iginrec_dn7, var_fn468_calc_ig__iginrec_dn8,)
    }
};
        var_fn468_calc_ig__iginrec = assign43560_e42171;
        var_fn468_calc_ig__iginrec_dn4 = assign43560_e42171_d_n4;
        var_fn468_calc_ig__iginrec_dn7 = assign43560_e42171_d_n7;
        var_fn468_calc_ig__iginrec_dn8 = assign43560_e42171_d_n8;

        let (assign43570_e42179, assign43570_e42179_d_n4, assign43570_e42179_d_n7, assign43570_e42179_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let assign43570_e42177: f64 = (var_fn468_calc_ig__igindiode + var_fn468_calc_ig__iginrec);
        (assign43570_e42177, (var_fn468_calc_ig__igindiode_dn4 + var_fn468_calc_ig__iginrec_dn4), (var_fn468_calc_ig__igindiode_dn7 + var_fn468_calc_ig__iginrec_dn7), (var_fn468_calc_ig__igindiode_dn8 + var_fn468_calc_ig__iginrec_dn8),)
    } else {
        (var_fn468_calc_ig__igout, var_fn468_calc_ig__igout_dn4, var_fn468_calc_ig__igout_dn7, var_fn468_calc_ig__igout_dn8,)
    }
};
        var_fn468_calc_ig__igout = assign43570_e42179;
        var_fn468_calc_ig__igout_dn4 = assign43570_e42179_d_n4;
        var_fn468_calc_ig__igout_dn7 = assign43570_e42179_d_n7;
        var_fn468_calc_ig__igout_dn8 = assign43570_e42179_d_n8;

        let (assign43580_e42185, assign43580_e42185_d_n4, assign43580_e42185_d_n7, assign43580_e42185_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (var_fn468_calc_ig__igout, var_fn468_calc_ig__igout_dn4, var_fn468_calc_ig__igout_dn7, var_fn468_calc_ig__igout_dn8,)
    } else {
        (var_fn468_calc_ig__return, var_fn468_calc_ig__return_dn4, var_fn468_calc_ig__return_dn7, var_fn468_calc_ig__return_dn8,)
    }
};
        var_fn468_calc_ig__return = assign43580_e42185;
        var_fn468_calc_ig__return_dn4 = assign43580_e42185_d_n4;
        var_fn468_calc_ig__return_dn7 = assign43580_e42185_d_n7;
        var_fn468_calc_ig__return_dn8 = assign43580_e42185_d_n8;

        let (assign43610_e42203, assign43610_e42203_d_n4, assign43610_e42203_d_n7, assign43610_e42203_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        (var_fn468_calc_ig__return, var_fn468_calc_ig__return_dn4, var_fn468_calc_ig__return_dn7, var_fn468_calc_ig__return_dn8,)
    } else {
        (var_idsch2, var_idsch2_dn4, var_idsch2_dn7, var_idsch2_dn8,)
    }
};
        var_idsch2 = assign43610_e42203;
        var_idsch2_dn4 = assign43610_e42203_d_n4;
        var_idsch2_dn7 = assign43610_e42203_d_n7;
        var_idsch2_dn8 = assign43610_e42203_d_n8;

        let assign43620_e42207: f64 = (p.p308 * p.p306);
        let assign43620_e42208: f64 = if var_vsch <= assign43620_e42207 { 1.0 } else { 0.0 };
        var_guard473 = assign43620_e42208;

        let (assign43630_e42237, assign43630_e42237_d_n7, assign43630_e42237_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard473 != 0.0)) {
        let assign43630_e42214: f64 = (p.p6 * 2.0);
        let assign43630_e42216: f64 = (assign43630_e42214 * p.p307);
        let assign43630_e42218: f64 = (assign43630_e42216 * p.p0);
        let assign43630_e42221: f64 = (1.0 - p.p311);
        let assign43630_e42222: f64 = (assign43630_e42218 * assign43630_e42221);
        let assign43630_e42224: f64 = (assign43630_e42222 * p.p2);
        let assign43630_e42226: f64 = (assign43630_e42224 * p.p306);
        let assign43630_e42231: f64 = (var_vsch / p.p306);
        let assign43630_e42232: f64 = (1.0 - assign43630_e42231);
        let assign43630_e42233: f64 = (assign43630_e42232).sqrt();
        let assign43630_e42234: f64 = (1.0 - assign43630_e42233);
        let assign43630_e42235: f64 = (assign43630_e42226 * assign43630_e42234);
        (assign43630_e42235, (assign43630_e42226 * (-((-(var_vsch_dn7 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn8 / p.p306)) / (2.0 * assign43630_e42233)))),)
    } else {
        (var_qsch, var_qsch_dn7, var_qsch_dn8,)
    }
};
        var_qsch = assign43630_e42237;
        var_qsch_dn7 = assign43630_e42237_d_n7;
        var_qsch_dn8 = assign43630_e42237_d_n8;

        let (assign43640_e42249,) = {
    if ((var_guard461 != 0.0) && (var_guard473 == 0.0)) {
        let assign43640_e42245: f64 = (1.0 - p.p308);
        let assign43640_e42246: f64 = (assign43640_e42245).sqrt();
        let assign43640_e42247: f64 = (1.0 - assign43640_e42246);
        (assign43640_e42247,)
    } else {
        (var_qsch0,)
    }
};
        var_qsch0 = assign43640_e42249;

        let assign43650_e42252: f64 = if p.p309 >= 1.0 { 1.0 } else { 0.0 };
        var_guard474 = assign43650_e42252;

        let (assign43660_e42270,) = {
    if (((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) {
        let assign43660_e42262: f64 = (2.0 * p.p306);
        let assign43660_e42265: f64 = (1.0 - p.p308);
        let assign43660_e42266: f64 = (assign43660_e42265).sqrt();
        let assign43660_e42267: f64 = (assign43660_e42262 * assign43660_e42266);
        let assign43660_e42268: f64 = (1.0 / assign43660_e42267);
        (assign43660_e42268,)
    } else {
        (var_qsch1c,)
    }
};
        var_qsch1c = assign43660_e42270;

        let (assign43670_e42283, assign43670_e42283_d_n7, assign43670_e42283_d_n8,) = {
    if (((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) {
        let assign43670_e42280: f64 = (p.p308 * p.p306);
        let assign43670_e42281: f64 = (var_vsch - assign43670_e42280);
        (assign43670_e42281, var_vsch_dn7, var_vsch_dn8,)
    } else {
        (var_vschfc1, var_vschfc1_dn7, var_vschfc1_dn8,)
    }
};
        var_vschfc1 = assign43670_e42283;
        var_vschfc1_dn7 = assign43670_e42283_d_n7;
        var_vschfc1_dn8 = assign43670_e42283_d_n8;

        let (assign43680_e42294, assign43680_e42294_d_n7, assign43680_e42294_d_n8,) = {
    if (((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) {
        let assign43680_e42292: f64 = (var_qsch1c * var_vschfc1);
        (assign43680_e42292, (var_qsch1c * var_vschfc1_dn7), (var_qsch1c * var_vschfc1_dn8),)
    } else {
        (var_qsch1, var_qsch1_dn7, var_qsch1_dn8,)
    }
};
        var_qsch1 = assign43680_e42294;
        var_qsch1_dn7 = assign43680_e42294_d_n7;
        var_qsch1_dn8 = assign43680_e42294_d_n8;

        let assign43690_e42297: f64 = if p.p309 >= 2.0 { 1.0 } else { 0.0 };
        var_guard475 = assign43690_e42297;

        let (assign43700_e42316,) = {
    if ((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) {
        let assign43700_e42309: f64 = (4.0 * p.p306);
        let assign43700_e42312: f64 = (1.0 - p.p308);
        let assign43700_e42313: f64 = (assign43700_e42309 * assign43700_e42312);
        let assign43700_e42314: f64 = (var_qsch1c / assign43700_e42313);
        (assign43700_e42314,)
    } else {
        (var_qsch2c,)
    }
};
        var_qsch2c = assign43700_e42316;

        let (assign43710_e42329, assign43710_e42329_d_n7, assign43710_e42329_d_n8,) = {
    if ((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) {
        let assign43710_e42327: f64 = (var_vschfc1 * var_vschfc1);
        (assign43710_e42327, ((var_vschfc1_dn7 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn7)), ((var_vschfc1_dn8 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn8)),)
    } else {
        (var_vschfc2, var_vschfc2_dn7, var_vschfc2_dn8,)
    }
};
        var_vschfc2 = assign43710_e42329;
        var_vschfc2_dn7 = assign43710_e42329_d_n7;
        var_vschfc2_dn8 = assign43710_e42329_d_n8;

        let (assign43720_e42342, assign43720_e42342_d_n7, assign43720_e42342_d_n8,) = {
    if ((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) {
        let assign43720_e42340: f64 = (var_qsch2c * var_vschfc2);
        (assign43720_e42340, (var_qsch2c * var_vschfc2_dn7), (var_qsch2c * var_vschfc2_dn8),)
    } else {
        (var_qsch2, var_qsch2_dn7, var_qsch2_dn8,)
    }
};
        var_qsch2 = assign43720_e42342;
        var_qsch2_dn7 = assign43720_e42342_d_n7;
        var_qsch2_dn8 = assign43720_e42342_d_n8;

        let assign43730_e42345: f64 = if p.p309 >= 3.0 { 1.0 } else { 0.0 };
        var_guard476 = assign43730_e42345;

        let (assign43740_e42366,) = {
    if (((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) {
        let assign43740_e42359: f64 = (2.0 * p.p306);
        let assign43740_e42362: f64 = (1.0 - p.p308);
        let assign43740_e42363: f64 = (assign43740_e42359 * assign43740_e42362);
        let assign43740_e42364: f64 = (var_qsch2c / assign43740_e42363);
        (assign43740_e42364,)
    } else {
        (var_qsch3c,)
    }
};
        var_qsch3c = assign43740_e42366;

        let (assign43750_e42381, assign43750_e42381_d_n7, assign43750_e42381_d_n8,) = {
    if (((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) {
        let assign43750_e42379: f64 = (var_vschfc2 * var_vschfc1);
        (assign43750_e42379, ((var_vschfc2_dn7 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn7)), ((var_vschfc2_dn8 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn8)),)
    } else {
        (var_vschfc3, var_vschfc3_dn7, var_vschfc3_dn8,)
    }
};
        var_vschfc3 = assign43750_e42381;
        var_vschfc3_dn7 = assign43750_e42381_d_n7;
        var_vschfc3_dn8 = assign43750_e42381_d_n8;

        let (assign43760_e42396, assign43760_e42396_d_n7, assign43760_e42396_d_n8,) = {
    if (((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) {
        let assign43760_e42394: f64 = (var_qsch3c * var_vschfc3);
        (assign43760_e42394, (var_qsch3c * var_vschfc3_dn7), (var_qsch3c * var_vschfc3_dn8),)
    } else {
        (var_qsch3, var_qsch3_dn7, var_qsch3_dn8,)
    }
};
        var_qsch3 = assign43760_e42396;
        var_qsch3_dn7 = assign43760_e42396_d_n7;
        var_qsch3_dn8 = assign43760_e42396_d_n8;

        let assign43770_e42399: f64 = if p.p309 >= 4.0 { 1.0 } else { 0.0 };
        var_guard477 = assign43770_e42399;

        let (assign43780_e42424,) = {
    if ((((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) && (var_guard477 != 0.0)) {
        let assign43780_e42414: f64 = (5.0 * var_qsch3c);
        let assign43780_e42417: f64 = (8.0 * p.p306);
        let assign43780_e42420: f64 = (1.0 - p.p308);
        let assign43780_e42421: f64 = (assign43780_e42417 * assign43780_e42420);
        let assign43780_e42422: f64 = (assign43780_e42414 / assign43780_e42421);
        (assign43780_e42422,)
    } else {
        (var_qsch4c,)
    }
};
        var_qsch4c = assign43780_e42424;

        let (assign43790_e42441, assign43790_e42441_d_n7, assign43790_e42441_d_n8,) = {
    if ((((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) && (var_guard477 != 0.0)) {
        let assign43790_e42439: f64 = (var_vschfc3 * var_vschfc1);
        (assign43790_e42439, ((var_vschfc3_dn7 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn7)), ((var_vschfc3_dn8 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn8)),)
    } else {
        (var_vschfc4, var_vschfc4_dn7, var_vschfc4_dn8,)
    }
};
        var_vschfc4 = assign43790_e42441;
        var_vschfc4_dn7 = assign43790_e42441_d_n7;
        var_vschfc4_dn8 = assign43790_e42441_d_n8;

        let (assign43800_e42458, assign43800_e42458_d_n7, assign43800_e42458_d_n8,) = {
    if ((((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) && (var_guard477 != 0.0)) {
        let assign43800_e42456: f64 = (var_qsch4c * var_vschfc4);
        (assign43800_e42456, (var_qsch4c * var_vschfc4_dn7), (var_qsch4c * var_vschfc4_dn8),)
    } else {
        (var_qsch4, var_qsch4_dn7, var_qsch4_dn8,)
    }
};
        var_qsch4 = assign43800_e42458;
        var_qsch4_dn7 = assign43800_e42458_d_n7;
        var_qsch4_dn8 = assign43800_e42458_d_n8;

        let assign43810_e42461: f64 = if p.p309 >= 5.0 { 1.0 } else { 0.0 };
        var_guard478 = assign43810_e42461;

        let (assign43820_e42488,) = {
    if (((((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) && (var_guard477 != 0.0)) && (var_guard478 != 0.0)) {
        let assign43820_e42478: f64 = (7.0 * var_qsch4c);
        let assign43820_e42481: f64 = (10.0 * p.p306);
        let assign43820_e42484: f64 = (1.0 - p.p308);
        let assign43820_e42485: f64 = (assign43820_e42481 * assign43820_e42484);
        let assign43820_e42486: f64 = (assign43820_e42478 / assign43820_e42485);
        (assign43820_e42486,)
    } else {
        (var_qsch5c,)
    }
};
        var_qsch5c = assign43820_e42488;

        let (assign43830_e42507, assign43830_e42507_d_n7, assign43830_e42507_d_n8,) = {
    if (((((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) && (var_guard477 != 0.0)) && (var_guard478 != 0.0)) {
        let assign43830_e42505: f64 = (var_vschfc4 * var_vschfc1);
        (assign43830_e42505, ((var_vschfc4_dn7 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn7)), ((var_vschfc4_dn8 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn8)),)
    } else {
        (var_vschfc5, var_vschfc5_dn7, var_vschfc5_dn8,)
    }
};
        var_vschfc5 = assign43830_e42507;
        var_vschfc5_dn7 = assign43830_e42507_d_n7;
        var_vschfc5_dn8 = assign43830_e42507_d_n8;

        let (assign43840_e42526, assign43840_e42526_d_n7, assign43840_e42526_d_n8,) = {
    if (((((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) && (var_guard477 != 0.0)) && (var_guard478 != 0.0)) {
        let assign43840_e42524: f64 = (var_qsch5c * var_vschfc5);
        (assign43840_e42524, (var_qsch5c * var_vschfc5_dn7), (var_qsch5c * var_vschfc5_dn8),)
    } else {
        (var_qsch5, var_qsch5_dn7, var_qsch5_dn8,)
    }
};
        var_qsch5 = assign43840_e42526;
        var_qsch5_dn7 = assign43840_e42526_d_n7;
        var_qsch5_dn8 = assign43840_e42526_d_n8;

        let (assign43850_e42544,) = {
    if (((((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) && (var_guard477 != 0.0)) && (var_guard478 == 0.0)) {
        (0.0,)
    } else {
        (var_qsch5c,)
    }
};
        var_qsch5c = assign43850_e42544;

        let (assign43860_e42560,) = {
    if ((((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) && (var_guard477 == 0.0)) {
        (0.0,)
    } else {
        (var_qsch4c,)
    }
};
        var_qsch4c = assign43860_e42560;

        let (assign43870_e42574,) = {
    if (((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 == 0.0)) {
        (0.0,)
    } else {
        (var_qsch3c,)
    }
};
        var_qsch3c = assign43870_e42574;

        let (assign43880_e42586,) = {
    if ((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 == 0.0)) {
        (0.0,)
    } else {
        (var_qsch2c,)
    }
};
        var_qsch2c = assign43880_e42586;

        let (assign43890_e42596,) = {
    if (((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 == 0.0)) {
        (0.0,)
    } else {
        (var_qsch1c,)
    }
};
        var_qsch1c = assign43890_e42596;

        let (assign43900_e42629, assign43900_e42629_d_n7, assign43900_e42629_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard473 == 0.0)) {
        let assign43900_e42603: f64 = (p.p6 * 2.0);
        let assign43900_e42605: f64 = (assign43900_e42603 * p.p307);
        let assign43900_e42607: f64 = (assign43900_e42605 * p.p0);
        let assign43900_e42610: f64 = (1.0 - p.p311);
        let assign43900_e42611: f64 = (assign43900_e42607 * assign43900_e42610);
        let assign43900_e42613: f64 = (assign43900_e42611 * p.p2);
        let assign43900_e42615: f64 = (assign43900_e42613 * p.p306);
        let assign43900_e42618: f64 = (var_qsch0 + var_qsch1);
        let assign43900_e42620: f64 = (assign43900_e42618 + var_qsch2);
        let assign43900_e42622: f64 = (assign43900_e42620 + var_qsch3);
        let assign43900_e42624: f64 = (assign43900_e42622 + var_qsch4);
        let assign43900_e42626: f64 = (assign43900_e42624 + var_qsch5);
        let assign43900_e42627: f64 = (assign43900_e42615 * assign43900_e42626);
        (assign43900_e42627, (assign43900_e42615 * ((((var_qsch1_dn7 + var_qsch2_dn7) + var_qsch3_dn7) + var_qsch4_dn7) + var_qsch5_dn7)), (assign43900_e42615 * ((((var_qsch1_dn8 + var_qsch2_dn8) + var_qsch3_dn8) + var_qsch4_dn8) + var_qsch5_dn8)),)
    } else {
        (var_qsch, var_qsch_dn7, var_qsch_dn8,)
    }
};
        var_qsch = assign43900_e42629;
        var_qsch_dn7 = assign43900_e42629_d_n7;
        var_qsch_dn8 = assign43900_e42629_d_n8;

        var_igscbd = 0.0;
        var_igscbd_dn0 = 0.0;
        var_igscbd_dn2 = 0.0;
        var_igscbd_dn4 = 0.0;
        var_igscbd_dn8 = 0.0;
        var_igscbd_dn18 = 0.0;
        var_igscbd_dn19 = 0.0;

        var_igdcbd = 0.0;
        var_igdcbd_dn0 = 0.0;
        var_igdcbd_dn2 = 0.0;
        var_igdcbd_dn4 = 0.0;
        var_igdcbd_dn8 = 0.0;
        var_igdcbd_dn18 = 0.0;
        var_igdcbd_dn19 = 0.0;

        let assign43990_e42658: f64 = ((nv19 - nv18) + (nv19 - nv8));
        let assign43990_e42659: f64 = (p.p6 * assign43990_e42658);
        var_vinscbd = assign43990_e42659;
        var_vinscbd_dn0 = 0.0;
        var_vinscbd_dn2 = 0.0;
        var_vinscbd_dn8 = (-p.p6);
        var_vinscbd_dn18 = (-p.p6);
        var_vinscbd_dn19 = (p.p6 * (1.0 + 1.0));

        let assign44000_e42663: f64 = ((nv18 - nv19) + (nv18 - nv8));
        let assign44000_e42664: f64 = (p.p6 * assign44000_e42663);
        var_vindcbd = assign44000_e42664;
        var_vindcbd_dn0 = 0.0;
        var_vindcbd_dn2 = 0.0;
        var_vindcbd_dn8 = (-p.p6);
        var_vindcbd_dn18 = (p.p6 * (1.0 + 1.0));
        var_vindcbd_dn19 = (-p.p6);

        *var_fn468_calc_ig__expirev_slot = var_fn468_calc_ig__expirev;
        *var_fn468_calc_ig__expirev_dn4_slot = var_fn468_calc_ig__expirev_dn4;
        *var_fn468_calc_ig__expirev_dn7_slot = var_fn468_calc_ig__expirev_dn7;
        *var_fn468_calc_ig__expirev_dn8_slot = var_fn468_calc_ig__expirev_dn8;
        *var_fn468_calc_ig__expirevarg_slot = var_fn468_calc_ig__expirevarg;
        *var_fn468_calc_ig__expirevarg_dn4_slot = var_fn468_calc_ig__expirevarg_dn4;
        *var_fn468_calc_ig__expirevarg_dn7_slot = var_fn468_calc_ig__expirevarg_dn7;
        *var_fn468_calc_ig__expirevarg_dn8_slot = var_fn468_calc_ig__expirevarg_dn8;
        *var_fn468_calc_ig__frecgin_slot = var_fn468_calc_ig__frecgin;
        *var_fn468_calc_ig__frecgin_dn7_slot = var_fn468_calc_ig__frecgin_dn7;
        *var_fn468_calc_ig__frecgin_dn8_slot = var_fn468_calc_ig__frecgin_dn8;
        *var_fn468_calc_ig__iginrec_slot = var_fn468_calc_ig__iginrec;
        *var_fn468_calc_ig__iginrec_dn4_slot = var_fn468_calc_ig__iginrec_dn4;
        *var_fn468_calc_ig__iginrec_dn7_slot = var_fn468_calc_ig__iginrec_dn7;
        *var_fn468_calc_ig__iginrec_dn8_slot = var_fn468_calc_ig__iginrec_dn8;
        *var_fn468_calc_ig__igout_slot = var_fn468_calc_ig__igout;
        *var_fn468_calc_ig__igout_dn4_slot = var_fn468_calc_ig__igout_dn4;
        *var_fn468_calc_ig__igout_dn7_slot = var_fn468_calc_ig__igout_dn7;
        *var_fn468_calc_ig__igout_dn8_slot = var_fn468_calc_ig__igout_dn8;
        *var_fn468_calc_ig__isrecout_slot = var_fn468_calc_ig__isrecout;
        *var_fn468_calc_ig__isrecout_dn4_slot = var_fn468_calc_ig__isrecout_dn4;
        *var_fn468_calc_ig__return_slot = var_fn468_calc_ig__return;
        *var_fn468_calc_ig__return_dn4_slot = var_fn468_calc_ig__return_dn4;
        *var_fn468_calc_ig__return_dn7_slot = var_fn468_calc_ig__return_dn7;
        *var_fn468_calc_ig__return_dn8_slot = var_fn468_calc_ig__return_dn8;
        *var_guard473_slot = var_guard473;
        *var_guard474_slot = var_guard474;
        *var_guard475_slot = var_guard475;
        *var_guard476_slot = var_guard476;
        *var_guard477_slot = var_guard477;
        *var_guard478_slot = var_guard478;
        *var_idsch2_slot = var_idsch2;
        *var_idsch2_dn4_slot = var_idsch2_dn4;
        *var_idsch2_dn7_slot = var_idsch2_dn7;
        *var_idsch2_dn8_slot = var_idsch2_dn8;
        *var_igdcbd_slot = var_igdcbd;
        *var_igdcbd_dn0_slot = var_igdcbd_dn0;
        *var_igdcbd_dn18_slot = var_igdcbd_dn18;
        *var_igdcbd_dn19_slot = var_igdcbd_dn19;
        *var_igdcbd_dn2_slot = var_igdcbd_dn2;
        *var_igdcbd_dn4_slot = var_igdcbd_dn4;
        *var_igdcbd_dn8_slot = var_igdcbd_dn8;
        *var_igscbd_slot = var_igscbd;
        *var_igscbd_dn0_slot = var_igscbd_dn0;
        *var_igscbd_dn18_slot = var_igscbd_dn18;
        *var_igscbd_dn19_slot = var_igscbd_dn19;
        *var_igscbd_dn2_slot = var_igscbd_dn2;
        *var_igscbd_dn4_slot = var_igscbd_dn4;
        *var_igscbd_dn8_slot = var_igscbd_dn8;
        *var_qsch_slot = var_qsch;
        *var_qsch0_slot = var_qsch0;
        *var_qsch1_slot = var_qsch1;
        *var_qsch1_dn7_slot = var_qsch1_dn7;
        *var_qsch1_dn8_slot = var_qsch1_dn8;
        *var_qsch1c_slot = var_qsch1c;
        *var_qsch2_slot = var_qsch2;
        *var_qsch2_dn7_slot = var_qsch2_dn7;
        *var_qsch2_dn8_slot = var_qsch2_dn8;
        *var_qsch2c_slot = var_qsch2c;
        *var_qsch3_slot = var_qsch3;
        *var_qsch3_dn7_slot = var_qsch3_dn7;
        *var_qsch3_dn8_slot = var_qsch3_dn8;
        *var_qsch3c_slot = var_qsch3c;
        *var_qsch4_slot = var_qsch4;
        *var_qsch4_dn7_slot = var_qsch4_dn7;
        *var_qsch4_dn8_slot = var_qsch4_dn8;
        *var_qsch4c_slot = var_qsch4c;
        *var_qsch5_slot = var_qsch5;
        *var_qsch5_dn7_slot = var_qsch5_dn7;
        *var_qsch5_dn8_slot = var_qsch5_dn8;
        *var_qsch5c_slot = var_qsch5c;
        *var_qsch_dn7_slot = var_qsch_dn7;
        *var_qsch_dn8_slot = var_qsch_dn8;
        *var_vindcbd_slot = var_vindcbd;
        *var_vindcbd_dn0_slot = var_vindcbd_dn0;
        *var_vindcbd_dn18_slot = var_vindcbd_dn18;
        *var_vindcbd_dn19_slot = var_vindcbd_dn19;
        *var_vindcbd_dn2_slot = var_vindcbd_dn2;
        *var_vindcbd_dn8_slot = var_vindcbd_dn8;
        *var_vinscbd_slot = var_vinscbd;
        *var_vinscbd_dn0_slot = var_vinscbd_dn0;
        *var_vinscbd_dn18_slot = var_vinscbd_dn18;
        *var_vinscbd_dn19_slot = var_vinscbd_dn19;
        *var_vinscbd_dn2_slot = var_vinscbd_dn2;
        *var_vinscbd_dn8_slot = var_vinscbd_dn8;
        *var_vschfc1_slot = var_vschfc1;
        *var_vschfc1_dn7_slot = var_vschfc1_dn7;
        *var_vschfc1_dn8_slot = var_vschfc1_dn8;
        *var_vschfc2_slot = var_vschfc2;
        *var_vschfc2_dn7_slot = var_vschfc2_dn7;
        *var_vschfc2_dn8_slot = var_vschfc2_dn8;
        *var_vschfc3_slot = var_vschfc3;
        *var_vschfc3_dn7_slot = var_vschfc3_dn7;
        *var_vschfc3_dn8_slot = var_vschfc3_dn8;
        *var_vschfc4_slot = var_vschfc4;
        *var_vschfc4_dn7_slot = var_vschfc4_dn7;
        *var_vschfc4_dn8_slot = var_vschfc4_dn8;
        *var_vschfc5_slot = var_vschfc5;
        *var_vschfc5_dn7_slot = var_vschfc5_dn7;
        *var_vschfc5_dn8_slot = var_vschfc5_dn8;
    }

    pub(super) fn stamp_transient_block_110(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_phit: f64,
        var_phit_dn4: f64,
        var_tfacdiode: f64,
        var_tfacdiode_dn4: f64,
        var_fn482_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn482_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn482_calc_ig__alphagin_slot: &mut f64,
        var_fn482_calc_ig__betarecin_slot: &mut f64,
        var_fn482_calc_ig__expbd1_slot: &mut f64,
        var_fn482_calc_ig__expbd1_dn0_slot: &mut f64,
        var_fn482_calc_ig__expbd1_dn18_slot: &mut f64,
        var_fn482_calc_ig__expbd1_dn19_slot: &mut f64,
        var_fn482_calc_ig__expbd1_dn2_slot: &mut f64,
        var_fn482_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn482_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn482_calc_ig__expbd2_slot: &mut f64,
        var_fn482_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn482_calc_ig__expbdarg1_slot: &mut f64,
        var_fn482_calc_ig__expbdarg1_dn0_slot: &mut f64,
        var_fn482_calc_ig__expbdarg1_dn18_slot: &mut f64,
        var_fn482_calc_ig__expbdarg1_dn19_slot: &mut f64,
        var_fn482_calc_ig__expbdarg1_dn2_slot: &mut f64,
        var_fn482_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn482_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn482_calc_ig__expbdarg2_slot: &mut f64,
        var_fn482_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn482_calc_ig__expffvarg_slot: &mut f64,
        var_fn482_calc_ig__expffvarg_dn0_slot: &mut f64,
        var_fn482_calc_ig__expffvarg_dn18_slot: &mut f64,
        var_fn482_calc_ig__expffvarg_dn19_slot: &mut f64,
        var_fn482_calc_ig__expffvarg_dn2_slot: &mut f64,
        var_fn482_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn482_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn482_calc_ig__expifor_slot: &mut f64,
        var_fn482_calc_ig__expifor_dn0_slot: &mut f64,
        var_fn482_calc_ig__expifor_dn18_slot: &mut f64,
        var_fn482_calc_ig__expifor_dn19_slot: &mut f64,
        var_fn482_calc_ig__expifor_dn2_slot: &mut f64,
        var_fn482_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn482_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_dn0_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_dn18_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_dn19_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_dn2_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn482_calc_ig__expirev_slot: &mut f64,
        var_fn482_calc_ig__expirev_dn0_slot: &mut f64,
        var_fn482_calc_ig__expirev_dn18_slot: &mut f64,
        var_fn482_calc_ig__expirev_dn19_slot: &mut f64,
        var_fn482_calc_ig__expirev_dn2_slot: &mut f64,
        var_fn482_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn482_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn482_calc_ig__expirevarg_slot: &mut f64,
        var_fn482_calc_ig__expirevarg_dn0_slot: &mut f64,
        var_fn482_calc_ig__expirevarg_dn18_slot: &mut f64,
        var_fn482_calc_ig__expirevarg_dn19_slot: &mut f64,
        var_fn482_calc_ig__expirevarg_dn2_slot: &mut f64,
        var_fn482_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn482_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn482_calc_ig__expphib_slot: &mut f64,
        var_fn482_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn482_calc_ig__ffvgin_slot: &mut f64,
        var_fn482_calc_ig__ffvgin_dn0_slot: &mut f64,
        var_fn482_calc_ig__ffvgin_dn18_slot: &mut f64,
        var_fn482_calc_ig__ffvgin_dn19_slot: &mut f64,
        var_fn482_calc_ig__ffvgin_dn2_slot: &mut f64,
        var_fn482_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn482_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn482_calc_ig__fracin_slot: &mut f64,
        var_fn482_calc_ig__frecgin_slot: &mut f64,
        var_fn482_calc_ig__frecgin_dn0_slot: &mut f64,
        var_fn482_calc_ig__frecgin_dn18_slot: &mut f64,
        var_fn482_calc_ig__frecgin_dn19_slot: &mut f64,
        var_fn482_calc_ig__frecgin_dn2_slot: &mut f64,
        var_fn482_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn482_calc_ig__iginbd_slot: &mut f64,
        var_fn482_calc_ig__iginbd_dn0_slot: &mut f64,
        var_fn482_calc_ig__iginbd_dn18_slot: &mut f64,
        var_fn482_calc_ig__iginbd_dn19_slot: &mut f64,
        var_fn482_calc_ig__iginbd_dn2_slot: &mut f64,
        var_fn482_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn482_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn482_calc_ig__igindiode_slot: &mut f64,
        var_fn482_calc_ig__igindiode_dn0_slot: &mut f64,
        var_fn482_calc_ig__igindiode_dn18_slot: &mut f64,
        var_fn482_calc_ig__igindiode_dn19_slot: &mut f64,
        var_fn482_calc_ig__igindiode_dn2_slot: &mut f64,
        var_fn482_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn482_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn482_calc_ig__iginrec_slot: &mut f64,
        var_fn482_calc_ig__iginrec_dn0_slot: &mut f64,
        var_fn482_calc_ig__iginrec_dn18_slot: &mut f64,
        var_fn482_calc_ig__iginrec_dn19_slot: &mut f64,
        var_fn482_calc_ig__iginrec_dn2_slot: &mut f64,
        var_fn482_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn482_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn482_calc_ig__igout_slot: &mut f64,
        var_fn482_calc_ig__igout_dn0_slot: &mut f64,
        var_fn482_calc_ig__igout_dn18_slot: &mut f64,
        var_fn482_calc_ig__igout_dn19_slot: &mut f64,
        var_fn482_calc_ig__igout_dn2_slot: &mut f64,
        var_fn482_calc_ig__igout_dn4_slot: &mut f64,
        var_fn482_calc_ig__igout_dn8_slot: &mut f64,
        var_fn482_calc_ig__ijin_slot: &mut f64,
        var_fn482_calc_ig__irecin_slot: &mut f64,
        var_fn482_calc_ig__isdiodeout_slot: &mut f64,
        var_fn482_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn482_calc_ig__isrecout_slot: &mut f64,
        var_fn482_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn482_calc_ig__kbdgatein_slot: &mut f64,
        var_fn482_calc_ig__ngf_slot: &mut f64,
        var_fn482_calc_ig__pbdgin_slot: &mut f64,
        var_fn482_calc_ig__pg_param1_slot: &mut f64,
        var_fn482_calc_ig__pg_paramin_slot: &mut f64,
        var_fn482_calc_ig__pgsrecin_slot: &mut f64,
        var_fn482_calc_ig__phitin_slot: &mut f64,
        var_fn482_calc_ig__phitin_dn4_slot: &mut f64,
        var_fn482_calc_ig__return_slot: &mut f64,
        var_fn482_calc_ig__return_dn0_slot: &mut f64,
        var_fn482_calc_ig__return_dn18_slot: &mut f64,
        var_fn482_calc_ig__return_dn19_slot: &mut f64,
        var_fn482_calc_ig__return_dn2_slot: &mut f64,
        var_fn482_calc_ig__return_dn4_slot: &mut f64,
        var_fn482_calc_ig__return_dn8_slot: &mut f64,
        var_fn482_calc_ig__t0_slot: &mut f64,
        var_fn482_calc_ig__t0_dn4_slot: &mut f64,
        var_fn482_calc_ig__tfacdiodein_slot: &mut f64,
        var_fn482_calc_ig__tfacdiodein_dn4_slot: &mut f64,
        var_fn482_calc_ig__type_slot: &mut f64,
        var_fn482_calc_ig__vbdgin_slot: &mut f64,
        var_fn482_calc_ig__vgin_slot: &mut f64,
        var_fn482_calc_ig__vgin_dn0_slot: &mut f64,
        var_fn482_calc_ig__vgin_dn18_slot: &mut f64,
        var_fn482_calc_ig__vgin_dn19_slot: &mut f64,
        var_fn482_calc_ig__vgin_dn2_slot: &mut f64,
        var_fn482_calc_ig__vgin_dn8_slot: &mut f64,
        var_fn482_calc_ig__vgsatin_slot: &mut f64,
        var_fn482_calc_ig__vgsatqin_slot: &mut f64,
        var_fn482_calc_ig__vjg_slot: &mut f64,
        var_fn482_calc_ig__w_slot: &mut f64,
        var_guard480_slot: &mut f64,
        var_guard481_slot: &mut f64,
        var_vindcbd_slot: &mut f64,
        var_vindcbd_dn0_slot: &mut f64,
        var_vindcbd_dn18_slot: &mut f64,
        var_vindcbd_dn19_slot: &mut f64,
        var_vindcbd_dn2_slot: &mut f64,
        var_vindcbd_dn8_slot: &mut f64,
        var_vinscbd_slot: &mut f64,
        var_vinscbd_dn0_slot: &mut f64,
        var_vinscbd_dn18_slot: &mut f64,
        var_vinscbd_dn19_slot: &mut f64,
        var_vinscbd_dn2_slot: &mut f64,
        var_vinscbd_dn8_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let mut var_fn482_calc_ig__alpha2_phit: f64 = *var_fn482_calc_ig__alpha2_phit_slot;
        let mut var_fn482_calc_ig__alpha2_phit_dn4: f64 = *var_fn482_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn482_calc_ig__alphagin: f64 = *var_fn482_calc_ig__alphagin_slot;
        let mut var_fn482_calc_ig__betarecin: f64 = *var_fn482_calc_ig__betarecin_slot;
        let mut var_fn482_calc_ig__expbd1: f64 = *var_fn482_calc_ig__expbd1_slot;
        let mut var_fn482_calc_ig__expbd1_dn0: f64 = *var_fn482_calc_ig__expbd1_dn0_slot;
        let mut var_fn482_calc_ig__expbd1_dn18: f64 = *var_fn482_calc_ig__expbd1_dn18_slot;
        let mut var_fn482_calc_ig__expbd1_dn19: f64 = *var_fn482_calc_ig__expbd1_dn19_slot;
        let mut var_fn482_calc_ig__expbd1_dn2: f64 = *var_fn482_calc_ig__expbd1_dn2_slot;
        let mut var_fn482_calc_ig__expbd1_dn4: f64 = *var_fn482_calc_ig__expbd1_dn4_slot;
        let mut var_fn482_calc_ig__expbd1_dn8: f64 = *var_fn482_calc_ig__expbd1_dn8_slot;
        let mut var_fn482_calc_ig__expbd2: f64 = *var_fn482_calc_ig__expbd2_slot;
        let mut var_fn482_calc_ig__expbd2_dn4: f64 = *var_fn482_calc_ig__expbd2_dn4_slot;
        let mut var_fn482_calc_ig__expbdarg1: f64 = *var_fn482_calc_ig__expbdarg1_slot;
        let mut var_fn482_calc_ig__expbdarg1_dn0: f64 = *var_fn482_calc_ig__expbdarg1_dn0_slot;
        let mut var_fn482_calc_ig__expbdarg1_dn18: f64 = *var_fn482_calc_ig__expbdarg1_dn18_slot;
        let mut var_fn482_calc_ig__expbdarg1_dn19: f64 = *var_fn482_calc_ig__expbdarg1_dn19_slot;
        let mut var_fn482_calc_ig__expbdarg1_dn2: f64 = *var_fn482_calc_ig__expbdarg1_dn2_slot;
        let mut var_fn482_calc_ig__expbdarg1_dn4: f64 = *var_fn482_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn482_calc_ig__expbdarg1_dn8: f64 = *var_fn482_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn482_calc_ig__expbdarg2: f64 = *var_fn482_calc_ig__expbdarg2_slot;
        let mut var_fn482_calc_ig__expbdarg2_dn4: f64 = *var_fn482_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn482_calc_ig__expffvarg: f64 = *var_fn482_calc_ig__expffvarg_slot;
        let mut var_fn482_calc_ig__expffvarg_dn0: f64 = *var_fn482_calc_ig__expffvarg_dn0_slot;
        let mut var_fn482_calc_ig__expffvarg_dn18: f64 = *var_fn482_calc_ig__expffvarg_dn18_slot;
        let mut var_fn482_calc_ig__expffvarg_dn19: f64 = *var_fn482_calc_ig__expffvarg_dn19_slot;
        let mut var_fn482_calc_ig__expffvarg_dn2: f64 = *var_fn482_calc_ig__expffvarg_dn2_slot;
        let mut var_fn482_calc_ig__expffvarg_dn4: f64 = *var_fn482_calc_ig__expffvarg_dn4_slot;
        let mut var_fn482_calc_ig__expffvarg_dn8: f64 = *var_fn482_calc_ig__expffvarg_dn8_slot;
        let mut var_fn482_calc_ig__expifor: f64 = *var_fn482_calc_ig__expifor_slot;
        let mut var_fn482_calc_ig__expifor_dn0: f64 = *var_fn482_calc_ig__expifor_dn0_slot;
        let mut var_fn482_calc_ig__expifor_dn18: f64 = *var_fn482_calc_ig__expifor_dn18_slot;
        let mut var_fn482_calc_ig__expifor_dn19: f64 = *var_fn482_calc_ig__expifor_dn19_slot;
        let mut var_fn482_calc_ig__expifor_dn2: f64 = *var_fn482_calc_ig__expifor_dn2_slot;
        let mut var_fn482_calc_ig__expifor_dn4: f64 = *var_fn482_calc_ig__expifor_dn4_slot;
        let mut var_fn482_calc_ig__expifor_dn8: f64 = *var_fn482_calc_ig__expifor_dn8_slot;
        let mut var_fn482_calc_ig__expiforarg: f64 = *var_fn482_calc_ig__expiforarg_slot;
        let mut var_fn482_calc_ig__expiforarg_dn0: f64 = *var_fn482_calc_ig__expiforarg_dn0_slot;
        let mut var_fn482_calc_ig__expiforarg_dn18: f64 = *var_fn482_calc_ig__expiforarg_dn18_slot;
        let mut var_fn482_calc_ig__expiforarg_dn19: f64 = *var_fn482_calc_ig__expiforarg_dn19_slot;
        let mut var_fn482_calc_ig__expiforarg_dn2: f64 = *var_fn482_calc_ig__expiforarg_dn2_slot;
        let mut var_fn482_calc_ig__expiforarg_dn4: f64 = *var_fn482_calc_ig__expiforarg_dn4_slot;
        let mut var_fn482_calc_ig__expiforarg_dn8: f64 = *var_fn482_calc_ig__expiforarg_dn8_slot;
        let mut var_fn482_calc_ig__expirev: f64 = *var_fn482_calc_ig__expirev_slot;
        let mut var_fn482_calc_ig__expirev_dn0: f64 = *var_fn482_calc_ig__expirev_dn0_slot;
        let mut var_fn482_calc_ig__expirev_dn18: f64 = *var_fn482_calc_ig__expirev_dn18_slot;
        let mut var_fn482_calc_ig__expirev_dn19: f64 = *var_fn482_calc_ig__expirev_dn19_slot;
        let mut var_fn482_calc_ig__expirev_dn2: f64 = *var_fn482_calc_ig__expirev_dn2_slot;
        let mut var_fn482_calc_ig__expirev_dn4: f64 = *var_fn482_calc_ig__expirev_dn4_slot;
        let mut var_fn482_calc_ig__expirev_dn8: f64 = *var_fn482_calc_ig__expirev_dn8_slot;
        let mut var_fn482_calc_ig__expirevarg: f64 = *var_fn482_calc_ig__expirevarg_slot;
        let mut var_fn482_calc_ig__expirevarg_dn0: f64 = *var_fn482_calc_ig__expirevarg_dn0_slot;
        let mut var_fn482_calc_ig__expirevarg_dn18: f64 = *var_fn482_calc_ig__expirevarg_dn18_slot;
        let mut var_fn482_calc_ig__expirevarg_dn19: f64 = *var_fn482_calc_ig__expirevarg_dn19_slot;
        let mut var_fn482_calc_ig__expirevarg_dn2: f64 = *var_fn482_calc_ig__expirevarg_dn2_slot;
        let mut var_fn482_calc_ig__expirevarg_dn4: f64 = *var_fn482_calc_ig__expirevarg_dn4_slot;
        let mut var_fn482_calc_ig__expirevarg_dn8: f64 = *var_fn482_calc_ig__expirevarg_dn8_slot;
        let mut var_fn482_calc_ig__expphib: f64 = *var_fn482_calc_ig__expphib_slot;
        let mut var_fn482_calc_ig__expphib_dn4: f64 = *var_fn482_calc_ig__expphib_dn4_slot;
        let mut var_fn482_calc_ig__ffvgin: f64 = *var_fn482_calc_ig__ffvgin_slot;
        let mut var_fn482_calc_ig__ffvgin_dn0: f64 = *var_fn482_calc_ig__ffvgin_dn0_slot;
        let mut var_fn482_calc_ig__ffvgin_dn18: f64 = *var_fn482_calc_ig__ffvgin_dn18_slot;
        let mut var_fn482_calc_ig__ffvgin_dn19: f64 = *var_fn482_calc_ig__ffvgin_dn19_slot;
        let mut var_fn482_calc_ig__ffvgin_dn2: f64 = *var_fn482_calc_ig__ffvgin_dn2_slot;
        let mut var_fn482_calc_ig__ffvgin_dn4: f64 = *var_fn482_calc_ig__ffvgin_dn4_slot;
        let mut var_fn482_calc_ig__ffvgin_dn8: f64 = *var_fn482_calc_ig__ffvgin_dn8_slot;
        let mut var_fn482_calc_ig__fracin: f64 = *var_fn482_calc_ig__fracin_slot;
        let mut var_fn482_calc_ig__frecgin: f64 = *var_fn482_calc_ig__frecgin_slot;
        let mut var_fn482_calc_ig__frecgin_dn0: f64 = *var_fn482_calc_ig__frecgin_dn0_slot;
        let mut var_fn482_calc_ig__frecgin_dn18: f64 = *var_fn482_calc_ig__frecgin_dn18_slot;
        let mut var_fn482_calc_ig__frecgin_dn19: f64 = *var_fn482_calc_ig__frecgin_dn19_slot;
        let mut var_fn482_calc_ig__frecgin_dn2: f64 = *var_fn482_calc_ig__frecgin_dn2_slot;
        let mut var_fn482_calc_ig__frecgin_dn8: f64 = *var_fn482_calc_ig__frecgin_dn8_slot;
        let mut var_fn482_calc_ig__iginbd: f64 = *var_fn482_calc_ig__iginbd_slot;
        let mut var_fn482_calc_ig__iginbd_dn0: f64 = *var_fn482_calc_ig__iginbd_dn0_slot;
        let mut var_fn482_calc_ig__iginbd_dn18: f64 = *var_fn482_calc_ig__iginbd_dn18_slot;
        let mut var_fn482_calc_ig__iginbd_dn19: f64 = *var_fn482_calc_ig__iginbd_dn19_slot;
        let mut var_fn482_calc_ig__iginbd_dn2: f64 = *var_fn482_calc_ig__iginbd_dn2_slot;
        let mut var_fn482_calc_ig__iginbd_dn4: f64 = *var_fn482_calc_ig__iginbd_dn4_slot;
        let mut var_fn482_calc_ig__iginbd_dn8: f64 = *var_fn482_calc_ig__iginbd_dn8_slot;
        let mut var_fn482_calc_ig__igindiode: f64 = *var_fn482_calc_ig__igindiode_slot;
        let mut var_fn482_calc_ig__igindiode_dn0: f64 = *var_fn482_calc_ig__igindiode_dn0_slot;
        let mut var_fn482_calc_ig__igindiode_dn18: f64 = *var_fn482_calc_ig__igindiode_dn18_slot;
        let mut var_fn482_calc_ig__igindiode_dn19: f64 = *var_fn482_calc_ig__igindiode_dn19_slot;
        let mut var_fn482_calc_ig__igindiode_dn2: f64 = *var_fn482_calc_ig__igindiode_dn2_slot;
        let mut var_fn482_calc_ig__igindiode_dn4: f64 = *var_fn482_calc_ig__igindiode_dn4_slot;
        let mut var_fn482_calc_ig__igindiode_dn8: f64 = *var_fn482_calc_ig__igindiode_dn8_slot;
        let mut var_fn482_calc_ig__iginrec: f64 = *var_fn482_calc_ig__iginrec_slot;
        let mut var_fn482_calc_ig__iginrec_dn0: f64 = *var_fn482_calc_ig__iginrec_dn0_slot;
        let mut var_fn482_calc_ig__iginrec_dn18: f64 = *var_fn482_calc_ig__iginrec_dn18_slot;
        let mut var_fn482_calc_ig__iginrec_dn19: f64 = *var_fn482_calc_ig__iginrec_dn19_slot;
        let mut var_fn482_calc_ig__iginrec_dn2: f64 = *var_fn482_calc_ig__iginrec_dn2_slot;
        let mut var_fn482_calc_ig__iginrec_dn4: f64 = *var_fn482_calc_ig__iginrec_dn4_slot;
        let mut var_fn482_calc_ig__iginrec_dn8: f64 = *var_fn482_calc_ig__iginrec_dn8_slot;
        let mut var_fn482_calc_ig__igout: f64 = *var_fn482_calc_ig__igout_slot;
        let mut var_fn482_calc_ig__igout_dn0: f64 = *var_fn482_calc_ig__igout_dn0_slot;
        let mut var_fn482_calc_ig__igout_dn18: f64 = *var_fn482_calc_ig__igout_dn18_slot;
        let mut var_fn482_calc_ig__igout_dn19: f64 = *var_fn482_calc_ig__igout_dn19_slot;
        let mut var_fn482_calc_ig__igout_dn2: f64 = *var_fn482_calc_ig__igout_dn2_slot;
        let mut var_fn482_calc_ig__igout_dn4: f64 = *var_fn482_calc_ig__igout_dn4_slot;
        let mut var_fn482_calc_ig__igout_dn8: f64 = *var_fn482_calc_ig__igout_dn8_slot;
        let mut var_fn482_calc_ig__ijin: f64 = *var_fn482_calc_ig__ijin_slot;
        let mut var_fn482_calc_ig__irecin: f64 = *var_fn482_calc_ig__irecin_slot;
        let mut var_fn482_calc_ig__isdiodeout: f64 = *var_fn482_calc_ig__isdiodeout_slot;
        let mut var_fn482_calc_ig__isdiodeout_dn4: f64 = *var_fn482_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn482_calc_ig__isrecout: f64 = *var_fn482_calc_ig__isrecout_slot;
        let mut var_fn482_calc_ig__isrecout_dn4: f64 = *var_fn482_calc_ig__isrecout_dn4_slot;
        let mut var_fn482_calc_ig__kbdgatein: f64 = *var_fn482_calc_ig__kbdgatein_slot;
        let mut var_fn482_calc_ig__ngf: f64 = *var_fn482_calc_ig__ngf_slot;
        let mut var_fn482_calc_ig__pbdgin: f64 = *var_fn482_calc_ig__pbdgin_slot;
        let mut var_fn482_calc_ig__pg_param1: f64 = *var_fn482_calc_ig__pg_param1_slot;
        let mut var_fn482_calc_ig__pg_paramin: f64 = *var_fn482_calc_ig__pg_paramin_slot;
        let mut var_fn482_calc_ig__pgsrecin: f64 = *var_fn482_calc_ig__pgsrecin_slot;
        let mut var_fn482_calc_ig__phitin: f64 = *var_fn482_calc_ig__phitin_slot;
        let mut var_fn482_calc_ig__phitin_dn4: f64 = *var_fn482_calc_ig__phitin_dn4_slot;
        let mut var_fn482_calc_ig__return: f64 = *var_fn482_calc_ig__return_slot;
        let mut var_fn482_calc_ig__return_dn0: f64 = *var_fn482_calc_ig__return_dn0_slot;
        let mut var_fn482_calc_ig__return_dn18: f64 = *var_fn482_calc_ig__return_dn18_slot;
        let mut var_fn482_calc_ig__return_dn19: f64 = *var_fn482_calc_ig__return_dn19_slot;
        let mut var_fn482_calc_ig__return_dn2: f64 = *var_fn482_calc_ig__return_dn2_slot;
        let mut var_fn482_calc_ig__return_dn4: f64 = *var_fn482_calc_ig__return_dn4_slot;
        let mut var_fn482_calc_ig__return_dn8: f64 = *var_fn482_calc_ig__return_dn8_slot;
        let mut var_fn482_calc_ig__t0: f64 = *var_fn482_calc_ig__t0_slot;
        let mut var_fn482_calc_ig__t0_dn4: f64 = *var_fn482_calc_ig__t0_dn4_slot;
        let mut var_fn482_calc_ig__tfacdiodein: f64 = *var_fn482_calc_ig__tfacdiodein_slot;
        let mut var_fn482_calc_ig__tfacdiodein_dn4: f64 = *var_fn482_calc_ig__tfacdiodein_dn4_slot;
        let mut var_fn482_calc_ig__type: f64 = *var_fn482_calc_ig__type_slot;
        let mut var_fn482_calc_ig__vbdgin: f64 = *var_fn482_calc_ig__vbdgin_slot;
        let mut var_fn482_calc_ig__vgin: f64 = *var_fn482_calc_ig__vgin_slot;
        let mut var_fn482_calc_ig__vgin_dn0: f64 = *var_fn482_calc_ig__vgin_dn0_slot;
        let mut var_fn482_calc_ig__vgin_dn18: f64 = *var_fn482_calc_ig__vgin_dn18_slot;
        let mut var_fn482_calc_ig__vgin_dn19: f64 = *var_fn482_calc_ig__vgin_dn19_slot;
        let mut var_fn482_calc_ig__vgin_dn2: f64 = *var_fn482_calc_ig__vgin_dn2_slot;
        let mut var_fn482_calc_ig__vgin_dn8: f64 = *var_fn482_calc_ig__vgin_dn8_slot;
        let mut var_fn482_calc_ig__vgsatin: f64 = *var_fn482_calc_ig__vgsatin_slot;
        let mut var_fn482_calc_ig__vgsatqin: f64 = *var_fn482_calc_ig__vgsatqin_slot;
        let mut var_fn482_calc_ig__vjg: f64 = *var_fn482_calc_ig__vjg_slot;
        let mut var_fn482_calc_ig__w: f64 = *var_fn482_calc_ig__w_slot;
        let mut var_guard480: f64 = *var_guard480_slot;
        let mut var_guard481: f64 = *var_guard481_slot;
        let mut var_vindcbd: f64 = *var_vindcbd_slot;
        let mut var_vindcbd_dn0: f64 = *var_vindcbd_dn0_slot;
        let mut var_vindcbd_dn18: f64 = *var_vindcbd_dn18_slot;
        let mut var_vindcbd_dn19: f64 = *var_vindcbd_dn19_slot;
        let mut var_vindcbd_dn2: f64 = *var_vindcbd_dn2_slot;
        let mut var_vindcbd_dn8: f64 = *var_vindcbd_dn8_slot;
        let mut var_vinscbd: f64 = *var_vinscbd_slot;
        let mut var_vinscbd_dn0: f64 = *var_vinscbd_dn0_slot;
        let mut var_vinscbd_dn18: f64 = *var_vinscbd_dn18_slot;
        let mut var_vinscbd_dn19: f64 = *var_vinscbd_dn19_slot;
        let mut var_vinscbd_dn2: f64 = *var_vinscbd_dn2_slot;
        let mut var_vinscbd_dn8: f64 = *var_vinscbd_dn8_slot;

        let assign44010_e42667: f64 = if p.p312 == 1.0 { 1.0 } else { 0.0 };
        var_guard480 = assign44010_e42667;

        let assign44020_e42670: f64 = if p.p313 == 0.0 { 1.0 } else { 0.0 };
        var_guard481 = assign44020_e42670;

        let (assign44030_e42680, assign44030_e42680_d_n0, assign44030_e42680_d_n2, assign44030_e42680_d_n8, assign44030_e42680_d_n18, assign44030_e42680_d_n19,) = {
    if ((var_guard480 != 0.0) && (var_guard481 != 0.0)) {
        let assign44030_e42677: f64 = ((nv2 - nv0) + (nv2 - nv8));
        let assign44030_e42678: f64 = (p.p6 * assign44030_e42677);
        (assign44030_e42678, (-p.p6), (p.p6 * (1.0 + 1.0)), (-p.p6), 0.0, 0.0,)
    } else {
        (var_vinscbd, var_vinscbd_dn0, var_vinscbd_dn2, var_vinscbd_dn8, var_vinscbd_dn18, var_vinscbd_dn19,)
    }
};
        var_vinscbd = assign44030_e42680;
        var_vinscbd_dn0 = assign44030_e42680_d_n0;
        var_vinscbd_dn2 = assign44030_e42680_d_n2;
        var_vinscbd_dn8 = assign44030_e42680_d_n8;
        var_vinscbd_dn18 = assign44030_e42680_d_n18;
        var_vinscbd_dn19 = assign44030_e42680_d_n19;

        let (assign44040_e42690, assign44040_e42690_d_n0, assign44040_e42690_d_n2, assign44040_e42690_d_n8, assign44040_e42690_d_n18, assign44040_e42690_d_n19,) = {
    if ((var_guard480 != 0.0) && (var_guard481 != 0.0)) {
        let assign44040_e42687: f64 = ((nv0 - nv2) + (nv0 - nv8));
        let assign44040_e42688: f64 = (p.p6 * assign44040_e42687);
        (assign44040_e42688, (p.p6 * (1.0 + 1.0)), (-p.p6), (-p.p6), 0.0, 0.0,)
    } else {
        (var_vindcbd, var_vindcbd_dn0, var_vindcbd_dn2, var_vindcbd_dn8, var_vindcbd_dn18, var_vindcbd_dn19,)
    }
};
        var_vindcbd = assign44040_e42690;
        var_vindcbd_dn0 = assign44040_e42690_d_n0;
        var_vindcbd_dn2 = assign44040_e42690_d_n2;
        var_vindcbd_dn8 = assign44040_e42690_d_n8;
        var_vindcbd_dn18 = assign44040_e42690_d_n18;
        var_vindcbd_dn19 = assign44040_e42690_d_n19;

        let (assign44050_e42694, assign44050_e42694_d_n0, assign44050_e42694_d_n2, assign44050_e42694_d_n4, assign44050_e42694_d_n8, assign44050_e42694_d_n18, assign44050_e42694_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__return, var_fn482_calc_ig__return_dn0, var_fn482_calc_ig__return_dn2, var_fn482_calc_ig__return_dn4, var_fn482_calc_ig__return_dn8, var_fn482_calc_ig__return_dn18, var_fn482_calc_ig__return_dn19,)
    }
};
        var_fn482_calc_ig__return = assign44050_e42694;
        var_fn482_calc_ig__return_dn0 = assign44050_e42694_d_n0;
        var_fn482_calc_ig__return_dn2 = assign44050_e42694_d_n2;
        var_fn482_calc_ig__return_dn4 = assign44050_e42694_d_n4;
        var_fn482_calc_ig__return_dn8 = assign44050_e42694_d_n8;
        var_fn482_calc_ig__return_dn18 = assign44050_e42694_d_n18;
        var_fn482_calc_ig__return_dn19 = assign44050_e42694_d_n19;

        let (assign44060_e42698, assign44060_e42698_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__isdiodeout, var_fn482_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn482_calc_ig__isdiodeout = assign44060_e42698;
        var_fn482_calc_ig__isdiodeout_dn4 = assign44060_e42698_d_n4;

        let (assign44070_e42702, assign44070_e42702_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__isrecout, var_fn482_calc_ig__isrecout_dn4,)
    }
};
        var_fn482_calc_ig__isrecout = assign44070_e42702;
        var_fn482_calc_ig__isrecout_dn4 = assign44070_e42702_d_n4;

        let (assign44080_e42706, assign44080_e42706_d_n0, assign44080_e42706_d_n2, assign44080_e42706_d_n8, assign44080_e42706_d_n18, assign44080_e42706_d_n19,) = {
    if (var_guard480 != 0.0) {
        (var_vinscbd, var_vinscbd_dn0, var_vinscbd_dn2, var_vinscbd_dn8, var_vinscbd_dn18, var_vinscbd_dn19,)
    } else {
        (var_fn482_calc_ig__vgin, var_fn482_calc_ig__vgin_dn0, var_fn482_calc_ig__vgin_dn2, var_fn482_calc_ig__vgin_dn8, var_fn482_calc_ig__vgin_dn18, var_fn482_calc_ig__vgin_dn19,)
    }
};
        var_fn482_calc_ig__vgin = assign44080_e42706;
        var_fn482_calc_ig__vgin_dn0 = assign44080_e42706_d_n0;
        var_fn482_calc_ig__vgin_dn2 = assign44080_e42706_d_n2;
        var_fn482_calc_ig__vgin_dn8 = assign44080_e42706_d_n8;
        var_fn482_calc_ig__vgin_dn18 = assign44080_e42706_d_n18;
        var_fn482_calc_ig__vgin_dn19 = assign44080_e42706_d_n19;

        let (assign44090_e42710, assign44090_e42710_d_n4,) = {
    if (var_guard480 != 0.0) {
        (var_phit, var_phit_dn4,)
    } else {
        (var_fn482_calc_ig__phitin, var_fn482_calc_ig__phitin_dn4,)
    }
};
        var_fn482_calc_ig__phitin = assign44090_e42710;
        var_fn482_calc_ig__phitin_dn4 = assign44090_e42710_d_n4;

        let (assign44100_e42714,) = {
    if (var_guard480 != 0.0) {
        (p.p260,)
    } else {
        (var_fn482_calc_ig__vgsatin,)
    }
};
        var_fn482_calc_ig__vgsatin = assign44100_e42714;

        let (assign44110_e42718,) = {
    if (var_guard480 != 0.0) {
        (p.p262,)
    } else {
        (var_fn482_calc_ig__alphagin,)
    }
};
        var_fn482_calc_ig__alphagin = assign44110_e42718;

        let (assign44120_e42722,) = {
    if (var_guard480 != 0.0) {
        (p.p261,)
    } else {
        (var_fn482_calc_ig__fracin,)
    }
};
        var_fn482_calc_ig__fracin = assign44120_e42722;

        let (assign44130_e42726,) = {
    if (var_guard480 != 0.0) {
        (0.0,)
    } else {
        (var_fn482_calc_ig__pg_paramin,)
    }
};
        var_fn482_calc_ig__pg_paramin = assign44130_e42726;

        let (assign44140_e42730,) = {
    if (var_guard480 != 0.0) {
        (p.p317,)
    } else {
        (var_fn482_calc_ig__pbdgin,)
    }
};
        var_fn482_calc_ig__pbdgin = assign44140_e42730;

        let (assign44150_e42734,) = {
    if (var_guard480 != 0.0) {
        (p.p316,)
    } else {
        (var_fn482_calc_ig__vbdgin,)
    }
};
        var_fn482_calc_ig__vbdgin = assign44150_e42734;

        let (assign44160_e42738, assign44160_e42738_d_n4,) = {
    if (var_guard480 != 0.0) {
        (var_tfacdiode, var_tfacdiode_dn4,)
    } else {
        (var_fn482_calc_ig__tfacdiodein, var_fn482_calc_ig__tfacdiodein_dn4,)
    }
};
        var_fn482_calc_ig__tfacdiodein = assign44160_e42738;
        var_fn482_calc_ig__tfacdiodein_dn4 = assign44160_e42738_d_n4;

        let (assign44170_e42742,) = {
    if (var_guard480 != 0.0) {
        (p.p0,)
    } else {
        (var_fn482_calc_ig__w,)
    }
};
        var_fn482_calc_ig__w = assign44170_e42742;

        let (assign44180_e42746,) = {
    if (var_guard480 != 0.0) {
        (p.p2,)
    } else {
        (var_fn482_calc_ig__ngf,)
    }
};
        var_fn482_calc_ig__ngf = assign44180_e42746;

        let (assign44190_e42750,) = {
    if (var_guard480 != 0.0) {
        (p.p314,)
    } else {
        (var_fn482_calc_ig__ijin,)
    }
};
        var_fn482_calc_ig__ijin = assign44190_e42750;

        let (assign44200_e42754,) = {
    if (var_guard480 != 0.0) {
        (1.0,)
    } else {
        (var_fn482_calc_ig__kbdgatein,)
    }
};
        var_fn482_calc_ig__kbdgatein = assign44200_e42754;

        let (assign44210_e42758,) = {
    if (var_guard480 != 0.0) {
        (p.p270,)
    } else {
        (var_fn482_calc_ig__vgsatqin,)
    }
};
        var_fn482_calc_ig__vgsatqin = assign44210_e42758;

        let (assign44220_e42762,) = {
    if (var_guard480 != 0.0) {
        (p.p271,)
    } else {
        (var_fn482_calc_ig__betarecin,)
    }
};
        var_fn482_calc_ig__betarecin = assign44220_e42762;

        let (assign44230_e42766,) = {
    if (var_guard480 != 0.0) {
        (0.0,)
    } else {
        (var_fn482_calc_ig__irecin,)
    }
};
        var_fn482_calc_ig__irecin = assign44230_e42766;

        let (assign44240_e42770,) = {
    if (var_guard480 != 0.0) {
        (p.p268,)
    } else {
        (var_fn482_calc_ig__pgsrecin,)
    }
};
        var_fn482_calc_ig__pgsrecin = assign44240_e42770;

        let (assign44250_e42774,) = {
    if (var_guard480 != 0.0) {
        (0.0,)
    } else {
        (var_fn482_calc_ig__pg_param1,)
    }
};
        var_fn482_calc_ig__pg_param1 = assign44250_e42774;

        let (assign44260_e42778,) = {
    if (var_guard480 != 0.0) {
        (p.p256,)
    } else {
        (var_fn482_calc_ig__vjg,)
    }
};
        var_fn482_calc_ig__vjg = assign44260_e42778;

        let (assign44270_e42782,) = {
    if (var_guard480 != 0.0) {
        (p.p6,)
    } else {
        (var_fn482_calc_ig__type,)
    }
};
        var_fn482_calc_ig__type = assign44270_e42782;

        let (assign44280_e42786, assign44280_e42786_d_n0, assign44280_e42786_d_n2, assign44280_e42786_d_n4, assign44280_e42786_d_n8, assign44280_e42786_d_n18, assign44280_e42786_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__igout, var_fn482_calc_ig__igout_dn0, var_fn482_calc_ig__igout_dn2, var_fn482_calc_ig__igout_dn4, var_fn482_calc_ig__igout_dn8, var_fn482_calc_ig__igout_dn18, var_fn482_calc_ig__igout_dn19,)
    }
};
        var_fn482_calc_ig__igout = assign44280_e42786;
        var_fn482_calc_ig__igout_dn0 = assign44280_e42786_d_n0;
        var_fn482_calc_ig__igout_dn2 = assign44280_e42786_d_n2;
        var_fn482_calc_ig__igout_dn4 = assign44280_e42786_d_n4;
        var_fn482_calc_ig__igout_dn8 = assign44280_e42786_d_n8;
        var_fn482_calc_ig__igout_dn18 = assign44280_e42786_d_n18;
        var_fn482_calc_ig__igout_dn19 = assign44280_e42786_d_n19;

        let (assign44290_e42790, assign44290_e42790_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__alpha2_phit, var_fn482_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn482_calc_ig__alpha2_phit = assign44290_e42790;
        var_fn482_calc_ig__alpha2_phit_dn4 = assign44290_e42790_d_n4;

        let (assign44300_e42794, assign44300_e42794_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__t0, var_fn482_calc_ig__t0_dn4,)
    }
};
        var_fn482_calc_ig__t0 = assign44300_e42794;
        var_fn482_calc_ig__t0_dn4 = assign44300_e42794_d_n4;

        let (assign44310_e42798, assign44310_e42798_d_n0, assign44310_e42798_d_n2, assign44310_e42798_d_n4, assign44310_e42798_d_n8, assign44310_e42798_d_n18, assign44310_e42798_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__ffvgin, var_fn482_calc_ig__ffvgin_dn0, var_fn482_calc_ig__ffvgin_dn2, var_fn482_calc_ig__ffvgin_dn4, var_fn482_calc_ig__ffvgin_dn8, var_fn482_calc_ig__ffvgin_dn18, var_fn482_calc_ig__ffvgin_dn19,)
    }
};
        var_fn482_calc_ig__ffvgin = assign44310_e42798;
        var_fn482_calc_ig__ffvgin_dn0 = assign44310_e42798_d_n0;
        var_fn482_calc_ig__ffvgin_dn2 = assign44310_e42798_d_n2;
        var_fn482_calc_ig__ffvgin_dn4 = assign44310_e42798_d_n4;
        var_fn482_calc_ig__ffvgin_dn8 = assign44310_e42798_d_n8;
        var_fn482_calc_ig__ffvgin_dn18 = assign44310_e42798_d_n18;
        var_fn482_calc_ig__ffvgin_dn19 = assign44310_e42798_d_n19;

        let (assign44320_e42802, assign44320_e42802_d_n0, assign44320_e42802_d_n2, assign44320_e42802_d_n4, assign44320_e42802_d_n8, assign44320_e42802_d_n18, assign44320_e42802_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__iginbd, var_fn482_calc_ig__iginbd_dn0, var_fn482_calc_ig__iginbd_dn2, var_fn482_calc_ig__iginbd_dn4, var_fn482_calc_ig__iginbd_dn8, var_fn482_calc_ig__iginbd_dn18, var_fn482_calc_ig__iginbd_dn19,)
    }
};
        var_fn482_calc_ig__iginbd = assign44320_e42802;
        var_fn482_calc_ig__iginbd_dn0 = assign44320_e42802_d_n0;
        var_fn482_calc_ig__iginbd_dn2 = assign44320_e42802_d_n2;
        var_fn482_calc_ig__iginbd_dn4 = assign44320_e42802_d_n4;
        var_fn482_calc_ig__iginbd_dn8 = assign44320_e42802_d_n8;
        var_fn482_calc_ig__iginbd_dn18 = assign44320_e42802_d_n18;
        var_fn482_calc_ig__iginbd_dn19 = assign44320_e42802_d_n19;

        let (assign44330_e42806, assign44330_e42806_d_n0, assign44330_e42806_d_n2, assign44330_e42806_d_n4, assign44330_e42806_d_n8, assign44330_e42806_d_n18, assign44330_e42806_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__igindiode, var_fn482_calc_ig__igindiode_dn0, var_fn482_calc_ig__igindiode_dn2, var_fn482_calc_ig__igindiode_dn4, var_fn482_calc_ig__igindiode_dn8, var_fn482_calc_ig__igindiode_dn18, var_fn482_calc_ig__igindiode_dn19,)
    }
};
        var_fn482_calc_ig__igindiode = assign44330_e42806;
        var_fn482_calc_ig__igindiode_dn0 = assign44330_e42806_d_n0;
        var_fn482_calc_ig__igindiode_dn2 = assign44330_e42806_d_n2;
        var_fn482_calc_ig__igindiode_dn4 = assign44330_e42806_d_n4;
        var_fn482_calc_ig__igindiode_dn8 = assign44330_e42806_d_n8;
        var_fn482_calc_ig__igindiode_dn18 = assign44330_e42806_d_n18;
        var_fn482_calc_ig__igindiode_dn19 = assign44330_e42806_d_n19;

        let (assign44340_e42810, assign44340_e42810_d_n0, assign44340_e42810_d_n2, assign44340_e42810_d_n8, assign44340_e42810_d_n18, assign44340_e42810_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__frecgin, var_fn482_calc_ig__frecgin_dn0, var_fn482_calc_ig__frecgin_dn2, var_fn482_calc_ig__frecgin_dn8, var_fn482_calc_ig__frecgin_dn18, var_fn482_calc_ig__frecgin_dn19,)
    }
};
        var_fn482_calc_ig__frecgin = assign44340_e42810;
        var_fn482_calc_ig__frecgin_dn0 = assign44340_e42810_d_n0;
        var_fn482_calc_ig__frecgin_dn2 = assign44340_e42810_d_n2;
        var_fn482_calc_ig__frecgin_dn8 = assign44340_e42810_d_n8;
        var_fn482_calc_ig__frecgin_dn18 = assign44340_e42810_d_n18;
        var_fn482_calc_ig__frecgin_dn19 = assign44340_e42810_d_n19;

        let (assign44350_e42814, assign44350_e42814_d_n0, assign44350_e42814_d_n2, assign44350_e42814_d_n4, assign44350_e42814_d_n8, assign44350_e42814_d_n18, assign44350_e42814_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__iginrec, var_fn482_calc_ig__iginrec_dn0, var_fn482_calc_ig__iginrec_dn2, var_fn482_calc_ig__iginrec_dn4, var_fn482_calc_ig__iginrec_dn8, var_fn482_calc_ig__iginrec_dn18, var_fn482_calc_ig__iginrec_dn19,)
    }
};
        var_fn482_calc_ig__iginrec = assign44350_e42814;
        var_fn482_calc_ig__iginrec_dn0 = assign44350_e42814_d_n0;
        var_fn482_calc_ig__iginrec_dn2 = assign44350_e42814_d_n2;
        var_fn482_calc_ig__iginrec_dn4 = assign44350_e42814_d_n4;
        var_fn482_calc_ig__iginrec_dn8 = assign44350_e42814_d_n8;
        var_fn482_calc_ig__iginrec_dn18 = assign44350_e42814_d_n18;
        var_fn482_calc_ig__iginrec_dn19 = assign44350_e42814_d_n19;

        let (assign44360_e42818, assign44360_e42818_d_n0, assign44360_e42818_d_n2, assign44360_e42818_d_n4, assign44360_e42818_d_n8, assign44360_e42818_d_n18, assign44360_e42818_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expbdarg1, var_fn482_calc_ig__expbdarg1_dn0, var_fn482_calc_ig__expbdarg1_dn2, var_fn482_calc_ig__expbdarg1_dn4, var_fn482_calc_ig__expbdarg1_dn8, var_fn482_calc_ig__expbdarg1_dn18, var_fn482_calc_ig__expbdarg1_dn19,)
    }
};
        var_fn482_calc_ig__expbdarg1 = assign44360_e42818;
        var_fn482_calc_ig__expbdarg1_dn0 = assign44360_e42818_d_n0;
        var_fn482_calc_ig__expbdarg1_dn2 = assign44360_e42818_d_n2;
        var_fn482_calc_ig__expbdarg1_dn4 = assign44360_e42818_d_n4;
        var_fn482_calc_ig__expbdarg1_dn8 = assign44360_e42818_d_n8;
        var_fn482_calc_ig__expbdarg1_dn18 = assign44360_e42818_d_n18;
        var_fn482_calc_ig__expbdarg1_dn19 = assign44360_e42818_d_n19;

        let (assign44370_e42822, assign44370_e42822_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expbdarg2, var_fn482_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn482_calc_ig__expbdarg2 = assign44370_e42822;
        var_fn482_calc_ig__expbdarg2_dn4 = assign44370_e42822_d_n4;

        let (assign44380_e42826, assign44380_e42826_d_n0, assign44380_e42826_d_n2, assign44380_e42826_d_n4, assign44380_e42826_d_n8, assign44380_e42826_d_n18, assign44380_e42826_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expbd1, var_fn482_calc_ig__expbd1_dn0, var_fn482_calc_ig__expbd1_dn2, var_fn482_calc_ig__expbd1_dn4, var_fn482_calc_ig__expbd1_dn8, var_fn482_calc_ig__expbd1_dn18, var_fn482_calc_ig__expbd1_dn19,)
    }
};
        var_fn482_calc_ig__expbd1 = assign44380_e42826;
        var_fn482_calc_ig__expbd1_dn0 = assign44380_e42826_d_n0;
        var_fn482_calc_ig__expbd1_dn2 = assign44380_e42826_d_n2;
        var_fn482_calc_ig__expbd1_dn4 = assign44380_e42826_d_n4;
        var_fn482_calc_ig__expbd1_dn8 = assign44380_e42826_d_n8;
        var_fn482_calc_ig__expbd1_dn18 = assign44380_e42826_d_n18;
        var_fn482_calc_ig__expbd1_dn19 = assign44380_e42826_d_n19;

        let (assign44390_e42830, assign44390_e42830_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expbd2, var_fn482_calc_ig__expbd2_dn4,)
    }
};
        var_fn482_calc_ig__expbd2 = assign44390_e42830;
        var_fn482_calc_ig__expbd2_dn4 = assign44390_e42830_d_n4;

        let (assign44400_e42834, assign44400_e42834_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expphib, var_fn482_calc_ig__expphib_dn4,)
    }
};
        var_fn482_calc_ig__expphib = assign44400_e42834;
        var_fn482_calc_ig__expphib_dn4 = assign44400_e42834_d_n4;

        let (assign44410_e42838, assign44410_e42838_d_n0, assign44410_e42838_d_n2, assign44410_e42838_d_n4, assign44410_e42838_d_n8, assign44410_e42838_d_n18, assign44410_e42838_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expffvarg, var_fn482_calc_ig__expffvarg_dn0, var_fn482_calc_ig__expffvarg_dn2, var_fn482_calc_ig__expffvarg_dn4, var_fn482_calc_ig__expffvarg_dn8, var_fn482_calc_ig__expffvarg_dn18, var_fn482_calc_ig__expffvarg_dn19,)
    }
};
        var_fn482_calc_ig__expffvarg = assign44410_e42838;
        var_fn482_calc_ig__expffvarg_dn0 = assign44410_e42838_d_n0;
        var_fn482_calc_ig__expffvarg_dn2 = assign44410_e42838_d_n2;
        var_fn482_calc_ig__expffvarg_dn4 = assign44410_e42838_d_n4;
        var_fn482_calc_ig__expffvarg_dn8 = assign44410_e42838_d_n8;
        var_fn482_calc_ig__expffvarg_dn18 = assign44410_e42838_d_n18;
        var_fn482_calc_ig__expffvarg_dn19 = assign44410_e42838_d_n19;

        let (assign44420_e42842, assign44420_e42842_d_n0, assign44420_e42842_d_n2, assign44420_e42842_d_n4, assign44420_e42842_d_n8, assign44420_e42842_d_n18, assign44420_e42842_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expiforarg, var_fn482_calc_ig__expiforarg_dn0, var_fn482_calc_ig__expiforarg_dn2, var_fn482_calc_ig__expiforarg_dn4, var_fn482_calc_ig__expiforarg_dn8, var_fn482_calc_ig__expiforarg_dn18, var_fn482_calc_ig__expiforarg_dn19,)
    }
};
        var_fn482_calc_ig__expiforarg = assign44420_e42842;
        var_fn482_calc_ig__expiforarg_dn0 = assign44420_e42842_d_n0;
        var_fn482_calc_ig__expiforarg_dn2 = assign44420_e42842_d_n2;
        var_fn482_calc_ig__expiforarg_dn4 = assign44420_e42842_d_n4;
        var_fn482_calc_ig__expiforarg_dn8 = assign44420_e42842_d_n8;
        var_fn482_calc_ig__expiforarg_dn18 = assign44420_e42842_d_n18;
        var_fn482_calc_ig__expiforarg_dn19 = assign44420_e42842_d_n19;

        let (assign44430_e42846, assign44430_e42846_d_n0, assign44430_e42846_d_n2, assign44430_e42846_d_n4, assign44430_e42846_d_n8, assign44430_e42846_d_n18, assign44430_e42846_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expifor, var_fn482_calc_ig__expifor_dn0, var_fn482_calc_ig__expifor_dn2, var_fn482_calc_ig__expifor_dn4, var_fn482_calc_ig__expifor_dn8, var_fn482_calc_ig__expifor_dn18, var_fn482_calc_ig__expifor_dn19,)
    }
};
        var_fn482_calc_ig__expifor = assign44430_e42846;
        var_fn482_calc_ig__expifor_dn0 = assign44430_e42846_d_n0;
        var_fn482_calc_ig__expifor_dn2 = assign44430_e42846_d_n2;
        var_fn482_calc_ig__expifor_dn4 = assign44430_e42846_d_n4;
        var_fn482_calc_ig__expifor_dn8 = assign44430_e42846_d_n8;
        var_fn482_calc_ig__expifor_dn18 = assign44430_e42846_d_n18;
        var_fn482_calc_ig__expifor_dn19 = assign44430_e42846_d_n19;

        let (assign44440_e42850, assign44440_e42850_d_n0, assign44440_e42850_d_n2, assign44440_e42850_d_n4, assign44440_e42850_d_n8, assign44440_e42850_d_n18, assign44440_e42850_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expirevarg, var_fn482_calc_ig__expirevarg_dn0, var_fn482_calc_ig__expirevarg_dn2, var_fn482_calc_ig__expirevarg_dn4, var_fn482_calc_ig__expirevarg_dn8, var_fn482_calc_ig__expirevarg_dn18, var_fn482_calc_ig__expirevarg_dn19,)
    }
};
        var_fn482_calc_ig__expirevarg = assign44440_e42850;
        var_fn482_calc_ig__expirevarg_dn0 = assign44440_e42850_d_n0;
        var_fn482_calc_ig__expirevarg_dn2 = assign44440_e42850_d_n2;
        var_fn482_calc_ig__expirevarg_dn4 = assign44440_e42850_d_n4;
        var_fn482_calc_ig__expirevarg_dn8 = assign44440_e42850_d_n8;
        var_fn482_calc_ig__expirevarg_dn18 = assign44440_e42850_d_n18;
        var_fn482_calc_ig__expirevarg_dn19 = assign44440_e42850_d_n19;

        let (assign44450_e42854, assign44450_e42854_d_n0, assign44450_e42854_d_n2, assign44450_e42854_d_n4, assign44450_e42854_d_n8, assign44450_e42854_d_n18, assign44450_e42854_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expirev, var_fn482_calc_ig__expirev_dn0, var_fn482_calc_ig__expirev_dn2, var_fn482_calc_ig__expirev_dn4, var_fn482_calc_ig__expirev_dn8, var_fn482_calc_ig__expirev_dn18, var_fn482_calc_ig__expirev_dn19,)
    }
};
        var_fn482_calc_ig__expirev = assign44450_e42854;
        var_fn482_calc_ig__expirev_dn0 = assign44450_e42854_d_n0;
        var_fn482_calc_ig__expirev_dn2 = assign44450_e42854_d_n2;
        var_fn482_calc_ig__expirev_dn4 = assign44450_e42854_d_n4;
        var_fn482_calc_ig__expirev_dn8 = assign44450_e42854_d_n8;
        var_fn482_calc_ig__expirev_dn18 = assign44450_e42854_d_n18;
        var_fn482_calc_ig__expirev_dn19 = assign44450_e42854_d_n19;

        *var_fn482_calc_ig__alpha2_phit_slot = var_fn482_calc_ig__alpha2_phit;
        *var_fn482_calc_ig__alpha2_phit_dn4_slot = var_fn482_calc_ig__alpha2_phit_dn4;
        *var_fn482_calc_ig__alphagin_slot = var_fn482_calc_ig__alphagin;
        *var_fn482_calc_ig__betarecin_slot = var_fn482_calc_ig__betarecin;
        *var_fn482_calc_ig__expbd1_slot = var_fn482_calc_ig__expbd1;
        *var_fn482_calc_ig__expbd1_dn0_slot = var_fn482_calc_ig__expbd1_dn0;
        *var_fn482_calc_ig__expbd1_dn18_slot = var_fn482_calc_ig__expbd1_dn18;
        *var_fn482_calc_ig__expbd1_dn19_slot = var_fn482_calc_ig__expbd1_dn19;
        *var_fn482_calc_ig__expbd1_dn2_slot = var_fn482_calc_ig__expbd1_dn2;
        *var_fn482_calc_ig__expbd1_dn4_slot = var_fn482_calc_ig__expbd1_dn4;
        *var_fn482_calc_ig__expbd1_dn8_slot = var_fn482_calc_ig__expbd1_dn8;
        *var_fn482_calc_ig__expbd2_slot = var_fn482_calc_ig__expbd2;
        *var_fn482_calc_ig__expbd2_dn4_slot = var_fn482_calc_ig__expbd2_dn4;
        *var_fn482_calc_ig__expbdarg1_slot = var_fn482_calc_ig__expbdarg1;
        *var_fn482_calc_ig__expbdarg1_dn0_slot = var_fn482_calc_ig__expbdarg1_dn0;
        *var_fn482_calc_ig__expbdarg1_dn18_slot = var_fn482_calc_ig__expbdarg1_dn18;
        *var_fn482_calc_ig__expbdarg1_dn19_slot = var_fn482_calc_ig__expbdarg1_dn19;
        *var_fn482_calc_ig__expbdarg1_dn2_slot = var_fn482_calc_ig__expbdarg1_dn2;
        *var_fn482_calc_ig__expbdarg1_dn4_slot = var_fn482_calc_ig__expbdarg1_dn4;
        *var_fn482_calc_ig__expbdarg1_dn8_slot = var_fn482_calc_ig__expbdarg1_dn8;
        *var_fn482_calc_ig__expbdarg2_slot = var_fn482_calc_ig__expbdarg2;
        *var_fn482_calc_ig__expbdarg2_dn4_slot = var_fn482_calc_ig__expbdarg2_dn4;
        *var_fn482_calc_ig__expffvarg_slot = var_fn482_calc_ig__expffvarg;
        *var_fn482_calc_ig__expffvarg_dn0_slot = var_fn482_calc_ig__expffvarg_dn0;
        *var_fn482_calc_ig__expffvarg_dn18_slot = var_fn482_calc_ig__expffvarg_dn18;
        *var_fn482_calc_ig__expffvarg_dn19_slot = var_fn482_calc_ig__expffvarg_dn19;
        *var_fn482_calc_ig__expffvarg_dn2_slot = var_fn482_calc_ig__expffvarg_dn2;
        *var_fn482_calc_ig__expffvarg_dn4_slot = var_fn482_calc_ig__expffvarg_dn4;
        *var_fn482_calc_ig__expffvarg_dn8_slot = var_fn482_calc_ig__expffvarg_dn8;
        *var_fn482_calc_ig__expifor_slot = var_fn482_calc_ig__expifor;
        *var_fn482_calc_ig__expifor_dn0_slot = var_fn482_calc_ig__expifor_dn0;
        *var_fn482_calc_ig__expifor_dn18_slot = var_fn482_calc_ig__expifor_dn18;
        *var_fn482_calc_ig__expifor_dn19_slot = var_fn482_calc_ig__expifor_dn19;
        *var_fn482_calc_ig__expifor_dn2_slot = var_fn482_calc_ig__expifor_dn2;
        *var_fn482_calc_ig__expifor_dn4_slot = var_fn482_calc_ig__expifor_dn4;
        *var_fn482_calc_ig__expifor_dn8_slot = var_fn482_calc_ig__expifor_dn8;
        *var_fn482_calc_ig__expiforarg_slot = var_fn482_calc_ig__expiforarg;
        *var_fn482_calc_ig__expiforarg_dn0_slot = var_fn482_calc_ig__expiforarg_dn0;
        *var_fn482_calc_ig__expiforarg_dn18_slot = var_fn482_calc_ig__expiforarg_dn18;
        *var_fn482_calc_ig__expiforarg_dn19_slot = var_fn482_calc_ig__expiforarg_dn19;
        *var_fn482_calc_ig__expiforarg_dn2_slot = var_fn482_calc_ig__expiforarg_dn2;
        *var_fn482_calc_ig__expiforarg_dn4_slot = var_fn482_calc_ig__expiforarg_dn4;
        *var_fn482_calc_ig__expiforarg_dn8_slot = var_fn482_calc_ig__expiforarg_dn8;
        *var_fn482_calc_ig__expirev_slot = var_fn482_calc_ig__expirev;
        *var_fn482_calc_ig__expirev_dn0_slot = var_fn482_calc_ig__expirev_dn0;
        *var_fn482_calc_ig__expirev_dn18_slot = var_fn482_calc_ig__expirev_dn18;
        *var_fn482_calc_ig__expirev_dn19_slot = var_fn482_calc_ig__expirev_dn19;
        *var_fn482_calc_ig__expirev_dn2_slot = var_fn482_calc_ig__expirev_dn2;
        *var_fn482_calc_ig__expirev_dn4_slot = var_fn482_calc_ig__expirev_dn4;
        *var_fn482_calc_ig__expirev_dn8_slot = var_fn482_calc_ig__expirev_dn8;
        *var_fn482_calc_ig__expirevarg_slot = var_fn482_calc_ig__expirevarg;
        *var_fn482_calc_ig__expirevarg_dn0_slot = var_fn482_calc_ig__expirevarg_dn0;
        *var_fn482_calc_ig__expirevarg_dn18_slot = var_fn482_calc_ig__expirevarg_dn18;
        *var_fn482_calc_ig__expirevarg_dn19_slot = var_fn482_calc_ig__expirevarg_dn19;
        *var_fn482_calc_ig__expirevarg_dn2_slot = var_fn482_calc_ig__expirevarg_dn2;
        *var_fn482_calc_ig__expirevarg_dn4_slot = var_fn482_calc_ig__expirevarg_dn4;
        *var_fn482_calc_ig__expirevarg_dn8_slot = var_fn482_calc_ig__expirevarg_dn8;
        *var_fn482_calc_ig__expphib_slot = var_fn482_calc_ig__expphib;
        *var_fn482_calc_ig__expphib_dn4_slot = var_fn482_calc_ig__expphib_dn4;
        *var_fn482_calc_ig__ffvgin_slot = var_fn482_calc_ig__ffvgin;
        *var_fn482_calc_ig__ffvgin_dn0_slot = var_fn482_calc_ig__ffvgin_dn0;
        *var_fn482_calc_ig__ffvgin_dn18_slot = var_fn482_calc_ig__ffvgin_dn18;
        *var_fn482_calc_ig__ffvgin_dn19_slot = var_fn482_calc_ig__ffvgin_dn19;
        *var_fn482_calc_ig__ffvgin_dn2_slot = var_fn482_calc_ig__ffvgin_dn2;
        *var_fn482_calc_ig__ffvgin_dn4_slot = var_fn482_calc_ig__ffvgin_dn4;
        *var_fn482_calc_ig__ffvgin_dn8_slot = var_fn482_calc_ig__ffvgin_dn8;
        *var_fn482_calc_ig__fracin_slot = var_fn482_calc_ig__fracin;
        *var_fn482_calc_ig__frecgin_slot = var_fn482_calc_ig__frecgin;
        *var_fn482_calc_ig__frecgin_dn0_slot = var_fn482_calc_ig__frecgin_dn0;
        *var_fn482_calc_ig__frecgin_dn18_slot = var_fn482_calc_ig__frecgin_dn18;
        *var_fn482_calc_ig__frecgin_dn19_slot = var_fn482_calc_ig__frecgin_dn19;
        *var_fn482_calc_ig__frecgin_dn2_slot = var_fn482_calc_ig__frecgin_dn2;
        *var_fn482_calc_ig__frecgin_dn8_slot = var_fn482_calc_ig__frecgin_dn8;
        *var_fn482_calc_ig__iginbd_slot = var_fn482_calc_ig__iginbd;
        *var_fn482_calc_ig__iginbd_dn0_slot = var_fn482_calc_ig__iginbd_dn0;
        *var_fn482_calc_ig__iginbd_dn18_slot = var_fn482_calc_ig__iginbd_dn18;
        *var_fn482_calc_ig__iginbd_dn19_slot = var_fn482_calc_ig__iginbd_dn19;
        *var_fn482_calc_ig__iginbd_dn2_slot = var_fn482_calc_ig__iginbd_dn2;
        *var_fn482_calc_ig__iginbd_dn4_slot = var_fn482_calc_ig__iginbd_dn4;
        *var_fn482_calc_ig__iginbd_dn8_slot = var_fn482_calc_ig__iginbd_dn8;
        *var_fn482_calc_ig__igindiode_slot = var_fn482_calc_ig__igindiode;
        *var_fn482_calc_ig__igindiode_dn0_slot = var_fn482_calc_ig__igindiode_dn0;
        *var_fn482_calc_ig__igindiode_dn18_slot = var_fn482_calc_ig__igindiode_dn18;
        *var_fn482_calc_ig__igindiode_dn19_slot = var_fn482_calc_ig__igindiode_dn19;
        *var_fn482_calc_ig__igindiode_dn2_slot = var_fn482_calc_ig__igindiode_dn2;
        *var_fn482_calc_ig__igindiode_dn4_slot = var_fn482_calc_ig__igindiode_dn4;
        *var_fn482_calc_ig__igindiode_dn8_slot = var_fn482_calc_ig__igindiode_dn8;
        *var_fn482_calc_ig__iginrec_slot = var_fn482_calc_ig__iginrec;
        *var_fn482_calc_ig__iginrec_dn0_slot = var_fn482_calc_ig__iginrec_dn0;
        *var_fn482_calc_ig__iginrec_dn18_slot = var_fn482_calc_ig__iginrec_dn18;
        *var_fn482_calc_ig__iginrec_dn19_slot = var_fn482_calc_ig__iginrec_dn19;
        *var_fn482_calc_ig__iginrec_dn2_slot = var_fn482_calc_ig__iginrec_dn2;
        *var_fn482_calc_ig__iginrec_dn4_slot = var_fn482_calc_ig__iginrec_dn4;
        *var_fn482_calc_ig__iginrec_dn8_slot = var_fn482_calc_ig__iginrec_dn8;
        *var_fn482_calc_ig__igout_slot = var_fn482_calc_ig__igout;
        *var_fn482_calc_ig__igout_dn0_slot = var_fn482_calc_ig__igout_dn0;
        *var_fn482_calc_ig__igout_dn18_slot = var_fn482_calc_ig__igout_dn18;
        *var_fn482_calc_ig__igout_dn19_slot = var_fn482_calc_ig__igout_dn19;
        *var_fn482_calc_ig__igout_dn2_slot = var_fn482_calc_ig__igout_dn2;
        *var_fn482_calc_ig__igout_dn4_slot = var_fn482_calc_ig__igout_dn4;
        *var_fn482_calc_ig__igout_dn8_slot = var_fn482_calc_ig__igout_dn8;
        *var_fn482_calc_ig__ijin_slot = var_fn482_calc_ig__ijin;
        *var_fn482_calc_ig__irecin_slot = var_fn482_calc_ig__irecin;
        *var_fn482_calc_ig__isdiodeout_slot = var_fn482_calc_ig__isdiodeout;
        *var_fn482_calc_ig__isdiodeout_dn4_slot = var_fn482_calc_ig__isdiodeout_dn4;
        *var_fn482_calc_ig__isrecout_slot = var_fn482_calc_ig__isrecout;
        *var_fn482_calc_ig__isrecout_dn4_slot = var_fn482_calc_ig__isrecout_dn4;
        *var_fn482_calc_ig__kbdgatein_slot = var_fn482_calc_ig__kbdgatein;
        *var_fn482_calc_ig__ngf_slot = var_fn482_calc_ig__ngf;
        *var_fn482_calc_ig__pbdgin_slot = var_fn482_calc_ig__pbdgin;
        *var_fn482_calc_ig__pg_param1_slot = var_fn482_calc_ig__pg_param1;
        *var_fn482_calc_ig__pg_paramin_slot = var_fn482_calc_ig__pg_paramin;
        *var_fn482_calc_ig__pgsrecin_slot = var_fn482_calc_ig__pgsrecin;
        *var_fn482_calc_ig__phitin_slot = var_fn482_calc_ig__phitin;
        *var_fn482_calc_ig__phitin_dn4_slot = var_fn482_calc_ig__phitin_dn4;
        *var_fn482_calc_ig__return_slot = var_fn482_calc_ig__return;
        *var_fn482_calc_ig__return_dn0_slot = var_fn482_calc_ig__return_dn0;
        *var_fn482_calc_ig__return_dn18_slot = var_fn482_calc_ig__return_dn18;
        *var_fn482_calc_ig__return_dn19_slot = var_fn482_calc_ig__return_dn19;
        *var_fn482_calc_ig__return_dn2_slot = var_fn482_calc_ig__return_dn2;
        *var_fn482_calc_ig__return_dn4_slot = var_fn482_calc_ig__return_dn4;
        *var_fn482_calc_ig__return_dn8_slot = var_fn482_calc_ig__return_dn8;
        *var_fn482_calc_ig__t0_slot = var_fn482_calc_ig__t0;
        *var_fn482_calc_ig__t0_dn4_slot = var_fn482_calc_ig__t0_dn4;
        *var_fn482_calc_ig__tfacdiodein_slot = var_fn482_calc_ig__tfacdiodein;
        *var_fn482_calc_ig__tfacdiodein_dn4_slot = var_fn482_calc_ig__tfacdiodein_dn4;
        *var_fn482_calc_ig__type_slot = var_fn482_calc_ig__type;
        *var_fn482_calc_ig__vbdgin_slot = var_fn482_calc_ig__vbdgin;
        *var_fn482_calc_ig__vgin_slot = var_fn482_calc_ig__vgin;
        *var_fn482_calc_ig__vgin_dn0_slot = var_fn482_calc_ig__vgin_dn0;
        *var_fn482_calc_ig__vgin_dn18_slot = var_fn482_calc_ig__vgin_dn18;
        *var_fn482_calc_ig__vgin_dn19_slot = var_fn482_calc_ig__vgin_dn19;
        *var_fn482_calc_ig__vgin_dn2_slot = var_fn482_calc_ig__vgin_dn2;
        *var_fn482_calc_ig__vgin_dn8_slot = var_fn482_calc_ig__vgin_dn8;
        *var_fn482_calc_ig__vgsatin_slot = var_fn482_calc_ig__vgsatin;
        *var_fn482_calc_ig__vgsatqin_slot = var_fn482_calc_ig__vgsatqin;
        *var_fn482_calc_ig__vjg_slot = var_fn482_calc_ig__vjg;
        *var_fn482_calc_ig__w_slot = var_fn482_calc_ig__w;
        *var_guard480_slot = var_guard480;
        *var_guard481_slot = var_guard481;
        *var_vindcbd_slot = var_vindcbd;
        *var_vindcbd_dn0_slot = var_vindcbd_dn0;
        *var_vindcbd_dn18_slot = var_vindcbd_dn18;
        *var_vindcbd_dn19_slot = var_vindcbd_dn19;
        *var_vindcbd_dn2_slot = var_vindcbd_dn2;
        *var_vindcbd_dn8_slot = var_vindcbd_dn8;
        *var_vinscbd_slot = var_vinscbd;
        *var_vinscbd_dn0_slot = var_vinscbd_dn0;
        *var_vinscbd_dn18_slot = var_vinscbd_dn18;
        *var_vinscbd_dn19_slot = var_vinscbd_dn19;
        *var_vinscbd_dn2_slot = var_vinscbd_dn2;
        *var_vinscbd_dn8_slot = var_vinscbd_dn8;
    }

    pub(super) fn stamp_transient_block_111(
        var_fn482_calc_ig__fracin: f64,
        var_fn482_calc_ig__ijin: f64,
        var_fn482_calc_ig__kbdgatein: f64,
        var_fn482_calc_ig__ngf: f64,
        var_fn482_calc_ig__pbdgin: f64,
        var_fn482_calc_ig__pg_param1: f64,
        var_fn482_calc_ig__pg_paramin: f64,
        var_fn482_calc_ig__phitin: f64,
        var_fn482_calc_ig__phitin_dn4: f64,
        var_fn482_calc_ig__tfacdiodein: f64,
        var_fn482_calc_ig__tfacdiodein_dn4: f64,
        var_fn482_calc_ig__type: f64,
        var_fn482_calc_ig__vbdgin: f64,
        var_fn482_calc_ig__vgin: f64,
        var_fn482_calc_ig__vgin_dn0: f64,
        var_fn482_calc_ig__vgin_dn18: f64,
        var_fn482_calc_ig__vgin_dn19: f64,
        var_fn482_calc_ig__vgin_dn2: f64,
        var_fn482_calc_ig__vgin_dn8: f64,
        var_fn482_calc_ig__vgsatin: f64,
        var_fn482_calc_ig__vjg: f64,
        var_fn482_calc_ig__w: f64,
        var_guard480: f64,
        var_fn482_calc_ig__expbd1_slot: &mut f64,
        var_fn482_calc_ig__expbd1_dn0_slot: &mut f64,
        var_fn482_calc_ig__expbd1_dn18_slot: &mut f64,
        var_fn482_calc_ig__expbd1_dn19_slot: &mut f64,
        var_fn482_calc_ig__expbd1_dn2_slot: &mut f64,
        var_fn482_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn482_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn482_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn482_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn482_calc_ig__expbd2_slot: &mut f64,
        var_fn482_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn482_calc_ig__expbdarg1_slot: &mut f64,
        var_fn482_calc_ig__expbdarg1_dn0_slot: &mut f64,
        var_fn482_calc_ig__expbdarg1_dn18_slot: &mut f64,
        var_fn482_calc_ig__expbdarg1_dn19_slot: &mut f64,
        var_fn482_calc_ig__expbdarg1_dn2_slot: &mut f64,
        var_fn482_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn482_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn482_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn482_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn482_calc_ig__expbdarg2_slot: &mut f64,
        var_fn482_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn482_calc_ig__expifor_slot: &mut f64,
        var_fn482_calc_ig__expifor_dn0_slot: &mut f64,
        var_fn482_calc_ig__expifor_dn18_slot: &mut f64,
        var_fn482_calc_ig__expifor_dn19_slot: &mut f64,
        var_fn482_calc_ig__expifor_dn2_slot: &mut f64,
        var_fn482_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn482_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn482_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn482_calc_ig__expifor_hinj_dn0_slot: &mut f64,
        var_fn482_calc_ig__expifor_hinj_dn18_slot: &mut f64,
        var_fn482_calc_ig__expifor_hinj_dn19_slot: &mut f64,
        var_fn482_calc_ig__expifor_hinj_dn2_slot: &mut f64,
        var_fn482_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn482_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn482_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn482_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn482_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn482_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_dn0_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_dn18_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_dn19_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_dn2_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_hinj_dn0_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_hinj_dn18_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_hinj_dn19_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_hinj_dn2_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn482_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn482_calc_ig__expphib_slot: &mut f64,
        var_fn482_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn482_calc_ig__iginbd_slot: &mut f64,
        var_fn482_calc_ig__iginbd_dn0_slot: &mut f64,
        var_fn482_calc_ig__iginbd_dn18_slot: &mut f64,
        var_fn482_calc_ig__iginbd_dn19_slot: &mut f64,
        var_fn482_calc_ig__iginbd_dn2_slot: &mut f64,
        var_fn482_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn482_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn482_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn482_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn482_calc_ig__igindiode_slot: &mut f64,
        var_fn482_calc_ig__igindiode_dn0_slot: &mut f64,
        var_fn482_calc_ig__igindiode_dn18_slot: &mut f64,
        var_fn482_calc_ig__igindiode_dn19_slot: &mut f64,
        var_fn482_calc_ig__igindiode_dn2_slot: &mut f64,
        var_fn482_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn482_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn482_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn482_calc_ig__igindiode_hinj_dn0_slot: &mut f64,
        var_fn482_calc_ig__igindiode_hinj_dn18_slot: &mut f64,
        var_fn482_calc_ig__igindiode_hinj_dn19_slot: &mut f64,
        var_fn482_calc_ig__igindiode_hinj_dn2_slot: &mut f64,
        var_fn482_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn482_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn482_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn482_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn482_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn482_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn482_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn482_calc_ig__igindiode_nohinj_dn0_slot: &mut f64,
        var_fn482_calc_ig__igindiode_nohinj_dn18_slot: &mut f64,
        var_fn482_calc_ig__igindiode_nohinj_dn19_slot: &mut f64,
        var_fn482_calc_ig__igindiode_nohinj_dn2_slot: &mut f64,
        var_fn482_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn482_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn482_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn482_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn482_calc_ig__isdiodeout_slot: &mut f64,
        var_fn482_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn482_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_fn482_calc_ig__t0_slot: &mut f64,
        var_fn482_calc_ig__t0_dn4_slot: &mut f64,
        var_guard483_slot: &mut f64,
    ) {
        let mut var_fn482_calc_ig__expbd1: f64 = *var_fn482_calc_ig__expbd1_slot;
        let mut var_fn482_calc_ig__expbd1_dn0: f64 = *var_fn482_calc_ig__expbd1_dn0_slot;
        let mut var_fn482_calc_ig__expbd1_dn18: f64 = *var_fn482_calc_ig__expbd1_dn18_slot;
        let mut var_fn482_calc_ig__expbd1_dn19: f64 = *var_fn482_calc_ig__expbd1_dn19_slot;
        let mut var_fn482_calc_ig__expbd1_dn2: f64 = *var_fn482_calc_ig__expbd1_dn2_slot;
        let mut var_fn482_calc_ig__expbd1_dn4: f64 = *var_fn482_calc_ig__expbd1_dn4_slot;
        let mut var_fn482_calc_ig__expbd1_dn8: f64 = *var_fn482_calc_ig__expbd1_dn8_slot;
        let mut var_fn482_calc_ig__expbd1_vgsat: f64 = *var_fn482_calc_ig__expbd1_vgsat_slot;
        let mut var_fn482_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn482_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn482_calc_ig__expbd2: f64 = *var_fn482_calc_ig__expbd2_slot;
        let mut var_fn482_calc_ig__expbd2_dn4: f64 = *var_fn482_calc_ig__expbd2_dn4_slot;
        let mut var_fn482_calc_ig__expbdarg1: f64 = *var_fn482_calc_ig__expbdarg1_slot;
        let mut var_fn482_calc_ig__expbdarg1_dn0: f64 = *var_fn482_calc_ig__expbdarg1_dn0_slot;
        let mut var_fn482_calc_ig__expbdarg1_dn18: f64 = *var_fn482_calc_ig__expbdarg1_dn18_slot;
        let mut var_fn482_calc_ig__expbdarg1_dn19: f64 = *var_fn482_calc_ig__expbdarg1_dn19_slot;
        let mut var_fn482_calc_ig__expbdarg1_dn2: f64 = *var_fn482_calc_ig__expbdarg1_dn2_slot;
        let mut var_fn482_calc_ig__expbdarg1_dn4: f64 = *var_fn482_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn482_calc_ig__expbdarg1_dn8: f64 = *var_fn482_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn482_calc_ig__expbdarg1_vgsat: f64 = *var_fn482_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn482_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn482_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn482_calc_ig__expbdarg2: f64 = *var_fn482_calc_ig__expbdarg2_slot;
        let mut var_fn482_calc_ig__expbdarg2_dn4: f64 = *var_fn482_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn482_calc_ig__expifor: f64 = *var_fn482_calc_ig__expifor_slot;
        let mut var_fn482_calc_ig__expifor_dn0: f64 = *var_fn482_calc_ig__expifor_dn0_slot;
        let mut var_fn482_calc_ig__expifor_dn18: f64 = *var_fn482_calc_ig__expifor_dn18_slot;
        let mut var_fn482_calc_ig__expifor_dn19: f64 = *var_fn482_calc_ig__expifor_dn19_slot;
        let mut var_fn482_calc_ig__expifor_dn2: f64 = *var_fn482_calc_ig__expifor_dn2_slot;
        let mut var_fn482_calc_ig__expifor_dn4: f64 = *var_fn482_calc_ig__expifor_dn4_slot;
        let mut var_fn482_calc_ig__expifor_dn8: f64 = *var_fn482_calc_ig__expifor_dn8_slot;
        let mut var_fn482_calc_ig__expifor_hinj: f64 = *var_fn482_calc_ig__expifor_hinj_slot;
        let mut var_fn482_calc_ig__expifor_hinj_dn0: f64 = *var_fn482_calc_ig__expifor_hinj_dn0_slot;
        let mut var_fn482_calc_ig__expifor_hinj_dn18: f64 = *var_fn482_calc_ig__expifor_hinj_dn18_slot;
        let mut var_fn482_calc_ig__expifor_hinj_dn19: f64 = *var_fn482_calc_ig__expifor_hinj_dn19_slot;
        let mut var_fn482_calc_ig__expifor_hinj_dn2: f64 = *var_fn482_calc_ig__expifor_hinj_dn2_slot;
        let mut var_fn482_calc_ig__expifor_hinj_dn4: f64 = *var_fn482_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn482_calc_ig__expifor_hinj_dn8: f64 = *var_fn482_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn482_calc_ig__expifor_hinj_vgsat: f64 = *var_fn482_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn482_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn482_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn482_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn482_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn482_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn482_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn482_calc_ig__expiforarg: f64 = *var_fn482_calc_ig__expiforarg_slot;
        let mut var_fn482_calc_ig__expiforarg_dn0: f64 = *var_fn482_calc_ig__expiforarg_dn0_slot;
        let mut var_fn482_calc_ig__expiforarg_dn18: f64 = *var_fn482_calc_ig__expiforarg_dn18_slot;
        let mut var_fn482_calc_ig__expiforarg_dn19: f64 = *var_fn482_calc_ig__expiforarg_dn19_slot;
        let mut var_fn482_calc_ig__expiforarg_dn2: f64 = *var_fn482_calc_ig__expiforarg_dn2_slot;
        let mut var_fn482_calc_ig__expiforarg_dn4: f64 = *var_fn482_calc_ig__expiforarg_dn4_slot;
        let mut var_fn482_calc_ig__expiforarg_dn8: f64 = *var_fn482_calc_ig__expiforarg_dn8_slot;
        let mut var_fn482_calc_ig__expiforarg_hinj: f64 = *var_fn482_calc_ig__expiforarg_hinj_slot;
        let mut var_fn482_calc_ig__expiforarg_hinj_dn0: f64 = *var_fn482_calc_ig__expiforarg_hinj_dn0_slot;
        let mut var_fn482_calc_ig__expiforarg_hinj_dn18: f64 = *var_fn482_calc_ig__expiforarg_hinj_dn18_slot;
        let mut var_fn482_calc_ig__expiforarg_hinj_dn19: f64 = *var_fn482_calc_ig__expiforarg_hinj_dn19_slot;
        let mut var_fn482_calc_ig__expiforarg_hinj_dn2: f64 = *var_fn482_calc_ig__expiforarg_hinj_dn2_slot;
        let mut var_fn482_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn482_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn482_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn482_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn482_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn482_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn482_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn482_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn482_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn482_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn482_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn482_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn482_calc_ig__expphib: f64 = *var_fn482_calc_ig__expphib_slot;
        let mut var_fn482_calc_ig__expphib_dn4: f64 = *var_fn482_calc_ig__expphib_dn4_slot;
        let mut var_fn482_calc_ig__iginbd: f64 = *var_fn482_calc_ig__iginbd_slot;
        let mut var_fn482_calc_ig__iginbd_dn0: f64 = *var_fn482_calc_ig__iginbd_dn0_slot;
        let mut var_fn482_calc_ig__iginbd_dn18: f64 = *var_fn482_calc_ig__iginbd_dn18_slot;
        let mut var_fn482_calc_ig__iginbd_dn19: f64 = *var_fn482_calc_ig__iginbd_dn19_slot;
        let mut var_fn482_calc_ig__iginbd_dn2: f64 = *var_fn482_calc_ig__iginbd_dn2_slot;
        let mut var_fn482_calc_ig__iginbd_dn4: f64 = *var_fn482_calc_ig__iginbd_dn4_slot;
        let mut var_fn482_calc_ig__iginbd_dn8: f64 = *var_fn482_calc_ig__iginbd_dn8_slot;
        let mut var_fn482_calc_ig__iginbd_vgsat: f64 = *var_fn482_calc_ig__iginbd_vgsat_slot;
        let mut var_fn482_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn482_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn482_calc_ig__igindiode: f64 = *var_fn482_calc_ig__igindiode_slot;
        let mut var_fn482_calc_ig__igindiode_dn0: f64 = *var_fn482_calc_ig__igindiode_dn0_slot;
        let mut var_fn482_calc_ig__igindiode_dn18: f64 = *var_fn482_calc_ig__igindiode_dn18_slot;
        let mut var_fn482_calc_ig__igindiode_dn19: f64 = *var_fn482_calc_ig__igindiode_dn19_slot;
        let mut var_fn482_calc_ig__igindiode_dn2: f64 = *var_fn482_calc_ig__igindiode_dn2_slot;
        let mut var_fn482_calc_ig__igindiode_dn4: f64 = *var_fn482_calc_ig__igindiode_dn4_slot;
        let mut var_fn482_calc_ig__igindiode_dn8: f64 = *var_fn482_calc_ig__igindiode_dn8_slot;
        let mut var_fn482_calc_ig__igindiode_hinj: f64 = *var_fn482_calc_ig__igindiode_hinj_slot;
        let mut var_fn482_calc_ig__igindiode_hinj_dn0: f64 = *var_fn482_calc_ig__igindiode_hinj_dn0_slot;
        let mut var_fn482_calc_ig__igindiode_hinj_dn18: f64 = *var_fn482_calc_ig__igindiode_hinj_dn18_slot;
        let mut var_fn482_calc_ig__igindiode_hinj_dn19: f64 = *var_fn482_calc_ig__igindiode_hinj_dn19_slot;
        let mut var_fn482_calc_ig__igindiode_hinj_dn2: f64 = *var_fn482_calc_ig__igindiode_hinj_dn2_slot;
        let mut var_fn482_calc_ig__igindiode_hinj_dn4: f64 = *var_fn482_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn482_calc_ig__igindiode_hinj_dn8: f64 = *var_fn482_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn482_calc_ig__igindiode_hinj_pre: f64 = *var_fn482_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn482_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn482_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn482_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn482_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn482_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn482_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn482_calc_ig__igindiode_nohinj: f64 = *var_fn482_calc_ig__igindiode_nohinj_slot;
        let mut var_fn482_calc_ig__igindiode_nohinj_dn0: f64 = *var_fn482_calc_ig__igindiode_nohinj_dn0_slot;
        let mut var_fn482_calc_ig__igindiode_nohinj_dn18: f64 = *var_fn482_calc_ig__igindiode_nohinj_dn18_slot;
        let mut var_fn482_calc_ig__igindiode_nohinj_dn19: f64 = *var_fn482_calc_ig__igindiode_nohinj_dn19_slot;
        let mut var_fn482_calc_ig__igindiode_nohinj_dn2: f64 = *var_fn482_calc_ig__igindiode_nohinj_dn2_slot;
        let mut var_fn482_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn482_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn482_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn482_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn482_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn482_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn482_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn482_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn482_calc_ig__isdiodeout: f64 = *var_fn482_calc_ig__isdiodeout_slot;
        let mut var_fn482_calc_ig__isdiodeout_dn4: f64 = *var_fn482_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn482_calc_ig__pg_paramin_hinj: f64 = *var_fn482_calc_ig__pg_paramin_hinj_slot;
        let mut var_fn482_calc_ig__t0: f64 = *var_fn482_calc_ig__t0_slot;
        let mut var_fn482_calc_ig__t0_dn4: f64 = *var_fn482_calc_ig__t0_dn4_slot;
        let mut var_guard483: f64 = *var_guard483_slot;

        let (assign44460_e42858,) = {
    if (var_guard480 != 0.0) {
        (0.0,)
    } else {
        (var_fn482_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn482_calc_ig__pg_paramin_hinj = assign44460_e42858;

        let (assign44470_e42862, assign44470_e42862_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expbdarg1_vgsat, var_fn482_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__expbdarg1_vgsat = assign44470_e42862;
        var_fn482_calc_ig__expbdarg1_vgsat_dn4 = assign44470_e42862_d_n4;

        let (assign44480_e42866, assign44480_e42866_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expbd1_vgsat, var_fn482_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__expbd1_vgsat = assign44480_e42866;
        var_fn482_calc_ig__expbd1_vgsat_dn4 = assign44480_e42866_d_n4;

        let (assign44490_e42870, assign44490_e42870_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__iginbd_vgsat, var_fn482_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__iginbd_vgsat = assign44490_e42870;
        var_fn482_calc_ig__iginbd_vgsat_dn4 = assign44490_e42870_d_n4;

        let (assign44500_e42874, assign44500_e42874_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expiforarg_nohinj_vgsat, var_fn482_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__expiforarg_nohinj_vgsat = assign44500_e42874;
        var_fn482_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign44500_e42874_d_n4;

        let (assign44510_e42878, assign44510_e42878_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expifor_nohinj_vgsat, var_fn482_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__expifor_nohinj_vgsat = assign44510_e42878;
        var_fn482_calc_ig__expifor_nohinj_vgsat_dn4 = assign44510_e42878_d_n4;

        let (assign44520_e42882, assign44520_e42882_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__igindiode_nohinj_vgsat, var_fn482_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__igindiode_nohinj_vgsat = assign44520_e42882;
        var_fn482_calc_ig__igindiode_nohinj_vgsat_dn4 = assign44520_e42882_d_n4;

        let (assign44530_e42886, assign44530_e42886_d_n0, assign44530_e42886_d_n2, assign44530_e42886_d_n4, assign44530_e42886_d_n8, assign44530_e42886_d_n18, assign44530_e42886_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__igindiode_nohinj, var_fn482_calc_ig__igindiode_nohinj_dn0, var_fn482_calc_ig__igindiode_nohinj_dn2, var_fn482_calc_ig__igindiode_nohinj_dn4, var_fn482_calc_ig__igindiode_nohinj_dn8, var_fn482_calc_ig__igindiode_nohinj_dn18, var_fn482_calc_ig__igindiode_nohinj_dn19,)
    }
};
        var_fn482_calc_ig__igindiode_nohinj = assign44530_e42886;
        var_fn482_calc_ig__igindiode_nohinj_dn0 = assign44530_e42886_d_n0;
        var_fn482_calc_ig__igindiode_nohinj_dn2 = assign44530_e42886_d_n2;
        var_fn482_calc_ig__igindiode_nohinj_dn4 = assign44530_e42886_d_n4;
        var_fn482_calc_ig__igindiode_nohinj_dn8 = assign44530_e42886_d_n8;
        var_fn482_calc_ig__igindiode_nohinj_dn18 = assign44530_e42886_d_n18;
        var_fn482_calc_ig__igindiode_nohinj_dn19 = assign44530_e42886_d_n19;

        let (assign44540_e42890, assign44540_e42890_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expiforarg_hinj_vgsat, var_fn482_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__expiforarg_hinj_vgsat = assign44540_e42890;
        var_fn482_calc_ig__expiforarg_hinj_vgsat_dn4 = assign44540_e42890_d_n4;

        let (assign44550_e42894, assign44550_e42894_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expifor_hinj_vgsat, var_fn482_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__expifor_hinj_vgsat = assign44550_e42894;
        var_fn482_calc_ig__expifor_hinj_vgsat_dn4 = assign44550_e42894_d_n4;

        let (assign44560_e42898, assign44560_e42898_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__igindiode_hinj_vgsat, var_fn482_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__igindiode_hinj_vgsat = assign44560_e42898;
        var_fn482_calc_ig__igindiode_hinj_vgsat_dn4 = assign44560_e42898_d_n4;

        let (assign44570_e42902, assign44570_e42902_d_n0, assign44570_e42902_d_n2, assign44570_e42902_d_n4, assign44570_e42902_d_n8, assign44570_e42902_d_n18, assign44570_e42902_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expiforarg_hinj, var_fn482_calc_ig__expiforarg_hinj_dn0, var_fn482_calc_ig__expiforarg_hinj_dn2, var_fn482_calc_ig__expiforarg_hinj_dn4, var_fn482_calc_ig__expiforarg_hinj_dn8, var_fn482_calc_ig__expiforarg_hinj_dn18, var_fn482_calc_ig__expiforarg_hinj_dn19,)
    }
};
        var_fn482_calc_ig__expiforarg_hinj = assign44570_e42902;
        var_fn482_calc_ig__expiforarg_hinj_dn0 = assign44570_e42902_d_n0;
        var_fn482_calc_ig__expiforarg_hinj_dn2 = assign44570_e42902_d_n2;
        var_fn482_calc_ig__expiforarg_hinj_dn4 = assign44570_e42902_d_n4;
        var_fn482_calc_ig__expiforarg_hinj_dn8 = assign44570_e42902_d_n8;
        var_fn482_calc_ig__expiforarg_hinj_dn18 = assign44570_e42902_d_n18;
        var_fn482_calc_ig__expiforarg_hinj_dn19 = assign44570_e42902_d_n19;

        let (assign44580_e42906, assign44580_e42906_d_n0, assign44580_e42906_d_n2, assign44580_e42906_d_n4, assign44580_e42906_d_n8, assign44580_e42906_d_n18, assign44580_e42906_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__expifor_hinj, var_fn482_calc_ig__expifor_hinj_dn0, var_fn482_calc_ig__expifor_hinj_dn2, var_fn482_calc_ig__expifor_hinj_dn4, var_fn482_calc_ig__expifor_hinj_dn8, var_fn482_calc_ig__expifor_hinj_dn18, var_fn482_calc_ig__expifor_hinj_dn19,)
    }
};
        var_fn482_calc_ig__expifor_hinj = assign44580_e42906;
        var_fn482_calc_ig__expifor_hinj_dn0 = assign44580_e42906_d_n0;
        var_fn482_calc_ig__expifor_hinj_dn2 = assign44580_e42906_d_n2;
        var_fn482_calc_ig__expifor_hinj_dn4 = assign44580_e42906_d_n4;
        var_fn482_calc_ig__expifor_hinj_dn8 = assign44580_e42906_d_n8;
        var_fn482_calc_ig__expifor_hinj_dn18 = assign44580_e42906_d_n18;
        var_fn482_calc_ig__expifor_hinj_dn19 = assign44580_e42906_d_n19;

        let (assign44590_e42910, assign44590_e42910_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__igindiode_hinj_pre, var_fn482_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn482_calc_ig__igindiode_hinj_pre = assign44590_e42910;
        var_fn482_calc_ig__igindiode_hinj_pre_dn4 = assign44590_e42910_d_n4;

        let (assign44600_e42914, assign44600_e42914_d_n0, assign44600_e42914_d_n2, assign44600_e42914_d_n4, assign44600_e42914_d_n8, assign44600_e42914_d_n18, assign44600_e42914_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__igindiode_hinj, var_fn482_calc_ig__igindiode_hinj_dn0, var_fn482_calc_ig__igindiode_hinj_dn2, var_fn482_calc_ig__igindiode_hinj_dn4, var_fn482_calc_ig__igindiode_hinj_dn8, var_fn482_calc_ig__igindiode_hinj_dn18, var_fn482_calc_ig__igindiode_hinj_dn19,)
    }
};
        var_fn482_calc_ig__igindiode_hinj = assign44600_e42914;
        var_fn482_calc_ig__igindiode_hinj_dn0 = assign44600_e42914_d_n0;
        var_fn482_calc_ig__igindiode_hinj_dn2 = assign44600_e42914_d_n2;
        var_fn482_calc_ig__igindiode_hinj_dn4 = assign44600_e42914_d_n4;
        var_fn482_calc_ig__igindiode_hinj_dn8 = assign44600_e42914_d_n8;
        var_fn482_calc_ig__igindiode_hinj_dn18 = assign44600_e42914_d_n18;
        var_fn482_calc_ig__igindiode_hinj_dn19 = assign44600_e42914_d_n19;

        let (assign44610_e42923, assign44610_e42923_d_n4,) = {
    if (var_guard480 != 0.0) {
        let assign44610_e42918: f64 = (var_fn482_calc_ig__pg_param1 / var_fn482_calc_ig__phitin);
        let assign44610_e42920: f64 = (-var_fn482_calc_ig__vjg);
        let assign44610_e42921: f64 = (assign44610_e42918 * assign44610_e42920);
        (assign44610_e42921, ((-((var_fn482_calc_ig__pg_param1 * var_fn482_calc_ig__phitin_dn4) / (var_fn482_calc_ig__phitin * var_fn482_calc_ig__phitin))) * assign44610_e42920),)
    } else {
        (var_fn482_calc_ig__expphib, var_fn482_calc_ig__expphib_dn4,)
    }
};
        var_fn482_calc_ig__expphib = assign44610_e42923;
        var_fn482_calc_ig__expphib_dn4 = assign44610_e42923_d_n4;

        let (assign44620_e42965, assign44620_e42965_d_n4,) = {
    if (var_guard480 != 0.0) {
        let assign44620_e42931: f64 = (-50.0);
        let (assign44620_e42963, assign44620_e42963_d_n4,) = {
            if ((!(var_fn482_calc_ig__expphib > 50.0)) && (!(var_fn482_calc_ig__expphib < assign44620_e42931))) {
                let assign44620_e42936: f64 = (var_fn482_calc_ig__expphib).exp();
                (assign44620_e42936, (assign44620_e42936 * var_fn482_calc_ig__expphib_dn4),)
            } else {
                let assign44620_e42943: f64 = (-50.0);
                let (assign44620_e42962, assign44620_e42962_d_n4,) = {
                    if ((!(var_fn482_calc_ig__expphib > 50.0)) && (var_fn482_calc_ig__expphib < assign44620_e42943)) {
                        let assign44620_e42947: f64 = (-50.0);
                        let assign44620_e42948: f64 = (assign44620_e42947).exp();
                        (assign44620_e42948, 0.0,)
                    } else {
                        let (assign44620_e42961, assign44620_e42961_d_n4,) = {
                            if (var_fn482_calc_ig__expphib > 50.0) {
                                let assign44620_e42953: f64 = (50.0_f64).exp();
                                let assign44620_e42957: f64 = (var_fn482_calc_ig__expphib - 50.0);
                                let assign44620_e42958: f64 = (1.0 + assign44620_e42957);
                                let assign44620_e42959: f64 = (assign44620_e42953 * assign44620_e42958);
                                (assign44620_e42959, (assign44620_e42953 * var_fn482_calc_ig__expphib_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign44620_e42961, assign44620_e42961_d_n4,)
                    }
                };
                (assign44620_e42962, assign44620_e42962_d_n4,)
            }
        };
        (assign44620_e42963, assign44620_e42963_d_n4,)
    } else {
        (var_fn482_calc_ig__t0, var_fn482_calc_ig__t0_dn4,)
    }
};
        var_fn482_calc_ig__t0 = assign44620_e42965;
        var_fn482_calc_ig__t0_dn4 = assign44620_e42965_d_n4;

        let (assign44630_e42976, assign44630_e42976_d_n0, assign44630_e42976_d_n2, assign44630_e42976_d_n4, assign44630_e42976_d_n8, assign44630_e42976_d_n18, assign44630_e42976_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign44630_e42969: f64 = (-var_fn482_calc_ig__vgin);
        let assign44630_e42971: f64 = (assign44630_e42969 - var_fn482_calc_ig__vbdgin);
        let assign44630_e42972: f64 = (var_fn482_calc_ig__pbdgin * assign44630_e42971);
        let assign44630_e42974: f64 = (assign44630_e42972 + var_fn482_calc_ig__expphib);
        (assign44630_e42974, (var_fn482_calc_ig__pbdgin * (-var_fn482_calc_ig__vgin_dn0)), (var_fn482_calc_ig__pbdgin * (-var_fn482_calc_ig__vgin_dn2)), var_fn482_calc_ig__expphib_dn4, (var_fn482_calc_ig__pbdgin * (-var_fn482_calc_ig__vgin_dn8)), (var_fn482_calc_ig__pbdgin * (-var_fn482_calc_ig__vgin_dn18)), (var_fn482_calc_ig__pbdgin * (-var_fn482_calc_ig__vgin_dn19)),)
    } else {
        (var_fn482_calc_ig__expbdarg1, var_fn482_calc_ig__expbdarg1_dn0, var_fn482_calc_ig__expbdarg1_dn2, var_fn482_calc_ig__expbdarg1_dn4, var_fn482_calc_ig__expbdarg1_dn8, var_fn482_calc_ig__expbdarg1_dn18, var_fn482_calc_ig__expbdarg1_dn19,)
    }
};
        var_fn482_calc_ig__expbdarg1 = assign44630_e42976;
        var_fn482_calc_ig__expbdarg1_dn0 = assign44630_e42976_d_n0;
        var_fn482_calc_ig__expbdarg1_dn2 = assign44630_e42976_d_n2;
        var_fn482_calc_ig__expbdarg1_dn4 = assign44630_e42976_d_n4;
        var_fn482_calc_ig__expbdarg1_dn8 = assign44630_e42976_d_n8;
        var_fn482_calc_ig__expbdarg1_dn18 = assign44630_e42976_d_n18;
        var_fn482_calc_ig__expbdarg1_dn19 = assign44630_e42976_d_n19;

        let (assign44640_e42985, assign44640_e42985_d_n4,) = {
    if (var_guard480 != 0.0) {
        let assign44640_e42979: f64 = (-var_fn482_calc_ig__pbdgin);
        let assign44640_e42981: f64 = (assign44640_e42979 * var_fn482_calc_ig__vbdgin);
        let assign44640_e42983: f64 = (assign44640_e42981 + var_fn482_calc_ig__expphib);
        (assign44640_e42983, var_fn482_calc_ig__expphib_dn4,)
    } else {
        (var_fn482_calc_ig__expbdarg2, var_fn482_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn482_calc_ig__expbdarg2 = assign44640_e42985;
        var_fn482_calc_ig__expbdarg2_dn4 = assign44640_e42985_d_n4;

        let (assign44650_e43027, assign44650_e43027_d_n0, assign44650_e43027_d_n2, assign44650_e43027_d_n4, assign44650_e43027_d_n8, assign44650_e43027_d_n18, assign44650_e43027_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign44650_e42993: f64 = (-50.0);
        let (assign44650_e43025, assign44650_e43025_d_n0, assign44650_e43025_d_n2, assign44650_e43025_d_n4, assign44650_e43025_d_n8, assign44650_e43025_d_n18, assign44650_e43025_d_n19,) = {
            if ((!(var_fn482_calc_ig__expbdarg1 > 50.0)) && (!(var_fn482_calc_ig__expbdarg1 < assign44650_e42993))) {
                let assign44650_e42998: f64 = (var_fn482_calc_ig__expbdarg1).exp();
                (assign44650_e42998, (assign44650_e42998 * var_fn482_calc_ig__expbdarg1_dn0), (assign44650_e42998 * var_fn482_calc_ig__expbdarg1_dn2), (assign44650_e42998 * var_fn482_calc_ig__expbdarg1_dn4), (assign44650_e42998 * var_fn482_calc_ig__expbdarg1_dn8), (assign44650_e42998 * var_fn482_calc_ig__expbdarg1_dn18), (assign44650_e42998 * var_fn482_calc_ig__expbdarg1_dn19),)
            } else {
                let assign44650_e43005: f64 = (-50.0);
                let (assign44650_e43024, assign44650_e43024_d_n0, assign44650_e43024_d_n2, assign44650_e43024_d_n4, assign44650_e43024_d_n8, assign44650_e43024_d_n18, assign44650_e43024_d_n19,) = {
                    if ((!(var_fn482_calc_ig__expbdarg1 > 50.0)) && (var_fn482_calc_ig__expbdarg1 < assign44650_e43005)) {
                        let assign44650_e43009: f64 = (-50.0);
                        let assign44650_e43010: f64 = (assign44650_e43009).exp();
                        (assign44650_e43010, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign44650_e43023, assign44650_e43023_d_n0, assign44650_e43023_d_n2, assign44650_e43023_d_n4, assign44650_e43023_d_n8, assign44650_e43023_d_n18, assign44650_e43023_d_n19,) = {
                            if (var_fn482_calc_ig__expbdarg1 > 50.0) {
                                let assign44650_e43015: f64 = (50.0_f64).exp();
                                let assign44650_e43019: f64 = (var_fn482_calc_ig__expbdarg1 - 50.0);
                                let assign44650_e43020: f64 = (1.0 + assign44650_e43019);
                                let assign44650_e43021: f64 = (assign44650_e43015 * assign44650_e43020);
                                (assign44650_e43021, (assign44650_e43015 * var_fn482_calc_ig__expbdarg1_dn0), (assign44650_e43015 * var_fn482_calc_ig__expbdarg1_dn2), (assign44650_e43015 * var_fn482_calc_ig__expbdarg1_dn4), (assign44650_e43015 * var_fn482_calc_ig__expbdarg1_dn8), (assign44650_e43015 * var_fn482_calc_ig__expbdarg1_dn18), (assign44650_e43015 * var_fn482_calc_ig__expbdarg1_dn19),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign44650_e43023, assign44650_e43023_d_n0, assign44650_e43023_d_n2, assign44650_e43023_d_n4, assign44650_e43023_d_n8, assign44650_e43023_d_n18, assign44650_e43023_d_n19,)
                    }
                };
                (assign44650_e43024, assign44650_e43024_d_n0, assign44650_e43024_d_n2, assign44650_e43024_d_n4, assign44650_e43024_d_n8, assign44650_e43024_d_n18, assign44650_e43024_d_n19,)
            }
        };
        (assign44650_e43025, assign44650_e43025_d_n0, assign44650_e43025_d_n2, assign44650_e43025_d_n4, assign44650_e43025_d_n8, assign44650_e43025_d_n18, assign44650_e43025_d_n19,)
    } else {
        (var_fn482_calc_ig__expbd1, var_fn482_calc_ig__expbd1_dn0, var_fn482_calc_ig__expbd1_dn2, var_fn482_calc_ig__expbd1_dn4, var_fn482_calc_ig__expbd1_dn8, var_fn482_calc_ig__expbd1_dn18, var_fn482_calc_ig__expbd1_dn19,)
    }
};
        var_fn482_calc_ig__expbd1 = assign44650_e43027;
        var_fn482_calc_ig__expbd1_dn0 = assign44650_e43027_d_n0;
        var_fn482_calc_ig__expbd1_dn2 = assign44650_e43027_d_n2;
        var_fn482_calc_ig__expbd1_dn4 = assign44650_e43027_d_n4;
        var_fn482_calc_ig__expbd1_dn8 = assign44650_e43027_d_n8;
        var_fn482_calc_ig__expbd1_dn18 = assign44650_e43027_d_n18;
        var_fn482_calc_ig__expbd1_dn19 = assign44650_e43027_d_n19;

        let (assign44660_e43069, assign44660_e43069_d_n4,) = {
    if (var_guard480 != 0.0) {
        let assign44660_e43035: f64 = (-50.0);
        let (assign44660_e43067, assign44660_e43067_d_n4,) = {
            if ((!(var_fn482_calc_ig__expbdarg2 > 50.0)) && (!(var_fn482_calc_ig__expbdarg2 < assign44660_e43035))) {
                let assign44660_e43040: f64 = (var_fn482_calc_ig__expbdarg2).exp();
                (assign44660_e43040, (assign44660_e43040 * var_fn482_calc_ig__expbdarg2_dn4),)
            } else {
                let assign44660_e43047: f64 = (-50.0);
                let (assign44660_e43066, assign44660_e43066_d_n4,) = {
                    if ((!(var_fn482_calc_ig__expbdarg2 > 50.0)) && (var_fn482_calc_ig__expbdarg2 < assign44660_e43047)) {
                        let assign44660_e43051: f64 = (-50.0);
                        let assign44660_e43052: f64 = (assign44660_e43051).exp();
                        (assign44660_e43052, 0.0,)
                    } else {
                        let (assign44660_e43065, assign44660_e43065_d_n4,) = {
                            if (var_fn482_calc_ig__expbdarg2 > 50.0) {
                                let assign44660_e43057: f64 = (50.0_f64).exp();
                                let assign44660_e43061: f64 = (var_fn482_calc_ig__expbdarg2 - 50.0);
                                let assign44660_e43062: f64 = (1.0 + assign44660_e43061);
                                let assign44660_e43063: f64 = (assign44660_e43057 * assign44660_e43062);
                                (assign44660_e43063, (assign44660_e43057 * var_fn482_calc_ig__expbdarg2_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign44660_e43065, assign44660_e43065_d_n4,)
                    }
                };
                (assign44660_e43066, assign44660_e43066_d_n4,)
            }
        };
        (assign44660_e43067, assign44660_e43067_d_n4,)
    } else {
        (var_fn482_calc_ig__expbd2, var_fn482_calc_ig__expbd2_dn4,)
    }
};
        var_fn482_calc_ig__expbd2 = assign44660_e43069;
        var_fn482_calc_ig__expbd2_dn4 = assign44660_e43069_d_n4;

        let (assign44670_e43075, assign44670_e43075_d_n0, assign44670_e43075_d_n2, assign44670_e43075_d_n4, assign44670_e43075_d_n8, assign44670_e43075_d_n18, assign44670_e43075_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign44670_e43073: f64 = (var_fn482_calc_ig__expbd1 - var_fn482_calc_ig__expbd2);
        (assign44670_e43073, var_fn482_calc_ig__expbd1_dn0, var_fn482_calc_ig__expbd1_dn2, (var_fn482_calc_ig__expbd1_dn4 - var_fn482_calc_ig__expbd2_dn4), var_fn482_calc_ig__expbd1_dn8, var_fn482_calc_ig__expbd1_dn18, var_fn482_calc_ig__expbd1_dn19,)
    } else {
        (var_fn482_calc_ig__iginbd, var_fn482_calc_ig__iginbd_dn0, var_fn482_calc_ig__iginbd_dn2, var_fn482_calc_ig__iginbd_dn4, var_fn482_calc_ig__iginbd_dn8, var_fn482_calc_ig__iginbd_dn18, var_fn482_calc_ig__iginbd_dn19,)
    }
};
        var_fn482_calc_ig__iginbd = assign44670_e43075;
        var_fn482_calc_ig__iginbd_dn0 = assign44670_e43075_d_n0;
        var_fn482_calc_ig__iginbd_dn2 = assign44670_e43075_d_n2;
        var_fn482_calc_ig__iginbd_dn4 = assign44670_e43075_d_n4;
        var_fn482_calc_ig__iginbd_dn8 = assign44670_e43075_d_n8;
        var_fn482_calc_ig__iginbd_dn18 = assign44670_e43075_d_n18;
        var_fn482_calc_ig__iginbd_dn19 = assign44670_e43075_d_n19;

        let (assign44680_e43087, assign44680_e43087_d_n4,) = {
    if (var_guard480 != 0.0) {
        let assign44680_e43079: f64 = (var_fn482_calc_ig__type * var_fn482_calc_ig__w);
        let assign44680_e43081: f64 = (assign44680_e43079 * var_fn482_calc_ig__ngf);
        let assign44680_e43083: f64 = (assign44680_e43081 * var_fn482_calc_ig__ijin);
        let assign44680_e43085: f64 = (assign44680_e43083 * var_fn482_calc_ig__tfacdiodein);
        (assign44680_e43085, (assign44680_e43083 * var_fn482_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn482_calc_ig__isdiodeout, var_fn482_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn482_calc_ig__isdiodeout = assign44680_e43087;
        var_fn482_calc_ig__isdiodeout_dn4 = assign44680_e43087_d_n4;

        let (assign44690_e43097, assign44690_e43097_d_n0, assign44690_e43097_d_n2, assign44690_e43097_d_n4, assign44690_e43097_d_n8, assign44690_e43097_d_n18, assign44690_e43097_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign44690_e43091: f64 = (var_fn482_calc_ig__pg_paramin / var_fn482_calc_ig__phitin);
        let assign44690_e43093: f64 = (assign44690_e43091 * var_fn482_calc_ig__vgin);
        let assign44690_e43095: f64 = (assign44690_e43093 + var_fn482_calc_ig__expphib);
        (assign44690_e43095, (assign44690_e43091 * var_fn482_calc_ig__vgin_dn0), (assign44690_e43091 * var_fn482_calc_ig__vgin_dn2), (((-((var_fn482_calc_ig__pg_paramin * var_fn482_calc_ig__phitin_dn4) / (var_fn482_calc_ig__phitin * var_fn482_calc_ig__phitin))) * var_fn482_calc_ig__vgin) + var_fn482_calc_ig__expphib_dn4), (assign44690_e43091 * var_fn482_calc_ig__vgin_dn8), (assign44690_e43091 * var_fn482_calc_ig__vgin_dn18), (assign44690_e43091 * var_fn482_calc_ig__vgin_dn19),)
    } else {
        (var_fn482_calc_ig__expiforarg, var_fn482_calc_ig__expiforarg_dn0, var_fn482_calc_ig__expiforarg_dn2, var_fn482_calc_ig__expiforarg_dn4, var_fn482_calc_ig__expiforarg_dn8, var_fn482_calc_ig__expiforarg_dn18, var_fn482_calc_ig__expiforarg_dn19,)
    }
};
        var_fn482_calc_ig__expiforarg = assign44690_e43097;
        var_fn482_calc_ig__expiforarg_dn0 = assign44690_e43097_d_n0;
        var_fn482_calc_ig__expiforarg_dn2 = assign44690_e43097_d_n2;
        var_fn482_calc_ig__expiforarg_dn4 = assign44690_e43097_d_n4;
        var_fn482_calc_ig__expiforarg_dn8 = assign44690_e43097_d_n8;
        var_fn482_calc_ig__expiforarg_dn18 = assign44690_e43097_d_n18;
        var_fn482_calc_ig__expiforarg_dn19 = assign44690_e43097_d_n19;

        let (assign44700_e43139, assign44700_e43139_d_n0, assign44700_e43139_d_n2, assign44700_e43139_d_n4, assign44700_e43139_d_n8, assign44700_e43139_d_n18, assign44700_e43139_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign44700_e43105: f64 = (-50.0);
        let (assign44700_e43137, assign44700_e43137_d_n0, assign44700_e43137_d_n2, assign44700_e43137_d_n4, assign44700_e43137_d_n8, assign44700_e43137_d_n18, assign44700_e43137_d_n19,) = {
            if ((!(var_fn482_calc_ig__expiforarg > 50.0)) && (!(var_fn482_calc_ig__expiforarg < assign44700_e43105))) {
                let assign44700_e43110: f64 = (var_fn482_calc_ig__expiforarg).exp();
                (assign44700_e43110, (assign44700_e43110 * var_fn482_calc_ig__expiforarg_dn0), (assign44700_e43110 * var_fn482_calc_ig__expiforarg_dn2), (assign44700_e43110 * var_fn482_calc_ig__expiforarg_dn4), (assign44700_e43110 * var_fn482_calc_ig__expiforarg_dn8), (assign44700_e43110 * var_fn482_calc_ig__expiforarg_dn18), (assign44700_e43110 * var_fn482_calc_ig__expiforarg_dn19),)
            } else {
                let assign44700_e43117: f64 = (-50.0);
                let (assign44700_e43136, assign44700_e43136_d_n0, assign44700_e43136_d_n2, assign44700_e43136_d_n4, assign44700_e43136_d_n8, assign44700_e43136_d_n18, assign44700_e43136_d_n19,) = {
                    if ((!(var_fn482_calc_ig__expiforarg > 50.0)) && (var_fn482_calc_ig__expiforarg < assign44700_e43117)) {
                        let assign44700_e43121: f64 = (-50.0);
                        let assign44700_e43122: f64 = (assign44700_e43121).exp();
                        (assign44700_e43122, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign44700_e43135, assign44700_e43135_d_n0, assign44700_e43135_d_n2, assign44700_e43135_d_n4, assign44700_e43135_d_n8, assign44700_e43135_d_n18, assign44700_e43135_d_n19,) = {
                            if (var_fn482_calc_ig__expiforarg > 50.0) {
                                let assign44700_e43127: f64 = (50.0_f64).exp();
                                let assign44700_e43131: f64 = (var_fn482_calc_ig__expiforarg - 50.0);
                                let assign44700_e43132: f64 = (1.0 + assign44700_e43131);
                                let assign44700_e43133: f64 = (assign44700_e43127 * assign44700_e43132);
                                (assign44700_e43133, (assign44700_e43127 * var_fn482_calc_ig__expiforarg_dn0), (assign44700_e43127 * var_fn482_calc_ig__expiforarg_dn2), (assign44700_e43127 * var_fn482_calc_ig__expiforarg_dn4), (assign44700_e43127 * var_fn482_calc_ig__expiforarg_dn8), (assign44700_e43127 * var_fn482_calc_ig__expiforarg_dn18), (assign44700_e43127 * var_fn482_calc_ig__expiforarg_dn19),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign44700_e43135, assign44700_e43135_d_n0, assign44700_e43135_d_n2, assign44700_e43135_d_n4, assign44700_e43135_d_n8, assign44700_e43135_d_n18, assign44700_e43135_d_n19,)
                    }
                };
                (assign44700_e43136, assign44700_e43136_d_n0, assign44700_e43136_d_n2, assign44700_e43136_d_n4, assign44700_e43136_d_n8, assign44700_e43136_d_n18, assign44700_e43136_d_n19,)
            }
        };
        (assign44700_e43137, assign44700_e43137_d_n0, assign44700_e43137_d_n2, assign44700_e43137_d_n4, assign44700_e43137_d_n8, assign44700_e43137_d_n18, assign44700_e43137_d_n19,)
    } else {
        (var_fn482_calc_ig__expifor, var_fn482_calc_ig__expifor_dn0, var_fn482_calc_ig__expifor_dn2, var_fn482_calc_ig__expifor_dn4, var_fn482_calc_ig__expifor_dn8, var_fn482_calc_ig__expifor_dn18, var_fn482_calc_ig__expifor_dn19,)
    }
};
        var_fn482_calc_ig__expifor = assign44700_e43139;
        var_fn482_calc_ig__expifor_dn0 = assign44700_e43139_d_n0;
        var_fn482_calc_ig__expifor_dn2 = assign44700_e43139_d_n2;
        var_fn482_calc_ig__expifor_dn4 = assign44700_e43139_d_n4;
        var_fn482_calc_ig__expifor_dn8 = assign44700_e43139_d_n8;
        var_fn482_calc_ig__expifor_dn18 = assign44700_e43139_d_n18;
        var_fn482_calc_ig__expifor_dn19 = assign44700_e43139_d_n19;

        let assign44710_e43142: f64 = if var_fn482_calc_ig__fracin == 1.0 { 1.0 } else { 0.0 };
        var_guard483 = assign44710_e43142;

        let (assign44720_e43156, assign44720_e43156_d_n0, assign44720_e43156_d_n2, assign44720_e43156_d_n4, assign44720_e43156_d_n8, assign44720_e43156_d_n18, assign44720_e43156_d_n19,) = {
    if ((var_guard480 != 0.0) && (var_guard483 != 0.0)) {
        let assign44720_e43150: f64 = (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd);
        let assign44720_e43151: f64 = (var_fn482_calc_ig__expifor - assign44720_e43150);
        let assign44720_e43153: f64 = (assign44720_e43151 - var_fn482_calc_ig__t0);
        let assign44720_e43154: f64 = (var_fn482_calc_ig__isdiodeout * assign44720_e43153);
        (assign44720_e43154, (var_fn482_calc_ig__isdiodeout * (var_fn482_calc_ig__expifor_dn0 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn0))), (var_fn482_calc_ig__isdiodeout * (var_fn482_calc_ig__expifor_dn2 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn2))), ((var_fn482_calc_ig__isdiodeout_dn4 * assign44720_e43153) + (var_fn482_calc_ig__isdiodeout * ((var_fn482_calc_ig__expifor_dn4 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn4)) - var_fn482_calc_ig__t0_dn4))), (var_fn482_calc_ig__isdiodeout * (var_fn482_calc_ig__expifor_dn8 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn8))), (var_fn482_calc_ig__isdiodeout * (var_fn482_calc_ig__expifor_dn18 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn18))), (var_fn482_calc_ig__isdiodeout * (var_fn482_calc_ig__expifor_dn19 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn19))),)
    } else {
        (var_fn482_calc_ig__igindiode, var_fn482_calc_ig__igindiode_dn0, var_fn482_calc_ig__igindiode_dn2, var_fn482_calc_ig__igindiode_dn4, var_fn482_calc_ig__igindiode_dn8, var_fn482_calc_ig__igindiode_dn18, var_fn482_calc_ig__igindiode_dn19,)
    }
};
        var_fn482_calc_ig__igindiode = assign44720_e43156;
        var_fn482_calc_ig__igindiode_dn0 = assign44720_e43156_d_n0;
        var_fn482_calc_ig__igindiode_dn2 = assign44720_e43156_d_n2;
        var_fn482_calc_ig__igindiode_dn4 = assign44720_e43156_d_n4;
        var_fn482_calc_ig__igindiode_dn8 = assign44720_e43156_d_n8;
        var_fn482_calc_ig__igindiode_dn18 = assign44720_e43156_d_n18;
        var_fn482_calc_ig__igindiode_dn19 = assign44720_e43156_d_n19;

        let (assign44730_e43170, assign44730_e43170_d_n4,) = {
    if ((var_guard480 != 0.0) && (var_guard483 == 0.0)) {
        let assign44730_e43163: f64 = (-var_fn482_calc_ig__vgsatin);
        let assign44730_e43165: f64 = (assign44730_e43163 - var_fn482_calc_ig__vbdgin);
        let assign44730_e43166: f64 = (var_fn482_calc_ig__pbdgin * assign44730_e43165);
        let assign44730_e43168: f64 = (assign44730_e43166 + var_fn482_calc_ig__expphib);
        (assign44730_e43168, var_fn482_calc_ig__expphib_dn4,)
    } else {
        (var_fn482_calc_ig__expbdarg1_vgsat, var_fn482_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__expbdarg1_vgsat = assign44730_e43170;
        var_fn482_calc_ig__expbdarg1_vgsat_dn4 = assign44730_e43170_d_n4;

        let (assign44740_e43215, assign44740_e43215_d_n4,) = {
    if ((var_guard480 != 0.0) && (var_guard483 == 0.0)) {
        let assign44740_e43181: f64 = (-50.0);
        let (assign44740_e43213, assign44740_e43213_d_n4,) = {
            if ((!(var_fn482_calc_ig__expbdarg1_vgsat > 50.0)) && (!(var_fn482_calc_ig__expbdarg1_vgsat < assign44740_e43181))) {
                let assign44740_e43186: f64 = (var_fn482_calc_ig__expbdarg1_vgsat).exp();
                (assign44740_e43186, (assign44740_e43186 * var_fn482_calc_ig__expbdarg1_vgsat_dn4),)
            } else {
                let assign44740_e43193: f64 = (-50.0);
                let (assign44740_e43212, assign44740_e43212_d_n4,) = {
                    if ((!(var_fn482_calc_ig__expbdarg1_vgsat > 50.0)) && (var_fn482_calc_ig__expbdarg1_vgsat < assign44740_e43193)) {
                        let assign44740_e43197: f64 = (-50.0);
                        let assign44740_e43198: f64 = (assign44740_e43197).exp();
                        (assign44740_e43198, 0.0,)
                    } else {
                        let (assign44740_e43211, assign44740_e43211_d_n4,) = {
                            if (var_fn482_calc_ig__expbdarg1_vgsat > 50.0) {
                                let assign44740_e43203: f64 = (50.0_f64).exp();
                                let assign44740_e43207: f64 = (var_fn482_calc_ig__expbdarg1_vgsat - 50.0);
                                let assign44740_e43208: f64 = (1.0 + assign44740_e43207);
                                let assign44740_e43209: f64 = (assign44740_e43203 * assign44740_e43208);
                                (assign44740_e43209, (assign44740_e43203 * var_fn482_calc_ig__expbdarg1_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign44740_e43211, assign44740_e43211_d_n4,)
                    }
                };
                (assign44740_e43212, assign44740_e43212_d_n4,)
            }
        };
        (assign44740_e43213, assign44740_e43213_d_n4,)
    } else {
        (var_fn482_calc_ig__expbd1_vgsat, var_fn482_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__expbd1_vgsat = assign44740_e43215;
        var_fn482_calc_ig__expbd1_vgsat_dn4 = assign44740_e43215_d_n4;

        *var_fn482_calc_ig__expbd1_slot = var_fn482_calc_ig__expbd1;
        *var_fn482_calc_ig__expbd1_dn0_slot = var_fn482_calc_ig__expbd1_dn0;
        *var_fn482_calc_ig__expbd1_dn18_slot = var_fn482_calc_ig__expbd1_dn18;
        *var_fn482_calc_ig__expbd1_dn19_slot = var_fn482_calc_ig__expbd1_dn19;
        *var_fn482_calc_ig__expbd1_dn2_slot = var_fn482_calc_ig__expbd1_dn2;
        *var_fn482_calc_ig__expbd1_dn4_slot = var_fn482_calc_ig__expbd1_dn4;
        *var_fn482_calc_ig__expbd1_dn8_slot = var_fn482_calc_ig__expbd1_dn8;
        *var_fn482_calc_ig__expbd1_vgsat_slot = var_fn482_calc_ig__expbd1_vgsat;
        *var_fn482_calc_ig__expbd1_vgsat_dn4_slot = var_fn482_calc_ig__expbd1_vgsat_dn4;
        *var_fn482_calc_ig__expbd2_slot = var_fn482_calc_ig__expbd2;
        *var_fn482_calc_ig__expbd2_dn4_slot = var_fn482_calc_ig__expbd2_dn4;
        *var_fn482_calc_ig__expbdarg1_slot = var_fn482_calc_ig__expbdarg1;
        *var_fn482_calc_ig__expbdarg1_dn0_slot = var_fn482_calc_ig__expbdarg1_dn0;
        *var_fn482_calc_ig__expbdarg1_dn18_slot = var_fn482_calc_ig__expbdarg1_dn18;
        *var_fn482_calc_ig__expbdarg1_dn19_slot = var_fn482_calc_ig__expbdarg1_dn19;
        *var_fn482_calc_ig__expbdarg1_dn2_slot = var_fn482_calc_ig__expbdarg1_dn2;
        *var_fn482_calc_ig__expbdarg1_dn4_slot = var_fn482_calc_ig__expbdarg1_dn4;
        *var_fn482_calc_ig__expbdarg1_dn8_slot = var_fn482_calc_ig__expbdarg1_dn8;
        *var_fn482_calc_ig__expbdarg1_vgsat_slot = var_fn482_calc_ig__expbdarg1_vgsat;
        *var_fn482_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn482_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn482_calc_ig__expbdarg2_slot = var_fn482_calc_ig__expbdarg2;
        *var_fn482_calc_ig__expbdarg2_dn4_slot = var_fn482_calc_ig__expbdarg2_dn4;
        *var_fn482_calc_ig__expifor_slot = var_fn482_calc_ig__expifor;
        *var_fn482_calc_ig__expifor_dn0_slot = var_fn482_calc_ig__expifor_dn0;
        *var_fn482_calc_ig__expifor_dn18_slot = var_fn482_calc_ig__expifor_dn18;
        *var_fn482_calc_ig__expifor_dn19_slot = var_fn482_calc_ig__expifor_dn19;
        *var_fn482_calc_ig__expifor_dn2_slot = var_fn482_calc_ig__expifor_dn2;
        *var_fn482_calc_ig__expifor_dn4_slot = var_fn482_calc_ig__expifor_dn4;
        *var_fn482_calc_ig__expifor_dn8_slot = var_fn482_calc_ig__expifor_dn8;
        *var_fn482_calc_ig__expifor_hinj_slot = var_fn482_calc_ig__expifor_hinj;
        *var_fn482_calc_ig__expifor_hinj_dn0_slot = var_fn482_calc_ig__expifor_hinj_dn0;
        *var_fn482_calc_ig__expifor_hinj_dn18_slot = var_fn482_calc_ig__expifor_hinj_dn18;
        *var_fn482_calc_ig__expifor_hinj_dn19_slot = var_fn482_calc_ig__expifor_hinj_dn19;
        *var_fn482_calc_ig__expifor_hinj_dn2_slot = var_fn482_calc_ig__expifor_hinj_dn2;
        *var_fn482_calc_ig__expifor_hinj_dn4_slot = var_fn482_calc_ig__expifor_hinj_dn4;
        *var_fn482_calc_ig__expifor_hinj_dn8_slot = var_fn482_calc_ig__expifor_hinj_dn8;
        *var_fn482_calc_ig__expifor_hinj_vgsat_slot = var_fn482_calc_ig__expifor_hinj_vgsat;
        *var_fn482_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn482_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn482_calc_ig__expifor_nohinj_vgsat_slot = var_fn482_calc_ig__expifor_nohinj_vgsat;
        *var_fn482_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn482_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn482_calc_ig__expiforarg_slot = var_fn482_calc_ig__expiforarg;
        *var_fn482_calc_ig__expiforarg_dn0_slot = var_fn482_calc_ig__expiforarg_dn0;
        *var_fn482_calc_ig__expiforarg_dn18_slot = var_fn482_calc_ig__expiforarg_dn18;
        *var_fn482_calc_ig__expiforarg_dn19_slot = var_fn482_calc_ig__expiforarg_dn19;
        *var_fn482_calc_ig__expiforarg_dn2_slot = var_fn482_calc_ig__expiforarg_dn2;
        *var_fn482_calc_ig__expiforarg_dn4_slot = var_fn482_calc_ig__expiforarg_dn4;
        *var_fn482_calc_ig__expiforarg_dn8_slot = var_fn482_calc_ig__expiforarg_dn8;
        *var_fn482_calc_ig__expiforarg_hinj_slot = var_fn482_calc_ig__expiforarg_hinj;
        *var_fn482_calc_ig__expiforarg_hinj_dn0_slot = var_fn482_calc_ig__expiforarg_hinj_dn0;
        *var_fn482_calc_ig__expiforarg_hinj_dn18_slot = var_fn482_calc_ig__expiforarg_hinj_dn18;
        *var_fn482_calc_ig__expiforarg_hinj_dn19_slot = var_fn482_calc_ig__expiforarg_hinj_dn19;
        *var_fn482_calc_ig__expiforarg_hinj_dn2_slot = var_fn482_calc_ig__expiforarg_hinj_dn2;
        *var_fn482_calc_ig__expiforarg_hinj_dn4_slot = var_fn482_calc_ig__expiforarg_hinj_dn4;
        *var_fn482_calc_ig__expiforarg_hinj_dn8_slot = var_fn482_calc_ig__expiforarg_hinj_dn8;
        *var_fn482_calc_ig__expiforarg_hinj_vgsat_slot = var_fn482_calc_ig__expiforarg_hinj_vgsat;
        *var_fn482_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn482_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn482_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn482_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn482_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn482_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn482_calc_ig__expphib_slot = var_fn482_calc_ig__expphib;
        *var_fn482_calc_ig__expphib_dn4_slot = var_fn482_calc_ig__expphib_dn4;
        *var_fn482_calc_ig__iginbd_slot = var_fn482_calc_ig__iginbd;
        *var_fn482_calc_ig__iginbd_dn0_slot = var_fn482_calc_ig__iginbd_dn0;
        *var_fn482_calc_ig__iginbd_dn18_slot = var_fn482_calc_ig__iginbd_dn18;
        *var_fn482_calc_ig__iginbd_dn19_slot = var_fn482_calc_ig__iginbd_dn19;
        *var_fn482_calc_ig__iginbd_dn2_slot = var_fn482_calc_ig__iginbd_dn2;
        *var_fn482_calc_ig__iginbd_dn4_slot = var_fn482_calc_ig__iginbd_dn4;
        *var_fn482_calc_ig__iginbd_dn8_slot = var_fn482_calc_ig__iginbd_dn8;
        *var_fn482_calc_ig__iginbd_vgsat_slot = var_fn482_calc_ig__iginbd_vgsat;
        *var_fn482_calc_ig__iginbd_vgsat_dn4_slot = var_fn482_calc_ig__iginbd_vgsat_dn4;
        *var_fn482_calc_ig__igindiode_slot = var_fn482_calc_ig__igindiode;
        *var_fn482_calc_ig__igindiode_dn0_slot = var_fn482_calc_ig__igindiode_dn0;
        *var_fn482_calc_ig__igindiode_dn18_slot = var_fn482_calc_ig__igindiode_dn18;
        *var_fn482_calc_ig__igindiode_dn19_slot = var_fn482_calc_ig__igindiode_dn19;
        *var_fn482_calc_ig__igindiode_dn2_slot = var_fn482_calc_ig__igindiode_dn2;
        *var_fn482_calc_ig__igindiode_dn4_slot = var_fn482_calc_ig__igindiode_dn4;
        *var_fn482_calc_ig__igindiode_dn8_slot = var_fn482_calc_ig__igindiode_dn8;
        *var_fn482_calc_ig__igindiode_hinj_slot = var_fn482_calc_ig__igindiode_hinj;
        *var_fn482_calc_ig__igindiode_hinj_dn0_slot = var_fn482_calc_ig__igindiode_hinj_dn0;
        *var_fn482_calc_ig__igindiode_hinj_dn18_slot = var_fn482_calc_ig__igindiode_hinj_dn18;
        *var_fn482_calc_ig__igindiode_hinj_dn19_slot = var_fn482_calc_ig__igindiode_hinj_dn19;
        *var_fn482_calc_ig__igindiode_hinj_dn2_slot = var_fn482_calc_ig__igindiode_hinj_dn2;
        *var_fn482_calc_ig__igindiode_hinj_dn4_slot = var_fn482_calc_ig__igindiode_hinj_dn4;
        *var_fn482_calc_ig__igindiode_hinj_dn8_slot = var_fn482_calc_ig__igindiode_hinj_dn8;
        *var_fn482_calc_ig__igindiode_hinj_pre_slot = var_fn482_calc_ig__igindiode_hinj_pre;
        *var_fn482_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn482_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn482_calc_ig__igindiode_hinj_vgsat_slot = var_fn482_calc_ig__igindiode_hinj_vgsat;
        *var_fn482_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn482_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn482_calc_ig__igindiode_nohinj_slot = var_fn482_calc_ig__igindiode_nohinj;
        *var_fn482_calc_ig__igindiode_nohinj_dn0_slot = var_fn482_calc_ig__igindiode_nohinj_dn0;
        *var_fn482_calc_ig__igindiode_nohinj_dn18_slot = var_fn482_calc_ig__igindiode_nohinj_dn18;
        *var_fn482_calc_ig__igindiode_nohinj_dn19_slot = var_fn482_calc_ig__igindiode_nohinj_dn19;
        *var_fn482_calc_ig__igindiode_nohinj_dn2_slot = var_fn482_calc_ig__igindiode_nohinj_dn2;
        *var_fn482_calc_ig__igindiode_nohinj_dn4_slot = var_fn482_calc_ig__igindiode_nohinj_dn4;
        *var_fn482_calc_ig__igindiode_nohinj_dn8_slot = var_fn482_calc_ig__igindiode_nohinj_dn8;
        *var_fn482_calc_ig__igindiode_nohinj_vgsat_slot = var_fn482_calc_ig__igindiode_nohinj_vgsat;
        *var_fn482_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn482_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn482_calc_ig__isdiodeout_slot = var_fn482_calc_ig__isdiodeout;
        *var_fn482_calc_ig__isdiodeout_dn4_slot = var_fn482_calc_ig__isdiodeout_dn4;
        *var_fn482_calc_ig__pg_paramin_hinj_slot = var_fn482_calc_ig__pg_paramin_hinj;
        *var_fn482_calc_ig__t0_slot = var_fn482_calc_ig__t0;
        *var_fn482_calc_ig__t0_dn4_slot = var_fn482_calc_ig__t0_dn4;
        *var_guard483_slot = var_guard483;
    }
}
