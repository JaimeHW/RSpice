#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_112(
        p: &Parameters,
        var_fn482_calc_ig__alphagin: f64,
        var_fn482_calc_ig__betarecin: f64,
        var_fn482_calc_ig__expbd1_vgsat: f64,
        var_fn482_calc_ig__expbd1_vgsat_dn4: f64,
        var_fn482_calc_ig__expbd2: f64,
        var_fn482_calc_ig__expbd2_dn4: f64,
        var_fn482_calc_ig__expifor: f64,
        var_fn482_calc_ig__expifor_dn0: f64,
        var_fn482_calc_ig__expifor_dn18: f64,
        var_fn482_calc_ig__expifor_dn19: f64,
        var_fn482_calc_ig__expifor_dn2: f64,
        var_fn482_calc_ig__expifor_dn4: f64,
        var_fn482_calc_ig__expifor_dn8: f64,
        var_fn482_calc_ig__expphib: f64,
        var_fn482_calc_ig__expphib_dn4: f64,
        var_fn482_calc_ig__fracin: f64,
        var_fn482_calc_ig__iginbd: f64,
        var_fn482_calc_ig__iginbd_dn0: f64,
        var_fn482_calc_ig__iginbd_dn18: f64,
        var_fn482_calc_ig__iginbd_dn19: f64,
        var_fn482_calc_ig__iginbd_dn2: f64,
        var_fn482_calc_ig__iginbd_dn4: f64,
        var_fn482_calc_ig__iginbd_dn8: f64,
        var_fn482_calc_ig__irecin: f64,
        var_fn482_calc_ig__isdiodeout: f64,
        var_fn482_calc_ig__isdiodeout_dn4: f64,
        var_fn482_calc_ig__kbdgatein: f64,
        var_fn482_calc_ig__ngf: f64,
        var_fn482_calc_ig__pg_paramin: f64,
        var_fn482_calc_ig__pgsrecin: f64,
        var_fn482_calc_ig__phitin: f64,
        var_fn482_calc_ig__phitin_dn4: f64,
        var_fn482_calc_ig__t0: f64,
        var_fn482_calc_ig__t0_dn4: f64,
        var_fn482_calc_ig__tfacdiodein: f64,
        var_fn482_calc_ig__tfacdiodein_dn4: f64,
        var_fn482_calc_ig__type: f64,
        var_fn482_calc_ig__vgin: f64,
        var_fn482_calc_ig__vgin_dn0: f64,
        var_fn482_calc_ig__vgin_dn18: f64,
        var_fn482_calc_ig__vgin_dn19: f64,
        var_fn482_calc_ig__vgin_dn2: f64,
        var_fn482_calc_ig__vgin_dn8: f64,
        var_fn482_calc_ig__vgsatin: f64,
        var_fn482_calc_ig__vgsatqin: f64,
        var_fn482_calc_ig__w: f64,
        var_guard480: f64,
        var_guard483: f64,
        var_fn482_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn482_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn482_calc_ig__expffvarg_slot: &mut f64,
        var_fn482_calc_ig__expffvarg_dn0_slot: &mut f64,
        var_fn482_calc_ig__expffvarg_dn18_slot: &mut f64,
        var_fn482_calc_ig__expffvarg_dn19_slot: &mut f64,
        var_fn482_calc_ig__expffvarg_dn2_slot: &mut f64,
        var_fn482_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn482_calc_ig__expffvarg_dn8_slot: &mut f64,
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
        var_fn482_calc_ig__expirevarg_slot: &mut f64,
        var_fn482_calc_ig__expirevarg_dn0_slot: &mut f64,
        var_fn482_calc_ig__expirevarg_dn18_slot: &mut f64,
        var_fn482_calc_ig__expirevarg_dn19_slot: &mut f64,
        var_fn482_calc_ig__expirevarg_dn2_slot: &mut f64,
        var_fn482_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn482_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn482_calc_ig__ffvgin_slot: &mut f64,
        var_fn482_calc_ig__ffvgin_dn0_slot: &mut f64,
        var_fn482_calc_ig__ffvgin_dn18_slot: &mut f64,
        var_fn482_calc_ig__ffvgin_dn19_slot: &mut f64,
        var_fn482_calc_ig__ffvgin_dn2_slot: &mut f64,
        var_fn482_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn482_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn482_calc_ig__frecgin_slot: &mut f64,
        var_fn482_calc_ig__frecgin_dn0_slot: &mut f64,
        var_fn482_calc_ig__frecgin_dn18_slot: &mut f64,
        var_fn482_calc_ig__frecgin_dn19_slot: &mut f64,
        var_fn482_calc_ig__frecgin_dn2_slot: &mut f64,
        var_fn482_calc_ig__frecgin_dn8_slot: &mut f64,
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
        var_fn482_calc_ig__isrecout_slot: &mut f64,
        var_fn482_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn482_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_guard484_slot: &mut f64,
        var_guard485_slot: &mut f64,
        var_guard486_slot: &mut f64,
    ) {
        let mut var_fn482_calc_ig__alpha2_phit: f64 = *var_fn482_calc_ig__alpha2_phit_slot;
        let mut var_fn482_calc_ig__alpha2_phit_dn4: f64 = *var_fn482_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn482_calc_ig__expffvarg: f64 = *var_fn482_calc_ig__expffvarg_slot;
        let mut var_fn482_calc_ig__expffvarg_dn0: f64 = *var_fn482_calc_ig__expffvarg_dn0_slot;
        let mut var_fn482_calc_ig__expffvarg_dn18: f64 = *var_fn482_calc_ig__expffvarg_dn18_slot;
        let mut var_fn482_calc_ig__expffvarg_dn19: f64 = *var_fn482_calc_ig__expffvarg_dn19_slot;
        let mut var_fn482_calc_ig__expffvarg_dn2: f64 = *var_fn482_calc_ig__expffvarg_dn2_slot;
        let mut var_fn482_calc_ig__expffvarg_dn4: f64 = *var_fn482_calc_ig__expffvarg_dn4_slot;
        let mut var_fn482_calc_ig__expffvarg_dn8: f64 = *var_fn482_calc_ig__expffvarg_dn8_slot;
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
        let mut var_fn482_calc_ig__expirevarg: f64 = *var_fn482_calc_ig__expirevarg_slot;
        let mut var_fn482_calc_ig__expirevarg_dn0: f64 = *var_fn482_calc_ig__expirevarg_dn0_slot;
        let mut var_fn482_calc_ig__expirevarg_dn18: f64 = *var_fn482_calc_ig__expirevarg_dn18_slot;
        let mut var_fn482_calc_ig__expirevarg_dn19: f64 = *var_fn482_calc_ig__expirevarg_dn19_slot;
        let mut var_fn482_calc_ig__expirevarg_dn2: f64 = *var_fn482_calc_ig__expirevarg_dn2_slot;
        let mut var_fn482_calc_ig__expirevarg_dn4: f64 = *var_fn482_calc_ig__expirevarg_dn4_slot;
        let mut var_fn482_calc_ig__expirevarg_dn8: f64 = *var_fn482_calc_ig__expirevarg_dn8_slot;
        let mut var_fn482_calc_ig__ffvgin: f64 = *var_fn482_calc_ig__ffvgin_slot;
        let mut var_fn482_calc_ig__ffvgin_dn0: f64 = *var_fn482_calc_ig__ffvgin_dn0_slot;
        let mut var_fn482_calc_ig__ffvgin_dn18: f64 = *var_fn482_calc_ig__ffvgin_dn18_slot;
        let mut var_fn482_calc_ig__ffvgin_dn19: f64 = *var_fn482_calc_ig__ffvgin_dn19_slot;
        let mut var_fn482_calc_ig__ffvgin_dn2: f64 = *var_fn482_calc_ig__ffvgin_dn2_slot;
        let mut var_fn482_calc_ig__ffvgin_dn4: f64 = *var_fn482_calc_ig__ffvgin_dn4_slot;
        let mut var_fn482_calc_ig__ffvgin_dn8: f64 = *var_fn482_calc_ig__ffvgin_dn8_slot;
        let mut var_fn482_calc_ig__frecgin: f64 = *var_fn482_calc_ig__frecgin_slot;
        let mut var_fn482_calc_ig__frecgin_dn0: f64 = *var_fn482_calc_ig__frecgin_dn0_slot;
        let mut var_fn482_calc_ig__frecgin_dn18: f64 = *var_fn482_calc_ig__frecgin_dn18_slot;
        let mut var_fn482_calc_ig__frecgin_dn19: f64 = *var_fn482_calc_ig__frecgin_dn19_slot;
        let mut var_fn482_calc_ig__frecgin_dn2: f64 = *var_fn482_calc_ig__frecgin_dn2_slot;
        let mut var_fn482_calc_ig__frecgin_dn8: f64 = *var_fn482_calc_ig__frecgin_dn8_slot;
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
        let mut var_fn482_calc_ig__isrecout: f64 = *var_fn482_calc_ig__isrecout_slot;
        let mut var_fn482_calc_ig__isrecout_dn4: f64 = *var_fn482_calc_ig__isrecout_dn4_slot;
        let mut var_fn482_calc_ig__pg_paramin_hinj: f64 = *var_fn482_calc_ig__pg_paramin_hinj_slot;
        let mut var_guard484: f64 = *var_guard484_slot;
        let mut var_guard485: f64 = *var_guard485_slot;
        let mut var_guard486: f64 = *var_guard486_slot;

        let (assign44750_e43224, assign44750_e43224_d_n4,) = {
    if ((var_guard480 != 0.0) && (var_guard483 == 0.0)) {
        let assign44750_e43222: f64 = (var_fn482_calc_ig__expbd1_vgsat - var_fn482_calc_ig__expbd2);
        (assign44750_e43222, (var_fn482_calc_ig__expbd1_vgsat_dn4 - var_fn482_calc_ig__expbd2_dn4),)
    } else {
        (var_fn482_calc_ig__iginbd_vgsat, var_fn482_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__iginbd_vgsat = assign44750_e43224;
        var_fn482_calc_ig__iginbd_vgsat_dn4 = assign44750_e43224_d_n4;

        let (assign44760_e43237, assign44760_e43237_d_n4,) = {
    if ((var_guard480 != 0.0) && (var_guard483 == 0.0)) {
        let assign44760_e43231: f64 = (var_fn482_calc_ig__pg_paramin / var_fn482_calc_ig__phitin);
        let assign44760_e43233: f64 = (assign44760_e43231 * var_fn482_calc_ig__vgsatin);
        let assign44760_e43235: f64 = (assign44760_e43233 + var_fn482_calc_ig__expphib);
        (assign44760_e43235, (((-((var_fn482_calc_ig__pg_paramin * var_fn482_calc_ig__phitin_dn4) / (var_fn482_calc_ig__phitin * var_fn482_calc_ig__phitin))) * var_fn482_calc_ig__vgsatin) + var_fn482_calc_ig__expphib_dn4),)
    } else {
        (var_fn482_calc_ig__expiforarg_nohinj_vgsat, var_fn482_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__expiforarg_nohinj_vgsat = assign44760_e43237;
        var_fn482_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign44760_e43237_d_n4;

        let (assign44770_e43282, assign44770_e43282_d_n4,) = {
    if ((var_guard480 != 0.0) && (var_guard483 == 0.0)) {
        let assign44770_e43248: f64 = (-50.0);
        let (assign44770_e43280, assign44770_e43280_d_n4,) = {
            if ((!(var_fn482_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (!(var_fn482_calc_ig__expiforarg_nohinj_vgsat < assign44770_e43248))) {
                let assign44770_e43253: f64 = (var_fn482_calc_ig__expiforarg_nohinj_vgsat).exp();
                (assign44770_e43253, (assign44770_e43253 * var_fn482_calc_ig__expiforarg_nohinj_vgsat_dn4),)
            } else {
                let assign44770_e43260: f64 = (-50.0);
                let (assign44770_e43279, assign44770_e43279_d_n4,) = {
                    if ((!(var_fn482_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (var_fn482_calc_ig__expiforarg_nohinj_vgsat < assign44770_e43260)) {
                        let assign44770_e43264: f64 = (-50.0);
                        let assign44770_e43265: f64 = (assign44770_e43264).exp();
                        (assign44770_e43265, 0.0,)
                    } else {
                        let (assign44770_e43278, assign44770_e43278_d_n4,) = {
                            if (var_fn482_calc_ig__expiforarg_nohinj_vgsat > 50.0) {
                                let assign44770_e43270: f64 = (50.0_f64).exp();
                                let assign44770_e43274: f64 = (var_fn482_calc_ig__expiforarg_nohinj_vgsat - 50.0);
                                let assign44770_e43275: f64 = (1.0 + assign44770_e43274);
                                let assign44770_e43276: f64 = (assign44770_e43270 * assign44770_e43275);
                                (assign44770_e43276, (assign44770_e43270 * var_fn482_calc_ig__expiforarg_nohinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign44770_e43278, assign44770_e43278_d_n4,)
                    }
                };
                (assign44770_e43279, assign44770_e43279_d_n4,)
            }
        };
        (assign44770_e43280, assign44770_e43280_d_n4,)
    } else {
        (var_fn482_calc_ig__expifor_nohinj_vgsat, var_fn482_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__expifor_nohinj_vgsat = assign44770_e43282;
        var_fn482_calc_ig__expifor_nohinj_vgsat_dn4 = assign44770_e43282_d_n4;

        let (assign44780_e43295, assign44780_e43295_d_n4,) = {
    if ((var_guard480 != 0.0) && (var_guard483 == 0.0)) {
        let assign44780_e43290: f64 = (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_vgsat);
        let assign44780_e43291: f64 = (var_fn482_calc_ig__expifor_nohinj_vgsat - assign44780_e43290);
        let assign44780_e43293: f64 = (assign44780_e43291 - var_fn482_calc_ig__t0);
        (assign44780_e43293, ((var_fn482_calc_ig__expifor_nohinj_vgsat_dn4 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_vgsat_dn4)) - var_fn482_calc_ig__t0_dn4),)
    } else {
        (var_fn482_calc_ig__igindiode_nohinj_vgsat, var_fn482_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__igindiode_nohinj_vgsat = assign44780_e43295;
        var_fn482_calc_ig__igindiode_nohinj_vgsat_dn4 = assign44780_e43295_d_n4;

        let (assign44790_e43310, assign44790_e43310_d_n0, assign44790_e43310_d_n2, assign44790_e43310_d_n4, assign44790_e43310_d_n8, assign44790_e43310_d_n18, assign44790_e43310_d_n19,) = {
    if ((var_guard480 != 0.0) && (var_guard483 == 0.0)) {
        let assign44790_e43304: f64 = (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd);
        let assign44790_e43305: f64 = (var_fn482_calc_ig__expifor - assign44790_e43304);
        let assign44790_e43307: f64 = (assign44790_e43305 - var_fn482_calc_ig__t0);
        let assign44790_e43308: f64 = (var_fn482_calc_ig__isdiodeout * assign44790_e43307);
        (assign44790_e43308, (var_fn482_calc_ig__isdiodeout * (var_fn482_calc_ig__expifor_dn0 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn0))), (var_fn482_calc_ig__isdiodeout * (var_fn482_calc_ig__expifor_dn2 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn2))), ((var_fn482_calc_ig__isdiodeout_dn4 * assign44790_e43307) + (var_fn482_calc_ig__isdiodeout * ((var_fn482_calc_ig__expifor_dn4 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn4)) - var_fn482_calc_ig__t0_dn4))), (var_fn482_calc_ig__isdiodeout * (var_fn482_calc_ig__expifor_dn8 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn8))), (var_fn482_calc_ig__isdiodeout * (var_fn482_calc_ig__expifor_dn18 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn18))), (var_fn482_calc_ig__isdiodeout * (var_fn482_calc_ig__expifor_dn19 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn19))),)
    } else {
        (var_fn482_calc_ig__igindiode_nohinj, var_fn482_calc_ig__igindiode_nohinj_dn0, var_fn482_calc_ig__igindiode_nohinj_dn2, var_fn482_calc_ig__igindiode_nohinj_dn4, var_fn482_calc_ig__igindiode_nohinj_dn8, var_fn482_calc_ig__igindiode_nohinj_dn18, var_fn482_calc_ig__igindiode_nohinj_dn19,)
    }
};
        var_fn482_calc_ig__igindiode_nohinj = assign44790_e43310;
        var_fn482_calc_ig__igindiode_nohinj_dn0 = assign44790_e43310_d_n0;
        var_fn482_calc_ig__igindiode_nohinj_dn2 = assign44790_e43310_d_n2;
        var_fn482_calc_ig__igindiode_nohinj_dn4 = assign44790_e43310_d_n4;
        var_fn482_calc_ig__igindiode_nohinj_dn8 = assign44790_e43310_d_n8;
        var_fn482_calc_ig__igindiode_nohinj_dn18 = assign44790_e43310_d_n18;
        var_fn482_calc_ig__igindiode_nohinj_dn19 = assign44790_e43310_d_n19;

        let assign44800_e43313: f64 = if var_fn482_calc_ig__fracin > 0.0 { 1.0 } else { 0.0 };
        var_guard484 = assign44800_e43313;

        let (assign44810_e43324,) = {
    if (((var_guard480 != 0.0) && (var_guard483 == 0.0)) && (var_guard484 != 0.0)) {
        let assign44810_e43322: f64 = (var_fn482_calc_ig__fracin * var_fn482_calc_ig__pg_paramin);
        (assign44810_e43322,)
    } else {
        (var_fn482_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn482_calc_ig__pg_paramin_hinj = assign44810_e43324;

        let (assign44820_e43339, assign44820_e43339_d_n4,) = {
    if (((var_guard480 != 0.0) && (var_guard483 == 0.0)) && (var_guard484 != 0.0)) {
        let assign44820_e43333: f64 = (var_fn482_calc_ig__pg_paramin_hinj / var_fn482_calc_ig__phitin);
        let assign44820_e43335: f64 = (assign44820_e43333 * var_fn482_calc_ig__vgsatin);
        let assign44820_e43337: f64 = (assign44820_e43335 + var_fn482_calc_ig__expphib);
        (assign44820_e43337, (((-((var_fn482_calc_ig__pg_paramin_hinj * var_fn482_calc_ig__phitin_dn4) / (var_fn482_calc_ig__phitin * var_fn482_calc_ig__phitin))) * var_fn482_calc_ig__vgsatin) + var_fn482_calc_ig__expphib_dn4),)
    } else {
        (var_fn482_calc_ig__expiforarg_hinj_vgsat, var_fn482_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__expiforarg_hinj_vgsat = assign44820_e43339;
        var_fn482_calc_ig__expiforarg_hinj_vgsat_dn4 = assign44820_e43339_d_n4;

        let (assign44830_e43386, assign44830_e43386_d_n4,) = {
    if (((var_guard480 != 0.0) && (var_guard483 == 0.0)) && (var_guard484 != 0.0)) {
        let assign44830_e43352: f64 = (-50.0);
        let (assign44830_e43384, assign44830_e43384_d_n4,) = {
            if ((!(var_fn482_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (!(var_fn482_calc_ig__expiforarg_hinj_vgsat < assign44830_e43352))) {
                let assign44830_e43357: f64 = (var_fn482_calc_ig__expiforarg_hinj_vgsat).exp();
                (assign44830_e43357, (assign44830_e43357 * var_fn482_calc_ig__expiforarg_hinj_vgsat_dn4),)
            } else {
                let assign44830_e43364: f64 = (-50.0);
                let (assign44830_e43383, assign44830_e43383_d_n4,) = {
                    if ((!(var_fn482_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (var_fn482_calc_ig__expiforarg_hinj_vgsat < assign44830_e43364)) {
                        let assign44830_e43368: f64 = (-50.0);
                        let assign44830_e43369: f64 = (assign44830_e43368).exp();
                        (assign44830_e43369, 0.0,)
                    } else {
                        let (assign44830_e43382, assign44830_e43382_d_n4,) = {
                            if (var_fn482_calc_ig__expiforarg_hinj_vgsat > 50.0) {
                                let assign44830_e43374: f64 = (50.0_f64).exp();
                                let assign44830_e43378: f64 = (var_fn482_calc_ig__expiforarg_hinj_vgsat - 50.0);
                                let assign44830_e43379: f64 = (1.0 + assign44830_e43378);
                                let assign44830_e43380: f64 = (assign44830_e43374 * assign44830_e43379);
                                (assign44830_e43380, (assign44830_e43374 * var_fn482_calc_ig__expiforarg_hinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign44830_e43382, assign44830_e43382_d_n4,)
                    }
                };
                (assign44830_e43383, assign44830_e43383_d_n4,)
            }
        };
        (assign44830_e43384, assign44830_e43384_d_n4,)
    } else {
        (var_fn482_calc_ig__expifor_hinj_vgsat, var_fn482_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__expifor_hinj_vgsat = assign44830_e43386;
        var_fn482_calc_ig__expifor_hinj_vgsat_dn4 = assign44830_e43386_d_n4;

        let (assign44840_e43401, assign44840_e43401_d_n4,) = {
    if (((var_guard480 != 0.0) && (var_guard483 == 0.0)) && (var_guard484 != 0.0)) {
        let assign44840_e43396: f64 = (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_vgsat);
        let assign44840_e43397: f64 = (var_fn482_calc_ig__expifor_hinj_vgsat - assign44840_e43396);
        let assign44840_e43399: f64 = (assign44840_e43397 - var_fn482_calc_ig__t0);
        (assign44840_e43399, ((var_fn482_calc_ig__expifor_hinj_vgsat_dn4 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_vgsat_dn4)) - var_fn482_calc_ig__t0_dn4),)
    } else {
        (var_fn482_calc_ig__igindiode_hinj_vgsat, var_fn482_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn482_calc_ig__igindiode_hinj_vgsat = assign44840_e43401;
        var_fn482_calc_ig__igindiode_hinj_vgsat_dn4 = assign44840_e43401_d_n4;

        let (assign44850_e43416, assign44850_e43416_d_n0, assign44850_e43416_d_n2, assign44850_e43416_d_n4, assign44850_e43416_d_n8, assign44850_e43416_d_n18, assign44850_e43416_d_n19,) = {
    if (((var_guard480 != 0.0) && (var_guard483 == 0.0)) && (var_guard484 != 0.0)) {
        let assign44850_e43410: f64 = (var_fn482_calc_ig__pg_paramin_hinj / var_fn482_calc_ig__phitin);
        let assign44850_e43412: f64 = (assign44850_e43410 * var_fn482_calc_ig__vgin);
        let assign44850_e43414: f64 = (assign44850_e43412 + var_fn482_calc_ig__expphib);
        (assign44850_e43414, (assign44850_e43410 * var_fn482_calc_ig__vgin_dn0), (assign44850_e43410 * var_fn482_calc_ig__vgin_dn2), (((-((var_fn482_calc_ig__pg_paramin_hinj * var_fn482_calc_ig__phitin_dn4) / (var_fn482_calc_ig__phitin * var_fn482_calc_ig__phitin))) * var_fn482_calc_ig__vgin) + var_fn482_calc_ig__expphib_dn4), (assign44850_e43410 * var_fn482_calc_ig__vgin_dn8), (assign44850_e43410 * var_fn482_calc_ig__vgin_dn18), (assign44850_e43410 * var_fn482_calc_ig__vgin_dn19),)
    } else {
        (var_fn482_calc_ig__expiforarg_hinj, var_fn482_calc_ig__expiforarg_hinj_dn0, var_fn482_calc_ig__expiforarg_hinj_dn2, var_fn482_calc_ig__expiforarg_hinj_dn4, var_fn482_calc_ig__expiforarg_hinj_dn8, var_fn482_calc_ig__expiforarg_hinj_dn18, var_fn482_calc_ig__expiforarg_hinj_dn19,)
    }
};
        var_fn482_calc_ig__expiforarg_hinj = assign44850_e43416;
        var_fn482_calc_ig__expiforarg_hinj_dn0 = assign44850_e43416_d_n0;
        var_fn482_calc_ig__expiforarg_hinj_dn2 = assign44850_e43416_d_n2;
        var_fn482_calc_ig__expiforarg_hinj_dn4 = assign44850_e43416_d_n4;
        var_fn482_calc_ig__expiforarg_hinj_dn8 = assign44850_e43416_d_n8;
        var_fn482_calc_ig__expiforarg_hinj_dn18 = assign44850_e43416_d_n18;
        var_fn482_calc_ig__expiforarg_hinj_dn19 = assign44850_e43416_d_n19;

        let (assign44860_e43463, assign44860_e43463_d_n0, assign44860_e43463_d_n2, assign44860_e43463_d_n4, assign44860_e43463_d_n8, assign44860_e43463_d_n18, assign44860_e43463_d_n19,) = {
    if (((var_guard480 != 0.0) && (var_guard483 == 0.0)) && (var_guard484 != 0.0)) {
        let assign44860_e43429: f64 = (-50.0);
        let (assign44860_e43461, assign44860_e43461_d_n0, assign44860_e43461_d_n2, assign44860_e43461_d_n4, assign44860_e43461_d_n8, assign44860_e43461_d_n18, assign44860_e43461_d_n19,) = {
            if ((!(var_fn482_calc_ig__expiforarg_hinj > 50.0)) && (!(var_fn482_calc_ig__expiforarg_hinj < assign44860_e43429))) {
                let assign44860_e43434: f64 = (var_fn482_calc_ig__expiforarg_hinj).exp();
                (assign44860_e43434, (assign44860_e43434 * var_fn482_calc_ig__expiforarg_hinj_dn0), (assign44860_e43434 * var_fn482_calc_ig__expiforarg_hinj_dn2), (assign44860_e43434 * var_fn482_calc_ig__expiforarg_hinj_dn4), (assign44860_e43434 * var_fn482_calc_ig__expiforarg_hinj_dn8), (assign44860_e43434 * var_fn482_calc_ig__expiforarg_hinj_dn18), (assign44860_e43434 * var_fn482_calc_ig__expiforarg_hinj_dn19),)
            } else {
                let assign44860_e43441: f64 = (-50.0);
                let (assign44860_e43460, assign44860_e43460_d_n0, assign44860_e43460_d_n2, assign44860_e43460_d_n4, assign44860_e43460_d_n8, assign44860_e43460_d_n18, assign44860_e43460_d_n19,) = {
                    if ((!(var_fn482_calc_ig__expiforarg_hinj > 50.0)) && (var_fn482_calc_ig__expiforarg_hinj < assign44860_e43441)) {
                        let assign44860_e43445: f64 = (-50.0);
                        let assign44860_e43446: f64 = (assign44860_e43445).exp();
                        (assign44860_e43446, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign44860_e43459, assign44860_e43459_d_n0, assign44860_e43459_d_n2, assign44860_e43459_d_n4, assign44860_e43459_d_n8, assign44860_e43459_d_n18, assign44860_e43459_d_n19,) = {
                            if (var_fn482_calc_ig__expiforarg_hinj > 50.0) {
                                let assign44860_e43451: f64 = (50.0_f64).exp();
                                let assign44860_e43455: f64 = (var_fn482_calc_ig__expiforarg_hinj - 50.0);
                                let assign44860_e43456: f64 = (1.0 + assign44860_e43455);
                                let assign44860_e43457: f64 = (assign44860_e43451 * assign44860_e43456);
                                (assign44860_e43457, (assign44860_e43451 * var_fn482_calc_ig__expiforarg_hinj_dn0), (assign44860_e43451 * var_fn482_calc_ig__expiforarg_hinj_dn2), (assign44860_e43451 * var_fn482_calc_ig__expiforarg_hinj_dn4), (assign44860_e43451 * var_fn482_calc_ig__expiforarg_hinj_dn8), (assign44860_e43451 * var_fn482_calc_ig__expiforarg_hinj_dn18), (assign44860_e43451 * var_fn482_calc_ig__expiforarg_hinj_dn19),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign44860_e43459, assign44860_e43459_d_n0, assign44860_e43459_d_n2, assign44860_e43459_d_n4, assign44860_e43459_d_n8, assign44860_e43459_d_n18, assign44860_e43459_d_n19,)
                    }
                };
                (assign44860_e43460, assign44860_e43460_d_n0, assign44860_e43460_d_n2, assign44860_e43460_d_n4, assign44860_e43460_d_n8, assign44860_e43460_d_n18, assign44860_e43460_d_n19,)
            }
        };
        (assign44860_e43461, assign44860_e43461_d_n0, assign44860_e43461_d_n2, assign44860_e43461_d_n4, assign44860_e43461_d_n8, assign44860_e43461_d_n18, assign44860_e43461_d_n19,)
    } else {
        (var_fn482_calc_ig__expifor_hinj, var_fn482_calc_ig__expifor_hinj_dn0, var_fn482_calc_ig__expifor_hinj_dn2, var_fn482_calc_ig__expifor_hinj_dn4, var_fn482_calc_ig__expifor_hinj_dn8, var_fn482_calc_ig__expifor_hinj_dn18, var_fn482_calc_ig__expifor_hinj_dn19,)
    }
};
        var_fn482_calc_ig__expifor_hinj = assign44860_e43463;
        var_fn482_calc_ig__expifor_hinj_dn0 = assign44860_e43463_d_n0;
        var_fn482_calc_ig__expifor_hinj_dn2 = assign44860_e43463_d_n2;
        var_fn482_calc_ig__expifor_hinj_dn4 = assign44860_e43463_d_n4;
        var_fn482_calc_ig__expifor_hinj_dn8 = assign44860_e43463_d_n8;
        var_fn482_calc_ig__expifor_hinj_dn18 = assign44860_e43463_d_n18;
        var_fn482_calc_ig__expifor_hinj_dn19 = assign44860_e43463_d_n19;

        let (assign44870_e43476, assign44870_e43476_d_n4,) = {
    if (((var_guard480 != 0.0) && (var_guard483 == 0.0)) && (var_guard484 != 0.0)) {
        let assign44870_e43472: f64 = (var_fn482_calc_ig__isdiodeout * var_fn482_calc_ig__igindiode_nohinj_vgsat);
        let assign44870_e43474: f64 = (assign44870_e43472 / var_fn482_calc_ig__igindiode_hinj_vgsat);
        (assign44870_e43474, (((((var_fn482_calc_ig__isdiodeout_dn4 * var_fn482_calc_ig__igindiode_nohinj_vgsat) + (var_fn482_calc_ig__isdiodeout * var_fn482_calc_ig__igindiode_nohinj_vgsat_dn4)) * var_fn482_calc_ig__igindiode_hinj_vgsat) - (assign44870_e43472 * var_fn482_calc_ig__igindiode_hinj_vgsat_dn4)) / (var_fn482_calc_ig__igindiode_hinj_vgsat * var_fn482_calc_ig__igindiode_hinj_vgsat)),)
    } else {
        (var_fn482_calc_ig__igindiode_hinj_pre, var_fn482_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn482_calc_ig__igindiode_hinj_pre = assign44870_e43476;
        var_fn482_calc_ig__igindiode_hinj_pre_dn4 = assign44870_e43476_d_n4;

        let (assign44880_e43493, assign44880_e43493_d_n0, assign44880_e43493_d_n2, assign44880_e43493_d_n4, assign44880_e43493_d_n8, assign44880_e43493_d_n18, assign44880_e43493_d_n19,) = {
    if (((var_guard480 != 0.0) && (var_guard483 == 0.0)) && (var_guard484 != 0.0)) {
        let assign44880_e43487: f64 = (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd);
        let assign44880_e43488: f64 = (var_fn482_calc_ig__expifor_hinj - assign44880_e43487);
        let assign44880_e43490: f64 = (assign44880_e43488 - var_fn482_calc_ig__t0);
        let assign44880_e43491: f64 = (var_fn482_calc_ig__igindiode_hinj_pre * assign44880_e43490);
        (assign44880_e43491, (var_fn482_calc_ig__igindiode_hinj_pre * (var_fn482_calc_ig__expifor_hinj_dn0 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn0))), (var_fn482_calc_ig__igindiode_hinj_pre * (var_fn482_calc_ig__expifor_hinj_dn2 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn2))), ((var_fn482_calc_ig__igindiode_hinj_pre_dn4 * assign44880_e43490) + (var_fn482_calc_ig__igindiode_hinj_pre * ((var_fn482_calc_ig__expifor_hinj_dn4 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn4)) - var_fn482_calc_ig__t0_dn4))), (var_fn482_calc_ig__igindiode_hinj_pre * (var_fn482_calc_ig__expifor_hinj_dn8 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn8))), (var_fn482_calc_ig__igindiode_hinj_pre * (var_fn482_calc_ig__expifor_hinj_dn18 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn18))), (var_fn482_calc_ig__igindiode_hinj_pre * (var_fn482_calc_ig__expifor_hinj_dn19 - (var_fn482_calc_ig__kbdgatein * var_fn482_calc_ig__iginbd_dn19))),)
    } else {
        (var_fn482_calc_ig__igindiode_hinj, var_fn482_calc_ig__igindiode_hinj_dn0, var_fn482_calc_ig__igindiode_hinj_dn2, var_fn482_calc_ig__igindiode_hinj_dn4, var_fn482_calc_ig__igindiode_hinj_dn8, var_fn482_calc_ig__igindiode_hinj_dn18, var_fn482_calc_ig__igindiode_hinj_dn19,)
    }
};
        var_fn482_calc_ig__igindiode_hinj = assign44880_e43493;
        var_fn482_calc_ig__igindiode_hinj_dn0 = assign44880_e43493_d_n0;
        var_fn482_calc_ig__igindiode_hinj_dn2 = assign44880_e43493_d_n2;
        var_fn482_calc_ig__igindiode_hinj_dn4 = assign44880_e43493_d_n4;
        var_fn482_calc_ig__igindiode_hinj_dn8 = assign44880_e43493_d_n8;
        var_fn482_calc_ig__igindiode_hinj_dn18 = assign44880_e43493_d_n18;
        var_fn482_calc_ig__igindiode_hinj_dn19 = assign44880_e43493_d_n19;

        let (assign44890_e43505, assign44890_e43505_d_n0, assign44890_e43505_d_n2, assign44890_e43505_d_n4, assign44890_e43505_d_n8, assign44890_e43505_d_n18, assign44890_e43505_d_n19,) = {
    if (((var_guard480 != 0.0) && (var_guard483 == 0.0)) && (var_guard484 == 0.0)) {
        let assign44890_e43503: f64 = (var_fn482_calc_ig__isdiodeout * var_fn482_calc_ig__igindiode_nohinj_vgsat);
        (assign44890_e43503, 0.0, 0.0, ((var_fn482_calc_ig__isdiodeout_dn4 * var_fn482_calc_ig__igindiode_nohinj_vgsat) + (var_fn482_calc_ig__isdiodeout * var_fn482_calc_ig__igindiode_nohinj_vgsat_dn4)), 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__igindiode_hinj, var_fn482_calc_ig__igindiode_hinj_dn0, var_fn482_calc_ig__igindiode_hinj_dn2, var_fn482_calc_ig__igindiode_hinj_dn4, var_fn482_calc_ig__igindiode_hinj_dn8, var_fn482_calc_ig__igindiode_hinj_dn18, var_fn482_calc_ig__igindiode_hinj_dn19,)
    }
};
        var_fn482_calc_ig__igindiode_hinj = assign44890_e43505;
        var_fn482_calc_ig__igindiode_hinj_dn0 = assign44890_e43505_d_n0;
        var_fn482_calc_ig__igindiode_hinj_dn2 = assign44890_e43505_d_n2;
        var_fn482_calc_ig__igindiode_hinj_dn4 = assign44890_e43505_d_n4;
        var_fn482_calc_ig__igindiode_hinj_dn8 = assign44890_e43505_d_n8;
        var_fn482_calc_ig__igindiode_hinj_dn18 = assign44890_e43505_d_n18;
        var_fn482_calc_ig__igindiode_hinj_dn19 = assign44890_e43505_d_n19;

        let (assign44900_e43516, assign44900_e43516_d_n4,) = {
    if ((var_guard480 != 0.0) && (var_guard483 == 0.0)) {
        let assign44900_e43512: f64 = (var_fn482_calc_ig__alphagin * var_fn482_calc_ig__alphagin);
        let assign44900_e43514: f64 = (assign44900_e43512 * var_fn482_calc_ig__phitin);
        (assign44900_e43514, (assign44900_e43512 * var_fn482_calc_ig__phitin_dn4),)
    } else {
        (var_fn482_calc_ig__alpha2_phit, var_fn482_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn482_calc_ig__alpha2_phit = assign44900_e43516;
        var_fn482_calc_ig__alpha2_phit_dn4 = assign44900_e43516_d_n4;

        let (assign44910_e43531, assign44910_e43531_d_n0, assign44910_e43531_d_n2, assign44910_e43531_d_n4, assign44910_e43531_d_n8, assign44910_e43531_d_n18, assign44910_e43531_d_n19,) = {
    if ((var_guard480 != 0.0) && (var_guard483 == 0.0)) {
        let assign44910_e43525: f64 = (var_fn482_calc_ig__alpha2_phit / 2.0);
        let assign44910_e43526: f64 = (var_fn482_calc_ig__vgsatin - assign44910_e43525);
        let assign44910_e43527: f64 = (var_fn482_calc_ig__vgin - assign44910_e43526);
        let assign44910_e43529: f64 = (assign44910_e43527 / var_fn482_calc_ig__alpha2_phit);
        (assign44910_e43529, (var_fn482_calc_ig__vgin_dn0 / var_fn482_calc_ig__alpha2_phit), (var_fn482_calc_ig__vgin_dn2 / var_fn482_calc_ig__alpha2_phit), ((((-(-(var_fn482_calc_ig__alpha2_phit_dn4 / 2.0))) * var_fn482_calc_ig__alpha2_phit) - (assign44910_e43527 * var_fn482_calc_ig__alpha2_phit_dn4)) / (var_fn482_calc_ig__alpha2_phit * var_fn482_calc_ig__alpha2_phit)), (var_fn482_calc_ig__vgin_dn8 / var_fn482_calc_ig__alpha2_phit), (var_fn482_calc_ig__vgin_dn18 / var_fn482_calc_ig__alpha2_phit), (var_fn482_calc_ig__vgin_dn19 / var_fn482_calc_ig__alpha2_phit),)
    } else {
        (var_fn482_calc_ig__expffvarg, var_fn482_calc_ig__expffvarg_dn0, var_fn482_calc_ig__expffvarg_dn2, var_fn482_calc_ig__expffvarg_dn4, var_fn482_calc_ig__expffvarg_dn8, var_fn482_calc_ig__expffvarg_dn18, var_fn482_calc_ig__expffvarg_dn19,)
    }
};
        var_fn482_calc_ig__expffvarg = assign44910_e43531;
        var_fn482_calc_ig__expffvarg_dn0 = assign44910_e43531_d_n0;
        var_fn482_calc_ig__expffvarg_dn2 = assign44910_e43531_d_n2;
        var_fn482_calc_ig__expffvarg_dn4 = assign44910_e43531_d_n4;
        var_fn482_calc_ig__expffvarg_dn8 = assign44910_e43531_d_n8;
        var_fn482_calc_ig__expffvarg_dn18 = assign44910_e43531_d_n18;
        var_fn482_calc_ig__expffvarg_dn19 = assign44910_e43531_d_n19;

        let assign44920_e43534: f64 = if var_fn482_calc_ig__expffvarg > 50.0 { 1.0 } else { 0.0 };
        var_guard485 = assign44920_e43534;

        let (assign44930_e43543, assign44930_e43543_d_n0, assign44930_e43543_d_n2, assign44930_e43543_d_n4, assign44930_e43543_d_n8, assign44930_e43543_d_n18, assign44930_e43543_d_n19,) = {
    if (((var_guard480 != 0.0) && (var_guard483 == 0.0)) && (var_guard485 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__ffvgin, var_fn482_calc_ig__ffvgin_dn0, var_fn482_calc_ig__ffvgin_dn2, var_fn482_calc_ig__ffvgin_dn4, var_fn482_calc_ig__ffvgin_dn8, var_fn482_calc_ig__ffvgin_dn18, var_fn482_calc_ig__ffvgin_dn19,)
    }
};
        var_fn482_calc_ig__ffvgin = assign44930_e43543;
        var_fn482_calc_ig__ffvgin_dn0 = assign44930_e43543_d_n0;
        var_fn482_calc_ig__ffvgin_dn2 = assign44930_e43543_d_n2;
        var_fn482_calc_ig__ffvgin_dn4 = assign44930_e43543_d_n4;
        var_fn482_calc_ig__ffvgin_dn8 = assign44930_e43543_d_n8;
        var_fn482_calc_ig__ffvgin_dn18 = assign44930_e43543_d_n18;
        var_fn482_calc_ig__ffvgin_dn19 = assign44930_e43543_d_n19;

        let assign44940_e43546: f64 = (-50.0);
        let assign44940_e43547: f64 = if var_fn482_calc_ig__expffvarg < assign44940_e43546 { 1.0 } else { 0.0 };
        var_guard486 = assign44940_e43547;

        let (assign44950_e43559, assign44950_e43559_d_n0, assign44950_e43559_d_n2, assign44950_e43559_d_n4, assign44950_e43559_d_n8, assign44950_e43559_d_n18, assign44950_e43559_d_n19,) = {
    if ((((var_guard480 != 0.0) && (var_guard483 == 0.0)) && (var_guard485 == 0.0)) && (var_guard486 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn482_calc_ig__ffvgin, var_fn482_calc_ig__ffvgin_dn0, var_fn482_calc_ig__ffvgin_dn2, var_fn482_calc_ig__ffvgin_dn4, var_fn482_calc_ig__ffvgin_dn8, var_fn482_calc_ig__ffvgin_dn18, var_fn482_calc_ig__ffvgin_dn19,)
    }
};
        var_fn482_calc_ig__ffvgin = assign44950_e43559;
        var_fn482_calc_ig__ffvgin_dn0 = assign44950_e43559_d_n0;
        var_fn482_calc_ig__ffvgin_dn2 = assign44950_e43559_d_n2;
        var_fn482_calc_ig__ffvgin_dn4 = assign44950_e43559_d_n4;
        var_fn482_calc_ig__ffvgin_dn8 = assign44950_e43559_d_n8;
        var_fn482_calc_ig__ffvgin_dn18 = assign44950_e43559_d_n18;
        var_fn482_calc_ig__ffvgin_dn19 = assign44950_e43559_d_n19;

        let (assign44960_e43577, assign44960_e43577_d_n0, assign44960_e43577_d_n2, assign44960_e43577_d_n4, assign44960_e43577_d_n8, assign44960_e43577_d_n18, assign44960_e43577_d_n19,) = {
    if ((((var_guard480 != 0.0) && (var_guard483 == 0.0)) && (var_guard485 == 0.0)) && (var_guard486 == 0.0)) {
        let assign44960_e43573: f64 = (var_fn482_calc_ig__expffvarg).exp();
        let assign44960_e43574: f64 = (1.0 + assign44960_e43573);
        let assign44960_e43575: f64 = (1.0 / assign44960_e43574);
        (assign44960_e43575, (-((assign44960_e43573 * var_fn482_calc_ig__expffvarg_dn0) / (assign44960_e43574 * assign44960_e43574))), (-((assign44960_e43573 * var_fn482_calc_ig__expffvarg_dn2) / (assign44960_e43574 * assign44960_e43574))), (-((assign44960_e43573 * var_fn482_calc_ig__expffvarg_dn4) / (assign44960_e43574 * assign44960_e43574))), (-((assign44960_e43573 * var_fn482_calc_ig__expffvarg_dn8) / (assign44960_e43574 * assign44960_e43574))), (-((assign44960_e43573 * var_fn482_calc_ig__expffvarg_dn18) / (assign44960_e43574 * assign44960_e43574))), (-((assign44960_e43573 * var_fn482_calc_ig__expffvarg_dn19) / (assign44960_e43574 * assign44960_e43574))),)
    } else {
        (var_fn482_calc_ig__ffvgin, var_fn482_calc_ig__ffvgin_dn0, var_fn482_calc_ig__ffvgin_dn2, var_fn482_calc_ig__ffvgin_dn4, var_fn482_calc_ig__ffvgin_dn8, var_fn482_calc_ig__ffvgin_dn18, var_fn482_calc_ig__ffvgin_dn19,)
    }
};
        var_fn482_calc_ig__ffvgin = assign44960_e43577;
        var_fn482_calc_ig__ffvgin_dn0 = assign44960_e43577_d_n0;
        var_fn482_calc_ig__ffvgin_dn2 = assign44960_e43577_d_n2;
        var_fn482_calc_ig__ffvgin_dn4 = assign44960_e43577_d_n4;
        var_fn482_calc_ig__ffvgin_dn8 = assign44960_e43577_d_n8;
        var_fn482_calc_ig__ffvgin_dn18 = assign44960_e43577_d_n18;
        var_fn482_calc_ig__ffvgin_dn19 = assign44960_e43577_d_n19;

        let (assign44970_e43592, assign44970_e43592_d_n0, assign44970_e43592_d_n2, assign44970_e43592_d_n4, assign44970_e43592_d_n8, assign44970_e43592_d_n18, assign44970_e43592_d_n19,) = {
    if ((var_guard480 != 0.0) && (var_guard483 == 0.0)) {
        let assign44970_e43584: f64 = (var_fn482_calc_ig__ffvgin * var_fn482_calc_ig__igindiode_nohinj);
        let assign44970_e43587: f64 = (1.0 - var_fn482_calc_ig__ffvgin);
        let assign44970_e43589: f64 = (assign44970_e43587 * var_fn482_calc_ig__igindiode_hinj);
        let assign44970_e43590: f64 = (assign44970_e43584 + assign44970_e43589);
        (assign44970_e43590, (((var_fn482_calc_ig__ffvgin_dn0 * var_fn482_calc_ig__igindiode_nohinj) + (var_fn482_calc_ig__ffvgin * var_fn482_calc_ig__igindiode_nohinj_dn0)) + (((-var_fn482_calc_ig__ffvgin_dn0) * var_fn482_calc_ig__igindiode_hinj) + (assign44970_e43587 * var_fn482_calc_ig__igindiode_hinj_dn0))), (((var_fn482_calc_ig__ffvgin_dn2 * var_fn482_calc_ig__igindiode_nohinj) + (var_fn482_calc_ig__ffvgin * var_fn482_calc_ig__igindiode_nohinj_dn2)) + (((-var_fn482_calc_ig__ffvgin_dn2) * var_fn482_calc_ig__igindiode_hinj) + (assign44970_e43587 * var_fn482_calc_ig__igindiode_hinj_dn2))), (((var_fn482_calc_ig__ffvgin_dn4 * var_fn482_calc_ig__igindiode_nohinj) + (var_fn482_calc_ig__ffvgin * var_fn482_calc_ig__igindiode_nohinj_dn4)) + (((-var_fn482_calc_ig__ffvgin_dn4) * var_fn482_calc_ig__igindiode_hinj) + (assign44970_e43587 * var_fn482_calc_ig__igindiode_hinj_dn4))), (((var_fn482_calc_ig__ffvgin_dn8 * var_fn482_calc_ig__igindiode_nohinj) + (var_fn482_calc_ig__ffvgin * var_fn482_calc_ig__igindiode_nohinj_dn8)) + (((-var_fn482_calc_ig__ffvgin_dn8) * var_fn482_calc_ig__igindiode_hinj) + (assign44970_e43587 * var_fn482_calc_ig__igindiode_hinj_dn8))), (((var_fn482_calc_ig__ffvgin_dn18 * var_fn482_calc_ig__igindiode_nohinj) + (var_fn482_calc_ig__ffvgin * var_fn482_calc_ig__igindiode_nohinj_dn18)) + (((-var_fn482_calc_ig__ffvgin_dn18) * var_fn482_calc_ig__igindiode_hinj) + (assign44970_e43587 * var_fn482_calc_ig__igindiode_hinj_dn18))), (((var_fn482_calc_ig__ffvgin_dn19 * var_fn482_calc_ig__igindiode_nohinj) + (var_fn482_calc_ig__ffvgin * var_fn482_calc_ig__igindiode_nohinj_dn19)) + (((-var_fn482_calc_ig__ffvgin_dn19) * var_fn482_calc_ig__igindiode_hinj) + (assign44970_e43587 * var_fn482_calc_ig__igindiode_hinj_dn19))),)
    } else {
        (var_fn482_calc_ig__igindiode, var_fn482_calc_ig__igindiode_dn0, var_fn482_calc_ig__igindiode_dn2, var_fn482_calc_ig__igindiode_dn4, var_fn482_calc_ig__igindiode_dn8, var_fn482_calc_ig__igindiode_dn18, var_fn482_calc_ig__igindiode_dn19,)
    }
};
        var_fn482_calc_ig__igindiode = assign44970_e43592;
        var_fn482_calc_ig__igindiode_dn0 = assign44970_e43592_d_n0;
        var_fn482_calc_ig__igindiode_dn2 = assign44970_e43592_d_n2;
        var_fn482_calc_ig__igindiode_dn4 = assign44970_e43592_d_n4;
        var_fn482_calc_ig__igindiode_dn8 = assign44970_e43592_d_n8;
        var_fn482_calc_ig__igindiode_dn18 = assign44970_e43592_d_n18;
        var_fn482_calc_ig__igindiode_dn19 = assign44970_e43592_d_n19;

        let (assign44980_e43638, assign44980_e43638_d_n0, assign44980_e43638_d_n2, assign44980_e43638_d_n8, assign44980_e43638_d_n18, assign44980_e43638_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign44980_e43595: f64 = (-var_fn482_calc_ig__vgin);
        let (assign44980_e43628, assign44980_e43628_d_n0, assign44980_e43628_d_n2, assign44980_e43628_d_n8, assign44980_e43628_d_n18, assign44980_e43628_d_n19,) = {
            if (p.p52 != 0.0) {
                let assign44980_e43603: f64 = (var_fn482_calc_ig__vgin / var_fn482_calc_ig__vgsatqin);
                let assign44980_e43606: f64 = (0.001 / p.p53);
                let assign44980_e43609: f64 = (var_fn482_calc_ig__vgin / var_fn482_calc_ig__vgsatqin);
                let assign44980_e43610: f64 = (assign44980_e43606 * assign44980_e43609);
                let assign44980_e43611: f64 = (assign44980_e43610).tanh();
                let assign44980_e43612: f64 = (assign44980_e43603 * assign44980_e43611);
                (assign44980_e43612, (((var_fn482_calc_ig__vgin_dn0 / var_fn482_calc_ig__vgsatqin) * assign44980_e43611) + (assign44980_e43603 * ((assign44980_e43606 * (var_fn482_calc_ig__vgin_dn0 / var_fn482_calc_ig__vgsatqin)) / ((assign44980_e43610).cosh() * (assign44980_e43610).cosh())))), (((var_fn482_calc_ig__vgin_dn2 / var_fn482_calc_ig__vgsatqin) * assign44980_e43611) + (assign44980_e43603 * ((assign44980_e43606 * (var_fn482_calc_ig__vgin_dn2 / var_fn482_calc_ig__vgsatqin)) / ((assign44980_e43610).cosh() * (assign44980_e43610).cosh())))), (((var_fn482_calc_ig__vgin_dn8 / var_fn482_calc_ig__vgsatqin) * assign44980_e43611) + (assign44980_e43603 * ((assign44980_e43606 * (var_fn482_calc_ig__vgin_dn8 / var_fn482_calc_ig__vgsatqin)) / ((assign44980_e43610).cosh() * (assign44980_e43610).cosh())))), (((var_fn482_calc_ig__vgin_dn18 / var_fn482_calc_ig__vgsatqin) * assign44980_e43611) + (assign44980_e43603 * ((assign44980_e43606 * (var_fn482_calc_ig__vgin_dn18 / var_fn482_calc_ig__vgsatqin)) / ((assign44980_e43610).cosh() * (assign44980_e43610).cosh())))), (((var_fn482_calc_ig__vgin_dn19 / var_fn482_calc_ig__vgsatqin) * assign44980_e43611) + (assign44980_e43603 * ((assign44980_e43606 * (var_fn482_calc_ig__vgin_dn19 / var_fn482_calc_ig__vgsatqin)) / ((assign44980_e43610).cosh() * (assign44980_e43610).cosh())))),)
            } else {
                let (assign44980_e43627, assign44980_e43627_d_n0, assign44980_e43627_d_n2, assign44980_e43627_d_n8, assign44980_e43627_d_n18, assign44980_e43627_d_n19,) = {
                    if (p.p52 == 0.0) {
                        let __rspice_inv_cse_0: f64 = 1.0 / var_fn482_calc_ig__vgsatqin;
                        let assign44980_e43618: f64 = (var_fn482_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign44980_e43621: f64 = (var_fn482_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign44980_e43622: f64 = (assign44980_e43618 * assign44980_e43621);
                        let assign44980_e43624: f64 = (assign44980_e43622 + p.p53);
                        let assign44980_e43625: f64 = (assign44980_e43624).sqrt();
                        (assign44980_e43625, ((((var_fn482_calc_ig__vgin_dn0 / var_fn482_calc_ig__vgsatqin) * assign44980_e43621) + (assign44980_e43618 * (var_fn482_calc_ig__vgin_dn0 / var_fn482_calc_ig__vgsatqin))) / (2.0 * assign44980_e43625)), ((((var_fn482_calc_ig__vgin_dn2 / var_fn482_calc_ig__vgsatqin) * assign44980_e43621) + (assign44980_e43618 * (var_fn482_calc_ig__vgin_dn2 / var_fn482_calc_ig__vgsatqin))) / (2.0 * assign44980_e43625)), ((((var_fn482_calc_ig__vgin_dn8 / var_fn482_calc_ig__vgsatqin) * assign44980_e43621) + (assign44980_e43618 * (var_fn482_calc_ig__vgin_dn8 / var_fn482_calc_ig__vgsatqin))) / (2.0 * assign44980_e43625)), ((((var_fn482_calc_ig__vgin_dn18 / var_fn482_calc_ig__vgsatqin) * assign44980_e43621) + (assign44980_e43618 * (var_fn482_calc_ig__vgin_dn18 / var_fn482_calc_ig__vgsatqin))) / (2.0 * assign44980_e43625)), ((((var_fn482_calc_ig__vgin_dn19 / var_fn482_calc_ig__vgsatqin) * assign44980_e43621) + (assign44980_e43618 * (var_fn482_calc_ig__vgin_dn19 / var_fn482_calc_ig__vgsatqin))) / (2.0 * assign44980_e43625)),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign44980_e43627, assign44980_e43627_d_n0, assign44980_e43627_d_n2, assign44980_e43627_d_n8, assign44980_e43627_d_n18, assign44980_e43627_d_n19,)
            }
        };
        let assign44980_e43630: f64 = (assign44980_e43628).powf(var_fn482_calc_ig__betarecin);
        let assign44980_e43631: f64 = (1.0 + assign44980_e43630);
        let assign44980_e43634: f64 = (1.0 / var_fn482_calc_ig__betarecin);
        let assign44980_e43635: f64 = (assign44980_e43631).powf(assign44980_e43634);
        let assign44980_e43636: f64 = (assign44980_e43595 / assign44980_e43635);
        (assign44980_e43636, ((((-var_fn482_calc_ig__vgin_dn0) * assign44980_e43635) - (assign44980_e43595 * if 0.0 == 0.0 && ((assign44980_e43634) as f64).is_finite() && ((assign44980_e43634) as f64).fract() == 0.0 { if assign44980_e43634 == 0.0 { 0.0 } else { (assign44980_e43634 * ((assign44980_e43631).powf(assign44980_e43634 - 1.0) * if 0.0 == 0.0 && ((var_fn482_calc_ig__betarecin) as f64).is_finite() && ((var_fn482_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn482_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn482_calc_ig__betarecin * ((assign44980_e43628).powf(var_fn482_calc_ig__betarecin - 1.0) * assign44980_e43628_d_n0)) } } else { (assign44980_e43630 * (var_fn482_calc_ig__betarecin * (assign44980_e43628_d_n0 / assign44980_e43628))) })) } } else { (assign44980_e43635 * (assign44980_e43634 * (if 0.0 == 0.0 && ((var_fn482_calc_ig__betarecin) as f64).is_finite() && ((var_fn482_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn482_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn482_calc_ig__betarecin * ((assign44980_e43628).powf(var_fn482_calc_ig__betarecin - 1.0) * assign44980_e43628_d_n0)) } } else { (assign44980_e43630 * (var_fn482_calc_ig__betarecin * (assign44980_e43628_d_n0 / assign44980_e43628))) } / assign44980_e43631))) })) / (assign44980_e43635 * assign44980_e43635)), ((((-var_fn482_calc_ig__vgin_dn2) * assign44980_e43635) - (assign44980_e43595 * if 0.0 == 0.0 && ((assign44980_e43634) as f64).is_finite() && ((assign44980_e43634) as f64).fract() == 0.0 { if assign44980_e43634 == 0.0 { 0.0 } else { (assign44980_e43634 * ((assign44980_e43631).powf(assign44980_e43634 - 1.0) * if 0.0 == 0.0 && ((var_fn482_calc_ig__betarecin) as f64).is_finite() && ((var_fn482_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn482_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn482_calc_ig__betarecin * ((assign44980_e43628).powf(var_fn482_calc_ig__betarecin - 1.0) * assign44980_e43628_d_n2)) } } else { (assign44980_e43630 * (var_fn482_calc_ig__betarecin * (assign44980_e43628_d_n2 / assign44980_e43628))) })) } } else { (assign44980_e43635 * (assign44980_e43634 * (if 0.0 == 0.0 && ((var_fn482_calc_ig__betarecin) as f64).is_finite() && ((var_fn482_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn482_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn482_calc_ig__betarecin * ((assign44980_e43628).powf(var_fn482_calc_ig__betarecin - 1.0) * assign44980_e43628_d_n2)) } } else { (assign44980_e43630 * (var_fn482_calc_ig__betarecin * (assign44980_e43628_d_n2 / assign44980_e43628))) } / assign44980_e43631))) })) / (assign44980_e43635 * assign44980_e43635)), ((((-var_fn482_calc_ig__vgin_dn8) * assign44980_e43635) - (assign44980_e43595 * if 0.0 == 0.0 && ((assign44980_e43634) as f64).is_finite() && ((assign44980_e43634) as f64).fract() == 0.0 { if assign44980_e43634 == 0.0 { 0.0 } else { (assign44980_e43634 * ((assign44980_e43631).powf(assign44980_e43634 - 1.0) * if 0.0 == 0.0 && ((var_fn482_calc_ig__betarecin) as f64).is_finite() && ((var_fn482_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn482_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn482_calc_ig__betarecin * ((assign44980_e43628).powf(var_fn482_calc_ig__betarecin - 1.0) * assign44980_e43628_d_n8)) } } else { (assign44980_e43630 * (var_fn482_calc_ig__betarecin * (assign44980_e43628_d_n8 / assign44980_e43628))) })) } } else { (assign44980_e43635 * (assign44980_e43634 * (if 0.0 == 0.0 && ((var_fn482_calc_ig__betarecin) as f64).is_finite() && ((var_fn482_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn482_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn482_calc_ig__betarecin * ((assign44980_e43628).powf(var_fn482_calc_ig__betarecin - 1.0) * assign44980_e43628_d_n8)) } } else { (assign44980_e43630 * (var_fn482_calc_ig__betarecin * (assign44980_e43628_d_n8 / assign44980_e43628))) } / assign44980_e43631))) })) / (assign44980_e43635 * assign44980_e43635)), ((((-var_fn482_calc_ig__vgin_dn18) * assign44980_e43635) - (assign44980_e43595 * if 0.0 == 0.0 && ((assign44980_e43634) as f64).is_finite() && ((assign44980_e43634) as f64).fract() == 0.0 { if assign44980_e43634 == 0.0 { 0.0 } else { (assign44980_e43634 * ((assign44980_e43631).powf(assign44980_e43634 - 1.0) * if 0.0 == 0.0 && ((var_fn482_calc_ig__betarecin) as f64).is_finite() && ((var_fn482_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn482_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn482_calc_ig__betarecin * ((assign44980_e43628).powf(var_fn482_calc_ig__betarecin - 1.0) * assign44980_e43628_d_n18)) } } else { (assign44980_e43630 * (var_fn482_calc_ig__betarecin * (assign44980_e43628_d_n18 / assign44980_e43628))) })) } } else { (assign44980_e43635 * (assign44980_e43634 * (if 0.0 == 0.0 && ((var_fn482_calc_ig__betarecin) as f64).is_finite() && ((var_fn482_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn482_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn482_calc_ig__betarecin * ((assign44980_e43628).powf(var_fn482_calc_ig__betarecin - 1.0) * assign44980_e43628_d_n18)) } } else { (assign44980_e43630 * (var_fn482_calc_ig__betarecin * (assign44980_e43628_d_n18 / assign44980_e43628))) } / assign44980_e43631))) })) / (assign44980_e43635 * assign44980_e43635)), ((((-var_fn482_calc_ig__vgin_dn19) * assign44980_e43635) - (assign44980_e43595 * if 0.0 == 0.0 && ((assign44980_e43634) as f64).is_finite() && ((assign44980_e43634) as f64).fract() == 0.0 { if assign44980_e43634 == 0.0 { 0.0 } else { (assign44980_e43634 * ((assign44980_e43631).powf(assign44980_e43634 - 1.0) * if 0.0 == 0.0 && ((var_fn482_calc_ig__betarecin) as f64).is_finite() && ((var_fn482_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn482_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn482_calc_ig__betarecin * ((assign44980_e43628).powf(var_fn482_calc_ig__betarecin - 1.0) * assign44980_e43628_d_n19)) } } else { (assign44980_e43630 * (var_fn482_calc_ig__betarecin * (assign44980_e43628_d_n19 / assign44980_e43628))) })) } } else { (assign44980_e43635 * (assign44980_e43634 * (if 0.0 == 0.0 && ((var_fn482_calc_ig__betarecin) as f64).is_finite() && ((var_fn482_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn482_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn482_calc_ig__betarecin * ((assign44980_e43628).powf(var_fn482_calc_ig__betarecin - 1.0) * assign44980_e43628_d_n19)) } } else { (assign44980_e43630 * (var_fn482_calc_ig__betarecin * (assign44980_e43628_d_n19 / assign44980_e43628))) } / assign44980_e43631))) })) / (assign44980_e43635 * assign44980_e43635)),)
    } else {
        (var_fn482_calc_ig__frecgin, var_fn482_calc_ig__frecgin_dn0, var_fn482_calc_ig__frecgin_dn2, var_fn482_calc_ig__frecgin_dn8, var_fn482_calc_ig__frecgin_dn18, var_fn482_calc_ig__frecgin_dn19,)
    }
};
        var_fn482_calc_ig__frecgin = assign44980_e43638;
        var_fn482_calc_ig__frecgin_dn0 = assign44980_e43638_d_n0;
        var_fn482_calc_ig__frecgin_dn2 = assign44980_e43638_d_n2;
        var_fn482_calc_ig__frecgin_dn8 = assign44980_e43638_d_n8;
        var_fn482_calc_ig__frecgin_dn18 = assign44980_e43638_d_n18;
        var_fn482_calc_ig__frecgin_dn19 = assign44980_e43638_d_n19;

        let (assign44990_e43653, assign44990_e43653_d_n4,) = {
    if (var_guard480 != 0.0) {
        let assign44990_e43641: f64 = (-var_fn482_calc_ig__type);
        let assign44990_e43643: f64 = (assign44990_e43641 * var_fn482_calc_ig__w);
        let assign44990_e43645: f64 = (assign44990_e43643 * var_fn482_calc_ig__ngf);
        let assign44990_e43647: f64 = (assign44990_e43645 * var_fn482_calc_ig__irecin);
        let assign44990_e43649: f64 = (assign44990_e43647 * var_fn482_calc_ig__tfacdiodein);
        let assign44990_e43651: f64 = assign44990_e43649;
        (assign44990_e43651, (assign44990_e43647 * var_fn482_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn482_calc_ig__isrecout, var_fn482_calc_ig__isrecout_dn4,)
    }
};
        var_fn482_calc_ig__isrecout = assign44990_e43653;
        var_fn482_calc_ig__isrecout_dn4 = assign44990_e43653_d_n4;

        let (assign45000_e43661, assign45000_e43661_d_n0, assign45000_e43661_d_n2, assign45000_e43661_d_n4, assign45000_e43661_d_n8, assign45000_e43661_d_n18, assign45000_e43661_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign45000_e43657: f64 = (var_fn482_calc_ig__pgsrecin / var_fn482_calc_ig__phitin);
        let assign45000_e43659: f64 = (assign45000_e43657 * var_fn482_calc_ig__frecgin);
        (assign45000_e43659, (assign45000_e43657 * var_fn482_calc_ig__frecgin_dn0), (assign45000_e43657 * var_fn482_calc_ig__frecgin_dn2), ((-((var_fn482_calc_ig__pgsrecin * var_fn482_calc_ig__phitin_dn4) / (var_fn482_calc_ig__phitin * var_fn482_calc_ig__phitin))) * var_fn482_calc_ig__frecgin), (assign45000_e43657 * var_fn482_calc_ig__frecgin_dn8), (assign45000_e43657 * var_fn482_calc_ig__frecgin_dn18), (assign45000_e43657 * var_fn482_calc_ig__frecgin_dn19),)
    } else {
        (var_fn482_calc_ig__expirevarg, var_fn482_calc_ig__expirevarg_dn0, var_fn482_calc_ig__expirevarg_dn2, var_fn482_calc_ig__expirevarg_dn4, var_fn482_calc_ig__expirevarg_dn8, var_fn482_calc_ig__expirevarg_dn18, var_fn482_calc_ig__expirevarg_dn19,)
    }
};
        var_fn482_calc_ig__expirevarg = assign45000_e43661;
        var_fn482_calc_ig__expirevarg_dn0 = assign45000_e43661_d_n0;
        var_fn482_calc_ig__expirevarg_dn2 = assign45000_e43661_d_n2;
        var_fn482_calc_ig__expirevarg_dn4 = assign45000_e43661_d_n4;
        var_fn482_calc_ig__expirevarg_dn8 = assign45000_e43661_d_n8;
        var_fn482_calc_ig__expirevarg_dn18 = assign45000_e43661_d_n18;
        var_fn482_calc_ig__expirevarg_dn19 = assign45000_e43661_d_n19;

        *var_fn482_calc_ig__alpha2_phit_slot = var_fn482_calc_ig__alpha2_phit;
        *var_fn482_calc_ig__alpha2_phit_dn4_slot = var_fn482_calc_ig__alpha2_phit_dn4;
        *var_fn482_calc_ig__expffvarg_slot = var_fn482_calc_ig__expffvarg;
        *var_fn482_calc_ig__expffvarg_dn0_slot = var_fn482_calc_ig__expffvarg_dn0;
        *var_fn482_calc_ig__expffvarg_dn18_slot = var_fn482_calc_ig__expffvarg_dn18;
        *var_fn482_calc_ig__expffvarg_dn19_slot = var_fn482_calc_ig__expffvarg_dn19;
        *var_fn482_calc_ig__expffvarg_dn2_slot = var_fn482_calc_ig__expffvarg_dn2;
        *var_fn482_calc_ig__expffvarg_dn4_slot = var_fn482_calc_ig__expffvarg_dn4;
        *var_fn482_calc_ig__expffvarg_dn8_slot = var_fn482_calc_ig__expffvarg_dn8;
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
        *var_fn482_calc_ig__expirevarg_slot = var_fn482_calc_ig__expirevarg;
        *var_fn482_calc_ig__expirevarg_dn0_slot = var_fn482_calc_ig__expirevarg_dn0;
        *var_fn482_calc_ig__expirevarg_dn18_slot = var_fn482_calc_ig__expirevarg_dn18;
        *var_fn482_calc_ig__expirevarg_dn19_slot = var_fn482_calc_ig__expirevarg_dn19;
        *var_fn482_calc_ig__expirevarg_dn2_slot = var_fn482_calc_ig__expirevarg_dn2;
        *var_fn482_calc_ig__expirevarg_dn4_slot = var_fn482_calc_ig__expirevarg_dn4;
        *var_fn482_calc_ig__expirevarg_dn8_slot = var_fn482_calc_ig__expirevarg_dn8;
        *var_fn482_calc_ig__ffvgin_slot = var_fn482_calc_ig__ffvgin;
        *var_fn482_calc_ig__ffvgin_dn0_slot = var_fn482_calc_ig__ffvgin_dn0;
        *var_fn482_calc_ig__ffvgin_dn18_slot = var_fn482_calc_ig__ffvgin_dn18;
        *var_fn482_calc_ig__ffvgin_dn19_slot = var_fn482_calc_ig__ffvgin_dn19;
        *var_fn482_calc_ig__ffvgin_dn2_slot = var_fn482_calc_ig__ffvgin_dn2;
        *var_fn482_calc_ig__ffvgin_dn4_slot = var_fn482_calc_ig__ffvgin_dn4;
        *var_fn482_calc_ig__ffvgin_dn8_slot = var_fn482_calc_ig__ffvgin_dn8;
        *var_fn482_calc_ig__frecgin_slot = var_fn482_calc_ig__frecgin;
        *var_fn482_calc_ig__frecgin_dn0_slot = var_fn482_calc_ig__frecgin_dn0;
        *var_fn482_calc_ig__frecgin_dn18_slot = var_fn482_calc_ig__frecgin_dn18;
        *var_fn482_calc_ig__frecgin_dn19_slot = var_fn482_calc_ig__frecgin_dn19;
        *var_fn482_calc_ig__frecgin_dn2_slot = var_fn482_calc_ig__frecgin_dn2;
        *var_fn482_calc_ig__frecgin_dn8_slot = var_fn482_calc_ig__frecgin_dn8;
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
        *var_fn482_calc_ig__isrecout_slot = var_fn482_calc_ig__isrecout;
        *var_fn482_calc_ig__isrecout_dn4_slot = var_fn482_calc_ig__isrecout_dn4;
        *var_fn482_calc_ig__pg_paramin_hinj_slot = var_fn482_calc_ig__pg_paramin_hinj;
        *var_guard484_slot = var_guard484;
        *var_guard485_slot = var_guard485;
        *var_guard486_slot = var_guard486;
    }

    pub(super) fn stamp_transient_block_113(
        p: &Parameters,
        var_fn482_calc_ig__expirevarg: f64,
        var_fn482_calc_ig__expirevarg_dn0: f64,
        var_fn482_calc_ig__expirevarg_dn18: f64,
        var_fn482_calc_ig__expirevarg_dn19: f64,
        var_fn482_calc_ig__expirevarg_dn2: f64,
        var_fn482_calc_ig__expirevarg_dn4: f64,
        var_fn482_calc_ig__expirevarg_dn8: f64,
        var_fn482_calc_ig__igindiode: f64,
        var_fn482_calc_ig__igindiode_dn0: f64,
        var_fn482_calc_ig__igindiode_dn18: f64,
        var_fn482_calc_ig__igindiode_dn19: f64,
        var_fn482_calc_ig__igindiode_dn2: f64,
        var_fn482_calc_ig__igindiode_dn4: f64,
        var_fn482_calc_ig__igindiode_dn8: f64,
        var_fn482_calc_ig__isrecout: f64,
        var_fn482_calc_ig__isrecout_dn4: f64,
        var_guard480: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_tfacdiode: f64,
        var_tfacdiode_dn4: f64,
        var_vindcbd: f64,
        var_vindcbd_dn0: f64,
        var_vindcbd_dn18: f64,
        var_vindcbd_dn19: f64,
        var_vindcbd_dn2: f64,
        var_vindcbd_dn8: f64,
        var_fn482_calc_ig__expirev_slot: &mut f64,
        var_fn482_calc_ig__expirev_dn0_slot: &mut f64,
        var_fn482_calc_ig__expirev_dn18_slot: &mut f64,
        var_fn482_calc_ig__expirev_dn19_slot: &mut f64,
        var_fn482_calc_ig__expirev_dn2_slot: &mut f64,
        var_fn482_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn482_calc_ig__expirev_dn8_slot: &mut f64,
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
        var_fn482_calc_ig__return_slot: &mut f64,
        var_fn482_calc_ig__return_dn0_slot: &mut f64,
        var_fn482_calc_ig__return_dn18_slot: &mut f64,
        var_fn482_calc_ig__return_dn19_slot: &mut f64,
        var_fn482_calc_ig__return_dn2_slot: &mut f64,
        var_fn482_calc_ig__return_dn4_slot: &mut f64,
        var_fn482_calc_ig__return_dn8_slot: &mut f64,
        var_fn487_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn487_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn487_calc_ig__alphagin_slot: &mut f64,
        var_fn487_calc_ig__betarecin_slot: &mut f64,
        var_fn487_calc_ig__expbd1_slot: &mut f64,
        var_fn487_calc_ig__expbd1_dn0_slot: &mut f64,
        var_fn487_calc_ig__expbd1_dn18_slot: &mut f64,
        var_fn487_calc_ig__expbd1_dn19_slot: &mut f64,
        var_fn487_calc_ig__expbd1_dn2_slot: &mut f64,
        var_fn487_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn487_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn487_calc_ig__expbd2_slot: &mut f64,
        var_fn487_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_dn0_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_dn18_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_dn19_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_dn2_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn487_calc_ig__expbdarg2_slot: &mut f64,
        var_fn487_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn487_calc_ig__expphib_slot: &mut f64,
        var_fn487_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn487_calc_ig__ffvgin_slot: &mut f64,
        var_fn487_calc_ig__ffvgin_dn0_slot: &mut f64,
        var_fn487_calc_ig__ffvgin_dn18_slot: &mut f64,
        var_fn487_calc_ig__ffvgin_dn19_slot: &mut f64,
        var_fn487_calc_ig__ffvgin_dn2_slot: &mut f64,
        var_fn487_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn487_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn487_calc_ig__fracin_slot: &mut f64,
        var_fn487_calc_ig__frecgin_slot: &mut f64,
        var_fn487_calc_ig__frecgin_dn0_slot: &mut f64,
        var_fn487_calc_ig__frecgin_dn18_slot: &mut f64,
        var_fn487_calc_ig__frecgin_dn19_slot: &mut f64,
        var_fn487_calc_ig__frecgin_dn2_slot: &mut f64,
        var_fn487_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn487_calc_ig__iginbd_slot: &mut f64,
        var_fn487_calc_ig__iginbd_dn0_slot: &mut f64,
        var_fn487_calc_ig__iginbd_dn18_slot: &mut f64,
        var_fn487_calc_ig__iginbd_dn19_slot: &mut f64,
        var_fn487_calc_ig__iginbd_dn2_slot: &mut f64,
        var_fn487_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn487_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn487_calc_ig__igindiode_slot: &mut f64,
        var_fn487_calc_ig__igindiode_dn0_slot: &mut f64,
        var_fn487_calc_ig__igindiode_dn18_slot: &mut f64,
        var_fn487_calc_ig__igindiode_dn19_slot: &mut f64,
        var_fn487_calc_ig__igindiode_dn2_slot: &mut f64,
        var_fn487_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn487_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn487_calc_ig__iginrec_slot: &mut f64,
        var_fn487_calc_ig__iginrec_dn0_slot: &mut f64,
        var_fn487_calc_ig__iginrec_dn18_slot: &mut f64,
        var_fn487_calc_ig__iginrec_dn19_slot: &mut f64,
        var_fn487_calc_ig__iginrec_dn2_slot: &mut f64,
        var_fn487_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn487_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn487_calc_ig__igout_slot: &mut f64,
        var_fn487_calc_ig__igout_dn0_slot: &mut f64,
        var_fn487_calc_ig__igout_dn18_slot: &mut f64,
        var_fn487_calc_ig__igout_dn19_slot: &mut f64,
        var_fn487_calc_ig__igout_dn2_slot: &mut f64,
        var_fn487_calc_ig__igout_dn4_slot: &mut f64,
        var_fn487_calc_ig__igout_dn8_slot: &mut f64,
        var_fn487_calc_ig__ijin_slot: &mut f64,
        var_fn487_calc_ig__irecin_slot: &mut f64,
        var_fn487_calc_ig__isdiodeout_slot: &mut f64,
        var_fn487_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn487_calc_ig__isrecout_slot: &mut f64,
        var_fn487_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn487_calc_ig__kbdgatein_slot: &mut f64,
        var_fn487_calc_ig__ngf_slot: &mut f64,
        var_fn487_calc_ig__pbdgin_slot: &mut f64,
        var_fn487_calc_ig__pg_param1_slot: &mut f64,
        var_fn487_calc_ig__pg_paramin_slot: &mut f64,
        var_fn487_calc_ig__pgsrecin_slot: &mut f64,
        var_fn487_calc_ig__phitin_slot: &mut f64,
        var_fn487_calc_ig__phitin_dn4_slot: &mut f64,
        var_fn487_calc_ig__return_slot: &mut f64,
        var_fn487_calc_ig__return_dn0_slot: &mut f64,
        var_fn487_calc_ig__return_dn18_slot: &mut f64,
        var_fn487_calc_ig__return_dn19_slot: &mut f64,
        var_fn487_calc_ig__return_dn2_slot: &mut f64,
        var_fn487_calc_ig__return_dn4_slot: &mut f64,
        var_fn487_calc_ig__return_dn8_slot: &mut f64,
        var_fn487_calc_ig__t0_slot: &mut f64,
        var_fn487_calc_ig__t0_dn4_slot: &mut f64,
        var_fn487_calc_ig__tfacdiodein_slot: &mut f64,
        var_fn487_calc_ig__tfacdiodein_dn4_slot: &mut f64,
        var_fn487_calc_ig__type_slot: &mut f64,
        var_fn487_calc_ig__vbdgin_slot: &mut f64,
        var_fn487_calc_ig__vgin_slot: &mut f64,
        var_fn487_calc_ig__vgin_dn0_slot: &mut f64,
        var_fn487_calc_ig__vgin_dn18_slot: &mut f64,
        var_fn487_calc_ig__vgin_dn19_slot: &mut f64,
        var_fn487_calc_ig__vgin_dn2_slot: &mut f64,
        var_fn487_calc_ig__vgin_dn8_slot: &mut f64,
        var_fn487_calc_ig__vgsatin_slot: &mut f64,
        var_fn487_calc_ig__vgsatqin_slot: &mut f64,
        var_fn487_calc_ig__vjg_slot: &mut f64,
        var_fn487_calc_ig__w_slot: &mut f64,
        var_igscbd_slot: &mut f64,
        var_igscbd_dn0_slot: &mut f64,
        var_igscbd_dn18_slot: &mut f64,
        var_igscbd_dn19_slot: &mut f64,
        var_igscbd_dn2_slot: &mut f64,
        var_igscbd_dn4_slot: &mut f64,
        var_igscbd_dn8_slot: &mut f64,
    ) {
        let mut var_fn482_calc_ig__expirev: f64 = *var_fn482_calc_ig__expirev_slot;
        let mut var_fn482_calc_ig__expirev_dn0: f64 = *var_fn482_calc_ig__expirev_dn0_slot;
        let mut var_fn482_calc_ig__expirev_dn18: f64 = *var_fn482_calc_ig__expirev_dn18_slot;
        let mut var_fn482_calc_ig__expirev_dn19: f64 = *var_fn482_calc_ig__expirev_dn19_slot;
        let mut var_fn482_calc_ig__expirev_dn2: f64 = *var_fn482_calc_ig__expirev_dn2_slot;
        let mut var_fn482_calc_ig__expirev_dn4: f64 = *var_fn482_calc_ig__expirev_dn4_slot;
        let mut var_fn482_calc_ig__expirev_dn8: f64 = *var_fn482_calc_ig__expirev_dn8_slot;
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
        let mut var_fn482_calc_ig__return: f64 = *var_fn482_calc_ig__return_slot;
        let mut var_fn482_calc_ig__return_dn0: f64 = *var_fn482_calc_ig__return_dn0_slot;
        let mut var_fn482_calc_ig__return_dn18: f64 = *var_fn482_calc_ig__return_dn18_slot;
        let mut var_fn482_calc_ig__return_dn19: f64 = *var_fn482_calc_ig__return_dn19_slot;
        let mut var_fn482_calc_ig__return_dn2: f64 = *var_fn482_calc_ig__return_dn2_slot;
        let mut var_fn482_calc_ig__return_dn4: f64 = *var_fn482_calc_ig__return_dn4_slot;
        let mut var_fn482_calc_ig__return_dn8: f64 = *var_fn482_calc_ig__return_dn8_slot;
        let mut var_fn487_calc_ig__alpha2_phit: f64 = *var_fn487_calc_ig__alpha2_phit_slot;
        let mut var_fn487_calc_ig__alpha2_phit_dn4: f64 = *var_fn487_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn487_calc_ig__alphagin: f64 = *var_fn487_calc_ig__alphagin_slot;
        let mut var_fn487_calc_ig__betarecin: f64 = *var_fn487_calc_ig__betarecin_slot;
        let mut var_fn487_calc_ig__expbd1: f64 = *var_fn487_calc_ig__expbd1_slot;
        let mut var_fn487_calc_ig__expbd1_dn0: f64 = *var_fn487_calc_ig__expbd1_dn0_slot;
        let mut var_fn487_calc_ig__expbd1_dn18: f64 = *var_fn487_calc_ig__expbd1_dn18_slot;
        let mut var_fn487_calc_ig__expbd1_dn19: f64 = *var_fn487_calc_ig__expbd1_dn19_slot;
        let mut var_fn487_calc_ig__expbd1_dn2: f64 = *var_fn487_calc_ig__expbd1_dn2_slot;
        let mut var_fn487_calc_ig__expbd1_dn4: f64 = *var_fn487_calc_ig__expbd1_dn4_slot;
        let mut var_fn487_calc_ig__expbd1_dn8: f64 = *var_fn487_calc_ig__expbd1_dn8_slot;
        let mut var_fn487_calc_ig__expbd2: f64 = *var_fn487_calc_ig__expbd2_slot;
        let mut var_fn487_calc_ig__expbd2_dn4: f64 = *var_fn487_calc_ig__expbd2_dn4_slot;
        let mut var_fn487_calc_ig__expbdarg1: f64 = *var_fn487_calc_ig__expbdarg1_slot;
        let mut var_fn487_calc_ig__expbdarg1_dn0: f64 = *var_fn487_calc_ig__expbdarg1_dn0_slot;
        let mut var_fn487_calc_ig__expbdarg1_dn18: f64 = *var_fn487_calc_ig__expbdarg1_dn18_slot;
        let mut var_fn487_calc_ig__expbdarg1_dn19: f64 = *var_fn487_calc_ig__expbdarg1_dn19_slot;
        let mut var_fn487_calc_ig__expbdarg1_dn2: f64 = *var_fn487_calc_ig__expbdarg1_dn2_slot;
        let mut var_fn487_calc_ig__expbdarg1_dn4: f64 = *var_fn487_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn487_calc_ig__expbdarg1_dn8: f64 = *var_fn487_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn487_calc_ig__expbdarg2: f64 = *var_fn487_calc_ig__expbdarg2_slot;
        let mut var_fn487_calc_ig__expbdarg2_dn4: f64 = *var_fn487_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn487_calc_ig__expphib: f64 = *var_fn487_calc_ig__expphib_slot;
        let mut var_fn487_calc_ig__expphib_dn4: f64 = *var_fn487_calc_ig__expphib_dn4_slot;
        let mut var_fn487_calc_ig__ffvgin: f64 = *var_fn487_calc_ig__ffvgin_slot;
        let mut var_fn487_calc_ig__ffvgin_dn0: f64 = *var_fn487_calc_ig__ffvgin_dn0_slot;
        let mut var_fn487_calc_ig__ffvgin_dn18: f64 = *var_fn487_calc_ig__ffvgin_dn18_slot;
        let mut var_fn487_calc_ig__ffvgin_dn19: f64 = *var_fn487_calc_ig__ffvgin_dn19_slot;
        let mut var_fn487_calc_ig__ffvgin_dn2: f64 = *var_fn487_calc_ig__ffvgin_dn2_slot;
        let mut var_fn487_calc_ig__ffvgin_dn4: f64 = *var_fn487_calc_ig__ffvgin_dn4_slot;
        let mut var_fn487_calc_ig__ffvgin_dn8: f64 = *var_fn487_calc_ig__ffvgin_dn8_slot;
        let mut var_fn487_calc_ig__fracin: f64 = *var_fn487_calc_ig__fracin_slot;
        let mut var_fn487_calc_ig__frecgin: f64 = *var_fn487_calc_ig__frecgin_slot;
        let mut var_fn487_calc_ig__frecgin_dn0: f64 = *var_fn487_calc_ig__frecgin_dn0_slot;
        let mut var_fn487_calc_ig__frecgin_dn18: f64 = *var_fn487_calc_ig__frecgin_dn18_slot;
        let mut var_fn487_calc_ig__frecgin_dn19: f64 = *var_fn487_calc_ig__frecgin_dn19_slot;
        let mut var_fn487_calc_ig__frecgin_dn2: f64 = *var_fn487_calc_ig__frecgin_dn2_slot;
        let mut var_fn487_calc_ig__frecgin_dn8: f64 = *var_fn487_calc_ig__frecgin_dn8_slot;
        let mut var_fn487_calc_ig__iginbd: f64 = *var_fn487_calc_ig__iginbd_slot;
        let mut var_fn487_calc_ig__iginbd_dn0: f64 = *var_fn487_calc_ig__iginbd_dn0_slot;
        let mut var_fn487_calc_ig__iginbd_dn18: f64 = *var_fn487_calc_ig__iginbd_dn18_slot;
        let mut var_fn487_calc_ig__iginbd_dn19: f64 = *var_fn487_calc_ig__iginbd_dn19_slot;
        let mut var_fn487_calc_ig__iginbd_dn2: f64 = *var_fn487_calc_ig__iginbd_dn2_slot;
        let mut var_fn487_calc_ig__iginbd_dn4: f64 = *var_fn487_calc_ig__iginbd_dn4_slot;
        let mut var_fn487_calc_ig__iginbd_dn8: f64 = *var_fn487_calc_ig__iginbd_dn8_slot;
        let mut var_fn487_calc_ig__igindiode: f64 = *var_fn487_calc_ig__igindiode_slot;
        let mut var_fn487_calc_ig__igindiode_dn0: f64 = *var_fn487_calc_ig__igindiode_dn0_slot;
        let mut var_fn487_calc_ig__igindiode_dn18: f64 = *var_fn487_calc_ig__igindiode_dn18_slot;
        let mut var_fn487_calc_ig__igindiode_dn19: f64 = *var_fn487_calc_ig__igindiode_dn19_slot;
        let mut var_fn487_calc_ig__igindiode_dn2: f64 = *var_fn487_calc_ig__igindiode_dn2_slot;
        let mut var_fn487_calc_ig__igindiode_dn4: f64 = *var_fn487_calc_ig__igindiode_dn4_slot;
        let mut var_fn487_calc_ig__igindiode_dn8: f64 = *var_fn487_calc_ig__igindiode_dn8_slot;
        let mut var_fn487_calc_ig__iginrec: f64 = *var_fn487_calc_ig__iginrec_slot;
        let mut var_fn487_calc_ig__iginrec_dn0: f64 = *var_fn487_calc_ig__iginrec_dn0_slot;
        let mut var_fn487_calc_ig__iginrec_dn18: f64 = *var_fn487_calc_ig__iginrec_dn18_slot;
        let mut var_fn487_calc_ig__iginrec_dn19: f64 = *var_fn487_calc_ig__iginrec_dn19_slot;
        let mut var_fn487_calc_ig__iginrec_dn2: f64 = *var_fn487_calc_ig__iginrec_dn2_slot;
        let mut var_fn487_calc_ig__iginrec_dn4: f64 = *var_fn487_calc_ig__iginrec_dn4_slot;
        let mut var_fn487_calc_ig__iginrec_dn8: f64 = *var_fn487_calc_ig__iginrec_dn8_slot;
        let mut var_fn487_calc_ig__igout: f64 = *var_fn487_calc_ig__igout_slot;
        let mut var_fn487_calc_ig__igout_dn0: f64 = *var_fn487_calc_ig__igout_dn0_slot;
        let mut var_fn487_calc_ig__igout_dn18: f64 = *var_fn487_calc_ig__igout_dn18_slot;
        let mut var_fn487_calc_ig__igout_dn19: f64 = *var_fn487_calc_ig__igout_dn19_slot;
        let mut var_fn487_calc_ig__igout_dn2: f64 = *var_fn487_calc_ig__igout_dn2_slot;
        let mut var_fn487_calc_ig__igout_dn4: f64 = *var_fn487_calc_ig__igout_dn4_slot;
        let mut var_fn487_calc_ig__igout_dn8: f64 = *var_fn487_calc_ig__igout_dn8_slot;
        let mut var_fn487_calc_ig__ijin: f64 = *var_fn487_calc_ig__ijin_slot;
        let mut var_fn487_calc_ig__irecin: f64 = *var_fn487_calc_ig__irecin_slot;
        let mut var_fn487_calc_ig__isdiodeout: f64 = *var_fn487_calc_ig__isdiodeout_slot;
        let mut var_fn487_calc_ig__isdiodeout_dn4: f64 = *var_fn487_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn487_calc_ig__isrecout: f64 = *var_fn487_calc_ig__isrecout_slot;
        let mut var_fn487_calc_ig__isrecout_dn4: f64 = *var_fn487_calc_ig__isrecout_dn4_slot;
        let mut var_fn487_calc_ig__kbdgatein: f64 = *var_fn487_calc_ig__kbdgatein_slot;
        let mut var_fn487_calc_ig__ngf: f64 = *var_fn487_calc_ig__ngf_slot;
        let mut var_fn487_calc_ig__pbdgin: f64 = *var_fn487_calc_ig__pbdgin_slot;
        let mut var_fn487_calc_ig__pg_param1: f64 = *var_fn487_calc_ig__pg_param1_slot;
        let mut var_fn487_calc_ig__pg_paramin: f64 = *var_fn487_calc_ig__pg_paramin_slot;
        let mut var_fn487_calc_ig__pgsrecin: f64 = *var_fn487_calc_ig__pgsrecin_slot;
        let mut var_fn487_calc_ig__phitin: f64 = *var_fn487_calc_ig__phitin_slot;
        let mut var_fn487_calc_ig__phitin_dn4: f64 = *var_fn487_calc_ig__phitin_dn4_slot;
        let mut var_fn487_calc_ig__return: f64 = *var_fn487_calc_ig__return_slot;
        let mut var_fn487_calc_ig__return_dn0: f64 = *var_fn487_calc_ig__return_dn0_slot;
        let mut var_fn487_calc_ig__return_dn18: f64 = *var_fn487_calc_ig__return_dn18_slot;
        let mut var_fn487_calc_ig__return_dn19: f64 = *var_fn487_calc_ig__return_dn19_slot;
        let mut var_fn487_calc_ig__return_dn2: f64 = *var_fn487_calc_ig__return_dn2_slot;
        let mut var_fn487_calc_ig__return_dn4: f64 = *var_fn487_calc_ig__return_dn4_slot;
        let mut var_fn487_calc_ig__return_dn8: f64 = *var_fn487_calc_ig__return_dn8_slot;
        let mut var_fn487_calc_ig__t0: f64 = *var_fn487_calc_ig__t0_slot;
        let mut var_fn487_calc_ig__t0_dn4: f64 = *var_fn487_calc_ig__t0_dn4_slot;
        let mut var_fn487_calc_ig__tfacdiodein: f64 = *var_fn487_calc_ig__tfacdiodein_slot;
        let mut var_fn487_calc_ig__tfacdiodein_dn4: f64 = *var_fn487_calc_ig__tfacdiodein_dn4_slot;
        let mut var_fn487_calc_ig__type: f64 = *var_fn487_calc_ig__type_slot;
        let mut var_fn487_calc_ig__vbdgin: f64 = *var_fn487_calc_ig__vbdgin_slot;
        let mut var_fn487_calc_ig__vgin: f64 = *var_fn487_calc_ig__vgin_slot;
        let mut var_fn487_calc_ig__vgin_dn0: f64 = *var_fn487_calc_ig__vgin_dn0_slot;
        let mut var_fn487_calc_ig__vgin_dn18: f64 = *var_fn487_calc_ig__vgin_dn18_slot;
        let mut var_fn487_calc_ig__vgin_dn19: f64 = *var_fn487_calc_ig__vgin_dn19_slot;
        let mut var_fn487_calc_ig__vgin_dn2: f64 = *var_fn487_calc_ig__vgin_dn2_slot;
        let mut var_fn487_calc_ig__vgin_dn8: f64 = *var_fn487_calc_ig__vgin_dn8_slot;
        let mut var_fn487_calc_ig__vgsatin: f64 = *var_fn487_calc_ig__vgsatin_slot;
        let mut var_fn487_calc_ig__vgsatqin: f64 = *var_fn487_calc_ig__vgsatqin_slot;
        let mut var_fn487_calc_ig__vjg: f64 = *var_fn487_calc_ig__vjg_slot;
        let mut var_fn487_calc_ig__w: f64 = *var_fn487_calc_ig__w_slot;
        let mut var_igscbd: f64 = *var_igscbd_slot;
        let mut var_igscbd_dn0: f64 = *var_igscbd_dn0_slot;
        let mut var_igscbd_dn18: f64 = *var_igscbd_dn18_slot;
        let mut var_igscbd_dn19: f64 = *var_igscbd_dn19_slot;
        let mut var_igscbd_dn2: f64 = *var_igscbd_dn2_slot;
        let mut var_igscbd_dn4: f64 = *var_igscbd_dn4_slot;
        let mut var_igscbd_dn8: f64 = *var_igscbd_dn8_slot;

        let (assign45010_e43703, assign45010_e43703_d_n0, assign45010_e43703_d_n2, assign45010_e43703_d_n4, assign45010_e43703_d_n8, assign45010_e43703_d_n18, assign45010_e43703_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign45010_e43669: f64 = (-50.0);
        let (assign45010_e43701, assign45010_e43701_d_n0, assign45010_e43701_d_n2, assign45010_e43701_d_n4, assign45010_e43701_d_n8, assign45010_e43701_d_n18, assign45010_e43701_d_n19,) = {
            if ((!(var_fn482_calc_ig__expirevarg > 50.0)) && (!(var_fn482_calc_ig__expirevarg < assign45010_e43669))) {
                let assign45010_e43674: f64 = (var_fn482_calc_ig__expirevarg).exp();
                (assign45010_e43674, (assign45010_e43674 * var_fn482_calc_ig__expirevarg_dn0), (assign45010_e43674 * var_fn482_calc_ig__expirevarg_dn2), (assign45010_e43674 * var_fn482_calc_ig__expirevarg_dn4), (assign45010_e43674 * var_fn482_calc_ig__expirevarg_dn8), (assign45010_e43674 * var_fn482_calc_ig__expirevarg_dn18), (assign45010_e43674 * var_fn482_calc_ig__expirevarg_dn19),)
            } else {
                let assign45010_e43681: f64 = (-50.0);
                let (assign45010_e43700, assign45010_e43700_d_n0, assign45010_e43700_d_n2, assign45010_e43700_d_n4, assign45010_e43700_d_n8, assign45010_e43700_d_n18, assign45010_e43700_d_n19,) = {
                    if ((!(var_fn482_calc_ig__expirevarg > 50.0)) && (var_fn482_calc_ig__expirevarg < assign45010_e43681)) {
                        let assign45010_e43685: f64 = (-50.0);
                        let assign45010_e43686: f64 = (assign45010_e43685).exp();
                        (assign45010_e43686, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign45010_e43699, assign45010_e43699_d_n0, assign45010_e43699_d_n2, assign45010_e43699_d_n4, assign45010_e43699_d_n8, assign45010_e43699_d_n18, assign45010_e43699_d_n19,) = {
                            if (var_fn482_calc_ig__expirevarg > 50.0) {
                                let assign45010_e43691: f64 = (50.0_f64).exp();
                                let assign45010_e43695: f64 = (var_fn482_calc_ig__expirevarg - 50.0);
                                let assign45010_e43696: f64 = (1.0 + assign45010_e43695);
                                let assign45010_e43697: f64 = (assign45010_e43691 * assign45010_e43696);
                                (assign45010_e43697, (assign45010_e43691 * var_fn482_calc_ig__expirevarg_dn0), (assign45010_e43691 * var_fn482_calc_ig__expirevarg_dn2), (assign45010_e43691 * var_fn482_calc_ig__expirevarg_dn4), (assign45010_e43691 * var_fn482_calc_ig__expirevarg_dn8), (assign45010_e43691 * var_fn482_calc_ig__expirevarg_dn18), (assign45010_e43691 * var_fn482_calc_ig__expirevarg_dn19),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign45010_e43699, assign45010_e43699_d_n0, assign45010_e43699_d_n2, assign45010_e43699_d_n4, assign45010_e43699_d_n8, assign45010_e43699_d_n18, assign45010_e43699_d_n19,)
                    }
                };
                (assign45010_e43700, assign45010_e43700_d_n0, assign45010_e43700_d_n2, assign45010_e43700_d_n4, assign45010_e43700_d_n8, assign45010_e43700_d_n18, assign45010_e43700_d_n19,)
            }
        };
        (assign45010_e43701, assign45010_e43701_d_n0, assign45010_e43701_d_n2, assign45010_e43701_d_n4, assign45010_e43701_d_n8, assign45010_e43701_d_n18, assign45010_e43701_d_n19,)
    } else {
        (var_fn482_calc_ig__expirev, var_fn482_calc_ig__expirev_dn0, var_fn482_calc_ig__expirev_dn2, var_fn482_calc_ig__expirev_dn4, var_fn482_calc_ig__expirev_dn8, var_fn482_calc_ig__expirev_dn18, var_fn482_calc_ig__expirev_dn19,)
    }
};
        var_fn482_calc_ig__expirev = assign45010_e43703;
        var_fn482_calc_ig__expirev_dn0 = assign45010_e43703_d_n0;
        var_fn482_calc_ig__expirev_dn2 = assign45010_e43703_d_n2;
        var_fn482_calc_ig__expirev_dn4 = assign45010_e43703_d_n4;
        var_fn482_calc_ig__expirev_dn8 = assign45010_e43703_d_n8;
        var_fn482_calc_ig__expirev_dn18 = assign45010_e43703_d_n18;
        var_fn482_calc_ig__expirev_dn19 = assign45010_e43703_d_n19;

        let (assign45020_e43711, assign45020_e43711_d_n0, assign45020_e43711_d_n2, assign45020_e43711_d_n4, assign45020_e43711_d_n8, assign45020_e43711_d_n18, assign45020_e43711_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign45020_e43708: f64 = (var_fn482_calc_ig__expirev - 1.0);
        let assign45020_e43709: f64 = (var_fn482_calc_ig__isrecout * assign45020_e43708);
        (assign45020_e43709, (var_fn482_calc_ig__isrecout * var_fn482_calc_ig__expirev_dn0), (var_fn482_calc_ig__isrecout * var_fn482_calc_ig__expirev_dn2), ((var_fn482_calc_ig__isrecout_dn4 * assign45020_e43708) + (var_fn482_calc_ig__isrecout * var_fn482_calc_ig__expirev_dn4)), (var_fn482_calc_ig__isrecout * var_fn482_calc_ig__expirev_dn8), (var_fn482_calc_ig__isrecout * var_fn482_calc_ig__expirev_dn18), (var_fn482_calc_ig__isrecout * var_fn482_calc_ig__expirev_dn19),)
    } else {
        (var_fn482_calc_ig__iginrec, var_fn482_calc_ig__iginrec_dn0, var_fn482_calc_ig__iginrec_dn2, var_fn482_calc_ig__iginrec_dn4, var_fn482_calc_ig__iginrec_dn8, var_fn482_calc_ig__iginrec_dn18, var_fn482_calc_ig__iginrec_dn19,)
    }
};
        var_fn482_calc_ig__iginrec = assign45020_e43711;
        var_fn482_calc_ig__iginrec_dn0 = assign45020_e43711_d_n0;
        var_fn482_calc_ig__iginrec_dn2 = assign45020_e43711_d_n2;
        var_fn482_calc_ig__iginrec_dn4 = assign45020_e43711_d_n4;
        var_fn482_calc_ig__iginrec_dn8 = assign45020_e43711_d_n8;
        var_fn482_calc_ig__iginrec_dn18 = assign45020_e43711_d_n18;
        var_fn482_calc_ig__iginrec_dn19 = assign45020_e43711_d_n19;

        let (assign45030_e43717, assign45030_e43717_d_n0, assign45030_e43717_d_n2, assign45030_e43717_d_n4, assign45030_e43717_d_n8, assign45030_e43717_d_n18, assign45030_e43717_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign45030_e43715: f64 = (var_fn482_calc_ig__igindiode + var_fn482_calc_ig__iginrec);
        (assign45030_e43715, (var_fn482_calc_ig__igindiode_dn0 + var_fn482_calc_ig__iginrec_dn0), (var_fn482_calc_ig__igindiode_dn2 + var_fn482_calc_ig__iginrec_dn2), (var_fn482_calc_ig__igindiode_dn4 + var_fn482_calc_ig__iginrec_dn4), (var_fn482_calc_ig__igindiode_dn8 + var_fn482_calc_ig__iginrec_dn8), (var_fn482_calc_ig__igindiode_dn18 + var_fn482_calc_ig__iginrec_dn18), (var_fn482_calc_ig__igindiode_dn19 + var_fn482_calc_ig__iginrec_dn19),)
    } else {
        (var_fn482_calc_ig__igout, var_fn482_calc_ig__igout_dn0, var_fn482_calc_ig__igout_dn2, var_fn482_calc_ig__igout_dn4, var_fn482_calc_ig__igout_dn8, var_fn482_calc_ig__igout_dn18, var_fn482_calc_ig__igout_dn19,)
    }
};
        var_fn482_calc_ig__igout = assign45030_e43717;
        var_fn482_calc_ig__igout_dn0 = assign45030_e43717_d_n0;
        var_fn482_calc_ig__igout_dn2 = assign45030_e43717_d_n2;
        var_fn482_calc_ig__igout_dn4 = assign45030_e43717_d_n4;
        var_fn482_calc_ig__igout_dn8 = assign45030_e43717_d_n8;
        var_fn482_calc_ig__igout_dn18 = assign45030_e43717_d_n18;
        var_fn482_calc_ig__igout_dn19 = assign45030_e43717_d_n19;

        let (assign45040_e43721, assign45040_e43721_d_n0, assign45040_e43721_d_n2, assign45040_e43721_d_n4, assign45040_e43721_d_n8, assign45040_e43721_d_n18, assign45040_e43721_d_n19,) = {
    if (var_guard480 != 0.0) {
        (var_fn482_calc_ig__igout, var_fn482_calc_ig__igout_dn0, var_fn482_calc_ig__igout_dn2, var_fn482_calc_ig__igout_dn4, var_fn482_calc_ig__igout_dn8, var_fn482_calc_ig__igout_dn18, var_fn482_calc_ig__igout_dn19,)
    } else {
        (var_fn482_calc_ig__return, var_fn482_calc_ig__return_dn0, var_fn482_calc_ig__return_dn2, var_fn482_calc_ig__return_dn4, var_fn482_calc_ig__return_dn8, var_fn482_calc_ig__return_dn18, var_fn482_calc_ig__return_dn19,)
    }
};
        var_fn482_calc_ig__return = assign45040_e43721;
        var_fn482_calc_ig__return_dn0 = assign45040_e43721_d_n0;
        var_fn482_calc_ig__return_dn2 = assign45040_e43721_d_n2;
        var_fn482_calc_ig__return_dn4 = assign45040_e43721_d_n4;
        var_fn482_calc_ig__return_dn8 = assign45040_e43721_d_n8;
        var_fn482_calc_ig__return_dn18 = assign45040_e43721_d_n18;
        var_fn482_calc_ig__return_dn19 = assign45040_e43721_d_n19;

        let (assign45070_e43733, assign45070_e43733_d_n0, assign45070_e43733_d_n2, assign45070_e43733_d_n4, assign45070_e43733_d_n8, assign45070_e43733_d_n18, assign45070_e43733_d_n19,) = {
    if (var_guard480 != 0.0) {
        (var_fn482_calc_ig__return, var_fn482_calc_ig__return_dn0, var_fn482_calc_ig__return_dn2, var_fn482_calc_ig__return_dn4, var_fn482_calc_ig__return_dn8, var_fn482_calc_ig__return_dn18, var_fn482_calc_ig__return_dn19,)
    } else {
        (var_igscbd, var_igscbd_dn0, var_igscbd_dn2, var_igscbd_dn4, var_igscbd_dn8, var_igscbd_dn18, var_igscbd_dn19,)
    }
};
        var_igscbd = assign45070_e43733;
        var_igscbd_dn0 = assign45070_e43733_d_n0;
        var_igscbd_dn2 = assign45070_e43733_d_n2;
        var_igscbd_dn4 = assign45070_e43733_d_n4;
        var_igscbd_dn8 = assign45070_e43733_d_n8;
        var_igscbd_dn18 = assign45070_e43733_d_n18;
        var_igscbd_dn19 = assign45070_e43733_d_n19;

        let (assign45080_e43737, assign45080_e43737_d_n0, assign45080_e43737_d_n2, assign45080_e43737_d_n4, assign45080_e43737_d_n8, assign45080_e43737_d_n18, assign45080_e43737_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__return, var_fn487_calc_ig__return_dn0, var_fn487_calc_ig__return_dn2, var_fn487_calc_ig__return_dn4, var_fn487_calc_ig__return_dn8, var_fn487_calc_ig__return_dn18, var_fn487_calc_ig__return_dn19,)
    }
};
        var_fn487_calc_ig__return = assign45080_e43737;
        var_fn487_calc_ig__return_dn0 = assign45080_e43737_d_n0;
        var_fn487_calc_ig__return_dn2 = assign45080_e43737_d_n2;
        var_fn487_calc_ig__return_dn4 = assign45080_e43737_d_n4;
        var_fn487_calc_ig__return_dn8 = assign45080_e43737_d_n8;
        var_fn487_calc_ig__return_dn18 = assign45080_e43737_d_n18;
        var_fn487_calc_ig__return_dn19 = assign45080_e43737_d_n19;

        let (assign45090_e43741, assign45090_e43741_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__isdiodeout, var_fn487_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn487_calc_ig__isdiodeout = assign45090_e43741;
        var_fn487_calc_ig__isdiodeout_dn4 = assign45090_e43741_d_n4;

        let (assign45100_e43745, assign45100_e43745_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__isrecout, var_fn487_calc_ig__isrecout_dn4,)
    }
};
        var_fn487_calc_ig__isrecout = assign45100_e43745;
        var_fn487_calc_ig__isrecout_dn4 = assign45100_e43745_d_n4;

        let (assign45110_e43749, assign45110_e43749_d_n0, assign45110_e43749_d_n2, assign45110_e43749_d_n8, assign45110_e43749_d_n18, assign45110_e43749_d_n19,) = {
    if (var_guard480 != 0.0) {
        (var_vindcbd, var_vindcbd_dn0, var_vindcbd_dn2, var_vindcbd_dn8, var_vindcbd_dn18, var_vindcbd_dn19,)
    } else {
        (var_fn487_calc_ig__vgin, var_fn487_calc_ig__vgin_dn0, var_fn487_calc_ig__vgin_dn2, var_fn487_calc_ig__vgin_dn8, var_fn487_calc_ig__vgin_dn18, var_fn487_calc_ig__vgin_dn19,)
    }
};
        var_fn487_calc_ig__vgin = assign45110_e43749;
        var_fn487_calc_ig__vgin_dn0 = assign45110_e43749_d_n0;
        var_fn487_calc_ig__vgin_dn2 = assign45110_e43749_d_n2;
        var_fn487_calc_ig__vgin_dn8 = assign45110_e43749_d_n8;
        var_fn487_calc_ig__vgin_dn18 = assign45110_e43749_d_n18;
        var_fn487_calc_ig__vgin_dn19 = assign45110_e43749_d_n19;

        let (assign45120_e43753, assign45120_e43753_d_n4,) = {
    if (var_guard480 != 0.0) {
        (var_phit, var_phit_dn4,)
    } else {
        (var_fn487_calc_ig__phitin, var_fn487_calc_ig__phitin_dn4,)
    }
};
        var_fn487_calc_ig__phitin = assign45120_e43753;
        var_fn487_calc_ig__phitin_dn4 = assign45120_e43753_d_n4;

        let (assign45130_e43757,) = {
    if (var_guard480 != 0.0) {
        (p.p265,)
    } else {
        (var_fn487_calc_ig__vgsatin,)
    }
};
        var_fn487_calc_ig__vgsatin = assign45130_e43757;

        let (assign45140_e43761,) = {
    if (var_guard480 != 0.0) {
        (p.p267,)
    } else {
        (var_fn487_calc_ig__alphagin,)
    }
};
        var_fn487_calc_ig__alphagin = assign45140_e43761;

        let (assign45150_e43765,) = {
    if (var_guard480 != 0.0) {
        (p.p266,)
    } else {
        (var_fn487_calc_ig__fracin,)
    }
};
        var_fn487_calc_ig__fracin = assign45150_e43765;

        let (assign45160_e43769,) = {
    if (var_guard480 != 0.0) {
        (0.0,)
    } else {
        (var_fn487_calc_ig__pg_paramin,)
    }
};
        var_fn487_calc_ig__pg_paramin = assign45160_e43769;

        let (assign45170_e43773,) = {
    if (var_guard480 != 0.0) {
        (p.p319,)
    } else {
        (var_fn487_calc_ig__pbdgin,)
    }
};
        var_fn487_calc_ig__pbdgin = assign45170_e43773;

        let (assign45180_e43777,) = {
    if (var_guard480 != 0.0) {
        (p.p318,)
    } else {
        (var_fn487_calc_ig__vbdgin,)
    }
};
        var_fn487_calc_ig__vbdgin = assign45180_e43777;

        let (assign45190_e43781, assign45190_e43781_d_n4,) = {
    if (var_guard480 != 0.0) {
        (var_tfacdiode, var_tfacdiode_dn4,)
    } else {
        (var_fn487_calc_ig__tfacdiodein, var_fn487_calc_ig__tfacdiodein_dn4,)
    }
};
        var_fn487_calc_ig__tfacdiodein = assign45190_e43781;
        var_fn487_calc_ig__tfacdiodein_dn4 = assign45190_e43781_d_n4;

        let (assign45200_e43785,) = {
    if (var_guard480 != 0.0) {
        (p.p0,)
    } else {
        (var_fn487_calc_ig__w,)
    }
};
        var_fn487_calc_ig__w = assign45200_e43785;

        let (assign45210_e43789,) = {
    if (var_guard480 != 0.0) {
        (p.p2,)
    } else {
        (var_fn487_calc_ig__ngf,)
    }
};
        var_fn487_calc_ig__ngf = assign45210_e43789;

        let (assign45220_e43793,) = {
    if (var_guard480 != 0.0) {
        (p.p315,)
    } else {
        (var_fn487_calc_ig__ijin,)
    }
};
        var_fn487_calc_ig__ijin = assign45220_e43793;

        let (assign45230_e43797,) = {
    if (var_guard480 != 0.0) {
        (1.0,)
    } else {
        (var_fn487_calc_ig__kbdgatein,)
    }
};
        var_fn487_calc_ig__kbdgatein = assign45230_e43797;

        let (assign45240_e43801,) = {
    if (var_guard480 != 0.0) {
        (p.p274,)
    } else {
        (var_fn487_calc_ig__vgsatqin,)
    }
};
        var_fn487_calc_ig__vgsatqin = assign45240_e43801;

        let (assign45250_e43805,) = {
    if (var_guard480 != 0.0) {
        (p.p275,)
    } else {
        (var_fn487_calc_ig__betarecin,)
    }
};
        var_fn487_calc_ig__betarecin = assign45250_e43805;

        let (assign45260_e43809,) = {
    if (var_guard480 != 0.0) {
        (0.0,)
    } else {
        (var_fn487_calc_ig__irecin,)
    }
};
        var_fn487_calc_ig__irecin = assign45260_e43809;

        let (assign45270_e43813,) = {
    if (var_guard480 != 0.0) {
        (p.p272,)
    } else {
        (var_fn487_calc_ig__pgsrecin,)
    }
};
        var_fn487_calc_ig__pgsrecin = assign45270_e43813;

        let (assign45280_e43817,) = {
    if (var_guard480 != 0.0) {
        (0.0,)
    } else {
        (var_fn487_calc_ig__pg_param1,)
    }
};
        var_fn487_calc_ig__pg_param1 = assign45280_e43817;

        let (assign45290_e43821,) = {
    if (var_guard480 != 0.0) {
        (p.p256,)
    } else {
        (var_fn487_calc_ig__vjg,)
    }
};
        var_fn487_calc_ig__vjg = assign45290_e43821;

        let (assign45300_e43825,) = {
    if (var_guard480 != 0.0) {
        (p.p6,)
    } else {
        (var_fn487_calc_ig__type,)
    }
};
        var_fn487_calc_ig__type = assign45300_e43825;

        let (assign45310_e43829, assign45310_e43829_d_n0, assign45310_e43829_d_n2, assign45310_e43829_d_n4, assign45310_e43829_d_n8, assign45310_e43829_d_n18, assign45310_e43829_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__igout, var_fn487_calc_ig__igout_dn0, var_fn487_calc_ig__igout_dn2, var_fn487_calc_ig__igout_dn4, var_fn487_calc_ig__igout_dn8, var_fn487_calc_ig__igout_dn18, var_fn487_calc_ig__igout_dn19,)
    }
};
        var_fn487_calc_ig__igout = assign45310_e43829;
        var_fn487_calc_ig__igout_dn0 = assign45310_e43829_d_n0;
        var_fn487_calc_ig__igout_dn2 = assign45310_e43829_d_n2;
        var_fn487_calc_ig__igout_dn4 = assign45310_e43829_d_n4;
        var_fn487_calc_ig__igout_dn8 = assign45310_e43829_d_n8;
        var_fn487_calc_ig__igout_dn18 = assign45310_e43829_d_n18;
        var_fn487_calc_ig__igout_dn19 = assign45310_e43829_d_n19;

        let (assign45320_e43833, assign45320_e43833_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__alpha2_phit, var_fn487_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn487_calc_ig__alpha2_phit = assign45320_e43833;
        var_fn487_calc_ig__alpha2_phit_dn4 = assign45320_e43833_d_n4;

        let (assign45330_e43837, assign45330_e43837_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__t0, var_fn487_calc_ig__t0_dn4,)
    }
};
        var_fn487_calc_ig__t0 = assign45330_e43837;
        var_fn487_calc_ig__t0_dn4 = assign45330_e43837_d_n4;

        let (assign45340_e43841, assign45340_e43841_d_n0, assign45340_e43841_d_n2, assign45340_e43841_d_n4, assign45340_e43841_d_n8, assign45340_e43841_d_n18, assign45340_e43841_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__ffvgin, var_fn487_calc_ig__ffvgin_dn0, var_fn487_calc_ig__ffvgin_dn2, var_fn487_calc_ig__ffvgin_dn4, var_fn487_calc_ig__ffvgin_dn8, var_fn487_calc_ig__ffvgin_dn18, var_fn487_calc_ig__ffvgin_dn19,)
    }
};
        var_fn487_calc_ig__ffvgin = assign45340_e43841;
        var_fn487_calc_ig__ffvgin_dn0 = assign45340_e43841_d_n0;
        var_fn487_calc_ig__ffvgin_dn2 = assign45340_e43841_d_n2;
        var_fn487_calc_ig__ffvgin_dn4 = assign45340_e43841_d_n4;
        var_fn487_calc_ig__ffvgin_dn8 = assign45340_e43841_d_n8;
        var_fn487_calc_ig__ffvgin_dn18 = assign45340_e43841_d_n18;
        var_fn487_calc_ig__ffvgin_dn19 = assign45340_e43841_d_n19;

        let (assign45350_e43845, assign45350_e43845_d_n0, assign45350_e43845_d_n2, assign45350_e43845_d_n4, assign45350_e43845_d_n8, assign45350_e43845_d_n18, assign45350_e43845_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__iginbd, var_fn487_calc_ig__iginbd_dn0, var_fn487_calc_ig__iginbd_dn2, var_fn487_calc_ig__iginbd_dn4, var_fn487_calc_ig__iginbd_dn8, var_fn487_calc_ig__iginbd_dn18, var_fn487_calc_ig__iginbd_dn19,)
    }
};
        var_fn487_calc_ig__iginbd = assign45350_e43845;
        var_fn487_calc_ig__iginbd_dn0 = assign45350_e43845_d_n0;
        var_fn487_calc_ig__iginbd_dn2 = assign45350_e43845_d_n2;
        var_fn487_calc_ig__iginbd_dn4 = assign45350_e43845_d_n4;
        var_fn487_calc_ig__iginbd_dn8 = assign45350_e43845_d_n8;
        var_fn487_calc_ig__iginbd_dn18 = assign45350_e43845_d_n18;
        var_fn487_calc_ig__iginbd_dn19 = assign45350_e43845_d_n19;

        let (assign45360_e43849, assign45360_e43849_d_n0, assign45360_e43849_d_n2, assign45360_e43849_d_n4, assign45360_e43849_d_n8, assign45360_e43849_d_n18, assign45360_e43849_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__igindiode, var_fn487_calc_ig__igindiode_dn0, var_fn487_calc_ig__igindiode_dn2, var_fn487_calc_ig__igindiode_dn4, var_fn487_calc_ig__igindiode_dn8, var_fn487_calc_ig__igindiode_dn18, var_fn487_calc_ig__igindiode_dn19,)
    }
};
        var_fn487_calc_ig__igindiode = assign45360_e43849;
        var_fn487_calc_ig__igindiode_dn0 = assign45360_e43849_d_n0;
        var_fn487_calc_ig__igindiode_dn2 = assign45360_e43849_d_n2;
        var_fn487_calc_ig__igindiode_dn4 = assign45360_e43849_d_n4;
        var_fn487_calc_ig__igindiode_dn8 = assign45360_e43849_d_n8;
        var_fn487_calc_ig__igindiode_dn18 = assign45360_e43849_d_n18;
        var_fn487_calc_ig__igindiode_dn19 = assign45360_e43849_d_n19;

        let (assign45370_e43853, assign45370_e43853_d_n0, assign45370_e43853_d_n2, assign45370_e43853_d_n8, assign45370_e43853_d_n18, assign45370_e43853_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__frecgin, var_fn487_calc_ig__frecgin_dn0, var_fn487_calc_ig__frecgin_dn2, var_fn487_calc_ig__frecgin_dn8, var_fn487_calc_ig__frecgin_dn18, var_fn487_calc_ig__frecgin_dn19,)
    }
};
        var_fn487_calc_ig__frecgin = assign45370_e43853;
        var_fn487_calc_ig__frecgin_dn0 = assign45370_e43853_d_n0;
        var_fn487_calc_ig__frecgin_dn2 = assign45370_e43853_d_n2;
        var_fn487_calc_ig__frecgin_dn8 = assign45370_e43853_d_n8;
        var_fn487_calc_ig__frecgin_dn18 = assign45370_e43853_d_n18;
        var_fn487_calc_ig__frecgin_dn19 = assign45370_e43853_d_n19;

        let (assign45380_e43857, assign45380_e43857_d_n0, assign45380_e43857_d_n2, assign45380_e43857_d_n4, assign45380_e43857_d_n8, assign45380_e43857_d_n18, assign45380_e43857_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__iginrec, var_fn487_calc_ig__iginrec_dn0, var_fn487_calc_ig__iginrec_dn2, var_fn487_calc_ig__iginrec_dn4, var_fn487_calc_ig__iginrec_dn8, var_fn487_calc_ig__iginrec_dn18, var_fn487_calc_ig__iginrec_dn19,)
    }
};
        var_fn487_calc_ig__iginrec = assign45380_e43857;
        var_fn487_calc_ig__iginrec_dn0 = assign45380_e43857_d_n0;
        var_fn487_calc_ig__iginrec_dn2 = assign45380_e43857_d_n2;
        var_fn487_calc_ig__iginrec_dn4 = assign45380_e43857_d_n4;
        var_fn487_calc_ig__iginrec_dn8 = assign45380_e43857_d_n8;
        var_fn487_calc_ig__iginrec_dn18 = assign45380_e43857_d_n18;
        var_fn487_calc_ig__iginrec_dn19 = assign45380_e43857_d_n19;

        let (assign45390_e43861, assign45390_e43861_d_n0, assign45390_e43861_d_n2, assign45390_e43861_d_n4, assign45390_e43861_d_n8, assign45390_e43861_d_n18, assign45390_e43861_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expbdarg1, var_fn487_calc_ig__expbdarg1_dn0, var_fn487_calc_ig__expbdarg1_dn2, var_fn487_calc_ig__expbdarg1_dn4, var_fn487_calc_ig__expbdarg1_dn8, var_fn487_calc_ig__expbdarg1_dn18, var_fn487_calc_ig__expbdarg1_dn19,)
    }
};
        var_fn487_calc_ig__expbdarg1 = assign45390_e43861;
        var_fn487_calc_ig__expbdarg1_dn0 = assign45390_e43861_d_n0;
        var_fn487_calc_ig__expbdarg1_dn2 = assign45390_e43861_d_n2;
        var_fn487_calc_ig__expbdarg1_dn4 = assign45390_e43861_d_n4;
        var_fn487_calc_ig__expbdarg1_dn8 = assign45390_e43861_d_n8;
        var_fn487_calc_ig__expbdarg1_dn18 = assign45390_e43861_d_n18;
        var_fn487_calc_ig__expbdarg1_dn19 = assign45390_e43861_d_n19;

        let (assign45400_e43865, assign45400_e43865_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expbdarg2, var_fn487_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn487_calc_ig__expbdarg2 = assign45400_e43865;
        var_fn487_calc_ig__expbdarg2_dn4 = assign45400_e43865_d_n4;

        let (assign45410_e43869, assign45410_e43869_d_n0, assign45410_e43869_d_n2, assign45410_e43869_d_n4, assign45410_e43869_d_n8, assign45410_e43869_d_n18, assign45410_e43869_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expbd1, var_fn487_calc_ig__expbd1_dn0, var_fn487_calc_ig__expbd1_dn2, var_fn487_calc_ig__expbd1_dn4, var_fn487_calc_ig__expbd1_dn8, var_fn487_calc_ig__expbd1_dn18, var_fn487_calc_ig__expbd1_dn19,)
    }
};
        var_fn487_calc_ig__expbd1 = assign45410_e43869;
        var_fn487_calc_ig__expbd1_dn0 = assign45410_e43869_d_n0;
        var_fn487_calc_ig__expbd1_dn2 = assign45410_e43869_d_n2;
        var_fn487_calc_ig__expbd1_dn4 = assign45410_e43869_d_n4;
        var_fn487_calc_ig__expbd1_dn8 = assign45410_e43869_d_n8;
        var_fn487_calc_ig__expbd1_dn18 = assign45410_e43869_d_n18;
        var_fn487_calc_ig__expbd1_dn19 = assign45410_e43869_d_n19;

        let (assign45420_e43873, assign45420_e43873_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expbd2, var_fn487_calc_ig__expbd2_dn4,)
    }
};
        var_fn487_calc_ig__expbd2 = assign45420_e43873;
        var_fn487_calc_ig__expbd2_dn4 = assign45420_e43873_d_n4;

        let (assign45430_e43877, assign45430_e43877_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expphib, var_fn487_calc_ig__expphib_dn4,)
    }
};
        var_fn487_calc_ig__expphib = assign45430_e43877;
        var_fn487_calc_ig__expphib_dn4 = assign45430_e43877_d_n4;

        *var_fn482_calc_ig__expirev_slot = var_fn482_calc_ig__expirev;
        *var_fn482_calc_ig__expirev_dn0_slot = var_fn482_calc_ig__expirev_dn0;
        *var_fn482_calc_ig__expirev_dn18_slot = var_fn482_calc_ig__expirev_dn18;
        *var_fn482_calc_ig__expirev_dn19_slot = var_fn482_calc_ig__expirev_dn19;
        *var_fn482_calc_ig__expirev_dn2_slot = var_fn482_calc_ig__expirev_dn2;
        *var_fn482_calc_ig__expirev_dn4_slot = var_fn482_calc_ig__expirev_dn4;
        *var_fn482_calc_ig__expirev_dn8_slot = var_fn482_calc_ig__expirev_dn8;
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
        *var_fn482_calc_ig__return_slot = var_fn482_calc_ig__return;
        *var_fn482_calc_ig__return_dn0_slot = var_fn482_calc_ig__return_dn0;
        *var_fn482_calc_ig__return_dn18_slot = var_fn482_calc_ig__return_dn18;
        *var_fn482_calc_ig__return_dn19_slot = var_fn482_calc_ig__return_dn19;
        *var_fn482_calc_ig__return_dn2_slot = var_fn482_calc_ig__return_dn2;
        *var_fn482_calc_ig__return_dn4_slot = var_fn482_calc_ig__return_dn4;
        *var_fn482_calc_ig__return_dn8_slot = var_fn482_calc_ig__return_dn8;
        *var_fn487_calc_ig__alpha2_phit_slot = var_fn487_calc_ig__alpha2_phit;
        *var_fn487_calc_ig__alpha2_phit_dn4_slot = var_fn487_calc_ig__alpha2_phit_dn4;
        *var_fn487_calc_ig__alphagin_slot = var_fn487_calc_ig__alphagin;
        *var_fn487_calc_ig__betarecin_slot = var_fn487_calc_ig__betarecin;
        *var_fn487_calc_ig__expbd1_slot = var_fn487_calc_ig__expbd1;
        *var_fn487_calc_ig__expbd1_dn0_slot = var_fn487_calc_ig__expbd1_dn0;
        *var_fn487_calc_ig__expbd1_dn18_slot = var_fn487_calc_ig__expbd1_dn18;
        *var_fn487_calc_ig__expbd1_dn19_slot = var_fn487_calc_ig__expbd1_dn19;
        *var_fn487_calc_ig__expbd1_dn2_slot = var_fn487_calc_ig__expbd1_dn2;
        *var_fn487_calc_ig__expbd1_dn4_slot = var_fn487_calc_ig__expbd1_dn4;
        *var_fn487_calc_ig__expbd1_dn8_slot = var_fn487_calc_ig__expbd1_dn8;
        *var_fn487_calc_ig__expbd2_slot = var_fn487_calc_ig__expbd2;
        *var_fn487_calc_ig__expbd2_dn4_slot = var_fn487_calc_ig__expbd2_dn4;
        *var_fn487_calc_ig__expbdarg1_slot = var_fn487_calc_ig__expbdarg1;
        *var_fn487_calc_ig__expbdarg1_dn0_slot = var_fn487_calc_ig__expbdarg1_dn0;
        *var_fn487_calc_ig__expbdarg1_dn18_slot = var_fn487_calc_ig__expbdarg1_dn18;
        *var_fn487_calc_ig__expbdarg1_dn19_slot = var_fn487_calc_ig__expbdarg1_dn19;
        *var_fn487_calc_ig__expbdarg1_dn2_slot = var_fn487_calc_ig__expbdarg1_dn2;
        *var_fn487_calc_ig__expbdarg1_dn4_slot = var_fn487_calc_ig__expbdarg1_dn4;
        *var_fn487_calc_ig__expbdarg1_dn8_slot = var_fn487_calc_ig__expbdarg1_dn8;
        *var_fn487_calc_ig__expbdarg2_slot = var_fn487_calc_ig__expbdarg2;
        *var_fn487_calc_ig__expbdarg2_dn4_slot = var_fn487_calc_ig__expbdarg2_dn4;
        *var_fn487_calc_ig__expphib_slot = var_fn487_calc_ig__expphib;
        *var_fn487_calc_ig__expphib_dn4_slot = var_fn487_calc_ig__expphib_dn4;
        *var_fn487_calc_ig__ffvgin_slot = var_fn487_calc_ig__ffvgin;
        *var_fn487_calc_ig__ffvgin_dn0_slot = var_fn487_calc_ig__ffvgin_dn0;
        *var_fn487_calc_ig__ffvgin_dn18_slot = var_fn487_calc_ig__ffvgin_dn18;
        *var_fn487_calc_ig__ffvgin_dn19_slot = var_fn487_calc_ig__ffvgin_dn19;
        *var_fn487_calc_ig__ffvgin_dn2_slot = var_fn487_calc_ig__ffvgin_dn2;
        *var_fn487_calc_ig__ffvgin_dn4_slot = var_fn487_calc_ig__ffvgin_dn4;
        *var_fn487_calc_ig__ffvgin_dn8_slot = var_fn487_calc_ig__ffvgin_dn8;
        *var_fn487_calc_ig__fracin_slot = var_fn487_calc_ig__fracin;
        *var_fn487_calc_ig__frecgin_slot = var_fn487_calc_ig__frecgin;
        *var_fn487_calc_ig__frecgin_dn0_slot = var_fn487_calc_ig__frecgin_dn0;
        *var_fn487_calc_ig__frecgin_dn18_slot = var_fn487_calc_ig__frecgin_dn18;
        *var_fn487_calc_ig__frecgin_dn19_slot = var_fn487_calc_ig__frecgin_dn19;
        *var_fn487_calc_ig__frecgin_dn2_slot = var_fn487_calc_ig__frecgin_dn2;
        *var_fn487_calc_ig__frecgin_dn8_slot = var_fn487_calc_ig__frecgin_dn8;
        *var_fn487_calc_ig__iginbd_slot = var_fn487_calc_ig__iginbd;
        *var_fn487_calc_ig__iginbd_dn0_slot = var_fn487_calc_ig__iginbd_dn0;
        *var_fn487_calc_ig__iginbd_dn18_slot = var_fn487_calc_ig__iginbd_dn18;
        *var_fn487_calc_ig__iginbd_dn19_slot = var_fn487_calc_ig__iginbd_dn19;
        *var_fn487_calc_ig__iginbd_dn2_slot = var_fn487_calc_ig__iginbd_dn2;
        *var_fn487_calc_ig__iginbd_dn4_slot = var_fn487_calc_ig__iginbd_dn4;
        *var_fn487_calc_ig__iginbd_dn8_slot = var_fn487_calc_ig__iginbd_dn8;
        *var_fn487_calc_ig__igindiode_slot = var_fn487_calc_ig__igindiode;
        *var_fn487_calc_ig__igindiode_dn0_slot = var_fn487_calc_ig__igindiode_dn0;
        *var_fn487_calc_ig__igindiode_dn18_slot = var_fn487_calc_ig__igindiode_dn18;
        *var_fn487_calc_ig__igindiode_dn19_slot = var_fn487_calc_ig__igindiode_dn19;
        *var_fn487_calc_ig__igindiode_dn2_slot = var_fn487_calc_ig__igindiode_dn2;
        *var_fn487_calc_ig__igindiode_dn4_slot = var_fn487_calc_ig__igindiode_dn4;
        *var_fn487_calc_ig__igindiode_dn8_slot = var_fn487_calc_ig__igindiode_dn8;
        *var_fn487_calc_ig__iginrec_slot = var_fn487_calc_ig__iginrec;
        *var_fn487_calc_ig__iginrec_dn0_slot = var_fn487_calc_ig__iginrec_dn0;
        *var_fn487_calc_ig__iginrec_dn18_slot = var_fn487_calc_ig__iginrec_dn18;
        *var_fn487_calc_ig__iginrec_dn19_slot = var_fn487_calc_ig__iginrec_dn19;
        *var_fn487_calc_ig__iginrec_dn2_slot = var_fn487_calc_ig__iginrec_dn2;
        *var_fn487_calc_ig__iginrec_dn4_slot = var_fn487_calc_ig__iginrec_dn4;
        *var_fn487_calc_ig__iginrec_dn8_slot = var_fn487_calc_ig__iginrec_dn8;
        *var_fn487_calc_ig__igout_slot = var_fn487_calc_ig__igout;
        *var_fn487_calc_ig__igout_dn0_slot = var_fn487_calc_ig__igout_dn0;
        *var_fn487_calc_ig__igout_dn18_slot = var_fn487_calc_ig__igout_dn18;
        *var_fn487_calc_ig__igout_dn19_slot = var_fn487_calc_ig__igout_dn19;
        *var_fn487_calc_ig__igout_dn2_slot = var_fn487_calc_ig__igout_dn2;
        *var_fn487_calc_ig__igout_dn4_slot = var_fn487_calc_ig__igout_dn4;
        *var_fn487_calc_ig__igout_dn8_slot = var_fn487_calc_ig__igout_dn8;
        *var_fn487_calc_ig__ijin_slot = var_fn487_calc_ig__ijin;
        *var_fn487_calc_ig__irecin_slot = var_fn487_calc_ig__irecin;
        *var_fn487_calc_ig__isdiodeout_slot = var_fn487_calc_ig__isdiodeout;
        *var_fn487_calc_ig__isdiodeout_dn4_slot = var_fn487_calc_ig__isdiodeout_dn4;
        *var_fn487_calc_ig__isrecout_slot = var_fn487_calc_ig__isrecout;
        *var_fn487_calc_ig__isrecout_dn4_slot = var_fn487_calc_ig__isrecout_dn4;
        *var_fn487_calc_ig__kbdgatein_slot = var_fn487_calc_ig__kbdgatein;
        *var_fn487_calc_ig__ngf_slot = var_fn487_calc_ig__ngf;
        *var_fn487_calc_ig__pbdgin_slot = var_fn487_calc_ig__pbdgin;
        *var_fn487_calc_ig__pg_param1_slot = var_fn487_calc_ig__pg_param1;
        *var_fn487_calc_ig__pg_paramin_slot = var_fn487_calc_ig__pg_paramin;
        *var_fn487_calc_ig__pgsrecin_slot = var_fn487_calc_ig__pgsrecin;
        *var_fn487_calc_ig__phitin_slot = var_fn487_calc_ig__phitin;
        *var_fn487_calc_ig__phitin_dn4_slot = var_fn487_calc_ig__phitin_dn4;
        *var_fn487_calc_ig__return_slot = var_fn487_calc_ig__return;
        *var_fn487_calc_ig__return_dn0_slot = var_fn487_calc_ig__return_dn0;
        *var_fn487_calc_ig__return_dn18_slot = var_fn487_calc_ig__return_dn18;
        *var_fn487_calc_ig__return_dn19_slot = var_fn487_calc_ig__return_dn19;
        *var_fn487_calc_ig__return_dn2_slot = var_fn487_calc_ig__return_dn2;
        *var_fn487_calc_ig__return_dn4_slot = var_fn487_calc_ig__return_dn4;
        *var_fn487_calc_ig__return_dn8_slot = var_fn487_calc_ig__return_dn8;
        *var_fn487_calc_ig__t0_slot = var_fn487_calc_ig__t0;
        *var_fn487_calc_ig__t0_dn4_slot = var_fn487_calc_ig__t0_dn4;
        *var_fn487_calc_ig__tfacdiodein_slot = var_fn487_calc_ig__tfacdiodein;
        *var_fn487_calc_ig__tfacdiodein_dn4_slot = var_fn487_calc_ig__tfacdiodein_dn4;
        *var_fn487_calc_ig__type_slot = var_fn487_calc_ig__type;
        *var_fn487_calc_ig__vbdgin_slot = var_fn487_calc_ig__vbdgin;
        *var_fn487_calc_ig__vgin_slot = var_fn487_calc_ig__vgin;
        *var_fn487_calc_ig__vgin_dn0_slot = var_fn487_calc_ig__vgin_dn0;
        *var_fn487_calc_ig__vgin_dn18_slot = var_fn487_calc_ig__vgin_dn18;
        *var_fn487_calc_ig__vgin_dn19_slot = var_fn487_calc_ig__vgin_dn19;
        *var_fn487_calc_ig__vgin_dn2_slot = var_fn487_calc_ig__vgin_dn2;
        *var_fn487_calc_ig__vgin_dn8_slot = var_fn487_calc_ig__vgin_dn8;
        *var_fn487_calc_ig__vgsatin_slot = var_fn487_calc_ig__vgsatin;
        *var_fn487_calc_ig__vgsatqin_slot = var_fn487_calc_ig__vgsatqin;
        *var_fn487_calc_ig__vjg_slot = var_fn487_calc_ig__vjg;
        *var_fn487_calc_ig__w_slot = var_fn487_calc_ig__w;
        *var_igscbd_slot = var_igscbd;
        *var_igscbd_dn0_slot = var_igscbd_dn0;
        *var_igscbd_dn18_slot = var_igscbd_dn18;
        *var_igscbd_dn19_slot = var_igscbd_dn19;
        *var_igscbd_dn2_slot = var_igscbd_dn2;
        *var_igscbd_dn4_slot = var_igscbd_dn4;
        *var_igscbd_dn8_slot = var_igscbd_dn8;
    }

    pub(super) fn stamp_transient_block_114(
        var_fn487_calc_ig__fracin: f64,
        var_fn487_calc_ig__ijin: f64,
        var_fn487_calc_ig__ngf: f64,
        var_fn487_calc_ig__pbdgin: f64,
        var_fn487_calc_ig__pg_param1: f64,
        var_fn487_calc_ig__pg_paramin: f64,
        var_fn487_calc_ig__phitin: f64,
        var_fn487_calc_ig__phitin_dn4: f64,
        var_fn487_calc_ig__tfacdiodein: f64,
        var_fn487_calc_ig__tfacdiodein_dn4: f64,
        var_fn487_calc_ig__type: f64,
        var_fn487_calc_ig__vbdgin: f64,
        var_fn487_calc_ig__vgin: f64,
        var_fn487_calc_ig__vgin_dn0: f64,
        var_fn487_calc_ig__vgin_dn18: f64,
        var_fn487_calc_ig__vgin_dn19: f64,
        var_fn487_calc_ig__vgin_dn2: f64,
        var_fn487_calc_ig__vgin_dn8: f64,
        var_fn487_calc_ig__vjg: f64,
        var_fn487_calc_ig__w: f64,
        var_guard480: f64,
        var_fn487_calc_ig__expbd1_slot: &mut f64,
        var_fn487_calc_ig__expbd1_dn0_slot: &mut f64,
        var_fn487_calc_ig__expbd1_dn18_slot: &mut f64,
        var_fn487_calc_ig__expbd1_dn19_slot: &mut f64,
        var_fn487_calc_ig__expbd1_dn2_slot: &mut f64,
        var_fn487_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn487_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn487_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn487_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__expbd2_slot: &mut f64,
        var_fn487_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_dn0_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_dn18_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_dn19_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_dn2_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__expbdarg2_slot: &mut f64,
        var_fn487_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn487_calc_ig__expffvarg_slot: &mut f64,
        var_fn487_calc_ig__expffvarg_dn0_slot: &mut f64,
        var_fn487_calc_ig__expffvarg_dn18_slot: &mut f64,
        var_fn487_calc_ig__expffvarg_dn19_slot: &mut f64,
        var_fn487_calc_ig__expffvarg_dn2_slot: &mut f64,
        var_fn487_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn487_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn487_calc_ig__expifor_slot: &mut f64,
        var_fn487_calc_ig__expifor_dn0_slot: &mut f64,
        var_fn487_calc_ig__expifor_dn18_slot: &mut f64,
        var_fn487_calc_ig__expifor_dn19_slot: &mut f64,
        var_fn487_calc_ig__expifor_dn2_slot: &mut f64,
        var_fn487_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn487_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_dn0_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_dn18_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_dn19_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_dn2_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn487_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_dn0_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_dn18_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_dn19_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_dn2_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_dn0_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_dn18_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_dn19_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_dn2_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__expirev_slot: &mut f64,
        var_fn487_calc_ig__expirev_dn0_slot: &mut f64,
        var_fn487_calc_ig__expirev_dn18_slot: &mut f64,
        var_fn487_calc_ig__expirev_dn19_slot: &mut f64,
        var_fn487_calc_ig__expirev_dn2_slot: &mut f64,
        var_fn487_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn487_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn487_calc_ig__expirevarg_slot: &mut f64,
        var_fn487_calc_ig__expirevarg_dn0_slot: &mut f64,
        var_fn487_calc_ig__expirevarg_dn18_slot: &mut f64,
        var_fn487_calc_ig__expirevarg_dn19_slot: &mut f64,
        var_fn487_calc_ig__expirevarg_dn2_slot: &mut f64,
        var_fn487_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn487_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn487_calc_ig__expphib_slot: &mut f64,
        var_fn487_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn487_calc_ig__iginbd_slot: &mut f64,
        var_fn487_calc_ig__iginbd_dn0_slot: &mut f64,
        var_fn487_calc_ig__iginbd_dn18_slot: &mut f64,
        var_fn487_calc_ig__iginbd_dn19_slot: &mut f64,
        var_fn487_calc_ig__iginbd_dn2_slot: &mut f64,
        var_fn487_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn487_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn487_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn487_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_dn0_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_dn18_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_dn19_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_dn2_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_dn0_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_dn18_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_dn19_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_dn2_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__isdiodeout_slot: &mut f64,
        var_fn487_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn487_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_fn487_calc_ig__t0_slot: &mut f64,
        var_fn487_calc_ig__t0_dn4_slot: &mut f64,
        var_guard488_slot: &mut f64,
    ) {
        let mut var_fn487_calc_ig__expbd1: f64 = *var_fn487_calc_ig__expbd1_slot;
        let mut var_fn487_calc_ig__expbd1_dn0: f64 = *var_fn487_calc_ig__expbd1_dn0_slot;
        let mut var_fn487_calc_ig__expbd1_dn18: f64 = *var_fn487_calc_ig__expbd1_dn18_slot;
        let mut var_fn487_calc_ig__expbd1_dn19: f64 = *var_fn487_calc_ig__expbd1_dn19_slot;
        let mut var_fn487_calc_ig__expbd1_dn2: f64 = *var_fn487_calc_ig__expbd1_dn2_slot;
        let mut var_fn487_calc_ig__expbd1_dn4: f64 = *var_fn487_calc_ig__expbd1_dn4_slot;
        let mut var_fn487_calc_ig__expbd1_dn8: f64 = *var_fn487_calc_ig__expbd1_dn8_slot;
        let mut var_fn487_calc_ig__expbd1_vgsat: f64 = *var_fn487_calc_ig__expbd1_vgsat_slot;
        let mut var_fn487_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn487_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__expbd2: f64 = *var_fn487_calc_ig__expbd2_slot;
        let mut var_fn487_calc_ig__expbd2_dn4: f64 = *var_fn487_calc_ig__expbd2_dn4_slot;
        let mut var_fn487_calc_ig__expbdarg1: f64 = *var_fn487_calc_ig__expbdarg1_slot;
        let mut var_fn487_calc_ig__expbdarg1_dn0: f64 = *var_fn487_calc_ig__expbdarg1_dn0_slot;
        let mut var_fn487_calc_ig__expbdarg1_dn18: f64 = *var_fn487_calc_ig__expbdarg1_dn18_slot;
        let mut var_fn487_calc_ig__expbdarg1_dn19: f64 = *var_fn487_calc_ig__expbdarg1_dn19_slot;
        let mut var_fn487_calc_ig__expbdarg1_dn2: f64 = *var_fn487_calc_ig__expbdarg1_dn2_slot;
        let mut var_fn487_calc_ig__expbdarg1_dn4: f64 = *var_fn487_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn487_calc_ig__expbdarg1_dn8: f64 = *var_fn487_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn487_calc_ig__expbdarg1_vgsat: f64 = *var_fn487_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn487_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn487_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__expbdarg2: f64 = *var_fn487_calc_ig__expbdarg2_slot;
        let mut var_fn487_calc_ig__expbdarg2_dn4: f64 = *var_fn487_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn487_calc_ig__expffvarg: f64 = *var_fn487_calc_ig__expffvarg_slot;
        let mut var_fn487_calc_ig__expffvarg_dn0: f64 = *var_fn487_calc_ig__expffvarg_dn0_slot;
        let mut var_fn487_calc_ig__expffvarg_dn18: f64 = *var_fn487_calc_ig__expffvarg_dn18_slot;
        let mut var_fn487_calc_ig__expffvarg_dn19: f64 = *var_fn487_calc_ig__expffvarg_dn19_slot;
        let mut var_fn487_calc_ig__expffvarg_dn2: f64 = *var_fn487_calc_ig__expffvarg_dn2_slot;
        let mut var_fn487_calc_ig__expffvarg_dn4: f64 = *var_fn487_calc_ig__expffvarg_dn4_slot;
        let mut var_fn487_calc_ig__expffvarg_dn8: f64 = *var_fn487_calc_ig__expffvarg_dn8_slot;
        let mut var_fn487_calc_ig__expifor: f64 = *var_fn487_calc_ig__expifor_slot;
        let mut var_fn487_calc_ig__expifor_dn0: f64 = *var_fn487_calc_ig__expifor_dn0_slot;
        let mut var_fn487_calc_ig__expifor_dn18: f64 = *var_fn487_calc_ig__expifor_dn18_slot;
        let mut var_fn487_calc_ig__expifor_dn19: f64 = *var_fn487_calc_ig__expifor_dn19_slot;
        let mut var_fn487_calc_ig__expifor_dn2: f64 = *var_fn487_calc_ig__expifor_dn2_slot;
        let mut var_fn487_calc_ig__expifor_dn4: f64 = *var_fn487_calc_ig__expifor_dn4_slot;
        let mut var_fn487_calc_ig__expifor_dn8: f64 = *var_fn487_calc_ig__expifor_dn8_slot;
        let mut var_fn487_calc_ig__expifor_hinj: f64 = *var_fn487_calc_ig__expifor_hinj_slot;
        let mut var_fn487_calc_ig__expifor_hinj_dn0: f64 = *var_fn487_calc_ig__expifor_hinj_dn0_slot;
        let mut var_fn487_calc_ig__expifor_hinj_dn18: f64 = *var_fn487_calc_ig__expifor_hinj_dn18_slot;
        let mut var_fn487_calc_ig__expifor_hinj_dn19: f64 = *var_fn487_calc_ig__expifor_hinj_dn19_slot;
        let mut var_fn487_calc_ig__expifor_hinj_dn2: f64 = *var_fn487_calc_ig__expifor_hinj_dn2_slot;
        let mut var_fn487_calc_ig__expifor_hinj_dn4: f64 = *var_fn487_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn487_calc_ig__expifor_hinj_dn8: f64 = *var_fn487_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn487_calc_ig__expifor_hinj_vgsat: f64 = *var_fn487_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn487_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn487_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn487_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn487_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn487_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__expiforarg: f64 = *var_fn487_calc_ig__expiforarg_slot;
        let mut var_fn487_calc_ig__expiforarg_dn0: f64 = *var_fn487_calc_ig__expiforarg_dn0_slot;
        let mut var_fn487_calc_ig__expiforarg_dn18: f64 = *var_fn487_calc_ig__expiforarg_dn18_slot;
        let mut var_fn487_calc_ig__expiforarg_dn19: f64 = *var_fn487_calc_ig__expiforarg_dn19_slot;
        let mut var_fn487_calc_ig__expiforarg_dn2: f64 = *var_fn487_calc_ig__expiforarg_dn2_slot;
        let mut var_fn487_calc_ig__expiforarg_dn4: f64 = *var_fn487_calc_ig__expiforarg_dn4_slot;
        let mut var_fn487_calc_ig__expiforarg_dn8: f64 = *var_fn487_calc_ig__expiforarg_dn8_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj: f64 = *var_fn487_calc_ig__expiforarg_hinj_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj_dn0: f64 = *var_fn487_calc_ig__expiforarg_hinj_dn0_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj_dn18: f64 = *var_fn487_calc_ig__expiforarg_hinj_dn18_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj_dn19: f64 = *var_fn487_calc_ig__expiforarg_hinj_dn19_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj_dn2: f64 = *var_fn487_calc_ig__expiforarg_hinj_dn2_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn487_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn487_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn487_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn487_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn487_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn487_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn487_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__expirev: f64 = *var_fn487_calc_ig__expirev_slot;
        let mut var_fn487_calc_ig__expirev_dn0: f64 = *var_fn487_calc_ig__expirev_dn0_slot;
        let mut var_fn487_calc_ig__expirev_dn18: f64 = *var_fn487_calc_ig__expirev_dn18_slot;
        let mut var_fn487_calc_ig__expirev_dn19: f64 = *var_fn487_calc_ig__expirev_dn19_slot;
        let mut var_fn487_calc_ig__expirev_dn2: f64 = *var_fn487_calc_ig__expirev_dn2_slot;
        let mut var_fn487_calc_ig__expirev_dn4: f64 = *var_fn487_calc_ig__expirev_dn4_slot;
        let mut var_fn487_calc_ig__expirev_dn8: f64 = *var_fn487_calc_ig__expirev_dn8_slot;
        let mut var_fn487_calc_ig__expirevarg: f64 = *var_fn487_calc_ig__expirevarg_slot;
        let mut var_fn487_calc_ig__expirevarg_dn0: f64 = *var_fn487_calc_ig__expirevarg_dn0_slot;
        let mut var_fn487_calc_ig__expirevarg_dn18: f64 = *var_fn487_calc_ig__expirevarg_dn18_slot;
        let mut var_fn487_calc_ig__expirevarg_dn19: f64 = *var_fn487_calc_ig__expirevarg_dn19_slot;
        let mut var_fn487_calc_ig__expirevarg_dn2: f64 = *var_fn487_calc_ig__expirevarg_dn2_slot;
        let mut var_fn487_calc_ig__expirevarg_dn4: f64 = *var_fn487_calc_ig__expirevarg_dn4_slot;
        let mut var_fn487_calc_ig__expirevarg_dn8: f64 = *var_fn487_calc_ig__expirevarg_dn8_slot;
        let mut var_fn487_calc_ig__expphib: f64 = *var_fn487_calc_ig__expphib_slot;
        let mut var_fn487_calc_ig__expphib_dn4: f64 = *var_fn487_calc_ig__expphib_dn4_slot;
        let mut var_fn487_calc_ig__iginbd: f64 = *var_fn487_calc_ig__iginbd_slot;
        let mut var_fn487_calc_ig__iginbd_dn0: f64 = *var_fn487_calc_ig__iginbd_dn0_slot;
        let mut var_fn487_calc_ig__iginbd_dn18: f64 = *var_fn487_calc_ig__iginbd_dn18_slot;
        let mut var_fn487_calc_ig__iginbd_dn19: f64 = *var_fn487_calc_ig__iginbd_dn19_slot;
        let mut var_fn487_calc_ig__iginbd_dn2: f64 = *var_fn487_calc_ig__iginbd_dn2_slot;
        let mut var_fn487_calc_ig__iginbd_dn4: f64 = *var_fn487_calc_ig__iginbd_dn4_slot;
        let mut var_fn487_calc_ig__iginbd_dn8: f64 = *var_fn487_calc_ig__iginbd_dn8_slot;
        let mut var_fn487_calc_ig__iginbd_vgsat: f64 = *var_fn487_calc_ig__iginbd_vgsat_slot;
        let mut var_fn487_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn487_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__igindiode_hinj: f64 = *var_fn487_calc_ig__igindiode_hinj_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_dn0: f64 = *var_fn487_calc_ig__igindiode_hinj_dn0_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_dn18: f64 = *var_fn487_calc_ig__igindiode_hinj_dn18_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_dn19: f64 = *var_fn487_calc_ig__igindiode_hinj_dn19_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_dn2: f64 = *var_fn487_calc_ig__igindiode_hinj_dn2_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_dn4: f64 = *var_fn487_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_dn8: f64 = *var_fn487_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_pre: f64 = *var_fn487_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn487_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn487_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn487_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj: f64 = *var_fn487_calc_ig__igindiode_nohinj_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj_dn0: f64 = *var_fn487_calc_ig__igindiode_nohinj_dn0_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj_dn18: f64 = *var_fn487_calc_ig__igindiode_nohinj_dn18_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj_dn19: f64 = *var_fn487_calc_ig__igindiode_nohinj_dn19_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj_dn2: f64 = *var_fn487_calc_ig__igindiode_nohinj_dn2_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn487_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn487_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn487_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn487_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__isdiodeout: f64 = *var_fn487_calc_ig__isdiodeout_slot;
        let mut var_fn487_calc_ig__isdiodeout_dn4: f64 = *var_fn487_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn487_calc_ig__pg_paramin_hinj: f64 = *var_fn487_calc_ig__pg_paramin_hinj_slot;
        let mut var_fn487_calc_ig__t0: f64 = *var_fn487_calc_ig__t0_slot;
        let mut var_fn487_calc_ig__t0_dn4: f64 = *var_fn487_calc_ig__t0_dn4_slot;
        let mut var_guard488: f64 = *var_guard488_slot;

        let (assign45440_e43881, assign45440_e43881_d_n0, assign45440_e43881_d_n2, assign45440_e43881_d_n4, assign45440_e43881_d_n8, assign45440_e43881_d_n18, assign45440_e43881_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expffvarg, var_fn487_calc_ig__expffvarg_dn0, var_fn487_calc_ig__expffvarg_dn2, var_fn487_calc_ig__expffvarg_dn4, var_fn487_calc_ig__expffvarg_dn8, var_fn487_calc_ig__expffvarg_dn18, var_fn487_calc_ig__expffvarg_dn19,)
    }
};
        var_fn487_calc_ig__expffvarg = assign45440_e43881;
        var_fn487_calc_ig__expffvarg_dn0 = assign45440_e43881_d_n0;
        var_fn487_calc_ig__expffvarg_dn2 = assign45440_e43881_d_n2;
        var_fn487_calc_ig__expffvarg_dn4 = assign45440_e43881_d_n4;
        var_fn487_calc_ig__expffvarg_dn8 = assign45440_e43881_d_n8;
        var_fn487_calc_ig__expffvarg_dn18 = assign45440_e43881_d_n18;
        var_fn487_calc_ig__expffvarg_dn19 = assign45440_e43881_d_n19;

        let (assign45450_e43885, assign45450_e43885_d_n0, assign45450_e43885_d_n2, assign45450_e43885_d_n4, assign45450_e43885_d_n8, assign45450_e43885_d_n18, assign45450_e43885_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expiforarg, var_fn487_calc_ig__expiforarg_dn0, var_fn487_calc_ig__expiforarg_dn2, var_fn487_calc_ig__expiforarg_dn4, var_fn487_calc_ig__expiforarg_dn8, var_fn487_calc_ig__expiforarg_dn18, var_fn487_calc_ig__expiforarg_dn19,)
    }
};
        var_fn487_calc_ig__expiforarg = assign45450_e43885;
        var_fn487_calc_ig__expiforarg_dn0 = assign45450_e43885_d_n0;
        var_fn487_calc_ig__expiforarg_dn2 = assign45450_e43885_d_n2;
        var_fn487_calc_ig__expiforarg_dn4 = assign45450_e43885_d_n4;
        var_fn487_calc_ig__expiforarg_dn8 = assign45450_e43885_d_n8;
        var_fn487_calc_ig__expiforarg_dn18 = assign45450_e43885_d_n18;
        var_fn487_calc_ig__expiforarg_dn19 = assign45450_e43885_d_n19;

        let (assign45460_e43889, assign45460_e43889_d_n0, assign45460_e43889_d_n2, assign45460_e43889_d_n4, assign45460_e43889_d_n8, assign45460_e43889_d_n18, assign45460_e43889_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expifor, var_fn487_calc_ig__expifor_dn0, var_fn487_calc_ig__expifor_dn2, var_fn487_calc_ig__expifor_dn4, var_fn487_calc_ig__expifor_dn8, var_fn487_calc_ig__expifor_dn18, var_fn487_calc_ig__expifor_dn19,)
    }
};
        var_fn487_calc_ig__expifor = assign45460_e43889;
        var_fn487_calc_ig__expifor_dn0 = assign45460_e43889_d_n0;
        var_fn487_calc_ig__expifor_dn2 = assign45460_e43889_d_n2;
        var_fn487_calc_ig__expifor_dn4 = assign45460_e43889_d_n4;
        var_fn487_calc_ig__expifor_dn8 = assign45460_e43889_d_n8;
        var_fn487_calc_ig__expifor_dn18 = assign45460_e43889_d_n18;
        var_fn487_calc_ig__expifor_dn19 = assign45460_e43889_d_n19;

        let (assign45470_e43893, assign45470_e43893_d_n0, assign45470_e43893_d_n2, assign45470_e43893_d_n4, assign45470_e43893_d_n8, assign45470_e43893_d_n18, assign45470_e43893_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expirevarg, var_fn487_calc_ig__expirevarg_dn0, var_fn487_calc_ig__expirevarg_dn2, var_fn487_calc_ig__expirevarg_dn4, var_fn487_calc_ig__expirevarg_dn8, var_fn487_calc_ig__expirevarg_dn18, var_fn487_calc_ig__expirevarg_dn19,)
    }
};
        var_fn487_calc_ig__expirevarg = assign45470_e43893;
        var_fn487_calc_ig__expirevarg_dn0 = assign45470_e43893_d_n0;
        var_fn487_calc_ig__expirevarg_dn2 = assign45470_e43893_d_n2;
        var_fn487_calc_ig__expirevarg_dn4 = assign45470_e43893_d_n4;
        var_fn487_calc_ig__expirevarg_dn8 = assign45470_e43893_d_n8;
        var_fn487_calc_ig__expirevarg_dn18 = assign45470_e43893_d_n18;
        var_fn487_calc_ig__expirevarg_dn19 = assign45470_e43893_d_n19;

        let (assign45480_e43897, assign45480_e43897_d_n0, assign45480_e43897_d_n2, assign45480_e43897_d_n4, assign45480_e43897_d_n8, assign45480_e43897_d_n18, assign45480_e43897_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expirev, var_fn487_calc_ig__expirev_dn0, var_fn487_calc_ig__expirev_dn2, var_fn487_calc_ig__expirev_dn4, var_fn487_calc_ig__expirev_dn8, var_fn487_calc_ig__expirev_dn18, var_fn487_calc_ig__expirev_dn19,)
    }
};
        var_fn487_calc_ig__expirev = assign45480_e43897;
        var_fn487_calc_ig__expirev_dn0 = assign45480_e43897_d_n0;
        var_fn487_calc_ig__expirev_dn2 = assign45480_e43897_d_n2;
        var_fn487_calc_ig__expirev_dn4 = assign45480_e43897_d_n4;
        var_fn487_calc_ig__expirev_dn8 = assign45480_e43897_d_n8;
        var_fn487_calc_ig__expirev_dn18 = assign45480_e43897_d_n18;
        var_fn487_calc_ig__expirev_dn19 = assign45480_e43897_d_n19;

        let (assign45490_e43901,) = {
    if (var_guard480 != 0.0) {
        (0.0,)
    } else {
        (var_fn487_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn487_calc_ig__pg_paramin_hinj = assign45490_e43901;

        let (assign45500_e43905, assign45500_e43905_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expbdarg1_vgsat, var_fn487_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__expbdarg1_vgsat = assign45500_e43905;
        var_fn487_calc_ig__expbdarg1_vgsat_dn4 = assign45500_e43905_d_n4;

        let (assign45510_e43909, assign45510_e43909_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expbd1_vgsat, var_fn487_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__expbd1_vgsat = assign45510_e43909;
        var_fn487_calc_ig__expbd1_vgsat_dn4 = assign45510_e43909_d_n4;

        let (assign45520_e43913, assign45520_e43913_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__iginbd_vgsat, var_fn487_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__iginbd_vgsat = assign45520_e43913;
        var_fn487_calc_ig__iginbd_vgsat_dn4 = assign45520_e43913_d_n4;

        let (assign45530_e43917, assign45530_e43917_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expiforarg_nohinj_vgsat, var_fn487_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__expiforarg_nohinj_vgsat = assign45530_e43917;
        var_fn487_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign45530_e43917_d_n4;

        let (assign45540_e43921, assign45540_e43921_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expifor_nohinj_vgsat, var_fn487_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__expifor_nohinj_vgsat = assign45540_e43921;
        var_fn487_calc_ig__expifor_nohinj_vgsat_dn4 = assign45540_e43921_d_n4;

        let (assign45550_e43925, assign45550_e43925_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__igindiode_nohinj_vgsat, var_fn487_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__igindiode_nohinj_vgsat = assign45550_e43925;
        var_fn487_calc_ig__igindiode_nohinj_vgsat_dn4 = assign45550_e43925_d_n4;

        let (assign45560_e43929, assign45560_e43929_d_n0, assign45560_e43929_d_n2, assign45560_e43929_d_n4, assign45560_e43929_d_n8, assign45560_e43929_d_n18, assign45560_e43929_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__igindiode_nohinj, var_fn487_calc_ig__igindiode_nohinj_dn0, var_fn487_calc_ig__igindiode_nohinj_dn2, var_fn487_calc_ig__igindiode_nohinj_dn4, var_fn487_calc_ig__igindiode_nohinj_dn8, var_fn487_calc_ig__igindiode_nohinj_dn18, var_fn487_calc_ig__igindiode_nohinj_dn19,)
    }
};
        var_fn487_calc_ig__igindiode_nohinj = assign45560_e43929;
        var_fn487_calc_ig__igindiode_nohinj_dn0 = assign45560_e43929_d_n0;
        var_fn487_calc_ig__igindiode_nohinj_dn2 = assign45560_e43929_d_n2;
        var_fn487_calc_ig__igindiode_nohinj_dn4 = assign45560_e43929_d_n4;
        var_fn487_calc_ig__igindiode_nohinj_dn8 = assign45560_e43929_d_n8;
        var_fn487_calc_ig__igindiode_nohinj_dn18 = assign45560_e43929_d_n18;
        var_fn487_calc_ig__igindiode_nohinj_dn19 = assign45560_e43929_d_n19;

        let (assign45570_e43933, assign45570_e43933_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expiforarg_hinj_vgsat, var_fn487_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__expiforarg_hinj_vgsat = assign45570_e43933;
        var_fn487_calc_ig__expiforarg_hinj_vgsat_dn4 = assign45570_e43933_d_n4;

        let (assign45580_e43937, assign45580_e43937_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expifor_hinj_vgsat, var_fn487_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__expifor_hinj_vgsat = assign45580_e43937;
        var_fn487_calc_ig__expifor_hinj_vgsat_dn4 = assign45580_e43937_d_n4;

        let (assign45590_e43941, assign45590_e43941_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__igindiode_hinj_vgsat, var_fn487_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__igindiode_hinj_vgsat = assign45590_e43941;
        var_fn487_calc_ig__igindiode_hinj_vgsat_dn4 = assign45590_e43941_d_n4;

        let (assign45600_e43945, assign45600_e43945_d_n0, assign45600_e43945_d_n2, assign45600_e43945_d_n4, assign45600_e43945_d_n8, assign45600_e43945_d_n18, assign45600_e43945_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expiforarg_hinj, var_fn487_calc_ig__expiforarg_hinj_dn0, var_fn487_calc_ig__expiforarg_hinj_dn2, var_fn487_calc_ig__expiforarg_hinj_dn4, var_fn487_calc_ig__expiforarg_hinj_dn8, var_fn487_calc_ig__expiforarg_hinj_dn18, var_fn487_calc_ig__expiforarg_hinj_dn19,)
    }
};
        var_fn487_calc_ig__expiforarg_hinj = assign45600_e43945;
        var_fn487_calc_ig__expiforarg_hinj_dn0 = assign45600_e43945_d_n0;
        var_fn487_calc_ig__expiforarg_hinj_dn2 = assign45600_e43945_d_n2;
        var_fn487_calc_ig__expiforarg_hinj_dn4 = assign45600_e43945_d_n4;
        var_fn487_calc_ig__expiforarg_hinj_dn8 = assign45600_e43945_d_n8;
        var_fn487_calc_ig__expiforarg_hinj_dn18 = assign45600_e43945_d_n18;
        var_fn487_calc_ig__expiforarg_hinj_dn19 = assign45600_e43945_d_n19;

        let (assign45610_e43949, assign45610_e43949_d_n0, assign45610_e43949_d_n2, assign45610_e43949_d_n4, assign45610_e43949_d_n8, assign45610_e43949_d_n18, assign45610_e43949_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__expifor_hinj, var_fn487_calc_ig__expifor_hinj_dn0, var_fn487_calc_ig__expifor_hinj_dn2, var_fn487_calc_ig__expifor_hinj_dn4, var_fn487_calc_ig__expifor_hinj_dn8, var_fn487_calc_ig__expifor_hinj_dn18, var_fn487_calc_ig__expifor_hinj_dn19,)
    }
};
        var_fn487_calc_ig__expifor_hinj = assign45610_e43949;
        var_fn487_calc_ig__expifor_hinj_dn0 = assign45610_e43949_d_n0;
        var_fn487_calc_ig__expifor_hinj_dn2 = assign45610_e43949_d_n2;
        var_fn487_calc_ig__expifor_hinj_dn4 = assign45610_e43949_d_n4;
        var_fn487_calc_ig__expifor_hinj_dn8 = assign45610_e43949_d_n8;
        var_fn487_calc_ig__expifor_hinj_dn18 = assign45610_e43949_d_n18;
        var_fn487_calc_ig__expifor_hinj_dn19 = assign45610_e43949_d_n19;

        let (assign45620_e43953, assign45620_e43953_d_n4,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__igindiode_hinj_pre, var_fn487_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn487_calc_ig__igindiode_hinj_pre = assign45620_e43953;
        var_fn487_calc_ig__igindiode_hinj_pre_dn4 = assign45620_e43953_d_n4;

        let (assign45630_e43957, assign45630_e43957_d_n0, assign45630_e43957_d_n2, assign45630_e43957_d_n4, assign45630_e43957_d_n8, assign45630_e43957_d_n18, assign45630_e43957_d_n19,) = {
    if (var_guard480 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__igindiode_hinj, var_fn487_calc_ig__igindiode_hinj_dn0, var_fn487_calc_ig__igindiode_hinj_dn2, var_fn487_calc_ig__igindiode_hinj_dn4, var_fn487_calc_ig__igindiode_hinj_dn8, var_fn487_calc_ig__igindiode_hinj_dn18, var_fn487_calc_ig__igindiode_hinj_dn19,)
    }
};
        var_fn487_calc_ig__igindiode_hinj = assign45630_e43957;
        var_fn487_calc_ig__igindiode_hinj_dn0 = assign45630_e43957_d_n0;
        var_fn487_calc_ig__igindiode_hinj_dn2 = assign45630_e43957_d_n2;
        var_fn487_calc_ig__igindiode_hinj_dn4 = assign45630_e43957_d_n4;
        var_fn487_calc_ig__igindiode_hinj_dn8 = assign45630_e43957_d_n8;
        var_fn487_calc_ig__igindiode_hinj_dn18 = assign45630_e43957_d_n18;
        var_fn487_calc_ig__igindiode_hinj_dn19 = assign45630_e43957_d_n19;

        let (assign45640_e43966, assign45640_e43966_d_n4,) = {
    if (var_guard480 != 0.0) {
        let assign45640_e43961: f64 = (var_fn487_calc_ig__pg_param1 / var_fn487_calc_ig__phitin);
        let assign45640_e43963: f64 = (-var_fn487_calc_ig__vjg);
        let assign45640_e43964: f64 = (assign45640_e43961 * assign45640_e43963);
        (assign45640_e43964, ((-((var_fn487_calc_ig__pg_param1 * var_fn487_calc_ig__phitin_dn4) / (var_fn487_calc_ig__phitin * var_fn487_calc_ig__phitin))) * assign45640_e43963),)
    } else {
        (var_fn487_calc_ig__expphib, var_fn487_calc_ig__expphib_dn4,)
    }
};
        var_fn487_calc_ig__expphib = assign45640_e43966;
        var_fn487_calc_ig__expphib_dn4 = assign45640_e43966_d_n4;

        let (assign45650_e44008, assign45650_e44008_d_n4,) = {
    if (var_guard480 != 0.0) {
        let assign45650_e43974: f64 = (-50.0);
        let (assign45650_e44006, assign45650_e44006_d_n4,) = {
            if ((!(var_fn487_calc_ig__expphib > 50.0)) && (!(var_fn487_calc_ig__expphib < assign45650_e43974))) {
                let assign45650_e43979: f64 = (var_fn487_calc_ig__expphib).exp();
                (assign45650_e43979, (assign45650_e43979 * var_fn487_calc_ig__expphib_dn4),)
            } else {
                let assign45650_e43986: f64 = (-50.0);
                let (assign45650_e44005, assign45650_e44005_d_n4,) = {
                    if ((!(var_fn487_calc_ig__expphib > 50.0)) && (var_fn487_calc_ig__expphib < assign45650_e43986)) {
                        let assign45650_e43990: f64 = (-50.0);
                        let assign45650_e43991: f64 = (assign45650_e43990).exp();
                        (assign45650_e43991, 0.0,)
                    } else {
                        let (assign45650_e44004, assign45650_e44004_d_n4,) = {
                            if (var_fn487_calc_ig__expphib > 50.0) {
                                let assign45650_e43996: f64 = (50.0_f64).exp();
                                let assign45650_e44000: f64 = (var_fn487_calc_ig__expphib - 50.0);
                                let assign45650_e44001: f64 = (1.0 + assign45650_e44000);
                                let assign45650_e44002: f64 = (assign45650_e43996 * assign45650_e44001);
                                (assign45650_e44002, (assign45650_e43996 * var_fn487_calc_ig__expphib_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign45650_e44004, assign45650_e44004_d_n4,)
                    }
                };
                (assign45650_e44005, assign45650_e44005_d_n4,)
            }
        };
        (assign45650_e44006, assign45650_e44006_d_n4,)
    } else {
        (var_fn487_calc_ig__t0, var_fn487_calc_ig__t0_dn4,)
    }
};
        var_fn487_calc_ig__t0 = assign45650_e44008;
        var_fn487_calc_ig__t0_dn4 = assign45650_e44008_d_n4;

        let (assign45660_e44019, assign45660_e44019_d_n0, assign45660_e44019_d_n2, assign45660_e44019_d_n4, assign45660_e44019_d_n8, assign45660_e44019_d_n18, assign45660_e44019_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign45660_e44012: f64 = (-var_fn487_calc_ig__vgin);
        let assign45660_e44014: f64 = (assign45660_e44012 - var_fn487_calc_ig__vbdgin);
        let assign45660_e44015: f64 = (var_fn487_calc_ig__pbdgin * assign45660_e44014);
        let assign45660_e44017: f64 = (assign45660_e44015 + var_fn487_calc_ig__expphib);
        (assign45660_e44017, (var_fn487_calc_ig__pbdgin * (-var_fn487_calc_ig__vgin_dn0)), (var_fn487_calc_ig__pbdgin * (-var_fn487_calc_ig__vgin_dn2)), var_fn487_calc_ig__expphib_dn4, (var_fn487_calc_ig__pbdgin * (-var_fn487_calc_ig__vgin_dn8)), (var_fn487_calc_ig__pbdgin * (-var_fn487_calc_ig__vgin_dn18)), (var_fn487_calc_ig__pbdgin * (-var_fn487_calc_ig__vgin_dn19)),)
    } else {
        (var_fn487_calc_ig__expbdarg1, var_fn487_calc_ig__expbdarg1_dn0, var_fn487_calc_ig__expbdarg1_dn2, var_fn487_calc_ig__expbdarg1_dn4, var_fn487_calc_ig__expbdarg1_dn8, var_fn487_calc_ig__expbdarg1_dn18, var_fn487_calc_ig__expbdarg1_dn19,)
    }
};
        var_fn487_calc_ig__expbdarg1 = assign45660_e44019;
        var_fn487_calc_ig__expbdarg1_dn0 = assign45660_e44019_d_n0;
        var_fn487_calc_ig__expbdarg1_dn2 = assign45660_e44019_d_n2;
        var_fn487_calc_ig__expbdarg1_dn4 = assign45660_e44019_d_n4;
        var_fn487_calc_ig__expbdarg1_dn8 = assign45660_e44019_d_n8;
        var_fn487_calc_ig__expbdarg1_dn18 = assign45660_e44019_d_n18;
        var_fn487_calc_ig__expbdarg1_dn19 = assign45660_e44019_d_n19;

        let (assign45670_e44028, assign45670_e44028_d_n4,) = {
    if (var_guard480 != 0.0) {
        let assign45670_e44022: f64 = (-var_fn487_calc_ig__pbdgin);
        let assign45670_e44024: f64 = (assign45670_e44022 * var_fn487_calc_ig__vbdgin);
        let assign45670_e44026: f64 = (assign45670_e44024 + var_fn487_calc_ig__expphib);
        (assign45670_e44026, var_fn487_calc_ig__expphib_dn4,)
    } else {
        (var_fn487_calc_ig__expbdarg2, var_fn487_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn487_calc_ig__expbdarg2 = assign45670_e44028;
        var_fn487_calc_ig__expbdarg2_dn4 = assign45670_e44028_d_n4;

        let (assign45680_e44070, assign45680_e44070_d_n0, assign45680_e44070_d_n2, assign45680_e44070_d_n4, assign45680_e44070_d_n8, assign45680_e44070_d_n18, assign45680_e44070_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign45680_e44036: f64 = (-50.0);
        let (assign45680_e44068, assign45680_e44068_d_n0, assign45680_e44068_d_n2, assign45680_e44068_d_n4, assign45680_e44068_d_n8, assign45680_e44068_d_n18, assign45680_e44068_d_n19,) = {
            if ((!(var_fn487_calc_ig__expbdarg1 > 50.0)) && (!(var_fn487_calc_ig__expbdarg1 < assign45680_e44036))) {
                let assign45680_e44041: f64 = (var_fn487_calc_ig__expbdarg1).exp();
                (assign45680_e44041, (assign45680_e44041 * var_fn487_calc_ig__expbdarg1_dn0), (assign45680_e44041 * var_fn487_calc_ig__expbdarg1_dn2), (assign45680_e44041 * var_fn487_calc_ig__expbdarg1_dn4), (assign45680_e44041 * var_fn487_calc_ig__expbdarg1_dn8), (assign45680_e44041 * var_fn487_calc_ig__expbdarg1_dn18), (assign45680_e44041 * var_fn487_calc_ig__expbdarg1_dn19),)
            } else {
                let assign45680_e44048: f64 = (-50.0);
                let (assign45680_e44067, assign45680_e44067_d_n0, assign45680_e44067_d_n2, assign45680_e44067_d_n4, assign45680_e44067_d_n8, assign45680_e44067_d_n18, assign45680_e44067_d_n19,) = {
                    if ((!(var_fn487_calc_ig__expbdarg1 > 50.0)) && (var_fn487_calc_ig__expbdarg1 < assign45680_e44048)) {
                        let assign45680_e44052: f64 = (-50.0);
                        let assign45680_e44053: f64 = (assign45680_e44052).exp();
                        (assign45680_e44053, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign45680_e44066, assign45680_e44066_d_n0, assign45680_e44066_d_n2, assign45680_e44066_d_n4, assign45680_e44066_d_n8, assign45680_e44066_d_n18, assign45680_e44066_d_n19,) = {
                            if (var_fn487_calc_ig__expbdarg1 > 50.0) {
                                let assign45680_e44058: f64 = (50.0_f64).exp();
                                let assign45680_e44062: f64 = (var_fn487_calc_ig__expbdarg1 - 50.0);
                                let assign45680_e44063: f64 = (1.0 + assign45680_e44062);
                                let assign45680_e44064: f64 = (assign45680_e44058 * assign45680_e44063);
                                (assign45680_e44064, (assign45680_e44058 * var_fn487_calc_ig__expbdarg1_dn0), (assign45680_e44058 * var_fn487_calc_ig__expbdarg1_dn2), (assign45680_e44058 * var_fn487_calc_ig__expbdarg1_dn4), (assign45680_e44058 * var_fn487_calc_ig__expbdarg1_dn8), (assign45680_e44058 * var_fn487_calc_ig__expbdarg1_dn18), (assign45680_e44058 * var_fn487_calc_ig__expbdarg1_dn19),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign45680_e44066, assign45680_e44066_d_n0, assign45680_e44066_d_n2, assign45680_e44066_d_n4, assign45680_e44066_d_n8, assign45680_e44066_d_n18, assign45680_e44066_d_n19,)
                    }
                };
                (assign45680_e44067, assign45680_e44067_d_n0, assign45680_e44067_d_n2, assign45680_e44067_d_n4, assign45680_e44067_d_n8, assign45680_e44067_d_n18, assign45680_e44067_d_n19,)
            }
        };
        (assign45680_e44068, assign45680_e44068_d_n0, assign45680_e44068_d_n2, assign45680_e44068_d_n4, assign45680_e44068_d_n8, assign45680_e44068_d_n18, assign45680_e44068_d_n19,)
    } else {
        (var_fn487_calc_ig__expbd1, var_fn487_calc_ig__expbd1_dn0, var_fn487_calc_ig__expbd1_dn2, var_fn487_calc_ig__expbd1_dn4, var_fn487_calc_ig__expbd1_dn8, var_fn487_calc_ig__expbd1_dn18, var_fn487_calc_ig__expbd1_dn19,)
    }
};
        var_fn487_calc_ig__expbd1 = assign45680_e44070;
        var_fn487_calc_ig__expbd1_dn0 = assign45680_e44070_d_n0;
        var_fn487_calc_ig__expbd1_dn2 = assign45680_e44070_d_n2;
        var_fn487_calc_ig__expbd1_dn4 = assign45680_e44070_d_n4;
        var_fn487_calc_ig__expbd1_dn8 = assign45680_e44070_d_n8;
        var_fn487_calc_ig__expbd1_dn18 = assign45680_e44070_d_n18;
        var_fn487_calc_ig__expbd1_dn19 = assign45680_e44070_d_n19;

        let (assign45690_e44112, assign45690_e44112_d_n4,) = {
    if (var_guard480 != 0.0) {
        let assign45690_e44078: f64 = (-50.0);
        let (assign45690_e44110, assign45690_e44110_d_n4,) = {
            if ((!(var_fn487_calc_ig__expbdarg2 > 50.0)) && (!(var_fn487_calc_ig__expbdarg2 < assign45690_e44078))) {
                let assign45690_e44083: f64 = (var_fn487_calc_ig__expbdarg2).exp();
                (assign45690_e44083, (assign45690_e44083 * var_fn487_calc_ig__expbdarg2_dn4),)
            } else {
                let assign45690_e44090: f64 = (-50.0);
                let (assign45690_e44109, assign45690_e44109_d_n4,) = {
                    if ((!(var_fn487_calc_ig__expbdarg2 > 50.0)) && (var_fn487_calc_ig__expbdarg2 < assign45690_e44090)) {
                        let assign45690_e44094: f64 = (-50.0);
                        let assign45690_e44095: f64 = (assign45690_e44094).exp();
                        (assign45690_e44095, 0.0,)
                    } else {
                        let (assign45690_e44108, assign45690_e44108_d_n4,) = {
                            if (var_fn487_calc_ig__expbdarg2 > 50.0) {
                                let assign45690_e44100: f64 = (50.0_f64).exp();
                                let assign45690_e44104: f64 = (var_fn487_calc_ig__expbdarg2 - 50.0);
                                let assign45690_e44105: f64 = (1.0 + assign45690_e44104);
                                let assign45690_e44106: f64 = (assign45690_e44100 * assign45690_e44105);
                                (assign45690_e44106, (assign45690_e44100 * var_fn487_calc_ig__expbdarg2_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign45690_e44108, assign45690_e44108_d_n4,)
                    }
                };
                (assign45690_e44109, assign45690_e44109_d_n4,)
            }
        };
        (assign45690_e44110, assign45690_e44110_d_n4,)
    } else {
        (var_fn487_calc_ig__expbd2, var_fn487_calc_ig__expbd2_dn4,)
    }
};
        var_fn487_calc_ig__expbd2 = assign45690_e44112;
        var_fn487_calc_ig__expbd2_dn4 = assign45690_e44112_d_n4;

        let (assign45700_e44118, assign45700_e44118_d_n0, assign45700_e44118_d_n2, assign45700_e44118_d_n4, assign45700_e44118_d_n8, assign45700_e44118_d_n18, assign45700_e44118_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign45700_e44116: f64 = (var_fn487_calc_ig__expbd1 - var_fn487_calc_ig__expbd2);
        (assign45700_e44116, var_fn487_calc_ig__expbd1_dn0, var_fn487_calc_ig__expbd1_dn2, (var_fn487_calc_ig__expbd1_dn4 - var_fn487_calc_ig__expbd2_dn4), var_fn487_calc_ig__expbd1_dn8, var_fn487_calc_ig__expbd1_dn18, var_fn487_calc_ig__expbd1_dn19,)
    } else {
        (var_fn487_calc_ig__iginbd, var_fn487_calc_ig__iginbd_dn0, var_fn487_calc_ig__iginbd_dn2, var_fn487_calc_ig__iginbd_dn4, var_fn487_calc_ig__iginbd_dn8, var_fn487_calc_ig__iginbd_dn18, var_fn487_calc_ig__iginbd_dn19,)
    }
};
        var_fn487_calc_ig__iginbd = assign45700_e44118;
        var_fn487_calc_ig__iginbd_dn0 = assign45700_e44118_d_n0;
        var_fn487_calc_ig__iginbd_dn2 = assign45700_e44118_d_n2;
        var_fn487_calc_ig__iginbd_dn4 = assign45700_e44118_d_n4;
        var_fn487_calc_ig__iginbd_dn8 = assign45700_e44118_d_n8;
        var_fn487_calc_ig__iginbd_dn18 = assign45700_e44118_d_n18;
        var_fn487_calc_ig__iginbd_dn19 = assign45700_e44118_d_n19;

        let (assign45710_e44130, assign45710_e44130_d_n4,) = {
    if (var_guard480 != 0.0) {
        let assign45710_e44122: f64 = (var_fn487_calc_ig__type * var_fn487_calc_ig__w);
        let assign45710_e44124: f64 = (assign45710_e44122 * var_fn487_calc_ig__ngf);
        let assign45710_e44126: f64 = (assign45710_e44124 * var_fn487_calc_ig__ijin);
        let assign45710_e44128: f64 = (assign45710_e44126 * var_fn487_calc_ig__tfacdiodein);
        (assign45710_e44128, (assign45710_e44126 * var_fn487_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn487_calc_ig__isdiodeout, var_fn487_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn487_calc_ig__isdiodeout = assign45710_e44130;
        var_fn487_calc_ig__isdiodeout_dn4 = assign45710_e44130_d_n4;

        let (assign45720_e44140, assign45720_e44140_d_n0, assign45720_e44140_d_n2, assign45720_e44140_d_n4, assign45720_e44140_d_n8, assign45720_e44140_d_n18, assign45720_e44140_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign45720_e44134: f64 = (var_fn487_calc_ig__pg_paramin / var_fn487_calc_ig__phitin);
        let assign45720_e44136: f64 = (assign45720_e44134 * var_fn487_calc_ig__vgin);
        let assign45720_e44138: f64 = (assign45720_e44136 + var_fn487_calc_ig__expphib);
        (assign45720_e44138, (assign45720_e44134 * var_fn487_calc_ig__vgin_dn0), (assign45720_e44134 * var_fn487_calc_ig__vgin_dn2), (((-((var_fn487_calc_ig__pg_paramin * var_fn487_calc_ig__phitin_dn4) / (var_fn487_calc_ig__phitin * var_fn487_calc_ig__phitin))) * var_fn487_calc_ig__vgin) + var_fn487_calc_ig__expphib_dn4), (assign45720_e44134 * var_fn487_calc_ig__vgin_dn8), (assign45720_e44134 * var_fn487_calc_ig__vgin_dn18), (assign45720_e44134 * var_fn487_calc_ig__vgin_dn19),)
    } else {
        (var_fn487_calc_ig__expiforarg, var_fn487_calc_ig__expiforarg_dn0, var_fn487_calc_ig__expiforarg_dn2, var_fn487_calc_ig__expiforarg_dn4, var_fn487_calc_ig__expiforarg_dn8, var_fn487_calc_ig__expiforarg_dn18, var_fn487_calc_ig__expiforarg_dn19,)
    }
};
        var_fn487_calc_ig__expiforarg = assign45720_e44140;
        var_fn487_calc_ig__expiforarg_dn0 = assign45720_e44140_d_n0;
        var_fn487_calc_ig__expiforarg_dn2 = assign45720_e44140_d_n2;
        var_fn487_calc_ig__expiforarg_dn4 = assign45720_e44140_d_n4;
        var_fn487_calc_ig__expiforarg_dn8 = assign45720_e44140_d_n8;
        var_fn487_calc_ig__expiforarg_dn18 = assign45720_e44140_d_n18;
        var_fn487_calc_ig__expiforarg_dn19 = assign45720_e44140_d_n19;

        let (assign45730_e44182, assign45730_e44182_d_n0, assign45730_e44182_d_n2, assign45730_e44182_d_n4, assign45730_e44182_d_n8, assign45730_e44182_d_n18, assign45730_e44182_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign45730_e44148: f64 = (-50.0);
        let (assign45730_e44180, assign45730_e44180_d_n0, assign45730_e44180_d_n2, assign45730_e44180_d_n4, assign45730_e44180_d_n8, assign45730_e44180_d_n18, assign45730_e44180_d_n19,) = {
            if ((!(var_fn487_calc_ig__expiforarg > 50.0)) && (!(var_fn487_calc_ig__expiforarg < assign45730_e44148))) {
                let assign45730_e44153: f64 = (var_fn487_calc_ig__expiforarg).exp();
                (assign45730_e44153, (assign45730_e44153 * var_fn487_calc_ig__expiforarg_dn0), (assign45730_e44153 * var_fn487_calc_ig__expiforarg_dn2), (assign45730_e44153 * var_fn487_calc_ig__expiforarg_dn4), (assign45730_e44153 * var_fn487_calc_ig__expiforarg_dn8), (assign45730_e44153 * var_fn487_calc_ig__expiforarg_dn18), (assign45730_e44153 * var_fn487_calc_ig__expiforarg_dn19),)
            } else {
                let assign45730_e44160: f64 = (-50.0);
                let (assign45730_e44179, assign45730_e44179_d_n0, assign45730_e44179_d_n2, assign45730_e44179_d_n4, assign45730_e44179_d_n8, assign45730_e44179_d_n18, assign45730_e44179_d_n19,) = {
                    if ((!(var_fn487_calc_ig__expiforarg > 50.0)) && (var_fn487_calc_ig__expiforarg < assign45730_e44160)) {
                        let assign45730_e44164: f64 = (-50.0);
                        let assign45730_e44165: f64 = (assign45730_e44164).exp();
                        (assign45730_e44165, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign45730_e44178, assign45730_e44178_d_n0, assign45730_e44178_d_n2, assign45730_e44178_d_n4, assign45730_e44178_d_n8, assign45730_e44178_d_n18, assign45730_e44178_d_n19,) = {
                            if (var_fn487_calc_ig__expiforarg > 50.0) {
                                let assign45730_e44170: f64 = (50.0_f64).exp();
                                let assign45730_e44174: f64 = (var_fn487_calc_ig__expiforarg - 50.0);
                                let assign45730_e44175: f64 = (1.0 + assign45730_e44174);
                                let assign45730_e44176: f64 = (assign45730_e44170 * assign45730_e44175);
                                (assign45730_e44176, (assign45730_e44170 * var_fn487_calc_ig__expiforarg_dn0), (assign45730_e44170 * var_fn487_calc_ig__expiforarg_dn2), (assign45730_e44170 * var_fn487_calc_ig__expiforarg_dn4), (assign45730_e44170 * var_fn487_calc_ig__expiforarg_dn8), (assign45730_e44170 * var_fn487_calc_ig__expiforarg_dn18), (assign45730_e44170 * var_fn487_calc_ig__expiforarg_dn19),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign45730_e44178, assign45730_e44178_d_n0, assign45730_e44178_d_n2, assign45730_e44178_d_n4, assign45730_e44178_d_n8, assign45730_e44178_d_n18, assign45730_e44178_d_n19,)
                    }
                };
                (assign45730_e44179, assign45730_e44179_d_n0, assign45730_e44179_d_n2, assign45730_e44179_d_n4, assign45730_e44179_d_n8, assign45730_e44179_d_n18, assign45730_e44179_d_n19,)
            }
        };
        (assign45730_e44180, assign45730_e44180_d_n0, assign45730_e44180_d_n2, assign45730_e44180_d_n4, assign45730_e44180_d_n8, assign45730_e44180_d_n18, assign45730_e44180_d_n19,)
    } else {
        (var_fn487_calc_ig__expifor, var_fn487_calc_ig__expifor_dn0, var_fn487_calc_ig__expifor_dn2, var_fn487_calc_ig__expifor_dn4, var_fn487_calc_ig__expifor_dn8, var_fn487_calc_ig__expifor_dn18, var_fn487_calc_ig__expifor_dn19,)
    }
};
        var_fn487_calc_ig__expifor = assign45730_e44182;
        var_fn487_calc_ig__expifor_dn0 = assign45730_e44182_d_n0;
        var_fn487_calc_ig__expifor_dn2 = assign45730_e44182_d_n2;
        var_fn487_calc_ig__expifor_dn4 = assign45730_e44182_d_n4;
        var_fn487_calc_ig__expifor_dn8 = assign45730_e44182_d_n8;
        var_fn487_calc_ig__expifor_dn18 = assign45730_e44182_d_n18;
        var_fn487_calc_ig__expifor_dn19 = assign45730_e44182_d_n19;

        let assign45740_e44185: f64 = if var_fn487_calc_ig__fracin == 1.0 { 1.0 } else { 0.0 };
        var_guard488 = assign45740_e44185;

        *var_fn487_calc_ig__expbd1_slot = var_fn487_calc_ig__expbd1;
        *var_fn487_calc_ig__expbd1_dn0_slot = var_fn487_calc_ig__expbd1_dn0;
        *var_fn487_calc_ig__expbd1_dn18_slot = var_fn487_calc_ig__expbd1_dn18;
        *var_fn487_calc_ig__expbd1_dn19_slot = var_fn487_calc_ig__expbd1_dn19;
        *var_fn487_calc_ig__expbd1_dn2_slot = var_fn487_calc_ig__expbd1_dn2;
        *var_fn487_calc_ig__expbd1_dn4_slot = var_fn487_calc_ig__expbd1_dn4;
        *var_fn487_calc_ig__expbd1_dn8_slot = var_fn487_calc_ig__expbd1_dn8;
        *var_fn487_calc_ig__expbd1_vgsat_slot = var_fn487_calc_ig__expbd1_vgsat;
        *var_fn487_calc_ig__expbd1_vgsat_dn4_slot = var_fn487_calc_ig__expbd1_vgsat_dn4;
        *var_fn487_calc_ig__expbd2_slot = var_fn487_calc_ig__expbd2;
        *var_fn487_calc_ig__expbd2_dn4_slot = var_fn487_calc_ig__expbd2_dn4;
        *var_fn487_calc_ig__expbdarg1_slot = var_fn487_calc_ig__expbdarg1;
        *var_fn487_calc_ig__expbdarg1_dn0_slot = var_fn487_calc_ig__expbdarg1_dn0;
        *var_fn487_calc_ig__expbdarg1_dn18_slot = var_fn487_calc_ig__expbdarg1_dn18;
        *var_fn487_calc_ig__expbdarg1_dn19_slot = var_fn487_calc_ig__expbdarg1_dn19;
        *var_fn487_calc_ig__expbdarg1_dn2_slot = var_fn487_calc_ig__expbdarg1_dn2;
        *var_fn487_calc_ig__expbdarg1_dn4_slot = var_fn487_calc_ig__expbdarg1_dn4;
        *var_fn487_calc_ig__expbdarg1_dn8_slot = var_fn487_calc_ig__expbdarg1_dn8;
        *var_fn487_calc_ig__expbdarg1_vgsat_slot = var_fn487_calc_ig__expbdarg1_vgsat;
        *var_fn487_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn487_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn487_calc_ig__expbdarg2_slot = var_fn487_calc_ig__expbdarg2;
        *var_fn487_calc_ig__expbdarg2_dn4_slot = var_fn487_calc_ig__expbdarg2_dn4;
        *var_fn487_calc_ig__expffvarg_slot = var_fn487_calc_ig__expffvarg;
        *var_fn487_calc_ig__expffvarg_dn0_slot = var_fn487_calc_ig__expffvarg_dn0;
        *var_fn487_calc_ig__expffvarg_dn18_slot = var_fn487_calc_ig__expffvarg_dn18;
        *var_fn487_calc_ig__expffvarg_dn19_slot = var_fn487_calc_ig__expffvarg_dn19;
        *var_fn487_calc_ig__expffvarg_dn2_slot = var_fn487_calc_ig__expffvarg_dn2;
        *var_fn487_calc_ig__expffvarg_dn4_slot = var_fn487_calc_ig__expffvarg_dn4;
        *var_fn487_calc_ig__expffvarg_dn8_slot = var_fn487_calc_ig__expffvarg_dn8;
        *var_fn487_calc_ig__expifor_slot = var_fn487_calc_ig__expifor;
        *var_fn487_calc_ig__expifor_dn0_slot = var_fn487_calc_ig__expifor_dn0;
        *var_fn487_calc_ig__expifor_dn18_slot = var_fn487_calc_ig__expifor_dn18;
        *var_fn487_calc_ig__expifor_dn19_slot = var_fn487_calc_ig__expifor_dn19;
        *var_fn487_calc_ig__expifor_dn2_slot = var_fn487_calc_ig__expifor_dn2;
        *var_fn487_calc_ig__expifor_dn4_slot = var_fn487_calc_ig__expifor_dn4;
        *var_fn487_calc_ig__expifor_dn8_slot = var_fn487_calc_ig__expifor_dn8;
        *var_fn487_calc_ig__expifor_hinj_slot = var_fn487_calc_ig__expifor_hinj;
        *var_fn487_calc_ig__expifor_hinj_dn0_slot = var_fn487_calc_ig__expifor_hinj_dn0;
        *var_fn487_calc_ig__expifor_hinj_dn18_slot = var_fn487_calc_ig__expifor_hinj_dn18;
        *var_fn487_calc_ig__expifor_hinj_dn19_slot = var_fn487_calc_ig__expifor_hinj_dn19;
        *var_fn487_calc_ig__expifor_hinj_dn2_slot = var_fn487_calc_ig__expifor_hinj_dn2;
        *var_fn487_calc_ig__expifor_hinj_dn4_slot = var_fn487_calc_ig__expifor_hinj_dn4;
        *var_fn487_calc_ig__expifor_hinj_dn8_slot = var_fn487_calc_ig__expifor_hinj_dn8;
        *var_fn487_calc_ig__expifor_hinj_vgsat_slot = var_fn487_calc_ig__expifor_hinj_vgsat;
        *var_fn487_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn487_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn487_calc_ig__expifor_nohinj_vgsat_slot = var_fn487_calc_ig__expifor_nohinj_vgsat;
        *var_fn487_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn487_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn487_calc_ig__expiforarg_slot = var_fn487_calc_ig__expiforarg;
        *var_fn487_calc_ig__expiforarg_dn0_slot = var_fn487_calc_ig__expiforarg_dn0;
        *var_fn487_calc_ig__expiforarg_dn18_slot = var_fn487_calc_ig__expiforarg_dn18;
        *var_fn487_calc_ig__expiforarg_dn19_slot = var_fn487_calc_ig__expiforarg_dn19;
        *var_fn487_calc_ig__expiforarg_dn2_slot = var_fn487_calc_ig__expiforarg_dn2;
        *var_fn487_calc_ig__expiforarg_dn4_slot = var_fn487_calc_ig__expiforarg_dn4;
        *var_fn487_calc_ig__expiforarg_dn8_slot = var_fn487_calc_ig__expiforarg_dn8;
        *var_fn487_calc_ig__expiforarg_hinj_slot = var_fn487_calc_ig__expiforarg_hinj;
        *var_fn487_calc_ig__expiforarg_hinj_dn0_slot = var_fn487_calc_ig__expiforarg_hinj_dn0;
        *var_fn487_calc_ig__expiforarg_hinj_dn18_slot = var_fn487_calc_ig__expiforarg_hinj_dn18;
        *var_fn487_calc_ig__expiforarg_hinj_dn19_slot = var_fn487_calc_ig__expiforarg_hinj_dn19;
        *var_fn487_calc_ig__expiforarg_hinj_dn2_slot = var_fn487_calc_ig__expiforarg_hinj_dn2;
        *var_fn487_calc_ig__expiforarg_hinj_dn4_slot = var_fn487_calc_ig__expiforarg_hinj_dn4;
        *var_fn487_calc_ig__expiforarg_hinj_dn8_slot = var_fn487_calc_ig__expiforarg_hinj_dn8;
        *var_fn487_calc_ig__expiforarg_hinj_vgsat_slot = var_fn487_calc_ig__expiforarg_hinj_vgsat;
        *var_fn487_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn487_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn487_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn487_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn487_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn487_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn487_calc_ig__expirev_slot = var_fn487_calc_ig__expirev;
        *var_fn487_calc_ig__expirev_dn0_slot = var_fn487_calc_ig__expirev_dn0;
        *var_fn487_calc_ig__expirev_dn18_slot = var_fn487_calc_ig__expirev_dn18;
        *var_fn487_calc_ig__expirev_dn19_slot = var_fn487_calc_ig__expirev_dn19;
        *var_fn487_calc_ig__expirev_dn2_slot = var_fn487_calc_ig__expirev_dn2;
        *var_fn487_calc_ig__expirev_dn4_slot = var_fn487_calc_ig__expirev_dn4;
        *var_fn487_calc_ig__expirev_dn8_slot = var_fn487_calc_ig__expirev_dn8;
        *var_fn487_calc_ig__expirevarg_slot = var_fn487_calc_ig__expirevarg;
        *var_fn487_calc_ig__expirevarg_dn0_slot = var_fn487_calc_ig__expirevarg_dn0;
        *var_fn487_calc_ig__expirevarg_dn18_slot = var_fn487_calc_ig__expirevarg_dn18;
        *var_fn487_calc_ig__expirevarg_dn19_slot = var_fn487_calc_ig__expirevarg_dn19;
        *var_fn487_calc_ig__expirevarg_dn2_slot = var_fn487_calc_ig__expirevarg_dn2;
        *var_fn487_calc_ig__expirevarg_dn4_slot = var_fn487_calc_ig__expirevarg_dn4;
        *var_fn487_calc_ig__expirevarg_dn8_slot = var_fn487_calc_ig__expirevarg_dn8;
        *var_fn487_calc_ig__expphib_slot = var_fn487_calc_ig__expphib;
        *var_fn487_calc_ig__expphib_dn4_slot = var_fn487_calc_ig__expphib_dn4;
        *var_fn487_calc_ig__iginbd_slot = var_fn487_calc_ig__iginbd;
        *var_fn487_calc_ig__iginbd_dn0_slot = var_fn487_calc_ig__iginbd_dn0;
        *var_fn487_calc_ig__iginbd_dn18_slot = var_fn487_calc_ig__iginbd_dn18;
        *var_fn487_calc_ig__iginbd_dn19_slot = var_fn487_calc_ig__iginbd_dn19;
        *var_fn487_calc_ig__iginbd_dn2_slot = var_fn487_calc_ig__iginbd_dn2;
        *var_fn487_calc_ig__iginbd_dn4_slot = var_fn487_calc_ig__iginbd_dn4;
        *var_fn487_calc_ig__iginbd_dn8_slot = var_fn487_calc_ig__iginbd_dn8;
        *var_fn487_calc_ig__iginbd_vgsat_slot = var_fn487_calc_ig__iginbd_vgsat;
        *var_fn487_calc_ig__iginbd_vgsat_dn4_slot = var_fn487_calc_ig__iginbd_vgsat_dn4;
        *var_fn487_calc_ig__igindiode_hinj_slot = var_fn487_calc_ig__igindiode_hinj;
        *var_fn487_calc_ig__igindiode_hinj_dn0_slot = var_fn487_calc_ig__igindiode_hinj_dn0;
        *var_fn487_calc_ig__igindiode_hinj_dn18_slot = var_fn487_calc_ig__igindiode_hinj_dn18;
        *var_fn487_calc_ig__igindiode_hinj_dn19_slot = var_fn487_calc_ig__igindiode_hinj_dn19;
        *var_fn487_calc_ig__igindiode_hinj_dn2_slot = var_fn487_calc_ig__igindiode_hinj_dn2;
        *var_fn487_calc_ig__igindiode_hinj_dn4_slot = var_fn487_calc_ig__igindiode_hinj_dn4;
        *var_fn487_calc_ig__igindiode_hinj_dn8_slot = var_fn487_calc_ig__igindiode_hinj_dn8;
        *var_fn487_calc_ig__igindiode_hinj_pre_slot = var_fn487_calc_ig__igindiode_hinj_pre;
        *var_fn487_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn487_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn487_calc_ig__igindiode_hinj_vgsat_slot = var_fn487_calc_ig__igindiode_hinj_vgsat;
        *var_fn487_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn487_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn487_calc_ig__igindiode_nohinj_slot = var_fn487_calc_ig__igindiode_nohinj;
        *var_fn487_calc_ig__igindiode_nohinj_dn0_slot = var_fn487_calc_ig__igindiode_nohinj_dn0;
        *var_fn487_calc_ig__igindiode_nohinj_dn18_slot = var_fn487_calc_ig__igindiode_nohinj_dn18;
        *var_fn487_calc_ig__igindiode_nohinj_dn19_slot = var_fn487_calc_ig__igindiode_nohinj_dn19;
        *var_fn487_calc_ig__igindiode_nohinj_dn2_slot = var_fn487_calc_ig__igindiode_nohinj_dn2;
        *var_fn487_calc_ig__igindiode_nohinj_dn4_slot = var_fn487_calc_ig__igindiode_nohinj_dn4;
        *var_fn487_calc_ig__igindiode_nohinj_dn8_slot = var_fn487_calc_ig__igindiode_nohinj_dn8;
        *var_fn487_calc_ig__igindiode_nohinj_vgsat_slot = var_fn487_calc_ig__igindiode_nohinj_vgsat;
        *var_fn487_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn487_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn487_calc_ig__isdiodeout_slot = var_fn487_calc_ig__isdiodeout;
        *var_fn487_calc_ig__isdiodeout_dn4_slot = var_fn487_calc_ig__isdiodeout_dn4;
        *var_fn487_calc_ig__pg_paramin_hinj_slot = var_fn487_calc_ig__pg_paramin_hinj;
        *var_fn487_calc_ig__t0_slot = var_fn487_calc_ig__t0;
        *var_fn487_calc_ig__t0_dn4_slot = var_fn487_calc_ig__t0_dn4;
        *var_guard488_slot = var_guard488;
    }

    pub(super) fn stamp_transient_block_115(
        p: &Parameters,
        var_fn487_calc_ig__alphagin: f64,
        var_fn487_calc_ig__betarecin: f64,
        var_fn487_calc_ig__expbd2: f64,
        var_fn487_calc_ig__expbd2_dn4: f64,
        var_fn487_calc_ig__expifor: f64,
        var_fn487_calc_ig__expifor_dn0: f64,
        var_fn487_calc_ig__expifor_dn18: f64,
        var_fn487_calc_ig__expifor_dn19: f64,
        var_fn487_calc_ig__expifor_dn2: f64,
        var_fn487_calc_ig__expifor_dn4: f64,
        var_fn487_calc_ig__expifor_dn8: f64,
        var_fn487_calc_ig__expphib: f64,
        var_fn487_calc_ig__expphib_dn4: f64,
        var_fn487_calc_ig__fracin: f64,
        var_fn487_calc_ig__iginbd: f64,
        var_fn487_calc_ig__iginbd_dn0: f64,
        var_fn487_calc_ig__iginbd_dn18: f64,
        var_fn487_calc_ig__iginbd_dn19: f64,
        var_fn487_calc_ig__iginbd_dn2: f64,
        var_fn487_calc_ig__iginbd_dn4: f64,
        var_fn487_calc_ig__iginbd_dn8: f64,
        var_fn487_calc_ig__isdiodeout: f64,
        var_fn487_calc_ig__isdiodeout_dn4: f64,
        var_fn487_calc_ig__kbdgatein: f64,
        var_fn487_calc_ig__pbdgin: f64,
        var_fn487_calc_ig__pg_paramin: f64,
        var_fn487_calc_ig__phitin: f64,
        var_fn487_calc_ig__phitin_dn4: f64,
        var_fn487_calc_ig__t0: f64,
        var_fn487_calc_ig__t0_dn4: f64,
        var_fn487_calc_ig__vbdgin: f64,
        var_fn487_calc_ig__vgin: f64,
        var_fn487_calc_ig__vgin_dn0: f64,
        var_fn487_calc_ig__vgin_dn18: f64,
        var_fn487_calc_ig__vgin_dn19: f64,
        var_fn487_calc_ig__vgin_dn2: f64,
        var_fn487_calc_ig__vgin_dn8: f64,
        var_fn487_calc_ig__vgsatin: f64,
        var_fn487_calc_ig__vgsatqin: f64,
        var_guard480: f64,
        var_guard488: f64,
        var_fn487_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn487_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn487_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn487_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn487_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__expffvarg_slot: &mut f64,
        var_fn487_calc_ig__expffvarg_dn0_slot: &mut f64,
        var_fn487_calc_ig__expffvarg_dn18_slot: &mut f64,
        var_fn487_calc_ig__expffvarg_dn19_slot: &mut f64,
        var_fn487_calc_ig__expffvarg_dn2_slot: &mut f64,
        var_fn487_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn487_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_dn0_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_dn18_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_dn19_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_dn2_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn487_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn487_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_dn0_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_dn18_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_dn19_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_dn2_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn487_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__ffvgin_slot: &mut f64,
        var_fn487_calc_ig__ffvgin_dn0_slot: &mut f64,
        var_fn487_calc_ig__ffvgin_dn18_slot: &mut f64,
        var_fn487_calc_ig__ffvgin_dn19_slot: &mut f64,
        var_fn487_calc_ig__ffvgin_dn2_slot: &mut f64,
        var_fn487_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn487_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn487_calc_ig__frecgin_slot: &mut f64,
        var_fn487_calc_ig__frecgin_dn0_slot: &mut f64,
        var_fn487_calc_ig__frecgin_dn18_slot: &mut f64,
        var_fn487_calc_ig__frecgin_dn19_slot: &mut f64,
        var_fn487_calc_ig__frecgin_dn2_slot: &mut f64,
        var_fn487_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn487_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn487_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__igindiode_slot: &mut f64,
        var_fn487_calc_ig__igindiode_dn0_slot: &mut f64,
        var_fn487_calc_ig__igindiode_dn18_slot: &mut f64,
        var_fn487_calc_ig__igindiode_dn19_slot: &mut f64,
        var_fn487_calc_ig__igindiode_dn2_slot: &mut f64,
        var_fn487_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn487_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_dn0_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_dn18_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_dn19_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_dn2_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn487_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_dn0_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_dn18_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_dn19_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_dn2_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn487_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn487_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_guard489_slot: &mut f64,
        var_guard490_slot: &mut f64,
        var_guard491_slot: &mut f64,
    ) {
        let mut var_fn487_calc_ig__alpha2_phit: f64 = *var_fn487_calc_ig__alpha2_phit_slot;
        let mut var_fn487_calc_ig__alpha2_phit_dn4: f64 = *var_fn487_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn487_calc_ig__expbd1_vgsat: f64 = *var_fn487_calc_ig__expbd1_vgsat_slot;
        let mut var_fn487_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn487_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__expbdarg1_vgsat: f64 = *var_fn487_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn487_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn487_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__expffvarg: f64 = *var_fn487_calc_ig__expffvarg_slot;
        let mut var_fn487_calc_ig__expffvarg_dn0: f64 = *var_fn487_calc_ig__expffvarg_dn0_slot;
        let mut var_fn487_calc_ig__expffvarg_dn18: f64 = *var_fn487_calc_ig__expffvarg_dn18_slot;
        let mut var_fn487_calc_ig__expffvarg_dn19: f64 = *var_fn487_calc_ig__expffvarg_dn19_slot;
        let mut var_fn487_calc_ig__expffvarg_dn2: f64 = *var_fn487_calc_ig__expffvarg_dn2_slot;
        let mut var_fn487_calc_ig__expffvarg_dn4: f64 = *var_fn487_calc_ig__expffvarg_dn4_slot;
        let mut var_fn487_calc_ig__expffvarg_dn8: f64 = *var_fn487_calc_ig__expffvarg_dn8_slot;
        let mut var_fn487_calc_ig__expifor_hinj: f64 = *var_fn487_calc_ig__expifor_hinj_slot;
        let mut var_fn487_calc_ig__expifor_hinj_dn0: f64 = *var_fn487_calc_ig__expifor_hinj_dn0_slot;
        let mut var_fn487_calc_ig__expifor_hinj_dn18: f64 = *var_fn487_calc_ig__expifor_hinj_dn18_slot;
        let mut var_fn487_calc_ig__expifor_hinj_dn19: f64 = *var_fn487_calc_ig__expifor_hinj_dn19_slot;
        let mut var_fn487_calc_ig__expifor_hinj_dn2: f64 = *var_fn487_calc_ig__expifor_hinj_dn2_slot;
        let mut var_fn487_calc_ig__expifor_hinj_dn4: f64 = *var_fn487_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn487_calc_ig__expifor_hinj_dn8: f64 = *var_fn487_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn487_calc_ig__expifor_hinj_vgsat: f64 = *var_fn487_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn487_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn487_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn487_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn487_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn487_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj: f64 = *var_fn487_calc_ig__expiforarg_hinj_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj_dn0: f64 = *var_fn487_calc_ig__expiforarg_hinj_dn0_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj_dn18: f64 = *var_fn487_calc_ig__expiforarg_hinj_dn18_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj_dn19: f64 = *var_fn487_calc_ig__expiforarg_hinj_dn19_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj_dn2: f64 = *var_fn487_calc_ig__expiforarg_hinj_dn2_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn487_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn487_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn487_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn487_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn487_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn487_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn487_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn487_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__ffvgin: f64 = *var_fn487_calc_ig__ffvgin_slot;
        let mut var_fn487_calc_ig__ffvgin_dn0: f64 = *var_fn487_calc_ig__ffvgin_dn0_slot;
        let mut var_fn487_calc_ig__ffvgin_dn18: f64 = *var_fn487_calc_ig__ffvgin_dn18_slot;
        let mut var_fn487_calc_ig__ffvgin_dn19: f64 = *var_fn487_calc_ig__ffvgin_dn19_slot;
        let mut var_fn487_calc_ig__ffvgin_dn2: f64 = *var_fn487_calc_ig__ffvgin_dn2_slot;
        let mut var_fn487_calc_ig__ffvgin_dn4: f64 = *var_fn487_calc_ig__ffvgin_dn4_slot;
        let mut var_fn487_calc_ig__ffvgin_dn8: f64 = *var_fn487_calc_ig__ffvgin_dn8_slot;
        let mut var_fn487_calc_ig__frecgin: f64 = *var_fn487_calc_ig__frecgin_slot;
        let mut var_fn487_calc_ig__frecgin_dn0: f64 = *var_fn487_calc_ig__frecgin_dn0_slot;
        let mut var_fn487_calc_ig__frecgin_dn18: f64 = *var_fn487_calc_ig__frecgin_dn18_slot;
        let mut var_fn487_calc_ig__frecgin_dn19: f64 = *var_fn487_calc_ig__frecgin_dn19_slot;
        let mut var_fn487_calc_ig__frecgin_dn2: f64 = *var_fn487_calc_ig__frecgin_dn2_slot;
        let mut var_fn487_calc_ig__frecgin_dn8: f64 = *var_fn487_calc_ig__frecgin_dn8_slot;
        let mut var_fn487_calc_ig__iginbd_vgsat: f64 = *var_fn487_calc_ig__iginbd_vgsat_slot;
        let mut var_fn487_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn487_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__igindiode: f64 = *var_fn487_calc_ig__igindiode_slot;
        let mut var_fn487_calc_ig__igindiode_dn0: f64 = *var_fn487_calc_ig__igindiode_dn0_slot;
        let mut var_fn487_calc_ig__igindiode_dn18: f64 = *var_fn487_calc_ig__igindiode_dn18_slot;
        let mut var_fn487_calc_ig__igindiode_dn19: f64 = *var_fn487_calc_ig__igindiode_dn19_slot;
        let mut var_fn487_calc_ig__igindiode_dn2: f64 = *var_fn487_calc_ig__igindiode_dn2_slot;
        let mut var_fn487_calc_ig__igindiode_dn4: f64 = *var_fn487_calc_ig__igindiode_dn4_slot;
        let mut var_fn487_calc_ig__igindiode_dn8: f64 = *var_fn487_calc_ig__igindiode_dn8_slot;
        let mut var_fn487_calc_ig__igindiode_hinj: f64 = *var_fn487_calc_ig__igindiode_hinj_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_dn0: f64 = *var_fn487_calc_ig__igindiode_hinj_dn0_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_dn18: f64 = *var_fn487_calc_ig__igindiode_hinj_dn18_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_dn19: f64 = *var_fn487_calc_ig__igindiode_hinj_dn19_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_dn2: f64 = *var_fn487_calc_ig__igindiode_hinj_dn2_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_dn4: f64 = *var_fn487_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_dn8: f64 = *var_fn487_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_pre: f64 = *var_fn487_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn487_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn487_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn487_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn487_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj: f64 = *var_fn487_calc_ig__igindiode_nohinj_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj_dn0: f64 = *var_fn487_calc_ig__igindiode_nohinj_dn0_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj_dn18: f64 = *var_fn487_calc_ig__igindiode_nohinj_dn18_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj_dn19: f64 = *var_fn487_calc_ig__igindiode_nohinj_dn19_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj_dn2: f64 = *var_fn487_calc_ig__igindiode_nohinj_dn2_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn487_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn487_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn487_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn487_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn487_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn487_calc_ig__pg_paramin_hinj: f64 = *var_fn487_calc_ig__pg_paramin_hinj_slot;
        let mut var_guard489: f64 = *var_guard489_slot;
        let mut var_guard490: f64 = *var_guard490_slot;
        let mut var_guard491: f64 = *var_guard491_slot;

        let (assign45750_e44199, assign45750_e44199_d_n0, assign45750_e44199_d_n2, assign45750_e44199_d_n4, assign45750_e44199_d_n8, assign45750_e44199_d_n18, assign45750_e44199_d_n19,) = {
    if ((var_guard480 != 0.0) && (var_guard488 != 0.0)) {
        let assign45750_e44193: f64 = (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd);
        let assign45750_e44194: f64 = (var_fn487_calc_ig__expifor - assign45750_e44193);
        let assign45750_e44196: f64 = (assign45750_e44194 - var_fn487_calc_ig__t0);
        let assign45750_e44197: f64 = (var_fn487_calc_ig__isdiodeout * assign45750_e44196);
        (assign45750_e44197, (var_fn487_calc_ig__isdiodeout * (var_fn487_calc_ig__expifor_dn0 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn0))), (var_fn487_calc_ig__isdiodeout * (var_fn487_calc_ig__expifor_dn2 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn2))), ((var_fn487_calc_ig__isdiodeout_dn4 * assign45750_e44196) + (var_fn487_calc_ig__isdiodeout * ((var_fn487_calc_ig__expifor_dn4 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn4)) - var_fn487_calc_ig__t0_dn4))), (var_fn487_calc_ig__isdiodeout * (var_fn487_calc_ig__expifor_dn8 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn8))), (var_fn487_calc_ig__isdiodeout * (var_fn487_calc_ig__expifor_dn18 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn18))), (var_fn487_calc_ig__isdiodeout * (var_fn487_calc_ig__expifor_dn19 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn19))),)
    } else {
        (var_fn487_calc_ig__igindiode, var_fn487_calc_ig__igindiode_dn0, var_fn487_calc_ig__igindiode_dn2, var_fn487_calc_ig__igindiode_dn4, var_fn487_calc_ig__igindiode_dn8, var_fn487_calc_ig__igindiode_dn18, var_fn487_calc_ig__igindiode_dn19,)
    }
};
        var_fn487_calc_ig__igindiode = assign45750_e44199;
        var_fn487_calc_ig__igindiode_dn0 = assign45750_e44199_d_n0;
        var_fn487_calc_ig__igindiode_dn2 = assign45750_e44199_d_n2;
        var_fn487_calc_ig__igindiode_dn4 = assign45750_e44199_d_n4;
        var_fn487_calc_ig__igindiode_dn8 = assign45750_e44199_d_n8;
        var_fn487_calc_ig__igindiode_dn18 = assign45750_e44199_d_n18;
        var_fn487_calc_ig__igindiode_dn19 = assign45750_e44199_d_n19;

        let (assign45760_e44213, assign45760_e44213_d_n4,) = {
    if ((var_guard480 != 0.0) && (var_guard488 == 0.0)) {
        let assign45760_e44206: f64 = (-var_fn487_calc_ig__vgsatin);
        let assign45760_e44208: f64 = (assign45760_e44206 - var_fn487_calc_ig__vbdgin);
        let assign45760_e44209: f64 = (var_fn487_calc_ig__pbdgin * assign45760_e44208);
        let assign45760_e44211: f64 = (assign45760_e44209 + var_fn487_calc_ig__expphib);
        (assign45760_e44211, var_fn487_calc_ig__expphib_dn4,)
    } else {
        (var_fn487_calc_ig__expbdarg1_vgsat, var_fn487_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__expbdarg1_vgsat = assign45760_e44213;
        var_fn487_calc_ig__expbdarg1_vgsat_dn4 = assign45760_e44213_d_n4;

        let (assign45770_e44258, assign45770_e44258_d_n4,) = {
    if ((var_guard480 != 0.0) && (var_guard488 == 0.0)) {
        let assign45770_e44224: f64 = (-50.0);
        let (assign45770_e44256, assign45770_e44256_d_n4,) = {
            if ((!(var_fn487_calc_ig__expbdarg1_vgsat > 50.0)) && (!(var_fn487_calc_ig__expbdarg1_vgsat < assign45770_e44224))) {
                let assign45770_e44229: f64 = (var_fn487_calc_ig__expbdarg1_vgsat).exp();
                (assign45770_e44229, (assign45770_e44229 * var_fn487_calc_ig__expbdarg1_vgsat_dn4),)
            } else {
                let assign45770_e44236: f64 = (-50.0);
                let (assign45770_e44255, assign45770_e44255_d_n4,) = {
                    if ((!(var_fn487_calc_ig__expbdarg1_vgsat > 50.0)) && (var_fn487_calc_ig__expbdarg1_vgsat < assign45770_e44236)) {
                        let assign45770_e44240: f64 = (-50.0);
                        let assign45770_e44241: f64 = (assign45770_e44240).exp();
                        (assign45770_e44241, 0.0,)
                    } else {
                        let (assign45770_e44254, assign45770_e44254_d_n4,) = {
                            if (var_fn487_calc_ig__expbdarg1_vgsat > 50.0) {
                                let assign45770_e44246: f64 = (50.0_f64).exp();
                                let assign45770_e44250: f64 = (var_fn487_calc_ig__expbdarg1_vgsat - 50.0);
                                let assign45770_e44251: f64 = (1.0 + assign45770_e44250);
                                let assign45770_e44252: f64 = (assign45770_e44246 * assign45770_e44251);
                                (assign45770_e44252, (assign45770_e44246 * var_fn487_calc_ig__expbdarg1_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign45770_e44254, assign45770_e44254_d_n4,)
                    }
                };
                (assign45770_e44255, assign45770_e44255_d_n4,)
            }
        };
        (assign45770_e44256, assign45770_e44256_d_n4,)
    } else {
        (var_fn487_calc_ig__expbd1_vgsat, var_fn487_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__expbd1_vgsat = assign45770_e44258;
        var_fn487_calc_ig__expbd1_vgsat_dn4 = assign45770_e44258_d_n4;

        let (assign45780_e44267, assign45780_e44267_d_n4,) = {
    if ((var_guard480 != 0.0) && (var_guard488 == 0.0)) {
        let assign45780_e44265: f64 = (var_fn487_calc_ig__expbd1_vgsat - var_fn487_calc_ig__expbd2);
        (assign45780_e44265, (var_fn487_calc_ig__expbd1_vgsat_dn4 - var_fn487_calc_ig__expbd2_dn4),)
    } else {
        (var_fn487_calc_ig__iginbd_vgsat, var_fn487_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__iginbd_vgsat = assign45780_e44267;
        var_fn487_calc_ig__iginbd_vgsat_dn4 = assign45780_e44267_d_n4;

        let (assign45790_e44280, assign45790_e44280_d_n4,) = {
    if ((var_guard480 != 0.0) && (var_guard488 == 0.0)) {
        let assign45790_e44274: f64 = (var_fn487_calc_ig__pg_paramin / var_fn487_calc_ig__phitin);
        let assign45790_e44276: f64 = (assign45790_e44274 * var_fn487_calc_ig__vgsatin);
        let assign45790_e44278: f64 = (assign45790_e44276 + var_fn487_calc_ig__expphib);
        (assign45790_e44278, (((-((var_fn487_calc_ig__pg_paramin * var_fn487_calc_ig__phitin_dn4) / (var_fn487_calc_ig__phitin * var_fn487_calc_ig__phitin))) * var_fn487_calc_ig__vgsatin) + var_fn487_calc_ig__expphib_dn4),)
    } else {
        (var_fn487_calc_ig__expiforarg_nohinj_vgsat, var_fn487_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__expiforarg_nohinj_vgsat = assign45790_e44280;
        var_fn487_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign45790_e44280_d_n4;

        let (assign45800_e44325, assign45800_e44325_d_n4,) = {
    if ((var_guard480 != 0.0) && (var_guard488 == 0.0)) {
        let assign45800_e44291: f64 = (-50.0);
        let (assign45800_e44323, assign45800_e44323_d_n4,) = {
            if ((!(var_fn487_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (!(var_fn487_calc_ig__expiforarg_nohinj_vgsat < assign45800_e44291))) {
                let assign45800_e44296: f64 = (var_fn487_calc_ig__expiforarg_nohinj_vgsat).exp();
                (assign45800_e44296, (assign45800_e44296 * var_fn487_calc_ig__expiforarg_nohinj_vgsat_dn4),)
            } else {
                let assign45800_e44303: f64 = (-50.0);
                let (assign45800_e44322, assign45800_e44322_d_n4,) = {
                    if ((!(var_fn487_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (var_fn487_calc_ig__expiforarg_nohinj_vgsat < assign45800_e44303)) {
                        let assign45800_e44307: f64 = (-50.0);
                        let assign45800_e44308: f64 = (assign45800_e44307).exp();
                        (assign45800_e44308, 0.0,)
                    } else {
                        let (assign45800_e44321, assign45800_e44321_d_n4,) = {
                            if (var_fn487_calc_ig__expiforarg_nohinj_vgsat > 50.0) {
                                let assign45800_e44313: f64 = (50.0_f64).exp();
                                let assign45800_e44317: f64 = (var_fn487_calc_ig__expiforarg_nohinj_vgsat - 50.0);
                                let assign45800_e44318: f64 = (1.0 + assign45800_e44317);
                                let assign45800_e44319: f64 = (assign45800_e44313 * assign45800_e44318);
                                (assign45800_e44319, (assign45800_e44313 * var_fn487_calc_ig__expiforarg_nohinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign45800_e44321, assign45800_e44321_d_n4,)
                    }
                };
                (assign45800_e44322, assign45800_e44322_d_n4,)
            }
        };
        (assign45800_e44323, assign45800_e44323_d_n4,)
    } else {
        (var_fn487_calc_ig__expifor_nohinj_vgsat, var_fn487_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__expifor_nohinj_vgsat = assign45800_e44325;
        var_fn487_calc_ig__expifor_nohinj_vgsat_dn4 = assign45800_e44325_d_n4;

        let (assign45810_e44338, assign45810_e44338_d_n4,) = {
    if ((var_guard480 != 0.0) && (var_guard488 == 0.0)) {
        let assign45810_e44333: f64 = (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_vgsat);
        let assign45810_e44334: f64 = (var_fn487_calc_ig__expifor_nohinj_vgsat - assign45810_e44333);
        let assign45810_e44336: f64 = (assign45810_e44334 - var_fn487_calc_ig__t0);
        (assign45810_e44336, ((var_fn487_calc_ig__expifor_nohinj_vgsat_dn4 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_vgsat_dn4)) - var_fn487_calc_ig__t0_dn4),)
    } else {
        (var_fn487_calc_ig__igindiode_nohinj_vgsat, var_fn487_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__igindiode_nohinj_vgsat = assign45810_e44338;
        var_fn487_calc_ig__igindiode_nohinj_vgsat_dn4 = assign45810_e44338_d_n4;

        let (assign45820_e44353, assign45820_e44353_d_n0, assign45820_e44353_d_n2, assign45820_e44353_d_n4, assign45820_e44353_d_n8, assign45820_e44353_d_n18, assign45820_e44353_d_n19,) = {
    if ((var_guard480 != 0.0) && (var_guard488 == 0.0)) {
        let assign45820_e44347: f64 = (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd);
        let assign45820_e44348: f64 = (var_fn487_calc_ig__expifor - assign45820_e44347);
        let assign45820_e44350: f64 = (assign45820_e44348 - var_fn487_calc_ig__t0);
        let assign45820_e44351: f64 = (var_fn487_calc_ig__isdiodeout * assign45820_e44350);
        (assign45820_e44351, (var_fn487_calc_ig__isdiodeout * (var_fn487_calc_ig__expifor_dn0 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn0))), (var_fn487_calc_ig__isdiodeout * (var_fn487_calc_ig__expifor_dn2 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn2))), ((var_fn487_calc_ig__isdiodeout_dn4 * assign45820_e44350) + (var_fn487_calc_ig__isdiodeout * ((var_fn487_calc_ig__expifor_dn4 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn4)) - var_fn487_calc_ig__t0_dn4))), (var_fn487_calc_ig__isdiodeout * (var_fn487_calc_ig__expifor_dn8 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn8))), (var_fn487_calc_ig__isdiodeout * (var_fn487_calc_ig__expifor_dn18 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn18))), (var_fn487_calc_ig__isdiodeout * (var_fn487_calc_ig__expifor_dn19 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn19))),)
    } else {
        (var_fn487_calc_ig__igindiode_nohinj, var_fn487_calc_ig__igindiode_nohinj_dn0, var_fn487_calc_ig__igindiode_nohinj_dn2, var_fn487_calc_ig__igindiode_nohinj_dn4, var_fn487_calc_ig__igindiode_nohinj_dn8, var_fn487_calc_ig__igindiode_nohinj_dn18, var_fn487_calc_ig__igindiode_nohinj_dn19,)
    }
};
        var_fn487_calc_ig__igindiode_nohinj = assign45820_e44353;
        var_fn487_calc_ig__igindiode_nohinj_dn0 = assign45820_e44353_d_n0;
        var_fn487_calc_ig__igindiode_nohinj_dn2 = assign45820_e44353_d_n2;
        var_fn487_calc_ig__igindiode_nohinj_dn4 = assign45820_e44353_d_n4;
        var_fn487_calc_ig__igindiode_nohinj_dn8 = assign45820_e44353_d_n8;
        var_fn487_calc_ig__igindiode_nohinj_dn18 = assign45820_e44353_d_n18;
        var_fn487_calc_ig__igindiode_nohinj_dn19 = assign45820_e44353_d_n19;

        let assign45830_e44356: f64 = if var_fn487_calc_ig__fracin > 0.0 { 1.0 } else { 0.0 };
        var_guard489 = assign45830_e44356;

        let (assign45840_e44367,) = {
    if (((var_guard480 != 0.0) && (var_guard488 == 0.0)) && (var_guard489 != 0.0)) {
        let assign45840_e44365: f64 = (var_fn487_calc_ig__fracin * var_fn487_calc_ig__pg_paramin);
        (assign45840_e44365,)
    } else {
        (var_fn487_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn487_calc_ig__pg_paramin_hinj = assign45840_e44367;

        let (assign45850_e44382, assign45850_e44382_d_n4,) = {
    if (((var_guard480 != 0.0) && (var_guard488 == 0.0)) && (var_guard489 != 0.0)) {
        let assign45850_e44376: f64 = (var_fn487_calc_ig__pg_paramin_hinj / var_fn487_calc_ig__phitin);
        let assign45850_e44378: f64 = (assign45850_e44376 * var_fn487_calc_ig__vgsatin);
        let assign45850_e44380: f64 = (assign45850_e44378 + var_fn487_calc_ig__expphib);
        (assign45850_e44380, (((-((var_fn487_calc_ig__pg_paramin_hinj * var_fn487_calc_ig__phitin_dn4) / (var_fn487_calc_ig__phitin * var_fn487_calc_ig__phitin))) * var_fn487_calc_ig__vgsatin) + var_fn487_calc_ig__expphib_dn4),)
    } else {
        (var_fn487_calc_ig__expiforarg_hinj_vgsat, var_fn487_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__expiforarg_hinj_vgsat = assign45850_e44382;
        var_fn487_calc_ig__expiforarg_hinj_vgsat_dn4 = assign45850_e44382_d_n4;

        let (assign45860_e44429, assign45860_e44429_d_n4,) = {
    if (((var_guard480 != 0.0) && (var_guard488 == 0.0)) && (var_guard489 != 0.0)) {
        let assign45860_e44395: f64 = (-50.0);
        let (assign45860_e44427, assign45860_e44427_d_n4,) = {
            if ((!(var_fn487_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (!(var_fn487_calc_ig__expiforarg_hinj_vgsat < assign45860_e44395))) {
                let assign45860_e44400: f64 = (var_fn487_calc_ig__expiforarg_hinj_vgsat).exp();
                (assign45860_e44400, (assign45860_e44400 * var_fn487_calc_ig__expiforarg_hinj_vgsat_dn4),)
            } else {
                let assign45860_e44407: f64 = (-50.0);
                let (assign45860_e44426, assign45860_e44426_d_n4,) = {
                    if ((!(var_fn487_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (var_fn487_calc_ig__expiforarg_hinj_vgsat < assign45860_e44407)) {
                        let assign45860_e44411: f64 = (-50.0);
                        let assign45860_e44412: f64 = (assign45860_e44411).exp();
                        (assign45860_e44412, 0.0,)
                    } else {
                        let (assign45860_e44425, assign45860_e44425_d_n4,) = {
                            if (var_fn487_calc_ig__expiforarg_hinj_vgsat > 50.0) {
                                let assign45860_e44417: f64 = (50.0_f64).exp();
                                let assign45860_e44421: f64 = (var_fn487_calc_ig__expiforarg_hinj_vgsat - 50.0);
                                let assign45860_e44422: f64 = (1.0 + assign45860_e44421);
                                let assign45860_e44423: f64 = (assign45860_e44417 * assign45860_e44422);
                                (assign45860_e44423, (assign45860_e44417 * var_fn487_calc_ig__expiforarg_hinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign45860_e44425, assign45860_e44425_d_n4,)
                    }
                };
                (assign45860_e44426, assign45860_e44426_d_n4,)
            }
        };
        (assign45860_e44427, assign45860_e44427_d_n4,)
    } else {
        (var_fn487_calc_ig__expifor_hinj_vgsat, var_fn487_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__expifor_hinj_vgsat = assign45860_e44429;
        var_fn487_calc_ig__expifor_hinj_vgsat_dn4 = assign45860_e44429_d_n4;

        let (assign45870_e44444, assign45870_e44444_d_n4,) = {
    if (((var_guard480 != 0.0) && (var_guard488 == 0.0)) && (var_guard489 != 0.0)) {
        let assign45870_e44439: f64 = (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_vgsat);
        let assign45870_e44440: f64 = (var_fn487_calc_ig__expifor_hinj_vgsat - assign45870_e44439);
        let assign45870_e44442: f64 = (assign45870_e44440 - var_fn487_calc_ig__t0);
        (assign45870_e44442, ((var_fn487_calc_ig__expifor_hinj_vgsat_dn4 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_vgsat_dn4)) - var_fn487_calc_ig__t0_dn4),)
    } else {
        (var_fn487_calc_ig__igindiode_hinj_vgsat, var_fn487_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn487_calc_ig__igindiode_hinj_vgsat = assign45870_e44444;
        var_fn487_calc_ig__igindiode_hinj_vgsat_dn4 = assign45870_e44444_d_n4;

        let (assign45880_e44459, assign45880_e44459_d_n0, assign45880_e44459_d_n2, assign45880_e44459_d_n4, assign45880_e44459_d_n8, assign45880_e44459_d_n18, assign45880_e44459_d_n19,) = {
    if (((var_guard480 != 0.0) && (var_guard488 == 0.0)) && (var_guard489 != 0.0)) {
        let assign45880_e44453: f64 = (var_fn487_calc_ig__pg_paramin_hinj / var_fn487_calc_ig__phitin);
        let assign45880_e44455: f64 = (assign45880_e44453 * var_fn487_calc_ig__vgin);
        let assign45880_e44457: f64 = (assign45880_e44455 + var_fn487_calc_ig__expphib);
        (assign45880_e44457, (assign45880_e44453 * var_fn487_calc_ig__vgin_dn0), (assign45880_e44453 * var_fn487_calc_ig__vgin_dn2), (((-((var_fn487_calc_ig__pg_paramin_hinj * var_fn487_calc_ig__phitin_dn4) / (var_fn487_calc_ig__phitin * var_fn487_calc_ig__phitin))) * var_fn487_calc_ig__vgin) + var_fn487_calc_ig__expphib_dn4), (assign45880_e44453 * var_fn487_calc_ig__vgin_dn8), (assign45880_e44453 * var_fn487_calc_ig__vgin_dn18), (assign45880_e44453 * var_fn487_calc_ig__vgin_dn19),)
    } else {
        (var_fn487_calc_ig__expiforarg_hinj, var_fn487_calc_ig__expiforarg_hinj_dn0, var_fn487_calc_ig__expiforarg_hinj_dn2, var_fn487_calc_ig__expiforarg_hinj_dn4, var_fn487_calc_ig__expiforarg_hinj_dn8, var_fn487_calc_ig__expiforarg_hinj_dn18, var_fn487_calc_ig__expiforarg_hinj_dn19,)
    }
};
        var_fn487_calc_ig__expiforarg_hinj = assign45880_e44459;
        var_fn487_calc_ig__expiforarg_hinj_dn0 = assign45880_e44459_d_n0;
        var_fn487_calc_ig__expiforarg_hinj_dn2 = assign45880_e44459_d_n2;
        var_fn487_calc_ig__expiforarg_hinj_dn4 = assign45880_e44459_d_n4;
        var_fn487_calc_ig__expiforarg_hinj_dn8 = assign45880_e44459_d_n8;
        var_fn487_calc_ig__expiforarg_hinj_dn18 = assign45880_e44459_d_n18;
        var_fn487_calc_ig__expiforarg_hinj_dn19 = assign45880_e44459_d_n19;

        let (assign45890_e44506, assign45890_e44506_d_n0, assign45890_e44506_d_n2, assign45890_e44506_d_n4, assign45890_e44506_d_n8, assign45890_e44506_d_n18, assign45890_e44506_d_n19,) = {
    if (((var_guard480 != 0.0) && (var_guard488 == 0.0)) && (var_guard489 != 0.0)) {
        let assign45890_e44472: f64 = (-50.0);
        let (assign45890_e44504, assign45890_e44504_d_n0, assign45890_e44504_d_n2, assign45890_e44504_d_n4, assign45890_e44504_d_n8, assign45890_e44504_d_n18, assign45890_e44504_d_n19,) = {
            if ((!(var_fn487_calc_ig__expiforarg_hinj > 50.0)) && (!(var_fn487_calc_ig__expiforarg_hinj < assign45890_e44472))) {
                let assign45890_e44477: f64 = (var_fn487_calc_ig__expiforarg_hinj).exp();
                (assign45890_e44477, (assign45890_e44477 * var_fn487_calc_ig__expiforarg_hinj_dn0), (assign45890_e44477 * var_fn487_calc_ig__expiforarg_hinj_dn2), (assign45890_e44477 * var_fn487_calc_ig__expiforarg_hinj_dn4), (assign45890_e44477 * var_fn487_calc_ig__expiforarg_hinj_dn8), (assign45890_e44477 * var_fn487_calc_ig__expiforarg_hinj_dn18), (assign45890_e44477 * var_fn487_calc_ig__expiforarg_hinj_dn19),)
            } else {
                let assign45890_e44484: f64 = (-50.0);
                let (assign45890_e44503, assign45890_e44503_d_n0, assign45890_e44503_d_n2, assign45890_e44503_d_n4, assign45890_e44503_d_n8, assign45890_e44503_d_n18, assign45890_e44503_d_n19,) = {
                    if ((!(var_fn487_calc_ig__expiforarg_hinj > 50.0)) && (var_fn487_calc_ig__expiforarg_hinj < assign45890_e44484)) {
                        let assign45890_e44488: f64 = (-50.0);
                        let assign45890_e44489: f64 = (assign45890_e44488).exp();
                        (assign45890_e44489, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign45890_e44502, assign45890_e44502_d_n0, assign45890_e44502_d_n2, assign45890_e44502_d_n4, assign45890_e44502_d_n8, assign45890_e44502_d_n18, assign45890_e44502_d_n19,) = {
                            if (var_fn487_calc_ig__expiforarg_hinj > 50.0) {
                                let assign45890_e44494: f64 = (50.0_f64).exp();
                                let assign45890_e44498: f64 = (var_fn487_calc_ig__expiforarg_hinj - 50.0);
                                let assign45890_e44499: f64 = (1.0 + assign45890_e44498);
                                let assign45890_e44500: f64 = (assign45890_e44494 * assign45890_e44499);
                                (assign45890_e44500, (assign45890_e44494 * var_fn487_calc_ig__expiforarg_hinj_dn0), (assign45890_e44494 * var_fn487_calc_ig__expiforarg_hinj_dn2), (assign45890_e44494 * var_fn487_calc_ig__expiforarg_hinj_dn4), (assign45890_e44494 * var_fn487_calc_ig__expiforarg_hinj_dn8), (assign45890_e44494 * var_fn487_calc_ig__expiforarg_hinj_dn18), (assign45890_e44494 * var_fn487_calc_ig__expiforarg_hinj_dn19),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign45890_e44502, assign45890_e44502_d_n0, assign45890_e44502_d_n2, assign45890_e44502_d_n4, assign45890_e44502_d_n8, assign45890_e44502_d_n18, assign45890_e44502_d_n19,)
                    }
                };
                (assign45890_e44503, assign45890_e44503_d_n0, assign45890_e44503_d_n2, assign45890_e44503_d_n4, assign45890_e44503_d_n8, assign45890_e44503_d_n18, assign45890_e44503_d_n19,)
            }
        };
        (assign45890_e44504, assign45890_e44504_d_n0, assign45890_e44504_d_n2, assign45890_e44504_d_n4, assign45890_e44504_d_n8, assign45890_e44504_d_n18, assign45890_e44504_d_n19,)
    } else {
        (var_fn487_calc_ig__expifor_hinj, var_fn487_calc_ig__expifor_hinj_dn0, var_fn487_calc_ig__expifor_hinj_dn2, var_fn487_calc_ig__expifor_hinj_dn4, var_fn487_calc_ig__expifor_hinj_dn8, var_fn487_calc_ig__expifor_hinj_dn18, var_fn487_calc_ig__expifor_hinj_dn19,)
    }
};
        var_fn487_calc_ig__expifor_hinj = assign45890_e44506;
        var_fn487_calc_ig__expifor_hinj_dn0 = assign45890_e44506_d_n0;
        var_fn487_calc_ig__expifor_hinj_dn2 = assign45890_e44506_d_n2;
        var_fn487_calc_ig__expifor_hinj_dn4 = assign45890_e44506_d_n4;
        var_fn487_calc_ig__expifor_hinj_dn8 = assign45890_e44506_d_n8;
        var_fn487_calc_ig__expifor_hinj_dn18 = assign45890_e44506_d_n18;
        var_fn487_calc_ig__expifor_hinj_dn19 = assign45890_e44506_d_n19;

        let (assign45900_e44519, assign45900_e44519_d_n4,) = {
    if (((var_guard480 != 0.0) && (var_guard488 == 0.0)) && (var_guard489 != 0.0)) {
        let assign45900_e44515: f64 = (var_fn487_calc_ig__isdiodeout * var_fn487_calc_ig__igindiode_nohinj_vgsat);
        let assign45900_e44517: f64 = (assign45900_e44515 / var_fn487_calc_ig__igindiode_hinj_vgsat);
        (assign45900_e44517, (((((var_fn487_calc_ig__isdiodeout_dn4 * var_fn487_calc_ig__igindiode_nohinj_vgsat) + (var_fn487_calc_ig__isdiodeout * var_fn487_calc_ig__igindiode_nohinj_vgsat_dn4)) * var_fn487_calc_ig__igindiode_hinj_vgsat) - (assign45900_e44515 * var_fn487_calc_ig__igindiode_hinj_vgsat_dn4)) / (var_fn487_calc_ig__igindiode_hinj_vgsat * var_fn487_calc_ig__igindiode_hinj_vgsat)),)
    } else {
        (var_fn487_calc_ig__igindiode_hinj_pre, var_fn487_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn487_calc_ig__igindiode_hinj_pre = assign45900_e44519;
        var_fn487_calc_ig__igindiode_hinj_pre_dn4 = assign45900_e44519_d_n4;

        let (assign45910_e44536, assign45910_e44536_d_n0, assign45910_e44536_d_n2, assign45910_e44536_d_n4, assign45910_e44536_d_n8, assign45910_e44536_d_n18, assign45910_e44536_d_n19,) = {
    if (((var_guard480 != 0.0) && (var_guard488 == 0.0)) && (var_guard489 != 0.0)) {
        let assign45910_e44530: f64 = (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd);
        let assign45910_e44531: f64 = (var_fn487_calc_ig__expifor_hinj - assign45910_e44530);
        let assign45910_e44533: f64 = (assign45910_e44531 - var_fn487_calc_ig__t0);
        let assign45910_e44534: f64 = (var_fn487_calc_ig__igindiode_hinj_pre * assign45910_e44533);
        (assign45910_e44534, (var_fn487_calc_ig__igindiode_hinj_pre * (var_fn487_calc_ig__expifor_hinj_dn0 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn0))), (var_fn487_calc_ig__igindiode_hinj_pre * (var_fn487_calc_ig__expifor_hinj_dn2 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn2))), ((var_fn487_calc_ig__igindiode_hinj_pre_dn4 * assign45910_e44533) + (var_fn487_calc_ig__igindiode_hinj_pre * ((var_fn487_calc_ig__expifor_hinj_dn4 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn4)) - var_fn487_calc_ig__t0_dn4))), (var_fn487_calc_ig__igindiode_hinj_pre * (var_fn487_calc_ig__expifor_hinj_dn8 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn8))), (var_fn487_calc_ig__igindiode_hinj_pre * (var_fn487_calc_ig__expifor_hinj_dn18 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn18))), (var_fn487_calc_ig__igindiode_hinj_pre * (var_fn487_calc_ig__expifor_hinj_dn19 - (var_fn487_calc_ig__kbdgatein * var_fn487_calc_ig__iginbd_dn19))),)
    } else {
        (var_fn487_calc_ig__igindiode_hinj, var_fn487_calc_ig__igindiode_hinj_dn0, var_fn487_calc_ig__igindiode_hinj_dn2, var_fn487_calc_ig__igindiode_hinj_dn4, var_fn487_calc_ig__igindiode_hinj_dn8, var_fn487_calc_ig__igindiode_hinj_dn18, var_fn487_calc_ig__igindiode_hinj_dn19,)
    }
};
        var_fn487_calc_ig__igindiode_hinj = assign45910_e44536;
        var_fn487_calc_ig__igindiode_hinj_dn0 = assign45910_e44536_d_n0;
        var_fn487_calc_ig__igindiode_hinj_dn2 = assign45910_e44536_d_n2;
        var_fn487_calc_ig__igindiode_hinj_dn4 = assign45910_e44536_d_n4;
        var_fn487_calc_ig__igindiode_hinj_dn8 = assign45910_e44536_d_n8;
        var_fn487_calc_ig__igindiode_hinj_dn18 = assign45910_e44536_d_n18;
        var_fn487_calc_ig__igindiode_hinj_dn19 = assign45910_e44536_d_n19;

        let (assign45920_e44548, assign45920_e44548_d_n0, assign45920_e44548_d_n2, assign45920_e44548_d_n4, assign45920_e44548_d_n8, assign45920_e44548_d_n18, assign45920_e44548_d_n19,) = {
    if (((var_guard480 != 0.0) && (var_guard488 == 0.0)) && (var_guard489 == 0.0)) {
        let assign45920_e44546: f64 = (var_fn487_calc_ig__isdiodeout * var_fn487_calc_ig__igindiode_nohinj_vgsat);
        (assign45920_e44546, 0.0, 0.0, ((var_fn487_calc_ig__isdiodeout_dn4 * var_fn487_calc_ig__igindiode_nohinj_vgsat) + (var_fn487_calc_ig__isdiodeout * var_fn487_calc_ig__igindiode_nohinj_vgsat_dn4)), 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__igindiode_hinj, var_fn487_calc_ig__igindiode_hinj_dn0, var_fn487_calc_ig__igindiode_hinj_dn2, var_fn487_calc_ig__igindiode_hinj_dn4, var_fn487_calc_ig__igindiode_hinj_dn8, var_fn487_calc_ig__igindiode_hinj_dn18, var_fn487_calc_ig__igindiode_hinj_dn19,)
    }
};
        var_fn487_calc_ig__igindiode_hinj = assign45920_e44548;
        var_fn487_calc_ig__igindiode_hinj_dn0 = assign45920_e44548_d_n0;
        var_fn487_calc_ig__igindiode_hinj_dn2 = assign45920_e44548_d_n2;
        var_fn487_calc_ig__igindiode_hinj_dn4 = assign45920_e44548_d_n4;
        var_fn487_calc_ig__igindiode_hinj_dn8 = assign45920_e44548_d_n8;
        var_fn487_calc_ig__igindiode_hinj_dn18 = assign45920_e44548_d_n18;
        var_fn487_calc_ig__igindiode_hinj_dn19 = assign45920_e44548_d_n19;

        let (assign45930_e44559, assign45930_e44559_d_n4,) = {
    if ((var_guard480 != 0.0) && (var_guard488 == 0.0)) {
        let assign45930_e44555: f64 = (var_fn487_calc_ig__alphagin * var_fn487_calc_ig__alphagin);
        let assign45930_e44557: f64 = (assign45930_e44555 * var_fn487_calc_ig__phitin);
        (assign45930_e44557, (assign45930_e44555 * var_fn487_calc_ig__phitin_dn4),)
    } else {
        (var_fn487_calc_ig__alpha2_phit, var_fn487_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn487_calc_ig__alpha2_phit = assign45930_e44559;
        var_fn487_calc_ig__alpha2_phit_dn4 = assign45930_e44559_d_n4;

        let (assign45940_e44574, assign45940_e44574_d_n0, assign45940_e44574_d_n2, assign45940_e44574_d_n4, assign45940_e44574_d_n8, assign45940_e44574_d_n18, assign45940_e44574_d_n19,) = {
    if ((var_guard480 != 0.0) && (var_guard488 == 0.0)) {
        let assign45940_e44568: f64 = (var_fn487_calc_ig__alpha2_phit / 2.0);
        let assign45940_e44569: f64 = (var_fn487_calc_ig__vgsatin - assign45940_e44568);
        let assign45940_e44570: f64 = (var_fn487_calc_ig__vgin - assign45940_e44569);
        let assign45940_e44572: f64 = (assign45940_e44570 / var_fn487_calc_ig__alpha2_phit);
        (assign45940_e44572, (var_fn487_calc_ig__vgin_dn0 / var_fn487_calc_ig__alpha2_phit), (var_fn487_calc_ig__vgin_dn2 / var_fn487_calc_ig__alpha2_phit), ((((-(-(var_fn487_calc_ig__alpha2_phit_dn4 / 2.0))) * var_fn487_calc_ig__alpha2_phit) - (assign45940_e44570 * var_fn487_calc_ig__alpha2_phit_dn4)) / (var_fn487_calc_ig__alpha2_phit * var_fn487_calc_ig__alpha2_phit)), (var_fn487_calc_ig__vgin_dn8 / var_fn487_calc_ig__alpha2_phit), (var_fn487_calc_ig__vgin_dn18 / var_fn487_calc_ig__alpha2_phit), (var_fn487_calc_ig__vgin_dn19 / var_fn487_calc_ig__alpha2_phit),)
    } else {
        (var_fn487_calc_ig__expffvarg, var_fn487_calc_ig__expffvarg_dn0, var_fn487_calc_ig__expffvarg_dn2, var_fn487_calc_ig__expffvarg_dn4, var_fn487_calc_ig__expffvarg_dn8, var_fn487_calc_ig__expffvarg_dn18, var_fn487_calc_ig__expffvarg_dn19,)
    }
};
        var_fn487_calc_ig__expffvarg = assign45940_e44574;
        var_fn487_calc_ig__expffvarg_dn0 = assign45940_e44574_d_n0;
        var_fn487_calc_ig__expffvarg_dn2 = assign45940_e44574_d_n2;
        var_fn487_calc_ig__expffvarg_dn4 = assign45940_e44574_d_n4;
        var_fn487_calc_ig__expffvarg_dn8 = assign45940_e44574_d_n8;
        var_fn487_calc_ig__expffvarg_dn18 = assign45940_e44574_d_n18;
        var_fn487_calc_ig__expffvarg_dn19 = assign45940_e44574_d_n19;

        let assign45950_e44577: f64 = if var_fn487_calc_ig__expffvarg > 50.0 { 1.0 } else { 0.0 };
        var_guard490 = assign45950_e44577;

        let (assign45960_e44586, assign45960_e44586_d_n0, assign45960_e44586_d_n2, assign45960_e44586_d_n4, assign45960_e44586_d_n8, assign45960_e44586_d_n18, assign45960_e44586_d_n19,) = {
    if (((var_guard480 != 0.0) && (var_guard488 == 0.0)) && (var_guard490 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__ffvgin, var_fn487_calc_ig__ffvgin_dn0, var_fn487_calc_ig__ffvgin_dn2, var_fn487_calc_ig__ffvgin_dn4, var_fn487_calc_ig__ffvgin_dn8, var_fn487_calc_ig__ffvgin_dn18, var_fn487_calc_ig__ffvgin_dn19,)
    }
};
        var_fn487_calc_ig__ffvgin = assign45960_e44586;
        var_fn487_calc_ig__ffvgin_dn0 = assign45960_e44586_d_n0;
        var_fn487_calc_ig__ffvgin_dn2 = assign45960_e44586_d_n2;
        var_fn487_calc_ig__ffvgin_dn4 = assign45960_e44586_d_n4;
        var_fn487_calc_ig__ffvgin_dn8 = assign45960_e44586_d_n8;
        var_fn487_calc_ig__ffvgin_dn18 = assign45960_e44586_d_n18;
        var_fn487_calc_ig__ffvgin_dn19 = assign45960_e44586_d_n19;

        let assign45970_e44589: f64 = (-50.0);
        let assign45970_e44590: f64 = if var_fn487_calc_ig__expffvarg < assign45970_e44589 { 1.0 } else { 0.0 };
        var_guard491 = assign45970_e44590;

        let (assign45980_e44602, assign45980_e44602_d_n0, assign45980_e44602_d_n2, assign45980_e44602_d_n4, assign45980_e44602_d_n8, assign45980_e44602_d_n18, assign45980_e44602_d_n19,) = {
    if ((((var_guard480 != 0.0) && (var_guard488 == 0.0)) && (var_guard490 == 0.0)) && (var_guard491 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn487_calc_ig__ffvgin, var_fn487_calc_ig__ffvgin_dn0, var_fn487_calc_ig__ffvgin_dn2, var_fn487_calc_ig__ffvgin_dn4, var_fn487_calc_ig__ffvgin_dn8, var_fn487_calc_ig__ffvgin_dn18, var_fn487_calc_ig__ffvgin_dn19,)
    }
};
        var_fn487_calc_ig__ffvgin = assign45980_e44602;
        var_fn487_calc_ig__ffvgin_dn0 = assign45980_e44602_d_n0;
        var_fn487_calc_ig__ffvgin_dn2 = assign45980_e44602_d_n2;
        var_fn487_calc_ig__ffvgin_dn4 = assign45980_e44602_d_n4;
        var_fn487_calc_ig__ffvgin_dn8 = assign45980_e44602_d_n8;
        var_fn487_calc_ig__ffvgin_dn18 = assign45980_e44602_d_n18;
        var_fn487_calc_ig__ffvgin_dn19 = assign45980_e44602_d_n19;

        let (assign45990_e44620, assign45990_e44620_d_n0, assign45990_e44620_d_n2, assign45990_e44620_d_n4, assign45990_e44620_d_n8, assign45990_e44620_d_n18, assign45990_e44620_d_n19,) = {
    if ((((var_guard480 != 0.0) && (var_guard488 == 0.0)) && (var_guard490 == 0.0)) && (var_guard491 == 0.0)) {
        let assign45990_e44616: f64 = (var_fn487_calc_ig__expffvarg).exp();
        let assign45990_e44617: f64 = (1.0 + assign45990_e44616);
        let assign45990_e44618: f64 = (1.0 / assign45990_e44617);
        (assign45990_e44618, (-((assign45990_e44616 * var_fn487_calc_ig__expffvarg_dn0) / (assign45990_e44617 * assign45990_e44617))), (-((assign45990_e44616 * var_fn487_calc_ig__expffvarg_dn2) / (assign45990_e44617 * assign45990_e44617))), (-((assign45990_e44616 * var_fn487_calc_ig__expffvarg_dn4) / (assign45990_e44617 * assign45990_e44617))), (-((assign45990_e44616 * var_fn487_calc_ig__expffvarg_dn8) / (assign45990_e44617 * assign45990_e44617))), (-((assign45990_e44616 * var_fn487_calc_ig__expffvarg_dn18) / (assign45990_e44617 * assign45990_e44617))), (-((assign45990_e44616 * var_fn487_calc_ig__expffvarg_dn19) / (assign45990_e44617 * assign45990_e44617))),)
    } else {
        (var_fn487_calc_ig__ffvgin, var_fn487_calc_ig__ffvgin_dn0, var_fn487_calc_ig__ffvgin_dn2, var_fn487_calc_ig__ffvgin_dn4, var_fn487_calc_ig__ffvgin_dn8, var_fn487_calc_ig__ffvgin_dn18, var_fn487_calc_ig__ffvgin_dn19,)
    }
};
        var_fn487_calc_ig__ffvgin = assign45990_e44620;
        var_fn487_calc_ig__ffvgin_dn0 = assign45990_e44620_d_n0;
        var_fn487_calc_ig__ffvgin_dn2 = assign45990_e44620_d_n2;
        var_fn487_calc_ig__ffvgin_dn4 = assign45990_e44620_d_n4;
        var_fn487_calc_ig__ffvgin_dn8 = assign45990_e44620_d_n8;
        var_fn487_calc_ig__ffvgin_dn18 = assign45990_e44620_d_n18;
        var_fn487_calc_ig__ffvgin_dn19 = assign45990_e44620_d_n19;

        let (assign46000_e44635, assign46000_e44635_d_n0, assign46000_e44635_d_n2, assign46000_e44635_d_n4, assign46000_e44635_d_n8, assign46000_e44635_d_n18, assign46000_e44635_d_n19,) = {
    if ((var_guard480 != 0.0) && (var_guard488 == 0.0)) {
        let assign46000_e44627: f64 = (var_fn487_calc_ig__ffvgin * var_fn487_calc_ig__igindiode_nohinj);
        let assign46000_e44630: f64 = (1.0 - var_fn487_calc_ig__ffvgin);
        let assign46000_e44632: f64 = (assign46000_e44630 * var_fn487_calc_ig__igindiode_hinj);
        let assign46000_e44633: f64 = (assign46000_e44627 + assign46000_e44632);
        (assign46000_e44633, (((var_fn487_calc_ig__ffvgin_dn0 * var_fn487_calc_ig__igindiode_nohinj) + (var_fn487_calc_ig__ffvgin * var_fn487_calc_ig__igindiode_nohinj_dn0)) + (((-var_fn487_calc_ig__ffvgin_dn0) * var_fn487_calc_ig__igindiode_hinj) + (assign46000_e44630 * var_fn487_calc_ig__igindiode_hinj_dn0))), (((var_fn487_calc_ig__ffvgin_dn2 * var_fn487_calc_ig__igindiode_nohinj) + (var_fn487_calc_ig__ffvgin * var_fn487_calc_ig__igindiode_nohinj_dn2)) + (((-var_fn487_calc_ig__ffvgin_dn2) * var_fn487_calc_ig__igindiode_hinj) + (assign46000_e44630 * var_fn487_calc_ig__igindiode_hinj_dn2))), (((var_fn487_calc_ig__ffvgin_dn4 * var_fn487_calc_ig__igindiode_nohinj) + (var_fn487_calc_ig__ffvgin * var_fn487_calc_ig__igindiode_nohinj_dn4)) + (((-var_fn487_calc_ig__ffvgin_dn4) * var_fn487_calc_ig__igindiode_hinj) + (assign46000_e44630 * var_fn487_calc_ig__igindiode_hinj_dn4))), (((var_fn487_calc_ig__ffvgin_dn8 * var_fn487_calc_ig__igindiode_nohinj) + (var_fn487_calc_ig__ffvgin * var_fn487_calc_ig__igindiode_nohinj_dn8)) + (((-var_fn487_calc_ig__ffvgin_dn8) * var_fn487_calc_ig__igindiode_hinj) + (assign46000_e44630 * var_fn487_calc_ig__igindiode_hinj_dn8))), (((var_fn487_calc_ig__ffvgin_dn18 * var_fn487_calc_ig__igindiode_nohinj) + (var_fn487_calc_ig__ffvgin * var_fn487_calc_ig__igindiode_nohinj_dn18)) + (((-var_fn487_calc_ig__ffvgin_dn18) * var_fn487_calc_ig__igindiode_hinj) + (assign46000_e44630 * var_fn487_calc_ig__igindiode_hinj_dn18))), (((var_fn487_calc_ig__ffvgin_dn19 * var_fn487_calc_ig__igindiode_nohinj) + (var_fn487_calc_ig__ffvgin * var_fn487_calc_ig__igindiode_nohinj_dn19)) + (((-var_fn487_calc_ig__ffvgin_dn19) * var_fn487_calc_ig__igindiode_hinj) + (assign46000_e44630 * var_fn487_calc_ig__igindiode_hinj_dn19))),)
    } else {
        (var_fn487_calc_ig__igindiode, var_fn487_calc_ig__igindiode_dn0, var_fn487_calc_ig__igindiode_dn2, var_fn487_calc_ig__igindiode_dn4, var_fn487_calc_ig__igindiode_dn8, var_fn487_calc_ig__igindiode_dn18, var_fn487_calc_ig__igindiode_dn19,)
    }
};
        var_fn487_calc_ig__igindiode = assign46000_e44635;
        var_fn487_calc_ig__igindiode_dn0 = assign46000_e44635_d_n0;
        var_fn487_calc_ig__igindiode_dn2 = assign46000_e44635_d_n2;
        var_fn487_calc_ig__igindiode_dn4 = assign46000_e44635_d_n4;
        var_fn487_calc_ig__igindiode_dn8 = assign46000_e44635_d_n8;
        var_fn487_calc_ig__igindiode_dn18 = assign46000_e44635_d_n18;
        var_fn487_calc_ig__igindiode_dn19 = assign46000_e44635_d_n19;

        let (assign46010_e44681, assign46010_e44681_d_n0, assign46010_e44681_d_n2, assign46010_e44681_d_n8, assign46010_e44681_d_n18, assign46010_e44681_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign46010_e44638: f64 = (-var_fn487_calc_ig__vgin);
        let (assign46010_e44671, assign46010_e44671_d_n0, assign46010_e44671_d_n2, assign46010_e44671_d_n8, assign46010_e44671_d_n18, assign46010_e44671_d_n19,) = {
            if (p.p52 != 0.0) {
                let assign46010_e44646: f64 = (var_fn487_calc_ig__vgin / var_fn487_calc_ig__vgsatqin);
                let assign46010_e44649: f64 = (0.001 / p.p53);
                let assign46010_e44652: f64 = (var_fn487_calc_ig__vgin / var_fn487_calc_ig__vgsatqin);
                let assign46010_e44653: f64 = (assign46010_e44649 * assign46010_e44652);
                let assign46010_e44654: f64 = (assign46010_e44653).tanh();
                let assign46010_e44655: f64 = (assign46010_e44646 * assign46010_e44654);
                (assign46010_e44655, (((var_fn487_calc_ig__vgin_dn0 / var_fn487_calc_ig__vgsatqin) * assign46010_e44654) + (assign46010_e44646 * ((assign46010_e44649 * (var_fn487_calc_ig__vgin_dn0 / var_fn487_calc_ig__vgsatqin)) / ((assign46010_e44653).cosh() * (assign46010_e44653).cosh())))), (((var_fn487_calc_ig__vgin_dn2 / var_fn487_calc_ig__vgsatqin) * assign46010_e44654) + (assign46010_e44646 * ((assign46010_e44649 * (var_fn487_calc_ig__vgin_dn2 / var_fn487_calc_ig__vgsatqin)) / ((assign46010_e44653).cosh() * (assign46010_e44653).cosh())))), (((var_fn487_calc_ig__vgin_dn8 / var_fn487_calc_ig__vgsatqin) * assign46010_e44654) + (assign46010_e44646 * ((assign46010_e44649 * (var_fn487_calc_ig__vgin_dn8 / var_fn487_calc_ig__vgsatqin)) / ((assign46010_e44653).cosh() * (assign46010_e44653).cosh())))), (((var_fn487_calc_ig__vgin_dn18 / var_fn487_calc_ig__vgsatqin) * assign46010_e44654) + (assign46010_e44646 * ((assign46010_e44649 * (var_fn487_calc_ig__vgin_dn18 / var_fn487_calc_ig__vgsatqin)) / ((assign46010_e44653).cosh() * (assign46010_e44653).cosh())))), (((var_fn487_calc_ig__vgin_dn19 / var_fn487_calc_ig__vgsatqin) * assign46010_e44654) + (assign46010_e44646 * ((assign46010_e44649 * (var_fn487_calc_ig__vgin_dn19 / var_fn487_calc_ig__vgsatqin)) / ((assign46010_e44653).cosh() * (assign46010_e44653).cosh())))),)
            } else {
                let (assign46010_e44670, assign46010_e44670_d_n0, assign46010_e44670_d_n2, assign46010_e44670_d_n8, assign46010_e44670_d_n18, assign46010_e44670_d_n19,) = {
                    if (p.p52 == 0.0) {
                        let __rspice_inv_cse_0: f64 = 1.0 / var_fn487_calc_ig__vgsatqin;
                        let assign46010_e44661: f64 = (var_fn487_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign46010_e44664: f64 = (var_fn487_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign46010_e44665: f64 = (assign46010_e44661 * assign46010_e44664);
                        let assign46010_e44667: f64 = (assign46010_e44665 + p.p53);
                        let assign46010_e44668: f64 = (assign46010_e44667).sqrt();
                        (assign46010_e44668, ((((var_fn487_calc_ig__vgin_dn0 / var_fn487_calc_ig__vgsatqin) * assign46010_e44664) + (assign46010_e44661 * (var_fn487_calc_ig__vgin_dn0 / var_fn487_calc_ig__vgsatqin))) / (2.0 * assign46010_e44668)), ((((var_fn487_calc_ig__vgin_dn2 / var_fn487_calc_ig__vgsatqin) * assign46010_e44664) + (assign46010_e44661 * (var_fn487_calc_ig__vgin_dn2 / var_fn487_calc_ig__vgsatqin))) / (2.0 * assign46010_e44668)), ((((var_fn487_calc_ig__vgin_dn8 / var_fn487_calc_ig__vgsatqin) * assign46010_e44664) + (assign46010_e44661 * (var_fn487_calc_ig__vgin_dn8 / var_fn487_calc_ig__vgsatqin))) / (2.0 * assign46010_e44668)), ((((var_fn487_calc_ig__vgin_dn18 / var_fn487_calc_ig__vgsatqin) * assign46010_e44664) + (assign46010_e44661 * (var_fn487_calc_ig__vgin_dn18 / var_fn487_calc_ig__vgsatqin))) / (2.0 * assign46010_e44668)), ((((var_fn487_calc_ig__vgin_dn19 / var_fn487_calc_ig__vgsatqin) * assign46010_e44664) + (assign46010_e44661 * (var_fn487_calc_ig__vgin_dn19 / var_fn487_calc_ig__vgsatqin))) / (2.0 * assign46010_e44668)),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign46010_e44670, assign46010_e44670_d_n0, assign46010_e44670_d_n2, assign46010_e44670_d_n8, assign46010_e44670_d_n18, assign46010_e44670_d_n19,)
            }
        };
        let assign46010_e44673: f64 = (assign46010_e44671).powf(var_fn487_calc_ig__betarecin);
        let assign46010_e44674: f64 = (1.0 + assign46010_e44673);
        let assign46010_e44677: f64 = (1.0 / var_fn487_calc_ig__betarecin);
        let assign46010_e44678: f64 = (assign46010_e44674).powf(assign46010_e44677);
        let assign46010_e44679: f64 = (assign46010_e44638 / assign46010_e44678);
        (assign46010_e44679, ((((-var_fn487_calc_ig__vgin_dn0) * assign46010_e44678) - (assign46010_e44638 * if 0.0 == 0.0 && ((assign46010_e44677) as f64).is_finite() && ((assign46010_e44677) as f64).fract() == 0.0 { if assign46010_e44677 == 0.0 { 0.0 } else { (assign46010_e44677 * ((assign46010_e44674).powf(assign46010_e44677 - 1.0) * if 0.0 == 0.0 && ((var_fn487_calc_ig__betarecin) as f64).is_finite() && ((var_fn487_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn487_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn487_calc_ig__betarecin * ((assign46010_e44671).powf(var_fn487_calc_ig__betarecin - 1.0) * assign46010_e44671_d_n0)) } } else { (assign46010_e44673 * (var_fn487_calc_ig__betarecin * (assign46010_e44671_d_n0 / assign46010_e44671))) })) } } else { (assign46010_e44678 * (assign46010_e44677 * (if 0.0 == 0.0 && ((var_fn487_calc_ig__betarecin) as f64).is_finite() && ((var_fn487_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn487_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn487_calc_ig__betarecin * ((assign46010_e44671).powf(var_fn487_calc_ig__betarecin - 1.0) * assign46010_e44671_d_n0)) } } else { (assign46010_e44673 * (var_fn487_calc_ig__betarecin * (assign46010_e44671_d_n0 / assign46010_e44671))) } / assign46010_e44674))) })) / (assign46010_e44678 * assign46010_e44678)), ((((-var_fn487_calc_ig__vgin_dn2) * assign46010_e44678) - (assign46010_e44638 * if 0.0 == 0.0 && ((assign46010_e44677) as f64).is_finite() && ((assign46010_e44677) as f64).fract() == 0.0 { if assign46010_e44677 == 0.0 { 0.0 } else { (assign46010_e44677 * ((assign46010_e44674).powf(assign46010_e44677 - 1.0) * if 0.0 == 0.0 && ((var_fn487_calc_ig__betarecin) as f64).is_finite() && ((var_fn487_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn487_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn487_calc_ig__betarecin * ((assign46010_e44671).powf(var_fn487_calc_ig__betarecin - 1.0) * assign46010_e44671_d_n2)) } } else { (assign46010_e44673 * (var_fn487_calc_ig__betarecin * (assign46010_e44671_d_n2 / assign46010_e44671))) })) } } else { (assign46010_e44678 * (assign46010_e44677 * (if 0.0 == 0.0 && ((var_fn487_calc_ig__betarecin) as f64).is_finite() && ((var_fn487_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn487_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn487_calc_ig__betarecin * ((assign46010_e44671).powf(var_fn487_calc_ig__betarecin - 1.0) * assign46010_e44671_d_n2)) } } else { (assign46010_e44673 * (var_fn487_calc_ig__betarecin * (assign46010_e44671_d_n2 / assign46010_e44671))) } / assign46010_e44674))) })) / (assign46010_e44678 * assign46010_e44678)), ((((-var_fn487_calc_ig__vgin_dn8) * assign46010_e44678) - (assign46010_e44638 * if 0.0 == 0.0 && ((assign46010_e44677) as f64).is_finite() && ((assign46010_e44677) as f64).fract() == 0.0 { if assign46010_e44677 == 0.0 { 0.0 } else { (assign46010_e44677 * ((assign46010_e44674).powf(assign46010_e44677 - 1.0) * if 0.0 == 0.0 && ((var_fn487_calc_ig__betarecin) as f64).is_finite() && ((var_fn487_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn487_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn487_calc_ig__betarecin * ((assign46010_e44671).powf(var_fn487_calc_ig__betarecin - 1.0) * assign46010_e44671_d_n8)) } } else { (assign46010_e44673 * (var_fn487_calc_ig__betarecin * (assign46010_e44671_d_n8 / assign46010_e44671))) })) } } else { (assign46010_e44678 * (assign46010_e44677 * (if 0.0 == 0.0 && ((var_fn487_calc_ig__betarecin) as f64).is_finite() && ((var_fn487_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn487_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn487_calc_ig__betarecin * ((assign46010_e44671).powf(var_fn487_calc_ig__betarecin - 1.0) * assign46010_e44671_d_n8)) } } else { (assign46010_e44673 * (var_fn487_calc_ig__betarecin * (assign46010_e44671_d_n8 / assign46010_e44671))) } / assign46010_e44674))) })) / (assign46010_e44678 * assign46010_e44678)), ((((-var_fn487_calc_ig__vgin_dn18) * assign46010_e44678) - (assign46010_e44638 * if 0.0 == 0.0 && ((assign46010_e44677) as f64).is_finite() && ((assign46010_e44677) as f64).fract() == 0.0 { if assign46010_e44677 == 0.0 { 0.0 } else { (assign46010_e44677 * ((assign46010_e44674).powf(assign46010_e44677 - 1.0) * if 0.0 == 0.0 && ((var_fn487_calc_ig__betarecin) as f64).is_finite() && ((var_fn487_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn487_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn487_calc_ig__betarecin * ((assign46010_e44671).powf(var_fn487_calc_ig__betarecin - 1.0) * assign46010_e44671_d_n18)) } } else { (assign46010_e44673 * (var_fn487_calc_ig__betarecin * (assign46010_e44671_d_n18 / assign46010_e44671))) })) } } else { (assign46010_e44678 * (assign46010_e44677 * (if 0.0 == 0.0 && ((var_fn487_calc_ig__betarecin) as f64).is_finite() && ((var_fn487_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn487_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn487_calc_ig__betarecin * ((assign46010_e44671).powf(var_fn487_calc_ig__betarecin - 1.0) * assign46010_e44671_d_n18)) } } else { (assign46010_e44673 * (var_fn487_calc_ig__betarecin * (assign46010_e44671_d_n18 / assign46010_e44671))) } / assign46010_e44674))) })) / (assign46010_e44678 * assign46010_e44678)), ((((-var_fn487_calc_ig__vgin_dn19) * assign46010_e44678) - (assign46010_e44638 * if 0.0 == 0.0 && ((assign46010_e44677) as f64).is_finite() && ((assign46010_e44677) as f64).fract() == 0.0 { if assign46010_e44677 == 0.0 { 0.0 } else { (assign46010_e44677 * ((assign46010_e44674).powf(assign46010_e44677 - 1.0) * if 0.0 == 0.0 && ((var_fn487_calc_ig__betarecin) as f64).is_finite() && ((var_fn487_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn487_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn487_calc_ig__betarecin * ((assign46010_e44671).powf(var_fn487_calc_ig__betarecin - 1.0) * assign46010_e44671_d_n19)) } } else { (assign46010_e44673 * (var_fn487_calc_ig__betarecin * (assign46010_e44671_d_n19 / assign46010_e44671))) })) } } else { (assign46010_e44678 * (assign46010_e44677 * (if 0.0 == 0.0 && ((var_fn487_calc_ig__betarecin) as f64).is_finite() && ((var_fn487_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn487_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn487_calc_ig__betarecin * ((assign46010_e44671).powf(var_fn487_calc_ig__betarecin - 1.0) * assign46010_e44671_d_n19)) } } else { (assign46010_e44673 * (var_fn487_calc_ig__betarecin * (assign46010_e44671_d_n19 / assign46010_e44671))) } / assign46010_e44674))) })) / (assign46010_e44678 * assign46010_e44678)),)
    } else {
        (var_fn487_calc_ig__frecgin, var_fn487_calc_ig__frecgin_dn0, var_fn487_calc_ig__frecgin_dn2, var_fn487_calc_ig__frecgin_dn8, var_fn487_calc_ig__frecgin_dn18, var_fn487_calc_ig__frecgin_dn19,)
    }
};
        var_fn487_calc_ig__frecgin = assign46010_e44681;
        var_fn487_calc_ig__frecgin_dn0 = assign46010_e44681_d_n0;
        var_fn487_calc_ig__frecgin_dn2 = assign46010_e44681_d_n2;
        var_fn487_calc_ig__frecgin_dn8 = assign46010_e44681_d_n8;
        var_fn487_calc_ig__frecgin_dn18 = assign46010_e44681_d_n18;
        var_fn487_calc_ig__frecgin_dn19 = assign46010_e44681_d_n19;

        *var_fn487_calc_ig__alpha2_phit_slot = var_fn487_calc_ig__alpha2_phit;
        *var_fn487_calc_ig__alpha2_phit_dn4_slot = var_fn487_calc_ig__alpha2_phit_dn4;
        *var_fn487_calc_ig__expbd1_vgsat_slot = var_fn487_calc_ig__expbd1_vgsat;
        *var_fn487_calc_ig__expbd1_vgsat_dn4_slot = var_fn487_calc_ig__expbd1_vgsat_dn4;
        *var_fn487_calc_ig__expbdarg1_vgsat_slot = var_fn487_calc_ig__expbdarg1_vgsat;
        *var_fn487_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn487_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn487_calc_ig__expffvarg_slot = var_fn487_calc_ig__expffvarg;
        *var_fn487_calc_ig__expffvarg_dn0_slot = var_fn487_calc_ig__expffvarg_dn0;
        *var_fn487_calc_ig__expffvarg_dn18_slot = var_fn487_calc_ig__expffvarg_dn18;
        *var_fn487_calc_ig__expffvarg_dn19_slot = var_fn487_calc_ig__expffvarg_dn19;
        *var_fn487_calc_ig__expffvarg_dn2_slot = var_fn487_calc_ig__expffvarg_dn2;
        *var_fn487_calc_ig__expffvarg_dn4_slot = var_fn487_calc_ig__expffvarg_dn4;
        *var_fn487_calc_ig__expffvarg_dn8_slot = var_fn487_calc_ig__expffvarg_dn8;
        *var_fn487_calc_ig__expifor_hinj_slot = var_fn487_calc_ig__expifor_hinj;
        *var_fn487_calc_ig__expifor_hinj_dn0_slot = var_fn487_calc_ig__expifor_hinj_dn0;
        *var_fn487_calc_ig__expifor_hinj_dn18_slot = var_fn487_calc_ig__expifor_hinj_dn18;
        *var_fn487_calc_ig__expifor_hinj_dn19_slot = var_fn487_calc_ig__expifor_hinj_dn19;
        *var_fn487_calc_ig__expifor_hinj_dn2_slot = var_fn487_calc_ig__expifor_hinj_dn2;
        *var_fn487_calc_ig__expifor_hinj_dn4_slot = var_fn487_calc_ig__expifor_hinj_dn4;
        *var_fn487_calc_ig__expifor_hinj_dn8_slot = var_fn487_calc_ig__expifor_hinj_dn8;
        *var_fn487_calc_ig__expifor_hinj_vgsat_slot = var_fn487_calc_ig__expifor_hinj_vgsat;
        *var_fn487_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn487_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn487_calc_ig__expifor_nohinj_vgsat_slot = var_fn487_calc_ig__expifor_nohinj_vgsat;
        *var_fn487_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn487_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn487_calc_ig__expiforarg_hinj_slot = var_fn487_calc_ig__expiforarg_hinj;
        *var_fn487_calc_ig__expiforarg_hinj_dn0_slot = var_fn487_calc_ig__expiforarg_hinj_dn0;
        *var_fn487_calc_ig__expiforarg_hinj_dn18_slot = var_fn487_calc_ig__expiforarg_hinj_dn18;
        *var_fn487_calc_ig__expiforarg_hinj_dn19_slot = var_fn487_calc_ig__expiforarg_hinj_dn19;
        *var_fn487_calc_ig__expiforarg_hinj_dn2_slot = var_fn487_calc_ig__expiforarg_hinj_dn2;
        *var_fn487_calc_ig__expiforarg_hinj_dn4_slot = var_fn487_calc_ig__expiforarg_hinj_dn4;
        *var_fn487_calc_ig__expiforarg_hinj_dn8_slot = var_fn487_calc_ig__expiforarg_hinj_dn8;
        *var_fn487_calc_ig__expiforarg_hinj_vgsat_slot = var_fn487_calc_ig__expiforarg_hinj_vgsat;
        *var_fn487_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn487_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn487_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn487_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn487_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn487_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn487_calc_ig__ffvgin_slot = var_fn487_calc_ig__ffvgin;
        *var_fn487_calc_ig__ffvgin_dn0_slot = var_fn487_calc_ig__ffvgin_dn0;
        *var_fn487_calc_ig__ffvgin_dn18_slot = var_fn487_calc_ig__ffvgin_dn18;
        *var_fn487_calc_ig__ffvgin_dn19_slot = var_fn487_calc_ig__ffvgin_dn19;
        *var_fn487_calc_ig__ffvgin_dn2_slot = var_fn487_calc_ig__ffvgin_dn2;
        *var_fn487_calc_ig__ffvgin_dn4_slot = var_fn487_calc_ig__ffvgin_dn4;
        *var_fn487_calc_ig__ffvgin_dn8_slot = var_fn487_calc_ig__ffvgin_dn8;
        *var_fn487_calc_ig__frecgin_slot = var_fn487_calc_ig__frecgin;
        *var_fn487_calc_ig__frecgin_dn0_slot = var_fn487_calc_ig__frecgin_dn0;
        *var_fn487_calc_ig__frecgin_dn18_slot = var_fn487_calc_ig__frecgin_dn18;
        *var_fn487_calc_ig__frecgin_dn19_slot = var_fn487_calc_ig__frecgin_dn19;
        *var_fn487_calc_ig__frecgin_dn2_slot = var_fn487_calc_ig__frecgin_dn2;
        *var_fn487_calc_ig__frecgin_dn8_slot = var_fn487_calc_ig__frecgin_dn8;
        *var_fn487_calc_ig__iginbd_vgsat_slot = var_fn487_calc_ig__iginbd_vgsat;
        *var_fn487_calc_ig__iginbd_vgsat_dn4_slot = var_fn487_calc_ig__iginbd_vgsat_dn4;
        *var_fn487_calc_ig__igindiode_slot = var_fn487_calc_ig__igindiode;
        *var_fn487_calc_ig__igindiode_dn0_slot = var_fn487_calc_ig__igindiode_dn0;
        *var_fn487_calc_ig__igindiode_dn18_slot = var_fn487_calc_ig__igindiode_dn18;
        *var_fn487_calc_ig__igindiode_dn19_slot = var_fn487_calc_ig__igindiode_dn19;
        *var_fn487_calc_ig__igindiode_dn2_slot = var_fn487_calc_ig__igindiode_dn2;
        *var_fn487_calc_ig__igindiode_dn4_slot = var_fn487_calc_ig__igindiode_dn4;
        *var_fn487_calc_ig__igindiode_dn8_slot = var_fn487_calc_ig__igindiode_dn8;
        *var_fn487_calc_ig__igindiode_hinj_slot = var_fn487_calc_ig__igindiode_hinj;
        *var_fn487_calc_ig__igindiode_hinj_dn0_slot = var_fn487_calc_ig__igindiode_hinj_dn0;
        *var_fn487_calc_ig__igindiode_hinj_dn18_slot = var_fn487_calc_ig__igindiode_hinj_dn18;
        *var_fn487_calc_ig__igindiode_hinj_dn19_slot = var_fn487_calc_ig__igindiode_hinj_dn19;
        *var_fn487_calc_ig__igindiode_hinj_dn2_slot = var_fn487_calc_ig__igindiode_hinj_dn2;
        *var_fn487_calc_ig__igindiode_hinj_dn4_slot = var_fn487_calc_ig__igindiode_hinj_dn4;
        *var_fn487_calc_ig__igindiode_hinj_dn8_slot = var_fn487_calc_ig__igindiode_hinj_dn8;
        *var_fn487_calc_ig__igindiode_hinj_pre_slot = var_fn487_calc_ig__igindiode_hinj_pre;
        *var_fn487_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn487_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn487_calc_ig__igindiode_hinj_vgsat_slot = var_fn487_calc_ig__igindiode_hinj_vgsat;
        *var_fn487_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn487_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn487_calc_ig__igindiode_nohinj_slot = var_fn487_calc_ig__igindiode_nohinj;
        *var_fn487_calc_ig__igindiode_nohinj_dn0_slot = var_fn487_calc_ig__igindiode_nohinj_dn0;
        *var_fn487_calc_ig__igindiode_nohinj_dn18_slot = var_fn487_calc_ig__igindiode_nohinj_dn18;
        *var_fn487_calc_ig__igindiode_nohinj_dn19_slot = var_fn487_calc_ig__igindiode_nohinj_dn19;
        *var_fn487_calc_ig__igindiode_nohinj_dn2_slot = var_fn487_calc_ig__igindiode_nohinj_dn2;
        *var_fn487_calc_ig__igindiode_nohinj_dn4_slot = var_fn487_calc_ig__igindiode_nohinj_dn4;
        *var_fn487_calc_ig__igindiode_nohinj_dn8_slot = var_fn487_calc_ig__igindiode_nohinj_dn8;
        *var_fn487_calc_ig__igindiode_nohinj_vgsat_slot = var_fn487_calc_ig__igindiode_nohinj_vgsat;
        *var_fn487_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn487_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn487_calc_ig__pg_paramin_hinj_slot = var_fn487_calc_ig__pg_paramin_hinj;
        *var_guard489_slot = var_guard489;
        *var_guard490_slot = var_guard490;
        *var_guard491_slot = var_guard491;
    }

    pub(super) fn stamp_transient_block_116(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_cofdmt: f64,
        var_cofdmt0: f64,
        var_cofdmt0_dn4: f64,
        var_cofdmt_dn4: f64,
        var_cofdsmt: f64,
        var_cofdsmt0: f64,
        var_cofdsmt0_dn4: f64,
        var_cofdsmt_dn4: f64,
        var_cofdsubmt: f64,
        var_cofdsubmt0: f64,
        var_cofdsubmt0_dn4: f64,
        var_cofdsubmt_dn4: f64,
        var_cofsmt: f64,
        var_cofsmt0: f64,
        var_cofsmt0_dn4: f64,
        var_cofsmt_dn4: f64,
        var_cofssubmt: f64,
        var_cofssubmt0: f64,
        var_cofssubmt0_dn4: f64,
        var_cofssubmt_dn4: f64,
        var_fn487_calc_ig__frecgin: f64,
        var_fn487_calc_ig__frecgin_dn0: f64,
        var_fn487_calc_ig__frecgin_dn18: f64,
        var_fn487_calc_ig__frecgin_dn19: f64,
        var_fn487_calc_ig__frecgin_dn2: f64,
        var_fn487_calc_ig__frecgin_dn8: f64,
        var_fn487_calc_ig__igindiode: f64,
        var_fn487_calc_ig__igindiode_dn0: f64,
        var_fn487_calc_ig__igindiode_dn18: f64,
        var_fn487_calc_ig__igindiode_dn19: f64,
        var_fn487_calc_ig__igindiode_dn2: f64,
        var_fn487_calc_ig__igindiode_dn4: f64,
        var_fn487_calc_ig__igindiode_dn8: f64,
        var_fn487_calc_ig__irecin: f64,
        var_fn487_calc_ig__ngf: f64,
        var_fn487_calc_ig__pgsrecin: f64,
        var_fn487_calc_ig__phitin: f64,
        var_fn487_calc_ig__phitin_dn4: f64,
        var_fn487_calc_ig__tfacdiodein: f64,
        var_fn487_calc_ig__tfacdiodein_dn4: f64,
        var_fn487_calc_ig__type: f64,
        var_fn487_calc_ig__w: f64,
        var_guard480: f64,
        var_rcd_w: f64,
        var_rcs_w: f64,
        var_fn487_calc_ig__expirev_slot: &mut f64,
        var_fn487_calc_ig__expirev_dn0_slot: &mut f64,
        var_fn487_calc_ig__expirev_dn18_slot: &mut f64,
        var_fn487_calc_ig__expirev_dn19_slot: &mut f64,
        var_fn487_calc_ig__expirev_dn2_slot: &mut f64,
        var_fn487_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn487_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn487_calc_ig__expirevarg_slot: &mut f64,
        var_fn487_calc_ig__expirevarg_dn0_slot: &mut f64,
        var_fn487_calc_ig__expirevarg_dn18_slot: &mut f64,
        var_fn487_calc_ig__expirevarg_dn19_slot: &mut f64,
        var_fn487_calc_ig__expirevarg_dn2_slot: &mut f64,
        var_fn487_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn487_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn487_calc_ig__iginrec_slot: &mut f64,
        var_fn487_calc_ig__iginrec_dn0_slot: &mut f64,
        var_fn487_calc_ig__iginrec_dn18_slot: &mut f64,
        var_fn487_calc_ig__iginrec_dn19_slot: &mut f64,
        var_fn487_calc_ig__iginrec_dn2_slot: &mut f64,
        var_fn487_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn487_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn487_calc_ig__igout_slot: &mut f64,
        var_fn487_calc_ig__igout_dn0_slot: &mut f64,
        var_fn487_calc_ig__igout_dn18_slot: &mut f64,
        var_fn487_calc_ig__igout_dn19_slot: &mut f64,
        var_fn487_calc_ig__igout_dn2_slot: &mut f64,
        var_fn487_calc_ig__igout_dn4_slot: &mut f64,
        var_fn487_calc_ig__igout_dn8_slot: &mut f64,
        var_fn487_calc_ig__isrecout_slot: &mut f64,
        var_fn487_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn487_calc_ig__return_slot: &mut f64,
        var_fn487_calc_ig__return_dn0_slot: &mut f64,
        var_fn487_calc_ig__return_dn18_slot: &mut f64,
        var_fn487_calc_ig__return_dn19_slot: &mut f64,
        var_fn487_calc_ig__return_dn2_slot: &mut f64,
        var_fn487_calc_ig__return_dn4_slot: &mut f64,
        var_fn487_calc_ig__return_dn8_slot: &mut f64,
        var_guard492_slot: &mut f64,
        var_guard493_slot: &mut f64,
        var_guard494_slot: &mut f64,
        var_guard497_slot: &mut f64,
        var_guard498_slot: &mut f64,
        var_guard499_slot: &mut f64,
        var_guard500_slot: &mut f64,
        var_guard501_slot: &mut f64,
        var_guard502_slot: &mut f64,
        var_guard503_slot: &mut f64,
        var_guard504_slot: &mut f64,
        var_guard505_slot: &mut f64,
        var_guard506_slot: &mut f64,
        var_igdcbd_slot: &mut f64,
        var_igdcbd_dn0_slot: &mut f64,
        var_igdcbd_dn18_slot: &mut f64,
        var_igdcbd_dn19_slot: &mut f64,
        var_igdcbd_dn2_slot: &mut f64,
        var_igdcbd_dn4_slot: &mut f64,
        var_igdcbd_dn8_slot: &mut f64,
        var_qofd_slot: &mut f64,
        var_qofd_dn0_slot: &mut f64,
        var_qofd_dn4_slot: &mut f64,
        var_qofd_dn6_slot: &mut f64,
        var_qofds_slot: &mut f64,
        var_qofds_dn0_slot: &mut f64,
        var_qofds_dn2_slot: &mut f64,
        var_qofds_dn4_slot: &mut f64,
        var_qofdsub_slot: &mut f64,
        var_qofdsub_dn0_slot: &mut f64,
        var_qofdsub_dn3_slot: &mut f64,
        var_qofdsub_dn4_slot: &mut f64,
        var_qofs_slot: &mut f64,
        var_qofs_dn2_slot: &mut f64,
        var_qofs_dn4_slot: &mut f64,
        var_qofs_dn6_slot: &mut f64,
        var_qofssub_slot: &mut f64,
        var_qofssub_dn2_slot: &mut f64,
        var_qofssub_dn3_slot: &mut f64,
        var_qofssub_dn4_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let mut var_fn487_calc_ig__expirev: f64 = *var_fn487_calc_ig__expirev_slot;
        let mut var_fn487_calc_ig__expirev_dn0: f64 = *var_fn487_calc_ig__expirev_dn0_slot;
        let mut var_fn487_calc_ig__expirev_dn18: f64 = *var_fn487_calc_ig__expirev_dn18_slot;
        let mut var_fn487_calc_ig__expirev_dn19: f64 = *var_fn487_calc_ig__expirev_dn19_slot;
        let mut var_fn487_calc_ig__expirev_dn2: f64 = *var_fn487_calc_ig__expirev_dn2_slot;
        let mut var_fn487_calc_ig__expirev_dn4: f64 = *var_fn487_calc_ig__expirev_dn4_slot;
        let mut var_fn487_calc_ig__expirev_dn8: f64 = *var_fn487_calc_ig__expirev_dn8_slot;
        let mut var_fn487_calc_ig__expirevarg: f64 = *var_fn487_calc_ig__expirevarg_slot;
        let mut var_fn487_calc_ig__expirevarg_dn0: f64 = *var_fn487_calc_ig__expirevarg_dn0_slot;
        let mut var_fn487_calc_ig__expirevarg_dn18: f64 = *var_fn487_calc_ig__expirevarg_dn18_slot;
        let mut var_fn487_calc_ig__expirevarg_dn19: f64 = *var_fn487_calc_ig__expirevarg_dn19_slot;
        let mut var_fn487_calc_ig__expirevarg_dn2: f64 = *var_fn487_calc_ig__expirevarg_dn2_slot;
        let mut var_fn487_calc_ig__expirevarg_dn4: f64 = *var_fn487_calc_ig__expirevarg_dn4_slot;
        let mut var_fn487_calc_ig__expirevarg_dn8: f64 = *var_fn487_calc_ig__expirevarg_dn8_slot;
        let mut var_fn487_calc_ig__iginrec: f64 = *var_fn487_calc_ig__iginrec_slot;
        let mut var_fn487_calc_ig__iginrec_dn0: f64 = *var_fn487_calc_ig__iginrec_dn0_slot;
        let mut var_fn487_calc_ig__iginrec_dn18: f64 = *var_fn487_calc_ig__iginrec_dn18_slot;
        let mut var_fn487_calc_ig__iginrec_dn19: f64 = *var_fn487_calc_ig__iginrec_dn19_slot;
        let mut var_fn487_calc_ig__iginrec_dn2: f64 = *var_fn487_calc_ig__iginrec_dn2_slot;
        let mut var_fn487_calc_ig__iginrec_dn4: f64 = *var_fn487_calc_ig__iginrec_dn4_slot;
        let mut var_fn487_calc_ig__iginrec_dn8: f64 = *var_fn487_calc_ig__iginrec_dn8_slot;
        let mut var_fn487_calc_ig__igout: f64 = *var_fn487_calc_ig__igout_slot;
        let mut var_fn487_calc_ig__igout_dn0: f64 = *var_fn487_calc_ig__igout_dn0_slot;
        let mut var_fn487_calc_ig__igout_dn18: f64 = *var_fn487_calc_ig__igout_dn18_slot;
        let mut var_fn487_calc_ig__igout_dn19: f64 = *var_fn487_calc_ig__igout_dn19_slot;
        let mut var_fn487_calc_ig__igout_dn2: f64 = *var_fn487_calc_ig__igout_dn2_slot;
        let mut var_fn487_calc_ig__igout_dn4: f64 = *var_fn487_calc_ig__igout_dn4_slot;
        let mut var_fn487_calc_ig__igout_dn8: f64 = *var_fn487_calc_ig__igout_dn8_slot;
        let mut var_fn487_calc_ig__isrecout: f64 = *var_fn487_calc_ig__isrecout_slot;
        let mut var_fn487_calc_ig__isrecout_dn4: f64 = *var_fn487_calc_ig__isrecout_dn4_slot;
        let mut var_fn487_calc_ig__return: f64 = *var_fn487_calc_ig__return_slot;
        let mut var_fn487_calc_ig__return_dn0: f64 = *var_fn487_calc_ig__return_dn0_slot;
        let mut var_fn487_calc_ig__return_dn18: f64 = *var_fn487_calc_ig__return_dn18_slot;
        let mut var_fn487_calc_ig__return_dn19: f64 = *var_fn487_calc_ig__return_dn19_slot;
        let mut var_fn487_calc_ig__return_dn2: f64 = *var_fn487_calc_ig__return_dn2_slot;
        let mut var_fn487_calc_ig__return_dn4: f64 = *var_fn487_calc_ig__return_dn4_slot;
        let mut var_fn487_calc_ig__return_dn8: f64 = *var_fn487_calc_ig__return_dn8_slot;
        let mut var_guard492: f64 = *var_guard492_slot;
        let mut var_guard493: f64 = *var_guard493_slot;
        let mut var_guard494: f64 = *var_guard494_slot;
        let mut var_guard497: f64 = *var_guard497_slot;
        let mut var_guard498: f64 = *var_guard498_slot;
        let mut var_guard499: f64 = *var_guard499_slot;
        let mut var_guard500: f64 = *var_guard500_slot;
        let mut var_guard501: f64 = *var_guard501_slot;
        let mut var_guard502: f64 = *var_guard502_slot;
        let mut var_guard503: f64 = *var_guard503_slot;
        let mut var_guard504: f64 = *var_guard504_slot;
        let mut var_guard505: f64 = *var_guard505_slot;
        let mut var_guard506: f64 = *var_guard506_slot;
        let mut var_igdcbd: f64 = *var_igdcbd_slot;
        let mut var_igdcbd_dn0: f64 = *var_igdcbd_dn0_slot;
        let mut var_igdcbd_dn18: f64 = *var_igdcbd_dn18_slot;
        let mut var_igdcbd_dn19: f64 = *var_igdcbd_dn19_slot;
        let mut var_igdcbd_dn2: f64 = *var_igdcbd_dn2_slot;
        let mut var_igdcbd_dn4: f64 = *var_igdcbd_dn4_slot;
        let mut var_igdcbd_dn8: f64 = *var_igdcbd_dn8_slot;
        let mut var_qofd: f64 = *var_qofd_slot;
        let mut var_qofd_dn0: f64 = *var_qofd_dn0_slot;
        let mut var_qofd_dn4: f64 = *var_qofd_dn4_slot;
        let mut var_qofd_dn6: f64 = *var_qofd_dn6_slot;
        let mut var_qofds: f64 = *var_qofds_slot;
        let mut var_qofds_dn0: f64 = *var_qofds_dn0_slot;
        let mut var_qofds_dn2: f64 = *var_qofds_dn2_slot;
        let mut var_qofds_dn4: f64 = *var_qofds_dn4_slot;
        let mut var_qofdsub: f64 = *var_qofdsub_slot;
        let mut var_qofdsub_dn0: f64 = *var_qofdsub_dn0_slot;
        let mut var_qofdsub_dn3: f64 = *var_qofdsub_dn3_slot;
        let mut var_qofdsub_dn4: f64 = *var_qofdsub_dn4_slot;
        let mut var_qofs: f64 = *var_qofs_slot;
        let mut var_qofs_dn2: f64 = *var_qofs_dn2_slot;
        let mut var_qofs_dn4: f64 = *var_qofs_dn4_slot;
        let mut var_qofs_dn6: f64 = *var_qofs_dn6_slot;
        let mut var_qofssub: f64 = *var_qofssub_slot;
        let mut var_qofssub_dn2: f64 = *var_qofssub_dn2_slot;
        let mut var_qofssub_dn3: f64 = *var_qofssub_dn3_slot;
        let mut var_qofssub_dn4: f64 = *var_qofssub_dn4_slot;

        let (assign46020_e44696, assign46020_e44696_d_n4,) = {
    if (var_guard480 != 0.0) {
        let assign46020_e44684: f64 = (-var_fn487_calc_ig__type);
        let assign46020_e44686: f64 = (assign46020_e44684 * var_fn487_calc_ig__w);
        let assign46020_e44688: f64 = (assign46020_e44686 * var_fn487_calc_ig__ngf);
        let assign46020_e44690: f64 = (assign46020_e44688 * var_fn487_calc_ig__irecin);
        let assign46020_e44692: f64 = (assign46020_e44690 * var_fn487_calc_ig__tfacdiodein);
        let assign46020_e44694: f64 = assign46020_e44692;
        (assign46020_e44694, (assign46020_e44690 * var_fn487_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn487_calc_ig__isrecout, var_fn487_calc_ig__isrecout_dn4,)
    }
};
        var_fn487_calc_ig__isrecout = assign46020_e44696;
        var_fn487_calc_ig__isrecout_dn4 = assign46020_e44696_d_n4;

        let (assign46030_e44704, assign46030_e44704_d_n0, assign46030_e44704_d_n2, assign46030_e44704_d_n4, assign46030_e44704_d_n8, assign46030_e44704_d_n18, assign46030_e44704_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign46030_e44700: f64 = (var_fn487_calc_ig__pgsrecin / var_fn487_calc_ig__phitin);
        let assign46030_e44702: f64 = (assign46030_e44700 * var_fn487_calc_ig__frecgin);
        (assign46030_e44702, (assign46030_e44700 * var_fn487_calc_ig__frecgin_dn0), (assign46030_e44700 * var_fn487_calc_ig__frecgin_dn2), ((-((var_fn487_calc_ig__pgsrecin * var_fn487_calc_ig__phitin_dn4) / (var_fn487_calc_ig__phitin * var_fn487_calc_ig__phitin))) * var_fn487_calc_ig__frecgin), (assign46030_e44700 * var_fn487_calc_ig__frecgin_dn8), (assign46030_e44700 * var_fn487_calc_ig__frecgin_dn18), (assign46030_e44700 * var_fn487_calc_ig__frecgin_dn19),)
    } else {
        (var_fn487_calc_ig__expirevarg, var_fn487_calc_ig__expirevarg_dn0, var_fn487_calc_ig__expirevarg_dn2, var_fn487_calc_ig__expirevarg_dn4, var_fn487_calc_ig__expirevarg_dn8, var_fn487_calc_ig__expirevarg_dn18, var_fn487_calc_ig__expirevarg_dn19,)
    }
};
        var_fn487_calc_ig__expirevarg = assign46030_e44704;
        var_fn487_calc_ig__expirevarg_dn0 = assign46030_e44704_d_n0;
        var_fn487_calc_ig__expirevarg_dn2 = assign46030_e44704_d_n2;
        var_fn487_calc_ig__expirevarg_dn4 = assign46030_e44704_d_n4;
        var_fn487_calc_ig__expirevarg_dn8 = assign46030_e44704_d_n8;
        var_fn487_calc_ig__expirevarg_dn18 = assign46030_e44704_d_n18;
        var_fn487_calc_ig__expirevarg_dn19 = assign46030_e44704_d_n19;

        let (assign46040_e44746, assign46040_e44746_d_n0, assign46040_e44746_d_n2, assign46040_e44746_d_n4, assign46040_e44746_d_n8, assign46040_e44746_d_n18, assign46040_e44746_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign46040_e44712: f64 = (-50.0);
        let (assign46040_e44744, assign46040_e44744_d_n0, assign46040_e44744_d_n2, assign46040_e44744_d_n4, assign46040_e44744_d_n8, assign46040_e44744_d_n18, assign46040_e44744_d_n19,) = {
            if ((!(var_fn487_calc_ig__expirevarg > 50.0)) && (!(var_fn487_calc_ig__expirevarg < assign46040_e44712))) {
                let assign46040_e44717: f64 = (var_fn487_calc_ig__expirevarg).exp();
                (assign46040_e44717, (assign46040_e44717 * var_fn487_calc_ig__expirevarg_dn0), (assign46040_e44717 * var_fn487_calc_ig__expirevarg_dn2), (assign46040_e44717 * var_fn487_calc_ig__expirevarg_dn4), (assign46040_e44717 * var_fn487_calc_ig__expirevarg_dn8), (assign46040_e44717 * var_fn487_calc_ig__expirevarg_dn18), (assign46040_e44717 * var_fn487_calc_ig__expirevarg_dn19),)
            } else {
                let assign46040_e44724: f64 = (-50.0);
                let (assign46040_e44743, assign46040_e44743_d_n0, assign46040_e44743_d_n2, assign46040_e44743_d_n4, assign46040_e44743_d_n8, assign46040_e44743_d_n18, assign46040_e44743_d_n19,) = {
                    if ((!(var_fn487_calc_ig__expirevarg > 50.0)) && (var_fn487_calc_ig__expirevarg < assign46040_e44724)) {
                        let assign46040_e44728: f64 = (-50.0);
                        let assign46040_e44729: f64 = (assign46040_e44728).exp();
                        (assign46040_e44729, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign46040_e44742, assign46040_e44742_d_n0, assign46040_e44742_d_n2, assign46040_e44742_d_n4, assign46040_e44742_d_n8, assign46040_e44742_d_n18, assign46040_e44742_d_n19,) = {
                            if (var_fn487_calc_ig__expirevarg > 50.0) {
                                let assign46040_e44734: f64 = (50.0_f64).exp();
                                let assign46040_e44738: f64 = (var_fn487_calc_ig__expirevarg - 50.0);
                                let assign46040_e44739: f64 = (1.0 + assign46040_e44738);
                                let assign46040_e44740: f64 = (assign46040_e44734 * assign46040_e44739);
                                (assign46040_e44740, (assign46040_e44734 * var_fn487_calc_ig__expirevarg_dn0), (assign46040_e44734 * var_fn487_calc_ig__expirevarg_dn2), (assign46040_e44734 * var_fn487_calc_ig__expirevarg_dn4), (assign46040_e44734 * var_fn487_calc_ig__expirevarg_dn8), (assign46040_e44734 * var_fn487_calc_ig__expirevarg_dn18), (assign46040_e44734 * var_fn487_calc_ig__expirevarg_dn19),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign46040_e44742, assign46040_e44742_d_n0, assign46040_e44742_d_n2, assign46040_e44742_d_n4, assign46040_e44742_d_n8, assign46040_e44742_d_n18, assign46040_e44742_d_n19,)
                    }
                };
                (assign46040_e44743, assign46040_e44743_d_n0, assign46040_e44743_d_n2, assign46040_e44743_d_n4, assign46040_e44743_d_n8, assign46040_e44743_d_n18, assign46040_e44743_d_n19,)
            }
        };
        (assign46040_e44744, assign46040_e44744_d_n0, assign46040_e44744_d_n2, assign46040_e44744_d_n4, assign46040_e44744_d_n8, assign46040_e44744_d_n18, assign46040_e44744_d_n19,)
    } else {
        (var_fn487_calc_ig__expirev, var_fn487_calc_ig__expirev_dn0, var_fn487_calc_ig__expirev_dn2, var_fn487_calc_ig__expirev_dn4, var_fn487_calc_ig__expirev_dn8, var_fn487_calc_ig__expirev_dn18, var_fn487_calc_ig__expirev_dn19,)
    }
};
        var_fn487_calc_ig__expirev = assign46040_e44746;
        var_fn487_calc_ig__expirev_dn0 = assign46040_e44746_d_n0;
        var_fn487_calc_ig__expirev_dn2 = assign46040_e44746_d_n2;
        var_fn487_calc_ig__expirev_dn4 = assign46040_e44746_d_n4;
        var_fn487_calc_ig__expirev_dn8 = assign46040_e44746_d_n8;
        var_fn487_calc_ig__expirev_dn18 = assign46040_e44746_d_n18;
        var_fn487_calc_ig__expirev_dn19 = assign46040_e44746_d_n19;

        let (assign46050_e44754, assign46050_e44754_d_n0, assign46050_e44754_d_n2, assign46050_e44754_d_n4, assign46050_e44754_d_n8, assign46050_e44754_d_n18, assign46050_e44754_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign46050_e44751: f64 = (var_fn487_calc_ig__expirev - 1.0);
        let assign46050_e44752: f64 = (var_fn487_calc_ig__isrecout * assign46050_e44751);
        (assign46050_e44752, (var_fn487_calc_ig__isrecout * var_fn487_calc_ig__expirev_dn0), (var_fn487_calc_ig__isrecout * var_fn487_calc_ig__expirev_dn2), ((var_fn487_calc_ig__isrecout_dn4 * assign46050_e44751) + (var_fn487_calc_ig__isrecout * var_fn487_calc_ig__expirev_dn4)), (var_fn487_calc_ig__isrecout * var_fn487_calc_ig__expirev_dn8), (var_fn487_calc_ig__isrecout * var_fn487_calc_ig__expirev_dn18), (var_fn487_calc_ig__isrecout * var_fn487_calc_ig__expirev_dn19),)
    } else {
        (var_fn487_calc_ig__iginrec, var_fn487_calc_ig__iginrec_dn0, var_fn487_calc_ig__iginrec_dn2, var_fn487_calc_ig__iginrec_dn4, var_fn487_calc_ig__iginrec_dn8, var_fn487_calc_ig__iginrec_dn18, var_fn487_calc_ig__iginrec_dn19,)
    }
};
        var_fn487_calc_ig__iginrec = assign46050_e44754;
        var_fn487_calc_ig__iginrec_dn0 = assign46050_e44754_d_n0;
        var_fn487_calc_ig__iginrec_dn2 = assign46050_e44754_d_n2;
        var_fn487_calc_ig__iginrec_dn4 = assign46050_e44754_d_n4;
        var_fn487_calc_ig__iginrec_dn8 = assign46050_e44754_d_n8;
        var_fn487_calc_ig__iginrec_dn18 = assign46050_e44754_d_n18;
        var_fn487_calc_ig__iginrec_dn19 = assign46050_e44754_d_n19;

        let (assign46060_e44760, assign46060_e44760_d_n0, assign46060_e44760_d_n2, assign46060_e44760_d_n4, assign46060_e44760_d_n8, assign46060_e44760_d_n18, assign46060_e44760_d_n19,) = {
    if (var_guard480 != 0.0) {
        let assign46060_e44758: f64 = (var_fn487_calc_ig__igindiode + var_fn487_calc_ig__iginrec);
        (assign46060_e44758, (var_fn487_calc_ig__igindiode_dn0 + var_fn487_calc_ig__iginrec_dn0), (var_fn487_calc_ig__igindiode_dn2 + var_fn487_calc_ig__iginrec_dn2), (var_fn487_calc_ig__igindiode_dn4 + var_fn487_calc_ig__iginrec_dn4), (var_fn487_calc_ig__igindiode_dn8 + var_fn487_calc_ig__iginrec_dn8), (var_fn487_calc_ig__igindiode_dn18 + var_fn487_calc_ig__iginrec_dn18), (var_fn487_calc_ig__igindiode_dn19 + var_fn487_calc_ig__iginrec_dn19),)
    } else {
        (var_fn487_calc_ig__igout, var_fn487_calc_ig__igout_dn0, var_fn487_calc_ig__igout_dn2, var_fn487_calc_ig__igout_dn4, var_fn487_calc_ig__igout_dn8, var_fn487_calc_ig__igout_dn18, var_fn487_calc_ig__igout_dn19,)
    }
};
        var_fn487_calc_ig__igout = assign46060_e44760;
        var_fn487_calc_ig__igout_dn0 = assign46060_e44760_d_n0;
        var_fn487_calc_ig__igout_dn2 = assign46060_e44760_d_n2;
        var_fn487_calc_ig__igout_dn4 = assign46060_e44760_d_n4;
        var_fn487_calc_ig__igout_dn8 = assign46060_e44760_d_n8;
        var_fn487_calc_ig__igout_dn18 = assign46060_e44760_d_n18;
        var_fn487_calc_ig__igout_dn19 = assign46060_e44760_d_n19;

        let (assign46070_e44764, assign46070_e44764_d_n0, assign46070_e44764_d_n2, assign46070_e44764_d_n4, assign46070_e44764_d_n8, assign46070_e44764_d_n18, assign46070_e44764_d_n19,) = {
    if (var_guard480 != 0.0) {
        (var_fn487_calc_ig__igout, var_fn487_calc_ig__igout_dn0, var_fn487_calc_ig__igout_dn2, var_fn487_calc_ig__igout_dn4, var_fn487_calc_ig__igout_dn8, var_fn487_calc_ig__igout_dn18, var_fn487_calc_ig__igout_dn19,)
    } else {
        (var_fn487_calc_ig__return, var_fn487_calc_ig__return_dn0, var_fn487_calc_ig__return_dn2, var_fn487_calc_ig__return_dn4, var_fn487_calc_ig__return_dn8, var_fn487_calc_ig__return_dn18, var_fn487_calc_ig__return_dn19,)
    }
};
        var_fn487_calc_ig__return = assign46070_e44764;
        var_fn487_calc_ig__return_dn0 = assign46070_e44764_d_n0;
        var_fn487_calc_ig__return_dn2 = assign46070_e44764_d_n2;
        var_fn487_calc_ig__return_dn4 = assign46070_e44764_d_n4;
        var_fn487_calc_ig__return_dn8 = assign46070_e44764_d_n8;
        var_fn487_calc_ig__return_dn18 = assign46070_e44764_d_n18;
        var_fn487_calc_ig__return_dn19 = assign46070_e44764_d_n19;

        let (assign46100_e44776, assign46100_e44776_d_n0, assign46100_e44776_d_n2, assign46100_e44776_d_n4, assign46100_e44776_d_n8, assign46100_e44776_d_n18, assign46100_e44776_d_n19,) = {
    if (var_guard480 != 0.0) {
        (var_fn487_calc_ig__return, var_fn487_calc_ig__return_dn0, var_fn487_calc_ig__return_dn2, var_fn487_calc_ig__return_dn4, var_fn487_calc_ig__return_dn8, var_fn487_calc_ig__return_dn18, var_fn487_calc_ig__return_dn19,)
    } else {
        (var_igdcbd, var_igdcbd_dn0, var_igdcbd_dn2, var_igdcbd_dn4, var_igdcbd_dn8, var_igdcbd_dn18, var_igdcbd_dn19,)
    }
};
        var_igdcbd = assign46100_e44776;
        var_igdcbd_dn0 = assign46100_e44776_d_n0;
        var_igdcbd_dn2 = assign46100_e44776_d_n2;
        var_igdcbd_dn4 = assign46100_e44776_d_n4;
        var_igdcbd_dn8 = assign46100_e44776_d_n8;
        var_igdcbd_dn18 = assign46100_e44776_d_n18;
        var_igdcbd_dn19 = assign46100_e44776_d_n19;

        let assign46110_e44779: f64 = if p.p313 == 0.0 { 1.0 } else { 0.0 };
        var_guard492 = assign46110_e44779;

        let assign46120_e44786: f64 = if ((var_rcd_w >= p.p353) && (var_rcd_w > 0.0)) { 1.0 } else { 0.0 };
        var_guard493 = assign46120_e44786;

        let assign46130_e44793: f64 = if ((var_rcs_w >= p.p353) && (var_rcs_w > 0.0)) { 1.0 } else { 0.0 };
        var_guard494 = assign46130_e44793;

        let assign46160_e44810: f64 = ((nv6 - nv2) - p.p27);
        let assign46160_e44812: f64 = (assign46160_e44810 / p.p28);
        let assign46160_e44814: f64 = if assign46160_e44812 > 50.0 { 1.0 } else { 0.0 };
        var_guard497 = assign46160_e44814;

        let (assign46170_e44830, assign46170_e44830_d_n2, assign46170_e44830_d_n4, assign46170_e44830_d_n6,) = {
    if (var_guard497 != 0.0) {
        let assign46170_e44818: f64 = (p.p0 * p.p2);
        let assign46170_e44821: f64 = (var_cofsmt0 * (nv6 - nv2));
        let assign46170_e44825: f64 = ((nv6 - nv2) - p.p27);
        let assign46170_e44826: f64 = (var_cofsmt * assign46170_e44825);
        let assign46170_e44827: f64 = (assign46170_e44821 + assign46170_e44826);
        let assign46170_e44828: f64 = (assign46170_e44818 * assign46170_e44827);
        (assign46170_e44828, (assign46170_e44818 * ((-var_cofsmt0) + (-var_cofsmt))), (assign46170_e44818 * ((var_cofsmt0_dn4 * (nv6 - nv2)) + (var_cofsmt_dn4 * assign46170_e44825))), (assign46170_e44818 * (var_cofsmt0 + var_cofsmt)),)
    } else {
        (var_qofs, var_qofs_dn2, var_qofs_dn4, var_qofs_dn6,)
    }
};
        var_qofs = assign46170_e44830;
        var_qofs_dn2 = assign46170_e44830_d_n2;
        var_qofs_dn4 = assign46170_e44830_d_n4;
        var_qofs_dn6 = assign46170_e44830_d_n6;

        let assign46180_e44833: f64 = ((nv6 - nv2) - p.p27);
        let assign46180_e44835: f64 = (assign46180_e44833 / p.p28);
        let assign46180_e44837: f64 = (-50.0);
        let assign46180_e44838: f64 = if assign46180_e44835 < assign46180_e44837 { 1.0 } else { 0.0 };
        var_guard498 = assign46180_e44838;

        let (assign46190_e44862, assign46190_e44862_d_n2, assign46190_e44862_d_n4, assign46190_e44862_d_n6,) = {
    if ((var_guard497 == 0.0) && (var_guard498 != 0.0)) {
        let assign46190_e44845: f64 = (p.p0 * p.p2);
        let assign46190_e44848: f64 = (var_cofsmt0 * (nv6 - nv2));
        let assign46190_e44851: f64 = (var_cofsmt * p.p28);
        let assign46190_e44854: f64 = ((nv6 - nv2) - p.p27);
        let assign46190_e44856: f64 = (assign46190_e44854 / p.p28);
        let assign46190_e44857: f64 = (assign46190_e44856).exp();
        let assign46190_e44858: f64 = (assign46190_e44851 * assign46190_e44857);
        let assign46190_e44859: f64 = (assign46190_e44848 + assign46190_e44858);
        let assign46190_e44860: f64 = (assign46190_e44845 * assign46190_e44859);
        (assign46190_e44860, (assign46190_e44845 * ((-var_cofsmt0) + (assign46190_e44851 * (assign46190_e44857 * (-1.0 / p.p28))))), (assign46190_e44845 * ((var_cofsmt0_dn4 * (nv6 - nv2)) + ((var_cofsmt_dn4 * p.p28) * assign46190_e44857))), (assign46190_e44845 * (var_cofsmt0 + (assign46190_e44851 * (assign46190_e44857 * (1.0 / p.p28))))),)
    } else {
        (var_qofs, var_qofs_dn2, var_qofs_dn4, var_qofs_dn6,)
    }
};
        var_qofs = assign46190_e44862;
        var_qofs_dn2 = assign46190_e44862_d_n2;
        var_qofs_dn4 = assign46190_e44862_d_n4;
        var_qofs_dn6 = assign46190_e44862_d_n6;

        let (assign46200_e44890, assign46200_e44890_d_n2, assign46200_e44890_d_n4, assign46200_e44890_d_n6,) = {
    if ((var_guard497 == 0.0) && (var_guard498 == 0.0)) {
        let assign46200_e44870: f64 = (p.p0 * p.p2);
        let assign46200_e44873: f64 = (var_cofsmt0 * (nv6 - nv2));
        let assign46200_e44876: f64 = (var_cofsmt * p.p28);
        let assign46200_e44880: f64 = ((nv6 - nv2) - p.p27);
        let assign46200_e44882: f64 = (assign46200_e44880 / p.p28);
        let assign46200_e44883: f64 = (assign46200_e44882).exp();
        let assign46200_e44884: f64 = (1.0 + assign46200_e44883);
        let assign46200_e44885: f64 = (assign46200_e44884).ln();
        let assign46200_e44886: f64 = (assign46200_e44876 * assign46200_e44885);
        let assign46200_e44887: f64 = (assign46200_e44873 + assign46200_e44886);
        let assign46200_e44888: f64 = (assign46200_e44870 * assign46200_e44887);
        (assign46200_e44888, (assign46200_e44870 * ((-var_cofsmt0) + (assign46200_e44876 * ((assign46200_e44883 * (-1.0 / p.p28)) / assign46200_e44884)))), (assign46200_e44870 * ((var_cofsmt0_dn4 * (nv6 - nv2)) + ((var_cofsmt_dn4 * p.p28) * assign46200_e44885))), (assign46200_e44870 * (var_cofsmt0 + (assign46200_e44876 * ((assign46200_e44883 * (1.0 / p.p28)) / assign46200_e44884)))),)
    } else {
        (var_qofs, var_qofs_dn2, var_qofs_dn4, var_qofs_dn6,)
    }
};
        var_qofs = assign46200_e44890;
        var_qofs_dn2 = assign46200_e44890_d_n2;
        var_qofs_dn4 = assign46200_e44890_d_n4;
        var_qofs_dn6 = assign46200_e44890_d_n6;

        let assign46210_e44893: f64 = ((nv6 - nv0) - p.p27);
        let assign46210_e44895: f64 = (assign46210_e44893 / p.p28);
        let assign46210_e44897: f64 = if assign46210_e44895 > 50.0 { 1.0 } else { 0.0 };
        var_guard499 = assign46210_e44897;

        let (assign46220_e44913, assign46220_e44913_d_n0, assign46220_e44913_d_n4, assign46220_e44913_d_n6,) = {
    if (var_guard499 != 0.0) {
        let assign46220_e44901: f64 = (p.p0 * p.p2);
        let assign46220_e44904: f64 = (var_cofdmt0 * (nv6 - nv0));
        let assign46220_e44908: f64 = ((nv6 - nv0) - p.p27);
        let assign46220_e44909: f64 = (var_cofdmt * assign46220_e44908);
        let assign46220_e44910: f64 = (assign46220_e44904 + assign46220_e44909);
        let assign46220_e44911: f64 = (assign46220_e44901 * assign46220_e44910);
        (assign46220_e44911, (assign46220_e44901 * ((-var_cofdmt0) + (-var_cofdmt))), (assign46220_e44901 * ((var_cofdmt0_dn4 * (nv6 - nv0)) + (var_cofdmt_dn4 * assign46220_e44908))), (assign46220_e44901 * (var_cofdmt0 + var_cofdmt)),)
    } else {
        (var_qofd, var_qofd_dn0, var_qofd_dn4, var_qofd_dn6,)
    }
};
        var_qofd = assign46220_e44913;
        var_qofd_dn0 = assign46220_e44913_d_n0;
        var_qofd_dn4 = assign46220_e44913_d_n4;
        var_qofd_dn6 = assign46220_e44913_d_n6;

        let assign46230_e44916: f64 = ((nv6 - nv0) - p.p27);
        let assign46230_e44918: f64 = (assign46230_e44916 / p.p28);
        let assign46230_e44920: f64 = (-50.0);
        let assign46230_e44921: f64 = if assign46230_e44918 < assign46230_e44920 { 1.0 } else { 0.0 };
        var_guard500 = assign46230_e44921;

        let (assign46240_e44945, assign46240_e44945_d_n0, assign46240_e44945_d_n4, assign46240_e44945_d_n6,) = {
    if ((var_guard499 == 0.0) && (var_guard500 != 0.0)) {
        let assign46240_e44928: f64 = (p.p0 * p.p2);
        let assign46240_e44931: f64 = (var_cofdmt0 * (nv6 - nv0));
        let assign46240_e44934: f64 = (var_cofdmt * p.p28);
        let assign46240_e44937: f64 = ((nv6 - nv0) - p.p27);
        let assign46240_e44939: f64 = (assign46240_e44937 / p.p28);
        let assign46240_e44940: f64 = (assign46240_e44939).exp();
        let assign46240_e44941: f64 = (assign46240_e44934 * assign46240_e44940);
        let assign46240_e44942: f64 = (assign46240_e44931 + assign46240_e44941);
        let assign46240_e44943: f64 = (assign46240_e44928 * assign46240_e44942);
        (assign46240_e44943, (assign46240_e44928 * ((-var_cofdmt0) + (assign46240_e44934 * (assign46240_e44940 * (-1.0 / p.p28))))), (assign46240_e44928 * ((var_cofdmt0_dn4 * (nv6 - nv0)) + ((var_cofdmt_dn4 * p.p28) * assign46240_e44940))), (assign46240_e44928 * (var_cofdmt0 + (assign46240_e44934 * (assign46240_e44940 * (1.0 / p.p28))))),)
    } else {
        (var_qofd, var_qofd_dn0, var_qofd_dn4, var_qofd_dn6,)
    }
};
        var_qofd = assign46240_e44945;
        var_qofd_dn0 = assign46240_e44945_d_n0;
        var_qofd_dn4 = assign46240_e44945_d_n4;
        var_qofd_dn6 = assign46240_e44945_d_n6;

        let (assign46250_e44973, assign46250_e44973_d_n0, assign46250_e44973_d_n4, assign46250_e44973_d_n6,) = {
    if ((var_guard499 == 0.0) && (var_guard500 == 0.0)) {
        let assign46250_e44953: f64 = (p.p0 * p.p2);
        let assign46250_e44956: f64 = (var_cofdmt0 * (nv6 - nv0));
        let assign46250_e44959: f64 = (var_cofdmt * p.p28);
        let assign46250_e44963: f64 = ((nv6 - nv0) - p.p27);
        let assign46250_e44965: f64 = (assign46250_e44963 / p.p28);
        let assign46250_e44966: f64 = (assign46250_e44965).exp();
        let assign46250_e44967: f64 = (1.0 + assign46250_e44966);
        let assign46250_e44968: f64 = (assign46250_e44967).ln();
        let assign46250_e44969: f64 = (assign46250_e44959 * assign46250_e44968);
        let assign46250_e44970: f64 = (assign46250_e44956 + assign46250_e44969);
        let assign46250_e44971: f64 = (assign46250_e44953 * assign46250_e44970);
        (assign46250_e44971, (assign46250_e44953 * ((-var_cofdmt0) + (assign46250_e44959 * ((assign46250_e44966 * (-1.0 / p.p28)) / assign46250_e44967)))), (assign46250_e44953 * ((var_cofdmt0_dn4 * (nv6 - nv0)) + ((var_cofdmt_dn4 * p.p28) * assign46250_e44968))), (assign46250_e44953 * (var_cofdmt0 + (assign46250_e44959 * ((assign46250_e44966 * (1.0 / p.p28)) / assign46250_e44967)))),)
    } else {
        (var_qofd, var_qofd_dn0, var_qofd_dn4, var_qofd_dn6,)
    }
};
        var_qofd = assign46250_e44973;
        var_qofd_dn0 = assign46250_e44973_d_n0;
        var_qofd_dn4 = assign46250_e44973_d_n4;
        var_qofd_dn6 = assign46250_e44973_d_n6;

        let assign46260_e44976: f64 = ((nv2 - nv0) - p.p27);
        let assign46260_e44978: f64 = (assign46260_e44976 / p.p28);
        let assign46260_e44980: f64 = if assign46260_e44978 > 50.0 { 1.0 } else { 0.0 };
        var_guard501 = assign46260_e44980;

        let (assign46270_e44996, assign46270_e44996_d_n0, assign46270_e44996_d_n2, assign46270_e44996_d_n4,) = {
    if (var_guard501 != 0.0) {
        let assign46270_e44984: f64 = (p.p0 * p.p2);
        let assign46270_e44987: f64 = (var_cofdsmt0 * (nv2 - nv0));
        let assign46270_e44991: f64 = ((nv2 - nv0) - p.p27);
        let assign46270_e44992: f64 = (var_cofdsmt * assign46270_e44991);
        let assign46270_e44993: f64 = (assign46270_e44987 + assign46270_e44992);
        let assign46270_e44994: f64 = (assign46270_e44984 * assign46270_e44993);
        (assign46270_e44994, (assign46270_e44984 * ((-var_cofdsmt0) + (-var_cofdsmt))), (assign46270_e44984 * (var_cofdsmt0 + var_cofdsmt)), (assign46270_e44984 * ((var_cofdsmt0_dn4 * (nv2 - nv0)) + (var_cofdsmt_dn4 * assign46270_e44991))),)
    } else {
        (var_qofds, var_qofds_dn0, var_qofds_dn2, var_qofds_dn4,)
    }
};
        var_qofds = assign46270_e44996;
        var_qofds_dn0 = assign46270_e44996_d_n0;
        var_qofds_dn2 = assign46270_e44996_d_n2;
        var_qofds_dn4 = assign46270_e44996_d_n4;

        let assign46280_e44999: f64 = ((nv2 - nv0) - p.p27);
        let assign46280_e45001: f64 = (assign46280_e44999 / p.p28);
        let assign46280_e45003: f64 = (-50.0);
        let assign46280_e45004: f64 = if assign46280_e45001 < assign46280_e45003 { 1.0 } else { 0.0 };
        var_guard502 = assign46280_e45004;

        let (assign46290_e45028, assign46290_e45028_d_n0, assign46290_e45028_d_n2, assign46290_e45028_d_n4,) = {
    if ((var_guard501 == 0.0) && (var_guard502 != 0.0)) {
        let assign46290_e45011: f64 = (p.p0 * p.p2);
        let assign46290_e45014: f64 = (var_cofdsmt0 * (nv2 - nv0));
        let assign46290_e45017: f64 = (var_cofdsmt * p.p28);
        let assign46290_e45020: f64 = ((nv2 - nv0) - p.p27);
        let assign46290_e45022: f64 = (assign46290_e45020 / p.p28);
        let assign46290_e45023: f64 = (assign46290_e45022).exp();
        let assign46290_e45024: f64 = (assign46290_e45017 * assign46290_e45023);
        let assign46290_e45025: f64 = (assign46290_e45014 + assign46290_e45024);
        let assign46290_e45026: f64 = (assign46290_e45011 * assign46290_e45025);
        (assign46290_e45026, (assign46290_e45011 * ((-var_cofdsmt0) + (assign46290_e45017 * (assign46290_e45023 * (-1.0 / p.p28))))), (assign46290_e45011 * (var_cofdsmt0 + (assign46290_e45017 * (assign46290_e45023 * (1.0 / p.p28))))), (assign46290_e45011 * ((var_cofdsmt0_dn4 * (nv2 - nv0)) + ((var_cofdsmt_dn4 * p.p28) * assign46290_e45023))),)
    } else {
        (var_qofds, var_qofds_dn0, var_qofds_dn2, var_qofds_dn4,)
    }
};
        var_qofds = assign46290_e45028;
        var_qofds_dn0 = assign46290_e45028_d_n0;
        var_qofds_dn2 = assign46290_e45028_d_n2;
        var_qofds_dn4 = assign46290_e45028_d_n4;

        let (assign46300_e45056, assign46300_e45056_d_n0, assign46300_e45056_d_n2, assign46300_e45056_d_n4,) = {
    if ((var_guard501 == 0.0) && (var_guard502 == 0.0)) {
        let assign46300_e45036: f64 = (p.p0 * p.p2);
        let assign46300_e45039: f64 = (var_cofdsmt0 * (nv2 - nv0));
        let assign46300_e45042: f64 = (var_cofdsmt * p.p28);
        let assign46300_e45046: f64 = ((nv2 - nv0) - p.p27);
        let assign46300_e45048: f64 = (assign46300_e45046 / p.p28);
        let assign46300_e45049: f64 = (assign46300_e45048).exp();
        let assign46300_e45050: f64 = (1.0 + assign46300_e45049);
        let assign46300_e45051: f64 = (assign46300_e45050).ln();
        let assign46300_e45052: f64 = (assign46300_e45042 * assign46300_e45051);
        let assign46300_e45053: f64 = (assign46300_e45039 + assign46300_e45052);
        let assign46300_e45054: f64 = (assign46300_e45036 * assign46300_e45053);
        (assign46300_e45054, (assign46300_e45036 * ((-var_cofdsmt0) + (assign46300_e45042 * ((assign46300_e45049 * (-1.0 / p.p28)) / assign46300_e45050)))), (assign46300_e45036 * (var_cofdsmt0 + (assign46300_e45042 * ((assign46300_e45049 * (1.0 / p.p28)) / assign46300_e45050)))), (assign46300_e45036 * ((var_cofdsmt0_dn4 * (nv2 - nv0)) + ((var_cofdsmt_dn4 * p.p28) * assign46300_e45051))),)
    } else {
        (var_qofds, var_qofds_dn0, var_qofds_dn2, var_qofds_dn4,)
    }
};
        var_qofds = assign46300_e45056;
        var_qofds_dn0 = assign46300_e45056_d_n0;
        var_qofds_dn2 = assign46300_e45056_d_n2;
        var_qofds_dn4 = assign46300_e45056_d_n4;

        let assign46310_e45059: f64 = ((nv3 - nv2) - p.p27);
        let assign46310_e45061: f64 = (assign46310_e45059 / p.p28);
        let assign46310_e45063: f64 = if assign46310_e45061 > 50.0 { 1.0 } else { 0.0 };
        var_guard503 = assign46310_e45063;

        let (assign46320_e45079, assign46320_e45079_d_n2, assign46320_e45079_d_n3, assign46320_e45079_d_n4,) = {
    if (var_guard503 != 0.0) {
        let assign46320_e45067: f64 = (p.p0 * p.p2);
        let assign46320_e45070: f64 = (var_cofssubmt0 * (nv3 - nv2));
        let assign46320_e45074: f64 = ((nv3 - nv2) - p.p27);
        let assign46320_e45075: f64 = (var_cofssubmt * assign46320_e45074);
        let assign46320_e45076: f64 = (assign46320_e45070 + assign46320_e45075);
        let assign46320_e45077: f64 = (assign46320_e45067 * assign46320_e45076);
        (assign46320_e45077, (assign46320_e45067 * ((-var_cofssubmt0) + (-var_cofssubmt))), (assign46320_e45067 * (var_cofssubmt0 + var_cofssubmt)), (assign46320_e45067 * ((var_cofssubmt0_dn4 * (nv3 - nv2)) + (var_cofssubmt_dn4 * assign46320_e45074))),)
    } else {
        (var_qofssub, var_qofssub_dn2, var_qofssub_dn3, var_qofssub_dn4,)
    }
};
        var_qofssub = assign46320_e45079;
        var_qofssub_dn2 = assign46320_e45079_d_n2;
        var_qofssub_dn3 = assign46320_e45079_d_n3;
        var_qofssub_dn4 = assign46320_e45079_d_n4;

        let assign46330_e45082: f64 = ((nv3 - nv2) - p.p27);
        let assign46330_e45084: f64 = (assign46330_e45082 / p.p28);
        let assign46330_e45086: f64 = (-50.0);
        let assign46330_e45087: f64 = if assign46330_e45084 < assign46330_e45086 { 1.0 } else { 0.0 };
        var_guard504 = assign46330_e45087;

        let (assign46340_e45111, assign46340_e45111_d_n2, assign46340_e45111_d_n3, assign46340_e45111_d_n4,) = {
    if ((var_guard503 == 0.0) && (var_guard504 != 0.0)) {
        let assign46340_e45094: f64 = (p.p0 * p.p2);
        let assign46340_e45097: f64 = (var_cofssubmt0 * (nv3 - nv2));
        let assign46340_e45100: f64 = (var_cofssubmt * p.p28);
        let assign46340_e45103: f64 = ((nv3 - nv2) - p.p27);
        let assign46340_e45105: f64 = (assign46340_e45103 / p.p28);
        let assign46340_e45106: f64 = (assign46340_e45105).exp();
        let assign46340_e45107: f64 = (assign46340_e45100 * assign46340_e45106);
        let assign46340_e45108: f64 = (assign46340_e45097 + assign46340_e45107);
        let assign46340_e45109: f64 = (assign46340_e45094 * assign46340_e45108);
        (assign46340_e45109, (assign46340_e45094 * ((-var_cofssubmt0) + (assign46340_e45100 * (assign46340_e45106 * (-1.0 / p.p28))))), (assign46340_e45094 * (var_cofssubmt0 + (assign46340_e45100 * (assign46340_e45106 * (1.0 / p.p28))))), (assign46340_e45094 * ((var_cofssubmt0_dn4 * (nv3 - nv2)) + ((var_cofssubmt_dn4 * p.p28) * assign46340_e45106))),)
    } else {
        (var_qofssub, var_qofssub_dn2, var_qofssub_dn3, var_qofssub_dn4,)
    }
};
        var_qofssub = assign46340_e45111;
        var_qofssub_dn2 = assign46340_e45111_d_n2;
        var_qofssub_dn3 = assign46340_e45111_d_n3;
        var_qofssub_dn4 = assign46340_e45111_d_n4;

        let (assign46350_e45139, assign46350_e45139_d_n2, assign46350_e45139_d_n3, assign46350_e45139_d_n4,) = {
    if ((var_guard503 == 0.0) && (var_guard504 == 0.0)) {
        let assign46350_e45119: f64 = (p.p0 * p.p2);
        let assign46350_e45122: f64 = (var_cofssubmt0 * (nv3 - nv2));
        let assign46350_e45125: f64 = (var_cofssubmt * p.p28);
        let assign46350_e45129: f64 = ((nv3 - nv2) - p.p27);
        let assign46350_e45131: f64 = (assign46350_e45129 / p.p28);
        let assign46350_e45132: f64 = (assign46350_e45131).exp();
        let assign46350_e45133: f64 = (1.0 + assign46350_e45132);
        let assign46350_e45134: f64 = (assign46350_e45133).ln();
        let assign46350_e45135: f64 = (assign46350_e45125 * assign46350_e45134);
        let assign46350_e45136: f64 = (assign46350_e45122 + assign46350_e45135);
        let assign46350_e45137: f64 = (assign46350_e45119 * assign46350_e45136);
        (assign46350_e45137, (assign46350_e45119 * ((-var_cofssubmt0) + (assign46350_e45125 * ((assign46350_e45132 * (-1.0 / p.p28)) / assign46350_e45133)))), (assign46350_e45119 * (var_cofssubmt0 + (assign46350_e45125 * ((assign46350_e45132 * (1.0 / p.p28)) / assign46350_e45133)))), (assign46350_e45119 * ((var_cofssubmt0_dn4 * (nv3 - nv2)) + ((var_cofssubmt_dn4 * p.p28) * assign46350_e45134))),)
    } else {
        (var_qofssub, var_qofssub_dn2, var_qofssub_dn3, var_qofssub_dn4,)
    }
};
        var_qofssub = assign46350_e45139;
        var_qofssub_dn2 = assign46350_e45139_d_n2;
        var_qofssub_dn3 = assign46350_e45139_d_n3;
        var_qofssub_dn4 = assign46350_e45139_d_n4;

        let assign46360_e45142: f64 = ((nv3 - nv0) - p.p27);
        let assign46360_e45144: f64 = (assign46360_e45142 / p.p28);
        let assign46360_e45146: f64 = if assign46360_e45144 > 50.0 { 1.0 } else { 0.0 };
        var_guard505 = assign46360_e45146;

        let (assign46370_e45162, assign46370_e45162_d_n0, assign46370_e45162_d_n3, assign46370_e45162_d_n4,) = {
    if (var_guard505 != 0.0) {
        let assign46370_e45150: f64 = (p.p0 * p.p2);
        let assign46370_e45153: f64 = (var_cofdsubmt0 * (nv3 - nv0));
        let assign46370_e45157: f64 = ((nv3 - nv0) - p.p27);
        let assign46370_e45158: f64 = (var_cofdsubmt * assign46370_e45157);
        let assign46370_e45159: f64 = (assign46370_e45153 + assign46370_e45158);
        let assign46370_e45160: f64 = (assign46370_e45150 * assign46370_e45159);
        (assign46370_e45160, (assign46370_e45150 * ((-var_cofdsubmt0) + (-var_cofdsubmt))), (assign46370_e45150 * (var_cofdsubmt0 + var_cofdsubmt)), (assign46370_e45150 * ((var_cofdsubmt0_dn4 * (nv3 - nv0)) + (var_cofdsubmt_dn4 * assign46370_e45157))),)
    } else {
        (var_qofdsub, var_qofdsub_dn0, var_qofdsub_dn3, var_qofdsub_dn4,)
    }
};
        var_qofdsub = assign46370_e45162;
        var_qofdsub_dn0 = assign46370_e45162_d_n0;
        var_qofdsub_dn3 = assign46370_e45162_d_n3;
        var_qofdsub_dn4 = assign46370_e45162_d_n4;

        let assign46380_e45165: f64 = ((nv3 - nv0) - p.p27);
        let assign46380_e45167: f64 = (assign46380_e45165 / p.p28);
        let assign46380_e45169: f64 = (-50.0);
        let assign46380_e45170: f64 = if assign46380_e45167 < assign46380_e45169 { 1.0 } else { 0.0 };
        var_guard506 = assign46380_e45170;

        let (assign46390_e45194, assign46390_e45194_d_n0, assign46390_e45194_d_n3, assign46390_e45194_d_n4,) = {
    if ((var_guard505 == 0.0) && (var_guard506 != 0.0)) {
        let assign46390_e45177: f64 = (p.p0 * p.p2);
        let assign46390_e45180: f64 = (var_cofdsubmt0 * (nv3 - nv0));
        let assign46390_e45183: f64 = (var_cofdsubmt * p.p28);
        let assign46390_e45186: f64 = ((nv3 - nv0) - p.p27);
        let assign46390_e45188: f64 = (assign46390_e45186 / p.p28);
        let assign46390_e45189: f64 = (assign46390_e45188).exp();
        let assign46390_e45190: f64 = (assign46390_e45183 * assign46390_e45189);
        let assign46390_e45191: f64 = (assign46390_e45180 + assign46390_e45190);
        let assign46390_e45192: f64 = (assign46390_e45177 * assign46390_e45191);
        (assign46390_e45192, (assign46390_e45177 * ((-var_cofdsubmt0) + (assign46390_e45183 * (assign46390_e45189 * (-1.0 / p.p28))))), (assign46390_e45177 * (var_cofdsubmt0 + (assign46390_e45183 * (assign46390_e45189 * (1.0 / p.p28))))), (assign46390_e45177 * ((var_cofdsubmt0_dn4 * (nv3 - nv0)) + ((var_cofdsubmt_dn4 * p.p28) * assign46390_e45189))),)
    } else {
        (var_qofdsub, var_qofdsub_dn0, var_qofdsub_dn3, var_qofdsub_dn4,)
    }
};
        var_qofdsub = assign46390_e45194;
        var_qofdsub_dn0 = assign46390_e45194_d_n0;
        var_qofdsub_dn3 = assign46390_e45194_d_n3;
        var_qofdsub_dn4 = assign46390_e45194_d_n4;

        *var_fn487_calc_ig__expirev_slot = var_fn487_calc_ig__expirev;
        *var_fn487_calc_ig__expirev_dn0_slot = var_fn487_calc_ig__expirev_dn0;
        *var_fn487_calc_ig__expirev_dn18_slot = var_fn487_calc_ig__expirev_dn18;
        *var_fn487_calc_ig__expirev_dn19_slot = var_fn487_calc_ig__expirev_dn19;
        *var_fn487_calc_ig__expirev_dn2_slot = var_fn487_calc_ig__expirev_dn2;
        *var_fn487_calc_ig__expirev_dn4_slot = var_fn487_calc_ig__expirev_dn4;
        *var_fn487_calc_ig__expirev_dn8_slot = var_fn487_calc_ig__expirev_dn8;
        *var_fn487_calc_ig__expirevarg_slot = var_fn487_calc_ig__expirevarg;
        *var_fn487_calc_ig__expirevarg_dn0_slot = var_fn487_calc_ig__expirevarg_dn0;
        *var_fn487_calc_ig__expirevarg_dn18_slot = var_fn487_calc_ig__expirevarg_dn18;
        *var_fn487_calc_ig__expirevarg_dn19_slot = var_fn487_calc_ig__expirevarg_dn19;
        *var_fn487_calc_ig__expirevarg_dn2_slot = var_fn487_calc_ig__expirevarg_dn2;
        *var_fn487_calc_ig__expirevarg_dn4_slot = var_fn487_calc_ig__expirevarg_dn4;
        *var_fn487_calc_ig__expirevarg_dn8_slot = var_fn487_calc_ig__expirevarg_dn8;
        *var_fn487_calc_ig__iginrec_slot = var_fn487_calc_ig__iginrec;
        *var_fn487_calc_ig__iginrec_dn0_slot = var_fn487_calc_ig__iginrec_dn0;
        *var_fn487_calc_ig__iginrec_dn18_slot = var_fn487_calc_ig__iginrec_dn18;
        *var_fn487_calc_ig__iginrec_dn19_slot = var_fn487_calc_ig__iginrec_dn19;
        *var_fn487_calc_ig__iginrec_dn2_slot = var_fn487_calc_ig__iginrec_dn2;
        *var_fn487_calc_ig__iginrec_dn4_slot = var_fn487_calc_ig__iginrec_dn4;
        *var_fn487_calc_ig__iginrec_dn8_slot = var_fn487_calc_ig__iginrec_dn8;
        *var_fn487_calc_ig__igout_slot = var_fn487_calc_ig__igout;
        *var_fn487_calc_ig__igout_dn0_slot = var_fn487_calc_ig__igout_dn0;
        *var_fn487_calc_ig__igout_dn18_slot = var_fn487_calc_ig__igout_dn18;
        *var_fn487_calc_ig__igout_dn19_slot = var_fn487_calc_ig__igout_dn19;
        *var_fn487_calc_ig__igout_dn2_slot = var_fn487_calc_ig__igout_dn2;
        *var_fn487_calc_ig__igout_dn4_slot = var_fn487_calc_ig__igout_dn4;
        *var_fn487_calc_ig__igout_dn8_slot = var_fn487_calc_ig__igout_dn8;
        *var_fn487_calc_ig__isrecout_slot = var_fn487_calc_ig__isrecout;
        *var_fn487_calc_ig__isrecout_dn4_slot = var_fn487_calc_ig__isrecout_dn4;
        *var_fn487_calc_ig__return_slot = var_fn487_calc_ig__return;
        *var_fn487_calc_ig__return_dn0_slot = var_fn487_calc_ig__return_dn0;
        *var_fn487_calc_ig__return_dn18_slot = var_fn487_calc_ig__return_dn18;
        *var_fn487_calc_ig__return_dn19_slot = var_fn487_calc_ig__return_dn19;
        *var_fn487_calc_ig__return_dn2_slot = var_fn487_calc_ig__return_dn2;
        *var_fn487_calc_ig__return_dn4_slot = var_fn487_calc_ig__return_dn4;
        *var_fn487_calc_ig__return_dn8_slot = var_fn487_calc_ig__return_dn8;
        *var_guard492_slot = var_guard492;
        *var_guard493_slot = var_guard493;
        *var_guard494_slot = var_guard494;
        *var_guard497_slot = var_guard497;
        *var_guard498_slot = var_guard498;
        *var_guard499_slot = var_guard499;
        *var_guard500_slot = var_guard500;
        *var_guard501_slot = var_guard501;
        *var_guard502_slot = var_guard502;
        *var_guard503_slot = var_guard503;
        *var_guard504_slot = var_guard504;
        *var_guard505_slot = var_guard505;
        *var_guard506_slot = var_guard506;
        *var_igdcbd_slot = var_igdcbd;
        *var_igdcbd_dn0_slot = var_igdcbd_dn0;
        *var_igdcbd_dn18_slot = var_igdcbd_dn18;
        *var_igdcbd_dn19_slot = var_igdcbd_dn19;
        *var_igdcbd_dn2_slot = var_igdcbd_dn2;
        *var_igdcbd_dn4_slot = var_igdcbd_dn4;
        *var_igdcbd_dn8_slot = var_igdcbd_dn8;
        *var_qofd_slot = var_qofd;
        *var_qofd_dn0_slot = var_qofd_dn0;
        *var_qofd_dn4_slot = var_qofd_dn4;
        *var_qofd_dn6_slot = var_qofd_dn6;
        *var_qofds_slot = var_qofds;
        *var_qofds_dn0_slot = var_qofds_dn0;
        *var_qofds_dn2_slot = var_qofds_dn2;
        *var_qofds_dn4_slot = var_qofds_dn4;
        *var_qofdsub_slot = var_qofdsub;
        *var_qofdsub_dn0_slot = var_qofdsub_dn0;
        *var_qofdsub_dn3_slot = var_qofdsub_dn3;
        *var_qofdsub_dn4_slot = var_qofdsub_dn4;
        *var_qofs_slot = var_qofs;
        *var_qofs_dn2_slot = var_qofs_dn2;
        *var_qofs_dn4_slot = var_qofs_dn4;
        *var_qofs_dn6_slot = var_qofs_dn6;
        *var_qofssub_slot = var_qofssub;
        *var_qofssub_dn2_slot = var_qofssub_dn2;
        *var_qofssub_dn3_slot = var_qofssub_dn3;
        *var_qofssub_dn4_slot = var_qofssub_dn4;
    }

    pub(super) fn stamp_transient_block_117(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_cofdsubmt: f64,
        var_cofdsubmt0: f64,
        var_cofdsubmt0_dn4: f64,
        var_cofdsubmt_dn4: f64,
        var_cofgsubmt: f64,
        var_cofgsubmt0: f64,
        var_cofgsubmt0_dn4: f64,
        var_cofgsubmt_dn4: f64,
        var_guard505: f64,
        var_guard506: f64,
        var_ids: f64,
        var_ids_dn22: f64,
        var_ids_dn23: f64,
        var_ids_dn25: f64,
        var_ids_dn26: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn8: f64,
        var_ids_dn9: f64,
        var_idsfp1: f64,
        var_idsfp1_dn14: f64,
        var_idsfp1_dn2: f64,
        var_idsfp1_dn3: f64,
        var_idsfp1_dn4: f64,
        var_idsfp1_dn5: f64,
        var_idsfp1_dn7: f64,
        var_idsfp2: f64,
        var_idsfp2_dn14: f64,
        var_idsfp2_dn15: f64,
        var_idsfp2_dn2: f64,
        var_idsfp2_dn3: f64,
        var_idsfp2_dn4: f64,
        var_idsfp2_dn7: f64,
        var_idsfp3: f64,
        var_idsfp3_dn15: f64,
        var_idsfp3_dn16: f64,
        var_idsfp3_dn2: f64,
        var_idsfp3_dn3: f64,
        var_idsfp3_dn4: f64,
        var_idsfp3_dn7: f64,
        var_idsfp4: f64,
        var_idsfp4_dn16: f64,
        var_idsfp4_dn17: f64,
        var_idsfp4_dn2: f64,
        var_idsfp4_dn3: f64,
        var_idsfp4_dn4: f64,
        var_idsfp4_dn7: f64,
        var_idsfps1: f64,
        var_idsfps1_dn10: f64,
        var_idsfps1_dn2: f64,
        var_idsfps1_dn3: f64,
        var_idsfps1_dn4: f64,
        var_idsfps1_dn7: f64,
        var_idsfps1_dn9: f64,
        var_idsfps2: f64,
        var_idsfps2_dn10: f64,
        var_idsfps2_dn11: f64,
        var_idsfps2_dn2: f64,
        var_idsfps2_dn3: f64,
        var_idsfps2_dn4: f64,
        var_idsfps2_dn7: f64,
        var_idsfps3: f64,
        var_idsfps3_dn11: f64,
        var_idsfps3_dn12: f64,
        var_idsfps3_dn2: f64,
        var_idsfps3_dn3: f64,
        var_idsfps3_dn4: f64,
        var_idsfps3_dn7: f64,
        var_idsfps4: f64,
        var_idsfps4_dn12: f64,
        var_idsfps4_dn13: f64,
        var_idsfps4_dn2: f64,
        var_idsfps4_dn3: f64,
        var_idsfps4_dn4: f64,
        var_idsfps4_dn7: f64,
        var_idsrd: f64,
        var_idsrd_dn0: f64,
        var_idsrd_dn17: f64,
        var_idsrd_dn18: f64,
        var_idsrd_dn2: f64,
        var_idsrd_dn20: f64,
        var_idsrd_dn4: f64,
        var_idsrs: f64,
        var_idsrs_dn0: f64,
        var_idsrs_dn13: f64,
        var_idsrs_dn19: f64,
        var_idsrs_dn2: f64,
        var_idsrs_dn4: f64,
        var_rcd_w: f64,
        var_rcs_w: f64,
        var_rdi: f64,
        var_rdi_dn4: f64,
        var_rsi: f64,
        var_rsi_dn4: f64,
        var_guard507_slot: &mut f64,
        var_guard508_slot: &mut f64,
        var_guard521_slot: &mut f64,
        var_guard522_slot: &mut f64,
        var_guard523_slot: &mut f64,
        var_pdiss_slot: &mut f64,
        var_pdiss_dn0_slot: &mut f64,
        var_pdiss_dn10_slot: &mut f64,
        var_pdiss_dn11_slot: &mut f64,
        var_pdiss_dn12_slot: &mut f64,
        var_pdiss_dn13_slot: &mut f64,
        var_pdiss_dn14_slot: &mut f64,
        var_pdiss_dn15_slot: &mut f64,
        var_pdiss_dn16_slot: &mut f64,
        var_pdiss_dn17_slot: &mut f64,
        var_pdiss_dn18_slot: &mut f64,
        var_pdiss_dn19_slot: &mut f64,
        var_pdiss_dn2_slot: &mut f64,
        var_pdiss_dn20_slot: &mut f64,
        var_pdiss_dn22_slot: &mut f64,
        var_pdiss_dn23_slot: &mut f64,
        var_pdiss_dn25_slot: &mut f64,
        var_pdiss_dn26_slot: &mut f64,
        var_pdiss_dn3_slot: &mut f64,
        var_pdiss_dn4_slot: &mut f64,
        var_pdiss_dn5_slot: &mut f64,
        var_pdiss_dn7_slot: &mut f64,
        var_pdiss_dn8_slot: &mut f64,
        var_pdiss_dn9_slot: &mut f64,
        var_qofdsub_slot: &mut f64,
        var_qofdsub_dn0_slot: &mut f64,
        var_qofdsub_dn3_slot: &mut f64,
        var_qofdsub_dn4_slot: &mut f64,
        var_qofgsub_slot: &mut f64,
        var_qofgsub_dn3_slot: &mut f64,
        var_qofgsub_dn4_slot: &mut f64,
        var_qofgsub_dn6_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let nv19 = ctx.node_voltage(nodes[19]);
        let mut var_guard507: f64 = *var_guard507_slot;
        let mut var_guard508: f64 = *var_guard508_slot;
        let mut var_guard521: f64 = *var_guard521_slot;
        let mut var_guard522: f64 = *var_guard522_slot;
        let mut var_guard523: f64 = *var_guard523_slot;
        let mut var_pdiss: f64 = *var_pdiss_slot;
        let mut var_pdiss_dn0: f64 = *var_pdiss_dn0_slot;
        let mut var_pdiss_dn10: f64 = *var_pdiss_dn10_slot;
        let mut var_pdiss_dn11: f64 = *var_pdiss_dn11_slot;
        let mut var_pdiss_dn12: f64 = *var_pdiss_dn12_slot;
        let mut var_pdiss_dn13: f64 = *var_pdiss_dn13_slot;
        let mut var_pdiss_dn14: f64 = *var_pdiss_dn14_slot;
        let mut var_pdiss_dn15: f64 = *var_pdiss_dn15_slot;
        let mut var_pdiss_dn16: f64 = *var_pdiss_dn16_slot;
        let mut var_pdiss_dn17: f64 = *var_pdiss_dn17_slot;
        let mut var_pdiss_dn18: f64 = *var_pdiss_dn18_slot;
        let mut var_pdiss_dn19: f64 = *var_pdiss_dn19_slot;
        let mut var_pdiss_dn2: f64 = *var_pdiss_dn2_slot;
        let mut var_pdiss_dn20: f64 = *var_pdiss_dn20_slot;
        let mut var_pdiss_dn22: f64 = *var_pdiss_dn22_slot;
        let mut var_pdiss_dn23: f64 = *var_pdiss_dn23_slot;
        let mut var_pdiss_dn25: f64 = *var_pdiss_dn25_slot;
        let mut var_pdiss_dn26: f64 = *var_pdiss_dn26_slot;
        let mut var_pdiss_dn3: f64 = *var_pdiss_dn3_slot;
        let mut var_pdiss_dn4: f64 = *var_pdiss_dn4_slot;
        let mut var_pdiss_dn5: f64 = *var_pdiss_dn5_slot;
        let mut var_pdiss_dn7: f64 = *var_pdiss_dn7_slot;
        let mut var_pdiss_dn8: f64 = *var_pdiss_dn8_slot;
        let mut var_pdiss_dn9: f64 = *var_pdiss_dn9_slot;
        let mut var_qofdsub: f64 = *var_qofdsub_slot;
        let mut var_qofdsub_dn0: f64 = *var_qofdsub_dn0_slot;
        let mut var_qofdsub_dn3: f64 = *var_qofdsub_dn3_slot;
        let mut var_qofdsub_dn4: f64 = *var_qofdsub_dn4_slot;
        let mut var_qofgsub: f64 = *var_qofgsub_slot;
        let mut var_qofgsub_dn3: f64 = *var_qofgsub_dn3_slot;
        let mut var_qofgsub_dn4: f64 = *var_qofgsub_dn4_slot;
        let mut var_qofgsub_dn6: f64 = *var_qofgsub_dn6_slot;

        let (assign46400_e45222, assign46400_e45222_d_n0, assign46400_e45222_d_n3, assign46400_e45222_d_n4,) = {
    if ((var_guard505 == 0.0) && (var_guard506 == 0.0)) {
        let assign46400_e45202: f64 = (p.p0 * p.p2);
        let assign46400_e45205: f64 = (var_cofdsubmt0 * (nv3 - nv0));
        let assign46400_e45208: f64 = (var_cofdsubmt * p.p28);
        let assign46400_e45212: f64 = ((nv3 - nv0) - p.p27);
        let assign46400_e45214: f64 = (assign46400_e45212 / p.p28);
        let assign46400_e45215: f64 = (assign46400_e45214).exp();
        let assign46400_e45216: f64 = (1.0 + assign46400_e45215);
        let assign46400_e45217: f64 = (assign46400_e45216).ln();
        let assign46400_e45218: f64 = (assign46400_e45208 * assign46400_e45217);
        let assign46400_e45219: f64 = (assign46400_e45205 + assign46400_e45218);
        let assign46400_e45220: f64 = (assign46400_e45202 * assign46400_e45219);
        (assign46400_e45220, (assign46400_e45202 * ((-var_cofdsubmt0) + (assign46400_e45208 * ((assign46400_e45215 * (-1.0 / p.p28)) / assign46400_e45216)))), (assign46400_e45202 * (var_cofdsubmt0 + (assign46400_e45208 * ((assign46400_e45215 * (1.0 / p.p28)) / assign46400_e45216)))), (assign46400_e45202 * ((var_cofdsubmt0_dn4 * (nv3 - nv0)) + ((var_cofdsubmt_dn4 * p.p28) * assign46400_e45217))),)
    } else {
        (var_qofdsub, var_qofdsub_dn0, var_qofdsub_dn3, var_qofdsub_dn4,)
    }
};
        var_qofdsub = assign46400_e45222;
        var_qofdsub_dn0 = assign46400_e45222_d_n0;
        var_qofdsub_dn3 = assign46400_e45222_d_n3;
        var_qofdsub_dn4 = assign46400_e45222_d_n4;

        let assign46410_e45225: f64 = ((nv6 - nv3) - p.p27);
        let assign46410_e45227: f64 = (assign46410_e45225 / p.p28);
        let assign46410_e45229: f64 = if assign46410_e45227 > 50.0 { 1.0 } else { 0.0 };
        var_guard507 = assign46410_e45229;

        let (assign46420_e45245, assign46420_e45245_d_n3, assign46420_e45245_d_n4, assign46420_e45245_d_n6,) = {
    if (var_guard507 != 0.0) {
        let assign46420_e45233: f64 = (p.p0 * p.p2);
        let assign46420_e45236: f64 = (var_cofgsubmt0 * (nv6 - nv3));
        let assign46420_e45240: f64 = ((nv6 - nv3) - p.p27);
        let assign46420_e45241: f64 = (var_cofgsubmt * assign46420_e45240);
        let assign46420_e45242: f64 = (assign46420_e45236 + assign46420_e45241);
        let assign46420_e45243: f64 = (assign46420_e45233 * assign46420_e45242);
        (assign46420_e45243, (assign46420_e45233 * ((-var_cofgsubmt0) + (-var_cofgsubmt))), (assign46420_e45233 * ((var_cofgsubmt0_dn4 * (nv6 - nv3)) + (var_cofgsubmt_dn4 * assign46420_e45240))), (assign46420_e45233 * (var_cofgsubmt0 + var_cofgsubmt)),)
    } else {
        (var_qofgsub, var_qofgsub_dn3, var_qofgsub_dn4, var_qofgsub_dn6,)
    }
};
        var_qofgsub = assign46420_e45245;
        var_qofgsub_dn3 = assign46420_e45245_d_n3;
        var_qofgsub_dn4 = assign46420_e45245_d_n4;
        var_qofgsub_dn6 = assign46420_e45245_d_n6;

        let assign46430_e45248: f64 = ((nv6 - nv3) - p.p27);
        let assign46430_e45250: f64 = (assign46430_e45248 / p.p28);
        let assign46430_e45252: f64 = (-50.0);
        let assign46430_e45253: f64 = if assign46430_e45250 < assign46430_e45252 { 1.0 } else { 0.0 };
        var_guard508 = assign46430_e45253;

        let (assign46440_e45277, assign46440_e45277_d_n3, assign46440_e45277_d_n4, assign46440_e45277_d_n6,) = {
    if ((var_guard507 == 0.0) && (var_guard508 != 0.0)) {
        let assign46440_e45260: f64 = (p.p0 * p.p2);
        let assign46440_e45263: f64 = (var_cofgsubmt0 * (nv6 - nv3));
        let assign46440_e45266: f64 = (var_cofgsubmt * p.p28);
        let assign46440_e45269: f64 = ((nv6 - nv3) - p.p27);
        let assign46440_e45271: f64 = (assign46440_e45269 / p.p28);
        let assign46440_e45272: f64 = (assign46440_e45271).exp();
        let assign46440_e45273: f64 = (assign46440_e45266 * assign46440_e45272);
        let assign46440_e45274: f64 = (assign46440_e45263 + assign46440_e45273);
        let assign46440_e45275: f64 = (assign46440_e45260 * assign46440_e45274);
        (assign46440_e45275, (assign46440_e45260 * ((-var_cofgsubmt0) + (assign46440_e45266 * (assign46440_e45272 * (-1.0 / p.p28))))), (assign46440_e45260 * ((var_cofgsubmt0_dn4 * (nv6 - nv3)) + ((var_cofgsubmt_dn4 * p.p28) * assign46440_e45272))), (assign46440_e45260 * (var_cofgsubmt0 + (assign46440_e45266 * (assign46440_e45272 * (1.0 / p.p28))))),)
    } else {
        (var_qofgsub, var_qofgsub_dn3, var_qofgsub_dn4, var_qofgsub_dn6,)
    }
};
        var_qofgsub = assign46440_e45277;
        var_qofgsub_dn3 = assign46440_e45277_d_n3;
        var_qofgsub_dn4 = assign46440_e45277_d_n4;
        var_qofgsub_dn6 = assign46440_e45277_d_n6;

        let (assign46450_e45305, assign46450_e45305_d_n3, assign46450_e45305_d_n4, assign46450_e45305_d_n6,) = {
    if ((var_guard507 == 0.0) && (var_guard508 == 0.0)) {
        let assign46450_e45285: f64 = (p.p0 * p.p2);
        let assign46450_e45288: f64 = (var_cofgsubmt0 * (nv6 - nv3));
        let assign46450_e45291: f64 = (var_cofgsubmt * p.p28);
        let assign46450_e45295: f64 = ((nv6 - nv3) - p.p27);
        let assign46450_e45297: f64 = (assign46450_e45295 / p.p28);
        let assign46450_e45298: f64 = (assign46450_e45297).exp();
        let assign46450_e45299: f64 = (1.0 + assign46450_e45298);
        let assign46450_e45300: f64 = (assign46450_e45299).ln();
        let assign46450_e45301: f64 = (assign46450_e45291 * assign46450_e45300);
        let assign46450_e45302: f64 = (assign46450_e45288 + assign46450_e45301);
        let assign46450_e45303: f64 = (assign46450_e45285 * assign46450_e45302);
        (assign46450_e45303, (assign46450_e45285 * ((-var_cofgsubmt0) + (assign46450_e45291 * ((assign46450_e45298 * (-1.0 / p.p28)) / assign46450_e45299)))), (assign46450_e45285 * ((var_cofgsubmt0_dn4 * (nv6 - nv3)) + ((var_cofgsubmt_dn4 * p.p28) * assign46450_e45300))), (assign46450_e45285 * (var_cofgsubmt0 + (assign46450_e45291 * ((assign46450_e45298 * (1.0 / p.p28)) / assign46450_e45299)))),)
    } else {
        (var_qofgsub, var_qofgsub_dn3, var_qofgsub_dn4, var_qofgsub_dn6,)
    }
};
        var_qofgsub = assign46450_e45305;
        var_qofgsub_dn3 = assign46450_e45305_d_n3;
        var_qofgsub_dn4 = assign46450_e45305_d_n4;
        var_qofgsub_dn6 = assign46450_e45305_d_n6;

        let assign46640_e45442: f64 = (var_ids * (nv5 - nv9));
        let assign46640_e45445: f64 = (var_idsrd * (nv18 - nv17));
        let assign46640_e45446: f64 = (assign46640_e45442 + assign46640_e45445);
        let assign46640_e45449: f64 = (var_idsrs * (nv13 - nv19));
        let assign46640_e45450: f64 = (assign46640_e45446 + assign46640_e45449);
        let assign46640_e45453: f64 = (var_idsfps4 * (nv12 - nv13));
        let assign46640_e45454: f64 = (assign46640_e45450 + assign46640_e45453);
        let assign46640_e45457: f64 = (var_idsfps3 * (nv11 - nv12));
        let assign46640_e45458: f64 = (assign46640_e45454 + assign46640_e45457);
        let assign46640_e45461: f64 = (var_idsfps2 * (nv10 - nv11));
        let assign46640_e45462: f64 = (assign46640_e45458 + assign46640_e45461);
        let assign46640_e45465: f64 = (var_idsfps1 * (nv9 - nv10));
        let assign46640_e45466: f64 = (assign46640_e45462 + assign46640_e45465);
        let assign46640_e45469: f64 = (var_idsfp1 * (nv14 - nv5));
        let assign46640_e45470: f64 = (assign46640_e45466 + assign46640_e45469);
        let assign46640_e45473: f64 = (var_idsfp2 * (nv15 - nv14));
        let assign46640_e45474: f64 = (assign46640_e45470 + assign46640_e45473);
        let assign46640_e45477: f64 = (var_idsfp3 * (nv16 - nv15));
        let assign46640_e45478: f64 = (assign46640_e45474 + assign46640_e45477);
        let assign46640_e45481: f64 = (var_idsfp4 * (nv17 - nv16));
        let assign46640_e45482: f64 = (assign46640_e45478 + assign46640_e45481);
        var_pdiss = assign46640_e45482;
        var_pdiss_dn0 = ((var_idsrd_dn0 * (nv18 - nv17)) + (var_idsrs_dn0 * (nv13 - nv19)));
        var_pdiss_dn2 = ((((((((((var_idsrd_dn2 * (nv18 - nv17)) + (var_idsrs_dn2 * (nv13 - nv19))) + (var_idsfps4_dn2 * (nv12 - nv13))) + (var_idsfps3_dn2 * (nv11 - nv12))) + (var_idsfps2_dn2 * (nv10 - nv11))) + (var_idsfps1_dn2 * (nv9 - nv10))) + (var_idsfp1_dn2 * (nv14 - nv5))) + (var_idsfp2_dn2 * (nv15 - nv14))) + (var_idsfp3_dn2 * (nv16 - nv15))) + (var_idsfp4_dn2 * (nv17 - nv16)));
        var_pdiss_dn3 = ((((((((var_idsfps4_dn3 * (nv12 - nv13)) + (var_idsfps3_dn3 * (nv11 - nv12))) + (var_idsfps2_dn3 * (nv10 - nv11))) + (var_idsfps1_dn3 * (nv9 - nv10))) + (var_idsfp1_dn3 * (nv14 - nv5))) + (var_idsfp2_dn3 * (nv15 - nv14))) + (var_idsfp3_dn3 * (nv16 - nv15))) + (var_idsfp4_dn3 * (nv17 - nv16)));
        var_pdiss_dn4 = (((((((((((var_ids_dn4 * (nv5 - nv9)) + (var_idsrd_dn4 * (nv18 - nv17))) + (var_idsrs_dn4 * (nv13 - nv19))) + (var_idsfps4_dn4 * (nv12 - nv13))) + (var_idsfps3_dn4 * (nv11 - nv12))) + (var_idsfps2_dn4 * (nv10 - nv11))) + (var_idsfps1_dn4 * (nv9 - nv10))) + (var_idsfp1_dn4 * (nv14 - nv5))) + (var_idsfp2_dn4 * (nv15 - nv14))) + (var_idsfp3_dn4 * (nv16 - nv15))) + (var_idsfp4_dn4 * (nv17 - nv16)));
        var_pdiss_dn5 = (((var_ids_dn5 * (nv5 - nv9)) + var_ids) + ((var_idsfp1_dn5 * (nv14 - nv5)) + (-var_idsfp1)));
        var_pdiss_dn7 = ((((((((var_idsfps4_dn7 * (nv12 - nv13)) + (var_idsfps3_dn7 * (nv11 - nv12))) + (var_idsfps2_dn7 * (nv10 - nv11))) + (var_idsfps1_dn7 * (nv9 - nv10))) + (var_idsfp1_dn7 * (nv14 - nv5))) + (var_idsfp2_dn7 * (nv15 - nv14))) + (var_idsfp3_dn7 * (nv16 - nv15))) + (var_idsfp4_dn7 * (nv17 - nv16)));
        var_pdiss_dn8 = (var_ids_dn8 * (nv5 - nv9));
        var_pdiss_dn9 = (((var_ids_dn9 * (nv5 - nv9)) + (-var_ids)) + ((var_idsfps1_dn9 * (nv9 - nv10)) + var_idsfps1));
        var_pdiss_dn10 = (((var_idsfps2_dn10 * (nv10 - nv11)) + var_idsfps2) + ((var_idsfps1_dn10 * (nv9 - nv10)) + (-var_idsfps1)));
        var_pdiss_dn11 = (((var_idsfps3_dn11 * (nv11 - nv12)) + var_idsfps3) + ((var_idsfps2_dn11 * (nv10 - nv11)) + (-var_idsfps2)));
        var_pdiss_dn12 = (((var_idsfps4_dn12 * (nv12 - nv13)) + var_idsfps4) + ((var_idsfps3_dn12 * (nv11 - nv12)) + (-var_idsfps3)));
        var_pdiss_dn13 = (((var_idsrs_dn13 * (nv13 - nv19)) + var_idsrs) + ((var_idsfps4_dn13 * (nv12 - nv13)) + (-var_idsfps4)));
        var_pdiss_dn14 = (((var_idsfp1_dn14 * (nv14 - nv5)) + var_idsfp1) + ((var_idsfp2_dn14 * (nv15 - nv14)) + (-var_idsfp2)));
        var_pdiss_dn15 = (((var_idsfp2_dn15 * (nv15 - nv14)) + var_idsfp2) + ((var_idsfp3_dn15 * (nv16 - nv15)) + (-var_idsfp3)));
        var_pdiss_dn16 = (((var_idsfp3_dn16 * (nv16 - nv15)) + var_idsfp3) + ((var_idsfp4_dn16 * (nv17 - nv16)) + (-var_idsfp4)));
        var_pdiss_dn17 = (((var_idsrd_dn17 * (nv18 - nv17)) + (-var_idsrd)) + ((var_idsfp4_dn17 * (nv17 - nv16)) + var_idsfp4));
        var_pdiss_dn18 = ((var_idsrd_dn18 * (nv18 - nv17)) + var_idsrd);
        var_pdiss_dn19 = ((var_idsrs_dn19 * (nv13 - nv19)) + (-var_idsrs));
        var_pdiss_dn20 = (var_idsrd_dn20 * (nv18 - nv17));
        var_pdiss_dn22 = (var_ids_dn22 * (nv5 - nv9));
        var_pdiss_dn23 = (var_ids_dn23 * (nv5 - nv9));
        var_pdiss_dn25 = (var_ids_dn25 * (nv5 - nv9));
        var_pdiss_dn26 = (var_ids_dn26 * (nv5 - nv9));

        let assign46650_e45489: f64 = if ((var_rcd_w >= p.p353) && (var_rcd_w > 0.0)) { 1.0 } else { 0.0 };
        var_guard521 = assign46650_e45489;

        let (assign46660_e45499, assign46660_e45499_d_n0, assign46660_e45499_d_n2, assign46660_e45499_d_n3, assign46660_e45499_d_n4, assign46660_e45499_d_n5, assign46660_e45499_d_n7, assign46660_e45499_d_n8, assign46660_e45499_d_n9, assign46660_e45499_d_n10, assign46660_e45499_d_n11, assign46660_e45499_d_n12, assign46660_e45499_d_n13, assign46660_e45499_d_n14, assign46660_e45499_d_n15, assign46660_e45499_d_n16, assign46660_e45499_d_n17, assign46660_e45499_d_n18, assign46660_e45499_d_n19, assign46660_e45499_d_n20, assign46660_e45499_d_n22, assign46660_e45499_d_n23, assign46660_e45499_d_n25, assign46660_e45499_d_n26,) = {
    if (var_guard521 != 0.0) {
        let assign46660_e45494: f64 = ((nv18 - nv0) * (nv18 - nv0));
        let assign46660_e45496: f64 = (assign46660_e45494 / var_rdi);
        let assign46660_e45497: f64 = (var_pdiss + assign46660_e45496);
        (assign46660_e45497, (var_pdiss_dn0 + (((-(nv18 - nv0)) + (-(nv18 - nv0))) / var_rdi)), var_pdiss_dn2, var_pdiss_dn3, (var_pdiss_dn4 + (-((assign46660_e45494 * var_rdi_dn4) / (var_rdi * var_rdi)))), var_pdiss_dn5, var_pdiss_dn7, var_pdiss_dn8, var_pdiss_dn9, var_pdiss_dn10, var_pdiss_dn11, var_pdiss_dn12, var_pdiss_dn13, var_pdiss_dn14, var_pdiss_dn15, var_pdiss_dn16, var_pdiss_dn17, (var_pdiss_dn18 + (((nv18 - nv0) + (nv18 - nv0)) / var_rdi)), var_pdiss_dn19, var_pdiss_dn20, var_pdiss_dn22, var_pdiss_dn23, var_pdiss_dn25, var_pdiss_dn26,)
    } else {
        (var_pdiss, var_pdiss_dn0, var_pdiss_dn2, var_pdiss_dn3, var_pdiss_dn4, var_pdiss_dn5, var_pdiss_dn7, var_pdiss_dn8, var_pdiss_dn9, var_pdiss_dn10, var_pdiss_dn11, var_pdiss_dn12, var_pdiss_dn13, var_pdiss_dn14, var_pdiss_dn15, var_pdiss_dn16, var_pdiss_dn17, var_pdiss_dn18, var_pdiss_dn19, var_pdiss_dn20, var_pdiss_dn22, var_pdiss_dn23, var_pdiss_dn25, var_pdiss_dn26,)
    }
};
        var_pdiss = assign46660_e45499;
        var_pdiss_dn0 = assign46660_e45499_d_n0;
        var_pdiss_dn2 = assign46660_e45499_d_n2;
        var_pdiss_dn3 = assign46660_e45499_d_n3;
        var_pdiss_dn4 = assign46660_e45499_d_n4;
        var_pdiss_dn5 = assign46660_e45499_d_n5;
        var_pdiss_dn7 = assign46660_e45499_d_n7;
        var_pdiss_dn8 = assign46660_e45499_d_n8;
        var_pdiss_dn9 = assign46660_e45499_d_n9;
        var_pdiss_dn10 = assign46660_e45499_d_n10;
        var_pdiss_dn11 = assign46660_e45499_d_n11;
        var_pdiss_dn12 = assign46660_e45499_d_n12;
        var_pdiss_dn13 = assign46660_e45499_d_n13;
        var_pdiss_dn14 = assign46660_e45499_d_n14;
        var_pdiss_dn15 = assign46660_e45499_d_n15;
        var_pdiss_dn16 = assign46660_e45499_d_n16;
        var_pdiss_dn17 = assign46660_e45499_d_n17;
        var_pdiss_dn18 = assign46660_e45499_d_n18;
        var_pdiss_dn19 = assign46660_e45499_d_n19;
        var_pdiss_dn20 = assign46660_e45499_d_n20;
        var_pdiss_dn22 = assign46660_e45499_d_n22;
        var_pdiss_dn23 = assign46660_e45499_d_n23;
        var_pdiss_dn25 = assign46660_e45499_d_n25;
        var_pdiss_dn26 = assign46660_e45499_d_n26;

        let assign46670_e45506: f64 = if ((var_rcs_w >= p.p353) && (var_rcs_w > 0.0)) { 1.0 } else { 0.0 };
        var_guard522 = assign46670_e45506;

        let (assign46680_e45516, assign46680_e45516_d_n0, assign46680_e45516_d_n2, assign46680_e45516_d_n3, assign46680_e45516_d_n4, assign46680_e45516_d_n5, assign46680_e45516_d_n7, assign46680_e45516_d_n8, assign46680_e45516_d_n9, assign46680_e45516_d_n10, assign46680_e45516_d_n11, assign46680_e45516_d_n12, assign46680_e45516_d_n13, assign46680_e45516_d_n14, assign46680_e45516_d_n15, assign46680_e45516_d_n16, assign46680_e45516_d_n17, assign46680_e45516_d_n18, assign46680_e45516_d_n19, assign46680_e45516_d_n20, assign46680_e45516_d_n22, assign46680_e45516_d_n23, assign46680_e45516_d_n25, assign46680_e45516_d_n26,) = {
    if (var_guard522 != 0.0) {
        let assign46680_e45511: f64 = ((nv19 - nv2) * (nv19 - nv2));
        let assign46680_e45513: f64 = (assign46680_e45511 / var_rsi);
        let assign46680_e45514: f64 = (var_pdiss + assign46680_e45513);
        (assign46680_e45514, var_pdiss_dn0, (var_pdiss_dn2 + (((-(nv19 - nv2)) + (-(nv19 - nv2))) / var_rsi)), var_pdiss_dn3, (var_pdiss_dn4 + (-((assign46680_e45511 * var_rsi_dn4) / (var_rsi * var_rsi)))), var_pdiss_dn5, var_pdiss_dn7, var_pdiss_dn8, var_pdiss_dn9, var_pdiss_dn10, var_pdiss_dn11, var_pdiss_dn12, var_pdiss_dn13, var_pdiss_dn14, var_pdiss_dn15, var_pdiss_dn16, var_pdiss_dn17, var_pdiss_dn18, (var_pdiss_dn19 + (((nv19 - nv2) + (nv19 - nv2)) / var_rsi)), var_pdiss_dn20, var_pdiss_dn22, var_pdiss_dn23, var_pdiss_dn25, var_pdiss_dn26,)
    } else {
        (var_pdiss, var_pdiss_dn0, var_pdiss_dn2, var_pdiss_dn3, var_pdiss_dn4, var_pdiss_dn5, var_pdiss_dn7, var_pdiss_dn8, var_pdiss_dn9, var_pdiss_dn10, var_pdiss_dn11, var_pdiss_dn12, var_pdiss_dn13, var_pdiss_dn14, var_pdiss_dn15, var_pdiss_dn16, var_pdiss_dn17, var_pdiss_dn18, var_pdiss_dn19, var_pdiss_dn20, var_pdiss_dn22, var_pdiss_dn23, var_pdiss_dn25, var_pdiss_dn26,)
    }
};
        var_pdiss = assign46680_e45516;
        var_pdiss_dn0 = assign46680_e45516_d_n0;
        var_pdiss_dn2 = assign46680_e45516_d_n2;
        var_pdiss_dn3 = assign46680_e45516_d_n3;
        var_pdiss_dn4 = assign46680_e45516_d_n4;
        var_pdiss_dn5 = assign46680_e45516_d_n5;
        var_pdiss_dn7 = assign46680_e45516_d_n7;
        var_pdiss_dn8 = assign46680_e45516_d_n8;
        var_pdiss_dn9 = assign46680_e45516_d_n9;
        var_pdiss_dn10 = assign46680_e45516_d_n10;
        var_pdiss_dn11 = assign46680_e45516_d_n11;
        var_pdiss_dn12 = assign46680_e45516_d_n12;
        var_pdiss_dn13 = assign46680_e45516_d_n13;
        var_pdiss_dn14 = assign46680_e45516_d_n14;
        var_pdiss_dn15 = assign46680_e45516_d_n15;
        var_pdiss_dn16 = assign46680_e45516_d_n16;
        var_pdiss_dn17 = assign46680_e45516_d_n17;
        var_pdiss_dn18 = assign46680_e45516_d_n18;
        var_pdiss_dn19 = assign46680_e45516_d_n19;
        var_pdiss_dn20 = assign46680_e45516_d_n20;
        var_pdiss_dn22 = assign46680_e45516_d_n22;
        var_pdiss_dn23 = assign46680_e45516_d_n23;
        var_pdiss_dn25 = assign46680_e45516_d_n25;
        var_pdiss_dn26 = assign46680_e45516_d_n26;

        let assign46690_e45519: f64 = if p.p320 > 0.0 { 1.0 } else { 0.0 };
        var_guard523 = assign46690_e45519;

        *var_guard507_slot = var_guard507;
        *var_guard508_slot = var_guard508;
        *var_guard521_slot = var_guard521;
        *var_guard522_slot = var_guard522;
        *var_guard523_slot = var_guard523;
        *var_pdiss_slot = var_pdiss;
        *var_pdiss_dn0_slot = var_pdiss_dn0;
        *var_pdiss_dn10_slot = var_pdiss_dn10;
        *var_pdiss_dn11_slot = var_pdiss_dn11;
        *var_pdiss_dn12_slot = var_pdiss_dn12;
        *var_pdiss_dn13_slot = var_pdiss_dn13;
        *var_pdiss_dn14_slot = var_pdiss_dn14;
        *var_pdiss_dn15_slot = var_pdiss_dn15;
        *var_pdiss_dn16_slot = var_pdiss_dn16;
        *var_pdiss_dn17_slot = var_pdiss_dn17;
        *var_pdiss_dn18_slot = var_pdiss_dn18;
        *var_pdiss_dn19_slot = var_pdiss_dn19;
        *var_pdiss_dn2_slot = var_pdiss_dn2;
        *var_pdiss_dn20_slot = var_pdiss_dn20;
        *var_pdiss_dn22_slot = var_pdiss_dn22;
        *var_pdiss_dn23_slot = var_pdiss_dn23;
        *var_pdiss_dn25_slot = var_pdiss_dn25;
        *var_pdiss_dn26_slot = var_pdiss_dn26;
        *var_pdiss_dn3_slot = var_pdiss_dn3;
        *var_pdiss_dn4_slot = var_pdiss_dn4;
        *var_pdiss_dn5_slot = var_pdiss_dn5;
        *var_pdiss_dn7_slot = var_pdiss_dn7;
        *var_pdiss_dn8_slot = var_pdiss_dn8;
        *var_pdiss_dn9_slot = var_pdiss_dn9;
        *var_qofdsub_slot = var_qofdsub;
        *var_qofdsub_dn0_slot = var_qofdsub_dn0;
        *var_qofdsub_dn3_slot = var_qofdsub_dn3;
        *var_qofdsub_dn4_slot = var_qofdsub_dn4;
        *var_qofgsub_slot = var_qofgsub;
        *var_qofgsub_dn3_slot = var_qofgsub_dn3;
        *var_qofgsub_dn4_slot = var_qofgsub_dn4;
        *var_qofgsub_dn6_slot = var_qofgsub_dn6;
    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.store_scalar(109, (p.p5 + 273.15));

        s.store_scalar(108, ctx_temp);

        s.store_voltage(110, ctx, nodes, Some(4), None);

        s.store_offset(111, 110, (s.v[108] + p.p3));

        s.b[298] = (s.v[111] < ((-270.0) + 273.15));
        s.store_scalar(298, if s.b[298] { 1.0 } else { 0.0 });

        if s.b[298] {
            s.store_scalar(111, ((-270.0) + 273.15));
        }

        s.b[299] = (s.v[111] > (1500.0 + 273.15));
        s.store_scalar(299, if s.b[299] { 1.0 } else { 0.0 });

        if ((!s.b[298]) && s.b[299]) {
            s.store_scalar(111, (1500.0 + 273.15));
        }

        s.store_scale(113, 111, (1.38062e-23 * 6.241457005723417e18));

        s.store_scale_ad(7, {
            if ((1.0 + (p.p21 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p21, (((((-s.v[109])) * (p.p21))) + (1.0)))
            }
        }, p.p9);

        s.store_scale_ad(8, {
            if ((1.0 + (p.p22 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p22, (((((-s.v[109])) * (p.p22))) + (1.0)))
            }
        }, p.p10);

        s.store_scale_ad(9, {
            if ((1.0 + (p.p23 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p23, (((((-s.v[109])) * (p.p23))) + (1.0)))
            }
        }, p.p11);

        s.store_scale_ad(10, {
            if ((1.0 + (p.p24 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p24, (((((-s.v[109])) * (p.p24))) + (1.0)))
            }
        }, p.p13);

        s.store_scale_ad(11, {
            if ((1.0 + (p.p25 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p25, (((((-s.v[109])) * (p.p25))) + (1.0)))
            }
        }, p.p12);

        s.store_scale_ad(12, {
            if ((1.0 + (p.p26 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p26, (((((-s.v[109])) * (p.p26))) + (1.0)))
            }
        }, p.p14);

        s.store_scale_ad(13, {
            if ((1.0 + (p.p21 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p21, (((((-s.v[109])) * (p.p21))) + (1.0)))
            }
        }, p.p15);

        s.store_scale_ad(14, {
            if ((1.0 + (p.p22 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p22, (((((-s.v[109])) * (p.p22))) + (1.0)))
            }
        }, p.p16);

        s.store_scale_ad(15, {
            if ((1.0 + (p.p23 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p23, (((((-s.v[109])) * (p.p23))) + (1.0)))
            }
        }, p.p17);

        s.store_scale_ad(16, {
            if ((1.0 + (p.p24 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p24, (((((-s.v[109])) * (p.p24))) + (1.0)))
            }
        }, p.p19);

        s.store_scale_ad(17, {
            if ((1.0 + (p.p25 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p25, (((((-s.v[109])) * (p.p25))) + (1.0)))
            }
        }, p.p18);

        s.store_scale_ad(18, {
            if ((1.0 + (p.p26 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p26, (((((-s.v[109])) * (p.p26))) + (1.0)))
            }
        }, p.p20);

        s.store_scale_ad(19, {
            if ((1.0 + (p.p8 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p8, (((((-s.v[109])) * (p.p8))) + (1.0)))
            }
        }, p.p7);

        s.store_scale_ad(20, {
            if ((1.0 + (p.p82 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p82, (((((-s.v[109])) * (p.p82))) + (1.0)))
            }
        }, p.p81);

        s.store_scale_ad(23, {
            if ((1.0 + (p.p104 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p104, (((((-s.v[109])) * (p.p104))) + (1.0)))
            }
        }, p.p103);

        s.store_scale_ad(26, {
            if ((1.0 + (p.p126 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p126, (((((-s.v[109])) * (p.p126))) + (1.0)))
            }
        }, p.p125);

        s.store_scale_ad(29, {
            if ((1.0 + (p.p148 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p148, (((((-s.v[109])) * (p.p148))) + (1.0)))
            }
        }, p.p147);

        s.store_scale_ad(21, {
            if ((1.0 + (p.p87 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p87, (((((-s.v[109])) * (p.p87))) + (1.0)))
            }
        }, p.p86);

        s.store_scale_ad(24, {
            if ((1.0 + (p.p109 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p109, (((((-s.v[109])) * (p.p109))) + (1.0)))
            }
        }, p.p108);

        s.store_scale_ad(27, {
            if ((1.0 + (p.p131 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p131, (((((-s.v[109])) * (p.p131))) + (1.0)))
            }
        }, p.p130);

        s.store_scale_ad(30, {
            if ((1.0 + (p.p153 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p153, (((((-s.v[109])) * (p.p153))) + (1.0)))
            }
        }, p.p152);

        s.store_scale_ad(22, {
            if ((1.0 + (p.p89 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p89, (((((-s.v[109])) * (p.p89))) + (1.0)))
            }
        }, p.p88);

        s.store_scale_ad(25, {
            if ((1.0 + (p.p111 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p111, (((((-s.v[109])) * (p.p111))) + (1.0)))
            }
        }, p.p110);

        s.store_scale_ad(28, {
            if ((1.0 + (p.p133 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p133, (((((-s.v[109])) * (p.p133))) + (1.0)))
            }
        }, p.p132);

        s.store_scale_ad(31, {
            if ((1.0 + (p.p155 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p155, (((((-s.v[109])) * (p.p155))) + (1.0)))
            }
        }, p.p154);

        s.store_scale_ad(32, {
            if ((1.0 + (p.p170 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p170, (((((-s.v[109])) * (p.p170))) + (1.0)))
            }
        }, p.p169);

        s.store_scale_ad(35, {
            if ((1.0 + (p.p192 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p192, (((((-s.v[109])) * (p.p192))) + (1.0)))
            }
        }, p.p191);

        s.store_scale_ad(38, {
            if ((1.0 + (p.p214 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p214, (((((-s.v[109])) * (p.p214))) + (1.0)))
            }
        }, p.p213);

        s.store_scale_ad(41, {
            if ((1.0 + (p.p236 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p236, (((((-s.v[109])) * (p.p236))) + (1.0)))
            }
        }, p.p235);

        s.store_scale_ad(33, {
            if ((1.0 + (p.p175 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p175, (((((-s.v[109])) * (p.p175))) + (1.0)))
            }
        }, p.p174);

        s.store_scale_ad(36, {
            if ((1.0 + (p.p197 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p197, (((((-s.v[109])) * (p.p197))) + (1.0)))
            }
        }, p.p196);

        s.store_scale_ad(39, {
            if ((1.0 + (p.p219 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p219, (((((-s.v[109])) * (p.p219))) + (1.0)))
            }
        }, p.p218);

        s.store_scale_ad(42, {
            if ((1.0 + (p.p241 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p241, (((((-s.v[109])) * (p.p241))) + (1.0)))
            }
        }, p.p240);

        s.store_scale_ad(34, {
            if ((1.0 + (p.p177 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p177, (((((-s.v[109])) * (p.p177))) + (1.0)))
            }
        }, p.p176);

        s.store_scale_ad(37, {
            if ((1.0 + (p.p199 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p199, (((((-s.v[109])) * (p.p199))) + (1.0)))
            }
        }, p.p198);

        s.store_scale_ad(40, {
            if ((1.0 + (p.p221 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p221, (((((-s.v[109])) * (p.p221))) + (1.0)))
            }
        }, p.p220);

        s.store_scale_ad(43, {
            if ((1.0 + (p.p243 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p243, (((((-s.v[109])) * (p.p243))) + (1.0)))
            }
        }, p.p242);

        s.store_scaled_voltage(44, ctx, nodes, Some(5), Some(9), p.p6);

        s.store_scaled_voltage(45, ctx, nodes, Some(8), Some(9), p.p6);

        s.store_scalar(224, 0.0);

        s.store_scalar(226, 0.0);

        s.store_scalar(225, 0.0);

        s.store_scalar(227, 0.0);

        s.store_scalar(228, 0.0);

        s.store_scalar(229, 0.0);

        s.store_scalar(230, 1.0);

        s.b[308] = (p.p328 == 1.0);
        s.store_scalar(308, if s.b[308] { 1.0 } else { 0.0 });

        s.b[309] = (p.p328 == 2.0);
        s.store_scalar(309, if s.b[309] { 1.0 } else { 0.0 });

        if ((!s.b[308]) && s.b[309]) {
            s.store_voltage(224, ctx, nodes, Some(22), None);
            s.store_voltage(225, ctx, nodes, Some(23), None);
            s.store_scaled_abs_ad(228, A::sub(s.ad_value(225), s.ad_value(224)), 1.0 / (p.p338));
            s.store_voltage(226, ctx, nodes, Some(25), None);
            s.store_voltage(227, ctx, nodes, Some(26), None);
            s.store_scaled_abs_ad(229, A::sub(s.ad_value(227), s.ad_value(226)), 1.0 / (p.p337));
            s.store_div_from_scalar_add_ad(230, 1.0, A::offset(s.ad_value(228), 1.0), s.ad_value(229));
        }

        s.b[312] = (p.p78 == 1.0);
        s.store_scalar(312, if s.b[312] { 1.0 } else { 0.0 });

        if s.b[312] {
            s.store_scaled_voltage(60, ctx, nodes, Some(7), Some(10), p.p6);
            s.store_scaled_voltage(62, ctx, nodes, Some(2), Some(10), p.p6);
        }

        if (!s.b[312]) {
            s.store_scaled_voltage(60, ctx, nodes, Some(2), Some(10), p.p6);
            s.store_scaled_voltage(62, ctx, nodes, Some(7), Some(10), p.p6);
        }

        s.store_scaled_voltage(61, ctx, nodes, Some(9), Some(10), p.p6);

        s.store_scaled_voltage(63, ctx, nodes, Some(3), Some(10), p.p6);

        s.b[313] = (p.p100 == 1.0);
        s.store_scalar(313, if s.b[313] { 1.0 } else { 0.0 });

        if s.b[313] {
            s.store_scaled_voltage(66, ctx, nodes, Some(7), Some(11), p.p6);
            s.store_scaled_voltage(68, ctx, nodes, Some(2), Some(11), p.p6);
        }

        if (!s.b[313]) {
            s.store_scaled_voltage(66, ctx, nodes, Some(2), Some(11), p.p6);
            s.store_scaled_voltage(68, ctx, nodes, Some(7), Some(11), p.p6);
        }

        s.store_scaled_voltage(67, ctx, nodes, Some(10), Some(11), p.p6);

        s.store_scaled_voltage(69, ctx, nodes, Some(3), Some(11), p.p6);

        s.b[314] = (p.p122 == 1.0);
        s.store_scalar(314, if s.b[314] { 1.0 } else { 0.0 });

        if s.b[314] {
            s.store_scaled_voltage(72, ctx, nodes, Some(7), Some(12), p.p6);
            s.store_scaled_voltage(74, ctx, nodes, Some(2), Some(12), p.p6);
        }

        if (!s.b[314]) {
            s.store_scaled_voltage(72, ctx, nodes, Some(2), Some(12), p.p6);
            s.store_scaled_voltage(74, ctx, nodes, Some(7), Some(12), p.p6);
        }

        s.store_scaled_voltage(73, ctx, nodes, Some(11), Some(12), p.p6);

        s.store_scaled_voltage(75, ctx, nodes, Some(3), Some(12), p.p6);

        s.b[315] = (p.p144 == 1.0);
        s.store_scalar(315, if s.b[315] { 1.0 } else { 0.0 });

        if s.b[315] {
            s.store_scaled_voltage(78, ctx, nodes, Some(7), Some(13), p.p6);
            s.store_scaled_voltage(80, ctx, nodes, Some(2), Some(13), p.p6);
        }

        if (!s.b[315]) {
            s.store_scaled_voltage(78, ctx, nodes, Some(2), Some(13), p.p6);
            s.store_scaled_voltage(80, ctx, nodes, Some(7), Some(13), p.p6);
        }

        s.store_scaled_voltage(79, ctx, nodes, Some(12), Some(13), p.p6);

        s.store_scaled_voltage(81, ctx, nodes, Some(3), Some(13), p.p6);

        s.b[316] = (p.p166 == 1.0);
        s.store_scalar(316, if s.b[316] { 1.0 } else { 0.0 });

        if s.b[316] {
            s.store_scaled_voltage(84, ctx, nodes, Some(7), Some(5), p.p6);
            s.store_scaled_voltage(86, ctx, nodes, Some(2), Some(5), p.p6);
        }

        if (!s.b[316]) {
            s.store_scaled_voltage(84, ctx, nodes, Some(2), Some(5), p.p6);
            s.store_scaled_voltage(86, ctx, nodes, Some(7), Some(5), p.p6);
        }

        s.store_scaled_voltage(85, ctx, nodes, Some(14), Some(5), p.p6);

        s.store_scaled_voltage(87, ctx, nodes, Some(3), Some(5), p.p6);

        s.b[317] = (p.p188 == 1.0);
        s.store_scalar(317, if s.b[317] { 1.0 } else { 0.0 });

        if s.b[317] {
            s.store_scaled_voltage(90, ctx, nodes, Some(7), Some(14), p.p6);
            s.store_scaled_voltage(92, ctx, nodes, Some(2), Some(14), p.p6);
        }

        if (!s.b[317]) {
            s.store_scaled_voltage(90, ctx, nodes, Some(2), Some(14), p.p6);
            s.store_scaled_voltage(92, ctx, nodes, Some(7), Some(14), p.p6);
        }

        s.store_scaled_voltage(91, ctx, nodes, Some(15), Some(14), p.p6);

        s.store_scaled_voltage(93, ctx, nodes, Some(3), Some(14), p.p6);

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[318] = (p.p210 == 1.0);
        s.store_scalar(318, if s.b[318] { 1.0 } else { 0.0 });

        if s.b[318] {
            s.store_scaled_voltage(96, ctx, nodes, Some(7), Some(15), p.p6);
            s.store_scaled_voltage(98, ctx, nodes, Some(2), Some(15), p.p6);
        }

        if (!s.b[318]) {
            s.store_scaled_voltage(96, ctx, nodes, Some(2), Some(15), p.p6);
            s.store_scaled_voltage(98, ctx, nodes, Some(7), Some(15), p.p6);
        }

        s.store_scaled_voltage(97, ctx, nodes, Some(16), Some(15), p.p6);

        s.store_scaled_voltage(99, ctx, nodes, Some(3), Some(15), p.p6);

        s.b[319] = (p.p232 == 1.0);
        s.store_scalar(319, if s.b[319] { 1.0 } else { 0.0 });

        if s.b[319] {
            s.store_scaled_voltage(102, ctx, nodes, Some(7), Some(16), p.p6);
            s.store_scaled_voltage(104, ctx, nodes, Some(2), Some(16), p.p6);
        }

        if (!s.b[319]) {
            s.store_scaled_voltage(102, ctx, nodes, Some(2), Some(16), p.p6);
            s.store_scaled_voltage(104, ctx, nodes, Some(7), Some(16), p.p6);
        }

        s.store_scaled_voltage(103, ctx, nodes, Some(17), Some(16), p.p6);

        s.store_scaled_voltage(105, ctx, nodes, Some(3), Some(16), p.p6);

        s.store_scalar(209, 0.0);

        s.store_scalar(210, 0.0);

        s.store_scalar(211, 0.0);

        s.store_scalar(212, 0.0);

        s.store_scalar(213, 0.0);

        s.b[320] = (p.p233 > p.p354);
        s.store_scalar(320, if s.b[320] { 1.0 } else { 0.0 });

        if s.b[320] {
            s.store_scalar(323, 0.0);
            s.store_scalar(324, 0.0);
            s.store_scalar(325, 0.0);
            s.store_scalar(326, 0.0);
            s.store_scalar(327, 0.0);
            s.store_scalar(328, 0.0);
            s.store_scalar(329, 0.0);
            s.copy_ad(330, 102);
            s.copy_ad(331, 103);
            s.store_scalar(332, p.p239);
            s.copy_ad(333, 104);
            s.copy_ad(334, 105);
            s.store_scalar(335, p.p237);
            s.copy_ad(336, 111);
            s.store_scalar(337, s.v[109]);
            s.copy_ad(338, 113);
            s.store_scalar(339, p.p0);
            s.store_scalar(340, p.p233);
            s.copy_ad(341, 41);
            s.store_scalar(342, p.p238);
            s.copy_ad(343, 42);
            s.copy_ad(344, 43);
            s.store_scalar(345, p.p234);
            s.store_scalar(346, p.p248);
            s.store_scalar(347, p.p247);
            s.store_scalar(348, 0.0);
            s.store_scalar(349, p.p249);
            s.store_scalar(350, p.p253);
            s.store_scalar(351, p.p244);
            s.store_scalar(352, p.p245);
            s.store_scalar(353, p.p246);
            s.store_scalar(354, p.p252);
            s.store_scalar(355, p.p251);
            s.store_scalar(356, p.p250);
            s.store_scalar(357, p.p39);
            s.store_scalar(358, p.p47);
            s.store_scalar(359, p.p45);
            s.store_scalar(360, p.p42);
            s.store_scalar(361, p.p2);
            s.store_scalar(362, p.p6);
            s.store_scalar(363, 1.0);
            s.store_scalar(364, 0.0);
            s.store_scalar(365, 0.0);
            s.store_scalar(366, 0.0);
            s.store_scalar(367, 0.0);
            s.store_scalar(368, 0.0);
            s.store_scalar(369, 0.0);
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 0.0);
            s.store_scalar(372, 0.0);
            s.store_scalar(373, 0.0);
            s.store_scalar(374, 0.0);
            s.store_scalar(375, 0.0);
            s.store_scalar(377, 0.0);
            s.store_scalar(378, 0.0);
            s.store_scalar(379, 0.0);
            s.store_scalar(380, 0.0);
            s.store_scalar(381, 0.0);
            s.store_scalar(382, 0.0);
            s.store_scalar(383, 0.0);
            s.store_scalar(384, 0.0);
            s.store_scalar(385, 0.0);
            s.store_scalar(386, 0.0);
            s.store_scalar(387, 0.0);
            s.store_scalar(388, 0.0);
            s.store_scalar(389, 0.0);
            s.store_scalar(390, 0.0);
            s.store_scalar(391, 0.0);
            s.store_scalar(392, 0.0);
            s.store_scalar(393, 0.0);
            s.store_scalar(394, 0.0);
            s.store_scalar(395, 0.0);
            s.store_scalar(396, 0.0);
            s.store_scalar(397, 0.0);
            s.store_scalar(398, 0.0);
            s.store_scalar(399, 0.0);
            s.store_scalar(400, 0.0);
            s.store_scalar(401, 0.0);
            s.store_scalar(402, 0.0);
            s.store_scalar(405, 0.0);
            s.store_scalar(406, 0.0);
            s.store_scalar(407, 0.0);
            s.store_scalar(408, 0.0);
            s.store_scalar(409, 0.0);
            s.store_scalar(410, 0.0);
            s.store_scalar(411, 0.0);
            s.store_scalar(412, 0.0);
            s.store_scalar(413, 0.0);
            s.store_scalar(414, 0.0);
            s.store_scalar(415, 0.0);
            s.store_scalar(416, 0.0);
            s.store_scalar(417, 0.0);
            s.store_scalar(418, 0.0);
            s.store_scalar(419, 0.0);
            s.store_scalar(420, 0.0);
            s.store_scalar(421, 0.0);
            s.store_scalar(422, 0.0);
            s.store_scalar(423, 0.0);
            s.store_scalar(424, 0.0);
            s.store_scalar(425, 0.0);
            s.store_scalar(426, 0.0);
            s.store_scalar(427, 0.0);
            s.store_scalar(428, 0.0);
            s.store_scalar(429, 0.0);
            s.store_scalar(430, 0.0);
            s.store_scalar(431, 0.0);
            s.store_scalar(432, 0.0);
        }

        if s.b[320] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(429, 331, A::tanh_scaled_input(s.ad_value(331), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(429, 331, p.p53);
                } else {
                    s.store_scalar(429, 0.0);
                }
            }
        }

        if s.b[320] {
            s.store_sub(430, 330, 331);
            s.store_mul(364, 350, 338);
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[320] {
            s.store_add_scaled_product_value_ad(366, A::div_scaled_inputs(s.ad_value(346), 1.0, s.ad_value(338), 2.302585092994046), 1.0, 349, 429, 1.0);
            s.store_add_scaled_product_right_sub(367, 345, 1.0, 356, 336, 337, 1.0);
            s.store_pow_ad(385, A::div(s.ad_value(336), s.ad_value(337)), s.ad_value(358));
        }

        s.b[433] = (s.v[357] != 0.0);
        s.store_scalar(433, if s.b[433] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[433]) {
            s.store_div_ad_rhs(368, 429, A::pow(A::offset(A::pow(A::div(s.ad_value(429), s.ad_value(357)), s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353))));
        }

        if (s.b[320] && (!s.b[433])) {
            s.store_scalar(368, 0.0);
        }

        if s.b[320] {
            s.store_mul_add_scaled_product_rhs(365, 429, s.ad_value(347), 1.0, s.ad_value(368), s.ad_value(348), (-1.0));
            s.store_sub(328, 367, 365);
            s.store_scaled_mul(370, 366, 338, 2.0);
            s.store_mul(371, 341, 370);
            s.store_sub_scaled_inputs(428, 328, 1.0, 364, (p.p51 * 0.5));
        }

        if s.b[320] {
            s.store_div_scaled_inputs2_mixed_aii(427, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sub(s.ad_value(330), s.ad_value(430)), A::tanh_scaled_input(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(330), s.ad_value(430)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 428, (-1.0), 364, 1.0);
        }

        s.b[434] = (s.v[427] > 50.0);
        s.store_scalar(434, if s.b[434] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[434]) {
            s.store_scalar(386, 0.0);
        }

        s.b[435] = (s.v[427] < (-50.0));
        s.store_scalar(435, if s.b[435] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[434])) && s.b[435]) {
            s.store_scalar(386, 1.0);
        }

        if ((s.b[320] && (!s.b[434])) && (!s.b[435])) {
            s.store_div_from_scalar_offset_ad(386, 1.0, A::exp(s.ad_value(427)), 1.0);
        }

        if s.b[320] {
            s.store_div_scaled_inputs2_mixed_aai(387, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sub(s.ad_value(330), s.ad_value(430)), A::tanh_scaled_input(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(330), s.ad_value(430)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(364), s.ad_value(386), (-(p.p51 * 0.1))), (-1.0), 370, 1.0);
        }

        s.b[436] = (s.v[387] > 50.0);
        s.store_scalar(436, if s.b[436] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[436]) {
            s.store_mul(388, 371, 387);
        }

        s.b[437] = (s.v[387] < (-50.0));
        s.store_scalar(437, if s.b[437] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[436])) && s.b[437]) {
            s.store_mul_exp_rhs(388, 371, 387);
        }

        if ((s.b[320] && (!s.b[436])) && (!s.b[437])) {
            s.store_mul_ln_one_plus_exp_rhs(388, 371, 387);
        }

        if s.b[320] {
            s.store_div_ad_rhs(374, 352, A::mul_offset_rhs(s.ad_value(385), A::div_scaled_product(s.ad_value(354), s.ad_value(388), 1.0, s.ad_value(341), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(375, 351, A::div_scaled_offset_numerator(A::mul(s.ad_value(359), s.ad_value(337)), 1.0, 1.0, A::offset(A::mul(s.ad_value(359), s.ad_value(336)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(360), s.ad_value(429), 1.0, s.ad_value(340), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(355), s.ad_value(388), 1.0, s.ad_value(341), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(392, 375, 340, 1.0, 374, 1.0);
            s.store_add_scaled_product_right_ad(393, 392, (-1.0), 392, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(388), 2.0, s.ad_value(341), s.ad_value(392), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(394, A::mul_sub_from_scalar_rhs(s.ad_value(392), 1.0, s.ad_value(386)), 1.0, 370, 386, 1.0);
            s.store_add_scaled_product_value_ad(329, A::mul_sub_from_scalar_rhs(s.ad_value(393), 1.0, s.ad_value(386)), 1.0, 370, 386, 1.0);
        }

        if s.b[320] {
            s.store_div_from_scalar_pow_ad(395, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(331), s.ad_value(329)), 0.5, A::div(s.ad_value(331), s.ad_value(329)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(331), s.ad_value(329))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(331), s.ad_value(329)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(331), s.ad_value(329))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
        }

        if s.b[320] {
            s.store_mul(396, 331, 395);
        }

        if s.b[320] {
            s.store_div_from_scalar_pow_ad(397, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
        }

        if s.b[320] {
            s.store_mul_neg_lhs(398, 331, 397);
            s.store_div_scaled_inputs2_indices(427, 330, 1.0, 428, (-1.0), 364, 1.0);
        }

        s.b[438] = (s.v[427] > 50.0);
        s.store_scalar(438, if s.b[438] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[438]) {
            s.store_scalar(369, 0.0);
        }

        s.b[439] = (s.v[427] < (-50.0));
        s.store_scalar(439, if s.b[439] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[438])) && s.b[439]) {
            s.store_scalar(369, 1.0);
        }

        if ((s.b[320] && (!s.b[438])) && (!s.b[439])) {
            s.store_div_from_scalar_offset_ad(369, 1.0, A::exp(s.ad_value(427)), 1.0);
        }

        if s.b[320] {
            s.store_div_scaled_inputs3_mixed_iiai(372, 430, 1.0, 398, (-1.0), A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(364), s.ad_value(369), (-(p.p51 * 0.1))), -1.0, 370, 1.0);
        }

        s.b[440] = (s.v[372] > 50.0);
        s.store_scalar(440, if s.b[440] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[440]) {
            s.store_mul(373, 371, 372);
        }

        s.b[441] = (s.v[372] < (-50.0));
        s.store_scalar(441, if s.b[441] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[440])) && s.b[441]) {
            s.store_mul_exp_rhs(373, 371, 372);
        }

        if ((s.b[320] && (!s.b[440])) && (!s.b[441])) {
            s.store_mul_ln_one_plus_exp_rhs(373, 371, 372);
        }

        if s.b[320] {
            s.store_div_scaled_inputs2_indices(427, 430, 1.0, 428, (-1.0), 364, 1.0);
        }

        s.b[442] = (s.v[427] > 50.0);
        s.store_scalar(442, if s.b[442] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[442]) {
            s.store_scalar(399, 0.0);
        }

        s.b[443] = (s.v[427] < (-50.0));
        s.store_scalar(443, if s.b[443] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[442])) && s.b[443]) {
            s.store_scalar(399, 1.0);
        }

        if ((s.b[320] && (!s.b[442])) && (!s.b[443])) {
            s.store_div_from_scalar_offset_ad(399, 1.0, A::exp(s.ad_value(427)), 1.0);
        }

        if s.b[320] {
            s.store_div_scaled_inputs3_mixed_iiai(400, 330, 1.0, 396, (-1.0), A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(364), s.ad_value(399), (-(p.p51 * 0.1))), -1.0, 370, 1.0);
        }

        s.b[444] = (s.v[400] > 50.0);
        s.store_scalar(444, if s.b[444] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[444]) {
            s.store_mul(401, 371, 400);
        }

        s.b[445] = (s.v[400] < (-50.0));
        s.store_scalar(445, if s.b[445] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[444])) && s.b[445]) {
            s.store_mul_exp_rhs(401, 371, 400);
        }

        if ((s.b[320] && (!s.b[444])) && (!s.b[445])) {
            s.store_mul_ln_one_plus_exp_rhs(401, 371, 400);
        }

        if s.b[320] {
            s.store_div_scaled_inputs2_indices(402, 373, 1.0, 401, (-1.0), 341, 1.0);
            s.store_div(428, 402, 394);
            s.store_div_scaled_inputs_indices(377, 346, 1.0, 338, 2.302585092994046);
            s.store_scaled_mul(379, 377, 338, 2.0);
            s.store_mul(380, 341, 379);
            s.store_sub_scaled_inputs(432, 367, 1.0, 364, (p.p51 * 0.5));
        }

        if s.b[320] {
            s.store_div_scaled_inputs2_mixed_aii(431, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sub(s.ad_value(330), s.ad_value(430)), A::tanh_scaled_input(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(330), s.ad_value(430)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 432, (-1.0), 364, 1.0);
        }

        s.b[446] = (s.v[431] > 50.0);
        s.store_scalar(446, if s.b[446] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[446]) {
            s.store_scalar(389, 0.0);
        }

        s.b[447] = (s.v[431] < (-50.0));
        s.store_scalar(447, if s.b[447] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[446])) && s.b[447]) {
            s.store_scalar(389, 1.0);
        }

        if ((s.b[320] && (!s.b[446])) && (!s.b[447])) {
            s.store_div_from_scalar_offset_ad(389, 1.0, A::exp(s.ad_value(431)), 1.0);
        }

        if s.b[320] {
            s.store_div_scaled_inputs2_mixed_aai(390, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sub(s.ad_value(330), s.ad_value(430)), A::tanh_scaled_input(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(330), s.ad_value(430)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(367), 1.0, s.ad_value(364), s.ad_value(389), (-(p.p51 * 0.1))), (-1.0), 379, 1.0);
        }

        s.b[448] = (s.v[390] > 50.0);
        s.store_scalar(448, if s.b[448] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[448]) {
            s.store_mul(391, 380, 390);
        }

        s.b[449] = (s.v[390] < (-50.0));
        s.store_scalar(449, if s.b[449] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[448])) && s.b[449]) {
            s.store_mul_exp_rhs(391, 380, 390);
        }

        if ((s.b[320] && (!s.b[448])) && (!s.b[449])) {
            s.store_mul_ln_one_plus_exp_rhs(391, 380, 390);
        }

        if s.b[320] {
            s.store_div(383, 352, 385);
            s.store_mul_div_scaled_offset_numerator_rhs(384, 351, A::mul(s.ad_value(359), s.ad_value(337)), 1.0, 1.0, A::offset(A::mul(s.ad_value(359), s.ad_value(336)), 1.0), 1.0);
            s.store_div_scaled_product_indices(405, 384, 340, 1.0, 383, 1.0);
            s.store_add_scaled_product_right_ad(406, 405, (-1.0), 405, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(391), 2.0, s.ad_value(341), s.ad_value(405), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(407, A::mul_sub_from_scalar_rhs(s.ad_value(406), 1.0, s.ad_value(389)), 1.0, 379, 389, 1.0);
        }

        if s.b[320] {
            s.store_div_from_scalar_pow_ad(408, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(331), s.ad_value(407)), 0.5, A::div(s.ad_value(331), s.ad_value(407)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(331), s.ad_value(407))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(331), s.ad_value(407)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(331), s.ad_value(407))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
        }

        if s.b[320] {
            s.store_mul(409, 331, 408);
        }

        if s.b[320] {
            s.store_div_from_scalar_pow_ad(410, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
        }

        if s.b[320] {
            s.store_mul_neg_lhs(411, 331, 410);
            s.store_div_scaled_inputs2_indices(431, 330, 1.0, 432, (-1.0), 364, 1.0);
        }

        s.b[450] = (s.v[431] > 50.0);
        s.store_scalar(450, if s.b[450] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[450]) {
            s.store_scalar(378, 0.0);
        }

        s.b[451] = (s.v[431] < (-50.0));
        s.store_scalar(451, if s.b[451] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[450])) && s.b[451]) {
            s.store_scalar(378, 1.0);
        }

        if ((s.b[320] && (!s.b[450])) && (!s.b[451])) {
            s.store_div_from_scalar_offset_ad(378, 1.0, A::exp(s.ad_value(431)), 1.0);
        }

        if s.b[320] {
            s.store_div_scaled_inputs3_mixed_iiai(381, 430, 1.0, 411, (-1.0), A::add_scaled_product(s.ad_value(367), 1.0, s.ad_value(364), s.ad_value(378), (-(p.p51 * 0.1))), -1.0, 379, 1.0);
        }

        s.b[452] = (s.v[381] > 50.0);
        s.store_scalar(452, if s.b[452] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[452]) {
            s.store_mul(382, 380, 381);
        }

        s.b[453] = (s.v[381] < (-50.0));
        s.store_scalar(453, if s.b[453] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[452])) && s.b[453]) {
            s.store_mul_exp_rhs(382, 380, 381);
        }

        if ((s.b[320] && (!s.b[452])) && (!s.b[453])) {
            s.store_mul_ln_one_plus_exp_rhs(382, 380, 381);
        }

        if s.b[320] {
            s.store_div_scaled_inputs2_indices(431, 430, 1.0, 432, (-1.0), 364, 1.0);
        }

        s.b[454] = (s.v[431] > 50.0);
        s.store_scalar(454, if s.b[454] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[454]) {
            s.store_scalar(412, 0.0);
        }

        s.b[455] = (s.v[431] < (-50.0));
        s.store_scalar(455, if s.b[455] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[454])) && s.b[455]) {
            s.store_scalar(412, 1.0);
        }

        if ((s.b[320] && (!s.b[454])) && (!s.b[455])) {
            s.store_div_from_scalar_offset_ad(412, 1.0, A::exp(s.ad_value(431)), 1.0);
        }

        if s.b[320] {
            s.store_div_scaled_inputs3_mixed_iiai(413, 330, 1.0, 409, (-1.0), A::add_scaled_product(s.ad_value(367), 1.0, s.ad_value(364), s.ad_value(412), (-(p.p51 * 0.1))), -1.0, 379, 1.0);
        }

        s.b[456] = (s.v[413] > 50.0);
        s.store_scalar(456, if s.b[456] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[456]) {
            s.store_mul(414, 380, 413);
        }

        s.b[457] = (s.v[413] < (-50.0));
        s.store_scalar(457, if s.b[457] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[456])) && s.b[457]) {
            s.store_mul_exp_rhs(414, 380, 413);
        }

        if ((s.b[320] && (!s.b[456])) && (!s.b[457])) {
            s.store_mul_ln_one_plus_exp_rhs(414, 380, 413);
        }

        if s.b[320] {
            s.store_offset_square(415, 382, 1e-38);
            s.store_offset_mul(416, 415, 382, 1e-57);
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[320] {
            s.store_offset_square(417, 414, 1e-38);
            s.store_offset_mul(418, 417, 414, 1e-57);
            s.store_offset_mul(419, 382, 414, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(420, 415, (2.0 / 3.0), 417, (2.0 / 3.0), 419, (2.0 / 3.0), A::offset(A::add(s.ad_value(382), s.ad_value(414)), 2e-19), 1.0);
            s.store_div_ad(421, A::add_scaled_inputs_products(s.ad_value(416), (2.0 * 2.0), s.ad_value(418), (3.0 * 2.0), s.ad_value(415), s.ad_value(414), (4.0 * 2.0), s.ad_value(417), s.ad_value(382), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(415), 15.0, s.ad_value(417), 15.0, s.ad_value(419), (2.0 * 15.0)));
            s.store_sub(422, 420, 421);
            s.copy_ad(423, 421);
            s.store_mul_product3_mixed_iaii(323, 363, A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(340)), 362, 422, 1.0);
            s.store_mul_product3_mixed_iaii(324, 363, A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(340)), 362, 423, 1.0);
        }

        s.b[458] = (s.v[332] == 1.0);
        s.store_scalar(458, if s.b[458] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[458]) {
            s.store_div_scaled_inputs3_indices(424, 333, 1.0, 367, -1.0, 364, (-(-(p.p51 * 0.5))), 379, 1.0);
        }

        s.b[459] = (s.v[424] > 50.0);
        s.store_scalar(459, if s.b[459] { 1.0 } else { 0.0 });

        if ((s.b[320] && s.b[458]) && s.b[459]) {
            s.copy_ad(427, 424);
        }

        s.b[460] = (s.v[424] < (-50.0));
        s.store_scalar(460, if s.b[460] { 1.0 } else { 0.0 });

        if (((s.b[320] && s.b[458]) && (!s.b[459])) && s.b[460]) {
            s.store_exp(427, 424);
        }

        if (((s.b[320] && s.b[458]) && (!s.b[459])) && (!s.b[460])) {
            s.store_ln_one_plus_exp(427, 424);
        }

        if (s.b[320] && s.b[458]) {
            s.store_mul_ad_product_lhs_mixed_ai(325, A::mul3(A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(362)), s.ad_value(343), s.ad_value(379)), 427, 363);
            s.store_div_scaled_inputs3_indices(425, 334, 1.0, 367, -1.0, 364, (-(-(p.p51 * 0.5))), 379, 1.0);
        }

        s.b[461] = (s.v[425] > 50.0);
        s.store_scalar(461, if s.b[461] { 1.0 } else { 0.0 });

        if ((s.b[320] && s.b[458]) && s.b[461]) {
            s.copy_ad(427, 425);
        }

        s.b[462] = (s.v[425] < (-50.0));
        s.store_scalar(462, if s.b[462] { 1.0 } else { 0.0 });

        if (((s.b[320] && s.b[458]) && (!s.b[461])) && s.b[462]) {
            s.store_exp(427, 425);
        }

        if (((s.b[320] && s.b[458]) && (!s.b[461])) && (!s.b[462])) {
            s.store_ln_one_plus_exp(427, 425);
        }

        if (s.b[320] && s.b[458]) {
            s.store_mul_ad_product_lhs_mixed_ai(326, A::mul3(A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(362)), s.ad_value(344), s.ad_value(379)), 427, 363);
        }

        if (s.b[320] && (!s.b[458])) {
            s.store_scalar(325, 0.0);
            s.store_scalar(326, 0.0);
        }

        s.b[463] = (s.v[335] == 1.0);
        s.store_scalar(463, if s.b[463] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[463]) {
            s.store_div_scaled_inputs3_indices(426, 330, 1.0, 367, -1.0, 364, (-(-(p.p51 * 0.5))), 379, 1.0);
        }

        s.b[464] = (s.v[426] > 50.0);
        s.store_scalar(464, if s.b[464] { 1.0 } else { 0.0 });

        if ((s.b[320] && s.b[463]) && s.b[464]) {
            s.copy_ad(427, 426);
        }

        s.b[465] = (s.v[426] < (-50.0));
        s.store_scalar(465, if s.b[465] { 1.0 } else { 0.0 });

        if (((s.b[320] && s.b[463]) && (!s.b[464])) && s.b[465]) {
            s.store_exp(427, 426);
        }

        if (((s.b[320] && s.b[463]) && (!s.b[464])) && (!s.b[465])) {
            s.store_ln_one_plus_exp(427, 426);
        }

        if (s.b[320] && s.b[463]) {
            s.store_mul_ad_product_lhs_mixed_ai(327, A::mul3(A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(362)), s.ad_value(342), s.ad_value(379)), 427, 363);
        }

        if (s.b[320] && (!s.b[463])) {
            s.store_scalar(327, 0.0);
        }

        if s.b[320] {
            s.copy_ad(209, 323);
            s.copy_ad(210, 324);
            s.copy_ad(211, 325);
            s.copy_ad(212, 326);
            s.copy_ad(213, 327);
        }

        s.b[466] = (p.p232 == 1.0);
        s.store_scalar(466, if s.b[466] { 1.0 } else { 0.0 });

        s.store_scalar(203, 0.0);

        s.store_scalar(204, 0.0);

        s.store_scalar(205, 0.0);

        s.store_scalar(206, 0.0);

        s.store_scalar(207, 0.0);

        s.b[467] = (p.p211 > p.p354);
        s.store_scalar(467, if s.b[467] { 1.0 } else { 0.0 });

        if s.b[467] {
            s.store_scalar(470, 0.0);
            s.store_scalar(471, 0.0);
            s.store_scalar(472, 0.0);
            s.store_scalar(473, 0.0);
            s.store_scalar(474, 0.0);
            s.store_scalar(475, 0.0);
            s.store_scalar(476, 0.0);
            s.copy_ad(477, 96);
            s.copy_ad(478, 97);
            s.store_scalar(479, p.p217);
            s.copy_ad(480, 98);
            s.copy_ad(481, 99);
            s.store_scalar(482, p.p215);
            s.copy_ad(483, 111);
            s.store_scalar(484, s.v[109]);
            s.copy_ad(485, 113);
            s.store_scalar(486, p.p0);
            s.store_scalar(487, p.p211);
            s.copy_ad(488, 38);
            s.store_scalar(489, p.p216);
            s.copy_ad(490, 39);
            s.copy_ad(491, 40);
            s.store_scalar(492, p.p212);
            s.store_scalar(493, p.p226);
            s.store_scalar(494, p.p225);
            s.store_scalar(495, 0.0);
            s.store_scalar(496, p.p227);
            s.store_scalar(497, p.p231);
            s.store_scalar(498, p.p222);
            s.store_scalar(499, p.p223);
            s.store_scalar(500, p.p224);
            s.store_scalar(501, p.p230);
            s.store_scalar(502, p.p229);
            s.store_scalar(503, p.p228);
            s.store_scalar(504, p.p39);
            s.store_scalar(505, p.p47);
            s.store_scalar(506, p.p45);
            s.store_scalar(507, p.p42);
            s.store_scalar(508, p.p2);
            s.store_scalar(509, p.p6);
            s.store_scalar(510, 1.0);
            s.store_scalar(511, 0.0);
            s.store_scalar(512, 0.0);
            s.store_scalar(513, 0.0);
            s.store_scalar(514, 0.0);
            s.store_scalar(515, 0.0);
            s.store_scalar(516, 0.0);
            s.store_scalar(517, 0.0);
            s.store_scalar(518, 0.0);
            s.store_scalar(519, 0.0);
            s.store_scalar(520, 0.0);
            s.store_scalar(521, 0.0);
            s.store_scalar(522, 0.0);
            s.store_scalar(524, 0.0);
            s.store_scalar(525, 0.0);
            s.store_scalar(526, 0.0);
            s.store_scalar(527, 0.0);
            s.store_scalar(528, 0.0);
            s.store_scalar(529, 0.0);
            s.store_scalar(530, 0.0);
            s.store_scalar(531, 0.0);
            s.store_scalar(532, 0.0);
            s.store_scalar(533, 0.0);
            s.store_scalar(534, 0.0);
            s.store_scalar(535, 0.0);
            s.store_scalar(536, 0.0);
            s.store_scalar(537, 0.0);
            s.store_scalar(538, 0.0);
            s.store_scalar(539, 0.0);
            s.store_scalar(540, 0.0);
            s.store_scalar(541, 0.0);
            s.store_scalar(542, 0.0);
            s.store_scalar(543, 0.0);
            s.store_scalar(544, 0.0);
            s.store_scalar(545, 0.0);
            s.store_scalar(546, 0.0);
            s.store_scalar(547, 0.0);
            s.store_scalar(548, 0.0);
            s.store_scalar(549, 0.0);
            s.store_scalar(552, 0.0);
            s.store_scalar(553, 0.0);
            s.store_scalar(554, 0.0);
            s.store_scalar(555, 0.0);
            s.store_scalar(556, 0.0);
            s.store_scalar(557, 0.0);
            s.store_scalar(558, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[467] {
            s.store_scalar(559, 0.0);
            s.store_scalar(560, 0.0);
            s.store_scalar(561, 0.0);
            s.store_scalar(562, 0.0);
            s.store_scalar(563, 0.0);
            s.store_scalar(564, 0.0);
            s.store_scalar(565, 0.0);
            s.store_scalar(566, 0.0);
            s.store_scalar(567, 0.0);
            s.store_scalar(568, 0.0);
            s.store_scalar(569, 0.0);
            s.store_scalar(570, 0.0);
            s.store_scalar(571, 0.0);
            s.store_scalar(572, 0.0);
            s.store_scalar(573, 0.0);
            s.store_scalar(574, 0.0);
            s.store_scalar(575, 0.0);
            s.store_scalar(576, 0.0);
            s.store_scalar(577, 0.0);
            s.store_scalar(578, 0.0);
            s.store_scalar(579, 0.0);
        }

        if s.b[467] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(576, 478, A::tanh_scaled_input(s.ad_value(478), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(576, 478, p.p53);
                } else {
                    s.store_scalar(576, 0.0);
                }
            }
        }

        if s.b[467] {
            s.store_sub(577, 477, 478);
            s.store_mul(511, 497, 485);
            s.store_add_scaled_product_value_ad(513, A::div_scaled_inputs(s.ad_value(493), 1.0, s.ad_value(485), 2.302585092994046), 1.0, 496, 576, 1.0);
            s.store_add_scaled_product_right_sub(514, 492, 1.0, 503, 483, 484, 1.0);
            s.store_pow_ad(532, A::div(s.ad_value(483), s.ad_value(484)), s.ad_value(505));
        }

        s.b[580] = (s.v[504] != 0.0);
        s.store_scalar(580, if s.b[580] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[580]) {
            s.store_div_ad_rhs(515, 576, A::pow(A::offset(A::pow(A::div(s.ad_value(576), s.ad_value(504)), s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500))));
        }

        if (s.b[467] && (!s.b[580])) {
            s.store_scalar(515, 0.0);
        }

        if s.b[467] {
            s.store_mul_add_scaled_product_rhs(512, 576, s.ad_value(494), 1.0, s.ad_value(515), s.ad_value(495), (-1.0));
            s.store_sub(475, 514, 512);
            s.store_scaled_mul(517, 513, 485, 2.0);
            s.store_mul(518, 488, 517);
            s.store_sub_scaled_inputs(575, 475, 1.0, 511, (p.p51 * 0.5));
        }

        if s.b[467] {
            s.store_div_scaled_inputs2_mixed_aii(574, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sub(s.ad_value(477), s.ad_value(577)), A::tanh_scaled_input(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(477), s.ad_value(577)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 575, (-1.0), 511, 1.0);
        }

        s.b[581] = (s.v[574] > 50.0);
        s.store_scalar(581, if s.b[581] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[581]) {
            s.store_scalar(533, 0.0);
        }

        s.b[582] = (s.v[574] < (-50.0));
        s.store_scalar(582, if s.b[582] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[581])) && s.b[582]) {
            s.store_scalar(533, 1.0);
        }

        if ((s.b[467] && (!s.b[581])) && (!s.b[582])) {
            s.store_div_from_scalar_offset_ad(533, 1.0, A::exp(s.ad_value(574)), 1.0);
        }

        if s.b[467] {
            s.store_div_scaled_inputs2_mixed_aai(534, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sub(s.ad_value(477), s.ad_value(577)), A::tanh_scaled_input(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(477), s.ad_value(577)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(475), 1.0, s.ad_value(511), s.ad_value(533), (-(p.p51 * 0.1))), (-1.0), 517, 1.0);
        }

        s.b[583] = (s.v[534] > 50.0);
        s.store_scalar(583, if s.b[583] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[583]) {
            s.store_mul(535, 518, 534);
        }

        s.b[584] = (s.v[534] < (-50.0));
        s.store_scalar(584, if s.b[584] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[583])) && s.b[584]) {
            s.store_mul_exp_rhs(535, 518, 534);
        }

        if ((s.b[467] && (!s.b[583])) && (!s.b[584])) {
            s.store_mul_ln_one_plus_exp_rhs(535, 518, 534);
        }

        if s.b[467] {
            s.store_div_ad_rhs(521, 499, A::mul_offset_rhs(s.ad_value(532), A::div_scaled_product(s.ad_value(501), s.ad_value(535), 1.0, s.ad_value(488), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(522, 498, A::div_scaled_offset_numerator(A::mul(s.ad_value(506), s.ad_value(484)), 1.0, 1.0, A::offset(A::mul(s.ad_value(506), s.ad_value(483)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(507), s.ad_value(576), 1.0, s.ad_value(487), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(502), s.ad_value(535), 1.0, s.ad_value(488), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(539, 522, 487, 1.0, 521, 1.0);
            s.store_add_scaled_product_right_ad(540, 539, (-1.0), 539, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(535), 2.0, s.ad_value(488), s.ad_value(539), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(541, A::mul_sub_from_scalar_rhs(s.ad_value(539), 1.0, s.ad_value(533)), 1.0, 517, 533, 1.0);
            s.store_add_scaled_product_value_ad(476, A::mul_sub_from_scalar_rhs(s.ad_value(540), 1.0, s.ad_value(533)), 1.0, 517, 533, 1.0);
        }

        if s.b[467] {
            s.store_div_from_scalar_pow_ad(542, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(478), s.ad_value(476)), 0.5, A::div(s.ad_value(478), s.ad_value(476)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(478), s.ad_value(476))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(478), s.ad_value(476)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(478), s.ad_value(476))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
        }

        if s.b[467] {
            s.store_mul(543, 478, 542);
        }

        if s.b[467] {
            s.store_div_from_scalar_pow_ad(544, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
        }

        if s.b[467] {
            s.store_mul_neg_lhs(545, 478, 544);
            s.store_div_scaled_inputs2_indices(574, 477, 1.0, 575, (-1.0), 511, 1.0);
        }

        s.b[585] = (s.v[574] > 50.0);
        s.store_scalar(585, if s.b[585] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[585]) {
            s.store_scalar(516, 0.0);
        }

        s.b[586] = (s.v[574] < (-50.0));
        s.store_scalar(586, if s.b[586] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[585])) && s.b[586]) {
            s.store_scalar(516, 1.0);
        }

        if ((s.b[467] && (!s.b[585])) && (!s.b[586])) {
            s.store_div_from_scalar_offset_ad(516, 1.0, A::exp(s.ad_value(574)), 1.0);
        }

        if s.b[467] {
            s.store_div_scaled_inputs3_mixed_iiai(519, 577, 1.0, 545, (-1.0), A::add_scaled_product(s.ad_value(475), 1.0, s.ad_value(511), s.ad_value(516), (-(p.p51 * 0.1))), -1.0, 517, 1.0);
        }

        s.b[587] = (s.v[519] > 50.0);
        s.store_scalar(587, if s.b[587] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[587]) {
            s.store_mul(520, 518, 519);
        }

        s.b[588] = (s.v[519] < (-50.0));
        s.store_scalar(588, if s.b[588] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[587])) && s.b[588]) {
            s.store_mul_exp_rhs(520, 518, 519);
        }

        if ((s.b[467] && (!s.b[587])) && (!s.b[588])) {
            s.store_mul_ln_one_plus_exp_rhs(520, 518, 519);
        }

        if s.b[467] {
            s.store_div_scaled_inputs2_indices(574, 577, 1.0, 575, (-1.0), 511, 1.0);
        }

        s.b[589] = (s.v[574] > 50.0);
        s.store_scalar(589, if s.b[589] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[589]) {
            s.store_scalar(546, 0.0);
        }

        s.b[590] = (s.v[574] < (-50.0));
        s.store_scalar(590, if s.b[590] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[589])) && s.b[590]) {
            s.store_scalar(546, 1.0);
        }

        if ((s.b[467] && (!s.b[589])) && (!s.b[590])) {
            s.store_div_from_scalar_offset_ad(546, 1.0, A::exp(s.ad_value(574)), 1.0);
        }

        if s.b[467] {
            s.store_div_scaled_inputs3_mixed_iiai(547, 477, 1.0, 543, (-1.0), A::add_scaled_product(s.ad_value(475), 1.0, s.ad_value(511), s.ad_value(546), (-(p.p51 * 0.1))), -1.0, 517, 1.0);
        }

        s.b[591] = (s.v[547] > 50.0);
        s.store_scalar(591, if s.b[591] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[591]) {
            s.store_mul(548, 518, 547);
        }

        s.b[592] = (s.v[547] < (-50.0));
        s.store_scalar(592, if s.b[592] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[591])) && s.b[592]) {
            s.store_mul_exp_rhs(548, 518, 547);
        }

        if ((s.b[467] && (!s.b[591])) && (!s.b[592])) {
            s.store_mul_ln_one_plus_exp_rhs(548, 518, 547);
        }

        if s.b[467] {
            s.store_div_scaled_inputs2_indices(549, 520, 1.0, 548, (-1.0), 488, 1.0);
            s.store_div(575, 549, 541);
            s.store_div_scaled_inputs_indices(524, 493, 1.0, 485, 2.302585092994046);
            s.store_scaled_mul(526, 524, 485, 2.0);
            s.store_mul(527, 488, 526);
            s.store_sub_scaled_inputs(579, 514, 1.0, 511, (p.p51 * 0.5));
        }

        if s.b[467] {
            s.store_div_scaled_inputs2_mixed_aii(578, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sub(s.ad_value(477), s.ad_value(577)), A::tanh_scaled_input(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(477), s.ad_value(577)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 579, (-1.0), 511, 1.0);
        }

        s.b[593] = (s.v[578] > 50.0);
        s.store_scalar(593, if s.b[593] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[593]) {
            s.store_scalar(536, 0.0);
        }

        s.b[594] = (s.v[578] < (-50.0));
        s.store_scalar(594, if s.b[594] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[593])) && s.b[594]) {
            s.store_scalar(536, 1.0);
        }

        if ((s.b[467] && (!s.b[593])) && (!s.b[594])) {
            s.store_div_from_scalar_offset_ad(536, 1.0, A::exp(s.ad_value(578)), 1.0);
        }

        if s.b[467] {
            s.store_div_scaled_inputs2_mixed_aai(537, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sub(s.ad_value(477), s.ad_value(577)), A::tanh_scaled_input(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(477), s.ad_value(577)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(514), 1.0, s.ad_value(511), s.ad_value(536), (-(p.p51 * 0.1))), (-1.0), 526, 1.0);
        }

        s.b[595] = (s.v[537] > 50.0);
        s.store_scalar(595, if s.b[595] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[595]) {
            s.store_mul(538, 527, 537);
        }

        s.b[596] = (s.v[537] < (-50.0));
        s.store_scalar(596, if s.b[596] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[595])) && s.b[596]) {
            s.store_mul_exp_rhs(538, 527, 537);
        }

        if ((s.b[467] && (!s.b[595])) && (!s.b[596])) {
            s.store_mul_ln_one_plus_exp_rhs(538, 527, 537);
        }

        if s.b[467] {
            s.store_div(530, 499, 532);
            s.store_mul_div_scaled_offset_numerator_rhs(531, 498, A::mul(s.ad_value(506), s.ad_value(484)), 1.0, 1.0, A::offset(A::mul(s.ad_value(506), s.ad_value(483)), 1.0), 1.0);
            s.store_div_scaled_product_indices(552, 531, 487, 1.0, 530, 1.0);
            s.store_add_scaled_product_right_ad(553, 552, (-1.0), 552, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(538), 2.0, s.ad_value(488), s.ad_value(552), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(554, A::mul_sub_from_scalar_rhs(s.ad_value(553), 1.0, s.ad_value(536)), 1.0, 526, 536, 1.0);
        }

        if s.b[467] {
            s.store_div_from_scalar_pow_ad(555, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(478), s.ad_value(554)), 0.5, A::div(s.ad_value(478), s.ad_value(554)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(478), s.ad_value(554))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(478), s.ad_value(554)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(478), s.ad_value(554))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
        }

        if s.b[467] {
            s.store_mul(556, 478, 555);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[467] {
            s.store_div_from_scalar_pow_ad(557, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
        }

        if s.b[467] {
            s.store_mul_neg_lhs(558, 478, 557);
            s.store_div_scaled_inputs2_indices(578, 477, 1.0, 579, (-1.0), 511, 1.0);
        }

        s.b[597] = (s.v[578] > 50.0);
        s.store_scalar(597, if s.b[597] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[597]) {
            s.store_scalar(525, 0.0);
        }

        s.b[598] = (s.v[578] < (-50.0));
        s.store_scalar(598, if s.b[598] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[597])) && s.b[598]) {
            s.store_scalar(525, 1.0);
        }

        if ((s.b[467] && (!s.b[597])) && (!s.b[598])) {
            s.store_div_from_scalar_offset_ad(525, 1.0, A::exp(s.ad_value(578)), 1.0);
        }

        if s.b[467] {
            s.store_div_scaled_inputs3_mixed_iiai(528, 577, 1.0, 558, (-1.0), A::add_scaled_product(s.ad_value(514), 1.0, s.ad_value(511), s.ad_value(525), (-(p.p51 * 0.1))), -1.0, 526, 1.0);
        }

        s.b[599] = (s.v[528] > 50.0);
        s.store_scalar(599, if s.b[599] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[599]) {
            s.store_mul(529, 527, 528);
        }

        s.b[600] = (s.v[528] < (-50.0));
        s.store_scalar(600, if s.b[600] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[599])) && s.b[600]) {
            s.store_mul_exp_rhs(529, 527, 528);
        }

        if ((s.b[467] && (!s.b[599])) && (!s.b[600])) {
            s.store_mul_ln_one_plus_exp_rhs(529, 527, 528);
        }

        if s.b[467] {
            s.store_div_scaled_inputs2_indices(578, 577, 1.0, 579, (-1.0), 511, 1.0);
        }

        s.b[601] = (s.v[578] > 50.0);
        s.store_scalar(601, if s.b[601] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[601]) {
            s.store_scalar(559, 0.0);
        }

        s.b[602] = (s.v[578] < (-50.0));
        s.store_scalar(602, if s.b[602] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[601])) && s.b[602]) {
            s.store_scalar(559, 1.0);
        }

        if ((s.b[467] && (!s.b[601])) && (!s.b[602])) {
            s.store_div_from_scalar_offset_ad(559, 1.0, A::exp(s.ad_value(578)), 1.0);
        }

        if s.b[467] {
            s.store_div_scaled_inputs3_mixed_iiai(560, 477, 1.0, 556, (-1.0), A::add_scaled_product(s.ad_value(514), 1.0, s.ad_value(511), s.ad_value(559), (-(p.p51 * 0.1))), -1.0, 526, 1.0);
        }

        s.b[603] = (s.v[560] > 50.0);
        s.store_scalar(603, if s.b[603] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[603]) {
            s.store_mul(561, 527, 560);
        }

        s.b[604] = (s.v[560] < (-50.0));
        s.store_scalar(604, if s.b[604] { 1.0 } else { 0.0 });

        if ((s.b[467] && (!s.b[603])) && s.b[604]) {
            s.store_mul_exp_rhs(561, 527, 560);
        }

        if ((s.b[467] && (!s.b[603])) && (!s.b[604])) {
            s.store_mul_ln_one_plus_exp_rhs(561, 527, 560);
        }

        if s.b[467] {
            s.store_offset_square(562, 529, 1e-38);
            s.store_offset_mul(563, 562, 529, 1e-57);
            s.store_offset_square(564, 561, 1e-38);
            s.store_offset_mul(565, 564, 561, 1e-57);
            s.store_offset_mul(566, 529, 561, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(567, 562, (2.0 / 3.0), 564, (2.0 / 3.0), 566, (2.0 / 3.0), A::offset(A::add(s.ad_value(529), s.ad_value(561)), 2e-19), 1.0);
            s.store_div_ad(568, A::add_scaled_inputs_products(s.ad_value(563), (2.0 * 2.0), s.ad_value(565), (3.0 * 2.0), s.ad_value(562), s.ad_value(561), (4.0 * 2.0), s.ad_value(564), s.ad_value(529), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(562), 15.0, s.ad_value(564), 15.0, s.ad_value(566), (2.0 * 15.0)));
            s.store_sub(569, 567, 568);
            s.copy_ad(570, 568);
            s.store_mul_product3_mixed_iaii(470, 510, A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(487)), 509, 569, 1.0);
            s.store_mul_product3_mixed_iaii(471, 510, A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(487)), 509, 570, 1.0);
        }

        s.b[605] = (s.v[479] == 1.0);
        s.store_scalar(605, if s.b[605] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[605]) {
            s.store_div_scaled_inputs3_indices(571, 480, 1.0, 514, -1.0, 511, (-(-(p.p51 * 0.5))), 526, 1.0);
        }

        s.b[606] = (s.v[571] > 50.0);
        s.store_scalar(606, if s.b[606] { 1.0 } else { 0.0 });

        if ((s.b[467] && s.b[605]) && s.b[606]) {
            s.copy_ad(574, 571);
        }

        s.b[607] = (s.v[571] < (-50.0));
        s.store_scalar(607, if s.b[607] { 1.0 } else { 0.0 });

        if (((s.b[467] && s.b[605]) && (!s.b[606])) && s.b[607]) {
            s.store_exp(574, 571);
        }

        if (((s.b[467] && s.b[605]) && (!s.b[606])) && (!s.b[607])) {
            s.store_ln_one_plus_exp(574, 571);
        }

        if (s.b[467] && s.b[605]) {
            s.store_mul_ad_product_lhs_mixed_ai(472, A::mul3(A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(509)), s.ad_value(490), s.ad_value(526)), 574, 510);
            s.store_div_scaled_inputs3_indices(572, 481, 1.0, 514, -1.0, 511, (-(-(p.p51 * 0.5))), 526, 1.0);
        }

        s.b[608] = (s.v[572] > 50.0);
        s.store_scalar(608, if s.b[608] { 1.0 } else { 0.0 });

        if ((s.b[467] && s.b[605]) && s.b[608]) {
            s.copy_ad(574, 572);
        }

        s.b[609] = (s.v[572] < (-50.0));
        s.store_scalar(609, if s.b[609] { 1.0 } else { 0.0 });

        if (((s.b[467] && s.b[605]) && (!s.b[608])) && s.b[609]) {
            s.store_exp(574, 572);
        }

        if (((s.b[467] && s.b[605]) && (!s.b[608])) && (!s.b[609])) {
            s.store_ln_one_plus_exp(574, 572);
        }

        if (s.b[467] && s.b[605]) {
            s.store_mul_ad_product_lhs_mixed_ai(473, A::mul3(A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(509)), s.ad_value(491), s.ad_value(526)), 574, 510);
        }

        if (s.b[467] && (!s.b[605])) {
            s.store_scalar(472, 0.0);
            s.store_scalar(473, 0.0);
        }

        s.b[610] = (s.v[482] == 1.0);
        s.store_scalar(610, if s.b[610] { 1.0 } else { 0.0 });

        if (s.b[467] && s.b[610]) {
            s.store_div_scaled_inputs3_indices(573, 477, 1.0, 514, -1.0, 511, (-(-(p.p51 * 0.5))), 526, 1.0);
        }

        s.b[611] = (s.v[573] > 50.0);
        s.store_scalar(611, if s.b[611] { 1.0 } else { 0.0 });

        if ((s.b[467] && s.b[610]) && s.b[611]) {
            s.copy_ad(574, 573);
        }

        s.b[612] = (s.v[573] < (-50.0));
        s.store_scalar(612, if s.b[612] { 1.0 } else { 0.0 });

        if (((s.b[467] && s.b[610]) && (!s.b[611])) && s.b[612]) {
            s.store_exp(574, 573);
        }

        if (((s.b[467] && s.b[610]) && (!s.b[611])) && (!s.b[612])) {
            s.store_ln_one_plus_exp(574, 573);
        }

        if (s.b[467] && s.b[610]) {
            s.store_mul_ad_product_lhs_mixed_ai(474, A::mul3(A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(509)), s.ad_value(489), s.ad_value(526)), 574, 510);
        }

        if (s.b[467] && (!s.b[610])) {
            s.store_scalar(474, 0.0);
        }

        if s.b[467] {
            s.copy_ad(203, 470);
            s.copy_ad(204, 471);
            s.copy_ad(205, 472);
            s.copy_ad(206, 473);
            s.copy_ad(207, 474);
        }

        s.b[613] = (p.p210 == 1.0);
        s.store_scalar(613, if s.b[613] { 1.0 } else { 0.0 });

        s.store_scalar(197, 0.0);

        s.store_scalar(198, 0.0);

        s.store_scalar(199, 0.0);

        s.store_scalar(200, 0.0);

        s.store_scalar(201, 0.0);

        s.b[614] = (p.p189 > p.p354);
        s.store_scalar(614, if s.b[614] { 1.0 } else { 0.0 });

        if s.b[614] {
            s.store_scalar(617, 0.0);
            s.store_scalar(618, 0.0);
            s.store_scalar(619, 0.0);
            s.store_scalar(620, 0.0);
            s.store_scalar(621, 0.0);
            s.store_scalar(622, 0.0);
            s.store_scalar(623, 0.0);
            s.copy_ad(624, 90);
            s.copy_ad(625, 91);
            s.store_scalar(626, p.p195);
            s.copy_ad(627, 92);
            s.copy_ad(628, 93);
            s.store_scalar(629, p.p193);
            s.copy_ad(630, 111);
            s.store_scalar(631, s.v[109]);
            s.copy_ad(632, 113);
            s.store_scalar(633, p.p0);
            s.store_scalar(634, p.p189);
            s.copy_ad(635, 35);
            s.store_scalar(636, p.p194);
            s.copy_ad(637, 36);
            s.copy_ad(638, 37);
            s.store_scalar(639, p.p190);
            s.store_scalar(640, p.p204);
            s.store_scalar(641, p.p203);
            s.store_scalar(642, 0.0);
            s.store_scalar(643, p.p205);
            s.store_scalar(644, p.p209);
            s.store_scalar(645, p.p200);
            s.store_scalar(646, p.p201);
            s.store_scalar(647, p.p202);
            s.store_scalar(648, p.p208);
            s.store_scalar(649, p.p207);
            s.store_scalar(650, p.p206);
            s.store_scalar(651, p.p39);
            s.store_scalar(652, p.p47);
            s.store_scalar(653, p.p45);
            s.store_scalar(654, p.p42);
            s.store_scalar(655, p.p2);
            s.store_scalar(656, p.p6);
            s.store_scalar(657, 1.0);
            s.store_scalar(658, 0.0);
            s.store_scalar(659, 0.0);
            s.store_scalar(660, 0.0);
            s.store_scalar(661, 0.0);
            s.store_scalar(662, 0.0);
            s.store_scalar(663, 0.0);
            s.store_scalar(664, 0.0);
            s.store_scalar(665, 0.0);
            s.store_scalar(666, 0.0);
            s.store_scalar(667, 0.0);
            s.store_scalar(668, 0.0);
            s.store_scalar(669, 0.0);
            s.store_scalar(671, 0.0);
            s.store_scalar(672, 0.0);
            s.store_scalar(673, 0.0);
            s.store_scalar(674, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[614] {
            s.store_scalar(675, 0.0);
            s.store_scalar(676, 0.0);
            s.store_scalar(677, 0.0);
            s.store_scalar(678, 0.0);
            s.store_scalar(679, 0.0);
            s.store_scalar(680, 0.0);
            s.store_scalar(681, 0.0);
            s.store_scalar(682, 0.0);
            s.store_scalar(683, 0.0);
            s.store_scalar(684, 0.0);
            s.store_scalar(685, 0.0);
            s.store_scalar(686, 0.0);
            s.store_scalar(687, 0.0);
            s.store_scalar(688, 0.0);
            s.store_scalar(689, 0.0);
            s.store_scalar(690, 0.0);
            s.store_scalar(691, 0.0);
            s.store_scalar(692, 0.0);
            s.store_scalar(693, 0.0);
            s.store_scalar(694, 0.0);
            s.store_scalar(695, 0.0);
            s.store_scalar(696, 0.0);
            s.store_scalar(699, 0.0);
            s.store_scalar(700, 0.0);
            s.store_scalar(701, 0.0);
            s.store_scalar(702, 0.0);
            s.store_scalar(703, 0.0);
            s.store_scalar(704, 0.0);
            s.store_scalar(705, 0.0);
            s.store_scalar(706, 0.0);
            s.store_scalar(707, 0.0);
            s.store_scalar(708, 0.0);
            s.store_scalar(709, 0.0);
            s.store_scalar(710, 0.0);
            s.store_scalar(711, 0.0);
            s.store_scalar(712, 0.0);
            s.store_scalar(713, 0.0);
            s.store_scalar(714, 0.0);
            s.store_scalar(715, 0.0);
            s.store_scalar(716, 0.0);
            s.store_scalar(717, 0.0);
            s.store_scalar(718, 0.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(721, 0.0);
            s.store_scalar(722, 0.0);
            s.store_scalar(723, 0.0);
            s.store_scalar(724, 0.0);
            s.store_scalar(725, 0.0);
            s.store_scalar(726, 0.0);
        }

        if s.b[614] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(723, 625, A::tanh_scaled_input(s.ad_value(625), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(723, 625, p.p53);
                } else {
                    s.store_scalar(723, 0.0);
                }
            }
        }

        if s.b[614] {
            s.store_sub(724, 624, 625);
            s.store_mul(658, 644, 632);
            s.store_add_scaled_product_value_ad(660, A::div_scaled_inputs(s.ad_value(640), 1.0, s.ad_value(632), 2.302585092994046), 1.0, 643, 723, 1.0);
            s.store_add_scaled_product_right_sub(661, 639, 1.0, 650, 630, 631, 1.0);
            s.store_pow_ad(679, A::div(s.ad_value(630), s.ad_value(631)), s.ad_value(652));
        }

        s.b[727] = (s.v[651] != 0.0);
        s.store_scalar(727, if s.b[727] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[727]) {
            s.store_div_ad_rhs(662, 723, A::pow(A::offset(A::pow(A::div(s.ad_value(723), s.ad_value(651)), s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647))));
        }

        if (s.b[614] && (!s.b[727])) {
            s.store_scalar(662, 0.0);
        }

        if s.b[614] {
            s.store_mul_add_scaled_product_rhs(659, 723, s.ad_value(641), 1.0, s.ad_value(662), s.ad_value(642), (-1.0));
            s.store_sub(622, 661, 659);
            s.store_scaled_mul(664, 660, 632, 2.0);
            s.store_mul(665, 635, 664);
            s.store_sub_scaled_inputs(722, 622, 1.0, 658, (p.p51 * 0.5));
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_mixed_aii(721, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(624), s.ad_value(724)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 722, (-1.0), 658, 1.0);
        }

        s.b[728] = (s.v[721] > 50.0);
        s.store_scalar(728, if s.b[728] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[728]) {
            s.store_scalar(680, 0.0);
        }

        s.b[729] = (s.v[721] < (-50.0));
        s.store_scalar(729, if s.b[729] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[728])) && s.b[729]) {
            s.store_scalar(680, 1.0);
        }

        if ((s.b[614] && (!s.b[728])) && (!s.b[729])) {
            s.store_div_from_scalar_offset_ad(680, 1.0, A::exp(s.ad_value(721)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_mixed_aai(681, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(624), s.ad_value(724)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(622), 1.0, s.ad_value(658), s.ad_value(680), (-(p.p51 * 0.1))), (-1.0), 664, 1.0);
        }

        s.b[730] = (s.v[681] > 50.0);
        s.store_scalar(730, if s.b[730] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[730]) {
            s.store_mul(682, 665, 681);
        }

        s.b[731] = (s.v[681] < (-50.0));
        s.store_scalar(731, if s.b[731] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[730])) && s.b[731]) {
            s.store_mul_exp_rhs(682, 665, 681);
        }

        if ((s.b[614] && (!s.b[730])) && (!s.b[731])) {
            s.store_mul_ln_one_plus_exp_rhs(682, 665, 681);
        }

        if s.b[614] {
            s.store_div_ad_rhs(668, 646, A::mul_offset_rhs(s.ad_value(679), A::div_scaled_product(s.ad_value(648), s.ad_value(682), 1.0, s.ad_value(635), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(669, 645, A::div_scaled_offset_numerator(A::mul(s.ad_value(653), s.ad_value(631)), 1.0, 1.0, A::offset(A::mul(s.ad_value(653), s.ad_value(630)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(654), s.ad_value(723), 1.0, s.ad_value(634), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(649), s.ad_value(682), 1.0, s.ad_value(635), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(686, 669, 634, 1.0, 668, 1.0);
            s.store_add_scaled_product_right_ad(687, 686, (-1.0), 686, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(682), 2.0, s.ad_value(635), s.ad_value(686), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(688, A::mul_sub_from_scalar_rhs(s.ad_value(686), 1.0, s.ad_value(680)), 1.0, 664, 680, 1.0);
            s.store_add_scaled_product_value_ad(623, A::mul_sub_from_scalar_rhs(s.ad_value(687), 1.0, s.ad_value(680)), 1.0, 664, 680, 1.0);
        }

        if s.b[614] {
            s.store_div_from_scalar_pow_ad(689, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(625), s.ad_value(623)), 0.5, A::div(s.ad_value(625), s.ad_value(623)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(625), s.ad_value(623))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(625), s.ad_value(623)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(625), s.ad_value(623))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul(690, 625, 689);
        }

        if s.b[614] {
            s.store_div_from_scalar_pow_ad(691, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul_neg_lhs(692, 625, 691);
            s.store_div_scaled_inputs2_indices(721, 624, 1.0, 722, (-1.0), 658, 1.0);
        }

        s.b[732] = (s.v[721] > 50.0);
        s.store_scalar(732, if s.b[732] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[732]) {
            s.store_scalar(663, 0.0);
        }

        s.b[733] = (s.v[721] < (-50.0));
        s.store_scalar(733, if s.b[733] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[732])) && s.b[733]) {
            s.store_scalar(663, 1.0);
        }

        if ((s.b[614] && (!s.b[732])) && (!s.b[733])) {
            s.store_div_from_scalar_offset_ad(663, 1.0, A::exp(s.ad_value(721)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs3_mixed_iiai(666, 724, 1.0, 692, (-1.0), A::add_scaled_product(s.ad_value(622), 1.0, s.ad_value(658), s.ad_value(663), (-(p.p51 * 0.1))), -1.0, 664, 1.0);
        }

        s.b[734] = (s.v[666] > 50.0);
        s.store_scalar(734, if s.b[734] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[734]) {
            s.store_mul(667, 665, 666);
        }

        s.b[735] = (s.v[666] < (-50.0));
        s.store_scalar(735, if s.b[735] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[734])) && s.b[735]) {
            s.store_mul_exp_rhs(667, 665, 666);
        }

        if ((s.b[614] && (!s.b[734])) && (!s.b[735])) {
            s.store_mul_ln_one_plus_exp_rhs(667, 665, 666);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_indices(721, 724, 1.0, 722, (-1.0), 658, 1.0);
        }

        s.b[736] = (s.v[721] > 50.0);
        s.store_scalar(736, if s.b[736] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[736]) {
            s.store_scalar(693, 0.0);
        }

        s.b[737] = (s.v[721] < (-50.0));
        s.store_scalar(737, if s.b[737] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[736])) && s.b[737]) {
            s.store_scalar(693, 1.0);
        }

        if ((s.b[614] && (!s.b[736])) && (!s.b[737])) {
            s.store_div_from_scalar_offset_ad(693, 1.0, A::exp(s.ad_value(721)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs3_mixed_iiai(694, 624, 1.0, 690, (-1.0), A::add_scaled_product(s.ad_value(622), 1.0, s.ad_value(658), s.ad_value(693), (-(p.p51 * 0.1))), -1.0, 664, 1.0);
        }

        s.b[738] = (s.v[694] > 50.0);
        s.store_scalar(738, if s.b[738] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[738]) {
            s.store_mul(695, 665, 694);
        }

        s.b[739] = (s.v[694] < (-50.0));
        s.store_scalar(739, if s.b[739] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[738])) && s.b[739]) {
            s.store_mul_exp_rhs(695, 665, 694);
        }

        if ((s.b[614] && (!s.b[738])) && (!s.b[739])) {
            s.store_mul_ln_one_plus_exp_rhs(695, 665, 694);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_indices(696, 667, 1.0, 695, (-1.0), 635, 1.0);
            s.store_div(722, 696, 688);
            s.store_div_scaled_inputs_indices(671, 640, 1.0, 632, 2.302585092994046);
            s.store_scaled_mul(673, 671, 632, 2.0);
            s.store_mul(674, 635, 673);
            s.store_sub_scaled_inputs(726, 661, 1.0, 658, (p.p51 * 0.5));
        }

    }

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[614] {
            s.store_div_scaled_inputs2_mixed_aii(725, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(624), s.ad_value(724)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 726, (-1.0), 658, 1.0);
        }

        s.b[740] = (s.v[725] > 50.0);
        s.store_scalar(740, if s.b[740] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[740]) {
            s.store_scalar(683, 0.0);
        }

        s.b[741] = (s.v[725] < (-50.0));
        s.store_scalar(741, if s.b[741] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[740])) && s.b[741]) {
            s.store_scalar(683, 1.0);
        }

        if ((s.b[614] && (!s.b[740])) && (!s.b[741])) {
            s.store_div_from_scalar_offset_ad(683, 1.0, A::exp(s.ad_value(725)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_mixed_aai(684, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(624), s.ad_value(724)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(661), 1.0, s.ad_value(658), s.ad_value(683), (-(p.p51 * 0.1))), (-1.0), 673, 1.0);
        }

        s.b[742] = (s.v[684] > 50.0);
        s.store_scalar(742, if s.b[742] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[742]) {
            s.store_mul(685, 674, 684);
        }

        s.b[743] = (s.v[684] < (-50.0));
        s.store_scalar(743, if s.b[743] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[742])) && s.b[743]) {
            s.store_mul_exp_rhs(685, 674, 684);
        }

        if ((s.b[614] && (!s.b[742])) && (!s.b[743])) {
            s.store_mul_ln_one_plus_exp_rhs(685, 674, 684);
        }

        if s.b[614] {
            s.store_div(677, 646, 679);
            s.store_mul_div_scaled_offset_numerator_rhs(678, 645, A::mul(s.ad_value(653), s.ad_value(631)), 1.0, 1.0, A::offset(A::mul(s.ad_value(653), s.ad_value(630)), 1.0), 1.0);
            s.store_div_scaled_product_indices(699, 678, 634, 1.0, 677, 1.0);
            s.store_add_scaled_product_right_ad(700, 699, (-1.0), 699, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(685), 2.0, s.ad_value(635), s.ad_value(699), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(701, A::mul_sub_from_scalar_rhs(s.ad_value(700), 1.0, s.ad_value(683)), 1.0, 673, 683, 1.0);
        }

        if s.b[614] {
            s.store_div_from_scalar_pow_ad(702, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(625), s.ad_value(701)), 0.5, A::div(s.ad_value(625), s.ad_value(701)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(625), s.ad_value(701))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(625), s.ad_value(701)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(625), s.ad_value(701))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul(703, 625, 702);
        }

        if s.b[614] {
            s.store_div_from_scalar_pow_ad(704, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul_neg_lhs(705, 625, 704);
            s.store_div_scaled_inputs2_indices(725, 624, 1.0, 726, (-1.0), 658, 1.0);
        }

        s.b[744] = (s.v[725] > 50.0);
        s.store_scalar(744, if s.b[744] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[744]) {
            s.store_scalar(672, 0.0);
        }

        s.b[745] = (s.v[725] < (-50.0));
        s.store_scalar(745, if s.b[745] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[744])) && s.b[745]) {
            s.store_scalar(672, 1.0);
        }

        if ((s.b[614] && (!s.b[744])) && (!s.b[745])) {
            s.store_div_from_scalar_offset_ad(672, 1.0, A::exp(s.ad_value(725)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs3_mixed_iiai(675, 724, 1.0, 705, (-1.0), A::add_scaled_product(s.ad_value(661), 1.0, s.ad_value(658), s.ad_value(672), (-(p.p51 * 0.1))), -1.0, 673, 1.0);
        }

        s.b[746] = (s.v[675] > 50.0);
        s.store_scalar(746, if s.b[746] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[746]) {
            s.store_mul(676, 674, 675);
        }

        s.b[747] = (s.v[675] < (-50.0));
        s.store_scalar(747, if s.b[747] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[746])) && s.b[747]) {
            s.store_mul_exp_rhs(676, 674, 675);
        }

        if ((s.b[614] && (!s.b[746])) && (!s.b[747])) {
            s.store_mul_ln_one_plus_exp_rhs(676, 674, 675);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2_indices(725, 724, 1.0, 726, (-1.0), 658, 1.0);
        }

        s.b[748] = (s.v[725] > 50.0);
        s.store_scalar(748, if s.b[748] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[748]) {
            s.store_scalar(706, 0.0);
        }

        s.b[749] = (s.v[725] < (-50.0));
        s.store_scalar(749, if s.b[749] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[748])) && s.b[749]) {
            s.store_scalar(706, 1.0);
        }

        if ((s.b[614] && (!s.b[748])) && (!s.b[749])) {
            s.store_div_from_scalar_offset_ad(706, 1.0, A::exp(s.ad_value(725)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs3_mixed_iiai(707, 624, 1.0, 703, (-1.0), A::add_scaled_product(s.ad_value(661), 1.0, s.ad_value(658), s.ad_value(706), (-(p.p51 * 0.1))), -1.0, 673, 1.0);
        }

        s.b[750] = (s.v[707] > 50.0);
        s.store_scalar(750, if s.b[750] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[750]) {
            s.store_mul(708, 674, 707);
        }

        s.b[751] = (s.v[707] < (-50.0));
        s.store_scalar(751, if s.b[751] { 1.0 } else { 0.0 });

        if ((s.b[614] && (!s.b[750])) && s.b[751]) {
            s.store_mul_exp_rhs(708, 674, 707);
        }

        if ((s.b[614] && (!s.b[750])) && (!s.b[751])) {
            s.store_mul_ln_one_plus_exp_rhs(708, 674, 707);
        }

        if s.b[614] {
            s.store_offset_square(709, 676, 1e-38);
            s.store_offset_mul(710, 709, 676, 1e-57);
            s.store_offset_square(711, 708, 1e-38);
            s.store_offset_mul(712, 711, 708, 1e-57);
            s.store_offset_mul(713, 676, 708, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(714, 709, (2.0 / 3.0), 711, (2.0 / 3.0), 713, (2.0 / 3.0), A::offset(A::add(s.ad_value(676), s.ad_value(708)), 2e-19), 1.0);
            s.store_div_ad(715, A::add_scaled_inputs_products(s.ad_value(710), (2.0 * 2.0), s.ad_value(712), (3.0 * 2.0), s.ad_value(709), s.ad_value(708), (4.0 * 2.0), s.ad_value(711), s.ad_value(676), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(709), 15.0, s.ad_value(711), 15.0, s.ad_value(713), (2.0 * 15.0)));
            s.store_sub(716, 714, 715);
            s.copy_ad(717, 715);
            s.store_mul_product3_mixed_iaii(617, 657, A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(634)), 656, 716, 1.0);
            s.store_mul_product3_mixed_iaii(618, 657, A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(634)), 656, 717, 1.0);
        }

        s.b[752] = (s.v[626] == 1.0);
        s.store_scalar(752, if s.b[752] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[752]) {
            s.store_div_scaled_inputs3_indices(718, 627, 1.0, 661, -1.0, 658, (-(-(p.p51 * 0.5))), 673, 1.0);
        }

        s.b[753] = (s.v[718] > 50.0);
        s.store_scalar(753, if s.b[753] { 1.0 } else { 0.0 });

        if ((s.b[614] && s.b[752]) && s.b[753]) {
            s.copy_ad(721, 718);
        }

        s.b[754] = (s.v[718] < (-50.0));
        s.store_scalar(754, if s.b[754] { 1.0 } else { 0.0 });

        if (((s.b[614] && s.b[752]) && (!s.b[753])) && s.b[754]) {
            s.store_exp(721, 718);
        }

        if (((s.b[614] && s.b[752]) && (!s.b[753])) && (!s.b[754])) {
            s.store_ln_one_plus_exp(721, 718);
        }

        if (s.b[614] && s.b[752]) {
            s.store_mul_ad_product_lhs_mixed_ai(619, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(656)), s.ad_value(637), s.ad_value(673)), 721, 657);
            s.store_div_scaled_inputs3_indices(719, 628, 1.0, 661, -1.0, 658, (-(-(p.p51 * 0.5))), 673, 1.0);
        }

        s.b[755] = (s.v[719] > 50.0);
        s.store_scalar(755, if s.b[755] { 1.0 } else { 0.0 });

        if ((s.b[614] && s.b[752]) && s.b[755]) {
            s.copy_ad(721, 719);
        }

        s.b[756] = (s.v[719] < (-50.0));
        s.store_scalar(756, if s.b[756] { 1.0 } else { 0.0 });

        if (((s.b[614] && s.b[752]) && (!s.b[755])) && s.b[756]) {
            s.store_exp(721, 719);
        }

        if (((s.b[614] && s.b[752]) && (!s.b[755])) && (!s.b[756])) {
            s.store_ln_one_plus_exp(721, 719);
        }

        if (s.b[614] && s.b[752]) {
            s.store_mul_ad_product_lhs_mixed_ai(620, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(656)), s.ad_value(638), s.ad_value(673)), 721, 657);
        }

        if (s.b[614] && (!s.b[752])) {
            s.store_scalar(619, 0.0);
            s.store_scalar(620, 0.0);
        }

        s.b[757] = (s.v[629] == 1.0);
        s.store_scalar(757, if s.b[757] { 1.0 } else { 0.0 });

        if (s.b[614] && s.b[757]) {
            s.store_div_scaled_inputs3_indices(720, 624, 1.0, 661, -1.0, 658, (-(-(p.p51 * 0.5))), 673, 1.0);
        }

        s.b[758] = (s.v[720] > 50.0);
        s.store_scalar(758, if s.b[758] { 1.0 } else { 0.0 });

        if ((s.b[614] && s.b[757]) && s.b[758]) {
            s.copy_ad(721, 720);
        }

        s.b[759] = (s.v[720] < (-50.0));
        s.store_scalar(759, if s.b[759] { 1.0 } else { 0.0 });

        if (((s.b[614] && s.b[757]) && (!s.b[758])) && s.b[759]) {
            s.store_exp(721, 720);
        }

        if (((s.b[614] && s.b[757]) && (!s.b[758])) && (!s.b[759])) {
            s.store_ln_one_plus_exp(721, 720);
        }

        if (s.b[614] && s.b[757]) {
            s.store_mul_ad_product_lhs_mixed_ai(621, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(656)), s.ad_value(636), s.ad_value(673)), 721, 657);
        }

        if (s.b[614] && (!s.b[757])) {
            s.store_scalar(621, 0.0);
        }

        if s.b[614] {
            s.copy_ad(197, 617);
            s.copy_ad(198, 618);
            s.copy_ad(199, 619);
            s.copy_ad(200, 620);
            s.copy_ad(201, 621);
        }

        s.b[760] = (p.p188 == 1.0);
        s.store_scalar(760, if s.b[760] { 1.0 } else { 0.0 });

        s.store_scalar(191, 0.0);

        s.store_scalar(192, 0.0);

        s.store_scalar(193, 0.0);

        s.store_scalar(194, 0.0);

        s.store_scalar(195, 0.0);

        s.b[761] = (p.p167 > p.p354);
        s.store_scalar(761, if s.b[761] { 1.0 } else { 0.0 });

        if s.b[761] {
            s.store_scalar(764, 0.0);
            s.store_scalar(765, 0.0);
            s.store_scalar(766, 0.0);
            s.store_scalar(767, 0.0);
            s.store_scalar(768, 0.0);
            s.store_scalar(769, 0.0);
            s.store_scalar(770, 0.0);
            s.copy_ad(771, 84);
            s.copy_ad(772, 85);
            s.store_scalar(773, p.p173);
            s.copy_ad(774, 86);
            s.copy_ad(775, 87);
            s.store_scalar(776, p.p171);
            s.copy_ad(777, 111);
            s.store_scalar(778, s.v[109]);
            s.copy_ad(779, 113);
            s.store_scalar(780, p.p0);
            s.store_scalar(781, p.p167);
            s.copy_ad(782, 32);
            s.store_scalar(783, p.p172);
            s.copy_ad(784, 33);
            s.copy_ad(785, 34);
            s.store_scalar(786, p.p168);
            s.store_scalar(787, p.p182);
            s.store_scalar(788, p.p181);
            s.store_scalar(789, 0.0);
            s.store_scalar(790, p.p183);
            s.store_scalar(791, p.p187);
            s.store_scalar(792, p.p178);
            s.store_scalar(793, p.p179);
        }

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[761] {
            s.store_scalar(794, p.p180);
            s.store_scalar(795, p.p186);
            s.store_scalar(796, p.p185);
            s.store_scalar(797, p.p184);
            s.store_scalar(798, p.p39);
            s.store_scalar(799, p.p47);
            s.store_scalar(800, p.p45);
            s.store_scalar(801, p.p42);
            s.store_scalar(802, p.p2);
            s.store_scalar(803, p.p6);
            s.store_scalar(804, 1.0);
            s.store_scalar(805, 0.0);
            s.store_scalar(806, 0.0);
            s.store_scalar(807, 0.0);
            s.store_scalar(808, 0.0);
            s.store_scalar(809, 0.0);
            s.store_scalar(810, 0.0);
            s.store_scalar(811, 0.0);
            s.store_scalar(812, 0.0);
            s.store_scalar(813, 0.0);
            s.store_scalar(814, 0.0);
            s.store_scalar(815, 0.0);
            s.store_scalar(816, 0.0);
            s.store_scalar(818, 0.0);
            s.store_scalar(819, 0.0);
            s.store_scalar(820, 0.0);
            s.store_scalar(821, 0.0);
            s.store_scalar(822, 0.0);
            s.store_scalar(823, 0.0);
            s.store_scalar(824, 0.0);
            s.store_scalar(825, 0.0);
            s.store_scalar(826, 0.0);
            s.store_scalar(827, 0.0);
            s.store_scalar(828, 0.0);
            s.store_scalar(829, 0.0);
            s.store_scalar(830, 0.0);
            s.store_scalar(831, 0.0);
            s.store_scalar(832, 0.0);
            s.store_scalar(833, 0.0);
            s.store_scalar(834, 0.0);
            s.store_scalar(835, 0.0);
            s.store_scalar(836, 0.0);
            s.store_scalar(837, 0.0);
            s.store_scalar(838, 0.0);
            s.store_scalar(839, 0.0);
            s.store_scalar(840, 0.0);
            s.store_scalar(841, 0.0);
            s.store_scalar(842, 0.0);
            s.store_scalar(843, 0.0);
            s.store_scalar(846, 0.0);
            s.store_scalar(847, 0.0);
            s.store_scalar(848, 0.0);
            s.store_scalar(849, 0.0);
            s.store_scalar(850, 0.0);
            s.store_scalar(851, 0.0);
            s.store_scalar(852, 0.0);
            s.store_scalar(853, 0.0);
            s.store_scalar(854, 0.0);
            s.store_scalar(855, 0.0);
            s.store_scalar(856, 0.0);
            s.store_scalar(857, 0.0);
            s.store_scalar(858, 0.0);
            s.store_scalar(859, 0.0);
            s.store_scalar(860, 0.0);
            s.store_scalar(861, 0.0);
            s.store_scalar(862, 0.0);
            s.store_scalar(863, 0.0);
            s.store_scalar(864, 0.0);
            s.store_scalar(865, 0.0);
            s.store_scalar(866, 0.0);
            s.store_scalar(867, 0.0);
            s.store_scalar(868, 0.0);
            s.store_scalar(869, 0.0);
            s.store_scalar(870, 0.0);
            s.store_scalar(871, 0.0);
            s.store_scalar(872, 0.0);
            s.store_scalar(873, 0.0);
        }

        if s.b[761] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(870, 772, A::tanh_scaled_input(s.ad_value(772), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(870, 772, p.p53);
                } else {
                    s.store_scalar(870, 0.0);
                }
            }
        }

        if s.b[761] {
            s.store_sub(871, 771, 772);
            s.store_mul(805, 791, 779);
            s.store_add_scaled_product_value_ad(807, A::div_scaled_inputs(s.ad_value(787), 1.0, s.ad_value(779), 2.302585092994046), 1.0, 790, 870, 1.0);
            s.store_add_scaled_product_right_sub(808, 786, 1.0, 797, 777, 778, 1.0);
            s.store_pow_ad(826, A::div(s.ad_value(777), s.ad_value(778)), s.ad_value(799));
        }

        s.b[874] = (s.v[798] != 0.0);
        s.store_scalar(874, if s.b[874] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[874]) {
            s.store_div_ad_rhs(809, 870, A::pow(A::offset(A::pow(A::div(s.ad_value(870), s.ad_value(798)), s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794))));
        }

        if (s.b[761] && (!s.b[874])) {
            s.store_scalar(809, 0.0);
        }

        if s.b[761] {
            s.store_mul_add_scaled_product_rhs(806, 870, s.ad_value(788), 1.0, s.ad_value(809), s.ad_value(789), (-1.0));
            s.store_sub(769, 808, 806);
            s.store_scaled_mul(811, 807, 779, 2.0);
            s.store_mul(812, 782, 811);
            s.store_sub_scaled_inputs(869, 769, 1.0, 805, (p.p51 * 0.5));
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_mixed_aii(868, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(771), s.ad_value(871)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 869, (-1.0), 805, 1.0);
        }

        s.b[875] = (s.v[868] > 50.0);
        s.store_scalar(875, if s.b[875] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[875]) {
            s.store_scalar(827, 0.0);
        }

        s.b[876] = (s.v[868] < (-50.0));
        s.store_scalar(876, if s.b[876] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[875])) && s.b[876]) {
            s.store_scalar(827, 1.0);
        }

        if ((s.b[761] && (!s.b[875])) && (!s.b[876])) {
            s.store_div_from_scalar_offset_ad(827, 1.0, A::exp(s.ad_value(868)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_mixed_aai(828, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(771), s.ad_value(871)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(827), (-(p.p51 * 0.1))), (-1.0), 811, 1.0);
        }

        s.b[877] = (s.v[828] > 50.0);
        s.store_scalar(877, if s.b[877] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[877]) {
            s.store_mul(829, 812, 828);
        }

        s.b[878] = (s.v[828] < (-50.0));
        s.store_scalar(878, if s.b[878] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[877])) && s.b[878]) {
            s.store_mul_exp_rhs(829, 812, 828);
        }

        if ((s.b[761] && (!s.b[877])) && (!s.b[878])) {
            s.store_mul_ln_one_plus_exp_rhs(829, 812, 828);
        }

        if s.b[761] {
            s.store_div_ad_rhs(815, 793, A::mul_offset_rhs(s.ad_value(826), A::div_scaled_product(s.ad_value(795), s.ad_value(829), 1.0, s.ad_value(782), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(816, 792, A::div_scaled_offset_numerator(A::mul(s.ad_value(800), s.ad_value(778)), 1.0, 1.0, A::offset(A::mul(s.ad_value(800), s.ad_value(777)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(801), s.ad_value(870), 1.0, s.ad_value(781), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(796), s.ad_value(829), 1.0, s.ad_value(782), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(833, 816, 781, 1.0, 815, 1.0);
            s.store_add_scaled_product_right_ad(834, 833, (-1.0), 833, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(829), 2.0, s.ad_value(782), s.ad_value(833), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(835, A::mul_sub_from_scalar_rhs(s.ad_value(833), 1.0, s.ad_value(827)), 1.0, 811, 827, 1.0);
            s.store_add_scaled_product_value_ad(770, A::mul_sub_from_scalar_rhs(s.ad_value(834), 1.0, s.ad_value(827)), 1.0, 811, 827, 1.0);
        }

        if s.b[761] {
            s.store_div_from_scalar_pow_ad(836, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(772), s.ad_value(770)), 0.5, A::div(s.ad_value(772), s.ad_value(770)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(772), s.ad_value(770))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(772), s.ad_value(770)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(772), s.ad_value(770))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul(837, 772, 836);
        }

        if s.b[761] {
            s.store_div_from_scalar_pow_ad(838, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(770), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul_neg_lhs(839, 772, 838);
            s.store_div_scaled_inputs2_indices(868, 771, 1.0, 869, (-1.0), 805, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[879] = (s.v[868] > 50.0);
        s.store_scalar(879, if s.b[879] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[879]) {
            s.store_scalar(810, 0.0);
        }

        s.b[880] = (s.v[868] < (-50.0));
        s.store_scalar(880, if s.b[880] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[879])) && s.b[880]) {
            s.store_scalar(810, 1.0);
        }

        if ((s.b[761] && (!s.b[879])) && (!s.b[880])) {
            s.store_div_from_scalar_offset_ad(810, 1.0, A::exp(s.ad_value(868)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs3_mixed_iiai(813, 871, 1.0, 839, (-1.0), A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(810), (-(p.p51 * 0.1))), -1.0, 811, 1.0);
        }

        s.b[881] = (s.v[813] > 50.0);
        s.store_scalar(881, if s.b[881] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[881]) {
            s.store_mul(814, 812, 813);
        }

        s.b[882] = (s.v[813] < (-50.0));
        s.store_scalar(882, if s.b[882] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[881])) && s.b[882]) {
            s.store_mul_exp_rhs(814, 812, 813);
        }

        if ((s.b[761] && (!s.b[881])) && (!s.b[882])) {
            s.store_mul_ln_one_plus_exp_rhs(814, 812, 813);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_indices(868, 871, 1.0, 869, (-1.0), 805, 1.0);
        }

        s.b[883] = (s.v[868] > 50.0);
        s.store_scalar(883, if s.b[883] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[883]) {
            s.store_scalar(840, 0.0);
        }

        s.b[884] = (s.v[868] < (-50.0));
        s.store_scalar(884, if s.b[884] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[883])) && s.b[884]) {
            s.store_scalar(840, 1.0);
        }

        if ((s.b[761] && (!s.b[883])) && (!s.b[884])) {
            s.store_div_from_scalar_offset_ad(840, 1.0, A::exp(s.ad_value(868)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs3_mixed_iiai(841, 771, 1.0, 837, (-1.0), A::add_scaled_product(s.ad_value(769), 1.0, s.ad_value(805), s.ad_value(840), (-(p.p51 * 0.1))), -1.0, 811, 1.0);
        }

        s.b[885] = (s.v[841] > 50.0);
        s.store_scalar(885, if s.b[885] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[885]) {
            s.store_mul(842, 812, 841);
        }

        s.b[886] = (s.v[841] < (-50.0));
        s.store_scalar(886, if s.b[886] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[885])) && s.b[886]) {
            s.store_mul_exp_rhs(842, 812, 841);
        }

        if ((s.b[761] && (!s.b[885])) && (!s.b[886])) {
            s.store_mul_ln_one_plus_exp_rhs(842, 812, 841);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_indices(843, 814, 1.0, 842, (-1.0), 782, 1.0);
            s.store_div(869, 843, 835);
            s.store_div_scaled_inputs_indices(818, 787, 1.0, 779, 2.302585092994046);
            s.store_scaled_mul(820, 818, 779, 2.0);
            s.store_mul(821, 782, 820);
            s.store_sub_scaled_inputs(873, 808, 1.0, 805, (p.p51 * 0.5));
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_mixed_aii(872, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(771), s.ad_value(871)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 873, (-1.0), 805, 1.0);
        }

        s.b[887] = (s.v[872] > 50.0);
        s.store_scalar(887, if s.b[887] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[887]) {
            s.store_scalar(830, 0.0);
        }

        s.b[888] = (s.v[872] < (-50.0));
        s.store_scalar(888, if s.b[888] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[887])) && s.b[888]) {
            s.store_scalar(830, 1.0);
        }

        if ((s.b[761] && (!s.b[887])) && (!s.b[888])) {
            s.store_div_from_scalar_offset_ad(830, 1.0, A::exp(s.ad_value(872)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_mixed_aai(831, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sub(s.ad_value(771), s.ad_value(871)), A::tanh_scaled_input(A::sub(s.ad_value(771), s.ad_value(871)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(771), 0.5, s.ad_value(871), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(771), s.ad_value(871)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(830), (-(p.p51 * 0.1))), (-1.0), 820, 1.0);
        }

        s.b[889] = (s.v[831] > 50.0);
        s.store_scalar(889, if s.b[889] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[889]) {
            s.store_mul(832, 821, 831);
        }

        s.b[890] = (s.v[831] < (-50.0));
        s.store_scalar(890, if s.b[890] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[889])) && s.b[890]) {
            s.store_mul_exp_rhs(832, 821, 831);
        }

        if ((s.b[761] && (!s.b[889])) && (!s.b[890])) {
            s.store_mul_ln_one_plus_exp_rhs(832, 821, 831);
        }

        if s.b[761] {
            s.store_div(824, 793, 826);
            s.store_mul_div_scaled_offset_numerator_rhs(825, 792, A::mul(s.ad_value(800), s.ad_value(778)), 1.0, 1.0, A::offset(A::mul(s.ad_value(800), s.ad_value(777)), 1.0), 1.0);
            s.store_div_scaled_product_indices(846, 825, 781, 1.0, 824, 1.0);
            s.store_add_scaled_product_right_ad(847, 846, (-1.0), 846, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(832), 2.0, s.ad_value(782), s.ad_value(846), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(848, A::mul_sub_from_scalar_rhs(s.ad_value(847), 1.0, s.ad_value(830)), 1.0, 820, 830, 1.0);
        }

        if s.b[761] {
            s.store_div_from_scalar_pow_ad(849, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(772), s.ad_value(848)), 0.5, A::div(s.ad_value(772), s.ad_value(848)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(772), s.ad_value(848))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(772), s.ad_value(848)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(772), s.ad_value(848))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul(850, 772, 849);
        }

        if s.b[761] {
            s.store_div_from_scalar_pow_ad(851, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(772), -1.0, s.ad_value(848), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(794)), 1.0), A::div_from_scalar(1.0, s.ad_value(794)));
        }

        if s.b[761] {
            s.store_mul_neg_lhs(852, 772, 851);
            s.store_div_scaled_inputs2_indices(872, 771, 1.0, 873, (-1.0), 805, 1.0);
        }

        s.b[891] = (s.v[872] > 50.0);
        s.store_scalar(891, if s.b[891] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[891]) {
            s.store_scalar(819, 0.0);
        }

        s.b[892] = (s.v[872] < (-50.0));
        s.store_scalar(892, if s.b[892] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[891])) && s.b[892]) {
            s.store_scalar(819, 1.0);
        }

        if ((s.b[761] && (!s.b[891])) && (!s.b[892])) {
            s.store_div_from_scalar_offset_ad(819, 1.0, A::exp(s.ad_value(872)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs3_mixed_iiai(822, 871, 1.0, 852, (-1.0), A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(819), (-(p.p51 * 0.1))), -1.0, 820, 1.0);
        }

        s.b[893] = (s.v[822] > 50.0);
        s.store_scalar(893, if s.b[893] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[893]) {
            s.store_mul(823, 821, 822);
        }

        s.b[894] = (s.v[822] < (-50.0));
        s.store_scalar(894, if s.b[894] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[893])) && s.b[894]) {
            s.store_mul_exp_rhs(823, 821, 822);
        }

        if ((s.b[761] && (!s.b[893])) && (!s.b[894])) {
            s.store_mul_ln_one_plus_exp_rhs(823, 821, 822);
        }

        if s.b[761] {
            s.store_div_scaled_inputs2_indices(872, 871, 1.0, 873, (-1.0), 805, 1.0);
        }

        s.b[895] = (s.v[872] > 50.0);
        s.store_scalar(895, if s.b[895] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[895]) {
            s.store_scalar(853, 0.0);
        }

        s.b[896] = (s.v[872] < (-50.0));
        s.store_scalar(896, if s.b[896] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[895])) && s.b[896]) {
            s.store_scalar(853, 1.0);
        }

        if ((s.b[761] && (!s.b[895])) && (!s.b[896])) {
            s.store_div_from_scalar_offset_ad(853, 1.0, A::exp(s.ad_value(872)), 1.0);
        }

        if s.b[761] {
            s.store_div_scaled_inputs3_mixed_iiai(854, 771, 1.0, 850, (-1.0), A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(805), s.ad_value(853), (-(p.p51 * 0.1))), -1.0, 820, 1.0);
        }

        s.b[897] = (s.v[854] > 50.0);
        s.store_scalar(897, if s.b[897] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[897]) {
            s.store_mul(855, 821, 854);
        }

        s.b[898] = (s.v[854] < (-50.0));
        s.store_scalar(898, if s.b[898] { 1.0 } else { 0.0 });

        if ((s.b[761] && (!s.b[897])) && s.b[898]) {
            s.store_mul_exp_rhs(855, 821, 854);
        }

        if ((s.b[761] && (!s.b[897])) && (!s.b[898])) {
            s.store_mul_ln_one_plus_exp_rhs(855, 821, 854);
        }

        if s.b[761] {
            s.store_offset_square(856, 823, 1e-38);
            s.store_offset_mul(857, 856, 823, 1e-57);
            s.store_offset_square(858, 855, 1e-38);
            s.store_offset_mul(859, 858, 855, 1e-57);
            s.store_offset_mul(860, 823, 855, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(861, 856, (2.0 / 3.0), 858, (2.0 / 3.0), 860, (2.0 / 3.0), A::offset(A::add(s.ad_value(823), s.ad_value(855)), 2e-19), 1.0);
            s.store_div_ad(862, A::add_scaled_inputs_products(s.ad_value(857), (2.0 * 2.0), s.ad_value(859), (3.0 * 2.0), s.ad_value(856), s.ad_value(855), (4.0 * 2.0), s.ad_value(858), s.ad_value(823), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(856), 15.0, s.ad_value(858), 15.0, s.ad_value(860), (2.0 * 15.0)));
            s.store_sub(863, 861, 862);
            s.copy_ad(864, 862);
            s.store_mul_product3_mixed_iaii(764, 804, A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(781)), 803, 863, 1.0);
            s.store_mul_product3_mixed_iaii(765, 804, A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(781)), 803, 864, 1.0);
        }

        s.b[899] = (s.v[773] == 1.0);
        s.store_scalar(899, if s.b[899] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[899]) {
            s.store_div_scaled_inputs3_indices(865, 774, 1.0, 808, -1.0, 805, (-(-(p.p51 * 0.5))), 820, 1.0);
        }

        s.b[900] = (s.v[865] > 50.0);
        s.store_scalar(900, if s.b[900] { 1.0 } else { 0.0 });

        if ((s.b[761] && s.b[899]) && s.b[900]) {
            s.copy_ad(868, 865);
        }

        s.b[901] = (s.v[865] < (-50.0));
        s.store_scalar(901, if s.b[901] { 1.0 } else { 0.0 });

        if (((s.b[761] && s.b[899]) && (!s.b[900])) && s.b[901]) {
            s.store_exp(868, 865);
        }

        if (((s.b[761] && s.b[899]) && (!s.b[900])) && (!s.b[901])) {
            s.store_ln_one_plus_exp(868, 865);
        }

        if (s.b[761] && s.b[899]) {
            s.store_mul_ad_product_lhs_mixed_ai(766, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(803)), s.ad_value(784), s.ad_value(820)), 868, 804);
            s.store_div_scaled_inputs3_indices(866, 775, 1.0, 808, -1.0, 805, (-(-(p.p51 * 0.5))), 820, 1.0);
        }

        s.b[902] = (s.v[866] > 50.0);
        s.store_scalar(902, if s.b[902] { 1.0 } else { 0.0 });

        if ((s.b[761] && s.b[899]) && s.b[902]) {
            s.copy_ad(868, 866);
        }

        s.b[903] = (s.v[866] < (-50.0));
        s.store_scalar(903, if s.b[903] { 1.0 } else { 0.0 });

        if (((s.b[761] && s.b[899]) && (!s.b[902])) && s.b[903]) {
            s.store_exp(868, 866);
        }

        if (((s.b[761] && s.b[899]) && (!s.b[902])) && (!s.b[903])) {
            s.store_ln_one_plus_exp(868, 866);
        }

        if (s.b[761] && s.b[899]) {
            s.store_mul_ad_product_lhs_mixed_ai(767, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(803)), s.ad_value(785), s.ad_value(820)), 868, 804);
        }

        if (s.b[761] && (!s.b[899])) {
            s.store_scalar(766, 0.0);
            s.store_scalar(767, 0.0);
        }

        s.b[904] = (s.v[776] == 1.0);
        s.store_scalar(904, if s.b[904] { 1.0 } else { 0.0 });

        if (s.b[761] && s.b[904]) {
            s.store_div_scaled_inputs3_indices(867, 771, 1.0, 808, -1.0, 805, (-(-(p.p51 * 0.5))), 820, 1.0);
        }

        s.b[905] = (s.v[867] > 50.0);
        s.store_scalar(905, if s.b[905] { 1.0 } else { 0.0 });

        if ((s.b[761] && s.b[904]) && s.b[905]) {
            s.copy_ad(868, 867);
        }

        s.b[906] = (s.v[867] < (-50.0));
        s.store_scalar(906, if s.b[906] { 1.0 } else { 0.0 });

        if (((s.b[761] && s.b[904]) && (!s.b[905])) && s.b[906]) {
            s.store_exp(868, 867);
        }

        if (((s.b[761] && s.b[904]) && (!s.b[905])) && (!s.b[906])) {
            s.store_ln_one_plus_exp(868, 867);
        }

        if (s.b[761] && s.b[904]) {
            s.store_mul_ad_product_lhs_mixed_ai(768, A::mul3(A::mul3(s.ad_value(780), s.ad_value(802), s.ad_value(803)), s.ad_value(783), s.ad_value(820)), 868, 804);
        }

        if (s.b[761] && (!s.b[904])) {
            s.store_scalar(768, 0.0);
        }

        if s.b[761] {
            s.copy_ad(191, 764);
            s.copy_ad(192, 765);
            s.copy_ad(193, 766);
            s.copy_ad(194, 767);
            s.copy_ad(195, 768);
        }

        s.b[907] = (p.p166 == 1.0);
        s.store_scalar(907, if s.b[907] { 1.0 } else { 0.0 });

        s.store_scalar(167, 0.0);

        s.store_scalar(168, 0.0);

        s.store_scalar(169, 0.0);

        s.store_scalar(170, 0.0);

        s.store_scalar(171, 0.0);

        s.b[908] = (p.p79 > p.p354);
        s.store_scalar(908, if s.b[908] { 1.0 } else { 0.0 });

        if s.b[908] {
            s.store_scalar(911, 0.0);
            s.store_scalar(912, 0.0);
            s.store_scalar(913, 0.0);
        }

    }
}
